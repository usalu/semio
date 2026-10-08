use super::*;

/// 🚀️ CAD owns its declared kind spellings and relay dialect independently of the OS catalog.
#[test]
fn cad_activation_coordinates_match_defining_artifact_and_relay() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🚀️activation/🔣️.json")).expect("CAD activation corpus");
    assert_eq!(corpus["artifactKinds"][0].as_str(), Some(CAD_DIALECT.artifact_kind));
    assert_eq!(corpus["artifactKinds"][1].as_str(), Some(artifact_kind().id.as_str()));
    let surface = format!("{}#{}", corpus["artifactRef"].as_str().unwrap(), corpus["role"].as_str().unwrap());
    let (dialect, role) = semio_framework::parse_surface_app_id(&surface).expect("CAD relay surface");
    assert_eq!(dialect.artifact_kind, CAD_DIALECT.artifact_kind);
    assert_eq!(dialect.standard, "1");
    assert_eq!(dialect.subset, "*");
    assert_eq!(role, semio_framework::AppRole::Editor);
}
