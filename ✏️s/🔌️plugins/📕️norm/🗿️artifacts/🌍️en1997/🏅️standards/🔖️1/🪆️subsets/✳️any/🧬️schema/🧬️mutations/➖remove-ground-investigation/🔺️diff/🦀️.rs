use crate::remove_ground_investigation::RemoveGroundInvestigation;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
use semio_s_artifact_norm_en1997_diff::{En1997Diff, En1997InvestigationsRows};

pub fn diff(payload: &RemoveGroundInvestigation, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if payload.index >= base.investigations.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("investigation #{} missing", payload.index), vec![payload.index.to_string()]);
    }
    protocol::MutationOutcome::new(En1997Diff { investigations: Some(En1997InvestigationsRows::removal(&base.investigations, payload.index)), ..Default::default() })
}
