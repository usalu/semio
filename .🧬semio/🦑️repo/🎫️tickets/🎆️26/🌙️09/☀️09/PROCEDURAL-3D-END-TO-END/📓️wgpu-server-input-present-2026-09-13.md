# 🌐️ wgpu BROWSER SERVER — presentation and input, on the peer's new host

Ticket `2026/09/09/PROCEDURAL-3D-END-TO-END`, lane "wgpu server input + present", 2026-09-13.
Target: `http://127.0.0.1:6118/?plugin=generation3d`, served by the peer's new
`🎯️targets/🧊️wgpu/🌐️server` (created 02:13–02:29, replaced the deleted `trunk serve`).

Repo MCP was down all session (`repo -32602 invalid initialize params`, `semio CONNECTION_CLOSED`);
no ticket was opened, closed or reopened. The react serve on 6018 was not touched, the procedural
guest was NOT restaged, no git-state-modifying command was run, and nothing of the peer's server was
reverted — every change below either extends it or lands in the shared renderer it serves.

---

## 1. TL;DR

| question | answer |
|---|---|
| **did the new server ever present?** | Yes — and it was already presenting when this lane started. The previous lane's "bare `#001117` ground" (`📓️wgpu-tree-row-hit-test-2026-09-12.md` §5.3) was not a missing route, an untransferred canvas or an unanswered `requestFrame`: all four module routes serve 200, the `OffscreenCanvas` transfer succeeds, and the UI isolate runs a 60 Hz rAF flush (measured: **3 362 batches → 3 362 frame replies in 60 s**). The page was blank because the surface had already **quarantined**. |
| **what quarantined it?** | `worker-present-failed: prepared frame authority was stale before acknowledgement` — read off the live fault banner (§3.2). **The first pointer event of any kind takes the surface down.** |
| **why** | `AppPresenter::present_step`'s `Acknowledge` phase re-read `presentation_authority.current()` and compared it with the packet it was about to acknowledge. That authority is a MOVING target — `OsHost::build_and_publish_snapshot` republishes `observe_presentation_input_generation(frame_generation)` every host tick and every input advances `frame_generation` — while a browser presentation deliberately spans several ticks (the interactive-ceiling loop this same ticket added). So any input arriving between `BeginGpu` and `Acknowledge` faulted a frame that was **already submitted to the GPU**. |
| **the fix** | Freshness is decided ONCE, at admission: `BeginGpu` already validates the packet against the live authority (`PreparedRenderGate::validate`) and freezes that pair into the cursor's `RasterTextureWitness`, which the very next check at `Acknowledge` already compares against. The moving-target comparison is deleted; the witness-based one stays. |
| **what that bought, measured** | Hover, click, wheel and keys no longer take the surface down (§3.4); the shell paints the full generate-mode workbench (§3.5, screenshot); and every DOM input now crosses the wire AND reaches the host's `WindowDelegate::handle_event` with the right coordinates (§3.6). |
| **the hop after that** | `Ui::dispatch_event` was never reached: the `RuntimeApply::DispatchEvents` the host enqueues was never applied, because the runtime completion mailbox was pumped from ONE phase of ONE frame build and a build parked on a checked-out interaction state held that session forever. Named here with console evidence at 04:10; a peer read this file and closed it at 04:15 (`🧵️frame-job` pumps the mailbox at every advance, plus an `InteractionCheckoutLedger`). Rebuilt on their tree and measured at 04:29: **`dispatch_normalized_event` now fires on the live target** (§5.2). |
| **what is still NOT claimed** | Only the FIRST input of a session dispatches — the `ResumeDispatch` that hands the interaction state back is not applied, so every later hover/click/key is enqueued and never dispatched (§5.2, same console). That is inside the peer's ten-minute-old lane and is left to them. Nothing downstream of it (row activation, `sceneInstances ≥ 1`, selection, orbit, chords) is claimed. |

---

## 2. What the new server actually is, and what it is not

`🌐️server/📜️script.ts serve generation3d dev` is a Vite host that COMPILES NOTHING. It mounts four
completed artifact roots and refuses to boot if any is missing (`createWgpuBrowserConfig`'s
`existsSync` gate):

