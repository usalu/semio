# WGPU Directory Bootstrap Retained-Home ACK — P0 Audit

Status: current-source audit, 2026-09-05. No product source was modified and no executable gate was run for this audit.

## Decision

**RED — the native WGPU shell is not a twin of the browser `DirectoryEventPageBootstrapV1` owner.** The hub, cross-language page contract, canonical Rust client, typed transition machine, acknowledged-only stream mode, and Home reducer are present. The WGPU shell mounts none of that chain: on identity completion it dials the legacy observed-frontier stream at `0`, and its frame pump dispatches raw events to whichever app is visible.

The smallest honest next packet is one native-shell integration packet, not new hub/client/schema work. It must add one shell-owned retained Home projection and wire this already-shipped Rust client machine through `fetch → Home publication witness → exact ACK → acknowledged live stream`. It must remove the raw global fold as the source of Home authority.

## Source-validated inventory

| Status | Seam | Evidence |
| --- | --- | --- |
| GREEN | Authenticated event-page source exists. | `get_directory_event_page_v1` admits canonical `after`, authenticates/revalidates the caller, bounds/seals the page and is registered at `/directory/event-page/v1`: `🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3984-4001`, `4072-4188`, `5736-5747`. |
| GREEN | The protocol is bounded and canonical at the native client boundary. | `DirectoryClient::event_page` checks cancellation before/after I/O, uses the raw UTF-8 body, caps it at `DIRECTORY_EVENT_PAGE_MAX_BYTES`, parses canonical JSON, and verifies the returned `after`: `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs:805-840`. |
| GREEN | A private page owner has exactly the fields needed for a non-secret acknowledgement. | `CanonicalDirectoryEventPageV1` retains canonical bytes plus binding, authorization generation, frontiers, `has_more`, and receipt; `DirectoryEventPageAckV1` has the five acknowledgement values: `…/🔌️client/🦀️.rs:226-267`. |
| GREEN | The shared native state machine already enforces ordering. | `DirectoryEventPageBootstrapV1::{present,acknowledge,reject,wake,close}` permits one pending page, compares epoch/receipt/binding/generation/through exactly, returns `Fetch` or `Live`, retries at the committed cursor, and resets to `0` only on rebootstrap: `…/🔌️client/🦀️.rs:284-360`. |
| GREEN | A live stream can be made acknowledgement-owned. | `stream_acknowledged` builds a stream with observed-frontier tracking disabled; only `DirectoryStream::acknowledge` may advance `since`: `…/🔌️client/🦀️.rs:852-868`, `1108-1139`. |
| GREEN | Home accepts only the typed sealed page route and persists its authority. | `applyDirectoryEventPage` takes only `pageJson`: `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:436-442`. The retained factory declares it Config-only: `…/🦀️.rs:59-85`, `120-160`. The reducer verifies receipt/authority/frontier, permits authority replacement only at zero, and atomically stores projection, binding, generation, and receipt: `…/🎚️config/🦀️.rs:125-155`. |
| GREEN | The browser worker contains the intended *transport* twin. | It owns the same machine and a cancellable owner, fetches a canonical page, waits for a message ACK, retries rejected/transport pages, then uses `streamAcknowledged`; event/heartbeat/rebootstrap merely wake a page refetch: `🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:1385-1569`, `1614-1624`. Bootstrap messages are explicitly kept on the TypeScript owner: `…/🧵️backbone-worker.ts:81-85`. |
| RED | Neither visible browser ShellHost nor WGPU currently mounts the worker/client owner through a retained Home ACK. | Browser identity still posts legacy `directory-open` with `since: 0`, and its message handler folds raw events: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:1450-1455`, `1677-1683`, `4281-4294`. Native has the equivalent legacy path below. Thus the worker class is a reusable reference, not a completed browser end-to-end integration. |

## Exact native RED seams

### 1. The shell has no retained Home projection owner

`ShellState` exposes only `session: Option<ActiveSession>` (`…/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:1647-1652`); `ActiveSession` is a plugin id, instance id, app definition, and view state (`1172-1178`). Identity fields contain just `directory_client`, one `directory_stream`, a transport, and one cancellation root (`1783-1802`). There is no field retaining a Home instance/controller, a bootstrap epoch, a pending canonical page, or a page-operation handle.

This is material, rather than a naming gap. Boot creates the landing app as the visible session (`2375-2397`), while managed and ordinary app switches replace the sole `session` with another instance (`4433-4471`, `4478-4501`). Generic `dispatch_action` derives its invocation address and calls `program.handle_action` with that **visible session's** instance (`3777-3784`, `3877-3898`). It cannot safely invoke Home after Studio or Space Index is visible, and `Result<(), String>` discards the invocation result after applying generic mutations (`3983-3984`).

The retained recipient therefore must be an explicit shell lifetime owner. It must not be `session`, a selected app's controller, or a global cache. It must be retired on both `prepare_hot_reload` (which currently destroys only the active session: `2361-2368`) and `Drop` (which currently cancels the directory root and runner only: `1872-1887`).

### 2. Identity completion bypasses the page/ACK protocol

On successful `poll_identity_bootstrap`, WGPU constructs the authenticated client and immediately calls `open_directory_stream` unless offline (`4173-4187`). That function creates `client.stream(0)` (`4202-4210`), the observed-frontier mode — not `client.stream_acknowledged(through)`.

This skips every required boundary: page GET, canonical-byte retention, `applyDirectoryEventPage`, Config publication, exact acknowledgement, and `throughSeqInclusive` hand-off. It also makes a reconnect cursor advance from raw observed events or heartbeats because this is the legacy stream mode.

### 3. The current pump makes raw websocket data authoritative

`pump_directory_events` drains `ShellDirectoryRunner`, collects `Event`s, discards `Heartbeat`, cancels/restarts the stream on `RebootstrapRequired`, and calls `dispatch_directory_event_batch` (`4212-4245`). The latter uses the currently visible session and dispatches `foldDirectoryEvents` (`4138-4149`). This contradicts page ownership in three ways:

1. a global event is treated as projection data rather than a wakeup;
2. it is applied to the active app instead of retained Home; and
3. rebootstrap restarts the same legacy stream instead of closing it, clearing the current page owner, and fetching with `after=0`.

There is also an existing payload defect that makes this raw path unsuitable as a stopgap. `fold_directory_events_action` serializes `{ events: [...] }` (`406-419`), but Home's action adapter reads only string field `eventsJson`, falling back to `[]` (`✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:480-482`). The typed page action intentionally uses the distinct canonical `pageJson` field (`440-442`); do not repair this legacy batch shape in the page packet.

### 4. Cancellation and retry primitives exist but are not connected to a page operation

`directory_ctx` creates a child of shell-wide `directory_cancel` (`4079-4087`). `ShellPoolFuture` is the retained worker-pool future primitive, and `ShellDirectoryRunner::cancel` closes the stream/context (`1327-1463`). The browser reference uses an `AbortController`, pending retry timer, and close path (`1444-1451`, `1475-1513`, `1614-1624`). WGPU has no equivalent finite page task/receiver, no retry timer, no epoch invalidation, and no terminal page-owner cleanup.

### 5. Existing checks do not prove the WGPU mounting seam

The shared client law covers canonical bytes, forged ACK rejection, retry cursor, cancel-before-I/O, size/canonical failures, wake coalescing, and acknowledged stream advancement (`…/🔌️client/🦀️.rs:1942-2037`). The language-neutral trace likewise proves two-page ACK ordering and a final socket cursor (`🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/🧭️event-page-bootstrap-v1.json:1-25`). However, its source oracle only asserts that the browser worker contains marker strings (`🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📜️script.ts:143-183`). The registered WGPU project has `test`/`test-native` but no directory-bootstrap target (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📜️script.ts:354-378`, `450-472`; `📋️project.json:96-143`). No existing gate can establish this integration.

