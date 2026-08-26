//! 🧬️ Vdi3805 artifact schema — every field of the artifact with its state class.

use std::collections::BTreeMap;

use crate::artifacts::vdi3805::{CatalogIndex, CharacteristicCurve, EditionId, EditionProfileChoice, ManufacturerCatalog, ManufacturerFile, ParametricGeometry, SecurityLimits};
use ::schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full Vdi3805 artifact state across the artifact and presence lanes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.vdi3805")]
pub struct Vdi3805Artifact {
    #[state(artifact)]
    pub manufacturer_file: ManufacturerFile,
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
    #[state(presence)]
    pub selected_check_index: Option<u32>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Vdi3805Artifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> Vdi3805Snapshot {
        Vdi3805Snapshot {
            manufacturer_file: self.manufacturer_file.clone(),
            catalog: self.catalog.clone(),
            edition_profile: self.edition_profile.clone(),
            correction_as_of: self.correction_as_of.clone(),
            strict_mode: self.strict_mode,
            index: self.index.clone(),
            geometry: self.geometry.clone(),
            curves: self.curves.clone(),
            limits: self.limits.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: Vdi3805Snapshot) -> Self {
        Self {
            manufacturer_file: snapshot.manufacturer_file,
            catalog: snapshot.catalog,
            edition_profile: snapshot.edition_profile,
            correction_as_of: snapshot.correction_as_of,
            strict_mode: snapshot.strict_mode,
            index: snapshot.index,
            geometry: snapshot.geometry,
            curves: snapshot.curves,
            limits: snapshot.limits,
            selected_check_index: None,
        }
    }
    /// 🔄 Overwrite persistent fields from a snapshot; leave shared-ui untouched.
    pub fn set_snapshot(&mut self, snapshot: Vdi3805Snapshot) {
        let selected = self.selected_check_index;
        *self = Self::from_snapshot(snapshot);
        self.selected_check_index = selected;
    }
}

//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.norm.vdi3805` — twenty handcrafted schema leaves.
pub fn vdi3805_artifact_schema_descriptor() -> ::schema::ArtifactSchemaDescriptor {
    ::schema::ArtifactSchemaDescriptor {
        id: "s.norm.vdi3805",
        artifact: ::schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
        snapshot: ::schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️component.rs"),
            typescript: include_str!("📸️snapshot/🟦️component.ts"),
            graphql: include_str!("📸️snapshot/🔗️component.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️component.json"),
            proto: include_str!("📸️snapshot/🛰️component.proto"),
        },
        diff: ::schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️component.rs"),
            typescript: include_str!("🔺️diff/🟦️component.ts"),
            graphql: include_str!("🔺️diff/🔗️component.graphql"),
            json_schema: include_str!("🔺️diff/🔣️component.json"),
            proto: include_str!("🔺️diff/🛰️component.proto"),
        },
        mutations: ::schema::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️component.rs"),
            typescript: include_str!("🧬️mutations/🟦️component.ts"),
            graphql: include_str!("🧬️mutations/🔗️component.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️component.json"),
            proto: include_str!("🧬️mutations/🛰️component.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use crate::artifacts::vdi3805::{Vdi3805Diff, Vdi3805Mutation, Vdi3805Snapshot};
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct Vdi3805BuilderConstruction {
        snapshot: Vdi3805Snapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for Vdi3805BuilderConstruction {
        type Snapshot = Vdi3805Snapshot;
        type Mutation = Vdi3805Mutation;
        type Diff = Vdi3805Diff;
        fn empty() -> Self {
            Self { snapshot: Vdi3805Snapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<Vdi3805Snapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<Vdi3805Snapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let outcome = <Vdi3805Mutation as protocol::Mutation<Vdi3805Snapshot>>::diff(&mutation, &self.snapshot);
            match <Self::Diff as protocol::MutationDiff<Self::Snapshot>>::apply(outcome.diff(), &self.snapshot) {
                Ok(snapshot) => self.snapshot = snapshot,
                Err(error) => self.diagnostics.push(dsl::Diagnostic::error("mutation.apply", dsl::TextSpan::at(1, 1), error.to_string())),
            }
            (self, outcome)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            let snapshot = <Vdi3805Diff as protocol::MutationDiff<Vdi3805Snapshot>>::apply(&diff, &self.snapshot)?;
            self.snapshot = snapshot;
            Ok(self)
        }
        fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
            if self.diagnostics.is_empty() {
                Ok(self.snapshot)
            } else {
                Err(self.diagnostics)
            }
        }
    }
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use crate::artifacts::vdi3805::Vdi3805Snapshot;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct Vdi3805Parts {
        pub snapshot: Option<Vdi3805Snapshot>,
    }

    pub struct Vdi3805AnalyzerAnalysis;

    impl ArtifactAnalysis for Vdi3805AnalyzerAnalysis {
        type Parts = Vdi3805Parts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.vdi3805", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = Vdi3805Parts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <Vdi3805Snapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <Vdi3805Snapshot as store::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.binary", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                }
            }
            Analysis { parts, dialect: Self::DIALECT, confidence, diagnostics }
        }
    }
}
pub use derived_analysis::*;
//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec Vdi3805BuilderFacets {
        construction: Vdi3805BuilderConstruction,
        analysis: Vdi3805AnalyzerAnalysis,
        composition: super::super::io::derived_composition::Vdi3805ComposerComposition,
    }
    builder: Vdi3805Builder,
    analyzer: Vdi3805Analyzer,
    composer: Vdi3805Composer,
);
//#endregion 🧬️DerivedArtifactFacets

