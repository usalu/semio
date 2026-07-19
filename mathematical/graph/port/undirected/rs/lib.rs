//! 🔀 The undirected port (multi-edge) graph family — NetworkX `MultiGraph` parity facade over `mathematical_graph::Storage<Ported, Undirected>`.

use std::collections::{BTreeMap, BTreeSet};

use mathematical_graph::{
    pairwise, AttrView, AttrWeight, Directed, EdgeId, EdgeRef, EdgeSubgraphView, EdgeWeights, GraphView, HandleId, Normal, NodeId,
    Ported, PropertyBag, PropertyValue, Storage, SubgraphView, Undirected,
};

// #region 🔖Construction
/// 🕸️ NetworkX `MultiGraph` parity facade: an undirected multigraph wrapping `Storage<Ported, Undirected>`. Every node gets a lazily-allocated "default handle" (see `handle_of`) so callers work at plain `NodeId` level, matching NetworkX's handle-free `MultiGraph` API — the `Ported` port model is only an internal storage detail here. `Storage<Ported,_>::add_edge_with` always mints a fresh `EdgeId`, which directly plays the role of NetworkX's per-pair insertion "key" (`G.add_edge(u, v)` returns a key); this is a deliberate simplification since `EdgeId` is globally unique/monotone rather than scoped per node pair like NetworkX's keys.
#[derive(Clone, Debug)]
pub struct PortUndirectedGraph {
    storage: Storage<Ported, Undirected>,
    default_handle: BTreeMap<NodeId, HandleId>,
}

impl Default for PortUndirectedGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl PortUndirectedGraph {
    /// 🆕 Empty multigraph.
    pub fn new() -> Self {
        Self { storage: Storage::new(), default_handle: BTreeMap::new() }
    }

    /// 🪝 Looks up, or lazily allocates via `Storage::add_handle`, the single default handle every node routes all of its edges through. Panics if `node` isn't already present in `storage` — every call site uses `ensure_node` first, so this never fires in practice.
    fn handle_of(&mut self, node: NodeId) -> HandleId {
        match self.default_handle.get(&node) {
            Some(&handle) => handle,
            None => {
                let handle = self.storage.add_handle(node).expect("handle_of is only called after the node is known to exist");
                self.default_handle.insert(node, handle);
                handle
            }
        }
    }

    /// 🌱 Creates `id` with empty attrs if not already present (NetworkX `MultiGraph.add_edge` auto-creates endpoint nodes).
    fn ensure_node(&mut self, id: NodeId) {
        if !self.storage.contains_node(id) {
            self.storage.add_node_with_id(id, PropertyBag::default());
        }
    }
}
// #endregion 🔖Construction

// #region 🔖NodeOps
impl PortUndirectedGraph {
    pub fn add_node(&mut self) -> NodeId {
        self.storage.add_node()
    }

    pub fn add_node_with(&mut self, attrs: PropertyBag) -> NodeId {
        self.storage.add_node_with(attrs)
    }

    pub fn add_node_with_id(&mut self, id: NodeId, attrs: PropertyBag) -> NodeId {
        self.storage.add_node_with_id(id, attrs)
    }

    pub fn add_nodes_from(&mut self, nodes: impl IntoIterator<Item = NodeId>) {
        for id in nodes {
            self.add_node_with_id(id, PropertyBag::default());
        }
    }

    /// 🗑️ Removes `id` and cascades (edges, handles) via `Storage::remove_node`, then drops this facade's own `default_handle` bookkeeping entry for it.
    pub fn remove_node(&mut self, id: NodeId) -> bool {
        let removed = self.storage.remove_node(id);
        if removed {
            self.default_handle.remove(&id);
        }
        removed
    }

    pub fn remove_nodes_from(&mut self, nodes: impl IntoIterator<Item = NodeId>) {
        for id in nodes {
            self.remove_node(id);
        }
    }

    pub fn has_node(&self, id: NodeId) -> bool {
        self.storage.contains_node(id)
    }

    pub fn number_of_nodes(&self) -> usize {
        self.storage.node_count()
    }

    pub fn order(&self) -> usize {
        self.storage.node_count()
    }

