//! 🧪️ `flowers-24` — the committed asset IS the print of the authored spec, and the example really
//! exercises the ground constraint and the pin lane.

use super::{indices, label, snapshot, EDGE, GROUND, PETAL, PRIMARY_TEXT, SEED, SKY, STEM};
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
fn the_sample_uses_all_four_colours_and_bands_the_ground() {
    let buffer = indices();
    assert_eq!(buffer.len(), (EDGE * EDGE) as usize);
    for colour in [SKY, GROUND, STEM, PETAL] {
        assert!(buffer.contains(&colour), "colour {colour} is used by the sample");
    }
    for x in 0..EDGE {
        assert_eq!(buffer[((EDGE - 1) * EDGE + x) as usize], GROUND, "the bottom row is all ground");
    }
    assert_eq!(buffer[0], SKY, "the top-left corner is sky");
}

#[test]
fn the_model_declares_the_ground_colour_and_a_narrow_symmetry_group() {
    let snapshot = snapshot();
    assert_eq!(snapshot.model.ground, Some(u32::from(GROUND)));
    assert_eq!(snapshot.model.symmetry, 2, "a flower upside down is not a flower");
    assert!(!snapshot.model.periodic_input);
    assert_eq!(snapshot.pinned.len(), 1, "the example exercises the pin lane too");
    assert_eq!(snapshot.seed, SEED);
    assert_eq!(label(), semio_framework_plugin::LocalizedLabel::native("Flowers 24", "Blumen 24"));
}
