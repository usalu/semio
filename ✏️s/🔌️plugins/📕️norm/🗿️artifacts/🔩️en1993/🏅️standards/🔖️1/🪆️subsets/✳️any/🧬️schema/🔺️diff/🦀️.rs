//! 🧬️ En1993 diff schema — sparse scalar fields plus keyed row deltas for every list the document owns.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use framework_schema::ArtifactSchema;
use protocol::{ColdDiffAlgebra, DiffAlgebra, MutationDiff};

use semio_s_artifact_norm_en1993_model::En1993Snapshot;

//#region 🔖️Rows
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

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `SteelJoint`.
    pub En1993JointPatch of semio_s_artifact_norm_en1993_model::SteelJoint { set { kind: semio_s_artifact_norm_en1993_model::JointKind, member_id: String, bolt_class: String, bolt_diameter: f64, bolt_rows: u32, bolts_per_row: u32, pitch: f64, gauge: f64, end_distance: f64, edge_distance: f64, shear_planes: u32, plate_thickness: f64, plate_fu: f64, weld_throat: f64, weld_length: f64, weld_fu: f64, weld_grade: String, actions: Vec<semio_s_artifact_norm_en1993_model::JointForceAction>, category: semio_s_artifact_norm_en1993_model::JointCategory, friction_mu: f64, preload_force: f64, slip_factor_ks: f64, friction_surfaces: u32 , plate_fy: f64, plate_width: f64, bolt_to_web: f64, extension_to_flange: f64, extension_edge: f64, alpha_factor: f64, bolt_stress_area: f64, bolt_grip_length: f64, column_flange_thickness: f64, column_web_thickness: f64, column_web_depth: f64, column_web_shear_area: f64, column_root_radius: f64, column_fy: f64, column_web_stress: f64, beam_flange_thickness: f64, compression_lever: f64, moment_demand: f64, beam_moment_resistance: f64, column_moment_resistance: f64, beam_inertia: f64, beam_span: f64, braced_frame: u32, stiffness_ratio: f64, transformation_parameter: f64, assumed_class: semio_s_artifact_norm_en1993_model::StiffnessClass, base_plate_length: f64, concrete_fcd: f64, concrete_modulus: f64, concrete_area_a0: f64, concrete_area_a1: f64, joint_coefficient: f64, anchor_area: f64, anchor_length: f64, base_normal_force: f64, base_moment: f64, base_lever_left: f64, base_lever_right: f64, beam_web_thickness: f64, beam_web_fy: f64, weld_force_longitudinal: f64, weld_force_transverse: f64} }
}

/// 🔑️ Key extractor of the foreign `SteelJoint` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct En1993JointDeltaKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1993_model::SteelJoint> for En1993JointDeltaKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1993_model::SteelJoint) -> String {
        row.id.clone()
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `SteelJoint` list.
    pub En1993JointDelta { removal: En1993JointRemoval, insertion: En1993JointInsertion, relocation: En1993JointRelocation, modification: En1993JointModification, row: semio_s_artifact_norm_en1993_model::SteelJoint, patch: En1993JointPatch, list: Vec<semio_s_artifact_norm_en1993_model::SteelJoint>, key: String = by En1993JointDeltaKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `FatigueDetail`.
    pub En1993FatigueDetailPatch of semio_s_artifact_norm_en1993_model::FatigueDetail { set { member_id: String, category: u8, method: semio_s_artifact_norm_en1993_model::FatigueMethod, spectrum: Vec<semio_s_artifact_norm_en1993_model::FatigueBand> , stress_history: String, consequence: semio_s_artifact_norm_en1993_model::FatigueConsequence} }
}

/// 🔑️ Key extractor of the foreign `FatigueDetail` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct En1993FatigueDetailDeltaKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1993_model::FatigueDetail> for En1993FatigueDetailDeltaKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1993_model::FatigueDetail) -> String {
        row.id.clone()
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `FatigueDetail` list.
    pub En1993FatigueDetailDelta { removal: En1993FatigueDetailRemoval, insertion: En1993FatigueDetailInsertion, relocation: En1993FatigueDetailRelocation, modification: En1993FatigueDetailModification, row: semio_s_artifact_norm_en1993_model::FatigueDetail, patch: En1993FatigueDetailPatch, list: Vec<semio_s_artifact_norm_en1993_model::FatigueDetail>, key: String = by En1993FatigueDetailDeltaKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `FireExposure`.
    pub En1993FireExposurePatch of semio_s_artifact_norm_en1993_model::FireExposure { set { member_id: String, rating: semio_s_artifact_norm_en1993_model::FireRating, protection_thickness: f64, section_factor: f64, mu0: f64, design_temperature: f64, protection_conductivity: f64, protection_density: f64, protection_specific_heat: f64 } }
}

/// 🔑️ Key extractor of the foreign `FireExposure` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct En1993FireExposureDeltaKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1993_model::FireExposure> for En1993FireExposureDeltaKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1993_model::FireExposure) -> String {
        row.id.clone()
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `FireExposure` list.
    pub En1993FireExposureDelta { removal: En1993FireExposureRemoval, insertion: En1993FireExposureInsertion, relocation: En1993FireExposureRelocation, modification: En1993FireExposureModification, row: semio_s_artifact_norm_en1993_model::FireExposure, patch: En1993FireExposurePatch, list: Vec<semio_s_artifact_norm_en1993_model::FireExposure>, key: String = by En1993FireExposureDeltaKey }
}

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
    /// 🩹 Sparse field patch of one `BridgeFatigue`.
    pub En1993BridgeFatiguePatch of semio_s_artifact_norm_en1993_model::BridgeFatigue { set { member_id: String, lambda: f64, phi2: f64, delta_sigma_p: f64, category: u8, method: semio_s_artifact_norm_en1993_model::FatigueMethod , heavy_vehicle_weight: f64, reference_weight: f64, observed_traffic: f64, design_life: f64, second_lane_traffic: f64, second_lane_weight: f64, lambda_1: f64, lambda_max: f64, detail_id: String, plate_thickness: f64, deck_thickness: f64, trough_spacing: f64, trough_width: f64, trough_thickness: f64, crossbeam_spacing: f64, surfacing_thickness: f64, wheel_load: f64, wheel_patch: f64, consequence: semio_s_artifact_norm_en1993_model::FatigueConsequence} }
}

