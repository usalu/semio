//! 🧪️ The `terrain` example is a real, solvable bitmap tile set whose transition law holds.

use super::*;
use crate::schema::snapshot::{decode_palette_indices, Grid2dSnapshot, WfcDirection2d};

#[test]
fn the_example_states_three_bitmap_tiles() {
    let document = document();
    assert_eq!(document.tiles.len(), 3);
    for tile in &document.tiles {
        match &tile.media {
            WfcTileMedia2d::Bitmap { width, height, palette, pixels } => {
                assert_eq!((*width, *height), (4, 4));
                assert_eq!(palette.len(), 2);
                assert_eq!(decode_palette_indices(pixels).len(), 16, "one palette index per pixel, row-major");
            }
            other => panic!("the terrain set is a bitmap set, got {other:?}"),
        }
    }
}

#[test]
fn grass_never_touches_water_in_either_canonical_direction() {
    let document = document();
    for rule in &document.rules {
        let pair = (rule.tile_a_id.as_str(), rule.tile_b_id.as_str());
        assert!(!matches!(pair, ("grass", "water") | ("water", "grass")), "{} whitelists a forbidden neighbourhood", rule.id);
        assert!(matches!(rule.direction, WfcDirection2d::Right | WfcDirection2d::Bottom), "only the canonical directions are authored");
    }
    assert!(document.rules.windows(2).all(|pair| pair[0].id < pair[1].id));
}

#[test]
fn the_example_source_prints_the_document_it_states() {
    let source = source();
    assert_eq!(source.id(), ID);
    let parsed = <Grid2dSnapshot as store::ArtifactDsl>::parse_dsl(&source.document()).expect("the printed example parses back");
    assert_eq!(parsed, document());
}

#[test]
fn the_example_solves_and_respects_its_pins() {
    let document = document();
    let commit = crate::schema::inferences::solve_with_job(&document).expect("the bundled example solves");
    assert!(!commit.contradiction);
    assert_eq!(commit.assignments.len(), 64);
    for pin in &document.pinned {
        let assigned = commit.assignments.iter().find(|(x, y, _)| *x == pin.x && *y == pin.y).expect("pinned cell assigned");
        assert_eq!(assigned.2, pin.tile_id);
    }
}
