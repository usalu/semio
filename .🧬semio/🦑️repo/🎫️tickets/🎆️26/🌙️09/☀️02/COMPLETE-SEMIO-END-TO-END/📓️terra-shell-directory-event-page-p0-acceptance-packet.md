# 📇️ Home Directory Event-Page P0 Acceptance Packet

**Audit mode:** source-only, 2026-09-05. No build, native test, browser test, or process journey was run in this audit.

## Verdict

**RED: Home/Space directory ingestion is still raw-event relay, not a bounded durable page protocol.** The current REST route returns an unframed, optionally unauthenticated, byte-unbounded visible `Vec<DirectoryEvent>`; React sends that raw vector to a batch-only Home action; native WGPU uses the wrong argument key. Neither client has a sealed-page owner, durable raw scan frontier, terminal ACK, or session/generation-safe reset.

The smallest honest P0 is **Home only**: authenticated `/directory/event-page/v1` → sealed page owner → one retained Home config replacement → terminal receipt ACK. The directory WebSocket becomes a dirty wake only for that lane. It does not make Space Index, Studio, generic directory replay, public discovery, or arbitrary live rendering correct.

## Current evidence and decisive REDs

| Boundary | Current source evidence | Consequence |
|---|---|---|
| Durable source | `🌎️hub/📇️directory/🦀️.rs:347-364` has `DIRECTORY_EVENT_READ_MAX = 10_000`; `bounded_event_read` only bounds `since`/count (`:461-466`). | There is no page item/response-byte contract. |
| HTTP event read | `🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3807-3893` defines `GET /directory/events`; it calls `events_since(..., limit.unwrap_or(500))`, then visibility-filters raw rows. | No required authentication, session/generation binding, raw scan cursor, continuation, byte cap, hash, cancellation ownership, or post-read revalidation. |
| Privacy | `event_visible`/`visibility_filter_events` (`bin.rs:3835-3887`) correctly make raw events member-only, with user-local events matching `user_id`. | A page may legally have invisible sequence holes; a client may not infer a contiguous visible event stream. |
| Shared fold | `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🦀️.rs:47-55,72-82` stores one cursor and silently ignores only `seq <= cursor`. | It neither proves a raw scan is complete nor detects a gap. The page frontier cannot be represented by `DirectoryReadModel.cursor` alone without a validated page envelope. |
| Home persistence | `.../🏠️home/.../✏️editor/🎚️config/🦀️.rs:27-54` omits `DirectorySpace.documents` and uses `unwrap_or_default()` on malformed persisted JSON. | The current Home round trip loses document descriptors and turns corruption into an empty successful projection. |
| Home command | `.../🏠️home/.../🎮️commands/📇️fold-directory-events/🦀️.rs:15-26` parses a whole string with `unwrap_or_default()` and emits one config mutation per event. | Malformed input is acknowledged as an empty success; work, retention, and atomicity are unbounded by page semantics. |
| Home factory | `.../🏠️home/.../✏️editor/🦀️.rs:57-156,212-239,336-385,561-577` excludes `foldDirectoryEvents` from the retained factory and classifies it `BatchOnlyPendingRewrite`; config preparation rejects it. The existing caps are 8 KiB raw / 512-byte value / 4 KiB step. | Dispatching the old command through the retained machinery cannot work truthfully. |
| Browser transport | `🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:553-560,1370-1400` whitelists only old `events?since`, opens one stream, and forwards raw frames. `ShellHost/🟦️.tsx:1450-1472,4233-4267` forwards events into a void `foldDirectoryEvents` action. | Browser has no correlated page request, completion, ACK, or persisted/recovered page frontier. |
| Browser stream cursor | `🧰️framework/🛍️products/💻️os/🟦️.ts:4333-4403` advances `lastSeq` from both event and heartbeat `headSeq`. | A heartbeat can skip an unseen event; it must not be used as an ingestion cursor. |
| Native transport | `.../📇️directory/🔌️client/🦀️.rs:914-964` similarly advances on heartbeat. `.../🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:406-419,4211-4245` drains raw frames and serializes `{events: ...}`. | Home and Space parsers require `eventsJson`; WGPU currently applies the empty default. Native has neither page ownership nor durable ACK. |
| Native result bridge | `.../📺️renderer/.../🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:4965-5003,5096-5103,7309-7315` already owns a typed operation-result exchange and explicit ACK. | The old blueprint's “no WGPU ACK bridge” statement is stale. P0 must reuse this exchange rather than add another queue. |
| Native runner | `.../🐚️Shell/.../🦀️.rs:1337-1463,4078-4086,4201-4245` has a 256-message/32-per-turn stream runner, but its context has generation `0` and no deadline. | It can be a bounded dirty-hint source only; it is not a page operation owner. |
| Space Index | `.../🪐️space/.../🎮️commands/📇fold-directory-events/🦀️.rs:35-40` folds each raw batch from `DirectoryReadModel::default()`. | It cannot consume incremental, filtered pages. Keep it out of P0. |

