//! 🔬️ Mutation-vocabulary laws — the kebab roster matches the enum, every builder round-trips through
//! its own op text and binary forms, and a collection insert lands at the canonical sorted position.

use super::*;
use crate::schema::snapshot::{Grid3dCell, Grid3dDirection, Grid3dPinnedCell, Grid3dRule, Grid3dTile};
use protocol::{OpBinary, OpText};

fn every_mutation() -> Vec<Grid3dMutation> {
    vec![
        change_seed(99),
        resize_grid(2, 2, 2),
        change_cell_sizes(crate::schema::snapshot::Grid3dAxis::X, vec![1.0, 2.0]),
        change_periodicity(true, false, true),
        create_tile(Grid3dTile { id: "zzz".into(), label: Some("Z".into()), weight: 2.0, media: Default::default() }),
        delete_tile("wall".into()),
        change_tile_weight("wall".into(), 5.0),
        change_tile_media("wall".into(), Default::default()),
        create_rule(Grid3dRule { id: "r-zzz".into(), tile_a_id: "air".into(), tile_b_id: "wall".into(), direction: Grid3dDirection::Top, allowed: true }),
        delete_rule("r-top-air-air".into()),
        pin_cell(Grid3dPinnedCell { x: 1, y: 1, z: 1, tile_id: "wall".into() }),
        unpin_cell(0, 0, 0),
        mask_cell(Grid3dCell { x: 2, y: 2, z: 2 }),
        unmask_cell(3, 2, 3),
    ]
}

#[test]
fn the_kebab_roster_matches_the_enum_in_declaration_order() {
    assert_eq!(KINDS.len(), every_mutation().len());
    for (kind, mutation) in KINDS.iter().zip(every_mutation()) {
        assert_eq!(*kind, protocol::SemanticMutation::<Grid3dSnapshot>::semantics(&mutation).kind, "roster order must match the enum");
    }
}

#[test]
fn every_mutation_round_trips_through_its_op_text_form() {
    for mutation in every_mutation() {
        let line = OpText::print_op(&mutation);
        assert!(!line.trim().is_empty());
        assert_eq!(<Grid3dMutation as OpText>::parse_op(&line).expect("op line parses"), mutation, "round trip failed for {line}");
    }
}

#[test]
fn every_mutation_round_trips_through_its_binary_form() {
    for mutation in every_mutation() {
        let bytes = OpBinary::encode_op(&mutation).expect("op encodes");
        assert_eq!(<Grid3dMutation as OpBinary>::decode_op(&bytes).expect("op decodes"), mutation);
    }
}

#[test]
fn an_ordered_index_is_the_existing_slot_or_the_sorted_insertion_point() {
    let items = ["b".to_string(), "d".to_string()];
    let key = |item: &String| item.clone();
    assert_eq!(ordered_index(&items, "a", key), 0);
    assert_eq!(ordered_index(&items, "b", key), 0);
    assert_eq!(ordered_index(&items, "c", key), 1);
    assert_eq!(ordered_index(&items, "e", key), 2);
}

#[test]
fn every_mutation_and_its_inverse_return_the_document_to_where_it_started() {
    let base = crate::examples::blocks::snapshot();
    for mutation in every_mutation() {
        let mut snapshot = base.clone();
        let inverse = inverse_grid3d_mutation(&base, &mutation);
        if apply_grid3d_mutation(&mut snapshot, &mutation).is_err() {
            continue;
        }
        for step in &inverse {
            apply_grid3d_mutation(&mut snapshot, step).expect("inverse step applies");
        }
        assert_eq!(snapshot, base, "inverse did not restore the document for {:?}", protocol::SemanticMutation::<Grid3dSnapshot>::semantics(&mutation).kind);
    }
}
