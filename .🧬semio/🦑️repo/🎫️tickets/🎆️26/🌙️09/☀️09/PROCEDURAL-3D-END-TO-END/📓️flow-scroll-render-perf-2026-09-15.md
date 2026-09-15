# 🖱️ Flow scroll render perf — lane `flow-scroll-render-perf`, 2026-09-15

User directive, 13:03: **"scrolling in the flow takes seconds to render."** Literally true, and measured:
a thirty-tick wheel zoom over the Flow node graph produced **one** board repaint, with a median
tick-to-paint of **653–1 110 ms** and a p95 of **1.2–1.6 s**.

Stage: `http://127.0.0.1:6022/?plugin=generation3d`, direct vite serve (`📜️serve-generation3d-react-6022.sh`,
recycled by pid + `rm -rf node_modules/.vite-temp` before the baseline, and again at 14:53 to pick up
lane `vite-stale-transform-guard`'s config change). Every module changed here was confirmed on the wire
with a `/@fs/` fetch before each verdict. Headless Chromium with `--use-angle=metal`. Examples:
`hexagonal-mushroom-column` (7 widgets) and `sphere-cut-with-torus` (6 widgets), both booting the real
example graph (`node-graph surface ready … nodes=7 edges=6`), never the fallback.

---

## 1. Result

| | before | after |
|---|---|---|
| board repaints per 30-tick wheel zoom | **1** | **81–88** |
| median tick → paint, wheel | 653 / 1 110 ms | **5.2 / 6.1 ms** |
| p95 tick → paint, wheel | 1 189 / 1 578 ms | 7.0 / 7.1 ms |
| median move → paint, 1 s drag-pan | 165 / 1 082 ms | **4.9 / 5.4 ms** |
| guest invocations **during** a gesture | 6–17 | **0** |
| camera publications at settle | 0–1 | **exactly 1** |
| `refreshUi` passes during a gesture | 2–5 | **0** |
| React commits during a gesture | 1–4 | **0** |
| long tasks > 50 ms | 0–1 | 0 |

Probe: `🐍️flow-scroll-render-perf-probe.mjs` (30 wheel ticks + a 1 s middle-button drag-pan at ~60 Hz,
per example). Gate: **GREEN** — median ≤ 16 ms, p95 ≤ 50 ms, 0 guest hops during, 1 publication at settle.

---

## 2. Per-gesture measurements

Read with the probe's own numbers. "paints" counts the `semio.hop.surface.paint` spans the node-graph
host publishes around one `renderCanvas` present; "guest during" counts `semio.hop.invoke` spans by
`actionId` between the first and last input event of the gesture; "settle" counts them after it.

### Before — `🗑️generated/flow-scroll/before/scroll.json`, 13:1x, fleet load not recorded

| example | gesture | events | paints | median ms | p95 ms | max ms | guest during | settle | refresh d/s | commits d/s | recalc ms | script ms |
|---|---|---:|---:|---:|---:|---:|---|---|---|---|---:|---:|
| hexagonal-mushroom-column | wheel-zoom | 30 | **1** | 1110.3 | 1578.0 | 1609.9 | 6 (`nodeGraphViewport` 5, `interactionHover` 1) | `nodeGraphViewport` 1 | 4/2 | 3/3 | 42 | 624 |
| hexagonal-mushroom-column | drag-pan | 63 | **2** | 1082.2 | 1938.4 | 1994.0 | 17 (`interactionHover` 17) | — | 3/0 | 3/0 | 73 | 855 |
| sphere-cut-with-torus | wheel-zoom | 30 | **1** | 653.3 | 1188.5 | 1221.3 | 10 (`nodeGraphViewport` 9, `interactionHover` 1) | `nodeGraphViewport` 1 | 5/2 | 4/3 | 57 | 721 |
| sphere-cut-with-torus | drag-pan | 63 | 53 | 165.1 | 951.8 | 1005.2 | 16 (`interactionHover` 16) | — | 2/0 | 1/1 | 66 | 838 |

### After — `🗑️generated/flow-scroll/after-final/scroll.json`, 16:03, **load average 31.8**

| example | gesture | events | paints | median ms | p95 ms | max ms | guest during | settle | refresh d/s | commits d/s | recalc ms | script ms |
|---|---|---:|---:|---:|---:|---:|---|---|---|---|---:|---:|
| hexagonal-mushroom-column | wheel-zoom | 30 | 88 | **6.1** | 7.0 | 8.2 | **0** | `nodeGraphViewport` 1 | 0/1 | 0/1 | 10 | 490 |
| hexagonal-mushroom-column | drag-pan | 63 | 148 | **5.4** | 5.9 | 6.1 | **0** | `nodeGraphViewport` 1 | 0/1 | 0/1 | 21 | 802 |
| sphere-cut-with-torus | wheel-zoom | 30 | 81 | **5.2** | 7.1 | 8.5 | **0** | `nodeGraphViewport` 1 | 0/1 | 0/1 | 9 | 417 |
| sphere-cut-with-torus | drag-pan | 63 | 142 | **4.9** | 5.2 | 5.4 | **0** | `nodeGraphViewport` 1 | 0/1 | 0/1 | 20 | 722 |

Intermediate runs under heavier load hold the same verdict: 14:56 at **load 56.1** gave medians
7.2–11.2 ms and p95 7.8–12.3 ms; the battery row at 15:12 (**load 38.4**) and at 16:06 (**load 30.3**)
both read **18/18, 0 page errors**.

Two honest notes on the comparison. The baseline was taken before the probe grew its shell-quiescence
wait and its "the gesture ends at its last input event" boundary (§5), and its load was not recorded;
at a 60–150× difference neither changes any conclusion. `script ms` is HIGHER after, because the
after-run actually paints 81–148 frames in the window where the before-run painted one — the thread
is doing the work the user asked for instead of waiting.

---

## 3. Root causes

Four, on three different layers. Each was measured before it was believed.

### 3.1 A guest round trip per wheel notch — `🕸️NodeGraph/🟦️.tsx`

`onWheel` fed the board its wheel delta and then, **on every notch**, read `session.viewport()` and
dispatched `nodeGraphViewport` to the plugin: a `performInvocation` → `command ingress` → `refreshUi`
→ React commit for every tick of a gesture that changes nothing the plugin owns. Measured at 5 and 9
publications inside one thirty-tick scroll (the rest were pre-empted by `observeFlowTask`'s
cancel-per-feature-key, which is its own defect — §3.4).

The camera is **board** state: `wheel_screen` moves `DagHost`'s own camera and the board repaints from
it with no guest involved (`🗺️surface/🕸️node-graph/🦀️.rs:500`). `nodeGraphViewport` exists so the NEXT
open of this graph honours where the user left the view — a per-gesture fact, not a per-tick one.

### 3.2 A guest round trip per pointer move during a drag — `🖱️ui/🎯️targets/⚛️react/🟦️.tsx`

`useCanvasPickInteraction.onCanvasPointerMove` resolved a hover target and published it on **every**
pointer move, including moves with a button held. A 1 s drag-pan therefore cost 16–17 `interactionHover`
invocations, each of which re-renders the shell's interaction-scoped bodies. A move with a button down
is a *drag* — orbit, pan, marquee, wire — and the surface that owns the gesture already tracks whatever
chrome the drag needs.

### 3.3 The flow ABI moved operation payloads at ~68 KB/s — `🌊️flow/🕸️wasm/🦀️.rs`

This is the one that made the *paint* slow rather than the dispatch, and it was invisible until
measured. With §3.1 and §3.2 fixed, the gesture still painted once: the `surface.paint` span showed a
median **3 193 ms** for a single `renderCanvas`, while the page spent only 439 ms of that window on
script. The thread was not computing; it was waiting on the flow bridge.

`🐍️flow-pump-recon.mjs` with the pump instrumented: on an **idle** shell the host pump ran 2 044 steps
and drained **130 617 ABI messages in 2 seconds** — 68 000/s, every one of them costing a
`flow_bridge_poll` (with an allocate/release pair) plus a `flow_bridge_send` reply. The census:

```
kinds  {t2:138, t3e2650:140, t3e2651:199714, t3e2652:139, t3e2653:134, t3e2655:12, t3e2656:137, t4i0:12}
progress events by operation
  2506 setNeuronKindInfosJson 88438 (1 request)   2581 renderFrame          30631 (7 requests)
  2574 labelOverlayPaintState 23771 (9 requests)  2505 setCatalogueJson     22849 (1 request)
  2537 sliderOverlayState      3644 (9 requests)  2610 synchronizeDocument   1873 (1 request)
```

Event `2651` is `FLOW_EVENT_PROGRESS`. Every incremental phase of a flow operation — argument decode,
the retained-DAG cursors (`DagCursorStep::Byte(u8)`, granted `fuel: 1`), output encode — advances by a
**single byte** and answers `Progress`, and `FlowBridge` turns each `Progress` into its own ABI event.
So `FlowProgramFeature::step`, handed a budget of 4 096 bytes of credit and an 8 ms deadline, moved one
byte and returned. A ~20 KB node-graph draw list cost ~20 000 round trips; that is the 200–450 ms
present, and under a gesture sharing the pipe with the overlay reads it became 2.6–3.2 s.

Two things this was **not**, both checked and discarded: the pump's `setTimeout(step, 0)` nesting clamp
(fixed anyway, §4.4 — it moved nothing on its own), and the poll credit being too small (raising it
from 4 096 to 65 536 changed the message count by under 3 %, because the credit was never being spent).

### 3.4 Gesture steps were pre-empted, and overlays repainted from the event — `🕸️NodeGraph/🟦️.tsx`

`observeFlowTask` keeps one task per feature key and **cancels the previous** — right for a query whose
answer is superseded, wrong for an input event. A cancelled `wheelScreen` that had not reached the
session yet is a zoom notch the board never applies, so a fast scroll silently loses travel; the same
for a `pointerMoveScreen`. And every pointer/wheel handler called `renderFlow()` + `paintOverlays()`
directly, so a 60 Hz gesture issued its own eleven-query overlay pass and its own present per event
instead of one per frame, and four of the overlay's `setState` calls allocated fresh objects every pass,
committing the surface's React subtree for a picture that had not moved.

Two late, narrow faults of the same family, found by running the gate twice more: a graph-change
**refit** and an **attach-time opening camera** are both decided asynchronously and could land *during*
a gesture — snapping the view out from under the user and publishing a viewport the gesture did not owe.

### 3.5 Not a cause: style recalc

`RecalcStyleDuration` was 42–73 ms per gesture window before and 10–27 ms after — 1–3 % of the window,
never the `:root`-animation storm of 2026-09-12 (`📓️summary-2026-09-12.md` item 4). Checked first, per
the memory note, and cleared. `contain: layout paint` was added to the board container anyway, so the
board's 60 repaints can never reach the document's layout or style work.

---

## 4. What changed, on the owning layer

### 4.1 `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🟦️.tsx`

- `createFlowCameraGesture` — the camera-gesture rule as one exported unit over injected ports. A wheel
  gesture has no release event, so its end is the absence of the next tick: every tick restarts
  `FLOW_CAMERA_GESTURE_SETTLE_MS` (140 ms), and the settle ends the gesture and publishes the camera
  **once**. `onWheel` now feeds the board and calls `wheelGesture.tick()`; nothing else.
- A gesture ledger (`beginGesture`/`endGesture`/`isGestureActive`) replaces the single
  `isGestureActiveRef` boolean — a surface can be under a wheel gesture and a drag at once, and the
  second `end` must not clear the first. While any gesture is open the demand scheduler is
  `beginContinuous`, so every repaint is coalesced onto a rAF.
- The canvas overlay's `onPointerDown`/`onPointerUp` now open and close that gesture (they never did —
  only the slider overlay did), so a canvas drag no longer resyncs the scene under itself.
