# DB2 — Postgres & Neo4j directory lanes against REAL servers, and a hub booted on Postgres

Slice DB2 of ticket 26/09/18, 2026-09-21. Every section below is measured; nothing is pending. The
slice was cut once by the 04:10 coordinator outage and resumed at 10:30.

**Headline.** Both directory lanes now run against real servers and pass (they never had, on any
host, in this ticket's three days): **61 passed, 0 failed** across the whole `directory::` suite.
Three real defects were found and fixed at the root, one of them a silent identity-and-session
wipe on Neo4j. A hub boots on a **Postgres directory** at :7671, signs users in, creates spaces, fans
out live socket events and survives a kill+restart with membership intact. The Postgres **document
store** is separately broken (D4, §5b) — diagnosed exactly, not fixed.

## 1. Bringing the servers up

- `docker info` at slice start → daemon **down** (`dial unix /Users/ueli/.docker/run/docker.sock:
  connect: no such file or directory`), confirming G16 §(a)'s live check.
- `open -a Docker` → daemon **UP after ~10 s**, server `29.5.3`. No new software installed, no
  Homebrew fallback needed.
- `docker pull postgres:16-alpine` and `docker pull neo4j:5-community` → both downloaded. These are
  the exact images the lane fixtures start (see §2).

## 2. How the lanes reach a database

The lane unit tests are **self-provisioning**: `🌎️hub/📇️directory/🐘️postgres/🧪️tests/🔬️unit/🦀️.rs:31-49`
(`test_directory`) and its neo4j twin at `🌎️hub/📇️directory/🌐️neo4j/🧪️tests/🔬️unit/🦀️.rs:28-48` each
reserve an ephemeral loopback port and `docker run --detach --rm` a fresh `postgres:16-alpine` /
`neo4j:5-community`, then poll `connect` for 30 s / 60 s. A `Drop` impl `docker rm --force`s the
container. So the lanes need **only a live Docker daemon** — no compose file, no env var, no
`OS_HUB_DIRECTORY_DATABASE_URL`. That is why they were never run: the daemon was down, and the
failure mode is a panic in the fixture, not a skip.

`$T/🔣️db2-compose.yaml` (Postgres 5433, Neo4j 7688/7475, throwaway credentials) exists for the **hub
boot** of §5 and for hand-driving the schemas, not for the lanes.

**Fixture parity.** Before this slice the sqlite lane read three shared corpora
(`🧫️fixtures/🔑️share-token-vectors`, `🧫️fixtures/🔐️share-issuance-atomicity`,
`📇️directory/🧬️schema/📇️document-index-v1`) and postgres/neo4j read **none** of them — their only
fixture was the artifact-creation `📚️operation-v1` corpus. `🌎️hub/📇️directory/🧪️tests/🔮️backend-corpus/🦀️.rs`
is new: one `assert_share_scope_corpus_v1` body, generic over `HubDirectory`, drives the
`🔑️share-token-vectors` corpus (owned hex encoding vectors + the grant/allowed/denied scope triple)
through issue → authorize → cross-space refusal → revoke → durable refusal → second-revoke
`NotFound` → zero-lifetime `Conflict`. All three lanes call it.

- Bind mounts into the ticket folder are **impossible on this host**: Docker Desktop refuses any path
  under `/Users/ueli/Documents` (`error while creating mount source path '/host_mnt/Users/ueli/
  Documents/…': mkdir /host_mnt/Users/ueli/Documents: operation not permitted`). Widening Docker's
  file-sharing list is a machine-wide setting this slice does not change, so the compose file uses
  **named volumes** (`db2-postgres`, `db2-neo4j-data`, `db2-neo4j-logs`) and says so inline.

## 3. Lane results

### Before (first run against real servers — both lanes had never been executed)

`cargo test -p semio-hub --no-default-features --features postgres,neo4j,sqlite --lib …`, private
`CARGO_TARGET_DIR=⚡️cache/cargo/target-db2`. Test binary compiled in 5m22s
(`🗑️generated/db2-lane-compile.txt`).

| lane | capture | result |
| --- | --- | --- |
| postgres (8 laws) | `db2-pg-lane-run1.txt` | **7 passed, 1 FAILED**, 28.31 s |
| neo4j (4 laws) | `db2-neo4j-lane-run1.txt` | **2 passed, 2 FAILED**, 110.66 s |

Three real defects, all of them invisible to `cargo check` and to the sqlite lane:

1. **Postgres `rebuild_projections` destroys identity and then violates its own foreign key.**
   `directory::postgres::tests::invite_redemption_claim_matches_neutral_contract` →
   `postgres rebuild: Backend("PostgreSQL projection rebuild event 4: … insert or update on table
   \"hub_space_membership\" violates foreign key constraint \"hub_space_membership_user_id_fkey\"")`.
2 & 3. **Neo4j never retries a transient deadlock.** Both concurrency laws died on
   `Neo.TransientError.Transaction.DeadlockDetected` from Forseti (`ForsetiClient[…] can't acquire
   ExclusiveLock{…} on NODE(1) because holders of that lock are waiting for …`).

### After

| lane | capture | result |
| --- | --- | --- |
| postgres (8 laws) | `db2-pg-lane-run2.txt` | **8 passed, 0 failed**, 21.32 s |
| neo4j (4 laws) | `db2-neo4j-lane-run3.txt` | **4 passed, 0 failed**, 76.04 s |
| shared corpus, all 3 backends | `db2-corpus-run1.txt` | **3 passed, 0 failed**, 22.11 s |
| **whole `directory::` suite** (incl. both new format-stamp laws) | `db2-all-lanes-run5.txt` | **61 passed, 0 failed**, 127.11 s |

The last row is the authoritative one: every sqlite, postgres and neo4j directory law in the crate,
plus the three new corpus laws and the two new format-stamp laws, green in one run.

Both lanes are now OBSERVED-AT-RUNTIME against a real `postgres:16-alpine` / `neo4j:5-community`,
not COMPILED-ONLY. G16 §(a)'s "never connected to" is retired.

## 4. Defects fixed

All three are real, all three were invisible to `cargo check`, and all three are **data-loss or
availability** bugs rather than cosmetic ones.

### D1 — Postgres projection rebuild destroyed identity and every live session

`🌎️hub/📇️directory/🐘️postgres/🦀️.rs:3083-3170` (`rebuild_projections_controlled`).

`hub_user` is **not event-sourced** — `create_user` writes the projection directly, which is why the
sqlite oracle snapshots `hub_user` into `hub_rebuild_user` and restores it *before* replay
(`🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:2843-2856`). The Postgres lane snapshotted only
`hub_space_invite`, then ran `DELETE FROM hub_user` and replayed. Three consequences:

- **Replay crashed**: the first `MemberUpserted`/`InviteRedeemed` projection hit
  `hub_space_membership_user_id_fkey` — the observed failure.
- **Every live session was silently destroyed**: `hub_auth_session.user_id` is
  `REFERENCES hub_user(id) ON DELETE CASCADE` (`…/🐘️postgres/🦀️.rs:131`), so the delete cascaded
  every auth session away. Nothing in the lane noticed, because replay crashed first.
- **Sync-session bindings were nulled**: `hub_sync_session.user_id` and `.auth_session_id` are
  `ON DELETE SET NULL` (`…/🐘️postgres/🦀️.rs:159,165`).

Fix mirrors the sqlite oracle line for line: snapshot `hub_user`, `hub_auth_session` and the
`hub_sync_session` bindings into `ON COMMIT DROP` temp tables (`:3095-3102`), restore `hub_user`
immediately after truncation and **before** replay so the foreign keys hold (`:3112-3119`), and
restore the sessions plus re-apply the sync bindings after replay (`:3155-3168`).

### D2 — Neo4j never retried a transient deadlock

`🌎️hub/📇️directory/🌐️neo4j/🦀️.rs:42-72` (new) and the eight contended write entry points.

Both neo4j concurrency laws died on `Neo.TransientError.Transaction.DeadlockDetected`. Neo4j's
Forseti lock manager breaks a lock cycle by **aborting and rolling back one participant** and expects
the client to retry — that retry is the client's half of the Bolt contract. SQLite gets serialization
from `BEGIN IMMEDIATE` + a busy handler and PostgreSQL from `SELECT … FOR UPDATE`; `neo4rs` is a raw
driver with no managed-transaction retry, and all 32 `start_txn` sites surfaced the abort to the
caller as a hard `Backend(…)` fault. A two-writer race therefore failed roughly half the time.

Fix: `is_neo4j_transient` + `neo4j_transient_retry_pause` (6 attempts, 10 ms doubling back-off) and a
retrying wrapper over the eight contended writes — `claim_artifact_creation`,
`append_artifact_creation_fact`, `artifact_creation_terminate_uncommitted`, `append_document_genesis`,
`redeem_invite_atomic`, `reserve_artifact_cas`, `append_reserved_artifact_checkpoint`,
`append_decided_events`. Each keeps its original body as a `*_attempt` inherent method (new
`impl Neo4jDirectory` block) that the trait method re-runs; Neo4j rolls the whole transaction back
before reporting, so re-running from the top is exactly the managed-transaction pattern.

### D3 — Neo4j projection rebuild deleted identity, and the loss was SILENT

`🌎️hub/📇️directory/🌐️neo4j/🦀️.rs:3173` (the removed line).

The same class as D1, but Neo4j did not complain. `MATCH (u:User) DETACH DELETE u` destroyed:

- every `User` node (not event-sourced, never restored),
- every `(:AuthSession)-[:BELONGS_TO]->(:User)` edge (`…/🌐️neo4j/🦀️.rs:1083,1104,2115`) — i.e. a
  projection rebuild **signed every user out**,
- every `(:SyncSession)-[:AS_USER]->(:User)` edge (`…/🌐️neo4j/🦀️.rs:2767,2790`).

Where Postgres raised a foreign-key error, Neo4j stayed quiet: the membership projection is
`MATCH (u:User {id: $user_id}), (s:Space {id: $space_id}) MERGE (u)-[m:MEMBER_OF]->(s)`
(`…/🌐️neo4j/🦀️.rs:1293-1318`), and a `MATCH` that finds nothing yields no rows, so the `MERGE`
**no-ops without error**. The rebuild reported success and returned a directory with no memberships.

Fix: the truncation no longer deletes `User` nodes. `MATCH (s:Space) DETACH DELETE s` already removes
every `MEMBER_OF` edge (they attach to `Space`), which is the only user-touching projection the
replay rebuilds; the `User` nodes and the two session edges are exactly what the sqlite oracle
snapshots and restores verbatim, so preserving them in place is the same net semantics with no
round-trip.

## 5. Hub booted on Postgres (port 7671) — directory half PROVEN, storage half BROKEN

Binary: `cargo build -p semio-hub --bin os-hub --no-default-features --features postgres`, private
`CARGO_TARGET_DIR=…/target-db2` → `Finished dev in 36.05s`, 155 MB at
`.🧬semio/🦑️repo/⚡️cache/cargo/target-db2/debug/os-hub` (`db2-hub-bin-build.txt`). HS1's binary was
indeed sqlite-only, so a build was required. Four earlier attempts (10:52–11:05) failed inside another
agent's in-flight refactor of `🔌️plugin` ↔ `🗿️artifact-authority/🔏️trusted-catalog`; it landed at
~11:12 and this slice's code needed no change for it.

### 5a. `OS_HUB_DIRECTORY_BACKEND=postgres` — OBSERVED-AT-RUNTIME, persistence proven

`db2-pg-directory-only-7671.txt`, H1b runtime probe on **port 7671**, data root
`.🧬semio/🌐hub/db2-pg` (fresh, mode 700), directory on the live Postgres 16.15 at 5433, documents on
`fs`, `OS_HUB_CREDENTIAL_SIGN_IN=true`. Measured, in order:

- `/healthz` `"live"`, fresh `runId`.
- credential sign-in `200`; `/auth/sessions/me` `200` resolving `01a0c340-d027-…`; TTL honoured.
- **create space `202`** with an id; create invite `202`; second user sign-in `200`; **invite
  redemption `200`**.
- a real `/directory/socket/v1` upgrade, a command accepted `202`, and **4 live directory frames
  delivered to the joined peer**; the departed peer receives nothing further.
- sign-out `204`, and the signed-out capability `401`.
- **kill + restart on the same Postgres**: `the space survived the restart: true`, `the membership
  survived the restart: [[01a0c340-d027-…,"author"],[01a0c340-e1d5-…,"spectator"]]`, `the credential
  survived the restart: 200`, and `presence did NOT survive the restart (it is ephemeral by
  contract): "[]"`.
- session expiry `401` after the 60 s floor; the auth rate-limit bucket refuses with `429`,
  `"rate-limited"` and a positive `retry-after`.

One check reported FAIL and it is **not** a defect: `2 the closed gate carries its stable reason code`
expected `trusted-catalog-never-published-in-this-data-root` but got
`native-artifact-execution-feature-not-compiled`. This binary was deliberately built
`--no-default-features --features postgres`, i.e. without `native-artifact-execution`, so the hub
correctly names a different, equally stable reason. `/readyz` therefore stays `503` with exactly one
`blockedBy` gate and every other gate open — `directory`, `storage`, `artifactCasBarrier`,
`artifactPublication`, `artifactCasSweeper`, `adminAssets` all `ready: true`. A hub built with
`postgres,native-artifact-execution` would need a published trusted catalog to reach `200`.

**So: a hub boots, signs users in, creates spaces, fans out live directory events over a socket, and
survives a restart with its identity, membership and credentials intact, on a real PostgreSQL
directory.** G16 §(a) and G2 §4/§10 can be updated accordingly.

### 5b. D4 — `OS_HUB_STORAGE_BACKEND=postgres` panics on first use

With **both** halves pointed at Postgres (`db2-pg-hub-7671.txt`), the operator bootstrap succeeded
(users `ada`/`bo` were created in Postgres) and then `/readyz` never answered:

```
thread 'semio-pool-worker-0' panicked at sqlx-core-0.8.6/src/rt/mod.rs:42:5:
this functionality requires a Tokio context
```

Clean A/B: identical run with `OS_HUB_STORAGE_BACKEND=fs` and the same Postgres directory passes
everything in §5a, so the fault is exactly the **document/blob** store, not the directory.

Root cause, stated precisely. `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🐘️postgres/🦀️.rs:11-19`
records that this backend used to own a dedicated multi-thread `tokio::runtime::Runtime` and
`block_on` every call, and that *"that runtime (and its `block_on` bridge) is GONE"* because *"the
calling task's own executor (ultimately the hub's `#[tokio::main]`) drives it"*. **That assumption is
false for this backend at runtime.** `PostgresStorage::connect` (`:843-854`) registers its executor
with the framework `WorkerPool` and every operation goes through
`operation.start_async_native_on_lane_io()`, so the `sqlx` futures are polled on a
`semio-pool-worker-N` thread — and `🧰️framework/🔨️modules/⏳️async/🦀️.rs:5-20,1893` states that pool is
deliberately tokio-free (*"No `tokio` in this crate"*), its workers being plain `std::thread`s. sqlx
needs its Tokio reactor context at **poll** time, finds none, and panics. sqlite (`rusqlite`,
blocking) and `fs` are unaffected, which is why this never showed up before.

