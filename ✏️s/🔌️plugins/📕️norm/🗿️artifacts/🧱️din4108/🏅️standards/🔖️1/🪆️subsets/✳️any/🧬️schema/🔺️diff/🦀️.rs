//! 🧬️ Din4108 diff schema — sparse scalar fields and ordered per-collection row edits.

use crate::{Din4108Snapshot, EnvelopeElement, LayerDocument, ThermalBridge, ThermalZone, ZoneWindow};
use framework_schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Sparse delta for the Din4108 artifact: changed scalars, and ordered row edits with field patches for zones, elements and thermal bridges.
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.norm.din4108")]
pub struct Din4108Diff {
    #[state(artifact)]
    pub climate_zone: Option<crate::document::ClimateZoneDe>,
    #[state(artifact)]
    pub usage: Option<String>,
    #[state(artifact)]
    pub t_int_c: Option<f64>,
    #[state(artifact)]
    pub rh_int: Option<f64>,
    #[state(artifact)]
    pub has_mechanical_ventilation: Option<bool>,
    #[state(artifact)]
    pub airtightness_n50: Option<f64>,
    #[state(artifact)]
    pub bb2_details_conform: Option<bool>,
    #[state(artifact)]
    pub zones: Vec<Din4108ZoneEdit>,
    #[state(artifact)]
    pub elements: Vec<Din4108ElementEdit>,
    #[state(artifact)]
    pub thermal_bridges: Vec<Din4108ThermalBridgeEdit>,
}
//#endregion 🔖️Diff

