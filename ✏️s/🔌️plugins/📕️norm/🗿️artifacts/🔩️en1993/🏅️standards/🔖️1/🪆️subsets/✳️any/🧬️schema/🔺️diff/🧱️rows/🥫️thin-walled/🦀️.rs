//! 🧱️ EN 1993 diff rows (thin-walled): the sparse field patch, key extractor and keyed row delta of 5 document row lists.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `ColdFormedMember`.
    pub En1993ColdFormedMemberPatch of semio_s_artifact_norm_en1993_model::ColdFormedMember { set { b_bar: f64, thickness: f64, k_sigma: f64, psi: f64, fy: f64, gross_resistance: f64, actions: Vec<semio_s_artifact_norm_en1993_model::ForceAction> , flange_width: f64, lip_length: f64, web_height: f64, web_slope: f64, opposite_flange_width: f64, spring_factor: f64, corner_radius: f64, flange_stress: f64, shear_force: f64, support_reaction: f64, support_length: f64, crippling_category: u32, stiffened_support: u32, bending_moment: f64, moment_resistance: f64, sheeting_stiffness: f64, required_restraint_stiffness: f64, purlin_span: f64, purlin_load: f64, purlin_lateral_ratio: f64} }
}

/// 🔑️ Key extractor of the foreign `ColdFormedMember` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct En1993ColdFormedMemberDeltaKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1993_model::ColdFormedMember> for En1993ColdFormedMemberDeltaKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1993_model::ColdFormedMember) -> String {
        row.id.clone()
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `ColdFormedMember` list.
    pub En1993ColdFormedMemberDelta { removal: En1993ColdFormedMemberRemoval, insertion: En1993ColdFormedMemberInsertion, relocation: En1993ColdFormedMemberRelocation, modification: En1993ColdFormedMemberModification, row: semio_s_artifact_norm_en1993_model::ColdFormedMember, patch: En1993ColdFormedMemberPatch, list: Vec<semio_s_artifact_norm_en1993_model::ColdFormedMember>, key: String = by En1993ColdFormedMemberDeltaKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `PlatedPanel`.
    pub En1993PlatedPanelPatch of semio_s_artifact_norm_en1993_model::PlatedPanel { set { a: f64, b: f64, thickness: f64, fy: f64, k_sigma: f64, actions: Vec<semio_s_artifact_norm_en1993_model::ForceAction> , stiffener_inertia: f64, stiffener_area: f64, stiffener_eccentricity: f64, stiffener_alpha: f64, compression_stress: f64, half_width: f64, equivalent_span: f64, hogging: u32, patch_type: semio_s_artifact_norm_en1993_model::PatchLoadType, patch_force: f64, patch_length: f64, patch_offset: f64, flange_thickness: f64, flange_width: f64, flange_fy: f64, longitudinal_offset: f64, longitudinal_inertia: f64, patch_moment: f64, patch_moment_resistance: f64, bending_moment: f64, plastic_moment: f64, flange_moment: f64, shear_force: f64, transverse_stiffener_inertia: f64, flange_induced_k: f64, flange_area: f64, stress_x: f64, stress_z: f64, shear_stress: f64, alpha_cr: f64, corrugation_flat: f64, corrugation_projection: f64, corrugation_depth: f64, web_height: f64, stiffener_spacing: f64, stiffener_shape: semio_s_artifact_norm_en1993_model::StiffenerShape, stiffener_depth: f64, stiffener_thickness: f64, stiffener_flange_width: f64, stiffener_flange_thickness: f64, lateral_pressure: f64, lateral_pressure_coefficient: f64, steel_grade: String, steel_subgrade: String, design_temperature: f64} }
}

/// 🔑️ Key extractor of the foreign `PlatedPanel` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct En1993PlatedPanelDeltaKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1993_model::PlatedPanel> for En1993PlatedPanelDeltaKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1993_model::PlatedPanel) -> String {
        row.id.clone()
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `PlatedPanel` list.
    pub En1993PlatedPanelDelta { removal: En1993PlatedPanelRemoval, insertion: En1993PlatedPanelInsertion, relocation: En1993PlatedPanelRelocation, modification: En1993PlatedPanelModification, row: semio_s_artifact_norm_en1993_model::PlatedPanel, patch: En1993PlatedPanelPatch, list: Vec<semio_s_artifact_norm_en1993_model::PlatedPanel>, key: String = by En1993PlatedPanelDeltaKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `SiloShell`.
    pub En1993SiloShellPatch of semio_s_artifact_norm_en1993_model::SiloShell { set { thickness: f64, radius: f64, depth: f64, k: f64, gamma: f64, fy: f64 , length: f64, fabrication_class: semio_s_artifact_norm_en1993_model::FabricationToleranceClass, meridional_stress: f64, hoop_stress: f64, shell_shear_stress: f64, boundary_factor_axial: f64, boundary_factor_hoop: f64, boundary_factor_shear: f64, wall_friction: f64, plan_area: f64, plan_perimeter: f64, liquid_density: f64, liquid_height: f64, course_elevation: f64, internal_pressure: f64, external_pressure: f64, temperature_change: f64, ring_spacing: f64, ring_area: f64, ring_inertia: f64, ring_offset: f64} }
}