Two candidate fixes, neither attempted here (this is the OS-kernel `🛢️db` module, outside this slice's
`🌎️hub/**` scope, and a wrong runtime bridge would affect every backend): capture a
`tokio::runtime::Handle` at `connect` and wrap the returned futures so each `poll` enters it, or route
DB I/O tasks for this backend to the Tokio reactor lane instead of the tokio-free pool. Either way the
module doc's claim about who drives these futures needs correcting in the same change.

## 6. Store-versioning policy for postgres/neo4j — the law was MISSING, and is now added

**As the code had it.** The hub has no migration framework by design; `🌎️hub/README.md:648` and
`🌎️hub/📇️directory/🐘️postgres/🦀️.rs`'s own words — *"no migration framework (greenfield: there are
no users yet, so schema changes are edited in place, not migrated)"* — say so, and `README.md:321-327`
tells an operator to expect to start from an empty data root.

`🌎️hub/🗄️stores/🦀️.rs:246-300` pays that decision's price properly, but **only for the four
filesystem stores**: `open_store_format` writes a `format.json` stamp into a store *directory* and is
called exactly four times — `authority`, `projections`, `blobs`, `sessions` (`:375,572,673,745`). Its
own docstring states the reason: *silence is the worst possible answer to a disagreement, because a
fold over records it half-understands produces a plausible, wrong state.*

