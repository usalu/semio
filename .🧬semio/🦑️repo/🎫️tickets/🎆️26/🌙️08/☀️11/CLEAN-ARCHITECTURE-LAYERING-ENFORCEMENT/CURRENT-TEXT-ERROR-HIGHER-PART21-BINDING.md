# Higher TextError Part21 Syntax Binding

The actual owned Part21Error enum is closed to unexpected end/character, unsupported source escape, invalid numeric literal and missing source literal. All selected calls are actual Part21 text admissions with those semantic parse cases, and explicitly choose InvalidValue while preserving author message/span. No control/IO exception is silently assigned that kind.

Complete immediate source text, inverse, authored full text, byte counts, hashes and exact per-site decisions are retained in generated/value-refusal/text-error-higher-part21-authored-1.json. Native admission is pending.

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🦀️.rs

```rust
//! 🧬️ StepSnapshot schema — persistent fields + real Part-21 codecs. `StepSnapshot` owns its own
//! typed ISO 10303-21 exchange-structure model (`StepHeader`, `StepEntity`, `StepValue`) — the
//! shared `semio_s_artifact_stdio_contract::part21` tokenizer/writer stays the reused SYNTAX layer (spec-mandated reuse,
//! same rationale as gif 87a/89a sharing one root), but the PERSISTED type is step's own, never a
//! raw `Part21Document` (that was the copy-paste-type defect flagged against ifc in
//! `w0-recon-report.md` §7 — step does not repeat it for itself).

use semio_s_artifact_stdio_contract::part21::{parse_part21, write_part21, Part21Document, Part21Header, Part21Instance, Part21Value};
use crate::STDIO_STEP_DOCUMENT_SCHEMA;
use framework_schema::ArtifactSchema;
#[path = "🪶️sqlite/🦀️.rs"]
pub mod sqlite_snapshot;

//#region 🔖️BrepModelReexport
/// 🧱 The BrepMesh analyzer types live with the derived view in `engine::brep`, not here — the
/// snapshot only stores the generic graph. Re-exported for pre-existing call sites' convenience.
pub use crate::engine::brep::{BrepFace, BrepMesh, BrepVertex};
//#endregion 🔖️BrepModelReexport

//#region 🔖️Value
/// 🔤️ One typed Part-21 argument value, step's own vocabulary (never `Part21Value` directly —
/// that stays the shared tokenizer's working representation). `Unset` = `, `Derived` = `*`,
/// `Reference` = a `#456` instance pointer, `Enum` = `.T.`/`.F.`/`.UNKNOWN.`-shaped enumeration or
/// domain-select literal, `Aggregate` = a parenthesized list, `TypedValue` = a simple/complex
/// defined-type wrapper (`IFCLENGTHMEASURE(3000.)`-shaped).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", rename_all_fields = "camelCase")]
#[derive(Default)]
pub enum StepValue {
    #[default]
    Unset,
    Derived,
    Integer(i64),
    Real(f64),
    String(String),
    Enum(String),
    Reference(u64),
    Aggregate(Vec<StepValue>),
    TypedValue {
        type_name: String,
        value: Box<StepValue>,
    },
}

//#endregion 🔖️Value

//#region 🔖️Header
/// 📜️ The one empty-but-populated `LIST[1:?] OF STRING` ISO 10303-21 §8.2 leaves a producer with
/// nothing to say: exactly one empty string. The standard's lower bound of one is a population
/// constraint, so `()` is not a legal spelling of "no description" — `('')` is, and it is what
/// every conformant writer (including the ruststep reference this repository measures against)
/// emits.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn unpopulated_string_list() -> Vec<String> {
    vec![String::new()]
}

/// 📜️ `implementation_level` for a file this codebase authored from scratch: ISO 10303-21 §8.2.2's
/// `'2;1'` — version 2 of the exchange structure, conformance option 1 (internal mapping, no
/// external references), which is what a document built from a `StepSnapshot` alone genuinely is.
pub const ISO_10303_21_IMPLEMENTATION_LEVEL: &str = "2;1";

