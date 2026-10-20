//! 📦️ Forms artifact — binary document surface + laws (constitutional: pack). `pack::snapshot::ArtifactPack for
//! FormsSnapshot` encodes the derived `dsl::DslRecord` spec (composed `structure`/`results` child handles
//! included) and validates the document marker and child identities both ways. This component also owns
//! the thin `encode`/`decode` wrappers, the pack↔dsl equivalence law and the command-envelope law.

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::FormsSnapshot;
use store::PackError;

//#region 🔖️ArtifactPack
impl semio_framework_os_kernel::os_store::ArtifactPack for FormsSnapshot {
    
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, PackError> {
        self.validate().map_err(|detail| PackError::Refusal(store::PackRefusal::Malformed { kind: semio_framework_value::ValueRefusalKind::InvalidValue, what: "pack", offset: 0, detail }))?;
        let inner = store::pack_rt::encode_document(&pack::record_spec(), &pack::record(self).map_err(PackError::from)?, options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as semio_framework_os_kernel::os_store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| PackError::from(e.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| PackError::from(e.into_value_error()))?;
        if !envelope.matches_identity(<Self as semio_framework_os_kernel::os_store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as pack::snapshot::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &pack::record_spec(), options)?;
        let snapshot = pack::reconstruct_record(&record).map_err(PackError::from)?;
        snapshot.validate().map_err(|detail| PackError::Refusal(store::PackRefusal::Malformed { kind: semio_framework_value::ValueRefusalKind::InvalidValue, what: "pack", offset: 0, detail }))?;
        Ok(snapshot)
    }
    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        Some(pack::record_spec())
    }
}
impl ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot for FormsSnapshot {
fn sqlite_snapshot_codec()->Option<store::os_store::serialization::ArtifactSqliteSnapshotCodec>{Some(<Self as store::os_store::ArtifactSqliteSnapshotOperations>::sqlite_codec())}
}
//#endregion 🔖️ArtifactPack

/// 📦️ Encodes a `FormsSnapshot` to its binary pack form.
pub fn encode(document: &FormsSnapshot) -> Vec<u8> {
    semio_framework_os_kernel::os_store::ArtifactPack::encode_pack(document)
}

/// 📖️ Decodes a `FormsSnapshot` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<FormsSnapshot, PackError> {
    <FormsSnapshot as semio_framework_os_kernel::os_store::ArtifactPack>::decode_pack(bytes)
}

//#region 🔖️Store














//#endregion 🔖️Store

//#region 🧪️Tests
//#endregion 🧪️Tests




#[path="📦️pack/🦀️.rs"]
pub mod pack;