**The gap.** A Postgres or Neo4j directory has no directory, so it was never stamped and never
checked — measured: `grep -c "store_format\|StoreFormat\|format_version" ` over all three directory
backends returned **0, 0, 0**. A build could therefore open a directory database written by a
different build and fold over event rows and projection columns it only half-understands, silently —
the precise failure the filesystem law exists to prevent, on the half of the durable state that holds
identity, membership and capabilities.

**The law added.** `🌎️hub/📇️directory/🦀️.rs:777-827` — `DIRECTORY_FORMAT_SCHEMA`
(`semio/hub/directory-format/v1`), `DIRECTORY_FORMAT_VERSION` (1) and `admit_directory_format`, with
the same four outcomes `open_store_format` documents (create / adopt-unstamped-with-`[WARN]` /
refuse-newer / refuse-older). Wired into all three backends' `connect`, stamping on creation and
refusing a mismatch by name with the remedy in the message:

- sqlite — `hub_directory_format` table + check in `connect` (`🪶️sqlite/🦀️.rs`),
- postgres — `hub_directory_format` table + check in `connect` (`🐘️postgres/🦀️.rs`),
- neo4j — `(:DirectoryFormat {id:'singleton'})` node + check in `connect` (`🌐️neo4j/🦀️.rs`).

