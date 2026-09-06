# DB Backend Registration Rollback

## Boundary

DB backend registration now reserves one fixed rejected-backend slot before it consumes an executor, owner operation, owner credit, or `WorkerPoolUse`. The public raw registration API returns `DbIoBackendRegistrationRejected`, which retains either the exact unregistered executor and pool or the exact committed retirement cursor. Production Memory, Filesystem, SQLite, Postgres, and Neo4j constructors expose `DbStorageOpenRejected`; only its `BeforeExecutor` variant is owner-free.

Post-admission failure commits the executor, operation, credit, pool, and pool use into the reserved rollback slot. That slot drives the backend close, returns the ledger credit, and releases the pool use only at terminal retirement. Registration no longer relies on the separately saturable lost-owner primary, overflow, or quarantine rings.

## Laws

The neutral backend-pool-use fixture has nine exact cases, including pre-transfer rollback reservation, full rollback and lost-owner tiers returning the exact executor, committed rollback retaining one pool use until terminal close, and post-transfer prepared registration refusal returning an exact close owner. `db_io_backend_registration_saturation_returns_exact_executor_before_pool_use` fills both fixed retirement tiers, observes the typed retained executor, frees capacity, retries that same owner, and proves the registered backend owns exactly one pool use until terminal close. `db_io_prepared_registration_failure_returns_exact_close_owner_after_submission_refusal` fills the I/O lane, forces a post-transfer registry refusal, observes one retained pool use, then drives the same rejection to its original cause and exact ledger/pool terminal witness.

## Receipts

- `bun ./📜️script.ts nx run @semio-tech/framework-os-kernel:wal-writer-authority-check --skip-nx-cache`: GREEN, `AJV=6`, `backend-pool-use=9`.
- `NX_ISOLATE_PLUGINS=false bun x nx run @semio-tech/framework-os-kernel:document-mount-single-flight-check --skip-nx-cache`: GREEN, `AJV=1`, `cases=10`.
- `rustfmt --edition 2021 --emit stdout` parsed the touched Storage, SQLite, Postgres, Neo4j, Artifact, and Engine Rust sources.

Native qualification remains pending root's exclusive warm-cache run. The report does not claim that dropping an undriven `DbStorageOpenRejected` completes cleanup or that all lower raw DB I/O escape APIs are globally guarded.

Root's `BWGgrn` writer run built and passed exact laws 0 through 30, then timed
out in `db_io_backend_registration_saturation_returns_exact_executor_before_pool_use`.
The timeout was localized to fixture teardown: each nonterminal fault sentinel
was dropped while its own lost-owner mutex remained locked, and `DbIoFault::drop`
therefore tried to re-enter that mutex to park itself. Teardown now closes every
exact sentinel to terminal before removing it from its tier. The independent
writer source gate remains GREEN (`AJV=6`, `backend-pool-use=9`); a post-fix
native rerun is still required.

Root post-fix receipt `seKuJE` is GREEN for all 37 exact writer laws, executable
SHA-256 `0ae537c57eb3eb68306d2e808eaad4d9db2817e6f779f0c8909cfc147d53c7bd`.
This qualifies the all-tier registration saturation and prepared-constructor
rollback laws, not the separate ArtifactAuthority retirement or document-mount
lifecycle.

Terra's current read-only review found the prepared rollback/rejection ownership
coherent at source: reservation commit retains executor, operation, credit, pool,
and pool use, while `retry_close` retains that same owner across incomplete or
faulted cleanup. This review is not a native receipt.

## Typed constructor boundary

The production prepared helper now returns the exact
`DbIoBackendRegistrationRejected` instead of mapping an admitted owner to
`DbError`. Memory, Filesystem, SQLite, Postgres, and Neo4j return one
`DbStorageOpenRejected`: `BeforeExecutor` is owner-free, `Registration` retains
the rollback cursor, and `Registered` retains the exact backend control after a
failed `BackendOpen`. `retry_close` returns the original cause only after the
retained owner reaches its terminal acknowledgement; cleanup faults preserve
the rejection for another controlled attempt.

`Database::open_at` propagates `DatabaseOpenAtRejected`, retaining either the
storage-open rejection or the concrete filesystem backend if later database
construction fails. CLI boundaries drive these typed rejections to terminal
before rendering their `DbError`; no compatibility conversion or `Display`
flattening was added. This source boundary is parse/oracle-qualified and awaits
native constructor-fault qualification.
