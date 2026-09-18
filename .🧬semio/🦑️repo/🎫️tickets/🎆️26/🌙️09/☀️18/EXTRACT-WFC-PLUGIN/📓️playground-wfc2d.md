# 🎮️ wfc2d playground — end to end (slice B3)

Variant `wfc2d`, app `s.wfc.wfc2d@1/*#editor`, served by the coordinator at
`http://127.0.0.1:6043/?plugin=wfc` (react renderer). Every claim below was produced by a probe run
recorded under `🗑️generated/playground-wfc2d/`; nothing here is inferred from source alone.

---

## 1. What the first boot found

`🐍️wfc-console-dump-probe.mjs` against the served build (log `🗑️generated/wfc2d-boot/`):

- the shell booted (`data-semio-os-ready = "wfc2d"`), both window hosts mounted with canvases and no
  host fault — `window:wfc-graph` (NodeGraph, 2 canvases) and `window:wfc-2d-preview` (Canvas2d, 1);
- **fault 1** — `setActiveExample refused: undeclared-action (user window=wfc-graph)`: no window kind
  of the app declared the id the navbar example picker dispatches, so the picker was dead;
- **fault 2** (not visible in the console, found by reading the framework) — **every other verb was
  dead too.** `wfc2d` declared its actions as `ActionDefinition::bounded_catalog` and shipped NO
  app-owned retained tool route, so each dispatch resolved to `QualifiedToolProof::Bounded` and
  faulted inside `TypedCommandFullOperationJob::step` with *"generic bounded proof has no resumable
  app-owned reducer job"*. Memory `project-bare-bounded-factory-means-every-action-dead` names this
  exact class;
- **fault 3** — `ArtifactEditor::command_from_action` was not overridden, so the trait default
  answered `app.command.unsupported` for every action id before the proof was even reached
  (pipeline-report pitfall #13);
- **fault 4** — the graph pane rendered one unreadable speck at the pane centre and the preview pane
  rendered a ~4-pixel mark: slot coordinates are authored in DOMAIN units (a corridor two units
  wide), which at camera zoom 1 are two PIXELS;
- **fault 5** — nothing ever dispatched `set-solve`, so the transient the preview reads was always
  `Wfc2dTransient::default()` and the preview could never show a solve (`📓️audit-wfc2d.md`'s
  blocking gap 5).

---

## 2. What landed

All in `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/`.

### 2.1 The retained tool route (`✏️editor/🦀️.rs`, region `🧵️Retained`)

The whole reason the pane was inert. Modelled on `📸️remodel`'s SDK route (not puzzle2d's bespoke
job), so the artifact owns as little machinery as possible:

| piece | what it is |
|---|---|
| `WFC_2D_RETAINED_TOOL_IDS` | 20 verbs: the 15 document kinds, `change-camera`, `change-active-tile`, `solve`, `setActiveExample`, `nodeGraphEdit` |
| `OpBinary::TOOL_JOB_IDS` | the same slice — `validate_tool_job_rows` demands `TOOL_JOB_IDS ∩ migrated == proofs` or the app boots `interactive-job.catalog-incomplete` |
| `WFC_2D_PUBLICATION_CONTRACTS` | one lane per route: `Artifact` ×16, `Config` ×2 (camera, armed tile), `Transient` (`solve`), `HostOnly` (`setActiveExample`) |
| `Wfc2dCommandWork` | the one bounded `ArtifactCommandWork` step — runs the pure `dispatch` reducer and, for `solve`, `CompleteWithEphemeral` carrying the transient mutation |
| `Wfc2dRetainedCommandJobFactory` | the app-owned `ToolJobFactory`, bound by `factory_type:` in `bounded_first_step_tool_proofs!` — that binding is what turns a bare proof into an exact-owner proof |
| store preparations | `build_artifact_store_one_item_preparation_factory`, `…config…`, `build_transient_store_one_item_preparation_factory`, `build_transient_local_root_retirement_factory` — a declared lane with no preparation is registered as an *unsupported publication contract* and stays dead |
| `command_from_action` | the `DslValue` args bridge for all 20 ids |
| `command_id` | `wfc2d_command_id` — `dispatch_typed_command_inner` rejects a command whose id ≠ the admitted action |

### 2.2 `setActiveExample` — a load, not an edit

`Wfc2dEditorCommand::SetActiveExample { example_id }` answers ONE
`semio_framework::kernel::Effect::LoadDocument { pack, spr }` built from the named example
(`wfc2d_example_document`), on a `HostOnly` lane. A `LoadDocument` is not a mutation, so re-picking
the example the app booted on mints no history entry at all — the "declared-state diff" pitfall #3
asks for `upserts=0, canUndo=false`, and a whole-document load gets there structurally instead of by
diffing. `🗒️note` uses the same effect; `📸️remodel` uses the mutation-diff shape, which wfc2d cannot
(this artifact has no whole-document-replace mutation, and a 15-verb replay would be 30+ edits).

