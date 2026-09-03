# Terra Plugin-Host Lifecycle Implementation Audit

## Scope and decision

This is a read-only reinspection of the current tree after the boxed-registry stack repair.  No production or test source was changed and no build or test was run for this audit.  The earlier reports remain useful historical evidence, but the conclusions below are limited to the source currently present.

The smallest safe ordering is:

1. **Sol L0 — production-safe retirement:** make replay owners safely destructible, funnel every mounted replay fault into close before reporting it, and give detached mounted relays a registry-owned bounded reaper with a real wake path.
2. **Sol L1 — stale expectation migration:** only after L0, update tests that confuse an unretained, same-turn cancellation with a failed guest cancellation, and add the lifecycle fixture/oracle laws.
3. Run focused L0/L1 tests, then the full plugin-host suite.  A full-suite result before both packets cannot distinguish the known obsolete assertions from an abort or a leaked counter.

L0 is deliberately confined to the plugin host.  It neither changes the WIT job protocol nor needs browser relay, socket grants, catalog completion, native mount repair, CAS, or a renderer change.

## Current ownership findings

| Finding | Current owner | Classification | Why it must change in L0 |
| --- | --- | --- | --- |
| `FixedReplaySeedPage`, `FixedReplaySeed`, `ReplaySpawnRefusal`, and `MountedReplaySeed` use `ManuallyDrop` and panic in debug unless incremental retirement completed.  In release the nonterminal branch does not destroy the owned values. | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🦀️.rs`, `FixedReplaySeedPage` through `MountedReplaySeed` | Genuine production defect | A host fault or ordinary unwinding can become a debug abort; release leaves page/ABI accounting and heap owners stranded. |
| `drive_replay_seed` returns directly from checkpoint, ABI admission, restore, UTF-8, missing-instance, and several start/restart paths.  `CaptureKind`, `CaptureInput`, and `CaptureCheckpoint` use `is_ok_and`, silently retaining a seed if copying fails. | Same shard file, `drive_replay_seed` | Genuine production defect | The job can retain its seed and counters after a fault rather than entering the one bounded closure path. |
| `unregister` clears the job maps and instance but does not directly mark matching mounted seeds/refusals for close.  The selector eventually notices an absent instance only if the shard is driven again. | Same shard file, `unregister` | Genuine production liveness defect | Actor retirement must leave a discoverable closing owner immediately, not rely on a later unrelated drive. |
| `GuestRelayMountedFuture::drop` only sets `close_requested`; `pump_retirement` makes one opportunistic scan when another foreground relay is polled and passes no waker.  A detached blocked relay can remain mounted indefinitely. | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️.rs`, `GuestRelayMountedRegistry`, `GuestRelayMountedFuture` | Genuine production liveness/capacity defect | An abandoned request can permanently consume one of the 16 slots and retain guest cleanup without another caller. |
| The `close_requested` boolean conflates a detached request with a normal terminal request whose live future still owns the result.  Cross-slot opportunistic retirement can therefore race caller-owned output. | Same host file, `GuestRelayMountedSession`, `pump_retirement`, `pump` | Genuine production ownership ambiguity | A registry reaper must only consume explicitly detached slots; it may never steal a live future's terminal output. |
| `GuestColdRelayJob::Drop` is another debug-only terminal assertion.  A correct reaper makes it unreachable in normal operation, but a drop guard must not turn a prior fault/unwind into a second panic. | Same host file, `GuestColdRelayJob::Drop` | Production hardening required by L0 | `Drop` may signal cancellation/last-resort cleanup, but must never be the normal asynchronous driver or panic in either build mode. |
| `cancel_job_effect_failure_retires_the_actor_and_surfaces_the_typed_fault` scripts Spawn+Cancel in one guest turn, then expects guest `cancel_job` to run and retire the actor.  The current protocol correctly sees the seed as not retained and locally marks it Closing instead. | Shard test in `🧵️shard/🦀️.rs` | Obsolete synchronous assertion | No guest job was admitted, so there is no guest cancellation to fail.  The test currently also exposes the real unsafe-drop defect when its undrained seed leaves scope. |
| The successful same-turn cancellation test asserts only that no job step occurs, but leaves its just-closing seed undrained. | Shard test in `🧵️shard/🦀️.rs` | Valid law with obsolete cleanup shape | Keep the semantic assertion; add bounded close-draining and baseline accounting assertions after L0. |

The boxed `replay_seeds`, refusal registry, and mounted relay slots are current concurrent stack-repair work and are not defects in this packet.  L0 must preserve their capacities and the existing heap-authority checks in `🧫️fixtures/🔣️stack-authority.json`.