//#region 🔖️RowEdits
/// 🧾️ The operation one ordered-row edit performs on its collection.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub enum Din4108RowOp {
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
pub enum Din4108RowView<'a, T, Q> {
    Insert(usize, &'a T),
    Remove(usize, &'a str),
    Replace(usize, &'a str, &'a T),
    Patch(usize, &'a str, &'a Q),
    Malformed(usize),
}

/// 🩹️ A sparse field patch of one row: sets only the fields it names and edits its nested collections.
pub trait Din4108RowPatch<T>: Clone + PartialEq + Sized {
    fn apply(&self, row: &mut T) -> Result<(), protocol::MutationApplyError>;
    fn inverse(&self, row: &T) -> Self;
    fn absorb(&mut self, other: Self);
}

/// 🧷️ One ordered-row edit of a collection: how it reads, and how the engine builds edits of its own kind.
pub trait Din4108RowEdit: Clone + PartialEq + Sized {
    type Row: Clone + PartialEq;
    type Patch: Din4108RowPatch<Self::Row>;
    fn key(row: &Self::Row) -> String;
    fn view(&self) -> Din4108RowView<'_, Self::Row, Self::Patch>;
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

fn guard<E: Din4108RowEdit>(rows: &[E::Row], index: usize, id: &str) -> Result<(), protocol::MutationApplyError> {
    match rows.get(index) {
        Some(row) if E::key(row) == id => Ok(()),
        Some(_) => Err(row_fault("row-key-mismatch", index, id)),
        None => Err(row_fault("row-out-of-range", index, id)),
    }
}

fn apply_rows<E: Din4108RowEdit>(rows: &mut Vec<E::Row>, edits: &[E]) -> Result<(), protocol::MutationApplyError> {
    for edit in edits {
        match edit.view() {
            Din4108RowView::Insert(index, row) if index <= rows.len() => rows.insert(index, row.clone()),
            Din4108RowView::Insert(index, row) => return Err(row_fault("row-out-of-range", index, &E::key(row))),
            Din4108RowView::Remove(index, id) => {
                guard::<E>(rows, index, id)?;
                rows.remove(index);
            }
            Din4108RowView::Replace(index, id, row) => {
                guard::<E>(rows, index, id)?;
                rows[index] = row.clone();
            }
            Din4108RowView::Patch(index, id, patch) => {
                guard::<E>(rows, index, id)?;
                Din4108RowPatch::apply(patch, &mut rows[index]).map_err(|error| error.under([index.to_string()]))?;
            }
            Din4108RowView::Malformed(index) => return Err(row_fault("row-malformed", index, "")),
        }
    }
    Ok(())
}

fn invert_rows<E: Din4108RowEdit>(base: &[E::Row], edits: &[E]) -> Vec<E> {
    let mut rows = base.to_vec();
    let mut inverse = Vec::with_capacity(edits.len());
    for edit in edits {
        let step = match edit.view() {
            Din4108RowView::Insert(index, row) => E::remove(index, E::key(row)),
            Din4108RowView::Remove(index, _) => match rows.get(index) {
                Some(old) => E::insert(index, old.clone()),
                None => break,
            },
            Din4108RowView::Replace(index, _, row) => match rows.get(index) {
                Some(old) => E::replace(index, E::key(row), old.clone()),
                None => break,
            },
            Din4108RowView::Patch(index, id, patch) => match rows.get(index) {
                Some(old) => E::patch(index, id.to_string(), Din4108RowPatch::inverse(patch, old)),
                None => break,
            },
            Din4108RowView::Malformed(_) => break,
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

fn merge_rows<E: Din4108RowEdit>(first: &E, next: &E) -> RowMerge<E> {
    match (first.view(), next.view()) {
        (Din4108RowView::Insert(at, row), Din4108RowView::Remove(to, id)) if at == to && E::key(row) == id => RowMerge::Cancel,
        (Din4108RowView::Insert(at, row), Din4108RowView::Replace(to, id, with)) if at == to && E::key(row) == id => RowMerge::Into(E::insert(at, with.clone())),
        (Din4108RowView::Replace(at, was, row), Din4108RowView::Replace(to, id, with)) if at == to && E::key(row) == id => RowMerge::Into(E::replace(at, was.to_string(), with.clone())),
        (Din4108RowView::Replace(at, was, row), Din4108RowView::Remove(to, id)) if at == to && E::key(row) == id => RowMerge::Into(E::remove(at, was.to_string())),
        (Din4108RowView::Remove(at, was), Din4108RowView::Insert(to, row)) if at == to && E::key(row) == was => RowMerge::Into(E::replace(at, was.to_string(), row.clone())),
        (Din4108RowView::Insert(at, row), Din4108RowView::Patch(to, id, patch)) if at == to && E::key(row) == id => {
            let mut row = row.clone();
            match Din4108RowPatch::apply(patch, &mut row) {
                Ok(()) => RowMerge::Into(E::insert(at, row)),
                Err(_) => RowMerge::Keep,
            }
        }
        (Din4108RowView::Replace(at, was, row), Din4108RowView::Patch(to, id, patch)) if at == to && E::key(row) == id => {
            let mut row = row.clone();
            match Din4108RowPatch::apply(patch, &mut row) {
                Ok(()) => RowMerge::Into(E::replace(at, was.to_string(), row)),
                Err(_) => RowMerge::Keep,
            }
        }
        (Din4108RowView::Patch(at, was, own), Din4108RowView::Patch(to, id, patch)) if at == to && was == id => {
            let mut own = own.clone();
            Din4108RowPatch::absorb(&mut own, patch.clone());
            RowMerge::Into(E::patch(at, was.to_string(), own))
        }
        (Din4108RowView::Patch(at, was, _), Din4108RowView::Remove(to, id)) if at == to && was == id => RowMerge::Into(E::remove(at, was.to_string())),
        (Din4108RowView::Patch(at, was, _), Din4108RowView::Replace(to, id, with)) if at == to && was == id => RowMerge::Into(E::replace(at, was.to_string(), with.clone())),
        _ => RowMerge::Keep,
    }
}

fn push_row<E: Din4108RowEdit>(edits: &mut Vec<E>, next: E) {
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

fn positional_rows<E: Din4108RowEdit>(base: &[E::Row], other: &[E::Row]) -> Vec<E> {
    let shared = base.iter().zip(other).take_while(|(left, right)| left == right).count();
    let removed = (shared..base.len()).rev().map(|index| E::remove(index, E::key(&base[index])));
    let inserted = (shared..other.len()).map(|index| E::insert(index, other[index].clone()));
    removed.chain(inserted).collect()
}

fn rows_between<E: Din4108RowEdit>(base: &[E::Row], other: &[E::Row]) -> Vec<E> {
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

/// 🧾️ One ordered-row edit of `zones`.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Din4108ZoneEdit {
    pub op: Din4108RowOp,
    pub index: u32,
    pub id: String,
    pub value: Option<ThermalZone>,
    pub patch: Option<Din4108ZonePatch>,
}

impl Din4108RowEdit for Din4108ZoneEdit {
    type Row = ThermalZone;
    type Patch = Din4108ZonePatch;
    fn key(row: &ThermalZone) -> String {
        row.id.clone()
    }

    fn view(&self) -> Din4108RowView<'_, ThermalZone, Din4108ZonePatch> {
        let (index, id) = (self.index as usize, self.id.as_str());
        match (self.op, &self.value, &self.patch) {
            (Din4108RowOp::Insert, Some(row), None) => Din4108RowView::Insert(index, row),
            (Din4108RowOp::Remove, None, None) => Din4108RowView::Remove(index, id),
            (Din4108RowOp::Replace, Some(row), None) => Din4108RowView::Replace(index, id, row),
            (Din4108RowOp::Patch, None, Some(patch)) => Din4108RowView::Patch(index, id, patch),
            _ => Din4108RowView::Malformed(index),
        }
    }

    fn insert(index: usize, row: ThermalZone) -> Self {
        Self { op: Din4108RowOp::Insert, index: slot(index), id: Self::key(&row), value: Some(row), patch: None }
    }

    fn remove(index: usize, id: String) -> Self {
        Self { op: Din4108RowOp::Remove, index: slot(index), id, value: None, patch: None }
    }

    fn replace(index: usize, id: String, row: ThermalZone) -> Self {
        Self { op: Din4108RowOp::Replace, index: slot(index), id, value: Some(row), patch: None }
    }

    fn patch(index: usize, id: String, patch: Din4108ZonePatch) -> Self {
        Self { op: Din4108RowOp::Patch, index: slot(index), id, value: None, patch: Some(patch) }
    }
}

/// 🩹️ Sparse field patch of one `ThermalZone`: the fields it names replace the row's, its nested edits edit the row's nested collections.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Din4108ZonePatch {
    pub floor_area_m2: Option<f64>,
    pub heaviness: Option<String>,
    pub night_ventilation: Option<String>,
    pub windows: Vec<Din4108WindowEdit>,
}

impl Din4108RowPatch<ThermalZone> for Din4108ZonePatch {
    fn apply(&self, row: &mut ThermalZone) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.floor_area_m2 {
            row.floor_area_m2 = value.clone();
        }
        if let Some(value) = &self.heaviness {
            row.heaviness = value.clone();
        }
        if let Some(value) = &self.night_ventilation {
            row.night_ventilation = value.clone();
        }
        apply_rows::<Din4108WindowEdit>(&mut row.windows, &self.windows).map_err(|error| error.under(["windows"]))?;
        Ok(())
    }

    fn inverse(&self, row: &ThermalZone) -> Self {
        Self {
            floor_area_m2: self.floor_area_m2.as_ref().map(|_| row.floor_area_m2.clone()),
            heaviness: self.heaviness.as_ref().map(|_| row.heaviness.clone()),
            night_ventilation: self.night_ventilation.as_ref().map(|_| row.night_ventilation.clone()),
            windows: invert_rows::<Din4108WindowEdit>(&row.windows, &self.windows),
        }
    }

    fn absorb(&mut self, other: Self) {
        if other.floor_area_m2.is_some() {
            self.floor_area_m2 = other.floor_area_m2;
        }
        if other.heaviness.is_some() {
            self.heaviness = other.heaviness;
        }
        if other.night_ventilation.is_some() {
            self.night_ventilation = other.night_ventilation;
        }
        for edit in other.windows {
            push_row(&mut self.windows, edit);
        }
    }
}

/// 🧾️ One ordered-row edit of `elements`.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Din4108ElementEdit {
    pub op: Din4108RowOp,
    pub index: u32,
    pub id: String,
    pub value: Option<EnvelopeElement>,
    pub patch: Option<Din4108ElementPatch>,
}

impl Din4108RowEdit for Din4108ElementEdit {
    type Row = EnvelopeElement;
    type Patch = Din4108ElementPatch;
    fn key(row: &EnvelopeElement) -> String {
        row.id.clone()
    }

    fn view(&self) -> Din4108RowView<'_, EnvelopeElement, Din4108ElementPatch> {
        let (index, id) = (self.index as usize, self.id.as_str());
        match (self.op, &self.value, &self.patch) {
            (Din4108RowOp::Insert, Some(row), None) => Din4108RowView::Insert(index, row),
            (Din4108RowOp::Remove, None, None) => Din4108RowView::Remove(index, id),
            (Din4108RowOp::Replace, Some(row), None) => Din4108RowView::Replace(index, id, row),
            (Din4108RowOp::Patch, None, Some(patch)) => Din4108RowView::Patch(index, id, patch),
            _ => Din4108RowView::Malformed(index),
        }
    }

    fn insert(index: usize, row: EnvelopeElement) -> Self {
        Self { op: Din4108RowOp::Insert, index: slot(index), id: Self::key(&row), value: Some(row), patch: None }
    }

    fn remove(index: usize, id: String) -> Self {
        Self { op: Din4108RowOp::Remove, index: slot(index), id, value: None, patch: None }
    }

    fn replace(index: usize, id: String, row: EnvelopeElement) -> Self {
        Self { op: Din4108RowOp::Replace, index: slot(index), id, value: Some(row), patch: None }
    }

    fn patch(index: usize, id: String, patch: Din4108ElementPatch) -> Self {
        Self { op: Din4108RowOp::Patch, index: slot(index), id, value: None, patch: Some(patch) }
    }
}

/// 🩹️ Sparse field patch of one `EnvelopeElement`: the fields it names replace the row's, its nested edits edit the row's nested collections.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Din4108ElementPatch {
    pub kind: Option<String>,
    pub orientation_deg: Option<f64>,
    pub inclination_deg: Option<f64>,
    pub adjacent: Option<String>,
    pub area_m2: Option<f64>,
    pub delta_u_g: Option<f64>,
    pub delta_u_f: Option<f64>,
    pub delta_u_r: Option<f64>,
    pub layers: Vec<Din4108LayerEdit>,
}

