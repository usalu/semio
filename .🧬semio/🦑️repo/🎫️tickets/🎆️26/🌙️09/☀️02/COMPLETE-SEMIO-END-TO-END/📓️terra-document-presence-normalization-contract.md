# Terra Document Presence Normalization Contract

Read-only current-tree audit, 2026-09-05. No source was edited and no build, native law, browser, or WGPU executable was run.

## Decision

Use the existing binary `protocol::PresencePeer` / TypeScript `ArtifactPresencePeer` as the one payload type. **Do not add an identity DTO, a parallel roster frame, or a client-to-server identity schema.** At Hub ingress, bounded-decode the client blob, discard all client-supplied authority fields, copy only allowed ephemeral fields into a fresh `PresencePeer`, and re-encode it with the existing `encode_presence_peer`. The resulting bytes—not an opaque client blob—are the only bytes stored in `PresenceLeaseSlot.peer` and sent in `ServerFrame::Presence`.

The current Hub authenticates the document socket well, but it never decodes its peer blob. Thus a raw authenticated socket can impersonate actor, user, role, label, color, surface, and connection time in browser/native roster chrome. A byte-size cap alone is insufficient: the current shared decoder allocates `Vec::with_capacity(count as usize)` from untrusted varints before confirming that enough bytes exist.

## Current Wire and Authority Evidence

`PresencePeer` is a stable existing shared binary shape, not JSON:

- Rust declaration and full field list: `🧰️framework/🔨️modules/📡️replication/📡️wire/🦀️.rs:1792-1824`.
- Rust binary codec: `:1923-2029`; TypeScript twin: `🧰️framework/🔨️modules/📡️replication/🟦️.ts:78-103,372-435`.
- `ClientFrame::Presence` and `ServerFrame::Presence` deliberately carry `Vec<u8>` in the frame layer (`📡️wire/🦀️.rs:53,561-567,880-913,969-1065`). That layer does not own the peer codec.
- Current Hub `refresh_presence` accepts/stores/publishes raw `Vec<u8>` at `🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:1590-1627`; its only ingress call forwards `ClientFrame::Presence { peer }` untouched at `:3294-3296`.

The document socket already contains all required authority; no client field is needed:

- `SocketGrantRecordV1` owns `actor_id`, `SocketSubjectV1`, scope, and optional plan at `bin.rs:634-713`; a session subject has the authoritative `user_id` and `SpaceRole` at `:654-685`.
- The initial hello replaces any client actor with `socket_grant.actor_id` at `:3317-3327`; the Hub captures the authoritative session user/role at `:3335-3342`.
- A plan-backed socket must match the authority subject, actor, scope, descriptor, catalog revision/checkpoint, and exact plan `surface_id` at `:2979-3020`; the URL surface is admitted before upgrade at `:3054-3070` and rechecked at `:3439-3441`.
- Hub assigns a per-space connection color via `acquire_color` at `:3360-3365`, sends it in `ServerFrame::Session` at `:3516-3536`, and already keeps the same color/user/surface beside each lease at `:428-436,1573-1581`.
- The existing directory `UserRecord` already owns `display_name`; do not invent a presence identity wrapper (`🌎️hub/📇️directory/🦀️.rs:64-76`). The handler already performs a post-admission `get_user` lookup for email at `bin.rs:3547-3551`; retrieve the display name once in that bounded connection setup and retain it in the lease.

## Exact Field Contract

| Peer field | Authority | Normalized output | Rationale |
| --- | --- | --- | --- |
| `actor` | `SocketGrantRecordV1.actor_id` | Always overwrite. | The server already uses it to key a live lease. |
| `connected_at_ms` | Hub capture once after admitted hello, retained for the socket lifetime | Always overwrite. | A client timestamp must not make an old/future session look new. |
| `user_id` | `SocketSubjectV1::Session.user_id`; `None` for Share | Always overwrite. | Never disclose/mint a user for a share capability. |
| `label` | Current `UserRecord.display_name` for a session; `None` for Share or unavailable user record | Always overwrite. | This is visible identity in both footers; client labels must not impersonate someone. A display lookup failure must not be replaced by the client value. |
| `role` | `SocketSubjectV1::Session.role.map(SpaceRole::as_str)` after the existing per-frame revalidation; `None` for Share | Always overwrite. | The document grant's role is current and revocation/downgrade is already fenced by `socket_live_authority`. |
| `color` | Hub's `acquire_color` result, retained in lease | Always overwrite. | A palette index is a server-assigned session resource. |
| `surface` | For a production plan-backed grant, `record.document_plan.surface.surface_id`; otherwise `None` | Always overwrite. | Do not promote the caller URL query to a roster authority. The plan route already binds this exact value; the test-only/non-plan fixture path gets no stamped surface. |
| `presence_pack`, `drag_ghost_json`, `interaction`, `views`, `ui` | Client ephemeral input | Preserve only after bounded exact decode. | These are app/artifact interaction data, not identity. They remain best-effort, non-durable, scope-confined, and limited by one peer byte budget. |

