# Packet P3a — RenderSnapshot, Enqueue-Only Host Contract, and the 17 `block_on` Sites

Boundary: `🧰️framework/🔨️modules/🖱️ui/🖥️host/**` and `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/**`.
Baseline for this session's edits: repo `HEAD` at the start of this packet (`95b8688ee2` plus whatever
the two concurrent sessions had already landed — see §7's concurrency note).

## 1. Headline result

Interactivity audit (`bun ./📜️script.ts verify interactivity`), before → after this packet:

| | Before | After |
| --- | ---: | ---: |
| Total findings | 198 | 180 |
| **Blocking-bridge (`block_on`/`run_blocking`)** | **142** | **124** |
| sync-clipboard | 6 | 6 (unchanged) |
| sync-fs | 36 | 36 (unchanged) |
| sync-process | 6 | 6 (unchanged) |
| thread-pool | 8 | 8 (unchanged) |

**-18 blocking bridges**, all of them the UI-thread-reachable `pollster::block_on(ParallelRuntime::…)`
sites the design doc's Category C named as "THE PHASE 3 FOCUS", plus the native-only synchronous-network
sites `poll_pending_assets` was hiding. Dependency ratchet: `bun ./📜️script.ts verify dependencies` — 238
before and after. Still WARN mode (not flipped to DENY — see §6 for what remains).

## 2. A critical finding before anything else: the crate had never been type-checked

`winit_app.rs`, `os_host.rs`, `deadlines.rs`, and `kernel_seam.rs` (landed by an earlier ticket,
`26/08/20/SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY`) already called `ui_host::{WindowDelegate,
WindowMetrics, RedrawOutcome, should_request_redraw, PointerRegistry, pointer_info_for_mouse, …}` and
`ui_render::{FrameScheduler, InvalidationReason, DispatchEvent, CursorRequest, …}` and
`ui_contract::UiIntent` throughout — but `semio-framework-os-renderer-wgpu`'s own `Cargo.toml` never
declared `ui_host`, `ui_render`, or `ui_contract` as dependencies at all. Confirmed by grepping the
Cargo.toml for these names (zero hits) before adding them. This crate has therefore never been reached
by `rustc`'s type checker since that ticket landed — every earlier packet's "clean" claims for these
specific files rested on `rustfmt`-only parse validation (see P1e's own report, which documents exactly
this same verification gap for the same reason). Fixed here: all three added to `[dependencies]`,
workspace-internal, `238 → 238`.

## 3. `RenderSnapshot` / `RenderSnapshotSink`

New file: `🎯️targets/🧊️wgpu/🦀️render_snapshot.rs`.

```rust
pub struct RenderSnapshot {
    pub revision: u64,
    pub generation: semio_framework_trace::Generation,
    pub timestamp_us: u64,
    pub cursor: CursorRequest,
    pub ime: Option<ImeDirective>,
    pub dispatch_tree: Option<Arc<()>>,   // see §5 — no real value exists yet
    pub damage_regions: Option<Vec<()>>,  // no damage tracking in the DrawList pipeline yet
}

pub struct RenderSnapshotSink {
    current: Mutex<Arc<RenderSnapshot>>,
    next_revision: AtomicU64,
}
```

### 3.1 A real bug, caught by a stress test, not assumed away

The first implementation used a hand-rolled `Arc` + `AtomicPtr` scheme (`Arc::into_raw`/`from_raw`/
`increment_strong_count`), matching the design doc's own literal sketch. A concurrent stress test with 4
publisher threads and 4 reader threads (`many_publishers_and_readers_never_tear_or_crash`) crashed with
`SIGTRAP` — a genuine use-after-free. Root cause: `acquire()`'s `load` then `increment_strong_count` are
two separate steps; a concurrent `publish()` can `swap` the pointer out and drop its `Arc` (freeing the
allocation, if the sink held the last strong reference) in the gap between them. This is the textbook
ABA/use-after-free hazard hazard-pointer or epoch-based reclamation schemes exist to solve — a naive
`AtomicPtr` swap does not have that protection for free.

