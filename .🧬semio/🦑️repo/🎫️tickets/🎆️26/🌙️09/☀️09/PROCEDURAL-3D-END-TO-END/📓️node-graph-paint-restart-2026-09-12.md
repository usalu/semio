# Flow node-graph canvas — the paint path after the 6018 restart (2026-09-12)

Closes the defect the coordinator measured once the restarted serve finally handed the page the fresh
`flow-core`: the node-graph canvas went **blank** — `canvas 0 context: "not-2d"`, 311 ink pixels on the
label overlay only. The GPU half engaged and *poisoned the element it could not present on*.

Lane: the flow node-graph paint path only (`render_frame` present decision, `🖥️flow-host.js`,
`replayFlowDrawList`/`renderFlowCanvas`, the React node-graph host attach). **No dev-server restart, no
flow-core wasm build.** The host JavaScript was restaged into `🫀️core/🕸️bindings/` — the copy
`…:wasm` makes itself — and vite invalidated it on the running 6018, which is how the fix below is
measured live. See §5 for exactly what does and does not need a restart.

---

## 0. Headline

| | before (coordinator-1, after the restart) | after |
|---|---|---|
| scene canvas, no WebGPU flags | `context: "not-2d"` — **0 readable ink**, the window blank but for three label strings | clear `221,218,205,255`, **393 022 ink px / 810 colours / 0 pure-black px**, ink bounds the full 483×814 |
| label canvas | 311 px | 311 px (unchanged) |
| scene canvas, `--enable-unsafe-webgpu …` | `present=webgpu`, painted | `present=webgpu`, painted (unchanged) |
| console | `No available adapters.` → `present=2d` → silence | no adapter is ever asked to bind a canvas |

`🗑️generated/canvas-paint/restart/{before,after}-{no-webgpu,webgpu}{,-checked}/`.

---

## 1. Root cause — two independent measurements, one mechanism

### 1.1 Which build is live

`🐍️flow-module-freshness-probe.mjs` against the running 6018, marker `flowAttachSurfaceCanvas`:

```
200 625620B marker=true  …/node_modules/@semio-tech/flow-core/flow_core.js   ← the FRESH wasm-bindgen glue
200 203197B marker=true  …/🌊️flow/🫀️core/🕸️bindings/🖥️flow-host.js          ← the fresh host
```

So the prior lane's honest limit (§4.1 of `📓️node-graph-canvas-paint-2026-09-12.md`) is gone: after the
restart the GPU half is really reachable from the page. That is what turned a working 2D fallback into a
blank canvas.

### 1.2 The mechanism, from the live console

`🗑️generated/canvas-paint/restart/before-no-webgpu/console.txt`, no WebGPU flags:

```
5448  [DEBUG] node-graph attach called surface=window:procedural-main 483 814 1
6357  warning  No available adapters.                ← wgpu, INSIDE the guest
6730  [DEBUG] flow surface created … present=2d      ← the attach correctly reports "no GPU"
      (no `flow surface context created 2d …` line ever follows)
```

and the probe reads `canvas 0 → context: "not-2d"`.

Headless Chromium **exposes `navigator.gpu`** and hands out **no adapter**. The host's guard checked only
for the object's presence, so the element reached the guest; `CanvasGpuSession::create_canvas_surface`
called `RenderContext::create_surface(SurfaceTarget::Canvas(canvas), …)`, and wgpu's WebGPU backend
**binds a `webgpu` context to the element as its first act** — before `request_adapter` runs and fails.
A canvas admits exactly one context kind for its whole life, so from that moment:

- `present_surface_scene` returns `false` (no device) → the frame correctly carries `present:"2d"` and
  the full 47-command draw list,
- `renderFlowCanvas` asks that canvas for a 2D context, gets `null`, and **returned silently**,
- every later frame repeats it. Nothing is painted, nothing is logged, forever.

So all three hypotheses in the brief were wrong in detail and right in outcome: the replay still runs, the
draw list is neither empty nor mis-scaled (the WebGPU run paints the identical picture from the identical
scene), and the GPU path is not *chosen* — it is *attempted*, fails, and takes the canvas down with it.

### 1.3 The second measurement: with an adapter, everything already worked

