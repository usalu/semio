use crate::insert_lab_test::InsertLabTest;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
use semio_s_artifact_norm_en1997_diff::{En1997Diff, En1997LabTestsRows};

pub fn diff(payload: &InsertLabTest, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if base.lab_tests.iter().any(|existing| existing.id == payload.lab_test.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("LabTest id {} already exists.", payload.lab_test.id), [payload.lab_test.id.clone()]);
    }
    if payload.index > base.lab_tests.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, base.lab_tests.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1997Diff { lab_tests: Some(En1997LabTestsRows::insertion(payload.index, payload.lab_test.clone())), ..Default::default() })
}
