# 🖤️ W3c — the black canvas: a swapchain texture held across host tasks

Packet W3c of ticket `26/09/17/WGPU-RENDERER-REACT-PARITY`. Every claim below is backed by a probe
output or a test that was RUN, named inline. Line numbers are post-edit.

**Result: the wgpu shell chrome paints on every boot — 8/8 identical, deterministic, no fault for
45 s** (`🗑️generated/w3c-final`, `🗑️generated/w3c-boot-final/final.png`). Two defects were found and
fixed: the swapchain texture was acquired in one host pump and written in a later one, and the glass
regions erased their own labels.

---

## 1 — The reported symptom was NOT what the brief described

The brief reported a permanently black canvas after W3a+W3b. Measured on the live serve first,
before touching anything:

| probe | build | verdict |
| --- | --- | --- |
| `🗑️generated/w3c-plain-1` | W3a/W3b build | **PAINTED** — full chrome, Anta, single-line chips |
| `🗑️generated/w3c-plain-2`, `-3` | same | PAINTED (byte-identical) |
| `🗑️generated/w3c-plain-4` | same | **BLACK** (`lit=0/13361`) |
| `🗑️generated/w3c-present-1` (6 boots) | same | 5 painted, 1 black |
| `🗑️generated/w3c-jiggle-1` (8 boots) | same | 6 painted, 2 black |

So the regression is **intermittent, roughly 1 boot in 5, and sticky**: a boot that is black at 10 s
is still black at 70 s, and stays black through 40 scripted pointer moves
(`🗑️generated/w3c-jiggle-1/run-4/after-jiggle.png`). W3a's `🗑️generated/w3a-4`/`-5`/`w3a-diag-headed`
were black boots and `w3a-diag-1` was a painted one **of the same build** — which is why the packet
brief and W3a §6 disagreed with each other.

Two further facts that cost time and are worth writing down:

- **The console is byte-identical between a painted and a black boot.** Diffing
  `w3c-plain-3/console.txt` against `w3c-plain-4/console.txt` (timestamps stripped) leaves only
  millisecond timings. No fault, no page error, no wgpu validation message. The failure is invisible
  above the GPU submit.
- **`dumpFrameStats` cannot see the chrome at all.** `{"drawCalls":0,"quadCount":17,"glyphCount":0}`
  is the *app* window (`puzzle3d-main-perspective`, a bare `componentScene`) — `dump_window_id`
  picks the largest viewport, which is a pane, never the shell. Those numbers are identical on
  painted and black boots (`w3c-present-1/run-*/dumps.json`) and say nothing about the chrome. The
  brief's "17 chrome quads exist" reading is a misattribution: they are the scene window's quads.

## 2 — Root cause 1: the swapchain texture was held across host tasks

Instrumented the ladder with two temporary `[DEBUG]` counters (both removed again, §6): a host-pump
sequence number bumped once per `OsHost::build_and_publish_snapshot`, and one line per
`PreparedGpuPresentPhase` transition carrying that sequence. `🗑️generated/w3c-ladder-2`, six boots:

| run | verdict | `2->3` (about to acquire) | `3->4` (acquired) | `6->7` (composite blitted) |
| --- | --- | --- | --- | --- |
| 1 | **BLACK** | pump 194 | pump **194** | pump **195** |
| 2 | painted | pump 191 | pump 191 | pump **191** |
| 3 | painted | pump 191 | pump 191 | pump **191** |
| 4 | **BLACK** | pump 188 | pump 188 | pump **189** |
| 5 | painted | pump 177 | pump 177 | pump **177** |
| 6 | **BLACK** | pump 192 | pump 192 | pump **193** |

**6/6 correlation.** Every black boot acquired the surface texture in one host pump and blitted the
scene onto it in a LATER one; every painted boot did both in the same pump.

The mechanism: **a browser expires the canvas's current texture at the end of the task that obtained
it**, and presents whatever that texture held at that moment. A `wgpu` `SurfaceTexture` in wasm is
therefore a single-task resource — `frame.present()` is not what publishes it. The prepared present
ladder is explicitly stepped ACROSS tasks:

```rust
let present_deadline_us = default_now_us().map(|now| now.saturating_add(INTERACTIVE_STEP_CEILING_US / 2));
loop { match self.presenter.present_step() { … } if deadline_passed { return; } }
```
(`🪟️winit-app/🦀️.rs:251-283`) — so the ladder yields to the browser wherever the 4 ms slice runs out.
When that yield fell between `AcquireSurface` and `EncodeComposite`, the compositor published the
untouched texture. `wgpu` configures the canvas `alphaMode: opaque`, so an untouched texture is
**pure black (0,0,0)** — exactly what the screenshots show, and distinguishable from the page's own
theme background `rgb(0,17,23)` that shows before the first present (`w3c-present-1/run-3/shot-8s.png`).

