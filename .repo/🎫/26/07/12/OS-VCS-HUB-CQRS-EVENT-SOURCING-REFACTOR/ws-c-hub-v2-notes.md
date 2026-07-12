# WS-C — Hub v2 implementation notes

## Scope delivered
- Kernel sync frames (`framework/core/rs/lib.rs`, 🔖Sync region → new `🔖HubProtocol` subregion).
- Full hub rewrite (`framework/product/os/hub/rs/bin.rs`) + `Cargo.toml`.
- Deleted `framework/product/os/hub/postgres/` (Postgres schema ported to SQLite).

## Storage decision
Chose `Arc<Mutex<rusqlite::Connection>>` with **synchronous** storage methods over a dedicated
command thread. Rationale: rusqlite calls are short and synchronous, hub concurrency is modest, and
the `MutexGuard` is never held across `.await`, so document-actor futures stay `Send`. Documented
inline on `HubStorage`. Single connection also sidesteps the `file::memory:?cache=shared` dance:
`:memory:` works within one `HubState` because there is exactly one connection.

## SQLite schema (bootstrapped via execute_batch, CREATE TABLE IF NOT EXISTS)
- `node(id TEXT PK, parent_id TEXT → node(id) ON DELETE CASCADE, name TEXT, kind TEXT)`
- `document(id TEXT PK, schema TEXT, snapshot TEXT/JSON, version INTEGER DEFAULT 0)`
- `document_op(id TEXT PK, document_id TEXT, version INTEGER, actor TEXT, envelope TEXT/JSON, created_at INTEGER)`
  — `id` = envelope OperationId, so `INSERT OR IGNORE` gives op-id dedupe.
- `session(id TEXT PK, document_id TEXT, client_name TEXT, created_at INTEGER)` — table exists per schema intent; not heavily wired.
- `share_token(token TEXT PK, document_id TEXT, created_at INTEGER)`

Dropped `node_id` from `document` and `document_id` from `node` (task's explicit node/document column
lists have no cross-link column); the postgres FK linkage was replaced by the flat VFS the task specifies.

## Versioning model
Single monotonic `version` per document (the `document.version` column), incremented once per newly
appended op AND once per envelope PUT. Ops record their assigned version. This matches the legacy
single-counter behaviour, removes the two-counter confusion, and keeps `Welcome.version` / `Ack.version`
/ `ops?since=` all on one axis.

## Bug fix — op-append version precondition removed
`POST /documents/{id}/ops` and WS `Ops` no longer require `body.version == document.version`. The
server assigns the version inside the `DocumentActor`, dedupes by op id, and returns the assigned
version. CAS (409/`Conflict`) is kept ONLY on `PUT /documents/{id}/envelope` and WS `PutEnvelope`.

## Architecture
- `DocumentActor` (tokio task) per document: owns `OpDag`, version counter, in-memory op cache, seen-id
  set, presence roster, and a per-document `broadcast::Sender<HubServerFrame>`. Spawned lazily on first
  access (open-on-demand), loading state from SQLite.
- `DocumentHandle` (clonable mpsc sender) is the only way handlers touch a document.
- `HubState { storage, actors: DashMap<String, DocumentHandle>, admin_token }`.
- Presence lives inside each actor's state (folded in, not a global map).

## WS handler (duplex, primary transport)
Hello → auth (token) → Subscribe → Welcome{version, envelope only when since==0, presence, backlog}.
Inbound Ops → append → per-new-op `Ack` back to the sender; the actor also broadcasts `Ops` to ALL
subscribers **including the origin** (simplest; origin dedupes by op id). PutEnvelope → CAS; success is
broadcast as `SnapshotReplaced` by the actor, failure returns `Conflict` to just that socket. Presence →
broadcast `Presence{peers}`. Disconnect/Bye → `presence_leave` (only broadcasts if the peer had joined).
REST `GET /ops?since=` kept as fallback/debug.

## Auth-lite
`share_token` table. `POST /documents/{id}/share` requires `OS_HUB_ADMIN_TOKEN` bearer (403 if unset).
Tokenless document = open (dev default); once any token issued, REST + WS `Hello.token` must present a
valid bearer for that document.

## VFS durable
`POST /nodes` (parentId/name/kind) + `GET /nodes?parent=` backed by the `node` table. No parent → roots.

## STUDIO_CATALOG_URIS
NOT deleted here — it lives in `framework/product/os/core/rs/lib.rs:1553` (WS-E's file, not WS-C's).
Reported to the parent instead of touched, per task instructions.

## Dependencies
Removed `sqlx-core` + `sqlx-postgres` and the unused `semio-framework-sync` dep (bin.rs never imported
it; keeping it coupled the hub to the mid-flight WS-A/vcs build which currently does not compile).
Added `rusqlite = { version = "0.38.0", features = ["bundled"] }` (matches vcs / os-core / compose).
Dev-dep `tokio-tungstenite = "0.26"` for the real end-to-end WS test.

Stale `sqlx-*` lines remain in Cargo.lock files + target/ fingerprints; orphaned (nothing references
sqlx now), pruned automatically on next full resolve.

## Tests (all green: `cargo test -p os-hub` → 9 passed)
- `ws_duplex_fan_out` — genuine two-socket test: A Hello+Ops, B receives `Ops{version:1,[op-1],origin:"A"}`.
- `persistence_round_trip_from_file` — append 3 ops, drop state, rebuild against same temp DB file, `ops_since(0)` returns full log.
- `op_id_dedupe` — same envelope twice → one row, second append `is_new=false`.
- `snapshot_cas_conflict` — stale-version PUT returns Err(current), state uncorrupted.
- `op_append_never_version_conflicts` — the bug fix: two appends after a version bump both succeed.
- `rest_append_increments_version`, `rest_ops_since_filters`, `nodes_create_and_list`, `share_token_gates_access`.

## Deferred / for other workstreams
- launch.json `OS_HUB_DB` env registration → WS-E dev host.
- TS backbone-worker consuming these frames → WS-E.
- `STUDIO_CATALOG_URIS` deletion → WS-E (os/core).
