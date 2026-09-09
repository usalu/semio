# Hub Backend Database, Presence, and Services Audit

**Date:** 2026-09-09  
**Scope:** Working tree only (`/Users/ueli/Documents/semio`)  
**Question:** Does the server hub backend currently work end-to-end with database, presence, and related services?

## Executive Verdict

**Partially — only on the default development stack.** The hub binary (`os-hub`) implements a monolithic Axum server with real SQLite directory persistence, filesystem document storage, local-bootstrap authentication, SocketGrant admission, document-socket Welcome/Session/Presence, and durable sync-session records. That path is heavily unit-tested but **not** proven as a single integrated boot-to-presence proof across alternate backends or production auth.

**Not end-to-end today:**

- Production mode (hard startup failure: no `IdentityAssertionVerifier` wired in `main`)
- PostgreSQL / Neo4j at the **running hub process** level (backend code exists; bin-level and live e2e are explicit nonclaims)
- `🧰️framework/🛍️products/🖥️server` product integration (metadata lists hub as consumer; hub does not import or use it)
- TypeScript `startHub()` harness (spawns without inherited bootstrap pipe; uses unused `OS_HUB_ADMIN_TOKEN`)
- Ephemeral runtime state (SocketGrant ledger, presence map) across hub restart

**Confidence:** High for code-structure conclusions; **no tests were executed in this audit** (read-only inspection).

---

## Architecture Snapshot

```
┌─────────────────────────────────────────────────────────────────┐
│  os-hub (🌎️hub/📦️packages/🦀️rust/🚀️bin.rs)                      │
│  Monolithic Axum router — NOT framework 🖥️server gateway        │
├─────────────────────────────────────────────────────────────────┤
│  HubState                                                       │
│    directory ──► HubDirectories (sqlite | postgres | neo4j)     │
│    db ─────────► db::Database (fs | sqlite | postgres | neo4j)  │
│    artifact_cas ► fs | sqlite | postgres | neo4j                │
│    socket_grants ► in-memory SocketGrantLedgerV1                │
│    presence ─────► in-memory ShardedMap (ephemeral roster)      │
│    fanout ───────► per-document broadcast lanes                   │
└─────────────────────────────────────────────────────────────────┘
```

Hub depends on `semio-framework-os-kernel` (`directory`), `semio-framework-os-kernel-db` (`db`), and `semio-framework-replication` (`protocol`). It does **not** depend on `semio-framework-product-server`.

---

## Boot and Readiness

### Entry (`🚀️bin.rs` `main`)

| Step | Behavior |
|------|----------|
| Mode | `OS_HUB_MODE` or inferred: loopback bind → `development`, else `production` |
| Auth (dev) | `InheritedLocalBootstrapTransport::open_inherited()` on **fd 3** |
| Auth (prod) | Requires `IdentityAssertionVerifier` — **always `None` in `main` today** |
| Directory | `connect_directory()` → default `sqlite`, `{OS_HUB_DATA}/directory.db` |
| Document DB | `connect_db()` → default `fs`, `{OS_HUB_DATA}/db/` |
| Artifact CAS | `connect_artifact_cas()` → follows `OS_HUB_STORAGE_BACKEND` |
| Crash cleanup | `directory.close_all_sync_sessions()` before serving |
| Local bootstrap task | `serve_local_bootstrap(transport, directory, control)` when dev |

### Readiness (`/readyz`, `hub_readiness`)

`status: "ready"` requires **all** of:

- `authentication.bootstrapReady` (dev: local bootstrap pipe ready; prod: verifier present)
- `artifactAuthority.ready` (trusted catalog loaded under data root)
- `adminAssets.ready` (`OS_HUB_ADMIN_DIR` dist exists)
- `artifact_cas_barrier.ready` (always set `true` at startup)
- sweeper healthy (checked per request)

`directory.ready` and `storage.ready` are **always reported true** at construction — they are not probe-tested.

### Production blocker (concrete)

```rust
// 🚀️bin.rs ~8300
let identity_verifier: Option<Arc<dyn IdentityAssertionVerifier>> = None;
```

`validate_auth_startup` for `HubMode::Production` rejects `verifier.is_none()` (`🚀️bin.rs` ~2134–2137). Production hub **cannot start** as written.

---

## Storage Backends

### Document / blob storage (`connect_db`, `OS_HUB_STORAGE_BACKEND`)

