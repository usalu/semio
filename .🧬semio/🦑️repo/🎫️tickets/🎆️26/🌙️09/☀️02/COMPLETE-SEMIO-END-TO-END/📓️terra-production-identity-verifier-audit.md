# Production Identity Assertion Verifier Audit

Status: read-only source audit, 2026-09-03. No build, test, server, identity-provider, browser, or backend was run. This is a concurrent-tree snapshot; line anchors identify the source inspected.

## Decision and first blocker

**Recommended production adapter: a first-party, provider-neutral `OidcAuthorizationCodeIdentityVerifierV1`, using OIDC Authorization Code + PKCE, with the hub owning the transaction and token exchange.** A browser selects only a configured `providerId`; it never submits an email, user id, issuer, audience, key URL, admin claim, or pre-verified identity. The hub binds a one-time transaction to the configured issuer/audience/redirect URI/nonce/PKCE verifier, validates the returned ID-token signature and claims, then transactionally maps its verified `(providerId, subject)` to a user.

**First deterministic blocker — critical:** no concrete production verifier can presently be constructed. `main` hard-codes `identity_verifier` to `None` at `🌎️hub/📦️packages/🦀️rust/📦️bin.rs:2242`, and production deliberately refuses startup without one at `:604-622`. A repository-wide source census found no OIDC, JWT/JWS, JWKS, WebAuthn, SAML, signature verifier, or identity callback implementation; the only `oidc.example` occurrences are test values (`🌎️hub/📇️directory/🦀️.rs:2204`, `🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:1263-1264`). The hub has SHA-256 and local HMAC mechanics, but no safe public-key signature, JWKS, or standards HTTP/TLS policy. Do not replace this failure with an email route, static bearer, local HMAC, forwarded header, or test double.

The existing development-only inherited bootstrap transport is the correct separate zero-touch developer path. It is not evidence that an external assertion is verified. `DevScript` starts that protected local flow at `🌎️hub/📦️packages/🦀️rust/📜️script.ts:351-378`; it must remain unavailable in production.

## Current evidence matrix