    pub fn nodes(&self) -> impl Iterator<Item = NodeId> + '_ {
        self.storage.nodes()
    }
}
// #endregion 🔖NodeOps

// #region 🔖EdgeOps
impl PortUndirectedGraph {
    /// ➕ Always creates a NEW parallel edge (NetworkX `MultiGraph.add_edge`); auto-creates `u`/`v` if unseen.
    pub fn add_edge(&mut self, u: NodeId, v: NodeId) -> EdgeId {
        self.add_edge_with(u, v, PropertyBag::default())
    }

    pub fn add_edge_with(&mut self, u: NodeId, v: NodeId, attrs: PropertyBag) -> EdgeId {
        self.ensure_node(u);
        self.ensure_node(v);
        let hu = self.handle_of(u);
        let hv = self.handle_of(v);
        self.storage.add_edge_with(hu, hv, attrs)
    }

    pub fn add_edges_from(&mut self, edges: impl IntoIterator<Item = (NodeId, NodeId)>) -> Vec<EdgeId> {
        edges.into_iter().map(|(u, v)| self.add_edge(u, v)).collect()
    }

    pub fn add_weighted_edges_from(&mut self, edges: impl IntoIterator<Item = (NodeId, NodeId, f64)>) -> Vec<EdgeId> {
        edges
            .into_iter()
            .map(|(u, v, weight)| {
                let mut attrs = PropertyBag::default();
                attrs.insert("weight".to_string(), PropertyValue::Number(weight));
                self.add_edge_with(u, v, attrs)
            })
            .collect()
    }

    /// 🗑️ Keyed removal — NetworkX `remove_edge(u, v, key)`. Here `EdgeId` IS the key (see the crate doc's simplification note), so this facade's `remove_edge` takes just the id rather than the simple-graph facades' `(u, v)`-keyed removal.
    pub fn remove_edge(&mut self, id: EdgeId) -> bool {
        self.storage.remove_edge(id)
    }

    /// 🗑️ NetworkX's convenience `remove_edge(u, v)` without a key removes an arbitrary parallel edge; this picks the smallest `EdgeId` for determinism.
    pub fn remove_one_edge(&mut self, u: NodeId, v: NodeId) -> bool {
        match self.edges_between(u, v).min() {
            Some(id) => self.storage.remove_edge(id),
            None => false,
        }
    }

    pub fn has_edge(&self, u: NodeId, v: NodeId) -> bool {
        self.storage.edges_between(u, v).next().is_some()
    }

    /// 🔢 Total edge count if both `u`/`v` are `None`; edge count between a specific pair if both are `Some` (the mixed one-`Some` NetworkX form is unsupported here — it falls back to the total).
    pub fn number_of_edges(&self, u: Option<NodeId>, v: Option<NodeId>) -> usize {
        match (u, v) {
            (Some(u), Some(v)) => self.storage.edges_between(u, v).count(),
            _ => self.storage.edge_count(),
        }
    }

    /// 🏷️ Keyed attribute lookup — several edges can share a `(u, v)` pair, so lookup is always by `EdgeId`.
    pub fn get_edge_data(&self, id: EdgeId) -> Option<&PropertyBag> {
        self.storage.edge_attrs(id)
    }

    pub fn edges_between(&self, u: NodeId, v: NodeId) -> impl Iterator<Item = EdgeId> + '_ {
        self.storage.edges_between(u, v).map(|e| e.id)
    }

    pub fn add_path(&mut self, nodes: impl IntoIterator<Item = NodeId>) -> Vec<EdgeId> {
        let nodes: Vec<NodeId> = nodes.into_iter().collect();
        pairwise(&nodes).map(|(a, b)| self.add_edge(a, b)).collect()
    }

    /// 🔁 Mirrors NetworkX `add_cycle`: a single node produces one self-loop, an empty input produces no edges, everything else wraps the last node back to the first.
    pub fn add_cycle(&mut self, nodes: impl IntoIterator<Item = NodeId>) -> Vec<EdgeId> {
        let nodes: Vec<NodeId> = nodes.into_iter().collect();
        match nodes.len() {
            0 => Vec::new(),
            1 => vec![self.add_edge(nodes[0], nodes[0])],
            _ => {
                let mut ids: Vec<EdgeId> = pairwise(&nodes).map(|(a, b)| self.add_edge(a, b)).collect();
                ids.push(self.add_edge(nodes[nodes.len() - 1], nodes[0]));
                ids
            }
        }
    }

    /// ⭐ First node is the hub, the rest are leaves connected to it (NetworkX `add_star`).
    pub fn add_star(&mut self, nodes: impl IntoIterator<Item = NodeId>) -> Vec<EdgeId> {
        let nodes: Vec<NodeId> = nodes.into_iter().collect();
        let Some(&hub) = nodes.first() else { return Vec::new() };
        nodes[1..].iter().map(|&leaf| self.add_edge(hub, leaf)).collect()
    }
}
// #endregion 🔖EdgeOps

