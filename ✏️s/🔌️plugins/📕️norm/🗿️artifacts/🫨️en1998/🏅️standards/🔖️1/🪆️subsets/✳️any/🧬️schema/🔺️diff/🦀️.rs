//! 🧬️ En1998 diff schema — sparse scalar fields and ordered per-collection row edits.

use crate::{En1998Assessment, En1998Bridge, En1998Building, En1998Foundation, En1998Member, En1998RetainingWall, En1998Silo, En1998Site, En1998Snapshot, En1998Storey, En1998System, En1998Tank, En1998Tower};
use framework_schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Sparse delta for the En1998 artifact: the changed annex and site, and ordered row edits with field patches per collection.
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.norm.en1998")]
pub struct En1998Diff {
    #[state(artifact)]
    pub annex: Option<String>,
    #[state(artifact)]
    pub site: Option<En1998Site>,
    #[state(artifact)]
    pub buildings: Vec<En1998BuildingEdit>,
    #[state(artifact)]
    pub bridges: Vec<En1998BridgeEdit>,
    #[state(artifact)]
    pub assessments: Vec<En1998AssessmentEdit>,
    #[state(artifact)]
    pub silos: Vec<En1998SiloEdit>,
    #[state(artifact)]
    pub tanks: Vec<En1998TankEdit>,
    #[state(artifact)]
    pub foundations: Vec<En1998FoundationEdit>,
    #[state(artifact)]
    pub retaining_walls: Vec<En1998RetainingWallEdit>,
    #[state(artifact)]
    pub towers: Vec<En1998TowerEdit>,
}
//#endregion 🔖️Diff

//#region 🔖️RowEdits
/// 🧾️ The operation one ordered-row edit performs on its collection.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub enum En1998RowOp {
    #[dsl(key = "insert")]
    Insert,
    #[dsl(key = "remove")]
    Remove,
    #[dsl(key = "replace")]
    Replace,
    #[dsl(key = "patch")]
    Patch,
}

