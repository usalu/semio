//! 🧬️ GIS map diff schema — sparse field delta over the artifact.

use crate::{MapFeature, MapFeaturePatch};
use ::semio_framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔹Diff
/// 🔺️ Sparse field delta for the GIS map artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff).
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, ToValue, FromValue)]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[artifact_schema(id = "s.gis.gismap")]
pub struct GisMapDiff {
    #[state(artifact)]
    pub artifact: Option<Box<crate::schema::GisMapArtifact>>,
    #[state(artifact)]
    pub positions: Option<GisMapFeaturesDelta>,
    #[state(artifact)]
    pub routes: Option<GisMapFeaturesDelta>,
    #[state(artifact)]
    pub regions: Option<GisMapFeaturesDelta>,
}
//#endregion 🔹Diff

//#region 🔹DeltaHelpers
/// Identified-collection delta for feature lists.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct GisMapFeaturesDelta {
    pub added: Vec<MapFeature>,
    pub removed: Vec<String>,
    pub patched: Vec<GisMapFeaturePatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched feature entry.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct GisMapFeaturePatchEntry {
    pub id: String,
    pub patch: MapFeaturePatch,
}
//#endregion 🔹DeltaHelpers