`views[].space` is an app-local view-space string, not the document execution surface; it must not be used to choose a roster scope. `presence_pack` remains opaque to Hub. Normalization must not parse/rewrite application payload or fabricate interaction state.

## Smallest Coherent Implementation Seam

### 1. Make shared decode safe for hostile network input

The current Rust decoder ignores trailing bytes (`decode_presence_peer`, `📡️wire/🦀️.rs:2005-2029`) and its `read_vec_presence_window_view` allocates from an attacker-controlled count (`:1767-1773`). `read_vec_str` has the same defect at `:3161-3167`, and interaction-domain decoding has the same count pattern at `:3192-3200`. The outer Hub 4,096-byte entry cap (`🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️.rs:1004-1006`) does not prevent `u64::MAX` from reaching `Vec::with_capacity`.

Have the protocol codec own one exact bounded decoder, shared by Rust and TypeScript, rather than adding a Hub parser:

```text
decode_presence_peer_bounded_exact(bytes, PresencePeerWireLimitsV1)
  1. reject bytes.len > maximumEntryBytes before any allocation;
  2. bound every string/blob by remaining bytes and its field cap;
  3. bound views, interaction domains, selected/hovered ids before allocation;
  4. accept only known flags, finite numeric view values, canonical varints, and exact EOF;
  5. return the existing PresencePeer only.
```

The TypeScript twin must impose the identical limits before `new Array`/loop allocation. This is an API hardening of the existing codec, not a second presence format. Remove/replace unbounded network ingress use rather than retaining an insecure compatibility decoder. Existing `encode_presence_peer` / `encodePresencePeer` remains the sole output codec.

### 2. Reconstruct at the Hub ownership boundary

Add one private Hub helper adjacent to `PresenceLeaseSlot`/`refresh_presence`, conceptually:

```text
normalize_document_presence_peer(raw, socket_grant, lease_admission) -> Result<Vec<u8>, Rejected>
```

`lease_admission` is not a new public identity type. It is the existing live connection values already in `handle_ws`: `actor`, `SocketSubjectV1`, Hub-captured connection time, server color, plan-bound surface, and the one `UserRecord.display_name` lookup. It can be stored directly as fields on the existing private `PresenceLeaseSlot` (add `connected_at_ms`, `label`, and the grant role); no external contract needs a new identity record.

Procedure:

1. After successful hello/plan validation, capture `connected_at_ms = now_ms()` once; look up the current session user once under the existing two-second directory bound; derive `label`/`user_id`/`role` from that admitted state. Do not retain email.
2. Before every refresh, run the existing `socket_live_authority` first. This preserves the current membership/session/plan fence.
3. Exact-bounded-decode the raw peer. On any failure, return `PresenceLeaseTransition::Rejected` without refreshing TTL, changing peer bytes, publishing, or mutating durable state.
4. Construct a fresh existing `protocol::PresencePeer`, copying only the five ephemeral fields and setting every authority field from the stored/live admission values. Encode it with the existing codec.
5. Reject if the normalized encoded bytes exceed `PRESENCE_ROSTER_MAXIMUM_ENTRY_BYTES`; otherwise give these bytes to the existing `refresh_presence` capacity/order/expiry path unchanged.

This preserves current actor sort, max-item/max-roster checks, stale-live protection, publication gate, and non-durability. It also turns the normalized `Vec<u8>` into canonical frame bytes: raw tails/noncanonical encodings cannot be retained or fanned out.

### 3. Consume normalized fields faithfully in hosts

- **Browser worker:** `🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:1632-1645` already decodes `ServerFrame::Presence` into the shared peer type and drops malformed entries. Its outbound `stampSession` at `:1035-1041,2519-2521` becomes defense-in-depth only; Hub is the authority.
- **React ShellHost:** it currently reduces every peer to only `{clientId, name}` (`.../🏛️ShellHost/🟦️.tsx:1771-1776,5862-5875`), losing normalized `userId`, `role`, `connectedAtMs`, `color`, and `surface`. Extend the local projection from the already-decoded event; never parse untrusted raw bytes in React. Filter against the opened document's **verified execution-target surface**, not an app alias, before feeding `PresenceBar`. Its public UI type already accepts these fields (`🧰️framework/🔨️modules/🖱️ui/🧱️elements/👥️PresenceBar/🟦️.tsx:33-42`).
- **Native WGPU:** the actor will receive normalized peers through existing `ArtifactEvent::Presence` (`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs:2416-2427`); no new transport is needed. `presence_peer_rows_for_surface` currently maps each peer but forces `color: None` at `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:473-498`, so change it to `peer.color`. It also checks only the shell's attached surface, never `peer.surface`; filter individual peers to `Some(target_surface)` before building rows. Its nearby comment claiming the wire peer has no surface is obsolete. The local heartbeat can retain its fake `current_shell_actor`/user label as input (`:3994-4025`), because the Hub overwrites them; it must not regain authority.

