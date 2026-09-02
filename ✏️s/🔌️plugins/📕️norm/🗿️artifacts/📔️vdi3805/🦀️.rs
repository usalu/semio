//! 🔧️ VDI 3805 manufacturer product data for building services: Part 1 + sheets 2–100 — document entities.

pub use crate::artifacts::vdi3805::schema::snapshot::Vdi3805Snapshot;

use crate::document::{NormError, QuantityKind};
use std::collections::{BTreeMap, BTreeSet};

// #region Shared
/// 🌐️ Locale-tagged text — re-exported from `crate::document`, the single canonical definition
/// shared across every norm artifact (kills this type's former duplicate here and in `iso16757`;
/// see `crate::document`'s `🔖️LocalizedText` region for the full rationale).
pub use crate::document::LocalizedText;

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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, dsl::DslScalar, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
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

impl From<VdiQuantityKind> for QuantityKind {
    fn from(value: VdiQuantityKind) -> Self {
        match value {
            VdiQuantityKind::Dimensionless => QuantityKind::Dimensionless,
            VdiQuantityKind::Length => QuantityKind::Length,
            VdiQuantityKind::Area => QuantityKind::Area,
            VdiQuantityKind::Volume => QuantityKind::Volume,
            VdiQuantityKind::Mass => QuantityKind::Mass,
            VdiQuantityKind::Time => QuantityKind::Time,
            VdiQuantityKind::Temperature => QuantityKind::Temperature,
            VdiQuantityKind::Force => QuantityKind::Force,
            VdiQuantityKind::Pressure => QuantityKind::Pressure,
            VdiQuantityKind::Stress => QuantityKind::Stress,
            VdiQuantityKind::Moment => QuantityKind::Moment,
            VdiQuantityKind::Energy => QuantityKind::Energy,
            VdiQuantityKind::Power => QuantityKind::Power,
            VdiQuantityKind::ThermalConductivity => QuantityKind::ThermalConductivity,
            VdiQuantityKind::ThermalResistance => QuantityKind::ThermalResistance,
            VdiQuantityKind::HeatTransferCoefficient => QuantityKind::HeatTransferCoefficient,
            VdiQuantityKind::AirPermeability => QuantityKind::AirPermeability,
            VdiQuantityKind::VentilationRate => QuantityKind::VentilationRate,
            VdiQuantityKind::Acceleration => QuantityKind::Acceleration,
        }
    }
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
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
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

/// 🔢️ Typed manufacturer value.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(tag = "kind", rename_all = "camelCase"))]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum VdiValue {
    Boolean { value: bool },
    Integer { value: i64 },
    Decimal { value: f64, unit: Option<VdiUnit> },
    Text { value: String },
    Enumeration { code: String },
    Range { min: f64, max: f64, unit: Option<VdiUnit> },
    List { items: Vec<VdiValue> },
    Null,
}

/// 🔗️ Hand `DslField` bridge for `VdiValue`: a deeply serde-tagged data enum embedded as a
/// `BTreeMap` VALUE type (`Configuration.parameters`), which mechanically requires `DslField` (map
/// values bind through `DslField`, not `DslVariants`) — `#[derive(dsl::DslEnum)]` only produces
/// `DslVariants`, so it can't satisfy that site. Binds through `Shape::Value` (the engine's existing
/// serde_json escape hatch), reusing the `Serialize`/`Deserialize` this type already has.
impl dsl::DslField for VdiValue {
    fn shape() -> dsl::Shape {
        dsl::Shape::Value
    }
    fn to_value(&self) -> dsl::FieldValue {
        dsl::FieldValue::Value(dsl::to_dsl_value(self).expect("VdiValue always serializes to DslValue"))
    }
    fn from_value(value: &dsl::FieldValue) -> Result<Self, String> {
        match value {
            dsl::FieldValue::Value(dsl_value) => {
                let normalized = store::pack_rt::renormalize_whole_number_floats(dsl_value.clone());
                dsl::from_dsl_value(normalized)
            }
            other => Err(format!("expected Value, found {other:?}")),
        }
    }
}

/// 🧩️ Lossless extension bag for unknown fields.
#[derive(Clone, Debug, Default, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub struct ExtensionBag {
    pub fields: BTreeMap<String, dsl::DslValue>,
}

/// 🆔️ Product identity within a manufacturer catalogue.
#[derive(Clone, Debug, PartialEq, Eq, Hash, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub struct ProductIdentity {
    pub manufacturer_code: String,
    pub product_group: String,
    pub article_number: String,
}

/// 🏭️ Manufacturer file header and payload references.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub struct ManufacturerFile {
    pub header_version: String,
    pub manufacturer: String,
    pub building_system_number: BuildingSystemNumber,
    pub created: String,
    pub charset: String,
    pub record_count: u32,
    pub extensions: ExtensionBag,
}

/// 🔗️ Accessory relationship between products.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub struct AccessoryLink {
    pub accessory_id: String,
    pub required: bool,
    pub quantity: u32,
}

/// 🧱️ Composition relationship (`hasPart`).
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub struct CompositionLink {
    pub component_id: String,
    pub quantity: u32,
}

/// 🔒️ Security limits for untrusted manufacturer files.
#[derive(Clone, Copy, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub struct SecurityLimits {
    pub max_file_bytes: usize,
    pub max_records: usize,
    pub max_field_length: usize,
    pub max_nesting_depth: usize,
}

impl Default for SecurityLimits {
    fn default() -> Self {
        Self { max_file_bytes: 16 * 1024 * 1024, max_records: 100_000, max_field_length: 8_192, max_nesting_depth: 32 }
    }
}

impl SecurityLimits {
    pub fn validate_text(&self, text: &str) -> Result<(), NormError> {
        if text.len() > self.max_file_bytes {
            return Err(NormError::InvalidValue { field: "file".into(), reason: format!("exceeds {} bytes", self.max_file_bytes) });
        }
        Ok(())
    }
}
// #endregion Shared

// #region Schema
/// 📄️ Sheet identifier (1…100).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(transparent)]
pub struct SheetId(pub u16);

impl SheetId {
    pub fn part_str(self) -> String {
        format!("{}", self.0)
    }
}

/// 🔗️ Hand `DslField` bridge for `SheetId`: a tuple ("newtype") struct has no named fields for
/// `#[derive(dsl::DslRecord)]` to enumerate, so it binds directly as `Shape::UInt` instead of
/// changing its public tuple shape (used pervasively as `.0` across this crate).
impl dsl::DslField for SheetId {
    fn shape() -> dsl::Shape {
        dsl::Shape::UInt
    }
    fn to_value(&self) -> dsl::FieldValue {
        dsl::FieldValue::UInt(self.0 as u64)
    }
    fn from_value(value: &dsl::FieldValue) -> Result<Self, String> {
        match value {
            dsl::FieldValue::UInt(v) => Ok(SheetId(*v as u16)),
            other => Err(format!("expected UInt, found {other:?}")),
        }
    }
}

/// 📅️ Edition identifier (year + month).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub struct EditionId {
    pub year: u16,
    pub month: u8,
}

impl EditionId {
    pub fn new(year: u16, month: u8) -> Self {
        Self { year, month }
    }

    pub fn key(self) -> u32 {
        (self.year as u32) * 100 + self.month as u32
    }
}

/// 📊️ Schema lifecycle status.
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub enum SchemaStatus {
    Published,
    Checked,
    Draft,
    Project,
    Withdrawn,
    Superseded,
    HistoricalProposal,
    Reserved,
}

impl SchemaStatus {
    pub fn is_operative(self) -> bool {
        matches!(self, Self::Published | Self::Checked)
    }
}

/// 🏷️ Building-services domain filter.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub enum Domain {
    Heating,
    Ventilation,
    Sanitary,
    BuildingAutomation,
    Electrical,
    Generic,
}

