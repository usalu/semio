# 🥽️ W12b — the wgpu twin of React's `registerBrushMesh`

Packet W12b, family **B** of `📓️w11a-prepared-world-mesh-missing.md` §6. React's `World3dHost`
announces every loaded GLB's collision geometry to the puzzle guest; the wgpu host dispatched
**nothing at all**, so the guest's brush and volume-brush utilities had no collision body on this
renderer. This packet implements the announcement lane in the world/mesh lane, per window instance,
with React's args, through the same action ledger and the same `gesture` provenance.

---

## 1 — What React actually does

### 1.1 The trigger

`BrushMeshRegistrar` (`🌐️World3dHost/🟦️.tsx:2415`) is mounted **once per mesh url per window**
(`:7448`, inside `brushMeshUrls.map`, where `brushMeshUrls` is every distinct `url` the scene lane's
`meshes` carries). It `useLoader(GLTFLoader, meshAssetTransportUrl(url))`s the GLB and, in an effect
keyed `[gltf, onRegister, revision, url]`, welds the collision mesh (`extractGlbCollisionMesh`,
already in the `GLB_MESH_FRAME_ROTATION_X = π/2` frame) and calls `handleRegisterBrushMesh`.

So the announcement fires when:

| # | trigger | why |
| --- | --- | --- |
| 1 | the GLB's `useLoader` promise resolves | the geometry exists for the first time |
| 2 | the url set changes (**example switch**) | a new registrar mounts per new url |
| 3 | the host remounts (**window reopen**, **role-viewer**, **role-editor**) | every registrar mounts afresh |
| 4 | `revision` changes | `brushMeshRevisions.generation + brushMeshRevisions.urls[url]`, driven by the guest's own `interactionJson.meshResidency` / `meshReuploadUrls` (`:6061`) |
| 5 | `onRegister`'s identity changes | React-only re-render noise — `dispatchBrushMesh` depends on `onAction`, so any ShellHost re-render re-fires every mounted registrar. This is why a pure chrome step (`pane-chip-windowoptions-toggle`) journals rows at all. |

### 1.2 The args

`dispatchBrushMesh` (`🌐️World3dHost/🟦️.tsx:5985`) sends
`{ surfaceId: node.surfaceId, windowId: windowInstanceId ?? node.surfaceId, ...args }` on the scene
host's own controller, awaiting each command's settle before the next. Two arg shapes:

| shape | keys | when |
| --- | --- | --- |
| **announce** | `surfaceId`, `windowId`, `url`, `digest` | the page-wide `puzzle3dBrushMeshRegistry` already holds `(url, digest)`, or holds the digest under a sibling id (`alias`) |
| **page** | `surfaceId`, `windowId`, `url`, `digest`, `page`, `pageCount`, `positionsB64?`, `indicesB64?` | a first upload — `puzzle3dBrushMeshPages` (`🛠️ShellHelpers/🟦️.tsx:3427`) |

`digest` is unkeyed BLAKE3 hex over the positions' little-endian `f32` bytes followed by the indices'
little-endian `u32` bytes — byte-identical to the guest's `brush_mesh_digest`
(`✏️editor/⏳️precompute/🦀️.rs:306`). A page carries at most `PUZZLE3D_MESH_PAGE_VALUES` = 1 024
values; positions fill each page first and the indices stream continues in whatever is left, so the
run is dense and its last page is the only partial one. Max run 384 pages.

### 1.3 The rows run-14 measured

`🗑️generated/w11a-parity-run-14/steps.json`, React column, every row
`action=registerBrushMesh origin=gesture controller=1 outcome=applied|open`:

| step | rows | `puzzle3d-main-top` | `puzzle3d-main-perspective` |
| --- | --- | --- | --- |
| 9 `pane-chip-engagement-toggle` | 6 | 3 | 3 |
| 14 `pane-chip-windowoptions-toggle` | 10 | 5 | 5 |
| 18 `window-reopen` | 22 | 11 | 11 |
| 34 `example-switch` | 59 | 30 | 29 |
| 35 `example-picker-dismiss` | 19 | 9 | 10 |
| 36 `role-viewer` | 8 | 3 | 5 |
| 37 `role-editor` | 28 | 14 | 14 |

