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

    pub fn from_minimum(
        clause: ClauseId,
        computed: Quantity,
        minimum: Quantity,
        message: impl Into<String>,
        annex: AnnexChoice,
    ) -> Self {
        let passes = computed.value >= minimum.value;
        let utilization = if passes {
            minimum.value / computed.value.max(minimum.value)
        } else {
            computed.value / minimum.value.max(f64::EPSILON)
        };
        if passes {
            Self::pass(clause, computed, minimum, utilization, message, annex)
        } else {
            Self::fail(clause, computed, minimum, utilization, message, annex)
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
    fn gamma_m(&self, _material: &str) -> f64 {
        1.0
    }
    fn gamma_r(&self) -> f64 {
        1.0
    }
    fn xi(&self, _category: &str) -> f64 {
        1.0
    }
    fn psi_0(&self, category: &str) -> f64;
    fn psi_1(&self, category: &str) -> f64;
    fn psi_2(&self, category: &str) -> f64;
}
// #endregion 🔖Annex

// #region 🔖Tables
/// 📊 One-dimensional table entry for norm lookups.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TableEntry1D {
    pub x: f64,
    pub y: f64,
}

/// 🔍 Linear interpolation in a sorted 1D table.
pub fn table_lookup_linear(table: &[TableEntry1D], x: f64) -> f64 {
    if table.is_empty() {
        return 0.0;
    }
    if x <= table[0].x {
        return table[0].y;
    }
    if x >= table[table.len() - 1].x {
        return table[table.len() - 1].y;
    }
    for w in table.windows(2) {
        if x >= w[0].x && x <= w[1].x {
            let t = (x - w[0].x) / (w[1].x - w[0].x);
            return w[0].y + t * (w[1].y - w[0].y);
        }
    }
    table[table.len() - 1].y
}

/// 🔍 Bilinear interpolation on a regular grid.
pub fn table_lookup_bilinear(
    x: f64,
    y: f64,
    x_vals: &[f64],
    y_vals: &[f64],
    z: &[f64],
) -> f64 {
    let nx = x_vals.len();
    let ny = y_vals.len();
    if nx == 0 || ny == 0 || z.len() < nx * ny {
        return 0.0;
    }
    let xi = x_vals.iter().position(|&v| x <= v).unwrap_or(nx - 1).max(1);
    let yi = y_vals.iter().position(|&v| y <= v).unwrap_or(ny - 1).max(1);
    let x0 = x_vals[xi - 1];
    let x1 = x_vals[xi.min(nx - 1)];
    let y0 = y_vals[yi - 1];
    let y1 = y_vals[yi.min(ny - 1)];
    let tx = if (x1 - x0).abs() < f64::EPSILON {
        0.0
    } else {
        ((x - x0) / (x1 - x0)).clamp(0.0, 1.0)
    };
    let ty = if (y1 - y0).abs() < f64::EPSILON {
        0.0
    } else {
        ((y - y0) / (y1 - y0)).clamp(0.0, 1.0)
    };
    let z00 = z[(yi - 1) * nx + (xi - 1)];
    let z10 = z[(yi - 1) * nx + xi.min(nx - 1)];
    let z01 = z[yi.min(ny - 1) * nx + (xi - 1)];
    let z11 = z[yi.min(ny - 1) * nx + xi.min(nx - 1)];
    let z0 = z00 + tx * (z10 - z00);
    let z1 = z01 + tx * (z11 - z01);
    z0 + ty * (z1 - z0)
}
// #endregion 🔖Tables

// #region 🔖DesignSituation
/// 🏗️ Design situation per EN 1990 Table A1.1.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DesignSituation {
    Persistent,
    Transient,
    Accidental,
    Seismic,
}

/// 📋 Consequence class per EN 1990.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConsequenceClass {
    Cc1,
    Cc2,
    Cc3,
}

impl ConsequenceClass {
    pub fn as_u8(self) -> u8 {
        match self {
            Self::Cc1 => 1,
            Self::Cc2 => 2,
            Self::Cc3 => 3,
        }
    }
}

/// 📊 Variable action category per EN 1991-1-1 Table 6.1.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImposedCategory {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl ImposedCategory {
    pub fn q_k_kn_m2(self) -> f64 {
        match self {
            Self::A => 2.0,
            Self::B => 2.5,
            Self::C => 3.0,
            Self::D => 4.0,
            Self::E => 5.0,
            Self::F => 3.0,
            Self::G => 5.0,
            Self::H => 20.0,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::A => "residential",
            Self::B => "office",
            Self::C => "congregation",
            Self::D => "retail",
            Self::E => "storage",
            Self::F => "traffic_light",
            Self::G => "traffic_heavy",
            Self::H => "roof",
        }
    }
}
// #endregion 🔖DesignSituation

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

    pub fn summer_design_temperature_c(self) -> f64 {
        match self {
            Self::Zone1 => 26.0,
            Self::Zone2 => 28.0,
            Self::Zone3 => 30.0,
            Self::Zone4 => 32.0,
        }
    }

    pub fn heating_degree_days(self) -> f64 {
        match self {
            Self::Zone1 => 3800.0,
            Self::Zone2 => 3200.0,
            Self::Zone3 => 2600.0,
            Self::Zone4 => 2000.0,
        }
    }
}

/// 🏠 Occupancy type for indoor environment norms.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum OccupancyType {
    Residential,
    Office,
    Classroom,
    Retail,
    Meeting,
    Kitchen,
    Corridor,
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
    fn table_lookup_linear_interpolates() {
        let table = [
            TableEntry1D { x: 0.0, y: 1.0 },
            TableEntry1D { x: 10.0, y: 2.0 },
        ];
        assert!((table_lookup_linear(&table, 5.0) - 1.5).abs() < 1e-9);
    }

    #[test]
    fn check_minimum_passes_when_above_threshold() {
        let result = CheckResult::from_minimum(
            ClauseId::new("DIN 4108-3", "§6", "6.1"),
            Quantity::new(QuantityKind::Dimensionless, 0.8),
            Quantity::new(QuantityKind::Dimensionless, 0.25),
            "f_Rsi",
            AnnexChoice::De,
        );
        assert_eq!(result.status, CheckStatus::Pass);
    }
}
