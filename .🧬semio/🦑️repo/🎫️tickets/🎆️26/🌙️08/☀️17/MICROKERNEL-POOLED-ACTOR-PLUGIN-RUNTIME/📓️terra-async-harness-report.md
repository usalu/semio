# 🧪️ terra-async-harness-spike (async-harness-spike) — Q1-Q6 re-proof on the REAL schema shape

Executor: `terra-async-harness-spike`. Everything below was built and RUN against real **wasmtime
47.0.3**, target `wasm32-wasip2`, `wit-bindgen 0.57.1`, `rustc 1.99.0-nightly` — never inferred from
source reading alone. Exit codes and pasted output are in `## commands + exit codes` and the copied
`terra-asyncharness-*.txt` files in this folder.

## fixture location chosen, and why

Owned path used: **`🧰️framework/🛍️products/💻️os/🧫️fixtures/🔌️asyncprobe/👽️guest-turn/` +
`🧰️framework/🛍️products/💻️os/🧫️fixtures/🔌️asyncprobe/🖥️host-turn/`** — two NEW sibling directories
inside the **same** `🔌️asyncprobe` fixture root the earlier `terra-probe-spikes` (S1-S9) packet
created, rather than a brand-new top-level fixture. Did **not** extend the existing `👽️guest`/
`🖥️host` pair directly: that pair's `wit_bindgen::generate!`/`bindgen!` calls are pinned to `world
asyncprobe`, which is a different, already-proven WIT world (S1-S9's shapes) — adding a structurally
different "reduced turn-shaped world" into the same `.wit` file would have meant a second `world` block
plus a second `wit_bindgen::generate!`/`bindgen!` invocation at the same module scope, which
`⏳️imports.rs`'s own module doc already notes wasmtime's bindgen cannot do twice in one scope. A
sibling package/world in sibling directories, reusing every proven idiom (`DeadlineCell`, the S1c
`black_box` discipline, S1's `AccessorTask`+oneshot pattern, tests A-F's named-interface accessor
calls, `terra-async-runtime-harness`'s tokio-based test D/E/F shapes) verbatim, was the lower-risk
choice. The earlier `terra-async-runtime` packet's own `runtimeprobe` harness was **never persisted**
as a fixture at all (built entirely under the session scratchpad, only its sources copied into this
ticket folder as `.txt`) — so there was no existing `runtimeprobe` fixture directory to extend either.

New files (all owned-path, all under `🔌️asyncprobe/`):
- `👽️guest-turn/Cargo.toml`, `👽️guest-turn/🦀️component.rs`, `👽️guest-turn/🧬️schema/📜️world.wit`
- `🖥️host-turn/Cargo.toml`, `🖥️host-turn/🦀️main.rs`

## a schema-shape mismatch, found before writing any code (read, not assumed)

The mission brief describes the target as "`world actor`, turn-shaped: `reactor.poll(events,
budget) -> turn-result`, plus async `jobs`/`checkpoint`/`describe`, importing `pure` + `host-async`."
Reading the REAL `🔌️plugin/🧬️schema/📜️component.wit` (lines 823-1050) directly, as of this packet,
shows **two separate worlds**, neither matching that description exactly:

- `world actor` (1029-1035): `import pure; export reactor; export jobs; export checkpoint; export
  describe;` — `reactor::poll` **is** already `async func`, and `jobs`/`checkpoint` are **already**
  plain `interface jobs`/`interface checkpoint` with **every function `async func`** (the S9 spike's
  predicted `jobs-async`/`checkpoint-async` split from `📓️terra-probe-spikes-report.md` was
  **superseded** — the real fix kept the interface NAMES `jobs`/`checkpoint` and just made their
  functions async, no new interfaces). But `world actor` imports **only** `pure` — no `host-async`.
- `world actor-async` (1044-1051): `import pure; import host-async; export runner; export jobs;
  export checkpoint; export describe;` — imports `host-async` as described, but its turn-loop entry
  point is `runner::run(events: stream<event>) -> result<_, plugin-error>` (the long-lived stream
  shape `⏳️imports.rs`/`⏳️runtime.rs` are ALREADY written against), not `reactor::poll`.

Neither real world combines an async `poll(events, budget) -> turn-result` export with a `host-async`
import the way the mission text describes. Per the mission's own instruction to build "a copy of the
REDUCED turn-shaped world" rather than the live schema, this packet built exactly that hypothetical
combination in `👽️guest-turn/🧬️schema/📜️world.wit` (package `semio:turnharness@0.1.0`) — field-for-
field on `budget`/`turn-status`/`turn-result`/`jobs`/`checkpoint` from the real schema, `event`/
`hostasync` reduced to exactly what Q1-Q6 need. **This is flagged, not silently assumed away**: if
the coordinator's actual plan is to migrate `world actor-async`'s `runner::run` stream shape to a
`poll`-per-call shape, that is a SCHEMA change nobody has made yet, separate from anything
`⏳️runtime.rs` alone can fix. The wasmtime-level mechanics this packet proves (a single async-func
turn entry point, awaited by the host, that the guest can suspend mid-call on a host import) are
IDENTICAL whether the entry point ends up named `poll` or `run` — every verdict below transfers to
either shape unchanged.

## verdict table

| # | Verdict | One-liner |
|---|---|---|
| Q1 | **GO** | Host calls async `poll(events, budget)` on a Store owned inside a `tokio::spawn`ed task; guest `.await`s a host import (`hostasync::wait-signal`) mid-turn; host resolves it from a **separate, concurrently-running** task (`oneshot::send`, no wall-clock coordination); `turn-result` comes back intact (`status: MoreWork`) and the guest's own `pure::log` call after resuming proves it genuinely continued past the await, not just returned a stale value. |
| Q2 | **GO** | Re-proven on the `poll` shape (not `run`): dropping only the `run_concurrent` future (`Box::pin`'d, manually polled once, then dropped) leaves `hostasync::hang`'s host future undropped (`dropped_future_only=false`); dropping the owning `Store` too flips it (`dropped_after_store=true`). Deterministic — no wall-clock races, a `DropSignal` guard flips an `AtomicBool` on `Drop::drop`. |
| Q3 | **GO** (epoch AND fuel) | Two separate `Store`s on one `Engine`, `poll` running a `black_box`'d CPU-bound `Burn(iters)` loop, multiplexed by host-level `tokio::join!` (no `Accessor::spawn`). Symmetric 40M/40M: ratio 1.00 (epoch) / 1.00 (fuel). Asymmetric 300M/5M: the 5M call finishes in ~27-38ms while the 300M call is still running for 840-950ms more, both levers. |
| Q4 | **GO** (same instance) / **GO** (different instance, trivial control) | `poll` suspended on `hostasync::hang` (never resolves); a `step-job` command dispatched via `accessor.spawn` (S1's `AccessorTask`+oneshot idiom) against the SAME live instance succeeds normally (`JobStep::Done(5)`) while `poll` is still parked. A separate, idle actor's `step-job` also succeeds trivially, as expected. |
| Q5 | **GO** (delta semantics confirmed; both cutoffs demonstrated) | `set_epoch_deadline` is confirmed a DELTA: `set_epoch_deadline(u64::MAX)` on an engine whose ticker has already run for real wall-clock time overflows `current_epoch + delta`, wraps to an already-past deadline, and traps **during `instantiate_async` itself** with `wasm trap: interrupt` (no callback needed to observe it — default behavior on an already-past deadline is an immediate trap, exactly `terra-probe-spikes-report.md`'s S9 finding, now reproduced on the `poll` shape). A real 20ms `DeadlineCell` cuts off a huge `Burn` after 16 legitimate `Yield`s with `wasm trap: interrupt`. A small hard fuel cap (5,000, armed **inside** the accessor after instantiation) cuts off the same call with `wasm trap: all fuel consumed by WebAssembly`. |
| Q6 | **GO** | A `tokio::runtime::Handle` (not a `Runtime`) is threaded into the test function's own signature; `handle.spawn(async move { let mut store = ...; store.run_concurrent(...).await })` — the `Store` constructed and owned INSIDE the spawned block — then `JoinHandle::abort()` genuinely tears it down: `dropped_before_abort=false`, `dropped_after_abort=true`. Ties directly to Q2/D/E. |

**All 6 questions: GO.** Reproduced twice with fully consistent categorical results (`terra-
asyncharness-final-run.txt`, `terra-asyncharness-reproduce-run.txt`) — only wall-clock timing numbers
vary run to run, never the pass/fail shape (drop flags, trap messages, ordering of tiny-vs-huge
completion, GO/FAIL of each sub-test).

## Q1 — turn shape works

```rust
// guest (👽️guest-turn/🦀️component.rs) — inside reactor::Guest::poll:
Event::AwaitSignal(id) => {
    let value = hostasync::wait_signal(id).await;
    pure::log("info".to_string(), format!("poll: wait-signal({id}) resolved = {value}")).await;
    status = TurnStatus::MoreWork;
}
```
```rust
// host — the import resolves from a SEPARATE task, concurrently with the poll call in flight:
let (tx, rx) = tokio::sync::oneshot::channel::<u32>();
state.signal_rx.lock().unwrap().insert(7, rx);          // pre-registered before instantiate
// ... instantiate, spawn the poll call via tokio::spawn(store.run_concurrent(...)) ...
tx.send(70)?;                                             // genuinely concurrent — no sleep needed
let turn = poll_task.await???;
```
Host-side `wait_signal` import: `rx.await.unwrap_or(0)` — a plain `oneshot::Receiver` awaited inside
the `HostWithStore` impl; whichever side (guest awaiting, host sending) reaches its half first, the
channel still resolves correctly, so no timing race exists to make non-deterministic.

## Q2 — cancellation requires dropping the Store (poll shape)

```rust
let fut = store.run_concurrent(async move |accessor| -> Result<TurnResult> {
    instance.semio_turnharness_reactor().call_poll(accessor, events, budget).await?.map_err(...)
});
let mut fut = Box::pin(fut);
match fut.as_mut().poll(&mut Context::from_waker(&Waker::noop())) {
    Poll::Pending => {}
    Poll::Ready(_) => panic!(...),
}
assert!(hang_started.load(Ordering::SeqCst));
drop(fut);                                    // (a) — hang_dropped STAYS false
drop(store);                                   // (b) — hang_dropped flips true
```
Byte-for-byte the same shape as `terra-async-runtime-harness-host-main.rs.txt`'s test D, just against
`call_poll` instead of `call_await_hang` — confirms the finding is about the Store-ownership
mechanism, not specific to the `run`/stream entry point.

## Q3 — preemption across actors (both levers)

```rust
// two separate Stores, each poll()ing a black_box'd CPU burn, host-level tokio::join! (NOT
// Accessor::spawn — that was S1's proven-wrong shape for cross-actor fairness):
let fut_a = async move { store_a.run_concurrent(|accessor| instance_a....call_poll(accessor, [Burn(iters_a)], budget)).await };
let fut_b = async move { store_b.run_concurrent(|accessor| instance_b....call_poll(accessor, [Burn(iters_b)], budget)).await };
tokio::join!(fut_a, fut_b);
```
Epoch lever: `store.epoch_deadline_callback(|_| Ok(Yield(1)))` + a 1ms ticker thread. Fuel-only lever:
a **separate Engine with `epoch_interruption` never enabled at all**
(`build_engine_fuel_only`) + `store.fuel_async_yield_interval(Some(500_000))` — isolates fuel as the
only interruption mechanism, S1c's own precedent. Guest loop: `std::hint::black_box(i).wrapping_mul(..)`
per iteration — without this LLVM strength-reduces the whole loop to a closed-form sum (S1c's own
self-inflicted confound, avoided here from the start rather than rediscovered).

## Q4 — jobs while a turn is live

```rust
struct StepJobTask { instance: Arc<Turnharness>, job: u64, reply: oneshot::Sender<...> }
impl AccessorTask<HostState> for StepJobTask {
    async fn run(self, accessor: &Accessor<HostState>) -> wasmtime::Result<()> {
        let r = self.instance.semio_turnharness_jobs().call_step_job(accessor, self.job, budget).await;
        let _ = self.reply.send(r);
        Ok(())
    }
}
// inside the SAME run_concurrent closure that's about to call poll (which will suspend on hang()):
accessor.spawn(StepJobTask { instance: instance_for_step, job: 5, reply: tx })?;
instance.semio_turnharness_reactor().call_poll(accessor, [AwaitHang(42)], budget).await
```
Result: `step-job` on the SAME instance succeeds normally (`JobStep::Done(5)`) while `poll` sits
suspended on `hostasync::hang`, which never resolves — no error, no deadlock, no special handling
needed. A second, idle actor's Store answers `step-job` directly (no `accessor.spawn` needed, nothing
else in flight) — the trivial control case, also GO. **This settles the mission's framing question:
jobs do NOT need their own dedicated instance** — the same `accessor.spawn` mechanism `⏳️runtime.rs`
already uses for `Checkpoint`/`StartJob`/`StepJob`/`CancelJob` (lines 463-520) is sufficient and was
never actually blocked; this packet supplies the missing direct evidence for it.

## Q5 — budgets

Exact trap text observed (from `terra-asyncharness-final-run.txt`):
```
Q5a (20ms DeadlineCell, huge burn): 16 legitimate Yields, then
    ... [async-lift]semio:turnharness/reactor@0.1.0#poll: wasm trap: interrupt
