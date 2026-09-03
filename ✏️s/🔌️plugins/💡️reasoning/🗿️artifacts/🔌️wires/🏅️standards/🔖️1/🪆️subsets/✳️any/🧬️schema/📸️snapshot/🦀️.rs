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

//#region 🔖️Snapshot
/// 📸️ Persisted wires document snapshot (persistent fields of the artifact).
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
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

//#region 🌉️ExternalCodecBridge
/// 📤️ Renders a [`WiresSnapshot`] as this facet's own camelCase JSON projection — the comparison
/// surface `mutate-wires-1`'s scenarios are measured through, and the shape the committed
/// `../🧬️mutations/<slug>/🧪️tests/<fixture>/📸️snapshot/{⬅️before,➡️after}/🔣️.json`
/// specification vectors are written in. Unlike `dag.dag`'s, this projection carries the board
/// INLINE inside `wiresFixture`, so a mutation's effect is visible in it directly rather than only
/// through a content digest.
///
/// A thin `dsl::os_pack::json` wrapper over `WiresSnapshot`'s own `ToValue` impl — first-party,
/// infallible (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS).
pub fn encode_wires_snapshot_json(snapshot: &WiresSnapshot) -> String {
    dsl::os_pack::json::to_json_string(snapshot)
}

/// 📥️ The inverse of [`encode_wires_snapshot_json`] — decodes those committed specification vectors
/// into real [`WiresSnapshot`] values, so `mutate-wires-1`'s adapter reads the committed fixture
/// rather than re-declaring it as a Rust literal beside it.
pub fn decode_wires_snapshot_json(text: &str) -> Result<WiresSnapshot, String> {
    dsl::os_pack::json::from_json_str(text).map_err(|error| error.to_string())
}

/// 📝️ Parses `.wires.dsl.semio` text into a [`WiresSnapshot`] — a named, non-async pass-through of
/// this type's own `store::ArtifactDsl` impl (`../../🚪️io/📸️snapshot/📝️text/🦀️.rs`), whose
/// trait and error type are both unnameable outside this crate, so `mutate-wires-1`'s
/// `identity-round-trip` scenario reaches the real committed artifact
/// (`../../📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio`) through this instead.
pub fn parse_wires_dsl(text: &str) -> Result<WiresSnapshot, String> {
    <WiresSnapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| format!("{error:?}"))
}

/// 📝️ Renders a [`WiresSnapshot`] back as `.wires.dsl.semio` text — the inverse of
/// [`parse_wires_dsl`], preamble and all five hex-encoded lines included.
pub fn print_wires_dsl(snapshot: &WiresSnapshot) -> String {
    store::ArtifactDsl::print_dsl(snapshot)
}

/// 🔎️ The board node ids the document currently carries, in board order — the readable half of a
/// divergence message, so a failing scenario names WHICH node moved rather than only that two long
/// hex lines differ.
pub fn wires_board_summary(snapshot: &WiresSnapshot) -> String {
    let board = crate::artifacts::wires::wires_working_board(snapshot);
    let ids = |key: &str| board.get(key).and_then(|value| value.as_array()).map(|items| items.iter().filter_map(|item| item.get("id").and_then(|value| value.as_str()).map(str::to_string)).collect::<Vec<_>>()).unwrap_or_default().join(" ");
    format!("nodes[{}] edges[{}]", ids("nodes"), ids("edges"))
}
//#endregion 🌉️ExternalCodecBridge
