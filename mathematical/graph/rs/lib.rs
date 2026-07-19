//! 🕸️ Pure graph foundation: topology markers, node/handle/edge kinds, and index-based algorithms; the interactive board engine lives in `infinite_board`.

use std::collections::{BTreeMap, BTreeSet};

pub use mathematical_graph_manifest::{PropertyBag, PropertyValue};

// #region 🔖Ids
/// 🧩 Stable node identifier.
pub type NodeId = u64;
/// 🪝 Stable handle identifier.
pub type HandleId = u64;
/// 🪢 Stable edge identifier.
pub type EdgeId = u64;
// #endregion 🔖Ids

// #region 🔖Edge
/// 🪢 Edge with typed endpoints (node id or handle id).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CoreEdge<E> {
    pub id: EdgeId,
    pub source: E,
    pub target: E,
}

impl<E: Copy + Ord> CoreEdge<E> {
    /// 📐 Normalize endpoints for undirected storage.
    pub fn normalize_undirected(source: E, target: E) -> (E, E) {
        if source <= target {
            (source, target)
        } else {
            (target, source)
        }
    }
}
// #endregion 🔖Edge

// #region 🔖Directedness
/// ↔️ Compile-time directed vs undirected graph axis.
pub trait Directedness {
    const DIRECTED: bool;
}

/// ➡️ Directed edges keep source→target order.
#[derive(Clone, Copy, Debug, Default)]
pub struct Directed;

impl Directedness for Directed {
    const DIRECTED: bool = true;
}

/// ↔️ Undirected edges store ordered endpoint pair.
#[derive(Clone, Copy, Debug, Default)]
pub struct Undirected;

impl Directedness for Undirected {
    const DIRECTED: bool = false;
}

/// 📐 Apply directedness when storing edge endpoints.
#[inline]
pub fn orient_endpoints<E: Copy + Ord, D: Directedness>(source: E, target: E) -> (E, E) {
    if D::DIRECTED {
        (source, target)
    } else {
        CoreEdge::<E>::normalize_undirected(source, target)
    }
}
// #endregion 🔖Directedness

// #region 🔖PortModel
/// 🔌 Compile-time normal (node) vs ported (handle) graph axis.
pub trait PortModel {
    type Endpoint: Copy + Ord + std::fmt::Debug;
    const HAS_PORTS: bool;
    /// 🪢 Whether this port model allows parallel edges between the same pair (the port axis IS the multi-edge axis: `Ported` ~ NetworkX `Multi(Di)Graph`, `Normal` ~ NetworkX `(Di)Graph`).
    const MULTI_EDGES: bool;
    fn endpoint_as_u64(endpoint: Self::Endpoint) -> u64;
    fn try_handle_endpoint(handle_id: HandleId) -> Option<Self::Endpoint>;
    fn endpoint_as_handle(endpoint: Self::Endpoint) -> Option<HandleId>;
}

/// 🟠 Node-to-node edges without handles.
#[derive(Clone, Copy, Debug, Default)]
pub struct Normal;

impl PortModel for Normal {
    type Endpoint = NodeId;
    const HAS_PORTS: bool = false;
    const MULTI_EDGES: bool = false;
    fn endpoint_as_u64(endpoint: Self::Endpoint) -> u64 {
        endpoint
    }
    fn try_handle_endpoint(_: HandleId) -> Option<Self::Endpoint> {
        None
    }
    fn endpoint_as_handle(_: Self::Endpoint) -> Option<HandleId> {
        None
    }
}

/// 🪝 Handle-to-handle edges on nodes.
#[derive(Clone, Copy, Debug, Default)]
pub struct Ported;

impl PortModel for Ported {
    type Endpoint = HandleId;
    const HAS_PORTS: bool = true;
    const MULTI_EDGES: bool = true;
    fn endpoint_as_u64(endpoint: Self::Endpoint) -> u64 {
        endpoint
    }
    fn try_handle_endpoint(handle_id: HandleId) -> Option<Self::Endpoint> {
        Some(handle_id)
    }
    fn endpoint_as_handle(endpoint: Self::Endpoint) -> Option<HandleId> {
        Some(endpoint)
    }
}
// #endregion 🔖PortModel

// #region 🔖Algorithms
pub mod algorithms {
    //! 🧮 Index-based graph algorithms: traversal, ordering, cycles, components, shortest paths.

    use std::collections::HashMap;

    // #region 🔖Adjacency
    /// 🧮 Compact adjacency built once per query batch.
    #[derive(Clone, Debug)]
    pub struct Adjacency {
        n: usize,
        out: Vec<Vec<usize>>,
        inc: Vec<Vec<usize>>,
    }

