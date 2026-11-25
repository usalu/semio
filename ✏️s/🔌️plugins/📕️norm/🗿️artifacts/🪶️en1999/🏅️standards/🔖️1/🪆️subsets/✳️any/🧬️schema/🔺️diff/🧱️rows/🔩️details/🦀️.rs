//! 🧱️ EN 1999 detail row diffs and keyed list deltas.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_value_derive as value_derive;

/// 🔑️ Key extractor of the foreign `SectionVertex` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SectionVertexKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1999_model::SectionVertex> for SectionVertexKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1999_model::SectionVertex) -> String {
        row.id.clone()
    }
}

/// 🔑️ Key extractor of the foreign `FatigueCategory` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FatigueCategoryKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1999_model::FatigueCategory> for FatigueCategoryKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1999_model::FatigueCategory) -> String {
        row.id.clone()
    }
}

/// 🔑️ Key extractor of the foreign `PatchLoad` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PatchLoadKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1999_model::PatchLoad> for PatchLoadKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1999_model::PatchLoad) -> String {
        row.id.clone()
    }
}

/// 🔑️ Key extractor of the foreign `RingStiffener` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RingStiffenerKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1999_model::RingStiffener> for RingStiffenerKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1999_model::RingStiffener) -> String {
        row.id.clone()
    }
}

/// 🔑️ Key extractor of the foreign `TransverseStiffener` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TransverseStiffenerKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1999_model::TransverseStiffener> for TransverseStiffenerKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1999_model::TransverseStiffener) -> String {
        row.id.clone()
    }
}

/// 🩹️ Sparse patch of the `vertices` row addressed by its key.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1999OutlineVerticesPatch {
    pub section_id: Option<String>,
    pub ring: Option<semio_s_artifact_norm_en1999_model::OutlineRing>,
    pub element_id: Option<String>,
    pub y: Option<f64>,
    pub z: Option<f64>,
}

impl protocol::list_delta::RowPatch<semio_s_artifact_norm_en1999_model::SectionVertex> for En1999OutlineVerticesPatch {
    fn commit_into(&self, row: &mut semio_s_artifact_norm_en1999_model::SectionVertex, _capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.section_id { row.section_id = value.clone(); }
        if let Some(value) = &self.ring { row.ring = value.clone(); }
        if let Some(value) = &self.element_id { row.element_id = value.clone(); }
        if let Some(value) = &self.y { row.y = value.clone(); }
        if let Some(value) = &self.z { row.z = value.clone(); }
        Ok(())
    }

    fn absorb(&mut self, later: Self) {
        if later.section_id.is_some() { self.section_id = later.section_id; }
        if later.ring.is_some() { self.ring = later.ring; }
        if later.element_id.is_some() { self.element_id = later.element_id; }
        if later.y.is_some() { self.y = later.y; }
        if later.z.is_some() { self.z = later.z; }
    }

    fn inverse(&self, row: &semio_s_artifact_norm_en1999_model::SectionVertex) -> Self {
        Self {
            section_id: self.section_id.as_ref().map(|_| row.section_id.clone()),
            ring: self.ring.as_ref().map(|_| row.ring.clone()),
            element_id: self.element_id.as_ref().map(|_| row.element_id.clone()),
            y: self.y.as_ref().map(|_| row.y.clone()),
            z: self.z.as_ref().map(|_| row.z.clone()),
        }
    }

    fn is_empty(&self) -> bool {
        self.section_id.is_none() && self.ring.is_none() && self.element_id.is_none() && self.y.is_none() && self.z.is_none()
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `outline_vertices` list (rows keyed by `id`).
    pub En1999OutlineVerticesRows { removal: En1999OutlineVerticesRemoved, insertion: En1999OutlineVerticesInserted, relocation: En1999OutlineVerticesMoved, modification: En1999OutlineVerticesModified, row: semio_s_artifact_norm_en1999_model::SectionVertex, patch: En1999OutlineVerticesPatch, list: Vec<semio_s_artifact_norm_en1999_model::SectionVertex>, key: String = by SectionVertexKey }
}
/// 🩹️ Sparse patch of the `categories` row addressed by its key.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1999FatigueCategoriesPatch {
    pub member_id: Option<String>,
    pub element_id: Option<String>,
    pub detail_category: Option<String>,
    pub detail_type: Option<semio_s_artifact_norm_en1999_model::DetailType>,
    pub location_x: Option<f64>,
    pub m1: Option<f64>,
    pub m2: Option<f64>,
}

