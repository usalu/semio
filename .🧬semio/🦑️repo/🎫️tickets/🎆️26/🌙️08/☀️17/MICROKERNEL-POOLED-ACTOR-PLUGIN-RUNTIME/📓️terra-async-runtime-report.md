# ⏳️ terra-async-runtime — `WasmtimeAsyncRuntime` (one root task per actor)

Executor: `terra-async-runtime`. Owned path: exactly one NEW file,
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/⏳️runtime.rs` (544 lines).

## mount line (for sol to apply to `🦀️component.rs`)

Insert immediately after the existing `pub mod imports;` mount (currently lines 15-20):

```rust
// 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-async-runtime): WasmtimeAsyncRuntime — one root
// task per actor (`AsyncActorTask::spawn`), driving `imports.rs`'s host-async import layer against a
// real `Store`. See that module's own doc.
#[path = "⏳️runtime.rs"]
pub mod runtime;
```

## the one required lease (blocking, not optional)

`⏳️imports.rs` line 43 declares `mod host_async_bindings { ... }` (private). `runtime.rs` needs the
generated world struct (`ActorAsync`), its `add_to_linker`, and the `events::Event` type to build the
`Linker`, instantiate, call `runner::run`, and type the event-stream producer. **Requesting exactly
one visibility change**, no other edit to that file:

```rust
mod host_async_bindings {   // was: (implicitly private)
```
→
```rust
pub(crate) mod host_async_bindings {
```

Nothing else in `imports.rs` needs to change — `AsyncActorHostState` and its methods (`new`,
`take_effects`, `take_patches`) are already `pub`.

## what this file does

- `AsyncEngineHandle` — one async-capable `Engine` (`wasm_component_model_async` +
  `concurrency_support` + `consume_fuel` + `epoch_interruption`, distinct from `build_shared_engine`'s
  sync-world engine), reusing `crate::EpochTicker`, plus a `Linker` with WASI-async and
  `host_async_bindings::ActorAsync::add_to_linker` wired.
- `GrantWindow` / `GrantedEventProducer` — a `StreamProducer<AsyncActorHostState>` that parks even
  when its internal queue is non-empty, whenever the current grant's event budget (`remaining`) hits
  zero — "a grant is delivery permission plus a refill, not a thread." Notifies an `exhausted: Notify`
  the instant a grant is spent.
- `GrantHandle` — the caller-facing refill API (`refill(TurnGrant)` / `close()`), safe to call from a
  different task/thread; cannot touch the `Store` directly (nothing outside the accessor closure can),
  so it stages `pending_budget` and wakes the control loop, which applies it.
- `DeadlineCell` / `install_epoch_budget` — the epoch callback returns `Yield(1)` while
  `Instant::now()` is before the current grant's wall-clock deadline, `Interrupt` once it has passed.
  Installed ONCE per `Store` (wasmtime allows exactly one `epoch_deadline_callback`), reading a shared
  `Mutex<Instant>` fresh every tick so one callback serves every grant across the actor's whole life.
- `synthesize_turn_result` — builds a `semio_framework::kernel::TurnResult` ENTIRELY host-side (fuel
  delta + `state.take_effects()`, already kernel-`Effect`-shaped by `imports.rs`'s own `emit` handler
  + a caller-supplied `TurnStatus`) — never calls into the guest. This is what makes mid-grant turn
  synthesis possible without the guest itself returning from anything.
- `AsyncActorCommand` / `AsyncActorTask` — one `Store<AsyncActorHostState>` per actor
  (`AsyncActorTask::spawn`), a `tokio::spawn`ed task that OWNS the store, instantiates once, then runs
  `store.run_concurrent(async move |accessor| { loop { select!(&mut call_run_fut, exhausted-notify,
  refilled-notify, commands) } })`. Emits `AsyncTurnOutcome::Turn`/`Finished` (raw
  `semio_framework::kernel::TurnResult`) over an outbound channel — the caller's next hop is
  `to_actor_turn_result` + `Kernel::complete`, EXACTLY the existing `ShardOutcome::Turn` →
  `ParallelRuntime::complete` bridge (`🎯️targets/🧊️wgpu/🎠️runtime.rs`), not a new mechanism. This file
  never calls `Kernel::complete` itself, matching that precedent (the shard executor doesn't either).

## budgets

- **Fuel**: `set_fuel(grant.budget.fuel)` at construction and on every refill (applied from inside the
  control loop via `access.as_context_mut().set_fuel(...)` — the only place a `StoreContextMut` is
  reachable once `run_concurrent` owns the store). No `fuel_async_yield_interval` — fuel is a hard
  per-grant ceiling (exhaustion traps → `Faulted`), not a cooperative-yield lever; that job belongs to
  epoch alone.
- **Epoch**: one wall-clock `Instant` deadline per grant (`DeadlineCell`), `Yield(1)` below it,
  `Interrupt` past it. `AsyncActorTask::spawn` REQUIRES an initial `TurnGrant` up front (mirrors
  `ShardExecutor::spawn`'s own `initial` list) so this file never encodes "no deadline" as a sentinel —
  see the harness finding below on why that matters.

## cancellation path

`AsyncActorTask::cancel(self)` calls `JoinHandle::abort()` THEN awaits the handle. This is deliberate,
not decorative — see harness tests D/E below: dropping the `run_concurrent` future ALONE does **not**
cancel an in-flight host import; the `Store` must be dropped too. `AsyncActorTask::spawn` constructs
the `Store` **inside** the spawned task body specifically so `abort()` drops the future and the Store
together in one shot. Guest future-drop (an individual host import call, via `CancelOnDrop`) is
`imports.rs`'s own established mechanism, unchanged and reused here at the "second export call"
(checkpoint/job) level via `accessor.spawn` + oneshot, matching S1's own proven idiom.

## checkpoint / jobs — schema history mid-packet, now unblocked

Two corrections arrived from the coordinator while this packet was in flight, both incorporated:

1. **First correction**: do not touch `component.wit`; build against sync `checkpoint`/`jobs` exactly
   as they stood, and record any dependency as a blocked seam rather than working around it.
2. **Second correction (post-S7, supersedes the first)**: S7 settled the open question — a plain sync
   `func` export is **uncallable at all** on a `wasm_component_model_async(true)` store (not merely
   unsafe when concurrent; fails identically on an idle store). The coordinator is landing
   `checkpoint-async`/`jobs-async` (`async func` throughout) and asked this file to be written against
   that shape.

`runtime.rs` now implements `AsyncActorCommand::{Checkpoint, Restore, StartJob, StepJob, CancelJob}`,
each dispatched as a short-lived `AccessorTask` + oneshot reply via `accessor.spawn` — proven pattern,
see harness test F. **What remains genuinely unverified** (not blocked, just not yet checkable): the
WIT interfaces `checkpoint-async`/`jobs-async` do not exist in `🧬️schema/📜️component.wit` as this
file is written (the coordinator is sequencing that change separately), so:
- the predicted generated accessor names (`instance.semio_framework_checkpoint_async()`,
  `instance.semio_framework_jobs_async()`) are extrapolated from the SAME naming convention this
  packet confirmed empirically for `runner`/`checkpoint-async` in its own scratch harness
  (`instance.semio_runtimeprobe_runner()` / `instance.semio_runtimeprobe_checkpoint_async()`), not
  independently checked against the real schema;
- `JobBudgetArg`/`JobStepResult` (local plain-Rust types) mirror the CURRENT `job-budget`/`job-step`
  WIT shapes (`fuel: u64, deadline-ms: u32` / `running|done|failed`) field-for-field, but the real
  generated bindgen types once `jobs-async` lands may not be named or shaped identically — a real
  compile will need to reconcile these.

## harness — six results proven against real wasmtime 47.0.3, not inferred

Standalone scratch project (never in-tree), guest world `runtime-probe` + host binary, both built and
RUN (`wasmtime 47.0.3`, `wit-bindgen 0.57.1`, `rustc 1.99.0-nightly`, target `wasm32-wasip2`,
`CARGO_TARGET_DIR=<scratchpad>/target-runtime-harness` per rule 24). Full sources copied to this
ticket folder: `terra-async-runtime-harness-world.wit.txt`,
`terra-async-runtime-harness-guest-component.rs.txt`, `terra-async-runtime-harness-host-main.rs.txt`.
Verbatim run output (exit 0): `terra-async-runtime-harness-run.txt`.

```
[host] A+B PASS: runner::run via named interface, grant-gated over 3 refills, summed = 66
[host] C PASS: Yield(1) preempted 24 times within budget, Interrupt trapped cleanly once the deadline passed
[host] D: hang_dropped after dropping the run_concurrent future alone = false, after ALSO dropping the Store = true
[host] E PASS: JoinHandle::abort() on a task that OWNS its Store cancels an in-flight import
[host] F PASS: checkpoint-async's named-interface export answered via accessor.spawn while the root run() call was still in flight
[host] ==== ALL PASS (A/B/C/D/E/F) ====
```

| # | proved | why it mattered |
|---|---|---|
| A | An `async func` export living inside a NAMED `interface` (`interface runner { run: ... }`) is called via `instance.semio_runtimeprobe_runner().call_run(accessor, events)` — the SAME accessor-taking shape as a bare world-level export. | No prior spike in this ticket tested a named-interface async export — every S1-S8 export was declared at world scope. Production's `runner`/`checkpoint-async`/`jobs-async` are all named interfaces. |
| B | `GrantedEventProducer` parks even when its queue holds MORE items than the current grant allows (3 events queued, budget-of-1 refill releases exactly 1, not all 3). | The proven `ChunkStreamProducer`/`WakeyProducer` (S5/imports.rs) only ever park on a genuinely empty queue — a strictly weaker property than "parks past the grant." |
| C | `epoch_deadline_callback` returning `Yield(1)` while `Instant::now()` < deadline, `Interrupt` once past it: 20-24 real Yields observed before a clean trap on `Interrupt`, message contains "interrupt". | The mission's literal budget-wiring instruction, not separately tested by any prior spike (S1c only ever returned `Yield(1)` unconditionally). |
| D | Dropping the `run_concurrent` future ALONE does **not** cancel an in-flight host import (`hang_dropped` stayed `false`); dropping the `Store` too flips it `true`. | A genuine, non-obvious finding that directly shaped `AsyncActorTask`'s design — see below. |
| E | The PRODUCTION shape — `Store` moved INTO a `tokio::spawn`ed async block, `JoinHandle::abort()` on that task — DOES fully cancel an in-flight import in one call. | Confirms `AsyncActorTask::spawn`'s "construct the Store inside the spawned body" design actually achieves cancellation; D alone would have been a landmine if not followed up. |
| F | `checkpoint-async`'s (named-interface) `checkpoint` answered via `accessor.spawn` (S1's own `AccessorTask`+oneshot idiom) WHILE `runner::run` was still parked mid-stream, never given a single event — then `run()` completed normally afterward. Re-run with the ONE instance `Arc`-wrapped and shared into the spawned task (rather than S1's own double-`instantiate_async` workaround) — confirmed the generated instance struct is `Send + Sync`. | Directly reproduces the coordinator's post-S7 finding (second concurrent call on the same instance) against a NAMED interface, and settles `runtime.rs`'s own `instance: Arc<ActorAsync>` design choice (one instance, not N) with evidence rather than copying S1's workaround blind. |

## in-tree compilation

**UNRUN**, and cannot be run as things stand — two independent reasons, both explained above, neither
a defect in this file:
1. `host_async_bindings` is private in `imports.rs` (the one lease requested above).
2. `checkpoint-async`/`jobs-async` do not exist in the schema yet (coordinator sequencing separately).

`rustfmt --check --edition 2021` on the file: exit 1, **reformatting diffs only** (long single-line
match arms/struct literals vs rustfmt's wrapping — same style `imports.rs`/`⚡️effects/🦀️component.rs`
already carry unformatted) — confirms the file **parses** cleanly; rustfmt only emits a diff after a
successful parse. Verbatim: `terra-async-runtime-rustfmt-check.txt`.

## honest gaps

- **The exact `select!` composition inside `AsyncActorTask::spawn` (racing `call_run_fut` against
  exhausted/refilled/commands in one loop) was designed from, but not itself separately re-run in, the
  harness** — the harness proves each primitive (A-F) individually; the specific 4-armed
  `tokio::select!` loop shape is standard tokio idiom (a pinned long-lived future in one arm, fresh
  futures in the others) but was not built as its own 7th harness test. Time/effort tradeoff, flagged
  rather than hidden.
- **`checkpoint-async`/`jobs-async` generated names and `JobBudgetArg`/`JobStepResult` shapes are
  predictions**, not verified against a real schema — see `## checkpoint / jobs` above.
- **`ui_patches: Vec::new()` and `next_wake: None`** in `synthesize_turn_result` are the SAME
  pre-existing open gaps `🦀️component.rs`'s own `execute_turn` and `imports.rs`'s own `patch_sink` doc
  already carry (WIT patch-op path/node encoding has no agreed kernel conversion yet; no per-grant
  timer-request signal reaches this file). Not new gaps, not silently invented values.
- **Fuel exhaustion traps rather than gracefully pausing for a refill.** `fuel_async_yield_interval`
  would allow a CPU-heavy actor to pause instead of fault when it burns through a grant's fuel without
  ever touching the event stream — deliberately not used here (S1b's Q3 showed it works but gives no
  host-visible hook to detect the pause point, unlike epoch's callback), so a CPU-bound actor that
  exhausts its fuel mid-grant currently synthesizes a `Faulted` turn, not a graceful `MoreWork` one.
  Documented tradeoff, not an oversight — a future packet chasing finer CPU-bound scheduling should
  revisit this.
- **No multi-actor scheduler is built here** — `AsyncActorTask` is the one-actor-per-`Store` unit
  S1b/S1c's shape calls for; a caller (kernel-loop-equivalent for async actors, the async analogue of
  `ParallelRuntime`) `tokio::spawn`s one per actor and drains their `AsyncTurnOutcome` channels. That
  caller, and the real `to_actor_turn_result`/`Kernel::complete` wiring, is explicitly the NEXT
  packet's job per the mission.
- **`Kernel::complete` is never called from this file** — by design, mirroring `ShardLoop`/
  `ShardExecutor` (which also never call it — `ParallelRuntime` does). `AsyncTurnOutcome` carries the
  raw `semio_framework::kernel::TurnResult`, the same type `ShardOutcome::Turn` carries, so the
  existing bridge applies unchanged.

## files touched

- **NEW** `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/⏳️runtime.rs` (544 lines) — this
  packet's entire deliverable.
- `TICKET_DIR/terra-async-runtime-harness-world.wit.txt`,
  `-harness-guest-component.rs.txt`, `-harness-host-main.rs.txt`, `-harness-run.txt`,
  `-rustfmt-check.txt` — scratch evidence (this ticket folder; harness itself built and run entirely
  under `<scratchpad>/harness` + `<scratchpad>/target-runtime-harness`, per rule 24, not persisted).
- Nothing else. `🦀️component.rs` was read but not edited (mount line handed to sol above);
  `⏳️imports.rs` was read but not edited (one lease requested above); no other file in the repo was
  modified.
