//! 🔺️ AssemblyDiff — a real sparse, id-keyed, structural delta over `AssemblySnapshot`. Never a
//! whole-snapshot capture: each mutation triad's `🔺️diff/🦀️component.rs` builds one of these
//! directly from `(payload, base)`. `absorb` is structural (map-merge over ids), never re-derived
//! from applied snapshot values.

use crate::artifacts::assembly::schema::snapshot::{AssemblyModuleWeight, AssemblyRule, AssemblySlot, AssemblySlotEdge, AssemblySnapshot};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region 🔖️AssemblyDiff
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.assembly")]
pub struct AssemblyDiff {
    #[state(artifact)] pub schema: Option<String>,
    #[state(artifact)] pub seed: Option<u64>,
    #[state(artifact)] pub slots_removed: Vec<String>,
    #[state(artifact)] pub slots_upserted: Vec<(usize, AssemblySlot)>,
    #[state(artifact)] pub edges_removed: Vec<String>,
    #[state(artifact)] pub edges_upserted: Vec<(usize, AssemblySlotEdge)>,
    #[state(artifact)] pub weights_removed: Vec<String>,
    #[state(artifact)] pub weights_upserted: Vec<AssemblyModuleWeight>,
    #[state(artifact)] pub rules_removed: Vec<String>,
    #[state(artifact)] pub rules_upserted: Vec<(usize, AssemblyRule)>,
}
//#endregion 🔖️AssemblyDiff

//#region 🔖️IdKeyedMerge
/// 🔀 Generic id-keyed upsert/remove merge, shared by every collection field's `absorb` step: `self`
/// is base→mid, `other` is mid→after — a later remove always wins over an earlier upsert of the SAME
/// id, and a later upsert always clears any earlier remove of the same id.
fn merge_upserts<T: Clone>(
    self_removed: &[String], self_upserted: &[(usize, T)], self_key: impl Fn(&T) -> &str,
    other_removed: &[String], other_upserted: &[(usize, T)], other_key: impl Fn(&T) -> &str,
) -> (Vec<String>, Vec<(usize, T)>) {
    let mut removed: BTreeMap<String, ()> = self_removed.iter().cloned().map(|id| (id, ())).collect();
    let mut upserted: BTreeMap<String, (usize, T)> = self_upserted.iter().map(|(i, v)| (self_key(v).to_string(), (*i, v.clone()))).collect();
    for id in other_removed {
        upserted.remove(id);
        removed.insert(id.clone(), ());
    }
    for (i, v) in other_upserted {
        let key = other_key(v).to_string();
        removed.remove(&key);
        upserted.insert(key, (*i, v.clone()));
    }
    (removed.into_keys().collect(), upserted.into_values().collect())
}
//#endregion 🔖️IdKeyedMerge

//#region 🔖️Apply
/// 🧬 Generic id-keyed collection apply: remove by id, then upsert (replace in place if the id
/// already exists, else insert at the given index clamped to bounds).
fn apply_collection<T: Clone>(base: &[T], removed: &[String], upserted: &[(usize, T)], key: impl Fn(&T) -> &str) -> Vec<T> {
    let mut items: Vec<T> = base.iter().filter(|item| !removed.contains(&key(item).to_string())).cloned().collect();
    for (index, value) in upserted {
        let value_key = key(value).to_string();
        if let Some(existing) = items.iter_mut().find(|item| key(item) == value_key) {
            *existing = value.clone();
        } else {
            let at = (*index).min(items.len());
            items.insert(at, value.clone());
        }
    }
    items
}

impl protocol::MutationDiff<AssemblySnapshot> for AssemblyDiff {
    fn apply(&self, base: &AssemblySnapshot) -> AssemblySnapshot {
        let mut next = base.clone();
        if let Some(schema) = &self.schema {
            next.schema = schema.clone();
        }
        if let Some(seed) = self.seed {
            next.seed = seed;
        }
        next.slots = apply_collection(&next.slots, &self.slots_removed, &self.slots_upserted, |slot| slot.id.as_str());
        next.edges = apply_collection(&next.edges, &self.edges_removed, &self.edges_upserted, |edge| edge.id.as_str());
        next.rules = apply_collection(&next.rules, &self.rules_removed, &self.rules_upserted, |rule| rule.id.as_str());
        let weights_upserted: Vec<(usize, AssemblyModuleWeight)> = self.weights_upserted.iter().map(|w| (usize::MAX, w.clone())).collect();
        next.weights = apply_collection(&next.weights, &self.weights_removed, &weights_upserted, |weight| weight.module_id.as_str());
        next
    }

