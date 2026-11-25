//! 🧱️ EN 1993 model rows (frame): 10 row and vocabulary types of the snapshot with their display and key helpers.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;



/// ⚓ Support of a frame node.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum FrameNodeSupport {
    Free,
    Pinned,
    Fixed,
    Roller,
}

/// 🕸 Node of a general planar frame in the x-z plane (z up) with its support and nodal design loads: input of the mechanics-kernel eigenvalue α_cr (EN 1993-1-1 5.2.1).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct FrameNode {
    pub id: String,
    pub label: String,
    pub x: f64,
    pub z: f64,
    pub support: FrameNodeSupport,
    pub load_x: f64,
    pub load_z: f64,
}

/// 🔗 End connection of a frame bar.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum FrameBarRelease {
    Rigid,
    Pinned,
    SemiRigid,
}

/// ⛓ End condition of a column at one node of a frame: framed, fixed or pinned.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum NodeBRestraint {
    Framed,
    Fixed,
    Pinned,
}

impl std::fmt::Display for NodeBRestraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl NodeBRestraint {
    /// 🔤 Wire token of the variant.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Framed => "Framed",
            Self::Fixed => "Fixed",
            Self::Pinned => "Pinned",
        }
    }
}

/// ⛓ End condition of a column at one node of a frame: framed, fixed or pinned.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum NodeARestraint {
    Framed,
    Fixed,
    Pinned,
}

impl std::fmt::Display for NodeARestraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl NodeARestraint {
    /// 🔤 Wire token of the variant.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Framed => "Framed",
            Self::Fixed => "Fixed",
            Self::Pinned => "Pinned",
        }
    }
}

/// ↔ Buckling mode of a frame: nodes held (non-sway) or free to translate (sway).
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum FrameSway {
    NonSway,
    Sway,
}

impl std::fmt::Display for FrameSway {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FrameSway {
    /// 🔤 Wire token of the variant.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NonSway => "NonSway",
            Self::Sway => "Sway",
        }
    }
}

/// 🧭 Buckling plane of a column: about the major or about the minor axis.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum BucklingPlane {
    MajorAxis,
    MinorAxis,
}

impl std::fmt::Display for BucklingPlane {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl BucklingPlane {
    /// 🔤 Wire token of the variant.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MajorAxis => "MajorAxis",
            Self::MinorAxis => "MinorAxis",
        }
    }
}

/// 🪢 Bar of a general planar frame between two frame nodes; section and material come from the referenced steel member, end releases make hinges.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct FrameBar {
    pub id: String,
    pub member_id: String,
    pub label: String,
    pub start_node_id: String,
    pub end_node_id: String,
    pub release_start: FrameBarRelease,
    pub release_end: FrameBarRelease,
}

/// 🧪 🧪 Elongation at failure and ultimate strain of a steel material (EN 1993-1-1 3.2.2): the values of the material certificate that, with f_u/f_y, decide whether a material is ductile enough for plastic global analysis.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct MaterialDuctility {
    pub id: String,
    /// 🧪 Id of the steel material the certificate values belong to.
    pub material_id: String,
    /// 📏 Elongation at failure of the material as a fraction (0.15 = 15 %).
    pub elongation: f64,
    /// 📏 Strain eps_u at the ultimate strength f_u as a fraction.
    pub ultimate_strain: f64,
}

/// 🪜 🪜 Buckling length of a column in a frame from the end restraint of the adjacent members (EN 1993-1-1 6.3.1.3 with the nomogram of the effective length factor): sway or non-sway mode, the stiffness sums of columns and beams at both ends, and the member whose buckling length it checks.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct BucklingLength {
    pub id: String,
    /// 🧱 Id of the column whose buckling length is derived.
    pub member_id: String,
    /// 🧭 Plane of the frame: buckling about the major axis (length L_cr,y) or the minor axis (L_cr,z).
    pub plane: BucklingPlane,
    /// ↔ Non-sway or sway buckling mode of the frame.
    pub sway: FrameSway,
    /// ⛓ End condition at node A (the lower end of a column): framed into columns and beams, fixed (rigid) or pinned.
    pub restraint_a: NodeARestraint,
    /// 🧮 Sum of I_c/L_c of the columns at node A, including the column itself.
    pub column_stiffness_a: f64,
    /// 🧮 Sum of alpha I_b/L_b of the beams at node A with the far-end factor alpha of each beam.
    pub beam_stiffness_a: f64,
    /// ⛓ End condition at node B (the upper end of a column): framed into columns and beams, fixed (rigid) or pinned.
    pub restraint_b: NodeBRestraint,
    /// 🧮 Sum of I_c/L_c of the columns at node B, including the column itself.
    pub column_stiffness_b: f64,
    /// 🧮 Sum of alpha I_b/L_b of the beams at node B with the far-end factor alpha of each beam.
    pub beam_stiffness_b: f64,
}

impl FrameNodeSupport {
    /// 🔤 Legacy string key of the variant, for the string-keyed registry, kernels and messages.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Free => "free",
            Self::Pinned => "pinned",
            Self::Fixed => "fixed",
            Self::Roller => "roller",
        }
    }
}

impl std::fmt::Display for FrameNodeSupport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FrameBarRelease {
    /// 🔤 Legacy string key of the variant, for the string-keyed registry, kernels and messages.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Rigid => "rigid",
            Self::Pinned => "pinned",
            Self::SemiRigid => "semiRigid",
        }
    }
}

impl std::fmt::Display for FrameBarRelease {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
