//! 🔺️ `change-tension-partial-factor` diff: patches `partial_factor` of the addressed tension component row.

use crate::change_tension_partial_factor::ChangeTensionPartialFactor;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993TensionComponentDelta, En1993TensionComponentPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeTensionPartialFactor, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_partial_factor.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(row) = base.tension_components.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "tension", Vec::<String>::new());
    };
    if row.partial_factor == payload.new_partial_factor {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The tension component already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { tension_components: En1993TensionComponentDelta::modification(payload.id.clone(), En1993TensionComponentPatch { partial_factor: Some(payload.new_partial_factor), ..Default::default() }), ..Default::default() })
}
