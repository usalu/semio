# Wgpu node-graph attach — the `SurfaceKind::NodeGraph` window on the wgpu shell (2026-09-10)

Closes the "New finding blocking the last mile" of `📓️wgpu-renderer-2026-09-10.md` §2: on wgpu the
node-graph engine host was never constructed in production, so generation3d's Flow window
(`procedural-main`, `SurfaceKind::NodeGraph`, body `procedural.play.main`) could not render.

Renderer-engineer lane. No `git` mutating command run, no browser tool used, no dev server started,
the vite serve on 6018 untouched. All cargo work in the private `CARGO_TARGET_DIR`
(`…/scratchpad/target-wgpu`) with `RUSTC_WRAPPER=""` and `--keep-going`.

(sections appended below as the work lands)

---

## 1. What was actually missing — bigger than the reported warning

`📓️wgpu-renderer-2026-09-10.md` §2 reported the symptom (`NodeGraphEngine`'s `Dag`/`Flow` variants
never constructed). Reading the registry that owns them turned up the cause, and it is one level
lower:

**No engine surface was ever created in production at all.** In
`🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs`, `EngineSurfaceRegistry::reserve`,
`::publish_reserved`, `::contains_key`, `::identity`, `EngineSurfaceId::try_from_str` and
`EngineSurfaceSnapshot` were **all `#[cfg(test)]`**, and the only `ensure_surface` in the tree was a
`#[cfg(test)] fn` inside `🧪️tests/🧊️wgpu-standalone/🦀️.rs`. So `ENGINE_SURFACES` was empty in every
production frame — which makes not just the node graph but the tiled map, the puzzle board and the
text-editor host dead on wgpu too, since every one of their entry points starts with
`map.get(surface_id)` / `map.get_mut(surface_id)`.

Three further holes on the same path, all confirmed by grep across the whole crate:

| hole | evidence |
|---|---|
| `ShellState.node_graph_states` (and `world3d_states`/`tiled_map_states`/`board2d_states`) never inserted | the only `try_insert`/`get_or_insert_with` call sites on `AdmittedSurfaceMap` are `SCENE_STATE`, `PENDING_RASTER_STATE` and the map's own tests. The OS event loop's node-graph pointer/wheel dispatch (`🧊️renderer/🦀️.rs:13504,13530,13535,13598,11578`) iterates that map, so it dispatched into an empty set. |
| `render_component_scene_step` painted **every** surface kind as a bare rounded panel | `🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:1208-1253` — four phases: surface id bytes, controller id bytes, screen height, one `push_rounded`. No per-kind branch existed. |
| the engine-packet **producer** was `#[cfg(test)]` | `EngineCanvasBuildContext::try_reserve_fresh_packet`/`publish_reserved`. The whole consumer half is live (`EngineGpuCandidate`, `reserve_engine_texture`, the frame's `EngineTransfer` phase at `🧊️renderer/🦀️.rs:13184`) and simply never received a packet. |

`paint_node_graph_labels`, `paint_node_graph_overlays` and `render_world_3d` likewise have **zero**
production call sites. An old comment in `🗣️Interpreter/🧪️tests/🔬️wgpu-ui-command-wiring/🦀️.rs:382`
still refers to "`engine_canvas::paint_text_editor` having lazily created one in `ENGINE_SURFACES`",
so this production seam existed once and was lost — the shape below restores it for the node graph.

---

## 2. Design — one production seam, four steps, all inside the existing frame machinery

No new channel, no second GPU path. The node graph reaches the screen through exactly the pipeline
that was already built for it and never fed:

```
render_ui_document_step  (Chrome phase, per frame, per window body)
  └─ SceneHost::paint_slot_step → scenes::render_component_scene_step(scene, bounds, ctx, cursor)
       phase 4  engine_canvas::sync_node_graph_scene   → ensure surface, construct engine, feed scene
       phase 5  engine_canvas::stage_node_graph_paint  → FlowHost/GraphHost::paint_scene → canvas::Scene
       phase 6  ctx.draw.push_raster_quad("engine:<surface>", bounds, …)
       phase 7  engine_canvas::paint_node_graph_labels    (text overlay, React's DOM label layer)
       phase 8  engine_canvas::paint_node_graph_overlays  (marquee + selection bounds)
  … Chrome completes →
ShellState::sync_node_graph_surface_states  → node_graph_states = this frame's attached surfaces
  … FrameBuildPhase::EngineTransfer →
engine_canvas::stage_engine_packet_step(resources) → EngineCanvasBuildContext → EngineCanvasPacket
  … EngineGpuCandidate (unchanged) → GpuContext::reserve_engine_texture("engine:<surface>") → vello
```

### 2.1 Attach and feed

| piece | file:line | role |
|---|---|---|
| `ensure_engine_surface` | `⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:1655` | the single production entry into `ENGINE_SURFACES`; promoted verbatim out of the test file, which now calls it instead of owning a copy |
| `NodeGraphSurfaceRegistration` / `NodeGraphSurfaceRegistry` | `🦀️.rs:1752` | the per-frame projection the shell mirrors — surface id, screen bounds, controller id, and whether THIS attach constructed the host |
| `take_node_graph_surface_registrations` | `🦀️.rs:1801` | drain-not-read, so a window that stops painting stops being pointer-dispatchable the same frame |
| `node_graph_scene_uses_flow_engine` | `🦀️.rs:1810` | the wgpu twin of React's `isFlowGraphScene(capabilitiesJson) \|\| Boolean(fixtureJson)`; generation3d carries `fixtureJson`, so it selects `Flow` |
| `node_graph_engine_from_scene` | `🦀️.rs:1823` | builds `FlowHost::from_fixture(parse(fixture_json))` or `GraphHost::default()`, and sets the canvas theme from the panel colour's luminance |
| `sync_node_graph_engine` | `🦀️.rs:1877` | operators → fixture → selection → hover → preview-off → lod → camera, each gated on a `NodeGraphSyncCache` field so an unchanged frame costs string compares only |
| `sync_node_graph_evaluation` | `🦀️.rs:1957` | eval BEFORE status/computing — React documents that ordering explicitly (`applyEvalOutputsJson` clears computing chrome) |
| `sync_node_graph_scene` | `🦀️.rs:1996` | the public attach entry; bumps `EngineSurface.scene_revision` only when something was applied, which is what lets the GPU freshness gate reuse an unchanged texture |

`NodeGraphSyncCache` gained three fields (`viewport_pixels`, `operator_ids`, `hover`) and both
`close_node_graph_sync` (`🦀️.rs:426`) and `node_graph_sync_terminal` (`🦀️.rs:1498`) were extended in
lockstep, so the retirement ladder still reaches an exact terminal witness.

### 2.2 Paint and packet

`stage_node_graph_paint` (`🦀️.rs:2043`) paints through the host's **own** vector renderer —
`FlowHost::paint_scene` / `GraphHost::paint_scene`, both delegating to `DagHost::paint_scene` in
`♾️infinite` — into a `canvas::Scene`, and stages it. Staging exists because paint happens deep inside
the retained chrome walk, which owns no `EngineCanvasBuildContext`; `STAGED_ENGINE_SCENES`
(`🦀️.rs:729-790`) is the one seam, keyed per surface so the newest paint replaces the older one and a
frame that never drained cannot accumulate scenes. `EngineCanvasBuildContext::publish_staged`
(`🦀️.rs:862`) admits it carrying the **surface's** own document/scene/metrics generations rather than
the build context's, so `engine_gpu_freshness_matches` compares like with like.

`engine_raster_key` (`🦀️.rs:2075`) exposes the `engine:<surface>` key `EngineGpuCandidate` already
reserves, so the retained `push_raster_quad` composites the vello texture into the window's draw list.

### 2.3 Input

The pointer/wheel entry points (`node_graph_pointer_down_into` and siblings) and the action writer
were already correct and already byte-identical to React for the node case — they had no host to
dispatch into. Two changes:

- **`node_graph_states` is now populated.** `ShellState::sync_node_graph_surface_states`
  (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:3300`), called from the frame build the moment Chrome completes
  (`🧊️renderer/🦀️.rs:13128`), mirrors the drained registrations in and removes surfaces that stopped
  painting. It also installs the app-static catalogue on hosts constructed **this** frame — closing
  `📓️wgpu-renderer-2026-09-10.md` §2's loop, whose `publish_app_catalogue` sweep iterated an empty map.
- **Hover is channel-qualified.** `graph_hovered_handle` (`🦀️.rs:2459`) asks the live host's own
  `pick_targets_at_screen_json` for the `"{nodeId}@{portId}"` channel under the pointer — a pure query,
  which matters because the bounded-action protocol must know the payload before the plan is
  committed. `write_graph_interaction_actions` (`🦀️.rs:2537`) then emits
  `{granularity: "handle", id: "extrude@wire"}` instead of the node target, exactly what React's
  `nodeGraphHoverActionArgs(nodeId, portId)` dispatches. Selection stays node-granular: that is what
  the DAG's own pointer plan selects, and React's `SelectionGather` path agrees.

---

## 3. Tests

`🧱️elements/⚙️EngineCanvas/🧪️tests/🕸️wgpu-node-graph/🦀️.rs` (new), mounted from the wgpu EngineCanvas
target at `🦀️.rs:1595-1597`. The scene is generation3d's own Flow window payload for the bundled
`hexagonal-mushroom-column` example — 7 widgets, 6 synapses — committed as data in
`🧪️tests/🕸️wgpu-node-graph/🔣️.json` with a `provenance` block naming the DSL asset and the producer
(`Generation3dSnapshot::parse_dsl → snapshot.fixture → dsl::json::to_json_string`, the flow window's
own path). It is committed rather than imported because the framework renderer must not depend on a
plugin crate; every field of it is re-validated by `FlowHost::parse_fixture_json` inside the lanes —
a wrong key would drop widgets or synapses and the `(7, 6)` assertion would fail.

The oracle for the *payloads* is the other implementation of this same host: React's `NodeGraphHost`
(`🧱️elements/🕸️NodeGraph/🟦️.tsx`), whose `nodeGraphSelectionActionArgs` / `nodeGraphHoverActionArgs` /
`nodeGraphViewportActionArgs` the assertions reproduce field for field.

```
test engine_canvas::node_graph_attach_tests::node_graph_window_attaches_the_flow_engine_and_paints_a_non_empty_draw_list ... ok
test engine_canvas::node_graph_attach_tests::pointer_down_on_a_node_emits_the_graph_domain_selection_react_dispatches ... ok
test engine_canvas::node_graph_attach_tests::wheel_zoom_emits_the_node_graph_viewport_action_with_the_moved_camera ... ok
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 485 filtered out; finished in 0.04s
```
(`🗑️generated/wgpu2-attach-tests.txt`; needs `RUST_MIN_STACK=536870912` — the flow host's fixture
rebuild overflows the default 2 MB test-thread stack, the same reason the peer generation3d lanes set it.)

1. **`…attaches_the_flow_engine_and_paints_a_non_empty_draw_list`** drives the REAL production paint
   (`scenes::render_component_scene_step` through a real `FrameworkWidgetContext`, not the attach
   function directly) and asserts: the engine is `Flow` (a scene carrying `fixtureJson` selects it,
   as React does), the DAG holds exactly 7 nodes and 6 edges, the window's draw list carries a raster
   quad under `engine:node-graph-attach-draw`, the staged paint reaches the frame packet ledger, the
   packet is 966×836, and `!packet.scene.retirement_is_empty()` — i.e. `FlowHost::paint_scene` emitted
   real vector commands. The run's own `[DEBUG] dag draw lod=detail zoom=1.784` line (a pre-existing
   log in the dag painter, not this lane's) independently witnesses the paint at the fixture camera.
2. **`pointer_down_on_a_node_emits_the_graph_domain_selection_react_dispatches`** ray-picks `extrude`
   through the live host's own `entity_screen_json` and asserts the published `interactionSelect` is
   `{domainId: "graph", merge: "replace", method: "pick", targets: "[{\"granularity\":\"node\",\"id\":\"extrude\"}]"}`
   and the paired `interactionHover` carries the same node target — then moves onto that node's `wire`
   input handle and asserts the hover target becomes
   `[{"granularity":"handle","id":"extrude@wire"}]`, the channel-qualified form.
3. **`wheel_zoom_emits_the_node_graph_viewport_action_with_the_moved_camera`** asserts a wheel-up
   publishes `nodeGraphViewport` whose `viewportJson` is *exactly* the camera the host committed
   (`{"x":…,"y":…,"zoom":…}`), and that the zoom rose above the fixture camera's 1.7844.

Teardown goes through the registry's own retirement ladder (`begin_engine_surface_close_token` +
`close_engine_surface_step`): `FlowFixture`'s ordered maps refuse to drop unretired, so a plain
`remove` panics **inside the registry mutex** and poisons it for every later test in the binary. The
first cut of this file did exactly that and turned two unrelated lanes into `PoisonError`.

### 3.1 Neighbouring suites

`cargo test -p semio-framework-os-renderer-wgpu --lib engine_canvas` — **27 passed, 1 failed**
(`🗑️generated/wgpu2-engine-canvas-tests.txt`). The failure is
`saturated_graph_and_board_wheel_queues_preserve_cameras`, on a board-host assertion
(`defers_descriptor_sync_from_js`); it reproduces in isolation, `git diff HEAD` shows **no board line
anywhere in this lane's diff**, and the only working-tree change under the board is the previous wgpu
lane's `puzzle::editor::puzzle2d::engine::BoardHost` → `infinite_canvas::BoardHost` type swap
(`🧪️tests/🧊️wgpu-standalone/🦀️.rs:144,362`) — two different types, which is the likely cause.

`cargo test -p semio-framework-os-renderer-wgpu --lib scenes::` — **111 passed, 2 failed**
(`🗑️generated/wgpu2-scenes-tests.txt`), both pre-existing:
`production_action_ingress_has_no_legacy_queue_and_text_vec_helpers_are_test_only` asserts
`handle_scene_wheel`/`_pointer_move`/`_pointer_button` exist as `#[cfg(test)] pub fn` in
`🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs` — a peer deleted them outright, so the source guard now fails on a
function that no longer exists; and `text_editor_tests::run_menu_action_rename_…` is unrelated.

---

## 4. Checks

### 4.1 Native

`cargo check -p semio-framework-os-renderer-wgpu --keep-going` — **0 errors, 18 warnings**
(`🗑️generated/wgpu2-native-check.txt`); the baseline taken before any edit this session was **0 errors,
20 warnings** (`🗑️generated/wgpu2-baseline.txt`).

**The `variants Dag and Flow are never constructed` warning is gone** — that was the whole point; it
was `⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:33` in the baseline and appears nowhere in the new log. So is
`EngineSurface`'s `fields width and height are never read` (`:66`), for the same reason. The one
remaining `never constructed` in the crate is `🎞️Scenes/…:325 variant InkResize`, untouched and
pre-existing. The other 17 are the same pre-existing dead-code/unnecessary-qualification warnings the
previous lane listed.

### 4.2 Wasm

The wgpu shell is built by trunk for **`wasm32-unknown-unknown`** (`📦️packages/🦀️rust/Trunk.toml`).

`cargo check -p semio-framework-os-renderer-wgpu --target wasm32-unknown-unknown --keep-going` —
**0 errors, 14 warnings** (`🗑️generated/wgpu2-wasm-check.txt`).

This is a change of state worth flagging to the coordinator: `📓️wgpu-renderer-2026-09-10.md` §4.2
reported this target blocked by **two** unrelated peer crates (`semio-s-artifact-puzzle-3d`'s
clipboard refactor, 12 errors; `semio-framework-os-kernel`'s store, 6 errors). **Both have cleared.**
The wasm precondition its §5 put in front of a trunk boot is satisfied as of this run.

### 4.3 Not run

The `native` winit target (`bun nx run …:native -- generation3d`) is `"continuous": true` and opens a
blocking window — skipped, as in the previous lane. No dev server was started and no browser tool used.

---

## 5. The wgpu wasm boot recipe, for the coordinator's verification

```bash
cd /Users/ueli/Documents/semio
CARGO_TARGET_DIR=<private-scratchpad-target> \
RUSTC_WRAPPER="" \
SEMIO_RENDERER=wgpu \
S_OS_PORT=6118 \
bun ./📜️script.ts dev procedural 3d
```

(or `.vscode/launch.json`, row `🛠️dev🔧️procedural🏙️3d🧊️wgpu🌐️wasm`.) Leave `SEMIO_PLUGIN_ONLY` unset —
it drops 10 of the 11 crates the closure needs. Then open `http://127.0.0.1:6118/?plugin=generation3d`.

**Readiness markers** (`…🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:1464-1498`):
- `[dev] Port 6118 already serving wgpu trunk at <url>` — reused.
- `[dev] wgpu trunk serving at <url>` — fresh boot succeeded.

**Preconditions, in order:**

1. **The wasm target is green as of this run** (§4.2) — the two peer blockers the previous lane hit
   have cleared. Re-run `cargo check -p semio-framework-os-renderer-wgpu --target wasm32-unknown-unknown`
   immediately before starting trunk anyway: a trunk failure surfaces much later and much less legibly.
2. **Rebuild the plugin closure.** `buildPlugins("generation3d")` runs as step 2 of the wgpu dev path;
   confirm with `strings <staged .core.wasm> | grep <a string unique to today's source>` — port 6118
   listening is not proof of a rebuild.
3. **Then check, in this order:**
   - The Flow window (`procedural-main`) now paints the graph itself — 7 nodes, 6 wires, node labels
     and port labels. That is this packet live. An **empty** Flow canvas now means one of: the engine
     texture never staged (look for a missing `engine:procedural.play` raster key), or the frame
     never reached `EngineTransfer`.
   - Click a node: the plugin should receive `interactionSelect` with `domainId: "graph"` and a bare
     `{granularity:"node", id:"<widget>"}` target; hover a port and it should become
     `{granularity:"handle", id:"<widget>@<port>"}`.
   - Wheel over the canvas: `nodeGraphViewport` with `{surfaceId, viewportJson}`; the graph should
     zoom under the cursor.
   - The canvas spotlight/palette should now carry the app catalogue — `refresh_app_catalogue` fetched
     it already (previous lane §2), and `sync_node_graph_surface_states` installs it on the host the
     frame it is constructed.
   - `procedural-preview` (World3d) is a **separate** path and is NOT fixed by this packet: the
     previous lane landed the mesh-wire bridge, but `ShellState.world3d_states` is still never
     populated in production (§1) and `render_world_3d` still has zero call sites. The 3D preview will
     stay blank until a `World3d` attach seam is added the way §2 adds the node-graph one.

### 5.1 Known gap to watch first

The attach and the packet are driven from the **retained scene paint**. If `render_component_scene_step`
is not re-entered on a frame where only the graph's camera moved (retained-tree revision unchanged),
the staged packet will not refresh and a pan/zoom could look frozen while the actions still fire. The
unit lanes cover one paint pass exactly; only a live boot can settle the per-frame cadence. If the graph
paints once and then freezes under pan, that is this, not the sync.

---

## 6. Files changed

| file | change |
|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs` | production `ensure_engine_surface`; `EngineSurfaceRegistry::reserve`/`publish_reserved`/`contains_key`/`identity` and `EngineSurfaceId::try_from_str`/`EngineSurfaceSnapshot` promoted out of `#[cfg(test)]`; new `//#region 🕸️NodeGraphAttach` (registration registry, engine choice, scene sync, evaluation sync, attach entry, paint staging, raster key); `StagedEngineScene(s)` + `stage_engine_packet_step` + `EngineCanvasBuildContext::publish_staged`; 3 new `NodeGraphSyncCache` fields with matching close/terminal steps; channel-qualified hover (`graph_hovered_handle`, `GraphInteractionSnapshot.hovered_handle`, `write_graph_interaction_actions`); mount the new test module |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧪️tests/🕸️wgpu-node-graph/🦀️.rs` | **new** — 3 attach lanes |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧪️tests/🕸️wgpu-node-graph/🔣️.json` | **new** — the committed generation3d `hexagonal-mushroom-column` flow fixture + provenance |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧪️tests/🧊️wgpu-standalone/🦀️.rs` | the `#[cfg(test)] fn ensure_surface` copy removed (it is production now); 3 call sites renamed |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs` | `render_component_scene_step` phases 4-8: the `SurfaceKind::NodeGraph` attach/paint/composite/labels/overlays branch |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` | `ShellState::sync_node_graph_surface_states` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs` | call it when Chrome completes; drain staged engine paints in the frame's `EngineTransfer` phase |

Raw logs: `🗑️generated/wgpu2-baseline.txt`, `wgpu2-native-check.txt`, `wgpu2-wasm-check.txt`,
`wgpu2-attach-tests.txt`, `wgpu2-engine-canvas-tests.txt`, `wgpu2-scenes-tests.txt`.

## 7. Housekeeping note for peers

The session scratchpad filled the root volume twice during this lane (138 MiB free at the worst point,
which is what an `ENOSPC` in a `.fingerprint` file or a `full.rmeta` write actually means). Reclaimed,
in order: this lane's own `target-wgpu/debug/incremental` (23 GB — `CARGO_INCREMENTAL=0` is set on every
cargo call here since), then the idle `target-cat`/`target-cat-wasm` dirs (32 GB), whose newest artifact
was 3h38m old, which had no live cargo/rustc process and no open file handle, and whose lane's report
(`📓️catalogue-surface-2026-09-09.md`) landed the day before. No other lane's target dir was touched.
