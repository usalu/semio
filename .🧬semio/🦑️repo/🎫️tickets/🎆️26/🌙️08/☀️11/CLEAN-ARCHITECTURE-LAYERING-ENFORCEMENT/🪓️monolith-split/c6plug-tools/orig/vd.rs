//! 🧬️ Vdi3805 snapshot schema — artifact-lane fields only.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use framework_schema::ArtifactSchema;
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, ArtifactSchema, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[dsl(id = "norm.vdi3805", layout = "lines")]
#[artifact_schema(id = "s.norm.vdi3805")]
pub struct Vdi3805Snapshot {
    #[state(artifact)]
    pub catalog: ManufacturerCatalog,
    #[state(artifact)]
    pub edition_profile: BTreeMap<String, EditionProfileChoice>,
    #[state(artifact)]
    pub correction_as_of: EditionId,
    #[state(artifact)]
    pub strict_mode: bool,
    #[state(artifact)]
    pub index: CatalogIndex,
    #[state(artifact)]
    pub geometry: BTreeMap<String, ParametricGeometry>,
    #[state(artifact)]
    pub curves: BTreeMap<String, CharacteristicCurve>,
    #[state(artifact)]
    pub limits: SecurityLimits,
}

impl Default for Vdi3805Snapshot {
    fn default() -> Self {
        crate::reference_fixture()
    }
}

pub fn attributes_from_records(sheet: SheetId, records: &[NativeRecord]) -> SheetAttributes {
    let mut kv: BTreeMap<String, String> = BTreeMap::new();
    for record in records {
        if record.family.0.as_str() == "210" {
            let mut it = record.fields.iter().skip(1);
            while let (Some(k), Some(v)) = (it.next(), it.next()) {
                if parse_f64(k).is_none() {
                    kv.insert(k.to_lowercase(), v.clone());
                }
            }
        } else if record.family.0.as_str() == "110" && record.fields.len() > 4 {
            let mut it = record.fields.iter().skip(4);
            while let (Some(k), Some(v)) = (it.next(), it.next()) {
                if parse_f64(k).is_none() {
                    kv.insert(k.to_lowercase(), v.clone());
                }
            }
        } else if record.family.0.as_str() == "200" && record.fields.len() > 5 {
            let mut it = record.fields.iter().skip(5);
            while let (Some(k), Some(v)) = (it.next(), it.next()) {
                if parse_f64(k).is_none() {
                    kv.insert(k.to_lowercase(), v.clone());
                }
            }
        }
        if record.family.0 == RecordFamilyId::R100 {
            if let Some(group) = record.fields.get(2) {
                kv.entry("product_group".into()).or_insert_with(|| group.clone());
            }
        }
    }
    match sheet.0 {
        2 => {
            let dn = kv.get("dn").and_then(|s| s.parse().ok()).unwrap_or(0);
            let kvs_m3_h = kv.get("kvs").and_then(|s| parse_f64(s)).unwrap_or(0.0);
            SheetAttributes::ValveHeating(ValveHeatingAttributes::from_kvs_m3_h(
                dn,
                kvs_m3_h,
                kv.get("pressure_class").cloned().unwrap_or_default(),
                kv.get("connection_type").cloned().unwrap_or_default(),
                kv.get("authority_min").and_then(|s| parse_f64(s)).unwrap_or(0.0),
                kv.get("authority_max").and_then(|s| parse_f64(s)).unwrap_or(0.0),
            ))
        }
        3 => SheetAttributes::Radiator(RadiatorAttributes {
            standard_output_w: kv.get("standard_output_w").or(kv.get("phi")).and_then(|s| parse_f64(s)).unwrap_or(0.0),
            heat_exponent_n: kv.get("n").or(kv.get("heat_exponent_n")).and_then(|s| parse_f64(s)).unwrap_or(0.0),
            length_m: kv.get("length_m").or(kv.get("l")).and_then(|s| parse_f64(s)).unwrap_or(0.0),
            height_m: kv.get("height_m").or(kv.get("h")).and_then(|s| parse_f64(s)).unwrap_or(0.0),
            depth_m: kv.get("depth_m").or(kv.get("d")).and_then(|s| parse_f64(s)).unwrap_or(0.0),
            connection_type: kv.get("connection_type").cloned().unwrap_or_default(),
        }),
        5 => SheetAttributes::PumpHeating(PumpHeatingAttributes {
            dn_suction: kv.get("dn_suction").and_then(|s| s.parse().ok()).unwrap_or(0),
            dn_discharge: kv.get("dn_discharge").or(kv.get("dn")).and_then(|s| s.parse().ok()).unwrap_or(0),
            nominal_flow_m3_s: kv.get("q").or(kv.get("nominal_flow_m3_s")).and_then(|s| parse_f64(s)).map(|q| if q > 1.0 { q / 3600.0 } else { q }).unwrap_or(0.0),
            nominal_head_m: kv.get("h").or(kv.get("nominal_head_m")).and_then(|s| parse_f64(s)).unwrap_or(0.0),
            motor_power_w: kv.get("p").or(kv.get("motor_power_w")).and_then(|s| parse_f64(s)).unwrap_or(0.0),
            hydraulic_efficiency: kv.get("eta").or(kv.get("hydraulic_efficiency")).and_then(|s| parse_f64(s)).unwrap_or(0.0),
            qh_curve_ref: kv.get("qh_curve_ref").cloned(),
        }),
        6 => SheetAttributes::HeatGenerator(HeatGeneratorAttributes {
            nominal_heat_output_w: kv.get("qn").or(kv.get("nominal_heat_output_w")).and_then(|s| parse_f64(s)).unwrap_or(0.0),
            fuel_type: kv.get("fuel_type").cloned().unwrap_or_default(),
            flow_temp_max_c: kv.get("flow_temp_max_c").and_then(|s| parse_f64(s)).unwrap_or(0.0),
            return_temp_min_c: kv.get("return_temp_min_c").and_then(|s| parse_f64(s)).unwrap_or(0.0),
        }),
        _ => SheetAttributes::Generic(GenericAttributes::from_map(&kv)),
    }
}
pub(crate) fn parse_f64(raw: &str) -> Option<f64> {
    raw.replace(',', ".").parse().ok()
}

