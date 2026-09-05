# Server-Local Presence Lease P0

## Boundary

This packet makes live presence a server-local, socket-owned lease. It changes no invitation, membership-revocation, directory event-page, durable connection, or public discovery contract. Presence remains opaque and ephemeral; only the authenticated socket live id and the server monotonic clock control visibility.

## Production implementation

- `HubState.presence` now stores `PresenceLeaseSlot { socket_live_id, expires_at, surface, user_id, color, peer }` under the exact full document scope key plus actor.
- `PRESENCE_LEASE_TTL_MS` is 15,000 ms. The three roster limits are imported from the OS SPR channel: 64 entries, 4,096 bytes per entry, and 262,144 aggregate bytes.
- One bounded `presence_publication_gate` linearizes install, refresh, expiry, and close. No database or directory await occurs while it is held.
- Install happens only after the authenticated `Session` frame. A reconnect replaces the selected live id with an invisible slot and publishes removal of an older visible row.
- Refresh checks exact live id and all limits before changing the deadline. An equal opaque peer refreshes only the deadline and emits no fanout. Rejection leaves the deadline unchanged.
- The authorized one-second socket tick hides a due peer at the exact server deadline but retains its owner slot. It does not notify, unregister, record a sync close, or close the socket.
- Handler cleanup uses `remove_if` on the exact live id before color release. A stale handler cannot erase its replacement.
- One traversal produces aligned document/directory projections sorted by server actor map key. Document fanout precedes the member-only directory projection.
- A private controllable monotonic clock exists only under `cfg(test)`; no deadline or live id enters a wire DTO.

## Neutral proof

The schema and fixture are under `🌎️hub/📦️packages/🦀️rust/🧪️fixtures/👥️presence-lease-v1`, with the fixture in its current `🧪️fixture` owner. Eight language-neutral traces cover exact-deadline expiry, equal refresh, reconnect replacement, stale refresh/tick/close immunity, current-owner revive/close, cross-document isolation, fixed roster bounds, actor ordering, and restart emptiness. The fixture contains only server ticks, opaque peer tags/byte lengths, and live ids; it admits no client deadline or decoded peer identity.

The independent Bun/AJV oracle replays the state machine without calling Rust, rejects four schema authority mutations, and mutation-tests five production fences. It also source-checks the browser's bounded five-second heartbeat lifecycle without treating that schedule as server runtime proof.

## Exact Rust laws

The process target selects exactly these four `os-hub` binary laws:

1. `presence_lease_reconnect_rejects_old_live_refresh_and_close` — two real grants and WebSockets for the same actor; B replaces A, A cannot refresh, and A close cannot remove B.
2. `presence_lease_expires_server_clocked_visibility_without_socket_close` — a real WebSocket expires on a server-controlled exact deadline and successfully refreshes afterward.
3. `presence_lease_enforces_shared_roster_bounds_and_actor_order` — fixed count/entry/aggregate accounting, no deadline refresh on rejection, and server-actor ordering independent of opaque bytes.
4. `presence_lease_restart_is_empty_and_directory_presence_is_member_only` — no durable directory append, member-only directory visibility, and an empty fresh state.

The native target selects the four hub laws plus the OS-kernel law `presence_roster_fixed_maximum_plus_one_returns_the_exact_rejected_owner`; the process target reruns the four hub laws as the server boundary.

## Registered gates and evidence

- `os-hub:presence-lease-source-check`
- `os-hub:presence-lease-native-check`
- `os-hub:presence-lease-process-check`

All permanent targets call `🌎️hub/📦️packages/🦀️rust/📜️script.ts`. Three launch entries are registered at orders 411.098, 411.099, and 411.0995 with ticket-local generated targets and one Cargo job.

Current-source evidence:

- direct source oracle: exit 0, 8 vectors + 4 schema hostiles + 5 source hostiles + 1 browser schedule check;
- registered source sessions `75065`, `95673`, and post-taxonomy final-source `11692`: exit 0, the same 18 checks, Nx target resolved exactly once;
- plugin-registry generation session `32617`: exit 0;
- plugin-registry freshness session `3596`: exit 0, generated catalog and launch bytes fresh.

Native/process runtime remains unclaimed at this checkpoint. The host had three unrelated one-job Cargo builds plus the long-running Stdio WASI compiler active when this source boundary completed, so this lane did not start a competing hub fan-in. A native/process result must name its exact artifact hash and assertion count; source/AJV success is not substituted for it.

## Nonclaims

- No persistent presence, directory event, invitation, presence promotion, or membership-revocation behavior.
- No public/outsider presence projection and no client-selected lease identity or deadline.
- No PostgreSQL/Neo4j process claim.
- No browser GUI, WGPU rendering, or full multi-user acceptance claim from the schedule/source oracle.
