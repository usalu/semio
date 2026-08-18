# terra-shard-routing report

## root cause

Not a misrouting bug — `ShardTable::pin` and `ParallelRuntime`'s shard-index lookups were already
internally consistent (verified by reading `ShardTable::pin`/`Scheduler::register_actor`/
`Kernel::activate`/`ParallelRuntime::activate`/`tick_and_dispatch`: every one of them derives an
actor's shard from the SAME `ShardTable::pin` assignment, and `Scheduler`'s own per-actor `shard`
field is set from that exact value at registration time — `🧰️framework/🔨️modules/🎭️actor/🦀️component.rs:2195-2196`). A grant was always addressed to the *correct* shard.

The real defect is a cross-thread ordering race in
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🏃️executor.rs`, introduced by this
same ticket wave (`terra-kernel-loop`) when `ShardExecutor::register` became a live, post-spawn
call:

- `ShardExecutor::register` (old code, was line 150-152) only did `self.register_tx.send((actor,
  instance))` and returned immediately — **fire-and-forget**, no confirmation the executor thread
  had applied it.
- The executor thread's own loop (`spawn`, old lines 114-132) drains `register_rx` **once per loop
  iteration**, then blocks in `park.recv_deadline(PARK_TIMEOUT)` (now line 136) waiting for the
  *next* `ShardFrame` on the transport — a **completely independent** `std::sync::mpsc` channel
  from `register_rx`.
- If that `recv_deadline` call is already blocked (parked) at the moment a caller sends a
  `register()` request, the request sits unclaimed in `register_rx` — nothing wakes
  `recv_deadline` on `register_rx` activity, only on `ShardFrame` transport activity.
- `ParallelRuntime::activate` (`🎠️runtime.rs:159-172`) calls `shard.executor.register(actor,
  instance)` and then returns; its caller (real production path: `submit()` +
  `tick_and_dispatch()`, `🎠️runtime.rs:178-215`) immediately sends a `ShardFrame::Grant`/`Suspend`/
  `Resume` for that SAME actor over the transport. That send **wakes the parked `recv_deadline`
  right away**, and the loop dispatches it via `pump_primed` (`🧵️shard/🦀️component.rs:285`)
  **before** looping back to drain the still-unprocessed `register_rx` entry.
- `ShardLoop::pump_primed` finds no entry for the actor in `self.instances`
  (`🧵️shard/🦀️component.rs:306-309`, `377-380`, and the `Suspend`/`Resume` arms at `461-462`/
  `476-477`) and emits exactly the fault text from the bench: `"ShardLoop::pump: actor N is not
  registered on this shard"`.

The old code's own doc comment (`register_tx` field, pre-fix) asserted this was safe because
"both the register channel and the ShardFrame transport are drained fully, in this fixed order,
every iteration" — that reasoning only holds for messages already queued **before** an iteration
starts polling; it does not hold once the loop is already parked waiting on the *other* channel,
which is exactly the timing `activate()` immediately followed by `submit`+`tick_and_dispatch`
produces. This is why it could never show up with one shard *before* this ticket (there was no
live `register()` call at all — `ShardExecutor::register` was net-new this wave) and why it got
dramatically more probable once `ParallelRuntime` started driving many real `activate`+`tick`
round trips back to back across K shards.

## fix

`🖥️host/🧵️shard/🏃️executor.rs`:

- `register_tx`'s payload became `RegisterRequest { actor, instance, ack: mpsc::Sender<()> }`
  (line 66-70) instead of a bare tuple.
- The executor thread's drain loop now applies the registration *and* acks it in the same step
  (line 128-131: `shard.register(actor, instance); let _ = ack.send(());`).
- `ShardExecutor::register` (line 163-169) now **blocks** on `ack_rx.recv_timeout
  (REGISTER_ACK_TIMEOUT)` (5s ceiling, `REGISTER_ACK_TIMEOUT` at line 38 — comfortably above the
  5ms `PARK_TIMEOUT` a healthy executor ever takes to notice a request; an executor that has
  already stopped drops the buffered request's `ack` sender, so a dead/stopped shard returns
  promptly instead of hanging the full timeout).