/// 🔭️ What one row edit says, read without caring which collection it edits.
pub enum En1998RowView<'a, T, Q> {
    Insert(usize, &'a T),
    Remove(usize, &'a str),
    Replace(usize, &'a str, &'a T),
    Patch(usize, &'a str, &'a Q),
    Malformed(usize),
}

/// 🩹️ A sparse field patch of one row: sets only the fields it names and edits its nested collections.
pub trait En1998RowPatch<T>: Clone + PartialEq + Sized {
    fn apply(&self, row: &mut T) -> Result<(), protocol::MutationApplyError>;
    fn inverse(&self, row: &T) -> Self;
    fn absorb(&mut self, other: Self);
}

/// 🧷️ One ordered-row edit of a collection: how it reads, and how the engine builds edits of its own kind.
pub trait En1998RowEdit: Clone + PartialEq + Sized {
    type Row: Clone + PartialEq;
    type Patch: En1998RowPatch<Self::Row>;
    fn key(row: &Self::Row) -> String;
    fn view(&self) -> En1998RowView<'_, Self::Row, Self::Patch>;
    fn insert(index: usize, row: Self::Row) -> Self;
    fn remove(index: usize, id: String) -> Self;
    fn replace(index: usize, id: String, row: Self::Row) -> Self;
    fn patch(index: usize, id: String, patch: Self::Patch) -> Self;
}

fn slot(index: usize) -> u32 {
    u32::try_from(index).unwrap_or(u32::MAX)
}

fn row_fault(code: &str, index: usize, id: &str) -> protocol::MutationApplyError {
    protocol::MutationApplyError::new(format!("mutation.apply.{code}"), format!("row {index} ('{id}') cannot take the edit.")).at([index.to_string()])
}

fn guard<E: En1998RowEdit>(rows: &[E::Row], index: usize, id: &str) -> Result<(), protocol::MutationApplyError> {
    match rows.get(index) {
        Some(row) if E::key(row) == id => Ok(()),
        Some(_) => Err(row_fault("row-key-mismatch", index, id)),
        None => Err(row_fault("row-out-of-range", index, id)),
    }
}

fn apply_rows<E: En1998RowEdit>(rows: &mut Vec<E::Row>, edits: &[E]) -> Result<(), protocol::MutationApplyError> {
    for edit in edits {
        match edit.view() {
            En1998RowView::Insert(index, row) if index <= rows.len() => rows.insert(index, row.clone()),
            En1998RowView::Insert(index, row) => return Err(row_fault("row-out-of-range", index, &E::key(row))),
            En1998RowView::Remove(index, id) => {
                guard::<E>(rows, index, id)?;
                rows.remove(index);
            }
            En1998RowView::Replace(index, id, row) => {
                guard::<E>(rows, index, id)?;
                rows[index] = row.clone();
            }
            En1998RowView::Patch(index, id, patch) => {
                guard::<E>(rows, index, id)?;
                En1998RowPatch::apply(patch, &mut rows[index]).map_err(|error| error.under([index.to_string()]))?;
            }
            En1998RowView::Malformed(index) => return Err(row_fault("row-malformed", index, "")),
        }
    }
    Ok(())
}

fn invert_rows<E: En1998RowEdit>(base: &[E::Row], edits: &[E]) -> Vec<E> {
    let mut rows = base.to_vec();
    let mut inverse = Vec::with_capacity(edits.len());
    for edit in edits {
        let step = match edit.view() {
            En1998RowView::Insert(index, row) => E::remove(index, E::key(row)),
            En1998RowView::Remove(index, _) => match rows.get(index) {
                Some(old) => E::insert(index, old.clone()),
                None => break,
            },
            En1998RowView::Replace(index, _, row) => match rows.get(index) {
                Some(old) => E::replace(index, E::key(row), old.clone()),
                None => break,
            },
            En1998RowView::Patch(index, id, patch) => match rows.get(index) {
                Some(old) => E::patch(index, id.to_string(), En1998RowPatch::inverse(patch, old)),
                None => break,
            },
            En1998RowView::Malformed(_) => break,
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

fn merge_rows<E: En1998RowEdit>(first: &E, next: &E) -> RowMerge<E> {
    match (first.view(), next.view()) {
        (En1998RowView::Insert(at, row), En1998RowView::Remove(to, id)) if at == to && E::key(row) == id => RowMerge::Cancel,
        (En1998RowView::Insert(at, row), En1998RowView::Replace(to, id, with)) if at == to && E::key(row) == id => RowMerge::Into(E::insert(at, with.clone())),
        (En1998RowView::Replace(at, was, row), En1998RowView::Replace(to, id, with)) if at == to && E::key(row) == id => RowMerge::Into(E::replace(at, was.to_string(), with.clone())),
        (En1998RowView::Replace(at, was, row), En1998RowView::Remove(to, id)) if at == to && E::key(row) == id => RowMerge::Into(E::remove(at, was.to_string())),
        (En1998RowView::Remove(at, was), En1998RowView::Insert(to, row)) if at == to && E::key(row) == was => RowMerge::Into(E::replace(at, was.to_string(), row.clone())),
        (En1998RowView::Insert(at, row), En1998RowView::Patch(to, id, patch)) if at == to && E::key(row) == id => {
            let mut row = row.clone();
            match En1998RowPatch::apply(patch, &mut row) {
                Ok(()) => RowMerge::Into(E::insert(at, row)),
                Err(_) => RowMerge::Keep,
            }
        }
        (En1998RowView::Replace(at, was, row), En1998RowView::Patch(to, id, patch)) if at == to && E::key(row) == id => {
            let mut row = row.clone();
            match En1998RowPatch::apply(patch, &mut row) {
                Ok(()) => RowMerge::Into(E::replace(at, was.to_string(), row)),
                Err(_) => RowMerge::Keep,
            }
        }
        (En1998RowView::Patch(at, was, own), En1998RowView::Patch(to, id, patch)) if at == to && was == id => {
            let mut own = own.clone();
            En1998RowPatch::absorb(&mut own, patch.clone());
            RowMerge::Into(E::patch(at, was.to_string(), own))
        }
        (En1998RowView::Patch(at, was, _), En1998RowView::Remove(to, id)) if at == to && was == id => RowMerge::Into(E::remove(at, was.to_string())),
        (En1998RowView::Patch(at, was, _), En1998RowView::Replace(to, id, with)) if at == to && was == id => RowMerge::Into(E::replace(at, was.to_string(), with.clone())),
        _ => RowMerge::Keep,
    }
}

fn push_row<E: En1998RowEdit>(edits: &mut Vec<E>, next: E) {
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

fn positional_rows<E: En1998RowEdit>(base: &[E::Row], other: &[E::Row]) -> Vec<E> {
    let shared = base.iter().zip(other).take_while(|(left, right)| left == right).count();
    let removed = (shared..base.len()).rev().map(|index| E::remove(index, E::key(&base[index])));
    let inserted = (shared..other.len()).map(|index| E::insert(index, other[index].clone()));
    removed.chain(inserted).collect()
}

fn rows_between<E: En1998RowEdit>(base: &[E::Row], other: &[E::Row]) -> Vec<E> {
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

/// 🧾️ One ordered-row edit of `buildings`.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct En1998BuildingEdit {
    pub op: En1998RowOp,
    pub index: u32,
    pub id: String,
    pub value: Option<En1998Building>,
    pub patch: Option<En1998BuildingPatch>,
}

impl En1998RowEdit for En1998BuildingEdit {
    type Row = En1998Building;
    type Patch = En1998BuildingPatch;
    fn key(row: &En1998Building) -> String {
        row.id.clone()
    }

    fn view(&self) -> En1998RowView<'_, En1998Building, En1998BuildingPatch> {
        let (index, id) = (self.index as usize, self.id.as_str());
        match (self.op, &self.value, &self.patch) {
            (En1998RowOp::Insert, Some(row), None) => En1998RowView::Insert(index, row),
            (En1998RowOp::Remove, None, None) => En1998RowView::Remove(index, id),
            (En1998RowOp::Replace, Some(row), None) => En1998RowView::Replace(index, id, row),
            (En1998RowOp::Patch, None, Some(patch)) => En1998RowView::Patch(index, id, patch),
            _ => En1998RowView::Malformed(index),
        }
    }

    fn insert(index: usize, row: En1998Building) -> Self {
        Self { op: En1998RowOp::Insert, index: slot(index), id: Self::key(&row), value: Some(row), patch: None }
    }

    fn remove(index: usize, id: String) -> Self {
        Self { op: En1998RowOp::Remove, index: slot(index), id, value: None, patch: None }
    }

    fn replace(index: usize, id: String, row: En1998Building) -> Self {
        Self { op: En1998RowOp::Replace, index: slot(index), id, value: Some(row), patch: None }
    }

    fn patch(index: usize, id: String, patch: En1998BuildingPatch) -> Self {
        Self { op: En1998RowOp::Patch, index: slot(index), id, value: None, patch: Some(patch) }
    }
}

/// 🩹️ Sparse field patch of one `En1998Building`: the fields it names replace the row's, its nested edits edit the row's nested collections.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1998BuildingPatch {
    pub plan_regular: Option<bool>,
    pub elevation_regular: Option<bool>,
    pub masonry_wall_area_ratio: Option<f64>,
    pub systems: Vec<En1998SystemEdit>,
    pub storeys: Vec<En1998StoreyEdit>,
    pub members: Vec<En1998MemberEdit>,
}

impl En1998RowPatch<En1998Building> for En1998BuildingPatch {
    fn apply(&self, row: &mut En1998Building) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.plan_regular {
            row.plan_regular = value.clone();
        }
        if let Some(value) = &self.elevation_regular {
            row.elevation_regular = value.clone();
        }
        if let Some(value) = &self.masonry_wall_area_ratio {
            row.masonry_wall_area_ratio = value.clone();
        }
        apply_rows::<En1998SystemEdit>(&mut row.systems, &self.systems).map_err(|error| error.under(["systems"]))?;
        apply_rows::<En1998StoreyEdit>(&mut row.storeys, &self.storeys).map_err(|error| error.under(["storeys"]))?;
        apply_rows::<En1998MemberEdit>(&mut row.members, &self.members).map_err(|error| error.under(["members"]))?;
        Ok(())
    }

    fn inverse(&self, row: &En1998Building) -> Self {
        Self {
            plan_regular: self.plan_regular.as_ref().map(|_| row.plan_regular.clone()),
            elevation_regular: self.elevation_regular.as_ref().map(|_| row.elevation_regular.clone()),
            masonry_wall_area_ratio: self.masonry_wall_area_ratio.as_ref().map(|_| row.masonry_wall_area_ratio.clone()),
            systems: invert_rows::<En1998SystemEdit>(&row.systems, &self.systems),
            storeys: invert_rows::<En1998StoreyEdit>(&row.storeys, &self.storeys),
            members: invert_rows::<En1998MemberEdit>(&row.members, &self.members),
        }
    }

    fn absorb(&mut self, other: Self) {
        if other.plan_regular.is_some() {
            self.plan_regular = other.plan_regular;
        }
        if other.elevation_regular.is_some() {
            self.elevation_regular = other.elevation_regular;
        }
        if other.masonry_wall_area_ratio.is_some() {
            self.masonry_wall_area_ratio = other.masonry_wall_area_ratio;
        }
        for edit in other.systems {
            push_row(&mut self.systems, edit);
        }
        for edit in other.storeys {
            push_row(&mut self.storeys, edit);
        }
        for edit in other.members {
            push_row(&mut self.members, edit);
        }
    }
}

/// 🧾️ One ordered-row edit of `bridges`.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct En1998BridgeEdit {
    pub op: En1998RowOp,
    pub index: u32,
    pub id: String,
    pub value: Option<En1998Bridge>,
    pub patch: Option<En1998BridgePatch>,
}

impl En1998RowEdit for En1998BridgeEdit {
    type Row = En1998Bridge;
    type Patch = En1998BridgePatch;
    fn key(row: &En1998Bridge) -> String {
        row.id.clone()
    }

    fn view(&self) -> En1998RowView<'_, En1998Bridge, En1998BridgePatch> {
        let (index, id) = (self.index as usize, self.id.as_str());
        match (self.op, &self.value, &self.patch) {
            (En1998RowOp::Insert, Some(row), None) => En1998RowView::Insert(index, row),
            (En1998RowOp::Remove, None, None) => En1998RowView::Remove(index, id),
            (En1998RowOp::Replace, Some(row), None) => En1998RowView::Replace(index, id, row),
            (En1998RowOp::Patch, None, Some(patch)) => En1998RowView::Patch(index, id, patch),
            _ => En1998RowView::Malformed(index),
        }
    }

    fn insert(index: usize, row: En1998Bridge) -> Self {
        Self { op: En1998RowOp::Insert, index: slot(index), id: Self::key(&row), value: Some(row), patch: None }
    }

    fn remove(index: usize, id: String) -> Self {
        Self { op: En1998RowOp::Remove, index: slot(index), id, value: None, patch: None }
    }

    fn replace(index: usize, id: String, row: En1998Bridge) -> Self {
        Self { op: En1998RowOp::Replace, index: slot(index), id, value: Some(row), patch: None }
    }

    fn patch(index: usize, id: String, patch: En1998BridgePatch) -> Self {
        Self { op: En1998RowOp::Patch, index: slot(index), id, value: None, patch: Some(patch) }
    }
}

