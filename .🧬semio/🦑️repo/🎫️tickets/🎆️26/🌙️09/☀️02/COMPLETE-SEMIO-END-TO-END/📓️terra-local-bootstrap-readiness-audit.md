# Local Bootstrap Transport And Hub Readiness Audit

Date: 2026-09-03  
Scope: zero-touch protected development bootstrap after the secure-session foundation. Read-only source audit; no build, test, endpoint, or source change was made.

## Decision

Implement one first-party **anonymous inherited-pipe `LocalBootstrapPipeV1`** owned by an explicit local profile launcher, plus a token-free per-browser-profile local relay. Do not add a loopback HTTP session-mint endpoint, a Unix-socket/Windows-named-pipe public fallback, a credential environment variable, command-line token, or a plaintext token file.

The parent launcher creates the pipes before it starts direct executable children. Possession of the child endpoint is the local peer-authentication boundary. The hub gets a private control endpoint, accepts bounded authenticated profile assertions, and issues a digest-only `DevelopmentLocal` session through the existing directory primitive. The parent retains the only plaintext capability in memory and hands it to native/MCP children over distinct inherited pipes. A browser never receives a hub bearer: its local relay holds that bearer in memory and proxies authenticated REST/WebSocket traffic after a one-use opaque browser boot nonce becomes an `HttpOnly; SameSite=Strict` relay cookie.

This is intentionally a development-local identity issuer, not a replacement for production external identity verification. Production continues to require an `IdentityAssertionVerifier`, explicit admin subject allowlist, and loopback-only cleartext HTTP.

## Current Evidence And Gaps

| Concern | Current evidence | Result |
|---|---|---|
| Fail-closed intent | [`🌎️hub/📦️packages/🦀️rust/📦️bin.rs:496`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📦️bin.rs:496) rejects development without `LocalBootstrapTransport`, and production without a verifier/admin subjects/loopback bind. | Correct policy shape. |
| No actual adapter | [`bin.rs:2101`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📦️bin.rs:2101) sets both adapter variables to `None`; only [`bin.rs:2177`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📦️bin.rs:2177) provides a test double. | Every current development hub launch deterministically fails before listen. |
| Adapter contract too weak | [`🌎️hub/📇️directory/🦀️.rs:602`](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:602) exposes an unstructured name/device/correlation request and one `respond(capability)` call. | It has no profile allowlist, peer proof, run binding, replay state, request ID, error frame, cancellation ownership, or safe multi-client exchange. Replace it rather than try to conceal these needs in strings. |
| Durable session seam is reusable | [`directory.rs:156`](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:156) persists only a capability digest; [`directory.rs:619`](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:619) bounds/authenticates an `AuthSessionIssue`; `DevelopmentLocal` is a first-class kind at [`directory.rs:131`](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:131). | Reuse `issue_auth_session`/`prepare_auth_session`; never persist or log the raw capability. |
| Existing bounded control | [`directory.rs:572`](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️directory/🦀️.rs:572) (actual file [`🌎️hub/📇️directory/🦀️.rs:572`](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:572)) supplies deadline/cancellation/progress checkpoints; assertion maximum is 16 KiB at [`directory.rs:333`](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:333). | Reuse, with checkpoints around every frame and issuance stage. |
| Hub accepts a typed bearer | [`bin.rs:1165`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📦️bin.rs:1165) parses `SessionCapability` then authenticates it; only authenticated `GET`/`DELETE /auth/sessions/me` exist at [`bin.rs:1619`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📦️bin.rs:1619). | There is no legitimate public session issuance route. Keep it that way. |
| Client contract drift | [`🧰️framework/🛍️products/💻️os/🟦️.ts:3981`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️.ts:3981) keeps a mutable raw bearer and calls `POST /auth/sessions` by email at [`:4015`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️.ts:4015). | This is stale test/client expectation, not a server route to restore. Remove/replace it. |
| Plaintext native persistence | [`🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🪪️identity/🦀️.rs:75`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🪪️identity/🦀️.rs:75) derives identity from `S_*`; [`:95`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🪪️identity/🦀️.rs:95) reads/writes `identity.json`, and [`:157`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🪪️identity/🦀️.rs:157) mints/restores the raw token. | Production defect: this violates protected local bootstrap and must be replaced with opaque local credential delivery. |
| Browser persistence | [`🧰️framework/🛍️products/💻️os/🟦️backbone-worker.ts:278`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️backbone-worker.ts:278) persists identity beneath the data directory. | The bearer must not be part of this persisted identity payload. |
| Existing cross-platform pipe precedent | [`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🚚️process-transport/🦀️.rs:65`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🚚️process-transport/🦀️.rs:65) has Windows `PeekNamedPipe` support and [`:212`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🚚️process-transport/🦀️.rs:212) starts a direct child with piped stdio. | Reuse the process-launch/handle-inheritance discipline; no runtime library is required. |
| Launch is incompatible today | The hub launch profile uses loopback and `OS_HUB_MODE=development` but no protected adapter at [`.vscode/launch.json:4342`](/Users/ueli/Documents/semio/.vscode/launch.json:4342). The TypeScript hub runner inherits ambient environment at [`🌎️hub/📦️packages/🦀️rust/📜️script.ts:38`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:38). | `cargo`/Bun wrapper processes cannot be the assertion channel owner: a direct hub executable must be the pipe child. |
| Runtime directory | `main` merely calls `create_dir_all` for extension data at [`bin.rs:2123`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📦️bin.rs:2123); launches use workspace-visible `.🧬semio` paths. | No private owned runtime-dir abstraction or permission/symlink validation exists. Do not store a bootstrap secret there. |

