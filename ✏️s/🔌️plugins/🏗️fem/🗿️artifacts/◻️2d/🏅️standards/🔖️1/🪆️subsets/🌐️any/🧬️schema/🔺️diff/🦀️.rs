//! 🧬️ Fem2d diff schema — positional keyed deltas (framework `protocol::list_delta`) over the artifact.

use crate::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshotValue;
use crate::standards::v1::subsets::any::schema::structure::FemAnalysisSettings;
use crate::standards::v1::subsets::any::schema::structure::FemCombination;
use crate::standards::v1::subsets::any::schema::structure::FemElement;
use crate::standards::v1::subsets::any::schema::structure::FemLoad;
use crate::standards::v1::subsets::any::schema::structure::FemLoadCase;
use semio_s_fem_material::planar::FemMaterialValue;
use semio_s_fem_node::planar::FemNodeValue;
use crate::standards::v1::subsets::any::schema::structure::FemRegion;
use crate::standards::v1::subsets::any::schema::structure::FemSection;
use crate::standards::v1::subsets::any::schema::structure::FemSupport;
use crate::standards::v1::subsets::any::schema::structure::element_id;
use crate::standards::v1::subsets::any::schema::structure::load_id;
use crate::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot;
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
pub struct Fem2dNodeKey;
impl protocol::list_delta::KeyOf<FemNodeValue>for Fem2dNodeKey{type Key=String;fn key(row:&FemNodeValue)->String{row.id.clone()}fn retire_row(row:FemNodeValue){<FemNodeValue as semio_framework_value::FromValue>::retire_decoded(row)}}

//#region 🔖️Diff
/// 🔺️ Sparse delta for the fem2d artifact: per-collection id-keyed rows plus an owned-field analysis patch.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, ArtifactSchema, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
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
        row: FemNodeValue,
        patch: Fem2dNodePatch,
        list: Vec<FemNodeValue>,
        key: String = by Fem2dNodeKey
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
pub struct Fem2dElementRemoval {
    pub id: String,
    pub index: usize,
}

/// ➕️ One `elements` row inserted at its index in the resulting list.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct Fem2dElementInsertion {
    pub index: usize,
    #[dsl(statements)]
    pub row: FemElement,
}

/// ↕️ One `elements` row moved from its base index to its index in the resulting list.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct Fem2dElementRelocation {
    pub id: String,
    pub from: usize,
    pub to: usize,
}

/// 🩹 One modified `elements` row (whole-row replacement).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct Fem2dElementsModification {
    pub id: String,
    pub patch: Fem2dElementPatch,
}

/// 🧩 Positional keyed delta of the `elements` list.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", default)]
pub struct Fem2dElementsDelta {
    pub removed: Vec<Fem2dElementRemoval>,
    pub inserted: Vec<Fem2dElementInsertion>,
    pub moved: Vec<Fem2dElementRelocation>,
    pub modified: Vec<Fem2dElementsModification>,
}

type Fem2dElementsDeltaParts = protocol::list_delta::Parts<FemElement, Fem2dElementPatch>;

impl Fem2dElementsDelta {
    /// ♻️ Disposes displaced or abandoned element lists through their typed row owner.
    pub fn retire_list(rows: Vec<FemElement>) { rows.into_iter().for_each(protocol::list_delta::Keyed::retire_cold); }
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
    /// 🧩 Positional keyed rows of the `regions` list.
    pub Fem2dRegionsDelta {
        removal: Fem2dRegionRemoval,
        insertion: Fem2dRegionInsertion,
        relocation: Fem2dRegionRelocation,
        modification: Fem2dRegionsModification,
        row: FemRegion,
        patch: Fem2dRegionPatch,
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
        row: FemMaterialValue,
        patch: Fem2dMaterialPatch,
        list: Vec<FemMaterialValue>,
        key: String = by Fem2dMaterialKey
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
        patch: Fem2dSectionPatch,
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
        patch: Fem2dSupportPatch,
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

/// ➖️ One `loads` row removed, with the base index the inverse reinserts it at.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct Fem2dLoadRemoval {
    pub id: String,
    pub index: usize,
}

/// ➕️ One `loads` row inserted at its index in the resulting list.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct Fem2dLoadInsertion {
    pub index: usize,
    #[dsl(statements)]
    pub row: FemLoad,
}

/// ↕️ One `loads` row moved from its base index to its index in the resulting list.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct Fem2dLoadRelocation {
    pub id: String,
    pub from: usize,
    pub to: usize,
}

/// 🩹 One modified `loads` row (whole-row replacement).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct Fem2dLoadsModification {
    pub id: String,
    pub patch: Fem2dLoadPatch,
}

/// 🧩 Positional keyed delta of the `loads` list.
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

/// 🎛️ Owned-field patch of the analysis settings: exactly the fields the mutation sets.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", default)]
pub struct Fem2dAnalysisPatch {
    pub modal_count: Option<usize>,
    pub buckling_count: Option<usize>,
    pub deformation_scale: Option<f64>,
}
//#endregion 🔖️DeltaHelpers

