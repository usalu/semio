# Remote WAL Writer Guards: SQLite, PostgreSQL, and Neo4j

Status: read-only source audit, 2026-09-05. No product source, build, server, or native selector was run by this audit. The reported worker primitive receipt is deliberately not treated as a storage integration receipt.

## Decision

The new fixed local writer table is necessary but does not itself provide remote exclusion. There are only two currently supportable guard surfaces:

| Backend | Correct cross-process guard | Current status | Required decision |
| --- | --- | --- | --- |
| physical SQLite | a stable, canonical-db-path + document-hash sidecar file held with the existing OS file lock | implementable now | use `WalFileWriterGuard` inside the SQLite executor table; do not use SQLite `lease` or an in-process connection mutex |
| PostgreSQL | a dedicated physical PostgreSQL session holding a **session-level** advisory lock allocated collision-free per document | implementable after a pollable retained-close seam | all six WAL effects must run through the held session; a pooled session or a row lock is insufficient |
| Neo4j | none exposed by the present `neo4rs`/Cypher surface | not implementable honestly | return `DbError::Unavailable` from writer acquisition before WAL inventory/recovery/mutation until a real session-scoped fencing provider is supplied |

This is intentionally fail-closed. A persisted holder node, the existing lease facilities, a process-local table, a timer/TTL, or a transaction lock released after each physical commit are not substitutes.

## Current ownership and effect map

The unguarded boundary is the six mutators in [`WalStorage`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:4586>): `create_segment`, `append`, `sync`, `seal`, `truncate_tail`, and `delete_segment` ([lines 4587–4627](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:4587>)). Their task forms also carry only backend/document today ([`DbIoTask`, lines 2021–2032](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:2021>)). The wrapper dispatcher is `WalRef` ([lines 4897–4910](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:4897>)).

`ArtifactWal::open_with_control` starts inventory before recovery ([`wal`, lines 2326–2348](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:2326>)); repair then calls `truncate_tail`, `sync`, and append/flush ([lines 2376–2391](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:2376>)). A guard acquired after inventory is already too late. `SegmentWriter::begin`, `commit_and_flush`, and rotation therefore all need the same retained permit ([`wal:2135`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:2135>), [`wal:2183`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:2183>), [`wal:2484`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:2484>)). `db_compact::apply_wal_retention` is the separate production delete caller ([`compact:176`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗜️compact/🦀️.rs:176>)); it must acquire the same document permit rather than retain a raw delete bypass.

The core table presently has the correct local slot/backend/document/generation pin shape ([`writer:47–122`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer/🦀️.rs:47>)), and `WalFileWriterGuard` already makes an actual file descriptor lock ([`writer:125–155`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer/🦀️.rs:125>)). It does not yet appear in any `WalStorage` implementation.

## SQLite: use the existing file guard, but key it correctly

`SqliteDbIoExecutor` owns only one local `Mutex<Option<Connection>>` ([`sqlite:91–109`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🪶️sqlite/🦀️.rs:91>)); that serializes one executor, not an independent process. `WalAppend` stages fragments in `db_io_stage` and yields before its final `TransactionBehavior::Immediate` commit ([`sqlite:116–127`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🪶️sqlite/🦀️.rs:116>), [`sqlite:241–257`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🪶️sqlite/🦀️.rs:241>)). Holding `BEGIN IMMEDIATE` only at that final step therefore cannot guard an `ArtifactWal` between submissions, recovery, rotation, or staged yields. The `lease` table is TTL-based ([`sqlite:48–53`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🪶️sqlite/🦀️.rs:48>)) and is forbidden for this purpose.

### Exact guard construction

1. In `BackendOpen`, after `Connection::open` has created the database and before any permit can be issued, derive the canonical database identity from `std::fs::canonicalize` of the actual database file. `SqliteStorage::open` already distinguishes physical paths from `open_in_memory` ([`sqlite:605–623`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🪶️sqlite/🦀️.rs:605>)). Reject a noncanonicalizable physical path rather than allowing `a.db` versus `./a.db` to create independent locks.
2. Make `sidecar = canonical_database.parent()/.semio-wal-writer/<sha256(canonical_database_bytes || 0x00 || document_utf8)>.lock`. Create only that fixed directory and never unlink a sidecar. Hashing avoids turning a user document identifier into a path component. The database and lock must be on the same shared filesystem namespace; `:memory:` gets only the local table guard and must make no cross-process claim.
3. Run `WalFileWriterGuard::try_acquire(sidecar)` only on the admitted I/O lane, immediately put the returned descriptor into `WalWriterTable<WalFileWriterGuard>`, and retain it from pre-inventory `ArtifactWal` acquire through terminal writer release. The source guard uses `OpenOptions(read/write/create, truncate=false)` and `File::try_lock`, maps contention to `Conflict`, and intentionally keeps the inode on close ([`writer:133–149`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer/🦀️.rs:133>)).
4. Stamp and validate every one of the six `DbIoTask` mutators before its first effect. In particular, hold the stamp/operation pin over every `write_stage_step` yield and over `close_operation_step`'s exact stage-row deletion ([`sqlite:515–528`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🪶️sqlite/🦀️.rs:515>)). Releasing while a staged append is pinned must retain the sidecar until the task cleanup unpins it.

