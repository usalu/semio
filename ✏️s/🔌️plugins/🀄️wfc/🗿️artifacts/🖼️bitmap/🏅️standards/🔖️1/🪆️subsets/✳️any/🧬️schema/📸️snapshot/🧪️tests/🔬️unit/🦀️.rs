//! 🧪️ Snapshot facet — the document's own invariants: pixel decoding, pin ordering, palette use,
//! and the region primitives every paint mutation is built on.

use super::*;

fn scene() -> BitmapSnapshot {
    BitmapSnapshot {
        input: BitmapInput { width: 3, height: 2, palette: vec![BitmapColor::opaque(0, 0, 0), BitmapColor::opaque(255, 255, 255)], pixels: encode_base64(&[0, 1, 0, 1, 0, 1]) },
        output: BitmapOutputSpec { width: 4, height: 4, periodic: false },
        pinned: vec![BitmapPinnedPixel { x: 1, y: 0, color: 1 }, BitmapPinnedPixel { x: 0, y: 2, color: 0 }],
        ..BitmapSnapshot::default()
    }
}

#[test]
fn a_default_snapshot_names_its_own_schema_and_decodes() {
    let snapshot = BitmapSnapshot::default();
    assert_eq!(snapshot.schema, WFC_BITMAP_DOCUMENT_SCHEMA);
    assert_eq!(snapshot.input.indices().as_deref(), Some([0u8].as_slice()));
}

#[test]
fn indices_refuse_a_buffer_of_the_wrong_length() {
    let mut snapshot = scene();
    snapshot.input.pixels = encode_base64(&[0, 1]);
    assert_eq!(snapshot.input.indices(), None, "a buffer that is not width * height bytes is not this bitmap's");
}

#[test]
fn pins_are_addressed_by_coordinate_and_ordered_row_major() {
    let snapshot = scene();
    assert_eq!(pin_index(&snapshot, 1, 0), Some(0));
    assert_eq!(pin_index(&snapshot, 0, 2), Some(1));
    assert_eq!(pin_index(&snapshot, 3, 3), None);
    assert_eq!(ordered_pin_index(&snapshot.pinned, 0, 0), 0, "(0,0) precedes (1,0)");
    assert_eq!(ordered_pin_index(&snapshot.pinned, 2, 0), 1, "(2,0) follows (1,0) and precedes (0,2)");
    assert_eq!(ordered_pin_index(&snapshot.pinned, 9, 9), 2, "a cell past every pin lands last");
    assert_eq!(pin_key(3, 4), "3:4");
}

#[test]
fn used_palette_indices_covers_pixels_and_pins() {
    let mut snapshot = scene();
    snapshot.input.palette.push(BitmapColor::opaque(9, 9, 9));
    assert_eq!(used_palette_indices(&snapshot), vec![0, 1], "the spare third colour is used by nothing");
    snapshot.pinned.push(BitmapPinnedPixel { x: 3, y: 3, color: 2 });
    assert_eq!(used_palette_indices(&snapshot), vec![0, 1, 2], "a pin counts as use");
}

#[test]
fn region_reads_and_writes_are_exact_inverses() {
    let mut buffer = vec![0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
    let prior = read_region(&buffer, 4, 3, 1, 1, 2, 2).expect("region reads");
    assert_eq!(prior, vec![5, 6, 9, 10]);
    assert!(write_region(&mut buffer, 4, 3, 1, 1, 2, 2, &[99, 98, 97, 96]));
    assert_eq!(read_region(&buffer, 4, 3, 1, 1, 2, 2).as_deref(), Some([99u8, 98, 97, 96].as_slice()));
    assert!(write_region(&mut buffer, 4, 3, 1, 1, 2, 2, &prior));
    assert_eq!(buffer, vec![0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11], "writing the prior bytes back restores the buffer exactly");
}

#[test]
fn a_region_outside_the_bitmap_is_refused_rather_than_clipped() {
    let mut buffer = vec![0u8; 12];
    assert!(!write_region(&mut buffer, 4, 3, 3, 0, 2, 1, &[1, 1]), "a clipped write could not be inverted");
    assert_eq!(read_region(&buffer, 4, 3, 0, 2, 4, 2), None);
    assert!(!write_region(&mut buffer, 4, 3, 0, 0, 2, 2, &[1, 1]), "a payload of the wrong length is refused");
}

#[test]
fn resizing_keeps_the_overlap_and_pads_with_index_zero() {
    let buffer = vec![1u8, 2, 3, 4, 5, 6];
    let grown = resized_buffer(&buffer, 3, 2, 4, 3);
    assert_eq!(grown, vec![1, 2, 3, 0, 4, 5, 6, 0, 0, 0, 0, 0]);
    let shrunk = resized_buffer(&grown, 4, 3, 2, 2);
    assert_eq!(shrunk, vec![1, 2, 4, 5]);
}

#[test]
fn a_colour_projects_to_the_hosts_unit_rgba_quadruple() {
    assert_eq!(BitmapColor::opaque(255, 0, 51).to_unit_rgba(), [1.0, 0.0, 0.2, 1.0]);
}
