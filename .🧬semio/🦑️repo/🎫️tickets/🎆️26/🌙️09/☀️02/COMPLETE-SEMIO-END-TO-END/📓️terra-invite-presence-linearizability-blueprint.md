# Invite Redemption and Presence Linearizability Audit

## Verdict

Both seams are current production REDs. No command or process proof was run for this audit.

1. Invite redemption releases the only service writer mutex before its durable append, and no backend ever writes `InviteRecord.accepted_at`. An invitation is therefore reusable sequentially and concurrently; the membership projection may remain idempotent, but the event log is not exactly once.
2. Document presence is an unleased `(document scope, stable actor)` map. A stopped connection remains forever, and an old connection can remove or overwrite a newer reconnect because entries do not carry the live-socket identity or generation.

The two repairs are independent. Invite consumption is durable, event-sourced state; presence is deliberately ephemeral and must not be made durable merely to add a lease.

## Current invite path

`DirectoryService` owns `write: tokio::sync::Mutex<HubClock>` at [directory/🦀️.rs](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:1517>). Its general command paths allocate events, call the backend append, explicitly drop the clock, then publish; see [the generic path](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:1602>). That ordering is already weaker than the nearby comment implies.

The redeem path is worse. [redeem_invite](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:1665>) authenticates and constructs `InviteRedeemed` under `write`, then calls `drop(clock)` at line 1690 before `append_events`, and broadcasts only afterwards. A competing directory command can occupy the writer and commit first while the redemption append is in flight.

`InviteRecord` has `accepted_at` ([directory/🦀️.rs](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:222>)), but current source initializes it to `None` and no redemption path assigns it. Each backend rejects a record only when that field is non-null:

- SQLite: [authenticate_invite](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:1209>).
- PostgreSQL: [authenticate_invite](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🐘️postgres/🦀️.rs:1314>).
- Neo4j: [authenticate_invite](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🌐️neo4j/🦀️.rs:1127>).

Thus those checks never make an invite single-use. SQLite persists the field in its schema ([line 159](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:159>)) and appends/projection-updates transactionally ([line 1631](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:1631>)), but its current projection of `InviteRedeemed` only upserts membership. Equivalent backend projection paths exist in PostgreSQL and Neo4j. Projection rebuild preserves stored `accepted_at`; it cannot repair a field never written.

The HTTP endpoint is [post_redeem_invite](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3675>), routed at line 5145. It authenticates the requesting user and forwards the token/user pair; it supplies no second consumption authority.

Current test coverage is one serial create/redeem/revoke round trip ([directory/🦀️.rs](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:4112>)). It does not assert consumption, races, failure atomicity, restart, or stream ordering. The registered admin SQLite gate selects only projection rebuild coverage, not redemption semantics ([hub script](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:4039>)).

## Invite P0: one backend transaction, then ordered publication

Do not extend the service mutex around the current generic append as the fix. That would still leave `accepted_at` unset and provides no cross-process/backend claim transaction.

Add one closed `HubDirectory` redemption operation near the existing invite trait methods ([directory/🦀️.rs](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:2059>)). It accepts only the authenticated actor/user, server clock value, and invite credential; it returns the already-persisted `DirectoryEvent` or a closed result (`Redeemed`, `AlreadyAccepted`, `Revoked`, `Expired`, `Denied`). The backend, not the HTTP caller, derives space, role, invite id, and event body from the claimed row.

For SQLite, PostgreSQL, and Neo4j respectively, make the following a *single backend transaction*:

1. Validate selector/digest, expiry/revocation, and target user.
2. Atomically claim the exact invite only if it remains unaccepted, unrevoked, and unexpired. SQLite uses its immediate write transaction; PostgreSQL uses a row lock or conditional `UPDATE … RETURNING`; Neo4j uses one transactional conditional update.
3. Set `accepted_at`, persist `InviteRedeemed`, and apply membership projection before commit. Any injected projection/event failure rolls back the claim as well.

`DirectoryService::redeem_invite` keeps its writer guard through HLC allocation, this committed backend operation, and synchronous `broadcast::Sender::send`; it releases only after the event has entered the ordered local stream. The other directory event paths should receive the same commit-then-send-before-unlock ordering. A send failure or subscriber lag after commit is not an invitation rollback: reconnect must recover by durable `events_since`, and the law must prove that recovery.

This is event sourcing, not a separate invite-consumption table or a client-provided idempotency claim. The existing invitation row is the durable command decision state; the event stream is the durable effect.

## Current presence path

`HubState.presence` is an `Arc<ShardedMap<(String, String), PresenceSession>>` ([bin.rs](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:1392>)). `PresenceSession` contains only surface, user id, colour and raw peer bytes ([line 410](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:410>)); it has no live lease id, generation, deadline, or server-observed update time.

On socket start, [handle_ws](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3154>) inserts `(scope, stable actor)` with no peer. Socket actors are stable per authenticated session ([socket_actor_id](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:1860>)), while `SocketLiveLeaseV1` separately has a fresh live id in the socket-grant ledger. The presence map discards that identity.

`ClientFrame::Presence` simply mutates the raw peer and fans a roster ([bin.rs](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:2906>)). It neither sets a deadline nor validates/stamps the peer. On every loop exit, cleanup unconditionally removes `(scope, actor)` and fans again ([bin.rs](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3317>)). Therefore an old socket closing after a reconnect removes the reconnect's entry; an old heartbeat can also mutate it.

