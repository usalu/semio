# Hub Authentication and Session Security Audit

Status: read-only source audit, 2026-09-03.  No build, server, socket, credential provider, or backend was run.  This is a snapshot of a concurrently edited tree; every conclusion below is tied to the listed current source seam.

## Decision

**Do not expose the current hub to any peer that is not fully trusted.** `POST /auth/sessions` converts an arbitrary email address into a live 30-day bearer for that user, including an already-existing user.  The document WebSocket separately accepts the caller's requested actor, so a member can impersonate another actor in presence and command attribution.  Both are high-severity production defects, not test drift.

The smallest clean greenfield replacement is a typed, high-entropy, digest-only session capability issued only after a trusted identity assertion, with a server-derived WebSocket actor.  There is no usable production password or SSO verifier today: the persisted `password_hash`, `sso_subject`, and `sso_provider` fields are data only.  Therefore the public mint route must be deleted, not relabeled.  Production must fail closed if no `IdentityAssertionVerifier` is configured.  Development can retain zero-touch onboarding only through an explicit OS-local bootstrap transport (Unix-domain socket on Unix; ACL-restricted named pipe on Windows), never an HTTP route and never a listener reachable on a network interface.

This is intentionally a greenfield wire/schema cut: no token, route, or storage compatibility is retained.

## Current authority path