/// 🔑️ Key extractor of the foreign `BridgeFatigue` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct En1993BridgeFatigueDeltaKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1993_model::BridgeFatigue> for En1993BridgeFatigueDeltaKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1993_model::BridgeFatigue) -> String {
        row.id.clone()
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `BridgeFatigue` list.
    pub En1993BridgeFatigueDelta { removal: En1993BridgeFatigueRemoval, insertion: En1993BridgeFatigueInsertion, relocation: En1993BridgeFatigueRelocation, modification: En1993BridgeFatigueModification, row: semio_s_artifact_norm_en1993_model::BridgeFatigue, patch: En1993BridgeFatiguePatch, list: Vec<semio_s_artifact_norm_en1993_model::BridgeFatigue>, key: String = by En1993BridgeFatigueDeltaKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `TowerLeg`.
    pub En1993TowerLegPatch of semio_s_artifact_norm_en1993_model::TowerLeg { set { member_id: String, force_coefficient: f64, dynamic_factor: f64, actions: Vec<semio_s_artifact_norm_en1993_model::ForceAction> , lattice_kind: semio_s_artifact_norm_en1993_model::LatticeKind, slenderness: f64, diameter: f64, height: f64, bending_stiffness: f64, mass_per_length: f64, strouhal: f64, log_decrement: f64, air_density: f64, mean_wind_speed: f64, crossing_rate: f64, design_life: f64, k_factor: f64, kw_factor: f64, lateral_force_coefficient: f64, permitted_amplitude_ratio: f64, ring_mode: f64, shell_thickness: f64, guy_area: f64, guy_angle: f64, guy_length: f64} }
}

/// 🔑️ Key extractor of the foreign `TowerLeg` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct En1993TowerLegDeltaKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1993_model::TowerLeg> for En1993TowerLegDeltaKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1993_model::TowerLeg) -> String {
        row.id.clone()
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `TowerLeg` list.
    pub En1993TowerLegDelta { removal: En1993TowerLegRemoval, insertion: En1993TowerLegInsertion, relocation: En1993TowerLegRelocation, modification: En1993TowerLegModification, row: semio_s_artifact_norm_en1993_model::TowerLeg, patch: En1993TowerLegPatch, list: Vec<semio_s_artifact_norm_en1993_model::TowerLeg>, key: String = by En1993TowerLegDeltaKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `SteelPile`.
    pub En1993PilePatch of semio_s_artifact_norm_en1993_model::SteelPile { set { section_id: String, material_id: String, driving_stress: f64, embedded_length: f64, shaft_perimeter: f64, actions: Vec<semio_s_artifact_norm_en1993_model::ForceAction> , environment: semio_s_artifact_norm_en1993_model::SheetPileEnvironment, design_life: f64, wall_thickness: f64, interlock_bending_factor: f64, interlock_stiffness_factor: f64, section_modulus: f64, shear_area: f64, design_moment: f64, design_shear: f64, interlock_shear: f64} }
}

/// 🔑️ Key extractor of the foreign `SteelPile` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct En1993PileDeltaKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1993_model::SteelPile> for En1993PileDeltaKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1993_model::SteelPile) -> String {
        row.id.clone()
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `SteelPile` list.
    pub En1993PileDelta { removal: En1993PileRemoval, insertion: En1993PileInsertion, relocation: En1993PileRelocation, modification: En1993PileModification, row: semio_s_artifact_norm_en1993_model::SteelPile, patch: En1993PilePatch, list: Vec<semio_s_artifact_norm_en1993_model::SteelPile>, key: String = by En1993PileDeltaKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `CraneRunway`.
    pub En1993CraneRunwayPatch of semio_s_artifact_norm_en1993_model::CraneRunway { set { member_id: String, wheel_contact_length: f64, dispersion: f64, web_thickness: f64, fy: f64, phi: f64, actions: Vec<semio_s_artifact_norm_en1993_model::ForceAction> , max_wheel_load: f64, load_factor: f64, rail_inertia: f64, rail_fixed: u32, flange_effective_width: f64, flange_thickness: f64, wheel_eccentricity: f64, torsion_constant: f64, longitudinal_stress: f64, rail_radius: f64, rail_contact_width: f64, crane_nu: f64, crane_kq: f64, damage_exponent: f64, stress_range: f64, detail_category_stress: f64, fatigue_phi: f64} }
}

/// 🔑️ Key extractor of the foreign `CraneRunway` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct En1993CraneRunwayDeltaKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1993_model::CraneRunway> for En1993CraneRunwayDeltaKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1993_model::CraneRunway) -> String {
        row.id.clone()
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `CraneRunway` list.
    pub En1993CraneRunwayDelta { removal: En1993CraneRunwayRemoval, insertion: En1993CraneRunwayInsertion, relocation: En1993CraneRunwayRelocation, modification: En1993CraneRunwayModification, row: semio_s_artifact_norm_en1993_model::CraneRunway, patch: En1993CraneRunwayPatch, list: Vec<semio_s_artifact_norm_en1993_model::CraneRunway>, key: String = by En1993CraneRunwayDeltaKey }
}
protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `Storey`.
    pub En1993StoreyPatch of semio_s_artifact_norm_en1993_model::Storey { set { member_id: String, label: String, height: f64, vertical_load: f64, horizontal_load: f64, drift: f64, columns: u32, plastic_analysis: u32, building_height: f64, bracing_load: f64, bracing_length: f64, bracing_deflection: f64} }
}