| route | root | producer |
|---|---|---|
| `/renderer-modules/wgpu` | `📦️packages/🦀️rust/dist/wasm-dev` | `@semio-tech/framework-renderer-wgpu:wasm` |
| `/🚀️boot.js` | `🚀️browser-boot/🤖️generated` | `…:generate-browser-boot` |
| `/🎞️frame-worker.js` | `🎞️frame-worker/🤖️generated` | `…:generate-frame-worker` |
| plugin/extension module routes | `🧑‍💻dev/🔌️plugin-modules`, `🧑‍💻dev/🧩️extension-modules` | `…:activate-generation3d-wgpu-dev` |

All four answer 200 on the live serve, measured:

```
/🚀️boot.js/🟨️.js                                        200 text/javascript   58497
/🎞️frame-worker.js/🟨️.js                                 200 text/javascript 1136242
/renderer-modules/wgpu/semio-framework-os-renderer-wgpu.js 200 text/javascript  177202
```

`semioEmojiIndexHtmlVitePlugin` rewrites `🌐️.html`'s `src="/🚀️boot.js/🟦️.ts"` to `/🚀️boot.js/🟨️.js`,
so the served page already addresses the generated bundle. **Nothing was added to the peer's server.**
It was never the defect.

The consequence worth recording: **new renderer wasm needs a page reload, not a serve restart.**

---

## 3. Deliverable 1 — presentation, root-caused and fixed

### 3.1 The four candidates the brief named, each ruled out by measurement

`<ticket>/🐍️wgpu-wire-probe.mjs` (added this lane) hooks `Worker.prototype.postMessage`, the Worker's
own `message` replies, `requestAnimationFrame` and `EventTarget.addEventListener` on the canvas, all
installed via `addInitScript` before any page script runs. On a 60-second settle in `?mode=generate`
(`🗑️generated/wgpu-input/wire-1/wire.json`):

```
sent     { boot: 1, shard-port: 4, activate: 1, turn: 83, batch: 3376, introspect: 8, close: 1 }
received { booted: 1, boot-progress: 31, boot-phase: 87, heartbeat: 170, wake: 127,
           frame: 3376, result: 84, introspection: 8 }
raf      { scheduled: 3373, fired: 3372 }
listeners{ pointermove:1 pointerdown:1 pointerup:1 wheel:1 keydown:1 keyup:1 … }
```

* **missing route?** No — §2.
* **canvas never transferred?** No — one `boot` message with the `OffscreenCanvas` in its transfer
  list, and the Worker answered `booted`.
* **worker never told the surface size?** No — `resize` is the first batch's only event
  (`{t:14000, kind:"batch", seq:1, gen:2, replaceable:["resize"]}`), and the shell's own dock plan
  answers it: `canvas=1434x836` for a 1440×900 viewport.
* **`requestFrame` never answered?** No — 3 376 batches, 3 376 frame replies, one per rAF.

### 3.2 What it really was — read off the live surface

The page under the fault banner in `🗑️generated/wgpu-input/wire-1/shot.png` is the FULLY PAINTED
shell (navbar, Form window, Preview). The banner over it:

```
wgpu renderer fault:

worker-present-failed: prepared frame authority was stale before acknowledgement

Surface: quarantined
Boot stage: ready · silent for 56184 ms
Worker terminated: no · input accepted: no
```

The `close` in the `sent` census above is `BrowserFrameTransport.quarantine()`'s
`requestWorkerClose()`. Batches freeze at 3 376 from that instant — which is exactly the "console goes
silent, nothing dispatches, screenshot is the bare ground" signature the previous lane measured, and
why its `nudge()`-every-second probe produced a bare page: the very first nudge quarantined the
surface 6.6 s into boot, before anything had been presented.

### 3.3 The mechanism, in the two lines that own it

`🧊️renderer/🦀️.rs`, `AppPresenter::present_step`:

* `AppPresentPhase::BeginGpu` — `let expected = self.presentation_authority.current();` then
  `begin_prepared_offscreen(token, &self.gate, packet, expected.scene_revision, expected.input_generation)`,
  whose `PreparedRenderGate::validate` refuses a packet that does not match that live pair, and then
  `raster_operation_authority.begin(expected.scene_revision, expected.input_generation)` freezes the
  same pair into `cursor.raster_witness`.
