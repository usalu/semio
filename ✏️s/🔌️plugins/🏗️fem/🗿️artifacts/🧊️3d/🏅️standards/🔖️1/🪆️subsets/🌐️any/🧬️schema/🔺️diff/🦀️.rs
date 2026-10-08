//! 🧬️ Fem3d diff schema — sparse id-keyed delta over the artifact.

use crate::{FemAnalysisSettings, FemCombination, FemElement, FemLoad, FemLoadCase, FemMaterial, FemNode, FemSection, FemSolid, FemSupport};
use ::semio_framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Diff
/// 🔺️ Sparse delta for the fem3d artifact: per-collection id-keyed rows plus an owned-field analysis patch.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, ArtifactSchema, semio_framework_dsl_record_derive::DslRecord)]
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
/// 🧩 Identified-collection delta for `nodes`.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct Fem3dNodesDelta {
    pub added: Vec<FemNode>,
    pub removed: Vec<String>,
    pub patched: Vec<Fem3dNodesPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `nodes` entry (whole-entity replacement).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct Fem3dNodesPatchEntry {
    pub id: String,
    pub item: FemNode,
}

/// 🧩 Identified-collection delta for `elements`.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct Fem3dElementsDelta {
    pub added: Vec<FemElement>,
    pub removed: Vec<String>,
    pub patched: Vec<Fem3dElementsPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `elements` entry (whole-entity replacement).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct Fem3dElementsPatchEntry {
    pub id: String,
    pub item: FemElement,
}

/// 🧩 Identified-collection delta for `materials`.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct Fem3dMaterialsDelta {
    pub added: Vec<FemMaterial>,
    pub removed: Vec<String>,
    pub patched: Vec<Fem3dMaterialsPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `materials` entry (whole-entity replacement).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct Fem3dMaterialsPatchEntry {
    pub id: String,
    pub item: FemMaterial,
}

/// 🧩 Identified-collection delta for `sections`.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct Fem3dSectionsDelta {
    pub added: Vec<FemSection>,
    pub removed: Vec<String>,
    pub patched: Vec<Fem3dSectionsPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `sections` entry (whole-entity replacement).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct Fem3dSectionsPatchEntry {
    pub id: String,
    pub item: FemSection,
}

/// 🧩 Identified-collection delta for `solids`.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct Fem3dSolidsDelta {
    pub added: Vec<FemSolid>,
    pub removed: Vec<String>,
    pub patched: Vec<Fem3dSolidsPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `solids` entry (whole-entity replacement).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct Fem3dSolidsPatchEntry {
    pub id: String,
    pub item: FemSolid,
}

/// 🧩 Identified-collection delta for `supports`.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct Fem3dSupportsDelta {
    pub added: Vec<FemSupport>,
    pub removed: Vec<String>,
    pub patched: Vec<Fem3dSupportsPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `supports` entry (whole-entity replacement).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct Fem3dSupportsPatchEntry {
    pub id: String,
    pub item: FemSupport,
}

/// 🧩 Identified-collection delta for `loadCases`.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct Fem3dLoadCasesDelta {
    pub added: Vec<FemLoadCase>,
    pub removed: Vec<String>,
    pub patched: Vec<Fem3dLoadCasesPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `loadCases` entry: only the fields the mutation owns.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct Fem3dLoadCasesPatchEntry {
    pub id: String,
    pub patch: Fem3dLoadCasePatch,
}

/// 🩹 Owned-field patch of one load case: a rename, a self-weight switch and/or keyed load rows.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct Fem3dLoadCasePatch {
    pub name: Option<String>,
    pub self_weight: Option<bool>,
    pub loads: Option<Fem3dLoadsDelta>,
}

/// 🧩 Identified-collection delta for the `loads` of one load case.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct Fem3dLoadsDelta {
    #[dsl(statements, block)]
    pub added: Vec<FemLoad>,
    pub removed: Vec<String>,
    pub patched: Vec<Fem3dLoadsPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `loads` entry (whole-load replacement).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct Fem3dLoadsPatchEntry {
    pub id: String,
    #[dsl(statements)]
    pub item: Box<FemLoad>,
}

