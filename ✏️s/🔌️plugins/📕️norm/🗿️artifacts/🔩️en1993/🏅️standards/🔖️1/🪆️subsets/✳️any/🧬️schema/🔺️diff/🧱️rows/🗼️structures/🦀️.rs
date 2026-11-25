//! 🧱️ EN 1993 diff rows (structures): the sparse field patch, key extractor and keyed row delta of 7 document row lists.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

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
    /// 🩹 Sparse field patch of one `Placement`.
    pub En1993PlacementPatch of semio_s_artifact_norm_en1993_model::Placement { set { x: f64, y: f64, z: f64, yaw: f64 } }
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
