use crate::insert_ground_investigation::InsertGroundInvestigation;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
use semio_s_artifact_norm_en1997_diff::{En1997Diff, En1997InvestigationsRows};

pub fn diff(payload: &InsertGroundInvestigation, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if base.investigations.iter().any(|existing| existing.id == payload.investigation.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Investigation id {} already exists.", payload.investigation.id), [payload.investigation.id.clone()]);
    }
    if payload.index > base.investigations.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, base.investigations.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1997Diff { investigations: Some(En1997InvestigationsRows::insertion(payload.index, payload.investigation.clone())), ..Default::default() })
}
