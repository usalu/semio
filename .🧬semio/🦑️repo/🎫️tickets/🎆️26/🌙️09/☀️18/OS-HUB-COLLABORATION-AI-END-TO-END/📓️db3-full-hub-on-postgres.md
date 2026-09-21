# DB3 — the WHOLE hub on Postgres (and the directory on Neo4j)

Slice DB3 of ticket 26/09/18, 2026-09-21. Everything below is measured on this host; nothing is
pending. Predecessor: DB2 (`📓️db2-postgres-neo4j-live-lanes.md`), whose **D4** was this slice's root
problem and whose §8 gap 1 and gap 2 are both retired here.

**Headline.** `OS_HUB_STORAGE_BACKEND=postgres` no longer aborts the hub. A hub boots on **7671**
with *both* durable halves on a real PostgreSQL 16, opens every readiness gate that does not
need a published catalog, signs two users in, creates a space, issues and redeems an invite, fans out
live socket frames, and survives `SIGTERM` + restart with identity, membership and credentials
intact: **42 of 42 probe checks pass, 0 fail** (§5a). The same hub with
`OS_HUB_DIRECTORY_BACKEND=neo4j` does the same. Three defects were found and
fixed at the root, two of them invisible to every existing test and to `cargo check`.

## 1. Root design

**Where a tokio-bound driver may be polled — and where it may not.**

A `PostgresStorage`/`Neo4jStorage` call does not run on the caller's executor. It becomes a typed
`DbIoTask` submitted to the one process `semio_framework_async::WorkerPool`, and an *async-native*
backend's future is driven by `db_io_poll_async_driver` as a `Job` on `Lane::Io`
(`🗄️storage/🦀️.rs:4166`). Those workers are plain `std::thread`s with no foreign runtime context at
all — the `⏳️async` crate says so in its first line ("No `tokio` in this crate") and spawns them
itself (`⏳️async/🦀️.rs:1868`). `sqlx` demands its Tokio context at **poll** time, finds none, and
aborts the process. That is D4, and `db_storage`'s own module doc asserted the opposite ("the calling
task's own executor … drives it"), which is why nobody looked.

The design is therefore a **seam, not a patch**, and it is an interface first (AGENTS.md: external
libraries behind an interface):

1. `db_storage` — which names no concrete runtime — declares
   **`DbIoAsyncDriverRuntime`** (`🗄️storage/🦀️.rs:2435`), one method:
   `fn detach(&self, future: DbIoAsyncDriverFuture) -> DbIoAsyncDriverFuture`. It takes the driver's
   future away and returns a *bridge* future that is safe to poll anywhere.
2. `DbIoTaskExecutor` gains **`driver_runtime()`** (`🗄️storage/🦀️.rs:2481`), defaulting to `None`:
   a backend declares whether its driver is bound to a runtime. Every backend in the crate except
   the two external ones answers `None`, so `fs`, `sqlite` and `memory` are untouched.
3. `DbIoAsyncTaskLease::start_on_lane_io` (`🗄️storage/🦀️.rs:4537-4541`) asks **before**
   `drive_async` consumes the executor, and puts the bridge — never the driver's future — into the
   task slot. This is the single choke point; there is no second path.
4. The one implementation is **`db_storage_driver_runtime`**
   (`🗄️storage/🧵️driver-runtime/🦀️.rs`), compiled only under `postgres`/`neo4j` — the features that
   link a driver needing one — and the only place in the `db` crate family that names `tokio`. It
   owns a bounded multi-thread runtime (2 worker threads, `SEMIO_DB_IO_DRIVER_THREADS` clamped to
   `1..=8`, 1 MiB stacks, threads named `semio-db-io-driver-N`), `spawn`s the driver future onto it,
   and hands back a `semio_framework_async::oneshot` receiver as the bridge — a repo-owned primitive
   whose `poll` only reads a value and registers a waker.
