# Authenticated WebSocket Client Migration Audit

## Result

**Do not migrate callers piecemeal.** The present hub has a durable, typed
`session.v1` capability foundation and an authorization generation, but its two
WebSocket handshakes predate it. The first deterministic security failure is a
valid member opening a document socket with an arbitrary `Hello.actor`: the hub
constructs both the security `Principal` and durable sync-session row from that
caller-owned string. It then permits a different caller-owned envelope actor as
well. This is identity/provenance spoofing and corrupts replay/budget partitioning;
it is not role escalation because the role still comes from the real session.

The smallest coherent replacement is a single schema-first, **one-time,
audience- and scope-bound `socket.v1` connect grant**. A long-lived
`session.v1` or `share.v1` capability is exchanged only through a protected
credential broker/REST header boundary; no query string, WebSocket URL, caller
actor, or client-selected authorization generation is accepted. The hub consumes
the grant atomically and derives `actor`, `session`, `generation`, audience, and
scope. It rechecks the durable binding before every authority-bearing frame and
closes all bindings immediately after durable revoke/kick.

This can land before trusted open-plan/catalog work. It preserves the existing
descriptor/hash check and P2-C rebootstrap flow, but does not make those
client-provided values authoritative. The later open plan becomes the authority
for codec/app/surface capability selection.

## Current Source-Backed Wire Census

| Lane | Current ingress and identity | Current revoke/kick behavior | Finding |
| --- | --- | --- | --- |
| Document | `GET /spaces/{space}/documents/{id}/ws?surface=`, then binary `ClientFrame::Hello { schema, pack_schema_hash, actor, token, frontier }`. `actor` and `token` are decoded from the frame. | A one-second ticker reruns `resolve_auth` and only closes on `Denied`; an admin kick is an in-memory `Notify` keyed by a document sync-session id. | **Critical:** caller actor drives `Principal`, color, presence, sync-session actor/client label, and DB hello. The ticker does not require the original session id/generation/role to remain equal. |
| Directory | `GET /directory/ws?token=&since=&spaceId=&documentId=` authenticates during upgrade. | Resource visibility calls the session-id/generation predicate, but the handler never terminally invalidates the socket; heartbeat remains visible without an active-caller check. | **Critical:** bearer is a URL secret and a revoked directory socket remains open, with uneven per-message enforcement. Scope query is also client selected. |
| Admin live connections | Admin REST uses an `Authorization` bearer, but `ConnectionsPage` creates `new DirectoryClient(window.location.origin)` without the admin credential and subscribes to ordinary `/directory/ws`. | Directory WS has no admin channel or durable connection record; the document-only close map cannot close it. | **High:** the claimed admin live stream is not authorized against a real hub; fake-WS coverage masks the failure. |
| React shell / worker | `ShellHost` mints/caches a raw session and passes it to the backbone worker. The worker sends it in `Hello` and hands it to `DirectoryClient`, which places it in the URL. | Reconnect reuses the same raw capability. | **Critical:** a browser worker gets a reusable hub capability; actor is a tab-random client string. |
| Native directory client | `DirectoryClient` stores `String` token, derives a `?token=` URL, and opens it through native or browser transports. | Reconnect recreates the same URL/token. | **High:** the transport seam has no authenticated-first-frame operation or grant provider. |
| MCP upstream and local bridge | `WorkspaceOrigin::Hub` holds a raw token and turns it into a hub persistence binding. Separately, the local MCP bridge uses its own process secret in `GET /bridge?token=`. | Neither is coupled to hub session revocation. | **High:** do not treat the bridge secret as a hub session. Both raw-token carriers must be retired; the bridge query secret is a separate local security defect. |

### Server evidence

