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
    async fn contains(&self, n: NodeId, adjacency: &AdjacencyView) -> bool {
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
    pub async fn new(model: CompiledModel, selector: PatternSelector, scope: Scope, min: u32, max: u32) -> Result<Self, ConstraintError> {
        if min > max {
            return Err(ConstraintError::InvalidBounds { reason: "min must not exceed max" });
        }
        Ok(Self { selector, scope, min, max, model })
    }
}

impl Constraint for CardinalityConstraint {
    async fn name(&self) -> &'static str {
        "cardinality"
    }

    async fn exactness(&self) -> Exactness {
        Exactness::Exact
    }

    async fn initialize(&self, domains: &DomainStore, _weights: &WeightTable, adjacency: &AdjacencyView) -> Result<Vec<(NodeId, PatternSet)>, ConstraintError> {
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

    async fn validate_complete(&self, assignment: &[PatternId], adjacency: &AdjacencyView) -> Result<(), String> {
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
mod tests {
    use super::*;
    use crate::wfc_engine::model::ModelBuilder;

    async fn two_pattern_model() -> CompiledModel {
        let mut b = ModelBuilder::new();
        let black = b.add_pattern(1.0);
        let white = b.add_pattern(1.0);
        let r = b.add_relation("adj");
        b.allow_mirrored(r, black, white);
        b.allow_mirrored(r, black, black);
        b.compile().unwrap()
    }

    async fn adjacency_line(n: usize) -> AdjacencyView {
        let mut neighbors = vec![Vec::new(); n];
        for i in 0..n.saturating_sub(1) {
            neighbors[i].push(NodeId::from_index(i + 1));
            neighbors[i + 1].push(NodeId::from_index(i));
        }
        AdjacencyView::new(neighbors, vec![RegionId(0); n])
    }

    #[test]
    async fn rejects_invalid_bounds() {
        let model = two_pattern_model();
        assert!(CardinalityConstraint::new(model, PatternSelector::Pattern(PatternId(0)), Scope::All, 5, 2).is_err());
    }

    #[test]
    async fn validate_complete_accepts_matching_count() {
        let model = two_pattern_model();
        let adjacency = adjacency_line(4);
        let c = CardinalityConstraint::new(model, PatternSelector::Pattern(PatternId(0)), Scope::All, 1, 2).unwrap();
        let assignment = vec![PatternId(0), PatternId(1), PatternId(0), PatternId(1)];
        assert!(c.validate_complete(&assignment, &adjacency).is_ok());
    }

    #[test]
    async fn validate_complete_rejects_out_of_bounds_count() {
        let model = two_pattern_model();
        let adjacency = adjacency_line(4);
        let c = CardinalityConstraint::new(model, PatternSelector::Pattern(PatternId(0)), Scope::All, 0, 1).unwrap();
        let assignment = vec![PatternId(0), PatternId(0), PatternId(0), PatternId(1)];
        assert!(c.validate_complete(&assignment, &adjacency).is_err());
    }

    #[test]
    async fn initialize_forces_pattern_when_min_equals_possible() {
        let model = two_pattern_model();
        let adjacency = adjacency_line(2);
        let weights = model.weights().clone();
        let mut domains = DomainStore::new_full(2, &weights);
        // Restrict node1 so only node0 can possibly satisfy "at least 1 pattern0".
        let mut white_only = PatternSet::new_empty(2);
        white_only.set(PatternId(1), true);
        domains.get_mut(NodeId(1)).restrict(&white_only, &weights);

        let c = CardinalityConstraint::new(model, PatternSelector::Pattern(PatternId(0)), Scope::All, 1, 2).unwrap();
        let restrictions = c.initialize(&domains, &weights, &adjacency).unwrap();
        assert!(restrictions.iter().any(|(n, set)| *n == NodeId(0) && set.get(PatternId(0)) && !set.get(PatternId(1))));
    }

    #[test]
    async fn initialize_signals_infeasible_min_via_empty_domain() {
        let model = two_pattern_model();
        let adjacency = adjacency_line(1);
        let weights = model.weights().clone();
        let domains = DomainStore::new_full(1, &weights);
        // Impossible: need at least 2 nodes matching pattern0, but only 1 node total.
        let c = CardinalityConstraint::new(model, PatternSelector::Pattern(PatternId(0)), Scope::All, 2, 2).unwrap();
        let restrictions = c.initialize(&domains, &weights, &adjacency).unwrap();
        assert!(restrictions.iter().any(|(_, set)| set.is_all_zero()));
    }
}
// #endregion 🔖️Tests
