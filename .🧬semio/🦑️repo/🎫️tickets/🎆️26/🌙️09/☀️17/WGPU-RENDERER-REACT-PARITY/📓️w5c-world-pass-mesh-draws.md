# 🧱️ W5c — the world pass's MESH and TEXTURED draws

Packet W5c of ticket `26/09/17/WGPU-RENDERER-REACT-PARITY`, the P0 handed over by
`📓️w3d-world3d-glb-url-lane.md` §7. Every number below comes from a probe output named inline; every
command in §6 was RUN. Line numbers are post-edit.

**Result: W3d's prime suspect is WRONG and is now ruled out by measurement — the depth state, the
camera, the W1f vertex layout and the instance colour are all innocent.** The World3d pass paints
only its LINE draws for two independent reasons, both found and both fixed at the root:

1. **The TEXTURED pass had no encoder at all.** `ScenePass3d::textured_draws` has been MEASURED into
   command pages since the scene pass existed (146 pages per frame on the live playground), but
   `encode_prepared_draw_scalar` had no arm for `DrawMeasureCursor::PassTextured*` and
   `WORLD3D_TEXTURED_PIPELINE` was declared in the shader contract as *"inferred — unwired in
   draw.rs"*. The reference underlay could not paint on ANY target, ever. §2.
2. **The MESH draw is never *submitted*, because the shell presents exactly ONE prepared frame per
   boot.** `encode_prepared_world_instance` is called ZERO times in a 45 s boot (§1.2). Everything
   that resolves asynchronously — the concrete-forest GLB at t≈12 s, the decoded plan image —
   arrives after the last presented packet and can never reach the screen. One contributing cause
   (five no-op interaction checkouts per frame, which superseded the very frame build that armed
   them) is fixed in §3; **a second, independent cause is still live and is this packet's hand-off**
   (§5.1).

The scene therefore still shows no 3D content on the live boot. What changed is that the *reason* is
now measured rather than suspected, and two of the three walls are gone.

---

## 1 — What the measurement actually says

### 1.1 — The starting state, reproduced

`🗑️generated/w5c-base-1` (diagnostics probe, 45 s, fresh profile). The census confirms W3d's
hand-off verbatim on the live serve:

```
world3d surface=puzzle3d-main-perspective bounds=956x814+481,54
  draws=1 translucent=0 instances=1 lines=1 textured=1 passes=2
  state-draws=1 state-instances=1 state-meshes=5 fault=None
  meshes=[… mesh:🧊️hexagonal-cut-concrete-forest-left:v847/i2874@[0.00,0.00,0.00]..[10.80,4.68,3.00] …]
```

`dumpFrameStats` answers `sceneDraws: 3, sceneInstances: 2` — the mesh draw with its instance, the
textured draw with its instance, and the line draw are all in the live DrawList, on both surfaces.

### 1.2 — The decisive counter: the mesh encoder is never called

A temporary `[DEBUG]` probe inside `UiPipelines::encode_prepared_world_instance` (and its twin in
`encode_prepared_world_line`), rebuilt and re-activated:

| probe | `w5c mesh` lines | `w5c line` lines |
| --- | --- | --- |
| `🗑️generated/w5c-probe-2` | **0** | (gated behind the first mesh, so 0) |

So no depth test, no vertex layout and no colour ever gets the chance to reject anything: the draw
call is not issued. Every hypothesis in the packet brief that lives *inside* the pass — depth
`Less` vs `LessEqual`, `LoadOp::Load` on a stale depth buffer, `CompareFunction::Always`, the
stride 24→40 change, `@location(2)`, the instance alpha, the frustum — is ruled out by this one
counter. (For the record, the geometry is fine too: the perspective camera sits at
`[9.17, -5.67, 4.26]` looking at `[3.5, 0, 0.005]` with `up = [0,0,1]`, fov 50°, and the model's
Z-up bounds are `[0..10.8, 0..4.68, 0..3.0]` — dead centre of the frustum.)