/// 🩹️ Sparse field patch of one `En1998Bridge`: the fields it names replace the row's, its nested edits edit the row's nested collections.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1998BridgePatch {
    pub v_rd_n: Option<f64>,
}

impl En1998RowPatch<En1998Bridge> for En1998BridgePatch {
    fn apply(&self, row: &mut En1998Bridge) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.v_rd_n {
            row.v_rd_n = value.clone();
        }
        Ok(())
    }

    fn inverse(&self, row: &En1998Bridge) -> Self {
        Self {
            v_rd_n: self.v_rd_n.as_ref().map(|_| row.v_rd_n.clone()),
        }
    }

    fn absorb(&mut self, other: Self) {
        if other.v_rd_n.is_some() {
            self.v_rd_n = other.v_rd_n;
        }
    }
}

/// 🧾️ One ordered-row edit of `assessments`.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct En1998AssessmentEdit {
    pub op: En1998RowOp,
    pub index: u32,
    pub id: String,
    pub value: Option<En1998Assessment>,
    pub patch: Option<En1998AssessmentPatch>,
}

impl En1998RowEdit for En1998AssessmentEdit {
    type Row = En1998Assessment;
    type Patch = En1998AssessmentPatch;
    fn key(row: &En1998Assessment) -> String {
        row.id.clone()
    }