5. The runtime is a process singleton created on first use and **never shut down**, by design: a
   detached turn holds the backend registry's executor for its whole duration, so tearing the runtime
   down under it would destroy state the registry owns. That also makes the bridge's "rendezvous
   closed" branch reachable *only* through a panic inside the detached turn, which is exactly what
   `db_io_poll_async_driver`'s existing `catch_unwind` already converts into the task's `Panic`
   fault, so the bridge re-raises rather than fabricating an executor it does not own
   (`🧵️driver-runtime/🦀️.rs:72-83`).

Same seam for Neo4j: `neo4rs` is equally tokio-bound, and `Neo4jDbIoExecutor` returns the same
runtime (`🗄️storage/🌐️neo4j/🦀️.rs:948`). The **directory** halves are a different story and need no
seam: their `sqlx`/`neo4rs` calls are awaited on the server's own `#[tokio::main]` runtime, which is
why DB2's directory-only hub worked while the document half did not.

One more call had to move: `PostgresDbIoExecutor::close_backend_step` used to poll `PgPool::close()`
directly from `Lane::Io`. It now detaches it through `detach_unit` and polls the rendezvous with the
caller's own waker (`🗄️storage/🐘️postgres/🦀️.rs:823-834`).

## 2. Seams landed (file:line)

| what | where |
|---|---|
| the interface | `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:2425-2440` (`DbIoAsyncDriverOutput`, `DbIoAsyncDriverRuntime`) |
| the backend's declaration | `…/🗄️storage/🦀️.rs:2478-2484` (`DbIoTaskExecutor::driver_runtime`) |
| the one choke point | `…/🗄️storage/🦀️.rs:4535-4542` (`DbIoAsyncTaskLease::start_on_lane_io`) |
| corrected module doc | `…/🗄️storage/🦀️.rs:33-36` (the "the calling task's own executor drives it" claim) |
| the implementation | `…/🗄️storage/🧵️driver-runtime/🦀️.rs` (**new**, 155 lines incl. laws) — `detach` :53, `shared` :113, `detach_unit` :122 |
| module registration | `…/🛢️db/📦️packages/🦀️rust/🦀️.rs:44-46` (`cfg(all(not(wasm32), any(postgres, neo4j)))`) |
| the tokio dep, feature-gated | `…/🛢️db/📦️packages/🦀️rust/Cargo.toml` — `postgres = ["dep:sqlx", "dep:tokio"]`, `neo4j = ["dep:neo4rs", "dep:tokio"]`, `rt,rt-multi-thread,net,time` only |
| postgres backend | `…/🗄️storage/🐘️postgres/🦀️.rs:791` (`driver_runtime`), `:823-834` (detached pool close), `:8-24` (corrected module doc) |
| neo4j backend | `…/🗄️storage/🌐️neo4j/🦀️.rs:948` (`driver_runtime`) |

## 3. Defects found and fixed

### D4 (DB2's) — the whole hub aborted on `OS_HUB_STORAGE_BACKEND=postgres`

Root-caused in §1, fixed by the seam in §2. **Before**: `db2-pg-hub-7671.txt` —
`thread 'semio-pool-worker-0' panicked at sqlx-core-0.8.6/src/rt/mod.rs:42: this functionality
requires a Tokio context`, `/readyz` never answered. **After**: `db3-pg-hub-7671-run1.txt` —
`storage: {"ready": true}`, 39 PASS, and the restart step green.

### D5 — the Neo4j space-administration page was a hard `500` on every Neo4j hub ever booted

`🌎️hub/📇️directory/🌐️neo4j/🦀️.rs:1900-1901` (`list_admin_space_summaries_page`).

Found by booting a hub on the Neo4j directory (§4b) — the first time anyone in this ticket did.
`GET /directory/spaces/{id}` answered `500` and the membership roster came back empty, while
`GET /directory/spaces` (a different query) answered `200` with a correct `memberCount`, and the
graph itself held all three `MEMBER_OF` edges. So nothing was lost; the page could not be read.

The cause is **invalid Cypher that had never once been executed**:

```
CALL { WITH s OPTIONAL MATCH (event:DirectoryEvent {spaceId: s.id})
       RETURN coalesce(max(event.recordedAt), s.createdAt) AS updatedAt }
```

