# Higher Fixture and IFC Syntax Kind Bindings

One private test fixture JSON/URI admission helper and two IFC2x3 physical syntax callers now supply explicit InvalidValue. The actual IFC decoder contains only UTF8, Part21 grammar and FILE_SCHEMA admission, with no control/quota interface. Original display prefixes and authored 1:1 spans remain. This is a finite caller binding, not a claim that every private String producer has been retired. Native consumer validation remains pending.

Complete immediate source text, inverse, authored full text, byte counts, hashes and exact per-site decisions are retained in generated/value-refusal/text-error-higher-fixture-ifc-syntax-authored-1.json. Native admission is pending.

## 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🖥️test-app-mutations-document/🦀️.rs

```rust
//#region 🧬️TestDocumentMutationRoot
//! 🧪️ Shared document state and direct mutation fixtures for the Plugin contract.

use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};
use store::ArtifactPack;

//#region 🧫️Snapshot
pub(crate) const MAXIMUM_CHILD_PROBE_BYTES: usize = 4 * 1024 * 1024;
pub(crate) static MAXIMUM_CHILD_CLONES: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
pub(crate) static MAXIMUM_CHILD_ENCODINGS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

#[derive(Debug, Default, PartialEq, Serialize, ToValue, Deserialize, FromValue, dsl::DslArtifact)]
#[dsl(extension = "testkit-macro")]
pub(crate) struct TestSnapshot {
    pub(crate) count: i32,
    pub(crate) label: String,
    /// 🧒️ The members declared in this document's one owned-child slot, `"slot"` — the field a
    /// schema-derived snapshot spells `#[child(kind = "s.test.child")]`. `store::ArtifactChild`
    /// carries `ToValue`/`FromValue`/`DslField` but no serde impl, so the derived serde half skips
    /// it and the hand-written pack/DSL codecs below carry it as its member's canonical
    /// `ArtifactRef` uri.
    #[serde(skip)]
    pub(crate) slot: Vec<store::ArtifactChild<TestSnapshot>>,
}

impl store::ArtifactSqliteSnapshot for TestSnapshot {
    const SQLITE_SCHEMA: &'static str = include_str!("🗄️.sql");
    fn to_sqlite_database(&self, control: &mut store::sqlite_snapshot::SqliteSnapshotControl<'_>) -> Result<store::sqlite_snapshot::SqliteDatabase, String> {
        use store::sqlite_snapshot::{SqliteDatabase, SqliteRow, SqliteValue, SqliteSnapshotPhase};
        control.check_rows(self.slot.len().checked_add(1).ok_or_else(|| "child row count overflow".to_string())?)?;
        control.check_value_bytes(self.label.len())?;
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 0, self.slot.len() + 1)?;
        let mut database = SqliteDatabase::from_schema(Self::SQLITE_SCHEMA).map_err(|error| error.to_string())?;
        database.table_mut("document_state")?.rows.push(SqliteRow { rowid: 1, values: vec![SqliteValue::Integer(1), SqliteValue::Integer(i64::from(self.count)), SqliteValue::Text(self.label.clone())] });
        for (position, child) in self.slot.iter().enumerate() {
            if position % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, position + 1, self.slot.len() + 1)?; }
            let id = i64::try_from(position + 1).map_err(|error| error.to_string())?;
            database.table_mut("slot_children")?.rows.push(SqliteRow { rowid: id, values: vec![SqliteValue::Integer(id), SqliteValue::Integer(1), SqliteValue::Integer(position as i64), SqliteValue::Text(child.target.artifact_id.clone()), SqliteValue::Text(child.target.dialect.artifact_kind.clone()), SqliteValue::Text(child.target.dialect.standard.clone()), SqliteValue::Text(child.target.dialect.subset.clone())] });
        }
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, self.slot.len() + 1, self.slot.len() + 1)?;
        Ok(database)
    }
    fn from_sqlite_database(database: &store::sqlite_snapshot::SqliteDatabase, control: &mut store::sqlite_snapshot::SqliteSnapshotControl<'_>) -> Result<Self, String> {
        use store::sqlite_snapshot::SqliteSnapshotPhase;
        let rows = &database.table("document_state")?.rows;
        let children = &database.table("slot_children")?.rows;
        control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, 0, children.len() + 1)?;
        if rows.len() != 1 || rows[0].rowid != 1 || rows[0].integer(0)? != 1 { return Err("document snapshot requires one state row".into()); }
        let mut snapshot = Self { count: i32::try_from(rows[0].integer(1)?).map_err(|error| error.to_string())?, label: rows[0].text(2)?.to_string(), slot: Vec::new() };
        let mut ordered = children.iter().collect::<Vec<_>>();
        ordered.sort_by_key(|row| row.integer(2).unwrap_or(-1));
        for (position, row) in ordered.into_iter().enumerate() {
            if position % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, position + 1, children.len() + 1)?; }
            if row.rowid != row.integer(0)? || row.integer(1)? != 1 || row.integer(2)? != position as i64 { return Err("invalid ordered document child relationship".into()); }
            let target = store::os_io::ArtifactRef { artifact_id: row.text(3)?.to_string(), dialect: store::os_io::ArtifactDialect { artifact_kind: row.text(4)?.to_string(), standard: row.text(5)?.to_string(), subset: row.text(6)?.to_string() } };
            let target = store::os_io::ArtifactRef::parse_uri(&target.to_uri())?;
            snapshot.slot.push(store::ArtifactChild::new(target.artifact_id.clone(), target));
        }
        control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, children.len() + 1, children.len() + 1)?;
        Ok(snapshot)
    }
}

