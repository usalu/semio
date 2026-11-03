//! 🔺️ `change-tension-pin-gap` diff: patches `pin_gap` of the addressed tension component row.

use crate::change_tension_pin_gap::ChangeTensionPinGap;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993TensionComponentDelta, En1993TensionComponentPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeTensionPinGap, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_pin_gap.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(row) = base.tension_components.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "tension", Vec::<String>::new());
    };
    if row.pin_gap == payload.new_pin_gap {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The tension component already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { tension_components: En1993TensionComponentDelta::modification(payload.id.clone(), En1993TensionComponentPatch { pin_gap: Some(payload.new_pin_gap), ..Default::default() }), ..Default::default() })
}
