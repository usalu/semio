//! 🧬️ GIS terrain diff schema — sparse field delta over the artifact.

use ::semio_framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔹Diff
/// 🔺️ Sparse field delta for the GIS terrain artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff).
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, ToValue, FromValue)]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[artifact_schema(id = "s.gis.gisterrain")]
pub struct GisTerrainDiff {
    #[state(artifact)]
    pub artifact: Option<Box<crate::schema::GisTerrainArtifact>>,
    #[state(artifact)]
    pub exaggeration: Option<f64>,
    #[state(artifact)]
    pub imported_map: Option<ImportedMapChange>,
}
/// 🔄️ A present replacement can explicitly clear the optional imported map owner.
#[derive(Clone,Debug,Default,PartialEq,ToValue,FromValue)]
#[value(rename_all="camelCase",deny_unknown_fields)]
pub struct ImportedMapChange{
    #[value(default,skip_serializing_if="Option::is_none")]
    pub value:Option<crate::schema::ImportedMap>,
}
//#endregion 🔹Diff
