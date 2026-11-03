//! 🔺️ `change-load-case-moment-permanent` diff: patches `moment_permanent` of the addressed footing load case row.

use crate::change_load_case_moment_permanent::ChangeLoadCaseMomentPermanent;
use semio_s_artifact_norm_en1997_diff::{En1997Diff, En1997FootingsLoadCasesPatch, En1997FootingsLoadCasesRows, En1997FootingsPatch, En1997FootingsRows};
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn diff(payload: &ChangeLoadCaseMomentPermanent, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if !payload.new_moment_permanent.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.footings.iter().find(|row| row.id == payload.footing_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("footing {} missing", payload.footing_id), vec![payload.footing_id.clone()]);
    };
    let Some(r1) = r0.load_cases.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("load-case {} missing", payload.id), vec![payload.id.clone()]);
    };
    if r1.moment_permanent == payload.new_moment_permanent {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The footing load case already has this value.");
    }
    protocol::MutationOutcome::new(En1997Diff { footings: Some(En1997FootingsRows::modification(&payload.footing_id, En1997FootingsPatch { load_cases: En1997FootingsLoadCasesRows::modification(&payload.id, En1997FootingsLoadCasesPatch { moment_permanent: Some(payload.new_moment_permanent), ..Default::default() }), ..Default::default() })), ..Default::default() })
}
