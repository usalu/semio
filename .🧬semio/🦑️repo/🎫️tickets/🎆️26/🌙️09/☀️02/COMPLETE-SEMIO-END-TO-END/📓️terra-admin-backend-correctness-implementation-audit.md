# Terra Audit — Session-Derived Admin Backend Correctness Packet

**Scope.** Read-only source audit on 2026-09-03. This reinspected the canonical plan and acceptance matrix; the prior hub-admin, session-security, local-relay, socket-grant, open-plan, catalog, and CAS reports; and the current hub, all three directory backends, shared directory schema, and admin SPA. No production or test source was changed. No build, test, database, browser, socket, or runtime process was run. Statements about a result below describe the current deterministic source path or a proposed implementation law, not runtime evidence.

## Decision

Land **Sol-A: Session-Derived Admin Intent and Snapshot Correctness** before browser relay, browser socket grants, catalog completion, or native mount repair. It is one narrow control-plane packet:

1. replace boolean admin gating with a verified `AdminPrincipalV1` used for every admin read and command;
2. replace the static `Admin` command actor with a server-derived `User` actor, repairing `create-space` and preserving the actual operator/session in event provenance;
3. make all pages and audit reads bounded/cursored, give credential operations real admin attribution, and write append-only operational audit records;
4. make `/admin/api/connections` an exact authenticated **recorded-binding snapshot**, deliberately omitting legacy client-asserted actor/surface/presence fields, and remove the SPA's unauthorised directory stream; and
5. retain the durable-revoke → best-effort-kick ordering, while making its two outcomes explicit and auditable.

It must **not** invent an administrator browser credential carrier, relax the directory member-stream filter, add an admin WebSocket, accept a client actor/package/surface, issue open plans, or expose an operator retention/delete action. Those remain fail-closed behind the stated dependencies.

## Current evidence and exact defects

| Area | Current source path | Consequence |
|---|---|---|
| Verified principal discarded | `is_admin` authenticates the bearer session and matches provider plus subject digest, but returns only `bool` (`🌎️hub/📦️packages/🦀️rust/📦️bin.rs:632-641`). Every admin handler calls it again rather than retaining a session/user/generation. | No command/audit path can faithfully attribute the verified administrator. |
| Deterministic create-space rejection | `/admin/api/commands` writes `{ kind: Admin, id: "admin" }` (`📦️bin.rs:1897-1913`); `DirectoryCommand::CreateSpace` extracts a user only from a `User` actor and rejects other kinds (`🌎️hub/📇️directory/🦀️.rs:1181-1219`). | An otherwise policy-authorised administrator cannot create a space. |
| Normal REST demonstrates the right actor kind but loses session identity | `/directory/commands` resolves the user, authorises the command, then writes `user:{userId}#hub-rest` (`📦️bin.rs:1472-1483`). | This is structurally valid for creation but not session-specific and cannot serve as administrator audit attribution. |
| Connection read is not authoritative | `SyncSessionRecord` persists separate `actor_id` and `client_label` (`🌎️hub/📇️directory/🦀️.rs:203-235`), but the legacy Hello gives `actor` to the document handler and it writes that same caller value into **both** fields (`📦️bin.rs:1036-1048, 1187-1191`). `connection_view` then publishes `client_label` and keys presence by it (`:1365-1386`). | Neither legacy actor field nor `surface`/presence is server-derived before SocketGrant. Selecting `actor_id` alone would merely relabel an untrusted claim. |
| Directory stream cannot be an admin stream | The per-message filter requires a live authenticated caller and current space membership for connection/presence frames (`📦️bin.rs:1586-1600`). `ConnectionsPage` opens a tokenless ordinary `DirectoryClient.stream()` after a REST snapshot (`🌎️hub/🔨️modules/🛡️admin/🧱️elements/🔴️ConnectionsPage/🟦️.tsx:35-76`). | An administrator is not thereby a member; the UI's “live” path is intentionally unauthorised and may be partial/stale. |
| Durable revoke and kick differ | User revoke first calls `revoke_auth_sessions_for_user`, then iterates active sync records and sends in-memory `Notify` kicks (`📦️bin.rs:1937-1953`). A single connection close is only `Notify` lookup/send (`:1924-1935`). | Revocation survives restart and fences the durable generation; kick affects one current socket only. Treating them as synonyms would be unsafe. |
| Recorded-session gap | Document admission swallows `record_sync_session_open` failure and creates an unregistered, unkickable socket (`📦️bin.rs:1187-1208`). | A snapshot is exact only over recorded sessions; this early admin packet must neither claim all live sockets are listed nor repair the document transport without SocketGrant S1/S2. |
| Audit/projection gaps | Domain events are available from `/admin/api/events`, while all three backends retain bounded `AuthAuditRecord`; no admin route reads it. Current session revoke passes `actor_user_id: None`; share issue/revoke likewise has no administrator actor (`📦️bin.rs:690-715, 1884-1894, 1939-1949`; `🌎️hub/📇️directory/🦀️.rs:250-264, 1691-1707`). | Domain, credential, and operator maintenance histories cannot be reconciled to a verified principal. |
| Unbounded/admin-only reads and uncontrolled rebuild | Users/documents use `i64::MAX`; events default to 500 without the core bounded read guard; rebuild invokes the uncontrolled method despite `ProjectionRebuildControl` already exposing cancellation/progress (`📦️bin.rs:1793-1921`; `🌎️hub/📇️directory/🦀️.rs:331-371, 1750-1755`). | An operator action can exceed a bounded response or run without observable/cancellable progress. |