// #region 🔖Queries
impl PortUndirectedGraph {
    /// 🎯 Distinct neighbor node ids; a node with several parallel edges to the same neighbor still yields it once (`Storage`'s adjacency map is already keyed by neighbor id, so this is deterministic and deduplicated for free).
    pub fn neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> + '_ {
        self.storage.neighbors(node)
    }

    /// 🔢 Counts every parallel edge; a self-loop counts twice (delegates straight to `Storage::degree`, which already encodes both conventions).
    pub fn degree(&self, node: NodeId) -> usize {
        self.storage.degree(node)
    }

    /// ⚖️ Sums the named numeric attribute (default `1.0` per edge) over every incident edge, counting a self-loop's weight twice like `degree`.
    pub fn weighted_degree(&self, node: NodeId, weight_name: &str) -> f64 {
        let weight = AttrWeight { graph: &self.storage, name: weight_name, default: 1.0 };
        let mut total = 0.0;
        for neighbor in self.storage.neighbors(node) {
            for edge in self.storage.edges_between(node, neighbor) {
                total += weight.weight(edge);
            }
        }
        total
    }

    /// 📐 `2*m/(n*(n-1))` using the multi-edge count `m`, matching NetworkX's `density` formula for `MultiGraph` — deliberately NOT clamped to `1.0`, since parallel edges can push it arbitrarily high.
    pub fn density(&self) -> f64 {
        let n = self.storage.node_count() as f64;
        let m = self.storage.edge_count() as f64;
        if n <= 1.0 {
            0.0
        } else {
            2.0 * m / (n * (n - 1.0))
        }
    }

    /// 🕳️ NetworkX `is_empty`: true when the graph has no EDGES (nodes may still be present).
    pub fn is_empty(&self) -> bool {
        self.storage.edge_count() == 0
    }
}
// #endregion 🔖Queries

// #region 🔖Transforms
impl PortUndirectedGraph {
    pub fn copy(&self) -> Self {
        self.clone()
    }

    /// 🔎 Owned copy restricted to `nodes`; rebuilt via a `SubgraphView` over `self`, so it gets fresh `NodeId`-stable but new `EdgeId`s (see the core `Views` module doc on why views never mutate in place).
    pub fn subgraph(&self, nodes: impl IntoIterator<Item = NodeId>) -> Self {
        let view = SubgraphView::new(&self.storage, nodes);
        let mut out = Self::new();
        for node in view.nodes() {
            let attrs = view.node_attrs(node).cloned().unwrap_or_default();
            out.storage.add_node_with_id(node, attrs);
        }
        for edge in view.edges() {
            let attrs = view.edge_attrs(edge.id).cloned().unwrap_or_default();
            out.add_edge_with(edge.u, edge.v, attrs);
        }
        out.storage.graph_attrs_mut().extend(self.storage.graph_attrs().clone());
        out
    }

    /// 🔎 Owned copy restricted to `edges`; nodes are exactly those edges' endpoints (`EdgeSubgraphView`).
    pub fn edge_subgraph(&self, edges: impl IntoIterator<Item = EdgeId>) -> Self {
        let view = EdgeSubgraphView::new(&self.storage, edges);
        let mut out = Self::new();
        for node in view.nodes() {
            let attrs = view.node_attrs(node).cloned().unwrap_or_default();
            out.storage.add_node_with_id(node, attrs);
        }
        for edge in view.edges() {
            let attrs = view.edge_attrs(edge.id).cloned().unwrap_or_default();
            out.add_edge_with(edge.u, edge.v, attrs);
        }
        out.storage.graph_attrs_mut().extend(self.storage.graph_attrs().clone());
        out
    }

