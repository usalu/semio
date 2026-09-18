# 🖼️ W7b — the shell that presented one frame per boot

Packet W7b of ticket `26/09/17/WGPU-RENDERER-REACT-PARITY`, the P0 handed over by
`📓️w5c-world-pass-mesh-draws.md` §5.1. Every number below comes from a probe output named inline;
every command in §7 was RUN. Line numbers are post-edit.

**Result: the wgpu shell presents frames continuously again, and the concrete-forest model, the plan
underlay and the world grid all paint** (`🗑️generated/w7b-final/run-1/final.png`). The canvas hash now
moves across the whole asset stream and settles only when the scene does:

```
before (w7b-probe-1)  6s:bc54791ef686 10s:a5e5be8ac997 14s:a5e5be8ac997 20s:a5e5be8ac997 30s:… 40s:…   ← frozen from t=10 s
after  (w7b-final r1) 6s:bc54791ef686 10s:331b58d35880 14s:18f8c879618e 20s:80be26ed8dde 30s:… 40s:…
after  (w7b-final r2) 6s:bc54791ef686 10s:331b58d35880 14s:335d03011cbe 20s:80be26ed8dde 30s:… 40s:…
```

Five defects were found; the first is the P0 and the second is why it survived a whole packet
undiagnosed.

---

## 1 — Root cause: an unbound raster producer killed every frame build after the underlay decoded

### 1.1 — The chain, measured

Four temporary `[DEBUG]` points (W5c §5.1, re-added one build at a time, all removed again — §8)
named it in three builds:

| probe | line |
| --- | --- |
| `🗑️generated/w7b-probe-1` | `w7b terminal requested=Generation(7) frame=Generation(7) completed=false terminal=none cancel-site=job.begin_close advances=17030 outcome=Cancelled` |
| `🗑️generated/w7b-probe-2` | same, with the cancel site split: `cancel-site=prepare.outcome`, `phase=prepare` |
| `🗑️generated/w7b-probe-4` | **46 × `w7b prepare fault site=render-job-outcome fault=Some("raster producer generation is stale")` in 25 s** |

So the build was never cancelled from outside at all. It cancelled **itself**, in
`ActiveFramePhase::Prepare`, because its own preparation refused — and the refusal came from the
prepared render job.

`World3dBuildContext::append_step` (`♾️infinite/🌍️world/🦀️.rs:291`) moved the world reference
underlay's `PreparedRasterProducer` into the frame's `PreparedRenderInput` **without ever binding it
to that frame**:

```rust
let Some(producer) = self.raster_producers[index].take() else { return Ok(false) };
…
if let Err(producer) = input.try_push_raster_producer(producer) { … }   // ← no bind_frame_generation
```

The chrome lane's own raster phase binds (`🧊️renderer/🦀️.rs:12637`,
`producer.bind_frame_generation(input.preview_generation)`); the world lane, the second and only
other producer source in the repo, never did. An unbound producer carries `frame_generation: None`,
and `PreparedRasterProducer::step(expected)` (`🎟️prepared/🦀️.rs:1095`) answers
`Fault("raster producer generation is stale")` for it. From there:

```
PreparedRasterProducer::step        → Fault("raster producer generation is stale")
PreparedRenderJob::step             → StepOutcome::Fault(JobFault{ detail: EMPTY payload })
AppFramePreparation::drive_step     → StepOutcome::Fault(JobFault{ detail: EMPTY payload })   (fault string dropped)
ActiveFrameBuild::advance (Prepare) → self.cancel.cancel_now()                                (no fault recorded)
drive_step (next opportunity)       → StepOutcome::Cancelled                                  (job never entered)
FrameBuildHandle Terminal arm       → completed = None → no presentation
```

**Why it looked like "one frame per boot".** The producer exists only once the reference image has
been fetched and decoded — around t≈10 s on this boot. Generation 3 completed and presented BEFORE
that (`w7b-probe-1`: `completed=true terminal=prepared`); generations 4–6 died of legitimate
`interaction` supersessions while the preferences load held the state; and **every build from
generation 7 to the end of the session died of this one line.** The asset the shell was waiting for
was the thing that stopped it painting.

