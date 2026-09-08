//! 🧭 `topology` — one named inference: the tile filmstrip's persisted order recast as a trivial
//! topology. `PresentationSnapshot.tiles` is a flat, unordered-by-reference list (no `SlotRef`/edge type
//! exists on `FigureTileDraft`), so the honest derived stat per the workflow/dag-shaped inference
//! category is a linear chain: `topoOrder` is the tile ids in persisted order, `depth` is each
//! tile's own index in that order (how far into the sequence it sits), `cycleFree` is always `true`
//! (a flat `Vec` cannot encode a cycle), `nodeCount` is `tiles.len()`. Whole-snapshot scalar, so a
//! plain function suffices — no `InferredField`/per-entity caching needed (see the family root's
//! doc comment for why).

use crate::PresentationSnapshot;
use std::collections::BTreeMap;

//#region 🔖️Topology
/// 🧭️ Presentation's tile-filmstrip topology — see module doc for the honest-degenerate-chain shape.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PresentationTopology {
    pub topo_order: Vec<String>,
    pub depth: BTreeMap<String, u32>,
    pub cycle_free: bool,
    pub node_count: u32,
}

/// 🧮️ Computes [`PresentationTopology`] from a presentation snapshot's persisted tile order (read through the
/// working-scene accessor off the `presentation` child handle — see
/// `crate::presentation_working_scene`).
pub fn compute_presentation_topology(snapshot: &PresentationSnapshot) -> PresentationTopology {
    let (_, tiles) = crate::presentation_working_scene(snapshot);
    let topo_order: Vec<String> = tiles.iter().map(|tile| tile.id.clone()).collect();
    let depth = topo_order.iter().enumerate().map(|(index, id)| (id.clone(), index as u32)).collect();
    PresentationTopology { topo_order, depth, cycle_free: true, node_count: tiles.len() as u32 }
}
//#endregion 🔖️Topology

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