The source allows `AuthSessionKind::DevelopmentLocal`; it does **not** yet implement a development issuer. The distinction matters: accepting arbitrary `S_USER`/email and then minting a bearer would reintroduce the defect the secure-session foundation deliberately removed.

## Platform Contract: LocalBootstrapPipeV1

### Ownership And Transport

`LocalProfileLauncherV1` is the sole owner of a launch run. It creates a fresh 128-bit `runId`, a fresh 256-bit `channelKey`, and **one anonymous bidirectional control pipe for the direct hub child**. The launcher closes the child end after spawning; the hub closes the parent end immediately after inheriting it. The handle is not a secret conveyed in an environment value—only the operating system's inherited descriptor/handle crosses the process boundary. Its narrow, non-secret bootstrap selector may be passed as launch configuration solely to choose the already-inherited endpoint.

* macOS, Linux, and devcontainers: use an anonymous inherited pipe/socket pair; close all unintended descriptors before `exec`, and do not run through a shell, `cargo run`, or a task-wrapper child which can retain the endpoint.
* Windows: use an inheritable anonymous pipe handle allowlist for precisely the hub child. The existing `PeekNamedPipe` implementation is evidence that this repository already supports the relevant native pipe behavior; do not substitute a globally named pipe with a weak default ACL.
* The pipe has no filesystem name, port, or discovery file. This removes UDS path-length, Windows named-pipe ACL, `TMPDIR`, and devcontainer mount differences from the authentication boundary.
* The launcher starts a direct already-built hub executable; the executable emits normal diagnostics only on stderr. stdin/stdout remain unavailable to the hub for bootstrap protocol. This is essential because MCP owns stdio and wrappers otherwise leak/retain handles.
* If non-secret run evidence is retained, create a unique per-run directory under an OS private runtime root, reject symlinks, require owner/current-user access and POSIX `0700`; fail closed if that cannot be achieved. On Windows the anonymous-pipe design does not rely on a filesystem ACL. Never use a workspace `.🧬semio` directory for secrets.

This establishes same-user/process-tree authority only. It is not a defense against a malicious process already running as the same OS principal, a compromised launcher, kernel debugger, or an untrusted browser extension. Production external identity remains required for those threat models.

### Schema-First Frames And Bounds

Add a language-neutral JSON Schema `semio.hub.local-bootstrap/v1` before implementations, with canonical UTF-8 JSON encoding and a four-byte big-endian length prefix. Cap a complete frame at **16 KiB**, outstanding requests at **8**, profiles per run at **8**, client issuance TTL at **15 minutes**, and an exchange deadline at **15 seconds**. Any oversize/truncated/unknown-version frame closes the pipe without echoing payload bytes.

All authenticated frames carry `schema`, `runId`, `sequence`, `exchangeId` (16 random bytes encoded canonically), `issuedAt`, `expiresAt`, and a `proof` generated as:

```
HMAC-SHA-256(channelKey,
  "semio/hub/local-bootstrap/v1\\0" || canonical-length-prefixed-frame-without-proof)
```

