//! 🧬️ Fem3d diff schema — positional keyed deltas (framework `protocol::list_delta`) over the artifact.

use crate::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshotValue;
use crate::standards::v1::subsets::any::schema::structure::FemAnalysisSettings;
use crate::standards::v1::subsets::any::schema::structure::FemCombination;
use crate::standards::v1::subsets::any::schema::structure::FemElement;
use crate::standards::v1::subsets::any::schema::structure::FemLoad;
use crate::standards::v1::subsets::any::schema::structure::FemLoadCase;
use semio_s_fem_material::volumetric::FemMaterialValue;
use semio_s_fem_node::volumetric::FemNodeValue;
use crate::standards::v1::subsets::any::schema::structure::FemSection;
use crate::standards::v1::subsets::any::schema::structure::FemSolid;
use crate::standards::v1::subsets::any::schema::structure::FemSupport;
use crate::standards::v1::subsets::any::schema::structure::element_id;
use crate::standards::v1::subsets::any::schema::structure::load_id;
use crate::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot;
use ::semio_framework_schema::ArtifactSchema;
use protocol::list_delta::RowPatch;
use protocol::ApplyCapability;
use protocol::DiffAlgebra;
use protocol::MutationApplyError;
use protocol::MutationApplyResult;
use protocol::MutationDiff;
use protocol::ColdMutationDiff;
use protocol::ColdDiffAlgebra;
use semio_framework_value_derive::FromValue;
use semio_framework_value_derive::ToValue;

/// 🪪️ Explicit reference node identity belongs to its cold delta schema.
#[derive(Clone,Debug,PartialEq)]
pub struct Fem3dNodeKey;
impl protocol::list_delta::KeyOf<FemNodeValue>for Fem3dNodeKey{type Key=String;fn key(row:&FemNodeValue)->String{row.id.clone()}fn retire_row(row:FemNodeValue){<FemNodeValue as semio_framework_value::FromValue>::retire_decoded(row)}}

//#region 🔖️Diff
/// 🔺️ Sparse delta for the fem3d artifact: per-collection id-keyed rows plus an owned-field analysis patch.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, ArtifactSchema, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.fem.fem3d")]
pub struct Fem3dDiff {
    #[state(artifact)]
    pub nodes: Option<Fem3dNodesDelta>,
    #[state(artifact)]
    pub elements: Option<Fem3dElementsDelta>,
    #[state(artifact)]
    pub materials: Option<Fem3dMaterialsDelta>,
    #[state(artifact)]
    pub sections: Option<Fem3dSectionsDelta>,
    #[state(artifact)]
    pub solids: Option<Fem3dSolidsDelta>,
    #[state(artifact)]
    pub supports: Option<Fem3dSupportsDelta>,
    #[state(artifact)]
    pub load_cases: Option<Fem3dLoadCasesDelta>,
    #[state(artifact)]
    pub combinations: Option<Fem3dCombinationsDelta>,
    #[state(artifact)]
    pub analysis: Option<Fem3dAnalysisPatch>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
protocol::list_delta! {
    /// 🧩 Positional keyed rows of the `nodes` list.
    pub Fem3dNodesDelta {
        removal: Fem3dNodeRemoval,
        insertion: Fem3dNodeInsertion,
        relocation: Fem3dNodeRelocation,
        modification: Fem3dNodesModification,
        row: FemNodeValue,
        patch: Fem3dNodePatch,
        list: Vec<FemNodeValue>,
        key: String = by Fem3dNodeKey
    }
}

impl protocol::list_delta::Keyed for FemElement {
    type Key = String;
    fn key(&self) -> String {
        element_id(self).to_string()
    }
}

/// ➖️ One `elements` row removed, with the base index the inverse reinserts it at.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct Fem3dElementRemoval {
    pub id: String,
    pub index: usize,
}

/// ➕️ One `elements` row inserted at its index in the resulting list.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct Fem3dElementInsertion {
    pub index: usize,
    #[dsl(statements)]
    pub row: FemElement,
}

/// ↕️ One `elements` row moved from its base index to its index in the resulting list.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct Fem3dElementRelocation {
    pub id: String,
    pub from: usize,
    pub to: usize,
}

/// 🩹 One modified `elements` row (whole-row replacement).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct Fem3dElementsModification {
    pub id: String,
    pub patch: Fem3dElementPatch,
}

/// 🧩 Positional keyed delta of the `elements` list.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", default)]
pub struct Fem3dElementsDelta {
    pub removed: Vec<Fem3dElementRemoval>,
    pub inserted: Vec<Fem3dElementInsertion>,
    pub moved: Vec<Fem3dElementRelocation>,
    pub modified: Vec<Fem3dElementsModification>,
}

type Fem3dElementsDeltaParts = protocol::list_delta::Parts<FemElement, Fem3dElementPatch>;

impl Fem3dElementsDelta {
    /// ♻️ Disposes displaced or abandoned element lists through their typed row owner.
    pub fn retire_list(rows: Vec<FemElement>) { rows.into_iter().for_each(protocol::list_delta::Keyed::retire_cold); }
    fn into_parts(self) -> Fem3dElementsDeltaParts {
        protocol::list_delta::Parts {
            removed: self.removed.into_iter().map(|entry| (entry.id, entry.index)).collect(),
            inserted: self.inserted.into_iter().map(|entry| (entry.index, entry.row)).collect(),
            moved: self.moved.into_iter().map(|entry| (entry.id, entry.from, entry.to)).collect(),
            modified: self.modified.into_iter().map(|entry| (entry.id, entry.patch)).collect(),
        }
    }

    fn from_parts(parts: Fem3dElementsDeltaParts) -> Self {
        Self {
            removed: parts.removed.into_iter().map(|(id, index)| Fem3dElementRemoval { id, index }).collect(),
            inserted: parts.inserted.into_iter().map(|(index, row)| Fem3dElementInsertion { index, row }).collect(),
            moved: parts.moved.into_iter().map(|(id, from, to)| Fem3dElementRelocation { id, from, to }).collect(),
            modified: parts.modified.into_iter().map(|(id, patch)| Fem3dElementsModification { id, patch }).collect(),
        }
    }

    /// ➖️ The delta that removes the row `id` found at `index` of the base list.
    pub fn removal_by_id(id: impl Into<String>, index: usize) -> Self {
        Self::from_parts(protocol::list_delta::Parts::removal_by_id(id.into(), index))
    }

    /// ✍️ The list this delta turns `base` into; reached only from the diff's own `apply`, under the central applier's capability.
    pub fn commit_onto(&self, base: &Vec<FemElement>, capability: ApplyCapability) -> Result<Vec<FemElement>, protocol::list_delta::ApplyError> {
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
        self.removed.is_empty() && self.inserted.is_empty() && self.moved.is_empty() && self.modified.iter().all(|entry| RowPatch::<FemElement>::is_empty(&entry.patch))
    }
}

protocol::list_delta! {
    /// 🧩 Positional keyed rows of the `materials` list.
    pub Fem3dMaterialsDelta {
        removal: Fem3dMaterialRemoval,
        insertion: Fem3dMaterialInsertion,
        relocation: Fem3dMaterialRelocation,
        modification: Fem3dMaterialsModification,
        row: FemMaterialValue,
        patch: Fem3dMaterialPatch,
        list: Vec<FemMaterialValue>,
        key: String = by Fem3dMaterialKey
    }
}

protocol::list_delta! {
    /// 🧩 Positional keyed rows of the `sections` list.
    pub Fem3dSectionsDelta {
        removal: Fem3dSectionRemoval,
        insertion: Fem3dSectionInsertion,
        relocation: Fem3dSectionRelocation,
        modification: Fem3dSectionsModification,
        row: FemSection,
        patch: Fem3dSectionPatch,
        key: id
    }
}

protocol::list_delta! {
    /// 🧩 Positional keyed rows of the `solids` list.
    pub Fem3dSolidsDelta {
        removal: Fem3dSolidRemoval,
        insertion: Fem3dSolidInsertion,
        relocation: Fem3dSolidRelocation,
        modification: Fem3dSolidsModification,
        row: FemSolid,
        patch: Fem3dSolidPatch,
        key: id
    }
}