| Area | Current source seam | Finding / severity |
| --- | --- | --- |
| Production admission | `🌎️hub/📦️packages/🦀️rust/📦️bin.rs:2238-2256` parses mode, sets the verifier to `None`, opens local bootstrap only in development, and calls startup validation. | **Critical availability/security boundary.** Production fails closed as intended, but has no configuration/adapter path to become ready. |
| Mode and network scope | `📦️bin.rs:507-522,604-622` defaults a loopback bind to development; both modes require loopback, and production has no cleartext non-loopback listener. | Good fail-closed start. Production still needs a stated HTTPS reverse-proxy/public-origin contract; no arbitrary forwarded identity headers may enter the verifier. |
| Existing port | `🌎️hub/📇️directory/🦀️.rs:533-600` bounds opaque assertion bytes to 16 KiB and exposes `IdentityAssertionVerifier::verify`; `:552-594` returns provider, subject, optional profile fields, issue/expiry/assurance and supports cancellation/deadline/progress. | **High design gap.** It neither binds a server transaction nor represents issuer, audience, nonce, auth time, `kid`/key generation, algorithm, or provider configuration generation. Its only implementation is a test double at `:2159-2239`. |
| Current session issue | `🌎️hub/📇️directory/🦀️.rs` contains the typed capability/auth-session model and bounded audit vocabulary; `📦️bin.rs:1272` onwards resolves session authority. | The secure-capability foundation is usable only *after* a verified identity. A verifier must issue an external session whose expiry cannot outlive the assertion. |
| User identity projection | `UserRecord` has one optional, raw `sso_provider`/`sso_subject` pair at `🌎️hub/📇️directory/🦀️.rs:68-76`; `HubDirectory` exposes separately callable create and lookup methods at `:1484-1492`. | **Critical correctness/privacy gap.** This cannot represent multiple linked external identities or atomically claim one. It must not map by email. |
| SQLite identity uniqueness | `🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:35-62` makes `email` unique but places no uniqueness constraint on the SSO pair; `:523-555` creates and queries in separate calls. | **High.** Concurrent first sign-ins can create duplicate subject mappings. |
| PostgreSQL / Neo4j parity | `🌎️hub/📇️directory/🐘️postgres/🦀️.rs:590-640` and `🌎️hub/📇️directory/🌐️neo4j/🦀️.rs:484-532` repeat separate create/select operations and no unique external identity projection. | **High.** Do not call SQLite complete while backend parity has this race. |
| Administrator policy | `🌎️hub/📦️packages/🦀️rust/📦️bin.rs:585-601` parses raw `OS_HUB_ADMIN_SUBJECTS=provider:subject,…`; `:628-640` compares the verified-session provider + constant-time subject digest. | The *comparison* is sound in direction, but the environment format leaks raw subjects and cannot unambiguously encode issuer-like values with `:`. Policy must move into protected structured provider config and consume only verified identity digests. |
| Readiness | `📦️bin.rs:523-583` reports production `identity-assertion-verifier`, but readiness uses only a boolean `bootstrap_ready`; production cannot reach it because the adapter is `None`. | **Medium.** Future readiness must include a redacted verifier/config/keyset health state, not assert readiness merely because a trait object exists. |
| Available crypto / dependencies | `🌎️hub/📦️packages/🦀️rust/Cargo.toml:20-48` has Axum/Serde/Tokio and optional DB drivers; no JWT/JWS/JWKS/OIDC/WebAuthn dependency. The optional SQLx Rustls feature is a DB transport detail, not an owned identity TLS/JWS policy. | **Critical implementation prerequisite.** No current first-party public-key verifier exists. SHA-256 and local HMAC must not be represented as external identity proof. |
| Local bootstrap | `🌎️hub/🔐️local-bootstrap/🦀️.rs:17-26` defines bounded local bootstrap frames, a 15-second deadline and HMAC domain; the script has a secure smoke at `🌎️hub/📦️packages/🦀️rust/📜️script.ts:309-311,379-390`. | Correctly separate **development-local** issuer. Its inherited process channel/HMAC only authenticates a local launcher; it is not an OIDC/JWT substitute. |
| Stale client mint flows | Rust posts `/auth/sessions` at `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs:336-342`; native `mint_or_restore` falls back to email at `…/📇️directory/🪪️identity/🦀️.rs:145-205`; TS does the same at `🧰️framework/🛍️products/💻️os/🟦️.ts:4015-4033`; React reads `VITE_S_USER` and mints at `…/ShellHost/🟦️.tsx:1287-1290,1557-1561`. | **High migration drift, not a live hub route.** Current router publishes only `GET`/`DELETE /auth/sessions/me` (`📦️bin.rs:2078`), so the stale POST path should fail. Remove it rather than restoring an insecure compatibility path. |
| Admin client carrier | `🌎️hub/🔨️modules/🛡️admin/🧱️elements/🔑️AdminSession/🟦️.tsx:37-75` accepts an optional bearer and applies it to every call. | **High migration work.** Production browser admin needs verifier-mediated browser session/cookie/relay, not a pasted/stored raw bearer. |

## Authority contract

### 1. Schema-first provider configuration

Introduce an owned, versioned `IdentityProviderSetV1` read at startup through a new `ProtectedIdentityConfigReader` port. It has a fixed maximum of 32 providers and must be a regular, symlink-free, owner-only file: Unix mode/owner validation on Linux/macOS/devcontainer and an equivalent current-user/service-account ACL check on Windows. The only launch environment values are its protected file path and the fixed public origin; logs/readiness expose neither raw claims nor client credentials.

