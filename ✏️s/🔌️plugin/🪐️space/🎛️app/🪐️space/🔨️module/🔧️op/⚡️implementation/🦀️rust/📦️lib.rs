//! ⚡️ S Studio app — operation enum + laws (constitutional: op).
//!
//! 🕳️ Deviation from the constitutional split recipe's usual "op" content: the Studio app's DOCUMENT
//! operation is `semio_framework_os::OsOperation`, owned entirely outside this plugin (see this crate's
//! sibling doc comments). What this crate DOES own is `SpaceConfigOperation` — space's real,
//! locally-defined `DocumentApp::ConfigOperation`, the B1 config-artifact pilot's op-crate content —
//! there is no `OsOperation` twin for config, config is genuinely local to this app.

use space::SpaceWindowCamera;
use space_engine::SpaceConfig;
use protocol::Operation;
use serde::{Deserialize, Serialize};

//#region 🔖️ConfigOperations
/// @emoji 🧮️ B1: `space_engine::SpaceConfig`'s operation enum — one variant per settled interaction
/// (mirrors the pre-B1 `StudioRuntimeState` field writes plus the deleted `ViewState.panel_json`-backed
/// `SpacePanelState.active_panel_tab`), plus a generic `Snapshot` every variant's `backwards()` returns:
/// a config-only dispatch is a plain `Apply` (not an `AmendLast`), so each tick is its own distinct, real
/// config edit and "undo this tick" is exactly "restore the whole-config snapshot from just before it" —
/// mirrors `shooting_op::ShootingConfigOperation`'s identical design (see its doc comment for the full
/// rationale). `Operation::Diff` is the WHOLE `SpaceConfig`, not a granular patch type.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum SpaceConfigOperation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: SpaceConfig,
    },
    /// 👁️ Selected workflow-node ids — was `selected_media_node_ids`/`selected_app_instance_ids`.
    #[dsl(key = "selection")]
    SetSelection { node_ids: Vec<String> },
    #[dsl(key = "hover")]
    SetHover { node_id: Option<String> },
    #[dsl(key = "active-node")]
    SetActiveNode { node_id: Option<String> },
    #[dsl(key = "focused-node")]
    SetFocusedNode { node_id: Option<String> },
    #[dsl(key = "clipboard")]
    SetClipboard { node_ids: Vec<String> },
    #[dsl(key = "collapsed")]
    SetCollapsed { node_ids: Vec<String> },
    #[dsl(key = "preview-off")]
    SetPreviewOff { node_ids: Vec<String> },
    /// 🎥️ Sets one window's workflow camera — window-instance-keyed (today always
    /// `space::S_PLAY_WINDOW_WORKFLOW`, see `SpaceConfig.camera`'s doc).
    #[dsl(key = "camera")]
    SetCamera {
        window_id: String,
        #[dsl(block)]
        camera: SpaceWindowCamera,
    },
    #[dsl(key = "workflow-engagement-input")]
    SetWorkflowEngagementInput { value: String },
    #[dsl(key = "compiled-dag-engagement-input")]
    SetCompiledDagEngagementInput { value: String },
    #[dsl(key = "pending-import")]
    SetPendingImport { node_id: Option<String>, format: Option<String> },
    #[dsl(key = "space-id")]
    SetSpaceId { space_id: Option<String> },
    #[dsl(key = "client")]
    SetClient { client_id: Option<String>, client_name: Option<String> },
    #[dsl(key = "active-panel-tab")]
    SetActivePanelTab { tab_id: String },
    #[dsl(key = "locale")]
    SetLocale { value: String },
}

impl Operation<SpaceConfig> for SpaceConfigOperation {
    type Diff = SpaceConfig;

    fn diff(&self, base: &SpaceConfig) -> SpaceConfig {
        let mut next = base.clone();
        match self {
            SpaceConfigOperation::Snapshot { config } => return config.clone(),
            SpaceConfigOperation::SetSelection { node_ids } => next.selected_node_ids = node_ids.clone(),
            SpaceConfigOperation::SetHover { node_id } => next.hovered_node_id = node_id.clone(),
            SpaceConfigOperation::SetActiveNode { node_id } => next.active_node_id = node_id.clone(),
            SpaceConfigOperation::SetFocusedNode { node_id } => next.focused_node_id = node_id.clone(),
            SpaceConfigOperation::SetClipboard { node_ids } => next.clipboard_node_ids = node_ids.clone(),
            SpaceConfigOperation::SetCollapsed { node_ids } => next.collapsed_node_ids = node_ids.clone(),
            SpaceConfigOperation::SetPreviewOff { node_ids } => next.preview_off_node_ids = node_ids.clone(),
            SpaceConfigOperation::SetCamera { window_id, camera } => {
                next.camera.insert(window_id.clone(), *camera);
            }
            SpaceConfigOperation::SetWorkflowEngagementInput { value } => next.workflow_engagement_input = value.clone(),
            SpaceConfigOperation::SetCompiledDagEngagementInput { value } => next.compiled_dag_engagement_input = value.clone(),
            SpaceConfigOperation::SetPendingImport { node_id, format } => {
                next.pending_import_node_id = node_id.clone();
                next.pending_import_format = format.clone();
            }
            SpaceConfigOperation::SetSpaceId { space_id } => next.space_id = space_id.clone(),
            SpaceConfigOperation::SetClient { client_id, client_name } => {
                next.client_id = client_id.clone();
                next.client_name = client_name.clone();
            }
            SpaceConfigOperation::SetActivePanelTab { tab_id } => next.active_panel_tab = tab_id.clone(),
            SpaceConfigOperation::SetLocale { value } => next.locale = value.clone(),
        }
        next
    }

