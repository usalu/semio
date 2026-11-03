use crate::remove_lab_test::RemoveLabTest;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
use semio_s_artifact_norm_en1997_diff::{En1997Diff, En1997LabTestsRows};

pub fn diff(payload: &RemoveLabTest, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if payload.index >= base.lab_tests.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("lab-test #{} missing", payload.index), vec![payload.index.to_string()]);
    }
    protocol::MutationOutcome::new(En1997Diff { lab_tests: Some(En1997LabTestsRows::removal(&base.lab_tests, payload.index)), ..Default::default() })
}
