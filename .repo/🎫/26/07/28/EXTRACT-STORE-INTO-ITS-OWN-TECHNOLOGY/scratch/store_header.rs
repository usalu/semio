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