### 1.2 — The fix

| site | change |
| --- | --- |
| `♾️infinite/🌍️world/🦀️.rs:291` | `append_step` binds the producer to `input.preview_generation` before pushing it, and treats a refused bind as the rejection it is. The docstring carries the fault chain above |

## 2 — Root cause 2: the refusal could not be named by anyone

`StepOutcome::Fault` carries a `RetainedJobPayload`, and every producer of one on this path builds it
`empty()` — a fixed `&'static str` needs no page. So the reason existed on the job and was thrown
away by its driver, twice. That is why W5c measured `outcome=Cancelled` with "no supersession, no
fault, no overrun quarantine" and a completely silent console: **there was nothing to read.**

| site | change |
| --- | --- |
| `🎟️prepared/🦀️.rs:2326` | new `PreparedRenderJob::fault() -> Option<&'static str>` — the string `fault_outcome` already recorded, now readable |
| `🧊️renderer/🦀️.rs:12833` | `AppFramePreparation` gains a `fault` slot; all five refusal arms name themselves (`prepared render job admission was refused`, `… session admission was refused`, `… lost its session`, `… was cancelled`, and the job's own fault) |
| `🧊️renderer/🦀️.rs:12852` | new `AppFramePreparation::fault()` |
| `🧵️frame-job/🦀️.rs:384` | the Prepare phase's `Cancelled | Fault` arm logs `os_host frame preparation refused: <fault>` once per transition before it cancels. This is a permanent diagnostic, not a `[DEBUG]` line |

A frame fault is deliberately NOT recorded: `record_frame_fault` is what the browser worker turns
into a surface quarantine, and a refused preparation must end one build, not the surface.

## 3 — The asset decode pump now takes a share of the step, not the whole one

W5c §5.2. `FrameTransaction::step` answered `Pending` before it did anything else while
`pump_renderer_asset_decode_step()` was true, and W3d's `while !deadline_exceeded && pump {}` spent
the entire wall slice there — measured as `asset-decode-pump units=306 deadline=true`, over and over,
still firing at t=15.3 s, with no transaction phase reached in between.

| site | change |
| --- | --- |
| `🧊️renderer/🦀️.rs:11946` | new `RENDERER_ASSET_DECODE_SLICE_US` = half of `INTERACTIVE_LANE_WALL_US`, and `renderer_asset_decode_slice_deadline_us(now)` — the pure bound, so the law is testable without a clock |
| `🧊️renderer/🦀️.rs:12058` | the decode loop is bounded by that share **and no longer returns**: the transaction's own phases get the rest of the step, so the shell keeps building and presenting while a GLB streams. React has no equivalent tax at all — its loader is not on the render path — so a share is the closest honest mirror |

## 4 — The world underlay is painted FIRST

W5c §8.4. React gives the reference plane `renderOrder = -10`, the grid `-5` and the model `0`, so
the plane is UNDER both. The prepared measure cursor ordered a scene pass opaque → lines →
translucent → textured, so the wgpu underlay composited last and tinted everything it covered.

| site | change |
| --- | --- |
| `🎟️prepared/🦀️.rs:2549` | `PassHeader` now enters `PassTextured` first |
| `🎟️prepared/🦀️.rs:2696` | new `next_after_textured` → opaque; `next_after_translucent` → the next pass header; `next_textured_draw` closes into `next_after_textured` |

The pass is a non-depth-writing `LessEqual` alpha blend either way (W5c §2), which is exactly what
makes drawing it first correct: the geometry in front of it occludes it.

## 5 — The grid is sized by the camera, not by a fixed square

W5c §5.4. `append_lod_grid_lines` submitted a FIXED 12 000-unit square for every LOD band and clamped
the divisions to 512 — so the drawn spacing silently stopped being the band's spacing (12 000 / 512 =
23.4 world units for a band whose step is 1) — against an orbit camera whose far plane is 1 000. Only
stray diagonals survived clipping.