- `🌎️hub/📦️packages/🦀️rust/📦️bin.rs:424-442` accepts one generic
  `HubCapability` for a document: session, exact-document share, or public
  fallback. `:935-1002` awaits an unbounded first receive, decodes the
  caller's `Hello`, and creates `Principal::new(actor.clone(), ...)`.
  `:1013-1014` passes the same actor into DB hello. `:1075-1093` persists it
  as the session actor and client label.
- `:863-901` admits a mutation with that principal but passes each
  caller-supplied `envelope.actor`; `db_security::SecurityGate` explicitly
  allows delegated actors and keys replay dedupe by the envelope actor
  (`🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔒️security/🦀️.rs:599-616`).
  Therefore neither layer ties operation provenance to the authenticated
  connection.
- `bin.rs:1111-1156` re-resolves only to detect `Denied`. A role change or
  changed authorization generation which still resolves as any member leaves
  the original write-capable gate in place. Failure to write a sync session is
  ignored and creates an unreachable kick `Notify` (`:1089-1110`).
- `bin.rs:1521-1549` reads bearer/since/scope from `DirectoryWsQuery` and
  authenticates once. The forwarding loop (`:1571-1614`) delegates resource
  visibility to `caller_active` through `directory_message_visible`, but never
  terminally invalidates the socket; its heartbeat arm remains universally
  visible. That is partial egress filtering, not a durable live-session bind.
- Session records contain a digest, expiry, revocation and generation
  (`🌎️hub/📇️directory/🦀️.rs:156-186`); `hub_sync_session` records only
  document-shaped fields (`:201-219`; SQLite DDL
  `🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:141-152`). SQLite and Postgres revocation
  increment the generation and append an auth audit record transactionally
  (`sqlite:285-315`, `postgres:296-336`). The durable substrate is reusable;
  live binding is the missing layer.
- `bin.rs:1817-1845` distinguishes connection close from durable user-session
  revoke, but only matches recorded document sync sessions. `DELETE
  /auth/sessions/me` does a durable revoke but does not broadcast to matching
  live connections (`:1633-1641`).
- Shares are durable selector/digest records scoped to one document
  (`🌎️hub/📇️directory/🦀️.rs:50-62`) and the existing test proves cross-space
  denial/read-only/revoke on the legacy document lane
  (`bin.rs:2686-2721`). This is reusable, but a future socket binding must
  retain the share id, exact scope and read-only mode instead of reducing it to
  `AuthOutcome::ShareToken`.
- P2-C sends a verified control then close `1013`, but its transfer control
  cannot cancel or report progress (`bin.rs:712-764`). The document loop
  rechecks the legacy raw token before a lagged rebootstrap (`:1146-1150`).

### Client evidence and test drift

- The protocol itself makes the unsafe shape canonical:
  `ClientFrame::Hello` includes `actor` and optional `token`
  (`🧰️framework/🔨️modules/📡️replication/📡️wire/🦀️.rs:45-56`), encoding them
  at `:851-891` and decoding them at `:895-929`.
- `PersistenceBinding` has raw `token` and `ArtifactActorConfig` has raw
  `actor` (`🧰️framework/🛍️products/💻️os/🟦️.ts:555-569`). The TS
  `DirectoryClient` is a mutable bearer holder (`:3975-4024`) and makes
  `/directory/ws?token=&since=` (`:4056-4130`); its current URL assertions
  (`:4238-4277`) are migration **test expectation drift**, not a desired
  security contract.
- `🧰️framework/🛍️products/💻️os/🟦️backbone-worker.ts:563-601` sends
  `binding.token` and `state.config.actor` in document Hello. `:1011-1020`
  creates a raw-token DirectoryClient; it retains a bounded local directory
  command queue, which is reusable after the credential carrier changes.
- React mints on `VITE_S_USER` and retains/sends the raw session
  (`ShellHost:1550-1592`). It also synthesizes `client-...` / `user:...` actors
  (`:1263-1269`) and includes the token in every default hub binding
  (`:3361-3390`, `:5440-5442`). These are production defects after the
  secure-session foundation, not only stale tests.