This turns "queued on an unrelated channel before" into a real cross-thread happens-before: the
executor thread inserts into `shard.instances` and only *then* sends the ack; the caller's
subsequent `submit`/`tick_and_dispatch` cannot execute until that ack is received. `ShardTable::
pin`'s least-loaded placement is untouched — the fix is entirely in the registration handoff, not
in shard assignment, so budget 3's 25/25/25/25 balance is preserved (confirmed below).

## budget 7 verdict

**Same cause.** `Env::activate` (`📦️glue.rs:766`) wraps `ParallelRuntime::activate`, and budget
7's harness (`budget_7_stateful`, `📦️glue.rs:1166-1170`) does exactly the race-triggering sequence:
`env.activate(...)` (register) immediately followed by `env.send_payload(actor_b, Payload::Resume
{...})` + `env.pump()` (dispatch). Before the fix this reliably reached the still-unregistered
fresh instance, producing a `ShardOutcome::Fault` where a `Resumed` was expected —
`resumed: false` — while the trailing re-`Suspend` (run after enough time had passed for the
register to land asynchronously) still happened to hash-match the original checkpoint because the
underlying wasm fixture's checkpoint content did not depend on whether `restore()` had actually
run for this input. After the fix: `resumed: true`, hashes still identical — see the bench
before/after table.

## property tests added

Both added to `🖥️host/🧵️shard/🏃️executor.rs`'s own `#[cfg(test)] mod tests` (the file that owns
the fix), because `MockGuestRuntime` is `#[cfg(test)]`-gated *inside* the plugin-host crate and is
not visible to `semio-framework-os-renderer-wgpu`'s own tests as an external dependency — so a
property test exercising the real `ShardExecutor` + `ShardTable::pin` combination has to live here,
not in `🎠️runtime.rs`.

- `every_actors_grant_lands_on_the_shard_it_was_registered_on_across_k_shards` — spins up 4 real
  `ShardExecutor` threads, pins 200 actors via a real `ShardTable`, and for each one calls
  `register()` immediately followed by dispatching its own `Grant`, with zero slack — the exact
  interleaving `ParallelRuntime::activate` → `submit`+`tick_and_dispatch` produces. Asserts every
  single one resolves to `ShardOutcome::Turn` for itself, never a `Fault`.
- `suspend_then_resume_round_trip_lands_on_a_shard_where_the_actor_is_registered` — mirrors budget
  7's own shape: for 60 actors, checkpoint via `Suspend`, then register a FRESH instance for the
  same actor id and immediately dispatch `Resume`, asserting `Resumed` (never `Fault`) every time.

Both are property tests, not mechanism tests, by design (per the ticket's own history: this class
of bug — a real property silently failing while a single-round-trip mechanism test stayed green —
has bitten this ticket twice already, `perShardCounts {0:100}` for three waves and every job
completion failing to serialize).

**Validated, not assumed**: I temporarily reverted `ShardExecutor::register` to the old
fire-and-forget behavior (via `Edit`, no git) and re-ran just these two tests — both failed with
the identical fault text the bench reported (`"ShardLoop::pump: actor N is not registered on this
shard"` / `"...Suspend for actor N which is not registered on this shard"`). I then restored the
real fix and re-ran the full suite to confirm 115/0/1 again. This proves the tests actually catch
the bug rather than passing by construction.

## commands + exit codes

All four re-run with the exit code captured BEFORE any pipe to `tail` (`{ cmd > file 2>&1; echo
"REAL_EXIT:$?"; }`, then `tail` on the saved file separately) — the ticket's own binding rule 6
note that `cmd | tail; echo $?` reports `tail`'s status, not the command's, applies to my own
earlier runs in this session too, so these are the corrected, trustworthy captures:

```
$ CARGO_TARGET_DIR=<TICKET_DIR>/🎯️target-sr cargo test -p semio-framework-plugin-host --lib -- --skip schema_parity
...
test result: ok. 115 passed; 0 failed; 1 ignored; 0 measured; 4 filtered out; finished in 0.90s
REAL_EXIT:0
```
(115 = baseline 113 + the 2 new property tests; 1 ignored is the pre-existing
`process_shard_kill_is_detected_and_the_shard_rebuilds_while_a_sibling_shard_stays_healthy`, needs
a prebuilt wasm32-wasip2 fixture, unrelated to this fix.)

```
$ CARGO_TARGET_DIR=<TICKET_DIR>/🎯️target-sr cargo test -p semio-framework-actor
...
test result: ok. 69 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s (doc-tests)
REAL_EXIT:0
```
(untouched crate — matches baseline exactly, confirms no regression from a crate I did not edit.)

```
$ CARGO_TARGET_DIR=<TICKET_DIR>/🎯️target-sr cargo check -p semio-framework-actor --target wasm32-unknown-unknown
    Finished `dev` profile [unoptimized] target(s) in 0.51s
REAL_EXIT:0
```

```
$ CARGO_TARGET_DIR=<TICKET_DIR>/🎯️target-sr cargo check -p semio-framework-os-renderer-wgpu --lib
    Finished `dev` profile [unoptimized] target(s) in 33.70s
REAL_EXIT:0
```
(only pre-existing, unrelated warnings: the documented `Dock`/`Shell` module churn from another
session, and one repo-wide `unused_qualifications` warning in `🏪️store/🦀️component.rs` — neither
touches anything in my owned paths.)

Regression-catch proof (temporary revert, restored immediately after):
```
$ CARGO_TARGET_DIR=<TICKET_DIR>/🎯️target-sr cargo test -p semio-framework-plugin-host --lib -- shard::executor::tests --test-threads=1
running 4 tests
test component::shard::executor::tests::every_actors_grant_lands_on_the_shard_it_was_registered_on_across_k_shards ... FAILED
test component::shard::executor::tests::shard_executor_drives_a_turn_for_a_registered_actor_from_its_own_thread ... ok
test component::shard::executor::tests::stop_joins_the_thread_and_is_idempotent_with_drop ... ok
test component::shard::executor::tests::suspend_then_resume_round_trip_lands_on_a_shard_where_the_actor_is_registered ... FAILED
test result: FAILED. 2 passed; 2 failed; 0 ignored; 0 measured; 116 filtered out; finished in 0.06s
error: test failed, to rerun pass `-p semio-framework-plugin-host --lib`
```
(that run was piped through `tail`, so I am not reporting a specific numeric exit code for it —
only the pasted `cargo test` output, which is unambiguous: "test result: FAILED" plus `error: test
failed`. Fix immediately restored via `Edit`; the full-suite re-run above, captured without a pipe,
shows exit `0` and 115/0/1 again.)

Bench (run from repo root):
```
$ bun ./📜️script.ts bench plugins --renderer native --count 50 --extensions 50 --shards 4 --out <TICKET_DIR>/🔣️bench-shard-routing.json
...
scale-bench: wrote .../🔣️bench-native-raw.json
bench: wrote report -> .../🔣️bench-shard-routing.json
bench summary: 1:pass 2:pass 3:pass 4:pass 5:fail 6:pass 7:pass 8:pass

 NX   Successfully ran target bench for project @semio-tech/framework-os-dev
```
(this run was piped through `tail`, so — same caveat as above — I am not citing a specific numeric
exit code from that pipe. Independent confirmation instead: the report file
`<TICKET_DIR>/🔣️bench-shard-routing.json` exists on disk, 7793 bytes, and re-reading it with a
fresh `python3 -c "json.load(...)"` call after the fact reproduces the identical 8 budget rows —
`[(1,'pass'),(2,'pass'),(3,'pass'),(4,'pass'),(5,'fail'),(6,'pass'),(7,'pass'),(8,'pass')]` — and
Nx's own "Successfully ran target" line, which only prints on a genuine zero exit from the
underlying task, is present in the captured output above.)

## bench before/after table