    fn backwards(&self, base: &SpaceConfig) -> Vec<Self> {
        vec![SpaceConfigOperation::Snapshot { config: base.clone() }]
    }
}
//#endregion 🔖️ConfigOperations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn round_trip(config: &SpaceConfig, operation: &SpaceConfigOperation) -> SpaceConfig {
        let forward = vcs::apply_operation(config, operation);
        let backwards = operation.backwards(config);
        let mut restored = forward.clone();
        for back in &backwards {
            restored = vcs::apply_operation(&restored, back);
        }
        assert_eq!(&restored, config, "backwards() must exactly restore the pre-operation config");
        forward
    }

    #[test]
    fn set_selection_round_trips() {
        let config = SpaceConfig::default();
        let operation = SpaceConfigOperation::SetSelection { node_ids: vec!["node-1".into(), "node-2".into()] };
        let next = round_trip(&config, &operation);
        assert_eq!(next.selected_node_ids, vec!["node-1".to_string(), "node-2".to_string()]);
    }

    #[test]
    fn set_camera_round_trips_and_keys_by_window_id() {
        let config = SpaceConfig::default();
        let camera = SpaceWindowCamera { x: 12.0, y: -4.0, zoom: 2.0 };
        let operation = SpaceConfigOperation::SetCamera { window_id: space::S_PLAY_WINDOW_WORKFLOW.into(), camera };
        let next = round_trip(&config, &operation);
        assert_eq!(next.camera.get(space::S_PLAY_WINDOW_WORKFLOW), Some(&camera));
    }

    #[test]
    fn set_active_panel_tab_round_trips() {
        let config = SpaceConfig::default();
        let operation = SpaceConfigOperation::SetActivePanelTab { tab_id: space::S_PLAY_PARAMETERS_TAB_ID.into() };
        let next = round_trip(&config, &operation);
        assert_eq!(next.active_panel_tab, space::S_PLAY_PARAMETERS_TAB_ID);
    }

    #[test]
    fn space_config_op_text_round_trips_every_variant() {
        store::test_support::assert_op_line_round_trip(&SpaceConfigOperation::Snapshot { config: SpaceConfig::default() });
        store::test_support::assert_op_line_round_trip(&SpaceConfigOperation::SetSelection { node_ids: vec!["a".into(), "b".into()] });
        store::test_support::assert_op_line_round_trip(&SpaceConfigOperation::SetHover { node_id: Some("a".into()) });
        store::test_support::assert_op_line_round_trip(&SpaceConfigOperation::SetHover { node_id: None });
        store::test_support::assert_op_line_round_trip(&SpaceConfigOperation::SetActiveNode { node_id: Some("a".into()) });
        store::test_support::assert_op_line_round_trip(&SpaceConfigOperation::SetFocusedNode { node_id: None });
        store::test_support::assert_op_line_round_trip(&SpaceConfigOperation::SetClipboard { node_ids: vec!["a".into()] });
        store::test_support::assert_op_line_round_trip(&SpaceConfigOperation::SetCollapsed { node_ids: vec!["a".into()] });
        store::test_support::assert_op_line_round_trip(&SpaceConfigOperation::SetPreviewOff { node_ids: vec!["a".into()] });
        store::test_support::assert_op_line_round_trip(&SpaceConfigOperation::SetCamera { window_id: "s-workflow".into(), camera: SpaceWindowCamera { x: 1.0, y: 2.0, zoom: 3.0 } });
        store::test_support::assert_op_line_round_trip(&SpaceConfigOperation::SetWorkflowEngagementInput { value: "draw draw".into() });
        store::test_support::assert_op_line_round_trip(&SpaceConfigOperation::SetCompiledDagEngagementInput { value: "".into() });
        store::test_support::assert_op_line_round_trip(&SpaceConfigOperation::SetPendingImport { node_id: Some("a".into()), format: Some("dwg".into()) });
        store::test_support::assert_op_line_round_trip(&SpaceConfigOperation::SetPendingImport { node_id: None, format: None });
        store::test_support::assert_op_line_round_trip(&SpaceConfigOperation::SetSpaceId { space_id: Some("demo".into()) });
        store::test_support::assert_op_line_round_trip(&SpaceConfigOperation::SetClient { client_id: Some("c1".into()), client_name: Some("Ada".into()) });
        store::test_support::assert_op_line_round_trip(&SpaceConfigOperation::SetActivePanelTab { tab_id: "s-play-catalogue".into() });
        store::test_support::assert_op_line_round_trip(&SpaceConfigOperation::SetLocale { value: "de".into() });
    }
}
//#endregion 🧪️Tests
