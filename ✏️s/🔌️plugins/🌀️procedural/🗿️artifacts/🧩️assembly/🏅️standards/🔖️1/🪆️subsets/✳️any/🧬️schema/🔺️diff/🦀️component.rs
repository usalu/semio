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
    #[state(artifact)]
    pub schema: Option<String>,
    #[state(artifact)]
    pub seed: Option<u64>,
    #[state(artifact)]
    pub slots_removed: Vec<String>,
    #[state(artifact)]
    pub slots_upserted: Vec<(usize, AssemblySlot)>,
    #[state(artifact)]
    pub edges_removed: Vec<String>,
    #[state(artifact)]
    pub edges_upserted: Vec<(usize, AssemblySlotEdge)>,
    #[state(artifact)]
    pub weights_removed: Vec<String>,
    #[state(artifact)]
    pub weights_upserted: Vec<AssemblyModuleWeight>,
    #[state(artifact)]
    pub rules_removed: Vec<String>,
    #[state(artifact)]
    pub rules_upserted: Vec<(usize, AssemblyRule)>,
}
//#endregion 🔖️AssemblyDiff

//#region 🔖️IdKeyedMerge
/// 🔀 Generic id-keyed upsert/remove merge, shared by every collection field's `absorb` step: `self`
/// is base→mid, `other` is mid→after — a later remove always wins over an earlier upsert of the SAME
/// id, and a later upsert always clears any earlier remove of the same id.
async fn merge_upserts<T: Clone>(self_removed: &[String], self_upserted: &[(usize, T)], self_key: impl Fn(&T) -> &str, other_removed: &[String], other_upserted: &[(usize, T)], other_key: impl Fn(&T) -> &str) -> (Vec<String>, Vec<(usize, T)>) {
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
/// 🧬 Validates and applies one id-keyed indexed collection delta atomically.
async fn apply_collection<T: Clone>(base: &[T], removed: &[String], upserted: &[(usize, T)], key: impl Fn(&T) -> &str) -> protocol::MutationApplyResult<Vec<T>> {
    for (index, id) in removed.iter().enumerate() {
        if !base.iter().any(|item| key(item) == id) {
            return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "removed item does not exist").at(["removed".to_string(), index.to_string()]));
        }
        if removed[..index].contains(id) {
            return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "item is removed more than once").at(["removed".to_string(), index.to_string()]));
        }
    }
    for (position, (index, value)) in upserted.iter().enumerate() {
        let value_key = key(value).to_string();
        if removed.contains(&value_key) {
            return Err(protocol::MutationApplyError::new("mutation.apply.conflicting-target", "item cannot be removed and upserted").at(["upserted".to_string(), position.to_string()]));
        }
        if upserted[..position].iter().any(|(_, prior)| key(prior) == value_key) {
            return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "item is upserted more than once").at(["upserted".to_string(), position.to_string()]));
        }
        if let Some(existing_index) = base.iter().position(|item| key(item) == value_key) {
            if *index != existing_index {
                return Err(protocol::MutationApplyError::new("mutation.apply.invalid-index", "replacement index does not match the existing item").at(["upserted".to_string(), position.to_string()]));
            }
        } else {
            let preceding_additions = upserted[..position].iter().filter(|(_, prior)| !base.iter().any(|item| key(item) == key(prior))).count();
            let available = base.len() - removed.len() + preceding_additions;
            if *index > available {
                return Err(protocol::MutationApplyError::new("mutation.apply.invalid-index", format!("insertion index {index} exceeds length {available}")).at(["upserted".to_string(), position.to_string()]));
            }
            if upserted[..position].iter().any(|(prior_index, prior)| prior_index == index && !base.iter().any(|item| key(item) == key(prior))) {
                return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "insertion index is targeted more than once").at(["upserted".to_string(), position.to_string()]));
            }
        }
    }
    let mut items: Vec<T> = base.iter().filter(|item| !removed.iter().any(|id| id == key(item))).cloned().collect();
    for (index, value) in upserted {
        let value_key = key(value).to_string();
        if let Some(existing) = items.iter_mut().find(|item| key(item) == value_key) {
            *existing = value.clone();
        } else {
            items.insert(*index, value.clone());
        }
    }
    Ok(items)
}

async fn apply_unordered_collection<T: Clone>(base: &[T], removed: &[String], upserted: &[T], key: impl Fn(&T) -> &str) -> protocol::MutationApplyResult<Vec<T>> {
    for (index, id) in removed.iter().enumerate() {
        if !base.iter().any(|item| key(item) == id) {
            return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "removed item does not exist").at(["removed".to_string(), index.to_string()]));
        }
        if removed[..index].contains(id) {
            return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "item is removed more than once").at(["removed".to_string(), index.to_string()]));
        }
    }
    for (index, value) in upserted.iter().enumerate() {
        let id = key(value);
        if removed.iter().any(|removed| removed == id) {
            return Err(protocol::MutationApplyError::new("mutation.apply.conflicting-target", "item cannot be removed and upserted").at(["upserted".to_string(), index.to_string()]));
        }
        if upserted[..index].iter().any(|prior| key(prior) == id) {
            return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "item is upserted more than once").at(["upserted".to_string(), index.to_string()]));
        }
    }
    let mut items: Vec<T> = base.iter().filter(|item| !removed.iter().any(|id| id == key(item))).cloned().collect();
    for value in upserted {
        if let Some(existing) = items.iter_mut().find(|item| key(item) == key(value)) {
            *existing = value.clone();
        } else {
            items.push(value.clone());
        }
    }
    Ok(items)
}

