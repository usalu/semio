# Plugin-Host Lifecycle Residuals Audit

## Scope and evidence status

Read-only audit of the current plugin-host source and tests after the default-stack repair. I read `📓️sol-plugin-host-stack-overflow.md`, `📓️sol-plugin-host-test-api-drift.md`, the current `🖥️host/🦀️.rs`, `🖥️host/🧵️shard/🦀️.rs`, and the current master plan.

The prior broad run is the sole durable evidence for **234 discovered tests, 19 reported failures, then an abort**. It does not retain the 19 test names, so this report does not invent an exact one-to-one list. I did not launch Cargo: concurrent jobs are active and a broad rerun would neither be bounded nor distinguish the moving worktree from the recorded run. Current source already includes part of the stack packet's foreground `Pending`/`Blocked` wake repair, so historical relay parking is attributed separately from what remains demonstrably unsafe now.

The prior 23 compile errors were a completed **test API drift** packet (7 missing `ui_patch_receipt` fields, 15 obsolete close-event constructions, one obsolete open field). They are not part of these 19 runtime failures.

## Residual matrix

| Area | Current source evidence | Classification | Severity |
| --- | --- | --- | --- |
| Replay-owner destruction | `FixedReplaySeedPage`, `FixedReplaySeed`, `ReplaySpawnRefusal`, and `MountedReplaySeed` use `ManuallyDrop` and panic if non-terminal in `🧵️shard/🦀️.rs:408-722`. Release builds skip the assertions and consequently skip destruction of the manually dropped fields and counter release. | Production bug; process abort in debug/unwind and resource/accounting leak in release. | High |
| Replay failure transition | `drive_replay_seed` propagates checkpoint, ABI-admission, restore, UTF-8, and missing-instance errors directly from non-terminal phases (`:1096-1100`, `:1171-1205`, `:1216-1235`, `:1246-1249`). Only live/restart `start_job` failures explicitly move to `Closing`. | Production bug; a fault can leave a discoverable but non-terminal seed with retained owners and reserved ABI bytes. | High |
| Actor retirement | `unregister` removes instance/job maps but does not mark matching replay seeds or refusals for close (`:1346-1354`). Both `Effect::CancelJob` failure and `Payload::Cancel` failure call it (`:1708-1712`, `:1966-1970`). | Production bug; cleanup is deferred to a later drive at best, unsafe at teardown. | High |
| Mounted relay abandonment | Dropping `GuestRelayMountedFuture` only calls `request_close` (`🖥️host/🦀️.rs:4067-4072`). `pump_retirement` advances another slot with no waker and, on `Blocked`, cannot register one (`:3851-3892`). The foreground path now does register a waker (`:3959-3975`), but the orphan/background path does not own a continuing driver. | Production liveness bug; explains the recorded serial `background_cleanup_cancel_panic...` park. | High |
| Relay test cluster | 15 lifecycle tests cover pending worker release, cancel race/drop, panic/failure cleanup, concurrent cleanup-pending rejection, and poisoned slot recovery (`🖥️host/🦀️.rs:4534-4924`). The recorded 19 failures plausibly span this cluster, but no durable output allocates every failure to it. | Mixed: foreground wake repair may have changed some historical results; orphan-close liveness remains a production risk. | High/medium |
| Immediate-admission shard tests | `Effect::SpawnJob` now creates a `CaptureKind` replay seed (`🧵️shard/🦀️.rs:1675-1691`); activation of `running_jobs` is later, one replay opportunity at a time (`:1082-1165`). `pump` executes one authority/replay opportunity (`:1438-1518`). Six tests still assert same-pump start/step/cancel semantics at `:2405`, `:2498`, `:2526`, `:2643`, `:2686`, and `:2727`. | Test expectation drift, except that their premature scope exit exposes the real destructor defect. | Medium |
| Explicit mounted-seed laws | Existing laws already drive one close opportunity at a time and drain the seed (`:3460-3548`). | Correct model to reuse for migrated tests; coverage lacks arbitrary-error and scope-drop paths. | Low gap |

## First deterministic failure chain

The named final abort is deterministic from the current test and source; it does not require the 19-name log.

1. `cancel_job_effect_failure_retires_the_actor_and_surfaces_the_typed_fault` scripts `SpawnJob` and `CancelJob` in the *same* guest turn and sets `fail_next_cancel` (`🧵️shard/🦀️.rs:2526-2537`).
2. Spawn mounts a seed in `CaptureKind`; it is not yet `Retained` and does not yet occupy `running_jobs` (`:1679-1684`, `:676`).
3. The following cancel takes the deliberately pre-start branch, sets that seed to `Closing`, and does **not** call `runtime.cancel_job` (`:1694-1697`). This agrees with the neighbouring successful same-turn cancellation test, whose contract is “before it is ever stepped” (`:2496-2523`).
4. Therefore the first false assertion is `!shard.is_registered(actor)` at `:2545`: the actor remains registered. The expected cancellation admission, typed cancellation fault, and retirement are impossible for this fixture.
5. Assertion unwinding drops `ShardLoop`; its boxed replay-slot array drops the still-`Closing` `MountedReplaySeed`. It still owns its fixed seed, original kind, and original input, so `terminal_is_empty()` is false (`:693-703`).
6. `MountedReplaySeed::drop` executes `debug_assert!(false, "mounted replay seed requires discoverable incremental close")` (`:706-721`) while the assertion panic is active. Rust aborts on this second panic during unwinding.