* `AppPresentPhase::Acknowledge` — re-read `presentation_authority.current()` and faulted when the
  packet no longer matched it.

The authority moves on every host tick: `🪟️winit-app/🦀️.rs`'s `build_and_publish_snapshot` calls
`self.runtime.observe_presentation_input_generation(self.frame_generation)`, and `frame_generation`
advances once per redraw plus once per `enqueue_host_event` — and `🌐️browser-worker`'s `enqueueBatch`
additionally rebases it onto the UI isolate's own input generation. With no input at all the value is
a fixed point (the rebase pins it, the single advance restores it), which is why a settled shell
presented forever and one pointer move killed it.

**The fix** deletes the moving-target comparison. Nothing else is relaxed: the packet is still refused
at admission if it is stale, and the witness comparison immediately below
(`raster_witness.scene_revision != packet.scene_revision() || raster_witness.preview_generation != packet.preview_generation()`)
still pins the packet to the authority that admitted it. A superseded frame is acknowledged and
replaced by the next one instead of becoming a surface fault. The reasoning is written into
`present_step`'s own docstring so the next reader does not re-add it.

### 3.4 Measured after the fix — no input kind takes the surface down

`🗑️generated/wgpu-input/wire-2/wire.json`, same probe, same target:

| stage | batches | frames | last frame |
|---|---|---|---|
| settled (45 s) | 1 921 | 1 921 | `gen 2, quarantined false, verdict admitted` |
| after hover | 1 971 | 1 971 | `gen 3, quarantined false` |
| after click | 2 003 | 2 003 | `gen 6, quarantined false` |
| after wheel + `f` + keyup | 2 066 | 2 066 | `gen 9, quarantined false` |
| +8 s idle | 2 549 | 2 549 | `gen 9, quarantined false` |

and the events the transport actually transferred, from the same file:

```
{seq:   1, gen: 2, replaceable:["resize"],       lossless:[]}
{seq:1940, gen: 3, replaceable:["pointer-move"], lossless:[]}
{seq:1973, gen: 6, replaceable:["pointer-move"], lossless:["pointer-down","pointer-up"]}
{seq:2006, gen: 7, replaceable:["wheel"],        lossless:[]}
{seq:2037, gen: 8, replaceable:[],               lossless:["key-down"]}
{seq:2038, gen: 9, replaceable:[],               lossless:["key-up"]}
```

No fault banner, no `close`, no quarantine, and the worker's `world3d` heartbeat keeps running past
every one of them.

### 3.5 The page, presented

`🗑️generated/wgpu-input/present-generate/final.png` — `?plugin=generation3d&mode=generate`, no input
of any kind, 22 s after load: the navbar with **Generate** lit, the Generations window showing
`Generations / <no generations> / Actions / Add Generation`, the Form window's
"Add a generation to edit input values.", and the Preview window's ground plane.
`🗑️generated/wgpu-input/baseline/final.png` is the same in edit mode, with the extruded column mesh in
the Preview and the node graph in the Flow window.

### 3.6 Input crosses the wire and reaches the host

`🗑️generated/wgpu-input/hop-probe.mjs` drives one resize, one hover, one click, `f` and `Tab`, with
`[DEBUG]` traces added to `WindowDelegate::handle_event`. Live console from the run after the fix:

```
46721 [DEBUG] os_host handle_event PointerMove { pointer: PointerInfo { id: PointerId(1), kind: Mouse, … }, x: 160.0, y: 138.0 } gen=4
50751 [DEBUG] os_host handle_event PointerDown { pointer: … PointerId(1), kind: Mouse, pressure: Some(0.5) …, x: 160.0, y: 138.0, button: Primary } gen=7
50751 [DEBUG] os_host handle_event PointerUp   { … x: 160.0, y: 138.0, button: Primary } gen=8
54766 [DEBUG] os_host handle_event KeyDown { key: "f", modifiers: EventModifiers { shift: false, ctrl: false, alt: false, meta: false } } gen=9
57264 [DEBUG] os_host handle_event KeyDown { key: "Tab", … } gen=11
50751 [DEBUG] os_host drain events lossless-capacity=true frame-generation=9
54766 [DEBUG] os_host drain events lossless-capacity=true frame-generation=11
```