| Concern | Current concrete path | Finding |
| --- | --- | --- |
| Public session mint | `🌎️hub/📦️packages/🦀️rust/📦️bin.rs:483-492,546-554,1921-1924` accepts `{email}`, finds or creates that user, and returns `session.id` as `token`; the router publishes it at `/auth/sessions`. | **Critical.** Any network peer that reaches the hub can mint a bearer for `owner@…`, not merely create its own user.  The comment expressly confirms no password/SSO check. |
| Automatic client use | Rust `DirectoryClient::mint_session` posts that route at `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs:336-342`; identity boot falls from a `/me` 401 to mint at `…/📇️directory/🪪️identity/🦀️.rs:158-194`.  The TS client does the same at `🧰️framework/🛍️products/💻️os/🟦️.ts:3989-3998`. | This is a production bootstrap path, not a test-only shortcut.  Removing the route needs a replacement identity acquisition path in both clients. |
| Credential capability | `UserRecord` includes `password_hash`, `sso_subject`, and `sso_provider` in `🌎️hub/📇️directory/🦀️.rs:63-75`; `HubDirectory` can look up email/SSO subjects around `:972-1005`. | **High.** No password verifier, password-KDF contract, OAuth/OIDC/JWT verifier, verified-email proof, identity-link policy, or sign-in endpoint consumes these fields.  They must not be treated as authentication. |
| Bearer storage and entropy | `AuthSessionRecord` has only `id`, user, timestamps, and optional provider (`🌎️hub/📇️directory/🦀️.rs:130-138`).  SQLite generates `id = time_ordered_id()` and stores/looks it up verbatim (`🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:568-588`); PostgreSQL and Neo4j repeat it (`…/🐘️postgres/🦀️.rs:647-672`, `…/🌐️neo4j/🦀️.rs:579-621`). | **High.** Database value is bearer secret; it has no digest, selector, revocation timestamp/reason, device/actor binding, or user-wide/session-generation revocation. `time_ordered_id` deliberately falls back to predictable clock/PID-seeded data when entropy fails (`🧰️framework/🛍️products/💻️os/🔨️modules/🪪️identity/🦀️.rs:98-115`). It is unsuitable for credentials. |
| Authorization resolution | `resolve_auth` reads that raw value, checks expiry, then membership (`🌎️hub/📦️packages/🦀️rust/📦️bin.rs:408-421`); directory routes duplicate raw lookup in `resolve_bearer_user` (`:1116-1135`). | Two independently maintained authorization parsers make token-type confusion and revocation bugs likely. |
| Document WS identity | First `ClientFrame::Hello` destructures client-supplied `actor` and `token` (`🌎️hub/📦️packages/🦀️rust/📦️bin.rs:870-936`), builds `Principal::new(actor.clone(), …)`, then records/uses it for command, preview, color, and presence (`:810-870,948-1049`). | **Critical.** A valid member can select another member's actor.  The authorization token establishes only a role, not authorship.  An envelope actor is not checked against a server-derived subject before submission. |
| Live revocation/kick | Each document socket re-resolves its original token on a one-second tick (`🌎️hub/📦️packages/🦀️rust/📦️bin.rs:1025-1064`).  Sync session records store user/role/client label but not an auth-session id (`:1002-1024`; Postgres schema `🌎️hub/📇️directory/🐘️postgres/🦀️.rs:117-127`). | Revocation is delayed and a directory write failure yields an un-kickable live socket.  There is no durable relation from an auth session to its sockets. |
| Session self-revoke and admin revoke | `/auth/sessions/me` reads/deletes the raw token (`🌎️hub/📦️packages/🦀️rust/📦️bin.rs:1557-1573`). `admin_revoke_user_sessions` is routed at `:1921-1941`; its current comment says it cannot enumerate browser auth sessions and it only notifies active sync sessions (`:1778-1795`). | **High.** Admin “revoke sessions” does not revoke stored bearer capabilities.  It cannot terminate a disconnected token that will reconnect. |
| Admin authorization | `is_admin` compares a raw `OS_HUB_ADMIN_TOKEN`, or grants all loopback peers when absent (`🌎️hub/📦️packages/🦀️rust/📦️bin.rs:449-460`; startup enables/warns at `:2057-2080`). Admin commands bypass normal user authorization (`:1728-1742`). | **High.** Any loopback process is an administrator in default launch.  A static raw bearer has no lifecycle, subject, expiry, audit, or constant-time comparison contract. |
| Directory WS credential exposure | Server accepts `?token=` in `DirectoryWsQuery` (`🌎️hub/📦️packages/🦀️rust/📦️bin.rs:1454-1477`). Rust client constructs that URL (`🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs:364-468`); TS constructs it too (`🧰️framework/🛍️products/💻️os/🟦️.ts:4056-4068`). | **High.** Bearers leak into browser history/diagnostics, proxy/access logs, telemetry, and referer-adjacent tooling. |
| Share capability | Shares use OS entropy (`secure_share_token`, `🌎️hub/📇️directory/🦀️.rs:256-263`), but store and equality-query the plaintext token (`🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:385-413`) and `resolve_auth` tries a bearer first as session then share (`🌎️hub/📦️packages/🦀️rust/📦️bin.rs:392-414`). | **Medium.** Scope is document-read, which is narrower than a session, but raw storage and an untyped shared bearer channel are unsafe and invite future confused-deputy mistakes. |
| Invite capability | SQLite generates the invite token with `time_ordered_id` and stores it raw (`🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:592-608`); PostgreSQL and Neo4j repeat it (`…/🐘️postgres/🦀️.rs:676-685`, `…/🌐️neo4j/🦀️.rs:625-635`). | **High adjacent defect.** An invite grants membership but is not cryptographically generated or digest-only.  Leaving it behind defeats the session repair. |
| Existing E2E assumption | The opt-in two-user test explicitly mints `user1@semio.dev` and `user2@semio.dev` (`🌎️hub/📦️packages/🟦️typescript/🧪️index.test.ts:425-447`) and remints after restart (`:563-586`). | Test expectation drift: it encodes the unsafe bootstrap.  It must be replaced, not preserved as product behavior. |

`semio_framework_hash::Sha256` is already an owned framework implementation used by hub authority code (`🧰️framework/🔨️modules/🔢️hash/🦀️.rs:5-138`, `🌎️hub/🗿️artifact-authority/🦀️.rs:6`).  `fill_entropy` is an existing OS-backed, error-returning entropy boundary.  They permit a secure implementation with no new runtime dependency; do not reuse `time_ordered_id` for a secret.

## Threat matrix

