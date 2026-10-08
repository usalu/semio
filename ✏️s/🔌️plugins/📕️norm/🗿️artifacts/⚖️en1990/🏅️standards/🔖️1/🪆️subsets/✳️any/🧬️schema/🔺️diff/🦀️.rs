//! 🧬️ En1990 diff schema — sparse scalar fields and ordered per-collection row edits.

use crate::document::AnnexChoice;
use crate::{En1990Snapshot, AccidentalAction, BridgeSls, Member, MemberEffect, PermanentAction, SeismicAction, VariableAction};
use framework_schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Sparse delta for the En1990 artifact: the changed scalars and ordered row edits per collection.
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
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
    pub permanents: Vec<En1990PermanentEdit>,
    #[state(artifact)]
    pub variables: Vec<En1990VariableEdit>,
    #[state(artifact)]
    pub accidentals: Vec<En1990AccidentalEdit>,
    #[state(artifact)]
    pub seismics: Vec<En1990SeismicEdit>,
    #[state(artifact)]
    pub members: Vec<En1990MemberEdit>,
    #[state(artifact)]
    pub bridge_sls: Vec<En1990BridgeSlsEdit>,
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

/// 🧾️ One ordered-row edit of `permanents`.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct En1990PermanentEdit {
    pub op: En1990RowOp,
    pub index: u32,
    pub id: String,
    pub value: Option<PermanentAction>,
}

impl En1990RowEdit for En1990PermanentEdit {
    type Row = PermanentAction;
    fn key(row: &PermanentAction) -> String {
        row.id.clone()
    }

    fn view(&self) -> En1990RowView<'_, PermanentAction> {
        let (index, id) = (self.index as usize, self.id.as_str());
        match (self.op, &self.value) {
            (En1990RowOp::Insert, Some(row)) => En1990RowView::Insert(index, row),
            (En1990RowOp::Remove, None) => En1990RowView::Remove(index, id),
            (En1990RowOp::Replace, Some(row)) => En1990RowView::Replace(index, id, row),
            _ => En1990RowView::Malformed(index),
        }
    }

    fn insert(index: usize, row: PermanentAction) -> Self {
        Self { op: En1990RowOp::Insert, index: slot(index), id: Self::key(&row), value: Some(row) }
    }

    fn remove(index: usize, id: String) -> Self {
        Self { op: En1990RowOp::Remove, index: slot(index), id, value: None }
    }

    fn replace(index: usize, id: String, row: PermanentAction) -> Self {
        Self { op: En1990RowOp::Replace, index: slot(index), id, value: Some(row) }
    }
}

/// 🧾️ One ordered-row edit of `variables`.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct En1990VariableEdit {
    pub op: En1990RowOp,
    pub index: u32,
    pub id: String,
    pub value: Option<VariableAction>,
}

impl En1990RowEdit for En1990VariableEdit {
    type Row = VariableAction;
    fn key(row: &VariableAction) -> String {
        row.id.clone()
    }

    fn view(&self) -> En1990RowView<'_, VariableAction> {
        let (index, id) = (self.index as usize, self.id.as_str());
        match (self.op, &self.value) {
            (En1990RowOp::Insert, Some(row)) => En1990RowView::Insert(index, row),
            (En1990RowOp::Remove, None) => En1990RowView::Remove(index, id),
            (En1990RowOp::Replace, Some(row)) => En1990RowView::Replace(index, id, row),
            _ => En1990RowView::Malformed(index),
        }
    }

    fn insert(index: usize, row: VariableAction) -> Self {
        Self { op: En1990RowOp::Insert, index: slot(index), id: Self::key(&row), value: Some(row) }
    }

    fn remove(index: usize, id: String) -> Self {
        Self { op: En1990RowOp::Remove, index: slot(index), id, value: None }
    }

    fn replace(index: usize, id: String, row: VariableAction) -> Self {
        Self { op: En1990RowOp::Replace, index: slot(index), id, value: Some(row) }
    }
}

/// 🧾️ One ordered-row edit of `accidentals`.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct En1990AccidentalEdit {
    pub op: En1990RowOp,
    pub index: u32,
    pub id: String,
    pub value: Option<AccidentalAction>,
}

impl En1990RowEdit for En1990AccidentalEdit {
    type Row = AccidentalAction;
    fn key(row: &AccidentalAction) -> String {
        row.id.clone()
    }

