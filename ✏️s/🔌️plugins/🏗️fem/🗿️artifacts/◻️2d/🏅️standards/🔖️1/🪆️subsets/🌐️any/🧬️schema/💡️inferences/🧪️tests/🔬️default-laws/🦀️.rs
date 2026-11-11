//! 🧪️ The fem2d inferences of an empty snapshot equal their defaults.

use semio_os_kernel::Inference;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::inferences::bounds::{compute_fem2d_bounds, Fem2dBounds};
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::inferences::Fem2dInference;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot;

#[test]
fn inference_default_law() {
    assert_eq!(Fem2dInference::infer(&Fem2dSnapshot::default()).expect("valid materialized inference fixture"), Fem2dInference::default());
}

#[test]
fn bounds_default_law() {
    assert_eq!(compute_fem2d_bounds(&Fem2dSnapshot::default()), Fem2dBounds::default());
}
