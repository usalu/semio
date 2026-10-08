//! 🧬️ Din16798 diff schema — sparse scalar fields and ordered per-collection row edits.

use crate::{Din16798Snapshot, VentSystemDocument, ZoneDocument};
use framework_schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Sparse delta for the Din16798 artifact: changed scalars, and ordered row edits with field patches for zones and ventilation systems.
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.norm.din16798")]
pub struct Din16798Diff {
    #[state(artifact)]
    pub annex: Option<crate::document::AnnexChoice>,
    #[state(artifact)]
    pub theta_rm_c: Option<f64>,
    #[state(artifact)]
    pub outdoor_co2_ppm: Option<f64>,
    #[state(artifact)]
    pub envelope_n50_h_inv: Option<f64>,
    #[state(artifact)]
    pub envelope_volume_m3: Option<f64>,
    #[state(artifact)]
    pub cellar_area_m2: Option<f64>,
    #[state(artifact)]
    pub cellar_ventilation_m3_h: Option<f64>,
    #[state(artifact)]
    pub night_setback_k: Option<f64>,
    #[state(artifact)]
    pub zones: Vec<Din16798ZoneEdit>,
    #[state(artifact)]
    pub vent_systems: Vec<Din16798VentSystemEdit>,
}
//#endregion 🔖️Diff

