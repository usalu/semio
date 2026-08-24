# P1q-R4 Platform Syscall Census

Date: 2026-08-24  
Scope: source census only  
Disposition: **B1–B6 backend boundaries are updated; repository R4 snapshot/index/WAL/caller work remains separately active and is not claimed green here.**

No runtime, network, database service, Cargo, Nx, Wasm, browser, latency, allocation, or broad build gate was run. Runtime max/p99 values therefore remain deferred rather than fabricated.

## Backend And Platform Boundaries

| Boundary | One indivisible opportunity | Pre-admitted repository owner | Cancellation/close | Runtime max/p99 |
| --- | --- | --- | --- | --- |
| Memory typed task | one `MemoryDbIoExecutor::execute_step` cursor grant | fixed memory arrays/boxed fixed WAL backing plus task/result pages under one operation ledger | cancellation between grants; backend owner/cursors/pages close explicitly | deferred |
| Filesystem open/stat/exists | one open/metadata/existence call | fixed task/path text and owned FS backend control | exact task remains attached through close | deferred |
| Filesystem read | one `Read::read` into a 16 KiB fragment | supplied admitted page writer | reader/offset/writer retained; cancellation between steps | deferred |
| Filesystem write/append | one retained page-fragment call | one immutable page fragment, at most 16 KiB | page advances only after success | deferred |
| Filesystem replace | one create/open/write/sync/rename phase | operation-keyed temporary and retained task/page | cleanup rediscovers and retires the exact operation temporary | deferred |
| Filesystem list | one `read_dir` successor opportunity | fixed 4,096-scalar list, stable successor cursor | no collected directory vector | deferred |
| SQLite open/schema | one connection/schema step | fixed path/schema and registered SQLite control | mandatory backend close cursor and terminal witness | deferred |
| SQLite staged write | one transaction/stage page update | one retained 16 KiB input fragment keyed by operation | rejected/faulted stage rediscovered at close | deferred |
| SQLite staged read | one bounded `substr` row | supplied admitted writer | one page transfer/result close per grant | deferred |
| SQLite list/catalog/lease | one ordered row/transaction step | fixed text/scalars/pages | typed fenced terminal and explicit close | deferred |
| PostgreSQL driver future | one async SQLx query/transaction future | generation-qualified Lane::Io task/executor lease; input fixed platform owner; `MAX_READ_BYTES` reserved before every driver `Vec` | actual `Vec::capacity` observed, bytes written into supplied task writer, source dropped, reservation closed, terminal completed under same operation | deferred |
| PostgreSQL list | one ordered `LIMIT 1` row | one scalar admitted per task turn | no `fetch_all` or public raw row vector | deferred |
| PostgreSQL backend close | one poll of owned `PgPool::close()` future | fixed backend-control slot and backend owner operation | terminal only when `PgPool::is_closed()` | deferred |
| Neo4j driver future | one async Bolt query/transaction future | generation-qualified Lane::Io task/executor lease; input fixed platform owner; maximum driver owner reserved before query | native `BoltBytes` owner observed, written into supplied writer, dropped, reservation explicitly closed; no base64 owner | deferred |
| Neo4j list | one ordered stream `next` | one scalar per opportunity | stable fixed list and typed terminal | deferred |
| Prepared contiguous input | one page-copy poll | one of 16 static platform slots plus operation page credit | ABA slot and page credit close explicitly or through mounted lost-handle retirement | deferred |
| Lost-owner maintenance | one typed owner close | fixed lost-owner ring; exact candidate returned when full | one owner per opportunity; production permanently retains an unserviceable full-ring candidate and records a fault | deferred |

## Capacity Truth

- Physical page: 16 KiB.
- Page arena: 1,024 process slots.
- Per operation: 64 pages (1 MiB physical page credit).
- Fixed task operation slots: 64.
- Fixed scalar list: 4,096 `u64` entries.
- Fixed repository text: 1,024 bytes.
- Prepared platform arena: 16 fixed slots.
- Registered backend controls: 64.
- PostgreSQL raw driver byte results reserve `MAX_READ_BYTES` before allocation and reconcile actual `Vec::capacity()` before repository transfer.
- Neo4j byte results reserve the same bounded owner class before the query, use native `BoltBytes`, and reconcile the driver's immutable byte-owner length before repository transfer. There is no production base64 string/vector codec.

## Repository R4 Status

The fresh Terra audit correctly marked these production families RED:

- snapshot `materialize_chain`/whole generation materialization;
- index raw key/value owners, whole build/decode/sort/collect/merge/scan/compaction graphs;
- WAL raw byte/text/list record and replay collections;
- downstream CLI/sync/cluster/compact/artifact/query/projection/state callers.

Those regions are currently owned by the disjoint `p1q_r4_snapshot_index_wal` lane, documented in `📓️codex-p1q-r4-snapshot-index-wal-retained-codec-packet-2026-08-24.md`. This census does not repeat that lane's in-progress claims as completed truth. R4 must update this section after its source, hostile mutations, caller reconciliation, and exact close inventory are final. A new full P1q source audit is deferred until that integration.

## Deferred Measurement Handoff

The serialized acceptance owner must measure each locally available syscall and repository cursor separately under WorkerPool 1/2/4/default, record max and p99, and prove every repository cursor grant remains below 8 ms. PostgreSQL/Neo4j rows must be marked unavailable when no local service is configured. The same owner must run native/applicable wasm compile gates and allocation counters around max/+1, cancellation, panic, fault, retry, result take, interrupted close, lost-owner saturation, and backend shutdown.
