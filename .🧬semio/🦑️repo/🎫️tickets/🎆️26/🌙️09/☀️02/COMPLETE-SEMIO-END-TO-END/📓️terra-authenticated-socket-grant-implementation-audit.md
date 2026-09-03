# Authenticated Socket-Grant Implementation Audit

Date: 2026-09-03  
Scope: read-only implementation audit after secure-session foundations and before authoritative open plans. No production/test source was edited and no build, test, or runtime probe was run.

## Decision

Land a small server-owned **`socket.v1` grant ledger** first. A protected HTTP issuance endpoint accepts the existing typed bearer capability, resolves the durable authorization binding and exact audience/scope, then returns a short-lived, one-use opaque grant. The browser/native WebSocket upgrade carries that grant only as a second `Sec-WebSocket-Protocol` offer; the URL carries no credential. The server consumes it atomically before upgrade, selects only `semio.socket.v1` (never echoes the grant), and the first binary `Hello` is credential-free: it contains only bounded protocol/schema/frontier data.

This packet derives actor and role on the server. It does not depend on catalog/open-plan work, local-bootstrap transport internals, or P2-D chunk CAS. It must not claim the old routes are secure until clients migrate and the legacy `Hello.token` route is removed.

## Current carrier and concrete failures

| Channel | Current route/carrier | Source evidence | Failure |
| --- | --- | --- | --- |
| Document sync | `GET /spaces/{space}/documents/{document}/ws?surface=`; first binary `ClientFrame::Hello` contains `actor` and optional `token`. | `🌎️hub/📦️packages/🦀️rust/📦️bin.rs:787-800,1033-1100`; wire definition `🧰️framework/🔨️modules/📡️replication/📡️wire/🦀️.rs:45-56`. | Caller chooses the actor retained in presence, principal, sync-session record, relay origin, and mutation path. Long-lived session/share bearer is present in app-frame bytes. |
| Directory stream | `GET /directory/ws?token=&since=`. | `bin.rs:1619-1647`; TypeScript URL construction `🧰️framework/🛍️products/💻️os/🟦️.ts:4056-4094`; native URL builder `…/📇️directory/🔌️client/🦀️.rs:364-384`. | Bearer appears in URL history, reverse-proxy/request logs, telemetry, and reconnection strings. It is not a scope/audience-bound one-use credential. |
| Admin live updates | No dedicated admin WebSocket exists. The SPA uses the same `DirectoryClient.stream`; all admin operations are authenticated REST. | Hub routes `bin.rs:2076-2104`; admin fake socket is the directory fake in `🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/🧪️admin.test.tsx:69-99`. | Admin’s directory stream inherits the URL-token defect; admin authority is not a socket audience. |
| React document worker | Browser opens document URL and sends `binding.token` plus configured caller actor in `Hello`. | `🧰️framework/🛍️products/💻️os/🟦️backbone-worker.ts:563-601`. | Direct proof of token-in-frame and actor spoofing path. |
| Native/store sync and MCP hub workspace | Native URL stays credential-free but sends `PersistenceBinding::Hub.token` in `Hello`; MCP turns its `Hub { base_url, space_id, token }` into that binding. | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs:757-767,1815-1838`; `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs:433-449`. | Native and MCP retain the same bearer-in-frame and caller-actor contract. |

`handle_ws` resolves a session, exact document share, or public visibility from the client token (`bin.rs:456-478,1040-1056`), but then directly accepts the caller's `actor` (`:1078-1113`). It stores that actor in the durable sync-session and uses it for the principal (`:1177-1189`), then accepts `MutationEnvelope.actor` through the gate (`:953-999`). A forged actor therefore crosses trust domains even though membership role is server-resolved.

The current session foundation is reusable: `AuthSessionRecord` has a durable id, digest-only secret, expiry, revocation state, and `authorization_generation` (`🌎️hub/📇️directory/🦀️.rs:158-210,724-748`; SQLite schema/atomic revocation `🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:127-145,338-370`). Directory stream already revalidates the exact session/generation for every visible outbound message (`bin.rs:1551-1600`). Share verification is exact `(space, document)` today (`bin.rs:469-473`). These must become the grant’s binding inputs, never copied bearer secrets.

## `SocketGrantV1` contract

### Schema and authority

Define a first-party schema in `🌎️hub/📇️directory/🦀️.rs`, with a neutral JSON fixture and generated Rust/TypeScript shape:

```text
SocketGrantV1
  schema = "semio.hub.socket-grant/v1"
  capability = "socket.v1.<selector>.<base64url-secret>"
  audience = document.v1 | directory.v1
  scope =
    document: { spaceId, documentId }
    directory: { subjectSessionId }
  subject =
    session: { sessionId, userId, authorizationGeneration, actorId, role }
    share: { shareSelector, spaceId, documentId, actorId, role: spectator }
  issuedAtMs, expiresAtMs, correlationId