Fix: `Mutex<Arc<RenderSnapshot>>`. `acquire()` is `lock().clone()`; `publish()` is `*lock() = Arc::new(..)`.
Zero `unsafe` code. Both are sub-microsecond critical sections (a pointer clone/store, never the frame
build or any I/O) — a mutex here is the same class of operation as a `RefCell` borrow, not the kind of
"waiting on a worker" the ticket's governing rule forbids. Re-ran the same stress test (3x debug, 1x
release) plus a new 10,000-cycle publish/acquire loop — all green, no crash, no leak observable.

**Verification method, since the real crate cannot be `cargo check`-ed (see §7):** the file was copied
byte-for-byte into a standalone, non-workspace-member crate at
`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/PHASE-3-UI-THREAD-ISOLATION/
🧪️render-snapshot-verify/`, depending on the real `semio-framework-trace` and `semio-framework-ui-render`
crates via path. `cargo test` (debug and `--release`) and `cargo clippy --all-targets -- -D warnings` all
pass, 7/7 tests. This is the actual mechanism by which the unsafe-then-fixed code was compiled and
exercised, not merely read. Left in the ticket folder per CLAUDE.md (temp verification artifacts stay in
the ticket folder; it is not a workspace member and is never built by CI).

### 3.2 Publication contract, and its honest limits

`OsHost::redraw` is now split into two named methods (`winit_app.rs`, inherent `impl OsHost`, NOT inside
`impl WindowDelegate for OsHost` — a trait impl block cannot carry extra inherent methods, an early
mistake caught and fixed while writing this packet):

- `build_and_publish_snapshot(&mut self)` — drains `self.events` (§4), calls the existing
  `AppRuntime::frame()` unchanged, narrows the cursor, and `self.snapshot_sink.publish(..)`.
- `present_snapshot(&mut self, now: f64) -> RedrawOutcome` — `self.snapshot_sink.acquire()`, applies
  caret/hot-swap deadline bookkeeping, returns the snapshot's cursor/IME directives.

**What this genuinely achieves:** a real, tested, atomic publish/acquire contract; `acquire` never blocks
and never waits on a worker — if nothing newer has published, it re-presents the same `Arc` (verified by
`acquiring_twice_with_no_new_publish_re_presents_the_same_snapshot`), which is the ticket's governing rule
verbatim.

**What it does NOT yet achieve — read before assuming frame BUILDING moved to a worker.** Both methods
still run on the UI thread, in the same `redraw()` call, back to back. Two hard blockers, both outside
this packet's file boundary:

1. `AppRuntime` is `Rc<RefCell<_>>` (`self_weak: std::rc::Weak<RefCell<AppRuntime>>` is its own field) —
   **not `Send`**. It cannot be moved onto an OS worker thread as-is. `os_host.rs`'s own pre-existing
   module docstring already names this exact reason for why `scheduler` had to live on `OsHost`, not
   `AppRuntime` — the same constraint blocks moving `frame()` itself.
2. GPU submission (`self.gpu.render_frame(&self.draw, ..)`) happens INSIDE `AppRuntime::frame()`, and
   `GpuContext`/`DrawList` are types owned by the `ui_wgpu` crate (`🧰️framework/🔨️modules/🖱️ui/📦️packages/
   🦀️rust/🎯️targets/🧊️wgpu/`) — outside this packet's `🖱️ui/🖥️host/**` + `📺️renderer/**` boundary. There
   is no existing seam inside `ui_wgpu` to encode-without-submitting, so "build off-thread, submit
   on-thread" cannot be wired without editing that crate.