The existing Hub behavior is document-wide fanout (`bin.rs:1497-1502,1567-1571`). Surface-specific footer rendering must therefore filter the normalized per-peer `surface` after receipt; the Hub should not silently drop other valid document-surface peers from its shared roster.

## Schema-First Neutral Corpus

Add one neutral fixture beside the shared presence codec, for example:

```text
🧰️framework/🔨️modules/📡️replication/🧫️fixtures/👥️presence-normalization-v1/
  🧬️schema/🔣️.json
  🧪️fixture/🔣️.json
```

It is deliberately separate from the existing Hub lease fixture `🌎️hub/📦️packages/🦀️rust/🧪️fixtures/👥️presence-lease-v1/`, which correctly models lease transitions but calls each blob an opaque `peerTag` and cannot prove identity replacement.

Each fixture vector declares fixed wire limits, an admission tuple (`actor`, optional `userId`/`label`/`role`, `color`, optional plan surface, server `connectedAtMs`), `rawPeerHex`, and one result: exact `normalizedPeerHex` plus the semantic output record, or `rejected`. Rust and TypeScript must independently validate schema, decode the exact raw bytes with the bounded decoder, reconstruct the existing peer, and compare `encode_*` output byte-for-byte to `normalizedPeerHex`. The Hub native law reuses the same raw bytes through an admitted socket and asserts the received `ServerFrame::Presence` bytes.

Minimum vectors:

1. Session user sends a peer with every authority field forged and all five ephemeral fields populated: normalized bytes contain the admitted actor/user/display label/role/color/plan surface/server time, while every ephemeral field is byte/semantic-equal.
2. Share subject forges a user/author/label: normalized bytes contain its server actor/color/time and `None` for user/label/role; ephemerals survive.
3. Same client ephemeral value, two refreshes: canonical normalized bytes are equal, refreshes lease TTL but causes no duplicate publication (reuses existing lease vector semantics).
4. Reconnect with same authenticated user but new server actor/color/time: old live cannot overwrite or close the new normalized row (compose with existing reconnect law).
5. Unknown flag, truncated field, trailing byte, and noncanonical varint: rejected before slot/TTL/publication mutation.
6. `views`, interaction-domain, selected, or hovered count set to a huge varint in a short blob: rejected without capacity allocation/panic.
7. Raw input over 4,096 bytes and a small raw input that becomes over-limit after authoritative fields are injected: rejected without refreshing the lease.
8. Plan surface mismatch/missing plan surface: production plan-backed row has exactly the plan surface; non-plan fixture row has no surface and cannot appear in a surface-specific footer.

The corpus must assert `durableWrites: 0` and no canonical page/directory membership mutation. Presence normalization is ephemeral socket work, not an identity or directory-event feature.

## Existing Tests/Gates to Reuse

- Shared Rust all-fields codec tests: `📡️wire/🦀️.rs:2032-2085`; its fixture bytes are consumed independently by TypeScript at `🧰️framework/🔨️modules/📡️replication/🟦️.ts:1541-1572,1623-1630`.
- Native actor session-stamp law is useful only as a client defense-in-depth check: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs:5288-5317`.
- Hub lease native laws and registration: `presence_lease_reconnect_rejects_old_live_refresh_and_close`, `presence_lease_expires_server_clocked_visibility_without_socket_close`, `presence_lease_enforces_shared_roster_bounds_and_actor_order`, and `presence_lease_restart_is_empty_and_directory_presence_is_member_only` in `🌎️hub/📦️packages/🦀️rust/📜️script.ts:6925-6956`; Nx targets are `os-hub:presence-lease-{source,native,process}-check` in `📋️project.json:105-127`.
- WGPU already has surface-row behavior tests near `🐚️Shell/.../🦀️.rs:843-862`; amend them with mixed normalized surfaces and actual colors. It is not a native runtime acceptance result until run.
- Browser worker has the `stampSession` overwrite law at `backbone-worker.ts:2831-2854`; add a server-normalized roster projection law rather than treating client stamping as security.

## Nonclaims

- The proposal does not make document opening, browser component activation, WGPU mounting, or a two-user browser journey accepted.
- It does not turn presence data durable, expose user emails, give shares a member identity, or widen a socket grant.
- A valid client may still send arbitrary application ephemeral content inside the bounded fields. That is intentional application-level presence; identity, scope, surface, role, color, and connection age are no longer client authority.
