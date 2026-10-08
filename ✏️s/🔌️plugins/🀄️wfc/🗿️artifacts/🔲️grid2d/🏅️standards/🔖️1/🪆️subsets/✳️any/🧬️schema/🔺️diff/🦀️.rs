//! 🔺️ `Grid2dDiff` — a field-sparse, id-keyed delta over `Grid2dSnapshot`: scalars are optional absolute values, every list carries removed identities, added rows
//! (landing at their canonical position) and per-row field patches — never a whole row or list copy. `absorb` coalesces per id and per field, `inverse` restores exact base values.

use crate::schema::snapshot::{Grid2dSnapshot, WfcAdjacencyRule2d, WfcCell2d, WfcDirection2d, WfcPinnedCell2d, WfcTile2d, WfcTileMedia2d};
use ::semio_framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Rows
/// 🔑️ A row of an id-keyed list kept in canonical order: `row_key` addresses it, `insert_at` is where a new row lands.
pub trait Grid2dRow: Clone + PartialEq {
    fn row_key(&self) -> String;
    fn insert_at(items: &[Self], row: &Self) -> usize;
}

/// 🩹 Field-sparse patch over one row: names the fields it sets with the values they take.
pub trait Grid2dPatch: Clone + Default + PartialEq {
    type Row;
    fn patched(&self, row: &Self::Row) -> Self::Row;
    fn between(from: &Self::Row, to: &Self::Row) -> Self;
    fn inverse(&self, base: &Self::Row) -> Self;
    fn absorb(&mut self, later: Self);
    fn is_empty(&self) -> bool;
}

/// 🩹 One patched row, addressed by its identity.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct Grid2dRowPatch<Q> {
    pub id: String,
    pub patch: Q,
}

/// 📂 Id-keyed row delta: removed identities, added rows (landing at their canonical position) and per-row field patches.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct Grid2dRows<T, Q> {
    #[value(default)]
    pub removed: Vec<String>,
    #[value(default)]
    pub added: Vec<T>,
    #[value(default)]
    pub patched: Vec<Grid2dRowPatch<Q>>,
}

impl<T, Q> Default for Grid2dRows<T, Q> {
    fn default() -> Self {
        Self { removed: Vec::new(), added: Vec::new(), patched: Vec::new() }
    }
}

fn rejection(code: &'static str, message: &'static str, lane: &str, id: &str) -> protocol::MutationApplyError {
    protocol::MutationApplyError::new(code, message).at([lane, id])
}