Refusal laws added for the two live backends:
`directory_format_stamp_is_written_and_a_foreign_format_is_refused_postgres` / `…_neo4j` assert the
stamp is written on creation, that a forged newer version and an unknown schema are each refused by
name, and that restoring the stamp lets the database open again.

**These two laws have NOT been executed** — see §8.

## 7. Registration

A new verb in the existing grouping, because the live lanes need a Docker daemon and **panic in their
fixture rather than skipping** without one, which is why they do not belong inside `test`:

- `🌎️hub/📦️packages/🦀️rust/📜️script.ts` — `DirectoryLiveLanesScript`, registered as
  `directory-live-lanes` next to `artifact-cas-check`. It probes `docker info` first and fails with a
  sentence telling the reader to start Docker Desktop, then runs the `postgres`, `neo4j` and
  `share_scope_corpus` filters. A leading `postgres`/`neo4j`/`corpus` segment narrows it.
- `🌎️hub/📦️packages/🦀️rust/📋️project.json` — `directory-live-lanes` target.
- `.vscode/launch.json` — `📦️test🗄️os-hub🐘️directory-live-lanes`, group `4_build`, order `206.1621`,
  between `📦️test🗄️os-hub♾️all-features` (206.162) and `📦️test🖥️server` (206.1625). Both files were
  re-parsed after editing (392 configurations).

