//! 🧱️ EN 1993 diff rows (members): the sparse field patch, key extractor and keyed row delta of 5 document row lists.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `SteelMaterial`.
    pub En1993MaterialPatch of semio_s_artifact_norm_en1993_model::SteelMaterial { set { grade: String, fy: f64, fu: f64, e_modulus: f64, g_modulus: f64, subgrade: String, kind: semio_s_artifact_norm_en1993_model::MaterialKind , ramberg_osgood_n: f64} }
}

/// 🔑️ Key extractor of the foreign `SteelMaterial` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct En1993MaterialDeltaKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1993_model::SteelMaterial> for En1993MaterialDeltaKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1993_model::SteelMaterial) -> String {
        row.id.clone()
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `SteelMaterial` list.
    pub En1993MaterialDelta { removal: En1993MaterialRemoval, insertion: En1993MaterialInsertion, relocation: En1993MaterialRelocation, modification: En1993MaterialModification, row: semio_s_artifact_norm_en1993_model::SteelMaterial, patch: En1993MaterialPatch, list: Vec<semio_s_artifact_norm_en1993_model::SteelMaterial>, key: String = by En1993MaterialDeltaKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `SteelSection`.
    pub En1993SectionPatch of semio_s_artifact_norm_en1993_model::SteelSection { set { designation: String, kind: semio_s_artifact_norm_en1993_model::SectionKind, h: f64, b: f64, tw: f64, tf: f64, r: f64, area: f64, shear_area_y: f64, shear_area_z: f64, iy: f64, iz: f64, it: f64, iw: f64, w_el_y: f64, w_el_z: f64, w_pl_y: f64, w_pl_z: f64, area_net: f64 , fabrication: semio_s_artifact_norm_en1993_model::HollowFabrication} }
}

/// 🔑️ Key extractor of the foreign `SteelSection` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct En1993SectionDeltaKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1993_model::SteelSection> for En1993SectionDeltaKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1993_model::SteelSection) -> String {
        row.id.clone()
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `SteelSection` list.
    pub En1993SectionDelta { removal: En1993SectionRemoval, insertion: En1993SectionInsertion, relocation: En1993SectionRelocation, modification: En1993SectionModification, row: semio_s_artifact_norm_en1993_model::SteelSection, patch: En1993SectionPatch, list: Vec<semio_s_artifact_norm_en1993_model::SteelSection>, key: String = by En1993SectionDeltaKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `SteelMember`.
    pub En1993MemberPatch of semio_s_artifact_norm_en1993_model::SteelMember { set { label: String, member_type: semio_s_artifact_norm_en1993_model::SteelMemberType, section_id: String, material_id: String, length: f64, buckling_length_y: f64, buckling_length_z: f64, ltb_length: f64, ltb_restraint_spacing: f64, load_application: semio_s_artifact_norm_en1993_model::LoadApplication, end_moment_ratio_psi: f64, moment_diagram: semio_s_artifact_norm_en1993_model::MomentDiagram, deflection_limit_ratio: f64, analysis: semio_s_artifact_norm_en1993_model::AnalysisKind , stainless_forming: semio_s_artifact_norm_en1993_model::StainlessForming, design_temperature: f64, toughness_stress_ratio: f64, radiation_shift: f64, stress_shift: f64, margin_shift: f64, strain_rate_shift: f64, cold_forming_radius: f64, web_material_id: String, torsion_load: semio_s_artifact_norm_en1993_model::TorsionLoad, torque_position: f64, warping_support: semio_s_artifact_norm_en1993_model::WarpingSupport} }
}

/// 🔑️ Key extractor of the foreign `SteelMember` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct En1993MemberDeltaKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1993_model::SteelMember> for En1993MemberDeltaKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1993_model::SteelMember) -> String {
        row.id.clone()
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `SteelMember` list.
    pub En1993MemberDelta { removal: En1993MemberRemoval, insertion: En1993MemberInsertion, relocation: En1993MemberRelocation, modification: En1993MemberModification, row: semio_s_artifact_norm_en1993_model::SteelMember, patch: En1993MemberPatch, list: Vec<semio_s_artifact_norm_en1993_model::SteelMember>, key: String = by En1993MemberDeltaKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `LoadCase`.
    pub En1993LoadCasePatch of semio_s_artifact_norm_en1993_model::LoadCase { set { name: String, kind: semio_s_artifact_norm_en1993_model::ActionKind, category: String } }
}

/// 🔑️ Key extractor of the foreign `LoadCase` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct En1993LoadCaseDeltaKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1993_model::LoadCase> for En1993LoadCaseDeltaKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1993_model::LoadCase) -> String {
        row.id.clone()
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `LoadCase` list.
    pub En1993LoadCaseDelta { removal: En1993LoadCaseRemoval, insertion: En1993LoadCaseInsertion, relocation: En1993LoadCaseRelocation, modification: En1993LoadCaseModification, row: semio_s_artifact_norm_en1993_model::LoadCase, patch: En1993LoadCasePatch, list: Vec<semio_s_artifact_norm_en1993_model::LoadCase>, key: String = by En1993LoadCaseDeltaKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `MemberAction`.
    pub En1993MemberActionPatch of semio_s_artifact_norm_en1993_model::MemberAction { set { member_id: String, load_case_id: String, action: semio_s_artifact_norm_en1993_model::DesignAction } }
}

/// 🔑️ Key extractor of the foreign `MemberAction` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct En1993MemberActionDeltaKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1993_model::MemberAction> for En1993MemberActionDeltaKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1993_model::MemberAction) -> String {
        row.id.clone()
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `MemberAction` list.
    pub En1993MemberActionDelta { removal: En1993MemberActionRemoval, insertion: En1993MemberActionInsertion, relocation: En1993MemberActionRelocation, modification: En1993MemberActionModification, row: semio_s_artifact_norm_en1993_model::MemberAction, patch: En1993MemberActionPatch, list: Vec<semio_s_artifact_norm_en1993_model::MemberAction>, key: String = by En1993MemberActionDeltaKey }
}
