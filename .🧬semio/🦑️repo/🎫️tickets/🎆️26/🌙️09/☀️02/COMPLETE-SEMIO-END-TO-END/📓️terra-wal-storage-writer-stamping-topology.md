# WAL Storage Writer-Stamping Topology

## Required breaking boundary

The current `WalStorage` trait exposes six unguarded mutations: `create_segment`, `append`, `sync`, `seal`, `truncate_tail`, and `delete_segment` ([storage](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:4621)). The exact task forms are `WalCreate`, `WalAppend`, `WalSync`, `WalSeal`, `WalTruncate`, and `WalDelete` ([storage](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:2026)). They must all carry the same non-cloneable writer authority. Reads, length/state/list inventory, and replay remain unguarded reads.

Use one breaking trait shape, not an un-stamped overload:

```rust
async fn acquire_writer(&self, document: &ArtifactId) -> Result<WalWriterPermit, DbError>;
async fn create_segment(&self, writer: &WalWriterPermit, index: u64) -> Result<(), DbError>;
async fn append(&self, writer: &WalWriterPermit, index: u64, bytes: DbIoPages) -> Result<u64, DbError>;
async fn sync(&self, writer: &WalWriterPermit, index: u64, class: DurabilityClass) -> Result<(), DbError>;
async fn seal(&self, writer: &WalWriterPermit, index: u64) -> Result<(), DbError>;
async fn truncate_tail(&self, writer: &WalWriterPermit, index: u64, new_len: u64) -> Result<(), DbError>;
async fn delete_segment(&self, writer: &WalWriterPermit, index: u64) -> Result<(), DbError>;
```

The mutable calls must derive both `document: DbIoText` and `writer: WalWriterKey` from the permit. They must not accept a second caller-supplied document value: that would recreate a source/stamp mismatch at the public boundary. The six `DbIoTask` variants get a crate-private `writer: WalWriterKey` field. A task is rejected before its first effect unless its backend control, full writer key, and task document match the table entry.

`WalWriterPermit` is already public while its `key()` and `document()` are crate-private ([writer](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer/🦀️.rs:12)); that is the right visibility for a public storage trait but non-forgeable task stamp. Do not make `WalWriterKey` public merely to preserve direct external construction of `DbIoTask`.

## Central task lifecycle edits — root-owned

`FsDbIoExecutor::WalAppend` yields once per retained page ([storage](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:6940)); a preflight-only validation is therefore insufficient. The table pin must span enqueue, every `Yield`, completion, cancellation, and task retirement.

1. In `DbIoTask`, add a helper that identifies only the six stamped mutable variants. Update its backend/close/terminal pattern groups where required by the new field (most existing `..` matches remain valid).
2. At task admission, validate the stamp but do **not** treat validation as a pin. At the first executor step, call `WalWriterTable::pin_operation(key, backend, document, operation)`; later yield steps revalidate the same `operation`. A release request then rejects new work while preserving the in-flight writer.
3. Centralize `finish_operation` in the task driver, not each storage mutation arm. It must run exactly once after every terminal `Complete` or `Err`, and after queued/async cancellation cleanup has retired the task payload. The relevant common paths are `db_io_executor_execute` ([storage](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:2802)), `db_io_drive_one` (starts at [storage:3558](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:3558)), and `DbIoTaskOperation::finish` ([storage:4128](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:4128)). Otherwise a dropped permit can signal a release permanently behind a stale operation pin.
4. Mirror that state across `db_io_take_async_executor`, `db_io_return_async_executor`, and the async-driver/lost-owner close path. The task is moved with the leased executor on async-native backends, so neither unlease nor a driver fault may erase the stamp before `finish_operation` has retired it. A blocking-only fix is incomplete for Postgres/Neo4j.
5. Add one `WalWriterTable<G>` to every writer-capable backend executor. Its `writer_release_step`, `close_backend_step`, and `backend_terminal_is_empty` must be the same table's release/close/empty witnesses. Memory's executor begins at [storage:5613](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:5613); filesystem's blocking executor begins at [storage:6914](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:6914).

The first Memory gate should prove: append yields under an operation pin; dropping the permit signals but cannot release it; terminal task retirement re-requests release; an old key/document/backend stamp is rejected; no table slot, signal waiter, task credit, or backend credit remains after close.

## Storage implementation inventory