## Sol L0: exact implementation packet

### 1. Make replay ownership ordinary and drop-safe

**Primary file:** `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🦀️.rs`.

Replace the `ManuallyDrop<Option<_>>` fields in `FixedReplaySeedPage`, `FixedReplaySeed`, `ReplaySpawnRefusal`, and `MountedReplaySeed` with ordinary `Option<_>` fields/arrays.  The manual incremental path remains for scheduling fairness, but it must be an optimization, not a precondition for memory safety:

- `FixedReplaySeedPage::Drop` releases exactly one `JOB_REPLAY_SEED_PAGES` unit if and only if its backing allocation is still present.  `close_one` removes a page from its slot and lets this destructor do the decrement; it must not decrement separately.
- `FixedReplaySeed` needs no terminal-only destructor.  Dropping its option arrays drops each remaining page, and therefore balances every admitted page.
- `ReplaySpawnRefusal` needs no custom destructor.  Its `String` and byte owners always drop, whether publication reached `RetireShell` or not.
- `MountedReplaySeed` retains at most one small custom `Drop`: atomically return its remaining `abi_reserved` amount exactly once, zero that field, then let its ordinary fields drop.  Add one internal `release_abi(bytes)` helper for normal phase transitions; it validates/debits the local reservation before decrementing `JOB_REPLAY_ABI_BYTES`.  All individual materialized owner retirements use this helper.

Do not retain `debug_assert!(false)` as a destruction policy.  Phase and counter invariants belong at explicit transition boundaries where a failure can be reported; unwinding and release-mode `Drop` must be idempotent, non-panicking, and accounting-complete.

The resulting accounting laws are:

```text
JOB_REPLAY_SEED_PAGES == number of live FixedReplaySeedPage backings
JOB_REPLAY_ABI_BYTES == sum(MountedReplaySeed.abi_reserved for live seeds)
0 <= either counter <= its existing process cap
```

Every successful page/ABI admission creates exactly one matching release, either through an explicit one-opportunity close transition or the drop fallback.  No field may be manually destroyed after it has been moved out, and no terminal path may decrement twice.

### 2. One failure-to-close funnel for every mounted replay stage

**Primary file:** the same shard file; **owner:** `ShardLoop::drive_replay_seed`.

Add a private, idempotent replay close reason and helper, for example:

```text
ReplaySeedCloseReason = Cancelled | ActorLost | Fault { stage, detail }
MountedReplaySeed { close_reason, phase, ... }
ShardLoop::begin_replay_seed_close(index, reason)
ShardLoop::fail_replay_seed(index, stage, error)
```

The names can vary, but the owner and behavior must not.  `begin_replay_seed_close` records only the first reason, moves the seed to `Closing`, and removes the exact `(actor, job)` keys from `running_jobs`, `job_turns`, `job_authorities`, and `job_placement` if activation already reached them.  It does not drop the seed.  `fail_replay_seed` calls it before attempting a terminal `Event::JobCompleted { Err(...) }`/`ShardOutcome` publication.  If publication itself is back-pressured or errors, the seed is already Closing and remains schedulable on the next grant.

Route all of the following through it rather than returning an error with a mounted owner still in an active phase:

- page-copy capacity/allocation refusal in each capture phase (replace `is_ok_and` with explicit `Ok(false)`, `Ok(true)`, and `Err` handling);
- checkpoint failure and a lost actor before checkpoint;
- live start failure and lost actor before live start;
- all three ABI-buffer admission failures;
- restore failure and a lost actor before restore;
- malformed materialized UTF-8, preserving/releasing its byte reservation even though `String::from_utf8` consumed the vector;
- replay restart failure and lost actor before restart;
- cancellation and actor-unregister paths.

The normal `Closing` branch still releases at most one owner/page per replay close opportunity.  It is the only path that performs scheduled retirement.  The first fault is externally observable once; later attempts merely continue close.  A cancellation before `Retained` is local seed cancellation, not a guest `cancel_job` call and not an actor retirement.

`unregister` must call a small `retire_actor_replay_owners(actor, ActorLost)` before/while clearing maps, marking all matching mounted seeds.  Treat a pending spawn refusal as an ordinary owned rejection: it must be safe to drop and must not block actor retirement; whether its already-authorized rejection completion is emitted must be defined once in this helper rather than inferred from a later missing-instance scan.

### 3. Registry-owned detached-relay reaper and wake path

**Primary file:** `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️.rs`.

