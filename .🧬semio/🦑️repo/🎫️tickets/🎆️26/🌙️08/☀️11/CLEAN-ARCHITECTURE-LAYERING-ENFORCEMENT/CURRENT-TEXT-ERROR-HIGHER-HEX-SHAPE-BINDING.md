# Actual Higher Hexadecimal and Shape Admission

Actual hexadecimal decoders admit hexadecimal syntax only; authored shape producers admit document markers, child identities and response uniqueness only. These existing ordinary syntax transports receive explicit InvalidValue. The Space occurrence checked row-count overflow receives WorkLimit. No control error is classified from its display message. The independently retained original TextError law supplies the mandatory kind contract; higher native consumer proof remains pending.

Complete immediate source text, inverse, authored full text, byte counts, hashes and exact per-site decisions are retained in generated/value-refusal/text-error-higher-hex-shape-authored-1.json. Native admission is pending.

## ✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📸️snapshot/📝️text/🦀️.rs

```rust
//! 🕸️ DAG document text carries its marker and five literal Graph child address fields.

use crate::DagSnapshot;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

/// 📄️ Canonical plugin document; its marker belongs to this artifact owner.

/// 📖️ Parses `.dag` DSL text into a `DagSnapshot`.
pub fn parse_dsl(text: &str) -> Result<DagSnapshot, semio_framework_diagnostic::TextError> {
    <DagSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `DagSnapshot` back to `.dag` DSL text.
pub fn print_dsl(document: &DagSnapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

//#region 🔖️HandcraftedArtifactDsl
impl store::ArtifactDsl for DagSnapshot {
    const EXTENSION: &'static str = "dag";
    fn envelope_id() -> &'static str {
        "dag.dag"
    }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        if let Ok((envelope, _)) = store::semio_format::split_text_preamble(text) {
            if !envelope.matches_identity(Self::envelope_id(), store::semio_format::Component::Dsl, 1) {
                return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "DAG text envelope mismatch", semio_framework_diagnostic::TextSpan::at(1, 1)));
            }
        }
        let body = store::semio_format::split_text_preamble(text).map(|(_, body)| body).unwrap_or(text);
        let record = dsl::parse(body, &Self::__dsl_spec(), &dsl::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: dsl::SourceMode::Document })?;
        let snapshot = Self::__dsl_from_record(&record)?;
        snapshot.validate().map_err(|message| semio_framework_diagnostic::TextError::new(message, semio_framework_diagnostic::TextSpan::at(1, 1)))?;
        Ok(snapshot)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(Self::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid DAG envelope");
        store::semio_format::wrap_text(&envelope, &body)
    }
}
//#endregion 🔖️HandcraftedArtifactDsl

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[cfg(test)]
#[path = "🧪️tests/🔬️semio-grammar-conformance/🦀️.rs"]
mod semio_grammar_conformance;

```

## ✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs

```rust
//! 🧬️ Cad snapshot schema — artifact-lane fields only.

use crate::{empty_cad_snapshot, CadDrawingChild, CadModelChild, CadNode, CadReferenceList};
use framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};
use std::collections::BTreeMap;

#[cfg(test)]
#[path = "🧪️tests/🪶️sqlite/🦀️.rs"]
mod sqlite_tests;

#[path = "🪶️sqlite/🦀️.rs"]
mod sqlite;

//#region 🔖️Snapshot
/// 📸️ Persisted cad document snapshot (persistent fields of the artifact). Ticket
/// `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 3: the four per-pane object/geometry field
/// pairs that used to duplicate `SemioBrepSnapshot`'s topology inline (`CadObject`/`CadGeometry` at
/// `crate::🦀️.rs`) are replaced by four fixed composed
/// `s.stdio.semio.model` CHILD slots — one per `CadPaneId` — plus a forward `drawings` composition
/// slot per the design map's `cad | engineering assembly | model, drawing` row. `#[child(...)]`
/// drives `#[derive(ArtifactSchema)]`'s slot-table emission; never hand-written.
///
/// 🛡️ `deny_unknown_fields` closes that replacement: a snapshot still carrying the retired inline
/// `objects`/`shapeGeometry`/`activeModelDefinitionId` keys must FAIL to decode, never decode with
/// them silently dropped (`🧫️fixtures/🪪️document`'s `invalidDocuments`).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, ArtifactSchema, dsl::DslRecord)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(extension = "cad")]
#[artifact_schema(id = "s.cad.cad")]
pub struct CadSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub id: String,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub shape_model: Option<CadModelChild>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub building_model: Option<CadModelChild>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub energy_model: Option<CadModelChild>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub structure_classic_model: Option<CadModelChild>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    #[value(default)]
    pub drawings: Vec<CadDrawingChild>,
    #[value(default)]
    #[state(artifact)]
    pub references_by_model_definition_id: BTreeMap<String, CadReferenceList>,
    #[value(default)]
    #[state(artifact)]
    pub nodes: Vec<CadNode>,
}

//#region 🔖️ExactChildren
fn exact_child(target: &store::os_io::ArtifactRef, subset: &str) -> Result<(), String> {
    if target.dialect.artifact_kind != "s.stdio.semio" || target.dialect.standard != "v1" || target.dialect.subset != subset {
        return Err(format!("cad child must target s.stdio.semio@v1/{subset}"));
    }
    Ok(())
}

/// 🛡️ Every literal child handle retains its independent local identity and exact declared domain.
fn require_exact_children(s: &CadSnapshot) -> Result<(), String> {
    for child in [&s.shape_model, &s.building_model, &s.energy_model, &s.structure_classic_model].into_iter().flatten() {
        exact_child(&child.target, "model")?;
    }
    for child in &s.drawings {
        exact_child(&child.target, "drawing")?;
    }
    Ok(())
}
//#endregion 🔖️ExactChildren

//#region 🔖️HandcraftedArtifactCodecs
/// ✉️ `ArtifactDsl` and `ArtifactPack` are the derived spec-driven text and pack of the one
/// `dsl::DslRecord` spec; both re-check every composed child's exact identity on decode.
impl store::ArtifactDsl for CadSnapshot {
    const EXTENSION: &'static str = "cad";
    fn envelope_id() -> &'static str {
        "cad.cad"
    }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = dsl::parse(body, &Self::__dsl_spec(), &dsl::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: dsl::SourceMode::Document })?;
        let snapshot = Self::__dsl_from_record(&record)?;
        require_exact_children(&snapshot).map_err(|error| semio_framework_diagnostic::TextError::new(error, semio_framework_diagnostic::TextSpan::at(1, 1)))?;
        Ok(snapshot)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for CadSnapshot {
    fn sqlite_snapshot_codec() -> Option<store::ArtifactSqliteSnapshotCodec> {
        Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec())
    }
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token())));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        let snapshot = Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)?;
        require_exact_children(&snapshot).map_err(store::PackError::Schema)?;
        Ok(snapshot)
    }
    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs
//#endregion 🔖️Snapshot

```

## ✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs

```rust
//! 🪐️ Literal ordered Space metadata, timestamp words and per-occurrence dialect identities.
use super::{SSpaceSnapshot,SpaceArtifactRow,SpaceArtifactDialect};
use store::{ArtifactSqliteSnapshot,sqlite_snapshot::{SqliteDatabase,SqliteRow,SqliteSnapshotControl,SqliteSnapshotPhase,SnapshotEncoding,validate_sqlite_database_schema,artifact::{Projection,Cell,NativeEncodingBound}}};
fn count(length:usize)->Result<usize,String>{length.checked_mul(2).and_then(|n|n.checked_add(1)).ok_or_else(||"Space occurrence row count overflow".into())}
fn words(value:u64)->[Cell<'static>;2]{[Cell::Integer((value>>32)as i64),Cell::Integer((value&0xffffffff)as i64)]}
fn timestamp(row:&SqliteRow,high:usize,low:usize)->Result<u64,String>{let high=u32::try_from(row.integer(high)?).map_err(|_|"Space timestamp high word exceeds unsigned32")?;let low=u32::try_from(row.integer(low)?).map_err(|_|"Space timestamp low word exceeds unsigned32")?;Ok((u64::from(high)<<32)|u64::from(low))}
impl ArtifactSqliteSnapshot for SSpaceSnapshot{
 const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
 fn encode_sqlite_snapshot_native(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<store::io_schema::IoPayload,String>{control.checkpoint(SqliteSnapshotPhase::EncodeNative,0,0)?;control.check_rows(count(self.artifacts.len())?)?;store::encode_sqlite_snapshot_record_native(encoding,"s.space",Self::__dsl_spec_producer(),|native|self.__dsl_to_record_controlled(native),control)}
 fn decode_sqlite_snapshot_native(payload:&store::io_schema::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<Self,String>{
  let maximum=control.limits().max_rows;store::decode_sqlite_snapshot_record_native(payload,"s.space",Self::__dsl_spec_producer(),|record,native|{
   let length=match record.get(2){Some(dsl::FieldValue::List(rows))=>rows.len(),_=>return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "Space native occurrence list missing",semio_framework_diagnostic::TextSpan::at(1,1)))};
   let rows=count(length).map_err(|e|semio_framework_diagnostic::TextError::new(e,semio_framework_diagnostic::TextSpan::at(1,1)))?;if rows>maximum{return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::WorkLimit, "Space native occurrence rows exceed caller limit",semio_framework_diagnostic::TextSpan::at(1,1)))}
   Self::__dsl_from_record_controlled(record,native)
  },control)
 }
 fn preflight_sqlite_snapshot_encoding(&self,_encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),String>{
  let mut bound=NativeEncodingBound::new(control)?;bound.add(4096)?;for value in[&self.schema,&self.space_id]{bound.repeated(value.len(),6)?;}for row in &self.artifacts{bound.add(1024)?;for value in[&row.id,&row.name,&row.kind_id,&row.schema,&row.created_by,&row.updated_by,&row.dialect.artifact_kind,&row.dialect.standard,&row.dialect.subset]{bound.repeated(value.len(),6)?;}}bound.finish()
 }
 fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,String>{
  control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,self.artifacts.len())?;control.check_rows(count(self.artifacts.len())?)?;let mut bytes=8usize.checked_add(self.schema.len()).and_then(|n|n.checked_add(self.space_id.len())).ok_or("Space document value overflow")?;control.check_value_bytes(bytes)?;
  for(index,row)in self.artifacts.iter().enumerate(){bytes=bytes.checked_add(64).ok_or("Space occurrence value overflow")?;for value in[&row.id,&row.name,&row.kind_id,&row.schema,&row.created_by,&row.updated_by,&row.dialect.artifact_kind,&row.dialect.standard,&row.dialect.subset]{bytes=bytes.checked_add(value.len()).ok_or("Space occurrence text overflow")?;control.check_value_bytes(bytes)?;}if(index+1)%256==0{control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,index+1,self.artifacts.len())?;}}
  let mut p=Projection::new(Self::SQLITE_SCHEMA,control)?;p.insert_key("space_document",1,&[Cell::Text(&self.schema),Cell::Text(&self.space_id)])?;
  for(index,row)in self.artifacts.iter().enumerate(){let ordinal=i64::try_from(index).map_err(|_|"Space ordinal exceeds integer64")?;let id=ordinal.checked_add(1).ok_or("Space occurrence identity overflow")?;let created=words(row.created_at_ms);let updated=words(row.updated_at_ms);
   p.insert_key("space_artifact",id,&[Cell::Integer(1),Cell::Integer(ordinal),Cell::Text(&row.id),Cell::Text(&row.name),Cell::Text(&row.kind_id),Cell::Text(&row.schema),created[0],created[1],Cell::Text(&row.created_by),updated[0],updated[1],Cell::Text(&row.updated_by)])?;
   p.insert_key("space_artifact_dialect",id,&[Cell::Text(&row.dialect.artifact_kind),Cell::Text(&row.dialect.standard),Cell::Text(&row.dialect.subset)])?;
  }p.finish()
 }
 fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,String>{
  control.check_database(database,SqliteSnapshotPhase::ReconstructSnapshot)?;validate_sqlite_database_schema(database,Self::SQLITE_SCHEMA,control.limits()).map_err(|e|e.to_string())?;
  if database.tables.len()!=3{return Err("Space requires its three literal tables".into())}let documents=database.table("space_document")?;let rows=database.table("space_artifact")?;let dialects=database.table("space_artifact_dialect")?;let root=documents.single_row()?;if rows.rows.len()!=dialects.rows.len(){return Err("Space occurrence and dialect ownership differs".into())}
  let maximum=control.limits().max_value_bytes;let mut progress=|state:protocol::native_decoding::NativeDecodeProgress|control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,state.completed,state.total).is_ok();let mut native=dsl::NativeDecodeControl::new(maximum,&mut progress);native.begin_stage(count(rows.rows.len())?)?;
  for(table,width)in[(documents,3),(rows,13),(dialects,4)]{for row in &table.rows{native.step()?;if row.values.len()!=width||row.integer(0)?!=row.rowid{return Err("Space row identity differs".into())}}}
  let mut ordered=native.allocate_vec::<Option<&SqliteRow>>(rows.rows.len())?;ordered.resize(rows.rows.len(),None);native.begin_stage(rows.rows.len())?;for row in &rows.rows{native.step()?;let ordinal=usize::try_from(row.integer(2)?).map_err(|_|"Space ordinal differs")?;if row.integer(1)?!=root.rowid||ordinal>=ordered.len()||ordered[ordinal].replace(row).is_some(){return Err("Space occurrence owner or ordinal differs".into())}}
  native.charge(dialects.rows.len().checked_mul(96).ok_or("Space dialect lookup overflow")?)?;let mut lookup=std::collections::BTreeMap::new();native.begin_stage(dialects.rows.len())?;for row in &dialects.rows{native.step()?;if lookup.insert(row.rowid,row).is_some(){return Err("Space dialect identity repeats".into())}}
  let schema=native.copy_text(root.text(1)?)?;let space_id=native.copy_text(root.text(2)?)?;let mut artifacts=native.allocate_vec::<SpaceArtifactRow>(ordered.len())?;native.begin_stage(ordered.len())?;
  for row in ordered{native.step()?;let row=row.ok_or("Space occurrence ordinal missing")?;let dialect=lookup.remove(&row.rowid).ok_or("Space dialect owner missing")?;artifacts.push(SpaceArtifactRow{id:native.copy_text(row.text(3)?)?,name:native.copy_text(row.text(4)?)?,kind_id:native.copy_text(row.text(5)?)?,schema:native.copy_text(row.text(6)?)?,dialect:SpaceArtifactDialect{artifact_kind:native.copy_text(dialect.text(1)?)?,standard:native.copy_text(dialect.text(2)?)?,subset:native.copy_text(dialect.text(3)?)?},created_at_ms:timestamp(row,7,8)?,created_by:native.copy_text(row.text(9)?)?,updated_at_ms:timestamp(row,10,11)?,updated_by:native.copy_text(row.text(12)?)?});}
  if !lookup.is_empty(){return Err("Space unowned dialect entities".into())}native.checkpoint()?;Ok(Self{schema,space_id,artifacts})
 }
 fn validate_sqlite_snapshot_subset(&self,dialect:&store::io_schema::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->store::io_schema::IoResult<()>{
  control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,0)?;if dialect.artifact_kind!="s.space.space"||dialect.standard!="1"||dialect.subset!="*"{return Err(String::from("Space does not own this semantic subset").into())}if Self::from_sqlite_database(database,control)?!=*self{return Err(String::from("Space owned document state differs").into())}Ok(store::io_schema::IoOutcome::clean(()))
 }
}

```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/✏️editor/🦀️.rs

```rust
//! ✏️ Csv editor — the FIRST authored `ArtifactEditor` surface for `s.stdio.csv@rfc4180/*`
//! (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET). One real window, `🪟️main`
//! (`TableWindowKit`), directly editing `CsvSnapshot.records` through the artifact's own
//! `CsvMutation::SetField`.

