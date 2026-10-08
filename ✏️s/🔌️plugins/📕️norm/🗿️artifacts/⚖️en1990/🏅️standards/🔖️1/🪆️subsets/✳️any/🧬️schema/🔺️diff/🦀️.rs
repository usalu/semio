//! 🧬️ En1990 diff schema — sparse scalar fields plus keyed row deltas for every list the document owns.

use framework_schema::ArtifactSchema;
use protocol::{DiffAlgebra, MutationDiff};
use semio_s_artifact_norm_contract::{norm_list_delta, norm_row_patch};

use crate::En1990Snapshot;
use crate::document::AnnexChoice;
use crate::MemberEffect;

//#region 🔖️Rows
norm_row_patch! {
    /// 🩹 Sparse field patch of one `PermanentAction`.
    pub En1990PermanentPatch of crate::PermanentAction { set { kind: String, gk: f64 } }
}

norm_list_delta! {
    /// 📋️ Keyed row delta of one `PermanentAction` list.
    pub En1990PermanentDelta { addition: En1990PermanentAddition, modification: En1990PermanentModification, row: crate::PermanentAction, patch: En1990PermanentPatch, key: id }
}

norm_row_patch! {
    /// 🩹 Sparse field patch of one `VariableAction`.
    pub En1990VariablePatch of crate::VariableAction { set { category: String, qk: f64 } }
}

norm_list_delta! {
    /// 📋️ Keyed row delta of one `VariableAction` list.
    pub En1990VariableDelta { addition: En1990VariableAddition, modification: En1990VariableModification, row: crate::VariableAction, patch: En1990VariablePatch, key: id }
}

norm_row_patch! {
    /// 🩹 Sparse field patch of one `AccidentalAction`.
    pub En1990AccidentalPatch of crate::AccidentalAction { set { ad: f64 } }
}

norm_list_delta! {
    /// 📋️ Keyed row delta of one `AccidentalAction` list.
    pub En1990AccidentalDelta { addition: En1990AccidentalAddition, modification: En1990AccidentalModification, row: crate::AccidentalAction, patch: En1990AccidentalPatch, key: id }
}

norm_row_patch! {
    /// 🩹 Sparse field patch of one `SeismicAction`.
    pub En1990SeismicPatch of crate::SeismicAction { set { a_ek: f64, importance_class: crate::ImportanceClass } }
}

norm_list_delta! {
    /// 📋️ Keyed row delta of one `SeismicAction` list.
    pub En1990SeismicDelta { addition: En1990SeismicAddition, modification: En1990SeismicModification, row: crate::SeismicAction, patch: En1990SeismicPatch, key: id }
}

norm_row_patch! {
    /// 🩹 Sparse field patch of one `Member`.
    pub En1990MemberPatch of crate::Member { set { label_en: String, label_de: String, rd_str: f64, rd_geo: f64, rd_equ_stab: f64, rd_equ_destab: f64, rd_fat: f64, span: f64, deflection_w: f64, deflection_limit_ratio: f64, vibration_frequency: f64, vibration_frequency_min: f64 } }
}

norm_list_delta! {
    /// 📋️ Keyed row delta of one `Member` list.
    pub En1990MemberDelta { addition: En1990MemberAddition, modification: En1990MemberModification, row: crate::Member, patch: En1990MemberPatch, key: id }
}

norm_row_patch! {
    /// 🩹 Sparse field patch of one `BridgeSls`.
    pub En1990BridgeSlsPatch of crate::BridgeSls { set { member_id: String, deck_acceleration: f64, deck_acceleration_limit: f64, deck_twist: f64, deck_twist_limit: f64, bridge_deflection: f64, bridge_deflection_limit: f64 } }
}

norm_list_delta! {
    /// 📋️ Keyed row delta of one `BridgeSls` list.
    pub En1990BridgeSlsDelta { addition: En1990BridgeSlsAddition, modification: En1990BridgeSlsModification, row: crate::BridgeSls, patch: En1990BridgeSlsPatch, key: id }
}
//#endregion 🔖️Rows

