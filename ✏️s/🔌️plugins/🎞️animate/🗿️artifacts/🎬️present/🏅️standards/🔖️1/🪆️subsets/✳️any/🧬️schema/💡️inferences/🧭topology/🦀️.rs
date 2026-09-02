//! 🧭 `topology` — one named inference: the tile filmstrip's persisted order recast as a trivial
//! topology. `PresentSnapshot.tiles` is a flat, unordered-by-reference list (no `SlotRef`/edge type
//! exists on `FigureTileDraft`), so the honest derived stat per the workflow/dag-shaped inference
//! category is a linear chain: `topoOrder` is the tile ids in persisted order, `depth` is each
//! tile's own index in that order (how far into the sequence it sits), `cycleFree` is always `true`
//! (a flat `Vec` cannot encode a cycle), `nodeCount` is `tiles.len()`. Whole-snapshot scalar, so a
//! plain function suffices — no `InferredField`/per-entity caching needed (see the family root's
//! doc comment for why).

use crate::artifacts::present::PresentSnapshot;
use std::collections::BTreeMap;

//#region 🔖️Topology
/// 🧭️ Present's tile-filmstrip topology — see module doc for the honest-degenerate-chain shape.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PresentTopology {
    pub topo_order: Vec<String>,
    pub depth: BTreeMap<String, u32>,
    pub cycle_free: bool,
    pub node_count: u32,
}

/// 🧮️ Computes [`PresentTopology`] from a present snapshot's persisted tile order (read through the
/// working-scene accessor off the `presentation` child handle — see
/// `crate::artifacts::present::present_working_scene`).
pub fn compute_present_topology(snapshot: &PresentSnapshot) -> PresentTopology {
    let (_, tiles) = crate::artifacts::present::present_working_scene(snapshot);
    let topo_order: Vec<String> = tiles.iter().map(|tile| tile.id.clone()).collect();
    let depth = topo_order.iter().enumerate().map(|(index, id)| (id.clone(), index as u32)).collect();
    PresentTopology { topo_order, depth, cycle_free: true, node_count: tiles.len() as u32 }
}
//#endregion 🔖️Topology

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use crate::artifacts::present::{FigureTileDraft, FigureTileFrame};

    fn tile(id: &str) -> FigureTileDraft {
        FigureTileDraft { id: id.into(), name: id.into(), crop: FigureTileFrame { x: 0.0, y: 0.0, width: 1.0, height: 1.0 } }
    }

    #[test]
    fn empty_tiles_is_the_vacuous_topology() {
        let snapshot = PresentSnapshot::default();
        let topology = compute_present_topology(&snapshot);
        assert!(topology.topo_order.is_empty());
        assert!(topology.depth.is_empty());
        assert!(topology.cycle_free);
        assert_eq!(topology.node_count, 0);
    }

    #[test]
    fn depth_matches_persisted_index() {
        let (source, _) = crate::artifacts::present::present_working_scene(&PresentSnapshot::default());
        let snapshot = crate::artifacts::present::present_snapshot_with_tiles(&source, &[tile("a"), tile("b")]);
        let topology = compute_present_topology(&snapshot);
        assert_eq!(topology.depth.get("a"), Some(&0));
        assert_eq!(topology.depth.get("b"), Some(&1));
    }
}
//#endregion 🧪️Tests