/// 📋️ Sheet registry entry.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SheetEntry {
    pub id: SheetId,
    pub title_de: &'static str,
    pub title_en: &'static str,
    pub status: SchemaStatus,
    pub domains: &'static [Domain],
    pub part1_edition: EditionId,
    pub current_edition: EditionId,
}

/// 🩹️ Correction overlay descriptor.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CorrectionOverlay {
    pub id: &'static str,
    pub sheet: SheetId,
    pub base_edition: EditionId,
    pub effective: EditionId,
    pub summary_de: &'static str,
    pub summary_en: &'static str,
}

impl CorrectionOverlay {
    pub fn applies_as_of(&self, as_of: EditionId) -> bool {
        as_of.key() >= self.effective.key()
    }
}

/// 📚️ Runtime edition registry.
#[derive(Clone, Debug, PartialEq)]
pub struct SchemaCatalog {
    sheets: Vec<SheetEntry>,
    corrections: &'static [CorrectionOverlay],
    filter: Option<SchemaStatus>,
}

/// 📚️ The full 100-sheet VDI 3805 registry (Part 1 + sheets 2–100), 1:1 from the norm's sheet index.
pub const SHEET_ENTRIES: &[SheetEntry] = &[
    SheetEntry { id: SheetId(1), title_de: "Grundlagen", title_en: "Fundamentals", status: SchemaStatus::Published, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(2), title_de: "Stellventile Heizung", title_en: "Control valves heating", status: SchemaStatus::Published, domains: &[Domain::Heating], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(3), title_de: "Heizkörper", title_en: "Radiators", status: SchemaStatus::Published, domains: &[Domain::Heating], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(4), title_de: "Rohrleitungen Heizung", title_en: "Pipes heating", status: SchemaStatus::Published, domains: &[Domain::Heating], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(5), title_de: "Pumpen Heizung", title_en: "Pumps heating", status: SchemaStatus::Published, domains: &[Domain::Heating], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(6), title_de: "Wärmeerzeuger", title_en: "Heat generators", status: SchemaStatus::Published, domains: &[Domain::Heating], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(7), title_de: "Speicher", title_en: "Storage tanks", status: SchemaStatus::Published, domains: &[Domain::Heating], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(8), title_de: "Armaturen Heizung", title_en: "Fittings heating", status: SchemaStatus::Published, domains: &[Domain::Heating], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2023, 3) },
    SheetEntry { id: SheetId(9), title_de: "Regelung Heizung", title_en: "Controls heating", status: SchemaStatus::Published, domains: &[Domain::Heating], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(10), title_de: "Verteiler Heizung", title_en: "Manifolds heating", status: SchemaStatus::Published, domains: &[Domain::Heating], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2023, 3) },
    SheetEntry { id: SheetId(11), title_de: "Messgeräte Heizung", title_en: "Meters heating", status: SchemaStatus::Published, domains: &[Domain::Heating], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry {
        id: SheetId(12),
        title_de: "Historischer Vorschlag A",
        title_en: "Historical proposal A",
        status: SchemaStatus::HistoricalProposal,
        domains: &[Domain::Generic],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2018, 6),
    },
    SheetEntry {
        id: SheetId(13),
        title_de: "Historischer Vorschlag B",
        title_en: "Historical proposal B",
        status: SchemaStatus::HistoricalProposal,
        domains: &[Domain::Generic],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2018, 6),
    },
    SheetEntry { id: SheetId(14), title_de: "Ventile Lüftung", title_en: "Valves ventilation", status: SchemaStatus::Published, domains: &[Domain::Ventilation], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2023, 3) },
    SheetEntry { id: SheetId(15), title_de: "Blatt 15", title_en: "Sheet 15", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(16), title_de: "Luftdurchlässe", title_en: "Air terminals", status: SchemaStatus::Published, domains: &[Domain::Ventilation], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(17), title_de: "Kanäle Lüftung", title_en: "Ducts ventilation", status: SchemaStatus::Published, domains: &[Domain::Ventilation], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(18), title_de: "Blatt 18", title_en: "Sheet 18", status: SchemaStatus::Published, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2023, 3) },
    SheetEntry { id: SheetId(19), title_de: "Filter Lüftung", title_en: "Filters ventilation", status: SchemaStatus::Published, domains: &[Domain::Ventilation], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(20), title_de: "Wärmerückgewinnung", title_en: "Heat recovery", status: SchemaStatus::Published, domains: &[Domain::Ventilation], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(21), title_de: "Sanitärarmaturen", title_en: "Sanitary fittings", status: SchemaStatus::Published, domains: &[Domain::Sanitary], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(22), title_de: "Rohrleitungen Sanitär", title_en: "Pipes sanitary", status: SchemaStatus::Published, domains: &[Domain::Sanitary], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(23), title_de: "Pumpen Sanitär", title_en: "Pumps sanitary", status: SchemaStatus::Published, domains: &[Domain::Sanitary], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(24), title_de: "Speicher Sanitär", title_en: "Storage sanitary", status: SchemaStatus::Published, domains: &[Domain::Sanitary], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry {
        id: SheetId(25),
        title_de: "Historischer Vorschlag Sanitär",
        title_en: "Historical proposal sanitary",
        status: SchemaStatus::HistoricalProposal,
        domains: &[Domain::Generic],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2018, 6),
    },
    SheetEntry { id: SheetId(26), title_de: "Regelung Sanitär", title_en: "Controls sanitary", status: SchemaStatus::Published, domains: &[Domain::Sanitary], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(27), title_de: "Messgeräte Sanitär", title_en: "Meters sanitary", status: SchemaStatus::Published, domains: &[Domain::Sanitary], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry {
        id: SheetId(28),
        title_de: "Gebäudeautomation",
        title_en: "Building automation",
        status: SchemaStatus::Published,
        domains: &[Domain::BuildingAutomation],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry { id: SheetId(29), title_de: "Elektro Komponenten", title_en: "Electrical components", status: SchemaStatus::Published, domains: &[Domain::Electrical], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(30), title_de: "Blatt 30", title_en: "Sheet 30", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(31), title_de: "Blatt 31", title_en: "Sheet 31", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(32), title_de: "Kältemaschinen", title_en: "Chillers", status: SchemaStatus::Published, domains: &[Domain::Heating], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(33), title_de: "Kühldecken", title_en: "Chilled ceilings", status: SchemaStatus::Published, domains: &[Domain::Heating], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2023, 3) },
    SheetEntry { id: SheetId(34), title_de: "Konvektoren", title_en: "Convectors", status: SchemaStatus::Published, domains: &[Domain::Heating], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(35), title_de: "Fußbodenheizung", title_en: "Underfloor heating", status: SchemaStatus::Published, domains: &[Domain::Heating], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(36), title_de: "Blatt 36", title_en: "Sheet 36", status: SchemaStatus::Published, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2023, 3) },
    SheetEntry { id: SheetId(37), title_de: "Blatt 37", title_en: "Sheet 37", status: SchemaStatus::Published, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2023, 3) },
    SheetEntry { id: SheetId(38), title_de: "Schalldämpfer", title_en: "Silencers", status: SchemaStatus::Published, domains: &[Domain::Ventilation], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(39), title_de: "Blatt 39", title_en: "Sheet 39", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(40), title_de: "Klappen Lüftung", title_en: "Dampers ventilation", status: SchemaStatus::Published, domains: &[Domain::Ventilation], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2023, 3) },
    SheetEntry { id: SheetId(41), title_de: "Ventilatoren", title_en: "Fans", status: SchemaStatus::Published, domains: &[Domain::Ventilation], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(42), title_de: "VAV-Regler", title_en: "VAV controllers", status: SchemaStatus::Published, domains: &[Domain::Ventilation], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2023, 3) },
    SheetEntry { id: SheetId(43), title_de: "Wärmetauscher", title_en: "Heat exchangers", status: SchemaStatus::Published, domains: &[Domain::Heating], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(44), title_de: "Druckerhöhung", title_en: "Pressure boosting", status: SchemaStatus::Published, domains: &[Domain::Sanitary], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(45), title_de: "Entgasung", title_en: "Degassing", status: SchemaStatus::Published, domains: &[Domain::Heating], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(46), title_de: "Blatt 46", title_en: "Sheet 46", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(47), title_de: "Blatt 47", title_en: "Sheet 47", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(48), title_de: "Blatt 48", title_en: "Sheet 48", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(49), title_de: "Blatt 49", title_en: "Sheet 49", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(50), title_de: "Brandschutzklappen", title_en: "Fire dampers", status: SchemaStatus::Published, domains: &[Domain::Ventilation], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(51), title_de: "Rohrbegleitheizung", title_en: "Trace heating", status: SchemaStatus::Published, domains: &[Domain::Heating], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(52), title_de: "Solarthermie", title_en: "Solar thermal", status: SchemaStatus::Published, domains: &[Domain::Heating], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(53), title_de: "Wärmepumpen", title_en: "Heat pumps", status: SchemaStatus::Published, domains: &[Domain::Heating], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2023, 3) },
    SheetEntry { id: SheetId(54), title_de: "Befeuchtung", title_en: "Humidification", status: SchemaStatus::Published, domains: &[Domain::Ventilation], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(55), title_de: "Entfeuchtung", title_en: "Dehumidification", status: SchemaStatus::Published, domains: &[Domain::Ventilation], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(56), title_de: "Blatt 56", title_en: "Sheet 56", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(57), title_de: "Blatt 57", title_en: "Sheet 57", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(58), title_de: "Blatt 58", title_en: "Sheet 58", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(59), title_de: "Blatt 59", title_en: "Sheet 59", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(60), title_de: "Kompensatoren", title_en: "Compensators", status: SchemaStatus::Published, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(61), title_de: "Trennstellen", title_en: "Separation points", status: SchemaStatus::Published, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(62), title_de: "Schmutzfänger", title_en: "Strainers", status: SchemaStatus::Published, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(63), title_de: "Rückschlagventile", title_en: "Check valves", status: SchemaStatus::Published, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(64), title_de: "Sicherheitsventile", title_en: "Safety valves", status: SchemaStatus::Published, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(65), title_de: "Absperrarmaturen", title_en: "Shut-off fittings", status: SchemaStatus::Published, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(66), title_de: "Mischventile", title_en: "Mixing valves", status: SchemaStatus::Published, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(67), title_de: "Blatt 67", title_en: "Sheet 67", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(68), title_de: "Blatt 68", title_en: "Sheet 68", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(69), title_de: "Blatt 69", title_en: "Sheet 69", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(70), title_de: "Blatt 70", title_en: "Sheet 70", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(71), title_de: "Blatt 71", title_en: "Sheet 71", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(72), title_de: "Blatt 72", title_en: "Sheet 72", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(73), title_de: "Blatt 73", title_en: "Sheet 73", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(74), title_de: "Blatt 74", title_en: "Sheet 74", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(75), title_de: "Blatt 75", title_en: "Sheet 75", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(76), title_de: "Blatt 76", title_en: "Sheet 76", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(77), title_de: "Blatt 77", title_en: "Sheet 77", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(78), title_de: "Blatt 78", title_en: "Sheet 78", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(79), title_de: "Blatt 79", title_en: "Sheet 79", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(80), title_de: "Blatt 80", title_en: "Sheet 80", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(81), title_de: "Blatt 81", title_en: "Sheet 81", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(82), title_de: "Blatt 82", title_en: "Sheet 82", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(83), title_de: "Blatt 83", title_en: "Sheet 83", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(84), title_de: "Blatt 84", title_en: "Sheet 84", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(85), title_de: "Blatt 85", title_en: "Sheet 85", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(86), title_de: "Blatt 86", title_en: "Sheet 86", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(87), title_de: "Blatt 87", title_en: "Sheet 87", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(88), title_de: "Blatt 88", title_en: "Sheet 88", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(89), title_de: "Blatt 89", title_en: "Sheet 89", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(90), title_de: "Blatt 90", title_en: "Sheet 90", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(91), title_de: "Blatt 91", title_en: "Sheet 91", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(92), title_de: "Blatt 92", title_en: "Sheet 92", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(93), title_de: "Blatt 93", title_en: "Sheet 93", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(94), title_de: "Blatt 94", title_en: "Sheet 94", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(95), title_de: "Blatt 95", title_en: "Sheet 95", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(96), title_de: "Blatt 96", title_en: "Sheet 96", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(97), title_de: "Blatt 97", title_en: "Sheet 97", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(98), title_de: "Blatt 98", title_en: "Sheet 98", status: SchemaStatus::Reserved, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(99), title_de: "Erweiterungen", title_en: "Extensions", status: SchemaStatus::Published, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2022, 6) },
    SheetEntry { id: SheetId(100), title_de: "Profilübergreifend", title_en: "Cross-profile", status: SchemaStatus::Published, domains: &[Domain::Generic], part1_edition: EditionId::new(2022, 6), current_edition: EditionId::new(2023, 3) },
];