Neo4j refuses it with `Neo.ClientError.Statement.SyntaxError: Aggregation column contains implicit
grouping expressions … Illegal expression(s): s.createdAt` — `s.createdAt` next to `max(...)` in the
same `RETURN` is an implicit grouping key. Fix: return `max(event.recordedAt) AS lastEventAt` from the
subquery and apply `coalesce(lastEventAt, s.createdAt)` in the outer `RETURN`. Verified against the
live Neo4j by hand (`cypher-shell`, four rows with correct `updatedAt`) and then by the new law (§3).

This is the class of defect the DB2 corpus was built to catch, one level up: not a write that behaves
differently, but a *read* that no test ever issues. Hence the new corpus in §3.

### D6 — every directory backend fault was a silent `500` with nothing logged

`🌎️hub/🏗️bootstrap/🦀️.rs:2034-2047` (`HubState::directory_fault`, new) and the five reads of
`build_directory_space_administration_page_v1`.

`directory_error_status` (`:5893`) maps `DirectoryError::Backend(detail)` to
`INTERNAL_SERVER_ERROR` and **drops `detail` on the floor**. The route emitted no trace record at
all, so a driver-level fault on a read route is indistinguishable from a handler bug — D5 cost a full
bisection (three hub boots and a hand-written probe) that one log line would have ended. `directory_fault`
notes `server.directory.backend` with the backend's own message and the route name, then defers to
`directory_error_status` for the status. Only the `Backend` arm is reported; every other arm already
names itself through its status code. Measured working: the capture in `db3-neo4j-space-page.txt`
is the D5 diagnosis, printed by this helper.

Scope note: the five call sites converted are the ones in the space-administration page. The other
~100 `.map_err(directory_error_status)` sites across `🏗️bootstrap` are unchanged — converting them
all would have collided with the peers editing that file right now, and is listed as a gap (§6).

### D7 — the `db` crate's whole test target did not compile (pre-existing, unblocked on the way)

`🛢️db/📦️packages/🦀️rust/Cargo.toml` (`[dev-dependencies]`).

`cargo check -p semio-framework-os-kernel-db --all-targets` failed on **`main` before any of my
edits** (verified by running it against the unmodified tree) with
`error[E0432]: unresolved import semio_framework_pack` at `🦀️.rs:12`. Cause: cargo derives a
dependency's extern name from the **lib target** name unless the dependency key renames it, and
`semio-framework-pack`'s lib target is named `pack` — which this crate has already bound to the
os-kernel (`extern crate semio_framework_os_kernel as pack`). Fixed by naming the key
`semio_framework_pack` with an explicit `package = "semio-framework-pack"`, which is what makes cargo
rename it. Without this no law in the `db` crate could be run at all, including this slice's own.

## 4. Laws

All five ran; none is written-but-unexecuted.

| law | where | lanes | capture |
|---|---|---|---|
| `detached_work_runs_on_a_driver_thread_inside_a_tokio_context` — the detached body runs on a `semio-db-io-driver-*` thread, **not** the caller's, sees a Tokio context there, and the caller learns the outcome through the rendezvous while having none itself | `🗄️storage/🧵️driver-runtime/🦀️.rs:135` | db crate | `db3-driver-runtime-lane.txt` — **2 passed** |
| `driver_thread_count_stays_inside_its_band` — the env override cannot leave `1..=8` | `…/🧵️driver-runtime/🦀️.rs:160` | db crate | same |
| `document_storage_opens_over_real_postgres_off_the_pool_workers` — **the D4 regression**: a `PostgresStorage` + a `db::Database` over it, opened through the `WorkerPool` I/O lane against a real `postgres:16-alpine`. Fails by process abort if the seam is ever bypassed | `🌎️hub/📇️directory/🐘️postgres/🧪️tests/🔬️unit/🦀️.rs:328` | postgres | `db3-pg-lane-run1.txt` |
| `assert_space_administration_read_surface_v1` — **the D5 regression**, backend-neutral: the exact five directory reads `/directory/spaces/{id}` performs (`list_admin_space_summaries_page`, `get_role`, `list_space_administration_members_page`, `list_space_administration_invites_page`, `list_document_descriptors_page`), asserted on their values | `🌎️hub/📇️directory/🧪️tests/🔮️backend-corpus/🦀️.rs:103` | **sqlite + postgres + neo4j** | `db3-read-surface-all-lanes.txt` — **3 passed** |

