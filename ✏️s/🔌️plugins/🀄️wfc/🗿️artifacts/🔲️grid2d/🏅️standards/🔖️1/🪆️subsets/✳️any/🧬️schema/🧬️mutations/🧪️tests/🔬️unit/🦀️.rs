//! 🧪️ `Grid2dMutation` laws — the kind roster, the canonical-position helpers, the text/binary op
//! round trip, and the cascade every collection mutation owes its own inverse.

use super::*;
use crate::schema::snapshot::{WfcAdjacencyRule2d, WfcCell2d, WfcDirection2d, WfcPinnedCell2d, WfcTile2d};

fn tile(id: &str) -> WfcTile2d {
    WfcTile2d { id: id.into(), weight: 1.0, ..Default::default() }
}

fn scene() -> Grid2dSnapshot {
    Grid2dSnapshot {
        width: 3,
        height: 2,
        tiles: vec![tile("a"), tile("c")],
        rules: vec![WfcAdjacencyRule2d { id: "r-a-c".into(), tile_a_id: "a".into(), tile_b_id: "c".into(), direction: WfcDirection2d::Right, allowed: true }],
        pinned: vec![WfcPinnedCell2d { x: 0, y: 0, tile_id: "c".into() }],
        masked: vec![WfcCell2d { x: 2, y: 1 }],
        ..Default::default()
    }
}

#[test]
fn the_kind_roster_matches_the_enum_in_declaration_order() {
    let scene = scene();
    let every: Vec<Grid2dMutation> = vec![
        change_seed(1),
        resize_grid(2, 2),
        change_cell_size(2.0, 2.0),
        change_periodicity(true, false),
        create_tile(tile("b")),
        delete_tile("a".into()),
        change_tile_weight("a".into(), 3.0),
        change_tile_media("a".into(), Default::default()),
        create_rule(WfcAdjacencyRule2d { id: "r-c-a".into(), tile_a_id: "c".into(), tile_b_id: "a".into(), direction: WfcDirection2d::Right, allowed: true }),
        delete_rule("r-a-c".into()),
        pin_cell(1, 0, "a".into()),
        unpin_cell(0, 0),
        mask_cell(1, 1),
        unmask_cell(2, 1),
    ];
    assert_eq!(every.len(), KINDS.len());
    for (mutation, kind) in every.iter().zip(KINDS) {
        assert_eq!(<Grid2dMutation as Mutation<Grid2dSnapshot>>::descriptor(mutation).semantic_kind, *kind);
        let _ = <Grid2dMutation as Mutation<Grid2dSnapshot>>::diff(mutation, &scene);
    }
}

#[test]
fn an_id_keyed_insert_lands_at_its_sorted_position_not_at_the_end() {
    let tiles = [tile("a"), tile("c")];
    assert_eq!(ordered_tile_index(&tiles, "b"), 1);
    assert_eq!(ordered_tile_index(&tiles, "z"), 2);
    assert_eq!(ordered_tile_index(&tiles, "A"), 0);
}

#[test]
fn a_cell_insert_lands_at_its_row_major_position() {
    let cells = [WfcCell2d { x: 2, y: 0 }, WfcCell2d { x: 1, y: 1 }];
    assert_eq!(ordered_cell_index(&cells, 0, 0, |cell| (cell.y, cell.x)), 0);
    assert_eq!(ordered_cell_index(&cells, 0, 1, |cell| (cell.y, cell.x)), 1);
    assert_eq!(ordered_cell_index(&cells, 2, 1, |cell| (cell.y, cell.x)), 2);
}

#[test]
fn every_mutation_round_trips_through_its_text_and_binary_op_form() {
    for mutation in [change_seed(42), resize_grid(4, 4), pin_cell(1, 1, "a".into()), delete_rule("r-a-c".into()), unmask_cell(2, 1)] {
        let line = protocol::OpText::print_op(&mutation);
        assert_eq!(<Grid2dMutation as protocol::OpText>::parse_op(&line).expect("op text parses"), mutation, "text round trip for {line}");
        let bytes = protocol::OpBinary::encode_op(&mutation).expect("op binary encodes");
        assert_eq!(<Grid2dMutation as protocol::OpBinary>::decode_op(&bytes).expect("op binary decodes"), mutation);
    }
}

#[test]
fn deleting_a_tile_cascades_every_rule_and_pin_that_named_it() {
    let base = scene();
    let mut applied = base.clone();
    apply_grid2d_mutation(&mut applied, &delete_tile("c".into())).expect("delete applies");
    assert!(applied.tiles.iter().all(|tile| tile.id != "c"));
    assert!(applied.rules.is_empty(), "the rule naming the tile must be gone");
    assert!(applied.pinned.is_empty(), "the pin naming the tile must be gone");
    for step in inverse_grid2d_mutation(&base, &delete_tile("c".into())) {
        apply_grid2d_mutation(&mut applied, &step).expect("inverse step applies");
    }
    assert_eq!(applied, base, "the cascade's inverse must restore value AND position");
}

#[test]
fn masking_a_pinned_cell_cascades_its_pin_and_the_inverse_restores_it() {
    let base = scene();
    let mutation = mask_cell(0, 0);
    let mut applied = base.clone();
    apply_grid2d_mutation(&mut applied, &mutation).expect("mask applies");
    assert!(applied.pinned.is_empty());
    for step in inverse_grid2d_mutation(&base, &mutation) {
        apply_grid2d_mutation(&mut applied, &step).expect("inverse step applies");
    }
    assert_eq!(applied, base);
}

#[test]
fn a_pin_on_a_masked_cell_is_refused_outright() {
    let outcome = <Grid2dMutation as Mutation<Grid2dSnapshot>>::diff(&pin_cell(2, 1, "a".into()), &scene());
    assert!(outcome.messages().iter().any(|message| message.code.0 == "mutation.invariant"));
}

#[test]
fn a_rule_naming_an_unknown_tile_is_refused_outright() {
    let rule = WfcAdjacencyRule2d { id: "r-ghost".into(), tile_a_id: "a".into(), tile_b_id: "ghost".into(), direction: WfcDirection2d::Top, allowed: true };
    let outcome = <Grid2dMutation as Mutation<Grid2dSnapshot>>::diff(&create_rule(rule), &scene());
    assert!(outcome.messages().iter().any(|message| message.code.0 == "mutation.invariant"));
}

#[test]
fn a_second_rule_on_the_same_pair_and_direction_is_refused() {
    let duplicate = WfcAdjacencyRule2d { id: "r-other".into(), tile_a_id: "a".into(), tile_b_id: "c".into(), direction: WfcDirection2d::Right, allowed: false };
    let outcome = <Grid2dMutation as Mutation<Grid2dSnapshot>>::diff(&create_rule(duplicate), &scene());
    assert!(outcome.messages().iter().any(|message| message.code.0 == "mutation.invariant"));
}

#[test]
fn a_non_positive_tile_weight_is_refused() {
    for weight in [0.0, -1.0, f64::NAN] {
        let outcome = <Grid2dMutation as Mutation<Grid2dSnapshot>>::diff(&change_tile_weight("a".into(), weight), &scene());
        assert!(outcome.messages().iter().any(|message| message.code.0 == "mutation.invariant"), "weight {weight} must be refused");
    }
}