### 1.3 — Why: exactly one prepared frame is presented per boot

A temporary census at the `Commands → BlurScene` transition of the prepared GPU ladder — one line
per PRESENTED frame — fired **once** in 50 s (`🗑️generated/w5c-probe-5`, `-6`, `-7`):

```
w5c ladder seen=0 revision=3 generation=2 pages=14255
  instances=0 lines=13024 textured=146 pass-headers=4 ui=509 foreground=164
  draw-passes=2 draw-draws=0 draw-instances=0 draw-textured=2
  pass viewport=[3.2, 54.4, 477.9, 813.6] draws=0 lines=1 textured=1 keys=[]
  pass viewport=[481.1, 54.4, 955.7, 813.6] draws=0 lines=1 textured=1 keys=[]
```

That packet is the BOOT frame: `draw-draws=0`, because the GLB has not landed yet. No later packet
is ever presented, so `draws=1` never reaches the GPU.

Confirmed independently, with no instrumentation at all, by the present probe
(`🗑️generated/w5c-present-1`): the canvas hash changes once and is then **byte-identical from
t=10 s to t=45 s**.

```
run 1 final=a314d261acba 6s:bc54791ef686 10s:a314d261acba 14s:a314d261acba
      20s:a314d261acba 30s:a314d261acba 40s:a314d261acba lit=13361/13361
```

Meanwhile the shell keeps *building* frames — 186 admitted builds and ~190 `world3d surface=…`
census lines per surface in the same window. **Frames are built and thrown away.**

## 2 — Fix 1: the world TEXTURED pass, built

`TexturedInstance3d { texture_key, model, tint }` is React's `WorldReferencePlaneItem`
(`🎨️r3f/🟦️.tsx:3958`): a `planeGeometry(width, height)` posed by the reference's origin, tinted,
`transparent`, `side={DoubleSide}`, `renderOrder=-10`. `render_world_3d`
(`♾️infinite/🌍️world/🦀️.rs:10846`) publishes exactly that and uploads the decoded pixels through
`ensure_world_plane_texture` into the retained raster table under the url as key. Everything up to
the GPU existed; the GPU end did not.

| site | change |
| --- | --- |
| `🖍️draw/🦀️.rs:311` | new `World3dTexturedGpuInstance` — model (4×`vec4`) + tint, **80 bytes**, the stride `WORLD3D_TEXTURED_PIPELINE` declares |
| `🖍️draw/🦀️.rs:337` | new `WORLD_PLANE_VERTICES` — a centred unit XY quad, position(3)+uv(2), stride 20, with `v = 0` at `+y` so row 0 of the decoded image is the quad's TOP (three.js `flipY`, which is what React shows) |
| `🖍️draw/🦀️.rs:2711` | the `world3d_textured_pipeline`: layout `[world globals (dynamic), scene sampler group]`, `ALPHA_BLENDING`, `cull_mode: None`, `depth_write_enabled: false`, `depth_compare: LessEqual`, `content_stencil_state()` |
| `🖍️draw/🦀️.rs:3413` | new `encode_prepared_world_textured` — same viewport/scissor/globals-ring contract as `encode_prepared_world_instance`, group 1 built per draw from the resident `RasterTexture::view` plus a clamped linear `world_plane_sampler` |
| `🧊️gpu/🦀️.rs:158` | `prepared_draw_scalar_is_glass_foreground` resolves `PassTexturedInstance` through its pass's `layer_index`, exactly like the mesh and line cursors, so a textured scalar lands on the same target its pass does |
| `🧊️gpu/🦀️.rs:733` | the ladder's missing arm: `DrawMeasureCursor::PassTexturedInstance` → `encode_prepared_world_textured` |
| `✨️shader-contract/🦀️.rs:724`, `:766` | the contract is no longer "inferred/unwired": the label, blend, depth and stride are now read off the real construction. `🍎️metal/🌐️world3d` and `🪟️d3d12/🌐️world3d` said the same thing in their own headers and are corrected |