The load also needed `build_document_store_initialization_job` → the SDK's
`bounded_document_store_initialization_job`. Without it the framework refuses every persisted
replacement with `artifact-store.persisted-initializer-refused`, which is what the first fix attempt
hit live.

An unknown example id is a named fault (`wfc2d.example.unknown`), never a blank document.

### 2.3 The solve reaches the preview

`solve` is a catalogued `Migrated` action on the **preview** window (`.window_kind_actions`). Its
work step calls `solve_transient`, which drives the artifact's own
`crate::inferences::solve_with_job` headless adapter to completion and answers one
`Wfc2dTransientMutation::SetSolve` carrying the assignment in DOCUMENT slot order plus the
contradiction verdict. The framework publishes it on the declared `Transient` lane, and
`render_with_request_context` (which already threaded the lane) paints it.

Proved live: the preview pane's pixels change after `solve`, screenshotted with the Actions pane
**closed on both sides** so the panel's own pixels cannot fake the diff
(`🗑️generated/playground-wfc2d/interact/2a-preview-unsolved.png` vs `2b-preview-solved.png`).

### 2.4 Both canvases are legible

- **preview** — `preview_fit` maps the document's own slot bounds onto a 640-unit box centred on the
  world origin (uniform scale, so tile media never stretches). Duplicated verbatim into the viewer's
  own board window, because `policyViewerPurityBreaches` forbids the viewer importing the editor's.
  Two laws cover it: the transform a solved tile is drawn with is derived from `preview_fit`, and
  every bundled example is centred on the origin and spans > 600 units.
