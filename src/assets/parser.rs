use super::asset_loader::VectorLoaderError;
use crate::assets::asset::VectorFile;
use crate::VelloAsset;
use bevy::prelude::*;
use once_cell::sync::Lazy;
use std::sync::Arc;
use vello::Scene;
use vello_svg::usvg::{self, fontdb::Database};

pub static FONT_DB: Lazy<Database> = Lazy::new(usvg::fontdb::Database::default);

/// Deserialize an SVG file from bytes.
pub fn load_svg_from_bytes(bytes: &[u8]) -> Result<VelloAsset, VectorLoaderError> {
    let svg_str = std::str::from_utf8(bytes)?;

    let usvg = usvg::Tree::from_str(svg_str, &usvg::Options::default(), &FONT_DB)?;

    // Process the loaded SVG into Vello-compatible data
    let mut scene = Scene::new();
    vello_svg::render_tree(&mut scene, &usvg);

    let width = usvg.size().width();
    let height = usvg.size().height();

    let vello_vector = VelloAsset {
        data: VectorFile::Svg(Arc::new(scene)),
        local_transform_center: {
            let mut transform = Transform::default();
            transform.translation.x = width / 2.0;
            transform.translation.y = -height / 2.0;
            transform
        },
        width,
        height,
    };

    Ok(vello_vector)
}

/// Deserialize an SVG file from a string slice.
pub fn load_svg_from_str(svg_str: &str) -> Result<VelloAsset, VectorLoaderError> {
    let bytes = svg_str.as_bytes();

    load_svg_from_bytes(bytes)
}

/// Deserialize a Lottie file from bytes.
pub fn load_lottie_from_bytes(bytes: &[u8]) -> Result<VelloAsset, VectorLoaderError> {
    // Load Lottie JSON bytes with the Velato (bodymovin) parser
    let composition = velato::Composition::from_slice(bytes)
        .map_err(|err| VectorLoaderError::Parse(format!("Unable to parse lottie JSON: {err:?}")))?;

    let width = composition.width as f32;
    let height = composition.height as f32;

    let vello_vector = VelloAsset {
        data: VectorFile::Lottie(Arc::new(composition)),
        local_transform_center: {
            let mut transform = Transform::default();
            transform.translation.x = width / 2.0;
            transform.translation.y = -height / 2.0;
            transform
        },
        width,
        height,
    };

    Ok(vello_vector)
}

/// Deserialize a Lottie file from a string slice.
pub fn load_lottie_from_str(json_str: &str) -> Result<VelloAsset, VectorLoaderError> {
    let bytes = json_str.as_bytes();

    load_lottie_from_bytes(bytes)
}
