# Presence Lease: Server-Local P0 Acceptance Packet

## Verdict

**RED — live presence has neither an owner identity nor a deadline.**  The hub currently indexes an opaque peer only by `(document_scope_key_v1, actor)`.  A former socket for the same actor can therefore refresh a newer connection's roster row, and its handler-exit cleanup unconditionally deletes that newer row.  An abandoned, successfully handshaken socket remains visible forever until a close path happens.  This is an in-memory correctness defect; it is not an invitation, membership-revocation, durable-directory, or 4401 packet.

No presence lease, expiry, reconnect, or deterministic roster native/process law was found.  The existing wire and UI plumbing is useful, but is not proof of the needed server behaviour.

## Current authority and projection map

| Boundary | Current source fact | P0 consequence |
| --- | --- | --- |
| Socket identity | [`SocketGrantLedgerV1::register_live`](../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:832) mints an opaque `time_ordered_id`; [`SocketLiveLeaseV1`](../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:1375) retains it until `Drop`.  [`socket_live_authority`](../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:1388) validates it before socket work. | Reuse this private `socket_live.id` as the sole lease identity.  Do not accept an epoch, deadline, or actor identity from a `PresencePeer`. |
| Presence state | [`PresenceSession`](../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:410) has only `surface`, `user_id`, `color`, and `peer`; `HubState.presence` is a fresh in-memory `ShardedMap` at [1452](../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:1452). | Add private `socket_live_id` and server-local deadline to this row (rename it `PresenceLeaseSlot` if that makes the visible/non-visible distinction clear).  Neither field belongs in any REST, directory, or binary wire schema. |
| Admission | [`handle_ws`](../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3269) blindly replaces the row after `Session`; `handle_client_frame`](../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3021) writes peer bytes without ownership or size checking. | Initial install, refresh, expiry, and close must all compare the private live id under the same presence publication linearization. |
| Cleanup | Handler exit at [`bin.rs:3432`](../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3432) calls unconditional `remove`, then broadcasts a roster. | Replace with `ShardedMap::remove_if`, which already offers an in-lock predicate at [`async/🦀️.rs:1424`](../../../../../../🧰️framework/🔨️modules/⏳️async/🦀️.rs:1424).  A prior live id must be a no-op: no deletion, no deadline change, and no broadcast. |
| Existing bounded authority | The server already depends on the OS kernel; `directory::os_spr::channel` defines 64 roster entries × 4,096 bytes = 262,144 bytes at [`channel/🦀️.rs:1004`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️.rs:1004).  The socket ledger has a global 4,096-record cap at [`bin.rs:542`](../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:542).  The hub currently enforces neither roster limit on opaque peers. | Reuse the 64 / 4,096 / 262,144 limits; do not create another runtime dependency or an unbounded roster.  A too-large peer or a 65th visible actor is ignored for presence (socket stays authenticated), never refreshes a lease, and produces no fanout. |
| Client wire | Browser worker sends `ClientFrame::Presence` at [`backbone-worker.ts:1764`](../../../../../../🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:1764); React schedules it initially and every five seconds at [`ShellHost/🟦️.tsx:4305`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:4305) and [4372](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:4372). | Client time and `PresencePeer.connected_at_ms` are display data only.  The server lease TTL must be at least three heartbeat periods; choose and pin **15,000 ms** in the server-local policy. |
| Native client | WGPU receives `ArtifactEvent::Presence` at [`Shell/🦀️.rs:3241`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:3241) and its normal chrome phase requests an outbound preview at [10154](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:10154), which reaches `presence_heartbeat_key` at [10475](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:10475). | The server packet need not change WGPU semantics.  Add a focused native shell maintenance test separately to prove the existing phase actually emits one bounded heartbeat; no WGPU rendering pass is implied by server lease acceptance. |
| Public/member/admin projections | `ServerFrame::Presence` becomes `ArtifactEvent::Presence` in sync at [`sync/🦀️.rs:2416`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs:2416).  `DirectoryStreamMessage::Presence` contains only hub-stamped `DirectoryPresenceActor` fields at [`directory schema:1195`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs:1195), and both directory visibility decisions require current membership at [`bin.rs:3872`](../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3872) and [3920](../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3920).  `ConnectionView.presence_known` is an in-memory read at [3544](../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3544); the admin recorded-connections endpoint is durable sync-session data, not a presence projection, at [5171](../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:5171). | On expiry, document and member-directory rosters are updated; `presenceKnown` becomes false on its next derived view.  Do not append a directory event, mutate `SyncSessionRecord`, expose a lease field, or make public discovery show presence. |

