# End-to-End OS ↔ Hub Collaboration Audit

**Ticket:** `26/09/23/END-TO-END-OS-HUB-COLLABORATION-MCP`  
**Date:** 2026-09-23  
**Scope:** Read-only trace of User A OS client → hub → User B OS client; verified with runnable tests where possible.

---

## 1. Sequence Diagram (Current Flow)

```
┌─────────────┐                              ┌──────────────┐                              ┌─────────────┐
│  OS Client  │                              │     Hub      │                              │  OS Client  │
│  (User A)   │                              │  (os-hub)    │                              │  (User B)   │
└──────┬──────┘                              └──────┬───────┘                              └──────┬──────┘
       │                                              │                                              │
       │ ① POST /auth/sessions (email+password)       │                                              │
       │─────────────────────────────────────────────>│                                              │
       │     SessionMintResponseV1 + session cookie     │                                              │
       │<─────────────────────────────────────────────│                                              │
       │                                              │  ①' (same for User B)                        │
       │                                              │<─────────────────────────────────────────────│
       │                                              │                                              │
       │ ② GET /directory/spaces, /directory/events   │                                              │
       │    (DirectoryClient — REST, no client cache) │                                              │
       │─────────────────────────────────────────────>│                                              │
       │                                              │<─────────────────────────────────────────────│
       │                                              │                                              │
       │ ③ POST /directory/commands (space create,     │                                              │
       │    share, artifact-creation, check-in, …)    │                                              │
       │─────────────────────────────────────────────>│                                              │
       │     HubDirectory append-only event log         │                                              │
       │     → fan-out via /directory/socket/v1       │                                              │
       │<═════════════════════════════════════════════│════════════════════════════════════════════>│
       │                                              │                                              │
       │ ④ POST …/open-plan → POST …/socket-grants    │                                              │
       │─────────────────────────────────────────────>│                                              │
       │     SocketGrantReceiptV1 (HMAC capability)     │                                              │
       │<─────────────────────────────────────────────│                                              │
       │                                              │                                              │
       │ ⑤ WS /spaces/{space}/documents/{doc}/socket/v1│                                             │
       │    ?surface=<kind>@<std>/<subset>#<role>       │                                              │
       │    subprotocol: semio.socket.v1                │                                              │
       │─────────────────────────────────────────────>│                                              │
       │                                              │<─────────────────────────────────────────────│
       │                                              │                                              │
       │ ⑥ ClientFrame::SocketHelloV1                  │                                              │
       │    (resume_token?, frontier, schema)           │                                              │
       │─────────────────────────────────────────────>│                                              │
       │     ServerFrame::Welcome { resume_token,       │                                              │
       │       server_frontier, bootstrap? }            │                                              │
       │<─────────────────────────────────────────────│                                              │
       │                                              │                                              │
       │ ⑦ Local edit → ArtifactStore::dispatch        │                                              │
       │    → backbone worker (TS browser / Rust actor)│                                              │
       │    → ClientFrame::Commands { envelopes }      │                                              │
       │─────────────────────────────────────────────>│                                              │
       │     db::ArtifactHandle::submit (Fsync WAL)    │                                              │
       │     hub linearizes via MergePolicy            │                                              │
       │     ServerFrame::Ack → submitter               │                                              │
       │     ServerFrame::Commands → other sockets       │                                              │
       │<─────────────────────────────────────────────│────────────────────────────────────────────>│
       │                                              │                                              │
       │ ⑧ ClientFrame::Presence { peer bytes }        │  (ephemeral shared)                          │
       │─────────────────────────────────────────────>│ broadcast (not durable)                        │
       │     ServerFrame::Presence → all doc sockets    │                                              │
       │<═════════════════════════════════════════════│════════════════════════════════════════════>│
       │                                              │                                              │
       │ ⑨ UI: scopedPresencePeersV1 → PresenceBar     │                                              │
       │    (avatars, hub-assigned color, agent badge) │                                              │
       │                                              │                                              │
       │ ─── short outage ───                           │                                              │
       │ ⑩ WS drops; worker schedule_reconnect()       │                                              │
       │    (HUB_RECONNECT_MIN_MS…MAX_MS backoff)       │                                              │
       │    SocketHelloV1 echoes resume_token           │                                              │
       │─────────────────────────────────────────────>│                                              │
       │     Welcome OR RebootstrapRequired             │                                              │
       │     → lag-rebootstrap canonical checkpoint pair│                                              │
       │<─────────────────────────────────────────────│                                              │
       │     IngestRemote → local projection rebuild    │                                              │
       │                                              │                                              │
```

