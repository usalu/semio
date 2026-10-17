//! 📇️ Directory event log wire contract (ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-
//! STUDIOS, contract C1): `DirectoryEvent`/`DirectoryEventBody` (persisted, backend-assigned dense
//! `seq`), `DirectoryCommand` (client intent posted to `/directory/commands`), `DirectoryStreamMessage`
//! (the `/directory/socket/v1` wire envelope), and the read DTOs (`SpaceView`/`MemberView`/`UserView`/
//! `ConnectionView`/`DocumentView`/`InviteView`) the hub's REST surface returns. Pure data, no fold
//! logic — see the module root `../🦀️.rs`'s `DirectoryReadModel`/`fold`. `DirectorySpaceKind`/
//! `DirectorySpaceVisibility`/`DirectorySpaceRole` mirror `🪐️space`'s `SpaceKind`/`SpaceVisibility`/
//! `SpaceRole` vocabulary (atelier/studio/archive, private/public, author/spectator) string-identically;
//! this wasm-safe kernel crate does not mount that module (`🦀️.rs`'s header note: unwired pending
//! dep-DAG cleanup), so the enums are re-declared here, same convention `🌎️hub/📇️directory`'s
//! `SpaceRole` already uses for the same reason.
//!
//! 🧭️ `space.created`'s and `create-space`'s space-kind fields are named `space_kind`
//! (`spaceKind` on the wire), not contract-freeze.md's bare `kind` — both bodies are internally
//! tagged (`#[value(tag = "kind")]`), so a same-named payload field would collide with the
//! discriminator. Flagged as a `sharedFileRequest` in lane 0-A's report.
//!
//! 🌉️ `ToValue`/`FromValue` (`#[derive(ToValue, FromValue)]`, not a `serde_json`-backed bridge):
//! unblocked by `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-
//! AND-ARTIFACTS/🔍️research/📓️dslvalue-integer-fidelity.md` — `DslValue::Number` now carries
//! `UInt`/`Int`/`Float` (not a lone `f64`), so `CreateInvite.ttl_secs: u64` etc. round-trip as bare
//! integers (`3600`, never `3600.0`) the way this contract's real external hub (`🌎️hub`'s sibling
//! Rust/serde types, strict — no `arbitrary_precision`) requires on the wire. An earlier pass
//! (`📓️directory-spr-serde-removal.md`) declined this conversion for exactly that reason, before the
//! fix landed. `#[value(...)]` mirrors every `#[serde(...)]` shape this file used: `tag` +
//! `rename_all_fields`, per-variant `rename`, and mixed `rename_all` casings across sibling enums —
//! all supported by `semio_framework_value_derive` today (see its own header docs).

use semio_framework_value_derive::{FromValue, ToValue};

#[path = "🪪️session-authority-v1/🦀️.rs"]
pub mod session_authority;
pub use session_authority::{DIRECTORY_SESSION_AUTHORITY_MAX_BYTES, DirectorySessionAuthorityV1, DirectorySessionKindV1};

#[path = "🌱️space-artifact-creation-v1/🦀️.rs"]
pub mod space_artifact_creation;

#[path = "📇️document-index-v1/🦀️.rs"]
pub mod document_index;
pub use document_index::{DirectoryIndexedDocumentViewV1, DocumentIndexEntryV1};

#[path = "📌️document-check-in-v1/🦀️.rs"]
pub mod document_check_in;
pub use document_check_in::{
    DocumentCheckInPhaseV1, DocumentCheckInProgressV1, DocumentCheckInReadyV1, DocumentCheckInRefusalV1, DocumentCheckInStatusV1, DocumentCheckInV1, DOCUMENT_CHECK_IN_MAX_BYTES, DOCUMENT_CHECK_IN_SCHEMA_V1, DOCUMENT_CHECK_IN_STATUS_SCHEMA_V1,
};

#[path = "🪢️canonical-checkpoint-pair-v1/🦀️.rs"]
pub mod canonical_checkpoint_pair;
pub use canonical_checkpoint_pair::{
    AdmittedCheckpointSelectionV1, CanonicalCheckpointPairRefusalV1, CanonicalCheckpointPairV1,
};

#[path = "🌐️browser-actor/🦀️.rs"]
pub mod browser_actor;
pub use browser_actor::{
    DOCUMENT_BROWSER_ACTOR_INTERFACES, DOCUMENT_BROWSER_ACTOR_MAX_BYTES, DocumentBrowserActorByteLengthV1, DocumentBrowserActorErrorV1, DocumentBrowserActorSourceV1, DocumentExecutionTargetBrowserActorV1, DocumentOpenBrowserActorV1,
};

#[path = "📖️events-commands-and-admin/🦀️.rs"]
mod events_commands_and_admin;
pub use events_commands_and_admin::*;

#[path = "🪟️views-and-document-open/🦀️.rs"]
mod views_and_document_open;
pub use views_and_document_open::*;

#[path = "🛰️execution-target-artifacts-and-stream/🦀️.rs"]
mod execution_target_artifacts_and_stream;
pub use execution_target_artifacts_and_stream::*;
