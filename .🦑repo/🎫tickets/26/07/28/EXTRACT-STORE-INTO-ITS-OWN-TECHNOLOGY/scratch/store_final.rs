//! 🗃️ Local-first, non-blocking, client-side, in-memory document store — hot-swappable
//! backbones (`temp://`/`file://`/`folder://`/`remote://`) layered on `vcs`'s version-graph
//! algebra. `DocumentStore`/`Backbone`/`BlobStore`/`Studio`/the serialization seam
//! (`DocumentDsl`/`DocumentPack`/`pack_rt`/`DocumentCodec`) all live here — apps depend on
//! `store`, never on `vcs`/`pack`/`dsl_core` directly (moved from `vcs/rs/lib.rs` by ticket
//! `26/07/28/EXTRACT-STORE-INTO-ITS-OWN-TECHNOLOGY`).

// The `dsl::DslDocument`/`dsl::DslOps` derive macros emit `::store::DocumentDsl`/`::store::OpText`
// paths (see `dsl/derive/rs/lib.rs`), which only resolve for crates that depend on `store` as an
// external crate — every real consumer, INCLUDING this crate's own `.ops` header grammar
// (`OpsHeaderLine` in `🔖TextFormat` below, derived on the engine directly) as well as its in-crate
// test fixtures (a crate is never its own dependency otherwise). `extern crate self as store;` is
// the same fix `vcs`/`dsl` use for their own in-crate derive usage: it makes `::store` resolve to
// this crate even when the derive is exercised in-crate.
extern crate self as store;

use dsl::{DslOps, DslRecord};
use semio_framework_core::{
    ActorId, DocumentDiff, DocumentId, DocumentVersion, HybridLogicalTimestamp, InverseOperation, OperationEnvelope, OperationId, PayloadHash, SchemaId, SchemaVersion, UndoPolicy,
};
// 🎞️ Unconditional — `operation_envelope_from_edit` below calls `hash_bytes` on every target (not
// just native), so gating the import to `not(wasm32)` broke the wasm32 build entirely (`store` is a
// dependency of `store_sync`'s wasm actor). `semio-framework-hash` itself is an unconditional
// dependency (pure blake3, no OS dependency).
use semio_framework_hash::hash_bytes;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{Arc, Mutex};
use protocol::{Edit, Operation, OperationDiff, OperationMeta, OpText, ReconcileReport};

// 🗃️ `store`'s facade over `vcs`'s version-graph algebra — apps that depend on `store` reach
// `Author`/`Change`/`Checkpoint`/`Alternative`/`VcsError`/etc through this crate, never through
// `vcs` directly (see the crate doc comment above).
pub use vcs::{absorb_diff, apply_collection_operation, apply_operation, collection_diff_from_operation, content_addressed_checkpoint_id, create_document_vcs_id, invert_collection_operation, Alternative, Author, Change, Checkpoint, CollectionDiff, CollectionOperation, DocumentVcs, Identified, ItemPatch, Patchable, VcsError};

//#region 🔖Schemas
/// @emoji 🔗 Identifies the channel a document synchronizes through, when one is attached.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentBackboneRef {
    pub uri: String,
}

/// @emoji 🔗 Builds a backbone reference from a channel URI.
pub fn document_backbone_ref(uri: &str) -> DocumentBackboneRef {
    DocumentBackboneRef { uri: uri.to_string() }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentEnvelope<P, Operation> {
    pub schema: String,
    pub id: String,
    pub vcs: DocumentVcs<P, Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backbone: Option<DocumentBackboneRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_alternative_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum DocumentCommand<Operation> {
    Apply {
        operations: Vec<Operation>,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
    },
    Undo,
    Redo,
    UndoWithPolicy {
        policy: UndoPolicy,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        semantic_command: Option<String>,
    },
    CommitCheckpoint {
        #[serde(skip_serializing_if = "Option::is_none")]
        message: Option<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        authors: Vec<Author>,
    },
    CreateAlternative {
        name: String,
    },
    SwitchAlternative {
        alternative_id: String,
    },
    CheckoutCheckpoint {
        #[serde(rename = "checkpointId")]
        checkpoint_id: String,
    },
    AmendLast {
        operations: Vec<Operation>,
        /// @emoji 🪢 Matches the last uncommitted edit's `coalesce_key` to absorb into it instead of creating a new edit.
        coalesce_key: Option<String>,
    },
}
//#endregion 🔖Schemas

//#region 🔖Text
//#region 🔖Text
/// @emoji 📍 1-based line/column position inside DSL or op-log source text. Lives in `dsl_core`
/// (the token-native DSL engine's foundation crate, which sits below `vcs`); re-exported here so
/// every existing `store::TextSpan`/`store::TextError` import across the workspace keeps compiling.
pub use dsl_core::{TextError, TextSpan};

/// @emoji 📜 Handcrafted textual representation of a document projection, implemented once per
/// technology next to its `Projection` type. LAW: `P::parse_dsl(&projection.print_dsl())` recovers
/// an equal projection — canonical `print_dsl` output is always a `parse_dsl` fixpoint; hand-written
/// text may normalize (whitespace, ordering) before reaching that fixpoint.
pub trait DocumentDsl: Sized {
    /// @emoji 🏷️ Canonical file extension WITHOUT the leading dot, e.g. `"note"`, `"puzzle3d"`.
    const EXTENSION: &'static str;
    fn parse_dsl(text: &str) -> Result<Self, TextError>;
    fn print_dsl(&self) -> String;
}

// 🎞️ CW3 kernel cut-over: `OpText` moved (method order flipped, behavior unchanged) to
// `protocol_command`, re-exported via the `🚧TEMPORARY protocol shim` near the top of this file.

//#endregion 🔖Text

//#region 🔖Pack
//#region 🔖Pack
/// @emoji 📦 Binary counterpart of `🔖Text` above — see the wave-1 design at
/// `.repo/🎫/26/07/27/PACK-BINARY-DOCUMENT-LAYER-ACROSS-ALL-APPS/` for the full container-format
/// contract. `pack`'s own `EncodeOptions`/`DecodeOptions`/`VerificationLevel` are re-exported under
/// a `Pack`-prefixed name (not a plain re-export — `dsl_derive`'s emitted `DocumentPack` impl and
/// every downstream caller spell them `store::PackEncodeOptions`/`store::PackDecodeOptions`/
/// `store::PackVerificationLevel`, so there is exactly one spelling repo-wide).
pub use pack::{DecodeOptions as PackDecodeOptions, EncodeOptions as PackEncodeOptions, PackError, VerificationLevel as PackVerificationLevel};

/// @emoji 🧵 Thin runtime bridge to `pack::{encode_document, decode_document}`, resolved as
/// `::store::pack_rt::...` by `dsl_derive`'s generated `DocumentPack` impl (app crates depend on
/// `vcs`, never on `pack` directly — same seam `::dsl::RecordSpec`/`RecordValue` already use). Also
/// hosts the schema-less JSON bridge behind `impl DocumentPack for serde_json::Value` below.
pub mod pack_rt {
    use super::{PackDecodeOptions, PackEncodeOptions, PackError};
    use dsl::{DslValue, FieldSpec, FieldValue, RecordLayout, RecordSpec, RecordValue, Shape};
    use std::collections::HashMap;

    /// @emoji 🚪 Forwards to `pack::encode_document`.
    pub fn encode_document(spec: &RecordSpec, record: &RecordValue, options: &PackEncodeOptions) -> Result<Vec<u8>, PackError> {
        pack::encode_document(spec, record, options)
    }

    /// @emoji 🚪 Forwards to `pack::decode_document`.
    pub fn decode_document(bytes: &[u8], spec: &RecordSpec, options: &PackDecodeOptions) -> Result<(RecordValue, pack::DecodeReport), PackError> {
        pack::decode_document(bytes, spec, options)
    }

    /// @emoji 🌱 Field id the JSON bridge's synthetic single-field record wraps a whole
    /// `serde_json::Value` payload in — mirrors `dsl::DslField for serde_json::Value`'s
    /// `Shape::Value` escape hatch (`dsl/rs/lib.rs`), lifted one level from "one field" to "one
    /// whole document" so schema-less apps (puzzle plugins, compose kit) get a pack encoding too.
    const JSON_BRIDGE_FIELD_ID: u16 = 1;

    fn json_bridge_spec() -> RecordSpec {
        RecordSpec::new(None, RecordLayout::Lines, vec![FieldSpec::new(JSON_BRIDGE_FIELD_ID, "value", Shape::Value)])
    }

    /// @emoji 🌱 Encodes an arbitrary `serde_json::Value` as a complete pack file. Infallible for any
    /// well-formed JSON value — mirrors `DocumentDsl::print_dsl`'s infallible signature; a
    /// `LimitExceeded` on a pathologically huge value is the one way this can panic, same ceiling
    /// `pack_value`'s own encoder enforces.
    pub fn encode_json_value(value: &serde_json::Value) -> Vec<u8> {
        let mut fields = HashMap::new();
        fields.insert(JSON_BRIDGE_FIELD_ID, FieldValue::Value(DslValue::from(value.clone())));
        let record = RecordValue { fields };
        encode_document(&json_bridge_spec(), &record, &PackEncodeOptions::default())
            .expect("json bridge encode is infallible for a well-formed DslValue")
    }

    /// @emoji 🌱 Inverse of `encode_json_value`.
    pub fn decode_json_value(bytes: &[u8]) -> Result<serde_json::Value, PackError> {
        let (record, _report) = decode_document(bytes, &json_bridge_spec(), &PackDecodeOptions::default())?;
        match record.get(JSON_BRIDGE_FIELD_ID) {
            Some(FieldValue::Value(dsl_value)) => Ok(serde_json::Value::from(dsl_value.clone())),
            _ => Ok(serde_json::Value::Null),
        }
    }
}

/// @emoji 📦 Binary counterpart to `DocumentDsl` — same shape, opposite face. LAW: `P::decode_pack(
/// &p.encode_pack())` recovers an equal `p`, AND (structurally, not just by test) `decode_pack(
/// encode_pack(p)) == parse_dsl(print_dsl(p))` — dsl and pack are two encodings of the identical
/// `(RecordSpec, RecordValue)` pair keyed by the same stable `u16` field ids `dsl_derive` assigns,
/// never two independent sources of truth. The `_with` methods are required (the seam
/// `dsl_derive`'s generated impl calls through `::store::pack_rt`); the plain names are provided
/// defaults over `Pack{Encode,Decode}Options::default()`.
pub trait DocumentPack: Sized {
    fn encode_pack_with(&self, options: &PackEncodeOptions) -> Result<Vec<u8>, PackError>;
    fn decode_pack_with(bytes: &[u8], options: &PackDecodeOptions) -> Result<Self, PackError>;

    /// @emoji 📦 `encode_pack_with` at default options — infallible in practice (mirrors
    /// `DocumentDsl::print_dsl`'s infallible signature); panics only on a `PackLimits` overflow.
    fn encode_pack(&self) -> Vec<u8> {
        self.encode_pack_with(&PackEncodeOptions::default()).expect("default-options pack encode is infallible")
    }

    /// @emoji 📦 `decode_pack_with` at default (Standard) verification.
    fn decode_pack(bytes: &[u8]) -> Result<Self, PackError> {
        Self::decode_pack_with(bytes, &PackDecodeOptions::default())
    }
}

/// @emoji 📦 Binary counterpart to `DocumentTextFiles`: `pack` is the encoded initial projection
/// (whole `.spk` container bytes), `ops` stays the op-log TEXT — the op grammar is format-invariant,
/// only the initial-projection encoding differs between the text and pack file pairs.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DocumentPackFiles {
    pub pack: Vec<u8>,
    pub ops: String,
}

/// @emoji 🌱 Pack counterpart of the schema-less `serde_json::Value` escape hatch (puzzle-plugin/
/// compose-kit apps stay on `serde_json::Value` end to end): delegates to `pack_rt`'s JSON bridge.
impl DocumentPack for serde_json::Value {
    fn encode_pack_with(&self, _options: &PackEncodeOptions) -> Result<Vec<u8>, PackError> {
        Ok(pack_rt::encode_json_value(self))
    }
    fn decode_pack_with(bytes: &[u8], _options: &PackDecodeOptions) -> Result<Self, PackError> {
        pack_rt::decode_json_value(bytes)
    }
}

/// @emoji 🔀 The closest `PackError` variant to "a text-format failure surfaced through a pack-facing
/// API" (e.g. `dsl_derive`'s generated `decode_pack_with`, whose `__dsl_from_record` step returns
/// `TextError`). A free function, not `impl From<TextError> for PackError`: both types are
/// re-exports of foreign crates (`dsl_core`/`pack_core`) through `vcs`, so a blanket `From` impl
/// here would violate the orphan rule — neither type is actually local to this crate.
pub fn text_error_to_pack_error(error: TextError) -> PackError {
    PackError::Schema(error.to_string())
}
//#endregion 🔖Pack

//#region 🔖CodecRegistry
//#region 🔖CodecRegistry
/// @emoji 🗂️ Type-erased document codec — the bridge a schema-string-keyed caller (chiefly
/// `framework/sync`'s `FolderEndpoint`) uses to print/parse pack+ops without naming the concrete
/// `P`/`Operation` types at that layer. Built once per document kind via `DocumentCodec::of`
/// (wrapped one line per app by `register_document_codec_for_app` in `framework/plugin/rs/lib.rs`,
/// wave 2) and looked up by `schema` string through `register_document_codec`/`document_codec`.
#[derive(Clone)]
pub struct DocumentCodec {
    pub schema: String,
    pub extension: &'static str,
    /// @emoji 📤 `envelope_json -> (pack files, dsl mirror text)` — the write path: `pack` is what
    /// `FolderTextStorage::write_pack`/`FolderSqliteStorage::write_pack` persist as authoritative,
    /// the returned `String` is the always-written DSL mirror (`print_dsl` on the initial
    /// projection).
    pub print: fn(&str) -> Result<(DocumentPackFiles, String), VcsError>,
    /// @emoji 📥 `(pack bytes, ops text) -> envelope_json` — the pack-first read path.
    pub parse: fn(&[u8], &str) -> Result<String, VcsError>,
    /// @emoji 📥 `(dsl text, ops text) -> envelope_json` — the DSL-mirror fallback read path (no
    /// `.pack` file yet: hand-authored or freshly imported documents).
    pub parse_dsl: fn(&str, &str) -> Result<String, VcsError>,
}

impl DocumentCodec {
    /// @emoji 🏗️ Monomorphizes three non-capturing bridge functions for `(P, Operation)` — each a
    /// genuine zero-sized `fn` item, coercible to a bare `fn` pointer — and pairs them with `schema`/
    /// `P::EXTENSION`. One call site per document kind (`register_document_codec_for_app`).
    pub fn of<P, Operation>(schema: impl Into<String>) -> Self
    where
        P: Clone + PartialEq + Serialize + DeserializeOwned + DocumentDsl + DocumentPack + Send + 'static,
        Operation: crate::Operation<P> + PartialEq + Serialize + DeserializeOwned + OpText + Send + 'static,
    {
        fn print_impl<P, Operation>(envelope_json: &str) -> Result<(DocumentPackFiles, String), VcsError>
        where
            P: DocumentDsl + DocumentPack + Serialize + DeserializeOwned,
            Operation: OpText + Serialize + DeserializeOwned,
        {
            let envelope: DocumentEnvelope<P, Operation> =
                serde_json::from_str(envelope_json).map_err(|error| VcsError::Deserialize(error.to_string()))?;
            let pack_files = print_document_pack(&envelope)?;
            let dsl_mirror = envelope.vcs.initial_projection.print_dsl();
            Ok((pack_files, dsl_mirror))
        }

        fn parse_impl<P, Operation>(pack: &[u8], ops: &str) -> Result<String, VcsError>
        where
            P: Clone + DocumentPack + Serialize + DeserializeOwned,
            Operation: OpText + crate::Operation<P> + Serialize + DeserializeOwned,
        {
            let parsed: ParsedDocumentText<P, Operation> = parse_document_pack(pack, ops).map_err(|error| VcsError::Deserialize(error.to_string()))?;
            serde_json::to_string(&parsed.envelope).map_err(|error| VcsError::Serialize(error.to_string()))
        }

        fn parse_dsl_impl<P, Operation>(dsl: &str, ops: &str) -> Result<String, VcsError>
        where
            P: Clone + DocumentDsl + Serialize + DeserializeOwned,
            Operation: OpText + crate::Operation<P> + Serialize + DeserializeOwned,
        {
            let parsed: ParsedDocumentText<P, Operation> = parse_document_text(dsl, ops).map_err(|error| VcsError::Deserialize(error.to_string()))?;
            serde_json::to_string(&parsed.envelope).map_err(|error| VcsError::Serialize(error.to_string()))
        }

        Self {
            schema: schema.into(),
            extension: P::EXTENSION,
            print: print_impl::<P, Operation>,
            parse: parse_impl::<P, Operation>,
            parse_dsl: parse_dsl_impl::<P, Operation>,
        }
    }
}

static DOCUMENT_CODEC_REGISTRY: std::sync::OnceLock<std::sync::RwLock<HashMap<String, DocumentCodec>>> = std::sync::OnceLock::new();

fn document_codec_registry() -> &'static std::sync::RwLock<HashMap<String, DocumentCodec>> {
    DOCUMENT_CODEC_REGISTRY.get_or_init(|| std::sync::RwLock::new(HashMap::new()))
}

/// @emoji 📝 Registers (or overwrites) the codec for `codec.schema` — idempotent, safe to call
/// repeatedly (every app's registration fn calls this once per document kind at plugin-init time).
pub fn register_document_codec(codec: DocumentCodec) {
    let mut registry = document_codec_registry().write().unwrap_or_else(|poisoned| poisoned.into_inner());
    registry.insert(codec.schema.clone(), codec);
}

/// @emoji 🔎 Looks up the codec registered for `schema`, if any.
pub fn document_codec(schema: &str) -> Option<DocumentCodec> {
    let registry = document_codec_registry().read().unwrap_or_else(|poisoned| poisoned.into_inner());
    registry.get(schema).cloned()
}
//#endregion 🔖CodecRegistry

//#region 🔖MergeHelpers
/// @emoji 🌳 Walks `checkpoint_id`'s ancestor chain via `parent_id` back to the root, nearest-first
/// (`checkpoint_id` itself is the first entry). Cycle-guarded (a malformed/adversarial parent chain
/// stops instead of looping forever) — every well-formed chain built by `reconcile_alternative`/
/// `CommitCheckpoint` is already acyclic, this is defense in depth, not a documented invariant break.
fn checkpoint_ancestors<P, Operation>(envelope: &DocumentEnvelope<P, Operation>, checkpoint_id: &str) -> Vec<String> {
    let mut chain = Vec::new();
    let mut seen = HashSet::new();
    let mut current = Some(checkpoint_id.to_string());
    while let Some(id) = current {
        if !seen.insert(id.clone()) {
            break;
        }
        let parent = envelope.vcs.checkpoints.iter().find(|checkpoint| checkpoint.id == id).and_then(|checkpoint| checkpoint.parent_id.clone());
        chain.push(id);
        current = parent;
    }
    chain
}

/// @emoji 🌳 The merge-base of checkpoints `a` and `b`: the nearest checkpoint common to both
/// ancestor chains (via `parent_id`), or `None` if their histories share no common ancestor.
/// Supports branch-merge tooling that needs to know "everything since the fork point" on either
/// side. `b`'s chain is walked nearest-to-farthest so the FIRST hit in `a`'s ancestor set is the
/// nearest (not merely *a*) common ancestor.
pub fn merge_base<P, Operation>(envelope: &DocumentEnvelope<P, Operation>, a: &str, b: &str) -> Option<String> {
    let ancestors_a: HashSet<String> = checkpoint_ancestors(envelope, a).into_iter().collect();
    checkpoint_ancestors(envelope, b).into_iter().find(|id| ancestors_a.contains(id))
}

pub fn reconcile_alternative<P, Operation>(
    envelope: &mut DocumentEnvelope<P, Operation>,
    alternative_name: &str,
    checkpoint_message: Option<String>,
    authors: Vec<Author>,
) -> Result<String, VcsError>
where
    P: Clone + Serialize + DeserializeOwned,
    Operation: Clone + Serialize + DeserializeOwned,
{
    if envelope.vcs.checkpoints.is_empty() {
        return Err(VcsError::NoCheckpoint);
    }
    let checkpoint_id = envelope
        .vcs
        .checkpoints
        .last()
        .map(|checkpoint| checkpoint.id.clone())
        .ok_or(VcsError::NoCheckpoint)?;
    let alternative_id = create_document_vcs_id("alternative");
    envelope.vcs.alternatives.push(Alternative {
        id: alternative_id.clone(),
        name: alternative_name.to_string(),
        checkpoint_ids: vec![checkpoint_id],
    });
    if let Some(message) = checkpoint_message {
        let change = Change {
            id: create_document_vcs_id("change"),
            edit_ids: Vec::new(),
            description: Some(message),
            saved_at: now_iso(),
        };
        let parent = envelope.vcs.checkpoints.last();
        let parent_id = parent.map(|checkpoint| checkpoint.id.clone());
        let mut change_ids = parent.map(|checkpoint| checkpoint.change_ids.clone()).unwrap_or_default();
        change_ids.push(change.id.clone());
        envelope.vcs.changes.push(change);
        let timestamp = now_iso();
        let checkpoint_message = Some("reconciled".to_string());
        let id = content_addressed_checkpoint_id(parent_id.as_deref(), &change_ids, &envelope.vcs.changes, checkpoint_message.as_deref(), &authors, &timestamp);
        envelope.vcs.checkpoints.push(Checkpoint {
            id,
            change_ids,
            parent_id,
            authors,
            message: checkpoint_message,
            timestamp,
        });
    }
    Ok(alternative_id)
}
//#endregion 🔖MergeHelpers

//#region 🔖Materialize
//#region 🔖Materialize
pub fn create_document_envelope<P, Operation>(
    schema: &str,
    id: &str,
    initial_projection: P,
    backbone: Option<DocumentBackboneRef>,
) -> DocumentEnvelope<P, Operation>
where
    P: Clone,
{
    DocumentEnvelope {
        schema: schema.into(),
        id: id.into(),
        vcs: DocumentVcs {
            initial_projection,
            edits: Vec::new(),
            changes: Vec::new(),
            checkpoints: Vec::new(),
            alternatives: Vec::new(),
        },
        backbone,
        active_alternative_id: None,
    }
}

pub fn edit_ids_for_changes<P, Operation>(envelope: &DocumentEnvelope<P, Operation>, change_ids: &[String]) -> Vec<String>
where
    Operation: Clone,
    P: Clone,
{
    let mut edit_ids = Vec::new();
    for change_id in change_ids {
        if let Some(change) = envelope.vcs.changes.iter().find(|entry| entry.id == *change_id) {
            edit_ids.extend(change.edit_ids.iter().cloned());
        }
    }
    edit_ids
}

pub fn materialize_document_projection<P, Operation>(
    envelope: &DocumentEnvelope<P, Operation>,
    applied_edit_ids: &[String],
) -> Result<P, VcsError>
where
    P: Clone,
    Operation: crate::Operation<P>,
{
    materialize_document_projection_with_conflicts(envelope, applied_edit_ids).map(|(projection, _conflicts)| projection)
}

/// @emoji 🤝 Adapts `protocol_command::Operation::reconcile`'s new instance-based signature (`&self`,
/// was a per-TYPE associated fn taking no instance at all) to the once-per-materialization call this
/// crate's replay/store paths always performed: runs the LAST applied operation's `reconcile` hook
/// against `projection`, or passes `projection` through unchanged (matching the trait's own no-op
/// default) if no operation has ever been applied yet. Every real `Operation` impl in this crate
/// (`StudioHistoryOperation`/`DemoOperation`/`TimestampedOperation`) inherits the default no-op
/// `reconcile`, which ignores `self` entirely and only inspects `projection` — so which specific
/// operation instance triggers the call is immaterial for every one of them; a technology that
/// overrides `reconcile` to do real cross-document/graph validation (see
/// `framework/product/os/core`'s `OsOperation`) is documented as inspecting the resulting
/// `projection`, not `self`, for the same reason. Maps `protocol::ReconcileReport` to this crate's
/// own `StudioConflict` at this edge — `protocol_command` deliberately doesn't know about studio
/// types (see its `Operation::reconcile` doc comment).
fn reconcile_with_last<P, Op: Operation<P>>(last_operation: Option<&Op>, projection: P) -> (P, Vec<StudioConflict>) {
    match last_operation {
        Some(operation) => {
            let (projection, reports) = operation.reconcile(projection);
            (projection, reports.into_iter().map(StudioConflict::from).collect())
        }
        None => (projection, Vec::new()),
    }
}

/// @emoji 🤝 Same replay as {@link materialize_document_projection}, additionally surfacing whatever
/// {@link Operation::reconcile} reports for the resulting projection. Kept as a twin function (rather
/// than changing `materialize_document_projection`'s signature) so every existing caller across the
/// workspace is unaffected; call sites that care about conflicts (e.g. `DocumentStore`) opt into
/// this one instead.
pub fn materialize_document_projection_with_conflicts<P, Operation>(
    envelope: &DocumentEnvelope<P, Operation>,
    applied_edit_ids: &[String],
) -> Result<(P, Vec<StudioConflict>), VcsError>
where
    P: Clone,
    Operation: crate::Operation<P>,
{
    let mut projection = envelope.vcs.initial_projection.clone();
    let mut last_operation: Option<&Operation> = None;
    for edit_id in applied_edit_ids {
        let edit = envelope
            .vcs
            .edits
            .iter()
            .find(|entry| entry.id == *edit_id)
            .ok_or_else(|| VcsError::UnknownEdit(edit_id.clone()))?;
        for operation in &edit.forwards {
            projection = apply_operation(&projection, operation);
            last_operation = Some(operation);
        }
    }
    Ok(reconcile_with_last(last_operation, projection))
}

