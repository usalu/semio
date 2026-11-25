//! 🧱️ EN 1993 model rows (structures): 16 row and vocabulary types of the snapshot with their display and key helpers.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_artifact_norm_en1993_model_rows_actions::*;
use semio_s_artifact_norm_en1993_model_rows_materials::*;

/// ❄️ Cold-formed thin-gauge member (1-3).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ColdFormedMember {
    pub id: String,
    pub b_bar: f64,
    pub thickness: f64,
    pub k_sigma: f64,
    pub psi: f64,
    pub fy: f64,
    pub gross_resistance: f64,
    #[dsl(table)]
    pub actions: Vec<ForceAction>,
    pub flange_width: f64,
    pub lip_length: f64,
    pub web_height: f64,
    pub web_slope: f64,
    pub opposite_flange_width: f64,
    pub spring_factor: f64,
    pub corner_radius: f64,
    pub flange_stress: f64,
    pub shear_force: f64,
    pub support_reaction: f64,
    pub support_length: f64,
    pub crippling_category: u32,
    pub stiffened_support: u32,
    pub bending_moment: f64,
    pub moment_resistance: f64,
    pub sheeting_stiffness: f64,
    pub required_restraint_stiffness: f64,
    pub purlin_span: f64,
    pub purlin_load: f64,
    pub purlin_lateral_ratio: f64,
}

/// 📍 Patch load type of EN 1993-1-5 Figure 6.1.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum PatchLoadType {
    Unset,
    Through,
    OneFlangeNearEnd,
    EndFlange,
}

/// 🧱 Plated panel for EN 1993-1-5.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct PlatedPanel {
    pub id: String,
    pub a: f64,
    pub b: f64,
    pub thickness: f64,
    pub fy: f64,
    pub k_sigma: f64,
    #[dsl(table)]
    pub actions: Vec<ForceAction>,
    pub stiffener_inertia: f64,
    pub stiffener_area: f64,
    pub stiffener_eccentricity: f64,
    pub stiffener_alpha: f64,
    pub compression_stress: f64,
    pub half_width: f64,
    pub equivalent_span: f64,
    pub hogging: u32,
    pub patch_type: PatchLoadType,
    pub patch_force: f64,
    pub patch_length: f64,
    pub patch_offset: f64,
    pub flange_thickness: f64,
    pub flange_width: f64,
    pub flange_fy: f64,
    pub longitudinal_offset: f64,
    pub longitudinal_inertia: f64,
    pub patch_moment: f64,
    pub patch_moment_resistance: f64,
    pub bending_moment: f64,
    pub plastic_moment: f64,
    pub flange_moment: f64,
    pub shear_force: f64,
    pub transverse_stiffener_inertia: f64,
    pub flange_induced_k: f64,
    pub flange_area: f64,
    pub stress_x: f64,
    pub stress_z: f64,
    pub shear_stress: f64,
    pub alpha_cr: f64,
    pub corrugation_flat: f64,
    pub corrugation_projection: f64,
    pub corrugation_depth: f64,
    pub web_height: f64,
    pub stiffener_spacing: f64,
    pub stiffener_shape: StiffenerShape,
    pub stiffener_depth: f64,
    pub stiffener_thickness: f64,
    pub stiffener_flange_width: f64,
    pub stiffener_flange_thickness: f64,
    pub lateral_pressure: f64,
    pub lateral_pressure_coefficient: f64,
    /// 🏷 Steel grade of the plate for the brittle fracture check of EN 1993-1-10 2.1 (Table 2.1 column, e.g. S355); empty = not given, the plate is then not assessed and no grade is assumed.
    pub steel_grade: String,
    /// 🏷 Sub-grade of the plate (JR, J0, J2, K2, ...) for Table 2.1 of EN 1993-1-10; empty = not given.
    pub steel_subgrade: String,
    /// 🌡 Reference temperature T_Ed of the plate in °C for EN 1993-1-10 2.1 (minimum member temperature with the shifts already applied); used with the plate grade, the stress level is taken as the highest transcribed column 0.75 f_y(t).
    pub design_temperature: f64,
}

/// 🏭 Fabrication tolerance class of a silo shell.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum FabricationToleranceClass {
    Unset,
    A,
    B,
    C,
}