`authorize_directory_command` already models the right distinction: administrator policy can override membership/owner checks, while the emitted event actor can still be a real user (`📦️bin.rs:1418-1450`). Sol-A preserves that distinction instead of treating `DirectoryActorKind::Admin` as a surrogate person.

## Sol-A contract

### Server-only principal and event actor

`authenticate_admin(headers, peer, state) -> Result<AdminPrincipalV1, AdminError>` replaces `is_admin`. It parses exactly one `session.v1` bearer, authenticates it against the durable directory record on **each** REST request, and applies `OS_HUB_ADMIN_SUBJECTS` by constant-time `(provider, subjectDigest)` comparison. It returns no bearer bytes and never derives identity from request JSON, email, peer address, client label, browser class, package, app, surface, or a prior cached decision.

```text
AdminPrincipalV1 {                    // server memory; never a browser DTO
  userId: OpaqueId,
  authSessionId: OpaqueId,
  authorizationGeneration: u64,
  identityProvider: BoundedText,
  identitySubjectDigest: [u8; 32],     // never serialised/audited as text
  expiresAtMs: i64,
  correlationId: OpaqueId,
  peerClass: "admin-rest"
}

DirectoryActor {
  kind: User,
  id: "user:{principal.userId}#admin-session:{principal.authSessionId}"
}
```

The actor string is constructed only by the server after the verification above. `actor_user_id` therefore resolves the real owner for `create-space`; existing-space commands use the existing admin-policy branch before `DirectoryService::execute`, so their event provenance is also the verified human, not a static administrator label. The client sends an intent only; it cannot nominate an actor, owner, auth session, generation, role, package, app, surface, catalog row, descriptor, checkpoint, or storage locator.

### Exact request/response schemas

Put the wire DTOs in the shared directory contract (`🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/{🦀️.rs,🟦️.ts,🔣️.json}`), with the hub retaining `AdminPrincipalV1` as an internal type. They are `additionalProperties: false`, UTF-8 bounded, and have no capability/digest/private-locator fields.

