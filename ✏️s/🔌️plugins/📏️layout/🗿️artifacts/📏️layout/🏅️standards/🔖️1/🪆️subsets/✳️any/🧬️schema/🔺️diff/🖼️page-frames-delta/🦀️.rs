//! 🖼️ Page frames delta of the layout diff: the keyed frame rows of a page and their delta algebra. See [`super`].

use super::*;

//#region 🔖️DeltaHelpers
protocol::row_patch! {
    /// 🗂️ Owned-field patch of one layer: its name and the two flags.
    pub LayerPatch of Layer { set { name: String, visible: bool, locked: bool } }
}

protocol::list_delta! {
    /// 🧩 Positional keyed rows of the `layers` list.
    pub PageLayersDelta {
        removal: PageLayerRemoval,
        insertion: PageLayerInsertion,
        relocation: PageLayerRelocation,
        modification: PageLayersModification,
        row: Layer,
        patch: LayerPatch,
        key: id
    }
}

/// ➖️ One `frames` row removed, with the base index the inverse reinserts it at.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct PageFrameRemoval {
    pub id: String,
    pub index: usize,
}

/// ➕️ One `frames` row inserted at its index in the resulting list.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct PageFrameInsertion {
    pub index: usize,
    #[dsl(statements)]
    pub row: Frame,
}

/// ↕️ One `frames` row moved from its base index to its index in the resulting list.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct PageFrameRelocation {
    pub id: String,
    pub from: usize,
    pub to: usize,
}

/// 🩹 One modified `frames` row.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct PageFramesModification {
    pub id: String,
    pub patch: FramePatch,
}

/// 🧩 Positional keyed rows of the `frames` list.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", default)]
pub struct PageFramesDelta {
    pub removed: Vec<PageFrameRemoval>,
    pub inserted: Vec<PageFrameInsertion>,
    pub moved: Vec<PageFrameRelocation>,
    pub modified: Vec<PageFramesModification>,
}

type PageFramesDeltaParts = protocol::list_delta::Parts<Frame, FramePatch>;

impl PageFramesDelta {
    pub(super) fn into_parts(self) -> PageFramesDeltaParts {
        protocol::list_delta::Parts {
            removed: self.removed.into_iter().map(|entry| (entry.id, entry.index)).collect(),
            inserted: self.inserted.into_iter().map(|entry| (entry.index, entry.row)).collect(),
            moved: self.moved.into_iter().map(|entry| (entry.id, entry.from, entry.to)).collect(),
            modified: self.modified.into_iter().map(|entry| (entry.id, entry.patch)).collect(),
        }
    }

    pub(super) fn from_parts(parts: PageFramesDeltaParts) -> Self {
        Self {
            removed: parts.removed.into_iter().map(|(id, index)| PageFrameRemoval { id, index }).collect(),
            inserted: parts.inserted.into_iter().map(|(index, row)| PageFrameInsertion { index, row: row }).collect(),
            moved: parts.moved.into_iter().map(|(id, from, to)| PageFrameRelocation { id, from, to }).collect(),
            modified: parts.modified.into_iter().map(|(id, patch)| PageFramesModification { id, patch }).collect(),
        }
    }

    /// ➕️ The delta that inserts `row` at `index` of the after list.
    pub fn insertion(index: usize, row: Frame) -> Self {
        Self::from_parts(protocol::list_delta::Parts::insertion(index, row))
    }

    /// ➖️ The delta that removes the row `id` found at `index` of the base list.
    pub fn removal_by_id(id: impl Into<String>, index: usize) -> Self {
        Self::from_parts(protocol::list_delta::Parts::removal_by_id(id.into(), index))
    }

    /// ↕️ The delta that moves the row `id` from `from` of the base list to `to` of the after list.
    pub fn relocation_by_id(id: impl Into<String>, from: usize, to: usize) -> Self {
        Self::from_parts(protocol::list_delta::Parts::relocation_by_id(id.into(), from, to))
    }

    /// 🩹 The delta that patches the row `id`.
    pub fn modification(id: impl Into<String>, patch: FramePatch) -> Self {
        Self::from_parts(protocol::list_delta::Parts::modification(id.into(), patch))
    }

    /// ✍️ The list this delta turns `base` into; reached only from the diff's own `apply`, under the central applier's capability.
    pub fn commit_onto(&self, base: &Vec<Frame>, capability: ApplyCapability) -> Result<Vec<Frame>, protocol::list_delta::ApplyError> {
        self.clone().into_parts().commit_onto(base, capability)
    }

    /// 🔁️ The negative delta over `base`, read row by row.
    pub fn inverse(&self, base: &Vec<Frame>) -> Self {
        Self::from_parts(self.clone().into_parts().inverse(base))
    }

    /// ➕️ Composes `self` with the delta `later` applied after it.
    pub fn absorb(&mut self, later: Self) {
        let mut parts = std::mem::take(self).into_parts();
        parts.absorb(later.into_parts());
        *self = Self::from_parts(parts);
    }

    /// 🕳️ Whether the delta changes nothing.
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.inserted.is_empty() && self.moved.is_empty() && self.modified.iter().all(|entry| RowPatch::<Frame>::is_empty(&entry.patch))
    }
}
//#endregion 🔖️DeltaHelpers