impl protocol::list_delta::RowPatch<semio_s_artifact_norm_en1999_model::FatigueCategory> for En1999FatigueCategoriesPatch {
    fn commit_into(&self, row: &mut semio_s_artifact_norm_en1999_model::FatigueCategory, _capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.member_id { row.member_id = value.clone(); }
        if let Some(value) = &self.element_id { row.element_id = value.clone(); }
        if let Some(value) = &self.detail_category { row.detail_category = value.clone(); }
        if let Some(value) = &self.detail_type { row.detail_type = value.clone(); }
        if let Some(value) = &self.location_x { row.location_x = value.clone(); }
        if let Some(value) = &self.m1 { row.m1 = value.clone(); }
        if let Some(value) = &self.m2 { row.m2 = value.clone(); }
        Ok(())
    }

    fn absorb(&mut self, later: Self) {
        if later.member_id.is_some() { self.member_id = later.member_id; }
        if later.element_id.is_some() { self.element_id = later.element_id; }
        if later.detail_category.is_some() { self.detail_category = later.detail_category; }
        if later.detail_type.is_some() { self.detail_type = later.detail_type; }
        if later.location_x.is_some() { self.location_x = later.location_x; }
        if later.m1.is_some() { self.m1 = later.m1; }
        if later.m2.is_some() { self.m2 = later.m2; }
    }

    fn inverse(&self, row: &semio_s_artifact_norm_en1999_model::FatigueCategory) -> Self {
        Self {
            member_id: self.member_id.as_ref().map(|_| row.member_id.clone()),
            element_id: self.element_id.as_ref().map(|_| row.element_id.clone()),
            detail_category: self.detail_category.as_ref().map(|_| row.detail_category.clone()),
            detail_type: self.detail_type.as_ref().map(|_| row.detail_type.clone()),
            location_x: self.location_x.as_ref().map(|_| row.location_x.clone()),
            m1: self.m1.as_ref().map(|_| row.m1.clone()),
            m2: self.m2.as_ref().map(|_| row.m2.clone()),
        }
    }

    fn is_empty(&self) -> bool {
        self.member_id.is_none() && self.element_id.is_none() && self.detail_category.is_none() && self.detail_type.is_none() && self.location_x.is_none() && self.m1.is_none() && self.m2.is_none()
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `fatigue_categories` list (rows keyed by `id`).
    pub En1999FatigueCategoriesRows { removal: En1999FatigueCategoriesRemoved, insertion: En1999FatigueCategoriesInserted, relocation: En1999FatigueCategoriesMoved, modification: En1999FatigueCategoriesModified, row: semio_s_artifact_norm_en1999_model::FatigueCategory, patch: En1999FatigueCategoriesPatch, list: Vec<semio_s_artifact_norm_en1999_model::FatigueCategory>, key: String = by FatigueCategoryKey }
}
/// 🩹️ Sparse patch of the `loads` row addressed by its key.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1999PatchLoadsPatch {
    pub subject_id: Option<String>,
    pub position: Option<semio_s_artifact_norm_en1999_model::PatchLoadPosition>,
    pub x: Option<f64>,
    pub bearing_length: Option<f64>,
    pub web_height: Option<f64>,
    pub web_angle: Option<f64>,
    pub corner_radius: Option<f64>,
    pub load: Option<f64>,
}

impl protocol::list_delta::RowPatch<semio_s_artifact_norm_en1999_model::PatchLoad> for En1999PatchLoadsPatch {
    fn commit_into(&self, row: &mut semio_s_artifact_norm_en1999_model::PatchLoad, _capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.subject_id { row.subject_id = value.clone(); }
        if let Some(value) = &self.position { row.position = value.clone(); }
        if let Some(value) = &self.x { row.x = value.clone(); }
        if let Some(value) = &self.bearing_length { row.bearing_length = value.clone(); }
        if let Some(value) = &self.web_height { row.web_height = value.clone(); }
        if let Some(value) = &self.web_angle { row.web_angle = value.clone(); }
        if let Some(value) = &self.corner_radius { row.corner_radius = value.clone(); }
        if let Some(value) = &self.load { row.load = value.clone(); }
        Ok(())
    }