/// 🔑️ Key extractor of the foreign `SiloShell` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct En1993SiloShellDeltaKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1993_model::SiloShell> for En1993SiloShellDeltaKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1993_model::SiloShell) -> String {
        row.id.clone()
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `SiloShell` list.
    pub En1993SiloShellDelta { removal: En1993SiloShellRemoval, insertion: En1993SiloShellInsertion, relocation: En1993SiloShellRelocation, modification: En1993SiloShellModification, row: semio_s_artifact_norm_en1993_model::SiloShell, patch: En1993SiloShellPatch, list: Vec<semio_s_artifact_norm_en1993_model::SiloShell>, key: String = by En1993SiloShellDeltaKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `TensionComponent`.
    pub En1993TensionComponentPatch of semio_s_artifact_norm_en1993_model::TensionComponent { set { f_uk: f64, f_k: f64, actions: Vec<semio_s_artifact_norm_en1993_model::ForceAction> , partial_factor: f64, cable_length: f64, cable_weight: f64, cable_horizontal_force: f64, cable_modulus: f64, cable_stress: f64, cable_unit_weight: f64, thread_factor: f64, rod_stress_area: f64, rod_fu: f64, pin_diameter: f64, pin_plate_thickness: f64, pin_gap: f64, pin_outer_thickness: f64, pin_inner_thickness: f64, pin_fup: f64, pin_fyp: f64, pin_plate_fy: f64} }
}

/// 🔑️ Key extractor of the foreign `TensionComponent` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct En1993TensionComponentDeltaKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1993_model::TensionComponent> for En1993TensionComponentDeltaKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1993_model::TensionComponent) -> String {
        row.id.clone()
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `TensionComponent` list.
    pub En1993TensionComponentDelta { removal: En1993TensionComponentRemoval, insertion: En1993TensionComponentInsertion, relocation: En1993TensionComponentRelocation, modification: En1993TensionComponentModification, row: semio_s_artifact_norm_en1993_model::TensionComponent, patch: En1993TensionComponentPatch, list: Vec<semio_s_artifact_norm_en1993_model::TensionComponent>, key: String = by En1993TensionComponentDeltaKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `BuiltUpMember`.
    pub En1993BuiltUpMemberPatch of semio_s_artifact_norm_en1993_model::BuiltUpMember { set { member_id: String, label: String, kind: semio_s_artifact_norm_en1993_model::BuiltUpKind, chord_area: f64, chord_inertia: f64, chord_distance: f64, panel_spacing: f64, planes: u32, batten_inertia: f64, diagonal_area: f64, diagonal_length: f64, design_axial: f64, design_moment: f64, back_to_back: u32, min_radius: f64, chain_spacing: f64, chord_shape: semio_s_artifact_norm_en1993_model::ChordShape, chord_depth: f64, chord_width: f64, chord_flange_thickness: f64, chord_web_thickness: f64} }
}

/// 🔑️ Key extractor of the foreign `BuiltUpMember` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct En1993BuiltUpMemberDeltaKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1993_model::BuiltUpMember> for En1993BuiltUpMemberDeltaKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1993_model::BuiltUpMember) -> String {
        row.id.clone()
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `BuiltUpMember` list.
    pub En1993BuiltUpMemberDelta { removal: En1993BuiltUpMemberRemoval, insertion: En1993BuiltUpMemberInsertion, relocation: En1993BuiltUpMemberRelocation, modification: En1993BuiltUpMemberModification, row: semio_s_artifact_norm_en1993_model::BuiltUpMember, patch: En1993BuiltUpMemberPatch, list: Vec<semio_s_artifact_norm_en1993_model::BuiltUpMember>, key: String = by En1993BuiltUpMemberDeltaKey }
}
