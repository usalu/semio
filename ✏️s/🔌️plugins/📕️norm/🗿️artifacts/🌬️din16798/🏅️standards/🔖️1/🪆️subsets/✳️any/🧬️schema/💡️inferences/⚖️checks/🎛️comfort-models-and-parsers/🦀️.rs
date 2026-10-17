//! 🎛️ DIN EN 16798 comfort, ventilation system and method models with the usage, category and pollution parsers.

use super::*;

/// 🎛️ Comfort evaluation model wire values (EN 16798-1 §7 / Annex A) — single source of truth.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComfortModel {
    FixedHvac,
    Adaptive,
}

impl ComfortModel {
    pub fn parse(s: &str) -> Self {
        match s.trim() {
            "adaptive" => Self::Adaptive,
            _ => Self::FixedHvac,
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FixedHvac => "fixed_hvac",
            Self::Adaptive => "adaptive",
        }
    }
}

/// 🌬️ Ventilation system type wire values (EN 16798-3) — single source of truth.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VentSystemType {
    CentralMech,
    DecentralMech,
    Natural,
}

impl VentSystemType {
    pub fn parse(s: &str) -> Self {
        match s.trim() {
            "decentral_mech" => Self::DecentralMech,
            "natural" => Self::Natural,
            _ => Self::CentralMech,
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CentralMech => "central_mech",
            Self::DecentralMech => "decentral_mech",
            Self::Natural => "natural",
        }
    }
}

/// 🌬️ EN 16798-1 §6.3 ventilation design method wire values.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VentMethod {
    Method1PerceivedAirQuality,
    Method2LimitConcentration,
    Method3PredefinedRates,
}

impl VentMethod {
    pub fn parse(s: &str) -> Self {
        match s.trim() {
            "method_2_limit_concentration" => Self::Method2LimitConcentration,
            "method_3_predefined_rates" => Self::Method3PredefinedRates,
            _ => Self::Method1PerceivedAirQuality,
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Method1PerceivedAirQuality => "method_1_perceived_air_quality",
            Self::Method2LimitConcentration => "method_2_limit_concentration",
            Self::Method3PredefinedRates => "method_3_predefined_rates",
        }
    }
}

pub(super) fn loc(en: impl Into<String>, de: impl Into<String>) -> LocalizedCopy {
    LocalizedCopy::new(en, de)
}

pub(super) fn zone_subject(zone: &ZoneDocument, path: String) -> SubjectRef {
    SubjectRef::new(
        &zone.id,
        path,
        loc(
            format!("Zone: {}", zone.name),
            format!("Raumzone: {}", zone.name),
        ),
    )
}

pub(super) fn vent_subject(vent: &VentSystemDocument, path: String) -> SubjectRef {
    SubjectRef::new(
        &vent.id,
        path,
        loc(
            format!("Ventilation system «{}»", vent.name),
            format!("Lüftungsanlage «{}»", vent.name),
        ),
    )
}

pub fn parse_usage(s: &str) -> OccupancyType {
    match s.to_ascii_lowercase().as_str() {
        "residential" => OccupancyType::Residential,
        "classroom" => OccupancyType::Classroom,
        "retail" => OccupancyType::Retail,
        "meeting" => OccupancyType::Meeting,
        "kitchen" => OccupancyType::Kitchen,
        "corridor" => OccupancyType::Corridor,
        _ => OccupancyType::Office,
    }
}

/// 🏷️ EN 16798-1 IEQ category I–IV.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComfortCategory { I, II, III, IV }

pub fn parse_category(s: &str) -> ComfortCategory {
    match s.trim().to_ascii_uppercase().as_str() {
        "I" | "1" => ComfortCategory::I,
        "III" | "3" => ComfortCategory::III,
        "IV" | "4" => ComfortCategory::IV,
        _ => ComfortCategory::II,
    }
}

/// 🏭️ Building pollution class for ventilation rate method (EN 16798-1 Table B.7).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PollutionClass { VeryLow, Low, NonLow }

pub fn parse_pollution(s: &str) -> PollutionClass {
    match s.to_ascii_lowercase().replace('-', "_").as_str() {
        "very_low" | "verylow" => PollutionClass::VeryLow,
        "non_low" | "nonlow" | "high" => PollutionClass::NonLow,
        _ => PollutionClass::Low,
    }
}
