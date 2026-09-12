//! 🧬️ Vdi3805 artifact schema — every field of the artifact with its state class.

use std::collections::BTreeMap;

use crate::{CatalogIndex, CharacteristicCurve, EditionId, EditionProfileChoice, ManufacturerCatalog, ManufacturerFile, ParametricGeometry, SecurityLimits};
use ::framework_schema::ArtifactSchema;

//#region 🔖️Artifact
/// 🧬️ Full Vdi3805 artifact state across the artifact and presence lanes.
#[derive(Clone, Debug, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
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
            correction_as_of: self.correction_as_of,
            strict_mode: self.strict_mode,
            index: self.index.clone(),
            geometry: self.geometry.clone(),
            curves: self.curves.clone(),
            limits: self.limits,
        }
    }

    /// 🧬️ Builds the document artifact from its snapshot.
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
        }
    }
    /// 🔄 Overwrite persistent fields from a snapshot; leave shared-ui untouched.
    pub fn set_snapshot(&mut self, snapshot: Vdi3805Snapshot) {
        *self = Self::from_snapshot(snapshot);
    }
}

//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.norm.vdi3805` — twenty handcrafted schema leaves.
pub fn vdi3805_artifact_schema_descriptor() -> ::framework_schema::ArtifactSchemaDescriptor {
    ::framework_schema::ArtifactSchemaDescriptor {
        id: "s.norm.vdi3805",
        artifact: ::framework_schema::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
        snapshot: ::framework_schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: ::framework_schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: ::framework_schema::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️.rs"),
            typescript: include_str!("🧬️mutations/🟦️.ts"),
            graphql: include_str!("🧬️mutations/🔗️.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️.json"),
            proto: include_str!("🧬️mutations/🛰️.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use crate::{Vdi3805Diff, Vdi3805Mutation, Vdi3805Snapshot};
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
    use crate::Vdi3805Snapshot;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct Vdi3805Parts {
        pub snapshot: Option<Vdi3805Snapshot>,
    }

    pub struct Vdi3805AnalyzerAnalysis;

    impl ArtifactAnalysis for Vdi3805AnalyzerAnalysis {
        type Parts = Vdi3805Parts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.norm.vdi3805", standard: StandardId("1"), subset: SubsetId("*") };

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
use crate::document::{AnnexChoice, CheckResult, CheckStatus, ClauseId, NormError, Quantity, QuantityKind};
/// 📐️ Pure VDI 3805 compliance helpers (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES)
/// — relocated verbatim from the deleted `⚙️engine`. Native-text parsing/serialization, structural
/// validation, the linear-map utility and diagnostic-to-check mapping are all pure helpers over
/// document types (`ManufacturerCatalog`, `Diagnostic`, …), never over the whole `Vdi3805Snapshot`.
/// `clause`/`na_check`/`pass_check`/`fail_check` are shared with `💡️inferences`'s per-sheet
/// conformance laws (99 `part_N::check(&Vdi3805Snapshot)` functions, which — unlike the Eurocode
/// artifacts' `part_N` modules — take the whole snapshot directly, so they live in `inferences` not
/// here). The whole-artifact JSON (de)serializers live in `🚪️io`.
use crate::*;
// 🔀️ Explicit single-item import: the glob above also pulls in `crate::dsl`
// (the mounted native-text grammar submodule, see `🦀️.rs`), which would otherwise shadow the
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
    let known: BTreeSet<&str> = RecordFamilyId::all_known().iter().copied().collect();
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

//#region 🧪️ComplianceTests
#[cfg(test)]
#[path = "🧪️tests/⚖️compliance/🦀️.rs"]
mod compliance_tests;
//#endregion 🧪️ComplianceTests
