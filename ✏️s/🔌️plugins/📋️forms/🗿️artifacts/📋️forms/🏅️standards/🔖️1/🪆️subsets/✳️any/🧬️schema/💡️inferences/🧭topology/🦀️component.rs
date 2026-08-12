//! 🧭 `topology` — one named inference: the form's step/block dependency structure. Nodes are step
//! ids and block ids (flattened across all steps); edges are (a) sequential document order — each
//! step follows the previous step, and each block follows the previous node within its step — plus
//! (b) `condition` data-dependency edges, added whenever a block's visibility condition references
//! another block's id via `FormExpr::Var`. Topologically sorted with Kahn's algorithm so `cycleFree`
//! genuinely reports whether the referenced conditions ever form a cycle, rather than assuming one
//! never occurs.

use crate::artifacts::forms::{FormExpr, FormStep};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};

//#region 🔖️Topology
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormsTopology {
    pub topo_order: Vec<String>,
    pub depth: BTreeMap<String, u32>,
    pub cycle_free: bool,
    pub node_count: u32,
}
//#endregion 🔖️Topology

//#region 🔖️Compute
fn collect_condition_vars(expr: &FormExpr, names: &mut Vec<String>) {
    match expr {
        FormExpr::Const { .. } => {}
        FormExpr::Var { name } => names.push(name.clone()),
        FormExpr::Eq { left, right } => {
            collect_condition_vars(left, names);
            collect_condition_vars(right, names);
        }
        FormExpr::And { items } | FormExpr::Or { items } => {
            for item in items {
                collect_condition_vars(item, names);
            }
        }
        FormExpr::Truthy { expr: inner } => collect_condition_vars(inner, names),
    }
}

/// 🧭️ Builds the step/block dependency graph (declaration order + condition-var reads) and
/// topologically sorts it.
pub fn compute_forms_topology(steps: &[FormStep]) -> FormsTopology {
    let mut nodes: Vec<String> = Vec::new();
    let mut edges: Vec<(String, String)> = Vec::new();
    let mut block_ids: HashSet<String> = HashSet::new();
    let mut previous: Option<String> = None;

    for step in steps {
        nodes.push(step.id.clone());
        if let Some(prev) = previous.take() {
            edges.push((prev, step.id.clone()));
        }
        previous = Some(step.id.clone());
        for block in &step.blocks {
            nodes.push(block.id.clone());
            block_ids.insert(block.id.clone());
            if let Some(prev) = previous.take() {
                edges.push((prev, block.id.clone()));
            }
            previous = Some(block.id.clone());
        }
    }

    for step in steps {
        for block in &step.blocks {
            if let Some(condition) = &block.condition {
                let mut names = Vec::new();
                collect_condition_vars(condition, &mut names);
                for name in names {
                    if name != block.id && block_ids.contains(&name) {
                        edges.push((name, block.id.clone()));
                    }
                }
            }
        }
    }

    topological_sort(nodes, edges)
}

/// 🧮️ Kahn's algorithm: a stable (declaration-order-first) topological sort that also yields each
/// node's longest-path depth from a root, and reports `cycleFree = false` when the queue drains
/// before every node is visited (the unvisited remainder is exactly the cyclic subgraph).
fn topological_sort(nodes: Vec<String>, edges: Vec<(String, String)>) -> FormsTopology {
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
    FormsTopology { topo_order, depth, cycle_free, node_count }
}
//#endregion 🔖️Compute

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use crate::artifacts::forms::FormQuestion;

    fn block(id: &str, condition: Option<FormExpr>) -> FormQuestion {
        FormQuestion {
            id: id.into(),
            label: id.into(),
            kind: "text".into(),
            description: None,
            required: None,
            placeholder: None,
            default: None,
            min: None,
            max: None,
            step: None,
            unit: None,
            text: None,
            options: None,
            fields: None,
            schema: None,
            src: None,
            accept: None,
            fixture_slug: None,
            params: None,
            condition,
        }
    }

    fn step(id: &str, blocks: Vec<FormQuestion>) -> FormStep {
        FormStep { id: id.into(), title: id.into(), description: None, blocks }
    }

    //#region 🧪️TopologyLaws
    #[test]
    fn a_direct_cycle_between_two_blocks_is_reported() {
        let a = block("a", Some(FormExpr::Var { name: "b".into() }));
        let b = block("b", Some(FormExpr::Var { name: "a".into() }));
        let topology = compute_forms_topology(&[step("s1", vec![a, b])]);
        assert!(!topology.cycle_free, "a reads b and b reads a — this is a genuine cycle");
    }

    #[test]
    fn sequential_steps_without_conditions_stay_cycle_free_with_increasing_depth() {
        let topology = compute_forms_topology(&[step("s1", vec![block("q1", None)]), step("s2", vec![block("q2", None)])]);
        assert!(topology.cycle_free);
        assert_eq!(topology.topo_order, vec!["s1", "q1", "s2", "q2"]);
        assert!(topology.depth["s2"] > topology.depth["s1"]);
    }
    //#endregion 🧪️TopologyLaws
}
//#endregion 🧪️Tests
