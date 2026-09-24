# Hub Backend Audit — 2026-09-23

Read-only audit of `🌎️hub` for end-to-end OS ↔ hub ↔ collaboration readiness. Command outputs live under `🗑️generated/hub/` in this ticket.

---

## Executive summary

`os-hub` is a **single Rust binary** (Axum HTTP + WebSockets) that embeds auth, directory, document/event store, presence fan-out, artifact authority, and GIS inference. Default dev topology is **zero-touch local**: filesystem event store + SQLite directory, no external DB. The codebase is **substantially implemented** (not stubs), but **fresh boots stay `/readyz` not-ready** until a trusted catalog is published, **quick tests fail** on one artifact-authority law, **hub TS typecheck fails**, and **`build-dev` staging is extremely slow** when the full dependency graph compiles under lock contention.

---

## 1. How to run locally

### 1.1 Binary and topology

| Item | Value |
|------|--------|
| Binary | `os-hub` — `🌎️hub/🏗️bootstrap/🦀️.rs` (Cargo package `semio-hub`) |
| Default port | `8787` (`OS_HUB_PORT`) |
| Default data root | `./.🧬semio/🌐hub/` (`OS_HUB_DATA`) |
| Document store (default) | `OS_HUB_STORAGE_BACKEND=fs` → `{OS_HUB_DATA}/db/` (event-sourced WAL) |
| Directory (default) | `OS_HUB_DIRECTORY_BACKEND=sqlite` → `{OS_HUB_DATA}/directory.db` (auto-created) |
| Instance stores (always on disk) | `{OS_HUB_DATA}/instance/{authority,projections,blobs,sessions}/` |

Two **modes** (`validate_auth_startup` in `🌎️hub/🏗️bootstrap/🦀️.rs`):

| Mode | How | Requirements |
|------|-----|--------------|
| **Development** | Supervised child of local-bootstrap (fd 3 pipe) | Loopback bind; `LocalBootstrapTransport` on inherited fd 3 |
| **Production** | Plain process / systemd / Docker | `OS_HUB_MODE=production`, `OS_HUB_CREDENTIAL_SIGN_IN=true`, `OS_HUB_ADMIN_SUBJECTS=…`; network bind also needs `OS_HUB_ALLOWED_ORIGINS` + `OS_HUB_TRUSTED_FORWARDING=proxy` |

Bare `os-hub` without fd 3 **exits** in development mode (by design).

### 1.2 launch.json (VS Code)

| Config name | Command |
|-------------|---------|
| `🛠️dev🗄️os-hub` | `bun nx run os-hub:dev` — data: `.🧬semio/🌐hub/hub-dev/` |
| `🛠️dev🗄️os-hub🛡️admin` | `bun nx run os-hub-admin:dev` |
| `🛠️dev🗄️os-hub` (compound) | with `🛠️dev🪐️space⚛️react` etc. |
| `🛠️dev🗄️os-hub` secure variants | `dev-secure-suite`, `dev-secure-native`, `dev-secure-mcp`, `dev-secure-admin` |
| `📦️build🗄️os-hub` | `bun nx run os-hub:build` (release) |
| `📦️build-dev🗄️os-hub` | `bun nx run os-hub:build-dev` |
| `📦️test🗄️os-hub` | `bun nx run os-hub:test` |
| `📦️check🗄️os-hub🚀️launch` | `bun nx run os-hub:local-bootstrap-launch-check` |
| `📦️check🗄️os-hub🟦️types` | `bun nx run os-hub-ts:typecheck` |
| `⚖️gate🔐️hub-auth🤝️live-sign-in` | `bun nx run os-hub:live-sign-in-check` |

### 1.3 Nx targets (`🌎️hub/📦️packages/🦀️rust/📋️project.json`)

All invoke `bun ./📜️script.ts <subcommand>` from `🌎️hub/📦️packages/🦀️rust/`.

**Build / run:** `setup`, `build`, `build-dev`, `build-dev-postgres`, `publish`, `dev`, `dev-postgres`, `dev-secure-suite`, `dev-secure-native`, `dev-secure-mcp`, `dev-secure-admin`, `secure-local-smoke`

