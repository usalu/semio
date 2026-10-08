//! 🧬️ Fem2d diff schema — positional keyed deltas (framework `protocol::list_delta`) over the artifact.

use crate::{FemAnalysisSettings, FemCombination, FemElement, FemLoad, FemLoadCase, FemMaterial, FemNode, FemRegion, FemSection, FemSupport};
use crate::{element_id, load_id, Fem2dSnapshot};
use ::semio_framework_schema::ArtifactSchema;
use protocol::list_delta::RowPatch;
use protocol::{ApplyCapability, DiffAlgebra, MutationApplyError, MutationApplyResult, MutationDiff};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Diff
/// 🔺️ Sparse delta for the fem2d artifact: per-collection id-keyed rows plus an owned-field analysis patch.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, ArtifactSchema, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.fem.fem2d")]
pub struct Fem2dDiff {
    #[state(artifact)]
    pub nodes: Option<Fem2dNodesDelta>,
    #[state(artifact)]
    pub elements: Option<Fem2dElementsDelta>,
    #[state(artifact)]
    pub regions: Option<Fem2dRegionsDelta>,
    #[state(artifact)]
    pub materials: Option<Fem2dMaterialsDelta>,
    #[state(artifact)]
    pub sections: Option<Fem2dSectionsDelta>,
    #[state(artifact)]
    pub supports: Option<Fem2dSupportsDelta>,
    #[state(artifact)]
    pub load_cases: Option<Fem2dLoadCasesDelta>,
    #[state(artifact)]
    pub combinations: Option<Fem2dCombinationsDelta>,
    #[state(artifact)]
    pub analysis: Option<Fem2dAnalysisPatch>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
protocol::list_delta! {
    /// 🧩 Positional keyed rows of the `nodes` list.
    pub Fem2dNodesDelta {
        removal: Fem2dNodeRemoval,
        insertion: Fem2dNodeInsertion,
        relocation: Fem2dNodeRelocation,
        modification: Fem2dNodesModification,
        row: FemNode,
        patch: FemNode,
        key: id
    }
}

impl protocol::list_delta::Keyed for FemElement {
    type Key = String;
    fn key(&self) -> String {
        element_id(self).to_string()
    }
}

/// ➖️ One `elements` row removed, with the base index the inverse reinserts it at.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct Fem2dElementRemoval {
    pub id: String,
    pub index: usize,
}

/// ➕️ One `elements` row inserted at its index in the resulting list.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct Fem2dElementInsertion {
    pub index: usize,
    #[dsl(statements)]
    pub row: FemElement,
}

/// ↕️ One `elements` row moved from its base index to its index in the resulting list.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct Fem2dElementRelocation {
    pub id: String,
    pub from: usize,
    pub to: usize,
}

/// 🩹 One modified `elements` row (whole-row replacement).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct Fem2dElementsModification {
    pub id: String,
    #[dsl(statements)]
    pub patch: Box<FemElement>,
}

/// 🧩 Positional keyed delta of the `elements` list.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct Fem2dElementsDelta {
    pub removed: Vec<Fem2dElementRemoval>,
    pub inserted: Vec<Fem2dElementInsertion>,
    pub moved: Vec<Fem2dElementRelocation>,
    pub modified: Vec<Fem2dElementsModification>,
}

type Fem2dElementsDeltaParts = protocol::list_delta::Parts<FemElement, Box<FemElement>>;

impl Fem2dElementsDelta {
    fn into_parts(self) -> Fem2dElementsDeltaParts {
        protocol::list_delta::Parts {
            removed: self.removed.into_iter().map(|entry| (entry.id, entry.index)).collect(),
            inserted: self.inserted.into_iter().map(|entry| (entry.index, entry.row)).collect(),
            moved: self.moved.into_iter().map(|entry| (entry.id, entry.from, entry.to)).collect(),
            modified: self.modified.into_iter().map(|entry| (entry.id, entry.patch)).collect(),
        }
    }

