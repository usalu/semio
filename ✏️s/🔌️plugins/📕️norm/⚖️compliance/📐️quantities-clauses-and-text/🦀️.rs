//! 📐️ Quantities, clauses and localized text of the norm compliance core. See [`super`].

use super::*;

// #region 🔖️Quantity
/// 📐️ Physical quantity kind for SI-normalized norm computations.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(any(test, feature = "compliance-testing"), derive(serde::Serialize, serde::Deserialize))]
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
    Acceleration,
}

/// 📊️ A scalar value tagged with its physical quantity kind (SI units).
#[derive(Clone, Copy, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(any(test, feature = "compliance-testing"), derive(serde::Serialize, serde::Deserialize))]
pub struct Quantity {
    pub kind: QuantityKind,
    pub value: f64,
}

impl Quantity {
    pub fn new(kind: QuantityKind, value: f64) -> Self {
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

    pub fn acceleration_m_s2(value: f64) -> Self {
        Self::new(QuantityKind::Acceleration, value)
    }
}
// #endregion 🔖️Quantity

// #region 🔖️Clause
/// 📑️ Stable clause identifier within a norm family (e.g. `EN 1992-1-1` §6.1).
#[derive(Clone, Debug, PartialEq, Eq, Hash, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(any(test, feature = "compliance-testing"), derive(serde::Serialize, serde::Deserialize))]
pub struct ClauseId {
    pub family: String,
    pub part: String,
    pub section: String,
}

impl ClauseId {
    pub fn new(family: impl Into<String>, part: impl Into<String>, section: impl Into<String>) -> Self {
        Self { family: family.into(), part: part.into(), section: section.into() }
    }
}

impl fmt::Display for ClauseId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} §{}", self.family, self.part, self.section)
    }
}
// #endregion 🔖️Clause

// #region 🔖️LocalizedText
/// 🌐️ Locale-tagged text — the ONE canonical shape shared by every norm artifact (kills the
/// `LocalizedText` duplication formerly hand-maintained separately in `vdi3805` and `iso16757`,
/// see UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM's `📓️design-full-plan.md` §4 norm line). Structurally
/// isomorphic to stdio's `✳️text` subset's `SemioTextRun{language,content,..}` (that subset's own
/// doc comment: "Absorbs the duplicated `LocalizedText` types that currently exist twice inside
/// the norm plugin") — kept as a local plain value type here rather than a composed
/// `store::os_store::artifact_child::ArtifactChild<SemioTextSnapshot>` because it is nested by the DOZENS as an ordinary
/// leaf field inside `Names`/`Subject`/`CatalogueProduct` etc., not a top-level content slot; a
/// composed child handle is for a single large content slot with its own working-scene cache
/// (`📓️migration-recipe.md` §1/§3), not a value type reused pervasively as a struct field.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "compliance-testing"), derive(serde::Serialize, serde::Deserialize))]
pub struct LocalizedText {
    pub locale: String,
    pub text: String,
}

impl LocalizedText {
    pub fn new(locale: impl Into<String>, text: impl Into<String>) -> Self {
        Self { locale: locale.into(), text: text.into() }
    }
}
// #endregion 🔖️LocalizedText
