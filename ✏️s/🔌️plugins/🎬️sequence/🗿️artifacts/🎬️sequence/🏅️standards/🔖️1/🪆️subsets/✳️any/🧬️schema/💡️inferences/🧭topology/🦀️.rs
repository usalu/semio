//! 🧭 `topology` — one named inference: a real topological sort over `SequenceSnapshot.steps` +
//! `.edges` (a genuine DAG — `SequenceEdge { from, to }` are step-id references, exactly the
//! `topoOrder`/`depth`/`cycleFree`/`nodeCount` shape the workflow/dag-shaped inference category
//! names). Kahn's algorithm, processing zero-indegree steps in persisted step order for a
//! deterministic result: `topoOrder` is a valid topological order when the graph is acyclic (any
//! step left over after the queue drains is a cycle member, appended in persisted order so the
//! result stays TOTAL per `Inference`'s law — never panics, never fails); `depth` is each step's
//! longest path length from any root (a step with no incoming edges), computed in the same pass;
//! `cycleFree` is whether every step was reachable by the algorithm. Whole-snapshot scalar, so a
//! plain function suffices — no `InferredField`/per-entity caching needed (see the family root's
//! doc comment for why).

use crate::SequenceSnapshot;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, VecDeque};

//#region 🔖️Topology
/// 🧭️ Sequence's step-DAG topology — see module doc for the Kahn's-algorithm derivation.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::ToValue, dsl::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct SequenceTopology {
    pub topo_order: Vec<String>,
    pub depth: BTreeMap<String, u32>,
    pub cycle_free: bool,
    pub node_count: u32,
}

/// 🧮️ Computes [`SequenceTopology`] via Kahn's algorithm over `steps`/`edges` (read off the
/// composed content child's working scene — see `sequence_working_scene`'s doc comment). Edges
/// referencing a missing step id are ignored (dangling refs never a source of truth for topology).
pub fn compute_sequence_topology(snapshot: &SequenceSnapshot) -> SequenceTopology {
    let scene = crate::sequence_working_scene(snapshot);
    let ids: Vec<String> = scene.steps.iter().map(|step| step.id.clone()).collect();
    let known: std::collections::BTreeSet<&String> = ids.iter().collect();

    let mut adjacency: BTreeMap<String, Vec<String>> = ids.iter().map(|id| (id.clone(), Vec::new())).collect();
    let mut in_degree: BTreeMap<String, u32> = ids.iter().map(|id| (id.clone(), 0)).collect();
    for edge in &scene.edges {
        if known.contains(&edge.from) && known.contains(&edge.to) {
            adjacency.get_mut(&edge.from).expect("from is known").push(edge.to.clone());
            *in_degree.get_mut(&edge.to).expect("to is known") += 1;
        }
    }

    let mut remaining_in_degree = in_degree.clone();
    let mut queue: VecDeque<String> = ids.iter().filter(|id| in_degree[*id] == 0).cloned().collect();
    let mut depth: BTreeMap<String, u32> = ids.iter().map(|id| (id.clone(), 0)).collect();
    let mut topo_order = Vec::with_capacity(ids.len());

    while let Some(id) = queue.pop_front() {
        topo_order.push(id.clone());
        let current_depth = depth[&id];
        for next in adjacency.get(&id).cloned().unwrap_or_default() {
            let candidate = current_depth + 1;
            let entry = depth.entry(next.clone()).or_insert(0);
            if candidate > *entry {
                *entry = candidate;
            }
            let deg = remaining_in_degree.get_mut(&next).expect("next tracked in in_degree");
            *deg -= 1;
            if *deg == 0 {
                queue.push_back(next);
            }
        }
    }

    let cycle_free = topo_order.len() == ids.len();
    if !cycle_free {
        // 🎯️ Total per `Inference`'s law: append any step a cycle kept out of the queue, in
        // persisted order, so `topoOrder` still covers every step even though it is no longer a
        // valid topological order for the cyclic remainder.
        for id in &ids {
            if !topo_order.contains(id) {
                topo_order.push(id.clone());
            }
        }
    }

    SequenceTopology { topo_order, depth, cycle_free, node_count: ids.len() as u32 }
}
//#endregion 🔖️Topology

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