fn now_iso() -> String {
    format!("{}", now_ms())
}

fn now_ms() -> u64 {
    #[cfg(any(not(target_arch = "wasm32"), target_env = "p2"))]
    {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_millis() as u64)
            .unwrap_or(0)
    }
    #[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
    {
        js_sys::Date::now() as u64
    }
}

fn uncommitted_edit_ids<P, Operation>(envelope: &DocumentEnvelope<P, Operation>, applied_edit_ids: &[String]) -> Vec<String>
where
    Operation: Clone,
    P: Clone,
{
    let committed: HashSet<String> = envelope
        .vcs
        .changes
        .iter()
        .flat_map(|change| change.edit_ids.iter().cloned())
        .collect();
    applied_edit_ids
        .iter()
        .filter(|id| !committed.contains(*id))
        .cloned()
        .collect()
}

//#endregion 🔖Materialize

//#region 🔖TextFormat
//#region 🔖TextFormat
/// @emoji 📄 The two files a textual VCS document is made of: the DSL text (initial projection) and
/// the append-only op log (every edit ever created, forwards-only — see {@link parse_document_text}).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DocumentTextFiles {
    pub dsl: String,
    pub ops: String,
}

/// @emoji 🧩 The result of loading a document from text: the reconstructed envelope plus the live
/// projection folded from every edit, so a caller never has to replay again after loading.
#[derive(Clone, Debug, PartialEq)]
pub struct ParsedDocumentText<P, Operation> {
    pub envelope: DocumentEnvelope<P, Operation>,
    pub projection: P,
}

//#region 🔖OpsHeaderGrammar
/// @emoji 🖋️ One `by=[...]` list entry on a `checkpoint` header line: id then name, both positional
/// (bare-preferred, quoted only when needed — e.g. a name containing a space). `Author::avatar` is
/// never part of the textual `.ops` format (this mirrors the pre-derive printer, which never carried
/// it either — see {@link Author}).
#[derive(Clone, Debug, PartialEq, DslRecord)]
struct OpsAuthor {
    #[dsl(positional)]
    id: String,
    #[dsl(positional)]
    name: String,
}

impl From<&Author> for OpsAuthor {
    fn from(author: &Author) -> Self {
        Self { id: author.id.clone(), name: author.name.clone() }
    }
}

impl From<OpsAuthor> for Author {
    fn from(author: OpsAuthor) -> Self {
        Self { id: author.id, name: author.name, avatar: None }
    }
}

/// @emoji 🧾 One `.ops` header/structural line — `doc`/`edit`/`change`/`checkpoint`/`alternative`/
/// `active` — re-derived directly on the `dsl_schema` grammar engine (`#[derive(DslOps)]` generates
/// `OpText::parse_op`/`print_op` from this declaration; see {@link print_edit_lines}/
/// {@link print_document_text}/{@link parse_document_text}, its only callers). Sigil-free lowercase
/// keywords (bare `doc`, never `@doc` — `@` is reserved for connection points everywhere else in the
/// unified DSL syntax); `id` is always the first positional field on every line; every other field is
/// a plain `key=value` attribute that is simply OMITTED when absent (no more `-` placeholder
/// sentinel); `edits`/`changes`/`checkpoints`/`by` are real DSL lists (`by=[ u1 "Ueli Saluz" ]`), not
/// comma-joined, percent-escaped strings.
#[derive(Clone, Debug, PartialEq, DslOps)]
enum OpsHeaderLine {
    Doc {
        #[dsl(positional)]
        id: String,
        schema: String,
    },
    Edit {
        #[dsl(positional)]
        id: String,
        started: String,
        actor: Option<String>,
        finished: Option<String>,
        key: Option<String>,
        description: Option<String>,
    },
    Change {
        #[dsl(positional)]
        id: String,
        saved: String,
        edits: Vec<String>,
        description: Option<String>,
    },
    Checkpoint {
        #[dsl(positional)]
        id: String,
        at: String,
        changes: Vec<String>,
        parent: Option<String>,
        by: Vec<OpsAuthor>,
        message: Option<String>,
    },
    Alternative {
        #[dsl(positional)]
        id: String,
        name: String,
        checkpoints: Vec<String>,
    },
    Active {
        #[dsl(positional)]
        id: String,
    },
}
//#endregion 🔖OpsHeaderGrammar

/// @emoji 📤 Prints one edit as an `edit ...` header line followed by one two-space-indented
/// `print_op` line per forward operation — the hot-path append unit for the op log. Backwards
/// operations and per-operation metadata are never serialized; they are recomputed during
/// {@link parse_document_text}'s load replay.
pub fn print_edit_lines<Operation: OpText>(edit: &Edit<Operation>) -> Result<String, VcsError> {
    let header = OpsHeaderLine::Edit {
        id: edit.id.clone(),
        started: edit.started_at.clone(),
        actor: edit.actor.clone(),
        finished: edit.finished_at.clone(),
        key: edit.coalesce_key.clone(),
        description: edit.description.clone(),
    };
    let mut out = header.print_op();
    out.push('\n');
    for operation in &edit.forwards {
        let printed = operation.print_op();
        if printed.contains('\n') {
            return Err(VcsError::Serialize("op-text print_op must not contain a newline".into()));
        }
        out.push_str("  ");
        out.push_str(&printed);
        out.push('\n');
    }
    Ok(out)
}

/// @emoji 📤 Builds just the op-log half of a textual/pack document — `doc` header, every edit ever
/// created as an `edit` block, then `change`/`checkpoint`/`alternative`/`active` records. Shared by
/// `print_document_text` and `print_document_pack`: the op-log grammar never touches
/// `initial_projection`, so it is provably format-invariant and both printers thin out to this plus
/// their own initial-projection encoding.
fn print_ops_log<P, Operation>(envelope: &DocumentEnvelope<P, Operation>) -> Result<String, VcsError>
where
    Operation: OpText,
{
    let mut ops = String::new();
    ops.push_str(&OpsHeaderLine::Doc { id: envelope.id.clone(), schema: envelope.schema.clone() }.print_op());
    ops.push('\n');
    for edit in &envelope.vcs.edits {
        ops.push_str(&print_edit_lines(edit)?);
    }
    for change in &envelope.vcs.changes {
        let header = OpsHeaderLine::Change {
            id: change.id.clone(),
            saved: change.saved_at.clone(),
            edits: change.edit_ids.clone(),
            description: change.description.clone(),
        };
        ops.push_str(&header.print_op());
        ops.push('\n');
    }
    for checkpoint in &envelope.vcs.checkpoints {
        let header = OpsHeaderLine::Checkpoint {
            id: checkpoint.id.clone(),
            at: checkpoint.timestamp.clone(),
            changes: checkpoint.change_ids.clone(),
            parent: checkpoint.parent_id.clone(),
            by: checkpoint.authors.iter().map(OpsAuthor::from).collect(),
            message: checkpoint.message.clone(),
        };
        ops.push_str(&header.print_op());
        ops.push('\n');
    }
    for alternative in &envelope.vcs.alternatives {
        let header = OpsHeaderLine::Alternative {
            id: alternative.id.clone(),
            name: alternative.name.clone(),
            checkpoints: alternative.checkpoint_ids.clone(),
        };
        ops.push_str(&header.print_op());
        ops.push('\n');
    }
    if let Some(active_id) = &envelope.active_alternative_id {
        ops.push_str(&OpsHeaderLine::Active { id: active_id.clone() }.print_op());
        ops.push('\n');
    }
    Ok(ops)
}

/// @emoji 📤 Prints the full textual VCS document: the DSL text (initial projection) and the complete
/// op log (`doc` header, every edit ever created as an `edit` block, then `change`/`checkpoint`/
/// `alternative`/`active` records). Replaces the JSON envelope as the canonical persisted form.
pub fn print_document_text<P, Operation>(envelope: &DocumentEnvelope<P, Operation>) -> Result<DocumentTextFiles, VcsError>
where
    P: DocumentDsl,
    Operation: OpText,
{
    let dsl = envelope.vcs.initial_projection.print_dsl();
    let ops = print_ops_log(envelope)?;
    Ok(DocumentTextFiles { dsl, ops })
}

/// @emoji 📤 Pack counterpart of `print_document_text`: identical op-log body (`print_ops_log`), but
/// the initial projection is encoded to pack bytes (`DocumentPack::encode_pack`) instead of printed
/// to DSL text.
pub fn print_document_pack<P, Operation>(envelope: &DocumentEnvelope<P, Operation>) -> Result<DocumentPackFiles, VcsError>
where
    P: DocumentPack,
    Operation: OpText,
{
    let pack = envelope.vcs.initial_projection.encode_pack();
    let ops = print_ops_log(envelope)?;
    Ok(DocumentPackFiles { pack, ops })
}

/// @emoji 📥 Replays `ops` against an already-obtained `initial_projection` — the parse-independent
/// tail shared by `parse_document_text` (which obtains the projection via `P::parse_dsl`) and
/// `parse_document_pack` (via `P::decode_pack`). Every `edit` in the log is treated as applied, in
/// file order — mirroring the existing JSON `load_document` semantics (undo/redo position and
/// checkout-alternative state are runtime-only and are not restored across a save/load cycle either
/// way).
fn replay_ops<P, Operation>(initial_projection: P, ops: &str) -> Result<ParsedDocumentText<P, Operation>, TextError>
where
    P: Clone,
    Operation: OpText + crate::Operation<P>,
{
    let mut schema = String::new();
    let mut id = String::new();
    let mut edits: Vec<Edit<Operation>> = Vec::new();
    let mut changes: Vec<Change> = Vec::new();
    let mut checkpoints: Vec<Checkpoint> = Vec::new();
    let mut alternatives: Vec<Alternative> = Vec::new();
    let mut active_alternative_id: Option<String> = None;
    let mut projection = initial_projection.clone();

    /// @emoji 🕰️ An `edit` header line's fields, held until its trailing indented op-lines are all
    /// read (its final `Edit` can only be built once `forwards` — and therefore `backwards`/
    /// `operation_meta`, both computed by replaying against `projection` — are known).
    struct PendingEdit {
        line_no: u32,
        id: String,
        actor: Option<String>,
        started_at: String,
        finished_at: Option<String>,
        coalesce_key: Option<String>,
        description: Option<String>,
    }
    let mut pending_edit: Option<PendingEdit> = None;
    let mut pending_forwards: Vec<Operation> = Vec::new();

    let flush_pending_edit =
        |pending_edit: &mut Option<PendingEdit>, pending_forwards: &mut Vec<Operation>, edits: &mut Vec<Edit<Operation>>, projection: &mut P| -> Result<(), TextError> {
            let Some(header) = pending_edit.take() else {
                return Ok(());
            };
            let forwards = std::mem::take(pending_forwards);
            let mut backwards = Vec::with_capacity(forwards.len());
            let mut operation_meta = Vec::with_capacity(forwards.len());
            for operation in &forwards {
                operation.validate(projection).map_err(|message| TextError::new(message, TextSpan::at(header.line_no, 1)))?;
                let mut back = operation.backwards(projection);
                back.reverse();
                backwards.extend(back);
                operation_meta.push(OperationMeta {
                    operation_id: Some(operation.operation_id().unwrap_or_else(|| OperationId(create_document_vcs_id("operation")))),
                    dependencies: operation.dependencies(),
                    base_version: operation.base_version().map(|version| version.0).unwrap_or(0),
                    author_id: Some(operation.author_id().unwrap_or_else(|| ActorId("local".into()))),
                    timestamp: operation.timestamp().unwrap_or_else(|| protocol::HybridLogicalTimestamp::new(0, now_ms())),
                    undo_policy: operation.undo_policy(),
                    payload_hash: None,
                });
                *projection = apply_operation(projection, operation);
            }
            edits.push(Edit {
                id: header.id,
                actor: header.actor,
                forwards,
                backwards,
                operation_meta,
                description: header.description,
                coalesce_key: header.coalesce_key,
                sequence_number: edits.len() as i32 + 1,
                started_at: header.started_at,
                finished_at: header.finished_at,
            });
            Ok(())
        };

    for (index, raw_line) in ops.lines().enumerate() {
        let line_no = index as u32 + 1;
        let trimmed = raw_line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        if raw_line.starts_with("  ") && pending_edit.is_some() {
            let operation = Operation::parse_op(trimmed)
                .map_err(|error| TextError::new(error.message, TextSpan::at(line_no, error.span.column)))?;
            pending_forwards.push(operation);
            continue;
        }
        flush_pending_edit(&mut pending_edit, &mut pending_forwards, &mut edits, &mut projection)?;
        let line = OpsHeaderLine::parse_op(trimmed).map_err(|error| TextError::new(error.message, TextSpan::at(line_no, error.span.column)))?;
        match line {
            OpsHeaderLine::Doc { id: doc_id, schema: doc_schema } => {
                schema = doc_schema;
                id = doc_id;
            }
            OpsHeaderLine::Edit { id: edit_id, started, actor, finished, key, description } => {
                pending_edit = Some(PendingEdit { line_no, id: edit_id, actor, started_at: started, finished_at: finished, coalesce_key: key, description });
                pending_forwards = Vec::new();
            }
            OpsHeaderLine::Change { id: change_id, saved, edits: edit_ids, description } => {
                changes.push(Change { id: change_id, edit_ids, description, saved_at: saved });
            }
            OpsHeaderLine::Checkpoint { id: checkpoint_id, at, changes: change_ids, parent, by, message } => {
                checkpoints.push(Checkpoint {
                    id: checkpoint_id,
                    change_ids,
                    parent_id: parent,
                    authors: by.into_iter().map(Author::from).collect(),
                    message,
                    timestamp: at,
                });
            }
            OpsHeaderLine::Alternative { id: alternative_id, name, checkpoints: checkpoint_ids } => {
                alternatives.push(Alternative { id: alternative_id, name, checkpoint_ids });
            }
            OpsHeaderLine::Active { id: active_id } => {
                active_alternative_id = Some(active_id);
            }
        }
    }
    flush_pending_edit(&mut pending_edit, &mut pending_forwards, &mut edits, &mut projection)?;

    let envelope = DocumentEnvelope {
        schema,
        id,
        vcs: DocumentVcs {
            initial_projection,
            edits,
            changes,
            checkpoints,
            alternatives,
        },
        backbone: None,
        active_alternative_id,
    };
    let last_operation = envelope.vcs.edits.last().and_then(|edit| edit.forwards.last());
    let (projection, _conflicts) = reconcile_with_last(last_operation, projection);
    Ok(ParsedDocumentText { envelope, projection })
}

/// @emoji 📥 Parses the textual VCS document back into an envelope plus its live (fully-replayed)
/// projection — obtains the initial projection via `P::parse_dsl` then shares `replay_ops`.
pub fn parse_document_text<P, Operation>(dsl: &str, ops: &str) -> Result<ParsedDocumentText<P, Operation>, TextError>
where
    P: Clone + DocumentDsl,
    Operation: OpText + crate::Operation<P>,
{
    let initial_projection = P::parse_dsl(dsl)?;
    replay_ops(initial_projection, ops)
}

/// @emoji 📥 Pack counterpart of `parse_document_text`: obtains the initial projection via
/// `DocumentPack::decode_pack` instead of `DocumentDsl::parse_dsl`, then shares the same
/// `replay_ops` tail.
pub fn parse_document_pack<P, Operation>(pack: &[u8], ops: &str) -> Result<ParsedDocumentText<P, Operation>, TextError>
where
    P: Clone + DocumentPack,
    Operation: OpText + crate::Operation<P>,
{
    let initial_projection = P::decode_pack(pack).map_err(|error| TextError::new(error.to_string(), TextSpan::at(1, 1)))?;
    replay_ops(initial_projection, ops)
}
//#endregion 🔖TextFormat

//#region 🔖History
//#region 🔖History
/// @emoji 📜 One row of a checkpoint history/ancestor graph.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoryColumn {
    pub checkpoint_id: String,
    pub timestamp: String,
    pub labels: Vec<String>,
    pub authors: Vec<Author>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_checkpoint_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub lane: usize,
    pub alternative_ids: Vec<String>,
}

fn checkpoint_alternatives<'a, P, Operation>(
    envelope: &'a DocumentEnvelope<P, Operation>,
    checkpoint_id: &str,
) -> Vec<&'a Alternative> {
    envelope
        .vcs
        .alternatives
        .iter()
        .filter(|alternative| alternative.checkpoint_ids.iter().any(|id| id == checkpoint_id))
        .collect()
}

fn is_checkpoint_main_only<P, Operation>(envelope: &DocumentEnvelope<P, Operation>, checkpoint_id: &str) -> bool {
    checkpoint_alternatives(envelope, checkpoint_id).is_empty()
}

fn has_main_only_descendant<P, Operation>(
    envelope: &DocumentEnvelope<P, Operation>,
    children_of: &HashMap<String, Vec<String>>,
    checkpoint_id: &str,
    seen: &mut HashSet<String>,
) -> bool {
    if !seen.insert(checkpoint_id.to_string()) {
        return false;
    }
    for child_id in children_of.get(checkpoint_id).into_iter().flatten() {
        if is_checkpoint_main_only(envelope, child_id) || has_main_only_descendant(envelope, children_of, child_id, seen) {
            return true;
        }
    }
    false
}

/// @emoji 🛤️ Assigns each checkpoint a swimlane: alternatives get lanes `1..n` in array order, lane
/// `0` is the main trunk. A checkpoint sits on lane 0 if it belongs to no alternative or has any
/// main-only descendant (cycle-guarded DFS); otherwise it takes its single alternative's lane, or
/// the minimum lane among several. Mirrors premigration `assignHistoryCheckpointLanes`.
fn assign_history_checkpoint_lanes<P, Operation>(envelope: &DocumentEnvelope<P, Operation>) -> HashMap<String, usize> {
    let mut lane_by_alternative: HashMap<String, usize> = HashMap::new();
    for (index, alternative) in envelope.vcs.alternatives.iter().enumerate() {
        lane_by_alternative.insert(alternative.id.clone(), index + 1);
    }
    let mut children_of: HashMap<String, Vec<String>> = HashMap::new();
    for checkpoint in &envelope.vcs.checkpoints {
        if let Some(parent_id) = &checkpoint.parent_id {
            children_of.entry(parent_id.clone()).or_default().push(checkpoint.id.clone());
        }
    }
    let mut lane_by_checkpoint_id: HashMap<String, usize> = HashMap::new();
    for checkpoint in &envelope.vcs.checkpoints {
        if checkpoint.parent_id.is_none() {
            lane_by_checkpoint_id.insert(checkpoint.id.clone(), 0);
            continue;
        }
        let mut seen = HashSet::new();
        if is_checkpoint_main_only(envelope, &checkpoint.id)
            || has_main_only_descendant(envelope, &children_of, &checkpoint.id, &mut seen)
        {
            lane_by_checkpoint_id.insert(checkpoint.id.clone(), 0);
            continue;
        }
        let alternatives = checkpoint_alternatives(envelope, &checkpoint.id);
        let lanes: Vec<usize> = alternatives
            .iter()
            .map(|alternative| *lane_by_alternative.get(&alternative.id).unwrap_or(&0))
            .collect();
        let lane = if lanes.len() == 1 {
            lanes[0]
        } else {
            lanes.into_iter().min().unwrap_or(0)
        };
        lane_by_checkpoint_id.insert(checkpoint.id.clone(), lane);
    }
    lane_by_checkpoint_id
}

/// @emoji 📜 Builds the ancestor-graph rows for a checkpoint history view: newest checkpoint first,
/// each carrying its swimlane, labels (alternative names, `"main"` fallback on the newest unlabeled
/// row), and authors. Mirrors premigration `buildHistoryColumns`.
pub fn build_history_columns<P, Operation>(envelope: &DocumentEnvelope<P, Operation>) -> Vec<HistoryColumn> {
    let lane_by_checkpoint_id = assign_history_checkpoint_lanes(envelope);
    envelope
        .vcs
        .checkpoints
        .iter()
        .rev()
        .enumerate()
        .map(|(index, checkpoint)| {
            let alternatives = checkpoint_alternatives(envelope, &checkpoint.id);
            let alternative_ids: Vec<String> = alternatives.iter().map(|alternative| alternative.id.clone()).collect();
            let mut labels: Vec<String> = alternatives.iter().map(|alternative| alternative.name.clone()).collect();
            if labels.is_empty() && index == 0 {
                labels.push("main".into());
            }
            HistoryColumn {
                checkpoint_id: checkpoint.id.clone(),
                timestamp: checkpoint.timestamp.clone(),
                labels,
                authors: checkpoint.authors.clone(),
                parent_checkpoint_id: checkpoint.parent_id.clone(),
                description: checkpoint.message.clone(),
                lane: *lane_by_checkpoint_id.get(&checkpoint.id).unwrap_or(&0),
                alternative_ids,
            }
        })
        .collect()
}
//#endregion 🔖History

//#region 🔖DocumentStore
//#region 🔖DocumentStore
pub struct DocumentStore<P, Operation>
where
    P: Clone + Serialize + DeserializeOwned,
    Operation: Clone + Serialize + DeserializeOwned + crate::Operation<P>,
{
    envelope: DocumentEnvelope<P, Operation>,
    backbone: Option<Box<dyn Backbone>>,
    dag: semio_framework_core::OpDag,
    applied_edit_ids: Vec<String>,
    redo_edit_ids: Vec<String>,
    edit_sequence: i32,
    generation: u64,
    /// @emoji 🧭 The checkpoint new commits parent onto; advances on commit/checkout/switch. Not
    /// part of the wire envelope — callers that reconstruct the store per call (e.g. a WASM plugin)
    /// must save/restore it themselves via {@link current_checkpoint_id}/{@link set_current_checkpoint_id}.
    current_checkpoint_id: Option<String>,
    /// @emoji 🖋️ Identity of the local actor driving this store. Set from each local `Apply`/
    /// `AmendLast`'s operation author; compared against `Edit.actor` so undo never touches foreign
    /// edits. Not part of the wire envelope — callers that reconstruct the store per call must
    /// save/restore it via {@link local_actor_id}/{@link set_local_actor_id}.
    local_actor_id: Option<String>,
    /// @emoji 🤝 Conflicts reported by the last {@link Operation::reconcile} pass, refreshed after
    /// remote ingestion (see {@link ingest_envelope}). Empty for every document kind that keeps the
    /// default no-operation `reconcile`. Not part of the wire envelope — it is derived, not source of truth.
    conflicts: Vec<StudioConflict>,
    /// @emoji ⚡ The live, incrementally-maintained RAW fold of `initial_projection` over every
    /// `forwards` operation in `applied_edit_ids` order — i.e. exactly what a full
    /// {@link materialize_document_projection} replay computes BEFORE its single final
    /// {@link Operation::reconcile} call. Kept in lock-step by every mutating command below instead of
    /// replaying on every read, so `projection()`/`Apply`/`AmendLast` are O(new work) instead of
    /// O(total history). Cold-path commands (checkout/switch/set_state, which reassign
    /// `applied_edit_ids` wholesale rather than appending) fall back to a full raw-fold recompute —
    /// see `fold_current`. Differential ground truth: `test_support::assert_live_equals_replay`.
    current: P,
    /// @emoji 🪢 `(edit_id, projection right before that edit's forwards were first applied)` for
    /// whichever edit is CURRENTLY the tail of `applied_edit_ids` — refreshed by `Apply`/`AmendLast`
    /// (fresh-edit branch)/`Redo`, left untouched by further amends to the same edit (so it always
    /// points at the state before the edit as a whole, not before its latest increment). Powers an
    /// O(1) `Undo` of exactly this edit; any other undo (not the cached tail, or `None`) falls back
    /// to `fold_current` — always correct, just not always O(1).
    tail_undo_cache: Option<(String, P)>,
}

/// @emoji 🖋️ Derives an edit's authoring actor from its per-operation metadata (the author of its
/// first operation), so a local edit records who produced it for later `UndoPolicy` classification.
fn edit_actor_from_meta(operation_meta: &[OperationMeta]) -> Option<String> {
    operation_meta.first().and_then(|meta| meta.author_id.clone()).map(|actor_id| actor_id.0)
}