//#region 🔖️Diff
/// 🔺️ Sparse delta for the En1990 artifact: the scalar fields a mutation sets and the keyed row deltas of its lists.
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.norm.en1990")]
pub struct En1990Diff {
    #[state(artifact)]
    pub annex: Option<AnnexChoice>,
    #[state(artifact)]
    pub project_id: Option<String>,
    #[state(artifact)]
    pub structure_kind: Option<String>,
    #[state(artifact)]
    pub altitude_m: Option<f64>,
    #[state(artifact)]
    pub consequence_class: Option<u8>,
    #[state(artifact)]
    pub reliability_class: Option<u8>,
    #[state(artifact)]
    pub design_working_life_category: Option<u8>,
    #[state(artifact)]
    pub design_working_life_years: Option<f64>,
    #[state(artifact)]
    pub reference_period_years: Option<f64>,
    #[state(artifact)]
    pub supervision_level: Option<String>,
    #[state(artifact)]
    pub inspection_level: Option<String>,
    #[state(artifact)]
    pub k_fi_declared: Option<f64>,
    #[state(artifact)]
    pub beta_computed: Option<f64>,
    #[state(artifact)]
    pub permanents: En1990PermanentDelta,
    #[state(artifact)]
    pub variables: En1990VariableDelta,
    #[state(artifact)]
    pub accidentals: En1990AccidentalDelta,
    #[state(artifact)]
    pub seismics: En1990SeismicDelta,
    #[state(artifact)]
    pub members: En1990MemberDelta,
    #[state(artifact)]
    pub bridge_sls: En1990BridgeSlsDelta,
    #[state(artifact)]
    pub effects: Vec<En1990EffectEdit>,
}
//#endregion 🔖️Diff

//#region 🔖️RowEdits
/// 🧾️ The operation one ordered-row edit performs on its collection.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub enum En1990RowOp {
    #[dsl(key = "insert")]
    Insert,
    #[dsl(key = "remove")]
    Remove,
    #[dsl(key = "replace")]
    Replace,
}