Replace `GuestRelayMountedSession.close_requested: bool` with a lifecycle that distinguishes ownership:

```text
Running
DrainingForCaller      // a live GuestRelayMountedFuture still owns the result/error
DetachedForReap        // its future dropped; registry now exclusively owns retirement
Empty
```

`GuestRelayMountedFuture::drop` generation-checks its slot, starts the underlying owner close/cancel, and transitions it to `DetachedForReap`.  `finish_outcome` and output-write failure transition to `DrainingForCaller`; they never make a live caller's result reaper-owned.  Stale generation requests remain no-ops.

Extend `GuestRelayMountedRegistry` with a small registry-owned reaper state: a rotating cursor, a single-active/recheck guard, and the current reaper waker/epoch needed to close the schedule race.  Construct it with the already available `plugin_host_worker_pool()` in `PluginInstanceHandle::new`; use the existing `GuestRelayPoolFuture` on `Lane::Maintenance`, not an OS thread or a new executor.  A private `GuestRelayMountedReaper` future is sufficient.

Its one poll does all of the following and nothing more:

1. Select at most one generation-qualified `DetachedForReap` slot, rotating from the cursor.
2. Perform one `pump_close` opportunity, preserving the existing one owner/page bound.
3. Empty the slot only after its owner is terminal.
4. On `Pending`, self-wake once for a later finite pool turn.
5. On `Blocked`, register the reaper's own waker with the underlying `WorkerJobSession`/relay completion path.  For an owner shape that cannot register a wake, schedule one coalesced, short maintenance timer recheck; the wait occupies no worker and is cancelled when the slot closes.
6. End when no detached slots remain.  Use a recheck epoch or equivalent two-phase handoff so a `request_close` racing this idle decision starts/wakes exactly one new reaper.

Delete the cross-request `pump_retirement(skip)` as the lifetime mechanism.  Foreground `pump` drives only its current, generation-matched caller-owned slot; the registry reaper drives detached slots.  This removes both the stalled-drop dependency on a later foreground request and the risk of reaping another live caller's completed result.

The reaper is cancellation-aware by construction: dropping a caller signals the existing job cancel token; detached retirement is bounded and continues only until the finite slot becomes empty.  Its progress unit is one slot/one `pump_close`/one page-or-owner release.  Its deadline fallback is a coalesced maintenance timer, never a busy loop or a worker-held sleep.

Change `GuestColdRelayJob::Drop` from a debug-only assertion into a non-panicking last-resort cancellation signal.  It may request the existing cleanup machinery, but it must not pretend to complete asynchronous cleanup in `Drop`; normal cleanup remains owned by the registry reaper.  This avoids a secondary unwind panic in debug while keeping the same release-mode ownership safety.

## Ownership state machine

```text
Replay seed
  Spawn owner -> CaptureKind -> CaptureInput -> Checkpoint -> CaptureCheckpoint
  -> LiveStart -> Activate maps -> Retained
  Retained -> Materialize -> Restore -> Restart -> Activate maps -> Retained
  any cancellation / actor loss / fault -----------------------------> Closing
  Closing -(one owner or page per grant)-> RetireSeedShell -> RetireMountedShell -> dropped

Mounted relay
  Reserved -> Running -> DrainingForCaller -> Empty       (live future receives result)
  Reserved -> Running -- future Drop --> DetachedForReap
  DetachedForReap -(one bounded reaper opportunity)-> Empty
```

There is no state in which a replay owner is both undiscoverable and non-droppable, and no state in which the registry reaper owns output intended for a live future.  The fallback destructors make either machine safe if the process instead unwinds through an intermediate state.

## Tests, neutral fixture, and independent oracle

L0 must land with focused tests; the test migration below must not be used to hide a production lifecycle failure.

### Neutral fixture and two implementations

Add `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧫️fixtures/🔣️relay-lifecycle.json` with a schema version, fixed slot/page capacities, and literal traces for:

- each replay failure stage becoming `Closing` before one terminal outcome;
- an abandoned pending relay receiving cancel, a blocked wake, bounded releases, and an empty slot;
- a live terminal relay retaining caller ownership of its output;
- stale generation close refusal and max-plus-one slot refusal;
- expected `seedPages`/`abiBytes` baselines before and after each trace.

Consume that fixture literally from a Rust adapter beside a Gherkin feature under `🖥️host/🧪️tests/relay-lifecycle/` and from an independent TypeScript adapter in the same directory, following the existing `mutate-os-config-identity` Rust/TypeScript adapter pattern.  The Rust subject uses the production shard/relay transition API and `serde_json`; the TypeScript adapter is a separate, small ownership-state interpreter over the literal fixture.  Neither implementation derives expected transitions from the other.  This gives a language-neutral specification and an independent oracle while retaining the existing third-party JSON parser in the native subject.

