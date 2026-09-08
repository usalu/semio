//! 🕸️ Sequence play app commands — bulk node-graph edits and viewport pan/zoom.

use crate::mutations::SequenceMutation;
use crate::{SequenceCamera, SequenceSnapshot};
use crate::editor::sequence::config::{SequenceConfig, SequenceConfigMutation};
use crate::editor::sequence::ops_from_host_mutation;
use semio_framework_plugin::{app::InteractionView, ArtifactView, ConfigView, Emit, Fault};
use serde_json::Value;
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️NodeGraphEdit
pub mod node_graph_edit {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "node-graph-edit")]
    pub struct NodeGraphEdit {
        pub operations_json: String,
    }

    fn edit_with_selection(payload: &NodeGraphEdit, fixture: &SequenceSnapshot, selected: &[String]) -> Emit<SequenceMutation, SequenceConfigMutation> {
        let sub_operations: Vec<Value> = serde_json::from_str(&payload.operations_json).unwrap_or_default();
        let ops = ops_from_host_mutation(fixture, |host| {
            for operation in &sub_operations {
                match operation.get("operation").and_then(|value| value.as_str()).unwrap_or("") {
                    "setFixture" => {
                        if let Some(fixture) = operation.get("fixtureJson").and_then(|value| value.as_str()).and_then(|json| dsl::os_pack::from_json_str::<crate::SequenceFixture>(json).ok()) {
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

    /// 🕹️ `app_commands!`'s generated `dispatch(doc, cfg).await` is framework-fixed at this exact 3-arg
    /// shape (no `interaction` slot — ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) —
    /// reachable only through that macro-generated path (`SequencePlayApp::handle` always routes this
    /// command through `apply` below instead), so its `"deleteSelection"` sub-operation degrades to
    /// treating the selection as empty; every other sub-operation (`setFixture`/`connect`) is unaffected.
    pub fn handle(payload: &NodeGraphEdit, doc: &ArtifactView<'_, SequenceSnapshot>, _cfg: &ConfigView<'_, SequenceConfig>) -> Result<Emit<SequenceMutation, SequenceConfigMutation>, Fault> {
        Ok(edit_with_selection(payload, doc.snapshot, &[]))
    }

    pub fn apply(payload: &NodeGraphEdit, doc: &ArtifactView<'_, SequenceSnapshot>, _cfg: &ConfigView<'_, SequenceConfig>, interaction: &InteractionView<'_>) -> Result<Emit<SequenceMutation, SequenceConfigMutation>, Fault> {
        Ok(edit_with_selection(payload, doc.snapshot, &interaction.selection(crate::editor::sequence::SEQUENCE_INTERACTION_STEPS).ids))
    }
}
//#endregion 🔖️NodeGraphEdit

//#region 🔖️SetViewport
pub mod set_viewport {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "set-viewport")]
    pub struct SetViewport {
        #[dsl(block)]
        pub camera: SequenceCamera,
    }

    pub fn handle(payload: &SetViewport, _doc: &ArtifactView<'_, SequenceSnapshot>, _cfg: &ConfigView<'_, SequenceConfig>) -> Result<Emit<SequenceMutation, SequenceConfigMutation>, Fault> {
        Ok(Emit::config(vec![SequenceConfigMutation::SetCamera(crate::editor::sequence::config::SetCamera { camera: payload.camera.clone() })]))
    }
}
//#endregion 🔖️SetViewport

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use crate::SequenceCamera;
    use crate::editor::sequence::testkit::{dispatch, new_app, new_app_with_registry_wired, select_steps};
    use crate::editor::sequence::SequenceCommand;
    use semio_framework_plugin::{PluginApp, ViewModel};

    use super::set_viewport::SetViewport;

    /// 🎥️ `SetViewport` is config-only — it must never emit a `SequenceMutation` (no VCS edit, no
    /// undo entry) and instead write straight into the config store.
    #[semio_framework_async_macros::async_test]
    async fn set_viewport_writes_config_not_operations() {
        let mut app = new_app().await;
        let result = app.dispatch_typed(SequenceCommand::SetViewport(SetViewport { camera: SequenceCamera { x: 5.0, y: 6.0, zoom: 2.0 } }), &semio_framework_plugin::testkit::meta("local")).await.expect("viewport pan/zoom");
        assert!(result.mutations.is_empty(), "setViewport must not emit a VCS operation");
        let node = app.render(crate::editor::sequence::modes::edit::windows::main::SEQUENCE_PLAY_BODY_MAIN, None, &ViewModel::default()).await.expect("render");
        let semio_framework_plugin::Component::Surface(props) = &node.root.component else { panic!("semantic graph") };
        let scene: semio_framework_plugin::NodeGraphScene = semio_framework_ui_scene::decode(props).expect("packed graph");
        assert_eq!(scene.viewport.expect("camera viewport").zoom, 2.0);
        semio_framework_plugin::testkit::project_and_retire_fixture_tree(node).expect("retire viewport graph");
    }

    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: picking is now the framework's
    /// injected `interactionSelect` verb against the "steps" domain — requires a registry-wired app
    /// (see `select_steps`'s own doc comment) so `NodeGraphEdit::apply` (not the macro-dispatched
    /// `handle`, which always sees an empty selection) reads the live selection.
    #[semio_framework_async_macros::async_test]
    async fn node_graph_edit_delete_selection_clears_selection() {
        let mut app = new_app_with_registry_wired().await;
        select_steps(&mut app, &["step-1"]).await;
        dispatch(&mut app, SequenceCommand::NodeGraphEdit(super::node_graph_edit::NodeGraphEdit { operations_json: "[{\"operation\":\"deleteSelection\"}]".into() })).await;
        assert!(!app.snapshot().expect("projection").to_fixture().steps.iter().any(|step| step.id == "step-1"));
    }
}
//#endregion 🧪️Tests