//#region 🍃️FacetLeaves
/// 🍃️ The five handcrafted leaves of the `snapshot` schema facet.
pub const SNAPSHOT_LEAVES: semio_framework_schema_registry::FacetLeaves = semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") };
//#endregion 🍃️FacetLeaves

use std::collections::BTreeMap;
use semio_s_plugin_norm_compliance::LocalizedText;
use semio_s_plugin_norm_compliance::NormError;
use semio_s_plugin_norm_compliance::QuantityKind;

use semio_s_plugin_norm_app_surface;

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

/// 🧩️ Typed extension bag (`string → string`) matching JSON Schema `ExtensionFields.fields`.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ExtensionBag {
    pub fields: BTreeMap<String, String>,
}

/// 🆔️ Product identity within a manufacturer catalogue.
#[derive(Clone, Debug, PartialEq, Eq, Hash, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ProductIdentity {
    pub manufacturer_code: String,
    pub product_group: String,
    pub article_number: String,
}

/// 🏭️ Manufacturer file header and payload references.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
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
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct AccessoryLink {
    pub accessory_id: String,
    pub required: bool,
    pub quantity: u32,
}

/// 🧱️ Composition relationship (`hasPart`).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct CompositionLink {
    pub component_id: String,
    pub quantity: u32,
}

/// 🔒️ Security limits for untrusted manufacturer files.
#[derive(Clone, Copy, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
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

/// 🏷️ Connection-type codes shared by catalogue tables, field-meta, and evaluate (VDI 3805-1 §6.2).
pub const CONNECTION_TYPE_CODES: &[&str] = &["flange", "thread", "weld", "press"];

/// 🏷️ Connection-type rows: code, en, de.
pub const CONNECTION_TYPE_ROWS: &[(&str, &str, &str)] = &[
    ("flange", "Flanged", "Flansch"),
    ("thread", "Threaded", "Gewinde"),
    ("weld", "Welded", "Schweißanschluss"),
    ("press", "Press fit", "Pressverbindung"),
];

/// 🏷️ PN pressure-class codes (DIN EN 1092 / VDI 3805 product sheets §4).
pub const PRESSURE_CLASS_CODES: &[&str] = &["PN6", "PN10", "PN16", "PN25", "PN40"];

