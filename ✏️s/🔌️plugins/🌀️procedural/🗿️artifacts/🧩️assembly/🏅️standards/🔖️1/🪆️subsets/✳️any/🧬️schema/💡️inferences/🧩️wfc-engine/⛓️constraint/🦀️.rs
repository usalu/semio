//! 🧷️ Global constraints beyond binary arc compatibility. Deliberately stateless (no per-constraint
//! mutable state to roll back on backtrack — the exact class of subtle bug this crate already hit
//! twice, once in `crate::wfc_engine::search`'s trail and once in `crate::wfc_engine::prop_ac4`'s counters): a constraint
//! only ever (1) restricts initial domains once, before search starts (no rollback needed — the
//! same treatment `crate::wfc_engine::search::solve` already gives fixed pins and per-node domain overrides),
//! and (2) validates a *complete* (all-singleton) candidate assignment, rejecting it — which
//! `crate::wfc_engine::search`'s existing, proven `backtrack_and_repair` machinery turns into an ordinary
//! backtrack, exactly as if a domain had been wiped. Full incremental mid-search propagation for
//! global constraints (tightening domains as decisions narrow them, not just at the two safe
//! points above) is deferred alongside AC-4's rollback integration.

use crate::wfc_engine::bitset::PatternSet;
use crate::wfc_engine::constraints_card::CardinalityConstraint;
use crate::wfc_engine::constraints_conn::{ConnectivityConstraint, ReachabilityConstraint};
use crate::wfc_engine::domain::DomainStore;
use crate::wfc_engine::error::ConstraintError;
use crate::wfc_engine::flow::FlowConstraint;
use crate::wfc_engine::ids::{NodeId, PatternId, RegionId};
use crate::wfc_engine::model::CompiledModel;
use crate::wfc_engine::weights::WeightTable;
// 🚦️ De-dyn (O1/R11 closed-set case): every `impl Constraint` lives in this same crate (4 total,
// imported above), so the closed-set mechanism applies — `dyn_enum_close!` below generates
// `Constraints`, replacing every `Box<dyn Constraint>` seam. See
// `.🧬semio/…/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/📓️terra-dedyn-fleet-procedural-report.md`.
use semio_framework_dispatch_macros::{dyn_enum, dyn_enum_close};

// #region 🔖️Selector
/// 🧷️ Which patterns a node counts as "selected" for a constraint — shared by every constraint
/// type in this crate so a caller building, say, a cardinality *and* a connectivity constraint
/// over the same "walkable" patterns writes the selector once.
#[derive(Clone, Debug)]
pub enum PatternSelector {
    Pattern(PatternId),
    Tag(u32),
    Any(PatternSet),
}

impl PatternSelector {
    pub fn matches(&self, model: &CompiledModel, p: PatternId) -> bool {
        match self {
            PatternSelector::Pattern(target) => *target == p,
            PatternSelector::Tag(tag) => model.pattern_info(p).tags.contains(tag),
            PatternSelector::Any(set) => set.get(p),
        }
    }

    pub fn as_pattern_set(&self, model: &CompiledModel) -> PatternSet {
        match self {
            PatternSelector::Any(set) => set.clone(),
            _ => {
                let mut set = PatternSet::new_empty(model.pattern_count());
                for i in 0..model.pattern_count() {
                    let p = PatternId::from_index(i);
                    if self.matches(model, p) {
                        set.set(p, true);
                    }
                }
                set
            }
        }
    }
}
// #endregion 🔖️Selector

// #region 🔖️Adjacency
/// 🧷️ A materialized, object-safe neighbor view — built once per solve from whatever concrete
/// `Topology` is in play, so constraints (which must work identically across `GraphTopology`,
/// `Grid2dTopology`, `Grid3dTopology`) never need `Topology` itself to be object-safe.
#[derive(Clone, Debug)]
pub struct AdjacencyView {
    neighbors: Vec<Vec<NodeId>>,
    regions: Vec<RegionId>,
}

impl AdjacencyView {
    pub(crate) fn new(neighbors: Vec<Vec<NodeId>>, regions: Vec<RegionId>) -> Self {
        debug_assert_eq!(neighbors.len(), regions.len());
        Self { neighbors, regions }
    }

    pub fn node_count(&self) -> usize {
        self.neighbors.len()
    }

    pub fn neighbors(&self, n: NodeId) -> &[NodeId] {
        &self.neighbors[n.index()]
    }

    pub fn region_of(&self, n: NodeId) -> RegionId {
        self.regions[n.index()]
    }
}

