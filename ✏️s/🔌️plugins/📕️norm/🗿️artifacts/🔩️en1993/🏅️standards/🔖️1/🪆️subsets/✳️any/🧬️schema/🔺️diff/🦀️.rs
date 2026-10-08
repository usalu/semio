//! 🧬️ En1993 diff schema — sparse scalar fields and ordered per-collection row edits.

use crate::{En1993Snapshot, BridgeFatigue, ColdFormedMember, CraneRunway, FatigueDetail, FireExposure, LoadCase, MemberAction, PlatedPanel, SiloShell, SteelJoint, SteelMaterial, SteelMember, SteelPile, SteelSection, TensionComponent, TowerLeg};
use framework_schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Sparse delta for the En1993 artifact: the changed annex and ordered row edits per collection.
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.norm.en1993")]
pub struct En1993Diff {
    #[state(artifact)]
    pub annex: Option<crate::document::AnnexChoice>,
    #[state(artifact)]
    pub materials: Vec<En1993MaterialEdit>,
    #[state(artifact)]
    pub sections: Vec<En1993SectionEdit>,
    #[state(artifact)]
    pub members: Vec<En1993MemberEdit>,
    #[state(artifact)]
    pub load_cases: Vec<En1993LoadCaseEdit>,
    #[state(artifact)]
    pub member_actions: Vec<En1993MemberActionEdit>,
    #[state(artifact)]
    pub joints: Vec<En1993JointEdit>,
    #[state(artifact)]
    pub fatigue_details: Vec<En1993FatigueDetailEdit>,
    #[state(artifact)]
    pub fire_exposures: Vec<En1993FireExposureEdit>,
    #[state(artifact)]
    pub cold_formed_members: Vec<En1993ColdFormedMemberEdit>,
    #[state(artifact)]
    pub plated_panels: Vec<En1993PlatedPanelEdit>,
    #[state(artifact)]
    pub silo_shells: Vec<En1993SiloShellEdit>,
    #[state(artifact)]
    pub tension_components: Vec<En1993TensionComponentEdit>,
    #[state(artifact)]
    pub bridge_fatigue: Vec<En1993BridgeFatigueEdit>,
    #[state(artifact)]
    pub tower_legs: Vec<En1993TowerLegEdit>,
    #[state(artifact)]
    pub piles: Vec<En1993PileEdit>,
    #[state(artifact)]
    pub crane_runways: Vec<En1993CraneRunwayEdit>,
}
//#endregion 🔖️Diff

//#region 🔖️RowEdits
/// 🧾️ The operation one ordered-row edit performs on its collection.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub enum En1993RowOp {
    #[dsl(key = "insert")]
    Insert,
    #[dsl(key = "remove")]
    Remove,
    #[dsl(key = "replace")]
    Replace,
}

