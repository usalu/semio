# Renderer engine fixes — three boot #3 defects

Lane: React renderer engine (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/**`).
Input: `📓️runtime-verification-2026-09-09.md` boot #3 (served procedural 3d, hidden tab, 1440×900).
All line numbers are post-fix unless stated otherwise.

## Summary

| # | Defect | Root cause | Fix |
|---|---|---|---|
| 1 | node-graph canvases stuck at 300×150 in a 966×836 window | nothing sized them outside a wasm-attach continuation / an `requestAnimationFrame` poll, and a hidden tab never ticks rAF | container-owned, synchronous `ResizeObserver` + initial measure in `FlowGraphCanvasHost`; rAF layout wait in the shared `GraphWasmCanvas` replaced by a `ResizeObserver` |
| 2 | 2× `SemioFaultError: window kind procedural-main does not own action ""` per boot | `wireEffectToFriendly` read the WIT `*-params` records **flatly**, so `dispatch-action`'s `action` decoded as `""` — the plugin's `flowEvalTick` self re-dispatch lost its id | decode the nested `params` record; drop an id-less `dispatch-action` at the boundary; typed `renderer.invocation.unaddressed` fault before anything leaves the renderer |
| 3 | React `Cannot update a component (UiNodeView) while rendering a different component (FrameworkOsShellInner)` | `builtNodeStoreFor(...)` called `UiDocumentStore.loadSnapshot` from the shell's JSX, notifying mounted `UiNodeView` subscribers mid-render; `pendingWindowUiNode()` allocates a fresh node per call, so it fired on *every* render of a not-yet-refreshed window | store cache defers every reload to a `useLayoutEffect` flush; the placeholder body is now one reference-stable module constant |

## 1 — Node-graph canvas sizing

### Root cause

The flow window `#framework.window.proceduralMain` renders `FlowGraphCanvasHost`
(`🧱️elements/🕸️NodeGraph/🟦️.tsx:2534-2535`, `<canvas className="absolute inset-0 block h-full w-full">` +
the `pointer-events-none absolute inset-0 z-40` overlay — the exact pair the boot report measured).

Before the fix neither canvas had an owner that could size it without the wasm session:

- the GPU canvas was sized only inside `session.attachCanvas(...).then(...)` → `session.setSize(...)`
  (old `🕸️NodeGraph/🟦️.tsx:2138-2146`), i.e. never if the surface attach does not settle;
- the label canvas was sized only from `paintDagLabelOverlays`, reached only from `paintOverlays`,
  which returns early while `sessionRef.current` is null (`🕸️NodeGraph/🟦️.tsx:2012-2015`).

The shared surface used by the *other* node-graph component (`WasmGraphSurface` →
`GraphWasmCanvas`) had the same class of defect for a different reason: its first attach was gated on
an rAF poll (`♾️infinite/🖼️canvas/🎨️react-renderer/🟦️.tsx`, old `waitForLayout`,
`waitRaf = requestAnimationFrame(waitForLayout)`, 120-frame cap). `document.hidden === true` ⇒ no frame
callbacks ⇒ `attach` never ran ⇒ `canvas.width/height` kept the HTML default 300×150.

### Fix

`🕸️NodeGraph/🟦️.tsx:1538-1553` — new exported, pure `resizeCanvasBackingStore(canvas, logicalW, logicalH, dpr)`
(device-pixel store + CSS size, returns whether the store changed). `paintDagLabelOverlays` now calls it
instead of duplicating the arithmetic.

`🕸️NodeGraph/🟦️.tsx:2061-2093` — `syncSurfaceSize()` + a mount effect that measures the container once
synchronously and then follows it with a `ResizeObserver`; it owns **both** canvases and forwards the
logical size to the wasm surface only when one is already attached:

```tsx
const syncSurfaceSize = useCallback(() => {
  const container = containerRef.current;
  if (!container) return;
  const rect = container.getBoundingClientRect();
  const dpr = globalThis.devicePixelRatio || 1;
  resizeCanvasBackingStore(gpuCanvasRef.current, rect.width, rect.height, dpr);
  resizeCanvasBackingStore(labelCanvasRef.current, rect.width, rect.height, dpr);
  setContainerSize((prev) => (prev.w === rect.width && prev.h === rect.height ? prev : { w: rect.width, h: rect.height }));
  const session = sessionRef.current;
  if (session && surfaceReadyRef.current) observeFlowTask(session, "setSize", session.setSize(...));
}, []);

useEffect(() => {
  const container = containerRef.current;
  if (!container) return;
  syncSurfaceSize();
  const observer = new ResizeObserver(() => { syncSurfaceSize(); renderFlow(); paintOverlays(); });
  observer.observe(container);
  return () => observer.disconnect();
}, [paintOverlays, renderFlow, syncSurfaceSize]);
```

The attach continuation (`🕸️NodeGraph/🟦️.tsx:2166-2174`) no longer owns a second `ResizeObserver`; it just
calls `syncSurfaceSize()/renderFlow()/paintOverlays()` once. This is the same ownership the World3d host
already has through R3F's own measure-and-observe canvas.

`♾️infinite/🖼️canvas/🎨️react-renderer/🟦️.tsx:187-210 (constant at :62)` — the rAF layout poll is gone:

```ts
const attachWhenLaidOut = (): boolean => {
  if (torndown) return true;
  const rect = container.getBoundingClientRect();
  if (rect.width < 8 || rect.height < 8) return false;
  attach(Math.round(rect.width), Math.round(rect.height), globalThis.devicePixelRatio || 1);
  return true;
};
if (!attachWhenLaidOut()) {
  layoutRo = new ResizeObserver(() => { if (attachWhenLaidOut()) { layoutRo?.disconnect(); layoutRo = null; } });
  layoutRo.observe(container);
  degenerateTimer = setTimeout(/* attach at whatever it reports */, DEGENERATE_LAYOUT_ATTACH_MS);
}
```

`DEGENERATE_LAYOUT_ATTACH_MS = 2000` replaces the old 120-frame cap for a pane that never reaches a real
size (a `setTimeout` still fires in a hidden tab, an animation frame does not).

### Test

`🧪️tests/🔬️engine-contract/🟦️.ts` → `describe("node-graph surface sizing")`:

- *adopts its container's size on mount without any animation frame* — mounts `FlowGraphCanvasHost` with
  `getBoundingClientRect` → 966×836, `devicePixelRatio` 2, `requestAnimationFrame` stubbed to a callback
  that never fires, and `createFlowSession` stubbed to a promise that never settles. Both canvases must
  read `1932×1672` / `966px×836px`.
- *keeps the exact device-pixel store the shared helper computes* — the pure helper, incl. the
  300×150 default, the idempotent second call, the ≥1 floor and the null canvas.

Regression proof (`🗑️generated/renderer-sizing-regression-probe.txt`): with the new effect disabled the
first test fails with `expected [ 300, 150 ] to deeply equal [ 1932, 1672 ]` — i.e. it reproduces the
observed defect exactly.

## 2 — Empty action id (`does not own action ""`)

### Root cause

Chain: plugin `Effect::DispatchAction` → `⚛️reactor/🦀️.rs:1614`

```rust
Effect::DispatchAction { req, action, args, delay_ms } =>
  wit::Effect::DispatchAction(wit_effects::DispatchActionEffect { req: req.0, params: wit_effects::DispatchActionParams { action, args: …, delay_ms } }),
