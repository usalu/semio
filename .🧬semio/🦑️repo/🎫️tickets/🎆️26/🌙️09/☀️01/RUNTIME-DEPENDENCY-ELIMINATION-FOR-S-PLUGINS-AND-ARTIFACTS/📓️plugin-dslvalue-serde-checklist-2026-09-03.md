# Plugin DslValue/serde checklist — 2026-09-03

## Method

Re-derived the ticket's own grep (field-level, not text-proximity):

```
grep -nE '^\s*(pub(\(crate\))?\s+)?[a-z_][a-zA-Z0-9_]*\s*:\s*.*DslValue' <file>
```

against every file the ticket's `awk` scan flagged, then traced each hit's owning `#[derive(...)]`
line and grepped every non-test call site of the type for `serde_json::{from_str,from_value,
from_slice,to_string}` / `Deserialize` to find real consumers. `cargo metadata --no-deps
--format-version 1` exits 0.

## Outcome: zero source edits this pass

Every type the ticket's grep surfaced is either **not actually a target** (no direct `DslValue`
field — a text-proximity false positive in the original scan) or **is a target but is already
correctly converted, or is blocked by a real, identifiable consumer**. None were a "leftover dual
derive" in the sense the ticket warns about (nothing here looks converted while still linking
serde — the retained `Serialize`/`Deserialize` derives are load-bearing).

## Checklist — REMAINING (blocked, consumer identified)

### 🖨️raster — `RasterLayerNode` enum
`✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🦀️.rs:481` —
`#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslEnum, Serialize,
Deserialize)]`, `Adjustment { params: RasterOwnedMap<dsl::DslValue>, .. }` (line 559).

Blocked: `RasterSnapshot` (the container, `layers: Vec<RasterLayerNode>`) needs its own
unconditional `Serialize`/`Deserialize` for `bridge_decode_pair`/`bridge_step` in
`🧬️schema/🧬️mutations/🦀️.rs:368-379` — `serde_json::from_str::<RasterSnapshot>`. Those bridge fns
are called only from `🧪️tests/` fixture leaves, but per that file's own doc the *generated test
host is a separate crate* that links only the plugin crate + `semio-repo-test-host` (no
`serde_json`), so the bridge must be compiled unconditionally (not `#[cfg(test)]`) to exist as
production API the external test-host crate can call. Rust field-derive requires every field type
to satisfy the container's derive, so `RasterLayerNode` (and its siblings `RasterViewportSize`,
`RasterCamera`, `RasterTransform`, `RasterLayerMask`, `RasterImageAsset`, `RasterLayerPatch`) must
stay serde too. Consumer: `bridge_decode_pair`/`bridge_step`,
`✏️s/🔌️plugins/🖨️raster/…/🧬️schema/🧬️mutations/🦀️.rs:368,377`.

### 💡️reasoning/wires — `WiresSnapshot`, `WiresArtifact`, `WiresDiff`, `CreateNode`, `ConnectNodes`
- `WiresSnapshot` (`wires_fixture`/`camera`/`meta: DslValue`) —
  `🧬️schema/📸️snapshot/🦀️.rs:21`, only `Serialize, Deserialize, ArtifactSchema` (no `ToValue`/
  `FromValue` at all — not a dual-derive leftover, just never migrated).
- `WiresArtifact` (same three fields) — `🧬️schema/🦀️.rs:10`.
- `WiresDiff` (`Option<DslValue>` versions) — `🧬️schema/🔺️diff/🦀️.rs:11`.
- `CreateNode { node: DslValue }` — `🧬️schema/🧬️mutations/🌱create-node/🦀️.rs:14`.
- `ConnectNodes { edge, relationship: DslValue }` — `🧬️schema/🧬️mutations/🔗connect-nodes/🦀️.rs:15`.

Blocked, same shape as raster: `encode_wires_snapshot_json`/`decode_wires_snapshot_json`
(`🧬️schema/📸️snapshot/🦀️.rs:47,55`, `serde_json::to_string`/`from_str::<WiresSnapshot>`) are
exported production API consumed only by the cross-crate `mutate-wires-1` fixture test (its own doc
at `🧪️tests/mutate-wires-1/🦀️.rs:35` says so explicitly: "own production code exports the bridges
instead"). Every per-mutation fixture leaf under `🧬️mutations/<slug>/🧪️tests/…` (resize-node,
change-node-kind, set-node-root, edit-node-text, change-node-shape, …) also does
`serde_json::from_str::<WiresDiff>(DIFF)` directly against the committed JSON quintet — same
cross-crate-test-host constraint, so `WiresDiff` can't go `#[cfg(test)]`-only either (the external
test-host crate compiles this crate in its normal, non-test configuration and would not see a
`cfg(test)`-gated derive). `CreateNode`/`ConnectNodes` follow the identical per-leaf
`🦠️mutation/🔣️.json` fixture-decode convention (each mutation dir has a `🧪️tests/<case>/🦠️mutation`
fixture folder). Consumer: the repo-wide generated-test-host fixture harness (committed
`📸️snapshot/{⬅️before,➡️after}` and `🦠️mutation` JSON vectors), not fixable without also rebuilding
that harness to go through `ToValue`/`FromValue`+`pack::json` instead of `serde_json` — out of this
ticket's scope (schema files only).

### 🧩️puzzle/3d — `ObjectKind`, `FixtureObject`, `WorldVolumeProps`, `BrushPlacePayload`
All four hold `scale: Option<dsl::DslValue>` and unconditionally dual-derive
`Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord`
(`🧬️schema/🦀️.rs:467,543,606,691`).