    fn from_parts(parts: Fem2dElementsDeltaParts) -> Self {
        Self {
            removed: parts.removed.into_iter().map(|(id, index)| Fem2dElementRemoval { id, index }).collect(),
            inserted: parts.inserted.into_iter().map(|(index, row)| Fem2dElementInsertion { index, row }).collect(),
            moved: parts.moved.into_iter().map(|(id, from, to)| Fem2dElementRelocation { id, from, to }).collect(),
            modified: parts.modified.into_iter().map(|(id, patch)| Fem2dElementsModification { id, patch }).collect(),
        }
    }

    /// ➖️ The delta that removes the row `id` found at `index` of the base list.
    pub fn removal_by_id(id: impl Into<String>, index: usize) -> Self {
        Self::from_parts(protocol::list_delta::Parts::removal_by_id(id.into(), index))
    }

    /// ✍️ The list this delta turns `base` into; reached only from the diff's own `apply`, under the central applier's capability.
    pub fn commit_onto(&self, base: &Vec<FemElement>, capability: protocol::ApplyCapability) -> Result<Vec<FemElement>, protocol::list_delta::ApplyError> {
        self.clone().into_parts().commit_onto(base, capability)
    }

    /// ➕️ Composes `self` with the delta `later` applied after it.
    pub fn absorb(&mut self, later: Self) {
        let mut parts = std::mem::take(self).into_parts();
        parts.absorb(later.into_parts());
        *self = Self::from_parts(parts);
    }

    /// 🔁️ The negative delta over `base`, read row by row.
    pub fn inverse(&self, base: &Vec<FemElement>) -> Self {
        Self::from_parts(self.clone().into_parts().inverse(base))
    }

    /// 🕳️ Whether the delta changes nothing.
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.inserted.is_empty() && self.moved.is_empty() && self.modified.iter().all(|entry| protocol::list_delta::RowPatch::<FemElement>::is_empty(&entry.patch))
    }
}

protocol::list_delta! {
    /// 🧩 Positional keyed rows of the `regions` list.
    pub Fem2dRegionsDelta {
        removal: Fem2dRegionRemoval,
        insertion: Fem2dRegionInsertion,
        relocation: Fem2dRegionRelocation,
        modification: Fem2dRegionsModification,
        row: FemRegion,
        patch: FemRegion,
        key: id
    }
}

protocol::list_delta! {
    /// 🧩 Positional keyed rows of the `materials` list.
    pub Fem2dMaterialsDelta {
        removal: Fem2dMaterialRemoval,
        insertion: Fem2dMaterialInsertion,
        relocation: Fem2dMaterialRelocation,
        modification: Fem2dMaterialsModification,
        row: FemMaterial,
        patch: FemMaterial,
        key: id
    }
}

protocol::list_delta! {
    /// 🧩 Positional keyed rows of the `sections` list.
    pub Fem2dSectionsDelta {
        removal: Fem2dSectionRemoval,
        insertion: Fem2dSectionInsertion,
        relocation: Fem2dSectionRelocation,
        modification: Fem2dSectionsModification,
        row: FemSection,
        patch: FemSection,
        key: id
    }
}

protocol::list_delta! {
    /// 🧩 Positional keyed rows of the `supports` list.
    pub Fem2dSupportsDelta {
        removal: Fem2dSupportRemoval,
        insertion: Fem2dSupportInsertion,
        relocation: Fem2dSupportRelocation,
        modification: Fem2dSupportsModification,
        row: FemSupport,
        patch: FemSupport,
        key: id
    }
}

protocol::list_delta! {
    /// 🧩 Positional keyed rows of the `load_cases` list.
    pub Fem2dLoadCasesDelta {
        removal: Fem2dLoadCaseRemoval,
        insertion: Fem2dLoadCaseInsertion,
        relocation: Fem2dLoadCaseRelocation,
        modification: Fem2dLoadCasesModification,
        row: FemLoadCase,
        patch: Fem2dLoadCasePatch,
        key: id
    }
}