`hello` performs mutual run/nonces proof before any profile is accepted. `issue` carries only an allowlisted `profileId`, `deviceInstanceId` (existing 128-byte bound), requested client class (`native`, `mcp`, `react-relay`, or `admin-relay`), and the one-use exchange ID. A profile maps in the launcher's immutable configuration to provider `semio.local.bootstrap/v1`, opaque fixed subject, display metadata, and allowed client classes. An asserted e-mail, `isAdmin` bit, hub URL, role, plugin, document, or space is not part of the protocol.

The hub verifies the run, HMAC, sequence, time window, profile/class mapping, and a fixed-size consumed-exchange cache before it provisions/locates the verified user and issues `AuthSessionIssue { session_kind: DevelopmentLocal, identity_provider, identity_subject_digest, device_instance_id, correlation_id, peer_class }`. Administrator power is still derived by matching the verified provider/subject against `OS_HUB_ADMIN_SUBJECTS`; a local profile cannot ask for it. The response includes only the exchange ID, expiration/session metadata, and one raw `SessionCapability`, protected by the pipe and returned exactly once. It is never placed in `Debug`, error bodies, events, readiness, child environment, command line, or a file.

The existing two-method `LocalBootstrapTransport` should be replaced by a service-shaped transport capable of `accept`, `issue`, `reject`, `cancel`, and `shutdown`, with explicit request/response IDs. A request maps to one response or one redacted terminal error. The service calls `IdentityVerificationContext::checkpoint` before decode, profile verification, durable issuance, and write; cancellation, a 15-second deadline, peer EOF, child exit, HMAC failure, expiry, duplicate exchange ID, or sequence regression consumes/invalidates the exchange and closes or rejects as appropriate. On hub shutdown it cancels pending issuance, closes the endpoint, and drops the in-memory key/replay cache. Pipe reconnect is never accepted for the same run; restart creates new run/key/credentials and clients must revalidate `/auth/sessions/me`.

### Credential Delivery To Each Client

| Consumer | Credential path | Forbidden path |
|---|---|---|
| Native process | Launcher creates a separate anonymous one-shot credential pipe for that direct client. `LocalCredentialEnvelopeV1` contains the matching run/profile/class/exchange proof and raw capability once; the client immediately validates `/auth/sessions/me`, holds it in memory, and closes the pipe. | `S_USER` authority, `S_*TOKEN`, `identity.json`, command line, shared data directory. |
| MCP process | Same envelope but on a distinct inherited control descriptor/handle—the MCP stdin/stdout pair remains exclusively MCP JSON-RPC. The repository needs a small owned cross-platform extra-handle adapter; it cannot claim this is already supplied by the existing stdio host transport. | Sending any credential on MCP stdio, environment, an MCP resource, tool response, or log. |
| React browser | A launcher-owned, per-profile loopback **relay** receives the bearer from its credential pipe, keeps it only in RAM, and proxies REST/WS to the loopback hub with the Authorization header. Browser launch has only a one-use 256-bit opaque boot nonce in the URL fragment; its tiny relay-origin bootstrap page POSTs the nonce, immediately removes the fragment, and receives an `HttpOnly; Secure where TLS; SameSite=Strict; Path=/` relay cookie. The relay never exposes the bearer, `/auth/sessions`, a generic token endpoint, or cross-profile cookies. | Bearer in Vite defines, JavaScript, DOM, local/session storage, URL/query/referrer, WebSocket subprotocol, browser cache, or devtools log. |
| Admin browser | Separate `admin-relay` profile and cookie jar; no ambient admin token. The hub decides administrator privileges from session subject. | `OS_HUB_ADMIN_TOKEN`, shared user relay, client-side role selection. |

The relay binds loopback in the same network namespace and verifies Host/origin, a fixed allowlisted origin, nonce single-use/expiry (60 seconds), client class, and request deadline. It strips inbound `Authorization`, never reflects upstream headers containing a bearer, redacts errors, bounds request/WS setup, and stops on its child/hub parent exit. `Secure` is used when the relay can offer TLS; a loopback cleartext development exception must be explicit and never cross a forwarded/container network boundary. If the browser is outside the devcontainer namespace, no implicit `0.0.0.0` bridge is permitted: use an explicit local desktop relay in that namespace or fail the launch with an actionable profile error.

