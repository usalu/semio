//! 🔺️ `change-uplift-total-stress` diff: patches `total_stress` of the addressed uplift case row.

use crate::change_uplift_total_stress::ChangeUpliftTotalStress;
use semio_s_artifact_norm_en1997_diff::{En1997Diff, En1997UpliftCasesPatch, En1997UpliftCasesRows};
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn diff(payload: &ChangeUpliftTotalStress, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if !payload.new_total_stress.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.uplift_cases.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("uplift {} missing", payload.id), vec![payload.id.clone()]);
    };
    if r0.total_stress == payload.new_total_stress {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The uplift case already has this value.");
    }
    protocol::MutationOutcome::new(En1997Diff { uplift_cases: Some(En1997UpliftCasesRows::modification(&payload.id, En1997UpliftCasesPatch { total_stress: Some(payload.new_total_stress), ..Default::default() })), ..Default::default() })
}