Lane results, `CARGO_TARGET_DIR=⚡️cache/cargo/target-db3`, both against live containers:

| lane | capture | result |
|---|---|---|
| `directory::postgres` (12 laws, incl. both new ones) | `db3-pg-lane-run1.txt` | **12 passed, 0 failed**, 19.34 s |
| `directory::neo4j` (7 laws, incl. the new read-surface law) | `db3-neo4j-lane-run1.txt` | **7 passed, 0 failed**, 98.58 s |
| `read_surface_v1_holds_on` across all three backends | `db3-read-surface-all-lanes.txt` | **3 passed, 0 failed**, 21.66 s |
| `db_storage_driver_runtime` | `db3-driver-runtime-lane.txt` | **2 passed, 0 failed** |
| `cargo check -p semio-hub --no-default-features --features sqlite,postgres,neo4j --all-targets` | `db3-hub-check-all-targets.txt` | **0 errors, 251 warnings** (warnings are the proof it really type-checked), 1m21s |
| `cargo check -p semio-framework-os-kernel-db --features postgres,neo4j --all-targets` | — | **0 errors** (was 1 error before D7) |
| `bun nx run os-hub-ts:typecheck` | `db3-hub-typecheck.txt` | **30 errors, all pre-existing** and none in the files this slice touched (3 in `📜️script.ts` are `creation.ready is possibly undefined` at :10420, the rest are in `🔌️plugin`). Recorded, not introduced. |

The hub-boot law the brief asked for is **not** a `cargo test`: `connect_db`, `connect_artifact_cas`
and the readiness gates live in the `os-hub` **bin**, and preamble rule 26 reserves bin test builds of
`semio-hub` for the coordinator. The equivalent proof is §5, run on a real process, captured.

## 5. Live proof

Binary `⚡️cache/cargo/target-db3/debug/os-hub`, built `--no-default-features --features postgres,neo4j`
(`db3-hub-bin-build{,2,3}.txt`), and once more with `native-artifact-execution` added
(`db3-hub-bin-build-nae.txt`, 4m41s). Databases: `$T/🔣️db3-compose.yaml` — `postgres:16-alpine` on
127.0.0.1:**5434**, `neo4j:5-community` on **7689/7476**, named volumes (bind mounts under
`/Users/ueli/Documents` are refused by this Docker Desktop, measured by DB2 §8.4 and unchanged here).
Boot script `$T/📜️db3-postgres-hub-boot.sh`, probe `$T/🐍️h1b-hub-runtime-probe.ts` (parameterised
`--binary/--port/--data` by DB2). Data roots `chmod 700`, `OS_HUB_CREDENTIAL_SIGN_IN=true`, port 7671.

### 5a. Both halves on PostgreSQL — `db3-pg-hub-7671-nae.txt` (42 PASS, 0 FAIL)

Run twice: first with `--features postgres,neo4j` (`db3-pg-hub-7671-run1.txt`, 39 PASS / 1 FAIL) and
then with `--features postgres,neo4j,native-artifact-execution` (`db3-pg-hub-7671-nae.txt`,
**42 PASS, 0 FAIL**). The single FAIL of the first run was the closed gate's reason code — a binary
without `native-artifact-execution` correctly names `native-artifact-execution-feature-not-compiled`
instead of `trusted-catalog-never-published-in-this-data-root` (DB2 §5a hit the same and called it a
non-defect). With the feature compiled in, the hub answers the expected
`trusted-catalog-never-published-in-this-data-root` and the probe is clean, so the artifact authority
reads its own state correctly over a Postgres-backed CAS. The table below is the first run; the
second differs only in that row.