## Required retained-Home contract

Introduce one private native `DirectoryHomeProjection` owned by `ShellState`. It must contain, at minimum:

```text
Home instance identity: pluginId, instanceId, app/controller address
Bootstrap identity: monotonic bootstrapEpoch
Protocol owner: DirectoryEventPageBootstrapV1
I/O ownership: one page task/receiver and its child CancelToken
Publication ownership: at most one pending canonical page / retained invocation
Live ownership: one acknowledged ShellDirectoryRunner and dirty latch
```

The Home instance must be created from the declared landing/Home app once and be retained when its UI is not visible. If landing is visible, the display session may reference that same instance; it must never create an independently diverging second Home projection. When another app becomes visible, retain the projection record and continue to address its stored `instanceId`, never the visible `session`. Hot reload, identity replacement, explicit shell close, and drop must cancel/close the page task and runner, clear canonical-page bytes, then destroy the retained Home instance exactly once.

The one allowable ACK is a typed witness constructed only after the dedicated Home invocation has reached terminal Config publication and the persisted/returned configuration proves all five values still match the page:

```text
{ bootstrapEpoch,
  sessionBindingSha256,
  authorizationGeneration,
  throughSeqInclusive,
  receiptSha256 }
```

Those are exactly the fields compared by `DirectoryEventPageBootstrapV1::acknowledge` (`…/🔌️client/🦀️.rs:318-334`). A successful `handle_action` call, a scheduled action, or a generic `dispatch_action` `Ok(())` is not an ACK. Add a narrow native `apply_directory_event_page_and_ack` path that invokes the stored Home instance, drives its retained Config publication to terminality, re-reads/returns the matching Config witness, and only then calls `bootstrap.acknowledge`. Do not broaden the generic action API or synthesize a second Home parser.

