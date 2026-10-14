//! 🔺️ `change-widget-input` sparse diff — replaces the ONE addressed operator with its input set to the typed literal
//! (or the addressed text source with its text set), read off the BASE widget through [`ChangeWidgetInput::landing`].

use semio_s_artifact_procedural_3d_model::diff::Generation3dDiff;
use semio_s_artifact_procedural_3d_model::diff::Generation3dWidgetPatch;
use semio_s_artifact_procedural_3d_model::diff::Generation3dWidgetModification;
use semio_s_artifact_procedural_3d_model::diff::Generation3dWidgetsDelta;
use crate::change_widget_input::ChangeWidgetInput;
use crate::widget_index;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;

/// 🏗️ A payload outside its schema bounds (empty address, overlong name or text, non-finite number) is
/// `mutation.invariant`; a missing widget or an input its record does not hold is `target-missing`; an input a wire drives,
/// or one holding a literal of another type or shape, is `target-mismatch`; the value the input already holds is `no-op`.
pub fn diff(payload: &ChangeWidgetInput, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
    let target = || [payload.id.clone()];
    if !payload.admissible() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Input \"{}\" of \"{}\" breaks its schema bounds.", payload.channel, payload.id), target());
    }
    let Some(index) = widget_index(&base.host_snapshot, &payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Widget \"{}\" does not exist.", payload.id), target());
    };
    let wired = base.host_snapshot.synapses.iter().any(|synapse| synapse.to == payload.id && synapse.to_port == payload.channel);
    match payload.landing(&base.host_snapshot.widgets[index], wired) {
        Err(refusal) => protocol::MutationOutcome::error(refusal.code(), format!("Input \"{}\" of \"{}\" {}.", payload.channel, payload.id, refusal.reason()), target()),
        Ok(None) => protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", format!("Input \"{}\" of \"{}\" already holds this value.", payload.channel, payload.id)).at(target())]),
        Ok(Some(next)) => protocol::MutationOutcome::new(Generation3dDiff { widgets: Some(Generation3dWidgetsDelta { modified: vec![Generation3dWidgetModification { id: payload.id.clone(), patch: Generation3dWidgetPatch::Replace { widget: next } }], ..Default::default() }), ..Default::default() }),
    }
}
