# Hub Secure Session Foundation

Status: implemented and focused-gated on 2026-09-03. This is the first security foundation packet from `📓️terra-hub-auth-session-security-audit.md`; the deliberately deferred socket/client and platform-adapter packet is listed precisely below.

## Outcome

- Deleted the public arbitrary-email `POST /auth/sessions` mint, its request/response types, and the raw-email test helper. The public router now exposes only inspection/revoke for an already authenticated typed session.
- Replaced the shared raw bearer shape with disjoint `SessionCapability`, `ShareCapability`, and `InviteCapability` parsers using exact `*.v1.<32-lower-hex-selector>.<64-lower-hex-secret>` grammars. There is no parser fallback between capability kinds.
- Capability issuance requests 48 bytes from the owned OS entropy boundary in one fallible call, uses 32 secret bytes, and fails instead of using `time_ordered_id`'s non-cryptographic fallback. Durable authority is a public selector plus a domain-separated SHA-256 digest. Verification compares all 32 digest bytes in constant time.
- Added checked positive TTLs capped at 31,536,000 seconds, a 128-byte device identity cap, a 16 KiB assertion cap, fixed text caps, timestamped revoke reason/state, and monotonically incremented authorization generations.
- Added owned async, object-safe `IdentityAssertionVerifier` and `LocalBootstrapTransport` ports with boxed project-owned futures plus deadline/cancellation/progress context. Test providers prove success, cancellation, deadline, and provider-error behavior.
- Replaced SQLite, PostgreSQL, and Neo4j auth/share/invite projections together. Their schemas retain selector/digest, lifecycle state, identity subject digest, session kind, device, and generation; sync sessions now retain nullable auth-session linkage, generation, and actor id. No compatibility columns or migrations remain.
- Added privacy-minimized append-only auth audit projections. Session/share/invite issuance and revocation append public ids, reason/outcome, correlation id, and peer class without raw capability, provider subject, email, or IP.
- User-wide and identity-wide durable revocation return the exact revoked session ids and incremented generations. The admin user-revoke handler performs that durable transition before separately signalling matching connection-kick handles.
- Replaced static `OS_HUB_ADMIN_TOKEN` and loopback-is-admin on the server. Admin routes require a live `session.v1` whose provider/subject digest matches one of at most 64 unique configured verified administrator subjects.
- Production validates its verifier and administrator-subject policy before database/network startup. Development validates a loopback bind and protected local-bootstrap transport before startup. The current executable composes neither adapter, so it fails closed rather than exposing a weaker route.
- Updated the `🛠️dev🗄️os-hub` launch profile to declare development mode and `127.0.0.1`; updated admin UI wording to request a verified administrator `session.v1`, never a static environment bearer.

## Schema and independent oracle

`hub/🔐️auth/🧬️schema/🔣️.json` is the neutral Draft 2020-12 schema. `hub/🔐️auth/🧪️fixtures/🧬️capability-v1/🔣️.json` fixes all three grammars, selectors, synthetic secrets, domain-separated digests, the provider/subject digest, revoke-generation transition, redacted audit payload, and caps. `hub/🔐️auth/🧬️schema/🟦️.ts` exposes separate nominal TypeScript parsers.

The Rust test consumes the same fixture and recomputes values with the owned SHA-256 implementation. The TypeScript test validates it with third-party AJV and independently recomputes all four digests with Node `crypto`; it also rejects cross-kind/uppercase values and checks the audit vector for secret/email/IP absence.

## Focused evidence

- `bun nx run os-hub-ts:test -- -t 'typed auth capabilities'` — green, 1 passed / 7 skipped. AJV schema and Node-crypto oracle both executed.
- `CARGO_TARGET_DIR=<ticket-generated>/secure-auth-target RUST_MIN_STACK=33554432 SEMIO_TEST_BUDGET_MS=120000 bun nx run os-hub:test -- typed_capabilities_match_neutral_sha256_vectors_and_fixed_boundaries` — green, 1 passed / 67 skipped. This all-feature build compiled SQLite, PostgreSQL, and Neo4j implementations.
- Same command with `auth_session_storage_is_digest_only_and_revoke_returns_generation` — green, 1 passed / 68 skipped. Real in-memory SQLite issued session/share/invite capabilities, independently inspected stored selector/digest rows, authenticated, durably revoked all user sessions, observed generation 2, rejected reuse, and inspected the audit projection for raw-secret absence.
- Same command with `identity_verifier_port_honors_progress_cancel_deadline_and_provider_error` — green, 1 passed / 68 skipped.
- Same command with `startup_auth_policy_fails_closed_without_owned_adapters` — green, 1 passed / 68 skipped.
- `bun nx run os-hub-admin:test` — no result: its entry/style graph oracles completed, then the unfiltered Vitest process exceeded the target's fundamental 15-second budget. A preceding name-filter attempt selected no test file. Neither run produced an assertion failure; no green claim is made for this UI package.
- Source hygiene: no `OS_HUB_ADMIN_TOKEN`, public `/auth/sessions` POST, plaintext auth/share/invite token column/query, or old raw lookup method remains in the Rust hub boundary. `git diff --check` is clean for owned files.

An earlier first compile was red on three Neo4j transaction cursors that omitted `txn.handle()`, one removed helper reference, and one renamed test field. Those exact faults were repaired before the green all-feature build. A later compile-only probe exceeded the fundamental 15-second assertion budget after compilation; the explicit 120-second focused runs above replaced that non-result.

## Exact residual seams

1. No production identity-provider adapter is composed. `main` deliberately passes no `IdentityAssertionVerifier`; no public assertion-completion route exists. The next packet must choose and implement a bounded trusted assertion protocol behind this port, map `(provider, subject)` to one user transactionally, and then call the digest-only issuance port.
2. No cross-platform local-bootstrap platform adapter is composed. Development therefore fails before listening. The remaining implementation is an owner-only Unix-domain socket for macOS/Linux/devcontainers, an ACL-restricted named pipe for Windows, its local developer client, and audit-backed identity provisioning. It must stay absent from the HTTP router.
3. OS credential storage and native/browser identity acquisition are not migrated. Existing Rust/TypeScript clients and the opt-in legacy E2E still expect the now-deleted email mint and cannot sign in. They must consume the verifier/local-bootstrap result and store raw session material only behind an owned OS-secret-storage port.
4. Per assignment boundary, document/directory WebSocket actor and client migration was not broadened. Directory WS still has its pre-existing query-token shape and document `Hello` still carries a client actor. The newly durable auth-session id/generation/actor fields and revoke return values are the handoff seam for the next socket packet: authenticate in the first bounded frame, derive actor server-side, remove URL credentials, index live sockets by auth-session id, and check generation before every authority-bearing frame.
5. PostgreSQL and Neo4j were compile-checked through the all-feature hub target but not runtime-tested because no configured external services were started. SQLite is the only runtime backend evidence claimed here.
6. The hub test harness still contains legacy `OS_HUB_ADMIN_TOKEN`/email-mint E2E assumptions. They are inert unless `HUB_E2E=1`, but must be replaced with verifier/local-bootstrap provisioning alongside residuals 1–3; retaining a fake compatibility route was intentionally rejected.

No claim is made that the hub is remotely usable yet. The sharp arbitrary-email mint and static/loopback administrator paths are gone, and all executable startup modes now refuse to listen until their explicit trust boundary is composed.
