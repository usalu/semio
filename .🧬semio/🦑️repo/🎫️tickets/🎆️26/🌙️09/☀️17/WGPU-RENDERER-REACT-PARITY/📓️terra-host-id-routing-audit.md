# Host-Identity Routing and Camera-Settle Audit

## Scope and evidence

Read-only source audit on 2026-09-21. No build, browser run, or production edit was made. Paths and line numbers below refer to the checked-out sources at review time. The three findings are reachable through retained production ingress; they are not fixture-only mismatches.

The post-migration contract is clear in the interpreter: a queued `SceneInteractionIntent` carries both a short-lived `tree_revision` and the stable document `surface_generation` ([Interpreter WGPU:1468-1493](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs)). It validates both before it runs ([2103-2110](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs)). That prevents an already-queued event from acting on a changed candidate tree. It must not turn an ordinary republish into a cancellation of an already captured interaction.

## 1. Passive-list capture stores a tree revision as its document lifetime

**Confidence: high; live defect.**

The generic retained-scene branch sends `intent.tree_revision` for passive `PointerDown`, `PointerUp`, and `PointerMove` ([Interpreter WGPU:2291-2302](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs)). Those are the `Table`, `Vfs`, `GraphTimeline`, `BlockList`, `DiffView`, and `EventFeed` route after bespoke surface kinds have been handled.

`passive_scene_pointer_button` accepts that value as `document_generation` ([Scenes WGPU:2505-2558](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs)); `passive_scene_pointer_move` passes it to `update_scene_list_transfer` ([2584-2586](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs)); and that function cancels the transfer whenever the stored and current values differ ([2404-2406](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs)). Thus a down at tree revision *r*, ordinary same-host refresh, then move at *r + 1* cancels a valid transfer even though the mounting document generation is unchanged.

The rendering half is already correct after the recent host migration: `FrameworkSceneHost::paint_slot_step` supplies `self.window_generation` to `render_component_scene_step` ([Interpreter WGPU:2387-2390](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs)). The local `document_generation = tree_revision` assignment at [2660](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs) feeds an unused `FrameworkSceneHost` field, so it is not the present transfer cancellation path.

### Minimal repair

Pass `intent.surface_generation` in the three passive pointer calls at Interpreter WGPU:2293, 2297, and 2301. Retain the existing tree-revision comparison at 2106: it is queue freshness, not capture lifetime. No wire or schema change is needed because the intent already carries the required generation.

### Fail-first law

The newly registered real-Shell law [`a_published_table_transfer_survives_a_same_host_document_refresh`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-shell-input/🦀️.rs:1608) is the right regression: physical down, same-key source republish and paint acknowledgment, move to a distinct destination, then up must emit exactly the one fixture destination action. Its helper confirms the old and refreshed targets are the same component host before it moves ([1620-1690](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-shell-input/🦀️.rs)). It should be red before the three substitutions and green after them.

## 2. World debug ledger writes a host key but reads a wire key

**Confidence: high; live diagnostic/probe defect.**

The actual World paint state is keyed by `scene.host_id`, and the one writer records the live camera under that same host identity ([Scenes WGPU:3182-3198](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs)). `WORLD_CAMERA_LEDGER` is therefore host-keyed in fact, although its docs and function parameters still call the argument `surface_id` ([Interpreter WGPU:3849-3871](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs)).

`mesh_stats_for_scene` preserves the canonical wire `surface_id` in its public `DumpMeshSurface`, but looks up `live_camera` with that wire ID ([Interpreter WGPU:3702-3713](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs)). After host-ID migration the two IDs differ, so a painted World reports `liveCamera: null` to `dumpMeshStats` even though the writer ran. This does not alter World drawing or interaction; it makes the browser parity diagnostic lose the painted live orbit.

### Minimal repair

Use `scene.host_id` for the lookup at Interpreter WGPU:3712. Rename the internal ledger function parameters and comments from `surface_id` to `host_id`. Keep `DumpMeshSurface.surface_id` as the public canonical document address; internal host identity must not leak into the diagnostic wire.

### Fail-first law

The new [`sibling_world_diagnostics_read_their_exact_host_camera_and_keep_the_public_surface_id`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🧪️tests/🔬️wgpu-introspection/🦀️.rs:149) is exact. It writes distinct cameras at two host IDs while each observation retains public `surfaceId = "shared-world-document"`; the current lookup returns `None` and fails. It proves both correct internal ownership and non-leakage of the public ID.

## 3. React camera dispatch settles at 120 ms; native Canvas2d/Paint2d remains 350 ms

**Confidence: high for Canvas2d/Paint2d; current World native implementation needs its own source anchor.**

The React authority is not 100 ms. `Canvas2dHost` exports `CAMERA_SYNC_DEBOUNCE_MS = 120` ([Canvas2dHost:741-743](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📐️Canvas2dHost/🟦️.tsx)); each local camera update replaces the pending timer and publishes `setCamera` after that exact trailing interval ([789-793](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📐️Canvas2dHost/🟦️.tsx)). `World3dHost` imports that same constant ([139](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx)) and uses it for its own trailing `setCamera` ([5865-5875](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx)).

Native `schedule_scene_camera_dispatch` documents 350 ms and writes `app_now_ms() + 350.0` for Canvas2d/Paint2d ([Scenes WGPU:1321-1335](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs)). Its cursor makes each deadline a single `setCamera` action only once the stored `at_ms` is due ([1337-1400](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs)), so changing the delay does not weaken the trailing-debounce or bounded-owner behavior.

The native comment says a World runtime deadline has the same 350 ms window, but the named `AppRuntime::world3d_camera_dispatch_deadlines_ms` implementation is not present in the current renderer source search; only this reference remains. Treat that comment as a stale anchor until the World owner source is located. The React side nevertheless establishes that any native World trailing dispatch should share 120 ms, not a separate 100 ms or 350 ms policy.

### Minimal repair and law

Use the already added shared `wheel.settleDelayMs` contract at **120 ms** for native scene-camera scheduling; do not create a second native timing constant. Preserve immediate local viewport mutation, host-ID-keyed deadline replacement, owner validation, and one-action cursor semantics. The complete law is: after the final user wheel/pan event, there is no `setCamera` publication at 119 ms and exactly one at 120 ms; a later event resets that deadline. The React mounted oracle now testing 119/120 is the browser authority. Native must get an equivalent real-input/cursor law before changing `+350.0`.

## Recommended order

1. Run the existing Native108 Table refresh and sibling World diagnostic laws to establish their intended red receipts.
2. Replace only passive-list call-site generations and World ledger lookup/parameter names.
3. Use the corrected 120-ms schema fixture and the real React 119/120 oracle to drive the native camera deadline change. Do not retain 100 ms in a test, fixture, or production constant.