It is sticky because the ladder is deterministic: once a boot's phase offsets put the yield inside
that window, every subsequent frame repeats it. It is only ~1 in 5 because the offset is decided by
boot jitter.

This was always latent; W3a (the asset lane now spends its whole wall slice) and W3b (a shaped atlas,
more uploads and more commands) changed the step timing enough to make it frequent. It also bit in a
second, quieter way on the boots that DID paint: `7->8` is ~131 ms after `6->7` in every run — the
glass scan over 13 770 command pages — so on a painted boot the surface texture was held across ~8
compositor frames and everything encoded after the first boundary was silently dropped.

### The fix

The swapchain is now touched by exactly ONE phase, which acquires it, writes it, submits and presents
inside a single opportunity. Everything before it composites offscreen.

| site | change |
| --- | --- |
| `🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🦀️.rs:153` | new `PreparedCompositeTarget` — one surface-sized, single-mip colour target, `RENDER_ATTACHMENT \| TEXTURE_BINDING`, recreated on resize. Docstring carries the law and the measurement |
| `🖍️draw/🦀️.rs:3769` | `blit_scene_to_swapchain` → `blit_sampled_color(view, source_view, sampler)`; `blit_prepared_scene` (`:3434`) and the new `blit_prepared_composite` (`:3440`) are its two callers |
| `🧊️gpu/🦀️.rs:256`, `:400` | `GpuContext::composite_color`; `ensure_scene_color` → `ensure_prepared_targets`, which ensures both; `resize` drops both |
| `🧊️gpu/🦀️.rs:22-33` | `AcquireSurface` and `CreateView` phases **deleted**. Ladder is now `EnsureTarget → ClearScene → Commands → BlurScene → EncodeComposite → GlassCommands → ForegroundCommands → Present` |
| `🧊️gpu/🦀️.rs:583` `EncodeComposite` | blits the scene into `composite.view()`, not the surface |
| `🧊️gpu/🦀️.rs:625` `Present` | acquires `get_current_texture`, creates the view, blits the composite onto it, submits and `frame.present()`s — **one straight line inside one opportunity**. It is the only code in the file that names `get_current_texture` |

Cost: one extra full-surface blit and one surface-sized texture (5.2 MB at 1440×900). The per-phase
2 ms opportunity ceiling and the `SUSTAINED_OVERRUN_QUARANTINE_STEPS` law are untouched — the terminal
step is an acquire plus one fullscreen quad.

**Measured: `🗑️generated/w3c-fix-1` — 8/8 boots painted, all byte-identical** (was 3/4, 5/6, 6/8).

## 3 — Root cause 2: every glass region erased its own label

With the composite landing reliably, a second defect became visible that the lost-composite bug had
been masking: the window cap row (`Puzzle 3D ⤢ ✕ ⠿`, y 32..80) was **blurred out**
(`🗑️generated/w3c-fix-1/run-1/final.png`, diff bbox `(3, 32, 1434, 80)` against the last painted
pre-fix boot).

`🛰️Dock/🎯️targets/🧊️wgpu/🦀️.rs:1547` pushes a `Level::Window` glass region over the dock group rect and
opens `begin_glass_content` for the chips that sit ON it; `🐚️Shell/…:3878` does the same. The BATCH
renderer has always honoured that split (`LayerBatchFilter::Backdrop` / `Foreground`,
`blit_scene → composite_glass_regions → render_glass_foreground`). **The scalar ladder did not**: it
encoded every layer into the scene and then composited the glass over the lot, so the chips were
painted and then sampled away by the very region that labels them. Natively this has always been the
behaviour; in the browser it was invisible because the glass encodes landed after the compositing
boundary and were thrown away.

