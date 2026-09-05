# Invite Consumption and Presence Lease P0 — Current-Source Audit

## Decisive boundary

These are **two independent P0 packets**, not one shared transaction:

1. **Invite redemption is a durable, cross-process directory decision.** The current service writer is no longer the reported gap: `DirectoryService::redeem_invite` holds it through `append_and_publish_locked`, which durably appends before it broadcasts. The actual RED is that no backend transaction ever marks the accepted invitation `accepted_at`; every redemption therefore continues to authenticate as reusable.
2. **Presence is intentionally ephemeral, but has no server lease or live-connection identity.** A silent socket leaves a ghost indefinitely, and unconditional old-socket cleanup can remove a newer reconnect for the same stable actor. This must remain distinct from durable connection/admin records and the scoped-directory-socket revocation executor.

No native, browser, or process gate was run for this audit.

## Corrections to the earlier audit

[`📓️terra-invite-presence-linearizability-blueprint.md`](./📓️terra-invite-presence-linearizability-blueprint.md) is partly stale:

- Its claim that redeem releases the writer before append is no longer true. Current [`redeem_invite`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:1716>) holds `write` while it invokes [`append_and_publish_locked`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:1604>), which calls backend append before synchronous local stream publication.
- Do **not** introduce a separate global presence registry or durable presence event. The actual document WebSocket already has a one-second authorization tick; its bounded socket loop is the correct lease-driving owner.
- The opaque `Presence.peer` bytes are currently intentionally forwarded without hub decoding. Binding/stamping their internal identity is a separate protocol-hardening decision, not something to assert as achieved by this lease P0.

## 1. Durable invite-consumption RED

### Current execution and storage facts

- `InviteRecord` already declares `accepted_at: Option<i64>` at [directory model](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:222>), and the shared directory trait exposes only lookup/authentication plus direct issue/revoke methods at [lines 2099–2112](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:2099>). It has no closed claim-and-append operation.
- `redeem_invite` authenticates the capability and locally checks revocation/expiry, builds `InviteRedeemed`, then asks the generic backend append path to persist it at [lines 1716–1735](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:1716>). The local mutex orders one process only; it is not a DB claim primitive.
- Every backend reads an invite only while `accepted_at` is absent:
  - SQLite [`authenticate_invite`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:1209>),
  - PostgreSQL [`authenticate_invite`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🐘️postgres/🦀️.rs:1314>),
  - Neo4j [`authenticate_invite`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🌐️neo4j/🦀️.rs:1127>).
- The same three `InviteRedeemed` projectors upsert membership but do not write the acceptance field:
  - SQLite [`project`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:612>),
  - PostgreSQL [`project`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🐘️postgres/🦀️.rs:653>),
  - Neo4j [`project`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🌐️neo4j/🦀️.rs:395>).
- This is not merely a projection reconstruction problem. SQLite persists event and projection in one immediate transaction; PostgreSQL and Neo4j likewise append and invoke projection inside their backend transactions. A serial redeem can therefore write an event while leaving the capability permanently eligible.
- Current coverage is only [`invite_create_redeem_revoke_round_trip`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:4233>): one successful redeem, no second attempt, racing writer, transaction failure, reopen, or rebuild check.

### Smallest clean transaction seam

Add a closed, backend-owned directory operation adjacent to the invite trait region, conceptually:

```text
redeem_invite_atomically(
  authenticated actor/user,
  capability,
  server HLC / recorded time
) -> RedeemInviteOutcome { persisted event } | Conflict / Unauthorized
```

It must **not** accept caller-provided space, role, invite id, event body, or `accepted_at`. The backend derives every one from the claimed invite row.

For a normal live redemption, one database transaction does, in this strict order:

1. Resolve the capability by selector/digest and validate bounded input; inspect the actual row.
2. Conditional claim exact `invite_id` only if `accepted_at IS NULL`, `revoked_at IS NULL`, and `expires_at > server_now`. The conditional mutation has to report **exactly one** changed row.
3. On success stamp `accepted_at` from the server event time, create exactly one `InviteRedeemed` event from the claimed row, and project its membership effect.
4. Commit. Only then may `DirectoryService` publish the persisted event while it still owns its in-process writer guard.

A zero-row conditional update is `Conflict`; it must create neither event nor membership projection. A failure after the claim but before commit rolls the claim back with the event. A post-commit response loss is a normal at-most-once outcome: retry observes a consumed invite, and a reconnect recovers the one durable event through `events_since`.

The backend implementation differs only in the atomic compare-and-set mechanism:

| Backend | Existing transaction boundary | Required claim |
| --- | --- | --- |
| SQLite | Existing immediate transaction in `append_events` | `UPDATE hub_space_invite ... WHERE id=? AND accepted_at IS NULL AND revoked_at IS NULL AND expires_at>?`, require one changed row |
| PostgreSQL | Existing SQL transaction in `append_events` | conditional `UPDATE ... RETURNING` or row lock plus condition, require one returned row |
| Neo4j | Existing `Txn` in `append_events` | one `MATCH/WHERE` conditional `SET acceptedAt`, require count one before event node/projection |

Do not “fix” this by adding an unconditional `accepted_at` projection update. Live acceptance needs the strict changed-row decision before event publication; replay/rebuild needs a separately idempotent projection of an already-existing historical event. The latter must accept only the same recorded acceptance value for the same invite, never manufacture a second event.

The existing [HTTP route](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3799>) remains a thin authenticated capability carrier. Map consumed/revoked/expired to its existing conflict semantics without returning invite details. The route neither gains a client idempotency token nor performs a separate read/claim.

### Invite source-first acceptance packet

1. **Neutral contract:** `directory-invite-redemption-v2` fixtures + schema: success; duplicate by same user; concurrent same token/different users; revoked; expired; bad secret; wrong-space/role substitution; claim-success/event-failure rollback; post-commit reply loss; reopen/rebuild. Expected state carries only hashed/token selectors, one event sequence, membership result, and accepted timestamp.
2. **Independent oracle:** AJV validates fixture shape; a small Node/TypeScript state machine independently models the conditional-claim result. Do not put raw secrets in fixture output.
3. **Exact native backend laws:** SQLite barrier race (two services/process-like handles to one DB), failure injection, reopen and projection rebuild. Port the same asserted transaction contract to PostgreSQL and Neo4j rather than treating their source strings as acceptance.
4. **Ordered-publication law:** extend the existing [`DirectoryOrderedPublicationCheckScript`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:4787>) so the redeem event is observable only after the committed projection and never after a subsequent persisted directory event.
5. **Process law:** one protected local hub, two independently authenticated users concurrently POST the same invite, restart over the same SQLite file, and read the ordered directory socket replay. Exactly one 2xx, one `invite.redeemed`, one membership, one stored `accepted_at`.

Register source/native gates through [hub `📜️script.ts`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts) and the launch **seed**, then generate launch output; never edit generated `launch.json`. Suggested exact commands once implemented:

```sh
bun nx run os-hub:invite-redemption-linearization-check --skip-nx-cache
bun nx run os-hub:invite-redemption-linearization-check --native --skip-nx-cache
bun nx run os-hub:invite-redemption-linearization-process-check --skip-nx-cache
```

The process gate qualifies SQLite only unless a configured PostgreSQL/Neo4j runner executes their exact backend laws.

## 2. Ephemeral presence-lease RED

### Current execution and lifecycle facts

- `PresenceSession` contains only `surface`, `user_id`, colour and optional opaque peer bytes at [bin.rs:410](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:410>). `HubState.presence` is an in-memory `ShardedMap<(document_scope_key, actor), PresenceSession>` at [lines 1452–1459](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:1452>).
- Document socket setup inserts it after `Session` at [line 3269](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3269>). An incoming `ClientFrame::Presence` merely replaces peer bytes and immediately broadcasts roster/directory telemetry at [lines 3021–3029](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3021>).
- The WebSocket has a one-second authorization tick at [lines 3302–3317](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3302>), but it does not timestamp or expire presence. It is correctly separate from scoped socket revocation: `socket_live_authority` checks session, plan and live grant at [lines 1388–1401](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:1388>).
- Handler exit blindly removes `(key, actor)` at [line 3432](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3432>). A reconnect using the same stable actor can overwrite that map entry; the old handler then removes the new presence.
- The roster is not ordered: [`presence_peers`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:1488>) scans the sharded map. This P0 must make expiry/reconnect transitions deterministic; stable wire roster ordering should be added to its fixture rather than relying on shard iteration.
- Sync-session rows are durable admin connection metadata, not presence leases. The trait only exposes open/close/list at [directory lines 2115–2142](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:2115>); SQLite stores just `connected_at`/`disconnected_at` at [lines 1242–1322](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:1242>). Hub startup explicitly clears crash residue. Do not add peer bytes or periodic presence writes to these three backend stores.
- Browser has a real five-second client heartbeat constant at [ShellHelpers line 279](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx:279>) and per-document ShellHost forwarding at [lines 4314–4359](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:4314>).
- Native WGPU declares `publish_presence_heartbeat` at [Shell WGPU line 3324](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:3324>), but current source has no call site. Native therefore has no demonstrated heartbeat schedule.