//#region 🔖️RowPatches
/// ⚖️ One keyed combination term owns only its authored scale.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all="camelCase",default,deny_unknown_fields)]
pub struct Fem2dCombinationTermPatch {
    #[value(skip_serializing_if="Option::is_none",deserialize_with="protocol::list_delta::supplied_value",deserialize_controlled_with="protocol::list_delta::supplied_value_controlled",default_controlled="protocol::list_delta::supplied_value_default",retire_with="protocol::list_delta::supplied_value_retire")]
    pub factor: Option<f64>,
}
impl RowPatch<crate::standards::v1::subsets::any::schema::structure::FemCombinationTerm> for Fem2dCombinationTermPatch {
    fn commit_into(&self,row:&mut crate::standards::v1::subsets::any::schema::structure::FemCombinationTerm,_:ApplyCapability)->Result<(),MutationApplyError>{if let Some(value)=self.factor{row.factor=value;}Ok(())}
    fn absorb(&mut self,later:Self){self.factor=later.factor.or(self.factor);}
    fn inverse(&self,row:&crate::standards::v1::subsets::any::schema::structure::FemCombinationTerm)->Self{Self{factor:self.factor.map(|_|row.factor)}}
    fn is_empty(&self)->bool{self.factor.is_none()}
}
protocol::list_delta! {
    /// 🔗️ Positional terms retain stable case identities through sparse edits and undo.
    pub Fem2dCombinationTermsDelta {
        removal: Fem2dCombinationTermRemoval,
        insertion: Fem2dCombinationTermInsertion,
        relocation: Fem2dCombinationTermRelocation,
        modification: Fem2dCombinationTermsModification,
        row: crate::standards::v1::subsets::any::schema::structure::FemCombinationTerm,
        patch: Fem2dCombinationTermPatch,
        key: case_id
    }
}
/// 🧩️ Combination names and independently keyed terms have separate ownership.
#[derive(Clone,Debug,Default,PartialEq,ToValue,FromValue,semio_framework_dsl_record_derive::DslRecord,semio_framework_value::RetireOwned)]
#[value(rename_all="camelCase",default,deny_unknown_fields)]
pub struct Fem2dCombinationPatch {
    #[value(skip_serializing_if="Option::is_none",deserialize_with="protocol::list_delta::supplied_value",deserialize_controlled_with="protocol::list_delta::supplied_value_controlled",default_controlled="protocol::list_delta::supplied_value_default",retire_with="protocol::list_delta::supplied_value_retire")]
    pub name: Option<String>,
    #[value(skip_serializing_if="Option::is_none",deserialize_with="protocol::list_delta::supplied_value",deserialize_controlled_with="protocol::list_delta::supplied_value_controlled",default_controlled="protocol::list_delta::supplied_value_default",retire_with="protocol::list_delta::supplied_value_retire")]
    pub terms: Option<Fem2dCombinationTermsDelta>,
}
impl RowPatch<FemCombination> for Fem2dCombinationPatch {
    fn commit_into(&self,row:&mut FemCombination,capability:ApplyCapability)->Result<(),MutationApplyError>{
        let terms = if let Some(delta)=&self.terms { let value={ delta.commit_onto(&row.terms, capability)? }; Some(protocol::PreparedField::new(value,Fem2dCombinationTermsDelta::retire_list,capability)) } else { None };
        if let Some(name)=&self.name {row.name=name.clone();}
        if let Some(terms)=terms {terms.install(&mut row.terms);}
        Ok(())
    }
    fn absorb(&mut self,later:Self){self.name=later.name.or_else(||self.name.take());if let Some(next)=later.terms{if let Some(terms)=&mut self.terms{terms.absorb(next);}else{self.terms=Some(next);}}}
    fn inverse(&self,row:&FemCombination)->Self{Self{name:self.name.as_ref().map(|_|row.name.clone()),terms:self.terms.as_ref().map(|terms|terms.inverse(&row.terms))}}
    fn is_empty(&self)->bool{self.name.is_none()&&self.terms.as_ref().is_none_or(|terms|terms.is_empty())}
}
impl Fem2dCombinationPatch {
    /// 🔁️ Explicit replacement owns the name and every term in its live base.
    pub fn replacement(base:&FemCombination,row:&FemCombination)->Self{Self{name:Some(row.name.clone()),terms:Some(Self::replacing_terms(&base.terms,&row.terms))}}
    /// 🔗️ Builds exact keyed term intent from an explicit whole-list or whole-map replacement.
    pub fn replacing_terms(base:&Vec<crate::standards::v1::subsets::any::schema::structure::FemCombinationTerm>,rows:&Vec<crate::standards::v1::subsets::any::schema::structure::FemCombinationTerm>)->Fem2dCombinationTermsDelta{
                Fem2dCombinationTermsDelta {
            removed: base.iter().enumerate().map(|(index,row)| Fem2dCombinationTermRemoval { id: row.case_id.clone(), index }).collect(),
            inserted: rows.iter().enumerate().map(|(index,row)| Fem2dCombinationTermInsertion { index, row: row.clone() }).collect(),
            ..Default::default()
        }
    }
}

/// 🧱️ Material properties owned by one diff; absent fields remain independently editable.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct Fem2dMaterialPatch {
    #[value(skip_serializing_if = "Option::is_none", deserialize_with = "protocol::list_delta::supplied_value", deserialize_controlled_with = "protocol::list_delta::supplied_value_controlled", default_controlled = "protocol::list_delta::supplied_value_default", retire_with = "protocol::list_delta::supplied_value_retire")]
    pub name: Option<String>,
    #[value(skip_serializing_if = "Option::is_none", deserialize_with = "protocol::list_delta::supplied_value", deserialize_controlled_with = "protocol::list_delta::supplied_value_controlled", default_controlled = "protocol::list_delta::supplied_value_default", retire_with = "protocol::list_delta::supplied_value_retire")]
    pub e: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none", deserialize_with = "protocol::list_delta::supplied_value", deserialize_controlled_with = "protocol::list_delta::supplied_value_controlled", default_controlled = "protocol::list_delta::supplied_value_default", retire_with = "protocol::list_delta::supplied_value_retire")]
    pub nu: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none", deserialize_with = "protocol::list_delta::supplied_value", deserialize_controlled_with = "protocol::list_delta::supplied_value_controlled", default_controlled = "protocol::list_delta::supplied_value_default", retire_with = "protocol::list_delta::supplied_value_retire")]
    pub rho: Option<f64>,
}

