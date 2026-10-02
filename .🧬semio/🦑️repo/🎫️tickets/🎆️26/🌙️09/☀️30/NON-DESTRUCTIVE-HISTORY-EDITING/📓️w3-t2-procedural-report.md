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

Successor executor S3-PROCEDURAL (coordinator `⚪b7db773a…`). Status: IN PROGRESS; updated at every milestone.

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