/// 🔭️ What one row edit says, read without caring which collection it edits.
pub enum En1993RowView<'a, T> {
    Insert(usize, &'a T),
    Remove(usize, &'a str),
    Replace(usize, &'a str, &'a T),
    Malformed(usize),
}

/// 🧷️ One ordered-row edit of a collection: how it reads, and how the engine builds edits of its own kind.
pub trait En1993RowEdit: Clone + PartialEq + Sized {
    type Row: Clone + PartialEq;
    fn key(row: &Self::Row) -> String;
    fn view(&self) -> En1993RowView<'_, Self::Row>;
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

fn guard<E: En1993RowEdit>(rows: &[E::Row], index: usize, id: &str) -> Result<(), protocol::MutationApplyError> {
    match rows.get(index) {
        Some(row) if E::key(row) == id => Ok(()),
        Some(_) => Err(row_fault("row-key-mismatch", index, id)),
        None => Err(row_fault("row-out-of-range", index, id)),
    }
}

fn apply_rows<E: En1993RowEdit>(rows: &mut Vec<E::Row>, edits: &[E]) -> Result<(), protocol::MutationApplyError> {
    for edit in edits {
        match edit.view() {
            En1993RowView::Insert(index, row) if index <= rows.len() => rows.insert(index, row.clone()),
            En1993RowView::Insert(index, row) => return Err(row_fault("row-out-of-range", index, &E::key(row))),
            En1993RowView::Remove(index, id) => {
                guard::<E>(rows, index, id)?;
                rows.remove(index);
            }
            En1993RowView::Replace(index, id, row) => {
                guard::<E>(rows, index, id)?;
                rows[index] = row.clone();
            }
            En1993RowView::Malformed(index) => return Err(row_fault("row-malformed", index, "")),
        }
    }
    Ok(())
}

fn invert_rows<E: En1993RowEdit>(base: &[E::Row], edits: &[E]) -> Vec<E> {
    let mut rows = base.to_vec();
    let mut inverse = Vec::with_capacity(edits.len());
    for edit in edits {
        let step = match edit.view() {
            En1993RowView::Insert(index, row) => E::remove(index, E::key(row)),
            En1993RowView::Remove(index, _) => match rows.get(index) {
                Some(old) => E::insert(index, old.clone()),
                None => break,
            },
            En1993RowView::Replace(index, _, row) => match rows.get(index) {
                Some(old) => E::replace(index, E::key(row), old.clone()),
                None => break,
            },
            En1993RowView::Malformed(_) => break,
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

fn merge_rows<E: En1993RowEdit>(first: &E, next: &E) -> RowMerge<E> {
    match (first.view(), next.view()) {
        (En1993RowView::Insert(at, row), En1993RowView::Remove(to, id)) if at == to && E::key(row) == id => RowMerge::Cancel,
        (En1993RowView::Insert(at, row), En1993RowView::Replace(to, id, with)) if at == to && E::key(row) == id => RowMerge::Into(E::insert(at, with.clone())),
        (En1993RowView::Replace(at, was, row), En1993RowView::Replace(to, id, with)) if at == to && E::key(row) == id => RowMerge::Into(E::replace(at, was.to_string(), with.clone())),
        (En1993RowView::Replace(at, was, row), En1993RowView::Remove(to, id)) if at == to && E::key(row) == id => RowMerge::Into(E::remove(at, was.to_string())),
        (En1993RowView::Remove(at, was), En1993RowView::Insert(to, row)) if at == to && E::key(row) == was => RowMerge::Into(E::replace(at, was.to_string(), row.clone())),
        _ => RowMerge::Keep,
    }
}

fn push_row<E: En1993RowEdit>(edits: &mut Vec<E>, next: E) {
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

fn positional_rows<E: En1993RowEdit>(base: &[E::Row], other: &[E::Row]) -> Vec<E> {
    let shared = base.iter().zip(other).take_while(|(left, right)| left == right).count();
    let removed = (shared..base.len()).rev().map(|index| E::remove(index, E::key(&base[index])));
    let inserted = (shared..other.len()).map(|index| E::insert(index, other[index].clone()));
    removed.chain(inserted).collect()
}

fn rows_between<E: En1993RowEdit>(base: &[E::Row], other: &[E::Row]) -> Vec<E> {
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

/// 🧾️ One ordered-row edit of `materials`.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct En1993MaterialEdit {
    pub op: En1993RowOp,
    pub index: u32,
    pub id: String,
    pub value: Option<SteelMaterial>,
}

impl En1993RowEdit for En1993MaterialEdit {
    type Row = SteelMaterial;
    fn key(row: &SteelMaterial) -> String {
        row.id.clone()
    }

    fn view(&self) -> En1993RowView<'_, SteelMaterial> {
        let (index, id) = (self.index as usize, self.id.as_str());
        match (self.op, &self.value) {
            (En1993RowOp::Insert, Some(row)) => En1993RowView::Insert(index, row),
            (En1993RowOp::Remove, None) => En1993RowView::Remove(index, id),
            (En1993RowOp::Replace, Some(row)) => En1993RowView::Replace(index, id, row),
            _ => En1993RowView::Malformed(index),
        }
    }

    fn insert(index: usize, row: SteelMaterial) -> Self {
        Self { op: En1993RowOp::Insert, index: slot(index), id: Self::key(&row), value: Some(row) }
    }

    fn remove(index: usize, id: String) -> Self {
        Self { op: En1993RowOp::Remove, index: slot(index), id, value: None }
    }

    fn replace(index: usize, id: String, row: SteelMaterial) -> Self {
        Self { op: En1993RowOp::Replace, index: slot(index), id, value: Some(row) }
    }
}

/// 🧾️ One ordered-row edit of `sections`.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct En1993SectionEdit {
    pub op: En1993RowOp,
    pub index: u32,
    pub id: String,
    pub value: Option<SteelSection>,
}

impl En1993RowEdit for En1993SectionEdit {
    type Row = SteelSection;
    fn key(row: &SteelSection) -> String {
        row.id.clone()
    }

    fn view(&self) -> En1993RowView<'_, SteelSection> {
        let (index, id) = (self.index as usize, self.id.as_str());
        match (self.op, &self.value) {
            (En1993RowOp::Insert, Some(row)) => En1993RowView::Insert(index, row),
            (En1993RowOp::Remove, None) => En1993RowView::Remove(index, id),
            (En1993RowOp::Replace, Some(row)) => En1993RowView::Replace(index, id, row),
            _ => En1993RowView::Malformed(index),
        }
    }

    fn insert(index: usize, row: SteelSection) -> Self {
        Self { op: En1993RowOp::Insert, index: slot(index), id: Self::key(&row), value: Some(row) }
    }

    fn remove(index: usize, id: String) -> Self {
        Self { op: En1993RowOp::Remove, index: slot(index), id, value: None }
    }

    fn replace(index: usize, id: String, row: SteelSection) -> Self {
        Self { op: En1993RowOp::Replace, index: slot(index), id, value: Some(row) }
    }
}

/// 🧾️ One ordered-row edit of `members`.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct En1993MemberEdit {
    pub op: En1993RowOp,
    pub index: u32,
    pub id: String,
    pub value: Option<SteelMember>,
}

impl En1993RowEdit for En1993MemberEdit {
    type Row = SteelMember;
    fn key(row: &SteelMember) -> String {
        row.id.clone()
    }

    fn view(&self) -> En1993RowView<'_, SteelMember> {
        let (index, id) = (self.index as usize, self.id.as_str());
        match (self.op, &self.value) {
            (En1993RowOp::Insert, Some(row)) => En1993RowView::Insert(index, row),
            (En1993RowOp::Remove, None) => En1993RowView::Remove(index, id),
            (En1993RowOp::Replace, Some(row)) => En1993RowView::Replace(index, id, row),
            _ => En1993RowView::Malformed(index),
        }
    }

    fn insert(index: usize, row: SteelMember) -> Self {
        Self { op: En1993RowOp::Insert, index: slot(index), id: Self::key(&row), value: Some(row) }
    }

    fn remove(index: usize, id: String) -> Self {
        Self { op: En1993RowOp::Remove, index: slot(index), id, value: None }
    }

    fn replace(index: usize, id: String, row: SteelMember) -> Self {
        Self { op: En1993RowOp::Replace, index: slot(index), id, value: Some(row) }
    }
}

/// 🧾️ One ordered-row edit of `load_cases`.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct En1993LoadCaseEdit {
    pub op: En1993RowOp,
    pub index: u32,
    pub id: String,
    pub value: Option<LoadCase>,
}

