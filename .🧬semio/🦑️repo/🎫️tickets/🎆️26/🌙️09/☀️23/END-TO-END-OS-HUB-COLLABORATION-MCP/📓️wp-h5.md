# WP-H5 — Hub Backend Gaps After H3: Graceful Shutdown, Pre-Existing Failures, Neo4j Compose, Final Gates

Slice: H5 (session 10). Ports: 7870–7879. Private cargo target: `.tmp-ticket/wp-h5/target`. Captures: `wp-h5/generated/`.

## Status

| Item | Status |
|------|--------|
| 1. Graceful hub shutdown (drain sockets, shut down `Database`, close storage → pg lock + neo4j lease released) + kill→restart law on 3 backends | DONE — e2e 2/2 on sqlite/postgres/neo4j; the law also exposed three sync-hello retirement bugs in `🛢️db/🔄️sync` (fixed + db law) |
| 2. Root-cause fixes: 3 db page-writer/mock laws, `execution-target-lease-check`, `wal-writer-authority-check` | DONE — all green |
| 3. `wal-writer-fence-live` via nx; Neo4j service in `🌎️hub/compose.yaml`; launch rows | DONE — nx 3/3 (sqlite, postgres:17-alpine, neo4j:5-community); compose `neo4j`; rows for fence-live (existing) + 3 two-client rows |
| 4. Final gates | DONE except full db `--lib` (pre-existing failures/hang, routed: r4 + list to main) |
| 5. Publish `.tmp-ticket/wp-h5/bin/os-hub`, tell c7, h4, g5, w1 | DONE (03:53 build, all 4 features) |

## Design

### Graceful shutdown (`🌎️hub/🏗️bootstrap/🦀️.rs`)
- `HubSocketDrainV1` (in `HubState.socket_drain`): every upgraded socket (document and directory) holds one
  admission for its whole life. `axum`'s graceful shutdown never waits for upgraded connections, so without this a
  document socket kept its `ArtifactHandle` (and the document's WAL writer) until the process died.
- On SIGINT/SIGTERM (and when the local bootstrap pipe closes), `begin()` makes every socket loop send close
  **1012 `hub-shutdown`** (`HUB_SHUTDOWN_CLOSE_CODE`; clients treat everything but 4401 as transient) and run its
  normal cleanup (directory session close, presence leave, color release). `main` waits ≤ `SOCKET_DRAIN_DEADLINE`
  (5 s), then runs the existing drains, then `close_hub_database`.
- `close_hub_database`: once `main` holds the last `Arc<db::Database>`, `Database::shutdown` (≤ 10 s,
  `DATABASE_SHUTDOWN_DEADLINE`), then `DbBackend::close()` — a new dispatch in `🛢️db/🗄️storage` — which drives the
  backend's writer table to terminal: Postgres advisory unlock + session close, Neo4j lease release, SQLite/fs sidecar
  release. The storage close runs even when the `Database` shutdown failed, so the cross-process fence is released
  whenever the process can still talk to the server. A `server.shutdown` trace line reports
  `retained-sockets=N database=closed|<error>`; a failure is the process exit status.
- Crash: nothing changes — Postgres ends the dead session (lock free), Neo4j frees the lease after `leaseTtlMs`.

### Law (schema-first)
`🌎️hub/🧫️fixtures/🤝️two-client-document-v1` (+ schema) gains `shutdown {closeCode 1012, gracefulExitWithinMs 15000,
crashReleaseSlackMs 30000, databaseClosedMarker, liveWriterProbe {postgres SQL, neo4j Cypher}}` and two expectations.
The e2e (`🤝️two-client-document/🟦️.ts`), after the existing steps: the server's own client (`psql`/`cypher-shell` in the
compose container) counts ≥ 1 live WAL writer while the document is open; SIGTERM with a socket open → socket closed
1012, exit 0, `database=closed`, and **0** live writers right after exit (independent of hub boot time); restart → the
document opens on the **first** attempt; SIGKILL with the document open → Postgres: the server frees the lock
(polled ≤ slack), Neo4j: the lease is still live right after the kill; restart → reopen per the writer contract's
`crashRelease` (`immediate` → first attempt; `after-lease-ttl` → refused while the lease lives when boot < TTL,
admitted within TTL + slack). Timings go to `HUB_E2E_RECEIPT` (JSON). Backend expectations are read from the writer fence contract
(`🔐️writer/🧫️fixtures/🌐️remote-guard`), not restated. Rust twin pins `closeCode == HUB_SHUTDOWN_CLOSE_CODE` and
`gracefulExitWithinMs ≥ drain + database deadlines`.

