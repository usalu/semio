# Hub Share-Grant and Directory-Stream Security

## Scope

This implementation addresses the two top-priority hub findings in `📓️terra-hub-ai-audit.md`:

1. Share links were predictable, permanent, document-only credentials that could cross space boundaries and received author authority.
2. The directory WebSocket authenticated neither the subscription nor all outbound frame variants, so private connection, presence, and identity data could cross space boundaries.

The change is intentionally limited to hub directory/share-link code. MCP and broader authentication work are outside this lane.

## Implemented Design

### Share grants

- Replaced time-ordered bearer tokens with 32 bytes (256 bits) of operating-system cryptographic entropy, encoded as 64 lowercase hexadecimal characters without a new runtime dependency.
- Generation fails closed if the platform entropy provider fails. There is no clock/process fallback for bearer credentials.
- Introduced `SpaceDocumentId`, making `(space_id, document_id)` an explicit authorization key at the trait boundary and in every backend query.
- Replaced the old document-only token rows/nodes with share grants containing:
  - a non-secret administration `id`;
  - the secret bearer `token`;
  - `space_id` and `document_id` scope;
  - `created_at`, `expires_at`, and nullable `revoked_at` timestamps.
- A private document is denied by default. The old “no rows means anonymously open” behavior is removed.
- Added positive, overflow-checked TTL handling; the HTTP API defaults to seven days and accepts `ttlSecs` for an explicit lifetime.
- Added admin revocation at `DELETE /spaces/{space_id}/documents/{document_id}/share/{share_id}`. The URL uses the non-secret grant ID rather than the bearer secret.
- Share-grant document sockets receive spectator/read-only authority. Only an authenticated space membership can confer author authority.
- An already-open share socket revalidates the grant every second. Revocation or expiry terminates that session within the bounded interval.
- Space deletion removes its share-grant projection in SQLite, PostgreSQL, and Neo4j.

Share grants are credential records rather than public directory events. In particular, bearer secrets are not placed in the broadcast directory event stream. Space lifecycle cleanup remains driven by the existing `SpaceDeleted` domain event.

### Directory WebSocket

- The directory WebSocket now requires a live, unexpired authentication session before subscribing or replaying.
- The resolved caller retains the exact session ID and is revalidated against storage for every outbound message.
- A single asynchronous privacy boundary covers every frame variant:
  - space events require public visibility or current membership;
  - global identity events are visible only to the identity they name;
  - connection and presence telemetry require current membership even for public spaces;
  - heartbeat frames reveal only the directory sequence and require an active caller session.
- Replay and live delivery use the same event visibility decision.
- The old unconditional forwarding of `Connection` and `Presence` frames is removed.
- An invalid or absent session is upgraded and immediately closed without replay or subscription delivery.

## Backend Parity

The `HubDirectory` contract and dispatcher now expose the same create/revoke/authorize operations for all configured backends.

| Backend | Space/document scope | Expiry | Revocation | Space-delete cleanup | Private default |
| --- | --- | --- | --- | --- | --- |
| SQLite | Implemented | Implemented | Implemented | Implemented | Implemented |
| PostgreSQL | Implemented | Implemented | Implemented | Implemented | Implemented |
| Neo4j | Implemented | Implemented | Implemented | Implemented | Implemented |

PostgreSQL and Neo4j service-backed runtime tests could not be reached because the shared database crate failed compilation first; see Verification.

## Tests Added and Updated

- Added the language-neutral fixture `🔣️share-token-vectors.json` with byte-to-hex vectors and equal-document/cross-space scope vectors.
- Added a third-party oracle test comparing the dependency-free encoder with SQLite's independent `lower(hex(...))` result.
- Added SQLite lifecycle coverage for:
  - tokenless private denial;
  - 256-bit lowercase-hex generation;
  - exact space/document authorization;
  - same-document cross-space denial;
  - revocation and idempotency rejection;
  - expiry denial;
  - invalid TTL rejection.