Every `handle_event` is followed within the same millisecond by a drain with capacity to spare, so
the events reach `RuntimeApply::DispatchEvents`. (The drain trace now also prints the drained shape
and the `enqueue_apply` answer; that richer form is in the tree but not yet in a browser build — §7.)

`(160, 138)` is the `Add Generation` row's page point, derived (never guessed) from the shell's own
`dock plan` trace plus `dumpStructure("generation3d-generations")`'s published rect
`[0, 72, 315.392, 24]` — the same derivation `🐍️wgpu-hit-probe.mjs` uses. The coordinate mapping
(DPR, dock plan) is therefore correct end to end, and the keyboard chords (`f`, `Tab`, and by the
same path the modifier-bearing ⌘Z) arrive with their modifier flags intact.

---

## 4. Deliverable 2 — the fixture and the two implementations that answer it

### 4.1 One seam, because there was none

The DOM→wire projection lived inline in `🚀️browser-boot/🟦️.ts`'s `wireInput` listeners — a module
that mounts the shell on import and therefore cannot be imported by a test. It now lives beside the
wire types it produces, in `🚚️browser-frame-transport/🟦️.ts`:

* `browserFrameEventFromDom(event, devicePixelRatio)` — the ONE place a CSS pixel becomes a physical
  pixel, the ONE place a DOM `button` integer becomes a name, and the ONE place a resize is measured.
* `browserFrameEventIsReplaceable(event)` — which lane the projected event belongs on.

`wireInput` is now listeners plus focus/capture: each one builds a `BrowserFrameDomEvent` and calls
`admit`. Both resize paths (`onReady` and the `ResizeObserver`) go through the same projection.

### 4.2 The oracle

`🧫️fixtures/🎮️wgpu-browser-input-wire/🔣️.json` — 14 rows, each with the three hops the brief named
(`dom` → `wire` → `dispatch`) and a `why`. It covers pointer move/down/up at dpr 1 and dpr 2, the
three DOM buttons, pen pressure+tilt, a zero-pressure touch, the wheel (position scaled, deltas not),
`f`, ⌘`z`, `Tab`, a key release, and the resize that is deliberately **not** a dispatch.

### 4.3 Rust — the law over the live enum

`🧪️tests/🎮️wgpu-browser-input-wire/🦀️.rs`, mounted from the new
`🎯️targets/🧊️wgpu/🎮️input-wire/🦀️.rs`. That module is the wire vocabulary
(`BrowserBatch`, `BrowserWireEvent`, `TextTarget`, `BrowserPointerKind`, `BrowserPointerButton`, the
`PointerKind`/`PointerButton` conversions) plus `stateless_dispatch`, split out of
`🌐️browser-worker/🦀️.rs` — none of it is a browser capability, only the `#[wasm_bindgen]` host that
decodes a `postMessage` payload is. Mounted unconditionally, so the law runs in an ordinary native
`cargo test` instead of only inside a browser. `apply_wire_event` now tries `stateless_dispatch`
first and keeps only what needs the Worker (resize → `handle_metrics`, segmented text streams).

Three laws: every fixture row's wire bytes decode and project onto the named dispatch; resize and
text chunks project onto no dispatch; and the camelCase spelling is the contract (a snake_case or
`pointerdown`-spelled payload is refused). Structure and spelling are compared exactly; the numeric
leaves carry one ulp-scale tolerance, because the wire's coordinates are `f32` (`160.696` physical
pixels round-trips as `160.6959991455078`) while a language-neutral oracle must state them in decimal
for its `f64` TypeScript twin to read the same rows.

### 4.4 TypeScript — the twin

`🧪️tests/🎮️wgpu-browser-input-wire/🟦️.ts`, registered in `🧪️tests/🎚️config/🟦️.ts` beside the
transport's existing suites. 21 cases: one per fixture row, plus DPR scaling applied exactly once,
scroll deltas never scaled, the lane choice per row, an unknown DOM button named `primary` rather
than dropped, an unknown pointer type read as a mouse, and — through a real `BrowserFrameTransport`
over a fake worker — the fixture's own wire bytes appearing in one flushed `batch`, with pointer
moves coalesced to one per pointer identity (`[1, 3, 7]`).

### 4.5 Both, run

