# P1e — OS Renderer: Retiring `thread_plan`/`ThreadBudget` and `ParallelRuntime`'s Own Threads

Scope: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/` (the OS renderer only). No file outside
this boundary was permanently edited — see "Cross-boundary breakage" below for one file that was
temporarily, locally patched purely to unblock `cargo check` and then byte-for-byte reverted (verified
via `diff` and `git status` showing zero change) before this packet finished.

## 1. What was rewritten and why

### `Shell/🧊️component.rs` — the directory-client `TokioHostRuntime`/`ComputePool` construction

Line ~50 imported the DELETED `thread_plan`/`ThreadBudget`; `ShellState::new` (~line 1312) used them
to size a private `TokioHostRuntime`/`ComputePool` pair for the directory client. Per P1a's and P1b's
own reports, this file was explicitly named as "the next place to inject a real, externally-owned
`WorkerPool`" rather than fall through to `semio-framework-os-services`'s own crate-private
`global_worker_pool()` default (that accessor is `fn`, not `pub fn` — unreachable from here by
design).

Rewritten to:
- Drop `thread_plan`/`ThreadBudget` from the import.
- Construct `TokioHostRuntime::with_pool(crate::renderer_worker_pool())` — the ONE process-wide
  `WorkerPool` this renderer crate now owns (see §2) — instead of `TokioHostRuntime::new(plan,
  &budget)` (a constructor that no longer exists post-P1a/P1b).
- Fix three call sites that were missing `.await` entirely on now-genuinely-`async fn` methods
  (`open_scope`, `ComputePool::new`, `NativeDirectoryTransport::with_new_http_pool` — all became
  `async fn` in the post-P1a/P1b API; this call site pre-dates that and had never been updated,
  confirmed a pre-existing latent bug, same family P1a/P1b/P1c/P1d each found elsewhere). Bridged via
  `TokioHostRuntime::block_on`, the SAME bridge this file's own `open_directory_stream` already uses
  for its background reconnect loop.
- Replaced `ComputePool::new(plan.compute)` (a value that no longer exists) with a named
  `DIRECTORY_COMPUTE_CAPACITY: u32 = 4` constant, matching the capacity every other `ComputePool::new`
  call site in the repo already uses (`4` — see `semio-framework-os-services`'s own tests and
  `🔌️plugin/🖥️host/⚡️effects/🦀️component.rs`).

### `📦️glue.rs` — the ONE process-wide `WorkerPool` this crate owns, plus `native_shard_count`

New `//#region 🧵️RendererWorkerPool` (crate root, `#[cfg(not(target_arch = "wasm32"))]`): one
`OnceLock<WorkerPool>`-backed `pub(crate) fn renderer_worker_pool() -> WorkerPool`,
`ProcessKind::InteractiveNative`, sized to `available_parallelism()`. This is the injected pool both
`Shell/🧊️component.rs`'s `TokioHostRuntime` and `kernel_runtime::KernelThreadState`'s `ParallelRuntime`
now share — real dependency injection, not a second lazy default. It is a SEPARATE pool object from
`semio-framework-os-services`'s own `global_worker_pool()` singleton (that accessor is crate-private;
cannot be reused here) — an honest compromise in the same shape P1b's own report already accepted for
that crate, not a new anti-pattern: there is exactly one pool for THIS renderer process's whole
lifetime, injected everywhere within this crate that needs one, never re-derived per component.

`kernel_runtime::native_shard_count()` (was `semio_framework_async::thread_plan(cores).shards as u16`,
a deleted symbol) now calls `semio_framework_async::worker_count_for(ProcessKind::InteractiveNative,
cores)` — the exact formula `renderer_worker_pool()` itself sizes from, so shard count and pool worker
count stay derived from the same source even though the shard count is minted slightly before the pool
object is touched at that call site.

`kernel_runtime::KernelThreadState::new()` now builds `Arc::new(crate::renderer_worker_pool())` and
passes it into `ParallelRuntime::new(pool, ..)`, wrapped in `pollster::block_on` (this file's own
pre-existing sync↔async bridge convention, used pervasively elsewhere in this exact file already).

Every `self.runtime.{activate,submit,tick_and_dispatch,complete,unregister}(..)` call site in
`kernel_runtime` (`create_app`, `activate_extensions_of`, `destroy_app`, `run_turn` — 8 call sites) and
in `scale_bench::Env` (`activate_on_lane`, `send_payload_lane`, `pump`, `pump_tracking`, `unregister` —
9 call sites) is now wrapped in `pollster::block_on`, because `ParallelRuntime`'s own methods became
`async fn` (see §3). `kernel()`/`kernel_mut()`/`shard_count()`/`try_recv_outcomes()`/
`wait_for_outcomes()` stayed synchronous — unchanged call sites.

