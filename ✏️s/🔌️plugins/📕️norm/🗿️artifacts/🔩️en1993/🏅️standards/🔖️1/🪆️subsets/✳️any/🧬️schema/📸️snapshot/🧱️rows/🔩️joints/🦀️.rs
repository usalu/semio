//! 🧱️ EN 1993 model rows (joints): 13 row and vocabulary types of the snapshot with their display and key helpers.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_artifact_norm_en1993_model_rows_actions::*;

/// 🔗 Assumed stiffness class of a joint.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum StiffnessClass {
    Unset,
    Rigid,
    SemiRigid,
    Pinned,
}

/// 🔗 Kind of a steel joint (EN 1993-1-8).
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum JointKind {
    Bolted,
    Welded,
    EndPlate,
    BasePlate,
}

/// 🔩 Category A, B or C of a bolted connection (EN 1993-1-8 Table 3.2).
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum JointCategory {
    A,
    B,
    C,
}

/// 🔩 Bolted or welded joint.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct SteelJoint {
    pub id: String,
    pub kind: JointKind,
    pub member_id: String,
    pub bolt_class: String,
    pub bolt_diameter: f64,
    pub bolt_rows: u32,
    pub bolts_per_row: u32,
    pub pitch: f64,
    pub gauge: f64,
    pub end_distance: f64,
    pub edge_distance: f64,
    pub shear_planes: u32,
    pub plate_thickness: f64,
    pub plate_fu: f64,
    pub weld_throat: f64,
    pub weld_length: f64,
    pub weld_fu: f64,
    pub weld_grade: String,
    #[dsl(table)]
    pub actions: Vec<JointForceAction>,
    pub category: JointCategory,
    pub friction_mu: f64,
    pub preload_force: f64,
    pub slip_factor_ks: f64,
    pub friction_surfaces: u32,
    pub plate_fy: f64,
    pub plate_width: f64,
    pub bolt_to_web: f64,
    pub extension_to_flange: f64,
    pub extension_edge: f64,
    pub alpha_factor: f64,
    pub bolt_stress_area: f64,
    pub bolt_grip_length: f64,
    pub column_flange_thickness: f64,
    pub column_web_thickness: f64,
    pub column_web_depth: f64,
    pub column_web_shear_area: f64,
    pub column_root_radius: f64,
    pub column_fy: f64,
    pub column_web_stress: f64,
    pub beam_flange_thickness: f64,
    pub compression_lever: f64,
    pub moment_demand: f64,
    pub beam_moment_resistance: f64,
    pub column_moment_resistance: f64,
    pub beam_inertia: f64,
    pub beam_span: f64,
    pub braced_frame: u32,
    pub stiffness_ratio: f64,
    pub transformation_parameter: f64,
    pub assumed_class: StiffnessClass,
    pub base_plate_length: f64,
    pub concrete_fcd: f64,
    pub concrete_modulus: f64,
    pub concrete_area_a0: f64,
    pub concrete_area_a1: f64,
    pub joint_coefficient: f64,
    pub anchor_area: f64,
    pub anchor_length: f64,
    pub base_normal_force: f64,
    pub base_moment: f64,
    pub base_lever_left: f64,
    pub base_lever_right: f64,
    /// 🔩 Beam web thickness t_wb of an end plate joint for the beam web in tension F_t,wb,Rd = b_eff,t,wb t_wb f_y,wb / gamma_M0 (EN 1993-1-8 6.2.6.8); 0 = not given, the component is then not assessed and no value is assumed.
    #[dsl(unit = "m")]
    pub beam_web_thickness: f64,
    /// 🔩 Yield strength f_y,wb of the beam web of an end plate joint for the beam web in tension (EN 1993-1-8 6.2.6.8); 0 = not given, the component is then not assessed and no value is assumed.
    #[dsl(unit = "Pa")]
    pub beam_web_fy: f64,
    /// 🔥 Design force on the fillet weld group of the joint along the weld axis F_w,par,Ed, the source of tau_par in the directional method of EN 1993-1-8 4.5.3.2 (throat stress = force / (a l_w)); 0 = not given, the force then does not enter the weld check and no value is assumed.
    #[dsl(unit = "N")]
    pub weld_force_longitudinal: f64,
    /// 🔥 Design force on the fillet weld group of the joint perpendicular to the weld axis along a leg F_w,perp,Ed, the source of sigma_perp and tau_perp in the directional method of EN 1993-1-8 4.5.3.2 (throat stress = force / (a l_w)); 0 = not given, the force then does not enter the weld check and no value is assumed.
    #[dsl(unit = "N")]
    pub weld_force_transverse: f64,
}

