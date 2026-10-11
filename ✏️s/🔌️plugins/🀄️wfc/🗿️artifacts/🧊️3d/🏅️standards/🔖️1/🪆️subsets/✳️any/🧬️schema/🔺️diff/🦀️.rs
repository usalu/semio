//! 🔺️ `Wfc3dDiff` — a field-sparse, id-keyed delta over `Wfc3dSnapshot`: scalars are optional absolute values, every list carries removed identities, added rows
//! (landing at their canonical position) and per-row field patches — never a whole row or list copy. `absorb` coalesces per id and per field, `inverse` restores exact base values.

use crate::schema::snapshot::{GraphRule, Slot3d, SlotEdge, Tile, TileMedia3d, Wfc3dSnapshot};
use ::semio_framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Rows
/// 📍️ The canonical position a new row of a list lands at in the after list.
pub trait Wfc3dRow: Clone + PartialEq {
    fn insert_at(items: &[Self], row: &Self) -> usize;
}
//#endregion 🔖️Rows

//#region 🔖️Optionals
/// 🔤 An optional String field set to a value or cleared.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", default)]
pub struct Wfc3dOptionalText {
    pub value: Option<String>,
}
//#endregion 🔖️Optionals

//#region 🔖️PatchMacro
/// 🩹 Declares a field-sparse patch struct for `$row` and its `protocol::list_delta::RowPatch` impl: `plain` fields set a value, `optional` fields set or clear an `Option` through their wrapper.
macro_rules! wfc_patch {
    ($(#[$doc:meta])* $name:ident for $row:ty { plain { $($field:ident : $ty:ty),* } optional { $($ofield:ident : $wrap:ident),* } }) => {
        $(#[$doc])*
        #[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, protocol::__value_derive::RetireOwned)]
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
impl Wfc3dRow for Slot3d {
    fn insert_at(items: &[Self], row: &Self) -> usize {
        items.iter().position(|item| item.id > row.id).unwrap_or(items.len())
    }
}

wfc_patch!(/// 📍 Field patch over a slot (its id is the row identity).
    Wfc3dSlotPatch for Slot3d { plain { x: f64, y: f64, z: f64, width: f64, height: f64, depth: f64 } optional { pinned_tile_id: Wfc3dOptionalText } });

protocol::list_delta! {
    /// 📂 Row delta over the slots.
    pub Wfc3dSlotsDelta { removal: Wfc3dSlotsRemoval, insertion: Wfc3dSlotsInsertion, relocation: Wfc3dSlotsRelocation, modification: Wfc3dSlotsModification, row: Slot3d, patch: Wfc3dSlotPatch, key: id, values_only }
}

impl Wfc3dRow for SlotEdge {
    fn insert_at(items: &[Self], row: &Self) -> usize {
        items.iter().position(|item| item.id > row.id).unwrap_or(items.len())
    }
}

wfc_patch!(/// 🔗 Field patch over an adjacency edge (its id is the row identity).
    Wfc3dEdgePatch for SlotEdge { plain { from_slot_id: String, to_slot_id: String, relation: String } optional {  } });

protocol::list_delta! {
    /// 📂 Row delta over the edges.
    pub Wfc3dEdgesDelta { removal: Wfc3dEdgesRemoval, insertion: Wfc3dEdgesInsertion, relocation: Wfc3dEdgesRelocation, modification: Wfc3dEdgesModification, row: SlotEdge, patch: Wfc3dEdgePatch, key: id, values_only }
}

impl Wfc3dRow for Tile {
    fn insert_at(items: &[Self], row: &Self) -> usize {
        items.iter().position(|item| item.id > row.id).unwrap_or(items.len())
    }
}

wfc_patch!(/// 🀄️ Field patch over a tile (its id is the row identity).
    Wfc3dTilePatch for Tile { plain { weight: f64, media: TileMedia3d } optional { label: Wfc3dOptionalText } });

protocol::list_delta! {
    /// 📂 Row delta over the tiles.
    pub Wfc3dTilesDelta { removal: Wfc3dTilesRemoval, insertion: Wfc3dTilesInsertion, relocation: Wfc3dTilesRelocation, modification: Wfc3dTilesModification, row: Tile, patch: Wfc3dTilePatch, key: id, values_only }
}

impl Wfc3dRow for GraphRule {
    fn insert_at(items: &[Self], row: &Self) -> usize {
        items.iter().position(|item| item.id > row.id).unwrap_or(items.len())
    }
}

wfc_patch!(/// ⛓️ Field patch over an adjacency rule (its id is the row identity).
    Wfc3dRulePatch for GraphRule { plain { tile_a_id: String, tile_b_id: String, allowed: bool } optional { relation: Wfc3dOptionalText } });

protocol::list_delta! {
    /// 📂 Row delta over the rules.
    pub Wfc3dRulesDelta { removal: Wfc3dRulesRemoval, insertion: Wfc3dRulesInsertion, relocation: Wfc3dRulesRelocation, modification: Wfc3dRulesModification, row: GraphRule, patch: Wfc3dRulePatch, key: id, values_only }
}

//#endregion 🔖️RowTypes

//#region 🔖️Diff
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, ArtifactSchema, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.wfc.wfc3d")]
pub struct Wfc3dDiff {
    #[state(artifact)]
    pub schema: Option<String>,
    #[state(artifact)]
    pub seed: Option<u64>,
    #[state(artifact)]
    pub slots: Wfc3dSlotsDelta,
    #[state(artifact)]
    pub edges: Wfc3dEdgesDelta,
    #[state(artifact)]
    pub tiles: Wfc3dTilesDelta,
    #[state(artifact)]
    pub rules: Wfc3dRulesDelta,
}
//#endregion 🔖️Diff

//#region 🔖️Apply
impl protocol::MutationDiff<Wfc3dSnapshot> for Wfc3dDiff {
    fn apply(&self, base: &Wfc3dSnapshot, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<Wfc3dSnapshot> {
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

impl protocol::DiffAlgebra<Wfc3dSnapshot> for Wfc3dDiff {
    fn inverse(&self, base: &Wfc3dSnapshot) -> Self {
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
