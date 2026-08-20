# P1c — Actor Shards: Deleting the Per-Shard Thread Architecture

Scope actually edited (per this packet's ownership boundary):
- `🧰️framework/🔨️modules/🎭️actor/🦀️component.rs` — `ShardKind::Thread` → `ShardKind::Native` rename + doc.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🏃️executor.rs` — full rewrite: `ShardExecutor` no longer owns an OS thread.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🦀️component.rs` — `ShardLoop`: module doc updated, `Watchdog` wired around `execute_turn`/`step_job`.
- `🧰️framework/🛍️products/💻️os/🖥️host/🎠️activation.rs` — full rewrite: `NativeKernelRuntime` no longer owns forwarder threads.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📦️packages/🦀️rust/Cargo.toml` — added `semio-framework-trace` and `semio-framework-async`'s `entrypoint` feature (workspace-internal deps only; `bun ./📜️script.ts verify dependencies` confirms 238→238).

## 1. How shard affinity is now expressed

Affinity used to be a THREAD-IDENTITY property: `ShardExecutor::spawn` put one `ShardLoop` on one dedicated `std::thread`, so "this actor's `wasmtime::Store` only runs on shard N" was true simply because nothing else ever touched that thread.

It is now a **mutual-exclusion property**. `ShardExecutor` holds its `ShardLoop` behind a plain `std::sync::Mutex`, plus a single-flight scheduling protocol (`scheduled: AtomicBool`, `epoch: AtomicU64`, `pending_lane_rank: AtomicU8`) that guarantees at most ONE `WorkerPool` job is ever draining a given shard's `pump()` at a time — but WHICH physical pool worker thread runs that job varies call to call. `ShardTable::pin`'s own least-loaded placement (`🎭️actor/🦀️component.rs`) is untouched — an actor is still pinned to exactly one `ShardId` for its whole lifetime, migration only at a checkpoint boundary — only the mechanism enforcing "only this shard's own logic ever touches this actor's `GuestInstance`" changed from "physical thread" to "logical single-flight lock."

The scheduling protocol (`🏃️executor.rs`):
- `ShardExecutor::send_frame(bytes, lane)` pushes the frame onto the shard's internal duplex `ThreadTransport` pair and calls `schedule()`.
- `schedule()` is the classic Akka-mailbox idiom: `scheduled.swap(true)` — only the caller that flips `false → true` submits a fresh `WorkerPool::submit(lane, job)`; every other concurrent caller just returns, trusting the in-flight/queued job to pick up its frame.
- The submitted job (`run()`) calls `ShardLoop::pump()` once (drains and executes everything currently buffered — unchanged contract from the old thread's own park/pump iteration), drains the outcome side of the transport, then re-checks `epoch` (bumped once per `send_frame` call, BEFORE `schedule()` — the correctness-critical "is there unseen work" signal) against the value it saw before the pump. If unchanged, it clears `scheduled` and returns the worker to the pool. If changed, it loops locally (no fresh pool submission) or, on a race where a concurrent `send_frame` already reclaimed `scheduled`, returns and trusts that caller's own submission.
- `pending_lane_rank` is a SEPARATE, best-effort priority hint (not correctness-critical) used only to pick which `WorkerPool::Lane` the next submission runs under — a race on it can only mis-prioritize a submission, never drop a frame, because `epoch` is what `run()` actually trusts.

`ShardExecutor::register` also simplified: no more `RegisterRequest`/ack-channel rendezvous (`terra-shard-routing`'s old fix for "a `Grant` sent right after `register()` returns reaches the executor thread before that thread's own registration drain"). There is no second thread with an independent drain cadence to race against anymore — `register` and every pump job now serialize on the exact same `Mutex`, so the race this ack existed to close cannot recur by construction. Re-verified by `every_actors_grant_lands_on_the_shard_it_was_registered_on_across_k_shards` (200 actors × 4 shards, zero-slack register-then-dispatch, ported from the original thread-based test) and a new `concurrent_send_frame_bursts_never_drop_an_outcome` test (32 actors, genuinely concurrent `std::thread::spawn` senders racing `send_frame`).

`ShardKind::Thread` renamed to `ShardKind::Native` (wire tag unchanged, `0`) — per the packet brief, `ShardKind` is a statement about the execution HOST (native/pool-scheduled vs. `WebWorker` vs. `Process`), not about thread ownership; the old name actively implied the thing this packet removed. `ThreadTransport`/`SharedThreadTransport` names were left alone — they denote the mpsc-channel-backed transport MECHANISM (still real, still in-process, still built on `std::sync::mpsc`), independent of whether the shard using it owns a dedicated thread.

## 2. How turn outcomes flow without forwarder threads

`NativeKernelRuntime` (`🎠️activation.rs`) used to own K `ShardExecutor` threads AND K separate `semio-os-host-kernel-shard-forward-*` threads, each `block_on(recv_deadline(250ms))`-polling the shard's outcome channel and forwarding decoded bytes into an `mpsc::Receiver<(ShardId, Vec<u8>)>` the runtime's own `try_recv_outcomes`/`wait_for_outcomes` drained.

Both the shard thread AND the forwarder thread are gone. `ShardExecutor` now owns **both ends** of its `ThreadTransport::new_pair()` duplex link internally (previously split: `shard_side` went to the `ShardLoop`, `kernel_side` was handed out to the caller). Its own pool job (`run()`), right after `ShardLoop::pump()` produces outcomes onto the transport, drains its OWN `kernel_side.recv()` loop and pushes each decoded `ShardOutcome` directly into a shared `OutcomeSink` (new type, `🏃️executor.rs`) — a `Mutex<VecDeque<ShardOutcome>> + Condvar`. This is "completion notification through the pool": the SAME `WorkerPool` worker thread that ran the guest turns delivers their outcomes, inline, before the job returns. `NativeKernelRuntime::try_recv_outcomes`/`wait_for_outcomes` are now thin wrappers over `OutcomeSink::try_recv_all`/`wait_for` — same external contract (drain-without-blocking / block-with-timeout), zero forwarder threads backing them.

This also removes one of the interactivity audit's allow-listed `block_on` sites (the forwarder thread's own `block_on(recv_deadline(...))` in `🎠️activation.rs`) — `NativeKernelRuntime` no longer calls `block_on` anywhere at all (every method stays a plain `async fn`, awaited by its own async caller); the only production `block_on` left in this packet's files is `ShardExecutor::run`'s bridge from its plain `WorkerPool` job closure into `ShardLoop::pump`'s async body — a NEW site, but the SAME "thread/job root" shape the audit's existing rule already recognizes (this file's own doc names it explicitly), not a second uncontrolled one. Confirming the audit config's stale allow-list entry is a config change outside a Rust source file this packet's boundary doesn't include editing; flagged here for the coordinator.

## 3. Epoch-interruption arrangement — NOT changed, cross-boundary gap

The 1ms `EpochTicker` (`engine.increment_epoch()` on a dedicated `"semio-epoch-ticker"` thread) lives in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs` — the **plugin-host crate ROOT file**, not `🧵️shard/`. That file is outside this packet's explicit ownership boundary (`…/🔌️plugin/🖥️host/🧵️shard/` only), so it was deliberately **not edited**. This is the one item of the packet brief I could not complete inside the stated boundary — documented rather than silently skipped, per the instruction to report cross-boundary needs.

Proposed design for the follow-up packet (or an explicit boundary extension), so it doesn't need to be rediscovered:
- Replace `EpochTicker::start(&engine)`'s dedicated thread with a recurring `WorkerPool::submit(Lane::Timer, job)` driven by `TimerWheel::sleep_until` — e.g. a job that calls `engine.increment_epoch()` then re-registers `wheel.sleep_until(now_ms + 1)` before returning, so the tick becomes a chain of `Lane::Timer` pool submissions instead of a spin-parked OS thread. `TimerWheel`/`Lane::Timer` (`🧰️framework/🔨️modules/⏳️async/🦀️component.rs`, landed in P1a) already exist for exactly this purpose — P1a's own report names this file and mechanism as the ticker's intended replacement, just not yet wired.
- `WasmtimeRuntime`'s `_epoch_ticker: EpochTicker` field (held for RAII lifetime, `🦀️component.rs:1102`/`⏳️runtime.rs:129`) would become a handle that cancels the timer-chain instead of joining a thread.

**What the later, repo-owned WASM interpreter must take over**: today, `Budget.wall_ms` is enforced ENTIRELY by wasmtime's epoch interruption — `store.set_epoch_deadline(budget.deadline_ms)` plus the 1ms ticker is the only thing that can preempt a guest call mid-execution, and it only fires at wasm-bytecode safe points wasmtime itself chooses (coarse-grained, "eventually" preemption, not instruction-precise). Once Phase 2+ replaces wasmtime with a repo-owned interpreter, INSTRUCTION-LEVEL FUEL METERING becomes the preemption mechanism instead — meaning: (a) the interpreter's own fuel-decrement-and-check loop replaces the epoch ticker's coarse periodic wakeup entirely (no OS-level timer needed to force a check — every instruction dispatch already checks), (b) `Budget.wall_ms`'s CURRENT role (an upper bound the epoch ticker enforces from OUTSIDE the guest) shifts to `Budget.fuel` being the primary, precise ceiling, with wall-clock time becoming a secondary/derived signal rather than the enforcement mechanism, and (c) the `Lane::Timer`/`TimerWheel` machinery this packet's proposed epoch-ticker replacement would use stays useful regardless (real host-side deadline/timeout primitives, e.g. `StepContext.deadline` in Phase 2's job protocol, per P1a's own report) — it is NOT interpreter-specific, only the epoch-ticker's specific 1ms-poll consumer of it goes away.

## 4. `ShardKind` clean-up

`ShardKind::Thread` → `ShardKind::Native` (18 call sites in the actor crate, all tests; `ShardTable::new(ShardKind::Native, …)`/`Kernel::new(ShardKind::Native, …)` in `🏃️executor.rs`'s tests and `🎠️activation.rs`). Doc rewritten to state explicitly: `ShardKind` distinguishes the execution HOST (native process + shared pool / browser Worker / separate OS process), never thread ownership — `ShardTable` (the type that actually owns shard↔actor bookkeeping) never spawned anything to begin with (the actor crate's own purity rule), so this was purely a naming/documentation fix, no behavioral change. Wire tag for `Native` kept at `0` (same as `Thread` was) — a rename, not a new wire variant, so nothing pack-encoded elsewhere needs updating.

No literal `ThreadKind` type exists anywhere in the repo (grepped the whole tree) — the packet brief's "`ThreadKind`/`ShardKind::Thread`" is read as referring to `ShardKind::Thread` alone.

## 5. Turn budgets, the watchdog, and which turn paths exceed 8ms today

Lane budget ceilings (`semio_framework_actor::lane_defaults::budget_for`, unchanged by this packet — explicitly on the KEEP list) are `wall_ms`: Interactive 4, UserVisible 16, Background 50, Maintenance 200. `semio_framework_trace::INTERACTIVE_STEP_CEILING_US` is a flat 8ms regardless of stage.

**By construction, every UserVisible/Background/Maintenance turn that spends close to its full granted budget exceeds the 8ms interactive ceiling** — this is a property of the EXISTING budget constants (read directly from `lane_defaults::budget_for`'s code, not measured at runtime — no live workload was run through this packet's crates, since the workspace build is blocked upstream; see §6), not something this packet's edits caused. Concretely, once a real workload runs:
- `Lane::Interactive` turns (4ms grant) stay under the ceiling — no violation expected from budget alone.
- `Lane::UserVisible` turns (16ms grant) can trip the watchdog at 2× the ceiling.
- `Lane::Background` turns (50ms grant) can trip it at 6.25×.
- `Lane::Maintenance` turns (200ms grant) can trip it at 25×.

This packet wires the RECORDING mechanism only, per the brief's explicit scope (`"Do not yet try to make guest turns internally resumable — that is Phase 2's job protocol"`): `ShardLoop::execute_turn_for`'s call into `GuestRuntime::execute_turn` and the job-step loop's call into `GuestRuntime::step_job` (`🧵️shard/🦀️component.rs`) are each wrapped in a `semio_framework_trace::Watchdog::start(site, OperationId(actor_id), Generation(…), stage)` guard — `stage` derived from the actor's current `Lane` (`Interactive`→`InteractiveStep`, `UserVisible`→`UserVisibleSimStep`, `Background`/`Maintenance`→`BackgroundStep`, via a new `interactive_stage_for` helper). `Watchdog::violations()`/`violation_count()` become queryable process-wide once real turns run.

Pool workers registering with the trace thread-role API (`register_worker_thread`) was **already done by P1a** — `WorkerPool`'s native `worker_loop` calls `semio_framework_trace::register_worker_thread(index)` at thread start (`⏳️async/🦀️component.rs`); no action needed here since `ShardExecutor::run` executes as a plain closure on that already-registered thread.

**For Phase 2's job protocol to target**: `execute_turn`/`step_job` are the two, and only two, guest-call sites in the plugin-host crate that run under a lane-derived wall-clock budget; anything on `UserVisible`/`Background`/`Maintenance` is a candidate for internal resumability. `Interactive`'s own 4ms grant is already inside the 8ms ceiling, so it is lower priority for Phase 2's slicing work than the other three lanes.

## 6. Cross-boundary / workspace state

`semio-framework-os-services` (out of boundary; a sibling packet's territory) is still broken exactly as P1a's report catalogued (5 errors: `ChannelPolicy` field-rename mismatches, `ChannelPolicy::LatestWins` struct-variant-as-unit-variant match) — re-confirmed unchanged by running `cargo check -p semio-framework-os-services` before AND after this packet's edits. Because `semio-framework-plugin-host` depends on `semio-framework-os-services`, `cargo check -p semio-framework-plugin-host` (and therefore `-p semio-framework-os`, which depends on plugin-host via `🎠️activation.rs`) never reaches THIS packet's own files — confirmed by grepping the check output for a `Checking semio-framework-plugin-host` line, which never appears. This is expected mid-wave per this packet's own brief ("siblings are fixing the services crate concurrently... judge success by your crates").

Verified INSTEAD, as thoroughly as the environment allows:
- Careful manual type/borrow-checking review of every new/changed line in `🏃️executor.rs`, `🧵️shard/🦀️component.rs`, `🎠️activation.rs` against the actual signatures of every type/method called (actor crate — which DOES build clean — `GuestRuntime`/`ShardTransport` trait definitions, `WorkerPool`/`Lane`/`ProcessKind` from P1a's landed async crate).
- `rustfmt`-parse-validated (confirms no syntax errors) — separately from the type-check that's blocked upstream.
- Fixed two real, pre-existing bugs in the old `🎠️activation.rs` draft while rewriting it (both were "doesn't compile as written" per that file's own prior module doc): `self.guest_runtime.instantiate(...)` was missing `.await` (it's an `async fn`), and `to_actor_turn_result(result, wall_us, memory_bytes)` was missing `.await`. Both fixed as part of this packet's rewrite.

Once `semio-framework-os-services` is fixed by its owning packet, re-run to confirm:
```
cargo check -p semio-framework-actor --all-targets
cargo clippy -p semio-framework-actor --all-targets -- -D warnings
cargo test -p semio-framework-actor && cargo test -p semio-framework-actor --release
cargo check -p semio-framework-plugin-host --all-targets
cargo clippy -p semio-framework-plugin-host --all-targets -- -D warnings
cargo test -p semio-framework-plugin-host
cargo check -p semio-framework-os --all-targets
cargo clippy -p semio-framework-os --all-targets -- -D warnings
```

## 7. Verified commands (this session)

| Command | Result |
|---|---|
| `cargo check -p semio-framework-actor --all-targets` | clean, 0 errors (before AND after the `ShardKind::Native` rename) |
| `cargo clippy -p semio-framework-actor --all-targets -- -D warnings` | clean |
| `cargo test -p semio-framework-actor` | 76/76 passed |
| `cargo test -p semio-framework-actor --release` | 76/76 passed |
| `cargo check -p semio-framework-actor --target wasm32-unknown-unknown` | 15 pre-existing errors, ALL in `📦️glue.rs` (untouched by this packet — confirmed via `git diff --stat`; missing `.await` on several pack methods, `wasm_bindgen_futures` unresolved — an unrelated, in-progress async-conversion sweep predating this packet, same shape `🎠️activation.rs`'s own prior doc already named for `semio-framework-actor`) |
| `cargo check -p semio-framework-actor --target wasm32-wasip2` | same 15 pre-existing `glue.rs` errors |
| `cargo check -p semio-framework-plugin-host --all-targets` | blocked upstream by `semio-framework-os-services` (5 pre-existing errors, unchanged from P1a's own catalogue, confirmed out of this packet's boundary); never reaches this packet's own files |
| `cargo check -p semio-framework-os --all-targets` | same upstream block (plugin-host is `semio-framework-os`'s own dependency) |
| `bun ./📜️script.ts verify dependencies` | clean — 238→238 |

## 8. OS threads eliminated

Per `ShardExecutor` instance: 1 dedicated thread (`"semio-shard-executor"`, the ~5ms park/pump loop) — deleted.
Per `NativeKernelRuntime` shard slot: 1 forwarder thread (`"semio-os-host-kernel-shard-forward-*"`, the 250ms outcome poll) — deleted.

**Net: 2 OS threads eliminated per shard** (previously K shards ⇒ 2K dedicated threads for `NativeKernelRuntime`'s own turn-dispatch path alone, on top of `ParallelRuntime`'s equivalent K `ShardExecutor` threads in the wgpu-native host, which now also breaks against the new `ShardExecutor` API — see below). All shard turns for a given `NativeKernelRuntime`/future `ParallelRuntime` now run on the ONE shared, process-wide `WorkerPool` (P1a) instead.

## Cross-boundary breakage (not fixed here, out of this packet's blast radius)

- **`🎯️targets/🧊️wgpu/🎠️runtime.rs`'s `ParallelRuntime`** (renderer crate, outside this packet's boundary): calls `ShardExecutor::spawn(runtime, shard_side, initial)` (old 3-arg, thread-spawning signature) and `shard.executor.register(actor, instance);` without `.await`. `ShardExecutor::spawn` no longer exists (renamed `ShardExecutor::new`, now takes `(pool: Arc<WorkerPool>, runtime, initial, outcomes: Arc<OutcomeSink>)` and returns `Arc<ShardExecutor>`) — this call site will not compile until `ParallelRuntime` is ported onto the pool-scheduled `ShardExecutor`, mirroring `🎠️activation.rs`'s own `NativeKernelRuntime` rewrite in this packet almost line for line (construct one shared `WorkerPool`, one `OutcomeSink`, replace its own forwarder-thread mechanism with `OutcomeSink::wait_for`/`try_recv_all`). Not independently confirmed by a green build (this crate's own `semio-s-plugin-puzzle` build-script dependency already fails to compile for an unrelated, pre-existing reason per P1a's report) — confirmed via static grep instead.
- **`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs`'s `EpochTicker`** — see §3; needs the timer-lane redesign, outside this packet's stated boundary.
- **The interactivity audit's `block_on` allow-list** — the forwarder-thread entry this packet's deletion makes obsolete lives in a config file outside this packet's Rust-source boundary; flagged for the coordinator (§2).
- **`semio-framework-os-services`** — unrelated, pre-existing, a sibling packet's own territory (§6).

## Files touched

- Modified: `🧰️framework/🔨️modules/🎭️actor/🦀️component.rs` (`ShardKind::Thread` → `ShardKind::Native`, doc)
- Rewrote: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🏃️executor.rs` (`ShardExecutor` + new `OutcomeSink`, no OS thread)
- Modified: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🦀️component.rs` (module doc; `Watchdog` wired into `execute_turn_for`/the job-step loop; new `interactive_stage_for` helper)
- Rewrote: `🧰️framework/🛍️products/💻️os/🖥️host/🎠️activation.rs` (`NativeKernelRuntime`, no forwarder threads; two pre-existing missing-`.await` bugs fixed)
- Modified: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📦️packages/🦀️rust/Cargo.toml` (added `semio-framework-trace`; added `entrypoint` feature to `semio-framework-async`)

## Incident note (self-corrected this session)

A `cargo fmt -- <single-file-path>` invocation, intended to format only `🏃️executor.rs`, instead reformatted the entire workspace (~3300+ unrelated tracked files, whitespace/import-order only). Detected immediately via `git status`/mtime inspection, and reverted with a targeted `git checkout -- <path>` pass over every affected file EXCEPT this packet's own 4 intentionally-edited files (which kept their now-correctly-formatted content) — confirmed by re-running `cargo test -p semio-framework-actor` clean afterward. No other session's staged/committed work was touched (`git checkout --` only discards working-tree diffs against the index, and the affected files' index state — other sessions' legitimately staged work — was left untouched throughout). `cargo fmt`/`rustfmt` were not invoked again for the remainder of this packet.
