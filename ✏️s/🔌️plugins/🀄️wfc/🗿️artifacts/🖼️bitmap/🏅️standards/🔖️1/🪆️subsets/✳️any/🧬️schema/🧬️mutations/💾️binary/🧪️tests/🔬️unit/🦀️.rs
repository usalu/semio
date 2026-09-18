//! 🧪️ Mutation binary facet — the op wire codec round-trips every variant and every kind carries a
//! distinct tag.

use super::*;
use crate::mutations::{add_palette_color, change_model, change_palette_color, change_seed, pin_pixel, remove_palette_color, resize_input, resize_output, set_input_pixels, unpin_pixel};
use crate::schema::snapshot::{encode_base64, BitmapColor};

fn every_variant() -> Vec<BitmapMutation> {
    vec![
        change_seed(1),
        resize_input(2, 2),
        set_input_pixels(0, 0, 1, 1, encode_base64(&[1])),
        add_palette_color(2, BitmapColor::opaque(1, 1, 1)),
        change_palette_color(0, BitmapColor::opaque(2, 2, 2)),
        remove_palette_color(1),
        resize_output(2, 2, true),
        change_model(3, 2, false, Some(1)),
        pin_pixel(0, 0, 0),
        unpin_pixel(0, 0),
    ]
}

#[test]
fn every_variant_round_trips_the_binary_op() {
    for mutation in every_variant() {
        let bytes = encode_op(&mutation).expect("op encodes");
        assert!(!bytes.is_empty());
        assert_eq!(decode_op(&bytes).expect("op decodes"), mutation);
    }
}

#[test]
fn no_two_kinds_share_a_wire_encoding() {
    let encoded: Vec<Vec<u8>> = every_variant().iter().map(|mutation| encode_op(mutation).expect("op encodes")).collect();
    for (left, first) in encoded.iter().enumerate() {
        for (right, second) in encoded.iter().enumerate().skip(left + 1) {
            assert_ne!(first, second, "kinds {left} and {right} share a wire encoding, so one of them decodes as the other");
        }
    }
}

#[test]
fn a_truncated_wire_op_is_refused_rather_than_misread() {
    let bytes = encode_op(&pin_pixel(1, 2, 0)).expect("op encodes");
    assert!(decode_op(&bytes[..bytes.len() / 2]).is_err(), "half an op is not an op");
    assert!(decode_op(&[]).is_err(), "an empty buffer is not an op");
}

#[test]
fn the_normative_protocol_names_this_facets_dialect() {
    assert!(COMPONENT_PROTOCOL_SEMIO.contains("protocol wfcbitmap.mutations"));
    assert!(COMPONENT_PROTOCOL_PATH.ends_with("📡️.protocol.semio"));
}
