# Wgpu engine surfaces — every `SurfaceKind` the shell hosts, created in production (2026-09-10)

Generalises `📓️wgpu-node-graph-2026-09-10.md` §2, which restored the production engine-surface seam
for `SurfaceKind::NodeGraph` only. `World3d` (generation3d's edit/generate preview windows and the
viewer preview), `TiledMap` and `Board2d` were still dark on wgpu: their hosts were never constructed
in a production frame, so `render_component_scene_step` painted them as a bare rounded panel.

Renderer-engineer lane. No `git` mutating command run, no browser tool used, no dev server started,
the vite serve on 6018 untouched. All cargo work in the private `CARGO_TARGET_DIR`
(`…/scratchpad/target-wgpu`) with `RUSTC_WRAPPER=""`, `CARGO_INCREMENTAL=0` and `--keep-going`.

(sections appended below as the work lands)

---

## 1. What was missing — three different holes, one missing mechanism

`📓️wgpu-node-graph-2026-09-10.md` §1 established that **no engine surface was ever created in
production on wgpu**, and closed it for `SurfaceKind::NodeGraph` only: `render_component_scene_step`'s
new phases 4-8 attached, painted and composited a node graph, and `ShellState::sync_node_graph_surface_states`
mirrored it into `node_graph_states`. Everything else still fell through phase 4's
`if scene.component_kind != SurfaceKind::NodeGraph { return cursor.finish(); }` and stayed a bare
rounded panel.

Re-reading the three remaining kinds turned up that they are NOT one problem but three, and only the
*creation* half is shared:

| kind | host | where the host has to live | how it reaches the screen |
|---|---|---|---|
| `NodeGraph` | `FlowHost`/`GraphHost` | `ENGINE_SURFACES` (thread-local registry) | vello `canvas::Scene` → engine packet → `engine:<surface>` raster quad |
| `TiledMap` | `MapHost` | `ENGINE_SURFACES` — the `map_host` field was already there, never assigned | same |
| `Board2d` | `infinite_canvas::BoardHost` | `ENGINE_SURFACES` — same, `board_host` never assigned | same |
| `World3d` | `World3dState` | the **shell's** `world3d_states`, because every downstream ladder addresses it there (asset fetch `🧊️renderer/🦀️.rs:9993-10270`, snapshot apply `:11395`, bounded pick/hover authority `:11452`, the OS event loop's orbit/wheel dispatch `:13491,13516,13591`) | `render_world_3d` → `DrawList::push_scene_pass` **straight into the window's own retained draw list** — no texture, no raster key |

Two further facts, both confirmed by grep across the crate before any edit:

- `MapSyncCache`/`BoardSyncCache` (`⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:1466,1481`) already carried a
  field per scene field, and their `close_map_sync`/`close_board_sync` retirement rungs already existed
  — but **no production code ever wrote a single one of those fields**. The only writer in the whole
  tree was `🧪️tests/🧊️wgpu-standalone/🦀️.rs:179`. The caches were built for a sync that was never written.
- `EngineSurface.map_host`/`.board_host` had readers everywhere (`with_map_host`, `with_board_host`,
  `board_pick_best_target_id`, the whole pointer/wheel surface of both kinds) and **zero writers**.

So the shape below adds exactly one thing that did not exist — the per-kind *feed* — and routes all
four kinds through one attach entry, one registration list and one shell mirror.

---

## 2. Design

### 2.1 One attach entry, one registration list, one shell mirror

Region `//#region 🕸️NodeGraphAttach` is now `//#region 🧩️EngineSurfaceAttach`
(`⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:1755-2404`).

