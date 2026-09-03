# Authenticated SocketGrant Current Implementation Audit

Date: 2026-09-03  
Scope: read-only refresh against the shared tree after secure local bootstrap/readiness and P2-C/P2-D work. No production or test source was changed; no build, test, or runtime probe was run.

## Decision

**ACCEPT one combined S1+S2 server vertical slice now; REJECT calling it an end-to-end transport-security fix until the later caller cutover is complete.**  It needs neither catalog/open-plan policy nor P2-D CAS.  It consists of a volatile, issuer-process `socket.v1` ledger, a small durable *id-only* binding-read port in each directory backend, issuance routes, and new v1 upgrades.  The v1 routes have no bearer fallback.  Existing socket routes remain demonstrably insecure until their callers are cut over and then removed.

This is not a durable cross-process grant design.  An issuer and its upgrade must reach the same hub process.  In the current single-process hub shape that is a valid bounded first packet; a multi-instance ingress needs explicit affinity before this ledger is deployable there.  Do not pretend that an independently load-balanced HTTP issue and WebSocket upgrade will converge.

## What Is Current

| Boundary | Current evidence | Result |
| --- | --- | --- |
| Document socket | `document_ws` upgrades without header authentication at `🌎️hub/📦️packages/🦀️rust/📦️bin.rs:957-960`; `handle_ws` decodes `Hello { actor, token }` and resolves that bearer at `:1193-1206`. | A long-lived session/share capability is in an app frame and the caller selects the connection actor. |
| Live revalidation | Document sessions re-run `resolve_auth` on the captured `Hello.token` once per second (`:1369-1376`) and before a lag rebootstrap close (`:1403-1407`). | Revocation is eventually noticed, but only by retaining and reusing the bearer. |
| Directory socket | `/directory/ws` parses `token`, `since`, and optional scope (`:1780-1791`) and resolves `?token=` before subscribing (`:1804-1807`). | The bearer is in the URL. |
| Replication framing | `ClientFrame::Hello` serializes mandatory `actor` and optional `token` in `🧰️framework/🔨️modules/📡️replication/📡️wire/🦀️.rs:48-56,851-865,912-921`; the TS twin is at `…/📡️replication/🟦️.ts:217-235,1072-1110`. | A secure v1 handshake needs a distinct credential-free frame, not a server convention around this encoding. |
| Command identity | `admit_writes` passes each client envelope actor into the security gate (`bin.rs:1121-1125`) and `submit_commands` persists and relays those envelopes (`:1090-1110,1155-1157`). | Ignoring only `Hello.actor` is insufficient; the v1 writer must bind/reject envelope actors. |
| Durable identity | `AuthSessionRecord` stores id, expiry, revocation, and authorization generation (`🌎️hub/📇️directory/🦀️.rs:158-177`); `ShareTokenRecord` stores id, selector, exact document scope, expiry, and revocation (`:53-65`). SQLite, PostgreSQL, and Neo4j authenticate/revoke those records. | The durable inputs needed by a volatile grant are present. |
| Bootstrap/readiness | Bootstrap issues an ordinary durable session at `🌎️hub/🔐️local-bootstrap/🦀️.rs:701-725`; readiness explicitly says `publicSessionIssuance: false` (`bin.rs:667-732,2611-2628`). | Bootstrap is just a protected bearer-delivery source.  It does not issue or validate socket grants. |
| P2-C/P2-D | Rebootstrap is reached after document authentication (`bin.rs:984-1022,1403-1407`). CAS coordinator/maintenance and readiness are created at `:2440-2499`. | Neither adds an authenticated socket carrier, actor binding, ledger, or catalog prerequisite. |

No `SocketGrant`, `socket.v1`, `HelloVNext`, or id-only socket-binding port exists in the current source.  The router has only the old document and directory WebSocket routes (`bin.rs:2249-2278`).

## Corrections To The Earlier Audit