| site | change |
| --- | --- |
| `🎬️scene/📐️math/🦀️.rs:1753` | new `camera_grid_fade_distance(camera_z, plane_z, step_world, far)` — the Rust twin of React's `cameraGridFadeDistance` on its viewport-free branch (`WORLD_LOD_GRID_FADE_HEIGHT_FACTOR = 32`, `WORLD_LOD_GRID_MIN_FADE_CELLS = 24`, power-of-two cell quantization), with ONE documented deviation: the quarter-far-plane cap is HARD here (§5.1) |
| `🎬️scene/📐️math/🦀️.rs:1775` | new `lod_grid_fade_alpha(radius, fade)` — React's drei `fadeStrength = 1.5` curve |
| `🎬️scene/📐️math/🦀️.rs:1747` | `WORLD_ORBIT_CAMERA_FAR` — the same 1 000 the orbit camera always used, now named and carrying §5.1 |
| `♾️infinite/🌍️world/🦀️.rs:6915` | `append_lod_grid_lines` keeps the band's EXACT spacing, sizes the square to that fade radius, DROPS a band whose divisions would exceed `WORLD_GRID_MAX_DIVISIONS` rather than distorting it, and splits every line at the centre so its rim vertices carry alpha 0 — a distance fade instead of a hard edge |

### 5.1 — Why the orbit camera's far plane is still 1 000, and what it is blocked on

React grows it every frame with `adaptiveOrbitCameraFar(dist)` (floor 524 288). Wiring the Rust twin
of that was implemented and **reverted**, because it silently breaks picking:
`world::tests::pick_hover_*` and `pick_select_*` (4 tests) go from green to red the moment the far
plane grows. `Camera3d::ray_from_screen` builds its direction as `unproject(ndc, z=1) -
unproject(ndc, z=0)`, and at a `near = 0.1` / `far = 524 288` ratio the inverse view-projection's
entries are around `1e-6`, so the near-plane unprojection is numerically empty in `f32`. three.js
does not have this problem because `Raycaster::setFromCamera` takes the camera POSITION as the origin
and ONE mid-depth unprojection as the second point. Rebuilding the ray that way is the prerequisite,
and it is a picking-lane change with its own fixture set — handed off in §9.1. The grid does not need
it: it sizes itself to a quarter of whatever far plane it is given.

## 6 — Tests added (all RUN)

| test | file | what it pins |
| --- | --- | --- |
| `an_appended_world_raster_producer_is_bound_to_the_frames_generation` | `♾️infinite/🌍️world/🧪️tests/🔬️unit/🦀️.rs:1513` | the P0: the producer this lane appends answers its own frame generation and refuses any other, and a real `PreparedRenderJob` driven over it yields with `fault() == None` |
| `an_unbound_raster_producer_refuses_its_prepared_job_with_a_readable_fault` | `🖱️ui/🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs:322` | the other half, from the ui side: an unbound producer faults with that exact string and the string is READABLE through `PreparedRenderJob::fault` |
| `a_refused_frame_preparation_names_its_fault_before_the_build_cancels` | `🧪️tests/🔬️wgpu-renderer-async-boundary/🦀️.rs:14` | every preparation refusal names itself and the build's cancel arm reads it — the visibility law of §2 |
| `the_renderer_asset_decode_lane_spends_a_share_of_the_step_and_never_the_whole_one` | same file | the share is half the interactive wall slice, saturates instead of wrapping, spends nothing without a clock, and the loop does not `return` before the transaction's phases |
| `a_scene_pass_measures_its_textured_underlay_before_everything_it_sits_under` | `🔬️targets-wgpu-prepared-unit/🦀️.rs` | the measured order is `["textured", "opaque", "lines", "translucent"]` |
| `the_grid_fade_radius_follows_the_camera_and_stays_inside_the_far_plane` | `🎬️scene/🧪️tests/🔬️math-unit/🦀️.rs:337` | the fade radius, its far-plane cap, its monotonicity in camera height, and the fade curve's endpoints |
| `lod_grid_lines_keep_their_band_spacing_and_fade_inside_the_far_plane` | `♾️infinite/🌍️world/🧪️tests/🔬️unit/🦀️.rs` (replaces `lod_grid_lines_generate_for_near_camera`) | every grid row lands on a band multiple (never on a clamped division), the whole grid sits inside `far/4` and nowhere near the old fixed square, and the rim carries alpha 0 |

