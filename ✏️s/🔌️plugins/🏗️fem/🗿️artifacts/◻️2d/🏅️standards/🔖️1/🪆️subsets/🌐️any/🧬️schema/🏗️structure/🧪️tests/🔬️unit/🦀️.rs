use crate::model::standards::v1::subsets::any::schema::structure::{FemAnalysisSettings, FemDof};
use semio_s_fem_model::Dof;

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
