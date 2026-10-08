//! 🔺️ `Grid3dDiff` — a field-sparse, id-keyed delta over `Grid3dSnapshot`: scalars are optional absolute values, every list carries removed identities, added rows
//! (landing at their canonical position) and per-row field patches — never a whole row or list copy. `absorb` coalesces per id and per field, `inverse` restores exact base values.

use crate::schema::snapshot::{cell_key, Grid3dCell, Grid3dDirection, Grid3dPinnedCell, Grid3dRule, Grid3dSnapshot, Grid3dTile, Grid3dTileMedia};
use ::semio_framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Rows
/// 🔑️ A row of an id-keyed list kept in canonical order: `row_key` addresses it, `insert_at` is where a new row lands.
pub trait Grid3dRow: Clone + PartialEq {
    fn row_key(&self) -> String;
    fn insert_at(items: &[Self], row: &Self) -> usize;
}

/// 🩹 Field-sparse patch over one row: names the fields it sets with the values they take.
pub trait Grid3dPatch: Clone + Default + PartialEq {
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
pub struct Grid3dRowPatch<Q> {
    pub id: String,
    pub patch: Q,
}

/// 📂 Id-keyed row delta: removed identities, added rows (landing at their canonical position) and per-row field patches.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct Grid3dRows<T, Q> {
    #[value(default)]
    pub removed: Vec<String>,
    #[value(default)]
    pub added: Vec<T>,
    #[value(default)]
    pub patched: Vec<Grid3dRowPatch<Q>>,
}

impl<T, Q> Default for Grid3dRows<T, Q> {
    fn default() -> Self {
        Self { removed: Vec::new(), added: Vec::new(), patched: Vec::new() }
    }
}

fn rejection(code: &'static str, message: &'static str, lane: &str, id: &str) -> protocol::MutationApplyError {
    protocol::MutationApplyError::new(code, message).at([lane, id])
}

impl<T: Grid3dRow, Q: Grid3dPatch<Row = T>> Grid3dRows<T, Q> {
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
        let patched: Vec<Grid3dRowPatch<Q>> = self
            .patched
            .iter()
            .filter(|entry| !self.removed.contains(&entry.id) && !self.added.iter().any(|row| row.row_key() == entry.id))
            .filter_map(|entry| find(&entry.id).map(|row| Grid3dRowPatch { id: entry.id.clone(), patch: entry.patch.inverse(row) }))
            .filter(|entry| !entry.patch.is_empty())
            .collect();
        Self { removed, added, patched }
    }

    /// 🧭️ The delta that turns `from` into `to`.
    pub fn between(from: &[T], to: &[T]) -> Self {
        let removed: Vec<String> = from.iter().map(T::row_key).filter(|key| !to.iter().any(|row| row.row_key() == *key)).collect();
        let added: Vec<T> = to.iter().filter(|row| !from.iter().any(|other| other.row_key() == row.row_key())).cloned().collect();
        let patched: Vec<Grid3dRowPatch<Q>> = from
            .iter()
            .filter_map(|row| to.iter().find(|other| other.row_key() == row.row_key()).filter(|other| *other != row).map(|other| Grid3dRowPatch { id: row.row_key(), patch: Q::between(row, other) }))
            .collect();
        Self { removed, added, patched }
    }
}
//#endregion 🔖️Rows

//#region 🔖️Optionals
/// 🔤 An optional String field set to a value or cleared.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct Grid3dOptionalText {
    pub value: Option<String>,
}
//#endregion 🔖️Optionals