- **graph** — `WFC_2D_GRAPH_VIEW_SCALE = 140.0` document→canvas units, applied in `Wfc2dGraphView`
  (the artifact's adapter) and divided back out in `wfc2d_host_snapshot_edit`. It lives in the
  artifact, never in the shared window file, so `wfc3d` keeps its own constant. B5 landed the same
  idea independently as `WFC_3D_GRAPH_UNIT = 120.0`.

### 2.5 Canvas gestures reach the document

The React `NodeGraph` host has three engines. `wfc2d` gets the **wasm dag surface**
(`useFlowEngine` is false: the scene declares neither `capabilitiesJson.engine == "flow"` nor a
`hostSnapshotJson`), which does not send the `move`/`connect` sub-ops the SSR `Diagram` fallback
sends — it sends the WHOLE graph as `nodeGraphEdit` `{operation:"setHostSnapshot", hostSnapshotJson}`.
`wfc2d_node_graph_edit` therefore handles both vocabularies, and `wfc2d_host_snapshot_edit` diffs the
`DagHostSnapshot` against the document:

1. a wire the canvas holds that the document does not → `connect-slots` with a fresh `{from}-{to}` id;
2. a document adjacency the canvas no longer holds → `disconnect-slots` by the document's own edge id;
3. otherwise the node with the LARGEST displacement → `move-slot` (one gesture is one edit; the
   canvas relayouts the rest).

Two guards, the second straight from B5's warning: adjacency is **undirected**, so `(a,b)` and
`(b,a)` are one adjacency and never a removal-plus-addition; and a snapshot carrying no wires at all
while the document carries several is a canvas that has not finished syncing, never a user who
deleted every edge in one gesture.

### 2.6 A framework fix the gesture needed (`🕸️NodeGraph/🟦️.tsx`)

**The wasm node-graph surface never told any plugin about a drag or a wire.** `commitGraphFixture`
was called from the selection-align chrome and nowhere else, so `onPointerUp` discarded every
gesture. Added `graphEditSignature()` (node ids/positions + wire endpoints, deliberately NOT camera,
selection or hover), captured at pointer-DOWN and compared at pointer-UP by
`commitGraphFixtureIfEdited`. A gesture that only panned, zoomed or picked sends nothing, so no
plugin gains traffic it did not have; a gesture that really changed the graph now reaches the guest.
This is framework-wide and benefits `dag`/`trinity`/`wfc3d` alike — flagged for the coordinator.

### 2.7 Palette verbs are usable and guarded

- Every entity-naming verb now carries a staged argument form. For the nine `wfc-graph` verbs the
  forms live in the shared window file (`slot_arg`/`x_arg`/… — all artifact-agnostic, so `wfc3d`
  still copies it verbatim; **noted for B5**, who has already re-mirrored and reports an empty diff);
  the tile/rule verbs get theirs from `.action_args(...)` in `create_wfc2d_editor`.
  Before this, activating `pin-slot` dispatched `id: ""` the instant the row was clicked and minted
  a history row ("Pin to corridor") against a slot that does not exist.
- `dispatch` now refuses an id the open document does not hold, by name:
  `wfc2d.slot.unknown-slot`, `wfc2d.tile.unknown-tile`, `wfc2d.edge.unknown-edge`,
  `wfc2d.rule.unknown-rule`, `wfc2d.id.taken`. A palette default is static and outlives the example
  it was authored against (pipeline-report pitfall #6); a named refusal reads as what it is.

### 2.8 An interaction domain

`.interaction(InteractionDefinition { id: "slot", … })` plus
`.window_kind_interactions(wfc-graph, ["slot"])` and an `interaction_topology` over the document's
slot ids. Declaring one domain is what injects the framework's own
`interactionSelect`/`interactionHover`/`clearSelection`/`selectAll`; without it every node click on
the canvas is dropped as `undeclared-action`.

---

## 3. Probes

Cloned into `$T` and adapted to wfc2d's ids; outputs under `🗑️generated/playground-wfc2d/<probe>/`.

| probe | what it proves |
|---|---|
| `🐍️wfc2d-console-dump-probe.mjs` | boot: beacon, both hosts, canvases, example options, screenshot, `faults: []` |
| `🐍️wfc2d-interact-probe.mjs` | 11 steps, each with its own console delta and fault array |
| `🐍️wfc2d-viewer-probe.mjs` | Viewer role renders `wfc-2d-board` with a canvas, and the Editor role comes back with both windows |

The interact probe's own instruments, worth reusing:

- **`state()` reads the History panel.** The React host logs only refusals, so "did an edit land" is
  read from `framework.history.entry.*` rows, not from the console. `FRAMEWORK_ROW` filters the
  shell's own chrome rows (Toggle Panel, Activate Window, Clear Selection, Undo…) out of any
  "document edit" assertion.
- **Screenshots are taken with the Actions pane closed.** The pane overlays the window, so a naive
  before/after diff around an action reports "repainted" for every action ever run. The first
  version of this probe did exactly that and was wrong.
- **Nodes are found by hovering.** The wasm canvas exposes no DOM node; the dag host logs
  `dag hover changed: <id>` on entry, so the probe sweeps the pane, refines around the first hit and
  answers the centroid.
- **`shell.*` refusals are counted separately** (`shellNotes`) — see §4.
- **A step may declare a refusal it is PROVING** (`note(..., expected)`), which is how the
  unknown-id guard is evidence rather than a fault.

### 3.1 Final run (all 11 steps, `faults = 0`)

| step | result |
|---|---|
| boot | ready, both hosts, no fault |
| solve | Actions row ran; preview signature `9f68c827e26a8575` → `3372916e2b23145b` with the pane closed on both sides |
| example-switch | picked `Terrain Ring` (the bitmap example); BOTH panes repainted; navbar label followed |
| example-reselect-boot | picked `Two Rooms And A Corridor`; `documentEntriesMinted: []` — **no phantom edit** |
| graph-drag | gesture sweep from the node; landed `Disconnect edge-a-corridor` (the wire runs through the node centre) |
| undo | `Meta+z` reverted it — the reverted row is journalled and `Undo` appears |
| graph-connect | reach sweep; landed `Disconnect edge-corridor-b` and **`Move slot room-a`** |
| action-pin-slot | form filled (`id=room-a`, `tileId=room`), executed → `Pin room-a to room` |
| action-unpin-slot | form filled, executed → `Unpin slot room-a` |
| action-connect-slots | form filled, executed → `Connect room-a to room-b` |
| guarded-unknown-id | `delete-slot id=no-such-slot` → `wfc2d.slot.unknown-slot`, `mintedNothing: true` |

So through the live canvas: **`move-slot` ✅, `disconnect-slots` ✅, undo ✅**; through the Actions
forms: **`connect-slots` ✅, `pin-slot` ✅, `unpin-slot` ✅, the guard ✅**.

---

## 4. Fault ledger — what is NOT wfc2d's

1. **`shell.windowActivate` (and its `shell.*` siblings) are dropped as `undeclared-action` by every
   app in this repo.** `ShellHost.noteShellCommand` records a history row whose `actionId` is
   `shell.windowActivate`; the replay of that row dispatches the id itself, and no plugin anywhere
   declares any `shell.*` id (`grep` over `✏️s/🔌️plugins` finds none). Framework-owned: the id
   belongs in `FRAMEWORK_RESERVED_ACTION_IDS` in `🛠️ShellHelpers/🟦️.tsx`. Counted as `shellNotes`,
   never hidden.
2. **The viewer's boot `setActiveExample` is still refused** —
   `app "s.wfc.wfc2d@1/*#viewer" dropped action "setActiveExample" … (window kinds: wfc-2d-board)`.
   This is pipeline-report pitfall #5 verbatim, whose stated framework half ("ShellHost skips the
   boot announcement when `undeclaredActionDiagnostic` says no window kind declares it") is **not
   in effect on this tree**. Giving the viewer a real `setActiveExample` means a second retained
   route on a read-only surface; left undone deliberately and reported. The viewer otherwise renders
   and round-trips to the Editor cleanly.
3. **A slot node cannot be dragged by its centre on the wasm dag canvas.** `fit_node_size` overwrites
   the plugin's width/height from the node kind's port layout, and a node with one input and one
   output is *entirely* port cells — so `pointer_down_screen` reaches `world_hits_handle` (wire) or
   the engine's marquee before `try_node_rectangle_pointer_down` (drag). The bounded selection-union
   drag that would otherwise catch it is gated on `DagDrawLod::Minimap`. A drag does land
   `move-slot` from the offsets that miss the ports (proved above), but "press the node, move it" is
   not reliable for a ports-only node. Framework-side.
4. **A wire the canvas DRAWS was not provable headlessly.** `dag port press … interaction=draw-edge(new)`
   is reachable (the probe logged it), but landing the release on the partner's port anchor needs the
   anchor's screen rect, and the session's `entityScreenJson`/`pickTargetsAtScreenJson` are not
   reachable from the page. B5 hit the same wall. The suggestion for whoever takes it: have
   `WasmGraphSurface` mirror `session.entityScreenJson("handle", …)` for the visible ports onto a
   `data-port-rects-json` attribute on the host element — the surface already publishes
   `data-selection-json` the same way, and a probe could then aim exactly. The `connect-slots`
   lowering itself is covered by a unit test and by the Actions-form path.
5. Peer breakage seen and waited out, not mine: `semio-framework-os-infinite` E0502 (fixed by the
   coordinator mid-slice), `semio-s-artifact-wfc-bitmap` `ActionArgDef::boolean`, and
   `semio-s-artifact-wfc-3d`'s viewer declaring editor-only trait methods.

---

## 5. Gates

Logs in `🗑️generated/playground-wfc2d/`.

| gate | result |
|---|---|
| `RUST_MIN_STACK=33554432 cargo test -p semio-s-artifact-wfc-2d --features component-app-assembly --lib -j 4` | ✅️ **190 passed / 0 failed** (`test-8.log`) — 174 at slice A3, +16 here |
| `cargo check -p semio-s-plugin-wfc --lib -j 4` | ✅️ (`check-plugin-1.log`) |
| `CARGO_PROFILE_WASM_DEV_DEBUG=false cargo check -p semio-s-artifact-wfc-2d --features component-app-assembly --lib --target wasm32-wasip2 -j 4` | ✅️ (`check-wasm-6.log`) |
| `bun nx run @semio-tech/framework-os-dev:activate-wfc2d-react-dev` | ✅️ (`activate-8.log`, 0 errors) |
| boot probe | ✅️ `faults: []` |
| interact probe | ✅️ 11 steps, `faults 0`, `shellNotes 0`–`1` |
| viewer probe | ⚠️ renders and round-trips; one fault, §4 item 2 |

New tests added here (all in `✏️editor/🧪️tests/🔬️unit/🦀️.rs` and the preview window's own suite):
a dragged node lands exactly one `move-slot` with the canvas' units divided back out; a drawn wire
lands exactly one `connect-slots`; a removed wire lands exactly one `disconnect-slots`; a
camera-only gesture mints nothing; six verbs against unknown ids are refused by their exact fault
codes; the solve verb answers a full, contradiction-free assignment over the boot example; the
example picker answers `LoadDocument` for all four examples and refuses an unknown id; the fit
centres every example on the origin.

---

## 6. For the other variants

Every artifact in this plugin has the same two holes wfc2d had, and they are invisible until a
playground boot: **no app-owned retained tool route** (so every verb faults in
`typed-command-reducer`) and **no `command_from_action`** (so every verb faults before that). Copy
`🧵️Retained` + the four store-preparation hooks + the args bridge; the only per-artifact parts are
the tool-id roster, the lane table and the arg forms. `grid2d` additionally runs its solve into a
window config rather than a transient — same machinery, different lane.
