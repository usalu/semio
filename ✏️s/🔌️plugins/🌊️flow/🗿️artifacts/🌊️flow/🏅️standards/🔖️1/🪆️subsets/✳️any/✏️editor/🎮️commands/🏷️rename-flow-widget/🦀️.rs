//! 🪟️ 🧩️ Flow play app commands command — `rename-flow-widget`.

use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use crate::{FlowMutation, FlowSnapshot};
use flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
pub struct RenameFlowWidget {
    pub old_id: String,
    pub value: String,
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the renamed widget used to also
/// re-point the selection at its NEW id here; selection is framework-owned `InteractionState` now, only
/// ever mutated by the framework's own injected `interactionSelect` handling — a rename that changes a
/// selected widget's id leaves that id stale in `graph`'s selection (pruned by `interaction_topology` on
/// the next dispatch, same as any other deleted-then-recreated id), an accepted UX regression for this
/// wave (mirrors note's `add-block`/`rename-flow-widget` no longer being able to steer selection).
pub fn handle(payload: &RenameFlowWidget, doc: &ArtifactView<'_, FlowSnapshot>, _cfg: &ConfigView<'_, NoConfig>, _session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, NoConfigMutation>, Fault> {
    rename_edit(payload, &crate::flow_composed_snapshot(doc.snapshot, &doc.children)?)
}

/// 🏷️ The rename — the one body the batch `handle` above and the retained `FlowGraphOperationWork` route both run, so
/// neither can drift from the other (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). The widget's synapse endpoints and
/// layout entry follow it, the result is published on the content child, and a missing widget or an empty or taken
/// new id is refused by name.
pub fn rename_edit(payload: &RenameFlowWidget, composed: &FlowSnapshot) -> Result<Emit<FlowMutation, NoConfigMutation>, Fault> {
    let refuse = |code: &str, message: String| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new(code), message);
    let mut edit = crate::editor::flow::edit_rules::ContentEdit::new(crate::editor::flow::flow_composed_content(composed)?);
    if !edit.content.nodes.iter().any(|node| node.id == payload.old_id) {
        return Err(refuse("mutation.target-missing", format!("renameFlowWidget found no widget \"{}\"", payload.old_id)));
    }
    if !edit.rename_node(&payload.old_id, payload.value.trim()) {
        return Err(refuse("flow.widget-id-unavailable", format!("renameFlowWidget cannot rename \"{}\" to \"{}\": the id is empty or taken", payload.old_id, payload.value.trim())));
    }
    Ok(crate::editor::flow::flow_content_leaves_emit(&composed.content.child_id, edit.leaves))
}
