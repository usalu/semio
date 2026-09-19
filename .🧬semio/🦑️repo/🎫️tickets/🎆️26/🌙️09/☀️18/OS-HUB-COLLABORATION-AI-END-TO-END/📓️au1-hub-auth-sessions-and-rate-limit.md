# AU1 — Hub authentication for real users: credential sign-in, session minting, rate limiting

Slice AU1, started 2026-09-19. Repo `/Users/ueli/Documents/semio`. Sources read before writing:
`📓️g2-hub-depth-audit.md` §4/§10/§12, `📓️audit-hub-backend.md`, `📓️worker-preamble.md`, AGENTS.md.

Status legend: ✅ landed and executed · 🚧 landed, not yet executed · ❌ not done (see §7 gaps).

---

## 1. Contract for AU2 (os-side sign-in UI) — READ THIS FIRST

This section is written before the implementation and is the binding contract. Anything the
implementation ends up deviating from is corrected here with a `CHANGED:` note and a date.

### 1.1 `POST /auth/sessions` — credential sign-in / session mint

- No `Authorization` header. `content-type: application/json`. Request body limit **1024 bytes**
  (a larger body is rejected by axum's `DefaultBodyLimit` with `413`, before any handler runs).
- Every response carries `cache-control: no-store`.

**Request** (camelCase, `additionalProperties: false` — an unknown field is a `400`):

```json
{
  "schema": "semio.hub.auth.credential-sign-in/v1",
  "email": "user@example.com",
  "password": "correct horse battery staple",
  "deviceInstanceId": "b1946ac92492d2347c6235b4d2611184",
  "clientClass": "browser"
}
```

| field | type | bounds |
|---|---|---|
| `schema` | const | exactly `semio.hub.auth.credential-sign-in/v1` |
| `email` | string | 3..=254 bytes, must contain exactly one `@`, no control bytes, lowercased by the hub before lookup |
| `password` | string | 8..=256 bytes, any UTF-8 except control bytes |
| `deviceInstanceId` | string | 1..=128 bytes, `[A-Za-z0-9._:-]` only (same charset the local-bootstrap path already uses) |
| `clientClass` | enum | `browser` \| `native` \| `cli` |

**200 response** (snake_case — deliberately: this is exactly the shape the os client's
`SessionMintResponse` already decodes, `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs:216`.
Do NOT add fields; that decoder is strict):

```json
{ "token": "session.v1.<32 lower-hex>.<64 lower-hex>", "user_id": "usr_…" }
```

`token` is a `SessionCapabilityV1` (schema `🌎️hub/🔐️auth/🧬️schema/🔣️.json`). It is shown exactly
once. AU2 sends it as `Authorization: Bearer <token>` on every later hub call, and immediately
calls `GET /auth/sessions/me` for the authority record (`expiresAt`, `displayName`, `sessionKind`,
`authorizationGeneration`) — the mint response deliberately carries no expiry so the two never
disagree.

**Error response** (camelCase), for every non-200 that reaches the handler:

```json
{ "schema": "semio.hub.auth.error/v1", "error": "invalid-credentials" }
```

| status | `error` | meaning for the UI |
|---|---|---|
| 400 | `malformed-request` | body/shape/bounds wrong — a client bug, do not retry unchanged |
| 401 | `invalid-credentials` | unknown email, user without a password credential, or wrong password — **one uniform code, no user enumeration**; the UI must not distinguish |
| 403 | `credential-sign-in-disabled` | this hub's policy has credential sign-in off (the default for a development hub, which mints through the local-bootstrap pipe instead). The UI should hide/disable the password form when it sees this. |
| 429 | `rate-limited` | plus `retry-after: <seconds>` header and `"retryAfterMs": <int>` in the body. Back off; do not resubmit before then. |
| 503 | `directory-unavailable` | hub backend fault, retry with backoff |

### 1.2 `DELETE /auth/sessions/me` — sign-out (already existed, unchanged)

`Authorization: Bearer <token>` → `204` on success, `401` if the token is unknown/expired/revoked,
`503` if the revocation gate is busy. Revocation bumps the session's `authorizationGeneration`,
invalidates every socket grant and open plan bound to it, and closes live sockets with `4401`.

### 1.3 `GET /auth/sessions/me` — session authority (already existed, unchanged)

`Authorization: Bearer <token>` → `200` `DirectorySessionAuthorityV1`, `401` when absent/expired/revoked.

### 1.4 Rate limiting (all four route families)

Refusal is always `429` + `retry-after` (integer seconds, ≥1) and, on the auth route, the
`semio.hub.auth.error/v1` body above. Other families get an empty `429`.

| family | routes | bucket key | capacity / refill |
|---|---|---|---|
| auth | `POST /auth/sessions`, `DELETE /auth/sessions/me` | remote address **and** email digest (sign-in) / session selector (sign-out) | 10 burst, 1 per 6 s |
| directory command | `POST /directory/commands` | remote address **and** session selector | 60 burst, 10 per s |
| invite redemption | `POST /directory/invites/{token}/redeem` | remote address **and** session selector | 10 burst, 1 per 6 s |
| socket grant | `POST /directory/socket-grants`, `POST /directory/spaces/{s}/documents/{d}/socket-grants`, `POST /spaces/{s}/documents/{id}/socket-grants` | remote address **and** session selector | 30 burst, 5 per s |

Both buckets must admit. A request with no bearer only consumes the remote-address bucket. For
`POST /auth/sessions` the remote-address bucket is charged by the middleware before the body is
read and the claimed-identity bucket by the handler after it is decoded, so a malformed body still
costs the address one token; whichever check refuses first reports its own `retry-after`. A refused
request charges nothing (the limiter refunds every bucket it already debited for that request).

---

## 2. Schema (schema-first) ✅

The schema was written before the Rust. `🌎️hub/🔐️auth/🧬️schema/🔣️.json` (the `hub.auth` scope's
existing authority, `$id .../hub/auth/schema.json`) gained eight `$defs`, all before the pre-existing
`SocketGrantReceiptV1`:

| `$def` | role |
|---|---|
| `CredentialHashV1` | the exact stored credential string, `^pbkdf2-sha256\$[1-9][0-9]{3,8}\$[0-9a-f]{32}\$[0-9a-f]{64}$` — the one shape `UserRecord::password_hash` ever carries |
| `CredentialSignInRequestV1` | the `POST /auth/sessions` body (closed, camelCase) |
| `SessionMintResponseV1` | the 200 body (`token` + `user_id`, snake_case, closed) |
| `AuthErrorV1` / `AuthErrorCodeV1` | the failure body and its five-code taxonomy |
| `AuthClientClassV1` | `browser` \| `native` \| `cli` |
| `AuthRateLimitPolicyV1` / `AuthRateLimitClassV1` | the burst/cost policy per rate-limited family |
| `AuthDurableEventKindV1` | `session-issued`, `session-revoked`, `credential-sign-in`, `credential-changed` — the four facts this scope appends |

TypeScript twin (`🌎️hub/🔐️auth/🧬️schema/🟦️.ts`, +50 lines): `AUTH_SESSION_MINT_ROUTE`,
`CREDENTIAL_SIGN_IN_SCHEMA`, `AUTH_ERROR_SCHEMA`, `CREDENTIAL_SIGN_IN_REQUEST_MAX_BYTES`, the four
types above, plus `credentialSignInRequestV1()` (bounds-checking builder), `parseSessionMintResponseV1()`
and `parseAuthErrorV1()`. **AU2 should import these three functions rather than hand-rolling the
wire** — the builder lowercases and bounds-checks the email exactly as the hub does.

Rust decoder authority: `🌎️hub/🔐️auth/🦀️.rs` (`semio_hub::auth`), field-for-field with the JSON
Schema, `deny_unknown_fields` on the request.

## 3. Implementation ✅

New Rust scope `semio_hub::auth`, mounted at `🌎️hub/📦️packages/🦀️rust/🦀️.rs:16-18`.

| file | what |
|---|---|
| `🌎️hub/🔐️auth/🦀️.rs` (new, 330 lines) | wire types + `CredentialSignInRequestV1::verify` (`:127`), `CredentialSignInPolicyV1` incl. `from_env` (`:251`), and the pure `decide_credential_sign_in` (`:322`) |
| `🌎️hub/🔐️auth/🔑️password/🦀️.rs` (new, 200 lines) | `PasswordCredentialV1` (mint/derive/parse/encode/verify) over an owned `hmac_sha256` (`:129`) and `pbkdf2_sha256` (`:157`) — RFC 2104 + RFC 8018 §5.2 on the framework's own `semio_framework_hash::Sha256`, **no external crypto crate** (AGENTS.md line 13) |
| `🌎️hub/🔐️auth/🚦️rate-limit/🦀️.rs` (new, 205 lines) | `HubRateLimiterV1` — millisecond-budget token buckets, integer-only, `RateLimitClockV1` injected, bounded map that sheds only fully recovered subjects and otherwise fails closed |
| `🌎️hub/🔐️auth/🧫️fixtures/🔑️pbkdf2-sha256-vectors-v1/🔣️.json` (new) | 3 HMAC-SHA256 + 4 PBKDF2-HMAC-SHA256 vectors **generated by python3 `hashlib`/`hmac` (OpenSSL)**, the third-party oracle AGENTS.md line 22 requires; includes the two published RFC 7914 §11 vectors |

Hub wiring (`🌎️hub/🏗️bootstrap/🦀️.rs`, kept deliberately small — H1/W3b are in this file):

- `HubState.credential_sign_in` + `HubState.rate_limits` (`:1594-1603`).
- `post_auth_session` (`:6591`) — the `POST /auth/sessions` handler.
- `journal_credential_sign_in` (`:6644`), `auth_error_response` (`:6658`).
- `rate_limit_middleware` (`:6679`) + `rate_limit_class` (`:6705`).
- `declare_public_session_issuance` (`:2125`) — `/readyz` now reports `publicSessionIssuance` from
  the policy instead of the constant `false`. A development hub running the local-bootstrap pipe
  still reports `false`, so `🚀️local-bootstrap/🧬️schema/🔣️.json`'s `"const": false` law holds.
- Router: one new `.route` (`:8299`) and one new `.layer` (`:8345`, inside the CORS layer so a
  `429` still carries CORS headers).
- `main`: `CredentialSignInPolicyV1::from_env()` (`:8584`), a boot failure on an unreadable value.

Password sign-in is **fail-closed**: `OS_HUB_CREDENTIAL_SIGN_IN=true` enables it,
`OS_HUB_SESSION_TTL_SECONDS` (default 43200, bounds 60..=31536000) and `OS_HUB_PASSWORD_ITERATIONS`
(default 210000 — OWASP's floor — bounds 1000..=999999999) configure it. Verification always uses
the iteration count stored *inside* the credential, so raising the default never invalidates an
existing credential.

## 4. Events in the durable log ✅

Nothing here writes a session row or a counter.

- **Session minting** is `HubDirectory::issue_auth_session`, which already appends `session-issued`
  in the same transaction as the session record.
- **Sign-out / revocation** is `revoke_auth_session` → `session-revoked` + an
  `authorization_generation` bump that invalidates socket grants and open plans and closes live
  sockets with `4401` (pre-existing, now reachable from a credential session).
- **Every sign-in attempt**, admitted or refused, appends a `credential-sign-in` fact
  (`outcome_code` `success`/`failure`, `reason_code` = the public error code, `peer_class` = the
  client class) — new trait method `HubDirectory::append_credential_audit`
  (`🌎️hub/📇️directory/🦀️.rs:3003`, sqlite at `🪶️sqlite/🦀️.rs:1812`).
- **Credential changes** go through `HubDirectory::set_password_credential`
  (`🌎️hub/📇️directory/🦀️.rs:3008`, sqlite at `🪶️sqlite/🦀️.rs:1819`), which updates the projection
  column and appends `credential-changed` **in one transaction**, so the two can never disagree.
- New model type `CredentialAuditFactV1` (`🌎️hub/📇️directory/🦀️.rs:336`). Both trait methods have
  erroring default bodies (the file's established pattern), so the postgres/neo4j backends compile
  unchanged and honestly report the capability as unavailable — see §7.

## 5. Tests and real counts

All counts below are from runs I executed; captures in `🗑️generated/au1-*.txt`.

### 5.1 New tests — all green ✅

| suite | command | result | capture |
|---|---|---|---|
| unit (`semio_hub::auth`) | `cargo test -p semio-hub --lib auth::` | **16 passed / 0 failed** (7.86 s) | `au1-auth-unit-3.txt` |
| hub integration (`os-hub` bin, real axum server + raw HTTP + real WebSocket) | `cargo test -p semio-hub --bin os-hub credential_sign_in` and the three named runs | **7 passed / 0 failed** | `au1-bin-test-1.txt`, `au1-bin-test-2.txt` |

The 16 unit laws: HMAC-SHA256 and PBKDF2-HMAC-SHA256 against the **python3 `hashlib`/OpenSSL**
vectors (third-party oracle, incl. RFC 7914 §11); credential round-trip / per-credential salt /
out-of-bounds + malformed encodings; request bounds = schema bounds; `deny_unknown_fields`; the
five statuses + exact JSON of both response shapes; disabled policy refuses before reading a
credential; **the four "wrong credential" shapes are indistinguishable**; mint at the policy
lifetime; bucket burst + exact `retry-after`; per-subject/per-class isolation; a refusal charges
nothing; the subject map stays bounded, fails closed when full and sheds only recovered subjects;
and `the_rust_decoders_and_the_json_schema_admit_exactly_the_same_wire` — every accepted and
rejected payload is run through the owned draft-07 validator compiled from `🔣️.json` itself, so
the Rust decoder can never drift from the schema.

The 7 integration laws (`🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs:6358-6563`): sign-in mints a session whose
token then authenticates `GET /auth/sessions/me` as `external` (with case-insensitive email and the
`credential-sign-in`+`session-issued` facts asserted in the durable log); every wrong credential is
one `401 invalid-credentials` and four malformed bodies are `400`, with exactly 3 journaled
failures; a hub that did not enable sign-in answers `403` and `/readyz` does not claim issuance;
**lockout** — the burst is admitted, the next attempt is `429` with `retry-after`, a *correct*
password is refused while the bucket is empty, and exactly one clock-recovered token later the same
correct password mints `200`; sign-out revokes durably (`204`, then `401` on both `GET` and a second
`DELETE`, `session-revoked` journaled); an expired session stops resolving; and an unauthenticated
document WebSocket is refused `401` at the upgrade — as is a session bearer presented in place of a
socket grant, and an anonymous `GET /spaces/{s}/documents/{id}`.

### 5.2 Regression runs — the tree is NOT green, and it was not green before me ⚠️

| run | result |
|---|---|
| `cargo test -p semio-hub --lib` (whole lib) | **141 passed / 33 failed** (`au1-lib-test-full.txt`) — **0 failures in `auth::`** |
| `cargo test -p semio-hub --bin os-hub` (whole bin suite, default parallelism) | 63 ok / 26 FAILED / ~10 still running after 11.5 min; I killed it (`au1-bin-test-full.txt`) |
| the 10 existing bin tests that actually traverse a rate-limited route, `--test-threads=1` | **10 passed / 0 failed** (`au1-rate-limited-routes-tests.txt`) |

Classification of the failures, with the evidence I have:

- **33 lib failures**: 21 `artifact_authority::trusted_catalog::*` all panicking on `"ticket-owned
  artifact root"` / `"ticket-owned catalog fixture root"`, 8 `inference::*`, 4 `directory::*` (one
  of them `"ticket artifacts: NotPresent"`). These are a peer's in-flight trusted-catalog rework
  (`🛡️opened-root/🦀️.rs` and its tests are `MM` in `git status`) demanding a ticket-owned data root
  that is not present. I touched none of these modules (my `📇️directory` diff is **+49/-0** and
  `🪶️sqlite` **+21/-0**, purely additive).
- **26 bin failures**: I sampled four. `trusted_catalog_startup_…` → the same missing fixture root.
  `space_public_boundary_…` → an anonymous `GET /directory/spaces/{id}` returning `500` (a `GET` is
  in no rate-limit class and passes my middleware untouched). `socket_grant_revoke_before_command_
  admission_…` → `"no revocation close before 5s deadline"` under load average **27.6** on a
  10-core machine (whole fleet running). `directory_invite_redemption_admitted_fence_precedes_
  archive` → **passes single-threaded**.
- **The decisive check for my slice**: the 10 existing tests that POST to `/directory/commands`,
  `/directory/invites/{token}/redeem` or `…/socket-grants` through the real router all pass, and 4
  of them were in the 26. No failing assertion anywhere mentions `429` or auth.
- **Honest limit**: I could not produce a before/after baseline — modifying git is forbidden for
  this ticket, and H1 only *compiled* the bin tests (`cargo test -p semio-hub --no-run --bins`,
  `📓️h1-hub-build-and-boot.md` line 130); nobody has run this suite to green in this tree.

### 5.3 TypeScript

`bun 📜️script.ts typecheck` in `🌎️hub/📦️packages/🟦️typescript`: **170 pre-existing errors, 0 of
them in `🔐️auth`** (`au1-ts-typecheck.txt`). That backlog is T2's slice.

## 6. Files changed

New:
- `🌎️hub/🔐️auth/🦀️.rs`
- `🌎️hub/🔐️auth/🔑️password/🦀️.rs`
- `🌎️hub/🔐️auth/🚦️rate-limit/🦀️.rs`
- `🌎️hub/🔐️auth/🧪️tests/🔬️unit/🦀️.rs`
- `🌎️hub/🔐️auth/🧫️fixtures/🔑️pbkdf2-sha256-vectors-v1/🔣️.json`

Modified:
- `🌎️hub/🔐️auth/🧬️schema/🔣️.json` (+8 `$defs`)
- `🌎️hub/🔐️auth/🧬️schema/🟦️.ts` (+50 lines: the TS twin AU2 imports)
- `🌎️hub/📦️packages/🦀️rust/🦀️.rs` (mounts `pub mod auth`)
- `🌎️hub/🏗️bootstrap/🦀️.rs` (+2 `HubState` fields, 1 route, 1 layer, 5 functions, policy at boot)
- `🌎️hub/📇️directory/🦀️.rs` (+`CredentialAuditFactV1`, +2 trait methods with erroring defaults, +2 enum forwards — additive only)
- `🌎️hub/📇️directory/🪶️sqlite/🦀️.rs` (+`append_credential_audit`, +`set_password_credential` — additive only)
- `🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs` (+7 integration laws, +the two new `HubState` fields at both test-state sites)

No `📜️script.ts` / `📋️project.json` / `.vscode/launch.json` row was added: this slice introduces no
new runnable command — the new tests run under the hub crate's existing cargo test targets.

## 7. Honest gaps

1. **The browser-broker-proof issuer is still not implemented on the hub, and I now believe it does
   not belong there.** G2 §4 read the `🔐️browser-broker-proof-lifecycle-v1` schema as a missing
   hub-side HTTP issuer. Reading the actual client (`🏪️store/👷️worker/🟦️.ts:642-812`) the proof is a
   **hash-chain ratchet between the os worker and the local relay** (`x-semio-browser-broker` +
   `x-semio-browser-broker-next` = `sha256("semio/browser-broker-proof/v1\0" ‖ next)`, answered by
   `x-semio-browser-broker-advanced: 1`), over `/_semio/hub/*` — the path space
   `🌎️hub/🚀️local-relay/🧭️routing/🟦️.ts` admits. It is a *relay* concern (the relay holds the bearer),
   not a hub route. **Nothing in this slice implements it**, and I deliberately did not add a hub
   route for it. Whoever owns the relay should implement `localRelayUpstreamPath`'s consumer with the
   ratchet and the `120000`/`15000` ms bootstrap/active bounds the fixture schema pins; the relay must
   also add `/auth/sessions` to its admitted path set before AU2's UI can sign in through it.
2. **Postgres and Neo4j cannot journal credential facts or store credentials.** Both new trait
   methods use the file's established erroring-default pattern, so those backends compile unchanged
   and fail honestly (`"credential audit is unavailable for this backend"`). A hub with
   `OS_HUB_DIRECTORY_BACKEND=postgres` and `OS_HUB_CREDENTIAL_SIGN_IN=true` would refuse every
   sign-in with `503`. Implementing them is a small, mechanical follow-up in
   `🐘️postgres/🦀️.rs` / `🌐️neo4j/🦀️.rs`, and neither backend has ever been compiled (G2 §2) — I did
   not want to add unverifiable code to two crates I cannot build.
3. **No route yet sets a password.** `set_password_credential` exists, is transactional and journals
   `credential-changed`, but no HTTP route or admin intent calls it — a credential is created today
   by passing the encoded hash to `create_user`. A `POST /auth/credentials` (self-service change,
   old password required) and an admin intent belong in a follow-up; both should revoke the user's
   other sessions (`revoke_auth_sessions_for_user`) on success.
4. **`set_password_credential` and the `credential-changed` fact are not covered by a test.** Only
   the sign-in path is. The method is 12 lines and transactional, but I did not execute it.
5. **The TypeScript twin has no test.** `credentialSignInRequestV1`/`parseSessionMintResponseV1`/
   `parseAuthErrorV1` are held against the schema by review only — the hub's vitest config
   (`🌎️hub/🧪️tests/🎚️config/🟦️.ts`) includes exactly one file, the `HUB_E2E=1`-gated integration
   suite, so adding a unit file means editing that shared config. AU2 exercising them in the os
   suite would close this.
6. **`hub.auth` is not registered in the schema-export registry.** The `💡️inference` and
   `🗿️artifact-authority` scopes each have a `🧬️schema/🦀️.rs` calling
   `register_scope_schema_exports` with an `EXPORTS` array and a standalone export-law test;
   `hub.auth` never had one (pre-existing) and I did not add one. The `x-semio-formats` annotations
   on the new `$defs` are written as that registration will need them.
7. **The hub bin suite is not green** — see §5.2. My slice's own laws pass, and I showed the
   rate-limited routes still pass, but I cannot claim the suite was green before or after.
8. **Rate-limit policy numbers are a judgement call, not a measurement.** 10 sign-ins per minute per
   identity and per address, 10 req/s directory commands, 5/s socket grants. They are one table
   (`RateLimitClassV1::policy`) and one edit away from change, and the schema
   (`AuthRateLimitPolicyV1`) describes them, but no deployment has been load-tested against them.
9. **The limiter is per process and per `HubState`.** A multi-process hub deployment would give each
   process its own buckets; a shared limiter would need the directory (or another shared store)
   behind the same `RateLimitClockV1` seam.