//#region 🔖️RowEdits
/// 🧾️ The operation one ordered-row edit performs on its collection.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub enum Din16798RowOp {
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
pub enum Din16798RowView<'a, T, Q> {
    Insert(usize, &'a T),
    Remove(usize, &'a str),
    Replace(usize, &'a str, &'a T),
    Patch(usize, &'a str, &'a Q),
    Malformed(usize),
}

/// 🩹️ A sparse field patch of one row: sets only the fields it names and edits its nested collections.
pub trait Din16798RowPatch<T>: Clone + PartialEq + Sized {
    fn apply(&self, row: &mut T) -> Result<(), protocol::MutationApplyError>;
    fn inverse(&self, row: &T) -> Self;
    fn absorb(&mut self, other: Self);
}

/// 🧷️ One ordered-row edit of a collection: how it reads, and how the engine builds edits of its own kind.
pub trait Din16798RowEdit: Clone + PartialEq + Sized {
    type Row: Clone + PartialEq;
    type Patch: Din16798RowPatch<Self::Row>;
    fn key(row: &Self::Row) -> String;
    fn view(&self) -> Din16798RowView<'_, Self::Row, Self::Patch>;
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

fn guard<E: Din16798RowEdit>(rows: &[E::Row], index: usize, id: &str) -> Result<(), protocol::MutationApplyError> {
    match rows.get(index) {
        Some(row) if E::key(row) == id => Ok(()),
        Some(_) => Err(row_fault("row-key-mismatch", index, id)),
        None => Err(row_fault("row-out-of-range", index, id)),
    }
}

fn apply_rows<E: Din16798RowEdit>(rows: &mut Vec<E::Row>, edits: &[E]) -> Result<(), protocol::MutationApplyError> {
    for edit in edits {
        match edit.view() {
            Din16798RowView::Insert(index, row) if index <= rows.len() => rows.insert(index, row.clone()),
            Din16798RowView::Insert(index, row) => return Err(row_fault("row-out-of-range", index, &E::key(row))),
            Din16798RowView::Remove(index, id) => {
                guard::<E>(rows, index, id)?;
                rows.remove(index);
            }
            Din16798RowView::Replace(index, id, row) => {
                guard::<E>(rows, index, id)?;
                rows[index] = row.clone();
            }
            Din16798RowView::Patch(index, id, patch) => {
                guard::<E>(rows, index, id)?;
                Din16798RowPatch::apply(patch, &mut rows[index]).map_err(|error| error.under([index.to_string()]))?;
            }
            Din16798RowView::Malformed(index) => return Err(row_fault("row-malformed", index, "")),
        }
    }
    Ok(())
}

fn invert_rows<E: Din16798RowEdit>(base: &[E::Row], edits: &[E]) -> Vec<E> {
    let mut rows = base.to_vec();
    let mut inverse = Vec::with_capacity(edits.len());
    for edit in edits {
        let step = match edit.view() {
            Din16798RowView::Insert(index, row) => E::remove(index, E::key(row)),
            Din16798RowView::Remove(index, _) => match rows.get(index) {
                Some(old) => E::insert(index, old.clone()),
                None => break,
            },
            Din16798RowView::Replace(index, _, row) => match rows.get(index) {
                Some(old) => E::replace(index, E::key(row), old.clone()),
                None => break,
            },
            Din16798RowView::Patch(index, id, patch) => match rows.get(index) {
                Some(old) => E::patch(index, id.to_string(), Din16798RowPatch::inverse(patch, old)),
                None => break,
            },
            Din16798RowView::Malformed(_) => break,
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

fn merge_rows<E: Din16798RowEdit>(first: &E, next: &E) -> RowMerge<E> {
    match (first.view(), next.view()) {
        (Din16798RowView::Insert(at, row), Din16798RowView::Remove(to, id)) if at == to && E::key(row) == id => RowMerge::Cancel,
        (Din16798RowView::Insert(at, row), Din16798RowView::Replace(to, id, with)) if at == to && E::key(row) == id => RowMerge::Into(E::insert(at, with.clone())),
        (Din16798RowView::Replace(at, was, row), Din16798RowView::Replace(to, id, with)) if at == to && E::key(row) == id => RowMerge::Into(E::replace(at, was.to_string(), with.clone())),
        (Din16798RowView::Replace(at, was, row), Din16798RowView::Remove(to, id)) if at == to && E::key(row) == id => RowMerge::Into(E::remove(at, was.to_string())),
        (Din16798RowView::Remove(at, was), Din16798RowView::Insert(to, row)) if at == to && E::key(row) == was => RowMerge::Into(E::replace(at, was.to_string(), row.clone())),
        (Din16798RowView::Insert(at, row), Din16798RowView::Patch(to, id, patch)) if at == to && E::key(row) == id => {
            let mut row = row.clone();
            match Din16798RowPatch::apply(patch, &mut row) {
                Ok(()) => RowMerge::Into(E::insert(at, row)),
                Err(_) => RowMerge::Keep,
            }
        }
        (Din16798RowView::Replace(at, was, row), Din16798RowView::Patch(to, id, patch)) if at == to && E::key(row) == id => {
            let mut row = row.clone();
            match Din16798RowPatch::apply(patch, &mut row) {
                Ok(()) => RowMerge::Into(E::replace(at, was.to_string(), row)),
                Err(_) => RowMerge::Keep,
            }
        }
        (Din16798RowView::Patch(at, was, own), Din16798RowView::Patch(to, id, patch)) if at == to && was == id => {
            let mut own = own.clone();
            Din16798RowPatch::absorb(&mut own, patch.clone());
            RowMerge::Into(E::patch(at, was.to_string(), own))
        }
        (Din16798RowView::Patch(at, was, _), Din16798RowView::Remove(to, id)) if at == to && was == id => RowMerge::Into(E::remove(at, was.to_string())),
        (Din16798RowView::Patch(at, was, _), Din16798RowView::Replace(to, id, with)) if at == to && was == id => RowMerge::Into(E::replace(at, was.to_string(), with.clone())),
        _ => RowMerge::Keep,
    }
}

fn push_row<E: Din16798RowEdit>(edits: &mut Vec<E>, next: E) {
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

fn positional_rows<E: Din16798RowEdit>(base: &[E::Row], other: &[E::Row]) -> Vec<E> {
    let shared = base.iter().zip(other).take_while(|(left, right)| left == right).count();
    let removed = (shared..base.len()).rev().map(|index| E::remove(index, E::key(&base[index])));
    let inserted = (shared..other.len()).map(|index| E::insert(index, other[index].clone()));
    removed.chain(inserted).collect()
}

fn rows_between<E: Din16798RowEdit>(base: &[E::Row], other: &[E::Row]) -> Vec<E> {
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
pub struct Din16798ZoneEdit {
    pub op: Din16798RowOp,
    pub index: u32,
    pub id: String,
    pub value: Option<ZoneDocument>,
    pub patch: Option<Din16798ZonePatch>,
}

impl Din16798RowEdit for Din16798ZoneEdit {
    type Row = ZoneDocument;
    type Patch = Din16798ZonePatch;
    fn key(row: &ZoneDocument) -> String {
        row.id.clone()
    }

    fn view(&self) -> Din16798RowView<'_, ZoneDocument, Din16798ZonePatch> {
        let (index, id) = (self.index as usize, self.id.as_str());
        match (self.op, &self.value, &self.patch) {
            (Din16798RowOp::Insert, Some(row), None) => Din16798RowView::Insert(index, row),
            (Din16798RowOp::Remove, None, None) => Din16798RowView::Remove(index, id),
            (Din16798RowOp::Replace, Some(row), None) => Din16798RowView::Replace(index, id, row),
            (Din16798RowOp::Patch, None, Some(patch)) => Din16798RowView::Patch(index, id, patch),
            _ => Din16798RowView::Malformed(index),
        }
    }

    fn insert(index: usize, row: ZoneDocument) -> Self {
        Self { op: Din16798RowOp::Insert, index: slot(index), id: Self::key(&row), value: Some(row), patch: None }
    }

    fn remove(index: usize, id: String) -> Self {
        Self { op: Din16798RowOp::Remove, index: slot(index), id, value: None, patch: None }
    }

    fn replace(index: usize, id: String, row: ZoneDocument) -> Self {
        Self { op: Din16798RowOp::Replace, index: slot(index), id, value: Some(row), patch: None }
    }

    fn patch(index: usize, id: String, patch: Din16798ZonePatch) -> Self {
        Self { op: Din16798RowOp::Patch, index: slot(index), id, value: None, patch: Some(patch) }
    }
}

/// 🩹️ Sparse field patch of one `ZoneDocument`: the fields it names replace the row's, its nested edits edit the row's nested collections.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Din16798ZonePatch {
    pub usage_type: Option<String>,
    pub floor_area_m2: Option<f64>,
    pub occupants: Option<u32>,
    pub comfort_category: Option<String>,
    pub pollution_class: Option<String>,
    pub comfort_model: Option<String>,
    pub t_op_winter_c: Option<f64>,
    pub t_op_summer_c: Option<f64>,
    pub air_speed_m_s: Option<f64>,
    pub clothing_clo: Option<f64>,
    pub metabolic_rate_met: Option<f64>,
    pub rh_percent: Option<f64>,
    pub outdoor_air_supplied_m3_h: Option<f64>,
    pub co2_ppm: Option<f64>,
    pub illuminance_lx: Option<f64>,
    pub noise_db: Option<f64>,
    pub turbulence_intensity_percent: Option<f64>,
    pub vent_method: Option<String>,
    pub vent_system_id: Option<String>,
}

impl Din16798RowPatch<ZoneDocument> for Din16798ZonePatch {
    fn apply(&self, row: &mut ZoneDocument) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.usage_type {
            row.usage_type = value.clone();
        }
        if let Some(value) = &self.floor_area_m2 {
            row.floor_area_m2 = value.clone();
        }
        if let Some(value) = &self.occupants {
            row.occupants = value.clone();
        }
        if let Some(value) = &self.comfort_category {
            row.comfort_category = value.clone();
        }
        if let Some(value) = &self.pollution_class {
            row.pollution_class = value.clone();
        }
        if let Some(value) = &self.comfort_model {
            row.comfort_model = value.clone();
        }
        if let Some(value) = &self.t_op_winter_c {
            row.t_op_winter_c = value.clone();
        }
        if let Some(value) = &self.t_op_summer_c {
            row.t_op_summer_c = value.clone();
        }
        if let Some(value) = &self.air_speed_m_s {
            row.air_speed_m_s = value.clone();
        }
        if let Some(value) = &self.clothing_clo {
            row.clothing_clo = value.clone();
        }
        if let Some(value) = &self.metabolic_rate_met {
            row.metabolic_rate_met = value.clone();
        }
        if let Some(value) = &self.rh_percent {
            row.rh_percent = value.clone();
        }
        if let Some(value) = &self.outdoor_air_supplied_m3_h {
            row.outdoor_air_supplied_m3_h = value.clone();
        }
        if let Some(value) = &self.co2_ppm {
            row.co2_ppm = value.clone();
        }
        if let Some(value) = &self.illuminance_lx {
            row.illuminance_lx = value.clone();
        }
        if let Some(value) = &self.noise_db {
            row.noise_db = value.clone();
        }
        if let Some(value) = &self.turbulence_intensity_percent {
            row.turbulence_intensity_percent = value.clone();
        }
        if let Some(value) = &self.vent_method {
            row.vent_method = value.clone();
        }
        if let Some(value) = &self.vent_system_id {
            row.vent_system_id = value.clone();
        }
        Ok(())
    }

    fn inverse(&self, row: &ZoneDocument) -> Self {
        Self {
            usage_type: self.usage_type.as_ref().map(|_| row.usage_type.clone()),
            floor_area_m2: self.floor_area_m2.as_ref().map(|_| row.floor_area_m2.clone()),
            occupants: self.occupants.as_ref().map(|_| row.occupants.clone()),
            comfort_category: self.comfort_category.as_ref().map(|_| row.comfort_category.clone()),
            pollution_class: self.pollution_class.as_ref().map(|_| row.pollution_class.clone()),
            comfort_model: self.comfort_model.as_ref().map(|_| row.comfort_model.clone()),
            t_op_winter_c: self.t_op_winter_c.as_ref().map(|_| row.t_op_winter_c.clone()),
            t_op_summer_c: self.t_op_summer_c.as_ref().map(|_| row.t_op_summer_c.clone()),
            air_speed_m_s: self.air_speed_m_s.as_ref().map(|_| row.air_speed_m_s.clone()),
            clothing_clo: self.clothing_clo.as_ref().map(|_| row.clothing_clo.clone()),
            metabolic_rate_met: self.metabolic_rate_met.as_ref().map(|_| row.metabolic_rate_met.clone()),
            rh_percent: self.rh_percent.as_ref().map(|_| row.rh_percent.clone()),
            outdoor_air_supplied_m3_h: self.outdoor_air_supplied_m3_h.as_ref().map(|_| row.outdoor_air_supplied_m3_h.clone()),
            co2_ppm: self.co2_ppm.as_ref().map(|_| row.co2_ppm.clone()),
            illuminance_lx: self.illuminance_lx.as_ref().map(|_| row.illuminance_lx.clone()),
            noise_db: self.noise_db.as_ref().map(|_| row.noise_db.clone()),
            turbulence_intensity_percent: self.turbulence_intensity_percent.as_ref().map(|_| row.turbulence_intensity_percent.clone()),
            vent_method: self.vent_method.as_ref().map(|_| row.vent_method.clone()),
            vent_system_id: self.vent_system_id.as_ref().map(|_| row.vent_system_id.clone()),
        }
    }

    fn absorb(&mut self, other: Self) {
        if other.usage_type.is_some() {
            self.usage_type = other.usage_type;
        }
        if other.floor_area_m2.is_some() {
            self.floor_area_m2 = other.floor_area_m2;
        }
        if other.occupants.is_some() {
            self.occupants = other.occupants;
        }
        if other.comfort_category.is_some() {
            self.comfort_category = other.comfort_category;
        }
        if other.pollution_class.is_some() {
            self.pollution_class = other.pollution_class;
        }
        if other.comfort_model.is_some() {
            self.comfort_model = other.comfort_model;
        }
        if other.t_op_winter_c.is_some() {
            self.t_op_winter_c = other.t_op_winter_c;
        }
        if other.t_op_summer_c.is_some() {
            self.t_op_summer_c = other.t_op_summer_c;
        }
        if other.air_speed_m_s.is_some() {
            self.air_speed_m_s = other.air_speed_m_s;
        }
        if other.clothing_clo.is_some() {
            self.clothing_clo = other.clothing_clo;
        }
        if other.metabolic_rate_met.is_some() {
            self.metabolic_rate_met = other.metabolic_rate_met;
        }
        if other.rh_percent.is_some() {
            self.rh_percent = other.rh_percent;
        }
        if other.outdoor_air_supplied_m3_h.is_some() {
            self.outdoor_air_supplied_m3_h = other.outdoor_air_supplied_m3_h;
        }
        if other.co2_ppm.is_some() {
            self.co2_ppm = other.co2_ppm;
        }
        if other.illuminance_lx.is_some() {
            self.illuminance_lx = other.illuminance_lx;
        }
        if other.noise_db.is_some() {
            self.noise_db = other.noise_db;
        }
        if other.turbulence_intensity_percent.is_some() {
            self.turbulence_intensity_percent = other.turbulence_intensity_percent;
        }
        if other.vent_method.is_some() {
            self.vent_method = other.vent_method;
        }
        if other.vent_system_id.is_some() {
            self.vent_system_id = other.vent_system_id;
        }
    }
}

/// 🧾️ One ordered-row edit of `vent_systems`.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Din16798VentSystemEdit {
    pub op: Din16798RowOp,
    pub index: u32,
    pub id: String,
    pub value: Option<VentSystemDocument>,
    pub patch: Option<Din16798VentSystemPatch>,
}

impl Din16798RowEdit for Din16798VentSystemEdit {
    type Row = VentSystemDocument;
    type Patch = Din16798VentSystemPatch;
    fn key(row: &VentSystemDocument) -> String {
        row.id.clone()
    }

    fn view(&self) -> Din16798RowView<'_, VentSystemDocument, Din16798VentSystemPatch> {
        let (index, id) = (self.index as usize, self.id.as_str());
        match (self.op, &self.value, &self.patch) {
            (Din16798RowOp::Insert, Some(row), None) => Din16798RowView::Insert(index, row),
            (Din16798RowOp::Remove, None, None) => Din16798RowView::Remove(index, id),
            (Din16798RowOp::Replace, Some(row), None) => Din16798RowView::Replace(index, id, row),
            (Din16798RowOp::Patch, None, Some(patch)) => Din16798RowView::Patch(index, id, patch),
            _ => Din16798RowView::Malformed(index),
        }
    }

    fn insert(index: usize, row: VentSystemDocument) -> Self {
        Self { op: Din16798RowOp::Insert, index: slot(index), id: Self::key(&row), value: Some(row), patch: None }
    }

    fn remove(index: usize, id: String) -> Self {
        Self { op: Din16798RowOp::Remove, index: slot(index), id, value: None, patch: None }
    }

    fn replace(index: usize, id: String, row: VentSystemDocument) -> Self {
        Self { op: Din16798RowOp::Replace, index: slot(index), id, value: Some(row), patch: None }
    }

    fn patch(index: usize, id: String, patch: Din16798VentSystemPatch) -> Self {
        Self { op: Din16798RowOp::Patch, index: slot(index), id, value: None, patch: Some(patch) }
    }
}

/// 🩹️ Sparse field patch of one `VentSystemDocument`: the fields it names replace the row's, its nested edits edit the row's nested collections.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Din16798VentSystemPatch {
    pub system_type: Option<String>,
    pub sfp_w_m3_s: Option<f64>,
    pub sfp_required_class: Option<u8>,
    pub heat_recovery_eta: Option<f64>,
    pub oda_class: Option<String>,
    pub filter_sup_class: Option<String>,
    pub years_since_inspection: Option<u32>,
    pub duct_class: Option<String>,
    pub duct_leakage_m3_s_m2: Option<f64>,
    pub design_airflow_m3_h: Option<f64>,
}

impl Din16798RowPatch<VentSystemDocument> for Din16798VentSystemPatch {
    fn apply(&self, row: &mut VentSystemDocument) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.system_type {
            row.system_type = value.clone();
        }
        if let Some(value) = &self.sfp_w_m3_s {
            row.sfp_w_m3_s = value.clone();
        }
        if let Some(value) = &self.sfp_required_class {
            row.sfp_required_class = value.clone();
        }
        if let Some(value) = &self.heat_recovery_eta {
            row.heat_recovery_eta = value.clone();
        }
        if let Some(value) = &self.oda_class {
            row.oda_class = value.clone();
        }
        if let Some(value) = &self.filter_sup_class {
            row.filter_sup_class = value.clone();
        }
        if let Some(value) = &self.years_since_inspection {
            row.years_since_inspection = value.clone();
        }
        if let Some(value) = &self.duct_class {
            row.duct_class = value.clone();
        }
        if let Some(value) = &self.duct_leakage_m3_s_m2 {
            row.duct_leakage_m3_s_m2 = value.clone();
        }
        if let Some(value) = &self.design_airflow_m3_h {
            row.design_airflow_m3_h = value.clone();
        }
        Ok(())
    }

    fn inverse(&self, row: &VentSystemDocument) -> Self {
        Self {
            system_type: self.system_type.as_ref().map(|_| row.system_type.clone()),
            sfp_w_m3_s: self.sfp_w_m3_s.as_ref().map(|_| row.sfp_w_m3_s.clone()),
            sfp_required_class: self.sfp_required_class.as_ref().map(|_| row.sfp_required_class.clone()),
            heat_recovery_eta: self.heat_recovery_eta.as_ref().map(|_| row.heat_recovery_eta.clone()),
            oda_class: self.oda_class.as_ref().map(|_| row.oda_class.clone()),
            filter_sup_class: self.filter_sup_class.as_ref().map(|_| row.filter_sup_class.clone()),
            years_since_inspection: self.years_since_inspection.as_ref().map(|_| row.years_since_inspection.clone()),
            duct_class: self.duct_class.as_ref().map(|_| row.duct_class.clone()),
            duct_leakage_m3_s_m2: self.duct_leakage_m3_s_m2.as_ref().map(|_| row.duct_leakage_m3_s_m2.clone()),
            design_airflow_m3_h: self.design_airflow_m3_h.as_ref().map(|_| row.design_airflow_m3_h.clone()),
        }
    }

    fn absorb(&mut self, other: Self) {
        if other.system_type.is_some() {
            self.system_type = other.system_type;
        }
        if other.sfp_w_m3_s.is_some() {
            self.sfp_w_m3_s = other.sfp_w_m3_s;
        }
        if other.sfp_required_class.is_some() {
            self.sfp_required_class = other.sfp_required_class;
        }
        if other.heat_recovery_eta.is_some() {
            self.heat_recovery_eta = other.heat_recovery_eta;
        }
        if other.oda_class.is_some() {
            self.oda_class = other.oda_class;
        }
        if other.filter_sup_class.is_some() {
            self.filter_sup_class = other.filter_sup_class;
        }
        if other.years_since_inspection.is_some() {
            self.years_since_inspection = other.years_since_inspection;
        }
        if other.duct_class.is_some() {
            self.duct_class = other.duct_class;
        }
        if other.duct_leakage_m3_s_m2.is_some() {
            self.duct_leakage_m3_s_m2 = other.duct_leakage_m3_s_m2;
        }
        if other.design_airflow_m3_h.is_some() {
            self.design_airflow_m3_h = other.design_airflow_m3_h;
        }
    }
}

