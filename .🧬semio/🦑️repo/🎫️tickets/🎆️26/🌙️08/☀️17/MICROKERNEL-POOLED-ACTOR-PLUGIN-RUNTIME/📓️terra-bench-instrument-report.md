# 📓️ terra-bench-instrument report

**One-sentence summary: the old budget-5 instrument was timing the wall-clock duration of the
entire 41-actor round (all 40 cpu actors plus the interactive actor draining to completion via
`pump()`'s bulk `wait_for_outcomes(decision.run.len(), ..)`), not the interactive command's own
command→patch latency — so it was structurally incapable of ever reporting ≤8ms regardless of
scheduler quality, and I corrected the instrument (not the budget) to time only the interactive
actor's own `ShardOutcome`.**

## what the old interval measured

`start = Instant::now()` was taken, then 40 `Event::Wake`s were sent to the cpu actors, then the
interactive `Event::AppCommandEvent` was sent, then `env.pump()` was called, then `start.elapsed()`
was recorded — AFTER `pump()` returned. `pump()`'s own contract (its pre-existing doc, confirmed by
reading its body) is "`Kernel::tick`-drives every actor with a non-empty mailbox to completion" via
`self.runtime.wait_for_outcomes(decision.run.len(), ..)`, which blocks until ALL of that tick's
granted outcomes have arrived — the slowest of the 41 actors, not the interactive one specifically.
Every sample was therefore the wall time of the whole round: 40 cpu-profile actors (each declared to
busy-loop for some fixture-configured duration) plus the interactive actor, whichever finished last.
This is why the number sat at ~150–240ms across every prior runtime improvement in this ticket
(295ms, 241ms, 217.9ms archived) — the 40 actors alone cannot plausibly finish inside 8ms, so the
budget was unreachable by construction, independent of how good the K-shard scheduler is.

## what the new interval measures

`start = Instant::now()` is now taken immediately before the interactive command's own
`send_payload_lane` call (after the 40 `Wake`s have already been submitted — enqueued to their
mailboxes but not yet granted/dispatched). The round then calls the new `Env::pump_tracking(target)`,
which drives the SAME `Kernel::tick`-to-completion loop as `pump()` (every granted actor, cpu actors
included, still gets a real `tick_and_dispatch` → `wait_for_outcomes` → `Kernel::complete` round
trip, so kernel bookkeeping — fuel/throttle/mailbox state — stays exactly as consistent for the next
round as `pump()` left it) but waits on `wait_for_outcomes(1, ..)` one outcome at a time instead of
bulk-waiting for the whole tick, and stamps `Instant::now()` the moment the interactive actor's OWN
`ShardOutcome` (`Turn` or `Fault`) is among them. `samples_ms` now records `seen_at - start`, i.e.
send → this one actor's own response, not send → all-41-actors-drained. The 40 cpu actors are still
genuinely running/contending on their own real `ShardExecutor` threads for the whole interval (they
were granted in the SAME `Kernel::tick` as the interactive command — `grants_per_tick` comfortably
covers 41 single-turn grants) — that contention is exactly the load budget 5 specifies; it simply no
longer gates the stopwatch.

## lane change

`Env::send_payload` was refactored to delegate to a new sibling, `Env::send_payload_lane(actor,
payload, lane)`, with `send_payload` itself unchanged in behavior (still hardcodes `Lane::Background`
when calling the sibling). Every existing call site — `Env::send` (used by budgets 2/3/4/6), and
budget 7's direct `Payload::Suspend`/`Payload::Resume` sends — still routes through `send_payload`
and is therefore still `Lane::Background`, byte-for-byte as before. Budget 5's round loop is the only
caller that invokes `send_payload_lane` directly, passing `Lane::Interactive` for the interactive
command's own envelope only; its 40 `Wake` sends still use plain `env.send` (`Lane::Background`,
unchanged).

**Important limitation, stated plainly**: `Envelope.lane` (what I changed) controls per-actor mailbox
pop-priority and backpressure-eviction priority (`🎭️actor/🦀️component.rs`'s `Mailbox::enqueue`/
`pop_next`) — with one envelope per actor per round, that has negligible effect here. The
terra-interactive-isolation packet's placement mechanism (`ShardTable::pin_avoiding`, steering a
new `Lane::Interactive` ACTOR away from saturated shards) gates on the actor's own **activation**
lane, set once in `Env::activate` (`glue.rs`, unconditionally `Lane::Background` for every bench
actor, every budget). `Env::activate` is shared by budgets 2/3/4/5/6, so changing it was out of this
packet's scope ("do not touch any other budget's logic"). Net effect: this fix makes the interactive
probe's ENVELOPE honest about its own lane; it does NOT make the isolation/placement mechanism
reachable from this bench. That gate still cannot fire here — same fact the mission text itself
already named, now also recorded at the `send_payload_lane` doc comment so the next reader does not
have to rediscover it.

## line ranges edited

All inside `📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️glue.rs`, entirely within the
`scale_bench` module (`//#region 🔖️ScaleBench`, lines 585–1310), per owned-path scope:

| What | Approx. lines (post-edit) |
|---|---|
| `Env::send_payload` — body replaced to delegate | 780–782 |
| `Env::send_payload_lane` — **new** sibling, takes `lane` | 784–804 |
| `Env::pump`'s doc comment — removed the now-false "this is the instrument budget 5 measures" claim, added a correction note pointing at `pump_tracking` | 806–815 |
| `Env::pump_tracking` — **new** method | 857–911 |
| `budget_4_and_5`'s round loop (budget 5 only) — 40 `Wake`s moved before `start`, interactive send switched to `send_payload_lane(.., Lane::Interactive)`, `env.pump()`/`start.elapsed()` replaced with `env.pump_tracking(interactive_actor)` + stamp arithmetic, comments added at the measurement site | ~1067–1108 |
| Budget 5's `row()` note string — rewritten to describe the new interval + lane, flag non-comparability | ~1118 |