impl TestSnapshot {
    /// 🧩️ The JSON carriage both hand-written codecs share. An undeclared slot writes no key at
    /// all, so a document with no child encodes exactly the bytes it always did.
    fn to_json(&self) -> Result<serde_json::Value, String> {
        let mut value = serde_json::to_value(self).map_err(|error| error.to_string())?;
        if !self.slot.is_empty() {
            let object = value.as_object_mut().ok_or_else(|| "test snapshot encodes as a json object".to_string())?;
            object.insert("slot".into(), serde_json::Value::Array(self.slot.iter().map(|child| serde_json::Value::String(child.target.to_uri())).collect()));
        }
        Ok(value)
    }

    /// 🧩️ Inverse of {@link TestSnapshot::to_json}: every declared row is one `ArtifactRef` uri,
    /// and a child's id IS its artifact id (what `ChildRestoreProjection` admits).
    fn from_json(mut value: serde_json::Value) -> Result<Self, String> {
        let declared = value.as_object_mut().and_then(|object| object.remove("slot"));
        let mut snapshot: Self = serde_json::from_value(value).map_err(|error| error.to_string())?;
        let Some(declared) = declared else { return Ok(snapshot) };
        for row in declared.as_array().ok_or_else(|| "declared child slot is an array of artifact ref uris".to_string())? {
            snapshot.slot.push(test_child_handle(row.as_str().ok_or_else(|| "declared child is an artifact ref uri".to_string())?)?);
        }
        Ok(snapshot)
    }
}

/// 🧒️ One declared owned-child handle from its canonical `ArtifactRef` uri.
pub(crate) fn test_child_handle(uri: &str) -> Result<store::ArtifactChild<TestSnapshot>, String> {
    let target = store::os_io::ArtifactRef::parse_uri(uri)?;
    Ok(store::ArtifactChild::new(target.artifact_id.clone(), target))
}

/// ♻️ The fixture child is an openable member, so its snapshot needs the same bounded owned-value
/// retirement a real member's does — `store::PackMemberSnapshotOpen` retires the decoded snapshot
/// through it when an open is cancelled or rejected mid-flight.
impl semio_framework_value::retirement::RetireOwned for TestSnapshot {
    fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> {
        let Self { count, label, slot } = self;
        semio_framework_value::retirement::sequence(vec![semio_framework_value::retirement::RetireOwned::retirement(count), semio_framework_value::retirement::RetireOwned::retirement(label), semio_framework_value::retirement::RetireOwned::retirement(slot)])
    }
}

impl semio_framework_schema_composition::ArtifactCompositionFields for TestSnapshot {
    fn visit_child_refs<'a, V: semio_framework_schema_composition::ChildRefVisitor<'a>>(&'a self, visitor: &mut V) -> Result<(), V::Error> {
        semio_framework_schema_composition::ChildFieldRefs::visit_child_field(&self.slot, "slot", visitor)
    }