impl<P, Operation> DocumentStore<P, Operation>
where
    P: Clone + Serialize + DeserializeOwned,
    Operation: Clone + Serialize + DeserializeOwned + crate::Operation<P>,
{
    /// @emoji 🚫 A store is always constructed with no backbone attached — the envelope's
    /// `backbone` field is a descriptor of the last attachment, never an instruction to
    /// reconnect. Callers attach explicitly via {@link attach_backbone}/{@link attach_backbone_uri}.
    pub fn new(envelope: DocumentEnvelope<P, Operation>) -> Self {
        let current_checkpoint_id = envelope.vcs.checkpoints.last().map(|checkpoint| checkpoint.id.clone());
        let current = envelope.vcs.initial_projection.clone();
        Self {
            envelope,
            backbone: None,
            dag: semio_framework_core::OpDag::new(),
            applied_edit_ids: Vec::new(),
            redo_edit_ids: Vec::new(),
            edit_sequence: 0,
            generation: 0,
            current_checkpoint_id,
            local_actor_id: None,
            conflicts: Vec::new(),
            current,
            tail_undo_cache: None,
        }
    }

    pub fn generation(&self) -> u64 {
        self.generation
    }

    pub fn envelope(&self) -> &DocumentEnvelope<P, Operation> {
        &self.envelope
    }

    pub fn applied_edit_ids(&self) -> &[String] {
        &self.applied_edit_ids
    }

    /// @emoji ↪️ Pending redo stack (edit ids undone since the last fresh `Apply`).
    pub fn redo_edit_ids(&self) -> &[String] {
        &self.redo_edit_ids
    }

    /// @emoji 🧭 The checkpoint new commits currently parent onto (defaults to the latest checkpoint
    /// on construction/`set_state`; advances on commit/checkout/switch).
    pub fn current_checkpoint_id(&self) -> Option<&str> {
        self.current_checkpoint_id.as_deref()
    }

    /// @emoji 🧭 Restores the checkout position after reconstructing the store from a serialized
    /// envelope (`set_state` resets it to the latest checkpoint, which is wrong once a caller has
    /// checked out an older one).
    pub fn set_current_checkpoint_id(&mut self, checkpoint_id: Option<String>) {
        self.current_checkpoint_id = checkpoint_id;
    }

    /// @emoji 🖋️ The local actor id used to distinguish this store's own edits from ingested ones.
    /// Not part of the wire envelope — a caller reconstructing the store per call must save/restore
    /// it via {@link set_local_actor_id} for `UndoPolicy` to keep classifying foreign edits.
    pub fn local_actor_id(&self) -> Option<&str> {
        self.local_actor_id.as_deref()
    }

    /// @emoji 🖋️ Sets the local actor id (see {@link local_actor_id}). Called automatically from each
    /// local `Apply`/`AmendLast`; callers that reconstruct the store per dispatch restore it here.
    pub fn set_local_actor_id(&mut self, actor_id: Option<String>) {
        self.local_actor_id = actor_id;
    }

    /// @emoji 🔧 The most recently created/amended edit's `(forwards, backwards, per-operation meta)`.
    /// Used right after `dispatch(Apply{..})`/`AmendLast` to build a `KernelOperation`/`InvocationResult`
    /// with a true inverse from the just-recorded `Edit.backwards`.
    pub fn edit_operations(&self) -> Option<(&[Operation], &[Operation], &[OperationMeta])> {
        self.envelope.vcs.edits.last().map(|edit| {
            (
                edit.forwards.as_slice(),
                edit.backwards.as_slice(),
                edit.operation_meta.as_slice(),
            )
        })
    }

    /// @emoji 📜 Ancestor-graph rows for this store's checkpoint history. See {@link build_history_columns}.
    pub fn history_columns(&self) -> Vec<HistoryColumn> {
        build_history_columns(&self.envelope)
    }

    pub fn set_envelope(&mut self, envelope: DocumentEnvelope<P, Operation>, applied_edit_ids: Vec<String>) {
        self.set_state(envelope, applied_edit_ids, Vec::new());
    }

    /// @emoji 💾 Restores full store state including the redo stack, so `Redo` survives
    /// round-tripping through a serialized envelope (e.g. one `dispatch` call per request).
    pub fn set_state(
        &mut self,
        envelope: DocumentEnvelope<P, Operation>,
        applied_edit_ids: Vec<String>,
        redo_edit_ids: Vec<String>,
    ) {
        self.backbone = None;
        self.edit_sequence = envelope
            .vcs
            .edits
            .iter()
            .map(|edit| edit.sequence_number)
            .max()
            .unwrap_or(0);
        self.current_checkpoint_id = envelope.vcs.checkpoints.last().map(|checkpoint| checkpoint.id.clone());
        self.envelope = envelope;
        // 🌱 These ids are adopted directly, not through `dag.insert`, so the dag never learns they're
        // satisfied — seed it or a later remote envelope whose `deps` reference one would sit `Pending`
        // forever (see `OpDag::seed_applied`). Covers every `set_state` caller: `set_envelope`
        // (store reconstruction from a persisted/cloned document), checkpoint checkout, etc.
        self.dag.seed_applied(applied_edit_ids.iter().cloned());
        self.applied_edit_ids = applied_edit_ids;
        self.redo_edit_ids = redo_edit_ids;
        self.conflicts = Vec::new();
        self.tail_undo_cache = None;
        self.current = self.fold_current().expect("set_state: fold_current should not fail for a consistent envelope");
        self.bump();
    }

    /// @emoji 🧭 Restores applied edits + checkout position for `checkpoint_id`, clearing redo.
    /// Shared by `createAlternative`/`switchAlternative`/`checkoutCheckpoint`. Mirrors premigration
    /// `checkoutCheckpointInternal`. Cold path: reassigns `applied_edit_ids` wholesale (not a tail
    /// append), so `current` is recomputed by a full raw-fold rather than an incremental update.
    fn checkout_checkpoint_internal(&mut self, checkpoint_id: String) {
        let applied = self
            .envelope
            .vcs
            .checkpoints
            .iter()
            .find(|checkpoint| checkpoint.id == checkpoint_id)
            .map(|checkpoint| edit_ids_for_changes(&self.envelope, &checkpoint.change_ids))
            .unwrap_or_default();
        self.applied_edit_ids = applied;
        self.redo_edit_ids.clear();
        self.current_checkpoint_id = Some(checkpoint_id);
        self.tail_undo_cache = None;
        self.current = self.fold_current().expect("checkout: fold_current should not fail for a consistent envelope");
    }

    /// @emoji ⚡ The live projection: `Operation::reconcile` applied to the incrementally-maintained
    /// `current` fold. Always `Ok` in practice (kept as `Result` for API stability); O(1) instead of a
    /// full replay. See the `current` field doc for the maintenance invariant.
    pub fn projection(&self) -> Result<P, VcsError> {
        Ok(reconcile_with_last(self.last_applied_operation(), self.current.clone()).0)
    }

    /// @emoji 🤝 `current` reconciled, plus whatever conflicts {@link Operation::reconcile} reports.
    /// O(1) instead of a full replay — see {@link projection}.
    pub fn projection_with_conflicts(&self) -> Result<(P, Vec<StudioConflict>), VcsError> {
        Ok(reconcile_with_last(self.last_applied_operation(), self.current.clone()))
    }

    /// @emoji 🎞️ The last-applied edit's last forward operation — the instance `reconcile_with_last`
    /// runs `Operation::reconcile` against (see that fn's doc comment for why any single instance is
    /// equivalent to the old per-TYPE associated-fn call for every technology in this repo today).
    fn last_applied_operation(&self) -> Option<&Operation> {
        self.applied_edit_ids.last().and_then(|edit_id| self.envelope.vcs.edits.iter().find(|edit| edit.id == *edit_id)).and_then(|edit| edit.forwards.last())
    }

    /// @emoji 🔂 Full raw fold of `initial_projection` over every `forwards` op in `applied_edit_ids`
    /// order, WITHOUT the final `Operation::reconcile` pass — the from-scratch computation `current`
    /// is an incrementally-maintained cache of. Used to recompute `current` on the cold paths that
    /// reassign `applied_edit_ids` wholesale instead of appending/popping its tail.
    fn fold_current(&self) -> Result<P, VcsError> {
        let mut projection = self.envelope.vcs.initial_projection.clone();
        for edit_id in &self.applied_edit_ids {
            let edit = self
                .envelope
                .vcs
                .edits
                .iter()
                .find(|entry| entry.id == *edit_id)
                .ok_or_else(|| VcsError::UnknownEdit(edit_id.clone()))?;
            for operation in &edit.forwards {
                projection = apply_operation(&projection, operation);
            }
        }
        Ok(projection)
    }

    /// @emoji 🤝 Conflicts from the last reconciliation pass (see {@link conflicts} field doc).
    pub fn conflicts(&self) -> &[StudioConflict] {
        &self.conflicts
    }

    pub fn dispatch(&mut self, command: DocumentCommand<Operation>) -> Result<(), VcsError> {
        self.pump()?;
        let is_apply = matches!(command, DocumentCommand::Apply { .. });
        self.dispatch_inner(command)?;
        self.flush_outbound(is_apply)
    }

    fn dispatch_inner(&mut self, command: DocumentCommand<Operation>) -> Result<(), VcsError> {
        match command {
            DocumentCommand::Undo => self.dispatch(DocumentCommand::UndoWithPolicy {
                policy: UndoPolicy::ExactBaseOnly,
                semantic_command: None,
            }),
            DocumentCommand::UndoWithPolicy {
                policy,
                semantic_command,
            } => match policy {
                UndoPolicy::ExactBaseOnly => {
                    let last = self.applied_edit_ids.last().cloned().ok_or(VcsError::NothingToUndo)?;
                    if !self.edit_is_local(&last) {
                        return Err(VcsError::ForeignEdit(last));
                    }
                    self.applied_edit_ids.pop();
                    self.redo_edit_ids.push(last.clone());
                    // ⚡ O(1) fast path when undoing exactly the cached tail edit; any other shape
                    // (cache miss, or a prior mid-history undo already invalidated it) falls back to a
                    // full raw-fold recompute — always correct, see `fold_current`.
                    match self.tail_undo_cache.take() {
                        Some((cached_id, cached_pre)) if cached_id == last => {
                            self.current = cached_pre;
                        }
                        _ => {
                            self.current = self.fold_current()?;
                        }
                    }
                    self.bump();
                    Ok(())
                }
                UndoPolicy::TransformAgainstConcurrent => {
                    let position = self
                        .applied_edit_ids
                        .iter()
                        .rposition(|id| self.edit_is_local(id))
                        .ok_or(VcsError::NothingToUndo)?;
                    let removed = self.applied_edit_ids.remove(position);
                    self.redo_edit_ids.push(removed);
                    // 🔂 Removing a MID-history edit has no cheap incremental inverse; cold-path replay.
                    self.tail_undo_cache = None;
                    self.current = self.fold_current()?;
                    self.bump();
                    Ok(())
                }
                UndoPolicy::SemanticUndo | UndoPolicy::CompensatingAction => {
                    let command_json = semantic_command.ok_or_else(|| {
                        VcsError::Backbone("semantic undo requires compensating command".into())
                    })?;
                    let command: DocumentCommand<Operation> =
                        serde_json::from_str(&command_json).map_err(|e| VcsError::Deserialize(e.to_string()))?;
                    self.dispatch_inner(command)
                }
            },
            DocumentCommand::Redo => {
                let next = self.redo_edit_ids.pop().ok_or(VcsError::NothingToRedo)?;
                self.applied_edit_ids.push(next.clone());
                // ⚡ Fold the redone edit's forwards onto `current` in their own natural order — cheap
                // and correct regardless of the edit's internal op grouping (unlike undo, this never
                // needs `Edit.backwards`). Re-seeds `tail_undo_cache` so a following Undo is O(1) again.
                if let Some(edit) = self.envelope.vcs.edits.iter().find(|entry| entry.id == next) {
                    let pre = self.current.clone();
                    let mut folded = pre.clone();
                    for operation in &edit.forwards {
                        folded = apply_operation(&folded, operation);
                    }
                    self.current = folded;
                    self.tail_undo_cache = Some((next, pre));
                }
                self.bump();
                Ok(())
            }
            DocumentCommand::CommitCheckpoint { message, authors } => {
                let pending = uncommitted_edit_ids(&self.envelope, &self.applied_edit_ids);
                if pending.is_empty() {
                    return Ok(());
                }
                let change = Change {
                    id: create_document_vcs_id("change"),
                    edit_ids: pending,
                    description: message.clone(),
                    saved_at: now_iso(),
                };
                let parent = self
                    .current_checkpoint_id
                    .as_ref()
                    .and_then(|id| self.envelope.vcs.checkpoints.iter().find(|cp| cp.id == *id));
                let mut change_ids = parent.map(|cp| cp.change_ids.clone()).unwrap_or_default();
                let parent_id = parent.map(|cp| cp.id.clone());
                change_ids.push(change.id.clone());
                // 🎞️ CW3: the new change is pushed BEFORE computing the checkpoint id (was after),
                // so `content_addressed_checkpoint_id` can hash its actual content, not a placeholder.
                self.envelope.vcs.changes.push(change);
                let timestamp = now_iso();
                let id = content_addressed_checkpoint_id(parent_id.as_deref(), &change_ids, &self.envelope.vcs.changes, message.as_deref(), &authors, &timestamp);
                let checkpoint = Checkpoint {
                    id,
                    change_ids,
                    parent_id,
                    authors,
                    message,
                    timestamp,
                };
                let checkpoint_id = checkpoint.id.clone();
                self.envelope.vcs.checkpoints.push(checkpoint);
                if let Some(alternative_id) = self.envelope.active_alternative_id.clone() {
                    if let Some(alternative) = self
                        .envelope
                        .vcs
                        .alternatives
                        .iter_mut()
                        .find(|alt| alt.id == alternative_id)
                    {
                        alternative.checkpoint_ids.push(checkpoint_id.clone());
                    }
                }
                self.current_checkpoint_id = Some(checkpoint_id);
                self.bump();
                Ok(())
            }
            DocumentCommand::CreateAlternative { name } => {
                if self.envelope.vcs.checkpoints.is_empty() {
                    self.dispatch(DocumentCommand::CommitCheckpoint {
                        message: None,
                        authors: Vec::new(),
                    })?;
                }
                let checkpoint_id = self
                    .current_checkpoint_id
                    .clone()
                    .or_else(|| self.envelope.vcs.checkpoints.last().map(|cp| cp.id.clone()))
                    .ok_or(VcsError::NoCheckpoint)?;
                let alt_id = create_document_vcs_id("alternative");
                self.envelope.vcs.alternatives.push(Alternative {
                    id: alt_id.clone(),
                    name,
                    checkpoint_ids: vec![checkpoint_id.clone()],
                });
                self.envelope.active_alternative_id = Some(alt_id);
                self.checkout_checkpoint_internal(checkpoint_id);
                self.bump();
                Ok(())
            }
            DocumentCommand::SwitchAlternative { alternative_id } => {
                let alternative = self
                    .envelope
                    .vcs
                    .alternatives
                    .iter()
                    .find(|alt| alt.id == alternative_id)
                    .ok_or_else(|| VcsError::UnknownAlternative(alternative_id.clone()))?
                    .clone();
                let checkpoint_id = alternative
                    .checkpoint_ids
                    .last()
                    .ok_or(VcsError::NoCheckpoint)?
                    .clone();
                if !self.envelope.vcs.checkpoints.iter().any(|cp| cp.id == checkpoint_id) {
                    return Err(VcsError::NoCheckpoint);
                }
                self.checkout_checkpoint_internal(checkpoint_id);
                self.envelope.active_alternative_id = Some(alternative_id);
                self.bump();
                Ok(())
            }
            DocumentCommand::CheckoutCheckpoint { checkpoint_id } => {
                if !self.envelope.vcs.checkpoints.iter().any(|cp| cp.id == checkpoint_id) {
                    return Err(VcsError::UnknownChange(checkpoint_id.clone()));
                }
                self.checkout_checkpoint_internal(checkpoint_id.clone());
                self.envelope.active_alternative_id = self
                    .envelope
                    .vcs
                    .alternatives
                    .iter()
                    .find(|alt| alt.checkpoint_ids.last() == Some(&checkpoint_id))
                    .map(|alt| alt.id.clone());
                self.bump();
                Ok(())
            }
            DocumentCommand::Apply {
                operations,
                description,
            } => {
                if operations.is_empty() {
                    return Err(VcsError::EmptyApply);
                }
                let started_at = now_iso();
                // ⚡ `current` is always up to date (maintained by every mutating command below), so
                // this is an O(1) clone instead of a full replay — see the `current` field doc.
                let pre_projection = self.current.clone();
                let (forwards, backwards, operation_meta, post) =
                    Self::replay_operations(&pre_projection, operations);
                let actor = edit_actor_from_meta(&operation_meta);
                self.local_actor_id = actor.clone();
                self.edit_sequence += 1;
                let edit = Edit {
                    id: create_document_vcs_id("edit"),
                    actor,
                    forwards,
                    backwards,
                    operation_meta,
                    description,
                    coalesce_key: None,
                    sequence_number: self.edit_sequence,
                    started_at,
                    finished_at: Some(now_iso()),
                };
                self.tail_undo_cache = Some((edit.id.clone(), pre_projection));
                self.applied_edit_ids.push(edit.id.clone());
                self.envelope.vcs.edits.push(edit);
                self.current = post;
                self.redo_edit_ids.clear();
                self.bump();
                Ok(())
            }
            DocumentCommand::AmendLast {
                operations,
                coalesce_key,
            } => {
                if operations.is_empty() {
                    return Err(VcsError::EmptyApply);
                }
                let amend_target = self.applied_edit_ids.last().cloned().filter(|last_id| {
                    coalesce_key.is_some()
                        && uncommitted_edit_ids(&self.envelope, &self.applied_edit_ids).contains(last_id)
                        && self
                            .envelope
                            .vcs
                            .edits
                            .iter()
                            .find(|edit| edit.id == *last_id)
                            .map(|edit| edit.coalesce_key == coalesce_key)
                            .unwrap_or(false)
                });
                if let Some(edit_id) = amend_target {
                    // ⚡ `current` already reflects this edit's existing forwards (it was folded in
                    // when the edit was created or last amended), so it's always the correct base for
                    // the NEW operations — O(1) instead of the old cache-validity dance.
                    let pre_projection = self.current.clone();
                    let (new_forwards, new_backwards, new_operation_meta, post) =
                        Self::replay_operations(&pre_projection, operations);
                    if let Some(edit) = self.envelope.vcs.edits.iter_mut().find(|edit| edit.id == edit_id) {
                        edit.forwards.extend(new_forwards);
                        edit.backwards.extend(new_backwards);
                        edit.operation_meta.extend(new_operation_meta);
                        edit.finished_at = Some(now_iso());
                    }
                    self.current = post;
                    self.redo_edit_ids.clear();
                    self.bump();
                    Ok(())
                } else {
                    let started_at = now_iso();
                    let pre_projection = self.current.clone();
                    let (forwards, backwards, operation_meta, post) =
                        Self::replay_operations(&pre_projection, operations);
                    let actor = edit_actor_from_meta(&operation_meta);
                    self.local_actor_id = actor.clone();
                    self.edit_sequence += 1;
                    let edit_id = create_document_vcs_id("edit");
                    let edit = Edit {
                        id: edit_id.clone(),
                        actor,
                        forwards,
                        backwards,
                        operation_meta,
                        description: None,
                        coalesce_key,
                        sequence_number: self.edit_sequence,
                        started_at,
                        finished_at: Some(now_iso()),
                    };
                    self.tail_undo_cache = Some((edit_id, pre_projection));
                    self.applied_edit_ids.push(edit.id.clone());
                    self.envelope.vcs.edits.push(edit);
                    self.current = post;
                    self.redo_edit_ids.clear();
                    self.bump();
                    Ok(())
                }
            }
        }
    }

    /// @emoji 🔂 Replays `operations` over `pre_projection`, returning forwards, reversed-backwards,
    /// per-operation metadata, and the resulting projection. Shared by `Apply` and `AmendLast`.
    fn replay_operations(pre_projection: &P, operations: Vec<Operation>) -> (Vec<Operation>, Vec<Operation>, Vec<OperationMeta>, P) {
        let mut projection = pre_projection.clone();
        let mut forwards = Vec::with_capacity(operations.len());
        let mut backwards = Vec::new();
        let mut operation_meta = Vec::with_capacity(operations.len());
        for operation in operations {
            let mut back = operation.backwards(&projection);
            back.reverse();
            backwards.extend(back);
            operation_meta.push(OperationMeta {
                operation_id: Some(operation
                    .operation_id()
                    .unwrap_or_else(|| OperationId(create_document_vcs_id("operation")))),
                dependencies: operation.dependencies(),
                base_version: operation.base_version().map(|version| version.0).unwrap_or(0),
                author_id: Some(operation.author_id().unwrap_or_else(|| ActorId("local".into()))),
                timestamp: operation
                    .timestamp()
                    .unwrap_or_else(|| protocol::HybridLogicalTimestamp::new(0, now_ms())),
                undo_policy: operation.undo_policy(),
                // 🎞️ CW3: direct blake3 (same primitive `pack_core::ContentHash` uses) replaces the
                // old `semio_framework_hash::hash_bytes` String hash — `protocol_core::PayloadHash` is
                // now `[u8; 32]`, not a hex string. NOT `pack::content_hash`, which reads a pack
                // FILE's footer rather than hashing arbitrary bytes.
                payload_hash: Some(protocol::PayloadHash(*blake3::hash(&serde_json::to_vec(&operation).unwrap_or_default()).as_bytes())),
            });
            projection = apply_operation(&projection, &operation);
            forwards.push(operation);
        }
        (forwards, backwards, operation_meta, projection)
    }

    pub fn dispatch_json(&mut self, command_json: &str) -> Result<(), VcsError> {
        let command: DocumentCommand<Operation> =
            serde_json::from_str(command_json).map_err(|e| VcsError::Deserialize(e.to_string()))?;
        self.dispatch(command)
    }

    pub fn envelope_json(&self) -> Result<String, VcsError> {
        serde_json::to_string(&self.envelope).map_err(|e| VcsError::Serialize(e.to_string()))
    }

    pub fn projection_json(&self) -> Result<String, VcsError> {
        let projection = self.projection()?;
        serde_json::to_string(&projection).map_err(|e| VcsError::Serialize(e.to_string()))
    }

    /// @emoji 🔗 Attaches a backbone channel, reconciling any already-persisted state before
    /// seeding it with this store's current snapshot.
    pub fn attach_backbone(&mut self, backbone: Box<dyn Backbone>) -> Result<(), VcsError> {
        self.envelope.backbone = Some(backbone.descriptor());
        self.backbone = Some(backbone);
        self.pump()?;
        self.flush_outbound(false)?;
        self.bump();
        Ok(())
    }

    /// @emoji 🔗 Resolves a backbone URI and attaches it. Only available inside the wasm sandbox,
    /// where every scheme forwards to the host over the injected {@link BackboneChannelPort} (a pure
    /// queue). On native targets, callers attach an explicit `Box<dyn Backbone>` via
    /// {@link attach_backbone} — the `framework/sync` actor layer owns all IO-performing endpoints.
    #[cfg(target_arch = "wasm32")]
    pub fn attach_backbone_uri(&mut self, uri: &str) -> Result<(), VcsError> {
        self.attach_backbone(resolve_backbone(uri)?)
    }

    /// @emoji ✂️ Detaches the backbone; the WIP graph stays in memory, simply unsynchronized.
    pub fn detach_backbone(&mut self) -> Option<Box<dyn Backbone>> {
        self.envelope.backbone = None;
        self.bump();
        self.backbone.take()
    }

    pub fn backbone_ref(&self) -> Option<&DocumentBackboneRef> {
        self.envelope.backbone.as_ref()
    }

    /// @emoji 📡 Drains inbound backbone messages into the edit timeline. Safe to call anytime;
    /// `dispatch` already calls this before every command.
    pub fn tick(&mut self) -> Result<bool, VcsError> {
        self.pump()
    }

    /// @emoji 🕸️ Feeds a remote {@link OperationEnvelope} through the causal DAG, applying it (and any
    /// now-unblocked dependents) into the edit timeline. Closes the sync gap between
    /// `framework/sync`'s `OpDag` and the vcs edit history.
    pub fn ingest_remote(&mut self, envelope: OperationEnvelope) -> Result<(), VcsError> {
        self.dag
            .insert(envelope)
            .map_err(|error| VcsError::Backbone(error.to_string()))?;
        for envelope in self.dag.drain_applied_envelopes() {
            self.ingest_envelope(envelope)?;
        }
        Ok(())
    }

    fn ingest_envelope(&mut self, envelope: OperationEnvelope) -> Result<(), VcsError> {
        let mut edit: Edit<Operation> = edit_from_operation_envelope(&envelope)?;
        edit.actor = Some(envelope.actor.0.clone());
        if self.envelope.vcs.edits.iter().any(|existing| existing.id == edit.id) {
            return Ok(());
        }
        self.edit_sequence = self.edit_sequence.max(edit.sequence_number);
        let edit_id = edit.id.clone();
        // ⚡ Fold just the new edit's forwards onto the existing `current` (which already reflects
        // every prior applied edit) — algebraically identical to a full raw-fold replay, in O(new ops).
        for operation in &edit.forwards {
            self.current = apply_operation(&self.current, operation);
        }
        self.envelope.vcs.edits.push(edit);
        self.applied_edit_ids.push(edit_id);
        self.tail_undo_cache = None;
        // 🤝 Tail reconciliation hook: remote ingestion is the one path where this store's projection
        // can diverge from what a local `Apply` alone would produce, so refresh conflicts here.
        let (_, conflicts) = reconcile_with_last(self.last_applied_operation(), self.current.clone());
        self.conflicts = conflicts;
        self.bump();
        Ok(())
    }

    fn merge_remote_snapshot(&mut self, envelope_json: &str) -> Result<(), VcsError> {
        let remote: DocumentEnvelope<P, Operation> =
            serde_json::from_str(envelope_json).map_err(|e| VcsError::Deserialize(e.to_string()))?;
        if self.envelope.vcs.edits.is_empty() {
            let applied: Vec<String> = remote.vcs.edits.iter().map(|edit| edit.id.clone()).collect();
            self.edit_sequence = remote
                .vcs
                .edits
                .iter()
                .map(|edit| edit.sequence_number)
                .max()
                .unwrap_or(0);
            let backbone_ref = self.envelope.backbone.clone();
            self.envelope = remote;
            self.envelope.backbone = backbone_ref;
            // 🌱 A snapshot adopts these edits directly (not through `dag.insert`), so the dag never
            // learns they're satisfied — seed it here or a later envelope whose `deps` point back at
            // one of these ids would sit `Pending` forever (see `OpDag::seed_applied`).
            self.dag.seed_applied(applied.iter().cloned());
            self.applied_edit_ids = applied;
            self.redo_edit_ids.clear();
            self.tail_undo_cache = None;
            // 🔂 Wholesale replacement, not a tail append — cold-path full raw-fold recompute.
            self.current = self.fold_current()?;
            self.bump();
            return Ok(());
        }
        let existing_edit_ids: HashSet<String> = self.envelope.vcs.edits.iter().map(|edit| edit.id.clone()).collect();
        let mut newly_merged_ids: Vec<String> = Vec::new();
        for edit in remote.vcs.edits {
            if existing_edit_ids.contains(&edit.id) {
                continue;
            }
            self.edit_sequence = self.edit_sequence.max(edit.sequence_number);
            self.applied_edit_ids.push(edit.id.clone());
            newly_merged_ids.push(edit.id.clone());
            // ⚡ Each newly-merged edit is appended at the tail, so folding its forwards onto `current`
            // in iteration order is exactly a prefix-extension of the existing raw fold.
            for operation in &edit.forwards {
                self.current = apply_operation(&self.current, operation);
            }
            self.envelope.vcs.edits.push(edit);
        }
        self.dag.seed_applied(newly_merged_ids);
        merge_by_id(&mut self.envelope.vcs.changes, remote.vcs.changes, |change| &change.id);
        merge_by_id(&mut self.envelope.vcs.checkpoints, remote.vcs.checkpoints, |checkpoint| &checkpoint.id);
        merge_by_id(&mut self.envelope.vcs.alternatives, remote.vcs.alternatives, |alternative| &alternative.id);
        self.tail_undo_cache = None;
        self.bump();
        Ok(())
    }

    fn previous_edit_dependency(&self) -> Vec<OperationId> {
        let len = self.applied_edit_ids.len();
        if len >= 2 {
            vec![OperationId(self.applied_edit_ids[len - 2].clone())]
        } else {
            Vec::new()
        }
    }

    /// @emoji 📥 Pumps every queued inbound message from the attached backbone into the timeline.
    fn pump(&mut self) -> Result<bool, VcsError> {
        let Some(mut backbone) = self.backbone.take() else {
            return Ok(false);
        };
        let received = backbone.receive();
        self.backbone = Some(backbone);
        let messages = received?;
        if messages.is_empty() {
            return Ok(false);
        }
        let mut acked_op_ids: Vec<String> = Vec::new();
        for message in messages {
            match message {
                BackboneMessage::Snapshot { envelope_json } => self.merge_remote_snapshot(&envelope_json)?,
                BackboneMessage::Operations { envelopes } => {
                    let op_ids: Vec<String> = envelopes.iter().map(|envelope| envelope.id.0.clone()).collect();
                    for envelope in envelopes {
                        self.ingest_remote(envelope)?;
                    }
                    acked_op_ids.extend(op_ids);
                }
                // A store never consumes acks (they flow store→actor); drain and ignore any that echo back.
                BackboneMessage::Ack { .. } => {}
            }
        }
        if !acked_op_ids.is_empty() {
            if let Some(mut backbone) = self.backbone.take() {
                let result = backbone.send(BackboneMessage::Ack { op_ids: acked_op_ids });
                self.backbone = Some(backbone);
                result?;
            }
        }
        Ok(true)
    }

    /// @emoji 📤 Sends the just-applied change outward: a single {@link OperationEnvelope} for `Apply`,
    /// or a full snapshot for every structural command (undo/redo/checkpoint/alternative/amend).
    fn flush_outbound(&mut self, is_apply: bool) -> Result<(), VcsError> {
        let Some(mut backbone) = self.backbone.take() else {
            return Ok(());
        };
        let result = if is_apply {
            match self.envelope.vcs.edits.last() {
                Some(edit) => {
                    let deps = self.previous_edit_dependency();
                    match operation_envelope_from_edit(&self.envelope, edit, deps) {
                        Ok(op_envelope) => {
                            // Registers this locally-authored edit as already-applied in our own
                            // DAG, so a later remote envelope that depends on it doesn't stall as pending.
                            let _ = self.dag.insert(op_envelope.clone());
                            backbone.send(BackboneMessage::Operations { envelopes: vec![op_envelope] })
                        }
                        Err(error) => Err(error),
                    }
                }
                None => Ok(()),
            }
        } else {
            self.envelope_json()
                .and_then(|json| backbone.send(BackboneMessage::Snapshot { envelope_json: json }))
        };
        self.backbone = Some(backbone);
        result
    }

    /// @emoji 🖋️ Whether `edit_id` was authored by the local actor. Unauthored (legacy) edits count
    /// as local; every other actor is foreign and must not be undone by this store.
    fn edit_is_local(&self, edit_id: &str) -> bool {
        self.envelope
            .vcs
            .edits
            .iter()
            .find(|edit| edit.id == edit_id)
            .map(|edit| edit.actor.is_none() || edit.actor.as_deref() == self.local_actor_id.as_deref())
            .unwrap_or(false)
    }

    fn bump(&mut self) {
        self.generation += 1;
    }
}

