//! 🧬️ Procedural2d diff schema — sparse field delta over the artifact.

use crate::artifacts::procedural2d::schema::Procedural2dArtifact;
use flow::playbook::GenerationPlayRoot;
use flow::CameraJson;
use flow::FlowFixture;
use schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️Procedural2dDiff
/// 🧬️ Procedural2dDiff facet type.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.procedural.procedural2d")]

pub struct Procedural2dDiff {
    #[state(artifact)]
    pub artifact: Option<Box<Procedural2dArtifact>>,
    #[state(artifact)]
    pub fixture: Option<FlowFixture>,
    #[state(artifact)]
    pub generation: Option<GenerationPlayRoot>,
    #[state(presence)]
    pub selected_ids: Option<Procedural2dStringList>,
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
//#endregion 🔖️Procedural2dDiff

//#region 🔖️Helpers
/// 📋 String-list wrapper so optional list diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct Procedural2dStringList {
    pub values: Vec<String>,
}
//#endregion 🔖️Helpers
