
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

#[test]
fn structure_window_is_localized_en_and_de() {
    let def = create_assembly_editor();
    let window = def.window_kinds.iter().find(|w| w.id == structure::WINDOW_KIND_ID).expect("structure");
    assert_eq!(window.label, semio_framework_plugin::LocalizedLabel::native("Structure", "Struktur"));
}

#[test]
fn structure_window_declares_the_nine_mutation_actions() {
    let def = create_assembly_editor();
    let window = def.window_kinds.iter().find(|w| w.id == structure::WINDOW_KIND_ID).expect("structure");
    for id in ["create-slot", "delete-slot", "create-rule", "delete-rule", "connect-slots", "disconnect-slots", "change-weight", "remove-weight", "change-seed"] {
        assert!(window.actions.iter().any(|action| action.id == id), "{id}");
    }
}

#[test]
fn editor_command_op_binary_round_trips() {
    let command = AssemblyEditorCommand::ChangeSeed { seed: 99 };
    let bytes = <AssemblyEditorCommand as protocol::OpBinary>::encode_op(&command).expect("encode");
    assert!(!bytes.is_empty());
    assert_eq!(<AssemblyEditorCommand as protocol::OpBinary>::decode_op(&bytes).expect("decode"), command);
}

#[test]
fn bundled_examples_are_non_empty_documents() {
    for source in crate::examples::sources() {
        assert!(!source.document().trim().is_empty(), "{}", source.id());
        let snapshot = <AssemblySnapshot as store::ArtifactDsl>::parse_dsl(source.document()).expect("parse");
        assert!(!snapshot.slots.is_empty(), "{}", source.id());
        let pack = <AssemblySnapshot as store::ArtifactPack>::encode_pack(&snapshot);
        assert!(pack.len() > 64, "{}", source.id());
    }
}
