//! 🕸️ Sequence play app commands — bulk node-graph edits and viewport pan/zoom.

use semio_framework_plugin::{NoConfig, NoConfigMutation};
use crate::editor::sequence::sequence_child_emit_from_host_mutation;
use crate::mutations::SequenceMutation;
use crate::{SequenceCamera, SequenceSnapshot};
use semio_framework_plugin::{app::InteractionView, ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};
use serde_json::Value;

//#region 🔖️NodeGraphEdit
pub mod node_graph_edit {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "node-graph-edit")]
    pub struct NodeGraphEdit {
        pub operations_json: String,
    }

    fn edit_with_selection(payload: &NodeGraphEdit, doc: &ArtifactView<'_, SequenceSnapshot>, selected: &[String]) -> Result<Emit<SequenceMutation, NoConfigMutation>, Fault> {
        let sub_operations: Vec<Value> = serde_json::from_str(&payload.operations_json).unwrap_or_default();
        sequence_child_emit_from_host_mutation(doc, |host| {
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
        })
    }

    /// 🕹️ `app_commands!`'s generated `dispatch(doc, cfg).await` is framework-fixed at this exact 3-arg
    /// shape (no `interaction` slot — ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) —
    /// reachable only through that macro-generated path (`SequencePlayApp::handle` always routes this
    /// command through `apply` below instead), so its `"deleteSelection"` sub-operation degrades to
    /// treating the selection as empty; every other sub-operation (`setFixture`/`connect`) is unaffected.
    pub fn handle(payload: &NodeGraphEdit, doc: &ArtifactView<'_, SequenceSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<SequenceMutation, NoConfigMutation>, Fault> {
        edit_with_selection(payload, doc, &[])
    }

    pub fn apply(payload: &NodeGraphEdit, doc: &ArtifactView<'_, SequenceSnapshot>, _cfg: &ConfigView<'_, NoConfig>, interaction: &InteractionView<'_>) -> Result<Emit<SequenceMutation, NoConfigMutation>, Fault> {
        edit_with_selection(payload, doc, &interaction.selection(crate::editor::sequence::SEQUENCE_INTERACTION_STEPS).ids)
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

    pub fn handle(_payload: &SetViewport, _doc: &ArtifactView<'_, SequenceSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<SequenceMutation, NoConfigMutation>, Fault> {
        Ok(Emit::default())
    }
}
//#endregion 🔖️SetViewport

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