| Threat / precondition | Current result | Severity | Required invariant after the cut |
| --- | --- | --- | --- |
| Remote or LAN caller submits another person's email to `/auth/sessions`. | Gets their account's valid bearer and any memberships. | Critical | No public route accepts an identifier as proof.  Only a verified identity assertion or local-only development bootstrap can issue a session. |
| Authenticated author chooses `Hello.actor = user:owner#…`; sends presence or commands. | Server accepts that actor as principal/attribution. | Critical | Actor is computed server-side from authenticated user, server session id, and bounded device instance; client cannot select or override it. |
| Database backup/read, SQL log, or URL/access log leaks a session/share/invite token. | Token is immediately replayable. | High | Persistence holds only fixed-length digests and public selectors/ids; WS tokens never occupy query strings; raw token is returned once over the authenticated channel. |
| Admin calls user-session revoke while bearer is disconnected. | No auth session is revoked; bearer reconnects. | High | Atomic `revoke_all_for_user` marks every active capability revoked, increments authorization generation, then signals all mapped live sockets. |
| Session is revoked while a socket is handling a command. | Up to one-second continued authority; socket may be un-kickable when record write failed. | High | Every admitted frame checks an in-memory/durable session generation captured at auth; revocation invalidates before accepting the next frame and closes all sockets indexed by auth-session id. |
| Any local process reaches a hub without `OS_HUB_ADMIN_TOKEN`. | It is an administrator. | High | Production refuses startup without verified administrator subject policy; development administrator capability originates only from protected local bootstrap IPC. |
| A share token is sent to a directory/admin/blob endpoint, or an auth token is tested as a share token. | Untyped fallback invites accidental cross-use. | Medium | Tokens have disjoint prefixes and parser types; share capability is accepted only by its exact document read gate; directory, admin, invite acceptance, and blobs reject it. |
| Guessable/fallback RNG invite is observed. | Membership grant can be guessed/replayed. | High | Invites use the same no-fallback 256-bit secret + digest scheme, separate `invite.v1` type and scope. |
| Local identity cache is read by another desktop user/process. | Cached plaintext session is usable; native code writes `identity.json` directly. | Medium | Store bearer in OS credential storage behind an owned interface; if unavailable, fail closed for persistent auth rather than downgrade to a predictable/file secret.  Atomic, user-private cache may retain only non-secret profile metadata. |

## Replacement contract and schema

### 1. A single credential taxonomy

Create an owned `HubCapability` parser and return one of `SessionCapability`, `ShareCapability`, or `InviteCapability`; exact grammar and prefix are part of the directory schema fixture:

```
session.v1.<public-selector>.<256-bit-random-secret>
share.v1.<public-selector>.<256-bit-random-secret>
invite.v1.<public-selector>.<256-bit-random-secret>
```

Generate the secret only through `fill_entropy(&mut [u8; 32])`; an entropy error is an issuance failure.  Hash domain-separated canonical bytes such as `"semio/hub/session/v1\0" || secret` with the existing owned `Sha256`, encode a fixed-size digest, and compare it in constant time.  The selector is an opaque lookup index, never authorization by itself.  Do not use raw bearer strings as primary keys or return a stored record containing a raw secret.

All three directory backends must receive the same projection from the same model/port before either becomes runnable:

| Projection | Required fields |
| --- | --- |
| `hub_auth_session` / `AuthSessionRecord` | public `id`, `selector`, `secret_digest`, `user_id`, `identity_provider`, `identity_subject_digest`, `issued_at`, `expires_at`, `revoked_at`, `revoked_reason`, `authorization_generation`, bounded `device_instance_id`, and `session_kind` (`external` or `development-local`). |
| `hub_sync_session` / `SyncSessionRecord` | `auth_session_id` nullable only for deliberate anonymous public/share readers, `authorization_generation`, server-derived `actor_id`, and existing document/surface lifecycle fields. |
| `hub_share_grant` | selector plus digest rather than plaintext token; document scope, issue/expiry/revocation. |
| `hub_space_invite` | selector plus digest rather than raw `time_ordered_id`; scope, role, issue/expiry/revocation and one-time/acceptance state if the product requires it. |
| `hub_auth_audit` | append-only `id`, occurred-at, event kind, auth-session/public id, target user id, actor user id when applicable, provider, outcome/reason code, correlation id, and privacy-minimized peer class.  Never persist raw credential, email assertion, IP, subject, or secret in the event payload. |

Use checked TTL arithmetic and reject non-positive/over-budget TTLs.  Revocation is a timestamped state change, not a delete: it preserves security auditability and makes an active session unambiguously invalid.  Ports must expose `authenticate(capability)`, `revoke_session`, `revoke_sessions_for_user`, `revoke_sessions_for_identity`, `record_sync_session_open(auth_session_id, generation, actor)`, and a transactionally returned list of revoked ids/generations.  Remove raw `get_auth_session(id)` / `revoke_auth_session(id)` as HTTP-bearer primitives.

