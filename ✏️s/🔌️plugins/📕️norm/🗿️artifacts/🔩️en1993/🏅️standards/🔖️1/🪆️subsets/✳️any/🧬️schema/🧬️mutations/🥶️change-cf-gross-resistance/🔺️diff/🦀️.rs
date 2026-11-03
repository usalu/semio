//! 🔺️ `change-cf-gross-resistance` diff: patches `gross_resistance` of the addressed cold-formed member row.

use crate::change_cf_gross_resistance::ChangeCfGrossResistance;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993ColdFormedMemberDelta, En1993ColdFormedMemberPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeCfGrossResistance, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_gross_resistance.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(row) = base.cold_formed_members.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "cold-formed", Vec::<String>::new());
    };
    if row.gross_resistance == payload.new_gross_resistance {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The cold-formed member already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { cold_formed_members: En1993ColdFormedMemberDelta::modification(payload.id.clone(), En1993ColdFormedMemberPatch { gross_resistance: Some(payload.new_gross_resistance), ..Default::default() }), ..Default::default() })
}
