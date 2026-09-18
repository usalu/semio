//! 🧪️ The `pipes` example is a real, solvable, canonically ordered document.

use super::*;

#[test]
fn the_example_states_five_tiles_and_a_complete_connector_rule_set() {
    let document = document();
    assert_eq!(document.tiles.len(), 5);
    assert!(document.tiles.windows(2).all(|pair| pair[0].id < pair[1].id), "tiles are kept sorted by id");
    assert!(document.rules.windows(2).all(|pair| pair[0].id < pair[1].id), "rules are kept sorted by id");
    assert!(document.rules.iter().all(|rule| rule.allowed), "only whitelisting rows are authored");
    assert!(document.rules.iter().any(|rule| rule.direction == WfcDirection2d::Right));
    assert!(document.rules.iter().any(|rule| rule.direction == WfcDirection2d::Bottom));
}

#[test]
fn every_rule_names_a_tile_the_catalogue_holds() {
    let document = document();
    for rule in &document.rules {
        assert!(document.tiles.iter().any(|tile| tile.id == rule.tile_a_id), "{}", rule.id);
        assert!(document.tiles.iter().any(|tile| tile.id == rule.tile_b_id), "{}", rule.id);
    }
    for cell in &document.pinned {
        assert!(document.tiles.iter().any(|tile| tile.id == cell.tile_id));
        assert!(cell.x < document.width && cell.y < document.height);
    }
}

#[test]
fn the_vector_media_draws_one_stroke_per_connector() {
    let document = document();
    let empty = document.tiles.iter().find(|tile| tile.id == "empty").expect("empty tile");
    match &empty.media {
        WfcTileMedia2d::Vector { paths } => assert!(paths.is_empty(), "the empty tile connects nowhere"),
        other => panic!("the pipes set is a vector set, got {other:?}"),
    }
    let elbow = document.tiles.iter().find(|tile| tile.id == "elbow-ne").expect("elbow tile");
    match &elbow.media {
        WfcTileMedia2d::Vector { paths } => assert_eq!(paths.len(), 2, "an elbow connects through exactly two sides"),
        other => panic!("expected vector media, got {other:?}"),
    }
}

#[test]
fn the_example_source_prints_the_document_it_states() {
    let source = source();
    assert_eq!(source.id(), ID);
    let parsed = <Grid2dSnapshot as store::ArtifactDsl>::parse_dsl(source.document()).expect("the printed example parses back");
    assert_eq!(parsed, document(), "the example text must be a PRINT of the Rust authority, never a second one");
}

#[test]
fn the_example_solves_deterministically() {
    let commit = crate::schema::inferences::solve_with_job(&document()).expect("the bundled example solves");
    assert!(!commit.contradiction, "a bundled example must be satisfiable");
    assert_eq!(commit.assignments.len(), 35, "every unmasked cell of the 6×6 grid is assigned");
    let again = crate::schema::inferences::solve_with_job(&document()).expect("the solve repeats");
    assert_eq!(commit.assignments, again.assignments);
}