| piece | file:line | role |
|---|---|---|
| `EngineSurfaceKindDetail` | `🦀️.rs:1756` | the per-kind tail the shell's bespoke pointer maps need beyond bounds+controller: `TiledMap { selection_method }` (what `tiled_map_pointer_down_into` takes), `Board2d { fixture_json }`, `NodeGraph`, `World3d` (none — its host keeps its own bounds) |
| `EngineSurfaceRegistration` | `🦀️.rs:1769` | replaces `NodeGraphSurfaceRegistration`; **every** kind is recorded here, so there is one drained witness of what the chrome walk painted |
| `AttachedSurfaceRegistry` / `take_engine_surface_registrations` | `🦀️.rs:1779,1820` | unchanged drain-not-read semantics: a window that stops painting a surface stops being pointer-dispatchable the same frame |
| `register_engine_surface` | `🦀️.rs:2062` | the single writer, called by all four attach paths |
| `sync_engine_scene` | `🦀️.rs:2343` | **THE** production attach entry for the vello-composited kinds — `match scene.component_kind` → node graph / map / board |
| `stage_engine_scene_paint` | `🦀️.rs:2354` | **THE** paint entry, replacing `stage_node_graph_paint`: `FlowHost/GraphHost::paint_scene`, `MapHost::prepare_visible_tiles` + `build_vector_scene`, `BoardHost::build_vector_scene`, each into the same `StagedEngineScene` |
| `ShellState::sync_engine_surface_states` | `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:3300` | replaces `sync_node_graph_surface_states`: prunes each of the three projection maps against **its own kind** in the drained list, then upserts bounds/controller/tail and installs the app catalogue on hosts constructed this frame |

`scenes::render_component_scene_step` (`🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:1218`) phases 4-8 are now
kind-general: phase 4 routes `World3d` out to its own step and otherwise calls `sync_engine_scene`;
phase 5 stages the paint with a per-kind clear colour (`engine_surface_clear`, `🦀️.rs:1323` — panel
behind a graph, `theme.canvas_clear` behind a map or a board, matching each React host's own
`clear_color`); phase 6 composites the raster quad; phases 7-8 (labels, marquee/selection overlays)
stay node-graph-only and now say so instead of relying on being unreachable.

### 2.2 `World3d` — the one kind whose host is not a texture

`World3d` cannot attach in `ENGINE_SURFACES`: `render_world_3d` needs `&mut World3dState` (the shell's,
because that is the object every other ladder mutates) and `&mut World3dBuildContext` (this frame's,
built in `FrameBuildPhase::WorldResources` and drained again in `FrameBuildPhase::WorldTransfer`), and
it paints into the window's retained `DrawList` rather than an offscreen vello target.

Rather than hide that behind a second thread-local, both are **threaded down the chrome walk** as one
borrowed struct — no globals, no `mem::take`, the same shape `Ui::frame` already uses for its
`SceneHost`:

```
AppRuntime frame, FrameBuildPhase::Chrome        🧊️renderer/🦀️.rs:13126
  └─ ShellState::render_chrome_step(…, world_resources)   🐚️Shell/…:9883
       ├─ render_main_window_step(…, world_resources)     🐚️Shell/…:10314
       └─ render_panel_step(…, world_resources)           🐚️Shell/…:10379
            └─ SceneEngineHosts { world3d_states: &mut self.world3d_states, world_resources }
                 └─ interpreter::render_ui_document_step(…, hosts)   🗣️Interpreter/…:1126
                      └─ FrameworkSceneHost { …, world3d_states, world_resources }   🗣️Interpreter/…:1032
                           └─ scenes::render_component_scene_step(…, hosts)  🎞️Scenes/…:1218
                                └─ render_world3d_surface_step               🎞️Scenes/…:1336
                                     ├─ world3d_states.get_or_insert_with(World3dState::new)
                                     ├─ infinite_world::world::render_world_3d(…)
                                     └─ register_engine_surface(…, World3d, created)
```

`SceneEngineHosts` (`🎞️Scenes/…:1213`) is borrowed for exactly one chrome walk and never stored. The
shell's `world3d_states` is a field of `&mut self` at both leaf call sites, so the split borrow costs
nothing; `world_resources` comes from the frame cursor, which is a different object from `interaction`,
so the renderer call site (`🧊️renderer/🦀️.rs:13126`) has no borrow conflict either.

