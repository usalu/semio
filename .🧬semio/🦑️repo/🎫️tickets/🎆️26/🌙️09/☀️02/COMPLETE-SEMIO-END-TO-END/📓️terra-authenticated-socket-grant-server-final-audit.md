# Authenticated SocketGrant Server Final Audit

Date: 2026-09-03  
Scope: read-only audit of the shared in-progress S1+S2 server packet. No production or test source, plan, or acceptance file was edited. This audit did not run a build, test, or runtime probe.

## Verdict

**ACCEPT for the scoped S1+S2 server packet. REJECT for transport-security release/S3.** The final reread retains the repaired server design: the document-loop send/command/lag windows are closed; its loopback laws cover post-Welcome bootstrap/session, matching and revoked command admission/storage, broadcast delivery, directory replay, and lag control; and ledger expiry retains live consumed records while reclaiming expired non-live tombstones. Sol attributes a clean, fresh-process all-feature packet and the independent AJV/WebCrypto oracle. The active old `/ws` and `/directory/ws` bearer/actor carriers keep the wider release **blocked by S3** in all cases.

This is a source review, not a runtime result. PostgreSQL and Neo4j are inspected implementations only; their parity is not runtime-proven here.

## Exact Invariant

For a v1 record `(selector, digest, exact audience, exact durable subject, liveId)`, an authority-bearing command, bootstrap/session frame, document broadcast, directory replay/live frame, or lag control may occur only while the ordered subject admission is held, the id-only durable binding is active, and that exact record remains live. A successful durable revoke is linearized before ledger invalidation; after invalidation, no pending grant can consume and no later authority action may cross the subject fence. The volatile ledger mutex is never held across an await.

## Boundary Decisions

| Boundary | Evidence in current source | Decision |
| --- | --- | --- |
| Capability grammar and receipt | `🌎️hub/📇️directory/🦀️.rs:413-584` parses lower-hex `socket.v1`; the auth schema fixes it at 107 characters and the receipt has the required schema/protocol/grant/actor/expiry fields. `SocketGrantCapability` is deliberately absent from `HubCapability`. | **ACCEPT, source** |
| Neutral fixture/oracle | `🌎️hub/🔐️auth/🧪️fixtures/🧬️capability-v1/🔣️.json` contains the corrected socket digest and rejected variants; Rust and TypeScript parsers exist. Sol attributes `bun nx run os-hub-ts:test --skip-nx-cache -- --run -t 'validates typed auth capabilities and independently recomputes the socket grant with AJV and WebCrypto'` exit 0, 1 passed/9 skipped. | **ACCEPT, attributed independent gate** |
| Issue, route and subprotocol boundary | Protected issue routes and v1 upgrades are mounted in `🌎️hub/📦️packages/🦀️rust/📦️bin.rs:3269-3302`; v1 query structs deny unknown keys; `socket_grant_from_protocol_header` requires exactly one bounded `Sec-WebSocket-Protocol: semio.socket.v1, socket.v1...` and rejects `Authorization`. The grant is not selected as the response protocol. | **ACCEPT, source** |
| Id-only durable subject | `SocketSubjectV1::revalidate` checks session id/user/generation/role/expiry or share id/selector/exact scope/expiry. The `HubDirectory` port dispatches to SQLite, PostgreSQL and Neo4j; the final attributed packet includes exact SQLite binding-status coverage. | **ACCEPT, source + SQLite gate; PostgreSQL/Neo4j runtime parity unproven** |
| Volatile ledger and consume race | The 4,096 total / 64 pending-per-binding limits, digest-only record, pending-copy then durable revalidate then atomic consume, expiry sweep, consumed/invalidation state, and late-register rejection are in `bin.rs:463-770`. Expiry retains an active consumed record but reclaims a failed-pre-live consumed tombstone at its dial deadline; lease drop removes the last live record. The standard mutex protects only synchronous ledger work. Restart is intentionally empty. | **ACCEPT, source** |
| Revocation and late live registration | Self session/share and admin batch revoke serialize against the same Session/Share or ordered User+Session gates, durably revoke first, then invalidate pending/live ledger entries. `is_live` makes consume→invalidate→late-register fail closed. | **ACCEPT, source**, except identity/bootstrap-only direct directory revocations remain outside this hub invalidator and require a callback before they become active live-revocation paths. |
| Credential-free handshake and actor binding | `SocketHelloV1` is a distinct wire tag with no actor/token. The document handler derives actor/principal from the record and rejects a forged envelope actor before the security gate/storage. A post-handshake legacy `Hello` closes `4401`. | **ACCEPT, source** |
| Document live authority | `socket_live_authority` acquires ordered subject gates with a two-second bound, revalidates durably, and requires live ledger membership. Current command handling holds it through the bounded `handle_client_frame`; broadcast holds it through send; document lag now holds it before the bounded control read/send. Bootstrap follow-up and `Session` sends are similarly gated. Loopback laws stage winning revokes at post-Welcome, command, broadcast, and lag boundaries; the matching actor command persists before its later revoke test. | **ACCEPT, source + attributed isolated all-feature laws** |
| Directory v1 authority and visibility | Directory replay/live sends use the same live guard and bounded visibility revalidation. Space events, presence and connection rows require current membership even for public spaces; global events require the caller user; only a session may receive directory v1. No admin v1 audience is issued. | **ACCEPT, source + attributed SQLite/loopback packet; PostgreSQL/Neo4j runtime parity unproven** |
| Bounds, cancellation and fail-closed paths | Issue, upgrade/live admission, self/share/admin revocation, durable binding reads, rebootstrap control reads, and authority-bearing sends are now bounded to two seconds and use `4401`/`1013` for invalid/unavailable bindings. `SocketRebootstrapControl::is_cancelled` is permanently false, but this path has a hard two-second control deadline rather than an unbounded background operation. | **ACCEPT, source** |
| Secret redaction | v1 URLs carry only routing fields, response failures use `socket grant rejected`, and the selected protocol is constant. Static inspection found no v1 grant/digest in the new errors/readiness surface. It is not an exhaustive audit/log/runtime proof. | **CONDITIONALLY ACCEPT, source only** |
| S3 release state | Legacy `/spaces/{space}/documents/{id}/ws` accepts `Hello { actor, token }`; `/directory/ws` still accepts `?token=` and both routes remain mounted. Existing browser/native/MCP consumers have not been cut over and removed. | **REJECT — explicit S3 non-release blocker** |

