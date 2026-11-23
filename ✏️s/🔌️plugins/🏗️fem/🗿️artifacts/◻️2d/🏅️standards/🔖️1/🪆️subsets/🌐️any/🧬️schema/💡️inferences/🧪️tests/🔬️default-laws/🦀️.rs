//! 🧪️ The fem2d inferences of an empty snapshot equal their defaults.

use semio_os_kernel::Inference;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::inferences::bounds::{compute_fem2d_bounds, Fem2dBounds};
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::inferences::Fem2dInference;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot;

#[test]
fn inference_default_law() {
    let snapshot = Fem2dSnapshot::default();
    let inferred = Fem2dInference::infer(&snapshot).expect("valid materialized inference fixture");
    semio_framework_value::FromValue::retire_decoded(snapshot);
    assert_eq!(inferred, Fem2dInference::default());
}

#[test]
fn bounds_default_law() {
    let snapshot = Fem2dSnapshot::default();
    let bounds = compute_fem2d_bounds(&snapshot);
    semio_framework_value::FromValue::retire_decoded(snapshot);
    assert_eq!(bounds, Fem2dBounds::default());
}