    /// 🧵 Collapses parallel edges into one simple edge per node pair, summing each collapsed group's `"weight"` attribute (missing weight defaults to `1.0` per parallel edge, matching `EdgeWeights`'s convention). Returns the raw `Storage`, not a sibling facade type, to avoid a circular dependency on the not-yet-built `mathematical_graph_normal_undirected` crate — not a literal NetworkX method, but a common multigraph-to-simple-graph convenience.
    pub fn to_simple(&self) -> Storage<Normal, Undirected> {
        let mut simple = Storage::<Normal, Undirected>::new();
        for node in self.storage.nodes() {
            let attrs = self.storage.node_attrs(node).cloned().unwrap_or_default();
            simple.add_node_with_id(node, attrs);
        }
        let mut weight_sums: BTreeMap<(NodeId, NodeId), f64> = BTreeMap::new();
        for edge in self.storage.edges() {
            let pair = if edge.u <= edge.v { (edge.u, edge.v) } else { (edge.v, edge.u) };
            let weight = self.storage.edge_attrs(edge.id).and_then(|attrs| attrs.get("weight")).and_then(PropertyValue::as_f64).unwrap_or(1.0);
            *weight_sums.entry(pair).or_insert(0.0) += weight;
        }
        for ((u, v), weight) in weight_sums {
            let mut attrs = PropertyBag::default();
            attrs.insert("weight".to_string(), PropertyValue::Number(weight));
            simple.add_edge_with(u, v, attrs);
        }
        simple.graph_attrs_mut().extend(self.storage.graph_attrs().clone());
        simple
    }

    /// ➡️ Each undirected parallel edge becomes two directed parallel edges (one each way), attributes cloned onto both; an undirected self-loop becomes exactly ONE directed self-loop, since NetworkX's own `to_directed` walks the (already-collapsed) adjacency structure where a self-loop appears only once. Builds a fresh handle-bookkeeping map for the new `Directed` storage — the old `default_handle` handle ids belong to a different `Storage` instance and can't be reused.
    pub fn to_directed(&self) -> Storage<Ported, Directed> {
        let mut directed = Storage::<Ported, Directed>::new();
        for node in self.storage.nodes() {
            let attrs = self.storage.node_attrs(node).cloned().unwrap_or_default();
            directed.add_node_with_id(node, attrs);
        }
        let mut handles: BTreeMap<NodeId, HandleId> = BTreeMap::new();
        for edge in self.storage.edges() {
            let attrs = self.storage.edge_attrs(edge.id).cloned().unwrap_or_default();
            let hu = *handles.entry(edge.u).or_insert_with(|| directed.add_handle(edge.u).expect("node added above"));
            let hv = *handles.entry(edge.v).or_insert_with(|| directed.add_handle(edge.v).expect("node added above"));
            directed.add_edge_with(hu, hv, attrs.clone());
            if edge.u != edge.v {
                directed.add_edge_with(hv, hu, attrs);
            }
        }
        directed.graph_attrs_mut().extend(self.storage.graph_attrs().clone());
        directed
    }

    /// 🧹 Removes every node, edge, and handle; also drops this facade's `default_handle` bookkeeping.
    pub fn clear(&mut self) {
        self.storage.clear();
        self.default_handle.clear();
    }

    /// 🧹 Removes every edge but keeps nodes AND their handles — `default_handle` is intentionally left untouched, since handles are anchored on nodes, not edges, and only get dropped when their owning node is removed.
    pub fn clear_edges(&mut self) {
        self.storage.clear_edges();
    }
}
// #endregion 🔖Transforms

// #region 🔖Attributes
impl PortUndirectedGraph {
    pub fn set_node_attributes(&mut self, node: NodeId, attrs: PropertyBag) {
        if let Some(bag) = self.storage.node_attrs_mut(node) {
            bag.extend(attrs);
        }
    }