/// 🔑️ Key extractor of the foreign `Storey` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct En1993StoreyDeltaKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1993_model::Storey> for En1993StoreyDeltaKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1993_model::Storey) -> String {
        row.id.clone()
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `Storey` list.
    pub En1993StoreyDelta { removal: En1993StoreyRemoval, insertion: En1993StoreyInsertion, relocation: En1993StoreyRelocation, modification: En1993StoreyModification, row: semio_s_artifact_norm_en1993_model::Storey, patch: En1993StoreyPatch, list: Vec<semio_s_artifact_norm_en1993_model::Storey>, key: String = by En1993StoreyDeltaKey }
}
protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `Restraint`.
    pub En1993RestraintPatch of semio_s_artifact_norm_en1993_model::Restraint { set { member_id: String, label: String, start_support: semio_s_artifact_norm_en1993_model::RestraintSupport, end_support: semio_s_artifact_norm_en1993_model::RestraintSupport, intermediate_position: f64, torsional_spring: f64, lateral_spring_position: f64, lateral_spring_stiffness: f64, load_height: f64, load_distribution: semio_s_artifact_norm_en1993_model::RestraintDistribution, design_moment: f64, design_axial: f64, general_method: u32, hinge_to_restraint: f64} }
}

/// 🔑️ Key extractor of the foreign `Restraint` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct En1993RestraintDeltaKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1993_model::Restraint> for En1993RestraintDeltaKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1993_model::Restraint) -> String {
        row.id.clone()
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `Restraint` list.
    pub En1993RestraintDelta { removal: En1993RestraintRemoval, insertion: En1993RestraintInsertion, relocation: En1993RestraintRelocation, modification: En1993RestraintModification, row: semio_s_artifact_norm_en1993_model::Restraint, patch: En1993RestraintPatch, list: Vec<semio_s_artifact_norm_en1993_model::Restraint>, key: String = by En1993RestraintDeltaKey }
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
protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `WeldGroup`.
    pub En1993WeldGroupPatch of semio_s_artifact_norm_en1993_model::WeldGroup { set { member_id: String, label: String, pattern: semio_s_artifact_norm_en1993_model::WeldPattern, throat: f64, width: f64, depth: f64, shear_force: f64, eccentricity: f64, steel_fu: f64, beta_w: f64} }
}

/// 🔑️ Key extractor of the foreign `WeldGroup` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct En1993WeldGroupDeltaKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1993_model::WeldGroup> for En1993WeldGroupDeltaKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1993_model::WeldGroup) -> String {
        row.id.clone()
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `WeldGroup` list.
    pub En1993WeldGroupDelta { removal: En1993WeldGroupRemoval, insertion: En1993WeldGroupInsertion, relocation: En1993WeldGroupRelocation, modification: En1993WeldGroupModification, row: semio_s_artifact_norm_en1993_model::WeldGroup, patch: En1993WeldGroupPatch, list: Vec<semio_s_artifact_norm_en1993_model::WeldGroup>, key: String = by En1993WeldGroupDeltaKey }
}
protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `ParametricSection`.
    pub En1993ParametricSectionPatch of semio_s_artifact_norm_en1993_model::ParametricSection { set { section_id: String, label: String, shape: semio_s_artifact_norm_en1993_model::ParametricShape, height: f64, width: f64, web_thickness: f64, flange_thickness: f64, tolerance: f64} }
}

/// 🔑️ Key extractor of the foreign `ParametricSection` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct En1993ParametricSectionDeltaKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1993_model::ParametricSection> for En1993ParametricSectionDeltaKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1993_model::ParametricSection) -> String {
        row.id.clone()
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `ParametricSection` list.
    pub En1993ParametricSectionDelta { removal: En1993ParametricSectionRemoval, insertion: En1993ParametricSectionInsertion, relocation: En1993ParametricSectionRelocation, modification: En1993ParametricSectionModification, row: semio_s_artifact_norm_en1993_model::ParametricSection, patch: En1993ParametricSectionPatch, list: Vec<semio_s_artifact_norm_en1993_model::ParametricSection>, key: String = by En1993ParametricSectionDeltaKey }
}
protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `HollowJoint`.
    pub En1993HollowJointPatch of semio_s_artifact_norm_en1993_model::HollowJoint { set { member_id: String, label: String, chord_width: f64, chord_height: f64, chord_thickness: f64, chord_fy: f64, chord_stress: f64, brace1_width: f64, brace1_height: f64, brace1_thickness: f64, brace1_angle: f64, brace1_force: f64, brace2_width: f64, brace2_height: f64, brace2_thickness: f64, brace2_angle: f64, brace2_force: f64, gap: f64, brace_fy: f64} }
}

/// 🔑️ Key extractor of the foreign `HollowJoint` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct En1993HollowJointDeltaKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1993_model::HollowJoint> for En1993HollowJointDeltaKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1993_model::HollowJoint) -> String {
        row.id.clone()
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `HollowJoint` list.
    pub En1993HollowJointDelta { removal: En1993HollowJointRemoval, insertion: En1993HollowJointInsertion, relocation: En1993HollowJointRelocation, modification: En1993HollowJointModification, row: semio_s_artifact_norm_en1993_model::HollowJoint, patch: En1993HollowJointPatch, list: Vec<semio_s_artifact_norm_en1993_model::HollowJoint>, key: String = by En1993HollowJointDeltaKey }
}
protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `JointRow`.
    pub En1993JointRowPatch of semio_s_artifact_norm_en1993_model::JointRow { set { joint_id: String, label: String, row_type: semio_s_artifact_norm_en1993_model::JointRowType, lever: f64, bolt_to_web: f64, edge_distance: f64, extension_to_flange: f64, extension_edge: f64, alpha_factor: f64, pitch_to_previous: f64, bolts: u32, bolt_diameter: f64, bolt_stress_area: f64, grip_length: f64, bolt_class: String, group_with_previous: u32} }
}

/// 🔑️ Key extractor of the foreign `JointRow` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct En1993JointRowDeltaKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1993_model::JointRow> for En1993JointRowDeltaKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1993_model::JointRow) -> String {
        row.id.clone()
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `JointRow` list.
    pub En1993JointRowDelta { removal: En1993JointRowRemoval, insertion: En1993JointRowInsertion, relocation: En1993JointRowRelocation, modification: En1993JointRowModification, row: semio_s_artifact_norm_en1993_model::JointRow, patch: En1993JointRowPatch, list: Vec<semio_s_artifact_norm_en1993_model::JointRow>, key: String = by En1993JointRowDeltaKey }
}
protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `FrameNode`.
    pub En1993FrameNodePatch of semio_s_artifact_norm_en1993_model::FrameNode { set { label: String, x: f64, z: f64, support: semio_s_artifact_norm_en1993_model::FrameNodeSupport, load_x: f64, load_z: f64} }
}