The existing hub writer ordering remains useful evidence only: `🌎️hub/📦️packages/🦀️rust/📜️script.ts:4723-4754` registers the source/native ordered append→broadcast law. It does **not** prove event-page paging or client projection.

## P0 wire contract

Add one shared Rust/TypeScript/schema record next to the existing directory schema, with a fixture under the same domain:

```text
DirectoryEventPageV1 {
  schema: "semio.directory.event-page.v1",
  sessionBindingSha256: lowercase hex SHA-256[64],
  authorizationGeneration: u64-safe integer,
  afterSeqExclusive: u64-safe integer,
  throughSeqInclusive: u64-safe integer,
  hasMore: boolean,
  events: DirectoryEvent[],
  receiptSha256: lowercase hex SHA-256[64]
}
```

`receiptSha256` is SHA-256 of canonical UTF-8 JSON for exactly the preceding fields in declaration order, excluding itself. It is a **binding/integrity receipt, not a bearer capability**. The server derives `sessionBindingSha256` from a domain-separated digest of its authenticated session identifier; raw `sessionId` is never returned.

Rules:

1. The endpoint is authenticated. Anonymous calls return 401, not `[]`.
2. `afterSeqExclusive` is the client’s **durable raw scan frontier**. `throughSeqInclusive >= after`; every emitted event is strictly increasing and lies in `(after, through]`. Missing visible sequence numbers are legal because invisible raw records consumed the scan.
3. The server reads at most **128 raw rows** per page, filters each row with the current member-only boundary, and sets `through` to the final raw row examined. `hasMore` means a further raw scan may exist; it is not a client-visible sequence claim.
4. The canonical response is at most **64 KiB**. A persisted directory event must be capped at **48 KiB canonical event bytes** at the append boundary so one visible row always fits; an oversized existing row faults the page without advancing the frontier. Do not skip it.
5. A page uses one server scan result. It must not mix rows from a later scan while retaining an earlier `through`. An empty raw read with `head > after` is a server/storage inconsistency and faults/resyncs rather than returning a looping empty page.
6. The page’s 64 KiB cap excludes HTTP framing but includes all canonical JSON. Unknown fields, duplicate record keys, noncanonical SHA hex, C0 control characters in page text, non-safe integer sequences/generation, and trailing bytes are denial cases.

There is deliberately no `headSeq` in the persisted receipt. A live append can change a diagnostic head between a lost ACK and re-fetch. The immutable page receipt makes the exact `(binding, generation, after, through, events)` idempotent without falsely treating a changed head hint as a different projection.

## Cursor and ownership law

Persist a `HomeDirectoryProjectionStateV1` inside Home config, not a bare `DirectoryReadModel` JSON string:

```text
{ binding, authorizationGeneration, scanThrough, lastReceiptSha256, model }
```

`model.cursor` is set to the validated raw `scanThrough` only **after** applying the page’s visible rows in increasing order. This permits privacy holes while preventing an old event from reentering the projection. Its wire representation must preserve `DirectorySpace.documents`; decoding is fallible and a corrupt persisted projection is a terminal local recovery state, never `default()`.

The state is bounded. P0 must set and enforce a fixed serialized-projection cap before config preparation (recommended **256 KiB including base, forward, inverse, and receipt**). If a valid projected Home exceeds it, retain the old state, close the page owner, expose a localized `directory.projection-too-large` recovery state, and do not advance `scanThrough`. That is an intentional P0 capacity boundary; a future directory snapshot protocol is required for larger accounts.

Introduce one retained Home-only command, e.g. `applyDirectoryEventPage`, replacing—not wrapping—`foldDirectoryEvents`:

1. A `DirectoryEventPageOwner` receives the canonical HTTP bytes under a 64 KiB ceiling. It scans/validates before exposing `events`; cancel before/after each read and close consumes bytes incrementally. `RetainedToolWireInput` (`🧰️framework/🔨️modules/🎯️action-bus/🦀️.rs:109-205,628-706`) remains the bounded **dispatch** owner, not a reason to buffer an unbounded HTTP response.
2. The app command accepts a sealed page only. It checks binding/generation, receipt, `after == state.scanThrough`, range/order, size, and projection capacity before preparing exactly one config replacement.
3. If an identical receipt has `through == state.scanThrough`, it returns an idempotent terminal receipt with no second mutation. If the client lost its local scan checkpoint, the precondition result returns only the expected local frontier/binding to the same local shell; it does not expose event rows. The shell refetches from that frontier.
4. A changed binding or authorization generation may reset only with a valid page whose `after == 0`; the command folds it into an empty model. Until that succeeds, the surface must show no prior session’s directory rows. A new binding is never folded into the old user’s state.
5. The result token is ACKed only after the config commit is durable and the renderer has consumed the terminal result. Cancel, precondition denial, stale generation, malformed page, or close keeps the prior config/frontier unchanged and retires the page owner.

This creates a sealed scan cursor (`DirectoryEventPageOwner` while mutable/unacknowledged) and a visible cursor (`HomeDirectoryProjectionStateV1.scanThrough` after durable terminal ACK). Neither is the socket’s `lastSeq`.

## Dependency-ordered, non-overlapping slices

### A. Shared page and hub endpoint — directory/hub owner

- Add `DirectoryEventPageV1` in `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/{🦀️.rs,🟦️.ts,🔣️.json}` and a schema-first `event-page-v1` fixture. Keep `DirectoryEvent` as the payload type; do not introduce a redacted public-event type.
- Replace client use of `GET /directory/events` with authenticated `GET /directory/event-page/v1?after=<u64>`. Retire the old client-facing raw read rather than retaining a second ingestion protocol. Keep `events_since` as a server-internal primitive.
- In `🌎️hub/📦️packages/🦀️rust/🚀️bin.rs`, add an authenticated page handler beside `get_directory_events`, reuse `caller_active`, `event_visible`, and `visibility_filter_events`, then revalidate the same session/user/generation immediately before emitting the response. Add the router entry beside `/directory/events`.
- Extend the local relay whitelist in `🌎️hub/📦️packages/🦀️rust/📜️script.ts:335-342` to permit only the new route and one decimal `after`; remove the old event-read allowance once all clients move.
- Add the append-time 48 KiB canonical event validation at the shared durable directory writer—not only the HTTP handler—so a valid append cannot later make paging impossible.

### B. Page owner and Home transaction — OS directory/Home owner

- Add retained scan/close types in the shared directory client domain. Native must use incremental byte fuel and a terminal close; TypeScript must use an `AbortSignal`, exact `Uint8Array` cap, and one terminal release. No raw `DirectoryEvent[]` becomes visible before the receipt check.
- Replace `DirectoryReadModelWire` and `directory_to_json`/`directory_from_json` in `.../🏠️home/.../🎚️config/🦀️.rs:27-132` with the strict projection state preserving documents. Remove the `unwrap_or_default()` recovery path.
- Replace the raw command in `.../🎮️commands/📇️fold-directory-events/🦀️.rs` and update the Home command enum, `HOME_RETAINED_TOOL_IDS`, factory, contract, config preparation, manifest/action classification, generated command schema, proof macro, and retained-command-limit fixture together. The new config preparation must reserve the projection’s real bounded footprint; the present 512/4096-byte constants are insufficient.
- Produce exactly one config mutation per accepted page. It must emit no artifact mutation, child emission, remote directory command, or generic `Emit` side effect.

### C. Browser and WGPU adapters — Shell owner

- Browser: change `BackboneWorkerRequest/Response` in `🧰️framework/🛍️products/💻️os/🟦️.ts`, `backbone-worker.ts`, and `ShellHost/🟦️.tsx` to a correlated fetch → sealed-page → terminal-result → ACK protocol. The worker retains at most one page request and a dirty bit. A socket event only sets dirty; it never calls `dispatchDirectoryEventBatch` or advances a cursor. The shell gives the worker the returned accepted/precondition frontier only after the app result; a fresh worker learns a persisted frontier by one precondition response, not by replaying raw history.
- Native: replace `pump_directory_events` raw batch dispatch in `.../🐚️Shell/.../🦀️.rs` with one page operation using a real generation/deadline/cancellation child. Reuse `MountedTypedOperationResultExchange`; ACK only the accepted/precondition terminal. The stream runner’s 256/32 limits remain wake backpressure, not page limits.
- Both clients must stop advancing `DirectoryClient.stream` from heartbeat `headSeq`. Either use a new no-replay authenticated dirty subscription, or treat the existing member-filtered stream strictly as a wake source and request pages from `scanThrough`; no raw stream body is persisted by this P0.
- Native’s `{events: ...}` action shape and React’s void raw action disappear with the old command. No adapter may retain the old `eventsJson` bridge as fallback.