**The underlay is a `LessEqual`, non-depth-writing, alpha-blended pass on purpose.** It is
semi-transparent (`tint.a = 0.85`) and sits under geometry that already wrote depth, so the model
still occludes it; a depth-writing underlay would punch its own quad through everything behind it.

**A key whose pixels have not landed yet draws nothing and is NOT a fault** — the instance is
published the frame the url appears, long before the fetch/decode finishes, exactly as React returns
`null` from `WorldReferencePlaneItem` until `media` resolves.

## 3 — Fix 2: the chrome I/O lane is pressure-driven again

With the ladder instrumented, the reason no second frame is ever presented came apart in two layers.
The first is a livelock between the shell's chrome maintenance and the frame transaction.

`render_chrome_step` armed **three** maintenance flags on EVERY walk, unconditionally:

```rust
ShellChromeFramePhase::Presence          => self.request_presence_preview(),
ShellChromeFramePhase::PersistLayout     => self.request_panel_layout_persist(),
ShellChromeFramePhase::PersistPreferences=> self.request_chrome_preferences_persist(),
```

and the preferences arm takes three deferred steps to clear — **five `FrameDeferredWork::ShellMaintenance`
steps per frame that all have nothing to do**. Each one is a full
`check_out_interaction("frame-deferred")` plus an async hand-back, and `FrameTransaction::step`
refuses to run at all while the interaction state is out
(`AppFrameTransactionStep::Superseded`, `🧊️renderer/🦀️.rs:12071`). Measured
(`🗑️generated/w5c-probe-9`/`-10`):

```
w5c supersede seen=1 interaction-unavailable site=Some("frame-deferred") opportunities=0
…                                            (every frame build, from t=10.4 s to t=45 s)
w5c deferred seen=0 start shell-maintenance
w5c deferred seen=1 resume returned=true
w5c deferred seen=2 start shell-maintenance      ← re-armed inside the same apply
```

`opportunities=0` is why nothing ever reported it: the checkout ledger only AGES a checkout while a
restoring completion is queued, so the abandoned-notice watchdog never fires on a lane that keeps
handing the state back and taking it again.

| site | change |
| --- | --- |
| `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:16600` | `request_panel_layout_persist` arms only when `last_persisted_dock_ui`/`last_persisted_dock_skeleton` differ from the live dock |
| `:16610` | `request_presence_preview` arms only on a native build with a live `sync_channel` — the browser has no sync backbone and the step there only cleared its own flag |
| `:16624` | `request_chrome_preferences_persist` arms only when `UiPrefsSnapshot::capture(self)` differs from `last_synced_preferences`, which is what its docstring always claimed |

**Measured effect** (`🗑️generated/w5c-probe-12`, same 45 s boot): deferred maintenance checkouts
**186 → 16**, transaction supersessions **186 → 3**, and the maintenance lane goes quiet at t≈10.5 s
and stays quiet. The remaining three supersessions are real pressure (the locale re-resolve that
follows the preferences load).

## 4 — What the mesh still waits on, stated precisely

After §3 the frame builds are no longer superseded — and they still produce no presentation
(`🗑️generated/w5c-probe-13` … `-21`):

```
w5c build seen=0  prepare complete presentation=true
w5c build seen=1  terminal requested=Generation(2)  frame=Generation(2)  completed=true
w5c build seen=2  terminal requested=Generation(3)  frame=Generation(3)  completed=false outcome=Complete(CommitCandidate{…})
w5c build seen=3  terminal requested=Generation(4)  frame=Generation(4)  completed=false outcome=Complete(…)
w5c build seen=5  terminal requested=Generation(6)  frame=Generation(6)  completed=false outcome=Cancelled
…                                                                        (every build to t=45 s)
```