    impl Adjacency {
        pub fn node_count(&self) -> usize {
            self.n
        }
        pub fn out_neighbors(&self, i: usize) -> &[usize] {
            &self.out[i]
        }
        pub fn in_neighbors(&self, i: usize) -> &[usize] {
            &self.inc[i]
        }
    }

    /// 🧮 Builds adjacency lists from index edges; `directed` controls whether reverse edges are also recorded as out-edges.
    pub fn adjacency(node_count: usize, edges: &[(usize, usize)], directed: bool) -> Adjacency {
        let mut out = vec![Vec::new(); node_count];
        let mut inc = vec![Vec::new(); node_count];
        for &(a, b) in edges {
            if a >= node_count || b >= node_count {
                continue;
            }
            out[a].push(b);
            inc[b].push(a);
            if !directed {
                out[b].push(a);
                inc[a].push(b);
            }
        }
        Adjacency { n: node_count, out, inc }
    }
    // #endregion 🔖Adjacency

    // #region 🔖IdIndex
    /// 🔤 Deterministic string-id <-> index bridge (ids sorted for reproducible ordering).
    #[derive(Clone, Debug, Default)]
    pub struct IdIndex {
        ids: Vec<String>,
        index: HashMap<String, usize>,
    }

    impl IdIndex {
        pub fn from_ids<'a>(ids: impl Iterator<Item = &'a str>) -> Self {
            let mut sorted: Vec<String> = ids.map(|s| s.to_string()).collect();
            sorted.sort();
            sorted.dedup();
            let index = sorted.iter().enumerate().map(|(i, s)| (s.clone(), i)).collect();
            Self { ids: sorted, index }
        }