/// 🪡️ Arrangement of the weld lines of a weld group (EN 1993-1-8 4.5.3.3): one line, two parallel lines or the four sides of a box.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum WeldPattern {
    Line,
    TwoLines,
    Box,
}

/// 🧵 Weld group under in-plane shear and torsion: elastic vector method (EN 1993-1-8 4.5.3.3).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct WeldGroup {
    pub id: String,
    pub member_id: String,
    pub label: String,
    pub pattern: WeldPattern,
    pub throat: f64,
    pub width: f64,
    pub depth: f64,
    pub shear_force: f64,
    pub eccentricity: f64,
    pub steel_fu: f64,
    pub beta_w: f64,
}

/// 🕳 Welded RHS truss joint with a gap (K and N joints): chord face failure and brace effective width (EN 1993-1-8 7.4).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct HollowJoint {
    pub id: String,
    pub member_id: String,
    pub label: String,
    pub chord_width: f64,
    pub chord_height: f64,
    pub chord_thickness: f64,
    pub chord_fy: f64,
    pub chord_stress: f64,
    pub brace1_width: f64,
    pub brace1_height: f64,
    pub brace1_thickness: f64,
    pub brace1_angle: f64,
    pub brace1_force: f64,
    pub brace2_width: f64,
    pub brace2_height: f64,
    pub brace2_thickness: f64,
    pub brace2_angle: f64,
    pub brace2_force: f64,
    pub gap: f64,
    pub brace_fy: f64,
}

/// 🔩 Row position of a bolt row.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum JointRowType {
    Extended,
    FirstBelow,
    Inner,
}

/// 🔩 Bolt row of an end plate joint: lever arm, T-stub geometry (m, e, m_x, e_x, alpha), pitch to the previous row, bolts and group effect for the EN 1993-1-8 component method.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct JointRow {
    pub id: String,
    pub joint_id: String,
    pub label: String,
    pub row_type: JointRowType,
    pub lever: f64,
    pub bolt_to_web: f64,
    pub edge_distance: f64,
    pub extension_to_flange: f64,
    pub extension_edge: f64,
    pub alpha_factor: f64,
    pub pitch_to_previous: f64,
    pub bolts: u32,
    pub bolt_diameter: f64,
    pub bolt_stress_area: f64,
    pub grip_length: f64,
    pub bolt_class: String,
    pub group_with_previous: u32,
}

/// 🪛 🪛 Mechanically fastened joint of cold-formed sheets (EN 1993-1-3 8.3): self-tapping screws or blind rivets in shear and tension with the sheet thicknesses and strengths, the fastener spacing and the shear resistance of the fastener from tests.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct FastenedJoint {
    pub id: String,
    /// 🥫 Id of the cold-formed member the joint belongs to.
    pub member_id: String,
    /// 🪛 Self-tapping screw or blind rivet.
    pub kind: FastenerKind,
    /// ⌀ Nominal diameter d of the screw or rivet.
    pub diameter: f64,
    /// ⭕ Diameter d_w of the washer or head that bears on the sheet in tension (pull-over); 0 = not given.
    pub washer_diameter: f64,
    /// 🔢 Number of fasteners n that share the force.
    pub count: u32,
    /// 🔢 Number of fasteners in the net section across the width.
    pub per_row: u32,
    /// 📏 Thickness t_1 of the sheet on the head side.
    pub head_thickness: f64,
    /// 📏 Thickness t_2 of the sheet or support the fastener ends in.
    pub base_thickness: f64,
    /// 💪 Ultimate tensile strength of the sheet t_1.
    pub head_strength: f64,
    /// 💪 Ultimate tensile strength of the sheet t_2.
    pub base_strength: f64,
    /// ↦ Distance e_1 from the fastener to the end of the sheet in the direction of the force.
    pub end_distance: f64,
    /// ↥ Distance e_2 from the fastener to the edge of the sheet perpendicular to the force.
    pub edge_distance: f64,
    /// ↔ Spacing p_1 of the fasteners in the direction of the force.
    pub pitch: f64,
    /// ↕ Spacing p_2 of the fastener rows perpendicular to the force.
    pub gauge: f64,
    /// ↔ Width b of the sheet in the net section across the fasteners.
    pub sheet_width: f64,
    /// ➡ Shear force on the fastened joint, shared by the n fasteners.
    pub shear_force: f64,
    /// ⬆ Tension force on the fastened joint (pull-out and pull-over), shared by the n fasteners.
    pub tension_force: f64,
    /// 🧪 Design shear resistance of one fastener from the manufacturer or tests; 0 = not given, only the sheet resistances are checked.
    pub shear_resistance: f64,
}

