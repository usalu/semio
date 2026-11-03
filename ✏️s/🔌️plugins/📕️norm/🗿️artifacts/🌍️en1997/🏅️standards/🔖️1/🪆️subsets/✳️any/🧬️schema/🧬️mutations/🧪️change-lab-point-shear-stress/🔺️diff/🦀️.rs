//! 🔺️ `change-lab-point-shear-stress` diff: patches `shear_stress` of the addressed laboratory test point row.

use crate::change_lab_point_shear_stress::ChangeLabPointShearStress;
use semio_s_artifact_norm_en1997_diff::{En1997Diff, En1997LabTestsPatch, En1997LabTestsPointsPatch, En1997LabTestsPointsRows, En1997LabTestsRows};
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn diff(payload: &ChangeLabPointShearStress, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if !payload.new_shear_stress.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.lab_tests.iter().find(|row| row.id == payload.lab_test_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("lab-test {} missing", payload.lab_test_id), vec![payload.lab_test_id.clone()]);
    };
    let Some(r1) = r0.points.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("lab-point {} missing", payload.id), vec![payload.id.clone()]);
    };
    if r1.shear_stress == payload.new_shear_stress {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The laboratory test point already has this value.");
    }
    protocol::MutationOutcome::new(En1997Diff { lab_tests: Some(En1997LabTestsRows::modification(&payload.lab_test_id, En1997LabTestsPatch { points: En1997LabTestsPointsRows::modification(&payload.id, En1997LabTestsPointsPatch { shear_stress: Some(payload.new_shear_stress), ..Default::default() }), ..Default::default() })), ..Default::default() })
}
