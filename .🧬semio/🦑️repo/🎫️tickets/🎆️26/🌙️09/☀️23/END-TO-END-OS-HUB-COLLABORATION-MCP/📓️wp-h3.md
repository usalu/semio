# WP-H3 — Hub On Every Database: Postgres/Neo4j WAL Writer Fence, Live Proof, Socket-Grant Secret Removal

Slice: H3 (session 10). Ports: 7840–7849. Private cargo target: `.tmp-ticket/wp-h3/target`.

## Status

| Item | Status |
|------|--------|
| 1. Read H2 + db fence code | DONE |
| 2. Lock contract schema-first (`WalWriterFenceV1`) | DONE |
| 3. Postgres session advisory-lock fence | DONE — live laws green |
| 4. Neo4j lease node + fencing token fence | DONE — live laws green |
| 5. Cross-backend conformance laws (sqlite/pg/neo4j) | DONE — 3/3 lanes green (Docker) |
| 6. Live two-client e2e on pg + neo4j | DONE — sqlite 2/2, postgres 2/2, neo4j 2/2 |
| 7. Hub suites with pg/neo4j features | DONE — quick 327/327, long 336/336, directory live lanes 22/22 |
| 8. Socket-grant receipt secret removal | DONE — schema-first, hub + OS Rust + OS TS + fixtures |

## Design

### Contract (schema-first)
`🧰️framework/…/🛢️db/🗄️storage/🔐️writer/🧬️schema/🔣️.json#/$defs/WalWriterFenceV1` (replaces the never-consumed
`RemoteGuardV1`), fixture `🔐️writer/🧫️fixtures/🌐️remote-guard/🔣️.json`: one document has at most one WAL writer across
every process on the same database. Per backend: mechanism, how ownership can be lost, how a crashed holder is released;
five shared laws (second instance conflicts, second process conflicts, release admits contender, process exit releases,
lost owner is fenced without WAL effect). Postgres lock-key derivation and Neo4j lease timings are contract values with
vectors.