This is what the pre-bounded-refactor renderer did — `render_component_scene`'s `world3d_states`
parameter and its `.entry(surface_id).or_insert_with(World3dState::new)` arm survive verbatim in
`.🧬semio/…/26/07/18/DEGENERALIZE-…/before/wgpu-lib.rs:7838-7845`. The seam existed once and was lost
when the walk was made stepped; this restores it with the state maps carried in a struct instead of as
five loose parameters.

### 2.3 Feeding the map and the board

Both syncs are field-by-field against their cache, applying only what moved, and both mirror their
React host's effect ladder exactly:

- `sync_map_engine` (`⚙️EngineCanvas/…:2121`) — size, theme, descriptor, render mode, vector style, lod,
  layer visibility, layer stroke scale, interaction, camera. `map_interaction_from_scene`
  (`:2080`) is the wgpu twin of `resolveMapInteractionSync` (`🧭️TiledMapHost/🟦️.tsx:180`): routes win
  over positions, and the hover only reaches the host when its kind matches the resolved granularity.
  `map_theme_json` (`:2104`) serialises the shell `Theme` into the rgba8 document
  `MapHost::set_map_theme_from_json` reads, leaving `MapPalette`'s defaults for the fields with no
  unambiguous shell counterpart — exactly what React's own partial theme document does.
  A camera document that does not parse falls back to `fit_world_camera()`, React's boot rule.
- `sync_board_engine` (`⚙️EngineCanvas/…:2219`) — size, fixture, glyph catalogs, placement compatibility,
  selection options, selection ids, camera, hover, active utility, grid snap, grid factor, suggestion
  offset, brush weights, lod. **`parse_fixture_json` resets selection AND camera to the fixture's own
  defaults**, so a fixture pass re-applies both silently right after — the rule
  `applyFixtureToSession` (`🖥️Board2dHost/🟦️.tsx:247-257`) states explicitly, and the reason
  `fixture_applied` gates those three branches.

Both hosts are constructed on the first painted frame (`MapHost::new()`,
`ManuallyDrop::new(BoardHost::default())`) and bump `EngineSurface.scene_revision` only when something
was applied, which is what lets the GPU freshness gate reuse an unchanged texture.

---

## 3. Two production bugs the World3d lane surfaced, both fixed

Attaching `World3d` for the first time in production immediately hit two defects that could not have
been observed before, because nothing had ever driven a real World3d interaction on wgpu. Both are in
`♾️infinite/🌍️world/🦀️.rs` and both were found by a test, not by reading.

### 3.1 Every pick, hover, orbit and wheel faulted the interaction authority

`WorldInteractionRegistryBuildCursor` runs as a **precondition of every intent** and resolves each
draw's mesh out of `interaction_meshes` by `(id, version)` (`🦀️.rs:2283`, and again at the two ray-pick
revalidation sites `:3679`, `:3791`). It compared against `SceneDraw3d::mesh_version`.

But the **only** producer that reaches `world3d_draw_rebuild_admit_draw` in production is the typed
snapshot apply, and it admitted every draw with a hard-coded `0` (`:9074`, `:9124`) — the mesh's real
version is only known after `publish_world3d_mesh_lease` computes
`generation().rotate_left(17) ^ revision()`. So the comparison could never match unless the version
happened to be `0`: the probe walked past the correctly-stored entry to an empty slot and returned
`WorldInteractionStep::Fault`, which sets `authority.faulted` and kills **every** subsequent intent on
that surface. Witnessed live before the fix:

```
[DEBUG] world interaction turn=0 generation=1 step=Pending
[DEBUG] world interaction turn=1 generation=1 step=Pending
[DEBUG] world interaction turn=2 generation=1 step=Fault
```