/// 🏷️ PN rows with nominal bar pressure for catalogue cells.
pub const PRESSURE_CLASS_ROWS: &[(&str, f64)] = &[("PN6", 6.0), ("PN10", 10.0), ("PN16", 16.0), ("PN25", 25.0), ("PN40", 40.0)];

/// 🏷️ Filter classes Blatt 19 (ISO 16890 / EN 779 legacy — VDI 3805-19 Table 1).
pub const FILTER_CLASS_CODES: &[&str] = &["G4", "M5", "M6", "F7", "F8", "F9", "ePM1", "ePM2_5", "ePM10"];

/// 🏷️ Filter-class rows: code, en, de.
pub const FILTER_CLASS_ROWS: &[(&str, &str, &str)] = &[
    ("G4", "Coarse G4", "Grob G4"),
    ("M5", "Medium M5", "Mittel M5"),
    ("M6", "Medium M6", "Mittel M6"),
    ("F7", "Fine F7", "Fein F7"),
    ("F8", "Fine F8", "Fein F8"),
    ("F9", "Fine F9", "Fein F9"),
    ("ePM1", "ePM1 85%", "ePM1 85 %"),
    ("ePM2_5", "ePM2.5 70%", "ePM2,5 70 %"),
    ("ePM10", "ePM10 50%", "ePM10 50 %"),
];

/// 🏷️ Generic type_code tokens (VDI 3805 multi-profile sheets §4.1).
pub const TYPE_CODE_CODES: &[&str] = &["STD", "STD-A", "STD-B", "TYPE-A", "TYPE-B", "TYPE-C", "DEFAULT"];

/// 📏 Blatt-sourced numeric domains used by evaluate and catalogue (min, max).
pub const SHEET_NUMERIC_BOUNDS_19_AIRFLOW: (f64, f64) = (1e-5, 50.0);

pub const SHEET_NUMERIC_BOUNDS_16_PRESSURE_DROP_PA: (f64, f64) = (0.0, 5000.0);

pub const SHEET_NUMERIC_BOUNDS_53_COP: (f64, f64) = (1.0, 10.0);

pub const SHEET_NUMERIC_BOUNDS_DN: (f64, f64) = (6.0, 600.0);

pub const SHEET_NUMERIC_BOUNDS_DN_PIPE: (f64, f64) = (6.0, 1200.0);

pub const SHEET_NUMERIC_BOUNDS_OUTER_DIAMETER_M: (f64, f64) = (0.006, 2.0);

pub const SHEET_NUMERIC_BOUNDS_WALL_THICKNESS_M: (f64, f64) = (0.0005, 0.1);

pub const SHEET_NUMERIC_BOUNDS_VOLUME_M3: (f64, f64) = (0.001, 500.0);

pub const SHEET_NUMERIC_BOUNDS_AXIAL_FORCE_N: (f64, f64) = (0.0, 1.0e7);

pub const SHEET_NUMERIC_BOUNDS_53_HEAT_W: (f64, f64) = (100.0, 5.0e6);

pub const SHEET_NUMERIC_BOUNDS_53_DN: (f64, f64) = (6.0, 300.0);

/// 📄️ Sheet identifier (1…100).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
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
impl semio_framework_dsl_record::BorrowedDslField for SheetId{const SHAPE:semio_framework_dsl_record::BorrowedShape=semio_framework_dsl_record::BorrowedShape::UInt;}

