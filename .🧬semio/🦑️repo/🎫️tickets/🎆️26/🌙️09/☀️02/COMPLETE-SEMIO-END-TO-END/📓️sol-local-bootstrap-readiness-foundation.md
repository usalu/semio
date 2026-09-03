# Secure Local Bootstrap And Readiness Foundation

Date: 2026-09-03
Ticket: `26/09/02/COMPLETE-SEMIO-END-TO-END`
Status: source implementation and the requested focused macOS evidence are complete.

## Outcome

The hub now has a schema-first, request-correlated development-local issuer over one anonymous inherited descriptor. The launcher direct-spawns the real built hub, retains the 256-bit per-run HMAC key only in memory, sends fixed-profile assertions over descriptor 3, and receives one-use `DevelopmentLocal` session capabilities. Public HTTP continues to expose only authenticated `GET`/`DELETE /auth/sessions/me`; no HTTP/email/static-token mint, diagnostic hook, or skip-build path was added.

The signed credential contract and authenticated session response now carry schema-backed `sessionKind: "development-local"` and a positive safe-integer `authorizationGeneration`. The launcher, one-shot consumer, live session check, Rust fixture oracle, and independent AJV oracle assert them. Public readiness remains redacted and explicitly rejects or detects either field name.

`GET /readyz` returns the strict redacted `semio.hub.readiness/v1` shape. Aggregate readiness fails closed when any required bootstrap, admin-assets, or artifact-authority predicate is false. In the audited no-catalog environment it therefore returns HTTP 503 with `status: "not-ready"`, `authentication.bootstrapReady: true`, and `artifactAuthority.ready: false`. Component and feature booleans remain independently truthful. Only the bounded security smoke may proceed through this exact partial-readiness state to exercise credential issuance; normal supervised consumer launch still requires HTTP 200 and full aggregate readiness.

## Contracts And Implementation

- `LocalBootstrapPipeV1`, `LocalCredentialEnvelopeV1`, and `HubReadinessV1` live under `🌎️hub/🔐️local-bootstrap/🧬️schema`, with neutral positive, hostile, limit, ready, partial-readiness, and not-ready fixture cases under `🧪️fixtures`.
- Authenticated frames use four-byte big-endian framing and `HMAC-SHA-256(key, "semio/hub/local-bootstrap/v1\0" || BE-u32(canonical JSON bytes) || canonical JSON bytes)`.
- Rust owns the inherited endpoint, OS-entropy hub nonce, constant-time proof comparison, 16 KiB frame cap, eight-request cap, eight-profile cap, monotonic sequence, 15-second exchange window, fixed replay/pending/cancellation slots, EOF/shutdown behavior, and key/capability buffer wiping.
- On Unix the inherited full-duplex descriptor is converted to a nonblocking Tokio `UnixStream`. This makes the idle-channel read deadline terminal and lets the runtime exit without an outstanding blocking-file worker. Signed exchange expiry remains an authenticated per-exchange rejection path. The Windows inherited-handle implementation remains separate and was not runtime-executed here.
- `LocalBootstrapTransport` is a project-owned async service port with accept/issue/reject/cancel/shutdown and explicit verified request identity. Native and MCP one-shot delivery have separate project-owned seams; MCP stdin/stdout is unchanged.
- Issuance maps only a launcher-owned profile to provider `semio.local.bootstrap/v1`, hashes the subject through the existing domain-separated identity digest, and persists only the `session.v1` digest through the directory backend. Administrator access still derives from the configured verified provider/subject allowlist.
- Durable issuance is an explicit commit point: the final progress notification is infallible. Cancellation or delivery failure after commit returns the issued identity to the service and durably revokes it; the service does not report a false failed issuance after persistence.
- The existing hub `dev` target is the launcher and starts a direct executable child. The focused `secure-local-smoke` target always builds the hub, creates a private `0700` per-run directory, strips ambient token authority, supplies no trusted catalog, and owns bounded child/pipe cleanup.

## Exact Evidence

Earlier foundation checks retained by this packet:

1. `CARGO_TARGET_DIR=<ticket>/🗑️generated/local-bootstrap-target RUST_MIN_STACK=33554432 SEMIO_TEST_BUDGET_MS=120000 bun nx run os-hub:test -- local_bootstrap_hmac_matches_neutral_node_oracle_and_rejects_boundaries`
   - Green: 1 passed, 76 filtered out.
   - Compiled the hub library, binary, and configured SQLite/PostgreSQL/Neo4j feature surfaces.
