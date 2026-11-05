//! 🧬️ En1998 diff schema — sparse scalar fields plus keyed row deltas for every list the document owns.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use framework_schema::ArtifactSchema;
use protocol::{ColdDiffAlgebra, DiffAlgebra, MutationDiff};

use semio_s_artifact_norm_en1998_model::En1998Snapshot;

//#region 🔖️Rows
/// 🔑️ Key extractor of the foreign `En1998System` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct En1998SystemKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1998_model::En1998System> for En1998SystemKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1998_model::En1998System) -> String {
        row.id.clone()
    }
}

/// 🔑️ Key extractor of the foreign `En1998Storey` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct En1998StoreyKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1998_model::En1998Storey> for En1998StoreyKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1998_model::En1998Storey) -> String {
        row.id.clone()
    }
}

/// 🔑️ Key extractor of the foreign `En1998Member` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct En1998MemberKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1998_model::En1998Member> for En1998MemberKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1998_model::En1998Member) -> String {
        row.id.clone()
    }
}

/// 🔑️ Key extractor of the foreign `En1998Building` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct En1998BuildingKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1998_model::En1998Building> for En1998BuildingKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1998_model::En1998Building) -> String {
        row.id.clone()
    }
}

/// 🔑️ Key extractor of the foreign `En1998Bridge` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct En1998BridgeKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1998_model::En1998Bridge> for En1998BridgeKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1998_model::En1998Bridge) -> String {
        row.id.clone()
    }
}

/// 🔑️ Key extractor of the foreign `En1998Assessment` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct En1998AssessmentKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1998_model::En1998Assessment> for En1998AssessmentKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1998_model::En1998Assessment) -> String {
        row.id.clone()
    }
}

/// 🔑️ Key extractor of the foreign `En1998Silo` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct En1998SiloKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1998_model::En1998Silo> for En1998SiloKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1998_model::En1998Silo) -> String {
        row.id.clone()
    }
}

/// 🔑️ Key extractor of the foreign `En1998Tank` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct En1998TankKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1998_model::En1998Tank> for En1998TankKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1998_model::En1998Tank) -> String {
        row.id.clone()
    }
}

/// 🔑️ Key extractor of the foreign `En1998SoilLayer` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct En1998SoilLayerKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1998_model::En1998SoilLayer> for En1998SoilLayerKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1998_model::En1998SoilLayer) -> String {
        row.id.clone()
    }
}

/// 🔑️ Key extractor of the foreign `En1998Foundation` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct En1998FoundationKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1998_model::En1998Foundation> for En1998FoundationKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1998_model::En1998Foundation) -> String {
        row.id.clone()
    }
}

/// 🔑️ Key extractor of the foreign `En1998RetainingWall` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct En1998RetainingWallKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1998_model::En1998RetainingWall> for En1998RetainingWallKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1998_model::En1998RetainingWall) -> String {
        row.id.clone()
    }
}

/// 🔑️ Key extractor of the foreign `En1998Tower` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct En1998TowerKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1998_model::En1998Tower> for En1998TowerKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1998_model::En1998Tower) -> String {
        row.id.clone()
    }
}