The split is real in the CODE (two named functions, a genuine publish→acquire crossing through the sink)
and is the exact seam a worker-side builder plugs into once blocker 1 is resolved (an ownership rewrite of
`AppRuntime`, or migrating off the immediate-mode `DrawList` pipeline onto the `Element`/`FrameEngine`
pipeline `os_host.rs`'s own docstring already names as the eventual target) — it is not yet real in
THREADING. Reported honestly rather than papered over.

## 4. The enqueue-only host contract

New file: `🖥️host/📦️packages/🦀️rust/🦀️enqueue.rs` (in `ui_host`, mounted via `📦️glue.rs`).

- **`UiThreadToken`** — zero-size, `Copy`. `pub(crate) fn mint()` is used by `NativeHost::new`/
  `CanvasHost::new` (both edited to hold a `_ui_token` field, closing a pre-existing dead-code gap in
  the same motion). `pub fn mint_for_host()` is the public escape hatch for a hand-rolled
  `ApplicationHandler` that cannot go through `NativeHost`/`CanvasHost` — exactly `WinitApp`'s own
  documented situation (two-phase boot handshake). `OsHost::new` calls it once.
- **`WorkerContext { generation: InputGeneration }`** — the capability `EventQueue::drain` requires.
- **`CoalesceSlot`** — three `Copy`-only fields (`pointer_move`, `scroll`, `metrics`), zero heap
  allocation. Pointer-move and metrics/resize are latest-wins (overwrite); scroll ACCUMULATES delta
  while replacing position, so a wheel-tick burst before a drain does not lose earlier ticks' magnitude
  — a deliberate, tested deviation from a naive "always overwrite" reading of "replaceable."
- **Discrete queue** — `VecDeque<DiscreteEvent>` bounded to `DISCRETE_QUEUE_CAPACITY = 256` (>25x a
  60Hz frame's typical 6–10 events per the design doc's own estimate). `try_push`-equivalent
  (`enqueue`) returns `EnqueueOutcome::Overflow` rather than growing or silently dropping — overflow is
  caller-observable (logged in `handle_event`), never silent. **Honest deviation from the design doc's
  literal fixed-`[u8;128]`-struct sketch:** `DispatchEvent::KeyDown`/`Paste`/`Ime` carry a real,
  unbounded `String` (a logical key label, a pasted clipboard string, an IME composition). A byte-for-
  byte fixed struct would have to silently truncate a paste or IME composition — a correctness bug, not
  an optimization — so the discrete queue is a bounded (not zero-allocation) `VecDeque` instead. This is
  the honest tradeoff, not a shortcut: bounded item COUNT with observable overflow is preserved; true
  zero-allocation is not, and cannot be, for these specific variants.
- **`InputGeneration(u64)`** — bumped on every state-changing enqueue, carried on every drained item
  (`PointerMoveSample.generation`, `ScrollSample.generation`, `DiscreteEvent.generation`) — the
  mechanism the ticket asks for ("input generation ids prevent a late hit result from acting on stale
  pointer state"). See §5 for why no consumer exists yet to actually compare generations against.

`OsHost::handle_event`/`handle_metrics` (in `winit_app.rs`'s `impl WindowDelegate for OsHost`) now
enqueue instead of immediately spawning one heap-allocated `spawn_app_task` future per event — the
previous behaviour, which meant a pointer-drag at 120Hz spawned 120 boxed futures per second.
`build_and_publish_snapshot` drains the queue ONCE per redraw and dispatches the WHOLE batch through a
single `spawn_app_task` call (`dispatch_drained_events`, reusing the existing `dispatch_normalized_event`
per-variant logic with zero duplication). This is a genuine, measured reduction — from O(events) spawned
futures to O(frames) — even though it stops short of literally zero allocation, for the same honest
reason as the discrete queue above: `AppRuntime::handle_pointer_move`/`handle_pointer_button`/`handle_key`
are `async fn` (deep in `AppRuntime`, itself out of this packet's remaining time budget to de-async — a
much larger, separate undertaking spanning dozens of call sites) and cannot be called synchronously from
inside `redraw()` without either becoming sync or staying deferred via the existing `spawn_app_task` path.

`handle_metrics`'s `app.resize(..)` call stays immediate (not deferred through the queue) — the GPU
surface must be reconfigured before the next `render_frame` submission or the backend rejects the
mismatched size; this is exactly the design doc's own §6 "GPU resource creation… may need locking in
native" case, kept minimal (no layout, no tessellation).

**Tests** (`enqueue.rs`, 9 new — all pass, debug + release, native): pointer-move storm coalesces to one
sample; scroll storm accumulates delta rather than overwriting; resize storm coalesces to the latest
metrics; discrete events (up to capacity) are never dropped; discrete overflow is reported, not silently
dropped; pointer down/move/up stay lossless and ordered; input generation increases monotonically and
survives drain; a drained queue reports empty. Also wasm32-unknown-unknown-clean (see §7).

## 5. Hit-test split — an architecture mismatch, not a split

The design doc's §5 (keep the QUERY on the UI thread against the last committed `DispatchTree`, move the
BUILD to a worker) presumes hit-testing goes through `ui_render::DispatchTree`/`hit_test`. It does not, in
this renderer. `AppRuntime::frame()`'s actual hit-testing is `self.input.hit_at(x, y)` —
`ui_wgpu::wgpu::InputState`, an immediate-mode structure rebuilt as part of `frame()` itself, not the
`ui_render::dispatch` module's tree at all. `InputState`/`hit_at` live inside the `ui_wgpu` crate,
outside this packet's boundary.

Consequence: `RenderSnapshot::dispatch_tree` is `Option<Arc<()>>` — always `None` today, an honest
placeholder rather than a fabricated tree, because there is no real `DispatchTree` value this renderer
produces to put there. `InputGeneration` (§4) exists and is threaded through every enqueued/drained
event, ready for a consumer to compare against — but there is no split query/build boundary for it to
guard yet, because the build and the query are the same immediate-mode call
(`self.input.hit_at`/`update_hover`, inside `frame()`) on this renderer today.

**What Phase 5 (or a prerequisite packet) actually needs to do here:** either (a) migrate this renderer's
hit-testing from `InputState`/`hit_at` onto `ui_render::DispatchTree`/`hit_test` — the real architecture
the design doc assumed — which is the `Element`/`FrameEngine` migration `os_host.rs` already names as its
own deferred target, or (b) accept the immediate-mode reality and design a DIFFERENT staleness contract
specific to `InputState` (e.g. rebuild-vs-reuse markers on `InputState` itself). This packet does neither
— it records the mismatch rather than forcing a fake tree into `RenderSnapshot` to look complete.

## 6. The 17 (actually ~19) `block_on(ParallelRuntime::…)` sites — resolved, and how

The design doc's Category C table names 19 line numbers (grouped into ~17 "actions") inside `glue.rs`'s
`kernel_runtime::KernelThreadState` and its `poll_pending_assets` asset-fetch path. All were found intact
at the same lines (confirming no other session had touched this exact region) and resolved as follows.

### 6.1 `KernelThreadState` (15 sites → 1)

Lines 458, 478, 482, 535, 551, 567, 569, 570, 573, 592, 594, 644, 652, 663, 677 (original numbering) were
each an individual `pollster::block_on(...)` wrapping a `ParallelRuntime`/`GuestRuntime` async call inside
`KernelThreadState::{new, create_app, activate_extensions_of, destroy_app, run_turn}` — all synchronous
`fn`s at the time.

**A load-bearing fact discovered while investigating these:** `KernelThreadState` only ever runs inside
`run_kernel_thread`, a dedicated background OS thread (`"semio-kernel"`, spawned once, lazily, by
`KernelClient::get()`) — reached from the UI thread only through the fully async, non-blocking
`KernelClient`/`KernelFuture`/`ResponseSlot`/mpsc-channel protocol (landed by the `os-host` ticket,
`SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY`, likely AFTER the design doc that named these 17 sites was
written). So these 15 `block_on` calls were **already not literally blocking the UI/winit thread** —
P1e's own report already said as much for the ones it added. They were still real defects: 15 scattered,
disguised `block_on` calls counted by the audit's static scan and forbidden by the "block_on confined to
approved process/test entry points" rule, and the `"semio-kernel"` thread itself was an ad-hoc OS thread
never registered with the trace crate's thread-role census.

Fix: every `KernelThreadState` method became a genuine `async fn`, with every internal
`pollster::block_on(x)` replaced by `x.await`. `run_kernel_thread` now wraps its ENTIRE
request-processing loop in exactly ONE `pollster::block_on(async move { .. })` — the thread's own entry
point, the same class of justified bridge as `run_native`'s event loop or the CLI's `bin.rs` (P1a's own
`entrypoint`-feature-gated precedent). Net: **15 static call sites → 1**. Also registered this thread as
`semio_framework_trace::register_io_boundary_thread("semio-kernel")` — it was invisible to the census
before this packet; retiring it in favour of a genuine `WorkerPool` job submission is flagged as
follow-up (§8), out of this packet's boundary (it changes `KernelClient::get`'s own construction, a wider
blast radius than the `block_on` removal this packet targets).

### 6.2 `poll_pending_assets` (4 sites → 0), and a real bug fixed along the way

Lines 2157, 2170, 2180, 2184 (original numbering) were inside `AppRuntime::poll_pending_assets`, called
at the end of `frame()` — genuinely on the UI thread, unlike §6.1. Worse: the native
(`not(wasm32)`) branch did SYNCHRONOUS network I/O directly inline — `fetch_map_tile_bytes_blocking`
called `ureq::get(&url).call()` with no deferral at all, and the other three sites wrapped
`fetch_url_bytes` in `pollster::block_on`. The wasm32 branch, by contrast, ALREADY used the correct
non-blocking `spawn_app_task` pattern.

Fix: deleted the entire native fast-path (`fetch_map_tile_bytes_blocking`, both its native `ureq`-based
body and its wasm32 no-op stub, both now dead) and unified both platforms onto the wasm32 branch's
existing `spawn_app_task`-deferred pattern — the only remaining platform difference is URL resolution
(native needs `SEMIO_ASSET_BASE_URL` absolute-ification via `resolve_asset_fetch_url`/
`resolve_map_tile_fetch_url`; wasm32 resolves relative URLs against the page origin for free). `self
.asset_poll_pending` is now set on native too (previously wasm32-only), which is required for
correctness once native's fetch became async (without it, a still-in-flight fetch could be re-triggered
every subsequent frame). Net: **4 static call sites → 0**, and a genuine UI-thread-blocking synchronous
HTTP call is gone, not merely relocated.