/// 🩹 Owned-field patch of one load case: a rename, a self-weight switch and/or keyed load rows.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
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

/// ➖️ One `loads` row removed, with the base index the inverse reinserts it at.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct Fem2dLoadRemoval {
    pub id: String,
    pub index: usize,
}

/// ➕️ One `loads` row inserted at its index in the resulting list.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct Fem2dLoadInsertion {
    pub index: usize,
    #[dsl(statements)]
    pub row: FemLoad,
}

/// ↕️ One `loads` row moved from its base index to its index in the resulting list.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct Fem2dLoadRelocation {
    pub id: String,
    pub from: usize,
    pub to: usize,
}

/// 🩹 One modified `loads` row (whole-row replacement).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct Fem2dLoadsModification {
    pub id: String,
    #[dsl(statements)]
    pub patch: Box<FemLoad>,
}

/// 🧩 Positional keyed delta of the `loads` list.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct Fem2dLoadsDelta {
    pub removed: Vec<Fem2dLoadRemoval>,
    pub inserted: Vec<Fem2dLoadInsertion>,
    pub moved: Vec<Fem2dLoadRelocation>,
    pub modified: Vec<Fem2dLoadsModification>,
}

type Fem2dLoadsDeltaParts = protocol::list_delta::Parts<FemLoad, Box<FemLoad>>;

impl Fem2dLoadsDelta {
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

    /// ➖️ The delta that removes the row `id` found at `index` of the base list.
    pub fn removal_by_id(id: impl Into<String>, index: usize) -> Self {
        Self::from_parts(protocol::list_delta::Parts::removal_by_id(id.into(), index))
    }

    /// ✍️ The list this delta turns `base` into; reached only from the diff's own `apply`, under the central applier's capability.
    pub fn commit_onto(&self, base: &Vec<FemLoad>, capability: protocol::ApplyCapability) -> Result<Vec<FemLoad>, protocol::list_delta::ApplyError> {
        self.clone().into_parts().commit_onto(base, capability)
    }

    /// ➕️ Composes `self` with the delta `later` applied after it.
    pub fn absorb(&mut self, later: Self) {
        let mut parts = std::mem::take(self).into_parts();
        parts.absorb(later.into_parts());
        *self = Self::from_parts(parts);
    }

    /// 🔁️ The negative delta over `base`, read row by row.
    pub fn inverse(&self, base: &Vec<FemLoad>) -> Self {
        Self::from_parts(self.clone().into_parts().inverse(base))
    }

    /// 🕳️ Whether the delta changes nothing.
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.inserted.is_empty() && self.moved.is_empty() && self.modified.iter().all(|entry| protocol::list_delta::RowPatch::<FemLoad>::is_empty(&entry.patch))
    }
}

protocol::list_delta! {
    /// 🧩 Positional keyed rows of the `combinations` list.
    pub Fem2dCombinationsDelta {
        removal: Fem2dCombinationRemoval,
        insertion: Fem2dCombinationInsertion,
        relocation: Fem2dCombinationRelocation,
        modification: Fem2dCombinationsModification,
        row: FemCombination,
        patch: FemCombination,
        key: id
    }
}

/// 🎛️ Owned-field patch of the analysis settings: exactly the fields the mutation sets.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct Fem2dAnalysisPatch {
    pub modal_count: Option<usize>,
    pub buckling_count: Option<usize>,
    pub deformation_scale: Option<f64>,
}
//#endregion 🔖️DeltaHelpers

//#region 🔖️RowPatches
/// 🩹 A row without finer owned fields is patched by replacing the whole row.
macro_rules! replace_row_patch {
    ($($row:ty),* $(,)?) => {$(
        impl RowPatch<$row> for $row {
            fn commit_into(&self, row: &mut $row, _capability: ApplyCapability) -> Result<(), MutationApplyError> {
                *row = self.clone();
                Ok(())
            }
            fn absorb(&mut self, later: Self) {
                *self = later;
            }
            fn inverse(&self, row: &$row) -> Self {
                row.clone()
            }
            fn is_empty(&self) -> bool {
                false
            }
        }
    )*};
}