**Every step carries rows for BOTH window instances** — that is the load-bearing fact: the
announcement is scoped to a window, never to the document. The probe journal carries no `args`
column for React (the input ledger records `{inputSeq, controllerId, action, origin, windowId,
causedBy, outcome}` only), so the arg table above is read from React's source, not from the run.

---

## 2 — The wgpu twin (file:line)

### 2.1 The lane — `♾️infinite/🌍️world/🦀️.rs`, region `🥽️WorldBrushMeshAnnounce`

| site | what |
| --- | --- |
| `🦀️.rs:13703` | constants: `WORLD_BRUSH_MESH_PAGE_VALUES` 1 024, `…_COMMAND_RAW_BYTES` 8 192, `…_PAGE_PADDING_CHARS` 8, `…_MAX_PAGES` 384, `…_REUPLOAD_CLAIMS` 2 — the guest's and React's own numbers |
| `🦀️.rs:13729` | `WorldBrushMeshRun` — one surface's open page run, holding the mesh's whole wire payload read back once when the run opened |
| `🦀️.rs:13761` | `WorldBrushMeshRegistry` + `WORLD_BRUSH_MESH_REGISTRY` — the **process-wide** claim ledger, the twin of React's module-singleton `puzzle3dBrushMeshRegistry`: the first window pages the bytes, every later window announces by `{url, digest}` alone |
| `🦀️.rs:13794` | `world_brush_mesh_revision` — React's `generation + urls[url]` |
| `🦀️.rs:13815` | `sync_world3d_brush_mesh_feedback` — folds the guest's `interactionJson.meshResidency` / `meshReuploadUrls` in; a residency that FELL voids every claim and bumps `generation`, a standing request claims at most twice and bumps that url's revision |
| `🦀️.rs:13859` | `world_brush_mesh_page_capacity` — React's `puzzle3dBrushMeshPageCapacity`, over the same probe envelope |
| `🦀️.rs:13871` | `world_brush_mesh_payload` — the resident `Mesh3dLease`'s own `page_cursor(Positions)` then `page_cursor(Indices)` raw bytes. That is already the layout React welds and the guest hashes, and the wgpu decoder bakes `glb_world_frame()` into the mesh it seals (W3d §3), so both renderers announce geometry in the same frame |
| `🦀️.rs:13921` | **`step_world3d_brush_mesh_announce`** — the one entry point: at most ONE action per call, in three ordered shapes (advance an open run → announce an identity the registry holds → open a run) |
| `🦀️.rs:13961` | `world_brush_mesh_next_page` — pages the payload, and only the run's LAST page confirms the identity in the registry and marks the surface announced |
| `🦀️.rs:13997` | `world3d_brush_mesh_announce_pending` — the per-frame predicate |
| `🦀️.rs:1464` | `World3dState.brush_mesh_announced` (url → the revision THIS surface announced at), `brush_mesh_run`, `brush_mesh_feedback_digest` |
| `🦀️.rs:11292` | `sync_world3d_state` now calls `sync_world3d_brush_mesh_feedback(state, world.interaction_json.as_deref())` — the wgpu world had never read `interactionJson` at all |
| `📦️packages/🦀️rust/Cargo.toml` | new `semio-framework-hash` edge for the digest |

### 2.2 The dispatch — `📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs`

| site | what |
| --- | --- |
| `🦀️.rs:12019` | new `AppFrameTransactionPhase::BrushMesh`, entered when `SceneCamera` completes and leaving to `Build` |
| `🦀️.rs:11961` | `FrameTransaction.brush_mesh_cursor` — a plain index into `world3d_states`, so the phase owns no closable authority and `close_step`/`terminal_is_empty` are untouched |
| `🦀️.rs:12146` | the phase arm: **one `registerBrushMesh` per world surface per frame**, pushed into `frame_actions` |
| `🦀️.rs:10952` | `has_pending_asset_decode` also answers `world3d_brush_mesh_announce_pending`, so a surface mid-run keeps its own frames coming |