impl Din4108RowPatch<EnvelopeElement> for Din4108ElementPatch {
    fn apply(&self, row: &mut EnvelopeElement) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.kind {
            row.kind = value.clone();
        }
        if let Some(value) = &self.orientation_deg {
            row.orientation_deg = value.clone();
        }
        if let Some(value) = &self.inclination_deg {
            row.inclination_deg = value.clone();
        }
        if let Some(value) = &self.adjacent {
            row.adjacent = value.clone();
        }
        if let Some(value) = &self.area_m2 {
            row.area_m2 = value.clone();
        }
        if let Some(value) = &self.delta_u_g {
            row.delta_u_g = value.clone();
        }
        if let Some(value) = &self.delta_u_f {
            row.delta_u_f = value.clone();
        }
        if let Some(value) = &self.delta_u_r {
            row.delta_u_r = value.clone();
        }
        apply_rows::<Din4108LayerEdit>(&mut row.layers, &self.layers).map_err(|error| error.under(["layers"]))?;
        Ok(())
    }

    fn inverse(&self, row: &EnvelopeElement) -> Self {
        Self {
            kind: self.kind.as_ref().map(|_| row.kind.clone()),
            orientation_deg: self.orientation_deg.as_ref().map(|_| row.orientation_deg.clone()),
            inclination_deg: self.inclination_deg.as_ref().map(|_| row.inclination_deg.clone()),
            adjacent: self.adjacent.as_ref().map(|_| row.adjacent.clone()),
            area_m2: self.area_m2.as_ref().map(|_| row.area_m2.clone()),
            delta_u_g: self.delta_u_g.as_ref().map(|_| row.delta_u_g.clone()),
            delta_u_f: self.delta_u_f.as_ref().map(|_| row.delta_u_f.clone()),
            delta_u_r: self.delta_u_r.as_ref().map(|_| row.delta_u_r.clone()),
            layers: invert_rows::<Din4108LayerEdit>(&row.layers, &self.layers),
        }
    }

    fn absorb(&mut self, other: Self) {
        if other.kind.is_some() {
            self.kind = other.kind;
        }
        if other.orientation_deg.is_some() {
            self.orientation_deg = other.orientation_deg;
        }
        if other.inclination_deg.is_some() {
            self.inclination_deg = other.inclination_deg;
        }
        if other.adjacent.is_some() {
            self.adjacent = other.adjacent;
        }
        if other.area_m2.is_some() {
            self.area_m2 = other.area_m2;
        }
        if other.delta_u_g.is_some() {
            self.delta_u_g = other.delta_u_g;
        }
        if other.delta_u_f.is_some() {
            self.delta_u_f = other.delta_u_f;
        }
        if other.delta_u_r.is_some() {
            self.delta_u_r = other.delta_u_r;
        }
        for edit in other.layers {
            push_row(&mut self.layers, edit);
        }
    }
}