That makes the primary test failure **expectation drift**, and the second panic/abort a separate **production destruction bug**. In a non-debug build the second panic is absent, but the `ManuallyDrop` fields are not dropped and the same path leaks owners. It is not safe to downgrade the defect to test-only.

## Shared root causes

### 1. Incremental ownership has no safe abandonment boundary

The design correctly wants one bounded page/owner release per opportunity, but it encoded this as “only a fully drained value may be dropped.” That contract is impossible to uphold on caller cancellation, test assertion unwind, allocation/guest faults, process shutdown, or a scheduler hand-off loss. The leaf page stores an admitted global page count; `MountedReplaySeed` stores a global ABI reservation. Skipping normal destruction also skips their decrements. A diagnostic assertion cannot replace ownership recovery.

### 2. Error paths are not uniformly converted into a close transition

Live/restart guest failures set `Closing`, but checkpoint, reserve, restore, and UTF-8 errors can escape while the same seed still owns pages, byte buffers, or checked-out data. Actor retirement similarly clears the routing maps before it records closure of the matching seed. The eventual “actor absent means closing” selection rule is useful (`:1063-1079`) but is not a disposal guarantee: it requires another budgeted drive and does nothing when the shard is being dropped.

### 3. Background cleanup is an ownership transfer, not a best-effort future poll

The stack repair correctly stops self-waking a genuinely blocked foreground close and registers the owner waker there. The orphaned future path has no equivalent durable reaper: `request_close` changes state, while a later unrelated registry poll may make one close step. If that step is `Blocked`, `pump_retirement(..., None)` cannot bind a wake. This leaves a leased/cleanup-pending instance without a progress owner and can stall the next route. Treating `Leased` as a normal admission preflight state also makes a caller wait behind the gate instead of returning a bounded busy/cleanup diagnostic.

### 4. Several assertions predate replay admission, not merely the actor wire update

The old tests assume that a spawn both invokes `start_job` and enters `running_jobs` in the originating `pump`. The current phase machine deliberately captures kind/input/checkpoint, retires the original checkpoint, starts, records authority/turn/running/placement, then reaches `Retained`; each is a separate bounded opportunity. Tests for completion, payload cancel, failed cancel, and placement must first drive or construct a retained seed. They must not restore synchronous production behavior to satisfy obsolete assertions.

## Production bugs versus test drift

| Item | Verdict | Required resolution |
| --- | --- | --- |
| Same-turn spawn + cancel should invoke a failing guest cancel | Test drift | Keep pre-start cancellation local. Rewrite the failure test to first reach `Retained`, then issue a later `Effect::CancelJob`. |
| Same-pump job step / placement assertions | Test drift | Drive the bounded seed protocol to `Retained`; obtain the admitted authority from the seed rather than reusing arbitrary test turns. |
| Any non-terminal replay/refusal/page `Drop` can panic or leak | Production bug | Make normal drop safe and exact even if cooperative retirement did not run. |
| Replay errors/actor unregister can abandon a mounted seed | Production bug | Centralize failure-to-closing transition, preserve exactly-once fault reporting, and schedule bounded cleanup. |
| Dropped mounted relay can remain blocked with no waker | Production bug | Transfer it to a registry-owned bounded reaper that owns progress/wake registration. |
| Historic 19-test allocation to individual rows | Unproven | Re-run focused groups only after the preceding fixes; retain the result under ticket `🗑️generated` during implementation. |

## Dependency-ordered implementation packet

### P0 — Make replay destruction fail-safe before changing test timing

Own `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🦀️.rs` and its existing test module.

- Replace the `ManuallyDrop` ownership fields in `FixedReplaySeedPage`, `FixedReplaySeed`, `ReplaySpawnRefusal`, and `MountedReplaySeed` with ordinary `Option`/array ownership, retaining the existing explicit bounded `close_one` protocol for normal scheduling.
- Make destruction non-panicking. A page dropped with storage still present must release storage and decrement `JOB_REPLAY_SEED_PAGES` exactly once; a mounted seed must release any remaining ABI reservation exactly once. Do not condition a release-build destructor on a debug assertion.
- Add phase-by-phase abandonment laws: drop each object before/after each close frontier, including a rejected seed. Assert no unwind abort, process page/ABI counters return to their entry values, and no owner is claimed twice. Preserve the existing sub-8-ms, one-owner close laws.

### P1 — Make every replay/retirement fault closeable and observable once

Continue in `🧵️shard/🦀️.rs`.