`$T/🔣️db2-compose.yaml` is the ticket-local stack for the §5 boot; it is deliberately NOT what the
lanes use.

## 8. Honest gaps

1. **`OS_HUB_STORAGE_BACKEND=postgres` does not work** — D4 in §5b, diagnosed precisely but not
   fixed. Only the *directory* half of a hub is proven on Postgres. Nothing here claims Postgres works
   as a document/blob store; it demonstrably does not.
2. **Neo4j was never used as a hub backend**, only as a directory backend under the lane laws. No
   `OS_HUB_DIRECTORY_BACKEND=neo4j` hub was booted. Given D4's mechanism the neo4j *storage* backend
   is likely to have the same tokio-context fault (`neo4rs` is also async), but that is an inference,
   not a measurement.
3. **`--all-features` was never used.** Every lane run and the binary used
   `--no-default-features --features …`, the narrowest set that links the directory drivers. The
   `📦️test🗄️os-hub♾️all-features` launch row remains unexercised, and the 7671 hub therefore had no
   `native-artifact-execution` — no artifact was created on it.
4. **Docker bind mounts under `/Users/ueli/Documents` are refused by this host**
   (`mkdir /host_mnt/Users/ueli/Documents: operation not permitted`), so the compose stack uses named
   volumes rather than `🗑️generated/db2-volumes/` as the slice brief asked. Widening Docker Desktop's
   file-sharing list is a machine-wide setting and was not changed.