Q5b (fuel=5000 armed inside the accessor, no yield interval, huge burn):
    ... [async-lift]semio:turnharness/reactor@0.1.0#poll: wasm trap: all fuel consumed by WebAssembly
Q5c (set_epoch_deadline(u64::MAX), no callback registered, real ticks already elapsed):
    set_epoch_deadline(u64::MAX) trapped during instantiate_async itself: wasm trap: interrupt
```
**Own-harness bug found and fixed along the way** (kept as evidence, not hidden): the first Q5b
attempt called `store.set_fuel(5_000)` **before** `instantiate_async`, and instantiation itself
(WASI setup + component instantiation, which also consumes fuel once `consume_fuel` is enabled)
exhausted the cap and trapped there — not inside the `poll()` call actually under test. Fixed by
instantiating with a generous cap, then arming the real per-grant cap via
`access.as_context_mut().set_fuel(5_000)` **inside** the accessor closure, immediately before the
call under test — this is the *exact* mechanism `⏳️runtime.rs`'s own control loop already uses for
refills (`access.as_context_mut().set_fuel(budget.fuel)`, line ~451), so this bug doubles as
independent confirmation that shape is load-bearing, not a style choice: **arming fuel at
`Store::new` time and expecting it to survive `instantiate_async` unchanged is NOT reliable if the
budget is small.**

Q5c's own-harness fix: the first version registered an unconditional `Yield(1)`-returning callback,
which made `set_epoch_deadline(u64::MAX)`'s overflow indistinguishable from an ordinary `Yield` (both
just caused one early callback firing, `hits=1`, then normal execution) — too weak a signal to
actually prove delta-vs-absolute semantics. Fixed by registering **no callback at all**, relying on
wasmtime's documented default (trap immediately on an already-past deadline) — this reproduces the
real S9 self-inflicted bug decisively: an immediate, uncaught `wasm trap: interrupt` during
`instantiate_async`, not a graceful Yield.

## Q6 — tokio shape

```rust
fn main() -> Result<()> {
    let rt = tokio::runtime::Builder::new_multi_thread().enable_all().build()?;
    let handle = rt.handle().clone();
    rt.block_on(async_main(handle))          // Runtime built ONCE; only a Handle is threaded down
}

