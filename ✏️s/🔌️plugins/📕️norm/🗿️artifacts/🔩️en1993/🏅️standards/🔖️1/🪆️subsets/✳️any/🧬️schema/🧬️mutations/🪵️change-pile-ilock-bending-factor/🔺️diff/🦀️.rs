//! 🔺️ `change-pile-ilock-bending-factor` diff: patches `interlock_bending_factor` of the addressed pile row.

use crate::change_pile_ilock_bending_factor::ChangePileIlockBendingFactor;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993PileDelta, En1993PilePatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangePileIlockBendingFactor, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_interlock_bending_factor.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(row) = base.piles.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "pile", Vec::<String>::new());
    };
    if row.interlock_bending_factor == payload.new_interlock_bending_factor {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The pile already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { piles: En1993PileDelta::modification(payload.id.clone(), En1993PilePatch { interlock_bending_factor: Some(payload.new_interlock_bending_factor), ..Default::default() }), ..Default::default() })
}