No other budget's function body was touched. No line in the winit host (`kernel_runtime`) or in
`🎠️runtime.rs` (`ParallelRuntime`) was touched.

## why the old and new numbers are incomparable

The old `samplesMs` values are round wall-clock time across 41 actors (bounded below by the sum/max
of 40 cpu-actor busy-loop durations, however those are distributed across 4 shards). The new
`samplesMs` values are one actor's own send→response latency. They do not share units in any
meaningful sense — a new number lower than 217.9ms is not "faster," it is answering a different
question. **All previously-archived budget-5 numbers (295ms, 241ms, 217.9ms) measured round
wall-time under load; any new number from this instrument measures interactive command→patch
latency. They must not be plotted on the same axis or presented as a trend.**

## expected sample behaviour (prediction, pre-measurement)

I expect **real, non-trivial spread** across the 30 samples, not a tight cluster — for a mechanical
reason, not a hope: the interactive actor and the 40 cpu actors are activated once, before the round
loop starts, so `ShardTable::pin`'s placement (and therefore which ~9–10 of the 40 cpu actors share
the interactive actor's own shard) is FIXED for all 30 rounds. Each `ShardExecutor` still drives its
one `ShardLoop` on a single dedicated OS thread — real parallelism is ACROSS shards, not within one
— so however many cpu-actor turns land on the interactive actor's own shard in a given tick are
processed sequentially by that one thread, and the interactive actor's queue position among its
shard-mates (itself subject to real OS thread-scheduling jitter across 4 shard threads + forwarder
threads + the main pump thread) determines most of the latency. I expect:

- The new p95 to be far below the old ~150–240ms band (it is no longer waiting for the slowest of
  41 actors), but I am NOT predicting it clears 8ms — if the interactive actor's shard happens to
  carry several cpu-busy-loop turns ahead of it in a tick, its own response could still be a multiple
  of a single cpu turn's duration. That would be an honest failure, not evidence of a broken
  instrument.
- Samples should vary round-to-round by a non-trivial margin (I'd expect at least low-single-digit-ms
  spread, plausibly more) due to genuine OS thread-scheduling jitter across the shard/forwarder
  threads, even though the shard assignment itself is static across rounds.
- If the 30 samples instead come back clustered inside a sub-millisecond band again, that is NOT
  confirmation of a fast scheduler — per this ticket's own prior finding (30 samples inside a 0.1ms
  band, from the single-physical-`ShardLoop` defect), a tight cluster is itself a signal that
  something is still shortcutting real cross-thread dispatch, and would need re-investigation before
  being reported as a pass.

## commands

Both **UNRUN** — I did not execute either, per binding rule 3 (coordinator owns all builds):

```
cargo check -p semio-framework-os-renderer-wgpu --lib
bun ./📜️script.ts bench plugins --renderer native --count 50 --extensions 50 --shards 4 --out <TICKET_DIR>/🔣️bench-instrument.json
```

## honest gaps

1. **Not compiled or run.** Per binding rule 3, I have zero build/runtime confirmation this compiles
   or that the bench produces the expected shape of output. `pump_tracking`'s structure closely
   mirrors `pump`'s pre-existing, presumably-compiling body (same field names, same match arms, same
   `ActorId`/`TurnResult`/`TurnStatus` construction), and I read every type/signature it touches
   (`ShardOutcome::{Turn,Fault}`, `ParallelRuntime::{tick_and_dispatch,wait_for_outcomes,complete}`,
   `Envelope`, `Lane`) directly from source rather than assuming them, but that is not a substitute
   for a real `cargo check`. As a cheap (non-build, non-acceptance) sanity check I did run `rustfmt
   --edition 2021 --check` directly on `glue.rs` — it parsed the entire file (including every
   `#[path]`-included module) without error and reported zero formatting diffs anywhere inside
   `send_payload_lane`, `pump_tracking`, or the rewritten budget-5 round loop (the diffs it did
   report are all pre-existing, at unrelated lines I did not touch). This rules out a broken-syntax
   edit; it does not rule out a type or borrow-check error, which only `cargo check` can catch.
2. **The isolation mechanism still cannot fire for this bench.** As stated above under "lane
   change": fixing the envelope's lane does not fix the actor's activation lane, which is what the
   terra-interactive-isolation packet's placement gate actually reads. Closing that gap would mean
   changing `Env::activate`'s hardcoded `Lane::Background`, which is shared by every other budget and
   therefore out of this packet's scope by the mission's own instruction.
3. **`pump_tracking`'s per-outcome polling (`wait_for_outcomes(1, ..)`) is finer-grained than
   `pump`'s bulk wait, but each call still has its own `recv_timeout` overhead.** For a single round
   of ~41 outcomes this is negligible relative to millisecond-scale measurement, but it is a real,
   if small, structural difference from `pump`'s batching that a future reader should know about if
   they ever compare the two methods' own overhead rather than the actor-level timings they enable.
4. **I did not attempt to reduce shard count or otherwise change the fixture/topology** to try to
   make the interactive actor's own queue position more favorable — mission rule 3 forbade touching
   the 40-actor count, round count, or any other budget's logic, and shard_count is a caller-supplied
   CLI parameter (`--shards 4` in the acceptance command), not something `scale_bench` owns.