`DirectoryClient` becomes an injected transport/opaque-session client, not an email minting client. Native identity becomes a `CredentialHandle` plus nonsecret profile metadata. A durable OS keychain abstraction may be added later behind a first-party interface, but it is not required for zero-touch development: restart reissues a short-lived capability through a new private run rather than caching it.

## HubReadinessV1

Add public, redacted `GET /readyz` after initialization. It returns `200` only when the listener, directory database, storage/artifact authority, static admin asset policy, and required authentication adapter are operational; otherwise `503` with a bounded redacted status. It never starts an issuer and never contains profile/subject/identity, admin configuration, file path, capability, endpoint handle, error trace, or database credential.

```json
{
  "schema": "semio.hub.readiness/v1",
  "status": "ready",
  "runId": "opaque-run-id",
  "mode": "development",
  "bindScope": "loopback",
  "authentication": {
    "kind": "local-bootstrap-pipe-v1",
    "bootstrapReady": true,
    "publicSessionIssuance": false
  },
  "directory": { "ready": true },
  "artifactAuthority": { "ready": true },
  "features": { "openPlan": false, "rebootstrap": true, "mcpWorkspace": false, "inference": false }
}
```

`features` is a truthful declared capability set, not a health guess. It prevents a release gate from treating an HTTP listener as proof that unfinished open-plan, catalog/CAS, MCP, or GIS work is usable. The exact body/fields, enum values, and error shape belong in `semio.hub.readiness/v1` schema and neutral fixtures before Rust/TypeScript implementations. Launcher readiness is bounded (suggested 30 seconds, cancellable); it must reject a ready response whose `runId`, `mode`, bind scope, or auth mode differs from its profile.

## Threat And Failure Matrix

| Threat/failure | Required prevention/detection | Severity now |
|---|---|---|
| Arbitrary email becomes bearer | No public mint; fixed launcher profile → verified provider/subject → directory issuance. | High |
| Remote process calls bootstrap | No listening bootstrap endpoint; inherited endpoint only; hub loopback plus relay nonce/origin checks. | High |
| Replay/stale launch frame | Per-run key/nonces, expiry, monotonic sequence, consumed-exchange cache, one response. | High |
| Token escapes client | Memory-only parent/relay, private pipes, redaction tests, no environment/URL/file/log. | High |
| MCP credential confused with protocol | Separate inherited descriptor/handle; MCP stdio remains protocol-only. | High |
| Browser gets bearer | Relay has Authorization upstream; browser has only HttpOnly relay cookie and opaque nonce. | High |
| Wrapper leaks handles or secrets | Direct executable child, explicit inheritance whitelist, close-extra descriptors, no ambient inherited environment authority. | High |
| Hang/abandoned launch | 15-second exchange and 30-second readiness deadlines, checkpoints, EOF/exit cancellation and deterministic cleanup. | Medium |
| Permission/symlink attack on run data | No secret files; unique validated private nonsecret evidence directory or fail closed. | Medium |
| Devcontainer accidentally exposes relay/hub | Same-namespace loopback only; explicit bridge profile or launch failure. | Medium |
| Readiness becomes private configuration oracle | Redacted fixed schema; no user/profile/path/secret/error details. | Low |

## Ordered, Bounded Implementation Packet

1. **Schema and neutral vectors (blocker H1).** Add `LocalBootstrapPipeV1`, `LocalCredentialEnvelopeV1`, and `HubReadinessV1` schemas under the hub schema taxonomy, with canonical positive and hostile fixtures: valid frames, wrong HMAC, replay, wrong run/sequence/profile/class, expired/oversize/truncated frame, deadline/cancel/EOF, bearer redaction, and ready/not-ready variants. Fix constants in the schema—not per-client defaults.
2. **Directory/bootstrap service (H2).** Replace [`LocalBootstrapTransport`](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:612) with the request-correlated bounded service and direct anonymous-pipe adapter. Reuse [`prepare_auth_session`](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:619), identity digest domain, revocation, and `IdentityVerificationContext`; add no raw capability to an event/projection.
3. **Hub assembly/readiness (H3).** Change [`main`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📦️bin.rs:2099) to accept only launcher-provided pipe ownership, fail closed for missing/invalid pipe, run bootstrap issuance, expose redacted `/readyz`, and record lifecycle audit events without secrets. Correct the test-only `None` wiring. Do this before any browser/native launch change.
4. **Owned profile launcher (H4).** Extend the existing OS dev orchestration [`📜️script.ts`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧪️dev/📜️script.ts:2722) and registered `.vscode` commands to build/select direct children, provision separate pipe endpoints, use unique private nonsecret run roots, wait/revalidate `HubReadinessV1`, manage cancellation, and terminate child trees/relays cleanly. Do not use the generic hub cargo wrapper as endpoint owner.
5. **Client consumption (H5).** Remove `DirectoryClient.mintSession`, native `S_USER` authority and `identity.json` raw token store; introduce memory-only opaque credential transport. Implement the MCP extra-handle adapter and per-browser-profile relay before wiring React/admin. Fail a client that lacks the expected client class/run/profile proof.
6. **Launch/test cleanup (M1).** Replace stale `OS_HUB_ADMIN_TOKEN` preflights/launches, add explicit Devcontainer bridge policy, and make all React/native/admin/MCP debug profiles request profiles—not tokens. No “bare `bun nx run os-hub:dev`” should pretend to be a ready local secure hub.