fn merge_by_id<T: Clone>(local: &mut Vec<T>, remote: Vec<T>, id_of: impl Fn(&T) -> &String) {
    let mut existing: HashSet<String> = local.iter().map(|item| id_of(item).clone()).collect();
    for item in remote {
        if existing.insert(id_of(&item).clone()) {
            local.push(item);
        }
    }
}

/// @emoji 📦 Serializes an `Edit` into the causal wire envelope exchanged over a backbone channel.
pub fn operation_envelope_from_edit<P, Operation>(
    envelope: &DocumentEnvelope<P, Operation>,
    edit: &Edit<Operation>,
    deps: Vec<OperationId>,
) -> Result<OperationEnvelope, VcsError>
where
    Operation: Serialize,
{
    let payload = serde_json::to_value(edit).map_err(|e| VcsError::Serialize(e.to_string()))?;
    let payload_hash = hash_bytes(&serde_json::to_vec(edit).unwrap_or_default());
    let author_id = edit
        .operation_meta
        .last()
        .and_then(|meta| meta.author_id.clone())
        .map(|actor_id| actor_id.0)
        .unwrap_or_else(|| "local".into());
    // 🎞️ CW3: `OperationMeta.undo_policy` is now `protocol::UndoPolicy` (the moved struct's field
    // type), while this envelope's `InverseOperation.undo_policy` stays `semio_framework_core`'s own
    // (kept local this wave — see the kernel cut-over note on `framework/core`'s `UndoPolicy`); both
    // enums share identical variants, so this is a plain, lossless re-tag, not a real conversion.
    let undo_policy = match edit.operation_meta.last().map(|meta| meta.undo_policy) {
        Some(protocol::UndoPolicy::ExactBaseOnly) | None => UndoPolicy::ExactBaseOnly,
        Some(protocol::UndoPolicy::TransformAgainstConcurrent) => UndoPolicy::TransformAgainstConcurrent,
        Some(protocol::UndoPolicy::SemanticUndo) => UndoPolicy::SemanticUndo,
        Some(protocol::UndoPolicy::CompensatingAction) => UndoPolicy::CompensatingAction,
    };
    Ok(OperationEnvelope {
        id: OperationId(edit.id.clone()),
        actor: ActorId(author_id),
        document: DocumentId(envelope.id.clone()),
        schema_version: SchemaVersion(envelope.schema.clone()),
        deps: deps.clone(),
        payload_hash: PayloadHash(payload_hash),
        diff: DocumentDiff {
            schema_id: SchemaId(envelope.schema.clone()),
            payload,
        },
        inverse: InverseOperation {
            target_operation: OperationId(edit.id.clone()),
            inverse_diff: DocumentDiff {
                schema_id: SchemaId(envelope.schema.clone()),
                payload: serde_json::json!({ "backwards": edit.backwards }),
            },
            base_version: DocumentVersion(edit.sequence_number as u64),
            dependencies: deps,
            undo_policy,
        },
    })
}

/// @emoji 📦 Recovers an `Edit` from the causal wire envelope produced by `operation_envelope_from_edit`.
pub fn edit_from_operation_envelope<Operation>(envelope: &OperationEnvelope) -> Result<Edit<Operation>, VcsError>
where
    Operation: DeserializeOwned,
{
    serde_json::from_value(envelope.diff.payload.clone()).map_err(|e| VcsError::Deserialize(e.to_string()))
}
//#endregion 🔖DocumentStore

//#region 🔖Backbone
//#region 🔖Backbone
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StudioConflict {
    pub kind: String,
    pub uri: String,
    pub message: String,
}

/// @emoji 🎞️ Maps `protocol_command::Operation::reconcile`'s new `Vec<ReconcileReport>` result onto
/// this crate's own conflict type — see `reconcile_with_last`'s doc comment for why the mapping
/// happens at this edge rather than `protocol_command` knowing about `StudioConflict` directly.
/// `kind: report.id` verbatim (NOT prefixed with severity) — a technology's own `reconcile` override
/// (e.g. `framework/product/os/core`'s `OsOperation`) round-trips its own `StudioConflict.kind`
/// through `ReconcileReport.id` on the way in (see that crate's `reconcile` wrapper), and callers
/// pattern-match `StudioConflict.kind` against exact strings (e.g. `"media-graph/edge-orphaned"`) —
/// mangling it here would silently break every such exact-match call site. `severity` has no
/// `StudioConflict` field to land in, so it is dropped (a real, structural information loss inherent
/// to `ReconcileReport`'s frozen shape, not fixable at this edge). `ReconcileReport` also has no
/// URI-shaped field (it targets a schema-opaque `id`, not a studio member resource), so `uri` is
/// left empty for any report that didn't originate from a `StudioConflict` round-trip.
impl From<ReconcileReport> for StudioConflict {
    fn from(report: ReconcileReport) -> Self {
        StudioConflict {
            kind: report.id,
            uri: String::new(),
            message: report.message,
        }
    }
}

/// @emoji 📨 Wire message exchanged over an attached backbone channel.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum BackboneMessage {
    Snapshot { envelope_json: String },
    Operations { envelopes: Vec<OperationEnvelope> },
    /// @emoji ✅ Acknowledges inbound operations the store has ingested (store→actor). Lets a future actor
    /// implement at-least-once redelivery with id-based dedupe — safe across store crashes/reloads.
    Ack { op_ids: Vec<String> },
}

/// @emoji 🧵 Non-blocking, IO-free in-memory queue contract between a `DocumentStore` and its
/// sync actor. `send`/`receive` MUST return immediately: implementations only enqueue/dequeue
/// `BackboneMessage`s — never HTTP, never filesystem, never a blocking wait. All IO (persistence,
/// hub sync, file watching, presence) lives behind this queue in `framework/sync`'s actor layer,
/// which owns the other end; the store's `pump()`/`flush_outbound()` run synchronously on the
/// caller's thread and must never be blocked by transport work.
///
/// URI schemes are resolved by the host actor (`framework/sync`): `temp://` (in-memory),
/// `file://` (single JSON blob), `folder://` (sqlite `.semio/document.db`), `remote://` (OS hub).
pub trait Backbone: Send + Sync {
    fn descriptor(&self) -> DocumentBackboneRef;
    fn send(&mut self, message: BackboneMessage) -> Result<(), VcsError>;
    fn receive(&mut self) -> Result<Vec<BackboneMessage>, VcsError>;
}

pub trait BackbonePort: Send + Sync {
    fn read(&self, uri: &str) -> Result<String, VcsError>;
    fn write(&self, uri: &str, payload: &str) -> Result<(), VcsError>;
}

static HOST_BACKBONE_PORT: Mutex<Option<Arc<dyn BackbonePort>>> = Mutex::new(None);

/// @emoji 🔌 Injects the browser or dev-server backbone port for wasm file/folder IO.
pub fn set_host_backbone_port(port: Arc<dyn BackbonePort>) {
    if let Ok(mut guard) = HOST_BACKBONE_PORT.lock() {
        *guard = Some(port);
    }
}

fn host_backbone_port() -> Option<Arc<dyn BackbonePort>> {
    HOST_BACKBONE_PORT.lock().ok().and_then(|guard| guard.clone())
}

#[derive(Default)]
pub struct MemoryBackbonePort {
    files: Mutex<HashMap<String, String>>,
}

impl MemoryBackbonePort {
    pub fn new() -> Self {
        Self::default()
    }
}

impl BackbonePort for MemoryBackbonePort {
    fn read(&self, uri: &str) -> Result<String, VcsError> {
        self.files
            .lock()
            .map_err(|_| VcsError::Backbone("lock poisoned".into()))?
            .get(uri)
            .cloned()
            .ok_or_else(|| VcsError::Backbone(format!("missing backbone file {uri}")))
    }

    fn write(&self, uri: &str, payload: &str) -> Result<(), VcsError> {
        self.files
            .lock()
            .map_err(|_| VcsError::Backbone("lock poisoned".into()))?
            .insert(uri.to_string(), payload.to_string());
        Ok(())
    }
}

#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
fn local_storage_backbone_key(uri: &str) -> String {
    format!("semio:vcs:{uri}")
}

/// @emoji 💾 Browser `localStorage` backbone port with in-memory fallback for native tests.
pub struct LocalStorageBackbonePort {
    fallback: MemoryBackbonePort,
}

impl LocalStorageBackbonePort {
    pub fn new() -> Self {
        Self {
            fallback: MemoryBackbonePort::new(),
        }
    }
}

impl Default for LocalStorageBackbonePort {
    fn default() -> Self {
        Self::new()
    }
}

impl BackbonePort for LocalStorageBackbonePort {
    fn read(&self, uri: &str) -> Result<String, VcsError> {
        if let Some(port) = host_backbone_port() {
            if let Ok(value) = port.read(uri) {
                return Ok(value);
            }
        }
        #[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
        {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    if let Ok(Some(value)) = storage.get_item(&local_storage_backbone_key(uri)) {
                        return Ok(value);
                    }
                }
            }
        }
        self.fallback.read(uri)
    }

    fn write(&self, uri: &str, payload: &str) -> Result<(), VcsError> {
        self.fallback.write(uri, payload)?;
        if let Some(port) = host_backbone_port() {
            let _ = port.write(uri, payload);
        }
        #[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
        {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    let _ = storage.set_item(&local_storage_backbone_key(uri), payload);
                }
            }
        }
        Ok(())
    }
}

/// @emoji 🕸️ Injectable duplex transport across the wasm sandbox boundary (plugin ↔ host process).
pub trait BackboneChannelPort: Send + Sync {
    fn send(&self, uri: &str, message_json: &str) -> Result<(), VcsError>;
    fn poll(&self, uri: &str) -> Result<Vec<String>, VcsError>;
}

static HOST_BACKBONE_CHANNEL: Mutex<Option<Arc<dyn BackboneChannelPort>>> = Mutex::new(None);

/// @emoji 🔌 Injects the plugin host's duplex backbone channel for wasm-sandboxed document stores.
pub fn set_host_backbone_channel(channel: Arc<dyn BackboneChannelPort>) {
    if let Ok(mut guard) = HOST_BACKBONE_CHANNEL.lock() {
        *guard = Some(channel);
    }
}

fn host_backbone_channel() -> Option<Arc<dyn BackboneChannelPort>> {
    HOST_BACKBONE_CHANNEL.lock().ok().and_then(|guard| guard.clone())
}

/// @emoji 🧵 Backbone that forwards messages across the wasm sandbox boundary to the host process,
/// which resolves the real `file://`/`folder://`/`remote://` backbone on its own (native) side.
pub struct PortBackbone {
    uri: String,
}

impl PortBackbone {
    pub fn new(uri: &str) -> Self {
        Self { uri: uri.to_string() }
    }
}

impl Backbone for PortBackbone {
    fn descriptor(&self) -> DocumentBackboneRef {
        document_backbone_ref(&self.uri)
    }

    fn send(&mut self, message: BackboneMessage) -> Result<(), VcsError> {
        let channel = host_backbone_channel()
            .ok_or_else(|| VcsError::Backbone("backbone channel requires host port".into()))?;
        let json = serde_json::to_string(&message).map_err(|e| VcsError::Serialize(e.to_string()))?;
        channel.send(&self.uri, &json)
    }

    fn receive(&mut self) -> Result<Vec<BackboneMessage>, VcsError> {
        let channel = host_backbone_channel()
            .ok_or_else(|| VcsError::Backbone("backbone channel requires host port".into()))?;
        channel
            .poll(&self.uri)?
            .into_iter()
            .map(|json| serde_json::from_str(&json).map_err(|e| VcsError::Deserialize(e.to_string())))
            .collect()
    }
}

/// @emoji 🔗 Two crossed in-memory channel ends: whatever `a` sends, `b` receives, and vice versa.
pub struct MemoryBackbone {
    uri: String,
    inbox: Arc<Mutex<VecDeque<BackboneMessage>>>,
    outbox: Arc<Mutex<VecDeque<BackboneMessage>>>,
}

impl MemoryBackbone {
    pub fn pair(uri_a: &str, uri_b: &str) -> (Self, Self) {
        let a_to_b = Arc::new(Mutex::new(VecDeque::new()));
        let b_to_a = Arc::new(Mutex::new(VecDeque::new()));
        (
            Self {
                uri: uri_a.to_string(),
                inbox: b_to_a.clone(),
                outbox: a_to_b.clone(),
            },
            Self {
                uri: uri_b.to_string(),
                inbox: a_to_b,
                outbox: b_to_a,
            },
        )
    }
}

impl Backbone for MemoryBackbone {
    fn descriptor(&self) -> DocumentBackboneRef {
        document_backbone_ref(&self.uri)
    }

    fn send(&mut self, message: BackboneMessage) -> Result<(), VcsError> {
        self.outbox
            .lock()
            .map_err(|_| VcsError::Backbone("lock poisoned".into()))?
            .push_back(message);
        Ok(())
    }

    fn receive(&mut self) -> Result<Vec<BackboneMessage>, VcsError> {
        let mut inbox = self.inbox.lock().map_err(|_| VcsError::Backbone("lock poisoned".into()))?;
        Ok(inbox.drain(..).collect())
    }
}

/// @emoji 🔗 The store-side end of a pair of crossed in-memory queues. Implements the non-blocking
/// {@link Backbone} contract; the matching {@link ChannelBackboneRemote} is held by an external sync
/// actor (built in `framework/sync`, a later workstream) that pushes inbound messages and drains the
/// store's outbound ones. This crate only provides the queue plumbing — never the actor itself.
pub struct ChannelBackbone {
    uri: String,
    inbound: Arc<Mutex<VecDeque<BackboneMessage>>>,
    outbound: Arc<Mutex<VecDeque<BackboneMessage>>>,
}

/// @emoji 🎛️ The actor-side end paired with a {@link ChannelBackbone}: `push` delivers a message to
/// the store's inbound queue, `drain` collects everything the store has sent outbound. Not a
/// `Backbone` — this is the handle an IO-owning actor endpoint holds across the store boundary.
pub struct ChannelBackboneRemote {
    uri: String,
    inbound: Arc<Mutex<VecDeque<BackboneMessage>>>,
    outbound: Arc<Mutex<VecDeque<BackboneMessage>>>,
}

impl ChannelBackbone {
    /// @emoji 🔗 Creates a crossed pair sharing a URI: the store attaches the `ChannelBackbone`; the
    /// actor keeps the `ChannelBackboneRemote`.
    pub fn pair(uri: &str) -> (ChannelBackbone, ChannelBackboneRemote) {
        let inbound = Arc::new(Mutex::new(VecDeque::new()));
        let outbound = Arc::new(Mutex::new(VecDeque::new()));
        (
            ChannelBackbone {
                uri: uri.to_string(),
                inbound: inbound.clone(),
                outbound: outbound.clone(),
            },
            ChannelBackboneRemote {
                uri: uri.to_string(),
                inbound,
                outbound,
            },
        )
    }
}

impl Backbone for ChannelBackbone {
    fn descriptor(&self) -> DocumentBackboneRef {
        document_backbone_ref(&self.uri)
    }

    fn send(&mut self, message: BackboneMessage) -> Result<(), VcsError> {
        self.outbound
            .lock()
            .map_err(|_| VcsError::Backbone("lock poisoned".into()))?
            .push_back(message);
        Ok(())
    }

    fn receive(&mut self) -> Result<Vec<BackboneMessage>, VcsError> {
        let mut inbound = self.inbound.lock().map_err(|_| VcsError::Backbone("lock poisoned".into()))?;
        Ok(inbound.drain(..).collect())
    }
}

impl ChannelBackboneRemote {
    pub fn descriptor(&self) -> DocumentBackboneRef {
        document_backbone_ref(&self.uri)
    }

    /// @emoji 📥 Delivers a message to the store's inbound queue (actor→store).
    pub fn push(&self, message: BackboneMessage) -> Result<(), VcsError> {
        self.inbound
            .lock()
            .map_err(|_| VcsError::Backbone("lock poisoned".into()))?
            .push_back(message);
        Ok(())
    }

    /// @emoji 📤 Collects everything the store has sent outbound (store→actor), draining the queue.
    pub fn drain(&self) -> Result<Vec<BackboneMessage>, VcsError> {
        let mut outbound = self.outbound.lock().map_err(|_| VcsError::Backbone("lock poisoned".into()))?;
        Ok(outbound.drain(..).collect())
    }
}


/// @emoji 🔌 Resolves a backbone URI to a concrete channel implementation. Only available inside the
/// wasm sandbox, where every scheme forwards to the host process over the injected
/// {@link BackboneChannelPort} (a pure in-memory queue). Native IO-performing backbones moved out of
/// this crate entirely — the `framework/sync` actor layer owns them.
#[cfg(target_arch = "wasm32")]
pub fn resolve_backbone(uri: &str) -> Result<Box<dyn Backbone>, VcsError> {
    Ok(Box::new(PortBackbone::new(uri)))
}
//#endregion 🔖Backbone

//#region 🔖BlobStore
//#region 🔖BlobStore
/// @emoji 📦 A content-addressed blob's identity + metadata. Never carries the bytes themselves —
/// callers that just put/read a blob already hold those; this is what gets embedded in a document
/// (e.g. a `MergeStrategyKind::ContentAddressedBlob` field) to reference it durably.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlobRef {
    pub hash: String,
    pub size: u64,
    pub media_type: String,
}

/// @emoji 🗄️ Content-addressed blob persistence backing `MergeStrategyKind::ContentAddressedBlob` /
/// `DocumentKind::ContentAddressedBlob` (`framework/core/rs` 🔖MergeStrategy region). `put` is idempotent —
/// it dedupes by the Blake3 hash of the bytes ({@link semio_framework_hash::hash_bytes}), so writing
/// the same content twice never rewrites storage. Implementors decide the backing medium (sqlite here,
/// a hub HTTP route in a later ticket, an IndexedDB cache in the browser).
pub trait BlobStore: Send + Sync {
    fn put(&self, bytes: &[u8], media_type: &str) -> Result<BlobRef, VcsError>;
    fn get(&self, hash: &str) -> Result<Option<Vec<u8>>, VcsError>;
    fn has(&self, hash: &str) -> Result<bool, VcsError>;
    fn delete(&self, hash: &str) -> Result<(), VcsError>;
}
//#endregion 🔖BlobStore

