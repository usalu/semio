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