    fn absorb(&mut self, later: Self) {
        if later.subject_id.is_some() { self.subject_id = later.subject_id; }
        if later.position.is_some() { self.position = later.position; }
        if later.x.is_some() { self.x = later.x; }
        if later.bearing_length.is_some() { self.bearing_length = later.bearing_length; }
        if later.web_height.is_some() { self.web_height = later.web_height; }
        if later.web_angle.is_some() { self.web_angle = later.web_angle; }
        if later.corner_radius.is_some() { self.corner_radius = later.corner_radius; }
        if later.load.is_some() { self.load = later.load; }
    }

    fn inverse(&self, row: &semio_s_artifact_norm_en1999_model::PatchLoad) -> Self {
        Self {
            subject_id: self.subject_id.as_ref().map(|_| row.subject_id.clone()),
            position: self.position.as_ref().map(|_| row.position.clone()),
            x: self.x.as_ref().map(|_| row.x.clone()),
            bearing_length: self.bearing_length.as_ref().map(|_| row.bearing_length.clone()),
            web_height: self.web_height.as_ref().map(|_| row.web_height.clone()),
            web_angle: self.web_angle.as_ref().map(|_| row.web_angle.clone()),
            corner_radius: self.corner_radius.as_ref().map(|_| row.corner_radius.clone()),
            load: self.load.as_ref().map(|_| row.load.clone()),
        }
    }

    fn is_empty(&self) -> bool {
        self.subject_id.is_none() && self.position.is_none() && self.x.is_none() && self.bearing_length.is_none() && self.web_height.is_none() && self.web_angle.is_none() && self.corner_radius.is_none() && self.load.is_none()
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `patch_loads` list (rows keyed by `id`).
    pub En1999PatchLoadsRows { removal: En1999PatchLoadsRemoved, insertion: En1999PatchLoadsInserted, relocation: En1999PatchLoadsMoved, modification: En1999PatchLoadsModified, row: semio_s_artifact_norm_en1999_model::PatchLoad, patch: En1999PatchLoadsPatch, list: Vec<semio_s_artifact_norm_en1999_model::PatchLoad>, key: String = by PatchLoadKey }
}
/// 🩹️ Sparse patch of the `stiffeners` row addressed by its key.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1999RingStiffenersPatch {
    pub shell_id: Option<String>,
    pub x: Option<f64>,
    pub depth: Option<f64>,
    pub thickness: Option<f64>,
    pub internal: Option<bool>,
}

impl protocol::list_delta::RowPatch<semio_s_artifact_norm_en1999_model::RingStiffener> for En1999RingStiffenersPatch {
    fn commit_into(&self, row: &mut semio_s_artifact_norm_en1999_model::RingStiffener, _capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.shell_id { row.shell_id = value.clone(); }
        if let Some(value) = &self.x { row.x = value.clone(); }
        if let Some(value) = &self.depth { row.depth = value.clone(); }
        if let Some(value) = &self.thickness { row.thickness = value.clone(); }
        if let Some(value) = &self.internal { row.internal = value.clone(); }
        Ok(())
    }

    fn absorb(&mut self, later: Self) {
        if later.shell_id.is_some() { self.shell_id = later.shell_id; }
        if later.x.is_some() { self.x = later.x; }
        if later.depth.is_some() { self.depth = later.depth; }
        if later.thickness.is_some() { self.thickness = later.thickness; }
        if later.internal.is_some() { self.internal = later.internal; }
    }

    fn inverse(&self, row: &semio_s_artifact_norm_en1999_model::RingStiffener) -> Self {
        Self {
            shell_id: self.shell_id.as_ref().map(|_| row.shell_id.clone()),
            x: self.x.as_ref().map(|_| row.x.clone()),
            depth: self.depth.as_ref().map(|_| row.depth.clone()),
            thickness: self.thickness.as_ref().map(|_| row.thickness.clone()),
            internal: self.internal.as_ref().map(|_| row.internal.clone()),
        }
    }

