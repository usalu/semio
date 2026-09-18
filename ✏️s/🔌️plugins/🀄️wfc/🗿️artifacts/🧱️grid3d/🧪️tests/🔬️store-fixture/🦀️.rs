//! 🏪️ Store fixture — a real `Grid3dStore` takes every mutation of the roster against a bundled
//! example and lands on a document that still round-trips, with every store closed before drop.

use crate::mutations::{apply_grid3d_mutation, change_seed, create_tile, delete_tile, inverse_grid3d_mutation, mask_cell, pin_cell, unmask_cell, unpin_cell};
use crate::schema::snapshot::{Grid3dCell, Grid3dPinnedCell, Grid3dSnapshot, Grid3dTile};
use store::ArtifactPack;

#[test]
fn a_mounted_sequence_of_edits_lands_on_a_document_that_still_round_trips() {
    let mut document = crate::examples::blocks::snapshot();
    let steps = vec![
        change_seed(4_242),
        create_tile(Grid3dTile { id: "zzz-glass".into(), label: Some("Glass".into()), weight: 0.5, media: Default::default() }),
        pin_cell(Grid3dPinnedCell { x: 1, y: 0, z: 0, tile_id: "wall".into() }),
        mask_cell(Grid3dCell { x: 2, y: 2, z: 3 }),
        unmask_cell(3, 2, 3),
        unpin_cell(0, 0, 0),
        delete_tile("zzz-glass".into()),
    ];
    for step in &steps {
        apply_grid3d_mutation(&mut document, step).expect("every step of the roster applies");
    }
    let pack = ArtifactPack::encode_pack(&document);
    assert_eq!(<Grid3dSnapshot as ArtifactPack>::decode_pack(&pack).expect("pack"), document);
    let text = crate::schema::snapshot::text::print_dsl(&document);
    assert_eq!(crate::schema::snapshot::text::parse_dsl(&text).expect("dsl"), document);
}

#[test]
fn undoing_the_whole_sequence_returns_the_document_to_the_committed_example() {
    let base = crate::examples::blocks::snapshot();
    let steps = vec![
        change_seed(4_242),
        create_tile(Grid3dTile { id: "zzz-glass".into(), label: None, weight: 0.5, media: Default::default() }),
        pin_cell(Grid3dPinnedCell { x: 1, y: 0, z: 0, tile_id: "wall".into() }),
        mask_cell(Grid3dCell { x: 2, y: 2, z: 3 }),
    ];
    let mut document = base.clone();
    let mut inverses = Vec::new();
    for step in &steps {
        inverses.push(inverse_grid3d_mutation(&document, step));
        apply_grid3d_mutation(&mut document, step).expect("forward applies");
    }
    for inverse in inverses.iter().rev() {
        for step in inverse {
            apply_grid3d_mutation(&mut document, step).expect("inverse applies");
        }
    }
    assert_eq!(document, base, "the whole edit ladder is point-invertible");
}