    fn view(&self) -> En1998RowView<'_, En1998Assessment, En1998AssessmentPatch> {
        let (index, id) = (self.index as usize, self.id.as_str());
        match (self.op, &self.value, &self.patch) {
            (En1998RowOp::Insert, Some(row), None) => En1998RowView::Insert(index, row),
            (En1998RowOp::Remove, None, None) => En1998RowView::Remove(index, id),
            (En1998RowOp::Replace, Some(row), None) => En1998RowView::Replace(index, id, row),
            (En1998RowOp::Patch, None, Some(patch)) => En1998RowView::Patch(index, id, patch),
            _ => En1998RowView::Malformed(index),
        }
    }

    fn insert(index: usize, row: En1998Assessment) -> Self {
        Self { op: En1998RowOp::Insert, index: slot(index), id: Self::key(&row), value: Some(row), patch: None }
    }

    fn remove(index: usize, id: String) -> Self {
        Self { op: En1998RowOp::Remove, index: slot(index), id, value: None, patch: None }
    }

    fn replace(index: usize, id: String, row: En1998Assessment) -> Self {
        Self { op: En1998RowOp::Replace, index: slot(index), id, value: Some(row), patch: None }
    }

    fn patch(index: usize, id: String, patch: En1998AssessmentPatch) -> Self {
        Self { op: En1998RowOp::Patch, index: slot(index), id, value: None, patch: Some(patch) }
    }
}

/// 🩹️ Sparse field patch of one `En1998Assessment`: the fields it names replace the row's, its nested edits edit the row's nested collections.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1998AssessmentPatch {
    pub r_k_n: Option<f64>,
}

impl En1998RowPatch<En1998Assessment> for En1998AssessmentPatch {
    fn apply(&self, row: &mut En1998Assessment) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.r_k_n {
            row.r_k_n = value.clone();
        }
        Ok(())
    }

    fn inverse(&self, row: &En1998Assessment) -> Self {
        Self {
            r_k_n: self.r_k_n.as_ref().map(|_| row.r_k_n.clone()),
        }
    }

    fn absorb(&mut self, other: Self) {
        if other.r_k_n.is_some() {
            self.r_k_n = other.r_k_n;
        }
    }
}

/// 🧾️ One ordered-row edit of `silos`.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct En1998SiloEdit {
    pub op: En1998RowOp,
    pub index: u32,
    pub id: String,
    pub value: Option<En1998Silo>,
}

impl En1998RowEdit for En1998SiloEdit {
    type Row = En1998Silo;
    fn key(row: &En1998Silo) -> String {
        row.id.clone()
    }

    fn view(&self) -> En1998RowView<'_, En1998Silo> {
        let (index, id) = (self.index as usize, self.id.as_str());
        match (self.op, &self.value) {
            (En1998RowOp::Insert, Some(row)) => En1998RowView::Insert(index, row),
            (En1998RowOp::Remove, None) => En1998RowView::Remove(index, id),
            (En1998RowOp::Replace, Some(row)) => En1998RowView::Replace(index, id, row),
            _ => En1998RowView::Malformed(index),
        }
    }

    fn insert(index: usize, row: En1998Silo) -> Self {
        Self { op: En1998RowOp::Insert, index: slot(index), id: Self::key(&row), value: Some(row) }
    }

    fn remove(index: usize, id: String) -> Self {
        Self { op: En1998RowOp::Remove, index: slot(index), id, value: None }
    }

    fn replace(index: usize, id: String, row: En1998Silo) -> Self {
        Self { op: En1998RowOp::Replace, index: slot(index), id, value: Some(row) }
    }
}

