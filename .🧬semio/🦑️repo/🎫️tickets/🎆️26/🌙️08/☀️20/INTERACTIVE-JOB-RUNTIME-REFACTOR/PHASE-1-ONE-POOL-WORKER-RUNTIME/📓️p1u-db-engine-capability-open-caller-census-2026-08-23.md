# P1u DB-engine Capability-open Caller Census — 2026-08-23

Date: 2026-08-23

## Pre-edit admission

This census was written before production edits for the bounded P1u packet. The selected source is
the single production `db_actor::block_on(storage.capabilities())` in
`Database::open_with`. No catalog-root read/CAS, create-document CAS, compaction, hello, VCS,
history, storage implementation, Hub renderer, or generated Compose source belongs to this packet.

## Exact selected caller and reachability

`Database::open_with` is the sole definition that performs the selected synchronous capabilities
bridge. Four public async constructors reach it:

1. `Database::open(pool, config, storage)`;
2. `Database::open_at(pool, root, profile)`, through `FsStorage::open` and then `open`;
3. `Database::open_with_emit(pool, config, storage, emit)`; and
4. `Database::open_with_authz(pool, config, storage, authz)`.

Authored production callers are:

- Hub `connect_db`: one Fs `open_at` arm and SQLite, Postgres, and Neo4j `open` arms;
- DB CLI `open_db`: one `open_at` call; and
- DB testkit replay-law support: three `open_at` calls in its public helper path.

The Hub startup calls make this group product/network-process reachable. The DB CLI makes it a
process-command entry path. Public constructors also make it framework/product reachable even when
an authored caller is absent for `open_with_emit` or `open_with_authz`. DB engine/facade tests have
additional test-only callers and are not counted as authored production reachability.

The generated Compose Hub still names an obsolete pool-less `Database::open_at` call. P1q already
records that generated-source drift; no compatibility wrapper is authorized here, and the generated
file is outside this packet.

## Pre-edit production wait census

The DB engine currently has exactly six non-test `db_actor::block_on` groups:

1. storage capabilities during `Database::open_with` — **selected**;
2. catalog-root read during `Database::open_with` — untouched;
3. empty-catalog initial CAS during `Database::open_with` — untouched;
4. create-document catalog CAS — untouched;
5. compact-document backend compaction — untouched; and
6. sync hello — untouched.

Removing only the selected bridge must leave exactly five production groups. The target census is
therefore catalog read, catalog initialization CAS, create-document CAS, compaction, and hello.

## Required retained boundary

The selected capability call will be admitted before backend polling and owned by one process
`WorkerPool` I/O-lane authority. Its operation/generation, wake, retry, cancellation, completion,
terminal work/result/job, byte/item credit, and exact storage owner must remain retained across
polls. Each worker opportunity may advance only one handoff, backend poll, terminal scalar, or
owner close. Public cancellation/terminal take-resume-close must preserve the exact owner and
generation through saturation and ABA.

The underlying one backend/platform capability query remains an honest P1q/Phase 9 indivisible
latency residual. This packet can remove the nested executor bridge and bound ownership/scheduling;
it cannot claim that a backend call completes within 8 ms without runtime evidence.

## Pre-edit status

- selected wait: one;
- production DB-engine wait groups: six;
- expected post-edit groups: five;
- Cargo/Nx/Wasm/browser/runtime/network: prohibited and unrun; and
- Phase 1: RED regardless of this packet.