/// 🔑️ Key extractor of the foreign `FrameNode` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct En1993FrameNodeDeltaKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1993_model::FrameNode> for En1993FrameNodeDeltaKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1993_model::FrameNode) -> String {
        row.id.clone()
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `FrameNode` list.
    pub En1993FrameNodeDelta { removal: En1993FrameNodeRemoval, insertion: En1993FrameNodeInsertion, relocation: En1993FrameNodeRelocation, modification: En1993FrameNodeModification, row: semio_s_artifact_norm_en1993_model::FrameNode, patch: En1993FrameNodePatch, list: Vec<semio_s_artifact_norm_en1993_model::FrameNode>, key: String = by En1993FrameNodeDeltaKey }
}
protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `FrameBar`.
    pub En1993FrameBarPatch of semio_s_artifact_norm_en1993_model::FrameBar { set { member_id: String, label: String, start_node_id: String, end_node_id: String, release_start: semio_s_artifact_norm_en1993_model::FrameBarRelease, release_end: semio_s_artifact_norm_en1993_model::FrameBarRelease} }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `Placement`.
    pub En1993PlacementPatch of semio_s_artifact_norm_en1993_model::Placement { set { x: f64, y: f64, z: f64, yaw: f64 } }
}

/// 🔑️ Key extractor of the foreign `FrameBar` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct En1993FrameBarDeltaKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1993_model::FrameBar> for En1993FrameBarDeltaKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1993_model::FrameBar) -> String {
        row.id.clone()
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `FrameBar` list.
    pub En1993FrameBarDelta { removal: En1993FrameBarRemoval, insertion: En1993FrameBarInsertion, relocation: En1993FrameBarRelocation, modification: En1993FrameBarModification, row: semio_s_artifact_norm_en1993_model::FrameBar, patch: En1993FrameBarPatch, list: Vec<semio_s_artifact_norm_en1993_model::FrameBar>, key: String = by En1993FrameBarDeltaKey }
}
protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `JointPlate`.
    pub En1993JointPlatePatch of semio_s_artifact_norm_en1993_model::JointPlate { set { joint_id: String, label: String, role: String, plate_height: f64, plate_width: f64, plate_thickness: f64, plate_count: u32, plate_fy: f64, plate_fu: f64, bolt_class: String, bolt_diameter: f64, hole_diameter: f64, bolt_rows: u32, bolts_per_row: u32, pitch: f64, gauge: f64, end_distance: f64, edge_distance: f64, shear_planes: u32, eccentricity: f64, weld_throat: f64, weld_fu: f64, weld_beta_w: f64, group_method: semio_s_artifact_norm_en1993_model::BoltGroupMethod} }
}

/// 🔑️ Key extractor of the foreign rows of `En1993JointPlateDelta` (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct En1993JointPlateDeltaKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1993_model::JointPlate> for En1993JointPlateDeltaKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1993_model::JointPlate) -> String {
        row.id.clone()
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `JointPlate` list.
    pub En1993JointPlateDelta { removal: En1993JointPlateRemoval, insertion: En1993JointPlateInsertion, relocation: En1993JointPlateRelocation, modification: En1993JointPlateModification, row: semio_s_artifact_norm_en1993_model::JointPlate, patch: En1993JointPlatePatch, list: Vec<semio_s_artifact_norm_en1993_model::JointPlate>, key: String = by En1993JointPlateDeltaKey }
}
protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `FoundationBlock`.
    pub En1993FoundationBlockPatch of semio_s_artifact_norm_en1993_model::FoundationBlock { set { joint_id: String, label: String, block_length: f64, block_width: f64, block_depth: f64, concrete_fck: f64, grout_fck: f64, grout_thickness: f64, anchor_count: u32, anchor_class: String, anchor_diameter: f64, anchor_stress_area: f64, anchor_embedment: f64, anchor_edge_distance: f64} }
}

/// 🔑️ Key extractor of the foreign rows of `En1993FoundationBlockDelta` (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct En1993FoundationBlockDeltaKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1993_model::FoundationBlock> for En1993FoundationBlockDeltaKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1993_model::FoundationBlock) -> String {
        row.id.clone()
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `FoundationBlock` list.
    pub En1993FoundationBlockDelta { removal: En1993FoundationBlockRemoval, insertion: En1993FoundationBlockInsertion, relocation: En1993FoundationBlockRelocation, modification: En1993FoundationBlockModification, row: semio_s_artifact_norm_en1993_model::FoundationBlock, patch: En1993FoundationBlockPatch, list: Vec<semio_s_artifact_norm_en1993_model::FoundationBlock>, key: String = by En1993FoundationBlockDeltaKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `MaterialDuctility`.
    pub En1993MaterialDuctilityPatch of semio_s_artifact_norm_en1993_model::MaterialDuctility { set { material_id: String, elongation: f64, ultimate_strain: f64 } }
}

/// 🔑️ Key extractor of the foreign rows of `En1993MaterialDuctilityDelta` (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct En1993MaterialDuctilityDeltaKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1993_model::MaterialDuctility> for En1993MaterialDuctilityDeltaKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1993_model::MaterialDuctility) -> String {
        row.id.clone()
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `MaterialDuctility` list.
    pub En1993MaterialDuctilityDelta { removal: En1993MaterialDuctilityRemoval, insertion: En1993MaterialDuctilityInsertion, relocation: En1993MaterialDuctilityRelocation, modification: En1993MaterialDuctilityModification, row: semio_s_artifact_norm_en1993_model::MaterialDuctility, patch: En1993MaterialDuctilityPatch, list: Vec<semio_s_artifact_norm_en1993_model::MaterialDuctility>, key: String = by En1993MaterialDuctilityDeltaKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `FormingGain`.
    pub En1993FormingGainPatch of semio_s_artifact_norm_en1993_model::FormingGain { set { member_id: String, forming: semio_s_artifact_norm_en1993_model::FormingProcess, ultimate_strength: f64, gross_area: f64, bends: u32 , used_yield: f64} }
}

