//! 🧭 `topology` — one named inference: the layout document's composition dependency structure.
//! Nodes are parent-page (master) ids, spread ids, and page ids. Edges are the real refs a `Page`
//! already carries: `page.spreadId` (the spread it belongs to) and the optional `page.parentPageId`
//! (the master it's based on) both precede the page in the topo order. Topologically sorted with
//! Kahn's algorithm so `cycleFree` genuinely reports whether those refs ever form a cycle.

use crate::{Page, ParentPage, Spread};
use semio_framework_value_derive::{FromValue, ToValue};
use std::collections::{BTreeMap, BTreeSet, HashMap, VecDeque};

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
    pub fn empty() -> Self {
        Self { topo_order: Vec::new(), depth: BTreeMap::new(), cycle_free: true, node_count: 0 }
    }
}
//#endregion 🔖️Topology

//#region 🔖️Compute
/// 🧭️ Builds the parent-page/spread/page composition graph (via `Page::spreadId`/`parentPageId`)
/// and topologically sorts it.
pub fn compute_layout_topology(parent_pages: &[ParentPage], spreads: &[Spread], pages: &[Page]) -> LayoutTopology {
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
fn topological_sort(nodes: Vec<String>, edges: Vec<(String, String)>) -> LayoutTopology {
    let node_count = nodes.len() as u32;
    let mut indegree: HashMap<String, u32> = nodes.iter().map(|id| (id.clone(), 0)).collect();
    let mut adjacency: HashMap<String, Vec<String>> = HashMap::new();
    for (from, to) in edges {
        if indegree.contains_key(&from) && indegree.contains_key(&to) {
            *indegree.get_mut(&to).expect("checked above") += 1;
            adjacency.entry(from).or_default().push(to);
        }
    }

    let mut depth: BTreeMap<String, u32> = BTreeMap::new();
    let mut queue: VecDeque<String> = VecDeque::new();
    for id in nodes {
        if indegree.get(&id).copied().unwrap_or(0) == 0 {
            depth.insert(id.clone(), 0);
            queue.push_back(id);
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
