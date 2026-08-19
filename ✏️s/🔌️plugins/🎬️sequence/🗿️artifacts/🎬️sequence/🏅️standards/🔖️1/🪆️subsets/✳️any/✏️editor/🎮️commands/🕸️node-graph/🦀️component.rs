//! 🕸️ Sequence play app commands — bulk node-graph edits and viewport pan/zoom.

use crate::editor::sequence::config::{SequenceConfig, SequenceConfigMutation};
use crate::editor::sequence::ops_from_host_mutation;
use crate::artifacts::sequence::mutations::SequenceMutation;
use crate::artifacts::sequence::{SequenceCamera, SequenceSnapshot};
use semio_framework_plugin::{app::InteractionView, ConfigView, ArtifactView, Emit, Fault};
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

    async fn edit_with_selection(payload: &NodeGraphEdit, fixture: &SequenceSnapshot, selected: &[String]) -> Emit<SequenceMutation, SequenceConfigMutation> {
        let sub_operations: Vec<Value> = serde_json::from_str(&payload.operations_json).unwrap_or_default();
        let ops = ops_from_host_mutation(fixture, |host| {
            for operation in &sub_operations {
                match operation.get("operation").and_then(|value| value.as_str()).unwrap_or("") {
                    "setFixture" => {
                        if let Some(fixture) = operation.get("fixtureJson").and_then(|value| value.as_str()).and_then(|json| serde_json::from_str::<crate::artifacts::sequence::SequenceFixture>(json).ok()) {
                            let _ = host.replace_snapshot(fixture);
                        }
                    }
                    "deleteSelection" => {
                        for step_id in selected {
                            host.remove_step(step_id);
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
        Emit::mutations(ops)
    }

    /// 🕹️ `app_commands!`'s generated `dispatch(doc, cfg)` is framework-fixed at this exact 3-arg
    /// shape (no `interaction` slot — ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) —
    /// reachable only through that macro-generated path (`SequencePlayApp::handle` always routes this
    /// command through `apply` below instead), so its `"deleteSelection"` sub-operation degrades to
    /// treating the selection as empty; every other sub-operation (`setFixture`/`connect`) is unaffected.
    pub async fn handle(payload: &NodeGraphEdit, doc: &ArtifactView<'_, SequenceSnapshot>, _cfg: &ConfigView<'_, SequenceConfig>) -> Result<Emit<SequenceMutation, SequenceConfigMutation>, Fault> {
        Ok(edit_with_selection(payload, doc.snapshot, &[]))
    }

    pub async fn apply(payload: &NodeGraphEdit, doc: &ArtifactView<'_, SequenceSnapshot>, _cfg: &ConfigView<'_, SequenceConfig>, interaction: &InteractionView<'_>) -> Result<Emit<SequenceMutation, SequenceConfigMutation>, Fault> {
        Ok(edit_with_selection(payload, doc.snapshot, &interaction.selection(crate::editor::sequence::SEQUENCE_INTERACTION_STEPS).ids))
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

    pub async fn handle(payload: &SetViewport, _doc: &ArtifactView<'_, SequenceSnapshot>, _cfg: &ConfigView<'_, SequenceConfig>) -> Result<Emit<SequenceMutation, SequenceConfigMutation>, Fault> {
        Ok(Emit::config(vec![SequenceConfigMutation::SetCamera { camera: payload.camera.clone() }]))
    }
}
//#endregion 🔖️SetViewport

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use crate::editor::sequence::testkit::{dispatch, new_app, new_app_with_registry_wired, select_steps};
    use crate::editor::sequence::SequenceCommand;
    use crate::artifacts::sequence::SequenceCamera;
    use semio_framework_plugin::{PluginApp, ViewModel};
    use serde_json::{json, Value};

    use super::set_viewport::SetViewport;

    /// 🎥️ `SetViewport` is config-only — it must never emit a `SequenceMutation` (no VCS edit, no
    /// undo entry) and instead write straight into the config store.
    #[semio_framework_async_macros::async_test]
    async fn set_viewport_writes_config_not_operations() {
        let mut app = new_app();
        let result = app.dispatch_typed(SequenceCommand::SetViewport(SetViewport { camera: SequenceCamera { x: 5.0, y: 6.0, zoom: 2.0 } }), &semio_framework_plugin::testkit::meta("local")).expect("viewport pan/zoom");
        assert!(result.mutations.is_empty(), "setViewport must not emit a VCS operation");
        let node = app.render(crate::editor::sequence::modes::edit::windows::main::SEQUENCE_PLAY_BODY_MAIN, None, &ViewModel::default()).expect("render");
        let payload: Value = serde_json::to_value(&node).unwrap();
        assert_eq!(payload["nodeGraph"]["viewport"]["zoom"], json!(2.0));
    }

    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: picking is now the framework's
    /// injected `interactionSelect` verb against the "steps" domain — requires a registry-wired app
    /// (see `select_steps`'s own doc comment) so `NodeGraphEdit::apply` (not the macro-dispatched
    /// `handle`, which always sees an empty selection) reads the live selection.
    #[semio_framework_async_macros::async_test]
    async fn node_graph_edit_delete_selection_clears_selection() {
        let mut app = new_app_with_registry_wired();
        select_steps(&mut app, &["step-1"]);
        dispatch(&mut app, SequenceCommand::NodeGraphEdit(super::node_graph_edit::NodeGraphEdit { operations_json: "[{\"operation\":\"deleteSelection\"}]".into() }));
        assert!(!app.snapshot().expect("projection").to_fixture().steps.iter().any(|step| step.id == "step-1"));
    }
}
//#endregion 🧪️Tests
