# 🧲️ wgpu NODE-GRAPH SURFACE RETENTION — the window instance owns the surface, not the painted frame

Lane `wgpu-node-graph-surface-retention`, ticket `26/09/09/PROCEDURAL-3D-END-TO-END`, 2026-09-13.

Continues `📓️wgpu-retained-controls-wires-2026-09-13.md` §7.3(b) — the blocker that lane could name but
not close.

---

## 1. TL;DR

The wgpu shell mirrored its three bespoke pointer-state maps (`node_graph_states`,
`tiled_map_states`, `board2d_states`) from the per-frame PAINT drain and **evicted every surface the
drain did not mention**. `world3d_states` was never a drain projection and its input worked
throughout. That asymmetry is now closed: an engine surface's pointer state is owned by its **window
instance**, refreshed by a paint and retired only when its window leaves the layout.

Closing it made the node graph reachable for the first time, and reaching it immediately exposed two
further defects on paths that had been dead — both fixed here, both law-first:

1. **A plain click on the graph aborted the pool worker.** `FlowHost::commit_gesture_history` dropped
   its undo baseline — a `FlowFixture`, which owns the fail-closed `OrderedMap<WidgetLayout>` root —
   whenever the gesture changed nothing. Measured on 6118: `ordered-map root must be explicitly
   retired before drop`, on press one, and the worker never logged again.
2. **A hover across a port faulted the whole dispatch.** The screen-path discriminator was asked only
   on a press, so a MOVE over a port/handle/port-insert/minimap fell through to the bounded
   `plan_pointer`, which answers `Unsupported` there. Measured on 6118 as
   `wgpu-shell graph move fault surface=procedural-main fault=Structure`, after which the session
   published nothing further.

Runtime, on `http://127.0.0.1:6118/?plugin=generation3d&example=hexagonal-mushroom-column`
(run `🗑️generated/wgpu-node-graph/run-8`): the retained graph is `procedural-main@487x814+491,54`,
**every one of six presses reaches it** (`graph button surface=procedural-main … outcome=Ok(true)`),
hovers resolve real ports and node bodies, and the **`Fit graph` chrome control works end to end** —
`hit=Some((NavbarItem, "shell.nodeGraph.fit::procedural-main"))` → `shell node-graph fit` →
`nodeGraphViewport{x:94.756, y:-97.508, zoom:1.784}`. No panic, no bounded fault.

**Not claimed:** a wire drawn on 6118, and a node selected on 6118 — §7 says exactly what each is
blocked behind and why neither is this layer.

---

## 2. Root cause, with file:line

### 2.1 The eviction

`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` (before this lane, `sync_engine_surface_states`):

```rust
let painted = |kind, id| registrations.iter().any(|entry| &entry.surface_id == id && kind(&entry.detail));
let stale_graphs: Vec<String> = self.node_graph_states.keys().filter(|id| !painted(…, id)).cloned().collect();
for id in stale_graphs { self.node_graph_states.remove(&id); }
```

and `⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs`'s own doc stated the intent:

> Ephemeral: rebuilt by the paint pass every frame and drained by the shell, so a window that stops
> painting its surface stops being pointer-dispatchable the same frame.

That is the whole defect, stated as a feature. `sync_engine_surface_states` runs once per COMPLETED
chrome walk (`🧊️renderer/🦀️.rs:13336`, `FrameBuildPhase::Chrome`), and a retained document that did
not change does not repaint — so on a loaded serve the drain carries the graph in some walks and not
in others, and in every walk it does not, the surface was removed.

While it is removed, **nothing can address the graph at all**, because every path starts by
hit-testing that map: `node_graph_pointer_{down,move,up}_into` (`🧊️renderer/🦀️.rs:13716/13742/13811`),
`node_graph_wheel_into` (`:11760`), the context menu (`🐚️Shell/…:7789`), the catalogue/tree drop
(`:7617`), `keyboard_fit_surface_id` (`:6267`) and the `Fit graph` chrome control itself
(`surface_overlay_controls`, `:12486`).