/// 🧾️ One ordered-row edit of `thermal_bridges`.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Din4108ThermalBridgeEdit {
    pub op: Din4108RowOp,
    pub index: u32,
    pub id: String,
    pub value: Option<ThermalBridge>,
    pub patch: Option<Din4108ThermalBridgePatch>,
}

impl Din4108RowEdit for Din4108ThermalBridgeEdit {
    type Row = ThermalBridge;
    type Patch = Din4108ThermalBridgePatch;
    fn key(row: &ThermalBridge) -> String {
        row.id.clone()
    }

    fn view(&self) -> Din4108RowView<'_, ThermalBridge, Din4108ThermalBridgePatch> {
        let (index, id) = (self.index as usize, self.id.as_str());
        match (self.op, &self.value, &self.patch) {
            (Din4108RowOp::Insert, Some(row), None) => Din4108RowView::Insert(index, row),
            (Din4108RowOp::Remove, None, None) => Din4108RowView::Remove(index, id),
            (Din4108RowOp::Replace, Some(row), None) => Din4108RowView::Replace(index, id, row),
            (Din4108RowOp::Patch, None, Some(patch)) => Din4108RowView::Patch(index, id, patch),
            _ => Din4108RowView::Malformed(index),
        }
    }

    fn insert(index: usize, row: ThermalBridge) -> Self {
        Self { op: Din4108RowOp::Insert, index: slot(index), id: Self::key(&row), value: Some(row), patch: None }
    }

    fn remove(index: usize, id: String) -> Self {
        Self { op: Din4108RowOp::Remove, index: slot(index), id, value: None, patch: None }
    }

    fn replace(index: usize, id: String, row: ThermalBridge) -> Self {
        Self { op: Din4108RowOp::Replace, index: slot(index), id, value: Some(row), patch: None }
    }

    fn patch(index: usize, id: String, patch: Din4108ThermalBridgePatch) -> Self {
        Self { op: Din4108RowOp::Patch, index: slot(index), id, value: None, patch: Some(patch) }
    }
}

