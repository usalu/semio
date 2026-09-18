# 🧷️ W11a — `prepared world mesh was missing`: root cause, fix, laws, probe evidence

Packet W11a (P0). The wgpu page died mid-journey at t≈42.7 s with
`worker-present-failed: prepared frame submit step: prepared world mesh was missing`
(`🗑️generated/parity-run-12/wgpu/console.txt:8644-8646`). After the fix the whole 37-step journey runs
to completion on both renderers with **no `worker-*-failed` and no wgpu page error**
(`🗑️generated/w11a-parity-run-14/`).

## 1. Root cause — an unbacked world draw, not a residency race

None of the four suspects in the packet was the cause. Retirement and presentation are strictly
serialised (`present_step` steps `self.retirement` only `if self.pending.is_none()`, and
`admit_next_frame` refuses while `has_pending_presentation()` — which includes `self.retirement`), so
an eviction can never land between a frame's `Uploads` phase and its `Render` phase. The
`Uploads`-phase stall of 293 steps was a **healthy** mesh upload, not a stall (§3).

The actual defect is on the **build** side. `World3dBuildContext::ensure_mesh` is what puts a mesh
into the frame's `PreparedRenderUpload::Mesh` list, and the `Uploads` phase guarantees residency for
everything in that list. Five sites published a `SceneDraw3d` **unconditionally** while calling
`ensure_mesh` only under `if let Some(mesh) = state.meshes.get(…)`:

| site | draw | lease |
| --- | --- | --- |
| `🌍️world/🦀️.rs` brush preview ghost | `"brush-preview"` | `begin_world_placeholder_mesh(…, Box)` |
| `🌍️world/🦀️.rs` catalogue drop preview | `"catalogue-drop-preview"` | `begin_world_placeholder_mesh(…, Box)` |
| `🌍️world/🦀️.rs` tool-run trace | `tool_run_trace_mesh_keys` / `brush_preview_mesh_id` | `begin_world_placeholder_mesh(…, Box)` |
| `🌍️world/🦀️.rs` component vertex markers | `VERTEX_MARKER_MESH` | `ensure_primitive_mesh` |
| `🌍️world/🦀️.rs` vortex arrows | `vortex-marker`, shaft, head | `ensure_primitive_mesh` |

Every one of them reaches for a lease that is admitted **asynchronously** — `begin_world_placeholder_mesh`
takes frames to land. On the frame the placeholder is still being built the site pushed a draw naming
`key@version` with **no upload behind it**; `mesh_store.get_versioned` missed at submit, and the miss
was `Err("prepared world mesh was missing")`, which `AppPresentPhase::Render` turns into
`worker-present-failed` and the browser shell turns into a dead page. React has no such window:
`BrushPreviewGhost` mounts `<boxGeometry>` with its geometry already in hand, and a `GlbInstanceMesh`
whose loader has not resolved simply renders nothing.

The journey died on `pane-chip-engagement-toggle` → `pane-chip-search-toggle`, which is exactly where
the engagement lane arms a tool and publishes a brush-preview ghost.

## 2. Fix

### 2.1 Source — a draw is published only behind a bound lease (file:line)

- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs:11267` tool-run trace — `ensure_mesh` + push moved inside `if let Some(mesh) = state.meshes.get(..).copied()`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs:11409` vertex markers — same
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs:11428` catalogue drop preview — same
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs:11460` brush preview ghost — same
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs:12807` vortex arrows — same

### 2.2 Structural guarantee — the pinning law at the source

- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs:221` `World3dBuildContext::has_mesh_request(key, version)`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs:7247` `retain_ensured_world_draws(gpu, draws)`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs:11516` both draw lists are filtered immediately before the single `ctx.draw.push_scene_pass(ScenePass3d { … })`

**LAW (pinning): a prepared frame's world-mesh references stay resident through submit**, because a
world draw is published only when THIS frame's uploads carry its exact `(key, version)` — and the
`Uploads` phase completes before `Stage`/`Render`, with no retirement interleaved. A sixth ghost site
cannot reintroduce the fault: its draw is dropped for the one frame its mesh is still landing, which
is precisely React's behaviour.

### 2.3 GPU — a miss is a skipped draw, not a quarantine

- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🦀️.rs:3377` `encode_prepared_world_instance` now returns `Result<bool, &'static str>` and answers a non-resident mesh with `Ok(false)` — the same "not a fault" shape `encode_prepared_world_textured` has always had for a raster whose pixels have not landed.
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🧊️gpu/🦀️.rs:293` `GpuContext::missing_world_mesh`, set at `🧊️gpu/🦀️.rs:780`, taken by `take_missing_world_mesh` (`🧊️gpu/🦀️.rs:460`).
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:13747` the `Render` phase reports it **once per transition**: `os_host prepared world mesh was not resident, draw skipped: key=… version=…`, and `os_host prepared world mesh residency restored` on the way back.

The fatal is now a readable, non-quarantining refusal where React would just keep rendering.

### 2.4 Upload stall — bounded and reported

`AppPresentCursor::upload` only moves when a whole upload ITEM completes, and `ensure_mesh_step`
writes **one vertex per step** — so a healthy 293-vertex mesh froze the watchdog signature for 293
steps, and the host's own gate census could only say `phase=Some(Uploads) … stall-steps=293` with no
reason attached. The signature now carries within-item progress:

- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🦀️.rs:565` `MeshGpuTable::upload_progress() -> (u32, u32)`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🧊️gpu/🦀️.rs:474` `GpuContext::prepared_upload_progress() -> (u32, u32, usize)` (mesh vertex, mesh index, atlas page)
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:13066` `AppPresentProgress` gains a fifth term; `🦀️.rs:13390` `note_present_stall`; `🦀️.rs:13543` the presenter reads it every step.

**LAW (upload stall): the `Uploads` phase either progresses — mesh vertex, mesh index or atlas page —
or it accumulates toward `APP_PRESENT_STALL_STEPS` and is NAMED**, with the frozen counters in the
shape. A progressing upload can never be reported; a frozen one always is.

## 3. Why the 293-step "stall" was never the bug

`os_host frame gate … phase=Some(Uploads) … stall-steps=N` fired all over run-12 (`N` = 105, 171, 240,
293, 326, 331, 376, 391, 437, 581) and **always cleared on its own** two log lines later. Each was one
mesh being written a vertex at a time. It is now invisible to the watchdog by construction (§2.4).

## 4. Tests

| test | file |
| --- | --- |
| `a_world_draw_without_this_frames_upload_is_never_published` | `♾️infinite/🌍️world/🧪️tests/🔬️unit/🦀️.rs` |
| `the_scene_pass_is_published_behind_the_residency_filter` | `♾️infinite/🌍️world/🧪️tests/🔬️unit/🦀️.rs` |
| `a_non_resident_prepared_world_mesh_is_skipped_and_reported_not_faulted` | `📺️renderer/🧑‍🎨engine/🧪️tests/🔬️wgpu-renderer-async-boundary/🦀️.rs` |
| `the_present_watchdog_signature_carries_within_item_upload_progress` | `📺️renderer/🧑‍🎨engine/🧪️tests/🔬️wgpu-renderer-async-boundary/🦀️.rs` |
| `the_presentation_watchdog_names_a_frozen_cursor_once_and_a_healthy_one_never` (fixture extended with two mesh-upload cases) | `📺️renderer/🧑‍🎨engine/🧪️tests/🐕️wgpu-present-stall-watch/🦀️.rs` + `🧫️fixtures/🐕️present-stall-watch/🔣️.json` |

Existing `gumball_draws_ensure_the_plane_mesh_they_reference` and
`the_gumball_plane_is_pinned_against_pool_eviction` still pass — the gumball was the 2026-09-13
instance of this same family, fixed one site at a time; §2.2 is the general law it was missing.

## 5. Verification

| gate | result | log |
| --- | --- | --- |
| `cargo check -p semio-framework-ui --features wgpu-engine --lib -j 4` | 0 errors | `🗑️generated/w11a-ui-check.txt` |
| `cargo check -p semio-framework-os-infinite --lib -j 4` | 0 errors | `🗑️generated/w11a-infinite-check.txt` |
| `cargo check -p semio-framework-os-renderer-wgpu --lib -j 4` | 0 errors | `🗑️generated/w11a-renderer-check.txt` |
| `cargo check -p semio-framework-os-renderer-wgpu --lib --target wasm32-unknown-unknown -j 4` | 0 errors | `🗑️generated/w11a-wasm-check.txt` |
| renderer `async_boundary_tests` + `present_stall_watch_tests` | 34 passed, 1 failed (pre-existing, §7) | `🗑️generated/w11a-renderer-tests.txt` |
| `semio-framework-os-infinite --lib` (full) | 430 passed, 4 failed (pre-existing, §7) | `🗑️generated/w11a-infinite-tests.txt` |
| ui mesh-GPU registry/retirement tests | 3 passed | `🗑️generated/w11a-ui-draw-tests.txt` |
| ui `prepared`/`world`/`mesh` tests | 56 passed | — |
| wasm build + activation | exit 0 | `🗑️generated/w11a-wasm-build.txt`, `🗑️generated/w11a-activate.txt` |

## 6. Probe evidence

`🗑️generated/w11a-parity-run-14/` (37 steps, both renderers, 2026-09-18T17:04Z):

- **wgpu console: 0 `page error`, 0 `worker-*-failed`, 0 `present_step faulted`, 0 `prepared world mesh`.**
  The only new lines are the residency diagnostic's own `residency restored` transition at t≈6.2 s.
- The run reaches `generation=Generation(…)` at t≈164.8 s, against run-12's death at t≈42.7 s.
- `🗑️generated/w11a-parity-run-13/` is the first post-fix run: wgpu likewise completed all 37 steps
  with zero faults, but the React reference at 6313 had wedged (`ERR_CONNECTION_REFUSED` on its module
  graph) so its column is empty. It was restarted (`🗑️generated/w11a-react-6313.txt`) before run-14.

### Tally — 12 match / 25 differ

**match (12)** `boot`, `dismiss-tour`, `panel-artifact`, `panel-catalogue`, `panel-inspection`,
`pick-instance`, `chord-command-palette`, `chord-fullscreen`, `chord-fullscreen-exit`,
`chord-panel-anchor-left`, `chord-panel-anchor-right`, `example-picker-open`.

**differ (25)**, in five behavioural families — none is a fault, none is fixed here:

| family | steps | shape |
| --- | --- | --- |
| **A. chrome click leaks to the world surface** | `panel-tool-runs`, `panel-chat`, `panel-chat-close`, `pane-chip-engagement-toggle`, `split-gutter-drag`, `window-cap-focus`, `window-cap-close`, `example-switch` | wgpu adds `interactionHover`/`interactionSelect` React does not; the pointer reaches the world pane under the chrome overlay (W10a gesture origin) |
| **B. no wgpu twin for React's brush-mesh registration** | `pane-chip-windowoptions-toggle`, `window-reopen`, `example-picker-dismiss`, `role-viewer`, `role-editor` | React dispatches `registerBrushMesh`, wgpu dispatches nothing |
| **C. chord verbs unrouted on wgpu** | `context-menu-dismiss`, `chord-escape`, `chord-undo`, `chord-redo` | React dispatches `engagementAbort` / `undo` / `redo` / `shell.windowActivate`; wgpu dispatches nothing (W10b, W1c) |
| **D. camera / selection lane** | `orbit-drag`, `pan-drag`, `zoom-wheel`, `context-menu` | wgpu drops `noteShellCommand`/`setCamera` on orbit, `interactionSelect` on pan and context-menu, and `noteWorldNavigation`+`setCamera` on wheel (W10a CameraLocal silent orbit) |
| **E. control resolution / surface movement** | `pane-chip-search-toggle`, `pane-chip-windowcontrols`, `pane-chip-utilitybar-unfold`, `pane-chip-projection-toggle` | chip hit targets resolve `absent`/`suffix` on opposite sides; React's projection toggle closes three surfaces wgpu leaves open |

## 7. Pre-existing failures NOT caused by this packet

Both verified against `git show HEAD:<path>` — i.e. they already fail without any of this packet's edits.

1. `async_boundary_tests::presenter_ack_retirement_source_mutations_are_denied` — its source-text
   contract asserts `DRAW_SOURCE` contains `mesh_gpu_retirement_preserves_acknowledged_versions` and
   `fixed_mesh_gpu_registry_rejects_capacity_plus_one_and_returns_exact_owner`, but those tests were
   moved into `🖱️ui/🧪️tests/🔬️targets-wgpu-draw-unit/🦀️.rs`; it also asserts the renderer mentions
   `runtime_presentation_authority_and_candidate_identity_change_independently` (0 occurrences) and
   `runtime.presentation_witness_for(self.generation.0)` exactly once (2 occurrences). The tests
   themselves all pass; the stale contract is the failure. Owner: whoever moved them.
2. `board::ports::directed_dag::tests::{dag_host_loads_demo_fixture,
   dag_host_slider_overlay_preserves_language_neutral_field_labels, minimap_widget_panel_uses_square_corners,
   selected_nodes_cursor_censuses_and_emits_one_byte_per_grant}` — unrelated DAG board port, peer in-flight.
