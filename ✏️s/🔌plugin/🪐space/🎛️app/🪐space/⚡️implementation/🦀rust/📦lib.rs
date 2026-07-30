//! 🎛️ S Studio app — document entities (constitutional: general).
//!
//! 🕳️ Deviation from the constitutional split recipe's usual "rs" content (a `#[derive(dsl::DslDocument)]`
//! entity + `X_DOCUMENT_SCHEMA` constant): the Studio app has no document type of its own — its
//! `DocumentApp::Projection`/`Operation` are `semio_framework_os::{OsProjection, OsOperation}`, owned
//! entirely by `framework/product/os/core/rs` (outside this plugin). What this app DOES own are its own
//! transient view-state records (panel bookkeeping, live-collaboration runtime state) — the closest
//! analog to "entity structs" this app has, so they live here instead.

use semio_framework_os::OsWorkflowCamera;
use serde::{Deserialize, Serialize};

//#region 🔖Constants
// 🕳️ Also a deviation from the usual "rs" content: these manifest/panel identifiers are shared by
// `space_engine` (`parse_panel_state`'s default tab id) and `space_ui` (manifest window/panel/body
// wiring, render dispatch) — centralized here rather than duplicated in both.
pub const S_PLAY_APP_ID: &str = "studio";
pub const S_PLAY_CONTROLLER_ID: &str = "s-play";
pub const S_PLAY_SURFACE_WORKFLOW: &str = "s.play.workflow";
pub const S_PLAY_SURFACE_MEDIA_VFS: &str = "s.play.media-vfs";
pub const S_PLAY_SURFACE_COMPILED_DAG: &str = "s.play.compiled-dag";
pub const S_PLAY_BODY_WORKFLOW: &str = "s.play.workflow";
pub const S_PLAY_BODY_MEDIA_VFS: &str = "s.play.media-vfs";
pub const S_PLAY_BODY_COMPILED_DAG: &str = "s.play.compiled-dag";
pub const S_PLAY_WINDOW_WORKFLOW: &str = "s-workflow";
pub const S_PLAY_WINDOW_MEDIA_VFS: &str = "s-media-vfs";
pub const S_PLAY_WINDOW_COMPILED_DAG: &str = "s-compiled-dag";
pub const S_PLAY_CATALOGUE_TAB_ID: &str = "s-play-catalogue";
pub const S_PLAY_PARAMETERS_TAB_ID: &str = "s-play-parameters";
pub const S_PLAY_INSPECTOR_TAB_ID: &str = "s-play-inspector";
pub const S_PLAY_CATALOGUE_BODY_KEY: &str = "s.play.catalogue";
pub const S_PLAY_PARAMETERS_BODY_KEY: &str = "s.play.parameters";
pub const S_PLAY_INSPECTOR_BODY_KEY: &str = "s.play.inspector";
pub const S_PLAY_CATALOGUE_DRAG_MIME: &str = "application/x-semio-catalogue-item";

pub const S_STUDIO_EXAMPLES: &[(&str, &str)] = &[("demo", "Demo Studio")];
//#endregion 🔖Constants

//#region 🔖Types
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpacePanelState {
    #[serde(default)]
    pub active_panel_tab: String,
    #[serde(default)]
    pub workflows: Vec<SpaceProgramEntry>,
    #[serde(default)]
    pub spawned_apps: Vec<SpawnedAppEntry>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_spawned_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpaceProgramEntry {
    pub plugin_id: String,
    pub workflow_step_id: String,
    pub app_id: String,
    pub label: String,
    pub document: Vec<String>,
    pub yields: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpawnedAppEntry {
    pub id: String,
    pub plugin_id: String,
    pub instance_id: u32,
    pub app_id: String,
    pub label: String,
    pub document: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StudioRuntimeState {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_instance_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focused_instance_id: Option<String>,
    #[serde(default)]
    pub selected_media_node_ids: Vec<String>,
    #[serde(default)]
    pub selected_app_instance_ids: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hovered_media_node_id: Option<String>,
    #[serde(default)]
    pub workflow_engagement_input: String,
    #[serde(default)]
    pub compiled_dag_engagement_input: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub space_id: Option<String>,
    #[serde(default)]
    pub clipboard_instance_ids: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_camera: Option<OsWorkflowCamera>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_import_instance_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_import_format: Option<String>,
}
//#endregion 🔖Types
