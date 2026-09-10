//! 🧪️ Laws of the mutation wire codec: every one of the nine kinds round-trips through BOTH
//! representations, the text keyword set is exactly the catalog's `KINDS`, and a binary tag never
//! decodes as a different kind than the one that encoded it.

use super::{decode_op, encode_op};
use crate::schema::mutations::text::{parse_op, print_op};
use crate::schema::mutations::{change_seed, change_weight, connect_slots, create_rule, create_slot, delete_rule, delete_slot, disconnect_slots, remove_weight, AssemblyMutation, KINDS};
use crate::schema::snapshot::{AssemblyRule, AssemblySlot, AssemblySlotEdge};
use semio_s_artifact_stdio_semio::standards::v1::subsets::value::schema::snapshot::{SemioValue, SemioValueEntry};

fn every_kind() -> Vec<AssemblyMutation> {
    vec![
        create_slot(2, AssemblySlot { id: "c".into(), x: 1.0, y: 2.0, z: 3.0, pinned_module_id: Some("wall".into()) }),
        delete_slot("a".into()),
        create_rule(
            0,
            AssemblyRule {
                id: "r".into(),
                module_a_id: "wall".into(),
                module_b_id: "roof".into(),
                allowed: false,
                params: SemioValue::Map { entries: vec![SemioValueEntry { key: "why".into(), value: SemioValue::Str { value: "structural".into() } }] },
            },
        ),
        delete_rule("r".into()),
        change_weight("wall".into(), 2.75),
        remove_weight("wall".into()),
        connect_slots(1, AssemblySlotEdge { id: "bc".into(), from_slot_id: "b".into(), to_slot_id: "c".into() }),
        disconnect_slots("ab".into()),
        change_seed(99),
    ]
}

#[test]
fn the_fixture_covers_every_declared_kind() {
    assert_eq!(every_kind().len(), KINDS.len());
}

#[test]
fn every_kind_round_trips_through_the_binary_codec() {
    for mutation in every_kind() {
        let bytes = encode_op(&mutation).expect("mutation encodes");
        assert_eq!(decode_op(&bytes).expect("mutation decodes"), mutation);
    }
}

#[test]
fn every_kind_round_trips_through_the_text_codec() {
    for mutation in every_kind() {
        let line = print_op(&mutation);
        assert_eq!(parse_op(&line).expect("mutation line parses"), mutation, "line was: {line}");
    }
}

#[test]
fn every_printed_line_opens_with_a_declared_keyword() {
    for mutation in every_kind() {
        let line = print_op(&mutation);
        assert!(KINDS.iter().any(|kind| line == *kind || line.starts_with(&format!("{kind} "))), "line does not open with a declared kind: {line}");
    }
}

#[test]
fn the_nine_binary_tags_are_pairwise_distinct() {
    let mut tags: Vec<Vec<u8>> = every_kind().iter().map(|mutation| encode_op(mutation).expect("mutation encodes")).collect();
    let before = tags.len();
    tags.sort();
    tags.dedup();
    assert_eq!(tags.len(), before, "two kinds share one binary encoding");
}

#[test]
fn a_line_no_kind_claims_is_refused() {
    assert!(parse_op("teleport-slot id=\"a\"").is_err());
}

#[test]
fn truncated_wire_bytes_are_refused_rather_than_guessed() {
    let bytes = encode_op(&change_seed(1)).expect("mutation encodes");
    assert!(decode_op(&bytes[..bytes.len() / 2]).is_err());
}