- Native `DirectoryClient` holds the bearer (`🔌️client/🦀️.rs:280-342`), forms
  the credential URL (`:364-385`), and dials it in every reconnect
  (`:448-469`). Its existing `DirectoryWsConnection::send_text` is the useful
  seam for a first text auth frame; both native and `web_sys` transports already
  provide it (`:742-795`, `:925-950`).
- MCP remote binding holds `WorkspaceOrigin::Hub { token }` and copies it into
  `PersistenceBinding::Hub` (`🌉️mcp/🏠️workspace/🦀️.rs:425-450`), while
  `NativeHubBindingDriver` sets the raw DirectoryClient token
  (`🌉️mcp/🏠️workspace/🔗️remote/🦀️.rs:440-479`). The separate bridge accepts
  `?token=` in both its Axum and owned transports
  (`🌉️mcp/🧵️bridge/🦀️.rs:2523-2577`,
  `🌉️mcp/🚚️transport/🦀️.rs:1462-1475`). It is not a `session.v1`, but must
  become a local-bootstrap/first-frame capability too.
- Admin REST correctly uses a bearer (`🔑️AdminSession/🟦️.tsx:40-112`) but
  stores a raw admin session in `sessionStorage` (`:115-170`). Its live page
  intentionally drops that token when making a DirectoryClient
  (`🔴️ConnectionsPage/🟦️.tsx:35-76`). The fake test's URL without a token
  (`admin.test.tsx:168-205`) cannot establish that a real socket is authorized.

## `SocketConnectV1` Contract

Add one shared schema before adapters:
`🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🔣️socket-connect-v1.json`,
with generated Rust/TS DTOs beside the existing directory schema. The binary
document codec owns the corresponding `ClientFrame::AuthenticateV1` tag in
`📡️replication/📡️wire/🦀️.rs`; directory/admin/bridge use the same canonical
JSON record for their first text frame. Do not overload legacy `Hello`.

### Durable capability and record

`socket.v1.<selector>.<secret>` uses the same fixed parser/OS entropy discipline
as `session.v1` (`🌎️hub/📇️directory/🦀️.rs:368-425`), but has the distinct
SHA-256 domain `semio/hub/socket-connect/v1\0`. Persist only selector and digest:

```text
SocketConnectGrantV1 {
  id, selector, secretDigest,
  audience: document | directory | admin-stream | mcp-upstream | local-bridge,
  parent: session(id, authorizationGeneration) | share(id, exactDocumentScope),
  exactScope?: (spaceId, documentId),
  readOnly, clientClass, issuedAt, expiresAt,
  consumedAt?, revokedAt?, correlationId
}
```

Maximum TTL is 60 seconds; a grant is single-use. `consume_socket_grant` is one
backend transaction: constant-time digest check; `consumed_at IS NULL`; expiry/
revocation check; parent session id **and generation** active check plus current
membership/admin policy, or active share exact-scope check; then conditional
`consumed_at` update. The successful transaction returns a private
`AuthenticatedSocketContext`, never the secret. Two parallel consumes yield one
success and one redacted replay close. Revoking a parent invalidates unconsumed
grants by parent lookup and immediately broadcasts that parent binding.

Do not make `session.v1` accepted in any WebSocket first frame. A protected
credential broker may use it in an Authorization header to issue the one-time
grant. `share.v1` can issue only a `document` grant for its exact scope and
`readOnly=true`; it cannot issue directory, admin, MCP, blob, or another-document
grants. An admin grant is issued only after server-side `is_admin` policy, not a
client audience claim. `mcp-upstream` and `local-bridge` grants are disjoint
audiences and prefixes; neither parses as a hub session/share capability.

### First frame, server derivation, and frame law

Every upgrade URL is credential-free:

```text
GET /spaces/{space}/documents/{document}/ws
GET /directory/ws
GET /admin/api/ws
GET /bridge
```