`world3d_states` is never touched by that function — it is written by the World3d paint step
(`🎞️Scenes/…:1337-1344`) and never removed — which is precisely why World3d orbit/pick kept working
while the graph did not. One owner per window instance is therefore not a new idea in this codebase;
it is the rule World3d already followed and the one React holds structurally
(`🕸️NodeGraph/🟦️.tsx:2464-2491` — the host effect closes on `[surfaceId]` and its cleanup is
documented as *"A retained surface host unmounts exactly once, when its window closes"*).

### 2.2 The no-op gesture that aborted the worker

`🌊️flow/🖥️host/🦀️.rs:2182` (before):

```rust
let baseline = self.pending_history_baseline.take().unwrap_or_else(|| self.fixture.clone());
if Self::content_changed(&baseline, &self.fixture) { … }   // ← else: baseline falls out of scope
```

`begin_gesture` (`:2176`) clones the whole fixture on every `pointer_down_screen`, and
`OrderedMap::drop` asserts (`🌱️value/🗂️ordered/🦀️.rs:81`). The same class of abort is already
documented two functions above (`history_store_from_baseline`, `:2126-2133`) — this branch was the
one case it did not cover, and it is the most common gesture there is: a click that moves nothing.

### 2.3 The hover that faulted

`⚙️EngineCanvas/…:2795, 2812` (before) passed `None` as the point for MOVE and UP:

```rust
if node_graph_gesture_is_screen_path(surface_id, None) { … }
```

so only a PRESS was tested against `DagHost::screen_pointer_gesture_begins_at`
(`🕸️dag/🦀️.rs:4677`) — the predicate that names exactly the four hits `bounded_node_hit_index`
(`:3183-3191`) answers `DagInteractionPlanFault::Unsupported` for. A hover over a port therefore
reached `plan_pointer`, faulted, and `record_action_fault` ended the session.

---

## 3. The fix, per file

### 3.1 A registration names its owner — `⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs`

`EngineSurfaceRegistration` gains `window_id: String`; `register_engine_surface`,
`sync_node_graph_scene`, `sync_tiled_map_scene`, `sync_board2d_scene` and `sync_engine_scene` all take
it. The struct's doc now says what the record IS — a refresh, not a lease.

### 3.2 The walk carries the window — `🎞️Scenes/…`, `🗣️Interpreter/…`

`SceneEngineHosts` gains `window_id: &'a str`, set from `render_ui_document_step`'s own `window_id`
(the same string `Ui::frame_into_step` paints under) and threaded through `FrameworkSceneHost`.
`NodeGraphSurface`/`TiledMapSurface`/`Board2dSurface` each gain `window_id: String`.

### 3.3 The retention rule — `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`

- `live_window_ids()` — the dock plan's bodies plus the open panels' tabs. Recomputed by
  `plan_dock_windows` on every chrome walk, independent of which bodies repainted.