/// 🩹 The boxed form of [`replace_row_patch`].
macro_rules! replace_boxed_row_patch {
    ($($row:ty),* $(,)?) => {$(
        impl RowPatch<$row> for Box<$row> {
            fn commit_into(&self, row: &mut $row, _capability: ApplyCapability) -> Result<(), MutationApplyError> {
                *row = (**self).clone();
                Ok(())
            }
            fn absorb(&mut self, later: Self) {
                *self = later;
            }
            fn inverse(&self, row: &$row) -> Self {
                Box::new(row.clone())
            }
            fn is_empty(&self) -> bool {
                false
            }
        }
    )*};
}

replace_row_patch!(FemNode, FemRegion, FemMaterial, FemSection, FemSupport, FemCombination);
replace_boxed_row_patch!(FemElement, FemLoad);

impl RowPatch<FemLoadCase> for Fem2dLoadCasePatch {
    fn commit_into(&self, row: &mut FemLoadCase, capability: ApplyCapability) -> Result<(), MutationApplyError> {
        if let Some(name) = &self.name {
            row.name = name.clone();
        }
        if let Some(self_weight) = self.self_weight {
            row.self_weight = self_weight;
        }
        if let Some(delta) = &self.loads {
            row.loads = delta.commit_onto(&row.loads, capability).map_err(|error| error.under(["loads"]))?;
        }
        Ok(())
    }
    fn absorb(&mut self, later: Self) {
        self.name = later.name.or_else(|| self.name.take());
        self.self_weight = later.self_weight.or(self.self_weight);
        self.loads = match (self.loads.take(), later.loads) {
            (Some(mut first), Some(second)) => {
                first.absorb(second);
                Some(first)
            }
            (first, None) => first,
            (None, second) => second,
        };
    }
    fn inverse(&self, row: &FemLoadCase) -> Self {
        Self {
            name: self.name.as_ref().map(|_| row.name.clone()),
            self_weight: self.self_weight.map(|_| row.self_weight),
            loads: self.loads.as_ref().map(|delta| delta.inverse(&row.loads)),
        }
    }
    fn is_empty(&self) -> bool {
        self.name.is_none() && self.self_weight.is_none() && self.loads.as_ref().is_none_or(|delta| delta.is_empty())
    }
}

impl Fem2dAnalysisPatch {
    fn applied(&self, base: &FemAnalysisSettings) -> FemAnalysisSettings {
        FemAnalysisSettings {
            modal_count: self.modal_count.unwrap_or(base.modal_count),
            buckling_count: self.buckling_count.unwrap_or(base.buckling_count),
            deformation_scale: self.deformation_scale.unwrap_or(base.deformation_scale),
        }
    }
    fn inverse_against(&self, base: &FemAnalysisSettings) -> Self {
        Self { modal_count: self.modal_count.map(|_| base.modal_count), buckling_count: self.buckling_count.map(|_| base.buckling_count), deformation_scale: self.deformation_scale.map(|_| base.deformation_scale) }
    }
    fn composed(&mut self, later: Self) {
        self.modal_count = later.modal_count.or(self.modal_count);
        self.buckling_count = later.buckling_count.or(self.buckling_count);
        self.deformation_scale = later.deformation_scale.or(self.deformation_scale);
    }
}
//#endregion 🔖️RowPatches

//#region 🔖️Apply
macro_rules! absorb_lists {
    ($first:ident, $second:ident, $($field:ident),+ $(,)?) => {$(
        if let Some(later) = $second.$field {
            let mut merged = $first.$field.take().unwrap_or_default();
            merged.absorb(later);
            $first.$field = (!merged.is_empty()).then_some(merged);
        }
    )+};
}