- Replaced the old share-link socket test with an end-to-end test covering private-default denial, cross-space denial, spectator write rejection, admin revocation, bounded live-socket closure, and rejected reconnect.
- Added a real directory-socket isolation test covering anonymous subscription rejection, authorized heartbeat delivery, suppression of another private space's connection/presence/account/member frames, and delivery of the caller's own authorized connection.
- Updated existing hub socket and REST tests to use real seeded author or member sessions now that private documents no longer have the insecure tokenless fallback.

## Files

- `🌎️hub/📇️directory/🦀️.rs`
- `🌎️hub/📇️directory/🪶️sqlite/🦀️.rs`
- `🌎️hub/📇️directory/🐘️postgres/🦀️.rs`
- `🌎️hub/📇️directory/🌐️neo4j/🦀️.rs`
- `🌎️hub/📇️directory/🧪️tests/🔣️share-token-vectors.json`
- `🌎️hub/📦️packages/🦀️rust/📦️bin.rs`

## Verification

### Static evidence

- `git diff --check -- '🌎️hub/📇️directory' '🌎️hub/📦️packages/🦀️rust'` completed successfully with no output.
- A final Rust call-site search found no obsolete `hub_share_token` schema/query and no remaining document-only share create/authorize call site in `🌎️hub`.
- Manual call-site review covered the trait, three backend implementations, backend dispatcher, REST create/revoke handlers, document authorization, live share revalidation, router registration, replay filter, and every directory stream frame variant.

### Commands attempted

1. `bun nx run os-hub:test-quick -- share_token`
   - Did not reach compilation or execute a test.
   - It remained blocked on `target/debug/.cargo-lock`, held by Cargo PID 21021, and was terminated with exit 130 to avoid consuming the shared worker indefinitely.
   - A final `lsof target/debug/.cargo-lock` still showed PID 21021 holding the lock.

2. `CARGO_TARGET_DIR=<ticket>/🗑️generated/hub-security-target bun nx run os-hub:test-quick --skip-nx-cache -- share_token`
   - Exited 1 before compiling the hub or executing a test.
   - `semio-framework-os-kernel-db` failed with 263 errors from concurrent upstream DB work.
   - Representative blockers: missing `DbIoAsyncDriverFuture`, missing `crate::db_storage::db_io_test_pool`, missing `ToValue`/`FromValue` implementations for hash projections/mutations, mismatched DB IO types, and existing borrow/lifetime failures.

3. `HUB_E2E=1 CARGO_TARGET_DIR=<ticket>/🗑️generated/hub-security-target bun nx run os-hub-ts:test-quick --skip-nx-cache`
   - This repository target requests the default SQLite hub build rather than `--all-features`.
   - Exited 1 before compiling the hub binary or executing a socket test.
   - The same shared `semio-framework-os-kernel-db` crate failed with 252 errors. Representative terminal diagnostics included `E0716` temporary-value lifetime failures and `E0502` mutable/immutable borrow conflicts in `storage/🦀️.rs`, in addition to the missing DB IO symbols and conversion bounds above.

### Exact result

No hub-security test executed in this lane because both an all-feature test target and the independent default-feature hub E2E build failed in the shared DB dependency before rustc reached `semio-hub`. Consequently, this report does **not** claim that the new tests pass or that runtime behavior has been confirmed. A post-DB acceptance lane must run at minimum:

- `bun nx run os-hub:test-quick -- share_token`
- `bun nx run os-hub:test-quick -- directory_ws_isolates_private_realtime_activity_and_global_identity`
- `bun nx run os-hub:test-quick`

and should confirm the PostgreSQL/Neo4j service-backed paths when their test services are available.

## Residual Risk

- Runtime and compiler acceptance of this lane remains pending the unrelated shared DB compilation repair.
- Share bearer secrets are intentionally isolated from the public directory event stream. They remain persisted credential material in each backend; encrypting or one-way hashing persisted bearer secrets is a separate hardening item not requested by the source audit finding.
- This lane does not change dev session minting, SSO, password handling, MCP authorization, rate limiting, or the audit report's lower-priority hub findings.
