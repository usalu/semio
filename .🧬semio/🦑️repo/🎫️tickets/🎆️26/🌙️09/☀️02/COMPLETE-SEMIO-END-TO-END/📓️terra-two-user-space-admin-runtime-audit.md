# Two-User Space Collaboration and Admin Runtime Audit

Status: **source-provisional control plane; RED for an executable two-user document journey.** This audit ran no build, process, browser, database, or socket. Runtime evidence below is explicitly quoted from the existing registered-gate record, not re-run here.

## Current authority and transport map

| Obligation | Current evidence | Classification |
| --- | --- | --- |
| Authenticated session and role | `resolve_auth` accepts only a durable `SessionCapability`, then re-reads the caller's current space role; an exact document share is read-only fallback ([`🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:1548-1572`]). Local development uses the FD3 authenticated bootstrap protocol; `startLocalHub` creates constrained profiles and issues a capability only through that channel ([`🌎️hub/📦️packages/🦀️rust/📜️script.ts:586-716`]). Production still has no wired `IdentityAssertionVerifier`: `main` initializes it as `None`, and production startup fails closed ([`🚀️bin.rs:5304-5320`]). | Local source path; production identity integration RED. |
| Space/membership CQRS | Rust and TS share the closed `DirectoryCommand` taxonomy, including member upsert/remove and document announcement ([`🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs:202-214`, [`🟦️.ts:135-158`]). REST derives the actor from the authenticated session and checks owner/author role before `DirectoryService.execute` ([`🚀️bin.rs:3556-3620`]); the service serializes ordinary decision → durable append → stream publish ([`🌎️hub/📇️directory/🦀️.rs:1602-1612`]). | Source-provisional. |
| Public/member/author privacy | Current DTOs distinguish `Public`, `Member`, and `Author`; public detail has no members/invites ([`directory schema:467-489`]). The shared decision function is used by REST, event, connection, presence, and rebootstrap stream filtering ([`🚀️bin.rs:3490-3554`, `3700-3890`]). | Source-provisional; supersedes the earlier public-member-PII finding. |
| Document socket and mutations | A plan/grant subject is derived from the durable session/share record, and a socket actor is server-stamped from the grant ([`🚀️bin.rs:1912-1948`, `2927-2964`]). `Commands` rejects a supplied actor different from the grant actor, applies the role/tenant/replay gate before Fsync submit, then fans out only an accepted command frame ([`🚀️bin.rs:2818-2922`]). | Strong source boundary; no two-user process result. |
| Join, update, leave | On `Session` the hub inserts `(scope, server actor)` presence, records a sync session, and publishes connection telemetry; `Presence` replaces opaque peer bytes and sends document-wide roster; handler exit closes recorded sync session and removes/fans-out presence ([`🚀️bin.rs:2907-2916`, `3154-3318`]). | Clean-close source path only. |
| Reconnect/revocation | Every document and directory socket revalidates its session/share binding on a one-second tick and before command/broadcast delivery; session revoke invalidates pending plans/grants and notifies recorded live sessions ([`🚀️bin.rs:3192-3307`, `4595-4628`]). The browser worker has one-use receipt exchange and jittered reconnect mechanics ([`🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:902-1005`]). | Source-provisional; no process reconnect/duplicate proof. |
| Admin commands | `AdminIntentV1` is closed and principal is reconstructed from durable session/provider/subject/generation on each request ([`directory schema:270-318`, [`🚀️bin.rs:1793-1820`]). Admin space/member actions go through event-sourced directory execution; revoke-session is durable then invalidates grants and signals each recorded live session; kick remains intentionally ephemeral ([`🚀️bin.rs:4529-4628`]). Cursors are principal/route/scope MAC-bound and reads are bounded ([`🚀️bin.rs:4030-4310`]). | Source-provisional plus qualified local runtime evidence below. |
| Restart | Startup reopens storage/directory, marks crash-residue sync sessions closed, and recreates ephemeral fanout/presence from empty state ([`🚀️bin.rs:5328-5351`]). | Source only for a document restart; no two-user tail/reopen proof. |

## Decisive current REDs

1. **No real two-user document socket can be started by the registered collaboration E2E.** D1 plan issuance needs `openable_catalog`; ordinary local startup has none unless a verified profile is configured ([`🚀️bin.rs:2018`, `5320-5364`]). The current linked catalog has no Flow provider/bundle, as recorded in [`📓️terra-flow-trusted-codec-bootstrap-audit.md`](📓️terra-flow-trusted-codec-bootstrap-audit.md). The server wire P0 must depend on that trusted Flow bootstrap; it must not use the test-only fixture grant or an undocumented direct socket path.
2. **The registered browser journey is obsolete at its first boot barrier.** `collabStartHub` exports legacy `OS_HUB_ADMIN_TOKEN` and polls `/admin/api/overview` with it ([`🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:2705-2724`]), while protected local startup strips that variable and requires bootstrap-issued sessions ([`🌎️hub/📦️packages/🦀️rust/📜️script.ts:608-640`]). It also starts the two shells with `S_USER`, which is explicitly a protected/removed environment carrier, not a credential ([`dev script:2846-2865`; `hub script:615,797-798`]). Thus `bun nx run @semio-tech/framework-os-dev:collab-e2e` cannot establish two authenticated people.
3. **The user action cannot reach a real hub document anyway.** The Flow/space action relay opens an app first and calls `openDocument` only when effect args contain `documentId` and `schema` ([`ShellHost:3153-3185`]); the E2E itself documents that its artifact effect lacks that identity ([`dev script:346-355`]). The worker also refuses a hub open without an installed target ([`🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:1600-1665`]). This is independent of the D1 server receipt and blocks browser-visible editor/mutation/presence proof.
4. **Presence has no lease expiry.** `PresenceSession` has surface/user/color/peer only; it is removed only on socket-handler exit ([`🚀️bin.rs:410-422`, `3315-3324`]). Client heartbeat producers exist, but there is no server last-seen/deadline/sweep; the only directory `Heartbeat` publication is a test (`🚀️bin.rs:7792`). A dead/stalled transport can retain a roster entry indefinitely, so failure-bounded leave/reconnect cannot be accepted.
5. **Invite redemption is not ordered with ordinary directory commands as its docstring claims.** It holds `self.write` while deciding, then explicitly drops it before `append_events` ([`🌎️hub/📇️directory/🦀️.rs:1671-1704`]). A concurrent command can append/publish ahead of the redeemed event. Do not use invite redemption as P0 membership establishment; use the ordinary author/admin upsert path until this is linearized and tested.

## Evidence that is real, but deliberately narrower

The existing admin journey report records a final registered process result: `bun nx run os-hub:admin-live-journey-check --skip-nx-cache`, session `30455`, exit 0. It used protected loopback bootstrap, the shipped SPA/relay, a real hub process and SQLite; it exercised EN/DE UI, overview, durable admin create-space, bounded reads, rebuild poll/cancel, and shutdown ([`📓️sol-admin-live-bilingual-sqlite-journey.md`](📓️sol-admin-live-bilingual-sqlite-journey.md)). This is credible local SQLite/admin evidence, **not** evidence for two user membership, a document socket, mutation fanout, presence expiry, restart recovery, PostgreSQL/Neo4j, or production OIDC.

The existing `os-hub:socket-grant-check` and exact Rust socket laws are source/native-law evidence for one-use grant, actor binding, current-scope revalidation, and revocation. They are not a real two-browser or two-identity process journey. Current client presence UI exists (`#s-presence-peers` is driven from document presence events at [`ShellHost:5456-5478`]), but it is not an accepted runtime assertion because its document-opening prerequisites are still RED.

## Smallest honest P0: authenticated two-user server-wire journey

**Dependency:** first land the server-only Flow trusted-provider/bootstrap packet. It must supply an actual Flow descriptor/component/profile and allow a verified Flow open target. This P0 does not claim browser/WGPU rendering or client target leasing.

**Owner and registration:** extend `🌎️hub/📦️packages/🦀️rust/📜️script.ts` with a `two-user-space-process-check` command, register `os-hub:two-user-space-process-check` in `🌎️hub/📦️packages/🦀️rust/📋️project.json`, and add its uncached generated launch entry through `.vscode/🧩️launch.seed.jsonc` then regenerate. The test owns a fresh `startLocalHub` run with three FD3 profiles (`author`, `member`, `administrator`), not email minting, `S_USER`, a static token, a mock fetch, or an in-process `HubState`.

**Exact process sequence:**

1. Materialize verified Flow bytes/profile before spawn; issue three local credentials after the mutual bootstrap handshake. Assert no public session-mint route exists and each `/auth/sessions/me` identity is distinct.
2. As author, create a private space, upsert the member as `Author`, and announce a descriptor built from the verified Flow record. Use the directory socket as an actual client-visible event stream and assert the member sees the durable events; an outsider sees neither raw events nor presence/connection telemetry.
3. As both authors, issue independent D1 open plan/one-use grant and open two raw v1 document sockets. Each sends actual `SocketHelloV1`, receives the server-stamped `Session`, publishes a valid `Presence` peer, and observes the same two-peer document roster. The assertions are decoded real `ServerFrame`/directory messages, not a mock client state.
4. Submit one valid author command and assert Fsync ack plus exactly one peer `Commands` frame. A spectator's equivalent command is rejected and changes neither peer frontier nor durable document state. Cross-space plan/grant/socket, actor substitution, replayed plan/grant, and duplicate command all fail without a peer frame.
5. Close member socket, reissue a **fresh** plan/grant, reconnect with the recorded frontier, and assert no duplicate command and a repaired roster. Admin revokes member sessions through `/admin/api/intents`; assert the live socket closes `4401`, a new plan/grant is denied, and the administrator's own session remains valid. Test kick separately: it closes a connection but does not revoke that member session.
6. Stop the hub, restart against the same data root with a new local-bootstrap run and re-materialized verified profile, issue fresh credentials, and assert directory membership/descriptor plus the accepted mutation frontier survive. Assert old plan/grant/connection does not survive the process epoch.

**Neutral hostile corpus:** versioned rows for `(author, member, viewer, outsider, admin)` identity bindings; scoped descriptor and frontier; expected REST status/close code; first receipt/second receipt; expected roster/event/mutation cardinalities; and restart epoch. Require at least: cross-space scope, spectator write, revoked admin session, stale/replayed socket grant, reconnect duplicate, actor substitution, membership removed while live, and restart with old credential/plan. Validate JSON with AJV plus independent Node SHA-256 over descriptor/receipt vectors; Rust and process driver consume the same rows.

**Explicit P0 boundary:** clean close and reconnect are covered; stalled-presence expiry is an expected RED until the lease/sweep schema lands. Browser Flow UI, WGPU, MCP collaboration controls, checkpoint/lag rebootstrap, public catalog discovery, production verifier/OIDC, and non-SQLite backends remain outside this packet.

## Commands and current disposition

```sh
# Existing registered local SQLite admin process proof; recorded PASS in session 30455, not re-run here.
bun nx run os-hub:admin-live-journey-check --skip-nx-cache

# Existing registered browser collaboration target: current-source RED at obsolete bootstrap and document identity/target wiring.
bun nx run @semio-tech/framework-os-dev:collab-e2e

# Proposed only after Flow trusted bootstrap and the P0 driver are implemented/registered.
bun nx run os-hub:two-user-space-process-check --skip-nx-cache
```

The future P0 must be exact-selector guarded, use its ticket run root only, and report external build/toolchain failures separately from failed assertions.