## Required state and lifecycle parity

The page controller must preserve this transition system; it mirrors the source machine and browser worker, while remaining native-shell owned:

```text
Bootstrap(after=0 or ACKed through)
  -- worker-pool GET event_page(after) --> AwaitingHomeAck(page)
  -- exact published Home witness --------> Bootstrap(next through) | Live(through)
  -- page/action retryable failure --------> Bootstrap(same ACKed through, bounded jitter)
  -- reject/cancel/identity replacement ----> Closed or Bootstrap(same ACKed through)

Live(ACKed through)
  -- event or heartbeat -------------------> close stream; Bootstrap(ACKed through)
  -- RebootstrapRequired / binding change --> close stream; Bootstrap(0, new epoch)
  -- close/drop ----------------------------> Closed
```

Rules that make the above executable rather than merely stateful:

- Fetch with `DirectoryClient::event_page(&child_ctx, bootstrap.after())` on `ShellPoolFuture`/the existing shared worker pool. Retain the original canonical JSON only while the page is awaiting Home.
- Start `ShellDirectoryRunner` only from the `Live { since }` transition, using `client.stream_acknowledged(since)`. Never open `client.stream(0)` in the global lane.
- On a post-live page ACK, either create a fresh acknowledged stream at `through` or call its explicit `acknowledge(through)` before any reconnect. Events and heartbeat heads never advance the durable cursor.
- Coalesce any number of live events/heartbeats into one dirty wakeup. Do not call `foldDirectoryEvents` from the global lane. The next page starts at the last Home-ACKed cursor, so duplicate wakeups are harmless and gaps remain replayable.
- Retry only transport/time-limit failures with bounded jitter. A page parse, receipt, or exact-ACK mismatch is terminal for that epoch; a Home rejection returns the still-acknowledged `after` only if it matches the still-owned pending receipt. A 401 follows identity revalidation and begins a fresh authenticated generation; it must not locally clear Home then open a socket.
- Cancellation is transitive: cancel page context/receiver, abort any in-flight Home publication, close the runner, drop pending canonical bytes, and suppress all late sender/receiver outcomes by epoch and owner identity. A cancellation never emits an ACK.
- A binding or authorization-generation change cannot be resumed from a nonzero page: Home explicitly rejects it (`…/🎚️config/🦀️.rs:129-143`). Treat it as close-stream, increment epoch, fetch `after=0`, then reacquire live only after the new page ACK.

## Smallest executable implementation packet

