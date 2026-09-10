# Wgpu renderer — mesh-wire bridge, app catalogue, World3d picking (2026-09-10)

Closes gaps #1, #2 and #4 of `📓️wgpu-path-audit-2026-09-10.md` §5. Renderer-engineer lane: only
`semio-framework-os-infinite`'s `♾️infinite/🌍️world`, the wgpu renderer/shell targets and the
generation3d example-geometry harness were touched. No `git` mutating command was run, no browser
tool used, no dev server started, the vite serve on 6018 untouched. All cargo work ran in a private
`CARGO_TARGET_DIR` (`…/scratchpad/target-wgpu`) with `RUSTC_WRAPPER=""` and `--keep-going`.

---

## 1. Gap #1 — `World3d` renders nothing on wgpu: the mesh-wire → snapshot bridge

### 1.1 What the audit found, re-confirmed

`sync_world3d_state` required `World3dScene.snapshot` to already be `Some(lease)` and set
`World3dSnapshotFault::Unavailable` otherwise, while every generation3d preview window publishes only
`meshes_json`/`instances_json`/`camera_json`/`selection_json` through `World3dScene::base`. Two
further facts the audit did not have, both found by reading the apply ladder itself:

- **The snapshot apply path had no geometry ingestion at all.** `step_world3d_snapshot`'s `Mesh` arm
  (`♾️infinite/🌍️world/🦀️.rs:9017`, item `flags == 30`) calls `begin_world_placeholder_mesh(…, Box)` —
  every draw a typed producer published rendered as a **placeholder box**, never real triangles. The
  `MeshVertex`/`MeshTriangle` page kinds exist in the enum and were never handled.
- **`World3dState.bound_domain_id` was never assigned anywhere in the repo.** `resolved_domain_id`'s
  own docstring says `sync_world3d_state` captures it; that function returned before it could. Every
  wgpu World3d pick/hover therefore addressed the OS's shared `world` domain with `surfaceId/`-prefixed
  ids, which generation3d's `graph` domain never sees (gap #4 — see §3).

### 1.2 Design

Geometry and the draw list travel on **two** channels, exactly as the already-working GLB asset path
does (`publish_world3d_asset_mesh_lease`): the fixed-page snapshot store carries the *draw list*
(mesh keys, per-instance transforms/colours, camera), and mesh *geometry* is published straight into
`World3dState.meshes` as a real `Mesh3dLease`. Routing 36 k floats through 64-item snapshot pages
would have been the wrong shape for the store; `Mesh3dSchema` already owns 16 MB of paged mesh bytes.

New region `//#region 🌉️World3dSceneBridge` (`♾️infinite/🌍️world/🦀️.rs:9197-9700`):

| piece | file:line | role |
|---|---|---|
| `stage_world3d_scene_bridge` | `🦀️.rs:9344` | FNV-1a content digest over `meshes_json`+`instances_json`+`camera_json`; stages a cursor only when the payload actually changed (an app republishing an identical scene costs one digest) |
| `step_world3d_scene_bridge` | `🦀️.rs:9399` | the budgeted turn: `Parse` → `Meshes` → `Pages`, each consuming `StepContext` fuel and yielding on `should_yield()` |
| `publish_world3d_scene_bridge_snapshot` | `🦀️.rs:9546` | builds **every page first**, then `world3d_snapshot_begin`/`admit_page`/`seal`, so admission can only fail on a bug; a failure aborts the write through `world3d_snapshot_abort_write(_step)` rather than leaking a slot |
| `step_world3d_mesh_close` | `🦀️.rs:9510` | one turn of the superseded-mesh close ladder — previously reachable ONLY from full-state retirement, so a live scene replacing a mesh key wedged `dynamic_mesh_close` forever |
| `step_world_placeholder_mesh_batch` | `🦀️.rs:9533` | 4 096 `mesh3d_*` writes per turn; the ladder writes one vec3/u32 per call, so a 36-vertex mesh finishes in one turn and a 20 k-vertex one in a handful instead of 60 k frames |
| `step_world3d_scene_bridge_close` | `🦀️.rs:9372` | drains a superseded lease and the staged build; polled first by `World3dDynamicRetirement::step` (`🦀️.rs:1546`) and counted in `world3d_dynamic_terminal_is_empty` (`🦀️.rs:1712`) so teardown never leaves a snapshot slot reserved |
| `sync_world3d_scene_selection` | `🦀️.rs:9669` | applies `selection_json` (ids, hover, granularity, componentIds, hoveredComponent, targets, activeObjectId, showEdges) **only when the document changed**, so an optimistic local preview between two identical refreshes is not clobbered — the rule React's host follows |