```text
AdminIntentV1 {
  requestId: OpaqueId,                 // client idempotency/retry key; max 256 bytes
  kind:
    directory { command: DirectoryCommand }
  | issue-document-share { scope: DocumentScope, ttlSecs: u32 }
  | revoke-document-share { scope: DocumentScope, shareId: OpaqueId,
                             reasonCode: BoundedCode }
  | revoke-user-sessions { userId: OpaqueId, reasonCode: BoundedCode }
  | kick-connection { syncSessionId: OpaqueId, reasonCode: BoundedCode }
  | rebuild-directory-projections { expectedHeadSeq: u64 }
}

AdminIntentReceiptV1 {
  operationId: OpaqueId, correlationId: OpaqueId,
  state: "succeeded" | "accepted" | "failed" | "cancelled",
  eventSeqFirst?: u64, eventSeqLast?: u64,
  result?: { inviteToken?: oneDisplayOnly; shareToken?: oneDisplayOnly },
  outcome: { code: BoundedCode, durable: boolean,
             kickAttempted?: u32, kickSignalled?: u32 }
}

AdminPageV1<T> { rows: T[], nextCursor?: OpaqueCursor, observedAtMs: i64 }

AdminRecordedConnectionV1 {
  syncSessionId: OpaqueId, scope: DocumentScope,
  authenticatedUserId?: OpaqueId, email?: BoundedText,
  role: DirectorySpaceRole, connectedAtMs: i64,
  source: "recorded-sync-session"
  // No actor, clientLabel, surface, or presenceKnown before SocketGrant.
}

AdminConnectionSnapshotV1 extends AdminPageV1<AdminRecordedConnectionV1> {
  source: "recorded-sync-sessions", headSeq: u64
}

AdminOperationAuditV1 {
  sequence: u64, operationId: OpaqueId, occurredAtMs: i64,
  phase: "accepted" | "succeeded" | "failed" | "cancelled",
  intentKind: BoundedCode, targetKind: BoundedCode, targetId: OpaqueId,
  principalUserId: OpaqueId, principalSessionId: OpaqueId,
  principalGeneration: u64, correlationId: OpaqueId,
  eventSeqFirst?: u64, eventSeqLast?: u64, outcomeCode: BoundedCode,
  reasonCode?: BoundedCode
}
```

`requestId`, the route-generated `operationId`, correlation, IDs, codes, and reasons share the existing 256-byte authentication-text ceiling. One decoded intent is at most 8 KiB. All query pages are 1–100 rows with opaque cursor and a 64-KiB encoded-response ceiling; do not retain the `i64::MAX` list paths. Existing directory event reads stay at their source-owned `DIRECTORY_EVENT_READ_MAX`; the admin event page must call that bounded gate rather than make a new unbounded reader. `ttlSecs` remains subject to the existing capability window. A raw invite/share token appears only in the successful issuance receipt once; it is absent from all audits, projections, list/detail DTOs, URLs, logs, and retry responses.

`POST /admin/api/intents` replaces generic `/admin/api/commands`; `POST /admin/api/operations/{operationId}/cancel` and `GET /admin/api/operations/{operationId}` exist only for the one rebuild operation. Queries become bounded `GET /admin/api/{spaces,users,documents,connections,events,auth-audit,operation-audit}` plus `GET /admin/api/documents/{spaceId}/{documentId}`. This is CQRS: intents append domain/capability/audit facts and queries project them; no generic update/delete/CRUD resource is introduced.

### Command and projection behaviour