SQLite's existing physical native fixture surface is usable: the writer primitive already starts a separate test executable, uses `SEMIO_TEST_ARTIFACT_DIR`, and verifies a PID-bound child conflict sentinel ([`writer:309–344`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer/🦀️.rs:309>)). Adapt it to open the same real `SqliteStorage` by canonical aliases and hold a real permit; do not substitute `open_in_memory` tests such as the present SQLite suite ([`sqlite:861–901`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🪶️sqlite/🦀️.rs:861>)).

## PostgreSQL: a session-level advisory lock is the supported remote guard

The present Postgres executor owns a 16-connection `PgPool` ([`postgres:115–134`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🐘️postgres/🦀️.rs:115>)). Each current WAL effect uses that pool: create and seal/delete use a pooled autocommit query, append and truncate use a short transaction with `FOR UPDATE`, and sync is a no-op ([`postgres:217–329`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🐘️postgres/🦀️.rs:217>)). Those row locks end at each transaction commit. They protect one append but cannot protect the next submission, tail repair, or rotation. The existing `db_lease` schema is explicitly expiry based ([`postgres:63–68`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🐘️postgres/🦀️.rs:63>)); it must not be reused.

### Schema-first server identity

Bootstrap two additional immutable mapping objects with the existing `SCHEMA_STATEMENTS` at [`postgres:33–69`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🐘️postgres/🦀️.rs:33>):

```sql
CREATE SEQUENCE IF NOT EXISTS db_wal_writer_key_seq AS BIGINT MINVALUE 1;
CREATE TABLE IF NOT EXISTS db_wal_writer_key (
  document_id TEXT PRIMARY KEY,
  advisory_key BIGINT NOT NULL UNIQUE
);
```

Acquire the stable, collision-free signed `BIGINT` with one transactional/upsert statement that inserts `nextval('db_wal_writer_key_seq')` only for a new document and returns the existing value otherwise. Do **not** hash document bytes down to the advisory `BIGINT`: a collision would falsely co-own two documents. On a freshly opened **dedicated `PgConnection`**, execute `SELECT pg_try_advisory_lock($1)` and require `true`. The session owns that server lock until a matching unlock or disconnect; a process crash closes the server connection without a TTL or a stale holder record.

The pool cannot own this guard. A pooled connection returned after a guard release would silently retain its session advisory lock; it also cannot ensure a mutation later uses the same session. Instead retain one direct `sqlx::postgres::PgConnection` per table entry (bounded by `WAL_WRITER_CAPACITY`). SQLx exposes precisely that physical connection type ([local `sqlx-postgres 0.8.6`, `connection/mod.rs:36–38`](</Users/ueli/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sqlx-postgres-0.8.6/src/connection/mod.rs:36>)). The capacity relationship must be explicit: 32 permits may require 32 dedicated server sessions in addition to the normal 16-query pool; admit a permit only after a direct connection and advisory lock have both succeeded.

### Effect and close rules

* Route **all six** guards-stamped mutations through `&mut PgConnection`, not `&PgPool`. Append and truncate may use short transactions on that same connection; commit makes the WAL bytes visible/durable while the session lock stays held. `sync` remains an effect-free Postgres durability acknowledgement, but still validates and pins the permit so release cannot cross it.
* A connection/transport error after acquisition poisons that table entry: it accepts no later mutation, retains the operation until terminal cleanup, and begins closing the same physical connection. It must never reuse a possibly-desynchronized session or relock transparently under the same permit/generation.
* `PgConnection::close` is asynchronous and must be driven to terminal. The local SQLx contract is explicit: merely dropping a connection cannot notify the server and can delay resource release until TCP keepalive; callers should call and await `close` ([local `sqlx-core 0.8.6`, `connection.rs:19–35`](</Users/ueli/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sqlx-core-0.8.6/src/connection.rs:19>)). It sends `Terminate` then shuts down the stream ([`sqlx-postgres connection/mod.rs:153–165`](</Users/ueli/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sqlx-postgres-0.8.6/src/connection/mod.rs:153>)). Therefore a synchronous `WalWriterGuard::close_step` is not sufficient for a PostgreSQL guard.