impl RowPatch<FemMaterialValue> for Fem2dMaterialPatch {
    fn commit_into(&self, row: &mut FemMaterialValue, _: ApplyCapability) -> Result<(), MutationApplyError> {
        if let Some(value) = &self.name { row.name = value.clone(); }
        if let Some(value) = self.e { row.e = value; }
        if let Some(value) = self.nu { row.nu = value; }
        if let Some(value) = self.rho { row.rho = value; }
        Ok(())
    }
    fn absorb(&mut self, later: Self) {
        self.name = later.name.or_else(|| self.name.take());
        self.e = later.e.or(self.e);
        self.nu = later.nu.or(self.nu);
        self.rho = later.rho.or(self.rho);
    }
    fn inverse(&self, row: &FemMaterialValue) -> Self {
        Self { name: self.name.as_ref().map(|_| row.name.clone()), e: self.e.map(|_| row.e), nu: self.nu.map(|_| row.nu), rho: self.rho.map(|_| row.rho) }
    }
    fn is_empty(&self) -> bool { self.name.is_none() && self.e.is_none() && self.nu.is_none() && self.rho.is_none() }
}

impl Fem2dMaterialPatch {
    /// 🧱️ Explicit whole-record replacement owns every mutable material field.
    pub fn replacement(material: &FemMaterialValue) -> Self {
        Self { name: Some(material.name.clone()), e: Some(material.e), nu: Some(material.nu), rho: Some(material.rho) }
    }

    /// 🩹️ Reads a candidate for validation without changing the live projection.
    pub fn candidate(&self, base: &FemMaterialValue) -> FemMaterialValue {
        FemMaterialValue { id: base.id.clone(), name: self.name.clone().unwrap_or_else(|| base.name.clone()), e: self.e.unwrap_or(base.e), nu: self.nu.unwrap_or(base.nu), rho: self.rho.unwrap_or(base.rho) }
    }
}

/// 🧩️ Sparse node setters retain independent field intent through replay and inversion.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct Fem2dNodePatch {
    #[value(skip_serializing_if = "Option::is_none", deserialize_with = "protocol::list_delta::supplied_value", deserialize_controlled_with = "protocol::list_delta::supplied_value_controlled", default_controlled = "protocol::list_delta::supplied_value_default", retire_with = "protocol::list_delta::supplied_value_retire")]
    pub x: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none", deserialize_with = "protocol::list_delta::supplied_value", deserialize_controlled_with = "protocol::list_delta::supplied_value_controlled", default_controlled = "protocol::list_delta::supplied_value_default", retire_with = "protocol::list_delta::supplied_value_retire")]
    pub y: Option<f64>,
}

impl RowPatch<FemNodeValue> for Fem2dNodePatch {
    fn commit_into(&self, row: &mut FemNodeValue, _: ApplyCapability) -> Result<(), MutationApplyError> {
        if let Some(value) = self.x { row.x = value; }
        if let Some(value) = self.y { row.y = value; }
        Ok(())
    }
    fn absorb(&mut self, later: Self) {
        self.x = later.x.or(self.x);
        self.y = later.y.or(self.y);
    }
    fn inverse(&self, row: &FemNodeValue) -> Self {
        Self { x: self.x.map(|_| row.x), y: self.y.map(|_| row.y) }
    }
    fn is_empty(&self) -> bool { self.x.is_none() && self.y.is_none() }
}

impl Fem2dNodePatch {
    /// 🔁️ Explicit replacement owns every mutable node field.
    pub fn replacement(row: &FemNodeValue) -> Self {
        Self { x: Some(row.x), y: Some(row.y) }
    }
    /// 🔎️ Reads a complete validation candidate without mutating a projection.
    pub fn candidate(&self, base: &FemNodeValue) -> FemNodeValue {
        FemNodeValue { id: base.id.clone(), x: self.x.unwrap_or(base.x), y: self.y.unwrap_or(base.y) }
    }
}


/// 🧩️ Sparse section setters retain independent field intent through replay and inversion.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct Fem2dSectionPatch {
    #[value(skip_serializing_if = "Option::is_none", deserialize_with = "protocol::list_delta::supplied_value", deserialize_controlled_with = "protocol::list_delta::supplied_value_controlled", default_controlled = "protocol::list_delta::supplied_value_default", retire_with = "protocol::list_delta::supplied_value_retire")]
    pub name: Option<String>,
    #[value(skip_serializing_if = "Option::is_none", deserialize_with = "protocol::list_delta::supplied_value", deserialize_controlled_with = "protocol::list_delta::supplied_value_controlled", default_controlled = "protocol::list_delta::supplied_value_default", retire_with = "protocol::list_delta::supplied_value_retire")]
    pub area: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none", deserialize_with = "protocol::list_delta::supplied_value", deserialize_controlled_with = "protocol::list_delta::supplied_value_controlled", default_controlled = "protocol::list_delta::supplied_value_default", retire_with = "protocol::list_delta::supplied_value_retire")]
    pub iy: Option<f64>,
}

