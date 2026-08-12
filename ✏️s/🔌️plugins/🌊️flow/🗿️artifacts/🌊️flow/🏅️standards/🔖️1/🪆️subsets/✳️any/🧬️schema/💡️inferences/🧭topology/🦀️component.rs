//! 🧭 `topology` — one named inference: execution-order topology stats derived from the flow's
//! own widget/synapse graph (topological order, per-widget longest-path depth, cycle-freedom,
//! widget count).

use flow::{SynapseSpec, Widget};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, VecDeque};

//#region 🔖️WidgetId
/// 🎛️ Every `Widget` variant carries its own `id: String` as its first field — this reaches
/// through the tag to read it generically, mirroring `flow`'s own internal `widget_id_for`
/// (private to the host crate, so re-derived here rather than depended on).
fn widget_id(widget: &Widget) -> &str {
    match widget {
        Widget::Neuron { id, .. } => id,
        Widget::InputSlider { id, .. } => id,
        Widget::InputNote { id, .. } => id,
        Widget::InputImage { id, .. } => id,
        Widget::Variable { id, .. } => id,
        Widget::OutputPreview { id, .. } => id,
        Widget::OutputAction { id, .. } => id,
        Widget::OutputExport { id, .. } => id,
        Widget::Cluster { id, .. } => id,
    }
}
//#endregion 🔖️WidgetId

//#region 🔖️Topology
/// 🧭 Whole-snapshot topology summary — a plain scalar inference (no per-entity `InferredField`
/// caching: recomputing a full topological sort over the widget/synapse graph on every read is
/// cheap at pilot scale, and the graph has no natural per-entity dependency-hash boundary the way
/// puzzle3d's flatten chain does).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowTopology {
    pub topo_order: Vec<String>,
    pub depth: BTreeMap<String, u32>,
    pub cycle_free: bool,
    pub node_count: u32,
}

impl Default for FlowTopology {
    fn default() -> Self {
        Self { topo_order: Vec::new(), depth: BTreeMap::new(), cycle_free: true, node_count: 0 }
    }
}

/// 🧭 Kahn's algorithm over `widgets`/`synapses` (`from` -> `to`), deterministic via
/// `BTreeMap`/sorted-adjacency iteration order; widgets left over after the queue drains (a
/// cycle) are appended in id order so `topo_order` always stays a total permutation of every
/// widget id.
pub fn compute_flow_topology(widgets: &[Widget], synapses: &[SynapseSpec]) -> FlowTopology {
    let ids: BTreeSet<String> = widgets.iter().map(|widget| widget_id(widget).to_string()).collect();
    let mut indegree: BTreeMap<String, u32> = ids.iter().cloned().map(|id| (id, 0)).collect();
    let mut adjacency: BTreeMap<String, Vec<String>> = ids.iter().cloned().map(|id| (id, Vec::new())).collect();
    for synapse in synapses {
        if ids.contains(&synapse.from) && ids.contains(&synapse.to) {
            adjacency.get_mut(&synapse.from).expect("from tracked in adjacency").push(synapse.to.clone());
            *indegree.get_mut(&synapse.to).expect("to tracked in indegree") += 1;
        }
    }
    for children in adjacency.values_mut() {
        children.sort();
    }

    let mut remaining = indegree.clone();
    let mut queue: VecDeque<String> = indegree.iter().filter(|(_, &degree)| degree == 0).map(|(id, _)| id.clone()).collect();
    let mut depth: BTreeMap<String, u32> = queue.iter().map(|id| (id.clone(), 0)).collect();
    let mut topo_order = Vec::new();

    while let Some(id) = queue.pop_front() {
        topo_order.push(id.clone());
        let current_depth = *depth.get(&id).unwrap_or(&0);
        if let Some(children) = adjacency.get(&id) {
            for child in children {
                let entry = remaining.get_mut(child).expect("child tracked in remaining");
                *entry -= 1;
                let slot = depth.entry(child.clone()).or_insert(0);
                if current_depth + 1 > *slot {
                    *slot = current_depth + 1;
                }
                if *entry == 0 {
                    queue.push_back(child.clone());
                }
            }
        }
    }

    let cycle_free = topo_order.len() == ids.len();
    if !cycle_free {
        let visited: BTreeSet<String> = topo_order.iter().cloned().collect();
        for id in &ids {
            if !visited.contains(id) {
                depth.entry(id.clone()).or_insert(0);
                topo_order.push(id.clone());
            }
        }
    }

    FlowTopology { topo_order, depth, cycle_free, node_count: ids.len() as u32 }
}
//#endregion 🔖️Topology

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;

    fn slider(id: &str) -> Widget {
        Widget::InputSlider { id: id.into(), value: 0.0, min: 0.0, max: 1.0, step: 0.1 }
    }

    fn synapse(id: &str, from: &str, to: &str) -> SynapseSpec {
        SynapseSpec { id: id.into(), from: from.into(), to: to.into(), from_port: String::new(), to_port: String::new() }
    }

    #[test]
    fn linear_chain_orders_roots_before_leaves_with_increasing_depth() {
        let widgets = vec![slider("a"), slider("b"), slider("c")];
        let synapses = vec![synapse("s1", "a", "b"), synapse("s2", "b", "c")];
        let topology = compute_flow_topology(&widgets, &synapses);
        assert_eq!(topology.topo_order, vec!["a".to_string(), "b".to_string(), "c".to_string()]);
        assert_eq!(topology.depth.get("c"), Some(&2));
        assert!(topology.cycle_free);
        assert_eq!(topology.node_count, 3);
    }

    #[test]
    fn a_cycle_is_reported_as_not_cycle_free_but_still_totals_every_widget() {
        let widgets = vec![slider("a"), slider("b")];
        let synapses = vec![synapse("s1", "a", "b"), synapse("s2", "b", "a")];
        let topology = compute_flow_topology(&widgets, &synapses);
        assert!(!topology.cycle_free);
        assert_eq!(topology.topo_order.len(), 2);
    }
}
//#endregion 🧪️Tests
