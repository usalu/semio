//! 🔧️ VDI 3805 manufacturer product data for building services: Part 1 + sheets 2–100 — document entities.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

/// 📜 Language-neutral package declaration owned by this artifact.
pub const ARTIFACT_DEFINITION_SCHEMA: &str = include_str!("📜️artifact-definition.json");

/// 📦 Validates this artifact's independently compiled package identity.
pub fn package_descriptor() -> Result<semio_s_artifact_norm_contract::NormArtifactPackage, semio_s_artifact_norm_contract::PackageSchemaError> {
    semio_s_artifact_norm_contract::package_from_schema(ARTIFACT_DEFINITION_SCHEMA)
}

use semio_s_plugin_norm_compliance::{NormError, QuantityKind};
use std::collections::{BTreeMap, BTreeSet};
use semio_s_artifact_norm_vdi3805_model::VdiQuantityKind;
use semio_s_artifact_norm_vdi3805_model::EditionId;
use semio_s_artifact_norm_vdi3805_model::SchemaStatus;
use semio_s_artifact_norm_vdi3805_model::SheetAttributes;
use semio_s_artifact_norm_vdi3805_model::RecordFamilyId;
use semio_s_artifact_norm_vdi3805_model::GenericAttributes;
use semio_s_artifact_norm_vdi3805_model::CatalogIndex;
use semio_s_artifact_norm_vdi3805_model::SecurityLimits;
use semio_s_artifact_norm_vdi3805_model::VdiUnit;
use semio_s_artifact_norm_vdi3805_model::SheetEntry;
use semio_s_artifact_norm_vdi3805_model::GenericAttribute;
use semio_s_artifact_norm_vdi3805_model::CatalogueProduct;
use semio_s_artifact_norm_vdi3805_model::Vdi3805Snapshot;

// #region Shared
/// 🌐️ Locale-tagged text — re-exported from `crate::document`, the single canonical definition
/// shared across every norm artifact (kills this type's former duplicate here and in `iso16757`;
/// see `crate::document`'s `🔖️LocalizedText` region for the full rationale).

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

/// 🔢️ Typed manufacturer value.
#[derive(Clone, Debug, PartialEq, crate::value_derive::ToValue, crate::value_derive::FromValue)]
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
impl semio_framework_dsl_record::DslField for VdiValue {
    fn shape() -> semio_framework_dsl_record::Shape {
        semio_framework_dsl_record::Shape::Value
    }
    fn to_value(&self) -> semio_framework_dsl_record::FieldValue {
        semio_framework_dsl_record::FieldValue::Value(semio_framework_value::ToValue::to_value(self))
    }
    fn from_value(value: &semio_framework_dsl_record::FieldValue) -> Result<Self, String> {
        match value {
            semio_framework_dsl_record::FieldValue::Value(dsl_value) => {
                let normalized = crate::store::pack_rt::renormalize_whole_number_floats(dsl_value.clone());
                semio_framework_value::FromValue::from_value(normalized).map_err(|error|error.to_string())
            }
            other => Err(format!("expected Value, found {other:?}")),
        }
    }
}

// #endregion Shared

// #region Schema

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