async fn test_q6_tokio_handle_spawn(handle: &tokio::runtime::Handle, ...) -> Result<String> {
    let mut store = Store::new(engine, state);           // Store constructed OUTSIDE the spawn call
    let instance = Turnharness::instantiate_async(&mut store, component, linker).await?;
    let join = handle.spawn(async move {                  // but MOVED INTO the spawned block
        store.run_concurrent(|accessor| instance....call_poll(accessor, [AwaitHang(11)], budget)).await
    });
    // ... wait for hang_started ...
    join.abort();
    let _ = join.await;                                    // dropped_after_abort == true
}
```
Confirms `handle.spawn` behaves identically to bare `tokio::spawn` for this purpose (expected — a
`Handle` is just a cheap, cloneable reference into the same runtime a `tokio::spawn` on the current
task would already resolve to) — the only thing genuinely new here versus
`terra-async-runtime-harness`'s test E is proving the API surface itself (accepting an injected
`Handle` parameter rather than assuming an ambient `#[tokio::main]` context) works end to end.

## corrections required to ⏳️runtime.rs

1. **`GrantWindow`/`GrantedEventProducer`/the `StreamReader<Event>` plumbing (lines 178-274, 415-420)
   become dead code if the turn-loop entry point becomes a plain per-call `poll(events: list<event>,
   budget) -> turn-result`** (as the mission's target world describes) rather than the currently-
   implemented `run(events: stream<event>)`. A `list<event>` argument needs no `StreamProducer`, no
   grant-parks-past-the-grant park/refill machinery, no `exhausted: Notify` — the host simply builds
   a `Vec<Event>` per call. **This is the single largest structural change**, and it only applies if
   the schema actually migrates from `runner::run`'s stream shape to a `poll`-per-call shape — see
   the schema-mismatch note above; as of this packet, the LIVE `world actor-async` still exports
   `runner::run`, so this correction is conditional on that migration, not yet applicable today.
2. **Replace the `call_run_fut`-racing `tokio::select!` loop (lines 424-524) with a command-only
   loop** once (1) lands: no long-lived call exists to race against any more. Every `AsyncActorCommand`
   variant, including a NEW `Poll { events: Vec<Event>, budget: Budget, reply: oneshot::Sender<...> }`,
   becomes `accessor.spawn`-dispatched (Q4's proven pattern, generalized) or answered directly on the
   loop for cheap ones. `AsyncTurnOutcome::Turn`/`Finished` collapse into the `Poll` command's own
   oneshot reply — there is no separate "grant exhausted" event to synthesize once `poll` is a normal
   per-call boundary rather than a live stream being drained.
3. **`synthesize_turn_result` (lines 290-294) narrows scope, does not disappear**: once the guest's
   `poll` call itself returns a real WIT `turn-result` (this packet's harness confirms the record
   round-trips intact, Q1), the host no longer builds the record from scratch — but it still MUST
   override `fuel_used` with `fuel_before.saturating_sub(fuel_after)` computed host-side (Q5b's own
   bug: a guest cannot observe true fuel consumption; this harness's guest always returns `fuel_used:
   0` by design, matching `⏳️runtime.rs`'s own existing "never trust the guest" stance — keep it).
4. **Interface names: NO `-async` suffix.** The real schema kept the names `jobs`/`checkpoint`
   verbatim and made their functions `async func` in place — `checkpoint-async`/`jobs-async` (lines
   105-116, 302-314, 463-520's `semio_framework_checkpoint_async()`/`semio_framework_jobs_async()`)
   were the S9 spike's PREDICTED fix, superseded by what actually landed. Correct accessor names:
   `instance.semio_framework_reactor()` (or `semio_framework_runner()` if `runner::run` stays live),
   `instance.semio_framework_jobs()`, `instance.semio_framework_checkpoint()` — confirmed by this
   packet's own `semio_turnharness_reactor()`/`semio_turnharness_jobs()`/`semio_turnharness_checkpoint()`
   compiling and resolving correctly against an identically-shaped package/interface pair.
5. **`JobBudgetArg`/`JobStepResult` (lines 315-324) field/variant shapes**: real `job-step` carries
   BYTE PAYLOADS on every arm — `running(option<list<u8>>)`, `done(list<u8>)`, `failed(list<u8>)` —
   `JobStepResult::Failed { error: Vec<u8> }` in the current file already matches this, but should be
   double-checked once real bindgen output exists; this packet's own reduced `job-step` (`Running`,
   `Done(u32)`, `Failed(string)`) was deliberately narrower and is not itself a source of truth for
   field types, only for the calling CONVENTION (accessor-taking, `.await`, `Result<JobStep, Fault>`
   wrapping), which is confirmed correct.
6. **Tokio-handle injection (mission's own ask, not yet in the file)**: `AsyncEngineHandle::new` and
   `AsyncActorTask::spawn` currently have no `tokio::runtime::Handle` parameter at all — they rely on
   an ambient runtime context for their internal `tokio::spawn`. Q6 confirms `handle.spawn(...)`
   behaves identically to bare `tokio::spawn(...)` for this purpose, so the fix is mechanical: thread
   a `Handle` parameter through and replace `tokio::spawn(async move { ... })` (line 388) with
   `handle.spawn(async move { ... })` — no other change needed, `JoinHandle::abort()` still tears the
   Store down correctly either way (Q2/Q6 both confirm this on the SAME underlying mechanism).
7. **`DeadlineCell`/`install_epoch_budget` (lines 134-157): PROVEN CORRECT AS WRITTEN, no change.**
   Q5a reproduces this exact shape against the `poll` entry point and gets the designed Yield-then-
   Interrupt behavior. This is a confirmation, worth recording as such rather than only listing
   corrections.
8. **Initial fuel arming order (line 397, `store.set_fuel(initial_budget.fuel)` called BEFORE
   `instantiate_async` at line 407) is a watch-item, not a confirmed bug**: Q5b shows this exact
   ordering fails when the fuel cap is small relative to instantiation's own cost. Production
   `initial_budget.fuel` values are presumably far larger than a bare `Store::new`+instantiate's
   fuel cost, so this may never trigger in practice — but the failure mode is real and cheap to avoid
   (arm generously at construction, apply the real initial grant's cap from inside the first
   iteration of the control loop instead, mirroring how every SUBSEQUENT grant is already applied).

## honest gaps

- The mission's exact target world (`poll` + `host-async` import combined in `world actor`) does not
  exist in the schema today — see the schema-mismatch section above. Every Q1-Q6 verdict is evidence
  about the underlying wasmtime MECHANICS (proven to transfer to either `poll` or `run` shape), not
  evidence that the schema migration itself has happened or is even scheduled.
- Q4 only exercised `step-job`; `start-job`/`cancel-job`/`checkpoint`/`restore` were not separately
  raced against a suspended `poll` — no reason to expect a different result (they share the exact
  same `accessor.spawn` calling convention this packet already confirmed for `step-job` and
  `terra-async-runtime-harness` already confirmed for `checkpoint`), but not independently re-run
  here to conserve budget.
- Q3's fuel-only variant reused `fuel_async_yield_interval(Some(500_000))` (S1b/S1c's own proven
  value) rather than sweeping other interval sizes — sufficient to answer "does the lever work at
  all", not a tuning study.
- Q5's fuel-cutoff sub-test used a single fuel cap (5,000); did not separately verify the exact fuel
  cost of `poll`'s own ABI overhead (lift/lower, instantiation) versus the guest loop body — the
  overhead is real (Q5b's own-harness bug proves it's non-trivial) but was not measured precisely.

## commands + exit codes

`CARGO_TARGET_DIR=<scratchpad>/target-asyncprobe` throughout (per binding rules — the ticket-folder
target dir is EPERM on this machine).

```
$ cargo build -p semio-turnharness-guest --release --target wasm32-wasip2 --manifest-path 👽️guest-turn/Cargo.toml
Finished `release` profile [optimized] target(s) in 15.09s
exit 0 (first try — the async-function-in-a-named-imported-interface guest binding shape,
        `semio::turnharness::pure::log(..).await` / `hostasync::wait_signal(..).await`, compiled and
        genuinely required `.await` on the first attempt, no guessing needed)

$ cargo build -p semio-turnharness-host --release --manifest-path 🖥️host-turn/Cargo.toml
error[E0432]: unresolved import `semio::turnharness::jobs` / `reactor`   (2 errors)
  → fix: EXPORTED named interfaces live under `exports::semio::turnharness::{jobs,reactor}` on the
    HOST side (bindgen!), mirroring the guest's own `exports::` convention — real imports (`pure`,
    `hostasync`) stay unqualified at `semio::turnharness::*`.
exit (compile error, 2 unresolved imports)

$ cargo build -p semio-turnharness-host --release --manifest-path 🖥️host-turn/Cargo.toml   # after fix
error[E0308]: mismatched types — call_poll expects `Vec<Event>`, found `&Vec<Event>`   (9 occurrences)
error[E0599]: no method `poll` found — `Future` trait not in scope
  → fixes: drop the `&` on every `call_poll(accessor, events, budget)` call site; add
    `use std::future::Future;`.
exit (compile error, 10 total)

$ cargo build -p semio-turnharness-host --release --manifest-path 🖥️host-turn/Cargo.toml   # after fix
Finished `release` profile [optimized] target(s) in 8.81s
exit 0 (3 harmless `unused_mut` warnings)

$ ASYNCPROBE_WASM=... TURNHARNESS_WASM=<scratchpad>/target-asyncprobe/wasm32-wasip2/release/semio_turnharness_guest.wasm <scratchpad>/target-asyncprobe/release/semio-turnharness-host
exit 0 — but Q5 collapsed to ONE combined error line, losing Q5a/Q5c's results: `test_q5_budgets`'s
own `?` on Q5b's `instantiate_async` (which itself traps under a too-small fuel cap) short-circuited
the whole function. terra-asyncharness bug, not an architectural finding — see `## Q5` above.

$ cargo build -p semio-turnharness-host --release --manifest-path 🖥️host-turn/Cargo.toml   # Q5b fix (arm fuel inside the accessor, catch errors per sub-test instead of `?`-propagating)
Finished `release` profile [optimized] target(s) in 9.63s
exit 0

$ TURNHARNESS_WASM=... <scratchpad>/target-asyncprobe/release/semio-turnharness-host
exit 0 — Q5a/b now report correctly, but Q5c's own callback (unconditional Yield) made the
delta-vs-absolute overflow signal too weak (hits=1, no trap) — see `## Q5` above.

$ cargo build ...   # Q5c fix (no epoch_deadline_callback registered — rely on the documented default trap-on-reached behavior)
Finished `release` profile [optimized] target(s) in 10.99s
exit 0

$ TURNHARNESS_WASM=... <scratchpad>/target-asyncprobe/release/semio-turnharness-host
exit 0 — all 6 questions GO, Q5c now decisively traps during instantiate_async (terra-asyncharness-run3.txt)

$ TURNHARNESS_WASM=... <scratchpad>/target-asyncprobe/release/semio-turnharness-host   # reproducibility check
exit 0 — identical categorical results, only wall-clock numbers differ (terra-asyncharness-reproduce-run.txt)

$ cargo build ...   # cosmetic: drop 3 unnecessary `mut` bindings flagged by the compiler
Finished `release` profile [optimized] target(s) in 10.70s
exit 0, zero warnings

$ TURNHARNESS_WASM=... <scratchpad>/target-asyncprobe/release/semio-turnharness-host   # FINAL canonical run
exit 0 (terra-asyncharness-final-run.txt — the run quoted throughout this report)

$ rustfmt --check --edition 2021 👽️guest-turn/🦀️component.rs
exit 0 — already rustfmt-clean.

$ rustfmt --check --edition 2021 🖥️host-turn/🦀️main.rs
exit 1 — reformatting diffs only (long single-line match arms/closures vs rustfmt's wrapping, same
style precedent `⏳️imports.rs`/`⏳️runtime.rs` themselves already carry unformatted) — confirms the
file PARSES cleanly; rustfmt only emits a diff after a successful parse (terra-asyncharness-
rustfmt-check.txt).
```

## files touched

- **NEW** `🧰️framework/🛍️products/💻️os/🧫️fixtures/🔌️asyncprobe/👽️guest-turn/Cargo.toml`,
  `👽️guest-turn/🦀️component.rs`, `👽️guest-turn/🧬️schema/📜️world.wit`
- **NEW** `🧰️framework/🛍️products/💻️os/🧫️fixtures/🔌️asyncprobe/🖥️host-turn/Cargo.toml`,
  `🖥️host-turn/🦀️main.rs`
- **NEW** (this ticket folder) `terra-asyncharness-world.wit.txt`,
  `terra-asyncharness-guest-component.rs.txt`, `terra-asyncharness-host-main.rs.txt`,
  `terra-asyncharness-final-run.txt`, `terra-asyncharness-reproduce-run.txt`,
  `terra-asyncharness-rustfmt-check.txt`, this report.
- Read but NOT edited: `⏳️runtime.rs`, `⏳️imports.rs`, `🧬️schema/📜️component.wit` (live schema —
  never touched, only read), `📓️terra-probe-spikes-report.md`, `📓️terra-async-runtime-report.md`,
  `terra-async-runtime-harness-*.txt`.
- Nothing else in the repo was modified. No git-modifying commands were run.