| Backend | Default | Env | Feature gate |
|---------|---------|-----|--------------|
| `fs` | **yes** | — | always |
| `sqlite` | no | `OS_HUB_DB_SQLITE` | `sqlite` |
| `postgres` | no | `OS_HUB_DATABASE_URL` | `postgres` |
| `neo4j` | no | `OS_HUB_NEO4J_URI`, user, password | `neo4j` |

Implementation: `🚀️bin.rs` `connect_db` (~8189–8219). Uses shared `hub_worker_pool()` for blocking IO dispatch.

### Directory / identity (`connect_directory`, `OS_HUB_DIRECTORY_BACKEND`)

| Backend | Default | Path / env | Feature gate |
|---------|---------|------------|--------------|
| `sqlite` | **yes** | `{OS_HUB_DATA}/directory.db` | `sqlite` |
| `postgres` | no | `OS_HUB_DIRECTORY_DATABASE_URL` | `postgres` |
| `neo4j` | no | `OS_HUB_DIRECTORY_NEO4J_URI` (+ user/password) | `neo4j` |

Implementations:

- `🌎️hub/📇️directory/🪶️sqlite/🦀️.rs`
- `🌎️hub/📇️directory/🐘️postgres/🦀️.rs`
- `🌎️hub/📇️directory/🌐️neo4j/🦀️.rs`

Directory and document storage backends are **independently selectable** (intentional split-brain avoidance comment in `connect_db`).

### Artifact chunk CAS (`connect_artifact_cas`)

Mirrors `OS_HUB_STORAGE_BACKEND` with parallel fs/sqlite/postgres/neo4j stores under `{data_dir}/artifact-cas/v1` for fs.

### Default dev split

Out of the box (no env overrides):

- **Directory:** SQLite `directory.db`
- **Documents:** filesystem tree `{OS_HUB_DATA}/db/`
- **CAS:** filesystem `{OS_HUB_DATA}/artifact-cas/v1/`

This is a valid but asymmetric default.

---

## Auth

### Schemas and capabilities

- `🌎️hub/🔐️auth/🧬️schema/` — capability wire formats (Session, SocketGrant, Share, Invite)
- Directory module: `SessionCapability`, `SocketGrantCapability`, `AuthSessionRecord`, `AuthSessionIssue` (`📇️directory/🦀️.rs`)

### Development: local bootstrap

Full implementation: `🌎️hub/🚀️local-bootstrap/🦀️.rs`

- Inherited handle fd **3** (Unix) / duplicated handle (Windows)
- HMAC-framed JSON protocol (`semio.hub.local-bootstrap/v1`)
- `serve_local_bootstrap` issues `AuthSession` via `directory.issue_auth_session`
- Creates SSO-linked users on first issue (`identity_provider: semio.local.bootstrap/v1`)

Launcher contract (TypeScript): `🌎️hub/📦️packages/🦀️rust/📜️script.ts` `startLocalHub()` — spawns hub with `stdio[3]` pipe, performs initialize/hello/issue handshake.

### Production

- Requires `IdentityAssertionVerifier` trait (`📇️directory/🦀️.rs` ~923)
- Test-only impls exist in `🧪️tests/🔬️bin-unit/🦀️.rs` and `📇️directory/🧪️tests/🔬️unit/🦀️.rs`
- **No production adapter registered in `main`**

### Admin REST

- `authenticate_admin_principal` — live `SessionCapability` + `OS_HUB_ADMIN_SUBJECTS` digest match
- `OS_HUB_ADMIN_TOKEN` is referenced only in `🌎️hub/📦️packages/🟦️typescript/🟦️.ts` `startHub()` and **deleted** in `📜️script.ts`; **not read by Rust hub**

---

## SocketGrant, Welcome, Session, Presence

### SocketGrant ledger

- In-process `SocketGrantLedgerV1` in `HubState` (`🚀️bin.rs` ~807–1075)
- States: Pending → Consumed → live registration
- **Not persisted** — restart clears all grants
- HTTP mint routes: `/directory/socket-grants`, scoped variants, document open-plan exchange

### Document WebSocket (`/spaces/{space_id}/documents/{id}/socket/v1`)

Handler: `handle_ws` (`🚀️bin.rs` ~4081+)

Sequence (contract §C7.3):

1. Consume SocketGrant (header)
2. Client `SocketHelloV1`
3. `db.hello()` → storage-backed **Welcome** frames
4. Bootstrap frames from hello session
5. **Session** frame (actor + color)
6. Install `PresenceLeaseSlot` (peer initially empty)
7. `record_sync_session_open` → **SQLite/Postgres/Neo4j** `hub_sync_session`
8. Loop: mutations, presence refresh/expire, fanout, admin kick