fn sheet_attribute_text(fields: &[(String, semio_framework_value::DslValue)], key: &str, control: &mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>) -> Result<String,semio_framework_value::ValueError> {
    control.copy_text(sheet_attribute_field(fields, key)?.as_str().ok_or_else(||vdi3805_invalid("expected VDI3805 attribute text"))?)
}
fn sheet_attribute_optional_text(fields: &[(String, semio_framework_value::DslValue)], key: &str, control: &mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>) -> Result<Option<String>,semio_framework_value::ValueError> {
    match fields.iter().find(|(name, _)| name == key).map(|(_, value)| value) {
        None | Some(semio_framework_value::DslValue::Null) => Ok(None),
        Some(semio_framework_value::DslValue::String(text)) => control.copy_text(text).map(Some),
        _ => Err(vdi3805_invalid("expected optional VDI3805 attribute text")),
    }
}
fn sheet_attributes_controlled(value: &semio_framework_value::DslValue, control: &mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>) -> Result<SheetAttributes,semio_framework_value::ValueError> {
    control.step()?;
    let semio_framework_value::DslValue::Object(fields) = value else { return Err(vdi3805_invalid("expected VDI3805 sheet attribute object")); };
    if fields.len() > 8 { return Err(vdi3805_invalid("VDI3805 sheet attribute object has excess fields")); }
    let kind = sheet_attribute_field(fields, "kind")?.as_str().ok_or_else(||vdi3805_invalid("expected VDI3805 attribute kind"))?;
    match kind {
        "valveHeating" => {
            let fields = sheet_attribute_object(value, &["kind", "dn", "kvsM3S", "pressureClass", "connectionType", "authorityMin", "authorityMax"], control)?;
            Ok(SheetAttributes::ValveHeating(ValveHeatingAttributes {
                dn: sheet_attribute_u16(fields, "dn")?, kvs_m3_s: sheet_attribute_real(fields, "kvsM3S")?,
                pressure_class: sheet_attribute_text(fields, "pressureClass", control)?, connection_type: sheet_attribute_text(fields, "connectionType", control)?,
                authority_min: sheet_attribute_real(fields, "authorityMin")?, authority_max: sheet_attribute_real(fields, "authorityMax")?,
            }))
        }
        "radiator" => {
            let fields = sheet_attribute_object(value, &["kind", "standardOutputW", "heatExponentN", "lengthM", "heightM", "depthM", "connectionType"], control)?;
            Ok(SheetAttributes::Radiator(RadiatorAttributes {
                standard_output_w: sheet_attribute_real(fields, "standardOutputW")?, heat_exponent_n: sheet_attribute_real(fields, "heatExponentN")?,
                length_m: sheet_attribute_real(fields, "lengthM")?, height_m: sheet_attribute_real(fields, "heightM")?, depth_m: sheet_attribute_real(fields, "depthM")?,
                connection_type: sheet_attribute_text(fields, "connectionType", control)?,
            }))
        }
        "pumpHeating" => {
            let fields = sheet_attribute_object(value, &["kind", "dnSuction", "dnDischarge", "nominalFlowM3S", "nominalHeadM", "motorPowerW", "hydraulicEfficiency", "qhCurveRef"], control)?;
            Ok(SheetAttributes::PumpHeating(PumpHeatingAttributes {
                dn_suction: sheet_attribute_u16(fields, "dnSuction")?, dn_discharge: sheet_attribute_u16(fields, "dnDischarge")?,
                nominal_flow_m3_s: sheet_attribute_real(fields, "nominalFlowM3S")?, nominal_head_m: sheet_attribute_real(fields, "nominalHeadM")?,
                motor_power_w: sheet_attribute_real(fields, "motorPowerW")?, hydraulic_efficiency: sheet_attribute_real(fields, "hydraulicEfficiency")?,
                qh_curve_ref: sheet_attribute_optional_text(fields, "qhCurveRef", control)?,
            }))
        }
        "heatGenerator" => {
            let fields = sheet_attribute_object(value, &["kind", "nominalHeatOutputW", "fuelType", "flowTempMaxC", "returnTempMinC"], control)?;
            Ok(SheetAttributes::HeatGenerator(HeatGeneratorAttributes {
                nominal_heat_output_w: sheet_attribute_real(fields, "nominalHeatOutputW")?, fuel_type: sheet_attribute_text(fields, "fuelType", control)?,
                flow_temp_max_c: sheet_attribute_real(fields, "flowTempMaxC")?, return_temp_min_c: sheet_attribute_real(fields, "returnTempMinC")?,
            }))
        }
        "generic" => {
            let fields = sheet_attribute_object(value, &["kind", "entries"], control)?;
            let values = match fields.iter().find(|(key, _)| key == "entries").map(|(_, value)| value) {
                None => &[][..], Some(semio_framework_value::DslValue::Array(values)) => values.as_slice(),
                _ => return Err(vdi3805_invalid("expected VDI3805 generic attribute list")),
            };
            let mut entries = control.allocate_vec::<GenericAttribute>(values.len())?;
            for value in values {
                let fields = sheet_attribute_object(value, &["key", "value", "unit"], control)?;
                entries.push(GenericAttribute { key: sheet_attribute_text(fields, "key", control)?, value: sheet_attribute_text(fields, "value", control)?, unit: sheet_attribute_optional_text(fields, "unit", control)? });
            }
            Ok(SheetAttributes::Generic(GenericAttributes { entries }))
        }
        _ => Err(vdi3805_invalid("unknown VDI3805 sheet attribute kind")),
    }
}