```

The record stores selector plus a domain-separated SHA-256 of the secret, never plaintext. Enforce lowercase/base64url grammar, 16-byte selector, 32-byte secret, fixed `socket.v1` domain, one use, 30-second maximum TTL, 256-byte scalar limits, and an at-most-64-grants-per-session/64-per-share bounded ledger. `actorId` is server-derived: a domain-separated digest of the durable session id for sessions; an opaque per-grant digest for read-only shares. A client never supplies, chooses, or learns a reusable actor capability.

`POST /spaces/{spaceId}/documents/{documentId}/socket-grants` accepts only `Authorization: Bearer <session|share>`. It resolves session membership or exact share **before** minting; public visibility does not mint a grant in this authenticated packet. Session grants obtain the current server role; share grants are always spectator. The response contains only `{ schema, protocol: "semio.socket.v1", grant, expiresAtMs }`; it must not return user email, role, session id, selector, digest, actor id, or a private locator.

`POST /directory/socket-grants` accepts only a valid session bearer and mints `directory.v1`, bound to that session id/generation. It has no share/public form. It does not confer administrator power: admin REST continues to authenticate and authorize independently.

### Upgrade and first frame

For document and directory routes, require these offered subprotocol values, in order:

```text
Sec-WebSocket-Protocol: semio.socket.v1, socket.v1.<selector>.<secret>
```

The route bounds both headers and URI before parsing. It atomically consumes the exact matching grant for its audience and route scope; a mismatch, expiry, replay, backend denial, missing second offer, duplicate grant, or malformed subprotocol returns a redacted HTTP rejection without upgrade. The server responds with only `Sec-WebSocket-Protocol: semio.socket.v1`. It never writes the grant into an error, audit payload, close reason, `Debug`, request URL, persistence record, or broadcast frame.

After successful consumption, the upgraded document socket accepts exactly one bounded binary `HelloVNext` before the handshake deadline. It has no `token` and no `actor`; retained fields are wire/protocol versions, descriptor/schema hash, optional bounded resume/frontier. An invalid/late/non-binary first frame closes `4400 protocol`. No data/presence/command frame is accepted before server `Welcome`. `surface` remains optional bounded telemetry on the URI and is not a grant scope, role selector, or open-plan capability.

The server constructs `Principal`, session color/presence key, sync-session audit fields, command relay origin, and every persisted mutation actor from consumed subject data. At command ingress it overwrites/rejects any envelope actor that differs from the bound actor; preferred clean contract is a wire `MutationEnvelope` without client actor and hub stamps it before `SecurityGate::admit_command`. This must be coordinated with the replication-wire owner, not hidden in an adapter.

### Revocation, close, and bounds

Add a narrow directory port returning public authorization state by id, not a secret-bearing capability:

```text
auth_session_binding_active(sessionId, userId, generation, now) -> active | revoked | expired | unavailable
share_binding_active(scope, selector, now) -> active | revoked | expired | unavailable
```

Implement it in SQLite, PostgreSQL, Neo4j, and memory parity. The grant ledger holds a binding index from session `(id,generation)` and share `(scope,selector)` to unconsumed grants and live-socket close notifiers. Session/self/admin/identity revocation and share revoke remove unconsumed grants and signal the exact live bindings. Every live socket also checks its binding on a fixed one-second tick and before a privileged inbound frame; unavailable fails closed (`1013 unavailable`), revoked/expired/membership loss closes terminally (`4401 authorization-revoked`). Do not keep using the bearer from `Hello` for revalidation.

Grant issue/consume are bounded, cancellable server operations: mint/consume deadline 2 seconds, one DB revalidation plus one ledger lock, bounded diagnostics at 256 bytes, and no progress event for the small operation. Existing checkpoint/rebootstrap operations remain independent: document lag still sends its storage-key-free control then `1013` (`bin.rs:840-861,1244-1248`). CAS pair transfer, checkpoint bytes, and catalog generation are not read by `SocketGrantV1`.

## Smallest backend-first Sol packet

### S1 — schema, digest, ledger, and neutral oracle (first blocker)

1. In `🌎️hub/📇️directory/🦀️.rs`, add `SocketGrantV1` request/receipt types, parsing, domain-separated digest, bounded scalar validation, redacted error enum, binding-state port, and neutral fixture under its existing test fixture family. Reuse capability parsing patterns at `:409-532`, not an external JWT/OIDC library.
2. In `🌎️hub/📦️packages/🦀️rust/📦️bin.rs`, add a process-local `SocketGrantLedger` to `HubState`: one mutex/sharded bounded map, monotonic expiry sweep at mint/consume, one atomic consume transition, and session/share binding indexes. A grant is intentionally volatile: restart invalidates it, which is safe because clients reacquire through authenticated HTTP.
3. Add unit/independent oracle tests for known vectors: parse rejects cross-kind/session/share confusion; two concurrent consumes produce exactly one success; different document/audience fails; expired/revoked/generation-changed binding fails; returned redactions contain no grant/secret; restart ledger contains none. Use a second SHA-256 implementation (WebCrypto in the existing Bun test lane or `openssl dgst` in a hermetic fixture) against a language-neutral fixture.

### S2 — protected issuance and grant-authenticated routes

1. Add the two issuance routes and a request authenticator that calls the current `authenticate_session`/`authenticate_share` only at issuance. Enforce exact document scope and membership there. Do not add a public `/auth/sessions` mint route; the current TS `DirectoryClient.mintSession()` is stale because the public router lists only `/auth/sessions/me` (`bin.rs:2076-2084`).
2. Add `document_ws_v1` and `directory_ws_v1` handlers that parse the subprotocol grant, consume before `on_upgrade`, attach only a server-owned `SocketSubjectV1`, and send terminal close/audit on invalid binding. Reuse directory replay/filter logic, not new event plumbing.
3. Rework document handshake to use `HelloVNext` without `token`/`actor`, derive role/actor, and stamp/reject mutation actors. Keep the old endpoints untouched only while S3 clients are not migrated; mark them insecure and do not call this packet an end-to-end fix.
4. Hook current `delete_session_me`, `admin_revoke_user_sessions`, share revoke, and identity-verifier revocation path into the ledger binding invalidator. Durable revoke happens before best-effort close, preserving the law already documented for admin revoke (`bin.rs:1928-1943`).

### S3 — separate client migration and insecure-carrier removal

This is intentionally outside the backend-first packet but blocks release:

- React `DirectoryClient.stream` stops creating query tokens (`🧰️framework/🛍️products/💻️os/🟦️.ts:4088-4111`) and obtains a `directory.v1` grant through a bounded bearer-header fetch.
- React backbone worker obtains a document grant, offers subprotocols, and sends credential/actor-free `HelloVNext` (`🟦️backbone-worker.ts:563-601`). Its reconnect loop must mint a new grant per dial, not reuse one after a network drop.
- Native `DirectoryClient` replaces `directory_ws_url(...token...)` (`…/📇️directory/🔌️client/🦀️.rs:364-384`); store sync replaces `PersistenceBinding::Hub.token` Hello uses (`…/🏪️store/🔄️sync/🦀️.rs:1815-1838`).
- MCP hub workspace uses the same native binding acquisition path and must not place `HubOptions.token` in a document frame (`…/🌉️mcp/🏠️workspace/🦀️.rs:433-449`).
- The admin SPA follows the directory migration; it needs no new admin socket.
- Once every caller uses v1, delete `Hello.token`, caller actor, query `token`, and the old insecure routes in the same breaking change. No compatibility carrier or fallback may survive.

Open-plan is deliberately later: it may decide which app/surface to issue after authoritative catalog verification, but this grant only proves transport subject plus structural endpoint scope. A later open plan must neither widen a grant’s scope nor allow client plugin/app identity to affect actor/role.

## Focused runtime laws and commands

Add these focused tests; none was run by this audit.

1. Rust neutral contract: schema/digest/expiry/replay/audience/scope/generation vectors in `🌎️hub/📇️directory/🦀️.rs` and each backend parity test under `🌎️hub/📇️directory/{🪶️sqlite,🐘️postgres,🌐️neo4j}`.
2. Real loopback WebSocket oracle beside `🌎️hub/📦️packages/🦀️rust/📦️bin.rs:2682-2737`: issue a grant via bearer header; connect with two offered subprotocols; assert server chooses only `semio.socket.v1`; send credential-free hello; assert server actor overrides a forged actor; race two upgrades and observe one success; reuse/wrong scope/wrong audience fail; revoke session/share and observe live close plus reconnect denial; assert URL, close, audit, and directory broadcasts never contain the grant.
3. TypeScript fake WebSocket tests in `🧰️framework/🛍️products/💻️os/🟦️.ts:4198-4385` and worker tests: URL has `since`/surface only, grant is offered as a subprotocol, each reconnect calls issue again, and first `HelloVNext` lacks bearer/actor.
4. Native/MCP transport tests use the same neutral fixture and assert an old token-bearing URL/Hello cannot compile/encode after S3.

Focused existing targets after implementation are:

```sh
bun nx run os-hub:test -- --exact socket_grant
bun nx run os-hub:test -- --exact ws_
bun nx run os-hub-admin:test -- --run directory
```

The existing release-local target is `bun nx run os-hub:secure-local-smoke` (`🌎️hub/📦️packages/🦀️rust/📋️project.json:72-80`), but it is not evidence for this packet until S3 exercises its real sockets. Optional PostgreSQL/Neo4j parity must run only where those services are provisioned; SQLite is the zero-touch local baseline.

## Dependency boundary and blocker order

1. **S1: server-owned schema/ledger plus id-only binding revalidation port.** This is the first deterministic blocker: without it a consumed grant cannot remain invalid after durable session/share revoke without retaining the original bearer secret.
2. **S2: HTTP issuance, atomic upgrade consumption, server-derived hello/actor, and binding-index close.** This can land independently of local bootstrap, P2-D CAS, P2-C rebootstrap, open-plan, and catalog work.
3. **S3: all React/native/MCP/admin client migrations, then removal of old routes/fields.** Until this is complete, the current token/actor carrier remains a high-severity production defect.
4. **Later open-plan/catalog policy.** It restricts application/surface capabilities on top of the authenticated structural scope; it is not a prerequisite for secure transport.

Local bootstrap is only a credential delivery source: it already issues durable sessions through a protected local transport (`🌎️hub/🔐️local-bootstrap/🦀️.rs:644-716`). S1/S2 accept those sessions exactly like an identity-verifier session and do not need to alter bootstrap IPC. CAS is only reached after document authentication for rebootstrap and is likewise not a dependency.
