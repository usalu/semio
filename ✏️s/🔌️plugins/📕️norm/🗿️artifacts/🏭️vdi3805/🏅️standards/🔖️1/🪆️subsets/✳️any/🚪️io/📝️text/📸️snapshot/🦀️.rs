//! 📜️ VDI 3805 app — textual document grammar surface + laws (constitutional: dsl).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::Vdi3805Snapshot;

/// 📜️ Bundled reference-catalogue example (`.semio` envelope + DSL body).
pub const REFERENCE_EXAMPLE_TEXT: &str = include_str!("../../../🖼️assets/🎬️demo/🗣️.dsl.semio");

/// 📖️ Parses VDI 3805 DSL text into a `Document`.
pub fn parse_dsl(text: &str) -> Result<Vdi3805Snapshot, semio_framework_diagnostic::TextError> {
    <Vdi3805Snapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Document` back to `.vdi3805` DSL text.
pub fn print_dsl(document: &Vdi3805Snapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type Vdi3805SnapshotText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::snapshot::*;
use crate::{CatalogIndex, CharacteristicCurve, EditionId, EditionProfileChoice, ManufacturerCatalog, ParametricGeometry, SecurityLimits};
use framework_schema::ArtifactSchema;
use std::collections::BTreeMap;

pub fn encode_vdi3805_snapshot_json(snapshot: &Vdi3805Snapshot) -> String { semio_framework_pack_json::to_json_string(snapshot) }

pub fn decode_vdi3805_snapshot_json(text: &str) -> Result<Vdi3805Snapshot, String> { semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|e| e.to_string()) }

pub fn decode_vdi3805_dsl(text: &str) -> Result<Vdi3805Snapshot, String> { <Vdi3805Snapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|e| format!("{e:?}")) }

pub fn encode_vdi3805_dsl(snapshot: &Vdi3805Snapshot) -> String { store::ArtifactDsl::print_dsl(snapshot) }
}
pub use snapshot_codec::*;

#[allow(unused_imports)]
mod snapshot_wire_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::*;
use std::collections::BTreeMap;
use crate::{GenericAttributes, CatalogIndex, CharacteristicCurve, EditionId, EditionProfileChoice, ManufacturerCatalog, ManufacturerFile, ParametricGeometry, SecurityLimits};
use ::framework_schema::ArtifactSchema;
use derived_construction::*;
use derived_analysis::*;
use crate::document::{AnnexChoice, ClauseId, LocalizedCopy, NormError, SubjectRef};
/// 📐️ Pure VDI 3805 compliance helpers — native-text parsing with typed record families,
/// Part 1 structural validation, and sheet attribute catalogues.
use crate::*;
use ::dsl;





/// 🔤️ Parse semicolon-delimited native VDI 3805 text into a typed catalogue.
pub fn parse_native_text(text: &str, limits: SecurityLimits) -> Result<ManufacturerCatalog, NormError> {
    limits.validate_text(text)?;
    let mut lines = text.lines().filter(|l| !l.trim().is_empty());
    let header_line = lines.next().ok_or(NormError::IncompleteInput { field: "header".into() })?;
    let header_fields: Vec<&str> = header_line.split(';').collect();
    let (header_version, manufacturer, bsn_raw, created, charset, record_count_raw) = if header_fields.first() == Some(&"010") {
        if header_fields.len() < 6 {
            return Err(NormError::IncompleteInput { field: "header_fields".into() });
        }
        (header_fields[1], header_fields[2], header_fields[3], header_fields.get(4).copied().unwrap_or(""), header_fields.get(5).copied().unwrap_or("UTF-8"), header_fields.get(6).copied().unwrap_or("0"))
    } else {
        if header_fields.len() < 5 {
            return Err(NormError::IncompleteInput { field: "header_fields".into() });
        }
        (header_fields[0], header_fields[1], header_fields[2], header_fields.get(3).copied().unwrap_or(""), "UTF-8", header_fields[4])
    };
    let bsn = BuildingSystemNumber::parse(bsn_raw)?;
    let record_count: u32 = record_count_raw.parse().map_err(|_| NormError::InvalidValue { field: "record_count".into(), reason: "numeric expected".into() })?;
    let mut products: Vec<CatalogueProduct> = Vec::new();
    let mut orphan_records: Vec<NativeRecord> = Vec::new();
    let mut actual_records: u32 = 0;
    for line in lines {
        if actual_records as usize >= limits.max_records {
            return Err(NormError::InvalidValue { field: "records".into(), reason: "too many records".into() });
        }
        let fields: Vec<String> = line.split(';').map(|s| s.to_string()).collect();
        if fields.is_empty() {
            continue;
        }
        if fields[0].len() > limits.max_field_length {
            return Err(NormError::InvalidValue { field: "field_length".into(), reason: "field exceeds limit".into() });
        }
        actual_records += 1;
        let family = RecordFamilyId(fields[0].clone());
        let record = NativeRecord { family: family.clone(), fields: fields.clone(), extensions: ExtensionBag::default() };
        match fields[0].as_str() {
            "100" if fields.len() >= 4 => {
                let article_number = fields.get(3).cloned().unwrap_or_default();
                let identity = ProductIdentity {
                    manufacturer_code: fields.get(1).cloned().unwrap_or_default(),
                    product_group: fields.get(2).cloned().unwrap_or_default(),
                    article_number: article_number.clone(),
                };
                let sheet_no: u16 = fields.get(4).and_then(|s| s.parse().ok()).unwrap_or(2);
                products.push(CatalogueProduct {
                    id: article_number.clone(),
                    identity,
                    title: bilingual("Produkt", "Product"),
                    sheet: SheetId(sheet_no),
                    records: vec![record],
                    configuration: Configuration {
                        id: format!("cfg.{}", article_number),
                        attributes: SheetAttributes::Generic(GenericAttributes::default()),
                        geometry_ref: None,
                        function_refs: Vec::new(),
                    },
                    accessories: Vec::new(),
                    components: Vec::new(),
                    extensions: ExtensionBag::default(),
                });
            }
            "110" => {
                if let Some(product) = products.last_mut() {
                    if let Some(cfg_id) = fields.get(1) {
                        product.configuration.id = cfg_id.clone();
                    }
                    if let Some(geom) = fields.get(2).filter(|s| !s.is_empty()) {
                        product.configuration.geometry_ref = Some(geom.clone());
                    }
                    product.configuration.function_refs = fields.iter().skip(3).filter(|s| !s.is_empty()).cloned().collect();
                    product.records.push(record);
                } else {
                    orphan_records.push(record);
                }
            }
            "200" | "210" | "400" | "700" | "900" => {
                if let Some(product) = products.last_mut() {
                    product.records.push(record);
                } else {
                    orphan_records.push(record);
                }
            }
            _ => {
                if let Some(product) = products.last_mut() {
                    product.records.push(record);
                } else {
                    orphan_records.push(record);
                }
            }
        }
    }
    for product in &mut products {
        product.configuration.attributes = attributes_from_records(product.sheet, &product.records);
        if let Some(de) = product.records.iter().find(|r| r.family.0 == "700").and_then(|r| r.fields.get(2)) {
            let en = product.records.iter().find(|r| r.family.0 == "700").and_then(|r| r.fields.get(4)).cloned().unwrap_or_else(|| de.clone());
            product.title = bilingual(de.clone(), en);
        }
    }
    if !orphan_records.is_empty() && products.is_empty() {
        return Err(NormError::IncompleteInput { field: "100".into() });
    }
    if actual_records != record_count {
        return Err(NormError::InvalidValue {
            field: "record_count".into(),
            reason: format!("declared {record_count} != actual {actual_records}"),
        });
    }
    let file = ManufacturerFile {
        header_version: header_version.into(),
        manufacturer: manufacturer.into(),
        building_system_number: bsn,
        created: created.into(),
        charset: charset.into(),
        record_count,
        extensions: ExtensionBag::default(),
    };
    Ok(ManufacturerCatalog { file, products, extensions: ExtensionBag::default() })
}
}
pub use snapshot_wire_codec::*;


#[allow(unused_imports)]
mod snapshot_wire_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::snapshot::*;
use crate::{CatalogIndex, CharacteristicCurve, EditionId, EditionProfileChoice, ManufacturerCatalog, ParametricGeometry, SecurityLimits};
use framework_schema::ArtifactSchema;
use std::collections::BTreeMap;




}
pub use snapshot_wire_codec::*;

#[allow(unused_imports)]
mod snapshot_wire2_codec {
use crate::standards::v1::subsets::any::schema::*;
use std::collections::BTreeMap;
use crate::{GenericAttributes, CatalogIndex, CharacteristicCurve, EditionId, EditionProfileChoice, ManufacturerCatalog, ManufacturerFile, ParametricGeometry, SecurityLimits};
use ::framework_schema::ArtifactSchema;
use crate::document::{AnnexChoice, ClauseId, LocalizedCopy, NormError, SubjectRef};
/// 📐️ Pure VDI 3805 compliance helpers — native-text parsing with typed record families,
/// Part 1 structural validation, and sheet attribute catalogues.
use crate::*;
use ::dsl;

/// 🔤️ Serialize catalogue to semicolon-delimited native text (010-style header + typed records).
pub fn serialize_native_text(catalog: &ManufacturerCatalog) -> String {
    let f = &catalog.file;
    let mut body = String::new();
    let mut line_count: u32 = 0;
    for product in &catalog.products {
        let mut wrote_100 = false;
        for record in &product.records {
            body.push_str(&record.fields.join(";"));
            body.push('\n');
            line_count += 1;
            if record.fields.first().is_some_and(|f| f == "100") {
                wrote_100 = true;
            }
        }
        if !wrote_100 {
            body.push_str(&format!(
                "100;{};{};{};{}\n",
                product.identity.manufacturer_code,
                product.identity.product_group,
                product.identity.article_number,
                product.sheet.0
            ));
            line_count += 1;
        }
        let has_210 = product.records.iter().any(|r| r.fields.first().is_some_and(|f| f == "210"));
        if !has_210 {
            if let Some(line) = attributes_to_native_line(&product.configuration.attributes) {
                body.push_str(&line);
                body.push('\n');
                line_count += 1;
            }
        }
    }
    format!(
        "010;{};{};{};{};{};{}\n{}",
        f.header_version,
        f.manufacturer,
        f.building_system_number.render(),
        f.created,
        f.charset,
        line_count,
        body
    )
}

pub(crate) fn attributes_to_native_fields(attributes: &SheetAttributes) -> Option<Vec<(String, String)>> {
    match attributes {
        SheetAttributes::ValveHeating(a) => Some(vec![
            ("dn".into(), a.dn.to_string()),
            ("kvs".into(), format!("{:.9}", a.kvs_m3_h())),
            ("pressure_class".into(), a.pressure_class.clone()),
            ("connection_type".into(), a.connection_type.clone()),
            ("authority_min".into(), format!("{}", a.authority_min)),
            ("authority_max".into(), format!("{}", a.authority_max)),
        ]),
        SheetAttributes::Radiator(a) => Some(vec![
            ("standard_output_w".into(), format!("{}", a.standard_output_w)),
            ("n".into(), format!("{}", a.heat_exponent_n)),
            ("length_m".into(), format!("{}", a.length_m)),
            ("height_m".into(), format!("{}", a.height_m)),
            ("depth_m".into(), format!("{}", a.depth_m)),
            ("connection_type".into(), a.connection_type.clone()),
        ]),
        SheetAttributes::PumpHeating(a) => Some(vec![
            ("dn_suction".into(), a.dn_suction.to_string()),
            ("dn_discharge".into(), a.dn_discharge.to_string()),
            ("q".into(), format!("{}", a.nominal_flow_m3_s)),
            ("h".into(), format!("{}", a.nominal_head_m)),
            ("p".into(), format!("{}", a.motor_power_w)),
            ("eta".into(), format!("{}", a.hydraulic_efficiency)),
        ]),
        SheetAttributes::HeatGenerator(a) => Some(vec![
            ("qn".into(), format!("{}", a.nominal_heat_output_w)),
            ("fuel_type".into(), a.fuel_type.clone()),
            ("flow_temp_max_c".into(), format!("{}", a.flow_temp_max_c)),
            ("return_temp_min_c".into(), format!("{}", a.return_temp_min_c)),
        ]),
        SheetAttributes::Generic(_) => None,
    }
}

pub(crate) fn attributes_to_native_line(attributes: &SheetAttributes) -> Option<String> {
    let fields = attributes_to_native_fields(attributes)?;
    let mut line = String::from("210");
    for (k, v) in fields {
        line.push(';');
        line.push_str(&k);
        line.push(';');
        line.push_str(&v);
    }
    Some(line)
}

/// 🧪 Round-trip helpers retained for tests (not emitted into the user-facing report).
pub fn assert_json_round_trip(catalog: &ManufacturerCatalog) -> Result<(), String> {
    let json = crate::standards::v1::subsets::any::io::catalog_to_json(catalog).map_err(|e| e.to_string())?;
    let restored = crate::standards::v1::subsets::any::io::catalog_from_json(&json).map_err(|e| e.to_string())?;
    if restored.products.len() != catalog.products.len() {
        return Err("catalog JSON product count mismatch".into());
    }
    Ok(())
}

pub fn assert_native_round_trip(catalog: &ManufacturerCatalog, limits: SecurityLimits) -> Result<(), String> {
    let native = serialize_native_text(catalog);
    let parsed = parse_native_text(&native, limits).map_err(|e| e.to_string())?;
    if parsed.products.len() != catalog.products.len() {
        return Err("native text product count mismatch".into());
    }
    Ok(())
}
}
pub use snapshot_wire2_codec::*;
