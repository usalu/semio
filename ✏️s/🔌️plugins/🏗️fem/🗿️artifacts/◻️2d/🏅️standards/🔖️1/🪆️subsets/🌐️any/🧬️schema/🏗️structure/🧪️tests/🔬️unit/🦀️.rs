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

#[test]
fn the_physics_fingerprint_ignores_display_only_settings_and_matches_the_language_agnostic_vectors() {
    let vectors: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔏️physics-fingerprint/🔣️.json")).unwrap();
    for case in vectors["cases"].as_array().unwrap() {
        let settings = FemAnalysisSettings { modal_count: case["modalCount"].as_u64().unwrap() as usize, buckling_count: case["bucklingCount"].as_u64().unwrap() as usize, deformation_scale: case["deformationScale"].as_f64().unwrap() };
        assert_eq!(format!("{:016x}", settings.physics_fingerprint()), case["fingerprint"].as_str().unwrap(), "{case}");
    }
    let declared: Vec<&str> = FemAnalysisSettings::display_only_fields().collect();
    assert_eq!(declared, vectors["displayOnly"].as_array().unwrap().iter().map(|name| name.as_str().unwrap()).collect::<Vec<_>>());
    let names: Vec<&str> = FemAnalysisSettings::ANALYSIS_FIELDS.iter().map(|field| field.name).collect();
    assert_eq!(names, vectors["fields"].as_array().unwrap().iter().map(|name| name.as_str().unwrap()).collect::<Vec<_>>());
}

#[test]
fn the_display_only_fields_are_the_ones_the_schema_declares() {
    let schema: serde_json::Value = serde_json::from_str(include_str!("../../../🔣️.json")).unwrap();
    let properties = schema["$defs"]["FemAnalysisSettings"]["properties"].as_object().unwrap();
    let declared: Vec<&str> = properties.iter().filter(|(_, property)| property["x-semio-display-only"] == true).map(|(name, _)| name.as_str()).collect();
    assert_eq!(declared, FemAnalysisSettings::display_only_fields().collect::<Vec<_>>());
    for field in FemAnalysisSettings::ANALYSIS_FIELDS {
        assert!(properties.contains_key(field.name), "{}", field.name);
    }
    assert_eq!(properties.len(), FemAnalysisSettings::ANALYSIS_FIELDS.len());
}
