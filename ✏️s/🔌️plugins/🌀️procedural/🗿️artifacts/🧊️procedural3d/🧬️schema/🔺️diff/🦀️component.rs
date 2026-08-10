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
    #[state(persistent)] pub artifact: Option<Box<Procedural3dArtifact>>,
    #[state(persistent)] pub fixture: Option<FlowFixture>,
    #[state(persistent)] pub generation: Option<GenerationPlayState>,
    #[state(shared_ui)] pub selected_node_ids: Option<Procedural3dStringList>,
    #[state(local_ui)] pub lod_mode: Option<String>,
    #[state(local_ui)] pub show_mode: Option<String>,
    #[state(local_ui)] pub selection_method: Option<String>,
    #[state(preview)] pub hovered_node_id: Option<Option<String>>,
    #[state(local_ui)] pub graph_camera: Option<CameraJson>,
    #[state(local_ui)] pub preview_camera: Option<Procedural3dPreviewCamera>,
    #[state(local_ui)] pub sun_json: Option<String>,
    #[state(shared_ui)] pub selected_generation_id: Option<Option<String>>,
    #[state(preview)] pub generation_preview_text: Option<Option<String>>,
    #[state(shared_ui)] pub active_utility_id: Option<String>,
    #[state(local_ui)] pub locale: Option<String>,
    #[state(local_ui)] pub contributions_json: Option<String>}
//#endregion 🔖️Procedural3dDiff

//#region 🔖️Helpers
/// 📋 String-list wrapper so optional list diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Procedural3dStringList {
    pub values: Vec<String>}
//#endregion 🔖️Helpers