impl protocol::MutationDiff<Din16798Snapshot> for Din16798Diff {
    fn apply(&self, base: &Din16798Snapshot, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<Din16798Snapshot> {
        let mut next = base.clone();
        if let Some(value) = &self.annex {
            next.annex = value.clone();
        }
        if let Some(value) = &self.theta_rm_c {
            next.theta_rm_c = value.clone();
        }
        if let Some(value) = &self.outdoor_co2_ppm {
            next.outdoor_co2_ppm = value.clone();
        }
        if let Some(value) = &self.envelope_n50_h_inv {
            next.envelope_n50_h_inv = value.clone();
        }
        if let Some(value) = &self.envelope_volume_m3 {
            next.envelope_volume_m3 = value.clone();
        }
        if let Some(value) = &self.cellar_area_m2 {
            next.cellar_area_m2 = value.clone();
        }
        if let Some(value) = &self.cellar_ventilation_m3_h {
            next.cellar_ventilation_m3_h = value.clone();
        }
        if let Some(value) = &self.night_setback_k {
            next.night_setback_k = value.clone();
        }
        apply_rows::<Din16798ZoneEdit>(&mut next.zones, &self.zones).map_err(|error| error.under(["zones"]))?;
        apply_rows::<Din16798VentSystemEdit>(&mut next.vent_systems, &self.vent_systems).map_err(|error| error.under(["ventSystems"]))?;
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        if other.annex.is_some() {
            self.annex = other.annex;
        }
        if other.theta_rm_c.is_some() {
            self.theta_rm_c = other.theta_rm_c;
        }
        if other.outdoor_co2_ppm.is_some() {
            self.outdoor_co2_ppm = other.outdoor_co2_ppm;
        }
        if other.envelope_n50_h_inv.is_some() {
            self.envelope_n50_h_inv = other.envelope_n50_h_inv;
        }
        if other.envelope_volume_m3.is_some() {
            self.envelope_volume_m3 = other.envelope_volume_m3;
        }
        if other.cellar_area_m2.is_some() {
            self.cellar_area_m2 = other.cellar_area_m2;
        }
        if other.cellar_ventilation_m3_h.is_some() {
            self.cellar_ventilation_m3_h = other.cellar_ventilation_m3_h;
        }
        if other.night_setback_k.is_some() {
            self.night_setback_k = other.night_setback_k;
        }
        for edit in other.zones {
            push_row(&mut self.zones, edit);
        }
        for edit in other.vent_systems {
            push_row(&mut self.vent_systems, edit);
        }
    }
}

impl protocol::DiffAlgebra<Din16798Snapshot> for Din16798Diff {
    fn inverse(&self, base: &Din16798Snapshot) -> Self {
        Self {
            annex: self.annex.as_ref().map(|_| base.annex.clone()),
            theta_rm_c: self.theta_rm_c.as_ref().map(|_| base.theta_rm_c.clone()),
            outdoor_co2_ppm: self.outdoor_co2_ppm.as_ref().map(|_| base.outdoor_co2_ppm.clone()),
            envelope_n50_h_inv: self.envelope_n50_h_inv.as_ref().map(|_| base.envelope_n50_h_inv.clone()),
            envelope_volume_m3: self.envelope_volume_m3.as_ref().map(|_| base.envelope_volume_m3.clone()),
            cellar_area_m2: self.cellar_area_m2.as_ref().map(|_| base.cellar_area_m2.clone()),
            cellar_ventilation_m3_h: self.cellar_ventilation_m3_h.as_ref().map(|_| base.cellar_ventilation_m3_h.clone()),
            night_setback_k: self.night_setback_k.as_ref().map(|_| base.night_setback_k.clone()),
            zones: invert_rows::<Din16798ZoneEdit>(&base.zones, &self.zones),
            vent_systems: invert_rows::<Din16798VentSystemEdit>(&base.vent_systems, &self.vent_systems),
        }
    }

