# Procedural + CAD plugin serde → ToValue/FromValue conversion

Scope for this pass: `✏️s/🔌️plugins/🌀️procedural` (crate `semio-s-plugin-procedural`) and
`✏️s/🔌️plugins/📐️cad` (crate `semio-s-plugin-cad`). Convert every production
`#[derive(Serialize, Deserialize)]` → `#[derive(ToValue, FromValue)]` and `#[serde(...)]` →
`#[value(...)]`, following the pattern already established elsewhere in this ticket
(`🔍️research/📓️serde-fanout-playbook.md`, `📓️serde-replacement-surface.md`).

## Mechanical conversion — done, both crates

Ran a Python codemod (`/private/tmp/.../scratchpad/serde_to_value.py`, not checked in) over every
`.rs` file in both crate trees, excluding `🧪️oracle/`, `🧪️tests?/`, `🔬️probes/`, `🏭️generator/`,
`🧫️fixtures/`. It:
- Replaced `use serde::{Deserialize, Serialize};` → `use semio_framework_value_derive::{FromValue,
  ToValue};`.
- Rewrote `#[derive(..., Serialize, Deserialize, ...)]` → `#[derive(..., ToValue, FromValue, ...)]`
  (also handled bare-qualified `serde::Serialize`/`serde::Deserialize` inside derive lists →
  `semio_framework_value_derive::ToValue`/`::FromValue`, found in
  `🧩️assembly/🧬️schema/💡️inferences/{🦀️.rs,🧩️wfc-engine/{🆔️ids,🧵️job,🎛️bitset}/🦀️.rs}`).
- Renamed every `#[serde(...)]` → `#[value(...)]` (container and field level). All attrs used were
  in the supported set (`rename_all`, `default`, `tag`+`content`, `skip_serializing_if`,
  `deserialize_with`, `transparent`) — no `flatten`/`with`/`skip`/`untagged` occurrences in either
  crate.

Result: **70/70 procedural files** and **39/39 cad files** with a `derive(Serialize/Deserialize)`
converted; `grep -rl "derive(.*Serialize\|derive(.*Deserialize"` over both trees (excluding test
dirs) now returns nothing.

`✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/Cargo.toml`: added
`semio-framework-value-derive` to `[dependencies]` (cad's Cargo.toml already had it from earlier
work on this ticket). Did **not** touch the `serde`/`serde_json` lines in either Cargo.toml per the
rule ("not until the code compiles without them") — both crates still use `serde_json::Value`
extensively for legitimate wire/JSON manipulation unrelated to the derive conversion.

## Real breakages found and fixed (compiler-shaped, found by static audit — see below)

The blind derive/attr swap is necessary but not sufficient: several call sites fed a
plugin-owned type directly into `serde_json::to_string`/`to_vec`/`to_value`/`from_str`/`from_slice`/
`from_value`, which broke the moment that type's `Serialize`/`Deserialize` derive was removed. Fixed
every one found by static audit, all via the same bridge: `DslValue` has
`impl From<serde_json::Value>`/`impl From<DslValue> for serde_json::Value` (framework-provided,
`🌱️value/🦀️.rs`), so `serde_json::to_string(&serde_json::Value::from(protocol::ToValue::to_value(x)))`
replaces a bare `serde_json::to_string(x)`, and `<T as protocol::FromValue>::from_value(protocol::DslValue::from(json))`
replaces `serde_json::from_str::<T>`/`from_value::<T>`.

**cad**:
- `✏️editor/⚙️engine/🕹️interaction/🦀️.rs`: `CadEngagementContext(HashMap<String, serde_json::Value>)`
  can't derive `#[value(transparent)]` (no `ToValue`/`FromValue` for `HashMap<_, serde_json::Value>`)
  — hand-wrote `ToValue`/`FromValue` routing through `serde_json::Value::Object`/back. Also fixed
  `parsed_specs()`'s `serde_json::from_str::<InteractionSpec>` (production, not test) via the bridge.
- `✏️editor/🎚️config/🦀️.rs`: `deserialize_cad_preview_generation` had the OLD `serde::Deserializer`
  signature (incompatible with `#[value(deserialize_with = ...)]`, which wants
  `fn(DslValue) -> Result<T, ValueError>`). Rewrote to decode via `i64` first, not `i32` — the
  derive's generated `i32::from_value` does an `as i32` cast on overflow (silent wraparound, not an
  error), so decoding straight to `i32` would have silently accepted `i32::MAX + 1` as a wrapped
  negative instead of rejecting it on its own terms.
