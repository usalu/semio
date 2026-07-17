//! 📏 Norm core: shared quantities, clause identity, compliance results, and national annex selection.

use serde::{Deserialize, Serialize};
use std::fmt;

// #region 🔖Quantity
/// 📐 Physical quantity kind for SI-normalized norm computations.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum QuantityKind {
    Dimensionless,
    Length,
    Area,
    Volume,
    Mass,
    Time,
    Temperature,
    Force,
    Pressure,
    Stress,
    Moment,
    Energy,
    Power,
    ThermalConductivity,
    ThermalResistance,
    HeatTransferCoefficient,
    AirPermeability,
    VentilationRate,
}

/// 📊 A scalar value tagged with its physical quantity kind (SI units).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Quantity {
    pub kind: QuantityKind,
    pub value: f64,
}

impl Quantity {
    pub const fn new(kind: QuantityKind, value: f64) -> Self {
        Self { kind, value }
    }

    pub fn length_m(value: f64) -> Self {
        Self::new(QuantityKind::Length, value)
    }

    pub fn area_m2(value: f64) -> Self {
        Self::new(QuantityKind::Area, value)
    }

    pub fn force_kn(value: f64) -> Self {
        Self::new(QuantityKind::Force, value * 1_000.0)
    }

    pub fn stress_mpa(value: f64) -> Self {
        Self::new(QuantityKind::Stress, value * 1_000_000.0)
    }

    pub fn thermal_resistance_m2k_w(value: f64) -> Self {
        Self::new(QuantityKind::ThermalResistance, value)
    }

    pub fn u_value_w_m2k(value: f64) -> Self {
        Self::new(QuantityKind::HeatTransferCoefficient, value)
    }
}
// #endregion 🔖Quantity

// #region 🔖Clause
/// 📑 Stable clause identifier within a norm family (e.g. `EN 1992-1-1` §6.1).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ClauseId {
    pub family: String,
    pub part: String,
    pub section: String,
}

impl ClauseId {
    pub fn new(family: impl Into<String>, part: impl Into<String>, section: impl Into<String>) -> Self {
        Self {
            family: family.into(),
            part: part.into(),
            section: section.into(),
        }
    }
}

impl fmt::Display for ClauseId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} §{}", self.family, self.part, self.section)
    }
}
// #endregion 🔖Clause

// #region 🔖Check
/// ✅ Outcome of a single norm compliance check.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CheckStatus {
    Pass,
    Fail,
    NotApplicable,
}

/// 📋 One computed check with clause traceability.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckResult {
    pub clause: ClauseId,
    pub status: CheckStatus,
    pub computed: Quantity,
    pub limit: Quantity,
    pub utilization: f64,
    pub message: String,
    pub annex: AnnexChoice,
}

impl CheckResult {
    pub fn pass(
        clause: ClauseId,
        computed: Quantity,
        limit: Quantity,
        utilization: f64,
        message: impl Into<String>,
        annex: AnnexChoice,
    ) -> Self {
        Self {
            clause,
            status: CheckStatus::Pass,
            computed,
            limit,
            utilization,
            message: message.into(),
            annex,
        }
    }

    pub fn fail(
        clause: ClauseId,
        computed: Quantity,
        limit: Quantity,
        utilization: f64,
        message: impl Into<String>,
        annex: AnnexChoice,
    ) -> Self {
        Self {
            clause,
            status: CheckStatus::Fail,
            computed,
            limit,
            utilization,
            message: message.into(),
            annex,
        }
    }

    pub fn from_utilization(
        clause: ClauseId,
        computed: Quantity,
        limit: Quantity,
        message: impl Into<String>,
        annex: AnnexChoice,
    ) -> Self {
        let utilization = if limit.value.abs() < f64::EPSILON {
            0.0
        } else {
            computed.value / limit.value
        };
        if utilization <= 1.0 {
            Self::pass(clause, computed, limit, utilization, message, annex)
        } else {
            Self::fail(clause, computed, limit, utilization, message, annex)
        }
    }
}

/// 📑 Aggregated compliance report for a norm computation run.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct CheckReport {
    pub checks: Vec<CheckResult>,
}

impl CheckReport {
    pub fn push(&mut self, check: CheckResult) {
        self.checks.push(check);
    }

    pub fn all_pass(&self) -> bool {
        self.checks
            .iter()
            .all(|c| c.status != CheckStatus::Fail)
    }

    pub fn worst_utilization(&self) -> f64 {
        self.checks
            .iter()
            .map(|c| c.utilization)
            .fold(0.0_f64, f64::max)
    }
}
// #endregion 🔖Check

// #region 🔖Annex
/// 🇪🇺 National annex selection for Eurocode / DIN EN families.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AnnexChoice {
    En,
    De,
}

impl AnnexChoice {
    pub fn label(self) -> &'static str {
        match self {
            Self::En => "EN",
            Self::De => "DE-NA",
        }
    }
}

/// 🗺️ Trait for national annex parameter overrides.
pub trait NationalAnnex {
    fn choice(&self) -> AnnexChoice;
    fn gamma_g(&self) -> f64;
    fn gamma_q(&self) -> f64;
    fn psi_0(&self, category: &str) -> f64;
    fn psi_1(&self, category: &str) -> f64;
    fn psi_2(&self, category: &str) -> f64;
}
// #endregion 🔖Annex

// #region 🔖Shared
/// ⚖️ Limit state per EN 1990.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LimitState {
    Uls,
    Sls,
    Als,
    Fls,
}

/// ⏱️ Load duration class for timber and similar materials.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LoadDuration {
    Permanent,
    Long,
    Medium,
    Short,
    Instantaneous,
}

/// 🌡️ Reference climate zone for thermal norms (Germany).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClimateZoneDe {
    Zone1,
    Zone2,
    Zone3,
    Zone4,
}

impl ClimateZoneDe {
    pub fn design_external_temperature_c(self) -> f64 {
        match self {
            Self::Zone1 => -16.0,
            Self::Zone2 => -14.0,
            Self::Zone3 => -12.0,
            Self::Zone4 => -10.0,
        }
    }
}
// #endregion 🔖Shared

// #region 🔖Error
/// ⚠️ Norm computation error.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NormError {
    IncompleteInput { field: String },
    OutOfScope { clause: ClauseId },
    InvalidValue { field: String, reason: String },
}

impl fmt::Display for NormError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IncompleteInput { field } => write!(f, "incomplete input: {field}"),
            Self::OutOfScope { clause } => write!(f, "out of scope: {clause}"),
            Self::InvalidValue { field, reason } => {
                write!(f, "invalid {field}: {reason}")
            }
        }
    }
}

impl std::error::Error for NormError {}
// #endregion 🔖Error

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_result_passes_when_utilization_below_one() {
        let clause = ClauseId::new("EN 1990", "§6.4", "6.10");
        let result = CheckResult::from_utilization(
            clause,
            Quantity::stress_mpa(250.0),
            Quantity::stress_mpa(300.0),
            "ULS stress check",
            AnnexChoice::De,
        );
        assert_eq!(result.status, CheckStatus::Pass);
        assert!(result.utilization < 1.0);
    }

    #[test]
    fn report_all_pass_ignores_not_applicable() {
        let mut report = CheckReport::default();
        report.push(CheckResult::pass(
            ClauseId::new("DIN 4108-2", "§4", "4.1"),
            Quantity::u_value_w_m2k(0.24),
            Quantity::u_value_w_m2k(0.28),
            0.86,
            "U-value",
            AnnexChoice::En,
        ));
        assert!(report.all_pass());
    }
}