Within 5 seconds, exactly one bounded first frame is required:

```text
SocketAuthenticateV1 {
  schemaVersion: 1,
  audience,
  connectGrant: "socket.v1...",
  document: { schema?, packSchemaHash?, frontier?, resumeToken? }, // document only
  directory: { since? },                                           // directory/admin only
  presentation: { surface? },                                      // document telemetry only
  clientInstanceHint?                                               // telemetry only
}
```

`connectGrant` is redacted at parsing, never put in an error, event, trace, URL,
browser storage, or connection projection. The path (not the frame) defines the
document scope. The server rejects a mismatched audience/scope before descriptor
or DB work. `schema` and `packSchemaHash` retain today's descriptor consistency
check only; `surface` is bounded untrusted peer telemetry and cannot select a
plugin, app, renderer, permission, filter, or document roster. The later
server-issued open plan can validate it against the authorized app/surface
capability.

On consume the server first creates the durable live connection row, then derives:

```text
connectionId = server time-ordered id
actorId      = "socket/v1/" + connectionId
principal    = { actorId, tenant=path.spaceId, current server role }
binding      = { parent id, parent generation/share id, audience, exact scope, readOnly }
```

If the durable row cannot be written, send no Welcome and close: no more
unrecorded/un-kickable sessions. Refactor `SyncSessionRecord` into an explicitly
tagged `LiveConnectionRecordV1` with `channel`, nullable document scope for
directory/admin/MCP, server-derived actor, non-authoritative surface telemetry,
and authenticated parent binding. Apply the same greenfield schema to SQLite,
Postgres, and Neo4j; do not add a migration layer.

After authentication no frame carries token, actor, role, generation, scope, or
audience. The server rejects a second auth/Hello. Before each command,
frontier-advertise, preview, presence write, or directory/admin emission, it
checks the binding's durable parent/generation and current membership. It rewrites
all mutation envelope provenance to `actorId` (or rejects any nonmatching
envelope actor); it must not use the existing delegated-actor exception at this
external boundary. Share and public read-only bindings may receive bootstrap and
commands but may not submit commands, previews, presence, or frontier writes.
Public visibility remains a separate explicit product policy and never creates a
directory/admin/MCP credential.

### Revocation, kick, reconnect, and P2-C

Install one `LiveBindingIndex` in HubState keyed by session id/generation and
share id, not only `syncSessionId`. It owns cancellation senders for every
document, directory, admin, MCP-upstream and bridge socket. After the directory
transaction commits a self revoke, admin revoke, membership removal/demotion,
share revoke, or administrator connection close, publish an invalidation to that
index; a failed publish cannot undo the durable decision. The one-second durable
check is a backstop, not the revocation SLO. It must compare id, expiry,
generation, current role, audience, and exact scope rather than merely
`AuthOutcome::Denied`.

Send a redacted terminal `auth-invalidated` control only after a successful
handshake, then close with a fixed deterministic code: `4401` invalid/revoked or
expired credential, `4403` scope/audience/role loss, `4408` first-frame deadline,
`4409` consumed/replayed grant, and retain `1013 rebootstrap-required`. Never
allow in-place reauthentication: clients must drop the old queue, ask their
credential broker for a new grant, reopen, and resubscribe from their durable
frontier/last directory sequence. A cancellation stops both grant request and
redial. P2-C keeps server-derived scope; on lag it may emit verified rebootstrap
only while the binding remains currently readable. Replace
`SocketRebootstrapControl`'s hardcoded false/no-op methods with the connection's
cancel token and monotonic progress event, bounded by the existing deadline.

## Bounded Safety Rules