/// 🩹️ Correction overlays applicable to individual sheets.
pub const CORRECTION_OVERLAYS: &[CorrectionOverlay] = &[
    CorrectionOverlay { id: "part-02-corr-2022-12", sheet: SheetId(2), base_edition: EditionId::new(2022, 12), effective: EditionId::new(2023, 2), summary_de: "Korrektur Blatt 2", summary_en: "Correction sheet 2" },
    CorrectionOverlay { id: "part-03-corr-2022-11", sheet: SheetId(3), base_edition: EditionId::new(2022, 11), effective: EditionId::new(2023, 1), summary_de: "Korrektur Blatt 3", summary_en: "Correction sheet 3" },
    CorrectionOverlay { id: "part-04-corr-2022-10", sheet: SheetId(4), base_edition: EditionId::new(2022, 10), effective: EditionId::new(2022, 12), summary_de: "Korrektur Blatt 4", summary_en: "Correction sheet 4" },
    CorrectionOverlay { id: "part-05-corr-2021-09", sheet: SheetId(5), base_edition: EditionId::new(2021, 9), effective: EditionId::new(2021, 11), summary_de: "Korrektur Blatt 5", summary_en: "Correction sheet 5" },
    CorrectionOverlay { id: "part-06-corr-2021-08", sheet: SheetId(6), base_edition: EditionId::new(2021, 8), effective: EditionId::new(2021, 10), summary_de: "Korrektur Blatt 6", summary_en: "Correction sheet 6" },
    CorrectionOverlay { id: "part-07-corr-2021-07", sheet: SheetId(7), base_edition: EditionId::new(2021, 7), effective: EditionId::new(2021, 9), summary_de: "Korrektur Blatt 7", summary_en: "Correction sheet 7" },
    CorrectionOverlay { id: "part-08-corr-2020-06", sheet: SheetId(8), base_edition: EditionId::new(2020, 6), effective: EditionId::new(2020, 8), summary_de: "Korrektur Blatt 8", summary_en: "Correction sheet 8" },
    CorrectionOverlay { id: "part-09-corr-2020-05", sheet: SheetId(9), base_edition: EditionId::new(2020, 5), effective: EditionId::new(2020, 7), summary_de: "Korrektur Blatt 9", summary_en: "Correction sheet 9" },
    CorrectionOverlay { id: "part-10-corr-2020-04", sheet: SheetId(10), base_edition: EditionId::new(2020, 4), effective: EditionId::new(2020, 6), summary_de: "Korrektur Blatt 10", summary_en: "Correction sheet 10" },
    CorrectionOverlay { id: "part-11-corr-2019-03", sheet: SheetId(11), base_edition: EditionId::new(2019, 3), effective: EditionId::new(2019, 5), summary_de: "Korrektur Blatt 11", summary_en: "Correction sheet 11" },
    CorrectionOverlay { id: "part-12-corr-2019-02", sheet: SheetId(12), base_edition: EditionId::new(2019, 2), effective: EditionId::new(2019, 4), summary_de: "Korrektur Blatt 12", summary_en: "Correction sheet 12" },
    CorrectionOverlay { id: "part-13-corr-2019-01", sheet: SheetId(13), base_edition: EditionId::new(2019, 1), effective: EditionId::new(2019, 3), summary_de: "Korrektur Blatt 13", summary_en: "Correction sheet 13" },
    CorrectionOverlay { id: "part-14-corr-2018-12", sheet: SheetId(14), base_edition: EditionId::new(2018, 12), effective: EditionId::new(2019, 2), summary_de: "Korrektur Blatt 14", summary_en: "Correction sheet 14" },
    CorrectionOverlay { id: "part-15-corr-2018-11", sheet: SheetId(15), base_edition: EditionId::new(2018, 11), effective: EditionId::new(2019, 1), summary_de: "Korrektur Blatt 15", summary_en: "Correction sheet 15" },
    CorrectionOverlay { id: "part-16-corr-2018-10", sheet: SheetId(16), base_edition: EditionId::new(2018, 10), effective: EditionId::new(2018, 12), summary_de: "Korrektur Blatt 16", summary_en: "Correction sheet 16" },
    CorrectionOverlay { id: "part-17-corr-2017-09", sheet: SheetId(17), base_edition: EditionId::new(2017, 9), effective: EditionId::new(2017, 11), summary_de: "Korrektur Blatt 17", summary_en: "Correction sheet 17" },
    CorrectionOverlay { id: "part-18-corr-2017-08", sheet: SheetId(18), base_edition: EditionId::new(2017, 8), effective: EditionId::new(2017, 10), summary_de: "Korrektur Blatt 18", summary_en: "Correction sheet 18" },
    CorrectionOverlay { id: "part-19-corr-2017-07", sheet: SheetId(19), base_edition: EditionId::new(2017, 7), effective: EditionId::new(2017, 9), summary_de: "Korrektur Blatt 19", summary_en: "Correction sheet 19" },
    CorrectionOverlay { id: "part-20-corr-2016-06", sheet: SheetId(20), base_edition: EditionId::new(2016, 6), effective: EditionId::new(2016, 8), summary_de: "Korrektur Blatt 20", summary_en: "Correction sheet 20" },
    CorrectionOverlay { id: "part-21-corr-2016-05", sheet: SheetId(21), base_edition: EditionId::new(2016, 5), effective: EditionId::new(2016, 7), summary_de: "Korrektur Blatt 21", summary_en: "Correction sheet 21" },
    CorrectionOverlay { id: "part-22-corr-2016-04", sheet: SheetId(22), base_edition: EditionId::new(2016, 4), effective: EditionId::new(2016, 6), summary_de: "Korrektur Blatt 22", summary_en: "Correction sheet 22" },
    CorrectionOverlay { id: "part-23-corr-2015-03", sheet: SheetId(23), base_edition: EditionId::new(2015, 3), effective: EditionId::new(2015, 5), summary_de: "Korrektur Blatt 23", summary_en: "Correction sheet 23" },
    CorrectionOverlay { id: "part-24-corr-2015-02", sheet: SheetId(24), base_edition: EditionId::new(2015, 2), effective: EditionId::new(2015, 4), summary_de: "Korrektur Blatt 24", summary_en: "Correction sheet 24" },
    CorrectionOverlay { id: "part-25-corr-2015-01", sheet: SheetId(25), base_edition: EditionId::new(2015, 1), effective: EditionId::new(2015, 3), summary_de: "Korrektur Blatt 25", summary_en: "Correction sheet 25" },
    CorrectionOverlay { id: "part-26-corr-2014-12", sheet: SheetId(26), base_edition: EditionId::new(2014, 12), effective: EditionId::new(2015, 2), summary_de: "Korrektur Blatt 26", summary_en: "Correction sheet 26" },
    CorrectionOverlay { id: "part-27-corr-2014-11", sheet: SheetId(27), base_edition: EditionId::new(2014, 11), effective: EditionId::new(2015, 1), summary_de: "Korrektur Blatt 27", summary_en: "Correction sheet 27" },
    CorrectionOverlay { id: "part-28-corr-2014-10", sheet: SheetId(28), base_edition: EditionId::new(2014, 10), effective: EditionId::new(2014, 12), summary_de: "Korrektur Blatt 28", summary_en: "Correction sheet 28" },
    CorrectionOverlay { id: "part-29-corr-2013-09", sheet: SheetId(29), base_edition: EditionId::new(2013, 9), effective: EditionId::new(2013, 11), summary_de: "Korrektur Blatt 29", summary_en: "Correction sheet 29" },
    CorrectionOverlay { id: "part-30-corr-2013-08", sheet: SheetId(30), base_edition: EditionId::new(2013, 8), effective: EditionId::new(2013, 10), summary_de: "Korrektur Blatt 30", summary_en: "Correction sheet 30" },
    CorrectionOverlay { id: "part-31-corr-2013-07", sheet: SheetId(31), base_edition: EditionId::new(2013, 7), effective: EditionId::new(2013, 9), summary_de: "Korrektur Blatt 31", summary_en: "Correction sheet 31" },
    CorrectionOverlay { id: "part-32-corr-2019-07", sheet: SheetId(32), base_edition: EditionId::new(2019, 7), effective: EditionId::new(2019, 9), summary_de: "Korrektur Blatt 32", summary_en: "Correction sheet 32" },
];