impl semio_framework_dsl_record::DslField for SheetId {
    fn shape_controlled<C:semio_framework_dsl_record::NativeSchemaControl>(control:&mut C)->Result<semio_framework_dsl_record::Shape,semio_framework_value::ValueError>{<u16 as semio_framework_dsl_record::DslField>::shape_controlled(control)}
    fn to_value_controlled(&self,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<semio_framework_dsl_record::FieldValue,semio_framework_value::ValueError>{<u16 as semio_framework_dsl_record::DslField>::to_value_controlled(&self.0,control)}

    fn from_value_controlled(value: &semio_framework_dsl_record::FieldValue, control: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self,semio_framework_value::ValueError> {
        <u16 as semio_framework_dsl_record::DslField>::from_value_controlled(value, control).map(Self)
    }
    fn shape() -> semio_framework_dsl_record::Shape {
        semio_framework_dsl_record::Shape::UInt
    }
    fn to_value(&self) -> semio_framework_dsl_record::FieldValue {
        semio_framework_dsl_record::FieldValue::UInt(self.0 as u64)
    }
    fn from_value(value: &semio_framework_dsl_record::FieldValue) -> Result<Self, String> {
        match value {
            semio_framework_dsl_record::FieldValue::UInt(v) => u16::try_from(*v).map(Self).map_err(|_| format!("sheet id {v} out of range for u16")),
            other => Err(format!("expected UInt, found {other:?}")),
        }
    }
}

/// 📅️ Edition identifier (year + month).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
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

/// 📊️ Schema lifecycle status.
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
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
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
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

/// 🏗️ Parsed building-system number (Anlagenkennzeichen).
#[derive(Clone, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
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
#[derive(Clone, Debug, PartialEq, Eq, Hash, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(transparent)]
pub struct RecordFamilyId(pub String);

/// 🔗️ Hand `DslField` bridge for `RecordFamilyId`: a tuple ("newtype") struct has no named fields
/// for `#[derive(dsl::DslRecord)]` to enumerate, so it binds directly as `Shape::Text` instead of
/// changing its public tuple shape (used pervasively as `.0` across this crate).
impl semio_framework_dsl_record::BorrowedDslField for RecordFamilyId{const SHAPE:semio_framework_dsl_record::BorrowedShape=semio_framework_dsl_record::BorrowedShape::Text;}

impl semio_framework_dsl_record::DslField for RecordFamilyId {
    fn shape_controlled<C:semio_framework_dsl_record::NativeSchemaControl>(control:&mut C)->Result<semio_framework_dsl_record::Shape,semio_framework_value::ValueError>{<String as semio_framework_dsl_record::DslField>::shape_controlled(control)}
    fn to_value_controlled(&self,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<semio_framework_dsl_record::FieldValue,semio_framework_value::ValueError>{<String as semio_framework_dsl_record::DslField>::to_value_controlled(&self.0,control)}

    fn from_value_controlled(value: &semio_framework_dsl_record::FieldValue, control: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self,semio_framework_value::ValueError> {
        <String as semio_framework_dsl_record::DslField>::from_value_controlled(value, control).map(Self)
    }
    fn shape() -> semio_framework_dsl_record::Shape {
        semio_framework_dsl_record::Shape::Text
    }
    fn to_value(&self) -> semio_framework_dsl_record::FieldValue {
        semio_framework_dsl_record::FieldValue::Text(self.0.clone())
    }
    fn from_value(value: &semio_framework_dsl_record::FieldValue) -> Result<Self, String> {
        match value {
            semio_framework_dsl_record::FieldValue::Text(s) => Ok(RecordFamilyId(s.clone())),
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
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct NativeRecord {
    pub family: RecordFamilyId,
    pub fields: Vec<String>,
    pub extensions: ExtensionBag,
}

/// 🎛 Sheet-2 heating control valve attributes (VDI 3805 Blatt 2).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ValveHeatingAttributes {
    pub dn: u16,
    pub kvs_m3_s: f64,
    pub pressure_class: String,
    pub connection_type: String,
    pub authority_min: f64,
    pub authority_max: f64,
}

impl ValveHeatingAttributes {
    pub fn kvs_m3_h(&self) -> f64 {
        self.kvs_m3_s * 3600.0
    }

    pub fn from_kvs_m3_h(dn: u16, kvs_m3_h: f64, pressure_class: impl Into<String>, connection_type: impl Into<String>, authority_min: f64, authority_max: f64) -> Self {
        Self { dn, kvs_m3_s: kvs_m3_h / 3600.0, pressure_class: pressure_class.into(), connection_type: connection_type.into(), authority_min, authority_max }
    }
}

/// ♨️ Sheet-3 radiator / heating-surface attributes (VDI 3805 Blatt 3; Φ at 75/65/20 °C).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct RadiatorAttributes {
    pub standard_output_w: f64,
    pub heat_exponent_n: f64,
    pub length_m: f64,
    pub height_m: f64,
    pub depth_m: f64,
    pub connection_type: String,
}

/// 💧 Sheet-5 heating pump attributes (VDI 3805 Blatt 5).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct PumpHeatingAttributes {
    pub dn_suction: u16,
    pub dn_discharge: u16,
    pub nominal_flow_m3_s: f64,
    pub nominal_head_m: f64,
    pub motor_power_w: f64,
    pub hydraulic_efficiency: f64,
    pub qh_curve_ref: Option<String>,
}

/// 🔥 Sheet-6 heat-generator attributes (VDI 3805 Blatt 6).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct HeatGeneratorAttributes {
    pub nominal_heat_output_w: f64,
    pub fuel_type: String,
    pub flow_temp_max_c: f64,
    pub return_temp_min_c: f64,
}

/// 🧩 Single generic Blatt attribute (mirrors a native 210 key/value[/unit] pair).
#[derive(Clone, Debug, PartialEq, Default, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct GenericAttribute {
    pub key: String,
    pub value: String,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default)]
    pub unit: Option<String>,
}

