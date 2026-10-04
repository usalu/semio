//! 🧪️ The mutation aggregate's own laws: the kind roster is honest against the enum, every verb is
//! approved, and EVERY kind's `inverse()` restores its pre-mutation document exactly.

use super::*;
use crate::diff::Wfc3dDiff;
use crate::schema::snapshot::{Color, GraphRule, Slot3d, SlotEdge, Tile, TileMedia3d, Wfc3dSnapshot};
use protocol::SemanticMutation;
use vcs::apply_mutation;

//#region 🧸️Fixtures
/// 🧸️ The document every case in this file and every committed fixture quintet starts from — the
/// `two-room-corridor` example, so a reader can see the same three slots in the DSL asset.
pub fn fixture_base() -> Wfc3dSnapshot {
    crate::examples::two_room_corridor::snapshot()
}

/// 🧸️ The same document with `room-a` pinned, for the cases that need an existing pin to release.
pub fn fixture_base_pinned() -> Wfc3dSnapshot {
    let mut snapshot = fixture_base();
    let index = snapshot.slots.iter().position(|slot| slot.id == "room-a").expect("room-a exists");
    snapshot.slots[index].pinned_tile_id = Some("room".into());
    snapshot
}

/// 🧸️ Every committed case, as `(mutation directory, case directory, base, mutation)`. The fixture
/// generator and the aggregate round-trip law read the SAME table, so a case can never exist in one
/// and be missing from the other.
pub fn fixture_cases() -> Vec<(&'static str, &'static str, Wfc3dSnapshot, Wfc3dMutation)> {
    let stair = Tile { id: "stair".into(), label: Some("Stair".into()), weight: 1.0, media: crate::unit_wedge_media(Some(Color { r: 150, g: 150, b: 150, a: 255 })) };
    vec![
        ("🎲️change-seed", "🎲️reseeds", fixture_base(), change_seed(99)),
        (
            "🧩️create-slot",
            "🧩️inserts",
            fixture_base(),
            create_slot(crate::schema::snapshot::canonical_slot_index(&fixture_base(), "room-c"), Slot3d { id: "room-c".into(), x: 3.0, y: 0.0, z: 0.0, width: 1.0, height: 1.0, depth: 1.0, pinned_tile_id: None }),
        ),
        ("🕳️delete-slot", "🕳️removes", fixture_base(), delete_slot("room-a".into())),
        ("🚚️move-slot", "🚚️lifts-room-b-one-storey", fixture_base(), move_slot("room-b".into(), 2.0, 1.0, 0.0)),
        ("📐️resize-slot", "📐️widens", fixture_base(), resize_slot("room-a".into(), 2.0, 1.0, 1.0)),
        (
            "🔗️connect-slots",
            "🔗️joins",
            fixture_base(),
            connect_slots(crate::schema::snapshot::canonical_edge_index(&fixture_base(), "edge-a-b"), SlotEdge { id: "edge-a-b".into(), from_slot_id: "room-a".into(), to_slot_id: "room-b".into(), relation: "beside".into() }),
        ),
        ("✂️disconnect-slots", "✂️severs", fixture_base(), disconnect_slots("edge-corridor-b".into())),
        ("📌️pin-slot", "📌️pins-room-a-to-the-room-tile", fixture_base(), pin_slot("room-a".into(), "room".into())),
        ("📍️unpin-slot", "📍️releases-the-pin-on-room-a", fixture_base_pinned(), unpin_slot("room-a".into())),
        ("🀄️create-tile", "🀄️adds", fixture_base(), create_tile(crate::schema::snapshot::canonical_tile_index(&fixture_base(), "stair"), stair)),
        ("🗑️delete-tile", "🗑️drops", fixture_base(), delete_tile("corridor".into())),
        ("⚖️change-tile-weight", "⚖️raises", fixture_base(), change_tile_weight("room".into(), 5.0)),
        ("🖼️change-tile-media", "🖼️swaps", fixture_base(), change_tile_media("corridor".into(), crate::unit_wedge_media(Some(Color { r: 200, g: 200, b: 210, a: 255 })))),
        (
            "🚦️create-rule",
            "🚦️forbids",
            fixture_base(),
            create_rule(
                crate::schema::snapshot::canonical_rule_index(&fixture_base(), "rule-room-room"),
                GraphRule { id: "rule-room-room".into(), tile_a_id: "room".into(), tile_b_id: "room".into(), relation: Some("beside".into()), allowed: false },
            ),
        ),
        ("🚫️delete-rule", "🚫️drops", fixture_base(), delete_rule("rule-room-corridor".into())),
        ("✋️drag-slots", "✋️lifts-both-rooms-two-storeys", fixture_base(), drag_slots(vec!["room-a".into(), "room-b".into()], 0.5, 1.0, 2.0)),
        ("✋️drag-slots", "⚠️skips", fixture_base(), drag_slots(vec!["corridor".into(), "ghost".into()], -1.0, 0.0, 0.5)),
        (
            "🎯️set-slot-positions",
            "🎯️sets",
            fixture_base(),
            set_slot_positions(vec![Wfc3dSlotPosition { id: "room-b".into(), x: 4.0, y: 0.0, z: 1.0 }, Wfc3dSlotPosition { id: "corridor".into(), x: 1.0, y: 2.0, z: 0.0 }]),
        ),
    ]
}
//#endregion 🧸️Fixtures