protocol::list_delta! {
    /// 🧩 Positional keyed rows of the `supports` list.
    pub Fem3dSupportsDelta {
        removal: Fem3dSupportRemoval,
        insertion: Fem3dSupportInsertion,
        relocation: Fem3dSupportRelocation,
        modification: Fem3dSupportsModification,
        row: FemSupport,
        patch: Fem3dSupportPatch,
        key: id
    }
}

protocol::list_delta! {
    /// 🧩 Positional keyed rows of the `load_cases` list.
    pub Fem3dLoadCasesDelta {
        removal: Fem3dLoadCaseRemoval,
        insertion: Fem3dLoadCaseInsertion,
        relocation: Fem3dLoadCaseRelocation,
        modification: Fem3dLoadCasesModification,
        row: FemLoadCase,
        patch: Fem3dLoadCasePatch,
        key: id
    }
}

/// 🩹 Owned-field patch of one load case: a rename, a self-weight switch and/or keyed load rows.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", default)]
pub struct Fem3dLoadCasePatch {
    pub name: Option<String>,
    pub self_weight: Option<bool>,
    pub loads: Option<Fem3dLoadsDelta>,
}

impl protocol::list_delta::Keyed for FemLoad {
    type Key = String;
    fn key(&self) -> String {
        load_id(self).to_string()
    }
}

/// ➖️ One `loads` row removed, with the base index the inverse reinserts it at.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct Fem3dLoadRemoval {
    pub id: String,
    pub index: usize,
}

/// ➕️ One `loads` row inserted at its index in the resulting list.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct Fem3dLoadInsertion {
    pub index: usize,
    #[dsl(statements)]
    pub row: FemLoad,
}

/// ↕️ One `loads` row moved from its base index to its index in the resulting list.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct Fem3dLoadRelocation {
    pub id: String,
    pub from: usize,
    pub to: usize,
}

/// 🩹 One modified `loads` row (whole-row replacement).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct Fem3dLoadsModification {
    pub id: String,
    pub patch: Fem3dLoadPatch,
}

/// 🧩 Positional keyed delta of the `loads` list.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", default)]
pub struct Fem3dLoadsDelta {
    pub removed: Vec<Fem3dLoadRemoval>,
    pub inserted: Vec<Fem3dLoadInsertion>,
    pub moved: Vec<Fem3dLoadRelocation>,
    pub modified: Vec<Fem3dLoadsModification>,
}

type Fem3dLoadsDeltaParts = protocol::list_delta::Parts<FemLoad, Fem3dLoadPatch>;

impl Fem3dLoadsDelta {
    /// 🧊️ Disposes displaced or abandoned load lists through their typed row owner.
    pub fn retire_list(rows: Vec<FemLoad>) { rows.into_iter().for_each(protocol::list_delta::Keyed::retire_cold); }
    fn into_parts(self) -> Fem3dLoadsDeltaParts {
        protocol::list_delta::Parts {
            removed: self.removed.into_iter().map(|entry| (entry.id, entry.index)).collect(),
            inserted: self.inserted.into_iter().map(|entry| (entry.index, entry.row)).collect(),
            moved: self.moved.into_iter().map(|entry| (entry.id, entry.from, entry.to)).collect(),
            modified: self.modified.into_iter().map(|entry| (entry.id, entry.patch)).collect(),
        }
    }

    fn from_parts(parts: Fem3dLoadsDeltaParts) -> Self {
        Self {
            removed: parts.removed.into_iter().map(|(id, index)| Fem3dLoadRemoval { id, index }).collect(),
            inserted: parts.inserted.into_iter().map(|(index, row)| Fem3dLoadInsertion { index, row }).collect(),
            moved: parts.moved.into_iter().map(|(id, from, to)| Fem3dLoadRelocation { id, from, to }).collect(),
            modified: parts.modified.into_iter().map(|(id, patch)| Fem3dLoadsModification { id, patch }).collect(),
        }
    }

    /// ➖️ The delta that removes the row `id` found at `index` of the base list.
    pub fn removal_by_id(id: impl Into<String>, index: usize) -> Self {
        Self::from_parts(protocol::list_delta::Parts::removal_by_id(id.into(), index))
    }

    /// ✍️ The list this delta turns `base` into; reached only from the diff's own `apply`, under the central applier's capability.
    pub fn commit_onto(&self, base: &Vec<FemLoad>, capability: ApplyCapability) -> Result<Vec<FemLoad>, protocol::list_delta::ApplyError> {
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
        self.removed.is_empty() && self.inserted.is_empty() && self.moved.is_empty() && self.modified.iter().all(|entry| RowPatch::<FemLoad>::is_empty(&entry.patch))
    }
}

protocol::list_delta! {
    /// 🧩 Positional keyed rows of the `combinations` list.
    pub Fem3dCombinationsDelta {
        removal: Fem3dCombinationRemoval,
        insertion: Fem3dCombinationInsertion,
        relocation: Fem3dCombinationRelocation,
        modification: Fem3dCombinationsModification,
        row: FemCombination,
        patch: Fem3dCombinationPatch,
        key: id
    }
}

/// 🎛️ Owned-field patch of the analysis settings: exactly the fields the mutation sets.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", default)]
pub struct Fem3dAnalysisPatch {
    pub modal_count: Option<usize>,
    pub buckling_count: Option<usize>,
    pub deformation_scale: Option<f64>,
}
//#endregion 🔖️DeltaHelpers

//#region 🔖️RowPatches
/// 🧩️ Combination names and independently keyed terms have separate ownership.
#[derive(Clone,Debug,Default,PartialEq,ToValue,FromValue,semio_framework_dsl_record_derive::DslRecord,semio_framework_value::RetireOwned)]
#[value(rename_all="camelCase",default,deny_unknown_fields)]
pub struct Fem3dCombinationPatch {
    #[value(skip_serializing_if="Option::is_none",deserialize_with="protocol::list_delta::supplied_value",deserialize_controlled_with="protocol::list_delta::supplied_value_controlled",default_controlled="protocol::list_delta::supplied_value_default",retire_with="protocol::list_delta::supplied_value_retire")]
    pub name: Option<String>,
    #[value(skip_serializing_if="Option::is_none",deserialize_with="protocol::list_delta::supplied_value",deserialize_controlled_with="protocol::list_delta::supplied_value_controlled",default_controlled="protocol::list_delta::supplied_value_default",retire_with="protocol::list_delta::supplied_value_retire")]
    pub terms: Option<protocol::command::MapDelta<f64>>,
}
impl RowPatch<FemCombination> for Fem3dCombinationPatch {
    fn commit_into(&self,row:&mut FemCombination,capability:ApplyCapability)->Result<(),MutationApplyError>{
        let terms = if let Some(delta)=&self.terms { let mut value=protocol::PreparedField::new(row.terms.clone(),<std::collections::BTreeMap<String,f64> as semio_framework_value::FromValue>::retire_decoded,capability); delta.apply_onto(value.value_mut())?; Some(value) } else { None };
        if let Some(name)=&self.name {row.name=name.clone();}
        if let Some(terms)=terms {terms.install(&mut row.terms);}
        Ok(())
    }
    fn absorb(&mut self,later:Self){self.name=later.name.or_else(||self.name.take());if let Some(next)=later.terms{if let Some(terms)=&mut self.terms{terms.absorb(next);}else{self.terms=Some(next);}}}
    fn inverse(&self,row:&FemCombination)->Self{Self{name:self.name.as_ref().map(|_|row.name.clone()),terms:self.terms.as_ref().map(|terms|terms.inverse(&row.terms))}}
    fn is_empty(&self)->bool{self.name.is_none()&&self.terms.as_ref().is_none_or(|terms|terms.is_empty())}
}
impl Fem3dCombinationPatch {
    /// 🔁️ Explicit replacement owns the name and every term in its live base.
    pub fn replacement(base:&FemCombination,row:&FemCombination)->Self{Self{name:Some(row.name.clone()),terms:Some(Self::replacing_terms(&base.terms,&row.terms))}}
    /// 🔗️ Builds exact keyed term intent from an explicit whole-list or whole-map replacement.
    pub fn replacing_terms(base:&std::collections::BTreeMap<String, f64>,rows:&std::collections::BTreeMap<String, f64>)->protocol::command::MapDelta<f64>{
        protocol::command::MapDelta::replacement(base, rows)
    }
}