`sync_world3d_state` (`🦀️.rs:9704`) now captures `domain_id`/`domain_granularity_id`, parses
`environment_json` into `state.environment` (sun + clear colour + neutral material), syncs the
selection document, and falls back to the bridge lease when the producer published no snapshot of its
own. A scene whose bridge is still building reports **no fault** — `Unavailable` is now reserved for a
scene that genuinely has neither snapshot nor stageable payload.

**Mesh publication reuses the existing ladder instead of duplicating it.** `WorldPlaceholderMeshCursor`
gained a `WorldMeshSource` (`🦀️.rs:7168`) with two variants — `Placeholder(kind)` (the marker
primitives) and `Inline(WorldMeshBuffers)` (the wire buffers) — plus independent `vertex_items`/
`index_items` (a tessellated mesh is welded; the marker primitives emit one vertex per face corner).
`WorldPlaceholderMeshCursor::inline` (`🦀️.rs:7223`) validates the schema and rejects an out-of-range
index before claiming any authority credit. `WorldMeshBuffers` (`🦀️.rs:7473`) — already the exact
`meshes_json.data` shape, including its `compute_normals` — was `#[cfg(test)]` and is now production;
a wire buffer set with no/short normals gets flat normals computed once at parse time.

**Multi-draw snapshots.** The apply ladder supported exactly one draw (`flags == 30`, hard-coded draw
index 0). A generation3d preview publishes one draw per previewed channel, so a new `Mesh` item
`flags == 31` was added (`🦀️.rs:9055`) carrying both this draw's credits (`indexes[0..2]`) and the
whole snapshot's (`indexes[3..6]`); the first ordinal claims the draw permit and opens the rebuild,
each ordinal admits its own draw, and `World3dSnapshotApplyCursor.draw_index` (`🦀️.rs:8926`) routes
the `Instance` items that follow. `flags == 30` is byte-for-byte unchanged, so the fem 3d plugin's
existing producer (`✏️s/🔌️plugins/🏗️fem/…/🧵️session/🦀️.rs:2577`) is untouched.

The bridge is driven from the frame transaction's own `World3dSnapshot` phase, before the draw
rebuild (`📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:11419`).

### 1.3 Test — a real generation3d mesh, not a synthetic one

`🧰️framework/…/♾️infinite/🌍️world/🧪️tests/🌉️scene-bridge/🔣️.json` (new) is the actual preview payload
`hexagonal-mushroom-column` produces, captured from the example-geometry harness running the real
pipeline natively (`parse_dsl` → `FlowHost::evaluate` → `tessellate_geometry(0.05)` →
`preview_payload_from_eval`) — three meshes (one 20-triangle/36-vertex solid plus two wire-only
previews), three channel-qualified instances, the window's own camera measure. Its `provenance` block
names the `parry3d` oracle that independently confirms the same solid in that harness
(volume 3.897114317029974, bbox ±0.5 × ±0.4330127 × 0..6).

Three lanes in `♾️infinite/🌍️world/🧪️tests/🔬️unit/🦀️.rs:2761-2910`:

```
test world::tests::scene_bridge_renders_the_generation3d_preview_payload_into_a_snapshot ... ok
test world::tests::scene_bridge_honours_selection_hover_camera_and_sun ... ok
test world::tests::scene_bridge_binds_the_apps_interaction_domain_for_world_picking ... ok
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 320 filtered out
```