```text
IdentityProviderSetV1 {
  schema: "semio.hub.identity-provider-set/v1",
  generation: u64,
  publicOrigin: https URL,
  providers: [{
    providerId: ASCII stable alias (not a client value),
    protocol: "oidc-authorization-code-pkce",
    issuer: canonical https URL,
    authorizationEndpoint: canonical https URL,
    tokenEndpoint: canonical https URL,
    jwksUri: canonical https URL,
    clientId: bounded string,
    allowedJwsAlgorithms: [one explicit supported asymmetric algorithm],
    redirectUris: exact configured public-origin paths,
    clockSkewMs, maxTokenAgeMs, maxAssertionBytes,
    keysetMaxAgeMs, sessionMaxTtlMs,
    administratorSubjectDigests: [[32 bytes]]
  }]
}
```

`providerId` binds one canonical issuer, client/audience and key source. A provider rename or issuer change is a new identity domain; do not silently retain mappings across it. Replace `OS_HUB_ADMIN_SUBJECTS` with bounded digests in this config. Never use email, `email_verified`, display name, a provider role claim, loopback proximity, `X-Forwarded-*`, or a caller-supplied provider value as administrator authority.

### 2. Transaction-bound assertion interface

Replace the raw one-step port with an owned transaction boundary while retaining the conceptual `IdentityAssertionVerifier` name:

```text
begin(providerId, clientClass, callbackKind, control)
  -> IdentityAuthorizationStartV1 { transactionId, authorizationUrl, expiresAt }

complete(transactionId, callbackCodeAndState, control)
  -> VerifiedExternalIdentityV1 {
       providerId, subject, subjectDigest, assertionIssuedAt, assertionExpiresAt,
       assurance: external-verified, configGeneration, keyFingerprint
     }
```

The server creates a 256-bit transaction secret/state and nonce, persists only their domain-separated digests, and consumes the transaction atomically. The PKCE verifier can remain only in a bounded, owner-memory transaction store: process restart invalidates it and fails the callback closed. Do not persist it until the product has an owned encryption/key-management abstraction. The browser receives the provider URL/state; it receives neither a client secret nor a verified identity object. Token exchange and ID-token validation occur in the server-owned adapter.

For the first concrete protocol, validate all of the following before constructing `VerifiedExternalIdentityV1`:

1. configured provider alias, exact canonical issuer, exact configured audience/client ID (and `azp` when applicable), configured callback URI, and a one-time unexpired transaction;
2. TLS-validated bounded token/JWKS fetches with no credential redirects; a declared JWK selected only from the configured provider keyset and an explicit asymmetric algorithm allow-list—never `alg` chosen by the token;
3. signature, canonical bounded claims decode, nonempty bounded `sub`, `exp`, `nbf`, `iat`, maximum token age, configured clock skew and exact transaction nonce;
4. current or specifically retained previous key generation, with the old generation retained only through maximum permitted token lifetime plus skew; and
5. one-time state/code transaction consumption before session issuance, including parallel callback/replay failure.

`VerifiedExternalIdentityV1` deliberately omits raw token, code, nonce, email, display-name and claim bag. A redacted key fingerprint and configuration generation are enough for audit/diagnostics. Verified email/display name may be offered as a non-authoritative profile suggestion after authentication; it never links or merges users.

### 3. Missing safe primitives: do not invent cryptography

The recommended OIDC adapter cannot safely land until owned interfaces exist for:

- `IdentityHttpsTransportV1`: certificate validation, exact HTTPS origin policy, bounded redirect/response/header/body budgets, cancellation/deadlines and redaction;
- `JwsVerifierV1`: strict compact-JWS parser, duplicate-safe JSON/base64url handling, asymmetric signature verification, fixed algorithm allow-list, JWK parsing/`kid` selection and key fingerprinting;
- `JwksCacheV1`: per-provider cache generation, single-flight refresh, maximum byte/key count, current/previous rotation window and hard stale deadline; and
- `ProtectedIdentityConfigReader` plus `IdentityVerifierReadinessV1`.