| Intent family | Correct Sol-A flow | Current capability / boundary |
|---|---|---|
| Space/member/invite directory command | Authenticate principal → authorise with `admin=true` → derive the `User` event actor → `DirectoryService::execute` → append accepted/terminal operation-audit facts with event sequence range. Thread `principal.userId` to capability audit for the two intentional invite-record exceptions. | Existing decider remains the invariant source for owner, atelier, archive, immutable descriptor and event ordering. `create-space` is repaired without an on-behalf-of owner field. |
| User session revoke | Authenticate → append accepted audit → **durably** revoke all target sessions with `actor_user_id=Some(principal.userId)` and one correlation → read matching recorded live sync sessions → signal available notifies → append terminal audit containing durable result and signalled/attempted counts. | `AuthAuditRecord` remains the credential ledger and receives the actual actor; a failed/missing notify cannot reverse durable revocation. |
| One connection kick | Authenticate → locate one recorded active sync session → signal its local notify → append an `ephemeral-only` terminal admin audit. | It never calls revocation, changes authorization generation, or reports success after restart/missing record. A `not-recorded-or-already-closed` result is explicit. |
| Document share | Authenticate → verify exact stored descriptor scope → issue/revoke through a capability API that accepts `actor_user_id` and correlation → append both credential audit and admin operation audit. | Share metadata query returns id/scope/expiry/revoked state only; add the missing bounded list method to `HubDirectory` and every backend. Issuance semantics can land server-side, but a browser token display/copy UI waits for the BFF relay. |
| Checkpoint/retention query | Read only the already-public descriptor, active published checkpoint, bounded lineage, and public retention DTO. Never serialise `ArtifactCheckpoint` private locators or CAS reservation/reference keys. | Query support is independent of catalog/native mounting but is non-authoritative for opening; it must not offer package/app/surface selection. |
| Retention/change/delete | No Sol-A intent or UI control. | Stay fail-closed until P2-D has durable reservation/reference release, guarded deletion fence and fenced sweep across SQLite/PostgreSQL/Neo4j. `ArtifactDirectoryCommand::AdvanceRetention` remains server-only authority; an admin session actor is not substituted for its trusted authority. |
| Document creation/announcement | No Sol-A public admin intent beyond existing metadata query. | Existing `AnnounceDocument { descriptor }` accepts client descriptor identity, so creation waits for server-owned catalog/descriptor/open-plan work. |
| Rebuild projections | One process-wide exclusive operation. Capture `expectedHeadSeq`, call the existing `rebuild_projections_controlled`, relay monotonic `completedEvents/totalEvents`, and append terminal audit only after success/rollback/cancel result. | It is projection maintenance, not a domain CRUD edit. At most one run, max existing 1,000,000 events, 30-s request/job deadline, explicit `cancelled` before commit, no background resurrection after process restart. |

Operational audit is an append-only backend-parity projection, not a mutable status row: write `accepted` before dispatch and one terminal fact after. A post-commit audit write failure produces `accepted/unknown-terminal` rather than a false “failed” result; reconciliation reads the domain event range and credential audit. This preserves truthful CQRS history. `AuthAuditRecord` continues to audit secret-bearing capability lifecycle, with no raw capability bytes.

### Exact connection authority, without a browser path

The admin snapshot handler uses only `list_active_sync_sessions`, but it maps the trusted recorded binding fields only: sync-session id, route-owned `DocumentScope`, durable-session-resolved user/email/role when present, and connection time. It must **not** call the legacy `connection_view`, expose `actor_id`, `client_label`, `surface`, or `presenceKnown`, or silently rename any of them as an actor. The current handler writes the legacy Hello actor into both actor columns, and its surface/presence key also depend on client input; none are administrator authority before SocketGrant. They may remain diagnostic storage while the old protocol exists, but they are never principals in Sol-A.

Sol-A deliberately has **no** `/admin/api/connections/ws`, no `DirectoryClient.stream` reuse, no query bearer, no first-frame browser alternative, and no exception to `directory_message_visible`. `GET /admin/api/connections` is administrator-policy REST read and says `source: recorded-sync-sessions`; it does not imply a complete transport-level roster while admission can swallow a record-write failure. The later SocketGrant S1/S2 packet must atomically persist each server-derived binding before Welcome, replace legacy actor/surface provenance, and may then offer an administrator grant/first-frame stream. That is the only point at which an actor or “live” state is honest.

## Backend parity and source ownership

