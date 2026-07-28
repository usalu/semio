//! 🔧 VDI 3805 manufacturer product data for building services: Part 1 + sheets 2–100.

use norm_core::{
    AnnexChoice, CheckReport, CheckResult, CheckStatus, ClauseId, NormError, NormFamily, NormFamilyId, NormHost, Quantity,
    QuantityKind, SetDocumentOperation,
};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

/// 🧬 `SetDocumentOperation<Document>` (whole-document replace) already implements both
/// `store::Operation<Document>` and, now that `Document` derives `dsl::DslDocument` (i.e.
/// `store::DocumentDsl`), `store::OpText` too — see `norm_core`'s generic `impl<D: DocumentDsl + ...>
/// OpText for SetDocumentOperation<D>`. A coarse, whole-value-replace operation is the legitimate,
/// sufficient choice per the migration cheat sheet: this reference/lookup-table document has no
/// existing interactive editor driving fine-grained field-level edits, so reusing this generic
/// pair (rather than hand-deriving a redundant one-variant `#[derive(dsl::DslOps)]` enum that would
/// duplicate exactly this shape) keeps every norm family crate's Operation layer DRY.
pub type Operation = SetDocumentOperation<Document>;
pub type Host = NormHost<Vdi3805Family>;

/// 📦 VCS envelope/store aliases for the VDI 3805 document, now that `Document`/`Operation` both
/// satisfy `store::DocumentDsl`/`store::OpText`.
pub type Vdi3805Envelope = store::DocumentEnvelope<Document, Operation>;
pub type Vdi3805Store = store::DocumentStore<Document, Operation>;

const FAMILY: &str = "VDI 3805";
const ANNEX: AnnexChoice = AnnexChoice::De;

fn clause(part: &str, section: &str) -> ClauseId {
    ClauseId::new(FAMILY, part, section)
}

fn na_check(part: &str, section: &str, message: impl Into<String>) -> CheckResult {
    CheckResult {
        clause: clause(part, section),
        status: CheckStatus::NotApplicable,
        computed: Quantity::new(QuantityKind::Dimensionless, 0.0),
        limit: Quantity::new(QuantityKind::Dimensionless, 1.0),
        utilization: 0.0,
        message: message.into(),
        annex: ANNEX,
    }
}

fn pass_check(part: &str, section: &str, message: impl Into<String>) -> CheckResult {
    CheckResult::pass(
        clause(part, section),
        Quantity::new(QuantityKind::Dimensionless, 1.0),
        Quantity::new(QuantityKind::Dimensionless, 1.0),
        1.0,
        message,
        ANNEX,
    )
}

fn fail_check(part: &str, section: &str, message: impl Into<String>) -> CheckResult {
    CheckResult::fail(
        clause(part, section),
        Quantity::new(QuantityKind::Dimensionless, 0.0),
        Quantity::new(QuantityKind::Dimensionless, 1.0),
        2.0,
        message,
        ANNEX,
    )
}

// #region Shared
/// 🌐 Locale-tagged text.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct LocalizedText {
    pub de: String,
    pub en: String,
}

impl LocalizedText {
    pub fn new(de: impl Into<String>, en: impl Into<String>) -> Self {
        Self { de: de.into(), en: en.into() }
    }
}

/// 🔒 A `QuantityKind` tag mirroring `norm_core::QuantityKind`'s 19 variants, kept locally: the DSL
/// engine's `DslField` binding can only be derived for a type/trait pair with a local half (orphan
/// rule), and `norm_core::QuantityKind` doesn't derive `dsl::DslScalar` itself. Converted at the
/// `VdiUnit` boundary via `From`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, dsl::DslScalar)]
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

/// 📐 VDI 3805 unit with absolute vs delta semantics.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
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

/// 🔢 Typed manufacturer value.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
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

/// 🔗 Hand `DslField` bridge for `VdiValue`: a deeply serde-tagged data enum embedded as a
/// `BTreeMap` VALUE type (`Configuration.parameters`), which mechanically requires `DslField` (map
/// values bind through `DslField`, not `DslVariants`) — `#[derive(dsl::DslEnum)]` only produces
/// `DslVariants`, so it can't satisfy that site. Binds through `Shape::Value` (the engine's existing
/// serde_json escape hatch), reusing the `Serialize`/`Deserialize` this type already has.
impl dsl::DslField for VdiValue {
    fn shape() -> dsl::Shape {
        dsl::Shape::Value
    }
    fn to_value(&self) -> dsl::FieldValue {
        let json = serde_json::to_value(self).expect("VdiValue always serializes to JSON");
        dsl::FieldValue::Value(dsl::DslValue::from(json))
    }
    fn from_value(value: &dsl::FieldValue) -> Result<Self, String> {
        match value {
            dsl::FieldValue::Value(dsl_value) => {
                let json = renormalize_whole_number_floats(serde_json::Value::from(dsl_value.clone()));
                serde_json::from_value(json).map_err(|e| e.to_string())
            }
            other => Err(format!("expected Value, found {other:?}")),
        }
    }
}

/// 🔧 `dsl::DslValue`'s `Number` variant is f64-only (no int/float distinction, per its own doc
/// comment) — round-tripping a value with an actual `i64` field (e.g. `VdiValue::Integer`) through
/// the `Shape::Value` bridge turns `50` into `50.0`, which `serde_json::from_value::<T>` then
/// rejects for an `i64` field ("invalid type: floating point `50`, expected i64"). Recursively
/// re-tags any whole-number JSON float back to a JSON integer before deserializing, so integer
/// fields still parse correctly on the far side of that bridge.
fn renormalize_whole_number_floats(value: serde_json::Value) -> serde_json::Value {
    match value {
        serde_json::Value::Number(n) => match n.as_f64() {
            Some(f) if f.fract() == 0.0 && f.abs() < i64::MAX as f64 => serde_json::Value::Number((f as i64).into()),
            _ => serde_json::Value::Number(n),
        },
        serde_json::Value::Array(items) => serde_json::Value::Array(items.into_iter().map(renormalize_whole_number_floats).collect()),
        serde_json::Value::Object(map) => serde_json::Value::Object(map.into_iter().map(|(k, v)| (k, renormalize_whole_number_floats(v))).collect()),
        other => other,
    }
}

/// 🧩 Lossless extension bag for unknown fields.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct ExtensionBag {
    pub fields: BTreeMap<String, serde_json::Value>,
}

/// 🆔 Product identity within a manufacturer catalogue.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, dsl::DslRecord)]
pub struct ProductIdentity {
    pub manufacturer_code: String,
    pub product_group: String,
    pub article_number: String,
}

/// 🏭 Manufacturer file header and payload references.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct ManufacturerFile {
    pub header_version: String,
    pub manufacturer: String,
    pub building_system_number: BuildingSystemNumber,
    pub created: String,
    pub charset: String,
    pub record_count: u32,
    pub extensions: ExtensionBag,
}

/// 🔗 Accessory relationship between products.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct AccessoryLink {
    pub accessory_id: String,
    pub required: bool,
    pub quantity: u32,
}

/// 🧱 Composition relationship (`hasPart`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct CompositionLink {
    pub component_id: String,
    pub quantity: u32,
}

/// 🔒 Security limits for untrusted manufacturer files.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct SecurityLimits {
    pub max_file_bytes: usize,
    pub max_records: usize,
    pub max_field_length: usize,
    pub max_nesting_depth: usize,
}

impl Default for SecurityLimits {
    fn default() -> Self {
        Self {
            max_file_bytes: 16 * 1024 * 1024,
            max_records: 100_000,
            max_field_length: 8_192,
            max_nesting_depth: 32,
        }
    }
}

impl SecurityLimits {
    pub fn validate_text(&self, text: &str) -> Result<(), NormError> {
        if text.len() > self.max_file_bytes {
            return Err(NormError::InvalidValue {
                field: "file".into(),
                reason: format!("exceeds {} bytes", self.max_file_bytes),
            });
        }
        Ok(())
    }
}
// #endregion Shared

// #region Schema
/// 📄 Sheet identifier (1…100).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct SheetId(pub u16);

impl SheetId {
    pub fn part_str(self) -> String {
        format!("{}", self.0)
    }
}

/// 🔗 Hand `DslField` bridge for `SheetId`: a tuple ("newtype") struct has no named fields for
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

/// 📅 Edition identifier (year + month).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, dsl::DslRecord)]
pub struct EditionId {
    pub year: u16,
    pub month: u8,
}

impl EditionId {
    pub const fn new(year: u16, month: u8) -> Self {
        Self { year, month }
    }

    pub fn key(self) -> u32 {
        (self.year as u32) * 100 + self.month as u32
    }
}

/// 📊 Schema lifecycle status.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Domain {
    Heating,
    Ventilation,
    Sanitary,
    BuildingAutomation,
    Electrical,
    Generic,
}

/// 📋 Sheet registry entry.
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

/// 🩹 Correction overlay descriptor.
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

/// 📚 Runtime edition registry.
#[derive(Clone, Debug, PartialEq)]
pub struct SchemaRegistry {
    sheets: Vec<SheetEntry>,
    corrections: &'static [CorrectionOverlay],
    filter: Option<SchemaStatus>,
}