/// 🧾️ One ordered-row edit of `tanks`.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct En1998TankEdit {
    pub op: En1998RowOp,
    pub index: u32,
    pub id: String,
    pub value: Option<En1998Tank>,
}

impl En1998RowEdit for En1998TankEdit {
    type Row = En1998Tank;
    fn key(row: &En1998Tank) -> String {
        row.id.clone()
    }

    fn view(&self) -> En1998RowView<'_, En1998Tank> {
        let (index, id) = (self.index as usize, self.id.as_str());
        match (self.op, &self.value) {
            (En1998RowOp::Insert, Some(row)) => En1998RowView::Insert(index, row),
            (En1998RowOp::Remove, None) => En1998RowView::Remove(index, id),
            (En1998RowOp::Replace, Some(row)) => En1998RowView::Replace(index, id, row),
            _ => En1998RowView::Malformed(index),
        }
    }

    fn insert(index: usize, row: En1998Tank) -> Self {
        Self { op: En1998RowOp::Insert, index: slot(index), id: Self::key(&row), value: Some(row) }
    }

    fn remove(index: usize, id: String) -> Self {
        Self { op: En1998RowOp::Remove, index: slot(index), id, value: None }
    }

    fn replace(index: usize, id: String, row: En1998Tank) -> Self {
        Self { op: En1998RowOp::Replace, index: slot(index), id, value: Some(row) }
    }
}

/// 🧾️ One ordered-row edit of `foundations`.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct En1998FoundationEdit {
    pub op: En1998RowOp,
    pub index: u32,
    pub id: String,
    pub value: Option<En1998Foundation>,
}

impl En1998RowEdit for En1998FoundationEdit {
    type Row = En1998Foundation;
    fn key(row: &En1998Foundation) -> String {
        row.id.clone()
    }

    fn view(&self) -> En1998RowView<'_, En1998Foundation> {
        let (index, id) = (self.index as usize, self.id.as_str());
        match (self.op, &self.value) {
            (En1998RowOp::Insert, Some(row)) => En1998RowView::Insert(index, row),
            (En1998RowOp::Remove, None) => En1998RowView::Remove(index, id),
            (En1998RowOp::Replace, Some(row)) => En1998RowView::Replace(index, id, row),
            _ => En1998RowView::Malformed(index),
        }
    }

    fn insert(index: usize, row: En1998Foundation) -> Self {
        Self { op: En1998RowOp::Insert, index: slot(index), id: Self::key(&row), value: Some(row) }
    }

    fn remove(index: usize, id: String) -> Self {
        Self { op: En1998RowOp::Remove, index: slot(index), id, value: None }
    }

    fn replace(index: usize, id: String, row: En1998Foundation) -> Self {
        Self { op: En1998RowOp::Replace, index: slot(index), id, value: Some(row) }
    }
}

/// 🧾️ One ordered-row edit of `retaining_walls`.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct En1998RetainingWallEdit {
    pub op: En1998RowOp,
    pub index: u32,
    pub id: String,
    pub value: Option<En1998RetainingWall>,
}

impl En1998RowEdit for En1998RetainingWallEdit {
    type Row = En1998RetainingWall;
    fn key(row: &En1998RetainingWall) -> String {
        row.id.clone()
    }

    fn view(&self) -> En1998RowView<'_, En1998RetainingWall> {
        let (index, id) = (self.index as usize, self.id.as_str());
        match (self.op, &self.value) {
            (En1998RowOp::Insert, Some(row)) => En1998RowView::Insert(index, row),
            (En1998RowOp::Remove, None) => En1998RowView::Remove(index, id),
            (En1998RowOp::Replace, Some(row)) => En1998RowView::Replace(index, id, row),
            _ => En1998RowView::Malformed(index),
        }
    }

    fn insert(index: usize, row: En1998RetainingWall) -> Self {
        Self { op: En1998RowOp::Insert, index: slot(index), id: Self::key(&row), value: Some(row) }
    }

    fn remove(index: usize, id: String) -> Self {
        Self { op: En1998RowOp::Remove, index: slot(index), id, value: None }
    }

    fn replace(index: usize, id: String, row: En1998RetainingWall) -> Self {
        Self { op: En1998RowOp::Replace, index: slot(index), id, value: Some(row) }
    }
}

/// 🧾️ One ordered-row edit of `towers`.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct En1998TowerEdit {
    pub op: En1998RowOp,
    pub index: u32,
    pub id: String,
    pub value: Option<En1998Tower>,
    pub patch: Option<En1998TowerPatch>,
}

impl En1998RowEdit for En1998TowerEdit {
    type Row = En1998Tower;
    type Patch = En1998TowerPatch;
    fn key(row: &En1998Tower) -> String {
        row.id.clone()
    }