impl SchemaCatalog {
    fn build(filter: Option<SchemaStatus>) -> Self {
        let sheets: Vec<SheetEntry> = SHEET_ENTRIES.iter().filter(|s| filter.is_none_or(|f| s.status == f)).copied().collect();
        Self { sheets, corrections: CORRECTION_OVERLAYS, filter }
    }

    pub fn current() -> Self {
        Self::build(None)
    }

    pub fn with_status(status: SchemaStatus) -> Self {
        Self::build(Some(status))
    }

    pub fn sheets(&self) -> &[SheetEntry] {
        &self.sheets
    }

    pub fn sheet(&self, id: SheetId) -> Option<&SheetEntry> {
        self.sheets.iter().find(|s| s.id == id)
    }

    pub fn sheets_in_domain(&self, domain: Domain) -> Vec<&SheetEntry> {
        self.sheets.iter().filter(|s| s.domains.contains(&domain)).collect()
    }

    pub fn operative_sheets(&self) -> Vec<&SheetEntry> {
        self.sheets.iter().filter(|s| s.status.is_operative()).collect()
    }

    pub fn corrections_for_sheet(&self, sheet: SheetId) -> Vec<&'static CorrectionOverlay> {
        self.corrections.iter().filter(|c| c.sheet == sheet).collect()
    }

    pub fn reserved_numbers(&self) -> BTreeSet<u16> {
        self.sheets.iter().filter(|s| s.status == SchemaStatus::Reserved).map(|s| s.id.0).collect()
    }
}
// #endregion Schema

// #region Part1
/// 🏗️ Parsed building-system number (Anlagenkennzeichen).
#[derive(Clone, Debug, PartialEq, Eq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub struct BuildingSystemNumber {
    pub system_code: String,
    pub subsystem: String,
    pub sequence: u32,
}

impl BuildingSystemNumber {
    pub fn parse(raw: &str) -> Result<Self, NormError> {
        let parts: Vec<&str> = raw.split('.').collect();
        if parts.len() != 3 {
            return Err(NormError::InvalidValue { field: "building_system_number".into(), reason: "expected SYS.SUB.NNN".into() });
        }
        let sequence: u32 = parts[2].parse().map_err(|_| NormError::InvalidValue { field: "building_system_number.sequence".into(), reason: "must be numeric".into() })?;
        Ok(Self { system_code: parts[0].into(), subsystem: parts[1].into(), sequence })
    }

    pub fn render(&self) -> String {
        format!("{}.{}.{}", self.system_code, self.subsystem, self.sequence)
    }
}