2. `bun nx run os-hub-ts:test -- -t 'local bootstrap'`
   - Green: 1 passed, 9 skipped.
   - Independent AJV 2020 and Node `crypto` reproduced the neutral schema/HMAC output and rejected hostile and max+1 inputs. The oracle is now also populated with the signed session metadata and the aggregate-ready versus bootstrap-ready-without-artifact distinction.
3. `bun build 🌎️hub/📦️packages/🦀️rust/📜️script.ts --target=bun --outdir <ticket>/🗑️generated/script-check-final`
   - Green: launcher script bundled successfully.

Final-source requested gates:

1. `CARGO_TARGET_DIR=<ticket>/🗑️generated/local-bootstrap-target RUST_MIN_STACK=33554432 SEMIO_TEST_BUDGET_MS=120000 bun nx run os-hub:test -- local_session_commit_survives_cancellation_observed_by_final_progress`
   - Green: 1 passed, 80 skipped.
   - Nextest run `e68b692c-dda6-495b-8ba0-51ddeb07305a`.
2. `CARGO_TARGET_DIR=<ticket>/🗑️generated/local-bootstrap-target RUST_MIN_STACK=33554432 SEMIO_TEST_BUDGET_MS=120000 bun nx run os-hub:test -- readiness_v1_is_redacted_and_never_claims_public_session_issuance`
   - Green: 1 passed, 80 skipped.
   - Nextest run `7dbeedce-b7ef-4c8c-84a9-a8788a9d20e1`.
3. `CARGO_TARGET_DIR=<ticket>/🗑️generated/local-bootstrap-target RUST_MIN_STACK=33554432 bun nx run os-hub:secure-local-smoke`
   - Green on native macOS with the real freshly built hub; terminal process exit 0, captured exec session `18097`.
   - Final build completed in 11.84 seconds.
   - Terminal oracle: `secure-local-smoke: truthful partial readiness, native/MCP delivery, admin isolation, issue/validate/revoke, replay/HMAC/class/profile/run/expiry/timeout/EOF rejection, redaction, and absent public mint passed`.
   - The smoke explicitly observed the no-catalog HTTP 503 readiness body, then used only its bounded security-smoke predicate to exercise mutual proof, three envelope deliveries, authenticated session correlation, revocation, adversarial frames, idle-channel terminal timeout, deterministic EOF, secret-free captured output/readiness, and absence of public mint routes.

One intermediate rerun exposed an unowned concurrent CAS test caller that still invoked removed `DbIoPageWriter::finish`. The P2-D lane migrated that caller to the production stepped sealing API and restored its all-feature gate. Both focused bootstrap laws above were then rerun against the repaired final shared source; the transient CAS compiler error is not counted as bootstrap evidence or a bootstrap failure. An unrelated isolated CAD WASI Cargo process was active during the process run but caused no observed failure.

## Exact Residuals

1. The browser/admin relay remains a typed fail-closed `BrowserCredentialRelay` port. No first-party relay or cookie/nonce flow is implemented, and no bearer is placed in environment, file, CLI, URL, DOM, JavaScript storage, or an HTTP mint route.
2. Native and MCP one-shot delivery primitives and a direct-child oracle exist, but the actual OS native client and MCP application launchers have not been migrated to consume the envelope. Their existing persisted `S_USER`/`identity.json` paths remain a separate migration blocker and are not described as secure.
3. The executed runtime evidence is macOS only. Linux, Windows, and devcontainer execution remains required; the Windows inherited-handle/async-file path in particular has not received the Unix lifecycle proof. No platform skip is counted as green.
4. Production remains deliberately fail closed because no concrete production `IdentityAssertionVerifier`/IdP is composed. Development startup without descriptor 3 also fails closed.
5. No trusted artifact catalog was composed in this packet. Consequently full hub readiness correctly remains HTTP 503/`not-ready` even though transport/bootstrap is ready; actual supervised consumer launch must wait for a trusted catalog and HTTP 200 aggregate readiness.
6. Feature booleans are intentionally independent and do not imply shipped browser relay, MCP workspace, open-plan, inference, or other consumer integrations.
7. The Nx targets are registered, but the repository launch-profile generator still does not emit the direct hub launch entry. Generated `.vscode/launch.json` output was not patched by hand.
8. This packet proves the scoped bootstrap/readiness foundation and macOS security oracle only. It does not claim an integrated release, real consumer migration, cross-platform runtime completion, production identity, or catalog authority.