/// 🩹️ Sparse field patch of one `ThermalBridge`: the fields it names replace the row's, its nested edits edit the row's nested collections.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Din4108ThermalBridgePatch {
    pub psi: Option<f64>,
    pub length_m: Option<f64>,
    pub bb2_type: Option<String>,
}

impl Din4108RowPatch<ThermalBridge> for Din4108ThermalBridgePatch {
    fn apply(&self, row: &mut ThermalBridge) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.psi {
            row.psi = value.clone();
        }
        if let Some(value) = &self.length_m {
            row.length_m = value.clone();
        }
        if let Some(value) = &self.bb2_type {
            row.bb2_type = value.clone();
        }
        Ok(())
    }

    fn inverse(&self, row: &ThermalBridge) -> Self {
        Self {
            psi: self.psi.as_ref().map(|_| row.psi.clone()),
            length_m: self.length_m.as_ref().map(|_| row.length_m.clone()),
            bb2_type: self.bb2_type.as_ref().map(|_| row.bb2_type.clone()),
        }
    }

    fn absorb(&mut self, other: Self) {
        if other.psi.is_some() {
            self.psi = other.psi;
        }
        if other.length_m.is_some() {
            self.length_m = other.length_m;
        }
        if other.bb2_type.is_some() {
            self.bb2_type = other.bb2_type;
        }
    }
}

/// 🧾️ One ordered-row edit of `windows`.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Din4108WindowEdit {
    pub op: Din4108RowOp,
    pub index: u32,
    pub id: String,
    pub value: Option<ZoneWindow>,
    pub patch: Option<Din4108WindowPatch>,
}

impl Din4108RowEdit for Din4108WindowEdit {
    type Row = ZoneWindow;
    type Patch = Din4108WindowPatch;
    fn key(row: &ZoneWindow) -> String {
        row.id.clone()
    }

