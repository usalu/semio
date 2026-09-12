# Wave B42 — the `cancelJob` guest turn that never yields

Ticket `26/09/02/PUZZLE-3D-END-TO-END`. Battery evidence: `🗑️generated/probe-2026-09-12T08-08-54.md`
(wasm #56), lines 1071–1073:

```
[DEBUG] shard 0 terminated by the host watchdog: the worker was silent for 18603 ms;
outstanding: cancelJob puzzle#1 started 18603 ms ago. A guest turn that never yields blocks the
worker's event loop and its progress ticker with it …
[DEBUG] action failed engagementAbort undefined Error: shard 0 terminated by the host watchdog …
```

`engagement-abort` itself verdicts PASS at 720.9 s; the shard death surfaces at 762.1 s
(`step outliner-rows … newFaults=…`), then `PluginRuntime: shard 0 lost, restoring actors: puzzle#1`
and 10 guest-death faults.

## 1. Wire path of the cancel (read, before measuring)

* `engagement_abort` — `✏️editor/🎮️commands/🛑️engagement-abort/🦀️.rs:26` — cancels the live plan
  (`precompute.cancel_fill_job_for`) and pushes `Effect::CancelJob { job }` + `SetActiveTool ""`.
* React host — `📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx:2263-2269` — `cancel-job`
  never becomes a `requestedEffects` entry; it is host work:
  `serializeCommandIngressForActor(actorId, () => shardClient.cancelJob(actorId, job))`.
* `ShardClient.cancelJob` — `🎭️actor/📮️shard-client/🟦️.ts:2082` — sends `{kind:"cancelJob", requestId}`,
  i.e. the **request-id** branch of the worker, not the fire-and-forget one.
* Shard worker — `🔌️plugin/📦️packages/🟦️typescript/🟦️.ts:531` — `await actor.api.cancelJob(msg.job)`
  inside `beginRequest()`; that is the outstanding request the watchdog names.
* Guest — `🔌️plugin/⚛️reactor/💼️jobs/🦀️.rs:403` `cancel_job` → `owner.cancel()` + `jobs.remove(&job)`.
* Puzzle owner — `✏️editor/⏳️precompute/🦀️.rs:3277` `BoundedJob::cancel` for `FILL_JOB_KIND`.

## 2. Synchronous drains found by reading (candidates for the runaway)

| file:line | shape |
| --- | --- |
| `⏳️precompute/🦀️.rs:3179-3201` | `Drop for Puzzle3dPrecomputeSession` loops `FILL_ENVELOPE_SESSION_CLOSE_TURNS = FILL_ENVELOPE_MAX_OPERATIONS * (FILL_ENVELOPE_MAX_ITEMS + 9) = 4 * 65 545 = 262 180` `pump_fill_terminal_step()` calls with **no deadline and no yield** |
| `⏳️precompute/🦀️.rs:2453-2461` | `Drop for Puzzle3dFillSession` re-installs itself into a throwaway `Puzzle3dPrecomputeSession`, whose `Drop` then runs the loop above |
| `✏️editor/🦀️.rs:3036-3053` | `puzzle3d_session_check_in` **drops** the whole `Puzzle3dSessionState` (fill session included) on a stale lease / byte-budget refusal / contended registry |
| `✏️editor/🦀️.rs:3036 (retire)` | `Puzzle3dSessionRegistry::retire` drops a slot's state **while holding the registry mutex** |
| `⏳️precompute/🦀️.rs:1559` | `CollisionMutationStep::Rejected(mut rejected) => while !rejected.retire_one() {}` — unbounded |

Each `pump_fill_terminal_step` at `close_cursor == 2` retires exactly ONE `FillBuilderRetirementCursor`
owner, so the drain is O(retained owners) — Nakagin-scale plans hold thousands.

## 3. Measurement (in progress)

### 3.1 The census of the runaway turn — measured, not inferred

Law `engagement_abort_tears_the_fill_plan_down_across_turns_and_never_inside_one`
(`✏️editor/🧪️tests/🔬️unit/🦀️.rs`) loads the real Nakagin example through the typed
`setActiveExample`, arms Fill, drives `fillBuildTick` + the REAL bounded `FILL_JOB_KIND` job until the
plan publishes readiness, then dispatches `engagementAbort` and executes the `Effect::CancelJob` it
requests through the real `reactor::jobs::cancel_job`. The census unit is one
`FillEnvelopeTerminalHandle::close_step` (one retired plan owner, or one close-cursor stage), read via
`precompute::fill_close_unit_census()`.

First run (before any fix):

```
the teardown the cancel started never finished:
abort_turn=0 units, cancel_job=0 units, teardown turns=65545 worst_turn=0 quiet=false, occupancy=(1, 1)
```

So the runaway is NOT where the watchdog text points. Two separate defects:

* **The guest's cancel is already cheap.** `engagementAbort` charges 0 close units and
  `jobs::cancel-job` charges 0 — `BoundedJob::cancel` only trips the token and requests the
  `Cancelled` terminal. There is no unyielding guest turn at all.
* **The teardown never advances.** 65 545 maintenance turns later the envelope is still occupied and
  not one close unit has been spent. The cancelled Nakagin-scale plan keeps every retained owner and
  one of the four process-wide slots until some `Drop` drains the whole ladder inline.

## 4. Root cause A — the shard death: an unanswered `cancelJob` request

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🟦️.ts`, the
`shardWorkerSource()` message handler (was line 475):

```js
if (kind === "cancelJob") {
  const actor = actors.get(msg.actorId);
  actor?.api.cancelJob(msg.job);           // no await, no reply, no heartbeat, no beginRequest
  return;
}
```

This early branch sits **before** the `requestId`/`actorId` gate, so it wins for every `cancelJob`
message — including the one `ShardClient.cancelJob` sends WITH a `requestId`
(`🎭️actor/📮️shard-client/🟦️.ts:2082`, whose `OutboundMessage` union declares `requestId` as
required for the kind). Consequences, all three fatal together:

1. Nothing is ever `reply()`d, so the client's `await this.send(…, requestId)` never settles and the
   entry stays in `pending` forever, ageing.
2. `heartbeat()`/`beginRequest()` are never reached, so no beat is recorded AND the while-busy
   progress ticker never starts — an otherwise idle worker emits nothing at all.
3. `case "cancelJob"` in the switch below (which DOES await and reply) is unreachable dead code.

The watchdog then reports the one thing that never happened: a stale heartbeat plus the oldest
outstanding request. `the worker was silent for 18603 ms; outstanding: cancelJob puzzle#1 … A guest
turn that never yields` is a misdiagnosis — the guest ran for 0 units.

Timeline from the battery confirms it: `engagement-abort` PASS at 720.9 s (the dispatch that pushes
`Effect::CancelJob`), the next interaction `outliner hide … waitedMs=30268` at 758.1 s, watchdog kill
inside that window, `outliner-hide-applies FAIL` / `outliner-show-restores FAIL` as collateral.

### Fix A

* Deleted the early fire-and-forget branch; `cancelJob` now falls through to the request-id gate, so it
  beats, starts the progress ticker, awaits the guest, and replies — exactly like `startJob`/`stepJob`.
* Kept the old branch's tolerance of a vanished actor by answering it instead of throwing:
  `if (!actor && kind === "cancelJob") { reply(requestId, undefined); return; }`.
* A guest cancel that traps now becomes an error result (through the handler's own `replyError`)
  instead of an unhandled promise rejection.

### Law A

`🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🧪️tests/🛑️cancel-job-reply/🟦️.ts`, registered from
`📮️shard-client/🟦️.ts`'s `import.meta.vitest` block. It runs the REAL `shardWorkerSource()` in a
`node:vm` context (the same harness `⏱️retryable-lifecycle-deadline` uses) and states three things:
a `cancelJob` request is answered with `{kind:"result", ok:true}` and at least one heartbeat and the
job reaches the guest; a `cancelJob` for a vanished actor is still answered; a trapping guest cancel
becomes `ok:false` rather than silence.

Before: 3 failed, plus `Unhandled Rejection: Error: guest cancel trapped`. After: 3 passed.

## 5. Root cause B — cancellation was never an incremental job

Two halves, both in the app, plus one in the framework:

1. **No always-on driver.** `fill_build_tick`
   (`✏️editor/🎮️commands/🪣️fill-build-tick/🦀️.rs:17`) early-returns when the Fill tool is inactive, and
   the host stops dispatching ticks entirely once it is
   (`🌐️World3dHost/🟦️.tsx:5513` `worldFillBuildShouldTick(activeUtility, …)`). `engagement_abort`
   cancels the plan AND pushes `SetActiveTool ""` in the same turn, so `poll_fill_job` —
   the one caller of `pump_fill_terminal_step` — is never reached again. Puzzle3d implemented NONE of
   `ArtifactEditor::mounted_job_maintenance_step` / `mounted_job_close_step` /
   `mounted_jobs_terminal_is_empty`, unlike `🔋️energy` and `🏗️fem`, so it had no per-turn hook at all.
2. **A `Drop` that drained the whole ladder inline.** `Drop for Puzzle3dPrecomputeSession`
   (`⏳️precompute/🦀️.rs:3200`) looped `FILL_ENVELOPE_SESSION_CLOSE_TURNS = 4 * (65 536 + 9) = 262 180`
   `pump_fill_terminal_step()` calls with no deadline and no yield, and `Drop for Puzzle3dFillSession`
   routed into it by installing itself into a throwaway session (allocating a whole
   `Puzzle3dCollision` on the way). Those drops fire from
   `puzzle3d_session_check_in`'s refusal paths (`✏️editor/🦀️.rs:3036`) and from
   `Puzzle3dSessionRegistry::retire` (`✏️editor/🦀️.rs:3053`) — the latter **while holding the session
   registry mutex** — so a plan-sized teardown landed in whichever guest turn happened to trigger it.
3. **The framework's maintenance ladder skipped the app's own stage while idle.**
   `PluginApp::maintenance_step` (`🔌️plugin/🦀️.rs:25787`) short-circuits to
   `advance_snapshot_read_returns_one` when its own queues are empty, and that early return leaves the
   round-robin stage cursor untouched (the testkit's `measure_maintenance_step` doc says so in as many
   words). Stage 15 — `A::mounted_job_maintenance_step` — is therefore unreachable exactly when an
   otherwise-idle app is the one with cleanup owing. The law measured this directly: 65 545 maintenance
   turns, `worst_turn=0`, envelope still occupied.

### Fix B — the incremental design

* **`precompute::fill_envelope_maintenance_step(maximum_items)`** — one `FillEnvelopeTerminalHandle::
  close_step` per call (one retired plan owner, or one close-cursor stage), returning
  `PluginCloseStep::Pending` while work remains, `Complete` when the registry holds nothing finished,
  `Blocked` while somebody else holds the plan. Its cursor is a process-global
  `Mutex<Option<FillEnvelopeTerminalHandle>>` (`fill_envelope_reaper()`), because the cursor has to
  outlive a turn.
* **`FillEnvelopeRegistry::take_finished`** — claims ANY envelope whose run is over and that nobody
  holds, whatever session abandoned it. `Complete` is deliberately excluded: a finished plan is what the
  fill-count slider reads and the user applies, so only the owning session may end it. The predicate is
  now one shared function, `fill_envelope_run_is_over`, which `take_terminal_fill_job` also decides by,
  so the owning session and the reaper can never disagree about what is still readable.
* **`FILL_FAULT_NOTICE`** — a faulted envelope reaped with no session left to report it latches the
  user-visible notice process-wide; `take_fill_fault_notice` takes it. Without this, giving the reaper
  authority over `Fault` terminals would have silently dropped the `fill_failed` notice.
* **Both `Drop`s are now constant time** — trip the cancel token, request the `Closed` terminal, hand the
  checkout back (`FillEnvelopeTerminalHandle::drop` already does that). No loop, no allocation.
  `FILL_ENVELOPE_SESSION_CLOSE_TURNS` survives as the TURN budget the law derives its bound from.
* **`ArtifactEditor` hooks on `Puzzle3dPlayApp`** (`✏️editor/🦀️.rs`) — `mounted_job_maintenance_step`,
  `mounted_job_close_step`, `mounted_jobs_terminal_is_empty` delegate to the three functions above, the
  same shape `🔋️energy`/`🏗️fem` use.
* **The framework idle predicate** (`🔌️plugin/🦀️.rs`) gains
  `&& self.live_runtime_instance_id.is_none_or(|id| A::mounted_jobs_terminal_is_empty(id))`: an app with
  owner-local cleanup still owing is not idle, so its stage 15 keeps being reached. This is a
  framework-wide correctness fix — every app with a `mounted_job_*` implementation was affected.

### Fix B — two further wedges the law uncovered on the way

1. **The witness deadlocked its own driver.** `fill_envelope_terminal_is_empty` first looked only at
   envelope PHASES. A cancel only *requests* the terminal (`request_fill_envelope_terminal` sets an
   atomic intent); the phase flips in `apply_fill_envelope_terminal_intent`, which only the reaper's own
   `take_finished` calls. So the witness answered "empty" for a freshly cancelled plan, the framework
   took its idle fast path, stage 15 never ran, and the intent was never applied — measured again as
   65 545 turns with `worst_turn=0`. It now also reports a PENDING intent
   (`fill_envelope_terminal_intent_is_pending`).
2. **`retire_one`'s terminal arm can spin forever, and nothing could say why.**
   `FillBuilderRetirementCursor::retire_one`'s `_` arm returns `false` for as long as
   `FillBuilder::terminal_owners_empty()` is false, and that was a 75-clause conjunction that could only
   answer "not empty". The old `Drop` drain therefore ALSO never finished — it just burned up to 262 180
   iterations and gave up, leaving the slot occupied. `terminal_owners_empty` is now derived from
   `FillBuilder::terminal_owner_debt() -> Option<&'static str>`, which names the first unmet clause;
   `FillBuilderRetirementCursor::owner_debt` and `precompute::fill_envelope_close_debt()` surface it, and
   `fill_envelope_close_diagnostics()` reports each envelope's `(slot, close_cursor, worker_outcome,
   worker, fill, retirement)`. That is what turned "never finished" into
   `close_debt=[(0, "fixture_terminal_owners_empty base")]` in one run.
3. **One failing law used to fail twelve more.** Several `fill_worker_*` laws asserted while HOLDING
   `fill_envelope_registry()`, so the first failure poisoned the process-wide mutex and every later law
   reported `try_lock` errors (`occupancy=(18446744073709551615, …)`) instead of its own verdict.
   `drain_orphaned_fill_envelope` now reads, releases, then asserts.

## 6. Laws and outputs

### Law A — the shard worker owes `cancelJob` an answer
`🎭️actor/📮️shard-client/🧪️tests/🛑️cancel-job-reply/🟦️.ts` (3 laws, real `shardWorkerSource()` in a
`node:vm`). Before: `Tests 3 failed`, plus `Unhandled Rejection: Error: guest cancel trapped`.
After: `Test Files 1 passed | 10 skipped (11)` / `Tests 3 passed | 246 skipped (249)`.

### Law B — the cancel turn, and the teardown across turns
`✏️editor/🧪️tests/🔬️unit/🦀️.rs::engagement_abort_tears_the_fill_plan_down_across_turns_and_never_inside_one`.
Passing census (`🗑️generated/wave-B42-laws.txt`):

```
puzzle3d.cancel-teardown-census abort_turn=0 units, cancel_job=0 units, ladder_units=3 over 96
maintenance turns, teardown grants=14388 worst_turn=1 quiet=true, occupancy=(0, 1),
close_position=[], close_debt=[]
```

Read it as: the `engagementAbort` turn and `jobs::cancel-job` charge ZERO close units; the framework's
own maintenance ladder does reach the app's teardown stage (3 units in 96 turns — once per
`MAINTENANCE_STAGES`); the whole Nakagin plan retires in 14 388 granted units, each granted call
spending exactly ONE; and the registry ends quiet with nothing owed.

### Law C — the other long-turn candidates, same census
`…::no_document_scale_turn_retires_a_live_fill_plan_inside_itself` — example switch, `importFixture` of
the Nakagin projection, and `deleteSelection`, each on its own app with a live fill plan armed:

```
puzzle3d.turn-close-units example-switch=admission:0/ladder:0 import-fixture=admission:0/ladder:0
delete-selection=admission:0/ladder:0
```

Hot-swap `destroyApp` is a renderer-side concern and reaches the guest only as `instance-close`, which
runs the framework's own close ladder (`mounted_job_close_step` is now wired to the same one-unit
grant); no puzzle3d code path tears a plan down inline any more.

## 7. Verification (all foreground)

| command | tail |
| --- | --- |
| `RUST_MIN_STACK=… cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib -- --test-threads=1 fill cancel abort` | `test result: ok. 91 passed; 0 failed; 0 ignored; 0 measured; 650 filtered out; finished in 666.07s` |
| `CARGO_INCREMENTAL=0 RUST_MIN_STACK=… cargo test -p semio-framework-plugin --lib -- --test-threads=1 cancel watchdog` | `test result: FAILED. 31 passed; 5 failed; …` — the SAME five names with my hunk temporarily removed, so pre-existing (see below) |
| `cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly` | `warning: … generated 92 warnings` / `Finished dev profile … in 22.60s` (0 errors) |
| `cargo check -p semio-s-plugin-puzzle --target wasm32-wasip2` | `Checking semio-s-plugin-puzzle v0.1.0 …` / `Finished dev profile … in 36.47s` (0 errors) |
| `bunx vitest run -t 'ShardWorkerCancelJobReply'` (actor package) | `Test Files 1 passed | 10 skipped (11)` / `Tests 3 passed | 246 skipped (249)` |

The five `semio-framework-plugin` failures are `instance_close_cancellation_drops_the_instances_tasks_…`,
`key_dedupe_cancels_the_previously_live_task_under_the_same_key`,
`peer_roster_saturation_cancel_stale_and_interrupted_close_preserve_exact_authority`,
`retained_composed_replacement_cancellation_during_open_closure_and_view_preparation_preserves_the_live_bundle`
and `cancel_of_a_parked_task_drops_it_and_frees_its_slot_for_reuse` — reactor-executor task supersession
(`plugin.task.supersession-pending`, "a detached slot must be reusable by a later spawn"), a peer's live
work. Baselined by removing the one-line idle-predicate hunk and re-running: byte-identical failure set.

## 8. Probe verdicts that should stop dying on #57

The cancel no longer strands a request, so shard 0 is not killed after `engagement-abort`. Everything
that died as collateral in #56 should come back:

* `battery-hard-faults` — the 13 hard faults in #56 were one watchdog kill plus `shard 0 lost, restoring
  actors: puzzle#1` plus ten `actor-activation.revoked` faults downstream of it.