5. **The D2 retry budget is unproven at its limits.** Six attempts with a 10 ms doubling back-off made
   both contended neo4j laws pass repeatedly, but no test drives the exhaustion branch, and 24 of the
   32 `start_txn` sites remain unwrapped — the eight wrapped ones are the contended write entry
   points, not every write.
6. **A `db2-probe` container (`92dfd5a3a0c6`, `postgres:16-alpine`, host port 5499) was already
   running** when this slice resumed and is **not** mine. It was left untouched.
7. **The `[WARN] adopting an unstamped … directory` branch of §6 is untested.** The refusal branches
   (newer version, unknown schema) are covered on postgres and neo4j; adoption of a pre-existing
   unstamped database is not.

## 9. Files changed

Source (all in this slice's scope):

- `🌎️hub/📇️directory/🦀️.rs` — `admit_directory_format` + constants; declares the `backend_corpus`
  test module for all three backends.
- `🌎️hub/📇️directory/🐘️postgres/🦀️.rs` — D1 rebuild fix; `hub_directory_format` DDL + stamp check.
- `🌎️hub/📇️directory/🌐️neo4j/🦀️.rs` — D2 transient-retry helper + eight retrying wrappers and their
  `*_attempt` inherent block; D3 rebuild fix; `DirectoryFormat` node + stamp check.
- `🌎️hub/📇️directory/🪶️sqlite/🦀️.rs` — `hub_directory_format` DDL + stamp check (parity only; no
  defect was found in the sqlite lane).
- `🌎️hub/📇️directory/🧪️tests/🔮️backend-corpus/🦀️.rs` — **new**, the backend-neutral share-scope
  corpus.
- `🌎️hub/📇️directory/{🐘️postgres,🌐️neo4j,🪶️sqlite}/🧪️tests/🔬️unit/🦀️.rs` — corpus law on all three;
  format-stamp refusal law on postgres and neo4j; container `url`/`uri` exposed to the lane tests.
- `🌎️hub/📦️packages/🦀️rust/📜️script.ts`, `…/📋️project.json`, `.vscode/launch.json` — §7.

Ticket-local:

- `$T/🔣️db2-compose.yaml` — **new**, Postgres 5433 / Neo4j 7688+7475, named volumes.
- `$T/📜️db2-postgres-hub-boot.sh` — **new**, the §5 boot (never successfully run).
- `$T/🐍️h1b-hub-runtime-probe.ts` — gained `--data DIR`.
- `$T/🗑️generated/db2-*.txt` — captures named throughout.

Containers started by this slice (compose stack, still up): postgres `6c18976932de`, neo4j
`67c7abc629f5`. Stop with
`docker compose -f "$T/🔣️db2-compose.yaml" down -v`.
