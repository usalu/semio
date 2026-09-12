//! Language-agnostic assembly mount contract — apps, window kinds, codecs, examples.

use crate::examples::{two_room_corridor, wall_roof_facade_strip};
use crate::schema::snapshot::text::{parse_dsl, print_dsl};
use crate::{AssemblySnapshot, ASSEMBLY_DIALECT};
use store::ArtifactPack;

#[test]
fn editor_and_viewer_share_the_assembly_dialect() {
    assert_eq!(<crate::editor::assembly::AssemblyEditor as semio_framework_plugin::ArtifactEditor>::DIALECT, ASSEMBLY_DIALECT);
    assert_eq!(<crate::viewer::assembly::AssemblyViewer as semio_framework_plugin::ArtifactViewer>::DIALECT, ASSEMBLY_DIALECT);
}

#[test]
fn editor_declares_the_structure_window_kind() {
    let def = crate::editor::assembly::create_assembly_editor();
    assert_eq!(def.id, "s.assembly@1/*#editor");
    assert!(def.window_kinds.iter().any(|window| window.id == "framework.window.tree"));
}

#[test]
fn examples_round_trip_dsl_and_pack() {
    for snapshot in [two_room_corridor::snapshot(), wall_roof_facade_strip::snapshot()] {
        let text = print_dsl(&snapshot);
        assert!(!text.trim().is_empty());
        assert_eq!(parse_dsl(&text).expect("dsl"), snapshot);
        let pack = ArtifactPack::encode_pack(&snapshot);
        assert!(pack.len() > 64);
        assert_eq!(<AssemblySnapshot as ArtifactPack>::decode_pack(&pack).expect("pack"), snapshot);
    }
}

#[test]
fn example_labels_are_localized_en_and_de() {
    assert_eq!(two_room_corridor::label(), semio_framework_plugin::LocalizedLabel::native("Two Rooms And A Corridor", "Zwei Räume und ein Korridor"));
    assert_eq!(wall_roof_facade_strip::label(), semio_framework_plugin::LocalizedLabel::native("Wall And Roof Facade Strip", "Wand-Dach-Fassadenstreifen"));
}