```

so the wire value is `{ tag: "dispatch-action", val: { req, params: { action, args, delayMs } } }`
(`🔌️plugin/🧬️schema/📜️.wit:349-358`; every `request-id`-carrying effect nests a `*-params` record —
`wireExtensionInvocation` already reads that shape).

`🔌️PluginRuntime/🟦️.tsx` `wireEffectToFriendly` read those records **flatly** (`str("action")`,
`num("delayMs")`, `str("kind")`, `str("dialogId")`, `str("pluginId")`…), so `dispatchAction.action`
decoded as `""`. That empty id flows through `ShellHost`'s `"dispatchAction" in effect` branch
(`🏛️ShellHost/🟦️.tsx:4702-4713`) → `makeEffectDispatchOne` → `encodeEffectActionInvocation`
(`🛠️ShellHelpers/🟦️.tsx:465-478`, `actionId: action`) → `handleAction` → the plugin, which answers with
`plugin_sdk_fault("window kind {} does not own action {}")` (`🔌️plugin/🦀️.rs:24902`) — an unhandled
rejection, twice per boot.

**Cross-lane consequence:** the dropped id belonged to `flowEvalTick`, the procedural editor's own
evaluation pump (`✏️s/🔌️plugins/🌀️procedural/…/✏️editor/🦀️.rs:1422`,
`🎮️commands/⏱️flow-eval-tick/🦀️.rs:18`, `🎮️commands/✅️flow-eval-resolve/🦀️.rs:19`,
`🎮️commands/🔺️flow-tessellate-resolve/🦀️.rs:21`). Every self re-dispatch that continues the evaluation
round trip was arriving unaddressed and faulting — the most likely reason boot #3's
`extrusion-axis` stayed `computing` and `extrude`/`profile` stayed `queued`. The flow-eval lane should
re-measure after this fix.

Same flat-read bug also affected `open-window`, `open-dialog` and `spawn-plugin-instance` (all silently
degraded to empty ids/kinds).

### Fix

`🔌️PluginRuntime/🟦️.tsx:791-806` — nested accessors (`paramStr`/`paramNum`/`paramText`/`paramPack`) and
`case "open-window" | "dispatch-action" | "open-dialog" | "spawn-plugin-instance"` switched onto them.

`🔌️PluginRuntime/🟦️.tsx:834-843` — an id-less `dispatch-action` is now dropped at the decoding boundary
with a `[DEBUG]` warning (same honest-drop policy as the `default:` arm) instead of being dispatched.

`🔌️PluginRuntime/🟦️.tsx:2229-2258` — `assertAddressedInvocation(invocation, kind, instanceId)`, called
from `performInvocation` before the frame is sent, throws a typed
`SemioFaultError({ origin: "renderer", code: "renderer.invocation.unaddressed", … })` whose message names
the window kind and whose scope names the plugin/app/instance — instead of the anonymous
`plugin.internal` "does not own action \"\"" coming back from the guest.

### Test

`🧪️tests/🔌️plugin-runtime/🟦️.tsx` → `describe("host effect address decoding")`, driven by the new fixture
`🧱️elements/🔌️PluginRuntime/🧫️fixtures/🎯️host-effect-address.json`:

- every request-carrying effect is read out of its nested `params` record;
- the pre-fix flat shape **and** an explicitly empty id both decode to `null` with a warning;
- `assertAddressedInvocation` throws the typed fault (code/origin/message/scope asserted) for an empty
  `actionId` and an empty `commandId`, and passes an addressed invocation through.

One pre-existing test (`keeps the command reply when publication supplies only an unsolicited UI scope`)
passed a bare `{}` as the invocation; it now carries a real address, which is what a production
invocation always has.

### Not fixed (flagged)

- `wireEffectToFriendly` has no `request-file-open` / `request-media-frames` case, so those effects are
  dropped by the `default:` arm on the browser path even though `ShellHost` implements them. Separate
  defect, out of this lane's three.
- `🎭️actor/📦️packages/🟦️typescript/🖼️wire-turn.ts:189-223` holds a second copy of `wireEffectToFriendly`
  with the same flat reads for `open-window`/`spawn-plugin-instance`. It is owned by the actor module and
  is not on the renderer's live path; consolidating the two (they differ in their pack decoder) is the
  clean long-term move and needs that lane.

## 3 — `setState` during render (UiNodeView / FrameworkOsShellInner)

### Root cause

`🏛️ShellHost/🟦️.tsx` called `builtNodeStoreFor(key, node)` **inline in the shell's JSX** (now `:8746`, `:8782`, `:8828`), and the old implementation did `store.loadSnapshot(builtNodeToSnapshot(key, node))`
whenever the node reference changed. `UiDocumentStore.loadSnapshot` → `notifyDiff` → the per-node
listeners that `useUiNode`'s `useSyncExternalStore` registered for every mounted `UiNodeView`
(`📃️UiDocumentStore/🟦️.tsx:442-471`, `🗣️Interpreter/🟦️.tsx:1365-1368`) — a store update raised from inside
another component's render, which React 19 reports verbatim as the observed warning.

It fired on essentially every render because the fallback body was `pendingWindowUiNode()`
(`🖥️platform/🟦️.ts:120`), which allocates a **fresh object per call**: any window without a refreshed
body looked like a changed authored node on every pass.

### Fix

`🏛️ShellHost/🟦️.tsx:1689-1716` — `createBuiltNodeStoreCacheV1()` (exported, so it is testable without the
shell): a first load for an unknown key still happens inline (a store nobody has subscribed to cannot
notify anyone); every later reload is queued and applied by `flushPendingReloads()`.

`🏛️ShellHost/🟦️.tsx:1960-1965` — the shell holds the cache in a ref and drains it from
`useLayoutEffect(() => builtNodeStoreCacheRef.current.flushPendingReloads())`: post-commit, pre-paint, so
nothing renders stale and no subscriber is updated during another component's render.

`🏛️ShellHost/🟦️.tsx:1672` — `const PENDING_WINDOW_UI_NODE: BuiltNode = pendingWindowUiNode()`, used by
the two JSX call sites, so a pending window no longer looks changed on every render.

### Test

`🧪️tests/🔬️engine-contract/🟦️.ts` → `describe("built-node store reloads")`:

- *loads a new key inline and defers every later reload to the flush* — first `storeFor` yields a loaded
  store; a changed node returns the SAME store, notifies nobody, and shows up in `pendingReloadKeys()`;
  only `flushPendingReloads()` swaps the content (`"first"` → `"second"`) and notifies; a repeat call with
  the settled node queues nothing.
- *never updates a subscribed UiNodeView while another component renders* — renders `InterpretedUiNode`
  over the cache with the shell's layout-effect flush, re-renders with a new node, and asserts no
  `console.error` containing "while rendering a different component". The same test carries a **control**
  that does the pre-fix thing (`loadSnapshot` in the render body of an already-subscribed store) and
  asserts the warning DOES appear — so the guard cannot silently become vacuous.

## Verification

| Command | Result | Log |
|---|---|---|
| `bun nx run @semio-tech/framework-renderer-react:test-long` | **20 files / 764 tests passed** (10.9 s) | `🗑️generated/renderer-test-long.txt` |
| `bun nx run @semio-tech/infinite-canvas-react-renderer:test` | 1 file / 1 test passed | `🗑️generated/renderer-infinite-canvas-test.txt` |
| `bun nx run @semio-tech/framework-os-dev:build-inputs` | ok, `98329f2dc7f6127ecde57686cb6b3ce000c57c87befe59fad9bdf29208634b5c` | `🗑️generated/renderer-build-inputs.txt` |
| `bun nx run @semio-tech/framework-renderer-react:typecheck` | 820 errors, **identical to the pre-change baseline**, none in the touched files (the one hit, `react-renderer/🟦️.tsx:245 import.meta.dir`, is pre-existing and untouched) | `🗑️generated/renderer-typecheck.txt` |
| sizing regression probe (fix disabled) | `expected [ 300, 150 ] to deeply equal [ 1932, 1672 ]` | `🗑️generated/renderer-sizing-regression-probe.txt` |

Not verified in a browser: the coordinator owns the pane and no cargo restage was run from this lane —
these are TypeScript-only changes and land at the next `activate`/serve restart.

## Files changed

| File | Change |
|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🟦️.tsx` | `resizeCanvasBackingStore`; container-owned `syncSurfaceSize` + `ResizeObserver`; attach continuation no longer owns sizing |
| `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🎨️react-renderer/🟦️.tsx` | rAF layout poll → `ResizeObserver` + `DEGENERATE_LAYOUT_ATTACH_MS` timeout |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx` | nested `*-params` decoding; id-less `dispatch-action` drop; `assertAddressedInvocation` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx` | `createBuiltNodeStoreCacheV1` + layout-effect flush; stable `PENDING_WINDOW_UI_NODE` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx` | re-exports `resizeCanvasBackingStore` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🧫️fixtures/🎯️host-effect-address.json` | **new** effect-address fixture |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts` | `node-graph surface sizing`, `built-node store reloads` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔌️plugin-runtime/🟦️.tsx` | `host effect address decoding`; addressed the one bare-`{}` invocation stub |