/// 🔭️ What one row edit says, read without caring which collection it edits.
pub enum En1990RowView<'a, T> {
    Insert(usize, &'a T),
    Remove(usize, &'a str),
    Replace(usize, &'a str, &'a T),
    Malformed(usize),
}

/// 🧷️ One ordered-row edit of a collection: how it reads, and how the engine builds edits of its own kind.
pub trait En1990RowEdit: Clone + PartialEq + Sized {
    type Row: Clone + PartialEq;
    fn key(row: &Self::Row) -> String;
    fn view(&self) -> En1990RowView<'_, Self::Row>;
    fn insert(index: usize, row: Self::Row) -> Self;
    fn remove(index: usize, id: String) -> Self;
    fn replace(index: usize, id: String, row: Self::Row) -> Self;
}

fn slot(index: usize) -> u32 {
    u32::try_from(index).unwrap_or(u32::MAX)
}

fn row_fault(code: &str, index: usize, id: &str) -> protocol::MutationApplyError {
    protocol::MutationApplyError::new(format!("mutation.apply.{code}"), format!("row {index} ('{id}') cannot take the edit.")).at([index.to_string()])
}

fn guard<E: En1990RowEdit>(rows: &[E::Row], index: usize, id: &str) -> Result<(), protocol::MutationApplyError> {
    match rows.get(index) {
        Some(row) if E::key(row) == id => Ok(()),
        Some(_) => Err(row_fault("row-key-mismatch", index, id)),
        None => Err(row_fault("row-out-of-range", index, id)),
    }
}

fn apply_rows<E: En1990RowEdit>(rows: &mut Vec<E::Row>, edits: &[E]) -> Result<(), protocol::MutationApplyError> {
    for edit in edits {
        match edit.view() {
            En1990RowView::Insert(index, row) if index <= rows.len() => rows.insert(index, row.clone()),
            En1990RowView::Insert(index, row) => return Err(row_fault("row-out-of-range", index, &E::key(row))),
            En1990RowView::Remove(index, id) => {
                guard::<E>(rows, index, id)?;
                rows.remove(index);
            }
            En1990RowView::Replace(index, id, row) => {
                guard::<E>(rows, index, id)?;
                rows[index] = row.clone();
            }
            En1990RowView::Malformed(index) => return Err(row_fault("row-malformed", index, "")),
        }
    }
    Ok(())
}

fn invert_rows<E: En1990RowEdit>(base: &[E::Row], edits: &[E]) -> Vec<E> {
    let mut rows = base.to_vec();
    let mut inverse = Vec::with_capacity(edits.len());
    for edit in edits {
        let step = match edit.view() {
            En1990RowView::Insert(index, row) => E::remove(index, E::key(row)),
            En1990RowView::Remove(index, _) => match rows.get(index) {
                Some(old) => E::insert(index, old.clone()),
                None => break,
            },
            En1990RowView::Replace(index, _, row) => match rows.get(index) {
                Some(old) => E::replace(index, E::key(row), old.clone()),
                None => break,
            },
            En1990RowView::Malformed(_) => break,
        };
        if apply_rows::<E>(&mut rows, std::slice::from_ref(edit)).is_err() {
            break;
        }
        inverse.push(step);
    }
    inverse.reverse();
    inverse
}

enum RowMerge<E> {
    Cancel,
    Into(E),
    Keep,
}

fn merge_rows<E: En1990RowEdit>(first: &E, next: &E) -> RowMerge<E> {
    match (first.view(), next.view()) {
        (En1990RowView::Insert(at, row), En1990RowView::Remove(to, id)) if at == to && E::key(row) == id => RowMerge::Cancel,
        (En1990RowView::Insert(at, row), En1990RowView::Replace(to, id, with)) if at == to && E::key(row) == id => RowMerge::Into(E::insert(at, with.clone())),
        (En1990RowView::Replace(at, was, row), En1990RowView::Replace(to, id, with)) if at == to && E::key(row) == id => RowMerge::Into(E::replace(at, was.to_string(), with.clone())),
        (En1990RowView::Replace(at, was, row), En1990RowView::Remove(to, id)) if at == to && E::key(row) == id => RowMerge::Into(E::remove(at, was.to_string())),
        (En1990RowView::Remove(at, was), En1990RowView::Insert(to, row)) if at == to && E::key(row) == was => RowMerge::Into(E::replace(at, was.to_string(), row.clone())),
        _ => RowMerge::Keep,
    }
}

fn push_row<E: En1990RowEdit>(edits: &mut Vec<E>, next: E) {
    let mut next = next;
    while let Some(last) = edits.pop() {
        match merge_rows(&last, &next) {
            RowMerge::Cancel => return,
            RowMerge::Into(merged) => next = merged,
            RowMerge::Keep => {
                edits.push(last);
                break;
            }
        }
    }
    edits.push(next);
}

fn positional_rows<E: En1990RowEdit>(base: &[E::Row], other: &[E::Row]) -> Vec<E> {
    let shared = base.iter().zip(other).take_while(|(left, right)| left == right).count();
    let removed = (shared..base.len()).rev().map(|index| E::remove(index, E::key(&base[index])));
    let inserted = (shared..other.len()).map(|index| E::insert(index, other[index].clone()));
    removed.chain(inserted).collect()
}

fn rows_between<E: En1990RowEdit>(base: &[E::Row], other: &[E::Row]) -> Vec<E> {
    let mut rows = base.to_vec();
    let mut edits = Vec::new();
    for index in (0..rows.len()).rev() {
        if !other.iter().any(|row| E::key(row) == E::key(&rows[index])) {
            edits.push(E::remove(index, E::key(&rows[index])));
            rows.remove(index);
        }
    }
    for (index, target) in other.iter().enumerate() {
        match rows.get(index) {
            Some(row) if E::key(row) == E::key(target) => {
                if row != target {
                    edits.push(E::replace(index, E::key(target), target.clone()));
                    rows[index] = target.clone();
                }
            }
            _ => {
                if let Some(at) = rows.iter().position(|row| E::key(row) == E::key(target)) {
                    edits.push(E::remove(at, E::key(target)));
                    rows.remove(at);
                }
                edits.push(E::insert(index, target.clone()));
                rows.insert(index.min(rows.len()), target.clone());
            }
        }
    }
    let mut check = base.to_vec();
    if apply_rows::<E>(&mut check, &edits).is_ok() && check == other {
        return edits;
    }
    positional_rows::<E>(base, other)
}
//#endregion 🔖️RowEdits

/// 🧾️ One ordered-row edit of `effects`.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct En1990EffectEdit {
    pub op: En1990RowOp,
    pub index: u32,
    pub id: String,
    pub value: Option<MemberEffect>,
}

impl En1990RowEdit for En1990EffectEdit {
    type Row = MemberEffect;
    fn key(row: &MemberEffect) -> String {
        String::new()
    }

