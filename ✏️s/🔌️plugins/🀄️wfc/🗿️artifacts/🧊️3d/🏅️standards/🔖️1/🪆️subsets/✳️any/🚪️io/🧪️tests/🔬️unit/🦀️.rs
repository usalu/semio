//! 🚪️ Native codec, stdio.txt hop declaration, and txt leaf for `s.wfc.wfc3d`.

use super::{export_stdio_kinds, import_stdio_kinds, io};
use crate::examples::{tower_stack, two_room_corridor, wall_roof_facade_strip};
use crate::mutations::{self as mutation_builders, Wfc3dMutation};
use crate::schema::snapshot::Wfc3dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::{binary as mutation_binary, text as mutation_text};
use crate::standards::v1::subsets::any::schema::snapshot::{binary as snapshot_binary, text as snapshot_text};
use protocol::{OpBinary, OpText};
use store::ArtifactDsl;

fn documents() -> [Wfc3dSnapshot; 3] {
    [two_room_corridor::snapshot(), wall_roof_facade_strip::snapshot(), tower_stack::snapshot()]
}

#[test]
fn io_declares_stdio_txt_and_a_native_codec() {
    assert_eq!(import_stdio_kinds(), &["stdio.txt"]);
    assert_eq!(export_stdio_kinds(), &["stdio.txt"]);
    let declaration = io();
    assert!(declaration.entries.is_empty(), "foreign hops ride the composer registry, not IoDeclaration.entries");
    assert!(declaration.native.snapshot.text.is_some());
    assert!(declaration.native.snapshot.binary.is_some());
    assert!(declaration.native.mutations.text.is_some());
    assert!(declaration.native.mutations.binary.is_some());
    assert!(declaration.native.diff.text.is_none());
    assert!(declaration.native.inferences.is_none());
}

#[test]
fn native_snapshot_text_round_trips_every_example() {
    for document in documents() {
        let printed = snapshot_text::print_dsl(&document);
        let parsed = snapshot_text::parse_dsl(&printed).unwrap_or_else(|error| panic!("seed {}: {error:?}", document.seed));
        assert_eq!(parsed, document);
        assert_eq!(<Wfc3dSnapshot as ArtifactDsl>::print_dsl(&document), printed);
    }
}

#[test]
fn native_snapshot_pack_round_trips_every_example() {
    for document in documents() {
        let bytes = snapshot_binary::encode(&document);
        let parsed = snapshot_binary::decode(&bytes).unwrap_or_else(|error| panic!("seed {}: {error:?}", document.seed));
        assert_eq!(parsed, document);
    }
}

#[test]
fn native_mutation_text_and_binary_round_trip_change_seed() {
    let operation = mutation_builders::change_seed(99);
    let line = mutation_text::print_op(&operation);
    assert_eq!(mutation_text::parse_op(&line).expect("op text parses"), operation);
    let bytes = mutation_binary::encode_op(&operation).expect("op binary encodes");
    assert_eq!(mutation_binary::decode_op(&bytes).expect("op binary decodes"), operation);
    assert_eq!(<Wfc3dMutation as OpText>::print_op(&operation), line);
    assert_eq!(
        <Wfc3dMutation as OpBinary>::decode_op(&<Wfc3dMutation as OpBinary>::encode_op(&operation).expect("encode")).expect("decode"),
        operation
    );
}

#[test]
fn stdio_txt_leaf_round_trips_tower_stack() {
    let document = tower_stack::snapshot();
    let printed = snapshot_text::print_dsl(&document);
    let parsed = crate::standards::v1::subsets::any::io::import::deserializers::artifacts::txt::v_utf_8::any::deserialize_bytes(printed.as_bytes()).expect("txt leaf parses");
    assert_eq!(parsed, document);
}