//#region 🔖️ComplianceHelpers
/// 📐️ Pure VDI 3805 compliance helpers (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES)
/// — relocated verbatim from the deleted `⚙️engine`. Native-text parsing/serialization, structural
/// validation, the linear-map utility and diagnostic-to-check mapping are all pure helpers over
/// document types (`ManufacturerCatalog`, `Diagnostic`, …), never over the whole `Vdi3805Snapshot`.
/// `clause`/`na_check`/`pass_check`/`fail_check` are shared with `💡️inferences`'s per-sheet
/// conformance laws (99 `part_N::check(&Vdi3805Snapshot)` functions, which — unlike the Eurocode
/// artifacts' `part_N` modules — take the whole snapshot directly, so they live in `inferences` not
/// here). The whole-artifact JSON (de)serializers live in `🚪️io`.
use crate::artifacts::vdi3805::*;
use crate::document::{AnnexChoice, CheckResult, CheckStatus, ClauseId, NormError, Quantity, QuantityKind};
// 🔀️ Explicit single-item import: the glob above also pulls in `crate::artifacts::vdi3805::dsl`
// (the mounted native-text grammar submodule, see `📦️glue.rs`), which would otherwise shadow the
// `extern crate semio_framework_os_kernel as dsl;` alias for every unqualified `dsl::…` path in this
// module — including the one `derive_artifact_facets!` (below) expands to. An explicit `use` always
// wins over a glob import for the same name, so this restores `dsl` to the intended crate alias.
use ::dsl;

const FAMILY: &str = "VDI 3805";
pub const ANNEX: AnnexChoice = AnnexChoice::De;

pub fn clause(part: &str, section: &str) -> ClauseId {
    ClauseId::new(FAMILY, part, section)
}