const SHEET_ENTRIES: &[SheetEntry] = &[
    SheetEntry {
        id: SheetId(1),
        title_de: "Grundlagen",
        title_en: "Fundamentals",
        status: SchemaStatus::Published,
        domains: &[Domain::Generic],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(2),
        title_de: "Stellventile Heizung",
        title_en: "Control valves heating",
        status: SchemaStatus::Published,
        domains: &[Domain::Heating],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(3),
        title_de: "Heizkörper",
        title_en: "Radiators",
        status: SchemaStatus::Published,
        domains: &[Domain::Heating],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(4),
        title_de: "Rohrleitungen Heizung",
        title_en: "Pipes heating",
        status: SchemaStatus::Published,
        domains: &[Domain::Heating],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(5),
        title_de: "Pumpen Heizung",
        title_en: "Pumps heating",
        status: SchemaStatus::Published,
        domains: &[Domain::Heating],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(6),
        title_de: "Wärmeerzeuger",
        title_en: "Heat generators",
        status: SchemaStatus::Published,
        domains: &[Domain::Heating],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(7),
        title_de: "Speicher",
        title_en: "Storage tanks",
        status: SchemaStatus::Published,
        domains: &[Domain::Heating],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(8),
        title_de: "Armaturen Heizung",
        title_en: "Fittings heating",
        status: SchemaStatus::Published,
        domains: &[Domain::Heating],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2023, 3),
    },
    SheetEntry {
        id: SheetId(9),
        title_de: "Regelung Heizung",
        title_en: "Controls heating",
        status: SchemaStatus::Published,
        domains: &[Domain::Heating],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(10),
        title_de: "Verteiler Heizung",
        title_en: "Manifolds heating",
        status: SchemaStatus::Published,
        domains: &[Domain::Heating],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2023, 3),
    },
    SheetEntry {
        id: SheetId(11),
        title_de: "Messgeräte Heizung",
        title_en: "Meters heating",
        status: SchemaStatus::Published,
        domains: &[Domain::Heating],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
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
    SheetEntry {
        id: SheetId(14),
        title_de: "Ventile Lüftung",
        title_en: "Valves ventilation",
        status: SchemaStatus::Published,
        domains: &[Domain::Ventilation],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2023, 3),
    },
    SheetEntry {
        id: SheetId(15),
        title_de: "Blatt 15",
        title_en: "Sheet 15",
        status: SchemaStatus::Reserved,
        domains: &[Domain::Generic],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(16),
        title_de: "Luftdurchlässe",
        title_en: "Air terminals",
        status: SchemaStatus::Published,
        domains: &[Domain::Ventilation],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(17),
        title_de: "Kanäle Lüftung",
        title_en: "Ducts ventilation",
        status: SchemaStatus::Published,
        domains: &[Domain::Ventilation],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(18),
        title_de: "Blatt 18",
        title_en: "Sheet 18",
        status: SchemaStatus::Published,
        domains: &[Domain::Generic],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2023, 3),
    },
    SheetEntry {
        id: SheetId(19),
        title_de: "Filter Lüftung",
        title_en: "Filters ventilation",
        status: SchemaStatus::Published,
        domains: &[Domain::Ventilation],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(20),
        title_de: "Wärmerückgewinnung",
        title_en: "Heat recovery",
        status: SchemaStatus::Published,
        domains: &[Domain::Ventilation],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(21),
        title_de: "Sanitärarmaturen",
        title_en: "Sanitary fittings",
        status: SchemaStatus::Published,
        domains: &[Domain::Sanitary],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(22),
        title_de: "Rohrleitungen Sanitär",
        title_en: "Pipes sanitary",
        status: SchemaStatus::Published,
        domains: &[Domain::Sanitary],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(23),
        title_de: "Pumpen Sanitär",
        title_en: "Pumps sanitary",
        status: SchemaStatus::Published,
        domains: &[Domain::Sanitary],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(24),
        title_de: "Speicher Sanitär",
        title_en: "Storage sanitary",
        status: SchemaStatus::Published,
        domains: &[Domain::Sanitary],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(25),
        title_de: "Historischer Vorschlag Sanitär",
        title_en: "Historical proposal sanitary",
        status: SchemaStatus::HistoricalProposal,
        domains: &[Domain::Generic],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2018, 6),
    },
    SheetEntry {
        id: SheetId(26),
        title_de: "Regelung Sanitär",
        title_en: "Controls sanitary",
        status: SchemaStatus::Published,
        domains: &[Domain::Sanitary],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(27),
        title_de: "Messgeräte Sanitär",
        title_en: "Meters sanitary",
        status: SchemaStatus::Published,
        domains: &[Domain::Sanitary],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(28),
        title_de: "Gebäudeautomation",
        title_en: "Building automation",
        status: SchemaStatus::Published,
        domains: &[Domain::BuildingAutomation],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(29),
        title_de: "Elektro Komponenten",
        title_en: "Electrical components",
        status: SchemaStatus::Published,
        domains: &[Domain::Electrical],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(30),
        title_de: "Blatt 30",
        title_en: "Sheet 30",
        status: SchemaStatus::Reserved,
        domains: &[Domain::Generic],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(31),
        title_de: "Blatt 31",
        title_en: "Sheet 31",
        status: SchemaStatus::Reserved,
        domains: &[Domain::Generic],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(32),
        title_de: "Kältemaschinen",
        title_en: "Chillers",
        status: SchemaStatus::Published,
        domains: &[Domain::Heating],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(33),
        title_de: "Kühldecken",
        title_en: "Chilled ceilings",
        status: SchemaStatus::Published,
        domains: &[Domain::Heating],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2023, 3),
    },
    SheetEntry {
        id: SheetId(34),
        title_de: "Konvektoren",
        title_en: "Convectors",
        status: SchemaStatus::Published,
        domains: &[Domain::Heating],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(35),
        title_de: "Fußbodenheizung",
        title_en: "Underfloor heating",
        status: SchemaStatus::Published,
        domains: &[Domain::Heating],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(36),
        title_de: "Blatt 36",
        title_en: "Sheet 36",
        status: SchemaStatus::Published,
        domains: &[Domain::Generic],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2023, 3),
    },
    SheetEntry {
        id: SheetId(37),
        title_de: "Blatt 37",
        title_en: "Sheet 37",
        status: SchemaStatus::Published,
        domains: &[Domain::Generic],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2023, 3),
    },
    SheetEntry {
        id: SheetId(38),
        title_de: "Schalldämpfer",
        title_en: "Silencers",
        status: SchemaStatus::Published,
        domains: &[Domain::Ventilation],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(39),
        title_de: "Blatt 39",
        title_en: "Sheet 39",
        status: SchemaStatus::Reserved,
        domains: &[Domain::Generic],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(40),
        title_de: "Klappen Lüftung",
        title_en: "Dampers ventilation",
        status: SchemaStatus::Published,
        domains: &[Domain::Ventilation],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2023, 3),
    },
    SheetEntry {
        id: SheetId(41),
        title_de: "Ventilatoren",
        title_en: "Fans",
        status: SchemaStatus::Published,
        domains: &[Domain::Ventilation],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(42),
        title_de: "VAV-Regler",
        title_en: "VAV controllers",
        status: SchemaStatus::Published,
        domains: &[Domain::Ventilation],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2023, 3),
    },
    SheetEntry {
        id: SheetId(43),
        title_de: "Wärmetauscher",
        title_en: "Heat exchangers",
        status: SchemaStatus::Published,
        domains: &[Domain::Heating],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(44),
        title_de: "Druckerhöhung",
        title_en: "Pressure boosting",
        status: SchemaStatus::Published,
        domains: &[Domain::Sanitary],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(45),
        title_de: "Entgasung",
        title_en: "Degassing",
        status: SchemaStatus::Published,
        domains: &[Domain::Heating],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(46),
        title_de: "Blatt 46",
        title_en: "Sheet 46",
        status: SchemaStatus::Reserved,
        domains: &[Domain::Generic],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(47),
        title_de: "Blatt 47",
        title_en: "Sheet 47",
        status: SchemaStatus::Reserved,
        domains: &[Domain::Generic],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(48),
        title_de: "Blatt 48",
        title_en: "Sheet 48",
        status: SchemaStatus::Reserved,
        domains: &[Domain::Generic],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(49),
        title_de: "Blatt 49",
        title_en: "Sheet 49",
        status: SchemaStatus::Reserved,
        domains: &[Domain::Generic],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(50),
        title_de: "Brandschutzklappen",
        title_en: "Fire dampers",
        status: SchemaStatus::Published,
        domains: &[Domain::Ventilation],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(51),
        title_de: "Rohrbegleitheizung",
        title_en: "Trace heating",
        status: SchemaStatus::Published,
        domains: &[Domain::Heating],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(52),
        title_de: "Solarthermie",
        title_en: "Solar thermal",
        status: SchemaStatus::Published,
        domains: &[Domain::Heating],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(53),
        title_de: "Wärmepumpen",
        title_en: "Heat pumps",
        status: SchemaStatus::Published,
        domains: &[Domain::Heating],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2023, 3),
    },
    SheetEntry {
        id: SheetId(54),
        title_de: "Befeuchtung",
        title_en: "Humidification",
        status: SchemaStatus::Published,
        domains: &[Domain::Ventilation],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(55),
        title_de: "Entfeuchtung",
        title_en: "Dehumidification",
        status: SchemaStatus::Published,
        domains: &[Domain::Ventilation],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(56),
        title_de: "Blatt 56",
        title_en: "Sheet 56",
        status: SchemaStatus::Reserved,
        domains: &[Domain::Generic],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(57),
        title_de: "Blatt 57",
        title_en: "Sheet 57",
        status: SchemaStatus::Reserved,
        domains: &[Domain::Generic],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(58),
        title_de: "Blatt 58",
        title_en: "Sheet 58",
        status: SchemaStatus::Reserved,
        domains: &[Domain::Generic],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(59),
        title_de: "Blatt 59",
        title_en: "Sheet 59",
        status: SchemaStatus::Reserved,
        domains: &[Domain::Generic],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(60),
        title_de: "Kompensatoren",
        title_en: "Compensators",
        status: SchemaStatus::Published,
        domains: &[Domain::Generic],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(61),
        title_de: "Trennstellen",
        title_en: "Separation points",
        status: SchemaStatus::Published,
        domains: &[Domain::Generic],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(62),
        title_de: "Schmutzfänger",
        title_en: "Strainers",
        status: SchemaStatus::Published,
        domains: &[Domain::Generic],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(63),
        title_de: "Rückschlagventile",
        title_en: "Check valves",
        status: SchemaStatus::Published,
        domains: &[Domain::Generic],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(64),
        title_de: "Sicherheitsventile",
        title_en: "Safety valves",
        status: SchemaStatus::Published,
        domains: &[Domain::Generic],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(65),
        title_de: "Absperrarmaturen",
        title_en: "Shut-off fittings",
        status: SchemaStatus::Published,
        domains: &[Domain::Generic],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(66),
        title_de: "Mischventile",
        title_en: "Mixing valves",
        status: SchemaStatus::Published,
        domains: &[Domain::Generic],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(67),
        title_de: "Blatt 67",
        title_en: "Sheet 67",
        status: SchemaStatus::Reserved,
        domains: &[Domain::Generic],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(68),
        title_de: "Blatt 68",
        title_en: "Sheet 68",
        status: SchemaStatus::Reserved,
        domains: &[Domain::Generic],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(69),
        title_de: "Blatt 69",
        title_en: "Sheet 69",
        status: SchemaStatus::Reserved,
        domains: &[Domain::Generic],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(70),
        title_de: "Blatt 70",
        title_en: "Sheet 70",
        status: SchemaStatus::Reserved,
        domains: &[Domain::Generic],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(71),
        title_de: "Blatt 71",
        title_en: "Sheet 71",
        status: SchemaStatus::Reserved,
        domains: &[Domain::Generic],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(72),
        title_de: "Blatt 72",
        title_en: "Sheet 72",
        status: SchemaStatus::Reserved,
        domains: &[Domain::Generic],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(73),
        title_de: "Blatt 73",
        title_en: "Sheet 73",
        status: SchemaStatus::Reserved,
        domains: &[Domain::Generic],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(74),
        title_de: "Blatt 74",
        title_en: "Sheet 74",
        status: SchemaStatus::Reserved,
        domains: &[Domain::Generic],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(75),
        title_de: "Blatt 75",
        title_en: "Sheet 75",
        status: SchemaStatus::Reserved,
        domains: &[Domain::Generic],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(76),
        title_de: "Blatt 76",
        title_en: "Sheet 76",
        status: SchemaStatus::Reserved,
        domains: &[Domain::Generic],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(77),
        title_de: "Blatt 77",
        title_en: "Sheet 77",
        status: SchemaStatus::Reserved,
        domains: &[Domain::Generic],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(78),
        title_de: "Blatt 78",
        title_en: "Sheet 78",
        status: SchemaStatus::Reserved,
        domains: &[Domain::Generic],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(79),
        title_de: "Blatt 79",
        title_en: "Sheet 79",
        status: SchemaStatus::Reserved,
        domains: &[Domain::Generic],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(80),
        title_de: "Blatt 80",
        title_en: "Sheet 80",
        status: SchemaStatus::Reserved,
        domains: &[Domain::Generic],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(81),
        title_de: "Blatt 81",
        title_en: "Sheet 81",
        status: SchemaStatus::Reserved,
        domains: &[Domain::Generic],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(82),
        title_de: "Blatt 82",
        title_en: "Sheet 82",
        status: SchemaStatus::Reserved,
        domains: &[Domain::Generic],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(83),
        title_de: "Blatt 83",
        title_en: "Sheet 83",
        status: SchemaStatus::Reserved,
        domains: &[Domain::Generic],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(84),
        title_de: "Blatt 84",
        title_en: "Sheet 84",
        status: SchemaStatus::Reserved,
        domains: &[Domain::Generic],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(85),
        title_de: "Blatt 85",
        title_en: "Sheet 85",
        status: SchemaStatus::Reserved,
        domains: &[Domain::Generic],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(86),
        title_de: "Blatt 86",
        title_en: "Sheet 86",
        status: SchemaStatus::Reserved,
        domains: &[Domain::Generic],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(87),
        title_de: "Blatt 87",
        title_en: "Sheet 87",
        status: SchemaStatus::Reserved,
        domains: &[Domain::Generic],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(88),
        title_de: "Blatt 88",
        title_en: "Sheet 88",
        status: SchemaStatus::Reserved,
        domains: &[Domain::Generic],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(89),
        title_de: "Blatt 89",
        title_en: "Sheet 89",
        status: SchemaStatus::Reserved,
        domains: &[Domain::Generic],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(90),
        title_de: "Blatt 90",
        title_en: "Sheet 90",
        status: SchemaStatus::Reserved,
        domains: &[Domain::Generic],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(91),
        title_de: "Blatt 91",
        title_en: "Sheet 91",
        status: SchemaStatus::Reserved,
        domains: &[Domain::Generic],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(92),
        title_de: "Blatt 92",
        title_en: "Sheet 92",
        status: SchemaStatus::Reserved,
        domains: &[Domain::Generic],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(93),
        title_de: "Blatt 93",
        title_en: "Sheet 93",
        status: SchemaStatus::Reserved,
        domains: &[Domain::Generic],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(94),
        title_de: "Blatt 94",
        title_en: "Sheet 94",
        status: SchemaStatus::Reserved,
        domains: &[Domain::Generic],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(95),
        title_de: "Blatt 95",
        title_en: "Sheet 95",
        status: SchemaStatus::Reserved,
        domains: &[Domain::Generic],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(96),
        title_de: "Blatt 96",
        title_en: "Sheet 96",
        status: SchemaStatus::Reserved,
        domains: &[Domain::Generic],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(97),
        title_de: "Blatt 97",
        title_en: "Sheet 97",
        status: SchemaStatus::Reserved,
        domains: &[Domain::Generic],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(98),
        title_de: "Blatt 98",
        title_en: "Sheet 98",
        status: SchemaStatus::Reserved,
        domains: &[Domain::Generic],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(99),
        title_de: "Erweiterungen",
        title_en: "Extensions",
        status: SchemaStatus::Published,
        domains: &[Domain::Generic],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2022, 6),
    },
    SheetEntry {
        id: SheetId(100),
        title_de: "Profilübergreifend",
        title_en: "Cross-profile",
        status: SchemaStatus::Published,
        domains: &[Domain::Generic],
        part1_edition: EditionId::new(2022, 6),
        current_edition: EditionId::new(2023, 3),
    },
];

