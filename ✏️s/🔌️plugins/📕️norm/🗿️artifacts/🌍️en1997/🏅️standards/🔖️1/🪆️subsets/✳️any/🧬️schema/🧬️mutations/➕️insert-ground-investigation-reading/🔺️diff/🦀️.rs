use crate::insert_ground_investigation_reading::InsertGroundInvestigationReading;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
use semio_s_artifact_norm_en1997_diff::{En1997Diff, En1997InvestigationsPatch, En1997InvestigationsRows, En1997InvestigationsReadingsRows};

pub fn diff(payload: &InsertGroundInvestigationReading, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    let Some(parent) = base.investigations.iter().find(|row| row.id == payload.investigation_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("laboratory test {} missing", payload.investigation_id), vec![payload.investigation_id.clone()]);
    };
    if parent.readings.iter().any(|existing| existing.id == payload.investigation_reading.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Laboratory test point id {} already exists.", payload.investigation_reading.id), [payload.investigation_reading.id.clone()]);
    }
    if payload.index > parent.readings.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, parent.readings.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1997Diff { investigations: Some(En1997InvestigationsRows::modification(&payload.investigation_id, En1997InvestigationsPatch { readings: En1997InvestigationsReadingsRows::insertion(payload.index, payload.investigation_reading.clone()), ..Default::default() })), ..Default::default() })
}