/// 📋 Generic sheet attributes as an ordered key/value/unit catalogue mirroring native records.
#[derive(Clone, Debug, PartialEq, Default, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct GenericAttributes {
    #[dsl(table)]
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub entries: Vec<GenericAttribute>,
}

impl GenericAttributes {
    pub fn from_map(kv: &BTreeMap<String, String>) -> Self {
        Self {
            entries: kv
                .iter()
                .map(|(k, v)| GenericAttribute {
                    key: k.clone(),
                    value: v.clone(),
                    unit: None,
                })
                .collect(),
        }
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.entries.iter().find(|e| e.key.eq_ignore_ascii_case(key)).map(|e| e.value.as_str())
    }

    pub fn ensure_keys(&mut self, keys: &[&str]) {
        for key in keys {
            if self.get(key).is_none() {
                self.entries.push(GenericAttribute {
                    key: (*key).to_string(),
                    value: String::new(),
                    unit: None,
                });
            }
        }
    }
}

/// 🎛 Typed per-sheet mandatory attribute payload — replaces the untyped parameters map.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(tag = "kind", rename_all = "camelCase"))]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum SheetAttributes {
    ValveHeating(ValveHeatingAttributes),
    Radiator(RadiatorAttributes),
    PumpHeating(PumpHeatingAttributes),
    HeatGenerator(HeatGeneratorAttributes),
    Generic(GenericAttributes),
}

impl SheetAttributes {
    pub fn dn(&self) -> Option<u16> {
        match self {
            Self::ValveHeating(a) => Some(a.dn),
            Self::PumpHeating(a) => Some(a.dn_discharge),
            _ => None,
        }
    }
}

impl semio_framework_dsl_record::BorrowedDslField for SheetAttributes{const SHAPE:semio_framework_dsl_record::BorrowedShape=semio_framework_dsl_record::BorrowedShape::Value;}

impl semio_framework_dsl_record::DslField for SheetAttributes {
    fn shape_controlled<C:semio_framework_dsl_record::NativeSchemaControl>(control:&mut C)->Result<semio_framework_dsl_record::Shape,semio_framework_value::ValueError>{control.checkpoint()?;Ok(semio_framework_dsl_record::Shape::Value)}
    fn to_value_controlled(&self,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<semio_framework_dsl_record::FieldValue,semio_framework_value::ValueError>{<Self as semio_framework_value::ToValue>::to_value_controlled(self,control).map(semio_framework_dsl_record::FieldValue::Value)}

    fn from_value_controlled(value: &semio_framework_dsl_record::FieldValue, control: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self,semio_framework_value::ValueError> {
        match value {
            semio_framework_dsl_record::FieldValue::Value(value) => sheet_attributes_controlled(value, control),
            _ => Err(vdi3805_invalid("expected VDI3805 sheet attribute value")),
        }
    }
    fn shape() -> semio_framework_dsl_record::Shape {
        semio_framework_dsl_record::Shape::Value
    }
    fn to_value(&self) -> semio_framework_dsl_record::FieldValue {
        semio_framework_dsl_record::FieldValue::Value(semio_framework_value::ToValue::to_value(self))
    }
    fn from_value(value: &semio_framework_dsl_record::FieldValue) -> Result<Self, String> {
        match value {
            semio_framework_dsl_record::FieldValue::Value(dsl_value) => {
                semio_framework_value::FromValue::from_value(dsl_value.clone()).map_err(|error|error.to_string())
            }
            other => Err(format!("expected Value, found {other:?}")),
        }
    }
}

/// 🚫️ An invalid VDI 3805 attribute value.
fn vdi3805_invalid(message: impl Into<String>) -> semio_framework_value::ValueError {
    semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, message)
}

pub fn sheet_attribute_object<'a>(value: &'a semio_framework_value::DslValue, allowed: &[&str], control: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<&'a [(String, semio_framework_value::DslValue)],semio_framework_value::ValueError> {
    let semio_framework_value::DslValue::Object(fields) = value else { return Err(vdi3805_invalid("expected VDI3805 attribute object")); };
    if fields.len() > allowed.len() { return Err(vdi3805_invalid("VDI3805 attribute object has excess fields")); }
    for (position, (key, _)) in fields.iter().enumerate() {
        control.step()?;
        if !allowed.contains(&key.as_str()) || fields[..position].iter().any(|(prior, _)| prior == key) { return Err(vdi3805_invalid("VDI3805 attribute object has unknown or duplicate fields")); }
    }
    Ok(fields)
}