const CORRECTION_OVERLAYS: &[CorrectionOverlay] = &[
    CorrectionOverlay {
        id: "part-02-corr-2022-12",
        sheet: SheetId(2),
        base_edition: EditionId::new(2022, 12),
        effective: EditionId::new(2023, 2),
        summary_de: "Korrektur Blatt 2",
        summary_en: "Correction sheet 2",
    },
    CorrectionOverlay {
        id: "part-03-corr-2022-11",
        sheet: SheetId(3),
        base_edition: EditionId::new(2022, 11),
        effective: EditionId::new(2023, 1),
        summary_de: "Korrektur Blatt 3",
        summary_en: "Correction sheet 3",
    },
    CorrectionOverlay {
        id: "part-04-corr-2022-10",
        sheet: SheetId(4),
        base_edition: EditionId::new(2022, 10),
        effective: EditionId::new(2022, 12),
        summary_de: "Korrektur Blatt 4",
        summary_en: "Correction sheet 4",
    },
    CorrectionOverlay {
        id: "part-05-corr-2021-09",
        sheet: SheetId(5),
        base_edition: EditionId::new(2021, 9),
        effective: EditionId::new(2021, 11),
        summary_de: "Korrektur Blatt 5",
        summary_en: "Correction sheet 5",
    },
    CorrectionOverlay {
        id: "part-06-corr-2021-08",
        sheet: SheetId(6),
        base_edition: EditionId::new(2021, 8),
        effective: EditionId::new(2021, 10),
        summary_de: "Korrektur Blatt 6",
        summary_en: "Correction sheet 6",
    },
    CorrectionOverlay {
        id: "part-07-corr-2021-07",
        sheet: SheetId(7),
        base_edition: EditionId::new(2021, 7),
        effective: EditionId::new(2021, 9),
        summary_de: "Korrektur Blatt 7",
        summary_en: "Correction sheet 7",
    },
    CorrectionOverlay {
        id: "part-08-corr-2020-06",
        sheet: SheetId(8),
        base_edition: EditionId::new(2020, 6),
        effective: EditionId::new(2020, 8),
        summary_de: "Korrektur Blatt 8",
        summary_en: "Correction sheet 8",
    },
    CorrectionOverlay {
        id: "part-09-corr-2020-05",
        sheet: SheetId(9),
        base_edition: EditionId::new(2020, 5),
        effective: EditionId::new(2020, 7),
        summary_de: "Korrektur Blatt 9",
        summary_en: "Correction sheet 9",
    },
    CorrectionOverlay {
        id: "part-10-corr-2020-04",
        sheet: SheetId(10),
        base_edition: EditionId::new(2020, 4),
        effective: EditionId::new(2020, 6),
        summary_de: "Korrektur Blatt 10",
        summary_en: "Correction sheet 10",
    },
    CorrectionOverlay {
        id: "part-11-corr-2019-03",
        sheet: SheetId(11),
        base_edition: EditionId::new(2019, 3),
        effective: EditionId::new(2019, 5),
        summary_de: "Korrektur Blatt 11",
        summary_en: "Correction sheet 11",
    },
    CorrectionOverlay {
        id: "part-12-corr-2019-02",
        sheet: SheetId(12),
        base_edition: EditionId::new(2019, 2),
        effective: EditionId::new(2019, 4),
        summary_de: "Korrektur Blatt 12",
        summary_en: "Correction sheet 12",
    },
    CorrectionOverlay {
        id: "part-13-corr-2019-01",
        sheet: SheetId(13),
        base_edition: EditionId::new(2019, 1),
        effective: EditionId::new(2019, 3),
        summary_de: "Korrektur Blatt 13",
        summary_en: "Correction sheet 13",
    },
    CorrectionOverlay {
        id: "part-14-corr-2018-12",
        sheet: SheetId(14),
        base_edition: EditionId::new(2018, 12),
        effective: EditionId::new(2019, 2),
        summary_de: "Korrektur Blatt 14",
        summary_en: "Correction sheet 14",
    },
    CorrectionOverlay {
        id: "part-15-corr-2018-11",
        sheet: SheetId(15),
        base_edition: EditionId::new(2018, 11),
        effective: EditionId::new(2019, 1),
        summary_de: "Korrektur Blatt 15",
        summary_en: "Correction sheet 15",
    },
    CorrectionOverlay {
        id: "part-16-corr-2018-10",
        sheet: SheetId(16),
        base_edition: EditionId::new(2018, 10),
        effective: EditionId::new(2018, 12),
        summary_de: "Korrektur Blatt 16",
        summary_en: "Correction sheet 16",
    },
    CorrectionOverlay {
        id: "part-17-corr-2017-09",
        sheet: SheetId(17),
        base_edition: EditionId::new(2017, 9),
        effective: EditionId::new(2017, 11),
        summary_de: "Korrektur Blatt 17",
        summary_en: "Correction sheet 17",
    },
    CorrectionOverlay {
        id: "part-18-corr-2017-08",
        sheet: SheetId(18),
        base_edition: EditionId::new(2017, 8),
        effective: EditionId::new(2017, 10),
        summary_de: "Korrektur Blatt 18",
        summary_en: "Correction sheet 18",
    },
    CorrectionOverlay {
        id: "part-19-corr-2017-07",
        sheet: SheetId(19),
        base_edition: EditionId::new(2017, 7),
        effective: EditionId::new(2017, 9),
        summary_de: "Korrektur Blatt 19",
        summary_en: "Correction sheet 19",
    },
    CorrectionOverlay {
        id: "part-20-corr-2016-06",
        sheet: SheetId(20),
        base_edition: EditionId::new(2016, 6),
        effective: EditionId::new(2016, 8),
        summary_de: "Korrektur Blatt 20",
        summary_en: "Correction sheet 20",
    },
    CorrectionOverlay {
        id: "part-21-corr-2016-05",
        sheet: SheetId(21),
        base_edition: EditionId::new(2016, 5),
        effective: EditionId::new(2016, 7),
        summary_de: "Korrektur Blatt 21",
        summary_en: "Correction sheet 21",
    },
    CorrectionOverlay {
        id: "part-22-corr-2016-04",
        sheet: SheetId(22),
        base_edition: EditionId::new(2016, 4),
        effective: EditionId::new(2016, 6),
        summary_de: "Korrektur Blatt 22",
        summary_en: "Correction sheet 22",
    },
    CorrectionOverlay {
        id: "part-23-corr-2015-03",
        sheet: SheetId(23),
        base_edition: EditionId::new(2015, 3),
        effective: EditionId::new(2015, 5),
        summary_de: "Korrektur Blatt 23",
        summary_en: "Correction sheet 23",
    },
    CorrectionOverlay {
        id: "part-24-corr-2015-02",
        sheet: SheetId(24),
        base_edition: EditionId::new(2015, 2),
        effective: EditionId::new(2015, 4),
        summary_de: "Korrektur Blatt 24",
        summary_en: "Correction sheet 24",
    },
    CorrectionOverlay {
        id: "part-25-corr-2015-01",
        sheet: SheetId(25),
        base_edition: EditionId::new(2015, 1),
        effective: EditionId::new(2015, 3),
        summary_de: "Korrektur Blatt 25",
        summary_en: "Correction sheet 25",
    },
    CorrectionOverlay {
        id: "part-26-corr-2014-12",
        sheet: SheetId(26),
        base_edition: EditionId::new(2014, 12),
        effective: EditionId::new(2015, 2),
        summary_de: "Korrektur Blatt 26",
        summary_en: "Correction sheet 26",
    },
    CorrectionOverlay {
        id: "part-27-corr-2014-11",
        sheet: SheetId(27),
        base_edition: EditionId::new(2014, 11),
        effective: EditionId::new(2015, 1),
        summary_de: "Korrektur Blatt 27",
        summary_en: "Correction sheet 27",
    },
    CorrectionOverlay {
        id: "part-28-corr-2014-10",
        sheet: SheetId(28),
        base_edition: EditionId::new(2014, 10),
        effective: EditionId::new(2014, 12),
        summary_de: "Korrektur Blatt 28",
        summary_en: "Correction sheet 28",
    },
    CorrectionOverlay {
        id: "part-29-corr-2013-09",
        sheet: SheetId(29),
        base_edition: EditionId::new(2013, 9),
        effective: EditionId::new(2013, 11),
        summary_de: "Korrektur Blatt 29",
        summary_en: "Correction sheet 29",
    },
    CorrectionOverlay {
        id: "part-30-corr-2013-08",
        sheet: SheetId(30),
        base_edition: EditionId::new(2013, 8),
        effective: EditionId::new(2013, 10),
        summary_de: "Korrektur Blatt 30",
        summary_en: "Correction sheet 30",
    },
    CorrectionOverlay {
        id: "part-31-corr-2013-07",
        sheet: SheetId(31),
        base_edition: EditionId::new(2013, 7),
        effective: EditionId::new(2013, 9),
        summary_de: "Korrektur Blatt 31",
        summary_en: "Correction sheet 31",
    },
    CorrectionOverlay {
        id: "part-32-corr-2019-07",
        sheet: SheetId(32),
        base_edition: EditionId::new(2019, 7),
        effective: EditionId::new(2019, 9),
        summary_de: "Korrektur Blatt 32",
        summary_en: "Correction sheet 32",
    },
];