They assert the exact triangle count (`lease.schema().indices / 3 == 20`) and vertex count (36) of the
published `Mesh3dLease` — i.e. that the bridged geometry is the tessellated solid and **not** the
12-triangle placeholder box the old path would have produced — one draw, the wire-only meshes
correctly dropped, channel-qualified instance ids preserved, selection/hover applied, the camera
measure honoured to 1e-4 and the sun direction to 1e-5.

The fixture is pinned back to the live kernel by
`hexagonal_mushroom_column_preview_payload_matches_the_scene_bridge_fixture`
(`✏️s/…/📚️examples/🧪️tests/🧩️geometry/🦀️.rs:434`), which regenerates the payload through the real
editor pipeline and fails if mesh ids, triangle/vertex counts, instance ids or the camera drift — so
the framework never depends on the plugin, and the fixture cannot rot:

```
test hexagonal_mushroom_column_preview_payload_matches_the_scene_bridge_fixture ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out
```

### 1.4 Known limit, deliberately scoped

Wire-**only** previews (`edgePositions` with no `positions`/`indices` — the `profile@wire` and
`extrusion-axis@vector` entries above) still do not paint on wgpu. They need the `MeshEdge` page kind
and a line pass, which is a separate packet; the bridge drops them rather than faulting, and the test
asserts that drop explicitly so it is a stated contract, not an accident.

---

## 2. Gap #2 — the app-static catalogue on wgpu

The catalogue rides the reserved `framework.section.catalogue` retained surface as a
`paged_text_carrier` of `Component::Text` leaves. The wgpu shell has no `UiDirtyScope`/`refresh-ui`
batch to hang it off, so it is fetched through the **same** `SurfaceVisible` → `AdvanceRetained`
protocol every window body already uses — no second channel invented.

| piece | file:line |
|---|---|
| `read_paged_text_document` | `🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:1207` — reassembles the payload by walking the record graph from `header.root` (not page order), refusing a cyclic graph |
| `ShellState.app_catalogue_json` / `.app_catalogue_instance` | `🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:2265` — the once-per-instance cache, mirroring React's `appCatalogue` + `uiRefreshWantsCatalogue`'s "full scope only" rule |
| `refresh_app_catalogue` | `…🐚️Shell/…/🦀️.rs:3263`, called from `refresh_ui` at `:3209`; a failed fetch keeps the previous catalogue instead of blanking the palette |
| `publish_app_catalogue` | `…🐚️Shell/…/🦀️.rs:3292` |
| `node_graph_set_catalogue_json` | `🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:1668` — installs it on `GraphHost::set_catalogue_json` / `FlowHost::set_host_catalogue_json`, both of which had **zero** production call sites |

Also removed the stale `graph.catalogue_json` payload-limit check in the wgpu Interpreter
(`🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:136`) — the field left `NodeGraphScene` with the
catalogue-surface change and was breaking the crate's own build.

Test (`🗣️Interpreter/🧪️tests/🛍️wgpu-app-catalogue/🦀️.rs`, new) pins the reader's whole contract — *the
payload is the depth-first concatenation of every `Component::Text` leaf under the document root* —
by publishing that document shape as `UiNodeRecord`s, deliberately including an intermediate page
level so a reader that only walked the root's direct children would fail:

```
test interpreter::app_catalogue_tests::app_catalogue_reader_returns_the_empty_payload_for_an_app_without_a_catalogue ... ok
test interpreter::app_catalogue_tests::app_catalogue_reader_walks_a_payload_larger_than_one_text_leaf ... ok
test interpreter::app_catalogue_tests::app_catalogue_section_document_round_trips_through_the_wgpu_reader ... ok
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 482 filtered out
```