Presence:

- In-memory `HubState.presence` (`ShardedMap`)
- `publish_presence_delta` → `ServerFrame::Presence` on document fanout
- TTL: `PRESENCE_LEASE_TTL_MS`, server clock (test clock injectable)
- Admin connections view joins `list_active_sync_sessions()` + presence map (`connection_view`, ~5113)

### Directory WebSocket (`/directory/socket/v1`)

Handler: `handle_directory_ws_v1` (`🚀️bin.rs` ~6345+)

- SocketGrant admission + `SocketHelloV1`
- **No Welcome/Session frames** — JSON `DirectoryStreamMessage` event replay + live stream
- Scoped variant filters by document membership

### Protocol dependency

Replication frames: `🧰️framework/🔨️modules/📡️replication/` (`ServerFrame::Welcome`, `Session`, `Presence`, `ClientFrame::SocketHelloV1`)

---

## Framework Server Product (`🧰️framework/🛍️products/🖥️server`)

| Module | Status vs hub |
|--------|----------------|
| `📡️gateway` | Parallel design; hub reimplements gateway concerns in `bin.rs` |
| `🗄️storage` | `StorageProfile::Embedded` only; in-memory reference backends |
| `🎭️authority` | Not used by hub |
| `🛡️policy` | Not used by hub (hub uses `db::security` directly in WS handler) |
| `🧬️contract` | Not imported by hub crate |

`🔨️modules/🔣️.json` lists hub as `productionConsumers` — **aspirational / not wired**.

---

## Test Coverage Matrix

| Layer | What is tested | Backend | Boot path |
|-------|----------------|---------|-----------|
| Directory sqlite unit | CRUD, sessions, events, projections | SQLite file | In-process |
| Directory postgres unit | Invites, events, creation (Docker) | Postgres 16 | In-process |
| Directory neo4j unit | Invites, redemption atomicity (Docker) | Neo4j 5 | In-process |
| Hub bin-unit (~100 tests) | SocketGrant, presence, Welcome/Session, admin, GIS, checkpoints | **SqliteDirectory only** | In-process `HubState` / spawned Axum |
| `📜️script.ts` laws | Local bootstrap, readiness, presence journeys, GIS collaboration | Mostly sqlite+fs | **`startLocalHub` (fd 3)** |
| `🧪️integration/🟦️.ts` | Schema/HMAC oracles; one `HUB_E2E=1` route-404 check | — | **`startHub` (broken path)** |

### Explicit nonclaims (fixtures / script)

`postgres-live-qualified`, `neo4j-live-qualified` in:

- `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧫️fixtures/🤝️gis-map-collaboration/🔣️.json`
- `🌎️hub/📦️packages/🦀️rust/📜️script.ts` (~8905)

Bin-unit postgres/neo4j coverage is **source-structure law** only for directory event-page append (`directory_event_page_v1_append_admission_is_transactional_for_sqlite_postgres_and_neo4j`), not a live hub boot.

### `startHub()` harness gap

`🌎️hub/📦️packages/🟦️typescript/🟦️.ts`:

- Spawns binary with default bind `0.0.0.0` → **production mode**
- No stdio pipe → **local bootstrap fails**
- No verifier → **production auth fails**
- Waits on `/admin/api/overview` with raw `adminToken` string (not a valid `SessionCapability`)

Only consumer: `🧪️tests/🤝️integration/🟦️.ts` (gated `HUB_E2E=1`). Real e2e uses `startLocalHub` in `📜️script.ts`.

---

## What *Does* Work (default dev stack)

When launched via `startLocalHub` / launch.json dev path with:

- `OS_HUB_MODE=development`, `OS_HUB_BIND=127.0.0.1`
- Inherited bootstrap pipe (fd 3)
- Default features: `sqlite`, `native-artifact-execution`
- Trusted catalog present under data root (for full readiness)
- Admin dist built

…the following chain is **implemented and covered by bin-unit / script laws**:

1. Local bootstrap → `AuthSession` in `hub_auth_session` (SQLite)
2. REST directory commands / admin intents (session bearer)
3. SocketGrant mint + single consume
4. Document WS: Hello → Welcome (db) → Session → presence slot
5. `hub_sync_session` durable open/close
6. Presence normalization, roster bounds, admin removal recovery (sqlite reopen test)
7. Directory event socket replay + live fanout

---