| Owner file(s) | Sol-A change |
|---|---|
| `🌎️hub/📦️packages/🦀️rust/📦️bin.rs` | `AdminPrincipalV1` extraction; one auth call per handler; typed intent dispatch; bounded page parsing; trusted recorded-binding snapshot (not legacy `connection_view`); user-derived event actor; durable-revoke/kick receipt/audit; controlled rebuild registry; routes. Keep `directory_message_visible` unchanged. |
| `🌎️hub/📇️directory/🦀️.rs` | `AdminOperationAuditRecord`, bounded cursor/append/list trait methods, share metadata list, actor-aware share/invite credential-audit arguments, `DirectoryService` admin intent helper, and existing controlled-rebuild use. Keep retention authority separate. |
| `🌎️hub/📇️directory/🪶️sqlite/🦀️.rs` | Greenfield schema-in-place `hub_admin_operation_audit` and share metadata projection; transactional append/list/cursor implementation. |
| `🌎️hub/📇️directory/🐘️postgres/🦀️.rs` | Equivalent schema-in-place tables/indexes and transactional implementation. |
| `🌎️hub/📇️directory/🌐️neo4j/🦀️.rs` | Equivalent constrained `AdminOperationAudit`/share nodes and cursor ordering; no fallback implementation. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs`, `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts`, `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🔣️.json`, `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🦀️.rs` | Shared strict intent, receipt, page, audit, document-detail and recorded-binding snapshot DTOs; schema/TS/Rust parity. Do not add an admin stream envelope. |
| `🌎️hub/🔨️modules/🛡️admin/🧱️elements/🔑️AdminSession/🟦️.tsx` | Bounded-page/intent client methods and typed failure states only. It remains a temporary direct-bearer caller until relay work; do not add a carrier here. |
| `🌎️hub/🔨️modules/🛡️admin/🧱️elements/🔴️ConnectionsPage/🟦️.tsx` and `🌎️hub/🔨️modules/🛡️admin/🧱️elements/📚️I18n/🟦️.tsx` | Remove tokenless `DirectoryClient.stream`; show a refreshable recorded-binding snapshot, observed time, explicit non-live state, and no legacy actor/surface/presence claim. Add complete EN/DE strings, `aria-live="polite"` result/error/provenance state, keyboard-accessible refresh/cancel, and row-specific kick confirmation/description. |
| `🌎️hub/🔨️modules/🛡️admin/🧱️elements/🙋️UsersPage/🟦️.tsx`, `🏠️OverviewPage/🟦️.tsx`, `📰️EventsPage/🟦️.tsx` | Only independently usable controls: page/retry state, revoke-vs-kick copy and confirmation, rebuild progress/cancel, and bounded domain/auth/operation audit readers. Share secret, catalog/open, checkpoint and retention screens wait for their own dependencies. |
| Tests/fixtures after implementation | Extend existing hub Rust/admin component tests and backend contracts; add only the neutral fixture/schema and test files below. No current test proves this audit. |

All three backends must either implement the whole added trait surface atomically or fail compilation; SQLite is not an authority substitute for PostgreSQL/Neo4j. Schema changes are greenfield in-place definitions, not migration scripts.

## Fixture, independent oracle, and focused laws

Add `🌎️hub/📇️directory/🧪️fixtures/🧬️admin-intent-v1/🔣️.json` plus a colocated JSON Schema. Use fake `session.v1.*`-shaped strings only as redaction probes—never a usable capability. The fixture has valid and invalid vectors for:

- verified matching subject → derived user/session actor; unconfigured subject, expired/revoked capability, zero/changed generation, malformed/oversized bearer/intent all fail before dispatch;
- `create-space` derives the principal as owner; body-supplied actor/owner/session/generation/package/app/surface fields fail closed; an admin override can alter an existing non-member space while event actor remains that verified user;
- accepted/succeeded/failed/cancelled audit fact ordering; event-sequence correlation; duplicate request id returns its original terminal receipt and never duplicates a capability or domain event;
- durable revoke increments/inactivates targets before a missing/kickable connection is considered; kick never changes a credential; an unrecorded/closed sync id does not become a fake success;
- the pre-SocketGrant admin snapshot omits legacy actor/client-label/surface/presence claims and exposes only recorded binding fields; a forged Hello actor cannot become an authority-bearing admin value;
- exact cursor page limits (0/101 rejected), 8-KiB request and 64-KiB response limits, 256-byte code/reason limit, one-display token absence from all audit/page JSON; and
- rebuild progress starts at 0, is monotonic, cancellation rolls back projection state, and `> DIRECTORY_PROJECTION_REBUILD_MAX_EVENTS` fails before mutation.

Use the Rust `ToValue`/`FromValue` codec and the TypeScript contract parser against the same neutral vectors; independently validate the JSON contract with the repository's existing AJV 2020 validator (`🌎️hub/📦️packages/🟦️typescript/🧪️index.test.ts:21-85`). A focused real-hub law suite (after implementation) must use test-issued verified sessions, not arbitrary-email minting or static administrator tokens, and prove only:

1. two distinct configured administrator sessions emit distinct session-derived user actors and create spaces owned by their respective verified users;
2. a configured admin who is not a space member can issue allowed existing-space intent through policy override without changing the event actor to static `admin`;
3. the REST snapshot is policy-authorised, contains only recorded bindings and no legacy actor/surface/presence claim, and no ordinary directory stream leaks all-space connection telemetry;
4. revoke is durable when a live-notify lookup fails; a successful kick closes/signals only the named recorded live session and has no auth-generation effect;
5. all three backends round-trip identical audit/share/page semantics; and
6. rebuild cancellation/progress obeys the existing rollback law and leaves an explicit audit terminal outcome.

These are proposed verification laws. They have not been run in this audit.

## Dependency boundary and blocker order

### Can land now, independently

- Session-derived principal/actor, typed directory intent, bounded read/audit projection, share capability actor attribution/metadata, explicit revoke-versus-kick receipt, and controlled rebuild use only durable directory/session state.
- The narrowed recorded-binding REST snapshot plus removal of the tokenless UI stream is independently safe: it removes false actor/live claims rather than adding access.
- The limited EN/DE/a11y status/confirmation/progress changes above are independently usable by the existing UI shape. They do not make manual browser bearer storage acceptable or complete an administrator sign-in journey.

### Must remain fail-closed

1. **Local relay / browser credential transport.** No new browser endpoint, cookie, query bearer, session storage path, or direct-hub UI journey. Admin UI is operationally complete only after the per-profile local BFF relay keeps the bearer outside browser code.
2. **SocketGrant S1/S2.** No admin live stream; no claim that every socket is recorded/kickable; no repair to the old `Hello` token/client-actor carrier in this packet. Socket grant admission must later derive actor/session/share generation and persist before Welcome.
3. **Catalog/open plan/native mount.** No client-selected descriptor/catalog/package/app/surface, no document creation from `AnnounceDocument`, no open/mount assertion. Admin metadata views cannot certify an artifact is openable.
4. **P2-D CAS/retention.** No retention advance, blob delete, sweep, or checkpoint publication control until durable reservations/references, retention/space-delete release, guarded deletion fence, and fenced sweep exist across every backend.
5. **Production identity substrate.** The current process still lacks a production verifier and protected development bootstrap registration; Sol-A consumes a verified session when one exists but does not make the configured launch able to issue one.

**Blocker order:** (1) land Sol-A backend/schema/backend-parity correction; (2) install protected local relay/direct-child authority so admin REST becomes browser-safe; (3) land SocketGrant S1/S2 and atomic sync-session admission, then an explicit admin first-frame stream if still needed; (4) complete verified catalog/server-owned descriptor/open plan and native mounting; (5) complete P2-D retention/reference deletion before any destructive artifact control; (6) prove the integrated relay/socket/catalog/CAS/native/admin journey with the focused laws and a real two-profile browser oracle.