/// 🧱️ Material properties owned by one diff; absent fields remain independently editable.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct Fem3dMaterialPatch {
    #[value(skip_serializing_if = "Option::is_none", deserialize_with = "protocol::list_delta::supplied_value", deserialize_controlled_with = "protocol::list_delta::supplied_value_controlled", default_controlled = "protocol::list_delta::supplied_value_default", retire_with = "protocol::list_delta::supplied_value_retire")]
    pub name: Option<String>,
    #[value(skip_serializing_if = "Option::is_none", deserialize_with = "protocol::list_delta::supplied_value", deserialize_controlled_with = "protocol::list_delta::supplied_value_controlled", default_controlled = "protocol::list_delta::supplied_value_default", retire_with = "protocol::list_delta::supplied_value_retire")]
    pub e: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none", deserialize_with = "protocol::list_delta::supplied_value", deserialize_controlled_with = "protocol::list_delta::supplied_value_controlled", default_controlled = "protocol::list_delta::supplied_value_default", retire_with = "protocol::list_delta::supplied_value_retire")]
    pub g: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none", deserialize_with = "protocol::list_delta::supplied_value", deserialize_controlled_with = "protocol::list_delta::supplied_value_controlled", default_controlled = "protocol::list_delta::supplied_value_default", retire_with = "protocol::list_delta::supplied_value_retire")]
    pub nu: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none", deserialize_with = "protocol::list_delta::supplied_value", deserialize_controlled_with = "protocol::list_delta::supplied_value_controlled", default_controlled = "protocol::list_delta::supplied_value_default", retire_with = "protocol::list_delta::supplied_value_retire")]
    pub rho: Option<f64>,
}

impl RowPatch<FemMaterialValue> for Fem3dMaterialPatch {
    fn commit_into(&self, row: &mut FemMaterialValue, _: ApplyCapability) -> Result<(), MutationApplyError> {
        if let Some(value) = &self.name { row.name = value.clone(); }
        if let Some(value) = self.e { row.e = value; }
        if let Some(value) = self.g { row.g = value; }
        if let Some(value) = self.nu { row.nu = value; }
        if let Some(value) = self.rho { row.rho = value; }
        Ok(())
    }
    fn absorb(&mut self, later: Self) {
        self.name = later.name.or_else(|| self.name.take());
        self.e = later.e.or(self.e);
        self.g = later.g.or(self.g);
        self.nu = later.nu.or(self.nu);
        self.rho = later.rho.or(self.rho);
    }
    fn inverse(&self, row: &FemMaterialValue) -> Self {
        Self { name: self.name.as_ref().map(|_| row.name.clone()), e: self.e.map(|_| row.e), g: self.g.map(|_| row.g), nu: self.nu.map(|_| row.nu), rho: self.rho.map(|_| row.rho) }
    }
    fn is_empty(&self) -> bool { self.name.is_none() && self.e.is_none() && self.g.is_none() && self.nu.is_none() && self.rho.is_none() }
}

impl Fem3dMaterialPatch {
    /// 🧱️ Explicit whole-record replacement owns every mutable material field.
    pub fn replacement(material: &FemMaterialValue) -> Self {
        Self { name: Some(material.name.clone()), e: Some(material.e), g: Some(material.g), nu: Some(material.nu), rho: Some(material.rho) }
    }

    /// 🩹️ Reads a candidate for validation without changing the live projection.
    pub fn candidate(&self, base: &FemMaterialValue) -> FemMaterialValue {
        FemMaterialValue { id: base.id.clone(), name: self.name.clone().unwrap_or_else(|| base.name.clone()), e: self.e.unwrap_or(base.e), g: self.g.unwrap_or(base.g), nu: self.nu.unwrap_or(base.nu), rho: self.rho.unwrap_or(base.rho) }
    }
}

/// 🧩️ Sparse node setters retain independent field intent through replay and inversion.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct Fem3dNodePatch {
    #[value(skip_serializing_if = "Option::is_none", deserialize_with = "protocol::list_delta::supplied_value", deserialize_controlled_with = "protocol::list_delta::supplied_value_controlled", default_controlled = "protocol::list_delta::supplied_value_default", retire_with = "protocol::list_delta::supplied_value_retire")]
    pub x: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none", deserialize_with = "protocol::list_delta::supplied_value", deserialize_controlled_with = "protocol::list_delta::supplied_value_controlled", default_controlled = "protocol::list_delta::supplied_value_default", retire_with = "protocol::list_delta::supplied_value_retire")]
    pub y: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none", deserialize_with = "protocol::list_delta::supplied_value", deserialize_controlled_with = "protocol::list_delta::supplied_value_controlled", default_controlled = "protocol::list_delta::supplied_value_default", retire_with = "protocol::list_delta::supplied_value_retire")]
    pub z: Option<f64>,
}

impl RowPatch<FemNodeValue> for Fem3dNodePatch {
    fn commit_into(&self, row: &mut FemNodeValue, _: ApplyCapability) -> Result<(), MutationApplyError> {
        if let Some(value) = self.x { row.x = value; }
        if let Some(value) = self.y { row.y = value; }
        if let Some(value) = self.z { row.z = value; }
        Ok(())
    }
    fn absorb(&mut self, later: Self) {
        self.x = later.x.or(self.x);
        self.y = later.y.or(self.y);
        self.z = later.z.or(self.z);
    }
    fn inverse(&self, row: &FemNodeValue) -> Self {
        Self { x: self.x.map(|_| row.x), y: self.y.map(|_| row.y), z: self.z.map(|_| row.z) }
    }
    fn is_empty(&self) -> bool { self.x.is_none() && self.y.is_none() && self.z.is_none() }
}

impl Fem3dNodePatch {
    /// 🔁️ Explicit replacement owns every mutable node field.
    pub fn replacement(row: &FemNodeValue) -> Self {
        Self { x: Some(row.x), y: Some(row.y), z: Some(row.z) }
    }
    /// 🔎️ Reads a complete validation candidate without mutating a projection.
    pub fn candidate(&self, base: &FemNodeValue) -> FemNodeValue {
        FemNodeValue { id: base.id.clone(), x: self.x.unwrap_or(base.x), y: self.y.unwrap_or(base.y), z: self.z.unwrap_or(base.z) }
    }
}


/// 🧩️ Sparse section setters retain independent field intent through replay and inversion.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct Fem3dSectionPatch {
    #[value(skip_serializing_if = "Option::is_none", deserialize_with = "protocol::list_delta::supplied_value", deserialize_controlled_with = "protocol::list_delta::supplied_value_controlled", default_controlled = "protocol::list_delta::supplied_value_default", retire_with = "protocol::list_delta::supplied_value_retire")]
    pub name: Option<String>,
    #[value(skip_serializing_if = "Option::is_none", deserialize_with = "protocol::list_delta::supplied_value", deserialize_controlled_with = "protocol::list_delta::supplied_value_controlled", default_controlled = "protocol::list_delta::supplied_value_default", retire_with = "protocol::list_delta::supplied_value_retire")]
    pub area: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none", deserialize_with = "protocol::list_delta::supplied_value", deserialize_controlled_with = "protocol::list_delta::supplied_value_controlled", default_controlled = "protocol::list_delta::supplied_value_default", retire_with = "protocol::list_delta::supplied_value_retire")]
    pub iy: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none", deserialize_with = "protocol::list_delta::supplied_value", deserialize_controlled_with = "protocol::list_delta::supplied_value_controlled", default_controlled = "protocol::list_delta::supplied_value_default", retire_with = "protocol::list_delta::supplied_value_retire")]
    pub iz: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none", deserialize_with = "protocol::list_delta::supplied_value", deserialize_controlled_with = "protocol::list_delta::supplied_value_controlled", default_controlled = "protocol::list_delta::supplied_value_default", retire_with = "protocol::list_delta::supplied_value_retire")]
    pub j: Option<f64>,
}

