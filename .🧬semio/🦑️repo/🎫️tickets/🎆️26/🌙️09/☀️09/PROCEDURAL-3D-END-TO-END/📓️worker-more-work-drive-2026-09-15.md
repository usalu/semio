# 🚚 The worker owns the MoreWork drive — the host stops polling a guest that answers with nothing (lane `worker-more-work-drive`, 2026-09-15)

Opus lane on port **6021** (`📜️serve-generation3d-react-6021.sh`, host TS vite-live, `SEMIO_VITE_HMR=0`)
and **6118** for the wgpu twin. Continues `📓️ui-turn-patch-batching-2026-09-15.md` §7, which named this
term and could not take it:

> **31.3 empty crossings per hop are still the dominant term** and none of them is a publication:
> they are the `settlePluginTurn` continuation loop polling a guest that answers `MoreWork` with
> nothing.

and `📓️reactor-reconcile-spin-2026-09-14.md` §7 items 2–3, which named why the hold that was supposed
to absorb them could not, and why the two heartbeats per crossing stayed:

> **The hold is 8 ms and a guest turn now costs 12.4 ms.** […] **The `heartbeat()` /
> `heartbeat("turn-step")` pair** the worker posts per crossing is still two extra main-thread
> messages per crossing […] no lane owns it.

This lane owns all three.

---

## 0. The headline

> **A guest that answers `MoreWork` with nothing is not sending the host a message — it is asking to
> be pumped, and in the browser the host round trip IS the pump. The worker now owns that pump: it
> runs the next turn itself, bounded by a ceiling DERIVED from what a turn measures, and crosses only
> when the turn produced something, when a host-owned input is pending, when the actor closes, or when
> the budget is spent.**

| | before (`…/before2/`, 10:08) | after (`…/after4/`, 13:05) |
|---|---:|---:|
| **worker crossings per `flowEvalTick` hop** | **60.70** | **36.20** (**−40 %**) |
| of those, `(none)` crossings | 658 (**32.9**/hop) | 169 (**8.4**/hop) |
| ui patches carried | 255 | 254 |
| guest turns the WORKER absorbed | 0 (the host polled every one) | **1 395** over 724 crossings (1.93/crossing, 69.8/hop) |
| crossings that ended because the drive ran out of COUNT | — | **0** (`steps` never fired; §2.7) |
| main-thread messages per turn crossing | 3 (`heartbeat`, `heartbeat`, `result`) | **1** (`result`, carrying the beat) |
| run | 53.0 s, 20 hops, 1 214 crossings, 10/10 converged | 51.8 s, 20 hops, **724** crossings, 10/10 converged |
| hop wall | 2 652 ms (load 14.8) | 2 592 ms (load 17.9) |
| journey | — | **23/23 converged, 16/16 mesh oracles green, 0 red** |

Read the hop wall with its load column, not against it (§5.4). The load-independent number is the
crossing count, and it moved **60.70 → 36.20** for the same 254 published patches.

---

## 1. What was actually being paid for

`settlePluginTurn` (`🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx`) drives a reactor-owned continuation
until its requested patch set is published or the actor quiesces. Every lap of that loop is a worker
POST, a `poll`, a structured clone and a main-thread pickup. 32.9 of the 60.7 laps per hop posted no
events at all, and the guest's only answer was `MoreWork` with an empty `turn-result`.

The previous lane put a **silent-turn hold** in the generated shard worker for exactly this
(`📓️reactor-reconcile-spin-2026-09-14.md` §3.5) and it barely fired. Its wall was
`SHARD_TURN_SILENT_HOLD_MS = 8` — borrowed from the reactor's own executor slice,
`run_until_deadline(64, 256 KiB, now + 8 ms)` — while ONE WHOLE guest turn measures **13 ms** on this
door. The deadline was taken after the first poll returned, so the loop admitted exactly one further
turn and then found its wall already spent. That is the incoherence: **the reactor's slice of a turn
is not a budget for whole turns.**

---

## 2. The design

### 2.1 The drive's bounds are derived from a measurement, not borrowed from the reactor

Two declarations, one on each side of the boundary, held equal by one language-agnostic contract
(`⚛️reactor/🧫️fixtures/🚚️more-work-drive.json`):

```rust
pub const REACTOR_TURN_EXECUTOR_HOLD_MS: u64 = 8;    // the reactor's OWN slice — unchanged, and no longer a drive budget
pub const MEASURED_GUEST_TURN_COST_MS:   u64 = 13;   // what a WHOLE turn costs, measured on this door
pub const fn more_work_drive_steps(grant_wall_ms, guest_turn_cost_ms) -> usize;      // = max(1, grant / cost)
pub const fn more_work_drive_budget_ms(grant_wall_ms, guest_turn_cost_ms) -> u64;    // = min(grant, steps × cost)
```

* **`MEASURED_GUEST_TURN_COST_MS = 13`** is a measurement carried as a declaration: the `worker.guest`
  mean of `🐍️react-hop-cost-probe.mjs` over a 10-step run (12.9 ms over 1 154 crossings and 13.2 ms
  over 1 214, both 2026-09-15; 12.8 ms over 1 165 the day before). It is the number to re-measure when
  the guest's turn cost moves, and the coherence law (§4.1) is what forces that re-reading.
* **The grant is the authority for the wall.** `Budget::deadline_ms` is the wall the host DECLARED when
  it posted this turn (`DEFAULT_SHARD_BUDGET.wallMs = 100` on this door), so a drive that spends it is
  spending what it was given and the watchdog ladder (`SHARD_LIVENESS_POLICY`) is already written
  against it. At 100 ms granted and 13 ms measured the ceiling is **7 turns** and the budget **91 ms**.