| command | result |
|---|---|
| `cargo test -p semio-framework-os-renderer-wgpu --lib browser_input_wire -- --nocapture` | **3 passed, 0 failed** |
| `bunx vitest run --config "../../🧪️tests/🎚️config/🟦️.ts" "🧪️tests/🎮️wgpu-browser-input-wire/🟦️.ts"` | **21 passed, 0 failed** |
| `bun ./📜️script.ts test-browser-worker` | **69 passed, 0 failed** (5 files) |
| `bun ./📜️script.ts check-browser-worker` | clean — the generated `🚀️boot.js`/`🎞️frame-worker.js` bytes match their sources after this lane's edits |
| `SEMIO_TEST_LEVEL=long bun ./📜️script.ts test-preview-generated` | **20 passed, 0 failed** |
| full `🧪️tests/🎚️config/🟦️.ts` via `bunx vitest` | 151 passed; the 6 `package-integration` failures are `ReferenceError: Bun is not defined` — that suite must run under its own `test-preview-generated` script, where it passes (row above) |

---

## 5. Deliverable 3 — what 6118 says, and the one hop still open

### 5.1 Claimed and proven, on the live target

* The shell **presents** in both `?mode=generate` and edit mode (§3.5, two screenshots).
* **No input kind quarantines the surface any more** (§3.4, 2 549 consecutive clean frames across
  hover, click, wheel and two keys).
* Every DOM input **crosses the wire** as the fixture's own bytes (§3.4) and **reaches
  `WindowDelegate::handle_event`** with the shell's coordinate mapping (§3.6).
* The host **drains** those events every tick (`[DEBUG] os_host drain events`, §3.6) and enqueues
  each as a `RuntimeApply::DispatchEvents` (`enqueue-apply=true`, §5.2).
* **`Ui::dispatch_event` is reached** — `dispatch_normalized_event` fires with the row's own page
  point, on the peer's post-04:15 renderer (§5.2).

### 5.2 The dispatch hop — named at 04:10, closed by a peer at 04:15, measured here at 04:29

This lane named the hop below from a build that had `[DEBUG]` traces on `handle_event`, the event
drain and `dispatch_normalized_event`, and refusal traces on all three of `start_dispatch`'s refusal
paths: input reached the host and was enqueued as a `RuntimeApply::DispatchEvents`, and **none of the
later rungs ever printed**. While the wasm rebuild that would carry the mailbox-side traces was
blocked behind three peer breakages (§7), a peer read this file and landed the fix in
`🧵️frame-job/🦀️.rs` + `🧊️renderer/🦀️.rs` at 04:15 — their own docstring cites
"`📓️wgpu-server-input-present-2026-09-13.md` §5.2" — replacing the once-per-build `ApplyPending`
phase with a mailbox opportunity at **every** frame-build advance, plus an
`InteractionCheckoutLedger` that ages a checkout and publishes a typed fault instead of freezing.

Rebuilt on their tree (the wasm-gated `AppRuntime` literal in `🌐️browser-worker/🦀️.rs` needed their
new `checkout` field completing — §7.5) and measured with `🐍️wgpu-dispatch-hop-probe.mjs`:

```
45154 [DEBUG] os_host handle_event PointerMove { … x: 160.0, y: 138.0 } gen=4
45154 [DEBUG] os_host drain events generation=InputGeneration(3) move=true discrete=0 enqueue-apply=true
45449 [DEBUG] apply_step DispatchEvents present=true terminal-empty=Some(false)
45449 [DEBUG] start_dispatch enter
45449 [DEBUG] start_dispatch: interaction state checked out for one event
45453 [DEBUG] os_host dispatch_normalized_event PointerMove { … x: 160.0, y: 138.0 }
45468 [DEBUG] start_dispatch enter
```

**`Ui::dispatch_event` is reached.** Every rung of the ladder now fires, on the exact page point the
`Add Generation` row publishes.

**What is still open, measured in the same run:** only the FIRST event dispatches. After 45468 the
ledger's checkout is never handed back — the hover at 46664, the click at 49183 and the keys at
53182/55698 each reach `handle_event` and are each enqueued with `enqueue-apply=true`, and no further
`dispatch_normalized_event` follows. The `ResumeDispatch` completion that carries the interaction
state home is not being applied. That is one hop further in than where this lane started, it is
inside the peer's ten-minute-old lane, and it is left to them rather than edited under them.