There is no evidence that these primitives or a safe equivalent already exist. A test-only `FixtureIdentityAssertionVerifierV1` may exact-match finite fixture bytes to predeclared synthetic subjects under `cfg(test)`; it must be unconfigurable in a production binary and must have no HMAC/shared-key fallback. It proves hub transactions and link semantics, not cryptography. A future implementation may use a vetted dependency only behind these owned interfaces, but no external runtime type may cross the hub domain API.

### 4. Transactional external-identity mapping

Replace optional SSO fields on `UserRecord` with an `ExternalIdentityV1` projection:

```text
hub_external_identity {
  provider_id, subject_digest[32], user_id,
  linked_at, disabled_at?, config_generation_at_link
  UNIQUE(provider_id, subject_digest)
}
```

The directory gets one operation, `resolve_or_provision_verified_identity(VerifiedExternalIdentityV1, control) -> UserRecord`, rather than a public lookup followed by `create_user`. It uses `identity_subject_digest`'s existing domain separation (`🌎️hub/📇️directory/🦀️.rs:520-529`) only after the verifier has validated the raw subject. It creates or returns exactly one identity owner in one transaction: SQLite `BEGIN IMMEDIATE` plus a unique index; PostgreSQL `INSERT … ON CONFLICT … RETURNING`; Neo4j a schema uniqueness constraint plus transactional `MERGE`. Any collision with a different user is an auditable fail-closed conflict.

Make `UserRecord.email` optional/non-identity profile data rather than fabricating or claiming an email. There is no automatic email-based account merge. Multiple external identities can be linked only through an explicit, already authenticated user-directed linking transaction; unlink/disable revokes all external sessions for that identity and kicks them through the secure-session index. Existing user/session records are greenfield assets: rebuild the projections coherently, with no legacy fallback columns or migration bridge.

The session TTL is `min(provider.sessionMaxTtlMs, verified assertion expiry - now)`, with a small fixed maximum such as 15 minutes. A provider/JWKS outage denies new issue/refresh. A known valid cached key may be used only through its hard cache deadline; after it, sign-in returns a generic unavailable outcome. Existing hub sessions remain locally verifiable only to their short expiry or durable revocation; the outage never turns old assertions into indefinitely accepted identities.

### 5. Progress, cancellation, audit and readiness

Keep the existing `IdentityVerificationContext` control shape (`🌎️hub/📇️directory/🦀️.rs:570-594`), but standardize seven checkpoints: admission, transaction lookup, configuration, keyset, signature, claims/nonce, atomic link. Each operation has a 15-second upper deadline, checks cancellation before and after every network/store operation, bounds concurrent exchanges per provider, and maps externally detailed errors to `denied`, `expired`, `cancelled`, or `unavailable` only.

Append protected auth audit records for transaction started/consumed/rejected, keyset refresh/deny, identity provision/link/disable, and session issued/revoked. Reuse the bounded correlation/outcome/peer vocabulary from the current directory audit model; add provider ID, configuration generation and key fingerprint only. Never record raw subject, email, authorization code, token, nonce, state, PKCE verifier, redirect URL query, claims, secrets or stack diagnostics.

Replace `HubAuthenticationReadinessV1.bootstrap_ready` (`🌎️hub/📦️packages/🦀️rust/📦️bin.rs:542-543`) with a redacted `IdentityVerifierReadinessV1`: adapter kind, configuration generation, configured-provider count, each provider's keyset state/generation and last-success age, no endpoint/secret/subject. Production is ready only after protected config parses and every configured adapter validates its static policy; do not require an IdP network call at process readiness, but do refuse identity issuance if a keyset is not within its hard freshness bound.

## Client and transport consequences

