
use super::*;
use crate::{FigureTileDraft, FigureTileFrame};
use protocol::Inference;

//#region 🧸️Fixtures
fn tile(id: &str, name: &str) -> FigureTileDraft {
    FigureTileDraft { id: id.into(), name: name.into(), crop: FigureTileFrame { x: 0.0, y: 0.0, width: 1.0, height: 1.0 } }
}

fn sample_snapshot() -> PresentationSnapshot {
    let (source, _) = crate::presentation_working_scene(&PresentationSnapshot::default());
    crate::presentation_snapshot_with_tiles(&source, &[tile("tile-1", "First"), tile("tile-2", "Second"), tile("tile-3", "Third")])
}
//#endregion 🧸️Fixtures

//#region 🧪️InferenceLaws
#[test]
fn inference_determinism_law() {
    let snapshot = sample_snapshot();
    assert_eq!(PresentationInference::infer(&snapshot), PresentationInference::infer(&snapshot));
}

#[test]
fn inference_default_law() {
    assert_eq!(PresentationInference::infer(&PresentationSnapshot::default()), PresentationInference::default());
}

#[test]
fn topology_orders_tiles_by_persisted_position() {
    let snapshot = sample_snapshot();
    let inferred = PresentationInference::infer(&snapshot);
    assert_eq!(inferred.topology.topo_order, vec!["tile-1".to_string(), "tile-2".to_string(), "tile-3".to_string()]);
    assert_eq!(inferred.topology.depth.get("tile-2"), Some(&1));
    assert!(inferred.topology.cycle_free);
    assert_eq!(inferred.topology.node_count, 3);
}
//#endregion 🧪️InferenceLaws