There is no hub presence timer. Existing hub intervals cover artifact CAS and authorization ticks, not lease expiry. Client heartbeats are 100 ms in the native store sync ([sync/🦀️.rs](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs:994>)) and 5 seconds in ShellHost ([ShellHost/🟦️.tsx](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:4280>)); neither is a server lease.

Roster enumeration is nondeterministic: `presence_peers` scans the sharded map ([bin.rs](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:1426>)), whose shards are hash maps ([async/🦀️.rs](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/⏳️async/🦀️.rs:1312>)). Separate mutation and fanout operations therefore cannot establish a document roster order. The joining socket is also inserted before its receiver/fanout setup, so it does not receive an immediate authoritative existing roster.

`ClientFrame::Presence` carries opaque bytes. `PresencePeer` itself encodes actor/user/role/colour/surface in the replication wire ([wire/🦀️.rs](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/📡️replication/📡️wire/🦀️.rs:1799>)). The hub currently stores and broadcasts client-provided bytes without binding those identity fields to the authenticated socket. This allows an authenticated client to forge client-visible peer identity unless admission decodes, bounds, and overwrites server-owned fields. The currently visible decoder tail should also be reviewed for exact-EOF enforcement before using it as an admission boundary.

Current directory presence messages are filtered telemetry, not an ordered document roster. They lack a lease/generation field in the TypeScript schema ([directory schema](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts:748>)). Presence is absent after process restart only because it is in memory, not because restart semantics are tested.

## Presence P0: per-scope live lease, server-stamped peer, ordered synchronous fanout

Keep presence ephemeral. Add a bounded `PresenceLeaseRegistry` owned by `HubState`, keyed by document scope and stable actor, whose entry includes:

`live_id` from `SocketLiveLeaseV1`, server deadline, stamped/canonical peer bytes, surface, authenticated user id and allocated colour.

Use an explicit per-scope mutex/registry state with fixed entry limits; do not sweep the unbounded `ShardedMap`. Each document websocket task already owns a `select` loop, so it can sleep until the next local deadline and remove only the matching live id. This avoids a global background queue and makes cancellation follow the socket task.

Activation, refresh, expiry, close, and reconnect replacement are short synchronous transitions under the same scope lock. Each transition conditionally compares `live_id`; stale heartbeat/close/timeout has no effect. The transition creates the sorted or otherwise canonical snapshot and calls synchronous fanout before releasing the scope lock. That establishes a single roster order without awaiting while locked. Register the receiver before activation so the new socket receives the existing roster immediately.

On `Presence`, first pass the normal socket live authority check, then decode a strictly bounded, exact-EOF `PresencePeer`. Discard or overwrite actor, user, role, colour and surface with server-authenticated values, re-encode canonical bytes, and renew the deadline. Repeated unchanged heartbeats renew without a new fanout. Only server time owns expiry; select a closed lease duration of at least three browser heartbeat periods and pin it in shared Rust/TypeScript fixture data rather than trusting client timestamps.

The current single entry per `(scope, stable actor)` means latest live connection wins. Preserve that semantics in P0; a per-tab roster is a separate product decision. A restart begins with an empty ephemeral registry, and the old live grant must fail before it can refresh.

## Required proof packets

Add language-neutral fixtures before source implementation:

- `invite-redemption-v1`: a symbolic credential/selector model with fresh redemption, same-actor replay, concurrent different-user race, wrong secret, revoked/expired/unknown user, cross-space injection, before-commit failure, projection failure, restart/rebuild, and ordered event/replay cases. Use AJV plus an independent first-party Node state machine; do not put raw invite secrets in fixtures.
- `presence-lease-v1`: server-time transitions for activation, unchanged refresh, deadline boundary, reconnect/live-id takeover, stale old close/refresh, cross-space isolation, malformed/forged peer rejection, capacity, restart, and ordered roster snapshots.

Native exact laws:

1. SQLite barrier race with two or three redeemers: exactly one success, one durable event/sequence, `accepted_at` persisted, one membership projection. Include transaction failpoint rollback and reopen/rebuild proof.
2. Directory stream law: committed redeem event cannot be observed after a later committed directory event; a lost broadcast recovers via durable replay.
3. Presence registry law: deadline/generation/cross-scope/capacity and canonical ordered snapshots under a test clock.
4. Real websocket law with paused or injected bounded server time: B sees A immediately, abandoning A evicts it at lease expiry, stale A1 cannot remove A2 after reconnect, forged peer cannot alter server-stamped identity, and revoke before refresh changes no roster.
5. Restart law: a new hub has no ephemeral roster and rejects old live authority.

After the native laws, add a separate real SQLite process target—proposed `invite-presence-linearizability-check`—from the hub `📜️script.ts`, `project.json`, and launch *seed* followed by generation. It should start the normal protected local hub with two explicit test profiles, use concurrent HTTP redemption and real WebSocket frames, restart the same SQLite directory, and inspect durable events/rosters. It must exact-select its native laws and never silently skip PostgreSQL, Neo4j, or OIDC claims. Current admin and collaboration commands do not prove this packet.

## Dependency order and nonclaims

1. Land neutral invite and presence contracts plus registered source oracles.
2. Add the backend atomic invite operation and SQLite native race/failure/restart laws; port the same closed operation to PostgreSQL and Neo4j.
3. Add the bounded presence registry and websocket admission/lease laws.
4. Add the real SQLite two-user process proof only after local authenticated profiles and the current document-open startup route are real.

This audit does not claim that PostgreSQL, Neo4j, OIDC, browser rendering, or any process journey currently passes. It also does not treat source-level socket authorization ticks as a presence lease.
