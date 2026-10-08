//! 🚪️ Native codec and empty foreign-hop declaration for `s.wfc.wfc2d`.

use super::{export_stdio_kinds, import_stdio_kinds, io};
use crate::examples;
use crate::standards::v1::subsets::any::io::text::{mutations, snapshot};
use crate::{Wfc2dMutation, Wfc2dSnapshot};
use protocol::{OpBinary, OpText};
use store::ArtifactDsl;

#[test]
fn io_declares_no_foreign_stdio_hops() {
    assert!(import_stdio_kinds().is_empty());
    assert!(export_stdio_kinds().is_empty());
    let declaration = io();
    assert!(declaration.entries.is_empty(), "wfc2d has no foreign composer entries");
    assert!(declaration.native.snapshot.text.is_some());
    assert!(declaration.native.snapshot.binary.is_some());
    assert!(declaration.native.mutations.text.is_some());
    assert!(declaration.native.mutations.binary.is_some());
    assert!(declaration.native.diff.text.is_some());
    assert!(declaration.native.inferences.is_none());
}

#[test]
fn native_snapshot_text_round_trips_every_example() {
    for document in examples::documents() {
        let printed = crate::standards::v1::subsets::any::io::text::snapshot::print_dsl(&document);
        let parsed = crate::standards::v1::subsets::any::io::text::snapshot::parse_dsl(&printed).unwrap_or_else(|error| panic!("seed {}: {error:?}", document.seed));
        assert_eq!(parsed, document);
        assert_eq!(<Wfc2dSnapshot as ArtifactDsl>::print_dsl(&document), printed);
    }
}

#[test]
fn native_snapshot_pack_round_trips_every_example() {
    for document in examples::documents() {
        let bytes = crate::standards::v1::subsets::any::io::binary::snapshot::encode(&document);
        let parsed = crate::standards::v1::subsets::any::io::binary::snapshot::decode(&bytes).unwrap_or_else(|error| panic!("seed {}: {error:?}", document.seed));
        assert_eq!(parsed, document);
    }
}

#[test]
fn native_mutation_text_and_binary_round_trip_change_seed() {
    let operation = crate::schema::mutations::change_seed::change_seed(99);
    let line = crate::standards::v1::subsets::any::io::text::mutations::print_op(&operation);
    assert_eq!(crate::standards::v1::subsets::any::io::text::mutations::parse_op(&line).expect("op text parses"), operation);
    let bytes = crate::standards::v1::subsets::any::io::binary::mutations::encode_op(&operation).expect("op binary encodes");
    assert_eq!(crate::standards::v1::subsets::any::io::binary::mutations::decode_op(&bytes).expect("op binary decodes"), operation);
    assert_eq!(<Wfc2dMutation as OpText>::print_op(&operation), line);
    assert_eq!(
        <Wfc2dMutation as OpBinary>::decode_op(&<Wfc2dMutation as OpBinary>::encode_op(&operation).expect("encode")).expect("decode"),
        operation
    );
}
