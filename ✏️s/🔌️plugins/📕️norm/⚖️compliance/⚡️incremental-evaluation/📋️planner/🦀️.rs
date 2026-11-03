//! 📋️ Planner: changed set to the ordered units that must be revisited.

use super::*;

/// 🔺️ What a mutation touched: document paths and entities, or everything.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ChangedSet {
    pub paths: Vec<String>,
    pub entities: Vec<EntityRef>,
    pub everything: bool,
}

impl ChangedSet {
    pub fn all() -> Self {
        Self { everything: true, ..Self::default() }
    }

    pub fn at(paths: impl IntoIterator<Item = impl Into<String>>, entities: Vec<EntityRef>) -> Self {
        Self { paths: paths.into_iter().map(Into::into).collect(), entities, everything: false }
    }
}

/// 📌️ One unit scheduled by a plan.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PlannedUnit {
    pub index: usize,
    pub id: UnitId,
    pub tier: CostTier,
}

/// 🗺️ Ordered units to revisit (cheap tiers first, units near the focus first) and the token that supersedes it.
#[derive(Clone, Debug)]
pub struct Plan {
    pub units: Vec<PlannedUnit>,
    pub token: CancelToken,
}

/// 🧭️ Plans the units whose declared reads overlap the changed paths, whose subjects depend on a changed entity, or that are stale.
pub fn plan_units<D>(units: &[CheckUnit<D>], graph: &EntityGraph, changed: &ChangedSet, stale: &HashSet<UnitId>, focus: &[EntityRef], token: CancelToken) -> Plan {
    let dirty = graph.closure(&changed.entities);
    let mut planned: Vec<(bool, PlannedUnit)> = units
        .iter()
        .enumerate()
        .filter(|(_, unit)| changed.everything || stale.contains(&unit.id) || unit.subjects.iter().any(|subject| dirty.contains(subject)) || changed.paths.iter().any(|path| unit.reads.overlaps(path)))
        .map(|(index, unit)| (!unit.subjects.iter().any(|subject| focus.contains(subject)), PlannedUnit { index, id: unit.id.clone(), tier: unit.tier }))
        .collect();
    planned.sort_by_key(|(far, unit)| (unit.tier, *far, unit.index));
    Plan { units: planned.into_iter().map(|(_, unit)| unit).collect(), token }
}
