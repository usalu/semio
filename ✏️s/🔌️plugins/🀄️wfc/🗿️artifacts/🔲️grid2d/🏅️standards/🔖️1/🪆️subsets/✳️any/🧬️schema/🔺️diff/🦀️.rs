//! 🔺️ `Grid2dDiff` — a real sparse, key-addressed structural delta over `Grid2dSnapshot`, never a
//! whole-snapshot capture: each mutation's own `🔺️diff/🦀️.rs` builds one directly from
//! `(payload, base)`. Collection rows carry their FINAL-state index so `apply` restores POSITION as
//! well as VALUE — the "retained rows must be point-invertible" law: every insert lands at the
//! canonical sorted position (tiles/rules by id, cells row-major), so a delete's inverse recreates
//! the row exactly where it was.

use crate::schema::snapshot::{Grid2dSnapshot, WfcAdjacencyRule2d, WfcCell2d, WfcPinnedCell2d, WfcTile2d};
use ::semio_framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};
use std::collections::BTreeMap;

//#region 🔖️Keys
/// 🔑️ The removal key of a grid cell — `"<x>,<y>"`, the same spelling both cell collections use.
pub fn cell_id(x: u32, y: u32) -> String {
    format!("{x},{y}")
}
//#endregion 🔖️Keys

//#region 🔖️Grid2dDiff
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.wfc.grid2d")]
pub struct Grid2dDiff {
    #[state(artifact)]
    pub schema: Option<String>,
    #[state(artifact)]
    pub seed: Option<u64>,
    #[state(artifact)]
    pub width: Option<u32>,
    #[state(artifact)]
    pub height: Option<u32>,
    #[state(artifact)]
    pub cell_width: Option<f64>,
    #[state(artifact)]
    pub cell_height: Option<f64>,
    #[state(artifact)]
    pub periodic_x: Option<bool>,
    #[state(artifact)]
    pub periodic_y: Option<bool>,
    #[state(artifact)]
    pub tiles_removed: Vec<String>,
    #[state(artifact)]
    pub tiles_upserted: Vec<(usize, WfcTile2d)>,
    #[state(artifact)]
    pub rules_removed: Vec<String>,
    #[state(artifact)]
    pub rules_upserted: Vec<(usize, WfcAdjacencyRule2d)>,
    #[state(artifact)]
    pub pinned_removed: Vec<String>,
    #[state(artifact)]
    pub pinned_upserted: Vec<(usize, WfcPinnedCell2d)>,
    #[state(artifact)]
    pub masked_removed: Vec<String>,
    #[state(artifact)]
    pub masked_upserted: Vec<(usize, WfcCell2d)>,
}
//#endregion 🔖️Grid2dDiff

//#region 🔖️IdKeyedMerge
/// 🔀 Generic key-addressed upsert/remove merge shared by every collection lane: `self` is
/// base→mid, `other` is mid→after — a later remove wins over an earlier upsert of the SAME key, and
/// a later upsert clears any earlier remove of it.
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
/// 🧬 Validates and applies one key-addressed indexed collection delta atomically.
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

fn tile_key(tile: &WfcTile2d) -> String {
    tile.id.clone()
}
fn rule_key(rule: &WfcAdjacencyRule2d) -> String {
    rule.id.clone()
}
fn pinned_key(cell: &WfcPinnedCell2d) -> String {
    cell_id(cell.x, cell.y)
}
fn masked_key(cell: &WfcCell2d) -> String {
    cell_id(cell.x, cell.y)
}

impl protocol::MutationDiff<Grid2dSnapshot> for Grid2dDiff {
    fn apply(&self, base: &Grid2dSnapshot) -> protocol::MutationApplyResult<Grid2dSnapshot> {
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
        if let Some(cell_width) = self.cell_width {
            next.cell_width = cell_width;
        }
        if let Some(cell_height) = self.cell_height {
            next.cell_height = cell_height;
        }
        if let Some(periodic_x) = self.periodic_x {
            next.periodic_x = periodic_x;
        }
        if let Some(periodic_y) = self.periodic_y {
            next.periodic_y = periodic_y;
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
        if other.cell_width.is_some() {
            self.cell_width = other.cell_width;
        }
        if other.cell_height.is_some() {
            self.cell_height = other.cell_height;
        }
        if other.periodic_x.is_some() {
            self.periodic_x = other.periodic_x;
        }
        if other.periodic_y.is_some() {
            self.periodic_y = other.periodic_y;
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