//#region 🔖️Laws
/// 🏷️ Every variant's verb is in the framework's closed vocabulary, and `KINDS` is exactly the
/// enum's own roster — the framework never parses Rust, so this is what keeps the list honest.
#[test]
fn dispatch_registers_semantic_descriptors_with_approved_verbs() {
    for kind in Wfc3dMutation::kinds() {
        assert!(protocol::is_approved_verb(kind.verb), "verb '{}' must be in APPROVED_VERBS", kind.verb);
    }
    assert_eq!(Wfc3dMutation::kinds().len(), 17);
}

#[test]
fn the_kinds_constant_matches_the_dispatch_roster_in_declaration_order() {
    let declared: Vec<&str> = Wfc3dMutation::kinds().iter().map(|kind| kind.kind).collect();
    assert_eq!(declared, KINDS.to_vec());
}

/// ↩️ THE point-invertibility law: apply, then apply every step of `inverse()`, and the document is
/// byte-identical to what it was — for every one of the seventeen kinds, including the two that
/// cascade.
#[test]
fn every_mutation_inverse_restores_its_own_base_exactly() {
    for (mutation_dir, case, base, mutation) in fixture_cases() {
        let (forward, _) = apply_mutation(&base, &mutation).unwrap_or_else(|error| panic!("{mutation_dir}/{case}: forward apply failed: {error}"));
        let mut restored = forward.clone();
        for back in mutation.inverse(&base).expect("valid retained mutation inverse fixture") {
            restored = apply_mutation(&restored, &back).unwrap_or_else(|error| panic!("{mutation_dir}/{case}: inverse apply failed: {error}")).0;
        }
        assert_eq!(restored, base, "{mutation_dir}/{case}: inverse() must restore the pre-mutation document");
    }
}

/// 🔤️ A `create-*` inserts at its collection's CANONICAL sorted position, so an undo puts the row
/// back exactly where it was and two authors who create the same id agree on the index.
#[test]
fn every_collection_stays_sorted_after_a_canonical_insert() {
    for (mutation_dir, case, base, mutation) in fixture_cases() {
        let (forward, _) = apply_mutation(&base, &mutation).expect("forward apply");
        let ids: Vec<String> = forward.slots.iter().map(|slot| slot.id.clone()).collect();
        let mut sorted = ids.clone();
        sorted.sort();
        assert_eq!(ids, sorted, "{mutation_dir}/{case}: slots left their canonical order");
        let ids: Vec<String> = forward.tiles.iter().map(|tile| tile.id.clone()).collect();
        let mut sorted = ids.clone();
        sorted.sort();
        assert_eq!(ids, sorted, "{mutation_dir}/{case}: tiles left their canonical order");
    }
}

/// ⚡️ Every kind round-trips through the single-line op text and the binary state-patch encodings.
#[test]
fn every_mutation_round_trips_through_text_and_binary_op_encodings() {
    for (mutation_dir, case, _base, mutation) in fixture_cases() {
        let line = <Wfc3dMutation as protocol::OpText>::print_op(&mutation);
        assert!(!line.trim().is_empty(), "{mutation_dir}/{case}: op text must not be empty");
        assert_eq!(<Wfc3dMutation as protocol::OpText>::parse_op(&line).expect("op text parses"), mutation, "{mutation_dir}/{case}: op text round trip");
        let bytes = <Wfc3dMutation as protocol::OpBinary>::encode_op(&mutation).expect("op binary encodes");
        assert_eq!(<Wfc3dMutation as protocol::OpBinary>::decode_op(&bytes).expect("op binary decodes"), mutation, "{mutation_dir}/{case}: op binary round trip");
    }
}

