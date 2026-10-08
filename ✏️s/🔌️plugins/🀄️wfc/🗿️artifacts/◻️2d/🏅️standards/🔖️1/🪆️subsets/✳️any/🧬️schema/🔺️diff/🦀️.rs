//! 🔺️ `Wfc2dDiff` — a field-sparse, id-keyed delta over `Wfc2dSnapshot`: scalars are optional absolute values, every list carries removed identities, added rows
//! (landing at their canonical position) and per-row field patches — never a whole row or list copy. `absorb` coalesces per id and per field, `inverse` restores exact base values.

use crate::schema::snapshot::{Wfc2dRule, Wfc2dSlot, Wfc2dSlotEdge, Wfc2dSnapshot, Wfc2dTile, Wfc2dTileMedia};
use ::semio_framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Rows
/// 📍️ The canonical position a new row of a list lands at in the after list.
pub trait Wfc2dRow: Clone + PartialEq {
    fn insert_at(items: &[Self], row: &Self) -> usize;
}
//#endregion 🔖️Rows

//#region 🔖️Optionals
/// 🔤 An optional String field set to a value or cleared.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct Wfc2dOptionalText {
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

//#region 🔖️RowTypes
impl Wfc2dRow for Wfc2dSlot {
    fn insert_at(items: &[Self], row: &Self) -> usize {
        items.iter().position(|item| item.id > row.id).unwrap_or(items.len())
    }
}

wfc_patch!(/// 📍 Field patch over a slot (its id is the row identity).
    Wfc2dSlotPatch for Wfc2dSlot { plain { x: f64, y: f64, width: f64, height: f64 } optional { pinned_tile_id: Wfc2dOptionalText } });

protocol::list_delta! {
    /// 📂 Row delta over the slots.
    pub Wfc2dSlotsDelta { removal: Wfc2dSlotsRemoval, insertion: Wfc2dSlotsInsertion, relocation: Wfc2dSlotsRelocation, modification: Wfc2dSlotsModification, row: Wfc2dSlot, patch: Wfc2dSlotPatch, key: id, values_only }
}

impl Wfc2dRow for Wfc2dSlotEdge {
    fn insert_at(items: &[Self], row: &Self) -> usize {
        items.iter().position(|item| item.id > row.id).unwrap_or(items.len())
    }
}

wfc_patch!(/// 🔗 Field patch over an adjacency edge (its id is the row identity).
    Wfc2dEdgePatch for Wfc2dSlotEdge { plain { from_slot_id: String, to_slot_id: String, relation: String } optional {  } });

protocol::list_delta! {
    /// 📂 Row delta over the edges.
    pub Wfc2dEdgesDelta { removal: Wfc2dEdgesRemoval, insertion: Wfc2dEdgesInsertion, relocation: Wfc2dEdgesRelocation, modification: Wfc2dEdgesModification, row: Wfc2dSlotEdge, patch: Wfc2dEdgePatch, key: id, values_only }
}

impl Wfc2dRow for Wfc2dTile {
    fn insert_at(items: &[Self], row: &Self) -> usize {
        items.iter().position(|item| item.id > row.id).unwrap_or(items.len())
    }
}

wfc_patch!(/// 🀄️ Field patch over a tile (its id is the row identity).
    Wfc2dTilePatch for Wfc2dTile { plain { weight: f64, media: Wfc2dTileMedia } optional { label: Wfc2dOptionalText } });

protocol::list_delta! {
    /// 📂 Row delta over the tiles.
    pub Wfc2dTilesDelta { removal: Wfc2dTilesRemoval, insertion: Wfc2dTilesInsertion, relocation: Wfc2dTilesRelocation, modification: Wfc2dTilesModification, row: Wfc2dTile, patch: Wfc2dTilePatch, key: id, values_only }
}

impl Wfc2dRow for Wfc2dRule {
    fn insert_at(items: &[Self], row: &Self) -> usize {
        items.iter().position(|item| item.id > row.id).unwrap_or(items.len())
    }
}

wfc_patch!(/// ⛓️ Field patch over an adjacency rule (its id is the row identity).
    Wfc2dRulePatch for Wfc2dRule { plain { tile_a_id: String, tile_b_id: String, allowed: bool } optional { relation: Wfc2dOptionalText } });

protocol::list_delta! {
    /// 📂 Row delta over the rules.
    pub Wfc2dRulesDelta { removal: Wfc2dRulesRemoval, insertion: Wfc2dRulesInsertion, relocation: Wfc2dRulesRelocation, modification: Wfc2dRulesModification, row: Wfc2dRule, patch: Wfc2dRulePatch, key: id, values_only }
}

//#endregion 🔖️RowTypes

//#region 🔖️Diff
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.wfc.wfc2d")]
pub struct Wfc2dDiff {
    #[state(artifact)]
    pub schema: Option<String>,
    #[state(artifact)]
    pub seed: Option<u64>,
    #[state(artifact)]
    pub slots: Wfc2dSlotsDelta,
    #[state(artifact)]
    pub edges: Wfc2dEdgesDelta,
    #[state(artifact)]
    pub tiles: Wfc2dTilesDelta,
    #[state(artifact)]
    pub rules: Wfc2dRulesDelta,
}
//#endregion 🔖️Diff

//#region 🔖️Apply
impl protocol::MutationDiff<Wfc2dSnapshot> for Wfc2dDiff {
    fn apply(&self, base: &Wfc2dSnapshot, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<Wfc2dSnapshot> {
        let mut next = base.clone();
        if let Some(schema) = &self.schema {
            next.schema.clone_from(schema);
        }
        if let Some(seed) = self.seed {
            next.seed = seed;
        }
        next.slots = self.slots.commit_onto(&base.slots, capability).map_err(|error| error.under(["slots"]))?;
        next.edges = self.edges.commit_onto(&base.edges, capability).map_err(|error| error.under(["edges"]))?;
        next.tiles = self.tiles.commit_onto(&base.tiles, capability).map_err(|error| error.under(["tiles"]))?;
        next.rules = self.rules.commit_onto(&base.rules, capability).map_err(|error| error.under(["rules"]))?;
        Ok(next)
    }
    fn absorb(&mut self, later: Self) {
        if later.schema.is_some() {
            self.schema = later.schema;
        }
        if later.seed.is_some() {
            self.seed = later.seed;
        }
        self.slots.absorb(later.slots);
        self.edges.absorb(later.edges);
        self.tiles.absorb(later.tiles);
        self.rules.absorb(later.rules);
    }
}

impl protocol::DiffAlgebra<Wfc2dSnapshot> for Wfc2dDiff {
    fn inverse(&self, base: &Wfc2dSnapshot) -> Self {
        Self {
            schema: self.schema.as_ref().map(|_| base.schema.clone()),
            seed: self.seed.map(|_| base.seed),
            slots: self.slots.inverse(&base.slots),
            edges: self.edges.inverse(&base.edges),
            tiles: self.tiles.inverse(&base.tiles),
            rules: self.rules.inverse(&base.rules),
        }
    }
    fn is_empty(&self) -> bool {
        self.schema.is_none()
            && self.seed.is_none()
            && self.slots.is_empty()
            && self.edges.is_empty()
            && self.tiles.is_empty()
            && self.rules.is_empty()
    }
}
//#endregion 🔖️Apply

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