The minimal table extension is a pollable retained guard close, not a detached release task from lost-owner cleanup:

```text
WalWriterGuard::close_step(&mut self, Context) -> Poll<Result<GuardClose, DbError>>
GuardClose::{Pending, Terminal}
WalWriterTable::{begin_release, poll_release, finish_operation}
```

`PostgresWriterGuard` owns exactly one of `Live(PgConnection)`, `Closing(BoxFuture<Result<(), sqlx::Error>>)` created by consuming `PgConnection::close()`, or `Empty`. It does not store a future borrowing a separately stored connection. The backend's fixed signal callback must obtain its preallocated controller/executor only when no async DB task leases that executor, poll one guard future outside table/registry mutexes, then return it. Its persistent waker re-requests the already-installed fixed maintenance ticket; construct that waker once with the backend controller, not from permit `Drop`. This is compatible with the current signal plan's no-I/O `Drop`; it requires a pollable callback/controller seam because `WorkerMaintenanceCallback` currently has only `fn([u64; 2]) -> WorkerMaintenanceStep` ([`async maintenance:26–37`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/⏳️async/🔔️maintenance/🦀️.rs:26>)).

Do not use a `PgPoolConnection` or `Drop` fallback. On normal close, only `pg_advisory_unlock` followed by awaited `Connection::close`, or awaited close alone, establishes bounded server release. On a real process crash, PostgreSQL releases the session advisory lock when the socket/session disappears. Both routes leave the core table in `Releasing` until the terminal witness and its signal-cell epoch advance; a new same-document permit remains rejected until then.

## Neo4j: fail closed; no present honest implementation

Current Neo4j writes have only transaction-scoped exclusion. The module documents that its read-modify-write operations lock only for one transaction and explicitly calls cross-process mutual exclusion an extension seam ([`neo4j:33–39`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🌐️neo4j/🦀️.rs:33>)). `Txn` indeed reserves a pool connection but releases it on handle drop ([local `neo4rs 0.8.0`, `txn.rs:10–18`](</Users/ueli/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/neo4rs-0.8.0/src/txn.rs:10>)); `commit` ends that transaction ([`txn.rs:59–75`](</Users/ueli/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/neo4rs-0.8.0/src/txn.rs:59>)). The actual append commits at [`neo4j:420–451`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🌐️neo4j/🦀️.rs:420>); truncate does the same ([`neo4j:509–540`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🌐️neo4j/🦀️.rs:509>)).

Holding a single `Txn` for the `ArtifactWal` lifetime would retain the lock but makes every append uncommitted/invisible until final release, contradicting physical commit+sync semantics. Committing each append releases the lock and reintroduces the interleaving race. `Graph::run`/`execute` use pooled auto-retry behavior ([local `neo4rs 0.8.0`, `graph.rs:69–141`](</Users/ueli/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/neo4rs-0.8.0/src/graph.rs:69>)), which is additionally unsuitable as a long-lived guard session.

A `(:WalWriter {document, holder})` node is not a solution. If retained after a crash, recovery permanently blocks; if another process reclaims it, it needs a forbidden time lease or an external failure detector and can split brain. The existing `(:Lease)` shape has `expiresAtMs` ([`neo4j:341–352`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🌐️neo4j/🦀️.rs:341>)) and is expressly excluded. A single-property index for `WalSegment` is not even a uniqueness fence ([`neo4j:203–216`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🌐️neo4j/🦀️.rs:203>)).

Accordingly, implement `Neo4jStorage::acquire_writer` as `Err(DbError::Unavailable("Neo4j lacks a session-scoped WAL writer fence"))`, and have `ArtifactWal::{create,open_with_control}` acquire before `list_segments`. The current capabilities (`durable`, `Fsync`, `cas`) may remain truthful for individual blob effects, but a new explicit exclusive-WAL-writer capability must be false. A future Neo4j implementation needs an independently supplied server-authoritative, session-bound fence whose effect queries verify the exact fence generation; it is a new provider boundary, not an adaptation of `LeaseStorage`.

## Compiler-coherent integration packet