/// 🔑️ Key extractor of the foreign rows of `En1993FormingGainDelta` (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct En1993FormingGainDeltaKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1993_model::FormingGain> for En1993FormingGainDeltaKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1993_model::FormingGain) -> String {
        row.id.clone()
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `FormingGain` list.
    pub En1993FormingGainDelta { removal: En1993FormingGainRemoval, insertion: En1993FormingGainInsertion, relocation: En1993FormingGainRelocation, modification: En1993FormingGainModification, row: semio_s_artifact_norm_en1993_model::FormingGain, patch: En1993FormingGainPatch, list: Vec<semio_s_artifact_norm_en1993_model::FormingGain>, key: String = by En1993FormingGainDeltaKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `BucklingLength`.
    pub En1993BucklingLengthPatch of semio_s_artifact_norm_en1993_model::BucklingLength { set { member_id: String, plane: semio_s_artifact_norm_en1993_model::BucklingPlane, sway: semio_s_artifact_norm_en1993_model::FrameSway, restraint_a: semio_s_artifact_norm_en1993_model::NodeARestraint, column_stiffness_a: f64, beam_stiffness_a: f64, restraint_b: semio_s_artifact_norm_en1993_model::NodeBRestraint, column_stiffness_b: f64, beam_stiffness_b: f64 } }
}

/// 🔑️ Key extractor of the foreign rows of `En1993BucklingLengthDelta` (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct En1993BucklingLengthDeltaKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1993_model::BucklingLength> for En1993BucklingLengthDeltaKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1993_model::BucklingLength) -> String {
        row.id.clone()
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `BucklingLength` list.
    pub En1993BucklingLengthDelta { removal: En1993BucklingLengthRemoval, insertion: En1993BucklingLengthInsertion, relocation: En1993BucklingLengthRelocation, modification: En1993BucklingLengthModification, row: semio_s_artifact_norm_en1993_model::BucklingLength, patch: En1993BucklingLengthPatch, list: Vec<semio_s_artifact_norm_en1993_model::BucklingLength>, key: String = by En1993BucklingLengthDeltaKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `FatigueStress`.
    pub En1993FatigueStressPatch of semio_s_artifact_norm_en1993_model::FatigueStress { set { detail_id: String, max_case_id: String, min_case_id: String, fibre_z: f64, fibre_y: f64, relief: semio_s_artifact_norm_en1993_model::StressRelief, concentration: f64, thickness: f64, cycles: f64 } }
}

/// 🔑️ Key extractor of the foreign rows of `En1993FatigueStressDelta` (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct En1993FatigueStressDeltaKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1993_model::FatigueStress> for En1993FatigueStressDeltaKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1993_model::FatigueStress) -> String {
        row.id.clone()
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `FatigueStress` list.
    pub En1993FatigueStressDelta { removal: En1993FatigueStressRemoval, insertion: En1993FatigueStressInsertion, relocation: En1993FatigueStressRelocation, modification: En1993FatigueStressModification, row: semio_s_artifact_norm_en1993_model::FatigueStress, patch: En1993FatigueStressPatch, list: Vec<semio_s_artifact_norm_en1993_model::FatigueStress>, key: String = by En1993FatigueStressDeltaKey }
}
//#endregion 🔖️Rows

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `JointPlate` list.
    pub En1993JointPlateDelta { removal: En1993JointPlateRemoval, insertion: En1993JointPlateInsertion, relocation: En1993JointPlateRelocation, modification: En1993JointPlateModification, row: semio_s_artifact_norm_en1993_model::JointPlate, patch: En1993JointPlatePatch, list: Vec<semio_s_artifact_norm_en1993_model::JointPlate>, key: String = by En1993JointPlateDeltaKey }
}
protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `FrameMemberBasis`.
    pub En1993FrameMemberBasisPatch of semio_s_artifact_norm_en1993_model::FrameMemberBasis { set { member_id: String, basis: semio_s_artifact_norm_en1993_model::MemberForceBasis } }
}

/// 🔑️ Key extractor of the foreign rows of `En1993FrameMemberBasisDelta` (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct En1993FrameMemberBasisDeltaKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1993_model::FrameMemberBasis> for En1993FrameMemberBasisDeltaKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1993_model::FrameMemberBasis) -> String {
        row.id.clone()
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `FrameMemberBasis` list.
    pub En1993FrameMemberBasisDelta { removal: En1993FrameMemberBasisRemoval, insertion: En1993FrameMemberBasisInsertion, relocation: En1993FrameMemberBasisRelocation, modification: En1993FrameMemberBasisModification, row: semio_s_artifact_norm_en1993_model::FrameMemberBasis, patch: En1993FrameMemberBasisPatch, list: Vec<semio_s_artifact_norm_en1993_model::FrameMemberBasis>, key: String = by En1993FrameMemberBasisDeltaKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `FastenedJoint`.
    pub En1993FastenedJointPatch of semio_s_artifact_norm_en1993_model::FastenedJoint { set { member_id: String, kind: semio_s_artifact_norm_en1993_model::FastenerKind, diameter: f64, washer_diameter: f64, count: u32, per_row: u32, head_thickness: f64, base_thickness: f64, head_strength: f64, base_strength: f64, end_distance: f64, edge_distance: f64, pitch: f64, gauge: f64, sheet_width: f64, shear_force: f64, tension_force: f64, shear_resistance: f64 } }
}

