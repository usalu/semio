# Playbook config/presence closure — 2026-09-03 (session 2)

Zero `cargo` commands run, zero sub-agents spawned. Verified by re-reading every edited region on
disk and re-running the accurate counter (`python3 /tmp/prodserde.py <plugin> 40`).

## Starting point (accurate counter, this session)

- `✏️s/🔌️plugins/📐️cad`: **7**
- `✏️s/🔌️plugins/📖️playbook`: **13**

This matches the prior same-day report `📓️cad-playbook-serde-elimination-2026-09-03.md`'s own
"single largest remaining structured chunk" callout: `PlaybookConfig`/`PlaybookConfigMutation`/
`PlaybookPresence*` were left unconverted there because `dsl::DslOps`/`dsl::DslArtifact` coexisting
with `ToValue`/`FromValue` hadn't been characterized as safe yet.

## Characterization: dsl::DslArtifact/DslOps + ToValue/FromValue — CONFIRMED SAFE

Found the exact working precedent already shipping in `🖨️raster` (one of the 12 fully
manifest-clean plugins): `RasterConfig`/`RasterConfigMutation`/`RasterPresence*`
(`✏️editor/🎚️config/🦀️.rs`, `👥️presence/🦀️.rs`, their `🧬️schema` leaves) all derive
`dsl::ToValue, dsl::FromValue` **alongside** `dsl::DslArtifact`/`dsl::DslOps`, with
`#[value(rename_all = "camelCase", default)]` at struct/enum level and `#[value(default = "fn")]`
per-field — zero serde. Mirrored this pattern exactly onto playbook's equivalents. Also confirmed the
framework's `ArtifactEditor::Presence` trait bound (`🔌️plugin/🦀️.rs:11161` etc.) requires
`protocol::ToValue + protocol::FromValue`, NOT `Serialize` — so `PlaybookPresence` deriving only
`Serialize/Deserialize` (no ToValue/FromValue, no hand-written impl) was a **latent non-compiling
trait-bound gap**, not just a serde cleanup.

## Files fixed (all within `📖️playbook`, no `Cargo.toml` touched)

- `✏️editor/🎚️config/🦀️.rs`: `PlaybookConfig` + `PlaybookConfigMutation` derives
  `Serialize, Deserialize` → `ToValue, FromValue` (3 refs → 0); `#[serde(...)]` → `#[value(...)]`.
- `✏️editor/🎚️config/🧬️schema/🦀️.rs`: same swap on the schema-leaf mirror (2 → 0).
- `✏️editor/👥️presence/🦀️.rs`: `PlaybookPresence` + `PlaybookPresenceMutation` same swap (3 → 0).
- `✏️editor/👥️presence/🧬️schema/🦀️.rs`: schema-leaf mirror (2 → 0).
- `✏️editor/🦀️.rs`:
  - `playbook_bounded_serialized_bytes<T: serde::Serialize>` (used `serde_json::to_writer` into a
    byte-counting `Write` sink) → `<T: protocol::ToValue>` using `protocol::json::to_json_string(v).len()`
    (1 → 0). **Trade-off noted in the code comment**: `pack::json` has no streaming-writer analog of
    `serde_json::to_writer`, so the 32KiB `PLAYBOOK_STORE_MAXIMUM_BYTES` cap is now checked after full
    materialization, not incrementally. Small bound, accepted.
  - Both `where … serde::Serialize` bounds on `PlaybookOneItemPreparationFactory`/
    `PlaybookOneItemPreparation` impls → `protocol::ToValue` (their only instantiation is
    `P=PlaybookConfig, M=PlaybookConfigMutation`, now ToValue-only).
- `🚪️io/📤️export/…/🔣️json/🔖️rfc8259/✳️any/🦀️.rs`: `serde_json::Value::from(&protocol::ToValue::to_value(snapshot))`
  → `protocol::json::from_dsl_value(&protocol::ToValue::to_value(snapshot))`, still handed to
  stdio's `JsonSnapshot::from_value(impl Into<JsonValue>)`, which already has
  `impl From<pack::JsonValue> for JsonValue` (1 → 0). Doc comment corrected (no longer claims a
  `serde_json` crossing).
- `🧩️extensions/🌀️procedural/🦀️.rs`: doc comment + `render_params_body` claimed
  `playbook::visible_blocks` (framework `📖️playbook/🦀️.rs`, reached via `flow::playbook`) is
  "hard-typed to `serde_json::Map<String, serde_json::Value>`" — **stale**, same pattern as the
  ticket's cad `MeshData` example. Its real signature is `&PlaybookValues` =
  `&HashMap<String, DslValue>` (confirmed at `🧰️framework/…/📖️playbook/🦀️.rs:228`). Replaced the
  serde round-trip with a direct per-field `json_to_dsl_value` map build (1 → 0).

## Result

- `📖️playbook`: **13 → 0** (fully manifest-clean, pending `Cargo.toml` move by the dev).
- `📐️cad`: **7 → 7**, unchanged — re-verified all 7 are genuine, already-minimal framework
  boundaries (see below), matching the prior session's own conclusion.

## cad: 7 refs re-verified as genuine framework boundaries, left untouched

- `🚪️io/🦀️.rs:718,720` (`cad_document_from_mesh`): return type is dictated by the framework's own
  `MeshDwgDocumentImporter = fn(&MeshData) -> Result<Value, String>` type alias
  (`🔌️plugin/🦀️.rs:3525`), where `Value` resolves to `serde_json::Value` in that module's own scope
  (`use serde_json::Value;` at line 288, same enclosing module). Framework-owned function-pointer
  type, not this plugin's boundary to move (`🧰️framework/**` is DO NOT TOUCH).
- `✏️editor/🎮️commands/🌞️sun/🦀️.rs:42,61,80` and `🎮️commands/🎥️camera/🦀️.rs:67,111`:
  `apply_world3d_sun_action`/`apply_world3d_projection_action`/`world3d_projection_action_moves_pose`
  (framework `🔌️plugin/🦀️.rs`'s `world3d_host` module) all take `Option<&Value>` where that module
  imports `serde_json::{json, Value}` (line 36279) — confirmed by reading the actual function
  signatures at lines 36371/36654/36711. Each cad call site already bridges via a single
  `serde_json::Value::from(&DslValue::object(...))` right at the call, matching the ticket's own
  "bridged once, at this exact boundary" guidance.

None of the 7 involve `serde_json::Value::from(...)`/`From<&DslValue>` used to paper over a wrong
destination type — every one is a hand-verified, unavoidable crossing into a framework-owned
`serde_json`-typed signature.

## Hunks NOT written by me (flagging per instructions — concurrent session in the same files)

- `✏️editor/🦀️.rs` line ~437 (`serde_json::from_str` → `protocol::json::from_json_str` for
  `PlaybookChapterPayload`) and line ~752 (`serde_json::to_string(&payload).unwrap()` →
  `protocol::json::to_json_string(&payload)` in a test helper).
- `🧩️extensions/🌀️procedural/🦀️.rs`: an unrelated comment rename, `Procedural3dDocument` →
  `Generation3dDocument`.
- The export json leaf's baseline (before my edit) already showed
  `serde_json::Value::from(&protocol::ToValue::to_value(snapshot))` rather than the original
  `serde_json::to_value(snapshot).map_err(...)` recorded in the prior session's report — another
  concurrent/earlier pass already landed partway through this exact fix before I finished it.

None of these were reverted; they were already present on disk when each file was read for editing
and are consistent with (not conflicting with) the changes made here.