impl En1993RowEdit for En1993LoadCaseEdit {
    type Row = LoadCase;
    fn key(row: &LoadCase) -> String {
        row.id.clone()
    }

    fn view(&self) -> En1993RowView<'_, LoadCase> {
        let (index, id) = (self.index as usize, self.id.as_str());
        match (self.op, &self.value) {
            (En1993RowOp::Insert, Some(row)) => En1993RowView::Insert(index, row),
            (En1993RowOp::Remove, None) => En1993RowView::Remove(index, id),
            (En1993RowOp::Replace, Some(row)) => En1993RowView::Replace(index, id, row),
            _ => En1993RowView::Malformed(index),
        }
    }

    fn insert(index: usize, row: LoadCase) -> Self {
        Self { op: En1993RowOp::Insert, index: slot(index), id: Self::key(&row), value: Some(row) }
    }

    fn remove(index: usize, id: String) -> Self {
        Self { op: En1993RowOp::Remove, index: slot(index), id, value: None }
    }

    fn replace(index: usize, id: String, row: LoadCase) -> Self {
        Self { op: En1993RowOp::Replace, index: slot(index), id, value: Some(row) }
    }
}

/// 🧾️ One ordered-row edit of `member_actions`.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct En1993MemberActionEdit {
    pub op: En1993RowOp,
    pub index: u32,
    pub id: String,
    pub value: Option<MemberAction>,
}

impl En1993RowEdit for En1993MemberActionEdit {
    type Row = MemberAction;
    fn key(row: &MemberAction) -> String {
        row.id.clone()
    }

    fn view(&self) -> En1993RowView<'_, MemberAction> {
        let (index, id) = (self.index as usize, self.id.as_str());
        match (self.op, &self.value) {
            (En1993RowOp::Insert, Some(row)) => En1993RowView::Insert(index, row),
            (En1993RowOp::Remove, None) => En1993RowView::Remove(index, id),
            (En1993RowOp::Replace, Some(row)) => En1993RowView::Replace(index, id, row),
            _ => En1993RowView::Malformed(index),
        }
    }

    fn insert(index: usize, row: MemberAction) -> Self {
        Self { op: En1993RowOp::Insert, index: slot(index), id: Self::key(&row), value: Some(row) }
    }

    fn remove(index: usize, id: String) -> Self {
        Self { op: En1993RowOp::Remove, index: slot(index), id, value: None }
    }

    fn replace(index: usize, id: String, row: MemberAction) -> Self {
        Self { op: En1993RowOp::Replace, index: slot(index), id, value: Some(row) }
    }
}

/// 🧾️ One ordered-row edit of `joints`.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct En1993JointEdit {
    pub op: En1993RowOp,
    pub index: u32,
    pub id: String,
    pub value: Option<SteelJoint>,
}

impl En1993RowEdit for En1993JointEdit {
    type Row = SteelJoint;
    fn key(row: &SteelJoint) -> String {
        row.id.clone()
    }