| Owner | Required exact change |
|---|---|
| `storage/🦀️.rs` `WalStorage` | Add `acquire_writer`; replace all six mutable signatures; add all six `writer` task fields; update task construction, task lifecycle pinning, `WalRef` forwarding at [storage:4945](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:4945), and the dynamic credit/static backing assertions. |
| Memory | Mount `WalWriterTable<()>` in `MemoryDbIoExecutor`; enable writer authority; make `[storage:6415](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:6415)` acquire and construct all six stamped tasks; validate/pin before the mutation arms at [storage:5908](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:5908). |
| Filesystem | Mount `WalWriterTable<WalFileWriterGuard>` in `FsDbIoExecutor`; acquire the **sidecar** guard within the admitted executor/table factory, not in the facade; stamp all six `FsStorage` methods at [storage:7336](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:7336), and pin all mutation steps including append's multi-yield body. This is WGPU-owned. |
| SQLite | Update both its task executor and facade `WalStorage` implementation (`storage/🪶️sqlite/🦀️.rs:203`, `:647`), including a transaction-visible writer guard across task yields. This is WGPU-owned. |
| Postgres / Neo4j | Their inner async executors and facades each implement `WalStorage` (`storage/🐘️postgres/🦀️.rs:217`, `:928`; `storage/🌐️neo4j/🦀️.rs:402`, `:1085`). They must compile against the single stamped trait and preserve the stamp through async driver handoff. They remain fail-closed until their real cross-process guard designs are mounted. |
| Fault adapter | `db/🧪️testkit/🦀️.rs:340` forwards the permit unchanged to its inner WAL for all six mutations, and delegates acquisition. Its injected append/sync errors must leave the permit live; the fixture, not the adapter, performs the later explicit release. |

## `ArtifactWal` ownership and acquisition order

`ArtifactWal::open_with_control` currently inventories with `list_segments` before it owns a writer ([wal](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:2326)). Change the construction boundary so acquisition happens **before** that inventory and stays owned by `ArtifactWal` through normal appends, recovery truncation/abort, rotation, and close:

```rust
let writer = storage.acquire_writer(&document).await?;
let (wal, report) = ArtifactWal::open_with_control(storage, writer, policy, now_ms, control).await?;
```

This should replace—not supplement—the existing document-taking `create/open/open_with_control` constructors. `ArtifactWal` owns `Option<WalWriterPermit>`; `SegmentWriter` never stores a reference to it, avoiding a self-referential owner. Thread `&WalWriterPermit` only through `SegmentWriter::begin`, `initialize_existing_empty`, `commit_and_flush`, and rotation. These are the physical mutation sites: create ([wal:2136](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:2136)), recovery truncate/sync and abort ([wal:2378](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:2378)), append/sync ([wal:2197](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:2197)), and seal/create rotation ([wal:2484](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:2484)).

The empty-inventory branch must call a private `create_acquired`, not public `create`, otherwise it double-acquires and deterministically conflicts. Every failed construction path must first retire its `SegmentWriter`/pages and then explicitly drive the retained writer-release owner; a bare `Drop` only signals asynchronous release and makes immediate reopen/error tests race the still-held guard.

Replace the synchronous `ArtifactWal::close_step` ([wal:2501](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:2501)) with one retained async close state: close the active `SegmentWriter`; then consume the permit into its exact release future; await its terminal witness (including defined fault/retry ownership). `terminal_is_empty` must require both buffer and permit/release state empty. Do not release before the segment buffer has reached terminal emptiness.

## Production callers that must retain and close

- `ArtifactEngine::create_retained` and `open_retained` obtain the permit before WAL create/open ([artifact:1183](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:1183), [artifact:1242](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:1242)). Its error cleanup at [artifact:1306](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:1306) and all engine retirement laws move to the async WAL close. The writer thereby also fences the subsequent replay of the same open document.
- Cluster tail replication must acquire the **follower** permit before replaying follower state/choosing the tail, then pass it to `ArtifactWal`; opening only at the existing tail branch ([cluster:217](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🌐️cluster/🦀️.rs:217) leaves a plan-to-append race. Leader reads require no permit. Close the follower WAL on every success/error branch.
- CLI `repair` currently drops `_wal` immediately after reporting ([cli:1051](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⌨️cli/🦀️.rs:1051)); it must explicitly close before returning success. CLI `migrate` ([cli:1254](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⌨️cli/🦀️.rs:1254)–[1279](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⌨️cli/🦀️.rs:1279)) must force flush then close in a single cleanup outcome, including bytes/batch/submission failures.
- Compaction's two production deletion loops mutate raw WAL directly at [compact:745](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗜️compact/🦀️.rs:745) and [compact:1249](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗜️compact/🦀️.rs:1249). Acquire writer before the horizon inventory ([compact:731](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗜️compact/🦀️.rs:731), [compact:1229](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗜️compact/🦀️.rs:1229)); retain it through candidate tracing/deletion; explicitly close it before relinquishing the compaction lease. The lease does not substitute for a process/backend writer guard.

## Fixtures and adapters

Raw physical fixture builders must acquire a real writer permit before using any of the six methods—even fixtures that intentionally seed malformed/torn/foreign physical frames. The permit authorizes storage mutation; it does not bless the bytes for parser admission. A fixture must retain and explicitly close the permit before calling `ArtifactWal::open` for its next phase. This affects direct seeds in `db/📝️wal/🦀️.rs`, compaction helpers/tests, `db/⌨️cli/🦀️.rs` tests, storage's Memory/FS contract tests, and `db/🧪️testkit/🦀️.rs` fault tests.

The first neutral/native corpus should cover: same document second acquire conflicts; different documents coexist; every one of the six stamped task kinds rejects forged old-key/wrong-document/wrong-backend stamps before effect; FS append survives a yield while release is pending; ArtifactWal cancellation/error releases the exact permit before a second open; compaction and an actor cannot both inventory/delete the same document; and a raw torn fixture can be reopened only after its fixture permit has closed.
