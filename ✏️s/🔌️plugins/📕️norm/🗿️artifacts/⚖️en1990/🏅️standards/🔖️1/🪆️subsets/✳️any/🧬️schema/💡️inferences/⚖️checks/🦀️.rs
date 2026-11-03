//! ⚖️ EN 1990 compliance kernel: NDP tables, combination generator, verification catalogue and reliability mathematics behind the compliance report.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_plugin_norm_compliance::{AnnexChoice, CheckResult, CheckStatus, ClauseId, ImposedCategory, LocalizedCopy, Quantity, QuantityKind, Remedy, SubjectRef};

pub use semio_s_plugin_norm_compliance::NationalAnnex;

#[path = "📈️reliability-math/🦀️.rs"]
mod reliability_math;
pub use reliability_math::*;

#[path = "📜️ndp-tables/🦀️.rs"]
mod ndp_tables;
pub use ndp_tables::*;

#[path = "🧮️combination-generator/🦀️.rs"]
mod combination_generator;
pub use combination_generator::*;

pub use semio_s_plugin_norm_mechanics::{continuous_beam_envelope, permanent_arrangements, span_subsets, SpanBeam, SpanEnvelope, SpanFactors};

#[path = "../../../🧭️spec-trace/🦀️.rs"]
pub mod spec_trace;

#[path = "🚄️rail-dynamics/🦀️.rs"]
pub mod rail_dynamics;

#[path = "📇️catalogue/🦀️.rs"]
pub mod catalogue;

//#region 🔖️NationalAnnexes
fn annex_gamma_m(annex: AnnexChoice, material: &str) -> f64 {
    ndp(annex, &format!("ndp.en1990.gM.{material}"))
}

fn annex_xi(annex: AnnexChoice, category: &str) -> f64 {
    match category {
        "accidental" | "seismic" => 1.0,
        _ => xi(annex, Domain::Building),
    }
}

fn annex_psi(annex: AnnexChoice, category: &str, pick: fn(PsiRow) -> f64) -> f64 {
    psi_lookup(annex, category).map_or(f64::NAN, pick)
}

/// 🇩️🇪️ German national annex parameters (DIN EN 1990/NA), read from the NDP and ψ tables.
#[derive(Clone, Copy, Debug, Default)]
pub struct NaDe;

impl NationalAnnex for NaDe {
    fn choice(&self) -> AnnexChoice {
        AnnexChoice::De
    }
    fn gamma_g(&self) -> f64 {
        ndp(AnnexChoice::De, "ndp.en1990.gG_sup.str.A1")
    }
    fn gamma_q(&self) -> f64 {
        ndp(AnnexChoice::De, "ndp.en1990.gQ.str.A1")
    }
    fn gamma_m(&self, material: &str) -> f64 {
        annex_gamma_m(AnnexChoice::De, material)
    }
    fn gamma_r(&self) -> f64 {
        ndp(AnnexChoice::De, "ndp.en1990.gR")
    }
    fn xi(&self, category: &str) -> f64 {
        annex_xi(AnnexChoice::De, category)
    }
    fn psi_0(&self, category: &str) -> f64 {
        annex_psi(AnnexChoice::De, category, |row| row.psi_0)
    }
    fn psi_1(&self, category: &str) -> f64 {
        annex_psi(AnnexChoice::De, category, |row| row.psi_1)
    }
    fn psi_2(&self, category: &str) -> f64 {
        annex_psi(AnnexChoice::De, category, |row| row.psi_2)
    }
}

/// 🇪️🇺️ Recommended values EN 1990, read from the NDP and ψ tables.
#[derive(Clone, Copy, Debug, Default)]
pub struct NaEn;

