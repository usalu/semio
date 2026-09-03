//! 🧭 `topology` — one named inference: the layout document's composition dependency structure.
//! Nodes are parent-page (master) ids, spread ids, and page ids. Edges are the real refs a `Page`
//! already carries: `page.spreadId` (the spread it belongs to) and the optional `page.parentPageId`
//! (the master it's based on) both precede the page in the topo order. Topologically sorted with
//! Kahn's algorithm so `cycleFree` genuinely reports whether those refs ever form a cycle.

use crate::artifacts::layout::{Page, ParentPage, Spread};
use std::collections::{BTreeMap, BTreeSet, HashMap, VecDeque};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Topology
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct LayoutTopology {
    pub topo_order: Vec<String>,
    pub depth: BTreeMap<String, u32>,
    pub cycle_free: bool,
    pub node_count: u32,
}

impl LayoutTopology {
    /// 🈳️ The topology of an empty document (no parent pages, spreads, or pages) — trivially
    /// cycle-free since there are no nodes to form a cycle among.
    pub async fn empty() -> Self {
        Self { topo_order: Vec::new(), depth: BTreeMap::new(), cycle_free: true, node_count: 0 }
    }
}
//#endregion 🔖️Topology

//#region 🔖️Compute
/// 🧭️ Builds the parent-page/spread/page composition graph (via `Page::spreadId`/`parentPageId`)
/// and topologically sorts it.
pub async fn compute_layout_topology(parent_pages: &[ParentPage], spreads: &[Spread], pages: &[Page]) -> LayoutTopology {
    let mut nodes: Vec<String> = Vec::new();
    let mut edges: Vec<(String, String)> = Vec::new();

    for parent_page in parent_pages {
        nodes.push(parent_page.id.clone());
    }
    for spread in spreads {
        nodes.push(spread.id.clone());
    }
    for page in pages {
        nodes.push(page.id.clone());
    }

    for page in pages {
        edges.push((page.spread_id.clone(), page.id.clone()));
        if let Some(parent_page_id) = &page.parent_page_id {
            edges.push((parent_page_id.clone(), page.id.clone()));
        }
    }

    topological_sort(nodes, edges)
}

/// 🧮️ Kahn's algorithm: a stable (declaration-order-first) topological sort that also yields each
/// node's longest-path depth from a root, and reports `cycleFree = false` when the queue drains
/// before every node is visited (the unvisited remainder is exactly the cyclic subgraph).
async fn topological_sort(nodes: Vec<String>, edges: Vec<(String, String)>) -> LayoutTopology {
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
    LayoutTopology { topo_order, depth, cycle_free, node_count }
}
//#endregion 🔖️Compute

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;

    async fn page(id: &str, spread_id: &str, parent_page_id: Option<&str>) -> Page {
        serde_json::from_value(serde_json::json!({
            "id": id, "name": id, "spreadId": spread_id, "parentPageId": parent_page_id,
            "width": 210.0, "height": 297.0,
            "margins": { "top": 0.0, "right": 0.0, "bottom": 0.0, "left": 0.0 },
            "columns": { "count": 1, "gutter": 0.0 },
            "guides": [], "layerIds": [], "layers": [], "frames": [], "overrides": []
        }))
        .expect("valid page json")
    }

    async fn spread(id: &str, page_ids: Vec<&str>) -> Spread {
        Spread { id: id.into(), name: id.into(), page_ids: page_ids.into_iter().map(String::from).collect() }
    }

    async fn parent_page(id: &str) -> ParentPage {
        ParentPage { id: id.into(), name: id.into(), width: 210.0, height: 297.0, layer_ids: Vec::new(), layers: Vec::new(), frames: Vec::new() }
    }

    //#region 🧪️TopologyLaws
    #[semio_framework_async_macros::async_test]
    async fn empty_document_has_empty_topology() {
        let topology = compute_layout_topology(&[], &[], &[]);
        assert_eq!(topology, LayoutTopology::empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn a_page_dangling_its_spread_ref_still_sorts_cleanly() {
        // 🕳️ `spreadId` points at a spread that doesn't exist in this snapshot — the edge is simply
        // dropped (see `topological_sort`'s `indegree.contains_key` guard), not a cycle.
        let topology = compute_layout_topology(&[], &[], &[page("orphan", "missing-spread", None)]);
        assert!(topology.cycle_free);
        assert_eq!(topology.topo_order, vec!["orphan"]);
    }

    #[semio_framework_async_macros::async_test]
    async fn master_and_spread_both_precede_their_page() {
        let topology = compute_layout_topology(&[parent_page("master-1")], &[spread("spread-1", vec!["page-1"])], &[page("page-1", "spread-1", Some("master-1"))]);
        assert!(topology.cycle_free);
        let page_depth = topology.depth["page-1"];
        assert!(page_depth > topology.depth["master-1"]);
        assert!(page_depth > topology.depth["spread-1"]);
    }
    //#endregion 🧪️TopologyLaws
}
//#endregion 🧪️Tests