impl SchemaRegistry {
    fn build(filter: Option<SchemaStatus>) -> Self {
        let sheets: Vec<SheetEntry> = SHEET_ENTRIES
            .iter()
            .filter(|s| filter.map_or(true, |f| s.status == f))
            .cloned()
            .collect();
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
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, dsl::DslRecord)]
pub struct BuildingSystemNumber {
    pub system_code: String,
    pub subsystem: String,
    pub sequence: u32,
}

impl BuildingSystemNumber {
    pub fn parse(raw: &str) -> Result<Self, NormError> {
        let parts: Vec<&str> = raw.split('.').collect();
        if parts.len() != 3 {
            return Err(NormError::InvalidValue {
                field: "building_system_number".into(),
                reason: "expected SYS.SUB.NNN".into(),
            });
        }
        let sequence: u32 = parts[2].parse().map_err(|_| NormError::InvalidValue {
            field: "building_system_number.sequence".into(),
            reason: "must be numeric".into(),
        })?;
        Ok(Self {
            system_code: parts[0].into(),
            subsystem: parts[1].into(),
            sequence,
        })
    }

    pub fn render(&self) -> String {
        format!("{}.{}.{}", self.system_code, self.subsystem, self.sequence)
    }
}

/// 📇 Record family identifier (010…970.41).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RecordFamilyId(pub String);

/// 🔗 Hand `DslField` bridge for `RecordFamilyId`: a tuple ("newtype") struct has no named fields
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
            Self::R010, Self::R020, Self::R030, Self::R040, Self::R050, Self::R060, Self::R070,
            Self::R080, Self::R090, Self::R100, Self::R110, Self::R120, Self::R130, Self::R140,
            Self::R150, Self::R160, Self::R170, Self::R180, Self::R190, Self::R200, Self::R210,
            Self::R220, Self::R230, Self::R240, Self::R250, Self::R260, Self::R270, Self::R280,
            Self::R290, Self::R300, Self::R310, Self::R320, Self::R330, Self::R340, Self::R350,
            Self::R360, Self::R370, Self::R380, Self::R390, Self::R400, Self::R410, Self::R420,
            Self::R430, Self::R440, Self::R450, Self::R460, Self::R470, Self::R480, Self::R490,
            Self::R500, Self::R510, Self::R520, Self::R530, Self::R540, Self::R550, Self::R560,
            Self::R570, Self::R580, Self::R590, Self::R600, Self::R610, Self::R620, Self::R630,
            Self::R640, Self::R650, Self::R660, Self::R670, Self::R680, Self::R690, Self::R700,
            Self::R710, Self::R720, Self::R730, Self::R740, Self::R750, Self::R760, Self::R770,
            Self::R780, Self::R790, Self::R800, Self::R810, Self::R820, Self::R830, Self::R840,
            Self::R850, Self::R860, Self::R870, Self::R880, Self::R890, Self::R900, Self::R910,
            Self::R920, Self::R930, Self::R940, Self::R950, Self::R960, Self::R970_41,
        ]
    }
}

/// 📄 One semicolon-delimited native record.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct NativeRecord {
    pub family: RecordFamilyId,
    pub fields: Vec<String>,
    pub extensions: ExtensionBag,
}

/// ⚙️ Product configuration block.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct Configuration {
    pub id: String,
    pub parameters: BTreeMap<String, VdiValue>,
    pub geometry_ref: Option<String>,
    pub function_refs: Vec<String>,
}

/// 📦 Catalogue product in Part 1 hierarchy.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct CatalogueProduct {
    pub identity: ProductIdentity,
    pub title: LocalizedText,
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

/// 📚 Manufacturer catalogue document (Part 1).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
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

/// 🔤 Parse semicolon-delimited native VDI 3805 text.
pub fn parse_native_text(text: &str, limits: SecurityLimits) -> Result<ManufacturerCatalog, NormError> {
    limits.validate_text(text)?;
    let mut lines = text.lines().filter(|l| !l.trim().is_empty());
    let header_line = lines.next().ok_or(NormError::IncompleteInput { field: "header".into() })?;
    let header_fields: Vec<&str> = header_line.split(';').collect();
    if header_fields.len() < 5 {
        return Err(NormError::IncompleteInput { field: "header_fields".into() });
    }
    let bsn = BuildingSystemNumber::parse(header_fields[2])?;
    let record_count: u32 = header_fields[4].parse().map_err(|_| NormError::InvalidValue {
        field: "record_count".into(),
        reason: "numeric expected".into(),
    })?;
    let mut records = Vec::new();
    let mut products = Vec::new();
    for line in lines {
        if records.len() >= limits.max_records {
            return Err(NormError::InvalidValue { field: "records".into(), reason: "too many records".into() });
        }
        let fields: Vec<String> = line.split(';').map(|s| s.to_string()).collect();
        if fields.is_empty() {
            continue;
        }
        let family = RecordFamilyId(fields[0].clone());
        if fields[0] == "100" && fields.len() >= 4 {
            let article_number = fields.get(3).cloned().unwrap_or_default();
            let identity = ProductIdentity {
                manufacturer_code: fields.get(1).cloned().unwrap_or_default(),
                product_group: fields.get(2).cloned().unwrap_or_default(),
                article_number: article_number.clone(),
            };
            let sheet_no: u16 = fields.get(4).and_then(|s| s.parse().ok()).unwrap_or(2);
            products.push(CatalogueProduct {
                identity,
                title: LocalizedText::new("Produkt", "Product"),
                sheet: SheetId(sheet_no),
                records: Vec::new(),
                configuration: Configuration {
                    id: format!("cfg.{}", article_number),
                    parameters: BTreeMap::new(),
                    geometry_ref: None,
                    function_refs: Vec::new(),
                },
                accessories: Vec::new(),
                components: Vec::new(),
                extensions: ExtensionBag::default(),
            });
        }
        records.push(NativeRecord { family, fields, extensions: ExtensionBag::default() });
    }
    let file = ManufacturerFile {
        header_version: header_fields[0].into(),
        manufacturer: header_fields[1].into(),
        building_system_number: bsn,
        created: header_fields.get(3).unwrap_or(&"").to_string(),
        charset: "UTF-8".into(),
        record_count,
        extensions: ExtensionBag::default(),
    };
    Ok(ManufacturerCatalog { file, products, extensions: ExtensionBag::default() })
}