    fn view(&self) -> En1998RowView<'_, En1998Tower, En1998TowerPatch> {
        let (index, id) = (self.index as usize, self.id.as_str());
        match (self.op, &self.value, &self.patch) {
            (En1998RowOp::Insert, Some(row), None) => En1998RowView::Insert(index, row),
            (En1998RowOp::Remove, None, None) => En1998RowView::Remove(index, id),
            (En1998RowOp::Replace, Some(row), None) => En1998RowView::Replace(index, id, row),
            (En1998RowOp::Patch, None, Some(patch)) => En1998RowView::Patch(index, id, patch),
            _ => En1998RowView::Malformed(index),
        }
    }

    fn insert(index: usize, row: En1998Tower) -> Self {
        Self { op: En1998RowOp::Insert, index: slot(index), id: Self::key(&row), value: Some(row), patch: None }
    }

    fn remove(index: usize, id: String) -> Self {
        Self { op: En1998RowOp::Remove, index: slot(index), id, value: None, patch: None }
    }

    fn replace(index: usize, id: String, row: En1998Tower) -> Self {
        Self { op: En1998RowOp::Replace, index: slot(index), id, value: Some(row), patch: None }
    }

    fn patch(index: usize, id: String, patch: En1998TowerPatch) -> Self {
        Self { op: En1998RowOp::Patch, index: slot(index), id, value: None, patch: Some(patch) }
    }
}

/// 🩹️ Sparse field patch of one `En1998Tower`: the fields it names replace the row's, its nested edits edit the row's nested collections.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1998TowerPatch {
    pub m_rd_nm: Option<f64>,
}

impl En1998RowPatch<En1998Tower> for En1998TowerPatch {
    fn apply(&self, row: &mut En1998Tower) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.m_rd_nm {
            row.m_rd_nm = value.clone();
        }
        Ok(())
    }

    fn inverse(&self, row: &En1998Tower) -> Self {
        Self {
            m_rd_nm: self.m_rd_nm.as_ref().map(|_| row.m_rd_nm.clone()),
        }
    }

    fn absorb(&mut self, other: Self) {
        if other.m_rd_nm.is_some() {
            self.m_rd_nm = other.m_rd_nm;
        }
    }
}

/// 🧾️ One ordered-row edit of `systems`.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct En1998SystemEdit {
    pub op: En1998RowOp,
    pub index: u32,
    pub id: String,
    pub value: Option<En1998System>,
    pub patch: Option<En1998SystemPatch>,
}

impl En1998RowEdit for En1998SystemEdit {
    type Row = En1998System;
    type Patch = En1998SystemPatch;
    fn key(row: &En1998System) -> String {
        row.id.clone()
    }

    fn view(&self) -> En1998RowView<'_, En1998System, En1998SystemPatch> {
        let (index, id) = (self.index as usize, self.id.as_str());
        match (self.op, &self.value, &self.patch) {
            (En1998RowOp::Insert, Some(row), None) => En1998RowView::Insert(index, row),
            (En1998RowOp::Remove, None, None) => En1998RowView::Remove(index, id),
            (En1998RowOp::Replace, Some(row), None) => En1998RowView::Replace(index, id, row),
            (En1998RowOp::Patch, None, Some(patch)) => En1998RowView::Patch(index, id, patch),
            _ => En1998RowView::Malformed(index),
        }
    }

    fn insert(index: usize, row: En1998System) -> Self {
        Self { op: En1998RowOp::Insert, index: slot(index), id: Self::key(&row), value: Some(row), patch: None }
    }

    fn remove(index: usize, id: String) -> Self {
        Self { op: En1998RowOp::Remove, index: slot(index), id, value: None, patch: None }
    }

    fn replace(index: usize, id: String, row: En1998System) -> Self {
        Self { op: En1998RowOp::Replace, index: slot(index), id, value: Some(row), patch: None }
    }

    fn patch(index: usize, id: String, patch: En1998SystemPatch) -> Self {
        Self { op: En1998RowOp::Patch, index: slot(index), id, value: None, patch: Some(patch) }
    }
}

/// 🩹️ Sparse field patch of one `En1998System`: the fields it names replace the row's, its nested edits edit the row's nested collections.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1998SystemPatch {
    pub base_shear_resistance_n: Option<f64>,
}

impl En1998RowPatch<En1998System> for En1998SystemPatch {
    fn apply(&self, row: &mut En1998System) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.base_shear_resistance_n {
            row.base_shear_resistance_n = value.clone();
        }
        Ok(())
    }

    fn inverse(&self, row: &En1998System) -> Self {
        Self {
            base_shear_resistance_n: self.base_shear_resistance_n.as_ref().map(|_| row.base_shear_resistance_n.clone()),
        }
    }

    fn absorb(&mut self, other: Self) {
        if other.base_shear_resistance_n.is_some() {
            self.base_shear_resistance_n = other.base_shear_resistance_n;
        }
    }
}

/// 🧾️ One ordered-row edit of `storeys`.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct En1998StoreyEdit {
    pub op: En1998RowOp,
    pub index: u32,
    pub id: String,
    pub value: Option<En1998Storey>,
    pub patch: Option<En1998StoreyPatch>,
}

impl En1998RowEdit for En1998StoreyEdit {
    type Row = En1998Storey;
    type Patch = En1998StoreyPatch;
    fn key(row: &En1998Storey) -> String {
        row.id.clone()
    }