impl<T: Grid2dRow, Q: Grid2dPatch<Row = T>> Grid2dRows<T, Q> {
    /// 🕳️ Whether the delta changes nothing.
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.added.is_empty() && self.patched.is_empty()
    }

    /// 🔑️ The list after the delta: removals first, then canonical-position insertions, then field patches; unknown targets and duplicated identities are rejected.
    pub fn apply(&self, items: &[T]) -> protocol::MutationApplyResult<Vec<T>> {
        let mut next = items.to_vec();
        for (position, id) in self.removed.iter().enumerate() {
            if self.removed[..position].contains(id) {
                return Err(rejection("mutation.apply.duplicate-target", "item is removed more than once", "removed", id));
            }
            let at = next.iter().position(|item| item.row_key() == *id).ok_or_else(|| rejection("mutation.apply.missing-target", "removed item does not exist", "removed", id))?;
            next.remove(at);
        }
        for (position, row) in self.added.iter().enumerate() {
            let key = row.row_key();
            if self.added[..position].iter().any(|prior| prior.row_key() == key) || next.iter().any(|item| item.row_key() == key) {
                return Err(rejection("mutation.apply.duplicate-target", "added item identity already exists", "added", &key));
            }
            let at = T::insert_at(&next, row);
            next.insert(at, row.clone());
        }
        for (position, entry) in self.patched.iter().enumerate() {
            if self.patched[..position].iter().any(|prior| prior.id == entry.id) {
                return Err(rejection("mutation.apply.duplicate-target", "item is patched more than once", "patched", &entry.id));
            }
            let at = next.iter().position(|item| item.row_key() == entry.id).ok_or_else(|| rejection("mutation.apply.missing-target", "patched item does not exist", "patched", &entry.id))?;
            let patched = entry.patch.patched(&next[at]);
            next[at] = patched;
        }
        Ok(next)
    }

    /// ➕️ Composes a later delta: create∘delete cancels, patch∘delete leaves the deletion, patch∘patch coalesces, delete∘create replaces.
    pub fn absorb(&mut self, later: Self) {
        for id in later.removed {
            if let Some(position) = self.added.iter().position(|row| row.row_key() == id) {
                self.added.remove(position);
            } else {
                self.patched.retain(|entry| entry.id != id);
                self.removed.push(id);
            }
        }
        self.added.extend(later.added);
        for entry in later.patched {
            if let Some(row) = self.added.iter_mut().find(|row| row.row_key() == entry.id) {
                *row = entry.patch.patched(row);
            } else if let Some(existing) = self.patched.iter_mut().find(|existing| existing.id == entry.id) {
                existing.patch.absorb(entry.patch);
            } else {
                self.patched.push(entry);
            }
        }
    }

    /// 🔁️ The delta that, applied after this one, restores `base` rows and positions exactly.
    pub fn inverse(&self, base: &[T]) -> Self {
        let find = |id: &str| base.iter().find(|row| row.row_key() == id);
        let removed: Vec<String> = self.added.iter().map(T::row_key).collect();
        let added: Vec<T> = self.removed.iter().filter_map(|id| find(id).cloned()).collect();
        let patched: Vec<Grid2dRowPatch<Q>> = self
            .patched
            .iter()
            .filter(|entry| !self.removed.contains(&entry.id) && !self.added.iter().any(|row| row.row_key() == entry.id))
            .filter_map(|entry| find(&entry.id).map(|row| Grid2dRowPatch { id: entry.id.clone(), patch: entry.patch.inverse(row) }))
            .filter(|entry| !entry.patch.is_empty())
            .collect();
        Self { removed, added, patched }
    }

    /// 🧭️ The delta that turns `from` into `to`.
    pub fn between(from: &[T], to: &[T]) -> Self {
        let removed: Vec<String> = from.iter().map(T::row_key).filter(|key| !to.iter().any(|row| row.row_key() == *key)).collect();
        let added: Vec<T> = to.iter().filter(|row| !from.iter().any(|other| other.row_key() == row.row_key())).cloned().collect();
        let patched: Vec<Grid2dRowPatch<Q>> = from
            .iter()
            .filter_map(|row| to.iter().find(|other| other.row_key() == row.row_key()).filter(|other| *other != row).map(|other| Grid2dRowPatch { id: row.row_key(), patch: Q::between(row, other) }))
            .collect();
        Self { removed, added, patched }
    }
}
//#endregion 🔖️Rows

//#region 🔖️Optionals
/// 🔤 An optional String field set to a value or cleared.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct Grid2dOptionalText {
    pub value: Option<String>,
}
//#endregion 🔖️Optionals