/// 🔑️ Key extractor of the foreign rows of `En1993FastenedJointDelta` (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct En1993FastenedJointDeltaKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1993_model::FastenedJoint> for En1993FastenedJointDeltaKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1993_model::FastenedJoint) -> String {
        row.id.clone()
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `FastenedJoint` list.
    pub En1993FastenedJointDelta { removal: En1993FastenedJointRemoval, insertion: En1993FastenedJointInsertion, relocation: En1993FastenedJointRelocation, modification: En1993FastenedJointModification, row: semio_s_artifact_norm_en1993_model::FastenedJoint, patch: En1993FastenedJointPatch, list: Vec<semio_s_artifact_norm_en1993_model::FastenedJoint>, key: String = by En1993FastenedJointDeltaKey }
}
//#endregion 🔖️Rows

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `JointPlate` list.
    pub En1993JointPlateDelta { removal: En1993JointPlateRemoval, insertion: En1993JointPlateInsertion, relocation: En1993JointPlateRelocation, modification: En1993JointPlateModification, row: semio_s_artifact_norm_en1993_model::JointPlate, patch: En1993JointPlatePatch, list: Vec<semio_s_artifact_norm_en1993_model::JointPlate>, key: String = by En1993JointPlateDeltaKey }
}
protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `FrameAction`.
    pub En1993FrameActionPatch of semio_s_artifact_norm_en1993_model::FrameAction { set { load_case_id: String, target_id: String, kind: semio_s_artifact_norm_en1993_model::FrameActionKind, fx: f64, fz: f64, my: f64, start_ratio: f64, end_ratio: f64 } }
}

/// 🔑️ Key extractor of the foreign rows of `En1993FrameActionDelta` (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct En1993FrameActionDeltaKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1993_model::FrameAction> for En1993FrameActionDeltaKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1993_model::FrameAction) -> String {
        row.id.clone()
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `FrameAction` list.
    pub En1993FrameActionDelta { removal: En1993FrameActionRemoval, insertion: En1993FrameActionInsertion, relocation: En1993FrameActionRelocation, modification: En1993FrameActionModification, row: semio_s_artifact_norm_en1993_model::FrameAction, patch: En1993FrameActionPatch, list: Vec<semio_s_artifact_norm_en1993_model::FrameAction>, key: String = by En1993FrameActionDeltaKey }
}
//#endregion 🔖️Rows

/// 🔑️ Key extractor of the foreign `Placement` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct En1993PlacementDeltaKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1993_model::Placement> for En1993PlacementDeltaKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1993_model::Placement) -> String {
        row.id.clone()
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `Placement` list.
    pub En1993PlacementDelta { removal: En1993PlacementRemoval, insertion: En1993PlacementInsertion, relocation: En1993PlacementRelocation, modification: En1993PlacementModification, row: semio_s_artifact_norm_en1993_model::Placement, patch: En1993PlacementPatch, list: Vec<semio_s_artifact_norm_en1993_model::Placement>, key: String = by En1993PlacementDeltaKey }
}

//#region 🔖️Diff
/// 🔺️ Sparse delta for the En1993 artifact: the scalar fields a mutation sets and the keyed row deltas of its lists.
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.norm.en1993")]
pub struct En1993Diff {
    #[state(artifact)]
    pub annex: Option<semio_s_plugin_norm_compliance::AnnexChoice>,
    #[state(artifact)]
    pub materials: En1993MaterialDelta,
    #[state(artifact)]
    pub sections: En1993SectionDelta,
    #[state(artifact)]
    pub members: En1993MemberDelta,
    #[state(artifact)]
    pub load_cases: En1993LoadCaseDelta,
    #[state(artifact)]
    pub member_actions: En1993MemberActionDelta,
    #[state(artifact)]
    pub joints: En1993JointDelta,
    #[state(artifact)]
    pub fatigue_details: En1993FatigueDetailDelta,
    #[state(artifact)]
    pub fire_exposures: En1993FireExposureDelta,
    #[state(artifact)]
    pub cold_formed_members: En1993ColdFormedMemberDelta,
    #[state(artifact)]
    pub plated_panels: En1993PlatedPanelDelta,
    #[state(artifact)]
    pub silo_shells: En1993SiloShellDelta,
    #[state(artifact)]
    pub tension_components: En1993TensionComponentDelta,
    #[state(artifact)]
    pub bridge_fatigue: En1993BridgeFatigueDelta,
    #[state(artifact)]
    pub tower_legs: En1993TowerLegDelta,
    #[state(artifact)]
    pub piles: En1993PileDelta,
    #[state(artifact)]
    pub crane_runways: En1993CraneRunwayDelta,
    #[state(artifact)]
    pub storeys: En1993StoreyDelta,
    #[state(artifact)]
    pub restraints: En1993RestraintDelta,
    #[state(artifact)]
    pub built_up_members: En1993BuiltUpMemberDelta,
    #[state(artifact)]
    pub weld_groups: En1993WeldGroupDelta,
    #[state(artifact)]
    pub parametric_sections: En1993ParametricSectionDelta,
    #[state(artifact)]
    pub hollow_joints: En1993HollowJointDelta,
    #[state(artifact)]
    pub joint_rows: En1993JointRowDelta,
    #[state(artifact)]
    pub frame_nodes: En1993FrameNodeDelta,
    #[state(artifact)]
    pub frame_bars: En1993FrameBarDelta,
    #[state(artifact)]
    pub placements: En1993PlacementDelta,
    #[state(artifact)]
    pub reliability_class: Option<semio_s_plugin_norm_compliance::ReliabilityClass>,
    #[state(artifact)]
    pub joint_plates: En1993JointPlateDelta,
    #[state(artifact)]
    pub foundation_blocks: En1993FoundationBlockDelta,
    #[state(artifact)]
    pub material_ductility: En1993MaterialDuctilityDelta,
    #[state(artifact)]
    pub forming_gains: En1993FormingGainDelta,
    #[state(artifact)]
    pub buckling_lengths: En1993BucklingLengthDelta,
    #[state(artifact)]
    pub fatigue_stresses: En1993FatigueStressDelta,
    #[state(artifact)]
    pub frame_actions: En1993FrameActionDelta,
    #[state(artifact)]
    pub frame_member_bases: En1993FrameMemberBasisDelta,
    #[state(artifact)]
    pub fastened_joints: En1993FastenedJointDelta,
}
//#endregion 🔖️Diff

impl MutationDiff<En1993Snapshot> for En1993Diff {}