impl NationalAnnex for NaEn {
    fn choice(&self) -> AnnexChoice {
        AnnexChoice::En
    }
    fn gamma_g(&self) -> f64 {
        ndp(AnnexChoice::En, "ndp.en1990.gG_sup.str.A1")
    }
    fn gamma_q(&self) -> f64 {
        ndp(AnnexChoice::En, "ndp.en1990.gQ.str.A1")
    }
    fn gamma_m(&self, material: &str) -> f64 {
        annex_gamma_m(AnnexChoice::En, material)
    }
    fn gamma_r(&self) -> f64 {
        ndp(AnnexChoice::En, "ndp.en1990.gR")
    }
    fn xi(&self, category: &str) -> f64 {
        annex_xi(AnnexChoice::En, category)
    }
    fn psi_0(&self, category: &str) -> f64 {
        annex_psi(AnnexChoice::En, category, |row| row.psi_0)
    }
    fn psi_1(&self, category: &str) -> f64 {
        annex_psi(AnnexChoice::En, category, |row| row.psi_1)
    }
    fn psi_2(&self, category: &str) -> f64 {
        annex_psi(AnnexChoice::En, category, |row| row.psi_2)
    }
}

use semio_framework_dispatch_macros::dyn_enum_close;
use semio_s_plugin_norm_compliance::__semio_dispatch_NationalAnnex;
dyn_enum_close! {
    pub enum NationalAnnexes: NationalAnnex {
        De(NaDe),
        En(NaEn),
    }
}

pub mod na_de {
    pub use crate::NaDe;
}

pub mod na_en {
    pub use crate::NaEn;
}
//#endregion 🔖️NationalAnnexes

//#region 🔖️Reliability
fn copy(en: impl Into<String>, de: impl Into<String>) -> LocalizedCopy {
    LocalizedCopy::new(en, de)
}

/// ✅️ Annex B/C check β ≥ β_target(RC, T) with the failure probability Φ(−β) in the explanation; an unknown RC or period yields a not-applicable result.
pub fn check_reliability_index_for(beta: f64, consequence_class: u8, reliability_class: u8, reference_period_years: f64, annex: AnnexChoice) -> CheckResult {
    let subject = SubjectRef::new("", "betaComputed", copy("Reliability index", "Zuverlässigkeitsindex"));
    let builder = CheckResult::assess("en1990.annex-c.beta", "EN 1990 Annex B", ClauseId::new("EN 1990", "Annex B", "Table B2"), subject.clone(), copy("Target reliability index β", "Zielwert des Zuverlässigkeitsindex β")).ndp("ndp.en1990.e90-28.1").annex(annex);
    let Some(target) = target_reliability_index_for(reliability_class, reference_period_years) else {
        return builder.not_applicable(copy("Reliability class or reference period outside Table B2.", "Zuverlässigkeitsklasse oder Bezugszeitraum außerhalb Tabelle B2.")).build();
    };
    let (pf, pf_target) = (phi_cdf(-beta), phi_cdf(-target));
    let mut builder = builder.minimum(Quantity::new(QuantityKind::Dimensionless, beta), Quantity::new(QuantityKind::Dimensionless, target)).explanation(copy(
        format!("β = {beta:.2} (P_f = {pf:.2e}) vs β_target = {target:.2} (P_f = {pf_target:.2e}) for RC{reliability_class}, T = {reference_period_years:.0} a, CC{consequence_class}"),
        format!("β = {beta:.2} (P_f = {pf:.2e}) gegenüber β_Ziel = {target:.2} (P_f = {pf_target:.2e}) für RC{reliability_class}, T = {reference_period_years:.0} a, CC{consequence_class}"),
    ));
    if beta < target {
        builder = builder.remedy(Remedy::at_least(
            subject,
            Quantity::new(QuantityKind::Dimensionless, beta),
            Quantity::new(QuantityKind::Dimensionless, target),
            copy(format!("Increase computed β from {beta:.2} to at least {target:.2}, or lower the reliability class."), format!("Berechnetes β von {beta:.2} auf mindestens {target:.2} erhöhen oder Zuverlässigkeitsklasse absenken.")),
        ));
    }
    builder.build()
}
//#endregion 🔖️Reliability

//#region 🧪️ComplianceTests
#[cfg(test)]
#[path = "../../🧪️tests/⚖️compliance/🦀️.rs"]
mod compliance_tests;
//#endregion 🧪️ComplianceTests