| # | check | observed |
|---|---|---|
| 1 | `GET /healthz` | **200**, `"live"`, fresh `runId` `630b8bf2…` |
| 2 | `GET /readyz` | **503**, `status: not-ready`, **one** `blockedBy` gate; `directory`, **`storage`**, `artifactCasBarrier`, `artifactPublication`, `artifactCasSweeper`, `adminAssets` all `ready: true` |
| 3 | `POST /auth/sessions` (ada) | **200**, token shape ok; `/auth/sessions/me` **200** → `01a0c357-6d09-…`; TTL honoured |
| 4 | create space / create invite / bo sign-in / redeem | **202 / 202 / 200 / 200** |
| 5 | `/directory/socket/v1` upgrade, rename command, fan-out | upgrade **OPEN**, command **202**, **4 live frames** to the joined peer; after departure the peer receives nothing more while a further command is still **202** |
| 6 | sign-out, then the signed-out capability | **204**, then **401** |
| 7 | `SIGTERM` + restart on the same Postgres | `/healthz` **200** with a **new** `runId`; credential re-mint **200**; **the space survived: true**; **the space page: 200**; **the membership survived: `[[01a0c357-6d09-…,"author"],[01a0c357-75c5-…,"spectator"]]`**; presence `[]` (ephemeral by contract) |
| 8 | session expiry after the 60 s floor | **401** |
| 9 | auth rate-limit bucket | **429**, `"rate-limited"`, positive `retry-after`, and a *correct* password still **429** while empty |

**Gates, both runs:** `directory`, `storage`, `artifactCasBarrier`, `artifactPublication`,
`artifactCasSweeper`, `adminAssets` all `ready: true`; exactly one `blockedBy` entry, which is
`artifactAuthority` and which cannot open without a published trusted catalog in this data root.
`/readyz` therefore stays `503` by contract, not by fault.

### 5b. Documents on PostgreSQL, directory on Neo4j — `db3-neo4j-directory-7671-run2.txt`

Same probe, `OS_HUB_DIRECTORY_BACKEND=neo4j` + `OS_HUB_DIRECTORY_NEO4J_URI=bolt://127.0.0.1:7689`,
`--skip-expiry`. **41 PASS, 1 FAIL** (the same reason-code line). Before the D5 fix
(`db3-neo4j-directory-7671.txt`) the same run was **39 PASS, 3 FAIL** — `the space page is readable
after the restart: 500` and `the membership survived the restart: []`. Both are now PASS, with the
roster byte-exact. **This is the first Neo4j-backed hub booted in this ticket**, retiring DB2 §8 gap 2.

### 5c. What actually moved into the databases (measured, not inferred)

`DROP SCHEMA public CASCADE` on the all-Postgres run reported **39 objects**, including the document
store (`db_wal_segment`, `db_snapshot_generation`, `db_payload`, `db_catalog_root`, `db_index_run`,
`db_lease`), the whole artifact chunk CAS (`hub_artifact_cas_object/ledger_journal/ledger_head/
reservation/reservation_object/reference/reference_object/delete_lease/barrier_identity/coordinator/
space_fence`), checkpoints (`hub_artifact_checkpoint`, `hub_artifact_checkpoint_private`,
`hub_artifact_retention`, `hub_checkpoint_publication_receipt`), and the whole directory
(`hub_user`, `hub_space`, `hub_space_membership`, `hub_auth_session`, `hub_sync_session`,
`hub_share_grant`, `hub_space_invite`, `hub_directory_event`, …). With the Neo4j directory the same
Postgres database holds exactly the document store + `hub_artifact_cas_*` and nothing else — verified
live with `\dt` (9 tables).

## 6. What stays on the filesystem, and why

The hub's data root after a full Postgres run contains **only**:

```
$OS_HUB_DATA/
├── extension-modules/
└── instance/{authority,projections,blobs,sessions}
```