use crate::editor::csv::modes::edit;
use crate::editor::csv::modes::edit::windows::main;
use crate::{CsvField, CsvMutation, CsvRecord, CsvSnapshot, STDIO_CSV_DOCUMENT_SCHEMA};
use semio_framework_plugin::retained_command::{ArtifactRetainedCommandInputs, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
use semio_framework_plugin::AppOperationContext;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::ArtifactOwnedToolJobFactory;
use semio_framework_plugin::ArtifactOwnedToolJobRequest;
use semio_framework_plugin::ArtifactStoreInitializationJob;
use semio_framework_plugin::ArtifactToolFactoryRegistry;
use semio_framework_plugin::ArtifactToolPublicationContract;
use semio_framework_plugin::ArtifactToolPublicationLane;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::Dialect;
use semio_framework_plugin::DraftView;
use semio_framework_plugin::Editor;
use semio_framework_plugin::EditorApp;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::InteractiveJobClassification;
use semio_framework_ui_locale::Label;
use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use semio_framework_plugin::NoDraft;
use semio_framework_plugin::NoDraftMutation;
use semio_framework_plugin::NoPresence;
use semio_framework_plugin::NoPresenceMutation;
use semio_framework_plugin::NoTransient;
use semio_framework_plugin::NoTransientMutation;
use semio_framework_plugin::StandardId;
use semio_framework_plugin::SubsetId;
use semio_framework_plugin::ToolExecutionContract;
use semio_framework_plugin::ToolFactoryKey;
use semio_framework_plugin::ToolJobFactory;
use semio_framework_plugin::ToolJobFactoryError;
use semio_framework_plugin::ToolOperationSpec;
use semio_s_artifact_stdio_contract::editing::SnapshotEditEvent;

//#region 🔖️Dialect
/// 🪪️ Artifact coordinate — verified against `crate::schema::derived_analysis::
/// CsvAnalyzerAnalysis::DIALECT` (the artifact's own real analysis-capability row), not guessed.
/// Duplicated (not imported) in the sibling `👁️viewer` surface root — never shared through an
/// `editor`-rooted import, so a viewer file can never depend on this module.
pub const CSV_EDITOR_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.csv", standard: StandardId("rfc4180"), subset: SubsetId::ANY };
//#endregion 🔖️Dialect

//#region 🔖️Command
/// ✏️ The editor's typed command channel — exactly the one edit `🪟️main`'s `editable_window_kind()`
/// action (`set-cell`, contract §2.6) can trigger. `row`/`column` index the rendered grid after the
/// header split and `revision` prevents a concurrent row shift from redirecting the edit.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub enum CsvEditorCommand {
    SetCell {
        row: u32,
        column: u32,
        revision: String,
        value: String,
    },
    AddRow {
        revision: String,
    },
    RemoveRow {
        row: u32,
        revision: String,
    },
    AddColumn {
        revision: String,
    },
    RemoveColumn {
        column: u32,
        revision: String,
    },
    SetHeader {
        column: u32,
        revision: String,
        value: String,
    },
    EditSnapshot {
        event: SnapshotEditEvent,
    },
    /// 🎬️ The navbar example picker's payload — see the `🧵️RetainedRoutes` region below.
    SetActiveExample {
        example_id: String,
    },
}