### Zero-touch e2e per backend
`os-hub-ts:two-client-e2e <sqlite|postgres|neo4j>` (`🌎️hub/📦️packages/🟦️typescript/📜️script.ts`): starts the backend's
own `🌎️hub/compose.yaml` service in a throwaway compose project (own volume, free loopback port), waits for its
readiness probe, runs the e2e at level `long`, removes container + volume. Binary = `OS_HUB_BINARY` or the Nx-staged
`build-dev-postgres` (links all three drivers). Launch rows `⚖️gate🤝️two-client-document🪶️sqlite|🐘️postgres|🕸️neo4j`.

### Compose
`neo4j` service (profile `neo4j`, `neo4j:5-community`, healthcheck, `hub-neo4j` volume) with the hub env to use it;
stale "postgres never run against a real server" note replaced. Every live lane (`fence-conformance`, hub
`directory::postgres`) now runs the compose image `postgres:17-alpine` (was 16).

## Root causes fixed

| Failure | Root cause | Fix |
|---|---|---|
| `db_io_page_writer_seal_…_is_one_opportunity` (ledger 1 page short) | `seal_retained_step` `take()`s an unused page before checking its phase; on a phase mismatch the error returns and the page lease drops into a lost handle — the rejected writer no longer owns it | check phase + transition on the borrowed page, `take()` only when it is returned to the arena (`🗄️storage/🦀️.rs`) |
| `db_io_result_page_reservation_plus_one_returns_the_writer` | law expected one `close_step` to retire page AND shell; the writer retires one owner per step (pages, then shell) by design | law asserts the exact sequence `Some(page) → Some(0) → None` |
| `db_io_postgres_and_neo4j_mock_drivers_…` | law reserved the whole `DB_IO_OPERATION_BYTES` for the driver on an operation that already holds page/task credit — can never fit | law asserts the full-budget reservation is refused and reserves one page |
| `wal-writer-authority-check` | stale markers: engine open future is now `Result<Box<ArtifactEngine<A, V>>, …>`; `remoteFixture.cases` gone since `WalWriterFenceV1` (`laws`); stale-controller law moved to the retained-fixture test file | markers follow the source; law presence read from the test file |
| `execution-target-lease-check` (manifest stage) | oracle hard-coded `appChannelVersion === 15`; the compiled constant is 17 | oracle uses the imported `DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1` |
| hub shutdown `database=unavailable … sync_hello: 1` (found by the new e2e law) | (a) `drive_one` spent a turn retiring a returned frame while a close was requested, reported "not pending" and never ran the close — the hello kept its registry slot and `WorkerPoolUse` forever; (b) `schedule()` from `arm_close`/`mount_close` published no signal, so a request that hit a busy driver was lost; (c) the tail follow-up close reset an empty envelope `Vec` instead of `None`, never reaching `origin`/`frontier` | (a) that turn stays pending; (b) `schedule()` publishes `wake_requested`, the turn clears it at start and re-reads after release; (c) `*envelopes = None`. Law `hello_sessions_retire_when_drained_and_when_close_races_the_returned_frame` (red 3/3 before, green 3/3 after) |

## Evidence