impl RowPatch<FemSection> for Fem2dSectionPatch {
    fn commit_into(&self, row: &mut FemSection, _: ApplyCapability) -> Result<(), MutationApplyError> {
        if let Some(value) = &self.name { row.name = value.clone(); }
        if let Some(value) = self.area { row.area = value; }
        if let Some(value) = self.iy { row.iy = value; }
        Ok(())
    }
    fn absorb(&mut self, later: Self) {
        self.name = later.name.or_else(|| self.name.take());
        self.area = later.area.or(self.area);
        self.iy = later.iy.or(self.iy);
    }
    fn inverse(&self, row: &FemSection) -> Self {
        Self { name: self.name.as_ref().map(|_| row.name.clone()), area: self.area.map(|_| row.area), iy: self.iy.map(|_| row.iy) }
    }
    fn is_empty(&self) -> bool { self.name.is_none() && self.area.is_none() && self.iy.is_none() }
}

impl Fem2dSectionPatch {
    /// 🔁️ Explicit replacement owns every mutable section field.
    pub fn replacement(row: &FemSection) -> Self {
        Self { name: Some(row.name.clone()), area: Some(row.area), iy: Some(row.iy) }
    }
    /// 🔎️ Reads a complete validation candidate without mutating a projection.
    pub fn candidate(&self, base: &FemSection) -> FemSection {
        FemSection { id: base.id.clone(), name: self.name.clone().unwrap_or_else(|| base.name.clone()), area: self.area.unwrap_or(base.area), iy: self.iy.unwrap_or(base.iy) }
    }
}

/// 🧩️ Sparse region fields preserve independent event intent.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct Fem2dRegionPatch {
    #[value(skip_serializing_if = "Option::is_none", deserialize_with = "protocol::list_delta::supplied_value", deserialize_controlled_with = "protocol::list_delta::supplied_value_controlled", default_controlled = "protocol::list_delta::supplied_value_default", retire_with = "protocol::list_delta::supplied_value_retire")]
    pub name: Option<String>,
    #[value(skip_serializing_if = "Option::is_none", deserialize_with = "protocol::list_delta::supplied_value", deserialize_controlled_with = "protocol::list_delta::supplied_value_controlled", default_controlled = "protocol::list_delta::supplied_value_default", retire_with = "protocol::list_delta::supplied_value_retire")]
    pub outline: Option<Vec<[f64; 2]>>,
    #[value(skip_serializing_if = "Option::is_none", deserialize_with = "protocol::list_delta::supplied_value", deserialize_controlled_with = "protocol::list_delta::supplied_value_controlled", default_controlled = "protocol::list_delta::supplied_value_default", retire_with = "protocol::list_delta::supplied_value_retire")]
    pub holes: Option<Vec<Vec<[f64; 2]>>>,
    #[value(skip_serializing_if = "Option::is_none", deserialize_with = "protocol::list_delta::supplied_value", deserialize_controlled_with = "protocol::list_delta::supplied_value_controlled", default_controlled = "protocol::list_delta::supplied_value_default", retire_with = "protocol::list_delta::supplied_value_retire")]
    pub thickness: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none", deserialize_with = "protocol::list_delta::supplied_value", deserialize_controlled_with = "protocol::list_delta::supplied_value_controlled", default_controlled = "protocol::list_delta::supplied_value_default", retire_with = "protocol::list_delta::supplied_value_retire")]
    pub material_id: Option<String>,
    #[value(skip_serializing_if = "Option::is_none", deserialize_with = "protocol::list_delta::supplied_value", deserialize_controlled_with = "protocol::list_delta::supplied_value_controlled", default_controlled = "protocol::list_delta::supplied_value_default", retire_with = "protocol::list_delta::supplied_value_retire")]
    pub mesh_size: Option<f64>,
}
impl RowPatch<FemRegion> for Fem2dRegionPatch {
    fn commit_into(&self, row: &mut FemRegion, _: ApplyCapability) -> Result<(), MutationApplyError> {
        if let Some(value) = &self.name { row.name = value.clone(); }
        if let Some(value) = &self.outline { row.outline = value.clone(); }
        if let Some(value) = &self.holes { row.holes = value.clone(); }
        if let Some(value) = &self.thickness { row.thickness = value.clone(); }
        if let Some(value) = &self.material_id { row.material_id = value.clone(); }
        if let Some(value) = &self.mesh_size { row.mesh_size = value.clone(); }
        Ok(())
    }
    fn absorb(&mut self, later: Self) {
        self.name = later.name.or_else(|| self.name.take());
        self.outline = later.outline.or_else(|| self.outline.take());
        self.holes = later.holes.or_else(|| self.holes.take());
        self.thickness = later.thickness.or_else(|| self.thickness.take());
        self.material_id = later.material_id.or_else(|| self.material_id.take());
        self.mesh_size = later.mesh_size.or_else(|| self.mesh_size.take());
    }
    fn inverse(&self, row: &FemRegion) -> Self { Self { name: self.name.as_ref().map(|_| row.name.clone()), outline: self.outline.as_ref().map(|_| row.outline.clone()), holes: self.holes.as_ref().map(|_| row.holes.clone()), thickness: self.thickness.as_ref().map(|_| row.thickness.clone()), material_id: self.material_id.as_ref().map(|_| row.material_id.clone()), mesh_size: self.mesh_size.as_ref().map(|_| row.mesh_size.clone()) } }
    fn is_empty(&self) -> bool { self.name.is_none() && self.outline.is_none() && self.holes.is_none() && self.thickness.is_none() && self.material_id.is_none() && self.mesh_size.is_none() }
}
impl Fem2dRegionPatch {
    /// 🔁️ An explicit replacement authors all mutable fields.
    pub fn replacement(row: &FemRegion) -> Self { Self { name: Some(row.name.clone()), outline: Some(row.outline.clone()), holes: Some(row.holes.clone()), thickness: Some(row.thickness.clone()), material_id: Some(row.material_id.clone()), mesh_size: Some(row.mesh_size.clone()) } }
    /// 🔎️ Builds the current validation candidate.
    pub fn candidate(&self, base: &FemRegion) -> FemRegion { FemRegion { id: base.id.clone(), name: self.name.clone().unwrap_or_else(|| base.name.clone()), outline: self.outline.clone().unwrap_or_else(|| base.outline.clone()), holes: self.holes.clone().unwrap_or_else(|| base.holes.clone()), thickness: self.thickness.clone().unwrap_or_else(|| base.thickness.clone()), material_id: self.material_id.clone().unwrap_or_else(|| base.material_id.clone()), mesh_size: self.mesh_size.clone().unwrap_or_else(|| base.mesh_size.clone()) } }
}