    fn is_empty(&self) -> bool {
        self.shell_id.is_none() && self.x.is_none() && self.depth.is_none() && self.thickness.is_none() && self.internal.is_none()
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `ring_stiffeners` list (rows keyed by `id`).
    pub En1999RingStiffenersRows { removal: En1999RingStiffenersRemoved, insertion: En1999RingStiffenersInserted, relocation: En1999RingStiffenersMoved, modification: En1999RingStiffenersModified, row: semio_s_artifact_norm_en1999_model::RingStiffener, patch: En1999RingStiffenersPatch, list: Vec<semio_s_artifact_norm_en1999_model::RingStiffener>, key: String = by RingStiffenerKey }
}

/// 🔑️ Key extractor of the foreign `SheetProfile` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SheetProfileKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1999_model::SheetProfile> for SheetProfileKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1999_model::SheetProfile) -> String {
        row.id.clone()
    }
}

/// 🩹️ Sparse patch of the `stiffeners` row addressed by its key.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1999SheetProfilesPatch {
    pub sheet_id: Option<String>,
    pub pitch: Option<f64>,
    pub top_flange: Option<f64>,
    pub bottom_flange: Option<f64>,
    pub web_height: Option<f64>,
}

impl protocol::list_delta::RowPatch<semio_s_artifact_norm_en1999_model::SheetProfile> for En1999SheetProfilesPatch {
    fn commit_into(&self, row: &mut semio_s_artifact_norm_en1999_model::SheetProfile, _capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.sheet_id { row.sheet_id = value.clone(); }
        if let Some(value) = &self.pitch { row.pitch = value.clone(); }
        if let Some(value) = &self.top_flange { row.top_flange = value.clone(); }
        if let Some(value) = &self.bottom_flange { row.bottom_flange = value.clone(); }
        if let Some(value) = &self.web_height { row.web_height = value.clone(); }
        Ok(())
    }

    fn absorb(&mut self, later: Self) {
        if later.sheet_id.is_some() { self.sheet_id = later.sheet_id; }
        if later.pitch.is_some() { self.pitch = later.pitch; }
        if later.top_flange.is_some() { self.top_flange = later.top_flange; }
        if later.bottom_flange.is_some() { self.bottom_flange = later.bottom_flange; }
        if later.web_height.is_some() { self.web_height = later.web_height; }
    }

    fn inverse(&self, row: &semio_s_artifact_norm_en1999_model::SheetProfile) -> Self {
        Self {
            sheet_id: self.sheet_id.as_ref().map(|_| row.sheet_id.clone()),
            pitch: self.pitch.as_ref().map(|_| row.pitch.clone()),
            top_flange: self.top_flange.as_ref().map(|_| row.top_flange.clone()),
            bottom_flange: self.bottom_flange.as_ref().map(|_| row.bottom_flange.clone()),
            web_height: self.web_height.as_ref().map(|_| row.web_height.clone()),
        }
    }

    fn is_empty(&self) -> bool {
        self.sheet_id.is_none() && self.pitch.is_none() && self.top_flange.is_none() && self.bottom_flange.is_none() && self.web_height.is_none()
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `sheet_profiles` list (rows keyed by `id`).
    pub En1999SheetProfilesRows { removal: En1999SheetProfilesRemoved, insertion: En1999SheetProfilesInserted, relocation: En1999SheetProfilesMoved, modification: En1999SheetProfilesModified, row: semio_s_artifact_norm_en1999_model::SheetProfile, patch: En1999SheetProfilesPatch, list: Vec<semio_s_artifact_norm_en1999_model::SheetProfile>, key: String = by SheetProfileKey }
}

/// 🔑️ Key extractor of the foreign `FireInsulation` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FireInsulationKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1999_model::FireInsulation> for FireInsulationKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1999_model::FireInsulation) -> String {
        row.id.clone()
    }
}

/// 🩹️ Sparse patch of the `stiffeners` row addressed by its key.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1999FireInsulationsPatch {
    pub fire_id: Option<String>,
    pub thickness: Option<f64>,
    pub conductivity: Option<f64>,
    pub specific_heat: Option<f64>,
    pub density: Option<f64>,
}

impl protocol::list_delta::RowPatch<semio_s_artifact_norm_en1999_model::FireInsulation> for En1999FireInsulationsPatch {
    fn commit_into(&self, row: &mut semio_s_artifact_norm_en1999_model::FireInsulation, _capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.fire_id { row.fire_id = value.clone(); }
        if let Some(value) = &self.thickness { row.thickness = value.clone(); }
        if let Some(value) = &self.conductivity { row.conductivity = value.clone(); }
        if let Some(value) = &self.specific_heat { row.specific_heat = value.clone(); }
        if let Some(value) = &self.density { row.density = value.clone(); }
        Ok(())
    }

