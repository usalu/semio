//! 🕸️ Sequence play app commands — bulk node-graph edits and viewport pan/zoom.

use crate::apps::sequence::config::{SequenceConfig, SequenceConfigOperation};
use crate::artifacts::sequence::engine::ops_from_host_mutation;
use crate::artifacts::sequence::op::SequenceOperation;
use crate::artifacts::sequence::{SequenceCamera, SequenceFixture};
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};
use serde_json::Value;

//#region 🔖️NodeGraphEdit
pub mod node_graph_edit {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "node-graph-edit")]
    pub struct NodeGraphEdit {
        pub operations_json: String,
    }

    pub fn handle(payload: &NodeGraphEdit, doc: &DocumentView<'_, SequenceFixture>, cfg: &ConfigView<'_, SequenceConfig>) -> Result<Emit<SequenceOperation, SequenceConfigOperation>, Fault> {
        let fixture = doc.projection;
        let sub_operations: Vec<Value> = serde_json::from_str(&payload.operations_json).unwrap_or_default();
        let selected = cfg.projection.selected_step_ids.clone();
        let mut cleared = false;
        let ops = ops_from_host_mutation(fixture, |host| {
            for operation in &sub_operations {
                match operation.get("operation").and_then(|value| value.as_str()).unwrap_or("") {
                    "setFixture" => {
                        if let Some(fixture) = operation.get("fixtureJson").and_then(|value| value.as_str()).and_then(|json| serde_json::from_str::<SequenceFixture>(json).ok()) {
                            let _ = host.replace_fixture(fixture);
                        }
                    }
                    "deleteSelection" => {
                        for step_id in &selected {
                            if host.remove_step(step_id) {
                                cleared = true;
                            }
                        }
                    }
                    "connect" => {
                        let from = operation.get("sourceNodeId").and_then(|value| value.as_str());
                        let to = operation.get("targetNodeId").and_then(|value| value.as_str());
                        if let (Some(from), Some(to)) = (from, to) {
                            let _ = host.connect_steps(from, to);
                        }
                    }
                    _ => {}
                }
            }
        });
        if cleared {
            Ok(Emit { document_operations: ops, config_operations: vec![SequenceConfigOperation::SetSelection { step_ids: Vec::new() }], ..Default::default() })
        } else {
            Ok(Emit::operations(ops))
        }
    }
}
//#endregion 🔖️NodeGraphEdit

//#region 🔖️SetViewport
pub mod set_viewport {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-viewport")]
    pub struct SetViewport {
        #[dsl(block)]
        pub camera: SequenceCamera,
    }

    pub fn handle(payload: &SetViewport, _doc: &DocumentView<'_, SequenceFixture>, _cfg: &ConfigView<'_, SequenceConfig>) -> Result<Emit<SequenceOperation, SequenceConfigOperation>, Fault> {
        Ok(Emit::config(vec![SequenceConfigOperation::SetCamera { camera: payload.camera.clone() }]))
    }
}
//#endregion 🔖️SetViewport

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use crate::apps::sequence::testkit::{dispatch, new_app};
    use crate::apps::sequence::SequenceCommand;
    use crate::artifacts::sequence::SequenceCamera;
    use semio_framework_plugin::{PluginApp, ViewState};
    use serde_json::{json, Value};

    use super::set_viewport::SetViewport;

    /// 🎥️ `SetViewport` is config-only — it must never emit a `SequenceOperation` (no VCS edit, no
    /// undo entry) and instead write straight into the config store.
    #[test]
    fn set_viewport_writes_config_not_operations() {
        let mut app = new_app();
        let result = app.dispatch_typed(SequenceCommand::SetViewport(SetViewport { camera: SequenceCamera { x: 5.0, y: 6.0, zoom: 2.0 } }), &semio_framework_plugin::testkit::meta("local")).expect("viewport pan/zoom");
        assert!(result.operations.is_empty(), "setViewport must not emit a VCS operation");
        let node = app.render(crate::apps::sequence::modes::edit::windows::main::SEQUENCE_PLAY_BODY_MAIN, None, &ViewState::default()).expect("render");
        let payload: Value = serde_json::to_value(&node).unwrap();
        assert_eq!(payload["nodeGraph"]["viewport"]["zoom"], json!(2.0));
    }

    #[test]
    fn node_graph_edit_delete_selection_clears_selection() {
        let mut app = new_app();
        dispatch(&mut app, SequenceCommand::SetSelection(crate::apps::sequence::commands::selection::set_selection::SetSelection { step_ids: vec!["step-1".into()] }));
        dispatch(&mut app, SequenceCommand::NodeGraphEdit(super::node_graph_edit::NodeGraphEdit { operations_json: "[{\"operation\":\"deleteSelection\"}]".into() }));
        assert!(!app.projection().expect("projection").steps.iter().any(|step| step.id == "step-1"));
    }
}
//#endregion 🧪️Tests