1. The current first-party capability grammar is lower-case hexadecimal, not base64url: 16 selector bytes and 32 secret bytes become `session.v1.<32 hex>.<64 hex>` (`🌎️hub/📇️directory/🦀️.rs:393-540`; schema `🌎️hub/🔐️auth/🧬️schema/🔣️.json`).  Define `socket.v1.<32 hex>.<64 hex>`—exactly 107 characters—with digest domain `semio/hub/socket/v1\0`.  Do not introduce a second token encoding.
2. The previous recommendation not to disclose the derived actor conflicts with the current mandatory `MutationEnvelope.actor` causal field (`🧰️framework/🔨️modules/📡️replication/🔗️causal/🦀️.rs:38-46,829-858`).  The issue receipt must return an opaque, non-secret `actorId`, or the client must receive it before it creates any v1 command.  It is an identity label, already observable in presence/relay frames, not a capability.  The server must reject—not rewrite—an envelope whose actor differs from the grant subject, preserving causal identity.
3. The admin live consumer is not merely URL-token-bearing: `ConnectionsPage` creates `new DirectoryClient(window.location.origin)` and discards the verified `AdminClient.token` (`🌎️hub/🔨️modules/🛡️admin/🧱️elements/🔴️ConnectionsPage/🟦️.tsx:50-74`).  The real handler therefore fails closed before it can stream.  Its fake test accepts an unauthenticated URL (`…/📦️packages/🟦️typescript/🧪️admin.test.tsx:171-188`).  A normal `directory.v1` member grant is also not an admin substitute: `directory_message_visible` requires membership for Connection/Presence telemetry (`bin.rs:1748-1762`), while admin REST sees all spaces.  Exclude admin from S1+S2; later give it an explicit admin-directory audience authorized by the existing verified-subject policy (`bin.rs:794-805`), or remove its live stream.
4. The public `/auth/sessions` mint assumed by several clients is still absent from the router.  The new protected issue endpoints must use an already delivered session/share capability; they must not restore public session issuance.

## S1+S2 Contract

### Capability and receipt schema

Extend `🌎️hub/🔐️auth/🧬️schema/🔣️.json`, its TypeScript nominal parser `…/🧬️schema/🟦️.ts`, and neutral fixture `🌎️hub/🔐️auth/🧪️fixtures/🧬️capability-v1/🔣️.json` with a separately parsed `SocketGrantCapabilityV1`.  In `🌎️hub/📇️directory/🦀️.rs`, add `CapabilityKind::Socket`/`SocketGrantCapability` but deliberately do **not** add it to `HubCapability`: a socket grant is never an HTTP bearer.

The issue receipt is exactly:

```text
schema: "semio.hub.socket-grant/v1"
protocol: "semio.socket.v1"
grant: socket.v1.<32 lower-hex selector>.<64 lower-hex secret>
actorId: "hub.v1.<64 lower-hex>"
expiresAtMs: integer
```

The ledger retains only `selector`, domain-separated secret digest, audience, exact scope, opaque actor id, immutable binding, issued/expiry timestamps, and state.  It never retains plaintext grant, session/share bearer, URL, email, or private artifact locator.

`audience` is either:

```text
document.v1: { spaceId, documentId }
directory.v1: { authSessionId, authorizationGeneration }
```

Document issuance accepts a valid session bearer with current membership or an active exact `(space, document)` share bearer.  A session subject records session id, user id, generation, current role, and a stable server-derived opaque actor; a share subject records share id, selector, exact scope, spectator role, expiry, and a per-grant opaque actor.  Public visibility has no form in this authenticated packet.  Directory issuance accepts only an active session binding.  Neither audience grants administrator authority.

Use the existing 256-byte auth-text ceiling for ids, scopes, actor, correlation, and diagnostics; a 30-second maximum socket TTL; at most 64 unconsumed grants per session binding and per share binding; one 2-second operation deadline; and a finite process-wide ledger cap.  Refuse capacity rather than evicting a valid grant.  Expiry is swept on issue and consume.  Issue/consume have no user-visible progress event because they are bounded, but must observe cancellation before entropy, before durable read, and before state transition.

### Durable binding port and backend work

Keep the grant ledger out of SQLite/PostgreSQL/Neo4j.  Add a narrow `HubDirectory` read port in `🌎️hub/📇️directory/🦀️.rs` and dispatch it through `HubDirectories`:

```text
socket_session_binding(sessionId, userId, authorizationGeneration, spaceId?, now)
  -> active { role?, expiresAtMs } | revoked | expired | membership-lost | unavailable
socket_share_binding(shareId, selector, scope, now)
  -> active { expiresAtMs } | revoked | expired | unavailable
```