    fn view(&self) -> En1993RowView<'_, SteelJoint> {
        let (index, id) = (self.index as usize, self.id.as_str());
        match (self.op, &self.value) {
            (En1993RowOp::Insert, Some(row)) => En1993RowView::Insert(index, row),
            (En1993RowOp::Remove, None) => En1993RowView::Remove(index, id),
            (En1993RowOp::Replace, Some(row)) => En1993RowView::Replace(index, id, row),
            _ => En1993RowView::Malformed(index),
        }
    }

    fn insert(index: usize, row: SteelJoint) -> Self {
        Self { op: En1993RowOp::Insert, index: slot(index), id: Self::key(&row), value: Some(row) }
    }

    fn remove(index: usize, id: String) -> Self {
        Self { op: En1993RowOp::Remove, index: slot(index), id, value: None }
    }

    fn replace(index: usize, id: String, row: SteelJoint) -> Self {
        Self { op: En1993RowOp::Replace, index: slot(index), id, value: Some(row) }
    }
}

/// 🧾️ One ordered-row edit of `fatigue_details`.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct En1993FatigueDetailEdit {
    pub op: En1993RowOp,
    pub index: u32,
    pub id: String,
    pub value: Option<FatigueDetail>,
}

impl En1993RowEdit for En1993FatigueDetailEdit {
    type Row = FatigueDetail;
    fn key(row: &FatigueDetail) -> String {
        row.id.clone()
    }

    fn view(&self) -> En1993RowView<'_, FatigueDetail> {
        let (index, id) = (self.index as usize, self.id.as_str());
        match (self.op, &self.value) {
            (En1993RowOp::Insert, Some(row)) => En1993RowView::Insert(index, row),
            (En1993RowOp::Remove, None) => En1993RowView::Remove(index, id),
            (En1993RowOp::Replace, Some(row)) => En1993RowView::Replace(index, id, row),
            _ => En1993RowView::Malformed(index),
        }
    }

    fn insert(index: usize, row: FatigueDetail) -> Self {
        Self { op: En1993RowOp::Insert, index: slot(index), id: Self::key(&row), value: Some(row) }
    }

    fn remove(index: usize, id: String) -> Self {
        Self { op: En1993RowOp::Remove, index: slot(index), id, value: None }
    }

    fn replace(index: usize, id: String, row: FatigueDetail) -> Self {
        Self { op: En1993RowOp::Replace, index: slot(index), id, value: Some(row) }
    }
}

/// 🧾️ One ordered-row edit of `fire_exposures`.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct En1993FireExposureEdit {
    pub op: En1993RowOp,
    pub index: u32,
    pub id: String,
    pub value: Option<FireExposure>,
}

impl En1993RowEdit for En1993FireExposureEdit {
    type Row = FireExposure;
    fn key(row: &FireExposure) -> String {
        row.id.clone()
    }

    fn view(&self) -> En1993RowView<'_, FireExposure> {
        let (index, id) = (self.index as usize, self.id.as_str());
        match (self.op, &self.value) {
            (En1993RowOp::Insert, Some(row)) => En1993RowView::Insert(index, row),
            (En1993RowOp::Remove, None) => En1993RowView::Remove(index, id),
            (En1993RowOp::Replace, Some(row)) => En1993RowView::Replace(index, id, row),
            _ => En1993RowView::Malformed(index),
        }
    }

    fn insert(index: usize, row: FireExposure) -> Self {
        Self { op: En1993RowOp::Insert, index: slot(index), id: Self::key(&row), value: Some(row) }
    }

    fn remove(index: usize, id: String) -> Self {
        Self { op: En1993RowOp::Remove, index: slot(index), id, value: None }
    }

    fn replace(index: usize, id: String, row: FireExposure) -> Self {
        Self { op: En1993RowOp::Replace, index: slot(index), id, value: Some(row) }
    }
}

/// 🧾️ One ordered-row edit of `cold_formed_members`.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct En1993ColdFormedMemberEdit {
    pub op: En1993RowOp,
    pub index: u32,
    pub id: String,
    pub value: Option<ColdFormedMember>,
}

impl En1993RowEdit for En1993ColdFormedMemberEdit {
    type Row = ColdFormedMember;
    fn key(row: &ColdFormedMember) -> String {
        row.id.clone()
    }

    fn view(&self) -> En1993RowView<'_, ColdFormedMember> {
        let (index, id) = (self.index as usize, self.id.as_str());
        match (self.op, &self.value) {
            (En1993RowOp::Insert, Some(row)) => En1993RowView::Insert(index, row),
            (En1993RowOp::Remove, None) => En1993RowView::Remove(index, id),
            (En1993RowOp::Replace, Some(row)) => En1993RowView::Replace(index, id, row),
            _ => En1993RowView::Malformed(index),
        }
    }

    fn insert(index: usize, row: ColdFormedMember) -> Self {
        Self { op: En1993RowOp::Insert, index: slot(index), id: Self::key(&row), value: Some(row) }
    }

    fn remove(index: usize, id: String) -> Self {
        Self { op: En1993RowOp::Remove, index: slot(index), id, value: None }
    }

    fn replace(index: usize, id: String, row: ColdFormedMember) -> Self {
        Self { op: En1993RowOp::Replace, index: slot(index), id, value: Some(row) }
    }
}