**Dev relay path:** Browser dev server proxies `/_semio/hub/*` → upstream hub via `🌎️hub/🚀️local-relay/🧭️routing/🟦️.ts` (`localRelayUpstreamPath`).

**Key modules:**

| Layer | Path | Role |
|-------|------|------|
| OS directory facade | `🧰️framework/🛍️products/💻️os/🟦️.ts` (`DirectoryClient`) | REST: auth, spaces, commands, events |
| OS backbone worker | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts` | Browser: WS grants, sync, rebootstrap |
| OS sync actor | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs` | Native: per-document actor, reconnect, resume_token |
| OS store | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs` | Local-first VCS graph; `dispatch` / `IngestRemote` |
| Wire protocol | `🧰️framework/🔨️modules/📡️replication/` | `ClientFrame`/`ServerFrame` lanes: Commands, Preview, Presence |
| Hub bootstrap | `🌎️hub/🏗️bootstrap/🦀️.rs` | axum routes, WS handler, `submit_commands`, presence fan-out |
| Hub directory | `🌎️hub/📇️directory/🦀️.rs` | Identity/tenancy event log, directory WS |
| Hub document DB | `db::Database` (via bootstrap) | Command persistence, ordering, WAL |
| Hub lag-rebootstrap | `🌎️hub/🛰️lag-rebootstrap/🦀️.rs` | Verified checkpoint pair transport on lag |
| Hub auth | `🌎️hub/🔐️auth/🦀️.rs` | `POST /auth/sessions`, socket-grant HMAC capabilities |
| Hub stores (framework port) | `🌎️hub/🗄️stores/🦀️.rs` | Event-sourced journals; **`Documents = NoDocumentAuthority`** |
| Presence UI | `…/🏛️ShellHost/👥️presence-scope/🟦️.ts` + `PresenceBar` | Scope-filtered peer roster in shell chrome |

---

## 2. Verified Status (Commands Run 2026-09-23)

| Test target | Command | Result | Log |
|-------------|---------|--------|-----|
| Hub Rust unit/integration | `bun nx run os-hub:test-quick` | **323 passed, 9 skipped** (~4m) | `🗑️generated/collab/os-hub-test-quick.log` |
| Hub TS contract harness | `bun nx run os-hub-ts:test-quick` | **10 passed, 1 failed, 1 skipped** (~19s) | `🗑️generated/collab/os-hub-ts-test-quick.log` |
| Hub E2E (binary boot) | `HUB_E2E=1 bun nx run os-hub-ts:test` | **Not run** (requires cargo build + live server; gated) | — |
| OS two-user Playwright collab | `🧑‍💻dev/🧪️tests/🤝️collaboration/🟦️.ts` | **Not run** (13-step; needs hub + 2 dev servers + Playwright; ~30min budget) | — |

### os-hub-ts failure detail

```
hub harness quick contract > validates local bootstrap … with independent HMAC
Error: strict mode: required property "blockedBy" is not defined at
  hub/local-bootstrap/schema.json#/allOf/0/then/not
