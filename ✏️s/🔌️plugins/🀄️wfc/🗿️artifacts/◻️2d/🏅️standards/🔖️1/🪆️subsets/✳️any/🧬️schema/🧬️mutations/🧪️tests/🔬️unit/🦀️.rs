//! 🧪️ The mutation vocabulary — roster, canonical ordering and op-codec laws.

use crate::mutations::{ordered_index, Wfc2dMutation, KINDS};

/// 🏷️ `KINDS` is exactly the enum's declaration order — the framework never parses Rust, so this is
/// what keeps the catalog, the grammar keyword list and the binary tag order honest against it.
#[test]
fn kinds_are_the_declared_roster() {
    assert_eq!(KINDS.len(), 15);
    assert_eq!(KINDS[0], "change-seed");
    assert_eq!(KINDS[14], "delete-rule");
    let mut sorted = KINDS.to_vec();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(sorted.len(), KINDS.len(), "every kind appears once");
}

/// 🔢 `ordered_index` is the sorted insertion position, which is what makes a create/delete pair
/// point-invertible: the position is a pure function of the id set, never of call order.
#[test]
fn ordered_index_is_the_sorted_position() {
    let ids = ["b", "d", "f"];
    assert_eq!(ordered_index(&ids, "a", |id| id), 0);
    assert_eq!(ordered_index(&ids, "c", |id| id), 1);
    assert_eq!(ordered_index(&ids, "e", |id| id), 2);
    assert_eq!(ordered_index(&ids, "z", |id| id), 3);
}

/// ⚡️ Every kind round-trips through its single-line text op and its binary tag.
#[test]
fn every_mutation_round_trips_through_both_op_codecs() {
    let document = crate::examples::two_room_corridor::document();
    let mutations: Vec<Wfc2dMutation> = vec![
        crate::mutations::change_seed(99),
        crate::mutations::create_slot(crate::schema::snapshot::Wfc2dSlot { id: "room-c".into(), x: 6.0, y: 0.0, width: 2.0, height: 2.0, pinned_tile_id: None }),
        crate::mutations::delete_slot("room-b".into()),
        crate::mutations::move_slot("room-a".into(), 1.0, 2.0),
        crate::mutations::resize_slot("room-a".into(), 3.0, 4.0),
        crate::mutations::connect_slots(crate::schema::snapshot::Wfc2dSlotEdge { id: "edge-x".into(), from_slot_id: "room-a".into(), to_slot_id: "room-b".into(), relation: "adjacent".into() }),
        crate::mutations::disconnect_slots("edge-a-corridor".into()),
        crate::mutations::pin_slot("room-a".into(), "room".into()),
        crate::mutations::unpin_slot("room-a".into()),
        crate::mutations::create_tile(document.tiles[0].clone()),
        crate::mutations::delete_tile("room".into()),
        crate::mutations::change_tile_weight("room".into(), 5.0),
        crate::mutations::change_tile_media("room".into(), document.tiles[0].media.clone()),
        crate::mutations::create_rule(document.rules[0].clone()),
        crate::mutations::delete_rule("rule-room-room".into()),
    ];
    assert_eq!(mutations.len(), KINDS.len());
    for mutation in &mutations {
        let line = protocol::OpText::print_op(mutation);
        let parsed: Wfc2dMutation = protocol::OpText::parse_op(&line).expect("mutation line parses");
        assert_eq!(&parsed, mutation, "text op round trip failed for {line}");
        let bytes = protocol::OpBinary::encode_op(mutation).expect("mutation encodes");
        let decoded: Wfc2dMutation = protocol::OpBinary::decode_op(&bytes).expect("mutation decodes");
        assert_eq!(&decoded, mutation, "binary op round trip failed for {line}");
    }
}