        pub fn from_edges<'a>(edges: impl Iterator<Item = (&'a str, &'a str)>) -> Self {
            let mut all: Vec<String> = Vec::new();
            for (a, b) in edges {
                all.push(a.to_string());
                all.push(b.to_string());
            }
            Self::from_ids(all.iter().map(|s| s.as_str()))
        }

        pub fn index_of(&self, id: &str) -> Option<usize> {
            self.index.get(id).copied()
        }

        pub fn id_of(&self, index: usize) -> Option<&str> {
            self.ids.get(index).map(|s| s.as_str())
        }

        pub fn len(&self) -> usize {
            self.ids.len()
        }

        pub fn is_empty(&self) -> bool {
            self.ids.is_empty()
        }

        pub fn edges_to_indices(&self, edges: &[(String, String)]) -> Vec<(usize, usize)> {
            edges.iter().filter_map(|(a, b)| Some((self.index_of(a)?, self.index_of(b)?))).collect()
        }
    }
    // #endregion 🔖IdIndex

    // #region 🔖Traversal
    /// 🌊 Breadth-first visitation order from the given seeds.
    pub fn bfs_order(adj: &Adjacency, seeds: &[usize]) -> Vec<usize> {
        let mut visited = vec![false; adj.n];
        let mut order = Vec::new();
        let mut queue = std::collections::VecDeque::new();
        for &s in seeds {
            if s < adj.n && !visited[s] {
                visited[s] = true;
                queue.push_back(s);
            }
        }
        while let Some(u) = queue.pop_front() {
            order.push(u);
            for &v in &adj.out[u] {
                if !visited[v] {
                    visited[v] = true;
                    queue.push_back(v);
                }
            }
        }
        order
    }

    /// 🌊 Breadth-first layers (distance bands) from the given seeds.
    pub fn bfs_layers(adj: &Adjacency, seeds: &[usize]) -> Vec<Vec<usize>> {
        let mut visited = vec![false; adj.n];
        let mut layers = Vec::new();
        let mut frontier: Vec<usize> = seeds.iter().copied().filter(|&s| s < adj.n).collect();
        for &s in &frontier {
            visited[s] = true;
        }
        while !frontier.is_empty() {
            layers.push(frontier.clone());
            let mut next = Vec::new();
            for &u in &frontier {
                for &v in &adj.out[u] {
                    if !visited[v] {
                        visited[v] = true;
                        next.push(v);
                    }
                }
            }
            frontier = next;
        }
        layers
    }

    /// 📏 Unweighted BFS distance from a single seed to every reachable node.
    pub fn bfs_distances(adj: &Adjacency, seed: usize) -> Vec<Option<u32>> {
        let mut dist = vec![None; adj.n];
        if seed >= adj.n {
            return dist;
        }
        dist[seed] = Some(0);
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(seed);
        while let Some(u) = queue.pop_front() {
            let du = dist[u].expect("every queued node was assigned a distance before being pushed");
            for &v in &adj.out[u] {
                if dist[v].is_none() {
                    dist[v] = Some(du + 1);
                    queue.push_back(v);
                }
            }
        }
        dist
    }

    /// 🌲 Depth-first preorder from a single seed.
    pub fn dfs_preorder(adj: &Adjacency, seed: usize) -> Vec<usize> {
        let mut visited = vec![false; adj.n];
        let mut order = Vec::new();
        if seed >= adj.n {
            return order;
        }
        let mut stack = vec![seed];
        while let Some(u) = stack.pop() {
            if visited[u] {
                continue;
            }
            visited[u] = true;
            order.push(u);
            for &v in adj.out[u].iter().rev() {
                if !visited[v] {
                    stack.push(v);
                }
            }
        }
        order
    }

    /// 🌲 Depth-first postorder from a single seed.
    pub fn dfs_postorder(adj: &Adjacency, seed: usize) -> Vec<usize> {
        let mut visited = vec![false; adj.n];
        let mut order = Vec::new();
        if seed >= adj.n {
            return order;
        }
        fn visit(u: usize, adj: &Adjacency, visited: &mut [bool], order: &mut Vec<usize>) {
            visited[u] = true;
            for &v in &adj.out[u] {
                if !visited[v] {
                    visit(v, adj, visited, order);
                }
            }
            order.push(u);
        }
        visit(seed, adj, &mut visited, &mut order);
        order
    }
    // #endregion 🔖Traversal

    // #region 🔖Ordering
    /// ⚠️ A cycle was found where a DAG was required; `cycle` lists the node indices on the cycle.
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct CycleError {
        pub cycle: Vec<usize>,
    }

    /// 🔢 Kahn's algorithm topological sort; index-ascending tie-break for determinism.
    pub fn topo_sort(adj: &Adjacency) -> Result<Vec<usize>, CycleError> {
        let mut in_deg = vec![0usize; adj.n];
        for list in &adj.out {
            for &v in list {
                in_deg[v] += 1;
            }
        }
        let mut heap = std::collections::BinaryHeap::new();
        for (i, &deg) in in_deg.iter().enumerate() {
            if deg == 0 {
                heap.push(std::cmp::Reverse(i));
            }
        }
        let mut order = Vec::with_capacity(adj.n);
        while let Some(std::cmp::Reverse(u)) = heap.pop() {
            order.push(u);
            for &v in &adj.out[u] {
                in_deg[v] -= 1;
                if in_deg[v] == 0 {
                    heap.push(std::cmp::Reverse(v));
                }
            }
        }
        if order.len() == adj.n {
            Ok(order)
        } else {
            let remaining: Vec<usize> = (0..adj.n).filter(|&i| in_deg[i] > 0).collect();
            Err(CycleError { cycle: find_cycle_among(adj, &remaining).unwrap_or(remaining) })
        }
    }

    /// 🪜 Topological levels: each level contains nodes whose dependencies are all in earlier levels.
    pub fn topo_levels(adj: &Adjacency) -> Result<Vec<Vec<usize>>, CycleError> {
        let mut in_deg = vec![0usize; adj.n];
        for list in &adj.out {
            for &v in list {
                in_deg[v] += 1;
            }
        }
        let mut levels = Vec::new();
        let mut remaining = in_deg.clone();
        let mut placed = vec![false; adj.n];
        let mut placed_count = 0;
        loop {
            let mut frontier: Vec<usize> = (0..adj.n).filter(|&i| !placed[i] && remaining[i] == 0).collect();
            if frontier.is_empty() {
                break;
            }
            frontier.sort_unstable();
            for &u in &frontier {
                placed[u] = true;
                placed_count += 1;
            }
            for &u in &frontier {
                for &v in &adj.out[u] {
                    remaining[v] -= 1;
                }
            }
            levels.push(frontier);
        }
        if placed_count == adj.n {
            Ok(levels)
        } else {
            let unplaced: Vec<usize> = (0..adj.n).filter(|&i| !placed[i]).collect();
            Err(CycleError { cycle: find_cycle_among(adj, &unplaced).unwrap_or(unplaced) })
        }
    }

    /// 🪜 Longest-path layer index per node (DAG layering for hierarchical drawing); layer 0 = roots.
    pub fn longest_path_layers(adj: &Adjacency) -> Result<Vec<u32>, CycleError> {
        let levels = topo_levels(adj)?;
        let mut layer = vec![0u32; adj.n];
        for (li, level) in levels.iter().enumerate() {
            for &u in level {
                layer[u] = li as u32;
            }
        }
        Ok(layer)
    }
    // #endregion 🔖Ordering

    // #region 🔖Cycles
    /// 🔎 Whether `to` is reachable from `from` following out-edges.
    pub fn is_reachable(adj: &Adjacency, from: usize, to: usize) -> bool {
        if from == to {
            return true;
        }
        bfs_order(adj, &[from]).contains(&to)
    }

    /// ➕ Whether adding an edge `source -> target` would create a cycle (i.e. `target` can already reach `source`).
    pub fn would_create_cycle(adj: &Adjacency, source: usize, target: usize) -> bool {
        source == target || is_reachable(adj, target, source)
    }

    /// ➕ String-id convenience: whether adding `source -> target` to `existing` directed edges would create a cycle.
    pub fn would_create_cycle_ids(existing: &[(String, String)], source: &str, target: &str) -> bool {
        if source == target {
            return true;
        }
        let index = IdIndex::from_edges(existing.iter().map(|(a, b)| (a.as_str(), b.as_str())));
        let (Some(s), Some(t)) = (index.index_of(source), index.index_of(target)) else {
            return false;
        };
        let adj = adjacency(index.len(), &index.edges_to_indices(existing), true);
        would_create_cycle(&adj, s, t)
    }

    /// ➕ Batched acyclic filter: for each `candidates[i]`, whether adding it to `existing` (+ prior accepted candidates) keeps the graph acyclic.
    pub fn acyclic_edge_subset(existing: &[(String, String)], candidates: &[(String, String)]) -> Vec<bool> {
        let all_ids = existing.iter().chain(candidates.iter()).flat_map(|(a, b)| [a.as_str(), b.as_str()]);
        let index = IdIndex::from_ids(all_ids);
        let mut edges = index.edges_to_indices(existing);
        let mut accepted = Vec::with_capacity(candidates.len());
        for (a, b) in candidates {
            let (Some(s), Some(t)) = (index.index_of(a), index.index_of(b)) else {
                accepted.push(false);
                continue;
            };
            let adj = adjacency(index.len(), &edges, true);
            if would_create_cycle(&adj, s, t) {
                accepted.push(false);
            } else {
                edges.push((s, t));
                accepted.push(true);
            }
        }
        accepted
    }

    fn find_cycle_among(adj: &Adjacency, candidates: &[usize]) -> Option<Vec<usize>> {
        let mut color = vec![0u8; adj.n];
        let mut path = Vec::new();
        fn dfs(u: usize, adj: &Adjacency, color: &mut [u8], path: &mut Vec<usize>) -> Option<Vec<usize>> {
            color[u] = 1;
            path.push(u);
            for &v in &adj.out[u] {
                if color[v] == 1 {
                    let start = path.iter().position(|&x| x == v).expect("color[v] == 1 means v is currently on the open dfs path");
                    return Some(path[start..].to_vec());
                }
                if color[v] == 0 {
                    if let Some(cycle) = dfs(v, adj, color, path) {
                        return Some(cycle);
                    }
                }
            }
            path.pop();
            color[u] = 2;
            None
        }
        for &start in candidates {
            if color[start] == 0 {
                if let Some(cycle) = dfs(start, adj, &mut color, &mut path) {
                    return Some(cycle);
                }
            }
        }
        None
    }

    /// 🔎 Finds one cycle in the graph, if any exist.
    pub fn find_cycle(adj: &Adjacency) -> Option<Vec<usize>> {
        let all: Vec<usize> = (0..adj.n).collect();
        find_cycle_among(adj, &all)
    }
    // #endregion 🔖Cycles

    // #region 🔖Components
    /// 🧮 Union-find (disjoint-set) with path compression and union-by-rank.
    #[derive(Clone, Debug)]
    pub struct UnionFind {
        parent: Vec<usize>,
        rank: Vec<u8>,
    }

    impl UnionFind {
        pub fn new(n: usize) -> Self {
            Self { parent: (0..n).collect(), rank: vec![0; n] }
        }

        pub fn find(&mut self, x: usize) -> usize {
            if self.parent[x] != x {
                self.parent[x] = self.find(self.parent[x]);
            }
            self.parent[x]
        }

        pub fn union(&mut self, a: usize, b: usize) {
            let (ra, rb) = (self.find(a), self.find(b));
            if ra == rb {
                return;
            }
            match self.rank[ra].cmp(&self.rank[rb]) {
                std::cmp::Ordering::Less => self.parent[ra] = rb,
                std::cmp::Ordering::Greater => self.parent[rb] = ra,
                std::cmp::Ordering::Equal => {
                    self.parent[rb] = ra;
                    self.rank[ra] += 1;
                }
            }
        }

        pub fn same_set(&mut self, a: usize, b: usize) -> bool {
            self.find(a) == self.find(b)
        }
    }

    /// 🧩 Weak connected-component id per node (undirected reachability, works for directed adjacency too).
    pub fn connected_components(adj: &Adjacency) -> Vec<usize> {
        let mut uf = UnionFind::new(adj.n);
        for u in 0..adj.n {
            for &v in &adj.out[u] {
                uf.union(u, v);
            }
        }
        let mut root_to_component: HashMap<usize, usize> = HashMap::new();
        let mut labels = vec![0usize; adj.n];
        for (u, label) in labels.iter_mut().enumerate() {
            let root = uf.find(u);
            let next_id = root_to_component.len();
            let id = *root_to_component.entry(root).or_insert(next_id);
            *label = id;
        }
        labels
    }

    /// 🧩 Tarjan's strongly connected components; returned in reverse-topological order, nodes sorted within each.
    pub fn strongly_connected_components(adj: &Adjacency) -> Vec<Vec<usize>> {
        struct State {
            index: Vec<Option<u32>>,
            lowlink: Vec<u32>,
            on_stack: Vec<bool>,
            stack: Vec<usize>,
            counter: u32,
            out: Vec<Vec<usize>>,
        }
        fn strongconnect(u: usize, adj: &Adjacency, st: &mut State) {
            st.index[u] = Some(st.counter);
            st.lowlink[u] = st.counter;
            st.counter += 1;
            st.stack.push(u);
            st.on_stack[u] = true;
            for &v in &adj.out[u] {
                if st.index[v].is_none() {
                    strongconnect(v, adj, st);
                    st.lowlink[u] = st.lowlink[u].min(st.lowlink[v]);
                } else if st.on_stack[v] {
                    st.lowlink[u] = st.lowlink[u].min(st.index[v].expect("on_stack[v] implies index[v] was assigned when v was first visited"));
                }
            }
            if st.lowlink[u] == st.index[u].expect("index[u] was assigned at the start of this strongconnect call") {
                let mut component = Vec::new();
                loop {
                    let w = st.stack.pop().expect("u is still on the tarjan stack until its own component is popped");
                    st.on_stack[w] = false;
                    component.push(w);
                    if w == u {
                        break;
                    }
                }
                component.sort_unstable();
                st.out.push(component);
            }
        }
        let mut st = State { index: vec![None; adj.n], lowlink: vec![0; adj.n], on_stack: vec![false; adj.n], stack: Vec::new(), counter: 0, out: Vec::new() };
        for u in 0..adj.n {
            if st.index[u].is_none() {
                strongconnect(u, adj, &mut st);
            }
        }
        st.out
    }

    /// ⬇️ In-degree per node.
    pub fn in_degrees(adj: &Adjacency) -> Vec<usize> {
        (0..adj.n).map(|i| adj.inc[i].len()).collect()
    }

    /// ⬆️ Out-degree per node.
    pub fn out_degrees(adj: &Adjacency) -> Vec<usize> {
        (0..adj.n).map(|i| adj.out[i].len()).collect()
    }

    /// 🌱 Indices of nodes with in-degree 0 (DAG roots).
    pub fn root_indices(adj: &Adjacency) -> Vec<usize> {
        (0..adj.n).filter(|&i| adj.inc[i].is_empty()).collect()
    }
    // #endregion 🔖Components

    // #region 🔖Paths
    /// 📏 Shortest path (by hop count) between two nodes, if reachable.
    pub fn shortest_path_unweighted(adj: &Adjacency, from: usize, to: usize) -> Option<Vec<usize>> {
        if from >= adj.n || to >= adj.n {
            return None;
        }
        let mut visited = vec![false; adj.n];
        let mut parent = vec![usize::MAX; adj.n];
        visited[from] = true;
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(from);
        while let Some(u) = queue.pop_front() {
            if u == to {
                let mut path = vec![to];
                let mut cur = to;
                while cur != from {
                    cur = parent[cur];
                    path.push(cur);
                }
                path.reverse();
                return Some(path);
            }
            for &v in &adj.out[u] {
                if !visited[v] {
                    visited[v] = true;
                    parent[v] = u;
                    queue.push_back(v);
                }
            }
        }
        None
    }

    /// 📏 Dijkstra shortest distances from `from` to every node, given non-negative edge weights parallel to adjacency out-edges.
    pub fn dijkstra(adj: &Adjacency, weights: &HashMap<(usize, usize), f64>, from: usize) -> Vec<Option<f64>> {
        let mut dist = vec![None; adj.n];
        if from >= adj.n {
            return dist;
        }
        dist[from] = Some(0.0);
        let mut heap = std::collections::BinaryHeap::new();
        heap.push(std::cmp::Reverse(OrderedFloat(0.0, from)));
        while let Some(std::cmp::Reverse(OrderedFloat(d, u))) = heap.pop() {
            if dist[u].map(|cur| d > cur).unwrap_or(true) {
                continue;
            }
            for &v in &adj.out[u] {
                let w = weights.get(&(u, v)).copied().unwrap_or(1.0);
                let nd = d + w;
                if dist[v].map(|cur| nd < cur).unwrap_or(true) {
                    dist[v] = Some(nd);
                    heap.push(std::cmp::Reverse(OrderedFloat(nd, v)));
                }
            }
        }
        dist
    }

    /// 📏 Dijkstra shortest path and distance between two nodes, if reachable.
    pub fn dijkstra_path(adj: &Adjacency, weights: &HashMap<(usize, usize), f64>, from: usize, to: usize) -> Option<(Vec<usize>, f64)> {
        if from >= adj.n || to >= adj.n {
            return None;
        }
        let mut dist = vec![None; adj.n];
        let mut parent = vec![usize::MAX; adj.n];
        dist[from] = Some(0.0);
        let mut heap = std::collections::BinaryHeap::new();
        heap.push(std::cmp::Reverse(OrderedFloat(0.0, from)));
        while let Some(std::cmp::Reverse(OrderedFloat(d, u))) = heap.pop() {
            if dist[u].map(|cur| d > cur).unwrap_or(true) {
                continue;
            }
            if u == to {
                let mut path = vec![to];
                let mut cur = to;
                while cur != from {
                    cur = parent[cur];
                    path.push(cur);
                }
                path.reverse();
                return Some((path, d));
            }
            for &v in &adj.out[u] {
                let w = weights.get(&(u, v)).copied().unwrap_or(1.0);
                let nd = d + w;
                if dist[v].map(|cur| nd < cur).unwrap_or(true) {
                    dist[v] = Some(nd);
                    parent[v] = u;
                    heap.push(std::cmp::Reverse(OrderedFloat(nd, v)));
                }
            }
        }
        None
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    struct OrderedFloat(f64, usize);
    impl Eq for OrderedFloat {}
    impl PartialOrd for OrderedFloat {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }
    impl Ord for OrderedFloat {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.0.partial_cmp(&other.0).unwrap_or(std::cmp::Ordering::Equal).then(self.1.cmp(&other.1))
        }
    }

    /// 🌲 Kruskal minimum spanning tree; returns the indices (into `edges`) of the selected edges.
    pub fn minimum_spanning_tree(node_count: usize, edges: &[(usize, usize, f64)]) -> Vec<usize> {
        let mut order: Vec<usize> = (0..edges.len()).collect();
        order.sort_by(|&a, &b| edges[a].2.partial_cmp(&edges[b].2).unwrap_or(std::cmp::Ordering::Equal));
        let mut uf = UnionFind::new(node_count);
        let mut selected = Vec::new();
        for i in order {
            let (a, b, _) = edges[i];
            if a >= node_count || b >= node_count {
                continue;
            }
            if !uf.same_set(a, b) {
                uf.union(a, b);
                selected.push(i);
            }
        }
        selected
    }
    // #endregion 🔖Paths

    // #region 🔖Tests
    #[cfg(test)]
    mod tests {
        use super::*;

        fn adj_from(n: usize, edges: &[(usize, usize)], directed: bool) -> Adjacency {
            adjacency(n, edges, directed)
        }

        #[test]
        fn bfs_order_visits_reachable_nodes_breadth_first() {
            let adj = adj_from(5, &[(0, 1), (0, 2), (1, 3), (2, 4)], true);
            let order = bfs_order(&adj, &[0]);
            assert_eq!(order, vec![0, 1, 2, 3, 4]);
        }

        #[test]
        fn bfs_layers_group_by_distance() {
            let adj = adj_from(4, &[(0, 1), (0, 2), (1, 3)], true);
            let layers = bfs_layers(&adj, &[0]);
            assert_eq!(layers, vec![vec![0], vec![1, 2], vec![3]]);
        }

        #[test]
        fn bfs_distances_unreachable_is_none() {
            let adj = adj_from(3, &[(0, 1)], true);
            let dist = bfs_distances(&adj, 0);
            assert_eq!(dist, vec![Some(0), Some(1), None]);
        }

        #[test]
        fn dfs_preorder_and_postorder_agree_on_leaf_first_last() {
            let adj = adj_from(3, &[(0, 1), (1, 2)], true);
            assert_eq!(dfs_preorder(&adj, 0), vec![0, 1, 2]);
            assert_eq!(dfs_postorder(&adj, 0), vec![2, 1, 0]);
        }

        #[test]
        fn topo_sort_orders_dependencies_before_dependents() {
            let adj = adj_from(4, &[(0, 1), (0, 2), (1, 3), (2, 3)], true);
            let order = topo_sort(&adj).expect("acyclic");
            let pos = |n: usize| order.iter().position(|&x| x == n).unwrap();
            assert!(pos(0) < pos(1));
            assert!(pos(1) < pos(3));
            assert!(pos(2) < pos(3));
        }

        #[test]
        fn topo_sort_detects_cycle() {
            let adj = adj_from(3, &[(0, 1), (1, 2), (2, 0)], true);
            let err = topo_sort(&adj).unwrap_err();
            assert_eq!(err.cycle.len(), 3);
        }

        #[test]
        fn topo_levels_groups_independent_nodes() {
            let adj = adj_from(4, &[(0, 2), (1, 2), (2, 3)], true);
            let levels = topo_levels(&adj).expect("acyclic");
            assert_eq!(levels[0], vec![0, 1]);
            assert_eq!(levels[1], vec![2]);
            assert_eq!(levels[2], vec![3]);
        }

        #[test]
        fn longest_path_layers_assigns_root_layer_zero() {
            let adj = adj_from(3, &[(0, 1), (1, 2)], true);
            let layers = longest_path_layers(&adj).expect("acyclic");
            assert_eq!(layers, vec![0, 1, 2]);
        }

        #[test]
        fn would_create_cycle_detects_back_edge() {
            let adj = adj_from(3, &[(0, 1), (1, 2)], true);
            assert!(would_create_cycle(&adj, 2, 0));
            assert!(!would_create_cycle(&adj, 0, 2));
        }

        #[test]
        fn would_create_cycle_ids_matches_index_version() {
            let existing = vec![("a".to_string(), "b".to_string()), ("b".to_string(), "c".to_string())];
            assert!(would_create_cycle_ids(&existing, "c", "a"));
            assert!(!would_create_cycle_ids(&existing, "a", "c"));
        }

        #[test]
        fn acyclic_edge_subset_accumulates_accepted_candidates() {
            let existing = vec![("a".to_string(), "b".to_string())];
            let candidates = vec![("b".to_string(), "c".to_string()), ("c".to_string(), "a".to_string()), ("c".to_string(), "d".to_string())];
            let accepted = acyclic_edge_subset(&existing, &candidates);
            assert_eq!(accepted, vec![true, false, true]);
        }

        #[test]
        fn find_cycle_returns_none_for_dag() {
            let adj = adj_from(3, &[(0, 1), (1, 2)], true);
            assert!(find_cycle(&adj).is_none());
        }

        #[test]
        fn find_cycle_returns_some_for_cyclic_graph() {
            let adj = adj_from(3, &[(0, 1), (1, 2), (2, 0)], true);
            assert!(find_cycle(&adj).is_some());
        }

        #[test]
        fn connected_components_groups_weak_components() {
            let adj = adj_from(5, &[(0, 1), (1, 2), (3, 4)], true);
            let labels = connected_components(&adj);
            assert_eq!(labels[0], labels[1]);
            assert_eq!(labels[1], labels[2]);
            assert_eq!(labels[3], labels[4]);
            assert_ne!(labels[0], labels[3]);
        }

        #[test]
        fn strongly_connected_components_finds_cycle_as_one_component() {
            let adj = adj_from(4, &[(0, 1), (1, 2), (2, 0), (2, 3)], true);
            let sccs = strongly_connected_components(&adj);
            let cyclic = sccs.iter().find(|c| c.contains(&0)).unwrap();
            assert_eq!(cyclic, &vec![0, 1, 2]);
            assert!(sccs.iter().any(|c| c == &vec![3]));
        }

        #[test]
        fn degrees_and_roots_match_edge_shape() {
            let adj = adj_from(3, &[(0, 1), (0, 2)], true);
            assert_eq!(out_degrees(&adj), vec![2, 0, 0]);
            assert_eq!(in_degrees(&adj), vec![0, 1, 1]);
            assert_eq!(root_indices(&adj), vec![0]);
        }

        #[test]
        fn union_find_unions_and_queries_sets() {
            let mut uf = UnionFind::new(4);
            uf.union(0, 1);
            uf.union(2, 3);
            assert!(uf.same_set(0, 1));
            assert!(!uf.same_set(0, 2));
        }

        #[test]
        fn shortest_path_unweighted_finds_hop_path() {
            let adj = adj_from(4, &[(0, 1), (1, 3), (0, 2), (2, 3)], true);
            let path = shortest_path_unweighted(&adj, 0, 3).expect("reachable");
            assert_eq!(path.len(), 3);
            assert_eq!(path[0], 0);
            assert_eq!(*path.last().unwrap(), 3);
        }

        #[test]
        fn shortest_path_unweighted_none_when_unreachable() {
            let adj = adj_from(3, &[(0, 1)], true);
            assert!(shortest_path_unweighted(&adj, 0, 2).is_none());
        }

        #[test]
        fn dijkstra_prefers_cheaper_longer_path() {
            let adj = adj_from(3, &[(0, 1), (1, 2), (0, 2)], true);
            let mut weights = HashMap::new();
            weights.insert((0, 1), 1.0);
            weights.insert((1, 2), 1.0);
            weights.insert((0, 2), 5.0);
            let dist = dijkstra(&adj, &weights, 0);
            assert_eq!(dist[2], Some(2.0));
        }

        #[test]
        fn dijkstra_path_reconstructs_cheapest_route() {
            let adj = adj_from(3, &[(0, 1), (1, 2), (0, 2)], true);
            let mut weights = HashMap::new();
            weights.insert((0, 1), 1.0);
            weights.insert((1, 2), 1.0);
            weights.insert((0, 2), 5.0);
            let (path, dist) = dijkstra_path(&adj, &weights, 0, 2).expect("reachable");
            assert_eq!(path, vec![0, 1, 2]);
            assert_eq!(dist, 2.0);
        }

        #[test]
        fn minimum_spanning_tree_selects_cheapest_edges_without_cycles() {
            let edges = vec![(0, 1, 1.0), (1, 2, 2.0), (0, 2, 3.0)];
            let selected = minimum_spanning_tree(3, &edges);
            assert_eq!(selected.len(), 2);
            assert!(selected.contains(&0));
            assert!(selected.contains(&1));
        }

        #[test]
        fn id_index_is_deterministic_and_sorted() {
            let edges = [("c".to_string(), "a".to_string()), ("a".to_string(), "b".to_string())];
            let index = IdIndex::from_edges(edges.iter().map(|(a, b)| (a.as_str(), b.as_str())));
            assert_eq!(index.id_of(0), Some("a"));
            assert_eq!(index.id_of(1), Some("b"));
            assert_eq!(index.id_of(2), Some("c"));
        }
    }
    // #endregion 🔖Tests
}
// #endregion 🔖Algorithms