Read that carefully, because it rules out most of the obvious candidates:

- the generations MATCH, so `generation_is_fresh` is not dropping the frame;
- `SUSTAINED_OVERRUN_QUARANTINES` stays **0** and `RECORDED_STEP_OVERRUNS` freezes at 16, so the job
  layer's overrun quarantine is not terminating the build;
- no `frame build superseded`, no `present_step faulted`, no `frame fault recorded`, no wgpu
  validation message — the failure is completely silent;
- the transaction DOES run: a stage probe counted **224 000** `FrameTransaction::step` calls in 45 s
  cycling `RouteIntents`/`BuildRenderPackets`, and the `world3d surface=…` census (which is printed
  from the scene paint) keeps firing.

Two facts point at where the remaining wall is:

1. **The asset decode pump eats the transaction's entire wall slice.** `FrameTransaction::step`
   returns `Pending` before it does anything else while `runtime.pump_renderer_asset_decode_step()`
   answers true, and W3d's `while !context.deadline_exceeded() && …{}` loop spends the whole slice
   there. Measured: `asset-decode-pump units=306 deadline=true`, over and over, and still firing at
   **t=15.3 s** (`🗑️generated/w5c-probe-16`, `-21`). A branch-tagged census of that pump
   (`🗑️generated/w5c-probe-17`) attributes ~100 % of its `true` answers to
   `RendererAssetProbeStep::Pending` — the probe's own bounded byte scan. Four GLB decodes (two
   meshes × two surfaces) plus two reference images legitimately cost tens of thousands of steps
   each, so this may be *slow* rather than *stuck*; what it is NOT is free, and while it is busy the
   frame transaction cannot reach its first phase.
2. **From generation 6 on the build's own outcome is `Cancelled`** — returned by the job layer
   before `ActiveFrameBuild::advance` runs even once (every `Complete(None)` path inside `advance`
   was instrumented and none of them fired). Something cancels the build's fresh
   `root_cancel_token()` between admission and its first step.

Both belong to the frame-build/presenter lane rather than to the world pass, and a wrong fix there
is worse than a precise hand-off — see §5.1.

## 5 — Hand-offs

### 5.1 — P0: the wgpu shell presents one frame per boot

This is now the ONLY thing between here and React's grey slab, and it is bigger than the world pass.
Everything needed to continue is in this report: the reproduction is
`SEMIO_PROBE_OUT=… bun 🐍️w3a-wgpu-diagnostics-probe.mjs` plus
`SEMIO_PROBE_RUNS=1 SEMIO_PROBE_SHOTS=6,10,14,20,30,40 bun 🐍️w3c-wgpu-present-probe.mjs` (the
canvas-hash cadence is the cheapest possible verdict — identical hashes from t=10 s means frozen),
and the four instrumentation points that produced §4 are described above so they can be re-added in
one build each:

1. a census at the ladder's `Commands → BlurScene` transition (one line per presented frame);
2. `debug_frame_build` at `FrameBuildHandle::poll_runtime_and_resubmit`'s `WorkerJobPoll::Terminal`
   arm, printing `owner.outcome()` and `completed.is_some()`;
3. a tag on each of the four `AppFrameTransactionStep::Superseded` returns in
   `FrameTransaction::step`;
4. a unit counter on `pump_renderer_asset_decode_step`'s `true` branches.

The next step is to find who cancels the build token (§4 item 2) — `ActiveFrameBuild::new` takes a
fresh `root_cancel_token()`, so the cancel arrives from outside the build.

### 5.2 — P1: the decode pump owns the transaction's whole slice

W3d's `while !context.deadline_exceeded() && runtime.pump_renderer_asset_decode_step() {}` is correct
about the fuel unit being the wrong bound, but it now means that while ANY asset is decoding the
frame transaction makes no progress at all. A share (say half the slice) rather than the whole slice
would let the shell keep presenting while a GLB streams — which is exactly what React does.

