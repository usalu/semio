//! 🎚️ Persisted local configuration for one exact Flow main window.

#[path = "🧬️schema/🦀️.rs"]
mod schema;
pub use schema::*;

use std::collections::HashMap;
use semio_framework_artifact_flow_flow::CameraJson;

impl FlowMainWindowConfig {
    pub fn automation_enabled(&self) -> HashMap<String, bool> {
        serde_json::from_str(&self.automation_enabled_json).unwrap_or_default()
    }
}








impl ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot for FlowMainWindowConfig {

}

semio_framework_os_kernel::config_record! {
    record: FlowMainWindowConfig,
    diff: FlowMainWindowConfigDiff,
    set: FlowMainWindowConfigMutation,
    owner: "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌊️main",
    payload_schema: "flow.mainwindowconfig",
    emoji: "🎚️",
    fields: {
        preview_off_node_ids: Vec<String> => SetPreviewOffNodeIds "set-preview-off-node-ids",
        camera: CameraJson => SetCamera "set-camera",
        lod_mode: String => SetLodMode "set-lod-mode",
        proximity_distance: f64 => SetProximityDistance "set-proximity-distance",
        grid_visible: bool => SetGridVisible "set-grid-visible",
        grid_snap_enabled: bool => SetGridSnapEnabled "set-grid-snap-enabled",
        grid_factor: f64 => SetGridFactor "set-grid-factor",
        catalogue_sections_json: String => SetCatalogueSectionsJson "set-catalogue-sections-json",
        automation_enabled_json: String => SetAutomationEnabledJson "set-automation-enabled-json",
    },
}





pub struct FlowMainWindowConfigOwner;

impl semio_framework_plugin::WindowConfigOwner for FlowMainWindowConfigOwner {
    const WINDOW_KIND_ID: &'static str = crate::editor::flow::editor_vocabulary::FLOW_PLAY_WINDOW_MAIN;
    const SCHEMA: &'static str = "flow.mainwindowconfig";
    const MAXIMUM_PUBLICATION_BYTES: usize = 65_536;
    type State = FlowMainWindowConfig;
    type Mutation = FlowMainWindowConfigMutation;
    type Edit = semio_framework_plugin::app::WindowConfigApplyEdit<FlowMainWindowConfig, FlowMainWindowConfigMutation>;
    const MAXIMUM_PREPARATION_DEPTH: usize = 64;

    fn build_retained_edit() -> std::sync::Arc<Self::Edit> {
        std::sync::Arc::new(semio_framework_plugin::app::WindowConfigApplyEdit::new())
    }

    fn build_store_owners() -> Result<store::os_store::document_store_owners::DocumentStoreOwners<Self::State, Self::Mutation>, semio_framework_value::ValueError> {
        semio_framework_plugin::bounded_window_config_store_owners::<Self>()
    }
    fn build_one_item_preparation_factory() -> std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::State, Self::Mutation>> {
        semio_framework_plugin::bounded_window_config_preparation_factory::<Self>()
    }
    fn build_store_disposer() -> Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::State, Self::Mutation>>> {
        semio_framework_plugin::bounded_window_config_store_disposer::<Self>()
    }
}

pub fn current<C>(view: &semio_framework_plugin::ConfigView<'_, C>) -> FlowMainWindowConfig {
    view.window::<FlowMainWindowConfigOwner>().cloned().unwrap_or_default()
}

pub fn from_snapshot(snapshot: Option<&semio_framework_plugin::WindowConfigSnapshot>) -> FlowMainWindowConfig {
    snapshot.and_then(|snapshot| snapshot.get::<FlowMainWindowConfigOwner>()).cloned().unwrap_or_default()
}

pub fn addressed(view: &semio_framework_plugin::ViewModel, base: &FlowMainWindowConfig, config: FlowMainWindowConfig) -> Result<Vec<semio_framework_plugin::WindowConfigMutation>, semio_framework_plugin::Fault> {
    let id = view.window_id.as_deref().ok_or_else(|| semio_framework_plugin::Fault::from("flow-main-window-required"))?;
    let kind = view.window_instances.iter().find(|window| window.id == id).map(|window| window.window_kind_id.as_str());
    if kind != Some(crate::editor::flow::editor_vocabulary::FLOW_PLAY_WINDOW_MAIN) {
        return Err(semio_framework_plugin::Fault::from("flow-main-window-kind-required"));
    }
    Ok(FlowMainWindowConfigMutation::setting(base, &config).into_iter().map(|mutation| semio_framework_plugin::WindowConfigMutation::of::<FlowMainWindowConfigOwner>(id, mutation)).collect())
}



#[path="🚪️io/📝️text/🧬️mutations/🦀️.rs"]
mod field_set_text_io;

#[path="🚪️io/💾️binary/🧬️mutations/🦀️.rs"]
mod field_set_binary_io;

#[path="🚪️io/📝️text/📸️snapshot/🦀️.rs"]
mod text_snapshot_io;

#[path="🚪️io/💾️binary/📸️snapshot/🦀️.rs"]
mod binary_snapshot_io;