impl RowPatch<FemSection> for Fem3dSectionPatch {
    fn commit_into(&self, row: &mut FemSection, _: ApplyCapability) -> Result<(), MutationApplyError> {
        if let Some(value) = &self.name { row.name = value.clone(); }
        if let Some(value) = self.area { row.area = value; }
        if let Some(value) = self.iy { row.iy = value; }
        if let Some(value) = self.iz { row.iz = value; }
        if let Some(value) = self.j { row.j = value; }
        Ok(())
    }
    fn absorb(&mut self, later: Self) {
        self.name = later.name.or_else(|| self.name.take());
        self.area = later.area.or(self.area);
        self.iy = later.iy.or(self.iy);
        self.iz = later.iz.or(self.iz);
        self.j = later.j.or(self.j);
    }
    fn inverse(&self, row: &FemSection) -> Self {
        Self { name: self.name.as_ref().map(|_| row.name.clone()), area: self.area.map(|_| row.area), iy: self.iy.map(|_| row.iy), iz: self.iz.map(|_| row.iz), j: self.j.map(|_| row.j) }
    }
    fn is_empty(&self) -> bool { self.name.is_none() && self.area.is_none() && self.iy.is_none() && self.iz.is_none() && self.j.is_none() }
}

impl Fem3dSectionPatch {
    /// 🔁️ Explicit replacement owns every mutable section field.
    pub fn replacement(row: &FemSection) -> Self {
        Self { name: Some(row.name.clone()), area: Some(row.area), iy: Some(row.iy), iz: Some(row.iz), j: Some(row.j) }
    }
    /// 🔎️ Reads a complete validation candidate without mutating a projection.
    pub fn candidate(&self, base: &FemSection) -> FemSection {
        FemSection { id: base.id.clone(), name: self.name.clone().unwrap_or_else(|| base.name.clone()), area: self.area.unwrap_or(base.area), iy: self.iy.unwrap_or(base.iy), iz: self.iz.unwrap_or(base.iz), j: self.j.unwrap_or(base.j) }
    }
}

/// 🧩️ Sparse solid fields preserve independent event intent.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct Fem3dSolidPatch {
    #[value(skip_serializing_if = "Option::is_none", deserialize_with = "protocol::list_delta::supplied_value", deserialize_controlled_with = "protocol::list_delta::supplied_value_controlled", default_controlled = "protocol::list_delta::supplied_value_default", retire_with = "protocol::list_delta::supplied_value_retire")]
    pub name: Option<String>,
    #[value(skip_serializing_if = "Option::is_none", deserialize_with = "protocol::list_delta::supplied_value", deserialize_controlled_with = "protocol::list_delta::supplied_value_controlled", default_controlled = "protocol::list_delta::supplied_value_default", retire_with = "protocol::list_delta::supplied_value_retire")]
    pub outline: Option<Vec<[f64; 2]>>,
    #[value(skip_serializing_if = "Option::is_none", deserialize_with = "protocol::list_delta::supplied_value", deserialize_controlled_with = "protocol::list_delta::supplied_value_controlled", default_controlled = "protocol::list_delta::supplied_value_default", retire_with = "protocol::list_delta::supplied_value_retire")]
    pub holes: Option<Vec<Vec<[f64; 2]>>>,
    #[value(skip_serializing_if = "Option::is_none", deserialize_with = "protocol::list_delta::supplied_value", deserialize_controlled_with = "protocol::list_delta::supplied_value_controlled", default_controlled = "protocol::list_delta::supplied_value_default", retire_with = "protocol::list_delta::supplied_value_retire")]
    pub base_z: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none", deserialize_with = "protocol::list_delta::supplied_value", deserialize_controlled_with = "protocol::list_delta::supplied_value_controlled", default_controlled = "protocol::list_delta::supplied_value_default", retire_with = "protocol::list_delta::supplied_value_retire")]
    pub height: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none", deserialize_with = "protocol::list_delta::supplied_value", deserialize_controlled_with = "protocol::list_delta::supplied_value_controlled", default_controlled = "protocol::list_delta::supplied_value_default", retire_with = "protocol::list_delta::supplied_value_retire")]
    pub layers: Option<usize>,
    #[value(skip_serializing_if = "Option::is_none", deserialize_with = "protocol::list_delta::supplied_value", deserialize_controlled_with = "protocol::list_delta::supplied_value_controlled", default_controlled = "protocol::list_delta::supplied_value_default", retire_with = "protocol::list_delta::supplied_value_retire")]
    pub axis: Option<crate::standards::v1::subsets::any::schema::structure::FemAxis>,
    #[value(skip_serializing_if = "Option::is_none", deserialize_with = "protocol::list_delta::supplied_value", deserialize_controlled_with = "protocol::list_delta::supplied_value_controlled", default_controlled = "protocol::list_delta::supplied_value_default", retire_with = "protocol::list_delta::supplied_value_retire")]
    pub material_id: Option<String>,
    #[value(skip_serializing_if = "Option::is_none", deserialize_with = "protocol::list_delta::supplied_value", deserialize_controlled_with = "protocol::list_delta::supplied_value_controlled", default_controlled = "protocol::list_delta::supplied_value_default", retire_with = "protocol::list_delta::supplied_value_retire")]
    pub mesh_size: Option<f64>,
}
impl RowPatch<FemSolid> for Fem3dSolidPatch {
    fn commit_into(&self, row: &mut FemSolid, _: ApplyCapability) -> Result<(), MutationApplyError> {
        if let Some(value) = &self.name { row.name = value.clone(); }
        if let Some(value) = &self.outline { row.outline = value.clone(); }
        if let Some(value) = &self.holes { row.holes = value.clone(); }
        if let Some(value) = &self.base_z { row.base_z = value.clone(); }
        if let Some(value) = &self.height { row.height = value.clone(); }
        if let Some(value) = &self.layers { row.layers = value.clone(); }
        if let Some(value) = &self.axis { row.axis = value.clone(); }
        if let Some(value) = &self.material_id { row.material_id = value.clone(); }
        if let Some(value) = &self.mesh_size { row.mesh_size = value.clone(); }
        Ok(())
    }
    fn absorb(&mut self, later: Self) {
        self.name = later.name.or_else(|| self.name.take());
        self.outline = later.outline.or_else(|| self.outline.take());
        self.holes = later.holes.or_else(|| self.holes.take());
        self.base_z = later.base_z.or_else(|| self.base_z.take());
        self.height = later.height.or_else(|| self.height.take());
        self.layers = later.layers.or_else(|| self.layers.take());
        self.axis = later.axis.or_else(|| self.axis.take());
        self.material_id = later.material_id.or_else(|| self.material_id.take());
        self.mesh_size = later.mesh_size.or_else(|| self.mesh_size.take());
    }
    fn inverse(&self, row: &FemSolid) -> Self { Self { name: self.name.as_ref().map(|_| row.name.clone()), outline: self.outline.as_ref().map(|_| row.outline.clone()), holes: self.holes.as_ref().map(|_| row.holes.clone()), base_z: self.base_z.as_ref().map(|_| row.base_z.clone()), height: self.height.as_ref().map(|_| row.height.clone()), layers: self.layers.as_ref().map(|_| row.layers.clone()), axis: self.axis.as_ref().map(|_| row.axis.clone()), material_id: self.material_id.as_ref().map(|_| row.material_id.clone()), mesh_size: self.mesh_size.as_ref().map(|_| row.mesh_size.clone()) } }
    fn is_empty(&self) -> bool { self.name.is_none() && self.outline.is_none() && self.holes.is_none() && self.base_z.is_none() && self.height.is_none() && self.layers.is_none() && self.axis.is_none() && self.material_id.is_none() && self.mesh_size.is_none() }
}
impl Fem3dSolidPatch {
    /// 🔁️ An explicit replacement authors all mutable fields.
    pub fn replacement(row: &FemSolid) -> Self { Self { name: Some(row.name.clone()), outline: Some(row.outline.clone()), holes: Some(row.holes.clone()), base_z: Some(row.base_z.clone()), height: Some(row.height.clone()), layers: Some(row.layers.clone()), axis: Some(row.axis.clone()), material_id: Some(row.material_id.clone()), mesh_size: Some(row.mesh_size.clone()) } }
    /// 🔎️ Builds the current validation candidate.
    pub fn candidate(&self, base: &FemSolid) -> FemSolid { FemSolid { id: base.id.clone(), name: self.name.clone().unwrap_or_else(|| base.name.clone()), outline: self.outline.clone().unwrap_or_else(|| base.outline.clone()), holes: self.holes.clone().unwrap_or_else(|| base.holes.clone()), base_z: self.base_z.clone().unwrap_or_else(|| base.base_z.clone()), height: self.height.clone().unwrap_or_else(|| base.height.clone()), layers: self.layers.clone().unwrap_or_else(|| base.layers.clone()), axis: self.axis.clone().unwrap_or_else(|| base.axis.clone()), material_id: self.material_id.clone().unwrap_or_else(|| base.material_id.clone()), mesh_size: self.mesh_size.clone().unwrap_or_else(|| base.mesh_size.clone()) } }
}