    fn absorb(&mut self, other: Self) {
        if other.schema.is_some() {
            self.schema = other.schema;
        }
        if other.seed.is_some() {
            self.seed = other.seed;
        }
        let (removed, upserted) = merge_upserts(&self.slots_removed, &self.slots_upserted, |s: &AssemblySlot| s.id.as_str(), &other.slots_removed, &other.slots_upserted, |s: &AssemblySlot| s.id.as_str());
        self.slots_removed = removed;
        self.slots_upserted = upserted;
        let (removed, upserted) = merge_upserts(&self.edges_removed, &self.edges_upserted, |e: &AssemblySlotEdge| e.id.as_str(), &other.edges_removed, &other.edges_upserted, |e: &AssemblySlotEdge| e.id.as_str());
        self.edges_removed = removed;
        self.edges_upserted = upserted;
        let (removed, upserted) = merge_upserts(&self.rules_removed, &self.rules_upserted, |r: &AssemblyRule| r.id.as_str(), &other.rules_removed, &other.rules_upserted, |r: &AssemblyRule| r.id.as_str());
        self.rules_removed = removed;
        self.rules_upserted = upserted;
        let self_weights_indexed: Vec<(usize, AssemblyModuleWeight)> = self.weights_upserted.iter().cloned().map(|w| (0, w)).collect();
        let other_weights_indexed: Vec<(usize, AssemblyModuleWeight)> = other.weights_upserted.iter().cloned().map(|w| (0, w)).collect();
        let (removed, upserted) = merge_upserts(&self.weights_removed, &self_weights_indexed, |w: &AssemblyModuleWeight| w.module_id.as_str(), &other.weights_removed, &other_weights_indexed, |w: &AssemblyModuleWeight| w.module_id.as_str());
        self.weights_removed = removed;
        self.weights_upserted = upserted.into_iter().map(|(_, w)| w).collect();
    }
}
//#endregion 🔖️Apply

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::MutationDiff;

    fn base() -> AssemblySnapshot {
        let mut snapshot = AssemblySnapshot::default();
        snapshot.slots.push(AssemblySlot { id: "s1".into(), x: 0.0, y: 0.0, z: 0.0, pinned_module_id: None });
        snapshot
    }

    #[test]
    fn upsert_by_id_replaces_in_place_never_duplicates() {
        let diff = AssemblyDiff { slots_upserted: vec![(0, AssemblySlot { id: "s1".into(), x: 9.0, y: 9.0, z: 0.0, pinned_module_id: None })], ..Default::default() };
        let after = diff.apply(&base());
        assert_eq!(after.slots.len(), 1);
        assert_eq!(after.slots[0].x, 9.0);
    }

    #[test]
    fn insert_at_index_for_a_new_id() {
        let diff = AssemblyDiff { slots_upserted: vec![(1, AssemblySlot { id: "s2".into(), x: 1.0, y: 1.0, z: 0.0, pinned_module_id: None })], ..Default::default() };
        let after = diff.apply(&base());
        assert_eq!(after.slots.len(), 2);
        assert_eq!(after.slots[1].id, "s2");
    }

    #[test]
    fn remove_drops_the_matching_id_only() {
        let diff = AssemblyDiff { slots_removed: vec!["s1".into()], ..Default::default() };
        let after = diff.apply(&base());
        assert!(after.slots.is_empty());
    }

    #[test]
    fn absorb_a_later_remove_wins_over_an_earlier_upsert_of_the_same_id() {
        let mut d1 = AssemblyDiff { slots_upserted: vec![(0, AssemblySlot { id: "s2".into(), ..Default::default() })], ..Default::default() };
        let d2 = AssemblyDiff { slots_removed: vec!["s2".into()], ..Default::default() };
        d1.absorb(d2);
        assert!(d1.slots_upserted.is_empty());
        assert_eq!(d1.slots_removed, vec!["s2".to_string()]);
    }

    #[test]
    fn absorb_a_later_upsert_clears_an_earlier_remove_of_the_same_id() {
        let mut d1 = AssemblyDiff { slots_removed: vec!["s1".into()], ..Default::default() };
        let d2 = AssemblyDiff { slots_upserted: vec![(0, AssemblySlot { id: "s1".into(), x: 5.0, ..Default::default() })], ..Default::default() };
        d1.absorb(d2);
        assert!(d1.slots_removed.is_empty());
        assert_eq!(d1.slots_upserted.len(), 1);
        assert_eq!(d1.slots_upserted[0].1.x, 5.0);
    }

    #[test]
    fn absorb_composes_to_the_same_result_as_applying_sequentially() {
        let start = base();
        let d1 = AssemblyDiff { seed: Some(7), ..Default::default() };
        let mid = d1.apply(&start);
        let d2 = AssemblyDiff { slots_removed: vec!["s1".into()], ..Default::default() };
        let after_sequential = d2.apply(&mid);
        let mut composed = d1.clone();
        composed.absorb(d2);
        let after_composed = composed.apply(&start);
        assert_eq!(after_sequential, after_composed);
    }
}
//#endregion 🧪️Tests