```

AJV strict-mode compile failure in `hubSchemaExport` — schema/test harness drift, not a runtime hub bug.

### os-hub Rust coverage (representative)

`🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs` exercises (among others):

- Document socket grants + scoped WS admission
- Cross-document grant rejection
- Command linearization + broadcast to second socket
- Directory socket event replay (`/directory/socket/v1?since=`)
- Lag/rebootstrap + session revoke under broadcast pause
- Presence-per-surface normalization

These prove **hub-side** collaboration primitives at the protocol/DB layer.

---

## 3. Broken / Missing Links

### 3.1 Critical gaps (block “two OS users editing same artifact” E2E proof)

| # | Gap | Evidence | Suggested fix |
|---|-----|----------|---------------|
| G1 | **No default-CI OS↔hub↔OS E2E** | `HUB_E2E=1` gates `os-hub-ts` live tests; `🤝️collaboration/🟦️.ts` is separate, heavy, documents expected upstream failures | Add language-agnostic two-client test (§5); wire into `os-hub:test-long` or dedicated nx target |
| G2 | **Dual backbone implementations** | Rust `🔄️sync/🦀️.rs` actor vs TS `👷️worker/🟦️.ts` twin; browser production uses TS path | Add parity contract test: same fixture frames → same `IngestRemote` projection in both paths |
| G3 | **local-bootstrap schema AJV break** | `os-hub-ts:test-quick` failure | Fix `🌎️hub/🚀️local-bootstrap/🧬️schema/🔣️.json`: declare `blockedBy` in `then/not` branch or relax strict keyword in harness for that export |
| G4 | **Remote cursors / in-canvas presence** | `PresenceBar` shows roster (avatars, colors); `ArtifactPresenceInteraction` wire exists but no audited plugin renderer for peer cursors in puzzle/writer viewports | Per-plugin: subscribe to presence `interaction` domains and render foreign selections/cursors in 3D/2D hosts |

### 3.2 Architectural seams (intentional but incomplete)

| # | Gap | Evidence | Suggested fix |
|---|-----|----------|---------------|
| A1 | **`HubInstance::Documents = NoDocumentAuthority`** | `🌎️hub/🗄️stores/🦀️.rs:62` — framework `ServerInstance` document port uninhabited | Wire `db::Database` document authority behind `ServerInstance::Documents` when migrating bootstrap to framework gateway |
| A2 | **Empty `HubModules` / `HubDeciders` / `HubResolvers`** | Same file — auth/deciders still in `🏗️bootstrap/🦀️.rs` | Incrementally move `🔐️auth`, command deciders into `ServerModule` variants |
| A3 | **Collaborative redo not durable** | `🌎️hub/🧫️fixtures/🤝️two-author-shell-v1/🔣️.json` lists `"durable-collaborative-redo"` as observation; GIS two-author process fences `"no-durable-collaborative-redo"` | Implement server-stamped redo log per user+document or document explicitly as out-of-scope |

### 3.3 Data-classification routing gaps

| # | Gap | Evidence | Suggested fix |
|---|-----|----------|---------------|
| D1 | **Home local-only vs hub spaces union** | `🪐️space/🫀️core/🦀️.rs` — local catalog has no membership roster | Ensure hub-created spaces always carry `origin: "hub"` and directory projection fold is active in both shells |
| D2 | **Ephemeral studio without backbone** | `create-studio` path; test asserts `document.backbone.is_none()` | Document as local-only; block share/collab UI for ephemeral studios |
| D3 | **Preview lane vs command lane** | Hub bootstrap: preview/presence = broadcast only, never WAL | Plugins using preview for “almost shared” state must not rely on it for durability |

---

## 4. Artifact / Plugin Routing: Shared vs Local-Only

Routing is determined by **how the document is opened**, not plugin kind alone:

- **Hub-shared (durable, multi-user):** `PersistenceBinding::Hub { base_url, space_id, surface }` → document WS → command lane → hub WAL → fan-out.
- **Folder-local:** `PersistenceBinding::Folder { path }` → `folder://` / `file://` event log; no hub relay.
- **Ephemeral local-only:** no backbone attachment or in-memory draft only.

| Plugin / artifact | Persisted mutations via hub command lane | Ephemeral local-only (not shared) |
|-------------------|------------------------------------------|-----------------------------------|
| **✒️ writer** | Yes, when opened in hub space (collab E2E target) | Child stdio document body local-only per test comments |
| **🧩 puzzle 3d/5d** | Yes, when hub-bound (editor ops → `dispatch`) | Brush precompute, suggestions link, per-window options/transform, vortex UI state |
| **🪐 space / 🏠 home** | Space index + directory commands (control plane) | Local-only catalog rows; `fold-directory-events` is view-only |
| **🖍️ draw** | Yes, when hub-bound | Presence read cursors (transient) |
| **📐 cad / 🌀 procedural 3d** | Inference jobs via hub REST when configured | Preview eval, scene materialization handles |
| **🗺️ gis map** | Full two-author fixture (inference + approval + rebootstrap) | — |
| **Directory metadata** | `POST /directory/commands` + directory WS | Never CRUD — event-sourced only |
| **Presence / cursors / selections** | Presence lane (ephemeral shared) | Preview lane payloads |
| **MCP / AI agents** | Agent sessions + delegated credentials (`principalKind: agent`) | Agent-local tool state |

**Rule of thumb:** If the artifact actor has a hub `PersistenceBinding` and an admitted document socket, **editor `dispatch` mutations** ride the shared command stream. UI-only, per-window, or preview state stays local.

---

## 5. Conflict / Ordering / Reconnect (Summary)

| Concern | Mechanism | Location |
|---------|-----------|----------|
| **Ordering** | Hub `db::ArtifactHandle::submit` + WAL `Fsync`; single writer per document | `🏗️bootstrap/🦀️.rs` `submit_commands` |
| **Conflicts** | `protocol::MergePolicy` graded per batch; `Ack` with `ApplyOutcome` | Same; client applies or rejects via VCS |
| **No CRDT** | Server linearization; clients replay committed envelopes | `🏪️store` + `IngestRemote` |
| **Reconnect** | Exponential backoff `HUB_RECONNECT_MIN_MS`–`MAX_MS`; `resume_token` in hello/welcome | `🔄️sync/🦀️.rs`, `👷️worker/🟦️.ts` |
| **Large lag** | `ServerFrame::RebootstrapRequired` → canonical checkpoint pair | `🛰️lag-rebootstrap/🦀️.rs` |
| **Short outage UX** | Worker keeps local queue; `schedule_reconnect` without freezing UI thread | Store actor design (mailbox + turn limits) |
| **Directory sync** | Separate `/directory/socket/v1` lane; `DirectoryClient.events` / event-page | Not mixed with document command lane |