// #endregion Part1

// #region Geometry

// #endregion Geometry

// #region Functions

// #endregion Functions

// #region Catalog

// #endregion Catalog

// #region Validate

// #endregion Validate

/// 📸️ Persisted snapshot — defined in `📸️snapshot/🧬️schema`, re-exported here.
// #region Session

/// 🏷️ Canonical DSL file extension for VDI 3805 documents.
pub const VDI3805_EXTENSION: &str = "vdi3805";


// #endregion Session

//#region 🪪️Declaration
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) — replaces
/// the old side-effecting `register()`/`register_pilot_languages()`/`register_artifact_schema()`/
/// `register_artifact_inferences()`/`register_io()`, each of which called a global registry directly
/// from the plugin root's `.setup()` fan-out (`register_norm_exports`, deleted by this same wave).
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    package_descriptor().map_err(|error| semio_framework_plugin::ArtifactDefinitionError::new("artifact.package-schema", error.to_string()))?;
    use semio_s_artifact_norm_contract::definition::{CapabilitySpec as C, ClaimSpec as Q, LocalizationSpec as L};
    const S: &[Q] = &[Q { namespace: "schema", value: "s.norm.vdi3805" }];
    const I: &[Q] = &[Q { namespace: "schema", value: "s.norm.vdi3805.inference" }];
    const M: &[Q] = &[Q { namespace: "dialect", value: "s.norm.vdi3805@1/*" }];
    const K: &[Q] = &[Q { namespace: "codec", value: "semio.norm.vdi3805/v1" }, Q { namespace: "codec-extension", value: "21:semio.norm.vdi3805/v1:vdi3805" }];
    const EN: &[L] = &[L { locale: "en", text: "VDI 3805 manufacturer product data" }];
    const DE: &[L] = &[L { locale: "de", text: "VDI 3805 Produktdaten der Technischen Gebäudeausrüstung" }];
    const ROWS: &[C] = &[
        C { identity: "s.norm.vdi3805.standard.v1", kind: "standard", descriptor: "v1", claims: &[], localizations: &[] },
        C { identity: "s.norm.vdi3805.standard.v1.profile.any", kind: "profile", descriptor: "any", claims: &[], localizations: &[] },
        C { identity: "s.norm.vdi3805.schema.artifact", kind: "schema", descriptor: "s.norm.vdi3805", claims: S, localizations: &[] },
        C { identity: "s.norm.vdi3805.inference.outline", kind: "inference", descriptor: "s.norm.vdi3805.inference", claims: I, localizations: &[] },
        C { identity: "s.norm.vdi3805.composer.any", kind: "composer", descriptor: "s.norm.vdi3805@1/*", claims: M, localizations: &[] },
        C { identity: "s.norm.vdi3805.grammar.document", kind: "grammar", descriptor: "vdi3805.document", claims: &[Q { namespace: "grammar", value: "vdi3805.document" }], localizations: &[] },
        C { identity: "s.norm.vdi3805.grammar.op", kind: "grammar", descriptor: "vdi3805.op", claims: &[Q { namespace: "grammar", value: "vdi3805.op" }], localizations: &[] },
        C { identity: "s.norm.vdi3805.grammar.diff", kind: "grammar", descriptor: "vdi3805.diff", claims: &[Q { namespace: "grammar", value: "vdi3805.diff" }], localizations: &[] },
        C { identity: "s.norm.vdi3805.grammar.pack", kind: "grammar", descriptor: "vdi3805.pack", claims: &[Q { namespace: "grammar", value: "vdi3805.pack" }], localizations: &[] },
        C { identity: "s.norm.vdi3805.grammar.spr", kind: "grammar", descriptor: "vdi3805.spr", claims: &[Q { namespace: "grammar", value: "vdi3805.spr" }], localizations: &[] },
        C { identity: "s.norm.vdi3805.codec.document.v1", kind: "codec", descriptor: "semio.norm.vdi3805/v1:vdi3805", claims: K, localizations: &[] },
        C { identity: "s.norm.vdi3805.localization.en", kind: "localization", descriptor: "VDI 3805 manufacturer product data", claims: &[], localizations: EN },
        C { identity: "s.norm.vdi3805.localization.de", kind: "localization", descriptor: "VDI 3805 Produktdaten der Technischen Gebäudeausrüstung", claims: &[], localizations: DE },
    ];
    semio_s_artifact_norm_contract::definition::assemble_definition("s.norm.vdi3805", ROWS)
}