**The build-completion law the brief asked for ("N admitted builds → N presented packets in a headless
run") is NOT among them, and deliberately.** A presented packet needs a real GPU surface; the headless
gates in this repo can only reach `AppFramePreparation`, which is exactly where the defect lived and
is what the first three tests now pin end to end. The live equivalent is the present-probe cadence
above, which is the cheapest possible verdict and is reproduced in §7.

## 7 — VERIFY (all foreground, all RUN)

| gate | result |
| --- | --- |
| `cargo check -p semio-framework-ui --features wgpu-engine --lib -j 4` | **0 errors** (`🗑️generated/w7b-check-ui.txt`) |
| `cargo check -p semio-framework-os-renderer-wgpu --lib -j 4` | **0 errors** (`w7b-check-renderer.txt`) |
| `cargo check -p semio-framework-ui --features wgpu-engine --lib --target wasm32-unknown-unknown -j 4` | **0 errors** (`w7b-check-ui-wasm.txt`) |
| `cargo check -p semio-framework-os-renderer-wgpu --lib --target wasm32-unknown-unknown -j 4` | **0 errors** (`w7b-check-renderer-wasm.txt`) |
| `cargo test -p semio-framework-ui --features wgpu-engine --lib -- --test-threads=1 wgpu::gpu wgpu::prepared wgpu::draw` | **90 passed / 0 failed** (`w7b-tests-ui.txt`) |
| `cargo test -p semio-framework-ui-scene --lib -- --test-threads=1 math::` | **80 passed / 0 failed** (`w7b-tests-scene.txt`) |
| `cargo test -p semio-framework-os-renderer-wgpu --lib -- --test-threads=1 async_boundary_tests` | 29 passed / **2 failed** — `presenter_ack_retirement_source_mutations_are_denied` and `raster_upload_cache_is_fixed_generation_witnessed_and_mutation_complete`, the exact two `📓️w3c` §5 / `📓️w3d` §5 / `📓️w5c` §6 all record as pre-existing peer-lane contracts (`w7b-tests-renderer.txt`) |
| `cargo test -p semio-framework-os-infinite --lib -- --test-threads=1 world::` | 150 passed / **7 failed** (`w7b-tests-world.txt`) — all seven pre-existing, and PROVEN so: with this packet's `append_step` change temporarily removed, `prepared_world_resources_are_send_and_deduplicate_uploads` fails identically (`uploads.len()` 1 vs the 2 it asserts), and the other six (`live_renderer_retains_…`, `world_authority_retains_…`, two `world_component_marquee_…`, `world_object_registry_…`, `world_saturation_owner_…`) fail with the far plane reverted too. The four `world::tests::pick_*` tests are GREEN (§5.1) |
| `…:framework-renderer-wgpu:wasm --skip-nx-cache` then `…:activate-puzzle3d-wgpu-dev` | **exit 0** (`w7b-wasm-final.txt`, `w7b-activate-final.txt`) |

Both nx targets failed for ~6 minutes mid-packet with `Failed to process project graph` /
`Component needs an authored deployment directory: ✏️s/🔌️plugins/🌊️wfc/…` — a peer creating that
plugin live (mtimes 12:24–12:26). Polling through it is the whole remedy; nothing in this lane.

### Probe evidence

| output | what it shows |
| --- | --- |
| `🗑️generated/w7b-probe-1` | the starting state reproduced: one presented frame (generation 3, `terminal=prepared`), every later build `outcome=Cancelled cancel-site=job.begin_close` |
| `🗑️generated/w7b-probe-2` | the cancel site split — `cancel-site=prepare.outcome phase=prepare`, so the build cancels ITSELF |
| `🗑️generated/w7b-probe-4` | the name: `fault=Some("raster producer generation is stale")`, 46 times in 25 s |
| `🗑️generated/w7b-fix-1` | the P0 fix alone: canvas hash moves `6s → 10s → 20s → 30s` for the first time |
| `🗑️generated/w7b-final/run-1`, `run-2` | the final build, two boots, byte-identical to each other: hashes move through the asset stream and settle; `lit=13361/13361` |
| `🗑️generated/w7b-final/run-1/final.png` | the model, the plan underlay UNDER it, and the grid |
| `🗑️generated/w7b-diag` | 45 s diagnostics boot on the final build: `draws=1 instances=1 lines=1 textured=1 passes=2`, **`fault=None` on all 160 census lines, zero page errors, zero `preparation refused`** |

## 8 — Instrumentation removed

`grep -rn "w7b\|debug_frame_build\|debug_frame_poll\|debug_frame_outcome\|debug_frame_deferred\|debug_asset_branch\|debug_stage" 🧰️framework`
returns only docstring references to this report. W5c's own `[DEBUG] w5c` helpers
(`debug_frame_build`, `debug_frame_poll`, `debug_frame_outcome`, `debug_frame_deferred`,
`debug_asset_branch`, `debug_stage`) were still in the tree and are gone with them.

## 9 — Remaining differences of the 3D view vs React, at 1440×900 dpr 1

Measured against `🗑️generated/react-6313/final.png`.

### 9.1 — P1: the orbit far plane and the pick ray

§5.1. React's camera far is ≥ 524 288 and follows the orbit distance; this one is a fixed 1 000, so a
scene an order of magnitude larger than the concrete forest will clip. The blocker is
`Camera3d::ray_from_screen`'s near-minus-far direction, which must become three's
`origin = camera.position, direction = unproject(ndc, z = 0.5) - origin` first. That is a picking-lane
change with `world::tests::pick_*`, the marquee fixtures and `project_point` as its gates.

### 9.2 — P1: the Top pane's camera is orthographic and the renderer has no orthographic projection

`dumpMeshStats` (`🗑️generated/w7b-diag/dumps.json`) reads
`"surfaceId":"puzzle3d-main-top" … "projection":{"mode":{"kind":"orthographic"},"orientation":{"type":"cardinal","view":"top"}},"zoom":1.0`,
while `Camera3d` is perspective-only (`mat4_perspective_m` is its single projection). The Top pane
therefore renders a perspective view from the ortho camera's eye and shows a narrow slice of the plan
where React shows the whole sheet. This is the single largest remaining visual gap and it is a
projection-math packet, not a presentation one.

### 9.3 — P2: the perspective framing

Both hosts are handed the same wire camera (`[9.17, -5.67, 4.26] → [3.5, 0, 0.005]`, fov 50°), and the
wgpu pane is visibly closer than React's. React's `fov` is applied to a camera whose aspect and
viewport come from its own canvas size; the wgpu pass viewport is `956 × 814` inside a `1440 × 900`
window. Worth one measurement of `mat4_perspective_m`'s aspect against React's before assuming a
camera difference.

### 9.4 — P2: lighting and material

Unchanged from W3d §8 / W5c §8.3: `WORLD3D_SHADER` is one directional term with a `0.28` ambient
floor and a flat instance tint; React's `GlbInstanceMesh` bakes colour, emissive, opacity, border and
the `celebrated` shader into the cloned GLB scene.

### 9.5 — P2: the grid is line geometry, React's is a shader

React's drei `Grid` is an infinite screen-space-derived shader plane that follows the camera and
never aliases. This is a real line grid centred on the orbit anchor. At the boot camera the two read
the same; at a grazing angle the line grid will alias where React's fades.

### 9.6 — tracked elsewhere

The boot tour (W4a/W7a) and the light/dark boot input (W5a) are their own lanes and unchanged here.
`dumpMeshStats` still reports `indices:0, positions:0` for every mesh while the model demonstrably
paints — the dump reads `World3dState`'s mesh table rather than the renderer's resident one, which is
a diagnostics defect, not a mesh one.
