//! 🔢️ Cardinality constraints: bound how many nodes (in some scope) end up assigned a pattern
//! matching a selector.

use crate::wfc_engine::bitset::PatternSet;
use crate::wfc_engine::constraint::{AdjacencyView, Constraint, Exactness, PatternSelector};
use crate::wfc_engine::domain::DomainStore;
use crate::wfc_engine::error::ConstraintError;
use crate::wfc_engine::ids::{NodeId, PatternId, RegionId};
use crate::wfc_engine::model::CompiledModel;
use crate::wfc_engine::weights::WeightTable;

// #region 🔖️Scope
/// 🔢️ Which nodes a cardinality bound applies to.
#[derive(Clone, Debug)]
pub enum Scope {
    All,
    Region(RegionId),
    Nodes(Vec<NodeId>),
}

impl Scope {
    fn contains(&self, n: NodeId, adjacency: &AdjacencyView) -> bool {
        match self {
            Scope::All => true,
            Scope::Region(r) => adjacency.region_of(n) == *r,
            Scope::Nodes(nodes) => nodes.contains(&n),
        }
    }
}
// #endregion 🔖️Scope

// #region 🔖️Constraint
/// 🔢️ Requires that between `min` and `max` (inclusive) nodes in `scope` end up matching
/// `selector`. `min == max` is an exact-count constraint.
#[derive(Clone, Debug)]
pub struct CardinalityConstraint {
    pub selector: PatternSelector,
    pub scope: Scope,
    pub min: u32,
    pub max: u32,
    model: CompiledModel,
}

impl CardinalityConstraint {
    pub fn new(model: CompiledModel, selector: PatternSelector, scope: Scope, min: u32, max: u32) -> Result<Self, ConstraintError> {
        if min > max {
            return Err(ConstraintError::InvalidBounds { reason: "min must not exceed max" });
        }
        Ok(Self { selector, scope, min, max, model })
    }
}

impl Constraint for CardinalityConstraint {
    fn name(&self) -> &'static str {
        "cardinality"
    }

    fn exactness(&self) -> Exactness {
        Exactness::Exact
    }

    fn initialize(&self, domains: &DomainStore, _weights: &WeightTable, adjacency: &AdjacencyView) -> Result<Vec<(NodeId, PatternSet)>, ConstraintError> {
        let selected = self.selector.as_pattern_set(&self.model);
        let scoped_nodes: Vec<NodeId> = (0..adjacency.node_count()).map(NodeId::from_index).filter(|&n| self.scope.contains(n, adjacency)).collect();

        // How many scoped nodes could still take a selected pattern, and how many are already
        // forced to (their domain is a selected-pattern-only singleton)?
        let possible = scoped_nodes.iter().filter(|&&n| domains.get(n).bits().intersects(&selected)).count() as u32;
        let required = scoped_nodes.iter().filter(|&&n| domains.get(n).bits().is_subset_of(&selected)).count() as u32;

        let mut out = Vec::new();
        if self.max < required {
            // Already over-required with no way to satisfy `max` — signal via an emptied domain
            // on the first scoped node so the caller's normal wipeout handling takes over.
            if let Some(&n) = scoped_nodes.first() {
                out.push((n, PatternSet::new_empty(self.model.pattern_count())));
            }
            return Ok(out);
        }
        if possible < self.min {
            if let Some(&n) = scoped_nodes.first() {
                out.push((n, PatternSet::new_empty(self.model.pattern_count())));
            }
            return Ok(out);
        }
        if possible == self.min && self.min > required {
            // Every possible-but-not-yet-required node must take the selected pattern to reach `min`.
            for &n in &scoped_nodes {
                let bits = domains.get(n).bits();
                if bits.intersects(&selected) && !bits.is_subset_of(&selected) {
                    out.push((n, selected.clone()));
                }
            }
        }
        if self.max == required {
            // No further scoped node may take the selected pattern.
            let mut not_selected = selected.clone();
            not_selected.clear_all();
            for i in 0..self.model.pattern_count() {
                not_selected.set(PatternId::from_index(i), !selected.get(PatternId::from_index(i)));
            }
            for &n in &scoped_nodes {
                let bits = domains.get(n).bits();
                if bits.intersects(&selected) && !bits.is_subset_of(&selected) {
                    out.push((n, not_selected.clone()));
                }
            }
        }
        Ok(out)
    }

    fn validate_complete(&self, assignment: &[PatternId], adjacency: &AdjacencyView) -> Result<(), String> {
        let count = (0..adjacency.node_count()).filter(|&n| self.scope.contains(NodeId::from_index(n), adjacency)).filter(|&n| self.selector.matches(&self.model, assignment[n])).count() as u32;
        if count < self.min || count > self.max {
            return Err(format!("cardinality constraint: expected [{}, {}], found {count}", self.min, self.max));
        }
        Ok(())
    }
}
// #endregion 🔖️Constraint

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
