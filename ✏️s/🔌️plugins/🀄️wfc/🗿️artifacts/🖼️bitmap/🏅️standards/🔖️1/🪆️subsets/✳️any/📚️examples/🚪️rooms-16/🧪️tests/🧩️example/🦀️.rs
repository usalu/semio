//! 🧪️ `rooms-16` — the committed asset IS the print of the authored spec, and the closed form
//! really produces the regularity the model is meant to learn.

use super::{indices, label, snapshot, DOOR, EDGE, FLOOR, PRIMARY_TEXT, SEED, WALL};
use crate::schema::snapshot::text::{parse_dsl, print_dsl};

#[test]
fn committed_asset_is_the_print_of_the_authored_spec() {
    assert_eq!(print_dsl(&snapshot()), PRIMARY_TEXT, "🗣️.dsl.semio must be regenerated from the Rust builder, never hand-edited");
}

#[test]
fn committed_asset_parses_back_to_the_authored_spec() {
    assert_eq!(parse_dsl(PRIMARY_TEXT).expect("committed asset parses"), snapshot());
}

#[test]
fn the_pack_codec_round_trips_the_same_document() {
    let bytes = crate::schema::snapshot::binary::encode(&snapshot());
    assert_eq!(crate::schema::snapshot::binary::decode(&bytes).expect("committed document decodes"), snapshot());
}

#[test]
fn the_sample_is_a_real_three_colour_plan() {
    let buffer = indices();
    assert_eq!(buffer.len(), (EDGE * EDGE) as usize);
    assert!(buffer.contains(&WALL) && buffer.contains(&FLOOR) && buffer.contains(&DOOR), "all three colours are used");
    assert_eq!(buffer[0], WALL, "the origin sits on a wall line");
    assert_eq!(buffer[(EDGE + 1) as usize], FLOOR, "the cell inside the first room is floor");
    assert_eq!(buffer[(2 * EDGE) as usize], DOOR, "the midpoint of the first vertical wall segment is a door");
}

#[test]
fn the_seed_and_the_label_are_persisted_and_authored() {
    assert_eq!(snapshot().seed, SEED);
    assert_eq!(label(), semio_framework_plugin::LocalizedLabel::native("Rooms 16", "Räume 16"));
}