/// 🫴️ Declares this owner’s actual controlled native record factories.
impl semio_framework_os_kernel::os_store::ArtifactNativeSnapshot for FormsSnapshot {
 fn decode_native_snapshot(payload:semio_framework_os_kernel::os_store::NativeSnapshotInput<'_>,control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<Self,semio_framework_value::ValueError>{store::decode_native_snapshot_record(payload,<Self as semio_framework_os_kernel::os_store::ArtifactDsl>::envelope_id(),pack::record_spec_producer(),|record,native|{let maximum=native.maximum_bytes();pack::reconstruct_record_controlled(record,native,maximum)},control)}
 fn encode_native_snapshot(&self,encoding:semio_framework_os_kernel::os_store::NativeSnapshotEncoding,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<store::io_schema::IoPayload,semio_framework_value::ValueError>{store::encode_native_snapshot_record(encoding,<Self as semio_framework_os_kernel::os_store::ArtifactDsl>::envelope_id(),pack::record_spec_producer(),|native|pack::record_native_controlled(self,native),control)}
}

//#region 🧩️ArtifactItems
pub mod artifact_items {
use semio_framework_artifact_playbook_playbook as playbook;

//#region 🔖️Types
pub use crate::playbook::{
    PlaybookBlock as FormQuestion, PlaybookBlockOption as FormQuestionOption, PlaybookExpr as FormExpr, PlaybookStep as FormStep, PlaybookValidationError as FormValidationError, PlaybookVectorField as FormVectorField,
    PLAYBOOK_BUILTIN_KINDS as FORM_BUILTIN_KINDS,
};

pub const FORMS_DOCUMENT_SCHEMA: &str = "forms.form";
/// 🪪️ This artifact's canonical `(artifact_kind, standard, subset)` coordinate (contract §1) — lives
/// at the ARTIFACT level, not under `editor`/`viewer`, specifically so a viewer file can read it
/// without ever importing through the sibling editor module. `artifact_kind` matches
/// `#[artifact_schema(id = "s.forms.forms")]` on `FormsArtifact` (`🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs`);
/// `standard`/`subset` match this file's own `🏅️standards/🔖️1/🪆️subsets/✳️any` location.
pub const FORMS_DIALECT: semio_framework_plugin::app::Dialect = semio_framework_plugin::app::Dialect { artifact_kind: "s.forms.forms", standard: semio_framework_plugin::app::StandardId("1"), subset: semio_framework_plugin::app::SubsetId::ANY };
//#endregion 🔖️Types
}
//#endregion 🧩️ArtifactItems

//#region 🧩️ArtifactItems2
pub mod artifact_items_2 {
use crate::{FormExpr, FormQuestion, FormQuestionOption, FormStep, FormVectorField};
use semio_framework_artifact_playbook_playbook as playbook;
use semio_s_artifact_stdio_semio_table_model::schema::snapshot::SemioTableCellKind;
use semio_s_artifact_stdio_semio_table_model::schema::snapshot::SemioTableColumn;
use semio_s_artifact_stdio_semio_table_model::schema::snapshot::SemioTableRow;
use semio_s_artifact_stdio_semio_table_model::schema::snapshot::SemioTableSnapshot;
use semio_s_artifact_stdio_semio_table_model::schema::snapshot::STDIO_SEMIOTABLE_DOCUMENT_SCHEMA;
use semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValue;
use semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValueEntry;
use semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValueSnapshot;
use semio_s_artifact_stdio_semio_value_model::schema::snapshot::STDIO_SEMIOVALUE_DOCUMENT_SCHEMA;
pub use crate::schema::snapshot::FormsSnapshot;

//#region 🔖️Composition
/// 🧩️ Owns the form's complete step and question structure; results are a derived table projection.
//#region 🔖️ChildTypes
pub type FormsStructureChild = store::os_store::artifact_child::ArtifactChild<SemioValueSnapshot>;
pub type FormsResultsChild = store::os_store::artifact_child::ArtifactChild<SemioTableSnapshot>;
//#endregion 🔖️ChildTypes

//#region 🔖️Converters
/// 🌉 `semio_framework_value::DslValue` (used by `default`/`params`) <-> `SemioValue` — real, bidirectional, bytes included. `Ref` is never
/// produced by `semio_value_from_dsl` (DslValue has no graph-reference primitive), so `dsl_from_semio_value` degrades it to
/// `Null` — a documented gap only reachable if a foreign composer ever wrote a `Ref` value into this plugin's own
/// `structure` child, never by this plugin's own round trip.
pub fn semio_value_from_dsl(value: &semio_framework_value::DslValue) -> SemioValue {
    match value {
        semio_framework_value::DslValue::Null => SemioValue::Null,
        semio_framework_value::DslValue::Bool(v) => SemioValue::Bool { value: *v },
        semio_framework_value::DslValue::Number(n) => match n {
            semio_framework_value::Number::UInt(v) => SemioValue::Int { lexeme: v.to_string() },
            semio_framework_value::Number::Int(v) => SemioValue::Int { lexeme: v.to_string() },
            semio_framework_value::Number::Float(v) if v.fract() == 0.0 && v.abs() < 1e15 => SemioValue::Int { lexeme: format!("{}", *v as i64) },
            semio_framework_value::Number::Float(v) => SemioValue::Float { lexeme: format!("{v}") },
        },
        semio_framework_value::DslValue::String(s) => SemioValue::Str { value: s.clone() },
        semio_framework_value::DslValue::Bytes(bytes) => SemioValue::Bytes { value: bytes.clone() },
        semio_framework_value::DslValue::Array(items) => SemioValue::List { items: items.iter().map(semio_value_from_dsl).collect() },
        semio_framework_value::DslValue::Object(entries) => SemioValue::Map { entries: entries.iter().map(|(k, v)| SemioValueEntry { key: k.clone(), value: semio_value_from_dsl(v) }).collect() },
    }
}
pub fn dsl_from_semio_value(value: &SemioValue) -> semio_framework_value::DslValue {
    match value {
        SemioValue::Null => semio_framework_value::DslValue::Null,
        SemioValue::Bool { value } => semio_framework_value::DslValue::Bool(*value),
        SemioValue::Int { lexeme } => semio_framework_value::DslValue::int(lexeme.parse().unwrap_or(0)),
        SemioValue::Float { lexeme } => semio_framework_value::DslValue::float(lexeme.parse().unwrap_or(0.0)),
        SemioValue::Str { value } => semio_framework_value::DslValue::String(value.clone()),
        SemioValue::List { items } => semio_framework_value::DslValue::Array(items.iter().map(dsl_from_semio_value).collect()),
        SemioValue::Map { entries } => semio_framework_value::DslValue::Object(entries.iter().map(|entry| (entry.key.clone(), dsl_from_semio_value(&entry.value))).collect()),
        SemioValue::Bytes { value } => semio_framework_value::DslValue::Bytes(value.clone()),
        SemioValue::Ref { .. } => semio_framework_value::DslValue::Null,
    }
}

pub fn semio_value_map_get<'v>(value: &'v SemioValue, key: &str) -> Option<&'v SemioValue> {
    match value {
        SemioValue::Map { entries } => entries.iter().find(|entry| entry.key == key).map(|entry| &entry.value),
        _ => None,
    }
}
pub fn semio_str(value: Option<&SemioValue>) -> Option<String> {
    match value {
        Some(SemioValue::Str { value }) => Some(value.clone()),
        _ => None,
    }
}
pub fn semio_bool(value: Option<&SemioValue>) -> Option<bool> {
    match value {
        Some(SemioValue::Bool { value }) => Some(*value),
        _ => None,
    }
}
pub fn semio_f64(value: Option<&SemioValue>) -> Option<f64> {
    match value {
        Some(SemioValue::Float { lexeme }) | Some(SemioValue::Int { lexeme }) => lexeme.parse().ok(),
        _ => None,
    }
}

/// 🌉 `PlaybookExpr` (the block `condition` recursive boolean tree) <-> `SemioValue` — real,
/// bidirectional; each variant becomes a tagged `Map{kind, ...}`.
pub fn semio_value_from_expr(expr: &FormExpr) -> SemioValue {
    match expr {
        FormExpr::Const { value } => SemioValue::Map { entries: vec![SemioValueEntry { key: "kind".into(), value: SemioValue::Str { value: "const".into() } }, SemioValueEntry { key: "value".into(), value: semio_value_from_dsl(value) }] },
        FormExpr::Var { name } => SemioValue::Map { entries: vec![SemioValueEntry { key: "kind".into(), value: SemioValue::Str { value: "var".into() } }, SemioValueEntry { key: "name".into(), value: SemioValue::Str { value: name.clone() } }] },
        FormExpr::Eq { left, right } => SemioValue::Map {
            entries: vec![
                SemioValueEntry { key: "kind".into(), value: SemioValue::Str { value: "eq".into() } },
                SemioValueEntry { key: "left".into(), value: semio_value_from_expr(left) },
                SemioValueEntry { key: "right".into(), value: semio_value_from_expr(right) },
            ],
        },
        FormExpr::And { items } => SemioValue::Map {
            entries: vec![SemioValueEntry { key: "kind".into(), value: SemioValue::Str { value: "and".into() } }, SemioValueEntry { key: "items".into(), value: SemioValue::List { items: items.iter().map(semio_value_from_expr).collect() } }],
        },
        FormExpr::Or { items } => SemioValue::Map {
            entries: vec![SemioValueEntry { key: "kind".into(), value: SemioValue::Str { value: "or".into() } }, SemioValueEntry { key: "items".into(), value: SemioValue::List { items: items.iter().map(semio_value_from_expr).collect() } }],
        },
        FormExpr::Truthy { expr } => SemioValue::Map { entries: vec![SemioValueEntry { key: "kind".into(), value: SemioValue::Str { value: "truthy".into() } }, SemioValueEntry { key: "expr".into(), value: semio_value_from_expr(expr) }] },
    }
}
pub fn expr_from_semio_value(value: &SemioValue) -> Option<FormExpr> {
    let kind = semio_str(semio_value_map_get(value, "kind"))?;
    match kind.as_str() {
        "const" => Some(FormExpr::Const { value: semio_value_map_get(value, "value").map_or(semio_framework_value::DslValue::Null, dsl_from_semio_value) }),
        "var" => Some(FormExpr::Var { name: semio_str(semio_value_map_get(value, "name")).unwrap_or_default() }),
        "eq" => {
            let left = expr_from_semio_value(semio_value_map_get(value, "left")?)?;
            let right = expr_from_semio_value(semio_value_map_get(value, "right")?)?;
            Some(FormExpr::Eq { left: Box::new(left), right: Box::new(right) })
        }
        "and" => {
            let items = match semio_value_map_get(value, "items") {
                Some(SemioValue::List { items }) => items.iter().filter_map(expr_from_semio_value).collect(),
                _ => Vec::new(),
            };
            Some(FormExpr::And { items })
        }
        "or" => {
            let items = match semio_value_map_get(value, "items") {
                Some(SemioValue::List { items }) => items.iter().filter_map(expr_from_semio_value).collect(),
                _ => Vec::new(),
            };
            Some(FormExpr::Or { items })
        }
        "truthy" => Some(FormExpr::Truthy { expr: Box::new(expr_from_semio_value(semio_value_map_get(value, "expr")?)?) }),
        _ => None,
    }
}

/// 🌉 One `FormQuestion` (block) <-> a tagged `SemioValue::Map` — every field real, none stubbed.
pub fn semio_value_from_block(block: &FormQuestion) -> SemioValue {
    let mut entries = vec![
        SemioValueEntry { key: "id".into(), value: SemioValue::Str { value: block.id.clone() } },
        SemioValueEntry { key: "label".into(), value: SemioValue::Str { value: block.label.clone() } },
        SemioValueEntry { key: "kind".into(), value: SemioValue::Str { value: block.kind.clone() } },
    ];
    if let Some(v) = &block.description {
        entries.push(SemioValueEntry { key: "description".into(), value: SemioValue::Str { value: v.clone() } });
    }
    if let Some(v) = block.required {
        entries.push(SemioValueEntry { key: "required".into(), value: SemioValue::Bool { value: v } });
    }
    if let Some(v) = &block.placeholder {
        entries.push(SemioValueEntry { key: "placeholder".into(), value: SemioValue::Str { value: v.clone() } });
    }
    if let Some(v) = &block.default {
        entries.push(SemioValueEntry { key: "default".into(), value: semio_value_from_dsl(v) });
    }
    if let Some(v) = block.min {
        entries.push(SemioValueEntry { key: "min".into(), value: SemioValue::Float { lexeme: format!("{v}") } });
    }
    if let Some(v) = block.max {
        entries.push(SemioValueEntry { key: "max".into(), value: SemioValue::Float { lexeme: format!("{v}") } });
    }
    if let Some(v) = block.step {
        entries.push(SemioValueEntry { key: "step".into(), value: SemioValue::Float { lexeme: format!("{v}") } });
    }
    if let Some(v) = &block.unit {
        entries.push(SemioValueEntry { key: "unit".into(), value: SemioValue::Str { value: v.clone() } });
    }
    if let Some(v) = &block.text {
        entries.push(SemioValueEntry { key: "text".into(), value: SemioValue::Str { value: v.clone() } });
    }
    if let Some(options) = &block.options {
        entries.push(SemioValueEntry {
            key: "options".into(),
            value: SemioValue::List {
                items: options
                    .iter()
                    .map(|option| SemioValue::Map {
                        entries: vec![SemioValueEntry { key: "value".into(), value: SemioValue::Str { value: option.value.clone() } }, SemioValueEntry { key: "label".into(), value: SemioValue::Str { value: option.label.clone() } }],
                    })
                    .collect(),
            },
        });
    }
    if let Some(fields) = &block.fields {
        entries.push(SemioValueEntry {
            key: "fields".into(),
            value: SemioValue::List {
                items: fields
                    .iter()
                    .map(|field| {
                        let mut field_entries = vec![SemioValueEntry { key: "key".into(), value: SemioValue::Str { value: field.key.clone() } }];
                        if let Some(label) = &field.label {
                            field_entries.push(SemioValueEntry { key: "label".into(), value: SemioValue::Str { value: label.clone() } });
                        }
                        if let Some(value) = field.value {
                            field_entries.push(SemioValueEntry { key: "value".into(), value: SemioValue::Float { lexeme: format!("{value}") } });
                        }
                        SemioValue::Map { entries: field_entries }
                    })
                    .collect(),
            },
        });
    }
    if let Some(v) = &block.schema {
        entries.push(SemioValueEntry { key: "schema".into(), value: SemioValue::Str { value: v.clone() } });
    }
    if let Some(v) = &block.src {
        entries.push(SemioValueEntry { key: "src".into(), value: SemioValue::Str { value: v.clone() } });
    }
    if let Some(v) = &block.accept {
        entries.push(SemioValueEntry { key: "accept".into(), value: SemioValue::Str { value: v.clone() } });
    }
    if let Some(v) = &block.example_id {
        entries.push(SemioValueEntry { key: "exampleId".into(), value: SemioValue::Str { value: v.clone() } });
    }
    if let Some(v) = &block.params {
        entries.push(SemioValueEntry { key: "params".into(), value: semio_value_from_dsl(v) });
    }
    if let Some(v) = &block.condition {
        entries.push(SemioValueEntry { key: "condition".into(), value: semio_value_from_expr(v) });
    }
    SemioValue::Map { entries }
}
pub fn block_from_semio_value(value: &SemioValue) -> FormQuestion {
    FormQuestion {
        id: semio_str(semio_value_map_get(value, "id")).unwrap_or_default(),
        label: semio_str(semio_value_map_get(value, "label")).unwrap_or_default(),
        kind: semio_str(semio_value_map_get(value, "kind")).unwrap_or_default(),
        description: semio_str(semio_value_map_get(value, "description")),
        required: semio_bool(semio_value_map_get(value, "required")),
        placeholder: semio_str(semio_value_map_get(value, "placeholder")),
        default: semio_value_map_get(value, "default").map(dsl_from_semio_value),
        min: semio_f64(semio_value_map_get(value, "min")),
        max: semio_f64(semio_value_map_get(value, "max")),
        step: semio_f64(semio_value_map_get(value, "step")),
        unit: semio_str(semio_value_map_get(value, "unit")),
        text: semio_str(semio_value_map_get(value, "text")),
        options: match semio_value_map_get(value, "options") {
            Some(SemioValue::List { items }) => {
                Some(items.iter().map(|item| FormQuestionOption { value: semio_str(semio_value_map_get(item, "value")).unwrap_or_default(), label: semio_str(semio_value_map_get(item, "label")).unwrap_or_default() }).collect())
            }
            _ => None,
        },
        fields: match semio_value_map_get(value, "fields") {
            Some(SemioValue::List { items }) => Some(
                items.iter().map(|item| FormVectorField { key: semio_str(semio_value_map_get(item, "key")).unwrap_or_default(), label: semio_str(semio_value_map_get(item, "label")), value: semio_f64(semio_value_map_get(item, "value")) }).collect(),
            ),
            _ => None,
        },
        schema: semio_str(semio_value_map_get(value, "schema")),
        src: semio_str(semio_value_map_get(value, "src")),
        accept: semio_str(semio_value_map_get(value, "accept")),
        example_id: semio_str(semio_value_map_get(value, "exampleId")),
        params: semio_value_map_get(value, "params").map(dsl_from_semio_value),
        condition: semio_value_map_get(value, "condition").and_then(expr_from_semio_value),
    }
}

/// 🌉 One `FormStep` <-> a tagged `SemioValue::Map` (id/title/description/blocks).
pub fn semio_value_from_step(step: &FormStep) -> SemioValue {
    let mut entries = vec![SemioValueEntry { key: "id".into(), value: SemioValue::Str { value: step.id.clone() } }, SemioValueEntry { key: "title".into(), value: SemioValue::Str { value: step.title.clone() } }];
    if let Some(v) = &step.description {
        entries.push(SemioValueEntry { key: "description".into(), value: SemioValue::Str { value: v.clone() } });
    }
    entries.push(SemioValueEntry { key: "blocks".into(), value: SemioValue::List { items: step.blocks.iter().map(semio_value_from_block).collect() } });
    SemioValue::Map { entries }
}
pub fn step_from_semio_value(value: &SemioValue) -> FormStep {
    FormStep {
        id: semio_str(semio_value_map_get(value, "id")).unwrap_or_default(),
        title: semio_str(semio_value_map_get(value, "title")).unwrap_or_default(),
        description: semio_str(semio_value_map_get(value, "description")),
        blocks: match semio_value_map_get(value, "blocks") {
            Some(SemioValue::List { items }) => items.iter().map(block_from_semio_value).collect(),
            _ => Vec::new(),
        },
    }
}

/// 🌉 Projects the authoritative definition into its structured value child.
pub fn forms_structure_from_steps(steps: &[FormStep]) -> SemioValueSnapshot {
    SemioValueSnapshot {
        schema: STDIO_SEMIOVALUE_DOCUMENT_SCHEMA.into(),
        root: SemioValue::Map { entries: vec![SemioValueEntry { key: "steps".into(), value: SemioValue::List { items: steps.iter().map(semio_value_from_step).collect() } }] },
        nodes: Vec::new(),
    }
}
pub fn forms_steps_from_structure(structure: &SemioValueSnapshot) -> Vec<FormStep> {
    match semio_value_map_get(&structure.root, "steps") {
        Some(SemioValue::List { items }) => items.iter().map(step_from_semio_value).collect(),
        _ => Vec::new(),
    }
}

/// 📊️ A normalized response table preserves each answer's original field label and typed JSON value.
pub fn forms_results_from_responses(responses: &[crate::schema::response::FormsResponse]) -> SemioTableSnapshot {
    let rows = responses.iter().flat_map(|response| response.answers.iter().map(move |answer| SemioTableRow {
        cells: crate::standards::v1::subsets::any::io::text::snapshot::response::export::response_row(response, answer).into_iter().map(|value| SemioValue::Str { value }).collect(),
    })).collect();
    SemioTableSnapshot { schema: STDIO_SEMIOTABLE_DOCUMENT_SCHEMA.into(), columns: crate::standards::v1::subsets::any::io::text::snapshot::response::export::RESPONSE_COLUMNS.into_iter().map(|name| SemioTableColumn { name: name.into(), kind: SemioTableCellKind::Str }).collect(), rows }
}

/// 🪆️ Response content has its own projection identity independent of definition edits.
pub fn forms_results_child(responses: &[crate::schema::response::FormsResponse]) -> FormsResultsChild {
    use std::hash::{Hash, Hasher};
    let mut digest = std::collections::hash_map::DefaultHasher::new();
    semio_framework_pack_json::to_json_string(&responses.to_vec()).hash(&mut digest);
    let id = format!("forms-table-{:016x}", digest.finish());
    store::os_store::artifact_child::ArtifactChild::new(id.clone(), semio_framework_artifact_reference::ArtifactRef { artifact_id: id, dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "table".into() } })
}
//#endregion 🔖️Converters