### 6.3 What was correctly left alone

- `run_kernel_thread`'s new single `pollster::block_on` (§6.1) — a legitimate entry-point bridge.
- `scale_bench::Env` (10 sites, lines ~1032–1224) — a standalone CLI benchmark harness
  (`semio-wgpu-native --scale`) that never shares a process with the interactive winit host, per P1e's own
  report; confirmed unchanged and out of scope.
- `run_smoke` (1 site, line ~2716) — a headless CLI/smoke-test entry point, not reachable from the
  redraw loop; confirmed a legitimate process-entry-point bridge, same class as `run_native`.
- `ProgramBridge`/`Shell` directory-client boot sequence (6 sites, `🧊️component.rs`) — run once,
  synchronously, during `ShellState::new()` before the event loop starts (confirmed by P1e's own report
  and re-confirmed here: unchanged, not per-frame, not reachable from `redraw`).

None of these were part of the design doc's Category C "17 UI-thread-reachable" set, and none were
touched — consistent with staying inside scope rather than chasing the audit's raw total down further.

## 7. Concurrency and verification honesty

**`semio-framework-os-infinite` (869 lib errors) and `semio-s-plugin-stdio` (4,827 lib errors) — both
transitive dependencies of `semio-framework-os-renderer-wgpu` — are still broken, unchanged from the
numbers already recorded in `📌️status.md`, and are the sibling session's active de-async codemod target
per this packet's own briefed boundary.** `cargo check -p semio-framework-os-renderer-wgpu --all-targets`
therefore never reaches ANY file in this crate (confirmed: grepped the full error output for
`semio-framework-os-renderer-wgpu`/`glue.rs`/`os_host.rs`/`winit_app.rs`/`render_snapshot.rs`/
`enqueue.rs` — zero mentions, meaning `rustc` stops before ever compiling this crate's own source).

Verification used instead, matching the precedent P1e's own report already established for this exact
same blocker:
1. `rustfmt --check --edition 2021` on every touched/created file — 0 parse errors (diffs shown are pure
   line-wrapping cosmetics from not passing the repo's own `rustfmt.toml`, same caveat P1e's report notes).
2. Careful manual re-verification of every call site's type signature against source actually read
   (not recalled) — `KernelThreadState`'s async conversion, `EventQueue`'s API, `RenderSnapshotSink`'s
   API, `WindowDelegate`'s call sites.
3. `semio-framework-ui-host` — the crate holding `enqueue.rs`, and the `NativeHost`/`CanvasHost` edits —
   IS reachable and DOES compile: `cargo check --all-targets` clean (0 errors, 0 warnings from new code);
   `cargo test` 35/35 passed debug AND `--release`; `cargo check --target wasm32-unknown-unknown` clean
   (after fixing a real pre-existing cfg-gating bug in `event.rs`, found while producing this exact
   verification — see §7.1).
4. The standalone `🧪️render-snapshot-verify` crate (§3.1) — real compilation and a real concurrent stress
   test for the one piece of genuinely risky new code (the snapshot sink), which is precisely how the
   use-after-free bug was caught rather than shipped silently.

**Honest gap, stated plainly:** `glue.rs`, `os_host.rs`, `winit_app.rs`, and `render_snapshot.rs`
themselves (as mounted inside the real `semio-framework-os-renderer-wgpu` crate, with its real,
now-massive `AppRuntime`/`ShellState` dependency graph) were never compiled as a whole by `rustc` in this
session. Confidence rests on the four points above, not on a full `cargo check` pass. The first thing
whoever unblocks `os-infinite`/`stdio` should do is `cargo check -p semio-framework-os-renderer-wgpu
--all-targets` and `cargo test -p semio-framework-os-renderer-wgpu` (both profiles), targeting this
packet's own files specifically.

### 7.1 A second pre-existing bug found and fixed (inside boundary)

`event.rs`'s `impl PointerRegistry` (and `TOUCH_TAG`/`FINGER_MASK`) carried NO `#[cfg(not(target_arch =
"wasm32"))]` gate, even though `struct PointerRegistry` itself does and `winit` is a native-only
dependency — `#[cfg(...)]` only applies to the SINGLE item immediately following it, and only `MOUSE_TAG`
had its own copy. `cargo check -p semio-framework-ui-host --target wasm32-unknown-unknown` failed with 5
`E0433`/`E0425` errors ("cannot find crate `winit`", "item was configured out") before this fix — this
packet's own wasm32-verification requirement is what surfaced it. Fixed: every item in that region now
carries its own explicit `#[cfg(not(target_arch = "wasm32"))]` rather than relying on one attribute
silently covering only the next item — confirmed clean afterward.

