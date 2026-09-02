//! 🧬️ GIS map diff schema — sparse field delta over the artifact.

use crate::artifacts::gismap::{MapFeature, MapFeaturePatch};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔹Diff
/// 🔺️ Sparse field delta for the GIS map artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff).
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, ToValue, FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.gis.gismap")]
pub struct GisMapDiff {
    #[state(artifact)]
    pub artifact: Option<Box<crate::artifacts::gismap::schema::GisMapArtifact>>,
    #[state(artifact)]
    pub positions: Option<GisMapFeaturesDelta>,
    #[state(artifact)]
    pub routes: Option<GisMapFeaturesDelta>,
    #[state(artifact)]
    pub regions: Option<GisMapFeaturesDelta>,
    #[state(presence)]
    pub layer_visibility: Option<GisMapBoolMapDelta>,
    #[state(presence)]
    pub layer_stroke_scale: Option<GisMapNumberMapDelta>,
    #[state(config)]
    pub camera_json: Option<String>,
    #[state(config)]
    pub render_mode: Option<String>,
    #[state(config)]
    pub vector_style: Option<String>,
    #[state(config)]
    pub lod_mode: Option<String>,
    #[state(config)]
    pub locale: Option<String>,
}
//#endregion 🔹Diff

//#region 🔹DeltaHelpers
/// 📂 Bool-map wrapper so optional map diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct GisMapBoolMapDelta {
    pub entries: BTreeMap<String, Option<bool>>,
}

/// 📂 Number-map wrapper so optional map diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct GisMapNumberMapDelta {
    pub entries: BTreeMap<String, Option<f64>>,
}

/// Identified-collection delta for feature lists.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct GisMapFeaturesDelta {
    pub added: Vec<MapFeature>,
    pub removed: Vec<String>,
    pub patched: Vec<GisMapFeaturePatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched feature entry.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct GisMapFeaturePatchEntry {
    pub id: String,
    pub patch: MapFeaturePatch,
}
//#endregion 🔹DeltaHelpers