`presence_peers` and `directory_presence_actors` currently traverse shards independently and unsorted ([1488](../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:1488), [1500](../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:1500)); that can yield inconsistent/non-deterministic snapshots under concurrent updates.  The P0 must replace them with one bounded, actor-sorted snapshot routine.

## Smallest production packet

### 1. Private policy and slot state

In `🌎️hub/📦️packages/🦀️rust/🚀️bin.rs`, add the private policy next to the existing socket constants:

* `PRESENCE_LEASE_TTL_MS: u64 = 15_000`.
* Reuse `directory::os_spr::channel::{PRESENCE_ROSTER_MAXIMUM_ITEMS, PRESENCE_ROSTER_MAXIMUM_ENTRY_BYTES, PRESENCE_ROSTER_MAXIMUM_BYTES}` exactly.  No duplicate numeric limits.
* A production monotonic server-local clock (`tokio::time::Instant`, or a private state-owned monotonic tick source); tests receive a controllable private clock.  Never use `PresencePeer.connected_at_ms`, browser `Date.now`, URL data, or wall-clock data supplied by a client.
* Add `socket_live_id: String` and `expires_at` to `PresenceSession`.  `peer: None` means “the currently selected live socket owns the slot but is not visible”, rather than absence of the slot.  This prevents an old reconnect from reappearing while the newer socket is still open but its visible peer has expired.

Add one private `presence_publication_gate: Arc<tokio::sync::Mutex<()>>` to `HubState`, initialized in production and the two state factories at [`bin.rs:6018`](../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:6018) / [6073](../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:6073).  It is deliberately a very small first slice: acquire with the existing bounded admission style, hold it across only synchronous map mutation + bounded snapshot + `broadcast::send` + `DirectoryService::publish`, and never across an `await`.  This establishes one total roster order without a new queue or a leaking per-document lock catalog.  `DirectoryService::publish` is synchronous (`🌎️hub/📇️directory/🦀️.rs:1813`).

### 2. One conditional state machine, then one shared publication

Place the following private helpers beside `HubState::presence_peers`; do not enlarge any wire DTO.

1. `install_presence_slot(scope, actor, socket_live_id, metadata, now)`: after the existing socket authority, replace an old slot with the new live id, `peer: None`, and deadline.  If it replaced a visible peer, form one empty/new sorted roster and publish it.  It never releases a color; colors remain ref-counted by socket lifetime at [`acquire_color`/`release_color`](../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:1510).
2. `refresh_presence(scope, actor, socket_live_id, peer, now)`: reject before mutation when `peer.len() > 4096`, when the slot has another live id, or when a visible new actor would exceed 64 entries / 262,144 bytes.  For the matching id, set server deadline to `now + TTL`; only a change from invisible/different peer produces a snapshot and fanout.  Equal-peer heartbeat refreshes only the deadline.
3. `expire_presence_for_live(scope, actor, socket_live_id, now)`: only when both the live id matches and `now >= expires_at`, change a visible peer to `None`; retain the ownership slot.  It sends exactly one roster delta.  A formerly visible, already-empty, foreign, or not-yet-due slot is a no-op.
4. `close_presence_for_live(scope, actor, socket_live_id)`: `remove_if` identity equality.  It sends a roster delta only if it removed a visible peer.  An old handler cannot remove a later reconnect.
5. `presence_snapshot(scope)`: one map traversal, collecting `(actor, peer, DirectoryPresenceActor)`, sorting by **map-key actor** before returning the raw peers and directory actors.  It never decodes the opaque peer blob, and enforces the shared fixed item/byte limits defensively.
6. `publish_presence_delta(scope, snapshot)`: under the publication gate, send `ServerFrame::Presence` first and then `DirectoryStreamMessage::Presence`, matching current order at [3027](../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3027).  No delta means no empty fanout spam.

The selected live id remains after visibility expiry so an old A cannot refresh while B is idle.  Once B’s real handler closes, the slot is removed conditionally; a still-live A may only become visible through a subsequent ordinary heartbeat.  That fallback is the current multi-live-socket policy, not an implicit socket revocation claim.

### 3. Integrate at three real lifecycle sites

* At [`handle_ws` initial install](../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3269), call `install_presence_slot` using `socket_live.id` after `Session` is sent and before releasing the existing socket authority.  Replacement of an old visible session must publish its removal.
* Extend [`handle_client_frame`](../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:2976) and its one call at [3341](../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3341) with the private live id and server time.  The `ClientFrame::Presence` arm calls only `refresh_presence`.
* In the existing one-second `authorization_tick` ([3302](../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3302)), after the current socket authority is Active, call `expire_presence_for_live`.  It is O(1) when not due and does not close or invalidate the socket.  At handler exit ([3432](../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3432)), call `close_presence_for_live` before `release_color`.