    fn child_slots() -> &'static [semio_framework_schema_composition::ChildSlotSpec] {
        &[semio_framework_schema_composition::ChildSlotSpec { name: "slot", kind: "s.test.child", many: true }]
    }
}

impl Clone for TestSnapshot {
    fn clone(&self) -> Self {
        if self.label.len() >= MAXIMUM_CHILD_PROBE_BYTES {
            MAXIMUM_CHILD_CLONES.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        }
        Self { count: self.count, label: self.label.clone(), slot: self.slot.clone() }
    }
}

impl store::ArtifactDsl for TestSnapshot {
    const EXTENSION: &'static str = "testkit-macro";
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        if text.trim().is_empty() {
            return Ok(Self::default());
        }
        let value: serde_json::Value = serde_json::from_str(text).map_err(|error| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string(), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
        Self::from_json(value).map_err(|error| semio_framework_diagnostic::TextError::new(error, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        self.to_json().and_then(|value| serde_json::to_string(&value).map_err(|error| error.to_string())).unwrap_or_default()
    }
}

impl ArtifactPack for TestSnapshot {
    /// 🪶️ Publishes this owner's actual relational snapshot capability.
    fn sqlite_snapshot_codec() -> Option<store::ArtifactSqliteSnapshotCodec> {
        Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec())
    }

    fn encode_pack_with(&self, _options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        if self.label.len() >= MAXIMUM_CHILD_PROBE_BYTES {
            MAXIMUM_CHILD_ENCODINGS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        }
        let value = self.to_json().map_err(store::PackError::Schema)?;
        serde_json::to_vec(&value).map_err(|error| store::PackError::Schema(error.to_string()))
    }
    fn decode_pack_with(bytes: &[u8], _options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        if bytes.is_empty() {
            return Ok(Self::default());
        }
        let value: serde_json::Value = serde_json::from_slice(bytes).map_err(|error| store::PackError::Schema(error.to_string()))?;
        Self::from_json(value).map_err(store::PackError::Schema)
    }
}
//#endregion 🧫️Snapshot

//#region 🔺️Diff
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
pub(crate) struct TestDiff {
    pub(crate) count: Option<i32>,
    pub(crate) label: Option<String>,
    /// 🧒️ The whole declared membership of the `"slot"` child slot, as canonical `ArtifactRef`
    /// uris — a child declaration is replaced wholesale, never merged row by row.
    pub(crate) slot: Option<Vec<String>>,
}

impl protocol::MutationDiff<TestSnapshot> for TestDiff {
    fn apply(&self, snapshot: &TestSnapshot) -> protocol::MutationApplyResult<TestSnapshot> {
        let slot = match &self.slot {
            None => snapshot.slot.clone(),
            Some(rows) => rows.iter().map(|uri| test_child_handle(uri)).collect::<Result<Vec<_>, String>>().map_err(|error| protocol::MutationApplyError::new("mutation.apply.declared-child-uri", error))?,
        };
        Ok(TestSnapshot { count: self.count.unwrap_or(snapshot.count), label: self.label.clone().unwrap_or_else(|| snapshot.label.clone()), slot })
    }