| site | change |
| --- | --- |
| `🧊️gpu/🦀️.rs:76` | `PreparedDrawTarget { Scene, Composite }` |
| `🧊️gpu/🦀️.rs:89` | `prepared_draw_scalar_is_glass_foreground(draw, cursor)` — resolves a measured scalar's owning layer (`LayerUi`/`LayerVector`/`LayerRaster` directly, `PassInstance`/`PassLineVertex` through `scene_passes[..].layer_index`) and reads the draw list's own `foreground_of` |
| `🧊️gpu/🦀️.rs:555` `Commands` | skips glass-foreground scalars; everything else goes to `PreparedDrawTarget::Scene` |
| `🧊️gpu/🦀️.rs:611` new `ForegroundCommands` | re-walks the measured command pages after the glass pass and encodes ONLY those scalars, into `PreparedDrawTarget::Composite` |
| `🧊️gpu/🦀️.rs:658` | `encode_prepared_draw_scalar` takes the target; the four `encode_prepared_*` callees take a `color_view: &wgpu::TextureView` instead of a `&SceneColorTarget` (`🖍️draw/🦀️.rs:3180`, `:3223`, `:3262`, `:3315`) |

Result: cap chips are crisp again and the 3D grid BEHIND them is blurred — the glass now does what it
is for. `🗑️generated/w3c-cap-before.png` / `w3c-cap-after.png` are the 8× crops; the only difference
between the pre-fix painted boot and the final one is the blurred backdrop inside the glass band.

## 4 — Root cause 3 (self-inflicted, caught by the probe): the watchdog could not see the new walk

The first build of §3 quarantined the surface on EVERY boot
(`🗑️generated/w3c-fix-2`, 6/6): `worker-present-failed: presentation stalled: phase=Render engine=0
upload=1 gpu-cursor=Some((6, 13770, 13770, 5))`. `PreparedGpuPresentCursor::progress()` is the only
thing `AppPresentStallWatch` can see inside one `AppPresentPhase::Render`, and the new
`foreground_command` index was not in it — so a healthy 13 770-page walk looked frozen and hit
`APP_PRESENT_STALL_STEPS`.

`progress()` is now the 5-tuple `(phase, command, glass_command, foreground_command, blur_mip)`
(`🧊️gpu/🦀️.rs:144`), `AppPresentProgress` follows (`🧊️renderer/🦀️.rs:12825`), and the shared fixture
`🧫️fixtures/🐕️present-stall-watch/🔣️.json` was widened with a new case that pins exactly this walk.
**The `progress()` docstring now states the rule: every cursor the ladder walks belongs in the
signature.**

## 5 — Tests

| test | file | what it pins |
| --- | --- | --- |
| `the_swapchain_is_acquired_written_and_presented_in_one_prepared_opportunity` | `📺️renderer/…/🧪️tests/🔬️wgpu-renderer-async-boundary/🦀️.rs:1216` | exactly one `get_current_texture` and one `frame.present()` in the ladder; acquire < composite blit < submit < present inside the `Present` arm; no `AcquireSurface`/`CreateView` phase exists; `EncodeComposite` and `GlassCommands` write `composite.view()` |
| `glass_foreground_scalars_are_encoded_after_the_glass_pass_and_never_into_the_scene` | same, `:1246` | `Commands` skips glass-foreground scalars (`PreparedDrawTarget::Scene`), `ForegroundCommands` encodes only them (`PreparedDrawTarget::Composite`), and its index is in the watchdog signature |
| `a_painted_chrome_window_reports_draw_calls_quads_and_glyphs` | `🗣️Interpreter/🧪️tests/🔬️wgpu-introspection/🦀️.rs:168` | **the brief's oracle law** — drives the REAL pipeline (`apply_tree` → real `MountedLayoutJob` → real retained paint) and asserts `build_frame_stats` answers `drawCalls > 0`, `quadCount > 0`, `glyphCount > 0`, and `glyphCount <= quadCount`. `build_frame_stats`/`dump_window_id`/`DumpFrameStats` are now `cfg(any(wasm32, test))` so the oracle is reachable natively at all |
| `every_ladder_index_moves_the_watchdog_signature` | `🖱️ui/🧪️tests/🔬️targets-wgpu-gpu-prepared-present/🦀️.rs:110` | §4's regression: command, glass command, glass-foreground command, blur mip and phase each move `progress()`, and a closed cursor zeroes them all |
| `only_a_glass_content_layer_is_split_off_the_scene_target` | same, `:140` | the classifier over a real `DrawList` with a backdrop layer, a `begin_glass_content` layer and a trailing layer; a stale layer index, a `Glass` cursor and a layerless scene pass are never foreground |
| `the_presentation_watchdog_names_a_frozen_cursor_once_and_a_healthy_one_never` (fixture extended) | `🧪️tests/🐕️wgpu-present-stall-watch/🦀️.rs` + `🧫️fixtures/🐕️present-stall-watch/🔣️.json` | new case: a `ForegroundCommands` walk that moves only its own index is healthy for 4 095 steps |