1. Extend `WalStorage` with `acquire_writer(document)` and a retained close/wait result; require `&WalWriterPermit` on exactly the six mutation methods. Keep reads, length, state, and inventory permit-free. Add `WalWriterAcquire`, stamp-bearing six mutations, and a release/signal result path to `DbIoTask`/`DbIoResult`; update `WalRef`, Memory, FS, SQLite, Postgres, Neo4j, `FaultStorage`, and the WAL fixture adapters together. No raw overload remains.
2. Make acquire preflight the local table capacity/generation and allocate backend ownership before constructing a remote guard. On a failed table insertion after a remote acquisition, drive that exact guard to terminal immediately rather than drop it. The real backend control must be bound before the table is published, matching the existing signal plan.
3. Pin the task's exact operation before its first mutable effect. A SQLite multi-page staging operation stays pinned across every yield. A Postgres operation borrows its retained direct connection only while the async executor is leased, then calls `finish_operation` on every normal, error, cancellation, and task-close path. A release request fences not-yet-pinned work immediately but permits the one exact in-flight operation to finish.
4. Give backend close precedence over neither an active task nor a releasing remote guard. `DbIoTaskExecutor::close_backend_step` already receives a `Context` ([`storage:2877–2914`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:2877>)); make `PostgresDbIoExecutor::backend_terminal_is_empty` include the writer table and all guard-close futures. The current Postgres operation close reports unconditional terminal ([`postgres:822–824`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🐘️postgres/🦀️.rs:822>)); that cannot stay true for a pinned stamped operation.
5. Preserve the permit in `ArtifactWal` from acquisition before inventory to an explicit asynchronous `CloseActive -> release permit -> wait signal terminal epoch` lifecycle. A dropped permit only sets the fixed signal cell; it never submits/polls a DB task while holding lost-owner state. The existing signal integration plan remains the owner of that nonreentrant route.

## Neutral corpus and acceptance laws

Add one strict schema-first fixture, proposed path `db/🗄️storage/🔐️writer/🧪️fixtures/🌐️remote-guard/{🧬️.schema.json,🔣️.json}`, with exact strings/enums, no numeric floating representation:

```json
{
  "version": "semio.db.wal-writer-remote-guard.v1",
  "mutations": ["create", "append", "sync", "seal", "truncateTail", "delete"],
  "cases": [
    {"backend":"sqlite","name":"canonical-alias-conflict","holderExit":"explicitClose","expect":"conflictThenAcquire"},
    {"backend":"sqlite","name":"crash-releases-sidecar","holderExit":"processExitWithoutRelease","expect":"reacquire"},
    {"backend":"postgres","name":"session-advisory-conflict","holderExit":"explicitClose","expect":"conflictThenAcquire"},
    {"backend":"postgres","name":"session-crash-releases-lock","holderExit":"processExitWithoutRelease","expect":"reacquire"},
    {"backend":"neo4j","name":"no-fence-fails-before-inventory","expect":"unavailableNoWalEffect"}
  ]
}
```

First executable laws:

1. **`sqlite_wal_writer_real_database_alias_and_crash_are_exclusive`** — parent and child use the same physical SQLite file with path aliases; child observes `Conflict` while parent holds a permit through a yielded staged append, then acquires only after parent terminal release. A child which exits without release proves OS descriptor cleanup and parent reacquisition. Verify no staged append permits a concurrent `create/append/sync/seal/truncate/delete` stamp.
2. **`postgres_wal_writer_session_advisory_fences_all_six_mutations`** — two independent `PostgresStorage` instances against a supplied disposable database; B conflicts before every effect while A holds the direct advisory session, A executes all six with the valid permit, and B can acquire only after A's awaited close. Include stale slot/backend/document stamps with unchanged bytes/seal state.
3. **`postgres_wal_writer_session_loss_releases_without_ttl`** — a child writes its PID sentinel after lock acquisition then exits without explicit release. Parent observes process exit and retries server acquisition until its ordinary session-disconnect release succeeds; this is not a clock-based lease handoff. The test must not use a pooled advisory session.
4. **`postgres_wal_writer_close_future_retains_signal_and_credit`** — request release while an operation is pinned, then make the direct close future pending once. Assert table slot, terminal epoch, file/session ownership, and backend credit remain; after its waker turn they retire exactly once. This guards the asynchronous close seam absent from the current file-only primitive.
5. **`neo4j_wal_writer_fence_is_unavailable_before_inventory`** — `ArtifactWal::open_with_control` receives the precise unavailable result before `list_segments`, `create_segment`, or any append/truncate/seal/delete query. This is a correctness gate, not proof of a Neo4j writer.

## Local facility reality

The existing SQLite child-process fixture is usable now. PostgreSQL source itself states that this environment has no `DATABASE_URL`/live server and that live integration tests are deferred ([`postgres:1111–1115`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🐘️postgres/🦀️.rs:1111>)); this audit found no registered Postgres test URL or executable facility. Make the Postgres laws an explicit native group requiring `SEMIO_TEST_POSTGRES_URL` for a disposable database and fail its preflight clearly when absent rather than silently skip. The repository README configures a developer Neo4j Desktop endpoint, but the current Neo4j storage tests also defer live integration ([`neo4j:1420–1429`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🌐️neo4j/🦀️.rs:1420>)); do not write into a developer graph merely to test an unsupported guard.

No runtime qualification follows from this packet.