- Add one internal transition that records a typed replay failure, makes the selected seed closeable before returning the fault, and prevents duplicate terminal publication. Use it for checkpoint, allocation, restore, UTF-8, live-start, restart, and missing-instance paths.
- Have `unregister`, `cancel_one`, and failed `Effect::CancelJob` mark every matching mounted seed/refusal for bounded retirement before discarding routing maps. Suppress completion publication to an actor that is already terminally retired.
- Keep the close budget rule: one owner/page or bounded byte slice per opportunity, no run-to-completion cleanup. But ordinary destruction remains the last-resort accounting-safe boundary from P0.
- Add focused laws for each injected guest failure and actor retirement. Assert one external fault, no later `step_job`, no ABI/page counter leak, and eventual slot removal under finite grants.

### P2 — Give dropped mounted relays a durable bounded reaper

Own `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️.rs` plus `guest_cold_relay_tests`.

- On `GuestRelayMountedFuture::drop`, transfer the slot to a registry-owned background retirement driver instead of only setting `close_requested`. It must schedule exactly one close opportunity, requeue only after bounded `Pending`, and register a persistent reaper waker on `Blocked`.
- Keep foreground `pump` one-opportunity behavior. Coordinate foreground and reaper claims by slot generation so there is one closer and no double `begin_close`/cancel admission.
- Reject a preflight `Leased` instance with a typed bounded busy/cleanup diagnostic; never make a new mounted request wait indefinitely for an abandoned prior request.
- Make `GuestColdRelayJob` destruction non-panicking and ensure the reaper owns completion/cancel work before a session can disappear.
- Add one-worker laws for: dropped pending step; start/step panic then successful cleanup; cancel panic/failure quarantine; `Blocked` reaper wake without spin; no duplicate cancel; competing route progress; generation-stale close no-op; and every slot/permit returning to a stable available or quarantined state.

### P3 — Migrate only stale shard tests to the retained protocol

Own the existing shard tests in `🧵️shard/🦀️.rs` after P0/P1 are green.

- Replace the synchronous setup in the six identified tests with a test helper that drives a bounded seed to `Retained` under explicit grants, returning its real authority/request.
- Keep a separate law for same-turn spawn/cancel: it must make no `cancel_job` or `step_job` admission, then explicitly drain the `Closing` seed before test exit.
- Rebuild cancellation-failure and placement tests around a later event/turn after retention. Verify order, typed fault, and exact counter baseline rather than pump counts inherited from the deleted synchronous path.

## Focused verification sequence

Run these only after the owning changes and with an isolated target; none are claimed as run by this audit.

```sh
TICKET_TARGET="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🗑️generated/plugin-host-lifecycle-target"
RUSTFLAGS='-Awarnings' CARGO_TARGET_DIR="$TICKET_TARGET" bun nx run @semio-tech/framework-plugin-host:check --skip-nx-cache

RUSTFLAGS='-Awarnings' CARGO_TARGET_DIR="$TICKET_TARGET" bun nx run @semio-tech/framework-plugin-host:test --skip-nx-cache -- mounted_replay
RUSTFLAGS='-Awarnings' CARGO_TARGET_DIR="$TICKET_TARGET" bun nx run @semio-tech/framework-plugin-host:test --skip-nx-cache -- cancel_job_effect_failure_retires_the_actor_and_surfaces_the_typed_fault
RUSTFLAGS='-Awarnings' CARGO_TARGET_DIR="$TICKET_TARGET" bun nx run @semio-tech/framework-plugin-host:test --skip-nx-cache -- guest_cold_relay_tests::background_cleanup_cancel_panic_quarantines_before_the_next_mounted_route
RUSTFLAGS='-Awarnings' CARGO_TARGET_DIR="$TICKET_TARGET" bun nx run @semio-tech/framework-plugin-host:test --skip-nx-cache -- guest_cold_relay_tests::drop_cleanup_cancel_failure_quarantines_before_the_next_mounted_route
RUSTFLAGS='-Awarnings' CARGO_TARGET_DIR="$TICKET_TARGET" bun nx run @semio-tech/framework-plugin-host:test --skip-nx-cache -- guest_cold_relay_tests::concurrent_route_rejects
```

After those pass, run the remaining shard lifecycle filters, then reserve one explicit full-suite window. Do not use a full-suite result to claim P2-A2, P2-B, or P2-C composition: P2-A2's adapter currently tops out at 496 KiB per blob versus the 64 MiB authority pair; P2-B's publisher/projections exist, but the trusted catalog loader and typed lag rebootstrap are separate active work. The plugin-host defect matters to those paths only as a host reliability prerequisite, not as evidence that the hub authority or rebootstrap is composed.

## Sharpest blocker

**High: `MountedReplaySeed` is deliberately non-droppable outside its cooperative close protocol.** The final test’s first failure is stale expectation, but its destructor then double-panics and aborts; the release configuration leaks the same owners/counters. Repair this fail-safe ownership boundary before interpreting the other 19 lifecycle outcomes or relying on plugin-host work as a prerequisite for trusted headless hub composition.