- `✏️editor/🦀️.rs`: added `json_string_of`/`json_string_to` helpers (DslValue↔JSON-text bridge) and
  rewired `cad_runtime_from_config`/`cad_config_from_runtime`'s `engagement_session_json` round trip,
  the `engagement_preview_operation_json` write, `CadPlayApp::gesture_preview`'s read, and
  `admit_cad_artifact_mutation`'s `serde_json::to_vec(mutation)`. Rewrote
  `preview_generation_cross_surface_domain_round_trips_max_and_rejects_plus_one` (the one test that
  mutated raw JSON text to prove an out-of-range generation is rejected) to build/mutate a
  `DslValue::Object` directly instead of round-tripping `CadConfig` through `serde_json` — avoids a
  cascade of `#[cfg_attr(test, serde)]` additions across `CadCamera`/`CadSunConfig`/
  `CadDislocateOptions`/`CadProjectionDsl` just to keep one test compiling.
- `🧬️schema/📸️snapshot/🦀️.rs`: `enc_json`/`dec_json` (the TEXT `ArtifactDsl` codec's structured-field
  helpers) were bound on `T: Serialize`/`DeserializeOwned` — rebound to `T: ToValue`/`FromValue` via
  the bridge. `encode_cad_snapshot_binary`/`decode_cad_snapshot_binary` (a **separate** binary pack
  codec) had their own direct `serde_json::to_string(&s.nodes)`-style calls, same breakage; gave them
  their own `json_of`/`from_json` (no hex-wrapping, unlike `enc_json`/`dec_json` — the binary format
  length-prefixes instead).
- `🚪️io/🦀️.rs`: `serde_json::to_string(&model)` (×2, `model: SemioModelSnapshot` from the `stdio`
  plugin, itself already converted) and `serde_json::to_value(document)` (`document: CadSnapshot`) →
  bridge.

**procedural**:
- `🧩️assembly/🧬️schema/💡️inferences/🦀️.rs`: `AssemblySolve`/`AssemblyContradiction`'s
  `dep_input(snapshot: &AssemblySnapshot, ...)` used `serde_json::to_vec(snapshot)` (×2) → bridge.
- `🧩️assembly/.../🧩️wfc-engine/🧵️job/🦀️.rs`: `emit_preview`/the restore-checkpoint step both did
  `serde_json::to_vec(&preview)` on `WfcPreview`/`RestorePreview` (both `ToValue`-only or
  `ToValue+FromValue`, never had `Deserialize` for `RestorePreview` even pre-conversion — write-only)
  → bridge.
- `🌀️procedural2d` and `🧊️procedural3d`'s `🚪️io/{📤️export/🧵️serializers,📥️import/🧩️deserializers}/…/🔣️json/🔖️rfc8259/✳️any/🦀️.rs`
  (the stdio-json codec leaves): `serde_json::to_value(snapshot)` / `serde_json::from_value(from.to_serde_value())`
  on `Procedural2dSnapshot`/`Procedural3dSnapshot` → bridge.
- `🧊️procedural3d/✏️editor/🦀️.rs`: `parse_preview_camera_json` (`serde_json::from_value::<Procedural3dPreviewCamera>`),
  `procedural3d_mesh_from_document` (`serde_json::from_value::<Procedural3dSnapshot>`),
  `procedural3d_document_from_mesh` (`serde_json::to_value(default_snapshot())`) → bridge. (Left
  `parse_flow_camera_json`'s `serde_json::from_value::<flow::CameraJson>` alone — `flow::CameraJson`
  still carries **unconditional** `Serialize, Deserialize` alongside `ToValue, FromValue` in the
  framework, confirmed by reading `🌊️flow/📄️artifact/🦀️.rs:177`, genuinely exempt.)
- `🧊️procedural3d/🧬️schema/🦀️.rs` and `🌀️procedural2d/🧬️schema/🦀️.rs`:
  `evaluate_generation_preview`/`generation_fixture_for`'s `serde_json::to_string(fixture)` on
  `fixture: &flow::FlowFixture` → bridge. **Important finding**: `FlowFixture` in the framework has
  `#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]` +
  `#[cfg_attr(test, derive(Serialize, Deserialize))]` — i.e. the framework itself already made this
  type's `serde` **test-only**, so "it's a framework type" is not automatically "exempt, still has
  production serde" — each framework type actually used against raw `serde_json::` calls needs its
  own derive checked, not assumed. `semio_framework_plugin::WorldSunConfig` and
  `semio_framework_plugin::MeshData` (`🔺️mesh-engine/🦀️.rs`) were checked and DO still carry
  unconditional `Serialize`/`Deserialize` — those call sites were left alone.

