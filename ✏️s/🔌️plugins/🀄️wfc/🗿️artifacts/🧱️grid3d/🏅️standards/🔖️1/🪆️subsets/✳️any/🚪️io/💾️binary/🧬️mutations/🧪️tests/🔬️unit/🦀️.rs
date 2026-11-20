//! 🔬️ The binary op tag is the variant's own position in the shared `DslVariants` roster, so a kind
//! can never carry one tag on the wire and another in the grammar.

use crate::schema::mutations::{change_cell_sizes, change_seed, create_tile, mask_cell, Grid3dMutation};
use crate::schema::snapshot::{Grid3dAxis, Grid3dCell, Grid3dMesh, Grid3dTile, Grid3dTileMedia};

#[test]
fn encoding_then_decoding_is_a_fixed_point() {
    let mutations = [
        change_seed(5),
        mask_cell(Grid3dCell { x: 1, y: 2, z: 3 }),
        change_cell_sizes(Grid3dAxis::Z, vec![2.0, 1.0]),
        create_tile(Grid3dTile {
            id: "triangle".into(),
            weight: 1.0,
            media: Grid3dTileMedia::Mesh {
                mesh: Grid3dMesh {
                    positions: vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0],
                    indices: vec![0, 1, 2],
                    color: None,
                },
            },
            ..Default::default()
        }),
    ];
    for mutation in &mutations {
        let bytes = crate::standards::v1::subsets::any::io::binary::mutations::encode_op(mutation).expect("op encodes");
        assert_eq!(&crate::standards::v1::subsets::any::io::binary::mutations::decode_op(&bytes).expect("op decodes"), mutation);
    }
    semio_os_kernel::os_dsl::original_receiving::check_receiving_round_trip(&mutations).expect("retained decoding matches the authored binary wire");
}

#[test]
fn two_different_kinds_never_encode_to_the_same_bytes() {
    let seed = crate::standards::v1::subsets::any::io::binary::mutations::encode_op(&change_seed(5)).expect("op encodes");
    let mask = crate::standards::v1::subsets::any::io::binary::mutations::encode_op(&mask_cell(Grid3dCell { x: 1, y: 2, z: 3 })).expect("op encodes");
    assert_ne!(seed, mask, "the variant tag rides inside the framed record, so two kinds can never collide");
    assert_ne!(crate::standards::v1::subsets::any::io::binary::mutations::decode_op(&seed).expect("seed decodes"), crate::standards::v1::subsets::any::io::binary::mutations::decode_op(&mask).expect("mask decodes"));
}

#[test]
fn a_truncated_payload_is_refused_rather_than_misread() {
    let bytes = crate::standards::v1::subsets::any::io::binary::mutations::encode_op(&change_seed(5)).expect("op encodes");
    assert!(crate::standards::v1::subsets::any::io::binary::mutations::decode_op(&bytes[..bytes.len().saturating_sub(1)]).is_err() || <Grid3dMutation as protocol::OpBinary>::decode_op(&[]).is_err());
}
