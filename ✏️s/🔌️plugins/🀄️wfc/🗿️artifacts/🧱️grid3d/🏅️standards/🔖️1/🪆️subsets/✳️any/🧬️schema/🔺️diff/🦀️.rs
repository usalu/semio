//! 🔺️ `Grid3dDiff` — a real sparse, id-keyed structural delta over `Grid3dSnapshot`, never a
//! whole-snapshot capture: each mutation's own `🔺️diff/🦀️.rs` builds one of these directly from
//! `(payload, base)`. `absorb` is structural (map-merge over the same canonical keys), never
//! re-derived from applied snapshot values.

use crate::schema::snapshot::{cell_key, Grid3dCell, Grid3dPinnedCell, Grid3dRule, Grid3dSnapshot, Grid3dTile};
use ::semio_framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};
use std::collections::BTreeMap;

//#region 🔖️Grid3dDiff
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.wfc.grid3d")]
pub struct Grid3dDiff {
    #[state(artifact)]
    pub schema: Option<String>,
    #[state(artifact)]
    pub seed: Option<u64>,
    #[state(artifact)]
    pub width: Option<u32>,
    #[state(artifact)]
    pub height: Option<u32>,
    #[state(artifact)]
    pub depth: Option<u32>,
    #[state(artifact)]
    pub cell_sizes_x: Option<Vec<f64>>,
    #[state(artifact)]
    pub cell_sizes_y: Option<Vec<f64>>,
    #[state(artifact)]
    pub cell_sizes_z: Option<Vec<f64>>,
    #[state(artifact)]
    pub periodic_x: Option<bool>,
    #[state(artifact)]
    pub periodic_y: Option<bool>,
    #[state(artifact)]
    pub periodic_z: Option<bool>,
    #[state(artifact)]
    pub tiles_removed: Vec<String>,
    #[state(artifact)]
    pub tiles_upserted: Vec<(usize, Grid3dTile)>,
    #[state(artifact)]
    pub rules_removed: Vec<String>,
    #[state(artifact)]
    pub rules_upserted: Vec<(usize, Grid3dRule)>,
    #[state(artifact)]
    pub pinned_removed: Vec<String>,
    #[state(artifact)]
    pub pinned_upserted: Vec<(usize, Grid3dPinnedCell)>,
    #[state(artifact)]
    pub masked_removed: Vec<String>,
    #[state(artifact)]
    pub masked_upserted: Vec<(usize, Grid3dCell)>,
}
//#endregion 🔖️Grid3dDiff

//#region 🔖️Keys
pub fn tile_key(tile: &Grid3dTile) -> String {
    tile.id.clone()
}

pub fn rule_key(rule: &Grid3dRule) -> String {
    rule.id.clone()
}

pub fn pinned_key(cell: &Grid3dPinnedCell) -> String {
    cell_key(cell.x, cell.y, cell.z)
}

pub fn masked_key(cell: &Grid3dCell) -> String {
    cell_key(cell.x, cell.y, cell.z)
}
//#endregion 🔖️Keys

//#region 🔖️IdKeyedMerge
/// 🔀️ Generic key-keyed upsert/remove merge, shared by every collection field's `absorb` step:
/// `self` is base→mid, `other` is mid→after — a later remove always wins over an earlier upsert of
/// the SAME key, and a later upsert always clears an earlier remove of the same key.
fn merge_upserts<T: Clone>(self_removed: &[String], self_upserted: &[(usize, T)], other_removed: &[String], other_upserted: &[(usize, T)], key: impl Fn(&T) -> String) -> (Vec<String>, Vec<(usize, T)>) {
    let mut removed: BTreeMap<String, ()> = self_removed.iter().cloned().map(|id| (id, ())).collect();
    let mut upserted: BTreeMap<String, (usize, T)> = self_upserted.iter().map(|(index, value)| (key(value), (*index, value.clone()))).collect();
    for id in other_removed {
        upserted.remove(id);
        removed.insert(id.clone(), ());
    }
    for (index, value) in other_upserted {
        let id = key(value);
        removed.remove(&id);
        upserted.insert(id, (*index, value.clone()));
    }
    (removed.into_keys().collect(), upserted.into_values().collect())
}
//#endregion 🔖️IdKeyedMerge