/// 🧩️ Support intent owns a node reference, a whole set, or individual DOF memberships.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct Fem2dSupportPatch {
    #[value(skip_serializing_if = "Option::is_none", deserialize_with = "protocol::list_delta::supplied_value", deserialize_controlled_with = "protocol::list_delta::supplied_value_controlled", default_controlled = "protocol::list_delta::supplied_value_default", retire_with = "protocol::list_delta::supplied_value_retire")]
    pub node_id: Option<String>,
    #[value(skip_serializing_if = "Option::is_none", deserialize_with = "protocol::list_delta::supplied_value", deserialize_controlled_with = "protocol::list_delta::supplied_value_controlled", default_controlled = "protocol::list_delta::supplied_value_default", retire_with = "protocol::list_delta::supplied_value_retire")]
    pub fixed: Option<Vec<crate::standards::v1::subsets::any::schema::structure::FemDof>>,
    #[value(skip_serializing_if = "Option::is_none", deserialize_with = "protocol::list_delta::supplied_value", deserialize_controlled_with = "protocol::list_delta::supplied_value_controlled", default_controlled = "protocol::list_delta::supplied_value_default", retire_with = "protocol::list_delta::supplied_value_retire")]
    pub tx: Option<bool>,
    #[value(skip_serializing_if = "Option::is_none", deserialize_with = "protocol::list_delta::supplied_value", deserialize_controlled_with = "protocol::list_delta::supplied_value_controlled", default_controlled = "protocol::list_delta::supplied_value_default", retire_with = "protocol::list_delta::supplied_value_retire")]
    pub ty: Option<bool>,
    #[value(skip_serializing_if = "Option::is_none", deserialize_with = "protocol::list_delta::supplied_value", deserialize_controlled_with = "protocol::list_delta::supplied_value_controlled", default_controlled = "protocol::list_delta::supplied_value_default", retire_with = "protocol::list_delta::supplied_value_retire")]
    pub rz: Option<bool>,
}
impl RowPatch<FemSupport> for Fem2dSupportPatch {
    fn commit_into(&self, row: &mut FemSupport, _: ApplyCapability) -> Result<(), MutationApplyError> {
        if !self.valid() { return Err(MutationApplyError { code: "mutation.apply.invalid-patch".into(), message: "Support set and membership setters must be disjoint and canonically ordered.".into(), target: vec![row.id.clone()] }); }
        if let Some(value) = &self.node_id { row.node_id = value.clone(); }
        row.fixed = self.memberships(&row.fixed);
        Ok(())
    }
    fn absorb(&mut self, later: Self) {
        if later.fixed.is_some() { self.tx = None; self.ty = None; self.rz = None; }
        if self.fixed.is_some() && later.fixed.is_none() { self.fixed = Some(later.memberships(self.fixed.as_ref().unwrap())); self.node_id = later.node_id.or_else(|| self.node_id.take()); return; }
        self.node_id = later.node_id.or_else(|| self.node_id.take());
        self.fixed = later.fixed.or_else(|| self.fixed.take());
        self.tx = later.tx.or_else(|| self.tx.take());
        self.ty = later.ty.or_else(|| self.ty.take());
        self.rz = later.rz.or_else(|| self.rz.take());
    }
    fn inverse(&self, row: &FemSupport) -> Self { Self { node_id: self.node_id.as_ref().map(|_| row.node_id.clone()), fixed: self.fixed.as_ref().map(|_| row.fixed.clone()), tx: self.tx.map(|_| row.fixed.contains(&crate::standards::v1::subsets::any::schema::structure::FemDof::Tx)), ty: self.ty.map(|_| row.fixed.contains(&crate::standards::v1::subsets::any::schema::structure::FemDof::Ty)), rz: self.rz.map(|_| row.fixed.contains(&crate::standards::v1::subsets::any::schema::structure::FemDof::Rz)) } }
    fn is_empty(&self) -> bool { self.node_id.is_none() && self.fixed.is_none() && self.tx.is_none() && self.ty.is_none() && self.rz.is_none() }
}
impl Fem2dSupportPatch {
    /// 🛂️ Whole sets and membership setters are disjoint authored modes.
    pub fn valid(&self) -> bool { self.fixed.as_ref().is_none_or(|fixed| crate::standards::v1::subsets::any::schema::structure::FemDof::canonical(fixed) && self.tx.is_none() && self.ty.is_none() && self.rz.is_none()) }
    fn memberships(&self, base: &[crate::standards::v1::subsets::any::schema::structure::FemDof]) -> Vec<crate::standards::v1::subsets::any::schema::structure::FemDof> {
        let mut fixed = self.fixed.as_deref().unwrap_or(base).to_vec();
        if let Some(value) = self.tx { fixed.retain(|entry| *entry != crate::standards::v1::subsets::any::schema::structure::FemDof::Tx); if value { fixed.push(crate::standards::v1::subsets::any::schema::structure::FemDof::Tx); } }
        if let Some(value) = self.ty { fixed.retain(|entry| *entry != crate::standards::v1::subsets::any::schema::structure::FemDof::Ty); if value { fixed.push(crate::standards::v1::subsets::any::schema::structure::FemDof::Ty); } }
        if let Some(value) = self.rz { fixed.retain(|entry| *entry != crate::standards::v1::subsets::any::schema::structure::FemDof::Rz); if value { fixed.push(crate::standards::v1::subsets::any::schema::structure::FemDof::Rz); } }
        crate::standards::v1::subsets::any::schema::structure::FemDof::ALL.into_iter().filter(|candidate| fixed.contains(candidate)).collect()
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
pub struct Fem2dElementPatch {
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
}
impl RowPatch<FemElement> for Fem2dElementPatch {
    fn commit_into(&self, row: &mut FemElement, _: ApplyCapability) -> Result<(), MutationApplyError> { *row = self.candidate(row)?; Ok(()) }
    fn absorb(&mut self, later: Self) {

        self.kind = later.kind.or_else(|| self.kind.take());
        self.start = later.start.or_else(|| self.start.take());
        self.end = later.end.or_else(|| self.end.take());
        self.material_id = later.material_id.or_else(|| self.material_id.take());
        self.section_id = later.section_id.or_else(|| self.section_id.take());
    }
    fn inverse(&self, row: &FemElement) -> Self { let original = Self::replacement(row); Self { kind: self.kind.as_ref().and(original.kind), start: self.start.as_ref().and(original.start), end: self.end.as_ref().and(original.end), material_id: self.material_id.as_ref().and(original.material_id), section_id: self.section_id.as_ref().and(original.section_id) } }
    fn is_empty(&self) -> bool { self.kind.is_none() && self.start.is_none() && self.end.is_none() && self.material_id.is_none() && self.section_id.is_none() }
}
impl Fem2dElementPatch {
    /// 🔁️ Replacement authors all mutable properties and the variant.
    pub fn replacement(row: &FemElement) -> Self {
        let (start, end, material_id, section_id, kind) = match row {
            FemElement::Bar { start, end, material_id, section_id, .. } => (start, end, material_id, section_id, "bar"),
            FemElement::Beam { start, end, material_id, section_id, .. } => (start, end, material_id, section_id, "beam"),
        };
        let mut patch = Self { kind: Some(kind.into()), start: Some(start.clone()), end: Some(end.clone()), material_id: Some(material_id.clone()), section_id: Some(section_id.clone()), ..Default::default() };

        patch
    }
    /// 🔎️ Common fields stay live and a newly created frame starts with zero roll.
    pub fn candidate(&self, base: &FemElement) -> Result<FemElement, MutationApplyError> {
        let (id, start, end, material_id, section_id, kind) = match base {
            FemElement::Bar { id, start, end, material_id, section_id } => (id, start, end, material_id, section_id, "bar"),
            FemElement::Beam { id, start, end, material_id, section_id } => (id, start, end, material_id, section_id, "beam"),
        };
        let kind = self.kind.as_deref().unwrap_or(kind);
        let (id, start, end, material_id, section_id) = (id.clone(), self.start.clone().unwrap_or_else(|| start.clone()), self.end.clone().unwrap_or_else(|| end.clone()), self.material_id.clone().unwrap_or_else(|| material_id.clone()), self.section_id.clone().unwrap_or_else(|| section_id.clone()));
        match kind {
            "bar" => {  Ok(FemElement::Bar { id, start, end, material_id, section_id }) },
            "beam" => Ok(FemElement::Beam { id, start, end, material_id, section_id }),
            _ => Err(MutationApplyError { code: "mutation.apply.variant-mismatch".into(), message: "Unknown element variant.".into(), target: vec![id] }),
        }
    }
}
/// 🧩️ Typed load fields use their variant tag as a replay precondition.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslEnum, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
#[dsl(field)]
pub enum Fem2dLoadPatch {
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
    },
    #[value(rename_all = "camelCase")]
    Area {
        #[value(skip_serializing_if = "Option::is_none", deserialize_with = "protocol::list_delta::supplied_value", deserialize_controlled_with = "protocol::list_delta::supplied_value_controlled", default_controlled = "protocol::list_delta::supplied_value_default", retire_with = "protocol::list_delta::supplied_value_retire")]
        pressure: Option<f64>,
        #[value(skip_serializing_if = "Option::is_none", deserialize_with = "protocol::list_delta::supplied_value", deserialize_controlled_with = "protocol::list_delta::supplied_value_controlled", default_controlled = "protocol::list_delta::supplied_value_default", retire_with = "protocol::list_delta::supplied_value_retire")]
        region_id: Option<String>,
    },
}
impl RowPatch<FemLoad> for Fem2dLoadPatch {
    fn commit_into(&self, row: &mut FemLoad, _: ApplyCapability) -> Result<(), MutationApplyError> { *row = self.candidate(row)?; Ok(()) }
    fn absorb(&mut self, later: Self) { match (self, later) {
            (Self::Nodal { node_id, dof, value }, Self::Nodal { node_id: next_node_id, dof: next_dof, value: next_value }) => { *node_id = next_node_id.or_else(|| node_id.take()); *dof = next_dof.or_else(|| dof.take()); *value = next_value.or_else(|| value.take()); },
            (Self::MemberUdl { element_id, wx, wy }, Self::MemberUdl { element_id: next_element_id, wx: next_wx, wy: next_wy }) => { *element_id = next_element_id.or_else(|| element_id.take()); *wx = next_wx.or_else(|| wx.take()); *wy = next_wy.or_else(|| wy.take()); },
            (Self::Area { pressure, region_id }, Self::Area { pressure: next_pressure, region_id: next_region_id }) => { *pressure = next_pressure.or_else(|| pressure.take()); *region_id = next_region_id.or_else(|| region_id.take()); },
        (current, next) => *current = next,
    } }
    fn inverse(&self, row: &FemLoad) -> Self { match (self, row) {
            (Self::Nodal { node_id, dof, value }, FemLoad::Nodal { node_id: base_node_id, dof: base_dof, value: base_value, .. }) => Self::Nodal { node_id: node_id.as_ref().map(|_| base_node_id.clone()), dof: dof.as_ref().map(|_| base_dof.clone()), value: value.as_ref().map(|_| base_value.clone()) },
            (Self::MemberUdl { element_id, wx, wy }, FemLoad::MemberUdl { element_id: base_element_id, wx: base_wx, wy: base_wy, .. }) => Self::MemberUdl { element_id: element_id.as_ref().map(|_| base_element_id.clone()), wx: wx.as_ref().map(|_| base_wx.clone()), wy: wy.as_ref().map(|_| base_wy.clone()) },
            (Self::Area { pressure, region_id }, FemLoad::Area { pressure: base_pressure, region_id: base_region_id, .. }) => Self::Area { pressure: pressure.as_ref().map(|_| base_pressure.clone()), region_id: region_id.as_ref().map(|_| base_region_id.clone()) },
        _ => self.clone(),
    } }
    fn is_empty(&self) -> bool { match self {
            Self::Nodal { node_id, dof, value } => node_id.is_none() && dof.is_none() && value.is_none(),
            Self::MemberUdl { element_id, wx, wy } => element_id.is_none() && wx.is_none() && wy.is_none(),
            Self::Area { pressure, region_id } => pressure.is_none() && region_id.is_none(),
    } }
}
impl Fem2dLoadPatch {
    /// 🧬️ Starts an empty patch in the current load's admissible variant.
    pub fn empty_for(row: &FemLoad) -> Self { match row {
            FemLoad::Nodal { .. } => Self::Nodal { node_id: None, dof: None, value: None },
            FemLoad::MemberUdl { .. } => Self::MemberUdl { element_id: None, wx: None, wy: None },
            FemLoad::Area { .. } => Self::Area { pressure: None, region_id: None },
    } }
    /// 🔁️ A replacement owns every field in the specified variant.
    pub fn replacement(row: &FemLoad) -> Self { match row {
            FemLoad::Nodal { node_id, dof, value, .. } => Self::Nodal { node_id: Some(node_id.clone()), dof: Some(dof.clone()), value: Some(value.clone()) },
            FemLoad::MemberUdl { element_id, wx, wy, .. } => Self::MemberUdl { element_id: Some(element_id.clone()), wx: Some(wx.clone()), wy: Some(wy.clone()) },
            FemLoad::Area { pressure, region_id, .. } => Self::Area { pressure: Some(pressure.clone()), region_id: Some(region_id.clone()) },
    } }
    /// 🔎️ Reads sparse fields only when the current variant still matches.
    pub fn candidate(&self, row: &FemLoad) -> Result<FemLoad, MutationApplyError> { match (self, row) {
            (Self::Nodal { node_id, dof, value }, FemLoad::Nodal { id, node_id: base_node_id, dof: base_dof, value: base_value }) => Ok(FemLoad::Nodal { id: id.clone(), node_id: node_id.clone().unwrap_or_else(|| base_node_id.clone()), dof: dof.clone().unwrap_or_else(|| base_dof.clone()), value: value.clone().unwrap_or_else(|| base_value.clone()) }),
            (Self::MemberUdl { element_id, wx, wy }, FemLoad::MemberUdl { id, element_id: base_element_id, wx: base_wx, wy: base_wy }) => Ok(FemLoad::MemberUdl { id: id.clone(), element_id: element_id.clone().unwrap_or_else(|| base_element_id.clone()), wx: wx.clone().unwrap_or_else(|| base_wx.clone()), wy: wy.clone().unwrap_or_else(|| base_wy.clone()) }),
            (Self::Area { pressure, region_id }, FemLoad::Area { id, pressure: base_pressure, region_id: base_region_id }) => Ok(FemLoad::Area { id: id.clone(), pressure: pressure.clone().unwrap_or_else(|| base_pressure.clone()), region_id: region_id.clone().unwrap_or_else(|| base_region_id.clone()) }),
        _ => Err(MutationApplyError { code: "mutation.apply.variant-mismatch".into(), message: "Load field intent requires its original variant.".into(), target: vec![load_id(row).into()] }),
    } }
}

impl RowPatch<FemLoadCase> for Fem2dLoadCasePatch {
    fn commit_into(&self, row: &mut FemLoadCase, capability: ApplyCapability) -> Result<(), MutationApplyError> {
        if let Some(name) = &self.name {
            row.name = name.clone();
        }
        if let Some(self_weight) = self.self_weight {
            row.self_weight = self_weight;
        }
        if let Some(delta) = &self.loads {
            let next = delta.commit_onto(&row.loads, capability).map_err(|error| error.under(["loads"]))?;
            Fem2dLoadsDelta::retire_list(std::mem::replace(&mut row.loads, next));
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


}
impl protocol::ColdMutationDiff<Fem2dSnapshotValue> for Fem2dDiff {

    fn absorb(&mut self, other: Self) {
        absorb_lists!(self, other, nodes, elements, regions, materials, sections, supports, load_cases, combinations);
        if let Some(later) = other.analysis {
            match &mut self.analysis {
                Some(earlier) => earlier.composed(later),
                None => self.analysis = Some(later),
            }
        }
    }

    fn apply(&self, next: &mut Fem2dSnapshotValue, capability: ApplyCapability) -> MutationApplyResult<()> {
        let nodes = self.nodes.as_ref().map(|delta| delta.commit_onto(&next.nodes, capability).map_err(|error| error.under(["nodes"]))).transpose()?.map(|value| protocol::PreparedField::new(value, Fem2dNodesDelta::retire_list, capability));
        let elements = self.elements.as_ref().map(|delta| delta.commit_onto(&next.elements, capability).map_err(|error| error.under(["elements"]))).transpose()?.map(|value| protocol::PreparedField::new(value, Fem2dElementsDelta::retire_list, capability));
        let regions = self.regions.as_ref().map(|delta| delta.commit_onto(&next.regions, capability).map_err(|error| error.under(["regions"]))).transpose()?.map(|value| protocol::PreparedField::new(value, Fem2dRegionsDelta::retire_list, capability));
        let materials = self.materials.as_ref().map(|delta| delta.commit_onto(&next.materials, capability).map_err(|error| error.under(["materials"]))).transpose()?.map(|value| protocol::PreparedField::new(value, Fem2dMaterialsDelta::retire_list, capability));
        let sections = self.sections.as_ref().map(|delta| delta.commit_onto(&next.sections, capability).map_err(|error| error.under(["sections"]))).transpose()?.map(|value| protocol::PreparedField::new(value, Fem2dSectionsDelta::retire_list, capability));
        let supports = self.supports.as_ref().map(|delta| delta.commit_onto(&next.supports, capability).map_err(|error| error.under(["supports"]))).transpose()?.map(|value| protocol::PreparedField::new(value, Fem2dSupportsDelta::retire_list, capability));
        let load_cases = self.load_cases.as_ref().map(|delta| delta.commit_onto(&next.load_cases, capability).map_err(|error| error.under(["loadCases"]))).transpose()?.map(|value| protocol::PreparedField::new(value, Fem2dLoadCasesDelta::retire_list, capability));
        let combinations = self.combinations.as_ref().map(|delta| delta.commit_onto(&next.combinations, capability).map_err(|error| error.under(["combinations"]))).transpose()?.map(|value| protocol::PreparedField::new(value, Fem2dCombinationsDelta::retire_list, capability));
        let analysis = self.analysis.as_ref().map(|patch| patch.applied(&next.analysis));
        if let Some(value) = nodes { value.install(&mut next.nodes); }
        if let Some(value) = elements { value.install(&mut next.elements); }
        if let Some(value) = regions { value.install(&mut next.regions); }
        if let Some(value) = materials { value.install(&mut next.materials); }
        if let Some(value) = sections { value.install(&mut next.sections); }
        if let Some(value) = supports { value.install(&mut next.supports); }
        if let Some(value) = load_cases { value.install(&mut next.load_cases); }
        if let Some(value) = combinations { value.install(&mut next.combinations); }
        if let Some(value) = analysis { next.analysis = value; }
        Ok(())
    }
}

impl DiffAlgebra<Fem2dSnapshot> for Fem2dDiff {
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
impl protocol::ColdDiffAlgebra<Fem2dSnapshotValue> for Fem2dDiff {
    fn inverse(&self, base: &Fem2dSnapshotValue) -> Self {
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
}
//#endregion 🔖️Apply

protocol::retained_row_patch!(pub retained_material_fields for Fem2dMaterialPatch => FemMaterialValue {name:String,e:f64,nu:f64,rho:f64});
protocol::retained_row_patch!(pub retained_node_fields for Fem2dNodePatch => FemNodeValue {x:f64,y:f64});
protocol::retained_row_patch!(pub retained_section_fields for Fem2dSectionPatch => FemSection {name:String,area:f64,iy:f64});
protocol::retained_row_patch!(pub retained_region_fields for Fem2dRegionPatch => FemRegion {name:String,outline:Vec<[f64;2]>,holes:Vec<Vec<[f64;2]>>,thickness:f64,material_id:String,mesh_size:f64});

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

impl DiffAlgebra<Fem2dSnapshotValue> for Fem2dDiff {fn is_empty(&self)->bool{<Self as DiffAlgebra<Fem2dSnapshot>>::is_empty(self)}}
impl MutationDiff<Fem2dSnapshotValue> for Fem2dDiff {}
impl Fem2dDiff {pub fn is_empty(&self)->bool{<Self as DiffAlgebra<Fem2dSnapshot>>::is_empty(self)}}

impl semio_s_fem_material::patch::NativeMaterialPatch<3> for Fem2dMaterialPatch{fn name(&self)->Option<&String>{self.name.as_ref()}fn numbers(&self)->[Option<f64>;3]{[self.e,self.nu,self.rho]}}

/// 🔑️ Explicit cold Material values use the existing local key marker contract.
#[derive(Clone,Debug,PartialEq)]
pub struct Fem2dMaterialKey;
impl protocol::list_delta::KeyOf<FemMaterialValue> for Fem2dMaterialKey{type Key=String;fn key_of(row:&FemMaterialValue)->String{row.id.clone()}fn retire_cold(row:FemMaterialValue){<FemMaterialValue as semio_framework_value::FromValue>::retire_decoded(row)}}