### 5.3 — P2: the interaction-checkout ledger cannot see a lane that re-takes the state

`opportunities=0` for a checkout that had held the state for 35 s (§3). The ledger ages a checkout
only while a restoring completion is queued, so a lane that hands the state back and immediately
takes it again is invisible to the abandoned-checkout watchdog. Ageing on *apply opportunities* — not
on queued restorers — would have named this in one line.

### 5.4 — P2: the world grid is kilometre-scale

The LOD grid submits segments from `-6000` to `+6000` at 100-unit spacing (`w5c line … v0=[-5500.0,
-6000.0, 0.002]`) against a `near=0.1 / far=1000` projection, so almost every line is clipped and the
pane shows a handful of stray diagonals where React shows a dense local grid. Not this packet's
lane, but it is why the wgpu pane looks empty rather than gridded.

## 6 — VERIFY (all foreground, all RUN)

| gate | result |
| --- | --- |
| `cargo check -p semio-framework-ui --features wgpu-engine --lib -j 4` | **0 errors** (`🗑️generated/w5c-check-ui.txt`) |
| `cargo check -p semio-framework-ui --features wgpu-engine --lib --target wasm32-unknown-unknown -j 4` | **0 errors** (`w5c-check-ui-wasm.txt`) |
| `cargo check -p semio-framework-os-renderer-wgpu --lib -j 4` | **0 errors** (`w5c-check-renderer.txt`) |
| `cargo check -p semio-framework-os-renderer-wgpu --lib --target wasm32-unknown-unknown -j 4` | **0 errors** (`w5c-check-renderer-wasm.txt`) |
| `cargo test -p semio-framework-ui --features wgpu-engine --lib -- --test-threads=1 wgpu::gpu wgpu::draw` | **50 passed / 0 failed** (48 before this packet; both new laws included) |
| `cargo test -p semio-framework-ui --features wgpu-engine --lib -- --test-threads=1 text chrome glyph` | **64 passed / 0 failed** |
| `cargo test -p semio-framework-os-renderer-wgpu --lib -- --test-threads=1 browser_prefs chrome_maintenance present_stall_watch_tests` | **15 passed / 0 failed** |
| `cargo test -p semio-framework-ui-render --lib -- --test-threads=1 metal d3d12 shader` | **6 passed / 0 failed** (incl. `world3d_wgsl_is_byte_identical_in_the_wgpu_target_and_the_shader_contract`) |
| `cargo test -p semio-framework-os-renderer-wgpu --lib -- --test-threads=1 async_boundary_tests` | 27 passed / **2 failed** — `presenter_ack_retirement_source_mutations_are_denied` and `raster_upload_cache_is_fixed_generation_witnessed_and_mutation_complete`, the exact two `📓️w3c-black-chrome-regression.md` §5 and `📓️w3d-world3d-glb-url-lane.md` §5 both record as pre-existing peer-lane source contracts (`⚙️EngineCanvas`, the mesh-GPU retirement lane). Neither mentions the textured pass |
| `…:framework-renderer-wgpu:wasm --skip-nx-cache` then `…:activate-puzzle3d-wgpu-dev` | **exit 0** (`w5c-wasm-final.txt`, `w5c-activate-final.txt`) |

### Tests added