    fn view(&self) -> Din4108RowView<'_, ZoneWindow, Din4108WindowPatch> {
        let (index, id) = (self.index as usize, self.id.as_str());
        match (self.op, &self.value, &self.patch) {
            (Din4108RowOp::Insert, Some(row), None) => Din4108RowView::Insert(index, row),
            (Din4108RowOp::Remove, None, None) => Din4108RowView::Remove(index, id),
            (Din4108RowOp::Replace, Some(row), None) => Din4108RowView::Replace(index, id, row),
            (Din4108RowOp::Patch, None, Some(patch)) => Din4108RowView::Patch(index, id, patch),
            _ => Din4108RowView::Malformed(index),
        }
    }

    fn insert(index: usize, row: ZoneWindow) -> Self {
        Self { op: Din4108RowOp::Insert, index: slot(index), id: Self::key(&row), value: Some(row), patch: None }
    }

    fn remove(index: usize, id: String) -> Self {
        Self { op: Din4108RowOp::Remove, index: slot(index), id, value: None, patch: None }
    }

    fn replace(index: usize, id: String, row: ZoneWindow) -> Self {
        Self { op: Din4108RowOp::Replace, index: slot(index), id, value: Some(row), patch: None }
    }

    fn patch(index: usize, id: String, patch: Din4108WindowPatch) -> Self {
        Self { op: Din4108RowOp::Patch, index: slot(index), id, value: None, patch: Some(patch) }
    }
}

/// 🩹️ Sparse field patch of one `ZoneWindow`: the fields it names replace the row's, its nested edits edit the row's nested collections.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Din4108WindowPatch {
    pub orientation: Option<String>,
    pub inclination_deg: Option<f64>,
    pub area_m2: Option<f64>,
    pub g_value: Option<f64>,
    pub shading_fc: Option<f64>,
}

impl Din4108RowPatch<ZoneWindow> for Din4108WindowPatch {
    fn apply(&self, row: &mut ZoneWindow) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.orientation {
            row.orientation = value.clone();
        }
        if let Some(value) = &self.inclination_deg {
            row.inclination_deg = value.clone();
        }
        if let Some(value) = &self.area_m2 {
            row.area_m2 = value.clone();
        }
        if let Some(value) = &self.g_value {
            row.g_value = value.clone();
        }
        if let Some(value) = &self.shading_fc {
            row.shading_fc = value.clone();
        }
        Ok(())
    }

    fn inverse(&self, row: &ZoneWindow) -> Self {
        Self {
            orientation: self.orientation.as_ref().map(|_| row.orientation.clone()),
            inclination_deg: self.inclination_deg.as_ref().map(|_| row.inclination_deg.clone()),
            area_m2: self.area_m2.as_ref().map(|_| row.area_m2.clone()),
            g_value: self.g_value.as_ref().map(|_| row.g_value.clone()),
            shading_fc: self.shading_fc.as_ref().map(|_| row.shading_fc.clone()),
        }
    }

    fn absorb(&mut self, other: Self) {
        if other.orientation.is_some() {
            self.orientation = other.orientation;
        }
        if other.inclination_deg.is_some() {
            self.inclination_deg = other.inclination_deg;
        }
        if other.area_m2.is_some() {
            self.area_m2 = other.area_m2;
        }
        if other.g_value.is_some() {
            self.g_value = other.g_value;
        }
        if other.shading_fc.is_some() {
            self.shading_fc = other.shading_fc;
        }
    }
}

/// 🧾️ One ordered-row edit of `layers`.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Din4108LayerEdit {
    pub op: Din4108RowOp,
    pub index: u32,
    pub id: String,
    pub value: Option<LayerDocument>,
    pub patch: Option<Din4108LayerPatch>,
}

impl Din4108RowEdit for Din4108LayerEdit {
    type Row = LayerDocument;
    type Patch = Din4108LayerPatch;
    fn key(row: &LayerDocument) -> String {
        row.id.clone()
    }

