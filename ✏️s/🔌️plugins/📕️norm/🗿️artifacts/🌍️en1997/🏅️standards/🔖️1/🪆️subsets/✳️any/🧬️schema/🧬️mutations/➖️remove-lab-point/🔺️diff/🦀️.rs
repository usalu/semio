use crate::remove_lab_point::RemoveLabPoint;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
use semio_s_artifact_norm_en1997_diff::{En1997Diff, En1997LabTestsPatch, En1997LabTestsRows, En1997LabTestsPointsRows};

pub fn diff(payload: &RemoveLabPoint, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    let Some(parent) = base.lab_tests.iter().find(|row| row.id == payload.lab_test_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("laboratory test {} missing", payload.lab_test_id), vec![payload.lab_test_id.clone()]);
    };
    if payload.index >= parent.points.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("lab-point #{} missing", payload.index), vec![payload.index.to_string()]);
    }
    protocol::MutationOutcome::new(En1997Diff { lab_tests: Some(En1997LabTestsRows::modification(&payload.lab_test_id, En1997LabTestsPatch { points: En1997LabTestsPointsRows::removal(&parent.points, payload.index), ..Default::default() })), ..Default::default() })
}
