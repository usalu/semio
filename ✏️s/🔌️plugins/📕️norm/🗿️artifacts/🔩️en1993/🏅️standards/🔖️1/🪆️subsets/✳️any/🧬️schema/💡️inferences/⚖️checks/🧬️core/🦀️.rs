//! 🧬️ EN 1993 checks core: shared compliance helpers, scope and structure helpers.

#![allow(async_fn_in_trait)]
#![allow(unused_imports)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_plugin_norm_compliance::AnnexChoice;
use semio_s_artifact_norm_en1993_model::{LoadCase, SteelJoint, SteelMaterial, SteelMember, SteelSection, TowerLeg};

//#region 🔖️ComplianceHelpers
/// 📐️ Pure EN 1993 compliance helpers — hierarchical subject, computed χ/χ_LT, remedies.
use semio_s_plugin_norm_compliance::{CheckReport, CheckResult, CheckStatus, ClauseId, LocalizedCopy, Quantity, QuantityKind, Remedy, SubjectRef};
use semio_s_artifact_norm_en1993_model::rolled_heb_catalogue;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn loc(en: &str, de: &str) -> LocalizedCopy {
    LocalizedCopy::new(en, de)
}

pub fn subject_member(member: &SteelMember, path: &str) -> SubjectRef {
    let de_label = format!("Bauteil {} ({})", member.id, member.label);
    SubjectRef::new(&member.id, path, loc(&member.label, &de_label))
}

pub fn subject_joint(joint: &SteelJoint, path: &str) -> SubjectRef {
    SubjectRef::new(&joint.id, path, loc(&format!("Joint {}", joint.id), &format!("Anschluss {}", joint.id)))
}

pub fn force_n(n: f64) -> Quantity {
    Quantity::new(QuantityKind::Force, n)
}

pub fn moment_nm(m: f64) -> Quantity {
    Quantity::new(QuantityKind::Moment, m)
}

pub fn stress_pa(s: f64) -> Quantity {
    Quantity::new(QuantityKind::Stress, s)
}

pub fn length_m(l: f64) -> Quantity {
    Quantity::length_m(l)
}

pub fn area_m2(a: f64) -> Quantity {
    Quantity::area_m2(a)
}

pub fn dimensionless(v: f64) -> Quantity {
    Quantity::new(QuantityKind::Dimensionless, v)
}

pub fn temperature_c(t: f64) -> Quantity {
    Quantity::new(QuantityKind::Temperature, t)
}

/// 🎯️ Restricts the per-row check loops to one row: `(scope tag, id)`; the default admits every row, `nothing` admits none.
#[derive(Clone, Copy, Debug, Default)]
pub struct Scope<'a>(Option<(&'a str, &'a str)>, bool);

impl<'a> Scope<'a> {
    pub fn row(tag: &'a str, id: &'a str) -> Self {
        Self(Some((tag, id)), false)
    }

    pub fn nothing() -> Self {
        Self(None, true)
    }

    pub fn admits(&self, tag: &str, id: &str) -> bool {
        !self.1 && self.0.is_none_or(|(t, i)| t == tag && i == id)
    }
}

pub use semio_s_artifact_norm_en1993_clauses as clauses;
pub use semio_s_artifact_norm_en1993_parts::*;

#[path = "../🧰️structure-helpers/🦀️.rs"]
pub mod structure_helpers;
pub use structure_helpers::*;

#[path = "../🔗️special-checks/🔩️joint-row-model/🦀️.rs"]
pub mod joint_row_model;
pub use joint_row_model::{joint_row_summary, joint_secant, JointRowSummary};

#[path = "../🕸️frame-envelope/🦀️.rs"]
pub mod frame_envelope;
pub use frame_envelope::frame_derived_effects;

//#endregion 🔖️ComplianceHelpers