### Postgres — session advisory lock
- Acquire opens a **dedicated** connection (`application_name=semio-wal-writer`, server-side TCP keepalive 10 s/5 s×3 so a
  vanished host's lock frees in ≤25 s) and takes `pg_try_advisory_lock(key)`, key = first 8 bytes BE of
  `sha256("semio/db/wal-writer/v1" ‖ 0 ‖ document)`. False → `Conflict`.
- Every WAL mutation of the permit (create/append/sync/seal/truncate/delete) runs **on that session** (lent to the one
  pinned operation). A session the server ended (`57P*`/`08*`, transport loss) → `Fenced`, never lent again. So a writer
  whose session was terminated cannot write — the storage enforces it, not the holder's belief.
- Crash: the OS closes the socket → Postgres ends the session → lock released (measured 37–46 ms).
- Release: detached `pg_advisory_unlock` + `close` on the driver runtime; completion re-requests the writer controller.

### Neo4j — lease node + fencing token
- `(:WalWriter {document})` (unique constraint), `holder`, monotonic `token`, `expiresAtMs` on the **server clock**
  (`timestamp()`), TTL 15 s, renewed every 5 s by a periodic task on the driver runtime (`detach_periodic`, aborted when
  the permit releases).
- Every Cypher statement first `SET`s a property to write-lock the node before reading ownership (no lost update under
  read-committed).
- Every WAL mutation runs in one transaction whose first statement proves `holder + token + unexpired` (and renews); on
  failure the txn is rolled back → `Fenced { expected: current token, actual: ours }`.
- Crash: lease frees after TTL (measured 15.1–15.2 s).

### Socket-grant receipt secret removal
- Contract: `os.directory#/$defs/DocumentSocketGrantReceiptV1` = `{schema: "semio.hub.document-socket-grant/v1",
  protocol: "semio.session.v1", actorId, expiresAtMs}` (replaces `BrowserDocumentOpenTransportSocketGrant`). The
  now-unreferenced `hub.directory#/$defs/SocketGrantReceiptV1` is deleted. `SocketGrantReceiptV1` (with `grant`) is
  directory-socket-only.
- Hub ledger: `SocketGrantRecordV1.key: SocketGrantKeyV1 { Capability(digest) | CredentialBinding }`. `issue(capability)`
  refuses Document audiences; the new `admit_document` mints no secret, uses a time-ordered selector, and must carry the
  sealed plan. `pending_document_binding` only matches `CredentialBinding`. Plan exchange returns the new receipt.
  `consume_document_socket_grant` is extracted from `document_ws_v1` and shared with the laws. The grant-based
  `consume_socket_grant` is now `consume_scoped_directory_socket_grant`, its only production use.
- OS Rust client: `DocumentSocketGrantReceiptV1` + `issue_document_socket_grant`. The direct-child probe now emits an
  exchange ordinal (`document-socket-grant {path} {n}`) instead of a selector digest. Store sync no longer takes and
  discards a grant, and uses the receipt's protocol for the upgrade.
- OS TS: `DocumentSocketGrantReceiptV1` + `parseDocumentSocketGrantReceiptV1` (exact 4 keys). The dead
  `SocketGrantIssuerV1.issueDocument` is removed: it POSTed without the plan body the route requires. The worker's
  document path has its own test seam `documentSocketGrantTestIssue`.
- Fixtures: browser-document-open-v1 and the hub lease corpus use the new receipt. The hub script's native socket
  probe fake and the checkpoint/GIS process probes had stale `semio.socket.v1` expectations (left over from H2's
  protocol switch); they now speak `semio.session.v1`.

### Shared writer-table change
`WalWriterGuard` gained `awaiting_wake` / `register_wake` (default no-op) and `WalWriterTable::pinned_guard_mut`, so a
remote unlock in flight is not busy-polled: the controller skips it until the unlock's completion re-requests it.

### Pre-existing bugs found by the first live run (fixed)
- Postgres `octet_length(bytes)` is INT4 but decoded as `i64` → every append/read/length failed. Cast `::BIGINT` (12 sites).
- Neo4j created segments with `bytes = ''` (a string) → every append to a fresh segment failed decoding. Now `$empty` bytes.
- `PostgresDbIoExecutor::new` built the `sqlx` pool (spawns its reaper) outside any Tokio context → panicked for every
  non-Tokio caller (`lost_postgres_facade_drives_the_real_lazy_pool_to_closed` failed). Now built inside the driver
  runtime (`db_storage_driver_runtime::within`).

## Evidence

| Command | Result | Capture |
|---|---|---|
| `cargo check -p semio-framework-os-kernel-db --features sqlite,postgres,neo4j --tests` | EXIT 0 (crate warnings present) | console |
| db lib tests `writer fence db_storage_postgres db_storage_neo4j sqlite_wal_writer` (parallel) | 70 pass / 11 fail; failures are shared-ledger laws run in parallel | `db-test-1.txt` |
| same 11 + 3 new laws, one process each (`--exact`) | 11/14 pass; the 3 still failing (`db_io_page_writer_seal_…`, `db_io_postgres_and_neo4j_mock_drivers_…`, `db_io_result_page_reservation_plus_one_…`) exercise only the page writer / mock executor — untouched code, pre-existing | `db-exact-1.txt` |
| `db_storage::writer::fence_conformance` (`--include-ignored`, Docker) run 1 | sqlite ok; pg + neo4j exposed the 2 pre-existing decode bugs above | `fence-live-1.txt` |
| same, run 2 | **3/3 ok**: sqlite 4 laws, postgres 5 laws, neo4j 5 laws; independent `psql`/`cypher-shell` cross-checks; crash release pg 46 ms, neo4j 15.17 s (TTL 15 s) | `fence-live-2.txt` |
| `bun ./📜️script.ts wal-writer-authority-check` | new fence oracle PASS (`AJV=1 lockKeys=3 laws=5`, node:crypto recomputes every lock key); the script then stops on a **pre-existing** unrelated marker (`🗿️artifact` `Future<Output = Result<ArtifactEngine…>>`) | `wal-writer-authority-check.txt` |
| hub `os-hub` build (sqlite,postgres,neo4j,native-artifact-execution), mutex | EXIT 0, staged `.tmp-ticket/wp-h3/bin/os-hub` 01:29 | `build-os-hub-2.txt` |
| `cargo check -p semio-hub --bin os-hub --tests` (all 4 features), `cargo check -p semio-framework-os-kernel --tests` | EXIT 0 (warnings present, none in changed code) | console |
| OS TS `tsc --noEmit` / os-hub-ts typecheck | EXIT 0 / EXIT 0 | `os-tsc.txt`, `hub-ts-typecheck.txt` |
| OS TS `test-quick` / `test-long -t "browser document actor"` | **373/373** / **3/3** | `os-ts-quick.txt`, `os-ts-actor.txt` |
| two-client e2e (open, Ack relay, presence join+roster replay+leave+lease expiry, late joiner, rejoin, hub restart) **sqlite** :7841 | **2/2 PASS** (254 s) | `two-client-sqlite-1.txt` |
| same **postgres** (`🌎️hub/compose.yaml` `postgres` service via `docker compose --profile postgres run -p 127.0.0.1:7845:5432`, postgres:17) :7842 | **2/2 PASS** (262 s). psql afterwards: 1 WAL segment of 1774 bytes in `db_wal_segment`, 0 advisory locks left | `two-client-postgres-1.txt` |
| same **neo4j** (neo4j:5-community, the repo's live-lane image; no neo4j service exists in hub compose, and the devcontainer's is baked into the whole workspace image) :7843 | **2/2 PASS** (306 s). cypher-shell: `WalWriter` token 2 (one per hub run), 1 segment / 1774 bytes | `two-client-neo4j-1.txt` |
| hub `test quick` (mutex) run 1 | 326/327: one law still asserted the old schema | `hub-test-quick-1.txt` |
| hub `test quick` run 2 | **327/327** EXIT 0 | `hub-test-quick-2.txt` |
| hub `test long` | **336/336** (335 + new `document_socket_grants_are_credential_bound_secret_free_oldest_first_and_single_consume`) | `hub-test-long-1.txt` |
| hub `directory-live-lanes all` (sqlite,postgres,neo4j features, Docker; includes DB3 PostgresStorage open law) | postgres 12 ok, neo4j 7 ok, corpus 3 ok | `hub-live-lanes-1.txt` |
| hub `open-plan-check` (exact laws incl. renamed `…admits_one_exact_bounded_secret_free_socket_grant`) | EXIT 0, oracle + parity + native laws pass | `open-plan-check.txt` |
| hub `execution-target-lease-check` | FAIL at the corpus's positive vector, stage `manifest` (a plan-vs-manifest field comparison; the `socketGrant` field is not read at that stage). Pre-existing as far as I can tell | `execution-target-lease-check.txt` |

## Files changed
- `🛢️db/🗄️storage/🔐️writer/🦀️.rs` — guard wake hooks, `pinned_guard_mut`, `WalWriterKey::generation`, conformance module.
- `🛢️db/🗄️storage/🔐️writer/🧬️schema/🔣️.json`, `🧫️fixtures/🌐️remote-guard/🔣️.json` — `WalWriterFenceV1`.
- `🛢️db/🗄️storage/🔐️writer/🧪️tests/🔬️fence-conformance/🦀️.rs` — new, cross-backend laws.
- `🛢️db/🗄️storage/🐘️postgres/🦀️.rs` + unit tests — advisory-lock fence, session-run mutations, BIGINT casts, pool in runtime.
- `🛢️db/🗄️storage/🌐️neo4j/🦀️.rs` + unit tests — lease fence, fenced txns, empty-bytes create.
- `🛢️db/🗄️storage/🧵️driver-runtime/🦀️.rs` — `within`, `detach_periodic`.
- `🛢️db/🗄️storage/🪶️sqlite/🧪️tests/🔬️sqlite-storage-unit/🦀️.rs` — reads the new contract.
- `💻️os/📦️packages/🦀️rust/📜️script.ts` + `📋️project.json` — fence oracle; `wal-writer-fence-live` verb.
- `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc` — `⚖️gate🔐️wal-writer-fence🐳️live`.
- `🌎️hub/🏗️bootstrap/🦀️.rs` — `SocketGrantKeyV1`, `admit_document`, `DocumentSocketGrantReceiptV1`, `consume_document_socket_grant`, `consume_scoped_directory_socket_grant`.
- `🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs`, `🧪️tests/🔬️standalone/🦀️.rs` — laws on binding-keyed admissions (H2's 78/79 idempotent-resend law kept).
- `🌎️hub/📇️directory/🧬️schema/🔣️.json` (removed `SocketGrantReceiptV1`), `📇️directory/🧫️fixtures/🔏️document-execution-target-lease-v1/🔣️.json`.
- `🌎️hub/📦️packages/🦀️rust/📜️script.ts` — receipt parsers, stale-protocol probes, MCP/lease oracles, renamed exact law.
- `🧰️framework/…/💻️os/🟦️.ts`, `🔨️modules/🏪️store/👷️worker/🟦️.ts`, `🧪️tests/🧪️space-artifact-creation-owner/🟦️.ts`, `🧪️tests/🧪️backbone-envelope-io/🟦️.ts`.
- `🧰️framework/…/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs` + unit tests, `📇️directory/🧬️schema/🔣️.json` + unit tests, `🧫️fixtures/📇️directory/🌐️browser-document-open-v1.json`.
- `🧰️framework/…/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs` + unit tests.

## Gaps
1. **Neo4j hub restart waits for the lease.** The hub never shuts down its `db::Database` (`🏗️bootstrap` main ends
   after the router drains; `Database::shutdown(&mut self)` is never called, because `HubState` holds it as
   `Arc<db::Database>`). SQLite and Postgres writers are released by the OS on exit. A Neo4j lease stays live until its
   TTL, so a restart within 15 s gets `Conflict` for documents that were open. Measured: after the e2e, the last hub's
   lease was still held (`holder` set, `expiresAtMs` = claim + 15 000), and the neo4j e2e took 306 s vs 254–262 s.
   Fix: shut the Database down in hub main after `saga_drain`. This is a hub-lifecycle change I did not make.
2. Pre-existing, untouched: 3 db page-writer/mock laws (`db-exact-1.txt`), the `🗿️artifact` source marker in
   `wal-writer-authority-check`, and `execution-target-lease-check` (manifest stage).
3. The `wal-writer-fence-live` nx verb is registered and the laws it runs pass (`fence-live-2.txt`), but I have not
   run it through nx.
4. Containers `h3-pg` (compose volume `semio-hub_hub-postgres` kept) and `h3-neo4j` have been removed. No hub process of mine is running. The `os-hub` for peers is `.tmp-ticket/wp-h3/bin/os-hub`; c7 has been told.
