//! 🧬️ Lowpoly diff schema — sparse field delta over the artifact.

use crate::{LowpolyObject, LowpolyPaintLayer};
use framework_schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
//#region 🔖️Diff
/// 🔺️ Sparse field delta for the lowpoly artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff).
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.lowpoly.lowpoly")]
pub struct LowpolyDiff {
    #[state(artifact)]
    pub artifact: Option<Box<crate::schema::LowpolyArtifact>>,
    #[state(artifact)]
    pub schema: Option<String>,
    #[state(artifact)]
    pub objects: Option<LowpolyObjectsDelta>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
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

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use crate::LowpolyObjectPatch;
//#endregion 🔁️Re-exports