Fix: `live_world3d_mesh_version` (`🦀️.rs:1782`) reads the version out of `mesh_versions`, which
`publish_world3d_mesh_lease` writes in the SAME transaction as the `interaction_meshes` entry
(`:8953-8963`), so it is exactly as strong a freshness witness and is what the GPU path already does
(`render_world_3d`'s own `let mesh_version = *state.mesh_versions.get(&draw.mesh_key)…`). All three
comparison sites now use it, and the two snapshot admissions pass it instead of `0`.

### 3.2 Orbit was unreachable — the right-drag was swallowed before it could plan

`WorldInteractionAuthority::step` intercepted `PointerMove + button 2 + down` to maintain the
right-click/right-drag discrimination (`right_press`/`right_dragged`, which gate the context menu) and
then **retired the intent and returned `Complete`** — before the phase dispatch that would have called
`plan_world3d_drag`. Since production only ever enqueues `wheel`/`pointer_button`/`pointer_move`
(`🧊️renderer/🦀️.rs:11546,13497,13522,13596` — the `PointerDrag` phase has no producer at all), the
`button == 2 && (alt || meta)` orbit branch and the `button == 2 && shift` pan branch of
`plan_world3d_drag` (`🌍️world/🦀️.rs:6039`) were dead code. wgpu had wheel-zoom and middle-button pan
and nothing else.

Fix (`🦀️.rs:5271-5291`): the tracker still records the drag distance, then falls through to
`plan_world3d_drag`; a planned gesture becomes an active `Plan`, and only an unplanned right-move
retires as before. The click/drag discrimination is untouched.

---

## 4. Tests

`⚙️EngineCanvas/🧪️tests/🧩️wgpu-engine-surfaces/🦀️.rs` (new), mounted at
`⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:1596-1597`. Every lane drives the **real** production paint —
`scenes::render_component_scene_step` through a real `FrameworkWidgetContext` and a real
`SceneEngineHosts` — never an attach function directly.

The committed fixture `🧪️tests/🧩️wgpu-engine-surfaces/🔣️.json` carries a `provenance` block naming each
payload's origin:

- **`world3d`** — a verbatim copy of `♾️infinite/🌍️world/🧪️tests/🌉️scene-bridge/🔣️.json`, which is
  s.procedural.generation3d's `hexagonal-mushroom-column` preview payload captured from the real
  pipeline (`parse_dsl` → `FlowHost::evaluate` → `tessellate_geometry(0.05)` → `preview_payload_from_eval`)
  in that plugin's example-geometry harness, where a `parry3d` oracle independently confirms the same
  solid. Copied rather than imported so the framework renderer never depends on a plugin crate; the
  source fixture is itself pinned back to the live kernel by
  `hexagonal_mushroom_column_preview_payload_matches_the_scene_bridge_fixture`.
- **`tiledMap`** — the `positions`/`routes`/`regions` descriptor shape `MapHost::sync_map_json`'s own
  lanes drive, as the gis 2d app publishes it through `TiledMapScene::base`.
- **`board2d`** — a verbatim copy of vector[0] of
  `♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧫️fixtures/🔣️board-ingress.json`, the
  `reasoning.mindmap.fixture` document the puzzle 2d board publishes, pinned there by
  `board_fixture_json_vectors_match_the_json_oracle`.

The oracle for the action payloads is the other implementation of these hosts: React's `World3dHost`
(`🧱️elements/🌐️World3dHost/🟦️.tsx`), `TiledMapHost` and `Board2dHost`.

```
test engine_canvas::engine_surface_attach_tests::tiled_map_and_board_windows_attach_their_engines_on_the_same_production_seam ... ok
test engine_canvas::engine_surface_attach_tests::world3d_orbit_and_wheel_emit_the_generation3d_set_camera_payload ... ok
test engine_canvas::engine_surface_attach_tests::world3d_pointer_down_emits_the_graph_domain_selection_react_dispatches ... ok
test engine_canvas::engine_surface_attach_tests::world3d_preview_window_attaches_the_world_engine_and_paints_the_tessellated_solid ... ok
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 488 filtered out; finished in 0.04s
```
(`🗑️generated/wgpu3-attach-tests.txt`; needs `RUST_MIN_STACK=536870912`, same as the node-graph lane.)

1. **`world3d_preview_window_attaches_the_world_engine_and_paints_the_tessellated_solid`** paints the
   generation3d preview window, drives the frame transaction's own `World3dSnapshot` ladder
   (`step_world3d_scene_bridge` → `step_world3d_draw_rebuild` → `step_world3d_snapshot`, exactly as
   `AppFrameTransactionPhase::World3dSnapshot` does) and asserts: the first painted frame constructs
   the `World3dState` in the shell's map; the window's draw list carries a `ScenePass3d` sized to the
   window body; that pass holds **1 draw** with mesh key `eval-extrude@solid#0` and the
   channel-qualified instance id `extrude@solid#0`; the drawn mesh carries **20 triangles / 36
   vertices** — i.e. the tessellated solid, not the 12-triangle placeholder box the old path produced;
   the scene's own selection document reaches the painted instance; and `snapshot_fault()` is `None`.
   The lane also pins the production frame cadence: attach needs more than one frame by design — the
   first paint STAGES the bridge, the ladder seals it, and only the next paint's `sync_world3d_state`
   picks the sealed lease up.
2. **`world3d_pointer_down_emits_the_graph_domain_selection_react_dispatches`** drives a press-then-
   release through `enqueue_world3d_event` + `step_world3d_interaction` — the production path the OS
   event loop uses, not a helper — and asserts the published `interactionSelect` carries
   `domainId: "graph"`, `merge: "replace"`, `method: "pick"` and the **bare channel-qualified** target
   `{granularity: "handle", id: "extrude@solid#0"}`, never `surfaceId/id`. It picks from an
   *unselected* preview on purpose: clicking an already-selected instance grabs the gumball and
   commits a `translateSelection` instead, which is also what React does.
3. **`world3d_orbit_and_wheel_emit_the_generation3d_set_camera_payload`** asserts a wheel publishes
   `setCamera` with generation3d's `📷️set-camera` payload (`{surfaceId, camera: {position[3],
   target[3], fov}}`) AND that the orbit camera actually moved, then that an alt+right drag publishes
   `setCamera` again and actually turned the camera. Both halves would have passed vacuously against a
   published-but-inert action, which is why each asserts the committed camera moved.
4. **`tiled_map_and_board_windows_attach_their_engines_on_the_same_production_seam`** paints a
   `TiledMap` and a `Board2d` window and asserts each host was constructed and fed whole (2 positions
   + 1 route reaching `MapHost.features`; 2 nodes + 1 edge reaching `BoardHost`), that the map's
   selection resolved through React's granularity rule (`position` granularity, `zurich` selected,
   `muenchen` hovered), that the board's selection survived `parse_fixture_json`'s reset, that both
   painted surfaces are composited into the window draw list under their `engine:<surface>` raster
   keys, and that both appear on the shared registration list the shell mirrors.

Teardown goes through each host's OWN retirement ladder — `begin_engine_surface_close_token` +
`close_engine_surface_step` for the registry kinds, and `begin_world3d_dynamic_retirement` +
`step_world3d_dynamic_retirement` for every `World3dState`. `World3dState`'s `WorldDynamicRegistry`s
panic in `Drop` if they are not retired, and a panic in a destructor **aborts the whole test binary**
(`thread caused non-unwinding panic`), taking every unrelated lane with it. The first cut of this file
did exactly that.

### 4.1 Neighbouring suites

`cargo test -p semio-framework-os-renderer-wgpu --lib engine_canvas` — **31 passed, 1 failed**
(`🗑️generated/wgpu3-engine-canvas-tests.txt`). The failure is
`saturated_graph_and_board_wheel_queues_preserve_cameras`, on
`assert!(!with_board_host(board_id, |host| host.defers_descriptor_sync_from_js()).unwrap())`
(`🧪️tests/🧊️wgpu-standalone/🦀️.rs:340`) — the **same** pre-existing failure
`📓️wgpu-node-graph-2026-09-10.md` §3.1 reported, on the previous wgpu lane's
`puzzle::editor::puzzle2d::engine::BoardHost` → `infinite_canvas::BoardHost` type swap. This lane
touches neither that test nor `defers_descriptor_sync_from_js`.

`cargo test -p semio-framework-os-renderer-wgpu --lib scenes::` — **111 passed, 2 failed**
(`🗑️generated/wgpu3-scenes-tests.txt`): `production_action_ingress_has_no_legacy_queue_and_text_vec_helpers_are_test_only`
and `text_editor_tests::run_menu_action_rename_…`, both the exact pair the node-graph lane already
attributed to a peer deletion of `handle_scene_wheel`/`_pointer_move`/`_pointer_button`.

`cargo test -p semio-framework-os-infinite --lib world::` — **105 passed, 10 failed**
(`🗑️generated/wgpu3-world-tests.txt`). None are this lane's:

- Every test that exercises a line this lane touched **passes**: all three
  `scene_bridge_*` lanes, all six `pick_*` lanes, `click_release_routes_to_pick_select_instead_of_empty_marquee`,
  `world_ray_pick_cursor_advances_one_triangle_or_boundary_per_grant`,
  `world_ray_pick_cursor_stale_and_interrupted_close_do_not_publish`.
- The 10 failures are `world_object_registry_enforces_capacity_revision_and_aba` (`registry.resolve`
  ABA), `prepared_world_resources_are_send_and_deduplicate_uploads`, `sync_terrain_state_…`,
  `world_authority_retains_front_plan_across_output_saturation_…`, three `world_*marquee*` lanes,
  `world_saturation_owner_blocks_new_ingress_…`, `live_renderer_retains_generation_wake_…` and
  `world_component_marquee_cursor_matches_legacy_vertex_edge_face_geometry`. This lane's eight hunks
  in that file are the two `World3dState` accessors, `live_world3d_mesh_version` + its three
  comparison sites + two admit sites, and the right-drag fall-through — none of which is on any of
  those paths. The three marquee lanes and the saturation lane drive **button 0** only
  (`pointer_button(…, 0, …)`), so the right-drag change cannot reach them.
- `live_renderer_retains_generation_wake_and_rejects_recreation_erasure_loss_and_duplicate_consumption`
  is a source-text guard over the wgpu glue/host/native/browser files. It fails on predicates this
  lane cannot have moved: `host.matches(HOST_TOKEN_FIELD).count() == 2` reads **0** in the working
  tree (`cursor_wake_requested: Option<crate::infinite_world::world::WorldCursorWakeToken>` no longer
  appears in `🏠️os-host/🦀️.rs` at all) and `glue.matches(TOKEN_FIELD).count() == 5` reads **6**. This
  lane never touched `🏠️os-host/🦀️.rs` and added no `cursor_wake` field anywhere. The working tree of
  `♾️infinite/🌍️world/🦀️.rs` also carries ~750 uncommitted lines from the previous wgpu lane, so HEAD
  is not a usable baseline here — the attribution above is by hunk, not by re-run.

---

## 5. Checks

### 5.1 Native

`cargo check -p semio-framework-os-renderer-wgpu --keep-going` — **0 errors, 18 warnings**
(`🗑️generated/wgpu3-native-check.txt`). The baseline taken before any edit this session was the same
**0 errors, 18 warnings** (`🗑️generated/wgpu3-baseline.txt`), i.e. this packet adds no warning and
removes none — the `never constructed` / `never read` warnings the node-graph lane cleared stay
cleared, and `map_host`/`board_host`, which had readers but no writers, never produced one.

### 5.2 Wasm

The wgpu shell is built by trunk for **`wasm32-unknown-unknown`** (`📦️packages/🦀️rust/Trunk.toml`).

`cargo check -p semio-framework-os-renderer-wgpu --target wasm32-unknown-unknown --keep-going` —
**0 errors, 14 warnings** (`🗑️generated/wgpu3-wasm-check.txt`). Unchanged from the node-graph lane's
count.

**Peer note for the coordinator.** Both checks were red for ~25 minutes mid-lane on a dependency this
packet does not touch: `semio-framework-plugin` failed first with
`cannot find value RETIREMENT_PROBE in this scope` (`⚛️reactor/🚪️lifetime/🦀️.rs:166,176`) and then with
``RequestCloseCursor` doesn't implement `std::fmt::Display`` (`⚛️reactor/🦀️.rs:1001`), each for a few
minutes at a time while a peer session edited that crate. Both cleared on their own. If a wgpu check
or a trunk build fails on `semio-framework-plugin`, poll rather than investigate — and note that a
red dependency makes `cargo check -p semio-framework-os-renderer-wgpu` fail *without ever compiling
this crate*, so a failure there says nothing about the renderer.