/// 🔑️ Key extractor of the foreign `En1998Placement` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct En1998PlacementKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1998_model::En1998Placement> for En1998PlacementKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1998_model::En1998Placement) -> String {
        row.id.clone()
    }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `En1998System`.
    pub En1998SystemPatch of semio_s_artifact_norm_en1998_model::En1998System { set { direction: semio_s_artifact_norm_en1998_model::AnalysisDirection, system_type: semio_s_artifact_norm_en1998_model::SystemType, material: semio_s_artifact_norm_en1998_model::StructuralMaterial, ductility_class: semio_s_artifact_norm_en1998_model::DuctilityClass, q0: f64, alpha_u_over_alpha_1: f64, k_w: f64, base_shear_resistance_n: f64 } }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `En1998System` list.
    pub En1998SystemDelta { removal: En1998SystemRemoval, insertion: En1998SystemInsertion, relocation: En1998SystemRelocation, modification: En1998SystemModification, row: semio_s_artifact_norm_en1998_model::En1998System, patch: En1998SystemPatch, list: Vec<semio_s_artifact_norm_en1998_model::En1998System>, key: String = by En1998SystemKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `En1998Storey`.
    pub En1998StoreyPatch of semio_s_artifact_norm_en1998_model::En1998Storey { set { height_m: f64, permanent_gk_n: f64, correlated_occupancy: bool, stiffness_x: f64, stiffness_y: f64, centre_of_mass_x_m: f64, centre_of_mass_y_m: f64, centre_of_stiffness_x_m: f64, centre_of_stiffness_y_m: f64, drift_x_m: f64, drift_y_m: f64, shear_resistance_n: f64 } nest { variables: En1998VariableActionDelta } }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `En1998Storey` list.
    pub En1998StoreyDelta { removal: En1998StoreyRemoval, insertion: En1998StoreyInsertion, relocation: En1998StoreyRelocation, modification: En1998StoreyModification, row: semio_s_artifact_norm_en1998_model::En1998Storey, patch: En1998StoreyPatch, list: Vec<semio_s_artifact_norm_en1998_model::En1998Storey>, key: String = by En1998StoreyKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `En1998Member`.
    pub En1998MemberPatch of semio_s_artifact_norm_en1998_model::En1998Member { set { material: semio_s_artifact_norm_en1998_model::StructuralMaterial, role: semio_s_artifact_norm_en1998_model::MemberKind, detailing_compatible_with_q: bool, min_dimension_m: f64, rho: f64, rho_prime: f64, omega_wd: f64, steel_section_class: u8, fastener_diameter_mm: f64, timber_thickness_mm: f64, wall_thickness_m: f64, wall_effective_height_m: f64, wall_length_m: f64, wall_direction_rad: f64, wall_storey_id: String, wall_start_x_m: f64, wall_start_y_m: f64, wall_poisson_ratio: f64, effective_stiffness_basis: semio_s_artifact_norm_en1998_model::StiffnessBasis, effective_stiffness_ratio: f64 } }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `En1998Member` list.
    pub En1998MemberDelta { removal: En1998MemberRemoval, insertion: En1998MemberInsertion, relocation: En1998MemberRelocation, modification: En1998MemberModification, row: semio_s_artifact_norm_en1998_model::En1998Member, patch: En1998MemberPatch, list: Vec<semio_s_artifact_norm_en1998_model::En1998Member>, key: String = by En1998MemberKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `En1998Building`.
    pub En1998BuildingPatch of semio_s_artifact_norm_en1998_model::En1998Building { set { name: String, plan_width_m: f64, plan_length_m: f64, plan_regular: bool, elevation_regular: bool, t1_method: semio_s_artifact_norm_en1998_model::T1Method, t1_given_s: f64, ct: f64, drift_limit_class: semio_s_artifact_norm_en1998_model::DriftLimitClass, nu: f64, multiple_resisting_systems: bool, claims_simple_masonry: bool, masonry_wall_area_ratio: f64, accidental_eccentricity_ratio: f64, setback_ratio: f64, plan_envelope_gap_ratio: f64, joint_gap_m: f64, nonstructural_weight_n: f64, nonstructural_height_m: f64, nonstructural_period_s: f64, nonstructural_q: f64, nonstructural_resistance_n: f64, column_moment_sum_nm: f64, beam_moment_sum_nm: f64, overturning_base_x_m: f64, overturning_base_y_m: f64, tie_down_moment_nm: f64, storey_stiffness_basis: semio_s_artifact_norm_en1998_model::StiffnessBasis, storey_stiffness_ratio: f64 } nest { systems: En1998SystemDelta, storeys: En1998StoreyDelta, members: En1998MemberDelta } }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `En1998Building` list.
    pub En1998BuildingDelta { removal: En1998BuildingRemoval, insertion: En1998BuildingInsertion, relocation: En1998BuildingRelocation, modification: En1998BuildingModification, row: semio_s_artifact_norm_en1998_model::En1998Building, patch: En1998BuildingPatch, list: Vec<semio_s_artifact_norm_en1998_model::En1998Building>, key: String = by En1998BuildingKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `En1998Bridge`.
    pub En1998BridgePatch of semio_s_artifact_norm_en1998_model::En1998Bridge { set { period_ratio: f64, fundamental_period_s: f64, v_rd_n: f64, bearing_d_rd_m: f64, permanent_gk_n: f64, correlated_occupancy: bool, pier_height_m: f64, pier_moment_rd_nm: f64, pier_shear_rd_n: f64, deck_length_m: f64, deck_ei_nm2: f64, importance_class: f64, vertical_period_s: f64, pier_cracked_inertia_ratio: f64 } nest { variables: En1998VariableActionDelta } }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `En1998Bridge` list.
    pub En1998BridgeDelta { removal: En1998BridgeRemoval, insertion: En1998BridgeInsertion, relocation: En1998BridgeRelocation, modification: En1998BridgeModification, row: semio_s_artifact_norm_en1998_model::En1998Bridge, patch: En1998BridgePatch, list: Vec<semio_s_artifact_norm_en1998_model::En1998Bridge>, key: String = by En1998BridgeKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `En1998Assessment`.
    pub En1998AssessmentPatch of semio_s_artifact_norm_en1998_model::En1998Assessment { set { knowledge_level: semio_s_artifact_norm_en1998_model::KnowledgeLevel, limit_state: semio_s_artifact_norm_en1998_model::LimitState, supported_building_id: String, r_k_n: f64, gamma_el: f64, chord_rotation_demand_rad: f64, chord_rotation_yield_rad: f64, chord_rotation_ultimate_rad: f64, section_depth_m: f64, section_width_m: f64, effective_depth_m: f64, shear_span_m: f64, axial_force_n: f64, tension_steel_area_m2: f64, compression_steel_area_m2: f64, bar_diameter_mm: f64, steel_yield_mpa: f64, concrete_strength_mpa: f64, stirrup_leg_area_m2: f64, stirrup_spacing_m: f64, stirrup_yield_mpa: f64, core_width_m: f64, core_depth_m: f64, restrained_bar_count: f64, restrained_bar_spacing_m: f64, diagonal_reinforcement_ratio: f64, shear_cracking_before_yield: f64, shear_force_demand_n: f64 , intermediate_steel_area_m2: f64, analysis_method: semio_s_artifact_norm_en1998_model::AnalysisMethodChoice} }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `En1998Assessment` list.
    pub En1998AssessmentDelta { removal: En1998AssessmentRemoval, insertion: En1998AssessmentInsertion, relocation: En1998AssessmentRelocation, modification: En1998AssessmentModification, row: semio_s_artifact_norm_en1998_model::En1998Assessment, patch: En1998AssessmentPatch, list: Vec<semio_s_artifact_norm_en1998_model::En1998Assessment>, key: String = by En1998AssessmentKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `En1998Silo`.
    pub En1998SiloPatch of semio_s_artifact_norm_en1998_model::En1998Silo { set { height_m: f64, radius_m: f64, permanent_gk_n: f64, content_qk_n: f64, content_category: semio_s_artifact_norm_en1998_model::ImposedCategory, filling_ratio: f64, n_rd_n: f64, v_rd_n: f64, q_nominal: f64 } }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `En1998Silo` list.
    pub En1998SiloDelta { removal: En1998SiloRemoval, insertion: En1998SiloInsertion, relocation: En1998SiloRelocation, modification: En1998SiloModification, row: semio_s_artifact_norm_en1998_model::En1998Silo, patch: En1998SiloPatch, list: Vec<semio_s_artifact_norm_en1998_model::En1998Silo>, key: String = by En1998SiloKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `En1998Tank`.
    pub En1998TankPatch of semio_s_artifact_norm_en1998_model::En1998Tank { set { height_m: f64, radius_m: f64, permanent_gk_n: f64, content_qk_n: f64, content_category: semio_s_artifact_norm_en1998_model::ImposedCategory, filling_ratio: f64, v_rd_n: f64, shell_strength_mpa: f64, shell_thickness_m: f64 } }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `En1998Tank` list.
    pub En1998TankDelta { removal: En1998TankRemoval, insertion: En1998TankInsertion, relocation: En1998TankRelocation, modification: En1998TankModification, row: semio_s_artifact_norm_en1998_model::En1998Tank, patch: En1998TankPatch, list: Vec<semio_s_artifact_norm_en1998_model::En1998Tank>, key: String = by En1998TankKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `En1998Foundation`.
    pub En1998FoundationPatch of semio_s_artifact_norm_en1998_model::En1998Foundation { set { supported_building_id: String, area_m2: f64, p_rd_pa: f64, passive_resistance_n: f64, k_foundation: f64, k_soil: f64, liq_n1_60: f64, liq_fines_percent: f64, liq_depth_m: f64, liq_water_depth_m: f64, liq_unit_weight_n_m3: f64, liq_magnitude: f64, liq_layer_thickness_m: f64, footing_width_m: f64, bearing_axial_n: f64, bearing_shear_n: f64, bearing_moment_nm: f64, bearing_n_max_n: f64, soil_friction_angle_deg: f64, pile_diameter_m: f64, pile_length_m: f64, pile_modulus_pa: f64, pile_subgrade_modulus_n_m3: f64, pile_head_shear_n: f64, pile_head_fixed: f64, pile_interface_depth_m: f64, pile_upper_vs_m_s: f64, pile_lower_vs_m_s: f64, pile_moment_rd_nm: f64, base_friction_angle_deg: f64, base_pore_pressure_pa: f64 } }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `En1998Foundation` list.
    pub En1998FoundationDelta { removal: En1998FoundationRemoval, insertion: En1998FoundationInsertion, relocation: En1998FoundationRelocation, modification: En1998FoundationModification, row: semio_s_artifact_norm_en1998_model::En1998Foundation, patch: En1998FoundationPatch, list: Vec<semio_s_artifact_norm_en1998_model::En1998Foundation>, key: String = by En1998FoundationKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `En1998RetainingWall`.
    pub En1998RetainingWallPatch of semio_s_artifact_norm_en1998_model::En1998RetainingWall { set { height_m: f64, phi_deg: f64, soil_gamma: f64, r: f64, h_rd_n_per_m: f64, water_height_m: f64, saturated_gamma: f64, permeability_m_s: f64 } }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `En1998RetainingWall` list.
    pub En1998RetainingWallDelta { removal: En1998RetainingWallRemoval, insertion: En1998RetainingWallInsertion, relocation: En1998RetainingWallRelocation, modification: En1998RetainingWallModification, row: semio_s_artifact_norm_en1998_model::En1998RetainingWall, patch: En1998RetainingWallPatch, list: Vec<semio_s_artifact_norm_en1998_model::En1998RetainingWall>, key: String = by En1998RetainingWallKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `En1998Tower`.
    pub En1998TowerPatch of semio_s_artifact_norm_en1998_model::En1998Tower { set { height_m: f64, m_rd_nm: f64, is_chimney: bool, q_nominal: f64, permanent_gk_n: f64, correlated_occupancy: bool, ei_nm2: f64, base_rotational_stiffness_nm: f64 } nest { variables: En1998VariableActionDelta } }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `En1998Placement`.
    pub En1998PlacementPatch of semio_s_artifact_norm_en1998_model::En1998Placement { set { x: f64, y: f64, z: f64, yaw: f64 } }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `En1998Tower` list.
    pub En1998TowerDelta { removal: En1998TowerRemoval, insertion: En1998TowerInsertion, relocation: En1998TowerRelocation, modification: En1998TowerModification, row: semio_s_artifact_norm_en1998_model::En1998Tower, patch: En1998TowerPatch, list: Vec<semio_s_artifact_norm_en1998_model::En1998Tower>, key: String = by En1998TowerKey }
}
//#endregion 🔖️Rows

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `En1998Placement` list.
    pub En1998PlacementDelta { removal: En1998PlacementRemoval, insertion: En1998PlacementInsertion, relocation: En1998PlacementRelocation, modification: En1998PlacementModification, row: semio_s_artifact_norm_en1998_model::En1998Placement, patch: En1998PlacementPatch, list: Vec<semio_s_artifact_norm_en1998_model::En1998Placement>, key: String = by En1998PlacementKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `En1998SoilLayer`.
    pub En1998SoilLayerPatch of semio_s_artifact_norm_en1998_model::En1998SoilLayer { set { foundation_id: String, thickness_m: f64, unit_weight_n_m3: f64, shear_wave_velocity_m_s: f64, n1_60: f64, fines_percent: f64 } }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `En1998SoilLayer` list.
    pub En1998SoilLayerDelta { removal: En1998SoilLayerRemoval, insertion: En1998SoilLayerInsertion, relocation: En1998SoilLayerRelocation, modification: En1998SoilLayerModification, row: semio_s_artifact_norm_en1998_model::En1998SoilLayer, patch: En1998SoilLayerPatch, list: Vec<semio_s_artifact_norm_en1998_model::En1998SoilLayer>, key: String = by En1998SoilLayerKey }
}

/// 🔑️ Key extractor of the foreign `En1998VariableAction` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct En1998VariableActionKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1998_model::En1998VariableAction> for En1998VariableActionKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1998_model::En1998VariableAction) -> String {
        row.id.clone()
    }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `En1998VariableAction`.
    pub En1998VariableActionPatch of semio_s_artifact_norm_en1998_model::En1998VariableAction { set { category: semio_s_artifact_norm_en1998_model::ImposedCategory, qk_n: f64 } }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of the `variables` list of a storey, bridge or tower.
    pub En1998VariableActionDelta { removal: En1998VariableActionRemoval, insertion: En1998VariableActionInsertion, relocation: En1998VariableActionRelocation, modification: En1998VariableActionModification, row: semio_s_artifact_norm_en1998_model::En1998VariableAction, patch: En1998VariableActionPatch, list: Vec<semio_s_artifact_norm_en1998_model::En1998VariableAction>, key: String = by En1998VariableActionKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `En1998Diaphragm`.
    pub En1998DiaphragmPatch of semio_s_artifact_norm_en1998_model::En1998Diaphragm { set { building_id: String, storey_id: String, material: semio_s_artifact_norm_en1998_model::DiaphragmMaterial, thickness_m: f64, span_m: f64, depth_m: f64, opening_width_m: f64, shear_strength_mpa: f64, chord_area_m2: f64, chord_strength_mpa: f64, gamma_d: f64 } }
}

/// 🔑️ Key extractor of the foreign `En1998Diaphragm` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct En1998DiaphragmDeltaKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1998_model::En1998Diaphragm> for En1998DiaphragmDeltaKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1998_model::En1998Diaphragm) -> String {
        row.id.clone()
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `En1998Diaphragm` list.
    pub En1998DiaphragmDelta { removal: En1998DiaphragmRemoval, insertion: En1998DiaphragmInsertion, relocation: En1998DiaphragmRelocation, modification: En1998DiaphragmModification, row: semio_s_artifact_norm_en1998_model::En1998Diaphragm, patch: En1998DiaphragmPatch, list: Vec<semio_s_artifact_norm_en1998_model::En1998Diaphragm>, key: String = by En1998DiaphragmDeltaKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `En1998Accelerogram`.
    pub En1998AccelerogramPatch of semio_s_artifact_norm_en1998_model::En1998Accelerogram { set { building_id: String, label: String, samples: String, dt_s: f64, scale_factor: f64 } }
}

/// 🔑️ Key extractor of the foreign `En1998Accelerogram` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct En1998AccelerogramDeltaKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1998_model::En1998Accelerogram> for En1998AccelerogramDeltaKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1998_model::En1998Accelerogram) -> String {
        row.id.clone()
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `En1998Accelerogram` list.
    pub En1998AccelerogramDelta { removal: En1998AccelerogramRemoval, insertion: En1998AccelerogramInsertion, relocation: En1998AccelerogramRelocation, modification: En1998AccelerogramModification, row: semio_s_artifact_norm_en1998_model::En1998Accelerogram, patch: En1998AccelerogramPatch, list: Vec<semio_s_artifact_norm_en1998_model::En1998Accelerogram>, key: String = by En1998AccelerogramDeltaKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `En1998Pier`.
    pub En1998PierPatch of semio_s_artifact_norm_en1998_model::En1998Pier { set { bridge_id: String, x_m: f64, height_m: f64, section_b_m: f64, section_h_m: f64, modulus_mpa: f64, moment_rd_nm: f64, shear_rd_n: f64 , reinforcement_ratio: f64, cover_m: f64, concrete_fck_mpa: f64, steel_fyk_mpa: f64, axial_load_n: f64, stiffness_basis: semio_s_artifact_norm_en1998_model::PierStiffnessBasis} }
}

/// 🔑️ Key extractor of the foreign `En1998Pier` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct En1998PierDeltaKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1998_model::En1998Pier> for En1998PierDeltaKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1998_model::En1998Pier) -> String {
        row.id.clone()
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `En1998Pier` list.
    pub En1998PierDelta { removal: En1998PierRemoval, insertion: En1998PierInsertion, relocation: En1998PierRelocation, modification: En1998PierModification, row: semio_s_artifact_norm_en1998_model::En1998Pier, patch: En1998PierPatch, list: Vec<semio_s_artifact_norm_en1998_model::En1998Pier>, key: String = by En1998PierDeltaKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `En1998Bearing`.
    pub En1998BearingPatch of semio_s_artifact_norm_en1998_model::En1998Bearing { set { bridge_id: String, pier_id: String, device: semio_s_artifact_norm_en1998_model::BearingDevice, x_m: f64, stiffness_n_per_m: f64, d_rd_m: f64, seat_length_m: f64, min_support_length_m: f64 } }
}

/// 🔑️ Key extractor of the foreign `En1998Bearing` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct En1998BearingDeltaKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1998_model::En1998Bearing> for En1998BearingDeltaKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1998_model::En1998Bearing) -> String {
        row.id.clone()
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `En1998Bearing` list.
    pub En1998BearingDelta { removal: En1998BearingRemoval, insertion: En1998BearingInsertion, relocation: En1998BearingRelocation, modification: En1998BearingModification, row: semio_s_artifact_norm_en1998_model::En1998Bearing, patch: En1998BearingPatch, list: Vec<semio_s_artifact_norm_en1998_model::En1998Bearing>, key: String = by En1998BearingDeltaKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `En1998Joint`.
    pub En1998JointPatch of semio_s_artifact_norm_en1998_model::En1998Joint { set { building_id: String, storey_id: String, column_id: String, beam_id: String, x_m: f64, y_m: f64, column_moment_rd_nm: f64, beam_moment_rd_nm: f64 } }
}

/// 🔑️ Key extractor of the foreign `En1998Joint` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct En1998JointDeltaKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1998_model::En1998Joint> for En1998JointDeltaKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1998_model::En1998Joint) -> String {
        row.id.clone()
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `En1998Joint` list.
    pub En1998JointDelta { removal: En1998JointRemoval, insertion: En1998JointInsertion, relocation: En1998JointRelocation, modification: En1998JointModification, row: semio_s_artifact_norm_en1998_model::En1998Joint, patch: En1998JointPatch, list: Vec<semio_s_artifact_norm_en1998_model::En1998Joint>, key: String = by En1998JointDeltaKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `En1998TieBeam`.
    pub En1998TieBeamPatch of semio_s_artifact_norm_en1998_model::En1998TieBeam { set { foundation_a_id: String, foundation_b_id: String, span: f64, width: f64, depth: f64, steel_area: f64, steel_design_strength: f64, relative_displacement: f64, displacement_limit: f64 } }
}

/// 🔑️ Key extractor of the foreign `En1998TieBeam` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct En1998TieBeamDeltaKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1998_model::En1998TieBeam> for En1998TieBeamDeltaKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1998_model::En1998TieBeam) -> String {
        row.id.clone()
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `En1998TieBeam` list.
    pub En1998TieBeamDelta { removal: En1998TieBeamRemoval, insertion: En1998TieBeamInsertion, relocation: En1998TieBeamRelocation, modification: En1998TieBeamModification, row: semio_s_artifact_norm_en1998_model::En1998TieBeam, patch: En1998TieBeamPatch, list: Vec<semio_s_artifact_norm_en1998_model::En1998TieBeam>, key: String = by En1998TieBeamDeltaKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `En1998Slope`.
    pub En1998SlopePatch of semio_s_artifact_norm_en1998_model::En1998Slope { set { governing_layer_id: String, height: f64, angle: f64, friction_angle: f64, cohesion: f64, topographic_amplification: f64, circle_centre_x: f64, circle_centre_z: f64, circle_radius: f64 } }
}

/// 🔑️ Key extractor of the foreign `En1998Slope` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct En1998SlopeDeltaKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1998_model::En1998Slope> for En1998SlopeDeltaKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1998_model::En1998Slope) -> String {
        row.id.clone()
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `En1998Slope` list.
    pub En1998SlopeDelta { removal: En1998SlopeRemoval, insertion: En1998SlopeInsertion, relocation: En1998SlopeRelocation, modification: En1998SlopeModification, row: semio_s_artifact_norm_en1998_model::En1998Slope, patch: En1998SlopePatch, list: Vec<semio_s_artifact_norm_en1998_model::En1998Slope>, key: String = by En1998SlopeDeltaKey }
}

//#region 🔖️Diff
/// 🔺️ Sparse delta for the En1998 artifact: the scalar fields a mutation sets and the keyed row deltas of its lists.
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.norm.en1998")]
pub struct En1998Diff {
    #[state(artifact)]
    pub annex: Option<semio_s_plugin_norm_compliance::AnnexChoice>,
    #[state(artifact)]
    pub site: Option<semio_s_artifact_norm_en1998_model::En1998Site>,
    #[state(artifact)]
    pub buildings: En1998BuildingDelta,
    #[state(artifact)]
    pub bridges: En1998BridgeDelta,
    #[state(artifact)]
    pub assessments: En1998AssessmentDelta,
    #[state(artifact)]
    pub silos: En1998SiloDelta,
    #[state(artifact)]
    pub tanks: En1998TankDelta,
    #[state(artifact)]
    pub foundations: En1998FoundationDelta,
    #[state(artifact)]
    pub retaining_walls: En1998RetainingWallDelta,
    #[state(artifact)]
    pub towers: En1998TowerDelta,
    #[state(artifact)]
    pub soil_layers: En1998SoilLayerDelta,
    #[state(artifact)]
    pub diaphragms: En1998DiaphragmDelta,
    #[state(artifact)]
    pub accelerograms: En1998AccelerogramDelta,
    #[state(artifact)]
    pub piers: En1998PierDelta,
    #[state(artifact)]
    pub bearings: En1998BearingDelta,
    #[state(artifact)]
    pub joints: En1998JointDelta,
    #[state(artifact)]
    pub tie_beams: En1998TieBeamDelta,
    #[state(artifact)]
    pub slopes: En1998SlopeDelta,
    #[state(artifact)]
    pub placements: En1998PlacementDelta,
}
//#endregion 🔖️Diff

impl MutationDiff<En1998Snapshot> for En1998Diff {}

impl protocol::ColdMutationDiff<En1998Snapshot> for En1998Diff {
    fn absorb(&mut self, other: Self) {
        if other.annex.is_some() {
            self.annex = other.annex;
        }
        if other.site.is_some() {
            self.site = other.site;
        }
        self.buildings.absorb(other.buildings);
        self.bridges.absorb(other.bridges);
        self.assessments.absorb(other.assessments);
        self.silos.absorb(other.silos);
        self.tanks.absorb(other.tanks);
        self.foundations.absorb(other.foundations);
        self.retaining_walls.absorb(other.retaining_walls);
        self.towers.absorb(other.towers);
        self.soil_layers.absorb(other.soil_layers);
        self.diaphragms.absorb(other.diaphragms);
        self.accelerograms.absorb(other.accelerograms);
        self.piers.absorb(other.piers);
        self.bearings.absorb(other.bearings);
        self.joints.absorb(other.joints);
        self.tie_beams.absorb(other.tie_beams);
        self.slopes.absorb(other.slopes);
        self.placements.absorb(other.placements);
    }

    fn apply(&self, unpublished: &mut En1998Snapshot, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        let base = &*unpublished;
        let mut next = base.clone();
        if let Some(value) = &self.annex {
            next.annex = value.clone();
        }
        if let Some(value) = &self.site {
            next.site = value.clone();
        }
        next.buildings = self.buildings.commit_onto(&base.buildings, capability).map_err(|error| error.under(["buildings"]))?;
        next.bridges = self.bridges.commit_onto(&base.bridges, capability).map_err(|error| error.under(["bridges"]))?;
        next.assessments = self.assessments.commit_onto(&base.assessments, capability).map_err(|error| error.under(["assessments"]))?;
        next.silos = self.silos.commit_onto(&base.silos, capability).map_err(|error| error.under(["silos"]))?;
        next.tanks = self.tanks.commit_onto(&base.tanks, capability).map_err(|error| error.under(["tanks"]))?;
        next.foundations = self.foundations.commit_onto(&base.foundations, capability).map_err(|error| error.under(["foundations"]))?;
        next.retaining_walls = self.retaining_walls.commit_onto(&base.retaining_walls, capability).map_err(|error| error.under(["retainingWalls"]))?;
        next.towers = self.towers.commit_onto(&base.towers, capability).map_err(|error| error.under(["towers"]))?;
        next.soil_layers = self.soil_layers.commit_onto(&base.soil_layers, capability).map_err(|error| error.under(["soilLayers"]))?;
        next.diaphragms = self.diaphragms.commit_onto(&base.diaphragms, capability).map_err(|error| error.under(["diaphragms"]))?;
        next.accelerograms = self.accelerograms.commit_onto(&base.accelerograms, capability).map_err(|error| error.under(["accelerograms"]))?;
        next.piers = self.piers.commit_onto(&base.piers, capability).map_err(|error| error.under(["piers"]))?;
        next.bearings = self.bearings.commit_onto(&base.bearings, capability).map_err(|error| error.under(["bearings"]))?;
        next.joints = self.joints.commit_onto(&base.joints, capability).map_err(|error| error.under(["joints"]))?;
        next.tie_beams = self.tie_beams.commit_onto(&base.tie_beams, capability).map_err(|error| error.under(["tieBeams"]))?;
        next.slopes = self.slopes.commit_onto(&base.slopes, capability).map_err(|error| error.under(["slopes"]))?;
        next.placements = self.placements.commit_onto(&base.placements, capability).map_err(|error| error.under(["placements"]))?;
        *unpublished = next;
        Ok(())
    }
}

impl DiffAlgebra<En1998Snapshot> for En1998Diff {
    fn is_empty(&self) -> bool {
        self.annex.is_none()
            && self.site.is_none()
            && self.buildings.is_empty()
            && self.bridges.is_empty()
            && self.assessments.is_empty()
            && self.silos.is_empty()
            && self.tanks.is_empty()
            && self.foundations.is_empty()
            && self.retaining_walls.is_empty()
            && self.towers.is_empty()
            && self.soil_layers.is_empty()
            && self.diaphragms.is_empty()
            && self.accelerograms.is_empty()
            && self.piers.is_empty()
            && self.bearings.is_empty()
            && self.joints.is_empty()
            && self.tie_beams.is_empty()
            && self.slopes.is_empty()
            && self.placements.is_empty()
    }
}

impl protocol::ColdDiffAlgebra<En1998Snapshot> for En1998Diff {
    fn inverse(&self, base: &En1998Snapshot) -> Self {
        Self {
            annex: self.annex.as_ref().map(|_| base.annex.clone()),
            site: self.site.as_ref().map(|_| base.site.clone()),
            buildings: self.buildings.inverse(&base.buildings),
            bridges: self.bridges.inverse(&base.bridges),
            assessments: self.assessments.inverse(&base.assessments),
            silos: self.silos.inverse(&base.silos),
            tanks: self.tanks.inverse(&base.tanks),
            foundations: self.foundations.inverse(&base.foundations),
            retaining_walls: self.retaining_walls.inverse(&base.retaining_walls),
            towers: self.towers.inverse(&base.towers),
            soil_layers: self.soil_layers.inverse(&base.soil_layers),
            diaphragms: self.diaphragms.inverse(&base.diaphragms),
            accelerograms: self.accelerograms.inverse(&base.accelerograms),
            piers: self.piers.inverse(&base.piers),
            bearings: self.bearings.inverse(&base.bearings),
            joints: self.joints.inverse(&base.joints),
            tie_beams: self.tie_beams.inverse(&base.tie_beams),
            slopes: self.slopes.inverse(&base.slopes),
            placements: self.placements.inverse(&base.placements),
        }
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

//#region 🍃️FacetLeaves
/// 🍃️ The five handcrafted leaves of the `diff` schema facet.
pub const DIFF_LEAVES: semio_framework_schema_registry::FacetLeaves = semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") };
//#endregion 🍃️FacetLeaves