    fn view(&self) -> En1990RowView<'_, AccidentalAction> {
        let (index, id) = (self.index as usize, self.id.as_str());
        match (self.op, &self.value) {
            (En1990RowOp::Insert, Some(row)) => En1990RowView::Insert(index, row),
            (En1990RowOp::Remove, None) => En1990RowView::Remove(index, id),
            (En1990RowOp::Replace, Some(row)) => En1990RowView::Replace(index, id, row),
            _ => En1990RowView::Malformed(index),
        }
    }

    fn insert(index: usize, row: AccidentalAction) -> Self {
        Self { op: En1990RowOp::Insert, index: slot(index), id: Self::key(&row), value: Some(row) }
    }

    fn remove(index: usize, id: String) -> Self {
        Self { op: En1990RowOp::Remove, index: slot(index), id, value: None }
    }

    fn replace(index: usize, id: String, row: AccidentalAction) -> Self {
        Self { op: En1990RowOp::Replace, index: slot(index), id, value: Some(row) }
    }
}

/// 🧾️ One ordered-row edit of `seismics`.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct En1990SeismicEdit {
    pub op: En1990RowOp,
    pub index: u32,
    pub id: String,
    pub value: Option<SeismicAction>,
}

impl En1990RowEdit for En1990SeismicEdit {
    type Row = SeismicAction;
    fn key(row: &SeismicAction) -> String {
        row.id.clone()
    }

    fn view(&self) -> En1990RowView<'_, SeismicAction> {
        let (index, id) = (self.index as usize, self.id.as_str());
        match (self.op, &self.value) {
            (En1990RowOp::Insert, Some(row)) => En1990RowView::Insert(index, row),
            (En1990RowOp::Remove, None) => En1990RowView::Remove(index, id),
            (En1990RowOp::Replace, Some(row)) => En1990RowView::Replace(index, id, row),
            _ => En1990RowView::Malformed(index),
        }
    }

    fn insert(index: usize, row: SeismicAction) -> Self {
        Self { op: En1990RowOp::Insert, index: slot(index), id: Self::key(&row), value: Some(row) }
    }

    fn remove(index: usize, id: String) -> Self {
        Self { op: En1990RowOp::Remove, index: slot(index), id, value: None }
    }

    fn replace(index: usize, id: String, row: SeismicAction) -> Self {
        Self { op: En1990RowOp::Replace, index: slot(index), id, value: Some(row) }
    }
}

/// 🧾️ One ordered-row edit of `members`.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct En1990MemberEdit {
    pub op: En1990RowOp,
    pub index: u32,
    pub id: String,
    pub value: Option<Member>,
}

impl En1990RowEdit for En1990MemberEdit {
    type Row = Member;
    fn key(row: &Member) -> String {
        row.id.clone()
    }

    fn view(&self) -> En1990RowView<'_, Member> {
        let (index, id) = (self.index as usize, self.id.as_str());
        match (self.op, &self.value) {
            (En1990RowOp::Insert, Some(row)) => En1990RowView::Insert(index, row),
            (En1990RowOp::Remove, None) => En1990RowView::Remove(index, id),
            (En1990RowOp::Replace, Some(row)) => En1990RowView::Replace(index, id, row),
            _ => En1990RowView::Malformed(index),
        }
    }

    fn insert(index: usize, row: Member) -> Self {
        Self { op: En1990RowOp::Insert, index: slot(index), id: Self::key(&row), value: Some(row) }
    }

    fn remove(index: usize, id: String) -> Self {
        Self { op: En1990RowOp::Remove, index: slot(index), id, value: None }
    }

    fn replace(index: usize, id: String, row: Member) -> Self {
        Self { op: En1990RowOp::Replace, index: slot(index), id, value: Some(row) }
    }
}

/// 🧾️ One ordered-row edit of `bridge_sls`.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct En1990BridgeSlsEdit {
    pub op: En1990RowOp,
    pub index: u32,
    pub id: String,
    pub value: Option<BridgeSls>,
}

impl En1990RowEdit for En1990BridgeSlsEdit {
    type Row = BridgeSls;
    fn key(row: &BridgeSls) -> String {
        row.id.clone()
    }