Each implementation must read by durable id and generation/selector only, never reparse a bearer.  Implement it with the existing auth-session/share tables/nodes in `🌎️hub/📇️directory/🪶️sqlite/🦀️.rs`, `🌎️hub/📇️directory/🐘️postgres/🦀️.rs`, and `🌎️hub/📇️directory/🌐️neo4j/🦀️.rs`; the zero-touch in-memory coverage is SQLite `:memory:` used by hub tests.  The current `authenticate_share` returns only `bool`, so add a separate authenticated-share-binding resolver at issuance to obtain the durable share id/selector.  Do not weaken it to a selector-only boolean.

Revocation wiring must invalidate by durable id, not post-revocation generation: `delete_session_me` (`bin.rs:1900-1908`) and `admin_revoke_user_sessions` (`:2097-2112`) already receive revoked session ids; `revoke_share` receives the share id (`:868-875`).  Each must call `SocketGrantLedger::invalidate_binding` only after the durable revoke succeeds.  That removes unconsumed grants and wakes every matching live v1 socket.  A failed wake can never undo the durable revoke.  There is currently no running identity-verifier revocation callback—startup sets `identity_verifier` to `None` (`:2414-2418`)—so do not claim one; any future caller of `revoke_auth_sessions_for_identity` must use the same invalidator.

### Ledger race law

Put `SocketGrantLedger` and its binding-to-pending/live indexes in `HubState` (`bin.rs:411-468,2464-2488`).  It is intentionally empty after restart; a client must issue again.  Its one legal consume path is:

1. Parse and bound header outside the lock; look up a copied pending record.
2. Revalidate the copied id-only durable binding outside the lock.
3. Reacquire the ledger lock and atomically require the same unexpired pending record and active binding index, then mark it consumed permanently.  A concurrent invalidation wins if it removed the record first.
4. Consume is never rolled back if TCP/WebSocket upgrade or first Hello fails.  A replay can therefore produce at most one upgraded candidate.
5. Before `Welcome`, register the live notifier and revalidate again.  A revoke in the consume-to-upgrade window closes before authority-bearing frames.  Live sockets revalidate before every command and at the one-second tick; backend unavailable is fail-closed (`1013`), revoked/expired/membership loss is terminal (`4401`).

This ordering means `referenced binding revoked => no pending grant and no continuing authority`, without an await while the ledger mutex is held.  It also handles a role/membership change that has no direct revocation callback.

### HTTP and upgrade shape

Add these protected endpoints in `🌎️hub/📦️packages/🦀️rust/📦️bin.rs`:

```text
POST /spaces/{spaceId}/documents/{documentId}/socket-grants
POST /directory/socket-grants
GET  /spaces/{spaceId}/documents/{documentId}/socket/v1?surface=
GET  /directory/socket/v1?since=&spaceId=&documentId=
```

Issuance uses `Authorization: Bearer session.v1|share.v1` once, validates exact audience/scope, and returns only the receipt above.  URI fields, header value count/bytes, subprotocol count, and `surface` are bounded before allocating/parsing.  The new URI may contain only non-credential routing metadata (`surface`, `since`, optional paired directory scope).

The v1 upgrade requires exactly this ordered offer and no third protocol:

```text
Sec-WebSocket-Protocol: semio.socket.v1, socket.v1.<selector>.<secret>
```

Read the raw header before `on_upgrade`, check the fixed grammar/digest/scope/audience, atomically consume, then select only `semio.socket.v1` through Axum's upgrade protocol selection.  Never echo the grant, put it in a close reason/audit/debug response, or permit it in `Authorization`/query on the v1 route.  Reject malformed/missing/replayed/wrong-scope/expired grants before upgrade with one redacted failure shape.

