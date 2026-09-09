use super::*;
use crate::{FigureTileDraft, FigureTileFrame};

fn tile(id: &str) -> FigureTileDraft {
    FigureTileDraft { id: id.into(), name: id.into(), crop: FigureTileFrame { x: 0.0, y: 0.0, width: 1.0, height: 1.0 } }
}

#[test]
fn empty_tiles_is_the_vacuous_topology() {
    let snapshot = PresentationSnapshot::default();
    let topology = compute_presentation_topology(&snapshot);
    assert!(topology.topo_order.is_empty());
    assert!(topology.depth.is_empty());
    assert!(topology.cycle_free);
    assert_eq!(topology.node_count, 0);
}

#[test]
fn depth_matches_persisted_index() {
    let (source, _) = crate::presentation_working_scene(&PresentationSnapshot::default());
    let snapshot = crate::presentation_snapshot_with_tiles(&source, &[tile("a"), tile("b")]);
    let topology = compute_presentation_topology(&snapshot);
    assert_eq!(topology.depth.get("a"), Some(&0));
    assert_eq!(topology.depth.get("b"), Some(&1));
}