/// 📇️ Record family identifier (010…970.41).
#[derive(Clone, Debug, PartialEq, Eq, Hash, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(transparent)]
pub struct RecordFamilyId(pub String);

/// 🔗️ Hand `DslField` bridge for `RecordFamilyId`: a tuple ("newtype") struct has no named fields
/// for `#[derive(dsl::DslRecord)]` to enumerate, so it binds directly as `Shape::Text` instead of
/// changing its public tuple shape (used pervasively as `.0` across this crate).
impl dsl::DslField for RecordFamilyId {
    fn shape() -> dsl::Shape {
        dsl::Shape::Text
    }
    fn to_value(&self) -> dsl::FieldValue {
        dsl::FieldValue::Text(self.0.clone())
    }
    fn from_value(value: &dsl::FieldValue) -> Result<Self, String> {
        match value {
            dsl::FieldValue::Text(s) => Ok(RecordFamilyId(s.clone())),
            other => Err(format!("expected Text, found {other:?}")),
        }
    }
}

impl RecordFamilyId {
    pub const R010: &'static str = "010";
    pub const R020: &'static str = "020";
    pub const R030: &'static str = "030";
    pub const R040: &'static str = "040";
    pub const R050: &'static str = "050";
    pub const R060: &'static str = "060";
    pub const R070: &'static str = "070";
    pub const R080: &'static str = "080";
    pub const R090: &'static str = "090";
    pub const R100: &'static str = "100";
    pub const R110: &'static str = "110";
    pub const R120: &'static str = "120";
    pub const R130: &'static str = "130";
    pub const R140: &'static str = "140";
    pub const R150: &'static str = "150";
    pub const R160: &'static str = "160";
    pub const R170: &'static str = "170";
    pub const R180: &'static str = "180";
    pub const R190: &'static str = "190";
    pub const R200: &'static str = "200";
    pub const R210: &'static str = "210";
    pub const R220: &'static str = "220";
    pub const R230: &'static str = "230";
    pub const R240: &'static str = "240";
    pub const R250: &'static str = "250";
    pub const R260: &'static str = "260";
    pub const R270: &'static str = "270";
    pub const R280: &'static str = "280";
    pub const R290: &'static str = "290";
    pub const R300: &'static str = "300";
    pub const R310: &'static str = "310";
    pub const R320: &'static str = "320";
    pub const R330: &'static str = "330";
    pub const R340: &'static str = "340";
    pub const R350: &'static str = "350";
    pub const R360: &'static str = "360";
    pub const R370: &'static str = "370";
    pub const R380: &'static str = "380";
    pub const R390: &'static str = "390";
    pub const R400: &'static str = "400";
    pub const R410: &'static str = "410";
    pub const R420: &'static str = "420";
    pub const R430: &'static str = "430";
    pub const R440: &'static str = "440";
    pub const R450: &'static str = "450";
    pub const R460: &'static str = "460";
    pub const R470: &'static str = "470";
    pub const R480: &'static str = "480";
    pub const R490: &'static str = "490";
    pub const R500: &'static str = "500";
    pub const R510: &'static str = "510";
    pub const R520: &'static str = "520";
    pub const R530: &'static str = "530";
    pub const R540: &'static str = "540";
    pub const R550: &'static str = "550";
    pub const R560: &'static str = "560";
    pub const R570: &'static str = "570";
    pub const R580: &'static str = "580";
    pub const R590: &'static str = "590";
    pub const R600: &'static str = "600";
    pub const R610: &'static str = "610";
    pub const R620: &'static str = "620";
    pub const R630: &'static str = "630";
    pub const R640: &'static str = "640";
    pub const R650: &'static str = "650";
    pub const R660: &'static str = "660";
    pub const R670: &'static str = "670";
    pub const R680: &'static str = "680";
    pub const R690: &'static str = "690";
    pub const R700: &'static str = "700";
    pub const R710: &'static str = "710";
    pub const R720: &'static str = "720";
    pub const R730: &'static str = "730";
    pub const R740: &'static str = "740";
    pub const R750: &'static str = "750";
    pub const R760: &'static str = "760";
    pub const R770: &'static str = "770";
    pub const R780: &'static str = "780";
    pub const R790: &'static str = "790";
    pub const R800: &'static str = "800";
    pub const R810: &'static str = "810";
    pub const R820: &'static str = "820";
    pub const R830: &'static str = "830";
    pub const R840: &'static str = "840";
    pub const R850: &'static str = "850";
    pub const R860: &'static str = "860";
    pub const R870: &'static str = "870";
    pub const R880: &'static str = "880";
    pub const R890: &'static str = "890";
    pub const R900: &'static str = "900";
    pub const R910: &'static str = "910";
    pub const R920: &'static str = "920";
    pub const R930: &'static str = "930";
    pub const R940: &'static str = "940";
    pub const R950: &'static str = "950";
    pub const R960: &'static str = "960";
    pub const R970_41: &'static str = "970.41";

    pub fn all_known() -> &'static [&'static str] {
        &[
            Self::R010,
            Self::R020,
            Self::R030,
            Self::R040,
            Self::R050,
            Self::R060,
            Self::R070,
            Self::R080,
            Self::R090,
            Self::R100,
            Self::R110,
            Self::R120,
            Self::R130,
            Self::R140,
            Self::R150,
            Self::R160,
            Self::R170,
            Self::R180,
            Self::R190,
            Self::R200,
            Self::R210,
            Self::R220,
            Self::R230,
            Self::R240,
            Self::R250,
            Self::R260,
            Self::R270,
            Self::R280,
            Self::R290,
            Self::R300,
            Self::R310,
            Self::R320,
            Self::R330,
            Self::R340,
            Self::R350,
            Self::R360,
            Self::R370,
            Self::R380,
            Self::R390,
            Self::R400,
            Self::R410,
            Self::R420,
            Self::R430,
            Self::R440,
            Self::R450,
            Self::R460,
            Self::R470,
            Self::R480,
            Self::R490,
            Self::R500,
            Self::R510,
            Self::R520,
            Self::R530,
            Self::R540,
            Self::R550,
            Self::R560,
            Self::R570,
            Self::R580,
            Self::R590,
            Self::R600,
            Self::R610,
            Self::R620,
            Self::R630,
            Self::R640,
            Self::R650,
            Self::R660,
            Self::R670,
            Self::R680,
            Self::R690,
            Self::R700,
            Self::R710,
            Self::R720,
            Self::R730,
            Self::R740,
            Self::R750,
            Self::R760,
            Self::R770,
            Self::R780,
            Self::R790,
            Self::R800,
            Self::R810,
            Self::R820,
            Self::R830,
            Self::R840,
            Self::R850,
            Self::R860,
            Self::R870,
            Self::R880,
            Self::R890,
            Self::R900,
            Self::R910,
            Self::R920,
            Self::R930,
            Self::R940,
            Self::R950,
            Self::R960,
            Self::R970_41,
        ]
    }
}

/// 📄️ One semicolon-delimited native record.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub struct NativeRecord {
    pub family: RecordFamilyId,
    pub fields: Vec<String>,
    pub extensions: ExtensionBag,
}

/// ⚙️ Product configuration block.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub struct Configuration {
    pub id: String,
    pub parameters: BTreeMap<String, VdiValue>,
    pub geometry_ref: Option<String>,
    pub function_refs: Vec<String>,
}

/// 📦️ Catalogue product in Part 1 hierarchy.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub struct CatalogueProduct {
    pub identity: ProductIdentity,
    #[dsl(table)]
    pub title: Vec<LocalizedText>,
    pub sheet: SheetId,
    #[dsl(table)]
    pub records: Vec<NativeRecord>,
    pub configuration: Configuration,
    #[dsl(table)]
    pub accessories: Vec<AccessoryLink>,
    #[dsl(table)]
    pub components: Vec<CompositionLink>,
    pub extensions: ExtensionBag,
}

