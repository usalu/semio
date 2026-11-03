use crate::insert_lab_point::InsertLabPoint;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
use semio_s_artifact_norm_en1997_diff::{En1997Diff, En1997LabTestsPatch, En1997LabTestsRows, En1997LabTestsPointsRows};

pub fn diff(payload: &InsertLabPoint, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    let Some(parent) = base.lab_tests.iter().find(|row| row.id == payload.lab_test_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("laboratory test {} missing", payload.lab_test_id), vec![payload.lab_test_id.clone()]);
    };
    if parent.points.iter().any(|existing| existing.id == payload.lab_point.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Laboratory test point id {} already exists.", payload.lab_point.id), [payload.lab_point.id.clone()]);
    }
    if payload.index > parent.points.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, parent.points.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1997Diff { lab_tests: Some(En1997LabTestsRows::modification(&payload.lab_test_id, En1997LabTestsPatch { points: En1997LabTestsPointsRows::insertion(payload.index, payload.lab_point.clone()), ..Default::default() })), ..Default::default() })
}