    fn absorb(&mut self, later: Self) {
        if later.fire_id.is_some() { self.fire_id = later.fire_id; }
        if later.thickness.is_some() { self.thickness = later.thickness; }
        if later.conductivity.is_some() { self.conductivity = later.conductivity; }
        if later.specific_heat.is_some() { self.specific_heat = later.specific_heat; }
        if later.density.is_some() { self.density = later.density; }
    }

    fn inverse(&self, row: &semio_s_artifact_norm_en1999_model::FireInsulation) -> Self {
        Self {
            fire_id: self.fire_id.as_ref().map(|_| row.fire_id.clone()),
            thickness: self.thickness.as_ref().map(|_| row.thickness.clone()),
            conductivity: self.conductivity.as_ref().map(|_| row.conductivity.clone()),
            specific_heat: self.specific_heat.as_ref().map(|_| row.specific_heat.clone()),
            density: self.density.as_ref().map(|_| row.density.clone()),
        }
    }

    fn is_empty(&self) -> bool {
        self.fire_id.is_none() && self.thickness.is_none() && self.conductivity.is_none() && self.specific_heat.is_none() && self.density.is_none()
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `fire_insulations` list (rows keyed by `id`).
    pub En1999FireInsulationsRows { removal: En1999FireInsulationsRemoved, insertion: En1999FireInsulationsInserted, relocation: En1999FireInsulationsMoved, modification: En1999FireInsulationsModified, row: semio_s_artifact_norm_en1999_model::FireInsulation, patch: En1999FireInsulationsPatch, list: Vec<semio_s_artifact_norm_en1999_model::FireInsulation>, key: String = by FireInsulationKey }
}
/// 🩹️ Sparse patch of the `stiffeners` row addressed by its key.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1999TransverseStiffenersPatch {
    pub girder_id: Option<String>,
    pub position: Option<f64>,
    pub width: Option<f64>,
    pub thickness: Option<f64>,
    pub sides: Option<semio_s_artifact_norm_en1999_model::StiffenerSides>,
}

impl protocol::list_delta::RowPatch<semio_s_artifact_norm_en1999_model::TransverseStiffener> for En1999TransverseStiffenersPatch {
    fn commit_into(&self, row: &mut semio_s_artifact_norm_en1999_model::TransverseStiffener, _capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.girder_id { row.girder_id = value.clone(); }
        if let Some(value) = &self.position { row.position = value.clone(); }
        if let Some(value) = &self.width { row.width = value.clone(); }
        if let Some(value) = &self.thickness { row.thickness = value.clone(); }
        if let Some(value) = &self.sides { row.sides = value.clone(); }
        Ok(())
    }

    fn absorb(&mut self, later: Self) {
        if later.girder_id.is_some() { self.girder_id = later.girder_id; }
        if later.position.is_some() { self.position = later.position; }
        if later.width.is_some() { self.width = later.width; }
        if later.thickness.is_some() { self.thickness = later.thickness; }
        if later.sides.is_some() { self.sides = later.sides; }
    }

    fn inverse(&self, row: &semio_s_artifact_norm_en1999_model::TransverseStiffener) -> Self {
        Self {
            girder_id: self.girder_id.as_ref().map(|_| row.girder_id.clone()),
            position: self.position.as_ref().map(|_| row.position.clone()),
            width: self.width.as_ref().map(|_| row.width.clone()),
            thickness: self.thickness.as_ref().map(|_| row.thickness.clone()),
            sides: self.sides.as_ref().map(|_| row.sides.clone()),
        }
    }

    fn is_empty(&self) -> bool {
        self.girder_id.is_none() && self.position.is_none() && self.width.is_none() && self.thickness.is_none() && self.sides.is_none()
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `transverse_stiffeners` list (rows keyed by `id`).
    pub En1999TransverseStiffenersRows { removal: En1999TransverseStiffenersRemoved, insertion: En1999TransverseStiffenersInserted, relocation: En1999TransverseStiffenersMoved, modification: En1999TransverseStiffenersModified, row: semio_s_artifact_norm_en1999_model::TransverseStiffener, patch: En1999TransverseStiffenersPatch, list: Vec<semio_s_artifact_norm_en1999_model::TransverseStiffener>, key: String = by TransverseStiffenerKey }
}