impl MutationDiff<Fem2dSnapshot> for Fem2dDiff {
    fn apply(&self, snapshot: &Fem2dSnapshot, capability: ApplyCapability) -> MutationApplyResult<Fem2dSnapshot> {
        let mut next = snapshot.clone();
        if let Some(delta) = &self.nodes {
            next.nodes = delta.commit_onto(&next.nodes, capability).map_err(|error| error.under(["nodes"]))?;
        }
        if let Some(delta) = &self.elements {
            next.elements = delta.commit_onto(&next.elements, capability).map_err(|error| error.under(["elements"]))?;
        }
        if let Some(delta) = &self.regions {
            next.regions = delta.commit_onto(&next.regions, capability).map_err(|error| error.under(["regions"]))?;
        }
        if let Some(delta) = &self.materials {
            next.materials = delta.commit_onto(&next.materials, capability).map_err(|error| error.under(["materials"]))?;
        }
        if let Some(delta) = &self.sections {
            next.sections = delta.commit_onto(&next.sections, capability).map_err(|error| error.under(["sections"]))?;
        }
        if let Some(delta) = &self.supports {
            next.supports = delta.commit_onto(&next.supports, capability).map_err(|error| error.under(["supports"]))?;
        }
        if let Some(delta) = &self.load_cases {
            next.load_cases = delta.commit_onto(&next.load_cases, capability).map_err(|error| error.under(["loadCases"]))?;
        }
        if let Some(delta) = &self.combinations {
            next.combinations = delta.commit_onto(&next.combinations, capability).map_err(|error| error.under(["combinations"]))?;
        }
        if let Some(patch) = &self.analysis {
            next.analysis = patch.applied(&next.analysis);
        }
        Ok(next)
    }
    fn absorb(&mut self, other: Self) {
        absorb_lists!(self, other, nodes, elements, regions, materials, sections, supports, load_cases, combinations);
        if let Some(later) = other.analysis {
            match &mut self.analysis {
                Some(earlier) => earlier.composed(later),
                None => self.analysis = Some(later),
            }
        }
    }
}

impl DiffAlgebra<Fem2dSnapshot> for Fem2dDiff {
    fn inverse(&self, base: &Fem2dSnapshot) -> Self {
        Self {
            nodes: self.nodes.as_ref().map(|delta| delta.inverse(&base.nodes)),
            elements: self.elements.as_ref().map(|delta| delta.inverse(&base.elements)),
            regions: self.regions.as_ref().map(|delta| delta.inverse(&base.regions)),
            materials: self.materials.as_ref().map(|delta| delta.inverse(&base.materials)),
            sections: self.sections.as_ref().map(|delta| delta.inverse(&base.sections)),
            supports: self.supports.as_ref().map(|delta| delta.inverse(&base.supports)),
            load_cases: self.load_cases.as_ref().map(|delta| delta.inverse(&base.load_cases)),
            combinations: self.combinations.as_ref().map(|delta| delta.inverse(&base.combinations)),
            analysis: self.analysis.as_ref().map(|patch| patch.inverse_against(&base.analysis)),
        }
    }
    fn is_empty(&self) -> bool {
        self.nodes.as_ref().is_none_or(|delta| delta.is_empty())
            && self.elements.as_ref().is_none_or(|delta| delta.is_empty())
            && self.regions.as_ref().is_none_or(|delta| delta.is_empty())
            && self.materials.as_ref().is_none_or(|delta| delta.is_empty())
            && self.sections.as_ref().is_none_or(|delta| delta.is_empty())
            && self.supports.as_ref().is_none_or(|delta| delta.is_empty())
            && self.load_cases.as_ref().is_none_or(|delta| delta.is_empty())
            && self.combinations.as_ref().is_none_or(|delta| delta.is_empty())
            && self.analysis.as_ref().is_none_or(|patch| patch == &Fem2dAnalysisPatch::default())
    }
}
//#endregion 🔖️Apply

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
