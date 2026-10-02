//! 🔺️ `change-widget-input` sparse diff — replaces the ONE addressed operator with its input set to the typed literal
//! (or the addressed text source with its text set), read off the BASE widget through [`ChangeWidgetInput::landing`].

use crate::standards::v1::subsets::any::schema::diff::{diff_fixture_from_helpers, Generation3dDiff, LayoutDiff, SynapsesDiff, WidgetsDiff};
use crate::standards::v1::subsets::any::schema::mutations::change_widget_input::ChangeWidgetInput;
use crate::standards::v1::subsets::any::schema::mutations::widget_index;
use crate::Generation3dSnapshot;

/// 🏗️ A payload outside its schema bounds (empty address, overlong name or text, non-finite number) is
/// `mutation.invariant`; a missing widget is `target-missing`; a widget without this input, an input a wire drives, or one
/// holding a literal of another type is `target-mismatch`; the value the input already holds is `no-op`.
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
        Err(mismatch) => protocol::MutationOutcome::error("mutation.target-mismatch", format!("Input \"{}\" of \"{}\" {}.", payload.channel, payload.id, mismatch.reason()), target()),
        Ok(None) => protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", format!("Input \"{}\" of \"{}\" already holds this value.", payload.channel, payload.id)).at(target())]),
        Ok(Some(next)) => {
            let widgets = WidgetsDiff { removed: Vec::new(), set: vec![(index, next)] };
            let diff = diff_fixture_from_helpers(base, &widgets, &SynapsesDiff::default(), &LayoutDiff::default(), None, None);
            for (_, widget) in widgets.set {
                widget.retire_cold();
            }
            protocol::MutationOutcome::new(diff)
        }
    }
}