/// 📇️ `FILE_DESCRIPTION(description, implementation_level)` — ISO 10303-21 §8.2.2. `description`
/// is `LIST[1:?] OF STRING`, hence the hand-written [`Default`]: a derived one would give the
/// empty list the standard forbids.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct StepFileDescription {
    #[value(default = "unpopulated_string_list")]
    pub description: Vec<String>,
    #[value(default)]
    pub implementation_level: String,
}

impl Default for StepFileDescription {
    fn default() -> Self {
        Self { description: unpopulated_string_list(), implementation_level: ISO_10303_21_IMPLEMENTATION_LEVEL.into() }
    }
}

/// 📇️ `FILE_NAME(name, timestamp, author, organization, preprocessor_version,
/// originating_system, authorization)` — ISO 10303-21 §8.2.3. `author` and `organization` are
/// `LIST[1:?] OF STRING`, so this carries the same hand-written [`Default`] as
/// [`StepFileDescription`] for the same reason.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct StepFileName {
    #[value(default)]
    pub name: String,
    #[value(default)]
    pub timestamp: String,
    #[value(default = "unpopulated_string_list")]
    pub author: Vec<String>,
    #[value(default = "unpopulated_string_list")]
    pub organization: Vec<String>,
    #[value(default)]
    pub preprocessor_version: String,
    #[value(default)]
    pub originating_system: String,
    #[value(default)]
    pub authorization: String,
}

impl Default for StepFileName {
    fn default() -> Self {
        Self { name: String::new(), timestamp: String::new(), author: unpopulated_string_list(), organization: unpopulated_string_list(), preprocessor_version: String::new(), originating_system: String::new(), authorization: String::new() }
    }
}

/// 📇️ `FILE_SCHEMA(schemas)` — ISO 10303-21 §8.2.4, `LIST[1:?] OF schema_name`. An unnamed schema
/// is a real (and diagnosable) state — `check_ccN_conformance` reports it as a hard `CODE_FILE_
/// SCHEMA` violation — but an empty LIST is not a state the exchange structure can even carry, so
/// the default is the one unpopulated entry rather than none.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct StepFileSchema {
    #[value(default = "unpopulated_string_list")]
    pub schemas: Vec<String>,
}

impl Default for StepFileSchema {
    fn default() -> Self {
        Self { schemas: unpopulated_string_list() }
    }
}

/// 📇️ The full typed `HEADER;` section — all three standard records. Its `Default` is ISO
/// 10303-21 §8.2's conformant minimum, so `StepSnapshot::default()` writes an exchange structure a
/// conformant reader accepts instead of one it refuses.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct StepHeader {
    #[value(default)]
    pub file_description: StepFileDescription,
    #[value(default)]
    pub file_name: StepFileName,
    #[value(default)]
    pub file_schema: StepFileSchema,
}
//#endregion 🔖️Header

//#region 🔖️Entity
/// 🧩️ An additional type record on a genuinely complex Part-21 instance
/// (`#N=(TYPE1(...)TYPE2(...))`) — spec-legal (ISO 10303-21 §4.2), rare in real AP214 exports
/// (far more common in IFC's select-type disambiguation), never silently dropped: a plain
/// single-typed instance leaves this empty; a complex one keeps every extra type here.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct StepComplexType {
    pub name: String,
    #[value(default)]
    pub args: Vec<StepValue>,
}

/// 🧩️ One `#N = TYPE(args...)` instance — id-keyed identity, positional argument list.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct StepEntity {
    pub id: u64,
    pub name: String,
    #[value(default)]
    pub args: Vec<StepValue>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub complex: Vec<StepComplexType>,
}
//#endregion 🔖️Entity

//#region 🔖️Snapshot
/// 📸️ Persisted `stdio.step` snapshot — typed HEADER triple + id-keyed entity graph. Complete per
/// FORMAT SPEC: nothing about a real AP214 exchange file is silently dropped (undecoded header
/// positions default gracefully; complex instances retain every constituent type via
/// `StepEntity::complex`). BrepMesh is a derived analyzer view
/// (`crate::engine::brep::analyze_brep_mesh`), not stored here.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.step")]
pub struct StepSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[value(default)]
    pub header: StepHeader,
    #[state(artifact)]
    #[value(default)]
    pub entities: Vec<StepEntity>,
}

