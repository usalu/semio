# 🧪️ terra-probe-spikes (W6) — S1-S8 runtime spikes for the async-first pooled-actor plan

Executor: `terra-probe-spikes`. Owned path: `🧰️framework/🛍️products/💻️os/🧫️fixtures/🔌️asyncprobe/**`.
Everything below was actually built and run (wasmtime 47.0.3, wit-bindgen 0.57.1, rustc 1.99.0-nightly,
target `wasm32-wasip2`), never inferred from source reading alone.

## verdict table

| # | Verdict | One-liner |
|---|---|---|
| S1 | **NO-GO (scope-narrowed by S1b, see below)** | epoch-Yield (fuel+epoch+async, `UpdateDeadline::Yield(1)`) fires correctly (139 confirmed callback hits over 202ms) but does **not** interleave two concurrently-`spawn`ed CPU-bound guest tasks *inside one `Store`* — instance A's `burn()` runs to 100% completion (all 9,766 progress entries) before instance B's first entry appears at all. Tried both `futures::join!` and proper `Accessor::spawn`/`JoinHandle`; same result both times. **This measures intra-store concurrency, which the design never asks for — see S1b.** |
| **S1b** | **GO** | *(coordinator follow-up, added after initial report)* Two **separate** `Store`s, each with its own `run_concurrent` future, multiplexed by a **host-level** `futures::join!` on one current thread (no `Accessor::spawn`) — the shape the design actually needs — **do** interleave: 139-149 switches across 19,532 progress entries, reproduced twice, ~1.25ms between switches (close to the 1ms epoch-ticker granularity). A fuel-only variant (`fuel_async_yield_interval`, zero epoch interruption) interleaves too, far more finely (3,041 switches, deterministic across runs). **This is the load-bearing result: the architecture's core fairness claim holds.** |
| **S1c** | **GO** | *(2nd coordinator follow-up — a peer session reported the opposite of S1b using both `join!` and `Accessor::spawn`, and the coordinator correctly suspected S1b's `burn` guest export calling the `progress` host import ~9,765 times was itself creating yield points unrelated to epoch/fuel-Yield)*. A new `burn-pure` export with **zero host-import calls anywhere in its loop**, tested in the identical S1b shape (separate `Store`s, host-level `futures::join!`), across 4 sub-tests (epoch×symmetric, epoch×asymmetric, fuel×symmetric, fuel×asymmetric) — **all 4 GO**. Symmetric equal-workload calls finish within ~1ms of each other (ratio ≈1.00, not ≈2.00); a tiny 5M-iteration call finishes in ~6-30ms *while* a 300M-iteration call on the sibling `Store` is still running for hundreds more ms. **Answers the coordinator's (A)/(B) question: (A) is true** — epoch/fuel yields genuinely preempt pure CPU-bound guest code with no import confound. First attempt at `burn-pure` (no anti-optimization) was itself confounded the other way: LLVM strength-reduced the side-effect-free loop to a closed-form sum and finished 300M "iterations" in 6 microseconds with 0 epoch-callback hits — fixed with `std::hint::black_box`, see S1c section for the full story. |
| **S9** | **NO-GO** | *(coordinator's message called this "S7", but S7 was already this report's sqlx/neo4rs Send compile-check below — renumbered to S9, next free slot, no log content altered, printed labels in the raw logs still say "S7-Q.." since that's what the code actually printed)* *(coordinator follow-up — the real `🔌️plugin/🧬️schema/📜️component.wit` declares `jobs`/`checkpoint` as plain sync `func` exports, but `world actor-async`'s store MUST have `wasm_component_model_async(true)` for `runner`/`host-async`)*. A plain sync `func` export is **categorically uncallable** on such a store — not "deadlocks if it awaits an import", literally cannot be invoked AT ALL, in every calling shape tested: reentrant via `Accessor::with` inside `run_concurrent` (matching how `step-job` would need to be called into a live actor), AND the classic `&mut Store` call on an otherwise-idle store with `run_concurrent` never even started. All fail immediately with `"store configuration requires that *_async functions are used instead"` — a clean, catchable Rust error, not a hang. The ASYNC-declared control export (identical import-awaiting logic) works normally. **`JobCtx::host()` is unimplementable in `world actor-async` as currently schema'd — needs `jobs-async`/`checkpoint-async` with `async func`, exact WIT diff in the S7 section below.**
| S2 | **PASS** | Guest-side subtask cancellation (manual single-poll-then-drop of the `hang` import's future) does drop the host-side future — the `DropSignal` guard fired, confirmed via `was_hang_dropped()` after a round-trip. |
| S3 | **PASS** (with caveat) | Calling a second export (`checkpoint`) while `run()` is parked mid-stream-read works — but **only if `checkpoint` is itself declared `async func` in WIT**. A plain sync `func` export errors at runtime with `"store configuration requires that *_async functions are used instead"` once the store has `wasm_component_model_async(true)` — sync exports are effectively unusable on such a store. |
| S4 | **PASS** | Compile-only: `store.run_concurrent(async move |accessor| {...})`'s future and bindgen's `instance.call_ping(accessor, n)` future are both `Send` (host still built cleanly with two dead-code `assert_send::<T: Send>` probes wired to real call sites). |
| S5 | **PASS** | Custom `StreamProducer::poll_produce` correctly stores a waker on an empty queue and returns `Pending`; a background OS thread later mutates the queue and calls `.wake()` from outside the executor entirely — guest resumed and summed the delayed items correctly (150 = 10+20+30+40+50), 7 `poll_produce` calls total. |
| S6 | **PASS** | A hand-rolled `Rc<RefCell>`-backed single-task "local executor" (poll forwarded straight through) correctly drove a host import (`delayed_echo`) that deliberately returns `Pending` once — waker propagation across the extra indirection layer worked, guest resumed with `"delayed:nested"`. |
| S7 | **PASS** (compile-only) | `sqlx::query(..).execute(&PgPool)` and `neo4rs::Graph::execute(..)` query futures both satisfy `Send`, using the exact `sqlx`/`neo4rs` versions+features from `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📦️packages/🦀️rust/Cargo.toml`. |
| S8 | **PASS** | `reqwest` with `default-features = false, features = ["rustls-tls"]` built and completed a real HTTPS GET to `https://example.com` → `200 OK`, 559-byte body. |

**Revised gate verdict (post-S9): 11 results across S1/S1b/S1c/S2-S9 — 9 GO/PASS, 2 NO-GO kept permanently on record (S1, S9), each narrower or more surgical than a first read suggests.** The original report shipped S1 as a genuine NO-GO for CPU-bound fairness, full stop. The coordinator correctly flagged that the tested shape (`Accessor::spawn` inside ONE `Store`) is not what the design uses — the design's rule is one root task per `Store`, never concurrent reentrant calls into a single instance; fairness is meant to come from a level up, with each actor's own `Store` multiplexed by the *host's* executor. **S1b tested exactly that shape and reported GO** — but S1b's `burn` guest export called the `progress` host import ~9,765 times per call, and a peer session reported the *opposite* result using the same `join!` mechanism plus `Accessor::spawn`, which the coordinator correctly suspected meant S1b's interleaving was import-call-driven, not epoch/fuel-Yield-driven. **S1c settles this with a `burn-pure` export that has zero host-import calls anywhere in its loop, in the identical two-`Store`/host-`join!` shape, across 4 independent sub-tests (epoch×symmetric, epoch×asymmetric, fuel×symmetric, fuel×asymmetric) — all 4 GO.** The coordinator's own (A)/(B) framing: **(A) is true** — epoch/fuel yields genuinely preempt pure CPU-bound guest code across separate `Store`s; the async shard executor may multiplex CPU-bound actors, and S1b's conclusion stands, now on solid ground rather than a confounded one. **S1, S1b, and S1c all stay in this report as separate, permanent results** — they answer three different, increasingly precise questions: S1 says wasmtime's own intra-store task scheduler doesn't give you fairness for free if you (mis)use `Accessor::spawn` to run two CPU-bound guest tasks on one instance's store (still a valid "don't do this" warning); S1b says the design's actual shape — separate `Store`s, host-level multiplexing — interleaves, but left open whether that was really epoch/fuel-Yield or an import-call artifact; S1c closes that gap and confirms it is genuinely epoch/fuel-Yield. I/O-bound concurrency (S2/S3/S5/S6, all real awaits/imports/streams) also works fine regardless, and was never in question. **S9 is the one genuinely fresh NO-GO in this report**: the real plugin schema's `jobs`/`checkpoint` interfaces are plain sync `func`, and a plain sync `func` export turns out to be categorically uncallable — not merely deadlock-risky — on any `Store` with `wasm_component_model_async(true)` enabled, which `world actor-async`'s `runner`/`host-async` require. `JobCtx::host()` is unimplementable in `world actor-async` as currently schema'd; the fix is `jobs-async`/`checkpoint-async` with `async func`, exact diff in the S9 section.

> **⚠️ Prominent, schema-relevant caveat (S3):** once a store has `wasm_component_model_async(true)`, any WIT export that may be called while another call on the *same instance* is in flight — this includes any `checkpoint`-shaped export in the pooled-actor design — **must be declared `async func` in WIT, never plain `func`.** A plain sync export still compiles and can be called when nothing else is in-flight, but the generated call **fails at runtime**, not compile time, the moment it's actually invoked concurrently ("store configuration requires that `*_async` functions are used instead"). This directly changes what the checkpoint export must look like in the schema — see the S3 row and the "fallbacks now required" section below.

## observed output

Full verbatim host stdout (`TICKET_DIR/terra-probe-spikes-final-run.txt`, exit code 0):

```
[host] echo import called from inside guest await: ping:41
[host] ping(41) = 42
[host] run(stream) summed = 21
[host] G3 PASS
[host] S1: burn(0) = 2850874112, burn(1) = 2850874112, elapsed = 202.677417ms, epoch_deadline_callback hits = 139
[host] S1: progress log has 19532 entries
[host] S1: FAIL — no interleaving observed (last id=0 at Some(9765), first id=1 at Some(9766))
[host] S3/S5: run() parked on manual first poll = true
[host] S3: PASS — checkpoint() = 42 succeeded while run() was parked
[host] S5: PASS — run() resumed after delayed wake, summed = 150 (7 poll_produce calls)
[host] hang(99) started, awaiting pending forever until dropped
[host] echo import called from inside guest await: post-cancel-sync
[host] S2: PASS — hang()'s host future was dropped after guest-side cancel
[host] S6: PASS — nested Rc<RefCell> executor resumed correctly, got "delayed:nested"
[host] ==== VERDICTS ====
[host] S1: FAIL — no interleaving observed (last id=0 at Some(9765), first id=1 at Some(9766))
[host] S3: PASS — checkpoint() = 42 succeeded while run() was parked
[host] S5: PASS — run() resumed after delayed wake, summed = 150 (7 poll_produce calls)
[host] S2: PASS — hang()'s host future was dropped after guest-side cancel
[host] S6: PASS — nested Rc<RefCell> executor resumed correctly, got "delayed:nested"
```

S7 (`semio-asyncprobe-driversend`, no I/O, exit 0):

```
[driversend] S7 PASS (compile-only): sqlx::query(..).execute(&PgPool) future is Send, neo4rs::Graph::execute(..) future is Send. This binary performs no I/O — see sqlx_query_future_is_send / neo4rs_execute_future_is_send, both dead code, both had to type-check for this to build at all.
```

S8 (`semio-asyncprobe-tlsprobe`, real network, exit 0):

```
[tlsprobe] GET https://example.com -> status = 200 OK, body_len = 559
[tlsprobe] S8 PASS — rustls-tls (no default features) completed a real HTTPS GET
```

### S1 evidence detail (why FAIL, not UNRESOLVED)

Two independent concurrency mechanisms were tried, both non-interleaved:

1. **`futures::join!(instance.call_burn(...), instance_b.call_burn(...))`** on 200K iterations/call: `epoch_deadline_callback hits = 144` over 326ms, log showed guest 0's 49 entries fully first, then guest 1's 49 entries (`terra-probe-spikes-run1.txt`).
2. **`Accessor::spawn` with dedicated `AccessorTask` + `JoinHandle`/`oneshot` result channel** (the architecturally-correct wasmtime-level way to run two guest tasks concurrently) on 40M iterations/call: `epoch_deadline_callback hits = 139` over 202ms, log showed guest 0's 9,766 entries fully first, then guest 1's 9,766 entries (`terra-probe-spikes-run3.txt` = final report run).

The epoch callback demonstrably fires repeatedly *during* the loop (139-144 times, ruling out "loop finished before the first epoch tick" as the explanation — that was true at 200K iterations in an earlier attempt but is definitively ruled out at 40M). `UpdateDeadline::Yield(1)` is confirmed to be invoked, accepted, and not trap. Yet guest instance B is never polled even once until guest instance A's `call_burn` fully resolves. The most consistent explanation from behavior alone (not fully traced through wasmtime's fiber-scheduler internals — that would need more budget than this spike had): epoch-Yield yields the *OS thread* back to whatever polls the store (useful for sharing a thread with unrelated async work, e.g. other stores/tokio tasks) but does not cause wasmtime's *internal* guest-task scheduler to switch which spawned task is "current" — the yielding task appears to simply get immediately re-scheduled ahead of a never-yet-polled sibling task.

## S1b — inter-store fairness (coordinator follow-up, resolves the S1 scope question)

**Why this exists:** the coordinator pointed out that S1's evidence — "two `Accessor::spawn`-ed CPU-bound guest tasks never interleave" — is **intra-store concurrency**: two guest tasks inside ONE `Store`, scheduled by wasmtime's own concurrent-call machinery. The design never asks for that; its central rule is the opposite — **one root task owns one `Store`, never concurrent reentrant calls into a single instance.** Fairness is supposed to come from a level up: actor A's `Store` and actor B's `Store` are *separate* `run_concurrent` futures, multiplexed by OUR executor on one thread, with epoch `Yield` returning control from whichever guest is running so the executor can poll the other. **S1's result does not answer whether that shape works. S1b tests it directly.** The prior sentence in the paragraph above — "does not cause wasmtime's internal guest-task scheduler to switch which spawned task is current" — turns out to be the precise boundary: that limitation is specific to `Accessor::spawn` inside one store's internal scheduler, not to epoch-Yield as a mechanism.

**Verdict: GO.** Reproduced twice.

**Harness (added to `🖥️host/🦀️main.rs`, right after the S1-S6 block, same shared `Engine`/`Component`/`Linker`):**

```rust
// Two SEPARATE stores, each with its own instance and its own epoch callback:
let (mut store_a, hits_a) = make_store_epoch(&engine, shared_log.clone(), progress_start);
let (mut store_b, hits_b) = make_store_epoch(&engine, shared_log.clone(), progress_start);
let instance_a = Asyncprobe::instantiate_async(&mut store_a, &component, &linker).await?;
let instance_b = Asyncprobe::instantiate_async(&mut store_b, &component, &linker).await?;

let fut_a = store_a.run_concurrent(async move |accessor: &Accessor<HostState>| -> Result<u32> {
    Ok(instance_a.call_burn(accessor, 20, iters).await?)
});
let fut_b = store_b.run_concurrent(async move |accessor: &Accessor<HostState>| -> Result<u32> {
    Ok(instance_b.call_burn(accessor, 21, iters).await?)
});
// 🎯️ the actual mechanism under test: HOST-LEVEL futures::join! on the current thread —
// NOT Accessor::spawn inside a single store.
let (ra, rb) = futures::join!(fut_a, fut_b);
```

`shared_log` is one `Arc<Mutex<Vec<(u32, u32, Duration)>>>` cloned into both stores' `HostState`, so `progress(guest_id, step)` calls from either instance land in one chronologically-ordered log regardless of which store's guest wrote them — that shared log, not per-store logs, is what makes "did they interleave" answerable across two independent `Store`s.

**Evidence (`terra-s1b-run1.txt`, `terra-s1b-run2.txt`, both exit 0):**

```
run1: [host] S1b: burn(20)=2850874112, burn(21)=2850874112, elapsed=193.668875ms, hits_a=75, hits_b=75, log entries=19532, switches=149
run2: [host] S1b: burn(20)=2850874112, burn(21)=2850874112, elapsed=176.923458ms, hits_a=70, hits_b=71, log entries=19532, switches=139
```

Both runs: same final `burn()` result as S1 (2850874112, confirming identical guest computation, only the orchestration shape changed), 139-149 alternations between guest 20 and guest 21 across the full 19,532-entry log — i.e. **not** one unbroken prefix per guest the way S1 showed. First 10 switch points (run1, `t` = time since this sub-probe's own start):

```
switch #0 — (id=20, step=205, t=2.060042ms) -> (id=21, step=0, t=2.074875ms)
switch #1 — (id=21, step=135, t=3.321917ms) -> (id=20, step=206, t=3.32725ms)
switch #2 — (id=20, step=341, t=4.5705ms) -> (id=21, step=136, t=4.590584ms)
switch #3 — (id=21, step=272, t=5.831834ms) -> (id=20, step=342, t=5.838209ms)
switch #4 — (id=20, step=480, t=7.090209ms) -> (id=21, step=273, t=7.098917ms)
```

### sub-question 1 — granularity

Consecutive switch timestamps are ~1.25ms apart (2.06 → 3.32 → 4.57 → 5.83 → 7.09ms, i.e. deltas of 1.26/1.25/1.26/1.26ms), each carrying ~130-145 progress markers (each marker = 4096 loop iterations). That lines up closely with the 1ms `engine.increment_epoch()` ticker used throughout this fixture — **the design's ~1ms-slice target is achievable with this exact mechanism**, modulo the ticker's own scheduling jitter (a real ticker thread sleeping `Duration::from_millis(1)` is not a hard real-time guarantee). Slice granularity is tunable independently by changing the ticker interval and/or `set_epoch_deadline`'s delta.

### sub-question 2 — does `Yield(1)` reach *our* executor, or only wasmtime's internal scheduler?

**It reaches our executor.** The only way `fut_b` (store B's `run_concurrent` future) can ever be polled by `futures::join!` is if `fut_a`'s `poll()` call returns `Poll::Pending` and controls returns to the `join!` combinator — and the switch data above shows exactly that happening repeatedly. This is the mechanism-level confirmation the coordinator asked for: `UpdateDeadline::Yield(1)` causes a real, externally-observable `Store::run_concurrent` future suspension, not just an internal wasmtime scheduling shuffle. (Had this been NO-GO, the signature would have looked identical to S1: one guest's full 9,766-entry run completing before the other's first entry — that specific failure shape did not appear here.)

### sub-question 3 — is epoch the only lever, or does fuel-per-slice work too?

Tested with a **fully separate `Engine`** that never enables `epoch_interruption` at all (so there is no epoch-deadline requirement on these stores whatsoever — isolates fuel as the only interruption mechanism in play), same two-`Store`/host-`join!` shape, `store.set_fuel(u64::MAX)` (unlimited total budget — this is a slicing lever, not a starvation lever) plus `store.fuel_async_yield_interval(Some(500_000))`:

```
[host] S1b-Q3 (fuel-only, no epoch): burn(30)=2850874112, burn(31)=2850874112, log entries=19532,
GO — fuel_async_yield_interval(Some(500_000)) ALSO interleaves with NO epoch interruption enabled at all
(3041 switches / 19532 entries). Answers Q3: fuel is an independent, sufficient lever, not merely epoch's helper.
```

**GO, and deterministic** — 3041 switches on both runs (fuel consumption per instruction is deterministic; wall-clock epoch ticks are not, which is why the epoch variant's switch count varies run to run — 139 vs 149 — while the fuel variant's does not). Fuel-interval yields are **far finer-grained** than the 1ms epoch ticker at this interval setting (switches roughly every 6-7 progress markers vs ~130-145 for epoch) — fuel slices by work done, epoch slices by wall-clock time elapsed; they are independent, both-working levers, and a production design could combine them (fuel as a hard per-slice cap, epoch as a wall-clock safety net) or pick either alone.

**S1b conclusion:** the architecture's core premise holds *for the shape the design actually uses*. S1's NO-GO stands as a real, separate finding about `Accessor::spawn`-based intra-store concurrency (worth keeping on record so nobody reaches for it later expecting fairness), but it does not threaten the pooled-actor plan, because the plan's actual multiplexing unit — one `Store` per actor, host-level `join!`/`select!`/`FuturesUnordered` across actors — is exactly what S1b confirms works, at both epoch- and fuel-driven granularities.

## S1c — pure-CPU preemption, ZERO host imports (2nd coordinator follow-up, settles S1 vs S1b)

**Why this exists:** a peer session (the previous coordinator of this ticket, who ran the original S1) reported the opposite of S1b — non-interleaving, confirmed both via `futures::join!` and `Accessor::spawn`. The current coordinator's hypothesis: S1b's `burn` guest export calls the `progress` **host import** every 4096 iterations (~9,765 calls across a 40M-iteration run). Even though `progress` is a plain sync WIT import, every guest→host call crosses the component-model-async ABI boundary, and *that* boundary crossing — not `UpdateDeadline::Yield` — might be what let `futures::join!` interleave the two `run_concurrent` futures in S1b. If so, S1b's GO would be an artifact of the import calls, not evidence that epoch/fuel-Yield alone can preempt pure CPU-bound guest code, and the peer's S1 would be the correct read on that narrower question.

**The decisive test:** a new export, `burn-pure`, structurally identical to `burn` but with **zero host-import calls anywhere in the loop** — nothing that can create a guest→host call boundary. Same S1b shape otherwise: two separate `Store`s, each with its own `run_concurrent` future, host-level `futures::join!` on one thread, no `Accessor::spawn`. Two independent, import-free interleaving signals, each run against both the epoch lever and the fuel-only lever (4 sub-tests total):
- **symmetric** (both calls do the same 40M-iteration workload): if truly interleaved, both finish within a few ms of each other (ratio of completion times ≈1.0); if sequential/blocking, the second only starts after the first resolves (ratio ≈2.0).
- **asymmetric** (one call does 300M iterations, the sibling does 5M): if truly interleaved, the tiny call finishes quickly regardless of the huge call's total duration; if sequential/blocking and the huge call is polled first, the tiny call cannot even start until the huge one resolves.

### a self-inflicted confound found and fixed along the way

The first `burn-pure` attempt (plain loop, no `black_box`) produced obviously-wrong numbers: the "huge" 300M-iteration call reported `t_a = 5.791µs` with `epoch_deadline_callback hits = 0` (`terra-s1c-run1-BROKEN-optimized-away.txt`). Zero epoch hits over a supposed 300-million-iteration loop is physically impossible if the loop actually ran — the epoch ticker fires every 1ms and this "call" claimed to finish in six *micro*seconds. **LLVM strength-reduced the side-effect-free loop to its closed-form arithmetic-series sum** (`Σ i·C` for `i` in `0..n` has an exact closed form, even under wrapping) and eliminated the loop entirely — a completely different confound than the one being tested for, self-inflicted by writing a "pure" loop that was too pure to survive optimization. Fixed by wrapping the per-iteration values in `std::hint::black_box` (`terra-s1c-guest-build2-blackbox-fix.txt`, then `terra-s1c-run2-blackbox-fix.txt` showing real multi-hundred-ms durations and dozens-to-hundreds of real epoch hits). This is a good general lesson for any future spike measuring "CPU-bound work takes N milliseconds": a loop with no host calls and no `black_box` is not a safe proxy for real CPU-bound guest work once the guest toolchain optimizes aggressively — it needs an explicit optimization barrier or the "work" may not exist at runtime at all.

**Verdict: GO on all 4 sub-tests, reproduced twice** (`terra-s1c-run2-blackbox-fix.txt`, `terra-s1c-run3-reproduced.txt`, both exit 0):

```
run2: S1c-sym-epoch:  GO — a=2850874112 t_a=229.142667ms, b=2850874112 t_b=230.116459ms, ratio=1.00, hits_a=92 hits_b=93
run2: S1c-asym-epoch: GO — huge(a) t_a=877.395ms, tiny(b) t_b=29.769583ms — tiny finished BEFORE huge. hits_a=685 hits_b=12
run2: S1c-sym-fuel:   GO — a t_a=52.581417ms, b t_b=52.643209ms, ratio=1.00
run2: S1c-asym-fuel:  GO — huge(a) t_a=197.267792ms, tiny(b) t_b=6.48875ms — tiny finished BEFORE huge

run3: S1c-sym-epoch:  GO — t_a=224.233167ms, t_b=225.361417ms, ratio=1.01, hits_a=90 hits_b=91
run3: S1c-asym-epoch: GO — huge t_a=859.019875ms, tiny t_b=29.283208ms. hits_a=674 hits_b=12
run3: S1c-sym-fuel:   GO — t_a=50.759417ms, t_b=50.811542ms, ratio=1.00
run3: S1c-asym-fuel:  GO — huge t_a=193.169958ms, tiny t_b=6.353542ms
```

Reading the asymmetric case plainly: a 300M-iteration call and a 5M-iteration call are started at the same instant on two separate `Store`s, joined with a plain `futures::join!`. The 5M-iteration call finishes in **~6-30ms** — while the 300M-iteration sibling is still running for **hundreds more milliseconds** (877ms / 859ms for the epoch variant, 197ms / 193ms for the fuel variant). That is only possible if the host thread was genuinely handing control back and forth between the two `Store`s' guest code — a purely sequential/blocking `run_concurrent` future (the failure mode S1 demonstrated at the `Accessor::spawn` level) would have forced the tiny call to wait for the huge one to fully resolve first, landing its completion at `huge's duration + tiny's own ~few ms`, not far *before* it. `hits_a`/`hits_b` (epoch-callback fire counts, host-side, no guest interaction, so not part of the confound being tested) confirm both `Store`s were genuinely being preempted throughout — 685 hits on the huge call, 12 on the tiny one, proportional to how long each ran before completing.

**This settles the coordinator's (A)/(B) question decisively: (A).** Epoch-Yield and fuel-interval-yield both genuinely preempt pure CPU-bound guest code with **zero** host-import calls in the loop, across **separate** `Store`s, multiplexed by a plain host-level `futures::join!`. S1b's GO was not an import-call artifact — the same mechanism reproduces cleanly once the import confound and the compiler-optimization confound are both removed. The async shard executor may multiplex CPU-bound actors on epoch/fuel-Yield, and does not need to fall back to one-OS-thread-per-hot-actor or budgeted resumable steppers/jobs *purely for the fairness question* — though those may still be the right design for other reasons (backpressure, cancellation granularity, resource accounting) not tested here.

## S9 — sync-lifted export driving an async-lowered import (3rd coordinator follow-up, real schema; coordinator called this "S7" — renumbered here, see verdict table note)

**Naming note:** the coordinator's message labeled this spike "S7". This report already has an
S7 (the sqlx/neo4rs Send compile-check, `driversend`) and S8 (tlsprobe HTTPS GET) from the original
W6 sweep — renumbering this one to **S9** to avoid collision. Nothing about the actual test changed;
the WIT doc comments, Rust `// #region S7` markers, `verdicts_s7` variable names, and every printed
`[host] S7-Q..` log line still literally say "S7" throughout the source and the raw `.txt` logs in
`TICKET_DIR`, because that is what was actually written and run — only this report's prose and
verdict-table slot use "S9" to keep the two unrelated findings addressable by different names.

**Why this exists:** the real `🔌️plugin/🧬️schema/📜️component.wit` declares `interface jobs`
(`start-job`/`step-job`/`cancel-job`) and `interface checkpoint` (`checkpoint`/`restore`) as plain
sync `func` — and `world actor` and `world actor-async` both export the SAME two interfaces
unchanged. The jobs runtime gates `JobCtx::host()` (a job body awaiting a host import) behind
`component-guest-async`. In the sync/poll world, `run_job_to_completion` never pumps `poll`, so a
host-await inside a job deadlocks there already (established separately). If a sync-lifted
`step-job` *also* cannot await an async import in the async world, `JobCtx::host()` would be
unimplementable in **both** worlds — a gate that can never be switched on. The coordinator asked
this be settled by experiment: can a synchronously-lifted export await an asynchronously-lowered
import, in wasmtime 47.0.3, inside a store driven by `run_concurrent`?

**Harness** (added to the asyncprobe fixture, NOT to the live `component.wit` — this is a
standalone probe mirroring the shape, not the real schema):
- `import s7-slow-op: async func(id: u32) -> u32;` — a *deterministic* Pending-then-Ready host
  import (exactly 5 real `poll()` calls before resolving, no wall-clock timing, no background
  thread), so the test can distinguish "the guest's manual poll loop genuinely drove this subtask
  forward" from "nothing happened at all" with zero timing confound.
- `export s7-sync-noop: func(id: u32) -> u32;` — trivial sync export, **zero** import calls, to
  isolate "can this store invoke a plain sync export at all" from "can it await an import".
- `export s7-sync-awaits-import: func(id: u32) -> u32;` — a plain (non-`async fn`) Guest trait
  method, which literally cannot write `.await` — the ONLY way its body can try to drive an async
  import is to construct the import call's future by hand and poll it in a busy-spin loop (capped
  at 2,000,000 spins, returning a `u32::MAX` sentinel on give-up, so a genuine deadlock produces a
  bounded, evidence-carrying result instead of hanging the whole probe process — S1c's own lesson
  about not trusting an impossibly-fast *or* impossibly-silent result, applied here to "don't let
  an unverified assumption about non-termination hang the harness either").
- `export s7-async-awaits-import: async func(id: u32) -> u32;` — control group, identical
  import-awaiting logic, declared the normal (already-proven-to-work) way.

### a self-inflicted bug found before the real result (own harness, not architecture)

First run trapped immediately with `Error: wasm trap: interrupt`, before any S7 output printed
(`terra-s7-run1-BROKEN-epoch-overflow.txt`). Cause: `Store::set_epoch_deadline(delta)` sets the
deadline as `current_epoch + delta`, **not an absolute value** — passing `u64::MAX` as `delta`
(intended to mean "never"), on a store created after S1/S1b/S1c's epoch ticker had already been
running for ~2 real seconds (thousands of ticks elapsed), overflowed the `u64` addition and wrapped
to a deadline already in the past. With no `epoch_deadline_callback` registered, the default
behavior on an already-past deadline is an immediate trap. Fixed with a large-but-safe delta
(`1_000_000`) plus an explicit `epoch_deadline_callback` returning `UpdateDeadline::Continue(..)`
(`terra-s7-host-build2-epoch-fix.txt`, `terra-s7-run2.txt`). Worth recording for its own sake: while
reading wasmtime's `UpdateDeadline` enum to fix this, found `UpdateDeadline::Yield` is documented
to itself **trap if returned during a synchronous invocation** — directly relevant background for
why `Continue`, not `Yield`, is the only safe epoch callback choice for any store a sync export
might be called on.

### the decisive result — NO-GO, and more categorical than the question assumed

```
S7-Q1a-outside-run_concurrent: NO-GO — classic sync call (outside run_concurrent, store otherwise idle) ALSO FAILED: store configuration requires that `*_async` functions are used instead
S7-Q1a-sync-noop:               NO-GO — plain sync `func` export call FAILED: store configuration requires that `*_async` functions are used instead
S7-Q1b-sync-awaits-import:      NO-GO (trap) — sync export awaiting an async import FAILED at the ABI level: store configuration requires that `*_async` functions are used instead
S7-Q3-async-control:            GO (control) — the ASYNC-declared twin resolves normally (107), confirming s7-slow-op itself and the harness are correct
S7-Q2-...-while-run-parked:     SKIPPED — Q1b already failed standalone; running the concurrent-with-run variant could only reconfirm the same failure or risk an undiagnosable hang for zero new information
```

Reproduced twice (`terra-s7-run3.txt`, `terra-s7-run4-reproduced.txt`), byte-identical error text
both times, both exit 0 (the *process* exits cleanly — the sync-call attempt returns a catchable
`wasmtime::Result::Err`, it does not trap the whole process; only my own epoch-overflow bug in
run1 did that).

**This is a cleaner and more absolute finding than the question as posed.** The coordinator's Q1-Q2
frame ("does a sync export's *await* of an async import trap, block, or work") presupposes the
sync export can be *called* at all. It cannot — `S7-Q1a-sync-noop`, which never touches the async
import, fails with the exact same error as `S7-Q1b`. And `S7-Q1a-outside-run_concurrent` (a fresh
test added specifically to isolate this) shows it fails even when the store is completely idle,
with no `run_concurrent` session active, no `run` in flight, no reentrancy at all — a plain call to
`instance.call_s7_sync_noop(&mut store, ..)` the classic non-async way. **A plain sync `func`
export is uncallable, full stop, on any `Store` where `Config::wasm_component_model_async(true)`
is enabled** — not "risky under concurrency", not "only while reentrant", categorically. This
answers Q4 for free too: it has nothing to do with whether `checkpoint`/`restore` ever await
anything internally (they don't, by design) — a genuinely no-op sync export fails identically to
one that tries to await. Q2 (concurrent with `run`) was correctly skipped: since the standalone
case already fails with a hard, deterministic, non-timing-dependent error, adding concurrency
cannot possibly change the outcome, only add wasted build/run time and a small risk of a real hang
on a code path already known to be broken.

**Verdict: (B).** `jobs`/`checkpoint` need `jobs-async`/`checkpoint-async` with `async func` in
`world actor-async`. The exact WIT diff — including the `job-budget`/`job-step` hoist into
`interface types` per rule 20 (a `use`d WIT type is an alias, not a distinct type, so both the sync
and async interfaces must share one canonical definition, not duplicate it) — is in
`TICKET_DIR/terra-s7-component-wit-diff.md`, reproduced here:

```wit
/// hoisted into `interface types`, before its closing brace:
record job-budget {
  fuel: u64,
  deadline-ms: u32,
}
variant job-step {
  running(option<list<u8>>),
  done(list<u8>),
  failed(list<u8>),
}

/// interface jobs (world `actor`, UNCHANGED behavior, just `use`s the hoisted types now):
interface jobs {
  use types.{plugin-error, job-budget, job-step};
  start-job: func(job: u64, kind: string, input: list<u8>) -> result<_, plugin-error>;
  step-job: func(job: u64, budget: job-budget) -> result<job-step, plugin-error>;
  cancel-job: func(job: u64);
}

/// NEW — interface jobs-async (world `actor-async`):
interface jobs-async {
  use types.{plugin-error, job-budget, job-step};
  start-job: async func(job: u64, kind: string, input: list<u8>) -> result<_, plugin-error>;
  step-job: async func(job: u64, budget: job-budget) -> result<job-step, plugin-error>;
  cancel-job: async func(job: u64);
}

/// interface checkpoint (world `actor`) stays exactly as-is.
/// NEW — interface checkpoint-async (world `actor-async`):
interface checkpoint-async {
  use types.{plugin-error};
  checkpoint: async func() -> result<list<u8>, plugin-error>;
  restore: async func(state: list<u8>) -> result<_, plugin-error>;
}

/// world actor: NOT TOUCHED — still exports plain jobs/checkpoint.
/// world actor-async: export jobs-async + checkpoint-async instead of jobs + checkpoint.
```

**Parity-test fallout (plugin-host, not touched here, flagged for the coordinator to
re-specify):** `both_worlds_share_the_same_export_surface_and_actor_is_untouched` will need to stop
asserting identical *interface names* between `actor` and `actor-async` (they now genuinely
differ: `jobs`/`checkpoint` vs `jobs-async`/`checkpoint-async`, by design) and assert instead: (a)
`world actor` itself is byte-identical, zero lines changed; (b) the underlying `job-budget`/
`job-step`/`plugin-error` types are the exact same hoisted definitions in both worlds; (c) function
names/params/returns match 1:1 modulo the interface suffix and `func`/`async func`.

### honest gaps (S9)

- Q2 (sync-awaits-import concurrent with a parked `run()` in the same instance) was never actually
  run — correctly skipped given Q1b's standalone failure, per the reasoning above, but that means
  there is *zero* direct evidence about reentrancy-specific failure modes beyond what Q1a-outside
  already implies (that it's not reentrancy-specific at all, since even the non-reentrant idle-store
  case fails identically).
- Did not test whether declaring ONLY `step-job` as `async func` while leaving `start-job`/
  `cancel-job` sync (or vice versa) is a legal *mixed* interface — the real fix hoists all three
  together into `jobs-async`, matching the coordinator's ask, and mixed-sync-async single interfaces
  were not probed since nothing in the coordinator's question needed that shape.
- The exact wasmtime-internal source location enforcing "store configuration requires *_async
  functions" was not traced to a cited line (same category of gap as S1's fiber-scheduler
  internals) — the error message itself was treated as sufficient, reproducible, decisive evidence
  for a NO-GO, consistent with how S3's original discovery of this same message was handled.
- `s7-slow-op`'s 5-poll Pending-then-Ready shape is a synthetic, deterministic stand-in for a real
  `host-async` import; a job body awaiting a REAL host-async call (e.g. an actual I/O-bound one
  with genuine wall-clock delay) was not tested, though there is no reason to expect the ABI-level
  "uncallable" failure to depend on the import's own timing characteristics — it fails before ever
  reaching the import at all (`S7-Q1a-sync-noop`, no import involved, same error).


## commands + exit codes

All builds used `CARGO_TARGET_DIR=$TICKET_DIR/🎯️target-probe` (absolute path — see honest gaps for one self-inflicted detour on this).

```
$ cargo build -p semio-asyncprobe-guest --release --target wasm32-wasip2   # (repeated after each WIT/guest edit)
Finished `release` profile [optimized] target(s) in 0.48s–49.41s
exit 0 (every time)

$ cargo build -p semio-asyncprobe-host --release   # (repeated after each host edit)
Finished `release` profile [optimized] target(s) in 0.45s–2m 50s (first pull of wasmtime 47.0.3 deps)
exit 0 (final state; two intermediate iterations had real compile errors, fixed and reconfirmed — see below)

$ ASYNCPROBE_WASM=.../semio_asyncprobe_guest.wasm .../semio-asyncprobe-host
exit 0 (all 4 runs: run1 baseline-instrumentation, run2 bigger-iters, run3 spawn-based S1, final = run3 copy)

$ cargo build -p semio-asyncprobe-driversend --release
Finished `release` profile [optimized] target(s) in 1m 09s
exit 0

$ .../semio-asyncprobe-driversend
exit 0

$ cargo build -p semio-asyncprobe-tlsprobe --release
Finished `release` profile [optimized] target(s) in 1m 25s
exit 0

$ .../semio-asyncprobe-tlsprobe
exit 0 (real network GET, status 200)

$ CARGO_TARGET_DIR=$TICKET_DIR/🎯️target-probe cargo build -p semio-asyncprobe-host --release --manifest-path 🖥️host/Cargo.toml   # S1b addition, guest unchanged (no wasm rebuild needed)
Finished `release` profile [optimized] target(s) in 10.41s (terra-s1b-build1.txt), then 0.13s no-op reconfirm (terra-s1b-build2.txt)
exit 0 (both)

$ ASYNCPROBE_WASM=.../semio_asyncprobe_guest.wasm .../semio-asyncprobe-host   # S1b runs, reusing the unchanged guest wasm
exit 0 (terra-s1b-run1.txt: 149 switches; terra-s1b-run2.txt: 139 switches — reproduced twice)

$ ASYNCPROBE_WASM=.../semio_asyncprobe_guest.wasm .../semio-asyncprobe-host   # plain re-run of the pre-S1b binary, sanity-checking the ORIGINAL report before extending it
exit 0 (terra-probe-spikes-reverify.txt — S1/S2/S3/S5/S6 output byte-identical in shape to the original report)
```

**S1c builds/runs used `CARGO_TARGET_DIR=<scratchpad>/target-probe-s1c` instead of the ticket-folder target dir** — the coordinator's message for this round stated the ticket-folder target dir now fails `EPERM`; run logs were copied into `TICKET_DIR` as `.txt` scratch files afterward, per binding rule 6.

```
$ CARGO_TARGET_DIR=<scratchpad>/target-probe-s1c cargo build -p semio-asyncprobe-guest --release --target wasm32-wasip2 --manifest-path 👽️guest/Cargo.toml   # burn-pure added to WIT + guest
Finished `release` profile [optimized] target(s) in 12.85s
exit 0 (terra-s1c-guest-build1... — this first version had the black_box bug, see below)

$ CARGO_TARGET_DIR=<scratchpad>/target-probe-s1c cargo build -p semio-asyncprobe-host --release --manifest-path 🖥️host/Cargo.toml   # S1c driving code added
Finished `release` profile [optimized] target(s) in 2m 39s (first pull of deps into the fresh scratchpad target dir; a concurrent re-run of the same command hit "Blocking waiting for file lock" and finished in 28.36s once the lock cleared — both exit 0, per binding rule 4's "wait, never kill")
exit 0 (terra-s1c-host-build.txt)

$ ASYNCPROBE_WASM=<scratchpad>/target-probe-s1c/.../semio_asyncprobe_guest.wasm <scratchpad>/target-probe-s1c/release/semio-asyncprobe-host
exit 0 (terra-s1c-run1-BROKEN-optimized-away.txt — burn-pure's loop was silently eliminated by LLVM, see "a self-inflicted confound" in the S1c section; hits_a=hits_b=0, microsecond "durations")

$ CARGO_TARGET_DIR=<scratchpad>/target-probe-s1c cargo build -p semio-asyncprobe-guest --release --target wasm32-wasip2 --manifest-path 👽️guest/Cargo.toml   # black_box fix
Finished `release` profile [optimized] target(s) in 0.29s
exit 0 (terra-s1c-guest-build2-blackbox-fix.txt)

$ ASYNCPROBE_WASM=<scratchpad>/target-probe-s1c/.../semio_asyncprobe_guest.wasm <scratchpad>/target-probe-s1c/release/semio-asyncprobe-host   # x2, reproducibility
exit 0, exit 0 (terra-s1c-run2-blackbox-fix.txt: all 4 GO; terra-s1c-run3-reproduced.txt: all 4 GO, consistent numbers)
```

**S9 builds/runs, also under `CARGO_TARGET_DIR=<scratchpad>/target-probe-s7` (directory name kept as `s7`, matching the code's internal naming — see the S9 section's naming note):**

```
$ CARGO_TARGET_DIR=<scratchpad>/target-probe-s7 cargo build -p semio-asyncprobe-guest --release --target wasm32-wasip2 --manifest-path 👽️guest/Cargo.toml   # s7-* WIT additions
Finished `release` profile [optimized] target(s) in 15.21s
exit 0 (terra-s7-guest-build.txt — mixed sync/async Guest trait methods compiled cleanly on the first try)

$ CARGO_TARGET_DIR=<scratchpad>/target-probe-s7 cargo build -p semio-asyncprobe-host --release --manifest-path 🖥️host/Cargo.toml
Finished `release` profile [optimized] target(s) in 1m 35s
exit 0 (terra-s7-host-build1.txt — accessor.with(|access| instance.call_s7_sync_noop(access,..)) and the classic &mut store form both compiled on the first try)

$ ASYNCPROBE_WASM=<scratchpad>/target-probe-s7/.../semio_asyncprobe_guest.wasm <scratchpad>/target-probe-s7/release/semio-asyncprobe-host
exit 1 (terra-s7-run1-BROKEN-epoch-overflow.txt — Error: wasm trap: interrupt; own harness bug, see S7 section, not an architectural finding)

$ CARGO_TARGET_DIR=<scratchpad>/target-probe-s7 cargo build -p semio-asyncprobe-host --release --manifest-path 🖥️host/Cargo.toml   # epoch-deadline overflow fix
Finished `release` profile [optimized] target(s) in 9.61s
exit 0 (terra-s7-host-build2-epoch-fix.txt)

$ ASYNCPROBE_WASM=<scratchpad>/target-probe-s7/.../semio_asyncprobe_guest.wasm <scratchpad>/target-probe-s7/release/semio-asyncprobe-host
exit 0 (terra-s7-run2.txt — first clean result: Q1a/Q1b NO-GO, Q3 control GO, Q2 correctly skipped)

$ CARGO_TARGET_DIR=<scratchpad>/target-probe-s7 cargo build -p semio-asyncprobe-host --release --manifest-path 🖥️host/Cargo.toml   # added the outside-run_concurrent test
Finished `release` profile [optimized] target(s) in 9.55s
exit 0 (terra-s7-host-build3-add-outside-test.txt)

$ ASYNCPROBE_WASM=<scratchpad>/target-probe-s7/.../semio_asyncprobe_guest.wasm <scratchpad>/target-probe-s7/release/semio-asyncprobe-host   # x2, reproducibility
exit 0, exit 0 (terra-s7-run3.txt, terra-s7-run4-reproduced.txt — identical error text both times, deterministic non-timing-dependent result)
```

Real compile errors hit and fixed along the way (all captured live, none guessed around):
- `poll_produce` return type mismatch (`anyhow::Error` vs `wasmtime::Error`) — trait requires the latter exactly.
- `Access::get()` requires `&mut self` — closures passed to `accessor.with(...)` needed `|mut access|`.
- `instance.call_checkpoint(accessor)` on a **sync** WIT export doesn't accept `&Accessor` (wants `impl AsContextMut`) — first real signal that sync exports don't fit this store's async-only calling convention; confirmed by then trying `accessor.with(|access| instance.call_checkpoint(access))` which compiled but errored *at runtime* with `"store configuration requires that *_async functions are used instead"` — the actual S3 finding.
- `Asyncprobe` (bindgen-generated world struct) implements neither `Clone` nor `Copy` — `Accessor::spawn`'s `AccessorTask: 'static` bound needed each spawned task to own a distinct `Instance`, so S1 instantiates 2 additional dedicated instances (`instance_s1a`/`instance_s1b`) rather than cloning.

## code patterns that worked

**S1 — `Accessor::spawn` + `AccessorTask` + result channel** (a real wasmtime-level way to run two tasks concurrently *within one `Store`*, but did not achieve CPU-bound interleaving. **Per S1b below, this is NOT the pattern for spawning pooled actors** — use one `Store` per actor and multiplex at the host level instead. Kept here as a documented dead end, not a template):

```rust
struct BurnTask {
    instance: Asyncprobe,
    guest_id: u32,
    iters: u32,
    result_tx: futures::channel::oneshot::Sender<wasmtime::Result<u32>>,
}
impl AccessorTask<HostState> for BurnTask {
    async fn run(self, accessor: &Accessor<HostState>) -> wasmtime::Result<()> {
        let r = self.instance.call_burn(accessor, self.guest_id, self.iters).await;
        let _ = self.result_tx.send(r);
        Ok(())
    }
}
let (tx_a, rx_a) = futures::channel::oneshot::channel();
let (tx_b, rx_b) = futures::channel::oneshot::channel();
accessor.spawn(BurnTask { instance: instance_s1a, guest_id: 0, iters, result_tx: tx_a })?;
accessor.spawn(BurnTask { instance: instance_s1b, guest_id: 1, iters, result_tx: tx_b })?;
let a = rx_a.await.expect("burn(0) task dropped without sending")?;
let b = rx_b.await.expect("burn(1) task dropped without sending")?;
```

Engine/store setup required for the fuel+epoch+async combination (S1's other half — this part *did* work, just didn't produce fairness):

```rust
config.wasm_component_model_async(true);
config.concurrency_support(true);
config.consume_fuel(true);
config.epoch_interruption(true);
// ...
store.set_fuel(u64::MAX)?;
store.epoch_deadline_callback(move |_ctx| { hits.fetch_add(1, Ordering::Relaxed); Ok(UpdateDeadline::Yield(1)) });
store.set_epoch_deadline(1);
// 1ms ticker on its own OS thread:
std::thread::spawn(move || { while !stop.load(Ordering::Relaxed) { engine.increment_epoch(); std::thread::sleep(Duration::from_millis(1)); } });
```

**S5 — custom `StreamProducer` with real cross-thread wake** (the exact shape the "grant drained → synthesized TurnResult" design needs):

```rust
impl StreamProducer<HostState> for WakeyProducer {
    type Item = u32;
    type Buffer = VecBuffer<u32>;
    fn poll_produce<'a>(
        self: Pin<&mut Self>, cx: &mut Context<'_>, _store: StoreContextMut<'a, HostState>,
        mut destination: Destination<'a, u32, VecBuffer<u32>>, _finish: bool,
    ) -> Poll<wasmtime::Result<StreamResult>> {
        let mut shared = self.shared.lock().unwrap();
        shared.poll_count += 1;
        if shared.queue.is_empty() {
            if shared.done { return Poll::Ready(Ok(StreamResult::Dropped)); }
            shared.waker = Some(cx.waker().clone());   // 🎯️ the core assertion
            return Poll::Pending;
        }
        let items: Vec<u32> = shared.queue.drain(..).collect();
        destination.set_buffer(items.into());
        Poll::Ready(Ok(StreamResult::Completed))
    }
}
```

A plain background `std::thread::spawn` mutating `shared.queue` then calling `waker.wake()` (fully decoupled from the wasmtime executor thread) is enough to resume the guest — no host-side async task/spawn needed on the producer side.

**S1b — one `Store` per actor, host-level `futures::join!`** (the actual pattern later packets should copy for the pooled-actor runtime — this is what a real scheduler's inner loop should look like, generalized from `join!` to `FuturesUnordered`/`select!` over N actors):

```rust
let fut_a = store_a.run_concurrent(async move |accessor: &Accessor<HostState>| -> Result<u32> {
    Ok(instance_a.call_burn(accessor, 20, iters).await?)
});
let fut_b = store_b.run_concurrent(async move |accessor: &Accessor<HostState>| -> Result<u32> {
    Ok(instance_b.call_burn(accessor, 21, iters).await?)
});
let (ra, rb) = futures::join!(fut_a, fut_b);   // host-level, current-thread, NOT Accessor::spawn
```

Each store independently configured with either epoch-Yield (`store.epoch_deadline_callback(|_| Ok(UpdateDeadline::Yield(1)))` + `store.set_epoch_deadline(1)` + a shared `engine.increment_epoch()` ticker) or `store.fuel_async_yield_interval(Some(n))` — both proven to work as the yield trigger; they can share one `Engine` (epoch variant) or not (fuel-only variant needs no epoch config at all).

**S1c — CPU-bound loop that survives optimization** (the pitfall to avoid when writing any future "pure CPU work" guest probe):

```rust
async fn burn_pure(_guest_id: u32, iters: u32) -> u32 {
    let mut acc: u32 = 0;
    let mut i: u32 = 0;
    while i < iters {
        let step = std::hint::black_box(i).wrapping_mul(2654435761);
        acc = std::hint::black_box(acc.wrapping_add(step));
        i = i.wrapping_add(1);
    }
    acc
}
```

Without both `black_box` calls, LLVM strength-reduces this exact loop shape (linear induction variable, wrapping arithmetic, no side effects) to its closed-form arithmetic-series sum and the "300M-iteration" call returns in ~6 microseconds with zero epoch-callback hits — see "a self-inflicted confound" in the S1c section above. Each `run_pure_pair(...)` future timestamps its OWN completion the instant its own `await` resolves (via a shared `Arc<Mutex<Option<Duration>>>` written right after `run_concurrent(...).await??`), independent of when the outer `futures::join!` itself returns — that per-future timestamp, not the `join!`'s own return time, is what lets "B finished quickly while A kept running" be distinguished from "B only started, and finished, right after A".

## fallbacks now required

- **S1 (no fallback needed — the design's actual shape is GO, see S1b):** epoch-Yield does not give fairness for two CPU-bound guest tasks spawned via `Accessor::spawn` inside ONE `Store`. **Do not use that shape** for pooled actors. This is not the design's shape anyway — keep it on record as a "don't reach for `Accessor::spawn` across actors" warning, not as an architecture blocker.
- **S1b (the design's actual shape): no fallback needed — GO, and S1c confirms it's real.** One `Store` per actor, host-level `futures::join!`/`select!`/`FuturesUnordered` across actors, epoch-Yield(1) at ~1ms granularity (or `fuel_async_yield_interval` for finer, deterministic, wall-clock-independent slicing — both work, can be combined). This is the mechanism the pooled-actor runtime should build on.
- **S1c (zero-host-import pure-CPU preemption): no fallback needed — GO on all 4 sub-tests.** Settles the question a peer session's contradictory S1 report and the coordinator's confound hypothesis raised: epoch/fuel-Yield genuinely preempts pure CPU-bound guest code with no host-import calls involved at all, across separate `Store`s, host-level-`join!`'d. The pooled-actor runtime does not need to fall back to one-OS-thread-per-hot-actor, an explicit guest-cooperation yield intrinsic, or budgeted resumable steppers/jobs *purely to get fairness* — though those may still be worth having for other reasons (backpressure, cancellation granularity, resource accounting), none of which this spike tested.
- **S3:** confirmed working, but *only* for `async func` exports — **and this is the schema-relevant part**: any WIT export meant to be callable while another call on the *same instance* is in flight — this includes the `checkpoint`-shaped export the pooled-actor design needs — **must be declared `async func` in the schema, never plain `func`**, once the store is `wasm_component_model_async(true)`. Plain sync exports on such a store can still be *declared* and *called when nothing else is in-flight*, but the generated call errors at runtime the moment concurrent access matters — not a compile-time signal, a footgun for later packets to know about. Action item for whoever writes the production checkpoint export: declare it `async func` from the start, don't let it get added as a plain `func` and discover this at runtime later.
- **S2:** none needed — drop-on-cancel confirmed working as designed.
- **S6:** none needed — nested `Rc<RefCell>` executor confirmed working; SDK does not need to special-case `wit_bindgen::rt::async_support::spawn`.
- **S9 (blocking, fallback required and specified): plain sync `func` exports (`jobs`, `checkpoint` in the REAL `🔌️plugin/🧬️schema/📜️component.wit`) are categorically uncallable on any `Store` with `wasm_component_model_async(true)` enabled — this generalizes and hardens the S3 footgun above from "risky under concurrency" to "broken regardless of concurrency, even on an idle store". `world actor-async` needs `jobs-async`/`checkpoint-async` with `async func`, sharing `job-budget`/`job-step`/`plugin-error` via a `types`-hoisted `use`, exact diff in the S9 section and `TICKET_DIR/terra-s7-component-wit-diff.md` (filename kept as `s7`, matching the code's internal naming). `world actor` (the sync/poll world) is untouched. `JobCtx::host()` becomes implementable once `jobs-async::step-job` exists; it stays exactly as unimplementable as before in `world actor`'s sync `jobs`, which was never in scope for this fix.

## honest gaps

- S1's root cause inside wasmtime's fiber/task scheduler was not traced to source-level certainty — the *behavior* (zero interleaving across two independent test methodologies, with confirmed frequent epoch-callback firing) is solid and reproducible, but *why* wasmtime's scheduler doesn't round-robin to a never-yet-polled sibling task after a Yield-triggered pend was not confirmed by reading `concurrent.rs`'s scheduler-selection logic itself (that code is large and the spike's time budget went to breadth across S1-S8 instead). Flagging as FAIL with strong evidence rather than UNRESOLVED because the observable behavior is unambiguous, but the exact wasmtime-internal mechanism is inference, not a cited source line.
- `store.epoch_deadline_async_yield_and_update(delta)` was read (in `runtime/store/async_.rs`) and confirmed to be *exactly* equivalent to the custom callback used here (`Box::new(move |_store| Ok(UpdateDeadline::Yield(delta)))`) — not separately runtime-tested since it would produce identical behavior, but noted in case a future packet is tempted to try it as an alternative to the manual callback.
- S1 was not tried with more than 2 concurrently-spawned tasks, nor with tasks of unequal iteration counts, nor with fuel actually being the limiting/triggering mechanism instead of epoch (fuel was set to `u64::MAX` purely to avoid an unrelated immediate trap, never exercised as the interruption trigger itself).
- Self-inflicted detour early in this run: `CARGO_TARGET_DIR` set as a *relative* path in one early command left two stray build-artifact directories nested inside `👽️guest/.🧬semio/...` and `🖥️host/.🧬semio/...` (Bash tool shell state does not persist env vars across separate tool calls, and cwd changes make relative `CARGO_TARGET_DIR` resolve unpredictably). Both were found and `rm -rf`'d before finishing — verified via `find` that the fixture tree now contains only source files (`Cargo.toml`/`Cargo.lock`/`🦀️main.rs`/`.wit`) plus the intentional `TICKET_DIR/🎯️target-probe` build output. Mentioning this so nobody mistakes stray build dirs for something intentional if they reappear.
- Did not attempt to reduce S1's iteration count to find the precise crossover point (if any) where interleaving might start happening — 200K and 40M iterations both showed zero interleaving, so this wasn't pursued further; a future packet chasing the fallback might want to sweep this.
- **S1b was only tried with exactly 2 actors, equal iteration counts (40M each), a single fixed epoch-ticker interval (1ms) and a single fixed fuel-yield interval (500,000).** Not tried: 3+ concurrently-running actors (does fairness degrade or stay round-robin-ish with more?), unequal workloads (does a much-larger burn starve a much-smaller one between yield points?), or sweeping the epoch-ticker/fuel-interval to find where the granularity/overhead tradeoff bites. The GO verdict is about the *mechanism working at all*, not about scheduling quality (round-robin vs. something else) under load — that's a fair follow-up for whichever packet builds the real scheduler.
- S1b's switch-count variance between runs (139 vs 149 for the epoch variant) is expected and consistent with real wall-clock scheduling jitter from the `Duration::from_millis(1)` ticker thread (OS thread-sleep is not exact); the fuel-only variant's switch count was exactly 3041 both runs, as expected since fuel consumption is deterministic per instruction and does not depend on wall-clock scheduling at all.
- S1b's fairness check (`switches > 0`) proves interleaving happened at all, not that it is *evenly* distributed between the two actors — the printed switch log (10 points per run) was eyeballed and looks alternating/balanced, but no formal balance metric (e.g. max consecutive-turns-by-one-actor) was computed.
- **S1c was only tried with 2 fixed workload shapes** (symmetric 40M/40M, asymmetric 300M/5M) and the same single epoch-ticker interval (1ms) / fuel-yield interval (500,000) as S1b — same scope limitation as S1b, now inherited by S1c. Did not sweep the asymmetric ratio to find where (if anywhere) fairness degrades, and did not try 3+ concurrent pure-CPU actors.
- **S1c does not explain why S1b's own `burn` (WITH the `progress` import) took ~150-200ms for 40M iterations while `burn_pure`'s honest (post-`black_box`) 40M-iteration run took only ~50-230ms depending on lever** — the two aren't quite comparable (different anti-optimization pressure, different per-iteration overhead from the import calls vs. `black_box` barriers), and no attempt was made to isolate how much of S1/S1b's measured wall-time was "real" loop work versus import-call overhead. This doesn't affect the S1c verdict (which is about interleaving, not absolute timing), but it's a loose thread — anyone using these numbers to estimate real per-actor CPU-slice costs for the production scheduler should re-measure with a workload representative of actual plugin logic, not this synthetic loop.
- The peer session's original contradictory S1 report (via `Accessor::spawn` AND `futures::join!`) was not independently re-run in this packet — its result is taken at face value from the coordinator's message, and its own numbers/logs were not inspected directly. Given S1's original result (also via both `join!` and `Accessor::spawn`, at the `Accessor::spawn`-inside-one-`Store` level) already matches what the peer reported, and S1c now independently confirms the opposite mechanism (separate `Store`s + host-`join!`) is GO, the discrepancy is adequately explained by "different shape being tested" without needing to re-run the peer's exact commands — but this is inference from consistency, not a byte-for-byte diff against their logs.
- **S9's gaps are listed in full in its own section above** (Q2 never actually run, mixed-sync-async single interfaces not probed, exact wasmtime source line not cited, only a synthetic deterministic import tested rather than a real `host-async` one) — not repeated here to avoid drift between two copies of the same list.

## files touched

- `🧰️framework/🛍️products/💻️os/🧫️fixtures/🔌️asyncprobe/👽️guest/🧬️schema/📜️world.wit` — extended world with S1-S6 imports/exports; **+S1c addition**: `export burn-pure: async func(guest-id: u32, iters: u32) -> u32;` (zero host-import calls in its loop, unlike `burn`).
- `🧰️framework/🛍️products/💻️os/🧫️fixtures/🔌️asyncprobe/👽️guest/🦀️component.rs` — guest impls for `burn`/`checkpoint`/`cancel_probe`/`nested_exec_probe`; **+S1c addition**: `burn_pure` impl, `std::hint::black_box`-guarded after the first attempt was compiled away entirely (see S1c section).
- `🧰️framework/🛍️products/💻️os/🧫️fixtures/🔌️asyncprobe/🖥️host/🦀️main.rs` — host impls, epoch/fuel wiring, S1-S6 driving code, S4 compile probes; **+S1b addition**: `progress_log`/`progress_start` generalized to a shared `Arc<Mutex<Vec<(u32,u32,Duration)>>>` + timestamp so two separate `HostState`s can share one chronological log, plus the whole S1b block (two-`Store` epoch variant + fuel-only variant with its own `Engine`/`Component`/`Linker`) inserted after the S1-S6 `run_concurrent` call; **+S1c addition**: `run_pure_pair(...)` helper (two fresh `Store`s + per-future completion-timestamp capture) driving 4 sub-tests (epoch×symmetric, epoch×asymmetric, fuel×symmetric, fuel×asymmetric) against `burn_pure`, inserted after the S1b block, plus its own fuel-only `Engine`/`Component`/`Linker` trio (`config_fuel_c`/`engine_fuel_c`/`component_fuel_c`/`linker_fuel_c`).
- `🧰️framework/🛍️products/💻️os/🧫️fixtures/🔌️asyncprobe/🩺️driversend/` (new) — `Cargo.toml` + `🦀️main.rs`, S7.
- `🧰️framework/🛍️products/💻️os/🧫️fixtures/🔌️asyncprobe/🌐️tlsprobe/` (new) — `Cargo.toml` + `🦀️main.rs`, S8.
- `TICKET_DIR/terra-probe-spikes-run1.txt`, `-run2.txt`, `-run3.txt`, `-final-run.txt` — verbatim run logs (scratch), original S1-S8 spike.
- `TICKET_DIR/terra-probe-spikes-reverify.txt` — sanity re-run of the pre-S1b binary before extending it, confirming the original report's S1/S2/S3/S5/S6 output still reproduces unchanged.
- `TICKET_DIR/terra-s1b-build1.txt`, `-build2.txt`, `-run1.txt`, `-run2.txt` — S1b build + run logs (scratch).
- `TICKET_DIR/terra-s1c-guest-build1-BROKEN-optimized-away.txt`, `-guest-build2-blackbox-fix.txt`, `-host-build.txt`, `-run1-BROKEN-optimized-away.txt`, `-run2-blackbox-fix.txt`, `-run3-reproduced.txt` — S1c build + run logs (scratch), copied in from the scratchpad target dir after each command; filenames deliberately keep "BROKEN" for the compiler-optimization confound attempt so nobody mistakes it for a real result later.
- `TICKET_DIR/🎯️target-probe/` — shared build output dir for the original four crates in this spike (guest/host/driversend/tlsprobe) plus S1a/S1b additions, per binding rule 4.
- `<scratchpad>/target-probe-s1c/` — S1c's build output dir; the coordinator's message for this round stated the ticket-folder target dir now fails `EPERM`, so S1c alone used the session scratchpad instead (`CARGO_TARGET_DIR=<scratchpad>/target-probe-s1c`) — not persisted with the ticket, only the copied-in `.txt` logs above are.
- `🧰️framework/🛍️products/💻️os/🧫️fixtures/🔌️asyncprobe/👽️guest/🧬️schema/📜️world.wit` — **+S9 addition** (internally labeled S7 in code, see naming note): `import s7-slow-op`, `export s7-sync-noop`, `export s7-sync-awaits-import`, `export s7-async-awaits-import`.
- `🧰️framework/🛍️products/💻️os/🧫️fixtures/🔌️asyncprobe/👽️guest/🦀️component.rs` — **+S9 addition**: `s7_sync_noop`/`s7_sync_awaits_import` (plain, non-`async fn` Guest trait methods; the latter manually spin-polls the import future, spin-capped at 2,000,000) and `s7_async_awaits_import` (control group).
- `🧰️framework/🛍️products/💻️os/🧫️fixtures/🔌️asyncprobe/🖥️host/🦀️main.rs` — **+S9 addition**: `s7_slow_op` import impl (deterministic 5-poll Pending-then-Ready), and the whole S9 driving block (`store7`/`store7b`, `verdicts_s7`) inserted after the S1c block.
- `TICKET_DIR/terra-s7-component-wit-diff.md` — the exact WIT diff for the coordinator to apply to the real `🔌️plugin/🧬️schema/📜️component.wit` (not applied to the live tree, per instructions).
- `TICKET_DIR/terra-s7-guest-build.txt`, `-host-build1.txt`, `-host-build2-epoch-fix.txt`, `-host-build3-add-outside-test.txt`, `-run1-BROKEN-epoch-overflow.txt`, `-run2.txt`, `-run3.txt`, `-run4-reproduced.txt` — S9 build + run logs (scratch); `-BROKEN-epoch-overflow` kept named that way so the self-inflicted `set_epoch_deadline` overflow bug (own harness, not architecture) is never mistaken for a real result.
- `<scratchpad>/target-probe-s7/` — S9's build output dir, same `EPERM`-driven scratchpad reasoning as S1c's; not persisted with the ticket, only the copied-in `.txt` logs above are.