Those four are the **server-product** stores (`🌎️hub/🗄️stores/🦀️.rs` — `HubAuthorityStore`,
`HubProjectionStore`, `HubBlobStore`, `HubSessionStore`). They are **filesystem-only by design, not by
omission**: they are opened from `server::storage::StorageProfile`, which has exactly two variants —
`Ephemeral` and `Embedded { data_dir }` (`🧰️framework/🛍️products/🖥️server/🔨️modules/🗄️storage/🦀️.rs:59-66`)
— and its doc states the rule out loud: *"Clustered and hosted shapes are added when a backend
implements them, never as a speculative option flag on these."* There is no `postgres` variant to
select, no database lane to point at, and no env var that would change it; each one writes `tokio::fs`
directly and stamps `format.json` (`🗄️stores/🦀️.rs:284`). So a `postgres` hub still needs a durable
`OS_HUB_DATA`, and a data root and its database must be backed up and restored **together** — now
stated in `🌎️hub/README.md`'s data-root section.

Everything else the brief named **is** on Postgres and was exercised: checkpoints and socket grants
(directory tables, §5c), **document sockets** (the `db_*` document store — the exact half D4 broke),
sessions (`hub_auth_session`/`hub_sync_session`, proven to survive a restart), the auth/credential
path, and presence (ephemeral by contract, proven to *not* survive). `trusted-catalog/` is also
filesystem (`current.json` + generations), which is why the artifact-authority gate is unaffected by
the storage backend.

## 7. Registration and documentation

- `🌎️hub/📦️packages/🦀️rust/📜️script.ts` — **`build-dev-postgres`** (stages `os-hub` with
  `--features postgres,neo4j` into `dist/build-dev-postgres`, so the driverless default build is never
  overwritten and a running hub is never replaced in place) and a **`postgres` segment for `dev`**
  (`applyPostgresBackendEnvironment`: requires `OS_HUB_DATABASE_URL`, sets `OS_HUB_STORAGE_BACKEND`,
  points the directory at the same database unless told otherwise, refuses `neo4j` without its URI).
  `DirectoryLiveLanesScript`'s filters updated for the new corpus name.
- `🌎️hub/🚀️local-bootstrap/🏃️execution/🟦️.ts` — `HUB_DEV_POSTGRES_BINARY_TARGET` +
  `hubDevPostgresBinaryPath`, same on-demand staging shape as `hubDevBinaryPath`.
- `🌎️hub/📦️packages/🦀️rust/📋️project.json` — `build-dev-postgres` and `dev-postgres` targets
  (108 targets, JSON re-parsed).
- `.vscode/launch.json` — `🛠️dev🐘️os-hub🗄️postgres` (`3_dev`, order 387.04, between `🛠️dev🗄️os-hub`
  and `🛠️dev🗄️os-hub🛡️admin`) and `📦️build-dev🐘️os-hub🗄️postgres` (`4_build`, order 206.16005,
  straight after `📦️build-dev🗄️os-hub`). Re-parsed: 392 configurations, no new duplicate order.
- `🌎️hub/README.md` — a new *"How a `postgres`/`neo4j` document store is actually driven"* subsection
  (the runtime-thread contract, `SEMIO_DB_IO_DRIVER_THREADS`, why the directory halves do not need the
  seam, why the runtime is never shut down), the `dev-postgres` launch route under the directory
  table, and the data-root section now saying exactly what moves into the database and what stays on
  disk.

## 8. Gaps (honest)

1. **No artifact was created on the Postgres hub, and `/readyz` never reached `200`.** With
   `native-artifact-execution` compiled in, the hub names the right blocking reason
   (`trusted-catalog-never-published-in-this-data-root`) and every other gate is open — but no
   trusted catalog was published, so no document was opened end-to-end. Publishing one is not a copy
   job here: with a Postgres storage backend the catalog's chunks go **into Postgres**, so DB2's and
   HS1's trick of copying an `fs` data root does not carry it, and the publish chain is the jco/wasm
   pipeline the preamble flags as expensive and policy-sensitive (rule 28). This is the single
   biggest remaining step for "the whole hub on Postgres, including artifacts".
