//! 🚰️ Resource-flow constraint: requires at least `min_flow` edge-disjoint paths (uniform
//! capacity 1 per adjacency edge — per-node/per-pattern-weighted capacities are deferred, see
//! this module's tests for the exact contract) from a source set to a sink set, moving only
//! through nodes whose assigned pattern matches a selector. Computed exactly at completion via a
//! hand-rolled Edmonds-Karp (BFS augmenting-path) max-flow over a virtual super-source/super-sink
//! network — no external graph/flow crate, matching this crate's zero-dependency convention.

use crate::wfc_engine::bitset::PatternSet;
use crate::wfc_engine::constraint::{AdjacencyView, Constraint, PatternSelector};
use crate::wfc_engine::domain::DomainStore;
use crate::wfc_engine::error::ConstraintError;
use crate::wfc_engine::ids::{NodeId, PatternId};
use crate::wfc_engine::model::CompiledModel;
use crate::wfc_engine::weights::WeightTable;
use std::collections::VecDeque;

// #region 🔖️Network
/// 🚰️ A minimal adjacency-list max-flow network: paired forward/reverse edges at consecutive
/// indices `(2k, 2k+1)`, so `e ^ 1` always gives an edge's reverse without extra bookkeeping.
struct FlowNetwork {
    adj: Vec<Vec<usize>>,
    to: Vec<usize>,
    cap: Vec<u32>,
}

impl FlowNetwork {
    fn new(n: usize) -> Self {
        Self { adj: vec![Vec::new(); n], to: Vec::new(), cap: Vec::new() }
    }

    fn add_edge(&mut self, u: usize, v: usize, capacity: u32) {
        self.adj[u].push(self.to.len());
        self.to.push(v);
        self.cap.push(capacity);
        self.adj[v].push(self.to.len());
        self.to.push(u);
        self.cap.push(0);
    }

    /// 🚰️ One BFS augmenting-path step: finds the shortest (fewest-edges) path from `s` to `t`
    /// with remaining capacity, pushes the bottleneck amount along it, and returns that amount
    /// (`0` once no augmenting path remains).
    fn bfs_augment(&mut self, s: usize, t: usize) -> u32 {
        let mut parent_edge: Vec<Option<usize>> = vec![None; self.adj.len()];
        let mut visited = vec![false; self.adj.len()];
        visited[s] = true;
        let mut queue = VecDeque::new();
        queue.push_back(s);
        'bfs: while let Some(u) = queue.pop_front() {
            for &e in &self.adj[u] {
                let v = self.to[e];
                if !visited[v] && self.cap[e] > 0 {
                    visited[v] = true;
                    parent_edge[v] = Some(e);
                    if v == t {
                        break 'bfs;
                    }
                    queue.push_back(v);
                }
            }
        }
        if !visited[t] {
            return 0;
        }
        let mut bottleneck = u32::MAX;
        let mut v = t;
        while v != s {
            let e = parent_edge[v].expect("visited[t] guarantees a parent chain back to s");
            bottleneck = bottleneck.min(self.cap[e]);
            v = self.to[e ^ 1];
        }
        v = t;
        while v != s {
            let e = parent_edge[v].expect("same parent chain as above");
            self.cap[e] -= bottleneck;
            self.cap[e ^ 1] += bottleneck;
            v = self.to[e ^ 1];
        }
        bottleneck
    }

    fn max_flow(&mut self, s: usize, t: usize) -> u32 {
        let mut total = 0u32;
        loop {
            let f = self.bfs_augment(s, t);
            if f == 0 {
                return total;
            }
            total += f;
        }
    }
}
// #endregion 🔖️Network

// #region 🔖️Constraint
/// 🚰️ Requires at least `min_flow` edge-disjoint paths from `sources` to `sinks`, through nodes
/// whose assigned pattern matches `selector` (source/sink endpoints must match too).
#[derive(Clone, Debug)]
pub struct FlowConstraint {
    pub selector: PatternSelector,
    pub sources: Vec<NodeId>,
    pub sinks: Vec<NodeId>,
    pub min_flow: u32,
    model: CompiledModel,
}

impl FlowConstraint {
    pub fn new(model: CompiledModel, selector: PatternSelector, sources: Vec<NodeId>, sinks: Vec<NodeId>, min_flow: u32) -> Self {
        Self { selector, sources, sinks, min_flow, model }
    }

    fn compute_max_flow(&self, assignment: &[PatternId], adjacency: &AdjacencyView) -> u32 {
        let n = assignment.len();
        let super_source = n;
        let super_sink = n + 1;
        let mut net = FlowNetwork::new(n + 2);

        for i in 0..n {
            if !self.selector.matches(&self.model, assignment[i]) {
                continue;
            }
            for &m in adjacency.neighbors(NodeId::from_index(i)) {
                if self.selector.matches(&self.model, assignment[m.index()]) {
                    net.add_edge(i, m.index(), 1);
                }
            }
        }
        for &s in &self.sources {
            if self.selector.matches(&self.model, assignment[s.index()]) {
                net.add_edge(super_source, s.index(), u32::MAX);
            }
        }
        for &t in &self.sinks {
            if self.selector.matches(&self.model, assignment[t.index()]) {
                net.add_edge(t.index(), super_sink, u32::MAX);
            }
        }
        net.max_flow(super_source, super_sink)
    }
}

impl Constraint for FlowConstraint {
    fn initialize(&self, _domains: &DomainStore, _weights: &WeightTable, _adjacency: &AdjacencyView) -> Result<Vec<(NodeId, PatternSet)>, ConstraintError> {
        Ok(Vec::new())
    }

    fn validate_complete(&self, assignment: &[PatternId], adjacency: &AdjacencyView) -> Result<(), String> {
        let flow = self.compute_max_flow(assignment, adjacency);
        if flow < self.min_flow {
            Err(format!("flow constraint: max flow {flow} is below the required minimum {}", self.min_flow))
        } else {
            Ok(())
        }
    }
}
// #endregion 🔖️Constraint

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