/// 🧷️ Materializes an [`AdjacencyView`] from any concrete `Topology` — the one place this crate
/// converts the hot-path, non-object-safe `Topology` trait into the object-safe shape constraints
/// need. Called once per solver `build()`, not per solve attempt.
pub(crate) fn build_adjacency_view<T: crate::wfc_engine::topology::Topology>(topo: &T) -> AdjacencyView {
    let node_count = topo.node_count();
    let mut neighbors = vec![Vec::new(); node_count];
    let mut regions = vec![RegionId(0); node_count];
    for i in 0..node_count {
        let n = NodeId::from_index(i);
        topo.for_each_out_arc(n, |m, _r| neighbors[i].push(m));
        regions[i] = topo.region_of(n);
    }
    AdjacencyView::new(neighbors, regions)
}
// #endregion 🔖️Adjacency

// #region 🔖️Constraint
/// 🧷️ Whether a constraint's [`Constraint::validate_complete`] is a sound-and-complete check (a
/// failure there always means the assignment is genuinely invalid — safe to use in exhaustive/
/// unsat-proof search) or merely a heuristic approximation.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Exactness {
    Exact,
    Heuristic,
}

/// 🧷️ One global constraint.
#[dyn_enum]
pub trait Constraint {
    fn name(&self) -> &'static str;
    fn exactness(&self) -> Exactness;

    /// 🧷️ Restricts initial per-node domains once, before search starts. Returning a narrower
    /// `PatternSet` than a node's current entry in `domains` intersects it in; returning the same
    /// set is a no-op. Called once per solve attempt, before the first propagation pass.
    fn initialize(&self, domains: &DomainStore, weights: &WeightTable, adjacency: &AdjacencyView) -> Result<Vec<(NodeId, PatternSet)>, ConstraintError>;

    /// 🧷️ Checks one complete (every node singleton) candidate assignment. `Ok(())` means this
    /// constraint accepts it.
    fn validate_complete(&self, assignment: &[PatternId], adjacency: &AdjacencyView) -> Result<(), String>;
}

// 🧷️ Every concrete `Constraint` this crate implements, closed here (bare `dyn_enum_close!`
// invocation — same module, trait declared first, see `dyn-enum-macro`'s recipe for why that
// avoids rustc#52234) rather than left `dyn`. Replaces every former boxed constraint field.
dyn_enum_close! {
    pub enum Constraints: Constraint {
        Flow(FlowConstraint),
        Connectivity(ConnectivityConstraint),
        Reachability(ReachabilityConstraint),
        Cardinality(CardinalityConstraint),
    }
}

// #region 🔖️Dispatch
impl Constraints {
    pub(crate) fn initialize(&self, domains: &DomainStore, weights: &WeightTable, adjacency: &AdjacencyView) -> Result<Vec<(NodeId, PatternSet)>, ConstraintError> {
        match self {
            Self::Flow(value) => Constraint::initialize(value, domains, weights, adjacency),
            Self::Connectivity(value) => Constraint::initialize(value, domains, weights, adjacency),
            Self::Reachability(value) => Constraint::initialize(value, domains, weights, adjacency),
            Self::Cardinality(value) => Constraint::initialize(value, domains, weights, adjacency),
        }
    }

    pub(crate) fn validate_complete(&self, assignment: &[PatternId], adjacency: &AdjacencyView) -> Result<(), String> {
        match self {
            Self::Flow(value) => Constraint::validate_complete(value, assignment, adjacency),
            Self::Connectivity(value) => Constraint::validate_complete(value, assignment, adjacency),
            Self::Reachability(value) => Constraint::validate_complete(value, assignment, adjacency),
            Self::Cardinality(value) => Constraint::validate_complete(value, assignment, adjacency),
        }
    }
}
// #endregion 🔖️Dispatch

/// 🧷️ A solver's constraints plus the adjacency view they read — bundled so `crate::wfc_engine::search`'s
/// internals take one extra `Option<&ConstraintSet>` parameter instead of two.
pub(crate) struct ConstraintSet<'a> {
    pub constraints: &'a [Constraints],
    pub adjacency: &'a AdjacencyView,
}
// #endregion 🔖️Constraint

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adjacency_view_exposes_neighbors_and_regions() {
        let view = AdjacencyView::new(vec![vec![NodeId(1)], vec![NodeId(0), NodeId(2)], vec![NodeId(1)]], vec![RegionId(0), RegionId(1), RegionId(0)]);
        assert_eq!(view.node_count(), 3);
        assert_eq!(view.neighbors(NodeId(1)), &[NodeId(0), NodeId(2)]);
        assert_eq!(view.region_of(NodeId(1)), RegionId(1));
    }
}
// #endregion 🔖️Tests