This gives progress/cancellation a crisp boundary: a failed/cancelled gate acquisition changes nothing; after a helper enters the synchronous publication critical section it has one terminal local outcome.  It neither awaits DB/directory work nor owns a retained resource.  Visibility expiry must **not** call `socket_live.notify`, `unregister_live`, `record_sync_session_close`, or any 4401 path.

## Schema-first proof and gates

Add a new neutral contract under `🌎️hub/📦️packages/🦀️rust/🧪️fixtures/👥️presence-lease-v1/{🧬️schema/🔣️.json,🧫️fixture/🔣️.json}` and an independent AJV + plain-state-machine oracle in the existing hub [`📜️script.ts`](../../../../../../🌎️hub/📦️packages/🦀️rust/📜️script.ts).  The fixture has server ticks, opaque peer byte lengths/tags, live ids, expected roster actor order, fanout count, directory actor list, and `socketStillLive`; it must not contain a client deadline or decoded peer fields.

Required vectors:

1. invisible install → heartbeat → expiry at the exact deadline, with one visible and one empty roster delta;
2. same bytes before expiry refresh deadline with zero fanout;
3. A visible, B same-actor reconnect replacement, then late A heartbeat, tick, and close: all are no-ops; B heartbeat remains visible;
4. B expires while live, A remains unable to revive it, B heartbeat restores it, then B close removes it;
5. same actor in a second document is independent;
6. entry 65 / byte 4097 / aggregate 262145 are ignored without deadline refresh or broadcast; a freed visible slot allows the next valid heartbeat;
7. actor ordering is lexical by server map key even when opaque peer bytes have contrary content;
8. restart begins with no slots/roster and no durable directory/sync-session mutation; member directory gets presence deltas, public/outsider projections get none.

Add four exact `os-hub` binary laws using the existing real-TCP harness [`run_socket_test`](../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:6004), `spawn_server`, `socket_request`, and binary-frame helpers around [6591](../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:6591):

* `presence_lease_reconnect_rejects_old_live_refresh_and_close`;
* `presence_lease_expires_server_clocked_visibility_without_socket_close`;
* `presence_lease_enforces_shared_roster_bounds_and_actor_order`;
* `presence_lease_restart_is_empty_and_directory_presence_is_member_only`.

The first two must use separate real grants/sockets plus a controllable private server clock/test tick, assert no 4401/1013 or sync-session close on expiry, and assert the old socket’s close cannot remove B.  The last law reopens a fresh `HubState`/SQLite root and confirms no presence durable record appears.  Extend the existing membership-only visibility law (`tests::socket_directory_visibility_requires_membership_even_for_public_spaces`) rather than adding a public exception.

Register a separate `presence-lease-check {source|native|process}` command in `🌎️hub/📦️packages/🦀️rust/📜️script.ts`, targets in `🌎️hub/📦️packages/🦀️rust/📋️project.json`, and the source `launch.seed.jsonc` (then generate `launch.json`; never directly edit generated launch data).  Gates are:

1. `bun nx run os-hub:presence-lease-source-check --skip-nx-cache`: schema/AJV/state-machine plus browser fake-timer test around the existing five-second schedule.  This proves scheduling only.
2. `bun nx run os-hub:presence-lease-native-check --skip-nx-cache`: exact four `os-hub` laws with `--test-threads=1`, plus the shared `PresenceRosterWire` fixed-limit law.
3. `bun nx run os-hub:presence-lease-process-check --skip-nx-cache`: actual three-client WebSocket sequence over the spawned SQLite hub, including reconnect, old-close, exact deadline advance, member directory observation, and restart-empty.  It proves SQLite process behaviour only; it makes no PostgreSQL/Neo4j claim.
4. Separate WGPU exact maintenance test: one attached hub document drives the normal presence chrome phase, emits one bounded heartbeat, applies a received empty roster, and stops on detach.  It is not a rendering, map, or browser acceptance substitute.

## Explicit nonclaims

* No invite redemption, event-sourcing schema, database write, cross-backend transaction, or persisted presence event is introduced.
* No membership-revocation/4401 policy is changed; existing socket revalidation remains the prerequisite before each lease tick.
* The server continues to treat peer bytes as opaque; it does not trust or reconcile a peer’s claimed actor, color, surface, or timestamp.
* Browser schedule and WGPU maintenance source are not evidence of a full two-user GUI runtime until their respective gates run.
* This packet does not repair broadcast lag/rebootstrap, durable connection telemetry, or arbitrary multi-live-socket promotion after the current owner’s final close.

