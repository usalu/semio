//! 🔺️ `change-lab-test-layer-id` diff: patches `layer_id` of the addressed laboratory test row.

use crate::change_lab_test_layer_id::ChangeLabTestLayerId;
use semio_s_artifact_norm_en1997_diff::{En1997Diff, En1997LabTestsPatch, En1997LabTestsRows};
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn diff(payload: &ChangeLabTestLayerId, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    let Some(r0) = base.lab_tests.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("lab-test {} missing", payload.id), vec![payload.id.clone()]);
    };
    if r0.layer_id == payload.new_layer_id {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The laboratory test already has this value.");
    }
    protocol::MutationOutcome::new(En1997Diff { lab_tests: Some(En1997LabTestsRows::modification(&payload.id, En1997LabTestsPatch { layer_id: Some(payload.new_layer_id.clone()), ..Default::default() })), ..Default::default() })
}
