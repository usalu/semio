//! 🔺️ `change-lab-test-void-ratio` diff: patches `void_ratio` of the addressed laboratory test row.

use crate::change_lab_test_void_ratio::ChangeLabTestVoidRatio;
use semio_s_artifact_norm_en1997_diff::{En1997Diff, En1997LabTestsPatch, En1997LabTestsRows};
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn diff(payload: &ChangeLabTestVoidRatio, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if !payload.new_void_ratio.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.lab_tests.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("lab-test {} missing", payload.id), vec![payload.id.clone()]);
    };
    if r0.void_ratio == payload.new_void_ratio {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The laboratory test already has this value.");
    }
    protocol::MutationOutcome::new(En1997Diff { lab_tests: Some(En1997LabTestsRows::modification(&payload.id, En1997LabTestsPatch { void_ratio: Some(payload.new_void_ratio), ..Default::default() })), ..Default::default() })
}