| test | file | what it pins |
| --- | --- | --- |
| `a_textured_world_instance_is_encoded_into_its_pass_target` | `🖱️ui/🧪️tests/🔬️targets-wgpu-gpu-prepared-present/🦀️.rs` | the ladder owns a `PassTexturedInstance` arm that reaches `encode_prepared_world_textured`, and a textured scalar is classified onto the same target as every other scalar of its pass (including the stale-index case) |
| `the_world_textured_pass_is_a_blended_underlay_on_a_centred_plane` | same | the 80-byte instance stride, the centred unit XY quad, `u` growing with `+x` and `v` with `-y` (row 0 of the image is the quad's top), and the built pipeline's `ALPHA_BLENDING` / `depth_write_enabled: false` / `LessEqual` / `cull_mode: None` / `array_stride: 20` |
| `no_chrome_maintenance_lane_arms_itself_without_pressure` | `🐚️Shell/🧪️tests/💓️chrome-maintenance-pressure/🦀️.rs` (new) | each of the three maintenance arms consults its own pressure before arming the shared I/O lane |
| `the_chrome_walk_still_visits_every_maintenance_phase` | same | the fix is the guard inside the arm, never a deleted chrome phase |

## 7 — Probe evidence

| output | what it shows |
| --- | --- |
| `🗑️generated/w5c-base-1` | the starting state: `draws=1 instances=1 textured=1`, mesh resident at `v847/i2874`, nothing painted |
| `🗑️generated/w5c-probe-2` | the decisive counter — `encode_prepared_world_instance` is called **zero** times |
| `🗑️generated/w5c-probe-5` … `-7` | exactly ONE presented packet per boot, and it is the boot packet (`draw-draws=0`) |
| `🗑️generated/w5c-present-1` | the same verdict with no instrumentation: canvas hash identical from 10 s to 45 s |
| `🗑️generated/w5c-probe-9`, `-10` | the maintenance livelock, named: `interaction-unavailable site=Some("frame-deferred") opportunities=0`, five `start shell-maintenance` per frame |
| `🗑️generated/w5c-probe-12` | after fix 2: deferred checkouts 186 → 16, supersessions 186 → 3, lane quiet from t≈10.5 s |
| `🗑️generated/w5c-probe-13` … `-21` | the remaining wall of §4: builds terminate `completed=false`, `outcome=Complete(…)` then `Cancelled`, with zero overrun quarantines |
| `🗑️generated/w5c-probe-16`, `-17` | the asset decode pump owning the transaction's whole slice, ~100 % of its work attributed to `RendererAssetProbeStep::Pending` |
| `🗑️generated/w5c-final/final.png` | the 45 s canonical boot on the final build |

All temporary `[DEBUG]` instrumentation was removed:
`grep -rn "w5c\|debug_frame_build\|debug_frame_poll\|debug_supersede\|debug_stage\|debug_asset_branch\|debug_world_probe\|debug_ladder_census\|debug_upload_cursor" 🧰️framework`
is empty, and the `"console"` web-sys feature the probes needed is out of
`🖱️ui/📦️packages/🦀️rust/Cargo.toml` again.

## 8 — Remaining differences of the 3D view vs React, at 1440×900 dpr 1

Measured against `🗑️generated/react-6313/final.png`.

1. **No 3D content at all** — the model and the plan underlay. §1.3 / §5.1; not a pass defect any
   more, a presentation-lane defect.
2. **The grid.** React paints a dense local grid under the model; wgpu submits a ±6000-unit grid at
   100-unit spacing against a 1000-unit far plane, so only a few stray diagonals survive clipping
   (§5.4).
3. **Lighting and material.** `WORLD3D_SHADER` is one directional term with a `0.28` ambient floor
   and a flat instance tint; React's `GlbInstanceMesh` bakes colour, emissive, opacity, border and
   the `celebrated` shader into the cloned GLB scene (W3d §8 item 4, unchanged here).
4. **The underlay's paint order.** React gives the reference `renderOrder=-10`, so the grid and the
   model draw OVER it. The prepared cursor's fixed order is opaque → lines → translucent → textured,
   so the wgpu underlay composites last and will tint the grid lines it covers. Changing this means
   moving `next_after_translucent`/`next_after_opaque`, which is a measure-cursor change with its own
   fixture; left as a deliberate, documented difference.
5. **No boot tour** (W4a's lane) and the light/dark boot input (W5a's lane) — both already tracked.