//#region 🔖Studio
//#region StudioMember
/// @emoji 🧑‍🤝‍🧑 Object-safe façade over a `DocumentStore<P, Operation>` so a studio host can hold a
/// heterogeneous registry of documents (`HashMap<String, Box<dyn StudioMember>>`) without knowing
/// each member's concrete `P`/`Operation`. Blanket-implemented below by delegating to `dispatch` — never
/// reimplement the underlying VCS mechanics here.
pub trait StudioMember {
    fn document_id(&self) -> &str;
    /// @emoji 🩸 Whether this member has edits applied since its last checkpoint (mirrors the
    /// `CommitCheckpoint` dispatch's own "nothing to commit" check via `uncommitted_edit_ids`).
    fn is_dirty(&self) -> bool;
    fn commit_checkpoint(&mut self, message: String, authors: Vec<Author>) -> Result<String, VcsError>;
    fn current_checkpoint_id(&self) -> Option<String>;
    fn current_alternative_id(&self) -> Option<String>;
    fn checkout(&mut self, checkpoint_id: &str, alternative_id: &str) -> Result<(), VcsError>;
    fn create_alternative(&mut self, name: String) -> Result<String, VcsError>;
    // 🎞️ CW3: `protocol::HybridLogicalTimestamp` (not `semio_framework_core`'s local one) — these
    // read `OperationMeta.timestamp`, which is the moved struct's field, typed against protocol_core.
    fn last_local_edit_timestamp(&self) -> Option<protocol::HybridLogicalTimestamp>;
    fn last_undone_local_edit_timestamp(&self) -> Option<protocol::HybridLogicalTimestamp>;
    fn undo(&mut self) -> Result<(), VcsError>;
    fn redo(&mut self) -> Result<(), VcsError>;
    /// @emoji 🪄 Downcast escape hatch: a studio host UI (or a test) needs the concrete
    /// `DocumentStore<P, Operation>` back out of a `Box<dyn StudioMember>` — e.g. to `Apply` a
    /// technology-specific `Operation`, which can't appear in this object-safe trait. `Self: 'static` is
    /// implied by every real `P`/`Operation` pair, so this never fails for a genuine member.
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

impl<P, Operation> StudioMember for DocumentStore<P, Operation>
where
    P: Clone + Serialize + DeserializeOwned + 'static,
    Operation: Clone + Serialize + DeserializeOwned + crate::Operation<P> + 'static,
{
    fn document_id(&self) -> &str {
        self.envelope().id.as_str()
    }

    fn is_dirty(&self) -> bool {
        !uncommitted_edit_ids(&self.envelope, self.applied_edit_ids()).is_empty()
    }

    fn commit_checkpoint(&mut self, message: String, authors: Vec<Author>) -> Result<String, VcsError> {
        self.dispatch(DocumentCommand::CommitCheckpoint {
            message: Some(message),
            authors,
        })?;
        // `self.current_checkpoint_id()` resolves to the inherent method (`Option<&str>`), not this
        // trait method — Rust prefers inherent methods over trait methods of the same name.
        self.current_checkpoint_id().map(|id| id.to_string()).ok_or(VcsError::NoCheckpoint)
    }

    fn current_checkpoint_id(&self) -> Option<String> {
        self.current_checkpoint_id().map(|id| id.to_string())
    }

    fn current_alternative_id(&self) -> Option<String> {
        self.envelope().active_alternative_id.clone()
    }

    fn checkout(&mut self, checkpoint_id: &str, alternative_id: &str) -> Result<(), VcsError> {
        if !alternative_id.is_empty() {
            let is_alternative_tip = self
                .envelope()
                .vcs
                .alternatives
                .iter()
                .find(|alternative| alternative.id == alternative_id)
                .map(|alternative| alternative.checkpoint_ids.last().map(String::as_str) == Some(checkpoint_id))
                .unwrap_or(false);
            if is_alternative_tip {
                return self.dispatch(DocumentCommand::SwitchAlternative {
                    alternative_id: alternative_id.to_string(),
                });
            }
        }
        self.dispatch(DocumentCommand::CheckoutCheckpoint {
            checkpoint_id: checkpoint_id.to_string(),
        })
    }

    fn create_alternative(&mut self, name: String) -> Result<String, VcsError> {
        self.dispatch(DocumentCommand::CreateAlternative { name })?;
        self.envelope().active_alternative_id.clone().ok_or(VcsError::NoCheckpoint)
    }

    fn last_local_edit_timestamp(&self) -> Option<protocol::HybridLogicalTimestamp> {
        self.applied_edit_ids().iter().rev().find_map(|edit_id| {
            if !self.edit_is_local(edit_id) {
                return None;
            }
            self.envelope()
                .vcs
                .edits
                .iter()
                .find(|edit| edit.id == *edit_id)
                .and_then(|edit| edit.operation_meta.last())
                .map(|meta| meta.timestamp)
        })
    }

    fn last_undone_local_edit_timestamp(&self) -> Option<protocol::HybridLogicalTimestamp> {
        self.redo_edit_ids().iter().rev().find_map(|edit_id| {
            if !self.edit_is_local(edit_id) {
                return None;
            }
            self.envelope()
                .vcs
                .edits
                .iter()
                .find(|edit| edit.id == *edit_id)
                .and_then(|edit| edit.operation_meta.last())
                .map(|meta| meta.timestamp)
        })
    }

    fn undo(&mut self) -> Result<(), VcsError> {
        self.dispatch(DocumentCommand::Undo)
    }

    fn redo(&mut self) -> Result<(), VcsError> {
        self.dispatch(DocumentCommand::Redo)
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
//#endregion StudioMember

//#region StudioHistoryDocument
/// @emoji 📌 One member document's position at the moment a `StudioCheckpoint` was recorded.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StudioMemberPin {
    pub document_id: String,
    pub checkpoint_id: String,
    /// @emoji 🌿 Empty string when the member had no active alternative (its own trunk) at pin time.
    #[serde(default)]
    pub alternative_id: String,
}

/// @emoji 🗄️ A studio-wide checkpoint: one pin per registered member, so checking it out (or an
/// alternative built on top of it) fans out deterministically to every member's own VCS.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StudioCheckpoint {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    pub message: String,
    pub authors: Vec<Author>,
    pub timestamp: HybridLogicalTimestamp,
    pub members: Vec<StudioMemberPin>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StudioAlternative {
    pub id: String,
    pub name: String,
    pub checkpoint_ids: Vec<String>,
}

/// @emoji 🗄️ Projection of the `"os.studio.history"` meta-document: itself an ordinary `DocumentVcs`
/// document kind (dogfooded — no bespoke transport), holding the studio-level checkpoint/alternative
/// graph that `StudioHost` composes on top of every registered member's own history.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StudioHistoryProjection {
    pub checkpoints: Vec<StudioCheckpoint>,
    pub alternatives: Vec<StudioAlternative>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_alternative_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "operation", rename_all = "camelCase")]
pub enum StudioHistoryOperation {
    CommitStudioCheckpoint { checkpoint: StudioCheckpoint },
    CreateStudioAlternative { alternative: StudioAlternative },
    SwitchStudioAlternative { alternative_id: String },
    /// @emoji ↩️ Mechanical inverse of `CommitStudioCheckpoint`; never dispatched directly by
    /// `StudioHost` (studio undo is derived and member-local, see `StudioHost::undo`), only
    /// produced by `backwards` for VCS round-trip correctness.
    RemoveStudioCheckpoint { checkpoint_id: String },
    /// @emoji ↩️ Mechanical inverse of `CreateStudioAlternative`; see `RemoveStudioCheckpoint`.
    RemoveStudioAlternative { alternative_id: String },
    /// @emoji ↩️ Mechanical inverse of `SwitchStudioAlternative`; see `RemoveStudioCheckpoint`.
    SetActiveStudioAlternative { alternative_id: Option<String> },
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StudioHistoryDiff {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_checkpoint: Option<StudioCheckpoint>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_checkpoint_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_alternative: Option<StudioAlternative>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_alternative_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub set_active_alternative_id: Option<Option<String>>,
}

impl OperationDiff<StudioHistoryProjection> for StudioHistoryDiff {
    fn apply(&self, projection: &StudioHistoryProjection) -> StudioHistoryProjection {
        let mut next = projection.clone();
        if let Some(checkpoint) = &self.add_checkpoint {
            next.checkpoints.push(checkpoint.clone());
        }
        if let Some(checkpoint_id) = &self.remove_checkpoint_id {
            next.checkpoints.retain(|checkpoint| checkpoint.id != *checkpoint_id);
        }
        if let Some(alternative) = &self.add_alternative {
            next.alternatives.push(alternative.clone());
        }
        if let Some(alternative_id) = &self.remove_alternative_id {
            next.alternatives.retain(|alternative| alternative.id != *alternative_id);
        }
        if let Some(active) = &self.set_active_alternative_id {
            next.active_alternative_id = active.clone();
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        if other.add_checkpoint.is_some() {
            self.add_checkpoint = other.add_checkpoint;
        }
        if other.remove_checkpoint_id.is_some() {
            self.remove_checkpoint_id = other.remove_checkpoint_id;
        }
        if other.add_alternative.is_some() {
            self.add_alternative = other.add_alternative;
        }
        if other.remove_alternative_id.is_some() {
            self.remove_alternative_id = other.remove_alternative_id;
        }
        if other.set_active_alternative_id.is_some() {
            self.set_active_alternative_id = other.set_active_alternative_id;
        }
    }
}

impl Operation<StudioHistoryProjection> for StudioHistoryOperation {
    type Diff = StudioHistoryDiff;

    fn diff(&self, _projection: &StudioHistoryProjection) -> StudioHistoryDiff {
        match self {
            StudioHistoryOperation::CommitStudioCheckpoint { checkpoint } => StudioHistoryDiff {
                add_checkpoint: Some(checkpoint.clone()),
                ..Default::default()
            },
            StudioHistoryOperation::CreateStudioAlternative { alternative } => StudioHistoryDiff {
                add_alternative: Some(alternative.clone()),
                set_active_alternative_id: Some(Some(alternative.id.clone())),
                ..Default::default()
            },
            StudioHistoryOperation::SwitchStudioAlternative { alternative_id } => StudioHistoryDiff {
                set_active_alternative_id: Some(Some(alternative_id.clone())),
                ..Default::default()
            },
            StudioHistoryOperation::RemoveStudioCheckpoint { checkpoint_id } => StudioHistoryDiff {
                remove_checkpoint_id: Some(checkpoint_id.clone()),
                ..Default::default()
            },
            StudioHistoryOperation::RemoveStudioAlternative { alternative_id } => StudioHistoryDiff {
                remove_alternative_id: Some(alternative_id.clone()),
                ..Default::default()
            },
            StudioHistoryOperation::SetActiveStudioAlternative { alternative_id } => StudioHistoryDiff {
                set_active_alternative_id: Some(alternative_id.clone()),
                ..Default::default()
            },
        }
    }

    fn backwards(&self, projection: &StudioHistoryProjection) -> Vec<Self> {
        match self {
            StudioHistoryOperation::CommitStudioCheckpoint { checkpoint } => {
                vec![StudioHistoryOperation::RemoveStudioCheckpoint {
                    checkpoint_id: checkpoint.id.clone(),
                }]
            }
            StudioHistoryOperation::CreateStudioAlternative { alternative } => vec![
                StudioHistoryOperation::SetActiveStudioAlternative {
                    alternative_id: projection.active_alternative_id.clone(),
                },
                StudioHistoryOperation::RemoveStudioAlternative {
                    alternative_id: alternative.id.clone(),
                },
            ],
            StudioHistoryOperation::SwitchStudioAlternative { .. } => vec![StudioHistoryOperation::SetActiveStudioAlternative {
                alternative_id: projection.active_alternative_id.clone(),
            }],
            StudioHistoryOperation::RemoveStudioCheckpoint { checkpoint_id } => projection
                .checkpoints
                .iter()
                .find(|checkpoint| checkpoint.id == *checkpoint_id)
                .map(|checkpoint| vec![StudioHistoryOperation::CommitStudioCheckpoint { checkpoint: checkpoint.clone() }])
                .unwrap_or_default(),
            StudioHistoryOperation::RemoveStudioAlternative { alternative_id } => projection
                .alternatives
                .iter()
                .find(|alternative| alternative.id == *alternative_id)
                .map(|alternative| vec![StudioHistoryOperation::CreateStudioAlternative { alternative: alternative.clone() }])
                .unwrap_or_default(),
            StudioHistoryOperation::SetActiveStudioAlternative { .. } => vec![StudioHistoryOperation::SetActiveStudioAlternative {
                alternative_id: projection.active_alternative_id.clone(),
            }],
        }
    }
}
//#endregion StudioHistoryDocument

//#region StudioHost
/// @emoji 🏛️ Composes many `StudioMember` documents under one studio-wide checkpoint/alternative
/// timeline, itself stored in a dogfooded `"os.studio.history"` meta-document. App-agnostic: this
/// crate has no notion of what a member document *is*, only that it satisfies `StudioMember`.
pub struct StudioHost {
    meta: DocumentStore<StudioHistoryProjection, StudioHistoryOperation>,
    members: HashMap<String, Box<dyn StudioMember>>,
}

impl StudioHost {
    pub fn new(meta_envelope: DocumentEnvelope<StudioHistoryProjection, StudioHistoryOperation>) -> Self {
        Self {
            meta: DocumentStore::new(meta_envelope),
            members: HashMap::new(),
        }
    }

    pub fn register_member(&mut self, member: Box<dyn StudioMember>) {
        self.members.insert(member.document_id().to_string(), member);
    }

    pub fn unregister_member(&mut self, document_id: &str) -> Option<Box<dyn StudioMember>> {
        self.members.remove(document_id)
    }

    pub fn member(&self, document_id: &str) -> Option<&dyn StudioMember> {
        self.members.get(document_id).map(|member| member.as_ref())
    }

    pub fn member_mut<'a>(&'a mut self, document_id: &str) -> Option<&'a mut (dyn StudioMember + 'a)> {
        match self.members.get_mut(document_id) {
            Some(member) => Some(member.as_mut()),
            None => None,
        }
    }

    pub fn meta_projection(&self) -> Result<StudioHistoryProjection, VcsError> {
        self.meta.projection()
    }

    /// @emoji 🔗 Attaches a backbone to the studio-wide meta-document, same runtime-attach/detach
    /// contract as any other `DocumentStore` — default is unattached, this is always an
    /// explicit call.
    pub fn attach_backbone(&mut self, backbone: Box<dyn Backbone>) -> Result<(), VcsError> {
        self.meta.attach_backbone(backbone)
    }

    /// @emoji ✂️ Detaches the meta-document's backbone; the studio history stays in memory.
    pub fn detach_backbone(&mut self) -> Option<Box<dyn Backbone>> {
        self.meta.detach_backbone()
    }

    pub fn backbone_ref(&self) -> Option<&DocumentBackboneRef> {
        self.meta.backbone_ref()
    }

    /// @emoji 📡 Drains inbound backbone messages into the meta-document's edit timeline.
    pub fn tick(&mut self) -> Result<bool, VcsError> {
        self.meta.tick()
    }

    /// @emoji 💾 Commits every dirty member (leaving clean members' existing checkpoints untouched),
    /// pins each member's resulting `(checkpoint, alternative)`, and records one `StudioCheckpoint`
    /// on the meta-document — applied *and* committed there too, so the studio history itself is
    /// durable the moment this returns.
    pub fn commit_studio_checkpoint(&mut self, message: String, authors: Vec<Author>) -> Result<String, VcsError> {
        let mut document_ids: Vec<String> = self.members.keys().cloned().collect();
        document_ids.sort();
        let mut pins = Vec::with_capacity(document_ids.len());
        for document_id in &document_ids {
            let member = self.members.get_mut(document_id).expect("just collected from members");
            if member.is_dirty() {
                member.commit_checkpoint(message.clone(), authors.clone())?;
            }
            let checkpoint_id = member.current_checkpoint_id().ok_or(VcsError::NoCheckpoint)?;
            pins.push(StudioMemberPin {
                document_id: document_id.clone(),
                checkpoint_id,
                alternative_id: member.current_alternative_id().unwrap_or_default(),
            });
        }
        let checkpoint_id = create_document_vcs_id("studio-checkpoint");
        let parent_id = self
            .meta
            .projection()?
            .checkpoints
            .last()
            .map(|checkpoint| checkpoint.id.clone());
        let checkpoint = StudioCheckpoint {
            id: checkpoint_id.clone(),
            parent_id,
            message: message.clone(),
            authors,
            timestamp: HybridLogicalTimestamp::new(0, now_ms()),
            members: pins,
        };
        self.meta.dispatch(DocumentCommand::Apply {
            operations: vec![StudioHistoryOperation::CommitStudioCheckpoint { checkpoint }],
            description: Some(message),
        })?;
        self.meta.dispatch(DocumentCommand::CommitCheckpoint {
            message: None,
            authors: Vec::new(),
        })?;
        Ok(checkpoint_id)
    }

    /// @emoji 🌿 Records a `StudioAlternative` pinned at the current studio checkpoint tip (or none,
    /// if nothing has been committed yet), so it can later be switched back into.
    pub fn create_studio_alternative(&mut self, name: String) -> Result<String, VcsError> {
        let checkpoint_id = self.meta.projection()?.checkpoints.last().map(|checkpoint| checkpoint.id.clone());
        let alternative_id = create_document_vcs_id("studio-alternative");
        let alternative = StudioAlternative {
            id: alternative_id.clone(),
            name,
            checkpoint_ids: checkpoint_id.into_iter().collect(),
        };
        self.meta.dispatch(DocumentCommand::Apply {
            operations: vec![StudioHistoryOperation::CreateStudioAlternative { alternative }],
            description: None,
        })?;
        Ok(alternative_id)
    }

    /// @emoji 🔀 Fans out to every member pinned by `checkpoint_id`'s `StudioCheckpoint`, restoring
    /// each to its exact recorded `(checkpoint, alternative)`.
    pub fn checkout_studio_checkpoint(&mut self, checkpoint_id: &str) -> Result<(), VcsError> {
        let projection = self.meta.projection()?;
        let checkpoint = projection
            .checkpoints
            .iter()
            .find(|checkpoint| checkpoint.id == checkpoint_id)
            .ok_or(VcsError::NoCheckpoint)?;
        for pin in &checkpoint.members {
            if let Some(member) = self.members.get_mut(&pin.document_id) {
                member.checkout(&pin.checkpoint_id, &pin.alternative_id)?;
            }
        }
        Ok(())
    }

    /// @emoji 🔀 Switches the studio's active alternative and fans out to its tip checkpoint's pins.
    pub fn switch_studio_alternative(&mut self, alternative_id: &str) -> Result<(), VcsError> {
        let projection = self.meta.projection()?;
        let alternative = projection
            .alternatives
            .iter()
            .find(|alternative| alternative.id == alternative_id)
            .ok_or_else(|| VcsError::UnknownAlternative(alternative_id.to_string()))?;
        let checkpoint_id = alternative.checkpoint_ids.last().cloned().ok_or(VcsError::NoCheckpoint)?;
        self.meta.dispatch(DocumentCommand::Apply {
            operations: vec![StudioHistoryOperation::SwitchStudioAlternative {
                alternative_id: alternative_id.to_string(),
            }],
            description: None,
        })?;
        self.checkout_studio_checkpoint(&checkpoint_id)
    }

    /// @emoji ↩️ Derived, local-only undo: targets whichever registered member has the most recent
    /// `last_local_edit_timestamp` (by {@link HybridLogicalTimestamp::cmp_key}) and undoes just that
    /// member. Never dispatched against the meta-document — studio-level undo has no `StudioHistoryOperation`
    /// of its own, it is purely a cross-member ordering policy.
    pub fn undo(&mut self) -> Result<(), VcsError> {
        let target = self
            .members
            .iter()
            .filter_map(|(document_id, member)| {
                member.last_local_edit_timestamp().map(|timestamp| (timestamp.cmp_key(), document_id.clone()))
            })
            .max_by_key(|(cmp_key, _)| *cmp_key)
            .map(|(_, document_id)| document_id);
        let document_id = target.ok_or(VcsError::NothingToUndo)?;
        self.members.get_mut(&document_id).ok_or(VcsError::NothingToUndo)?.undo()
    }

    /// @emoji ↪️ Derived, local-only redo: mirrors `undo`, targeting the member with the most
    /// recent `last_undone_local_edit_timestamp`.
    pub fn redo(&mut self) -> Result<(), VcsError> {
        let target = self
            .members
            .iter()
            .filter_map(|(document_id, member)| {
                member
                    .last_undone_local_edit_timestamp()
                    .map(|timestamp| (timestamp.cmp_key(), document_id.clone()))
            })
            .max_by_key(|(cmp_key, _)| *cmp_key)
            .map(|(_, document_id)| document_id);
        let document_id = target.ok_or(VcsError::NothingToRedo)?;
        self.members.get_mut(&document_id).ok_or(VcsError::NothingToRedo)?.redo()
    }
}
//#endregion StudioHost
//#endregion 🔖Studio

//#region 🔖TestSupport
/// @emoji 🧪 Round-trip assertions shared by every technology crate's `Operation` test suite.
pub mod test_support {
    use super::*;

    /// @emoji 🔁 Asserts that applying `operation` then applying its reversed `backwards(pre)` restores `pre`.
    pub fn assert_operation_round_trip<P, Operation>(pre: &P, operation: Operation)
    where
        P: Clone + PartialEq + std::fmt::Debug,
        Operation: crate::Operation<P>,
    {
        let post = apply_operation(pre, &operation);
        let mut backwards = operation.backwards(pre);
        backwards.reverse();
        let restored = backwards
            .iter()
            .fold(post, |projection, back_operation| apply_operation(&projection, back_operation));
        assert_eq!(&restored, pre, "operation backwards did not restore pre-state");
    }