/// 📚️ Manufacturer catalogue document (Part 1).
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub struct ManufacturerCatalog {
    pub file: ManufacturerFile,
    #[dsl(table)]
    pub products: Vec<CatalogueProduct>,
    pub extensions: ExtensionBag,
}

impl ManufacturerCatalog {
    pub fn product_for_sheet(&self, sheet: SheetId) -> Option<&CatalogueProduct> {
        self.products.iter().find(|p| p.sheet == sheet)
    }
}
// #endregion Part1

// #region Geometry
/// 📦️ Axis-aligned bounding box [m].
#[derive(Clone, Copy, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub struct BoundingBox {
    pub min_x: f64,
    pub min_y: f64,
    pub min_z: f64,
    pub max_x: f64,
    pub max_y: f64,
    pub max_z: f64,
}

impl BoundingBox {
    pub fn from_size(w: f64, h: f64, d: f64) -> Self {
        Self { min_x: 0.0, min_y: 0.0, min_z: 0.0, max_x: w, max_y: h, max_z: d }
    }

    pub fn volume_m3(self) -> f64 {
        (self.max_x - self.min_x) * (self.max_y - self.min_y) * (self.max_z - self.min_z)
    }

    pub fn overlaps(self, other: Self, clearance: f64) -> bool {
        self.min_x - clearance < other.max_x && self.max_x + clearance > other.min_x && self.min_y - clearance < other.max_y && self.max_y + clearance > other.min_y && self.min_z - clearance < other.max_z && self.max_z + clearance > other.min_z
    }
}

/// 🔌️ Connection point on product geometry.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub struct ConnectionPoint {
    pub id: String,
    pub medium: String,
    pub position: [f64; 3],
    pub direction: [f64; 3],
    #[dsl(unit = "mm")]
    pub diameter_mm: Option<f64>,
}

/// 🧊️ Parametric geometry definition.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub struct ParametricGeometry {
    pub id: String,
    pub bbox: BoundingBox,
    #[dsl(table)]
    pub connections: Vec<ConnectionPoint>,
    pub parameters: BTreeMap<String, f64>,
}

impl ParametricGeometry {
    pub fn evaluate_bbox(&self) -> BoundingBox {
        let scale = self.parameters.get("scale").copied().unwrap_or(1.0);
        BoundingBox { min_x: self.bbox.min_x * scale, min_y: self.bbox.min_y * scale, min_z: self.bbox.min_z * scale, max_x: self.bbox.max_x * scale, max_y: self.bbox.max_y * scale, max_z: self.bbox.max_z * scale }
    }

    pub fn connection_count(&self) -> usize {
        self.connections.len()
    }
}
// #endregion Geometry

// #region Functions
/// 📈️ Characteristic curve point.
#[derive(Clone, Copy, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub struct CurvePoint {
    pub x: f64,
    pub y: f64,
}

/// 📉️ Characteristic curve with linear interpolation.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub struct CharacteristicCurve {
    pub id: String,
    pub x_unit: VdiUnit,
    pub y_unit: VdiUnit,
    #[dsl(table)]
    pub points: Vec<CurvePoint>,
}

impl CharacteristicCurve {
    pub fn interpolate(&self, x: f64) -> f64 {
        if self.points.is_empty() {
            return 0.0;
        }
        if x <= self.points[0].x {
            return self.points[0].y;
        }
        let last = self.points.len() - 1;
        if x >= self.points[last].x {
            return self.points[last].y;
        }
        for w in self.points.windows(2) {
            if x >= w[0].x && x <= w[1].x {
                let t = if (w[1].x - w[0].x).abs() < f64::EPSILON { 0.0 } else { (x - w[0].x) / (w[1].x - w[0].x) };
                return w[0].y + t * (w[1].y - w[0].y);
            }
        }
        self.points[last].y
    }
}
// #endregion Functions

// #region Catalog
/// 🔍️ Product index entry.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub struct CatalogIndexEntry {
    pub product_id: String,
    pub sheet: SheetId,
    pub tags: Vec<String>,
    pub dn: Option<u16>,
}

/// 📚️ Searchable catalogue index.
#[derive(Clone, Debug, Default, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub struct CatalogIndex {
    #[dsl(table)]
    pub entries: Vec<CatalogIndexEntry>,
}

impl CatalogIndex {
    pub fn from_catalog(catalog: &ManufacturerCatalog) -> Self {
        let entries = catalog
            .products
            .iter()
            .map(|p| CatalogIndexEntry {
                product_id: p.identity.article_number.clone(),
                sheet: p.sheet,
                tags: p.title.iter().map(|t| t.text.clone()).collect(),
                dn: p.configuration.parameters.get("dn").and_then(|v| match v {
                    VdiValue::Integer { value } => Some(*value as u16),
                    VdiValue::Decimal { value, .. } => Some(*value as u16),
                    _ => None,
                }),
            })
            .collect();
        Self { entries }
    }

    pub fn filter_by_sheet(&self, sheet: SheetId) -> Vec<&CatalogIndexEntry> {
        self.entries.iter().filter(|e| e.sheet == sheet).collect()
    }

    pub fn filter_by_dn(&self, dn: u16) -> Vec<&CatalogIndexEntry> {
        self.entries.iter().filter(|e| e.dn == Some(dn)).collect()
    }

    pub fn filter_by_tag(&self, tag: &str) -> Vec<&CatalogIndexEntry> {
        let lower = tag.to_lowercase();
        self.entries.iter().filter(|e| e.tags.iter().any(|t| t.to_lowercase().contains(&lower))).collect()
    }
}
// #endregion Catalog

// #region Validate
/// 🩺️ Validation diagnostic with severity.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub struct Diagnostic {
    pub field: String,
    pub message: String,
    pub severity: Severity,
}

/// ⚠️ Diagnostic severity.
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub enum Severity {
    Info,
    Warning,
    Error,
}

impl Diagnostic {
    pub fn info(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self { field: field.into(), message: message.into(), severity: Severity::Info }
    }

    pub fn warning(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self { field: field.into(), message: message.into(), severity: Severity::Warning }
    }

    pub fn error(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self { field: field.into(), message: message.into(), severity: Severity::Error }
    }
}
// #endregion Validate

/// 📸️ Persisted snapshot — defined in `📸️snapshot/🧬️schema`, re-exported here.
// #region Session
/// 📅️ Edition profile selection for multi-profile sheets.
#[derive(Clone, Copy, Debug, PartialEq, Eq, dsl::DslScalar, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub enum EditionProfileChoice {
    Legacy,
    Current,
}

/// 🏷️ Canonical DSL file extension for VDI 3805 documents.
pub const VDI3805_EXTENSION: &str = "vdi3805";