impl Default for StepSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_STEP_DOCUMENT_SCHEMA.into(), header: StepHeader::default(), entities: Vec::new() }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️Part21Conversion
/// 🔁️ `Part21Value` (shared tokenizer) <-> `StepValue` (step's own). `TypedValue` always
/// round-trips through exactly ONE wrapped value on the way back out — the real,
/// EXPRESS-conformant shape for every defined-type/select wrapper this codebase's fixtures
/// exercise (`IFCLENGTHMEASURE(3000.)`, `IFCCARTESIANPOINT((1.,2.,3.))`-shaped constructs alike,
/// since the single wrapped value can itself be an `Aggregate`). A `Typed(name, items)` with
/// `items.len() != 1` is grammar-permitted but spec-illegal for an AP214 defined type; it is
/// still captured losslessly as data (via the same `Aggregate` wrapper) but re-nests as a single
/// list argument on re-emission — a documented normal form, never fabricated, matching the
/// recipe's `codec_retention_law` allowance.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn value_from_part21(v: &Part21Value) -> StepValue {
    match v {
        Part21Value::Unset => StepValue::Unset,
        Part21Value::Derived => StepValue::Derived,
        Part21Value::Int(i) => StepValue::Integer(*i),
        Part21Value::Real(r) => StepValue::Real(r.to_f64().unwrap_or_default()),
        Part21Value::Str(s) => StepValue::String(s.clone()),
        Part21Value::Enum(s) => StepValue::Enum(s.clone()),
        Part21Value::Ref(id) => StepValue::Reference(*id),
        Part21Value::List(items) => StepValue::Aggregate(items.iter().map(value_from_part21).collect()),
        Part21Value::Typed { name, items } => {
            let value = if items.len() == 1 { value_from_part21(&items[0]) } else { StepValue::Aggregate(items.iter().map(value_from_part21).collect()) };
            StepValue::TypedValue { type_name: name.clone(), value: Box::new(value) }
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn value_to_part21(v: &StepValue) -> Part21Value {
    match v {
        StepValue::Unset => Part21Value::Unset,
        StepValue::Derived => Part21Value::Derived,
        StepValue::Integer(i) => Part21Value::Int(*i),
        StepValue::Real(r) => Part21Value::Real((*r).into()),
        StepValue::String(s) => Part21Value::Str(s.clone()),
        StepValue::Enum(s) => Part21Value::Enum(s.clone()),
        StepValue::Reference(id) => Part21Value::Ref(*id),
        StepValue::Aggregate(items) => Part21Value::List(items.iter().map(value_to_part21).collect()),
        StepValue::TypedValue { type_name, value } => Part21Value::Typed { name: type_name.clone(), items: vec![value_to_part21(value)] },
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn as_string(v: Option<&Part21Value>) -> String {
    match v {
        Some(Part21Value::Str(s)) => s.clone(),
        Some(Part21Value::Enum(s)) => s.clone(),
        _ => String::new(),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn as_string_list(v: Option<&Part21Value>) -> Vec<String> {
    match v {
        Some(Part21Value::List(items)) => items
            .iter()
            .filter_map(|it| match it {
                Part21Value::Str(s) => Some(s.clone()),
                _ => None,
            })
            .collect(),
        _ => Vec::new(),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn file_description_from_part21(args: &[Part21Value]) -> StepFileDescription {
    StepFileDescription { description: as_string_list(args.first()), implementation_level: as_string(args.get(1)) }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn file_description_to_part21(d: &StepFileDescription) -> Vec<Part21Value> {
    vec![Part21Value::List(d.description.iter().cloned().map(Part21Value::Str).collect()), Part21Value::Str(d.implementation_level.clone())]
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn file_name_from_part21(args: &[Part21Value]) -> StepFileName {
    StepFileName {
        name: as_string(args.first()),
        timestamp: as_string(args.get(1)),
        author: as_string_list(args.get(2)),
        organization: as_string_list(args.get(3)),
        preprocessor_version: as_string(args.get(4)),
        originating_system: as_string(args.get(5)),
        authorization: as_string(args.get(6)),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn file_name_to_part21(f: &StepFileName) -> Vec<Part21Value> {
    vec![
        Part21Value::Str(f.name.clone()),
        Part21Value::Str(f.timestamp.clone()),
        Part21Value::List(f.author.iter().cloned().map(Part21Value::Str).collect()),
        Part21Value::List(f.organization.iter().cloned().map(Part21Value::Str).collect()),
        Part21Value::Str(f.preprocessor_version.clone()),
        Part21Value::Str(f.originating_system.clone()),
        Part21Value::Str(f.authorization.clone()),
    ]
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn file_schema_from_part21(args: &[Part21Value]) -> StepFileSchema {
    StepFileSchema { schemas: as_string_list(args.first()) }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn file_schema_to_part21(s: &StepFileSchema) -> Vec<Part21Value> {
    vec![Part21Value::List(s.schemas.iter().cloned().map(Part21Value::Str).collect())]
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn header_from_part21(h: &Part21Header) -> StepHeader {
    StepHeader { file_description: file_description_from_part21(&h.file_description), file_name: file_name_from_part21(&h.file_name), file_schema: file_schema_from_part21(&h.file_schema) }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn header_to_part21(h: &StepHeader) -> Part21Header {
    Part21Header { file_description: file_description_to_part21(&h.file_description), file_name: file_name_to_part21(&h.file_name), file_schema: file_schema_to_part21(&h.file_schema) }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn entity_from_part21(inst: &Part21Instance) -> StepEntity {
    let mut types = inst.entities.iter();
    let (name, args) = match types.next() {
        Some((n, a)) => (n.clone(), a.iter().map(value_from_part21).collect()),
        None => (String::new(), Vec::new()),
    };
    let complex = types.map(|(n, a)| StepComplexType { name: n.clone(), args: a.iter().map(value_from_part21).collect() }).collect();
    StepEntity { id: inst.id, name, args, complex }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn entity_to_part21(e: &StepEntity) -> Part21Instance {
    let mut entities = vec![(e.name.clone(), e.args.iter().map(value_to_part21).collect())];
    entities.extend(e.complex.iter().map(|c| (c.name.clone(), c.args.iter().map(value_to_part21).collect())));
    Part21Instance { id: e.id, entities }
}

/// 🔁️ `Part21Document` -> `(StepHeader, Vec<StepEntity>)` — the ONLY place the shared generic
/// graph is decoded into step's own model.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn step_snapshot_from_part21(doc: &Part21Document) -> (StepHeader, Vec<StepEntity>) {
    let header = header_from_part21(&doc.header);
    let entities = doc.instances.iter().map(entity_from_part21).collect();
    (header, entities)
}
/// 🔁️ `(StepHeader, &[StepEntity])` -> `Part21Document` — the inverse, used by the DSL/pack codecs
/// and by every real consumer (conformance-class ladder checks, the cad/process3d plugins' STEP
/// import/export) that still wants the generic view.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn step_snapshot_to_part21(header: &StepHeader, entities: &[StepEntity]) -> Part21Document {
    Part21Document { header: header_to_part21(header), instances: entities.iter().map(entity_to_part21).collect() }
}

impl StepSnapshot {
    /// 🔁️ Materializes the shared generic Part-21 graph on demand — never stored, always derived
    /// from the typed `header`/`entities` fields.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_part21_document(&self) -> Part21Document {
        step_snapshot_to_part21(&self.header, &self.entities)
    }
    /// 🔁️ Builds a `StepSnapshot` from a generic Part-21 graph (e.g. one built by
    /// `engine::brep::brep_mesh_to_part21` or hand-assembled in a test).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_part21_document(doc: &Part21Document) -> Self {
        let (header, entities) = step_snapshot_from_part21(doc);
        Self { schema: STDIO_STEP_DOCUMENT_SCHEMA.into(), header, entities }
    }
}
//#endregion 🔖️Part21Conversion

//#region 🔖️Part21Codec
impl store::ArtifactDsl for StepSnapshot {
    const EXTENSION: &'static str = "step";
    fn envelope_id() -> &'static str {
        "stdio.step"
    }

    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let document = parse_part21(body).map_err(|e| semio_framework_diagnostic::TextError::new(format!("step parse: {e}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
        Ok(Self::from_part21_document(&document))
    }
    fn print_dsl(&self) -> String {
        let body = write_part21(&self.to_part21_document());
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for StepSnapshot {
    fn sqlite_snapshot_codec() -> Option<store::ArtifactSqliteSnapshotCodec> { Some(<Self as semio_framework_os_kernel::ArtifactSqliteSnapshot>::sqlite_codec()) }
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = write_part21(&self.to_part21_document()).into_bytes();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token())));
        }
        let _ = options;
        let text = String::from_utf8(inner).map_err(|e| store::PackError::Schema(e.to_string()))?;
        let document = parse_part21(&text).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(Self::from_part21_document(&document))
    }
}
//#endregion 🔖️Part21Codec

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
#[cfg(test)]
#[path = "🧪️tests/🪶️sqlite/🦀️.rs"]
mod sqlite_snapshot_tests;
//#endregion 🧪️Tests

```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs

```rust
//! 📥️ Deserialize `stdio.step` from stdio.txt.
use crate::{StepSnapshot, STDIO_STEP_DOCUMENT_SCHEMA};
use semio_s_artifact_stdio_txt::TxtSnapshot;
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn deserialize(from: &TxtSnapshot) -> Result<StepSnapshot, semio_framework_diagnostic::TextError> {
    let _ = STDIO_STEP_DOCUMENT_SCHEMA;
    let document = semio_s_artifact_stdio_contract::part21::parse_part21(from.to_body().trim()).map_err(|e| semio_framework_diagnostic::TextError::new(format!("step parse: {e}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
    Ok(StepSnapshot::from_part21_document(&document))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn deserialize_text(text: &str) -> Result<StepSnapshot, semio_framework_diagnostic::TextError> {
    deserialize(&<TxtSnapshot as store::ArtifactDsl>::parse_dsl(text)?)
}

```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs

```rust
//! 🧬️ IfcSnapshot schema — OWN typed model of the IFC4 EXPRESS-schema data (ticket
//! 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION, W0 §7's most severe
//! finding: the prior `IfcSnapshot.document: semio_s_artifact_stdio_contract::part21::Part21Document` used STEP's
//! own persisted type verbatim as this artifact's snapshot). IFC4 rides the same ISO 10303-21
//! Part-21 EXCHANGE-STRUCTURE grammar as STEP (both real, both documented on
//! `semio_s_artifact_stdio_contract::part21`'s own module doc as a legitimate shared low-level tokenizer — same
//! spirit as OPC being shared by the OOXML trio), but the DATA MODEL is IFC4's own EXPRESS
//! schema, semantically unrelated to AP214 — so this snapshot declares its OWN
//! `IfcEntity`/`IfcValue`/`IfcHeader` types (a near-duplicate of STEP's value grammar shape,
//! which is CORRECT per the plan's specific-over-generic mandate) and converts to/from the
//! shared `Part21Document` only at the parse/write boundary, never storing it.
//! https://www.iso.org/standard/70303.html (IFC4) / https://www.iso.org/standard/63141.html (Part 21)

use crate::STDIO_IFC_DOCUMENT_SCHEMA;
#[path="🪶️sqlite/🦀️.rs"]
pub mod sqlite_snapshot;
#[cfg(test)]
#[path="🧪️tests/🪶️sqlite/🦀️.rs"]
mod sqlite_snapshot_tests;
use framework_schema::ArtifactSchema;
use semio_s_artifact_stdio_contract::part21::{parse_part21, write_part21, Part21Document, Part21Header, Part21Instance, Part21Value};

//#region 🔖️Value
/// 🔤️ One typed value in IFC4's Part-21 argument-list syntax — own enum, mirrors
/// `semio_s_artifact_stdio_contract::part21::Part21Value`'s shape but is IFC's own type (never shared cross-artifact).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", content = "value", rename_all = "camelCase")]
#[derive(Default)]
pub enum IfcValue {
    /// `$` — attribute explicitly unset.
    #[default]
    Unset,
    /// `*` — attribute derived from a supertype, not stored here.
    Derived,
    Integer(i64),
    Real(f64),
    String(String),
    /// `.EDGE.` style enumeration literal (name kept without the surrounding dots).
    Enum(String),
    /// `#N` — a reference to another entity's id.
    Reference(u64),
    /// `(a, b, c)` — a parenthesized list of values (SET/LIST/ARRAY, all indistinguishable at the
    /// Part-21 syntax level).
    Aggregate(Vec<IfcValue>),
    /// `IFCLENGTHMEASURE(3000.)` — a "defined type" wrapper: EXPRESS keyword + its own arg list.
    TypedValue {
        name: String,
        items: Vec<IfcValue>,
    },
}

impl IfcValue {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn as_reference(&self) -> Option<u64> {
        if let IfcValue::Reference(id) = self {
            Some(*id)
        } else {
            None
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn as_str(&self) -> Option<&str> {
        if let IfcValue::String(s) = self {
            Some(s.as_str())
        } else {
            None
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn as_enum(&self) -> Option<&str> {
        if let IfcValue::Enum(s) = self {
            Some(s.as_str())
        } else {
            None
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn as_real(&self) -> Option<f64> {
        match self {
            IfcValue::Real(r) => Some(*r),
            IfcValue::Integer(i) => Some(*i as f64),
            _ => None,
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn as_aggregate(&self) -> Option<&[IfcValue]> {
        if let IfcValue::Aggregate(items) = self {
            Some(items.as_slice())
        } else {
            None
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn as_typed(&self) -> Option<(&str, &[IfcValue])> {
        if let IfcValue::TypedValue { name, items } = self {
            Some((name.as_str(), items.as_slice()))
        } else {
            None
        }
    }
}
//#endregion 🔖️Value

//#region 🔖️Entity
/// 🧩️ One additional `(TYPE(args...) ...)` member of an IFC4 Part-21 COMPLEX instance — beyond
/// the primary `name`/`args` carried on [`IfcEntity`] itself. Ordinary (non-complex) instances
/// carry an empty `complex` vec; nothing about a real complex instance's extra type members is
/// ever silently dropped (typed raw-retention, per the recipe).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct IfcComplexType {
    pub name: String,
    pub args: Vec<IfcValue>,
}

/// 📦️ One `#N = TYPE(args...);` IFC4 instance — id-keyed strong entity (per the recipe: numeric
/// id key, like STEP's own `#id` and PDF's `(id,gen)`). `name` is the EXPRESS entity type keyword
/// (e.g. `"IFCWALL"`, `"IFCPROJECT"`) — the generic `{id, name, args}` shape uniformly covers every
/// IFC4 entity type, matching how the format itself is structured; a derived analyzer view can
/// filter by `name` for domain-specific queries (see `engine::spatial`) without this snapshot
/// needing a hand-modeled Rust type per IFC entity kind.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct IfcEntity {
    pub id: u64,
    pub name: String,
    pub args: Vec<IfcValue>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub complex: Vec<IfcComplexType>,
}
//#endregion 🔖️Entity

//#region 🔖️Header
/// 📇️ The three standard `HEADER;` records (`FILE_DESCRIPTION`/`FILE_NAME`/`FILE_SCHEMA`), typed
/// via IFC's own [`IfcValue`] — kept as their raw tuple-of-values shape (not schema-interpreted
/// into named sub-fields), matching the recipe's "typed HEADER section" completeness target.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct IfcHeader {
    pub file_description: Vec<IfcValue>,
    pub file_name: Vec<IfcValue>,
    pub file_schema: Vec<IfcValue>,
}

/// 🌱 ISO 10303-21 §8.2's conformant minimum HEADER, in IFC's own value type — the exact mirror of
/// [`Part21Header::default`] (`iso_10303_21_minimum`). A derived all-empty default is NOT a fixed
/// point of this artifact's own codec: `write_part21` pads every mandatory record to its full
/// attribute arity and spells an empty `LIST[1:?] OF STRING` `('')`, so an all-empty header would
/// come back from `parse_dsl`/`decode_pack` as this one anyway.
/// https://www.iso.org/standard/63141.html
impl Default for IfcHeader {
    fn default() -> Self {
        ifc_header_from_part21(&Part21Header::default())
    }
}
//#endregion 🔖️Header

//#region 🔖️Snapshot
/// 📸️ Persisted `stdio.ifc` snapshot — the full, lossless IFC4 Part-21 graph in IFC's OWN typed
/// model (never `semio_s_artifact_stdio_contract::part21::Part21Document`). Spatial structure/placement
/// matrices/property sets stay a derived analyzer view (`engine::spatial::analyze_spatial`,
/// which is handed a `Part21Document` built on demand via [`to_part21_document`]), not stored here.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.ifc")]
pub struct IfcSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[value(default)]
    pub header: IfcHeader,
    /// 🆔️ Id-keyed, order-preserving — the strong collection this artifact's diff/mutations work
    /// against (see `schema::diff::IfcEntitiesDiff`).
    #[state(artifact)]
    #[value(default)]
    pub entities: Vec<IfcEntity>,
}

impl Default for IfcSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_IFC_DOCUMENT_SCHEMA.into(), header: IfcHeader::default(), entities: Vec::new() }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️Part21Conversion
/// 🔁️ `Part21Value` -> `IfcValue`, structurally 1:1 (own enum, recursive).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn ifc_value_from_part21(v: &Part21Value) -> IfcValue {
    match v {
        Part21Value::Ref(id) => IfcValue::Reference(*id),
        Part21Value::Str(s) => IfcValue::String(s.clone()),
        Part21Value::Enum(s) => IfcValue::Enum(s.clone()),
        Part21Value::Int(i) => IfcValue::Integer(*i),
        Part21Value::Real(r) => IfcValue::Real(r.to_f64().unwrap_or_default()),
        Part21Value::List(items) => IfcValue::Aggregate(items.iter().map(ifc_value_from_part21).collect()),
        Part21Value::Typed { name, items } => IfcValue::TypedValue { name: name.clone(), items: items.iter().map(ifc_value_from_part21).collect() },
        Part21Value::Unset => IfcValue::Unset,
        Part21Value::Derived => IfcValue::Derived,
    }
}

/// 🔁️ `IfcValue` -> `Part21Value`, the exact inverse of [`ifc_value_from_part21`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn part21_value_from_ifc(v: &IfcValue) -> Part21Value {
    match v {
        IfcValue::Reference(id) => Part21Value::Ref(*id),
        IfcValue::String(s) => Part21Value::Str(s.clone()),
        IfcValue::Enum(s) => Part21Value::Enum(s.clone()),
        IfcValue::Integer(i) => Part21Value::Int(*i),
        IfcValue::Real(r) => Part21Value::Real((*r).into()),
        IfcValue::Aggregate(items) => Part21Value::List(items.iter().map(part21_value_from_ifc).collect()),
        IfcValue::TypedValue { name, items } => Part21Value::Typed { name: name.clone(), items: items.iter().map(part21_value_from_ifc).collect() },
        IfcValue::Unset => Part21Value::Unset,
        IfcValue::Derived => Part21Value::Derived,
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn ifc_header_from_part21(h: &Part21Header) -> IfcHeader {
    IfcHeader { file_description: h.file_description.iter().map(ifc_value_from_part21).collect(), file_name: h.file_name.iter().map(ifc_value_from_part21).collect(), file_schema: h.file_schema.iter().map(ifc_value_from_part21).collect() }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn part21_header_from_ifc(h: &IfcHeader) -> Part21Header {
    Part21Header { file_description: h.file_description.iter().map(part21_value_from_ifc).collect(), file_name: h.file_name.iter().map(part21_value_from_ifc).collect(), file_schema: h.file_schema.iter().map(part21_value_from_ifc).collect() }
}

/// 🔁️ One `Part21Instance` -> `IfcEntity`: the first `(name, args)` pair becomes the entity's
/// primary `name`/`args`, any further pairs (real IFC4 COMPLEX instances, e.g.
/// `IfcQuantityArea`+`IfcPhysicalSimpleQuantity`) are retained verbatim in `complex`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn ifc_entity_from_instance(inst: &Part21Instance) -> IfcEntity {
    let mut pairs = inst.entities.iter();
    let (name, args) = match pairs.next() {
        Some((name, args)) => (name.clone(), args.iter().map(ifc_value_from_part21).collect()),
        None => (String::new(), Vec::new()),
    };
    let complex = pairs.map(|(name, args)| IfcComplexType { name: name.clone(), args: args.iter().map(ifc_value_from_part21).collect() }).collect();
    IfcEntity { id: inst.id, name, args, complex }
}

/// 🔁️ Exact inverse of [`ifc_entity_from_instance`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn instance_from_ifc_entity(e: &IfcEntity) -> Part21Instance {
    let mut entities = vec![(e.name.clone(), e.args.iter().map(part21_value_from_ifc).collect())];
    for c in &e.complex {
        entities.push((c.name.clone(), c.args.iter().map(part21_value_from_ifc).collect()));
    }
    Part21Instance { id: e.id, entities }
}

/// 📤️ Builds the shared generic Part-21 graph from `snapshot` — used at the parse/write boundary
/// (codecs below) and by the derived spatial analyzer (`engine::spatial::analyze_spatial`), which
/// still walks the generic graph shape for its relationship-graph traversal.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn to_part21_document(snapshot: &IfcSnapshot) -> Part21Document {
    Part21Document { header: part21_header_from_ifc(&snapshot.header), instances: snapshot.entities.iter().map(instance_from_ifc_entity).collect() }
}

/// 📥️ Builds an `IfcSnapshot` from a parsed generic Part-21 graph.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn from_part21_document(schema: impl Into<String>, doc: &Part21Document) -> IfcSnapshot {
    IfcSnapshot { schema: schema.into(), header: ifc_header_from_part21(&doc.header), entities: doc.instances.iter().map(ifc_entity_from_instance).collect() }
}
//#endregion 🔖️Part21Conversion

//#region 🔖️Part21Codec
impl store::ArtifactDsl for IfcSnapshot {
    const EXTENSION: &'static str = "ifc";
    fn envelope_id() -> &'static str {
        "stdio.ifc"
    }

    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let document = parse_part21(body).map_err(|e| semio_framework_diagnostic::TextError::new(format!("ifc parse: {e}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
        Ok(from_part21_document(STDIO_IFC_DOCUMENT_SCHEMA, &document))
    }
    fn print_dsl(&self) -> String {
        let body = write_part21(&to_part21_document(self));
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for IfcSnapshot {
    fn sqlite_snapshot_codec()->Option<store::ArtifactSqliteSnapshotCodec>{Some(<Self as semio_framework_os_kernel::ArtifactSqliteSnapshot>::sqlite_codec())}
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = write_part21(&to_part21_document(self)).into_bytes();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token())));
        }
        let _ = options;
        let text = String::from_utf8(inner).map_err(|e| store::PackError::Schema(e.to_string()))?;
        let document = parse_part21(&text).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(from_part21_document(STDIO_IFC_DOCUMENT_SCHEMA, &document))
    }
}
//#endregion 🔖️Part21Codec

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs

```rust
//! 📥️ Deserialize `stdio.ifc` from stdio.txt.
use crate::{IfcSnapshot, STDIO_IFC_DOCUMENT_SCHEMA};
use semio_s_artifact_stdio_txt::TxtSnapshot;
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn deserialize(from: &TxtSnapshot) -> Result<IfcSnapshot, semio_framework_diagnostic::TextError> {
    let document = semio_s_artifact_stdio_contract::part21::parse_part21(from.to_body().trim()).map_err(|e| semio_framework_diagnostic::TextError::new(format!("ifc parse: {e}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
    Ok(crate::schema::snapshot::from_part21_document(STDIO_IFC_DOCUMENT_SCHEMA, &document))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn deserialize_text(text: &str) -> Result<IfcSnapshot, semio_framework_diagnostic::TextError> {
    deserialize(&<TxtSnapshot as store::ArtifactDsl>::parse_dsl(text)?)
}

```


## Actual Mount Receipt

Actual mounted set is two sites in two imported Part21 text deserializers. Two selected STEP/IFC snapshot source calls were independently repaired before mutation; their full current bytes were captured and excluded. No higher native consumer proof is claimed.