    fn view(&self) -> En1990RowView<'_, BridgeSls> {
        let (index, id) = (self.index as usize, self.id.as_str());
        match (self.op, &self.value) {
            (En1990RowOp::Insert, Some(row)) => En1990RowView::Insert(index, row),
            (En1990RowOp::Remove, None) => En1990RowView::Remove(index, id),
            (En1990RowOp::Replace, Some(row)) => En1990RowView::Replace(index, id, row),
            _ => En1990RowView::Malformed(index),
        }
    }

    fn insert(index: usize, row: BridgeSls) -> Self {
        Self { op: En1990RowOp::Insert, index: slot(index), id: Self::key(&row), value: Some(row) }
    }

    fn remove(index: usize, id: String) -> Self {
        Self { op: En1990RowOp::Remove, index: slot(index), id, value: None }
    }

    fn replace(index: usize, id: String, row: BridgeSls) -> Self {
        Self { op: En1990RowOp::Replace, index: slot(index), id, value: Some(row) }
    }
}

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

impl protocol::MutationDiff<En1990Snapshot> for En1990Diff {
    fn apply(&self, base: &En1990Snapshot, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<En1990Snapshot> {
        let mut next = base.clone();
        if let Some(value) = &self.annex {
            next.annex = value.clone();
        }
        if let Some(value) = &self.project_id {
            next.project_id = value.clone();
        }
        if let Some(value) = &self.structure_kind {
            next.structure_kind = value.clone();
        }
        if let Some(value) = &self.altitude_m {
            next.altitude_m = value.clone();
        }
        if let Some(value) = &self.consequence_class {
            next.consequence_class = value.clone();
        }
        if let Some(value) = &self.reliability_class {
            next.reliability_class = value.clone();
        }
        if let Some(value) = &self.design_working_life_category {
            next.design_working_life_category = value.clone();
        }
        if let Some(value) = &self.design_working_life_years {
            next.design_working_life_years = value.clone();
        }
        if let Some(value) = &self.reference_period_years {
            next.reference_period_years = value.clone();
        }
        if let Some(value) = &self.supervision_level {
            next.supervision_level = value.clone();
        }
        if let Some(value) = &self.inspection_level {
            next.inspection_level = value.clone();
        }
        if let Some(value) = &self.k_fi_declared {
            next.k_fi_declared = value.clone();
        }
        if let Some(value) = &self.beta_computed {
            next.beta_computed = value.clone();
        }
        apply_rows::<En1990PermanentEdit>(&mut next.permanents, &self.permanents).map_err(|error| error.under(["permanents"]))?;
        apply_rows::<En1990VariableEdit>(&mut next.variables, &self.variables).map_err(|error| error.under(["variables"]))?;
        apply_rows::<En1990AccidentalEdit>(&mut next.accidentals, &self.accidentals).map_err(|error| error.under(["accidentals"]))?;
        apply_rows::<En1990SeismicEdit>(&mut next.seismics, &self.seismics).map_err(|error| error.under(["seismics"]))?;
        apply_rows::<En1990MemberEdit>(&mut next.members, &self.members).map_err(|error| error.under(["members"]))?;
        apply_rows::<En1990BridgeSlsEdit>(&mut next.bridge_sls, &self.bridge_sls).map_err(|error| error.under(["bridgeSls"]))?;
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
        for edit in other.permanents {
            push_row(&mut self.permanents, edit);
        }
        for edit in other.variables {
            push_row(&mut self.variables, edit);
        }
        for edit in other.accidentals {
            push_row(&mut self.accidentals, edit);
        }
        for edit in other.seismics {
            push_row(&mut self.seismics, edit);
        }
        for edit in other.members {
            push_row(&mut self.members, edit);
        }
        for edit in other.bridge_sls {
            push_row(&mut self.bridge_sls, edit);
        }
        for edit in other.effects {
            push_row(&mut self.effects, edit);
        }
    }
}

impl protocol::DiffAlgebra<En1990Snapshot> for En1990Diff {
    fn inverse(&self, base: &En1990Snapshot) -> Self {
        Self {
            annex: self.annex.as_ref().map(|_| base.annex.clone()),
            project_id: self.project_id.as_ref().map(|_| base.project_id.clone()),
            structure_kind: self.structure_kind.as_ref().map(|_| base.structure_kind.clone()),
            altitude_m: self.altitude_m.as_ref().map(|_| base.altitude_m.clone()),
            consequence_class: self.consequence_class.as_ref().map(|_| base.consequence_class.clone()),
            reliability_class: self.reliability_class.as_ref().map(|_| base.reliability_class.clone()),
            design_working_life_category: self.design_working_life_category.as_ref().map(|_| base.design_working_life_category.clone()),
            design_working_life_years: self.design_working_life_years.as_ref().map(|_| base.design_working_life_years.clone()),
            reference_period_years: self.reference_period_years.as_ref().map(|_| base.reference_period_years.clone()),
            supervision_level: self.supervision_level.as_ref().map(|_| base.supervision_level.clone()),
            inspection_level: self.inspection_level.as_ref().map(|_| base.inspection_level.clone()),
            k_fi_declared: self.k_fi_declared.as_ref().map(|_| base.k_fi_declared.clone()),
            beta_computed: self.beta_computed.as_ref().map(|_| base.beta_computed.clone()),
            permanents: invert_rows::<En1990PermanentEdit>(&base.permanents, &self.permanents),
            variables: invert_rows::<En1990VariableEdit>(&base.variables, &self.variables),
            accidentals: invert_rows::<En1990AccidentalEdit>(&base.accidentals, &self.accidentals),
            seismics: invert_rows::<En1990SeismicEdit>(&base.seismics, &self.seismics),
            members: invert_rows::<En1990MemberEdit>(&base.members, &self.members),
            bridge_sls: invert_rows::<En1990BridgeSlsEdit>(&base.bridge_sls, &self.bridge_sls),
            effects: invert_rows::<En1990EffectEdit>(&base.effects, &self.effects),
        }
    }