- `issueFlowGestureStep` — wheel ticks and pointer down/move/up are issued **without** the
  cancel-per-feature-key pre-emption. A step is an increment on board state; it has no successor that
  could carry its delta.
- Pointer/wheel handlers no longer call `renderFlow()`/`paintOverlays()`; they invalidate the scheduler.
  The per-move `hoveredChannelJson` read is folded into the coalesced overlay pass, and
  `selectionBounds`/`marquee`/`wireRefusal` keep their previous identity when the picture did not move
  (`sameOverlayValue`).
- A released **camera pan** (middle button, `flowGestureIsCameraPan`) publishes the viewport and nothing
  else; it changed neither selection nor fixture, so it owes no `interactionSelect`/`interactionHover`.
- The async **refit** and the attach-time **opening camera** both stand down when a gesture is live.
- A scene that arrives while a gesture holds the session is **deferred, not dropped**. The sync was
  skipped for the duration of a gesture and the effect only ever runs again on the NEXT scene, so with
  gestures now covering wheel and pan the window was wide enough to swallow a real edit. The scene
  effect's body is now `applyScene`, and a camera gesture runs it when it ends; a content gesture's own
  commit brings the next scene.
- `contain: layout paint` on the board container.
- A `semio.hop.surface.paint` span around `renderCanvas` — the present marker the gate reads.

