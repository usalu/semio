# Serde Fan-Out — CAD Extensions, Mathematical, Energy

Batch: 4 `📐️cad` extensions (`🏛️aec-building-structure`, `🏢️aec-building`, `📐️spatial-shape`,
`🔥️aec-building-energy`), plus `➗️mathematical` and `🔋️energy`. Companion docs (read first, both
authoritative): `📓️serde-replacement-surface.md`, `📓️serde-fanout-playbook.md`.

## Framework changes landed this pass (benefit every future batch, not just this one)

1. **`CompositeMutationKind<P, Op>` supertrait migrated off serde.**
   `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️component.rs`:
   `MutationLeaf + Clone + serde::Serialize + serde::de::DeserializeOwned` →
   `MutationLeaf + Clone + protocol::value::ToValue + protocol::value::FromValue`. Mirrors the
   already-landed `Mutation`/`MutationDiff` migration. Only 2 implementors repo-wide
   (`aec-building`'s `CreateBuildingStorey`, flow's `DuplicateWidget`) — both updated.
2. **`encode_contributed_wire`/`decode_contributed_wire`** (`🔌️plugin/🦀️component.rs`) now speak
   `protocol::ToValue`/`protocol::FromValue` directly instead of the `to_dsl_value`/`from_dsl_value`
   serde bridge — the wire path a contributed composite mutation's payload travels.
3. Three framework-owned `➕️add-value` test fixtures (`🔌️plugin/🧪️tests/…`,
   `🔌️plugin/🏗️builder/🧪️tests/…`, `🔌️plugin/⚛️reactor/…/🧪️tests/…`) gained `ToValue`/`FromValue`
   derives alongside their existing `serde` derives (their `MutationKind` impl still needs serde —
   that supertrait is untouched, ~200 implementors, out of scope) plus the
   `semio-framework-value-derive` dev-dependency on `semio-framework-plugin`'s `Cargo.toml`.
4. `✏️s/🔌️plugins/🌊️flow`'s `DuplicateWidget` (the flow plugin's own `CompositeMutationKind`
   implementor, NOT part of this batch) got `ToValue, FromValue` added alongside its existing
   `Serialize, Deserialize` so flow keeps compiling under the new bound — flow's own manifest
   cleanup is a separate batch's job, its `serde`/`serde_json` lines were left untouched.
5. Discovered — **not authored this pass, already present** — `pack::json`'s
   `to_json_string<T: ToValue>`/`from_json_str<T: FromValue>`/`dsl_to_json`/`json_to_dsl` bridge
   (`🧰️framework/🔨️modules/🎒️pack/🔤️json/🦀️component.rs` `🔖️DslBridge` region): the
   `serde_json::to_string`/`from_str` analog over `ToValue`/`FromValue`, filling the "not yet
   written" gap #5 the surface doc flagged. A peer landed this concurrently; it is what makes the
   mathematical/energy JSON test-bridge functions convertible at all.

All resolve-checked (`ls -d <manifest-dir>/<relative-path>` on every path dep added below).

## `📐️cad/🧩️extensions/🏛️aec-building-structure` — serde REMOVED, serde_json retained (documented framework blocker)

`serde` deleted from `[dependencies]` entirely. `CadImportProfileManifest`/`CadComputersManifest`
(pure `#[derive(Serialize)]` DTOs, never deserialized) replaced by functions building
`pack::json::Value` directly (`pack = { package = "semio-framework-pack" }` added). Struct
literals → `json::object`/`json::array` calls with the same key names (`camelCase`, matching the
old `#[serde(rename_all)]`), verified by inspection against the original field-by-field.

**`serde_json` cannot be removed**: `ExtensionBundle::contributes_topic(topic, payload:
serde_json::Value)` (`🔌️plugin/🦀️component.rs`) and `TopicContribution` (`🛂️manifest/🦀️component.rs`,
`payload: serde_json::Value`) are hard-typed to `serde_json::Value`, not generic. **26 call sites**
across `✏️s/` (flow extensions, process extensions, all 4 cad extensions in this batch) construct
their topic payload this way — converting the framework signature to `pack::json::Value` would
break all 26 in one shot, most outside this batch and likely mid-edit by other concurrent
sessions. Out of a single 6-manifest batch's blast-radius budget; flagged as its own framework
wave (see "Follow-up" below). `computersJson`'s STRING content is produced via
`json::to_string(&computers_manifest())` (first-party) and nested as a string value inside the one
remaining `serde_json::json!({...})` 5-key wrapper object.

PROVEN BY A PASSING CHECK: no — `cargo check -p semio-s-plugin-cad-aec-building` (started for the
sibling `aec-building` crate, same dependency graph) sat at 0% CPU for 8+ minutes against a
46-cargo-process-saturated shared target dir and had not completed when this session ended.
WRITTEN BUT UNVERIFIED.

## `📐️cad/🧩️extensions/🏢️aec-building` — serde REMOVED entirely, serde_json retained (same blocker)

Same `pack::json` rewrite for `computers_manifest`/`building_import_profile`. Additionally:

- `CreateBuildingStorey` (a real `CompositeMutationKind` payload, contributed onto cad's own
  artifact): `#[derive(Serialize, Deserialize)]` → `#[derive(ToValue, FromValue)]`, made possible
  by framework change (1) above. `#[serde(rename_all = "camelCase")]` → `#[value(rename_all =
  "camelCase")]`.
- `BuildingStructureSummary` (an inference payload, never bound by a serde-requiring trait — only
  hand-serialized): `#[derive(ToValue, FromValue)]`, `serde_json::to_vec(&summary)` →
  `pack_rt::encode_wire_value(&ToValue::to_value(&summary))`
  (`semio_framework_os_kernel::pack_rt`, the SAME native wire codec
  `encode_contributed_wire`/`decode_contributed_wire` use — not JSON at all, no `pack::json`
  needed here). The one test decoding it: `serde_json::from_slice` →
  `pack_rt::decode_wire_value` + `BuildingStructureSummary::from_value`.
