//! 🧬️ Generation2d diff schema — sparse field delta over the artifact.

use crate::artifacts::generation2d::schema::Generation2dArtifact;
use flow::playbook::GenerationPlayRoot;
use flow::CameraJson;
use flow::FlowFixture;
use schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️Generation2dDiff
/// 🧬️ Generation2dDiff facet type.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.procedural.generation2d")]

pub struct Generation2dDiff {
    #[state(artifact)]
    pub artifact: Option<Box<Generation2dArtifact>>,
    #[state(artifact)]
    pub fixture: Option<FlowFixture>,
    #[state(artifact)]
    pub generation: Option<GenerationPlayRoot>,
    #[state(presence)]
    pub selected_ids: Option<Generation2dStringList>,
    #[state(config)]
    pub graph_camera: Option<CameraJson>,
    #[state(config)]
    pub show_mode: Option<String>,
    #[state(presence)]
    pub selected_generation_id: Option<Option<String>>,
    #[state(artifact)]
    pub generation_preview_text: Option<Option<String>>,
    #[state(config)]
    pub locale: Option<String>,
}
//#endregion 🔖️Generation2dDiff

//#region 🔖️Helpers
/// 📋 String-list wrapper so optional list diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct Generation2dStringList {
    pub values: Vec<String>,
}
//#endregion 🔖️Helpers