- `mirror_engine_surface_states(...)` — `Self`-less (a `ShellState` fixture is impractical: 90+
  fields, a constraint the crate's own `🔬️wgpu-shell-input` tests already record), so its law drives
  the real production body. It retires the surfaces whose `window_id` has left `live_window_ids`,
  refreshes what the drain mentions, and answers which hosts were constructed on this drain — the
  list the app-static catalogue is installed on.
- `sync_engine_surface_states` is now the thin wrapper that computes the live set and installs the
  catalogue.

### 3.4 A gesture that changed nothing retires its baseline — `🌊️flow/🖥️host/🦀️.rs:2182`

```rust
if !Self::content_changed(&baseline, &self.fixture) {
    baseline.retire_cold();
    return;
}
```

`FlowFixture::retire_cold` is the codebase's own disposal for this owner
(`🌊️flow/🗿️artifacts/🌊️flow/🧵️retained/🦀️.rs:524`), the same one `undo` (`:2209`) uses.

### 3.5 Every phase over an unsupported hit — `⚙️EngineCanvas/…:2778, 2795, 2812`

All three phases now ask `node_graph_gesture_is_screen_path(surface_id, Some((sx, sy)))`.
`DagHost::screen_pointer_gesture_begins_at`'s doc is widened to say it answers for a POINT, not for a
press. Nothing else moved: node selection, node dragging and marquee keep the bounded,
atomically-admitted plan/commit path, because none of those points is in the predicate's set
(`widget_hit_at` is inline widgets — sliders and selects — not node bodies).

### 3.6 One fault line kept — `🧊️renderer/🦀️.rs`

`[DEBUG] wgpu-shell graph move fault surface=… fault=…` and `[DEBUG] wgpu-shell graph button
surface=… down=… outcome=…`. A bounded fault on this path is not recoverable in practice — the
session stops publishing — and without a line it reads as "the pointer stopped working". Both fire
only on a real press or a real fault, never per frame.

---

## 4. Laws

### 4.1 Engine-surface retention (new)

- Oracle: `🧑‍🎨engine/🧫️fixtures/🧲️engine-surface-retention/🔣️.json` — 6 declared rules, **6 cases**:
  a painted surface becomes dispatchable; **a frame without a repaint keeps it**; a repaint refreshes
  bounds and controller; a closed window drops it; all three composited kinds alike; creation is
  announced once.
- Rust law: `🐚️Shell/🧪️tests/🧲️engine-surface-retention/🦀️.rs`, driving the real
  `ShellState::mirror_engine_surface_states` over real `AdmittedSurfaceMap`s and resolving each probe
  the way the OS event loop does (`bounds.contains`).
  `cargo test -p semio-framework-os-renderer-wgpu --lib -- engine_surface_retention` →
  **2 passed; 0 failed** (545 filtered).
- **It is a law, not a description.** Re-instated the drain-eviction rule inside the same function and
  re-ran: `a-frame-without-a-repaint-keeps-the-surface: a pointer at (400, 300) resolved to no engine
  surface at all` — the exact defect, in the exact words.
- TypeScript twin: `🧑‍🎨engine/🧪️tests/🧲️engine-surface-retention/🟦️.ts` — re-derives all six cases a
  second time from the fixture's declared rules through an independent reference mirror, and pins the
  rule to the renderer that already held it (React's `[surfaceId]`-keyed host effect) and to the one
  that now does (`live_window_ids.contains(&surface.window_id…)`, `pub window_id: String`).
  **9 pass; 0 fail.**

### 4.2 The no-op gesture retires its baseline (new)

`🌊️flow/🖥️host/🧹️retirement/🧪️tests/🧹️retirement/🦀️.rs` —
`a_gesture_that_changed_nothing_retires_its_history_baseline` drives three real
`pointer_down_screen`/`pointer_up_screen` pairs over an unchanged fixture and then retires the host.
A bare drop ABORTS rather than unwinds, so the law cannot be written with `catch_unwind`: it is the
run that proves it. `cargo test -p semio-framework-os-flow --lib -- retirement` →
**8 passed; 0 failed**. With the guard removed: `FAILED … ordered-map root must be explicitly retired
before drop`.

### 4.3 Every phase over an unsupported hit (extended, not forked)

`🕸️dag/🧫️fixtures/🔗️wire-edit/🔣️.json` gains the rule `everyPhaseOverAnUnsupportedHit` and two cases
(`a-hover-over-a-port-belongs-to-the-screen-path`, `a-hover-over-empty-canvas-stays-on-the-bounded-path`),
and `🕸️dag/🧪️tests/🔗️wire-edit/🦀️.rs` a `probePhases` arm that asserts the discriminator and the
bounded fault set are the SAME set for `Down`, `Move` and `Up`.
`cargo test -p semio-framework-os-infinite --lib -- wire_edit` → **4 passed; 0 failed** (346
filtered). The half no Rust law can reach — that the RENDERER asks on all three phases — is pinned in
the existing twin `🧪️tests/🔗️node-graph-wire-edit/🟦️.ts` by reading the three call sites out of
`⚙️EngineCanvas/…/🦀️.rs`.

### 4.4 TypeScript twins, together

`bun test` over `🧲️engine-surface-retention`, `🔗️node-graph-wire-edit`, `🎛️retained-control-commit`,
`🎯️retained-hit-targets` → **45 pass; 0 fail; 215 expect() calls**.

---

## 5. Build and test evidence

| command | result |
|---|---|
| `cargo check -p semio-framework-os-renderer-wgpu` | `Finished` |
| `cargo test -p semio-framework-os-renderer-wgpu --lib -- engine_surface_retention` | **2 passed; 0 failed** |
| `cargo test -p semio-framework-os-infinite --lib -- wire_edit` | **4 passed; 0 failed** |
| `cargo test -p semio-framework-os-flow --lib -- retirement` | **8 passed; 0 failed** |
| `bun test` (4 twins) | **45 pass; 0 fail** |
| `nx run @semio-tech/framework-renderer-wgpu:wasm` | `Successfully ran target`, dist 2026-09-13 20:03, this lane's strings verified present and the removed one absent |

---

## 6. 6118 — what the runtime actually does now

Probes (both new, both in the ticket root): `🐍️wgpu-node-graph-retention-probe.mjs`,
`🐍️wgpu-press-admission-probe.mjs`. Raw console, verdicts and screenshots under
`🗑️generated/wgpu-node-graph/{run-1…run-9, press-edit, press-edit-2, control-1, control-2}`.

### 6.1 The retained surface

`[DEBUG] wgpu-shell engine surfaces syncs=61 drains-with-graph=61 drain=2
live=["procedural-main@487x814+491,54"] old-rule-would-evict=[]
windows=["procedural-main", "procedural-preview", "framework.panel.catalogue", …]`

The graph is the RIGHT HALF of the `procedural-main` body (`975x814+3,54`) — a fact no probe could
guess and the reason earlier sweeps over the window rect found so little. Note what the map holds
across syncs: one surface, stable, with its owning window named.

### 6.2 Per-press, run `run-8` (segmented on the host's own `handle_event PointerDown`)

| press | at | what the runtime logged |
|---|---|---|
| 1–2 select + node drag | `873,552` (hover: node `column-preview`) | `graph button surface=procedural-main down=true … outcome=Ok(true)` ×4; `interactionSelect` ×11, `nodeGraphViewport` ×11 |
| 3 wire `extrude@vector` → `extrusion-axis@y` | `719,370` | `outcome=Ok(true)`; `interactionSelect` ×8 |
| 4 wire back | `843,552` | `outcome=Ok(true)`; `interactionSelect` ×8 |
| 5 minimap | `923,823` | `outcome=Ok(true)` |
| **6 `Fit graph`** | `525,76` | `pointer button … hit=Some((NavbarItem, Some("shell.nodeGraph.fit::procedural-main")))` → `[DEBUG] shell node-graph fit {"surface":"procedural-main","x":94.7558…,"y":-97.5083…,"zoom":1.7844…}` → `action=nodeGraphViewport` carrying exactly that camera |

`panicked` 0, `BoundedActionFault` 0, `graph move fault` 0 across the whole run.

### 6.3 The two defects this lane found by reaching the surface, before and after

| | before | after |
|---|---|---|
| press admission (`🐍️wgpu-press-admission-probe.mjs`, 5 fixed clicks) | 1 `PointerDown`, then `wgpu-worker panicked: ordered-map root must be explicitly retired before drop`, and every later press lost (`press-edit`) | **5/5 `PointerDown` + `PointerUp`, `graph button surface` ×6, `panicked` 0** (`press-edit-2`) |
| hover across a port | `wgpu-shell graph move fault surface=procedural-main fault=Structure`, console dead from that line on (`run-4`) | hover resolves real ports — `interactionHover targets=[{"granularity":"handle","id":"profile@wire"}]`, `extrude@wire`, `extrude@solid`, `extrude@vector` — with zero faults (`run-5`, `run-8`) |

### 6.4 What is NOT claimed on 6118, and why

- **A wire drawn.** `node-graph screen gesture` 0, `action=nodeGraphEdit` 0, `dag edge connected` 0.
  The press-drag-release reaches the host correctly (down at the source port, five intermediate moves
  all dispatched, up at the target — `run-5` lines 6820-6880), but every port pair the hover sweep
  could find in this example was output→output or input→input, which is a legitimate no-op. The
  mechanism itself is law-proven against the real `DagHost` (§4.3 and the previous lane's
  `a_port_to_port_drag_creates_a_wire_and_journals_it_for_the_guest`). Finding an
  output→input pair needs a denser sweep than the serve currently survives — see below.
- **A node selected.** Every press dispatches `interactionSelect` with `targets:"[]"`. The only node
  body the sweep landed on was `column-preview`; `fixture_draggable_node_hit` (`🕸️dag/🦀️.rs:4816`)
  only returns nodes the engine marks `draggable`, so an empty selection there may well be correct.
  Selecting a draggable operator was not reached, so nothing is claimed either way.
- **Both are gated by an admission ceiling that is not this layer's.** Every pointer move over the
  graph reserves three bounded action credits, and a sweep faster than the frame loop drains them
  answers `BoundedActionFault::ItemCredits` — after which the session publishes nothing at all
  (`run-6`: faulted at sweep point ~200, console dead; `run-9`, on a serve loaded by peer lanes: 43
  faults, one press admitted). A discrete press can also be deferred indefinitely behind
  `apply_pending_step blocked: head needs the interaction state, checked out at Some("frame-deferred")`
  (`run-7`: 6 `PointerDown` drained, 0 dispatched). Owning layer: input admission / frame scheduling.

---

## 7. The exact next step, for whoever picks this up

1. **The wire.** Re-run `🐍️wgpu-node-graph-retention-probe.mjs` on an idle serve. It already sweeps
   for ports, records each one's `"{nodeId}@{portId}"` id, and drags every ordered pair; the hop that
   wires will log `[DEBUG] wgpu node-graph screen gesture surface=… edits=[Connect { … }]`
   (`⚙️EngineCanvas/…:3030`, the previous lane's marker, deliberately still in place) followed by
   `action=nodeGraphEdit`. Pick a pair whose ids name DIFFERENT nodes and whose channels differ, e.g.
   `profile@wire` → `extrude@wire`.
2. **The sweep ceiling.** `node_graph_pointer_move_into` reserving three action credits per MOVE is
   the thing that makes a hover sweep unsurvivable. A hover that changes nothing need not reserve at
   all. That is the input/admission lane's call, and it is what stands between this probe and a
   dense enough sweep to find every port.
3. **Node selection.** Instrument `bounded_node_hit_index`/`fixture_draggable_node_hit` against the
   engine's own hover pick: the hover says `column-preview`, the bounded plan selects nothing, and
   whether that is `draggable: false` or a genuine disagreement is one log line away.

---

## 8. The previous lane's three `[DEBUG]` logs

The brief was to remove them "once their blockers are closed". Blocker **(b)** — this lane's — is
closed. Blocker **(a)** (`addGeneration` reaching the guest but creating no generation) is not, and
§6.4 shows the wire itself is still unproven at runtime. So, per log:

- `⚙️EngineCanvas/…:3030` — `wgpu node-graph screen gesture surface=… edits=…`. **Kept.** It fires
  only when a gesture actually edits a wire, and it is the single witness whoever closes §7.1 needs.
- `🐚️Shell/…:6474` — `wgpu-shell pointer button … targets=… hit=…`. **Kept**: blocker (a) is
  diagnosed through it, and it is what identified the `Fit graph` control in §6.2.
- `🐚️Shell/…:6894` — `wgpu-shell retained press …`. **Kept**, same reason.

This lane's own temporaries: the per-move success line (`graph move into`, ~250 lines/run) was added,
used for §6.1's rect discovery, and **removed**. What remains are three lines that fire once per
completed chrome walk or once per real press — the retention census, the press outcome and the fault
— each documented as a diagnostic rather than as a temporary.

---

## 9. Peer boundaries observed, and reds that are not this lane's

- **wgpu-world3d-interaction** — untouched. Its wasm build and mine collided twice; both waited.
- **`semio-framework-os-infinite` did not compile** for ~10 minutes around 18:30 (`world_merge_wire_label`,
  then `MergeMode::Range` non-exhaustive) — a peer's live `MergeMode` refactor in
  `♾️infinite/🌍️world/🦀️.rs`. Waited; it resolved itself. **Not** fixed forward.
- **`engine_canvas_slot_tables_are_heap_first_and_fit_a_bounded_thread_stack` is RED and is not this
  lane's.** `EngineSurfaceSlot` measures 71 848 bytes against a committed budget of 67 528 — a
  +4 320-byte growth of `EngineSurface` via `NodeGraphEngine` → `FlowHost` → `DagHost`. This lane
  added no field to any of the three measured structs; the budget fixture
  (`⏳️async/🧫️fixtures/🧱️boxed-fixed-slots/🔣️.json`) was last committed in `🚩️617` (11:27) and
  `🕸️dag/🦀️.rs` in `🚩️618` (14:41). Re-committing someone else's budget would hide their growth, so
  it is reported instead.
- Three other renderer tests fail in isolation and are untouched by this lane's files:
  `shell_absolute_refusal_returns_the_exact_max_plus_one_owner_before_mutation` (document arena
  `ArenaFull`), `runtime_single_enqueue_reader_cannot_observe_completion_without_its_scene_invalidation`,
  and `renderer_asset_probe_keeps_pages_owned_across_chunk_boundaries_and_rejects_malformed_length`
  (aborts the test process with SIGABRT).
- `cargo test -p semio-framework-os-flow --lib` (whole crate) was blocked at 20:10 by a peer's
  `RequestOutcome: Eq` break in `semio-framework`; the targeted `-- retirement` filter had passed
  minutes earlier.

---

## 10. Files

**Changed**

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs` — `window_id` on `EngineSurfaceRegistration` and every attach entry; `mark_engine_scene_repaint`; the three-phase path discriminator
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs` — `SceneEngineHosts.window_id`; `window_id` on the three surface records
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs` — `FrameworkSceneHost.window_id`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` — `live_window_ids`, `mirror_engine_surface_states`, the retention census, the law's module
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs` — press outcome + move fault lines
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs` — `commit_gesture_history` retires a no-op baseline
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs` — `screen_pointer_gesture_begins_at` answers for a point, every phase
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧫️fixtures/🔗️wire-edit/🔣️.json` — `everyPhaseOverAnUnsupportedHit` + 2 cases
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔗️wire-edit/🦀️.rs` — `probePhases`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🧹️retirement/🧪️tests/🧹️retirement/🦀️.rs` — the gesture-history retirement law
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔗️node-graph-wire-edit/🟦️.ts` — the three-phase renderer pin
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧪️tests/🕸️wgpu-node-graph/🦀️.rs`, `…/🧪️tests/🧩️wgpu-engine-surfaces/🦀️.rs` — call sites

**Created**

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧫️fixtures/🧲️engine-surface-retention/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🧲️engine-surface-retention/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧲️engine-surface-retention/🟦️.ts`
- `<ticket>/🐍️wgpu-node-graph-retention-probe.mjs`
- `<ticket>/🐍️wgpu-press-admission-probe.mjs`
- this report