* **At least one step is always admitted**, exactly as the turn-patch page always admits its first
  patch: a grant smaller than one turn still has to make progress.
* `SHARD_TURN_DRIVE_STEP_CEILING = 512` is a hard BACKSTOP against an unbounded loop — a guest whose
  turns cost microseconds — not a latency control and not the derivation. See §2.7 for why the derived
  count is an expectation rather than the runtime cap.

The reactor's 8 ms hold is untouched. What changed is that nothing else reads it as a budget: the
literal `8` at its only call site is now `REACTOR_TURN_EXECUTOR_HOLD_MS`, and the coherence law asserts
`REACTOR_TURN_EXECUTOR_HOLD_MS / MEASURED_GUEST_TURN_COST_MS == 0` — i.e. that a wall-only hold of the
reactor's slice admits **no further turn at all**.

### 2.2 The drive, and its six stops (checked in this order)

`🎭️actor/🖼️wire-turn/🟦️.ts`'s `driveShardTurnMoreWorkV1` owns the law; the generated worker
(`🔌️plugin/🌐️browser-bundle/🏗️materialization/🟦️.ts`) carries its verbatim twin on the `turn` path.

| stop | means |
|---|---|
| `carried` | the turn produced something for the host — **the only reason the host is told anything** |
| `idle` | the guest stopped answering `more-work` |
| `input` | a host-owned input is pending behind the drive — an ingress message, a cancel, a view-state change |
| `budget` | the WALL the host granted this crossing is spent (§2.7) |
| `steps` | the hard backstop count is spent — a guest whose turns cost microseconds (§2.7) |
| `closed` | the actor was disposed or re-activated under a new generation |

A result is only discarded when `shardTurnCarriesNothingV1` admits it, and that predicate names EVERY
field of WIT `turn-result` a host reads (`uiPatches`, `effects`, `presence`, `nextWake`,
`lifecycleReceipt`, `uiPatchReceipt`, and both ingress lanes, which must be `idle`). `fuelUsed` is the
one field with no host reader and is the stated omission. The drive is therefore lossless by
construction, and the exhaustiveness is asserted rather than trusted.

### 2.3 How a host-owned input can interrupt a drive at all

`input` is not free to detect. A worker's `await` settles on the MICROTASK queue, which never drains
the message queue, so a drive that only `await`s the guest is **uninterruptible by construction** —
`hostInputSeq` could not move even if the host had already posted.

So two things:

* the worker's `message` listener bumps `hostInputSeq` as its first act, for **every** kind without
  exception — a kind this worker has never heard of is still the host speaking;
* the drive takes ONE macrotask between steps (`driveYield`), so a message the host already posted is
  delivered before the next turn runs. It is a `MessageChannel` task, not `setTimeout(0)`, for the
  reason the host's own `hostContinuations` gives: a timer chain is throttled to one tick per second in
  a hidden tab (once per minute under intensive throttling) and a channel message is a macrotask
  visibility never throttles. Waiters are a queue, because two actors may be driving one worker at once.

The drive re-reads `hostInputSeq`, the actor registry and the activation generation both before and
after that yield, so a dispose that lands inside it ends the drive on the spot.

**Measured on the live door: 27 of 810 crossings ended on `input`** — the interrupt is not theoretical.

### 2.4 One message carries liveness

`heartbeat(phase)` became two functions:

* `beat(phase)` advances `turnSeq`, mirrors it into the `Atomics` slot, and **returns** the beat;
* `postBeat(phase)` posts one, for a beat with no crossing to ride.

Every `reply` now carries `beat` (`beat("turn-step")` at the step boundary of `turn`/`stepJob`, a plain
beat everywhere else), `replyError` carries `beat("fault")`, and `ShardClient.handleMessage` folds a
carried beat into exactly the `recordHeartbeat` a dedicated `heartbeat` message would have reached.

**Why that is lossless, stated rather than assumed.** `ShardClient.noteLiveness` already treats EVERY
inbound message as proof of life, and `evaluateShardLiveness` already counts the request's own start
instant (`max(lastLivenessAtMs, oldestPendingStartedAtMs)`) as proven-alive — so the start-of-request
`heartbeat` contributed nothing the post itself did not. The `turn-step` beat is what proves a guest
running a BUDGETED job is alive, and it is emitted at the same instant as the reply it now rides. What
is NOT foldable is a beat with no crossing: `loadActor`'s three await boundaries
(`module-fetch`/`module-ready`/`actor-ready`) and the while-busy `progress` ticker still post, and the
shard liveness contract (`SHARD_LIVENESS_POLICY`, `🧫️fixtures/🔣️.json`) is unchanged.

**Two main-thread messages per turn crossing removed**, on a main thread the hop measurement shows is
the binding constraint (`worker.reply` is 257 ms/hop of pickup latency).

### 2.5 Readiness is per allocation, not global

`next_ready_index` (`⚛️reactor/🩹️patches/🦀️.rs`) refused to publish a ready output while the MINIMUM
generation over EVERY slot still holding a producer or a job was lower than it — a GLOBAL serialization,
so one slow surface blocked every other surface's finished output. That is the term
`📓️ui-turn-patch-batching-2026-09-15.md` §7.1 names: 18 of 25 measured boot turns had exactly ONE
surface ready while the wire could already carry sixteen.

