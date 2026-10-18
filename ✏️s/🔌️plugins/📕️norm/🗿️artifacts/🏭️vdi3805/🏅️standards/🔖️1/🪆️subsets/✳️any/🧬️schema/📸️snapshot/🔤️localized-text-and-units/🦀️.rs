//! 🔤️ Localized text and units of the VDI 3805 snapshot: the bilingual text builders, the local quantity kind tag with its conversion from the compliance quantity kind and the unit record. See [`super`].

use semio_s_plugin_norm_compliance::{LocalizedText, QuantityKind};

/// 🇩🇪️🇬🇧️ Builds a two-entry `de`+`en` `LocalizedText` list — this artifact's product titles were
/// always exactly a German+English pair before the `LocalizedText` unification; a `Vec` (matching
/// `iso16757::Names.alternatives`'s established `Vec<LocalizedText>` convention) is genuinely
/// more general than the old hardcoded-bilingual struct, not just a rename.
pub fn bilingual(de: impl Into<String>, en: impl Into<String>) -> Vec<LocalizedText> {
    vec![LocalizedText::new("de", de), LocalizedText::new("en", en)]
}

/// 🔎️ Reads the text for one `locale` out of a `Vec<LocalizedText>`, `""` if absent.
pub fn text_in(variants: &[LocalizedText], locale: &str) -> String {
    variants.iter().find(|t| t.locale == locale).map(|t| t.text.clone()).unwrap_or_default()
}

/// 🔒️ A `QuantityKind` tag mirroring `crate::document::QuantityKind`'s 19 variants, kept locally: the DSL
/// engine's `DslField` binding can only be derived for a type/trait pair with a local half (orphan
/// rule), and `crate::document::QuantityKind` doesn't derive `dsl::DslScalar` itself. Converted at the
/// `VdiUnit` boundary via `From`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub enum VdiQuantityKind {
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
    #[dsl(key = "thermalConductivity")]
    ThermalConductivity,
    #[dsl(key = "thermalResistance")]
    ThermalResistance,
    #[dsl(key = "heatTransferCoefficient")]
    HeatTransferCoefficient,
    #[dsl(key = "airPermeability")]
    AirPermeability,
    #[dsl(key = "ventilationRate")]
    VentilationRate,
    Acceleration,
}

impl From<QuantityKind> for VdiQuantityKind {
    fn from(value: QuantityKind) -> Self {
        match value {
            QuantityKind::Dimensionless => VdiQuantityKind::Dimensionless,
            QuantityKind::Length => VdiQuantityKind::Length,
            QuantityKind::Area => VdiQuantityKind::Area,
            QuantityKind::Volume => VdiQuantityKind::Volume,
            QuantityKind::Mass => VdiQuantityKind::Mass,
            QuantityKind::Time => VdiQuantityKind::Time,
            QuantityKind::Temperature => VdiQuantityKind::Temperature,
            QuantityKind::Force => VdiQuantityKind::Force,
            QuantityKind::Pressure => VdiQuantityKind::Pressure,
            QuantityKind::Stress => VdiQuantityKind::Stress,
            QuantityKind::Moment => VdiQuantityKind::Moment,
            QuantityKind::Energy => VdiQuantityKind::Energy,
            QuantityKind::Power => VdiQuantityKind::Power,
            QuantityKind::ThermalConductivity => VdiQuantityKind::ThermalConductivity,
            QuantityKind::ThermalResistance => VdiQuantityKind::ThermalResistance,
            QuantityKind::HeatTransferCoefficient => VdiQuantityKind::HeatTransferCoefficient,
            QuantityKind::AirPermeability => VdiQuantityKind::AirPermeability,
            QuantityKind::VentilationRate => VdiQuantityKind::VentilationRate,
            QuantityKind::Acceleration => VdiQuantityKind::Acceleration,
        }
    }
}

/// 📐️ VDI 3805 unit with absolute vs delta semantics.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct VdiUnit {
    pub symbol: String,
    pub kind: VdiQuantityKind,
    pub delta: bool,
    pub si_factor: f64,
}

impl VdiUnit {
    pub fn absolute(symbol: impl Into<String>, kind: VdiQuantityKind, si_factor: f64) -> Self {
        Self { symbol: symbol.into(), kind, delta: false, si_factor }
    }

    pub fn delta(symbol: impl Into<String>, kind: VdiQuantityKind, si_factor: f64) -> Self {
        Self { symbol: symbol.into(), kind, delta: true, si_factor }
    }
}
