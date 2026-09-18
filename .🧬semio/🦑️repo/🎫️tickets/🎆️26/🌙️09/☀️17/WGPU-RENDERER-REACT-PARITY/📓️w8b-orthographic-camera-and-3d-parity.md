# 🎥️ W8b — the orthographic camera, the adaptive far plane, and React's 3D material

Packet W8b of ticket `26/09/17/WGPU-RENDERER-REACT-PARITY`. Every number below comes from a probe or
a test named inline; every command in §6 was RUN. Line numbers are post-edit.

**Result: both puzzle 3d panes now read like React's.** The `Top` pane is a real plan view — an
orthographic camera on React's pixel frustum, framed on the scene's content, with the reference sheet
and a grid (`🗑️generated/w8b-boot-2/final.png`, left half). The `Perspective` pane's model is lit by
React's own five-light R3F rig instead of one directional term
(`🗑️generated/w8b-boot-2/final.png`, right half). The orbit camera's far plane now follows the orbit
distance like React's, and every `world::tests::pick_*` stays GREEN — the blocker `📓️w7b` §9.1 handed
over is cleared.

| | before (`🗑️generated/w7b-final/run-1/final.png`) | after (`🗑️generated/w8b-boot-2/final.png`) |
| --- | --- | --- |
| `Top` pane | a perspective slice from the plan camera's eye, `up` degenerate | the whole sheet, plan view, grid |
| `Perspective` pane | flat tint × one directional term + a 0.28 ambient floor | React's ambient + hemisphere + 3 directionals, Lambert/π |
| grid | four stacked bands (100 · 25 · 5 · 1 world units) | React's ONE band at `lodGridStepWorld` |
| far plane | fixed 1 000 | `adaptiveOrbitCameraFar` (≥ 524 288), picks intact |

---

## 1 — Item 1: orthographic projection

### 1.1 — Root cause

`Camera3d` was perspective-only: `mat4_perspective_m` was its single projection and
`OrbitController::to_camera` hard-coded `up = [0, 0, 1]`. The `Top` pane's wire camera is
`{"position":[3.5,0,9.455],"target":[3.5,0,0.005],"up":[0,1,0],"zoom":1.0,"projection":{"mode":{"kind":"orthographic"},"orientation":{"type":"cardinal","view":"top"}}}`
(`🗑️generated/w7b-diag/dumps.json`, unchanged in `🗑️generated/w8b-diag/dumps.json`), so **three
things** were wrong at once, and each on its own would have ruined the pane:

1. the `projection` and `zoom` members were not even carried across the snapshot wire — the camera
   page was 10 numbers, `[position, target, up, fov]`;
2. `up = [0,1,0]` was dropped for Z-up, and a plan view whose up is `+Z` makes `look_at`'s cross
   product zero — a `NaN` view matrix;
3. even with both, there was no parallel projection matrix to build.

### 1.2 — Fix