`scale_bench::Env::new` builds its OWN `WorkerPool` (`ProcessKind::HeadlessBatch`) rather than reusing
`crate::renderer_worker_pool()` — deliberate, not an oversight: this bench is a standalone, one-shot
CLI invocation (`semio-wgpu-native --scale`) that never shares a process with the interactive winit
host, mirroring `NativeKernelRuntime`'s own equally-standalone-CLI-caller shape (P1c,
`🖥️host/🎠️activation.rs`). One pool per process's whole lifetime, sized once — not the "component
sizes itself while sharing a process with others that also do" pattern P1e's brief forbids.

### `🎠️runtime.rs` — `ParallelRuntime`, full rewrite

Old `ParallelRuntime` owned K dedicated `ShardExecutor` threads plus K
`"semio-kernel-shard-forward-*"` outcome-forwarder threads (`std::sync::mpsc`-backed fan-in, a
250ms-bounded poll loop) — the exact per-shard-thread architecture P1c deleted from
`semio-framework-plugin-host`'s own `ShardExecutor`/`NativeKernelRuntime`. This file had NOT been
updated to the new API (P1a/P1c's own reports both named it as confirmed-broken, static-grep-only,
blocked on this exact packet).

Rewritten as a near-verbatim mirror of P1c's own `NativeKernelRuntime` (`🖥️host/🎠️activation.rs`) —
that file's own module doc calls itself "a parallel implementation of the same proven pattern" and
names `ParallelRuntime` as the natural next site to receive it. Every method
(`activate`/`submit`/`tick_and_dispatch`/`unregister`/`complete`) is now `async fn`, matching
`Kernel`'s own async surface; shards are `Vec<Arc<ShardExecutor>>`; outcomes flow through one shared
`Arc<OutcomeSink>`, pushed directly by whichever `WorkerPool` worker ran the turn — no forwarder
threads, no `//#region 🔀️OutcomeForwarding` (deleted entirely, ~40 lines: `FORWARD_POLL` const,
`ShardHandle`, the per-shard `std::thread::Builder::spawn` loop, the custom `Drop` impl that used to
join K forwarder threads). `ShardKind::Thread` → `ShardKind::Native` (P1c's rename).

**One deliberate deviation from the `NativeKernelRuntime` template**: `ParallelRuntime::new` takes an
injected `pool: Arc<WorkerPool>` parameter instead of constructing its own — P1a's and P1b's reports
both name this file explicitly as where a caller should inject a real, externally-owned pool instead
of a type building its own. `📦️glue.rs::renderer_worker_pool()`/`scale_bench::Env::new`'s own
`HeadlessBatch` pool are the two injection points (§1/§2 above).

### Cargo.toml

- `semio-framework-trace = { workspace = true }` added to the crate's unconditional `[dependencies]`
  (native AND wasm — the watchdog, §4, runs on every target). Workspace-internal, confirmed
  `bun ./📜️script.ts verify dependencies` stays 238→238.
- Stale doc comment on the native-only `semio-framework-async` dependency line updated (it still named
  the deleted `thread_plan`).

## 2. `ParallelRuntime` — what happened to it

Answered above (§1, "🎠️runtime.rs"): NOT deleted, rewritten. It still owns one `Kernel` and K
`ShardExecutor`s and is still the type both the interactive winit host (`kernel_runtime`) and the
standalone scale-bench harness (`scale_bench::Env`) drive their actors through. What changed is
*how* it gets its concurrency: every shard is now a pool-scheduled job on a caller-injected
`WorkerPool` rather than an OS thread the type spawns itself, and its façade is `async fn` throughout
to match `Kernel`'s own async surface (P1c). The type's own public shape
(`activate`/`submit`/`tick_and_dispatch`/`unregister`/`complete`/`try_recv_outcomes`/
`wait_for_outcomes`/`kernel`/`kernel_mut`/`shard_count`) is unchanged; every method's *signature*
(argument list, return type) is unchanged except `new` gaining the leading `pool` parameter and every
method except the four sync accessors gaining `async`.

## 3. How the renderer obtains the pool

`📦️glue.rs::renderer_worker_pool()` (native-only, `OnceLock`-backed, `ProcessKind::InteractiveNative`)
is the ONE pool this renderer crate mints for its own process. Two consumers inject it explicitly:
`Shell/🧊️component.rs`'s `TokioHostRuntime::with_pool(..)` and `kernel_runtime::KernelThreadState`'s
`ParallelRuntime::new(pool, ..)`. This is a SEPARATE pool object from
`semio-framework-os-services`'s own `global_worker_pool()` (used internally by `ComputePool::new`,
`HttpPool`, `StorageScheduler`, `TimerWheel` — none of which this packet's boundary lets it change, and
whose accessor is crate-private) — so within the whole OS host PROCESS there are, honestly, still two
`WorkerPool` instances alive (this renderer's own, and `os-services`'s internal one, which
`TokioHostRuntime`'s OWN `ComputePool` field still resolves internally regardless of which
`TokioHostRuntime` constructor built it). Closing that gap needs `os-services::global_worker_pool`
to become `pub`, or a process-level bootstrap that constructs ONE pool before either crate touches its
own default — both are edits outside this packet's stated boundary; flagged here rather than
papered over. Within THIS renderer crate's own boundary, "one process-wide pool, injected, never
self-sized per component" is fully true.

