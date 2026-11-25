//! 🧱️ EN 1993 model rows (materials): 10 row and vocabulary types of the snapshot with their display and key helpers.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;



impl StiffenerShape {
    /// 🔢️ Every outline in token order, the options of its choice grip.
    pub const ALL: [Self; 4] = [Self::None, Self::Flat, Self::Tee, Self::Angle];
}

impl WarpingSupport {
    /// 🎚️ Every end condition in token order, the options of its choice grip.
    pub const ALL: [Self; 4] = [Self::Fork, Self::Fixed, Self::FixedFork, Self::Cantilever];
}

/// 🔩 Material family of a steel grade: carbon or stainless steel (EN 1993-1-1 / EN 1993-1-4).
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum MaterialKind {
    Carbon,
    Stainless,
}

/// 🧱 Steel material grade entry (SI: fy/fu/E/G in Pa).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct SteelMaterial {
    pub id: String,
    pub grade: String,
    pub fy: f64,
    pub fu: f64,
    pub e_modulus: f64,
    pub g_modulus: f64,
    pub subgrade: String,
    pub kind: MaterialKind,
    pub ramberg_osgood_n: f64,
}

/// 📐 Kind of a steel section: rolled I, welded I, rectangular or square hollow section, circular hollow section or angle (equal or unequal legs; its y-y and z-z values are the principal u-u and v-v values, EN 1993-1-1 Figure 1.1).
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum SectionKind {
    RolledI,
    WeldedI,
    Hss,
    Chs,
    Angle,
}

/// 📐️ Steel section with full property table (SI: m, m², m³, m⁴).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct SteelSection {
    pub id: String,
    pub designation: String,
    pub kind: SectionKind,
    pub h: f64,
    pub b: f64,
    pub tw: f64,
    pub tf: f64,
    pub r: f64,
    pub area: f64,
    pub shear_area_y: f64,
    pub shear_area_z: f64,
    pub iy: f64,
    pub iz: f64,
    pub it: f64,
    pub iw: f64,
    pub w_el_y: f64,
    pub w_el_z: f64,
    pub w_pl_y: f64,
    pub w_pl_z: f64,
    pub area_net: f64,
    /// 🏭 Fabrication route of a hollow section for the buckling curve of EN 1993-1-1 Table 6.2: hot finished (curve a, a0 for S460) or cold formed (curve c); Unset for sections that are not hollow. A hollow section left Unset has no buckling curve, so its buckling rows stay open.
    pub fabrication: HollowFabrication,
}

/// 🌀️ Torsion and warping end conditions of a member (EN 1993-1-1 6.2.7): fork, fixed, fixed-fork or cantilever.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum WarpingSupport {
    Fork,
    Fixed,
    FixedFork,
    Cantilever,
}

/// 🧱️ Outline of a transverse stiffener of a plated panel (EN 1993-1-5 9.2.1): none, flat bar, tee or angle.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum StiffenerShape {
    None,
    Flat,
    Tee,
    Angle,
}

/// 📏 Shape of a parametric steel section.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum ParametricShape {
    WeldedI,
    Rhs,
    Chs,
    Angle,
}

/// 📐 Parametric section: welded I, rectangular and circular hollow sections by their dimensions, compared with the section row (A, I_y, I_z, W_el,y, W_pl,y).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ParametricSection {
    pub id: String,
    pub section_id: String,
    pub label: String,
    pub shape: ParametricShape,
    pub height: f64,
    pub width: f64,
    pub web_thickness: f64,
    pub flange_thickness: f64,
    pub tolerance: f64,
}

/// 🏭 Fabrication route of a hollow section (EN 1993-1-1 Table 6.2): hot finished or cold formed; Unset for sections that are not hollow.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum HollowFabrication {
    Unset,
    HotFinished,
    ColdFormed,
}

/// 🧯 Residual stress state of a fatigue detail: as welded, welded and stress relieved or non-welded.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum StressRelief {
    AsWelded,
    StressRelieved,
    NonWelded,
}

impl std::fmt::Display for StressRelief {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl StressRelief {
    /// 🔤 Wire token of the variant.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AsWelded => "AsWelded",
            Self::StressRelieved => "StressRelieved",
            Self::NonWelded => "NonWelded",
        }
    }
}

impl std::fmt::Display for HollowFabrication {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl HollowFabrication {
    /// 🔤 Wire token of the variant.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unset => "Unset",
            Self::HotFinished => "HotFinished",
            Self::ColdFormed => "ColdFormed",
        }
    }
}

impl MaterialKind {
    /// 🔤 Legacy string key of the variant, for the string-keyed registry, kernels and messages.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Carbon => "carbon",
            Self::Stainless => "stainless",
        }
    }
}

impl std::fmt::Display for MaterialKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl SectionKind {
    /// 🔤 Legacy string key of the variant, for the string-keyed registry, kernels and messages.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RolledI => "rolledI",
            Self::WeldedI => "weldedI",
            Self::Hss => "hss",
            Self::Chs => "chs",
            Self::Angle => "angle",
        }
    }
}

impl std::fmt::Display for SectionKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl ParametricShape {
    /// 🔤 Legacy string key of the variant, for the string-keyed registry, kernels and messages.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WeldedI => "weldedI",
            Self::Rhs => "rhs",
            Self::Chs => "chs",
            Self::Angle => "angle",
        }
    }
}

impl std::fmt::Display for ParametricShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