`before-webgpu/console.txt` → `present=webgpu`, and `2-surface.png` shows grid, node bodies, ports, wires
and the minimap. The GPU half needed no repair. **Only the fallback did** — and a viewer without WebGPU is
a real user, so the fallback is not an edge case.

---

## 2. The fix — the presentation decision is made before the element is touched

Three changes, no placeholder branch, no third case.

### 2.1 An adapter, not `navigator.gpu`, decides (`🖥️flow-host.js`, `requestFlowSurfacePresentation`)

```js
if (!gpu || typeof bindings?.flowAttachSurfaceCanvas !== "function") return false;
let adapter;
try { adapter = await gpu.requestAdapter?.(); } catch { adapter = undefined; }
if (!adapter) return false;                       // the canvas is never handed over
return (await bindings.flowAttachSurfaceCanvas(surface, canvas, width, height, dpr)) === true;
```

The existing `navigator.gpu` guard stays (a wgpu bring-up panic inside the guest is an unrecoverable trap);
this adds the only evidence that actually predicts the bring-up can finish. It is also the half that works
without a wasm rebuild, which is why the live proof in §5 needed no serve restart.

### 2.2 …and the guest keeps the same order (`♾️infinite/🖼️canvas/🦀️.rs`, `create_canvas_surface`)

```rust
let mut render_ctx = util::RenderContext::new();
if render_ctx.device(None).await.is_none() {
    return Err("no WebGPU adapter".to_string());   // the canvas is still untouched
}
let surface = render_ctx.create_surface(wgpu::SurfaceTarget::Canvas(canvas), pw, ph, …).await?;
```

`RenderContext::device(None)` is vello's own adapter/device acquisition with no surface. This is the
structural half: `create_canvas_surface` is the presenter **every** browser board surface uses
(`DagSession`, `BoardSession`, the node-graph/paint/tiled-map/editor sessions, puzzle2d), and all of them
had the same latent poisoning. They already propagate the `Err` — with no adapter `create_surface` failed
anyway — so the only change for them is that it now fails *before* the element is claimed.

### 2.3 A refused 2D context is a verdict, not a silence (`🖥️flow-host.js` + `🕸️NodeGraph/🟦️.tsx`)

`renderFlowCanvas` is exported and returns `FlowPresentation`: `gpu` (the guest already presented — the
element is never touched), `replayed`, `none` (no canvas), and `unpresentable` — the frame wants the 2D
replay and the element cannot give a 2D context. `renderFlowSurface` carries the verdict on the frame
state, so the React host sees it.

`FlowGraphCanvasHost` then **replaces the element**, because that is the only recovery a canvas has:

```tsx
const [canvasGeneration, setCanvasGeneration] = useState(0);
…
<canvas key={flowSurfaceCanvasKey(surfaceId, canvasGeneration)} ref={gpuCanvasRef} … />
```

