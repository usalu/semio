
use super::*;

#[test]
fn create_assembly_viewer_builds_a_definition_for_the_viewer_role() {
    let def = create_assembly_viewer();
    assert_eq!(def.role, semio_framework_plugin::AppRole::Viewer);
    assert_eq!(def.dialect, ASSEMBLY_DIALECT.into());
}

#[test]
fn viewer_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<AssemblyViewer as ArtifactViewer>::DIALECT, ASSEMBLY_DIALECT);
}

#[test]
fn viewer_declares_the_structure_window() {
    let def = create_assembly_viewer();
    assert!(def.window_kinds.iter().any(|w| w.id == structure::WINDOW_KIND_ID));
}

#[test]
fn structure_window_is_localized_en_and_de() {
    let def = create_assembly_viewer();
    let window = def.window_kinds.iter().find(|w| w.id == structure::WINDOW_KIND_ID).expect("structure");
    assert_eq!(window.label, semio_framework_plugin::LocalizedLabel::native("Structure", "Struktur"));
}