| Rule | Contract |
| --- | --- |
| Upgrade/auth | 5-second first-frame deadline, exactly one binary/text frame appropriate to lane, `SocketAuthenticateV1` at most 16 KiB, max 256 UTF-8 bytes each for ids/hints, strict trailing-byte rejection. Existing document hello budget is 64 KiB (`bin.rs:1013-1014`); do not inherit it for credential ingress. |
| Runtime frames | Preserve protocol-specific command limits; impose fixed per-socket in-flight command/preview and byte budgets before decode. Reject malformed/oversize input without echoing bytes. Directory's existing eight-invalid-frame yield cap (`🔌️client/🦀️.rs:448-486`) is a useful client-side precedent, not server validation. |
| Deadlines/cancel | Grant issuance, durable consume, descriptor lookup, and P2-C transfer take an operation deadline and cancellation token. Report `grant-issued`, `auth-consumed`, `descriptor-checked`, and P2-C byte/chunk progress only to the owning local client; do not publish capability or membership diagnostics to a shared stream. |
| Redaction | Fixed public close reason; audit only selector/id, audience, scope hash, outcome, correlation id and code. Never raw capability, bearer, email in cross-space stream, or internal authorization reason. |
| Privacy | Directory and admin streams filter from a server-derived subject. A share is exact document/read-only and cannot observe directory/presence. Document presence remains document-wide roster keyed by structural document scope and server actor; `surface` stays non-authoritative telemetry. |

## Ordered Implementation Packet

1. **Schema and backend contract (critical).** Add `SocketConnectV1`, capability
   parser/domain, `SocketConnectGrantRecord`, `SocketAudience`,
   `AuthenticatedSocketContext`, conditional consume and parent invalidation to
   `🌎️hub/📇️directory/🦀️.rs`. Implement matching schema/table/projection paths in
   `🪶️sqlite/🦀️.rs`, `🐘️postgres/🦀️.rs`, and `🌐️neo4j/🦀️.rs`. Expand the
   live-connection record rather than faking directory/admin rows as documents.
   Add append-only auth audit events for grant issued/consumed/replayed/invalidated.

2. **Hub first-frame authority (critical).** In
   `🌎️hub/📦️packages/🦀️rust/📦️bin.rs`, remove `DocumentWsQuery` credentials/scope
   authority and `DirectoryWsQuery.token`; add grant issue endpoints behind the
   protected broker; implement document, directory, admin, and local-bridge
   first-frame consumers. Replace `resolve_auth` at WS ingress with a typed
   `authenticate_socket`; make connection projection success mandatory before
   Welcome. Replace `session_kicks` with the binding index and route both self and
   admin revoke through it.

3. **Replication and document enforcement (critical).** In
   `🧰️framework/🔨️modules/📡️replication/📡️wire/🦀️.rs`, add
   `AuthenticateV1`/authenticated server control and delete token/actor from
   `Hello` in the same breaking schema change. In `bin.rs:863-931`, accept only
   the server actor, rewrite/reject envelope provenance, and revalidate before
   each action. Make `surface` first-frame telemetry and update the P2-C control
   to use the new authenticated context.

4. **Directory and admin channels (high).** Add an `AdminStream` audience and
   `/admin/api/ws`; do not reuse ordinary directory visibility for admin live
   connections. Update directory stream to send `SocketAuthenticateV1 { since }`
   on open and wait for `Ready` before replay. Update `ConnectionsPage` to use an
   admin credential broker and AdminStream—not a blank DirectoryClient. Keep
   normal directory filtering for member clients and recheck before every emitted
   message.

5. **Credential carrier migration (high).** Replace raw `token` fields in
   `PersistenceBinding`, TS `DirectoryClient`, worker directory-open, and
   `ShellHost` with a `HubCredentialBroker` / audience-specific grant provider.
   The landed `LocalBootstrapClientClass` and delivery interfaces
   (`🌎️hub/📇️directory/🦀️.rs:602-679`) are the boundary: React/admin relays own
   the long session; native/MCP receive it only over their protected native
   delivery and expose audience grants to their client code. Delete
   `mintSession(email)` and `VITE_S_USER` bootstrap. Client local queues may retry
   only after a fresh grant and must surface terminal revocation rather than retry
   indefinitely.