`📜️script.ts:11648-11662` (the P5d `gpuBoundary` law) and
`🖌️render/🧪️tests/🔬️interactivity-mounted-prepared-render/🟦️.ts` were updated for the new call shape,
the new phase and the new cursor, plus a new `bulk-gpu-glass-foreground` mutation.

### VERIFY (all foreground, all RUN)

| gate | result |
| --- | --- |
| `cargo check -p semio-framework-ui --features wgpu-engine --lib -j 4` | **0 errors** |
| `cargo check -p semio-framework-os-renderer-wgpu --lib -j 4` | **0 errors** |
| `cargo check -p semio-framework-os-renderer-wgpu --lib --target wasm32-unknown-unknown -j 4` | **0 errors** |
| `cargo test -p semio-framework-ui --features wgpu-engine --lib -- --test-threads=1 wgpu::gpu wgpu::draw` | **48 passed / 0 failed** (incl. both new laws) |
| `cargo test -p semio-framework-ui --features wgpu-engine --lib -- --test-threads=1 text chrome glyph` | **62 passed / 0 failed** (W3b's and W1g's laws still hold) |
| `cargo test -p semio-framework-os-renderer-wgpu --lib -- --exact interpreter::introspection_tests::a_painted_chrome_window_reports_draw_calls_quads_and_glyphs` | **1 passed** |
| `cargo test -p semio-framework-os-renderer-wgpu --lib -- --test-threads=1 present_stall_watch_tests` | **2 passed** |
| `cargo test -p semio-framework-os-renderer-wgpu --lib -- --test-threads=1 async_boundary_tests::the_swapchain async_boundary_tests::glass_foreground` | **2 passed** |
| `activate-puzzle3d-wgpu-dev` | **exit 0** (`🗑️generated/w3c-activate-7.txt`) |

**Failures that are NOT this packet's**, each attributed by code path against files this packet never
opened:

- `async_boundary_tests::raster_upload_cache_is_fixed_generation_witnessed_and_mutation_complete` and
  `…::presenter_ack_retirement_source_mutations_are_denied` — their contracts want literals that no
  longer exist anywhere (`self.ensure_raster_texture_step(key, pixels`, `surface.view = view`,
  `mesh_gpu_retirement_preserves_acknowledged_versions`), in `⚙️EngineCanvas/…/🦀️.rs` (1 002 peer
  insertions in the working tree) and in the mesh-GPU lane.
- `interpreter::render_plan_validator_tests::*` (4) — inline SVG data-url decoding, a peer's ui-image
  lane.
- The P5d policy self-test still reports two failures, both pre-existing drift in code this packet did
  not touch: `gpuBoundary` wants `"default_now_ms() - started > 2"` (the ladder moved to
  `default_now_us()` + `admit_prepared_gpu_opportunity` in an earlier ticket) and
  `preparationBoundary` wants `"step_budget_ms: 1"`. Every literal THIS packet is responsible for
  binds — verified term by term.

## 6 — Temporary instrumentation (added, used, removed)

`crate::debug_present_pump_seq`/`debug_bump_present_pump_seq` (a thread-local pump counter in
`🧊️renderer/🦀️.rs`), the `[DEBUG] gpu ladder <from>-><to> pump=…` line in `AppPresentPhase::Render`,
and the pump bump in `🪟️winit-app/🦀️.rs` were added for §2 and are **gone**: `git diff` of
`🪟️winit-app/🦀️.rs` now shows only W1g's and W3a's edits, and `grep -rn "ladder_before\|ladder_after\|
debug_bump_present\|DEBUG_PRESENT_PUMP" 🧰️framework` is empty.

## 7 — Probe evidence