### 5.3 Not run

The `native` winit target (`bun nx run …:native -- generation3d`) is `"continuous": true` and opens a
blocking window — skipped, as in both previous wgpu lanes. No dev server was started, no browser tool
used, and the vite serve on 6018 was left alone.

---

## 6. The wgpu wasm boot recipe, for the coordinator

```bash
cd /Users/ueli/Documents/semio
CARGO_TARGET_DIR=<private-scratchpad-target> \
RUSTC_WRAPPER="" \
SEMIO_RENDERER=wgpu \
S_OS_PORT=6118 \
bun ./📜️script.ts dev procedural 3d
```

(or `.vscode/launch.json` row `🛠️dev🔧️procedural🏙️3d🧊️wgpu🌐️wasm`, `.vscode/launch.json:2077-2094`.)
Leave `SEMIO_PLUGIN_ONLY` **unset** — it drops 10 of the 11 crates the closure needs
(`📓️boot-path-audit-2026-09-09.md` §2). Then open `http://127.0.0.1:6118/?plugin=generation3d`.

**Readiness markers** (`…🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:1464-1498`):
- `[dev] Port 6118 already serving wgpu trunk at <url>` — an existing server was reused.
- `[dev] wgpu trunk serving at <url>` — a fresh boot succeeded.

**The plugin closure that must be rebuilt** — `buildPlugins("generation3d")`, the wgpu dev path's own
step 2, writing to `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️plugin-modules/`. Per
`📓️wgpu-path-audit-2026-09-10.md` §4 that tree is the **wgpu-only, unprofiled** staging root and every
one of its 11 crates is stale — `🌀️procedural` (2026-09-07), `📋️forms` (2026-08-17, the oldest at ~23
days) and the nine `flow-extension-*` crates (`🏘️bim`, `🧊️brep`, `📚️dictionary`, `📝️text`,
`🔤️primitive`, `🎨️draw`, `🔀️logic`, `🧮️math`, `📃️list`, 2026-09-06…09-08). All of them predate this
ticket's tessellation, catalogue-surface and preview fixes, so **a boot without that rebuild validates
stale plugin code, not the fixes**. Port 6118 listening is not proof of a rebuild — confirm with
`strings <staged .core.wasm> | grep <a string unique to today's source>`.