//#region 🔖️PatchMacro
/// 🩹 Declares a field-sparse patch struct for `$row` and its [`Grid2dPatch`] impl: `plain` fields set a value, `optional` fields set or clear an `Option` through their wrapper.
macro_rules! wfc_patch {
    ($(#[$doc:meta])* $name:ident for $row:ty { plain { $($field:ident : $ty:ty),* } optional { $($ofield:ident : $wrap:ident),* } }) => {
        $(#[$doc])*
        #[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
        #[value(rename_all = "camelCase", default)]
        pub struct $name {
            $(pub $field: Option<$ty>,)*
            $(pub $ofield: Option<$wrap>,)*
        }

        impl Grid2dPatch for $name {
            type Row = $row;
            fn patched(&self, row: &$row) -> $row {
                #[allow(unused_mut)]
                let mut next = row.clone();
                $(if let Some(value) = &self.$field { next.$field = value.clone(); })*
                $(if let Some(value) = &self.$ofield { next.$ofield = value.value.clone(); })*
                next
            }
            fn between(from: &$row, to: &$row) -> Self {
                Self { $($field: (from.$field != to.$field).then(|| to.$field.clone()),)* $($ofield: (from.$ofield != to.$ofield).then(|| $wrap { value: to.$ofield.clone() }),)* }
            }
            fn inverse(&self, base: &$row) -> Self {
                Self { $($field: self.$field.as_ref().map(|_| base.$field.clone()),)* $($ofield: self.$ofield.as_ref().map(|_| $wrap { value: base.$ofield.clone() }),)* }
            }
            fn absorb(&mut self, later: Self) {
                $(if later.$field.is_some() { self.$field = later.$field; })*
                $(if later.$ofield.is_some() { self.$ofield = later.$ofield; })*
            }
            fn is_empty(&self) -> bool {
                true $(&& self.$field.is_none())* $(&& self.$ofield.is_none())*
            }
        }
    };
}
//#endregion 🔖️PatchMacro

//#region 🔖️Keys
/// 🔑️ The removal key of a grid cell — `"<x>,<y>"`, the same spelling both cell collections use.
pub fn cell_id(x: u32, y: u32) -> String {
    format!("{x},{y}")
}
//#endregion 🔖️Keys

//#region 🔖️RowTypes
impl Grid2dRow for WfcTile2d {
    fn row_key(&self) -> String {
        self.id.clone()
    }
    fn insert_at(items: &[Self], row: &Self) -> usize {
        items.iter().position(|item| item.id > row.id).unwrap_or(items.len())
    }
}

wfc_patch!(/// 🀄️ Field patch over a tile (its id is the row identity).
    Grid2dTilePatch for WfcTile2d { plain { weight: f64, media: WfcTileMedia2d } optional { label: Grid2dOptionalText } });

/// 📂 Row delta over the tiles.
pub type Grid2dTilesDelta = Grid2dRows<WfcTile2d, Grid2dTilePatch>;

impl Grid2dRow for WfcAdjacencyRule2d {
    fn row_key(&self) -> String {
        self.id.clone()
    }
    fn insert_at(items: &[Self], row: &Self) -> usize {
        items.iter().position(|item| item.id > row.id).unwrap_or(items.len())
    }
}

wfc_patch!(/// ⛓️ Field patch over an adjacency rule (its id is the row identity).
    Grid2dRulePatch for WfcAdjacencyRule2d { plain { tile_a_id: String, tile_b_id: String, direction: WfcDirection2d, allowed: bool } optional {  } });

/// 📂 Row delta over the rules.
pub type Grid2dRulesDelta = Grid2dRows<WfcAdjacencyRule2d, Grid2dRulePatch>;

impl Grid2dRow for WfcPinnedCell2d {
    fn row_key(&self) -> String {
        format!("{},{}", self.x, self.y)
    }
    fn insert_at(items: &[Self], row: &Self) -> usize {
        items.iter().position(|item| (item.y, item.x) > (row.y, row.x)).unwrap_or(items.len())
    }
}

wfc_patch!(/// 📌️ Field patch over a pinned cell (its `x,y` is the row identity).
    Grid2dPinnedPatch for WfcPinnedCell2d { plain { tile_id: String } optional {  } });

/// 📂 Row delta over the pinned.
pub type Grid2dPinnedDelta = Grid2dRows<WfcPinnedCell2d, Grid2dPinnedPatch>;

impl Grid2dRow for WfcCell2d {
    fn row_key(&self) -> String {
        format!("{},{}", self.x, self.y)
    }
    fn insert_at(items: &[Self], row: &Self) -> usize {
        items.iter().position(|item| (item.y, item.x) > (row.y, row.x)).unwrap_or(items.len())
    }
}

wfc_patch!(/// 🕳️ Field patch over a masked cell (a masked cell has no field besides its coordinates).
    Grid2dMaskedPatch for WfcCell2d { plain {  } optional {  } });

/// 📂 Row delta over the masked.
pub type Grid2dMaskedDelta = Grid2dRows<WfcCell2d, Grid2dMaskedPatch>;

//#endregion 🔖️RowTypes

//#region 🔖️Diff
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
    pub tiles: Grid2dTilesDelta,
    #[state(artifact)]
    pub rules: Grid2dRulesDelta,
    #[state(artifact)]
    pub pinned: Grid2dPinnedDelta,
    #[state(artifact)]
    pub masked: Grid2dMaskedDelta,
}
//#endregion 🔖️Diff

//#region 🔖️Apply
impl protocol::MutationDiff<Grid2dSnapshot> for Grid2dDiff {
    fn apply(&self, base: &Grid2dSnapshot, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<Grid2dSnapshot> {
        let mut next = base.clone();
        if let Some(schema) = &self.schema {
            next.schema.clone_from(schema);
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
        next.tiles = self.tiles.apply(&base.tiles).map_err(|error| error.under(["tiles"]))?;
        next.rules = self.rules.apply(&base.rules).map_err(|error| error.under(["rules"]))?;
        next.pinned = self.pinned.apply(&base.pinned).map_err(|error| error.under(["pinned"]))?;
        next.masked = self.masked.apply(&base.masked).map_err(|error| error.under(["masked"]))?;
        Ok(next)
    }
    fn absorb(&mut self, later: Self) {
        if later.schema.is_some() {
            self.schema = later.schema;
        }
        if later.seed.is_some() {
            self.seed = later.seed;
        }
        if later.width.is_some() {
            self.width = later.width;
        }
        if later.height.is_some() {
            self.height = later.height;
        }
        if later.cell_width.is_some() {
            self.cell_width = later.cell_width;
        }
        if later.cell_height.is_some() {
            self.cell_height = later.cell_height;
        }
        if later.periodic_x.is_some() {
            self.periodic_x = later.periodic_x;
        }
        if later.periodic_y.is_some() {
            self.periodic_y = later.periodic_y;
        }
        self.tiles.absorb(later.tiles);
        self.rules.absorb(later.rules);
        self.pinned.absorb(later.pinned);
        self.masked.absorb(later.masked);
    }
}

impl protocol::DiffAlgebra<Grid2dSnapshot> for Grid2dDiff {
    fn inverse(&self, base: &Grid2dSnapshot) -> Self {
        Self {
            schema: self.schema.as_ref().map(|_| base.schema.clone()),
            seed: self.seed.map(|_| base.seed),
            width: self.width.map(|_| base.width),
            height: self.height.map(|_| base.height),
            cell_width: self.cell_width.map(|_| base.cell_width),
            cell_height: self.cell_height.map(|_| base.cell_height),
            periodic_x: self.periodic_x.map(|_| base.periodic_x),
            periodic_y: self.periodic_y.map(|_| base.periodic_y),
            tiles: self.tiles.inverse(&base.tiles),
            rules: self.rules.inverse(&base.rules),
            pinned: self.pinned.inverse(&base.pinned),
            masked: self.masked.inverse(&base.masked),
        }
    }
    fn between(base: &Grid2dSnapshot, other: &Grid2dSnapshot) -> Self {
        Self {
            schema: (base.schema != other.schema).then(|| other.schema.clone()),
            seed: (base.seed != other.seed).then_some(other.seed),
            width: (base.width != other.width).then_some(other.width),
            height: (base.height != other.height).then_some(other.height),
            cell_width: (base.cell_width != other.cell_width).then_some(other.cell_width),
            cell_height: (base.cell_height != other.cell_height).then_some(other.cell_height),
            periodic_x: (base.periodic_x != other.periodic_x).then_some(other.periodic_x),
            periodic_y: (base.periodic_y != other.periodic_y).then_some(other.periodic_y),
            tiles: Grid2dRows::between(&base.tiles, &other.tiles),
            rules: Grid2dRows::between(&base.rules, &other.rules),
            pinned: Grid2dRows::between(&base.pinned, &other.pinned),
            masked: Grid2dRows::between(&base.masked, &other.masked),
        }
    }
    fn is_empty(&self) -> bool {
        self.schema.is_none()
            && self.seed.is_none()
            && self.width.is_none()
            && self.height.is_none()
            && self.cell_width.is_none()
            && self.cell_height.is_none()
            && self.periodic_x.is_none()
            && self.periodic_y.is_none()
            && self.tiles.is_empty()
            && self.rules.is_empty()
            && self.pinned.is_empty()
            && self.masked.is_empty()
    }
}
//#endregion 🔖️Apply

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
