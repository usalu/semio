# FS and SQLite WAL Writer Authority

Status: source-complete for the bounded filesystem and SQLite backend slice on 2026-09-05. The language-neutral source gate is green. Rust compilation and native laws remain pending the shared breaking `WalStorage` caller migration and an explicit cache handoff.

## Implemented boundary

- `FsDbIoExecutor` preallocates one boxed unbound `WalWriterTable<WalFileWriterGuard>` before backend owner reservation, binds the real backend control exactly once, acquires stable canonical-root/document SHA-256 sidecars only inside `WalWriterTable::acquire_with`, pins all six stamped operations through task cleanup, and drains the table before existing reader/hash owners during backend close.
- `SqliteDbIoExecutor` preallocates one boxed unbound `WalWriterTable<SqliteWalWriterGuard>`. Physical databases retain their canonical UTF-8 identity after `BackendOpen` and use `<database-parent>/.semio-wal-writer/<sha256(canonical-database || 0x00 || document)>.lock`; in-memory databases use only a process-local table guard and make no cross-process claim.
- Both `WalStorage` facades expose `acquire_writer(document)` and derive the task document plus private writer key solely from the returned non-cloneable permit for `create`, `append`, `sync`, `seal`, `truncate_tail`, and `delete_segment`.
- Release callbacks use the same mounted table. Operation finish is idempotent and occurs only after central task cleanup reports terminal, so filesystem page yields and SQLite staged rows cannot outlive the operation pin.
- Backend close retires writer guards before connections, cursors, hashes, or terminal witnesses. Sidecar files are never unlinked.
- The WAL test module's raw fixture builders now acquire one exact writer permit for each complete physical mutation phase, explicitly await release before every `ArtifactWal` recovery/reopen, pass the permit into `SegmentWriter` begin/flush, and close every live `ArtifactWal` before a later owner is admitted. `AbortCancellationStorage` delegates writer acquisition and forwards the same permit through all six mutators, so its injected cancellation remains inside the stamped operation rather than bypassing authority.
- The CLI repair path now terminally closes its recovered WAL before reporting success. Migration executes admission, submission, forced flush, retained record/batch cleanup, and WAL close as one outcome: every failure branch still awaits the WAL close, primary and cleanup failures are both reported, and no successful exit retains a writer permit. Its raw verification fixture acquires/releases an FS permit around its physical seed.
- FaultStorage's six mutation-law fixtures acquire and explicitly release one delegated permit per test. The synchronous exhaustive corruption decoder likewise releases its raw Memory seed permit before constructing `ArtifactEngine`, including when seed construction fails.

## Schema and executable laws

The strict neutral remote-guard corpus is in `db/🗄️storage/🔐️writer/🧪️fixtures/🌐️remote-guard`. It fixes the two SQLite, two PostgreSQL, and one Neo4j decisions from the Terra packet and fixes the exact six mutations. The memory-backing corpus now separately describes the boxed 32-slot writer table and dynamic controller wake credit.

Registered native laws added to `wal-writer-authority-native-check`:

- `db_io_memory_backend_heap_tables_have_exact_preflight_credit_and_terminal_return`
- `fs_storage_canonical_alias_writer_fences_all_six_mutations`
- `sqlite_wal_writer_real_database_alias_and_crash_are_exclusive`

The FS law proves canonical aliases share stable sidecars, all six foreign-backend stamps fail before an effect, exact owner operations succeed, release permits reacquisition, and lock inodes survive close. The SQLite law opens one real physical database through aliases and a separate test process, proves conflict while another process owns the permit, proves terminal reacquisition, exits a child without cleanup, and proves the OS descriptor release permits a new writer without a TTL.

## Receipts and nonclaims

Registered source gate:

```text
NX_ISOLATE_PLUGINS=false bun x nx run @semio-tech/framework-os-kernel:wal-writer-authority-check --skip-nx-cache
session 63980, exit 0
wal-writer-authority-independent-oracle: AJV=3 exact-u64=1 cases=3 mutations=6 remote=5 writer-slots=32 retained-result=1
```

This final source receipt was captured after the WAL, CLI, and FaultStorage test migrations above. It validates the three AJV corpora and exact source ownership markers. It is not a Rust compile, filesystem runtime, SQLite runtime, or native qualification receipt. Root-owned production WAL, compaction, PostgreSQL, and Neo4j migration was still converging, so the native group remained intentionally unlaunched.
