# Node-graph attach — why the flow window never drew (boot #4)

Lane: node-graph host + flow browser host
(`📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph`, `🌊️flow/🕸️wasm/📦️packages/🟨️javascript`).
Input: `📓️runtime-verification-2026-09-09.md` boot #4 — flow window canvases 966×836, both 2D, **0 painted
pixels**, no attach/engine lines in the console.
Continues `📓️renderer-fixes-2026-09-10.md` (canvas sizing, `wireEffectToFriendly`, setState-in-render).
All line numbers post-fix.

## Root cause

`attachFlowSurface` **hard-required a WebGPU device** before it would report the surface `created`
(`🌊️flow/🕸️wasm/📦️packages/🟨️javascript/🖥️flow-host.js`, pre-fix `:273-275`):

```js
const adapter = await gpu?.requestAdapter?.();          // gpu = globalThis.navigator?.gpu
if (!adapter) throw new Error("Flow GPU adapter unavailable");
const device = await adapter.requestDevice();
```

A flow surface **presents through a plain 2D canvas context** — `renderFlowSurface` →
`renderFlowCanvas` (`🖥️flow-host.js:445`, `canvas.getContext("2d")`); the `device` the attach acquired is
returned in the attachment record and **never read by anyone**. So the whole node-graph render path was
gated on a resource it does not use. In the Claude pane (and in Firefox, in Safari, on a blocklisted
adapter, in any headless Chrome without `--enable-unsafe-webgpu`) `navigator.gpu` is absent, the attach
rejected, and in `FlowGraphCanvasHost` that rejection landed in an **anonymous swallow**:

- `🕸️NodeGraph/🟦️.tsx:2196` (pre-fix) `.catch(() => { /* already attached … */ })`
- so `surfaceReadyRef.current` stayed `false` (`:2201` only ever clears it),
- so `renderFlow()` returned at its first line for the rest of the window's life
  (`🕸️NodeGraph/🟦️.tsx:2054-2062`, `if (… || !flowSurfaceRenderAllowed(surfaceReadyRef.current)) return;`),
- with **nothing in the console** — which is exactly what the boot report measured: correctly sized
  canvases, a live session, a scene that reached the host, and zero painted pixels.

`getContext('2d')` succeeding on the "GPU" canvas was **not** evidence of a missing WebGL/WebGPU context:
`attachFlowSurface` never creates a canvas context at all, and the only context the flow surface ever
creates is the 2D one. The canvas was contextless because nothing ever presented into it.

### Second, compounding defect (would have kept the window blank even after a successful attach)

`renderFlowCanvas` re-derived the backing store from the frame, falling back to `clientWidth`
(pre-fix `:453-456`):

```js
const width = Math.max(1, state?.width ?? canvas.clientWidth ?? 1);
canvas.width = Math.round(width * dpr);
```

`clientWidth` is `0` for a canvas in a hidden tab or an unlaid-out pane, so **any frame that does not
carry its own size collapses the 966×836 store the host just measured to 1×1** — undoing the sizing fix
from `📓️renderer-fixes-2026-09-10.md` on the first paint. Reproduced live in the new DOM test: before
this repair its marker read `[DEBUG] node-graph first draw surface=procedural.main store=1x1`.

### Answers to the four checks

