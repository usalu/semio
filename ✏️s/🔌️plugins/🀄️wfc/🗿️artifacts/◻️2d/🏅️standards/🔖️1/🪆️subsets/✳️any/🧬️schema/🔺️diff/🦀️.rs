//! 🔺️ `Wfc2dDiff` — a field-sparse, id-keyed delta over `Wfc2dSnapshot`: scalars are optional absolute values, every list carries removed identities, added rows
//! (landing at their canonical position) and per-row field patches — never a whole row or list copy. `absorb` coalesces per id and per field, `inverse` restores exact base values.

use crate::schema::snapshot::{Wfc2dRule, Wfc2dSlot, Wfc2dSlotEdge, Wfc2dSnapshot, Wfc2dTile, Wfc2dTileMedia};
use ::semio_framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Rows
/// 🔑️ A row of an id-keyed list kept in canonical order: `row_key` addresses it, `insert_at` is where a new row lands.
pub trait Wfc2dRow: Clone + PartialEq {
    fn row_key(&self) -> String;
    fn insert_at(items: &[Self], row: &Self) -> usize;
}

/// 🩹 Field-sparse patch over one row: names the fields it sets with the values they take.
pub trait Wfc2dPatch: Clone + Default + PartialEq {
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
pub struct Wfc2dRowPatch<Q> {
    pub id: String,
    pub patch: Q,
}

/// 📂 Id-keyed row delta: removed identities, added rows (landing at their canonical position) and per-row field patches.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct Wfc2dRows<T, Q> {
    #[value(default)]
    pub removed: Vec<String>,
    #[value(default)]
    pub added: Vec<T>,
    #[value(default)]
    pub patched: Vec<Wfc2dRowPatch<Q>>,
}

impl<T, Q> Default for Wfc2dRows<T, Q> {
    fn default() -> Self {
        Self { removed: Vec::new(), added: Vec::new(), patched: Vec::new() }
    }
}

fn rejection(code: &'static str, message: &'static str, lane: &str, id: &str) -> protocol::MutationApplyError {
    protocol::MutationApplyError::new(code, message).at([lane, id])
}

