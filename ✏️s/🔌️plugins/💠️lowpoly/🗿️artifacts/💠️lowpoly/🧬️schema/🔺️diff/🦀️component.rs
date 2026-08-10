//! 🧬️ Lowpoly diff schema — sparse field delta over the artifact.

use crate::artifacts::lowpoly::{LowpolyObject, LowpolyObjectPatch, LowpolyPaintLayer, LowpolySelection};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the lowpoly artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.lowpoly.lowpoly")]
pub struct LowpolyDiff {
    #[state(persistent)] pub artifact: Option<Box<crate::artifacts::lowpoly::schema::LowpolyArtifact>>,
    #[state(persistent)] pub schema: Option<String>,
    #[state(persistent)] pub objects: Option<LowpolyObjectsDelta>,
    #[state(shared_ui)] pub active_object_id: Option<Option<String>>,
    #[state(shared_ui)] pub selection: Option<crate::artifacts::lowpoly::LowpolySelection>,
    #[state(shared_ui)] pub selected_object_ids: Option<LowpolyStringList>,
    #[state(shared_ui)] pub paint_utility: Option<String>,
    #[state(shared_ui)] pub active_paint_layer: Option<u32>,
    #[state(shared_ui)] pub active_utility_id: Option<String>,
    #[state(local_ui)] pub show_edges: Option<bool>,
    #[state(local_ui)] pub sun_enabled: Option<bool>,
    #[state(local_ui)] pub sun_azimuth: Option<f64>,
    #[state(local_ui)] pub sun_elevation: Option<f64>,
    #[state(local_ui)] pub sun_intensity: Option<f64>,
    #[state(local_ui)] pub sun_color: Option<String>,
    #[state(local_ui)] pub world_camera_position_x: Option<f64>,
    #[state(local_ui)] pub world_camera_position_y: Option<f64>,
    #[state(local_ui)] pub world_camera_position_z: Option<f64>,
    #[state(local_ui)] pub world_camera_target_x: Option<f64>,
    #[state(local_ui)] pub world_camera_target_y: Option<f64>,
    #[state(local_ui)] pub world_camera_target_z: Option<f64>,
    #[state(local_ui)] pub world_camera_fov: Option<f64>,
    #[state(local_ui)] pub utility_params_json: Option<String>,
    #[state(local_ui)] pub paint_color_r: Option<u32>,
    #[state(local_ui)] pub paint_color_g: Option<u32>,
    #[state(local_ui)] pub paint_color_b: Option<u32>,
    #[state(local_ui)] pub paint_color_a: Option<u32>,
    #[state(local_ui)] pub selection_method: Option<String>,
    #[state(local_ui)] pub selection_mode_default: Option<String>,
    #[state(local_ui)] pub engagement_input: Option<String>,
    #[state(local_ui)] pub locale: Option<String>,
    #[state(preview)] pub hovered_object_id: Option<Option<String>>,
    #[state(preview)] pub hovered_target_object_id: Option<Option<String>>,
    #[state(preview)] pub hovered_target_mode: Option<Option<String>>,
    #[state(preview)] pub hovered_target_id: Option<Option<u32>>,
    #[state(preview)] pub stroke_drag_active: Option<bool>,
    #[state(preview)] pub transform_drag_active: Option<bool>,
    #[state(preview)] pub preview_seq: Option<i64>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 📋 String-list wrapper so optional list diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct LowpolyStringList {
    pub values: Vec<String>,
}

/// 🧩 Identified-collection delta for `objects`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct LowpolyObjectsDelta {
    pub added: Vec<LowpolyObject>,
    pub removed: Vec<String>,
    pub patched: Vec<LowpolyObjectPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched object entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LowpolyObjectPatchEntry {
    pub id: String,
    pub patch: LowpolyObjectPatch,
    #[serde(default)]
    pub paint_layers: Option<LowpolyPaintLayersDelta>,
}

/// 🖌️ Paint-layer sub-delta under an object patch.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct LowpolyPaintLayersDelta {
    pub added: Vec<LowpolyIndexedPaintLayer>,
    pub removed: Vec<u32>,
    pub patched: Vec<LowpolyIndexedPaintLayerPatch>,
    pub strokes: Vec<LowpolyPaintStrokeAt>,
}

/// ➕️ Paint layer at index.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LowpolyIndexedPaintLayer {
    pub index: u32,
    pub layer: LowpolyPaintLayer,
}

/// 🩹 Paint layer metadata patch at index.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LowpolyIndexedPaintLayerPatch {
    pub index: u32,
    pub patch: LowpolyPaintLayerPatch,
}

/// 🖌️ Pixel runs on one layer.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LowpolyPaintStrokeAt {
    pub layer_index: u32,
    pub runs: Vec<PixelRun>,
}

/// 🩸 Contiguous RGBA run.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PixelRun {
    pub offset: u32,
    #[serde(with = "pixel_run_bytes_base64")]
    pub bytes: Vec<u8>,
}

mod pixel_run_bytes_base64 {
    use base64::Engine;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S: Serializer>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&base64::engine::general_purpose::STANDARD.encode(bytes))
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<u8>, D::Error> {
        let encoded = String::deserialize(deserializer)?;
        base64::engine::general_purpose::STANDARD.decode(encoded.as_bytes()).map_err(serde::de::Error::custom)
    }
}

/// 🩹 Paint-layer metadata patch.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct LowpolyPaintLayerPatch {
    pub name: Option<String>,
    pub visible: Option<bool>,
    pub opacity: Option<f32>,
    pub blend_mode: Option<String>,
}
//#endregion 🔖️DeltaHelpers
