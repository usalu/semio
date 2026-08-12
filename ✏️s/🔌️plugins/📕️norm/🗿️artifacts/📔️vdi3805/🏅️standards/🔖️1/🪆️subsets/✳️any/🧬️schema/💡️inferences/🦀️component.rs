//! 💡️ Vdi3805 inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧾outline/`).

use crate::artifacts::vdi3805::Vdi3805Snapshot;
use ::schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::outline::Vdi3805Outline;

//#region 🔖️Inference
/// 💡️ Everything inferable from a vdi3805 snapshot. One field per named inference under
/// `💡️inferences/` (currently: `outline`, backed by the `🧾outline/` slug dir — this document's
/// own field/section structure, since a norm compliance record IS the document it describes).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.vdi3805.inference")]
pub struct Vdi3805Inference {
    #[state(inferred)]
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
        &[protocol::InferenceFieldSpec { id: "s.norm.vdi3805.inference.outline", reads: &["edition_profile", "geometry", "curves"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::artifacts::vdi3805::standards::v1::subsets::any::schema::Vdi3805Builder {
    type Snapshot = Vdi3805Snapshot;
    type Inference = Vdi3805Inference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.norm.vdi3805.inference`'s facet leaves into the OS-wide inference catalog — call once at
/// plugin init, alongside `vdi3805_artifact_schema_descriptor`'s registration.
pub fn vdi3805_artifact_inference_descriptor() -> ::schema::ArtifactInferenceDescriptor {
    ::schema::ArtifactInferenceDescriptor {
        id: "s.norm.vdi3805.inference",
        inference: ::schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
    }
}
//#endregion 🔖️Descriptor

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use protocol::Inference;

    #[test]
    fn inference_determinism_law() {
        let snapshot = Vdi3805Snapshot::default();
        assert_eq!(Vdi3805Inference::infer(&snapshot), Vdi3805Inference::infer(&snapshot));
    }

    #[test]
    fn inference_default_law() {
        assert_eq!(Vdi3805Inference::infer(&Vdi3805Snapshot::default()), Vdi3805Inference::default());
    }
}
//#endregion 🧪️Tests

//#region 🔖️ComplianceReport
/// 📋️ Full VDI 3805 compliance-report conformance law (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — relocated verbatim from the deleted
/// `⚙️engine`. Unlike the Eurocode artifacts, every `part_N::check` here takes the whole
/// `Vdi3805Snapshot` directly (99 macro-generated per-sheet conformance laws), so the entire
/// `SheetParts`/`Session` machinery lives here rather than in `🧬️schema`. `clause`/`na_check`/
/// `pass_check`/`fail_check`/`validate_structure`/`diagnostics_to_report` are pure helpers imported
/// from the parent `🧬️schema`; the JSON (de)serializers come from `🚪️io`.
use crate::artifacts::vdi3805::*;
use crate::document::{AnnexChoice, CheckReport, CheckResult, CheckStatus, ClauseId, NormError, Quantity, QuantityKind};
use crate::artifacts::vdi3805::standards::v1::subsets::any::schema::{clause, diagnostics_to_report, fail_check, na_check, pass_check, validate_structure, parse_native_text, serialize_native_text, ANNEX};
use crate::artifacts::vdi3805::standards::v1::subsets::any::io::{catalog_from_json, catalog_to_json};

// #region SheetParts
macro_rules! define_vdi_part {
    ($module:ident, $num:literal, reserved) => {
        pub mod $module {
            use super::*;

            pub fn metadata() -> &'static SheetEntry {
                &SHEET_ENTRIES[$num - 1]
            }

            pub fn check(document: &Vdi3805Snapshot) -> CheckResult {
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

            pub fn check(document: &Vdi3805Snapshot) -> CheckResult {
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

            pub fn check(document: &Vdi3805Snapshot) -> CheckResult {
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

            pub fn check(document: &Vdi3805Snapshot) -> CheckResult {
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

    pub fn check(document: &Vdi3805Snapshot) -> CheckResult {
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
fn all_part_checks(document: &Vdi3805Snapshot) -> Vec<CheckResult> {
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

pub fn evaluate(document: &Vdi3805Snapshot) -> CheckReport {
    let mut report = CheckReport::default();

    for check in all_part_checks(document) {
        report.push(check);
    }

    let registry = SchemaCatalog::current();
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
//#endregion 🔖️ComplianceReport

//#region 🧪️ComplianceReportTests
#[cfg(test)]
mod compliance_report_tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn evaluate_reaches_operative_sheet_families() {
        let report = evaluate(&Vdi3805Snapshot::default());
        let parts: BTreeSet<String> = report.checks.iter().map(|c| c.clause.part.clone()).filter(|p| p.chars().all(|ch| ch.is_ascii_digit())).collect();
        let registry = SchemaCatalog::current();
        for sheet in registry.operative_sheets() {
            let part = sheet.id.0.to_string();
            if sheet.status == SchemaStatus::Reserved {
                continue;
            }
            assert!(parts.contains(&part), "missing checks for sheet {part}");
        }
    }

    #[test]
    fn reserved_sheet_returns_not_applicable() {
        let doc = Vdi3805Snapshot::default();
        let result = part_15::check(&doc);
        assert_eq!(result.status, CheckStatus::NotApplicable);
        let result = part_67::check(&doc);
        assert_eq!(result.status, CheckStatus::NotApplicable);
    }

    #[test]
    fn historical_part_check_respects_strict_mode() {
        let mut doc = Vdi3805Snapshot { strict_mode: true, ..Vdi3805Snapshot::default() };
        let result = part_12::check(&doc);
        assert_eq!(result.status, CheckStatus::Fail);

        doc.strict_mode = false;
        let result = part_12::check(&doc);
        assert_eq!(result.status, CheckStatus::Pass);
    }

    #[test]
    fn multi_profile_part_check_reports_metadata_when_no_product() {
        let doc = Vdi3805Snapshot::default();
        let result = part_08::check(&doc);
        assert_eq!(result.status, CheckStatus::Pass);
    }

    #[test]
    fn evaluate_reports_strict_mode_check() {
        let doc = Vdi3805Snapshot { strict_mode: true, ..Vdi3805Snapshot::default() };
        let report = evaluate(&doc);
        assert!(report.checks.iter().any(|c| c.clause.section == "strict"));
    }

    #[test]
    fn evaluate_skips_geometry_and_curve_checks_when_absent() {
        let mut doc = Vdi3805Snapshot::default();
        doc.geometry.clear();
        doc.curves.clear();
        let report = evaluate(&doc);
        assert!(!report.checks.iter().any(|c| c.clause.part == "geometry"));
        assert!(!report.checks.iter().any(|c| c.clause.part == "functions"));
    }
}
//#endregion 🧪️ComplianceReportTests