// #region 🔖PropertyJson
/// 🧾 Converts JSON fixture `userData` into a typed property bag.
pub fn property_bag_from_json(value: &serde_json::Value) -> PropertyBag {
    serde_json::from_value(value.clone()).unwrap_or_default()
}

/// 🧾 Serializes a property bag back to JSON for fixture export.
pub fn property_bag_to_json(bag: &PropertyBag) -> Option<serde_json::Value> {
    if bag.is_empty() {
        None
    } else {
        serde_json::to_value(bag).ok()
    }
}
// #endregion 🔖PropertyJson

// #region 🔖Kinds
use mathematical_geometry::Point;

/// 🔵 Circle or axis-aligned rectangle node body.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum NodeShape {
    #[default]
    Circle,
    Rectangle,
}

/// 🪝 Port direction for directed edge wiring.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum HandleRole {
    Source,
    Target,
    #[default]
    Any,
}

/// 🏷️ Semantic kind and property payload shared by graph elements.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ElementSemantics {
    pub kind: Option<String>,
    pub properties: PropertyBag,
}

/// 🟠 Retained node state with world-space center and shape extents.
#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    pub id: NodeId,
    pub center: Point,
    pub radius: f64,
    pub width: f64,
    pub height: f64,
    pub shape: NodeShape,
    pub draggable: bool,
    pub kind: Option<String>,
    pub label: Option<String>,
    pub properties: PropertyBag,
}

/// 🟣 Tangent handle anchored to a node at a polar angle.
#[derive(Clone, Debug, PartialEq)]
pub struct Handle {
    pub angle: f64,
    pub id: HandleId,
    pub node_id: NodeId,
    pub radius: f64,
    pub role: HandleRole,
    pub kind: Option<String>,
    pub properties: PropertyBag,
}

/// 🪢 Retained edge with typed endpoints.
pub type GraphEdge<E> = CoreEdge<E>;
// #endregion 🔖Kinds