2. **`OS_HUB_STORAGE_BACKEND=neo4j` was never booted.** The Neo4j *document* store now goes through
   the same seam and compiles, and the Neo4j *directory* is proven live (§5b), but no hub ran with
   Neo4j as its document store, so its own queries are still unexercised at runtime.
3. **The `Backend`-fault log (D6) covers one route.** Five call sites in the space-administration page
   now report; the other ~100 `.map_err(directory_error_status)` sites in `🏗️bootstrap/🦀️.rs` still
   turn a backend message into a bare `500`. Converting them all belongs in a slice that owns that
   file, not one sharing it with two live peers.
4. **The driver runtime's saturation behaviour is unmeasured.** Two threads served every run here
   comfortably, but no test drives enough concurrent database I/O to show where 2 stops being enough,
   and `SEMIO_DB_IO_DRIVER_THREADS` has only its clamp law.
5. **The bridge's panic branch is untested.** `DetachedDriver`'s "rendezvous closed" arm re-raises so
   the existing `catch_unwind` can fault the task; no law forces a detached turn to panic, so that
   path is reasoned, not run.
6. **`dev postgres` was not launched end-to-end.** The verb, the Nx targets and the launch rows exist
   and the script type-checks, but the proof in §5 was run through `📜️db3-postgres-hub-boot.sh` and
   the H1b probe, not through `bun nx run os-hub:dev-postgres` — that route also publishes a trusted
   catalog, which is gap 1.
7. **The stack is torn down.** `semio-db3-postgres-1` / `semio-db3-neo4j-1` were removed with
   `docker compose … down -v` after the last capture, so re-running §5 means re-running
   `📜️db3-postgres-hub-boot.sh --fresh`, which brings them back. The live **lanes** need nothing of
   mine: each lane fixture starts its own container (DB2 §2).
8. **A stray `db2-probe` container (`92dfd5a3a0c6`, `postgres:16-alpine`, host port 5499)** is still
   running and is not this slice's; it was left untouched, as DB2 also recorded.

## 9. Files changed

Source:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs` — `DbIoAsyncDriverOutput`,
  `DbIoAsyncDriverRuntime`, `DbIoTaskExecutor::driver_runtime`, the `start_on_lane_io` choke point,
  corrected module doc.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🧵️driver-runtime/🦀️.rs` — **new**.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🐘️postgres/🦀️.rs` — `driver_runtime`,
  detached pool close, corrected module doc.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🌐️neo4j/🦀️.rs` — `driver_runtime`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📦️packages/🦀️rust/🦀️.rs` — module registration.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📦️packages/🦀️rust/Cargo.toml` — feature-gated `tokio`,
  the D7 dev-dependency rename, corrected comments.
- `🌎️hub/📇️directory/🌐️neo4j/🦀️.rs` — D5 Cypher fix.
- `🌎️hub/🏗️bootstrap/🦀️.rs` — D6 `HubState::directory_fault` + five call sites.
- `🌎️hub/📇️directory/🧪️tests/🔮️backend-corpus/🦀️.rs` — the space-administration read corpus.
- `🌎️hub/📇️directory/{🐘️postgres,🌐️neo4j,🪶️sqlite}/🧪️tests/🔬️unit/🦀️.rs` — the corpus law on all
  three lanes; the D4 regression law on postgres.
- `🌎️hub/📦️packages/🦀️rust/📜️script.ts`, `…/📋️project.json`,
  `🌎️hub/🚀️local-bootstrap/🏃️execution/🟦️.ts`, `.vscode/launch.json`, `🌎️hub/README.md` — §7.

Ticket-local:

- `$T/🔣️db3-compose.yaml`, `$T/📜️db3-postgres-hub-boot.sh`,
  `$T/🐍️db3-space-page-diagnose.ts` — **new**.
- `$T/🗑️generated/db3-*.txt` — every capture named above.

Containers started by this slice (still up): `semio-db3-postgres-1`, `semio-db3-neo4j-1`. Stop with
`docker compose -f "$T/🔣️db3-compose.yaml" down -v`.