**Test tiers:** `test`, `test-quick`, `test-long`, `test-exhaustive`, `test-all-features`

**Gates (sample):** `live-sign-in-check`, `local-bootstrap-launch-check`, `foundation-source-check`, `socket-grant-check`, `presence-*-check`, `inference-relay-check`, `directory-live-lanes`, … (60+ check targets)

**TS project:** `os-hub-ts:typecheck` → `🌎️hub/📦️packages/🟦️typescript/📜️script.ts typecheck`

### 1.4 script.ts entrypoints

Router at `🌎️hub/📦️packages/🦀️rust/📜️script.ts`:

- `build-dev` → stages `dist/build-dev/os-hub` via `buildCargoArtifacts(..., ["--bin", "os-hub"])`
- `dev` → `🚀️local-bootstrap/🏃️execution/🟦️.ts` `startLocalHub` (fd 3, trusted-catalog bootstrap on fresh root)
- Operator verbs on binary: `os-hub credential set`, `os-hub trusted-catalog publish`

Root `📜️script.ts` also exposes `publish os-hub` tarball.

### 1.5 Docker

- `🌎️hub/Dockerfile` + `🌎️hub/compose.yaml` — production topology, port `127.0.0.1:8787:8787`
- `docker compose config` resolves; **image not built** in this audit (README: ~40 min cold build)
- Postgres profile available but commented; requires `postgres` cargo feature on image

### 1.6 Zero-touch provisioning (default dev)

On first `os-hub:dev`:

1. Creates `OS_HUB_DATA` tree (`db/`, `directory.db`, `extension-modules/`, instance stores)
2. Local-bootstrap supervises hub on loopback with pipe-authenticated session issuance
3. **Trusted catalog**: `dev` materializes stdio+GIS bundle if missing (native compile) — until done, `/readyz` gate `artifactAuthority` stays closed
4. No postgres/docker required for default path

First **production** user: `os-hub credential set --email …` (stdin password) before first HTTP sign-in.

---

## 2. Architecture map

```
🌎️hub/
├── 🏗️bootstrap/🦀️.rs          ← main(), Axum router, HubState, all HTTP/WS routes
├── 📦️packages/🦀️rust/🦀️.rs   ← semio-hub library (re-exports modules)
├── 🔐️auth/                    ← password sessions, agent delegations, rate limits
├── 📇️directory/               ← identity/tenancy; sqlite | postgres | neo4j
├── 🗄️stores/🦀️.rs            ← HubInstance: authority/projection/blob/session journals
├── 🗿️artifact-authority/      ← trusted catalog, chunk CAS, native open/creation
├── 💡️inference/               ← GIS map inference runtime + sqlite ledger
├── 🚀️local-bootstrap/         ← dev launcher (fd 3, framing, credential issuance)
├── 🚀️local-relay/             ← dev HTTP relay routing (browser/MCP → hub paths)
├── 🛰️lag-rebootstrap/          ← verified rebootstrap source
├── 🔨️modules/🛡️admin/         ← admin SPA (built to 📤️dist, served at /admin)
├── 🧬️schema/                  ← hub gate schemas (foundation-source, socket-grant, …)
└── 🧪️tests/ + 🧫️fixtures/     ← law checks and fixtures

Framework dependencies:
├── 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/     ← event-sourced document store backends
├── 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust   ← os_directory kernel types
├── 🧰️framework/🔨️modules/📡️replication/            ← wire protocol (presence, mutations)
└── 🧰️framework/🛍️products/🖥️server/                 ← ServerInstance product ports (partially wired)
```

### 2.1 HubState wiring (`🏗️bootstrap/🦀️.rs`)

Single process holds: `db`, `directory`, `directory_service`, `artifact_cas`, `artifact_authority`, `artifact_publication`, `inference_runtime` (feature-gated), `presence` map, `fanout` broadcasters, `credential_sign_in`, `rate_limits`, admin surface.

Key routes (all on one listener):

- Health: `GET /healthz`, `GET /readyz`
- Auth: `POST /auth/sessions`, `/auth/credentials`, agent delegation routes
- Directory: `POST /directory/commands`, `GET /directory/events`, `GET /directory/socket/v1`, scoped document sockets
- Documents: blobs, open-plan, execution-target, `GET …/socket/v1` (collaboration)
- Inference: `/spaces/…/inference/gis-map/…`
- Admin: `/admin`, `/admin/api/*`

