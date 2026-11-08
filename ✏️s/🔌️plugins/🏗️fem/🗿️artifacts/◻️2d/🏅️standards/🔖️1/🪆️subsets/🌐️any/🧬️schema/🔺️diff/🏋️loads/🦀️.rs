//! 🏋️ Fem2d load-case patch and the positional keyed delta of its `loads` rows.

use super::*;

/// 🌾️ Owned-field patch of one load case: a rename, a self-weight switch and/or keyed load rows.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", default)]
pub struct Fem2dLoadCasePatch {
    pub name: Option<String>,
    pub self_weight: Option<bool>,
    pub loads: Option<Fem2dLoadsDelta>,
}

impl protocol::list_delta::Keyed for FemLoad {
    type Key = String;
    fn key(&self) -> String {
        load_id(self).to_string()
    }
}

/// 🏋️ One `loads` row removed, with the base index the inverse reinserts it at.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct Fem2dLoadRemoval {
    pub id: String,
    pub index: usize,
}

/// 📥️ One `loads` row inserted at its index in the resulting list.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct Fem2dLoadInsertion {
    pub index: usize,
    #[dsl(statements)]
    pub row: FemLoad,
}

/// 🚚️ One `loads` row moved from its base index to its index in the resulting list.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct Fem2dLoadRelocation {
    pub id: String,
    pub from: usize,
    pub to: usize,
}

/// 🔄️ One modified `loads` row (whole-row replacement).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct Fem2dLoadsModification {
    pub id: String,
    pub patch: Fem2dLoadPatch,
}

/// 🌂️ Positional keyed delta of the `loads` list.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", default)]
pub struct Fem2dLoadsDelta {
    pub removed: Vec<Fem2dLoadRemoval>,
    pub inserted: Vec<Fem2dLoadInsertion>,
    pub moved: Vec<Fem2dLoadRelocation>,
    pub modified: Vec<Fem2dLoadsModification>,
}

type Fem2dLoadsDeltaParts = protocol::list_delta::Parts<FemLoad, Fem2dLoadPatch>;

impl Fem2dLoadsDelta {
    /// 🧊️ Disposes displaced or abandoned load lists through their typed row owner.
    pub fn retire_list(rows: Vec<FemLoad>) { rows.into_iter().for_each(protocol::list_delta::Keyed::retire_cold); }
    fn into_parts(self) -> Fem2dLoadsDeltaParts {
        protocol::list_delta::Parts {
            removed: self.removed.into_iter().map(|entry| (entry.id, entry.index)).collect(),
            inserted: self.inserted.into_iter().map(|entry| (entry.index, entry.row)).collect(),
            moved: self.moved.into_iter().map(|entry| (entry.id, entry.from, entry.to)).collect(),
            modified: self.modified.into_iter().map(|entry| (entry.id, entry.patch)).collect(),
        }
    }

    fn from_parts(parts: Fem2dLoadsDeltaParts) -> Self {
        Self {
            removed: parts.removed.into_iter().map(|(id, index)| Fem2dLoadRemoval { id, index }).collect(),
            inserted: parts.inserted.into_iter().map(|(index, row)| Fem2dLoadInsertion { index, row }).collect(),
            moved: parts.moved.into_iter().map(|(id, from, to)| Fem2dLoadRelocation { id, from, to }).collect(),
            modified: parts.modified.into_iter().map(|(id, patch)| Fem2dLoadsModification { id, patch }).collect(),
        }
    }

    /// 🏆️ The delta that removes the row `id` found at `index` of the base list.
    pub fn removal_by_id(id: impl Into<String>, index: usize) -> Self {
        Self::from_parts(protocol::list_delta::Parts::removal_by_id(id.into(), index))
    }

    /// 🔟️ The list this delta turns `base` into; reached only from the diff's own `apply`, under the central applier's capability.
    pub fn commit_onto(&self, base: &Vec<FemLoad>, capability: ApplyCapability) -> Result<Vec<FemLoad>, protocol::list_delta::ApplyError> {
        self.clone().into_parts().commit_onto(base, capability)
    }

    /// 🌃️ Composes `self` with the delta `later` applied after it.
    pub fn absorb(&mut self, later: Self) {
        let mut parts = std::mem::take(self).into_parts();
        parts.absorb(later.into_parts());
        *self = Self::from_parts(parts);
    }

    /// 🌄️ The negative delta over `base`, read row by row.
    pub fn inverse(&self, base: &Vec<FemLoad>) -> Self {
        Self::from_parts(self.clone().into_parts().inverse(base))
    }

    /// 🌅️ Whether the delta changes nothing.
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.inserted.is_empty() && self.moved.is_empty() && self.modified.iter().all(|entry| RowPatch::<FemLoad>::is_empty(&entry.patch))
    }
}

protocol::list_delta! {
    /// 🌆️ Positional keyed rows of the `combinations` list.
    pub Fem2dCombinationsDelta {
        removal: Fem2dCombinationRemoval,
        insertion: Fem2dCombinationInsertion,
        relocation: Fem2dCombinationRelocation,
        modification: Fem2dCombinationsModification,
        row: FemCombination,
        patch: Fem2dCombinationPatch,
        key: id
    }
}