### 2. Trusted identity issuance, not password-shaped data

Introduce an owned async port in `🌎️hub/📇️directory/` (not an SDK type in a public API):

```
IdentityAssertionVerifier::verify(assertion, cancellation)
  -> VerifiedIdentity { provider, subject, verified_email?, display_name?, issued_at, expires_at, assurance }
```

The hub maps `(provider, subject)` to exactly one user transactionally, provisions a user only from that verified result, links identity once under an explicit collision policy, audits success/failure, then issues a normal `session.v1` capability.  Configure administrator authority as a set of verified `(provider, subject)` identities/claims; it is not a static bearer or loopback address.

There is deliberately **no password endpoint in this slice**.  The existing opaque `password_hash` cannot be safely verified without a versioned KDF/pepper/rate-limit/reset design.  Keep password support disabled and make production startup fail closed until a verifier adapter is present.  An external provider implementation belongs behind the port and may use deployment infrastructure, but no external runtime dependency is required in the hub core.

Public routes become only a verifier-mediated session completion/refresh surface and `/auth/session` inspection/revoke for an already authenticated session.  The exact assertion wire must be schema-first and bounded; it must never accept `{email}` or a client-selected user id.  If a browser needs provider redirects, terminate and verify that protocol at a configured trusted identity adapter/proxy, then pass a verified assertion to the port—not a forwarded unverified email header.

### 3. Explicit, non-network development bootstrap

`development-local` is an issuer, not a weakened production route:

1. Add explicit `OS_HUB_MODE=development|production`; default to `production` when the bind address is not loopback.  Production refuses startup if the identity verifier or administrator-subject policy is absent.  It also rejects non-loopback cleartext HTTP/WS.
2. In development, bind the HTTP hub to loopback only and create a separately named local bootstrap endpoint under `OS_HUB_DATA`.  Implement that endpoint through an owned `LocalBootstrapTransport`: Unix-domain socket with owner-only permissions on macOS/Linux/devcontainer, ACL-restricted named pipe on Windows.
3. A local developer command talks to that IPC endpoint and requests a named development identity.  It creates/uses a `development-local` identity and returns a session only through that IPC response.  The public router contains no development mint handler.  The endpoint must reject missing/incorrect local transport security, emit an audit event, and be unavailable in production.
4. The native identity module replaces `mint_or_restore`'s HTTP fallback with this local bootstrap client only when the declared hub mode is development.  Browser/remote clients receive “sign-in required”; they do not synthesize identity from `S_USER`/email.

This preserves one-command local development while making the trust boundary visible and unrouteable from LAN/browser HTTP.  It is a deliberate hard failure on platforms where local transport protection cannot be established.

### 4. Server-owned WebSocket identity and revocation

Make authentication the first bounded control frame.  It carries a `session.v1` capability in the encrypted WS message body only; it never appears in the URL.  The server verifies capability, scope membership, expiry/revocation and generation before descriptor/replay data or presence is sent.  It derives:

```
actor = "user:<user-id>#<auth-session-public-id>:<validated-device-instance>"
```

The server sends this actor in `ServerFrame::Session`; remove `actor` from `ClientFrame::Hello`, or reject it as an unknown field after the schema cut.  Every command envelope supplied by the client is normalized to that actor or rejected if it claims any other actor.  Presence, color, `Principal`, replay guard, preview, audit and `SyncSessionRecord` all consume the same server-derived actor.

The server indexes live sockets by `auth_session_id`.  `revoke_*` changes session state/generation transactionally, appends the audit event, sends close/re-auth notices to every mapped socket, and the frame admission gate checks the captured generation before each authority-bearing frame.  The periodic tick may remain as a bounded failed-notification recovery, but not as the correctness mechanism.  A failed `record_sync_session_open` must not make a live authenticated socket invisible: retain an in-process id mapping and retry/close according to a bounded policy.

Directory WS uses the same first authenticated control message, then accepts `since`; replace both current `?token=` URL builders.  HTTP continues to accept `Authorization: Bearer session.v1…` only on configured TLS/loopback development listeners and must scrub it from errors/logging.