/// 🧾️ One ordered-row edit of `plated_panels`.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct En1993PlatedPanelEdit {
    pub op: En1993RowOp,
    pub index: u32,
    pub id: String,
    pub value: Option<PlatedPanel>,
}

impl En1993RowEdit for En1993PlatedPanelEdit {
    type Row = PlatedPanel;
    fn key(row: &PlatedPanel) -> String {
        row.id.clone()
    }

    fn view(&self) -> En1993RowView<'_, PlatedPanel> {
        let (index, id) = (self.index as usize, self.id.as_str());
        match (self.op, &self.value) {
            (En1993RowOp::Insert, Some(row)) => En1993RowView::Insert(index, row),
            (En1993RowOp::Remove, None) => En1993RowView::Remove(index, id),
            (En1993RowOp::Replace, Some(row)) => En1993RowView::Replace(index, id, row),
            _ => En1993RowView::Malformed(index),
        }
    }

    fn insert(index: usize, row: PlatedPanel) -> Self {
        Self { op: En1993RowOp::Insert, index: slot(index), id: Self::key(&row), value: Some(row) }
    }

    fn remove(index: usize, id: String) -> Self {
        Self { op: En1993RowOp::Remove, index: slot(index), id, value: None }
    }

    fn replace(index: usize, id: String, row: PlatedPanel) -> Self {
        Self { op: En1993RowOp::Replace, index: slot(index), id, value: Some(row) }
    }
}

/// 🧾️ One ordered-row edit of `silo_shells`.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct En1993SiloShellEdit {
    pub op: En1993RowOp,
    pub index: u32,
    pub id: String,
    pub value: Option<SiloShell>,
}

impl En1993RowEdit for En1993SiloShellEdit {
    type Row = SiloShell;
    fn key(row: &SiloShell) -> String {
        row.id.clone()
    }

    fn view(&self) -> En1993RowView<'_, SiloShell> {
        let (index, id) = (self.index as usize, self.id.as_str());
        match (self.op, &self.value) {
            (En1993RowOp::Insert, Some(row)) => En1993RowView::Insert(index, row),
            (En1993RowOp::Remove, None) => En1993RowView::Remove(index, id),
            (En1993RowOp::Replace, Some(row)) => En1993RowView::Replace(index, id, row),
            _ => En1993RowView::Malformed(index),
        }
    }

    fn insert(index: usize, row: SiloShell) -> Self {
        Self { op: En1993RowOp::Insert, index: slot(index), id: Self::key(&row), value: Some(row) }
    }

    fn remove(index: usize, id: String) -> Self {
        Self { op: En1993RowOp::Remove, index: slot(index), id, value: None }
    }

    fn replace(index: usize, id: String, row: SiloShell) -> Self {
        Self { op: En1993RowOp::Replace, index: slot(index), id, value: Some(row) }
    }
}

/// 🧾️ One ordered-row edit of `tension_components`.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct En1993TensionComponentEdit {
    pub op: En1993RowOp,
    pub index: u32,
    pub id: String,
    pub value: Option<TensionComponent>,
}

impl En1993RowEdit for En1993TensionComponentEdit {
    type Row = TensionComponent;
    fn key(row: &TensionComponent) -> String {
        row.id.clone()
    }

    fn view(&self) -> En1993RowView<'_, TensionComponent> {
        let (index, id) = (self.index as usize, self.id.as_str());
        match (self.op, &self.value) {
            (En1993RowOp::Insert, Some(row)) => En1993RowView::Insert(index, row),
            (En1993RowOp::Remove, None) => En1993RowView::Remove(index, id),
            (En1993RowOp::Replace, Some(row)) => En1993RowView::Replace(index, id, row),
            _ => En1993RowView::Malformed(index),
        }
    }

    fn insert(index: usize, row: TensionComponent) -> Self {
        Self { op: En1993RowOp::Insert, index: slot(index), id: Self::key(&row), value: Some(row) }
    }

    fn remove(index: usize, id: String) -> Self {
        Self { op: En1993RowOp::Remove, index: slot(index), id, value: None }
    }

    fn replace(index: usize, id: String, row: TensionComponent) -> Self {
        Self { op: En1993RowOp::Replace, index: slot(index), id, value: Some(row) }
    }
}

