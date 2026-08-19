//! 🧭 `topology` — one named inference: graph-shape stats derived from the wires board's own
//! `nodes`/`edges` arrays (node count, edge count, connected-component count, cycle-freedom).
//! `WiresSnapshot`'s `board_fixture` stays an opaque `dsl::DslValue` by this artifact's own design
//! (see `crate::artifacts::wires`'s module doc), so this leaf reads it generically via
//! `DslValue::get`/`as_array`/`as_str` rather than through the `BoardFixtureDsl` typed mirror,
//! matching how the artifact's own tests already probe `board_fixture` (`empty_snapshot_has_empty_fixtures`).

use dsl::DslValue;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

//#region 🔖️Topology
/// 🧭 Whole-snapshot topology summary — a plain scalar inference (no per-entity `InferredField`
/// caching: recomputing a spanning-forest pass over the board's node/edge graph on every read is
/// cheap at pilot scale, and an undirected mindmap board has no natural per-entity
/// dependency-hash boundary the way puzzle3d's flatten chain does).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WiresTopology {
    pub node_count: u32,
    pub edge_count: u32,
    pub component_count: u32,
    pub cycle_free: bool,
}

impl Default for WiresTopology {
    async fn default() -> Self {
        Self { node_count: 0, edge_count: 0, component_count: 0, cycle_free: true }
    }
}

/// 🔍️ Union-find root lookup with no path compression (pilot-scale graphs only).
async fn find(parent: &BTreeMap<String, String>, id: &str) -> String {
    let mut root = id.to_string();
    while let Some(next) = parent.get(&root) {
        if next == &root {
            break;
        }
        root = next.clone();
    }
    root
}

/// 🧭 Reads `board.nodes`/`board.edges` off the raw `DslValue` board fixture (undirected: a wires
/// board connects node ids directly, no ports) and folds them through a union-find — an edge
/// whose endpoints already share a root closes a cycle; `component_count` is the final number of
/// distinct roots among every counted node.
pub async fn compute_wires_topology(board_fixture: &DslValue) -> WiresTopology {
    let ids: BTreeSet<String> = board_fixture
        .get("nodes")
        .and_then(DslValue::as_array)
        .map(|items| items.iter().filter_map(|item| item.get("id").and_then(DslValue::as_str)).map(str::to_string).collect())
        .unwrap_or_default();

    let edges: Vec<(String, String)> = board_fixture
        .get("edges")
        .and_then(DslValue::as_array)
        .map(|items| {
            items
                .iter()
                .filter_map(|item| {
                    let source = item.get("source").and_then(DslValue::as_str)?;
                    let target = item.get("target").and_then(DslValue::as_str)?;
                    Some((source.to_string(), target.to_string()))
                })
                .collect()
        })
        .unwrap_or_default();

    let mut parent: BTreeMap<String, String> = ids.iter().cloned().map(|id| (id.clone(), id)).collect();
    let mut cycle_free = true;
    let mut edge_count = 0u32;
    for (source, target) in &edges {
        if !ids.contains(source) || !ids.contains(target) {
            continue;
        }
        edge_count += 1;
        let root_source = find(&parent, source);
        let root_target = find(&parent, target);
        if root_source == root_target {
            cycle_free = false;
        } else {
            parent.insert(root_source, root_target);
        }
    }

    let component_count = ids.iter().map(|id| find(&parent, id)).collect::<BTreeSet<_>>().len() as u32;
    WiresTopology { node_count: ids.len() as u32, edge_count, component_count, cycle_free }
}
//#endregion 🔖️Topology

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;

    async fn board(nodes: &[&str], edges: &[(&str, &str)]) -> DslValue {
        DslValue::object([
            ("nodes".into(), DslValue::Array(nodes.iter().map(|id| DslValue::object([("id".into(), DslValue::String((*id).into()))])).collect())),
            ("edges".into(), DslValue::Array(edges.iter().map(|(source, target)| DslValue::object([("source".into(), DslValue::String((*source).into())), ("target".into(), DslValue::String((*target).into()))])).collect())),
        ])
    }

    #[test]
    async fn a_tree_is_cycle_free_with_one_component() {
        let topology = compute_wires_topology(&board(&["a", "b", "c"], &[("a", "b"), ("b", "c")]));
        assert_eq!(topology.node_count, 3);
        assert_eq!(topology.edge_count, 2);
        assert_eq!(topology.component_count, 1);
        assert!(topology.cycle_free);
    }

    #[test]
    async fn a_triangle_closes_a_cycle() {
        let topology = compute_wires_topology(&board(&["a", "b", "c"], &[("a", "b"), ("b", "c"), ("c", "a")]));
        assert!(!topology.cycle_free);
        assert_eq!(topology.component_count, 1);
    }

    #[test]
    async fn disconnected_nodes_count_as_separate_components() {
        let topology = compute_wires_topology(&board(&["a", "b"], &[]));
        assert_eq!(topology.component_count, 2);
        assert_eq!(topology.edge_count, 0);
    }
}
//#endregion 🧪️Tests
