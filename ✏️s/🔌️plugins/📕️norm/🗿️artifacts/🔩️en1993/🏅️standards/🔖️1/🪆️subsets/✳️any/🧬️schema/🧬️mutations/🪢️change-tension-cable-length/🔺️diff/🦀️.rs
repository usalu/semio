//! 🔺️ `change-tension-cable-length` diff: patches `cable_length` of the addressed tension component row.

use crate::change_tension_cable_length::ChangeTensionCableLength;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993TensionComponentDelta, En1993TensionComponentPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeTensionCableLength, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_cable_length.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(row) = base.tension_components.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "tension", Vec::<String>::new());
    };
    if row.cable_length == payload.new_cable_length {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The tension component already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { tension_components: En1993TensionComponentDelta::modification(payload.id.clone(), En1993TensionComponentPatch { cable_length: Some(payload.new_cable_length), ..Default::default() }), ..Default::default() })
}
