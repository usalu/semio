# Shooting / Animate / Flow — serde → value-derive conversion (2026-09-02)

Wave scope: convert production `#[derive(Serialize, Deserialize)]` / `#[serde(...)]` to
`#[derive(ToValue, FromValue)]` / `#[value(...)]` in three plugins: `semio-s-plugin-shooting`
(`✏️s/🔌️plugins/🎥️shooting`), `semio-s-plugin-animate` (`✏️s/🔌️plugins/🎞️animate`, artifact
name `present`), `semio-s-plugin-flow` (`✏️s/🔌️plugins/🌊️flow`, 9 extension sub-crates confirmed
to have zero serde usage outside test dirs, left untouched). Done via 3 parallel foreground
subagents, one per plugin, plus manual follow-up verification/fixes by the coordinating agent.

## Setup pattern used in all three

Each plugin's crate root (`📦️packages/🦀️rust/🦀️.rs`) already aliases
`semio_framework_os_kernel` as `dsl`/`store`/`protocol` (+ `vcs` in animate). Added
`extern crate semio_framework_value_derive as value_derive;` to each crate root (flow's
Cargo.toml already had the dependency; shooting/animate needed
`semio-framework-value-derive = { path = "../../../../../🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust", package = "semio-framework-value-derive" }`
added to `[dependencies]`). Production JSON/PDF/PPTX/CSV io bridges that used to call
`serde_json::to_value(snapshot)`/`from_value` directly on the plugin's own domain types were
rewritten to go through the value system instead:
`let value: serde_json::Value = dsl::ToValue::to_value(x).into();` /
`let v: dsl::DslValue = json_value.into(); dsl::FromValue::from_value(v)`, relying on the
framework's own hand-written `DslValue <-> serde_json::Value` `From` bridges — no new runtime
dependency introduced.

## Per-plugin results

**shooting** (`semio-s-plugin-shooting`): 38 files converted with the `cfg_attr(test, serde(...))`
oracle pattern kept (the whole schema/mutations/diff graph — ~30 mutation-leaf tests round-trip
`ShootingSnapshot`/`ShootingMutation`/`ShootingDiff` through `serde_json` as a differential
oracle), 4 files converted plain (config/presence, no oracle test touches them), 6 io-bridge call
sites fixed (4 requested + 2 more found: the op-log text/binary codec and an import/export
command handler). No `with`/`skip` fields hit. `cargo check -p semio-s-plugin-shooting` (run by
the coordinator, real output): stops at 6 pre-existing `error[E0277]: MeshData: serde::Serialize`
in the **upstream** `semio-s-plugin-stdio` dependency (gltf/semio-kit viewer files, nothing shooting
touches) — confirmed unrelated to this conversion, same root cause as the stdio-side serde-loss
already tracked elsewhere in this ticket. shooting's own body was never reached by that run because
cargo halts at the first broken crate in the dependency graph.

**animate** (`semio-s-plugin-animate`): full derive/attribute conversion across the present
schema/mutations/diff/inference graph, editor config/presence, engine (scene/video/text/quality
config), tile-editor windows. 12 io-bridge call sites fixed (8 requested + 4 more: mutation
json codec, deck/scene/sections json writers in the engine, video export command). One
`cfg_attr(test, serde(...))` oracle kept: `PresentConfigMutation` (a real `serde_json` round-trip
test exists). Coordinator's own `cargo check -p semio-s-plugin-animate` (real output, 93
`error[...]` lines) found and the coordinator fixed **one genuine regression** the subagent missed:
`animate_present_config_edit_bytes` (editor/🦀️.rs) called `serde_json::to_writer` directly on
`protocol::Edit<PresentConfigMutation>` in **production** (a real byte-budget guard, not a test) —
the subagent only saw the test-oracle usage and cfg-gated the derive, breaking this second,
unconditional call site. Fixed by mirroring the identical, already-compiling pattern in the
`fem` plugin's `fem3d_config_edit_bytes`: replaced with
`counter.write_all(dsl::json::to_json_string(edit).as_bytes())` (first-party
`pack::json::to_json_string<T: ToValue>`, re-exported as `dsl::json::to_json_string`). All other
errors in that 93-line run were verified unrelated to serde: `error[E0053]`/found-future (the
repo-wide async-convention-debt wave), `Label`/`String` mismatches and missing `.id()`/`.build()`
builder methods (a concurrent UI-contract API rewrite touching the whole editor/panels tree),
`E0432` missing `mutations::*::mutation` submodules (pre-existing/concurrent module-structure
issue, not something a derive-attribute change could cause), and two more
`SemioPresentationSnapshot`/`SemioAnimationSnapshot: Serialize` errors — same upstream stdio
serde-loss as shooting's `MeshData`, not animate's own types.

