//! 🔺️ `Wfc3dDiff` — a sparse, id-keyed, structural delta over `Wfc3dSnapshot`. Never a
//! whole-snapshot capture: each mutation's own `🔺️diff/🦀️.rs` builds one of these directly from
//! `(payload, base)`. `absorb` is structural (map-merge over ids), never re-derived from applied
//! snapshot values. Every collection lane carries its member's INDEX, which is what makes an
//! inverse position-exact.

use crate::schema::snapshot::{GraphRule, Slot3d, SlotEdge, Tile, Wfc3dSnapshot};
use ::semio_framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};
use std::collections::BTreeMap;

//#region 🔖️Wfc3dDiff
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.wfc.wfc3d")]
pub struct Wfc3dDiff {
    #[state(artifact)]
    pub schema: Option<String>,
    #[state(artifact)]
    pub seed: Option<u64>,
    #[state(artifact)]
    pub slots_removed: Vec<String>,
    #[state(artifact)]
    pub slots_upserted: Vec<(usize, Slot3d)>,
    #[state(artifact)]
    pub edges_removed: Vec<String>,
    #[state(artifact)]
    pub edges_upserted: Vec<(usize, SlotEdge)>,
    #[state(artifact)]
    pub tiles_removed: Vec<String>,
    #[state(artifact)]
    pub tiles_upserted: Vec<(usize, Tile)>,
    #[state(artifact)]
    pub rules_removed: Vec<String>,
    #[state(artifact)]
    pub rules_upserted: Vec<(usize, GraphRule)>,
}
//#endregion 🔖️Wfc3dDiff

//#region 🔖️IdKeyedMerge
/// 🔀 Generic id-keyed upsert/remove merge, shared by every collection field's `absorb` step: `self`
/// is base→mid, `other` is mid→after — a later remove always wins over an earlier upsert of the SAME
/// id, and a later upsert always clears any earlier remove of the same id.
fn merge_upserts<T: Clone>(self_removed: &[String], self_upserted: &[(usize, T)], self_key: impl Fn(&T) -> &str, other_removed: &[String], other_upserted: &[(usize, T)], other_key: impl Fn(&T) -> &str) -> (Vec<String>, Vec<(usize, T)>) {
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
fn apply_collection<T: Clone>(base: &[T], removed: &[String], upserted: &[(usize, T)], key: impl Fn(&T) -> &str) -> protocol::MutationApplyResult<Vec<T>> {
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

impl protocol::MutationDiff<Wfc3dSnapshot> for Wfc3dDiff {
    fn apply(&self, base: &Wfc3dSnapshot) -> protocol::MutationApplyResult<Wfc3dSnapshot> {
        let mut next = base.clone();
        if let Some(schema) = &self.schema {
            next.schema = schema.clone();
        }
        if let Some(seed) = self.seed {
            next.seed = seed;
        }
        next.slots = apply_collection(&next.slots, &self.slots_removed, &self.slots_upserted, |slot| slot.id.as_str()).map_err(|error| error.under(["slots"]))?;
        next.edges = apply_collection(&next.edges, &self.edges_removed, &self.edges_upserted, |edge| edge.id.as_str()).map_err(|error| error.under(["edges"]))?;
        next.tiles = apply_collection(&next.tiles, &self.tiles_removed, &self.tiles_upserted, |tile| tile.id.as_str()).map_err(|error| error.under(["tiles"]))?;
        next.rules = apply_collection(&next.rules, &self.rules_removed, &self.rules_upserted, |rule| rule.id.as_str()).map_err(|error| error.under(["rules"]))?;
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        if other.schema.is_some() {
            self.schema = other.schema;
        }
        if other.seed.is_some() {
            self.seed = other.seed;
        }
        let (removed, upserted) = merge_upserts(&self.slots_removed, &self.slots_upserted, |s: &Slot3d| s.id.as_str(), &other.slots_removed, &other.slots_upserted, |s: &Slot3d| s.id.as_str());
        self.slots_removed = removed;
        self.slots_upserted = upserted;
        let (removed, upserted) = merge_upserts(&self.edges_removed, &self.edges_upserted, |e: &SlotEdge| e.id.as_str(), &other.edges_removed, &other.edges_upserted, |e: &SlotEdge| e.id.as_str());
        self.edges_removed = removed;
        self.edges_upserted = upserted;
        let (removed, upserted) = merge_upserts(&self.tiles_removed, &self.tiles_upserted, |t: &Tile| t.id.as_str(), &other.tiles_removed, &other.tiles_upserted, |t: &Tile| t.id.as_str());
        self.tiles_removed = removed;
        self.tiles_upserted = upserted;
        let (removed, upserted) = merge_upserts(&self.rules_removed, &self.rules_upserted, |r: &GraphRule| r.id.as_str(), &other.rules_removed, &other.rules_upserted, |r: &GraphRule| r.id.as_str());
        self.rules_removed = removed;
        self.rules_upserted = upserted;
    }
}
//#endregion 🔖️Apply

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