//#region 🔖️PatchMacro
/// 🩹 Declares a field-sparse patch struct for `$row` and its [`Grid3dPatch`] impl: `plain` fields set a value, `optional` fields set or clear an `Option` through their wrapper.
macro_rules! wfc_patch {
    ($(#[$doc:meta])* $name:ident for $row:ty { plain { $($field:ident : $ty:ty),* } optional { $($ofield:ident : $wrap:ident),* } }) => {
        $(#[$doc])*
        #[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
        #[value(rename_all = "camelCase", default)]
        pub struct $name {
            $(pub $field: Option<$ty>,)*
            $(pub $ofield: Option<$wrap>,)*
        }

        impl Grid3dPatch for $name {
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

//#region 🔖️Axes
/// 📏 One cell-size row of an axis: the size the cell at `index` takes.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct Grid3dAxisSize {
    pub index: u32,
    pub size: f64,
}

/// 📏 Sparse change of one axis' cell sizes: an optional new length, then per-cell size rows (cells appended by a longer length are set by rows too).
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct Grid3dAxisPatch {
    pub length: Option<u32>,
    pub sizes: Vec<Grid3dAxisSize>,
}

impl Grid3dAxisPatch {
    /// 🔑️ The axis' sizes after the patch: resized to `length`, then every row's cell set; a row past the end is rejected.
    pub fn patched(&self, base: &[f64]) -> protocol::MutationApplyResult<Vec<f64>> {
        let mut next = base.to_vec();
        if let Some(length) = self.length {
            next.resize(length as usize, 0.0);
        }
        for (position, row) in self.sizes.iter().enumerate() {
            if self.sizes[..position].iter().any(|prior| prior.index == row.index) {
                return Err(rejection("mutation.apply.duplicate-target", "cell size is set more than once", "sizes", &row.index.to_string()));
            }
            let cell = next.get_mut(row.index as usize).ok_or_else(|| rejection("mutation.apply.invalid-index", "cell size index is past the axis", "sizes", &row.index.to_string()))?;
            *cell = row.size;
        }
        Ok(next)
    }

    /// 🧭️ The patch that turns the `from` sizes into the `to` sizes.
    pub fn between(from: &[f64], to: &[f64]) -> Self {
        let sizes = to.iter().enumerate().filter(|(index, size)| from.get(*index) != Some(*size)).map(|(index, size)| Grid3dAxisSize { index: index as u32, size: *size }).collect();
        Self { length: (from.len() != to.len()).then_some(to.len() as u32), sizes }
    }

    /// 🔁️ The patch that, applied after this one, restores the `base` sizes.
    pub fn inverse(&self, base: &[f64]) -> Self {
        let truncated = self.length.map_or(0..0, |length| (length as usize).min(base.len())..base.len());
        let mut indices: Vec<u32> = self.sizes.iter().map(|row| row.index).filter(|index| (*index as usize) < base.len()).chain(truncated.map(|index| index as u32)).collect();
        indices.sort_unstable();
        indices.dedup();
        Self { length: self.length.map(|_| base.len() as u32), sizes: indices.into_iter().map(|index| Grid3dAxisSize { index, size: base[index as usize] }).collect() }
    }

    /// ➕️ Composes a later patch: the later length wins and cuts earlier rows past it, the later row of a cell wins.
    pub fn absorb(&mut self, later: Self) {
        if let Some(length) = later.length {
            self.length = Some(length);
            self.sizes.retain(|row| row.index < length);
        }
        for row in later.sizes {
            match self.sizes.iter_mut().find(|existing| existing.index == row.index) {
                Some(existing) => existing.size = row.size,
                None => self.sizes.push(row),
            }
        }
        self.sizes.sort_by_key(|row| row.index);
    }

    /// 🕳️ Whether the patch changes nothing.
    pub fn is_empty(&self) -> bool {
        self.length.is_none() && self.sizes.is_empty()
    }
}
//#endregion 🔖️Axes

//#region 🔖️RowTypes
impl Grid3dRow for Grid3dTile {
    fn row_key(&self) -> String {
        self.id.clone()
    }
    fn insert_at(items: &[Self], row: &Self) -> usize {
        items.iter().position(|item| item.id >= row.id).unwrap_or(items.len())
    }
}

wfc_patch!(/// 🀄️ Field patch over a tile (its id is the row identity).
    Grid3dTilePatch for Grid3dTile { plain { weight: f64, media: Grid3dTileMedia } optional { label: Grid3dOptionalText } });

/// 📂 Row delta over the tiles.
pub type Grid3dTilesDelta = Grid3dRows<Grid3dTile, Grid3dTilePatch>;

impl Grid3dRow for Grid3dRule {
    fn row_key(&self) -> String {
        self.id.clone()
    }
    fn insert_at(items: &[Self], row: &Self) -> usize {
        items.iter().position(|item| item.id >= row.id).unwrap_or(items.len())
    }
}

wfc_patch!(/// ⛓️ Field patch over an adjacency rule (its id is the row identity).
    Grid3dRulePatch for Grid3dRule { plain { tile_a_id: String, tile_b_id: String, direction: Grid3dDirection, allowed: bool } optional {  } });

/// 📂 Row delta over the rules.
pub type Grid3dRulesDelta = Grid3dRows<Grid3dRule, Grid3dRulePatch>;

impl Grid3dRow for Grid3dPinnedCell {
    fn row_key(&self) -> String {
        cell_key(self.x, self.y, self.z)
    }
    fn insert_at(items: &[Self], row: &Self) -> usize {
        items.iter().position(|item| cell_key(item.x, item.y, item.z) >= cell_key(row.x, row.y, row.z)).unwrap_or(items.len())
    }
}

wfc_patch!(/// 📌️ Field patch over a pinned cell (its `x:y:z` key is the row identity).
    Grid3dPinnedPatch for Grid3dPinnedCell { plain { tile_id: String } optional {  } });

/// 📂 Row delta over the pinned.
pub type Grid3dPinnedDelta = Grid3dRows<Grid3dPinnedCell, Grid3dPinnedPatch>;

impl Grid3dRow for Grid3dCell {
    fn row_key(&self) -> String {
        cell_key(self.x, self.y, self.z)
    }
    fn insert_at(items: &[Self], row: &Self) -> usize {
        items.iter().position(|item| cell_key(item.x, item.y, item.z) >= cell_key(row.x, row.y, row.z)).unwrap_or(items.len())
    }
}

wfc_patch!(/// 🕳️ Field patch over a masked cell (a masked cell has no field besides its coordinates).
    Grid3dMaskedPatch for Grid3dCell { plain {  } optional {  } });

/// 📂 Row delta over the masked.
pub type Grid3dMaskedDelta = Grid3dRows<Grid3dCell, Grid3dMaskedPatch>;

//#endregion 🔖️RowTypes

//#region 🔖️Diff
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
    pub periodic_x: Option<bool>,
    #[state(artifact)]
    pub periodic_y: Option<bool>,
    #[state(artifact)]
    pub periodic_z: Option<bool>,
    #[state(artifact)]
    pub cell_sizes_x: Option<Grid3dAxisPatch>,
    #[state(artifact)]
    pub cell_sizes_y: Option<Grid3dAxisPatch>,
    #[state(artifact)]
    pub cell_sizes_z: Option<Grid3dAxisPatch>,
    #[state(artifact)]
    pub tiles: Grid3dTilesDelta,
    #[state(artifact)]
    pub rules: Grid3dRulesDelta,
    #[state(artifact)]
    pub pinned: Grid3dPinnedDelta,
    #[state(artifact)]
    pub masked: Grid3dMaskedDelta,
}
//#endregion 🔖️Diff

//#region 🔖️Apply
impl protocol::MutationDiff<Grid3dSnapshot> for Grid3dDiff {
    fn apply(&self, base: &Grid3dSnapshot, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<Grid3dSnapshot> {
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
        if let Some(depth) = self.depth {
            next.depth = depth;
        }
        if let Some(periodic_x) = self.periodic_x {
            next.periodic_x = periodic_x;
        }
        if let Some(periodic_y) = self.periodic_y {
            next.periodic_y = periodic_y;
        }
        if let Some(periodic_z) = self.periodic_z {
            next.periodic_z = periodic_z;
        }
        if let Some(patch) = &self.cell_sizes_x {
            next.cell_sizes_x = patch.patched(&base.cell_sizes_x).map_err(|error| error.under(["cellSizesX"]))?;
        }
        if let Some(patch) = &self.cell_sizes_y {
            next.cell_sizes_y = patch.patched(&base.cell_sizes_y).map_err(|error| error.under(["cellSizesY"]))?;
        }
        if let Some(patch) = &self.cell_sizes_z {
            next.cell_sizes_z = patch.patched(&base.cell_sizes_z).map_err(|error| error.under(["cellSizesZ"]))?;
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
        if later.depth.is_some() {
            self.depth = later.depth;
        }
        if later.periodic_x.is_some() {
            self.periodic_x = later.periodic_x;
        }
        if later.periodic_y.is_some() {
            self.periodic_y = later.periodic_y;
        }
        if later.periodic_z.is_some() {
            self.periodic_z = later.periodic_z;
        }
        match (&mut self.cell_sizes_x, later.cell_sizes_x) {
            (Some(earlier), Some(later)) => earlier.absorb(later),
            (target @ None, later) => *target = later,
            (Some(_), None) => {}
        }
        match (&mut self.cell_sizes_y, later.cell_sizes_y) {
            (Some(earlier), Some(later)) => earlier.absorb(later),
            (target @ None, later) => *target = later,
            (Some(_), None) => {}
        }
        match (&mut self.cell_sizes_z, later.cell_sizes_z) {
            (Some(earlier), Some(later)) => earlier.absorb(later),
            (target @ None, later) => *target = later,
            (Some(_), None) => {}
        }
        self.tiles.absorb(later.tiles);
        self.rules.absorb(later.rules);
        self.pinned.absorb(later.pinned);
        self.masked.absorb(later.masked);
    }
}

impl protocol::DiffAlgebra<Grid3dSnapshot> for Grid3dDiff {
    fn inverse(&self, base: &Grid3dSnapshot) -> Self {
        Self {
            schema: self.schema.as_ref().map(|_| base.schema.clone()),
            seed: self.seed.map(|_| base.seed),
            width: self.width.map(|_| base.width),
            height: self.height.map(|_| base.height),
            depth: self.depth.map(|_| base.depth),
            periodic_x: self.periodic_x.map(|_| base.periodic_x),
            periodic_y: self.periodic_y.map(|_| base.periodic_y),
            periodic_z: self.periodic_z.map(|_| base.periodic_z),
            cell_sizes_x: self.cell_sizes_x.as_ref().map(|patch| patch.inverse(&base.cell_sizes_x)).filter(|patch| !patch.is_empty()),
            cell_sizes_y: self.cell_sizes_y.as_ref().map(|patch| patch.inverse(&base.cell_sizes_y)).filter(|patch| !patch.is_empty()),
            cell_sizes_z: self.cell_sizes_z.as_ref().map(|patch| patch.inverse(&base.cell_sizes_z)).filter(|patch| !patch.is_empty()),
            tiles: self.tiles.inverse(&base.tiles),
            rules: self.rules.inverse(&base.rules),
            pinned: self.pinned.inverse(&base.pinned),
            masked: self.masked.inverse(&base.masked),
        }
    }
    fn between(base: &Grid3dSnapshot, other: &Grid3dSnapshot) -> Self {
        Self {
            schema: (base.schema != other.schema).then(|| other.schema.clone()),
            seed: (base.seed != other.seed).then_some(other.seed),
            width: (base.width != other.width).then_some(other.width),
            height: (base.height != other.height).then_some(other.height),
            depth: (base.depth != other.depth).then_some(other.depth),
            periodic_x: (base.periodic_x != other.periodic_x).then_some(other.periodic_x),
            periodic_y: (base.periodic_y != other.periodic_y).then_some(other.periodic_y),
            periodic_z: (base.periodic_z != other.periodic_z).then_some(other.periodic_z),
            cell_sizes_x: Some(Grid3dAxisPatch::between(&base.cell_sizes_x, &other.cell_sizes_x)).filter(|patch| !patch.is_empty()),
            cell_sizes_y: Some(Grid3dAxisPatch::between(&base.cell_sizes_y, &other.cell_sizes_y)).filter(|patch| !patch.is_empty()),
            cell_sizes_z: Some(Grid3dAxisPatch::between(&base.cell_sizes_z, &other.cell_sizes_z)).filter(|patch| !patch.is_empty()),
            tiles: Grid3dRows::between(&base.tiles, &other.tiles),
            rules: Grid3dRows::between(&base.rules, &other.rules),
            pinned: Grid3dRows::between(&base.pinned, &other.pinned),
            masked: Grid3dRows::between(&base.masked, &other.masked),
        }
    }
    fn is_empty(&self) -> bool {
        self.schema.is_none()
            && self.seed.is_none()
            && self.width.is_none()
            && self.height.is_none()
            && self.depth.is_none()
            && self.periodic_x.is_none()
            && self.periodic_y.is_none()
            && self.periodic_z.is_none()
            && self.cell_sizes_x.as_ref().is_none_or(Grid3dAxisPatch::is_empty)
            && self.cell_sizes_y.as_ref().is_none_or(Grid3dAxisPatch::is_empty)
            && self.cell_sizes_z.as_ref().is_none_or(Grid3dAxisPatch::is_empty)
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
