# Plugin-Host Lifecycle Repair

Date: 2026-09-03  
Ticket: `26/09/02/COMPLETE-SEMIO-END-TO-END`

## Dependency order

The implementation follows the live-tree audit order: drop-safe replay ownership and exact accounting; one idempotent replay failure-to-close funnel and immediate actor-owner retirement; a registry-owned detached relay reaper; then migration of only the source-audited synchronous test expectations. The existing heap-backed capacities remain unchanged.

## Initial evidence

The first cold focused attempt, session `8386`, reached 19 compile diagnostics while the lifecycle conversion was incomplete. No semantic test ran. The immediate restoration gate used the isolated ticket target, disabled compiler wrappers, and ran the full plugin-host library test binary with `--no-run`. Session `10048` was green, exit `0`, with zero diagnostics in 1m52s.

The first production ownership law then passed directly from that exact binary:

```text
component::shard::tests::replay_owners_drop_safely_from_every_owned_frontier_and_balance_accounting
1 passed; 0 failed; 234 filtered; 0.19s
```

It drops replay owners from representative CaptureInput, Retained, and Closing frontiers plus a spawn refusal under `catch_unwind`, and proves page/ABI process counters return to their entry baselines.

The original stale same-turn cancellation test was deliberately rerun before changing its expectation. It failed cleanly at the first semantic assertion in 0.71s:

```text
a failed hot-shard cancellation must retire the uncertain actor instance
```

There was no `MountedReplaySeed` destructor panic, second unwind, SIGABRT, or leaked-counter diagnostic. This separates the obsolete same-turn guest-cancel expectation from the repaired production destruction boundary.

## P1 replay close funnel and audited expectation migration

The mounted replay owner now records one first-writer close reason and enters one idempotent `Closing` transition before releasing maps or reporting a failure. Capture-page refusal, checkpoint/start/restore/restart failure, ABI admission, malformed UTF-8, missing actor, retained step authority/placement/sequence faults, guest step/terminal-checkpoint failure, normal completion, cancellation, and unregister all use that transition. Scheduled close still releases at most one page or owner per maintenance opportunity; ordinary `Drop` remains the safe unwind fallback.

Exactly six source-audited tests were migrated to admitted replay authority rather than synthetic same-turn stepping: spawn/multi-step, pre-retention effect cancellation, retained effect cancellation failure, payload cancellation success, payload cancellation failure, and exclusive-before-inline placement. Session `51006` used the current-source binary SHA-256 `6bd88f7dc4f2348ee36d8db5c55abfbf015f99ab598b1a887b9f823ea5777d15`: P0, first-reason/unregister, spawn/multi-step, and pre-retention effect cancellation passed; retained effect cancellation then exposed one stale post-retirement pump count (`1` consumed rejected authority versus `0`) with all typed-fault, actor-retirement, no-step, and close-reason assertions already holding. Only that count was corrected to encode the consumed-without-guest-execution contract.

Absolute-target compile session `72371` was green with zero diagnostics in 3m16s. The full eight-law rerun in session `50447` passed the first seven laws and exposed one genuine placement transition gap: after both jobs reached `Retained`, no explicit `JobStep` authority had been granted. The repair now admits both exact retained authorities in one `Grant` and selects `Exclusive` only within the consecutive same-actor JobStep frontier. A separate law proves this selection cannot cross a Cancel/lifecycle barrier. Compile session `53258` ended before source diagnostics because another process deleted its target directory while Cargo was writing `invoked.timestamp`; it is recorded as external generated-target deletion, not a source result. All subsequent commands use the unique absolute ticket target `plugin-host-lifecycle-sol-target`.

The unique-target current-source compile in session `88510` was green with zero diagnostics in 14m02s. Exact-binary session `54004` then passed all twelve focused native laws: P0/P1 `9/9` and detached-relay P2 `3/3`. The pending-drop law used a real gated guest step and reached an empty registry slot with exactly one guest cancel admission in 1.42s without a second foreground registry poll. The other P2 laws prove one-slot/one-opportunity rotating reclamation, max-plus-one/stale-generation refusal, and that a detached reaper cannot steal exact output from `DrainingForCaller`.

The neutral `relay-lifecycle` JSON Schema and five literal traces are consumed by the native production laws and by Rust/Gherkin and TypeScript adapters. The independent Bun/AJV state-machine oracle passed `5/5`. The existing plugin-host `📜️script.ts` now owns `lifecycle-check`; `📋️project.json` delegates the Nx target to it, and the launch seed registers the ordered gate. Generated launch freshness is verified after the broad suite so concurrent registry generation is not overwritten mid-run.

## Full-suite baseline and final-audit repair

Pre-audit broad session `78722` discovered 240 library tests and reached one unrelated effects-capability failure before entering `background_cleanup_cancel_panic_quarantines_before_the_next_mounted_route`. It then ended at the exact 900-second alarm with exit `142`; no later outcome was observed. This is retained only as a RED baseline, not as a full-suite result.

The final audit found that the original relay trace adapter reconstructed lifecycle state rather than driving production, and that a blocked detached session armed a timer despite registering a real completion wake. Both source defects are now superseded:

- `WorkerJobSession::register_close_wake` replaces stale outer wakes by delegating to the exact job-owned close source. `GuestColdRelayJob` registers its relay completion slot; wake-capable reaping parks without a timer. Timer fallback is reserved for wake-incapable owners and is coalesced at eight milliseconds.
- The pending-drop law observes one registered close wake, zero timers, and zero additional reaper polls across eight maintenance barriers before the actual guest completion wake. It then requires a finite wake-driven retirement. A separate law proves two fallback requests create one bounded timer.
- The Rust trace subject calls a narrow production seam. Replay events use actual `MountedReplaySeed` close transitions. Abandoned relay events use the real mounted registry, detached reaper, and a controllable `WorkerJobSession`. The live-output trace reaches `DrainingForCaller` through `try_step_on_caller`, terminal checkout, retained output and `finish_outcome`; its competing `reap-other` event mounts and reclaims a second detached owner before the caller receives the exact output. Stale-generation and capacity traces use production registry operations.
- The protocol-v2 Gherkin case dispatches its literal doc-string ids through that Rust subject. `lifecycle-check` now runs the independent Bun/AJV `5/5` oracle and the repository test host's Rust subject before fourteen focused laws, the full library suite and an all-feature check.

Production-only check session `73491` reached the lifecycle source and found one local missing probe-wake method; that was corrected. The following current-source session `44939` stopped before the host crate because a concurrently renamed UI contract include requested a nonexistent `🧬️🧬️typed/🦀️.rs`. The cfg(test) compile session `74804` likewise stopped on a concurrently renamed OS-config test-module path. These are external taxonomy-path blockers, not passing lifecycle evidence. Repo-test contract discovery is also globally RED (828 pre-existing/current concurrent breaches); the selected lifecycle-specific breach is the current semantic-icon case directory not satisfying the platform's kebab-only check. The registered subject phase does not run that global contract phase.

Selected repo-test subject session `84539` proved that the permanent command resolves the `♻️relay-lifecycle` Rust adapter and generated host, then exhausted the repository runner's default fifteen-second fundamental budget during its cold Cargo build before a source diagnostic. The owned script therefore sets both the child test budget and command budget to a bounded 900 seconds, matching the observed fourteen-minute clean unique-target build. This changes no test semantics.

Current-source session `31044` again stopped before the plugin host at the unchanged external UI include (`📋️copy/🦀️.rs:236`, missing `../🧬️typed/🦀️.rs`), with Cargo status `101`. The independent oracle executed afterward and remained green `5/5`. The session supplies no post-audit lifecycle compile credit. An owned-path `git diff --check` is clean.

## Current claim boundary

Registered lifecycle session `7067` advanced beyond the earlier boundary: the independent oracle passed `5/5`, the production subject passed `5/5`, exact preflight resolved every one of the fourteen then-registered laws, and all fourteen exact laws passed. The following parallel 242-test library phase remained RED. It reported independent shard checkpoint/unregister failures, two concurrent cleanup failures, and five guest panic/failure cleanup tests parked beyond sixty seconds; the process was stopped after those hard failures, so the all-feature stage was not reached.

The first parked law, `background_cleanup_cancel_panic_quarantines_before_the_next_mounted_route`, reproduced alone. Its ticket-local three-second sample found the test owner parked while the worker pool was idle, and the runtime emitted `WorkerPool: mandatory submission failed closed: Contended`. The retained relay future used mandatory admission from its wake path; that panic occurred outside its future-poll unwind guard and left `scheduled=true`, permanently losing the task owner. The production repair now retains the task across `Contended`/`Saturated`, arms one coalesced timer-wheel callback without bypassing queue admission, and explicitly terminalizes `Shutdown`/`Poisoned` through the failure continuation. A deterministic private one-worker saturated-queue plus shutdown law is registered in the permanent gate. Its current compile/runtime result and the original isolated panic-cleanup rerun are still pending, so lifecycle acceptance remains partial. The unclosed worker pools in the mounted fixed-replay test and the unrelated shard failures remain separate broad-suite residuals.

## Current scheduler and broad-suite repair

The retained compute scheduler now distinguishes transient outer-pool and inner mounted-job contention from terminal shutdown/poison. It retains the exact closure through one bounded timer-wheel retry, delivers `WorkerLost` on terminal refusal, closes the retained result/session under bounded work, and leaves final owner retirement to `WorkerJobSession` drop. Production `ComputePool::run_job` is mounted by the router rather than bypassed by tests. Exact service and router laws prove successful retained execution and stopped-pool failure release the originating owner.

The shard executor now owns a bounded lane-aware ingress ring. A sixth neutral fixture case submits an earlier background actor and a later interactive actor, limits each drive to one frame, and requires interactive-before-background selection without draining an unbounded queue. The independent Bun/AJV oracle passes `6/6`; the Rust executor law consumes the same fixture and passes. Replay tests that inspect process-global ownership counters are serialized by a test-only RAII authority, eliminating cross-test false races without changing production caps or accounting.

Current direct evidence on the source preceding the registered rerun is:

- all 24 guest-cold relay laws pass serially;
- the stopped compute-pool service law passes `1/1` through its registered services target;
- both router compute laws pass `2/2`;
- the full plugin-host library suite passes `244`, fails `0`, ignores `1` under both serial and default-parallel execution (`13.21 s` and `6.22 s` respectively); and
- the owned diff check is clean.

The permanent lifecycle gate now contains nineteen exact laws: the original replay/relay boundaries plus actor-owned budget, FIFO ingress, successful router compute, and stopped-pool owner release. It performs one complete test inventory and requires exactly one fully qualified match for every suffix before any exact run, avoiding nineteen redundant cold inventories. The launch entry uses the repository root `📜️script.ts nx` router, which disables Nx plugin-isolation IPC; raw isolated Nx corrupts long emoji project roots and is not the registered execution path. Registered session `74785` has passed the independent `6/6` oracle, repository production subject `5/5`, and exact-one discovery for all nineteen laws. Its exact laws, broad suite, and all-feature terminal are still running, so this section does not yet upgrade lifecycle acceptance.