1. **Native owner and lifecycle.** In `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`, add the private retained Home projection plus bootstrap state beside `directory_client`/`directory_stream`. Initialize it from the declared landing app in `boot`; preserve it through `switch_to_managed_app` and `switch_to_app`; retire it in `prepare_hot_reload` and `Drop`.
2. **Narrow publication witness.** Factor the current action invocation seam around `dispatch_action` (`3877-3984`) so a stored Home instance can be invoked without borrowing/rewriting the visible session. Add only `apply_directory_event_page_and_ack`; it must use `applyDirectoryEventPage` / `{ pageJson }`, terminal Config publication, and the five-field witness above.
3. **Asynchronous bootstrap pump.** Replace `open_directory_stream` / the directory portion of `pump_directory_events` (`4202-4245`) with a single finite controller. Reuse `directory_ctx`, `ShellPoolFuture`, `ShellDirectoryRunner::cancel`, and the shared `DirectoryEventPageBootstrapV1`; add no runtime or HTTP implementation. Delete `dispatch_directory_event_batch` from the global bootstrap/live path.
4. **Typed failure/cancellation/rebootstrap.** Add one epoch per active native bootstrap, one cancellable page receiver/task, one bounded retry timer, and explicit terminal cleanup. `RebootstrapRequired`, identity replacement, and Home authority mismatch must start a fresh epoch at zero. Event/heartbeat are only dirty wakeups at the prior ACKed frontier.
5. **Register proof, then run it.** Extend the owning WGPU `📜️script.ts` and `📋️project.json` with a `directory-event-page-bootstrap` check/native check that dispatches exactly named Rust laws; add the ordered launch entries to `.vscode/🧩️launch.seed.jsonc` and regenerate `launch.json`. Do not overload the existing OS-kernel `directory-event-page-bootstrap-check`: it does not touch WGPU.

## Acceptance laws for that packet

Use the existing JSON trace schema as the language-neutral fixture owner, extend it for cancellation/retry/rebootstrap, validate it with Bun/AJV, and run an independent Rust interpreter/controller law. The WGPU integration law must use a fake page transport and fake retained-Home bridge that records input/Config witness; merely counting callbacks is insufficient.

| Law | Required observation |
| --- | --- |
| Two pages and exact ACK | Page 2 is not requested before page 1's terminal Config witness. Every one of receipt, binding, generation, through, and epoch forged independently leaves `after` unchanged and opens no socket. |
| Retained Home lifetime | Switch Home → Studio/Space Index → Home while bootstrap is active; the action targets the stored Home instance, not the visible session; exactly one Home owner survives and is destroyed once at close/hot reload. |
| Canonical transport | Client receives original canonical bytes, rejects noncanonical/over-64-KiB/frontier-mismatched pages, and does no I/O when its child cancel token is already cancelled. Reuse/extend the existing client law rather than duplicating parsers. |
| Publication before ACK | Delayed/faulting Home publication produces no ACK and no page advance. Exact replay is acknowledged only after the actual persisted/returned config witness agrees; a stale Config frontier is a terminal mismatch. |
| Retry and cancellation | Transport failure and matching Home rejection retry the identical committed `after`; close, drop, hot reload, and identity replacement cancel the page task, clear pending canonical data, close the stream, and ignore a late result. |
| Live wakeup and reconnect | Final ACK dials exactly `since=through`; events and heartbeat only cause one dirty refetch; reconnect before a new ACK uses the old committed cursor; no global `foldDirectoryEvents` is invoked. |
| Rebootstrap | A `RebootstrapRequired` message, binding change, or generation mismatch closes live, increments epoch, and fetches `after=0`. A stale epoch ACK cannot revive the retired stream. |
| Process proof | With two authenticated sessions, append an event after final-page response but before socket dial. The resumed page/socket path sees it once, without raw session/bearer in page, ACK, fixture, or log output. |

The existing shared client assertions are necessary but do not satisfy these mounting laws. This packet is GREEN only when the WGPU-specific native law and the two-session process law execute, and the registered source/fixture/AJV checks agree.

## Handoff

The hub route and cross-language client are ready dependencies. The blocker is wholly in the native shell's ownership and invocation seams: no retained Home recipient, no Config-publication ACK, and a still-live legacy `stream(0)`/raw-fold loop. Implement the packet above before claiming browser/native directory bootstrap parity.