    pub fn get_node_attributes(&self, node: NodeId) -> Option<&PropertyBag> {
        self.storage.node_attrs(node)
    }

    /// 🏷️ Keyed by `EdgeId` (not `(u, v)`), since several parallel edges can share a node pair.
    pub fn set_edge_attributes(&mut self, edge: EdgeId, attrs: PropertyBag) {
        if let Some(bag) = self.storage.edge_attrs_mut(edge) {
            bag.extend(attrs);
        }
    }

    pub fn get_edge_attributes(&self, edge: EdgeId) -> Option<&PropertyBag> {
        self.storage.edge_attrs(edge)
    }

    pub fn name(&self) -> Option<&str> {
        self.storage.graph_attrs().get("name").and_then(PropertyValue::as_str)
    }

    pub fn set_name(&mut self, name: impl Into<String>) {
        self.storage.graph_attrs_mut().insert("name".to_string(), PropertyValue::String(name.into()));
    }
}
// #endregion 🔖Attributes

// #region 🔖SelfLoops
impl PortUndirectedGraph {
    pub fn selfloop_edges(&self) -> impl Iterator<Item = EdgeId> + '_ {
        self.storage.edges().filter(|edge| edge.u == edge.v).map(|edge| edge.id)
    }

    pub fn number_of_selfloops(&self) -> usize {
        self.selfloop_edges().count()
    }

    pub fn nodes_with_selfloops(&self) -> impl Iterator<Item = NodeId> + '_ {
        let nodes: BTreeSet<NodeId> = self.storage.edges().filter(|edge| edge.u == edge.v).map(|edge| edge.u).collect();
        nodes.into_iter()
    }
}
// #endregion 🔖SelfLoops

// #region 🔖Views
/// 🪟 Structural view delegation — every method borrows `self.storage`'s own `GraphView` impl, which already operates at node level regardless of the `Ported` model underneath.
impl GraphView for PortUndirectedGraph {
    fn node_count(&self) -> usize {
        self.storage.node_count()
    }
    fn nodes(&self) -> impl Iterator<Item = NodeId> {
        self.storage.nodes()
    }
    fn contains_node(&self, node: NodeId) -> bool {
        self.storage.contains_node(node)
    }
    fn edge_count(&self) -> usize {
        self.storage.edge_count()
    }
    fn edges(&self) -> impl Iterator<Item = EdgeRef> {
        self.storage.edges()
    }
    fn neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> {
        self.storage.neighbors(node)
    }
    fn out_neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> {
        self.storage.out_neighbors(node)
    }
    fn in_neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> {
        self.storage.in_neighbors(node)
    }
    fn degree(&self, node: NodeId) -> usize {
        self.storage.degree(node)
    }
    fn out_degree(&self, node: NodeId) -> usize {
        self.storage.out_degree(node)
    }
    fn in_degree(&self, node: NodeId) -> usize {
        self.storage.in_degree(node)
    }
    fn is_directed(&self) -> bool {
        self.storage.is_directed()
    }
    fn is_multigraph(&self) -> bool {
        self.storage.is_multigraph()
    }
    fn edges_between(&self, u: NodeId, v: NodeId) -> impl Iterator<Item = EdgeRef> {
        self.storage.edges_between(u, v)
    }
}

impl AttrView for PortUndirectedGraph {
    fn node_attrs(&self, node: NodeId) -> Option<&PropertyBag> {
        self.storage.node_attrs(node)
    }
    fn edge_attrs(&self, edge: EdgeId) -> Option<&PropertyBag> {
        self.storage.edge_attrs(edge)
    }
    fn graph_attrs(&self) -> &PropertyBag {
        self.storage.graph_attrs()
    }
}