/// 🎯️ A refused mutation raises its own diagnostic and produces an EMPTY delta, so applying it is a
/// no-op on the document rather than an error — the guard order (`target-missing` → invariant →
/// no-op → apply) has to hold on a real refusal, not only on paper.
#[test]
fn a_missing_target_is_refused_and_changes_nothing() {
    let base = fixture_base();
    let outcome = <Wfc3dMutation as Mutation<Wfc3dSnapshot>>::diff(&delete_slot("ghost".into()), &base);
    assert!(outcome.messages().iter().any(|message| message.code.0 == "mutation.target-missing"), "a missing slot must be named");
    assert_eq!(outcome.diff(), &Wfc3dDiff::default(), "a refusal authors no delta");
    let mut projection = base.clone();
    apply_wfc3d_mutation(&mut projection, &delete_slot("ghost".into())).expect("an empty delta still applies");
    assert_eq!(projection, base, "a refused mutation must leave the document untouched");
}

/// 🚫️ A duplicate id is FATAL, never a silent overwrite.
#[test]
fn a_duplicate_id_is_fatal() {
    let base = fixture_base();
    let duplicate = create_tile(0, Tile { id: "room".into(), label: None, weight: 1.0, media: TileMedia3d::default() });
    let outcome = <Wfc3dMutation as Mutation<Wfc3dSnapshot>>::diff(&duplicate, &base);
    assert!(outcome.messages().iter().any(|message| message.code.0 == "mutation.duplicate-id"));
}
//#endregion 🔖️Laws

//#region 🏭️FixtureGenerator
/// 🏭️ Writes every case's committed quintet from the SAME table the laws above read. Ignored by
/// default: it authors files, it does not assert. Run with
/// `cargo test -p semio-s-artifact-wfc-3d --lib -- --ignored regenerate_committed_fixture_quintets`.
#[test]
#[ignore]
fn regenerate_committed_fixture_quintets() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations");
    for (mutation_dir, case, base, mutation) in fixture_cases() {
        let directory = root.join(mutation_dir).join(case);
        std::fs::create_dir_all(directory.join("📸️snapshot/⬅️before")).expect("before dir");
        std::fs::create_dir_all(directory.join("📸️snapshot/➡️after")).expect("after dir");
        std::fs::create_dir_all(directory.join("🦠️mutation")).expect("mutation dir");
        std::fs::create_dir_all(directory.join("🔺️diff")).expect("diff dir");
        std::fs::create_dir_all(directory.join("🎯️outcome")).expect("outcome dir");

        let raised = <Wfc3dMutation as Mutation<Wfc3dSnapshot>>::diff(&mutation, &base);
        let (after, _) = apply_mutation(&base, &mutation).expect("forward apply");
        let messages: Vec<String> = raised
            .messages()
            .iter()
            .map(|message| {
                let level = semio_framework_pack_json::to_json_string(&message.level);
                format!("{{\"level\":{level},\"code\":{}}}", semio_framework_pack_json::to_json_string(&message.code.0))
            })
            .collect();
        let outcome = if messages.is_empty() { "{\"status\":\"applied\"}".to_string() } else { format!("{{\"status\":\"applied\",\"messages\":[{}]}}", messages.join(",")) };

        std::fs::write(directory.join("📸️snapshot/⬅️before/🔣️.json"), semio_framework_pack_json::to_json_string(&base)).expect("write before");
        std::fs::write(directory.join("📸️snapshot/➡️after/🔣️.json"), semio_framework_pack_json::to_json_string(&after)).expect("write after");
        std::fs::write(directory.join("🦠️mutation/🔣️.json"), semio_framework_pack_json::to_json_string(&mutation)).expect("write mutation");
        std::fs::write(directory.join("🔺️diff/🔣️.json"), semio_framework_pack_json::to_json_string(raised.diff())).expect("write diff");
        std::fs::write(directory.join("🎯️outcome/🔣️.json"), outcome).expect("write outcome");
    }
}
//#endregion 🏭️FixtureGenerator
