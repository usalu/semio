//! 🧬️ Wires snapshot schema — artifact-lane fields only.
//!
//! `content` composes stdio's neutral `s.stdio.semio@v1/graph` subset: the board's nodes and edges live ONLY in that child
//! (design §20.15); `wires_fixture` is the parent's identity layer (`schema`, `identities`) and `meta` the kind catalogues,
//! while the camera is owned by each concrete canvas window.
//!
//! Ticket `26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM` design.md §1 CORRECTION: the native
//! codec (`impl store::ArtifactDsl`/`impl store::ArtifactPack for WiresSnapshot`, formerly here) now
//! lives directly under `🚪️io/📸️snapshot/{📝️text,💾️binary}` — one bidirectional trait impl per
//! representation, unsplit, never mirrored under import/export. This file keeps only the type + its
//! schema derive, per design.md rule 3 ("🧬️schema keeps types + pure transforms").

use crate::WiresContentChild;
use semio_framework_value::DslValue;
use framework_schema::ArtifactSchema;

//#region 🔖️Snapshot
/// 📸️ Persisted wires document snapshot (persistent fields of the artifact).
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.reasoning.wires")]
pub struct WiresSnapshot {
    #[state(artifact)]
    pub wires_fixture: DslValue,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub content: WiresContentChild,
    #[state(artifact)]
    pub meta: DslValue,
}
#[path="📦️pack/🦀️.rs"]
pub(crate) mod owned_pack;
#[path="🪶️sqlite/🦀️.rs"]
mod sqlite_snapshot;
#[cfg(test)]
#[path="🧪️tests/🪶️sqlite/🦀️.rs"]
mod sqlite_snapshot_tests;
//#endregion 🔖️Snapshot