---

## 3. Services: implemented vs stubbed vs wired

| Service | Status | Wired at runtime? | Notes |
|---------|--------|-------------------|-------|
| **Auth** | **Implemented** | Yes | Password sign-in, sessions, agent delegations, rate limits. `IdentityAssertionVerifier` external IdP: **trait only, always `None`** in `main` (`🏗️bootstrap/🦀️.rs:10377-10378`) |
| **Directory** | **Implemented** | Yes | SQLite default; postgres/neo4j behind cargo features. Commands + event page + socket grants |
| **Document / event store** | **Implemented** | Yes | `connect_db()` — fs/sqlite/postgres/neo4j (`🏗️bootstrap/🦀️.rs:10214+`). Event-sourced, not CRUD |
| **Instance stores** | **Implemented** | Yes | `🗄️stores/🦀️.rs` — journal-backed authority/projections/blobs; sessions ephemeral files |
| **Presence** | **Implemented** | Yes | In-memory `HubState.presence` + `fanout`; WS handlers on document/directory sockets. **Ephemeral** (rebuilt on restart) |
| **Relay** | **Implemented (dev)** | Dev only | `🚀️local-relay/🧭️routing/🟦️.ts` — Bun HTTP proxy; not a separate production service |
| **Artifact authority** | **Implemented** | Yes (gated) | Closed until trusted catalog published; `/readyz` reports `artifactAuthority` |
| **Inference** | **Implemented** | Feature-gated | Requires `native-artifact-execution` + verified GIS map binding; routes registered in router |
| **Admin SPA** | **Implemented** | Yes | Static files from `OS_HUB_ADMIN_DIR`; API behind admin capability |
| **Server product modules** | **Stub seam** | Partial | `HubModules`/`HubDeciders`/… uninhabited in `🗄️stores/🦀️.rs` — routes not yet behind module ports |

No `todo!()` / `unimplemented!()` found under `🌎️hub/**/*.rs`.

---

## 4. Build & test verification

Commands run 2026-09-23 from repo root unless noted. Logs: `🗑️generated/hub/`.

| Command | Result | Detail |
|---------|--------|--------|
| `cargo check -p semio-hub --bin os-hub` | **PASS** (exit 0, ~4m46s) | 10 warnings (dead code in bootstrap). Log: `cargo-check.log` |
| `cargo build -p semio-hub --bin os-hub` | **PASS** (exit 0, ~39s) | `Finished dev profile`. Log: `cargo-build-bin.log`. Binary not left in `target/debug/` (likely consumed/staged by concurrent `build-dev`) |
| `bun nx run os-hub:build-dev` (sandbox) | **Inconclusive** | Nx unix-socket EPERM in sandbox; no build output. Log: `cargo-build-dev.log` |
| `bun ./📜️script.ts build-dev` (direct) | **Still running >33min at audit end** | Stuck in `buildCargoArtifacts` heartbeat compiling `semio-hub` dependency graph under file lock. Log: `cargo-build-dev-direct.log` |
| `bun ./📜️script.ts test quick` | **FAIL** | 44/45 passed before cancel; failure: `trusted_publication_owner_process_crash_releases_exact_lock` — `"publication child did not acknowledge its exact lock"`. Log: `cargo-test-quick.log` |
| `bun ./📜️script.ts typecheck` (os-hub-ts) | **FAIL** (script exit 0) | TS errors in plugin native-codec tests + `📜️script.ts:10454` + missing `.mjs` types. Log: `ts-typecheck-direct.log` |
| `bun nx run os-hub-ts:typecheck` (sandbox) | **Inconclusive** | Same Nx socket issue. Log: `ts-typecheck.log` |
| `bun ./📜️script.ts live-sign-in-check` | **PASS** (exit 0, ~80s) | Real hub child, auth + directory + invites + rate limit. Log: `runtime-live-sign-in.log` |
| `bun ./📜️script.ts local-bootstrap-launch-check` | **PASS** | 4 staging cases. Log: `runtime-local-bootstrap.log` |
| `bun ./📜️script.ts presence-lease-check source` | **FAIL** | ENOENT: expects `🧫️fixtures/👥️presence-lease-v1/🧪️fixture/🔣️.json` but fixture is `…/🔣️.json`. Log: `presence-lease-source.log` |
| `bun ./📜️script.ts socket-grant-check` | **In progress >7min** | Waiting on cargo file lock + long Rust test. Log: `socket-grant-check.log` (incomplete) |

