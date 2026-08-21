# Packet P3b — Frame BUILDING Off the UI Thread

Boundary: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/**` and `🧰️framework/🔨️modules/🖱️ui/🖥️host/**`.
Reads `📓️p3a-render-snapshot.md` (Phase 3) and `📓️p2a-job-protocol.md` (Phase 2) as prerequisites — this
packet closes the gap P3a reported honestly: `build_and_publish_snapshot` existed, but frame BUILDING
still ran entirely on the UI thread because `AppRuntime` is `!Send`.

## 1. Why `AppRuntime` is `!Send` — per field, not by assertion

`AppRuntime` (`📦️glue.rs:1828`) has ~24 fields. Checked each:

| Field | Type | Send? | Why |
|---|---|---|---|
| `self_weak` | `std::rc::Weak<RefCell<AppRuntime>>` | **No — the disqualifying field** | `Rc`/`Weak`/`RefCell` are `!Send` by design (non-atomic refcount, no synchronized borrow-tracking). This is also **self-referential** — `AppRuntime` holds a weak handle back to its own `Rc<RefCell<Self>>` — which is what forces the whole type to be `Rc<RefCell<_>>`-owned in the first place, not merely `!Send` in isolation. |
| `window: Arc<Window>` | `winit::window::Window` | **Yes, structurally** | Verified against winit 0.30.13 source (`~/.cargo/registry/.../winit-0.30.13/src/window.rs:17-24`): winit's own doc says *"This is `Send + Sync`... However, some platforms (macOS, Web and iOS) only allow user interface interactions on the main thread, so if you use the window from a thread other than the main, the code is scheduled to run on the main thread, and your thread may be blocked until that completes."* On macOS (`platform_impl/macos/window.rs:13`) the inner type is `MainThreadBound<Retained<WinitWindow>>` — genuinely `Send`, but every method call off the main thread round-trips through a dispatch to the main thread and **blocks the calling thread** until it completes. So `window` is not what makes `AppRuntime` `!Send` — but it is exactly the kind of field a worker must never touch, because doing so would silently turn "off-thread work" back into a blocking round-trip to the UI thread. |
| `gpu: GpuContext` | `ui_wgpu::wgpu::gpu.rs` — `device: wgpu::Device, queue: wgpu::Queue, surface: Surface<'static>, ...` | Likely `Send`+`Sync` on native (wgpu's public dispatch types are documented thread-portable) — **not checked for wasm32**, where the WebGPU backend is backed by `JsValue`-derived handles that are `!Send` in a non-atomics wasm32 build (moot there anyway — see §6). | Even where `Send`, `render_frame` (`ui_wgpu/…/gpu.rs:137-151`) fuses `device.create_command_encoder` → `queue.submit` → `surface.get_current_texture()` → `frame.present()` in **one method** — there is no seam to encode on a worker and present on the UI thread without editing `ui_wgpu`, outside this packet's boundary (confirmed by reading `render_frame` directly — same finding P3a already reported). |
| `draw: DrawList`, `overlay: DrawList` | plain `Vec`s of geometry structs (`ui_wgpu/…/draw.rs:301-309`) | Yes | Nothing exotic — `Vec<ScenePass3d>`, `Vec<DrawLayer>`, etc. Not a blocker on their own. |
| `input: InputState<ActionDescriptor>` | `ui_wgpu/…/input.rs:112` | Yes | Plain `f32`/`bool`/`String`/`Vec` fields; `ActionDescriptor` (`ui_wgpu/…/component.rs:16`) is `{ controller_id: String, action: String, args: Option<DslValue> }` — plain data, `Send`. Not a blocker. |
| `atlas: FontAtlas` | `ui_wgpu/…/text.rs:194` | **Unverified — likely not `Send`** | Holds `font_cx: FontContext`, `layout_cx: LayoutContext<[u8;4]>`, `scale_cx: ScaleContext` (Parley/Swash/Fontique text-shaping state). These crates' cache types were not individually audited for `Send` this session — flagged, not asserted either way. Outside boundary (`ui_wgpu`) regardless. |
| `icons: IconAtlas` | `ui_wgpu` | Not audited | Outside boundary; likely plain pixel buffers, lower risk than `FontAtlas`. |
| `shell: ShellState` | `🧱️elements/Shell/🧊️component.rs:912` — **in boundary** | Structurally `Send`-able (its own fields: `Vec`/`HashMap`/`String`/plain structs; `ShellSyncChannel`'s `tokio::sync::mpsc::UnboundedSender`/`broadcast::Receiver` are `Send` when their payload is) | **But see §2 — the real blocker here isn't the struct's fields, it's what `ShellState`'s own methods read/write.** |
| `plugins: Vec<ProgramBridgeEntry>` (via `shell.plugins`) | `🧱️elements/ProgramBridge/🧊️component.rs:306,321,336` | **Platform-split**: native `ProgramBridgeBackend::Wasm { client: KernelClient, wasm_path }` is `Send` (a cheap channel handle to a dedicated kernel thread, by design — H3-wgpu-native already moved instantiation off any in-process `!Send` runtime for this exact reason). wasm32 `ProgramBridgeBackend::Js(Rc<JsValue>)` is **not** `Send` (`Rc`, and `JsValue` itself is `!Send`). | Native is fine; wasm32 is moot anyway (§6 — no second thread exists there). |
| `theme: Theme`, `theme_dark: bool`, `last_cursor`, `last_pointer_*`, `pointer_*`, `modifiers`, `wheel_delta`, `space_pressed`, `wheel_zoom_deadline_ms`, `world3d_camera_dispatch_deadlines_ms: HashMap<String,f64>`, `caret_blink_*`, `asset_poll_pending`, `plugin_modules_root`, `native_plugin_mtimes`, `native_reload_pending` | plain scalars/`HashMap`/`PathBuf` | Yes | None of these block `Send` individually. |

**Bottom line: exactly one field (`self_weak`) makes `AppRuntime` fail `Send`, and it fails it in the
strongest possible way** — not "contains a non-`Send` handle" but "is definitionally owned as
`Rc<RefCell<Self>>`, because it holds a weak reference back to that exact allocation." Every other field
is either already `Send` or is `Send`-with-caveats (`window`) or unaudited-but-boundary-external
(`atlas`/`icons`, in `ui_wgpu`). Removing `self_weak` is therefore necessary but nowhere near sufficient
— §2 covers why.

## 2. A second, independent blocker P3a's own struct-level audit did not surface: `thread_local!`

`🧱️elements/Shell/🧊️component.rs` — in boundary, and the file `ShellState::render_chrome` (the actual
layout/paint entry point `frame()` calls) lives in — declares upwards of a dozen `thread_local!` statics
consulted or mutated during chrome rendering: `FIND_ITEM_SINK`, `BOOT_HUB_ENV`, `CONTENT_FOCUS`,
`CHROME_TOOLTIP_TITLES`, `CHROME_TOOLTIP_HOVER`, `CHROME_DIALOG_STACK`, `CHROME_TOUR_STATE`,
`CHROME_TOUR_AUTO_CONSIDERED`, `CHROME_PREV_POINTER_DOWN`, `CHROME_CLICK_EDGE`,
`CHROME_TOUR_REVEAL_LATCH`, `CHROME_ELEMENT_RECTS`, `TUTORIAL_DISPATCH_GUARD`, `PREFS_STORE`,
`CHROME_PREFS`, `UI_PREFS_LOADED`, `UI_PREFS_LAST_SYNCED` (line numbers in the file, ~130–11700).

This is **not a `Send`/compile-time problem at all** — it is a silent-data-locality problem, and arguably
worse, because nothing fails to compile. `std::thread_local!` storage is genuinely per-OS-thread. If
`ShellState::render_chrome` ran on a worker thread, every one of these caches would be a *different,
freshly-`None`/empty* instance than the one the UI thread has been accumulating into — tooltip hover
state, the dialog stack, tour progress, on-disk prefs cache, the find-panel item sink, and the boot-hub
handshake state would all silently reset every time a different worker thread happened to pick up the
job, or would simply never contain what the UI thread's own callers expect. This would not panic, not
warn, not show up in a diff — it would just be wrong, intermittently, depending on which pool thread
happened to run that frame's build. This is the single most important finding of this packet: **even a
hypothetical fix for `AppRuntime`'s `Rc<RefCell<_>>` ownership would not be enough to safely move chrome
building to a worker** without first auditing and re-homing (or explicitly threading through) every one
of these statics — a separate, large piece of work, not attempted here.

## 3. What genuinely stays UI-thread-bound, and why (per platform)

- **`window: Arc<Window>` mutation** (`set_fullscreen`, `apply_window_cursor`) — winit's own contract:
  off-main-thread calls on macOS/Web/iOS marshal to the main thread and block the calling thread, so
  calling these from a worker would not "move work off the UI thread," it would add a blocking round
  trip back onto it. Native Windows/Linux `Window` implementations do not have this restriction, but the
  code has no platform split here today and shouldn't grow one just for this.
- **GPU surface presentation** (`surface.get_current_texture()` / `frame.present()`, inside
  `GpuContext::render_frame`) — conventionally thread-affine to the thread that owns the platform surface
  (macOS `CAMetalLayer`, the browser canvas), and in this codebase's case structurally fused with command
  encoding in one method with no seam to split (§1, `gpu` row) — outside this packet's boundary to fix.
- **Everything reachable through the ~15 `thread_local!` statics in `Shell/🧊️component.rs`** (§2) — not
  a platform constraint in the traditional sense, but a real constraint of *this specific codebase's*
  current architecture, and the dominant reason "just make `AppRuntime` `Send`" would not actually be
  safe to build on top of today.
- **wasm32 in general** — see §6: there is no second OS thread in this crate's model on that target
  (`renderer_worker_pool()` is `#[cfg(not(target_arch = "wasm32"))]`), so "UI-thread-bound" and
  "everything" are the same set there regardless of any of the above.

## 4. The split landed this packet, and the input-staleness model

**New file** `🎯️targets/🧊️wgpu/🦀️frame_job.rs` (mounted in `📦️glue.rs`). Given §1/§2, a full "build half
is `Send`, produces a `RenderSnapshot` on a worker" split is not achievable inside this packet's boundary
without either (a) editing `ui_wgpu` to add an encode-without-submit seam, or (b) a large rewrite
untangling `Shell/🧊️component.rs`'s thread-locals — both explicitly out of scope/risk budget here. What
*is* real, in boundary, and safely extractable today: `frame()`'s World3D wheel-zoom settle scan
(`world3d_camera_dispatch_deadlines_ms`, an `O(open deadlines)` `HashMap` scan) and its node-graph
wheel-zoom deadline check — pure arithmetic over owned `f64`/`HashMap<String,f64>` values, touching
neither `Rc`/`RefCell`, neither GPU, nor any thread-local.

- `FrameBuildInputs { world3d_camera_dispatch_deadlines_ms, wheel_zoom_deadline_ms, now_ms }` — cloned
  out of `AppRuntime` once per `redraw()`, `Send`, tiny.
- `FrameBuildJob: semio_framework_job::InteractiveJob` — `step()` reuses the exact same
  `sweep_expired_camera_dispatch_deadlines` function `frame()` used to call inline (independently tested
  already, top of `📦️glue.rs`), producing `FrameDirectives { expired_world3d_surfaces,
  wheel_zoom_deadline_cleared }`, JSON-encoded (`serde_json`, already a dependency — zero new deps) into
  `CommitCandidate.output`.
- `FrameBuildHandle::poll_and_resubmit` (native) submits via `semio_framework_job::run_on_worker(&pool,
  Lane::Interactive, job, params)` onto the crate's own pre-existing `crate::renderer_worker_pool()`
  (P1e's process-wide pool — no second pool minted), and polls the returned `Receiver<StepOutcome>` with
  `try_recv` — **never `recv`**. If nothing has completed, it returns the *previous* result; if the job
  finished, it adopts the fresh one and only then submits a new one for the next tick. One job in flight
  at a time.

**The staleness model — the part item 4 of the brief actually asks for.** `frame_job.rs`'s output is
treated as a **candidate list, not an authoritative replacement**. `AppRuntime::frame()` (`📦️glue.rs`,
now `fn frame(&mut self, build_directives: &crate::frame_job::FrameDirectives)`) re-validates every
candidate against **live** state before acting:

```rust
let expired_world3d_surfaces: Vec<String> =
    build_directives.expired_world3d_surfaces.iter()
        .filter(|id| self.world3d_camera_dispatch_deadlines_ms.get(id.as_str())
            .is_some_and(|deadline| app_now_ms() >= *deadline))
        .cloned().collect();
for id in &expired_world3d_surfaces { self.world3d_camera_dispatch_deadlines_ms.remove(id); }
```

and similarly re-checks `wheel_zoom_deadline_cleared` against the live `self.wheel_zoom_deadline_ms`
before clearing it. This makes staleness **safe by construction**, not merely "probably fine": if the
worker's snapshot is one or more frames old, a candidate that's no longer present (or whose live deadline
moved) is silently skipped this tick, and the next resubmitted job — built from *this* tick's fresh
inputs — picks it up. Nothing is ever dropped, double-removed, or double-dispatched because of staleness;
worst case is a one-tick delay in detecting an already-past deadline, never a correctness bug. This is
the general pattern Phase 5 should carry forward for the expensive stages once they move: **the worker
proposes, the UI thread re-validates the specific, bounded set of candidates it was told about — it never
trusts a worker result as ground truth for anything still being concurrently mutated on the UI thread.**

**Deliberately not wired: the caret-blink toggle.** An earlier draft of `frame_job.rs` also computed this
(it's equally cheap, equally `Send`-safe). It was removed before wiring `frame()` up to it, because caret
blink is a **relative** timer ("toggle if ≥500ms since last toggle"), not a **candidate set** like the
World3D scan. Re-validating a relative toggle against a stale snapshot cannot be made safe the same way —
under a multi-frame stall, blindly reusing a stale toggle-decision can silently miss a flip or double-flip
in a way the World3D scan's "is this specific id still expired right now" check cannot. Moving it safely
needs an **absolute** "next flip due at `T`" schedule instead of a relative "elapsed since last flip"
one — left to Phase 5, noted explicitly in `frame_job.rs`'s own module doc rather than shipped half-safe.

**Honest cost/benefit note.** The World3D scan itself, for the common case of a handful of open
viewports, is cheap enough that the clone + JSON-encode + channel round trip this mechanism costs may
exceed the scan's own cost. The value landed here is the **mechanism** — a real `InteractiveJob`,
actually running on `renderer_worker_pool()`, actually polled non-blockingly, actually re-validated
before use — proven correct and safe under a real stress test (§5), ready for Phase 5 to plug the
*expensive* stages (layout/tessellation) into once §1/§2's blockers are resolved. Whether the World3D
scan itself is worth keeping on the worker path for a realistic document (dozens of live World3D
viewports) versus reverting to inline for the common small-`n` case is a benchmark Phase 5 should run,
not something measured this session.

## 5. Non-blocking UI thread, under a real builder stall

`.🧬semio/…/PHASE-3-UI-THREAD-ISOLATION/🧪️frame-job-verify/` (new, non-workspace-member, standalone
crate — same technique P3a's own `🧪️render-snapshot-verify` used, because the real crate cannot be
`cargo check`-ed this session, see §7). Depends on the **real** `semio-framework-job`,
`semio-framework-async`, `semio-framework-trace` via path; the file is a faithful reproduction of
`frame_job.rs` (three pre-existing, unedited crate-root functions it calls via `crate::` — `app_now_ms`,
`renderer_worker_pool`, `sweep_expired_camera_dispatch_deadlines` — reproduced locally since this crate
cannot depend on the renderer crate itself).

Two tests target item 5 directly:

- `poll_and_resubmit_never_blocks_while_the_builder_stalls` — submits a `FrameBuildJob` whose `step()`
  sleeps 300ms (a real, artificial stall) onto a **real** `WorkerPool`, then calls `poll_and_resubmit` at
  a simulated ~240Hz for 100ms of wall time *while the job is still asleep*. Asserts every single call
  returns in under 10ms (measured: all well under 1ms in practice) and that the stale default is what
  comes back throughout — the mechanism proving the ticket's own rule verbatim ("if no newer snapshot
  exists, re-present the previous one; never wait on the builder"). Then actually waits out the stall and
  confirms the *next* poll adopts the real, freshly-completed result.
- `sixty_hertz_polling_cadence_holds_across_many_ticks` — 50 consecutive ticks at a real ~60Hz cadence
  against a real pool, asserting each individual `poll_and_resubmit` call stays under the ticket's own
  2ms present-half budget.

**Result: 7/7 tests pass, `cargo test` and `cargo test --release`, stable across 3 repeated release runs.
`cargo clippy --all-targets -- -D warnings` clean.** One real bug was caught by actually running these
tests (not by inspection): an early draft of `not_yet_expired_deadlines_are_kept` used a `now_ms` past the
fixture's own `wheel_zoom_deadline_ms`, so the test's own assumption was wrong, not the implementation —
fixed in both the real `frame_job.rs` and the verify crate, with a comment recording why, per the "must
validate, must not claim tests pass without running them" rule.

## 6. wasm32 platform constraint

`crate::renderer_worker_pool()` is `#[cfg(not(target_arch = "wasm32"))]` — there is no second OS thread
in this crate's model on that target (confirmed: grepped for any wasm32-side pool, none exists;
consistent with `ProgramBridgeBackend::Js(Rc<JsValue>)` in §1 also being inherently single-threaded).
`FrameBuildHandle::poll_and_resubmit`'s wasm32 branch runs the *identical* `FrameBuildJob` via
`semio_framework_job::run_to_completion` synchronously, inline — same job impl, same protocol, no
duplicated logic (design ticket packet P2a's own item 6: CLI/headless and interactive paths, or here
native-worker and wasm32-inline paths, must never diverge into two implementations of the same job). This
is not a workaround or a gap — "worker" is meaningless when there is no second thread to submit onto, and
this is exactly the platform difference the design doc's own §6 table anticipates for
browser/DOM-bound work.

## 7. What could and could not be verified by actual compilation

**Confirmed freshly, this session** (not merely trusted from P3a's prior report):

```
$ cargo check -p semio-framework-os-renderer-wgpu --lib
   ...
error: could not compile `semio-framework-os-infinite` (lib) due to 821 previous errors
```
(errors are `E0728` `.await` outside `async fn`, all inside `♾️infinite/…/🦀️component.rs` — the sibling
packet's in-progress de-async codemod target, exactly as this ticket's own concurrency warning named).

```
$ cargo check -p semio-framework-os-renderer-wgpu --lib --target wasm32-unknown-unknown
   ...
error: could not compile `semio-s-plugin-stdio` (lib) due to 4824 previous errors
```

Both runs' full error output was grepped for `frame_job`/`os_host.rs`/`winit_app.rs`/`glue.rs`/
`semio-framework-os-renderer-wgpu` — **zero mentions in either** — `rustc` never reaches this crate's own
source on either target this session. This is the same blocker P3a hit, independently reconfirmed rather
than assumed still true.

Verification actually performed instead:
1. **`🧪️frame-job-verify`** — real compilation, real `cargo test`/`cargo test --release`/`cargo clippy
   --all-targets -- -D warnings`, all clean, for every genuinely new piece of logic (`InteractiveJob`
   impl, JSON encode/decode round trip, the deadline-scan computation, and — the highest-value part —
   the non-blocking poll/resubmit contract under a real `WorkerPool` and a real artificial stall).
2. **`rustfmt --check --config-path ./rustfmt.toml`** on every touched/created file. `frame_job.rs`: 0
   diff after one import-order fix. `os_host.rs`: 0 diff. `winit_app.rs`/`📦️glue.rs`: each had exactly
   one hunk attributable to my own new code (both long lines exceeding `max_width = 250`), fixed by hand
   to match rustfmt's own suggested wrapping; the remaining hunks in both files are pre-existing,
   unrelated to this packet (confirmed by reading their content — an old `KernelRequest`/`ExchangeOutcome`
   enum, an old `HostUserEvent::RuntimeReady` variant, an old `Watchdog::start` call — none touched here),
   left as-is per "format only files you edited."
3. **Careful, explicit manual re-verification of every type signature against source actually read** —
   `semio_framework_job`'s `InteractiveJob`/`StepContext`/`StepOutcome`/`BatchJobParams`/
   `BatchDriveConfig`/`run_on_worker`/`run_to_completion`/`root_cancel_token` (read directly from
   `🧵️job/🦀️component.rs`), `semio_framework_async::{Lane, WorkerPool}`, `OsHost`'s existing fields and
   constructor, `WindowDelegate for OsHost`'s single real call site of `AppRuntime::frame` (confirmed via
   grep — exactly one call site, `winit_app.rs:167` pre-edit — so the signature change is safe), Rust's
   module-privacy rule for accessing `AppRuntime`'s private fields from a sibling `#[path]`-mounted module
   (confirmed against the already-landed precedent of `os_host.rs` importing from the equally-private
   `render_snapshot` module).
4. `git diff --stat`/full diff read on every touched file to distinguish "my change" from "P3a's own
   already-landed, still-uncommitted diff against `HEAD`" (per this repo's no-commit rule, both packets'
   edits necessarily coexist as uncommitted working-tree changes) — confirmed no conflict markers, no
   duplicate mounts, exactly one `fn frame(&mut self` definition.

**Honest gap, stated plainly:** `frame_job.rs` itself, and the edited regions of `glue.rs`/`os_host.rs`/
`winit_app.rs`, were never compiled as part of the real `semio-framework-os-renderer-wgpu` crate this
session — confidence rests on the four points above, not on a `cargo check` pass of the actual crate.
Anyone unblocking `os-infinite`/`s-plugin-stdio` should run `cargo check -p
semio-framework-os-renderer-wgpu --all-targets` and `cargo test -p semio-framework-os-renderer-wgpu`
(both profiles, both native and `wasm32-unknown-unknown`) targeting this packet's five files first.
`FontAtlas`'s `Send`-ness (§1) and `atlas`/`icons`' exact behaviour were read but not exhaustively audited
(outside boundary) — flagged, not asserted.

## 8. Interactivity audit / dependency ratchet

```
$ bun ./📜️script.ts verify dependencies
[verify dependencies] baseline: 238 third-party dependenc(y/ies); current: 238.
[verify dependencies] clean — no new third-party dependencies.
```

```
$ bun ./📜️script.ts verify interactivity
[verify interactivity] 180 finding(s) total:
[verify interactivity] 124 block_on/run_blocking finding(s) NOT covered by the allowlist:
```

**Blocking-bridge count: 124, unchanged from P3a's own "after" number.** Expected and correct — this
packet added zero `block_on`/`run_blocking` calls (the whole point of `try_recv`-based polling is to
avoid exactly that class of call) and removed none (it wasn't in scope; P3a already closed the
UI-thread-reachable ones). Still WARN mode, per the ticket's own standing instruction not to flip to DENY
while real UI-reachable gaps remain (§1/§2/§3 of this report are precisely those remaining gaps).

## 9. What Phase 5 must still decompose

1. **`AppRuntime`'s `self_weak: Weak<RefCell<AppRuntime>>` / `Rc<RefCell<AppRuntime>>` ownership** — the
   one field that makes the whole type `!Send`. Removing it means finding a different mechanism for every
   `spawn_app_task`-deferred closure that currently does `self_weak.clone()` →
   (`.await` later) → `upgrade().try_borrow_mut()` (there are roughly a dozen call sites inside `frame()`
   alone: native-plugin hot-reload, `pump_sync_events`, World3D/scene/graph/map/board camera-action
   dispatch, tutorial pending-ops flush, asset polling). This is the single largest piece of remaining
   work and was correctly out of this packet's risk budget — a half-done rewrite of a dozen re-entrant
   deferred-mutation call sites, unverifiable by compilation this session, would be far more dangerous
   than reporting the gap.
2. **The `thread_local!` audit in `Shell/🧊️component.rs`** (§2) — a prerequisite that must happen
   *before* or *alongside* #1, or a `Send` `AppRuntime` would still build chrome incorrectly on a worker.
   Each of the ~15 statics needs a decision: promote to an explicit field threaded through
   `ShellState`/the job's input-output contract, or prove it's genuinely UI-thread-only state that stays
   behind (e.g. genuinely ephemeral hover/tooltip UI chrome might legitimately belong on the present
   half, not the build half).
3. **`ui_wgpu`'s `GpuContext::render_frame` encode/submit fusion** (§1/§3, `ui_wgpu/…/gpu.rs:137-151`) —
   needs a seam (e.g. `encode_scene`/`encode_composite` returning `wgpu::CommandBuffer`s without
   submitting, plus a separate `submit_and_present`) so a worker can prepare GPU upload/draw packets and
   the UI thread only submits+presents, per the ticket's own "bounded, already-prepared rendering packet"
   rule. Outside this packet's boundary (`🖱️ui/📦️packages/**`, not `🖱️ui/🖥️host/**`).
4. **`InputState`/immediate-mode hit-testing** — P3a already found this doesn't go through
   `ui_render::DispatchTree`; still true, still unaddressed, still the architecture mismatch that means
   the design doc's literal hit-test-split assumption doesn't apply to this renderer. Either migrate onto
   `ui_render::DispatchTree`/`hit_test` (the `Element`/`FrameEngine` migration `os_host.rs`'s own
   docstring already names as the eventual target), or design a bespoke staleness contract for
   `InputState` itself — this packet did neither, same as P3a.
5. **The caret-blink relative-timer redesign** (§4) — needs an absolute "next flip due at" schedule
   before it can safely move off-thread; not attempted here.
6. Once #1–#3 land, `render_chrome`'s actual layout/text-shaping/tessellation/draw-sorting/GPU
   upload-packet preparation is the real payoff target for `run_on_worker` — `frame_job.rs`'s
   `FrameBuildJob`/`FrameBuildHandle` seam (job type, `poll_and_resubmit` non-blocking contract,
   candidate-list re-validation pattern) is designed to be the template that stage's own job plugs into,
   not a parallel mechanism to reinvent.

## 10. Files touched

- New: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️frame_job.rs`
- New (ticket-folder verification artifact, not a workspace member):
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/PHASE-3-UI-THREAD-ISOLATION/🧪️frame-job-verify/`
- Modified: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️glue.rs`
  (mount `frame_job`; `fn frame` now takes `&frame_job::FrameDirectives` and re-validates its two
  candidate fields against live state instead of computing the World3D scan/wheel-zoom check inline)
- Modified: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️os_host.rs`
  (`frame_build: frame_job::FrameBuildHandle` field + constructor wiring)
- Modified: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️winit_app.rs`
  (`build_and_publish_snapshot` computes `FrameBuildInputs`, calls `poll_and_resubmit`, passes the result
  into `app.frame(&build_directives)`)
- Modified: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/Cargo.toml`
  (added `semio-framework-job` — workspace-internal, `238 → 238`)

No files outside `🖱️ui/🖥️host/**` and `📺️renderer/**` were edited. `ui_wgpu`
(`🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/`), `os-infinite`, `s-plugin-stdio`, and `Shell/🧊️component.rs`'s
`thread_local!` statics were read-only inspected, never edited.