### 4.2 `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx`

`onCanvasPointerMove` returns early while a pointer button is held. Shared by every canvas host, and
right for all of them: a move with a button down is a drag, not a hover.

### 4.3 `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🦀️.rs` (+ the built `🫀️core/🕸️bindings`)

`FlowProgramFeature::step` loops `advance` until the granted `byte_credit` is spent, publishing the LAST
`Progress` of the run. Every other step — `Yield`, `Complete`, `Failed`, a page, a checkpoint, a preview
— returns immediately, so nothing carrying a payload or ending the operation is coalesced away, and the
loop is bounded by the credit whatever the action does. The budget already said how much one poll may
do; it was being under-spent by three orders of magnitude. Rebuilt with
`nx run semio-framework-os-flow-core:wasm` (2m 48s).

### 4.4 `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🖥️host/🏃️runtime/🟨️.js` (+ the bindings copy)

`createFlowPumpScheduler` — the pump re-schedules on a `MessageChannel` post instead of
`setTimeout(step, 0)`, which Chrome clamps to 4 ms once a self-rescheduling callback chain nests past
five levels. It still yields between rounds, so input and rendering keep their turn. On its own this
moved nothing (§3.3); it is kept because a pump paying a timer for every round is wrong regardless, and
because with §4.3 the rounds are now few enough that per-round latency is what remains.

