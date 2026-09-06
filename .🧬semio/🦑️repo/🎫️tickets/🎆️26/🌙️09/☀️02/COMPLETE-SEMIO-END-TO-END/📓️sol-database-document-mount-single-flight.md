# Database Document Mount Single Flight

## Outcome

`Database` now owns one generation-stamped retained mount operation per document. `ensure_document`, `create_document`, and `document` all pass through the same `Vacant -> Opening -> Ready` registry. The first caller installs the owner before any catalog or WAL await; later callers join one of 32 fixed waiter slots.

The mount operation future is retained by `DatabaseDocumentMountOwner` and is polled on the injected `WorkerPool`. Dropping a request removes only its waiter. Catalog publication, authority construction, exactly-once lifecycle emission, and typed WAL/engine rejection cleanup remain owned by the mount operation. `Emit` and its `EventSink` substrate now promise a `Send` future, so the retained owner awaits emission before it publishes `Ready` or fans out handles. Fanout is materialized in a fixed 32-slot array while the registry is locked; the owner drops its internal completion authority, unlocks the registry, and only then invokes arbitrary waiter wakers. A failed writer unlock moves the exact rejection and same-owner open continuation into `Parked`; it is not busy-retried. A join only requests an ordinary poll and cannot authorize another unlock attempt. An uncancelled shutdown step explicitly requests one controlled resume, and that request is consumed only after the owner is already `Parked`. A matching generation alone may replace `Opening` with `Ready` or remove a failed slot.

The owner driver is now an exact `Idle -> Queued -> Polling` coalescer. Join/wake/resume calls never contend on the work mutex and at most one weak-captured poll job is queued. A racing resume is recorded until the active poll consumes it. A transient `Contended`/`Saturated` submission retains its exact job behind one timer; a hard `Shutdown`/`Poisoned` refusal instead enters `NonRunnable`, retains the job without a timer, and makes database shutdown report an executor block rather than a false-progress loop.

Hub now calls `Database::ensure_document` directly. It drives typed rejection cleanup to terminal before returning a `DbError`, so a storage error frame cannot precede retained writer cleanup.

## Scope

- `db/engine`: fixed-capacity single-flight registry, retained owner scheduler, waiter cancellation, common create/open/ensure policy, catalog-loser refresh, owner-ordered lifecycle emission, unlocked bounded fanout, non-blocking active-owner drive requests, controlled retained cleanup, hard scheduler witness, shutdown participation, and fourteen exact native laws.
- `db/version-graph` and `db/observe`: explicit `Send` futures for lifecycle emission and event-sink writes; audit checksum folding is synchronous before retained state mutation.
- `Hub`: removal of the document/create TOCTOU helper.
- schema/neutral fixture: ten state traces, exact one owner future, fixed 32 waiters plus 33rd refusal/reuse, owner-ordered emission, cancellation, retained close fault, shutdown interruption, generation retry, unlock-before-wake ordering, and a join while the owner poll is active.
- registered Bun/Nx source and exact-native targets plus launch seeds 411.086/411.087.

## Verification

The latest native receipt `0Rcb2U/00` reached eleven mount laws GREEN and then
exposed a retained-use handoff after the document authority was published. The
completed catalog transaction still kept its independently acquired
`WorkerPoolUse` inside the worker's final `Arc<DatabaseCreateCatalogState>`.
That is not a second long-lived database owner: the result acknowledgement now
takes the optional catalog-operation use in `release_success`, and terminal
retirement takes it on the cancellation/error path, before mount fanout.

The subsequent terminal-runner audit also found that a strong retained Job was
an owner but not an exclusive state. `ArtifactRunnerDriver` is now one packed
`RunnableIdle | Queued | Polling | Parked | ClosingReady | ClosingPolling |
ClosingParked | Terminal` latch. Ordinary work can
only enter `RunnableIdle -> Queued`; terminal placement owns `Queued/Idle ->
Parked`; checking the Job out leaves `Parked`; resume writes `Queued` before
pool admission; and close owns `Parked -> Closing`. The exact
`artifact_runner_terminal_authority_latch_preserves_external_job_and_one_resume`
law covers checkout, a concurrent ordinary admission refusal, Drop handback,
and one resumed turn. The external terminal close API now returns
`Result<(), ArtifactRunnerTerminalJob>`: a pending History close returns the
same strong cursor to its caller, `ClosingParked` forbids poll-before-wake, and
the retained waker promotes exactly once to `ClosingReady`. The exact
`artifact_runner_terminal_close_returns_exact_cursor_until_retained_wake` law
covers the authority-dropped path and proves that an intervening close call
does not repoll. These corrections postdate every native receipt above and
remain source-qualified pending the next exact run.