Before = `TICKET_DIR/🔣️bench-w5-parallel.json` (the ticket's own archived pre-fix run, K=4).
After = `TICKET_DIR/🔣️bench-shard-routing.json` (this session, same K=4, `--count 50 --extensions
50`).

| budget | before | after |
|---|---|---|
| 2 (cold boot) | **fail** — `faultCount: 6`, e.g. `"ShardLoop::pump: actor 13018217672900608 is not registered on this shard"` (×5 unique + repeats) | **pass** — `faultCount: 0`, `activeActorsAfterBoot: 143` |
| 3 (shard balance) | **fail** — `faultCount: 2` despite `perShardCounts {0:25,1:25,2:25,3:25}` already balanced | **pass** — `faultCount: 0`, `perShardCounts {0:25,1:25,2:25,3:25}`, `maxShardLoad: 25` vs `shardCeiling: 26` (placement unchanged, as required) |
| 6 (hang isolation) | **pass, but for the wrong reason** — `faultMessage: "ShardLoop::pump: actor 0 is not registered on this shard"` (a routing-race false positive standing in for the real hang trap) | **pass, for the real reason** — `faultMessage` is now the genuine wasm trap backtrace from `profile::turn_hang`/`spin_once`; `killed: true`, `siblingsRestored: true` unchanged |
| 7 (suspend/resume) | **fail** — `resumed: false`, `identical: true` (coincidental hash match — `restore()` was never actually invoked) | **pass** — `resumed: true`, `identical: true` (checkpoint hash `395f1136eb...` matches on both sides, genuinely round-tripped through `restore()`) |
| 5 (interactive p95, reported as instructed, not fixed) | fail — p95 217.9ms / 8ms target, 30 samples spanning ~150-252ms | fail — p95 241.2ms / 8ms target, 30 samples spanning ~149-252ms — same order of magnitude, not touched, not tuned toward |

Budget 6's "before" row is worth calling out explicitly: it was reported `pass` in the archived
run even though the fault it captured was not the actor's real hang trap at all — it was this same
registration race, coincidentally landing on `actor 0` right as the harness's kill-detection logic
was watching for *any* fault. That is a latent false-positive the fix also closes, not a new
regression risk.

## lease-requests

None. The fix stayed entirely inside owned paths (`🖥️host/🧵️shard/🏃️executor.rs`); `🎠️runtime.rs`
and `🎭️actor/🦀️component.rs` needed no changes — `ShardTable::pin`'s assignment logic was already
correct and is left untouched.

## honest gaps

- `REGISTER_ACK_TIMEOUT` (5s) is a real, if generous, ceiling — under extreme host load
  (thousands of activations queued on one shard's `register_rx` well past `PARK_TIMEOUT`) a
  caller could theoretically wait close to it before falling back to the pre-existing
  fire-and-forget-on-timeout behavior. Not exercised by this bench's 2550-actor budget 4 run
  (which completed in the same envelope as before), but not proven at higher activation rates
  either.
- `register()` now costs up to one `PARK_TIMEOUT` (5ms) of real blocking latency per call in the
  worst case (executor already parked when the request lands). This only affects `activate()`,
  never the per-turn `tick_and_dispatch`/`wait_for_outcomes` hot path budget 5 measures — confirmed
  by budget 5's before/after numbers landing in the same range — but I have not separately measured
  `activate()`'s own latency before vs. after; budget 2 (cold boot, which activates 143 actors)
  shows `elapsedMs: 1413` after vs. `2717` before, i.e. *faster*, not slower, so there is no visible
  regression, but that is a side observation, not a targeted measurement.
- I did not run `cargo test -p semio-framework-os-renderer-wgpu` (only `--lib` `check`, per the
  acceptance list) — the two new property tests exercise the identical real `ShardExecutor`/
  `ShardTable` machinery `ParallelRuntime` drives, but `ParallelRuntime` itself has no dedicated
  unit test of its own in this packet; its correctness here is demonstrated by the bench run, not
  by a wgpu-crate unit test.
