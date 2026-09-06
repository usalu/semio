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

- `rustfmt --emit stdout` parse checks: green for engine, artifact, and Hub Rust sources.
- `git diff --check`: green for the scoped sources and task registration.
- `bun ./📜️script.ts document-mount-single-flight-check` from the OS Rust package: green, `AJV=1 cases=10 waiters=32 owner-futures=1`.
- Native receipt `U4wlLS/00` compiled and discovered the prior eight-law group; laws 1–5 were green and law 6 exposed a retained internal completion authority during shutdown. The bounded unlocked-fanout correction and reentrant-waker law below postdate that receipt, so a native rerun remains pending.
- Native receipt `RNAs9H/00` built and listed the then-current group; law 1 was green and law 2 timed out. A direct reproduction plus native process stack sample proved the test thread was blocked in `DatabaseDocumentMountOwner::drive` on the work mutex while the worker held that mutex across the deliberately gated mount poll.
- Root native receipt `WFy5o0/00`, executable SHA-256 `07ebac781d2f276c2c88b64563f31ba403ffb05ec5ef1fdcf8fbfe76e27d7956`, is GREEN for all nine registered laws at that boundary. The later coalesced driver, hard non-runnable witness, and WorkerPoolUse laws remain native-pending.
- Root native receipt `RIqL0s/00`, executable SHA-256 `f0a58d83f9579b4d05e9e8ac20056efd349a9100c6cfbf6b312c2bc5576d642e`, built/discovered the fourteen-law boundary and passed laws 1–8. Law 9 failed because the second join called `drive()` with controlled-resume authority and consumed the injected unlock failure before `Parked` could be observed. Current source changes mount callers to `request_drive(false)` and preserves `resume_requested` until work is actually `Parked`; this post-receipt correction is source-qualified only.
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

## Nonclaims

- This slice does not change presence, plan authorization, or SQLite recovery semantics.
- Native and Hub concurrent-hello qualification require terminal exact receipts.
