
use super::*;

#[test]
fn create_assembly_editor_builds_a_definition_for_the_editor_role() {
    let def = create_assembly_editor();
    assert_eq!(def.role, semio_framework_plugin::AppRole::Editor);
    assert_eq!(def.dialect, ASSEMBLY_DIALECT.into());
}

#[test]
fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<AssemblyEditor as ArtifactEditor>::DIALECT, ASSEMBLY_DIALECT);
}

#[test]
fn editor_declares_the_structure_window() {
    let def = create_assembly_editor();
    assert!(def.window_kinds.iter().any(|w| w.id == structure::WINDOW_KIND_ID));
}