pub fn declaration(definition: semio_framework_plugin::ArtifactDefinition) -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    semio_framework_plugin::ArtifactDeclaration::builder(definition)
        .schema(semio_s_artifact_norm_vdi3805_schema::vdi3805_artifact_schema_descriptor())
        .inferences([semio_s_artifact_norm_vdi3805_inferences::vdi3805_artifact_inference_descriptor()])
        .composers(semio_s_artifact_norm_vdi3805_io::io_registry::entries())
        .languages(pilot_languages())
        .document_codec::<semio_framework_plugin::EditorApp<semio_s_artifact_norm_vdi3805_editor::Vdi3805PlayApp>>()
        .try_build()
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built once
/// and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`, mirroring the
/// `OnceLock`-backed `io_registry::entries()` convention below.
fn pilot_languages() -> &'static [semio_framework_dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<semio_framework_dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                semio_framework_dsl::LanguageSpec {
                    id: "vdi3805.document",
                    extension: Some("vdi3805"),
                    role: semio_framework_dsl::LanguageRole::Document,
                    grammar: Some(semio_s_artifact_norm_vdi3805_io::text::snapshot::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(semio_s_artifact_norm_vdi3805_io::text::snapshot::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(semio_s_artifact_norm_vdi3805_io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(semio_s_artifact_norm_vdi3805_io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("vdi3805.document"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "vdi3805.op",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Ops,
                    grammar: Some(semio_s_artifact_norm_vdi3805_io::text::mutations::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(semio_s_artifact_norm_vdi3805_io::text::mutations::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(semio_s_artifact_norm_vdi3805_io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(semio_s_artifact_norm_vdi3805_io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("vdi3805.op"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "vdi3805.diff",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Diff,
                    grammar: Some(semio_s_artifact_norm_vdi3805_io::text::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(semio_s_artifact_norm_vdi3805_io::text::diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: semio_framework_dsl::passthrough_hooks("vdi3805.diff"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "vdi3805.pack",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(semio_s_artifact_norm_vdi3805_io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(semio_s_artifact_norm_vdi3805_io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("vdi3805.pack"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "vdi3805.spr",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(semio_s_artifact_norm_vdi3805_io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(semio_s_artifact_norm_vdi3805_io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("vdi3805.spr"),
                },
            ]
        })
        .as_slice()
}
//#endregion 🪪️Declaration

// ---- Shims: keep pre-migration module paths resolving for external callers ----

//#region 🪢️Mounts
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🧪️tests/🔬️unit/🦀️.rs"]
mod diff_unit;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📄️blatt-16/🧪️tests/🎩️example/🦀️.rs"]
mod examples_blatt_16_example;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📄️blatt-19/🧪️tests/🎩️example/🦀️.rs"]
mod examples_blatt_19_example;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📄️blatt-3/🧪️tests/🎩️example/🦀️.rs"]
mod examples_blatt_3_example;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📄️blatt-4/🧪️tests/🎩️example/🦀️.rs"]
mod examples_blatt_4_example;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📄️blatt-53/🧪️tests/🎩️example/🦀️.rs"]
mod examples_blatt_53_example;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📄️blatt-5/🧪️tests/🎩️example/🦀️.rs"]
mod examples_blatt_5_example;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📄️blatt-60/🧪️tests/🎩️example/🦀️.rs"]
mod examples_blatt_60_example;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📄️blatt-6/🧪️tests/🎩️example/🦀️.rs"]
mod examples_blatt_6_example;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📄️blatt-7/🧪️tests/🎩️example/🦀️.rs"]
mod examples_blatt_7_example;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📄️blatt-8/🧪️tests/🎩️example/🦀️.rs"]
mod examples_blatt_8_example;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌶️nonconforming/🧪️tests/🧩️example/🦀️.rs"]
mod examples_nonconforming_example;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧪️tests/🔬️compliance-report/🦀️.rs"]
mod inferences_compliance_report;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🔬️oracle/🦀️.rs"]
mod schema_oracle;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
mod sqlite_snapshot_tests;
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🪢️Mounts