### Smallest bounded lease design

Keep presence **non-durable** and retain its existing stable-actor-per-document semantic. Extend each stored entry with:

```text
socket_live_id: exact SocketLiveLeaseV1 id
lease_epoch: u64 (monotonic per replacement)
deadline_server_ms: u64
```

The server—not a client timestamp—sets the deadline after successful `socket_live_authority`. A `ClientFrame::Presence` renews it only after the normal frame authority check. The initial insert carries `socket_live.id`; the frame handler needs that id and injected/server time as a narrow added parameter. No client should select actor, scope, epoch, or deadline.

Use the existing per-document socket `authorization_tick` to inspect the current entry once per tick. It conditionally removes an entry only when the map record still has the same `socket_live_id` and epoch and its deadline is reached. It must not await while mutating the map. Build the roster after the conditional mutation, canonicalize order by server-owned actor key, then fan it out once.

**Lease expiry is not a socket revoke.** Expiry removes only an inactive peer from the ephemeral roster; the valid socket and its durable `SyncSessionRecord` remain until actual handler exit/revocation. A later valid heartbeat may re-establish a presence entry with a new epoch. This avoids falsely closing the durable admin connection record. Actual membership/session revocation remains solely the scoped-socket executor’s 4401 path.

Handler close needs the same compare-and-remove. An old connection can release its colour/session resources, but it must not remove a presence entry whose `socket_live_id` belongs to the replacement connection. Replacement after reconnect likewise must not let an old timeout delete the new entry.

A closed constant should bind clients and server: browser sends at 5 seconds now, so define a server lease TTL in a shared schema/fixture as at least three heartbeat periods. Keep the actual timing server-configured/injected in native tests; do not trust client `connectedAtMs`.

Do not add a background queue, durable heartbeat DB writes, raw directory events, or a 4401 close on an ordinary presence timeout. This composes with existing authorized append→fanout behavior without overlapping the scoped-socket executor.

### Presence source-first acceptance packet

1. **Neutral `presence-lease-v1` schema + fixture:** initial activation; refresh; exact deadline; no early expiry; same-actor reconnect replacement; stale old refresh/close/timeout; cross-space same actor; repeated heartbeat no duplicate roster; revocation-before-refresh; server restart empty roster. Each row supplies server ticks and expected canonical roster plus number of fanouts.
2. **Native hub law with injected clock/ticks:** confirms server-only deadline, conditional old cleanup, deterministic order, no await under map mutation, and no sync-session close on peer expiry.
3. **WebSocket two-user law:** B sees A; A goes silent; B sees one removal at expiry; A reconnects and appears once; delayed cleanup from A1 cannot erase A2; membership revoke wins over A2 heartbeat and closes 4401 by existing socket authority.
4. **Browser law:** fake timers assert a mounted document sends beats at the fixture cadence and stop/unmount cancels it. It is client scheduling proof only, not server liveness proof.
5. **Native WGPU law:** wire `publish_presence_heartbeat` into its active document/sync lifecycle and test one bounded emitted beat plus cancellation/close. Until this exists, native presence is RED.
6. **SQLite process law:** two authenticated document sockets plus virtual/bounded clock or short fixture lease; silence/reconnect/restart. Assert no ghost roster and no durable presence table/event. Restart’s existing `close_all_sync_sessions` must remain an admin-session cleanup, not be asserted as a presence lease.

Register separately:

```sh
bun nx run os-hub:presence-lease-check --skip-nx-cache
bun nx run os-hub:presence-lease-check --native --skip-nx-cache
bun nx run os-hub:presence-lease-process-check --skip-nx-cache
```

## Ordered handoff and nonclaims

1. First land the invite contract + strict claim transaction in SQLite, then exact race/failure/restart law; port the *same* closed operation to PostgreSQL and Neo4j.
2. Independently land the ephemeral lease entry and document-socket tick/lifecycle laws.
3. Then add React and WGPU scheduling plus the real two-user SQLite process proof.

This packet makes no claim for PostgreSQL/Neo4j execution, OIDC, public raw-directory events, semantic peer-byte identity stamping, browser rendering, WGPU rendering, server-driven socket close on ordinary presence expiry, or the scoped-membership-revocation implementation. Those boundaries remain intentionally separate.

