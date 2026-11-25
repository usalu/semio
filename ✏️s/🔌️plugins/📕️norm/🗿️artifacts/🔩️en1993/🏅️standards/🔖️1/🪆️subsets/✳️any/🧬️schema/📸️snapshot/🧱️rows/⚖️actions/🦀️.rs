//! 🧱️ EN 1993 model rows (actions): 10 row and vocabulary types of the snapshot with their display and key helpers.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;



/// 🏋️ Design action effects (SI: N, N·m).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct DesignAction {
    pub n: f64,
    pub vy: f64,
    pub vz: f64,
    pub my: f64,
    pub mz: f64,
    pub t: f64,
}

/// 🏋 Kind of an action of a load case (EN 1990 Table A1.1).
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum ActionKind {
    Permanent,
    Imposed,
    Wind,
    Snow,
    Accidental,
}

/// 📋 Load case identity.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct LoadCase {
    pub id: String,
    pub name: String,
    pub kind: ActionKind,
    pub category: String,
}

/// 🔗 Member action under a load case.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct MemberAction {
    pub id: String,
    pub member_id: String,
    pub load_case_id: String,
    pub action: DesignAction,
}

/// 📉 Characteristic scalar force under a load case (SI N).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ForceAction {
    pub id: String,
    pub load_case_id: String,
    pub force: f64,
}

/// 🔩 Characteristic joint forces under a load case (SI N).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct JointForceAction {
    pub id: String,
    pub load_case_id: String,
    pub shear: f64,
    pub tension: f64,
}

/// 🏗️ Action of one load case on the frame (EN 1990 6.4.3): a point force or moment at a node, or a uniform line load on a bar between two positions along it; the load cases are combined by the EN 1990 rules and every combination is solved on its own.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct FrameAction {
    pub id: String,
    pub load_case_id: String,
    pub target_id: String,
    pub kind: FrameActionKind,
    pub fx: f64,
    pub fz: f64,
    pub my: f64,
    pub start_ratio: f64,
    pub end_ratio: f64,
}

/// 🧭 Where the design forces of a steel member come from: the frame analysis (envelope over the combinations of its bars) or its explicit member actions (EN 1993-1-1 5.2.2, EN 1990 6.4.3).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct FrameMemberBasis {
    pub id: String,
    pub member_id: String,
    pub basis: MemberForceBasis,
}

/// 🏗️ Kind of a frame action: point force at a node, point moment at a node or uniform line load on a bar.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum FrameActionKind {
    NodalForce,
    NodalMoment,
    BarUniform,
}

/// 🧭 Source of the design forces of a member: the envelope of the frame bars of the member or its explicit member actions.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum MemberForceBasis {
    FrameDerived,
    Explicit,
}

impl std::fmt::Display for MemberForceBasis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl MemberForceBasis {
    /// 🔤 Wire token of the variant.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FrameDerived => "frameDerived",
            Self::Explicit => "explicit",
        }
    }
}

impl FrameActionKind {
    /// 🔤 Wire token of the variant.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NodalForce => "nodalForce",
            Self::NodalMoment => "nodalMoment",
            Self::BarUniform => "barUniform",
        }
    }
}

impl std::fmt::Display for FrameActionKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl ActionKind {
    /// 🔤 Legacy string key of the variant, for the string-keyed registry, kernels and messages.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Permanent => "permanent",
            Self::Imposed => "imposed",
            Self::Wind => "wind",
            Self::Snow => "snow",
            Self::Accidental => "accidental",
        }
    }
}

impl std::fmt::Display for ActionKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