impl protocol::ColdMutationDiff<En1993Snapshot> for En1993Diff {
    fn absorb(&mut self, other: Self) {
        if other.annex.is_some() {
            self.annex = other.annex;
        }
        if other.reliability_class.is_some() {
            self.reliability_class = other.reliability_class;
        }
        self.materials.absorb(other.materials);
        self.sections.absorb(other.sections);
        self.members.absorb(other.members);
        self.load_cases.absorb(other.load_cases);
        self.member_actions.absorb(other.member_actions);
        self.joints.absorb(other.joints);
        self.fatigue_details.absorb(other.fatigue_details);
        self.fire_exposures.absorb(other.fire_exposures);
        self.cold_formed_members.absorb(other.cold_formed_members);
        self.plated_panels.absorb(other.plated_panels);
        self.silo_shells.absorb(other.silo_shells);
        self.tension_components.absorb(other.tension_components);
        self.bridge_fatigue.absorb(other.bridge_fatigue);
        self.tower_legs.absorb(other.tower_legs);
        self.piles.absorb(other.piles);
        self.crane_runways.absorb(other.crane_runways);
        self.storeys.absorb(other.storeys);
        self.restraints.absorb(other.restraints);
        self.built_up_members.absorb(other.built_up_members);
        self.weld_groups.absorb(other.weld_groups);
        self.parametric_sections.absorb(other.parametric_sections);
        self.hollow_joints.absorb(other.hollow_joints);
        self.joint_rows.absorb(other.joint_rows);
        self.frame_nodes.absorb(other.frame_nodes);
        self.frame_bars.absorb(other.frame_bars);
        self.placements.absorb(other.placements);
        self.joint_plates.absorb(other.joint_plates);
        self.foundation_blocks.absorb(other.foundation_blocks);
        self.material_ductility.absorb(other.material_ductility);
        self.forming_gains.absorb(other.forming_gains);
        self.buckling_lengths.absorb(other.buckling_lengths);
        self.fatigue_stresses.absorb(other.fatigue_stresses);
        self.frame_member_bases.absorb(other.frame_member_bases);
        self.fastened_joints.absorb(other.fastened_joints);
        self.frame_actions.absorb(other.frame_actions);
    }

    fn apply(&self, unpublished: &mut En1993Snapshot, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        let base = &*unpublished;
        let mut next = base.clone();
        if let Some(value) = self.annex {
            next.annex = value;
        }
        if let Some(value) = self.reliability_class {
            next.reliability_class = value;
        }
        next.materials = self.materials.commit_onto(&base.materials, capability).map_err(|error| error.under(["materials"]))?;
        next.sections = self.sections.commit_onto(&base.sections, capability).map_err(|error| error.under(["sections"]))?;
        next.members = self.members.commit_onto(&base.members, capability).map_err(|error| error.under(["members"]))?;
        next.load_cases = self.load_cases.commit_onto(&base.load_cases, capability).map_err(|error| error.under(["loadCases"]))?;
        next.member_actions = self.member_actions.commit_onto(&base.member_actions, capability).map_err(|error| error.under(["memberActions"]))?;
        next.joints = self.joints.commit_onto(&base.joints, capability).map_err(|error| error.under(["joints"]))?;
        next.fatigue_details = self.fatigue_details.commit_onto(&base.fatigue_details, capability).map_err(|error| error.under(["fatigueDetails"]))?;
        next.fire_exposures = self.fire_exposures.commit_onto(&base.fire_exposures, capability).map_err(|error| error.under(["fireExposures"]))?;
        next.cold_formed_members = self.cold_formed_members.commit_onto(&base.cold_formed_members, capability).map_err(|error| error.under(["coldFormedMembers"]))?;
        next.plated_panels = self.plated_panels.commit_onto(&base.plated_panels, capability).map_err(|error| error.under(["platedPanels"]))?;
        next.silo_shells = self.silo_shells.commit_onto(&base.silo_shells, capability).map_err(|error| error.under(["siloShells"]))?;
        next.tension_components = self.tension_components.commit_onto(&base.tension_components, capability).map_err(|error| error.under(["tensionComponents"]))?;
        next.bridge_fatigue = self.bridge_fatigue.commit_onto(&base.bridge_fatigue, capability).map_err(|error| error.under(["bridgeFatigue"]))?;
        next.tower_legs = self.tower_legs.commit_onto(&base.tower_legs, capability).map_err(|error| error.under(["towerLegs"]))?;
        next.piles = self.piles.commit_onto(&base.piles, capability).map_err(|error| error.under(["piles"]))?;
        next.crane_runways = self.crane_runways.commit_onto(&base.crane_runways, capability).map_err(|error| error.under(["craneRunways"]))?;
        next.storeys = self.storeys.commit_onto(&base.storeys, capability).map_err(|error| error.under(["storeys"]))?;
        next.restraints = self.restraints.commit_onto(&base.restraints, capability).map_err(|error| error.under(["restraints"]))?;
        next.built_up_members = self.built_up_members.commit_onto(&base.built_up_members, capability).map_err(|error| error.under(["builtUpMembers"]))?;
        next.weld_groups = self.weld_groups.commit_onto(&base.weld_groups, capability).map_err(|error| error.under(["weldGroups"]))?;
        next.parametric_sections = self.parametric_sections.commit_onto(&base.parametric_sections, capability).map_err(|error| error.under(["parametricSections"]))?;
        next.hollow_joints = self.hollow_joints.commit_onto(&base.hollow_joints, capability).map_err(|error| error.under(["hollowJoints"]))?;
        next.joint_rows = self.joint_rows.commit_onto(&base.joint_rows, capability).map_err(|error| error.under(["jointRows"]))?;
        next.frame_nodes = self.frame_nodes.commit_onto(&base.frame_nodes, capability).map_err(|error| error.under(["frameNodes"]))?;
        next.frame_bars = self.frame_bars.commit_onto(&base.frame_bars, capability).map_err(|error| error.under(["frameBars"]))?;
        next.placements = self.placements.commit_onto(&base.placements, capability).map_err(|error| error.under(["placements"]))?;
        next.joint_plates = self.joint_plates.commit_onto(&base.joint_plates, capability).map_err(|error| error.under(["jointPlates"]))?;
        next.foundation_blocks = self.foundation_blocks.commit_onto(&base.foundation_blocks, capability).map_err(|error| error.under(["foundationBlocks"]))?;
        next.material_ductility = self.material_ductility.commit_onto(&base.material_ductility, capability).map_err(|error| error.under(["materialDuctility"]))?;
        next.forming_gains = self.forming_gains.commit_onto(&base.forming_gains, capability).map_err(|error| error.under(["formingGains"]))?;
        next.buckling_lengths = self.buckling_lengths.commit_onto(&base.buckling_lengths, capability).map_err(|error| error.under(["bucklingLengths"]))?;
        next.fatigue_stresses = self.fatigue_stresses.commit_onto(&base.fatigue_stresses, capability).map_err(|error| error.under(["fatigueStresses"]))?;
        next.frame_member_bases = self.frame_member_bases.commit_onto(&base.frame_member_bases, capability).map_err(|error| error.under(["frameMemberBases"]))?;
        next.fastened_joints = self.fastened_joints.commit_onto(&base.fastened_joints, capability).map_err(|error| error.under(["fastenedJoints"]))?;
        next.frame_actions = self.frame_actions.commit_onto(&base.frame_actions, capability).map_err(|error| error.under(["frameActions"]))?;
        *unpublished = next;
        Ok(())
    }
}