6. **Native/MCP bridge (high).** Change native `DirectoryTransport` to dial a
   token-free URL and send first-frame auth through its existing `send_text` seam.
   Replace `WorkspaceOrigin::Hub { token }` with a non-debuggable
   `McpUpstreamCredentialSource`; restrict it to its audience/space. Change the
   local MCP `/bridge` adapters to a distinct local-bootstrap bridge grant in
   their first frame, retire `?token=`, and never accept a session/share grant
   there. This is independent of full MCP open-plan/catalog routing.

7. **Open plan/catalog follow-on (medium dependency).** Once the channel context
   exists, open-plan binds the already-authenticated subject to the immutable
   descriptor and allowed app/surface capabilities. Catalog loading is not a
   prerequisite for fixing actor/bearer/revoke security; do not make the security
   patch wait for it.

## Neutral Fixtures, Oracles, and Focused Gates

Create language-neutral JSON vectors for grant grammar/domain, decoded first
frame, consume race ordering, audience/scope matrix, parent-generation mismatch,
share exactness, redacted terminal codes, and client reconnect transitions. Have
Rust, TypeScript, and native client codecs all consume the same fixtures.

Add an independent real-socket oracle using the existing `tokio-tungstenite`
client against the Axum hub (not the in-process fake) that verifies:

1. URL queries containing `token`, `actor`, `since`, `spaceId`, or `documentId`
   are ignored/rejected; token-free URL plus one valid grant succeeds once.
2. A second consume races and loses; wrong audience, wrong document, expired
   parent, stale generation, and share-to-directory/admin/MCP attempts close
   without secret reflection.
3. An author cannot choose the resulting actor or envelope actor; authenticated
   server provenance is visible to the neutral fixture oracle.
4. Self revoke, admin revoke, role demotion, share revoke and connection kick
   close document/directory/admin/MCP sockets promptly, persist close/audit state,
   and prevent a stale queued write after reconnect.
5. Directory replay is gap-free from `lastSeq` only after `Ready`; a P2-C lag
   control is delivered only to the authorized exact scope, reports cancellation,
   and a fresh rebootstrap connection has a new actor/connection id.
6. Admin live connection stream is admin-only; an ordinary member/share/public
   client cannot observe cross-space roster, email, role, session, or audit data.

Use existing task routers after the implementation, not a broad simultaneous
Cargo run:

```sh
bun nx run os-hub:test -- quick
bun nx run framework-os:test -- quick
bun nx run os-hub-admin:test
bun nx run framework-os-mcp-rs:test -- quick
```

Run the SQLite real-socket oracle as the zero-touch gate. Run equivalent
PostgreSQL/Neo4j cases only when their configured service is actually ready; an
unavailable backend is a skipped explicit environment prerequisite, never a
green result. The current hub `main` wires both identity verifier and local
bootstrap as `None` (`bin.rs:2106-2147`) and validation rejects a real dev or
production configuration (`:503-531`), so an end-to-end runtime gate is blocked
until the protected LocalBootstrap adapter is actually registered.

## Severity and Dependency Order

1. **Critical — server-derived socket identity and one-time audience grant.**
   Blocks all authenticated collaboration claims.
2. **Critical — durable generation/role/share invalidation per frame plus live
   binding index.** Blocks safe revoke, kick and reconnect.
3. **High — token-free TS/native directory/document migration and dedicated
   admin stream.** Blocks real clients from using the repaired server safely.
4. **High — MCP upstream/local bridge credential separation.** Blocks
   authenticated agent/bridge claims.
5. **High — real LocalBootstrap adapter registration.** Blocks a zero-touch
   runtime even after protocol work.
6. **Medium — P2-C cancellation/progress integration, then open-plan/catalog
   capability selection.** These consume, but must not postpone, the preceding
   transport authority fix.