    /// @emoji 🗄️ Asserts a full store round trip: Apply→Undo restores `initial`, Redo restores the
    /// post-apply projection, and replay-materialization agrees with the live store projection.
    pub fn assert_store_roundtrip<P, Operation>(initial: P, operation: Operation)
    where
        P: Clone + Serialize + DeserializeOwned + PartialEq + std::fmt::Debug,
        Operation: Clone + Serialize + DeserializeOwned + crate::Operation<P>,
    {
        let envelope = create_document_envelope("test/v1", "test", initial.clone(), None);
        let mut store = DocumentStore::new(envelope);
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![operation],
                description: None,
            })
            .expect("apply");
        let post = store.projection().expect("post projection");
        store.dispatch(DocumentCommand::Undo).expect("undo");
        assert_eq!(
            store.projection().expect("undo projection"),
            initial,
            "undo did not restore initial projection"
        );
        store.dispatch(DocumentCommand::Redo).expect("redo");
        assert_eq!(
            store.projection().expect("redo projection"),
            post,
            "redo did not restore post projection"
        );
        let replayed = materialize_document_projection(store.envelope(), store.applied_edit_ids()).expect("replay");
        assert_eq!(replayed, post, "materialization from replay diverged from store projection");
    }

    /// @emoji 📜 Asserts a DSL round trip: `P::parse_dsl(&projection.print_dsl())` recovers an equal
    /// projection. The compile-time validation ground truth for every technology's `🔖Dsl` region —
    /// call this from a `#[test]` over every `include_str!` fixture.
    pub fn assert_dsl_round_trip<P>(projection: &P)
    where
        P: DocumentDsl + PartialEq + std::fmt::Debug,
    {
        let printed = projection.print_dsl();
        let parsed = P::parse_dsl(&printed).unwrap_or_else(|error| panic!("dsl parse failed: {error}"));
        assert_eq!(&parsed, projection, "dsl round trip diverged;\nprinted:\n{printed}");
    }

    /// @emoji 📦 Asserts a pack round trip: `P::decode_pack(&projection.encode_pack())` recovers an
    /// equal projection — the pack sibling of `assert_dsl_round_trip`.
    pub fn assert_pack_round_trip<P>(projection: &P)
    where
        P: DocumentPack + PartialEq + std::fmt::Debug,
    {
        let bytes = projection.encode_pack();
        let decoded = P::decode_pack(&bytes).unwrap_or_else(|error| panic!("pack decode failed: {error}"));
        assert_eq!(&decoded, projection, "pack round trip diverged");
    }

    /// @emoji ⚖️ Asserts dsl and pack are two projections of the SAME value: `decode_pack(
    /// encode_pack(p)) == parse_dsl(print_dsl(p)) == p` — the compile-time validation ground truth
    /// for the whole pack rollout's central LAW (see `DocumentPack`'s doc comment).
    pub fn assert_dsl_pack_equivalence<P>(projection: &P)
    where
        P: DocumentDsl + DocumentPack + Clone + PartialEq + std::fmt::Debug,
    {
        let via_pack = P::decode_pack(&projection.encode_pack()).unwrap_or_else(|error| panic!("pack decode failed: {error}"));
        let via_dsl = P::parse_dsl(&projection.print_dsl()).unwrap_or_else(|error| panic!("dsl parse failed: {error}"));
        assert_eq!(&via_pack, projection, "pack round trip diverged from source projection");
        assert_eq!(&via_dsl, projection, "dsl round trip diverged from source projection");
        assert_eq!(via_pack, via_dsl, "pack and dsl round trips diverged from each other");
    }

    /// @emoji ⚡ Asserts an op-text round trip for a single operation: `print_op` contains no newline
    /// and `Op::parse_op` recovers an equal operation from it. The compile-time validation ground
    /// truth for every technology's `🔖OpText` region — call this once per `Operation` variant.
    pub fn assert_op_line_round_trip<Op>(operation: &Op)
    where
        Op: OpText + PartialEq + std::fmt::Debug,
    {
        let printed = operation.print_op();
        assert!(!printed.contains('\n'), "print_op must be one line, got: {printed:?}");
        let parsed = Op::parse_op(&printed).unwrap_or_else(|error| panic!("op parse failed: {error}"));
        assert_eq!(&parsed, operation, "op-text round trip diverged; printed: {printed:?}");
    }

    /// @emoji 📄 Asserts that printing a store's envelope to text and parsing it back yields the same
    /// live projection the store already holds — the ground truth for {@link print_document_text}/
    /// {@link parse_document_text} on any technology once it implements `DocumentDsl` + `OpText`.
    pub fn assert_document_text_round_trip<P, Operation>(store: &DocumentStore<P, Operation>)
    where
        P: Clone + DocumentDsl + PartialEq + std::fmt::Debug + Serialize + DeserializeOwned,
        Operation: Clone + OpText + crate::Operation<P> + PartialEq + Serialize + DeserializeOwned,
    {
        let live = store.projection().expect("store projection");
        let files = print_document_text(store.envelope()).expect("print document text");
        let parsed: ParsedDocumentText<P, Operation> =
            parse_document_text(&files.dsl, &files.ops).unwrap_or_else(|error| panic!("parse document text failed: {error}"));
        assert_eq!(parsed.projection, live, "document-text round trip diverged from store projection");
    }

    /// @emoji 🗄️ Asserts a full pack-based document round trip: mirrors
    /// `assert_document_text_round_trip` but via `print_document_pack`/`parse_document_pack`, and
    /// additionally asserts the pack path's parsed projection agrees with the text path's — the two
    /// storage formats must never diverge on the same store.
    pub fn assert_document_pack_round_trip<P, Operation>(store: &DocumentStore<P, Operation>)
    where
        P: Clone + DocumentDsl + DocumentPack + PartialEq + std::fmt::Debug + Serialize + DeserializeOwned,
        Operation: Clone + OpText + crate::Operation<P> + PartialEq + Serialize + DeserializeOwned,
    {
        let live = store.projection().expect("store projection");
        let pack_files = print_document_pack(store.envelope()).expect("print document pack");
        let parsed_pack: ParsedDocumentText<P, Operation> =
            parse_document_pack(&pack_files.pack, &pack_files.ops).unwrap_or_else(|error| panic!("parse document pack failed: {error}"));
        assert_eq!(parsed_pack.projection, live, "document-pack round trip diverged from store projection");

        let text_files = print_document_text(store.envelope()).expect("print document text");
        let parsed_text: ParsedDocumentText<P, Operation> =
            parse_document_text(&text_files.dsl, &text_files.ops).unwrap_or_else(|error| panic!("parse document text failed: {error}"));
        assert_eq!(parsed_pack.projection, parsed_text.projection, "document-pack path diverged from document-text path");
    }

    /// @emoji ✉️ Asserts that converting an `Edit<Operation>` into `protocol::OperationEnvelope`s
    /// (`protocol_causal`'s canonical wire/causal representation, moved from `framework/core` in CW3,
    /// via `protocol::operation_envelope_from_edit`) preserves every operation's essential facts —
    /// the causal-wire sibling of `assert_pack_round_trip`/`assert_dsl_round_trip` for the app
    /// fan-out's "pack laws" cluster.
    ///
    /// `OperationEnvelope` is a runtime struct that is never itself re-serialized back into an
    /// `Edit` (unlike `encode_pack`/`decode_pack`, there is no `envelope_to_edit` inverse — vcs's OWN
    /// `edit_from_operation_envelope` recovers a *whole edit* from vcs's own, differently-shaped,
    /// per-edit `semio_framework_core::OperationEnvelope`, not from this per-operation
    /// `protocol_causal` one), so a byte-level encode-then-decode law is not meaningful here.
    /// Instead this checks the two LAWS that actually matter for this bridge: (1) whatever
    /// `edit.operation_meta` explicitly recorded for a slot (the ground-truth source
    /// `operation_envelope_from_edit` prefers over its own `Operation`-trait/structural fallbacks —
    /// see that function's own doc comment) survives unchanged onto the envelope's
    /// `operation_id`/`dependencies`/`actor`/`timestamp`; and (2) `envelope.diff.payload`/
    /// `envelope.inverse.inverse_diff` decode back (via `Operation`'s own `Deserialize` impl) into
    /// operations equal to `edit.forwards[i]`/`edit.backwards[i]` — the part a hand-rolled
    /// `Serialize`/`Deserialize` pair can silently break. Deliberately does NOT recompute the
    /// envelope's fallback chain (id/actor/deps when `operation_meta` is absent) itself, since doing
    /// so would just re-run `operation_envelope_from_edit`'s own logic against itself and always
    /// agree — see this function's `🧪Tests` sibling for a deliberately lossy `Operation` impl that
    /// trips law (2).
    pub fn assert_command_envelope_round_trip<P, Operation>(edit: &Edit<Operation>, document_id: &DocumentId)
    where
        P: Clone + PartialEq + std::fmt::Debug,
        Operation: crate::Operation<P> + PartialEq + std::fmt::Debug,
    {
        let envelopes = protocol::operation_envelope_from_edit::<P, Operation>(edit, document_id);
        assert_eq!(envelopes.len(), edit.forwards.len(), "one envelope must be produced per forward operation");
        for (index, envelope) in envelopes.iter().enumerate() {
            assert_eq!(envelope.document_id, *document_id, "document id did not survive the envelope conversion");
            if let Some(meta) = edit.operation_meta.get(index) {
                if let Some(operation_id) = &meta.operation_id {
                    assert_eq!(&envelope.operation_id, operation_id, "explicit operation id did not survive the envelope conversion");
                }
                assert_eq!(envelope.dependencies, meta.dependencies, "explicit dependencies did not survive the envelope conversion");
                if let Some(author_id) = &meta.author_id {
                    assert_eq!(&envelope.actor, author_id, "explicit author id did not survive the envelope conversion");
                }
                assert_eq!(envelope.timestamp, meta.timestamp, "explicit timestamp did not survive the envelope conversion");
            }
            let recovered_forward: Operation = serde_json::from_value(envelope.diff.payload.clone())
                .unwrap_or_else(|error| panic!("envelope diff payload must decode back into an equal operation: {error}"));
            assert_eq!(&recovered_forward, &edit.forwards[index], "envelope diff payload did not decode back into an equal forward operation");
            match edit.backwards.get(index) {
                Some(backward) => {
                    let recovered_backward: Operation = serde_json::from_value(envelope.inverse.inverse_diff.clone())
                        .unwrap_or_else(|error| panic!("envelope inverse payload must decode back into an equal operation: {error}"));
                    assert_eq!(&recovered_backward, backward, "envelope inverse payload did not decode back into an equal backward operation");
                }
                None => assert_eq!(envelope.inverse.inverse_diff, serde_json::Value::Null, "inverse payload must be Null when the edit has no corresponding backwards op"),
            }
        }
    }

    /// @emoji 🩺 Asserts the store's incrementally-maintained live projection agrees with a
    /// from-scratch full replay — the differential check for `DocumentStore`'s stateful `current`
    /// field. Call after arbitrary command sequences (apply/amend/undo/redo/checkpoint/switch
    /// interleavings) in a tech's own tests to confirm the incremental fast paths never diverge from
    /// the replay ground truth.
    pub fn assert_live_equals_replay<P, Operation>(store: &DocumentStore<P, Operation>)
    where
        P: Clone + PartialEq + std::fmt::Debug + Serialize + DeserializeOwned,
        Operation: Clone + Serialize + DeserializeOwned + crate::Operation<P>,
    {
        let live = store.projection().expect("store projection");
        let replayed = materialize_document_projection(store.envelope(), store.applied_edit_ids()).expect("replay");
        assert_eq!(live, replayed, "store's live projection diverged from full-replay materialization");
    }
}
//#endregion 🔖TestSupport

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
    #[dsl(extension = "demo")]
    struct DemoProjection {
        n: i32,
    }

    // `impl store::DocumentPack for DemoProjection` is now generated automatically by
    // `#[derive(dsl::DslDocument)]` above (see dsl/derive/rs/lib.rs's `🔖DslDocument` region) —
    // same seam as its `impl store::DocumentDsl for DemoProjection` sibling.

    #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
    struct DemoDiff {
        n: Option<i32>,
    }

    impl OperationDiff<DemoProjection> for DemoDiff {
        fn apply(&self, projection: &DemoProjection) -> DemoProjection {
            DemoProjection {
                n: self.n.unwrap_or(projection.n),
            }
        }

        fn absorb(&mut self, other: Self) {
            if other.n.is_some() {
                self.n = other.n;
            }
        }
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
    #[serde(tag = "operation")]
    enum DemoOperation {
        #[dsl(key = "set-n")]
        SetN { n: i32 },
    }

    impl Operation<DemoProjection> for DemoOperation {
        type Diff = DemoDiff;

        fn diff(&self, _projection: &DemoProjection) -> DemoDiff {
            match self {
                DemoOperation::SetN { n } => DemoDiff { n: Some(*n) },
            }
        }

        fn backwards(&self, projection: &DemoProjection) -> Vec<Self> {
            vec![DemoOperation::SetN { n: projection.n }]
        }
    }

    /// @emoji 🛰️ Builds a foreign {@link OperationEnvelope} (as if authored by `actor` on another peer) by

    /// applying `operation` in a throwaway peer store and stamping the envelope's actor id.
    fn foreign_operation_envelope(actor: &str, operation: DemoOperation) -> OperationEnvelope {
        let mut peer = DocumentStore::new(create_document_envelope::<DemoProjection, DemoOperation>(
            "demo/v1",
            "demo",
            DemoProjection { n: 0 },
            None,
        ));
        peer.dispatch(DocumentCommand::Apply {
            operations: vec![operation],
            description: None,
        })
        .expect("peer apply");
        let edit = peer.envelope().vcs.edits.last().expect("peer edit").clone();
        let mut envelope = operation_envelope_from_edit(peer.envelope(), &edit, Vec::new()).expect("operation envelope");
        envelope.actor = ActorId(actor.to_string());
        envelope
    }

    #[test]
    fn materialize_replays_forward_operations() {
        let envelope = create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 1 }],
                description: None,
            })
            .expect("apply");
        assert_eq!(store.projection().expect("projection").n, 1);
        assert_eq!(store.envelope().vcs.edits.len(), 1);
    }

    #[test]
    fn undo_redo_round_trip() {
        let envelope = create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 1 }],
                description: None,
            })
            .expect("apply");
        store.dispatch(DocumentCommand::Undo).expect("undo");
        assert_eq!(store.projection().expect("projection").n, 0);
        store.dispatch(DocumentCommand::Redo).expect("redo");
        assert_eq!(store.projection().expect("projection").n, 1);
    }

    #[test]
    fn apply_computes_backwards_from_pre_state() {
        let envelope = create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 5 }],
                description: None,
            })
            .expect("apply");
        let edit = &store.envelope().vcs.edits[0];
        assert_eq!(edit.backwards, vec![DemoOperation::SetN { n: 0 }]);
    }

    #[test]
    fn commit_checkpoint_wraps_edits_into_change() {
        let envelope = create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 1 }],
                description: None,
            })
            .expect("apply");
        store
            .dispatch(DocumentCommand::CommitCheckpoint {
                message: Some("init".into()),
                authors: vec![Author {
                    id: "a1".into(),
                    name: "Alice".into(),
                    avatar: None,
                }],
            })
            .expect("commit");
        assert_eq!(store.envelope().vcs.changes.len(), 1);
        assert_eq!(store.envelope().vcs.checkpoints.len(), 1);
        assert_eq!(store.envelope().vcs.checkpoints[0].message, Some("init".into()));
    }

    #[test]
    fn checkout_checkpoint_restores_applied_edits() {
        let envelope = create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 1 }],
                description: None,
            })
            .expect("apply");
        store
            .dispatch(DocumentCommand::CommitCheckpoint {
                message: Some("c1".into()),
                authors: Vec::new(),
            })
            .expect("commit");
        let checkpoint_id = store.envelope().vcs.checkpoints[0].id.clone();
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 9 }],
                description: None,
            })
            .expect("apply2");
        assert_eq!(store.projection().expect("projection").n, 9);
        store
            .dispatch(DocumentCommand::CheckoutCheckpoint {
                checkpoint_id,
            })
            .expect("checkout");
        assert_eq!(store.projection().expect("projection").n, 1);
    }

    #[test]
    fn alternatives_switch_restores_checkpoint_chain() {
        let envelope = create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 1 }],
                description: None,
            })
            .expect("apply");
        store
            .dispatch(DocumentCommand::CreateAlternative {
                name: "branch-a".into(),
            })
            .expect("create alternative");
        let alt_id = store.envelope().vcs.alternatives[0].id.clone();
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 2 }],
                description: None,
            })
            .expect("apply on branch");
        store
            .dispatch(DocumentCommand::SwitchAlternative {
                alternative_id: alt_id,
            })
            .expect("switch");
        assert_eq!(store.projection().expect("projection").n, 1);
    }

    #[test]
    fn checkout_old_checkpoint_then_commit_creates_a_fork() {
        let envelope = create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 1 }],
                description: None,
            })
            .expect("apply");
        store
            .dispatch(DocumentCommand::CommitCheckpoint {
                message: Some("c1".into()),
                authors: Vec::new(),
            })
            .expect("commit c1");
        let c1 = store.envelope().vcs.checkpoints[0].id.clone();
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 2 }],
                description: None,
            })
            .expect("apply");
        store
            .dispatch(DocumentCommand::CommitCheckpoint {
                message: Some("c2".into()),
                authors: Vec::new(),
            })
            .expect("commit c2");
        store
            .dispatch(DocumentCommand::CheckoutCheckpoint { checkpoint_id: c1.clone() })
            .expect("checkout c1");
        assert_eq!(store.current_checkpoint_id(), Some(c1.as_str()));
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 9 }],
                description: None,
            })
            .expect("apply");
        store
            .dispatch(DocumentCommand::CommitCheckpoint {
                message: Some("fork".into()),
                authors: Vec::new(),
            })
            .expect("commit fork");
        let children: Vec<&Checkpoint> = store
            .envelope()
            .vcs
            .checkpoints
            .iter()
            .filter(|checkpoint| checkpoint.parent_id.as_deref() == Some(c1.as_str()))
            .collect();
        assert_eq!(children.len(), 2, "checking out an old checkpoint before committing must fork, not extend the trunk");
    }

    #[test]
    fn create_alternative_appends_commits_to_its_own_checkpoint_chain() {
        let envelope = create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 1 }],
                description: None,
            })
            .expect("apply");
        store
            .dispatch(DocumentCommand::CommitCheckpoint {
                message: Some("root".into()),
                authors: Vec::new(),
            })
            .expect("commit root");
        store
            .dispatch(DocumentCommand::CreateAlternative { name: "feature-a".into() })
            .expect("create alternative");
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 2 }],
                description: None,
            })
            .expect("apply");
        store
            .dispatch(DocumentCommand::CommitCheckpoint {
                message: Some("branch commit".into()),
                authors: Vec::new(),
            })
            .expect("commit on branch");
        assert_eq!(store.envelope().vcs.alternatives[0].checkpoint_ids.len(), 2);
        assert_eq!(store.envelope().vcs.checkpoints.len(), 2);
    }

    #[test]
    fn history_columns_orders_newest_first_and_labels_trunk_root() {
        let envelope = create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 1 }],
                description: None,
            })
            .expect("apply");
        store
            .dispatch(DocumentCommand::CommitCheckpoint {
                message: Some("c1".into()),
                authors: Vec::new(),
            })
            .expect("commit c1");
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 2 }],
                description: None,
            })
            .expect("apply");
        store
            .dispatch(DocumentCommand::CommitCheckpoint {
                message: Some("c2".into()),
                authors: Vec::new(),
            })
            .expect("commit c2");
        let columns = store.history_columns();
        assert_eq!(columns.len(), 2);
        assert_eq!(columns[0].description, Some("c2".into()), "newest checkpoint must be first");
        assert_eq!(columns[0].lane, 0);
        assert_eq!(columns[0].labels, vec!["main".to_string()], "newest unlabeled row falls back to main");
        assert!(columns[1].labels.is_empty(), "only the newest row gets the main fallback");
        let json = serde_json::to_string(&columns[0]).expect("serialize");
        assert!(json.contains("checkpointId"), "wire format must be camelCase: {json}");
    }

    #[test]
    fn history_columns_assigns_distinct_lanes_and_pulls_main_only_descendants_to_trunk() {
        let envelope = create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 1 }],
                description: None,
            })
            .expect("apply");
        store
            .dispatch(DocumentCommand::CommitCheckpoint {
                message: Some("root".into()),
                authors: Vec::new(),
            })
            .expect("commit root");
        let root = store.envelope().vcs.checkpoints[0].id.clone();

        store
            .dispatch(DocumentCommand::CreateAlternative { name: "feature-a".into() })
            .expect("create feature-a");
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 2 }],
                description: None,
            })
            .expect("apply");
        store
            .dispatch(DocumentCommand::CommitCheckpoint {
                message: Some("a1".into()),
                authors: Vec::new(),
            })
            .expect("commit a1");

        store
            .dispatch(DocumentCommand::CheckoutCheckpoint { checkpoint_id: root.clone() })
            .expect("checkout root");
        store
            .dispatch(DocumentCommand::CreateAlternative { name: "feature-b".into() })
            .expect("create feature-b");
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 3 }],
                description: None,
            })
            .expect("apply");
        store
            .dispatch(DocumentCommand::CommitCheckpoint {
                message: Some("b1".into()),
                authors: Vec::new(),
            })
            .expect("commit b1");

        store
            .dispatch(DocumentCommand::CheckoutCheckpoint { checkpoint_id: root.clone() })
            .expect("checkout root again");
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 4 }],
                description: None,
            })
            .expect("apply");
        store
            .dispatch(DocumentCommand::CommitCheckpoint {
                message: Some("main resumed".into()),
                authors: Vec::new(),
            })
            .expect("commit main resumed");

        let columns = store.history_columns();
        assert_eq!(columns.len(), 4, "root + a1 + b1 + main-resumed");
        let by_message: HashMap<String, &HistoryColumn> = columns
            .iter()
            .filter_map(|column| column.description.clone().map(|description| (description, column)))
            .collect();
        assert_eq!(by_message["root"].lane, 0, "root has no parent, lane 0");
        assert_eq!(by_message["main resumed"].lane, 0, "commit with no alternative stays on the trunk");
        let a_lane = by_message["a1"].lane;
        let b_lane = by_message["b1"].lane;
        assert_ne!(a_lane, 0, "a1 belongs to an alternative, not the trunk");
        assert_ne!(b_lane, 0, "b1 belongs to an alternative, not the trunk");
        assert_ne!(a_lane, b_lane, "distinct alternatives must get distinct swimlanes");

        let root_children: Vec<&HistoryColumn> = columns
            .iter()
            .filter(|column| column.parent_checkpoint_id.as_deref() == Some(root.as_str()))
            .collect();
        assert_eq!(root_children.len(), 3, "root forked three ways: a1, b1, main-resumed");
    }

    #[test]
    fn no_backbone_by_default() {
        let envelope: DocumentEnvelope<DemoProjection, DemoOperation> =
            create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        assert!(envelope.backbone.is_none(), "a fresh document has no attached backbone");
        let store = DocumentStore::new(envelope);
        assert!(store.backbone_ref().is_none());
    }

    #[test]
    fn memory_backbone_pair_propagates_edits_bidirectionally() {
        let (backbone_a, backbone_b) = MemoryBackbone::pair("peer-a", "peer-b");
        let envelope_a: DocumentEnvelope<DemoProjection, DemoOperation> =
            create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let envelope_b: DocumentEnvelope<DemoProjection, DemoOperation> =
            create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store_a = DocumentStore::new(envelope_a);
        let mut store_b = DocumentStore::new(envelope_b);
        store_a.attach_backbone(Box::new(backbone_a)).expect("attach a");
        store_b.attach_backbone(Box::new(backbone_b)).expect("attach b");

        store_a
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 1 }],
                description: None,
            })
            .expect("apply on a");
        store_b.tick().expect("tick b");
        assert_eq!(store_b.projection().expect("projection b").n, 1, "b receives a's edit");

        store_b
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 2 }],
                description: None,
            })
            .expect("apply on b");
        store_a.tick().expect("tick a");
        assert_eq!(store_a.projection().expect("projection a").n, 2, "a receives b's edit");
    }

    #[test]
    fn detach_backbone_stops_synchronizing_but_keeps_the_wip_graph() {
        let (backbone_a, backbone_b) = MemoryBackbone::pair("peer-a", "peer-b");
        let envelope: DocumentEnvelope<DemoProjection, DemoOperation> =
            create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store_a = DocumentStore::new(envelope.clone());
        let mut store_b = DocumentStore::new(envelope);
        store_a.attach_backbone(Box::new(backbone_a)).expect("attach a");
        store_b.attach_backbone(Box::new(backbone_b)).expect("attach b");
        store_a.detach_backbone();
        assert!(store_a.backbone_ref().is_none());

        store_a
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 9 }],
                description: None,
            })
            .expect("apply after detach still works on the in-memory graph");
        assert_eq!(store_a.projection().expect("projection a").n, 9);
        store_b.tick().expect("tick b");
        assert_eq!(store_b.projection().expect("projection b").n, 0, "detached edits never reach the peer");
    }

    #[test]
    fn deserialized_envelope_with_stale_backbone_ref_never_auto_attaches() {
        let envelope: DocumentEnvelope<DemoProjection, DemoOperation> =
            create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut stale_json: serde_json::Value =
            serde_json::to_value(&envelope).expect("serialize envelope");
        stale_json["backbone"] = serde_json::json!({ "uri": "folder:///nonexistent/path" });
        let stale_envelope: DocumentEnvelope<DemoProjection, DemoOperation> =
            serde_json::from_value(stale_json).expect("deserialize envelope with stale backbone ref");

        let mut store = DocumentStore::new(stale_envelope.clone());
        assert!(
            store.tick().expect("tick with no live backbone is a no-operation") == false,
            "no backbone was ever attached, so there is nothing to pump"
        );
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 1 }],
                description: None,
            })
            .expect("apply works purely against the in-memory graph");
        assert_eq!(store.projection().expect("projection").n, 1);

        store.set_state(stale_envelope, Vec::new(), Vec::new());
        assert!(
            store.tick().expect("tick after set_state with no live backbone is a no-operation") == false,
            "set_state must not resurrect IO from a stale backbone descriptor either"
        );
    }





    #[test]
    fn document_codec_of_round_trips_pack_and_dsl() {
        let codec = DocumentCodec::of::<DemoProjection, DemoOperation>("demo/v1");
        assert_eq!(codec.schema, "demo/v1");
        assert_eq!(codec.extension, "demo");

        let envelope: DocumentEnvelope<DemoProjection, DemoOperation> =
            create_document_envelope("demo/v1", "demo", DemoProjection { n: 4 }, None);
        let envelope_json = serde_json::to_string(&envelope).expect("envelope json");

        let (pack_files, dsl_mirror) = (codec.print)(&envelope_json).expect("codec print");
        assert_eq!(dsl_mirror, DemoProjection { n: 4 }.print_dsl(), "dsl mirror matches the initial projection's print_dsl");

        let parsed_json = (codec.parse)(&pack_files.pack, &pack_files.ops).expect("codec parse");
        let parsed: DocumentEnvelope<DemoProjection, DemoOperation> = serde_json::from_str(&parsed_json).expect("parse envelope json");
        assert_eq!(parsed.vcs.initial_projection.n, 4, "codec.parse round trips through pack bytes");

        let parsed_dsl_json = (codec.parse_dsl)(&dsl_mirror, &pack_files.ops).expect("codec parse_dsl");
        let parsed_dsl: DocumentEnvelope<DemoProjection, DemoOperation> = serde_json::from_str(&parsed_dsl_json).expect("parse envelope json (dsl path)");
        assert_eq!(
            parsed.vcs.initial_projection, parsed_dsl.vcs.initial_projection,
            "codec.parse and codec.parse_dsl agree on the same document"
        );

        register_document_codec(codec);
        assert!(document_codec("demo/v1").is_some(), "registered codec is discoverable by schema string");
        assert!(document_codec("no-such-schema").is_none());
    }


    #[test]
    fn attach_reconciles_a_pushed_snapshot() {
        let (channel, remote) = ChannelBackbone::pair("chan");
        let seeded: DocumentEnvelope<DemoProjection, DemoOperation> =
            create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut seed_store = DocumentStore::new(seeded);
        seed_store
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 5 }],
                description: None,
            })
            .expect("apply");
        remote
            .push(BackboneMessage::Snapshot {
                envelope_json: seed_store.envelope_json().expect("seed json"),
            })
            .expect("push snapshot");

        let fresh: DocumentEnvelope<DemoProjection, DemoOperation> =
            create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(fresh);
        store.attach_backbone(Box::new(channel)).expect("attach reconciles the pushed snapshot");
        assert_eq!(store.projection().expect("projection").n, 5, "adopted the pushed snapshot's edit");
    }

    #[test]
    fn channel_backbone_round_trips_between_store_and_actor() {
        let (channel, remote) = ChannelBackbone::pair("chan");
        let envelope: DocumentEnvelope<DemoProjection, DemoOperation> =
            create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);
        store.attach_backbone(Box::new(channel)).expect("attach");
        let attach_flush = remote.drain().expect("drain attach");
        assert!(
            attach_flush.iter().any(|message| matches!(message, BackboneMessage::Snapshot { .. })),
            "attach flushes a snapshot to the actor end: {attach_flush:?}"
        );

        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 4 }],
                description: None,
            })
            .expect("apply");
        let outbound = remote.drain().expect("drain apply");
        assert!(
            outbound.iter().any(|message| matches!(message, BackboneMessage::Operations { .. })),
            "a local apply is sent outbound as operations: {outbound:?}"
        );

        remote
            .push(BackboneMessage::Operations {
                envelopes: vec![foreign_operation_envelope("peer", DemoOperation::SetN { n: 8 })],
            })
            .expect("push inbound operations");
        store.tick().expect("tick");
        assert_eq!(store.projection().expect("projection").n, 8, "store ingests the actor's inbound operations");
    }

    #[test]
    fn pump_acks_ingested_operations() {
        let (channel, remote) = ChannelBackbone::pair("chan");
        let envelope: DocumentEnvelope<DemoProjection, DemoOperation> =
            create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);
        store.attach_backbone(Box::new(channel)).expect("attach");
        let _ = remote.drain().expect("drain attach snapshot");

        let inbound = foreign_operation_envelope("peer", DemoOperation::SetN { n: 7 });
        let operation_id = inbound.id.0.clone();
        remote
            .push(BackboneMessage::Operations { envelopes: vec![inbound] })
            .expect("push inbound operations");
        store.tick().expect("tick");
        assert_eq!(store.projection().expect("projection").n, 7, "ingested the inbound operation");

        let outbound = remote.drain().expect("drain ack");
        assert!(
            outbound
                .iter()
                .any(|message| matches!(message, BackboneMessage::Ack { op_ids } if op_ids == &vec![operation_id.clone()])),
            "successful operations ingest emits an Ack for the ingested operation ids: {outbound:?}"
        );
    }

    #[test]
    fn exact_base_only_undo_refuses_a_foreign_tail() {
        let envelope: DocumentEnvelope<DemoProjection, DemoOperation> =
            create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 1 }],
                description: None,
            })
            .expect("local apply");
        store
            .ingest_remote(foreign_operation_envelope("peer", DemoOperation::SetN { n: 2 }))
            .expect("ingest foreign");
        assert_eq!(store.projection().expect("projection").n, 2, "foreign edit sits at the tail");

        let error = store
            .dispatch(DocumentCommand::UndoWithPolicy {
                policy: UndoPolicy::ExactBaseOnly,
                semantic_command: None,
            })
            .expect_err("undo must refuse a foreign tail");
        assert!(matches!(error, VcsError::ForeignEdit(_)), "got {error:?}");
        assert_eq!(store.projection().expect("projection").n, 2, "the timeline is untouched after refusal");
    }

    #[test]
    fn transform_against_concurrent_undo_skips_over_a_foreign_tail() {
        let envelope: DocumentEnvelope<DemoProjection, DemoOperation> =
            create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 1 }],
                description: None,
            })
            .expect("local apply");
        let local_edit_id = store.applied_edit_ids()[0].clone();
        let foreign = foreign_operation_envelope("peer", DemoOperation::SetN { n: 2 });
        let foreign_id = foreign.id.0.clone();
        store.ingest_remote(foreign).expect("ingest foreign");
        assert_eq!(store.applied_edit_ids().len(), 2, "local + foreign are both applied");

        store
            .dispatch(DocumentCommand::UndoWithPolicy {
                policy: UndoPolicy::TransformAgainstConcurrent,
                semantic_command: None,
            })
            .expect("transform undo removes the local edit from mid-timeline");
        assert_eq!(
            store.applied_edit_ids(),
            std::slice::from_ref(&foreign_id),
            "only the local edit is removed; the concurrent foreign edit stays applied"
        );
        assert_eq!(
            store.redo_edit_ids(),
            std::slice::from_ref(&local_edit_id),
            "the local edit is on the redo stack"
        );
        assert_eq!(store.projection().expect("projection").n, 2, "projection re-materializes from the foreign edit alone");

        store.dispatch(DocumentCommand::Redo).expect("redo brings the local edit back");
        assert_eq!(store.applied_edit_ids().len(), 2);
        assert_eq!(store.projection().expect("projection").n, 1, "redo re-applies the local edit at the tail");
    }

    #[test]
    fn compensating_undo_dispatches_semantic_command() {
        let envelope: DocumentEnvelope<DemoProjection, DemoOperation> =
            create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 5 }],
                description: None,
            })
            .expect("apply");
        let undo_apply = serde_json::to_string(&DocumentCommand::Apply {
            operations: vec![DemoOperation::SetN { n: 0 }],
            description: Some("compensate".into()),
        })
        .expect("serialize undo apply");
        store
            .dispatch(DocumentCommand::UndoWithPolicy {
                policy: UndoPolicy::CompensatingAction,
                semantic_command: Some(undo_apply),
            })
            .expect("compensating undo");
        assert_eq!(store.projection().expect("projection").n, 0);
    }

    #[test]
    fn edit_operations_exposes_the_latest_edit() {
        let envelope: DocumentEnvelope<DemoProjection, DemoOperation> =
            create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);
        assert!(store.edit_operations().is_none(), "no edits yet");
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 5 }],
                description: None,
            })
            .expect("apply");
        let (forwards, backwards, meta) = store.edit_operations().expect("edit operations");
        assert_eq!(forwards, &[DemoOperation::SetN { n: 5 }]);
        assert_eq!(backwards, &[DemoOperation::SetN { n: 0 }], "backwards restores the pre-state");
        assert_eq!(meta.len(), 1);
    }

    #[test]
    fn amend_last_absorbs_into_matching_coalesce_key() {
        let envelope: DocumentEnvelope<DemoProjection, DemoOperation> =
            create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);
        store
            .dispatch(DocumentCommand::AmendLast {
                operations: vec![DemoOperation::SetN { n: 1 }],
                coalesce_key: Some("drag".into()),
            })
            .expect("first amend");
        store
            .dispatch(DocumentCommand::AmendLast {
                operations: vec![DemoOperation::SetN { n: 2 }],
                coalesce_key: Some("drag".into()),
            })
            .expect("second amend");
        assert_eq!(store.envelope().vcs.edits.len(), 1, "coalesced into a single edit");
        assert_eq!(store.projection().expect("projection").n, 2);
        store.dispatch(DocumentCommand::Undo).expect("undo");
        assert_eq!(
            store.projection().expect("projection after undo").n,
            0,
            "undo restores pre-gesture state in one step"
        );
    }

    #[test]
    fn amend_last_incremental_path_matches_full_replay_over_many_amends() {
        // 🪢 Regression guard for the incremental `AmendLast` path (see `AmendCache`): many sequential
        // amends into the same coalesced edit — e.g. a long slider drag — must still produce exactly the
        // same edit (forwards/backwards/operation_meta length, final projection, one-step undo) as the
        // previous full-replay-every-time implementation, just without re-replaying history each time.
        let envelope: DocumentEnvelope<DemoProjection, DemoOperation> =
            create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);
        for n in 1..=50 {
            store
                .dispatch(DocumentCommand::AmendLast {
                    operations: vec![DemoOperation::SetN { n }],
                    coalesce_key: Some("drag".into()),
                })
                .expect("amend");
        }
        assert_eq!(store.envelope().vcs.edits.len(), 1, "still a single coalesced edit");
        let edit = store.envelope().vcs.edits.last().expect("edit");
        assert_eq!(edit.forwards.len(), 50);
        assert_eq!(edit.backwards.len(), 50);
        assert_eq!(edit.operation_meta.len(), 50);
        assert_eq!(store.projection().expect("projection").n, 50);
        store.dispatch(DocumentCommand::Undo).expect("undo");
        assert_eq!(
            store.projection().expect("projection after undo").n,
            0,
            "one undo reverts the whole 50-step coalesced gesture"
        );
    }

    #[test]
    fn amend_last_incremental_cache_survives_undo_redo_round_trip() {
        // 🪢 Undo/redo only move edit ids between `applied_edit_ids`/`redo_edit_ids` — they never mutate
        // an edit's own `forwards`, so a cached post-projection keyed by `(edit_id, forwards_len)` stays
        // valid across an undo immediately followed by a redo of the very same coalesced edit.
        let envelope: DocumentEnvelope<DemoProjection, DemoOperation> =
            create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);
        store
            .dispatch(DocumentCommand::AmendLast {
                operations: vec![DemoOperation::SetN { n: 1 }],
                coalesce_key: Some("drag".into()),
            })
            .expect("first amend");
        store.dispatch(DocumentCommand::Undo).expect("undo");
        store.dispatch(DocumentCommand::Redo).expect("redo");
        store
            .dispatch(DocumentCommand::AmendLast {
                operations: vec![DemoOperation::SetN { n: 2 }],
                coalesce_key: Some("drag".into()),
            })
            .expect("amend after undo/redo");
        assert_eq!(store.envelope().vcs.edits.len(), 1, "still coalesced into the original edit");
        assert_eq!(store.projection().expect("projection").n, 2);
        store.dispatch(DocumentCommand::Undo).expect("undo again");
        assert_eq!(store.projection().expect("projection after undo").n, 0);
    }

    #[test]
    fn amend_last_starts_new_edit_when_coalesce_key_differs() {
        let envelope: DocumentEnvelope<DemoProjection, DemoOperation> =
            create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);
        store
            .dispatch(DocumentCommand::AmendLast {
                operations: vec![DemoOperation::SetN { n: 1 }],
                coalesce_key: Some("drag-a".into()),
            })
            .expect("first drag");
        store
            .dispatch(DocumentCommand::AmendLast {
                operations: vec![DemoOperation::SetN { n: 2 }],
                coalesce_key: Some("drag-b".into()),
            })
            .expect("second drag");
        assert_eq!(store.envelope().vcs.edits.len(), 2, "distinct gestures are separate edits");
    }

    #[test]
    fn amend_last_does_not_absorb_into_committed_edit() {
        let envelope: DocumentEnvelope<DemoProjection, DemoOperation> =
            create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);
        store
            .dispatch(DocumentCommand::AmendLast {
                operations: vec![DemoOperation::SetN { n: 1 }],
                coalesce_key: Some("drag".into()),
            })
            .expect("amend");
        store
            .dispatch(DocumentCommand::CommitCheckpoint {
                message: None,
                authors: Vec::new(),
            })
            .expect("commit");
        store
            .dispatch(DocumentCommand::AmendLast {
                operations: vec![DemoOperation::SetN { n: 2 }],
                coalesce_key: Some("drag".into()),
            })
            .expect("amend after commit");
        assert_eq!(
            store.envelope().vcs.edits.len(),
            2,
            "committed edits are never amended, even with a matching coalesce key"
        );
    }

    #[test]
    fn test_support_round_trip_helpers_pass_for_demo_operation() {
        test_support::assert_operation_round_trip(&DemoProjection { n: 4 }, DemoOperation::SetN { n: 9 });
        test_support::assert_store_roundtrip(DemoProjection { n: 4 }, DemoOperation::SetN { n: 9 });

        let edit = Edit::<DemoOperation> {
            id: "edit-command-envelope".into(),
            actor: Some("actor-fallback".into()),
            forwards: vec![DemoOperation::SetN { n: 9 }],
            backwards: vec![DemoOperation::SetN { n: 4 }],
            operation_meta: vec![OperationMeta {
                operation_id: Some(OperationId("op-a".into())),
                dependencies: vec![OperationId("op-0".into())],
                base_version: 0,
                author_id: Some(ActorId("actor-explicit".into())),
                timestamp: protocol::HybridLogicalTimestamp::new(1, 1000),
                undo_policy: protocol::UndoPolicy::ExactBaseOnly,
                payload_hash: None,
            }],
            description: None,
            coalesce_key: None,
            sequence_number: 1,
            started_at: "2026-07-27T00:00:00Z".into(),
            finished_at: None,
        };
        test_support::assert_command_envelope_round_trip::<DemoProjection, DemoOperation>(&edit, &DocumentId("doc-command-envelope".into()));
    }

    /// @emoji 🪤 Proves `assert_command_envelope_round_trip` is not a trivially-true check: a hand-rolled
    /// `Operation` whose `Deserialize` impl silently drops its own field (encodes `n` faithfully but
    /// always decodes to `n: 0`) must trip law (2) of the doc comment on
    /// `assert_command_envelope_round_trip` — the same "deliberately lossy impl" pattern
    /// `protocol_testkit`'s `op_text_round_trip_panics_on_a_lossy_impl` uses for `assert_op_text_round_trip`.
    #[test]
    #[should_panic(expected = "did not decode back into an equal forward operation")]
    fn command_envelope_round_trip_panics_on_a_lossy_operation() {
        #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
        struct LossyDiff;

        impl OperationDiff<DemoProjection> for LossyDiff {
            fn apply(&self, projection: &DemoProjection) -> DemoProjection {
                projection.clone()
            }
            fn absorb(&mut self, _other: Self) {}
        }

        #[derive(Clone, Debug, PartialEq)]
        struct LossyOperation {
            n: i32,
        }

        impl Serialize for LossyOperation {
            fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                serializer.serialize_i32(self.n)
            }
        }

        impl<'de> Deserialize<'de> for LossyOperation {
            fn deserialize<D: serde::Deserializer<'de>>(_deserializer: D) -> Result<Self, D::Error> {
                Ok(LossyOperation { n: 0 })
            }
        }

        impl Operation<DemoProjection> for LossyOperation {
            type Diff = LossyDiff;
            fn diff(&self, _projection: &DemoProjection) -> LossyDiff {
                LossyDiff
            }
            fn backwards(&self, _projection: &DemoProjection) -> Vec<Self> {
                vec![self.clone()]
            }
        }

        let edit = Edit::<LossyOperation> {
            id: "edit-lossy".into(),
            actor: None,
            forwards: vec![LossyOperation { n: 7 }],
            backwards: vec![],
            operation_meta: vec![],
            description: None,
            coalesce_key: None,
            sequence_number: 0,
            started_at: "2026-07-27T00:00:00Z".into(),
            finished_at: None,
        };
        test_support::assert_command_envelope_round_trip::<DemoProjection, LossyOperation>(&edit, &DocumentId("doc-lossy".into()));
    }

    // `DemoProjection`'s `store::DocumentDsl` impl and `DemoOperation`'s `store::OpText` impl are now
    // generated by `#[derive(dsl::DslDocument)]`/`#[derive(dsl::DslOps)]` on the type definitions
    // themselves (see `DemoProjection`/`DemoOperation` above) — the `dsl_schema` grammar replaces
    // this crate's own hand-rolled `"n <value>"`/`"set-n <value>"` printer/parser.

    #[test]
    fn demo_dsl_round_trips() {
        test_support::assert_dsl_round_trip(&DemoProjection { n: 42 });
    }

    #[test]
    fn demo_dsl_pack_equivalence() {
        test_support::assert_dsl_pack_equivalence(&DemoProjection { n: 42 });
    }

    #[test]
    fn demo_op_text_round_trips() {
        test_support::assert_op_line_round_trip(&DemoOperation::SetN { n: 7 });
    }

    #[test]
    fn print_edit_lines_emits_one_indented_line_per_forward_op() {
        let envelope = create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 1 }],
                description: None,
            })
            .expect("apply");
        let edit = store.envelope().vcs.edits.last().expect("edit");
        let printed = print_edit_lines(edit).expect("print edit lines");
        assert!(printed.starts_with("edit "), "got {printed:?}");
        assert!(printed.contains("\n  set-n n=1\n"));
    }

    #[test]
    fn document_text_round_trips_after_apply_and_checkpoint() {
        let envelope = create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 3 }],
                description: Some("bump".into()),
            })
            .expect("apply");
        store
            .dispatch(DocumentCommand::CommitCheckpoint {
                message: Some("c1".into()),
                authors: vec![Author {
                    id: "a1".into(),
                    name: "Alice".into(),
                    avatar: None,
                }],
            })
            .expect("commit");
        test_support::assert_document_text_round_trip(&store);
        test_support::assert_document_pack_round_trip(&store);
    }

    #[test]
    fn parse_document_text_rejects_invalid_op_line_with_span() {
        let files = DocumentTextFiles {
            dsl: "n=0\n".to_string(),
            ops: "doc demo schema=demo/v1\nedit e1 started=\"1\"\n  not-an-op\n".to_string(),
        };
        let error = parse_document_text::<DemoProjection, DemoOperation>(&files.dsl, &files.ops).unwrap_err();
        assert_eq!(error.span.line, 3);
    }

    /// @emoji 🩺 Stresses the stateful `current`/`tail_undo_cache` fast paths — multi-op edits, amend
    /// gestures, undo/redo, and a checkpoint (cold-path recompute) all interleaved — against the
    /// full-replay differential oracle, so any divergence between the incremental paths and a
    /// from-scratch replay fails loudly here rather than surfacing as a silent projection bug later.
    #[test]
    fn stateful_current_matches_full_replay_across_interleaved_commands() {
        let envelope = create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);

        // Multi-operation edit: current must fold both ops, matching a from-scratch replay.
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 1 }, DemoOperation::SetN { n: 2 }],
                description: None,
            })
            .expect("apply multi-op edit");
        test_support::assert_live_equals_replay(&store);
        assert_eq!(store.projection().expect("projection").n, 2);

        // Amend gesture: the first `AmendLast` cannot merge into the preceding `Apply`-created edit
        // (`Apply` never sets a `coalesce_key`, so it can never match), so it starts a NEW edit; the
        // second `AmendLast` shares that edit's key and merges into it — two edits total, the second
        // one carrying two coalesced increments (3 then 4).
        store
            .dispatch(DocumentCommand::AmendLast {
                operations: vec![DemoOperation::SetN { n: 3 }],
                coalesce_key: Some("drag".into()),
            })
            .expect("amend 1");
        store
            .dispatch(DocumentCommand::AmendLast {
                operations: vec![DemoOperation::SetN { n: 4 }],
                coalesce_key: Some("drag".into()),
            })
            .expect("amend 2");
        test_support::assert_live_equals_replay(&store);
        assert_eq!(store.projection().expect("projection").n, 4);
        assert_eq!(store.envelope().vcs.edits.len(), 2, "the amend gesture started its own edit, not a third");

        // Undo the whole amended edit (O(1) tail-cache path) restores the `Apply`-edit's state, not
        // the initial projection — only the amend gesture's edit is undone here.
        store.dispatch(DocumentCommand::Undo).expect("undo");
        test_support::assert_live_equals_replay(&store);
        assert_eq!(store.projection().expect("projection").n, 2);
        store.dispatch(DocumentCommand::Redo).expect("redo");
        test_support::assert_live_equals_replay(&store);
        assert_eq!(store.projection().expect("projection").n, 4);

        // Checkpoint (cold path through `checkout_checkpoint_internal` is NOT exercised by commit
        // itself, but a following apply + a second, older undo still must agree with replay).
        store
            .dispatch(DocumentCommand::CommitCheckpoint {
                message: Some("c1".into()),
                authors: Vec::new(),
            })
            .expect("commit");
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 5 }],
                description: None,
            })
            .expect("apply after checkpoint");
        test_support::assert_live_equals_replay(&store);
        store.dispatch(DocumentCommand::Undo).expect("undo after checkpoint");
        test_support::assert_live_equals_replay(&store);
        assert_eq!(store.projection().expect("projection").n, 4);
    }

    //#region 🏛️StudioTests
    /// @emoji ⏱️ Like `DemoOperation` but with an explicit, test-controlled `timestamp()` override, so
    /// undo-ordering-by-HLT tests don't depend on real wall-clock resolution.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(tag = "operation")]
    enum TimestampedOperation {
        SetN { n: i32, physical_ms: u64 },
    }

    impl Operation<DemoProjection> for TimestampedOperation {
        type Diff = DemoDiff;

        fn diff(&self, _projection: &DemoProjection) -> DemoDiff {
            match self {
                TimestampedOperation::SetN { n, .. } => DemoDiff { n: Some(*n) },
            }
        }

        fn backwards(&self, projection: &DemoProjection) -> Vec<Self> {
            vec![TimestampedOperation::SetN {
                n: projection.n,
                physical_ms: 0,
            }]
        }

        fn timestamp(&self) -> Option<protocol::HybridLogicalTimestamp> {
            match self {
                TimestampedOperation::SetN { physical_ms, .. } => Some(protocol::HybridLogicalTimestamp::new(0, *physical_ms)),
            }
        }
    }

    /// @emoji 🪄 Downcasts a registered `dyn StudioMember` back to its concrete demo store.
    fn demo_member<'a, Operation: crate::Operation<DemoProjection> + 'static>(
        host: &'a mut StudioHost,
        document_id: &str,
    ) -> &'a mut DocumentStore<DemoProjection, Operation> {
        host.member_mut(document_id)
            .expect("member registered")
            .as_any_mut()
            .downcast_mut::<DocumentStore<DemoProjection, Operation>>()
            .expect("concrete member type matches")
    }

    #[test]
    fn studio_checkpoint_commits_dirty_members_and_pins_their_checkpoints() {
        let mut member_a = DocumentStore::new(create_document_envelope::<DemoProjection, DemoOperation>(
            "demo/v1",
            "member-a",
            DemoProjection { n: 0 },
            None,
        ));
        member_a
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 1 }],
                description: None,
            })
            .expect("apply a");

        let mut member_b = DocumentStore::new(create_document_envelope::<DemoProjection, DemoOperation>(
            "demo/v1",
            "member-b",
            DemoProjection { n: 0 },
            None,
        ));
        member_b
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 5 }],
                description: None,
            })
            .expect("apply b");
        member_b
            .dispatch(DocumentCommand::CommitCheckpoint {
                message: Some("b-init".into()),
                authors: Vec::new(),
            })
            .expect("commit b upfront, so it starts clean");
        let member_b_checkpoint = member_b.current_checkpoint_id().expect("b checkpoint").to_string();

        let mut host = StudioHost::new(create_document_envelope(
            "os.studio.history/v1",
            "studio",
            StudioHistoryProjection::default(),
            None,
        ));
        host.register_member(Box::new(member_a));
        host.register_member(Box::new(member_b));

        let studio_checkpoint_id = host
            .commit_studio_checkpoint(
                "studio init".into(),
                vec![Author {
                    id: "a1".into(),
                    name: "Alice".into(),
                    avatar: None,
                }],
            )
            .expect("commit studio checkpoint");

        let projection = host.meta_projection().expect("meta projection");
        assert_eq!(projection.checkpoints.len(), 1);
        let checkpoint = &projection.checkpoints[0];
        assert_eq!(checkpoint.id, studio_checkpoint_id);
        assert_eq!(checkpoint.members.len(), 2, "pins one entry per registered member");
        let pin_b = checkpoint.members.iter().find(|pin| pin.document_id == "member-b").expect("pin b");
        assert_eq!(pin_b.checkpoint_id, member_b_checkpoint, "clean member reuses its existing checkpoint");
        assert!(
            !host.member("member-a").expect("member a").is_dirty(),
            "dirty member-a is committed (and therefore clean) by the studio checkpoint"
        );
    }

    #[test]
    fn studio_vcs_host_meta_document_is_backbone_attachable_and_detachable() {
        let (backbone_a, backbone_b) = MemoryBackbone::pair("studio-a", "studio-b");
        let meta_envelope: DocumentEnvelope<StudioHistoryProjection, StudioHistoryOperation> =
            create_document_envelope("os.studio.history/v1", "studio", StudioHistoryProjection::default(), None);
        let mut host_a = StudioHost::new(meta_envelope.clone());
        let mut host_b = StudioHost::new(meta_envelope);
        assert!(host_a.backbone_ref().is_none(), "default is unattached, like any other DocumentStore");

        host_a.attach_backbone(Box::new(backbone_a)).expect("attach a");
        host_b.attach_backbone(Box::new(backbone_b)).expect("attach b");
        assert!(host_a.backbone_ref().is_some());

        let mut member = DocumentStore::new(create_document_envelope::<DemoProjection, DemoOperation>(
            "demo/v1",
            "member-a",
            DemoProjection { n: 0 },
            None,
        ));
        member
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 1 }],
                description: None,
            })
            .expect("apply on member, so it's dirty and can be committed");
        host_a.register_member(Box::new(member));
        host_a
            .commit_studio_checkpoint("studio init".into(), Vec::new())
            .expect("commit studio checkpoint on a");

        host_b.tick().expect("tick b");
        assert_eq!(
            host_b.meta_projection().expect("meta projection b").checkpoints.len(),
            1,
            "the studio-wide checkpoint replicates through the meta-document's backbone"
        );

        host_a.detach_backbone();
        assert!(host_a.backbone_ref().is_none());
        host_a
            .commit_studio_checkpoint("studio offline".into(), Vec::new())
            .expect("meta history keeps working purely in memory once detached");
        host_b.tick().expect("tick b again");
        assert_eq!(
            host_b.meta_projection().expect("meta projection b unchanged").checkpoints.len(),
            1,
            "detached studio edits never reach the peer"
        );
    }

    #[test]
    fn studio_checkout_checkpoint_fans_out_and_restores_pinned_member_state() {
        let member_a = DocumentStore::new(create_document_envelope::<DemoProjection, DemoOperation>(
            "demo/v1",
            "member-a",
            DemoProjection { n: 0 },
            None,
        ));
        let mut host = StudioHost::new(create_document_envelope(
            "os.studio.history/v1",
            "studio",
            StudioHistoryProjection::default(),
            None,
        ));
        host.register_member(Box::new(member_a));

        demo_member::<DemoOperation>(&mut host, "member-a")
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 1 }],
                description: None,
            })
            .expect("apply 1");
        let studio_checkpoint_1 = host.commit_studio_checkpoint("first".into(), Vec::new()).expect("commit 1");

        demo_member::<DemoOperation>(&mut host, "member-a")
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 2 }],
                description: None,
            })
            .expect("apply 2");
        host.commit_studio_checkpoint("second".into(), Vec::new()).expect("commit 2");
        assert_eq!(
            demo_member::<DemoOperation>(&mut host, "member-a").projection().expect("projection").n,
            2,
            "member reflects the second studio checkpoint before checking out the first"
        );

        host.checkout_studio_checkpoint(&studio_checkpoint_1).expect("checkout studio checkpoint 1");
        assert_eq!(
            demo_member::<DemoOperation>(&mut host, "member-a").projection().expect("projection").n,
            1,
            "checking out the first studio checkpoint fans out and restores member-a's pinned state"
        );
    }

    #[test]
    fn studio_switch_alternative_fans_out_and_restores_pinned_member_state() {
        let member_a = DocumentStore::new(create_document_envelope::<DemoProjection, DemoOperation>(
            "demo/v1",
            "member-a",
            DemoProjection { n: 0 },
            None,
        ));
        let mut host = StudioHost::new(create_document_envelope(
            "os.studio.history/v1",
            "studio",
            StudioHistoryProjection::default(),
            None,
        ));
        host.register_member(Box::new(member_a));

        demo_member::<DemoOperation>(&mut host, "member-a")
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 1 }],
                description: None,
            })
            .expect("apply 1");
        host.commit_studio_checkpoint("root".into(), Vec::new()).expect("commit root");

        let alt_id = host.create_studio_alternative("branch-a".into()).expect("create alternative");

        demo_member::<DemoOperation>(&mut host, "member-a")
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 2 }],
                description: None,
            })
            .expect("apply 2 (uncommitted at the studio level)");
        assert_eq!(
            demo_member::<DemoOperation>(&mut host, "member-a").projection().expect("projection").n,
            2,
            "uncommitted edit is live before switching"
        );

        host.switch_studio_alternative(&alt_id).expect("switch alternative fans out to its pinned checkpoint");
        assert_eq!(
            demo_member::<DemoOperation>(&mut host, "member-a").projection().expect("projection").n,
            1,
            "switching alternatives restores each member to its pinned checkpoint, discarding the uncommitted edit"
        );
    }

    #[test]
    fn studio_undo_and_redo_target_the_member_with_the_most_recent_local_edit_by_hlt() {
        let mut member_early = DocumentStore::new(create_document_envelope::<DemoProjection, TimestampedOperation>(
            "demo-ts/v1",
            "member-early",
            DemoProjection { n: 0 },
            None,
        ));
        member_early
            .dispatch(DocumentCommand::Apply {
                operations: vec![TimestampedOperation::SetN { n: 1, physical_ms: 1_000 }],
                description: None,
            })
            .expect("apply early");

        let mut member_late = DocumentStore::new(create_document_envelope::<DemoProjection, TimestampedOperation>(
            "demo-ts/v1",
            "member-late",
            DemoProjection { n: 0 },
            None,
        ));
        member_late
            .dispatch(DocumentCommand::Apply {
                operations: vec![TimestampedOperation::SetN { n: 9, physical_ms: 2_000 }],
                description: None,
            })
            .expect("apply late");

        let mut host = StudioHost::new(create_document_envelope(
            "os.studio.history/v1",
            "studio",
            StudioHistoryProjection::default(),
            None,
        ));
        host.register_member(Box::new(member_early));
        host.register_member(Box::new(member_late));

        host.undo().expect("studio undo targets the member with the higher HLT");
        assert_eq!(
            demo_member::<TimestampedOperation>(&mut host, "member-early").projection().expect("early projection").n,
            1,
            "earlier local edit (lower HLT) is untouched"
        );
        assert_eq!(
            demo_member::<TimestampedOperation>(&mut host, "member-late").projection().expect("late projection").n,
            0,
            "later local edit (higher HLT) is the one undone"
        );

        host.redo().expect("studio redo targets the most recently undone edit");
        assert_eq!(
            demo_member::<TimestampedOperation>(&mut host, "member-late").projection().expect("late projection after redo").n,
            9,
            "redo restores the member's most recently undone edit"
        );
    }

    #[test]
    fn default_reconcile_hook_is_a_no_op_for_existing_document_kinds() {
        let projection = DemoProjection { n: 4 };
        let (reconciled, conflicts) = DemoOperation::SetN { n: 4 }.reconcile(projection.clone());
        assert_eq!(reconciled, projection, "default reconcile leaves the projection untouched");
        assert!(conflicts.is_empty(), "default reconcile reports no conflicts");

        let envelope = create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 3 }],
                description: None,
            })
            .expect("apply");
        let replayed = materialize_document_projection(store.envelope(), store.applied_edit_ids()).expect("replay");
        assert_eq!(replayed.n, 3, "materialize_document_projection is unaffected by the no-operation default reconcile hook");
        let (with_conflicts, conflicts) = store.projection_with_conflicts().expect("projection with conflicts");
        assert_eq!(with_conflicts.n, 3);
        assert!(conflicts.is_empty());
        assert!(store.conflicts().is_empty(), "no remote ingestion happened, so the store's conflict buffer stays empty");
    }

    #[test]
    fn studio_history_op_round_trips() {
        let checkpoint = StudioCheckpoint {
            id: "sc-1".into(),
            parent_id: None,
            message: "root".into(),
            authors: Vec::new(),
            timestamp: HybridLogicalTimestamp::new(0, 1),
            members: vec![StudioMemberPin {
                document_id: "member-a".into(),
                checkpoint_id: "cp-1".into(),
                alternative_id: String::new(),
            }],
        };
        test_support::assert_operation_round_trip(
            &StudioHistoryProjection::default(),
            StudioHistoryOperation::CommitStudioCheckpoint {
                checkpoint: checkpoint.clone(),
            },
        );

        let with_checkpoint = StudioHistoryProjection {
            checkpoints: vec![checkpoint],
            alternatives: Vec::new(),
            active_alternative_id: None,
        };
        let alternative = StudioAlternative {
            id: "sa-1".into(),
            name: "branch".into(),
            checkpoint_ids: vec!["sc-1".into()],
        };
        test_support::assert_operation_round_trip(
            &with_checkpoint,
            StudioHistoryOperation::CreateStudioAlternative { alternative },
        );

        let with_alternative_active = StudioHistoryProjection {
            active_alternative_id: Some("sa-1".into()),
            ..with_checkpoint
        };
        test_support::assert_operation_round_trip(
            &with_alternative_active,
            StudioHistoryOperation::SwitchStudioAlternative {
                alternative_id: "sa-other".into(),
            },
        );
    }

    //#endregion 🏛️StudioTests

    //#region 🔖TextFormatHelpers
    #[test]
    fn ops_author_conversion_drops_avatar_matching_the_ops_text_format() {
        let author = Author { id: "a1".into(), name: "Alice".into(), avatar: Some("http://example/a1.png".into()) };
        let round_tripped: Author = OpsAuthor::from(&author).into();
        assert_eq!(round_tripped, Author { id: "a1".into(), name: "Alice".into(), avatar: None }, "OpsAuthor never carries avatar — it is not part of the .ops text format");
    }

    #[test]
    fn ops_header_line_checkpoint_round_trips_including_delimiter_and_quote_characters_in_authors() {
        let header = OpsHeaderLine::Checkpoint {
            id: "c1".to_string(),
            at: "18".to_string(),
            changes: vec!["ch1".to_string(), "ch2".to_string()],
            parent: None,
            by: vec![
                OpsAuthor { id: "a:1,x".to_string(), name: "Alice, A. \"the great\"".to_string() },
                OpsAuthor { id: "b2".to_string(), name: "Bob".to_string() },
            ],
            message: Some("first \"checkpoint\"".to_string()),
        };
        let printed = header.print_op();
        assert!(!printed.contains('\n'), "print_op must be one line: {printed:?}");
        assert!(!printed.contains("parent="), "an absent optional field must be omitted, not printed as a '-' placeholder: {printed}");
        let parsed = OpsHeaderLine::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op failed for {printed:?}: {e}"));
        assert_eq!(parsed, header, "OpsHeaderLine::Checkpoint round trip diverged for {printed:?}");
    }

    #[test]
    fn ops_header_line_edit_round_trips_including_a_quoted_description() {
        let header = OpsHeaderLine::Edit {
            id: "e1".to_string(),
            started: "1".to_string(),
            actor: None,
            finished: None,
            key: None,
            description: Some("hello \"world\"".to_string()),
        };
        let printed = header.print_op();
        assert!(!printed.contains('\n'), "print_op must be one line: {printed:?}");
        assert!(!printed.contains("actor="), "an absent optional field must be omitted: {printed}");
        let parsed = OpsHeaderLine::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op failed for {printed:?}: {e}"));
        assert_eq!(parsed, header, "OpsHeaderLine::Edit round trip diverged for {printed:?}");
    }

    #[test]
    fn ops_header_line_parse_op_rejects_a_line_with_no_known_keyword() {
        let error = OpsHeaderLine::parse_op("not a structural line").unwrap_err();
        assert!(error.message.contains("unknown operation line"), "got {error:?}");
    }

    #[test]
    fn parse_document_text_rejects_a_header_line_missing_its_required_positional_id() {
        let files = DocumentTextFiles {
            dsl: "n=0\n".to_string(),
            ops: "active\n".to_string(),
        };
        let error = parse_document_text::<DemoProjection, DemoOperation>(&files.dsl, &files.ops).unwrap_err();
        assert!(error.message.contains("expected Text"), "got {error:?}");
        assert_eq!(error.span.line, 1);
    }

    #[test]
    fn parse_document_text_rejects_an_unknown_header_line_keyword() {
        let files = DocumentTextFiles {
            dsl: "n=0\n".to_string(),
            ops: "doc demo schema=demo/v1\nbogus id=x\n".to_string(),
        };
        let error = parse_document_text::<DemoProjection, DemoOperation>(&files.dsl, &files.ops).unwrap_err();
        assert!(error.message.contains("unknown operation line"), "got {error:?}");
        assert_eq!(error.span.line, 2);
    }

    #[test]
    fn document_text_round_trips_with_an_active_alternative_and_a_quoted_description() {
        let envelope = create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 1 }],
                description: Some("said \"hi\" and used a \\ backslash".into()),
            })
            .expect("apply");
        store
            .dispatch(DocumentCommand::CreateAlternative { name: "branch \"a\"".into() })
            .expect("create alternative (auto-commits and activates it)");
        assert!(store.envelope().active_alternative_id.is_some(), "precondition: an alternative is active");
        let files = print_document_text(store.envelope()).expect("print document text");
        assert!(files.ops.lines().any(|line| line.starts_with("active ")), "an active alternative must print an `active` header line: {}", files.ops);
        test_support::assert_document_text_round_trip(&store);
        test_support::assert_document_pack_round_trip(&store);
    }

    //#endregion 🔖TextFormatHelpers

    //#region 🔖CommandErrorPaths
    #[test]
    fn apply_with_no_operations_is_rejected() {
        let envelope: DocumentEnvelope<DemoProjection, DemoOperation> =
            create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);
        let error = store
            .dispatch(DocumentCommand::Apply { operations: Vec::new(), description: None })
            .unwrap_err();
        assert_eq!(error, VcsError::EmptyApply);
    }

    #[test]
    fn amend_last_with_no_operations_is_rejected() {
        let envelope: DocumentEnvelope<DemoProjection, DemoOperation> =
            create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);
        let error = store
            .dispatch(DocumentCommand::AmendLast { operations: Vec::new(), coalesce_key: None })
            .unwrap_err();
        assert_eq!(error, VcsError::EmptyApply);
    }

    #[test]
    fn undo_with_nothing_applied_is_rejected() {
        let envelope: DocumentEnvelope<DemoProjection, DemoOperation> =
            create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);
        assert_eq!(store.dispatch(DocumentCommand::Undo).unwrap_err(), VcsError::NothingToUndo);
    }

    #[test]
    fn redo_with_nothing_undone_is_rejected() {
        let envelope: DocumentEnvelope<DemoProjection, DemoOperation> =
            create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);
        assert_eq!(store.dispatch(DocumentCommand::Redo).unwrap_err(), VcsError::NothingToRedo);
    }

    #[test]
    fn checkout_of_an_unknown_checkpoint_is_rejected() {
        let envelope: DocumentEnvelope<DemoProjection, DemoOperation> =
            create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);
        let error = store
            .dispatch(DocumentCommand::CheckoutCheckpoint { checkpoint_id: "nope".into() })
            .unwrap_err();
        assert_eq!(error, VcsError::UnknownChange("nope".into()));
    }

    #[test]
    fn switch_to_an_unknown_alternative_is_rejected() {
        let envelope: DocumentEnvelope<DemoProjection, DemoOperation> =
            create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);
        let error = store
            .dispatch(DocumentCommand::SwitchAlternative { alternative_id: "nope".into() })
            .unwrap_err();
        assert_eq!(error, VcsError::UnknownAlternative("nope".into()));
    }

    #[test]
    fn switch_to_an_alternative_whose_pinned_checkpoint_is_missing_is_rejected() {
        let mut envelope: DocumentEnvelope<DemoProjection, DemoOperation> =
            create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        envelope.vcs.alternatives.push(Alternative {
            id: "alt-dangling".into(),
            name: "dangling".into(),
            checkpoint_ids: vec!["checkpoint-that-was-never-recorded".into()],
        });
        let mut store = DocumentStore::new(envelope);
        let error = store
            .dispatch(DocumentCommand::SwitchAlternative { alternative_id: "alt-dangling".into() })
            .unwrap_err();
        assert_eq!(error, VcsError::NoCheckpoint, "the alternative's pinned checkpoint id must actually exist");
    }

    #[test]
    fn create_alternative_with_no_edits_and_no_checkpoints_is_rejected() {
        let envelope: DocumentEnvelope<DemoProjection, DemoOperation> =
            create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);
        let error = store
            .dispatch(DocumentCommand::CreateAlternative { name: "x".into() })
            .unwrap_err();
        assert_eq!(error, VcsError::NoCheckpoint, "the auto-commit has nothing pending, so there is still no checkpoint to branch from");
    }

    #[test]
    fn compensating_undo_without_a_semantic_command_is_rejected() {
        let envelope: DocumentEnvelope<DemoProjection, DemoOperation> =
            create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);
        store
            .dispatch(DocumentCommand::Apply { operations: vec![DemoOperation::SetN { n: 1 }], description: None })
            .expect("apply");
        let error = store
            .dispatch(DocumentCommand::UndoWithPolicy { policy: UndoPolicy::CompensatingAction, semantic_command: None })
            .unwrap_err();
        assert!(matches!(error, VcsError::Backbone(_)), "got {error:?}");
    }

    #[test]
    fn materialize_document_projection_rejects_an_unknown_edit_id() {
        let envelope: DocumentEnvelope<DemoProjection, DemoOperation> =
            create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let error = materialize_document_projection(&envelope, &["missing-edit".to_string()]).unwrap_err();
        assert_eq!(error, VcsError::UnknownEdit("missing-edit".into()));
    }

    #[test]
    fn dispatch_json_applies_a_serialized_command_and_projection_json_reflects_it() {
        let envelope: DocumentEnvelope<DemoProjection, DemoOperation> =
            create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);
        let command_json = serde_json::to_string(&DocumentCommand::Apply {
            operations: vec![DemoOperation::SetN { n: 7 }],
            description: None,
        })
        .expect("serialize command");
        store.dispatch_json(&command_json).expect("dispatch json");
        assert_eq!(store.projection_json().expect("projection json"), serde_json::to_string(&DemoProjection { n: 7 }).unwrap());

        let error = store.dispatch_json("not json").unwrap_err();
        assert!(matches!(error, VcsError::Deserialize(_)), "got {error:?}");
    }

    //#endregion 🔖CommandErrorPaths

    //#region 🔖ReconcileAlternative
    #[test]
    fn reconcile_alternative_requires_an_existing_checkpoint() {
        let mut envelope: DocumentEnvelope<DemoProjection, DemoOperation> =
            create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let error = reconcile_alternative(&mut envelope, "reconciled", None, Vec::new()).unwrap_err();
        assert_eq!(error, VcsError::NoCheckpoint);
    }

    #[test]
    fn reconcile_alternative_pins_the_latest_checkpoint_and_optionally_records_a_reconciliation_checkpoint() {
        let envelope: DocumentEnvelope<DemoProjection, DemoOperation> =
            create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);
        store
            .dispatch(DocumentCommand::Apply { operations: vec![DemoOperation::SetN { n: 1 }], description: None })
            .expect("apply");
        store
            .dispatch(DocumentCommand::CommitCheckpoint { message: Some("c1".into()), authors: Vec::new() })
            .expect("commit");
        let base_checkpoint_id = store.envelope().vcs.checkpoints[0].id.clone();

        let mut without_message = store.envelope().clone();
        let alt_id = reconcile_alternative(&mut without_message, "no-record", None, Vec::new()).expect("reconcile without message");
        assert_eq!(without_message.vcs.alternatives.last().unwrap().checkpoint_ids, vec![base_checkpoint_id.clone()]);
        assert_eq!(without_message.vcs.checkpoints.len(), 1, "no checkpoint_message means no new checkpoint is recorded");
        assert!(!alt_id.is_empty());

        let mut with_message = store.envelope().clone();
        let authors = vec![Author { id: "a1".into(), name: "Alice".into(), avatar: None }];
        reconcile_alternative(&mut with_message, "recorded", Some("merged concurrent work".into()), authors.clone())
            .expect("reconcile with message");
        assert_eq!(with_message.vcs.checkpoints.len(), 2, "a checkpoint_message appends one reconciliation checkpoint");
        let recorded_checkpoint = with_message.vcs.checkpoints.last().unwrap();
        assert_eq!(recorded_checkpoint.parent_id, Some(base_checkpoint_id));
        assert_eq!(recorded_checkpoint.authors, authors);
        assert_eq!(recorded_checkpoint.message, Some("reconciled".into()), "the reconciliation checkpoint's own message is fixed, distinct from the change description");
        assert_eq!(
            with_message.vcs.changes.last().unwrap().description,
            Some("merged concurrent work".into()),
            "the passed checkpoint_message becomes the change's description"
        );
    }

    #[test]
    fn commit_checkpoint_mints_distinct_content_addressed_ids_for_distinct_commits() {
        let envelope: DocumentEnvelope<DemoProjection, DemoOperation> = create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);
        store.dispatch(DocumentCommand::Apply { operations: vec![DemoOperation::SetN { n: 1 }], description: None }).expect("apply 1");
        store.dispatch(DocumentCommand::CommitCheckpoint { message: Some("first".into()), authors: Vec::new() }).expect("commit 1");
        store.dispatch(DocumentCommand::Apply { operations: vec![DemoOperation::SetN { n: 2 }], description: None }).expect("apply 2");
        store.dispatch(DocumentCommand::CommitCheckpoint { message: Some("second".into()), authors: Vec::new() }).expect("commit 2");

        let ids: Vec<&str> = store.envelope().vcs.checkpoints.iter().map(|checkpoint| checkpoint.id.as_str()).collect();
        assert_eq!(ids.len(), 2);
        assert_ne!(ids[0], ids[1], "two distinct commits must mint two distinct checkpoint ids");
        assert!(ids.iter().all(|id| id.starts_with("ck-")));
    }

    #[test]
    fn merge_base_finds_the_nearest_common_ancestor_across_a_fork() {
        let envelope: DocumentEnvelope<DemoProjection, DemoOperation> = create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);
        store.dispatch(DocumentCommand::Apply { operations: vec![DemoOperation::SetN { n: 1 }], description: None }).expect("apply root");
        store.dispatch(DocumentCommand::CommitCheckpoint { message: Some("root".into()), authors: Vec::new() }).expect("commit root");
        let root_id = store.envelope().vcs.checkpoints[0].id.clone();

        store.dispatch(DocumentCommand::CreateAlternative { name: "feature-a".into() }).expect("create feature-a");
        store.dispatch(DocumentCommand::Apply { operations: vec![DemoOperation::SetN { n: 2 }], description: None }).expect("apply a");
        store.dispatch(DocumentCommand::CommitCheckpoint { message: Some("a1".into()), authors: Vec::new() }).expect("commit a1");
        let a1_id = store.envelope().vcs.checkpoints.last().unwrap().id.clone();

        store.dispatch(DocumentCommand::CheckoutCheckpoint { checkpoint_id: root_id.clone() }).expect("checkout root");
        store.dispatch(DocumentCommand::CreateAlternative { name: "feature-b".into() }).expect("create feature-b");
        store.dispatch(DocumentCommand::Apply { operations: vec![DemoOperation::SetN { n: 3 }], description: None }).expect("apply b");
        store.dispatch(DocumentCommand::CommitCheckpoint { message: Some("b1".into()), authors: Vec::new() }).expect("commit b1");
        let b1_id = store.envelope().vcs.checkpoints.last().unwrap().id.clone();

        assert_eq!(merge_base(store.envelope(), &a1_id, &b1_id), Some(root_id.clone()), "a1 and b1 forked at root");
        assert_eq!(merge_base(store.envelope(), &a1_id, &root_id), Some(root_id.clone()), "root is its own descendant's merge-base");
        assert_eq!(merge_base(store.envelope(), &root_id, &root_id), Some(root_id), "a checkpoint is its own merge-base");
    }

    #[test]
    fn merge_base_is_none_for_a_dangling_unknown_checkpoint_id() {
        let envelope: DocumentEnvelope<DemoProjection, DemoOperation> = create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);
        store.dispatch(DocumentCommand::Apply { operations: vec![DemoOperation::SetN { n: 1 }], description: None }).expect("apply");
        store.dispatch(DocumentCommand::CommitCheckpoint { message: Some("root".into()), authors: Vec::new() }).expect("commit");
        let root_id = store.envelope().vcs.checkpoints[0].id.clone();

        assert_eq!(merge_base(store.envelope(), &root_id, "unknown-checkpoint"), None, "an id absent from the checkpoint list shares no ancestry with anything");
    }

    //#endregion 🔖ContentAddressedCheckpointAndMergeBase

    //#region 🔖RemoteSnapshotMerge
    #[test]
    fn snapshot_merge_into_a_nonempty_store_adds_only_the_new_remote_edits_and_records() {
        let envelope: DocumentEnvelope<DemoProjection, DemoOperation> =
            create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);
        store
            .dispatch(DocumentCommand::Apply { operations: vec![DemoOperation::SetN { n: 1 }], description: None })
            .expect("local apply");
        store
            .dispatch(DocumentCommand::CommitCheckpoint { message: Some("local".into()), authors: Vec::new() })
            .expect("local commit");

        let mut remote_store = DocumentStore::new(store.envelope().clone());
        remote_store.set_state(store.envelope().clone(), store.applied_edit_ids().to_vec(), Vec::new());
        remote_store
            .dispatch(DocumentCommand::Apply { operations: vec![DemoOperation::SetN { n: 2 }], description: None })
            .expect("remote apply");
        remote_store
            .dispatch(DocumentCommand::CommitCheckpoint { message: Some("remote".into()), authors: Vec::new() })
            .expect("remote commit");

        let (channel, remote_end) = ChannelBackbone::pair("chan");
        store.attach_backbone(Box::new(channel)).expect("attach");
        let _ = remote_end.drain().expect("drain attach snapshot");
        remote_end
            .push(BackboneMessage::Snapshot { envelope_json: remote_store.envelope_json().expect("remote json") })
            .expect("push snapshot");
        store.tick().expect("tick merges the pushed snapshot");

        assert_eq!(store.envelope().vcs.edits.len(), 2, "the shared original edit is deduped, only the new remote edit is added");
        assert_eq!(store.envelope().vcs.checkpoints.len(), 2, "the remote's new checkpoint is merged in by id");
        assert_eq!(store.projection().expect("projection").n, 2, "current folds in the newly merged edit's forwards");
    }

    //#endregion 🔖RemoteSnapshotMerge

    //#region 🔖StudioMemberCheckoutRouting
    #[test]
    fn studio_member_checkout_switches_at_the_alternative_tip_and_falls_back_to_checkout_when_stale() {
        let envelope: DocumentEnvelope<DemoProjection, DemoOperation> =
            create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);
        store
            .dispatch(DocumentCommand::Apply { operations: vec![DemoOperation::SetN { n: 1 }], description: None })
            .expect("apply");
        store
            .dispatch(DocumentCommand::CreateAlternative { name: "feature".into() })
            .expect("create alternative (auto-commits since no checkpoint existed yet)");
        let alt_id = store.envelope().vcs.alternatives[0].id.clone();
        let tip = store.envelope().vcs.alternatives[0].checkpoint_ids.last().expect("alt has a tip").clone();

        StudioMember::checkout(&mut store, &tip, &alt_id).expect("checkout at the tip routes through SwitchAlternative");
        assert_eq!(store.envelope().active_alternative_id, Some(alt_id.clone()), "switching to the tip keeps it active");

        store
            .dispatch(DocumentCommand::Apply { operations: vec![DemoOperation::SetN { n: 2 }], description: None })
            .expect("apply on branch");
        store
            .dispatch(DocumentCommand::CommitCheckpoint { message: Some("c2".into()), authors: Vec::new() })
            .expect("commit c2, advancing the alt's tip past `tip`");

        StudioMember::checkout(&mut store, &tip, &alt_id).expect("checkout of the now-stale tip falls back to CheckoutCheckpoint");
        assert_eq!(store.projection().expect("projection").n, 1, "restored the old checkpoint's state");
        assert_eq!(
            store.envelope().active_alternative_id, None,
            "the checked-out checkpoint is no longer any alternative's tip, so nothing is active"
        );
    }

    //#endregion 🔖StudioMemberCheckoutRouting

    //#region 🔖BackbonePorts
    #[test]
    fn memory_backbone_port_round_trips_and_reports_a_missing_file() {
        let port = MemoryBackbonePort::new();
        let error = port.read("file://nowhere").unwrap_err();
        assert!(matches!(error, VcsError::Backbone(_)), "got {error:?}");
        port.write("file://a", "payload-1").expect("write");
        assert_eq!(port.read("file://a").expect("read"), "payload-1");
        port.write("file://a", "payload-2").expect("overwrite");
        assert_eq!(port.read("file://a").expect("read after overwrite"), "payload-2", "write is an upsert");
    }

    #[test]
    fn local_storage_backbone_port_falls_back_to_its_in_memory_store() {
        let port = LocalStorageBackbonePort::new();
        let error = port.read("local://missing").unwrap_err();
        assert!(matches!(error, VcsError::Backbone(_)), "got {error:?}");
        port.write("local://a", "value").expect("write falls back to the in-memory store");
        assert_eq!(port.read("local://a").expect("read falls back too"), "value");

        let defaulted = LocalStorageBackbonePort::default();
        assert!(defaulted.read("local://a").is_err(), "Default constructs its own independent fallback store");
    }

    //#endregion 🔖BackbonePorts
}
//#endregion 🧪Tests


}
//#endregion 🧪Tests
