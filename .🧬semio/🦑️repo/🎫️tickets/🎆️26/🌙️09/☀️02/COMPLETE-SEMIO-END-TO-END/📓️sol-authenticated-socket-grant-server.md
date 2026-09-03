# Authenticated SocketGrant S1+S2 Server

Date: 2026-09-03  
Umbrella ticket: `26/09/02/COMPLETE-SEMIO-END-TO-END`  
Scope: SocketGrant S1+S2 server vertical slice only

## Result

The schema-first authenticated SocketGrant server slice is implemented and its focused final-source gate is green. The v1 grant is a process-local, single-consume, short-lived dial credential. After admission, authority is the exact durable session/share binding, not the grant TTL. Every v1 authority-bearing command, bootstrap, broadcast, lag-control read, directory replay/event, and send is ordered through the subject admission/revoke gate and a bounded durable revalidation.

This is not a release claim. The old insecure WebSocket carrier/routes remain for the separate S3 client migration/removal packet. The admin-directory/UI journey is also outside this slice.

## Contract delivered

- Canonical capability grammar is lower-hex `socket.v1.<32hex>.<64hex>` (107 characters). The digest is SHA-256 over `semio/hub/socket/v1\0 || selector || secret`.
- The neutral schema and fixture include the exact digest vector, hostile grammar/digest vectors, and the required public receipt. The receipt contains `grant`, expiry, protocol/schema identities, and a stable non-secret server-derived `actorId`; it contains no session/share secret or durable selector metadata.
- `CapabilityKind::Socket` is domain-separated from HTTP `HubCapability`; SocketGrant is never accepted as an HTTP bearer capability.
- `SocketHelloV1` is an explicitly credential-free Rust/TypeScript wire frame. V1 rejects the legacy actor/token Hello carrier after upgrade.
- Protected issue routes authenticate and authorize before descriptor disclosure. Unauthorized existing and missing documents have the same opaque 401 result.
- Exact v1 WebSocket routes require the SocketGrant subprotocol and reject HTTP `Authorization` on the upgrade.
- The durable binding read port is id-only and returns exact session generation/user/space membership or exact share selector/scope/status. Memory dispatch, SQLite, PostgreSQL, and Neo4j implementations are present; SQLite has the focused runtime law, while PostgreSQL/Neo4j are compile-qualified below.
- The process ledger is bounded to 4096 records and 64 pending grants per binding. It stores only the selector and secret digest, consumes exactly once, binds live IDs to exact selectors, and has no await while holding its standard mutex.
- Expired pending grants and expired consumed pre-live tombstones are reclaimed. An expired consumed record remains while its exact live lease exists because grant TTL governs dial/consume only; the last unregister reclaims it. The full-capacity abandoned-upgrade law proves recovery without permitting replay.
- Session subject ordering is User then Session; share ordering uses the Share gate. Gate acquisition, durable revalidation/read/action, and authority-bearing send operations are bounded at two seconds and fail closed.
- Admission drops gates during slow Hello/descriptor preparation, then reacquires before exact live registration, final durable validation, and Welcome. Gates are dropped immediately after each linearized authority frame/action rather than being held for the connection lifetime.
- Successful self/share/admin revocation holds the appropriate subject gate across durable commit and ledger invalidation/notification. Failed or unavailable issuance does not invalidate unrelated bindings. Admin batch revoke uses the user gate so a late same-user issue/admission cannot cross the durable revoke.
- Document v1 constructs the server actor and rejects a forged envelope actor. Durable revocation winning before a matching-actor command prevents storage mutation and Ack.
- Welcome, bootstrap, Session, broadcast, command, lag-control, and directory paths revalidate in the required authorization-before-disclosure/action order. A winning revoke produces 4401 and suppresses subsequent authority frames/actions. Directory v1 is membership-filtered even for public spaces.
- Grant and receipt debug/serialization paths redact secrets.

## Source surfaces

- `🌎️hub/🔐️auth/🧬️schema/🔣️.json`
- `🌎️hub/🔐️auth/🧬️schema/🟦️.ts`
- `🌎️hub/🔐️auth/🧪️fixtures/🧬️capability-v1/🔣️.json`
- `🧰️framework/🔨️modules/📡️replication/📡️wire/🦀️.rs`
- `🧰️framework/🔨️modules/📡️replication/🟦️.ts`
- `🌎️hub/📇️directory/{🦀️.rs,🪶️sqlite/🦀️.rs,🐘️postgres/🦀️.rs,🌐️neo4j/🦀️.rs}`
- `🌎️hub/📦️packages/🦀️rust/📦️bin.rs`
- `🌎️hub/📦️packages/🟦️typescript/🧪️index.test.ts`
- `🌎️hub/📦️packages/🦀️rust/{📜️script.ts,📋️project.json}`
- `.vscode/{🧩️launch.seed.jsonc,launch.json}`

