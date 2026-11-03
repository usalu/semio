//! 🔺️ `change-ground-investigation-kind` diff: patches `kind` of the addressed ground investigation row.

use crate::change_ground_investigation_kind::ChangeGroundInvestigationKind;
use semio_s_artifact_norm_en1997_diff::{En1997Diff, En1997InvestigationsPatch, En1997InvestigationsRows};
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn diff(payload: &ChangeGroundInvestigationKind, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    let Some(r0) = base.investigations.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("ground-investigation {} missing", payload.id), vec![payload.id.clone()]);
    };
    if r0.kind == payload.new_kind {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The ground investigation already has this value.");
    }
    protocol::MutationOutcome::new(En1997Diff { investigations: Some(En1997InvestigationsRows::modification(&payload.id, En1997InvestigationsPatch { kind: Some(payload.new_kind), ..Default::default() })), ..Default::default() })
}
