//! 🔺️ `change-ground-investigation-position-y` diff: patches `position_y` of the addressed ground investigation row.

use crate::change_ground_investigation_position_y::ChangeGroundInvestigationPositionY;
use semio_s_artifact_norm_en1997_diff::{En1997Diff, En1997InvestigationsPatch, En1997InvestigationsRows};
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn diff(payload: &ChangeGroundInvestigationPositionY, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if !payload.new_position_y.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.investigations.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("ground-investigation {} missing", payload.id), vec![payload.id.clone()]);
    };
    if r0.position_y == payload.new_position_y {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The ground investigation already has this value.");
    }
    protocol::MutationOutcome::new(En1997Diff { investigations: Some(En1997InvestigationsRows::modification(&payload.id, En1997InvestigationsPatch { position_y: Some(payload.new_position_y), ..Default::default() })), ..Default::default() })
}