//#region 🔖️DurableDefinition
pub fn forms_scene_id(steps: &[FormStep]) -> String {
    let content_json = semio_framework_pack_json::to_json_string(&steps.to_vec());
    store::content_id("forms-scene", content_json.as_bytes())
}

/// 📝️ Replaces the durable definition while retaining its owned projection handles.
pub fn replace_forms_steps(snapshot: &mut FormsSnapshot, steps: Vec<FormStep>) {
    snapshot.definition.steps = steps;
}

/// 🪆️ Derives projection handles without using process-local document caches.
pub fn forms_children_from_steps(steps: &[FormStep]) -> (FormsStructureChild, FormsResultsChild) {
    let scene_id = forms_scene_id(steps);
    let target_for = |subset: &str| semio_framework_artifact_reference::ArtifactRef {
        artifact_id: format!("forms-{subset}-{}", scene_id.strip_prefix("forms-scene-").unwrap_or(&scene_id)),
        dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: subset.into() },
    };
    let structure = target_for("value");
    (store::os_store::artifact_child::ArtifactChild::new(structure.artifact_id.clone(), structure), forms_results_child(&[]))
}

/// 🔎️ Reads the authoritative durable definition.
pub fn forms_steps(snapshot: &FormsSnapshot) -> Vec<FormStep> { snapshot.definition.steps.clone() }