    fn absorb(&mut self, other: Self) {
        if other.count.is_some() {
            self.count = other.count;
        }
        if other.label.is_some() {
            self.label = other.label;
        }
        if other.slot.is_some() {
            self.slot = other.slot;
        }
    }
}
//#endregion 🔺️Diff

//#region 🧬️Mutations
#[path = "../../🧫️fixtures/🖥️test-app-mutations/🧬️document/🧬️mutations/🦀️.rs"]
pub mod mutations;
pub(crate) use mutations::{SetCount, SetLabel, SetSlotChildren, TestMutation};
//#endregion 🧬️Mutations
//#endregion 🧬️TestDocumentMutationRoot

```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🧬️schema/🦀️.rs

```rust
//! 🧬️ Ifc2x3Artifact schema — full artifact state for the `2x3` standard (buildingSMART
//! Coordination View 2.0 era, ISO/PAS 16739:2005 schema). Sibling of `4️⃣4`'s `IfcArtifact`, own
//! distinct schema id `s.stdio.ifc.2x3` so the two standards' descriptors never collide in the
//! flat `::semio_framework_schema_registry::register_artifact_schema_descriptor` registry.

use crate::standards::v2x3::subsets::base::schema::snapshot::Ifc2x3Snapshot;
use framework_schema::ArtifactSchema;

//#region 🔖️Artifact
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.ifc.2x3")]
pub struct Ifc2x3Artifact {
    #[state(artifact)]
    pub schema: String,
    /// 📦️ The full, lossless generic Part-21 graph, wrapped in this standard's own
    /// [`Ifc2x3Snapshot`] type — the actual persisted state.
    #[state(artifact)]
    #[value(default)]
    pub document: semio_s_artifact_stdio_contract::part21::Part21Document,
    #[state(artifact)]
    #[value(default)]
    pub edm_preamble: Option<Ifc2x3EdmPreamble>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for Ifc2x3Artifact {
    fn default() -> Self {
        Self::from_snapshot(Ifc2x3Snapshot::default())
    }
}

impl Ifc2x3Artifact {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_snapshot(&self) -> Ifc2x3Snapshot {
        Ifc2x3Snapshot { schema: self.schema.clone(), document: self.document.clone(), edm_preamble: self.edm_preamble.clone() }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_snapshot(snapshot: Ifc2x3Snapshot) -> Self {
        Self { schema: snapshot.schema, document: snapshot.document, edm_preamble: snapshot.edm_preamble }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn set_snapshot(&mut self, snapshot: Ifc2x3Snapshot) {
        self.schema = snapshot.schema;
        self.document = snapshot.document;
        self.edm_preamble = snapshot.edm_preamble;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn ifc2x3_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.stdio.ifc.2x3",
        artifact: semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
        snapshot: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: semio_framework_schema_registry::FacetLeaves {
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
    use crate::standards::v2x3::subsets::base::schema::diff::Ifc2x3Diff;
    use crate::standards::v2x3::subsets::base::schema::mutations::{apply_ifc2x3_mutation, Ifc2x3Mutation};
    use crate::standards::v2x3::subsets::base::schema::snapshot::Ifc2x3Snapshot;
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Builder
    #[derive(Clone, Debug, Default)]
    pub struct Ifc2x3BuilderConstruction {
        snapshot: Ifc2x3Snapshot,
        diagnostics: Vec<semio_framework_diagnostic::Diagnostic>,
    }

    impl ArtifactBuilder for Ifc2x3BuilderConstruction {
        type Snapshot = Ifc2x3Snapshot;
        type Mutation = Ifc2x3Mutation;
        type Diff = Ifc2x3Diff;
        fn empty() -> Self {
            Self { snapshot: Ifc2x3Snapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(<Ifc2x3Snapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<Ifc2x3Snapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = apply_ifc2x3_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = <Ifc2x3Diff as protocol::MutationDiff<Ifc2x3Snapshot>>::apply(&diff, &self.snapshot)?;
            Ok(self)
        }
        fn build(self) -> Result<Self::Snapshot, Vec<semio_framework_diagnostic::Diagnostic>> {
            if self.diagnostics.is_empty() {
                Ok(self.snapshot)
            } else {
                Err(self.diagnostics)
            }
        }
    }
    //#endregion 🔖️Builder
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use crate::standards::v2x3::subsets::base::schema::snapshot::Ifc2x3Snapshot;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    //#region 🔖️Parts
    /// 🧩 Analyzed `stdio.ifc.2x3` parts.
    #[derive(Clone, Debug, Default)]
    pub struct Ifc2x3Parts {
        pub snapshot: Option<Ifc2x3Snapshot>,
    }
    //#endregion 🔖️Parts

    //#region 🔖️Sniff
    /// 🔍️ Real, honest confidence probe: `High` when the text/bytes look like a Part-21 envelope AND
    /// declare `IFC2X3` in `FILE_SCHEMA`; `Medium` for a Part-21 envelope of an unknown schema (could
    /// still decode -- IFC2X3 is layered on the same generic tokenizer); `Low` otherwise.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn sniff_text(body: &str) -> IoConfidence {
        let trimmed = body.trim_start();
        if trimmed.starts_with("ISO-10303-21") {
            if trimmed.contains("IFC2X3") {
                IoConfidence::High
            } else {
                IoConfidence::Medium
            }
        } else {
            IoConfidence::Low
        }
    }
    //#endregion 🔖️Sniff

    //#region 🔖️Analyzer
    pub struct Ifc2x3AnalyzerAnalysis;

    impl ArtifactAnalysis for Ifc2x3AnalyzerAnalysis {
        type Parts = Ifc2x3Parts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.ifc", standard: StandardId("2x3"), subset: SubsetId("*") };

        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            match source {
                AnalyzeSource::Text(text) => {
                    let body = match store::semio_format::split_text_preamble(text) {
                        Ok((_, rest)) => rest,
                        Err(_) => text,
                    };
                    sniff_text(body)
                }
                AnalyzeSource::Binary(bytes) => match std::str::from_utf8(bytes) {
                    Ok(text) => sniff_text(text),
                    Err(_) => IoConfidence::Low,
                },
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = Ifc2x3Parts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match if text.trim_start().starts_with("ISO-10303-21") {
                        crate::standards::v2x3::engine::decode_ifc2x3(text.as_bytes()).map_err(|error| semio_framework_diagnostic::TextError::new(error, semio_framework_diagnostic::TextSpan::at(1, 1)))
                    } else {
                        <Ifc2x3Snapshot as store::ArtifactDsl>::parse_dsl(text)
                    } {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.text", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <Ifc2x3Snapshot as store::ArtifactPack>::decode_pack(bytes).or_else(|_| crate::standards::v2x3::engine::decode_ifc2x3(bytes).map_err(store::PackError::Schema)) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.binary", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                }
            }
            Analysis { parts, dialect: Self::DIALECT, confidence, diagnostics }
        }
    }
    //#endregion 🔖️Analyzer

    //#region 🧪️Tests
    #[cfg(test)]
    include!("🧪️tests/🔬️derived-analysis-unit/🦀️.rs");
    //#endregion 🧪️Tests
}
pub use derived_analysis::*;
//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec Ifc2x3BuilderFacets {
        construction: Ifc2x3BuilderConstruction,
        analysis: Ifc2x3AnalyzerAnalysis,
        composition: super::super::io::derived_composition::Ifc2x3ComposerComposition,
    }
    builder: Ifc2x3Builder,
    analyzer: Ifc2x3Analyzer,
    composer: Ifc2x3Composer,
);
//#endregion 🧬️DerivedArtifactFacets

//#region 🔖️DocumentHelpers
/// 🌱 Empty persisted snapshot. Dissolved out of `⚙️engine`
/// (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — reached as
/// `crate::standards::v2x3::engine::empty_ifc2x3_snapshot` through the `engine`
/// barrel shim.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn empty_ifc2x3_snapshot() -> Ifc2x3Snapshot {
    Ifc2x3Snapshot::default()
}

/// 📄️ Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: the demo
/// `stdio.ifc.2x3` document — a real, minimal IFC2X3 exchange structure (raw HEADER value tuples +
/// two real entities incl. an `IFCOWNERHISTORY` reference chain), matching `4`'s own
/// `demo_ifc_snapshot()` shape but declaring `FILE_SCHEMA(('IFC2X3'))` so `decode_ifc2x3`'s own
/// schema gate accepts it. Fodder for `mutations::demo_mutation_cases()`/`diff::demo_diff_cases()`
/// and this standard's own `conformance_laws` tests (a non-empty snapshot, unlike the prior
/// `empty_ifc2x3_snapshot()` stub, so every recognizer/walk law actually exercises real content).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn demo_ifc2x3_snapshot() -> Ifc2x3Snapshot {
    use semio_s_artifact_stdio_contract::part21::{Part21Document, Part21Header, Part21Instance, Part21Value};
    let document = Part21Document {
        header: Part21Header {
            file_description: vec![Part21Value::List(vec![Part21Value::Str(String::new())]), Part21Value::Str("2;1".into())],
            file_name: vec![
                Part21Value::Str("semio.ifc".into()),
                Part21Value::Str("2026-08-11T00:00:00".into()),
                Part21Value::List(vec![Part21Value::Str("Ueli".into())]),
                Part21Value::List(vec![Part21Value::Str("semio".into())]),
                Part21Value::Str("semio".into()),
                Part21Value::Str("".into()),
                Part21Value::Str("".into()),
            ],
            file_schema: vec![Part21Value::List(vec![Part21Value::Str("IFC2X3".into())])],
        },
        instances: vec![
            Part21Instance { id: 1, entities: vec![("IFCPROJECT".into(), vec![Part21Value::Str("gid-project".into()), Part21Value::Ref(2), Part21Value::Str("Demo Project".into())])] },
            Part21Instance { id: 2, entities: vec![("IFCOWNERHISTORY".into(), vec![Part21Value::Unset, Part21Value::Int(0)])] },
        ],
    };

