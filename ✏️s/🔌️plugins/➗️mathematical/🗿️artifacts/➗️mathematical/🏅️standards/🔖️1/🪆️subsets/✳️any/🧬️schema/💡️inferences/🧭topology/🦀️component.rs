//! 🧭 `topology` — one named inference: the graph playground's topological order. Nodes are
//! `MathematicalNode` ids; edges are `MathematicalEdge`'s own `source`/`target` id pair, taken as
//! given regardless of the `directed` display flag (the edge data always carries an explicit
//! direction). Topologically sorted with Kahn's algorithm so `cycleFree` genuinely reports whether
//! the edges form a cycle, and `depth` gives each node's longest-path distance from a root — the
//! same information the `algorithm: "topo"` playground mode visualizes interactively.

use crate::artifacts::mathematical::MathematicalGraph;
use serde::{Deserialize, Serialize};
// 🌱️ Additive `ToValue`/`FromValue` — see `🦀️component.rs`'s own docstring note on this crate's
// interim (not-yet-serde-free) state.
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};
use std::collections::{BTreeMap, BTreeSet, HashMap, VecDeque};

//#region 🔖️Topology
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct MathematicalTopology {
    pub topo_order: Vec<String>,
    pub depth: BTreeMap<String, u32>,
    pub cycle_free: bool,
    pub node_count: u32,
}
//#endregion 🔖️Topology

//#region 🔖️Compute
/// 🧭️ Builds the node/edge graph from the playground's own `source`/`target` edges and
/// topologically sorts it.
pub async fn compute_mathematical_topology(graph: &MathematicalGraph) -> MathematicalTopology {
    let nodes: Vec<String> = graph.nodes.iter().map(|node| node.id.clone()).collect();
    let edges: Vec<(String, String)> = graph.edges.iter().map(|edge| (edge.source.clone(), edge.target.clone())).collect();
    topological_sort(nodes, edges)
}

/// 🧮️ Kahn's algorithm: a stable (declaration-order-first) topological sort that also yields each
/// node's longest-path depth from a root, and reports `cycleFree = false` when the queue drains
/// before every node is visited (the unvisited remainder is exactly the cyclic subgraph).
async fn topological_sort(nodes: Vec<String>, edges: Vec<(String, String)>) -> MathematicalTopology {
    let node_count = nodes.len() as u32;
    let mut indegree: HashMap<String, u32> = nodes.iter().map(|id| (id.clone(), 0)).collect();
    let mut adjacency: HashMap<String, Vec<String>> = HashMap::new();
    for (from, to) in &edges {
        if indegree.contains_key(from) && indegree.contains_key(to) {
            *indegree.get_mut(to).expect("checked above") += 1;
            adjacency.entry(from.clone()).or_default().push(to.clone());
        }
    }

    let mut depth: BTreeMap<String, u32> = BTreeMap::new();
    let mut queue: VecDeque<String> = VecDeque::new();
    for id in &nodes {
        if indegree.get(id).copied().unwrap_or(0) == 0 {
            depth.insert(id.clone(), 0);
            queue.push_back(id.clone());
        }
    }

    let mut topo_order: Vec<String> = Vec::new();
    let mut visited: BTreeSet<String> = BTreeSet::new();
    while let Some(current) = queue.pop_front() {
        if !visited.insert(current.clone()) {
            continue;
        }
        topo_order.push(current.clone());
        let current_depth = depth.get(&current).copied().unwrap_or(0);
        if let Some(neighbors) = adjacency.get(&current) {
            for next in neighbors {
                let next_depth = current_depth + 1;
                let entry = depth.entry(next.clone()).or_insert(0);
                if next_depth > *entry {
                    *entry = next_depth;
                }
                let remaining = indegree.get_mut(next).expect("every edge target was registered above");
                *remaining -= 1;
                if *remaining == 0 {
                    queue.push_back(next.clone());
                }
            }
        }
    }

    let cycle_free = topo_order.len() as u32 == node_count;
    MathematicalTopology { topo_order, depth, cycle_free, node_count }
}
//#endregion 🔖️Compute

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use crate::artifacts::mathematical::{MathematicalEdge, MathematicalNode};

    async fn node(id: &str) -> MathematicalNode {
        MathematicalNode { id: id.into(), label: id.into(), x: 0.0, y: 0.0 }
    }

    async fn edge(id: &str, source: &str, target: &str) -> MathematicalEdge {
        MathematicalEdge { id: id.into(), source: source.into(), target: target.into() }
    }

    async fn graph(nodes: Vec<MathematicalNode>, edges: Vec<MathematicalEdge>) -> MathematicalGraph {
        MathematicalGraph { directed: true, nodes, edges, algorithm: "topo".into(), algorithm_seed: None }
    }

    //#region 🧪️TopologyLaws
    #[semio_framework_async_macros::async_test]
    async fn a_direct_cycle_between_two_nodes_is_reported() {
        let g = graph(vec![node("a"), node("b")], vec![edge("e1", "a", "b"), edge("e2", "b", "a")]);
        let topology = compute_mathematical_topology(&g);
        assert!(!topology.cycle_free, "a->b->a is a genuine cycle");
    }

    #[semio_framework_async_macros::async_test]
    async fn an_edge_to_a_missing_node_is_dropped_not_a_cycle() {
        let g = graph(vec![node("a")], vec![edge("e1", "a", "missing")]);
        let topology = compute_mathematical_topology(&g);
        assert!(topology.cycle_free);
        assert_eq!(topology.topo_order, vec!["a"]);
    }

    #[semio_framework_async_macros::async_test]
    async fn an_undirected_display_flag_still_uses_the_edge_data_source_target() {
        let mut g = graph(vec![node("a"), node("b")], vec![edge("e1", "a", "b")]);
        g.directed = false;
        let topology = compute_mathematical_topology(&g);
        assert!(topology.cycle_free);
        assert_eq!(topology.depth["b"], 1);
    }
    //#endregion 🧪️TopologyLaws
}
//#endregion 🧪️Tests