**flow** (`semio-s-plugin-flow`): full conversion of the flow schema/mutations/diff/inference
graph (10 mutation-payload leaves), editor config/presence + their schema-variant twins, node-graph
command handlers, and the retained-bounded-serialization generic bound
(`flow_bounded_serialized_bytes<T: serde::Serialize>` → `<T: dsl::ToValue>`). 4 io-bridge call
sites (json/csv import+export) converted to the bridge pattern. Oracle exception kept on
`FlowSnapshot`, `FlowMutation`, `FlowDiff`, `FlowStringList`, `FlowArtifact`, `FlowWorkingScene`
(Serialize-only) and all 10 mutation leaves — every mutation-verb test fixture round-trips these
through `serde_json` against committed JSON to prove canonical encoding. All 9 extension crates
(`brep`/`dictionary`/`bim`/`logic`/`primitive`/`math`/`list`/`draw`/`text`) reconfirmed to need no
changes. The subagent's own `cargo check -p semio-s-plugin-flow` runs (~15 attempts against a
heavily contended shared build lock) consistently showed **zero errors under any `✏️s/` path**
before failing further upstream on `semio-framework-plugin`
(`error[E0277]: MeshData: serde::Serialize`, the same stdio-family serde-loss) — not re-confirmed
end-to-end by the coordinator this session due to time; recommend a follow-up
`cargo check -p semio-s-plugin-flow --message-format=short` once the shared blocker clears.

## Cross-cutting note

All three plugins currently sit downstream of the same unresolved blocker: several
`semio-s-plugin-stdio` types (`MeshData`, `SemioImageSnapshot`, `SemioPresentationSnapshot`,
`SemioAnimationSnapshot`, `PdfSnapshot`, …) have lost their `Serialize`/`Deserialize` derives from
a concurrent stdio-side wave of this same ticket without (yet) gaining `ToValue`/`FromValue` or a
test-only cfg_attr fallback, and several plugins' own bodies still call `serde_json` on them
directly (production, not test). This blocks a clean `cargo check` for shooting/animate/flow (and
likely every other plugin that touches these stdio artifact types) regardless of anything done in
this wave. Not fixed here — out of scope (stdio is a different crate) — but flagged since it is
the actual remaining blocker for a green build across this ticket's whole plugin fleet.

## Files touched (high-level; full per-file list is in each subagent's transcript)

- `✏️s/🔌️plugins/🎥️shooting/📦️packages/🦀️rust/Cargo.toml`, `.../🦀️.rs` (crate root)
- `✏️s/🔌️plugins/🎞️animate/📦️packages/🦀️rust/Cargo.toml`, `.../🦀️.rs` (crate root),
  `.../🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` (coordinator's
  `animate_present_config_edit_bytes` fix)
- `✏️s/🔌️plugins/🌊️flow/📦️packages/🦀️rust/🦀️.rs` (crate root; Cargo.toml already had the dep)
- ~38 (shooting) + ~35 (animate) + ~29 (flow) domain/schema/mutation/io files across the three
  plugins' `🗿️artifacts/` trees.
