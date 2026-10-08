//! 🔺️ `Grid2dDiff` — a field-sparse, id-keyed delta over `Grid2dSnapshot`: scalars are optional absolute values, every list carries removed identities, added rows
//! (landing at their canonical position) and per-row field patches — never a whole row or list copy. `absorb` coalesces per id and per field, `inverse` restores exact base values.

use crate::schema::snapshot::{Grid2dSnapshot, WfcAdjacencyRule2d, WfcCell2d, WfcDirection2d, WfcPinnedCell2d, WfcTile2d, WfcTileMedia2d};
use ::semio_framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Rows
/// 📍️ The canonical position a new row of a list lands at in the after list.
pub trait Grid2dRow: Clone + PartialEq {
    fn insert_at(items: &[Self], row: &Self) -> usize;
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

//#region 🔖️Keys
/// 🔑️ The removal key of a grid cell — `"<x>,<y>"`, the same spelling both cell collections use.
pub fn cell_id(x: u32, y: u32) -> String {
    format!("{x},{y}")
}
//#endregion 🔖️Keys

//#region 🔖️RowTypes
impl Grid2dRow for WfcTile2d {
    fn insert_at(items: &[Self], row: &Self) -> usize {
        items.iter().position(|item| item.id > row.id).unwrap_or(items.len())
    }
}

wfc_patch!(/// 🀄️ Field patch over a tile (its id is the row identity).
    Grid2dTilePatch for WfcTile2d { plain { weight: f64, media: WfcTileMedia2d } optional { label: Grid2dOptionalText } });

protocol::list_delta! {
    /// 📂 Row delta over the tiles.
    pub Grid2dTilesDelta { removal: Grid2dTilesRemoval, insertion: Grid2dTilesInsertion, relocation: Grid2dTilesRelocation, modification: Grid2dTilesModification, row: WfcTile2d, patch: Grid2dTilePatch, key: id, values_only }
}

impl Grid2dRow for WfcAdjacencyRule2d {
    fn insert_at(items: &[Self], row: &Self) -> usize {
        items.iter().position(|item| item.id > row.id).unwrap_or(items.len())
    }
}

wfc_patch!(/// ⛓️ Field patch over an adjacency rule (its id is the row identity).
    Grid2dRulePatch for WfcAdjacencyRule2d { plain { tile_a_id: String, tile_b_id: String, direction: WfcDirection2d, allowed: bool } optional {  } });

protocol::list_delta! {
    /// 📂 Row delta over the rules.
    pub Grid2dRulesDelta { removal: Grid2dRulesRemoval, insertion: Grid2dRulesInsertion, relocation: Grid2dRulesRelocation, modification: Grid2dRulesModification, row: WfcAdjacencyRule2d, patch: Grid2dRulePatch, key: id, values_only }
}

impl Grid2dRow for WfcPinnedCell2d {
    fn insert_at(items: &[Self], row: &Self) -> usize {
        items.iter().position(|item| (item.y, item.x) > (row.y, row.x)).unwrap_or(items.len())
    }
}

wfc_patch!(/// 📌️ Field patch over a pinned cell (its `x,y` is the row identity).
    Grid2dPinnedPatch for WfcPinnedCell2d { plain { tile_id: String } optional {  } });

protocol::list_delta! {
    /// 📂 Row delta over the pinned.
    pub Grid2dPinnedDelta { removal: Grid2dPinnedRemoval, insertion: Grid2dPinnedInsertion, relocation: Grid2dPinnedRelocation, modification: Grid2dPinnedModification, row: WfcPinnedCell2d, patch: Grid2dPinnedPatch, list: Vec<WfcPinnedCell2d>, key: String = |row| format!("{},{}", row.x, row.y), values_only }
}

impl Grid2dRow for WfcCell2d {
    fn insert_at(items: &[Self], row: &Self) -> usize {
        items.iter().position(|item| (item.y, item.x) > (row.y, row.x)).unwrap_or(items.len())
    }
}

wfc_patch!(/// 🕳️ Field patch over a masked cell (a masked cell has no field besides its coordinates).
    Grid2dMaskedPatch for WfcCell2d { plain {  } optional {  } });

protocol::list_delta! {
    /// 📂 Row delta over the masked.
    pub Grid2dMaskedDelta { removal: Grid2dMaskedRemoval, insertion: Grid2dMaskedInsertion, relocation: Grid2dMaskedRelocation, modification: Grid2dMaskedModification, row: WfcCell2d, patch: Grid2dMaskedPatch, list: Vec<WfcCell2d>, key: String = |row| format!("{},{}", row.x, row.y), values_only }
}

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
    fn apply(&self, base: &Grid2dSnapshot, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<Grid2dSnapshot> {
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