This packet is deliberately independent of unfinished P2-C/open-plan/CAS internals: readiness can report those capabilities false and the protected issuer still verifies `/auth/sessions/me`. Do not make catalog/CAS readiness `true` until its own authority/retention prerequisites are actually initialized.

## Required Tests And Focused Commands

No commands were run for this audit. After the packets land, use the existing targets/launch topology rather than broad workspace reruns:

```sh
bun nx run os-hub:test-quick
bun nx run os-hub-ts:test
bun nx run @semio-tech/framework-os-dev:test
bun nx run @semio-tech/framework-os-mcp:test
```

Add an explicit registered launch/target pair, such as **Hub: Secure Local Profile Smoke**, which starts the direct hub under a profile launcher—not `cargo run`—and runs this bounded real-process oracle:

1. Parent asserts a non-admin and an admin profile; each has a unique run/profile/device exchange.
2. Confirm `GET /readyz` has `local-bootstrap-pipe-v1`, loopback bind, no secrets, and all required component readiness; prove direct `POST /auth/sessions` is absent.
3. Confirm each envelope works only for its client class at `GET /auth/sessions/me`; duplicate, expired, wrong-HMAC, wrong-profile, and cross-run frames fail; revoke invalidates the session.
4. Capture hub/parent/client stdout and stderr and reject any exact capability, channel key, or profile subject leakage.
5. Open two relay browser profiles and two isolated native/MCP clients; prove one cannot use the other's cookie/envelope, that a wrong/used nonce fails, and that a WS reconnect requires a current session.
6. Kill the hub/launcher at every framed stage; prove pipe EOF/cancellation, no hang beyond deadlines, no usable stale credential, no residue secret, and child-tree cleanup.

The independent oracle must consume the neutral fixtures with Node `crypto`/JSON Schema validation (the existing hub TypeScript test stack already uses these) and compare its HMAC/frame/expiry decisions with the Rust implementation. Run that direct process test on Windows, macOS, Linux, and a devcontainer; a platform skip is not a green result. The test needs captured redacted logs, a clock/control fake for deterministic expiry/cancellation, and a real loopback REST/WS request—not a synthetic directory call.

## First Deterministic Blockers

1. **High — no concrete protected adapter or issuance loop:** startup intentionally supplies `None`, so development hub cannot start; the trait is only a test fixture and no production code calls `receive`/`respond`.
2. **High — client authentication contracts are stale/insecure:** TypeScript and native client flows still attempt arbitrary-email `POST /auth/sessions`; native/browser paths retain raw credentials. Restoring that endpoint would be a regression, not a fix.
3. **High — no secure client delivery mechanism:** native/MCP lack distinct private credential channels and browser/admin lack the token-free relay, so a real bearer would otherwise leak through env, file, stdin, or JS.
4. **High — current launch topology cannot own pipe handles:** wrapper-oriented `nx`/cargo launches and workspace data dirs do not establish parent/child ownership or private run lifecycle.
5. **Medium — readiness has no contract/route:** launch tooling cannot distinguish a listening hub from a securely authenticated, directory/artifact-ready hub.
6. **Medium — path and namespace policy is absent:** data permissions/symlink checks and devcontainer browser-namespace handling must be explicit before calling the flow zero-touch/cross-platform.