/// 🛢️ Silo / shell of revolution.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct SiloShell {
    pub id: String,
    pub thickness: f64,
    pub radius: f64,
    pub depth: f64,
    pub k: f64,
    pub gamma: f64,
    pub fy: f64,
    pub length: f64,
    pub fabrication_class: FabricationToleranceClass,
    pub meridional_stress: f64,
    pub hoop_stress: f64,
    pub shell_shear_stress: f64,
    pub boundary_factor_axial: f64,
    pub boundary_factor_hoop: f64,
    pub boundary_factor_shear: f64,
    pub wall_friction: f64,
    pub plan_area: f64,
    pub plan_perimeter: f64,
    pub liquid_density: f64,
    pub liquid_height: f64,
    pub course_elevation: f64,
    pub internal_pressure: f64,
    pub external_pressure: f64,
    pub temperature_change: f64,
    pub ring_spacing: f64,
    pub ring_area: f64,
    pub ring_inertia: f64,
    pub ring_offset: f64,
}

/// 🪢 Tension component / cable (1-11).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct TensionComponent {
    pub id: String,
    pub f_uk: f64,
    pub f_k: f64,
    #[dsl(table)]
    pub actions: Vec<ForceAction>,
    pub partial_factor: f64,
    pub cable_length: f64,
    pub cable_weight: f64,
    pub cable_horizontal_force: f64,
    pub cable_modulus: f64,
    pub cable_stress: f64,
    pub cable_unit_weight: f64,
    pub thread_factor: f64,
    pub rod_stress_area: f64,
    pub rod_fu: f64,
    pub pin_diameter: f64,
    pub pin_plate_thickness: f64,
    pub pin_gap: f64,
    pub pin_outer_thickness: f64,
    pub pin_inner_thickness: f64,
    pub pin_fup: f64,
    pub pin_fyp: f64,
    pub pin_plate_fy: f64,
}

/// 🗼 Lattice member kind of a tower leg.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum LatticeKind {
    Unset,
    Leg,
    Diagonal,
    Redundant,
}

/// 🗼 Tower leg (EN 1993-3-1).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct TowerLeg {
    pub id: String,
    pub member_id: String,
    pub force_coefficient: f64,
    pub dynamic_factor: f64,
    #[dsl(table)]
    pub actions: Vec<ForceAction>,
    pub lattice_kind: LatticeKind,
    pub slenderness: f64,
    pub diameter: f64,
    pub height: f64,
    pub bending_stiffness: f64,
    pub mass_per_length: f64,
    pub strouhal: f64,
    pub log_decrement: f64,
    pub air_density: f64,
    pub mean_wind_speed: f64,
    pub crossing_rate: f64,
    pub design_life: f64,
    pub k_factor: f64,
    pub kw_factor: f64,
    pub lateral_force_coefficient: f64,
    pub permitted_amplitude_ratio: f64,
    pub ring_mode: f64,
    pub shell_thickness: f64,
    pub guy_area: f64,
    pub guy_angle: f64,
    pub guy_length: f64,
}

/// 🌊 Corrosion environment of a sheet pile.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum SheetPileEnvironment {
    Unset,
    UndisturbedSoil,
    PollutedSoil,
    CompactedFill,
    NonCompactedFill,
}

/// 🪵 Steel pile (EN 1993-5).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct SteelPile {
    pub id: String,
    pub section_id: String,
    pub material_id: String,
    pub driving_stress: f64,
    pub embedded_length: f64,
    pub shaft_perimeter: f64,
    #[dsl(table)]
    pub actions: Vec<ForceAction>,
    pub environment: SheetPileEnvironment,
    pub design_life: f64,
    pub wall_thickness: f64,
    pub interlock_bending_factor: f64,
    pub interlock_stiffness_factor: f64,
    pub section_modulus: f64,
    pub shear_area: f64,
    pub design_moment: f64,
    pub design_shear: f64,
    pub interlock_shear: f64,
}

/// 🏗️ Crane runway local web check (EN 1993-6).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct CraneRunway {
    pub id: String,
    pub member_id: String,
    pub wheel_contact_length: f64,
    pub dispersion: f64,
    pub web_thickness: f64,
    pub fy: f64,
    pub phi: f64,
    #[dsl(table)]
    pub actions: Vec<ForceAction>,
    pub max_wheel_load: f64,
    pub load_factor: f64,
    pub rail_inertia: f64,
    pub rail_fixed: u32,
    pub flange_effective_width: f64,
    pub flange_thickness: f64,
    pub wheel_eccentricity: f64,
    pub torsion_constant: f64,
    pub longitudinal_stress: f64,
    pub rail_radius: f64,
    pub rail_contact_width: f64,
    pub crane_nu: f64,
    pub crane_kq: f64,
    pub damage_exponent: f64,
    pub stress_range: f64,
    pub detail_category_stress: f64,
    pub fatigue_phi: f64,
}