    fn view(&self) -> Din4108RowView<'_, LayerDocument, Din4108LayerPatch> {
        let (index, id) = (self.index as usize, self.id.as_str());
        match (self.op, &self.value, &self.patch) {
            (Din4108RowOp::Insert, Some(row), None) => Din4108RowView::Insert(index, row),
            (Din4108RowOp::Remove, None, None) => Din4108RowView::Remove(index, id),
            (Din4108RowOp::Replace, Some(row), None) => Din4108RowView::Replace(index, id, row),
            (Din4108RowOp::Patch, None, Some(patch)) => Din4108RowView::Patch(index, id, patch),
            _ => Din4108RowView::Malformed(index),
        }
    }

    fn insert(index: usize, row: LayerDocument) -> Self {
        Self { op: Din4108RowOp::Insert, index: slot(index), id: Self::key(&row), value: Some(row), patch: None }
    }

    fn remove(index: usize, id: String) -> Self {
        Self { op: Din4108RowOp::Remove, index: slot(index), id, value: None, patch: None }
    }

    fn replace(index: usize, id: String, row: LayerDocument) -> Self {
        Self { op: Din4108RowOp::Replace, index: slot(index), id, value: Some(row), patch: None }
    }

    fn patch(index: usize, id: String, patch: Din4108LayerPatch) -> Self {
        Self { op: Din4108RowOp::Patch, index: slot(index), id, value: None, patch: Some(patch) }
    }
}

/// 🩹️ Sparse field patch of one `LayerDocument`: the fields it names replace the row's, its nested edits edit the row's nested collections.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Din4108LayerPatch {
    pub material_id: Option<String>,
    pub thickness_m: Option<f64>,
    pub lambda: Option<f64>,
    pub mu: Option<f64>,
    pub application_type: Option<String>,
    pub compressive_class: Option<String>,
}

impl Din4108RowPatch<LayerDocument> for Din4108LayerPatch {
    fn apply(&self, row: &mut LayerDocument) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.material_id {
            row.material_id = value.clone();
        }
        if let Some(value) = &self.thickness_m {
            row.thickness_m = value.clone();
        }
        if let Some(value) = &self.lambda {
            row.lambda = value.clone();
        }
        if let Some(value) = &self.mu {
            row.mu = value.clone();
        }
        if let Some(value) = &self.application_type {
            row.application_type = value.clone();
        }
        if let Some(value) = &self.compressive_class {
            row.compressive_class = value.clone();
        }
        Ok(())
    }

    fn inverse(&self, row: &LayerDocument) -> Self {
        Self {
            material_id: self.material_id.as_ref().map(|_| row.material_id.clone()),
            thickness_m: self.thickness_m.as_ref().map(|_| row.thickness_m.clone()),
            lambda: self.lambda.as_ref().map(|_| row.lambda.clone()),
            mu: self.mu.as_ref().map(|_| row.mu.clone()),
            application_type: self.application_type.as_ref().map(|_| row.application_type.clone()),
            compressive_class: self.compressive_class.as_ref().map(|_| row.compressive_class.clone()),
        }
    }

    fn absorb(&mut self, other: Self) {
        if other.material_id.is_some() {
            self.material_id = other.material_id;
        }
        if other.thickness_m.is_some() {
            self.thickness_m = other.thickness_m;
        }
        if other.lambda.is_some() {
            self.lambda = other.lambda;
        }
        if other.mu.is_some() {
            self.mu = other.mu;
        }
        if other.application_type.is_some() {
            self.application_type = other.application_type;
        }
        if other.compressive_class.is_some() {
            self.compressive_class = other.compressive_class;
        }
    }
}