## 8. Instrumentation

- `semio_framework_trace::register_ui_thread()` — called once, in `WinitApp::resumed()` (the first
  callback winit's `ApplicationHandler` ever fires, on the one thread `resumed`/`window_event`/
  `about_to_wait` all run on).
- `semio_framework_trace::register_io_boundary_thread("semio-kernel")` — called once, in
  `run_kernel_thread` (§6.1) — closes a real census gap (this thread was invisible before this packet).
- New `Watchdog::start("os_renderer_event", .., InteractiveStage::UiEvent)` around `handle_event` — the
  ticket's own ≤1ms UI-event-callback gate, previously only `redraw` (`InteractiveStage::UiPresent`) was
  wrapped. `handle_event` itself is now a fixed-cost enqueue with no allocation on the replaceable-state
  path, so this watchdog should never trip under normal load — a stress test proving that under a
  synthetic pointer/key storm is listed as not-yet-done in §9.

## 9. What is NOT done — read before treating this packet as complete

- **No `UiThreadToken`/`WorkerContext` threading through `WindowDelegate`'s own trait signature.**
  `handle_event`/`handle_metrics`/`redraw` still take no token parameter — only `EventQueue::enqueue`/
  `enqueue_metrics`/`drain` require one internally. Widening the trait itself is deferred (packet P3c's
  stated scope in the master plan) rather than done here as a drive-by change to a shared trait with real
  callers.