/// 🪜 Storey of a frame: sway stability of EN 1993-1-1 5.2 and 5.3 (α_cr, imperfection, bracing).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Storey {
    pub id: String,
    pub member_id: String,
    pub label: String,
    pub height: f64,
    pub vertical_load: f64,
    pub horizontal_load: f64,
    pub drift: f64,
    pub columns: u32,
    pub plastic_analysis: u32,
    pub building_height: f64,
    pub bracing_load: f64,
    pub bracing_length: f64,
    pub bracing_deflection: f64,
}

/// 🏭 Forming process of a cold-formed section: cold roll forming or other forming.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum FormingProcess {
    RollForming,
    OtherForming,
}

impl std::fmt::Display for FormingProcess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FormingProcess {
    /// 🔤 Wire token of the variant.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RollForming => "RollForming",
            Self::OtherForming => "OtherForming",
        }
    }
}

/// 📍️ Persisted pose of one scene entity (`id` is the entity id; SI: m, rad). Entities without a placement follow the documented default layout.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Placement {
    pub id: String,
    /// ↔️ Origin x [m].
    pub x: f64,
    /// ↕️ Origin y [m].
    pub y: f64,
    /// ⬆️ Origin z [m].
    pub z: f64,
    /// 🧭️ Yaw about +Z [rad].
    pub yaw: f64,
}

/// 🧱 Concrete block, grout layer and anchor bolts under a base plate joint: block size and strength, grout thickness and strength, anchors in tension.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct FoundationBlock {
    pub id: String,
    pub joint_id: String,
    pub label: String,
    pub block_length: f64,
    pub block_width: f64,
    pub block_depth: f64,
    pub concrete_fck: f64,
    pub grout_fck: f64,
    pub grout_thickness: f64,
    pub anchor_count: u32,
    pub anchor_class: String,
    pub anchor_diameter: f64,
    pub anchor_stress_area: f64,
    pub anchor_embedment: f64,
    pub anchor_edge_distance: f64,
}

/// 🥫 🥫 Enhanced yield strength of a cold-formed member from the work hardening of the bends (EN 1993-1-3 3.2.2): process, ultimate strength of the base material, gross area and number of 90 degree bends with a small radius.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct FormingGain {
    pub id: String,
    /// 🥫 Id of the cold-formed member whose f_y is checked against the enhanced yield strength.
    pub member_id: String,
    /// 🏭 Cold roll forming or other forming (brake pressing) selects the coefficient k of f_ya.
    pub forming: FormingProcess,
    /// 💪 Ultimate tensile strength of the base material before forming.
    pub ultimate_strength: f64,
    /// ▭ Gross cross-section area A_g of the member.
    pub gross_area: f64,
    /// 🔢 Number of 90 degree bends with an inner radius r <= 5 t (bends of other angles count proportionally).
    pub bends: u32,
    /// 🥫 Yield strength the design uses for the member: the basic f_yb, or an enhanced value that must not exceed the average yield strength f_ya of the cold forming (EN 1993-1-3 3.2.2); 0 = the basic yield strength of the member is used.
    #[dsl(unit = "Pa")]
    pub used_yield: f64,
}

impl PatchLoadType {
    /// 🔤 Legacy string key of the variant, for the string-keyed registry, kernels and messages.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unset => "",
            Self::Through => "through",
            Self::OneFlangeNearEnd => "oneFlangeNearEnd",
            Self::EndFlange => "endFlange",
        }
    }
}

impl std::fmt::Display for PatchLoadType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FabricationToleranceClass {
    /// 🔤 Legacy string key of the variant, for the string-keyed registry, kernels and messages.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unset => "",
            Self::A => "A",
            Self::B => "B",
            Self::C => "C",
        }
    }
}

impl std::fmt::Display for FabricationToleranceClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl LatticeKind {
    /// 🔤 Legacy string key of the variant, for the string-keyed registry, kernels and messages.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unset => "",
            Self::Leg => "leg",
            Self::Diagonal => "diagonal",
            Self::Redundant => "redundant",
        }
    }
}

impl std::fmt::Display for LatticeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl SheetPileEnvironment {
    /// 🔤 Legacy string key of the variant, for the string-keyed registry, kernels and messages.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unset => "",
            Self::UndisturbedSoil => "undisturbedSoil",
            Self::PollutedSoil => "pollutedSoil",
            Self::CompactedFill => "compactedFill",
            Self::NonCompactedFill => "nonCompactedFill",
        }
    }
}

impl std::fmt::Display for SheetPileEnvironment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
