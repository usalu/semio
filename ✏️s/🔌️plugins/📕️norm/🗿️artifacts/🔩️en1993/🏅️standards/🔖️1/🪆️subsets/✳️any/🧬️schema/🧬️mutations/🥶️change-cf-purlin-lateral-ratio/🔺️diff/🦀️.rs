//! 🔺️ `change-cf-purlin-lateral-ratio` diff: patches `purlin_lateral_ratio` of the addressed cold-formed member row.

use crate::change_cf_purlin_lateral_ratio::ChangeCfPurlinLateralRatio;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993ColdFormedMemberDelta, En1993ColdFormedMemberPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeCfPurlinLateralRatio, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_purlin_lateral_ratio.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(row) = base.cold_formed_members.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "cold-formed", Vec::<String>::new());
    };
    if row.purlin_lateral_ratio == payload.new_purlin_lateral_ratio {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The cold-formed member already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { cold_formed_members: En1993ColdFormedMemberDelta::modification(payload.id.clone(), En1993ColdFormedMemberPatch { purlin_lateral_ratio: Some(payload.new_purlin_lateral_ratio), ..Default::default() }), ..Default::default() })
}