## Remaining Concrete Blockers

| # | Blocker | Evidence path |
|---|---------|----------------|
| 1 | **Production hub cannot start** — `identity_verifier` hardcoded `None` | `🌎️hub/📦️packages/🦀️rust/🚀️bin.rs` ~8300, ~2134–2137 |
| 2 | **No `IdentityAssertionVerifier` production adapter** in tree | Only test impls in `🧪️tests/` |
| 3 | **Framework server product unused** — duplicate gateway/storage stacks | Hub `Cargo.toml` vs `🖥️server/`; no imports |
| 4 | **Postgres/Neo4j not qualified at running hub** | Explicit nonclaims; bin tests sqlite-only; docker tests stop at directory trait |
| 5 | **`startHub` TS harness non-functional** for real auth/boot | `🟦️typescript/🟦️.ts` vs `📜️script.ts` `startLocalHub` |
| 6 | **SocketGrant + presence ephemeral** — lost on restart | `SocketGrantLedgerV1`, `HubState.presence` in `🚀️bin.rs` |
| 7 | **Readiness hard-deps on artifact catalog + admin dist** | `hub_readiness`, `configured_artifact_authority` |
| 8 | **Split default storage** (fs docs + sqlite directory) untested as deployment profile | `connect_db` default `fs`, `connect_directory` default `sqlite` |
| 9 | **No single cross-language “boot hub → presence on postgres” proof** | Test matrix above |
| 10 | **`publicSessionIssuance: false` forever** — no HTTP session mint outside bootstrap | `HubAuthenticationReadinessV1` ~2101 |

---

## Highest-Leverage Next Implementation Lane

**Lane: Production-capable boot + one integrated hub-process proof on the existing dev stack**

Rationale: The largest gap between “lots of unit tests” and “end-to-end works” is not another directory backend feature — it is **(a)** wiring a real `IdentityAssertionVerifier` (or dev-only documented entry) in `main`, and **(b)** replacing/fixing `startHub` to use the `startLocalHub` inherited-pipe pattern, then adding **one** gated process test that:

1. Boots `os-hub` via pipe
2. Issues credential → session
3. Opens document socket → observes Welcome + Session + Presence
4. Asserts `hub_sync_session` row + directory event on reconnect

This unblocks production startup, fixes the broken TS harness, and turns the already-implemented sqlite/fs path into an auditable e2e proof without waiting for postgres/neo4j hub integration.

**Secondary lane (after above):** Postgres live hub boot (`OS_HUB_DIRECTORY_BACKEND=postgres` + `OS_HUB_STORAGE_BACKEND=postgres`) with docker fixture — currently an explicit nonclaim.

**Defer:** Migrating `bin.rs` onto `🖥️server` gateway — large refactor, no current wiring; does not unblock the default path.

---

## File Index (primary)

| Concern | Path |
|---------|------|
| Hub binary / router / WS | `🌎️hub/📦️packages/🦀️rust/🚀️bin.rs` |
| Hub library surface | `🌎️hub/📦️packages/🦀️rust/🦀️.rs` |
| Cargo features / deps | `🌎️hub/📦️packages/🦀️rust/Cargo.toml` |
| Local bootstrap | `🌎️hub/🚀️local-bootstrap/🦀️.rs` |
| Directory trait + capabilities | `🌎️hub/📇️directory/🦀️.rs` |
| SQLite directory | `🌎️hub/📇️directory/🪶️sqlite/🦀️.rs` |
| Postgres directory | `🌎️hub/📇️directory/🐘️postgres/🦀️.rs` |
| Neo4j directory | `🌎️hub/📇️directory/🌐️neo4j/🦀️.rs` |
| Auth schemas | `🌎️hub/🔐️auth/🧬️schema/` |
| Bin unit tests | `🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs` |
| TS integration (minimal) | `🌎️hub/🧪️tests/🤝️integration/🟦️.ts` |
| Real boot + laws | `🌎️hub/📦️packages/🦀️rust/📜️script.ts` |
| Broken TS harness | `🌎️hub/📦️packages/🟦️typescript/🟦️.ts` |
| Replication frames | `🧰️framework/🔨️modules/📡️replication/` |
| Framework server (unused) | `🧰️framework/🛍️products/🖥️server/` |
| Document DB engine | `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/` |

---

## Audit Method

- Read-only inspection of working tree sources and fixtures
- No `git` write operations
- No test execution (results not claimed)
- Ticket markdown used only as navigation hints, not evidentiary proof
