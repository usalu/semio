//! 🪟️ 🧩️ Flow play app commands command — `rename-flow-widget`.

use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use crate::schema::widget_id;
use crate::{op::FlowMutation, FlowSnapshot};
use flow::FlowEvalSession;
use semio_framework_artifact_flow_flow::Widget;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
pub struct RenameFlowWidget {
    pub old_id: String,
    pub value: String,
}

/// ✏️ Renames a widget id (rewiring synapses and layout) purely in the fixture; `None` if the target
/// id is blank, unchanged, or already taken. Operates on the live `semio_framework_artifact_flow_flow::FlowHostSnapshot` (via
/// `to_host_snapshot`/`from_host_snapshot`) rather than `FlowSnapshot`'s own composed `content` handle.
fn renamed_fixture(snapshot: &FlowSnapshot, old_id: &str, new_id: &str) -> Option<FlowSnapshot> {
    let trimmed = new_id.trim();
    let mut fixture = snapshot.to_host_snapshot();
    if trimmed.is_empty() || trimmed == old_id || fixture.widgets.iter().any(|widget| widget_id(widget) == trimmed) {
        fixture.retire_cold();
        return None;
    }
    for widget in fixture.widgets.iter_mut() {
        if widget_id(widget) == old_id {
            match widget {
                Widget::Neuron { id, .. }
                | Widget::InputSlider { id, .. }
                | Widget::InputNote { id, .. }
                | Widget::InputImage { id, .. }
                | Widget::Variable { id, .. }
                | Widget::OutputPreview { id, .. }
                | Widget::OutputAction { id, .. }
                | Widget::OutputExport { id, .. }
                | Widget::Cluster { id, .. } => *id = trimmed.to_string(),
            }
        }
    }
    for synapse in fixture.synapses.iter_mut() {
        if synapse.from == old_id {
            synapse.from = trimmed.into();
        }
        if synapse.to == old_id {
            synapse.to = trimmed.into();
        }
    }
    if let Some(layout) = fixture.layout.remove(old_id) {
        fixture.layout.insert(trimmed.into(), (*layout).clone());
    }
    Some(FlowSnapshot::from_host_snapshot(fixture))
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
    if !crate::flow_working_scene(composed).widgets.iter().any(|widget| widget_id(widget) == payload.old_id) {
        return Err(refuse("mutation.target-missing", format!("renameFlowWidget found no widget \"{}\"", payload.old_id)));
    }
    let next = renamed_fixture(composed, &payload.old_id, &payload.value).ok_or_else(|| refuse("flow.widget-id-unavailable", format!("renameFlowWidget cannot rename \"{}\" to \"{}\": the id is empty or taken", payload.old_id, payload.value.trim())))?;
    let scene = crate::flow_working_scene(&next);
    crate::editor::flow::flow_scene_publication(composed, &scene.widgets, &scene.synapses, &scene.layout)
}