/// 🧾️ One ordered-row edit of `bridge_fatigue`.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct En1993BridgeFatigueEdit {
    pub op: En1993RowOp,
    pub index: u32,
    pub id: String,
    pub value: Option<BridgeFatigue>,
}

impl En1993RowEdit for En1993BridgeFatigueEdit {
    type Row = BridgeFatigue;
    fn key(row: &BridgeFatigue) -> String {
        row.id.clone()
    }

    fn view(&self) -> En1993RowView<'_, BridgeFatigue> {
        let (index, id) = (self.index as usize, self.id.as_str());
        match (self.op, &self.value) {
            (En1993RowOp::Insert, Some(row)) => En1993RowView::Insert(index, row),
            (En1993RowOp::Remove, None) => En1993RowView::Remove(index, id),
            (En1993RowOp::Replace, Some(row)) => En1993RowView::Replace(index, id, row),
            _ => En1993RowView::Malformed(index),
        }
    }

    fn insert(index: usize, row: BridgeFatigue) -> Self {
        Self { op: En1993RowOp::Insert, index: slot(index), id: Self::key(&row), value: Some(row) }
    }

    fn remove(index: usize, id: String) -> Self {
        Self { op: En1993RowOp::Remove, index: slot(index), id, value: None }
    }

    fn replace(index: usize, id: String, row: BridgeFatigue) -> Self {
        Self { op: En1993RowOp::Replace, index: slot(index), id, value: Some(row) }
    }
}

/// 🧾️ One ordered-row edit of `tower_legs`.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct En1993TowerLegEdit {
    pub op: En1993RowOp,
    pub index: u32,
    pub id: String,
    pub value: Option<TowerLeg>,
}

impl En1993RowEdit for En1993TowerLegEdit {
    type Row = TowerLeg;
    fn key(row: &TowerLeg) -> String {
        row.id.clone()
    }

    fn view(&self) -> En1993RowView<'_, TowerLeg> {
        let (index, id) = (self.index as usize, self.id.as_str());
        match (self.op, &self.value) {
            (En1993RowOp::Insert, Some(row)) => En1993RowView::Insert(index, row),
            (En1993RowOp::Remove, None) => En1993RowView::Remove(index, id),
            (En1993RowOp::Replace, Some(row)) => En1993RowView::Replace(index, id, row),
            _ => En1993RowView::Malformed(index),
        }
    }

    fn insert(index: usize, row: TowerLeg) -> Self {
        Self { op: En1993RowOp::Insert, index: slot(index), id: Self::key(&row), value: Some(row) }
    }

    fn remove(index: usize, id: String) -> Self {
        Self { op: En1993RowOp::Remove, index: slot(index), id, value: None }
    }

    fn replace(index: usize, id: String, row: TowerLeg) -> Self {
        Self { op: En1993RowOp::Replace, index: slot(index), id, value: Some(row) }
    }
}

/// 🧾️ One ordered-row edit of `piles`.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct En1993PileEdit {
    pub op: En1993RowOp,
    pub index: u32,
    pub id: String,
    pub value: Option<SteelPile>,
}

impl En1993RowEdit for En1993PileEdit {
    type Row = SteelPile;
    fn key(row: &SteelPile) -> String {
        row.id.clone()
    }

    fn view(&self) -> En1993RowView<'_, SteelPile> {
        let (index, id) = (self.index as usize, self.id.as_str());
        match (self.op, &self.value) {
            (En1993RowOp::Insert, Some(row)) => En1993RowView::Insert(index, row),
            (En1993RowOp::Remove, None) => En1993RowView::Remove(index, id),
            (En1993RowOp::Replace, Some(row)) => En1993RowView::Replace(index, id, row),
            _ => En1993RowView::Malformed(index),
        }
    }

    fn insert(index: usize, row: SteelPile) -> Self {
        Self { op: En1993RowOp::Insert, index: slot(index), id: Self::key(&row), value: Some(row) }
    }

    fn remove(index: usize, id: String) -> Self {
        Self { op: En1993RowOp::Remove, index: slot(index), id, value: None }
    }

    fn replace(index: usize, id: String, row: SteelPile) -> Self {
        Self { op: En1993RowOp::Replace, index: slot(index), id, value: Some(row) }
    }
}

/// 🧾️ One ordered-row edit of `crane_runways`.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct En1993CraneRunwayEdit {
    pub op: En1993RowOp,
    pub index: u32,
    pub id: String,
    pub value: Option<CraneRunway>,
}

impl En1993RowEdit for En1993CraneRunwayEdit {
    type Row = CraneRunway;
    fn key(row: &CraneRunway) -> String {
        row.id.clone()
    }