| check | finding |
|---|---|
| (a) rAF / IntersectionObserver / visibility gate in the attach chain | **None left.** Zero `IntersectionObserver` / `document.hidden` / `visibilitychange` sites in `🧑‍🎨engine/` and `♾️infinite/` outside tests. The flow host's own message pump is `setTimeout`-driven (`🖥️flow-host.js:25`, `schedule = (step) => setTimeout(step, 0)`), which ticks while hidden. The two rAF users are **post**-attach repaint loops, now hidden-tab-safe (below). |
| (b) engine wasm module staged, init succeeds | **Yes.** `@semio-tech/flow-core` → `node_modules/@semio-tech/flow-core` symlink → `🌊️flow/🫀️core/🕸️bindings/` (all files 2026-09-10 02:23, `flow_core_bg.wasm` 43 MB). `flow_core.js` exports `flow_bridge_*` on the raw instance, so `createFlowBrowserRuntime` takes the `source.exports` fast path and does **not** re-initialize (the report's "wasm fetched twice" is not a double init). `🖥️flow-host.js` in the bindings dir is a byte-identical copy of the source module. |
| (c) scene → engine feed | **It happens, and always did** — `syncFlowSessionFromScene` is called from the scene effect (`🕸️NodeGraph/🟦️.tsx:2213-2224`) and is *not* gated on `surfaceReady`; `synchronizeDocumentJson` carries `scene.fixtureJson`. Asserted in the new DOM test (op `2610` observed on the bridge). |
| (d) label overlay paints | It was reached (`paintOverlays` needs only a session), but the same collapse applies: `paintDagLabelOverlays` sizes the label canvas from the container, so it painted into a correctly sized store — the window still read blank because the **GPU** canvas, the one carrying the nodes, never painted. Asserted in the new DOM test (op `2574` + a recorded `clearRect` on the label canvas). |

## Fix

| file | change |
|---|---|
| `🌊️flow/🕸️wasm/📦️packages/🟨️javascript/🖥️flow-host.js:260-273` | new `requestFlowSurfaceDevice(gpu)` — acquires the adapter/device **optionally**, returns `null` instead of throwing. `attachFlowSurface:287` uses it; only a cancellation aborts the attach now. `device?.lost?.…` handles the null case. |
| `…/🖥️flow-host.js:449-461` | `renderFlowCanvas` only resizes when the frame carries a positive `width`/`height`; otherwise it presents into the store its host already owns (`positiveFlowSize` helper, `:445`). |
| `🌊️flow/🫀️core/🕸️bindings/🖥️flow-host.js` | the staged copy, refreshed by hand exactly as the `wasm` target's `copyFileSync` does (`🌊️flow/🫀️core/📦️packages/🦀️rust/📜️script.ts:78`) — **no cargo build was run**. `@semio-tech/flow-core` is in `FRAMEWORK_ENGINE_OPTIMIZE_DEPS_EXCLUDE` (`🧑‍💻dev/📦️packages/🟦️typescript/⚙️vite.config.ts:50`), so vite serves it as source and a **page reload is enough**. |
| `🕸️NodeGraph/🟦️.tsx:2196-2203` | the anonymous attach swallow is replaced by a `[DEBUG]` warning naming the surface and the failure. |
| `♾️infinite/🖼️canvas/🎨️react-renderer/🟦️.tsx:64-79` | new shared `scheduleDemandFrame(tick)` + `HIDDEN_DEMAND_FRAME_MS = 32`: falls back to `setTimeout` when `document.hidden` or no `requestAnimationFrame` exists. Placed in the infinite-canvas leaf (next to its sibling `DEGENERATE_LAYOUT_ATTACH_MS`) so both rAF users share one copy. |
| `♾️infinite/🖼️canvas/🎨️react-renderer/🟦️.tsx:182-186, 241` | `GraphWasmCanvas`'s post-attach frame loop uses it (it was an unconditional `requestAnimationFrame` loop that simply stopped in a hidden tab); removed the already-dead `rafRef`. |
| `🪪️WasmSessionLoader/🟦️.tsx:56-96` | `createDemandFrameScheduler` drives every slot through `scheduleDemandFrame`, so an `invalidate()` in a hidden tab still repaints. |

### Not fixed (flagged)

- `🌊️flow/🖥️host/🧹️retirement/🧪️tests/🧪️source-contract/🟦️.ts:36` asserts that
  `♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs` contains `assert!(turns > 1_600)`; it no longer does,
  so `semio-framework-os-flow-core:test-source` fails on a **Rust-source** assertion unrelated to this
  lane. Its broken *import* (below) is repaired; the assertion belongs to whoever changed that `.rs`.
- The flow node-graph's production painter is still the placeholder `renderFlowCanvas` — one grey
  `fillRect(x-60, y-24, 120, 48)` per widget (`🖥️flow-host.js:462-466`). A correct attach now yields seven
  boxes, not a laid-out node graph. Separate lane.

### Repairs of pre-existing breakage found on the way (both blocked this lane's gates)

- `🌊️flow/🕸️wasm/🧪️tests/🖥️host/🟨️.js:5` and `🌊️flow/🖥️host/🧹️retirement/🧪️tests/🧪️source-contract/🟦️.ts:5`
  imported `flowWasmContract` from `🕸️wasm/📦️packages/🟨️javascript/📜️script.ts`, deleted by the
  fixtures-separation lane in `ebbace9b32` (2026-09-09). Repointed at its new owner,
  `🌊️flow/🧪️tests/🌐️browser-declaration/🟦️.ts`. Without this, `test-browser`/`test-source` did not even load.
- `🔬️engine-contract/🟦️.ts:1543` — a peer added two top-level `test(...)` blocks (`:9008`, `:9014`) without
  importing `test`; the whole suite failed to load with `ReferenceError: test is not defined`. Added
  `test` to the existing vitest import.

## `[DEBUG]` markers for the next boot

In order, one line each per node-graph window (`surface=procedural.main` for the flow window):

```
[DEBUG] node-graph host mount surface=procedural.main hidden=true
[DEBUG] node-graph session ready surface=procedural.main
[DEBUG] node-graph attach called surface=procedural.main 966x836 dpr=1
[DEBUG] flow surface created surface=1 966x836 dpr=1 device=none      ← device=none is now OK, not a failure
[DEBUG] node-graph surface ready surface=procedural.main nodes=7 edges=6
[DEBUG] flow surface context created 2d 966x836 dpr=1 widgets=7
[DEBUG] node-graph first draw surface=procedural.main store=966x836
```

Read them like this:

| stops after | meaning |
|---|---|
| `host mount` only | `createFlowSession()` never resolved — look for `[DEBUG] node-graph session failed …` (new) or a failed `flow_core_bg.wasm` fetch. |
| `session ready`, no `attach called` | the container never mounted or `sessionReady` never flipped. |
| `attach called`, no `flow surface created` | the reactor rejected `attachSurface`, or the attach was cancelled — look for `[DEBUG] node-graph attach failed surface=… <message>` (new; this line **is** the defect this report fixes, and must no longer appear). |
| `surface ready` but no `first draw` | `renderCanvas` faults inside the engine (`observeFlowTask` still swallows those). |
| `first draw … store=1x1` | the size-collapse defect is back. |
| `widgets=0` in the context line | the fixture never reached the session — a scene-feed problem, not a renderer one. |

`device=none` is expected in the Claude pane and is no longer a failure. All markers are `console.log`
except the two failure lines, which are `console.warn`.

## Test

New, and it reproduces the reported condition exactly:
`🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts` → `describe("node-graph surface attachment in a hidden tab")`
→ *attaches, feeds the scene and draws without any animation frame*.

It mounts `FlowGraphCanvasHost` in jsdom with

- `document.hidden === true` and `requestAnimationFrame` stubbed to a callback that **never fires**
  (asserted: `expect(frame).not.toHaveBeenCalled()`),
- `navigator.gpu === undefined` (asserted — jsdom ships none, exactly like the pane),
- a **real** `createFlowBrowserRuntime` session over `MockFlowBridge`, so the attach runs through the
  production `attachFlowSurface`/`renderFlowSurface`, not a hand-written double,
- the served `hexagonal-mushroom-column` shape: 7 nodes, 6 edges, a 7-widget `fixtureJson`,
- a recording 2D context (jsdom implements none), so a paint is *witnessed*, not assumed.

Asserted: bridge ops `attachSurface`(2575), `surfaceStatus`(2576), `synchronizeDocumentJson`(2610),
`setCamera`(2565), `setSize`(2578), `renderFrame`(2581), `labelOverlayPaintStateJson`(2574); a
`clearRect` recorded on **both** canvases; both stores still `966×836`; the GPU canvas's clear is exactly
`clearRect(0,0,966,836)`. Operation ids come from the owned ABI (`🕸️wasm/🧬️schema/📡️abi.json`), not a
hand-typed number.

Independent engine-level oracle (`🌊️flow/🕸️wasm/🧪️tests/🖥️host/🟨️.js:165-180`): `attachFlowSurface` reaches
`created` with `device === null` on **four** deviceless hosts — absent `navigator.gpu`, adapter `null`,
device `null`, and a throwing `requestAdapter`. The pre-existing cancellation law
(`cancel-gpu-terminal`) still holds, so the fix did not weaken cancellation.

### Regression proof

| probe | result | log |
|---|---|---|
| pre-fix vs post-fix `attachFlowSurface`, four deviceless hosts (HEAD copy vs working tree, same MockFlowBridge) | pre-fix: 4/4 `REJECTED` (`Flow GPU adapter unavailable` ×2, `null is not an object (evaluating 'device.lost')`, `blocklisted`); post-fix: 4/4 `ATTACHED device=null` | `🗑️generated/ng-deviceless-attach-probe.txt` |
| the new DOM test with the pre-fix gate restored | **fails**: `expected [ 2500, 2575, 2610, 2574, 2518, …(9) ] to include 2581` — i.e. `renderFrame` is never issued — plus `[DEBUG] node-graph attach failed surface=procedural.main Flow GPU adapter unavailable` | `🗑️generated/ng-attach-regression-probe.txt` |

## Verification

| command | result | log |
|---|---|---|
| `bun nx run @semio-tech/framework-renderer-react:test-long` | **20 files / 768 tests passed** | `🗑️generated/ng-renderer-test-long.txt` |
| `bun nx run semio-framework-os-flow-core:test-browser` | passed (incl. the four deviceless laws) | `🗑️generated/ng-flow-test-browser.txt` |
| `bun nx run @semio-tech/infinite-canvas-react-renderer:test` | 1 file / 1 test passed | `🗑️generated/ng-infinite-canvas-test.txt` |
| `bun nx run @semio-tech/framework-renderer-react:typecheck` | **820 errors — identical to the pre-change baseline**; the only hit in a touched file is the documented pre-existing `react-renderer/🟦️.tsx:261 import.meta.dir` (line moved by +16 added lines, untouched) | `🗑️generated/ng-renderer-typecheck.txt` |
| `bun nx run semio-framework-os-flow-core:test-source` | **fails on a pre-existing Rust-source assertion** (see "Not fixed"); its import breakage is repaired | `🗑️generated/ng-flow-test-source.txt` |

No browser verification from this lane — the coordinator owns the pane. These are JS/TS-only changes on
a vite-excluded package, so a **page reload** is sufficient; no restage, no cargo build was run.

## Files changed

| file | change |
|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/📦️packages/🟨️javascript/🖥️flow-host.js` | optional GPU device; size-preserving `renderFlowCanvas`; two `[DEBUG]` markers |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/🕸️bindings/🖥️flow-host.js` | staged copy refreshed (byte-identical to the source module) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🟦️.tsx` | attach-rejection is reported instead of swallowed; mount/session/attach/first-draw `[DEBUG]` markers |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🪪️WasmSessionLoader/🟦️.tsx` | `createDemandFrameScheduler` drives `scheduleDemandFrame` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🎨️react-renderer/🟦️.tsx` | **new** shared `scheduleDemandFrame`/`HIDDEN_DEMAND_FRAME_MS`; `GraphWasmCanvas` loop uses it; dead `rafRef` removed |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts` | **new** `node-graph surface attachment in a hidden tab`; missing `test` import repaired |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🧪️tests/🖥️host/🟨️.js` | **new** four deviceless attachment laws; broken `flowWasmContract` import repaired |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🧹️retirement/🧪️tests/🧪️source-contract/🟦️.ts` | broken `flowWasmContract` import repaired |