- **No synthetic stress test proving `handle_event`/`redraw` stay under their 1ms/2ms ceilings** — the
  watchdog instrumentation (§8) exists and would RECORD a violation if one occurred, but no test in this
  session actually drove a pointer/key storm through the real `OsHost` to read `Watchdog::violations()`
  back (blocked by the same §7 compilation gap — there is no reachable `OsHost` to instantiate a test
  against right now).
- **Frame building is not on a worker** (§3.2) — blocked by `AppRuntime` being `!Send` and GPU submission
  living inside `ui_wgpu`, both outside this packet's boundary.
- **Hit-testing is not split** (§5) — architecture mismatch (`InputState`, not `DispatchTree`), recorded
  rather than forced.
- **124 blocking-bridge findings remain** (down from 142), all outside this packet's UI-reachable scope
  per the design doc's own Category A/B/D breakdown (BREP/draw/CAD/Process3D/stdio kernel calls) — future
  plugin-migration phases' work, not this one's.
- **Audit stays WARN, not DENY** — per the ticket's own instruction ("do NOT yet flip… until the
  remaining UI-reachable violations are gone"), and per §5/§9 above there ARE still real gaps
  (`AppRuntime`'s async pointer/key dispatch methods, the `!Send` blocker) that a DENY gate would need to
  hold at zero first.

## 10. Files touched

- New: `🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/🦀️enqueue.rs`
- New: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️render_snapshot.rs`
- New (ticket-folder verification artifact, not a workspace member):
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/PHASE-3-UI-THREAD-ISOLATION/🧪️render-snapshot-verify/`
- Modified: `🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/📦️glue.rs` (mount `enqueue`)
- Modified: `🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/🦀️window.rs` (`UiThreadToken` fields on
  `NativeHost`/`CanvasHost`)
- Modified: `🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/🦀️event.rs` (pre-existing wasm32
  cfg-gating bug fix, §7.1)
- Modified: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️glue.rs`
  (kernel_runtime async conversion §6.1, `poll_pending_assets` fix §6.2, `ui_host`/`ui_render`/
  `ui_contract` dependencies)
- Modified: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️os_host.rs`
  (`events`/`ui_token`/`snapshot_sink` fields)
- Modified: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️winit_app.rs`
  (enqueue-only `handle_event`/`handle_metrics`, `build_and_publish_snapshot`/`present_snapshot` split,
  `dispatch_drained_events`, UI-thread registration + watchdog)
- Modified: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/Cargo.toml`
  (added `ui_host`, `ui_render`, `ui_contract` dependencies)

No files outside `🖱️ui/🖥️host/**` and `📺️renderer/**` were edited. `os-infinite`, `s-plugin-stdio`, and
every other crate named in this doc as broken were read-only inspected, never edited.