### 4.1 Nx agent sandbox note

Cursor agent sandbox blocks Nx unix sockets (`/tmp/.nx/… EPERM`). Use `bun ./📜️script.ts …` directly or run nx with full permissions / `NX_SOCKET_DIR` in workspace.

---

## 5. Runtime probe (live-sign-in)

`live-sign-in-check` started a **fresh** hub:

```
OS_HUB_DATA=/private/tmp/au3-hub-data-SQZ82A
port=8831
OS_HUB_CREDENTIAL_SIGN_IN=1
```

Observed:

- **`/readyz`**: `status=not-ready` on fresh root (expected: no trusted catalog yet; `artifactAuthority` gate closed)
- **`POST /auth/sessions`**: 200, session capability minted, `semio.hub.auth.error/v1` on bad credentials
- **`GET /auth/sessions/me`**: 200 with principal metadata
- **Directory commands**: create-space, create-invite, invite redemption — 202/200
- **Credential change / sign-out / rate limit**: all passed (429 with `rate-limited` class)
- **Persistence**: SQLite directory + fs store under temp data root (implicit from command lifecycle)
- **Presence WebSocket**: **not exercised** by this gate (HTTP-only)

Manual curl probe was not duplicated; live-sign-in is the authoritative runtime gate for auth/directory HTTP.

---

## 6. Schema & client/server agreement

### 6.1 Hub-local `🧬️schema/`

Gate/test contracts only (JSON Schema draft-07):

- `🧱️foundation-source/🔣️.json` — source boundary for hub TS/Rust owners
- `🧱️socket-grant-command-source/🔣️.json`
- `🔐️browser-broker-proof-lifecycle-v1/🔣️.json`

No GraphQL/Proto in this folder; hub HTTP bodies use **`semio.hub.*` / directory schema IDs** in JSON.

### 6.2 Shared protocol surfaces

| Domain | Server | Client / twin | Agreement mechanism |
|--------|--------|---------------|---------------------|
| Auth capabilities | `🔐️auth/🧬️schema/🔣️.json` + Rust | TS imports in hub scripts | JSON schema + shared regex patterns |
| Directory commands/events | `🧰️…/📇️directory/🧬️schema/` (JSON + `🟦️.ts` + `🦀️.rs`) | OS frontend, MCP, hub `📜️script.ts` | Schema-first; Rust/TS parsers |
| Replication wire (presence, mutations) | `🧰️framework/🔨️modules/📡️replication/` Rust `protocol` | `🟦️.ts` twin | Byte-identical fixtures in `🧫️fixtures/📡️wire` |
| Inference | `💡️inference/🧬️schema/🔣️.json` | Hub routes + TS constants | JSON schema |
| Local bootstrap | `🚀️local-bootstrap/🧬️schema/🔣️.json` | Launcher TS | Framed pipe + schema |

**Client agreement:** OS renderer and MCP import directory/replication TS from framework; hub server uses Rust equivalents from same schema tree. **No separate OpenAPI** — contract is schema JSON + typed parsers + law tests.

---

## 7. Prioritized gaps & suggested fixes

### P0 — Blocks smooth dev boot

1. **`build-dev` compile time / lock contention**  
   - **Symptom:** `build-dev` >30min with only heartbeat lines; concurrent `cargo build` + `build-dev` fight for artifact directory lock.  
   - **Fix:** Serialize hub builds in CI/dev; document single builder; consider `-p semio-hub` incremental artifact cache warmup target.

2. **Fresh `/readyz` not-ready until trusted catalog**  
   - **Symptom:** live-sign-in shows `not-ready`; collaboration/inference gates closed.  
   - **Fix:** Document as expected; ensure `dev` path always completes catalog materialization before declaring ready (or expose progress on `/readyz` blocked_by).