| Command | Result | Capture |
|---|---|---|
| `cargo check -p semio-hub --bin os-hub --tests --features sqlite,postgres,neo4j,native-artifact-execution` | EXIT 0 | `check-1.txt`, `check-2.txt` |
| db 3 laws (before) | 0/3 | `db-three-1.txt` |
| db 3 laws (after) | **3/3** | `db-three-2.txt` |
| `bun ./📜️script.ts wal-writer-authority-check` (os kernel) | EXIT 0 (`fence-laws=5 …`) | `wal-writer-authority-check-3.txt` |
| `bun ./📜️script.ts execution-target-lease-check` (hub) | EXIT 0 (positive=1, hostile=73, source passed) | `execution-target-lease-check-1.txt` |
| `docker compose -f 🌎️hub/compose.yaml --profile neo4j --profile postgres config --services` | hub, neo4j, postgres | console |
| two-client e2e sqlite (H5 build 1, before db fix) | 1/2: SIGTERM exit status 1, `database shutdown deadline elapsed in phase PoolUse … sync_hello: 1` | `two-client-sqlite-1.txt` |
| db law `hello_sessions_retire_…` before fix | 3/3 FAIL (`admission saturated`, `deadline`, `session 1 retired (1,1)`) | `db-hello-retire-before-*.txt`, `db-hello-debug-2.txt` |
| same after fix | **3/3 PASS** | `db-hello-retire-after-*.txt` |
| `bun nx run @semio-tech/framework-os-kernel:wal-writer-fence-live` | **3/3** (sqlite, postgres, neo4j laws; 39.6 s) | `fence-live-nx-1.txt` |
| `bun nx run os-hub-ts:typecheck` | EXIT 0 | `hub-ts-typecheck-3.txt` |
| hub `test quick` (mutex, `--no-fail-fast`) | **327/327**, 10 skipped | `hub-tests-1.txt` |
| hub `test long` (mutex) | **336/336**, 1 skipped | `hub-tests-1.txt` |
| hub `directory-live-lanes all` (mutex, Docker; pg on 17-alpine) | postgres **12** ok, neo4j **7** ok, corpus **3** ok | `hub-lanes-and-build-3.txt` |
| `cargo build -p semio-hub --bin os-hub --features sqlite,postgres,neo4j,native-artifact-execution` (mutex) | EXIT 0 → staged `wp-h5/bin/os-hub` 03:53 | `hub-lanes-and-build-3.txt` |
| two-client e2e **sqlite** (`nx os-hub-ts:two-client-e2e -- sqlite`, staged binary) | **2/2** — SIGTERM→exit 437 ms, close 1012, restart opens first attempt; SIGKILL → reopen first attempt | `two-client-final-sqlite.txt`, `two-client-receipt-sqlite.json` |
| same **postgres** (compose `postgres` service) | **2/2** — live writers open 1 → after SIGTERM exit 0 (exit 87 ms); SIGKILL → server released the lock in 377 ms; reopen first attempt both times | `two-client-final-postgres.txt`, `two-client-receipt-postgres.json` |
| same **neo4j** (compose `neo4j` service) | **2/2** — live leases open 1 → after SIGTERM exit **0** (exit 100 ms; no TTL wait); right after SIGKILL **1** (lease outlives the crash), reopen after 30.3 s (boot 27.8 s > 15 s TTL, so 0 refusals) | `two-client-final-neo4j.txt`, `two-client-receipt-neo4j.json` |
| e2e teardown when a run fails (`OS_HUB_BINARY=/nonexistent`) | test fails, no container/volume left | `two-client-teardown-probe.txt` |
| db `--lib` storage+sync (serial) | 164 pass / 17 fail; in isolation 9 fail identically with my db edits reverted (pre-existing), 1 (`…snapshot_cursor…`) red→green by my fix, rest were cascade/stack | `db-storage-sync-1.txt`, `db-storage-sync-exact-reverted.txt`, `db-storage-sync-exact-mine.txt` |
| db `--lib` full | parallel: SIGABRT (destructor panic under capacity contention); serial: hangs in `document_authority_submits_and_queries_over_finite_pool_turns` (also with my edits reverted) — routed to r4 (root cause per r4: Memory-durability submit leaves WAL pending, close errors forever) | `db-lib-1.txt`, `db-lib-serial-1.txt`, `db-authority-submit-reverted.txt` |