/// 📋️ VDI 3805 evaluation document.
/// 🧪️ Minimal valid heating valve (sheet 2) reference fixture.
pub fn reference_fixture() -> Vdi3805Snapshot {
    let bsn = BuildingSystemNumber { system_code: "420".into(), subsystem: "10".into(), sequence: 1 };
    let file = ManufacturerFile { header_version: "3805".into(), manufacturer: "DEMO".into(), building_system_number: bsn, created: "2026-07-22".into(), charset: "UTF-8".into(), record_count: 3, extensions: ExtensionBag::default() };
    let mut parameters = BTreeMap::new();
    parameters.insert("dn".into(), VdiValue::Integer { value: 50 });
    parameters.insert("kvs".into(), VdiValue::Decimal { value: 4.5, unit: Some(VdiUnit::absolute("m3/h", VdiQuantityKind::Volume, 1.0)) });
    let product = CatalogueProduct {
        identity: ProductIdentity { manufacturer_code: "DEMO".into(), product_group: "HV".into(), article_number: "VLV-50-001".into() },
        title: bilingual("Stellventil DN50", "Control valve DN50"),
        sheet: SheetId(2),
        records: vec![
            NativeRecord { family: RecordFamilyId(RecordFamilyId::R100.to_string()), fields: vec!["100".into(), "DEMO".into(), "HV".into(), "VLV-50-001".into(), "2".into()], extensions: ExtensionBag::default() },
            NativeRecord { family: RecordFamilyId(RecordFamilyId::R200.to_string()), fields: vec!["200".into(), "dn".into(), "50".into()], extensions: ExtensionBag::default() },
        ],
        configuration: Configuration { id: "cfg.VLV-50-001".into(), parameters, geometry_ref: Some("geom.valve.50".into()), function_refs: vec!["curve.kvs".into()] },
        accessories: Vec::new(),
        components: Vec::new(),
        extensions: ExtensionBag::default(),
    };
    let catalog = ManufacturerCatalog { file: file.clone(), products: vec![product], extensions: ExtensionBag::default() };
    let index = CatalogIndex::from_catalog(&catalog);
    let geometry = BTreeMap::from([(
        "geom.valve.50".into(),
        ParametricGeometry {
            id: "geom.valve.50".into(),
            bbox: BoundingBox::from_size(0.15, 0.20, 0.10),
            connections: vec![
                ConnectionPoint { id: "in".into(), medium: "water".into(), position: [0.0, 0.1, 0.05], direction: [-1.0, 0.0, 0.0], diameter_mm: Some(50.0) },
                ConnectionPoint { id: "out".into(), medium: "water".into(), position: [0.15, 0.1, 0.05], direction: [1.0, 0.0, 0.0], diameter_mm: Some(50.0) },
            ],
            parameters: BTreeMap::from([("scale".into(), 1.0)]),
        },
    )]);
    let curves = BTreeMap::from([(
        "curve.kvs".into(),
        CharacteristicCurve {
            id: "curve.kvs".into(),
            x_unit: VdiUnit::delta("%", VdiQuantityKind::Dimensionless, 0.01),
            y_unit: VdiUnit::absolute("m3/h", VdiQuantityKind::Volume, 1.0),
            points: vec![CurvePoint { x: 0.0, y: 0.0 }, CurvePoint { x: 100.0, y: 4.5 }],
        },
    )]);
    Vdi3805Snapshot { manufacturer_file: file, catalog, edition_profile: BTreeMap::new(), correction_as_of: EditionId::new(2024, 1), strict_mode: false, index, geometry, curves, limits: SecurityLimits::default() }
}
// #endregion Session

//#region 🔖️ArtifactKind
/// 🗿️ The computed-compliance artifact this standard publishes on its app's `report:out` port —
/// lifted out of the pre-migration manifest's inline `.artifact_kind(ArtifactKindSpec { .. })` so the
/// artifact node, not the app, owns its own kind declaration.
pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    crate::app_surface::artifact_kind_spec("vdi3805", "VDI 3805")
}
//#endregion 🔖️ArtifactKind

/// 🪪️ This subset's canonical `(artifact_kind, standard, subset)` coordinate (ticket
/// 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §1) — lives at the ARTIFACT level, not
/// under the sibling `editor` module, so a viewer file can read it without ever importing through it.
pub const VDI3805_DIALECT: semio_framework_plugin::app::Dialect = semio_framework_plugin::app::Dialect { artifact_kind: "s.norm.vdi3805", standard: semio_framework_plugin::app::StandardId("1"), subset: semio_framework_plugin::app::SubsetId::ANY };
pub const VDI3805_DOCUMENT_SCHEMA: &str = "semio.norm.vdi3805/v1";

#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    fn building_system_number_parse_render() {
        let bsn = BuildingSystemNumber::parse("420.10.1").expect("parse");
        assert_eq!(bsn.render(), "420.10.1");
    }

    #[semio_framework_async_macros::async_test]
    fn building_system_number_parse_rejects_wrong_part_count() {
        let err = BuildingSystemNumber::parse("420.10").unwrap_err();
        assert!(matches!(err, NormError::InvalidValue { field, .. } if field == "building_system_number"));
    }

    #[semio_framework_async_macros::async_test]
    fn building_system_number_parse_rejects_non_numeric_sequence() {
        let err = BuildingSystemNumber::parse("420.10.abc").unwrap_err();
        assert!(matches!(err, NormError::InvalidValue { field, .. } if field == "building_system_number.sequence"));
    }

    #[semio_framework_async_macros::async_test]
    fn security_limits_validate_text_rejects_oversized_input() {
        let limits = SecurityLimits { max_file_bytes: 8, ..SecurityLimits::default() };
        let err = limits.validate_text("this text is way longer than eight bytes").unwrap_err();
        assert!(matches!(err, NormError::InvalidValue { field, .. } if field == "file"));
    }

    #[semio_framework_async_macros::async_test]
    fn security_limits_validate_text_accepts_within_bound() {
        let limits = SecurityLimits::default();
        assert!(limits.validate_text("short").is_ok());
    }

    #[semio_framework_async_macros::async_test]
    fn characteristic_curve_interpolates() {
        let doc = Vdi3805Snapshot::default();
        let curve = doc.curves.get("curve.kvs").expect("curve");
        let y = curve.interpolate(50.0);
        assert!((y - 2.25).abs() < 1e-6);
    }

    #[semio_framework_async_macros::async_test]
    fn characteristic_curve_interpolate_handles_edges() {
        let empty = CharacteristicCurve { id: "empty".into(), x_unit: VdiUnit::delta("%", VdiQuantityKind::Dimensionless, 0.01), y_unit: VdiUnit::absolute("m3/h", VdiQuantityKind::Volume, 1.0), points: Vec::new() };
        assert_eq!(empty.interpolate(10.0), 0.0);

        let doc = Vdi3805Snapshot::default();
        let curve = doc.curves.get("curve.kvs").expect("curve");
        assert_eq!(curve.interpolate(-10.0), curve.points[0].y);
        assert_eq!(curve.interpolate(1000.0), curve.points[curve.points.len() - 1].y);
    }

    #[semio_framework_async_macros::async_test]
    fn bounding_box_overlaps_detects_intersection_and_gap() {
        let a = BoundingBox::from_size(1.0, 1.0, 1.0);
        let b = BoundingBox { min_x: 0.5, min_y: 0.5, min_z: 0.5, max_x: 1.5, max_y: 1.5, max_z: 1.5 };
        assert!(a.overlaps(b, 0.0));
        let c = BoundingBox { min_x: 5.0, min_y: 5.0, min_z: 5.0, max_x: 6.0, max_y: 6.0, max_z: 6.0 };
        assert!(!a.overlaps(c, 0.0));
    }

    #[semio_framework_async_macros::async_test]
    fn geometry_bbox_volume() {
        let doc = Vdi3805Snapshot::default();
        let geom = doc.geometry.get("geom.valve.50").expect("geom");
        let bbox = geom.evaluate_bbox();
        assert!((bbox.volume_m3() - 0.003).abs() < 1e-6);
    }

    #[semio_framework_async_macros::async_test]
    fn catalog_index_filters_by_dn() {
        let doc = Vdi3805Snapshot::default();
        let matches = doc.index.filter_by_dn(50);
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].product_id, "VLV-50-001");
    }

    #[semio_framework_async_macros::async_test]
    fn catalog_index_filter_by_sheet_and_tag() {
        let doc = Vdi3805Snapshot::default();
        let by_sheet = doc.index.filter_by_sheet(SheetId(2));
        assert_eq!(by_sheet.len(), 1);
        let by_tag = doc.index.filter_by_tag("control valve");
        assert_eq!(by_tag.len(), 1);
        assert!(doc.index.filter_by_tag("nonexistent-tag").is_empty());
    }

    #[semio_framework_async_macros::async_test]
    fn correction_overlay_applicability() {
        let registry = SchemaCatalog::current();
        let corrections = registry.corrections_for_sheet(SheetId(2));
        let corr = corrections.first().expect("part 2 correction");
        assert!(corr.id.starts_with("part-02-corr-"));
        assert!(corr.applies_as_of(EditionId::new(2024, 1)));
        assert!(!corr.applies_as_of(EditionId::new(2010, 1)));
    }

    #[semio_framework_async_macros::async_test]
    fn schema_registry_with_status_and_sheet_lookup() {
        let registry = SchemaCatalog::with_status(SchemaStatus::Reserved);
        assert!(registry.sheets().iter().all(|s| s.status == SchemaStatus::Reserved));
        let full = SchemaCatalog::current();
        let sheet = full.sheet(SheetId(2)).expect("sheet 2");
        assert_eq!(sheet.title_en, "Control valves heating");
        assert!(full.sheet(SheetId(9999)).is_none());
    }

    #[semio_framework_async_macros::async_test]
    fn schema_registry_sheets_in_domain_and_reserved_numbers() {
        let registry = SchemaCatalog::current();
        let heating = registry.sheets_in_domain(Domain::Heating);
        assert!(heating.iter().any(|s| s.id == SheetId(2)));
        let reserved = registry.reserved_numbers();
        assert!(reserved.contains(&15));
        assert!(!reserved.contains(&2));
    }

    #[semio_framework_async_macros::async_test]
    fn sheet_id_part_str_and_edition_id_key() {
        assert_eq!(SheetId(42).part_str(), "42");
        assert!(EditionId::new(2023, 3).key() > EditionId::new(2022, 6).key());
    }

    #[semio_framework_async_macros::async_test]
    fn schema_status_is_operative() {
        assert!(SchemaStatus::Published.is_operative());
        assert!(SchemaStatus::Checked.is_operative());
        assert!(!SchemaStatus::Draft.is_operative());
        assert!(!SchemaStatus::Reserved.is_operative());
    }

    #[semio_framework_async_macros::async_test]
    fn record_family_id_all_known_contains_expected() {
        let known = RecordFamilyId::all_known();
        assert!(known.contains(&RecordFamilyId::R010));
        assert!(known.contains(&RecordFamilyId::R970_41));
    }

    #[semio_framework_async_macros::async_test]
    fn manufacturer_catalog_product_for_sheet() {
        let doc = Vdi3805Snapshot::default();
        assert!(doc.catalog.product_for_sheet(SheetId(2)).is_some());
        assert!(doc.catalog.product_for_sheet(SheetId(3)).is_none());
    }
}