**Preconditions, in order:**

1. Re-run `cargo check -p semio-framework-os-renderer-wgpu --target wasm32-unknown-unknown`
   immediately before starting trunk (see §5.2 for this lane's result and for the peer crate that was
   intermittently red while this lane ran) — a trunk failure surfaces much later and much less legibly.
2. `buildPlugins("generation3d")`, as above.
3. **Then check, in this order:**
   - **The 3D preview** (`procedural-preview` in edit mode, `generation3d-generate-preview` after a
     generation) now paints the tessellated solid — this packet's headline. An **empty** viewport now
     means one of: the World3d attach never ran (the window body never painted a `SurfaceKind::World3d`
     slot), or the mesh-wire bridge never sealed (watch for `World3dSnapshotFault`). It no longer means
     "no host exists": one is constructed on the first painted frame.
   - **Click an instance in the preview**: the plugin should receive `interactionSelect` with
     `domainId: "graph"`, `granularity: "handle"` and a **bare** channel-qualified id such as
     `extrude@solid#0` — never `surfaceId/extrude@solid#0`. Hovering should mirror it as
     `interactionHover`, and the hover should light the matching port in the Flow window (the two
     domains now speak the same id).
   - **Wheel** over the preview → `setCamera` with `{surfaceId, camera:{position,target,fov}}` and the
     camera should visibly dolly. **Alt + right-drag** → `setCamera` again, orbiting. A plain
     right-drag pans only with `shift`; a plain left-drag is a marquee, not a camera move.
   - **The Flow window** keeps everything `📓️wgpu-node-graph-2026-09-10.md` §5 listed — that packet is
     unchanged by this one except that its attach now runs through the shared `sync_engine_scene`.
   - **A `TiledMap` or `Board2d` window** (gis 2d / puzzle 2d, not generation3d) now paints its own
     content instead of a bare panel, and its pointer/wheel dispatch is live.