/// 🔤 Serialize catalogue to semicolon-delimited native text.
pub fn serialize_native_text(catalog: &ManufacturerCatalog) -> String {
    let f = &catalog.file;
    let mut out = format!(
        "{};{};{};{};{}\n",
        f.header_version,
        f.manufacturer,
        f.building_system_number.render(),
        f.created,
        f.record_count
    );
    for product in &catalog.products {
        out.push_str(&format!(
            "100;{};{};{};{}\n",
            product.identity.manufacturer_code,
            product.identity.product_group,
            product.identity.article_number,
            product.sheet.0
        ));
    }
    for product in &catalog.products {
        for record in &product.records {
            if record.fields.first().is_some_and(|f| f == "100") {
                continue;
            }
            out.push_str(&record.fields.join(";"));
            out.push('\n');
        }
    }
    out
}

/// ✅ Structural validation of Part 1 catalogue.
pub fn validate_structure(catalog: &ManufacturerCatalog) -> Vec<Diagnostic> {
    let mut issues = Vec::new();
    if catalog.file.manufacturer.is_empty() {
        issues.push(Diagnostic::error("manufacturer", "missing manufacturer code"));
    }
    if catalog.products.is_empty() {
        issues.push(Diagnostic::warning("products", "empty product list"));
    }
    for product in &catalog.products {
        if product.identity.article_number.is_empty() {
            issues.push(Diagnostic::error(
                &format!("product.{}", product.sheet.0),
                "missing article number",
            ));
        }
        if product.configuration.id.is_empty() {
            issues.push(Diagnostic::warning(
                &format!("configuration.{}", product.sheet.0),
                "missing configuration id",
            ));
        }
    }
    let known: BTreeSet<&str> = RecordFamilyId::all_known().iter().copied().collect();
    for product in &catalog.products {
        for record in &product.records {
            if !known.contains(record.family.0.as_str()) && !record.family.0.starts_with("9") {
                issues.push(Diagnostic::info(
                    &format!("record.{}", record.family.0),
                    "unknown record family preserved",
                ));
            }
        }
    }
    issues
}
// #endregion Part1

// #region Geometry
/// 📦 Axis-aligned bounding box [m].
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
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
        self.min_x - clearance < other.max_x
            && self.max_x + clearance > other.min_x
            && self.min_y - clearance < other.max_y
            && self.max_y + clearance > other.min_y
            && self.min_z - clearance < other.max_z
            && self.max_z + clearance > other.min_z
    }
}

/// 🔌 Connection point on product geometry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct ConnectionPoint {
    pub id: String,
    pub medium: String,
    pub position: [f64; 3],
    pub direction: [f64; 3],
    pub diameter_mm: Option<f64>,
}

/// 🧊 Parametric geometry definition.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
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
        BoundingBox {
            min_x: self.bbox.min_x * scale,
            min_y: self.bbox.min_y * scale,
            min_z: self.bbox.min_z * scale,
            max_x: self.bbox.max_x * scale,
            max_y: self.bbox.max_y * scale,
            max_z: self.bbox.max_z * scale,
        }
    }

    pub fn connection_count(&self) -> usize {
        self.connections.len()
    }
}
// #endregion Geometry

// #region Functions
/// 📈 Characteristic curve point.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct CurvePoint {
    pub x: f64,
    pub y: f64,
}

/// 📉 Characteristic curve with linear interpolation.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
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
                let t = if (w[1].x - w[0].x).abs() < f64::EPSILON {
                    0.0
                } else {
                    (x - w[0].x) / (w[1].x - w[0].x)
                };
                return w[0].y + t * (w[1].y - w[0].y);
            }
        }
        self.points[last].y
    }
}

/// 🔢 Linear map between two scalar domains.
pub fn linear_map(x: f64, x0: f64, x1: f64, y0: f64, y1: f64) -> f64 {
    if (x1 - x0).abs() < f64::EPSILON {
        return y0;
    }
    y0 + (x - x0) * (y1 - y0) / (x1 - x0)
}
// #endregion Functions

// #region Catalog
/// 🔍 Product index entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct CatalogIndexEntry {
    pub product_id: String,
    pub sheet: SheetId,
    pub tags: Vec<String>,
    pub dn: Option<u16>,
}

/// 📚 Searchable catalogue index.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
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
                tags: vec![p.title.de.clone(), p.title.en.clone()],
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
        self.entries
            .iter()
            .filter(|e| e.tags.iter().any(|t| t.to_lowercase().contains(&lower)))
            .collect()
    }
}
// #endregion Catalog

// #region Io
/// 📤 JSON round-trip for manufacturer catalogues.
pub fn catalog_to_json(catalog: &ManufacturerCatalog) -> Result<String, NormError> {
    serde_json::to_string_pretty(catalog).map_err(|e| NormError::InvalidValue {
        field: "json".into(),
        reason: e.to_string(),
    })
}

pub fn catalog_from_json(json: &str) -> Result<ManufacturerCatalog, NormError> {
    serde_json::from_str(json).map_err(|e| NormError::InvalidValue {
        field: "json".into(),
        reason: e.to_string(),
    })
}

pub fn document_to_json(document: &Document) -> Result<String, NormError> {
    serde_json::to_string_pretty(document).map_err(|e| NormError::InvalidValue {
        field: "json".into(),
        reason: e.to_string(),
    })
}

pub fn document_from_json(json: &str) -> Result<Document, NormError> {
    serde_json::from_str(json).map_err(|e| NormError::InvalidValue {
        field: "json".into(),
        reason: e.to_string(),
    })
}
// #endregion Io

// #region Validate
/// 🩺 Validation diagnostic with severity.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Diagnostic {
    pub field: String,
    pub message: String,
    pub severity: Severity,
}

/// ⚠️ Diagnostic severity.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
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

pub fn diagnostics_to_report(diagnostics: &[Diagnostic], part: &str, section: &str) -> Vec<CheckResult> {
    diagnostics
        .iter()
        .map(|d| {
            let status = match d.severity {
                Severity::Error => CheckStatus::Fail,
                Severity::Warning | Severity::Info => CheckStatus::Pass,
            };
            let utilization = if status == CheckStatus::Fail { 2.0 } else { 1.0 };
            CheckResult {
                clause: clause(part, section),
                status,
                computed: Quantity::new(QuantityKind::Dimensionless, if status == CheckStatus::Fail { 0.0 } else { 1.0 }),
                limit: Quantity::new(QuantityKind::Dimensionless, 1.0),
                utilization,
                message: format!("{}: {}", d.field, d.message),
                annex: ANNEX,
            }
        })
        .collect()
}
// #endregion Validate

// #region SheetParts
macro_rules! define_vdi_part {
    ($module:ident, $num:literal, reserved) => {
        pub mod $module {
            use super::*;

            pub fn metadata() -> &'static SheetEntry {
                &SHEET_ENTRIES[$num - 1]
            }

            pub fn check(document: &Document) -> CheckResult {
                let _ = document;
                na_check(stringify!($num), "scope", format!("sheet {} reserved", $num))
            }
        }
    };
    ($module:ident, $num:literal, historical) => {
        pub mod $module {
            use super::*;

            pub fn metadata() -> &'static SheetEntry {
                &SHEET_ENTRIES[$num - 1]
            }

            pub fn check(document: &Document) -> CheckResult {
                if document.strict_mode {
                    fail_check(stringify!($num), "status", "historical proposal not allowed in strict mode")
                } else {
                    pass_check(stringify!($num), "status", "historical proposal acknowledged")
                }
            }
        }
    };
    ($module:ident, $num:literal, multi_profile) => {
        pub mod $module {
            use super::*;

            #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
            pub enum EditionProfile {
                Legacy,
                Current,
            }

            pub fn metadata() -> &'static SheetEntry {
                &SHEET_ENTRIES[$num - 1]
            }

            pub fn check(document: &Document) -> CheckResult {
                let product = document.catalog.product_for_sheet(SheetId($num));
                let profile = document.edition_profile.get(stringify!($num)).copied().unwrap_or(EditionProfileChoice::Current);
                if product.is_none() {
                    return pass_check(stringify!($num), "metadata", format!("sheet {} metadata reachable (no product)", $num));
                }
                let msg = format!("sheet {} profile {:?} validated", $num, profile);
                pass_check(stringify!($num), "profile", msg)
            }
        }
    };
    ($module:ident, $num:literal) => {
        pub mod $module {
            use super::*;

            pub fn metadata() -> &'static SheetEntry {
                &SHEET_ENTRIES[$num - 1]
            }

            pub fn check(document: &Document) -> CheckResult {
                if let Some(product) = document.catalog.product_for_sheet(SheetId($num)) {
                    if product.identity.article_number.is_empty() {
                        return fail_check(stringify!($num), "identity", "missing article number");
                    }
                    pass_check(stringify!($num), "identity", format!("sheet {} product present", $num))
                } else {
                    pass_check(stringify!($num), "metadata", format!("sheet {} metadata reachable", $num))
                }
            }
        }
    };
}

pub mod part_1 {
    use super::*;

    pub fn metadata() -> &'static SheetEntry {
        &SHEET_ENTRIES[0]
    }

    pub fn check(document: &Document) -> CheckResult {
        let issues = validate_structure(&document.catalog);
        if issues.iter().any(|d| d.severity == Severity::Error) {
            fail_check("1", "structure", "Part 1 structural errors")
        } else {
            pass_check("1", "structure", "Part 1 structure valid")
        }
    }
}