/// 🧩️ Support intent owns a node reference, a whole set, or individual DOF memberships.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct Fem3dSupportPatch {
    #[value(skip_serializing_if = "Option::is_none", deserialize_with = "protocol::list_delta::supplied_value", deserialize_controlled_with = "protocol::list_delta::supplied_value_controlled", default_controlled = "protocol::list_delta::supplied_value_default", retire_with = "protocol::list_delta::supplied_value_retire")]
    pub node_id: Option<String>,
    #[value(skip_serializing_if = "Option::is_none", deserialize_with = "protocol::list_delta::supplied_value", deserialize_controlled_with = "protocol::list_delta::supplied_value_controlled", default_controlled = "protocol::list_delta::supplied_value_default", retire_with = "protocol::list_delta::supplied_value_retire")]
    pub fixed: Option<Vec<crate::standards::v1::subsets::any::schema::structure::FemDof>>,
    #[value(skip_serializing_if = "Option::is_none", deserialize_with = "protocol::list_delta::supplied_value", deserialize_controlled_with = "protocol::list_delta::supplied_value_controlled", default_controlled = "protocol::list_delta::supplied_value_default", retire_with = "protocol::list_delta::supplied_value_retire")]
    pub tx: Option<bool>,
    #[value(skip_serializing_if = "Option::is_none", deserialize_with = "protocol::list_delta::supplied_value", deserialize_controlled_with = "protocol::list_delta::supplied_value_controlled", default_controlled = "protocol::list_delta::supplied_value_default", retire_with = "protocol::list_delta::supplied_value_retire")]
    pub ty: Option<bool>,
    #[value(skip_serializing_if = "Option::is_none", deserialize_with = "protocol::list_delta::supplied_value", deserialize_controlled_with = "protocol::list_delta::supplied_value_controlled", default_controlled = "protocol::list_delta::supplied_value_default", retire_with = "protocol::list_delta::supplied_value_retire")]
    pub tz: Option<bool>,
    #[value(skip_serializing_if = "Option::is_none", deserialize_with = "protocol::list_delta::supplied_value", deserialize_controlled_with = "protocol::list_delta::supplied_value_controlled", default_controlled = "protocol::list_delta::supplied_value_default", retire_with = "protocol::list_delta::supplied_value_retire")]
    pub rx: Option<bool>,
    #[value(skip_serializing_if = "Option::is_none", deserialize_with = "protocol::list_delta::supplied_value", deserialize_controlled_with = "protocol::list_delta::supplied_value_controlled", default_controlled = "protocol::list_delta::supplied_value_default", retire_with = "protocol::list_delta::supplied_value_retire")]
    pub ry: Option<bool>,
    #[value(skip_serializing_if = "Option::is_none", deserialize_with = "protocol::list_delta::supplied_value", deserialize_controlled_with = "protocol::list_delta::supplied_value_controlled", default_controlled = "protocol::list_delta::supplied_value_default", retire_with = "protocol::list_delta::supplied_value_retire")]
    pub rz: Option<bool>,
}
impl RowPatch<FemSupport> for Fem3dSupportPatch {
    fn commit_into(&self, row: &mut FemSupport, _: ApplyCapability) -> Result<(), MutationApplyError> {
        if !self.valid() { return Err(MutationApplyError { code: "mutation.apply.invalid-patch".into(), message: "Support set and membership setters must be disjoint and canonically ordered.".into(), target: vec![row.id.clone()] }); }
        if let Some(value) = &self.node_id { row.node_id = value.clone(); }
        row.fixed = self.memberships(&row.fixed);
        Ok(())
    }
    fn absorb(&mut self, later: Self) {
        if later.fixed.is_some() { self.tx = None; self.ty = None; self.tz = None; self.rx = None; self.ry = None; self.rz = None; }
        if self.fixed.is_some() && later.fixed.is_none() { self.fixed = Some(later.memberships(self.fixed.as_ref().unwrap())); self.node_id = later.node_id.or_else(|| self.node_id.take()); return; }
        self.node_id = later.node_id.or_else(|| self.node_id.take());
        self.fixed = later.fixed.or_else(|| self.fixed.take());
        self.tx = later.tx.or_else(|| self.tx.take());
        self.ty = later.ty.or_else(|| self.ty.take());
        self.tz = later.tz.or_else(|| self.tz.take());
        self.rx = later.rx.or_else(|| self.rx.take());
        self.ry = later.ry.or_else(|| self.ry.take());
        self.rz = later.rz.or_else(|| self.rz.take());
    }
    fn inverse(&self, row: &FemSupport) -> Self { Self { node_id: self.node_id.as_ref().map(|_| row.node_id.clone()), fixed: self.fixed.as_ref().map(|_| row.fixed.clone()), tx: self.tx.map(|_| row.fixed.contains(&crate::FemDof::Tx)), ty: self.ty.map(|_| row.fixed.contains(&crate::FemDof::Ty)), tz: self.tz.map(|_| row.fixed.contains(&crate::FemDof::Tz)), rx: self.rx.map(|_| row.fixed.contains(&crate::FemDof::Rx)), ry: self.ry.map(|_| row.fixed.contains(&crate::FemDof::Ry)), rz: self.rz.map(|_| row.fixed.contains(&crate::FemDof::Rz)) } }
    fn is_empty(&self) -> bool { self.node_id.is_none() && self.fixed.is_none() && self.tx.is_none() && self.ty.is_none() && self.tz.is_none() && self.rx.is_none() && self.ry.is_none() && self.rz.is_none() }
}
impl Fem3dSupportPatch {
    /// 🛂️ Whole sets and membership setters are disjoint authored modes.
    pub fn valid(&self) -> bool { self.fixed.as_ref().is_none_or(|fixed| crate::FemDof::canonical(fixed) && self.tx.is_none() && self.ty.is_none() && self.tz.is_none() && self.rx.is_none() && self.ry.is_none() && self.rz.is_none()) }
    fn memberships(&self, base: &[crate::standards::v1::subsets::any::schema::structure::FemDof]) -> Vec<crate::standards::v1::subsets::any::schema::structure::FemDof> {
        let mut fixed = self.fixed.as_deref().unwrap_or(base).to_vec();
        if let Some(value) = self.tx { fixed.retain(|entry| *entry != crate::FemDof::Tx); if value { fixed.push(crate::FemDof::Tx); } }
        if let Some(value) = self.ty { fixed.retain(|entry| *entry != crate::FemDof::Ty); if value { fixed.push(crate::FemDof::Ty); } }
        if let Some(value) = self.tz { fixed.retain(|entry| *entry != crate::FemDof::Tz); if value { fixed.push(crate::FemDof::Tz); } }
        if let Some(value) = self.rx { fixed.retain(|entry| *entry != crate::FemDof::Rx); if value { fixed.push(crate::FemDof::Rx); } }
        if let Some(value) = self.ry { fixed.retain(|entry| *entry != crate::FemDof::Ry); if value { fixed.push(crate::FemDof::Ry); } }
        if let Some(value) = self.rz { fixed.retain(|entry| *entry != crate::FemDof::Rz); if value { fixed.push(crate::FemDof::Rz); } }
        crate::FemDof::ALL.into_iter().filter(|candidate| fixed.contains(candidate)).collect()
    }
    /// 🔁️ A replacement authors the node and whole restraint set.
    pub fn replacement(row: &FemSupport) -> Self { Self { node_id: Some(row.node_id.clone()), fixed: Some(row.fixed.clone()), ..Default::default() } }
    /// 🔎️ Reads current fields before semantic validation.
    pub fn candidate(&self, base: &FemSupport) -> FemSupport { FemSupport { id: base.id.clone(), node_id: self.node_id.clone().unwrap_or_else(|| base.node_id.clone()), fixed: self.memberships(&base.fixed) } }
}
/// 🧩️ Member field intent preserves the current variant and omitted sibling references.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct Fem3dElementPatch {
    #[value(skip_serializing_if = "Option::is_none", deserialize_with = "protocol::list_delta::supplied_value", deserialize_controlled_with = "protocol::list_delta::supplied_value_controlled", default_controlled = "protocol::list_delta::supplied_value_default", retire_with = "protocol::list_delta::supplied_value_retire")]
    pub kind: Option<String>,
    #[value(skip_serializing_if = "Option::is_none", deserialize_with = "protocol::list_delta::supplied_value", deserialize_controlled_with = "protocol::list_delta::supplied_value_controlled", default_controlled = "protocol::list_delta::supplied_value_default", retire_with = "protocol::list_delta::supplied_value_retire")]
    pub start: Option<String>,
    #[value(skip_serializing_if = "Option::is_none", deserialize_with = "protocol::list_delta::supplied_value", deserialize_controlled_with = "protocol::list_delta::supplied_value_controlled", default_controlled = "protocol::list_delta::supplied_value_default", retire_with = "protocol::list_delta::supplied_value_retire")]
    pub end: Option<String>,
    #[value(skip_serializing_if = "Option::is_none", deserialize_with = "protocol::list_delta::supplied_value", deserialize_controlled_with = "protocol::list_delta::supplied_value_controlled", default_controlled = "protocol::list_delta::supplied_value_default", retire_with = "protocol::list_delta::supplied_value_retire")]
    pub material_id: Option<String>,
    #[value(skip_serializing_if = "Option::is_none", deserialize_with = "protocol::list_delta::supplied_value", deserialize_controlled_with = "protocol::list_delta::supplied_value_controlled", default_controlled = "protocol::list_delta::supplied_value_default", retire_with = "protocol::list_delta::supplied_value_retire")]
    pub section_id: Option<String>,
    #[value(skip_serializing_if = "Option::is_none", deserialize_with = "protocol::list_delta::supplied_value", deserialize_controlled_with = "protocol::list_delta::supplied_value_controlled", default_controlled = "protocol::list_delta::supplied_value_default", retire_with = "protocol::list_delta::supplied_value_retire")]
    pub roll: Option<f64>,
}
impl RowPatch<FemElement> for Fem3dElementPatch {
    fn commit_into(&self, row: &mut FemElement, _: ApplyCapability) -> Result<(), MutationApplyError> { *row = self.candidate(row)?; Ok(()) }
    fn absorb(&mut self, later: Self) {
        if self.kind.as_deref() == Some("bar") && later.kind.as_deref() == Some("frame") && later.roll.is_none() { self.roll = Some(0.0); }
        if later.kind.as_deref() == Some("bar") { self.roll = None; }
        self.kind = later.kind.or_else(|| self.kind.take());
        self.start = later.start.or_else(|| self.start.take());
        self.end = later.end.or_else(|| self.end.take());
        self.material_id = later.material_id.or_else(|| self.material_id.take());
        self.section_id = later.section_id.or_else(|| self.section_id.take());
        self.roll = later.roll.or_else(|| self.roll.take());
    }
    fn inverse(&self, row: &FemElement) -> Self { let original = Self::replacement(row); Self { kind: self.kind.as_ref().and(original.kind), start: self.start.as_ref().and(original.start), end: self.end.as_ref().and(original.end), material_id: self.material_id.as_ref().and(original.material_id), section_id: self.section_id.as_ref().and(original.section_id), roll: if self.kind.is_some() || self.roll.is_some() { original.roll } else { None } } }
    fn is_empty(&self) -> bool { self.kind.is_none() && self.start.is_none() && self.end.is_none() && self.material_id.is_none() && self.section_id.is_none() && self.roll.is_none() }
}
impl Fem3dElementPatch {
    /// 🔁️ Replacement authors all mutable properties and the variant.
    pub fn replacement(row: &FemElement) -> Self {
        let (start, end, material_id, section_id, kind) = match row {
            FemElement::Bar { start, end, material_id, section_id, .. } => (start, end, material_id, section_id, "bar"),
            FemElement::Frame { start, end, material_id, section_id, .. } => (start, end, material_id, section_id, "frame"),
        };
        let mut patch = Self { kind: Some(kind.into()), start: Some(start.clone()), end: Some(end.clone()), material_id: Some(material_id.clone()), section_id: Some(section_id.clone()), ..Default::default() };
        if let FemElement::Frame { roll, .. } = row { patch.roll = Some(*roll); }
        patch
    }
    /// 🔎️ Common fields stay live and a newly created frame starts with zero roll.
    pub fn candidate(&self, base: &FemElement) -> Result<FemElement, MutationApplyError> {
        let (id, start, end, material_id, section_id, kind, roll) = match base {
            FemElement::Bar { id, start, end, material_id, section_id } => (id, start, end, material_id, section_id, "bar", 0.0),
            FemElement::Frame { id, start, end, material_id, section_id, roll } => (id, start, end, material_id, section_id, "frame", *roll),
        };
        let kind = self.kind.as_deref().unwrap_or(kind);
        let (id, start, end, material_id, section_id) = (id.clone(), self.start.clone().unwrap_or_else(|| start.clone()), self.end.clone().unwrap_or_else(|| end.clone()), self.material_id.clone().unwrap_or_else(|| material_id.clone()), self.section_id.clone().unwrap_or_else(|| section_id.clone()));
        match kind {
            "bar" => { if self.roll.is_some() { return Err(MutationApplyError { code: "mutation.apply.variant-mismatch".into(), message: "Only frame elements own roll.".into(), target: vec![id.clone()] }); } Ok(FemElement::Bar { id, start, end, material_id, section_id }) },
            "frame" => Ok(FemElement::Frame { id, start, end, material_id, section_id, roll: self.roll.unwrap_or(roll) }),
            _ => Err(MutationApplyError { code: "mutation.apply.variant-mismatch".into(), message: "Unknown element variant.".into(), target: vec![id] }),
        }
    }
}
/// 🧩️ Typed load fields use their variant tag as a replay precondition.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslEnum, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
#[dsl(field)]
pub enum Fem3dLoadPatch {
    #[value(rename_all = "camelCase")]
    Nodal {
        #[value(skip_serializing_if = "Option::is_none", deserialize_with = "protocol::list_delta::supplied_value", deserialize_controlled_with = "protocol::list_delta::supplied_value_controlled", default_controlled = "protocol::list_delta::supplied_value_default", retire_with = "protocol::list_delta::supplied_value_retire")]
        node_id: Option<String>,
        #[value(skip_serializing_if = "Option::is_none", deserialize_with = "protocol::list_delta::supplied_value", deserialize_controlled_with = "protocol::list_delta::supplied_value_controlled", default_controlled = "protocol::list_delta::supplied_value_default", retire_with = "protocol::list_delta::supplied_value_retire")]
        dof: Option<crate::standards::v1::subsets::any::schema::structure::FemDof>,
        #[value(skip_serializing_if = "Option::is_none", deserialize_with = "protocol::list_delta::supplied_value", deserialize_controlled_with = "protocol::list_delta::supplied_value_controlled", default_controlled = "protocol::list_delta::supplied_value_default", retire_with = "protocol::list_delta::supplied_value_retire")]
        value: Option<f64>,
    },
    #[value(rename_all = "camelCase")]
    MemberUdl {
        #[value(skip_serializing_if = "Option::is_none", deserialize_with = "protocol::list_delta::supplied_value", deserialize_controlled_with = "protocol::list_delta::supplied_value_controlled", default_controlled = "protocol::list_delta::supplied_value_default", retire_with = "protocol::list_delta::supplied_value_retire")]
        element_id: Option<String>,
        #[value(skip_serializing_if = "Option::is_none", deserialize_with = "protocol::list_delta::supplied_value", deserialize_controlled_with = "protocol::list_delta::supplied_value_controlled", default_controlled = "protocol::list_delta::supplied_value_default", retire_with = "protocol::list_delta::supplied_value_retire")]
        wx: Option<f64>,
        #[value(skip_serializing_if = "Option::is_none", deserialize_with = "protocol::list_delta::supplied_value", deserialize_controlled_with = "protocol::list_delta::supplied_value_controlled", default_controlled = "protocol::list_delta::supplied_value_default", retire_with = "protocol::list_delta::supplied_value_retire")]
        wy: Option<f64>,
        #[value(skip_serializing_if = "Option::is_none", deserialize_with = "protocol::list_delta::supplied_value", deserialize_controlled_with = "protocol::list_delta::supplied_value_controlled", default_controlled = "protocol::list_delta::supplied_value_default", retire_with = "protocol::list_delta::supplied_value_retire")]
        wz: Option<f64>,
    },
    #[value(rename_all = "camelCase")]
    Area {
        #[value(skip_serializing_if = "Option::is_none", deserialize_with = "protocol::list_delta::supplied_value", deserialize_controlled_with = "protocol::list_delta::supplied_value_controlled", default_controlled = "protocol::list_delta::supplied_value_default", retire_with = "protocol::list_delta::supplied_value_retire")]
        pressure: Option<f64>,
        #[value(skip_serializing_if = "Option::is_none", deserialize_with = "protocol::list_delta::supplied_value", deserialize_controlled_with = "protocol::list_delta::supplied_value_controlled", default_controlled = "protocol::list_delta::supplied_value_default", retire_with = "protocol::list_delta::supplied_value_retire")]
        solid_id: Option<String>,
    },
}
impl RowPatch<FemLoad> for Fem3dLoadPatch {
    fn commit_into(&self, row: &mut FemLoad, _: ApplyCapability) -> Result<(), MutationApplyError> { *row = self.candidate(row)?; Ok(()) }
    fn absorb(&mut self, later: Self) { match (self, later) {
            (Self::Nodal { node_id, dof, value }, Self::Nodal { node_id: next_node_id, dof: next_dof, value: next_value }) => { *node_id = next_node_id.or_else(|| node_id.take()); *dof = next_dof.or_else(|| dof.take()); *value = next_value.or_else(|| value.take()); },
            (Self::MemberUdl { element_id, wx, wy, wz }, Self::MemberUdl { element_id: next_element_id, wx: next_wx, wy: next_wy, wz: next_wz }) => { *element_id = next_element_id.or_else(|| element_id.take()); *wx = next_wx.or_else(|| wx.take()); *wy = next_wy.or_else(|| wy.take()); *wz = next_wz.or_else(|| wz.take()); },
            (Self::Area { pressure, solid_id }, Self::Area { pressure: next_pressure, solid_id: next_solid_id }) => { *pressure = next_pressure.or_else(|| pressure.take()); *solid_id = next_solid_id.or_else(|| solid_id.take()); },
        (current, next) => *current = next,
    } }
    fn inverse(&self, row: &FemLoad) -> Self { match (self, row) {
            (Self::Nodal { node_id, dof, value }, FemLoad::Nodal { node_id: base_node_id, dof: base_dof, value: base_value, .. }) => Self::Nodal { node_id: node_id.as_ref().map(|_| base_node_id.clone()), dof: dof.as_ref().map(|_| base_dof.clone()), value: value.as_ref().map(|_| base_value.clone()) },
            (Self::MemberUdl { element_id, wx, wy, wz }, FemLoad::MemberUdl { element_id: base_element_id, wx: base_wx, wy: base_wy, wz: base_wz, .. }) => Self::MemberUdl { element_id: element_id.as_ref().map(|_| base_element_id.clone()), wx: wx.as_ref().map(|_| base_wx.clone()), wy: wy.as_ref().map(|_| base_wy.clone()), wz: wz.as_ref().map(|_| base_wz.clone()) },
            (Self::Area { pressure, solid_id }, FemLoad::Area { pressure: base_pressure, solid_id: base_solid_id, .. }) => Self::Area { pressure: pressure.as_ref().map(|_| base_pressure.clone()), solid_id: solid_id.as_ref().map(|_| base_solid_id.clone()) },
        _ => self.clone(),
    } }
    fn is_empty(&self) -> bool { match self {
            Self::Nodal { node_id, dof, value } => node_id.is_none() && dof.is_none() && value.is_none(),
            Self::MemberUdl { element_id, wx, wy, wz } => element_id.is_none() && wx.is_none() && wy.is_none() && wz.is_none(),
            Self::Area { pressure, solid_id } => pressure.is_none() && solid_id.is_none(),
    } }
}
impl Fem3dLoadPatch {
    /// 🧬️ Starts an empty patch in the current load's admissible variant.
    pub fn empty_for(row: &FemLoad) -> Self { match row {
            FemLoad::Nodal { .. } => Self::Nodal { node_id: None, dof: None, value: None },
            FemLoad::MemberUdl { .. } => Self::MemberUdl { element_id: None, wx: None, wy: None, wz: None },
            FemLoad::Area { .. } => Self::Area { pressure: None, solid_id: None },
    } }
    /// 🔁️ A replacement owns every field in the specified variant.
    pub fn replacement(row: &FemLoad) -> Self { match row {
            FemLoad::Nodal { node_id, dof, value, .. } => Self::Nodal { node_id: Some(node_id.clone()), dof: Some(dof.clone()), value: Some(value.clone()) },
            FemLoad::MemberUdl { element_id, wx, wy, wz, .. } => Self::MemberUdl { element_id: Some(element_id.clone()), wx: Some(wx.clone()), wy: Some(wy.clone()), wz: Some(wz.clone()) },
            FemLoad::Area { pressure, solid_id, .. } => Self::Area { pressure: Some(pressure.clone()), solid_id: Some(solid_id.clone()) },
    } }
    /// 🔎️ Reads sparse fields only when the current variant still matches.
    pub fn candidate(&self, row: &FemLoad) -> Result<FemLoad, MutationApplyError> { match (self, row) {
            (Self::Nodal { node_id, dof, value }, FemLoad::Nodal { id, node_id: base_node_id, dof: base_dof, value: base_value }) => Ok(FemLoad::Nodal { id: id.clone(), node_id: node_id.clone().unwrap_or_else(|| base_node_id.clone()), dof: dof.clone().unwrap_or_else(|| base_dof.clone()), value: value.clone().unwrap_or_else(|| base_value.clone()) }),
            (Self::MemberUdl { element_id, wx, wy, wz }, FemLoad::MemberUdl { id, element_id: base_element_id, wx: base_wx, wy: base_wy, wz: base_wz }) => Ok(FemLoad::MemberUdl { id: id.clone(), element_id: element_id.clone().unwrap_or_else(|| base_element_id.clone()), wx: wx.clone().unwrap_or_else(|| base_wx.clone()), wy: wy.clone().unwrap_or_else(|| base_wy.clone()), wz: wz.clone().unwrap_or_else(|| base_wz.clone()) }),
            (Self::Area { pressure, solid_id }, FemLoad::Area { id, pressure: base_pressure, solid_id: base_solid_id }) => Ok(FemLoad::Area { id: id.clone(), pressure: pressure.clone().unwrap_or_else(|| base_pressure.clone()), solid_id: solid_id.clone().unwrap_or_else(|| base_solid_id.clone()) }),
        _ => Err(MutationApplyError { code: "mutation.apply.variant-mismatch".into(), message: "Load field intent requires its original variant.".into(), target: vec![load_id(row).into()] }),
    } }
}