/// 🌉️ The snapshot as the `PlaybookSpec` the playbook kernel helpers are typed on; steps come from the durable definition.
pub fn as_playbook_spec(snapshot: &FormsSnapshot) -> playbook::PlaybookSpec {
    playbook::PlaybookSpec { schema: snapshot.schema.clone(), id: snapshot.id.clone(), version: snapshot.version.clone(), title: snapshot.title.clone(), steps: forms_steps(snapshot) }
}

/// 🏗️ Creates a form with a durable definition and no responses.
pub fn forms_snapshot_with_state(schema: String, id: String, version: String, title: Option<String>, steps: &[FormStep]) -> FormsSnapshot {
    let (structure, results) = forms_children_from_steps(steps);
    FormsSnapshot { schema, id, version, title, definition: crate::schema::definition::FormsDefinition { steps: steps.to_vec() }, responses: Vec::new(), structure, results }
}

/// 🌱️ Materializes the exact derived child requested by the framework's archive loader.
pub fn forms_genesis_child_pack(snapshot: &FormsSnapshot, slot: &str, child_id: &str) -> Option<Vec<u8>> {
    use pack::snapshot::ArtifactPack;
    if slot == "structure" && child_id == snapshot.structure.child_id { return Some(forms_structure_from_steps(&snapshot.definition.steps).encode_pack()); }
    if slot == "results" && child_id == snapshot.results.child_id { return Some(forms_results_from_responses(&snapshot.responses).encode_pack()); }
    None
}
//#endregion 🔖️DurableDefinition
//#endregion 🔖️Composition
}
//#endregion 🧩️ArtifactItems2
