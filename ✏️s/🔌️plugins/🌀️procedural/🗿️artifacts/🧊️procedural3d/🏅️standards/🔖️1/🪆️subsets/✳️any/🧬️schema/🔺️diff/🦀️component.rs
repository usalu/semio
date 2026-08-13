//! 🧬️ Procedural3d diff schema — sparse field delta over the artifact.

use crate::artifacts::procedural3d::schema::Procedural3dArtifact;
use crate::artifacts::procedural3d::schema::Procedural3dPreviewCamera;
use flow::CameraJson;
use flow::FlowFixture;
use flow::playbook::GenerationPlayState;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Procedural3dDiff
/// 🧬️ Procedural3dDiff facet type.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.procedural.procedural3d")]

pub struct Procedural3dDiff {
    #[state(artifact)] pub artifact: Option<Box<Procedural3dArtifact>>,
    #[state(artifact)] pub fixture: Option<FlowFixture>,
    #[state(artifact)] pub generation: Option<GenerationPlayState>,
    #[state(presence)] pub selected_node_ids: Option<Procedural3dStringList>,
    #[state(config)] pub lod_mode: Option<String>,
    #[state(config)] pub show_mode: Option<String>,
    #[state(config)] pub selection_method: Option<String>,
    #[state(artifact)] pub hovered_node_id: Option<Option<String>>,
    #[state(config)] pub graph_camera: Option<CameraJson>,
    #[state(config)] pub preview_camera: Option<Procedural3dPreviewCamera>,
    #[state(config)] pub sun_json: Option<String>,
    #[state(presence)] pub selected_generation_id: Option<Option<String>>,
    #[state(artifact)] pub generation_preview_text: Option<Option<String>>,
    #[state(presence)] pub active_utility_id: Option<String>,
    #[state(config)] pub locale: Option<String>,
    #[state(config)] pub contributions_json: Option<String>}
//#endregion 🔖️Procedural3dDiff

//#region 🔖️Helpers
/// 📋 String-list wrapper so optional list diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Procedural3dStringList {
    pub values: Vec<String>}
//#endregion 🔖️Helpers