/// 🧩 Identified-collection delta for `combinations`.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct Fem3dCombinationsDelta {
    pub added: Vec<FemCombination>,
    pub removed: Vec<String>,
    pub patched: Vec<Fem3dCombinationsPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `combinations` entry (whole-entity replacement).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct Fem3dCombinationsPatchEntry {
    pub id: String,
    pub item: FemCombination,
}

/// 🎛️ Owned-field patch of the analysis settings: exactly the fields the mutation sets.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct Fem3dAnalysisPatch {
    pub modal_count: Option<usize>,
    pub buckling_count: Option<usize>,
    pub deformation_scale: Option<f64>,
}
//#endregion 🔖️DeltaHelpers

use crate::element_id;
use crate::load_id;
use crate::Fem3dSnapshot;
use protocol::{DiffAlgebra, MutationApplyError, MutationApplyResult, MutationDiff};

//#region 🔖️Insertion
/// 📍 Complete id order that places `id` at `index` among `ids`; `None` when it lands last, because appending is already the natural order of an added row.
pub fn insertion_order<'a>(ids: impl IntoIterator<Item = &'a str>, id: &str, index: Option<usize>) -> Option<Vec<String>> {
    let at = index?;
    let mut order: Vec<String> = ids.into_iter().map(str::to_owned).collect();
    (at < order.len()).then(|| {
        order.insert(at, id.to_owned());
        order
    })
}
//#endregion 🔖️Insertion

//#region 🔖️RowAlgebra
pub(crate) trait HasId {
    fn id(&self) -> &str;
}

impl HasId for FemNode {
    fn id(&self) -> &str {
        &self.id
    }
}

impl HasId for FemElement {
    fn id(&self) -> &str {
        element_id(self)
    }
}

impl HasId for FemMaterial {
    fn id(&self) -> &str {
        &self.id
    }
}

impl HasId for FemSection {
    fn id(&self) -> &str {
        &self.id
    }
}

impl HasId for FemSupport {
    fn id(&self) -> &str {
        &self.id
    }
}

impl HasId for FemLoadCase {
    fn id(&self) -> &str {
        &self.id
    }
}

impl HasId for FemLoad {
    fn id(&self) -> &str {
        load_id(self)
    }
}

impl HasId for FemCombination {
    fn id(&self) -> &str {
        &self.id
    }
}

impl HasId for FemSolid {
    fn id(&self) -> &str {
        &self.id
    }
}

/// 🩹 How one patch row edits one item: applied forward, inverted against a base item, composed with a later row.
pub(crate) trait RowPatch<T>: Clone + Sized {
    fn applied(&self, item: &T) -> MutationApplyResult<T>;
    fn inverse_against(&self, base: &T) -> Self;
    fn composed(&mut self, later: Self);
    fn between(base: &T, other: &T) -> Option<Self>;
}

impl<T: Clone + PartialEq> RowPatch<T> for T {
    fn applied(&self, _item: &T) -> MutationApplyResult<T> {
        Ok(self.clone())
    }
    fn inverse_against(&self, base: &T) -> Self {
        base.clone()
    }
    fn composed(&mut self, later: Self) {
        *self = later;
    }
    fn between(base: &T, other: &T) -> Option<Self> {
        (base != other).then(|| other.clone())
    }
}

impl RowPatch<FemLoad> for Box<FemLoad> {
    fn applied(&self, _item: &FemLoad) -> MutationApplyResult<FemLoad> {
        Ok((**self).clone())
    }
    fn inverse_against(&self, base: &FemLoad) -> Self {
        Box::new(base.clone())
    }
    fn composed(&mut self, later: Self) {
        *self = later;
    }
    fn between(base: &FemLoad, other: &FemLoad) -> Option<Self> {
        (base != other).then(|| Box::new(other.clone()))
    }
}