    fn between(base: &Din16798Snapshot, other: &Din16798Snapshot) -> Self {
        Self {
            annex: (base.annex != other.annex).then(|| other.annex.clone()),
            theta_rm_c: (base.theta_rm_c != other.theta_rm_c).then(|| other.theta_rm_c.clone()),
            outdoor_co2_ppm: (base.outdoor_co2_ppm != other.outdoor_co2_ppm).then(|| other.outdoor_co2_ppm.clone()),
            envelope_n50_h_inv: (base.envelope_n50_h_inv != other.envelope_n50_h_inv).then(|| other.envelope_n50_h_inv.clone()),
            envelope_volume_m3: (base.envelope_volume_m3 != other.envelope_volume_m3).then(|| other.envelope_volume_m3.clone()),
            cellar_area_m2: (base.cellar_area_m2 != other.cellar_area_m2).then(|| other.cellar_area_m2.clone()),
            cellar_ventilation_m3_h: (base.cellar_ventilation_m3_h != other.cellar_ventilation_m3_h).then(|| other.cellar_ventilation_m3_h.clone()),
            night_setback_k: (base.night_setback_k != other.night_setback_k).then(|| other.night_setback_k.clone()),
            zones: rows_between::<Din16798ZoneEdit>(&base.zones, &other.zones),
            vent_systems: rows_between::<Din16798VentSystemEdit>(&base.vent_systems, &other.vent_systems),
        }
    }

    fn is_empty(&self) -> bool {
        self.annex.is_none() && self.theta_rm_c.is_none() && self.outdoor_co2_ppm.is_none() && self.envelope_n50_h_inv.is_none() && self.envelope_volume_m3.is_none() && self.cellar_area_m2.is_none() && self.cellar_ventilation_m3_h.is_none() && self.night_setback_k.is_none() && self.zones.is_empty() && self.vent_systems.is_empty()
    }
}