    fn view(&self) -> En1990RowView<'_, MemberEffect> {
        let (index, id) = (self.index as usize, self.id.as_str());
        match (self.op, &self.value) {
            (En1990RowOp::Insert, Some(row)) => En1990RowView::Insert(index, row),
            (En1990RowOp::Remove, None) => En1990RowView::Remove(index, id),
            (En1990RowOp::Replace, Some(row)) => En1990RowView::Replace(index, id, row),
            _ => En1990RowView::Malformed(index),
        }
    }

    fn insert(index: usize, row: MemberEffect) -> Self {
        Self { op: En1990RowOp::Insert, index: slot(index), id: Self::key(&row), value: Some(row) }
    }

    fn remove(index: usize, id: String) -> Self {
        Self { op: En1990RowOp::Remove, index: slot(index), id, value: None }
    }

    fn replace(index: usize, id: String, row: MemberEffect) -> Self {
        Self { op: En1990RowOp::Replace, index: slot(index), id, value: Some(row) }
    }
}

impl MutationDiff<En1990Snapshot> for En1990Diff {
    fn apply(&self, base: &En1990Snapshot, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<En1990Snapshot> {
        let mut next = base.clone();
        if let Some(value) = self.annex {
            next.annex = value;
        }
        if let Some(value) = &self.project_id {
            next.project_id = value.clone();
        }
        if let Some(value) = &self.structure_kind {
            next.structure_kind = value.clone();
        }
        if let Some(value) = self.altitude_m {
            next.altitude_m = value;
        }
        if let Some(value) = self.consequence_class {
            next.consequence_class = value;
        }
        if let Some(value) = self.reliability_class {
            next.reliability_class = value;
        }
        if let Some(value) = self.design_working_life_category {
            next.design_working_life_category = value;
        }
        if let Some(value) = self.design_working_life_years {
            next.design_working_life_years = value;
        }
        if let Some(value) = self.reference_period_years {
            next.reference_period_years = value;
        }
        if let Some(value) = &self.supervision_level {
            next.supervision_level = value.clone();
        }
        if let Some(value) = &self.inspection_level {
            next.inspection_level = value.clone();
        }
        if let Some(value) = self.k_fi_declared {
            next.k_fi_declared = value;
        }
        if let Some(value) = self.beta_computed {
            next.beta_computed = value;
        }
        next.permanents = self.permanents.commit_onto(&base.permanents).map_err(|error| error.under(["permanents"]))?;
        next.variables = self.variables.commit_onto(&base.variables).map_err(|error| error.under(["variables"]))?;
        next.accidentals = self.accidentals.commit_onto(&base.accidentals).map_err(|error| error.under(["accidentals"]))?;
        next.seismics = self.seismics.commit_onto(&base.seismics).map_err(|error| error.under(["seismics"]))?;
        next.members = self.members.commit_onto(&base.members).map_err(|error| error.under(["members"]))?;
        next.bridge_sls = self.bridge_sls.commit_onto(&base.bridge_sls).map_err(|error| error.under(["bridgeSls"]))?;
        apply_rows::<En1990EffectEdit>(&mut next.effects, &self.effects).map_err(|error| error.under(["effects"]))?;
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        if other.annex.is_some() {
            self.annex = other.annex;
        }
        if other.project_id.is_some() {
            self.project_id = other.project_id;
        }
        if other.structure_kind.is_some() {
            self.structure_kind = other.structure_kind;
        }
        if other.altitude_m.is_some() {
            self.altitude_m = other.altitude_m;
        }
        if other.consequence_class.is_some() {
            self.consequence_class = other.consequence_class;
        }
        if other.reliability_class.is_some() {
            self.reliability_class = other.reliability_class;
        }
        if other.design_working_life_category.is_some() {
            self.design_working_life_category = other.design_working_life_category;
        }
        if other.design_working_life_years.is_some() {
            self.design_working_life_years = other.design_working_life_years;
        }
        if other.reference_period_years.is_some() {
            self.reference_period_years = other.reference_period_years;
        }
        if other.supervision_level.is_some() {
            self.supervision_level = other.supervision_level;
        }
        if other.inspection_level.is_some() {
            self.inspection_level = other.inspection_level;
        }
        if other.k_fi_declared.is_some() {
            self.k_fi_declared = other.k_fi_declared;
        }
        if other.beta_computed.is_some() {
            self.beta_computed = other.beta_computed;
        }
        self.permanents.absorb(other.permanents);
        self.variables.absorb(other.variables);
        self.accidentals.absorb(other.accidentals);
        self.seismics.absorb(other.seismics);
        self.members.absorb(other.members);
        self.bridge_sls.absorb(other.bridge_sls);
        for edit in other.effects {
            push_row(&mut self.effects, edit);
        }
    }
}

impl DiffAlgebra<En1990Snapshot> for En1990Diff {
    fn inverse(&self, base: &En1990Snapshot) -> Self {
        Self {
            annex: self.annex.map(|_| base.annex),
            project_id: self.project_id.as_ref().map(|_| base.project_id.clone()),
            structure_kind: self.structure_kind.as_ref().map(|_| base.structure_kind.clone()),
            altitude_m: self.altitude_m.map(|_| base.altitude_m),
            consequence_class: self.consequence_class.map(|_| base.consequence_class),
            reliability_class: self.reliability_class.map(|_| base.reliability_class),
            design_working_life_category: self.design_working_life_category.map(|_| base.design_working_life_category),
            design_working_life_years: self.design_working_life_years.map(|_| base.design_working_life_years),
            reference_period_years: self.reference_period_years.map(|_| base.reference_period_years),
            supervision_level: self.supervision_level.as_ref().map(|_| base.supervision_level.clone()),
            inspection_level: self.inspection_level.as_ref().map(|_| base.inspection_level.clone()),
            k_fi_declared: self.k_fi_declared.map(|_| base.k_fi_declared),
            beta_computed: self.beta_computed.map(|_| base.beta_computed),
            permanents: self.permanents.inverse(&base.permanents),
            variables: self.variables.inverse(&base.variables),
            accidentals: self.accidentals.inverse(&base.accidentals),
            seismics: self.seismics.inverse(&base.seismics),
            members: self.members.inverse(&base.members),
            bridge_sls: self.bridge_sls.inverse(&base.bridge_sls),
            effects: invert_rows::<En1990EffectEdit>(&base.effects, &self.effects),
        }
    }

