//! ⚖️ EN 1993 compliance checks: the clause-by-clause verification behind the compliance report.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_plugin_norm_compliance::AnnexChoice;
use semio_s_artifact_norm_en1993_model::{LoadCase, SteelJoint, SteelMaterial, SteelMember, SteelSection};

//#region 🔖️ComplianceHelpers
/// 📐️ Pure EN 1993 compliance helpers — hierarchical subject, computed χ/χ_LT, remedies.
use semio_s_plugin_norm_compliance::{CheckReport, CheckResult, CheckStatus, ClauseId, LocalizedCopy, Quantity, QuantityKind, Remedy, SubjectRef};
use semio_s_artifact_norm_en1993_model::rolled_heb_catalogue;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

/// 🇩🇪 EN vs DIN EN NA γ factors for EN 1993.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AnnexParams {
    pub choice: AnnexChoice,
    pub gamma_m0: f64,
    pub gamma_m1: f64,
    pub gamma_m2: f64,
    pub gamma_m3: f64,
    pub gamma_mf: f64,
}

/// 📐️ Shared EN 1993-1-1 partial factors (also published by catalogue `reference_tables`).
pub const GAMMA_M0_EN: f64 = 1.0;
pub const GAMMA_M1_EN: f64 = 1.0;
pub const GAMMA_M2_EN: f64 = 1.25;
pub const GAMMA_M3_EN: f64 = 1.25;
pub const GAMMA_MF_EN: f64 = 1.15;
pub const GAMMA_M0_DE: f64 = 1.0;
pub const GAMMA_M1_DE: f64 = 1.1;
pub const GAMMA_M2_DE: f64 = 1.25;
pub const GAMMA_M3_DE: f64 = 1.25;
pub const GAMMA_MF_DE: f64 = 1.15;
/// 🧮 EN 1990 STR combination factors (identical EN recommended / DE NA).
pub const GAMMA_G_STR: f64 = 1.35;
pub const GAMMA_Q_STR: f64 = 1.50;
pub const XI_STR: f64 = 0.85;

impl AnnexParams {
    /// 🇪🇺 EN recommended values.
    pub fn en() -> Self {
        Self { choice: AnnexChoice::En, gamma_m0: GAMMA_M0_EN, gamma_m1: GAMMA_M1_EN, gamma_m2: GAMMA_M2_EN, gamma_m3: GAMMA_M3_EN, gamma_mf: GAMMA_MF_EN }
    }

    /// 🇩🇪 DIN EN 1993-1-1/NA: γ_M1 = 1.1.
    pub fn de() -> Self {
        Self { choice: AnnexChoice::De, gamma_m0: GAMMA_M0_DE, gamma_m1: GAMMA_M1_DE, gamma_m2: GAMMA_M2_DE, gamma_m3: GAMMA_M3_DE, gamma_mf: GAMMA_MF_DE }
    }

    pub fn for_choice(choice: AnnexChoice) -> Self {
        match choice {
            AnnexChoice::En => Self::en(),
            AnnexChoice::De => Self::de(),
        }
    }
}

fn loc(en: &str, de: &str) -> LocalizedCopy {
    LocalizedCopy::new(en, de)
}

fn subject_member(member: &SteelMember, path: &str) -> SubjectRef {
    let de_label = format!("Bauteil {} ({})", member.id, member.label);
    SubjectRef::new(&member.id, path, loc(&member.label, &de_label))
}

fn subject_joint(joint: &SteelJoint, path: &str) -> SubjectRef {
    SubjectRef::new(&joint.id, path, loc(&format!("Joint {}", joint.id), &format!("Anschluss {}", joint.id)))
}

fn force_n(n: f64) -> Quantity {
    Quantity::new(QuantityKind::Force, n)
}

fn moment_nm(m: f64) -> Quantity {
    Quantity::new(QuantityKind::Moment, m)
}

fn stress_pa(s: f64) -> Quantity {
    Quantity::new(QuantityKind::Stress, s)
}

fn length_m(l: f64) -> Quantity {
    Quantity::length_m(l)
}

fn area_m2(a: f64) -> Quantity {
    Quantity::area_m2(a)
}

fn dimensionless(v: f64) -> Quantity {
    Quantity::new(QuantityKind::Dimensionless, v)
}

fn temperature_c(t: f64) -> Quantity {
    Quantity::new(QuantityKind::Temperature, t)
}

#[path = "📏️parts/🦀️.rs"]
mod parts;
pub use parts::*;

#[path = "🧰️structure-helpers/🦀️.rs"]
mod structure_helpers;
use structure_helpers::*;

#[path = "🧱️member-checks/🦀️.rs"]
mod member_checks;
pub use member_checks::*;

#[path = "🔗️special-element-checks/🦀️.rs"]
mod special_element_checks;
use special_element_checks::*;

//#endregion 🔖️ComplianceHelpers

#[cfg(test)]
#[path = "../../🧪️tests/⚖️compliance/🦀️.rs"]
mod compliance_tests;