### 5. Share, invite and admin separation

Shares remain anonymous, document-scoped, read-only capabilities.  They cannot create a directory session, call directory/admin APIs, read space blobs, invoke an invite, or write a document.  Their first WS control frame causes a server-derived anonymous viewer actor and spectator policy.  Invite acceptance requires an already authenticated `SessionCapability`; it never upgrades a share bearer into membership.  Hash and revoke both as above.

Admin routes use the same verified user session with a global administration claim established by the verifier/bootstrap policy.  Delete `OS_HUB_ADMIN_TOKEN` and loopback-is-admin.  Admin share creation/revocation, connection kick, user-session revocation, member removal and destructive directory commands each append a protected audit record with target and correlation id.  Admin audit visibility is a separate authorization policy; ordinary directory event streams must not reveal cross-space identities, login outcomes, raw provider subjects, or session activity.

## Ordered implementation packet

1. **Freeze the unsafe boundary first.** In `🌎️hub/📦️packages/🦀️rust/📦️bin.rs`, remove public `CreateAuthSessionRequest/Response`, `create_auth_session`, `/auth/sessions` POST, raw `is_admin` fallback and every test helper that calls them.  Make startup reject unsafe production configuration.  Update `.vscode/launch.json:4342-4359` so `🛠️dev🗄️os-hub` explicitly sets `OS_HUB_MODE=development`, loopback bind, and an isolated dev data directory; add separate clearly named production-like launch that requires verifier/admin configuration and does not enable bootstrap.
2. **Make credentials durable and backend-consistent.** Define capability parser, secret issuance/digest/constant-time equality, session/share/invite/audit model and `HubDirectory` ports in `🌎️hub/📇️directory/🦀️.rs`.  Replace the DDL/projections and implementation in `🌎️hub/📇️directory/🪶️sqlite/🦀️.rs`, `…/🐘️postgres/🦀️.rs`, and `…/🌐️neo4j/🦀️.rs` together.  Because this is greenfield, rebuild projection schemas; do not write compatibility migrations or retain plaintext columns.
3. **Install the identity boundary.** Add `IdentityAssertionVerifier`, verified identity link/provision transaction, fail-closed production configuration, auth audit writer, and protected local bootstrap transport.  Do not add a password verifier in this packet.  Replace native `mint_or_restore` at `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🪪️identity/🦀️.rs:145-205`; replace plaintext durable token caching with an owned OS-secret-storage port or fail closed for persistence.
4. **Replace all raw client APIs.** In Rust `DirectoryClient` (`…/📇️directory/🔌️client/🦀️.rs:310-342,364-468`) and TypeScript `DirectoryClient` (`🧰️framework/🛍️products/💻️os/🟦️.ts:3949-4068`), delete `mintSession`, add authenticated-session inspection/revoke and first-message WS authentication.  Update config identity/schema callers, never exposing token in URLs.  This must land with the schema/wire fixture, not as a client-only shim.
5. **Bind document and directory sockets.** Change the replication wire schema and all `ClientFrame::Hello` producers/consumers.  In `handle_ws` / `handle_client_frame` (`🌎️hub/📦️packages/🦀️rust/📦️bin.rs:795-1064`), derive actor/principal/envelope identity, persist auth-session linkage, index live sockets and atomically enforce revocation.  Change `/directory/ws` at `:1437-1533` to authenticate in-frame rather than query string.
6. **Make admin and sharing use the new authority.** Rewire `create_share`, `revoke_share`, every `/admin/api/*` gate, user-session revoke and connection kick (`🌎️hub/📦️packages/🦀️rust/📦️bin.rs:514-542,1657-1795`) to verified admin sessions and audit records.  Scope share/blob authorization separately; do not reuse `resolve_auth(space, hash, token)`.
7. **Replace unsafe E2E setup, then gate release.** Rewrite `🌎️hub/📦️packages/🟦️typescript/🧪️index.test.ts:425-586` to obtain two different local-bootstrap fixtures or verifier-issued test assertions.  It must never use email-as-proof or a static admin bearer.  Make active revocation, actor derivation, token non-disclosure, and cross-space isolation hard acceptance gates before P2-C/loader work is considered a remotely usable hub.