### 4.5 `🧰️framework/🔨️modules/⏱️trace/🟦️.ts` + `🧫️fixtures/🪃️hop-stages/🔣️.json`

One new declared stage, `surface.paint`, owned by `FlowGraphCanvasHost.renderFlow`. The board's own
`[DEBUG] dag draw` line is **not** a frame marker — it is emitted only when the LOD bucket changes, so
it counts LOD transitions. Measuring paints with it reads 1 draw for a whole scroll whatever the
renderer does.

---

## 5. Probe

`🐍️flow-scroll-render-perf-probe.mjs` — Playwright + CDP `Performance.getMetrics`, patterned on
`🐍️react-hop-cost-probe.mjs`. Per example it drives 30 real wheel ticks and a 1 s middle-button
drag-pan at ~60 Hz over the board (located through the host's own `__semioFlowGraphProbe` registry) and
reports, per gesture: tick → next present in ms (median / p95 / max / misses), present duration, guest
invocations by `actionId` during and at settle, `refreshUi` passes, React commits,
`RecalcStyleDuration`/`LayoutDuration`/`ScriptDuration`/`TaskDuration` deltas, and every long task > 50 ms.

Two measurement rules it had to learn, both of which were wrong verdicts before they were fixed:

- **A gesture is measured from a quiet shell.** A surface can re-attach a dozen seconds into a boot and
  publish its opening camera; a gesture started on that tail counts it as a per-tick hop that never
  happened. The probe waits until the `semio.hop.invoke` count holds still for 1.5 s — it waits on
  exactly what it measures.
- **A gesture ends at its last input EVENT**, not at the probe's next CDP round trip. A drag's settled
  publication is dispatched by `pointerup` itself, so a boundary drawn after the release counts the
  settle as "during".

Added as battery row `flow-scroll` (14 min budget, 18 assertions: median ≤ 16 ms, p95 ≤ 50 ms, 0 guest
hops during, 1 camera publication at settle, per example per gesture).

`🐍️flow-pump-recon.mjs` — the ABI census of §3.3. It needs three temporary counters inside the pump to
report the message breakdown; its paint-span half always works.

---

## 6. Laws, with output

**TS — `🧱️elements/🕸️NodeGraph/🧪️tests/🖱️scroll-gesture/🟦️.ts`** (registered in the react renderer's
vitest config; drives the real `createFlowCameraGesture` over a virtual clock):

```
SEMIO_TEST_LEVEL=long bunx vitest run --config …/⚛️react/🧪️tests/🎚️config/🟦️.ts -t "flow node-graph scroll gesture"
 Test Files  1 passed | 49 skipped (50)
      Tests  8 passed | 1269 skipped (1277)
```

The eight: 30 ticks cost the plugin nothing; the settled camera is published exactly once after the
ticks stop; one scroll opens ONE gesture and closes it once; each tick asks the scheduler for a frame
and never paints from the event; two scrolls separated by a pause are two gestures with two
publications; a gesture is active for the whole scroll and settled after it; a disposed gesture
publishes nothing; a middle-button press reads as a camera pan and a left-button press does not.

**Rust — `🗺️surface/🕸️node-graph/🧪️tests/🔬️unit/🦀️.rs`**, two laws: 30 wheel ticks accumulate on the
board monotonically and one read at settle equals the last of thirty reads; 60 pan ticks move the
camera and never the zoom. Written against the public `viewport()` accessor, not the struct field, so a
peer's in-flight `fixture` → `host_document` rename cannot break them.

**Rust — `🌊️flow/🕸️wasm/🧪️tests/🔬️component-domain-laws/🦀️.rs`**, two laws:
`one_poll_spends_its_whole_byte_credit_instead_of_one_byte` (at 4 096× the credit an operation must cost
under 1/100 of the `Progress` steps it costs at credit 1) and
`spending_the_credit_does_not_change_the_operation_result` (the bytes an operation answers are identical
at credit 1 and credit 4 096).

```
cargo test -p … --lib byte_credit        one_poll_spends_its_whole_byte_credit_instead_of_one_byte ... ok
cargo test -p … --lib spending_the_credit spending_the_credit_does_not_change_the_operation_result ... ok
cargo test -p … --lib accumulate_on_the_board
  graph_host_pan_ticks_accumulate_on_the_board_without_a_publication ... ok
  graph_host_wheel_ticks_accumulate_on_the_board_without_a_publication ... ok
```

The byte-credit pair had to be rewritten once before it could run at all: driven at the feature level
they abandoned a `FlowDomainAdapter` mid-operation and tripped `ordered-map root must be explicitly
retired before drop`. They now drive a real `FlowBridge` end to end and close it, which also makes them
count **ABI crossings** rather than feature steps — the unit that actually costs.

**Delay, not a claim:** these four could not be run for most of this lane. A peer's repo-wide
`fixture`/`ColdDocumentPair` → `host_document`/`ColdArtifactPair` rename was landing across `♾️infinite`,
`🌊️flow` and `semio-framework-artifact-flow-flow` from 14:40 to 16:00 (143 errors, then 0, then 22, then
0). They were run as soon as the sweep settled, and both crates were green when they were.

---

## 7. Gate

| gate | result |
|---|---|
| `flow-scroll` probe, `SEMIO_PROBE_GATE=1`, 16:03, load 31.8 | **GREEN** — 4/4 gestures, median 4.9–6.1 ms, p95 5.2–7.1 ms, max 8.5 ms, 0 guest hops during, 1 publication at settle |
| same probe, 14:56, load 56.1 | **GREEN** — median 7.2–11.2 ms, p95 7.8–12.3 ms |
| battery `--only=flow-window,flow-reorganize,graph-keyboard,journey,flow-scroll`, 16:06, load 30.3 | **4/5, 0 page errors** — journey 25/25, flow-window 3/3, graph-keyboard 13/13, **flow-scroll 18/18**; `flow-reorganize` 2/3, red for a reason outside this lane (§9) |
| TS law `flow node-graph scroll gesture` | **8/8** |
| Rust laws (4) | **4/4** — §6 |

`SEMIO_BATTERY_URL=http://127.0.0.1:6022/?plugin=generation3d SEMIO_BATTERY_ROOT=react-scroll-final`,
scoreboard at `🗑️generated/react-scroll-final/scoreboard.json`.

A note on what a red battery row meant today. At 15:15, mid-run, `journey` fell to 15/25 and
`flow-reorganize` to 2/3 while `flow-scroll`, `flow-window` and `graph-keyboard` stayed green on the same
tree — a peer's rename was landing through `PluginRuntime`, `ShellHost`, `Interpreter` and the kernel
during that run, and the new request-time vite freshness guard hands a running page each edit as it
lands. `journey` was **25/25 again** at 16:06 with nothing changed but the clock. `flow-reorganize` is a
real, still-red reader defect, diagnosed in §9.

---

## 8. Files

Product:

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🟦️.tsx`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🖥️host/🏃️runtime/🟨️.js`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/🕸️bindings/🖥️host/🟨️.js` (verbatim copy of the above)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/🕸️bindings/flow_core_bg.wasm` + `🌐️browser/🟨️.js` (rebuilt)
- `🧰️framework/🔨️modules/⏱️trace/🟦️.ts`, `🧰️framework/🔨️modules/⏱️trace/🧫️fixtures/🪃️hop-stages/🔣️.json`

Laws:

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🧪️tests/🖱️scroll-gesture/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts` (registration)
- `🧰️framework/🔨️modules/🗺️surface/🕸️node-graph/🧪️tests/🔬️unit/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🧪️tests/🔬️component-domain-laws/🦀️.rs`

Ticket:

- `🐍️flow-scroll-render-perf-probe.mjs`, `🐍️flow-pump-recon.mjs`, `🐍️snapshot-shape-recon.mjs`,
  `🐍️react-battery.mjs` (new `flow-scroll` row)
- the `fixtureJson` → `hostSnapshotJson` reader rename in `🐍️flow-window-probe.mjs`, `🐍️port-fit-probe.mjs`,
  `🐍️react-gap-probe.mjs`, `🐍️react-gap-recon.mjs`, `🐍️wire-type-guard-probe.mjs`
- evidence: `🗑️generated/flow-scroll/{before,after,after-fresh,after-final,diag,diag2}/`,
  `🗑️generated/react-scroll/`, `🗑️generated/react-scroll-final/`

---

## 9. Open, handed on

- **`flow-reorganize` is red for a reader that a peer's rename left behind, not for a product defect.**
  `mod+alt+L` reaches the plugin and settles (`performInvocation … actionId: "reorganize" … effects: 1`,
  623 bytes, one page), and the probe reports `moved: []` only because it reads widget positions out of
  the node-graph surface's published host snapshot and both `before` and `after` come back `null`. The
  repo-wide `fixture` → `host_document` / `host_snapshot` sweep renamed the surface's probe getter
  (`fixtureJson` → `hostSnapshotJson`) — I updated every ticket probe that read the old name — but the
  scene field behind it is now **undefined**: `🐍️snapshot-shape-recon.mjs` reads
  `{"getters":["entity","hostSnapshotJson","rect"],"snapshot":"null"}` on a shell whose graph is fully
  painted (7 nodes, 6 edges, preview idle 7/7). Whoever owns that rename has to make the scene publish
  the snapshot again under its new name; until then the reorganize row cannot be decided.
- `setNeuronKindInfosJson` moved **88 438 bytes** through the ABI for one request and `setCatalogueJson`
  22 849 — both are once-per-boot, and §4.3 makes them ~4 096× cheaper, but they are the two largest
  single payloads the flow surface carries and neither has been looked at for size.
- The board still re-attaches a second time ~14 s into a boot (`node-graph surface ready` twice on one
  page). Harmless now that a live gesture wins against it, but nothing here explains why it happens.
- `emitInteractionState` still publishes selection **and** hover **and** viewport on every non-pan
  pointer-up, including a plain click that changed no selection. Out of this lane's gate; three hops
  where one would do.
- The `♾️infinite` and `🌊️flow` crates were red for most of this afternoon under the same rename sweep
  (143 → 0 → 22 → 0 errors between 14:40 and 16:00). Everything here was verified after it settled, but
  a peer running these gates during a sweep window will read failures that belong to the sweep.
