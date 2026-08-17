//! 🪟️ 🧩️ Flow play app commands command — `rename-flow-widget`.

use crate::artifacts::flow::schema::widget_id;
use crate::artifacts::flow::{op::FlowMutation, FlowSnapshot};
use crate::editor::flow::config::{FlowConfig, FlowConfigMutation};
use flow::{FlowEvalSession, Widget};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct RenameFlowWidget {
    pub old_id: String,
    pub value: String,
}

/// ✏️ Renames a widget id (rewiring synapses and layout) purely in the fixture; `None` if the target
/// id is blank, unchanged, or already taken. Operates on the live `flow::FlowFixture` (via
/// `to_fixture`/`from_fixture`) rather than `FlowSnapshot`'s own composed `content` handle.
fn renamed_fixture(snapshot: &FlowSnapshot, old_id: &str, new_id: &str) -> Option<FlowSnapshot> {
    let trimmed = new_id.trim();
    let mut fixture = snapshot.to_fixture();
    if trimmed.is_empty() || trimmed == old_id || fixture.widgets.iter().any(|widget| widget_id(widget) == trimmed) {
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
        fixture.layout.insert(trimmed.into(), layout);
    }
    Some(FlowSnapshot::from_fixture(fixture))
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the renamed widget used to also
/// re-point the selection at its NEW id here; selection is framework-owned `InteractionState` now, only
/// ever mutated by the framework's own injected `interactionSelect` handling — a rename that changes a
/// selected widget's id leaves that id stale in `graph`'s selection (pruned by `interaction_topology` on
/// the next dispatch, same as any other deleted-then-recreated id), an accepted UX regression for this
/// wave (mirrors note's `add-block`/`rename-flow-widget` no longer being able to steer selection).
pub fn handle(payload: &RenameFlowWidget, doc: &ArtifactView<'_, FlowSnapshot>, _cfg: &ConfigView<'_, FlowConfig>, _session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, FlowConfigMutation>, Fault> {
    let fixture = doc.snapshot;
    match renamed_fixture(fixture, &payload.old_id, &payload.value) {
        Some(next) => Ok(Emit::mutations(crate::artifacts::flow::schema::mutations::snapshot_operations(fixture, &next))),
        None => Ok(Emit::default()),
    }
}