### 5.2.1 What that means for the brief's proof list — still NOT claimed

Because only the first input of a session dispatches (§5.2), this lane does **not** claim any of:

* that clicking `Add Generation` dispatches `addGeneration`,
* that `dumpFrameStats("generation3d-generate-preview")` then reports `sceneInstances ≥ 1`,
* that hovering sets `state.hovered` (measured: still `false` on every node of the Generations window
  after a two-step hover — `🗑️generated/wgpu-input/hover-peek.json`),
* that a click on a Preview instance publishes a selection, that a wheel orbits, or that the chords
  switch mode or role.

The original bound, for the record, was measured rather than inferred: the build that printed every
`os_host handle_event` line in §3.6 also carried `[DEBUG]` traces on all three of
`AppRuntime::start_dispatch`'s refusal paths (mailbox handle expired / interaction state checked out /
interaction future credits exhausted), and none of them printed — `start_dispatch` was not refusing
the drained events, it was never entered.

`OsHost::build_and_publish_snapshot` drains the queue and calls
`self.runtime.enqueue_apply(None, true, RuntimeApply::DispatchEvents(…))`. That apply is only ever
executed from `🧵️frame-job/🦀️.rs`'s `ActiveFramePhase::ApplyPending`, through
`RuntimeMailbox::apply_pending_step`, which refuses — without popping — when

```rust
queue.ready.front().is_some_and(|completion| completion.requires_interaction && !runtime.interaction_available())
```

`DispatchEvents` is enqueued with `requires_interaction = true`, so a single interaction state that is
checked out and never returned blocks the queue head and every input behind it forever. The same
gate explains a second observation from the same runs: after the fix a viewport resize no longer
produces a second `wgpu-shell dock plan` line, and `RuntimeApply::Resize` travels through that same
mailbox with `requires_interaction = true`.

Transition-only `[DEBUG]` traces for exactly this predicate (runtime mutex held / interaction checked
out) are in the tree at `RuntimeMailbox::apply_pending_step`, together with traces at
`RuntimeApply::apply_step` and `AppRuntime::start_dispatch`. They are what §5.2's measurement reads,
and they are what the next lane should read again after the `ResumeDispatch` hand-back is closed.

### 5.3 How to reproduce, exactly

```
screen -dmS g3dwgpu "<ticket>/📜️serve-generation3d-wgpu.sh"
cd <ticket> && SEMIO_PROBE_SETTLE=45 SEMIO_PROBE_OUT=wgpu-input/wire-N bun 🐍️wgpu-wire-probe.mjs
cd <ticket> && SEMIO_PROBE_SETTLE=75 SEMIO_PROBE_OUT=wgpu-input/hit-N  bun 🐍️wgpu-hit-probe.mjs
```

The serve does not need restarting for new renderer wasm — reload the page.

---

## 6. Deliverable 4 — the serve command, registered

`.vscode/🧩️launch.seed.jsonc`'s `0_dev` block synthesised `serve`/`dev` × `dev`/`release` rows for
**react only**, although the plugin registry infers the identical
`@semio-tech/framework-os-dev:<verb>-<variant>-wgpu-<profile>` targets for every playground — and it
is those targets the peer wired to the new server
(`serve-generation3d-wgpu-dev` → `bun …/🧊️wgpu/🌐️server/📜️script.ts serve generation3d dev`).

`🚀️launch/🟦️.ts` now carries a `RENDERER_ROWS` table and emits both renderers, so every playground
gets its wgpu browser serve with no seed edit. Regenerated with the repo target
(`bunx nx run @semio-tech/plugin-registry:generate`); `.vscode/launch.json` now has, following the
existing procedural rows' naming:

```
🎮️serve🧩️generation3d🧊️wgpu dev      → @semio-tech/framework-os-dev:serve-generation3d-wgpu-dev   (order 383.25)
🎮️dev🧩️generation3d🧊️wgpu dev        → …:dev-generation3d-wgpu-dev                                 (order 383.251)
🎮️serve🧩️generation3d🧊️wgpu release  → …:serve-generation3d-wgpu-release                           (order 383.26)
🎮️dev🧩️generation3d🧊️wgpu release    → …:dev-generation3d-wgpu-release                             (order 383.261)
```