impl RowPatch<FemLoadCase> for Fem3dLoadCasePatch {
    fn commit_into(&self, row: &mut FemLoadCase, capability: ApplyCapability) -> Result<(), MutationApplyError> {
        if let Some(name) = &self.name {
            row.name = name.clone();
        }
        if let Some(self_weight) = self.self_weight {
            row.self_weight = self_weight;
        }
        if let Some(delta) = &self.loads {
            let next = delta.commit_onto(&row.loads, capability).map_err(|error| error.under(["loads"]))?;
            Fem3dLoadsDelta::retire_list(std::mem::replace(&mut row.loads, next));
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

impl Fem3dAnalysisPatch {
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

impl MutationDiff<Fem3dSnapshot> for Fem3dDiff {


}
impl protocol::ColdMutationDiff<Fem3dSnapshotValue> for Fem3dDiff {

    fn absorb(&mut self, other: Self) {
        absorb_lists!(self, other, nodes, elements, materials, sections, solids, supports, load_cases, combinations);
        if let Some(later) = other.analysis {
            match &mut self.analysis {
                Some(earlier) => earlier.composed(later),
                None => self.analysis = Some(later),
            }
        }
    }

    fn apply(&self, next: &mut Fem3dSnapshotValue, capability: ApplyCapability) -> MutationApplyResult<()> {
        let nodes = self.nodes.as_ref().map(|delta| delta.commit_onto(&next.nodes, capability).map_err(|error| error.under(["nodes"]))).transpose()?.map(|value| protocol::PreparedField::new(value, Fem3dNodesDelta::retire_list, capability));
        let elements = self.elements.as_ref().map(|delta| delta.commit_onto(&next.elements, capability).map_err(|error| error.under(["elements"]))).transpose()?.map(|value| protocol::PreparedField::new(value, Fem3dElementsDelta::retire_list, capability));
        let materials = self.materials.as_ref().map(|delta| delta.commit_onto(&next.materials, capability).map_err(|error| error.under(["materials"]))).transpose()?.map(|value| protocol::PreparedField::new(value, Fem3dMaterialsDelta::retire_list, capability));
        let sections = self.sections.as_ref().map(|delta| delta.commit_onto(&next.sections, capability).map_err(|error| error.under(["sections"]))).transpose()?.map(|value| protocol::PreparedField::new(value, Fem3dSectionsDelta::retire_list, capability));
        let solids = self.solids.as_ref().map(|delta| delta.commit_onto(&next.solids, capability).map_err(|error| error.under(["solids"]))).transpose()?.map(|value| protocol::PreparedField::new(value, Fem3dSolidsDelta::retire_list, capability));
        let supports = self.supports.as_ref().map(|delta| delta.commit_onto(&next.supports, capability).map_err(|error| error.under(["supports"]))).transpose()?.map(|value| protocol::PreparedField::new(value, Fem3dSupportsDelta::retire_list, capability));
        let load_cases = self.load_cases.as_ref().map(|delta| delta.commit_onto(&next.load_cases, capability).map_err(|error| error.under(["loadCases"]))).transpose()?.map(|value| protocol::PreparedField::new(value, Fem3dLoadCasesDelta::retire_list, capability));
        let combinations = self.combinations.as_ref().map(|delta| delta.commit_onto(&next.combinations, capability).map_err(|error| error.under(["combinations"]))).transpose()?.map(|value| protocol::PreparedField::new(value, Fem3dCombinationsDelta::retire_list, capability));
        let analysis = self.analysis.as_ref().map(|patch| patch.applied(&next.analysis));
        if let Some(value) = nodes { value.install(&mut next.nodes); }
        if let Some(value) = elements { value.install(&mut next.elements); }
        if let Some(value) = materials { value.install(&mut next.materials); }
        if let Some(value) = sections { value.install(&mut next.sections); }
        if let Some(value) = solids { value.install(&mut next.solids); }
        if let Some(value) = supports { value.install(&mut next.supports); }
        if let Some(value) = load_cases { value.install(&mut next.load_cases); }
        if let Some(value) = combinations { value.install(&mut next.combinations); }
        if let Some(value) = analysis { next.analysis = value; }
        Ok(())
    }
}

impl DiffAlgebra<Fem3dSnapshot> for Fem3dDiff {
    fn is_empty(&self) -> bool {
        self.nodes.as_ref().is_none_or(|delta| delta.is_empty())
            && self.elements.as_ref().is_none_or(|delta| delta.is_empty())
            && self.materials.as_ref().is_none_or(|delta| delta.is_empty())
            && self.sections.as_ref().is_none_or(|delta| delta.is_empty())
            && self.solids.as_ref().is_none_or(|delta| delta.is_empty())
            && self.supports.as_ref().is_none_or(|delta| delta.is_empty())
            && self.load_cases.as_ref().is_none_or(|delta| delta.is_empty())
            && self.combinations.as_ref().is_none_or(|delta| delta.is_empty())
            && self.analysis.as_ref().is_none_or(|patch| patch == &Fem3dAnalysisPatch::default())
    }
}
impl protocol::ColdDiffAlgebra<Fem3dSnapshotValue> for Fem3dDiff {
    fn inverse(&self, base: &Fem3dSnapshotValue) -> Self {
        Self {
            nodes: self.nodes.as_ref().map(|delta| delta.inverse(&base.nodes)),
            elements: self.elements.as_ref().map(|delta| delta.inverse(&base.elements)),
            materials: self.materials.as_ref().map(|delta| delta.inverse(&base.materials)),
            sections: self.sections.as_ref().map(|delta| delta.inverse(&base.sections)),
            solids: self.solids.as_ref().map(|delta| delta.inverse(&base.solids)),
            supports: self.supports.as_ref().map(|delta| delta.inverse(&base.supports)),
            load_cases: self.load_cases.as_ref().map(|delta| delta.inverse(&base.load_cases)),
            combinations: self.combinations.as_ref().map(|delta| delta.inverse(&base.combinations)),
            analysis: self.analysis.as_ref().map(|patch| patch.inverse_against(&base.analysis)),
        }
    }
}
//#endregion 🔖️Apply

protocol::retained_row_patch!(pub retained_material_fields for Fem3dMaterialPatch => FemMaterialValue {name:String,e:f64,g:f64,nu:f64,rho:f64});
protocol::retained_row_patch!(pub retained_node_fields for Fem3dNodePatch => FemNodeValue {x:f64,y:f64,z:f64});
protocol::retained_row_patch!(pub retained_section_fields for Fem3dSectionPatch => FemSection {name:String,area:f64,iy:f64,iz:f64,j:f64});
protocol::retained_row_patch!(pub retained_solid_fields for Fem3dSolidPatch => FemSolid {name:String,outline:Vec<[f64;2]>,holes:Vec<Vec<[f64;2]>>,base_z:f64,height:f64,layers:usize,axis:crate::standards::v1::subsets::any::schema::structure::FemAxis,material_id:String,mesh_size:f64});

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

impl DiffAlgebra<Fem3dSnapshotValue> for Fem3dDiff {fn is_empty(&self)->bool{<Self as DiffAlgebra<Fem3dSnapshot>>::is_empty(self)}}
impl MutationDiff<Fem3dSnapshotValue> for Fem3dDiff {}
impl Fem3dDiff {pub fn is_empty(&self)->bool{<Self as DiffAlgebra<Fem3dSnapshot>>::is_empty(self)}}

impl semio_s_fem_material::patch::NativeMaterialPatch<4> for Fem3dMaterialPatch{fn name(&self)->Option<&String>{self.name.as_ref()}fn numbers(&self)->[Option<f64>;4]{[self.e,self.g,self.nu,self.rho]}}

/// 🔑️ Explicit cold Material values use the existing local key marker contract.
#[derive(Clone,Debug,PartialEq)]
pub struct Fem3dMaterialKey;
impl protocol::list_delta::KeyOf<FemMaterialValue> for Fem3dMaterialKey{type Key=String;fn key_of(row:&FemMaterialValue)->String{row.id.clone()}fn retire_cold(row:FemMaterialValue){<FemMaterialValue as semio_framework_value::FromValue>::retire_decoded(row)}}