impl<T: Wfc2dRow, Q: Wfc2dPatch<Row = T>> Wfc2dRows<T, Q> {
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
        let patched: Vec<Wfc2dRowPatch<Q>> = self
            .patched
            .iter()
            .filter(|entry| !self.removed.contains(&entry.id) && !self.added.iter().any(|row| row.row_key() == entry.id))
            .filter_map(|entry| find(&entry.id).map(|row| Wfc2dRowPatch { id: entry.id.clone(), patch: entry.patch.inverse(row) }))
            .filter(|entry| !entry.patch.is_empty())
            .collect();
        Self { removed, added, patched }
    }

    /// 🧭️ The delta that turns `from` into `to`.
    pub fn between(from: &[T], to: &[T]) -> Self {
        let removed: Vec<String> = from.iter().map(T::row_key).filter(|key| !to.iter().any(|row| row.row_key() == *key)).collect();
        let added: Vec<T> = to.iter().filter(|row| !from.iter().any(|other| other.row_key() == row.row_key())).cloned().collect();
        let patched: Vec<Wfc2dRowPatch<Q>> = from
            .iter()
            .filter_map(|row| to.iter().find(|other| other.row_key() == row.row_key()).filter(|other| *other != row).map(|other| Wfc2dRowPatch { id: row.row_key(), patch: Q::between(row, other) }))
            .collect();
        Self { removed, added, patched }
    }
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
/// 🩹 Declares a field-sparse patch struct for `$row` and its [`Wfc2dPatch`] impl: `plain` fields set a value, `optional` fields set or clear an `Option` through their wrapper.
macro_rules! wfc_patch {
    ($(#[$doc:meta])* $name:ident for $row:ty { plain { $($field:ident : $ty:ty),* } optional { $($ofield:ident : $wrap:ident),* } }) => {
        $(#[$doc])*
        #[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
        #[value(rename_all = "camelCase", default)]
        pub struct $name {
            $(pub $field: Option<$ty>,)*
            $(pub $ofield: Option<$wrap>,)*
        }

        impl Wfc2dPatch for $name {
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

//#region 🔖️RowTypes
impl Wfc2dRow for Wfc2dSlot {
    fn row_key(&self) -> String {
        self.id.clone()
    }
    fn insert_at(items: &[Self], row: &Self) -> usize {
        items.iter().position(|item| item.id > row.id).unwrap_or(items.len())
    }
}

wfc_patch!(/// 📍 Field patch over a slot (its id is the row identity).
    Wfc2dSlotPatch for Wfc2dSlot { plain { x: f64, y: f64, width: f64, height: f64 } optional { pinned_tile_id: Wfc2dOptionalText } });

/// 📂 Row delta over the slots.
pub type Wfc2dSlotsDelta = Wfc2dRows<Wfc2dSlot, Wfc2dSlotPatch>;

impl Wfc2dRow for Wfc2dSlotEdge {
    fn row_key(&self) -> String {
        self.id.clone()
    }
    fn insert_at(items: &[Self], row: &Self) -> usize {
        items.iter().position(|item| item.id > row.id).unwrap_or(items.len())
    }
}

wfc_patch!(/// 🔗 Field patch over an adjacency edge (its id is the row identity).
    Wfc2dEdgePatch for Wfc2dSlotEdge { plain { from_slot_id: String, to_slot_id: String, relation: String } optional {  } });

/// 📂 Row delta over the edges.
pub type Wfc2dEdgesDelta = Wfc2dRows<Wfc2dSlotEdge, Wfc2dEdgePatch>;

impl Wfc2dRow for Wfc2dTile {
    fn row_key(&self) -> String {
        self.id.clone()
    }
    fn insert_at(items: &[Self], row: &Self) -> usize {
        items.iter().position(|item| item.id > row.id).unwrap_or(items.len())
    }
}

wfc_patch!(/// 🀄️ Field patch over a tile (its id is the row identity).
    Wfc2dTilePatch for Wfc2dTile { plain { weight: f64, media: Wfc2dTileMedia } optional { label: Wfc2dOptionalText } });

/// 📂 Row delta over the tiles.
pub type Wfc2dTilesDelta = Wfc2dRows<Wfc2dTile, Wfc2dTilePatch>;

impl Wfc2dRow for Wfc2dRule {
    fn row_key(&self) -> String {
        self.id.clone()
    }
    fn insert_at(items: &[Self], row: &Self) -> usize {
        items.iter().position(|item| item.id > row.id).unwrap_or(items.len())
    }
}

wfc_patch!(/// ⛓️ Field patch over an adjacency rule (its id is the row identity).
    Wfc2dRulePatch for Wfc2dRule { plain { tile_a_id: String, tile_b_id: String, allowed: bool } optional { relation: Wfc2dOptionalText } });

/// 📂 Row delta over the rules.
pub type Wfc2dRulesDelta = Wfc2dRows<Wfc2dRule, Wfc2dRulePatch>;

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
    fn apply(&self, base: &Wfc2dSnapshot, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<Wfc2dSnapshot> {
        let mut next = base.clone();
        if let Some(schema) = &self.schema {
            next.schema.clone_from(schema);
        }
        if let Some(seed) = self.seed {
            next.seed = seed;
        }
        next.slots = self.slots.apply(&base.slots).map_err(|error| error.under(["slots"]))?;
        next.edges = self.edges.apply(&base.edges).map_err(|error| error.under(["edges"]))?;
        next.tiles = self.tiles.apply(&base.tiles).map_err(|error| error.under(["tiles"]))?;
        next.rules = self.rules.apply(&base.rules).map_err(|error| error.under(["rules"]))?;
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
    fn between(base: &Wfc2dSnapshot, other: &Wfc2dSnapshot) -> Self {
        Self {
            schema: (base.schema != other.schema).then(|| other.schema.clone()),
            seed: (base.seed != other.seed).then_some(other.seed),
            slots: Wfc2dRows::between(&base.slots, &other.slots),
            edges: Wfc2dRows::between(&base.edges, &other.edges),
            tiles: Wfc2dRows::between(&base.tiles, &other.tiles),
            rules: Wfc2dRows::between(&base.rules, &other.rules),
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
