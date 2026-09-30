# 📓️ Audit: Interaction Paths Outside the Tool-Machine Model (W3-T Wave 2)

Ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING`. Read-only audit (no repo file edited except this report; no cargo, no bun, no
dev server, no git write). Requirement under test: every tool is a state machine that yields parametric mutations inside ONE
`ToolTransaction`; tools are never history-editable, their yielded mutations are (`📋️design.md` §5/§7/§10, `🧭️plan.md` "W3-T brief",
reference `📓️w2-d-report.md` §1).

**Verification level.** Everything below is static reading of the tree as of 2026-09-30 ~21:40 (files churn under peers; anchor on
symbol names when a line moved). Nothing was executed. Claims marked *(by reading)* were not reproduced at runtime. The
contradiction-prone ones are listed in §9.

## 0. Verdict

1. **Already owned by running executors** (puzzle 3d/5d, draw + note, shooting + lowpoly + fem (+ generation3d + layout gumball per
   the 21:36 SPATIAL decision), flow + cad): not re-audited except where a shared host or vocabulary crosses into other plugins (§3).
2. **Still outside the model after those four land: 18 plugins (+ animate, driver unknown) in six shapes.** The pointer gestures
   are the node-graph drags on the shared `nodeGraphEdit` wire, the wfc bitmap stroke and the raster stroke. By count the dominant
   population is **continuous controls (sliders, held spinners, number/text inputs)** and **typing**, which the pointer-centric
   brief does not name but which are the largest source of non-parametric, per-tick or per-keystroke document edits (§5.2, §5.3).
3. **The wave-2 list in the status log is partly wrong** (§8): `block` has no gesture, `process`/`playbook`/`norm` are not per-tick
   amend plugins, while `energy`, `vcs`, `gis`, `wfc`, `raster`, `sequence` and the `stdio` text editors are missing.
4. **Three reusable machines remove most of the work**: a *scrub* machine (every slider/held-number control already speaks
   `{value, gesture, commit}` through one framework lane), a *typing-run* machine (timer-driven idle commit), and the node-graph
   gesture record the flow executor must define (12 guests share `nodeGraphEdit`). Without a shared scrub machine, draw (D11),
   flow (F6) and the gen3d slider paths will each invent their own.
5. **One framework defect is one line**: `⏯️tool-run` publishes every finalized run with `transaction: None` (§4 F-1). Until fixed,
   the 17 algorithmic tools are state machines that yield into **no** `TransactionRef`.

## 1. Method

- Census greps (`/usr/bin/grep`, per plugin, non-test `.rs`): `Emit::amend(`, `amend_config(`, `coalesce_key: Some`, `set_coalesce_key`,
  `Emit::commit(`, `transformBegin/End`, `paintStrokeBegin/End`, `statechart!`, `ToolMachine`, slider/stepper/input builders,
  `TextWindowKit`, `child_emit`; then a read of every handler behind a hit and of the hosts that feed it
  (`RE/🗣️Interpreter`, `RE/🕸️NodeGraph`, `RE/✏️TextEditor`, `RE/🖌️Paint2dHost`, `RE/🌐️World3dHost`, `RE/📐️Canvas2dHost`, wgpu
  `⚙️EngineCanvas` and `🖱️ui/🎯️targets/🧊️wgpu`).
- Every plugin under `✏️s/🔌️plugins/**` (34 directories) and every `OS/**/🗿️artifacts/**` module (flow, workflow, playbook, space,
  infinite/dag: libraries only, no `Emit` call sites) was checked. Framework runtime: `OS/🔌️plugin/🦀️.rs`, `OS/🔌️plugin/⏯️tool-run`.
- Aliases: `PL` = `✏️s/🔌️plugins`; `OS` = `🧰️framework/🛍️products/💻️os/🔨️modules`; `FM` = `🧰️framework/🔨️modules`;
  `RE` = `OS/📺️renderer/🧑‍🎨engine/🧱️elements`; `PLG` = `OS/🔌️plugin/🦀️.rs`; `S1` = `🏅️standards/🔖️1/🪆️subsets/✳️any`.
  Plugin aliases: `PR3` = `PL/🌀️procedural/🗿️artifacts/🧊️generation3d/S1/✏️editor`, `PR2` = `…/🌀️generation2d/S1/✏️editor`,
  `WR` = `PL/✒️writer/🗿️artifacts/✒️writer/S1/✏️editor`, `FO` = `PL/📋️forms/🗿️artifacts/📋️forms/S1/✏️editor`,
  `EN` = `PL/🔋️energy/🗿️artifacts/🔋️model/S1/✏️editor`, `DG` = `PL/🕸️dag/🗿️artifacts/🕸️dag/S1/✏️editor`,
  `SP` = `PL/🪐️space/⚙️engine/🪐️space`, `SQ` = `PL/🎬️sequence/🗿️artifacts/🎬️sequence/S1/✏️editor`,
  `MA` = `PL/➗️mathematical/🗿️artifacts/➗️equation/S1/✏️editor`, `WB` = `PL/🀄️wfc/🗿️artifacts/🖼️bitmap/S1/✏️editor`,
  `W3` = `PL/🀄️wfc/🗿️artifacts/🧊️3d/S1/✏️editor`, `W2` = `PL/🀄️wfc/🗿️artifacts/◻️2d/S1/✏️editor`,
  `TJ` = `PL/🔱️trinity/🗿️artifacts/🔌️jack/S1/✏️editor`, `TR` = `PL/🔱️trinity/🗿️artifacts/♻️rewriting/S1/✏️editor`,
  `GT` = `PL/🌍️gis/🗿️artifacts/🏔️gisterrain/S1/✏️editor`, `RM` = `PL/📸️remodel/🗿️artifacts/📸️remodeling/S1/✏️editor`,
  `RA` = `PL/🖨️raster/🗿️artifacts/🖨️raster/S1/✏️editor`, `PC` = `PL/🏭️process/🗿️artifacts/🧊️process3d/S1/✏️editor`,
  `VC` = `PL/🌿️vcs/🗿️artifacts/🌿️vcs/S1/✏️editor`, `PB` = `PL/📖️playbook/🗿️artifacts/📖️playbook/S1/✏️editor`,
  `LA` = `PL/📏️layout/🗿️artifacts/📏️layout/S1/✏️editor`.

### Classification key

| Code | Shape | Verdict |
|---|---|---|
| **G** | Continuous direct-manipulation gesture (drag, stroke, brush, marquee-with-effect) | MUST convert to a tool machine |
| **C** | Continuous control (slider, held spinner, number field riding the lane) | MUST convert (shared scrub machine) |
| **T** | Typing session (text keystrokes/runs) | convert to typing-run machine (decision §6.1) |
| **I** | Streamed host-fed ingest (async frames arriving over time) | convert for atomicity/zero-trace; low edit value |
| **V** | View state stored as document mutation | move out of history (config), not a tool |
| **O** | One-shot command, one edit | fine; **O\*** = fine but built by scratch-fixture diff / whole-state leaf (leaf-quality flag) |
| **K** | Config / transient / camera | not history; fine |

## 2. How amend and the hosts work today (shared facts)

- `Emit::amend` (`PLG:12888`) puts the tick into `coalesce_key`; `dispatch_emit_inner` turns it into `ArtifactCommand::AmendLast`
  (`PLG:28029`; config twin `PLG:27978`, retained lane `PLG:31533/31559`). The store merges into the last applied, not-checkpointed
  edit with an **equal key string** (`batch_amend_target`). A static key therefore merges two separate gestures that have no other
  edit in between (by reading; no test covers it, `📓️explore-tools-transactions.md` §8.2).
- The applied-edit ledger is a fixed 64 slots (`OS/🌿️vcs/🦀️.rs:196 ARTIFACT_HISTORY_LEDGER_CAPACITY = 64`, refusal text
  `preinstalled fixed applied and revision capacity` at `OS/🏪️store/🦀️.rs:18519` (the store file grew ~130 lines while this audit ran;
  anchor on the string); ticket 26/09/23 F1 measured it: the 65th typed character of the Jack query was refused). One gesture
  of any mutation count occupies ONE slot (store test at `OS/🏪️store/🧪️tests/🔬️unit/🦀️.rs:3712-3735`). An amend into the last
  matching edit does not take a new slot (`let amending = self.batch_amend_target(..)` guard directly above that refusal). **A non-coalesced per-tick or per-keystroke edit is therefore not merely noisy; it
  exhausts the ledger.** This is the decisive fact for §5.2/§5.3.
- **Continuous controls share ONE framework lane.** React `RE/🗣️Interpreter/🟦️.tsx`: `useContinuousTriggerLane` (`:1087`) wraps
  `createContinuousGestureLane` (`FM/🖱️ui/🎬️scene/🟦️.ts:944`), single-flight latest-wins, always sends the release; every tick
  is `change {value, gesture: "<key>:<ms>", commit: bool}`. Consumers: `SliderView` (`:1436`) and `InputView` for `number` with no
  `commit` mode (`:1321`, `continuous = kind === "number" && !commitOnBlur`; blur is the release). A text `Input` with no
  `commit: "blur"` dispatches one plain `change {value}` per keystroke (`:1368-1393`, `renderUiControl :1158`). The NodeGraph
  canvas has its own twin `useGraphSliderLanes` (`RE/🕸️NodeGraph/🟦️.tsx:2255`, `setSlider {widgetId, value, gesture, commit}`).
  wgpu keeps a local `slider_draft_value` (`FM/🖱️ui/🎯️targets/🧊️wgpu/⚡️events/🦀️.rs:2046`) and fires `Change` with a bare value;
  whether it fires per drag step or only on release, and whether it carries a gesture id, was not verified.
- **Node graph.** One shared wire `nodeGraphEdit` (`nodeGraphActions.edit`): React `commitGraphFixture` (`RE/🕸️NodeGraph/🟦️.tsx:972`)
  sends `{setHostSnapshot: <whole fixture JSON>}` after every committed gesture (drag, wire, relayout); SSR fallback
  `onNodeDragStop` (`:1369`) and `onConnect` (`:1378`); wgpu `plan_graph_edits` + `write_graph_edit_action`
  (`RE/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:3843/4093`) sends `{move: {nodeId, x, y}}` × N. The guest cannot tell WHAT was dragged;
  it diffs a whole snapshot. The flow report counts 12 guests on this vocabulary (dag, sequence, procedural 2d/3d, mathematical,
  wfc 2d/3d, space, trinity, architect, flow).
- **Text.** `RE/✏️TextEditor/🟦️.tsx` delivers a typed run through a single-flight outbox (`createTextEditorOutboxV1 :274/:488`,
  ticket 26/09/23 F1/C11 on queue-full). Three modes exist: live splice typing (`textSplice`, writer), live whole-text
  `textEdit` (everything else), and **explicit draft** (`commit: "explicit"`, local undo stack, dirty/conflict state, one dispatch
  on Apply: `textEditorExplicitDraftAction :386`, produced by `TextWindowKit::render_draft`, `PLG:35592`). stdio `txt`, `zip` and
  the registry details windows use the explicit draft; stdio `md` and `html` use the live whole-text mode.
- **ToolRun** (`OS/🔌️plugin/⏯️tool-run/🦀️.rs:1844 publish_tool_run`) is a real state machine but publishes through
  `begin_outbound_apply_batch(..., Some(description), factory, None)` (`:1854-1863`, last arg = `transaction`), stamps only
  `group_id = "toolRun:<n>"` (`:1882`), and labels the row with the tool label resolved to **English only** (`:1852`).

## 3. Already owned (reference only)

| Owner | Covered gestures | Crossing point for other plugins |
|---|---|---|
| puzzle 3d/5d executor | board/world drags, gumball brackets, brush | `World3dHost` phase protocol (SPATIAL decision: non-live gumball = one-shot delta on release, live = stream/commit/abort, brackets + manifest rule deleted) |
| draw + note | D1-D12, N1-N7 (incl. draw opacity slider D11, note ink phases) | D11 is a slider: should reuse the shared scrub machine (§6.2) |
| shooting + lowpoly + fem (+ gen3d and layout gumball) | gumball translate/rotate/scale, lowpoly `transformBegin/End`+`paintStroke*`, fem gumball | **procedural generation3d gumball (`PR3/🎮️commands/🧭️transforms/🦀️.rs:56,70`, `↔️/🔄️/📏️…-selection`) and layout gumball (`LA/🎮️commands/🧭️gumball/🦀️.rs:46,73,99`) are SPATIAL scope; do not reassign.** shooting `🎥️camera` amends are config (`amend_config`), fine |
| flow + cad | F1-F8, C1-C5 | the shared `nodeGraphEdit` gesture record + composed-child transactions (design addendum §12): every node-graph guest in WP-GRAPHS/PROCEDURAL depends on it |

## 4. Framework-level findings

| # | Finding | Where | Verdict / fix |
|---|---|---|---|
| F-1 | ToolRun finalize passes `transaction: None`, stamps `group_id` only, labels en-only | `⏯️tool-run/🦀️.rs:1852,1862,1882` | pass `Some(TransactionRef{id: group_id, tool: tool_id})` (the batch API already takes it), resolve the label per locale (or drop `description` and let leaf labels carry the row like puzzle 2d). 17 tools in 8 plugins per `📓️explore-tools-transactions.md` §4.3 (wfc fill 5, puzzle fill 3, generation previewEval 2, reasoning/trinity/dag reorganize 3, remodel reconstruction, energy simulation) become transactional with no plugin edit |
| F-2 | `Emit::amend`, `Emit.coalesce_key` on the artifact lane, `AmendLast` and the `UtilityPreviewContract` doc still exist and are the documented pattern (a)/(b) | `PLG:12888-12902`, `:13430`, `:28029` | delete for artifact mutations after the last conversion (config `amend_config` stays); extend the `⏯️ToolRunPolicy` predicates (`📜️script.ts`, explore §9.3) to fail on `coalesce_key: Some` / `Emit::amend` / `Emit::commit` with a gesture; greenfield, no shim |
| F-3 | Every continuous UI control is a hidden tool with no owner; handlers that ignore `gesture`/`commit` get one edit per tick | `RE/🗣️Interpreter/🟦️.tsx:1087-1110`; `FM/🖱️ui/🎬️scene/🟦️.ts:944` | generic scrub machine + runtime glue (§6.2) |
| F-4 | Text `Input` default `commit` = none → one `change` per keystroke into a document handler | `Interpreter :1321-1393`; UI contract `InputProps.commit: Option<UiText>` (`FM/🖱️ui/🧬️contract/🧩️component/🦀️.rs:362`) | lint/default: a document-bound `Input` must say `commit("blur")` (Enter also commits) or be a declared typing editor. Offenders (by reading): energy `📌️panels/🔍️inspection/🦀️.rs:133,139,526,572`, forms `📌️panels/🔍️inspection/🧱️controls/🦀️.rs:28,108,126` + `🫥️visibility:50`, norm `🖥️app-surface/🦀️.rs:1126,1141`, space `📌️panels/🔢️parameters/🦀️.rs:65,95,146,167`, raster inspection `:81` |
| F-5 | `TextWindowKit::editable_window_kind` declares `textEdit` as whole-text replace per delivery; md/html map it to `Emit::mutations(SetSnapshot{whole snapshot})`, no key | `PLG:35534-35570`; `PL/🗄️stdio/🗿️artifacts/📝️md/…/✏️editor/🦀️.rs:134`, `🌐️html/…:134` | per-delivery whole-snapshot edit = ledger exhaustion class of ticket F1 *(by reading, not reproduced)*; fix at the kit (§6.1) |
| F-6 | Shared `nodeGraphEdit` wire is snapshot-diff, not intent | see §2 | owner = flow executor; guests follow (WP-GRAPHS/PROCEDURAL) |
| F-7 | Preview of a provisional transaction for *derived* views (procedural mesh preview, energy 3d colouring, dag slider readout) must read an overlay; today they read the amended document | puzzle 2d paints `puzzle2d_select_tool_preview(document, state)` from transient; ToolRun has an overlay (`⏯️tool-run:399,564`) | the scrub/stroke machines need a generic "document ⊕ provisional leaf" projection hook in the runtime (`render_snapshot_or` already exists for time travel, `📋️design.md` §7); otherwise derived previews freeze until release |
| F-8 | Live co-viewing of another user's in-flight gesture disappears once ticks stop being announced | amend announces each tick (`announce_operations`) | product decision: accept, or publish an ephemeral-shared summary through presence (`PresenceToolRun` precedent) |
| F-9 | In-flight gesture scratch kept in **window config** (persisted local-only, own 64-edit ledger) instead of transient | wfc bitmap `WB/🦀️.rs:921-929`; cad engagement (flow+cad scope) | scratch belongs in window transient (puzzle 2d `select_tool`) |
| F-10 | Bracket verbs `transformBegin/End` | SPATIAL deletes them | nothing further |

## 5. Plugin census

### 5.1 Direct-manipulation gestures (G)

| Plugin | file:line | Gesture | Current pattern | Doc/History? | Recommendation |
|---|---|---|---|---|---|
| dag | `DG/🎮️commands/🚚️move-media-node/🦀️.rs:20` | node drag (palette/MCP verb) | `Emit::amend(move_node(x,y), "move-{id}")` per tick, absolute | DOC | covered by node-graph `move-nodes{ids,dx,dy}` tool (WP-GRAPHS); static key merges consecutive drags of one node *(by reading)* |
| dag | `DG/🎮️commands/✏️node-graph-edit/🦀️.rs:54-63` | canvas drag/relayout | `setHostSnapshot` → `dag_snapshot_mutations` diff, absolute per node, one edit, no key | DOC | same tool; leaf `move-nodes` replaces snapshot-diff; `connect`/`deleteSelection` stay one-shot intents (O) |
| space | `SP/🎮️commands/🚚️move-media-node/🦀️.rs:18` | node drag | `Emit::amend(MoveNode, "moveMediaNode:{id}")` per tick | DOC | WP-GRAPHS |
| space | `SP/🎮️commands/✏️node-graph-edit/🦀️.rs:17-58` | canvas gestures | `edit_with_selection` → artifact+config+effects, no key | DOC | WP-GRAPHS |
| sequence | `SQ/🎮️commands/🕸️node-graph/🦀️.rs:21-56` | canvas drag | `sequence_child_emit_from_host_mutation`: whole composed-child `SetSnapshot`-style publish | DOC (child lane) | WP-GRAPHS, needs the composed-child transaction extension (decision 21:43) |
| mathematical | `MA/🎮️commands/🕸️node-graph-edit/🦀️.rs:62` | node move/add/connect/delete | **every** op rebuilds `equation_graph` and emits `ReplaceGraph{whole graph}` | DOC | WP-GRAPHS: one node move must not rewrite the graph; leaves `move-nodes`, `add-node`, `connect`, `delete-nodes` |
| procedural 3d | `PR3/🎮️commands/✏️node-graph-edit/🦀️.rs:42-92` | canvas drag + wire + slider widgets | `setHostSnapshot`/`move` → `commit_host_snapshot` (scratch host diff → `Generation3dMutation` create/update/move-widget…), key only for `setSlider` | DOC | WP-PROCEDURAL: `move-widgets{ids,dx,dy}` tool on the flow record; keep connect/disconnect one-shot |
| procedural 2d | `PR2/🎮️commands/✏️node-graph-edit/🦀️.rs:29-86`, `🚚️move-media-node/🦀️.rs:21`, `🔌️connect-media-ports/🦀️.rs:22` | same | `host_operations` scratch diff, `Emit::mutations` | DOC | WP-PROCEDURAL |
| wfc 2d | `W2/🦀️.rs:444-543,657` (`wfc2d_host_snapshot_edit`, `move_slot`) | node drag | one `move-slot` per gesture chosen by "largest displacement" heuristic from the whole-graph snapshot | DOC | WP-STROKES: `move-slots{ids,dx,dy}`; the heuristic drops every other moved node |
| wfc 3d | `W3/🦀️.rs:705-707,811-921` (`graph_edit_mutations`, `move_slot`) | node drag | one `Emit` of N absolute `move_slot` per released gesture | DOC | same |
| trinity rewriting | `TR/🎮️commands/🕸️node-graph-edit/🦀️.rs:95-102` | node drag/wire | `rewriting_snapshot_mutations(state, next)` scratch diff | DOC | WP-TEXT (plugin-owned) on the flow record |
| wfc bitmap | `WB/🦀️.rs:921-929,935-951` (`accumulate_stroke`, `commit_stroke`, `stroke_mutation`) | paint stroke (drag) | ticks grow a bounding box in the pane's **window config** under `wfc-bitmap-stroke`; release → ONE absolute `set-input-pixels{x,y,w,h,base64(vec![color; w*h])}` | config scratch + DOC commit | WP-STROKES: stroke tool in transient; leaf `paint-input-rect{x,y,width,height,color}` (the pixel array is fully derivable, so the stored payload hides the intent and stomps concurrent edits inside the box) |
| raster | `RE/🖌️Paint2dHost/✍️editing/🟦️.tsx:305-315` → `RA/🎮️commands/🎨️edit-pixels/🦀️.rs:56-217` | brush/eraser/mask stroke | host-local points ref, ONE `editPixels{operation:{kind:"stroke",points≤2048,size,opacity,hardness,color,erase}}` at release; guest publishes the **resulting PNG** as new image asset | DOC | intent is in the request and thrown away at publish. Decision needed (§9): leaf `paint-stroke{layerId,target,points,brush}` with a deterministic rasterizer (replay cost: PNG re-encode) vs leave as one edit and stamp `TransactionRef` only |
| process3d | `PC/🎮️commands/🌍️world/🦀️.rs:70,101`, host `RE/🌐️World3dHost/🟦️.tsx:7326` | push/pull face drag, click-to-cut/drill/attach | host-local drag, ONE `worldFaceDragEnd{distance,normal,startPoint}` → `insert_step_mutations` | DOC | O (one edit, parameter inside the step); no amend. Optionally wrap in a tool machine only to get a `TransactionRef` |
| gis map | `PL/🌍️gis/🗿️artifacts/🗺️gismap/S1/✏️editor/🎮️commands/🗺️features/🦀️.rs:174-186` | `move-feature` | one-shot, `collection_operations(before, after)` scratch diff | DOC | O\*; no host dispatches it as a gesture *(TiledMapHost sends selection/camera/hover only)* |

### 5.2 Continuous controls (C)

The host sends `{value, gesture, commit}` (React) for every row below. "Gesture consumed" = the handler reads `gesture`.

| Plugin | file:line | Control | Current pattern | Gesture consumed? | Recommendation |
|---|---|---|---|---|---|
| energy | `EN/📌️panels/🔍️inspection/🦀️.rs:143-147` (`slider_row`; SHGC/VLT `:296-297`, absorptance/emissivity `:339-359`), number inputs `:133,526,572`, text `:139` → `EN/🦀️.rs:1035-1290` (`reduce`, `model_edit :447`) | ~10 sliders + number/text fields | `Command::Set*Property{id, property, value:String}` via scratch `Model` copy + diff; `Emit` with description, **no key** | no (decode reads `value` only, `EN/🦀️.rs:295-320`) | **worst case**: one document edit per tick/keystroke → ledger exhaustion. Scrub machine for sliders/numbers, `commit("blur")` for text; leaves are already field-parametric |
| forms | `FO/🎮️commands/🩹️patch-questions/🦀️.rs:48` (`patch:{field}:{ids}`), `✏️patch-step:32`, `🎛️patch-question-options:32`, `🩺️patch-vector-field:39`, `♻️update-form:17` (`change-form-title`) | inspector text/number/vector fields | amend with a **static** per-field key; leaf = `ReplaceBlock{stepId, block}` (the WHOLE question block) | no | merge-across-time defect + non-parametric leaf: introduce `patch-question{questionId, field, value}` (+ condition/param variants); typing → blur/Enter commit; numbers → scrub |
| norm | `PL/📕️norm/🖥️app-surface/🦀️.rs:1126,1141` inputs, `:1577-1586` (`commit_snapshot(_fields)`, `Emit::commit`) | per-norm setField/insertItem inputs | `Emit::commit` described edit per change; `XMutation::from_snapshot` decomposition (scratch) | no | blur commit; leaves `change-<field>` already parametric |
| procedural 3d | `PR3/🎮️commands/🩹️patch-flow-widgets/🦀️.rs:29-53`, `🎚️update-generation-values/🦀️.rs:43-44`, `🎚️set-widget-input/🦀️.rs:66`, `✏️node-graph-edit/🦀️.rs:90-92,123` (`setSlider`) | slider widgets, generation values, widget input | amend key `widget-field:{gesture}` / `graph-slider:{gesture}` / `widget-input:{id}:{ch}:{gesture}`; leaf = scratch host diff | yes (keys) | scrub machine; preview re-eval must read the provisional overlay (F-7) |
| procedural 2d | `PR2/🎮️commands/🧬️generation/🦀️.rs:38-43` (`generation-values` static key), `✏️node-graph-edit/🦀️.rs:84-86` | generation values, slider widgets | **static** key `generation-values` for every question/generation (merges across questions) | partly | scrub machine, one leaf `change-generation-value{generationId,questionId,value}` |
| dag | `DG/🎮️commands/🩹️patch-dag-nodes/🦀️.rs:46` | slider node value/min/max live drag | `Emit::amend(change_name|replace_node_kind|resize_node…, "patch-{field}-{ids}")` | no | scrub machine; leaf `set-slider{nodeId, value|min|max}` instead of `replace_node_kind` + `resize_node` |
| gis terrain | `GT/🎮️commands/🏔️exaggeration/🦀️.rs:24` | exaggeration slider | `Emit::amend(ChangeExaggeration, GIS3D_EXAGGERATION_COALESCE_KEY)` static | no | scrub machine; leaf exists |
| space | `SP/📌️panels/🔢️parameters/🦀️.rs:65,95,146,167` → `patch-parameter`, `patch-media-nodes`, `patch-app-instances` | parameter/number/text inputs (`Trigger::Change`, no commit) | one edit per keystroke/tick | no | blur commit (F-4); parameter numbers scrub |
| raster | `RA/📌️panels/🔍️inspection/🦀️.rs:81` | layer property inputs | patch per change | no | blur commit |
| animate | `PL/🎞️animate/🗿️artifacts/🎬️presentation/S1/✏️editor/🎮️commands/✂️patch-tile-crops/🦀️.rs` | crop coordinate `value: f64` | `Emit::mutations(ResizeTileCrop)` per dispatch | no | O unless a slider/drag drives it (driver not located); leaf doc names it "the play app's patch-tile-crops gesture" |
| playbook | `PB/🎮️commands/♻️update-playbook/🦀️.rs:16` | title field | `Emit::amend(change_title, "playbook.title")` | static key | blur commit (single-line field) |
| draw | (assigned) `set-selected-opacity` amend `"opacity"` | | | | use the shared scrub machine |
| flow | (assigned) `patchFlowWidgets` amend `patch-<field>-<ids>` (F6) | | | | same |

### 5.3 Typing sessions (T)

| Plugin | file:line | Path | Current pattern | Notes |
|---|---|---|---|---|
| writer | `WR/🦀️.rs:405,413`; `🎮️commands/📝️text-edit/🦀️.rs:18`; `✂️text-splice/🦀️.rs:33` | prose editor | `Emit::amend(EditText{whole text} / splice_text, "writer-text-edit")`: **static key**, merges across caret jumps and sessions until another doc edit intervenes *(by reading)*; host splice = one typed run `{start,deleted,insert,before,after,seq}` | `EditText` = whole-text, non-mergeable; `splice-text` is positional and mergeable |
| vcs (demo) | `VC/🦀️.rs:372`, `VC/🎮️commands/🩹️edit/🦀️.rs:52-63` (`VCS_TEXT_TYPING_COALESCE_KEY`) | JSON snapshot text editor | typed JSON → `vcs_demo_projection_diff_operations` (scratch diff) amended under `vcs-text-typing` | demo plugin; ticket 26/09/23 F1 workaround |
| trinity jack | `TJ/🎮️commands/✏️text-edit/🦀️.rs:11-18` (`JACK_QUERY_TYPING_COALESCE_KEY`) | query editor | whole query as `set-query{text}` per delivery, static key | the comment cites the 65th-character refusal |
| trinity rewriting | `TR/🎮️commands/👈️set-lhs-json/🦀️.rs`, `👉️set-rhs-json`, `🎛️set-parameter/🦀️.rs:11` | LHS/RHS JSON + parameter fields | `state.clone()` edit + `rewriting_snapshot_mutations` diff, `Emit::mutations`, no key | per-change edits |
| stdio md, html | `PL/🗄️stdio/🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/✏️editor/🦀️.rs:134`, `🌐️html/…/🦀️.rs:134`; kit `PLG:35534` | live text windows (`TextWindowKit::render`, `read_only:false`) | `ReplaceText` → `Emit::mutations(SetSnapshot{snapshot})`, no key | F-5; `txt`/`zip`/details use explicit draft (good) |
| forms | see 5.2 | inspector titles/labels | static-key amend | handled by blur commit |
| playbook | see 5.2 | title | static-key amend | same |
| note/draw | assigned | inline text | | |

### 5.4 Streamed ingest (I) and view state in the document (V)

| Plugin | file:line | Path | Pattern | Recommendation |
|---|---|---|---|---|
| remodel | `RM/🎮️commands/📼️import-video-frame-payload/🦀️.rs:221`, `🖼️import-frame-payload:128`, `✅️import-video-done:31` | host-decoded video import | one `Emit::amend` per decoded frame (`create_asset` + `add_stream_frame`/`create_stream`, JPEG base64) under `remodeling-import:{stream}`, closed by the `done` tick; cancel mid-import leaves half the frames in the document (I) | a machine that Upserts frames keyed by frame index and commits on `import-video-done`; abort = zero trace. Little edit value per frame, real value in atomicity, one row, bounded memory. `💽️import-video-bytes-payload` is already one `Emit` (O) |
| process3d | `PC/🎮️commands/⏱️cursor/🦀️.rs:29,47,68,89`, `🎛️engagement/🦀️.rs:38-40` | timeline cursor `resolved_up_to` (`setCursor`, `stepCursor*`, engagement back/forward/all) | one document edit per click, although its own header says it is "NOT framework History — these move the replay cursor" | V: move `resolved_up_to` to window config; today every scrub is a history row |

### 5.5 One-shot commands built from scratch diffs or whole-state leaves (O\*)

Fine as one edit; flagged because the leaf is not the intent, so a history edit of "its inputs" edits an opaque blob.

| Plugin | file:line | Path | Why flagged |
|---|---|---|---|
| procedural 3d | `PR3/🎮️commands/🥽️edit-mesh-selection/🦀️.rs:69-78` (`amount`,`cuts`,`dx..dz`), `🔪️knife-mesh-selection:32-40`, `🗺️reorganize:19` | insert a mesh-op neuron | scratch host + `commit_host_snapshot`: inputs end up inside `create-widget` params; leaf `insert-mesh-operation{operation, targets, params}` would make them editable |
| mathematical | `MA/🎮️commands/🧮️set-algorithm/🦀️.rs:19` | `Emit::commit(ReplaceGraph, "setAlgorithm")` | whole-graph replace |
| energy | `EN/🦀️.rs:447` (`model_edit`), `:1035-1290` | every setter copies the `Model`, mutates, diffs | leaves are field-level (fine) but stringly `value` |
| forms | see 5.2 | `ReplaceBlock` whole block | fix in WP-CONTROLS |
| block | `PL/🧱️block/🗿️artifacts/🧊️3d/S1/✏️editor/🎮️commands/🎨️edit/🦀️.rs:12` | `replace_document_operations` (example/whole-document load) | load, not a gesture |
| trinity rewriting | see 5.3 | | |
| norm | `🖥️app-surface:1540-1590` | `import_media`, `commit_snapshot_fields` decomposition | scratch decomposition of `set-snapshot` |
| gis map | `…/🗺️features/🦀️.rs:135-236` | add/move/rename/delete feature via `collection_operations` diff | |

### 5.6 Fine as is (K and intent one-shots)

| Plugin | file:line | Why |
|---|---|---|
| shooting | `…/🎮️commands/🎥️camera/🦀️.rs:93,112` (`Emit::amend_config`) | camera + draft label are config, not history |
| gis map/terrain, energy, layout viewers | `gismap/👁️viewer/🦀️.rs:103`, `energy/👁️viewer/🦀️.rs:133`, `EN/🦀️.rs:1275` | camera on window config with coalesce key |
| wfc bitmap | `WB/🦀️.rs` `set_active_color`, `StrokeBegin/Extend` config write | active colour = pane config (the stroke scratch itself is F-9) |
| forms | `FO/🎮️commands/🎯️set-try-value/🦀️.rs:320` | transient |
| fem | result-animation tick, `🫧️transient` | transient (gumball is SPATIAL) |
| procedural 2d | `⬇️/⬆️/🖱️canvas-pointer-*` | no-ops (no drag on this canvas) |
| layout | `canvas-pointer-*` | hover hit-test only; `canvasDrop` is a one-shot drop (O) |
| vcs, reasoning | `canvas-pointer-*` | no-ops; reasoning `add-relationship` one-shot |
| animate | `canvas-pointer-down` | selection effect only |
| architect | `🕸️graph:15-56` | node-graph verb handles only `connect`/`deleteSelection`, both intent one-shots |
| playbook, forms, imperative, raster | `move-step`, `move-block`, `move-question`, `move-layer` | drag-and-drop list reorder dispatched once on drop with `{id, target, index}`; already parametric intent (O) |
| sourcing | `drop-on-pool`, `drop-on-curated` | one-shot drops |
| block | `place-vortex`, `set-brush-*` | one click, brush options in config |
| norm, stdio | one-shot field/format commands | O (see F-4/F-5 for the input binding) |
| demonstrator | only descriptor JSON mentions `transformBegin/End` | generated text from the puzzle roster, not source |
| wfc grid2d/grid3d, fem, stdio editors | click-to-pin / structured edits | O |

## 6. Design recommendations

### 6.1 Typing sessions: decision and reasoning

**Recommendation: typing is a tool machine. A typing run yields ONE net leaf per commit; do not keep `Emit::amend`.** Keep the
live splice delivery to the guest only as the *input event stream* of the machine.

Shape (reusable, `🛠️tool-machine` combinator):

- Events: `Edit{splice | text}`, `SelectionJump`, `Blur`, `Enter`(single-line), `Apply`(explicit), `OtherVerb`, `BaseMoved`, timer `idle`.
- Machine state: the run's net edit (for prose: composed `splice-text{start, deleted, insert}`; for single buffers: the final text).
- Effects: `Upsert{key:"text", <net leaf>}` on each edit (replace-by-key keeps ONE entry), `Commit` on idle (≈ 750-1000 ms),
  caret jump away from the run, blur, pagehide/visibilitychange, explicit apply, any non-typing verb; `Abort` only on `baseMoved`
  conflict or frozen time-travel. `ToolMachine` already supports timers through the `Host` (`📓️w1-c-report.md` §2).
- Preview: the host editor buffer is the preview (already true), plus window transient for the pending run; peers follow
  through presence, not through the store.

Why, grounded in undo/time-travel semantics and in the tree:

1. **The unit a human undoes and a reviewer edits is a run, not a keystroke.** Amend makes one *edit* but keeps N micro-mutations
   inside it; the history panel would offer N "editable" rows whose inputs (one inserted character at a shifted offset) are
   meaningless and whose downstream replay is position-dependent. A net `splice-text` has ONE editable input, the typed text.
2. **A static key is not a session.** `"writer-text-edit"`, `"vcs-text-typing"`, `"jack-query-typing"`, `"playbook.title"`,
   forms `patch:{field}:{ids}` merge any two runs that have no other document edit between them (`batch_amend_target` compares
   the key string), so "undo" and "edit this input" address an arbitrary time span. A run needs an identity, which is exactly what
   `ToolTransaction` mints (`TransactionRef{id, tool}`).
3. **The ledger forces run granularity either way.** Per-keystroke edits are impossible (64 slots, ticket F1: the 65th character
   was refused). Amend works around it by making every keystroke durable and announced while hiding it inside one slot; a commit
   per run costs one slot per run and announces one envelope. Idle commit every ≤ 1 s keeps the slot budget at "one per pause",
   the same as any editor's undo coalescing.
4. **Abort semantics differ from drags but the machine still fits.** Typing has no "cancel = zero trace" gesture, so the machine
   almost never aborts; its value is identity, net leaf and bounded commits, not rollback.
5. **The framework already shipped the pattern**: `TextWindowKit::render_draft` + `commit:"explicit"` (stdio `txt`, `zip`, details
   windows) is a typing session that yields one `set-text` on Apply. The recommendation generalizes it: explicit Apply for
   structured text (JSON/markup editors: vcs, trinity rewriting JSON, stdio md/html/json), idle-run commit for prose (writer,
   jack query).
6. **Collaboration cost and mitigation.** Amended ticks are visible to peers live; a run commit shows peers the text at the run
   boundary. That is a real regression for co-editing prose. Mitigation: ≤ 1 s idle bound, peers' caret/selection via presence,
   optional ephemeral-shared run preview (F-8). If co-editing latency is a hard product requirement, the fallback is **not** a static
   key: mint a per-run key, stamp `TransactionRef` on the run, and accept N micro-mutations (degraded option B). I do not recommend B
   because it keeps non-editable micro-mutations in history.
7. **Crash/unload risk.** A run pending for < 1 s is lost on hard crash; flush on `pagehide`/`visibilitychange`/blur.

Short single-line fields (titles, names, labels) need no machine at all: `commit("blur")` (+ Enter) = one dispatch = one edit.

### 6.2 Continuous controls: one scrub machine

Every slider/held-spinner already produces the complete tool protocol (`gesture` id = transaction identity, `commit` = release,
single-flight latest-wins tick). Build ONE generic machine instead of per-plugin ones:

- `ScrubMachine<M>` in `🛠️tool-machine` (reusable combinator, statechart `idle → scrubbing`): events `Tick{gesture, value}`,
  `Commit{gesture, value}`, `Abort{reason: blur|captureLost|frozen|baseMoved|retired}`; yields `Upsert{key: target, leaf(value)}`
  replace-by-key, `Commit`. The plugin supplies only `fn leaf(args) -> M`.
- Runtime glue (`PLG`): route an action whose args carry `gesture` (+ `commit`) into a runner persisted in the window transient
  exactly like puzzle 2d `select_tool` (stable-id configuration, base revision, open `TransactionRef`), emit
  `Emit::commit_transaction` on commit, `ui_scope` of the stream phase kept (procedural's `slider_gesture_ui_scope`).
- The leaf is the ABSOLUTE set (`set-x{target, value}`): for a slider the user's intent *is* the target value, relative deltas
  would be wrong. The transaction keeps the net value; history edits the value with the W1-E slider/stepper widgets.
- wgpu: if the slider fires per drag step with no gesture id, add `gesture`/`commit` (verify first).
- Consequence: draw D11, flow F6, gis, dag, procedural, energy, forms all reduce to a leaf constructor.

### 6.3 Node-graph guests

The flow executor defines the gesture record (`{gestureId, kind: move, nodeIds, dx, dy}`) and the composed-child transaction.
Each guest then implements a parametric leaf (`move-nodes{ids, dx, dy}` modelled on puzzle 2d `drag-selection`), a `node_drag`
machine (reuse the flow one), and deletes `setHostSnapshot` adoption. `connect`/`disconnect`/`deleteSelection` stay intent
one-shots. Not before flow lands.

## 7. Work packages (ordered by user impact)

Ranking = gesture frequency × defect severity (broken > wrong granularity > wrong leaf). Sizes: S ≤ 1 executor-day, M ≈ 2, L ≈ 3+.

| Order | WP | Scope (plugins) | Gestures (table ids) | Depends on | Size |
|---|---|---|---|---|---|
| 1 | **W3-T2-CONTROLS** | framework scrub machine + runtime glue + `Input` commit lint (F-3, F-4); **energy**, **forms**, **norm**, **gis terrain** (+ **playbook** title, a one-line blur commit) | 5.2 energy/forms/norm/gis/playbook; F-3, F-4 | none (publish the machine API to draw/flow executors first) | L |
| 2 | **W3-T2-TEXT** | typing-run machine + explicit-draft policy in `TextWindowKit` (F-5); **writer**, **vcs**, **trinity** (jack query + rewriting JSON/parameter + rewriting node graph), **stdio** `md`/`html` (and the kit for `json`/`binary`/`deflate`) | 5.3; trinity rewriting node-graph | none, except rewriting node graph waits for the flow record | M |
| 3 | **W3-T2-PROCEDURAL** | **procedural** generation2d + generation3d: sliders (scrub), generation values, widget inputs, node-graph moves, scratch-diff one-shots → parametric leaves | 5.1 procedural, 5.2 procedural, 5.5 procedural | WP-CONTROLS scrub machine, flow node-graph record, SPATIAL (gen3d gumball) | L |
| 4 | **W3-T2-GRAPHS** | **dag**, **sequence**, **space**, **mathematical** (+ space parameter inspector blur commit) | 5.1 dag/space/sequence/mathematical | flow executor: gesture record + composed-child transactions (sequence) | M |
| 5 | **W3-T2-STROKES** | **wfc** (bitmap stroke → `paint-input-rect`, stroke scratch config → transient; 2d/3d `move-slots`), **raster** (stroke leaf decision), **remodel** (import transaction), **process** (cursor → config; world drag wrap) | 5.1 wfc/raster, 5.4 | flow record for wfc 2d/3d | M |
| 6 | **W3-T2-CLOSURE** | ToolRun `TransactionRef` + localized label (F-1); delete `Emit::amend`/`AmendLast` artifact path/`UtilityPreviewContract`; policy gate on `coalesce_key`/`Emit::commit` gestures (F-2); `Input` commit lint enforcement; docstring/taxonomy/descriptor regeneration list | F-1, F-2, F-4 | all previous WPs | S |

Start now (no cross-dependency): **CONTROLS, TEXT (minus rewriting graph), STROKES (minus wfc graph), F-1 of CLOSURE** (one-line, can
land immediately and unblock ToolRun rows). Then PROCEDURAL and GRAPHS when the flow record is published. CLOSURE last.

File-ownership conflicts to pre-arrange:

- `PR3/🦀️.rs` (dispatch table, `generation3d_retained_reduce`) is touched by SPATIAL (gumball) and WP-PROCEDURAL: sequence SPATIAL first.
- `PLG` (`Emit`, dispatch) is touched by WP-CONTROLS (scrub glue), W2-A follow-ups and WP-CLOSURE: hot file, use unique-anchor Edit calls only.
- `RE/🗣️Interpreter` and wgpu slider: WP-CONTROLS only; draw/flow must not patch them.
- Every WP: `nodeGraphEdit` callers (`RE/🕸️NodeGraph`, wgpu `EngineCanvas`) belong to the flow executor, not to these WPs.

Per-WP acceptance (shared, from the W3-T brief): schema-first parametric leaves with `x-semio-ui` en/de, wire witnesses, payload law,
labels via `SemanticMutation::label`, one gesture = one edit = one row with `TransactionRef`, cancel = zero trace, two gestures = two
transactions, replay determinism through `state_before`/`begin_report_replay`, host phases in both React and wgpu with a shared
corpus, `schema mutation-inputs`/`mutation-payloads` stay 0, `cargo check --target wasm32-wasip2`.

## 8. Corrections to the status-log wave-2 list

| Status-log plugin | Finding |
|---|---|
| forms | confirmed, but the amend is typing/inspector, plus a whole-block `ReplaceBlock` leaf (WP-CONTROLS) |
| procedural | confirmed; gumball belongs to SPATIAL, the rest to WP-PROCEDURAL |
| remodel | confirmed: import tick amend (5.4), not a pointer tool |
| writer | confirmed (typing) |
| dag | confirmed (move amend, slider amend, node graph) |
| space | confirmed (move amend, node graph, parameter inputs) |
| trinity | confirmed (jack typing; rewriting typing + node graph) |
| playbook | trivial: one title field amend |
| **block** | **no gesture**: `place-vortex` is one click; no amend; drop from the wave |
| **process** | **no amend**: only the in-document replay cursor (V) and one-shot world drags |
| mathematical | not amend; node-graph `ReplaceGraph` whole-graph rewrite |
| norm "scratch" | not scratch: per-change `Emit::commit` from unbound-commit inputs |
| **missing** | energy (worst: per-tick/keystroke edits), vcs, gis terrain, wfc (bitmap stroke in window config; node-graph move-slot), raster (stroke published as image), sequence (child-lane node graph), stdio md/html live text, animate (driver unknown) |

## 9. Not verified / open points

1. **No runtime evidence** for any per-tick claim (energy sliders producing one row per tick; static-key merging of consecutive
   gestures; md/html whole-snapshot edit per delivery hitting the ledger). Each is a strong reading of code + the ticket 26/09/23
   F1 precedent; a two-minute probe per item would confirm.
2. wgpu slider: fire cadence and gesture id not verified (`FM/🖱️ui/🎯️targets/🧊️wgpu/⚡️events/🦀️.rs:2046-2086`).
3. animate `patch-tile-crops` driver (slider, handles or MCP) not located.
4. raster stroke leaf: performance budget of replaying a rasterizer + PNG encode inside `state_before` / Report replay is unknown;
   recommend the WP-STROKES executor prototype before committing to the leaf.
5. The wave-2 list in the status log and plan was taken from `📓️status.md` 21:29; other executors may already have moved files
   named here.
6. stdio `md`/`html` still map typing to a whole-snapshot `SetSnapshot` leaf (design §11 treats `SetSnapshot` as banned for
   writer): a splice-style leaf per text artifact is the long-term fix and is **not** in WP-TEXT's scope.
7. Product decisions to confirm: typing run commit on idle (§6.1) vs degraded option B; loss of live co-viewing of in-flight
   gestures (F-8); process3d cursor leaving history (5.4).

## 10. Files read (evidence index)

`🧭️plan.md` W3-T brief, `📓️explore-tools-transactions.md` §4-§9, `📓️w2-d-report.md` §1, `📋️design.md` §5-§10, `📓️status.md`,
`📓️w3-t-flow-cad-report.md`, `📓️w3-t-draw-note-report.md`, `📓️w1-c-report.md`; `PLG` (Emit, dispatch_emit_inner, TextWindowKit),
`OS/🔌️plugin/⏯️tool-run/🦀️.rs`, `OS/🏪️store/🦀️.rs` + unit tests (ledger), `OS/🌿️vcs/🦀️.rs:196`; `RE/🗣️Interpreter`, `RE/🕸️NodeGraph`,
`RE/✏️TextEditor`, `RE/🖌️Paint2dHost`, `RE/🌐️World3dHost`, wgpu `⚙️EngineCanvas` and `🖱️ui/🎯️targets/🧊️wgpu`, `FM/🖱️ui/🎬️scene/🟦️.ts`;
plus the plugin files quoted with line numbers above.