impl protocol::OpText for CsvEditorCommand {
    fn print_op(&self) -> String {
        match self {
            CsvEditorCommand::SetCell { row, column, revision, value } => format!("set-cell row={row} column={column} revision={} value={}", crate::schema::diff::hex_encode(revision.as_bytes()), crate::schema::diff::hex_encode(value.as_bytes())),
            CsvEditorCommand::AddRow { revision } => format!("add-row revision={}", crate::schema::diff::hex_encode(revision.as_bytes())),
            CsvEditorCommand::RemoveRow { row, revision } => format!("remove-row row={row} revision={}", crate::schema::diff::hex_encode(revision.as_bytes())),
            CsvEditorCommand::AddColumn { revision } => format!("add-column revision={}", crate::schema::diff::hex_encode(revision.as_bytes())),
            CsvEditorCommand::RemoveColumn { column, revision } => format!("remove-column column={column} revision={}", crate::schema::diff::hex_encode(revision.as_bytes())),
            CsvEditorCommand::SetHeader { column, revision, value } => format!("set-header column={column} revision={} value={}", crate::schema::diff::hex_encode(revision.as_bytes()), crate::schema::diff::hex_encode(value.as_bytes())),
            CsvEditorCommand::EditSnapshot { event } => format!("snapshot-edit event={}", crate::schema::diff::hex_encode(&<SnapshotEditEvent as protocol::OpBinary>::encode_op(event).expect("snapshot edit event encodes"))),
            CsvEditorCommand::SetActiveExample { example_id } => format!("active-example id={}", crate::schema::diff::hex_encode(example_id.as_bytes())),
        }
    }
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        if let Some(rest) = line.strip_prefix("active-example id=") {
            let bytes = crate::schema::diff::hex_decode(rest).map_err(|error| semio_framework_diagnostic::TextError::new(format!("csv editor command: invalid id hex: {error}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
            let example_id = String::from_utf8(bytes).map_err(|error| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("csv editor command: invalid id utf8: {error}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
            return Ok(CsvEditorCommand::SetActiveExample { example_id });
        }
        if let Some(raw) = line.strip_prefix("snapshot-edit event=") {
            let bytes = crate::schema::diff::hex_decode(raw).map_err(|error| semio_framework_diagnostic::TextError::new(format!("csv editor command: invalid snapshot edit hex: {error}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
            let event = <SnapshotEditEvent as protocol::OpBinary>::decode_op(&bytes).map_err(|error| semio_framework_diagnostic::TextError::new(format!("csv editor command: invalid snapshot edit: {error}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
            return Ok(CsvEditorCommand::EditSnapshot { event });
        }
        let (action, rest) = line.split_once(' ').ok_or_else(|| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("csv editor command: unknown line {line:?}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
        let mut row = None;
        let mut column = None;
        let mut revision = None;
        let mut value = None;
        for token in rest.split(' ') {
            let (key, raw) = token.split_once('=').ok_or_else(|| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("csv editor command: bad token {token:?}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
            match key {
                "row" => row = raw.parse::<u32>().ok(),
                "column" => column = raw.parse::<u32>().ok(),
                "revision" => {
                    let bytes = crate::schema::diff::hex_decode(raw).map_err(|error| semio_framework_diagnostic::TextError::new(format!("csv editor command: invalid revision hex: {error}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
                    revision = Some(String::from_utf8(bytes).map_err(|error| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("csv editor command: invalid revision utf8: {error}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?);
                }
                "value" => {
                    let bytes = crate::schema::diff::hex_decode(raw).map_err(|error| semio_framework_diagnostic::TextError::new(format!("csv editor command: invalid value hex: {error}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
                    value = Some(String::from_utf8(bytes).map_err(|error| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("csv editor command: invalid value utf8: {error}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?);
                }
                _ => return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("csv editor command: unknown argument {key:?}"), semio_framework_diagnostic::TextSpan::at(1, 1))),
            }
        }
        let missing = |fields: &str| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("csv editor command: missing {fields}"), semio_framework_diagnostic::TextSpan::at(1, 1));
        match action {
            "set-cell" => {
                Ok(CsvEditorCommand::SetCell { row: row.ok_or_else(|| missing("row"))?, column: column.ok_or_else(|| missing("column"))?, revision: revision.ok_or_else(|| missing("revision"))?, value: value.ok_or_else(|| missing("value"))? })
            }
            "add-row" => Ok(CsvEditorCommand::AddRow { revision: revision.ok_or_else(|| missing("revision"))? }),
            "remove-row" => Ok(CsvEditorCommand::RemoveRow { row: row.ok_or_else(|| missing("row"))?, revision: revision.ok_or_else(|| missing("revision"))? }),
            "add-column" => Ok(CsvEditorCommand::AddColumn { revision: revision.ok_or_else(|| missing("revision"))? }),
            "remove-column" => Ok(CsvEditorCommand::RemoveColumn { column: column.ok_or_else(|| missing("column"))?, revision: revision.ok_or_else(|| missing("revision"))? }),
            "set-header" => Ok(CsvEditorCommand::SetHeader { column: column.ok_or_else(|| missing("column"))?, revision: revision.ok_or_else(|| missing("revision"))?, value: value.ok_or_else(|| missing("value"))? }),
            _ => Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("csv editor command: unknown action {action:?}"), semio_framework_diagnostic::TextSpan::at(1, 1))),
        }
    }
}

impl protocol::OpBinary for CsvEditorCommand {
    /// 🎯️ The app-owned retained routes this command channel carries — the join key
    /// `AppActionRegistry::validate_tool_job_rows` demands an exact owner-local proof for. The `TableWindowKit`
    /// mints `set-cell`, but only this editor can reduce it into its own mutation, so it is an
    /// app-owned route exactly like the example switch.
    const TOOL_JOB_IDS: &'static [&'static str] = CSV_COMMAND_TOOL_IDS;

    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(<Self as protocol::OpText>::print_op(self).into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let line = String::from_utf8(bytes.to_vec()).map_err(|error| protocol::ProtocolError::Malformed { what: "csv editor command utf8", offset: 0, detail: error.to_string() })?;
        <Self as protocol::OpText>::parse_op(&line).map_err(|error| protocol::ProtocolError::Malformed { what: "csv editor command", offset: 0, detail: error.to_string() })
    }
}
//#endregion 🔖️Command

//#region 🔖️GridMapping
/// 🧮️ Pure row-offset math, kept standalone so it is directly unit-testable without constructing
/// a full `ArtifactView`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn grid_row_to_record_index(has_header: bool, row: u32) -> usize {
    if has_header {
        row as usize + 1
    } else {
        row as usize
    }
}
//#endregion 🔖️GridMapping

//#region 🧵️RetainedRoutes
/// 🪟️ The verb the `TableWindowKit` mints for `🪟️main` — declared by the framework, reduced only here.
const CSV_KIT_ACTION_ID: &str = "set-cell";
/// 🧵️ The app-owned retained routes this editor declares: the example switch and `set-cell`.
/// `validate_ui_dispatch_classification` refuses any verb that is not `Migrated`, and `Migrated`
/// only survives the guest's `interactive-job.catalog-incomplete` boot check when this roster, the
/// publication contracts and the `bounded_first_step_tool_proofs!` block below all name the same
/// ids. Without the kit verb's row the reactor refused every `set-cell` with
/// `interactive-job.missing-factory`.
const CSV_RETAINED_TOOL_IDS: &[&str] = &[
    semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID,
    CSV_KIT_ACTION_ID,
    semio_s_artifact_stdio_contract::ADD_TABLE_ROW_ACTION_ID,
    semio_s_artifact_stdio_contract::REMOVE_TABLE_ROW_ACTION_ID,
    semio_s_artifact_stdio_contract::ADD_TABLE_COLUMN_ACTION_ID,
    semio_s_artifact_stdio_contract::REMOVE_TABLE_COLUMN_ACTION_ID,
    semio_s_artifact_stdio_contract::SET_TABLE_HEADER_ACTION_ID,
];
const CSV_COMMAND_TOOL_IDS: &[&str] = &[
    semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID,
    CSV_KIT_ACTION_ID,
    semio_s_artifact_stdio_contract::ADD_TABLE_ROW_ACTION_ID,
    semio_s_artifact_stdio_contract::REMOVE_TABLE_ROW_ACTION_ID,
    semio_s_artifact_stdio_contract::ADD_TABLE_COLUMN_ACTION_ID,
    semio_s_artifact_stdio_contract::REMOVE_TABLE_COLUMN_ACTION_ID,
    semio_s_artifact_stdio_contract::SET_TABLE_HEADER_ACTION_ID,
    semio_s_artifact_stdio_contract::editing::SET_SNAPSHOT_VALUE_ACTION_ID,
    semio_s_artifact_stdio_contract::editing::INSERT_SNAPSHOT_VALUE_ACTION_ID,
    semio_s_artifact_stdio_contract::editing::REMOVE_SNAPSHOT_VALUE_ACTION_ID,
    semio_s_artifact_stdio_contract::editing::MOVE_SNAPSHOT_VALUE_ACTION_ID,
    semio_s_artifact_stdio_contract::editing::RENAME_SNAPSHOT_KEY_ACTION_ID,
    semio_s_artifact_stdio_contract::editing::REPLACE_SNAPSHOT_SOURCE_ACTION_ID,
];
const CSV_RETAINED_PAYLOAD_SCHEMA: &str = "stdio.csv.tool-command.v1";
const CSV_RETAINED_RAW_BYTES: usize = 8_192;
/// 🚦️ The example switch publishes into NO document lane: it hands the host one
/// `Effect::LoadDocument`, so its only lane is `HostOnly`. `set-cell` publishes the artifact
/// mutation it reduces into, so its only lane is `Artifact`.
const CSV_RETAINED_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] = &[
    ArtifactToolPublicationContract { tool_id: semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: CSV_KIT_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: semio_s_artifact_stdio_contract::ADD_TABLE_ROW_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: semio_s_artifact_stdio_contract::REMOVE_TABLE_ROW_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: semio_s_artifact_stdio_contract::ADD_TABLE_COLUMN_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: semio_s_artifact_stdio_contract::REMOVE_TABLE_COLUMN_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: semio_s_artifact_stdio_contract::SET_TABLE_HEADER_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
];

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn csv_retained_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(CSV_RETAINED_RAW_BYTES, 64, 1, 65_536, 7_500)
}

/// 📚️ The document a named example loads. The subset publishes exactly one (`crate::examples::demo`,
/// `ID = "demo"`), whose asset IS this app's curated document; every other id — including the empty
/// id the shell sends for "the app's own default document" — opens the genesis document.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn csv_example_snapshot(example_id: &str) -> CsvSnapshot {
    if example_id == crate::examples::demo::ID {
        <CsvSnapshot as store::ArtifactDsl>::parse_dsl(crate::examples::demo::PRIMARY_TEXT).unwrap_or_default()
    } else {
        CsvSnapshot::default()
    }
}

/// 🌉️ Resolves the react/wgpu shells' `{action, args}` pair into this editor's typed command.
/// `ArtifactEditor::command_from_action`'s default refuses EVERY id, which is why the boot example,
/// every navbar pick and every Actions-pane row died before reaching a command.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn csv_command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<CsvEditorCommand, Fault> {
    if let Some(event) = semio_s_artifact_stdio_contract::editing::snapshot_edit_event_from_action(action, args)? {
        return Ok(CsvEditorCommand::EditSnapshot { event });
    }
    match action {
        semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID => Ok(CsvEditorCommand::SetActiveExample { example_id: semio_s_artifact_stdio_contract::example_id_argument(args, "") }),
        CSV_KIT_ACTION_ID => {
            let edit = semio_s_artifact_stdio_contract::window_kit_revisioned_cell_edit(args)?;
            Ok(CsvEditorCommand::SetCell { row: edit.row, column: edit.column, revision: edit.revision, value: edit.value })
        }
        semio_s_artifact_stdio_contract::ADD_TABLE_ROW_ACTION_ID => Ok(CsvEditorCommand::AddRow { revision: semio_s_artifact_stdio_contract::window_kit_required_text_argument(args, "revision")? }),
        semio_s_artifact_stdio_contract::REMOVE_TABLE_ROW_ACTION_ID => {
            Ok(CsvEditorCommand::RemoveRow { row: semio_s_artifact_stdio_contract::window_kit_required_index_argument(args, "row")?, revision: semio_s_artifact_stdio_contract::window_kit_required_text_argument(args, "revision")? })
        }
        semio_s_artifact_stdio_contract::ADD_TABLE_COLUMN_ACTION_ID => Ok(CsvEditorCommand::AddColumn { revision: semio_s_artifact_stdio_contract::window_kit_required_text_argument(args, "revision")? }),
        semio_s_artifact_stdio_contract::REMOVE_TABLE_COLUMN_ACTION_ID => {
            Ok(CsvEditorCommand::RemoveColumn { column: semio_s_artifact_stdio_contract::window_kit_required_index_argument(args, "column")?, revision: semio_s_artifact_stdio_contract::window_kit_required_text_argument(args, "revision")? })
        }
        semio_s_artifact_stdio_contract::SET_TABLE_HEADER_ACTION_ID => Ok(CsvEditorCommand::SetHeader {
            column: semio_s_artifact_stdio_contract::window_kit_required_index_argument(args, "column")?,
            revision: semio_s_artifact_stdio_contract::window_kit_required_text_argument(args, "revision")?,
            value: semio_s_artifact_stdio_contract::window_kit_required_text_argument(args, "value")?,
        }),
        other => Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.csv.unhandled-action"), format!("action '{other}' is not one of this editor's declared verbs (setActiveExample, set-cell)"))),
    }
}

/// 🏷️ The manifest id each command was declared under.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn csv_command_id(command: &CsvEditorCommand) -> &'static str {
    match command {
        CsvEditorCommand::SetCell { .. } => CSV_KIT_ACTION_ID,
        CsvEditorCommand::AddRow { .. } => semio_s_artifact_stdio_contract::ADD_TABLE_ROW_ACTION_ID,
        CsvEditorCommand::RemoveRow { .. } => semio_s_artifact_stdio_contract::REMOVE_TABLE_ROW_ACTION_ID,
        CsvEditorCommand::AddColumn { .. } => semio_s_artifact_stdio_contract::ADD_TABLE_COLUMN_ACTION_ID,
        CsvEditorCommand::RemoveColumn { .. } => semio_s_artifact_stdio_contract::REMOVE_TABLE_COLUMN_ACTION_ID,
        CsvEditorCommand::SetHeader { .. } => semio_s_artifact_stdio_contract::SET_TABLE_HEADER_ACTION_ID,
        CsvEditorCommand::EditSnapshot { event } => event.action_id(),
        CsvEditorCommand::SetActiveExample { .. } => semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID,
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn csv_retained_extent(_command: &CsvEditorCommand, _snapshot: &CsvSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    Some(1)
}

/// ✏️ The one reduction `handle` and the retained route share: the example switch hands the host
/// its document, `set-cell` becomes this artifact's own mutation.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn csv_emit(command: &CsvEditorCommand, snapshot: &CsvSnapshot) -> Result<Emit<CsvMutation, NoConfigMutation, NoDraftMutation>, Fault> {
    csv_emit_at_revision(command, snapshot, None)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn csv_emit_at_revision(command: &CsvEditorCommand, snapshot: &CsvSnapshot, canonical_revision: Option<&str>) -> Result<Emit<CsvMutation, NoConfigMutation, NoDraftMutation>, Fault> {
    if let CsvEditorCommand::EditSnapshot { event } = command {
        return <CsvEditor as semio_s_artifact_stdio_contract::editing::SnapshotEditingEditor>::snapshot_edit_emit(event, snapshot);
    }
    if let CsvEditorCommand::SetActiveExample { example_id } = command {
        return Ok(Emit { effects: vec![semio_s_artifact_stdio_contract::load_example_effect(&csv_example_snapshot(example_id), STDIO_CSV_DOCUMENT_SCHEMA)], ..Default::default() });
    }
    let revision = match command {
        CsvEditorCommand::SetCell { revision, .. }
        | CsvEditorCommand::AddRow { revision }
        | CsvEditorCommand::RemoveRow { revision, .. }
        | CsvEditorCommand::AddColumn { revision }
        | CsvEditorCommand::RemoveColumn { revision, .. }
        | CsvEditorCommand::SetHeader { revision, .. } => revision,
        CsvEditorCommand::EditSnapshot { .. } | CsvEditorCommand::SetActiveExample { .. } => unreachable!(),
    };
    let current_revision = canonical_revision.map(str::to_owned).unwrap_or_else(|| semio_s_artifact_stdio_contract::window_kit_snapshot_revision(snapshot));
    if current_revision != *revision {
        return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.csv.table-conflict"), "The CSV document changed before this table draft was applied."));
    }
    let mutation = match command {
        CsvEditorCommand::SetCell { row, column, value, .. } => {
            let record_index = grid_row_to_record_index(snapshot.has_header, *row);
            let field = snapshot
                .records
                .get(record_index)
                .and_then(|record| record.fields.get(*column as usize))
                .ok_or_else(|| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.csv.cell-stale"), format!("CSV cell {row},{column} no longer exists")))?;
            if field.value == *value {
                return Ok(Emit::default());
            }
            CsvMutation::SetField(crate::schema::mutations::set_field::SetField { record_index, field_index: *column as usize, value: value.clone(), quoted: field.quoted })
        }
        CsvEditorCommand::AddRow { .. } => {
            let width = snapshot.records.first().filter(|_| snapshot.has_header).map(|record| record.fields.len()).unwrap_or_else(|| snapshot.records.iter().map(|record| record.fields.len()).max().unwrap_or(0)).max(1);
            let row = CsvRecord { fields: vec![CsvField::default(); width] };
            if snapshot.has_header && snapshot.records.is_empty() {
                let mut next = snapshot.clone();
                next.records.push(CsvRecord { fields: vec![CsvField::default(); width] });
                next.records.push(row);
                CsvMutation::SetSnapshot(crate::schema::mutations::set_snapshot::SetSnapshot { snapshot: next })
            } else {
                CsvMutation::InsertRecord(crate::schema::mutations::insert_record::InsertRecord { index: snapshot.records.len(), record: row })
            }
        }
        CsvEditorCommand::RemoveRow { row, .. } => {
            let index = grid_row_to_record_index(snapshot.has_header, *row);
            if snapshot.records.get(index).is_none() {
                return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.csv.row-stale"), format!("CSV row {row} no longer exists")));
            }
            CsvMutation::RemoveRecord(crate::schema::mutations::remove_record::RemoveRecord { index })
        }
        CsvEditorCommand::AddColumn { .. } => {
            let mut next = snapshot.clone();
            if next.records.is_empty() {
                next.records.push(CsvRecord::default());
            }
            for record in &mut next.records {
                record.fields.push(CsvField::default());
            }
            CsvMutation::SetSnapshot(crate::schema::mutations::set_snapshot::SetSnapshot { snapshot: next })
        }
        CsvEditorCommand::RemoveColumn { column, .. } => {
            let column = *column as usize;
            if !snapshot.records.iter().any(|record| column < record.fields.len()) {
                return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.csv.column-stale"), format!("CSV column {column} no longer exists")));
            }
            let mut next = snapshot.clone();
            for record in &mut next.records {
                if column < record.fields.len() {
                    record.fields.remove(column);
                }
            }
            CsvMutation::SetSnapshot(crate::schema::mutations::set_snapshot::SetSnapshot { snapshot: next })
        }
        CsvEditorCommand::SetHeader { column, value, .. } => {
            let column = *column as usize;
            if snapshot.has_header {
                let field = snapshot
                    .records
                    .first()
                    .and_then(|record| record.fields.get(column))
                    .ok_or_else(|| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.csv.header-stale"), format!("CSV header {column} no longer exists")))?;
                if field.value == *value {
                    return Ok(Emit::default());
                }
                CsvMutation::SetField(crate::schema::mutations::set_field::SetField { record_index: 0, field_index: column, value: value.clone(), quoted: field.quoted })
            } else {
                let mut next = snapshot.clone();
                let width = next.records.iter().map(|record| record.fields.len()).max().unwrap_or(0).max(column + 1);
                let mut header = CsvRecord { fields: vec![CsvField::default(); width] };
                header.fields[column].value = value.clone();
                next.records.insert(0, header);
                next.has_header = true;
                CsvMutation::SetSnapshot(crate::schema::mutations::set_snapshot::SetSnapshot { snapshot: next })
            }
        }
        CsvEditorCommand::EditSnapshot { .. } | CsvEditorCommand::SetActiveExample { .. } => unreachable!(),
    };
    Ok(Emit { artifact_mutations: vec![mutation], ..Default::default() })
}

#[expect(clippy::too_many_arguments, reason = "Implements the framework ArtifactCommandReducer callback signature.")]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn csv_retained_reduce(
    command: &CsvEditorCommand,
    snapshot: &CsvSnapshot,
    _config: &NoConfig,
    _history: &semio_framework_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<CsvEditor>>>,
    operation: &AppOperationContext,
) -> Result<Emit<CsvMutation, NoConfigMutation, NoDraftMutation>, Fault> {
    let revision = semio_s_artifact_stdio_contract::window_kit_canonical_revision(operation.canonical_base_revision);
    csv_emit_at_revision(command, snapshot, Some(&revision))
}

struct CsvRetainedCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl CsvRetainedCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: CSV_RETAINED_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl ToolJobFactory for CsvRetainedCommandJobFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<CsvEditor>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<CsvEditor>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }
    fn payload_schema_id(&self) -> &str {
        CSV_RETAINED_PAYLOAD_SCHEMA
    }
    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }
    fn execution_contract(&self) -> ToolExecutionContract {
        csv_retained_contract()
    }
    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
        Ok(ArtifactRetainedCommandJob::new(payload))
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework_plugin::action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework_plugin::action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (ToolJobFactoryError, semio_framework_plugin::action_bus::RetainedToolWireInput, Option<semio_framework_plugin::action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > CSV_RETAINED_RAW_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("stdio csv retained command rejects oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl ArtifactOwnedToolJobFactory for CsvRetainedCommandJobFactory {
    type Owner = EditorApp<CsvEditor>;
    const TOOL_IDS: &'static [&'static str] = CSV_RETAINED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = STDIO_CSV_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = CSV_RETAINED_PUBLICATION_CONTRACTS;
}
//#endregion 🧵️RetainedRoutes

//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct CsvEditor;

impl ArtifactEditor for CsvEditor {
    /// 📚️ Artifact catalogue stamped by `PluginBuilder::editor` onto the navbar dropdown.
    fn examples() -> Vec<semio_framework_plugin::ExampleSource> {
        vec![crate::examples::demo::source()]
    }
    type Snapshot = CsvSnapshot;
    type Mutation = CsvMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = CsvEditorCommand;

    const DIALECT: Dialect = CSV_EDITOR_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_CSV_DOCUMENT_SCHEMA;

    semio_s_artifact_stdio_contract::snapshot_editing_bounded_first_step_tool_proofs! {
        owner: EditorApp<CsvEditor>,
        owner_file: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.stdio.csv@rfc4180/*#editor",
        artifact_schema: "stdio.csv",
        factory: "CsvRetainedCommandJobFactory",
        factory_type: CsvRetainedCommandJobFactory,
        contract: csv_retained_contract(),
        tools: ["setActiveExample", "set-cell", "add-row", "remove-row", "add-column", "remove-column", "set-header"]
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(CsvRetainedCommandJobFactory::new(&controller))?;
        semio_s_artifact_stdio_contract::editing::register_snapshot_edit_tool_factory::<Self>(registry)
    }

    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<ToolOperationSpec>, Fault> {
        if semio_s_artifact_stdio_contract::editing::is_snapshot_edit_action(&request.tool_id) {
            return semio_s_artifact_stdio_contract::editing::build_snapshot_edit_tool_job::<Self>(request);
        }
        if !CSV_RETAINED_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        if csv_command_id(&request.command) != request.tool_id {
            return Err(Fault::from("stdio-csv-retained-command-tool-mismatch"));
        }
        let tool_id = csv_command_id(&request.command);
        let operation = AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id,
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
            authoring_seed: request.authoring_seed.clone(),
        };
        let payload = ArtifactRetainedCommandPayload::try_new(
            ArtifactRetainedCommandInputs {
                command: *request.command,
                snapshot: request.snapshot,
                config: request.config,
                history: request.history,
                interaction_state: request.interaction_state,
                interaction_hover: request.interaction_hover,
                context: Some(request.context),
                operation,
                completion: request.completion,
            },
            csv_command_id,
            CSV_RETAINED_RAW_BYTES,
            1,
            Box::new(BoundedArtifactCommandWork::new(tool_id, csv_retained_reduce, csv_retained_extent)),
        )?;
        Ok(Some(ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    fn build_document_store_owners() -> Option<store::DocumentStoreOwners<Self::Snapshot, Self::Mutation>> {
        Some(semio_framework_plugin::bounded_document_store_owners::<Self::Snapshot, Self::Mutation>())
    }

    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(semio_framework_plugin::bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
    }

    /// 📤️ The artifact lane's one-item publication authority. The kit verb's route declares the
    /// `Artifact` lane, and without this authority every such route fails closed with
    /// `interactive-job.publication-authority-missing`.
    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(semio_framework_plugin::bounded_config_store_one_item_preparation_factory::<Self::Snapshot, Self::Mutation>("stdio-csv-artifact-retained", store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES))
    }

    /// 🧹️ The rest of the close protocol installing a document owner implies: an app that owns its
    /// document store must own EVERY store it opens, or `close_step` refuses with
    /// `interactive-job.close-owned-disposer-missing`. Every one of these lanes is a `No…` unit type
    /// here, so each takes the framework's own empty-terminal owner.
    fn build_config_store_owners() -> Option<store::DocumentStoreOwners<Self::Config, Self::ConfigMutation>> {
        Some(semio_framework_plugin::no_config_store_owners())
    }

    fn build_config_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        Some(semio_framework_plugin::no_config_store_disposer())
    }

    fn build_draft_store_owners() -> Option<store::DocumentStoreOwners<Self::Draft, Self::DraftMutation>> {
        Some(semio_framework_plugin::no_draft_store_owners())
    }

    fn build_draft_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::DraftStore<Self::Draft, Self::DraftMutation>>>> {
        Some(semio_framework_plugin::no_draft_store_disposer())
    }

    fn build_presence_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
        Some(semio_framework_plugin::no_presence_store_disposer())
    }

    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::no_presence_local_root_retirement_factory())
    }

    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::no_presence_peer_retirement_factory())
    }

    fn build_transient_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
        Some(semio_framework_plugin::no_transient_store_disposer())
    }

    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        Some(semio_framework_plugin::no_transient_local_root_retirement_factory())
    }

    /// 🏗️ Admits the whole-document replacement the example switch emits. The trait default refuses
    /// the envelope, which answers every `setActiveExample` with
    /// `artifact-store.persisted-initializer-refused` at the archive-load boundary.
    #[allow(clippy::result_large_err, reason = "Mirrors the framework trait signature, which returns the original envelope on refusal.")]

    fn command_id(command: &Self::Command) -> &'static str {
        csv_command_id(command)
    }

    fn command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<Self::Command, Fault> {
        csv_command_from_action(action, args)
    }

    fn initial_snapshot() -> CsvSnapshot {
        CsvSnapshot::default()
    }

    /// ✏️ Maps the rendered grid's `row` back to `CsvSnapshot.records`' real index — `+1` when
    /// `has_header` (row 0 in the grid is `records[1]`), unchanged otherwise. Out-of-range is a
    /// documented no-op (`Emit::default()`), never a panic.
    fn handle(
        command: &Self::Command,
        doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &semio_framework_2d::compute::EngineHandles,
    ) -> Result<Emit<Self::Mutation>, Fault> {
        if let Some(event) = <Self as semio_s_artifact_stdio_contract::editing::SnapshotEditingEditor>::snapshot_edit_event(command) {
            return <Self as semio_s_artifact_stdio_contract::editing::SnapshotEditingEditor>::snapshot_edit_emit(event, doc.snapshot);
        }
        let revision = doc.operation_optional().map(|operation| semio_s_artifact_stdio_contract::window_kit_canonical_revision(operation.canonical_base_revision));
        csv_emit_at_revision(command, doc.snapshot, revision.as_deref())
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        match body_key {
            main::BODY_KEY => {
                let revision = doc
                    .render_operation()
                    .map(|operation| semio_s_artifact_stdio_contract::window_kit_canonical_revision(operation.canonical_base_revision))
                    .unwrap_or_else(|| semio_s_artifact_stdio_contract::window_kit_snapshot_revision(doc.snapshot));
                main::render_revisioned(doc.snapshot, &revision, view_state.locale, &semio_framework_plugin::TreeWindows::for_body(view_state, main::BODY_KEY)).map(semio_framework_plugin::built_to_component_tree)
            }
            semio_s_artifact_stdio_contract::editing::SNAPSHOT_DETAILS_BODY_KEY => semio_s_artifact_stdio_contract::editing::render_snapshot_details(
                doc.snapshot,
                view_state.locale,
                "s.stdio.csv@rfc4180/*#editor",
                &semio_framework_plugin::TreeWindows::for_body(view_state, semio_s_artifact_stdio_contract::editing::SNAPSHOT_DETAILS_BODY_KEY),
            )
            .map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}

impl semio_s_artifact_stdio_contract::editing::SnapshotEditingEditor for CsvEditor {
    fn snapshot_edit_event(command: &Self::Command) -> Option<&SnapshotEditEvent> {
        match command {
            CsvEditorCommand::EditSnapshot { event } => Some(event),
            _ => None,
        }
    }

    fn snapshot_edit_mutations(event: &SnapshotEditEvent, snapshot: &Self::Snapshot) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, Fault> {
        semio_s_artifact_stdio_contract::editing::snapshot_edit_patch(event, snapshot, |patch| CsvMutation::PatchSnapshot(crate::schema::mutations::patch_snapshot::PatchSnapshot { patch }))
    }
}
//#endregion 🔖️Editor

//#region 🔖️Manifest
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_csv_editor() -> semio_framework_plugin::AppDefinition {
    let builder = Editor::builder(CSV_EDITOR_DIALECT)
        .document(["semio", "stdio", "csv"])
        .icon_id("table-2")
        .mode_def(edit::definition())
        .default_mode_id(edit::CSV_EDIT_MODE_ID)
        .window_kind_def(main::definition())
        .window_kind_def(semio_s_artifact_stdio_contract::editing::snapshot_details_window_definition())
        .default_layout(semio_s_artifact_stdio_contract::editing::snapshot_details_split_layout(main::WINDOW_KIND_ID, "Table"))
        // 🎬️ Example picker — one option per example `register_apps` publishes for this dialect.
        .action_with(semio_s_artifact_stdio_contract::set_active_example_action())
        .action_args(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, semio_s_artifact_stdio_contract::set_active_example_args(&[(crate::examples::demo::ID, crate::examples::demo::label())], crate::examples::demo::ID))
        .action_destructive(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID)
        .action_describe(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, semio_s_artifact_stdio_contract::set_active_example_description())
        .action_interactive_job(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, InteractiveJobClassification::Migrated);
    semio_s_artifact_stdio_contract::editing::snapshot_edit_actions_with(builder).build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🛜️i-json/✏️editor/🦀️.rs

```rust
//! ✏️ Json editor — the FIRST authored `ArtifactEditor` surface for `s.stdio.json@rfc8259/*`
//! (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET). One real window, `🪟️main`
//! (`TreeWindowKit`), directly editing ANY node of `JsonSnapshot.value` through the artifact's own
//! `JsonMutation::SetScalar` — the frozen `set-node` command always replaces the whole subtree
//! addressed by the node's path, never merges into an existing object/array (documented scope: this
//! is a "replace this node" editor, not a structural insert/remove editor — `SetMember`/
//! `RemoveMember`/`InsertArrayElement`/`RemoveArrayElement` stay unreachable through this window).

use crate::editor::json_i_json::modes::edit;
use crate::editor::json_i_json::modes::edit::windows::main;
use crate::{JsonMutation, JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};
use semio_framework_plugin::retained_command::{ArtifactRetainedCommandInputs, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
use semio_framework_plugin::AppOperationContext;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::ArtifactOwnedToolJobFactory;
use semio_framework_plugin::ArtifactOwnedToolJobRequest;
use semio_framework_plugin::ArtifactStoreInitializationJob;
use semio_framework_plugin::ArtifactToolFactoryRegistry;
use semio_framework_plugin::ArtifactToolPublicationContract;
use semio_framework_plugin::ArtifactToolPublicationLane;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::Dialect;
use semio_framework_plugin::DraftView;
use semio_framework_plugin::Editor;
use semio_framework_plugin::EditorApp;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::InteractiveJobClassification;
use semio_framework_ui_locale::Label;
use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use semio_framework_plugin::NoDraft;
use semio_framework_plugin::NoDraftMutation;
use semio_framework_plugin::NoPresence;
use semio_framework_plugin::NoPresenceMutation;
use semio_framework_plugin::NoTransient;
use semio_framework_plugin::NoTransientMutation;
use semio_framework_plugin::StandardId;
use semio_framework_plugin::SubsetId;
use semio_framework_plugin::ToolExecutionContract;
use semio_framework_plugin::ToolFactoryKey;
use semio_framework_plugin::ToolJobFactory;
use semio_framework_plugin::ToolJobFactoryError;
use semio_framework_plugin::ToolOperationSpec;
use semio_s_artifact_stdio_contract::editing::SnapshotEditEvent;

//#region 🔖️Dialect
/// 🪪️ Artifact coordinate — verified against the artifact's own `🚪️io`/`🧬️schema` `DIALECT`
/// consts. Duplicated (not imported) in the sibling `👁️viewer` surface root.
pub const JSON_I_JSON_EDITOR_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("i-json") };
//#endregion 🔖️Dialect

//#region 🔖️Command
/// ✏️ The editor's typed command channel — exactly the one edit `🪟️main`'s `editable_window_kind()`
/// action (`set-node`, contract §2.6) can trigger. `node_id` is the window's own `k=`/`i=` path
/// encoding (see `main::encode_path_id`), decoded back into a real `JsonPath` in `handle`.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub enum JsonIJsonIJsonEditorCommand {
    SetNode {
        node_id: String,
        revision: String,
        value: String,
    },
    EditSnapshot {
        event: SnapshotEditEvent,
    },
    /// 🎬️ The navbar example picker's payload — see the `🧵️RetainedRoutes` region below.
    SetActiveExample {
        example_id: String,
    },
}

/// 🧭️ `main::encode_path_id`'s inverse. `node_id` is the addressed node's window PATH — its
/// ancestors' sibling keys and its own, joined by `TREE_WINDOW_PATH_SEPARATOR` — and
/// `main::JSON_ROOT_NODE_ID` alone is the root.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode_path_id(node_id: &str) -> Result<String, String> {
    if node_id == main::JSON_ROOT_NODE_ID {
        return Ok("/value".into());
    }
    let separator = semio_framework_plugin::TREE_WINDOW_PATH_SEPARATOR;
    let mut segments = node_id.split(separator);
    if segments.next() != Some(main::JSON_ROOT_NODE_ID) {
        return Err("i-json editor command: node path must begin with exactly one root segment".into());
    }
    segments.try_fold("/value".to_string(), |mut path, segment| {
        if segment.is_empty() || segment == main::JSON_ROOT_NODE_ID {
            return Err(format!("i-json editor command: non-canonical path segment {segment:?}"));
        }
        let (field, raw) = segment.strip_prefix("m=").map(|value| ("members", value)).or_else(|| segment.strip_prefix("i=").map(|value| ("items", value))).ok_or_else(|| format!("json editor command: bad path segment {segment:?}"))?;
        let index = raw.parse::<usize>().map_err(|error| error.to_string())?;
        if index.to_string() != raw {
            return Err(format!("json editor command: non-canonical path segment {segment:?}"));
        }
        path.push('/');
        path.push_str(field);
        path.push('/');
        path.push_str(raw);
        if field == "members" {
            path.push_str("/value");
        }
        Ok(path)
    })
}

impl protocol::OpText for JsonIJsonIJsonEditorCommand {
    fn print_op(&self) -> String {
        match self {
            JsonIJsonIJsonEditorCommand::SetNode { node_id, revision, value } => {
                format!("set-node node-id={} revision={} value={}", crate::schema::diff::hex_encode(node_id.as_bytes()), crate::schema::diff::hex_encode(revision.as_bytes()), crate::schema::diff::hex_encode(value.as_bytes()))
            }
            JsonIJsonIJsonEditorCommand::EditSnapshot { event } => format!("snapshot-edit event={}", crate::schema::diff::hex_encode(&<SnapshotEditEvent as protocol::OpBinary>::encode_op(event).expect("snapshot edit event encodes"))),
            JsonIJsonIJsonEditorCommand::SetActiveExample { example_id } => format!("active-example id={}", crate::schema::diff::hex_encode(example_id.as_bytes())),
        }
    }
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        if let Some(rest) = line.strip_prefix("active-example id=") {
            let bytes = crate::schema::diff::hex_decode(rest).map_err(|error| semio_framework_diagnostic::TextError::new(format!("i-json editor command: invalid id hex: {error}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
            let example_id = String::from_utf8(bytes).map_err(|error| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("i-json editor command: invalid id utf8: {error}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
            return Ok(JsonIJsonIJsonEditorCommand::SetActiveExample { example_id });
        }
        if let Some(raw) = line.strip_prefix("snapshot-edit event=") {
            let bytes = crate::schema::diff::hex_decode(raw).map_err(|error| semio_framework_diagnostic::TextError::new(format!("i-json editor command: invalid snapshot edit hex: {error}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
            let event = <SnapshotEditEvent as protocol::OpBinary>::decode_op(&bytes).map_err(|error| semio_framework_diagnostic::TextError::new(format!("i-json editor command: invalid snapshot edit: {error}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
            return Ok(JsonIJsonIJsonEditorCommand::EditSnapshot { event });
        }
        let rest = line.strip_prefix("set-node ").ok_or_else(|| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("json editor command: unknown line {line:?}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
        let mut node_id = None;
        let mut revision = None;
        let mut value = None;
        for token in rest.split(' ') {
            let (key, raw) = token.split_once('=').ok_or_else(|| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("json editor command: bad token {token:?}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
            match key {
                "node-id" => {
                    let bytes = crate::schema::diff::hex_decode(raw).map_err(|error| semio_framework_diagnostic::TextError::new(format!("i-json editor command: invalid node id hex: {error}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
                    node_id = Some(String::from_utf8(bytes).map_err(|error| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("i-json editor command: invalid node id utf8: {error}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?);
                }
                "value" => {
                    let bytes = crate::schema::diff::hex_decode(raw).map_err(|error| semio_framework_diagnostic::TextError::new(format!("i-json editor command: invalid value hex: {error}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
                    value = Some(String::from_utf8(bytes).map_err(|error| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("i-json editor command: invalid value utf8: {error}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?);
                }
                "revision" => {
                    let bytes = crate::schema::diff::hex_decode(raw).map_err(|error| semio_framework_diagnostic::TextError::new(format!("i-json editor command: invalid revision hex: {error}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
                    revision = Some(String::from_utf8(bytes).map_err(|error| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("i-json editor command: invalid revision utf8: {error}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?);
                }
                _ => {}
            }
        }
        let (node_id, revision, value) =
            node_id.zip(revision).zip(value).map(|((node_id, revision), value)| (node_id, revision, value)).ok_or_else(|| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "json editor command: missing node-id/revision/value", semio_framework_diagnostic::TextSpan::at(1, 1)))?;
        Ok(JsonIJsonIJsonEditorCommand::SetNode { node_id, revision, value })
    }
}

impl protocol::OpBinary for JsonIJsonIJsonEditorCommand {
    /// 🎯️ The app-owned retained routes this command channel carries — the join key
    /// `AppActionRegistry::validate_tool_job_rows` demands an exact owner-local proof for. The `TreeWindowKit`
    /// mints `set-node`, but only this editor can reduce it into its own mutation, so it is an
    /// app-owned route exactly like the example switch.
    const TOOL_JOB_IDS: &'static [&'static str] = JSON_I_JSON_COMMAND_TOOL_IDS;

    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(<Self as protocol::OpText>::print_op(self).into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let line = String::from_utf8(bytes.to_vec()).map_err(|error| protocol::ProtocolError::Malformed { what: "json editor command utf8", offset: 0, detail: error.to_string() })?;
        <Self as protocol::OpText>::parse_op(&line).map_err(|error| protocol::ProtocolError::Malformed { what: "json editor command", offset: 0, detail: error.to_string() })
    }
}
//#endregion 🔖️Command

//#region 🧵️RetainedRoutes
/// 🪟️ The verb the `TreeWindowKit` mints for `🪟️main` — declared by the framework, reduced only here.
const JSON_I_JSON_KIT_ACTION_ID: &str = "set-node";
/// 🧵️ The app-owned retained routes this editor declares: the example switch and `set-node`.
/// `validate_ui_dispatch_classification` refuses any verb that is not `Migrated`, and `Migrated`
/// only survives the guest's `interactive-job.catalog-incomplete` boot check when this roster, the
/// publication contracts and the `bounded_first_step_tool_proofs!` block below all name the same
/// ids. Without the kit verb's row the reactor refused every `set-node` with
/// `interactive-job.missing-factory`.
const JSON_I_JSON_RETAINED_TOOL_IDS: &[&str] = &[semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, JSON_I_JSON_KIT_ACTION_ID];
const JSON_I_JSON_COMMAND_TOOL_IDS: &[&str] = &[
    semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID,
    JSON_I_JSON_KIT_ACTION_ID,
    semio_s_artifact_stdio_contract::editing::SET_SNAPSHOT_VALUE_ACTION_ID,
    semio_s_artifact_stdio_contract::editing::INSERT_SNAPSHOT_VALUE_ACTION_ID,
    semio_s_artifact_stdio_contract::editing::REMOVE_SNAPSHOT_VALUE_ACTION_ID,
    semio_s_artifact_stdio_contract::editing::MOVE_SNAPSHOT_VALUE_ACTION_ID,
    semio_s_artifact_stdio_contract::editing::RENAME_SNAPSHOT_KEY_ACTION_ID,
    semio_s_artifact_stdio_contract::editing::REPLACE_SNAPSHOT_SOURCE_ACTION_ID,
];
const JSON_I_JSON_RETAINED_PAYLOAD_SCHEMA: &str = "stdio.json.i-json.tool-command.v1";
const JSON_I_JSON_RETAINED_RAW_BYTES: usize = 16 * 1_024 * 1_024;
/// 🚦️ The example switch publishes into NO document lane: it hands the host one
/// `Effect::LoadDocument`, so its only lane is `HostOnly`. `set-node` publishes the artifact
/// mutation it reduces into, so its only lane is `Artifact`.
const JSON_I_JSON_RETAINED_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] = &[
    ArtifactToolPublicationContract { tool_id: semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: JSON_I_JSON_KIT_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
];

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn json_i_json_retained_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(JSON_I_JSON_RETAINED_RAW_BYTES, 4_096, 1, JSON_I_JSON_RETAINED_RAW_BYTES, 7_500)
}

/// 📚️ The document a named example loads. The subset publishes exactly one (crate::standards::v_rfc8259::subsets::i_json::examples::demo, `ID = "demo"`),
/// whose asset IS this app's curated document; every other id — including the empty id the shell
/// sends for "the app's own default document" — opens the genesis document.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn json_i_json_example_snapshot(example_id: &str) -> JsonSnapshot {
    if example_id == crate::standards::v_rfc8259::subsets::i_json::examples::demo::ID {
        <JsonSnapshot as store::ArtifactDsl>::parse_dsl(crate::standards::v_rfc8259::subsets::i_json::examples::demo::PRIMARY_TEXT).unwrap_or_default()
    } else {
        JsonSnapshot::default()
    }
}

/// 🌉️ Resolves the react/wgpu shells' `{action, args}` pair into this editor's typed command.
/// `ArtifactEditor::command_from_action`'s default refuses EVERY id, which is why the boot example,
/// every navbar pick and every Actions-pane row died before reaching a command.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn json_i_json_command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<JsonIJsonIJsonEditorCommand, Fault> {
    if let Some(event) = semio_s_artifact_stdio_contract::editing::snapshot_edit_event_from_action(action, args)? {
        return Ok(JsonIJsonIJsonEditorCommand::EditSnapshot { event });
    }
    match action {
        semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID => Ok(JsonIJsonIJsonEditorCommand::SetActiveExample { example_id: semio_s_artifact_stdio_contract::example_id_argument(args, "") }),
        JSON_I_JSON_KIT_ACTION_ID => Ok(JsonIJsonIJsonEditorCommand::SetNode {
            node_id: semio_s_artifact_stdio_contract::window_kit_required_text_argument(args, "nodeId")?,
            revision: semio_s_artifact_stdio_contract::window_kit_required_text_argument(args, "revision")?,
            value: semio_s_artifact_stdio_contract::window_kit_required_text_argument(args, "value")?,
        }),
        other => {
            Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.json.i-json.unhandled-action"), format!("action '{other}' is not one of this editor's declared verbs (setActiveExample, set-node)")))
        }
    }
}

/// 🏷️ The manifest id each command was declared under.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn json_i_json_command_id(command: &JsonIJsonIJsonEditorCommand) -> &'static str {
    match command {
        JsonIJsonIJsonEditorCommand::SetNode { .. } => JSON_I_JSON_KIT_ACTION_ID,
        JsonIJsonIJsonEditorCommand::EditSnapshot { event } => event.action_id(),
        JsonIJsonIJsonEditorCommand::SetActiveExample { .. } => semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID,
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn json_i_json_retained_extent(_command: &JsonIJsonIJsonEditorCommand, _snapshot: &JsonSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    Some(1)
}

/// ✏️ The one reduction `handle` and the retained route share: the example switch hands the host
/// its document, `set-node` becomes this artifact's own mutation.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn json_i_json_emit(command: &JsonIJsonIJsonEditorCommand, snapshot: &JsonSnapshot, canonical_revision: Option<[u8; 32]>) -> Result<Emit<JsonMutation, NoConfigMutation, NoDraftMutation>, Fault> {
    if let JsonIJsonIJsonEditorCommand::EditSnapshot { event } = command {
        return <JsonIJsonEditor as semio_s_artifact_stdio_contract::editing::SnapshotEditingEditor>::snapshot_edit_emit(event, snapshot);
    }
    let (node_id, revision, value) = match command {
        JsonIJsonIJsonEditorCommand::SetActiveExample { example_id } => {
            return Ok(Emit { effects: vec![semio_s_artifact_stdio_contract::load_example_effect(&json_i_json_example_snapshot(example_id), STDIO_JSON_DOCUMENT_SCHEMA)], ..Default::default() })
        }
        JsonIJsonIJsonEditorCommand::SetNode { node_id, revision, value } => (node_id, revision, value),
        JsonIJsonIJsonEditorCommand::EditSnapshot { .. } => unreachable!(),
    };
    let current_revision = canonical_revision.map_or_else(|| semio_s_artifact_stdio_contract::window_kit_snapshot_revision(snapshot), semio_s_artifact_stdio_contract::window_kit_canonical_revision);
    if revision != &current_revision {
        return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.i-json.stale-node-edit"), "the JSON document changed while this node draft was open"));
    }
    let path = decode_path_id(node_id).map_err(|detail| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.i-json.invalid-node-path"), detail))?;
    let parsed = crate::schema::snapshot::parse_json_text(value)
        .map_err(|error| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.i-json.invalid-node-value"), format!("value for node '{node_id}' is not valid JSON: {error}")))?;
    let event = SnapshotEditEvent::SetValue { path, value: dsl::ToValue::to_value(&parsed) };
    <JsonIJsonEditor as semio_s_artifact_stdio_contract::editing::SnapshotEditingEditor>::snapshot_edit_emit(&event, snapshot)
}

#[expect(clippy::too_many_arguments, reason = "Implements the framework ArtifactCommandReducer callback signature.")]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn json_i_json_retained_reduce(
    command: &JsonIJsonIJsonEditorCommand,
    snapshot: &JsonSnapshot,
    _config: &NoConfig,
    _history: &semio_framework_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<JsonIJsonEditor>>>,
    operation: &AppOperationContext,
) -> Result<Emit<JsonMutation, NoConfigMutation, NoDraftMutation>, Fault> {
    json_i_json_emit(command, snapshot, Some(operation.canonical_base_revision))
}

struct JsonIJsonRetainedCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl JsonIJsonRetainedCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: JSON_I_JSON_RETAINED_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl ToolJobFactory for JsonIJsonRetainedCommandJobFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<JsonIJsonEditor>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<JsonIJsonEditor>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }
    fn payload_schema_id(&self) -> &str {
        JSON_I_JSON_RETAINED_PAYLOAD_SCHEMA
    }
    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }
    fn execution_contract(&self) -> ToolExecutionContract {
        json_i_json_retained_contract()
    }
    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
        Ok(ArtifactRetainedCommandJob::new(payload))
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework_plugin::action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework_plugin::action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (ToolJobFactoryError, semio_framework_plugin::action_bus::RetainedToolWireInput, Option<semio_framework_plugin::action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > JSON_I_JSON_RETAINED_RAW_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("stdio json i-json retained command rejects oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl ArtifactOwnedToolJobFactory for JsonIJsonRetainedCommandJobFactory {
    type Owner = EditorApp<JsonIJsonEditor>;
    const TOOL_IDS: &'static [&'static str] = JSON_I_JSON_RETAINED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = STDIO_JSON_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = JSON_I_JSON_RETAINED_PUBLICATION_CONTRACTS;
}
//#endregion 🧵️RetainedRoutes

//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct JsonIJsonEditor;

impl ArtifactEditor for JsonIJsonEditor {
    /// 📚️ Artifact catalogue stamped by `PluginBuilder::editor` onto the navbar dropdown.
    fn examples() -> Vec<semio_framework_plugin::ExampleSource> {
        vec![crate::standards::v_rfc8259::subsets::i_json::examples::demo::source()]
    }
    type Snapshot = JsonSnapshot;
    type Mutation = JsonMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = JsonIJsonIJsonEditorCommand;

    const DIALECT: Dialect = JSON_I_JSON_EDITOR_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_JSON_DOCUMENT_SCHEMA;

    semio_s_artifact_stdio_contract::snapshot_editing_bounded_first_step_tool_proofs! {
        owner: EditorApp<JsonIJsonEditor>,
        owner_file: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🛜️i-json/✏️editor/🦀️.rs",
        controller: "s.stdio.json@rfc8259/i-json#editor",
        artifact_schema: "stdio.json",
        factory: "JsonIJsonRetainedCommandJobFactory",
        factory_type: JsonIJsonRetainedCommandJobFactory,
        contract: json_i_json_retained_contract(),
        tools: ["setActiveExample", "set-node"]
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(JsonIJsonRetainedCommandJobFactory::new(&controller))?;
        semio_s_artifact_stdio_contract::editing::register_snapshot_edit_tool_factory::<Self>(registry)
    }

    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<ToolOperationSpec>, Fault> {
        if semio_s_artifact_stdio_contract::editing::is_snapshot_edit_action(&request.tool_id) {
            return semio_s_artifact_stdio_contract::editing::build_snapshot_edit_tool_job::<Self>(request);
        }
        if !JSON_I_JSON_RETAINED_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        if json_i_json_command_id(&request.command) != request.tool_id {
            return Err(Fault::from("stdio-json-i-json-retained-command-tool-mismatch"));
        }
        let tool_id = json_i_json_command_id(&request.command);
        let operation = AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id,
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
            authoring_seed: request.authoring_seed.clone(),
        };
        let payload = ArtifactRetainedCommandPayload::try_new(
            ArtifactRetainedCommandInputs {
                command: *request.command,
                snapshot: request.snapshot,
                config: request.config,
                history: request.history,
                interaction_state: request.interaction_state,
                interaction_hover: request.interaction_hover,
                context: Some(request.context),
                operation,
                completion: request.completion,
            },
            json_i_json_command_id,
            JSON_I_JSON_RETAINED_RAW_BYTES,
            1,
            Box::new(BoundedArtifactCommandWork::new(tool_id, json_i_json_retained_reduce, json_i_json_retained_extent)),
        )?;
        Ok(Some(ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    fn build_document_store_owners() -> Option<store::DocumentStoreOwners<Self::Snapshot, Self::Mutation>> {
        Some(semio_framework_plugin::bounded_document_store_owners::<Self::Snapshot, Self::Mutation>())
    }

    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(semio_framework_plugin::bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
    }

    /// 📤️ The artifact lane's one-item publication authority. The kit verb's route declares the
    /// `Artifact` lane, and without this authority every such route fails closed with
    /// `interactive-job.publication-authority-missing`.
    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(semio_framework_plugin::bounded_config_store_one_item_preparation_factory::<Self::Snapshot, Self::Mutation>("stdio-json-i-json-artifact-retained", store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES))
    }

    /// 🧹️ The rest of the close protocol installing a document owner implies: an app that owns its
    /// document store must own EVERY store it opens, or `close_step` refuses with
    /// `interactive-job.close-owned-disposer-missing`. Every one of these lanes is a `No…` unit type
    /// here, so each takes the framework's own empty-terminal owner.
    fn build_config_store_owners() -> Option<store::DocumentStoreOwners<Self::Config, Self::ConfigMutation>> {
        Some(semio_framework_plugin::no_config_store_owners())
    }

    fn build_config_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        Some(semio_framework_plugin::no_config_store_disposer())
    }

    fn build_draft_store_owners() -> Option<store::DocumentStoreOwners<Self::Draft, Self::DraftMutation>> {
        Some(semio_framework_plugin::no_draft_store_owners())
    }

    fn build_draft_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::DraftStore<Self::Draft, Self::DraftMutation>>>> {
        Some(semio_framework_plugin::no_draft_store_disposer())
    }

    fn build_presence_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
        Some(semio_framework_plugin::no_presence_store_disposer())
    }

    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::no_presence_local_root_retirement_factory())
    }

    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::no_presence_peer_retirement_factory())
    }

    fn build_transient_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
        Some(semio_framework_plugin::no_transient_store_disposer())
    }

    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        Some(semio_framework_plugin::no_transient_local_root_retirement_factory())
    }

    /// 🏗️ Admits the whole-document replacement the example switch emits. The trait default refuses
    /// the envelope, which answers every `setActiveExample` with
    /// `artifact-store.persisted-initializer-refused` at the archive-load boundary.
    #[allow(clippy::result_large_err, reason = "Mirrors the framework trait signature, which returns the original envelope on refusal.")]

    fn command_id(command: &Self::Command) -> &'static str {
        json_i_json_command_id(command)
    }

    fn command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<Self::Command, Fault> {
        json_i_json_command_from_action(action, args)
    }

    fn initial_snapshot() -> JsonSnapshot {
        JsonSnapshot::default()
    }

    /// ✏️ An unparseable `node_id` is a documented no-op (`Emit::default()`), never a panic. The
    /// node value is parsed as one complete JSON value, preserving null, boolean, number, string,
    /// array and object kinds. Invalid input fails atomically before publishing a mutation.
    fn handle(
        command: &Self::Command,
        doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &semio_framework_2d::compute::EngineHandles,
    ) -> Result<Emit<Self::Mutation>, Fault> {
        if let Some(event) = <Self as semio_s_artifact_stdio_contract::editing::SnapshotEditingEditor>::snapshot_edit_event(command) {
            return <Self as semio_s_artifact_stdio_contract::editing::SnapshotEditingEditor>::snapshot_edit_emit(event, doc.snapshot);
        }
        json_i_json_emit(command, doc.snapshot, doc.operation_optional().map(|operation| operation.canonical_base_revision))
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        match body_key {
            main::BODY_KEY => {
                let revision =
                    doc.render_operation().map_or_else(|| semio_s_artifact_stdio_contract::window_kit_snapshot_revision(doc.snapshot), |operation| semio_s_artifact_stdio_contract::window_kit_canonical_revision(operation.canonical_base_revision));
                main::render_editor(doc.snapshot, view_state.locale, &semio_framework_plugin::TreeWindows::for_body(view_state, main::BODY_KEY), "s.stdio.json@rfc8259/i-json#editor", &revision).map(semio_framework_plugin::built_to_component_tree)
            }
            semio_s_artifact_stdio_contract::editing::SNAPSHOT_DETAILS_BODY_KEY => semio_s_artifact_stdio_contract::editing::render_snapshot_details(
                doc.snapshot,
                view_state.locale,
                "s.stdio.json@rfc8259/i-json#editor",
                &semio_framework_plugin::TreeWindows::for_body(view_state, semio_s_artifact_stdio_contract::editing::SNAPSHOT_DETAILS_BODY_KEY),
            )
            .map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}

impl semio_s_artifact_stdio_contract::editing::SnapshotEditingEditor for JsonIJsonEditor {
    fn snapshot_edit_event(command: &Self::Command) -> Option<&SnapshotEditEvent> {
        match command {
            JsonIJsonIJsonEditorCommand::EditSnapshot { event } => Some(event),
            _ => None,
        }
    }

    fn snapshot_edit_mutations(event: &SnapshotEditEvent, snapshot: &Self::Snapshot) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, Fault> {
        semio_s_artifact_stdio_contract::editing::snapshot_edit_patch(event, snapshot, |patch| JsonMutation::PatchSnapshot(crate::schema::mutations::patch_snapshot::PatchSnapshot { patch }))
    }
}
//#endregion 🔖️Editor

//#region 🔖️Manifest
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_json_i_json_editor() -> semio_framework_plugin::AppDefinition {
    let builder = Editor::builder(JSON_I_JSON_EDITOR_DIALECT)
        .document(["semio", "stdio", "json"])
        .icon_id("list-tree")
        .mode_def(edit::definition())
        .default_mode_id(edit::JSON_I_JSON_EDIT_MODE_ID)
        .window_kind_def(main::definition())
        .window_kind_def(semio_s_artifact_stdio_contract::editing::snapshot_details_window_definition())
        .default_layout(semio_s_artifact_stdio_contract::editing::snapshot_details_split_layout(main::WINDOW_KIND_ID, "Tree"))
        // 🎬️ Example picker — one option per example `register_apps` publishes for this dialect.
        .action_with(semio_s_artifact_stdio_contract::set_active_example_action())
        .action_args(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, semio_s_artifact_stdio_contract::set_active_example_args(&[(crate::standards::v_rfc8259::subsets::i_json::examples::demo::ID, crate::standards::v_rfc8259::subsets::i_json::examples::demo::label())], crate::standards::v_rfc8259::subsets::i_json::examples::demo::ID))
        .action_destructive(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID)
        .action_describe(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, semio_s_artifact_stdio_contract::set_active_example_description())
        .action_interactive_job(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, InteractiveJobClassification::Migrated);
    semio_s_artifact_stdio_contract::editing::snapshot_edit_actions_with(builder).build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/✏️editor/🦀️.rs

```rust
//! ✏️ Json editor — the FIRST authored `ArtifactEditor` surface for `s.stdio.json@rfc8259/*`
//! (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET). One real window, `🪟️main`
//! (`TreeWindowKit`), directly editing ANY node of `JsonSnapshot.value` through the artifact's own
//! a compact typed snapshot patch — the frozen `set-node` command always replaces the whole subtree
//! addressed by the node's path, never merges into an existing object/array (documented scope: this
//! is a "replace this node" editor, not a structural insert/remove editor — `SetMember`/
//! `RemoveMember`/`InsertArrayElement`/`RemoveArrayElement` stay unreachable through this window).

use crate::editor::json_any::modes::edit;
use crate::editor::json_any::modes::edit::windows::main;
use crate::{JsonMutation, JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};
use semio_framework_plugin::retained_command::{ArtifactRetainedCommandInputs, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
use semio_framework_plugin::AppOperationContext;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::ArtifactOwnedToolJobFactory;
use semio_framework_plugin::ArtifactOwnedToolJobRequest;
use semio_framework_plugin::ArtifactStoreInitializationJob;
use semio_framework_plugin::ArtifactToolFactoryRegistry;
use semio_framework_plugin::ArtifactToolPublicationContract;
use semio_framework_plugin::ArtifactToolPublicationLane;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::Dialect;
use semio_framework_plugin::DraftView;
use semio_framework_plugin::Editor;
use semio_framework_plugin::EditorApp;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::InteractiveJobClassification;
use semio_framework_ui_locale::Label;
use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use semio_framework_plugin::NoDraft;
use semio_framework_plugin::NoDraftMutation;
use semio_framework_plugin::NoPresence;
use semio_framework_plugin::NoPresenceMutation;
use semio_framework_plugin::NoTransient;
use semio_framework_plugin::NoTransientMutation;
use semio_framework_plugin::StandardId;
use semio_framework_plugin::SubsetId;
use semio_framework_plugin::ToolExecutionContract;
use semio_framework_plugin::ToolFactoryKey;
use semio_framework_plugin::ToolJobFactory;
use semio_framework_plugin::ToolJobFactoryError;
use semio_framework_plugin::ToolOperationSpec;
use semio_s_artifact_stdio_contract::editing::SnapshotEditEvent;

//#region 🔖️Dialect
/// 🪪️ Artifact coordinate — verified against the artifact's own `🚪️io`/`🧬️schema` `DIALECT`
/// consts. Duplicated (not imported) in the sibling `👁️viewer` surface root.
pub const JSON_EDITOR_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId::ANY };
//#endregion 🔖️Dialect

//#region 🔖️Command
/// ✏️ The editor's typed command channel — exactly the one edit `🪟️main`'s `editable_window_kind()`
/// action (`set-node`, contract §2.6) can trigger. `node_id` is the window's own `k=`/`i=` path
/// encoding (see `main::encode_path_id`), decoded back into a real `JsonPath` in `handle`.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub enum JsonAnyEditorCommand {
    SetNode {
        node_id: String,
        revision: String,
        value: String,
    },
    EditSnapshot {
        event: SnapshotEditEvent,
    },
    /// 🎬️ The navbar example picker's payload — see the `🧵️RetainedRoutes` region below.
    SetActiveExample {
        example_id: String,
    },
}

/// 🧭️ `main::encode_path_id`'s inverse. `node_id` is the addressed node's window PATH — its
/// ancestors' sibling keys and its own, joined by `TREE_WINDOW_PATH_SEPARATOR` — and
/// `main::JSON_ROOT_NODE_ID` alone is the root.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode_path_id(node_id: &str) -> Result<String, String> {
    if node_id == main::JSON_ROOT_NODE_ID {
        return Ok("/value".into());
    }
    let separator = semio_framework_plugin::TREE_WINDOW_PATH_SEPARATOR;
    let mut segments = node_id.split(separator);
    if segments.next() != Some(main::JSON_ROOT_NODE_ID) {
        return Err("json editor command: node path must begin with exactly one root segment".into());
    }
    segments.try_fold("/value".to_string(), |mut path, segment| {
        if segment.is_empty() || segment == main::JSON_ROOT_NODE_ID {
            return Err(format!("json editor command: non-canonical path segment {segment:?}"));
        }
        let (field, raw) = segment.strip_prefix("m=").map(|value| ("members", value)).or_else(|| segment.strip_prefix("i=").map(|value| ("items", value))).ok_or_else(|| format!("json editor command: bad path segment {segment:?}"))?;
        let index = raw.parse::<usize>().map_err(|error| error.to_string())?;
        if index.to_string() != raw {
            return Err(format!("json editor command: non-canonical path segment {segment:?}"));
        }
        path.push('/');
        path.push_str(field);
        path.push('/');
        path.push_str(raw);
        if field == "members" {
            path.push_str("/value");
        }
        Ok(path)
    })
}

impl protocol::OpText for JsonAnyEditorCommand {
    fn print_op(&self) -> String {
        match self {
            JsonAnyEditorCommand::SetNode { node_id, revision, value } => {
                format!("set-node node-id={} revision={} value={}", crate::schema::diff::hex_encode(node_id.as_bytes()), crate::schema::diff::hex_encode(revision.as_bytes()), crate::schema::diff::hex_encode(value.as_bytes()))
            }
            JsonAnyEditorCommand::EditSnapshot { event } => format!("snapshot-edit event={}", crate::schema::diff::hex_encode(&<SnapshotEditEvent as protocol::OpBinary>::encode_op(event).expect("snapshot edit event encodes"))),
            JsonAnyEditorCommand::SetActiveExample { example_id } => format!("active-example id={}", crate::schema::diff::hex_encode(example_id.as_bytes())),
        }
    }
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        if let Some(rest) = line.strip_prefix("snapshot-edit event=") {
            let bytes = crate::schema::diff::hex_decode(rest).map_err(|error| semio_framework_diagnostic::TextError::new(format!("json editor command: invalid snapshot edit hex: {error}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
            let event = <SnapshotEditEvent as protocol::OpBinary>::decode_op(&bytes).map_err(|error| semio_framework_diagnostic::TextError::new(format!("json editor command: invalid snapshot edit: {error}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
            return Ok(JsonAnyEditorCommand::EditSnapshot { event });
        }
        if let Some(rest) = line.strip_prefix("active-example id=") {
            let bytes = crate::schema::diff::hex_decode(rest).map_err(|error| semio_framework_diagnostic::TextError::new(format!("json editor command: invalid id hex: {error}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
            let example_id = String::from_utf8(bytes).map_err(|error| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("json editor command: invalid id utf8: {error}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
            return Ok(JsonAnyEditorCommand::SetActiveExample { example_id });
        }
        let rest = line.strip_prefix("set-node ").ok_or_else(|| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("json editor command: unknown line {line:?}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
        let mut node_id = None;
        let mut revision = None;
        let mut value = None;
        for token in rest.split(' ') {
            let (key, raw) = token.split_once('=').ok_or_else(|| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("json editor command: bad token {token:?}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
            match key {
                "node-id" => {
                    let bytes = crate::schema::diff::hex_decode(raw).map_err(|error| semio_framework_diagnostic::TextError::new(format!("json editor command: invalid node id hex: {error}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
                    node_id = Some(String::from_utf8(bytes).map_err(|error| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("json editor command: invalid node id utf8: {error}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?);
                }
                "value" => {
                    let bytes = crate::schema::diff::hex_decode(raw).map_err(|error| semio_framework_diagnostic::TextError::new(format!("json editor command: invalid value hex: {error}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
                    value = Some(String::from_utf8(bytes).map_err(|error| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("json editor command: invalid value utf8: {error}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?);
                }
                "revision" => {
                    let bytes = crate::schema::diff::hex_decode(raw).map_err(|error| semio_framework_diagnostic::TextError::new(format!("json editor command: invalid revision hex: {error}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
                    revision = Some(String::from_utf8(bytes).map_err(|error| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("json editor command: invalid revision utf8: {error}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?);
                }
                _ => {}
            }
        }
        let (node_id, revision, value) =
            node_id.zip(revision).zip(value).map(|((node_id, revision), value)| (node_id, revision, value)).ok_or_else(|| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "json editor command: missing node-id/revision/value", semio_framework_diagnostic::TextSpan::at(1, 1)))?;
        Ok(JsonAnyEditorCommand::SetNode { node_id, revision, value })
    }
}

impl protocol::OpBinary for JsonAnyEditorCommand {
    /// 🎯️ The app-owned retained routes this command channel carries — the join key
    /// `AppActionRegistry::validate_tool_job_rows` demands an exact owner-local proof for. The `TreeWindowKit`
    /// mints `set-node`, but only this editor can reduce it into its own mutation, so it is an
    /// app-owned route exactly like the example switch.
    const TOOL_JOB_IDS: &'static [&'static str] = JSON_ANY_COMMAND_TOOL_IDS;

    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(<Self as protocol::OpText>::print_op(self).into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let line = String::from_utf8(bytes.to_vec()).map_err(|error| protocol::ProtocolError::Malformed { what: "json editor command utf8", offset: 0, detail: error.to_string() })?;
        <Self as protocol::OpText>::parse_op(&line).map_err(|error| protocol::ProtocolError::Malformed { what: "json editor command", offset: 0, detail: error.to_string() })
    }
}
//#endregion 🔖️Command

//#region 🧵️RetainedRoutes
/// 🪟️ The verb the `TreeWindowKit` mints for `🪟️main` — declared by the framework, reduced only here.
const JSON_ANY_KIT_ACTION_ID: &str = "set-node";
/// 🧵️ The app-owned retained routes this editor declares: the example switch and `set-node`.
/// `validate_ui_dispatch_classification` refuses any verb that is not `Migrated`, and `Migrated`
/// only survives the guest's `interactive-job.catalog-incomplete` boot check when this roster, the
/// publication contracts and the `bounded_first_step_tool_proofs!` block below all name the same
/// ids. Without the kit verb's row the reactor refused every `set-node` with
/// `interactive-job.missing-factory`.
const JSON_ANY_RETAINED_TOOL_IDS: &[&str] = &[semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, JSON_ANY_KIT_ACTION_ID];
const JSON_ANY_COMMAND_TOOL_IDS: &[&str] = &[
    semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID,
    JSON_ANY_KIT_ACTION_ID,
    semio_s_artifact_stdio_contract::editing::SET_SNAPSHOT_VALUE_ACTION_ID,
    semio_s_artifact_stdio_contract::editing::INSERT_SNAPSHOT_VALUE_ACTION_ID,
    semio_s_artifact_stdio_contract::editing::REMOVE_SNAPSHOT_VALUE_ACTION_ID,
    semio_s_artifact_stdio_contract::editing::MOVE_SNAPSHOT_VALUE_ACTION_ID,
    semio_s_artifact_stdio_contract::editing::RENAME_SNAPSHOT_KEY_ACTION_ID,
    semio_s_artifact_stdio_contract::editing::REPLACE_SNAPSHOT_SOURCE_ACTION_ID,
];
const JSON_ANY_RETAINED_PAYLOAD_SCHEMA: &str = "stdio.json.tool-command.v1";
const JSON_ANY_RETAINED_RAW_BYTES: usize = 16 * 1_024 * 1_024;
/// 🚦️ The example switch publishes into NO document lane: it hands the host one
/// `Effect::LoadDocument`, so its only lane is `HostOnly`. `set-node` publishes the artifact
/// mutation it reduces into, so its only lane is `Artifact`.
const JSON_ANY_RETAINED_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] = &[
    ArtifactToolPublicationContract { tool_id: semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: JSON_ANY_KIT_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
];

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn json_any_retained_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(JSON_ANY_RETAINED_RAW_BYTES, 4_096, 1, JSON_ANY_RETAINED_RAW_BYTES, 7_500)
}

/// 📚️ The document a named example loads. The subset publishes exactly one (crate::examples::demo, `ID = "demo"`),
/// whose asset IS this app's curated document; every other id — including the empty id the shell
/// sends for "the app's own default document" — opens the genesis document.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn json_any_example_snapshot(example_id: &str) -> JsonSnapshot {
    if example_id == crate::examples::demo::ID {
        <JsonSnapshot as store::ArtifactDsl>::parse_dsl(crate::examples::demo::PRIMARY_TEXT).unwrap_or_default()
    } else {
        JsonSnapshot::default()
    }
}

/// 🌉️ Resolves the react/wgpu shells' `{action, args}` pair into this editor's typed command.
/// `ArtifactEditor::command_from_action`'s default refuses EVERY id, which is why the boot example,
/// every navbar pick and every Actions-pane row died before reaching a command.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn json_any_command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<JsonAnyEditorCommand, Fault> {
    if let Some(event) = semio_s_artifact_stdio_contract::editing::snapshot_edit_event_from_action(action, args)? {
        return Ok(JsonAnyEditorCommand::EditSnapshot { event });
    }
    match action {
        semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID => Ok(JsonAnyEditorCommand::SetActiveExample { example_id: semio_s_artifact_stdio_contract::example_id_argument(args, "") }),
        JSON_ANY_KIT_ACTION_ID => Ok(JsonAnyEditorCommand::SetNode {
            node_id: semio_s_artifact_stdio_contract::window_kit_required_text_argument(args, "nodeId")?,
            revision: semio_s_artifact_stdio_contract::window_kit_required_text_argument(args, "revision")?,
            value: semio_s_artifact_stdio_contract::window_kit_required_text_argument(args, "value")?,
        }),
        other => Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.json.unhandled-action"), format!("action '{other}' is not one of this editor's declared verbs (setActiveExample, set-node)"))),
    }
}

/// 🏷️ The manifest id each command was declared under.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn json_any_command_id(command: &JsonAnyEditorCommand) -> &'static str {
    match command {
        JsonAnyEditorCommand::SetNode { .. } => JSON_ANY_KIT_ACTION_ID,
        JsonAnyEditorCommand::EditSnapshot { event } => event.action_id(),
        JsonAnyEditorCommand::SetActiveExample { .. } => semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID,
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn json_any_retained_extent(_command: &JsonAnyEditorCommand, _snapshot: &JsonSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    Some(1)
}

/// ✏️ The one reduction `handle` and the retained route share: the example switch hands the host
/// its document, `set-node` becomes this artifact's own mutation.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn json_any_emit(command: &JsonAnyEditorCommand, snapshot: &JsonSnapshot, canonical_revision: Option<[u8; 32]>) -> Result<Emit<JsonMutation, NoConfigMutation, NoDraftMutation>, Fault> {
    if let JsonAnyEditorCommand::EditSnapshot { event } = command {
        return <JsonAnyEditor as semio_s_artifact_stdio_contract::editing::SnapshotEditingEditor>::snapshot_edit_emit(event, snapshot);
    }
    let (node_id, revision, value) = match command {
        JsonAnyEditorCommand::SetActiveExample { example_id } => {
            return Ok(Emit { effects: vec![semio_s_artifact_stdio_contract::load_example_effect(&json_any_example_snapshot(example_id), STDIO_JSON_DOCUMENT_SCHEMA)], ..Default::default() })
        }
        JsonAnyEditorCommand::SetNode { node_id, revision, value } => (node_id, revision, value),
        JsonAnyEditorCommand::EditSnapshot { .. } => unreachable!(),
    };
    let current_revision = canonical_revision.map_or_else(|| semio_s_artifact_stdio_contract::window_kit_snapshot_revision(snapshot), semio_s_artifact_stdio_contract::window_kit_canonical_revision);
    if revision != &current_revision {
        return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.json.stale-node-edit"), "the JSON document changed while this node draft was open"));
    }
    let path = decode_path_id(node_id).map_err(|detail| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.json.invalid-node-path"), detail))?;
    let parsed = crate::schema::snapshot::parse_json_text(value)
        .map_err(|error| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.json.invalid-node-value"), format!("value for node '{node_id}' is not valid JSON: {error}")))?;
    let event = SnapshotEditEvent::SetValue { path, value: dsl::ToValue::to_value(&parsed) };
    <JsonAnyEditor as semio_s_artifact_stdio_contract::editing::SnapshotEditingEditor>::snapshot_edit_emit(&event, snapshot)
}

#[expect(clippy::too_many_arguments, reason = "Implements the framework ArtifactCommandReducer callback signature.")]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn json_any_retained_reduce(
    command: &JsonAnyEditorCommand,
    snapshot: &JsonSnapshot,
    _config: &NoConfig,
    _history: &semio_framework_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<JsonAnyEditor>>>,
    operation: &AppOperationContext,
) -> Result<Emit<JsonMutation, NoConfigMutation, NoDraftMutation>, Fault> {
    json_any_emit(command, snapshot, Some(operation.canonical_base_revision))
}

struct JsonAnyRetainedCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl JsonAnyRetainedCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: JSON_ANY_RETAINED_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl ToolJobFactory for JsonAnyRetainedCommandJobFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<JsonAnyEditor>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<JsonAnyEditor>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }
    fn payload_schema_id(&self) -> &str {
        JSON_ANY_RETAINED_PAYLOAD_SCHEMA
    }
    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }
    fn execution_contract(&self) -> ToolExecutionContract {
        json_any_retained_contract()
    }
    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
        Ok(ArtifactRetainedCommandJob::new(payload))
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework_plugin::action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework_plugin::action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (ToolJobFactoryError, semio_framework_plugin::action_bus::RetainedToolWireInput, Option<semio_framework_plugin::action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > JSON_ANY_RETAINED_RAW_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("stdio json retained command rejects oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl ArtifactOwnedToolJobFactory for JsonAnyRetainedCommandJobFactory {
    type Owner = EditorApp<JsonAnyEditor>;
    const TOOL_IDS: &'static [&'static str] = JSON_ANY_RETAINED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = STDIO_JSON_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = JSON_ANY_RETAINED_PUBLICATION_CONTRACTS;
}
//#endregion 🧵️RetainedRoutes

//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct JsonAnyEditor;

impl ArtifactEditor for JsonAnyEditor {
    /// 📚️ Artifact catalogue stamped by `PluginBuilder::editor` onto the navbar dropdown.
    fn examples() -> Vec<semio_framework_plugin::ExampleSource> {
        vec![crate::examples::demo::source()]
    }
    type Snapshot = JsonSnapshot;
    type Mutation = JsonMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = JsonAnyEditorCommand;

    const DIALECT: Dialect = JSON_EDITOR_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_JSON_DOCUMENT_SCHEMA;

    semio_s_artifact_stdio_contract::snapshot_editing_bounded_first_step_tool_proofs! {
        owner: EditorApp<JsonAnyEditor>,
        owner_file: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/✏️editor/🦀️.rs",
        controller: "s.stdio.json@rfc8259/*#editor",
        artifact_schema: "stdio.json",
        factory: "JsonAnyRetainedCommandJobFactory",
        factory_type: JsonAnyRetainedCommandJobFactory,
        contract: json_any_retained_contract(),
        tools: ["setActiveExample", "set-node"]
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(JsonAnyRetainedCommandJobFactory::new(&controller))?;
        semio_s_artifact_stdio_contract::editing::register_snapshot_edit_tool_factory::<Self>(registry)
    }

    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<ToolOperationSpec>, Fault> {
        if semio_s_artifact_stdio_contract::editing::is_snapshot_edit_action(&request.tool_id) {
            return semio_s_artifact_stdio_contract::editing::build_snapshot_edit_tool_job::<Self>(request);
        }
        if !JSON_ANY_RETAINED_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        if json_any_command_id(&request.command) != request.tool_id {
            return Err(Fault::from("stdio-json-retained-command-tool-mismatch"));
        }
        let tool_id = json_any_command_id(&request.command);
        let operation = AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id,
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
            authoring_seed: request.authoring_seed.clone(),
        };
        let payload = ArtifactRetainedCommandPayload::try_new(
            ArtifactRetainedCommandInputs {
                command: *request.command,
                snapshot: request.snapshot,
                config: request.config,
                history: request.history,
                interaction_state: request.interaction_state,
                interaction_hover: request.interaction_hover,
                context: Some(request.context),
                operation,
                completion: request.completion,
            },
            json_any_command_id,
            JSON_ANY_RETAINED_RAW_BYTES,
            1,
            Box::new(BoundedArtifactCommandWork::new(tool_id, json_any_retained_reduce, json_any_retained_extent)),
        )?;
        Ok(Some(ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    fn build_document_store_owners() -> Option<store::DocumentStoreOwners<Self::Snapshot, Self::Mutation>> {
        Some(semio_framework_plugin::bounded_document_store_owners::<Self::Snapshot, Self::Mutation>())
    }

    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(semio_framework_plugin::bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
    }

    /// 📤️ The artifact lane's one-item publication authority. The kit verb's route declares the
    /// `Artifact` lane, and without this authority every such route fails closed with
    /// `interactive-job.publication-authority-missing`.
    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(semio_framework_plugin::bounded_config_store_one_item_preparation_factory::<Self::Snapshot, Self::Mutation>("stdio-json-artifact-retained", store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES))
    }

    /// 🧹️ The rest of the close protocol installing a document owner implies: an app that owns its
    /// document store must own EVERY store it opens, or `close_step` refuses with
    /// `interactive-job.close-owned-disposer-missing`. Every one of these lanes is a `No…` unit type
    /// here, so each takes the framework's own empty-terminal owner.
    fn build_config_store_owners() -> Option<store::DocumentStoreOwners<Self::Config, Self::ConfigMutation>> {
        Some(semio_framework_plugin::no_config_store_owners())
    }

    fn build_config_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        Some(semio_framework_plugin::no_config_store_disposer())
    }

    fn build_draft_store_owners() -> Option<store::DocumentStoreOwners<Self::Draft, Self::DraftMutation>> {
        Some(semio_framework_plugin::no_draft_store_owners())
    }

    fn build_draft_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::DraftStore<Self::Draft, Self::DraftMutation>>>> {
        Some(semio_framework_plugin::no_draft_store_disposer())
    }

    fn build_presence_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
        Some(semio_framework_plugin::no_presence_store_disposer())
    }

    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::no_presence_local_root_retirement_factory())
    }

    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::no_presence_peer_retirement_factory())
    }

    fn build_transient_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
        Some(semio_framework_plugin::no_transient_store_disposer())
    }

    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        Some(semio_framework_plugin::no_transient_local_root_retirement_factory())
    }

    /// 🏗️ Admits the whole-document replacement the example switch emits. The trait default refuses
    /// the envelope, which answers every `setActiveExample` with
    /// `artifact-store.persisted-initializer-refused` at the archive-load boundary.
    #[allow(clippy::result_large_err, reason = "Mirrors the framework trait signature, which returns the original envelope on refusal.")]

    fn command_id(command: &Self::Command) -> &'static str {
        json_any_command_id(command)
    }

    fn command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<Self::Command, Fault> {
        json_any_command_from_action(action, args)
    }

    fn initial_snapshot() -> JsonSnapshot {
        JsonSnapshot::default()
    }

    /// ✏️ An unparseable `node_id` is a documented no-op (`Emit::default()`), never a panic. The
    /// node value is parsed as one complete JSON value, preserving null, boolean, number, string,
    /// array and object kinds. Invalid input fails atomically before publishing a mutation.
    fn handle(
        command: &Self::Command,
        doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &semio_framework_2d::compute::EngineHandles,
    ) -> Result<Emit<Self::Mutation>, Fault> {
        if let Some(event) = <Self as semio_s_artifact_stdio_contract::editing::SnapshotEditingEditor>::snapshot_edit_event(command) {
            return <Self as semio_s_artifact_stdio_contract::editing::SnapshotEditingEditor>::snapshot_edit_emit(event, doc.snapshot);
        }
        json_any_emit(command, doc.snapshot, doc.operation_optional().map(|operation| operation.canonical_base_revision))
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        match body_key {
            main::BODY_KEY => {
                let revision =
                    doc.render_operation().map_or_else(|| semio_s_artifact_stdio_contract::window_kit_snapshot_revision(doc.snapshot), |operation| semio_s_artifact_stdio_contract::window_kit_canonical_revision(operation.canonical_base_revision));
                main::render_editor(doc.snapshot, view_state.locale, &semio_framework_plugin::TreeWindows::for_body(view_state, main::BODY_KEY), "s.stdio.json@rfc8259/*#editor", &revision).map(semio_framework_plugin::built_to_component_tree)
            }
            semio_s_artifact_stdio_contract::editing::SNAPSHOT_DETAILS_BODY_KEY => semio_s_artifact_stdio_contract::editing::render_snapshot_details(
                doc.snapshot,
                view_state.locale,
                "s.stdio.json@rfc8259/*#editor",
                &semio_framework_plugin::TreeWindows::for_body(view_state, semio_s_artifact_stdio_contract::editing::SNAPSHOT_DETAILS_BODY_KEY),
            )
            .map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}

impl semio_s_artifact_stdio_contract::editing::SnapshotEditingEditor for JsonAnyEditor {
    fn snapshot_edit_event(command: &Self::Command) -> Option<&SnapshotEditEvent> {
        match command {
            JsonAnyEditorCommand::EditSnapshot { event } => Some(event),
            _ => None,
        }
    }

    fn snapshot_edit_mutations(event: &SnapshotEditEvent, snapshot: &Self::Snapshot) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, Fault> {
        semio_s_artifact_stdio_contract::editing::snapshot_edit_patch(event, snapshot, |patch| JsonMutation::PatchSnapshot(crate::schema::mutations::patch_snapshot::PatchSnapshot { patch }))
    }
}
//#endregion 🔖️Editor

//#region 🔖️Manifest
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_json_editor() -> semio_framework_plugin::AppDefinition {
    let builder = Editor::builder(JSON_EDITOR_DIALECT)
        .document(["semio", "stdio", "json"])
        .icon_id("list-tree")
        .mode_def(edit::definition())
        .default_mode_id(edit::JSON_EDIT_MODE_ID)
        .window_kind_def(main::definition())
        .window_kind_def(semio_s_artifact_stdio_contract::editing::snapshot_details_window_definition())
        .default_layout(semio_s_artifact_stdio_contract::editing::snapshot_details_split_layout(main::WINDOW_KIND_ID, "Tree"))
        // 🎬️ Example picker — one option per example `register_apps` publishes for this dialect.
        .action_with(semio_s_artifact_stdio_contract::set_active_example_action())
        .action_args(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, semio_s_artifact_stdio_contract::set_active_example_args(&[(crate::examples::demo::ID, crate::examples::demo::label())], crate::examples::demo::ID))
        .action_destructive(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID)
        .action_describe(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, semio_s_artifact_stdio_contract::set_active_example_description())
        .action_interactive_job(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, InteractiveJobClassification::Migrated);
    semio_s_artifact_stdio_contract::editing::snapshot_edit_actions_with(builder).build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🦀️.rs

```rust
//! 🔺️ PdfDiff (1.4) — handcrafted sparse diff over the document's page tree.
//!
//! 📚️ **Why this is a collection triple and not three flat fields.** `PdfSnapshot` carries
//! `pages: Vec<PageDoc>` (a real PDF 1.4 document has a real page tree — see
//! `../📸️snapshot/🦀️.rs`), so its diff is the recipe's INDEX-KEYED triple:
//! `removed`/`modified`/`added`, `modified` carrying a sparse [`PdfPageDiff`] and `added` carrying
//! a whole [`PageDoc`]. That is what makes "delete page 12" a three-byte diff instead of a
//! whole-document replacement, and it is why there is still no `snapshot: Option<PdfSnapshot>`
//! full-replace slot on `PdfDiff`; document differences stay sparse and page-addressed.
//!
//! ✍️ **Why the codecs are handcrafted rather than derived.** `#[derive(dsl::DslDiff)]` generates a
//! printer whose exact token shape this module does not choose, and the three facet files next to
//! it (`📝️text/📖️.grammar.semio`, `💾️binary/📡️.protocol.semio`) have to state that
//! shape production for production. The sibling 1.7 standard hand-rolls its own `DiffCodec` for the
//! same reason and its grammar file is written from its own `format!` call sites; this one follows
//! it exactly, at 1.4's own much smaller field set (`W`=width, `H`=height, `X`=text).

use crate::standards::v1_4::subsets::base::schema::snapshot::{PageDoc, PdfSnapshot};
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use std::collections::{HashMap, HashSet};

//#region 🔖️PageDiff
/// 📄️ Sparse per-field patch for one [`PageDoc`] — a WEAK entity per the recipe (a value struct,
/// never sub-diffed beyond its own flat fields). No tri-state field exists: `PageDoc` has no
/// optional field of its own.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfPageDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_page_diff(page: &mut PageDoc, diff: &PdfPageDiff) {
    if let Some(value) = diff.width {
        page.width = value;
    }
    if let Some(value) = diff.height {
        page.height = value;
    }
    if let Some(value) = &diff.text {
        page.text = value.clone();
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn page_diff_between(a: &PageDoc, b: &PageDoc) -> PdfPageDiff {
    PdfPageDiff { width: (a.width != b.width).then_some(b.width), height: (a.height != b.height).then_some(b.height), text: (a.text != b.text).then(|| b.text.clone()) }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn is_page_diff_empty(diff: &PdfPageDiff) -> bool {
    diff == &PdfPageDiff::default()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_page_diff(base: &mut PdfPageDiff, other: PdfPageDiff) {
    if other.width.is_some() {
        base.width = other.width;
    }
    if other.height.is_some() {
        base.height = other.height;
    }
    if other.text.is_some() {
        base.text = other.text;
    }
}
//#endregion 🔖️PageDiff

//#region 🔖️PagesTriple
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfPageModified {
    pub index: usize,
    pub diff: PdfPageDiff,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfPageAdded {
    pub index: usize,
    pub page: PageDoc,
}

/// 📦️ Index-keyed `pages` triple (positional — the recipe's "index usize" key kind).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfPagesDiff {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<PdfPageModified>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<PdfPageAdded>,
}

impl PdfPagesDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }
}

/// ▶️ Apply semantics (normative): `removed`/`modified` indices refer to BASE state (removals
/// processed descending); `added` indices refer to FINAL state (ascending insert).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_pages_diff(diff: &PdfPagesDiff, base: &[PageDoc]) -> Vec<PageDoc> {
    let mut pages: Vec<PageDoc> = base.to_vec();
    for modified in &diff.modified {
        if let Some(page) = pages.get_mut(modified.index) {
            apply_page_diff(page, &modified.diff);
        }
    }
    let mut removed_sorted = diff.removed.clone();
    removed_sorted.sort_unstable();
    removed_sorted.dedup();
    for index in removed_sorted.into_iter().rev() {
        if index < pages.len() {
            pages.remove(index);
        }
    }
    let mut added_sorted: Vec<&PdfPageAdded> = diff.added.iter().collect();
    added_sorted.sort_by_key(|added| added.index);
    for added in added_sorted {
        pages.insert(added.index.min(pages.len()), added.page.clone());
    }
    pages
}

/// 🧭️ `between` matching for index-keyed collections (recipe): pairwise `0..min(len)` as
/// `modified`, base tail as `removed`, other tail as `added`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn pages_diff_between(a: &[PageDoc], b: &[PageDoc]) -> PdfPagesDiff {
    let common = a.len().min(b.len());
    let mut modified = Vec::new();
    for index in 0..common {
        let diff = page_diff_between(&a[index], &b[index]);
        if !is_page_diff_empty(&diff) {
            modified.push(PdfPageModified { index, diff });
        }
    }
    let removed: Vec<usize> = if a.len() > b.len() { (b.len()..a.len()).collect() } else { Vec::new() };
    let added: Vec<PdfPageAdded> = if b.len() > a.len() { (a.len()..b.len()).map(|index| PdfPageAdded { index, page: b[index].clone() }).collect() } else { Vec::new() };
    PdfPagesDiff { removed, modified, added }
}

/// ➕️ Index-transported absorb via symbolic position simulation — the recipe's canonical algorithm
/// (`Insert+Remove-before`, `Insert+Insert` at one index both surviving, `Add+SetField` patching
/// into the carried added payload), specialized to the flat [`PdfPageDiff`] because a page is a
/// weak entity with no nested collection of its own.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_pages_diff(first: &PdfPagesDiff, second: &PdfPagesDiff) -> PdfPagesDiff {
    enum Origin {
        Base(usize),
        FirstAdded(usize),
    }
    enum AfterSlot {
        Base { original: usize, diff: Option<PdfPageDiff> },
        FirstAdded { tag: usize, patch: Option<PdfPageDiff> },
        SecondAdded(PageDoc),
    }

    let max_referenced = first
        .removed
        .iter()
        .copied()
        .chain(first.modified.iter().map(|item| item.index))
        .chain(first.added.iter().map(|item| item.index))
        .chain(second.removed.iter().copied())
        .chain(second.modified.iter().map(|item| item.index))
        .chain(second.added.iter().map(|item| item.index))
        .max()
        .unwrap_or(0);
    let simulated = max_referenced + first.removed.len() + second.removed.len() + 64;

    let mut middle: Vec<Origin> = (0..simulated).map(Origin::Base).collect();
    let mut first_removed = first.removed.clone();
    first_removed.sort_unstable();
    first_removed.dedup();
    for index in first_removed.iter().rev() {
        if *index < middle.len() {
            middle.remove(*index);
        }
    }
    let mut first_added_order: Vec<usize> = (0..first.added.len()).collect();
    first_added_order.sort_by_key(|tag| first.added[*tag].index);
    for tag in first_added_order {
        let position = first.added[tag].index.min(middle.len());
        middle.insert(position, Origin::FirstAdded(tag));
    }
    let first_modified: HashMap<usize, PdfPageDiff> = first.modified.iter().map(|item| (item.index, item.diff.clone())).collect();

    let mut after: Vec<AfterSlot> = middle
        .iter()
        .map(|origin| match origin {
            Origin::Base(original) => AfterSlot::Base { original: *original, diff: first_modified.get(original).cloned() },
            Origin::FirstAdded(tag) => AfterSlot::FirstAdded { tag: *tag, patch: None },
        })
        .collect();

    let mut final_removed: Vec<usize> = first.removed.clone();
    let mut second_removed = second.removed.clone();
    second_removed.sort_unstable();
    second_removed.dedup();
    for index in second_removed.iter().rev() {
        if *index < after.len() {
            if let AfterSlot::Base { original, .. } = after.remove(*index) {
                final_removed.push(original);
            }
        }
    }
    for modified in &second.modified {
        if let Some(slot) = after.get_mut(modified.index) {
            let target = match slot {
                AfterSlot::Base { diff, .. } => Some(diff),
                AfterSlot::FirstAdded { patch, .. } => Some(patch),
                AfterSlot::SecondAdded(_) => None,
            };
            if let Some(target) = target {
                let combined = match target.take() {
                    Some(mut existing) => {
                        absorb_page_diff(&mut existing, modified.diff.clone());
                        existing
                    }
                    None => modified.diff.clone(),
                };
                *target = (!is_page_diff_empty(&combined)).then_some(combined);
            }
        }
    }
    let mut second_added_order: Vec<usize> = (0..second.added.len()).collect();
    second_added_order.sort_by_key(|tag| second.added[*tag].index);
    for tag in second_added_order {
        let position = second.added[tag].index.min(after.len());
        after.insert(position, AfterSlot::SecondAdded(second.added[tag].page.clone()));
    }

    let mut modified = Vec::new();
    let mut added = Vec::new();
    for (position, slot) in after.into_iter().enumerate() {
        match slot {
            AfterSlot::Base { original, diff: Some(diff) } => modified.push(PdfPageModified { index: original, diff }),
            AfterSlot::Base { .. } => {}
            AfterSlot::FirstAdded { tag, patch } => {
                let mut page = first.added[tag].page.clone();
                if let Some(patch) = patch {
                    apply_page_diff(&mut page, &patch);
                }
                added.push(PdfPageAdded { index: position, page });
            }
            AfterSlot::SecondAdded(page) => added.push(PdfPageAdded { index: position, page }),
        }
    }
    final_removed.sort_unstable();
    final_removed.dedup();
    PdfPagesDiff { removed: final_removed, modified, added }
}

/// 🛡️ The index triple's own well-formedness, checked before anything is applied: a removal or a
/// modification must name a page the base has, no index may repeat, and an addition must land
/// inside the collection the diff itself produces.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_pages_diff(diff: &PdfPagesDiff, base_len: usize) -> MutationApplyResult<()> {
    let at = |index: usize| vec!["pages".to_string(), index.to_string()];
    let mut removed = HashSet::new();
    for index in &diff.removed {
        if *index >= base_len {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "indexed removal target does not exist").at(at(*index)));
        }
        if !removed.insert(*index) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "indexed removal target is repeated").at(at(*index)));
        }
    }
    let mut modified = HashSet::new();
    for item in &diff.modified {
        if item.index >= base_len {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "indexed modification target does not exist").at(at(item.index)));
        }
        if removed.contains(&item.index) {
            return Err(MutationApplyError::new("mutation.apply.conflicting-target", "indexed modification targets a removed page").at(at(item.index)));
        }
        if !modified.insert(item.index) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "indexed modification target is repeated").at(at(item.index)));
        }
    }
    let final_len = base_len - removed.len() + diff.added.len();
    let mut added = HashSet::new();
    for item in &diff.added {
        if item.index >= final_len {
            return Err(MutationApplyError::new("mutation.apply.invalid-index", "indexed addition is outside the final collection").at(at(item.index)));
        }
        if !added.insert(item.index) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "indexed addition occupies a repeated final position").at(at(item.index)));
        }
    }
    Ok(())
}
//#endregion 🔖️PagesTriple

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.pdf` (1.4). `schema` is an identity field and is never diffed.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.pdf.diff")]
pub struct PdfDiff {
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub pages: Option<PdfPagesDiff>,
}

impl MutationDiff<PdfSnapshot> for PdfDiff {
    fn apply(&self, base: &PdfSnapshot) -> MutationApplyResult<PdfSnapshot> {
        let mut next = base.clone();
        if let Some(pages) = &self.pages {
            validate_pages_diff(pages, base.pages.len())?;
            next.pages = apply_pages_diff(pages, &base.pages);
        }
        Ok(next)
    }

    /// ➕️ Structural, total, base-free, sequential-coalesce (`## Absorb` contract) — the page
    /// triple composes through [`absorb_pages_diff`]'s index-transported simulation.
    fn absorb(&mut self, other: Self) {
        self.pages = match (self.pages.take(), other.pages) {
            (None, other) => other,
            (mine, None) => mine,
            (Some(mine), Some(other)) => {
                let combined = absorb_pages_diff(&mine, &other);
                (!combined.is_empty()).then_some(combined)
            }
        };
    }
}

impl DiffAlgebra<PdfSnapshot> for PdfDiff {
    /// 🔁️ Diff-level undo, derived generically from `between` (correct by construction).
    fn inverse(&self, base: &PdfSnapshot) -> Self {
        let mid = self.apply(base).unwrap_or_else(|_| base.clone());
        Self::between(&mid, base)
    }

    fn between(base: &PdfSnapshot, other: &PdfSnapshot) -> Self {
        let pages = pages_diff_between(&base.pages, &other.pages);
        PdfDiff { pages: (!pages.is_empty()).then_some(pages) }
    }

    fn is_empty(&self) -> bool {
        self.pages.is_none()
    }
}

//#endregion 🔖️Diff

//#region 🔖️TextCodec
/// 🔤️ Hex, so a page's text can carry any byte (including the separators this grammar uses)
/// without an escape layer. Same primitive the sibling 1.7 diff codec uses.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_str(text: &str) -> String {
    text.bytes().map(|byte| format!("{byte:02x}")).collect()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_str(text: &str) -> Result<String, String> {
    if !text.len().is_multiple_of(2) {
        return Err(format!("odd hex length: {text:?}"));
    }
    let bytes: Result<Vec<u8>, String> = (0..text.len()).step_by(2).map(|index| u8::from_str_radix(&text[index..index + 2], 16).map_err(|error| error.to_string())).collect();
    String::from_utf8(bytes?).map_err(|error| error.to_string())
}

/// 🧭️ Bracket-depth-aware split: a top-level `separator` inside nested brackets is never mistaken
/// for a field separator.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn split_top_level(text: &str, separator: char) -> Vec<&str> {
    if text.is_empty() {
        return Vec::new();
    }
    let mut out = Vec::new();
    let mut depth = 0i32;
    let mut start = 0usize;
    for (index, character) in text.char_indices() {
        match character {
            '[' => depth += 1,
            ']' => depth -= 1,
            other if other == separator && depth == 0 => {
                out.push(&text[start..index]);
                start = index + other.len_utf8();
            }
            _ => {}
        }
    }
    out.push(&text[start..]);
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn strip_brackets(text: &str) -> Result<&str, String> {
    text.strip_prefix('[').and_then(|inner| inner.strip_suffix(']')).ok_or_else(|| format!("expected [...], got {text:?}"))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_usize(text: &str) -> Result<usize, String> {
    text.parse().map_err(|error: std::num::ParseIntError| error.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_f64(text: &str) -> Result<f64, String> {
    text.parse().map_err(|error: std::num::ParseFloatError| error.to_string())
}

/// 📄️ A whole `PageDoc` literal: `[width,height,hex-text]`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_page(page: &PageDoc) -> String {
    format!("[{},{},{}]", page.width, page.height, enc_str(&page.text))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_page(text: &str) -> Result<PageDoc, String> {
    let parts = split_top_level(strip_brackets(text)?, ',');
    let [width, height, body] = parts.as_slice() else { return Err(format!("page: expected 3 fields, got {}", parts.len())) };
    Ok(PageDoc { width: parse_f64(width)?, height: parse_f64(height)?, text: dec_str(body)? })
}

/// 🏷️ `PdfPageDiff`'s sparse fields as single-letter `tag:value` pairs: `W`=width, `H`=height,
/// `X`=text.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_page_diff(diff: &PdfPageDiff) -> String {
    let mut parts = Vec::new();
    if let Some(value) = diff.width {
        parts.push(format!("W:{value}"));
    }
    if let Some(value) = diff.height {
        parts.push(format!("H:{value}"));
    }
    if let Some(value) = &diff.text {
        parts.push(format!("X:{}", enc_str(value)));
    }
    format!("[{}]", parts.join(","))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_page_diff(text: &str) -> Result<PdfPageDiff, String> {
    let mut diff = PdfPageDiff::default();
    for entry in split_top_level(strip_brackets(text)?, ',') {
        if entry.is_empty() {
            continue;
        }
        let (tag, value) = entry.split_once(':').ok_or_else(|| format!("page diff: bad entry {entry:?}"))?;
        match tag {
            "W" => diff.width = Some(parse_f64(value)?),
            "H" => diff.height = Some(parse_f64(value)?),
            "X" => diff.text = Some(dec_str(value)?),
            other => return Err(format!("page diff: unknown tag {other:?}")),
        }
    }
    Ok(diff)
}

/// 📦️ The triple: `[removed];[modified];[added]`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_pages_diff(diff: &PdfPagesDiff) -> String {
    let removed = diff.removed.iter().map(|index| index.to_string()).collect::<Vec<_>>().join(",");
    let modified = diff.modified.iter().map(|item| format!("{}:{}", item.index, enc_page_diff(&item.diff))).collect::<Vec<_>>().join(",");
    let added = diff.added.iter().map(|item| format!("{}:{}", item.index, enc_page(&item.page))).collect::<Vec<_>>().join(",");
    format!("[{removed}];[{modified}];[{added}]")
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_pages_diff(body: &str) -> Result<PdfPagesDiff, String> {
    let three = split_top_level(body, ';');
    let [removed_section, modified_section, added_section] = three.as_slice() else { return Err(format!("pages diff: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_section)?, ',').into_iter().filter(|entry| !entry.is_empty()).map(parse_usize).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_section)?, ',')
        .into_iter()
        .filter(|entry| !entry.is_empty())
        .map(|entry| {
            let (index, rest) = entry.split_once(':').ok_or_else(|| format!("pages modified: bad entry {entry:?}"))?;
            Ok(PdfPageModified { index: parse_usize(index)?, diff: dec_page_diff(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_section)?, ',')
        .into_iter()
        .filter(|entry| !entry.is_empty())
        .map(|entry| {
            let (index, rest) = entry.split_once(':').ok_or_else(|| format!("pages added: bad entry {entry:?}"))?;
            Ok(PdfPageAdded { index: parse_usize(index)?, page: dec_page(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(PdfPagesDiff { removed, modified, added })
}
//#endregion 🔖️TextCodec

//#region 🔖️BinaryCodec
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_str_lp(out: &mut Vec<u8>, text: &str) {
    store::pack_rt::write_varint_u64(out, text.len() as u64);
    out.extend_from_slice(text.as_bytes());
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_str_lp(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    let length = reader.read_varint_u64().map_err(|error| error.to_string())? as usize;
    String::from_utf8(reader.read_bytes(length).map_err(|error| error.to_string())?.to_vec()).map_err(|error| error.to_string())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_page_bin(page: &PageDoc, out: &mut Vec<u8>) {
    out.extend_from_slice(&page.width.to_le_bytes());
    out.extend_from_slice(&page.height.to_le_bytes());
    write_str_lp(out, &page.text);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_page_bin(reader: &mut store::ByteReader<'_>) -> Result<PageDoc, String> {
    let width = reader.read_f64_le().map_err(|error| error.to_string())?;
    let height = reader.read_f64_le().map_err(|error| error.to_string())?;
    Ok(PageDoc { width, height, text: read_str_lp(reader)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_page_diff_bin(diff: &PdfPageDiff, out: &mut Vec<u8>) {
    out.push(u8::from(diff.width.is_some()));
    if let Some(value) = diff.width {
        out.extend_from_slice(&value.to_le_bytes());
    }
    out.push(u8::from(diff.height.is_some()));
    if let Some(value) = diff.height {
        out.extend_from_slice(&value.to_le_bytes());
    }
    out.push(u8::from(diff.text.is_some()));
    if let Some(value) = &diff.text {
        write_str_lp(out, value);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_page_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<PdfPageDiff, String> {
    let mut diff = PdfPageDiff::default();
    if reader.read_u8().map_err(|error| error.to_string())? != 0 {
        diff.width = Some(reader.read_f64_le().map_err(|error| error.to_string())?);
    }
    if reader.read_u8().map_err(|error| error.to_string())? != 0 {
        diff.height = Some(reader.read_f64_le().map_err(|error| error.to_string())?);
    }
    if reader.read_u8().map_err(|error| error.to_string())? != 0 {
        diff.text = Some(read_str_lp(reader)?);
    }
    Ok(diff)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_pages_diff_bin(diff: &PdfPagesDiff, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, diff.removed.len() as u64);
    for index in &diff.removed {
        store::pack_rt::write_varint_u64(out, *index as u64);
    }
    store::pack_rt::write_varint_u64(out, diff.modified.len() as u64);
    for item in &diff.modified {
        store::pack_rt::write_varint_u64(out, item.index as u64);
        enc_page_diff_bin(&item.diff, out);
    }
    store::pack_rt::write_varint_u64(out, diff.added.len() as u64);
    for item in &diff.added {
        store::pack_rt::write_varint_u64(out, item.index as u64);
        enc_page_bin(&item.page, out);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_pages_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<PdfPagesDiff, String> {
    let removed_count = reader.read_varint_u64().map_err(|error| error.to_string())?;
    let mut removed = Vec::with_capacity(removed_count as usize);
    for _ in 0..removed_count {
        removed.push(reader.read_varint_u64().map_err(|error| error.to_string())? as usize);
    }
    let modified_count = reader.read_varint_u64().map_err(|error| error.to_string())?;
    let mut modified = Vec::with_capacity(modified_count as usize);
    for _ in 0..modified_count {
        let index = reader.read_varint_u64().map_err(|error| error.to_string())? as usize;
        modified.push(PdfPageModified { index, diff: dec_page_diff_bin(reader)? });
    }
    let added_count = reader.read_varint_u64().map_err(|error| error.to_string())?;
    let mut added = Vec::with_capacity(added_count as usize);
    for _ in 0..added_count {
        let index = reader.read_varint_u64().map_err(|error| error.to_string())? as usize;
        added.push(PdfPageAdded { index, page: dec_page_bin(reader)? });
    }
    Ok(PdfPagesDiff { removed, modified, added })
}
//#endregion 🔖️BinaryCodec

//#region 🔖️DiffCodec
impl protocol::DiffCodec for PdfDiff {
    /// **Grammar**: `pages=<triple>` when the page lane moved, the empty string when nothing did —
    /// one space-separated `name=value` token per changed top-level field, exactly the convention
    /// the sibling 1.7 diff prints in.
    fn print_diff(&self) -> String {
        match &self.pages {
            Some(pages) => format!("pages={}", enc_pages_diff(pages)),
            None => String::new(),
        }
    }

    fn parse_diff(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let parse = |line: &str| -> Result<Self, String> {
            let mut diff = PdfDiff::default();
            if line.is_empty() {
                return Ok(diff);
            }
            for token in line.split(' ') {
                match token.strip_prefix("pages=") {
                    Some(rest) => diff.pages = Some(dec_pages_diff(rest)?),
                    None => return Err(format!("pdf 1.4 diff: unknown token {token:?}")),
                }
            }
            Ok(diff)
        };
        parse(line).map_err(|error| semio_framework_diagnostic::TextError::new(error, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }

    /// 🧪️ Real binary frame (`format u8 | flags u8 | [pages]`), matching
    /// `../💾️binary/📡️.protocol.semio`'s `header fixed 2` + `chain payload bytes` shape —
    /// varint-counted, length-prefixed, genuinely structured, never `print_diff().into_bytes()`.
    fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let flags: u8 = u8::from(self.pages.is_some());
        let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, flags];
        if let Some(pages) = &self.pages {
            enc_pages_diff_bin(pages, &mut out);
        }
        Ok(out)
    }

    fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut reader = store::ByteReader::new(bytes);
        let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
        let format = reader.read_u8().map_err(|error| malformed("diff format", 0, error.to_string()))?;
        if format != store::pack_rt::OP_BINARY_FORMAT {
            return Err(malformed("diff format", 0, format!("expected {}, got {format}", store::pack_rt::OP_BINARY_FORMAT)));
        }
        let flags = reader.read_u8().map_err(|error| malformed("diff flags", 1, error.to_string()))?;
        if flags & !0b0000_0001 != 0 {
            return Err(malformed("diff flags", 1, format!("unknown flag bits {:#010b}", flags & !0b0000_0001)));
        }
        let pages = if flags & 0b0000_0001 != 0 { Some(dec_pages_diff_bin(&mut reader).map_err(|error| malformed("diff pages", reader.position(), error))?) } else { None };
        if reader.remaining() != 0 {
            return Err(malformed("diff trailing bytes", reader.position(), format!("{} trailing bytes", reader.remaining())));
        }
        Ok(PdfDiff { pages })
    }
}
//#endregion 🔖️DiffCodec

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

```

## ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📸️snapshot/📝️text/🦀️.rs

```rust
//! 📜️ Forms artifact — textual document grammar surface + laws (constitutional: dsl). Ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM (design.md §1 CORRECTION): `store::ArtifactDsl
//! for FormsSnapshot` now lives HERE (moved from `🧬️schema/📸️snapshot`, which keeps only the struct)
//! — the native codec is one bidirectional thing and sits directly under `🚪️io/<facet>/<representation>`,
//! unsplit. This component owns the real `parse_dsl`/`print_dsl` impl plus the thin artifact-facing
//! wrappers and the canonical example fixtures and their round-trip laws.

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::FormsSnapshot;

//#region 🔖️HandcraftedArtifactDsl
/// ✉️ `ArtifactDsl` over the derived spec-driven text of `FormsSnapshot::__dsl_spec()`, the same record
/// the pack encodes; a parsed document must also pass `FormsSnapshot::validate`.
impl store::ArtifactDsl for FormsSnapshot {
    const EXTENSION: &'static str = "forms";
    fn envelope_id() -> &'static str {
        crate::FORMS_DOCUMENT_SCHEMA
    }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((envelope, rest)) => {
                if !envelope.matches_identity(Self::envelope_id(), store::semio_format::Component::Dsl, 1) {
                    return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "Forms text envelope mismatch", semio_framework_diagnostic::TextSpan::at(1, 1)));
                }
                rest
            },
            Err(_) => text,
        };
        let record = dsl::parse(body, &crate::schema::snapshot::native_pack::record_spec(), &dsl::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: dsl::SourceMode::Document })?;
        let snapshot = crate::schema::snapshot::native_pack::reconstruct_record(&record)?;
        snapshot.validate().map_err(|error| semio_framework_diagnostic::TextError::new(error, semio_framework_diagnostic::TextSpan::at(1, 1)))?;
        Ok(snapshot)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&crate::schema::snapshot::native_pack::record(self).expect("valid Forms native state"), &crate::schema::snapshot::native_pack::record_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}
//#endregion 🔖️HandcraftedArtifactDsl

/// 📄️ The building-component fixture, handcrafted in the `.forms` DSL.
pub const BUILDING_COMPONENT_EXAMPLE_TEXT: &str = include_str!("../../../🖼️assets/🎬️demo/🗣️.dsl.semio");

/// 📇️ The Contact template in the native saved-document format.
pub const DEFAULT_EXAMPLE_TEXT: &str = include_str!("../../../🖼️assets/📇️contact/🗣️.dsl.semio");

/// 🌱️ The Onboarding template in the native saved-document format.
pub const ONBOARDING_EXAMPLE_TEXT: &str = include_str!("../../../🖼️assets/🌱️onboarding/🗣️.dsl.semio");

/// 📖️ Parses `.forms` DSL text into a `FormsSnapshot` — `FormsSnapshot`'s OWN persisted wire
/// format, the derived text of its own `dsl::DslRecord` spec.
pub fn parse_dsl(text: &str) -> Result<FormsSnapshot, semio_framework_diagnostic::TextError> {
    <FormsSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `FormsSnapshot` back to `.forms` DSL text.
pub fn print_dsl(document: &FormsSnapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🔖️ExternalBridges
/// 📖️ Parses `.forms` DSL text with a plain-`String` error, reachable from OUTSIDE this crate —
/// `store` is a private `extern crate` alias (`🦀️.rs`), so `store::TextError` cannot be named
/// by the exhaustive mutation case's test adapter that has to read the committed
/// `🗣️.dsl.semio` artifact.
// 🚫️async: E1 pure codec helper (file verified I/O-free) — see R9
pub fn parse_forms_dsl(text: &str) -> Result<FormsSnapshot, String> {
    <FormsSnapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| error.to_string())
}

/// 🖨️ Prints a [`FormsSnapshot`] back to `.forms` DSL text under a name an external caller can reach, paired
/// with [`parse_forms_dsl`].
// 🚫️async: E1 pure codec helper (file verified I/O-free) — see R9
pub fn print_forms_dsl(snapshot: &FormsSnapshot) -> String {
    store::ArtifactDsl::print_dsl(snapshot)
}
//#endregion 🔖️ExternalBridges

```

## ✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📸️snapshot/📝️text/🦀️.rs

```rust
//! 📜️ Sourcing curation artifact — textual document grammar surface + laws (constitutional: dsl).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::CurationSnapshot;

/// 📄️ The `demo` example, handcrafted in the `.curation` DSL.
pub const DEMO_STOCK_TEXT: &str = crate::examples::demo::PRIMARY_TEXT;

/// 📄️ The empty curation the shell's "no example" loads — empty stock and curated table. `catalog`'s
/// handle is content-addressed from an empty stock (`catalog_child_handle(&[])`, same value
/// `CurationSnapshot::default()` mints).
pub const EMPTY_CURATION_TEXT: &str = r#"semio curation.curation.dsl v1
catalog=child_id=catalog-4f53cda18c2baa0c target=artifact-id=catalog-4f53cda18c2baa0c artifact-kind=s.stdio.semio standard=v1 subset=kit stock-extra=[ ]
curated [object-id:REF count:UINT] {
}
"#;

/// 📖️ Parses `.curation` DSL text into a `CurationSnapshot`.
pub fn parse_dsl(text: &str) -> Result<CurationSnapshot, semio_framework_diagnostic::TextError> {
    <CurationSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `CurationSnapshot` back to `.curation` DSL text.
pub fn print_dsl(document: &CurationSnapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

impl store::ArtifactDsl for CurationSnapshot {
    const EXTENSION: &'static str = "curation";
    fn envelope_id() -> &'static str { "curation.curation" }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = if text.trim_start().starts_with("semio ") {
            let (envelope, body) = store::semio_format::split_text_preamble(text).map_err(|error| semio_framework_diagnostic::TextError::new(error.to_string(), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
            if !envelope.matches_identity(Self::envelope_id(), store::semio_format::Component::Dsl, 1) { return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "Curation text envelope mismatch", semio_framework_diagnostic::TextSpan::at(1, 1))); }
            body
        } else { text };
        let record = dsl::parse(body, &Self::__dsl_spec(), &dsl::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: dsl::SourceMode::Document })?;
        let result = Self::__dsl_from_record(&record)?;
        result.validate().map_err(|message| semio_framework_diagnostic::TextError::new(message, semio_framework_diagnostic::TextSpan::at(1, 1)))?;
        Ok(result)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(Self::envelope_id(), store::semio_format::Component::Dsl, 1).expect("Curation envelope");
        store::semio_format::wrap_text(&envelope, &body)
    }
}
/// 📥 Parses the document's native text representation.
pub fn parse_curation_dsl(text: &str) -> Result<CurationSnapshot, String> { parse_dsl(text).map_err(|error| format!("{error:?}")) }
/// 📤 Emits the document's native text representation.
pub fn print_curation_dsl(snapshot: &CurationSnapshot) -> String { print_dsl(snapshot) }

```

