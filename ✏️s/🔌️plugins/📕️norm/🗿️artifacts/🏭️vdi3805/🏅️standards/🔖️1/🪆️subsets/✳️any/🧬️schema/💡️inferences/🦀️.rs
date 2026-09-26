//! 💡️ Vdi3805 inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧾outline/`).

use crate::Vdi3805Snapshot;
use ::framework_schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;

//#region 🔖️Inference
/// 💡️ Everything inferable from a vdi3805 snapshot. One field per named inference under
/// `💡️inferences/` (currently: `outline`, backed by the `🧾outline/` slug dir — this document's
/// own field/section structure, since a norm compliance record IS the document it describes).
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.vdi3805.inference")]
pub struct Vdi3805Inference {
    #[derived]
    pub outline: Vdi3805Outline,
}

impl protocol::Inference<Vdi3805Snapshot> for Vdi3805Inference {
    fn infer(snapshot: &Vdi3805Snapshot) -> Self {
        Self { outline: Vdi3805Outline::compute(snapshot) }
    }
}

impl protocol::InferenceSpec<Vdi3805Snapshot> for Vdi3805Inference {
    fn inference_schema_id() -> &'static str {
        "s.norm.vdi3805.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.norm.vdi3805.inference.outline", reads: &["catalog", "edition_profile", "correction_as_of", "strict_mode", "index", "geometry", "curves"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for standards::v1::subsets::any::schema::Vdi3805Builder {
    type Snapshot = Vdi3805Snapshot;
    type Inference = Vdi3805Inference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.norm.vdi3805.inference`'s facet leaves into the OS-wide inference catalog — call once at
/// plugin init, alongside `vdi3805_artifact_schema_descriptor`'s registration.
pub fn vdi3805_artifact_inference_descriptor() -> ::framework_schema::ArtifactInferenceDescriptor {
    ::framework_schema::ArtifactInferenceDescriptor {
        id: "s.norm.vdi3805.inference",
        inference: ::framework_schema::FacetLeaves {
            rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto")
        },
    }
}
//#endregion 🔖️Descriptor

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🔖️ComplianceReport
use crate::document::{CheckReport, CheckResult, CheckStatus, LocalizedCopy, Quantity, QuantityKind, Remedy, SubjectRef};
use crate::standards::v1::subsets::any::schema::{
    attributes_from_records, clause, subject_dataset, subject_product, validate_structure, valve_min_kvs_m3_h, ANNEX, RADIATOR_N_MAX, RADIATOR_N_MIN, VALVE_DN_SERIES,
};
use crate::*;
use std::collections::{BTreeMap, BTreeSet};

fn part_label(sheet: u16) -> String {
    match sheet {
        1 => "VDI 3805 Blatt 1".into(),
        n => format!("VDI 3805 Blatt {n}"),
    }
}

fn copy(en: impl Into<String>, de: impl Into<String>) -> LocalizedCopy {
    LocalizedCopy::new(en, de)
}

fn product_path(article: &str, field: &str) -> String {
    format!("catalog.products[id={article}].{field}")
}

fn curve_point_path(curve_id: &str, index: usize, axis: &str) -> String {
    format!("curves.{curve_id}.points[{index}].{axis}")
}

fn nearest_dn(dn: u16) -> u16 {
    VALVE_DN_SERIES.iter().copied().min_by_key(|c| (*c as i32 - dn as i32).unsigned_abs()).unwrap_or(50)
}

fn is_operative(status: SchemaStatus) -> bool {
    matches!(status, SchemaStatus::Published | SchemaStatus::Checked)
}

/// 📋 Mandatory Blatt attribute keys carried in native 210 (or typed attributes) per sheet + edition profile.
fn sheet_mandatory_keys(sheet: u16, profile: EditionProfileChoice) -> &'static [&'static str] {
    let current = match sheet {
        2 => &["dn", "kvs", "pressure_class", "connection_type", "authority_min", "authority_max"][..],
        3 => &["standard_output_w", "n", "length_m", "height_m", "depth_m", "connection_type"][..],
        4 => &["dn", "pressure_class", "connection_type", "outer_diameter_m", "wall_thickness_m"][..],
        5 => &["dn_suction", "dn_discharge", "nominal_flow_m3_s", "nominal_head_m", "motor_power_w", "hydraulic_efficiency"][..],
        6 => &["nominal_heat_output_w", "fuel_type", "flow_temp_max_c", "return_temp_min_c"][..],
        7 => &["dn", "pressure_class", "connection_type", "volume_m3"][..],
        8 | 10 | 14 | 18 | 33 | 36 | 37 | 40 | 42 | 100 => &["product_group", "type_code", "dn", "pressure_class"][..],
        9 | 11 | 28 | 32 | 34 | 35 | 38 | 41 | 43 | 44 | 45 | 50 | 51 | 52 | 54 | 55 | 99 => {
            &["product_group", "type_code", "dn"][..]
        }
        16 | 17 => &["product_group", "type_code", "airflow_m3_s", "pressure_drop_pa"][..],
        19 => &["product_group", "type_code", "airflow_m3_s", "filter_class"][..],
        20 | 21 | 22 | 23 | 24 | 26 | 27 | 29 => &["product_group", "type_code", "airflow_m3_s"][..],
        53 => &["product_group", "type_code", "dn", "nominal_heat_output_w", "cop"][..],
        60..=66 => &["dn", "pressure_class", "connection_type", "axial_force_n"][..],
        _ => &["product_group", "type_code"][..],
    };
    let legacy = match sheet {
        8 | 10 | 14 | 18 | 33 | 36 | 37 | 40 | 42 | 53 | 100 => &["product_group", "dn"][..],
        4 | 7 | 60..=66 => &["dn", "connection_type"][..],
        16 | 17 | 19 => &["product_group", "type_code"][..],
        _ => current,
    };
    match profile {
        EditionProfileChoice::Legacy => legacy,
        EditionProfileChoice::Current => current,
    }
}

/// 📏 Blatt-sourced numeric domains (VDI 3805 Part 1 §5 field ranges + product-sheet tables).
/// Citations: DN series DIN EN ISO 6708 / Blatt tables; airflow/pressure VDI 3805-16/19 §4; COP VDI 3805-53 §4.2.
fn sheet_numeric_bounds(sheet: u16) -> &'static [(&'static str, f64, f64)] {
    match sheet {
        // VDI 3805-4 §4.1 pipe DN / outer diameter / wall (mm→m already SI).
        4 => &[("dn", 6.0, 1200.0), ("outer_diameter_m", 0.006, 2.0), ("wall_thickness_m", 0.0005, 0.1)],
        // VDI 3805-7 §4.1 storage volume.
        7 => &[("dn", 6.0, 600.0), ("volume_m3", 0.001, 500.0)],
        8 | 10 | 14 | 18 | 33 | 36 | 37 | 40 | 42 | 100 => &[("dn", 6.0, 600.0)],
        9 | 11 | 28 | 32 | 34 | 35 | 38 | 41 | 43 | 44 | 45 | 50 | 51 | 52 | 54 | 55 | 99 => &[("dn", 6.0, 600.0)],
        // VDI 3805-16/17/19 §4 airflow + Δp.
        16 | 17 | 19 | 20 | 21 | 22 | 23 | 24 | 26 | 27 | 29 => &[("airflow_m3_s", 1e-5, 50.0), ("pressure_drop_pa", 0.0, 5000.0)],
        // VDI 3805-53 §4.2 heat-pump COP / Qn.
        53 => &[("dn", 6.0, 300.0), ("nominal_heat_output_w", 100.0, 5.0e6), ("cop", 1.0, 10.0)],
        // VDI 3805-60..66 §4.1 axial force.
        60..=66 => &[("dn", 6.0, 600.0), ("axial_force_n", 0.0, 1.0e7)],
        _ => &[],
    }
}

/// 🏷️ Connection-type code list shared by heating/sanitary/ventilation product sheets (VDI 3805-1 §6.2).
const CONNECTION_TYPE_CODES: &[&str] = &["flange", "thread", "weld", "press"];

/// 🏷️ PN pressure-class code list (DIN EN 1092 / VDI 3805 product sheets §4).
const PRESSURE_CLASS_CODES: &[&str] = &["PN6", "PN10", "PN16", "PN25", "PN40"];

/// 🏷️ Filter classes for Blatt 19 (ISO 16890 / EN 779 legacy — VDI 3805-19 Table 1).
const FILTER_CLASS_CODES: &[&str] = &["G4", "M5", "M6", "F7", "F8", "F9", "ePM1", "ePM2_5", "ePM10"];

/// 🏷️ Generic type_code tokens admitted on multi-profile sheets (VDI 3805-8/10/… §4.1).
const TYPE_CODE_PATTERN_OK: &[&str] = &["STD", "STD-A", "STD-B", "TYPE-A", "TYPE-B", "TYPE-C", "DEFAULT"];

fn code_list_for_key(sheet: u16, key: &str) -> Option<&'static [&'static str]> {
    match key {
        "connection_type" => Some(CONNECTION_TYPE_CODES),
        "pressure_class" => Some(PRESSURE_CLASS_CODES),
        "filter_class" if sheet == 19 => Some(FILTER_CLASS_CODES),
        "type_code" => Some(TYPE_CODE_PATTERN_OK),
        "fuel_type" => Some(&["gas", "oil", "electric", "biomass", "district"]),
        _ => None,
    }
}

fn record_kv(records: &[NativeRecord]) -> BTreeMap<String, String> {
    let mut kv = BTreeMap::new();
    for record in records {
        if record.family.0.as_str() == "210" {
            let mut it = record.fields.iter().skip(1);
            while let (Some(k), Some(v)) = (it.next(), it.next()) {
                if k.parse::<f64>().is_err() {
                    kv.insert(k.to_lowercase(), v.clone());
                }
            }
        } else if record.family.0.as_str() == "110" && record.fields.len() > 4 {
            let mut it = record.fields.iter().skip(4);
            while let (Some(k), Some(v)) = (it.next(), it.next()) {
                if k.parse::<f64>().is_err() {
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
    kv
}

fn profile_for_sheet(document: &Vdi3805Snapshot, sheet: u16) -> EditionProfileChoice {
    document
        .edition_profile
        .get(&sheet.to_string())
        .copied()
        .unwrap_or(EditionProfileChoice::Current)
}

fn valve_attrs_from_product(product: &CatalogueProduct) -> ValveHeatingAttributes {
    match attributes_from_records(product.sheet, &product.records) {
        SheetAttributes::ValveHeating(a) => a,
        _ => match &product.configuration.attributes {
            SheetAttributes::ValveHeating(a) => a.clone(),
            _ => ValveHeatingAttributes::from_kvs_m3_h(0, 0.0, "", "", 0.0, 0.0),
        },
    }
}

fn radiator_attrs_from_product(product: &CatalogueProduct) -> RadiatorAttributes {
    match attributes_from_records(product.sheet, &product.records) {
        SheetAttributes::Radiator(a) => a,
        _ => match &product.configuration.attributes {
            SheetAttributes::Radiator(a) => a.clone(),
            _ => RadiatorAttributes {
                standard_output_w: 0.0,
                heat_exponent_n: 0.0,
                length_m: 0.0,
                height_m: 0.0,
                depth_m: 0.0,
                connection_type: String::new(),
            },
        },
    }
}

fn pump_attrs_from_product(product: &CatalogueProduct) -> PumpHeatingAttributes {
    match attributes_from_records(product.sheet, &product.records) {
        SheetAttributes::PumpHeating(a) => a,
        _ => match &product.configuration.attributes {
            SheetAttributes::PumpHeating(a) => a.clone(),
            _ => PumpHeatingAttributes {
                dn_suction: 0,
                dn_discharge: 0,
                nominal_flow_m3_s: 0.0,
                nominal_head_m: 0.0,
                motor_power_w: 0.0,
                hydraulic_efficiency: 0.0,
                qh_curve_ref: None,
            },
        },
    }
}

fn heat_attrs_from_product(product: &CatalogueProduct) -> HeatGeneratorAttributes {
    match attributes_from_records(product.sheet, &product.records) {
        SheetAttributes::HeatGenerator(a) => a,
        _ => match &product.configuration.attributes {
            SheetAttributes::HeatGenerator(a) => a.clone(),
            _ => HeatGeneratorAttributes {
                nominal_heat_output_w: 0.0,
                fuel_type: String::new(),
                flow_temp_max_c: 0.0,
                return_temp_min_c: 0.0,
            },
        },
    }
}


fn attr_leaf_path(article: &str, leaf: &str) -> String {
    product_path(article, &format!("configuration.attributes.{leaf}"))
}

fn generic_entry_value_path(article: &str, entries: &[GenericAttribute], key: &str) -> String {
    if let Some((i, _)) = entries.iter().enumerate().find(|(_, e)| e.key.eq_ignore_ascii_case(key)) {
        attr_leaf_path(article, &format!("entries[{i}].value"))
    } else {
        product_path(article, "identity.productGroup")
    }
}


fn sync_divergence_remedy(article: &str, configured: &SheetAttributes, derived: &SheetAttributes) -> Remedy {
    match (configured, derived) {
        (SheetAttributes::ValveHeating(a), SheetAttributes::ValveHeating(b)) => {
            if a.dn != b.dn {
                return Remedy::exactly(
                    SubjectRef::new(article, attr_leaf_path(article, "dn"), copy("Nominal diameter DN", "Nennweite DN")),
                    Quantity::new(QuantityKind::Dimensionless, a.dn as f64),
                    Quantity::new(QuantityKind::Dimensionless, b.dn as f64),
                    copy(format!("Set attributes.dn to {0} from native 210 (records authoritative).", b.dn), format!("attributes.dn auf {0} aus nativem 210 setzen (Sätze maßgeblich).", b.dn)),
                );
            }
            if (a.kvs_m3_s - b.kvs_m3_s).abs() >= 1e-12 {
                return Remedy::exactly(
                    SubjectRef::new(article, attr_leaf_path(article, "kvsM3S"), copy("Flow coefficient kvs", "Durchflusskoeffizient kvs")),
                    Quantity::new(QuantityKind::Volume, a.kvs_m3_s),
                    Quantity::new(QuantityKind::Volume, b.kvs_m3_s),
                    copy("Align attributes.kvsM3S with the native 210 kvs value.", "attributes.kvsM3S an den nativen 210-kvs-Wert angleichen."),
                );
            }
            Remedy::exactly(
                SubjectRef::new(article, attr_leaf_path(article, "pressureClass"), copy("Pressure class", "Druckstufe")),
                Quantity::new(QuantityKind::Dimensionless, 0.0),
                Quantity::new(QuantityKind::Dimensionless, 1.0),
                copy("Align pressureClass with the native 210 pressure_class.", "pressureClass an natives 210-pressure_class angleichen."),
            )
        }
        (SheetAttributes::Radiator(a), SheetAttributes::Radiator(b)) => Remedy::exactly(
            SubjectRef::new(article, attr_leaf_path(article, "standardOutputW"), copy("Standard output", "Normwärmeleistung")),
            Quantity::new(QuantityKind::Power, a.standard_output_w),
            Quantity::new(QuantityKind::Power, b.standard_output_w),
            copy("Align standardOutputW with native 210 Φ.", "standardOutputW an natives 210-Φ angleichen."),
        ),
        (SheetAttributes::PumpHeating(a), SheetAttributes::PumpHeating(b)) => Remedy::exactly(
            SubjectRef::new(article, attr_leaf_path(article, "nominalFlowM3S"), copy("Nominal flow", "Nennvolumenstrom")),
            Quantity::new(QuantityKind::Volume, a.nominal_flow_m3_s),
            Quantity::new(QuantityKind::Volume, b.nominal_flow_m3_s),
            copy("Align nominalFlowM3S with native 210 Q.", "nominalFlowM3S an natives 210-Q angleichen."),
        ),
        (SheetAttributes::HeatGenerator(a), SheetAttributes::HeatGenerator(b)) => Remedy::exactly(
            SubjectRef::new(article, attr_leaf_path(article, "nominalHeatOutputW"), copy("Nominal heat output", "Nennwärmeleistung")),
            Quantity::new(QuantityKind::Power, a.nominal_heat_output_w),
            Quantity::new(QuantityKind::Power, b.nominal_heat_output_w),
            copy("Align nominalHeatOutputW with native 210 Qn.", "nominalHeatOutputW an natives 210-Qn angleichen."),
        ),
        (SheetAttributes::Generic(g), SheetAttributes::Generic(d)) => {
            let key = d.entries.first().map(|e| e.key.as_str()).unwrap_or("product_group");
            let req = d.get(key).unwrap_or("").parse::<f64>().unwrap_or(1.0);
            let cur = g.get(key).unwrap_or("").parse::<f64>().unwrap_or(0.0);
            Remedy::exactly(
                SubjectRef::new(article, generic_entry_value_path(article, &g.entries, key), copy("Generic attribute", "Generisches Attribut")),
                Quantity::new(QuantityKind::Dimensionless, cur),
                Quantity::new(QuantityKind::Dimensionless, req),
                copy(format!("Align generic attribute `{key}` with native 210."), format!("Generisches Attribut `{key}` an natives 210 angleichen.")),
            )
        }
        (_, SheetAttributes::ValveHeating(b)) => Remedy::exactly(
            SubjectRef::new(article, attr_leaf_path(article, "dn"), copy("Nominal diameter DN", "Nennweite DN")),
            Quantity::new(QuantityKind::Dimensionless, 0.0),
            Quantity::new(QuantityKind::Dimensionless, b.dn as f64),
            copy(
                "configuration.attributes must be valveHeating matching native 210; set dn from records then re-sync.",
                "configuration.attributes muss valveHeating gemäß 210 sein; dn aus Sätzen setzen und neu synchronisieren.",
            ),
        ),
        (_, SheetAttributes::Radiator(b)) => Remedy::exactly(
            SubjectRef::new(article, attr_leaf_path(article, "standardOutputW"), copy("Standard output", "Normwärmeleistung")),
            Quantity::new(QuantityKind::Power, 0.0),
            Quantity::new(QuantityKind::Power, b.standard_output_w.max(500.0)),
            copy("configuration.attributes must be radiator matching native 210 Φ.", "configuration.attributes muss radiator gemäß 210-Φ sein."),
        ),
        (_, SheetAttributes::PumpHeating(b)) => Remedy::exactly(
            SubjectRef::new(article, attr_leaf_path(article, "nominalFlowM3S"), copy("Nominal flow", "Nennvolumenstrom")),
            Quantity::new(QuantityKind::Volume, 0.0),
            Quantity::new(QuantityKind::Volume, b.nominal_flow_m3_s.max(0.0025)),
            copy("configuration.attributes must be pumpHeating matching native 210 Q.", "configuration.attributes muss pumpHeating gemäß 210-Q sein."),
        ),
        (_, SheetAttributes::HeatGenerator(b)) => Remedy::exactly(
            SubjectRef::new(article, attr_leaf_path(article, "nominalHeatOutputW"), copy("Nominal heat output", "Nennwärmeleistung")),
            Quantity::new(QuantityKind::Power, 0.0),
            Quantity::new(QuantityKind::Power, b.nominal_heat_output_w.max(10_000.0)),
            copy("configuration.attributes must be heatGenerator matching native 210 Qn.", "configuration.attributes muss heatGenerator gemäß 210-Qn sein."),
        ),
        _ => Remedy::exactly(
            SubjectRef::new(article, product_path(article, "identity.productGroup"), copy("Product group", "Produktgruppe")),
            Quantity::new(QuantityKind::Dimensionless, 0.0),
            Quantity::new(QuantityKind::Dimensionless, 1.0),
            copy("Align typed attributes with native 210 records.", "Typisierte Attribute an native 210-Sätze angleichen."),
        ),
    }
}

fn check_attributes_records_sync(product: &CatalogueProduct) -> Vec<CheckResult> {
    let article = product.id.as_str();
    let derived = attributes_from_records(product.sheet, &product.records);
    let configured = &product.configuration.attributes;
    let ok = match (configured, &derived) {
        (SheetAttributes::ValveHeating(a), SheetAttributes::ValveHeating(b)) => {
            a.dn == b.dn
                && (a.kvs_m3_s - b.kvs_m3_s).abs() < 1e-12
                && a.pressure_class == b.pressure_class
                && a.connection_type == b.connection_type
                && (a.authority_min - b.authority_min).abs() < 1e-12
                && (a.authority_max - b.authority_max).abs() < 1e-12
        }
        (SheetAttributes::Radiator(a), SheetAttributes::Radiator(b)) => {
            (a.standard_output_w - b.standard_output_w).abs() < 1e-9
                && (a.heat_exponent_n - b.heat_exponent_n).abs() < 1e-9
                && (a.length_m - b.length_m).abs() < 1e-9
                && (a.height_m - b.height_m).abs() < 1e-9
                && (a.depth_m - b.depth_m).abs() < 1e-9
        }
        (SheetAttributes::PumpHeating(a), SheetAttributes::PumpHeating(b)) => {
            a.dn_suction == b.dn_suction
                && a.dn_discharge == b.dn_discharge
                && (a.nominal_flow_m3_s - b.nominal_flow_m3_s).abs() < 1e-12
                && (a.nominal_head_m - b.nominal_head_m).abs() < 1e-9
                && (a.motor_power_w - b.motor_power_w).abs() < 1e-6
                && (a.hydraulic_efficiency - b.hydraulic_efficiency).abs() < 1e-12
        }
        (SheetAttributes::HeatGenerator(a), SheetAttributes::HeatGenerator(b)) => {
            (a.nominal_heat_output_w - b.nominal_heat_output_w).abs() < 1e-9
                && a.fuel_type == b.fuel_type
                && (a.flow_temp_max_c - b.flow_temp_max_c).abs() < 1e-9
                && (a.return_temp_min_c - b.return_temp_min_c).abs() < 1e-9
        }
        (SheetAttributes::Generic(a), SheetAttributes::Generic(b)) => {
            let mut ak: Vec<_> = a.entries.iter().map(|e| (e.key.to_lowercase(), e.value.clone())).collect();
            let mut bk: Vec<_> = b.entries.iter().map(|e| (e.key.to_lowercase(), e.value.clone())).collect();
            ak.sort();
            bk.sort();
            ak == bk
        }
        _ => false,
    };
    let typed_sheet = matches!(product.sheet.0, 2 | 3 | 5 | 6);
    let configured_typed = matches!(
        configured,
        SheetAttributes::ValveHeating(_) | SheetAttributes::Radiator(_) | SheetAttributes::PumpHeating(_) | SheetAttributes::HeatGenerator(_)
    );
    let derived_typed = matches!(
        derived,
        SheetAttributes::ValveHeating(_) | SheetAttributes::Radiator(_) | SheetAttributes::PumpHeating(_) | SheetAttributes::HeatGenerator(_)
    );
    let mut out = Vec::new();
    if typed_sheet || configured_typed || derived_typed || matches!(configured, SheetAttributes::Generic(_)) {
        let mut b = CheckResult::assess(
            format!("vdi3805.{}.sync.{}", product.sheet.0, article),
            part_label(product.sheet.0),
            clause(&product.sheet.0.to_string(), "4.0"),
            subject_product(product),
            copy("Attributes ↔ records sync", "Attribute ↔ Datensätze Synchronität"),
        )
        .annex(ANNEX);
        if ok && !(typed_sheet && !configured_typed && derived_typed) {
            b = b
                .status(CheckStatus::Pass)
                .explanation(copy(
                    "Typed attributes match native 210 records.",
                    "Typisierte Attribute stimmen mit nativen 210-Sätzen überein.",
                ));
        } else {
            b = b
                .status(CheckStatus::Fail)
                .explanation(copy(
                    "configuration.attributes diverge from native records — align the typed attribute leaf; sync regenerates the 210 record.",
                    "configuration.attributes weichen von nativen Sätzen ab — typisiertes Attributblatt angleichen; Sync erzeugt den 210-Satz neu.",
                ))
                .remedy(sync_divergence_remedy(article, configured, &derived));
        }
        out.push(b.build());
    }
    out
}

fn check_part1(document: &Vdi3805Snapshot) -> Vec<CheckResult> {
    let mut out = Vec::new();
    let issues = validate_structure(document);
    let errors: Vec<_> = issues.iter().filter(|d| d.severity == Severity::Error).collect();
    let subject = subject_dataset();
    if errors.is_empty() {
        out.push(
            CheckResult::assess("vdi3805.1.structure", part_label(1), clause("1", "4.1"), subject.clone(), copy("Part 1 record structure", "Teil 1 Datensatzstruktur"))
                .status(CheckStatus::Pass)
                .annex(ANNEX)
                .explanation(copy("Header, mandatory records, field arities and referential integrity are valid.", "Kopf, Pflichtsätze, Feldanzahlen und Referenzintegrität sind gültig."))
                .build(),
        );
    } else {
        for err in errors.iter() {
            let path = err.field.clone();
            let required_count = if path.contains("recordCount") {
                document.catalog.products.iter().map(|p| p.records.len() as f64).sum::<f64>()
            } else {
                1.0
            };
            let current = if path.contains("recordCount") {
                document.catalog.file.record_count as f64
            } else {
                0.0
            };
            let target = SubjectRef::new("", path.clone(), copy(format!("Structural field `{path}`"), format!("Strukturfeld `{path}`")));
            let slug = path.replace('.', "-").replace('/', "-").replace('[', "-").replace(']', "");
            let mut builder = CheckResult::assess(
                format!("vdi3805.1.structure.{slug}"),
                part_label(1),
                clause("1", "4.1"),
                subject.clone(),
                copy("Part 1 structural rule", "Teil-1-Strukturregel"),
            )
            .status(CheckStatus::Fail)
            .annex(ANNEX)
            .explanation(copy(format!("Part 1 structural error at {}: {}", err.field, err.message), format!("Teil-1-Strukturfehler bei {}: {}", err.field, err.message)));
            if path.contains("recordCount") {
                builder = builder.remedy(Remedy::exactly(
                    target,
                    Quantity::new(QuantityKind::Dimensionless, current),
                    Quantity::new(QuantityKind::Dimensionless, required_count),
                    copy(
                        format!("Set manufacturerFile.recordCount from {current} to {required_count}."),
                        format!("manufacturerFile.recordCount von {current} auf {required_count} setzen."),
                    ),
                ));
            } else if path.contains("geometryRef") {
                let article = document.catalog.products.first().map(|p| p.id.as_str()).unwrap_or("");
                let geom_target = SubjectRef::new(article, product_path(article, "configuration.geometryRef"), copy("Geometry reference", "Geometriereferenz"));
                builder = builder.remedy(Remedy::one_of(
                    geom_target,
                    document.geometry.keys().cloned().collect(),
                    copy(
                        "Point geometryRef at an existing geometry id or add the missing geometry block.",
                        "geometryRef auf eine vorhandene Geometrie-ID setzen oder den fehlenden Geometrieblock ergänzen.",
                    ),
                ));
            } else if path.contains("functionRefs") {
                let article = document.catalog.products.first().map(|p| p.id.as_str()).unwrap_or("");
                builder = builder.remedy(Remedy::one_of(
                    SubjectRef::new(article, product_path(article, "configuration.functionRefs"), copy("Curve references", "Kennlinienreferenzen")),
                    document.curves.keys().cloned().collect(),
                    copy("Replace dangling curve refs with existing curve ids.", "Hängende Kennlinienreferenzen durch vorhandene IDs ersetzen."),
                ));
            } else {
                builder = builder.remedy(Remedy::exactly(
                    target,
                    Quantity::new(QuantityKind::Dimensionless, 0.0),
                    Quantity::new(QuantityKind::Dimensionless, 1.0),
                    copy(format!("Fix Part 1 field `{}`: {}.", err.field, err.message), format!("Teil-1-Feld `{}` korrigieren: {}.", err.field, err.message)),
                ));
            }
            out.push(builder.build());
        }
    }
    out
}

fn mono_increasing_remedy(curve_id: &str, curve: &CharacteristicCurve) -> Option<(usize, f64, f64)> {
    for (i, w) in curve.points.windows(2).enumerate() {
        if w[1].y + 1e-12 < w[0].y {
            return Some((i + 1, w[1].y, w[0].y));
        }
        if w[1].x + 1e-12 < w[0].x {
            return Some((i + 1, w[1].x, w[0].x));
        }
    }
    None
}

fn mono_decreasing_y_remedy(curve_id: &str, curve: &CharacteristicCurve) -> Option<(usize, f64, f64)> {
    let _ = curve_id;
    for (i, w) in curve.points.windows(2).enumerate() {
        if w[1].y > w[0].y + 1e-12 {
            return Some((i + 1, w[1].y, w[0].y));
        }
    }
    None
}

fn check_valve(document: &Vdi3805Snapshot, product: &CatalogueProduct, attrs: &ValveHeatingAttributes) -> Vec<CheckResult> {
    let mut out = Vec::new();
    let article = product.id.as_str();
    let subject = subject_product(product);
    let part = part_label(2);

    let dn_ok = VALVE_DN_SERIES.contains(&attrs.dn);
    let nearest = nearest_dn(attrs.dn);
    let mut dn_builder = CheckResult::assess(format!("vdi3805.2.dn.{article}"), part.clone(), clause("2", "4.2.1"), subject.clone(), copy("Nominal diameter DN series", "Nennweite DN-Reihe"))
        .annex(ANNEX)
        .utilization(Quantity::new(QuantityKind::Dimensionless, if dn_ok { 1.0 } else { 2.0 }), Quantity::new(QuantityKind::Dimensionless, 1.0));
    if dn_ok {
        dn_builder = dn_builder.explanation(copy(format!("DN {} is in the admitted series.", attrs.dn), format!("DN {} liegt in der zugelassenen Reihe.", attrs.dn)));
    } else {
        dn_builder = dn_builder
            .explanation(copy(format!("DN {} is not in the VDI 3805 Blatt 2 series.", attrs.dn), format!("DN {} ist nicht in der VDI-3805-Blatt-2-Reihe.", attrs.dn)))
            .remedy(Remedy::exactly(
                SubjectRef::new(article, product_path(article, "configuration.attributes.dn"), copy("DN", "DN")),
                Quantity::new(QuantityKind::Dimensionless, attrs.dn as f64),
                Quantity::new(QuantityKind::Dimensionless, nearest as f64),
                copy(format!("Change DN from {} to {}.", attrs.dn, nearest), format!("DN von {} auf {} ändern.", attrs.dn, nearest)),
            ));
    }
    out.push(dn_builder.build());

    let min_kvs = valve_min_kvs_m3_h(if dn_ok { attrs.dn } else { nearest });
    let kvs_h = attrs.kvs_m3_h();
    let mut kvs_builder = CheckResult::assess(format!("vdi3805.2.kvs.{article}"), part.clone(), clause("2", "4.2.2"), subject.clone(), copy("Flow coefficient kvs", "Durchflusskoeffizient kvs"))
        .annex(ANNEX)
        .minimum(Quantity::new(QuantityKind::Volume, attrs.kvs_m3_s), Quantity::new(QuantityKind::Volume, min_kvs / 3600.0));
    if kvs_h + 1e-12 >= min_kvs {
        kvs_builder = kvs_builder.explanation(copy(format!("kvs = {kvs_h:.3} m³/h ≥ {min_kvs:.3} m³/h for DN {}.", attrs.dn), format!("kvs = {kvs_h:.3} m³/h ≥ {min_kvs:.3} m³/h für DN {}.", attrs.dn)));
    } else {
        kvs_builder = kvs_builder
            .explanation(copy(format!("kvs = {kvs_h:.3} m³/h below Blatt 2 minimum {min_kvs:.3} m³/h.",), format!("kvs = {kvs_h:.3} m³/h unter Blatt-2-Minimum {min_kvs:.3} m³/h.")))
            .remedy(Remedy::at_least(
                SubjectRef::new(article, product_path(article, "configuration.attributes.kvsM3S"), copy("kvs", "kvs")),
                Quantity::new(QuantityKind::Volume, attrs.kvs_m3_s),
                Quantity::new(QuantityKind::Volume, min_kvs / 3600.0),
                copy(format!("Increase kvs from {kvs_h:.3} m³/h to at least {min_kvs:.3} m³/h."), format!("kvs von {kvs_h:.3} m³/h auf mindestens {min_kvs:.3} m³/h erhöhen.")),
            ));
    }
    out.push(kvs_builder.build());

    let pc_ok = !attrs.pressure_class.is_empty();
    let mut pc = CheckResult::assess(format!("vdi3805.2.pn.{article}"), part.clone(), clause("2", "4.2.3"), subject.clone(), copy("Pressure class", "Druckstufe"))
        .annex(ANNEX)
        .status(if pc_ok { CheckStatus::Pass } else { CheckStatus::Fail });
    if pc_ok {
        pc = pc.explanation(copy(format!("Pressure class {} present.", attrs.pressure_class), format!("Druckstufe {} vorhanden.", attrs.pressure_class)));
    } else {
        pc = pc
            .explanation(copy("Pressure class is empty.", "Druckstufe fehlt."))
            .remedy(Remedy::one_of(
                SubjectRef::new(article, product_path(article, "configuration.attributes.pressureClass"), copy("Pressure class", "Druckstufe")),
                vec!["PN6".into(), "PN10".into(), "PN16".into(), "PN25".into()],
                copy("Set pressure class to a PN series value (e.g. PN16).", "Druckstufe auf einen PN-Wert setzen (z. B. PN16)."),
            ));
    }
    out.push(pc.build());

    let ct = attrs.connection_type.as_str();
    let ct_ok = CONNECTION_TYPE_CODES.iter().any(|c| *c == ct);
    let mut ct_check = CheckResult::assess(format!("vdi3805.2.connectionType.{article}"), part.clone(), clause("2", "4.2.5"), subject.clone(), copy("Connection type", "Anschlussart"))
        .annex(ANNEX)
        .status(if ct_ok { CheckStatus::Pass } else { CheckStatus::Fail });
    if ct_ok {
        ct_check = ct_check.explanation(copy(format!("connectionType `{ct}` is in the Blatt 2 code list."), format!("connectionType `{ct}` ist in der Blatt-2-Codeliste.")));
    } else {
        ct_check = ct_check
            .explanation(copy(
                if ct.is_empty() { "Mandatory connectionType omitted from record 210.".into() } else { format!("connectionType `{ct}` is not in the Blatt 2 code list.") },
                if ct.is_empty() { "Pflichtfeld connectionType fehlt im Satz 210.".into() } else { format!("connectionType `{ct}` ist nicht in der Blatt-2-Codeliste.") },
            ))
            .remedy(Remedy::one_of(
                SubjectRef::new(article, product_path(article, "configuration.attributes.connectionType"), copy("Connection type", "Anschlussart")),
                CONNECTION_TYPE_CODES.iter().map(|s| (*s).to_string()).collect(),
                copy("Set connectionType to a Blatt code-list value (e.g. flange).", "connectionType auf einen Blatt-Codelistenwert setzen (z. B. flange)."),
            ));
    }
    out.push(ct_check.build());

    // PN domain (not just presence) — VDI 3805-2 §4.2.3 / DIN EN 1092.
    if !attrs.pressure_class.is_empty() && !PRESSURE_CLASS_CODES.iter().any(|c| *c == attrs.pressure_class.as_str()) {
        out.push(
            CheckResult::assess(format!("vdi3805.2.pn.domain.{article}"), part.clone(), clause("2", "4.2.3"), subject.clone(), copy("Pressure class domain", "Druckstufen-Wertebereich"))
                .annex(ANNEX)
                .status(CheckStatus::Fail)
                .explanation(copy(format!("pressureClass `{}` is not an admitted PN code.", attrs.pressure_class), format!("pressureClass `{}` ist kein zugelassener PN-Code.", attrs.pressure_class)))
                .remedy(Remedy::one_of(
                    SubjectRef::new(article, product_path(article, "configuration.attributes.pressureClass"), copy("Pressure class", "Druckstufe")),
                    PRESSURE_CLASS_CODES.iter().map(|s| (*s).to_string()).collect(),
                    copy("Choose an admitted PN class (PN6…PN40).", "Zugelassene PN-Klasse wählen (PN6…PN40)."),
                ))
                .build(),
        );
    }

    let auth_ok = attrs.authority_min < attrs.authority_max && attrs.authority_min >= 0.0 && attrs.authority_max <= 1.0;
    let mut auth = CheckResult::assess(format!("vdi3805.2.authority.{article}"), part.clone(), clause("2", "4.2.4"), subject.clone(), copy("Valve authority range", "Ventilautoritätsbereich"))
        .annex(ANNEX)
        .status(if auth_ok { CheckStatus::Pass } else { CheckStatus::Fail });
    if auth_ok {
        auth = auth.explanation(copy(format!("Authority [{}, {}] is valid.", attrs.authority_min, attrs.authority_max), format!("Autorität [{}, {}] ist gültig.", attrs.authority_min, attrs.authority_max)));
    } else {
        auth = auth
            .explanation(copy(format!("Authority [{}, {}] is invalid.", attrs.authority_min, attrs.authority_max), format!("Autorität [{}, {}] ist ungültig.", attrs.authority_min, attrs.authority_max)))
            .remedy(Remedy::exactly(
                SubjectRef::new(article, product_path(article, "configuration.attributes.authorityMax"), copy("Authority max", "Autorität max")),
                Quantity::new(QuantityKind::Dimensionless, attrs.authority_max),
                Quantity::new(QuantityKind::Dimensionless, (attrs.authority_min + 0.2).min(1.0).max(attrs.authority_min + 0.05)),
                copy("Set authorityMax above authorityMin within [0, 1].", "authorityMax oberhalb von authorityMin im Intervall [0, 1] setzen."),
            ));
    }
    out.push(auth.build());

    for curve_id in &product.configuration.function_refs {
        if let Some(curve) = document.curves.get(curve_id) {
            let mono = curve.points.windows(2).all(|w| w[1].x + 1e-12 >= w[0].x && w[1].y + 1e-12 >= w[0].y);
            let mut c = CheckResult::assess(format!("vdi3805.2.curve.{article}.{curve_id}"), part.clone(), clause("2", "5.1"), subject.clone(), copy("Characteristic curve monotonicity", "Kennlinien-Monotonie"))
                .annex(ANNEX)
                .status(if mono { CheckStatus::Pass } else { CheckStatus::Fail });
            if mono {
                c = c.explanation(copy(format!("Curve {curve_id} is non-decreasing.",), format!("Kennlinie {curve_id} ist monoton steigend.")));
            } else if let Some((idx, current_y, required_y)) = mono_increasing_remedy(curve_id, curve) {
                let axis = if curve.points.get(idx).is_some_and(|p| curve.points.get(idx - 1).is_some_and(|prev| p.x + 1e-12 < prev.x)) {
                    "x"
                } else {
                    "y"
                };
                let (cur, req) = if axis == "x" {
                    let prev = curve.points[idx - 1].x;
                    (curve.points[idx].x, prev)
                } else {
                    (current_y, required_y)
                };
                c = c
                    .explanation(copy(format!("Curve {curve_id} is not monotonic.",), format!("Kennlinie {curve_id} ist nicht monoton.")))
                    .remedy(Remedy::exactly(
                        SubjectRef::new(curve_id, curve_point_path(curve_id, idx, axis), copy("Curve point", "Kennlinienpunkt")),
                        Quantity::new(QuantityKind::Dimensionless, cur),
                        Quantity::new(QuantityKind::Dimensionless, req),
                        copy(
                            format!("Raise points[{idx}].{axis} so the curve is non-decreasing."),
                            format!("points[{idx}].{axis} anheben, damit die Kennlinie monoton steigt."),
                        ),
                    ));
            }
            out.push(c.build());
        }
    }
    out
}

fn check_radiator(product: &CatalogueProduct, attrs: &RadiatorAttributes) -> Vec<CheckResult> {
    let mut out = Vec::new();
    let article = product.id.as_str();
    let subject = subject_product(product);
    let part = part_label(3);
    let mut phi = CheckResult::assess(format!("vdi3805.3.phi.{article}"), part.clone(), clause("3", "4.1"), subject.clone(), copy("Standard output at 75/65/20 °C", "Normwärmeleistung bei 75/65/20 °C"))
        .annex(ANNEX)
        .minimum(Quantity::new(QuantityKind::Power, attrs.standard_output_w), Quantity::new(QuantityKind::Power, 1.0));
    if attrs.standard_output_w >= 1.0 {
        phi = phi.explanation(copy(format!("Φ = {:.0} W at 75/65/20 °C.", attrs.standard_output_w), format!("Φ = {:.0} W bei 75/65/20 °C.", attrs.standard_output_w)));
    } else {
        phi = phi
            .explanation(copy("Standard output missing or non-positive.", "Normwärmeleistung fehlt oder ist nicht positiv."))
            .remedy(Remedy::at_least(
                SubjectRef::new(article, product_path(article, "configuration.attributes.standardOutputW"), copy("Standard output", "Normwärmeleistung")),
                Quantity::new(QuantityKind::Power, attrs.standard_output_w),
                Quantity::new(QuantityKind::Power, 500.0),
                copy("Set standardOutputW to the EN 442 / Blatt 3 rated output in watts.", "standardOutputW auf die EN-442-/Blatt-3-Nennleistung in Watt setzen."),
            ));
    }
    out.push(phi.build());

    let n_ok = attrs.heat_exponent_n >= RADIATOR_N_MIN && attrs.heat_exponent_n <= RADIATOR_N_MAX;
    let mut n = CheckResult::assess(format!("vdi3805.3.n.{article}"), part.clone(), clause("3", "4.2"), subject.clone(), copy("Heat exponent n", "Heizexponent n"))
        .annex(ANNEX)
        .status(if n_ok { CheckStatus::Pass } else { CheckStatus::Fail });
    if n_ok {
        n = n.explanation(copy(format!("n = {:.3} within [{RADIATOR_N_MIN}, {RADIATOR_N_MAX}].", attrs.heat_exponent_n), format!("n = {:.3} im Bereich [{RADIATOR_N_MIN}, {RADIATOR_N_MAX}].", attrs.heat_exponent_n)));
    } else {
        let required = attrs.heat_exponent_n.clamp(RADIATOR_N_MIN, RADIATOR_N_MAX);
        n = n
            .explanation(copy(format!("n = {:.3} outside Blatt 3 range.", attrs.heat_exponent_n), format!("n = {:.3} außerhalb des Blatt-3-Bereichs.", attrs.heat_exponent_n)))
            .remedy(Remedy::exactly(
                SubjectRef::new(article, product_path(article, "configuration.attributes.heatExponentN"), copy("Heat exponent", "Heizexponent")),
                Quantity::new(QuantityKind::Dimensionless, attrs.heat_exponent_n),
                Quantity::new(QuantityKind::Dimensionless, if attrs.heat_exponent_n == 0.0 { 1.3 } else { required }),
                copy(format!("Set heatExponentN into [{RADIATOR_N_MIN}, {RADIATOR_N_MAX}] (typical 1.3)."), format!("heatExponentN in [{RADIATOR_N_MIN}, {RADIATOR_N_MAX}] setzen (typisch 1,3).")),
            ));
    }
    out.push(n.build());

    for (leaf, en, de, value, min_v) in [
        ("lengthM", "Radiator length", "Heizkörperlänge", attrs.length_m, 0.1),
        ("heightM", "Radiator height", "Heizkörperhöhe", attrs.height_m, 0.1),
        ("depthM", "Radiator depth", "Heizkörpertiefe", attrs.depth_m, 0.02),
    ] {
        let mut dim = CheckResult::assess(
            format!("vdi3805.3.{leaf}.{article}"),
            part_label(3),
            clause("3", "4.3"),
            subject.clone(),
            copy(en, de),
        )
        .annex(ANNEX)
        .minimum(Quantity::new(QuantityKind::Length, value), Quantity::new(QuantityKind::Length, min_v));
        if value + 1e-12 >= min_v {
            dim = dim.explanation(copy(format!("{en} = {value:.3} m."), format!("{de} = {value:.3} m.")));
        } else {
            dim = dim
                .explanation(copy(format!("{en} missing or below {min_v} m."), format!("{de} fehlt oder unter {min_v} m.")))
                .remedy(Remedy::at_least(
                    SubjectRef::new(article, attr_leaf_path(article, leaf), copy(en, de)),
                    Quantity::new(QuantityKind::Length, value),
                    Quantity::new(QuantityKind::Length, min_v),
                    copy(format!("Set {leaf} to at least {min_v} m."), format!("{leaf} auf mindestens {min_v} m setzen.")),
                ));
        }
        out.push(dim.build());
    }
    let ct = attrs.connection_type.as_str();
    let ct_ok = CONNECTION_TYPE_CODES.iter().any(|c| *c == ct);
    let mut ct_check = CheckResult::assess(format!("vdi3805.3.connectionType.{article}"), part_label(3), clause("3", "4.4"), subject.clone(), copy("Connection type", "Anschlussart"))
        .annex(ANNEX)
        .status(if ct_ok { CheckStatus::Pass } else { CheckStatus::Fail });
    if ct_ok {
        ct_check = ct_check.explanation(copy(format!("connectionType `{ct}` admitted."), format!("connectionType `{ct}` zugelassen.")));
    } else {
        ct_check = ct_check
            .explanation(copy("connectionType missing or not in Blatt code list.", "connectionType fehlt oder nicht in der Blatt-Codeliste."))
            .remedy(Remedy::one_of(
                SubjectRef::new(article, product_path(article, "configuration.attributes.connectionType"), copy("Connection type", "Anschlussart")),
                CONNECTION_TYPE_CODES.iter().map(|s| (*s).to_string()).collect(),
                copy("Set connectionType to a Blatt code-list value.", "connectionType auf einen Blatt-Codelistenwert setzen."),
            ));
    }
    out.push(ct_check.build());
    out
}

fn check_pump(document: &Vdi3805Snapshot, product: &CatalogueProduct, attrs: &PumpHeatingAttributes) -> Vec<CheckResult> {
    let mut out = Vec::new();
    let article = product.id.as_str();
    let subject = subject_product(product);
    let part = part_label(5);
    let mut flow = CheckResult::assess(format!("vdi3805.5.q.{article}"), part.clone(), clause("5", "4.1"), subject.clone(), copy("Nominal volume flow", "Nennvolumenstrom"))
        .annex(ANNEX)
        .minimum(Quantity::new(QuantityKind::Volume, attrs.nominal_flow_m3_s), Quantity::new(QuantityKind::Volume, 1e-6));
    if attrs.nominal_flow_m3_s >= 1e-6 {
        flow = flow.explanation(copy(format!("Q = {:.4} m³/s.", attrs.nominal_flow_m3_s), format!("Q = {:.4} m³/s.", attrs.nominal_flow_m3_s)));
    } else {
        flow = flow
            .explanation(copy("Nominal flow missing.", "Nennvolumenstrom fehlt."))
            .remedy(Remedy::at_least(
                SubjectRef::new(article, product_path(article, "configuration.attributes.nominalFlowM3S"), copy("Nominal flow", "Nennvolumenstrom")),
                Quantity::new(QuantityKind::Volume, attrs.nominal_flow_m3_s),
                Quantity::new(QuantityKind::Volume, 0.0025),
                copy("Set nominalFlowM3S to the rated pump flow in m³/s.", "nominalFlowM3S auf den Nennvolumenstrom in m³/s setzen."),
            ));
    }
    out.push(flow.build());

    let eta_ok = attrs.hydraulic_efficiency > 0.0 && attrs.hydraulic_efficiency <= 1.0;
    let mut eta = CheckResult::assess(format!("vdi3805.5.eta.{article}"), part.clone(), clause("5", "4.2"), subject.clone(), copy("Hydraulic efficiency", "Hydraulischer Wirkungsgrad"))
        .annex(ANNEX)
        .status(if eta_ok { CheckStatus::Pass } else { CheckStatus::Fail });
    if eta_ok {
        eta = eta.explanation(copy(format!("η = {:.3}.", attrs.hydraulic_efficiency), format!("η = {:.3}.", attrs.hydraulic_efficiency)));
    } else {
        eta = eta
            .explanation(copy("Hydraulic efficiency must be in (0, 1].", "Hydraulischer Wirkungsgrad muss in (0, 1] liegen."))
            .remedy(Remedy::exactly(
                SubjectRef::new(article, product_path(article, "configuration.attributes.hydraulicEfficiency"), copy("Efficiency", "Wirkungsgrad")),
                Quantity::new(QuantityKind::Dimensionless, attrs.hydraulic_efficiency),
                Quantity::new(QuantityKind::Dimensionless, 0.45),
                copy("Set hydraulicEfficiency to a value in (0, 1], e.g. 0.45.", "hydraulicEfficiency auf einen Wert in (0, 1] setzen, z. B. 0,45."),
            ));
    }
    out.push(eta.build());

    for (leaf, en, de, value, min_v, kind) in [
        ("dnSuction", "Suction DN", "Saugseitige DN", attrs.dn_suction as f64, 10.0, QuantityKind::Dimensionless),
        ("dnDischarge", "Discharge DN", "Druckseitige DN", attrs.dn_discharge as f64, 10.0, QuantityKind::Dimensionless),
        ("nominalHeadM", "Nominal head", "Nennförderhöhe", attrs.nominal_head_m, 0.5, QuantityKind::Length),
        ("motorPowerW", "Motor power", "Motorleistung", attrs.motor_power_w, 10.0, QuantityKind::Power),
    ] {
        let mut c = CheckResult::assess(
            format!("vdi3805.5.{leaf}.{article}"),
            part.clone(),
            clause("5", "4.3"),
            subject.clone(),
            copy(en, de),
        )
        .annex(ANNEX)
        .minimum(Quantity::new(kind, value), Quantity::new(kind, min_v));
        if value + 1e-12 >= min_v {
            c = c.explanation(copy(format!("{en} = {value}."), format!("{de} = {value}.")));
        } else {
            c = c
                .explanation(copy(format!("{en} must be ≥ {min_v}."), format!("{de} muss ≥ {min_v} sein.")))
                .remedy(Remedy::at_least(
                    SubjectRef::new(article, attr_leaf_path(article, leaf), copy(en, de)),
                    Quantity::new(kind, value),
                    Quantity::new(kind, min_v),
                    copy(format!("Set {leaf} to at least {min_v}."), format!("{leaf} auf mindestens {min_v} setzen.")),
                ));
        }
        out.push(c.build());
    }

    if let Some(curve_id) = attrs.qh_curve_ref.as_ref().or(product.configuration.function_refs.first()) {
        if let Some(curve) = document.curves.get(curve_id) {
            let mono = curve.points.windows(2).all(|w| w[1].x + 1e-12 >= w[0].x && w[1].y <= w[0].y + 1e-12);
            let mut c = CheckResult::assess(format!("vdi3805.5.qh.{article}"), part, clause("5", "5.1"), subject, copy("Q-H curve shape", "Q-H-Kennlinie"))
                .annex(ANNEX)
                .status(if mono { CheckStatus::Pass } else { CheckStatus::Fail });
            if mono {
                c = c.explanation(copy("Q-H head is non-increasing with flow.", "Förderhöhe fällt mit dem Volumenstrom nicht an."));
            } else if let Some((idx, current_y, required_y)) = mono_decreasing_y_remedy(curve_id, curve) {
                c = c
                    .explanation(copy("Q-H curve must not rise with flow.", "Q-H-Kennlinie darf mit dem Volumenstrom nicht steigen."))
                    .remedy(Remedy::exactly(
                        SubjectRef::new(curve_id, curve_point_path(curve_id, idx, "y"), copy("Q-H point y", "Q-H-Punkt y")),
                        Quantity::new(QuantityKind::Dimensionless, current_y),
                        Quantity::new(QuantityKind::Dimensionless, required_y),
                        copy("Lower the rising head point so Q-H is non-increasing.", "Ansteigenden Förderhöhenpunkt absenken, damit Q-H nicht steigt."),
                    ));
            }
            out.push(c.build());
        }
    }
    out
}

fn check_heat_generator(product: &CatalogueProduct, attrs: &HeatGeneratorAttributes) -> Vec<CheckResult> {
    let mut out = Vec::new();
    let article = product.id.as_str();
    let subject = subject_product(product);
    let part = part_label(6);
    let mut qn = CheckResult::assess(format!("vdi3805.6.qn.{article}"), part.clone(), clause("6", "4.1"), subject.clone(), copy("Nominal heat output", "Nennwärmeleistung"))
        .annex(ANNEX)
        .minimum(Quantity::new(QuantityKind::Power, attrs.nominal_heat_output_w), Quantity::new(QuantityKind::Power, 1.0));
    if attrs.nominal_heat_output_w >= 1.0 {
        qn = qn.explanation(copy(format!("Qn = {:.0} W.", attrs.nominal_heat_output_w), format!("Qn = {:.0} W.", attrs.nominal_heat_output_w)));
    } else {
        qn = qn
            .explanation(copy("Nominal heat output missing.", "Nennwärmeleistung fehlt."))
            .remedy(Remedy::at_least(
                SubjectRef::new(article, product_path(article, "configuration.attributes.nominalHeatOutputW"), copy("Nominal heat output", "Nennwärmeleistung")),
                Quantity::new(QuantityKind::Power, attrs.nominal_heat_output_w),
                Quantity::new(QuantityKind::Power, 10_000.0),
                copy("Set nominalHeatOutputW to the rated generator output in watts.", "nominalHeatOutputW auf die Nennleistung in Watt setzen."),
            ));
    }
    out.push(qn.build());

    let fuel_ok = !attrs.fuel_type.is_empty();
    let mut fuel = CheckResult::assess(format!("vdi3805.6.fuel.{article}"), part.clone(), clause("6", "4.2"), subject.clone(), copy("Fuel / energy type", "Brennstoff-/Energieträger"))
        .annex(ANNEX)
        .status(if fuel_ok { CheckStatus::Pass } else { CheckStatus::Fail });
    if fuel_ok {
        fuel = fuel.explanation(copy(format!("Fuel type {} present.", attrs.fuel_type), format!("Energieträger {} vorhanden.", attrs.fuel_type)));
    } else {
        fuel = fuel
            .explanation(copy("Fuel type is empty.", "Energieträger fehlt."))
            .remedy(Remedy::one_of(
                SubjectRef::new(article, product_path(article, "configuration.attributes.fuelType"), copy("Fuel type", "Energieträger")),
                vec!["gas".into(), "oil".into(), "electric".into(), "biomass".into(), "district".into()],
                copy("Set fuelType to a Blatt 6 energy carrier code.", "fuelType auf einen Blatt-6-Energieträger setzen."),
            ));
    }
    out.push(fuel.build());

    let flow_ok = attrs.flow_temp_max_c >= 30.0 && attrs.flow_temp_max_c <= 110.0;
    let mut flow = CheckResult::assess(
        format!("vdi3805.6.flowTempMaxC.{article}"),
        part.clone(),
        clause("6", "4.3"),
        subject.clone(),
        copy("Maximum flow temperature", "Maximale Vorlauftemperatur"),
    )
    .annex(ANNEX)
    .status(if flow_ok { CheckStatus::Pass } else { CheckStatus::Fail });
    if flow_ok {
        flow = flow.explanation(copy(format!("t_flow,max = {:.1} °C.", attrs.flow_temp_max_c), format!("t_VL,max = {:.1} °C.", attrs.flow_temp_max_c)));
    } else {
        flow = flow
            .explanation(copy("flowTempMaxC must lie in [30, 110] °C.", "flowTempMaxC muss in [30, 110] °C liegen."))
            .remedy(Remedy::exactly(
                SubjectRef::new(article, attr_leaf_path(article, "flowTempMaxC"), copy("Flow temperature max", "Vorlauftemperatur max")),
                Quantity::new(QuantityKind::Temperature, attrs.flow_temp_max_c),
                Quantity::new(QuantityKind::Temperature, 80.0),
                copy("Set flowTempMaxC to a Blatt 6 design flow temperature (e.g. 80 °C).", "flowTempMaxC auf eine Blatt-6-Auslegungsvorlauftemperatur setzen (z. B. 80 °C)."),
            ));
    }
    out.push(flow.build());

    let ret_ok = attrs.return_temp_min_c >= 5.0 && attrs.return_temp_min_c < attrs.flow_temp_max_c.max(30.0);
    let mut ret = CheckResult::assess(
        format!("vdi3805.6.returnTempMinC.{article}"),
        part,
        clause("6", "4.4"),
        subject,
        copy("Minimum return temperature", "Minimale Rücklauftemperatur"),
    )
    .annex(ANNEX)
    .status(if ret_ok { CheckStatus::Pass } else { CheckStatus::Fail });
    if ret_ok {
        ret = ret.explanation(copy(format!("t_return,min = {:.1} °C.", attrs.return_temp_min_c), format!("t_RL,min = {:.1} °C.", attrs.return_temp_min_c)));
    } else {
        let required = (attrs.flow_temp_max_c - 20.0).clamp(20.0, 70.0);
        ret = ret
            .explanation(copy("returnTempMinC must be below flowTempMaxC and ≥ 5 °C.", "returnTempMinC muss unter flowTempMaxC und ≥ 5 °C liegen."))
            .remedy(Remedy::exactly(
                SubjectRef::new(article, attr_leaf_path(article, "returnTempMinC"), copy("Return temperature min", "Rücklauftemperatur min")),
                Quantity::new(QuantityKind::Temperature, attrs.return_temp_min_c),
                Quantity::new(QuantityKind::Temperature, required),
                copy(format!("Set returnTempMinC to about {required:.0} °C."), format!("returnTempMinC auf etwa {required:.0} °C setzen.")),
            ));
    }
    out.push(ret.build());
    out
}

fn check_operative_sheet(document: &Vdi3805Snapshot, product: &CatalogueProduct) -> Vec<CheckResult> {
    let mut out = Vec::new();
    let article = product.id.as_str();
    let sheet = product.sheet.0;
    let entry = SHEET_ENTRIES.get(sheet.saturating_sub(1) as usize);
    let status = entry.map(|e| e.status).unwrap_or(SchemaStatus::Reserved);
    if status == SchemaStatus::Reserved {
        return vec![CheckResult::assess(
            format!("vdi3805.{sheet}.reserved.{article}"),
            part_label(sheet),
            clause(&sheet.to_string(), "scope"),
            subject_product(product),
            copy("Reserved sheet", "Reserviertes Blatt"),
        )
        .annex(ANNEX)
        .not_applicable(copy("Sheet is reserved in the registry.", "Blatt ist im Register reserviert."))
        .build()];
    }
    if status == SchemaStatus::HistoricalProposal {
        return Vec::new();
    }
    let profile = profile_for_sheet(document, sheet);
    let keys = sheet_mandatory_keys(sheet, profile);
    let kv = record_kv(&product.records);
    let generic_entries = match &product.configuration.attributes {
        SheetAttributes::Generic(g) => g.entries.clone(),
        _ => GenericAttributes::from_map(&kv).entries,
    };
    let mut missing = Vec::new();
    for key in keys {
        let present = match *key {
            "product_group" => !product.identity.product_group.is_empty() || kv.contains_key("product_group") || generic_entries.iter().any(|e| e.key.eq_ignore_ascii_case("product_group") && !e.value.is_empty()),
            other => {
                kv.get(other).is_some_and(|v| !v.is_empty())
                    || generic_entries.iter().any(|e| e.key.eq_ignore_ascii_case(other) && !e.value.is_empty())
            }
        };
        if !present {
            missing.push(*key);
        }
    }
    let title_ok = !product.title.is_empty() && product.title.iter().any(|t| t.locale == "en" || t.locale == "de");
    let identity_ok = !product.identity.article_number.is_empty() && product.id == product.identity.article_number;
    let mut b = CheckResult::assess(
        format!("vdi3805.{sheet}.mandatory.{article}"),
        part_label(sheet),
        clause(&sheet.to_string(), "4.1"),
        subject_product(product),
        copy("Mandatory Blatt attributes", "Pflichtattribute des Blatts"),
    )
    .annex(ANNEX);
    if identity_ok && title_ok && missing.is_empty() {
        b = b.status(CheckStatus::Pass).explanation(copy(
            format!("Sheet {sheet} ({profile:?}) mandatory keys present in native records."),
            format!("Blatt {sheet} ({profile:?}): Pflichtattribute in nativen Sätzen vorhanden."),
        ));
    } else {
        let first = missing.first().copied().unwrap_or("product_group");
        let path = if first == "product_group" {
            product_path(article, "identity.productGroup")
        } else {
            generic_entry_value_path(article, &generic_entries, first)
        };
        let required = match first {
            "dn" | "dn_suction" | "dn_discharge" => 50.0,
            "airflow_m3_s" => 0.1,
            "cop" => 3.0,
            "volume_m3" => 0.1,
            "axial_force_n" => 1000.0,
            "outer_diameter_m" => 0.05,
            "wall_thickness_m" => 0.002,
            "pressure_drop_pa" => 50.0,
            "nominal_heat_output_w" => 5000.0,
            _ => 1.0,
        };
        b = b
            .status(CheckStatus::Fail)
            .explanation(copy(
                format!("Sheet {sheet} ({profile:?}) missing mandatory keys {missing:?}; identity_ok={identity_ok}; title_ok={title_ok}."),
                format!("Blatt {sheet} ({profile:?}) fehlt Pflichtfelder {missing:?}; identity_ok={identity_ok}; title_ok={title_ok}."),
            ))
            .remedy(Remedy::exactly(
                SubjectRef::new(article, path, copy("Mandatory attribute", "Pflichtattribut")),
                Quantity::new(QuantityKind::Dimensionless, 0.0),
                Quantity::new(QuantityKind::Dimensionless, required),
                copy(
                    format!("Provide Blatt {sheet} mandatory attribute `{first}` for {profile:?}; sync regenerates the 210 record."),
                    format!("Blatt-{sheet}-Pflichtattribut `{first}` für {profile:?} setzen; Sync erzeugt den 210-Satz neu."),
                ),
            ));
    }
    out.push(b.build());

    // Domain validation for mandatory enum keys (not just presence) — Blatt code lists.
    for key in keys {
        let Some(codes) = code_list_for_key(sheet, key) else { continue };
        let raw = kv.get(*key).cloned().or_else(|| generic_entries.iter().find(|e| e.key.eq_ignore_ascii_case(key)).map(|e| e.value.clone())).unwrap_or_default();
        if raw.is_empty() {
            continue; // presence Fail already emitted
        }
        let ok = codes.iter().any(|c| *c == raw.as_str());
        let path = generic_entry_value_path(article, &generic_entries, key);
        let mut c = CheckResult::assess(
            format!("vdi3805.{sheet}.domain.{key}.{article}"),
            part_label(sheet),
            clause(&sheet.to_string(), "4.1"),
            subject_product(product),
            copy(format!("Blatt {sheet} `{key}` domain"), format!("Blatt-{sheet}-`{key}`-Wertebereich")),
        )
        .annex(ANNEX)
        .status(if ok { CheckStatus::Pass } else { CheckStatus::Fail });
        if ok {
            c = c.explanation(copy(format!("`{key}` = `{raw}` is in the Blatt code list."), format!("`{key}` = `{raw}` ist in der Blatt-Codeliste.")));
        } else {
            c = c
                .explanation(copy(format!("`{key}` = `{raw}` is not an admitted code."), format!("`{key}` = `{raw}` ist kein zugelassener Code.")))
                .remedy(Remedy::one_of(
                    SubjectRef::new(article, path, copy(format!("Attribute {key}"), format!("Attribut {key}"))),
                    codes.iter().map(|s| (*s).to_string()).collect(),
                    copy(format!("Set `{key}` to an admitted Blatt code."), format!("`{key}` auf einen zugelassenen Blatt-Code setzen.")),
                ));
        }
        out.push(c.build());
    }

    for (key, min_v, max_v) in sheet_numeric_bounds(sheet) {
        let raw = kv.get(*key).cloned().or_else(|| generic_entries.iter().find(|e| e.key.eq_ignore_ascii_case(key)).map(|e| e.value.clone()));
        let Some(raw) = raw.filter(|s| !s.is_empty()) else { continue };
        let Ok(value) = raw.parse::<f64>() else { continue };
        let ok = value + 1e-12 >= *min_v && value <= *max_v + 1e-12;
        let path = generic_entry_value_path(article, &generic_entries, key);
        let mut c = CheckResult::assess(
            format!("vdi3805.{sheet}.range.{key}.{article}"),
            part_label(sheet),
            clause(&sheet.to_string(), "4.2"),
            subject_product(product),
            copy(format!("Blatt {sheet} attribute `{key}` range"), format!("Blatt-{sheet}-Attribut `{key}` Bereich")),
        )
        .annex(ANNEX)
        .status(if ok { CheckStatus::Pass } else { CheckStatus::Fail });
        if ok {
            c = c.explanation(copy(format!("{key} = {value} within [{min_v}, {max_v}]."), format!("{key} = {value} in [{min_v}, {max_v}].")));
        } else {
            let required = value.clamp(*min_v, *max_v);
            c = c
                .explanation(copy(format!("{key} = {value} outside [{min_v}, {max_v}]."), format!("{key} = {value} außerhalb [{min_v}, {max_v}].")))
                .remedy(Remedy::exactly(
                    SubjectRef::new(article, path, copy(format!("Attribute {key}"), format!("Attribut {key}"))),
                    Quantity::new(QuantityKind::Dimensionless, value),
                    Quantity::new(QuantityKind::Dimensionless, required),
                    copy(format!("Clamp `{key}` into [{min_v}, {max_v}]."), format!("`{key}` in [{min_v}, {max_v}] begrenzen.")),
                ));
        }
        out.push(c.build());
    }

    for curve_id in &product.configuration.function_refs {
        if let Some(curve) = document.curves.get(curve_id) {
            let mono = curve.points.windows(2).all(|w| w[1].x + 1e-12 >= w[0].x);
            let mut c = CheckResult::assess(
                format!("vdi3805.{sheet}.curve.{article}.{curve_id}"),
                part_label(sheet),
                clause(&sheet.to_string(), "5.1"),
                subject_product(product),
                copy("Characteristic data monotonicity", "Kennlinien-Monotonie"),
            )
            .annex(ANNEX)
            .status(if mono { CheckStatus::Pass } else { CheckStatus::Fail });
            if mono {
                c = c.explanation(copy(format!("Curve {curve_id} x-order is non-decreasing."), format!("Kennlinie {curve_id}: x-Folge ist monoton.")));
            } else if let Some((idx, cur, req)) = mono_increasing_remedy(curve_id, curve) {
                c = c
                    .explanation(copy(format!("Curve {curve_id} is not ordered."), format!("Kennlinie {curve_id} ist nicht geordnet.")))
                    .remedy(Remedy::exactly(
                        SubjectRef::new(curve_id, curve_point_path(curve_id, idx, "x"), copy("Curve point x", "Kennlinienpunkt x")),
                        Quantity::new(QuantityKind::Dimensionless, cur),
                        Quantity::new(QuantityKind::Dimensionless, req),
                        copy("Repair characteristic point so x is non-decreasing.", "Kennlinienpunkt so korrigieren, dass x monoton steigt."),
                    ));
            }
            out.push(c.build());
        }
    }
    out
}

fn check_sheet_product(document: &Vdi3805Snapshot, product: &CatalogueProduct) -> Vec<CheckResult> {
    let mut out = check_attributes_records_sync(product);
    match product.sheet.0 {
        2 => {
            let attrs = valve_attrs_from_product(product);
            out.extend(check_valve(document, product, &attrs));
        }
        3 => {
            let attrs = radiator_attrs_from_product(product);
            out.extend(check_radiator(product, &attrs));
        }
        5 => {
            let attrs = pump_attrs_from_product(product);
            out.extend(check_pump(document, product, &attrs));
        }
        6 => {
            let attrs = heat_attrs_from_product(product);
            out.extend(check_heat_generator(product, &attrs));
        }
        _ => out.extend(check_operative_sheet(document, product)),
    }
    out
}

fn check_catalog_integrity(document: &Vdi3805Snapshot) -> Vec<CheckResult> {
    let mut out = Vec::new();
    let product_ids: BTreeSet<_> = document.catalog.products.iter().map(|p| p.id.clone()).collect();

    let year = document.correction_as_of.year;
    let month = document.correction_as_of.month;
    let date_ok = (1990..=2100).contains(&year) && (1..=12).contains(&month);
    let mut corr = CheckResult::assess(
        "vdi3805.1.correctionAsOf",
        part_label(1),
        clause("1", "4.3"),
        subject_dataset(),
        copy("Correction date", "Korrekturdatum"),
    )
    .annex(ANNEX);
    if date_ok {
        corr = corr.status(CheckStatus::Pass).explanation(copy(
            format!("correctionAsOf {year:04}-{month:02} is a valid edition stamp."),
            format!("correctionAsOf {year:04}-{month:02} ist ein gültiger Ausgabenstempel."),
        ));
    } else {
        corr = corr
            .status(CheckStatus::Fail)
            .explanation(copy(
                format!("correctionAsOf {year}-{month} is out of range (year 1990–2100, month 1–12)."),
                format!("correctionAsOf {year}-{month} außerhalb des Bereichs (Jahr 1990–2100, Monat 1–12)."),
            ))
            .remedy(Remedy::exactly(
                SubjectRef::new("", "correctionAsOf.year", copy("Correction year", "Korrekturjahr")),
                Quantity::new(QuantityKind::Dimensionless, year as f64),
                Quantity::new(QuantityKind::Dimensionless, 2022.0),
                copy("Set correctionAsOf.year to a valid catalogue year (e.g. 2022).", "correctionAsOf.year auf ein gültiges Katalogjahr setzen (z. B. 2022)."),
            ));
    }
    out.push(corr.build());

    let mut dangling_index = Vec::new();
    for entry in &document.index.entries {
        if !product_ids.contains(&entry.product_id) {
            dangling_index.push(entry.product_id.clone());
        }
    }
    let mut idx = CheckResult::assess(
        "vdi3805.1.index",
        part_label(1),
        clause("1", "4.4"),
        subject_dataset(),
        copy("Catalogue index references", "Katalogindex-Referenzen"),
    )
    .annex(ANNEX);
    if dangling_index.is_empty() {
        idx = idx.status(CheckStatus::Pass).explanation(copy("Every index entry resolves to a catalogue product.", "Jeder Indexeintrag verweist auf ein Katalogprodukt."));
    } else {
        idx = idx
            .status(CheckStatus::Fail)
            .explanation(copy(
                format!("Index entries reference missing products: {dangling_index:?}."),
                format!("Indexeinträge verweisen auf fehlende Produkte: {dangling_index:?}."),
            ))
            .remedy(Remedy::exactly(
                SubjectRef::new("", "index.entries[0].productId", copy("Index product id", "Index-Produkt-ID")),
                Quantity::new(QuantityKind::Dimensionless, 0.0),
                Quantity::new(QuantityKind::Dimensionless, 1.0),
                copy("Point index.entries[].productId at an existing catalogue product id.", "index.entries[].productId auf eine vorhandene Katalog-Produkt-ID setzen."),
            ));
    }
    out.push(idx.build());

    for product in &document.catalog.products {
        let article = product.id.as_str();
        let accessory_ids: Vec<String> = product.accessories.iter().map(|a| a.accessory_id.clone()).collect();
        let dangling_acc: Vec<_> = accessory_ids.iter().filter(|id| !product_ids.contains(*id)).cloned().collect();
        let mut c = CheckResult::assess(
            format!("vdi3805.1.accessories.{article}"),
            part_label(1),
            clause("1", "4.5"),
            subject_product(product),
            copy("Product accessories", "Produktzubehör"),
        )
        .annex(ANNEX);
        if dangling_acc.is_empty() {
            c = c.status(CheckStatus::Pass).explanation(copy(
                "All accessory references resolve.",
                "Alle Zubehörreferenzen sind auflösbar.",
            ));
        } else {
            c = c
                .status(CheckStatus::Fail)
                .explanation(copy(
                    format!("accessories reference unknown products {dangling_acc:?}."),
                    format!("accessories verweist auf unbekannte Produkte {dangling_acc:?}."),
                ))
                .remedy(Remedy::exactly(
                    SubjectRef::new(article, product_path(article, "accessories[0].accessoryId"), copy("Accessory id", "Zubehör-ID")),
                    Quantity::new(QuantityKind::Dimensionless, 0.0),
                    Quantity::new(QuantityKind::Dimensionless, 1.0),
                    copy(
                        "Point accessories[].accessoryId at an existing product id or clear the list.",
                        "accessories[].accessoryId auf eine vorhandene Produkt-ID setzen oder die Liste leeren.",
                    ),
                ));
        }
        out.push(c.build());

        let component_ids: Vec<String> = product.components.iter().map(|c| c.component_id.clone()).collect();
        let dangling_comp: Vec<_> = component_ids.iter().filter(|id| !product_ids.contains(*id)).cloned().collect();
        let mut c = CheckResult::assess(
            format!("vdi3805.1.components.{article}"),
            part_label(1),
            clause("1", "4.5"),
            subject_product(product),
            copy("Product components", "Produktkomponenten"),
        )
        .annex(ANNEX);
        if dangling_comp.is_empty() {
            c = c.status(CheckStatus::Pass).explanation(copy(
                "All component references resolve.",
                "Alle Komponentenreferenzen sind auflösbar.",
            ));
        } else {
            c = c
                .status(CheckStatus::Fail)
                .explanation(copy(
                    format!("components reference unknown products {dangling_comp:?}."),
                    format!("components verweist auf unbekannte Produkte {dangling_comp:?}."),
                ))
                .remedy(Remedy::exactly(
                    SubjectRef::new(article, product_path(article, "components[0].componentId"), copy("Component id", "Komponenten-ID")),
                    Quantity::new(QuantityKind::Dimensionless, 0.0),
                    Quantity::new(QuantityKind::Dimensionless, 1.0),
                    copy(
                        "Point components[].componentId at an existing product id or clear the list.",
                        "components[].componentId auf eine vorhandene Produkt-ID setzen oder die Liste leeren.",
                    ),
                ));
        }
        out.push(c.build());

        // Accessories quantity ≥1 and required accessories present (VDI 3805-1 §4.5).
        for (i, link) in product.accessories.iter().enumerate() {
            let qty_ok = link.quantity >= 1;
            let mut q = CheckResult::assess(
                format!("vdi3805.1.accessories.quantity.{article}.{i}"),
                part_label(1),
                clause("1", "4.5"),
                subject_product(product),
                copy("Accessory quantity", "Zubehöranzahl"),
            )
            .annex(ANNEX)
            .status(if qty_ok { CheckStatus::Pass } else { CheckStatus::Fail });
            if qty_ok {
                q = q.explanation(copy(format!("Accessory `{}` quantity {} ≥ 1.", link.accessory_id, link.quantity), format!("Zubehör `{}` Anzahl {} ≥ 1.", link.accessory_id, link.quantity)));
            } else {
                q = q
                    .explanation(copy("Accessory quantity must be ≥ 1.", "Zubehöranzahl muss ≥ 1 sein."))
                    .remedy(Remedy::at_least(
                        SubjectRef::new(article, product_path(article, &format!("accessories[{i}].quantity")), copy("Accessory quantity", "Zubehöranzahl")),
                        Quantity::new(QuantityKind::Dimensionless, link.quantity as f64),
                        Quantity::new(QuantityKind::Dimensionless, 1.0),
                        copy("Set accessories[].quantity to at least 1.", "accessories[].quantity auf mindestens 1 setzen."),
                    ));
            }
            out.push(q.build());
            if link.required && !product_ids.contains(&link.accessory_id) {
                out.push(
                    CheckResult::assess(
                        format!("vdi3805.1.accessories.required.{article}.{i}"),
                        part_label(1),
                        clause("1", "4.5"),
                        subject_product(product),
                        copy("Required accessory present", "Erforderliches Zubehör vorhanden"),
                    )
                    .annex(ANNEX)
                    .status(CheckStatus::Fail)
                    .explanation(copy(
                        format!("Required accessory `{}` is missing from the catalogue.", link.accessory_id),
                        format!("Erforderliches Zubehör `{}` fehlt im Katalog.", link.accessory_id),
                    ))
                    .remedy(Remedy::exactly(
                        SubjectRef::new(article, product_path(article, &format!("accessories[{i}].accessoryId")), copy("Accessory id", "Zubehör-ID")),
                        Quantity::new(QuantityKind::Dimensionless, 0.0),
                        Quantity::new(QuantityKind::Dimensionless, 1.0),
                        copy("Add the required accessory product to the catalogue.", "Erforderliches Zubehörprodukt dem Katalog hinzufügen."),
                    ))
                    .build(),
                );
            }
        }
        for (i, link) in product.components.iter().enumerate() {
            let qty_ok = link.quantity >= 1;
            let mut q = CheckResult::assess(
                format!("vdi3805.1.components.quantity.{article}.{i}"),
                part_label(1),
                clause("1", "4.6"),
                subject_product(product),
                copy("Component quantity", "Komponentenanzahl"),
            )
            .annex(ANNEX)
            .status(if qty_ok { CheckStatus::Pass } else { CheckStatus::Fail });
            if qty_ok {
                q = q.explanation(copy(format!("Component `{}` quantity {} ≥ 1.", link.component_id, link.quantity), format!("Komponente `{}` Anzahl {} ≥ 1.", link.component_id, link.quantity)));
            } else {
                q = q
                    .explanation(copy("Component quantity must be ≥ 1.", "Komponentenanzahl muss ≥ 1 sein."))
                    .remedy(Remedy::at_least(
                        SubjectRef::new(article, product_path(article, &format!("components[{i}].quantity")), copy("Component quantity", "Komponentenanzahl")),
                        Quantity::new(QuantityKind::Dimensionless, link.quantity as f64),
                        Quantity::new(QuantityKind::Dimensionless, 1.0),
                        copy("Set components[].quantity to at least 1.", "components[].quantity auf mindestens 1 setzen."),
                    ));
            }
            out.push(q.build());
        }

        // Geometry bbox / connections / parameters when geometryRef set (VDI 3805-1 §5.2).
        if let Some(gref) = product.configuration.geometry_ref.as_ref() {
            if let Some(geom) = document.geometry.get(gref) {
                let bbox = &geom.bbox;
                let mut outside = Vec::new();
                for conn in &geom.connections {
                    let [x, y, z] = conn.position;
                    if x < bbox.min_x || x > bbox.max_x || y < bbox.min_y || y > bbox.max_y || z < bbox.min_z || z > bbox.max_z {
                        outside.push(conn.id.clone());
                    }
                }
                let bbox_ok = outside.is_empty() && bbox.max_x >= bbox.min_x && bbox.max_y >= bbox.min_y && bbox.max_z >= bbox.min_z;
                let mut g = CheckResult::assess(
                    format!("vdi3805.1.geometry.bbox.{article}"),
                    part_label(1),
                    clause("1", "5.2"),
                    subject_product(product),
                    copy("Geometry bounding box", "Geometrie-Begrenzungsrahmen"),
                )
                .annex(ANNEX)
                .status(if bbox_ok { CheckStatus::Pass } else { CheckStatus::Fail });
                if bbox_ok {
                    g = g.explanation(copy("All connection points lie inside the geometry bbox.", "Alle Anschlusspunkte liegen im Geometrie-BBox."));
                } else {
                    g = g
                        .explanation(copy(format!("Connection points outside bbox: {outside:?}."), format!("Anschlusspunkte außerhalb der BBox: {outside:?}.")))
                        .remedy(Remedy::exactly(
                            SubjectRef::new(gref, format!("geometry[id={gref}].bbox.maxX"), copy("BBox max X", "BBox max X")),
                            Quantity::new(QuantityKind::Length, bbox.max_x),
                            Quantity::new(QuantityKind::Length, bbox.max_x.max(bbox.min_x) + 0.01),
                            copy("Expand geometry bbox so every connection point is inside.", "Geometrie-BBox so erweitern, dass jeder Anschlusspunkt innen liegt."),
                        ));
                }
                out.push(g.build());
                let params_ok = !geom.parameters.is_empty();
                let mut p = CheckResult::assess(
                    format!("vdi3805.1.geometry.parameters.{article}"),
                    part_label(1),
                    clause("1", "5.2"),
                    subject_product(product),
                    copy("Geometry parameters", "Geometrieparameter"),
                )
                .annex(ANNEX)
                .status(if params_ok { CheckStatus::Pass } else { CheckStatus::Fail });
                if params_ok {
                    p = p.explanation(copy("Geometry parameters present for referenced geometry.", "Geometrieparameter für referenzierte Geometrie vorhanden."));
                } else {
                    p = p
                        .explanation(copy("geometryRef set but parameters map is empty.", "geometryRef gesetzt, aber Parameter-Map leer."))
                        .remedy(Remedy::exactly(
                            SubjectRef::new(gref, format!("geometry[id={gref}].parameters"), copy("Geometry parameters", "Geometrieparameter")),
                            Quantity::new(QuantityKind::Dimensionless, 0.0),
                            Quantity::new(QuantityKind::Dimensionless, 1.0),
                            copy("Provide Blatt geometry definition parameters for the referenced geometry.", "Blatt-Geometriedefinitionsparameter für die referenzierte Geometrie angeben."),
                        ));
                }
                out.push(p.build());
            }
        }
    }

    // Index entry consistency with products (sheet / DN / tags) — VDI 3805-1 §4.4.
    for (i, entry) in document.index.entries.iter().enumerate() {
        let Some(product) = document.catalog.products.iter().find(|p| p.id == entry.product_id) else { continue };
        let sheet_ok = entry.sheet == product.sheet;
        let mut s = CheckResult::assess(
            format!("vdi3805.1.index.sheet.{i}"),
            part_label(1),
            clause("1", "4.4"),
            subject_dataset(),
            copy("Index sheet consistency", "Index-Blatt-Konsistenz"),
        )
        .annex(ANNEX)
        .status(if sheet_ok { CheckStatus::Pass } else { CheckStatus::Fail });
        if sheet_ok {
            s = s.explanation(copy(format!("Index sheet {} matches product.", entry.sheet.0), format!("Index-Blatt {} entspricht dem Produkt.", entry.sheet.0)));
        } else {
            s = s
                .explanation(copy(format!("Index sheet {} ≠ product sheet {}.", entry.sheet.0, product.sheet.0), format!("Index-Blatt {} ≠ Produktblatt {}.", entry.sheet.0, product.sheet.0)))
                .remedy(Remedy::exactly(
                    SubjectRef::new("", format!("index.entries[{i}].sheet"), copy("Index sheet", "Index-Blatt")),
                    Quantity::new(QuantityKind::Dimensionless, entry.sheet.0 as f64),
                    Quantity::new(QuantityKind::Dimensionless, product.sheet.0 as f64),
                    copy("Align index.entries[].sheet with the referenced product sheet.", "index.entries[].sheet an das referenzierte Produktblatt angleichen."),
                ));
        }
        out.push(s.build());

        if let Some(dn) = entry.dn {
            let product_dn = match &product.configuration.attributes {
                SheetAttributes::ValveHeating(a) => Some(a.dn),
                SheetAttributes::PumpHeating(a) => Some(a.dn_discharge),
                _ => record_kv(&product.records).get("dn").and_then(|v| v.parse().ok()),
            };
            if let Some(pdn) = product_dn {
                let dn_ok = dn == pdn;
                let mut d = CheckResult::assess(
                    format!("vdi3805.1.index.dn.{i}"),
                    part_label(1),
                    clause("1", "4.4"),
                    subject_dataset(),
                    copy("Index DN consistency", "Index-DN-Konsistenz"),
                )
                .annex(ANNEX)
                .status(if dn_ok { CheckStatus::Pass } else { CheckStatus::Fail });
                if dn_ok {
                    d = d.explanation(copy(format!("Index DN {dn} matches product."), format!("Index-DN {dn} entspricht dem Produkt.")));
                } else {
                    d = d
                        .explanation(copy(format!("Index DN {dn} ≠ product DN {pdn}."), format!("Index-DN {dn} ≠ Produkt-DN {pdn}.")))
                        .remedy(Remedy::exactly(
                            SubjectRef::new("", format!("index.entries[{i}].dn"), copy("Index DN", "Index-DN")),
                            Quantity::new(QuantityKind::Dimensionless, dn as f64),
                            Quantity::new(QuantityKind::Dimensionless, pdn as f64),
                            copy("Align index.entries[].dn with the product DN.", "index.entries[].dn an die Produkt-DN angleichen."),
                        ));
                }
                out.push(d.build());
            }
        }

        let tag_ok = entry.tags.is_empty() || entry.tags.iter().any(|t| product.title.iter().any(|title| title.text.to_lowercase().contains(&t.to_lowercase()) || product.identity.product_group.eq_ignore_ascii_case(t)));
        let mut tg = CheckResult::assess(
            format!("vdi3805.1.index.tags.{i}"),
            part_label(1),
            clause("1", "4.4"),
            subject_dataset(),
            copy("Index tags consistency", "Index-Tag-Konsistenz"),
        )
        .annex(ANNEX)
        .status(if tag_ok { CheckStatus::Pass } else { CheckStatus::Fail });
        if tag_ok {
            tg = tg.explanation(copy("Index tags relate to the product identity/title.", "Index-Tags beziehen sich auf Produktidentität/Titel."));
        } else {
            tg = tg
                .explanation(copy(format!("Index tags {:?} do not match product title/group.", entry.tags), format!("Index-Tags {:?} passen nicht zu Produkttitel/-gruppe.", entry.tags)))
                .remedy(Remedy::exactly(
                    SubjectRef::new("", format!("index.entries[{i}].tags[0]"), copy("Index tag", "Index-Tag")),
                    Quantity::new(QuantityKind::Dimensionless, 0.0),
                    Quantity::new(QuantityKind::Dimensionless, 1.0),
                    copy("Align index.entries[].tags with the product title or product group.", "index.entries[].tags an Produkttitel oder Produktgruppe angleichen."),
                ));
        }
        out.push(tg.build());
    }

    // Header version / building-system / created (VDI 3805-1 §4.1–4.3) — single header at catalog.file.
    let file = &document.catalog.file;
    let expected_hv = "3805";
    let hv_ok = file.header_version == expected_hv || file.header_version.starts_with("3805");
    let mut hv = CheckResult::assess(
        "vdi3805.1.headerVersion",
        part_label(1),
        clause("1", "4.1"),
        subject_dataset(),
        copy("Manufacturer file header version", "Herstellerdatei-Kopfversion"),
    )
    .annex(ANNEX)
    .status(if hv_ok { CheckStatus::Pass } else { CheckStatus::Fail });
    if hv_ok {
        hv = hv.explanation(copy(format!("headerVersion `{}` matches the Blatt edition family.", file.header_version), format!("headerVersion `{}` entspricht der Blatt-Ausgabenfamilie.", file.header_version)));
    } else {
        hv = hv
            .explanation(copy(format!("headerVersion `{}` must identify VDI 3805 (e.g. `3805`).", file.header_version), format!("headerVersion `{}` muss VDI 3805 kennzeichnen (z. B. `3805`).", file.header_version)))
            .remedy(Remedy::exactly(
                SubjectRef::new("", "catalog.file.headerVersion", copy("Header version", "Kopfversion")),
                Quantity::new(QuantityKind::Dimensionless, 0.0),
                Quantity::new(QuantityKind::Dimensionless, 3805.0),
                copy("Set catalog.file.headerVersion to `3805` for the declared Blatt edition.", "catalog.file.headerVersion auf `3805` für die deklarierte Blattausgabe setzen."),
            ));
    }
    out.push(hv.build());

    let sheets: BTreeSet<u16> = document.catalog.products.iter().map(|p| p.sheet.0).collect();
    // Building-system (Gewerk) number must be consistent with product sheets' domain (VDI 3805-1 §4.2).
    let bsn = &file.building_system_number;
    let bsn_ok = !bsn.system_code.is_empty() && !bsn.subsystem.is_empty();
    let mut bs = CheckResult::assess(
        "vdi3805.1.buildingSystemNumber",
        part_label(1),
        clause("1", "4.2"),
        subject_dataset(),
        copy("Building system number", "Gewerknummer"),
    )
    .annex(ANNEX)
    .status(if bsn_ok { CheckStatus::Pass } else { CheckStatus::Fail });
    if bsn_ok {
        bs = bs.explanation(copy(
            format!("buildingSystemNumber {}.{}.{} present; products on sheets {:?}.", bsn.system_code, bsn.subsystem, bsn.sequence, sheets),
            format!("buildingSystemNumber {}.{}.{} vorhanden; Produkte auf Blättern {:?}.", bsn.system_code, bsn.subsystem, bsn.sequence, sheets),
        ));
    } else {
        bs = bs
            .explanation(copy("buildingSystemNumber is incomplete.", "buildingSystemNumber ist unvollständig."))
            .remedy(Remedy::exactly(
                SubjectRef::new("", "catalog.file.buildingSystemNumber.systemCode", copy("System code", "Systemcode")),
                Quantity::new(QuantityKind::Dimensionless, 0.0),
                Quantity::new(QuantityKind::Dimensionless, 420.0),
                copy("Set catalog.file.buildingSystemNumber to the Gewerk matching the product sheets.", "catalog.file.buildingSystemNumber auf das zu den Produktblättern passende Gewerk setzen."),
            ));
    }
    out.push(bs.build());

    // created vs correctionAsOf / edition dates (VDI 3805-1 §4.3).
    let created_ok = {
        let parts: Vec<_> = file.created.split('-').collect();
        if parts.len() >= 2 {
            let y: u16 = parts[0].parse().unwrap_or(0);
            let m: u16 = parts[1].parse().unwrap_or(0);
            y > 0
                && m >= 1
                && m <= 12
                && (y as i32 * 12 + m as i32) <= (document.correction_as_of.year as i32 * 12 + document.correction_as_of.month as i32 + 12)
        } else {
            false
        }
    };
    let mut cr = CheckResult::assess(
        "vdi3805.1.created",
        part_label(1),
        clause("1", "4.3"),
        subject_dataset(),
        copy("Manufacturer file created date", "Erstellungsdatum der Herstellerdatei"),
    )
    .annex(ANNEX)
    .status(if created_ok { CheckStatus::Pass } else { CheckStatus::Fail });
    if created_ok {
        cr = cr.explanation(copy(
            format!("created `{}` is consistent with correctionAsOf {}-{:02}.", file.created, document.correction_as_of.year, document.correction_as_of.month),
            format!("created `{}` ist konsistent mit correctionAsOf {}-{:02}.", file.created, document.correction_as_of.year, document.correction_as_of.month),
        ));
    } else {
        cr = cr
            .explanation(copy(
                format!("created `{}` must be a YYYY-MM[-DD] date not after correctionAsOf.", file.created),
                format!("created `{}` muss ein YYYY-MM[-DD]-Datum nicht nach correctionAsOf sein.", file.created),
            ))
            .remedy(Remedy::exactly(
                SubjectRef::new("", "catalog.file.created", copy("Created date", "Erstellungsdatum")),
                Quantity::new(QuantityKind::Dimensionless, 0.0),
                Quantity::new(QuantityKind::Dimensionless, document.correction_as_of.year as f64),
                copy("Set catalog.file.created to a date consistent with correctionAsOf / edition dates.", "catalog.file.created auf ein zu correctionAsOf/Ausgabedaten passendes Datum setzen."),
            ));
    }
    out.push(cr.build());

    out
}

fn check_historical(document: &Vdi3805Snapshot) -> Vec<CheckResult> {
    let mut out = Vec::new();
    for product in &document.catalog.products {
        let entry = SHEET_ENTRIES.get(product.sheet.0.saturating_sub(1) as usize);
        if entry.is_some_and(|e| e.status == SchemaStatus::HistoricalProposal) {
            let mut b = CheckResult::assess(
                format!("vdi3805.{}.historical.{}", product.sheet.0, product.id),
                part_label(product.sheet.0),
                clause(&product.sheet.0.to_string(), "status"),
                subject_product(product),
                copy("Historical proposal sheet", "Historischer Vorschlagsblatt"),
            )
            .annex(ANNEX);
            if document.strict_mode {
                b = b
                    .status(CheckStatus::Fail)
                    .explanation(copy("Historical proposal sheet not allowed in strict mode.", "Historisches Vorschlagsblatt in Strict Mode nicht zulässig."))
                    .remedy(Remedy::exactly(
                        SubjectRef::new("", "strictMode", copy("Strict mode", "Strict Mode")),
                        Quantity::new(QuantityKind::Dimensionless, 1.0),
                        Quantity::new(QuantityKind::Dimensionless, 0.0),
                        copy("Remove the historical-sheet product or set strictMode to false.", "Produkt mit historischem Blatt entfernen oder strictMode auf false setzen."),
                    ));
            } else {
                b = b.status(CheckStatus::Pass).explanation(copy("Historical proposal acknowledged (strict mode off).", "Historischer Vorschlag anerkannt (Strict Mode aus)."));
            }
            out.push(b.build());
        }
    }
    out
}

fn check_edition_profiles(document: &Vdi3805Snapshot) -> Vec<CheckResult> {
    let mut out = Vec::new();
    for (key, profile) in &document.edition_profile {
        let Ok(sheet) = key.parse::<u16>() else { continue };
        let product = document.catalog.products.iter().find(|p| p.sheet.0 == sheet);
        let mut b = CheckResult::assess(
            format!("vdi3805.{sheet}.edition_profile"),
            part_label(sheet),
            clause(&sheet.to_string(), "edition"),
            subject_dataset(),
            copy("Edition profile selection", "Editionsprofilwahl"),
        )
        .annex(ANNEX);
        let Some(product) = product else {
            out.push(b.not_applicable(copy(format!("No product for multi-profile sheet {sheet}."), format!("Kein Produkt für Mehrprofil-Blatt {sheet}."))).build());
            continue;
        };
        let keys = sheet_mandatory_keys(sheet, *profile);
        let kv = record_kv(&product.records);
        let missing: Vec<_> = keys
            .iter()
            .copied()
            .filter(|key| match *key {
                "product_group" => product.identity.product_group.is_empty() && !kv.contains_key("product_group"),
                other => !kv.get(other).is_some_and(|v| !v.is_empty()),
            })
            .collect();
        if missing.is_empty() {
            b = b
                .status(CheckStatus::Pass)
                .explanation(copy(format!("Sheet {sheet} profile {profile:?} mandatory set satisfied."), format!("Blatt {sheet} Profil {profile:?}: Pflichtsatz erfüllt.")));
        } else {
            b = b
                .status(CheckStatus::Fail)
                .explanation(copy(
                    format!("Profile {profile:?} missing mandatory keys {missing:?}."),
                    format!("Profil {profile:?} fehlt Pflichtfelder {missing:?}."),
                ))
                .remedy({
                    let first = missing.first().copied().unwrap_or("product_group");
                    let path = if first == "product_group" {
                        product_path(&product.id, "identity.productGroup")
                    } else {
                        let entries = match &product.configuration.attributes {
                            SheetAttributes::Generic(g) => g.entries.clone(),
                            _ => Vec::new(),
                        };
                        generic_entry_value_path(&product.id, &entries, first)
                    };
                    Remedy::exactly(
                        SubjectRef::new(&product.id, path, copy("Edition mandatory attribute", "Editions-Pflichtattribut")),
                        Quantity::new(QuantityKind::Dimensionless, 0.0),
                        Quantity::new(QuantityKind::Dimensionless, 1.0),
                        copy(
                            format!("Set `{first}` for {profile:?} on sheet {sheet}; sync regenerates the 210 record."),
                            format!("`{first}` für {profile:?} auf Blatt {sheet} setzen; Sync erzeugt den 210-Satz neu."),
                        ),
                    )
                });
        }
        out.push(b.build());
    }
    out
}

fn check_operative_sheet_coverage(document: &Vdi3805Snapshot) -> Vec<CheckResult> {
    let missing: Vec<u16> = SHEET_ENTRIES
        .iter()
        .filter(|e| is_operative(e.status) && e.id.0 != 1)
        .filter(|e| !document.catalog.products.iter().any(|p| p.sheet == e.id))
        .map(|e| e.id.0)
        .collect();
    if missing.is_empty() {
        return Vec::new();
    }
    vec![CheckResult::assess(
        "vdi3805.coverage.operative",
        part_label(1),
        clause("1", "scope"),
        subject_dataset(),
        copy("Operative sheet coverage", "Abdeckung operativer Blätter"),
    )
    .annex(ANNEX)
    .not_applicable(copy(
        format!("{} operative sheets have no catalogue product (e.g. {:?}).", missing.len(), &missing[..missing.len().min(8)]),
        format!("{} operative Blätter ohne Katalogprodukt (z. B. {:?}).", missing.len(), &missing[..missing.len().min(8)]),
    ))
    .build()]
}

/// 📋️ VDI 3805 Part 1 + claimed Blatt conformance report (no registry/IO plumbing noise).
pub fn evaluate(document: &Vdi3805Snapshot) -> CheckReport {
    let mut report = CheckReport::default();
    report.extend(check_part1(document));
    report.extend(check_catalog_integrity(document));
    for product in &document.catalog.products {
        report.extend(check_sheet_product(document, product));
    }
    report.extend(check_historical(document));
    report.extend(check_edition_profiles(document));
    report.extend(check_operative_sheet_coverage(document));
    if document.catalog.products.is_empty() {
        report.push(
            CheckResult::assess("vdi3805.1.empty", part_label(1), clause("1", "4.1"), subject_dataset(), copy("Catalogue products", "Katalogprodukte"))
                .annex(ANNEX)
                .not_applicable(copy("No products to assess against product sheets.", "Keine Produkte für Blattprüfungen vorhanden."))
                .build(),
        );
    }
    report
}

// Preserve part_N modules as thin applicability wrappers for existing tests.
macro_rules! define_vdi_part {
    ($module:ident, $num:literal, reserved) => {
        pub mod $module {
            use super::*;
            pub fn metadata() -> &'static SheetEntry {
                &SHEET_ENTRIES[$num - 1]
            }
            pub fn check(document: &Vdi3805Snapshot) -> CheckResult {
                let _ = document;
                CheckResult::assess(format!("vdi3805.{}.reserved", $num), part_label($num), clause(stringify!($num), "scope"), subject_dataset(), copy("Reserved sheet", "Reserviertes Blatt"))
                    .annex(ANNEX)
                    .not_applicable(copy(format!("sheet {} reserved", $num), format!("Blatt {} reserviert", $num)))
                    .build()
            }
        }
    };
    ($module:ident, $num:literal, historical) => {
        pub mod $module {
            use super::*;
            pub fn metadata() -> &'static SheetEntry {
                &SHEET_ENTRIES[$num - 1]
            }
            pub fn check(document: &Vdi3805Snapshot) -> CheckResult {
                let has = document.catalog.products.iter().any(|p| p.sheet.0 == $num);
                if !has {
                    return CheckResult::assess(format!("vdi3805.{}.historical", $num), part_label($num), clause(stringify!($num), "status"), subject_dataset(), copy("Historical proposal sheet", "Historischer Vorschlagsblatt"))
                        .annex(ANNEX)
                        .not_applicable(copy("No product on historical sheet.", "Kein Produkt auf historischem Blatt."))
                        .build();
                }
                evaluate(document).checks.into_iter().find(|c| c.id.contains(&format!(".{}.historical", $num))).unwrap_or_else(|| {
                    CheckResult::assess(format!("vdi3805.{}.historical", $num), part_label($num), clause(stringify!($num), "status"), subject_dataset(), copy("Historical proposal sheet", "Historischer Vorschlagsblatt"))
                        .annex(ANNEX)
                        .status(CheckStatus::Pass)
                        .build()
                })
            }
        }
    };
    ($module:ident, $num:literal, multi_profile) => {
        pub mod $module {
            use super::*;
            pub fn metadata() -> &'static SheetEntry {
                &SHEET_ENTRIES[$num - 1]
            }
            pub fn check(document: &Vdi3805Snapshot) -> CheckResult {
                if document.catalog.product_for_sheet(SheetId($num)).is_none() {
                    return CheckResult::assess(format!("vdi3805.{}.profile", $num), part_label($num), clause(stringify!($num), "profile"), subject_dataset(), copy("Edition profile", "Editionsprofil"))
                        .annex(ANNEX)
                        .not_applicable(copy(format!("No product for sheet {}.", $num), format!("Kein Produkt für Blatt {}.", $num)))
                        .build();
                }
                let profile = profile_for_sheet(document, $num);
                let product = document.catalog.product_for_sheet(SheetId($num)).expect("product");
                let keys = sheet_mandatory_keys($num, profile);
                let kv = record_kv(&product.records);
                let missing: Vec<_> = keys
                    .iter()
                    .copied()
                    .filter(|key| match *key {
                        "product_group" => product.identity.product_group.is_empty() && !kv.contains_key("product_group"),
                        other => !kv.get(other).is_some_and(|v| !v.is_empty()),
                    })
                    .collect();
                if missing.is_empty() {
                    CheckResult::assess(format!("vdi3805.{}.profile", $num), part_label($num), clause(stringify!($num), "profile"), subject_dataset(), copy("Edition profile", "Editionsprofil"))
                        .annex(ANNEX)
                        .status(CheckStatus::Pass)
                        .explanation(copy(format!("sheet {} profile {:?} ok", $num, profile), format!("Blatt-{}-Profil {:?} ok", $num, profile)))
                        .build()
                } else {
                    CheckResult::assess(format!("vdi3805.{}.profile", $num), part_label($num), clause(stringify!($num), "profile"), subject_dataset(), copy("Edition profile", "Editionsprofil"))
                        .annex(ANNEX)
                        .status(CheckStatus::Fail)
                        .explanation(copy(format!("profile {:?} missing {:?}", profile, missing), format!("Profil {:?} fehlt {:?}", profile, missing)))
                        .remedy({
                            let first = missing.first().copied().unwrap_or("product_group");
                            let path = if first == "product_group" {
                                product_path(&product.id, "identity.productGroup")
                            } else {
                                let entries = match &product.configuration.attributes {
                                    SheetAttributes::Generic(g) => g.entries.clone(),
                                    _ => Vec::new(),
                                };
                                generic_entry_value_path(&product.id, &entries, first)
                            };
                            Remedy::exactly(
                                SubjectRef::new(&product.id, path, copy("Edition mandatory attribute", "Editions-Pflichtattribut")),
                                Quantity::new(QuantityKind::Dimensionless, 0.0),
                                Quantity::new(QuantityKind::Dimensionless, 1.0),
                                copy(
                                    format!("Set `{first}` to satisfy {:?} keys: {}", profile, keys.join(", ")),
                                    format!("`{first}` setzen für {:?}: {}", profile, keys.join(", ")),
                                ),
                            )
                        })
                        .build()
                }
            }
        }
    };
    ($module:ident, $num:literal) => {
        pub mod $module {
            use super::*;
            pub fn metadata() -> &'static SheetEntry {
                &SHEET_ENTRIES[$num - 1]
            }
            pub fn check(document: &Vdi3805Snapshot) -> CheckResult {
                if let Some(product) = document.catalog.product_for_sheet(SheetId($num)) {
                    let checks = check_sheet_product(document, product);
                    if let Some(fail) = checks.iter().find(|c| c.status == CheckStatus::Fail) {
                        return fail.clone();
                    }
                    if let Some(pass) = checks.into_iter().find(|c| c.status == CheckStatus::Pass) {
                        return pass;
                    }
                    return CheckResult::assess(
                        format!("vdi3805.{}.na", $num),
                        part_label($num),
                        clause(stringify!($num), "scope"),
                        subject_product(product),
                        copy("Sheet without applicable checks", "Blatt ohne anwendbare Prüfung"),
                    )
                    .annex(ANNEX)
                    .not_applicable(copy(format!("No applicable Pass/Fail for sheet {}.", $num), format!("Keine anwendbare Prüfung für Blatt {}.", $num)))
                    .build();
                }
                CheckResult::assess(format!("vdi3805.{}.na", $num), part_label($num), clause(stringify!($num), "scope"), subject_dataset(), copy("Sheet without product", "Blatt ohne Produkt"))
                    .annex(ANNEX)
                    .not_applicable(copy(format!("No product for sheet {}.", $num), format!("Kein Produkt für Blatt {}.", $num)))
                    .build()
            }
        }
    };
}

pub mod part_1 {
    use super::*;
    pub fn metadata() -> &'static SheetEntry {
        &SHEET_ENTRIES[0]
    }
    pub fn check(document: &Vdi3805Snapshot) -> CheckResult {
        check_part1(document).into_iter().find(|c| c.status == CheckStatus::Fail).unwrap_or_else(|| {
            CheckResult::assess("vdi3805.1.structure", part_label(1), clause("1", "structure"), subject_dataset(), copy("Part 1 structure", "Teil-1-Struktur"))
                .annex(ANNEX)
                .status(CheckStatus::Pass)
                .explanation(copy("Part 1 structure valid", "Teil-1-Struktur gültig"))
                .build()
        })
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
//#endregion 🔖️ComplianceReport

//#region 🧪️ComplianceReportTests
#[cfg(test)]
#[path = "🧪️tests/🔬️compliance-report/🦀️.rs"]
mod compliance_report_tests;
//#endregion 🧪️ComplianceReportTests

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use super::outline::Vdi3805Outline;
//#endregion 🔁️Re-exports