The gate is now `ready_output_is_admissible(ready_key, ready_generation, in_flight)` — a named function
so it can be stated as a law rather than inferred from a ladder — and it refuses only while the SAME
allocation holds lower-generation work in flight. The argument: a retained patch is addressed by its own
`(surface, revision)` and applied into that surface's own retained document; two allocations share no
revision line, no document and no reader, so surface B's in-flight job says nothing about when surface
A's finished output may publish. §4.1's law walks all 64 generation pairs on both sides of that
distinction, and the ladder law proves each surface's own publication line stays strictly increasing.

### 2.7 The drive spends the WALL it was granted, not the wall rounded down to whole turns

The first shape of this drive bounded itself by `steps × cost` — 7 × 13 = 91 ms of a 100 ms grant — and
by a step COUNT of 7. Both were wrong for the same reason, and the measurement says so directly: in the
11:48 run **106 of 824 crossings ended on `steps`**, i.e. crossed back carrying NOTHING because a count
ran out, not because the guest or the host had anything to say. A crossing that carries nothing is the
exact cost this whole lane exists to remove.

A peer measured the consequence on a real workload (ticket `26/09/13/INTERACTIVE-TOOLS-VISIBLE-PROCESS`
`📓️status.md` phase 8, `🔍️w5-fill-timeline-probe.ts --profile`): during a live tool run the React shell
requests window refreshes every 100–400 ms, the world window's patches cost ~30 ms of host intake each,
and they arrive **250–800 ms after the request and not once per request** — the reconcile finishes on
some later, unrelated drain turn rather than inside the drive that was asked for it.

So the bounds were re-ordered around what each one is FOR:

| bound | what it is for | value on this door |
|---|---|---|
| `input` | **the latency control** — the wire is released the instant the host has something to say | checked first, every step |
| `budget` | **the drive's real bound** — the whole wall the grant declared | 100 ms |
| `steps` | a hard backstop against an unbounded loop | 512 |
| `more_work_drive_steps` | the grant's derived EXPECTATION, and the subject of the coherence law | 7 |

The step count keeps its job — it is what says a 100 ms grant fits seven 13 ms turns while the reactor's
own 8 ms slice fits none — and stops being the runtime cap. Measured after the re-order:
**`steps` fired 0 times, `budget` 8 times**, the drive absorbed **1.93 guest turns per crossing** instead
of 1.58, and the hop fell **41.20 → 36.20** crossings (§5.1).

### 2.6 The trap a cut page was setting — reported by a peer, owned here

A peer lane (INTERACTIVE-TOOLS-VISIBLE-PROCESS, puzzle 3d on :6013) reported that since the
ui-turn-patch-batching change a fresh puzzle plugin trapped on its FIRST turns
(`setActiveExample`, `windowResize`) with `plugin.reactor-close-authority: pending patch emission left
a borrowed publication unstaged`, and nothing mounted. Their suspected mechanism was a PAGED
`publish_into`; the real one is one layer up, and this lane owns the files, so it is fixed here.

**What actually happened.** `take_turn_patch_page` admits the first patch whatever it costs and then
stops at the per-turn BYTE budget, returning the patch that did not fit through `hand_back_turn` —
which puts it back INTO its borrowed cell. `stage_emission` then compared the page's entries against
EVERY borrowed cell, so a cut page always had one more borrowed cell than entries and the whole turn
faulted. A surface small enough that two of its patches fit one budget never cuts a page; puzzle 3d's
world-3d surface cuts every one. That is exactly the split between "our lanes stay green" and "their
plugin never mounts".

**The fix, in three parts.**

1. **A cell records the identity of the patch that left it** (`published: Option<(SurfaceId, u64)>`),
   and `stage_emission`/`hand_back_turn` match a page entry to its cell by that identity. The old rule
   was POSITIONAL — "cells are borrowed in ascending index and a refused page is drained in publication
   order" — which holds only while no cell survives a turn. A cut page makes a cell survive, and from
   the next turn on index order and publication order are two different orders, so the positional rule
   would have returned a patch to a stranger's slot.
2. **What a receipt may be staged against is what this turn DELIVERED** — a borrowed cell whose patch
   has left it. A cell still holding a patch keeps its sequence and its slot, arms the next turn through
   `has_undelivered()`, and travels then, in order, exactly once. `commit_emission` releases only the
   delivered cells.
3. **The peer's suspected mechanism is refused BY NAME rather than left possible.** `publish_into` and
   the external move are both atomic — the whole source or nothing — so a cell that reports bytes and
   hands out no patch is a broken publication contract. `take_one` now answers
   `"pending publication reported bytes and handed out no patch"` instead of leaving a borrowed cell
   nobody can stage a receipt against.

§4.1's `a_page_cut_by_its_byte_budget_stages_exactly_what_it_delivered_and_loses_nothing` reproduces the
trap against the production ladder (`take_turn_patch_page` with the budget dialled to a measured patch
unit): **4 pages, 3 of them cut**, 4 surfaces delivered exactly once each, nothing duplicated, nothing
unstaged.

---

## 3. Files changed

