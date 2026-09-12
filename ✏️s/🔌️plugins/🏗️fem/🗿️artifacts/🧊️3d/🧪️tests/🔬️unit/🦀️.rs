use super::*;
use crate::model::Dof;

#[test]
fn fem_dof_round_trips_through_core_dof() {
    for dof in FemDof::ALL {
        assert_eq!(FemDof::from(Dof::from(dof)), dof);
    }
}

#[test]
fn fem_analysis_settings_default_matches_pre_migration_values() {
    let settings = FemAnalysisSettings::default();
    assert_eq!(settings.modal_count, 3);
    assert_eq!(settings.buckling_count, 3);
    assert_eq!(settings.deformation_scale, 50.0);
}

#[test]
fn fem_initial_viewport_has_a_valid_explicit_pose() {
    assert!(crate::viewport::INITIAL.validate().is_ok());
    assert_eq!(crate::viewport::INITIAL.position, [4.0, -4.0, 3.0]);
}

#[test]
fn computation_artifact_kind_matches_computation_fem3d() {
    let kind = computation_artifact_kind();
    assert_eq!(kind.id, "computation.fem3d");
    assert_eq!(kind.component_kind, "fem3d-results");
    assert_eq!(kind.schema, "computation.fem3d");
}
