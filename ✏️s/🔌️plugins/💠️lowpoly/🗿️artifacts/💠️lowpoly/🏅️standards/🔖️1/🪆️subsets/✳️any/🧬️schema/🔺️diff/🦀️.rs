//! 🧬️ Lowpoly diff schema — sparse field delta over the artifact.

use crate::artifacts::lowpoly::{LowpolyObject, LowpolyObjectPatch, LowpolyPaintLayer};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the lowpoly artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff).
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.lowpoly.lowpoly")]
pub struct LowpolyDiff {
    #[state(artifact)]
    pub artifact: Option<Box<crate::artifacts::lowpoly::schema::LowpolyArtifact>>,
    #[state(artifact)]
    pub schema: Option<String>,
    #[state(artifact)]
    pub objects: Option<LowpolyObjectsDelta>,
    #[state(presence)]
    pub active_object_id: Option<Option<String>>,
    #[state(presence)]
    pub selection: Option<crate::artifacts::lowpoly::LowpolySelection>,
    #[state(presence)]
    pub selected_object_ids: Option<LowpolyStringList>,
    #[state(presence)]
    pub paint_utility: Option<String>,
    #[state(presence)]
    pub active_paint_layer: Option<u32>,
    #[state(presence)]
    pub active_utility_id: Option<String>,
    #[state(config)]
    pub show_edges: Option<bool>,
    #[state(config)]
    pub sun_enabled: Option<bool>,
    #[state(config)]
    pub sun_azimuth: Option<f64>,
    #[state(config)]
    pub sun_elevation: Option<f64>,
    #[state(config)]
    pub sun_intensity: Option<f64>,
    #[state(config)]
    pub sun_color: Option<String>,
    #[state(config)]
    pub world_camera_position_x: Option<f64>,
    #[state(config)]
    pub world_camera_position_y: Option<f64>,
    #[state(config)]
    pub world_camera_position_z: Option<f64>,
    #[state(config)]
    pub world_camera_target_x: Option<f64>,
    #[state(config)]
    pub world_camera_target_y: Option<f64>,
    #[state(config)]
    pub world_camera_target_z: Option<f64>,
    #[state(config)]
    pub world_camera_fov: Option<f64>,
    #[state(config)]
    pub utility_params_json: Option<String>,
    #[state(config)]
    pub paint_color_r: Option<u32>,
    #[state(config)]
    pub paint_color_g: Option<u32>,
    #[state(config)]
    pub paint_color_b: Option<u32>,
    #[state(config)]
    pub paint_color_a: Option<u32>,
    #[state(config)]
    pub selection_method: Option<String>,
    #[state(config)]
    pub selection_mode_default: Option<String>,
    #[state(config)]
    pub engagement_input: Option<String>,
    #[state(config)]
    pub locale: Option<String>,
    #[state(artifact)]
    pub hovered_object_id: Option<Option<String>>,
    #[state(artifact)]
    pub hovered_target_object_id: Option<Option<String>>,
    #[state(artifact)]
    pub hovered_target_mode: Option<Option<String>>,
    #[state(artifact)]
    pub hovered_target_id: Option<Option<u32>>,
    #[state(artifact)]
    pub stroke_drag_active: Option<bool>,
    #[state(artifact)]
    pub transform_drag_active: Option<bool>,
    #[state(artifact)]
    pub preview_seq: Option<i64>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 📋 String-list wrapper so optional list diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct LowpolyStringList {
    pub values: Vec<String>,
}

/// 🧩 Identified-collection delta for `objects`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct LowpolyObjectsDelta {
    pub added: Vec<LowpolyObject>,
    pub removed: Vec<String>,
    pub patched: Vec<LowpolyObjectPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched object entry.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct LowpolyObjectPatchEntry {
    pub id: String,
    pub patch: LowpolyObjectPatch,
    #[value(default)]
    pub paint_layers: Option<LowpolyPaintLayersDelta>,
}

/// 🖌️ Paint-layer sub-delta under an object patch.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct LowpolyPaintLayersDelta {
    pub added: Vec<LowpolyIndexedPaintLayer>,
    pub removed: Vec<u32>,
    pub patched: Vec<LowpolyIndexedPaintLayerPatch>,
    pub strokes: Vec<LowpolyPaintStrokeAt>,
}

/// ➕️ Paint layer at index.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct LowpolyIndexedPaintLayer {
    pub index: u32,
    pub layer: LowpolyPaintLayer,
}

/// 🩹 Paint layer metadata patch at index.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct LowpolyIndexedPaintLayerPatch {
    pub index: u32,
    pub patch: LowpolyPaintLayerPatch,
}

/// 🖌️ Pixel runs on one layer.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct LowpolyPaintStrokeAt {
    pub layer_index: u32,
    pub runs: Vec<PixelRun>,
}

/// 🩸 Contiguous RGBA run.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct PixelRun {
    pub offset: u32,
    #[serde(with = "pixel_run_bytes_base64")]
    pub bytes: Vec<u8>,
}

mod pixel_run_bytes_base64 {
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S: Serializer>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&base64_codec::base64_standard_encode(bytes))
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<u8>, D::Error> {
        let encoded = String::deserialize(deserializer)?;
        base64_codec::base64_standard_decode(encoded.as_bytes()).map_err(serde::de::Error::custom)
    }
}

/// 🩹 Paint-layer metadata patch.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct LowpolyPaintLayerPatch {
    pub name: Option<String>,
    pub visible: Option<bool>,
    pub opacity: Option<f32>,
    pub blend_mode: Option<String>,
}
//#endregion 🔖️DeltaHelpers
