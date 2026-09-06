# Database Retained-Use Shutdown Audit

Read-only current-source audit for the `tE5UHu` retained-use repair. No build was run.

## Verdict

The new `DatabaseCreateCatalogState.pool_use: Mutex<Option<Arc<WorkerPoolUse>>>` is the right shape: the successful create-catalog result itself retains `Arc<DatabaseCreateCatalogState>`, so an immutable `_pool_use` would survive normal result ownership. Its current success postamble does take that exact clone. However, two P0 lifetime holes remain before `DatabaseShutdownProgress::Complete` can mean that the database has no concealed pool-use work:

1. `Database::hello_retained` checks the database guard, drops that clone immediately, and then makes `DatabaseSyncHelloFuture` acquire a different pool-use. A live hello future/session is therefore not visible to the database shutdown strong-count fence.
2. `DatabaseCreateCatalogState::release_success` may run while its public completion is still owned by the unresolved future. It marks the state finished before result Drop can schedule retained retirement, creating an unreachable self-cycle on the dropped result path.

`Complete` currently means only that the database's own `pool_use` has no clones. It does **not** mean its backend registration's separate pool-use was closed: `Database::shutdown_step` has no storage-close phase, while `open_at` installs `FsStorage` on that same pool.

## Current Call Graph And Pool-Use Census

| Source | Acquisition | Reached from an open `Database`? | Current lifetime / verdict |
|---|---|---|---|
| `⚙️engine/🦀️.rs:8067` `Database::open_with` | One database root guard | Yes, construction | Stored in `Database.pool_use`; its clones are the shutdown fence at `:8489-93`. Correct baseline. |
| `:883` `DatabaseCapabilityOpenFuture::try_prepare` | New use | No instance path; public `open_retained` static wrapper at `:8052` uses it | `open_with` correctly calls `try_prepare_with_use` at `:8068`, sharing the root guard. Standalone caller owns a separate operation. |
| `:1988` `DatabaseCatalogReadFuture::try_prepare` | New use | Yes, only `refresh_catalog_document` uses `try_submit` at `:8192` | The refresh is awaited wholly inside `run_document_mount` (`:8355-66`); that mount owns a clone passed from `mount_document` (`:8310`, `:8373-84`). It is transient but does not escape the mounting owner. |
| `:3340` `DatabaseCatalogBootstrapFuture::try_prepare_with_key` | New use | No instance path after open; static wrapper at `:8062` | `open_with` instead passes the root clone through `try_prepare_with_use` at `:8100-09`. |
| `:7086` `DatabaseCreateCatalogFuture::try_prepare` | New use | The private direct path is not used by a live `Database` method | `create_document_catalog_retained` and `publish_mount_catalog` both use `try_prepare_with_use` (`:8381-83`, `:8226-29`) with a clone of `Database.pool_use`. This is the right repaired admission path, subject to the public-completion P0 below. |
| `🔄️sync/🦀️.rs:1835` `DatabaseSyncHelloFuture::try_submit` | New use | **Yes**: `Database::hello_retained` calls it at `⚙️engine/🦀️.rs:8563-64` | **P0.** The precheck clone named `_pool_use` is dropped before `try_submit`; state retains only the independently acquired use (`🔄️sync/🦀️.rs:1487-90`, `:1860-63`). A returned `DatabaseSyncHelloSession` retains that state through `:2082-2123`, but not a clone seen by `Database::shutdown_step`. |
| `🗿️artifact/🦀️.rs:4652` `ArtifactAuthority::spawn` | New use | No: engine explicitly calls `spawn_with_pool_use` in open/create mount paths (`⚙️engine/🦀️.rs:8245-46`, `:8290`) | Correct: actor lifetime keeps the database guard clone and is also represented in the mount/authority shutdown registry. |
| `🗜️compact/🦀️.rs:2172` `DatabaseCompactionFuture::try_submit` | New use | No engine callsite | `Database::compact_document_retained` routes to the already mounted artifact actor (`⚙️engine/🦀️.rs:8533-42`) instead. |
| Backend constructors, e.g. `🗄️storage/🦀️.rs:8175` FS, `:6834` memory, SQLite `🪶️sqlite/🦀️.rs:722`, Postgres `🐘️postgres/🦀️.rs:846`, Neo4j `🌐️neo4j/🦀️.rs:1002` | One backend-registration use | Yes for `open_at` (FS); external supplied storage may have one too | Separate `DbIoBackend` use is released only by backend `close`/retirement. It is not measured by `Database.pool_use`'s strong count. |

`checkpoint_document` (`⚙️engine/🦀️.rs:8584-90`) keeps its `require_open_use()` clone across its own await, so it does not add a separate-acquire problem. The static retained probe APIs are intentionally independently owned and should not be folded into instance shutdown accounting.

## P0: Sync Hello Can Outlive Database Completion

`Database::hello_retained` at `⚙️engine/🦀️.rs:8555-65` currently does this:

1. clones `Database.pool_use` only to reject an already-closed database;
2. drops that clone at the semicolon;
3. calls `DatabaseSyncHelloFuture::try_submit`, which allocates an unrelated pool-use;
4. hands the state to `DatabaseSyncHelloSession`, whose Drop begins asynchronous close (`🔄️sync/🦀️.rs:2117-23`).

