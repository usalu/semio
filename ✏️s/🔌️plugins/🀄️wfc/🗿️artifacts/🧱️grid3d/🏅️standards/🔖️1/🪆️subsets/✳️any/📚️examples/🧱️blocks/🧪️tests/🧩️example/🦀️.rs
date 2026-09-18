//! 🧩️ The bundled example round-trips through its own document codecs and stays internally
//! consistent: every rule and every pin names a declared tile, every cell size array matches its
//! axis' extent, and no cell is both pinned and masked.

use crate::schema::snapshot::Grid3dSnapshot;
use store::ArtifactPack;

#[test]
fn the_example_round_trips_through_dsl_and_pack() {
    let snapshot = super::snapshot();
    let text = crate::schema::snapshot::text::print_dsl(&snapshot);
    assert!(!text.trim().is_empty());
    assert_eq!(crate::schema::snapshot::text::parse_dsl(&text).expect("dsl"), snapshot);
    let pack = ArtifactPack::encode_pack(&snapshot);
    assert_eq!(<Grid3dSnapshot as ArtifactPack>::decode_pack(&pack).expect("pack"), snapshot);
}

#[test]
fn every_rule_and_pin_names_a_declared_tile() {
    let snapshot = super::snapshot();
    let tiles: Vec<&str> = snapshot.tiles.iter().map(|tile| tile.id.as_str()).collect();
    for rule in &snapshot.rules {
        assert!(tiles.contains(&rule.tile_a_id.as_str()), "rule {} names an undeclared tile", rule.id);
        assert!(tiles.contains(&rule.tile_b_id.as_str()), "rule {} names an undeclared tile", rule.id);
    }
    for cell in &snapshot.pinned {
        assert!(tiles.contains(&cell.tile_id.as_str()), "pin {}:{}:{} names an undeclared tile", cell.x, cell.y, cell.z);
    }
}

#[test]
fn the_geometry_is_internally_consistent() {
    let snapshot = super::snapshot();
    assert_eq!(snapshot.cell_sizes_x.len(), snapshot.width as usize);
    assert_eq!(snapshot.cell_sizes_y.len(), snapshot.height as usize);
    assert_eq!(snapshot.cell_sizes_z.len(), snapshot.depth as usize);
    assert!(snapshot.cell_sizes_x.iter().chain(&snapshot.cell_sizes_y).chain(&snapshot.cell_sizes_z).all(|size| size.is_finite() && *size > 0.0));
    for cell in &snapshot.pinned {
        assert!(crate::schema::snapshot::cell_in_grid(&snapshot, cell.x, cell.y, cell.z));
        assert!(crate::schema::snapshot::masked_index(&snapshot, cell.x, cell.y, cell.z).is_none(), "a pinned cell may never also be masked");
    }
    for cell in &snapshot.masked {
        assert!(crate::schema::snapshot::cell_in_grid(&snapshot, cell.x, cell.y, cell.z));
    }
}

#[test]
fn the_collections_are_in_canonical_order() {
    let snapshot = super::snapshot();
    assert!(snapshot.tiles.windows(2).all(|pair| pair[0].id < pair[1].id), "tiles must be sorted by id");
    assert!(snapshot.rules.windows(2).all(|pair| pair[0].id < pair[1].id), "rules must be sorted by id");
}

#[test]
fn the_example_source_carries_its_own_printed_text() {
    let source = super::source();
    assert_eq!(source.id(), super::ID);
    assert_eq!(super::snapshot().seed, super::SEED);
}
