//! 🔺️ `change-ground-investigation-level` diff: patches `ground_level` of the addressed ground investigation row.

use crate::change_ground_investigation_level::ChangeGroundInvestigationLevel;
use semio_s_artifact_norm_en1997_diff::{En1997Diff, En1997InvestigationsPatch, En1997InvestigationsRows};
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn diff(payload: &ChangeGroundInvestigationLevel, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if !payload.new_ground_level.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.investigations.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("ground-investigation {} missing", payload.id), vec![payload.id.clone()]);
    };
    if r0.ground_level == payload.new_ground_level {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The ground investigation already has this value.");
    }
    protocol::MutationOutcome::new(En1997Diff { investigations: Some(En1997InvestigationsRows::modification(&payload.id, En1997InvestigationsPatch { ground_level: Some(payload.new_ground_level), ..Default::default() })), ..Default::default() })
}