Keep `🧫️fixtures/🔣️stack-authority.json` and its current Rust test.  Extend it only if reaper metadata changes the inline-size authority; the `guest-relay-mounted` storage remains `heap` and its capacity remains 16.

### Focused native laws

Add or tighten tests in the existing shard test module and `guest_cold_relay_tests` in the two owner files:

1. Drop a mounted replay seed from every representative pre-retained, retained/materialized, and closing phase under `catch_unwind`; no debug panic occurs, fields are freed, and both counters return to their entry baselines.
2. Force each failure funnel family (capture admission, checkpoint, live start, ABI admission, restore, UTF-8, restart, lost actor) and assert: first reason wins, phase is `Closing` before reporting, no later job step occurs, and one bounded close sequence reaches an empty slot.
3. Cancel an unretained same-turn spawn and assert zero guest cancel/step admissions, actor remains registered, then drain the seed and verify accounting.  Separately retain a job, cancel it, and assert one guest cancellation plus exact map removal.
4. Unregister an actor with pre-retained, retained, and refused owners; assert they become discoverable retirement work immediately and close without another actor's event.
5. Drop a pending `GuestRelayMountedFuture`, release its gate, and prove the registry reaper cancels and frees the slot with no second foreground `infer`/`compose`/`io_run` poll.  Cover `Blocked` wake and timer-fallback paths.
6. Fill all mounted slots, detach them, and show round-robin bounded reclamation; verify a stale generation cannot close a new occupant.
7. Complete a still-live relay while another detached relay reaps; the live future receives its exact output/error, proving the reaper does not steal `DrainingForCaller` state.
8. Retain the stack-size law and assert the reaper's small registry state stays below the fixture's maximum inline bytes; all fixed slot storage stays boxed.

Use finite attempt bounds expressed as opportunities, not wall-clock sleeps.  The only timer assertion is that a blocked recheck is coalesced and does not hold a worker while waiting.

## L1: test migration after L0

Only after the L0 laws pass, change `cancel_job_effect_failure_retires_the_actor_and_surfaces_the_typed_fault` into two tests:

- **unretained Spawn+Cancel:** local seed cancellation, no `GuestRuntime::cancel_job`, no actor retirement, bounded seed close and baseline counters;
- **retained cancellation failure:** first advance the seed through activation, then script guest cancellation failure and assert exactly one actor retirement/fault plus bounded seed close.

Keep `cancel_job_effect_stops_a_job_before_it_is_ever_stepped`, but add its required drain/accounting assertions.  Do not weaken assertion counts, replace assertions with sleeps, or declare failures flaky.  This is a semantic migration from synchronous admission assumptions to the current staged replay protocol.

## Concurrent-change and dependency boundaries

- The following files are actively modified in the shared tree and overlap this packet: `🖥️host/🦀️.rs`, `🖥️host/🧵️shard/🦀️.rs`, `🖥️host/🧵️shard/🧵️executor/🦀️.rs`, `🖥️host/🧵️shard/🚚️process-transport/🦀️.rs`, and `🖥️host/🧫️fixtures/🔣️stack-authority.json`.  Assign one writer for L0, re-read those exact regions immediately before editing, and preserve unrelated stack and transport changes.
- The boxed registries from `📓️sol-plugin-host-stack-overflow.md` are a prerequisite already visible in the current tree.  L0 must not move them back onto the stack or change their declared capacities.
- Browser socket grants, server document-open plans, catalog completion, native mount repair, and CAS are not prerequisites for L0.  They must remain fail-closed independently; this packet does not invent a browser credential or a document-opening path.
- No runtime/build assertion is made here.  The prior report's observed abort motivates the order, but this audit ran no commands that execute code.

## Path and blocker order

1. `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🦀️.rs` — replay ownership, fault funnel, actor retirement, shard laws.
2. `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️.rs` — detached relay lifecycle, registry reaper/wake, host laws.
3. `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧫️fixtures/🔣️relay-lifecycle.json` and `🧪️tests/relay-lifecycle/` — neutral trace and independent oracle.
4. Existing shard tests — only after 1–3, migrate the two synchronous cancellation shapes.
5. Run focused tests, then a meaningful full plugin-host suite rerun.

The only current blocker is shared-file coordination for items 1–2.  There is no architecture dependency on the unfinished catalog/native-mount or browser-relay work.