| file | change |
|---|---|
| `🧰️framework/…/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs` | `REACTOR_TURN_EXECUTOR_HOLD_MS` (the literal `8` at its only call site, now named), `MEASURED_GUEST_TURN_COST_MS`, `more_work_drive_steps`, `more_work_drive_budget_ms` |
| `🧰️framework/…/🔌️plugin/⚛️reactor/🩹️patches/🦀️.rs` | `ready_output_is_admissible`; `next_ready_index` gates per allocation instead of globally; `in_flight_generations` (test accessor) |
| `🧰️framework/…/🔌️plugin/⚛️reactor/📨️pending/🦀️.rs` | §2.6: `TurnPatchCell.published` (the cell's wire identity); identity-matched `hand_back_turn`/`stage_emission`; `borrowed_sequences` counts only DELIVERED cells; retained cells served oldest-sequence-first and kept across turns; `commit_emission` releases only delivered cells; `take_one` refuses a non-atomic publication by name |
| `🧰️framework/…/🔌️plugin/⚛️reactor/📨️pending/🧪️tests/🩹️receipt/🦀️.rs` | the handback law now asserts the corrected invariant: a handed-back patch is stage-able by nobody until a later turn takes it out, and that turn re-borrows the SAME slot |
| `🧰️framework/…/🔌️plugin/⚛️reactor/🦀️.rs` | registers the new law module |
| `🧰️framework/…/🔌️plugin/⚛️reactor/🧫️fixtures/🚚️more-work-drive.json` | **new** — the language-agnostic drive contract both twins read |
| `🧰️framework/…/🔌️plugin/⚛️reactor/🧪️tests/🚚️more-work-drive/🦀️.rs` | **new law** (7) |
| `🧰️framework/🔨️modules/🎭️actor/🖼️wire-turn/🟦️.ts` | `driveShardTurnMoreWorkV1`, `shardTurnDriveStepsV1`, `shardTurnDriveBudgetMsV1` replace `driveShardTurnSilentHoldV1` |
| `🧰️framework/…/🔌️plugin/🌐️browser-bundle/🏗️materialization/🟦️.ts` | `SHARD_TURN_GUEST_COST_MS`/`SHARD_TURN_DRIVE_STEP_CEILING` replace `SHARD_TURN_SILENT_HOLD_MS`/`_POLLS`; the generated worker's drive loop, `hostInputSeq`, `driveYield`, `shardTurnDriveSteps`/`…BudgetMs`, `beat`/`postBeat`, and `reply`/`replyError` carrying the beat |
| `🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts` | `ShardCarriedBeat`; `handleMessage` folds a carried beat into `recordHeartbeat`; `drivePolls`/`driveSteps`/`driveStopped` on the turn timings and on the published span detail |
| `🧰️framework/…/🧑‍🎨engine/🧪️tests/🚚️more-work-drive/🟦️.ts` | **renamed** from `🤫️silent-turn-hold` and rewritten — **28 laws** |
| `🧰️framework/…/🧑‍🎨engine/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts` | registers the renamed suite |
| `🧰️framework/…/🎭️actor/📮️shard-client/🧪️tests/🛑️cancel-job-reply/🟦️.ts` | the beat it asserts now rides the reply |
| `🧰️framework/…/🎭️actor/📮️shard-client/🧪️tests/🧪️shardclient-reserved-response-settlement/🟦️.ts` | activation beats lose the start-of-request post; the generated source's SECOND dynamic import and the `self.location`-reading diagnostics switch are redirected so the law can run at all (§4.4) |
| `🧰️framework/…/🎭️actor/📤️return/📨️response/🧪️tests/🧪️actorreturnresponseframing-…/🟦️.ts` + `…/📥️inbox/🧫️fixtures/🔣️.json` + `…/📥️inbox/🧬️schema/🔣️.json` | the worker-inbox trace vectors lose their two per-crossing heartbeats |
| `<ticket>/🐍️react-hop-cost-probe.mjs` | reads `drivePolls`/`driveStopped` off the span detail and prints the drive's absorption and stop histogram — so §5 is a reading, not an inference from `worker.guest`'s mean |

---

## 4. Laws, with output

### 4.1 Rust — `cargo test -p semio-framework-plugin --lib more_work_drive -- --test-threads=1 --nocapture` (foreground)

```
running 9 tests
[DEBUG] more-work-drive cancellation steps=7
[DEBUG] more-work-drive silent=1 crossings=1 turns=2 steps=7
[DEBUG] more-work-drive silent=4 crossings=1 turns=5 steps=7
[DEBUG] more-work-drive silent=6 crossings=1 turns=7 steps=7
[DEBUG] more-work-drive cut-page unit=40580B pages=4 cuts=3 delivered=[("74:surface-0", 1), ("74:surface-1", 1), ("74:surface-2", 1), ("74:surface-3", 1)]
[DEBUG] more-work-drive ingress interrupt within=1 steps=7
[DEBUG] more-work-drive readiness gate: per-allocation, 64 generation pairs checked on both sides
[DEBUG] more-work-drive continuation ceiling=1024 steps=7 guest-turn reach=7168
[DEBUG] more-work-drive ladder host-polled crossings=560 turns=560 idle=550 | worker-driven crossings=87 turns=560 idle=77
[DEBUG] more-work-drive readiness ladder turns=560 published=8 overlapped=0 surfaces=8
[DEBUG] more-work-drive coherence hold=8ms cost=13ms grant=100ms steps=7 budget=91ms
test result: ok. 9 passed; 0 failed; 749 filtered out
```

| law | asserts |
|---|---|
| `a_page_cut_by_its_byte_budget_stages_exactly_what_it_delivered_and_loses_nothing` | **§2.6's law.** The production `take_turn_patch_page` with the budget dialled to a measured patch unit: 4 pages, **3 of them genuinely cut**, 4 surfaces delivered exactly once each on their own `(surface, revision)`, nothing duplicated and nothing left unstaged |
| `a_guest_answering_more_work_k_times_with_nothing_costs_one_host_crossing` | **THE law.** k ∈ {1, 4, 6} silent turns plus the one that carries cost **1** crossing, and the drive runs every one of those k + 1 turns |
| `a_pending_host_input_interrupts_the_drive_within_one_turn` | for an input arriving at each of the 7 step positions, the drive ends on `Input` within the declared 1-turn interrupt |
| `a_cancellation_mid_drive_ends_the_drive_on_the_turn_it_lands` | a close landing after k turns stops the drive at exactly k, for every k inside the ceiling |
| `the_hosts_continuation_ceiling_covers_more_guest_turns_under_the_drive_never_fewer` | **the ceiling law the coordinator asked for.** For every k from 1 to 64, a guest needing k turns costs `ceil(k / steps)` host continuations — never more than k, and `steps` times fewer at the ceiling, so `PLUGIN_UI_CONTINUATION_LIMIT` and the command-ingress drain's 1 024 both cover **7 168** guest turns instead of 1 024 |
| `the_worker_drive_budget_is_derived_from_the_measured_turn_cost_not_from_the_reactors_hold` | **the coherence law.** The fixture and the reactor name the same slice and the same measured cost; `cost > hold`; `hold / cost == 0` (a wall-only hold of the reactor's slice admits no further turn); the step ceiling and the wall budget ARE the derivation; a drive never spends more than the grant; a sub-turn grant and an unmeasured cost both still admit one turn |
| `a_ready_output_is_not_blocked_by_another_allocations_running_job` | the widened gate, exhaustively: the same allocation's older work holds its own output back, its equal and newer generations do not, and **no** foreign allocation ever gates it — 64 generation pairs on both sides |
| `the_widened_readiness_gate_keeps_every_surfaces_own_publication_order` | the production ladder: 8 surfaces driven one reconcile opportunity per turn publish exactly once each, on 8 distinct publication lines, in strictly increasing revision order per surface |
| `the_production_reconcile_ladder_costs_fewer_crossings_when_the_worker_owns_the_pump` | the same production ladder (`drive_reconcile_within`, `take_one`, `stage_emission`, `commit_emission`, `apply_issued_ack`, `reconcile_arms_turn`) costs **560** crossings host-polled and **87** worker-driven for the identical 560 guest turns, with the crossings that carry nothing falling 550 → 77 |

### 4.2 TypeScript twin — `bun ./📜️script.ts test long "more-work-drive"` (foreground)

```
 Test Files  1 passed (1)
      Tests  29 passed (29)
```

Nine carrier refusals (one per `turn-result` field) and the malformed/absent cases; the six stops of the
drive, including that a pending host input wins over BOTH the backstop and the wall budget, that the
drive spends its WHOLE granted wall rather than the wall rounded down to whole turns (§2.7), that the
host's continuation ceiling covers `ceil(k / steps)` crossings for a guest needing k turns and never more
than k, and that a turn which already carried is never polled; the derivation held equal to the reactor's
contract fixture, with the coherence clause restated on this side; and four source-level twin assertions
against the generated worker — its interpolated constants, its stop conditions in the right ORDER
(`budget` before `steps`), its `hostInputSeq`/`driveYield` pair (with `setTimeout` explicitly refused),
and that both per-crossing heartbeats now ride the reply while exactly the four unridable beats still
post.

### 4.3 The corpora

| gate | result |
|---|---|
| `cargo test -p semio-framework-plugin --lib reactor:: -- --test-threads=1` | **175 passed, 5 failed** — the SAME five the previous two lanes recorded, each in a file this lane did not touch: `async_actor_poll_awaits_exchange_and_render_work` (a source-TEXT scan for a line a peer boxed), `patches::tests::mounted_document_tree_publishes_nested_interactive_rows` (`SURFACE_RECONCILE_AGGREGATE_BYTES` 52 559 872 vs a fixture's 33 554 432), `executor::tests::cancel_of_a_parked_task_…`, `m1_m2_reactor_tests::revision_guard_…` and `turn_execution_tests::guest_turn_execution_resets_for_every_turn` (both order-flaky under the shared thread-local reactor state `--test-threads=1` forces). This lane adds 9 passes and no reds; the one red it caused on the way (`pending::issued_receipt_tests::reactor_uncommitted_patch_handback_preserves_exact_slot_and_retry`, which asserted the pre-§2.6 invariant that a handed-back cell is still stage-able) is fixed in the same change. |
| React engine `bun ./📜️script.ts test long` | **1 254 passed, 4 failed** (48 files) — the same four pre-existing peer-owned reds the previous two lanes recorded (`🎟️resident-refresh-budget`'s census and three `🔬️engine-contract` items). None in a file this lane touched. |
| `🎭️actor` TS `bun ./📜️script.ts test` | **254 passed, 14 failed.** Eight are ajv `can't resolve reference …#/$defs/NonZeroU64` schema-resolution failures; four are source-inventory drifts against peer additions (`ShardClient.actorsPastFirstTurn`/`firstTurnTimeoutMs`, `PendingEntry.kind`/`firstTurn`); one is the turn-patch-batch lane's own (`captureUiPatchAuthority` on two patches no longer throws `actor-ui-patch.pairing`, which is that lane's intended new behaviour with a fixture still asserting the old one). All pre-existing and peer-owned. The three reds this lane DID cause are fixed in the same commit (§3's last three test rows). |
| `cargo check -p semio-framework-os-renderer-wgpu --lib` | green (warnings only, 1 m 10 s) |
| `cargo test -p semio-framework-plugin --lib turn_patch_batch -- --test-threads=1` | **5 passed, 0 failed** — the batching lane's own laws, unchanged by §2.6: `surfaces=1/3/6/8 crossings=2 idle=0`, the ordered handback of a refused page, the split under a two-patch budget, the always-admitted first patch and the contiguous-ceiling capacity |
| `@semio-tech/framework-renderer-wgpu:generate-frame-worker` | green (16.5 s then 19.1 s, 4 tasks); the regenerated `🎞️frame-worker/🤖️generated/🟨️.js` carries `drivePolls` and the `message.beat` fold |
| `@semio-tech/framework-renderer-wgpu:wasm` | green (2 m 48 s, 8 tasks) |

### 4.4 One pre-existing red repaired, and why

`ShardClient liveness watchdog > beats at every generated-worker activation boundary` could not run at
all: it asserts the rewritten worker source contains no `@vite-ignore`, and the worker has carried a
SECOND dynamic import since `armGuestRuntimeDiagnostics` landed (2026-09-12) — plus that function reads
`self.location.href`, which a bare `vm` context does not have, so the activation it drives would have
thrown even once the assertion passed. Both are redirected in the test's own source rewrite (the shim
import to a rejected promise, the switch to `false`), because this is precisely the law that had to
observe §2.4: it now reads the activation beats off the REAL generated worker and confirms the
start-of-request post is gone while `module-fetch`/`progress`/`progress`/`module-ready`/`actor-ready`
still post.

---

## 5. Before / after, measured on 6021

Restage (foreground, no cache):
`CARGO_PROFILE_WASM_DEV_DEBUG=false NX_DAEMON=false NX_SKIP_NX_CACHE=true bun nx run
@semio-tech/framework-os-dev:activate-generation3d-react-dev` → `🗑️generated/more-work/restage4.txt`,
**exit 0**, 9 m 12 s, 33 tasks, "11 completed components (changed)" (the fourth of four: the shard
worker is materialized at restage, so §2.7's and §2.6's edits each needed one). Both runs are on a serve that was
recycled by pid immediately beforehand (coordinator's standing note on stale vite transforms), and the
served worker was verified to carry the change rather than assumed
(`feedback-vite-stale-transform-verify-served-module`):

```
$ curl -s http://127.0.0.1:6021/🔌️plugin-modules/🧵️shard/🟨️shard-worker.js | grep -a "GUEST_TURN_COST_MS\|hostInputSeq += 1\|driveYield()\|const driveSteps"
const GUEST_TURN_COST_MS = 13;
  hostInputSeq += 1;
          const driveSteps = DRIVE_STEP_CEILING;
            await driveYield();
$ … | grep -ac 'heartbeat("turn-step")'
0
```

### 5.1 Crossings

`SEMIO_PROBE_URL=http://127.0.0.1:6021/?plugin=generation3d SEMIO_PROBE_OUT=more-work/{before2,after4} bun 🐍️react-hop-cost-probe.mjs`

| worker crossing (posted event kinds) | before /hop | after /hop | before patches | after patches |
|---|---:|---:|---:|---:|
| **(none)** | **32.9** (658) | **8.4** (169) | 238 | 41 |
| `message` | 10.0 (200) | 10.0 (200) | 0 | 0 |
| `patch-ack` | 9.9 (198) | 9.8 (197) | 17 | 160 |
| `surface-visible` | 4.0 (79) | 4.0 (79) | 0 | 53 |
| `request` | 1.9 (38) | 1.9 (38) | 0 | 0 |
| `completed` | 1.9 (38) | 1.9 (38) | 0 | 0 |
| lifecycle (`instance-open`, `job-completed`, `instance-lifecycle-ack`) | 0.3 (3) | 0.3 (3) | 0 | 0 |
| **total** | **60.70** (1 214) | **36.20** (724) | **255** | **254** |

**The `(none)` row is the whole attributable effect, and it is the row this lane is about**: 658 → 169
crossings, 32.9 → 8.4 per hop, for the same published work — a **74 % cut** in the crossings that were
the host asking a guest that had nothing to say. Every other kind is a host-owned input — the host still
crosses to say something, which is the contract — and not one of them moved.

The patch columns move between kinds for a reason worth naming: the drive no longer stops at the turn
that received the events, so the `patch-ack` and `surface-visible` crossings now carry the publication
their OWN turn produced (17 → 160 and 0 → 53) instead of leaving it for a following empty crossing
(238 → 41). Total patches: **255 → 254**.

### 5.2 What the drive actually absorbed

Read directly off the worker's own `drivePolls`/`driveStopped`, not inferred:

```
worker MoreWork drive: 1395 guest turns absorbed over 724 crossings (1.93 per crossing, 69.8 per hop);
stops carried=471 idle=218 input=27 budget=8
```

* **1 395 guest turns ran without a host crossing** — 69.8 per hop that the host used to pay a POST, a
  poll, a structured clone and a main-thread pickup for.
* **471 crossings ended because the turn CARRIED something** — the contract's only "tell the host" stop.
* **218 ended `idle`**: the guest quiesced inside the drive, so the crossing that discovered it was the
  same one that delivered.
* **27 ended on a pending host input** — §2.3's macrotask yield doing exactly what it is there for; a
  drive is never the reason the host waits.
* **8 ended on the granted wall, and `steps` fired 0 times** — §2.7's re-order, measured: before it, 106
  crossings per run ended because a COUNT ran out while the grant was unspent.
* `closed` never fired: no actor was disposed mid-drive in this run.

### 5.3 The per-crossing cost moves the right way

| stage | before | after |
|---|---:|---:|
| `worker.guest` mean | 13.2 ms | **23.5 ms** |
| `worker.reply` per hop | 269 ms | 244 ms |
| `patch-ack` crossing mean | 12.9 ms | 37.6 ms |
| `RecalcStyleDuration` | 1.60 % of wall | 1.64 % (gate: under 5 %) |

Read the means the right way round: a crossing now contains 1.93 guest turns instead of one, so its own
span is longer while the number of crossings fell by 40 %.

### 5.4 The load caveat, stated rather than buried

`uptime` 1-minute load averages on a machine carrying ~16 lanes: **14.78** at the before run, **17.93**
at the after run. The hop wall (2 652 → 2 592 ms) and every millisecond column in §5.3 are therefore NOT
attributable to this change. The crossing counts and the drive histogram are, and they are what
§5.1/§5.2 are read for.

### 5.5 The journey, and the Flow window

`SEMIO_PROBE_URL=… bun 🐍️journey-probe.mjs` → `🗑️generated/more-work/journey3/`:

```
DONE steps 23 meshSteps 20
```

**23/23 steps converged (`converged=false` count: 0)** and **all 16 rows that declare a mesh oracle
report `meshOk=true`** with exact triangle counts and bounds inside their fixtures
(`eval-extrude@solid#0`, `eval-brep_bool_cut_5@solid#0`, `eval-fillet@solid#0`, `eval-fuse@solid#0`,
`eval-shell@solid#0`, `eval-rect@wire#0`); `meshOk=false` count: 0. Identical to the previous lane's run.

The coordinator asked this lane to check, urgently, whether the FLOW window on this stage shows the
example's graph or the fallback `Number → Math.add` one (reported on :6027). On a freshly recycled
:6021 carrying every change of this lane, booted at `?plugin=generation3d&example=hexagonal-mushroom-column`
(`🗑️generated/more-work/flow-check/`):

```
[DEBUG] node-graph surface ready surface=window:procedural-main nodes=7 edges=6
window:procedural-main {"height":"ok","radius":"ok","sides":"ok","profile":"ok","extrusion-axis":"ok","extrude":"ok","column-preview":"ok"}
```

**Seven nodes, six edges, and the seven port statuses are the hexagonal mushroom column's own** — the
example graph, not the fallback. No `Math.add` anywhere in the console. Whatever :6027 is showing is not
reproducible on this stage, so it is not attributable to the worker drive, the turn-result folding, the
`wire-turn` change or the shard-worker regeneration; the most likely remaining difference is the stage
:6027 is serving.

### 5.6 The fleet's `1024 continuations` fault is not this lane's

The coordinator asked this lane to verify the shell fault
`command ingress did not complete within 1024 continuations`, reported across the fleet since ~10:08.

**On :6021 it does not occur, before or after.** Grepped over every console this lane captured:

| run | `did not complete within 1024` |
|---|---:|
| `…/before2/console.txt` (10:08, 1 214 crossings, pre-change tree) | **0** |
| `…/after3/console.txt` (11:48, 824 crossings, post-change) | **0** |
| `…/journey2/console.txt` (23 steps) | **0** |
| `…/ingress-check/console.txt` (a 120 s boot, `🐍️console-dump-probe.mjs`, as asked) | **0** |

**And it predates every edit of this lane.** The fault is already in another lane's capture at **09:53**
(`🗑️generated/react-gaps2/menus/console.txt`, `action failed interactionSelect … 1024 continuations`),
and again at 10:28/10:30 (`react-gen-wire/{generate-mode,flow-wire}/console.txt`). This lane's first
host-TS edit was ~10:15 and nothing of it was SERVED anywhere before its own restage at 10:41. The
nearest change to its onset is the 09:46 auto-commit `700854a9ab`, which carries the
ui-turn-patch-batching lane's reshape of `PluginRuntime.acceptUiPatches`/`submitUiAcknowledgements` —
the same `settlePluginTurn` path the command-ingress drain runs through. That is where the evidence
points; the mechanism is not established here and is not claimed.

**What this lane does owe that ceiling, it now proves**: §4.1's
`the_hosts_continuation_ceiling_covers_more_guest_turns_under_the_drive_never_fewer` — a drive may never
cost MORE host continuations than the host-polled shape, and at the derived ceiling one ceiling of
1 024 continuations covers **7 168** guest turns instead of 1 024. The observed statuses in the peer's
fault are `idle`, i.e. the command page was never admitted at all — a state no number of guest turns
advances, so the drive neither causes nor hides it.

---

## 6. The wgpu twin

The wgpu shell's frame Worker runs the same generated shard worker and the same reactor, so all five
changes apply to it unchanged.

| gate | result |
|---|---|
| `cargo check -p semio-framework-os-renderer-wgpu --lib` | green (warnings only, 1 m 10 s) |
| `@semio-tech/framework-renderer-wgpu:generate-frame-worker` | green, twice (16.5 s / 19.1 s); the regenerated `🎞️frame-worker/🤖️generated/🟨️.js` carries `drivePolls` and the `message.beat` fold |
| `@semio-tech/framework-renderer-wgpu:wasm` | green (2 m 48 s, 8 tasks) |
| `activate-generation3d-wgpu-dev` | green twice (`NX_SKIP_NX_CACHE=true`, exit 0 — 1 m 18 s, then 4 m 9 s after §2.6) |
| `bun 🐍️wgpu-battery.mjs --only=frame-loop,examples` on **6118**, behind the standing gate | **`frame-loop` green**: 4/4 steps, 124 s, **0 page errors**, 20 gestures, **0 quarantines**, 7 058 frame batches all answered with **0 faults**, 16 actions published. `examples` red — see below. |

**The `examples` lane's failures are a stale served renderer on :6118, not this lane.** Every red row
names the same cause verbatim:

```
the served renderer exposes no dumpMeshStats export — rebuild @semio-tech/framework-renderer-wgpu:wasm and reload
worker-present-failed: presentation stalled: phase=Render engine=0 upload=1 gpu-cursor=true
```

`…:wasm` WAS rebuilt by this lane (green, 2 m 48 s), but :6118 is a shared serve this lane does not own
and it was never recycled onto that build, so the page is still running the previous renderer. The two
rows that did pass through it (`viewer: sphere-box-fuse`, `viewer: face-sweep-extrude`) publish the
right mesh counts and the right bounds (`eval-fuse@solid`, `eval-extrude@solid`), which is what says the
GUEST half — the half this lane changed — is delivering correctly. `frame-loop`, which does not need
`dumpMeshStats`, is green end to end. **The examples lane is not claimed either way** until whoever owns
:6118 recycles it.

## 7. What is left, named with its evidence

**36.20 crossings per hop, against a target of ≤ 10.** The term this lane owned is down by 74 % and the
remaining hop decomposes as:

| crossing | per hop | whose it is |
|---|---:|---|
| `message` | 10.0 | shell↔guest messages — host-owned input, **now the largest row** |
| `patch-ack` | 9.8 | the host's acknowledgement of a publication — host-owned input |
| `(none)` | 8.4 | and 41 of the run's 254 patches ride them, so about a quarter of this row is delivery |
| `surface-visible` | 4.0 | host-owned input |
| `request`/`completed` | 3.8 | host-owned input |

1. **27.6 of the remaining 36.2 crossings per hop are the HOST speaking**, and the drive is forbidden by
   contract from absorbing any of them. ≤ 10 per hop is below that floor on this workload unless the
   host says less: `message` (10.0/hop) and `patch-ack` (9.8/hop) are the two to attack, and both are
   host-side coalescing questions, not drive questions. **That is now the whole gap** — the drive's own
   term is 8.4 and a quarter of it is delivery.
2. **A refresh whose surfaces are still reconciling is answered inside the drive only as far as the
   grant reaches.** §2.7 removed the count that was cutting it short, and `steps` now fires zero times;
   what remains is the HOST's half, which this lane deliberately did not change:
   `settlePluginTurn` returns after one crossing when the refresh declared an EMPTY required-surface set
   (`hasRequiredUiPatches` is vacuously true, so `hasWork()` is false), so a reconcile the drive could
   not finish inside one grant waits for the next unrelated drain turn. Making that settle keep draining
   while the surfaces it asked for are still reconciling is a `PluginRuntime` cut, with the peer's
   250–800 ms per-request latency (ticket `26/09/13/INTERACTIVE-TOOLS-VISIBLE-PROCESS` phase 8) as its
   measurement. It is named here, not built here.
3. **`MEASURED_GUEST_TURN_COST_MS` now only feeds the coherence law and the expectation**, not the
   runtime cap (§2.7). It still has to be re-measured when the guest's turn cost moves, and §4.1 is the
   gate that forces it.
4. **The widened readiness gate did not measurably widen the live batch.** §2.5's law proves it correct
   and §4.1's ladder law proves ordering; the browser's patches-per-crossing concentrated because the
   DRIVE carries more turns per crossing, not because the gate admitted more at once.

## 8. Not claimed

- **The ≤ 10 crossings-per-hop target is NOT met**: 60.70 → 36.20. §7 item 1 names the floor with its
  measurement — 27.6 crossings per hop are the host speaking, and the drive may not absorb them.
- **No millisecond in §5.3 is attributed to this change.** The before and after runs straddle a
  14.8 → 17.9 load difference (§5.4). Only the crossing counts and the drive histogram are
  load-independent and only they are claimed.
- **The per-request refresh latency the coordinator asked for is NOT measured before/after here.** This
  lane has no :6013 serve and did not run the peer's tool timeline; what §2.7 states is the structural
  change (the drive spends its whole grant and `steps` stops firing — 106 count-exhausted crossings per
  run → 0) and §7 item 2 names the host-side half that still bounds it, with the peer's own numbers
  quoted as their evidence, not re-measured as mine.
- **`MEASURED_GUEST_TURN_COST_MS = 13` is a measurement of THIS door**, not a universal.
- **The widened readiness gate is proven correct per allocation, not proven to widen batches on this
  workload** (§7.4). Its ladder law prints `overlapped=0` rather than asserting it, because the drive
  extracts a ready output inside the same opportunity that produces it.
- **§2.6's fix is proven on the production ladder with a cut page, not on puzzle 3d.** The peer has
  since confirmed it on :6013 (their release boots and a 100-object fill completes); what THIS lane
  verified is the Rust law (4 pages, 3 cut, 4 surfaces delivered exactly once), the batching lane's own
  five laws still green, and the reactor corpus back to its five pre-existing reds.
- **The peer's suspected mechanism for that trap (a PAGED `publish_into`) is not what was wrong**, and
  is now refused by name instead of left possible (§2.6 part 3).
- **The fleet's `1024 continuations` fault is not attributed to this lane and its cause is not claimed**
  (§5.6): zero occurrences on :6021 before or after, including a 120 s boot; and a 09:53 occurrence in
  another lane's capture, before this lane's first host-TS edit and long before anything of it was
  served.
- **The :6027 fallback-Flow-graph report does not reproduce on this stage** (§5.5): a fresh :6021 boot
  at the same example publishes `nodes=7 edges=6` and the example's own seven ports. No cause is
  claimed for what :6027 is showing.
- **The wgpu `examples` lane is not claimed either way** (§6): every red row names a stale served
  renderer on a port this lane does not own. `frame-loop` IS claimed, and is green.
- **The 14 `🎭️actor` reds, the 5 reactor reds and the 4 React reds are pre-existing and peer-owned**,
  each named in §4.3. The two pre-existing reds this lane repaired (§4.4) are the ones that had to run
  for §2.4 to be observable.
- **Nothing about `settlePluginTurn` itself was changed.** The host loop is unchanged; it simply has
  fewer continuations to run because the guest stops asking for them.