`<ticket>/📜️serve-generation3d-wgpu.sh` now starts that server directly (its obsolete `trunk serve`
fallback is gone — `Trunk.toml` no longer exists), under screen `g3dwgpu`, deliberately WITHOUT the
`activate-generation3d-wgpu-dev` dependency the launch row carries, because this lane must not
restage the procedural guest.

---

## 7. Builds, and the five peer breakages this ran through

| command | result |
|---|---|
| `bunx nx run @semio-tech/framework-renderer-wgpu:wasm` | **published** twice (2 m 17 s, 2 m 4 s) — the fix, then the fix plus the `[DEBUG]` traces |
| `bunx nx run-many -t generate-browser-boot generate-frame-worker` | published both bundles after the transport/boot edits |
| `bunx nx run @semio-tech/plugin-registry:generate` | regenerated `.vscode/launch.json` |
| `bun ../../🏗️compiler/🌐️wasm/📜️script.ts build dev` | published twice more, on the peer's 04:15 and 04:24 renderer, for §5.2's measurement |

Waits taken and attributed rather than worked around, in order:

1. **`semio-framework-os-infinite`** went red at 03:23 (`WorldBrushPreviewRecord` missing
   `Default`/`Clone`). Waited; the peer landed the derives.
2. **`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`** went red at 03:26 (`sync_world3d_catalogue_drop_preview`
   taking `&mut self` inside an existing `&mut self.tree_drag` borrow, E0499). Waited the prescribed
   five minutes twice; while preparing the minimal completion the peer landed their own — nothing of
   theirs was touched.
3. **`semio-framework-pack`** went red and stayed red for the rest of the session. The peer is
   migrating `📡️replication/⚙️codec/🦀️.rs`'s `DeflateRetainedCursor` from a one-shot `close()` to an
   incremental `close_step(maximum_items, maximum_bytes)` (and `try_new` gained a third
   `maximum_allocation_bytes` argument); `🎒️pack/📐️format/🦀️.rs` followed at 04:03 and now names
   `semio_framework_deflate::RetainedInflateCloseStep` — a type `semio-framework-pack` has no
   dependency on, because `close_step`'s return type crossed the `📡️replication` boundary without a
   re-export. Polled from 03:59 to 04:13 with both files stable and the whole workspace red behind
   them, then **completed the way CLAUDE.md prescribes and nothing more**: `⚙️codec` now
   `pub use`s its own `close_step` answer type (`RetainedInflateCloseStep`, feature-gated, with the
   reason in its docstring) and `📐️format`'s two references name it as `crate::codec::…`, the path
   `pack` already re-exports. No semantics touched, no signature changed, nothing of the peer's
   migration reverted. `cargo check -p semio-framework-pack --lib` went clean immediately after.
4. **`semio-framework-plugin`** went red at 04:12 (`⚛️reactor/🔄️turn/🦀️.rs:1187` naming
   `PatchTracker` unqualified where the rest of the file says `patches::PatchTracker`) — a peer two
   minutes into an edit. Left alone and watched; they landed it at 04:16.
5. **`🌐️browser-worker/🦀️.rs`'s `AppRuntime` literal** was missing the `checkout` field the peer's
   04:15 `InteractionCheckoutLedger` added — the wasm-only twin of a struct whose native literal they
   did update, so `cargo check` stayed green and only the browser target broke (the standing
   "native cargo misses wasm-gated code" trap). **Completed** with the same
   `InteractionCheckoutLedger::default()` their native literal uses, spelled through the crate path
   that module can reach. Nothing of theirs reverted.

With those, the wasm target built clean and §5.2's measurement was taken.

Additionally, `@semio-tech/framework-graph:generate` failed mid-session with
`generatorContracts[…] owner project is missing` for three `📋️project.json` paths — another peer's
taxonomy move. The wasm build was taken through
`bun ../../🏗️compiler/🌐️wasm/📜️script.ts build dev` (the exact command the `wasm` target runs) to
step around that unrelated dependency rather than wait on it.