define_vdi_part!(part_02, 2);
define_vdi_part!(part_03, 3);
define_vdi_part!(part_04, 4);
define_vdi_part!(part_05, 5);
define_vdi_part!(part_06, 6);
define_vdi_part!(part_07, 7);
define_vdi_part!(part_08, 8, multi_profile);
define_vdi_part!(part_09, 9);
define_vdi_part!(part_10, 10, multi_profile);
define_vdi_part!(part_11, 11);
define_vdi_part!(part_12, 12, historical);
define_vdi_part!(part_13, 13, historical);
define_vdi_part!(part_14, 14, multi_profile);
define_vdi_part!(part_15, 15, reserved);
define_vdi_part!(part_16, 16);
define_vdi_part!(part_17, 17);
define_vdi_part!(part_18, 18, multi_profile);
define_vdi_part!(part_19, 19);
define_vdi_part!(part_20, 20);
define_vdi_part!(part_21, 21);
define_vdi_part!(part_22, 22);
define_vdi_part!(part_23, 23);
define_vdi_part!(part_24, 24);
define_vdi_part!(part_25, 25, historical);
define_vdi_part!(part_26, 26);
define_vdi_part!(part_27, 27);
define_vdi_part!(part_28, 28);
define_vdi_part!(part_29, 29);
define_vdi_part!(part_30, 30, reserved);
define_vdi_part!(part_31, 31, reserved);
define_vdi_part!(part_32, 32);
define_vdi_part!(part_33, 33, multi_profile);
define_vdi_part!(part_34, 34);
define_vdi_part!(part_35, 35);
define_vdi_part!(part_36, 36, multi_profile);
define_vdi_part!(part_37, 37, multi_profile);
define_vdi_part!(part_38, 38);
define_vdi_part!(part_39, 39, reserved);
define_vdi_part!(part_40, 40, multi_profile);
define_vdi_part!(part_41, 41);
define_vdi_part!(part_42, 42, multi_profile);
define_vdi_part!(part_43, 43);
define_vdi_part!(part_44, 44);
define_vdi_part!(part_45, 45);
define_vdi_part!(part_46, 46, reserved);
define_vdi_part!(part_47, 47, reserved);
define_vdi_part!(part_48, 48, reserved);
define_vdi_part!(part_49, 49, reserved);
define_vdi_part!(part_50, 50);
define_vdi_part!(part_51, 51);
define_vdi_part!(part_52, 52);
define_vdi_part!(part_53, 53, multi_profile);
define_vdi_part!(part_54, 54);
define_vdi_part!(part_55, 55);
define_vdi_part!(part_56, 56, reserved);
define_vdi_part!(part_57, 57, reserved);
define_vdi_part!(part_58, 58, reserved);
define_vdi_part!(part_59, 59, reserved);
define_vdi_part!(part_60, 60);
define_vdi_part!(part_61, 61);
define_vdi_part!(part_62, 62);
define_vdi_part!(part_63, 63);
define_vdi_part!(part_64, 64);
define_vdi_part!(part_65, 65);
define_vdi_part!(part_66, 66);
define_vdi_part!(part_67, 67, reserved);
define_vdi_part!(part_68, 68, reserved);
define_vdi_part!(part_69, 69, reserved);
define_vdi_part!(part_70, 70, reserved);
define_vdi_part!(part_71, 71, reserved);
define_vdi_part!(part_72, 72, reserved);
define_vdi_part!(part_73, 73, reserved);
define_vdi_part!(part_74, 74, reserved);
define_vdi_part!(part_75, 75, reserved);
define_vdi_part!(part_76, 76, reserved);
define_vdi_part!(part_77, 77, reserved);
define_vdi_part!(part_78, 78, reserved);
define_vdi_part!(part_79, 79, reserved);
define_vdi_part!(part_80, 80, reserved);
define_vdi_part!(part_81, 81, reserved);
define_vdi_part!(part_82, 82, reserved);
define_vdi_part!(part_83, 83, reserved);
define_vdi_part!(part_84, 84, reserved);
define_vdi_part!(part_85, 85, reserved);
define_vdi_part!(part_86, 86, reserved);
define_vdi_part!(part_87, 87, reserved);
define_vdi_part!(part_88, 88, reserved);
define_vdi_part!(part_89, 89, reserved);
define_vdi_part!(part_90, 90, reserved);
define_vdi_part!(part_91, 91, reserved);
define_vdi_part!(part_92, 92, reserved);
define_vdi_part!(part_93, 93, reserved);
define_vdi_part!(part_94, 94, reserved);
define_vdi_part!(part_95, 95, reserved);
define_vdi_part!(part_96, 96, reserved);
define_vdi_part!(part_97, 97, reserved);
define_vdi_part!(part_98, 98, reserved);
define_vdi_part!(part_99, 99);
define_vdi_part!(part_100, 100, multi_profile);
// #endregion SheetParts

// #region Session
/// 📅 Edition profile selection for multi-profile sheets.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
pub enum EditionProfileChoice {
    Legacy,
    Current,
}

/// 🏷️ Canonical DSL file extension for VDI 3805 documents.
pub const VDI3805_EXTENSION: &str = "vdi3805";

/// 📋 VDI 3805 evaluation document.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[dsl(extension = "vdi3805", layout = "lines")]
pub struct Document {
    pub manufacturer_file: ManufacturerFile,
    pub catalog: ManufacturerCatalog,
    pub edition_profile: BTreeMap<String, EditionProfileChoice>,
    pub correction_as_of: EditionId,
    pub strict_mode: bool,
    pub index: CatalogIndex,
    pub geometry: BTreeMap<String, ParametricGeometry>,
    pub curves: BTreeMap<String, CharacteristicCurve>,
    pub limits: SecurityLimits,
}

impl Default for Document {
    fn default() -> Self {
        reference_fixture()
    }
}