## 4. Watchdog instrumentation

`winit_app.rs`'s `OsHost::redraw()` — the sole call site of `AppRuntime::frame()` (confirmed via grep;
`frame()` is called from nowhere else, on native or wasm, since both targets drive the same
`WinitApp`/`OsHost` `ApplicationHandler`) — now wraps the whole `app.frame()` call in a
`semio_framework_trace::Watchdog::start("os_renderer_frame", <op id>, Generation(frame_generation),
InteractiveStage::UiPresent)` guard. `OsHost` gained a `frame_generation: u64` field, incremented once
per call; `winit_app.rs` gained a lazily-allocated, process-unique `OperationId` (one logical
operation — "the render loop" — spanning many generations, one per frame). Any call exceeding
`INTERACTIVE_STEP_CEILING_US` (8ms) is now recorded as a `ContractViolation`, queryable via
`Watchdog::violations()`/`violation_count()` — the exact instrumentation Phase 3's exit gate is meant
to read. `semio-framework-trace` has zero dependencies and already builds clean on
`wasm32-unknown-unknown`/`wasm32-wasip2` (P0a's own report), so this applies uniformly on every target.

## 5. Renderer work still on the UI thread — Phase 3/5 surface, NOT touched here

`AppRuntime::frame()` (`📦️glue.rs`) remains one undivided call doing ALL of: input/hover processing,
chrome layout (`ShellState::render_chrome`), draw-list building/tessellation, font/icon atlas uploads,
and the actual `self.gpu.render_frame(..)` GPU submission — all synchronously on the native winit
thread or the wasm main thread. This packet deliberately did NOT split any of that apart (explicitly
out of scope — "Phase 3/5's job"). The watchdog (§4) makes an overrun of this whole call VISIBLE; it
does not change what runs where. Concretely, still on the UI thread and unmigrated:
- `ShellState::render_chrome` — chrome widget layout and `DrawList` construction.
- `self.gpu.render_frame(..)` — GPU command encoding + submission (`ui_wgpu`'s wgpu target).
- Font atlas (`self.atlas`), icon atlas (`self.icons`), raster texture uploads.
- Tutorial tick, wheel/drag input resolution, hit-testing.

No NEW UI-thread work was introduced by this packet. The `pollster::block_on` bridges added to
`kernel_runtime`/`scale_bench::Env` run on the dedicated `"semio-kernel"` background thread
(`KernelClient::get`'s own thread spawn, pre-existing) or inside the standalone `scale_bench` process
— never on the winit/UI thread. The `TokioHostRuntime::block_on` calls added in `Shell/🧊️component.rs`
run once, synchronously, during `ShellState::new()` (process boot, before the event loop starts) —
same timing the code they replaced already had (a synchronous constructor call), not a new blocking
wait introduced into the steady-state frame loop.

## 6. Cross-boundary breakage found (NOT fixed, outside this packet's boundary)

1. **`✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/build.rs`** — `async fn main()` for a cargo build
   script (illegal; build scripts cannot be async) plus two un-awaited `out_dir()` calls. Confirmed
   pre-existing (already committed at HEAD, dated 2026-08-19, git status shows zero diff) and already
   flagged by both P1a's and P1c's own reports as "an unrelated, in-progress async-conversion sweep
   predating this packet." This is a required (non-dev, non-optional) dependency of
   `semio-framework-os-renderer-wgpu`, so it fully blocks `cargo check`/`cargo build`/`cargo test` for
   the whole renderer crate on every target. **Temporarily patched** (both `async fn` → `fn`, no other
   change) purely to unblock verification of this packet's own edits, then **reverted byte-for-byte**
   before finishing (`diff` against a saved copy confirms identity; `git status` on that file shows no
   change). Left broken exactly as found, per this packet's ownership boundary.
2. **`semio-framework-graph`** (`dsl_core` usage inside `🕸️graph/📦️packages/🦀️rust/../../🗣️dsl/🦀️component.rs`)
   and **`semio-framework-ui`** (`🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/{draw,widgets}.rs`) —
   NEWLY surfaced by this session (not named in any prior Phase 1 report): the same "async-ify" sweep
   left both crates non-compiling — missing/extra `.await` on plain synchronous helpers now returning
   `impl Future<...>` (`Vec3::dot`/`length`/`scale`, `PathSegment` vec building, `WireValue` text
   parsing/rendering, `Shape`/`FieldValue` writer methods). Confirmed pre-existing via `git status`
   showing zero uncommitted diff on any of the affected files (i.e. this is the current, committed
   state of the repo, not a concurrent session's in-flight edit) — reproduced identically on both
   native and `wasm32-unknown-unknown`. `semio-framework-2d` also fails the same way as a further
   downstream consequence. These are required dependencies of `semio-framework-os-renderer-wgpu`
   (`framework_surface_node_graph`, `infinite_canvas`, `ui_wgpu`), so — independent of the puzzle
   build-script issue above — a full `cargo check -p semio-framework-os-renderer-wgpu` cannot reach
   this packet's own files at all right now. Both `🕸️graph/`, `🖱️ui/` are sibling framework modules
   under `🔨️modules/`, outside `📺️renderer/` — not edited here.

## 7. Verified commands (this session)

| Command | Result |
|---|---|
| `cargo check -p semio-framework-async` (transitively re-verified, unedited) | clean |
| `cargo check -p semio-framework-actor --all-targets` (transitively re-verified, unedited) | clean |
| `cargo check -p semio-framework-plugin-host --all-targets` | clean, 0 errors (confirms the `ShardExecutor`/`OutcomeSink` API `🎠️runtime.rs` now calls matches its real shape) |
| `cargo check -p semio-framework-os-services --all-targets` | clean, 0 errors (confirms `TokioHostRuntime::with_pool`/`ComputePool::new`/`block_on` match their real shape) |
| `cargo check -p semio-framework-os-renderer-wgpu --all-targets` | **blocked**: `semio-s-plugin-puzzle` build-script failure (§6.1) |
| `cargo check -p semio-framework-os-renderer-wgpu --all-targets` (puzzle build.rs temporarily de-asynced for verification only, reverted after) | **blocked**: `semio-framework-2d`/`semio-framework-graph`/`semio-framework-ui` compile failures (§6.2), never reaches this crate's own files |
| `cargo check -p semio-framework-os-renderer-wgpu --target wasm32-unknown-unknown` (same temporary bypass) | same §6.2 blocker, target-independent |
| `rustfmt --check --edition 2021` on all 5 edited `.rs` files | 0 parse errors (confirms syntactic validity end to end); diffs shown are pure `rustfmt.toml`-config cosmetics from not passing the repo's own config, no file was reformatted |
| `bun ./📜️script.ts verify dependencies` | clean — 238 → 238 |
| `cargo test` / `cargo test --release` for this crate | **not run** — blocked by §6.1/§6.2, same reason `cargo check` cannot complete |
| `wasm32-wasip2` check | **not run** — same blocker applies transitively (this crate's own `[target.'cfg(target_arch = "wasm32")'.dependencies]` are identical across both wasm targets) |

**Honest verification gap**: this packet's own files (`Shell/🧊️component.rs`, `📦️glue.rs`,
`🎠️runtime.rs`, `🦀️winit_app.rs`, `🦀️os_host.rs`, the crate's `Cargo.toml`) were never reached by
`rustc`'s type checker in this session — both blockers above are confirmed pre-existing and outside
this packet's boundary, not caused by this packet's edits, but that does not substitute for a real
compile. Confidence instead comes from: (a) `semio-framework-plugin-host` and
`semio-framework-os-services` — the two crates whose real API surface this packet's new code calls
into (`ShardExecutor`, `OutcomeSink`, `TokioHostRuntime`, `ComputePool`, `WorkerPool`,
`worker_count_for`, `ProcessKind`) — both compile clean, and every signature used here was read
directly from their current source, not recalled from memory; (b) `🎠️runtime.rs` is a near line-for-line
mirror of P1c's own already-green `NativeKernelRuntime`; (c) every `pollster::block_on` bridge added to
`📦️glue.rs` follows that file's own pre-existing, already-compiling convention verbatim; (d) `rustfmt`
parsed all five files without error. The FIRST thing whoever unblocks §6.1/§6.2 should do is
`cargo check -p semio-framework-os-renderer-wgpu --all-targets` and `cargo test -p
semio-framework-os-renderer-wgpu` (both profiles) against this packet's files specifically.

## Files touched

- Modified: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Shell/🧊️component.rs`
- Modified: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️glue.rs`
- Rewrote: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🎠️runtime.rs`
- Modified: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️winit_app.rs`
- Modified: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️os_host.rs`
- Modified: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/Cargo.toml`
- Temporarily patched then byte-for-byte reverted (verified clean, zero net change):
  `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/build.rs`
- No other files edited (constraint on blast radius honored).