The earlier `GaiJEI/00` coalescing failure was a fixture admission error, not a
production starvation result: one-shot `try_submit` treated legitimate queue
contention as terminal. The release is now retained through `submit_at(now)`;
the fixture still requires a real UserVisible worker, so the rival-poller
deadlock continues to time out rather than being hidden.

- `rustfmt --emit stdout` parse checks: green for engine, artifact, and Hub Rust sources.
- `git diff --check`: green for the scoped sources and task registration.
- `bun ./📜️script.ts document-mount-single-flight-check` from the OS Rust package: green, `AJV=1 cases=10 waiters=32 owner-futures=1`.
- Native receipt `U4wlLS/00` compiled and discovered the prior eight-law group; laws 1–5 were green and law 6 exposed a retained internal completion authority during shutdown. The bounded unlocked-fanout correction and reentrant-waker law below postdate that receipt, so a native rerun remains pending.
- Native receipt `RNAs9H/00` built and listed the then-current group; law 1 was green and law 2 timed out. A direct reproduction plus native process stack sample proved the test thread was blocked in `DatabaseDocumentMountOwner::drive` on the work mutex while the worker held that mutex across the deliberately gated mount poll.
- Root native receipt `WFy5o0/00`, executable SHA-256 `07ebac781d2f276c2c88b64563f31ba403ffb05ec5ef1fdcf8fbfe76e27d7956`, is GREEN for all nine registered laws at that boundary. The later coalesced driver, hard non-runnable witness, and WorkerPoolUse laws remain native-pending.
- Root native receipts `RIqL0s/00` and `uv5IaM/00` (executable SHA-256 `f0a58d83f9579b4d05e9e8ac20056efd349a9100c6cfbf6b312c2bc5576d642e`) built/discovered the fourteen-law boundary and passed laws 1–8 before law 9 failed to observe `Parked`. A correlated writer receipt, `yDnbws`, proved the lower ownership defect: `WalWriterPermit::release` requested backend retirement before the retained rejection reached its caller, so exclusivity could disappear before explicit cleanup. Current source transfers a dormant `WalWriterRelease`; its first poll begins retirement, while dropping either the permit or an unpolled release still starts nonblocking cleanup. Law 9 now waits on a post-`Parked`, post-work-lock test witness and verifies the exact injected unlock error rather than racing a fixed yield count. This correction is source-qualified only.
- Root native receipt `bw5a5F/00` passed the first eleven mount laws, then exposed a test-only one-shot mandatory submission during the pool-use liveness probe. The probe now retains that exact job through `submit_at(now)`; it no longer treats legitimate queue contention as a hard scheduler failure. The corrected law 12 and the three terminal-runner laws below remain native-pending.
- Root native receipt `gfGodw/00` is GREEN for all seventeen registered laws at the current terminal-cursor boundary; executable SHA-256 `fc3caa3df81b904c9cabdc6d6da02baff4997b2c7d5c9bbf85cb0fb6a6fa9157`. It qualifies the packed state transition laws and explicit external close cursor. It does not yet substitute for an end-to-end gated `HistoryReplayFuture` terminal-owner law.
- Root receipt `t7DuS8` failed the first current mount law after database
  shutdown returned but `WorkerPool::shutdown` still observed one retained use.
  `Database::shutdown_step` had consumed its own pool-use clone without proving
  that it was the final clone; a completing mount/open handoff could still be
  releasing a transient clone on its worker. Shutdown now reports a distinct
  `PoolUse` progress phase until the use is uniquely database-owned, then
  consumes it and returns `Complete`. This terminal-ACK correction remains
  native-pending.
- Root receipt `tE5UHu/00` compiled that strong-count fence but again exposed
  `Busy { retained_uses: 1 }` after the Database had returned terminal ACK.
  The remaining owner was outside that `Arc`: Database construction and live
  catalog publication still called `acquire_use()` for capability, catalog
  read/bootstrap, and create-catalog operations. Their retained states could
  therefore outlive the Database-owned use without being visible to the
  terminal strong-count fence. Database-owned paths now pass the existing
  `Arc<WorkerPoolUse>` through private `try_prepare_with_use` boundaries; the
  standalone retained APIs still acquire their own exact use. The source
  oracle requires exactly one acquisition in `open_with` and checks every
  internal handoff. Create-catalog also performs an idempotent release check
  after its physical driver relinquishes poll authority, closing the
  completion-consumption/worker-tail interleaving. This correction postdates
  `tE5UHu` and remains native-pending.