impl EdgeWeights for PortUndirectedGraph {
    fn weight(&self, edge: EdgeRef) -> f64 {
        self.storage.weight(edge)
    }
}
// #endregion 🔖Views

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parallel_edges_get_distinct_ids() {
        let mut g = PortUndirectedGraph::new();
        let a = g.add_node();
        let b = g.add_node();
        let e1 = g.add_edge(a, b);
        let e2 = g.add_edge(a, b);
        assert_ne!(e1, e2);
        assert_eq!(g.edges_between(a, b).count(), 2);
    }

    #[test]
    fn neighbors_dedupe_across_parallel_edges() {
        let mut g = PortUndirectedGraph::new();
        let a = g.add_node();
        let b = g.add_node();
        g.add_edge(a, b);
        g.add_edge(a, b);
        g.add_edge(a, b);
        let neighbors: Vec<NodeId> = g.neighbors(a).collect();
        assert_eq!(neighbors, vec![b]);
    }

    #[test]
    fn degree_counts_every_parallel_edge() {
        let mut g = PortUndirectedGraph::new();
        let a = g.add_node();
        let b = g.add_node();
        g.add_edge(a, b);
        g.add_edge(a, b);
        assert_eq!(g.degree(a), 2);
        assert_eq!(g.degree(b), 2);
    }

    #[test]
    fn self_loop_counts_twice_towards_degree() {
        let mut g = PortUndirectedGraph::new();
        let a = g.add_node();
        g.add_edge(a, a);
        assert_eq!(g.degree(a), 2);
    }

    #[test]
    fn remove_one_edge_drops_exactly_one_parallel_edge() {
        let mut g = PortUndirectedGraph::new();
        let a = g.add_node();
        let b = g.add_node();
        let e1 = g.add_edge(a, b);
        let e2 = g.add_edge(a, b);
        assert!(g.remove_one_edge(a, b));
        let remaining: Vec<EdgeId> = g.edges_between(a, b).collect();
        assert_eq!(remaining.len(), 1);
        assert_eq!(remaining[0], e1.min(e2));
    }

    #[test]
    fn to_simple_sums_parallel_edge_weights() {
        let mut g = PortUndirectedGraph::new();
        let a = g.add_node();
        let b = g.add_node();
        for weight in [1.0, 2.0, 3.0] {
            let mut attrs = PropertyBag::default();
            attrs.insert("weight".to_string(), PropertyValue::Number(weight));
            g.add_edge_with(a, b, attrs);
        }
        let simple = g.to_simple();
        assert_eq!(simple.edge_count(), 1);
        let edge = simple.edges().next().expect("one collapsed edge");
        let weight = simple.edge_attrs(edge.id).and_then(|attrs| attrs.get("weight")).and_then(PropertyValue::as_f64);
        assert_eq!(weight, Some(6.0));
    }

    #[test]
    fn edges_between_returns_all_parallel_ids() {
        let mut g = PortUndirectedGraph::new();
        let a = g.add_node();
        let b = g.add_node();
        let e1 = g.add_edge(a, b);
        let e2 = g.add_edge(a, b);
        let e3 = g.add_edge(a, b);
        let mut ids: Vec<EdgeId> = g.edges_between(a, b).collect();
        ids.sort_unstable();
        let mut expected = vec![e1, e2, e3];
        expected.sort_unstable();
        assert_eq!(ids, expected);
    }

    #[test]
    fn add_edge_auto_creates_unseen_nodes() {
        let mut g = PortUndirectedGraph::new();
        assert!(!g.has_node(42));
        assert!(!g.has_node(7));
        g.add_edge(42, 7);
        assert!(g.has_node(42));
        assert!(g.has_node(7));
        assert!(g.has_edge(42, 7));
    }

    #[test]
    fn subgraph_and_edge_subgraph_are_independent_copies() {
        let mut g = PortUndirectedGraph::new();
        let a = g.add_node();
        let b = g.add_node();
        let c = g.add_node();
        let e_ab = g.add_edge(a, b);
        g.add_edge(b, c);

        let sub = g.subgraph([a, b]);
        assert_eq!(sub.number_of_nodes(), 2);
        assert_eq!(sub.number_of_edges(None, None), 1);

        let esub = g.edge_subgraph([e_ab]);
        assert_eq!(esub.number_of_nodes(), 2);
        assert_eq!(esub.number_of_edges(None, None), 1);

        g.add_edge(a, c);
        assert_eq!(sub.number_of_edges(None, None), 1);
        assert_eq!(esub.number_of_edges(None, None), 1);
    }
}
// #endregion 🔖Tests