pub fn sheet_attribute_field<'a>(fields: &'a [(String, semio_framework_value::DslValue)], key: &str) -> Result<&'a semio_framework_value::DslValue,semio_framework_value::ValueError> {
    fields.iter().find(|(name, _)| name == key).map(|(_, value)| value).ok_or_else(||vdi3805_invalid(format!("VDI3805 attribute requires {key}")))
}

pub fn sheet_attribute_real(fields: &[(String, semio_framework_value::DslValue)], key: &str) -> Result<f64,semio_framework_value::ValueError> {
    sheet_attribute_field(fields, key)?.as_f64().ok_or_else(||vdi3805_invalid(format!("VDI3805 attribute {key} requires a number")))
}

pub fn sheet_attribute_u16(fields: &[(String, semio_framework_value::DslValue)], key: &str) -> Result<u16,semio_framework_value::ValueError> {
    let word = sheet_attribute_field(fields, key)?.as_u64().ok_or_else(||vdi3805_invalid("expected unsigned VDI3805 attribute"))?;
    u16::try_from(word).map_err(|_|vdi3805_invalid("VDI3805 attribute exceeds u16"))
}

/// ⚙️ Product configuration block.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Configuration {
    pub id: String,
    pub attributes: SheetAttributes,
    pub geometry_ref: Option<String>,
    pub function_refs: Vec<String>,
}

/// 📦️ Catalogue product in Part 1 hierarchy.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct CatalogueProduct {
    pub id: String,
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
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
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

/// 📦️ Axis-aligned bounding box [m].
#[derive(Clone, Copy, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
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
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ConnectionPoint {
    pub id: String,
    pub medium: String,
    pub position: [f64; 3],
    pub direction: [f64; 3],
    #[dsl(unit = "mm")]
    pub diameter_mm: Option<f64>,
}

/// 🧊️ Parametric geometry definition.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
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

/// 📈️ Characteristic curve point.
#[derive(Clone, Copy, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct CurvePoint {
    pub x: f64,
    pub y: f64,
}

/// 📉️ Characteristic curve with linear interpolation.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
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

/// 🔍️ Product index entry.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct CatalogIndexEntry {
    pub product_id: String,
    pub sheet: SheetId,
    pub tags: Vec<String>,
    pub dn: Option<u16>,
}

/// 📚️ Searchable catalogue index.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct CatalogIndex {
    #[dsl(table)]
    pub entries: Vec<CatalogIndexEntry>,
}