- Current registered source receipt is GREEN with `AJV=1 cases=10 waiters=32 owner-futures=1`; the strict driver fixture now also contains three neutral interleavings, including hard-refusal retention.
- Terra's independent current-source review found the coalesced sole-poller, generation, join, and retained-refusal flow coherent. Its one P0 finding was the post-terminal catalog/sync/checkpoint escape; the current source fences those calls through `require_open_use` and preserves typed retained rejections. This correction is source-qualified and remains native-pending with laws 12–14.

## Exact Native Laws

1. `db_engine::tests::database_concurrent_ensure_mounts_one_actor_and_one_writer`
2. `db_engine::tests::database_published_opening_joins_without_actor_overwrite`
3. `db_engine::tests::database_cancelled_ensure_waiter_does_not_cancel_mount_owner`
4. `db_engine::tests::database_document_mount_failure_waiters_share_terminal_cleanup_and_retry_generation`
5. `db_engine::tests::database_mount_owner_emits_before_ready_and_survives_elected_waiter_cancellation`
6. `db_engine::tests::database_mount_waiter_capacity_rejects_33_and_reuses_one_cancelled_slot`
7. `db_engine::tests::database_document_mount_fanout_wakes_only_after_registry_unlock_and_internal_owner_handoff`
8. `db_engine::tests::database_shutdown_interrupt_retains_waiterless_opening_owner_until_ready`
9. `db_engine::tests::database_document_mount_unlock_fault_parks_exact_owner_until_controlled_shutdown_resume`
10. `db_engine::tests::database_document_mount_coalesces_join_drives_without_shared_pool_starvation`
11. `db_engine::tests::database_document_mount_cleanup_fault_consumes_racing_resume_request_exactly_once`
12. `db_engine::tests::database_worker_pool_use_blocks_early_shutdown_and_releases_at_terminal_ack`
13. `db_engine::tests::database_worker_pool_use_is_admitted_before_the_first_storage_probe`
14. `db_engine::tests::database_document_mount_hard_scheduler_fault_retains_nonrunnable_job_without_retry_timer`
15. `db_artifact::tests::artifact_runner_terminal_authority_latch_preserves_external_job_and_one_resume`
16. `db_artifact::tests::artifact_runner_closing_poll_waits_for_retained_wake_before_next_turn`
17. `db_artifact::tests::artifact_runner_terminal_close_returns_exact_cursor_until_retained_wake`
18. `db_artifact::tests::artifact_runner_terminal_resume_refusal_returns_exact_cursor_for_close`
19. `db_artifact::tests::artifact_authority_drop_transfers_parked_terminal_job_to_registered_close_owner`
20. `db_artifact::tests::artifact_runner_retirement_panic_retains_exact_cursor_until_explicit_retry`
21. `db_artifact::tests::artifact_authority_drop_reuses_registered_retirement_slot_beyond_capacity`

## Nonclaims

- This slice does not change presence, plan authorization, or SQLite recovery semantics.
- Native and Hub concurrent-hello qualification require terminal exact receipts.
- The external terminal resume cursor is source-qualified: `resume(self)` now
  returns `Result<(), Self>` and restores the exact job on pool refusal. Its
  eighteenth exact law forces refusal, observes zero turns, and closes the same
  cursor to terminal. A bare `ArtifactAuthority` now pre-admits a fixed
  retirement slot and WorkerPool maintenance hook before spawning its runner;
  Drop transfers its strong close owner, exact handoff, and pool use into that
  slot. Law 19 parks an internal terminal job, drops the authority without
  checking it out, and requires pool shutdown plus an empty retirement registry.
  This later owner path is source-qualified only; `gfGodw/00` predates it.
  The pool callback now returns `WorkerMaintenanceStep::Retire` so the exact hook
  is removed after its running invocation returns. Law 20 proves a panicking
  close restores the exact cursor/generation/pool use for explicit retry. Law 21 reuses that registered
  path 65 times, beyond its fixed 64-slot capacity; native qualification remains
  pending.