impl protocol::MutationDiff<Din4108Snapshot> for Din4108Diff {
    fn apply(&self, base: &Din4108Snapshot, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<Din4108Snapshot> {
        let mut next = base.clone();
        if let Some(value) = &self.climate_zone {
            next.climate_zone = value.clone();
        }
        if let Some(value) = &self.usage {
            next.usage = value.clone();
        }
        if let Some(value) = &self.t_int_c {
            next.t_int_c = value.clone();
        }
        if let Some(value) = &self.rh_int {
            next.rh_int = value.clone();
        }
        if let Some(value) = &self.has_mechanical_ventilation {
            next.has_mechanical_ventilation = value.clone();
        }
        if let Some(value) = &self.airtightness_n50 {
            next.airtightness_n50 = value.clone();
        }
        if let Some(value) = &self.bb2_details_conform {
            next.bb2_details_conform = value.clone();
        }
        apply_rows::<Din4108ZoneEdit>(&mut next.zones, &self.zones).map_err(|error| error.under(["zones"]))?;
        apply_rows::<Din4108ElementEdit>(&mut next.elements, &self.elements).map_err(|error| error.under(["elements"]))?;
        apply_rows::<Din4108ThermalBridgeEdit>(&mut next.thermal_bridges, &self.thermal_bridges).map_err(|error| error.under(["thermalBridges"]))?;
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        if other.climate_zone.is_some() {
            self.climate_zone = other.climate_zone;
        }
        if other.usage.is_some() {
            self.usage = other.usage;
        }
        if other.t_int_c.is_some() {
            self.t_int_c = other.t_int_c;
        }
        if other.rh_int.is_some() {
            self.rh_int = other.rh_int;
        }
        if other.has_mechanical_ventilation.is_some() {
            self.has_mechanical_ventilation = other.has_mechanical_ventilation;
        }
        if other.airtightness_n50.is_some() {
            self.airtightness_n50 = other.airtightness_n50;
        }
        if other.bb2_details_conform.is_some() {
            self.bb2_details_conform = other.bb2_details_conform;
        }
        for edit in other.zones {
            push_row(&mut self.zones, edit);
        }
        for edit in other.elements {
            push_row(&mut self.elements, edit);
        }
        for edit in other.thermal_bridges {
            push_row(&mut self.thermal_bridges, edit);
        }
    }
}

impl protocol::DiffAlgebra<Din4108Snapshot> for Din4108Diff {
    fn inverse(&self, base: &Din4108Snapshot) -> Self {
        Self {
            climate_zone: self.climate_zone.as_ref().map(|_| base.climate_zone.clone()),
            usage: self.usage.as_ref().map(|_| base.usage.clone()),
            t_int_c: self.t_int_c.as_ref().map(|_| base.t_int_c.clone()),
            rh_int: self.rh_int.as_ref().map(|_| base.rh_int.clone()),
            has_mechanical_ventilation: self.has_mechanical_ventilation.as_ref().map(|_| base.has_mechanical_ventilation.clone()),
            airtightness_n50: self.airtightness_n50.as_ref().map(|_| base.airtightness_n50.clone()),
            bb2_details_conform: self.bb2_details_conform.as_ref().map(|_| base.bb2_details_conform.clone()),
            zones: invert_rows::<Din4108ZoneEdit>(&base.zones, &self.zones),
            elements: invert_rows::<Din4108ElementEdit>(&base.elements, &self.elements),
            thermal_bridges: invert_rows::<Din4108ThermalBridgeEdit>(&base.thermal_bridges, &self.thermal_bridges),
        }
    }

    fn between(base: &Din4108Snapshot, other: &Din4108Snapshot) -> Self {
        Self {
            climate_zone: (base.climate_zone != other.climate_zone).then(|| other.climate_zone.clone()),
            usage: (base.usage != other.usage).then(|| other.usage.clone()),
            t_int_c: (base.t_int_c != other.t_int_c).then(|| other.t_int_c.clone()),
            rh_int: (base.rh_int != other.rh_int).then(|| other.rh_int.clone()),
            has_mechanical_ventilation: (base.has_mechanical_ventilation != other.has_mechanical_ventilation).then(|| other.has_mechanical_ventilation.clone()),
            airtightness_n50: (base.airtightness_n50 != other.airtightness_n50).then(|| other.airtightness_n50.clone()),
            bb2_details_conform: (base.bb2_details_conform != other.bb2_details_conform).then(|| other.bb2_details_conform.clone()),
            zones: rows_between::<Din4108ZoneEdit>(&base.zones, &other.zones),
            elements: rows_between::<Din4108ElementEdit>(&base.elements, &other.elements),
            thermal_bridges: rows_between::<Din4108ThermalBridgeEdit>(&base.thermal_bridges, &other.thermal_bridges),
        }
    }

    fn is_empty(&self) -> bool {
        self.climate_zone.is_none() && self.usage.is_none() && self.t_int_c.is_none() && self.rh_int.is_none() && self.has_mechanical_ventilation.is_none() && self.airtightness_n50.is_none() && self.bb2_details_conform.is_none() && self.zones.is_empty() && self.elements.is_empty() && self.thermal_bridges.is_empty()
    }
}
