//! 🔺️ `change-lab-test-design-life` diff: patches `design_life` of the addressed laboratory test row.

use crate::change_lab_test_design_life::ChangeLabTestDesignLife;
use semio_s_artifact_norm_en1997_diff::{En1997Diff, En1997LabTestsPatch, En1997LabTestsRows};
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn diff(payload: &ChangeLabTestDesignLife, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if !payload.new_design_life.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.lab_tests.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("lab-test {} missing", payload.id), vec![payload.id.clone()]);
    };
    if r0.design_life == payload.new_design_life {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The laboratory test already has this value.");
    }
    protocol::MutationOutcome::new(En1997Diff { lab_tests: Some(En1997LabTestsRows::modification(&payload.id, En1997LabTestsPatch { design_life: Some(payload.new_design_life), ..Default::default() })), ..Default::default() })
}