    fn view(&self) -> En1998RowView<'_, En1998Storey, En1998StoreyPatch> {
        let (index, id) = (self.index as usize, self.id.as_str());
        match (self.op, &self.value, &self.patch) {
            (En1998RowOp::Insert, Some(row), None) => En1998RowView::Insert(index, row),
            (En1998RowOp::Remove, None, None) => En1998RowView::Remove(index, id),
            (En1998RowOp::Replace, Some(row), None) => En1998RowView::Replace(index, id, row),
            (En1998RowOp::Patch, None, Some(patch)) => En1998RowView::Patch(index, id, patch),
            _ => En1998RowView::Malformed(index),
        }
    }

    fn insert(index: usize, row: En1998Storey) -> Self {
        Self { op: En1998RowOp::Insert, index: slot(index), id: Self::key(&row), value: Some(row), patch: None }
    }

    fn remove(index: usize, id: String) -> Self {
        Self { op: En1998RowOp::Remove, index: slot(index), id, value: None, patch: None }
    }

    fn replace(index: usize, id: String, row: En1998Storey) -> Self {
        Self { op: En1998RowOp::Replace, index: slot(index), id, value: Some(row), patch: None }
    }

    fn patch(index: usize, id: String, patch: En1998StoreyPatch) -> Self {
        Self { op: En1998RowOp::Patch, index: slot(index), id, value: None, patch: Some(patch) }
    }
}

/// 🩹️ Sparse field patch of one `En1998Storey`: the fields it names replace the row's, its nested edits edit the row's nested collections.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1998StoreyPatch {
    pub permanent_gk_n: Option<f64>,
    pub stiffness_x: Option<f64>,
    pub drift_x_m: Option<f64>,
}

impl En1998RowPatch<En1998Storey> for En1998StoreyPatch {
    fn apply(&self, row: &mut En1998Storey) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.permanent_gk_n {
            row.permanent_gk_n = value.clone();
        }
        if let Some(value) = &self.stiffness_x {
            row.stiffness_x = value.clone();
        }
        if let Some(value) = &self.drift_x_m {
            row.drift_x_m = value.clone();
        }
        Ok(())
    }

    fn inverse(&self, row: &En1998Storey) -> Self {
        Self {
            permanent_gk_n: self.permanent_gk_n.as_ref().map(|_| row.permanent_gk_n.clone()),
            stiffness_x: self.stiffness_x.as_ref().map(|_| row.stiffness_x.clone()),
            drift_x_m: self.drift_x_m.as_ref().map(|_| row.drift_x_m.clone()),
        }
    }

    fn absorb(&mut self, other: Self) {
        if other.permanent_gk_n.is_some() {
            self.permanent_gk_n = other.permanent_gk_n;
        }
        if other.stiffness_x.is_some() {
            self.stiffness_x = other.stiffness_x;
        }
        if other.drift_x_m.is_some() {
            self.drift_x_m = other.drift_x_m;
        }
    }
}

/// 🧾️ One ordered-row edit of `members`.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct En1998MemberEdit {
    pub op: En1998RowOp,
    pub index: u32,
    pub id: String,
    pub value: Option<En1998Member>,
    pub patch: Option<En1998MemberPatch>,
}

impl En1998RowEdit for En1998MemberEdit {
    type Row = En1998Member;
    type Patch = En1998MemberPatch;
    fn key(row: &En1998Member) -> String {
        row.id.clone()
    }

    fn view(&self) -> En1998RowView<'_, En1998Member, En1998MemberPatch> {
        let (index, id) = (self.index as usize, self.id.as_str());
        match (self.op, &self.value, &self.patch) {
            (En1998RowOp::Insert, Some(row), None) => En1998RowView::Insert(index, row),
            (En1998RowOp::Remove, None, None) => En1998RowView::Remove(index, id),
            (En1998RowOp::Replace, Some(row), None) => En1998RowView::Replace(index, id, row),
            (En1998RowOp::Patch, None, Some(patch)) => En1998RowView::Patch(index, id, patch),
            _ => En1998RowView::Malformed(index),
        }
    }

    fn insert(index: usize, row: En1998Member) -> Self {
        Self { op: En1998RowOp::Insert, index: slot(index), id: Self::key(&row), value: Some(row), patch: None }
    }

    fn remove(index: usize, id: String) -> Self {
        Self { op: En1998RowOp::Remove, index: slot(index), id, value: None, patch: None }
    }

    fn replace(index: usize, id: String, row: En1998Member) -> Self {
        Self { op: En1998RowOp::Replace, index: slot(index), id, value: Some(row), patch: None }
    }

    fn patch(index: usize, id: String, patch: En1998MemberPatch) -> Self {
        Self { op: En1998RowOp::Patch, index: slot(index), id, value: None, patch: Some(patch) }
    }
}

/// 🩹️ Sparse field patch of one `En1998Member`: the fields it names replace the row's, its nested edits edit the row's nested collections.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1998MemberPatch {
    pub detailing_compatible_with_q: Option<bool>,
}