impl RowPatch<FemLoadCase> for Fem3dLoadCasePatch {
    fn applied(&self, item: &FemLoadCase) -> MutationApplyResult<FemLoadCase> {
        let loads = match &self.loads {
            Some(delta) => apply_delta(&item.loads, delta).map_err(|error| error.under(["loads"]))?,
            None => item.loads.clone(),
        };
        Ok(FemLoadCase { id: item.id.clone(), name: self.name.clone().unwrap_or_else(|| item.name.clone()), loads, self_weight: self.self_weight.unwrap_or(item.self_weight) })
    }
    fn inverse_against(&self, base: &FemLoadCase) -> Self {
        Self {
            name: self.name.as_ref().map(|_| base.name.clone()),
            self_weight: self.self_weight.map(|_| base.self_weight),
            loads: self.loads.as_ref().map(|delta| inverse_delta(delta, &base.loads)),
        }
    }
    fn composed(&mut self, later: Self) {
        self.name = later.name.or_else(|| self.name.take());
        self.self_weight = later.self_weight.or(self.self_weight);
        self.loads = match (self.loads.take(), later.loads) {
            (Some(first), Some(second)) => Some(absorb_delta(first, second)),
            (first, None) => first,
            (None, second) => second,
        };
    }
    fn between(base: &FemLoadCase, other: &FemLoadCase) -> Option<Self> {
        let patch = Self {
            name: (base.name != other.name).then(|| other.name.clone()),
            self_weight: (base.self_weight != other.self_weight).then_some(other.self_weight),
            loads: between_delta(&base.loads, &other.loads),
        };
        (patch != Self::default()).then_some(patch)
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
    fn between(base: &FemAnalysisSettings, other: &FemAnalysisSettings) -> Option<Self> {
        let patch = Self {
            modal_count: (base.modal_count != other.modal_count).then_some(other.modal_count),
            buckling_count: (base.buckling_count != other.buckling_count).then_some(other.buckling_count),
            deformation_scale: (base.deformation_scale != other.deformation_scale).then_some(other.deformation_scale),
        };
        (patch != Self::default()).then_some(patch)
    }
}

/// 🧩 The shared shape of every id-keyed collection delta.
pub(crate) trait Delta: Default + Clone {
    type Item: HasId + Clone;
    type Patch: RowPatch<Self::Item>;
    fn added(&self) -> &[Self::Item];
    fn removed(&self) -> &[String];
    fn patched(&self) -> Vec<(&str, &Self::Patch)>;
    fn reordered(&self) -> Option<&[String]>;
    fn from_parts(added: Vec<Self::Item>, removed: Vec<String>, patched: Vec<(String, Self::Patch)>, reordered: Option<Vec<String>>) -> Self;
}

macro_rules! impl_delta {
    ($delta:ty, $item:ty, $patch:ty, $entry:ident, $field:ident) => {
        impl Delta for $delta {
            type Item = $item;
            type Patch = $patch;
            fn added(&self) -> &[$item] {
                &self.added
            }
            fn removed(&self) -> &[String] {
                &self.removed
            }
            fn patched(&self) -> Vec<(&str, &$patch)> {
                self.patched.iter().map(|entry| (entry.id.as_str(), &entry.$field)).collect()
            }
            fn reordered(&self) -> Option<&[String]> {
                self.reordered.as_deref()
            }
            fn from_parts(added: Vec<$item>, removed: Vec<String>, patched: Vec<(String, $patch)>, reordered: Option<Vec<String>>) -> Self {
                Self { added, removed, patched: patched.into_iter().map(|(id, $field)| $entry { id, $field }).collect(), reordered }
            }
        }
    };
}

impl_delta!(Fem3dNodesDelta, FemNode, FemNode, Fem3dNodesPatchEntry, item);
impl_delta!(Fem3dElementsDelta, FemElement, FemElement, Fem3dElementsPatchEntry, item);
impl_delta!(Fem3dSolidsDelta, FemSolid, FemSolid, Fem3dSolidsPatchEntry, item);
impl_delta!(Fem3dMaterialsDelta, FemMaterial, FemMaterial, Fem3dMaterialsPatchEntry, item);
impl_delta!(Fem3dSectionsDelta, FemSection, FemSection, Fem3dSectionsPatchEntry, item);
impl_delta!(Fem3dSupportsDelta, FemSupport, FemSupport, Fem3dSupportsPatchEntry, item);
impl_delta!(Fem3dLoadCasesDelta, FemLoadCase, Fem3dLoadCasePatch, Fem3dLoadCasesPatchEntry, patch);
impl_delta!(Fem3dLoadsDelta, FemLoad, Box<FemLoad>, Fem3dLoadsPatchEntry, item);
impl_delta!(Fem3dCombinationsDelta, FemCombination, FemCombination, Fem3dCombinationsPatchEntry, item);
//#endregion 🔖️RowAlgebra

//#region 🔖️DeltaAlgebra
fn rejection(code: &str, message: &str, at: [String; 2]) -> MutationApplyError {
    MutationApplyError::new(code, message).at(at)
}

fn apply_delta<D: Delta>(items: &[D::Item], delta: &D) -> MutationApplyResult<Vec<D::Item>> {
    for (index, id) in delta.removed().iter().enumerate() {
        if !items.iter().any(|item| item.id() == id) {
            return Err(rejection("mutation.apply.missing-target", "removed item does not exist", ["removed".into(), index.to_string()]));
        }
        if delta.removed()[..index].contains(id) {
            return Err(rejection("mutation.apply.duplicate-target", "item is removed more than once", ["removed".into(), index.to_string()]));
        }
    }
    for (index, item) in delta.added().iter().enumerate() {
        let survives = items.iter().any(|existing| existing.id() == item.id()) && !delta.removed().iter().any(|id| id == item.id());
        if survives || delta.added()[..index].iter().any(|existing| existing.id() == item.id()) {
            return Err(rejection("mutation.apply.duplicate-target", "added item identity already exists", ["added".into(), index.to_string()]));
        }
    }
    let patched = delta.patched();
    for (index, (id, _)) in patched.iter().enumerate() {
        if !items.iter().any(|existing| existing.id() == *id) {
            return Err(rejection("mutation.apply.missing-target", "patched item does not exist", ["patched".into(), index.to_string()]));
        }
        if delta.removed().iter().any(|removed| removed == id) {
            return Err(rejection("mutation.apply.conflicting-target", "item cannot be removed and patched", ["patched".into(), index.to_string()]));
        }
        if patched[..index].iter().any(|(prior, _)| prior == id) {
            return Err(rejection("mutation.apply.duplicate-target", "item is patched more than once", ["patched".into(), index.to_string()]));
        }
    }
    let mut next: Vec<D::Item> = items.iter().filter(|item| !delta.removed().iter().any(|id| id == item.id())).cloned().collect();
    next.extend(delta.added().iter().cloned());
    for (id, patch) in patched {
        if let Some(position) = next.iter().position(|existing| existing.id() == id) {
            next[position] = patch.applied(&next[position]).map_err(|error| error.under(["patched", id]))?;
        }
    }
    let mut resulting_ids = std::collections::HashSet::new();
    if !next.iter().all(|item| resulting_ids.insert(item.id())) {
        return Err(MutationApplyError::new("mutation.apply.duplicate-target", "resulting collection contains duplicate identities").at(["identities"]));
    }
    if let Some(order) = delta.reordered() {
        if order.len() != next.len() || order.iter().enumerate().any(|(index, id)| order[..index].contains(id) || !next.iter().any(|item| item.id() == id)) {
            return Err(MutationApplyError::new("mutation.apply.invalid-order", "reorder must be a complete unique permutation").at(["reordered"]));
        }
        let mut by_id: std::collections::BTreeMap<String, D::Item> = next.into_iter().map(|item| (item.id().to_string(), item)).collect();
        let mut ordered = Vec::with_capacity(order.len());
        for id in order {
            ordered.push(by_id.remove(id).ok_or_else(|| MutationApplyError::new("mutation.apply.missing-target", "reordered item does not exist").at(["reordered".to_string(), id.clone()]))?);
        }
        next = ordered;
    }
    Ok(next)
}

fn is_empty_delta<D: Delta>(delta: &D) -> bool {
    delta.added().is_empty() && delta.removed().is_empty() && delta.patched().is_empty() && delta.reordered().is_none()
}

enum Net<T, P> {
    Patch(P),
    Remove,
    Add(T),
    Replace(T),
}

/// ➕️ Composes `first` then `second` per id (patch∘patch → one patch, add∘remove → nothing, remove∘add → replace) in a canonical row order.
fn absorb_delta<D: Delta>(first: D, second: D) -> D {
    let mut nets: std::collections::BTreeMap<String, Net<D::Item, D::Patch>> = std::collections::BTreeMap::new();
    let mut appended: Vec<String> = Vec::new();
    let second_removed: Vec<String> = second.removed().to_vec();
    let second_added: Vec<String> = second.added().iter().map(|item| item.id().to_string()).collect();
    let first_order = first.reordered().map(<[String]>::to_vec);
    let second_order = second.reordered().map(<[String]>::to_vec);
    for delta in [first, second] {
        let added = delta.added().to_vec();
        let removed = delta.removed().to_vec();
        let patched: Vec<(String, D::Patch)> = delta.patched().into_iter().map(|(id, patch)| (id.to_string(), patch.clone())).collect();
        for id in removed {
            match nets.remove(&id) {
                Some(Net::Add(_)) => appended.retain(|existing| existing != &id),
                Some(Net::Replace(_)) => {
                    appended.retain(|existing| existing != &id);
                    nets.insert(id, Net::Remove);
                }
                _ => {
                    nets.insert(id, Net::Remove);
                }
            }
        }
        for item in added {
            let id = item.id().to_string();
            let net = match nets.remove(&id) {
                Some(Net::Remove) => Net::Replace(item),
                _ => Net::Add(item),
            };
            appended.retain(|existing| existing != &id);
            appended.push(id.clone());
            nets.insert(id, net);
        }
        for (id, patch) in patched {
            let net = match nets.remove(&id) {
                None => Net::Patch(patch),
                Some(Net::Remove) => Net::Remove,
                Some(Net::Patch(mut earlier)) => {
                    earlier.composed(patch);
                    Net::Patch(earlier)
                }
                Some(Net::Add(item)) => match patch.applied(&item) {
                    Ok(patched) => Net::Add(patched),
                    Err(_) => Net::Add(item),
                },
                Some(Net::Replace(item)) => match patch.applied(&item) {
                    Ok(patched) => Net::Replace(patched),
                    Err(_) => Net::Replace(item),
                },
            };
            nets.insert(id, net);
        }
    }
    let reordered = match (second_order, first_order) {
        (Some(order), _) => Some(order),
        (None, Some(order)) => Some(order.into_iter().filter(|id| !second_removed.contains(id)).chain(second_added.into_iter().filter(|id| nets.contains_key(id))).collect()),
        (None, None) => None,
    };
    let mut removed = Vec::new();
    let mut patched = Vec::new();
    let mut adds: std::collections::BTreeMap<String, D::Item> = std::collections::BTreeMap::new();
    for (id, net) in nets {
        match net {
            Net::Patch(patch) => patched.push((id, patch)),
            Net::Remove => removed.push(id),
            Net::Add(item) => {
                adds.insert(id, item);
            }
            Net::Replace(item) => {
                removed.push(id.clone());
                adds.insert(id, item);
            }
        }
    }
    let mut added: Vec<D::Item> = Vec::with_capacity(adds.len());
    if reordered.is_some() {
        added.extend(adds.into_values());
    } else {
        for id in &appended {
            if let Some(item) = adds.remove(id) {
                added.push(item);
            }
        }
    }
    D::from_parts(added, removed, patched, reordered)
}

fn absorb_optional<D: Delta>(first: &mut Option<D>, second: Option<D>) {
    let Some(second) = second else { return };
    let merged = absorb_delta(first.take().unwrap_or_default(), second);
    *first = (!is_empty_delta(&merged)).then_some(merged);
}

fn forward_order<D: Delta>(base_ids: &[String], delta: &D) -> Vec<String> {
    let mut ids: Vec<String> = base_ids.iter().filter(|id| !delta.removed().contains(id)).cloned().collect();
    ids.extend(delta.added().iter().map(|item| item.id().to_string()));
    match delta.reordered() {
        Some(order) => order.to_vec(),
        None => ids,
    }
}

/// 🔁️ The negative delta against `base`: patches restore base rows, adds become removes, removes re-add base rows.
fn inverse_delta<D: Delta>(delta: &D, base: &[D::Item]) -> D {
    let find = |id: &str| base.iter().find(|item| item.id() == id);
    let removed: Vec<String> = delta.added().iter().map(|item| item.id().to_string()).collect();
    let added: Vec<D::Item> = delta.removed().iter().filter_map(|id| find(id).cloned()).collect();
    let patched: Vec<(String, D::Patch)> = delta.patched().into_iter().filter_map(|(id, patch)| find(id).map(|item| (id.to_string(), patch.inverse_against(item)))).collect();
    let base_ids: Vec<String> = base.iter().map(|item| item.id().to_string()).collect();
    let mut simulated: Vec<String> = forward_order(&base_ids, delta).into_iter().filter(|id| !removed.contains(id)).collect();
    simulated.extend(added.iter().map(|item| item.id().to_string()));
    let reordered = (simulated != base_ids).then_some(base_ids);
    D::from_parts(added, removed, patched, reordered)
}

fn inverse_optional<D: Delta>(delta: &Option<D>, base: &[D::Item]) -> Option<D> {
    delta.as_ref().map(|delta| inverse_delta(delta, base))
}

fn between_delta<D: Delta>(base: &[D::Item], other: &[D::Item]) -> Option<D> {
    let removed: Vec<String> = base.iter().filter(|item| !other.iter().any(|candidate| candidate.id() == item.id())).map(|item| item.id().to_string()).collect();
    let added: Vec<D::Item> = other.iter().filter(|item| !base.iter().any(|candidate| candidate.id() == item.id())).cloned().collect();
    let patched: Vec<(String, D::Patch)> = base
        .iter()
        .filter_map(|item| other.iter().find(|candidate| candidate.id() == item.id()).and_then(|candidate| D::Patch::between(item, candidate)).map(|patch| (item.id().to_string(), patch)))
        .collect();
    let mut natural: Vec<String> = base.iter().filter(|item| !removed.iter().any(|id| id == item.id())).map(|item| item.id().to_string()).collect();
    natural.extend(added.iter().map(|item| item.id().to_string()));
    let target: Vec<String> = other.iter().map(|item| item.id().to_string()).collect();
    let reordered = (natural != target).then_some(target);
    (!(removed.is_empty() && added.is_empty() && patched.is_empty() && reordered.is_none())).then(|| D::from_parts(added, removed, patched, reordered))
}
//#endregion 🔖️DeltaAlgebra

//#region 🔖️Apply
impl MutationDiff<Fem3dSnapshot> for Fem3dDiff {
    fn apply(&self, snapshot: &Fem3dSnapshot, _capability: protocol::ApplyCapability) -> MutationApplyResult<Fem3dSnapshot> {
        let mut next = snapshot.clone();
        if let Some(delta) = &self.nodes {
            next.nodes = apply_delta(&next.nodes, delta).map_err(|error| error.under(["nodes"]))?;
        }
        if let Some(delta) = &self.elements {
            next.elements = apply_delta(&next.elements, delta).map_err(|error| error.under(["elements"]))?;
        }
        if let Some(delta) = &self.solids {
            next.solids = apply_delta(&next.solids, delta).map_err(|error| error.under(["solids"]))?;
        }
        if let Some(delta) = &self.materials {
            next.materials = apply_delta(&next.materials, delta).map_err(|error| error.under(["materials"]))?;
        }
        if let Some(delta) = &self.sections {
            next.sections = apply_delta(&next.sections, delta).map_err(|error| error.under(["sections"]))?;
        }
        if let Some(delta) = &self.supports {
            next.supports = apply_delta(&next.supports, delta).map_err(|error| error.under(["supports"]))?;
        }
        if let Some(delta) = &self.load_cases {
            next.load_cases = apply_delta(&next.load_cases, delta).map_err(|error| error.under(["loadCases"]))?;
        }
        if let Some(delta) = &self.combinations {
            next.combinations = apply_delta(&next.combinations, delta).map_err(|error| error.under(["combinations"]))?;
        }
        if let Some(patch) = &self.analysis {
            next.analysis = patch.applied(&next.analysis);
        }
        Ok(next)
    }
    fn absorb(&mut self, other: Self) {
        absorb_optional(&mut self.nodes, other.nodes);
        absorb_optional(&mut self.elements, other.elements);
        absorb_optional(&mut self.solids, other.solids);
        absorb_optional(&mut self.materials, other.materials);
        absorb_optional(&mut self.sections, other.sections);
        absorb_optional(&mut self.supports, other.supports);
        absorb_optional(&mut self.load_cases, other.load_cases);
        absorb_optional(&mut self.combinations, other.combinations);
        if let Some(later) = other.analysis {
            match &mut self.analysis {
                Some(earlier) => earlier.composed(later),
                None => self.analysis = Some(later),
            }
        }
    }
}

impl DiffAlgebra<Fem3dSnapshot> for Fem3dDiff {
    fn inverse(&self, base: &Fem3dSnapshot) -> Self {
        Self {
            nodes: inverse_optional(&self.nodes, &base.nodes),
            elements: inverse_optional(&self.elements, &base.elements),
            solids: inverse_optional(&self.solids, &base.solids),
            materials: inverse_optional(&self.materials, &base.materials),
            sections: inverse_optional(&self.sections, &base.sections),
            supports: inverse_optional(&self.supports, &base.supports),
            load_cases: inverse_optional(&self.load_cases, &base.load_cases),
            combinations: inverse_optional(&self.combinations, &base.combinations),
            analysis: self.analysis.as_ref().map(|patch| patch.inverse_against(&base.analysis)),
        }
    }
    fn between(base: &Fem3dSnapshot, other: &Fem3dSnapshot) -> Self {
        Self {
            nodes: between_delta(&base.nodes, &other.nodes),
            elements: between_delta(&base.elements, &other.elements),
            solids: between_delta(&base.solids, &other.solids),
            materials: between_delta(&base.materials, &other.materials),
            sections: between_delta(&base.sections, &other.sections),
            supports: between_delta(&base.supports, &other.supports),
            load_cases: between_delta(&base.load_cases, &other.load_cases),
            combinations: between_delta(&base.combinations, &other.combinations),
            analysis: Fem3dAnalysisPatch::between(&base.analysis, &other.analysis),
        }
    }
    fn is_empty(&self) -> bool {
        self.nodes.as_ref().is_none_or(is_empty_delta)
            && self.elements.as_ref().is_none_or(is_empty_delta)
            && self.solids.as_ref().is_none_or(is_empty_delta)
            && self.materials.as_ref().is_none_or(is_empty_delta)
            && self.sections.as_ref().is_none_or(is_empty_delta)
            && self.supports.as_ref().is_none_or(is_empty_delta)
            && self.load_cases.as_ref().is_none_or(is_empty_delta)
            && self.combinations.as_ref().is_none_or(is_empty_delta)
            && self.analysis.as_ref().is_none_or(|patch| patch == &Fem3dAnalysisPatch::default())
    }
}
//#endregion 🔖️Apply

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