- Added `semio-framework-value-derive` + `semio-framework-pack` path deps; `semio-framework-os-kernel`
  was already a dependency (needed for `pack_rt`).

`serde_json` retained ONLY for `contributes_topic` — same documented blocker as above.

**Open question, not resolved**: `CreateBuildingStorey` implements `CompositeMutationKind` but no
`MutationLeaf` impl/derive is visible anywhere in this file or repo-wide for that type, and
`CompositeMutationKind: MutationLeaf + ...` requires one. This may be a PRE-EXISTING gap (the crate
may not have compiled before this batch's edits either) — not introduced by this pass, but not
diagnosed either; the stalled `cargo check` above would have surfaced it. Flagged for the next
verification pass, not silently worked around.

PROVEN BY A PASSING CHECK: no. WRITTEN BUT UNVERIFIED — same stalled `cargo check` as above (this
crate's own check, `-p semio-s-plugin-cad-aec-building`, is the one that hung).

## `📐️cad/🧩️extensions/📐️spatial-shape` — serde REMOVED entirely, serde_json retained (same blocker)

Simplest of the four — no import profiles, no `BTreeMap`. Same `pack::json` rewrite,
same `contributes_topic` blocker, same dependency edit shape. WRITTEN BUT UNVERIFIED.

## `📐️cad/🧩️extensions/🔥️aec-building-energy` — serde REMOVED entirely, serde_json retained (same blocker)

Structurally identical to `aec-building-structure` (import profiles + layer typology). Same
rewrite, same blocker, same dependency edit shape. WRITTEN BUT UNVERIFIED.

## `➗️mathematical` — PARTIAL, additive pass only, `serde`/`serde_json` NOT removed from Cargo.toml

Scope reality check done first: 63 files under `🗿️artifacts/➗️mathematical` reference `serde`;
44 after excluding `🧪️oracle`/`🔬️probes`/`🏭️generator`/`🧫️fixtures`/`🧪️tests` (already-compliant
by directory role). This is a full domain-model conversion (a recursive equation AST, a
`store::ArtifactChild<S>`-composed snapshot, a content-hash production function, JSON
differential-test bridges) — not reachable to full zero-third-party completion in this session
alongside the cad batch above. Converted, in the pilot's own established **additive** pattern
(`ToValue`/`FromValue` added ALONGSIDE existing `Serialize`/`Deserialize`, matching
`📖️playbook`'s own interim state — `serde` is NOT removed from anything yet):

- **15 mutation leaf structs** (`🧬️schema/🧬️mutations/*/🦀️.rs` — `CreateNode`, `DeleteNode`,
  `DeleteNodes`, `MoveNode`, `MovePoint`, `InsertPoint`, `RemovePoint`, `ReplacePoints`,
  `ConnectNodes`, `DisconnectNodes`, `ChangeNodeLabel`, `ChangeGraphDirected`,
  `UpdateGraphAlgorithm`, `ReplaceGraph`, `ChangeCoefficient`): bulk-scripted, all identical
  shape, `Serialize, Deserialize, dsl::MutationLeaf` → `Serialize, Deserialize, ToValueDerive,
  FromValueDerive, dsl::MutationLeaf` (aliased imports to avoid any doubt about macro-vs-trait
  namespace collision, though none actually exists). None of these structs carry `#[serde(...)]`
  field attributes (verified by grep) — no `#[value(...)]` equivalents needed.
- **`🗿️artifacts/➗️mathematical/🦀️component.rs`** (the plugin's own domain types):
  `MathematicalNode`, `MathematicalEdge`, `MathematicalCamera`, `MathematicalGraph`,
  `MathematicalPoint`, `MathematicalGeometry` all gained `ToValue, FromValue` +
  `#[value(rename_all = "camelCase")]`/`#[value(default)]` mirroring their `#[serde(...)]`
  attributes exactly.
- **`🧬️schema/📸️snapshot/🦀️component.rs`**:
  - `EquationNodeLabel` (`pub struct EquationNodeLabel(pub u64)`, a tuple struct — the derive only
    supports named-field structs): **hand-written** `impl ToValue`/`impl FromValue`, delegating to
    `u64`'s own impl.
  - `EquationNode` (named-field struct), `EquationNodeKind` (internally-tagged recursive enum —
    `Add { terms: Vec<EquationNode> }`, `Pow { base: Box<EquationNode>, exponent:
    Box<EquationNode> }`, etc.), `EquationSnapshot`: all three derived, `#[serde(tag = "kind",
    rename_all = "camelCase")]` → `#[value(tag = "kind", rename_all = "camelCase")]` on the enum.
  - `MathematicalSnapshot` itself: **hand-written** `impl ToValue`/`impl FromValue` (fan-out
    playbook trap #3 — its `notation`/`results`/`computed` fields are `store::ArtifactChild<S>`,
    which carries a `local_owner: Option<Arc<dyn Any>>` field the derive cannot route through).
    Bridged per composed field through the pre-existing
    `semio_framework_os_kernel::{to_dsl_value, from_dsl_value}` serde bridge (framework-internal,
    exempt); `equation: EquationSnapshot` goes through `ToValue`/`FromValue` directly.

**NOT converted this pass** (real remaining work, not silently skipped):

- `MathematicalDiff` and the `MathematicalMutation` dispatch enum
  (`🧬️schema/🧬️mutations/🦀️component.rs`) — the dispatch enum's `#[derive(dsl::Mutations)]`
  generates `impl protocol::Mutation`/`SemanticMutation`, both already `ToValue + FromValue`-bound
  by the pilot (not this pass), so converting the enum itself is likely mechanical once `Diff` is
  done — not attempted for lack of time.
- `mathematical_mutation_report_json`/`mathematical_identity_report_json` (the two JSON
  test-bridge functions, same file + `📸️snapshot/🦀️component.rs`) — convertible via the newly
  discovered `pack::json::{to_json_string, from_json_str, dsl_to_json}` bridge (see framework
  change #5 above): `serde_json::from_str::<MathematicalSnapshot>` → `pack::json::from_json_str`,
  `serde_json::to_value(&x)` → `pack::json::dsl_to_json(&x.to_value())`, `serde_json::json!({...})`
  → `pack::json::object([...])`. Not attempted — mechanical but not free, given time.
- `mathematical_scene_id`'s content-hash (`🦀️component.rs`, PRODUCTION code, not test):
  `serde_json::to_string(&(graph, geometry))` — needs the same `to_json_string` swap.
- The `🚪️io/📥️import`/`📤️export`/`📸️snapshot` codec files, the `👥️presence`/`🎚️config`/`🎮️commands`
  editor-command structs (~20 files) — all reported to match the "trivial `DslRecord` command
  struct" mechanical case per the playbook, none attempted.
- One test in `🦀️component.rs` (`scene_owner_fixture_proves_…`) deliberately keeps
  `serde_json::to_value`/`from_value` as a **third-party oracle** comparing our wire format against
  serde's (`"wireOmission"` case) — this is `#[cfg(test)]`, compliant by design (dev-dependency
  oracle), correctly left untouched.

**`Cargo.toml` NOT edited** — `serde`/`serde_json` both still declared; removing them now would
break the ~30 unconverted files above. `semio-framework-value-derive` was NOT added as a
dependency either (needed before any of the above compiles) — this is the single next edit
whoever continues this file needs to make.

WRITTEN BUT UNVERIFIED — no `cargo check -p semio-s-plugin-mathematical` was run (machine
saturated the whole session, see cad section above; running it now would also need the
`semio-framework-value-derive` Cargo.toml edit first, not yet made).

## `🔋️energy` — NOT STARTED

60 files under `✏️s/🔌️plugins/🔋️energy` reference `serde` (outside `🧪️oracle`/`🔬️probes`/
`🏭️generator`/`🧫️fixtures`/`🧪️tests`), comparable scale to mathematical. Not surveyed beyond the
count — no changes made. The mathematical section above is a validated template: expect the same
shape (mutation leaf structs, a composed `#[child(...)]` snapshot needing a hand-written
`ToValue`/`FromValue` impl per trap #3, a recursive or nested domain model, JSON test bridges
convertible via `pack::json::{to_json_string, from_json_str}`).

## Follow-up work this pass surfaced (not done, scoped for a dedicated wave)

1. **`TopicContribution.payload: serde_json::Value` / `ExtensionBundle::contributes_topic`** —
   26 call sites repo-wide (`grep -rl "contributes_topic(" ✏️s`), the only reason ALL FOUR cad
   extensions in this batch cannot reach zero-third-party. Needs its own wave: change the
   signature to `pack::json::Value`, convert every caller's `serde_json::json!(...)` to
   `pack::json::object`/`array` builders. Framework-wide, high blast radius, needs coordination
   with whoever owns the flow/process extension batches.
2. **`➗️mathematical`/`🔋️energy` full conversion** — both need the mechanical leaf/domain-type
   pass finished (mathematical is ~70% there structurally, per the "NOT converted" list above),
   the two JSON test-bridge functions swapped to `pack::json`, then `serde`/`serde_json` deleted
   from `Cargo.toml` only once EVERY file compiles clean — per the playbook's own explicit warning
   against partial-conversion dependency removal.
3. **The possible pre-existing `CreateBuildingStorey`/`MutationLeaf` gap** noted above — needs a
   completed `cargo check` to confirm whether it's real or whether `MutationLeaf` is satisfied
   some way this session's read-through missed.

## Verification — honest

The shared `target/` dir was saturated the entire session (~46 concurrent `cargo`/`rustc`
processes observed via `ps`). One `cargo check -p semio-s-plugin-cad-aec-building
--message-format=short` was started in the foreground, ran the required 46-process check, and sat
at 0.0% CPU (lock-blocked, not building) for 8+ minutes without producing output before this
session ended. No crate in this batch has a passing check. Every conversion above is WRITTEN BUT
UNVERIFIED by inspection and by matching the already-proven pilot pattern (`📖️playbook`,
`semio-framework-replication --lib` 225/226 green) — none is marked PROVEN.

## `cargo check` result — arrived after this doc's first draft

The backgrounded `cargo check -p semio-s-plugin-cad-aec-building --message-format=short` (started
foreground, ran ~9 minutes lock-blocked at 0% CPU, then completed once contention eased) finished
with **95 compile errors, ALL confined to `🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/**`** —
a module this batch never touched. Root cause: `WorkflowMutation`/`RunMutation`/`WorkflowDiff`/
`RunDiff` reference `ToValue`/`FromValue` (via `semio_framework_value_derive`), but the `semio-framework`
package's own `Cargo.toml` doesn't yet depend on `semio-framework-value-derive` —
`error[E0432]: unresolved import semio_framework_value_derive` / `error[E0433]: cannot find
semio_framework_value_derive in the crate root`, cascading into ~90
`error[E0277]: the trait bound ... ToValue/FromValue is not satisfied`. This is another concurrent
agent's in-flight serde-wave work on the `workflow` artifact, not part of this batch — confirmed
by grepping the full error list for every path in this batch (`aec-building*`, `spatial-shape`,
`mathematical`, `📡️spr/🎮️command`, `🔌️plugin/🦀️component.rs`, `🌊️flow`): **zero matches**. Because
`semio-s-plugin-cad-aec-building` depends on `semio-framework` (which depends on `workflow`), the
check never reached this batch's own crate — `semio-framework` itself failed to compile first.

Per the ticket's own instruction ("errors from semio-framework-os-kernel or
semio-framework-replication are another agent's in-flight work... don't fix, don't wait"): recorded,
confirmed not ours, not fixed. **Still WRITTEN BUT UNVERIFIED** for this whole batch — not because
of a defect found in this batch's own edits, but because the dependency graph never got far enough
to check them. Whoever re-runs this once `workflow`'s own fan-out wave adds
`semio-framework-value-derive` to `semio-framework`'s `Cargo.toml` should get a real signal.

---

# ➗️ `mathematical` and 🔋️ `energy` — this session's continuation (picked up where the batch above left off)

Companion reading, in order: `📓️serde-fanout-playbook.md` (mechanical recipe), the "PARTIAL" /
"NOT STARTED" sections above (starting state for this session). Both crates are now **converted in
every production code path except two documented, deliberate cross-plugin blockers each** — full
detail below. Neither crate's `Cargo.toml` had `serde`/`serde_json` removed — see "Why the manifests
still declare serde" at the end.

## Framework change landed this pass: externally-tagged enum support in `#[derive(ToValue, FromValue)]`

`🧰️framework/🔨️modules/🌱️value/✨️derive/🦀️component.rs` (`expand_to_value`/`expand_from_value`, both
functions): added a THIRD enum representation. Previously the derive only supported (a) an
internally-tagged enum (`#[value(tag = "…")]`) or (b) a bare-string all-unit-variant enum. Every
`MathematicalMutation`-shaped dispatch enum that predates the pilot's own tag convention (i.e. one
with NO `#[serde(tag = …)]` at all — serde's own default "externally tagged" representation,
`{"VariantName": <payload>}`) hit the derive's hard error immediately. Added a new match arm to both
functions, guarded by `container.tag.is_none()` (after the existing all-unit special case): a unit
variant still encodes as a bare wire-name string, a single-unnamed-field or named-field variant
becomes a one-key object. Verified by a clean `cargo check -p semio-framework-value-derive`
(exit 0, only 2 pre-existing cosmetic warnings) before continuing the fan-out. Docstring updated to
describe the new mode.

**Collision note**: a concurrent peer was independently extending the SAME file in the same window
(adding `transparent`, `bound`, `serialize_with`/`deserialize_with` support — visible in the
docstring and `ContainerAttrs`/`FieldAttrs` struct fields, though `parse_container_attrs`/
`parse_field_attrs` don't wire those new keys into the match arms yet, so that half looks
mid-flight). Per this ticket's own "never revert a peer" rule, their edits were left untouched;
my own externally-tagged arms sit alongside them and were re-verified present with a direct
`grep -n "container.tag.is_none()"` after their edits landed.

## `➗️mathematical` — every remaining serde-touching file converted (production code); 2 files still carry `serde_json` intentionally

Starting point (per the "PARTIAL" section above): 15 mutation-leaf structs, the plugin's domain
types, and the equation AST already had additive `ToValue`/`FromValue`. ~20 files remained
(dispatch enum, diff, JSON test bridges, io/editor files) — all now converted:

- **`🧬️schema/🧬️mutations/🦀️component.rs`** (`MathematicalMutation`, the dispatch enum): added
  `ToValueDerive, FromValueDerive` — no `#[value(tag = …)]`, relies on the new externally-tagged
  mode, confirmed to match the committed `🦠️mutation/🔣️component.json` fixture files' wire shape
  (`{"ChangeGraphDirected": {"new_directed": true}}`) byte-for-byte before committing to that path
  (checked one fixture by hand). `mathematical_mutation_report_json` (the JSON test bridge) rewritten
  onto `pack::json::{object, from_dsl_value, to_string}` — `MutationMessage`
  (`🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs`, framework-owned) still only has
  `Serialize`, so its two call sites go through the pre-existing `protocol::to_dsl_value` serde
  bridge (framework-internal, exempt) then `pack::json::from_dsl_value`.
- **`🧬️schema/🔺️diff/🦀️component.rs`** (`MathematicalDiff`): hand-written `ToValue`/`FromValue`
  (fan-out playbook trap #3 — `notation`/`results`/`computed` are
  `Option<store::ArtifactChild<S>>`), bridged per field via `to_dsl_value`/`from_dsl_value`;
  `equation`/`cameraX`/`cameraY`/`cameraZoom`/`locale` go through `ToValue`/`FromValue` directly.
- **`🧬️schema/🦀️component.rs`** (`MathematicalArtifact`, the schema family's full-artifact mirror of
  the snapshot): same hand-written pattern as `MathematicalSnapshot`'s own impl (which the earlier
  pass in this doc already wrote) — not previously done.
- **`🧬️schema/📸️snapshot/🦀️component.rs`**: `mathematical_identity_report_json` (the DSL-identity
  test bridge) rewritten onto `pack::json::object`/`from_dsl_value`/`to_string`.
- **`🚪️io/📸️snapshot/📝️text/🦀️component.rs`**: `MathematicalGraphDsl` (hand-rolled `Serialize`/
  `Deserialize` already, since it round-trips through `MathematicalGraph`'s own serde-able shape —
  see its own doc comment) got a MATCHING hand-written `ToValue`/`FromValue`, same round-trip-through-
  `MathematicalGraph` shape (`ToValue::to_value` has no `Result` to propagate a conversion failure
  through, unlike `Serialize::serialize` — falls back to `DslValue::Null`, which cannot happen for
  any value actually produced by `math_graph_to_dsl`). `enc_equation`/`dec_equation` (hex-encoded
  `EquationSnapshot` JSON) swapped `serde_json` → `pack::json::{to_json_string, from_json_str}`.
- **`🚪️io/📸️snapshot/💾️binary/🦀️component.rs`**: same `enc_equation`/`dec_equation` swap for the
  binary codec's own copy of the same helper pair.
- **`🚪️io/🧬️mutations/📝️text/🦀️component.rs`**: `enc_graph`/`dec_graph` (whole-`MathematicalGraph`
  text form used by `replace-graph`'s wire encoding) swapped to `pack::json`.
- **`🚪️io/📤️export/…/json/…/🦀️component.rs`**: fully converted — `serde_json::to_value`/
  `to_vec_pretty` → `pack::json::to_json_string(from).into_bytes()`. `pack::json` has no
  pretty-printer (documented "deliberately not built" in that module) — compact JSON is still
  exact/lossless and nothing round-trips through this hop byte-for-byte, so this is a safe swap, not
  a behavior regression. `IoError` import dropped (now unused, the new path is infallible).
- **`🚪️io/📥️import/…/json/…/🦀️component.rs`**: **NOT converted** — genuine blocker, see below.
- **`🧬️schema/💡️inferences/🦀️component.rs`, `🌱roots/🦀️component.rs`, `🧭topology/🦀️component.rs`**:
  mechanical `ToValueDerive, FromValueDerive` additions (`MathematicalInference`, `MathematicalRoot`,
  `MathematicalTopology`) — none were blocking anything, done for the crate's eventual full
  serde-freedom goal.
- **`✏️editor/🦀️component.rs`** (1785 lines, the largest single file in this batch): the real
  production JSON surface — `geometry_layers_json` (canvas layer JSON), `mathematical_edit_preflight`
  + the `JsonDecode` work phase (both parse `operations_json`, a genuinely stringly-typed JSON-array
  field per the playbook's "JSON text vs DslValue" guidance), `export_media`'s `"result:out"` port
  payload, and `drive_retained` (a TEST helper, converted to compare `protocol::DslValue` directly —
  `DslValue: PartialEq` already, so no JSON round trip is even needed there anymore) all moved to
  `pack::json`. Three test-only `serde_json::to_string(&some_framework_ui_type)` substring checks
  (window-layout/`UiNode` render assertions) converted to `format!("{:?}", …)` instead — `WindowLayout`/
  `UiNode`/`AppDefinition` (`semio-framework-plugin`, framework-owned) have not themselves gained
  `ToValue`, and `Debug` gives the identical "does the render/manifest mention X" check without
  needing `serde_json` for a type this batch doesn't own.
- **All 7 `✏️editor/🎮️commands/*` payload structs** (`SetArtifact`, `SetPoints`, `NodeGraphEdit`,
  `NodeGraphViewport`, `SetAlgorithm`, `SetDirected`, `SetLocale`): mechanical
  `ToValueDerive, FromValueDerive` addition (the "trivial `DslRecord` command struct" case the
  playbook names). `NodeGraphEdit`'s `handle()` — genuine production code parsing `operations_json`
  — converted to `pack::json::Value` (same `as_str`/`as_f64`/`as_array`/`get` API shape as
  `serde_json::Value`, confirmed by the module's own docstring, which explicitly mirrors
  `serde_json::Value`'s ergonomics including `Index`/cross-type `PartialEq`). `set-algorithm`'s test
  module's own `node_graph_edit()` helper and every `serde_json::json!({...})` call site inside it
  converted to `pack::json::object`/`array` builder calls.
- **`✏️editor/👥️presence/🦀️component.rs` + `🧬️schema/🦀️component.rs`**: `MathematicalPresence`
  (implements `protocol::MutationDiff<MathematicalPresence>`) and `MathematicalPresenceMutation`
  (implements `Mutation<MathematicalPresence>`) — both **required** `ToValue`/`FromValue` for the
  trait bound, not just additive polish; mechanical add.
- **`✏️editor/🎚️config/🦀️component.rs` + `🧬️schema/🦀️component.rs`**: `MathematicalConfig`
  (implements `Mutation`'s snapshot side) and `MathematicalConfigMutation` (implements
  `Mutation<MathematicalConfig>`) — same "required by the trait bound" reason.
- **Window/mode test files** (`🎭️modes/✏️edit/🦀️component.rs`,
  `🎭️modes/✏️edit/🪟️windows/{📐️geometry,🕸️graph}/🦀️component.rs`,
  `👁️viewer/🎭️modes/👁️view/🪟️windows/📐️geometry/🦀️component.rs`): the same `Debug`-instead-of-
  `serde_json` swap as above, for the identical reason (framework `UiNode`/`WindowLayout`/`TableView`
  render assertions).
- **Top-level `🦀️component.rs`**: `mathematical_scene_id`'s content-hash function (production code,
  not test) converted `serde_json::to_string(&(graph, geometry))` → `pack::json::to_json_string(&
  (graph.clone(), geometry.clone()))` (the blanket 2-tuple `ToValue` impl covers owned tuples, not
  `(&A, &B)`, hence the clones — cheap, already done elsewhere in that same function). The fixture
  read in `scene_owner_fixture_proves_…` converted to `pack::json::parse`. The SAME test's
  `"wireOmission"` law — which deliberately compares OUR wire format against `serde_json`'s own
  serialization of the SAME snapshot as a third-party oracle — was **left untouched**, exactly
  matching the earlier pass's own documented precedent for that test.

**2 files intentionally still carry `serde_json`** — both are the same documented shape as the cad
batch's `TopicContribution.payload` blocker:

1. **`🚪️io/📥️import/…/json/…/🦀️component.rs`**: `JsonSnapshot::from_value`/`.to_serde_value()`
   (`semio_s_plugin_stdio::artifacts::json`) are hard-typed to `serde_json::Value` — a foreign
   plugin's own API this batch does not own. `🗄️stdio`'s own conversion is its own huge, separately
   deferred wave (per `📓️verified-outcomes.md`'s own "NOT proven" list, ~563 call-site files).
2. The `"wireOmission"` third-party-oracle test in the top-level `🦀️component.rs` (unchanged, by
   design).

## `🔋️energy` — converted from a cold start; 60 files touched, 4 documented exceptions

Scope reality check first: 60 serde-touching files outside `🧪️oracle`/`🔬️probes`/`🏭️generator`/
`🧫️fixtures`/`🧪️tests`. 44 of those are under `🔨️modules/⚡️simulation/⚙️engine/` — the actual BEM
(Building Energy Model) numerical simulation engine (~19k lines total). Critically, **zero**
`#[serde(...)]` field/container attributes exist anywhere in that whole engine tree (verified by
grep before starting) — every struct/enum uses plain field names, no `rename_all`, no `default`, no
`skip_serializing_if`. That made the 44-file engine pass a genuinely mechanical, script-driven
operation rather than 44 individual hand-conversions:

### Engine tree (44 files) — scripted `ToValueDerive, FromValueDerive` addition

A Python pass (kept in this session's scratchpad, not committed) added `ToValueDerive,
FromValueDerive` to every `#[derive(...Serialize...Deserialize...)]` line and inserted the matching
`use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};` import
into each of the 44 files. **First pass had a real bug**, caught and fixed before moving on: the
import-placement heuristic picked the LAST `use` line anywhere in the file as the insertion point,
which for every file landed the import INSIDE that file's own `#[cfg(test)] mod tests { use
super::*; … }` block (wrong scope — the top-level struct/enum derives couldn't see it from there).
Caught by a brace-depth verifier script (not by inspection — all 44 files had this bug
simultaneously), fixed by a second pass that recomputes true top-level (depth-0) placement; a
follow-up pass caught 2 files (`⚖️units`, `🔢️num`) where the naive "first top-level item" heuristic
inserted the `use` line between a `///` doc comment and the item it documents, detaching the doc —
fixed by walking back over the contiguous doc/attribute block. Final state re-verified with the same
depth checker: 0 files with a misplaced import.

**3 tuple structs** the mechanical pass correctly skipped (the derive only supports named-field
structs) got hand-written `ToValue`/`FromValue` delegating to their inner integer, same shape as
`➗️mathematical`'s own `EquationNodeLabel`: `EntityId(pub u32)`, `ScheduleId(pub u32)` (both in
`🔋️model/🦀️component.rs`), `ZoneEquipmentPriority(pub u8)` (`🎛️controls/🦀️component.rs`).

**1 generic struct** the mechanical pass wrongly auto-converted (caught by a follow-up scan for
`struct \w+<`, not by the mechanical pass itself, which has no way to know the derive can't infer
per-type-parameter bounds): `FixedTable<K, V>` (`🔋️model/🦀️component.rs`) — reverted the
auto-added derive, hand-wrote `impl<K: ToValue, V: ToValue> ToValue for FixedTable<K, V>` /
`impl<K: FromValue, V: FromValue> FromValue for FixedTable<K, V>` mirroring its own 4 fields
verbatim (`slots` as a JSON-like array of `null`-or-`[key, value]` entries, `len`/`admitted`/
`faulted` directly) — matches the playbook's own "generic struct… may need a hand-written impl
instead" guidance. **This one matters beyond its own file**: `FixedTable<K, V>` is a field on 4
OTHER auto-converted structs (`📤️output::TimeSeries`'s owner, `🌰️kernel`'s zone/surface state,
`🧮️meters::Meter`'s owner, `🧠️precompute`'s 6 `FixedTable`-keyed fields) — without this hand-written
impl, all 4 of THOSE derives would have failed to compile too.

**Post-mechanical-pass verification, done by grep/AST-walk rather than a compiler** (see
"Verification" below for why a real compile wasn't available): every struct/enum body reachable from
a `ToValueDerive` derive line was scanned for field types the codec doesn't cover —
`HashMap`/`HashSet`/`BTreeSet`/`Arc`/`Rc`/`RefCell`/`Cell`/`Weak` (none found), 3+-element tuples
(none found), and non-local (`::`-qualified) field types (only `crate::model::ScheduleId` and
`crate::units::Unit`, both themselves converted). This is NOT a substitute for `cargo check` and is
flagged as such — see "Verification" below — but it is real, structural evidence the mechanical
pass didn't leave an obviously-broken generic/collection edge case behind.

**1 real serde_json usage inside the engine tree** — `🧪️sim/🦀️component.rs`'s
`p7c1_language_agnostic_law_fixture_matches_reference_parser` test (reads a committed JSON law
fixture, not an oracle comparison) — converted to `pack::json::parse`.

### Artifact-level files (16 files, `🗿️artifacts/🔋️model/**`)

- **`🗿️artifacts/🔋️model/🦀️component.rs`** (the composition root): `semio_value_from_json`/
  `json_from_semio_value`/`energy_structure_from_model`/`energy_model_from_structure` — the
  `crate::model::Model` ↔ `SemioValue` (stdio's generic value tree) bridge — **deliberately NOT**
  converted to `ToValue`/`DslValue`, documented inline with a new docstring paragraph: `DslValue::
  Number` is unconditionally `f64` (confirmed in `semio-framework-value-derive`'s own scalar codec —
  `i32.to_value()` literally emits `DslValue::Number(*self as f64)`), so a `Model -> DslValue` round
  trip cannot recover whether a leaf was an integer or float field — exactly the distinction
  `SemioValue::Int{lexeme}` vs `::Float{lexeme}` exists to carry for a UI table cell or a
  JSON-schema-typed consumer. `serde_json::Value::Number` preserves that distinction because
  `serde_json::to_value` dispatches through each field's own `serialize_i32`/`serialize_f64` call.
  This is a genuine, considered exception, not an oversight — going through `DslValue` here would be
  a silent behavior/fidelity regression, not a safe mechanical swap.
- **`🧬️schema/📸️snapshot/🦀️component.rs`** (`EnergyModelSnapshot`): hand-written `ToValue`/
  `FromValue` (composed `structure`/`zones` children + `Option<store::ArtifactLink>`, same
  `to_dsl_value`/`from_dsl_value` bridge shape as `➗️mathematical`'s `MathematicalSnapshot`). Split
  the file's existing generic `enc_json`/`dec_json` helpers into TWO pairs: the original stays on
  `serde_json` for `referenced_model` only (`store::ArtifactLink` is framework-owned, `Serialize`-
  only, out of scope); a NEW `enc_dsl_json`/`dec_dsl_json` pair on `pack::json`/`ToValue` handles
  `model: crate::model::Model` — safe here (unlike the artifact-root bridge above) because this
  round-trips back into the SAME typed `Model`, and `FromValue` for every integer primitive recovers
  `n as $int_ty` exactly, so no int/float ambiguity survives the round trip. `energy_model_identity_
  report_json` (the DSL-identity test bridge) converted to `pack::json::object`/`from_dsl_value`.
- **`🧬️schema/🦀️component.rs`** (`EnergyModelArtifact`): same hand-written `ToValue`/`FromValue`
  pattern as the snapshot above (its full-artifact mirror).
- **`🧬️schema/🔺️diff/🦀️component.rs`** (`EnergyModelDiff`): hand-written `ToValue`/`FromValue` —
  `artifact: Option<Box<EnergyModelArtifact>>` needs NO bridge (composes straight through the
  blanket `Box`/`Option` impls now that `EnergyModelArtifact` itself has `ToValue`/`FromValue`);
  `structure`/`zones`/`referenced_model` bridged the same way as the snapshot.
- **`🧬️schema/🧬️mutations/🦀️component.rs`** (`EnergyModelMutation`): already had `#[serde(tag =
  "mutation", rename_all = "camelCase")]` (internally-tagged, pre-dates this pass) — mechanical
  `ToValueDerive, FromValueDerive` + matching `#[value(tag = …, rename_all = …)]`, no new derive
  mode needed here. Its own `#[cfg(test)]` structural-correspondence test (reads descriptor/payload-
  schema/catalog JSON files, NOT an oracle comparison) converted to `pack::json::parse`.
- **`🧬️schema/🧬️mutations/♻️replace-model/🦀️.rs`** (`ReplaceModel`, the sole mutation leaf):
  `new_model_json: String` — the "trivial `DslRecord` command struct" case, mechanical derive add.
  `energy_model_mutation_report_json` (the JSON test bridge, same shape as
  `➗️mathematical`'s) fully converted to `pack::json`, including the `MutationMessage` bridge via
  `protocol::to_dsl_value`. The test module's own `demo_model_json` helper converted to
  `pack::json::to_json_string`.
- **`…♻️replace-model/🔺️diff/🦀️component.rs`**: `diff()` parses `payload.new_model_json` into a
  `Model` — production code, converted to `pack::json::from_json_str` (same "round-trips back into
  the same typed struct, so no precision loss" reasoning as the snapshot's `model` field above).
- **`…♻️replace-model/💾️binary/🦀️component.rs`**: `encode_payload`/`decode_payload` (the mutation's
  OWN "direct payload" binary codec, distinct from the DSL-line `OpBinary` in the sibling
  `📝️text/🦀️component.rs`, which stays untouched — it only ever carries `new_model_json` as an
  opaque string, never parses it) — converted to `pack::json::to_json_string`/`from_json_str` over
  the whole `ReplaceModel` struct.
- **`…♻️replace-model/↩️inverse/🦀️component.rs`**: `inverse()` re-serializes the base state's
  `Model` back into `new_model_json` — converted to `pack::json::to_json_string`.
- **`🧬️schema/💡️inferences/🦀️component.rs` + `🗃entries/🦀️component.rs`**: `EnergyModelInference`/
  `EnergyModelEntries` got mechanical `ToValueDerive, FromValueDerive`. `compute_energy_model_entries`
  (production code — computes a real byte-size/entry-count/digest census over the working `Model`)
  converted: `byte_size` now measured over `pack::json::to_json_string(&model)`'s own output,
  `entry_count` now read directly off `model.to_value()`'s `DslValue::Object` length (no JSON
  round-trip needed for that half at all — `DslValue::Object` already IS the count). Both files'
  tests that independently re-derived an "expected" JSON string for comparison
  (`expected_bytes`/`expected_json`) updated to use `pack::json::to_json_string` too — REQUIRED for
  those assertions to still hold, since `pack::json` and `serde_json` do not necessarily produce
  byte-identical output (this is the one place in this whole pass where leaving one side on
  `serde_json` and switching the other would have silently broken a real test).
- **`✏️editor/🦀️component.rs`**: `EnergyModelEditorCommand` (the editor's command channel, mirrors
  `➗️mathematical`'s `MathematicalCommand`) got mechanical `ToValueDerive, FromValueDerive` — an
  externally-tagged, data-carrying enum, exercising the same new derive mode as
  `MathematicalMutation`. The `SetZoneCell` command handler's `new_model_json` re-encode (production
  code) converted to `pack::json::to_json_string`.
- **`🧵️simulation-session/🦀️component.rs`**: `EnergySimulationConfigProjection`/
  `EnergySimulationRequestIdentity` — mechanical derive add (2 plain `Copy` structs).
- **`🚪️io/📤️export/…/json/…/🦀️component.rs`** and **`…📥️import/…/json/…/🦀️component.rs`**: **NOT
  converted** — same `JsonSnapshot`-is-foreign-and-serde_json-typed blocker as `➗️mathematical`'s
  sibling files, documented inline with the same reasoning. `EnergyModelSnapshot` keeps `Serialize`
  specifically so this one bridge still compiles.

## Why the manifests still declare `serde`/`serde_json`

Per the playbook's own explicit warning against partial-conversion dependency removal, and because
BOTH crates have real, structural, documented reasons `serde`/`serde_json` cannot reach zero without
work outside this batch's scope:

1. **The `to_dsl_value`/`from_dsl_value` serde bridge for composed children** (`ArtifactChild<S>`,
   `store::ArtifactLink`) — framework types, not yet converted (playbook trap #6, flagged there as
   its own follow-up wave, not this one's).
2. **Foreign `JsonSnapshot::from_value`/`.to_serde_value()`** (`semio_s_plugin_stdio::artifacts::
   json`) — hard-typed to `serde_json::Value`, `🗄️stdio`'s own huge deferred wave.
3. **`➗️mathematical`'s `"wireOmission"` third-party oracle test** — deliberately serde, by design.
4. **`🔋️energy`'s `Model` ↔ `SemioValue` structure-child bridge** — deliberately serde, for the
   int/float-fidelity reason above; this is a NEW documented exception this session found, not
   inherited from the earlier pass.

Both crates' `Cargo.toml` are otherwise unchanged from the earlier pass in this doc except adding
`semio-framework-value-derive` and `pack` (`semio-framework-pack`) as path dependencies — both
resolve-checked with `ls -d <manifest-dir>/<relative-path>` before use, same depth (5× `../`) for
both crates since both manifests sit at the identical taxonomy depth.

## Verification — honest, and why it's incomplete

**No `cargo check -p semio-s-plugin-mathematical` or `-p semio-s-plugin-energy` produced a clean (or
even a same-crate) result this session.** Two consecutive attempts on `mathematical`:

1. First attempt: `semio-framework-plugin` (a framework crate both plugins depend on) failed with
   864 errors, all `E0277`/`E0599` about `ArtifactApp::{Snapshot,Mutation,Config,...}: Serialize`
   inside `🔌️plugin/🦀️component.rs` — confirmed via `git status`/`stat` to be a concurrent peer's
   in-flight edit (file modified ~6 minutes before the check started), matching
   `📓️verified-outcomes.md`'s own note that `ArtifactApp::Snapshot` migration is "in progress."
2. Second attempt (after re-running the derive extension check first, ~15 minutes later):
   `semio-framework-os-kernel` itself failed with ONE error — `protocol::Edit<Mutation>: protocol::
   ToValue` not satisfied, in `🏪️store/🦀️component.rs:11963` — same file, confirmed modified 26
   seconds before that check started. Another in-flight peer edit.
3. Third attempt (this doc's final one, after the peer's `🏪️store` edit had been quiet for ~16
   minutes): got past both of the above, and past `semio-framework-os-kernel` cleanly, but then
   `semio-s-plugin-stdio` itself — a DEPENDENCY of both `mathematical` and `energy` — failed with
   **2217 errors across ~563 files**, every one an `E0277` "trait bound `<StdioType>: serde::
   {Serialize,Deserialize}` is not satisfied." Grepped the full error list for `➗️mathematical` and
   `🔋️energy` paths: **zero matches** — the failure is entirely confined to `🗄️stdio`'s own files,
   which `📓️verified-outcomes.md`'s "NOT proven" section already named as its own ~563-file
   deferred wave, not touched by this pass. Because `mathematical`/`energy` depend on `stdio`, the
   check never reaches either crate's own files.

**What WAS verified, in lieu of a real compile**: every file this session touched or created (106
files across both plugins plus the framework derive file) parses as syntactically valid Rust —
`rustfmt --edition 2021 --check` on each file individually reported formatting diffs only (import
ordering / line-wrapping from hand-inserted lines, not run through `cargo fmt` afterward — left
as-is rather than risk reformatting a concurrent peer's adjacent in-flight edits in the same shared
files) and **zero** parse errors. This proves syntax validity, not type correctness — it does not
catch a wrong field name, a missing trait bound, or a type mismatch. Additionally: every `ToValueDerive`
addition was cross-checked by static grep/AST-walk for the known-unsupported shapes (generics, tuple
structs, `HashMap`/`Arc`/etc. fields, 3+-tuples) — real defects were found and fixed this way
(`FixedTable<K,V>`, the 3 tuple structs), so this was not a no-op check, but it is explicitly NOT a
substitute for a compiler.

**PROVEN BY A PASSING CHECK**: `semio-framework-value-derive` (the derive extension itself), exit 0.

**WRITTEN BUT UNVERIFIED**: everything else in both `➗️mathematical` and `🔋️energy` described above
— blocked transitively by `🗄️stdio`'s own pre-existing, already-documented, explicitly-deferred
compile failure, confirmed NOT caused by this session's edits (zero mathematical/energy paths in
stdio's 2217-error list). Whoever picks up the `🗄️stdio` wave next should re-run
`cargo check -p semio-s-plugin-mathematical --message-format=short` and
`cargo check -p semio-s-plugin-energy --message-format=short` once stdio compiles — that is the
real, still-outstanding verification step for this doc's whole `mathematical`/`energy` section.