`replaceUnpresentableCanvas` cancels the retired surface first (`surfaceStatus: "cancelled"`, not
`"device-lost"` — the latter parks the surface as recoverable and keeps the session's single slot
occupied, so the successor's `attachSurface` would be refused with *"Flow surface requires exact close
before replacement"*), then bumps the generation; the attach effect depends on it, so the successor
re-attaches and the GPU-or-replay decision is **made once per canvas element and re-made on device loss**.

One bound makes this terminate: **only a canvas that was presenting on the GPU can be replaced**
(`presentsOnGpu` is kept from the attach result). An element that refuses a 2D context on a surface that
never had a device is not poisoned — it is an environment with no 2D canvas at all (jsdom, a stripped
embedder), where the successor would refuse identically. Without that gate the host remounts forever:
measured, in the first run of the React suite, as two flow-host contract tests spinning on repeating
`[DEBUG] node-graph attach called`. With it, a device loss costs exactly one replacement (the successor
re-attaches without a device, stays 2D-capable, and is never replaced again).

---

## 3. Tests — all run, all quoted

### 3.1 Rust law over the real generation3d payload (`🌊️flow/🕸️wasm/🧪️tests/🎬️draw-list/🦀️.rs`)

New row `without_a_gpu_adapter_the_frame_replays_a_list_that_covers_the_node_layout`: drives the production
`FlowDomainAdapter::render_frame` at 966×814 with no attached canvas (the native arm's
`present_surface_scene` is always `false` — the deviceless exit), then decodes every command's own shape
through its own affine (`command_screen_bounds`, the encoding's tag layout) and asserts

- `present == "2d"` and a non-empty command list,
- the replayable ink covers more than half the viewport,
- **every node the host reports as on screen (`entity_screen_json("node", id)`) has something node-sized
  drawn inside its own screen rect** — which makes it a camera law, not a "the list is long" law.

```
test wasm_session::draw_list_laws::moving_the_camera_moves_the_encoded_picture ... ok
test wasm_session::draw_list_laws::the_generation_3d_flow_window_paints_a_scene_that_survives_encoding ... ok
test wasm_session::draw_list_laws::render_frame_carries_the_draw_list_the_host_must_paint ... ok
test wasm_session::draw_list_laws::without_a_gpu_adapter_the_frame_replays_a_list_that_covers_the_node_layout ... ok
test result: ok. 4 passed; 0 failed
```

**Falsified, not assumed.** A one-line temporary `set_camera(400, 300, 1)` between the render and the
`entity_screen_json` reads — i.e. the host's camera no longer the one the list was painted with — fails it:

```
nothing node-sized was drawn inside on-screen node extrusion-axis at
[-2.263270210368887, -30.45687403531781, 40.0, 42.0] — the replay would paint an empty canvas where
the host says a node is
```

(reverted; the 4-pass run above is after the revert).

### 3.2 JavaScript twin (`🌊️flow/🕸️wasm/🧪️tests/🎬️draw-list/🟨️.js`)

Same shared fixture and the same pinned draw list. `recordedPrimitives` is the mirror of the Rust helper —
each recorded primitive's box through the transform in force when it was issued — and the camera affine is
**derived from the fixture**, not read out of the encoded list, so "the picture is at the camera's
coordinates" is not asserted against itself. Rows: the deviceless frame returns `replayed`; the replay is
non-empty; the ink spans the viewport; every fixture node has a `roundRect` at its camera-projected rect
(1e-3); plus the verdicts — a `gpu` frame never asks the element for a context, a canvas that refuses 2D
under a 2D frame is `unpresentable`, no canvas is `none`.

### 3.3 Attach laws (`🌊️flow/🕸️wasm/🧪️tests/🖥️host/🟨️.js`)

Four new hosts that expose `navigator.gpu` and hand out no adapter — `requestAdapter` returning `null`,
returning `undefined`, throwing, and absent — each asserting the guest bring-up is **never reached** and the
surface still reaches `created` with `presentsOnGpu === false`. This is the live defect as a law.

```
[DEBUG] Flow surface attachment reached created on four hosts that could not bind a WebGPU presenter
[DEBUG] Flow surface attachment kept the canvas 2D-capable on four hosts that expose navigator.gpu and hand out no adapter
[DEBUG] Flow draw-list replay reproduced every primitive of the shared fixture as its own 2D primitive
[DEBUG] Flow draw-list replay applied the camera affine and composed the device-pixel scale over it
[DEBUG] Flow draw-list replay refused every malformed list and unwound its own layer stack
[DEBUG] Flow deviceless frame replayed a non-empty list over every node of the shared fixture and named its own presentation verdict
NX   Successfully ran target test-browser for project semio-framework-os-flow-core
```

### 3.4 React contract (`🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts`)

New law *"replaces a canvas that can no longer give a 2D context and re-attaches its surface to the
successor"*: the surface first presents on the GPU (stubbed adapter + a `flowAttachSurfaceCanvas` that
succeeds once), its element is then poisoned, and the test asserts a second `attachSurface`, a **different**
canvas element at the same size, a real `clearRect` on the successor, nothing painted on the corpse, and —
the termination bound — **exactly two** attaches with the successor still in place after a settle.

```
node-graph surface attachment in a hidden tab: 3 passed
```

### 3.5 Neighbouring suites

| command | result |
|---|---|
| `bun nx run @semio-tech/framework-renderer-react:test-long` | **970 passed, 9 failed** — 6 × `extension invocation completion ownership` (a peer's cancellation lane: the SDK call now carries an `AbortSignal`), 2 × `PluginRuntime` (a peer's `ops` document-pack field), 1 × `noteShellCommand`. All nine are peer lanes or the lane's own recorded baseline; the first run of this lane had 13, and the four extra were the remount spin §2.3 names, which the `presentsOnGpu` bound fixed. |
| `bun nx run semio-framework-os-flow-core:test-browser` | passed (§3.3) |
| `cargo test -p semio-framework-os-flow --lib draw_list_laws` | 4 passed |
| `cargo test -p semio-framework-os-infinite --lib draw_list` | 5 passed |
| `cargo check -p semio-framework-os-infinite --lib --target wasm32-unknown-unknown` | 0 errors |
| `cargo check -p semio-framework-os-flow --target wasm32-unknown-unknown` | 0 errors, 1 pre-existing warning |
| `bun nx run semio-framework-os-flow-core:test-source` | **fails pre-existing** at `🧹️retirement/🧪️tests/🧪️source-contract/🟦️.ts:36` (`assert!(turns > 1_600)`), exactly as the prior lane recorded |

### 3.6 Playwright pixel statistics in the probe (`🐍️canvas-paint-probe.mjs`)

New `📏️PixelStatisticsCheck` region; the probe now **exits non-zero** when the graph is not painted. On the
2D path it requires ink ≥ 10 % of the surface, 0 pure-black pixels (the old placeholder's signature),
≥ 100 distinct colours, and an ink bounding box spanning ≥ 60 % of the canvas in both axes. On a WebGPU
canvas `getImageData` is impossible by construction, so there it requires that the guest really took the
GPU exit (`present=webgpu`) and that no frame ever reported `unpresentable`.

The 10 % bar is calibrated, not guessed: 393 022 ink px at the document's own camera and 78 702 zoomed out
on this 483×814 window (a wide-open grid is legitimately sparser, since "ink" is everything unlike the clear
colour). A first pass at 25 % failed the zoomed-out frame and was lowered — recorded here because it is the
one number in this report that is a judgement call.

**Falsified against the stored runs** (`🗑️generated/canvas-paint/restart/check-over-baselines.txt`):

```
[DEBUG] before-no-webgpu: FAILED ["settled:gpu-present-reported","zoomed-out:gpu-present-reported"]
[DEBUG] before-webgpu:            PASSED
[DEBUG] after-no-webgpu-checked:  PASSED
[DEBUG] after-webgpu-checked:     PASSED
```

The check fails exactly the defect run and passes the three that were correct.

---

## 4. Runtime — 6018, both modes, before and after

Probe `🐍️canvas-paint-probe.mjs`, 1440×900 headless, the Flow window 483×814.

| run | scene canvas (index 0) | check |
|---|---|---|
| `restart/before-no-webgpu` | `context: "not-2d"` — unreadable, blank (`2-surface.png`: three label strings on an empty ground) | **FAILED** |
| `restart/before-webgpu` | `present=webgpu`, painted | PASSED |
| `restart/after-no-webgpu-checked` | clear `221,218,205,255`, **393 022 ink px**, **810 colours**, **0 pure-black px**, ink bounds `{0,0,483,814}` | **PASSED** |
| `restart/after-webgpu-checked` | `present=webgpu` ×2 (a remount re-attached), no `unpresentable` | **PASSED** |

`after-no-webgpu/4-zoomed-out.png` is the clearest frame: LOD grid, the `Extrude` node body with its two
semicircular input ports, port labels (`! wire`, `! vector`), the `Polygon`/`Vector`/`! x ! y ! z` label
column, and the minimap with its red viewport rectangle. `after-webgpu/4-zoomed-out.png` is the same
picture off the GPU.

Interaction round-trip after the change: `interactions.json` still reports hover and select lines on the
clicked quadrants, so host-side picking is unaffected (canvas geometry, `data-surface-id` and both
`absolute inset-0` canvases are untouched).

---

## 5. Does 6018 need a restart?

**No, for everything measured above.** Two of the three changes are live on the running serve already:

- `🕸️NodeGraph/🟦️.tsx` is a watched source module,
- `🖥️flow-host.js` was restaged into `🫀️core/🕸️bindings/` (a byte-identical copy of the source, which is
  precisely the `copyFileSync` that `semio-framework-os-flow-core:wasm` performs). That path **is** watched,
  and the freshness probe proves the page received the new bytes: `200 216603B marker=true … 🖥️flow-host.js`
  while `node_modules/@semio-tech/flow-core/flow_core.js` is unchanged at `625620B`.

**Yes, for the Rust half — whenever someone next runs the flow-core wasm build.** `create_canvas_surface`
(§2.2) lives in the wasm, and `node_modules/@semio-tech/flow-core/flow_core.js` is never invalidated by
vite (`UNWATCHED_REPOSITORY_SEGMENTS`). I deliberately did **not** run `…:wasm`: it rewrites the 43 MB
artifact every peer's page is being served from, the lane is not blocked on it, and the live combination is
already correct — the JS never lets an adapterless host reach the guest, so the staged wasm's older
bind-then-probe order is unreachable. When the next `:wasm` build lands, restart 6018 and the two layers
agree.

---

## 6. Findings handed on (not this lane)

1. **Every other browser board surface had the same poisoning** and is fixed by §2.2 only once the wasm is
   rebuilt: `DagSession`, `BoardSession`, the paint, tiled-map and editor sessions, puzzle2d's editor. Until
   then, any of them on a machine with no adapter leaves its canvas unusable by anything else. None of them
   has a 2D fallback to be denied, so today they simply fail their attach — but the element is still lost.
2. **The flow window remounts and re-attaches roughly every 20–50 s** (`after-webgpu-checked` shows
   `flow surface created surface=1` at 5.2 s and `surface=2` at 54.7 s). Prior lane's finding #2, unchanged.
3. **`renderFrame` reply latency 9–40 s** on a cold boot. Prior lane's finding #1, unchanged; it is why the
   probe needs a 70 s settle and a 45 s post-zoom settle.
4. **The verdict has no static type across the boundary.** `renderCanvas` is declared `FlowTask<unknown>` in
   the generated `📝️flow-browser.d.ts`, so the React host matches the `"unpresentable"` string the way every
   other flow surface word (`"created"`, `"cancelled"`, `"device-lost"`) is matched on both sides of the byte
   ABI. If the browser-declaration generator ever grows a frame type, `FlowPresentation` belongs in it.

---

## 7. Files

| file | change |
|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🦀️.rs` | `create_canvas_surface` acquires the adapter (`RenderContext::device(None)`) **before** wgpu can bind a `webgpu` context to the element |
| `…/🌊️flow/🕸️wasm/📦️packages/🟨️javascript/🖥️flow-host.js` | `requestAdapter` pre-check in `requestFlowSurfacePresentation`; new `FlowPresentation`; `renderFlowCanvas` exported, returns its verdict and warns loudly instead of returning silently; `renderFlowSurface` carries the verdict on the frame state |
| `…/🌊️flow/🫀️core/🕸️bindings/🖥️flow-host.js` | restaged copy (identical bytes; what `…:wasm` does) so the running 6018 serves the fix |
| `…/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🟦️.tsx` | `flowSurfaceCanvasKey` + `canvasGeneration`; the attach result's `{surface, surfaceGeneration, presentsOnGpu}` is retained; `replaceUnpresentableCanvas` cancels the retired surface and mints a successor; the attach effect depends on the generation |
| `…/🌊️flow/🕸️wasm/🧪️tests/🎬️draw-list/🦀️.rs` | **new** law `without_a_gpu_adapter_…` + `command_screen_bounds` |
| `…/🌊️flow/🕸️wasm/🧪️tests/🎬️draw-list/🟨️.js` | **new** `🙈️DevicelessReplay` region — the twin, plus the three verdict rows |
| `…/🌊️flow/🕸️wasm/🧪️tests/🖥️host/🟨️.js` | **new** four adapterless attach laws |
| `…/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts` | **new** canvas-replacement law with its termination bound |
| `<ticket>/🐍️canvas-paint-probe.mjs` | **new** `📏️PixelStatisticsCheck` region; the probe now fails (exit 1) on an unpainted graph |

Evidence: `🗑️generated/canvas-paint/restart/` — `before-no-webgpu/`, `before-webgpu/`,
`after-no-webgpu/`, `after-webgpu/`, `after-no-webgpu-checked/`, `after-webgpu-checked/`
(each with `ink-*.json`, `pixel-statistics-check.json`, `interactions.json`, `console.txt`,
`1-settled.png` … `4-zoomed-out.png`), plus `check-over-baselines.txt`, `react-test-long.txt`,
`after-*-checked-run.txt`.
