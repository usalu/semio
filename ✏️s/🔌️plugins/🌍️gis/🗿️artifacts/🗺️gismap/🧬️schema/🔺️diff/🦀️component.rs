//! 🧬️ GIS map diff schema — sparse field delta over the artifact.

use crate::artifacts::gismap::{MapFeature, MapFeaturePatch};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region 🔹Diff
/// 🔺️ Sparse field delta for the GIS map artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.gis.gismap")]
pub struct GisMapDiff {
    #[state(persistent)] pub artifact: Option<Box<crate::artifacts::gismap::schema::GisMapArtifact>>,
    #[state(persistent)] pub positions: Option<GisMapFeaturesDelta>,
    #[state(persistent)] pub routes: Option<GisMapFeaturesDelta>,
    #[state(persistent)] pub regions: Option<GisMapFeaturesDelta>,
    #[state(shared_ui)] pub selected_ids: Option<GisMapStringList>,
    #[state(shared_ui)] pub feature_selection_json: Option<String>,
    #[state(shared_ui)] pub layer_visibility: Option<GisMapBoolMapDelta>,
    #[state(shared_ui)] pub layer_stroke_scale: Option<GisMapNumberMapDelta>,
    #[state(local_ui)] pub camera_json: Option<String>,
    #[state(local_ui)] pub render_mode: Option<String>,
    #[state(local_ui)] pub vector_style: Option<String>,
    #[state(local_ui)] pub lod_mode: Option<String>,
    #[state(local_ui)] pub hover_json: Option<String>,
    #[state(local_ui)] pub selection_method: Option<String>,
    #[state(local_ui)] pub selection_mode: Option<String>,
    #[state(local_ui)] pub locale: Option<String>,
}
//#endregion 🔹Diff

//#region 🔹DeltaHelpers
/// 📋 String-list wrapper so optional list diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct GisMapStringList {
    pub values: Vec<String>,
}

/// 📂 Bool-map wrapper so optional map diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct GisMapBoolMapDelta {
    pub entries: BTreeMap<String, Option<bool>>,
}

/// 📂 Number-map wrapper so optional map diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct GisMapNumberMapDelta {
    pub entries: BTreeMap<String, Option<f64>>,
}

/// Identified-collection delta for feature lists.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct GisMapFeaturesDelta {
    pub added: Vec<MapFeature>,
    pub removed: Vec<String>,
    pub patched: Vec<GisMapFeaturePatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched feature entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GisMapFeaturePatchEntry {
    pub id: String,
    pub patch: MapFeaturePatch,
}
//#endregion 🔹DeltaHelpers