The shared cargo build directory was used throughout (`CARGO_PROFILE_WASM_DEV_DEBUG=false`,
`NX_DAEMON=false`); no private `CARGO_TARGET_DIR`, no `--release`, no git-state-modifying command,
and no process of anyone else's was killed — this lane ran no `kill`, `pkill` or `screen` command at
all.

⚠️ Worth recording because it changed under this lane and was NOT this lane's doing: the react serve
on **6018** and its `g3dreact` screen socket were both gone by 04:41 (a `restage` screen stands in
their place). 6118 is still up and answering 200 on the same pid it started with (55855, `bun
./📜️script.ts serve generation3d dev`, started 02:32).

---

## 8. Files

**Changed**

| file | what |
|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs` | `present_step`: the moving-target authority comparison at `Acknowledge` deleted, with the freshness contract written into the function's docstring; `input_wire` mounted; `[DEBUG]` traces at `apply_step`, `start_dispatch` and `apply_pending_step` plus a transition-only `log_debug_once_per_transition` helper |
| `…/🎯️targets/🧊️wgpu/🌐️browser-worker/🦀️.rs` | wire vocabulary moved out; `apply_wire_event` projects through `stateless_dispatch` and keeps only the Worker-stateful arms |
| `…/🎯️targets/🧊️wgpu/🪟️winit-app/🦀️.rs` | `[DEBUG]` traces at `handle_event`, at the event drain (with the drained shape and the `enqueue_apply` answer) and at `dispatch_normalized_event` |
| `…/🎯️targets/🧊️wgpu/🚚️browser-frame-transport/🟦️.ts` | `BrowserFrameDomEvent`, `browserFrameEventFromDom`, `browserFrameEventIsReplaceable` — the single DOM→wire seam |
| `…/🎯️targets/🧊️wgpu/🚀️browser-boot/🟦️.ts` | `wireInput` and both resize paths go through that seam |
| `…/🎯️targets/🧊️wgpu/🧪️tests/🎚️config/🟦️.ts` | registers the new twin |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🚀️launch/🟦️.ts` | `RENDERER_ROWS`: the `0_dev` serve/dev rows are emitted for wgpu as well as react |
| `.vscode/launch.json` | regenerated (four new rows per playground variant) |
| `<ticket>/📜️serve-generation3d-wgpu.sh` | starts the peer's new browser server; the dead trunk fallback removed |
| `🧰️framework/🔨️modules/📡️replication/⚙️codec/🦀️.rs` | `pub use semio_framework_deflate::RetainedInflateCloseStep` — completing a peer's live migration so its own `close_step` answer is nameable by its callers (§7.3) |
| `🧰️framework/🔨️modules/🎒️pack/📐️format/🦀️.rs` | the two references to that type now take the re-exported path (§7.3) |
| `…/🎯️targets/🧊️wgpu/🌐️browser-worker/🦀️.rs` | completed a peer's new `AppRuntime::checkout` field in the wasm-only literal (§7.5) |

**Added**

| file | what |
|---|---|
| `…/🎯️targets/🧊️wgpu/🎮️input-wire/🦀️.rs` | the platform-neutral wire vocabulary and `stateless_dispatch` |
| `…/🧑‍🎨engine/🧫️fixtures/🎮️wgpu-browser-input-wire/🔣️.json` | the language-neutral oracle, 14 rows |
| `…/🧑‍🎨engine/🧪️tests/🎮️wgpu-browser-input-wire/🦀️.rs` | the Rust law |
| `…/🧑‍🎨engine/🧪️tests/🎮️wgpu-browser-input-wire/🟦️.ts` | the TypeScript twin |
| `<ticket>/🐍️wgpu-wire-probe.mjs` | the UI→Worker wire probe (postMessage / replies / rAF / listeners / DOM events) |
| `<ticket>/🐍️wgpu-dispatch-hop-probe.mjs` | the hop ladder — drives resize/hover/click/`f`/`Tab` and prints every rung of the renderer's `[DEBUG]` chain between the marks, so the first rung that stops printing names the defect |

**Generated (disposable, under `<ticket>/🗑️generated/wgpu-input/`)**

`baseline/`, `hit-baseline/`, `wire-1/`, `wire-2/`, `present-generate/`, `hover-peek.json`, and the
three throwaway drivers `dom-peek.mjs`, `fault-timing.mjs`, `hop-probe.mjs`, `hover-peek.mjs`.