- **Browser/React/admin:** redirect through `/auth/transactions` and callback under the exact configured public origin. Bind the subsequent browser session to an HttpOnly, Secure, SameSite-appropriate cookie/owned local relay; remove `VITE_S_USER`, `mintSession`, raw `AdminClient` bearer entry and sessionStorage token dependence. The current admin bearer façade is migration drift, not a safe production feature.
- **Native:** launch the system browser for the transaction. Return completion to a one-shot native process-owned channel/custom callback with a random local correlation, then transfer session material through the secure carrier work—not `S_USER`, environment token or `identity.json`. A restart/replay of a completion is rejected by the consumed hub transaction.
- **MCP:** it must not accept a token in config/URL. In development it may receive a local-bootstrap capability only through the protected inherited channel; production requires an explicit interactive/device transaction adapter after the OIDC foundation, never email minting.
- **Hub transport:** TLS must terminate at an explicitly configured HTTPS public origin; the hub remains loopback cleartext behind an owned/restricted proxy. The proxy must not assert identity by request headers. WebSocket first-frame/session migration remains a separate dependency, but it must inherit the server-issued session and generation rather than client actor/token query material.

## Threat and failure matrix

| Threat | Required invariant |
| --- | --- |
| Email/account collision or malicious email claim | `(providerId, subjectDigest)`, never email, selects identity ownership; no automatic merge. |
| Issuer/audience/provider confusion | Configured alias pins issuer, client/audience, redirect and JWKS origin; client cannot choose them. |
| `alg:none`, symmetric-key, malicious `jku`/`kid`, or duplicate JSON claim | Strict owned parser and explicit asymmetric allow-list; key material comes only from bounded configured cache. |
| Callback/state/nonce/code replay or race | 256-bit state/nonce, digest-only one-time transaction and atomic consumption; process restart invalidates memory state. |
| Rotated/revoked/stale signing key | Current/previous bounded cache generations; refresh limits; hard cache deadline; no new session after expiry/outage. |
| Provider/network short outage | Existing session stays usable only until its short local expiry/revocation; new sign-in/refresh fails closed and redacted. |
| Token/claim/subject leakage | No raw identity material in user rows, audit, readiness, URL queries, environment, browser storage or errors. |
| Administrator escalation | Protected config has only digest allow-list; it is compared against verifier-derived identity and revocation is durable. |
| Local developer transport promoted to production | Production constructs no `LocalBootstrapTransport`; only explicit loopback development script owns it. |

## Ordered bounded implementation packets

1. **P0 — make configuration and the schema explicit.** In `🌎️hub/📇️directory/🦀️.rs`, replace the under-specified verifier input/output with versioned transaction/config/readiness types and fixed budget/error codes. In `🌎️hub/📦️packages/🦀️rust/📦️bin.rs`, replace raw admin environment parsing with protected config loading and make production fail before binding if it is unavailable. This keeps the present secure failure mode while defining a real constructor seam.
2. **P1 — external-identity projection parity.** Remove singleton `sso_*` fields and add `ExternalIdentityV1` plus one atomic resolve/provision port. Implement the same schema/event/projection in `🌎️hub/📇️directory/🪶️sqlite/🦀️.rs`, `…/🐘️postgres/🦀️.rs`, and `…/🌐️neo4j/🦀️.rs`. Add identity disable → durable session revoke/kick. This is independent of loader/P2-C and must precede any real verifier.
3. **P2 — first-party safe-verifier substrate.** Implement owned HTTPS, protected configuration reader, canonical JWS/JWK parsing/signature interface and bounded JWKS cache. Add no provider implementation until algorithm verification has a neutral corpus and an independent oracle. If selecting a third-party crypto/TLS implementation, hide it behind these ports; no external runtime type enters hub APIs.
4. **P3 — OIDC authorization-code adapter.** Implement server-owned start/callback/token exchange, PKCE/nonce/state transaction store, exact claim checks, redacted audit and short external session issuance. Start with one configured provider only; provider pluralization follows the same schema, not special cases. Add `IdentityVerifierReadinessV1` to `/healthz` without secrets.
5. **P4 — carriers and stale client deletion.** Delete Rust/TS `mint_session` and `mint_or_restore` email fallback, React `VITE_S_USER` mint, admin pasted bearer flow and native plaintext token cache. Land browser/native/MCP credential carriers in parallel with their auth first-frame work, but do not reopen POST `/auth/sessions` for compatibility.
6. **P5 — launch and real proof.** Update existing `🛠️dev🗄️os-hub` (`.vscode/launch.json:4342-4359`) to state development/loopback/local-bootstrap explicitly. Add an adjacent production-like launch requiring protected provider config and public TLS proxy origin; do not set bootstrap transport values. The existing `DevScript`/`secure-local-smoke` stays development-only.