## Final-source evidence

### Registered focused Rust/backend/wire/compile gate

Command:

```text
CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🗑️generated/socket-grant-target bun nx run os-hub:socket-grant-check --skip-nx-cache
```

Terminal session `6875`, exit `0`.

The owning `📜️script.ts` runs each of the following 12 bin laws in a fresh process with `--all-features`, `--exact`, one test thread, and a 256 MiB test thread stack:

1. directory forced-lag scope authorization and 1013 close
2. document forced-lag verified control and 1013 close
3. admin user-gate versus late same-user grant
4. directory revoke after admission suppresses replay without deadlock
5. public-space directory visibility still requires membership
6. credential-free directory grant, live revoke
7. document exact scope/replay/actor/redaction/legacy-carrier/live+pending revoke behavior
8. bounded ledger, single consume, restart, exact selector, revoke race, TTL/live retention, unregister reclamation, and 4096-record abandoned-consume recovery
9. Welcome versus durable revoke bounded linearization
10. broadcast receive versus revoke disclosure suppression
11. matching-actor command admission versus revoke, with no durable mutation
12. lag authorization versus revoke, with no private rebootstrap/control read

All 12 exact processes passed. The same target then passed:

- Rust neutral SHA-256 capability vectors and fixed boundaries: `1 passed`.
- SQLite exact id/generation/selector/scope/status binding reads: `1 passed`.
- credential-free `SocketHelloV1` wire round-trip: `1 passed`.
- `cargo check --manifest-path Cargo.toml --all-features --bin os-hub`: exit `0`.

The all-feature compile covers PostgreSQL and Neo4j implementations. No PostgreSQL runtime fixture was provisioned. Neo4j has no runtime test fixture in this package; the Hub-specific `OS_HUB_NEO4J_*` configuration was not provisioned for this gate. PostgreSQL and Neo4j are therefore compile-only evidence, not runtime claims.

### Independent neutral third-party oracle

Command:

```text
bun nx run os-hub-ts:test --skip-nx-cache -- --run -t 'validates typed auth capabilities and independently recomputes the socket grant with AJV and WebCrypto'
```

Terminal session `16133`, exit `0`: `1 passed`, `9 skipped`. AJV validated the neutral schema and WebCrypto independently recomputed the exact SHA-256 digest; the test also exercised grammar rejection, wrong-secret mismatch, and receipt redaction.

### Launch generation and diff hygiene

```text
bun nx run @semio-tech/plugin-registry:check-generated --skip-nx-cache
```

Terminal session `86037`, exit `0`: generated catalog and launch bytes are fresh.

`git diff HEAD --check` over all listed owned surfaces exited `0`.

## Qualified residuals and nonclaims

1. A direct single-process `cargo test --all-features --bin os-hub socket_ -- --test-threads=1` final-source diagnostic was red after ten green tests: `socket_grant_revoke_before_command_admission_has_no_storage_effect` timed out before Welcome, and `socket_grant_revoke_before_lag_authorization_reads_no_private_control` failed opening its DB with `db I/O task capacity exhausted`. The DB storage task arena is process-global and fixed at 64; repeated server fixtures do not expose a bounded Database/server shutdown seam. Both exact laws pass in fresh processes, and the registered focused gate deliberately makes that isolation explicit. The combined package-filter result is not claimed green and the shared test DB lifecycle remains a concrete harness residual.
2. PostgreSQL and Neo4j binding implementations are compile-only in this evidence set. No runtime parity claim is made for those backends.
3. S3 remains mandatory: migrate Rust/TypeScript/browser/native clients, delete the actor/token Hello carrier and old insecure WebSocket routes, and prove no bearer appears in URL/log/history surfaces. Until S3 is complete, this packet is not releasable.
4. Admin directory/UI/product completion is excluded. The admin batch-revoke user gate included here only closes the server admission race; it is not an admin feature completion claim.
5. The all-feature build emits existing workspace warnings and a future-incompatibility notice. This packet does not claim a zero-warning workspace.

No generated compiler logs are retained. The ticket-owned Cargo target directory is temporary and is removed after recording this report.