    fn between(base: &En1990Snapshot, other: &En1990Snapshot) -> Self {
        Self {
            annex: (base.annex != other.annex).then(|| other.annex.clone()),
            project_id: (base.project_id != other.project_id).then(|| other.project_id.clone()),
            structure_kind: (base.structure_kind != other.structure_kind).then(|| other.structure_kind.clone()),
            altitude_m: (base.altitude_m != other.altitude_m).then(|| other.altitude_m.clone()),
            consequence_class: (base.consequence_class != other.consequence_class).then(|| other.consequence_class.clone()),
            reliability_class: (base.reliability_class != other.reliability_class).then(|| other.reliability_class.clone()),
            design_working_life_category: (base.design_working_life_category != other.design_working_life_category).then(|| other.design_working_life_category.clone()),
            design_working_life_years: (base.design_working_life_years != other.design_working_life_years).then(|| other.design_working_life_years.clone()),
            reference_period_years: (base.reference_period_years != other.reference_period_years).then(|| other.reference_period_years.clone()),
            supervision_level: (base.supervision_level != other.supervision_level).then(|| other.supervision_level.clone()),
            inspection_level: (base.inspection_level != other.inspection_level).then(|| other.inspection_level.clone()),
            k_fi_declared: (base.k_fi_declared != other.k_fi_declared).then(|| other.k_fi_declared.clone()),
            beta_computed: (base.beta_computed != other.beta_computed).then(|| other.beta_computed.clone()),
            permanents: rows_between::<En1990PermanentEdit>(&base.permanents, &other.permanents),
            variables: rows_between::<En1990VariableEdit>(&base.variables, &other.variables),
            accidentals: rows_between::<En1990AccidentalEdit>(&base.accidentals, &other.accidentals),
            seismics: rows_between::<En1990SeismicEdit>(&base.seismics, &other.seismics),
            members: rows_between::<En1990MemberEdit>(&base.members, &other.members),
            bridge_sls: rows_between::<En1990BridgeSlsEdit>(&base.bridge_sls, &other.bridge_sls),
            effects: rows_between::<En1990EffectEdit>(&base.effects, &other.effects),
        }
    }

    fn is_empty(&self) -> bool {
        self.annex.is_none() && self.project_id.is_none() && self.structure_kind.is_none() && self.altitude_m.is_none() && self.consequence_class.is_none() && self.reliability_class.is_none() && self.design_working_life_category.is_none() && self.design_working_life_years.is_none() && self.reference_period_years.is_none() && self.supervision_level.is_none() && self.inspection_level.is_none() && self.k_fi_declared.is_none() && self.beta_computed.is_none() && self.permanents.is_empty() && self.variables.is_empty() && self.accidentals.is_empty() && self.seismics.is_empty() && self.members.is_empty() && self.bridge_sls.is_empty() && self.effects.is_empty()
    }
}
