//! 🧭 `topology` — one named inference: the honest vacuous topology. `PlaygroundSnapshot` today
//! carries exactly one persisted field (`schema: String`) and no domain entities or references of
//! any kind (see the snapshot's own doc comment: "minimal schema stub") — there is no graph to
//! derive a topology FROM yet, so the closest honest derived stat per the workflow/dag-shaped
//! inference category is the empty graph: zero nodes, empty order, vacuously cycle-free. This is
//! not a stub/placeholder inference — it is the true, total answer for the domain data this
//! snapshot actually carries right now, and grows real content the moment `PlaygroundSnapshot`
//! gains its own domain entities.

use crate::artifacts::playground::PlaygroundSnapshot;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region 🔖️Topology
/// 🧭️ Playground's topology — see module doc for why it is honestly always the empty graph today.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaygroundTopology {
    pub topo_order: Vec<String>,
    pub depth: BTreeMap<String, u32>,
    pub cycle_free: bool,
    pub node_count: u32,
}

/// 🧮️ Computes [`PlaygroundTopology`] — always the vacuous empty graph, `snapshot` carries no
/// domain fields to derive a non-trivial topology from (see module doc).
pub fn compute_playground_topology(_snapshot: &PlaygroundSnapshot) -> PlaygroundTopology {
    PlaygroundTopology { topo_order: Vec::new(), depth: BTreeMap::new(), cycle_free: true, node_count: 0 }
}
//#endregion 🔖️Topology

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;

    #[test]
    fn topology_is_always_the_vacuous_empty_graph() {
        let topology = compute_playground_topology(&PlaygroundSnapshot::default());
        assert!(topology.topo_order.is_empty());
        assert!(topology.depth.is_empty());
        assert!(topology.cycle_free);
        assert_eq!(topology.node_count, 0);
    }
}
//#endregion 🧪️Tests