Add `ClientFrame::SocketHelloV1` as a new explicit binary tag in `🧰️framework/🔨️modules/📡️replication/📡️wire/🦀️.rs` and the TS mirror `…/📡️replication/🟦️.ts`; do not change the insecure tag-0 layout under existing callers.  Its bounded fields are `wireVersion`, `protocolVersion`, `schema`, `packSchemaHash`, `resumeToken?`, and `frontier?`—never actor or token.  The v1 handler accepts exactly that first binary frame within the handshake deadline, then constructs principal, session audit record, color/presence key, relay origin, and security gate from `SocketSubjectV1`.  A v1 `Commands` actor must equal receipt `actorId`; reject before `SecurityGate`/storage otherwise.  No client-supplied actor is silently rewritten.

For P2-C, lag handling first checks the id-only subject binding.  A valid v1 document socket still receives the existing storage-key-free rebootstrap control then `1013`; an invalid binding gets only authorization close.  Rebootstrap does not renew, serialize, or otherwise preserve a consumed grant; reconnect issues a fresh one.  P2-D remains below this boundary and is never read by issue/consume.

## Focused Acceptance Oracles

Add the fixture/schema and a separately implemented SHA-256 oracle for the `socket.v1` vector.  The existing auth fixture family is the neutral source; validate it independently from the Rust capability implementation (for example the existing Bun/WebCrypto lane), including wrong kind, uppercase, wrong width, secret/domain mismatch, and receipt redaction.

Add hub loopback WebSocket tests adjacent to `bin.rs`'s existing socket test support (`:2700-2779`):

1. Issue via bearer, offer both protocols, assert only `semio.socket.v1` is selected, send `SocketHelloV1`, and prove the server-recorded actor equals the receipt actor.
2. Race two upgrades on one grant: exactly one reaches the v1 handshake; expiry, wrong document, wrong audience, reordered/duplicated protocol, wrong digest, and restart all fail without leaking the grant.
3. Revoke session/share after issue and after live open; pending grant is unusable, live socket gets terminal close, and no command after revocation reaches the gate.  Change membership/role and verify the tick/pre-command recheck is fail-closed.
4. Send a command with a forged envelope actor and prove storage/relay receive nothing; a matching actor has the ordinary authorization result.
5. Force lag with a live v1 subject and prove rebootstrap control still precedes `1013`; revoke first and prove that control is not disclosed.
6. Assert URL, selected protocol, error/close strings, auth audit, directory event, and readiness JSON contain neither raw grant nor secret digest.

Backend parity must exercise the id-only port for active, expired, revoked, generation-changed, and membership-lost records in SQLite, PostgreSQL, and Neo4j.  No test was run by this audit.

## Dependency And Cutover Order

1. **S1+S2 now:** capability/schema/vector, three-backend id-only binding port, volatile ledger, explicit v1 handshake tag, issue routes, v1 routes, invalidation, and the neutral/loopback oracles above.  This is the smallest useful Sol packet; splitting ledger-only from routes produces no usable authority boundary.
2. **S3 document and normal-directory callers:** migrate `🧰️framework/🛍️products/💻️os/🟦️backbone-worker.ts:540-606`, `…/🦀️store/🔄️sync/🦀️.rs:1808-1840,3192-3235`, browser/native directory clients (`🟦️.ts:4050-4140`, `…/📇️directory/🔌️client/🦀️.rs:364-505`), and MCP workspace binding (`…/🌉️mcp/🏠️workspace/🦀️.rs:425-449`).  Every reconnect obtains a new grant; no grant enters a URL or a persisted binding.  The derived public actor is installed before client command construction.
3. **Separate admin decision:** either add a narrowly audited admin-directory audience and an admin-only v1 stream, or make the connections panel snapshot/poll only.  Do not silently widen `directory.v1` visibility.
4. **Single breaking removal:** once all real callers use v1, delete old `Hello.actor`, `Hello.token`, tag-0 document WS acceptance, query-token directory WS, old token-bearing fixtures, and token fields from hub persistence bindings.  There is no v1 fallback or compatibility carrier.
5. **Later only:** open-plan/catalog can further restrict application/surface semantics, but cannot widen the fixed transport scope; P2-D/CAS affects post-auth bootstrap only.

## Acceptance Boundary

Accept the combined packet only when all v1 invariants and focused oracles above hold, with the single-process/affinity prerequisite documented.  Reject release acceptance until S3 and the admin decision remove every live bearer/actor carrier.  Secure bootstrap/readiness and P2-C/P2-D are useful foundations, but they do not supersede any of those release blockers.