    fn between(base: &En1990Snapshot, other: &En1990Snapshot) -> Self {
        Self {
            annex: (base.annex != other.annex).then_some(other.annex),
            project_id: (base.project_id != other.project_id).then(|| other.project_id.clone()),
            structure_kind: (base.structure_kind != other.structure_kind).then(|| other.structure_kind.clone()),
            altitude_m: (base.altitude_m != other.altitude_m).then_some(other.altitude_m),
            consequence_class: (base.consequence_class != other.consequence_class).then_some(other.consequence_class),
            reliability_class: (base.reliability_class != other.reliability_class).then_some(other.reliability_class),
            design_working_life_category: (base.design_working_life_category != other.design_working_life_category).then_some(other.design_working_life_category),
            design_working_life_years: (base.design_working_life_years != other.design_working_life_years).then_some(other.design_working_life_years),
            reference_period_years: (base.reference_period_years != other.reference_period_years).then_some(other.reference_period_years),
            supervision_level: (base.supervision_level != other.supervision_level).then(|| other.supervision_level.clone()),
            inspection_level: (base.inspection_level != other.inspection_level).then(|| other.inspection_level.clone()),
            k_fi_declared: (base.k_fi_declared != other.k_fi_declared).then_some(other.k_fi_declared),
            beta_computed: (base.beta_computed != other.beta_computed).then_some(other.beta_computed),
            permanents: En1990PermanentDelta::between(&base.permanents, &other.permanents),
            variables: En1990VariableDelta::between(&base.variables, &other.variables),
            accidentals: En1990AccidentalDelta::between(&base.accidentals, &other.accidentals),
            seismics: En1990SeismicDelta::between(&base.seismics, &other.seismics),
            members: En1990MemberDelta::between(&base.members, &other.members),
            bridge_sls: En1990BridgeSlsDelta::between(&base.bridge_sls, &other.bridge_sls),
            effects: rows_between::<En1990EffectEdit>(&base.effects, &other.effects),
        }
    }

    fn is_empty(&self) -> bool {
        self.annex.is_none()
            && self.project_id.is_none()
            && self.structure_kind.is_none()
            && self.altitude_m.is_none()
            && self.consequence_class.is_none()
            && self.reliability_class.is_none()
            && self.design_working_life_category.is_none()
            && self.design_working_life_years.is_none()
            && self.reference_period_years.is_none()
            && self.supervision_level.is_none()
            && self.inspection_level.is_none()
            && self.k_fi_declared.is_none()
            && self.beta_computed.is_none()
            && self.permanents.is_empty()
            && self.variables.is_empty()
            && self.accidentals.is_empty()
            && self.seismics.is_empty()
            && self.members.is_empty()
            && self.bridge_sls.is_empty()
            && self.effects.is_empty()
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