* `outliner-hide-applies` and `outliner-show-restores` — the two verdicts that FAILED at 758.1 s after
  `waitedMs=30268`; that wait WAS the wedged shard.
* every verdict after 758 s that reported `action failed … Error: actor-activation.revoked` —
  `engagementAbort`, `engagementSubmit`, `setSelectionFlag`, and the typed-operation completion effects.
* `engagement-abort` itself keeps passing, and now leaves the actor alive behind it.

Two things will NOT change on #57 and are not this wave's: the five `semio-framework-plugin` reactor
task-cancellation laws above, and whatever the peers' in-flight taxonomy relocation is still moving.

## 9. Files

* `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🟦️.ts` — worker `cancelJob` now
  answers its request; the vanished-actor case replies instead of throwing.
* `🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🧪️tests/🛑️cancel-job-reply/🟦️.ts` (new) +
  `📮️shard-client/🟦️.ts` registration.
* `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` — `maintenance_step`'s idle predicate now
  consults `A::mounted_jobs_terminal_is_empty`; trait doc says why.
* `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/…/✏️editor/⏳️precompute/🦀️.rs` — reaper, registry
  `take_finished`, shared `fill_envelope_run_is_over`, pending-intent witness, process-wide fault notice,
  close-unit census, close diagnostics/debt, `reap_fill_envelopes_for_test`, both `Drop`s made constant
  time.
* `…/✏️editor/⏳️precompute/🪣️fill/🦀️.rs` — `terminal_owner_debt` names the first unmet retained-owner
  clause; `terminal_owners_empty` derives from it; `FillBuilderRetirementCursor::owner_debt`.
* `…/✏️editor/🦀️.rs` — `mounted_job_maintenance_step` / `mounted_job_close_step` /
  `mounted_jobs_terminal_is_empty` on `Puzzle3dPlayApp`.
* `…/✏️editor/🧪️tests/🔬️unit/🦀️.rs` — laws B and C.
* `…/✏️editor/⏳️precompute/🧪️tests/🔬️unit/🦀️.rs` — the two drop-semantics laws restated against the
  granted reaper; `drain_orphaned_fill_envelope` no longer asserts under the registry lock.
* Repaired, not mine: six `context::` → `artifact_app_laws::` call sites inside the peer's new inner
  `mod context` in `…/✏️editor/🧪️tests/🔬️unit/🦀️.rs`. Their codemod renamed the FRAMEWORK's `testkit::`
  along with the module it was inlining into, leaving `pub fn meta` calling `context::meta` (unresolvable
  and self-recursive) and the whole crate unbuildable.
