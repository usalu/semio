use super::*;
use crate::default_presentation_snapshot;
use crate::standards::v1::subsets::any::schema::{populate_tile_drafts_from_grid, FigureTileGridSeedSpec};
use store::os_store::test_support;

#[test]
fn dsl_round_trip_default_presentation_snapshot() {
    test_support::assert_dsl_round_trip(&default_presentation_snapshot());
    test_support::assert_dsl_pack_equivalence(&default_presentation_snapshot());
}

#[test]
fn dsl_round_trip_presentation_deck_with_tiles() {
    let deck = default_presentation_snapshot();
    let (source, _) = crate::presentation_working_scene(&deck);
    let tiles = populate_tile_drafts_from_grid(FigureTileGridSeedSpec { source: &source, rows: 2, columns: 2, gap: 0.0, key_prefix: "tile" });
    let deck = crate::presentation_snapshot_with_tiles(&source, &tiles);
    test_support::assert_dsl_round_trip(&deck);
    test_support::assert_dsl_pack_equivalence(&deck);
}

#[test]
fn presentation_dsl_round_trips_bundled_default_example() {
    let deck = parse_dsl(PRESENTATION_EXAMPLE_TEXT).expect("🎞️default.presentation must parse");
    test_support::assert_dsl_round_trip(&deck);
    test_support::assert_dsl_pack_equivalence(&deck);
}

/// 📚️ The committed `demo` example asset IS this crate's own printed `demo_snapshot()` — the app's
/// `setActiveExample("demo")` loads that snapshot in the guest while the manifest ships this text, so a
/// drift between the two would serve the pane a different deck than the one the plugin builds.
#[test]
fn the_committed_demo_asset_is_the_printers_own_demo_snapshot() {
    assert_eq!(PRESENTATION_EXAMPLE_TEXT, print_dsl(&crate::demo_presentation_snapshot()), "🖼️assets/🎬️demo: the committed asset must be this crate's own printed output");
}

/// ✍️ Regenerates the committed `demo` asset from THIS crate's own printer — the only sanctioned way
/// to move it (the composed child handle is a content hash, so no hand edit can be correct). Runs
/// last by name; `the_committed_demo_asset_is_the_printers_own_demo_snapshot` above is the law it
/// serves, and that law still reads the `include_str!` constant baked into THIS binary, so the
/// regenerated text is proven by the NEXT run.
#[test]
fn zzz_write_demo_example_asset() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/🎬️demo/🗣️.dsl.semio");
    let printed = print_dsl(&crate::demo_presentation_snapshot());
    if printed != PRESENTATION_EXAMPLE_TEXT {
        std::fs::write(&path, &printed).expect("🖼️assets/🎬️demo is writable");
    }
}