//#region 🪪️Declaration
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) — replaces
/// the old side-effecting `register()`/`register_pilot_languages()`/`register_artifact_schema()`/
/// `register_artifact_inferences()`/`register_io()`, each of which called a global registry directly
/// from the plugin root's `.setup()` fan-out (`register_norm_exports`, deleted by this same wave).
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use crate::artifacts::definition::{CapabilitySpec as C, ClaimSpec as Q, LocalizationSpec as L};
    const S: &[Q] = &[Q { namespace: "schema", value: "s.norm.vdi3805" }];
    const I: &[Q] = &[Q { namespace: "schema", value: "s.norm.vdi3805.inference" }];
    const M: &[Q] = &[Q { namespace: "dialect", value: "s.vdi3805@1/*" }];
    const K: &[Q] = &[Q { namespace: "codec", value: "semio.norm.vdi3805/v1" }, Q { namespace: "extension", value: "vdi3805" }];
    const EN: &[L] = &[L { locale: "en", text: "VDI 3805 manufacturer product data" }];
    const DE: &[L] = &[L { locale: "de", text: "VDI 3805 Produktdaten der Technischen Gebäudeausrüstung" }];
    const ROWS: &[C] = &[
        C { identity: "s.vdi3805.standard.v1", kind: "standard", descriptor: "v1", claims: &[], localizations: &[] },
        C { identity: "s.vdi3805.standard.v1.profile.any", kind: "profile", descriptor: "any", claims: &[], localizations: &[] },
        C { identity: "s.vdi3805.schema.artifact", kind: "schema", descriptor: "s.norm.vdi3805", claims: S, localizations: &[] },
        C { identity: "s.vdi3805.inference.outline", kind: "inference", descriptor: "s.norm.vdi3805.inference", claims: I, localizations: &[] },
        C { identity: "s.vdi3805.composer.any", kind: "composer", descriptor: "s.vdi3805@1/*", claims: M, localizations: &[] },
        C { identity: "s.vdi3805.grammar.document", kind: "grammar", descriptor: "vdi3805.document", claims: &[Q { namespace: "grammar", value: "vdi3805.document" }], localizations: &[] },
        C { identity: "s.vdi3805.grammar.op", kind: "grammar", descriptor: "vdi3805.op", claims: &[Q { namespace: "grammar", value: "vdi3805.op" }], localizations: &[] },
        C { identity: "s.vdi3805.grammar.diff", kind: "grammar", descriptor: "vdi3805.diff", claims: &[Q { namespace: "grammar", value: "vdi3805.diff" }], localizations: &[] },
        C { identity: "s.vdi3805.grammar.pack", kind: "grammar", descriptor: "vdi3805.pack", claims: &[Q { namespace: "grammar", value: "vdi3805.pack" }], localizations: &[] },
        C { identity: "s.vdi3805.grammar.spr", kind: "grammar", descriptor: "vdi3805.spr", claims: &[Q { namespace: "grammar", value: "vdi3805.spr" }], localizations: &[] },
        C { identity: "s.vdi3805.codec.document.v1", kind: "codec", descriptor: "semio.norm.vdi3805/v1:vdi3805", claims: K, localizations: &[] },
        C { identity: "s.vdi3805.localization.en", kind: "localization", descriptor: "VDI 3805 manufacturer product data", claims: &[], localizations: EN },
        C { identity: "s.vdi3805.localization.de", kind: "localization", descriptor: "VDI 3805 Produktdaten der Technischen Gebäudeausrüstung", claims: &[], localizations: DE },
    ];
    crate::artifacts::definition::assemble_definition("s.vdi3805", ROWS)
}

pub fn declaration(definition: semio_framework_plugin::ArtifactDefinition) -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    semio_framework_plugin::ArtifactDeclaration::builder(definition)
        .schema(crate::artifacts::vdi3805::schema::vdi3805_artifact_schema_descriptor())
        .inferences([crate::artifacts::vdi3805::standards::v1::subsets::any::schema::inferences::vdi3805_artifact_inference_descriptor()])
        .composers(crate::artifacts::vdi3805::standards::v1::subsets::any::io::io_registry::entries())
        .languages(pilot_languages())
        .document_codec::<semio_framework_plugin::EditorApp<crate::editor::vdi3805::Vdi3805PlayApp>>()
        .try_build()
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built once
/// and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`, mirroring the
/// `OnceLock`-backed `io_registry::entries()` convention below.
fn pilot_languages() -> &'static [dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                dsl::LanguageSpec {
                    id: "vdi3805.document",
                    extension: Some("vdi3805"),
                    role: dsl::LanguageRole::Document,
                    grammar: Some(crate::artifacts::vdi3805::dsl::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::vdi3805::dsl::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::vdi3805::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::vdi3805::snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("vdi3805.document"),
                },
                dsl::LanguageSpec {
                    id: "vdi3805.op",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(crate::artifacts::vdi3805::op::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::vdi3805::op::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::vdi3805::spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::vdi3805::spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("vdi3805.op"),
                },
                dsl::LanguageSpec {
                    id: "vdi3805.diff",
                    extension: None,
                    role: dsl::LanguageRole::Diff,
                    grammar: Some(crate::artifacts::vdi3805::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::vdi3805::diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: dsl::passthrough_hooks("vdi3805.diff"),
                },
                dsl::LanguageSpec {
                    id: "vdi3805.pack",
                    extension: None,
                    role: dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::vdi3805::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::vdi3805::snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("vdi3805.pack"),
                },
                dsl::LanguageSpec {
                    id: "vdi3805.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::vdi3805::spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::vdi3805::spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("vdi3805.spr"),
                },
            ]
        })
        .as_slice()
}
//#endregion 🪪️Declaration