---

## 6. Auth / Identity Across Clients

```
Credential (email+password)
  → POST /auth/sessions (hub.auth)
  → HubDirectory::issue_auth_session (durable session-issued fact)
  → HTTP session cookie / bearer
  → POST /directory/socket-grants or /spaces/.../socket-grants
  → SocketGrantCapabilityV1 (HMAC: semio/hub/socket/v1)
  → WS subprotocol semio.socket.v1
  → SocketHelloV1.actor + surface query param
  → PresencePeer (userId, label, color, principalKind)
```

- **Local dev bootstrap:** `🚀️local-bootstrap` pipe issues one-shot credentials (HMAC-proved).
- **Admin:** `OS_HUB_ADMIN_TOKEN`; `/admin/api/connections` lists live sockets.
- **Agents:** `post_agent_session` mints delegated identity; UI badges `isAgent` in `PresenceBar`.
- **Revocation:** `authorization_generation` on session; socket admission re-checks binding.

Files: `🌎️hub/🔐️auth/🦀️.rs`, `🌎️hub/🚀️local-bootstrap/`, `🧰️framework/…/📇️directory/🧬️schema/`.

---

## 7. Proposal: Language-Agnostic Two-Client E2E Test

**Goal:** Prove User A command reaches User B projection without Playwright or plugin UI.

**Fixture:** `🌎️hub/🧫️fixtures/🤝️two-author-shell-v1/🔣️.json` pattern — minimal writer or raw document schema.

**Harness:** Extend `🌎️hub/🤝️integration-harness/🟦️.ts` (`startHub`, `hubSchemaExport`).

**Steps:**

1. `startHub({ dataDir: mkdtemp, adminToken })` — spawn real `os-hub` binary.
2. Provision two credentials via `os-hub credential set` (CLI, not network).
3. **Client A:** `DirectoryClient` → `mintSession` → `command(create-space)` → `command(create-artifact)` → `openPlan` → `socketGrant` → open `WebSocket` to document URL.
4. **Client B:** same hub URL, different session → `command(share-space)` or membership → same artifact `openPlan` + WS.
5. Both send `SocketHelloV1`; await `Welcome`.
6. **Client A:** `encodeClientFrame(Commands { envelopes: [single text-insert op] })`.
7. **Client B:** await `ServerFrame::Commands` with same envelope (decode via `@semio-tech/framework-replication`).
8. Assert hub WAL frontier advanced (`headSeq` via directory event or `Ack` frontier).
9. **Presence:** A sends `Presence` frame; B receives `ServerFrame::Presence` with A's actor.
10. **Reconnect:** drop B's WS; A sends another command; B reconnects with `resume_token`; assert B receives missed `Commands` or completes rebootstrap to same frontier.
11. **Restart:** `hub.stop()`; `startHub` same `dataDir`; B reconnects; assert artifact still listed and frontier matches.

**Implementation sketch:**

```
🌎️hub/🧪️tests/🤝️two-client-document/🟦️.ts
  describe.skipIf(process.env.HUB_E2E !== "1")("two-client document collaboration", …)
```

Register in `os-hub-ts` vitest config; run: `HUB_E2E=1 bun nx run os-hub-ts:test-long`.

**Pass criteria:** Steps 7–10 green; no browser, no plugin WASM — only `DirectoryClient` + raw WS + shared replication codec (already used in `🧪️tests/🤝️integration/🟦️.ts` and `🔬️bin-unit/🦀️.rs`).

**Follow-up:** Thin wrapper that mounts writer plugin and asserts DOM text — optional layer on top, not gate for protocol proof.

---

## 8. Recommended Priority

1. Fix G3 (local-bootstrap schema) — unblocks harness CI signal.
2. Implement §7 two-client WS test — closes G1 with minimal surface.
3. G2 backbone parity test — TS vs Rust worker for same envelope fixture.
4. G4 presence cursors in puzzle/writer viewports — user-visible collab.
5. A1/A2 framework port migration — long-term; not blocking protocol path.

---

*Test logs: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/END-TO-END-OS-HUB-COLLABORATION-MCP/🗑️generated/collab/`*
