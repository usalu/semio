# WAL Segment State Contract Implementation

Status: implementation, registered source oracle, and exact native execution are green.

## Implemented boundary

- `WalStorage` exposes the read-only `segment_state(document, index)` contract with exact `WalSegmentState::{Active, Sealed}` results and `DbError::NotFound` for absent segments.
- The typed I/O taxonomy carries `DbIoTask::WalState` and `DbIoResult::WalSegmentState`; task close/terminal ownership and scalar result close/terminal/handback paths classify it explicitly.
- `WalRef` dispatches every enabled backend and boxes the recursive `FaultStorage` arm.
- Memory reads the retained segment's persisted `sealed` flag.
- Filesystem first requires the segment byte file, then classifies an existing marker as sealed and only marker `NotFound` as active. Other marker metadata failures remain I/O errors, and a stale marker cannot make an absent byte file appear present.
- SQLite reads only the `sealed` column, decodes exactly integer zero/one, and rejects any other stored value as corruption. Append and truncate reuse the same decoder.
- PostgreSQL uses one `SELECT sealed ... fetch_optional` query without row locking or byte projection.
- Neo4j uses one `CYPHER_WAL_STATE` projection returning only `n.sealed AS sealed`.
- `FaultStorage` delegates the observation directly without touching append, delegated-sync, or CAS counters.

## Laws and registration

- The shared backend law proves active after create, sealed after seal, missing-segment `NotFound`, and byte/length stability across both observations; it runs for Memory and filesystem.
- The filesystem-specific stale-marker law removes only the segment byte file after sealing and proves the surviving marker cannot resurrect the segment.
- The `DbBackend` facet law proves active-to-sealed dispatch.
- SQLite has a third-party-backed active/sealed/missing lane law and a hostile non-boolean decoder law.
- PostgreSQL and Neo4j have pure query/projection and boolean mapper laws.
- The fault-wrapper law proves both state values while all fault counters remain zero.
- Registered targets: `@semio-tech/framework-os-kernel:wal-segment-state-check` and `@semio-tech/framework-os-kernel:wal-segment-state-native-check`.
- Registered launch entries: `⚖️gate🚦️wal-segment-state` and `⚖️gate🚦️wal-segment-state🦀️native`.

## Evidence

- Registered Nx source oracle: `wal-segment-state-check: checks=12 clean`, exit 0 on both the initial and post-stale-marker runs.
- `rustfmt --check` parsed all five changed Rust units successfully; it returned exit 1 only for pre-existing repository formatting differences, including unrelated regions in the large storage module. It was not treated as a green formatting receipt.
- `git diff --check` is clean for the storage/testkit and registration patch.

## Exact native receipt

- Registered Nx target `wal-segment-state-native-check` passed all nine selected laws across the neutral, filesystem-stale-marker, SQLite, PostgreSQL, Neo4j, and fault seams.
- Receipt: `🗑️generated/wal-segment-state-exact/exact-cargo-laws-XVVabJ/00`.
- Executable: `db-47a198dc303a982f`; SHA-256 `033e97b70b8eac956ae3239217af1e7663572ed7dcc0f369067e93ae6fa18092`.