//#region 🔖️Apply
/// 🧬️ Validates and applies one key-keyed indexed collection delta atomically — the shared body
/// every collection of this document goes through, so an out-of-range insert or a double remove is
/// refused identically everywhere.
fn apply_collection<T: Clone>(base: &[T], removed: &[String], upserted: &[(usize, T)], key: impl Fn(&T) -> String) -> protocol::MutationApplyResult<Vec<T>> {
    for (index, id) in removed.iter().enumerate() {
        if !base.iter().any(|item| &key(item) == id) {
            return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "removed item does not exist").at(["removed".to_string(), index.to_string()]));
        }
        if removed[..index].contains(id) {
            return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "item is removed more than once").at(["removed".to_string(), index.to_string()]));
        }
    }
    for (position, (index, value)) in upserted.iter().enumerate() {
        let value_key = key(value);
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
    let mut items: Vec<T> = base.iter().filter(|item| !removed.iter().any(|id| id == &key(item))).cloned().collect();
    for (index, value) in upserted {
        let value_key = key(value);
        if let Some(existing) = items.iter_mut().find(|item| key(item) == value_key) {
            *existing = value.clone();
        } else {
            items.insert(*index, value.clone());
        }
    }
    Ok(items)
}

impl protocol::MutationDiff<Grid3dSnapshot> for Grid3dDiff {
    fn apply(&self, base: &Grid3dSnapshot) -> protocol::MutationApplyResult<Grid3dSnapshot> {
        let mut next = base.clone();
        if let Some(schema) = &self.schema {
            next.schema = schema.clone();
        }
        if let Some(seed) = self.seed {
            next.seed = seed;
        }
        if let Some(width) = self.width {
            next.width = width;
        }
        if let Some(height) = self.height {
            next.height = height;
        }
        if let Some(depth) = self.depth {
            next.depth = depth;
        }
        if let Some(sizes) = &self.cell_sizes_x {
            next.cell_sizes_x = sizes.clone();
        }
        if let Some(sizes) = &self.cell_sizes_y {
            next.cell_sizes_y = sizes.clone();
        }
        if let Some(sizes) = &self.cell_sizes_z {
            next.cell_sizes_z = sizes.clone();
        }
        if let Some(periodic) = self.periodic_x {
            next.periodic_x = periodic;
        }
        if let Some(periodic) = self.periodic_y {
            next.periodic_y = periodic;
        }
        if let Some(periodic) = self.periodic_z {
            next.periodic_z = periodic;
        }
        next.tiles = apply_collection(&next.tiles, &self.tiles_removed, &self.tiles_upserted, tile_key).map_err(|error| error.under(["tiles"]))?;
        next.rules = apply_collection(&next.rules, &self.rules_removed, &self.rules_upserted, rule_key).map_err(|error| error.under(["rules"]))?;
        next.pinned = apply_collection(&next.pinned, &self.pinned_removed, &self.pinned_upserted, pinned_key).map_err(|error| error.under(["pinned"]))?;
        next.masked = apply_collection(&next.masked, &self.masked_removed, &self.masked_upserted, masked_key).map_err(|error| error.under(["masked"]))?;
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        if other.schema.is_some() {
            self.schema = other.schema;
        }
        if other.seed.is_some() {
            self.seed = other.seed;
        }
        if other.width.is_some() {
            self.width = other.width;
        }
        if other.height.is_some() {
            self.height = other.height;
        }
        if other.depth.is_some() {
            self.depth = other.depth;
        }
        if other.cell_sizes_x.is_some() {
            self.cell_sizes_x = other.cell_sizes_x;
        }
        if other.cell_sizes_y.is_some() {
            self.cell_sizes_y = other.cell_sizes_y;
        }
        if other.cell_sizes_z.is_some() {
            self.cell_sizes_z = other.cell_sizes_z;
        }
        if other.periodic_x.is_some() {
            self.periodic_x = other.periodic_x;
        }
        if other.periodic_y.is_some() {
            self.periodic_y = other.periodic_y;
        }
        if other.periodic_z.is_some() {
            self.periodic_z = other.periodic_z;
        }
        let (removed, upserted) = merge_upserts(&self.tiles_removed, &self.tiles_upserted, &other.tiles_removed, &other.tiles_upserted, tile_key);
        self.tiles_removed = removed;
        self.tiles_upserted = upserted;
        let (removed, upserted) = merge_upserts(&self.rules_removed, &self.rules_upserted, &other.rules_removed, &other.rules_upserted, rule_key);
        self.rules_removed = removed;
        self.rules_upserted = upserted;
        let (removed, upserted) = merge_upserts(&self.pinned_removed, &self.pinned_upserted, &other.pinned_removed, &other.pinned_upserted, pinned_key);
        self.pinned_removed = removed;
        self.pinned_upserted = upserted;
        let (removed, upserted) = merge_upserts(&self.masked_removed, &self.masked_upserted, &other.masked_removed, &other.masked_upserted, masked_key);
        self.masked_removed = removed;
        self.masked_upserted = upserted;
    }
}
//#endregion 🔖️Apply

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