The work is parallel-safe with loader/P2-C: steps 1–4 define identity and storage contracts independent of artifact loading.  Step 5 can take the existing WS/replay seam, while P2-C continues to own descriptor/checkpoint/rebootstrap mechanics.  The integration point is simple: successful authentication is required before any descriptor bytes, checkpoint frontier, catalog payload, or rebootstrap response is disclosed.

## Required evidence and focused verification

Add language-neutral fixtures for token grammar, domain-separated digest input, invalid/expired/revoked capability outcomes, trusted identity assertions, actor derivation, admin claims and redacted audit records.  Validate the fixture against the owned Rust schema and the existing third-party Ajv validator used by hub TypeScript tests; independently compute SHA-256 vectors with Node `crypto` so the digest implementation is not its own oracle.  Fixtures contain synthetic secrets only and are never copied into logs.

Focused tests to add/run after implementation (do not run the broad current Cargo target while concurrent jobs are active):

| Check | Focused command / oracle | Pass condition |
| --- | --- | --- |
| Capability model and all directory backends | Add a filterable Rust auth-contract test target to `🌎️hub/📦️packages/🦀️rust/📜️script.ts`, then `bun nx run os-hub:test quick -- auth_session` | SQLite, Postgres, Neo4j projections agree: raw secret is absent from storage/read model; expiry/revoke/user-wide revoke are deterministic.  Run Postgres/Neo4j only when their Docker/backend prerequisites are explicitly healthy. |
| Schema/digest oracle | `bun nx run os-hub-ts:test quick -- --runInBand auth-session-contract` | Owned schema accepts exactly the vectors that Ajv accepts; Node `crypto` produces each fixture digest; logs/responses omit raw bearer. |
| Client wire and URL hygiene | `bun nx run os-shell:test quick -- directory-auth` (or the dedicated existing package target once added) | Rust and TS WebSocket dials have no `token=` query; first control frame is bounded and cancellation-aware. |
| Actor-spoof regression | `bun nx run os-hub:test quick -- websocket_actor_is_server_derived` | Two authenticated users cannot select each other's actor; spoofed envelope is rejected; presence/audit/commands retain server actor. |
| Revocation/kick | `bun nx run os-hub:test quick -- revoke_user_sessions_closes_and_blocks_reconnect` | Admin revocation atomically invalidates every bearer, closes all mapped sockets, and rejects reconnect before descriptor/replay output. |
| Real two-client socket | Start through the updated `🛠️dev🗄️os-hub` launch profile, provision two identities only through local IPC, then `HUB_E2E=1 bun nx run os-hub-ts:test -- --testNamePattern='authenticated session'` | Two users can collaborate only under distinct server actors; static/raw email mint and loopback admin attempts return no session/no authority; revoke closes the selected socket and blocks the old bearer. |

The last command necessarily builds/starts the hub and needs the local SQLite backend plus its generated trusted-catalog/runtime prerequisites.  It was not run in this audit.  Do not use the current `bun nx run os-hub:test` as a focused security signal: its script unconditionally invokes Cargo with `--all-features` (`🌎️hub/📦️packages/🦀️rust/📜️script.ts:27-35`), and the current TS E2E both builds the binary and encodes unsafe minting (`🌎️hub/📦️packages/🟦️typescript/📜️script.ts:13-29`).

## Exit criteria

The defect is closed only when a source and real-socket run establish all of the following:

- no network endpoint accepts email/user id as authentication proof; production has a configured verifier or refuses to listen;
- bearer, share and invite secrets have 256-bit no-fallback entropy, typed parsing, digest-only durable storage, redacted logs and distinct scopes;
- no WS URL contains a credential, and no user-controlled actor reaches `Principal`, presence, replay, command attribution, or audit identity;
- session expiry, self-revoke, user-wide revoke, role/membership removal and admin kick are atomic enough to block the next authority-bearing frame and prevent reconnect;
- admin capability is verified identity policy, never loopback proximity or a static raw bearer;
- audit trails are append-only, protected and privacy-minimized; directory visibility remains cross-space isolated; and
- two independently provisioned real clients pass the socket proof across restart without reminting from an email address.

Until then the sharpest blocker is the public arbitrary-email `/auth/sessions` route combined with client-selected WebSocket actor: a remote caller can both become an existing user and attribute actions to another actor.