impl CatalogIndex {
    pub fn from_catalog(catalog: &ManufacturerCatalog) -> Self {
        let mut entries: Vec<CatalogIndexEntry> = catalog
            .products
            .iter()
            .map(|p| CatalogIndexEntry {
                product_id: p.identity.article_number.clone(),
                sheet: p.sheet,
                tags: p.title.iter().map(|t| t.text.clone()).collect(),
                dn: p.configuration.attributes.dn(),
            })
            .collect();
        entries.sort_by(|a, b| a.product_id.cmp(&b.product_id));
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

/// 🩺️ Validation diagnostic with severity.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Diagnostic {
    pub field: String,
    pub message: String,
    pub severity: Severity,
}

/// ⚠️ Diagnostic severity.
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
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

/// 📅️ Edition profile selection for multi-profile sheets.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub enum EditionProfileChoice {
    Legacy,
    Current,
}

pub fn reference_fixture() -> Vdi3805Snapshot {
    conforming_valve_dataset()
}

/// ✅️ Fully conforming sheet-2 heating control valve dataset (Part 1 + Blatt 2).
pub fn conforming_valve_dataset() -> Vdi3805Snapshot {
    let bsn = BuildingSystemNumber { system_code: "420".into(), subsystem: "10".into(), sequence: 1 };
    let file = ManufacturerFile {
        header_version: "3805".into(),
        manufacturer: "DEMO".into(),
        building_system_number: bsn,
        created: "2023-06-01".into(),
        charset: "UTF-8".into(),
        record_count: 11,
        extensions: ExtensionBag::default(),
    };
    let attributes = SheetAttributes::ValveHeating(ValveHeatingAttributes::from_kvs_m3_h(50, 4.5, "PN16", "flange", 0.3, 0.7));
    let product = CatalogueProduct {
        id: "VLV-50-001".into(),
        identity: ProductIdentity { manufacturer_code: "DEMO".into(), product_group: "HV".into(), article_number: "VLV-50-001".into() },
        title: bilingual("Stellventil DN50", "Control valve DN50"),
        sheet: SheetId(2),
        records: vec![
            NativeRecord { family: RecordFamilyId(RecordFamilyId::R100.to_string()), fields: vec!["100".into(), "DEMO".into(), "HV".into(), "VLV-50-001".into(), "2".into()], extensions: ExtensionBag::default() },
            NativeRecord {
                family: RecordFamilyId(RecordFamilyId::R110.to_string()),
                fields: vec!["110".into(), "cfg.VLV-50-001".into(), "geom-valve-50".into(), "curve-kvs".into()],
                extensions: ExtensionBag::default(),
            },
            NativeRecord {
                family: RecordFamilyId(RecordFamilyId::R200.to_string()),
                fields: vec!["200".into(), "geom-valve-50".into(), "0.15".into(), "0.20".into(), "0.10".into()],
                extensions: ExtensionBag::default(),
            },
            NativeRecord {
                family: RecordFamilyId(RecordFamilyId::R210.to_string()),
                fields: vec![
                    "210".into(),
                    "dn".into(),
                    "50".into(),
                    "kvs".into(),
                    "4.5".into(),
                    "pressure_class".into(),
                    "PN16".into(),
                    "connection_type".into(),
                    "flange".into(),
                    "authority_min".into(),
                    "0.3".into(),
                    "authority_max".into(),
                    "0.7".into(),
                ],
                extensions: ExtensionBag::default(),
            },
            NativeRecord {
                family: RecordFamilyId(RecordFamilyId::R400.to_string()),
                fields: vec!["400".into(), "curve-kvs".into(), "0".into(), "0".into(), "100".into(), "4.5".into()],
                extensions: ExtensionBag::default(),
            },
            NativeRecord {
                family: RecordFamilyId(RecordFamilyId::R700.to_string()),
                fields: vec!["700".into(), "de".into(), "Stellventil DN50".into(), "en".into(), "Control valve DN50".into()],
                extensions: ExtensionBag::default(),
            },
        ],
        configuration: Configuration { id: "cfg.VLV-50-001".into(), attributes, geometry_ref: Some("geom-valve-50".into()), function_refs: vec!["curve-kvs".into()] },
        accessories: Vec::new(),
        components: Vec::new(),
        extensions: ExtensionBag::default(),
    };
    
    let accessory = CatalogueProduct {
        id: "ACT-01".into(),
        identity: ProductIdentity { manufacturer_code: "DEMO".into(), product_group: "ACT".into(), article_number: "ACT-01".into() },
        title: bilingual("Stellantrieb", "Actuator"),
        sheet: SheetId(2),
        records: vec![
            NativeRecord {
                family: RecordFamilyId(RecordFamilyId::R100.to_string()),
                fields: vec!["100".into(), "DEMO".into(), "ACT".into(), "ACT-01".into(), "2".into()],
                extensions: ExtensionBag::default(),
            },
            NativeRecord {
                family: RecordFamilyId(RecordFamilyId::R210.to_string()),
                fields: vec![
                    "210".into(),
                    "dn".into(),
                    "50".into(),
                    "kvs".into(),
                    "4.5".into(),
                    "pressure_class".into(),
                    "PN16".into(),
                    "connection_type".into(),
                    "flange".into(),
                    "authority_min".into(),
                    "0.3".into(),
                    "authority_max".into(),
                    "0.7".into(),
                ],
                extensions: ExtensionBag::default(),
            },
            NativeRecord {
                family: RecordFamilyId(RecordFamilyId::R700.to_string()),
                fields: vec!["700".into(), "de".into(), "Stellantrieb".into(), "en".into(), "Actuator".into()],
                extensions: ExtensionBag::default(),
            },
        ],
        configuration: Configuration {
            id: "cfg.ACT-01".into(),
            attributes: SheetAttributes::ValveHeating(ValveHeatingAttributes::from_kvs_m3_h(50, 4.5, "PN16", "flange", 0.3, 0.7)),
            geometry_ref: None,
            function_refs: vec![],
        },
        accessories: Vec::new(),
        components: Vec::new(),
        extensions: ExtensionBag::default(),
    };
    let mut product = product;
    product.accessories = vec![AccessoryLink { accessory_id: "ACT-01".into(), required: true, quantity: 1 }];
    product.components = vec![CompositionLink { component_id: "ACT-01".into(), quantity: 1 }];
    let mut catalog = ManufacturerCatalog { file: file.clone(), products: vec![product, accessory], extensions: ExtensionBag::default() };

    let geometry = BTreeMap::from([(
        "geom-valve-50".into(),
        ParametricGeometry {
            id: "geom-valve-50".into(),
            bbox: BoundingBox::from_size(0.15, 0.20, 0.10),
            connections: vec![ConnectionPoint {
                id: "in".into(),
                medium: "water".into(),
                position: [0.0, 0.1, 0.05],
                direction: [-1.0, 0.0, 0.0],
                diameter_mm: Some(50.0),
            }],
            parameters: BTreeMap::from([("scale".into(), 1.0), ("clearance".into(), 0.01)]),
        },
    )]);
    let curves = BTreeMap::from([(
        "curve-kvs".into(),
        CharacteristicCurve {
            id: "curve-kvs".into(),
            x_unit: VdiUnit::delta("%", VdiQuantityKind::Dimensionless, 0.01),
            y_unit: VdiUnit::absolute("m3/h", VdiQuantityKind::Volume, 1.0 / 3600.0),
            points: vec![CurvePoint { x: 0.0, y: 0.0 }, CurvePoint { x: 100.0, y: 4.5 }],
        },
    )]);
    catalog.file.record_count = catalog.products.iter().map(|p| p.records.len() as u32).sum();
    let index = CatalogIndex::from_catalog(&catalog);
    Vdi3805Snapshot {
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

/// 🗿️ The computed-compliance artifact this standard publishes on its app's `report:out` port —
/// lifted out of the pre-migration manifest's inline `.artifact_kind(ArtifactKindSpec { .. })` so the
/// artifact node, not the app, owns its own kind declaration.
pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    app_surface::artifact_kind_spec("vdi3805", "VDI 3805", VDI3805_DOCUMENT_SCHEMA)
}

/// 🪪️ This subset's canonical `(artifact_kind, standard, subset)` coordinate (ticket
/// 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §1) — lives at the ARTIFACT level, not
/// under the sibling `editor` module, so a viewer file can read it without ever importing through it.
pub const VDI3805_DIALECT: semio_framework_plugin::app::Dialect = semio_framework_plugin::app::Dialect { artifact_kind: "s.norm.vdi3805", standard: semio_framework_plugin::app::StandardId("1"), subset: semio_framework_plugin::app::SubsetId::ANY };

pub const VDI3805_DOCUMENT_SCHEMA: &str = "semio.norm.vdi3805/v1";
