//! 🗄️ Generic document VCS engine — Operation/Edit/Change/Checkpoint/Alternative, materialize-by-replay, backbone.

// The `dsl::DslDocument`/`dsl::DslOps` derive macros emit `::vcs::DocumentDsl`/`::vcs::OpText` paths
// (see `dsl/derive/rs/lib.rs`), which only resolve for crates that depend on `vcs` as an external
// crate — every real consumer, INCLUDING this crate's own `.ops` header grammar (`OpsHeaderLine` in
// `🔖TextFormat` below, derived on the engine directly) as well as its in-crate `🔖Dsl`/`🔖OpText`
// test fixtures (a crate is never its own dependency otherwise). `extern crate self as vcs;` is the
// same fix `dsl/rs/lib.rs` uses for its own in-crate derive usage: it makes `::vcs` resolve to this
// crate even when the derive is exercised in-crate. Unconditional (not `#[cfg(test)]`-gated) because
// `OpsHeaderLine` is production code, not just a test fixture.
extern crate self as vcs;

use dsl::{DslOps, DslRecord};
use semio_framework_core::{
    ActorId, DocumentDiff, DocumentId, DocumentVersion, HybridLogicalTimestamp, InverseOperation, OperationEnvelope, OperationId, PayloadHash, SchemaId, SchemaVersion, UndoPolicy,
};
// 🎞️ CW5 fix: unconditional now — `operation_envelope_from_edit` below calls `hash_bytes` on every
// target (not just native), so gating the import to `not(wasm32)` broke the wasm32 build entirely
// (`vcs` is a dependency of `framework/sync`'s wasm actor). `semio-framework-hash` itself is an
// unconditional dependency (pure blake3, no OS dependency) — only the native-only `BlobStore for
// FolderSqliteStorage` impl further down actually needed the old native-only restriction, and that
// impl block is already separately `#[cfg(not(target_arch = "wasm32"))]`-gated on its own.
use semio_framework_hash::hash_bytes;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use thiserror::Error;

// 🎞️ CW8: the temporary public `pub use protocol::{...}` shim (CW3-CW8) is gone — every dependent
// crate now imports `protocol::{Operation, OperationDiff, OpText, OperationMeta, Edit,
// ReconcileReport, ...}` directly, so `vcs::Operation`/`vcs::OpText`/etc no longer resolve
// externally. This crate's OWN body still spells these names bare throughout (generic bounds like
// `Operation: OpText`, `Edit<Operation>` struct literals, `crate::Operation<P>` disambiguating the
// trait from the same-named generic parameter, etc.) — a private (non-`pub`) import keeps that
// internal ergonomics unchanged without re-exposing the names on `vcs`'s own public API.
// `merge_concurrent_diffs`/`ReconcileSeverity` aren't referenced bare anywhere in this crate (only
// fully-qualified `protocol::` elsewhere), so they're deliberately not imported here.
use protocol::{Edit, Operation, OperationDiff, OperationMeta, OpText, ReconcileReport};

static ID_COUNTER: AtomicU64 = AtomicU64::new(0);

/// @emoji 🆔 Allocates stable ids for document VCS entities.
pub fn create_document_vcs_id(prefix: &str) -> String {
    let n = ID_COUNTER.fetch_add(1, Ordering::Relaxed) + 1;
    format!("{prefix}-{n}")
}

