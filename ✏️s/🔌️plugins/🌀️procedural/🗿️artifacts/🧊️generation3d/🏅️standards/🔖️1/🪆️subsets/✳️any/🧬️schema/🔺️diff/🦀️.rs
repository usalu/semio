//! 🧬️ Generation3d diff schema — sparse field delta over the artifact.

use crate::artifacts::generation3d::schema::Generation3dArtifact;
use crate::artifacts::generation3d::schema::Generation3dPreviewCamera;
use flow::playbook::GenerationPlayRoot;
use flow::CameraJson;
use flow::FlowFixture;
use schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️Generation3dDiff
/// 🧬️ Generation3dDiff facet type.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.procedural.generation3d")]

pub struct Generation3dDiff {
    #[state(artifact)]
    pub artifact: Option<Box<Generation3dArtifact>>,
    #[state(artifact)]
    pub fixture: Option<FlowFixture>,
    #[state(artifact)]
    pub generation: Option<GenerationPlayRoot>,
    #[state(presence)]
    pub selected_node_ids: Option<Generation3dStringList>,
    #[state(config)]
    pub lod_mode: Option<String>,
    #[state(config)]
    pub show_mode: Option<String>,
    #[state(config)]
    pub selection_method: Option<String>,
    #[state(artifact)]
    pub hovered_node_id: Option<Option<String>>,
    #[state(config)]
    pub graph_camera: Option<CameraJson>,
    #[state(config)]
    pub preview_camera: Option<Generation3dPreviewCamera>,
    #[state(config)]
    pub sun_json: Option<String>,
    #[state(presence)]
    pub selected_generation_id: Option<Option<String>>,
    #[state(artifact)]
    pub generation_preview_text: Option<Option<String>>,
    #[state(presence)]
    pub active_utility_id: Option<String>,
    #[state(config)]
    pub locale: Option<String>,
}
//#endregion 🔖️Generation3dDiff

//#region 🔖️Helpers
/// 📋 String-list wrapper so optional list diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct Generation3dStringList {
    pub values: Vec<String>,
}
//#endregion 🔖️Helpers
