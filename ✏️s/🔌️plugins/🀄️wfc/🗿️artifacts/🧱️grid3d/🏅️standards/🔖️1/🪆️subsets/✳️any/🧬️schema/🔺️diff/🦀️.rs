//! 🔺️ `Grid3dDiff` — a field-sparse, id-keyed delta over `Grid3dSnapshot`: scalars are optional absolute values, every list carries removed identities, added rows
//! (landing at their canonical position) and per-row field patches — never a whole row or list copy. `absorb` coalesces per id and per field, `inverse` restores exact base values.

use crate::schema::snapshot::{cell_key, Grid3dCell, Grid3dDirection, Grid3dPinnedCell, Grid3dRule, Grid3dSnapshot, Grid3dTile, Grid3dTileMedia};
use ::semio_framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Rows
/// 📍️ The canonical position a new row of a list lands at in the after list.
pub trait Grid3dRow: Clone + PartialEq {
    fn insert_at(items: &[Self], row: &Self) -> usize;
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
/// 🩹 Declares a field-sparse patch struct for `$row` and its `protocol::list_delta::RowPatch` impl: `plain` fields set a value, `optional` fields set or clear an `Option` through their wrapper.
macro_rules! wfc_patch {
    ($(#[$doc:meta])* $name:ident for $row:ty { plain { $($field:ident : $ty:ty),* } optional { $($ofield:ident : $wrap:ident),* } }) => {
        $(#[$doc])*
        #[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
        #[value(rename_all = "camelCase", default)]
        pub struct $name {
            $(pub $field: Option<$ty>,)*
            $(pub $ofield: Option<$wrap>,)*
        }

        impl protocol::list_delta::RowPatch<$row> for $name {
            fn commit_into(&self, row: &mut $row, _capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
                $(if let Some(value) = &self.$field { row.$field = value.clone(); })*
                $(if let Some(value) = &self.$ofield { row.$ofield = value.value.clone(); })*
                Ok(())
            }
            fn inverse(&self, row: &$row) -> Self {
                Self { $($field: self.$field.as_ref().map(|_| row.$field.clone()),)* $($ofield: self.$ofield.as_ref().map(|_| $wrap { value: row.$ofield.clone() }),)* }
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
        let length = self.length.map_or(base.len(), |length| length as usize);
        for (position, row) in self.sizes.iter().enumerate() {
            if self.sizes[..position].iter().any(|prior| prior.index == row.index) {
                return Err(rejection("mutation.apply.duplicate-target", "cell size is set more than once", "sizes", &row.index.to_string()));
            }
            if row.index as usize >= length {
                return Err(rejection("mutation.apply.invalid-index", "cell size index is past the axis", "sizes", &row.index.to_string()));
            }
        }
        Ok((0..length).map(|index| self.sizes.iter().find(|row| row.index as usize == index).map_or_else(|| base.get(index).copied().unwrap_or(0.0), |row| row.size)).collect())
    }

    /// 📐️ The patch resizing the `base` sizes to `extent` cells: truncation sets only the length, growth also sets the appended cells.
    pub fn resizing(base: &[f64], extent: u32) -> Self {
        let next = crate::schema::snapshot::resized_axis(base, extent);
        let sizes = next.iter().enumerate().skip(base.len()).map(|(index, size)| Grid3dAxisSize { index: index as u32, size: *size }).collect();
        Self { length: (base.len() != next.len()).then_some(next.len() as u32), sizes }
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
    fn insert_at(items: &[Self], row: &Self) -> usize {
        items.iter().position(|item| item.id >= row.id).unwrap_or(items.len())
    }
}

wfc_patch!(/// 🀄️ Field patch over a tile (its id is the row identity).
    Grid3dTilePatch for Grid3dTile { plain { weight: f64, media: Grid3dTileMedia } optional { label: Grid3dOptionalText } });

protocol::list_delta! {
    /// 📂 Row delta over the tiles.
    pub Grid3dTilesDelta { removal: Grid3dTilesRemoval, insertion: Grid3dTilesInsertion, relocation: Grid3dTilesRelocation, modification: Grid3dTilesModification, row: Grid3dTile, patch: Grid3dTilePatch, key: id, values_only }
}

impl Grid3dRow for Grid3dRule {
    fn insert_at(items: &[Self], row: &Self) -> usize {
        items.iter().position(|item| item.id >= row.id).unwrap_or(items.len())
    }
}

wfc_patch!(/// ⛓️ Field patch over an adjacency rule (its id is the row identity).
    Grid3dRulePatch for Grid3dRule { plain { tile_a_id: String, tile_b_id: String, direction: Grid3dDirection, allowed: bool } optional {  } });

protocol::list_delta! {
    /// 📂 Row delta over the rules.
    pub Grid3dRulesDelta { removal: Grid3dRulesRemoval, insertion: Grid3dRulesInsertion, relocation: Grid3dRulesRelocation, modification: Grid3dRulesModification, row: Grid3dRule, patch: Grid3dRulePatch, key: id, values_only }
}

impl Grid3dRow for Grid3dPinnedCell {
    fn insert_at(items: &[Self], row: &Self) -> usize {
        items.iter().position(|item| cell_key(item.x, item.y, item.z) >= cell_key(row.x, row.y, row.z)).unwrap_or(items.len())
    }
}

wfc_patch!(/// 📌️ Field patch over a pinned cell (its `x:y:z` key is the row identity).
    Grid3dPinnedPatch for Grid3dPinnedCell { plain { tile_id: String } optional {  } });

protocol::list_delta! {
    /// 📂 Row delta over the pinned.
    pub Grid3dPinnedDelta { removal: Grid3dPinnedRemoval, insertion: Grid3dPinnedInsertion, relocation: Grid3dPinnedRelocation, modification: Grid3dPinnedModification, row: Grid3dPinnedCell, patch: Grid3dPinnedPatch, list: Vec<Grid3dPinnedCell>, key: String = |row| cell_key(row.x, row.y, row.z), values_only }
}

impl Grid3dRow for Grid3dCell {
    fn insert_at(items: &[Self], row: &Self) -> usize {
        items.iter().position(|item| cell_key(item.x, item.y, item.z) >= cell_key(row.x, row.y, row.z)).unwrap_or(items.len())
    }
}

wfc_patch!(/// 🕳️ Field patch over a masked cell (a masked cell has no field besides its coordinates).
    Grid3dMaskedPatch for Grid3dCell { plain {  } optional {  } });

protocol::list_delta! {
    /// 📂 Row delta over the masked.
    pub Grid3dMaskedDelta { removal: Grid3dMaskedRemoval, insertion: Grid3dMaskedInsertion, relocation: Grid3dMaskedRelocation, modification: Grid3dMaskedModification, row: Grid3dCell, patch: Grid3dMaskedPatch, list: Vec<Grid3dCell>, key: String = |row| cell_key(row.x, row.y, row.z), values_only }
}

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
    fn apply(&self, base: &Grid3dSnapshot, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<Grid3dSnapshot> {
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
        next.tiles = self.tiles.commit_onto(&base.tiles, capability).map_err(|error| error.under(["tiles"]))?;
        next.rules = self.rules.commit_onto(&base.rules, capability).map_err(|error| error.under(["rules"]))?;
        next.pinned = self.pinned.commit_onto(&base.pinned, capability).map_err(|error| error.under(["pinned"]))?;
        next.masked = self.masked.commit_onto(&base.masked, capability).map_err(|error| error.under(["masked"]))?;
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
