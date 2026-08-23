# P1v DB-engine Catalog-root Read Caller Census — 2026-08-23

Date: 2026-08-23

## Pre-edit admission

This census was written before P1v production edits. The only selected bridge is the single
production `db_actor::block_on(async { storage.catalog().await.read_root().await })` in
`Database::open_with`. The adjacent empty-catalog initial CAS, create-document catalog CAS,
compaction, and sync hello bridges are explicitly outside this packet.

## Exact caller and reachability

`Database::open_with` is the sole definition that performs the selected catalog-root read. It is
reached by four public async constructors:

1. `Database::open(pool, config, storage)`;
2. `Database::open_at(pool, root, profile)` through `FsStorage::open` and `open`;
3. `Database::open_with_emit(pool, config, storage, emit)`; and
4. `Database::open_with_authz(pool, config, storage, authz)`.

Authored production reachability is unchanged from accepted P1u: Hub reaches `open_at` for Fs and
`open` for SQLite/Postgres/Neo4j; DB CLI reaches `open_at`; and DB testkit replay-law support exposes
three `open_at` helper paths. The selected read is therefore product-startup, process-command, and
public-framework reachable.

## Exact pre-edit wait census

The non-test DB engine contains exactly five `db_actor::block_on` groups:

1. catalog-root read during open — **selected**;
2. empty-catalog initial CAS — untouched;
3. create-document catalog CAS — untouched;
4. compaction — untouched; and
5. sync hello — untouched.

Removing only the selected read must leave exactly four groups in that order. The accepted P1u
capability authority and its single retained backend capability await are not part of this edit.

## Required ownership boundary

The retained operation must claim a fixed generation slot plus exact item/byte credits before the
catalog request exists. It owns the `Arc<DbBackend>` and an exact singleton catalog-root key by value,
polls only on the supplied process `WorkerPool` I/O lane, and persists handoff, one backend poll,
work/result retention, publication, cancellation, stale-generation, panic, saturation, and public
terminal cleanup. Pending wakes coalesce by generation; rejected and terminal checkout APIs return
the exact storage/key/result owners and release at most one owner per close grant.

The backend `CatalogStorage::read_root` poll remains one honest Phase 9 indivisible-latency residual.
This packet bounds scheduling and ownership; it makes no 8 ms backend-call claim.

## Pre-edit status

- selected production bridge: one;
- current production DB-engine wait groups: five;
- required post-edit groups: four;
- Cargo, Nx, Wasm, browser, runtime, network, and native tests: prohibited and unrun; and
- Phase 1: RED regardless of this packet.