pub fn na_check(part: &str, section: &str, message: impl Into<String>) -> CheckResult {
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

pub fn pass_check(part: &str, section: &str, message: impl Into<String>) -> CheckResult {
    CheckResult::pass(clause(part, section), Quantity::new(QuantityKind::Dimensionless, 1.0), Quantity::new(QuantityKind::Dimensionless, 1.0), 1.0, message, ANNEX)
}

pub fn fail_check(part: &str, section: &str, message: impl Into<String>) -> CheckResult {
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
                title: bilingual("Produkt", "Product"),
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
//#endregion 🔖️ComplianceHelpers

//#region 🧪️ComplianceHelpersTests
#[cfg(test)]
mod compliance_helpers_tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    fn native_text_round_trip() {
        let doc = Vdi3805Snapshot::default();
        let text = serialize_native_text(&doc.catalog);
        let parsed = parse_native_text(&text, SecurityLimits::default()).expect("parse");
        assert_eq!(parsed.products.len(), doc.catalog.products.len());
        assert_eq!(parsed.file.manufacturer, doc.catalog.file.manufacturer);
    }

    #[semio_framework_async_macros::async_test]
    fn parse_native_text_rejects_empty_input() {
        let err = parse_native_text("", SecurityLimits::default()).unwrap_err();
        assert!(matches!(err, NormError::IncompleteInput { field } if field == "header"));
    }

    #[semio_framework_async_macros::async_test]
    fn parse_native_text_rejects_incomplete_header() {
        let err = parse_native_text("3805;DEMO;420.10.1\n", SecurityLimits::default()).unwrap_err();
        assert!(matches!(err, NormError::IncompleteInput { field } if field == "header_fields"));
    }

    #[semio_framework_async_macros::async_test]
    fn parse_native_text_rejects_invalid_building_system_number() {
        let err = parse_native_text("3805;DEMO;bad;2026-07-22;3\n", SecurityLimits::default()).unwrap_err();
        assert!(matches!(err, NormError::InvalidValue { field, .. } if field == "building_system_number"));
    }

    #[semio_framework_async_macros::async_test]
    fn parse_native_text_rejects_non_numeric_record_count() {
        let err = parse_native_text("3805;DEMO;420.10.1;2026-07-22;abc\n", SecurityLimits::default()).unwrap_err();
        assert!(matches!(err, NormError::InvalidValue { field, .. } if field == "record_count"));
    }

    #[semio_framework_async_macros::async_test]
    fn parse_native_text_rejects_too_many_records() {
        let limits = SecurityLimits { max_records: 0, ..SecurityLimits::default() };
        let text = "3805;DEMO;420.10.1;2026-07-22;1\n200;dn;50\n";
        let err = parse_native_text(text, limits).unwrap_err();
        assert!(matches!(err, NormError::InvalidValue { field, .. } if field == "records"));
    }

    #[semio_framework_async_macros::async_test]
    fn parse_native_text_parses_product_records() {
        let text = "3805;DEMO;420.10.1;2026-07-22;1\n100;DEMO;HV;VLV-1;2\n200;dn;50\n";
        let parsed = parse_native_text(text, SecurityLimits::default()).expect("parse");
        assert_eq!(parsed.products.len(), 1);
        assert_eq!(parsed.products[0].identity.article_number, "VLV-1");
        assert_eq!(parsed.products[0].sheet, SheetId(2));
    }

    #[semio_framework_async_macros::async_test]
    fn validate_structure_reports_missing_manufacturer() {
        let mut doc = Vdi3805Snapshot::default();
        doc.catalog.file.manufacturer = String::new();
        let issues = validate_structure(&doc.catalog);
        assert!(issues.iter().any(|d| d.field == "manufacturer" && d.severity == Severity::Error));
    }

    #[semio_framework_async_macros::async_test]
    fn validate_structure_reports_empty_products() {
        let mut doc = Vdi3805Snapshot::default();
        doc.catalog.products.clear();
        let issues = validate_structure(&doc.catalog);
        assert!(issues.iter().any(|d| d.field == "products" && d.severity == Severity::Warning));
    }

    #[semio_framework_async_macros::async_test]
    fn validate_structure_reports_missing_article_number_and_config_id() {
        let mut doc = Vdi3805Snapshot::default();
        doc.catalog.products[0].identity.article_number = String::new();
        doc.catalog.products[0].configuration.id = String::new();
        let issues = validate_structure(&doc.catalog);
        assert!(issues.iter().any(|d| d.severity == Severity::Error && d.field.starts_with("product.")));
        assert!(issues.iter().any(|d| d.severity == Severity::Warning && d.field.starts_with("configuration.")));
    }

    #[semio_framework_async_macros::async_test]
    fn validate_structure_reports_unknown_record_family() {
        let mut doc = Vdi3805Snapshot::default();
        doc.catalog.products[0].records.push(NativeRecord { family: RecordFamilyId("888".into()), fields: vec!["888".into()], extensions: ExtensionBag::default() });
        let issues = validate_structure(&doc.catalog);
        assert!(issues.iter().any(|d| d.severity == Severity::Info && d.field.contains("888")));
    }

    #[semio_framework_async_macros::async_test]
    fn linear_map_interpolates_and_handles_degenerate_domain() {
        assert!((linear_map(5.0, 0.0, 10.0, 0.0, 100.0) - 50.0).abs() < 1e-9);
        assert_eq!(linear_map(5.0, 3.0, 3.0, 7.0, 42.0), 7.0);
    }

    #[semio_framework_async_macros::async_test]
    fn diagnostic_constructors_and_report_mapping() {
        let diags = vec![Diagnostic::error("f1", "bad"), Diagnostic::warning("f2", "meh"), Diagnostic::info("f3", "fyi")];
        let report = diagnostics_to_report(&diags, "1", "validate");
        assert_eq!(report[0].status, CheckStatus::Fail);
        assert_eq!(report[1].status, CheckStatus::Pass);
        assert_eq!(report[2].status, CheckStatus::Pass);
    }
}
//#endregion 🧪️ComplianceHelpersTests