## Neutral fixtures, independent oracle, and focused commands

Create language-neutral JSON fixtures for `IdentityProviderSetV1`, transaction lifecycle, claim acceptance/rejection, identity-link collision, key rotation/staleness, audit redaction, session TTL cap and local-vs-production isolation. Fixture claims use synthetic subjects/secrets only.

Use a development-only third-party JWS/OIDC oracle (for example `jose`) to mint and independently verify a fixed EdDSA corpus, while Node `crypto` independently checks digest vectors. The owned verifier must accept/reject exactly the corpus; fixture verifier tests do not count as crypto validation. A real test provider is `FixtureIdentityAssertionVerifierV1`, compiled only under `cfg(test)`, with exact finite assertions and no configurable production constructor.

After implementation, use existing narrow targets rather than broad concurrent Cargo runs:

| Scope | Command / prerequisite | Required evidence |
| --- | --- | --- |
| Hub model + verifier filters | `bun nx run os-hub:test-quick -- identity_assertion` | Transaction expiry/cancel/replay, provider mismatch, digest-only link, admin subject digest, redacted errors. Current target runs Cargo with all features (`🌎️hub/📦️packages/🦀️rust/📜️script.ts:338-347`); do not run it during concurrent backend work. |
| TypeScript fixture oracle | Add a filterable existing hub TypeScript test case, then `bun nx run os-hub-ts:test-quick -- identity-verifier-contract` | AJV/schema and `jose` match the owned acceptance/rejection corpus. |
| Development transport regression | `bun nx run os-hub:secure-local-smoke` | The existing development issue/validate/revoke/replay/redaction check remains green; it is explicitly not an OIDC proof. |
| Production constructor | production-like `.vscode` launch after P0–P3 | It refuses missing/weak config before listen; with synthetic test adapter it reports redacted readiness only. |
| Backend parity | focused `os-hub:test-quick` identity projection filter with SQLite; then PostgreSQL/Neo4j only when their explicit Docker/services are healthy | Duplicate concurrent external identities collapse to one user and provider disable revokes sessions across all backends. |
| Real browser/native flow | two isolated users through configured test IdP, no raw token in URL/storage/logs | callback state replay, wrong issuer/audience/nonce/key, admin subject mismatch and IdP outage all fail closed. |

None of these commands was run for this audit. The current `test-quick` still uses `--all-features`, and PostgreSQL/Neo4j require their runtime backends; a passing source-only or fixture test is not a real IdP/TLS proof.

## Exit criteria

Production can be considered identity-ready only when it starts with a concrete protected, redacted verifier configuration; an independently signed corpus and a real configured provider prove signature/issuer/audience/nonce/time/key rotation behavior; each backend atomically owns `(providerId, subjectDigest)`; no email/client actor/forwarded header determines identity; external session expiry is bounded by assertion expiry; provider disable/session revoke closes future authority; and browser/native/MCP development paths never leak a credential in URL, env, log, local plaintext cache, or cross-client carrier.

Until P2 exists, the sharpest blocker remains the absence of a safe first-party HTTPS/JWKS/JWS verification substrate. The right behavior today is the current production startup refusal, not a partially trusted identity adapter.
