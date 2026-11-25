//! 🧱️ EN 1993 model rows (members): 13 row and vocabulary types of the snapshot with their display and key helpers.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_artifact_norm_en1993_model_rows_materials::*;

/// 🔩 Member type: beam or beam-column.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum SteelMemberType {
    Beam,
    Column,
    BeamColumn,
    Brace,
    Tie,
}

/// ⬇️ Level of the transverse load relative to the shear centre.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum LoadApplication {
    ShearCenter,
    TopFlange,
    BottomFlange,
}

/// 📈 Shape of the bending moment diagram between the lateral restraints.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum MomentDiagram {
    Linear,
    Uniform,
    Parabolic,
}

/// 🧮 Global analysis type (elastic or plastic).
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum AnalysisKind {
    Elastic,
    Plastic,
}

/// 🔩 Forming of a stainless steel member (EN 1993-1-4).
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum StainlessForming {
    Unset,
    ColdFormedOpen,
    WeldedOpen,
    Hollow,
}

/// 🌀 Application of the torsion action on a fork-supported member.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum TorsionLoad {
    Point,
    Udl,
}

/// 🏗️ Structural member (lengths in m; ψ dimensionless).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct SteelMember {
    pub id: String,
    pub label: String,
    pub member_type: SteelMemberType,
    pub section_id: String,
    pub material_id: String,
    pub length: f64,
    pub buckling_length_y: f64,
    pub buckling_length_z: f64,
    pub ltb_length: f64,
    pub ltb_restraint_spacing: f64,
    pub load_application: LoadApplication,
    pub end_moment_ratio_psi: f64,
    pub moment_diagram: MomentDiagram,
    pub deflection_limit_ratio: f64,
    pub analysis: AnalysisKind,
    pub stainless_forming: StainlessForming,
    pub design_temperature: f64,
    pub toughness_stress_ratio: f64,
    pub radiation_shift: f64,
    pub stress_shift: f64,
    pub margin_shift: f64,
    pub strain_rate_shift: f64,
    pub cold_forming_radius: f64,
    pub web_material_id: String,
    pub torsion_load: TorsionLoad,
    pub torque_position: f64,
    pub warping_support: WarpingSupport,
}

/// ⚓ Support condition at the end of a restrained member.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum RestraintSupport {
    Fork,
    Fixed,
}

/// 📈 Load and moment distribution of a restrained member.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum RestraintDistribution {
    Uniform,
    Udl,
    Linear,
}

/// 🧷 Supports and restraints of a member: end supports, an intermediate support, continuous torsional and discrete lateral springs, load height; M_cr by finite elements (EN 1993-1-1 6.3.2, 6.3.4, 6.3.5.3).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Restraint {
    pub id: String,
    pub member_id: String,
    pub label: String,
    pub start_support: RestraintSupport,
    pub end_support: RestraintSupport,
    pub intermediate_position: f64,
    pub torsional_spring: f64,
    pub lateral_spring_position: f64,
    pub lateral_spring_stiffness: f64,
    pub load_height: f64,
    pub load_distribution: RestraintDistribution,
    pub design_moment: f64,
    pub design_axial: f64,
    pub general_method: u32,
    pub hinge_to_restraint: f64,
}

/// ⬜ Chord section outline shape of a built-up member.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum ChordShape {
    Unset,
    Channel,
    I,
    Rhs,
    Chs,
}

/// 🪜 Kind of a built-up compression member: laced, battened or closely spaced (EN 1993-1-1 6.4).
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum BuiltUpKind {
    Laced,
    Battened,
    Closely,
}

/// 🪜 Built-up compression member: laced, battened or closely spaced chords (EN 1993-1-1 6.4).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct BuiltUpMember {
    pub id: String,
    pub member_id: String,
    pub label: String,
    pub kind: BuiltUpKind,
    pub chord_area: f64,
    pub chord_inertia: f64,
    pub chord_distance: f64,
    pub panel_spacing: f64,
    pub planes: u32,
    pub batten_inertia: f64,
    pub diagonal_area: f64,
    pub diagonal_length: f64,
    pub design_axial: f64,
    pub design_moment: f64,
    pub back_to_back: u32,
    pub min_radius: f64,
    pub chain_spacing: f64,
    pub chord_shape: ChordShape,
    pub chord_depth: f64,
    pub chord_width: f64,
    pub chord_flange_thickness: f64,
    pub chord_web_thickness: f64,
}

impl SteelMemberType {
    /// 🔤 Legacy string key of the variant, for the string-keyed registry, kernels and messages.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Beam => "beam",
            Self::Column => "column",
            Self::BeamColumn => "beamColumn",
            Self::Brace => "brace",
            Self::Tie => "tie",
        }
    }
}

impl std::fmt::Display for SteelMemberType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl LoadApplication {
    /// 🔤 Legacy string key of the variant, for the string-keyed registry, kernels and messages.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ShearCenter => "shearCenter",
            Self::TopFlange => "topFlange",
            Self::BottomFlange => "bottomFlange",
        }
    }
}

impl std::fmt::Display for LoadApplication {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl MomentDiagram {
    /// 🔤 Legacy string key of the variant, for the string-keyed registry, kernels and messages.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Linear => "linear",
            Self::Uniform => "uniform",
            Self::Parabolic => "parabolic",
        }
    }
}

impl std::fmt::Display for MomentDiagram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl AnalysisKind {
    /// 🔤 Legacy string key of the variant, for the string-keyed registry, kernels and messages.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Elastic => "elastic",
            Self::Plastic => "plastic",
        }
    }
}

impl std::fmt::Display for AnalysisKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl StainlessForming {
    /// 🔤 Legacy string key of the variant, for the string-keyed registry, kernels and messages.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unset => "",
            Self::ColdFormedOpen => "coldFormedOpen",
            Self::WeldedOpen => "weldedOpen",
            Self::Hollow => "hollow",
        }
    }
}

impl std::fmt::Display for StainlessForming {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl TorsionLoad {
    /// 🔤 Legacy string key of the variant, for the string-keyed registry, kernels and messages.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Point => "point",
            Self::Udl => "udl",
        }
    }
}

impl std::fmt::Display for TorsionLoad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl RestraintSupport {
    /// 🔤 Legacy string key of the variant, for the string-keyed registry, kernels and messages.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Fork => "fork",
            Self::Fixed => "fixed",
        }
    }
}

impl std::fmt::Display for RestraintSupport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl RestraintDistribution {
    /// 🔤 Legacy string key of the variant, for the string-keyed registry, kernels and messages.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Uniform => "uniform",
            Self::Udl => "udl",
            Self::Linear => "linear",
        }
    }
}

impl std::fmt::Display for RestraintDistribution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl ChordShape {
    /// 🔤 Legacy string key of the variant, for the string-keyed registry, kernels and messages.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unset => "",
            Self::Channel => "channel",
            Self::I => "i",
            Self::Rhs => "rhs",
            Self::Chs => "chs",
        }
    }
}

impl std::fmt::Display for ChordShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl BuiltUpKind {
    /// 🔤 Legacy string key of the variant, for the string-keyed registry, kernels and messages.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Laced => "laced",
            Self::Battened => "battened",
            Self::Closely => "closely",
        }
    }
}

impl std::fmt::Display for BuiltUpKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