| site | change |
| --- | --- |
| `🎬️scene/📐️math/🦀️.rs:216` | new `CameraProjection3d { Perspective, Orthographic }` with `from_mode_kind` — React's `worldProjectionFamily` (`orthographic \| axonometric \| oblique` are parallel) |
| `🎬️scene/📐️math/🦀️.rs:174` | new `mat4_orthographic_m` in the same `z ∈ [0,1]` clip convention `mat4_perspective_m` emits, so both families share one depth pipeline |
| `🎬️scene/📐️math/🦀️.rs:239` | `Camera3d` gains `projection` and `zoom`; `Default` moves to React's `near = 0.2` / adaptive far |
| `🎬️scene/📐️math/🦀️.rs:276` | new `projection_matrix(width, height)` — parallel takes drei's PIXEL frustum (`left = -width/2 / zoom`, the shape `worldProjectionGoalMatrix` builds), perspective divides `tan(fov/2)` by `zoom` exactly as three's `PerspectiveCamera` does |
| `🎬️scene/📐️math/🦀️.rs:294` | `view_proj(aspect)` → `view_proj(width, height)`: an aspect ratio cannot size a pixel frustum. 55 call sites across 7 files moved with it |
| `🎬️scene/📐️math/🦀️.rs:327` | new `Camera3d::stable_up` — the requested up, or the world twin that is not parallel to the view axis. This is defect 2 |
| `🎬️scene/📐️math/🦀️.rs:343` | `OrbitController` gains `projection`, `zoom` and `up`; `from_camera`/`to_camera` carry all three, and `from_camera` clamps its `asin` argument so a pole-on pose cannot answer `NaN` |
| `🎬️scene/📐️math/🦀️.rs:410` | `OrbitController::zoom` scales the parallel `zoom` and dollies only a perspective orbit — three's `OrbitControls` split, which is why `captureNavigationSnapshot` reports a live `camera.zoom` for an ortho pane and a bare `1` otherwise |
| `🎬️scene/📐️math/🦀️.rs:399` | `pan` uses `1 / zoom` world units per pixel under the parallel frustum (three's `panLeft/panUp` ortho branch) |
| `🎬️scene/📐️math/🦀️.rs:442`, `:456`, `:472` | new `world_projection_ortho_zoom` (React's `worldProjectionOrthoZoom`), `frame_orbit_to_bounds` now takes `width`/`height` and branches on the family, and new `screen_half_extent` projects the box's 8 corners onto the camera's own screen axes — the generalisation of React's cardinal-only `worldProjectionViewHalfExtent` |
| `🌍️world/🦀️.rs:10052`, `:10068` | `World3dSceneCameraRecord` gains `zoom` and the composed `projection` taxonomy object, with `projection()`/`fov_degrees()` mirroring `parseWorldProjectionField` + `worldProjectionPerspectiveFov` |
| `🌍️world/🦀️.rs` camera page | the snapshot camera item is now 12 numbers — `…, fov, zoom, parallel` — and its apply arm requires `number_len >= 12` and rebuilds the orbit with the family and zoom |
| `🌍️world/🦀️.rs:9413` | `world3d_camera_zoom` replaces the `WORLD3D_PERSPECTIVE_CAMERA_ZOOM` constant: `setCamera` now reports the LIVE parallel zoom (React's `camera instanceof ThreeOrthographicCamera ? camera.zoom : 1`), and `up` comes from the live orbit instead of the former `[0,0,1]` constant |

### 1.3 — The framing, and one deliberate deviation

With the projection alone the `Top` pane showed a correct plan view of 478 × 814 WORLD units — the
sheet was a 50-pixel speck (`🗑️generated/w8b-boot/final.png`). React does not leave it at the wire's
`zoom: 1` either: `WorldProjectionContentFrame` is mounted at boot for any pane whose camera carries a
projection spec (`fitProjectionContent = !viewportOwned && !cameraNavigating && !lockContentFrame &&
hasProjectionSeed`, all four true on a fresh boot) and frames `worldSceneContentBounds` — every
non-provisional instance position plus every VISIBLE reference plane's footprint.

| site | change |
| --- | --- |
| `🌍️world/🦀️.rs:10114` | new `sync_world3d_projection_content_frame`, called from `sync_world3d_state` right after the producer's fit lane |
| `🌍️world/🦀️.rs:10132` | new `world3d_content_bounds` — React's `worldSceneContentBounds`, half-unit floor and all — plus a bounds key so a tool that grows the scene re-frames and nothing else does |
| `🌍️world/🦀️.rs` state | `projection_frame_key` + `projection_frame_zoom`: the latch has to notice a staged wire camera landing after it and replacing the whole orbit, or the framing silently reverts to `zoom = 1` |

**The deviation, stated plainly.** React mounts that component for BOTH panes, but
`frameWorldProjectionPose`'s perspective branch would dolly the eye to `max(span · 1.5, 2)` — with a
≈285-unit reference sheet that is ≈577 units back, and the React reference image plainly does not do
that to the `Perspective` pane (`🗑️generated/react-6313/final.png`). So this frames the PARALLEL
family only and leaves the perspective pane to its delivered pose plus the producer's fit lane, which
is what both renderers now show.

### 1.4 — The `Projection` chip actually switches the projection

W6a wired the chip's fold and its rows as view state; the selection reached no camera.

| site | change |
| --- | --- |
| `🐚️Shell/…/🦀️.rs:13285` | `WorldProjectionTemplate` gains `family` — React's `worldProjectionFamily` over the taxonomy: the whole `Parallel` subtree is orthographic, the whole `Perspective` subtree is not |
| `🐚️Shell/…/🦀️.rs:9687` | the `shell.projection.template.…` hit arm now moves the pane's orbit onto that family and queues the SAME zero-delta camera intent a wheel settle queues, so the new pose publishes through the bounded interaction lane. React's switch dispatches no `setProjection` either — it remounts the camera and lets the ordinary camera report carry the pose |
| `🌍️world/🦀️.rs:9427` | new `apply_world3d_projection` — the mode-only transition keeps the eye and moves only the zoom: into the parallel family through `worldProjectionMatchedOrthoZoom` (so the apparent scale does not jump) and back out to the perspective identity, which is `worldProjectionTransitionPose`'s `orientationUnchanged` arm |
| `🐚️Shell/…/🦀️.rs` `world_projection_template_id` | a pane with no explicit press now reads its LIVE camera family instead of the static default, which is React's `cameraState.projectionSpec ?? (…"orthographic" ? … : …"threePoint")` fallback. Visible in `🗑️generated/w8b-boot-2/final.png`: the `Top` pane's chip wears the orthographic icon and the `Perspective` pane's the three-point one |

---

## 2 — Item 2: the adaptive far plane and the three.js pick ray

### 2.1 — Root cause

`WORLD_ORBIT_CAMERA_FAR = 1_000` was pinned there by `📓️w7b` §5.1, and its own docstring named the
blocker: `Camera3d::ray_from_screen` built its direction as `unproject(ndc, z = 1) - unproject(ndc, z
= 0)`. At React's `near = 0.2` / `far ≥ 524 288` the inverse view-projection's near-plane entries are
around `1e-6`, that difference is numerically empty in `f32`, and every pick missed.

### 2.2 — Fix

| site | change |
| --- | --- |
| `🎬️scene/📐️math/🦀️.rs:308` | `ray_from_screen(x, y, width, height)` is now three's `Raycaster::setFromCamera`: a perspective ray starts at the camera POSITION with `unproject(ndc, 0.5) - origin`; a parallel ray starts on the near plane at that pixel and runs down the view axis. The `aspect` parameter is gone — the viewport it already took is enough |
| `🎬️scene/📐️math/🦀️.rs:1963` | new `adaptive_orbit_camera_far` — React's `adaptiveOrbitCameraFar`, floor `WORLD_ORBIT_CAMERA_MIN_FAR = 524 288`, `distance × 1 024`, quantized to a power of two |
| `🎬️scene/📐️math/🦀️.rs:376` | `to_camera` takes `near = WORLD_ORBIT_CAMERA_NEAR (0.2)` and that adaptive far. `WORLD_ORBIT_CAMERA_FAR` is deleted |

All twelve `world::tests::pick_*` / `pick_component_*` / `world_ray_pick_*` tests are green under the
new far plane (`🗑️generated/w8b-tests-world.txt`), which is the proof `📓️w7b` §9.1 asked for.

---

## 3 — Item 3: lighting, material and the grid

### 3.1 — The light rig

`WORLD3D_SHADER`'s fragment was `max(dot(n, light_dir), 0.28)` times a flat instance tint. React's
R3F scene (`🌐️World3dHost/🟦️.tsx:7377`, the no-sun branch puzzle 3d takes) is
`<ambientLight intensity={1.15}>`, `<hemisphereLight color="#ffffff" groundColor="#9aa0ab"
intensity={1.35} position={[0,0,1]}>` and three `<directionalLight>`s at `[12,18,10] × 2.4`,
`[-14,-10,6] × 1.2`, `[0,0,-16] × 0.75`, over a `MeshStandardMaterial` with `metalness 0, roughness 1`.

| site | change |
| --- | --- |
| `🎨️shaders/🦀️.rs:241` | new `world3d_irradiance(n)` — that exact rig, with the hemisphere's ground colour in LINEAR space (`#9aa0ab → 0.32313, 0.35153, 0.40724`) because the world pass renders into an sRGB view |
| `🎨️shaders/🦀️.rs:260` | the fragment is now `irradiance · albedo / π + albedo · emissive`, three's `BRDF_Lambert` with `useLegacyLights = false` plus `totalEmissiveRadiance` |
| `🎨️shaders/🦀️.rs` flags | the selected/hovered arms no longer mix a hard-coded blue and amber; they answer React's `MESH_STYLE_PAINT` emissive intensities (`0.35` selected, `0.08` hovered) |
| `🌍️world/🦀️.rs:6959` | new `world3d_style_paint` — React BAKES the style's fill into the material rather than tinting the neutral one, so the host resolves `tokenVar("primary")` (`theme.celebrate[0]`) and `semanticVar("hover-interactive-fill")` (`theme.row_hover`) onto the instance colour. The shader knows no theme; this half cannot live there |
| `🌍️world/🦀️.rs:12611` | new `srgb_to_linear`, and `parse_color` uses it. It used to hand raw sRGB bytes to a pipeline that encodes them a SECOND time on write, so every wire colour was washed out against React, whose `new Color(hex)` linearizes through `ColorManagement`. `Theme` was always linear; only this reader was not |
| `✨️shader-contract/🦀️.rs:592`, `🪟️d3d12/✨️hlsl/🦀️.rs:266`, `🍎️metal/✨️msl/🦀️.rs:270` | the same rig transcribed into the other three backends |

### 3.2 — The grid

`append_lod_grid_lines` drew `lod_progressive_grid_layers` — four stacked bands at fixed quanta,
which at the puzzle 3d boot camera is 100 · 25 · 5 · 1 world units where React paints a single 10-unit
grid. React has no counterpart for that idea at all; `WorldLodGridHelper` draws ONE band at
`lodGridStepWorld`.

| site | change |
| --- | --- |
| `🎬️scene/📐️math/🦀️.rs:2101` | `lod_grid_step_world` is now React's `lodGridStepWorld` — `gridFactor × quantum × magnitude` over the `1 · 2.5 · 5 · 10` ladder, instead of "the finest progressive band" |
| `🎬️scene/📐️math/🦀️.rs:1991`, `:2007` | new `camera_grid_visible_radius` (React's `cameraGridVisibleRadius`, parallel branch and all) and `camera_grid_fade_distance` rebuilt on React's viewport branch. W7b's HARD quarter-far-plane cap is gone — it only existed because the far plane could not grow, and it now can |
| `🎬️scene/📐️math/🦀️.rs:1982` | new `lod_orbit_distance_for_camera` — React's `lodOrbitDistanceForCamera`, so a parallel pane bands its LOD on `worldProjectionMatchedPerspectiveDistance(zoom)` and scrolling retunes the grid |
| `🌍️world/🦀️.rs:6919` | `append_lod_grid_lines` draws that ONE band, sized by the camera's own fade radius |
| `🌍️world/🦀️.rs:6881` | `scene_lod` reads the family-aware distance |

**One deviation, documented in the function.** React's grid is an infinite shader plane; this is real
line geometry, so a radius past `WORLD_GRID_MAX_DIVISIONS` is CLAMPED (and the fade curve clamped with
it, so the rim still reaches alpha 0) rather than dropping the band and leaving the pane gridless —
which is what W7b's version did to a plan pane.

### 3.3 — Not changed, and why

The reference underlay's **paint order** stays `textured → opaque → lines → translucent` (W7b §4).
React's order is plane `-10`, grid `-5`, model `0`, i.e. lines before opaque. The visible result is
identical either way: the grid does not write depth and sits 0.002 below the model, so it is occluded
by the model whichever side of it the pass draws, and re-ordering means moving the prepared measure
cursor and its pinned fixture for no pixel. Left as a named difference rather than churn.

---

## 4 — Tests added or rewritten (all RUN)

| test | file | what it pins |
| --- | --- | --- |
| `the_parallel_frustum_is_dreis_pixel_frustum_and_ignores_the_eyes_distance` | `🎬️scene/🧪️tests/🔬️math-unit/🦀️.rs` | zoom 1 shows exactly `width` world units, the half-extent lands on the rim, moving the eye along the view axis changes nothing, 4× zoom shows a quarter |
| `a_parallel_pick_ray_round_trips_through_the_pixel_it_came_from` | same | three parallel rays run down the view axis and project back to their own pixel |
| `a_perspective_pick_ray_survives_the_adaptive_far_plane` | same | at `far = 524 288` the ray starts at the eye, carries a unit direction, and round-trips — the §2.1 defect |
| `a_parallel_fit_moves_the_zoom_and_a_perspective_fit_moves_the_distance` | same | both families centre on the box; only the perspective one dollies; the parallel zoom is React's over the box's SCREEN half-extent and the box fits the frustum it chose |
| `the_wheel_dollies_a_perspective_orbit_and_zooms_a_parallel_one` | same | the family split, and that the parallel frustum never inverts |
| `the_puzzle3d_pane_cameras_survive_the_orbit_round_trip` | same | **the packet's fixture**: both panes' delivered cameras, verbatim off the wire, survive `from_camera`/`to_camera` with family, `up` and zoom intact, and the plan view-projection is finite, not `NaN` |
| `the_matched_zoom_and_distance_mappings_invert_each_other` | same | `worldProjectionMatchedOrthoZoom` ↔ `…MatchedPerspectiveDistance`, and the LOD distance each family bands on |
| `the_grid_fade_radius_follows_the_camera_and_stays_inside_the_far_plane` | same (rewritten) | the adaptive far plane's floor AND its quantized growth, plus the fade radius laws |
| `the_grid_visible_radius_reads_the_cameras_own_family` | same | parallel reads its zoomed pixel extent, perspective its height above the plane |
| `lod_grid_step_world_quantizes_like_reacts_helper` | same (rewritten) | the `1 · 2.5 · 5 · 10` ladder across two decades |
| `lod_grid_lines_keep_reacts_band_spacing_and_fade_inside_the_far_plane` | `🌍️world/🧪️tests/🔬️unit/🦀️.rs` (rewritten) | every row lands on React's band multiple, the grid sits inside `far/4`, the rim is alpha 0, and a PLAN pane gets its own grid inside the division ceiling |
| `world3d_lighting_constants_are_identical_in_every_backend_mirror` | `🖌️render/🧪️tests/🔬️shader-contract-unit/🦀️.rs` | **the twin law across all four backends**: every rig value present in the WGSL, the HLSL and the MSL, and the old `0.28` floor gone from all three |
| `the_projection_chip_folds_its_own_pane_and_switches_its_template` | `🐚️Shell/🧪️tests/🧭️wgpu-navbar-footer-parity/🦀️.rs` (extended) | the chip press now moves the pane's CAMERA family and queues the settle that publishes it, not only an icon |

---

## 5 — Probe evidence

| output | what it shows |
| --- | --- |
| `🗑️generated/w8b-boot/final.png` | the projection alone: the `Top` pane is a genuine plan view with a grid, but at the wire's `zoom = 1` the sheet is a 50-pixel speck — §1.3's starting point |
| `🗑️generated/w8b-boot-2/final.png` | **the packet's result**: the whole sheet framed in the `Top` pane, React's lighting on the model in the `Perspective` pane, one grid per pane, per-pane projection chip icons |
| `🗑️generated/w8b-diag/` | 45 s diagnostics boot on the final build: `data-semio-os-error = null`, **136 `fault=None` census lines and ZERO `fault=Some` / `preparation refused` / page errors**, both surfaces' wire cameras unchanged |
| `🗑️generated/w8b-chrome/chrome-dump.json` | 46 published chrome hits including `shell.projection.fold.puzzle3d-main-top` and `…-perspective` |
| `🗑️generated/w8b-projection/` | the `Projection` chip press located by control id and clicked live (`rect [1357.4, 842.4, 76.2, 22.4]`); see §7.2 |
| `🐍️w8b-projection-switch-probe.mjs` | new probe: finds a pane's projection fold + template rows by control id in `dumpChrome`, clicks them and reads the cameras back. Needs `SEMIO_RUNTIME_DIAGNOSTICS` armed or `dumpChrome` answers an empty ledger |

---

## 6 — VERIFY (all foreground, all RUN)

| gate | result |
| --- | --- |
| `cargo check -p semio-framework-ui --features wgpu-engine --lib -j 4` | **0 errors** (`🗑️generated/w8b-check-ui.txt`) |
| `cargo check -p semio-framework-os-infinite --lib -j 4` | **0 errors** (`w8b-check-infinite.txt`) |
| `cargo check -p semio-framework-os-renderer-wgpu --lib -j 4` | **0 errors** (`w8b-check-renderer.txt`) |
| `cargo check … --target wasm32-unknown-unknown` (both crates) | **0 errors** (`w8b-check-ui-wasm.txt`, `w8b-check-renderer-wasm.txt`) |
| `cargo test -p semio-framework-ui-scene --lib -- --test-threads=1 math::` | **88 passed / 0 failed** (`w8b-tests-scene.txt`) |
| `cargo test -p semio-framework-ui-render --lib -- --test-threads=1` | **132 passed / 0 failed** (`w8b-tests-render.txt`) — includes the new four-backend twin law |
| `cargo test -p semio-framework-ui --features wgpu-engine --lib -- --test-threads=1 wgpu::gpu wgpu::prepared wgpu::draw` | **90 passed / 0 failed** (`w8b-tests-ui.txt`) |
| `cargo test -p semio-framework-os-infinite --lib -- --test-threads=1 world::` | 150 passed / **7 failed** (`w8b-tests-world.txt`) — the SAME seven `📓️w7b` §7 records as pre-existing (`live_renderer_retains_…`, `prepared_world_resources_…`, `world_authority_retains_…`, two `world_component_marquee_…`, `world_object_registry_…`, `world_saturation_owner_…`). All twelve `pick_*` are green |
| `cargo test -p semio-framework-os-renderer-wgpu --lib -- --test-threads=1` | 858 passed / **7 failed** (`w8b-tests-renderer.txt`). Two are the `async_boundary_tests` pair `📓️w3c`/`📓️w3d`/`📓️w5c`/`📓️w7b` all record as pre-existing. The other five — `engine_canvas::saturated_graph_and_board_wheel_queues_preserve_cameras`, `shell::chrome_overlays_tour_tests::window_silhouette_border_…`, `shell::command_registry_tests::directory_home_bootstrap_…`, two `shell::tool_run_panel_tests::…` — are peer lanes: none of their sources mention `Camera3d`, `projection`, `view_proj`, `parse_color` or `WORLD3D_SHADER`, and their assertions are a board host's `defers_descriptor_sync_from_js`, a chrome silhouette outline, a bootstrap cursor and a tool-run focus order |
| `…framework-renderer-wgpu:wasm --skip-nx-cache` then `…:activate-puzzle3d-wgpu-dev` | **exit 0** twice (`w8b-wasm-2.txt`, `w8b-activate-2.txt`) |

Two fixtures moved with the code and are part of the change:
`🔨️modules/⏳️async/🧫️fixtures/🧱️boxed-fixed-slots/🔣️.json` (the `AdmittedSurfaceMap<World3dState>`
slot grew 96 bytes when the orbit gained its family, zoom and up) and the world unit fixture's camera
page, now 12 numbers.

The first `wasm` attempt failed for ~1 minute on a peer's live taxonomy edit
(`semanticDirectoryMemberKinds["members-of-examples"] has invalid exact member "⬡️hex-ring"`, ticket
26/09/18 EXTRACT-WFC-PLUGIN); polling through it is the whole remedy. `df -g /` reported 41 GB free.

The 6213 serve was not running when this packet started; it was restarted with
`S_OS_PORT=6213 nx run @semio-tech/framework-os-dev:serve-puzzle3d-wgpu-dev` (the target's default
6113 is held by a peer) and left running — `WGPU browser ready: http://127.0.0.1:6213/?plugin=puzzle3d`,
log `🗑️generated/w8b-serve-6213.txt`.

---

## 7 — Remaining differences vs React

### 7.1 — P2: the `Top` pane's framing is slightly wider than React's

React's sheet spans ≈285 px of the 478-px pane; this one ≈350 px
(`🗑️generated/react-6313/final.png` vs `🗑️generated/w8b-boot-2/final.png`). Both frame the same
content with the same 1.35 padding, so the gap is in how the extent is composed — React's
`worldProjectionViewHalfExtent` takes the cardinal `(hx, hy)` of the bounds, this takes the box's
projected screen half-extent, and the two differ when the reference footprint is not square. One
measurement of `references[].widthWorld` against the framed zoom settles it.

### 7.2 — P1 (W6a's lane): the unfolded projection pane paints no body live

`shell.projection.fold.<windowId>` is published and the press lands — the chip's fold state flips —
but after it, no `shell.projection.template.<windowId>::<id>` row appears in `dumpChrome` within 14 s
of retries, and no body is painted (`🗑️generated/w8b-projection/unfolded.png`). `paint_window_projection_step`
IS called from the live chrome walk (`🐚️Shell/…/🦀️.rs:17477`), and the unit test drives it to
completion in 8 192 opportunities, so the suspect is the walk's per-frame budget against
`world_projection_column_width`'s 15 measured labels. The camera half of the switch is proven at the
state level by `the_projection_chip_folds_its_own_pane_and_switches_its_template`; the live half is
blocked on this. Belongs to the packet that owns the projection pane's paint.

### 7.3 — P2: the grid is line geometry, React's is a shader

Unchanged from `📓️w7b` §9.5, and now with one named consequence: §3.2's division ceiling clamps a
plan pane's grid radius where React's infinite plane would keep going.

### 7.4 — P2: `celebrated`, `provisional` and `disabled` material styles

`world3d_style_paint` covers React's `selected` and `hovered` rows of `MESH_STYLE_PAINT`. The
`celebrated` conic ShaderMaterial, `provisional`'s own fill/opacity (which the world lane already
applies separately at its `provisional_draws` branch) and `disabled`'s 0.45 opacity are not folded
into one table yet. No boot pixel depends on them.

### 7.5 — tracked elsewhere

`dumpMeshStats` still reports the WIRE camera rather than the live orbit, so a local projection switch
is invisible to it (`📓️w7b` §9.6's diagnostics defect, same family). The boot tour (W4a/W7a) and the
light/dark boot input (W5a) are unchanged here.
