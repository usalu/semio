use crate::remove_ground_investigation_reading::RemoveGroundInvestigationReading;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
use semio_s_artifact_norm_en1997_diff::{En1997Diff, En1997InvestigationsPatch, En1997InvestigationsRows, En1997InvestigationsReadingsRows};

pub fn diff(payload: &RemoveGroundInvestigationReading, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    let Some(parent) = base.investigations.iter().find(|row| row.id == payload.investigation_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("laboratory test {} missing", payload.investigation_id), vec![payload.investigation_id.clone()]);
    };
    if payload.index >= parent.readings.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("investigation-reading #{} missing", payload.index), vec![payload.index.to_string()]);
    }
    protocol::MutationOutcome::new(En1997Diff { investigations: Some(En1997InvestigationsRows::modification(&payload.investigation_id, En1997InvestigationsPatch { readings: En1997InvestigationsReadingsRows::removal(&parent.readings, payload.index), ..Default::default() })), ..Default::default() })
}
