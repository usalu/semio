//! 🔺️ `change-pile-group-shaft-coh-sum` diff: patches `shaft_cohesion_sum` of the addressed pile group row.

use crate::change_pile_group_shaft_coh_sum::ChangePileGroupShaftCohSum;
use semio_s_artifact_norm_en1997_diff::{En1997Diff, En1997PileGroupsPatch, En1997PileGroupsRows};
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn diff(payload: &ChangePileGroupShaftCohSum, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if !payload.new_shaft_cohesion_sum.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.pile_groups.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("pile-group {} missing", payload.id), vec![payload.id.clone()]);
    };
    if r0.shaft_cohesion_sum == payload.new_shaft_cohesion_sum {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The pile group already has this value.");
    }
    protocol::MutationOutcome::new(En1997Diff { pile_groups: Some(En1997PileGroupsRows::modification(&payload.id, En1997PileGroupsPatch { shaft_cohesion_sum: Some(payload.new_shaft_cohesion_sum), ..Default::default() })), ..Default::default() })
}