## Acceptance matrix

### Language-neutral fixture/oracle

Register a single event-page fixture with Rust and TypeScript schemas plus a Bun/AJV/`TextEncoder` + Node `crypto.createHash("sha256")` oracle. It must construct canonical bytes independently of the Rust writer and cover:

- visible events with invisible raw holes; 128 invisible raw rows that legitimately commit an empty page;
- exact 128/129 raw-row continuation; 64 KiB edge, 48 KiB event edge, and an oversize event that faults without `through` movement;
- duplicate exact receipt; duplicate fetch with changed arrival timing but the same immutable receipt; reordered, repeated, out-of-range, forged receipt, unsafe integer, unknown field, C0, trailing-byte, and malformed payload denial;
- session A → session B and authorization-generation change: only validated `after:0` reset may replace the state;
- cancel before response, during scan, after seal, after prepare, and after commit/before ACK; stale request/result IDs; close at every nonterminal phase;
- source `DocumentAnnounced` then config encode/decode: document descriptor survives; malformed local state faults rather than silently rendering empty.

### Native laws

Add an isolated hub `directory-event-page-check` and `directory-event-page-native-check`, analogous to `os-hub:directory-ordered-publication-check` / `...-native-check` in `🌎️hub/📦️packages/🦀️rust/📋️project.json:103-116`; register the launch entries in `.vscode/🧩️launch.seed.jsonc`, then generate `.vscode/launch.json` (never hand-edit generated launch JSON).

Required executable laws:

1. Hub authenticated page scan consumes raw invisible rows, exposes only allowed rows, preserves `(after, through)` and rejects stale session/generation after the read.
2. Home retained operation at grants 1, 64, and full cap reaches one prepared config commit → terminal receipt → ACK → terminal-empty; every cancellation/denial retains the old projection and closes all page/dispatch owners.
3. WGPU fake HTTP/socket runner proves wake → page request → typed result exchange → ACK; heartbeat alone does not advance the visible frontier; 4401 clears/hides binding state and 1013 reconnects/resyncs without duplicate projection.
4. React worker/host integration proves a correlated page reaches the Home row in EN and DE accessibility trees, rejects stale result IDs, and does not call the legacy action.

### Process law

After the native laws are green, add one bounded two-session loopback process test: session A receives a Home page, is revoked or has its generation advanced while a page is pending, gets 401/4401 and no committed A page after revocation; session B starts at zero, receives only B-visible projection, reconnects after a 1013 wake, and ends with exactly one persistent page receipt per accepted raw range. This is not satisfied by the existing ordered-publication, admin, or socket-revocation fixture alone.

## Explicit nonclaims

- No public/raw event stream, public-space telemetry, Space Index incremental fold, Studio projection, offline multi-hour sync, CRDT, or generic directory snapshot is made correct.
- Existing server append→broadcast and socket membership checks remain valuable prerequisites but do not prove page ingestion.
- No current source/native/process test proves this P0; all listed commands/laws are required registrations, not results.

## Earlier-blueprint corrections

`📓️terra-shell-directory-event-page-retained-owner-blueprint.md` is superseded for implementation planning in these respects:

1. The proposed `sessionId` field is replaced by a non-capability session-binding digest; raw session identity must not be sent to the client.
2. A `headSeq`-bound receipt is not stable across lost ACK/re-fetch. The durable receipt covers the immutable continuation page; a socket dirty signal closes the live race.
3. WGPU does have a typed result exchange/ACK facility. The missing work is its directory-page caller, page owner, and terminal wiring.
4. Home is not merely missing a handler: its factory/catalog/config preparation explicitly rejects the old command, and its JSON wire currently loses documents.
5. P0 is Home-only. Current Space Index rebuilds from an empty model, so including it would create a false incremental-sync claim.
6. Stream heartbeat/event sequence is not a valid persisted page cursor. It is a wake hint only.

