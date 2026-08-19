//! 🧭 `topology` — one named inference: the DAG shape of a procedural3d snapshot's `fixture`
//! widget/synapse graph. A whole-snapshot scalar (not per-entity), so this leaf holds a plain pure
//! function rather than an `InferredField` dependency chain — `fixture` is small and always
//! recomputed wholesale (widget/synapse edits are already coarse-grained mutations).

use crate::artifacts::procedural3d::Procedural3dSnapshot;
use flow::Widget;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, VecDeque};

//#region 🔖️Topology
/// 🧭️ `topology` — the DAG shape of `fixture`'s widget/synapse graph: node/edge counts, a
/// topological order (Kahn's algorithm; empty when the graph has a cycle), whether it is acyclic,
/// and the longest dependency chain's depth (0 for an empty or edge-free graph).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Procedural3dTopology {
    pub node_count: u32,
    pub edge_count: u32,
    pub topo_order: Vec<String>,
    pub depth: u32,
    pub cycle_free: bool,
}

/// 🪪️ A `flow::Widget`'s stable id, across every variant.
async fn widget_id(widget: &Widget) -> &str {
    match widget {
        Widget::Neuron { id, .. }
        | Widget::InputSlider { id, .. }
        | Widget::InputNote { id, .. }
        | Widget::InputImage { id, .. }
        | Widget::Variable { id, .. }
        | Widget::OutputPreview { id, .. }
        | Widget::OutputAction { id, .. }
        | Widget::OutputExport { id, .. }
        | Widget::Cluster { id, .. } => id,
    }
}

/// 🧭️ Computes `topology` from a procedural3d snapshot's `fixture.widgets`/`fixture.synapses` via
/// Kahn's algorithm: widgets are nodes, synapses (`from` → `to`) are directed edges.
pub async fn compute_procedural3d_topology(snapshot: &Procedural3dSnapshot) -> Procedural3dTopology {
    let widget_ids: Vec<String> = snapshot.fixture.widgets.iter().map(|w| widget_id(w).to_string()).collect();
    let mut adjacency: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut in_degree: BTreeMap<String, u32> = widget_ids.iter().map(|id| (id.clone(), 0)).collect();
    for synapse in &snapshot.fixture.synapses {
        adjacency.entry(synapse.from.clone()).or_default().push(synapse.to.clone());
        if let Some(degree) = in_degree.get_mut(&synapse.to) {
            *degree += 1;
        }
    }

    let mut remaining = in_degree.clone();
    let mut queue: VecDeque<String> = in_degree.iter().filter(|(_, degree)| **degree == 0).map(|(id, _)| id.clone()).collect();
    let mut depth_of: BTreeMap<String, u32> = queue.iter().map(|id| (id.clone(), 0)).collect();
    let mut order = Vec::new();
    while let Some(id) = queue.pop_front() {
        let current_depth = *depth_of.get(&id).unwrap_or(&0);
        order.push(id.clone());
        if let Some(children) = adjacency.get(&id) {
            for child in children {
                if let Some(degree) = remaining.get_mut(child) {
                    *degree -= 1;
                    let candidate_depth = current_depth + 1;
                    let entry = depth_of.entry(child.clone()).or_insert(0);
                    if candidate_depth > *entry {
                        *entry = candidate_depth;
                    }
                    if *degree == 0 {
                        queue.push_back(child.clone());
                    }
                }
            }
        }
    }

    let cycle_free = order.len() == widget_ids.len();
    let depth = depth_of.values().copied().max().unwrap_or(0);
    Procedural3dTopology {
        node_count: widget_ids.len() as u32,
        edge_count: snapshot.fixture.synapses.len() as u32,
        topo_order: if cycle_free { order } else { Vec::new() },
        depth: if cycle_free { depth } else { 0 },
        cycle_free,
    }
}
//#endregion 🔖️Topology