### 6.1 Known gaps to watch first

- **The per-frame cadence.** The attach and the packet are driven from the retained scene paint. If
  `render_component_scene_step` is not re-entered on a frame where only a camera moved, a pan/zoom
  could look frozen while the actions still fire. The node-graph lane flagged this for the graph; it
  applies identically to the map and the board. World3d is **not** exposed to it — its pass is pushed
  into the window draw list on the same frame it is painted.
- **World3d needs more than one frame to first paint, by design** (§4 lane 1): frame N stages the
  mesh-wire bridge, the frame transaction's `World3dSnapshot` phase seals it, frame N+1 picks the
  sealed lease up. A preview that is blank for one or two frames after opening is expected; blank
  after a second is not.
- **Wire-only previews still do not paint** (`profile@wire`, `extrusion-axis@vector`) — the bridge
  drops them, a stated contract from `📓️wgpu-renderer-2026-09-10.md` §1.4, needing a `MeshEdge` page
  kind and a line pass. The solid does paint.
- **`SurfaceKind::IconRender`** is the one remaining kind with a `World3dState` host and no production
  attach: `render_icon_render` was deleted outright (its region in `🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:3947`
  is now an empty comment) and `ShellState.icon_render_states` has no writer. It is a separate packet;
  `SceneEngineHosts` is the seam it will attach through when it comes back.