Meanwhile shutdown only checks `Arc::strong_count(self.pool_use) == 1` at `⚙️engine/🦀️.rs:8489-93`. It can return `Complete` with the session alive. The subsequent `WorkerPool::shutdown` remains Busy for an owner the now-complete Database does not expose or drive.

Small coherent correction:

- Add `DatabaseSyncHelloFuture::try_submit_with_use(pool, pool_use, storage, document, frontier, session_id, origin, chunk_bytes)` beside the existing standalone `try_submit`; it must take the existing guard only after all caller-owned inputs are formed and retain it in `DatabaseSyncHelloState`.
- Change `Database::hello_retained` to bind `let pool_use = self.require_open_use()?;` and use the new constructor. Keep direct `try_submit` for standalone sync callers, where its independent lifetime is explicit.
- Do not merely retain `_pool_use` in `hello_retained`; it dies on return. Do not make shutdown cancel a caller-owned session behind its back. The shared `Arc<WorkerPoolUse>` lets `shutdown_step` report `PoolUse` until normal session completion/terminal close has released it.

First native law: create a Database on its own pool, obtain a `DatabaseSyncHelloSession` through `Database::hello`, then call `shutdown_step`. It must report `Progress { phase: PoolUse, .. }`, never `Complete`; after `session.cancel()`/Drop and terminal cleanup, shutdown reaches `Complete` and `pool.shutdown()` succeeds. Run the same law with a hello rejected after admission and with an unread returned frame, so its retained close path is exercised.

## P0: Create-Catalog Success Releases Before Public Result Ownership Is Settled

The repair at `⚙️engine/🦀️.rs:7052-62` removes `state.pool_use` when `roots_are_empty()`. `drive_one` invokes it unconditionally after reaching terminal at `:6278-80`. But `publish_one` has just stored:

```text
completion = Ok(DatabaseCreateCatalogResult { state: Some(self.clone()), storage, document, actual })
```

at `:6955-61`; `roots_are_empty` at `:6975-6995` deliberately excludes `completion`.

If the caller drops the resolved `DatabaseCreateCatalogFuture` before polling or drops the returned `DatabaseCreateCatalogResult` without `into_parts`, result Drop moves the output to `terminal_completion` and arms close (`:5515-27`). The state was already `finished`, however, so `schedule` is a no-op (`:6134-38`) and `callback_close_one` exits on `finished` (`:6212-19`). `terminal_completion` owns a result that owns `Arc<State>`; state owns `terminal_completion`. This is a self-cycle retaining storage/document/outcome with no registry or callback owner. The early `pool_use.take()` hides it from the database shutdown fence.

Small coherent correction:

- Make `release_success` additionally require that **public** `completion` is empty. Do not include `terminal_completion` in `roots_are_empty`: `retire_terminal_one` needs terminal completion outside that predicate so it can drain it.
- Let `DatabaseCreateCatalogResult::into_parts` continue to call `release_success` after it has taken all output. If the result is dropped instead, leave state unfinished so existing callback-close/terminal retirement runs, then takes `pool_use` only at terminal empty.

First native law: resolve `database.create_document_catalog_retained`, hold the returned result, and show `shutdown_step` remains in `PoolUse`; drop the result before `into_parts`, drive its callback close, then verify state terminal-empty, registry slot reusable, database shutdown completes, and pool shutdown succeeds. A twin should call `into_parts` and prove release exactly once. This specifically covers the `tE5UHu` use that normal `publish_mount_catalog(...).await?.into_parts()` does not expose.

## Backend Scope Of `Complete`

`Database::shutdown_step` has phases only `Authority`, `VersionGraph`, `Emit`, and `PoolUse` (`⚙️engine/🦀️.rs:7377-82`). It never calls the already-existing backend close methods, e.g. FS `🗄️storage/🦀️.rs:8184-92`, Memory `:6860-68`, SQLite `🪶️sqlite/🦀️.rs:740-48`, Postgres `🐘️postgres/🦀️.rs:862-68`, or Neo4j `🌐️neo4j/🦀️.rs:1018-24`.

Therefore `Database::open_at(pool, ...)` creates an FS backend guard (`🗄️storage/🦀️.rs:8172-81`) on the same pool, and `Database` continues to own its `Arc<DbBackend>` after setting `shutdown_complete`. The current unit at `⚙️engine/🦀️.rs:12560-90` constructs memory storage on **`db_io_test_pool()`**, not the database pool, so it cannot detect this class.

This is a separate lifecycle contract, not evidence that the create-catalog repair is wrong. Do not call global pool completion from `Database::shutdown` until either:

1. shutdown adds a retained `Storage` phase that refuses external `Arc<DbBackend>` aliases, drives a common `DbBackend` close dispatch to terminal, and retries errors with the same backend owner; or
2. the public contract explicitly says callers own storage shutdown and `Database::shutdown Complete` only terminates database/actor work.

For an owning `open_at` API, (1) is the honest eventual target. First law: `open_at` and database on one fresh pool; after database shutdown, pool shutdown must still be Busy until the storage close phase acknowledges backend terminality, then succeed. A separate external-storage/extra-`Arc` law must block rather than closing another caller's backend.

## Existing Coverage Boundary

`database_worker_pool_use_blocks_early_shutdown_and_releases_at_terminal_ack` at `⚙️engine/🦀️.rs:12560-90` correctly proves the database root guard, mounted authority, post-terminal admission denial, and an unrelated use. It does not retain a create-catalog result, a sync hello/session, or same-pool backend storage. No qualification claim is made by this audit.