The producer half (`paged_text_carrier`'s chunking) is covered where it lives, in the plugin crate.
A first cut of this lane drove `paged_text_carrier` directly and minted ids by walking `BuiltNode`;
that scaffolding read only the first leaf of a multi-leaf carrier (`BuiltChildren`'s consuming
iterator did not yield what the census counted), which made the lane pass on single-leaf payloads and
fail on paged ones — a bug in the test's own tree walk, not in the reader. It was replaced rather than
patched: the reader is what this lane exists to hold, and the record-level shape holds it directly.

**New finding blocking the last mile.** The native check emits *variants `Dag` and `Flow` are never
constructed* for `NodeGraphEngine` (`⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:33`) — on wgpu the
node-graph **engine host itself is never constructed in production**; only tests build one. The
bespoke `node_graph_states` pointer/wheel dispatch the audit §1.2 found is real, but it dispatches
into a surface whose `node_graph` is `None`. So the wgpu Flow window has no node-graph host to render
*at all*, catalogue or not. The fetch/cache/push side is now complete and correct; wiring
`NodeGraphEngine` construction from a `NodeGraphScene` is a separate, larger packet and is the real
blocker for the Flow window on wgpu.

---

## 3. Gap #4 — World3d picking/hover parity

`sync_world3d_state` (`🦀️.rs:9704`, capture at `:9710-9711`) now captures `World3dScene.domain_id`/`domain_granularity_id` onto
`bound_domain_id`/`bound_domain_granularity_id`. Everything downstream was already written for it and
was simply never fed:

- `resolved_domain_id`/`resolved_domain_granularity_id` (`🦀️.rs:10368/10374`) now return generation3d's
  `"graph"` / `"handle"` instead of falling back to the OS `world` / `item` domain.
- The bounded-action pick/hover authority (`🦀️.rs:3102`, `:3175`) branches on `bound_domain_id`, so
  target ids are emitted **bare** — `extrude@solid#0` — instead of `surface-1/extrude@solid#0`. That
  is byte-identical to what React's `World3dHost` dispatches, and byte-identical to the port id the
  node graph's own picks use, which is what makes hover bidirectional between graph and preview.
- `apply_world_action_preview`'s `interactionSelect`/`interactionHover` arms (`🦀️.rs:10483`, `:10492`)
  gate on `resolved_domain_id`, so a `graph`-domain action now applies locally on wgpu.

`scene_bridge_binds_the_apps_interaction_domain_for_world_picking` proves it end to end on the bridged
geometry: it ray-picks through the fixture camera's own target, gets `extrude@solid#0` back from
`pick_instance_at` (i.e. the pick hits the *bridged* triangles, which is only possible because §1
landed), and asserts the emitted `interactionSelect` carries `domainId: "graph"`,
`granularity: "handle"` and the bare channel-qualified id — plus the same for `interactionHover`.

---

## 4. Checks

### 4.1 Native

`cargo check -p semio-framework-os-renderer-wgpu --keep-going` — **0 errors, 37 warnings**
(`🗑️generated/wgpu-native-check.txt`). None of the 20 warnings attributed to this crate come from the
new code; they are pre-existing dead-code/unnecessary-qualification warnings, the most load-bearing
being the `NodeGraphEngine` variants-never-constructed one reported in §2.

Two pre-existing compile errors in the crate had to be fixed to reach a green check at all — both
peer fallout, both in this crate's own files:
- `ArtifactEvent::DocumentBackbone` non-exhaustive match (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:3708`),
  matched explicitly with an honest "no port to route it to in channel v12" note rather than a wildcard.
- the stale `NodeGraphScene.catalogue_json` limit check (§2).

Test-target fixes in the same crates, same category: `WorldTerrainMeshCursor::new`'s tuple signature
(3 call sites, `♾️infinite/🌍️world/🧪️tests/🔬️unit/🦀️.rs:270,302,310`) and
`puzzle::editor::puzzle2d::engine::BoardHost` → `infinite_canvas::BoardHost`
(`⚙️EngineCanvas/🧪️tests/🧊️wgpu-standalone/🦀️.rs:144,362`).

`cargo test -p semio-framework-os-infinite --lib world::tests` — **105 passed, 10 failed**
(`🗑️generated/wgpu-world-suite.txt`). Every one of the 10 is peer-owned in-flight work
(`world_marquee_*`, `world_object_registry_*`, `world_saturation_*`, `world_authority_*`,
`prepared_world_resources_*`, `live_renderer_*`, `sync_terrain_state_*`); a grep of each body for
`sync_world3d_state` / `scene_bridge` / `placeholder` / `WorldMeshBuffers` returns 0 hits in all ten.
Two sync-contract lanes that this change **fixed** rather than broke:
`sync_parses_selection_targets_and_active_object` and `sync_world3d_state_captures_scene_bound_domain`
could never have passed before (sync returned before touching either).

`cargo test -p semio-framework-os-renderer-wgpu --lib` cannot complete: a peer's
`async_boundary_tests::interrupted_frame_maintenance_execution_restores_before_incremental_recovery`
overflows its stack and `SIGABRT`s the whole test binary before any result line, single-threaded too
(`🗑️generated/wgpu-lib-suite.txt`). The three catalogue lanes were therefore run filtered on that same
binary, where they pass (§2). Four separate peer breakages landed *during* this session and each
cleared on its own within the hour (`semio-framework-os-kernel`'s `ArtifactOwnedSprEditAuthority`,
`semio-s-artifact-puzzle-3d`'s clipboard API and then its example operations, `semio-framework-os-flow`'s
`Arc<HashMap<_, OperatorInfo>>` host field) — a filtered run of this crate needed several retries to
land at all, worth knowing when reading these logs.

### 4.2 Wasm

The wgpu wasm shell is built by trunk (`📦️packages/🦀️rust/Trunk.toml`, target `🌐️.html`,
dist `.🧬semio/🦑️repo/⚡️cache/📺️renderer-modules/🧊️wgpu`), whose triple is **`wasm32-unknown-unknown`**
(the crate's `[target.'cfg(all(target_arch = "wasm32", target_os = "unknown"))'.dependencies]` block
pulls `wasm-bindgen`/`js-sys`/`web-sys`).

`cargo check -p semio-framework-os-renderer-wgpu --target wasm32-unknown-unknown --keep-going` —
**12 errors, all in `semio-s-artifact-puzzle-3d`** (`🗑️generated/wgpu-wasm-unknown-check.txt`): a peer's
in-flight `ArtifactEditor` clipboard refactor (`copy_fragment`/`cut_operations` arity,
`ClipboardError`/`ClipboardFragment`/`PastePlacement` imports, `dsl::JsonValue::is_object`). The wgpu
crate depends on `puzzle`, so the compiler never reaches this lane's files.

Falling back to the crate that actually holds the bridge,
`cargo check -p semio-framework-os-infinite --target wasm32-unknown-unknown --keep-going` — **6 errors,
all in `semio-framework-os-kernel`'s `🔨️modules/🏪️store/🦀️.rs:6971-6993`**
(`🗑️generated/wgpu-wasm-infinite-check.txt`), a second, independent peer refactor.

**So the wasm target is blocked in two unrelated peer crates today, not by this lane.** Nothing in
the new code is target-gated: the bridge is plain `std` + `serde_json` in a file that already builds
for wasm, and `read_paged_text_document` only walks `ui_contract` records. Re-run both commands once
the two peer crates settle.

### 4.3 `native` target — skipped, correctly

`bun nx run @semio-tech/framework-renderer-wgpu:native -- generation3d` is **not** a headless smoke and
was not run. `project.json`'s `native` target is `"continuous": true` and its script
(`📦️packages/🦀️rust/📜️script.ts:280`, `NativeRunScript`) prepares plugins, starts an asset server and
then `runNativeBinary(…)`s the winit app, which opens a native window and blocks. Per the task brief:
skipped.

---

## 5. What a wgpu wasm boot verification needs next

The recipe is unchanged from `📓️wgpu-path-audit-2026-09-10.md` §6:

```bash
cd /Users/ueli/Documents/semio
CARGO_TARGET_DIR=<private-scratchpad-target> \
RUSTC_WRAPPER="" \
SEMIO_RENDERER=wgpu \
S_OS_PORT=6118 \
bun ./📜️script.ts dev procedural 3d
```

(or `.vscode/launch.json:2077-2094`, row `🛠️dev🔧️procedural🏙️3d🧊️wgpu🌐️wasm`). Leave `SEMIO_PLUGIN_ONLY`
unset — it drops 10 of the 11 crates the closure needs.

**Readiness markers** (`…🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:1464-1498`):
- `[dev] Port 6118 already serving wgpu trunk at <url>` — reused.
- `[dev] wgpu trunk serving at <url>` — fresh boot succeeded.
Then open `http://127.0.0.1:6118/?plugin=generation3d`.

**Preconditions this pass adds to that list, in order:**

1. **The two peer wasm blockers in §4.2 must clear first.** Trunk builds
   `wasm32-unknown-unknown`; while `semio-s-artifact-puzzle-3d` and `semio-framework-os-kernel` do not
   compile for that triple, `trunk build` cannot produce a shell at all. Verify with the two
   `cargo check --target wasm32-unknown-unknown` commands above **before** starting trunk — a trunk
   failure surfaces much later and much less legibly.
2. **Rebuild the plugin closure.** The audit's §4 staleness (1.4–23 days, all 11 crates predating this
   ticket's fixes) still stands; `buildPlugins("generation3d")` runs as step 2 of the wgpu dev path.
   Confirm with `strings <staged .core.wasm> | grep <a string unique to today's source>` — port
   6118 listening is not proof of a rebuild.
3. **Then check, in this order:**
   - `procedural-preview` (edit mode) shows the tessellated solid rather than an empty viewport or a
     box — that is §1 live. A `[DEBUG]`-free frame with real geometry is the marker; a frame fault
     reading `world3d scene mesh-wire bridge faulted`
     (`🧊️renderer/🦀️.rs:11421`) is the bridge reporting a capacity/schema problem with the live payload.
   - Click a preview instance and watch the node graph light up the matching port — that is §3 live
     (bare `graph`/`handle` targets).
   - The Flow window's canvas spotlight will still be **empty**, and that is expected: §2's finding is
     that the wgpu node-graph engine host is never constructed in production. Do not read that as a
     catalogue-fetch failure; check `ShellState.app_catalogue_json` instead (it should be the ~100 KB
     `flow_app_catalogue_json` payload).

---

## 6. Files changed

| file | change |
|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs` | the bridge region, `WorldMeshSource`/inline mesh cursor, `flags == 31` multi-draw apply, `sync_world3d_state` domain/environment/selection capture, mesh-close pump, 8 new `World3dState` fields |
| `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧪️tests/🔬️unit/🦀️.rs` | 3 new scene-bridge lanes + helpers; `WorldTerrainMeshCursor::new` call-site repair (peer fallout) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧪️tests/🌉️scene-bridge/🔣️.json` | **new** — the committed generation3d preview payload + expectations |
| `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🧩️component.rs` | re-export `WORLD3D_SNAPSHOT_PAGE_CAPACITY`/`_PAGE_ITEM_CAPACITY` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs` | drive `step_world3d_scene_bridge` from the frame transaction's `World3dSnapshot` phase |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs` | `read_paged_text_document`; drop the stale `nodeGraph.catalogue` limit check; mount the new test module |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🧪️tests/🛍️wgpu-app-catalogue/🦀️.rs` | **new** — 3 catalogue reader lanes |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` | catalogue fetch + per-instance cache + publish; `ArtifactEvent::DocumentBackbone` arm |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs` | `node_graph_set_catalogue_json` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧪️tests/🧊️wgpu-standalone/🦀️.rs` | `BoardHost` path repair (peer fallout) |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🧪️tests/🧩️geometry/🦀️.rs` | the scene-bridge fixture provenance lane |

Raw logs: `🗑️generated/wgpu-native-check.txt`, `wgpu-wasm-unknown-check.txt`,
`wgpu-wasm-infinite-check.txt`, `wgpu-scene-bridge-tests.txt`, `wgpu-world-suite.txt`,
`wgpu-generation3d-provenance.txt`, `wgpu-app-catalogue-tests.txt`, `wgpu-lib-suite.txt`.