/// 🧪 Minimal valid heating valve (sheet 2) reference fixture.
pub fn reference_fixture() -> Document {
    let bsn = BuildingSystemNumber {
        system_code: "420".into(),
        subsystem: "10".into(),
        sequence: 1,
    };
    let file = ManufacturerFile {
        header_version: "3805".into(),
        manufacturer: "DEMO".into(),
        building_system_number: bsn,
        created: "2026-07-22".into(),
        charset: "UTF-8".into(),
        record_count: 3,
        extensions: ExtensionBag::default(),
    };
    let mut parameters = BTreeMap::new();
    parameters.insert("dn".into(), VdiValue::Integer { value: 50 });
    parameters.insert("kvs".into(), VdiValue::Decimal {
        value: 4.5,
        unit: Some(VdiUnit::absolute("m3/h", VdiQuantityKind::Volume, 1.0)),
    });
    let product = CatalogueProduct {
        identity: ProductIdentity {
            manufacturer_code: "DEMO".into(),
            product_group: "HV".into(),
            article_number: "VLV-50-001".into(),
        },
        title: LocalizedText::new("Stellventil DN50", "Control valve DN50"),
        sheet: SheetId(2),
        records: vec![
            NativeRecord {
                family: RecordFamilyId(RecordFamilyId::R100.to_string()),
                fields: vec!["100".into(), "DEMO".into(), "HV".into(), "VLV-50-001".into(), "2".into()],
                extensions: ExtensionBag::default(),
            },
            NativeRecord {
                family: RecordFamilyId(RecordFamilyId::R200.to_string()),
                fields: vec!["200".into(), "dn".into(), "50".into()],
                extensions: ExtensionBag::default(),
            },
        ],
        configuration: Configuration {
            id: "cfg.VLV-50-001".into(),
            parameters,
            geometry_ref: Some("geom.valve.50".into()),
            function_refs: vec!["curve.kvs".into()],
        },
        accessories: Vec::new(),
        components: Vec::new(),
        extensions: ExtensionBag::default(),
    };
    let catalog = ManufacturerCatalog {
        file: file.clone(),
        products: vec![product],
        extensions: ExtensionBag::default(),
    };
    let index = CatalogIndex::from_catalog(&catalog);
    let geometry = BTreeMap::from([(
        "geom.valve.50".into(),
        ParametricGeometry {
            id: "geom.valve.50".into(),
            bbox: BoundingBox::from_size(0.15, 0.20, 0.10),
            connections: vec![
                ConnectionPoint {
                    id: "in".into(),
                    medium: "water".into(),
                    position: [0.0, 0.1, 0.05],
                    direction: [-1.0, 0.0, 0.0],
                    diameter_mm: Some(50.0),
                },
                ConnectionPoint {
                    id: "out".into(),
                    medium: "water".into(),
                    position: [0.15, 0.1, 0.05],
                    direction: [1.0, 0.0, 0.0],
                    diameter_mm: Some(50.0),
                },
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
            points: vec![
                CurvePoint { x: 0.0, y: 0.0 },
                CurvePoint { x: 100.0, y: 4.5 },
            ],
        },
    )]);
    Document {
        manufacturer_file: file,
        catalog,
        edition_profile: BTreeMap::new(),
        correction_as_of: EditionId::new(2024, 1),
        strict_mode: false,
        index,
        geometry,
        curves,
        limits: SecurityLimits::default(),
    }
}

fn all_part_checks(document: &Document) -> Vec<CheckResult> {
    vec![
        part_1::check(document),

        part_02::check(document),
        part_03::check(document),
        part_04::check(document),
        part_05::check(document),
        part_06::check(document),
        part_07::check(document),
        part_08::check(document),
        part_09::check(document),
        part_10::check(document),
        part_11::check(document),
        part_12::check(document),
        part_13::check(document),
        part_14::check(document),
        part_15::check(document),
        part_16::check(document),
        part_17::check(document),
        part_18::check(document),
        part_19::check(document),
        part_20::check(document),
        part_21::check(document),
        part_22::check(document),
        part_23::check(document),
        part_24::check(document),
        part_25::check(document),
        part_26::check(document),
        part_27::check(document),
        part_28::check(document),
        part_29::check(document),
        part_30::check(document),
        part_31::check(document),
        part_32::check(document),
        part_33::check(document),
        part_34::check(document),
        part_35::check(document),
        part_36::check(document),
        part_37::check(document),
        part_38::check(document),
        part_39::check(document),
        part_40::check(document),
        part_41::check(document),
        part_42::check(document),
        part_43::check(document),
        part_44::check(document),
        part_45::check(document),
        part_46::check(document),
        part_47::check(document),
        part_48::check(document),
        part_49::check(document),
        part_50::check(document),
        part_51::check(document),
        part_52::check(document),
        part_53::check(document),
        part_54::check(document),
        part_55::check(document),
        part_56::check(document),
        part_57::check(document),
        part_58::check(document),
        part_59::check(document),
        part_60::check(document),
        part_61::check(document),
        part_62::check(document),
        part_63::check(document),
        part_64::check(document),
        part_65::check(document),
        part_66::check(document),
        part_67::check(document),
        part_68::check(document),
        part_69::check(document),
        part_70::check(document),
        part_71::check(document),
        part_72::check(document),
        part_73::check(document),
        part_74::check(document),
        part_75::check(document),
        part_76::check(document),
        part_77::check(document),
        part_78::check(document),
        part_79::check(document),
        part_80::check(document),
        part_81::check(document),
        part_82::check(document),
        part_83::check(document),
        part_84::check(document),
        part_85::check(document),
        part_86::check(document),
        part_87::check(document),
        part_88::check(document),
        part_89::check(document),
        part_90::check(document),
        part_91::check(document),
        part_92::check(document),
        part_93::check(document),
        part_94::check(document),
        part_95::check(document),
        part_96::check(document),
        part_97::check(document),
        part_98::check(document),
        part_99::check(document),
        part_100::check(document),
    ]
}

pub fn evaluate(document: &Document) -> CheckReport {
    let mut report = CheckReport::default();

    for check in all_part_checks(document) {
        report.push(check);
    }

    let registry = SchemaRegistry::current();
    let reserved = registry.reserved_numbers();
    for n in 15u16..=98 {
        if reserved.contains(&n) {
            report.push(na_check(&n.to_string(), "reserved", format!("sheet {n} reserved")));
        }
    }

    let operative: BTreeSet<u16> = registry
        .operative_sheets()
        .iter()
        .map(|s| s.id.0)
        .collect();
    report.push(pass_check(
        "registry",
        "operative",
        format!("{} operative sheets", operative.len()),
    ));

    for corr in registry.corrections_for_sheet(SheetId(2)) {
        let applies = corr.applies_as_of(document.correction_as_of);
        report.push(if applies {
            pass_check("2", "correction", format!("{} applies", corr.id))
        } else {
            pass_check("2", "correction", format!("{} not yet effective", corr.id))
        });
    }

    match catalog_to_json(&document.catalog) {
        Ok(json) => match catalog_from_json(&json) {
            Ok(restored) => {
                if restored.products.len() == document.catalog.products.len() {
                    report.push(pass_check("io", "json", "catalog JSON round-trip"));
                } else {
                    report.push(fail_check("io", "json", "catalog JSON product count mismatch"));
                }
            }
            Err(err) => report.push(fail_check("io", "json", err.to_string())),
        },
        Err(err) => report.push(fail_check("io", "json", err.to_string())),
    }

    let native = serialize_native_text(&document.catalog);
    match parse_native_text(&native, document.limits) {
        Ok(parsed) => {
            if parsed.products.len() == document.catalog.products.len() {
                report.push(pass_check("io", "native", "native text round-trip"));
            } else {
                report.push(fail_check("io", "native", "native text product count mismatch"));
            }
        }
        Err(err) => report.push(fail_check("io", "native", err.to_string())),
    }

    let heating = registry.sheets_in_domain(Domain::Heating);
    report.push(pass_check(
        "catalog",
        "index",
        format!(
            "index {} entries, {} heating sheets",
            document.index.entries.len(),
            heating.len()
        ),
    ));

    let dn50 = document.index.filter_by_dn(50);
    if !dn50.is_empty() {
        report.push(pass_check("catalog", "filter", "DN50 filter matched"));
    }

    if let Some(geom) = document.geometry.get("geom.valve.50") {
        let bbox = geom.evaluate_bbox();
        report.push(CheckResult::from_utilization(
            clause("geometry", "bbox"),
            Quantity::new(QuantityKind::Volume, bbox.volume_m3()),
            Quantity::new(QuantityKind::Volume, 0.003),
            format!("bbox volume {:.6} m3, {} connections", bbox.volume_m3(), geom.connection_count()),
            ANNEX,
        ));
    }

    if let Some(curve) = document.curves.get("curve.kvs") {
        let y = curve.interpolate(50.0);
        report.push(CheckResult::from_utilization(
            clause("functions", "curve"),
            Quantity::new(QuantityKind::Volume, y),
            Quantity::new(QuantityKind::Volume, 2.25),
            format!("kvs curve at 50% = {y:.3}"),
            ANNEX,
        ));
    }

    for diag in validate_structure(&document.catalog) {
        report.checks.extend(diagnostics_to_report(&[diag], "1", "validate"));
    }

    if document.strict_mode {
        report.push(pass_check("session", "strict", "strict mode enabled"));
    }

    report
}

pub struct Vdi3805Family;

impl NormFamily for Vdi3805Family {
    type Document = Document;
    type Operation = Operation;

    fn family_id() -> NormFamilyId {
        NormFamilyId::Vdi3805
    }

    fn evaluate(document: &Document) -> CheckReport {
        evaluate(document)
    }
}
// #endregion Session

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn evaluate_reaches_operative_sheet_families() {
        let report = evaluate(&Document::default());
        let parts: BTreeSet<String> = report
            .checks
            .iter()
            .map(|c| c.clause.part.clone())
            .filter(|p| p.chars().all(|ch| ch.is_ascii_digit()))
            .collect();
        let registry = SchemaRegistry::current();
        for sheet in registry.operative_sheets() {
            let part = sheet.id.0.to_string();
            if sheet.status == SchemaStatus::Reserved {
                continue;
            }
            assert!(parts.contains(&part), "missing checks for sheet {part}");
        }
    }

    #[test]
    fn native_text_round_trip() {
        let doc = Document::default();
        let text = serialize_native_text(&doc.catalog);
        let parsed = parse_native_text(&text, SecurityLimits::default()).expect("parse");
        assert_eq!(parsed.products.len(), doc.catalog.products.len());
        assert_eq!(parsed.file.manufacturer, doc.catalog.file.manufacturer);
    }

    #[test]
    fn correction_overlay_applicability() {
        let registry = SchemaRegistry::current();
        let corrections = registry.corrections_for_sheet(SheetId(2));
        let corr = corrections.first().expect("part 2 correction");
        assert!(corr.id.starts_with("part-02-corr-"));
        assert!(corr.applies_as_of(EditionId::new(2024, 1)));
        assert!(!corr.applies_as_of(EditionId::new(2010, 1)));
    }

    #[test]
    fn reserved_sheet_returns_not_applicable() {
        let doc = Document::default();
        let result = part_15::check(&doc);
        assert_eq!(result.status, CheckStatus::NotApplicable);
        let result = part_67::check(&doc);
        assert_eq!(result.status, CheckStatus::NotApplicable);
    }

    #[test]
    fn norm_family_id() {
        assert_eq!(Vdi3805Family::family_id(), NormFamilyId::Vdi3805);
        assert_eq!(NormFamilyId::Vdi3805.label(), "VDI 3805");
    }

    #[test]
    fn building_system_number_parse_render() {
        let bsn = BuildingSystemNumber::parse("420.10.1").expect("parse");
        assert_eq!(bsn.render(), "420.10.1");
    }

    #[test]
    fn catalog_index_filters_by_dn() {
        let doc = Document::default();
        let matches = doc.index.filter_by_dn(50);
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].product_id, "VLV-50-001");
    }

    #[test]
    fn geometry_bbox_volume() {
        let doc = Document::default();
        let geom = doc.geometry.get("geom.valve.50").expect("geom");
        let bbox = geom.evaluate_bbox();
        assert!((bbox.volume_m3() - 0.003).abs() < 1e-6);
    }

    #[test]
    fn characteristic_curve_interpolates() {
        let doc = Document::default();
        let curve = doc.curves.get("curve.kvs").expect("curve");
        let y = curve.interpolate(50.0);
        assert!((y - 2.25).abs() < 1e-6);
    }

    #[test]
    fn norm_host_recomputes() {
        let mut host = Host::from_document(Document::default());
        assert!(!host.report().checks.is_empty());
        host.replace_document(Document::default());
        assert!(host.report().all_pass());
    }

    #[test]
    fn security_limits_validate_text_rejects_oversized_input() {
        let limits = SecurityLimits { max_file_bytes: 8, ..SecurityLimits::default() };
        let err = limits.validate_text("this text is way longer than eight bytes").unwrap_err();
        assert!(matches!(err, NormError::InvalidValue { field, .. } if field == "file"));
    }

    #[test]
    fn security_limits_validate_text_accepts_within_bound() {
        let limits = SecurityLimits::default();
        assert!(limits.validate_text("short").is_ok());
    }

    #[test]
    fn building_system_number_parse_rejects_wrong_part_count() {
        let err = BuildingSystemNumber::parse("420.10").unwrap_err();
        assert!(matches!(err, NormError::InvalidValue { field, .. } if field == "building_system_number"));
    }

    #[test]
    fn building_system_number_parse_rejects_non_numeric_sequence() {
        let err = BuildingSystemNumber::parse("420.10.abc").unwrap_err();
        assert!(matches!(err, NormError::InvalidValue { field, .. } if field == "building_system_number.sequence"));
    }

    #[test]
    fn parse_native_text_rejects_empty_input() {
        let err = parse_native_text("", SecurityLimits::default()).unwrap_err();
        assert!(matches!(err, NormError::IncompleteInput { field } if field == "header"));
    }

    #[test]
    fn parse_native_text_rejects_incomplete_header() {
        let err = parse_native_text("3805;DEMO;420.10.1\n", SecurityLimits::default()).unwrap_err();
        assert!(matches!(err, NormError::IncompleteInput { field } if field == "header_fields"));
    }

    #[test]
    fn parse_native_text_rejects_invalid_building_system_number() {
        let err = parse_native_text("3805;DEMO;bad;2026-07-22;3\n", SecurityLimits::default()).unwrap_err();
        assert!(matches!(err, NormError::InvalidValue { field, .. } if field == "building_system_number"));
    }

    #[test]
    fn parse_native_text_rejects_non_numeric_record_count() {
        let err = parse_native_text("3805;DEMO;420.10.1;2026-07-22;abc\n", SecurityLimits::default()).unwrap_err();
        assert!(matches!(err, NormError::InvalidValue { field, .. } if field == "record_count"));
    }

    #[test]
    fn parse_native_text_rejects_too_many_records() {
        let limits = SecurityLimits { max_records: 0, ..SecurityLimits::default() };
        let text = "3805;DEMO;420.10.1;2026-07-22;1\n200;dn;50\n";
        let err = parse_native_text(text, limits).unwrap_err();
        assert!(matches!(err, NormError::InvalidValue { field, .. } if field == "records"));
    }

    #[test]
    fn parse_native_text_parses_product_records() {
        let text = "3805;DEMO;420.10.1;2026-07-22;1\n100;DEMO;HV;VLV-1;2\n200;dn;50\n";
        let parsed = parse_native_text(text, SecurityLimits::default()).expect("parse");
        assert_eq!(parsed.products.len(), 1);
        assert_eq!(parsed.products[0].identity.article_number, "VLV-1");
        assert_eq!(parsed.products[0].sheet, SheetId(2));
    }

    #[test]
    fn validate_structure_reports_missing_manufacturer() {
        let mut doc = Document::default();
        doc.catalog.file.manufacturer = String::new();
        let issues = validate_structure(&doc.catalog);
        assert!(issues.iter().any(|d| d.field == "manufacturer" && d.severity == Severity::Error));
    }

    #[test]
    fn validate_structure_reports_empty_products() {
        let mut doc = Document::default();
        doc.catalog.products.clear();
        let issues = validate_structure(&doc.catalog);
        assert!(issues.iter().any(|d| d.field == "products" && d.severity == Severity::Warning));
    }

    #[test]
    fn validate_structure_reports_missing_article_number_and_config_id() {
        let mut doc = Document::default();
        doc.catalog.products[0].identity.article_number = String::new();
        doc.catalog.products[0].configuration.id = String::new();
        let issues = validate_structure(&doc.catalog);
        assert!(issues.iter().any(|d| d.severity == Severity::Error && d.field.starts_with("product.")));
        assert!(issues.iter().any(|d| d.severity == Severity::Warning && d.field.starts_with("configuration.")));
    }

    #[test]
    fn validate_structure_reports_unknown_record_family() {
        let mut doc = Document::default();
        doc.catalog.products[0].records.push(NativeRecord {
            family: RecordFamilyId("888".into()),
            fields: vec!["888".into()],
            extensions: ExtensionBag::default(),
        });
        let issues = validate_structure(&doc.catalog);
        assert!(issues.iter().any(|d| d.severity == Severity::Info && d.field.contains("888")));
    }

    #[test]
    fn characteristic_curve_interpolate_handles_edges() {
        let empty = CharacteristicCurve {
            id: "empty".into(),
            x_unit: VdiUnit::delta("%", VdiQuantityKind::Dimensionless, 0.01),
            y_unit: VdiUnit::absolute("m3/h", VdiQuantityKind::Volume, 1.0),
            points: Vec::new(),
        };
        assert_eq!(empty.interpolate(10.0), 0.0);

        let doc = Document::default();
        let curve = doc.curves.get("curve.kvs").expect("curve");
        assert_eq!(curve.interpolate(-10.0), curve.points[0].y);
        assert_eq!(curve.interpolate(1000.0), curve.points[curve.points.len() - 1].y);
    }

    #[test]
    fn linear_map_interpolates_and_handles_degenerate_domain() {
        assert!((linear_map(5.0, 0.0, 10.0, 0.0, 100.0) - 50.0).abs() < 1e-9);
        assert_eq!(linear_map(5.0, 3.0, 3.0, 7.0, 42.0), 7.0);
    }

    #[test]
    fn bounding_box_overlaps_detects_intersection_and_gap() {
        let a = BoundingBox::from_size(1.0, 1.0, 1.0);
        let b = BoundingBox { min_x: 0.5, min_y: 0.5, min_z: 0.5, max_x: 1.5, max_y: 1.5, max_z: 1.5 };
        assert!(a.overlaps(b, 0.0));
        let c = BoundingBox { min_x: 5.0, min_y: 5.0, min_z: 5.0, max_x: 6.0, max_y: 6.0, max_z: 6.0 };
        assert!(!a.overlaps(c, 0.0));
    }

    #[test]
    fn catalog_index_filter_by_sheet_and_tag() {
        let doc = Document::default();
        let by_sheet = doc.index.filter_by_sheet(SheetId(2));
        assert_eq!(by_sheet.len(), 1);
        let by_tag = doc.index.filter_by_tag("control valve");
        assert_eq!(by_tag.len(), 1);
        assert!(doc.index.filter_by_tag("nonexistent-tag").is_empty());
    }

    #[test]
    fn diagnostic_constructors_and_report_mapping() {
        let diags = vec![
            Diagnostic::error("f1", "bad"),
            Diagnostic::warning("f2", "meh"),
            Diagnostic::info("f3", "fyi"),
        ];
        let report = diagnostics_to_report(&diags, "1", "validate");
        assert_eq!(report[0].status, CheckStatus::Fail);
        assert_eq!(report[1].status, CheckStatus::Pass);
        assert_eq!(report[2].status, CheckStatus::Pass);
    }

    #[test]
    fn schema_registry_with_status_and_sheet_lookup() {
        let registry = SchemaRegistry::with_status(SchemaStatus::Reserved);
        assert!(registry.sheets().iter().all(|s| s.status == SchemaStatus::Reserved));
        let full = SchemaRegistry::current();
        let sheet = full.sheet(SheetId(2)).expect("sheet 2");
        assert_eq!(sheet.title_en, "Control valves heating");
        assert!(full.sheet(SheetId(9999)).is_none());
    }

    #[test]
    fn schema_registry_sheets_in_domain_and_reserved_numbers() {
        let registry = SchemaRegistry::current();
        let heating = registry.sheets_in_domain(Domain::Heating);
        assert!(heating.iter().any(|s| s.id == SheetId(2)));
        let reserved = registry.reserved_numbers();
        assert!(reserved.contains(&15));
        assert!(!reserved.contains(&2));
    }

    #[test]
    fn historical_part_check_respects_strict_mode() {
        let mut doc = Document::default();
        doc.strict_mode = true;
        let result = part_12::check(&doc);
        assert_eq!(result.status, CheckStatus::Fail);

        doc.strict_mode = false;
        let result = part_12::check(&doc);
        assert_eq!(result.status, CheckStatus::Pass);
    }

    #[test]
    fn multi_profile_part_check_reports_metadata_when_no_product() {
        let doc = Document::default();
        let result = part_08::check(&doc);
        assert_eq!(result.status, CheckStatus::Pass);
    }

    #[test]
    fn evaluate_reports_strict_mode_check() {
        let mut doc = Document::default();
        doc.strict_mode = true;
        let report = evaluate(&doc);
        assert!(report.checks.iter().any(|c| c.clause.section == "strict"));
    }

    #[test]
    fn evaluate_skips_geometry_and_curve_checks_when_absent() {
        let mut doc = Document::default();
        doc.geometry.clear();
        doc.curves.clear();
        let report = evaluate(&doc);
        assert!(!report.checks.iter().any(|c| c.clause.part == "geometry"));
        assert!(!report.checks.iter().any(|c| c.clause.part == "functions"));
    }

    #[test]
    fn sheet_id_part_str_and_edition_id_key() {
        assert_eq!(SheetId(42).part_str(), "42");
        assert!(EditionId::new(2023, 3).key() > EditionId::new(2022, 6).key());
    }

    #[test]
    fn schema_status_is_operative() {
        assert!(SchemaStatus::Published.is_operative());
        assert!(SchemaStatus::Checked.is_operative());
        assert!(!SchemaStatus::Draft.is_operative());
        assert!(!SchemaStatus::Reserved.is_operative());
    }

    #[test]
    fn record_family_id_all_known_contains_expected() {
        let known = RecordFamilyId::all_known();
        assert!(known.contains(&RecordFamilyId::R010));
        assert!(known.contains(&RecordFamilyId::R970_41));
    }

    #[test]
    fn catalog_and_document_json_round_trip() {
        let doc = Document::default();
        let json = catalog_to_json(&doc.catalog).expect("to_json");
        let restored = catalog_from_json(&json).expect("from_json");
        assert_eq!(restored.products.len(), doc.catalog.products.len());
        assert!(catalog_from_json("not json").is_err());

        let doc_json = document_to_json(&doc).expect("doc to_json");
        let restored_doc = document_from_json(&doc_json).expect("doc from_json");
        assert_eq!(restored_doc.strict_mode, doc.strict_mode);
        assert!(document_from_json("not json").is_err());
    }

    #[test]
    fn manufacturer_catalog_product_for_sheet() {
        let doc = Document::default();
        assert!(doc.catalog.product_for_sheet(SheetId(2)).is_some());
        assert!(doc.catalog.product_for_sheet(SheetId(3)).is_none());
    }

    // #region 🔖DslTests
    #[test]
    fn document_dsl_round_trips_the_reference_fixture() {
        store::test_support::assert_dsl_round_trip(&reference_fixture());
    }

    #[test]
    // 🪲 Blocked on the confirmed upstream `pack` crate bug root-caused by the `draw` wave-2 family
    // (`.repo/🎫/26/07/27/PACK-BINARY-DOCUMENT-LAYER-ACROSS-ALL-APPS/wave2-draw.txt` §4):
    // `pack/value/rs/lib.rs`'s `decode_table_soa` fallback branch drops the column's `Shape` (passes
    // `None` where `encode_table`'s matching branch passes `Some(&field.shape)`), so a `#[dsl(table)]`
    fn document_dsl_pack_equivalence_the_reference_fixture() {
        store::test_support::assert_dsl_pack_equivalence(&reference_fixture());
    }

    #[test]
    fn set_document_operation_op_text_round_trips_for_vdi3805() {
        store::test_support::assert_op_line_round_trip(&Operation::SetDocument { document: reference_fixture() });
    }

    #[test]
    fn document_text_round_trips_through_a_vcs_store() {
        let envelope = store::create_document_envelope(VDI3805_EXTENSION, "vdi3805.demo", reference_fixture(), None);
        let mut store = Vdi3805Store::new(envelope);
        let mut mutated = reference_fixture();
        mutated.strict_mode = true;
        store
            .dispatch(store::DocumentCommand::Apply { operations: vec![Operation::SetDocument { document: mutated }], description: None })
            .expect("apply");
        store::test_support::assert_document_text_round_trip(&store);
    }

    #[test]
    fn document_pack_round_trips_through_a_vcs_store() {
        let envelope = store::create_document_envelope(VDI3805_EXTENSION, "vdi3805.demo", reference_fixture(), None);
        let mut store = Vdi3805Store::new(envelope);
        let mut mutated = reference_fixture();
        mutated.strict_mode = true;
        store
            .dispatch(store::DocumentCommand::Apply { operations: vec![Operation::SetDocument { document: mutated }], description: None })
            .expect("apply");
        store::test_support::assert_document_pack_round_trip(&store);
    }
    // #endregion 🔖DslTests
}

