//! 🧭 `topology` — one named inference: the hierarchy shape of `elements` (via `parentId`).
//! `ProgramElement.parent_id` is the only structural (non-scalar-requirement) relationship on the
//! snapshot's persistent fields, so a topology derivation over it — node/root counts, max depth,
//! cycle-freedom, and a stable topological order — is the honest whole-snapshot statistic (per the
//! family doc's workflow/dag-shaped guidance: "the closest honest derived stat from that
//! artifact's actual fields"). Whole-snapshot scalar, not per-entity, so this leaf holds a plain
//! pure function rather than an `InferredField` chain — the family root's
//! `impl protocol::Inference<ProgramSnapshot>` calls it directly.

use crate::artifacts::program::standards::v1::subsets::any::schema::registers::ProgramElement;
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};

//#region 🔖️ProgramTopology
/// 🧭️ Hierarchy shape of `elements`, derived from `parentId` links.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramTopology {
    pub node_count: u64,
    pub root_count: u64,
    pub max_depth: u64,
    pub cycle_free: bool,
    pub topo_order: Vec<String>,
}

impl Default for ProgramTopology {
    /// 🌉️ Zero elements: 0 nodes, 0 roots, depth 0, vacuously cycle-free, empty order — matches
    /// `compute_topology(&[])` exactly, so no hand-fixup is needed for the zero-element case.
    fn default() -> Self {
        Self { node_count: 0, root_count: 0, max_depth: 0, cycle_free: true, topo_order: Vec::new() }
    }
}
//#endregion 🔖️ProgramTopology

//#region 🔖️Derivation
/// 🧭️ Walks `elements`' `parentId` links to a full topology summary in one pass-family:
/// - a parent id that does not resolve to another element in the same snapshot is treated as "no
///   parent" (a dangling reference can't anchor a real depth/order relationship);
/// - `cycleFree` uses three-color DFS (white/gray/black) — a cycle is any edge back into a
///   currently-gray (in-progress) node;
/// - `maxDepth` is guarded against cycles independently (a per-call visited-set stops infinite
///   recursion, returning a 0 contribution for any node already on the current recursion path);
/// - `topoOrder` is a deterministic BFS from lexicographically-sorted roots, children visited in
///   sorted-id order; any element unreached by that BFS (only possible inside a cycle) is appended
///   afterward in sorted-id order, so the output is always a total, deterministic order over every
///   element id regardless of `cycleFree`.
pub fn compute_topology(elements: &[ProgramElement]) -> ProgramTopology {
    let ids: Vec<String> = elements.iter().map(|e| e.header.id.0.clone()).collect();
    let id_set: HashSet<&str> = ids.iter().map(|s| s.as_str()).collect();
    let parent_of: HashMap<String, Option<String>> = elements
        .iter()
        .map(|e| {
            let id = e.header.id.0.clone();
            let parent = e.parent_id.as_ref().map(|p| p.0.clone()).filter(|p| id_set.contains(p.as_str()));
            (id, parent)
        })
        .collect();

    let node_count = elements.len() as u64;
    let root_count = parent_of.values().filter(|p| p.is_none()).count() as u64;
    let cycle_free = is_cycle_free(&ids, &parent_of);
    let max_depth = max_depth(&ids, &parent_of);
    let topo_order = topo_order(&ids, &parent_of);

    ProgramTopology { node_count, root_count, max_depth, cycle_free, topo_order }
}

#[derive(Clone, Copy, PartialEq)]
enum Color {
    White,
    Gray,
    Black,
}

fn is_cycle_free(ids: &[String], parent_of: &HashMap<String, Option<String>>) -> bool {
    let mut colors: HashMap<&str, Color> = ids.iter().map(|id| (id.as_str(), Color::White)).collect();
    let mut cycle_free = true;

    fn visit<'a>(node: &'a str, parent_of: &'a HashMap<String, Option<String>>, colors: &mut HashMap<&'a str, Color>, cycle_free: &mut bool) {
        match colors.get(node).copied() {
            Some(Color::Black) => return,
            Some(Color::Gray) => {
                *cycle_free = false;
                return;
            }
            _ => {}
        }
        colors.insert(node, Color::Gray);
        if let Some(Some(parent)) = parent_of.get(node) {
            visit(parent.as_str(), parent_of, colors, cycle_free);
        }
        colors.insert(node, Color::Black);
    }

    for id in ids {
        visit(id.as_str(), parent_of, &mut colors, &mut cycle_free);
    }
    cycle_free
}