The row therefore leaves through `FrameDeferredWork::Action` →
`ShellState::dispatch_gesture_action` (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:7541`), which stamps
`SHELL_DISPATCH_ORIGIN_GESTURE` on the chrome ledger's one tap
(`crate::interpreter::note_dispatched_action`) — **`origin: "gesture"`, exactly React's**. `windowId`
and `surfaceId` are both `state.surface_id`, which on this target IS the window instance id
(`puzzle3d-main-top` / `puzzle3d-main-perspective`; the same value `orbit_camera_action` puts in
`setCamera.windowId`, matched against React in run-14 step 20).

### 2.3 Back pressure

React awaits each page's settle before queueing the next, so a user action that lands mid-run waits
behind ONE page rather than behind a whole 72-command run. The wgpu twin gets the same bound for
free: one page per surface per frame, and `frame_actions` is the FIFO the frame-deferred lane
dispatches serially.

### 2.4 Deliberate divergence

React's trigger #5 (re-render identity) is re-announcement NOISE — the guest answers most of those
rows with `adopt_shared_mesh` or a refusal, which is exactly the storm `request_reupload`'s docstring
and wave B46/B48 were written against. The wgpu twin answers triggers #1–#4 (the ones that carry
information) and does **not** reproduce #5. That is why a wgpu step's row COUNT will be lower than
React's while the set of identities the guest ends up holding is the same.

---

## 3 — Tests

All four in `♾️infinite/🌍️world/🧪️tests/🔬️unit/🦀️.rs`, region `🥽️BrushMeshAnnounceTests`, driven
from the new fixture
`📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🧫️fixtures/🥽️brush-mesh-registration.json`
(React's measured per-step, per-window row counts from run-14 plus the two arg shapes and the digest
contract).

| test | law |
| --- | --- |
| `a_resident_mesh_announces_one_register_brush_mesh_per_window_with_reacts_args` | one row per window per revision, React's arg keys (`surfaceId`/`windowId`/`url`/`digest`/`pageCount`), and a sibling window announces by id alone with NO payload; no duplicate in the same frame or the next |
| `a_brush_mesh_run_pages_positions_then_indices_under_the_announced_digest` | the run's pages reassemble byte-for-byte into the resident mesh's positions-then-indices payload, and every page names the digest the whole run hashes to |
| `a_guest_restart_or_a_reupload_request_makes_every_surface_announce_again` | a climbing `meshResidency` is not news; a standing `meshReuploadUrls` entry takes the PAGE path; a residency below the high-water mark voids the claim and pages again |
| `every_react_measured_trigger_announces_once_per_window_instance` | all seven measured family-B steps, both window instances, from the fixture |

---

## 4 — Verification (all RUN, 2026-09-18)

| gate | result | log |
| --- | --- | --- |
| `cargo check -p semio-framework-os-infinite --lib -j 4` | **0 errors** | `🗑️generated/w12b-infinite-check.txt` |
| `cargo test -p semio-framework-os-infinite --lib world:: -j 4 -- --test-threads=1` | **178 passed, 0 failed** (the four new laws among them) | `🗑️generated/w12b-infinite-world-tests.txt` |
| `cargo check -p semio-framework-os-renderer-wgpu --lib -j 4` | **0 errors** | `🗑️generated/w12b-renderer-check.txt` |
| `cargo check -p semio-framework-os-renderer-wgpu --lib --target wasm32-unknown-unknown -j 4` | **0 errors** | `🗑️generated/w12b-wasm-check.txt` |

Live confirmation — whether the guest's brush utility now holds collision geometry on wgpu — is
**W12c's** next parity probe. This packet did not run `activate-*`, trunk or the wgpu wasm task.

## 5 — Two peer breakages crossed on the way, both since cleared

Neither was in this packet's diff; both are recorded because the gate logs above were re-run after
they lifted.

1. `♾️infinite/🌍️world/🧪️tests/📜️journal-sequences/🦀️.rs:253` failed `E0503` (`state.bounds.x`/`.w`
   read while the same call borrowed `&mut state`), which blocked the whole `--lib` test target.
   Untracked file, a live peer's lane (W10a/W12c); cleared by its owner.
2. `🌐️browser-worker/🦀️.rs:645` failed `E0063`: `AppInteractionState.pointer_capture`, a field W12a
   added in `🧊️renderer/🦀️.rs:11418`, was not initialized in the wasm-only constructor. Cleared by
   its owner; the wasm32 gate is green above.