3. **Quick test failure: trusted catalog publication lock**  
   - **File:** `🗿️artifact-authority/🔏️trusted-catalog/🛡️opened-root/🧪️tests/🔬️publication/🦀️.rs:72`  
   - **Fix:** Debug child lock handshake in `trusted_publication_owner_process_crash_releases_exact_lock`.

### P1 — Broken gates / tooling

4. **Presence lease check fixture path**  
   - **Script expects:** `🧫️fixtures/👥️presence-lease-v1/🧪️fixture/🔣️.json`  
   - **Actual:** `🧫️fixtures/👥️presence-lease-v1/🔣️.json`  
   - **Fix:** Align path in `📜️script.ts` `provePresenceLeaseFixture` (~line 15437).

5. **Hub TS typecheck failures**  
   - Plugin test `unknown` types; hub script `creation.ready` possibly undefined; missing declaration for `🕸️dependencies/🧩️runtime/🟨️.mjs`.  
   - **Fix:** Tighten types in affected files; make `typecheck` script exit non-zero on tsc errors.

6. **Nx unusable in agent sandbox**  
   - **Fix:** Document `bun ./📜️script.ts` direct invocation; or set `NX_SOCKET_DIR` under workspace for agents.

### P2 — Production / E2E collaboration

7. **External IdP unimplemented** — production relies on password credentials only (`IdentityAssertionVerifier = None`).

8. **Postgres/neo4j live lanes untested** — README + compose admit compile/unit-test only; no verified run against real postgres (ticket 26/09/18).

9. **Docker image unbuilt** — no container runtime verification in this audit.

10. **No CI** — `.github/workflows/` empty; regressions depend on local launch gates.

11. **Trusted catalog jco policy bump** — published generation invalidated on codegen policy change (README known gap); breaks restart after toolchain bump.

12. **Presence/collaboration WS not covered by live-sign-in** — need `socket-grant-check` / scoped-presence browser serve completion for E2E proof.

13. **Server product module ports uninhabited** — long-term refactor to move routes behind `ServerInstance` modules (`🗄️stores/🦀️.rs` documents intent).

---

## 8. Command log index

| Log file | Command |
|----------|---------|
| `cargo-check.log` | `cargo check -p semio-hub --bin os-hub` |
| `cargo-build-bin.log` | `cargo build -p semio-hub --bin os-hub` |
| `cargo-build-dev-direct.log` | `bun ./📜️script.ts build-dev` (incomplete) |
| `cargo-test-quick.log` | `bun ./📜️script.ts test quick` |
| `ts-typecheck-direct.log` | `bun ./📜️script.ts typecheck` in os-hub-ts |
| `runtime-live-sign-in.log` | `bun ./📜️script.ts live-sign-in-check` |
| `runtime-local-bootstrap.log` | `bun ./📜️script.ts local-bootstrap-launch-check` |
| `presence-lease-source.log` | `bun ./📜️script.ts presence-lease-check source` |
| `socket-grant-check.log` | `bun ./📜️script.ts socket-grant-check` (incomplete) |
| `binary-inventory.log` | dist/ directory listing (empty before build) |

---

## 9. Recommended dev loop (verified path)

```bash
# From repo root — prefer direct script over nx in sandboxes:
cd 🌎️hub/📦️packages/🦀️rust
bun ./📜️script.ts build-dev          # once; expect long first compile
bun ./📜️script.ts dev                # or: bun nx run os-hub:dev from IDE launch

# Gates that passed this audit:
bun ./📜️script.ts live-sign-in-check
bun ./📜️script.ts local-bootstrap-launch-check

# Health (when hub running on 8787):
curl -s http://127.0.0.1:8787/healthz
curl -s http://127.0.0.1:8787/readyz | jq .
```

Production smoke (loopback):

```bash
export OS_HUB_DATA=/tmp/semio-hub-data
./dist/build-dev/os-hub credential set --email ada@example.com
OS_HUB_MODE=production OS_HUB_BIND=127.0.0.1 \
  OS_HUB_CREDENTIAL_SIGN_IN=true \
  OS_HUB_ADMIN_SUBJECTS=credential.password.v1:ada@example.com \
  ./dist/build-dev/os-hub
```