| output | what it shows |
| --- | --- |
| `🗑️generated/w3c-plain-1`…`-4` | the flake: 3 painted, 1 black, same build, identical console |
| `🗑️generated/w3c-present-1` (6 runs) | 5/6 painted; the theme background at 8 s vs pure black at 16 s |
| `🗑️generated/w3c-jiggle-1` (8 runs) | black is sticky through 40 pointer moves; `run-8` also caught a late `frame world resource admission exceeded fixed credits` (W3a §3's P1, still live, unrelated to the black) |
| `🗑️generated/w3c-ladder-1`, `-2` | the instrumented bisect — the 6/6 pump correlation of §2 |
| `🗑️generated/w3c-fix-1` (8 runs) | composite target only: 8/8 painted, cap text blurred by the glass |
| `🗑️generated/w3c-fix-2` (6 runs) | the watchdog stall of §4 |
| `🗑️generated/w3c-fix-3` (6 runs) | glass-foreground split: 6/6 painted, cap crisp |
| **`🗑️generated/w3c-final` (8 runs, diagnostics OFF)** | **8/8 painted, byte-identical** |
| **`🗑️generated/w3c-boot-final/final.png`** | the 45 s canonical boot: navbar, both window caps, both panes, footer; every `fault=` line reads `None`; 88 console lines, 1 failed request (favicon) |
| `🗑️generated/w3c-cap-before.png`, `w3c-cap-after.png` | 8× crops of the window cap, pre-fix painted vs final |

New probe script: `🐍️w3c-wgpu-present-probe.mjs` — runs N boots in one browser, screenshots at a fixed
cadence, hashes each shot, samples the canvas itself for lit pixels (`lit=0/13361` is the black
verdict) and dumps `dumpFrameStats` per run. `SEMIO_PROBE_RUNS/SECONDS/SHOTS/DIAGNOSTICS/JIGGLE/DPR`.
A single boot proves nothing here; this is the instrument that made the flake measurable.

## 8 — Remaining chrome differences vs React, at the same viewport

⚠️ **The React serve on 6013 is DOWN** — every request answered `ERR_CONNECTION_REFUSED` during this
packet (`🗑️generated/w3c-react/requests-failed.txt`, 109 failures, blank cream page). The comparison
below is against W3b's earlier capture `🗑️generated/w3b-react/final.png` (same probe, same 1440×900,
dpr 1) plus W3b's measured DOM geometry. Whoever recycles 6013 should re-take it.

1. **Appearance.** React boots LIGHT (cream `--background`); the wgpu shell boots DARK. The frame
   Worker realm owns no `window`, so `prefers_dark_scheme()` falls through to `true` and the shell is
   dark whatever the host prefers. This is the single largest visual difference and it is a boot-input
   gap, not a paint gap.
2. **No boot tour.** React paints a 5-step `Welcome to Puzzle 3D` overlay (Skip / `1 / 5` / Next) over
   a blurred backdrop. wgpu paints none.
3. **No 3D content and no reference underlay.** React shows the plan underlay and the model; both wgpu
   panes show only the world grid and axes. That is W3a §9's P0 (the URL half of the World3d mesh lane)
   and W3d's packet, not this one.
4. **The footer utility rail has no React counterpart** (`Transform · Brush · Volume Brush · Relocate`)
   and there is still no `Remote: detached` sync pill and no presence pill — W3b §3c, unchanged here.
5. **Window cap chips.** wgpu's cap now reads `Puzzle 3D` with Focus/Close/grip on a frosted band,
   which is React's shape; W3b §5's per-pane overlay rows (`Actions` / `Search` / `Window Options` /
   `Utilities` / `Projection`) are still absent from the wgpu panes.

## 9 — Hand-offs

1. **P1 — a glass region costs a third full walk of the command pages.** `Commands`, `GlassCommands`
   and now `ForegroundCommands` each scan all 13 770 pages (~130 ms each in a debug wasm build) to
   find the handful that matter. The honest fix is for `advance_pipeline` to measure the glass section
   and the glass-foreground layers as their own addressable spans, so each phase walks only its own.
2. **P1 — `dumpFrameStats` still cannot see the shell chrome.** It reports the largest-viewport
   window, which on every playground is an app pane. A black shell and a painted shell answer the same
   numbers. A `dumpFrameStats` that also carried the SHELL's own draw list would have made §1 a
   one-minute diagnosis.
3. **P2 — the same task-scoped-texture rule applies natively but harmlessly.** On a native surface the
   `SurfaceTexture` stays valid until `present()`, so the old ladder was correct there; the new one is
   correct on both. Nothing to do, but worth knowing why this only ever bit in a browser.
4. **P2 — `frame world resource admission exceeded fixed credits` still fires occasionally**
   (`🗑️generated/w3c-jiggle-1/run-8`, t=17.2 s) and quarantines the surface. That is W3a §9's P1 and it
   is a different failure from the black canvas — it announces itself loudly on the DOM overlay.
5. **P2 — two pre-existing P5d policy literals have drifted** (`default_now_ms() - started > 2`,
   `step_budget_ms: 1`); the law has been red since before this packet and nothing enforces the GPU
   opportunity ceiling at the policy level until they are re-pointed at
   `admit_prepared_gpu_opportunity`.
