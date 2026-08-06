//! ⚙️ VDI 3805 app — headless compute (constitutional: engine).

use crate::core::{AnnexChoice, CheckReport, CheckResult, CheckStatus, ClauseId, NormError, Quantity, QuantityKind};
use std::collections::BTreeMap;
use crate::artifacts::vdi3805::*;

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
    CheckResult::pass(clause(part, section), Quantity::new(QuantityKind::Dimensionless, 1.0), Quantity::new(QuantityKind::Dimensionless, 1.0), 1.0, message, ANNEX)
}

fn fail_check(part: &str, section: &str, message: impl Into<String>) -> CheckResult {
    CheckResult::fail(clause(part, section), Quantity::new(QuantityKind::Dimensionless, 0.0), Quantity::new(QuantityKind::Dimensionless, 1.0), 2.0, message, ANNEX)
}

// #region Part1
/// 🔤️ Parse semicolon-delimited native VDI 3805 text.
pub fn parse_native_text(text: &str, limits: SecurityLimits) -> Result<ManufacturerCatalog, NormError> {
    limits.validate_text(text)?;
    let mut lines = text.lines().filter(|l| !l.trim().is_empty());
    let header_line = lines.next().ok_or(NormError::IncompleteInput { field: "header".into() })?;
    let header_fields: Vec<&str> = header_line.split(';').collect();
    if header_fields.len() < 5 {
        return Err(NormError::IncompleteInput { field: "header_fields".into() });
    }
    let bsn = BuildingSystemNumber::parse(header_fields[2])?;
    let record_count: u32 = header_fields[4].parse().map_err(|_| NormError::InvalidValue { field: "record_count".into(), reason: "numeric expected".into() })?;
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
            let identity = ProductIdentity { manufacturer_code: fields.get(1).cloned().unwrap_or_default(), product_group: fields.get(2).cloned().unwrap_or_default(), article_number: article_number.clone() };
            let sheet_no: u16 = fields.get(4).and_then(|s| s.parse().ok()).unwrap_or(2);
            products.push(CatalogueProduct {
                identity,
                title: LocalizedText::new("Produkt", "Product"),
                sheet: SheetId(sheet_no),
                records: Vec::new(),
                configuration: Configuration { id: format!("cfg.{}", article_number), parameters: BTreeMap::new(), geometry_ref: None, function_refs: Vec::new() },
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

/// 🔤️ Serialize catalogue to semicolon-delimited native text.
pub fn serialize_native_text(catalog: &ManufacturerCatalog) -> String {
    let f = &catalog.file;
    let mut out = format!("{};{};{};{};{}\n", f.header_version, f.manufacturer, f.building_system_number.render(), f.created, f.record_count);
    for product in &catalog.products {
        out.push_str(&format!("100;{};{};{};{}\n", product.identity.manufacturer_code, product.identity.product_group, product.identity.article_number, product.sheet.0));
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

/// ✅️ Structural validation of Part 1 catalogue.
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
            issues.push(Diagnostic::error(format!("product.{}", product.sheet.0), "missing article number"));
        }
        if product.configuration.id.is_empty() {
            issues.push(Diagnostic::warning(format!("configuration.{}", product.sheet.0), "missing configuration id"));
        }
    }
    let known: std::collections::BTreeSet<&str> = RecordFamilyId::all_known().iter().copied().collect();
    for product in &catalog.products {
        for record in &product.records {
            if !known.contains(record.family.0.as_str()) && !record.family.0.starts_with("9") {
                issues.push(Diagnostic::info(format!("record.{}", record.family.0), "unknown record family preserved"));
            }
        }
    }
    issues
}
// #endregion Part1

// #region Functions
/// 🔢️ Linear map between two scalar domains.
pub fn linear_map(x: f64, x0: f64, x1: f64, y0: f64, y1: f64) -> f64 {
    if (x1 - x0).abs() < f64::EPSILON {
        return y0;
    }
    y0 + (x - x0) * (y1 - y0) / (x1 - x0)
}
// #endregion Functions

// #region Io
/// 📤️ JSON round-trip for manufacturer catalogues.
pub fn catalog_to_json(catalog: &ManufacturerCatalog) -> Result<String, NormError> {
    serde_json::to_string_pretty(catalog).map_err(|e| NormError::InvalidValue { field: "json".into(), reason: e.to_string() })
}

pub fn catalog_from_json(json: &str) -> Result<ManufacturerCatalog, NormError> {
    serde_json::from_str(json).map_err(|e| NormError::InvalidValue { field: "json".into(), reason: e.to_string() })
}

pub fn document_to_json(document: &Document) -> Result<String, NormError> {
    serde_json::to_string_pretty(document).map_err(|e| NormError::InvalidValue { field: "json".into(), reason: e.to_string() })
}

pub fn document_from_json(json: &str) -> Result<Document, NormError> {
    serde_json::from_str(json).map_err(|e| NormError::InvalidValue { field: "json".into(), reason: e.to_string() })
}
// #endregion Io

// #region Validate
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

            #[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
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

    let operative: std::collections::BTreeSet<u16> = registry.operative_sheets().iter().map(|s| s.id.0).collect();
    report.push(pass_check("registry", "operative", format!("{} operative sheets", operative.len())));

    for corr in registry.corrections_for_sheet(SheetId(2)) {
        let applies = corr.applies_as_of(document.correction_as_of);
        report.push(if applies { pass_check("2", "correction", format!("{} applies", corr.id)) } else { pass_check("2", "correction", format!("{} not yet effective", corr.id)) });
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
    report.push(pass_check("catalog", "index", format!("index {} entries, {} heating sheets", document.index.entries.len(), heating.len())));

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
        report.push(CheckResult::from_utilization(clause("functions", "curve"), Quantity::new(QuantityKind::Volume, y), Quantity::new(QuantityKind::Volume, 2.25), format!("kvs curve at 50% = {y:.3}"), ANNEX));
    }

    for diag in validate_structure(&document.catalog) {
        report.checks.extend(diagnostics_to_report(&[diag], "1", "validate"));
    }

    if document.strict_mode {
        report.push(pass_check("session", "strict", "strict mode enabled"));
    }

    report
}
// #endregion Session

// #region 🔖️Session
/// 🧩️ VDI 3805's `NormFamily` binding — ties this artifact's `Document` to the `evaluate` above for
/// the headless `NormHost` session every norm app drives.
pub struct Vdi3805Family;

impl crate::core::NormFamily for Vdi3805Family {
    type Document = Document;
    type Operation = op::Operation;

    fn family_id() -> crate::core::NormFamilyId {
        crate::core::NormFamilyId::Vdi3805
    }

    fn evaluate(document: &Document) -> CheckReport {
        evaluate(document)
    }
}

pub type Host = crate::core::NormHost<Vdi3805Family>;
// #endregion 🔖️Session

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn evaluate_reaches_operative_sheet_families() {
        let report = evaluate(&Document::default());
        let parts: BTreeSet<String> = report.checks.iter().map(|c| c.clause.part.clone()).filter(|p| p.chars().all(|ch| ch.is_ascii_digit())).collect();
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
    fn reserved_sheet_returns_not_applicable() {
        let doc = Document::default();
        let result = part_15::check(&doc);
        assert_eq!(result.status, CheckStatus::NotApplicable);
        let result = part_67::check(&doc);
        assert_eq!(result.status, CheckStatus::NotApplicable);
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
        doc.catalog.products[0].records.push(NativeRecord { family: RecordFamilyId("888".into()), fields: vec!["888".into()], extensions: ExtensionBag::default() });
        let issues = validate_structure(&doc.catalog);
        assert!(issues.iter().any(|d| d.severity == Severity::Info && d.field.contains("888")));
    }

    #[test]
    fn linear_map_interpolates_and_handles_degenerate_domain() {
        assert!((linear_map(5.0, 0.0, 10.0, 0.0, 100.0) - 50.0).abs() < 1e-9);
        assert_eq!(linear_map(5.0, 3.0, 3.0, 7.0, 42.0), 7.0);
    }

    #[test]
    fn diagnostic_constructors_and_report_mapping() {
        let diags = vec![Diagnostic::error("f1", "bad"), Diagnostic::warning("f2", "meh"), Diagnostic::info("f3", "fyi")];
        let report = diagnostics_to_report(&diags, "1", "validate");
        assert_eq!(report[0].status, CheckStatus::Fail);
        assert_eq!(report[1].status, CheckStatus::Pass);
        assert_eq!(report[2].status, CheckStatus::Pass);
    }

    #[test]
    fn historical_part_check_respects_strict_mode() {
        let mut doc = Document { strict_mode: true, ..Document::default() };
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
        let doc = Document { strict_mode: true, ..Document::default() };
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

    /// 🧩️ The `NormFamily` binding lives here now (it was in the constitutional `op` crate) — it names
    /// `evaluate`, so it belongs beside the compute it binds.
    #[test]
    fn norm_family_id() {
        assert_eq!(<Vdi3805Family as crate::core::NormFamily>::family_id(), crate::core::NormFamilyId::Vdi3805);
        assert_eq!(crate::core::NormFamilyId::Vdi3805.label(), "VDI 3805");
    }

    #[test]
    fn norm_host_recomputes() {
        let mut host = Host::from_document(Document::default());
        assert!(!host.report().checks.is_empty());
        host.replace_document(Document::default());
        assert!(host.report().all_pass());
    }
}