/// 🔩 Method for the force on each bolt of a bolt group: elastic vector method or instantaneous centre of rotation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum BoltGroupMethod {
    Elastic,
    InstantaneousCentre,
}

/// 🪛 Mechanical fastener of cold-formed sheets: self-tapping screw or blind rivet.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum FastenerKind {
    SelfTappingScrew,
    BlindRivet,
}

impl std::fmt::Display for FastenerKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FastenerKind {
    /// 🔤 Wire token of the variant.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SelfTappingScrew => "SelfTappingScrew",
            Self::BlindRivet => "BlindRivet",
        }
    }
}

impl std::fmt::Display for BoltGroupMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl BoltGroupMethod {
    /// 🔤 Wire token of the variant.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Elastic => "Elastic",
            Self::InstantaneousCentre => "InstantaneousCentre",
        }
    }
}

/// 🧩 Plate of a fin plate or splice joint: role, plate size and steel, bolt pattern, eccentricity of the bolt line to the support and weld to the support or member.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct JointPlate {
    pub id: String,
    pub joint_id: String,
    pub label: String,
    pub role: String,
    pub plate_height: f64,
    pub plate_width: f64,
    pub plate_thickness: f64,
    pub plate_count: u32,
    pub plate_fy: f64,
    pub plate_fu: f64,
    pub bolt_class: String,
    pub bolt_diameter: f64,
    pub hole_diameter: f64,
    pub bolt_rows: u32,
    pub bolts_per_row: u32,
    pub pitch: f64,
    pub gauge: f64,
    pub end_distance: f64,
    pub edge_distance: f64,
    pub shear_planes: u32,
    pub eccentricity: f64,
    pub weld_throat: f64,
    pub weld_fu: f64,
    pub weld_beta_w: f64,
    /// 🔩 Distribution of the load over the bolt group of the plate (EN 1993-1-8 3.12): Elastic is the vector method with the polar moment of the bolts, InstantaneousCentre is the plastic method with the load-deformation law of the bolts about the instantaneous centre of rotation.
    pub group_method: BoltGroupMethod,
}

impl StiffnessClass {
    /// 🔤 Legacy string key of the variant, for the string-keyed registry, kernels and messages.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unset => "",
            Self::Rigid => "rigid",
            Self::SemiRigid => "semiRigid",
            Self::Pinned => "pinned",
        }
    }
}

impl std::fmt::Display for StiffnessClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl WeldPattern {
    /// 🔤 Legacy string key of the variant, for the string-keyed registry, kernels and messages.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Line => "line",
            Self::TwoLines => "twoLines",
            Self::Box => "box",
        }
    }
}

impl std::fmt::Display for WeldPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl JointRowType {
    /// 🔤 Legacy string key of the variant, for the string-keyed registry, kernels and messages.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Extended => "extended",
            Self::FirstBelow => "firstBelow",
            Self::Inner => "inner",
        }
    }
}

impl std::fmt::Display for JointRowType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl JointKind {
    /// 🔤 Legacy string key of the variant, for the string-keyed registry, kernels and messages.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Bolted => "bolted",
            Self::Welded => "welded",
            Self::EndPlate => "endPlate",
            Self::BasePlate => "basePlate",
        }
    }
}

impl std::fmt::Display for JointKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl JointCategory {
    /// 🔤 Legacy string key of the variant, for the string-keyed registry, kernels and messages.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::A => "A",
            Self::B => "B",
            Self::C => "C",
        }
    }
}

impl std::fmt::Display for JointCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
