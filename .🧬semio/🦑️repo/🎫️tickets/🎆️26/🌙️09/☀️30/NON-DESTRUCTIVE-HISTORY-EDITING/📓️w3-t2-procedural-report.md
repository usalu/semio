# 📓️ W3-T2-PROCEDURAL Report: Procedural Gestures as Tool Machines (generation 2d / 3d)

Ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING`. Scope: `✏️s/🔌️plugins/🌀️procedural/**` (generation2d + generation3d
artifacts, incl. generation3d's gumball consumer of `World3dHost`), the procedural laws in `✏️s/🧑‍💻dev/🧩️composition/🧪️tests/🧊️generation3d/**`
and the procedural composition `🌎️hub/🧩️compositions/🌀️procedural/**`. Brief: `🧭️plan.md` "W3-T brief" + session-2 roster row
S2-PROCEDURAL. Contract: `📋️design.md` §5, §7, §10, §11, §13.

Aliases: `G3` = `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any`,
`G2` = `…/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any`, `DEV` = `✏️s/🧑‍💻dev/🧩️composition/🧪️tests/🧊️generation3d`.

## Session 2 — 2026-10-01

Successor executor S2-PROCEDURAL. The session-1 executor (launched 03:19) was cut ~08:00 without a report; its edits were
auto-committed in `4e36b2b5012` (11:16). Everything below "Reconstruction" is what that commit + `🧪️w3-t2-procedural-leaves.py`
+ `🗑️generated/w3-t2-procedural/check-g3-1.txt` show; everything after is session-2 work.


### S2.1 Census (audit §5.1/§5.2/§5.5, procedural rows) and verdict

| # | Gesture / control | Host → verb | Before (HEAD `fa3fdf76eb7`) | Now |
|---|---|---|---|---|
| P1 | gen3d gumball translate/rotate/scale (World3d; live = mesh components, `gumballLiveDispatch`) | `translateSelection` / `rotateSelection` / `scaleSelection` (+ `phase`, `reason`, `windowId`) | scratch host + `commit_host_snapshot` (absolute `update-widget` of the operator params), coalesce key `gumball-<id>-<mode>-<components>` | gumball ToolMachine (`🧭️transforms`): relative `drag-`/`rotate-`/`scale-transforms` leaf + splice rows, ONE `ToolTransaction`, `Emit::commit_transaction`; live stream/commit/abort in the instance's per-window `GumballGestures`; host events abort |
| P2 | gen3d node drag (both node-graph hosts) | `nodeGraphEdit {move}` | absolute `{nodeId,x,y}` → `move-widget` scratch diff | node-graph gesture record `{gestureId,nodeIds,dx,dy}` → relative `move-nodes` through `node_drag_commit` (ONE transaction) |
| P3 | gen2d node drag | `nodeGraphEdit {move}` | same as P2 | same as P2 (gen2d `move-nodes`) |
| P4 | gen2d palette/agent drop | `moveMediaNode {nodeId,x,y}` | absolute `move-widget` scratch diff, no transaction | relative `move-nodes` from the base position through the node-drag machine (S2) |
| P5 | inline graph slider (React NodeGraph lane) | `nodeGraphEdit {setSlider}` + top-level `gesture`/`commit`/`abort` | whole-snapshot scratch diff, key `graph-slider:<gesture>` | ABSOLUTE `change-slider-value`; framework ScrubMachine owns the press (ticks provisional, release = ONE transaction) |
| P6 | gen3d inspector slider value (number field, continuous lane) | `patchFlowWidgets {field:value}` | scratch diff, key `widget-field:<gesture>` | absolute `change-slider-value` per moving slider; ScrubMachine |
| P7 | generation form values (2d + 3d; slider/number continuous, text commit on blur) | `updateGenerationValues` | static key `generation-values` (merged across questions) | absolute `change-generation-value` (existing field leaf); ScrubMachine; text inputs `commit("blur")` + `Trigger::Commit` |
| P8 | gen3d operator input field (inspector, `commit: blur`) | `setWidgetInput` | scratch diff, key `widget-input:<id>:<ch>:<gesture>` | one-shot edit per commit (whole-operator `update-widget`), gesture arg deleted |
| P9 | gen3d mesh edits (extrude/inset/…, knife), reorganize | `editMeshSelection`, `knifeMeshSelection`, `reorganize` | one-shot scratch splice | unchanged — O\* (one edit; leaf is the splice, not the intent). Open item §S2.6 |
| P10 | `params:in` media port | `import_media` | absolute `update-widget` | absolute `change-slider-value` per changed slider |

Host side: React `🕸️NodeGraph` (flow executor) sends P2/P3 records and the P5 press at top level; wgpu `EngineCanvas` sends the
same record; React `World3dHost` streams P1 only where the selection record asks `gumballLiveDispatch` (mesh components), every
other gumball is ONE one-shot delta on release in both React and wgpu (`📓️w3-t-spatial-report.md` §2). Continuous controls P6/P7
ride the framework lanes owned by S2-CONTROLS (React Interpreter + wgpu presses).

### S2.2 Reconstruction of session 1 (commit `4e36b2b5012`, last source edit 07:48, last cargo attempt 07:51)

Landed by the predecessor (verified by reading every file; nothing of it had been compiled — its only check,
`🗑️generated/w3-t2-procedural/check-g3-1.txt`, died on a peer's os-kernel error):

- Leaves (schema-first via `🧪️w3-t2-procedural-leaves.py`, re-runnable byte for byte): gen3d `🎚️change-slider-value` (tag 14),
  `✋️drag-transforms` (15), `🔃️rotate-transforms` (16, `x-semio-invariant: axis-nonzero`), `📏️scale-transforms` (17),
  `🚚️move-nodes` (18); gen2d `🎚️change-slider-value` (14), `🚚️move-nodes` (15). Each: descriptor `🔣️.json`, payload schema
  with full `x-semio-ui` (en/de labels, widget, step, precision, `role: target` + `ref {kind: widget, domain: graph}`; angle
  as dial rad→° ), Rust leaf + `🔺️diff` + `↩️inverse` (exact absolute inverses), TS payload twin with `parse<Type>()`
  (gen3d also diff/inverse twins), binary tags in `💾️binary/📡️.protocol.semio` + codec, aggregate variants + `KINDS`,
  oracles catalog rows, committed wire witnesses `🧫️fixtures/🧬️mutations/<leaf>/🧾️wire-witness/🦠️mutation/🔣️.json`.
  Outcome codes only from the 9-code vocabulary (`target-missing`, `target-mismatch`, `partial`, `no-op`, `invariant`).
- gen3d gumball ToolMachine (`G3/✏️editor/🎮️commands/🧭️transforms/🦀️.rs` region `🛠️GumballTool`): `statechart! gumball_tool`
  (`idle` —Once/Finish[moves]→ commit, —Stream[moves]→ `streaming`; `streaming` —Stream→ upsert net, —Finish→ commit net,
  —Cancel→ abort), `GumballMotion` algebra, `GumballGestures` (per-window open gesture on the instance owner, `baseMoved` /
  `captureLost` aborts, `provisional()` overlay rows), `gumball_once` for the marks-free entry. Old absolute `apply`,
  `apply_selected`, `apply_components`, gumball params-json helpers and coalesce keys deleted.
- gen3d editor (`G3/✏️editor/🦀️.rs`): `GumballGestures` on `Generation3dInstanceOperationOwner`; retained reduce routes the
  three gumball verbs through it; `host_event` maps blur/captureLost/utility switch/retiring/time-travel freeze/baseMoved to
  a typed `translateSelection{phase: abort}`; flow-eval ticks evaluate `generation3d_provisional_snapshot` (framework
  provisional leaves from `ArtifactOwnedToolJobContext::provisional()` + open gumball rows, design F-7), `pending_effects`
  folds `provisional_generation()` into the preview digest; describe texts updated (en/de).
- gen3d + gen2d `nodeGraphEdit`: slider rows → `slider_leaf` (absolute), move records → `move-nodes` via `node_drag_commit`,
  structural rows keep the host diff; `gesture_coalesce_key`, `patch_coalesce_key`, static `generation-values` key and every
  `gesture` field deleted (`PatchFlowWidgets`, `SetWidgetInput`, `UpdateGenerationValues`).
- Semantic wire corpus (`G3/🧬️schema/🧬️mutations/💾️binary/🧫️fixtures/🧬️semantic-wire` + schema + TS Ajv twin + Rust
  `semantic_wire_vectors`), gesture-leaf laws `G3/🧬️schema/🧬️mutations/🧪️tests/🧪️gesture-leaves/🦀️.rs`.

Half-finished / broken on arrival (repaired in S2.3): the `✏️node-graph-edit` unit tests of both artifacts still exercised the
deleted `gesture_coalesce_key` and the absolute `move` row; the 🎚️slider-gesture fixture still described coalesce keys; the
procedural laws in `DEV` (moved by a peer to `✏️s/🧑‍💻dev/🧩️composition` during this session) still used the deleted
`TranslateSelection{..}` shape, `gesture` fields, gumball params helpers, absolute `move` rows and the sub-operation
`gesture`; the generate-mode interaction table still declared `move {nodeId,x,y}`; gen2d `moveMediaNode` was still absolute;
tool ids used the local tag `procedural3d-play` instead of the editor surface id.

### S2.3 Changes this session (repair first, then completion)

Repairs of the session-1 state (rule 21):
- `G3/✏️editor/🎮️commands/✏️node-graph-edit/🧪️tests/🔬️unit/🦀️.rs` rewritten: slider-press laws over the v2 table (ticks
  provisional, one press = one edit = one transaction of `change-slider-value`, second press = second transaction, cancel =
  zero trace, one-shot = one plain edit, unchanged value = no edit), row labels en/de, narrow tick scope, node-drag record laws
  (one transaction of `move-nodes`, positions = base + offset, zero-offset / stranger id = zero trace, two drags = two
  transactions). Drives `handle_action("nodeGraphEdit", {operations, gesture, commit, abort})` exactly like the host lane.
- `G3/🧫️fixtures/🎚️slider-gesture.json` → schema `s.procedural.generation3d.slider-gesture/v2` (`presses` with top-level
  `gesture`/`commit`/`abort`, expected `committed` values, `edits`, `transactions`; `uiScope` and `liveEvaluations` kept).
- `G2/✏️editor/🎮️commands/✏️node-graph-edit/🧪️tests/🔬️unit/🦀️.rs` rewritten the same way (wire round trip kept).
- `G3/🧫️fixtures/🎛️generate-mode-interactions.json` + its TS twin `G3/✏️editor/🧪️tests/🔬️generate-interactions/🟦️.ts`:
  the `move` row requires the gesture record (`gestureId`, `nodeIds`, `dx`, `dy`).
- `DEV` (procedural laws in the peer-moved composition crate): `🦀️.rs` imports of the deleted gumball helpers removed and
  replaced by local `gumball_param_vector` / `gumball_param_number`; every `Translate/Rotate/ScaleSelection` literal gains
  `phase/reason/window_id: None`; `gesture` fields removed (`PatchFlowWidgets`, `UpdateGenerationValues`, `SetWidgetInput`);
  `node_move_operations_json` writes the gesture record (convergence law); generate-interactions and node-graph-edit `move`
  laws offset from the base position; the v1 coalescing structs replaced by a v2 commit-count law; slider-values presses go
  through the scrub door (`context::act_with_view`, new); patch-flow-widgets law restated for the absolute leaf.

Completion:
- Tool ids are the editor surface ids: `GENERATION3D_EDITOR_APP_ID = "s.procedural.generation3d@1/*#editor"`,
  `GENERATION2D_EDITOR_APP_ID = "s.procedural.generation2d@1/*#editor"` (gumball, node drag, palette drop).
- gen2d `moveMediaNode` (`G2/✏️editor/🎮️commands/🚚️move-media-node/🦀️.rs`): relative `move-nodes` from the base position
  through `node_drag_commit` (tool `…#moveMediaNode`); a stranger widget / unplaced widget / non-finite drop is refused by
  name. Shared helper `generation2d_node_drag_emit` in `✏️node-graph-edit`.
- gen2d derived previews follow open presses (F-7, twin of gen3d): `Generation2dFlowEvalWork` evaluates
  `generation2d_provisional_snapshot` (committed ⊕ `context.provisional()`); hops record and `pending_effects` compares
  `generation2d_preview_digest` = committed target digest ⊕ overlay generation, so every tick/release/abort owes exactly one
  fresh evaluation and an evaluated overlay owes nothing more.
- Laws (new):
  - `G3/✏️editor/🎮️commands/🧭️transforms/🧪️tests/🔬️unit/🦀️.rs` — gumball statechart: one-shot = one transaction, stream =
    ONE open entry holding the net motion, release commits under the ref minted at the first tick, cancel / host abort = zero
    trace, identity release = empty, guarded no-motion one-shot = idle; motion algebra; host phase parsing.
  - `G3/🧬️schema/🧬️mutations/🧪️tests/🧪️gesture-leaves/🦀️.rs` region `⏪️TimeTravel` — store-level time travel over the gesture
    leaves: `state_before` = fresh fold of the prefix, Report replay of an edited `drag-transforms` / `change-slider-value` /
    `move-nodes` equals the fresh fold of the edited log, overwrite commits that head; a slider leaf edited onto a stranger id
    reports Error `mutation.target-missing` and blocks finalize.
  - `G2/🧬️schema/🧬️mutations/🧪️tests/🧪️gesture-leaves/🦀️.rs` (new, wired in the aggregate) — gen2d leaf laws (absolute slider
    incl. range widening, relative node drag, vocabulary codes, exact inverses, labels en/de, witnesses through both op codecs)
    + the same time-travel law.
  - `DEV/…/↔️translate-selection/🧪️tests/🔬️unit/🦀️.rs` region `🛠️GumballTool` — mounted gumball with the real brep operator
    registry: one-shot drag = one row stamped `…#translateSelection` ending on `drag-transforms`, re-grab = the leaf alone
    ("Drag 1 shape(s) by (0.5, 0, 0)" / "1 Form(en) um (0,5; 0; 0) ziehen"), streamed drag keeps the committed graph still until
    the release and commits the NET offset as ONE row, `abort{captureLost}` = zero trace, and no session row per tick.
  - TS twins: `G3/…/💾️binary/🧪️tests/🧬️semantic-wire/🟦️.ts` now also checks every `parse<Type>()` against Ajv (corpus record
    parses to its payload, forged tag and one out-of-bounds payload per leaf refused by both, `axis-nonzero` declared as
    `x-semio-invariant` and refused by the parser alone); `G2/🧬️schema/🧬️mutations/🧪️tests/🧪️gesture-leaves/🟦️.ts` (new,
    registered as twin `generation2d-gesture-leaves` in `G2/../📦️packages/🦀️rust/📜️script.ts`): witnesses meet their schema
    and parse identically, out-of-bounds payloads refused by both.

### S2.4 Blockers met and environment repair (12:00–16:40)

- 12:05–12:47 compile blocked by peers (os-kernel store `HistoryPageStack`/`line_id` rollout, ui-contract `InputProps`/
  `SliderProps` fields, then `DslValue::Bytes` non-exhaustive match in `♾️infinite/🗿️artifacts/🕸️dag/🧵️retained/🦀️.rs:229`).
- 12:18–13:35 REPO-PATH-BUDGET (other session `⚪5dba80e6…`) rewrote references by basename and then moved schema twins
  (`twins.py`); reported to the coordinator 12:55 (routed to S2-INFRA). Usage cut ~13:35 → 16:30.
- 16:35 resume: the rename left 100 dangling references in my trees (crate-root `#[path]`s to `✂️disconnect`,
  `🍄️hexagonal-mushroom`, `🧩️set`, `🏷️rename`, `🎨️set`, `🔬️set-lod`; editor/viewer config/presence/transient leaf modules
  pointing at the long names `⚙️set`/`🕸️set`/`🧬️set-selected-generation`/`👁️set-generation-preview`/`📷️set-preview-camera`/
  `👁️set-preview-eval` while the dirs are short; mutate-case fixture `include_str!`s; TS imports of `✂️disconnect`; DEV
  example includes). Repaired by adopting the names on disk with `T/🧪️s2-procedural-dangling-refs.py --apply` (re-runnable,
  dry run by default; resolves a missing segment to the ONE same-emoji sibling whose slug equals / extends / shortens it at a
  hyphen boundary; never moves a directory; skips the sequence-runtime test, not mine): 100 fixed, 0 dangling left (the 15
  "unresolved" rows are extension-less TS imports, not dangling).

### S2.5 Verification so far (one gated cargo at a time; swap 92–96 %)

| command | result |
|---|---|
| `cargo check -j 2 --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-procedural-generation2d --features component-app-assembly --tests --message-format=short` (22:30) | **ok** (0 errors) after two lifetime repairs in the sqlite peer's `🫀️core/🧬️generation/🪶️sqlite/📥️reconstruction/🦀️.rs` (`order(t: &'a str)`, `expanded<'a>(…, table: &'a str)`), which failed only the `lib test` target (E0621 / "lifetime may not live long enough") |
| `cargo check … -p semio-s-artifact-procedural-generation3d --features component-app-assembly` (16:40, 17:30) | NOT REACHED: first run SIGKILLed after 20 min under swap pressure (exit 137), second blocked by a peer's in-flight `🗄️stdio/🧊️gltf/…/📦️pack/🦀️.rs` (`to_value` on `Option`, mid-edit 17:36) |
| `CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=…/target-nde-s2-procedural cargo test -j 2 … -p semio-s-artifact-procedural-generation2d --features component-app-assembly --lib` (22:45) | 284 passed, 9 failed → triage below |
| `bun ./📜️script.ts verify semantic-wire` (gen3d package) | **checks=32**, independent Ajv, pass |
| `bun ./📜️script.ts canonical-architecture` (gen2d package) | `generation2d-gesture-leaves checks=18`, `generation2d-snapshot-fixture-asset checks=5`, pass |
| `bun test ./✏️s/…/🧊️generation3d/…/✏️editor/🧪️tests/🔬️generate-interactions/🟦️.ts` | 1 pass, 0 fail |
| `bun ./📜️script.ts verify taxonomy report --scope <dir>` for the 7 new leaf dirs, 2 new fixture leaf dirs, both `🧬️mutations/🧪️tests` trees and `🧭️transforms` | clean for every new dir; `🧭️transforms` itself and its `🧫️fixtures/🧷️gesture` report the pre-existing `directory-kind-unresolved` (dated 09-28) |

Triage of the 9 gen2d failures (22:45 run) and what was done (no cargo since — fleet CARGO HOLD rule 26 at 02:45):
1–3. my `✏️node-graph-edit` laws: `history_snapshot().upserts` is newest first, so `rows[before..]` read the setup row →
   rows are now sorted by `seq` (also in gen3d and the DEV gumball law). The slider-press law already proved 2 rows / 2
   transactions / absolute leaf before the ordering assertion tripped.
4. my gen2d time-travel law: "ordered-map root must be explicitly retired before drop" — the `Arc` returned by
   `state_before` was dropped bare → now unwrapped and retired cold when this law holds the last reference (also gen3d).
5. `dispatch_registers_semantic_descriptors` hard-coded 14 kinds → `KINDS.len()`.
6. `kinds_match_the_enum_and_the_catalog`: the new kinds were missing from the committed oracle manifests → both
   `🔮️oracles/🔣️.json` catalogs gain `deferredKinds` (gen2d: `change-slider-value`, `move-nodes`; gen3d: the five gesture
   leaves) — the CAD precedent for wire-witnessed leaves without quintets. gen3d's `every_mutation()` gains the five leaves.
7. `vcs_artifact_app_non_empty_retained_maintenance_swap…`: the store's edit envelope now requires `line` (peer's
   paged-ledger / per-viewer-head rollout) → `"line": null` added to the gen2d and gen3d production-envelope fixtures.
8. `sqlite_snapshot_procedural_generation2d_erased_native_encodings…`: "snapshot owner has no controlled native decoding
   implementation" — the SQLite-snapshot peer's lane, not touched.
9. `generation2d_window_camera_ownership…`: expects a retained rejection, observes a framework `app.message` fault — runtime
   fault-classification change by a peer, not touched.

### S2.6 Open items (design questions raised, not improvised)

1. **First gumball on a fresh shape labels its row from the splice**: the transaction is `[create-widget, synapse rows…,
   drag-transforms]` (the operator must exist before the relative leaf applies), and the runtime labels a transaction row by
   its FIRST leaf (`history_leaf_row_label`), so the row reads "Create widget … (+N)" instead of "Drag 1 shape(s) by …".
   Re-grabs are the leaf alone and read correctly. Proposal for S2-W2A: let an `Emit::commit_transaction` name its primary
   leaf (or label a transaction row by its last leaf), instead of every tool reordering around the rule.
2. **P8/P9 leaf quality (O\*)**: `setWidgetInput` (operator input field, commits on blur) and the mesh edits
   (`editMeshSelection`, `knifeMeshSelection`) are one edit each, but their leaves are the whole-operator `update-widget` /
   the splice (`create-widget` with the parameters inside `params`), so a history edit edits a structured widget record rather
   than "extrude distance = 0.1". A generic absolute `change-widget-input {id, channel, value: typed literal}` leaf would serve
   P8 and, appended after an insert, P9; it needs a decision on the typed-literal union in `x-semio-ui` (number | text |
   boolean | point | vector) — raised for the coordinator.
3. **Stream ticks and session rows**: the plugin-side gumball stream (and every plugin-side streamed tool: puzzle, fem,
   shooting) returns an empty emit per tick; `record_settled_typed_operation_command` logs a Mutation verb that published
   nothing as a session row. The DEV law `a_streamed_gumball_drag_is_one_edit_and_an_abort_is_zero_trace` asserts no such
   row appears; if it fails, the fix belongs to the runtime (S2-W2A), like the scrub glue's `command_logged`.
4. Central regenerations (coordinator, rule 23): `schema generate` (the 7 new leaves are `leafUncatalogued`), `describe`
   for the procedural composition (gen3d describe texts for translate/patchFlowWidgets changed; gen2d `moveMediaNode`
   semantics), launch rows if any.

## Session 3 — 2026-10-02

Successor executor S3-PROCEDURAL (coordinator `⚪b7db773a…`). Status (19:25): SOURCE-COMPLETE for every assigned item; the owed
cargo verification (S3.7.1) waits for the coordinator's "TREE GREEN" (framework red from a peer DSL-crate extraction).

### S3.1 Repair-first diff (rule 28)

Files in my trees newer than the S2 section (02:45): only peer sweeps — the `RecordSpecProducer` dsl refactor
(`spec_fn()` → `(spec_fn.ordinary)()` in every config/transient/presence/binary codec), `store::EngineHandles` →
`semio_framework_2d::compute::EngineHandles` (+ `semio-framework-2d` dependency in both crate manifests). My predecessor's
S2.5 triage fixes 1–7 are on disk (rows sorted by `seq`, `retire_cold` of the `state_before` base, `KINDS.len()`,
`deferredKinds`, `"line": null`). No half-finished edit found.

### S3.2 Verification (one gated cargo at a time)

| command | result |
|---|---|
| `cargo check -j 4 --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-procedural-generation2d -p semio-s-artifact-procedural-generation3d --features <both>/component-app-assembly --lib --tests --keep-going --message-format=short` (check-1, 11:00–11:55; gen3d reached after my S3.3 + leaf wiring landed) | gen2d lib + lib test **ok**; gen3d lib **ok**; gen3d lib test **2 errors** — `🔄️rotate-selection/🧪️tests/🔬️unit/🦀️.rs:8,9` `rotate_ids` (deleted by session 1, law never rewritten) → fixed (S3.5) |
| `bun ./📜️script.ts verify semantic-wire` (gen3d `📦️packages/🦀️rust`) | before the new leaf **checks=32**; after **checks=58** (10 vectors × 5 + 8), independent Ajv, pass |
| `bun ./📜️script.ts canonical-architecture` (gen2d package) | `generation2d-gesture-leaves checks=18`, `generation2d-snapshot-fixture-asset checks=5`, pass |
| `bun ./📜️script.ts canonical-architecture` (gen3d package) | `widget-creation 334`, `terminology 2921`, `snapshot-fixture-asset 5`, `graph-keyboard 47`, pass |
| `bun test ./…/🧊️generation3d/…/✏️editor/🧪️tests/🔬️generate-interactions/🟦️.ts` | 1 pass, 0 fail |
| `node_modules/.bin/tsc -p 🗑️generated/s3-procedural/tsconfig.change-widget-input.json` (strict; new leaf twins, aggregate union, semantic-wire test) | 0 errors (after typing the corpus: the predecessor's test had 4 `TS18046` under strict) |
| `bun ./📜️script.ts schema mutation-payloads --under ✏️s/🔌️plugins/🌀️procedural` (cwd `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`) | before: 54/54 payloads, 52/52 witnessed, **0 findings**; after the new leaf: **55/55, 53/53, 0 findings** |
| `bun ./📜️script.ts schema mutation-inputs --under ✏️s/🔌️plugins/🌀️procedural` | before: 80/80 inputs, **0 findings**; after: 1 finding = `change-widget-input` `leafUncatalogued` → central `schema generate` (coordinator). The TS reader `mutationInputDefs` reads the new root union without error (variant selector `/type` + per-variant inputs; probe `🗑️generated/s3-procedural/input-defs-probe.ts`) |
| `bun ./📜️script.ts verify taxonomy report --scope <leaf dir>` / `<fixture dir>` for `🎛️change-widget-input` | both `clean=true errors=0 warnings=0` |
| check-2 (12:00–12:31, `-p gen2d -p gen3d -p semio-s-composition-laws --lib --tests --keep-going`) | RED, not mine: a peer schema-crate split (12:24) rewrote 161 plugin files to `::semio_framework_schema_registry` before the Cargo deps landed (gen3d 14× E0433) — converged by 12:32 |
| check-3 (12:44–12:51) | RED, not mine: `semio-framework` `🛂️manifest/🦀️.rs:1266-1269` + infinite-dag `semio_framework_schema_state` (same peer split, reported 12:52); usage cut ~13:05, reboot ~17:00 |
| check-4 (18:44–18:54, after the reboot; framework green again) | my crates compile (lib); 18 errors in MY test files only: E0603 `protocol::Terminology` / `protocol::Locale` now private (peer layering sweep; the labels live in `semio_framework_ui_locale`) — gen2d gesture-leaves + node-graph-edit laws, gen3d node-graph-edit laws, DEV translate-selection law → repointed to `semio_framework_ui_locale::{Locale, Terminology}` |
| check-5 (19:05) / check-6 (19:15–19:17) | check-5 SIGKILL (exit 137, guard/OOM); check-6 RED, not mine: `semio-framework-plugin` 13× E0425 `dsl::LanguageSpec` / `preflight_languages` / `register_languages` (🔌️plugin/🦀️.rs:3015…39124; peer moving the language registry out of 🗣️dsl), reported 19:18 |
| check-7 (19:18) + 3 gen3d-lib probes (19:19–19:24) | RED, not mine: `semio-framework-os-kernel` 61–295 errors (`os_dsl::Severity/FaultCode/FaultOrigin`, `dsl::Diagnostic`; Codex DSL-crate extraction, `semio_framework_dsl` not on disk yet). Coordinator: no polling, wait for "TREE GREEN" |
| `schema mutation-payloads --under ✏️s/🔌️plugins/🌀️procedural` (10-03 06:00) | 55/55 payloads, 53/53 witnessed, **0 findings** |
| `schema mutation-inputs --under ✏️s/🔌️plugins/🌀️procedural` (10-03 06:00) | 13 findings: `change-widget-input` `leafUncatalogued` (central `schema generate`) + 12 NOT mine-by-edit: gen3d CONFIG leaves `app.procedural.3d.config.mutation.set-camera` / `set-snapshot` camera `x/y/zoom` `uiInvalid` + `widgetIncompatible` (appeared overnight with the peer sweeps; config lane, not history) |
| `verify semantic-wire` (gen3d package, 10-03 06:00) | **checks=83**, independent Ajv, pass (after registering `x-semio-inverse-rows`) |
| check-8 (10-03 06:02–06:07, TREE GREEN core) | RED, not mine: generation3d's deps `semio-s-artifact-stdio-zip` (52), `-step` (125), `-gltf` (13), `-svg` (6) — E0277 `From<ValueError>` (zip `🔖️2.0/…/🪶️sqlite/🦀️.rs:11,104`, svg `🔖️1.1/…/🦀️.rs:197-200`), reported 06:08 |
| check-9 (10-03 06:30–06:34, same command) | RED, not mine: stdio `-zip` (2), `-gltf` (13), `-svg` (6) still; `-step` green |
| `cargo test -p semio-framework-os-kernel-neural-engine --lib` (private target, 06:40) | **67 passed, 0 failed**, incl. the new §20.10 law `a_wire_shadows_the_literal_its_neuron_records_for_that_port` (written with the fix; red-before not executed) |
| `cargo test -p semio-framework-os-flow --lib` (private target, 06:40) | RED, not mine: `semio-framework-os-kernel` 52 errors — a peer's in-flight `IoError` refactor (`🔨️modules/🚪️io/🦀️.rs`, `🏪️store/🦀️.rs`: `IoError` has no field `message`, no `From<String>`); compiled clean in check-9 ten minutes earlier |

### S3.3 Changes (item 3: generation3d as the `World3dHost` live consumer, audit C-5)

Gap found: gen3d published `gumballLiveDispatch` only for mesh components, so a shape gumball (the neuron-param
transforms `drag-`/`rotate-`/`scale-transforms`) was previewed by the host as a rigid local instance move — downstream
operators (booleans, arrays, analysis) never followed the drag. And even for the live component gumball, the FIRST grab
painted nothing: the flow evaluation folds the open gesture (overlay splices the transform operator and moves the
`preview` flag onto it), but the preview bodies rendered the COMMITTED snapshot and the committed `graph` marks, so the
spliced operator's output was never painted and the selection resolved to no instance.
- `G3/✏️editor/🎮️commands/🧭️transforms/🦀️.rs`: new `GumballSelection {nodes, components}` (+ `writes()`, the commit's
  replacing interaction writes) and `GumballPreview {rows, selections}`; `gumball_splice` returns the selection;
  `GumballGestures::provisional` returns rows + selections and no longer clones the open leaf.
- `G3/✏️editor/🦀️.rs`: `preview_selection_json` sets `gumballLiveDispatch: true` for every gumball (shapes too);
  `render_with_request_context` paints the two preview bodies from `generation3d_gumball_preview` (committed ⊕ every open
  gesture — the same overlay the flow evaluation reads — and `PreviewInteractionMarks::following` the gesture's
  selection) while a gesture is open; overlay retired cold after the render.
- C-5 (`📓️audit-s2-wave-a.md`): the discarded checked-adapter results in both editors are now one named fold each
  (`generation3d_fold_provisional`, `generation2d_fold_provisional`) whose contract is documented: a refused provisional
  leaf paints nothing; the commit of the same leaf reports the refusal as the row outcome.
- Laws: `G3/…/🧭️transforms/🧪️tests/🔬️unit/🦀️.rs` `an_open_gesture_previews_exactly_what_its_release_commits` (the
  preview overlay equals the committed snapshot after the release; marks follow onto the operator; committed untouched
  while open; abort → nothing to paint); DEV `…/🧬️generate/🪟️windows/👁️preview/🧪️tests/🔬️unit/🦀️.rs` asserts
  `gumballLiveDispatch` on a shape selection.

### S3.4 Design questions → coordinator decisions (design §19, 11:40)

Q1 = §S2.6.1 → §19.1 `ArtifactApp::tool_intent_kinds` (runtime by S3-W2A, landed in `🔌️plugin/🦀️.rs`
`history_intent_label`). Q2 = §S2.6.2 → §19.2 approved: `change-widget-input {id, channel, value}` with a discriminated root
union over `type`; P9 inserts with DEFAULT params + one input leaf per user-set channel; whole-record paths deleted.

### S3.5 Changes (design §19)

- §19.1 adopter: `G3/✏️editor/🦀️.rs` `ArtifactEditor::tool_intent_kinds` — `…#editor#translateSelection` →
  `drag-transforms`, `#rotateSelection` → `rotate-transforms`, `#scaleSelection` → `scale-transforms`. DEV law
  `a_gumball_drag_is_one_transaction_of_the_relative_leaf` now asserts the FIRST grab row (splice + leaf) reads
  "Drag 1 shape(s) by (1, 2, 3)…".
- §19.2 new leaf `G3/🧬️schema/🧬️mutations/🎛️change-widget-input/` (binary tag 19): Rust leaf (`WidgetInputValue`
  `#[value(tag = "type", content = "value")]` flattened into the payload — wire `{mutation, id, channel, type, value}`,
  point/vector as `[x, y, z]`; `literal()`/`of_literal()` against the operator param literal `{"$schema", value|x,y,z}`;
  `admissible()` = schema hard bounds; ONE `landing()` shared by the diff and the retained replay), `🔺️diff` (invariant /
  target-missing / target-mismatch {wired, no such input, untyped, other type} / no-op), `↩️inverse` (base widget whole), TS
  twins (`parseChangeWidgetInput`, diff/inverse mirrors), descriptor + payload schema (root `oneOf` over `type`, member
  labels en/de, per-variant widget: stepper / multiline / toggle / vector) and wire witness — the schema-first surfaces are
  generated by `T/🧪️s3-procedural-change-widget-input.py` (re-runnable byte for byte). Wired into the aggregate (`mod`,
  variant, `KINDS`, aggregate `🔣️.json` + `🟦️.ts`), `💾️binary/📡️.protocol.semio`, the binary codec (DSL mirror with the
  input as one dynamic value, retained decoder ordinal 19 reusing the dynamic-value slot of ordinal 13, retained replay via
  `landing()`, initialization digest, variant count 20), oracle `deferredKinds`.
- P8 `🎚️set-widget-input`: `input_leaf()` validates against the declared port (unconnected, scalar, declared literal type)
  and emits ONE `change-widget-input` (a text source's `text` is a text input; an unchanged field is no edit);
  `apply_to_host` + the whole-operator `update-widget` diff path deleted.
- P9 `🥽️edit-mesh-selection` / `🔪️knife-mesh-selection`: `inputs()` + `edit_rows()` / `cut_rows()` — the splice inserts the
  operator with DEFAULT params (`insert_mesh_operation` no longer sets params), then one `change-widget-input` per input
  (components text list, offset, cuts, distance/amount; face, start, end); the host-level `insert_operation` deleted.
- Laws: gesture-leaves region `🎛️WidgetInput` (every literal type sets + inverts; vocabulary codes; literal round trip),
  witness + label rows, time-travel law gains an edited `change-widget-input` (downstream input overrides it); semantic-wire
  corpus +5 vectors (one per type; schema 10 cases, tag ≤ 19, `expected.input`), Rust semantic-wire arm, TS twin test (parser,
  out-of-bounds, every type covered; schemas compiled once per `$id`); retained-authority fixtures assert distinct variants
  (`every_variant_decodes_through_retained_structural_grants`); unit `every_mutation()`; set-widget-input law rewritten;
  DEV mesh-edit and knife laws fold the new rows and re-evaluate; `rotate-selection` law rewritten (the deleted
  `rotate_ids` was still referenced: the only gen3d lib-test compile error in check-1).
- Warnings fixed in my files: unused `serde_json::Value` import and dead `owe_attached_previews` in `G3/✏️editor/🦀️.rs`,
  no-op `.clone()` in `import_media`.

### S3.6 Node-graph row contract (coordinator 12:3x, S3-FLOWCAD/S3-GRAPHS schema `🔣️node-graph-edit-rows`)

- gen2d + gen3d `✏️node-graph-edit`: rows decoded by the ONE shared decoder (`🛠️tool-machine`
  `node_graph_edit_rows` / `NodeGraphEditRow`) — at admission (`command_from_action` refuses a bad batch) and at authoring
  (`rows()`); `setHostSnapshot` and the ambient `deleteSelection` row are gone with their whole-fixture / ambient paths;
  `connect` / `disconnect` / `insertPort` / `delete {nodeIds, synapseIds}` edit by id and a refused host edit refuses the batch
  (gen2d keeps its document-level `disconnect-synapse` fallback for wires an uncontributed-kind rebuild dropped);
  `setSlider` → absolute `change-slider-value`; `move` → node-drag machine. The interaction-reading `apply` /
  `apply_selected` entry points are deleted (one `handle` for every route; gen2d `handle` / retained reduce no longer read
  the selection). Context menus use FLOWCAD's fixed `node_graph_delete_selection_spec(.., &nodes, &edges, ViaNodeGraphEdit)`
  (explicit ids); gen3d's own `deleteSelection` verb (keyboard) now also cuts selected wires.
- Laws: both crates `node_graph_edit_takes_exactly_the_shared_row_vocabulary` (renderer fixture: accepted rows decode, every
  refused row refuses the whole batch at admission and authoring); gen3d `a_delete_row_deletes_exactly_the_named_widget_and_wire_as_one_edit`;
  gen2d context-menu law asserts the delete row is `Delete {nodeIds: ["slider"]}`; drag-law setup uses `reorganize` (no
  whole-fixture placement) with base-relative expectations; gen3d interactions fixture + TS twin + DEV law list the six rows.
- §19.1 law `every_gumball_tool_declares_the_leaf_it_yields_as_its_intent` (transforms unit tests).

### S3.9 Resume 10-03 05:50 (TREE GREEN): §19.4 (audit P1), §20.9 (audit P2, option A), gumball corpus (audit S2)

- **§20.9 / P2 — pure folds over self-describing records.** Flow framework (with S3-FLOWCAD's go-ahead, "you write it"):
  `🌊️flow/🗿️artifacts/🌊️flow/🧬️schema/📸️snapshot/🦀️.rs` new `default_neuron_params_from_info` —
  `widget_from_descriptor_with_info` records EVERY declared input's default literal in `params` at insertion (data inputs
  without a default stay absent). Law `an_inserted_operator_records_every_declared_default_input` (flow host unit tests).
  Leaf: `ChangeWidgetInput::landing` no longer reads `flow_extension_registry()` — a peer had added registry port checks
  overnight — so existence and shape come from the record alone: a channel `params` does not hold (and a slider, or a
  text source off `text`) → `InputRefusal::UnknownChannel` = **`mutation.target-missing`**; wired / other literal type /
  scalar-vs-list / other item type → `target-mismatch`. The retained replay reads the same `landing`. Laws: gesture-leaves
  vocabulary law (phantom channel → target-missing, list onto scalar → mismatch); the semantic-wire Rust law now folds every
  corpus vector on a replica WITHOUT any flow extension installed (the peer's registry install is deleted).
- **§19.4 / P1.** `🧭️transforms`: `ensure_component_node` inserts with DEFAULT params (no `set_neuron_params`).
  `gumball_splice` appends one `change-widget-input` per channel the gesture sets on a newly inserted operator:
  `component_gesture_inputs` (mode, component set, pivot for rotate/scale) plus `schema::gumball_identity` (zero offset,
  zero turn about +z, unit factors), via `schema::record_input_leaves`. The latter types each value like the record's
  literal and skips channels the record does not hold or already holds. The identity rows are needed because the
  materialized defaults are not the identity (mesh translate offset (0,0,1), xform rotate angle π/4). Laws (DEV):
  `a_first_component_grab_inserts_defaults_then_one_input_per_chosen_channel` (create-widget holds the default
  `mode = vertex`, then `mode = face`, `selection = [1]`, `offset = 0`; a re-grab appends nothing); the component laws
  land the inputs through `land_component_inputs`.
- **S2 corpus.** DEV law `the_world3d_gumball_live_protocol_lands_as_its_guest_edits` (shared
  `🛠️gumball-live-protocol.json`, recipe from S3-SPATIAL): every guest case through `handle_action` with `ids = ["extrude"]`,
  edits and offset per case, `consumed >= 8`. The S3.3 preview law moved to the DEV crate (it needs the brep operator
  registry, which the generation3d lib tests do not install; `generation3d_gumball_preview` is now `pub`).
- **Incident (05:50, repaired).** Re-running `T/🧪️s3-procedural-change-widget-input.py` overwrote a peer's overnight list
  extension of the leaf's payload schema and TS twin. The generator now emits all 10 variants, rebuilt from the peer's
  variant schemas printed just before; `verify semantic-wire` passes against the peer's 15-case corpus (checks=83).
  Reported to the coordinator. Coordinator decision: there is no list widget (array inputs render through the N2 array
  editor, inferred from the array schema) → `widget: "list"` dropped from the five list variants (variants, bounds and
  labels kept). **Both generators are now safe**: a bare run is a DRY RUN; `--write` writes only differing files and
  REFUSES any file whose content is not that generator's last output (sha256 provenance stamps in
  `T/🧪️s3-procedural-generator-stamps.json`); unknown arguments exit non-zero. Proof: `🧪️s3-procedural-change-widget-input.py`
  dry run → 1 file (the schema), `--bogus` → exit 1, `--write` → 1 file, rerun → 0 files (idempotent);
  `🧪️w3-t2-procedural-leaves.py` (session-1 generator) now REFUSES the 8 files peers edited since (CLOSURE's
  `x-semio-inverse-rows` on the four gumball/node leaves, TS twin sweeps), exit 1 — it would have clobbered them the same way.
  The semantic-wire TS test registers `x-semio-inverse-rows` with Ajv (CLOSURE's new keyword failed strict mode); checks=83.
- Peer sweeps overnight also touched my trees: `dsl::json` → `semio_framework_pack_json`, labels → `semio_framework_ui_locale`,
  `MutationMessage::warn` → `warning`, plus a collection-input extension of P8 (`edit_collection_value`). All kept.

### S3.10 Design §20.10 — a wire shadows the recorded literal (10-03 06:20–06:50)

- **Defect found in my §20.9 change.** The neural engine merged a neuron's `params` OVER its wired inputs
  (`🧠️neural/⚙️engine/🦀️.rs` compute merge sites, `input.merge(&neuron.params)`), and `params` reach the tree unfiltered
  (`🌊️flow/…/📸️snapshot/🦀️.rs` `widget_properties`, `tree_from_host_snapshot`). With every declared default now recorded at
  insertion, an operator inserted from the catalogue and then wired evaluated its recorded DEFAULT, not its wire (box
  `width` ignores the slider; the flow host laws on `host_with_two_node_chain` — `math.passThrough` default `number` 0, wired
  `add→pass@number` — would read "0" for "3"). The gumball first grab is unaffected (its data ports have no default).
  It also fixed a latent bug from before §20.9: an inspector-set literal that was later wired hid its wire.
- **Decision (coordinator, recorded as design §20.10):** precedence wire > recorded literal > declared default. Engine
  (file untouched for 4 h before the edit, region-scoped edits on unique anchors, the peer's 162-line diff left alone): one helper
  `unwired_params(tree, neuron)` drops every key that is the non-empty `to_port` of an incoming synapse; used at the two
  compute merge sites (`evaluate_channels_budgeted`, `evaluate_channels_cached`). Boundary kinds, `count`/`operators` and
  empty-`to_port` dictionary wires are unchanged.
- Laws: engine `a_wire_shadows_the_literal_its_neuron_records_for_that_port` (parallel and sequential walks: wired → 4, an
  unwired recorded literal → 20) — **pass**; flow host `an_inserted_operator_wired_into_a_defaulted_input_evaluates_its_wire`
  (written; run blocked by the os-kernel break, S3.2).
- **Examples normalized to the materialized form (§20.9), in progress.** Laws (TDD, red until the assets are rewritten):
  DEV `✏️editor/🧪️tests/🔬️fold-contract` `every_bundled_example_records_every_declared_default_of_its_operators` (the 9 gen3d
  examples, registry installed by the serial lock; an unknown kind fails too) and DEV `generation2d-example-export`
  `the_bundled_example_records_every_declared_default_of_its_operators`. The assets (`📚️examples/*/🖼️assets/*/🗣️.dsl.semio`,
  28 gen3d operators with `params=[ ]`, 2 gen2d) are rewritten from the codec's own printer: two temporary `[DEBUG]` tests
  (`debug_print_normalized_examples`, `debug_print_normalized_example`) print each example with its defaults recorded. They
  are deleted after one run.
- **Camera lint findings** (12, S3.2): cause = the uncommitted regenerated `generation3d/…/🧬️schema/🔣️.json` now types
  `CameraJson` x/y/zoom as `$ref: Binary64Transport` (anyOf `Binary64Word` | number), which the input reader does not read
  as a number for `step`; gen2d's artifact schema carries the same `$ref`. Routed by the coordinator to S3-AGNOSTIC (generic
  input-reader fix).
- **For S3-FLOWCAD (flow fixtures/tests whose expectations change with materialized params):**
  `🌊️flow/🖥️host/🧪️tests/🔬️unit/🦀️.rs` — every `add_widget` of `math.passThrough` that wires `number`
  (`host_with_two_node_chain` :301 and the chains at :583, :645): expectations unchanged ONLY with §20.10; `:2464-2465`
  (`plugin.left/right`, defaults per their test infos). The flow DEFAULT document (`🌊️flow/…/📸️snapshot/🦀️.rs:276`,
  `math.add` with `params: Dictionary::new()`; also the tree at :353) is not self-describing: its `b` default is not recorded,
  so a `change-widget-input` on `add@b` of the default document is `mutation.target-missing`. Normalizing it is FLOWCAD's call
  (generation3d's `default_generation3d_snapshot()` is that document).

- **Audit P3 (closed in source).** Text labels print at most 32 characters, then `…` (`CHANGE_WIDGET_INPUT_LABEL_TEXT`;
  gesture-leaves label law: a 36-character text shows its first 32 characters and never its tail). Bounds: there is no
  operator range to bound against — `ChannelSpec` declares type, cardinality and default only — and a pure fold reads no kind
  descriptor (§20.9); the hard bounds stay finite numbers + the 16 MiB text cap, documented on `admissible()`.
- **Coordinator answers / open questions (06:4x):** `channel` stays `widget: "text"` until S3-W1E lands
  `x-semio-ui.optionSource {snapshot: <pointer template>}` (decision (a)); then I declare it over
  `/host_snapshot/widgets/{id}/params`. S4 (localized gumball refusals) needs a guest fault-notice mechanism (a `Fault`
  carries only `{code, message}`, `ActionMeta` no locale; the only code→en/de table is the kernel's `HISTORY_NOTICE_LABELS`);
  P4 (mesh edits as one-shot tool transactions) needs the intent choice — both asked.

- **Audit S4 (coordinator: named codes now, localized table when S3-NOTICES lands).** `🧬️schema/🦀️.rs` `GumballRefusal`
  (12 variants, `code()` = `generation3d.gumball.{unknown-operation, no-shape-source, kind-unavailable, no-shape-output,
  list-output, identifier-occupied, transform-unavailable, mesh-missing, not-indexed-mesh, selection-changed,
  component-selection, host-edit}`, `detail()` = the English developer text); `ensure_gumball_node`,
  `ensure_component_node` and `validate_component_gesture` refuse with it; `From<GumballRefusal> for Fault` (no more
  `app.message`). Law `every_gumball_refusal_is_a_named_fault_code` (transforms unit tests: one code per refusal, all under
  `generation3d.gumball.`, the fault carries it; a changed component set → `selection-changed`).
- **Audit P4 (coordinator option (ii)).** `editMeshSelection`, `knifeMeshSelection` and the component-mode `deleteSelection`
  are ONE tool transaction each: `edit_mesh_selection::mesh_edit_emit(verb, doc, rows)` commits the rows one-shot through
  the framework `Scrub` machine at rest (ref minted from the admission's authoring seed; plain without one);
  `tool_intent_kinds` = `["create-widget"]` for the three verbs. `create-widget`'s label names the operator kind from the
  catalogue terminology — `Insert "Extrude Mesh Faces" (id)` / `"Mesh-Flächen extrudieren" (id) einfügen` (non-operator
  widgets keep "Create widget"). For that the label set `crate::terminology` is compiled with every feature set (declared at
  the crate root; `editor::generation3d::terminology` re-exports it). Laws: transforms intent law (three mesh verbs →
  `create-widget`), gesture-leaves label row, DEV app law `retained mesh edit` asserts the row's tool
  `…#editor#editMeshSelection` and an "Insert …(extrude__<op>)" label.
- check-10 (06:58): RED, not mine — `semio-framework-os-kernel` 11 errors (same peer `IoError` refactor converging:
  `🔨️modules/🧬️semio/🦀️.rs:4,37,38`, `🏪️store/📜️space-history`, `📦️codec`, `🚪️io/🦀️.rs:2654`).
- S3-W1E landed `x-semio-ui.optionSource` (§20.11); declaring it on `channel` waits for an `{id}` record-lookup form, because
  `hostSnapshot.widgets` is an array of records — asked S3-W1E.

- **Resume 10:42 (after the usage cut ~07:15).** The §20.10 engine edits were complete before the cut (helper
  `unwired_params` at `🧠️neural/⚙️engine/🦀️.rs:2639`, used at :2368 and :2470; the peer's later edits at 08:16 kept them).
  check-11 (07:18): RED, not mine — `semio-s-artifact-stdio-txt` (12), `-dwg` (8) (peer stdio migration; cargo waits).
- **`optionSource` declared (S3-W1E §20.11 + `{id}` record lookup).** `change-widget-input.channel` is now
  `widget: "select"` with `optionSource: {snapshot: "/hostSnapshot/widgets/{id}/params"}` — the options are the keys of the
  edited operator's `params` in the previewed document; the schema stays open (no enum), so a stale/phantom channel is still
  decided by the fold (`mutation.target-missing`). Written by the schema-first generator (`--write` → 1 file; rerun → 0
  files). TS reader probe (`🗑️generated/s3-procedural/input-defs-probe.ts`): every variant's `/channel` reads
  `{kind: "string", optionSource: {kind: "snapshot", pointer: "/hostSnapshot/widgets/{id}/params"}}`. `verify semantic-wire`
  **checks=83**. `schema mutation-inputs --under ✏️s/🔌️plugins/🌀️procedural`: **80/80 inputs of 53 leaves, 1 finding** =
  `change-widget-input` `leafUncatalogued` (central `schema generate`); the 12 camera findings are gone (S3-AGNOSTIC).

- **S4 localized (S3-NOTICES §20.12 landed).** `🧭️transforms/🦀️.rs` `gumball_fault_notices()` — 12 rows, one fixed en/de
  sentence per `generation3d.gumball.*` code (`{kind}` on `kind-unavailable` / `transform-unavailable`, filled from
  `Fault::with_param("kind", …)` in `From<GumballRefusal> for Fault`; the English developer detail is never a notice);
  declared by the editor's `ArtifactEditor::fault_notices` (the viewer emits none). Law
  `every_gumball_refusal_code_has_a_localized_notice` (framework `validate_fault_notices` empty, one notice per refusal,
  every locale × terminology filled, de `{kind}` fill). Reported to S3-NOTICES (file + 12 rows).

### S3.7 Open items

1. **Owed runs, in order, one gated cargo at a time, once the peer stdio migration is green.** Every Rust change since
   10-02 11:55 is WRITTEN, NOT VERIFIED; the only exception is the §20.10 engine law, which ran green (67/67).
   - `cargo check` gen2d + gen3d + `semio-s-composition-laws` `--tests`.
   - `cargo test -p semio-framework-os-flow --lib`, which includes `an_inserted_operator_wired_into_a_defaulted_input_evaluates_its_wire`
     and the `host_with_two_node_chain` laws.
   - `cargo test --lib` for gen2d and gen3d, using the private target `target-nde-s3-procedural`.
   - DEV `--test generation3d-app-laws`, which covers:
     - the streamed-gumball law and the §19.1 first-grab label;
     - the §19.4 component law and the corpus law;
     - the preview law and the mesh-edit/knife laws (P4 row tool and label);
     - `every_bundled_example_records_every_declared_default_of_its_operators`.
   - DEV `--test generation2d-example-export`.
   - `cargo check --target wasm32-wasip2` for both plugin crates.
2. **Example normalization (§20.9), finish right after item 1 compiles:**
   1. Run the two `[DEBUG]` printers (`debug_print_normalized_examples`, `debug_print_normalized_example`) with `--nocapture` into `🗑️generated/s3-procedural/`.
   2. Run `T/🧪️s3-procedural-normalize-examples.py <capture>...`. It is a dry run that prints the diff and refuses any asset whose authored lines would change. Then run it again with `--write`.
   3. Delete both `[DEBUG]` printers.
   4. Rerun the two example laws to green.
3. Live-only E2E check: with `gumballLiveDispatch` on shapes, the React host must not re-anchor the gumball to the moving
   answer mid-drag (shapes have no `gumballTarget`; fem 3d sets one). Verify on a 6018-class serve.
4. S3-NOTICES' fault-notice gate, `schema fault-notices --under ✏️s/🔌️plugins/🌀️procedural --census`, run on 10-03:
   12 of 19 guest codes are labelled, from 12 declared notices; all `generation3d.gumball.*` codes are covered.
   The 126 remaining procedural findings are routed to the coordinator; they are pre-existing and not gumball:
   - 3 missing notices: `generation3d.io.export`, `generation3d.io.import-accept`, `generation3d.widget.add`;
   - 2 codes with only two segments: `generation2d.child-projection`, `generation3d.child-projection`;
   - 2 framework-namespace codes raised as faults: `mutation.target-mismatch` / `-missing`;
   - 118 anonymous `Fault::from(text)`;
   - 1 `faultNoticeDescriptor`, which needs a `describe`.

### S3.8 Coordinator actions

- Central `schema generate`: `change-widget-input` is `leafUncatalogued`, the only `schema mutation-inputs` finding in my scope.
- `describe` for the procedural composition (`🌎️hub/🧩️compositions/🌀️procedural/🔣️.json`):
  - gen3d: the new leaf, the `channel` option source, `setWidgetInput`/mesh-edit semantics, mesh edits as tool transactions, the context-menu delete row, the named gumball codes and the editor's `faultNotices` (12 rows);
  - gen2d: the `nodeGraphEdit` rows.
- Activation of the procedural lanes for the live consumer check (open item 3).

## Session 4 — 2026-10-04

Continued by S4-TOOLS-B in `📓️s4-tools-b-report.md` (one report for both inherited WPs, fleet rule 34).