fn max_depth(ids: &[String], parent_of: &HashMap<String, Option<String>>) -> u64 {
    let mut depth_of: HashMap<&str, u64> = HashMap::new();

    fn depth<'a>(node: &'a str, parent_of: &'a HashMap<String, Option<String>>, depth_of: &mut HashMap<&'a str, u64>, guard: &mut HashSet<&'a str>) -> u64 {
        if let Some(d) = depth_of.get(node) {
            return *d;
        }
        if !guard.insert(node) {
            return 0;
        }
        let d = match parent_of.get(node) {
            Some(Some(parent)) => 1 + depth(parent.as_str(), parent_of, depth_of, guard),
            _ => 0,
        };
        depth_of.insert(node, d);
        d
    }

    let mut max = 0u64;
    for id in ids {
        let mut guard = HashSet::new();
        max = max.max(depth(id.as_str(), parent_of, &mut depth_of, &mut guard));
    }
    max
}

fn topo_order(ids: &[String], parent_of: &HashMap<String, Option<String>>) -> Vec<String> {
    let mut children_of: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for id in ids {
        if let Some(Some(parent)) = parent_of.get(id) {
            children_of.entry(parent.as_str()).or_default().push(id.as_str());
        }
    }
    for children in children_of.values_mut() {
        children.sort();
    }

    let mut roots: Vec<&str> = ids.iter().filter(|id| matches!(parent_of.get(id.as_str()), Some(None))).map(|s| s.as_str()).collect();
    roots.sort();

    let mut order: Vec<String> = Vec::new();
    let mut visited: HashSet<&str> = HashSet::new();
    let mut queue: VecDeque<&str> = roots.into_iter().collect();
    while let Some(node) = queue.pop_front() {
        if !visited.insert(node) {
            continue;
        }
        order.push(node.to_string());
        if let Some(children) = children_of.get(node) {
            for child in children {
                queue.push_back(child);
            }
        }
    }

    let mut leftover: Vec<&str> = ids.iter().map(|s| s.as_str()).filter(|id| !visited.contains(id)).collect();
    leftover.sort();
    order.extend(leftover.into_iter().map(String::from));
    order
}
//#endregion 🔖️Derivation

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use crate::artifacts::program::kernel::{EntityHeader, EntityId, QuantitySpec};
    use crate::artifacts::program::standards::v1::subsets::any::schema::registers::ProgramElementKind;

    fn element(id: &str, parent: Option<&str>) -> ProgramElement {
        ProgramElement {
            header: EntityHeader::new(EntityId(id.into()), id),
            code: id.into(),
            kind: ProgramElementKind::Room,
            parent_id: parent.map(|p| EntityId(p.into())),
            level: None,
            area: QuantitySpec::default(),
            volume: QuantitySpec::default(),
            height: QuantitySpec::default(),
            occupancy: QuantitySpec::default(),
            function_ids: Vec::new(),
            activity_ids: Vec::new(),
            user_profile_ids: Vec::new(),
            adjacency_ids: Vec::new(),
            quantity_ids: Vec::new(),
            requirement_ids: Vec::new(),
            location_hint: None,
            orientation: None,
            daylight_requirement: None,
            acoustic_class: None,
            security_zone: None,
            flexibility_notes: Vec::new(),
            growth_allocation: None,
            circulation_role: None,
            visibility_level: None,
            adjacency_preferences: Vec::new(),
            environmental_zone: None,
        }
    }

    #[test]
    fn empty_elements_yield_default_topology() {
        assert_eq!(compute_topology(&[]), ProgramTopology::default());
    }

    #[test]
    fn linear_chain_has_matching_depth_and_order() {
        let elements = vec![element("a", None), element("b", Some("a")), element("c", Some("b"))];
        let topology = compute_topology(&elements);
        assert_eq!(topology.node_count, 3);
        assert_eq!(topology.root_count, 1);
        assert_eq!(topology.max_depth, 2);
        assert!(topology.cycle_free);
        assert_eq!(topology.topo_order, vec!["a".to_string(), "b".to_string(), "c".to_string()]);
    }

    #[test]
    fn a_cycle_is_detected_and_still_yields_a_total_order() {
        let elements = vec![element("a", Some("b")), element("b", Some("a"))];
        let topology = compute_topology(&elements);
        assert_eq!(topology.node_count, 2);
        assert_eq!(topology.root_count, 0);
        assert!(!topology.cycle_free);
        assert_eq!(topology.topo_order.len(), 2);
    }

    #[test]
    fn a_dangling_parent_id_is_treated_as_a_root() {
        let elements = vec![element("a", Some("nonexistent"))];
        let topology = compute_topology(&elements);
        assert_eq!(topology.root_count, 1);
        assert_eq!(topology.max_depth, 0);
    }
}
//#endregion 🧪️Tests