impl En1998RowPatch<En1998Member> for En1998MemberPatch {
    fn apply(&self, row: &mut En1998Member) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.detailing_compatible_with_q {
            row.detailing_compatible_with_q = value.clone();
        }
        Ok(())
    }

    fn inverse(&self, row: &En1998Member) -> Self {
        Self {
            detailing_compatible_with_q: self.detailing_compatible_with_q.as_ref().map(|_| row.detailing_compatible_with_q.clone()),
        }
    }

    fn absorb(&mut self, other: Self) {
        if other.detailing_compatible_with_q.is_some() {
            self.detailing_compatible_with_q = other.detailing_compatible_with_q;
        }
    }
}

impl protocol::MutationDiff<En1998Snapshot> for En1998Diff {
    fn apply(&self, base: &En1998Snapshot, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<En1998Snapshot> {
        let mut next = base.clone();
        if let Some(value) = &self.annex {
            next.annex = value.clone();
        }
        if let Some(value) = &self.site {
            next.site = value.clone();
        }
        apply_rows::<En1998BuildingEdit>(&mut next.buildings, &self.buildings).map_err(|error| error.under(["buildings"]))?;
        apply_rows::<En1998BridgeEdit>(&mut next.bridges, &self.bridges).map_err(|error| error.under(["bridges"]))?;
        apply_rows::<En1998AssessmentEdit>(&mut next.assessments, &self.assessments).map_err(|error| error.under(["assessments"]))?;
        apply_rows::<En1998SiloEdit>(&mut next.silos, &self.silos).map_err(|error| error.under(["silos"]))?;
        apply_rows::<En1998TankEdit>(&mut next.tanks, &self.tanks).map_err(|error| error.under(["tanks"]))?;
        apply_rows::<En1998FoundationEdit>(&mut next.foundations, &self.foundations).map_err(|error| error.under(["foundations"]))?;
        apply_rows::<En1998RetainingWallEdit>(&mut next.retaining_walls, &self.retaining_walls).map_err(|error| error.under(["retainingWalls"]))?;
        apply_rows::<En1998TowerEdit>(&mut next.towers, &self.towers).map_err(|error| error.under(["towers"]))?;
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        if other.annex.is_some() {
            self.annex = other.annex;
        }
        if other.site.is_some() {
            self.site = other.site;
        }
        for edit in other.buildings {
            push_row(&mut self.buildings, edit);
        }
        for edit in other.bridges {
            push_row(&mut self.bridges, edit);
        }
        for edit in other.assessments {
            push_row(&mut self.assessments, edit);
        }
        for edit in other.silos {
            push_row(&mut self.silos, edit);
        }
        for edit in other.tanks {
            push_row(&mut self.tanks, edit);
        }
        for edit in other.foundations {
            push_row(&mut self.foundations, edit);
        }
        for edit in other.retaining_walls {
            push_row(&mut self.retaining_walls, edit);
        }
        for edit in other.towers {
            push_row(&mut self.towers, edit);
        }
    }
}

impl protocol::DiffAlgebra<En1998Snapshot> for En1998Diff {
    fn inverse(&self, base: &En1998Snapshot) -> Self {
        Self {
            annex: self.annex.as_ref().map(|_| base.annex.clone()),
            site: self.site.as_ref().map(|_| base.site.clone()),
            buildings: invert_rows::<En1998BuildingEdit>(&base.buildings, &self.buildings),
            bridges: invert_rows::<En1998BridgeEdit>(&base.bridges, &self.bridges),
            assessments: invert_rows::<En1998AssessmentEdit>(&base.assessments, &self.assessments),
            silos: invert_rows::<En1998SiloEdit>(&base.silos, &self.silos),
            tanks: invert_rows::<En1998TankEdit>(&base.tanks, &self.tanks),
            foundations: invert_rows::<En1998FoundationEdit>(&base.foundations, &self.foundations),
            retaining_walls: invert_rows::<En1998RetainingWallEdit>(&base.retaining_walls, &self.retaining_walls),
            towers: invert_rows::<En1998TowerEdit>(&base.towers, &self.towers),
        }
    }

    fn between(base: &En1998Snapshot, other: &En1998Snapshot) -> Self {
        Self {
            annex: (base.annex != other.annex).then(|| other.annex.clone()),
            site: (base.site != other.site).then(|| other.site.clone()),
            buildings: rows_between::<En1998BuildingEdit>(&base.buildings, &other.buildings),
            bridges: rows_between::<En1998BridgeEdit>(&base.bridges, &other.bridges),
            assessments: rows_between::<En1998AssessmentEdit>(&base.assessments, &other.assessments),
            silos: rows_between::<En1998SiloEdit>(&base.silos, &other.silos),
            tanks: rows_between::<En1998TankEdit>(&base.tanks, &other.tanks),
            foundations: rows_between::<En1998FoundationEdit>(&base.foundations, &other.foundations),
            retaining_walls: rows_between::<En1998RetainingWallEdit>(&base.retaining_walls, &other.retaining_walls),
            towers: rows_between::<En1998TowerEdit>(&base.towers, &other.towers),
        }
    }

    fn is_empty(&self) -> bool {
        self.annex.is_none() && self.site.is_none() && self.buildings.is_empty() && self.bridges.is_empty() && self.assessments.is_empty() && self.silos.is_empty() && self.tanks.is_empty() && self.foundations.is_empty() && self.retaining_walls.is_empty() && self.towers.is_empty()
    }
}
