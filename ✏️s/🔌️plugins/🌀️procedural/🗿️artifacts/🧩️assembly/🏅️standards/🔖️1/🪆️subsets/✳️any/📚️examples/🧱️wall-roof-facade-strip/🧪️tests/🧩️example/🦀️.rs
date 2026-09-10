//! 🧪️ `wall-roof-facade-strip` — the committed asset IS the print of the authored spec, and nothing else.

use super::{snapshot, PRIMARY_TEXT, SEED};
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
fn the_seed_is_persisted_and_authored() {
    assert_eq!(snapshot().seed, SEED);
}
