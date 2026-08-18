//! 🧬️ Wires snapshot schema — artifact-lane fields only.
//!
//! Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` (`reasoning/dag→C:graph`): `content` composes
//! stdio's neutral `s.stdio.semio.graph` subset (nodes/edges) instead of an inline `board_fixture`
//! blob. `camera`/`meta` stay their own small persisted `DslValue` fields (view state / app config,
//! never part of the neutral graph subset — see `crate::artifacts::wires`'s module doc).
//!
//! Ticket `26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM` design.md §1 CORRECTION: the native
//! codec (`impl store::ArtifactDsl`/`impl store::ArtifactPack for WiresSnapshot`, formerly here) now
//! lives directly under `🚪️io/📸️snapshot/{📝️text,💾️binary}` — one bidirectional trait impl per
//! representation, unsplit, never mirrored under import/export. This file keeps only the type + its
//! schema derive, per design.md rule 3 ("🧬️schema keeps types + pure transforms").

use crate::artifacts::wires::WiresContentChild;
use dsl::DslValue;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Snapshot
/// 📸️ Persisted wires document snapshot (persistent fields of the artifact).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.reasoning.wires")]
pub struct WiresSnapshot {
    #[state(artifact)]
    pub wires_fixture: DslValue,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.graph")]
    pub content: WiresContentChild,
    #[state(artifact)]
    pub camera: DslValue,
    #[state(artifact)]
    pub meta: DslValue,
}
//#endregion 🔖️Snapshot