## Current Document-Loop Review

The source now has the right essential ordering for the formerly unsafe branches:

- Incoming v1 frames decode, acquire `socket_live_authority`, and hold that admission across `handle_client_frame` and its storage/security work.
- Document fan-out acquires the same admission and holds it across the bounded binary send; its loopback law pauses after receipt, completes durable revocation, then proves no binary frame crosses.
- `send_socket_document_rebootstrap` and the directory equivalent acquire the admission **before** the bounded rebootstrap-control read and send.

The bootstrap loop does not send a follow-up frame or `Session` after a failed live revalidation. The loopback law now stages a revoke immediately after `Welcome` and asserts a terminal `4401` with no authority-bearing binary frame. Its `hello_session.next_frame()` materialization still occurs before the per-frame admission; its output cannot cross the boundary, but prove it is non-authoritative if that method later gains subject-sensitive storage side effects.

## Scoped Acceptance and Release Blocker

The S1+S2 packet is accepted. Sol attributes session `6875`, run from the absolute ticket target: `bun nx run os-hub:socket-grant-check --skip-nx-cache`, exit 0. It reports all 12 binary laws passing in isolated processes, including neutral Rust capability parsing, exact SQLite binding status, credential-free wire codec, and the final `cargo check --all-features --bin os-hub`. Separately, it reports the independent `bun nx run os-hub-ts:test --skip-nx-cache -- --run -t 'validates typed auth capabilities and independently recomputes the socket grant with AJV and WebCrypto'` exit 0 (the selected oracle passed; the surrounding suite has nonmatching skips).

**S3 remains mandatory and is rejected.** Migrate every actual caller to issue → ordered-header v1 upgrade → credential-free hello, then delete both old routes and their `Hello.actor`/`Hello.token` and query-token carriers in one breaking release. Do not advertise this server slice as transport-security release completion beforehand.

## Evidence Qualifications

- This report itself ran no build, test, or runtime probe. The green commands and session are attributed to Sol via the coordinating agent, not independently observed execution by this audit.
- The reported independent final-source command is `bun nx run os-hub-ts:test --skip-nx-cache -- --run -t 'validates typed auth capabilities and independently recomputes the socket grant with AJV and WebCrypto'`, exit 0. The reported registered Rust packet is `bun nx run os-hub:socket-grant-check --skip-nx-cache`, session `6875`, exit 0.
- An earlier shared-process 12-law binary attempt reached 10/12 before database-task exhaustion. It is superseded by, and not counted in place of, the reported isolated-process all-12 green result.
- SQLite has attributed exact binding-status coverage. PostgreSQL and Neo4j implement the same port and were included in the all-feature compile, but no configured backend runtime result is available.
- The ledger is intentionally issuer-process-local. An HTTP issuance and WebSocket upgrade must reach the same process; multi-instance deployment needs explicit affinity or a different durable grant design.

## Final Scope

The accepted packet is server S1+S2 only. It does not establish multi-process grant portability: the ledger deliberately remains issuer-process-local, so deployment must retain issue/upgrade affinity until a future durable-grant design supersedes it. It also does not release any old socket carrier; that is S3's sole, explicit non-release boundary.