    Ifc2x3Snapshot { schema: crate::standards::v2x3::subsets::base::schema::snapshot::STDIO_IFC2X3_DOCUMENT_SCHEMA.into(), document, edm_preamble: None }
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️Register
/// 🗂️ **Deliberately left imperative and callable** (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-
/// APP-STATE-MACHINES, per the ticket's own explicit instruction: "leave ifc's registration
/// alone" — `ArtifactDeclaration` has exactly one `.schema()`/`.document_codec()` slot and
/// cannot hold both `4`'s and `2x3`'s independent descriptors/codecs at once, see the artifact
/// root `🦀️.rs`'s own doc comment). Only physically dissolved out of `⚙️engine`; reached
/// as `crate::standards::v2x3::engine::register()` through the `engine` barrel
/// shim, which is exactly the path `🦀️.rs`'s root `ifc::engine::register()` override calls
/// explicitly (alongside `v4::engine::register()`).
///
/// Registers this standard's schema descriptor, document codec, 5-role `LanguageSpec`s, and (via
/// each real subset's own composer) its `SubsetValidator`s. Does NOT call the artifact-level
/// `ifc::composer::register()` (that union is already invoked once from `4`'s own
/// `engine::register()`, extended by this ticket to also union `v2x3::composer::entries()` —
/// calling it a second time here would be a redundant registration, same reasoning gif's
/// `89a::engine::register` doc comment gives).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {
    ::semio_framework_schema_registry::register_artifact_schema_descriptor(ifc2x3_artifact_schema_descriptor()).expect("schema descriptor publication");
    register_artifact_inferences();
    register_pilot_languages();
    semio_framework_plugin::io::register_native_document_codec(semio_framework_plugin::Dialect { artifact_kind: "s.stdio.ifc", standard: semio_framework_plugin::StandardId("2x3"), subset: semio_framework_plugin::SubsetId("*") }, store::ArtifactCodec::bare::<Ifc2x3Snapshot, crate::standards::v2x3::subsets::base::schema::mutations::Ifc2x3Mutation>(crate::standards::v2x3::subsets::base::schema::snapshot::STDIO_IFC2X3_DOCUMENT_SCHEMA))
        .expect("static Stdio registration must be available and conflict-free");
    // 🛡️ D5's generic validate-on-build hook: registers each real subset's `SubsetValidator` so
    // `io_dispatch`/`wire_artifact_compose` re-check them for free. Each subset's `ComposerEntry`
    // is registered separately via this standard's own `composer::entries()` aggregation.
    crate::standards::v2x3::subsets::cv20::io::register();
    crate::standards::v2x3::subsets::sav::io::register();
    crate::standards::v2x3::subsets::cobie::io::register();
}

/// 💡️ Registers `s.stdio.ifc.2x3.inference`'s facet leaves into the OS-wide inference catalog —
/// sibling to the schema descriptor registration above (separate registry, ticket
/// 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register_artifact_inferences() {
    ::semio_framework_schema_registry::register_artifact_inference_descriptor(crate::standards::v2x3::subsets::base::schema::inferences::ifc2x3_artifact_inference_descriptor()).expect("schema descriptor publication");
}

/// 📌️ Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: 5-role
/// `LanguageSpec` registration (Document/Ops/Diff/Pack/Spr), per the recipe's json exemplar —
/// `stdio.ifc.2x3`/`.op`/`.diff`/`.pack`/`.spr`, all `dsl::passthrough_hooks`. `diff`'s `protocol`
/// slot stays `None` matching the exemplar's own shape exactly (the 5-role scheme has no dedicated
/// "diff binary" role even though `🔺️diff/💾️binary/📡️.protocol.semio` is a real,
/// conformance-tested file — its binary form is exercised directly by `protocol_walk_law` below,
/// just not wired through a 6th `LanguageRole`), same precedent `4`'s own
/// `register_pilot_languages` established.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register_pilot_languages() {
    use crate::standards::v2x3::subsets::base::schema::{diff, mutations, snapshot};
    semio_framework_dsl::register_language(semio_framework_dsl::LanguageSpec {
        id: "stdio.ifc.2x3",
        extension: Some("ifc"),
        role: semio_framework_dsl::LanguageRole::Document,
        grammar: Some(snapshot::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(snapshot::text::COMPONENT_GRAMMAR_PATH),
        protocol: Some(snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(snapshot::binary::COMPONENT_PROTOCOL_PATH),
        hooks: semio_framework_dsl::passthrough_hooks("stdio.ifc.2x3"),
    });
    semio_framework_dsl::register_language(semio_framework_dsl::LanguageSpec {
        id: "stdio.ifc.2x3.op",
        extension: None,
        role: semio_framework_dsl::LanguageRole::Ops,
        grammar: Some(mutations::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(mutations::text::COMPONENT_GRAMMAR_PATH),
        protocol: Some(mutations::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(mutations::binary::COMPONENT_PROTOCOL_PATH),
        hooks: semio_framework_dsl::passthrough_hooks("stdio.ifc.2x3.op"),
    });
    semio_framework_dsl::register_language(semio_framework_dsl::LanguageSpec {
        id: "stdio.ifc.2x3.diff",
        extension: None,
        role: semio_framework_dsl::LanguageRole::Diff,
        grammar: Some(diff::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(diff::text::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: semio_framework_dsl::passthrough_hooks("stdio.ifc.2x3.diff"),
    });
    semio_framework_dsl::register_language(semio_framework_dsl::LanguageSpec {
        id: "stdio.ifc.2x3.pack",
        extension: None,
        role: semio_framework_dsl::LanguageRole::Pack,
        grammar: None,
        grammar_path: None,
        protocol: Some(snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(snapshot::binary::COMPONENT_PROTOCOL_PATH),
        hooks: semio_framework_dsl::passthrough_hooks("stdio.ifc.2x3.pack"),
    });
    semio_framework_dsl::register_language(semio_framework_dsl::LanguageSpec {
        id: "stdio.ifc.2x3.spr",
        extension: None,
        role: semio_framework_dsl::LanguageRole::Spr,
        grammar: None,
        grammar_path: None,
        protocol: Some(mutations::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(mutations::binary::COMPONENT_PROTOCOL_PATH),
        hooks: semio_framework_dsl::passthrough_hooks("stdio.ifc.2x3.spr"),
    });
}

// 📌️ `dsl::registry::register_schema_spec` is intentionally NOT called here — `Part21Value` (a
// genuine data-carrying enum) has no `DslField` impl, so no `fn() -> RecordSpec` exists for
// `Ifc2x3Snapshot`/`Ifc2x3Diff` at all (same `register-schema-spec-needs-recordspec` mechanism gap
// `4`'s own `IfcSnapshot`/`IfcDiff` doc comment documents for the isomorphic shape) — filed as a
// `mechanism_gaps` entry rather than fabricating an unrelated spec.
//#endregion 🔖️Register

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use crate::standards::v2x3::subsets::base::schema::snapshot::Ifc2x3EdmPreamble;
//#endregion 🔁️Re-exports

```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs

```rust
//! 📥️ Deserialize `stdio.ifc.2x3` from stdio.txt.

use crate::standards::v2x3::subsets::base::schema::snapshot::Ifc2x3Snapshot;
use semio_s_artifact_stdio_txt::TxtSnapshot;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn deserialize(from: &TxtSnapshot) -> Result<Ifc2x3Snapshot, semio_framework_diagnostic::TextError> {
    crate::standards::v2x3::engine::decode_ifc2x3(from.to_body().as_bytes()).map_err(|e| semio_framework_diagnostic::TextError::new(format!("ifc2x3 parse: {e}"), semio_framework_diagnostic::TextSpan::at(1, 1)))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn deserialize_text(text: &str) -> Result<Ifc2x3Snapshot, semio_framework_diagnostic::TextError> {
    deserialize(&<TxtSnapshot as store::ArtifactDsl>::parse_dsl(text)?)
}

```