    fn view(&self) -> En1993RowView<'_, CraneRunway> {
        let (index, id) = (self.index as usize, self.id.as_str());
        match (self.op, &self.value) {
            (En1993RowOp::Insert, Some(row)) => En1993RowView::Insert(index, row),
            (En1993RowOp::Remove, None) => En1993RowView::Remove(index, id),
            (En1993RowOp::Replace, Some(row)) => En1993RowView::Replace(index, id, row),
            _ => En1993RowView::Malformed(index),
        }
    }

    fn insert(index: usize, row: CraneRunway) -> Self {
        Self { op: En1993RowOp::Insert, index: slot(index), id: Self::key(&row), value: Some(row) }
    }

    fn remove(index: usize, id: String) -> Self {
        Self { op: En1993RowOp::Remove, index: slot(index), id, value: None }
    }

    fn replace(index: usize, id: String, row: CraneRunway) -> Self {
        Self { op: En1993RowOp::Replace, index: slot(index), id, value: Some(row) }
    }
}

impl protocol::MutationDiff<En1993Snapshot> for En1993Diff {
    fn apply(&self, base: &En1993Snapshot, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<En1993Snapshot> {
        let mut next = base.clone();
        if let Some(value) = &self.annex {
            next.annex = value.clone();
        }
        apply_rows::<En1993MaterialEdit>(&mut next.materials, &self.materials).map_err(|error| error.under(["materials"]))?;
        apply_rows::<En1993SectionEdit>(&mut next.sections, &self.sections).map_err(|error| error.under(["sections"]))?;
        apply_rows::<En1993MemberEdit>(&mut next.members, &self.members).map_err(|error| error.under(["members"]))?;
        apply_rows::<En1993LoadCaseEdit>(&mut next.load_cases, &self.load_cases).map_err(|error| error.under(["loadCases"]))?;
        apply_rows::<En1993MemberActionEdit>(&mut next.member_actions, &self.member_actions).map_err(|error| error.under(["memberActions"]))?;
        apply_rows::<En1993JointEdit>(&mut next.joints, &self.joints).map_err(|error| error.under(["joints"]))?;
        apply_rows::<En1993FatigueDetailEdit>(&mut next.fatigue_details, &self.fatigue_details).map_err(|error| error.under(["fatigueDetails"]))?;
        apply_rows::<En1993FireExposureEdit>(&mut next.fire_exposures, &self.fire_exposures).map_err(|error| error.under(["fireExposures"]))?;
        apply_rows::<En1993ColdFormedMemberEdit>(&mut next.cold_formed_members, &self.cold_formed_members).map_err(|error| error.under(["coldFormedMembers"]))?;
        apply_rows::<En1993PlatedPanelEdit>(&mut next.plated_panels, &self.plated_panels).map_err(|error| error.under(["platedPanels"]))?;
        apply_rows::<En1993SiloShellEdit>(&mut next.silo_shells, &self.silo_shells).map_err(|error| error.under(["siloShells"]))?;
        apply_rows::<En1993TensionComponentEdit>(&mut next.tension_components, &self.tension_components).map_err(|error| error.under(["tensionComponents"]))?;
        apply_rows::<En1993BridgeFatigueEdit>(&mut next.bridge_fatigue, &self.bridge_fatigue).map_err(|error| error.under(["bridgeFatigue"]))?;
        apply_rows::<En1993TowerLegEdit>(&mut next.tower_legs, &self.tower_legs).map_err(|error| error.under(["towerLegs"]))?;
        apply_rows::<En1993PileEdit>(&mut next.piles, &self.piles).map_err(|error| error.under(["piles"]))?;
        apply_rows::<En1993CraneRunwayEdit>(&mut next.crane_runways, &self.crane_runways).map_err(|error| error.under(["craneRunways"]))?;
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        if other.annex.is_some() {
            self.annex = other.annex;
        }
        for edit in other.materials {
            push_row(&mut self.materials, edit);
        }
        for edit in other.sections {
            push_row(&mut self.sections, edit);
        }
        for edit in other.members {
            push_row(&mut self.members, edit);
        }
        for edit in other.load_cases {
            push_row(&mut self.load_cases, edit);
        }
        for edit in other.member_actions {
            push_row(&mut self.member_actions, edit);
        }
        for edit in other.joints {
            push_row(&mut self.joints, edit);
        }
        for edit in other.fatigue_details {
            push_row(&mut self.fatigue_details, edit);
        }
        for edit in other.fire_exposures {
            push_row(&mut self.fire_exposures, edit);
        }
        for edit in other.cold_formed_members {
            push_row(&mut self.cold_formed_members, edit);
        }
        for edit in other.plated_panels {
            push_row(&mut self.plated_panels, edit);
        }
        for edit in other.silo_shells {
            push_row(&mut self.silo_shells, edit);
        }
        for edit in other.tension_components {
            push_row(&mut self.tension_components, edit);
        }
        for edit in other.bridge_fatigue {
            push_row(&mut self.bridge_fatigue, edit);
        }
        for edit in other.tower_legs {
            push_row(&mut self.tower_legs, edit);
        }
        for edit in other.piles {
            push_row(&mut self.piles, edit);
        }
        for edit in other.crane_runways {
            push_row(&mut self.crane_runways, edit);
        }
    }
}

impl protocol::DiffAlgebra<En1993Snapshot> for En1993Diff {
    fn inverse(&self, base: &En1993Snapshot) -> Self {
        Self {
            annex: self.annex.as_ref().map(|_| base.annex.clone()),
            materials: invert_rows::<En1993MaterialEdit>(&base.materials, &self.materials),
            sections: invert_rows::<En1993SectionEdit>(&base.sections, &self.sections),
            members: invert_rows::<En1993MemberEdit>(&base.members, &self.members),
            load_cases: invert_rows::<En1993LoadCaseEdit>(&base.load_cases, &self.load_cases),
            member_actions: invert_rows::<En1993MemberActionEdit>(&base.member_actions, &self.member_actions),
            joints: invert_rows::<En1993JointEdit>(&base.joints, &self.joints),
            fatigue_details: invert_rows::<En1993FatigueDetailEdit>(&base.fatigue_details, &self.fatigue_details),
            fire_exposures: invert_rows::<En1993FireExposureEdit>(&base.fire_exposures, &self.fire_exposures),
            cold_formed_members: invert_rows::<En1993ColdFormedMemberEdit>(&base.cold_formed_members, &self.cold_formed_members),
            plated_panels: invert_rows::<En1993PlatedPanelEdit>(&base.plated_panels, &self.plated_panels),
            silo_shells: invert_rows::<En1993SiloShellEdit>(&base.silo_shells, &self.silo_shells),
            tension_components: invert_rows::<En1993TensionComponentEdit>(&base.tension_components, &self.tension_components),
            bridge_fatigue: invert_rows::<En1993BridgeFatigueEdit>(&base.bridge_fatigue, &self.bridge_fatigue),
            tower_legs: invert_rows::<En1993TowerLegEdit>(&base.tower_legs, &self.tower_legs),
            piles: invert_rows::<En1993PileEdit>(&base.piles, &self.piles),
            crane_runways: invert_rows::<En1993CraneRunwayEdit>(&base.crane_runways, &self.crane_runways),
        }
    }