## Known gaps — do not block `cargo check`, would block `cargo test`

Every one below is inside `#[cfg(test)]`/`mod tests` (verified line-by-line against the file's own
`#[cfg(test)]`/`mod tests` marker), so it does not affect the mandated `cargo check` gate, but a
`cargo test` run would still fail on these until someone either restores a
`#[cfg_attr(test, derive(Serialize, Deserialize))]` twin (cascades into any nested plugin-owned field
types) or rewrites the test to go through `ToValue`/`FromValue` the way
`preview_generation_cross_surface_domain_round_trips_max_and_rejects_plus_one` was rewritten above:

- cad `✏️editor/🦀️.rs`: 3 `mod tests` sites (`render_direct(...)` UI-node round trip ×2,
  `shape`/`building` render round trip) and `at_a_again`/`scene` `CadWorkingScene` round trip.
- cad `🎬️interaction-spec/🦀️.rs`: the asset-corpus `serde_json::from_str::<InteractionSpec>` sweep
  test (production `parsed_specs()` in `⚙️engine/🕹️interaction/🦀️.rs` was already fixed above — this
  is a second, test-only, copy).
- procedural `🧩️assembly/🧬️schema/💡️inferences/🦀️.rs`: `json_round_trips` (`AssemblySnapshot`,
  genuine differential-oracle shape — sanctioned by the ticket's own rule, just needs
  `#[cfg_attr(test, derive(Serialize, Deserialize))]` restored on `AssemblySnapshot` + its `Vec`
  field element types), plus two `AssemblyInferenceCommit` round-trip asserts.
- procedural `🧩️assembly/.../🧩️wfc-engine/{💾️serial,🆔️ids,🎛️bitset}/🦀️.rs`: several
  `serde_json` round-trip unit tests on `SourceModelDoc`/`CheckpointDoc`/`RelationId`/`PatternSet` —
  same shape, same fix.
- procedural `🧩️assembly/.../🧩️wfc-engine/🧵️job/🦀️.rs`: 2 `WfcPreview` `from_slice` reads inside
  `mod tests` (decoding bytes written by the now-bridged `emit_preview`/restore-step encode — the
  test's own decode side still calls raw `serde_json::from_slice::<WfcPreview>`).

None of these were "discovered broken" by a passing build — they were found by reading each call
site's surrounding `#[cfg(test)]`/`mod tests` boundary directly, since the `cargo check` run below
never completed.

## Verification — NOT achieved this session, honestly

`cargo check -p semio-s-plugin-procedural --message-format=short` was launched in the foreground,
auto-backgrounded by the tool at its 120s timeout (not a `run_in_background` choice), and was
**still at 0 output / ~0.4s accumulated CPU time after 15+ minutes wall-clock** when this session
ended. `ps aux` showed 39-63 concurrent `cargo check`/`rustc` processes system-wide throughout (up
to 42 at last check) — this matches the exact contention pattern already documented in
`🔍️research/📓️serde-fanout-playbook.md` ("up to 113 cargo/rustc processes across 12 sessions" /
"queued behind lock contention for ~2 hours"). Per that doc's own guidance and this ticket's
"do not kill and retry" instruction, the process was left running rather than killed.

**A separate `cargo check -p semio-s-plugin-cad` was never started** — procedural's check never
returned, so there was no verified signal to build on before running cad's.

**Before/after error counts: not obtained.** Do not trust any number that isn't from a completed run
started after this session's edits — per the same doc's warning, a long-queued check reflects
the tree at its START time, not completion.

## What whoever picks this up next should do

1. `ps aux | grep "cargo check"` — if concurrency has dropped, run
   `cargo check -p semio-s-plugin-procedural --message-format=short` fresh (foreground, patient,
   shared target dir) then `cargo check -p semio-s-plugin-cad --message-format=short`. Report the
   real error counts.
2. Fix whatever the compiler finds — the mechanical conversion and the ~15 call-site bridges above
   are believed correct by inspection but were never confirmed by an actual build in this session.
3. Once both are clean under `cargo check`, address the "Known gaps" test-only list above before
   calling either crate really done (`cargo test`).
4. Only after both crates compile clean with zero remaining `Serialize`/`Deserialize`/`#[serde(...)]`
   sites (production AND test) should `serde`/`serde_json` be considered for removal from either
   Cargo.toml — and even then `serde_json::Value` usage for wire/JSON manipulation unrelated to the
   derive system is legitimate and explicitly out of scope for this specific conversion task.