impl protocol::MutationDiff<AssemblySnapshot> for AssemblyDiff {
    async fn apply(&self, base: &AssemblySnapshot) -> protocol::MutationApplyResult<AssemblySnapshot> {
        Ok({
            let mut next = base.clone();
            if let Some(schema) = &self.schema {
                next.schema = schema.clone();
            }
            if let Some(seed) = self.seed {
                next.seed = seed;
            }
            next.slots = apply_collection(&next.slots, &self.slots_removed, &self.slots_upserted, |slot| slot.id.as_str()).map_err(|error| error.under(["slots"]))?;
            next.edges = apply_collection(&next.edges, &self.edges_removed, &self.edges_upserted, |edge| edge.id.as_str()).map_err(|error| error.under(["edges"]))?;
            next.rules = apply_collection(&next.rules, &self.rules_removed, &self.rules_upserted, |rule| rule.id.as_str()).map_err(|error| error.under(["rules"]))?;
            next.weights = apply_unordered_collection(&next.weights, &self.weights_removed, &self.weights_upserted, |weight| weight.module_id.as_str()).map_err(|error| error.under(["weights"]))?;
            next
        })
    }
    async fn absorb(&mut self, other: Self) {
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

    async fn base() -> AssemblySnapshot {
        let mut snapshot = AssemblySnapshot::default();
        snapshot.slots.push(AssemblySlot { id: "s1".into(), x: 0.0, y: 0.0, z: 0.0, pinned_module_id: None });
        snapshot
    }

    #[semio_framework_async_macros::async_test]
    async fn upsert_by_id_replaces_in_place_never_duplicates() {
        let diff = AssemblyDiff { slots_upserted: vec![(0, AssemblySlot { id: "s1".into(), x: 9.0, y: 9.0, z: 0.0, pinned_module_id: None })], ..Default::default() };
        let after = diff.apply(&base()).expect("valid mutation diff");
        assert_eq!(after.slots.len(), 1);
        assert_eq!(after.slots[0].x, 9.0);
    }

    #[semio_framework_async_macros::async_test]
    async fn insert_at_index_for_a_new_id() {
        let diff = AssemblyDiff { slots_upserted: vec![(1, AssemblySlot { id: "s2".into(), x: 1.0, y: 1.0, z: 0.0, pinned_module_id: None })], ..Default::default() };
        let after = diff.apply(&base()).expect("valid mutation diff");
        assert_eq!(after.slots.len(), 2);
        assert_eq!(after.slots[1].id, "s2");
    }

    #[semio_framework_async_macros::async_test]
    async fn malformed_indexed_diff_rejects_without_changing_the_base() {
        let base = base();
        let diff = AssemblyDiff { slots_upserted: vec![(99, AssemblySlot { id: "s2".into(), ..Default::default() })], ..Default::default() };
        let error = diff.apply(&base).expect_err("out-of-range insertion must reject");
        assert_eq!(error.code, "mutation.apply.invalid-index");
        assert_eq!(error.target, ["slots", "upserted", "0"]);
        assert_eq!(base.slots.len(), 1);
        assert_eq!(base.slots[0].id, "s1");
    }

    #[semio_framework_async_macros::async_test]
    async fn remove_drops_the_matching_id_only() {
        let diff = AssemblyDiff { slots_removed: vec!["s1".into()], ..Default::default() };
        let after = diff.apply(&base()).expect("valid mutation diff");
        assert!(after.slots.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn absorb_a_later_remove_wins_over_an_earlier_upsert_of_the_same_id() {
        let mut d1 = AssemblyDiff { slots_upserted: vec![(0, AssemblySlot { id: "s2".into(), ..Default::default() })], ..Default::default() };
        let d2 = AssemblyDiff { slots_removed: vec!["s2".into()], ..Default::default() };
        d1.absorb(d2);
        assert!(d1.slots_upserted.is_empty());
        assert_eq!(d1.slots_removed, vec!["s2".to_string()]);
    }

    #[semio_framework_async_macros::async_test]
    async fn absorb_a_later_upsert_clears_an_earlier_remove_of_the_same_id() {
        let mut d1 = AssemblyDiff { slots_removed: vec!["s1".into()], ..Default::default() };
        let d2 = AssemblyDiff { slots_upserted: vec![(0, AssemblySlot { id: "s1".into(), x: 5.0, ..Default::default() })], ..Default::default() };
        d1.absorb(d2);
        assert!(d1.slots_removed.is_empty());
        assert_eq!(d1.slots_upserted.len(), 1);
        assert_eq!(d1.slots_upserted[0].1.x, 5.0);
    }

    #[semio_framework_async_macros::async_test]
    async fn absorb_composes_to_the_same_result_as_applying_sequentially() {
        let start = base();
        let d1 = AssemblyDiff { seed: Some(7), ..Default::default() };
        let mid = d1.apply(&start).expect("valid mutation diff");
        let d2 = AssemblyDiff { slots_removed: vec!["s1".into()], ..Default::default() };
        let after_sequential = d2.apply(&mid).expect("valid mutation diff");
        let mut composed = d1.clone();
        composed.absorb(d2);
        let after_composed = composed.apply(&start).expect("valid mutation diff");
        assert_eq!(after_sequential, after_composed);
    }
}
//#endregion 🧪️Tests