    fn between(base: &En1993Snapshot, other: &En1993Snapshot) -> Self {
        Self {
            annex: (base.annex != other.annex).then(|| other.annex.clone()),
            materials: rows_between::<En1993MaterialEdit>(&base.materials, &other.materials),
            sections: rows_between::<En1993SectionEdit>(&base.sections, &other.sections),
            members: rows_between::<En1993MemberEdit>(&base.members, &other.members),
            load_cases: rows_between::<En1993LoadCaseEdit>(&base.load_cases, &other.load_cases),
            member_actions: rows_between::<En1993MemberActionEdit>(&base.member_actions, &other.member_actions),
            joints: rows_between::<En1993JointEdit>(&base.joints, &other.joints),
            fatigue_details: rows_between::<En1993FatigueDetailEdit>(&base.fatigue_details, &other.fatigue_details),
            fire_exposures: rows_between::<En1993FireExposureEdit>(&base.fire_exposures, &other.fire_exposures),
            cold_formed_members: rows_between::<En1993ColdFormedMemberEdit>(&base.cold_formed_members, &other.cold_formed_members),
            plated_panels: rows_between::<En1993PlatedPanelEdit>(&base.plated_panels, &other.plated_panels),
            silo_shells: rows_between::<En1993SiloShellEdit>(&base.silo_shells, &other.silo_shells),
            tension_components: rows_between::<En1993TensionComponentEdit>(&base.tension_components, &other.tension_components),
            bridge_fatigue: rows_between::<En1993BridgeFatigueEdit>(&base.bridge_fatigue, &other.bridge_fatigue),
            tower_legs: rows_between::<En1993TowerLegEdit>(&base.tower_legs, &other.tower_legs),
            piles: rows_between::<En1993PileEdit>(&base.piles, &other.piles),
            crane_runways: rows_between::<En1993CraneRunwayEdit>(&base.crane_runways, &other.crane_runways),
        }
    }

    fn is_empty(&self) -> bool {
        self.annex.is_none() && self.materials.is_empty() && self.sections.is_empty() && self.members.is_empty() && self.load_cases.is_empty() && self.member_actions.is_empty() && self.joints.is_empty() && self.fatigue_details.is_empty() && self.fire_exposures.is_empty() && self.cold_formed_members.is_empty() && self.plated_panels.is_empty() && self.silo_shells.is_empty() && self.tension_components.is_empty() && self.bridge_fatigue.is_empty() && self.tower_legs.is_empty() && self.piles.is_empty() && self.crane_runways.is_empty()
    }
}