impl DiffAlgebra<En1993Snapshot> for En1993Diff {
    fn is_empty(&self) -> bool {
        self.annex.is_none()
            && self.reliability_class.is_none()
            && self.materials.is_empty()
            && self.sections.is_empty()
            && self.members.is_empty()
            && self.load_cases.is_empty()
            && self.member_actions.is_empty()
            && self.joints.is_empty()
            && self.fatigue_details.is_empty()
            && self.fire_exposures.is_empty()
            && self.cold_formed_members.is_empty()
            && self.plated_panels.is_empty()
            && self.silo_shells.is_empty()
            && self.tension_components.is_empty()
            && self.bridge_fatigue.is_empty()
            && self.tower_legs.is_empty()
            && self.piles.is_empty()
            && self.crane_runways.is_empty()
            && self.storeys.is_empty()
            && self.restraints.is_empty()
            && self.built_up_members.is_empty()
            && self.weld_groups.is_empty()
            && self.parametric_sections.is_empty()
            && self.hollow_joints.is_empty()
            && self.joint_rows.is_empty()
            && self.frame_nodes.is_empty()
            && self.frame_bars.is_empty()
            && self.placements.is_empty()
            && self.joint_plates.is_empty()
            && self.foundation_blocks.is_empty()
            && self.material_ductility.is_empty()
            && self.forming_gains.is_empty()
            && self.buckling_lengths.is_empty()
            && self.fatigue_stresses.is_empty()
            && self.frame_member_bases.is_empty()
            && self.fastened_joints.is_empty()
            && self.frame_actions.is_empty()
    }
}

impl protocol::ColdDiffAlgebra<En1993Snapshot> for En1993Diff {
    fn inverse(&self, base: &En1993Snapshot) -> Self {
        Self {
            annex: self.annex.map(|_| base.annex),
            reliability_class: self.reliability_class.map(|_| base.reliability_class),
            materials: self.materials.inverse(&base.materials),
            sections: self.sections.inverse(&base.sections),
            members: self.members.inverse(&base.members),
            load_cases: self.load_cases.inverse(&base.load_cases),
            member_actions: self.member_actions.inverse(&base.member_actions),
            joints: self.joints.inverse(&base.joints),
            fatigue_details: self.fatigue_details.inverse(&base.fatigue_details),
            fire_exposures: self.fire_exposures.inverse(&base.fire_exposures),
            cold_formed_members: self.cold_formed_members.inverse(&base.cold_formed_members),
            plated_panels: self.plated_panels.inverse(&base.plated_panels),
            silo_shells: self.silo_shells.inverse(&base.silo_shells),
            tension_components: self.tension_components.inverse(&base.tension_components),
            bridge_fatigue: self.bridge_fatigue.inverse(&base.bridge_fatigue),
            tower_legs: self.tower_legs.inverse(&base.tower_legs),
            piles: self.piles.inverse(&base.piles),
            crane_runways: self.crane_runways.inverse(&base.crane_runways),
            storeys: self.storeys.inverse(&base.storeys),
            restraints: self.restraints.inverse(&base.restraints),
            built_up_members: self.built_up_members.inverse(&base.built_up_members),
            weld_groups: self.weld_groups.inverse(&base.weld_groups),
            parametric_sections: self.parametric_sections.inverse(&base.parametric_sections),
            hollow_joints: self.hollow_joints.inverse(&base.hollow_joints),
            joint_rows: self.joint_rows.inverse(&base.joint_rows),
            frame_nodes: self.frame_nodes.inverse(&base.frame_nodes),
            frame_bars: self.frame_bars.inverse(&base.frame_bars),
            placements: self.placements.inverse(&base.placements),
            joint_plates: self.joint_plates.inverse(&base.joint_plates),
            foundation_blocks: self.foundation_blocks.inverse(&base.foundation_blocks),
            material_ductility: self.material_ductility.inverse(&base.material_ductility),
            forming_gains: self.forming_gains.inverse(&base.forming_gains),
            buckling_lengths: self.buckling_lengths.inverse(&base.buckling_lengths),
            fatigue_stresses: self.fatigue_stresses.inverse(&base.fatigue_stresses),
            frame_member_bases: self.frame_member_bases.inverse(&base.frame_member_bases),
            fastened_joints: self.fastened_joints.inverse(&base.fastened_joints),
            frame_actions: self.frame_actions.inverse(&base.frame_actions),
        }
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

//#region 🍃️FacetLeaves
/// 🍃️ The five handcrafted leaves of the `diff` schema facet.
pub const DIFF_LEAVES: semio_framework_schema_registry::FacetLeaves = semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: semio_s_artifact_norm_en1993_diff_typescript::LEAF, graphql: semio_s_artifact_norm_en1993_diff_graphql::LEAF, json_schema: semio_s_artifact_norm_en1993_diff_json_schema::LEAF, proto: semio_s_artifact_norm_en1993_diff_proto::LEAF };
//#endregion 🍃️FacetLeaves