---

## 7. Files changed

| file | change |
|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs` | region `🕸️NodeGraphAttach` → `🧩️EngineSurfaceAttach`; `NodeGraphSurfaceRegistration` → `EngineSurfaceRegistration` + `EngineSurfaceKindDetail`; `NodeGraphSurfaceRegistry` → `AttachedSurfaceRegistry`; `take_node_graph_surface_registrations` → `take_engine_surface_registrations`; new `register_engine_surface`, `engine_camera_from_json`, `map_interaction_from_scene`, `map_theme_json`, `sync_map_engine`, `sync_tiled_map_scene`, `sync_board_engine`, `sync_board2d_scene`, `sync_engine_scene`; `stage_node_graph_paint` → kind-general `stage_engine_scene_paint`; `SurfaceKind` imported; new test module mounted |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧪️tests/🧩️wgpu-engine-surfaces/🦀️.rs` | **new** — 4 attach lanes + the `World3dState` retirement teardown |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧪️tests/🧩️wgpu-engine-surfaces/🔣️.json` | **new** — the generation3d World3d preview payload, a tiled-map descriptor and a board fixture, with provenance |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧪️tests/🕸️wgpu-node-graph/🦀️.rs` | follows the renamed drain and the new `render_component_scene_step` signature |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs` | new `SceneEngineHosts`; `render_component_scene_step` takes it and its phases 4-8 are kind-general; new `engine_surface_clear`, `render_world3d_surface_step` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs` | `FrameworkSceneHost` carries `world3d_states`/`world_resources`; `render_ui_document_step` takes `SceneEngineHosts` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` | `sync_node_graph_surface_states` → kind-general `sync_engine_surface_states`; `render_chrome_step`/`render_main_window_step`/`render_panel_step` thread `world_resources` and build `SceneEngineHosts` at both document paint sites |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs` | `FrameBuildPhase::Chrome` passes the frame's `World3dBuildContext` into the chrome walk and calls `sync_engine_surface_states` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs` | `World3dState::snapshot_fault()`/`mesh_lease()` read accessors; `live_world3d_mesh_version` + its use at the three interaction-registry comparison sites and the two snapshot draw admissions (§3.1); the right-drag fall-through (§3.2) |

Raw logs: `🗑️generated/wgpu3-baseline.txt`, `wgpu3-attach-tests.txt`, `wgpu3-engine-canvas-tests.txt`,
`wgpu3-scenes-tests.txt`, `wgpu3-world-tests.txt`, `wgpu3-native-check.txt`, `wgpu3-wasm-check.txt`.