## Files changed
- `🌎️hub/🏗️bootstrap/🦀️.rs` — `HubSocketDrainV1`, `HUB_SHUTDOWN_CLOSE_CODE`, drain arms in both socket loops, `close_hub_database`, `main` wiring.
- `🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs` — `socket_drain` in the two `HubState` fixtures.
- `🌎️hub/🧪️tests/🤝️two-client-document/🟦️.ts` + `🦀️.rs`, `🌎️hub/🧫️fixtures/🤝️two-client-document-v1/🔣️.json`, `🌎️hub/🧬️schema/🤝️two-client-document-v1/🔣️.json`.
- `🌎️hub/📦️packages/🟦️typescript/📜️script.ts` + `📋️project.json` — `two-client-e2e` (compose service per backend, live-writer probe client, exit-safe teardown).
- `🌎️hub/compose.yaml` — `neo4j` service.
- `🌎️hub/📦️packages/🦀️rust/📜️script.ts` — lease oracle channel version; live-lane image text.
- `🌎️hub/📇️directory/🐘️postgres/🧪️tests/🔬️unit/🦀️.rs` — `postgres:17-alpine`.
- `🧰️framework/…/🛢️db/🗄️storage/🦀️.rs` — `DbBackend::close`, seal page ownership.
- `🧰️framework/…/🛢️db/🗄️storage/🧪️tests/🔬️db-io-retained-fixtures/🦀️.rs` — two laws corrected.
- `🧰️framework/…/🛢️db/🗄️storage/🔐️writer/🧪️tests/🔬️fence-conformance/🦀️.rs` — `postgres:17-alpine`.
- `🧰️framework/…/🛢️db/🧪️tests/🧯️fault-storage/🦀️.rs` — `FaultStorage::close`.
- `🧰️framework/…/🛢️db/🔄️sync/🦀️.rs` — hello driver signalling, close-after-returned-frame, envelope close.
- `🧰️framework/…/🛢️db/⚙️engine/🧪️tests/🔬️unit/🦀️.rs` — hello retirement law.
- `🧰️framework/…/💻️os/📦️packages/🦀️rust/📜️script.ts` — authority-check markers; fence-live image text.
- `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc` — three two-client rows.

## Pids

| pid | what |
|---|---|
| 66809, 90878, 24981 | hub builds / live lanes through the hub mutex (done) |
| 16974 | hub quick + long through the hub mutex (done) |
| 31998 | final e2e loop (done) |

No hub, container or cargo of mine is running. Throwaway compose projects `semio-hub-e2e-*` are removed (one leftover from a failed run was removed by hand, which led to the exit-safe teardown).

## Gaps
- Full db `--lib` is not green, and not because of H5. An authority that took a Memory-durability `submit` never
  closes (`ClosingParked`). The coordinator routed this to r4, whose root cause is: the WAL stays pending and the
  close errors "force_flush is required before close". The parallel run also aborts on a destructor panic in
  `ArtifactHistoryAdmission::drop`, which r4 is making unwind-safe. Nine storage/sync laws are red with and without
  my edits; the list went to main.
- The binary was built at 03:53 with r4's in-progress `CloseFlush` edit in the tree. The e2e runs and live lanes ran
  on that binary. Hub quick and long ran just before r4's edit.
- A Neo4j restart "within 15 s" can't be timed end to end with a debug hub, because boot alone takes 23–28 s. The law
  proves the mechanism instead: the server's own client counts 0 live leases right after the SIGTERM exit, 1 right
  after a SIGKILL, and the restart opens on the first attempt.
