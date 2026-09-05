# Hub Presence Normalization Native Preflight — Current Audit

Audit date: 2026-09-05. Read-only source inspection of the current Hub additions and the six exact native-law selectors. No Cargo build, native law, or socket process was run.

## Result

I found **no current static signature/type mismatch** in the added normalization path or the six selected law names. The `PresenceLeaseSlot` fields used to reconstruct the peer match `protocol::PresencePeer` exactly, `DirectoryPresenceActor` receives a concrete `String` only after the optional plan surface is present, and every selector in the script has a matching native function.

The prior route-surface split is not present in current source: the slot has only `document_surface`, the normalized peer takes that same field, and the directory projection is made only when that field is `Some` ([`🚀️bin.rs:429-440,1547-1574,1591-1607`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:429>)). The independent oracle now uses UInt64 LEB128 and includes `4_294_967_296` and the maximum JavaScript safe integer in its fixture, closing the earlier U32 timestamp disagreement ([`📜️script.ts:7187-7249`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:7187>), [fixture](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🧪️fixtures/🪪️presence-normalization-v1/🧪️fixture/🔣️.json:279>)).

The selected laws are registered correctly with the current source names and `--features sqlite` target:

1. `presence_normalization_matches_neutral_authority_and_no_effect_rejections`
2. `presence_normalization_socket_overwrites_identity_and_rejects_without_refresh`
3. `presence_lease_reconnect_rejects_old_live_refresh_and_close`
4. `presence_lease_expires_server_clocked_visibility_without_socket_close`
5. `presence_lease_enforces_shared_roster_bounds_and_actor_order`
6. `presence_lease_restart_is_empty_and_directory_presence_is_member_only`

See [`📜️script.ts:7262-7283`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:7262>) and native definitions at [`🚀️bin.rs:9755-10023`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:9755>).

## Concrete preflight findings

### P1 — expiry law does not synchronize the negative assertion with a server tick

`presence_lease_expires_server_clocked_visibility_without_socket_close` advances its test clock to `TTL - 1`, sleeps 1.1 seconds, and then infers that the server evaluated the early deadline merely because `socket.next()` remains quiet for 100 ms ([`🚀️bin.rs:9938-9962`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:9938>)). `handle_ws` owns an ordinary one-second `tokio::time::interval` and has no test barrier around `expire_presence_for_live` ([`🚀️bin.rs:3598-3615`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3598>)). Under scheduler delay, its first expiry evaluation can occur only after the test advances the controllable clock to the deadline. The law would still pass if `now >= expires_at` were mistakenly changed to an early predicate, because it has not forced an evaluation at `TTL - 1`.

Minimal exact fix: add a test-only `presence_tick_admitted`/`presence_tick_release` pair immediately before the loop's `expire_presence_for_live` call. The law sets the clock to `TTL - 1`, waits for admission, releases exactly one tick, then proves no frame/snapshot mutation; it repeats at `TTL`, releases exactly one tick, and consumes the one empty roster. Do not replace the live socket law with a direct helper call—the production loop is the boundary being claimed.

### P2 — two selected laws exercise raw helper storage rather than the normalization ingress

The roster-bound law fills slots through `refresh_presence` with arbitrary `Vec<u8>` (including all-zero 4-KiB peers), and the restart law publishes `b"opaque"` through the same raw helper ([`🚀️bin.rs:9965-10022`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:9965>)). That is a valid narrow storage/aggregate-bound test, but it is not evidence that the selected “presence normalization” gate applies its bounds to canonical, admitted peer bytes. The production ingress is `refresh_document_presence`, whose decode/reconstruct/encode/redecode completes before it calls that helper ([`🚀️bin.rs:1591-1607`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:1591>)).

Keep the raw-helper tests for capacity arithmetic, but add one fixture-backed normalization route row at the 64th/65th actor and aggregate-byte boundary: install actual slots, call `refresh_document_presence` with valid canonical fixture peers, and assert rejected row leaves deadline/snapshot/fanout unchanged. This is a coverage correction, not a production signature change.

## Test-order checks that are currently sound

- The normalization socket law sends hostile frames followed by a `PreviewPublish` FIFO marker before it checks unchanged lease/roster state; a serial WebSocket receive loop processes the marker only after prior hostile frames ([`🚀️bin.rs:9845-9866,3622-3646`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:9845>)). This replaces the earlier unsynchronized quiet-window pattern.
- The reconnect law is now plan-backed, observes old-row removal through both the old socket and a distinct observer, fences stale refresh with a preview marker, waits for the old live lease to unregister, and then proves the replacement can refresh ([`🚀️bin.rs:9872-9935`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:9872>)). Its plan helper issues/exchanges real plan-derived socket grants ([`🚀️bin.rs:7984-8000`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:7984>)).
- `PresenceLeaseSlot` mutators all compare `socket_live_id` under the publication lock, so stale refresh/close cannot alter a replacement ([`🚀️bin.rs:1578-1681`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:1578>)).

## Nonclaims

Static inspection is not compilation or runtime proof. This review does not claim the six laws pass, nor execution-target retrieval, browser projection, renderer behavior, or cross-user browser presence visibility.