Blocked by **real production** JSON deserialization, not a test bridge:
- `ObjectKind`/`FixtureObject`/`WorldVolumeProps` are reachable through `SceneConfig`
  (`🧬️schema/🦀️.rs:635`, doc: "the exact same shape `Puzzle3dCollision::set_scene`'s JSON payload
  has always deserialized into"), which is `serde_json::from_str::<SceneConfig>`-decoded at
  `✏️editor/🦀️.rs:1064` and `✏️editor/⏳️precompute/🦀️.rs:1052` — both non-test, live command-dispatch
  code paths (wasm engine command ingress), not test fixtures.
- `BrushPlacePayload` is decoded directly at `✏️editor/🎮️commands/🖌️add-brush-object/🦀️.rs:14`
  (`serde_json::from_value::<BrushPlacePayload>`) and in the sibling `🖐️5d` puzzle artifact at
  `✏️editor/🦀️.rs:3889` and `🧠️precompute/🦀️.rs:85,112` (`serde_json::from_str::<BrushPlacePayload>`).

These four are NOT converted because doing so would break live user-facing command dispatch, not
merely a test convenience. Sibling types `BrushPreviewState`/`FillBuildPreview` in the same file
*are* already correctly converted (`ToValue`/`FromValue` unconditional +
`#[cfg_attr(test, derive(Serialize, Deserialize))]`) — they have no such production JSON entry
point, which is exactly the difference that makes them safe and these four not.

## Already correct / not actual targets (no action needed)

- `✏️s/🔌️plugins/🗄️stdio/…/🧿️semio/…/mesh/💡️inferences/📦aabb/🦀️.rs` `SemioAabb` and
  `…/drawing/💡️inferences/🎛flattened-scene/🦀️.rs` `FlattenedNode` — no direct `DslValue` field
  (only `SemioPoint3`/`SemioTransform`/`DrawStyle`, themselves already serde-free). Both already use
  the sanctioned pattern: unconditional `ToValue`/`FromValue`, plus a **hand-written**
  `impl Serialize`/`Deserialize` that bridges *through* `ToValue`/`FromValue` (not
  `serde_json::Value` on the raw struct) — required because `store::InferredField::Value` bounds on
  `Serialize + DeserializeOwned` for its byte-cache codec. Correctly documented, no change.
- `✏️s/🔌️plugins/🗄️stdio/…/🧊️gltf/…/📸️snapshot/🦀️.rs` (`GltfSnapshot`, `GltfSourceForm`, `GltfJson`,
  and ~25 sibling types) — dual-derives unconditionally, but the file's own module doc already
  states the exact consumer: "some other production call site outside this module still serializes
  it — gating it broke the `wasm32-wasip2` component build." None of these hold a direct `DslValue`
  field anyway (only `GltfJson`/nested spec-object fields) — already a documented exception, not a
  new finding.
- `✏️s/🔌️plugins/📖️playbook/…` (`PlaybookArtifact`, `PlaybookDiff`, `PlaybookStringList`) and
  `✏️s/🔌️plugins/🔋️energy/…` (`EnergyModelArtifact`, `EnergyModelDiff`) and
  `🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🔋️model/🦀️.rs` (`EntityId`, `FixedTable`, `Site`, `Zone`,
  `Space`, …) — **false positives** from the ticket's own text-proximity `awk` scan. None of these
  types has a field literally typed `DslValue`/`Option<DslValue>`; the `DslValue` text the scan
  matched lives in a nearby **hand-written** `impl ToValue`/`impl FromValue` block (composed-child
  bridging via `to_dsl_value`/`from_dsl_value`, documented inline as "Hand-written, not derived" —
  the ArtifactSchema-level composition pattern every `*Artifact`/`*Diff` type in this repo already
  uses) or in an unrelated tuple-struct hand-impl (`EntityId`, `ScheduleId`). Nothing to convert.

## Exit-criterion distance

`🧰️framework/🔨️modules/🌱️value/🦀️.rs:281,288`'s `impl Serialize for DslValue` /
`impl<'de> Deserialize<'de> for DslValue` cannot be removed yet. Every remaining serde-deriving
`DslValue`-holding type found in `✏️s/🔌️plugins/**` this pass is blocked by a real, already-cited
consumer (repo-wide generated-test-host JSON fixture harness for wires/raster; live command-dispatch
JSON decode for puzzle3d; the `wasm32-wasip2` build for gltf). Clearing the gate fully requires
either (a) migrating the generated-test-host fixture harness and the puzzle3d/gltf production JSON
entry points off `serde_json` onto `ToValue`/`FromValue` + `pack::json`, or (b) accepting those as
permanent, documented exceptions. Both are out of this ticket's stated scope (schema files only).

## Verification

- Made zero source edits this session — every candidate was either a false positive or genuinely
  blocked; nothing was safe to convert without breaking a real consumer.
- `cargo metadata --no-deps --format-version 1` → exit 0.
- `cargo check -p semio-framework --message-format short` → **54 errors**, NOT the "currently 0"
  baseline the ticket assumed. All 54 are `serde::Deserialize` trait-bound failures on
  `WorkflowParameter`/`WorkflowNode`/`WorkflowEdge`/`RunOutputArtifact`/`RunNodeStatus`/
  `PortFingerprint` in `🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/**` — `git status` shows 7
  uncommitted files under that exact path right now, confirming this is a **live peer migration**
  (someone else mid-flight converting workflow types off serde), not caused by this session (zero
  edits made) and not in this ticket's plugin list. Reported, not chased, per the ticket's own
  "measure baseline, don't chase peer churn" instruction.
- No plugin `cargo check` runs were needed since no plugin source was touched.
