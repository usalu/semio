# Shell Directory Event-Page Owner: P0 Blueprint

Status: current-source audit, 2026-09-05. No builds or runtime commands were run for this report.

## Decision

The first honest slice is **Home-only, authenticated directory projection ingestion**. It must replace the raw `foldDirectoryEvents` action with one bounded, retained `foldDirectoryEventPage` operation, server-issued pages, and a shell-owned pull/resync loop. Do not extend the existing Space Index fold in this packet: it intentionally rebuilds from `DirectoryReadModel::default()` and explicitly requires the complete history, so a delta page would regress its member/visibility projection.

The decisive current RED is not merely a missing retry. Both shells turn every stream event into an unbounded JSON action and do not receive an accepted cursor. The native WGPU shell additionally sends `{ events: [...] }`, while the two current app decoders accept only `eventsJson`, so it folds an empty batch. The React spelling was repaired to `eventsJson`, but it still has no sequence/page authority, bounds, retained lifetime, or resync acknowledgement.

## Current route and proven boundaries

| Boundary | Current source | Current behavior / defect |
| --- | --- | --- |
| Durable sequence authority | [directory core](../../../../../../../../../🌎️hub/📇️directory/🦀️.rs#L1532) | The writer lock plus backend transaction makes durable sequences dense and ordered; `events_since(0)` is asserted dense through `head_seq` in the core test around lines 3550–3564. `DIRECTORY_EVENT_READ_MAX` is 10,000, with no response-byte ceiling. |
| REST event read | [hub route](../../../../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs#L3765) | `GET /directory/events` returns a bare `Vec<DirectoryEvent>` after visibility filtering. It has neither the raw scan cursor, head, page hash, session generation, nor a continuation truth. It cannot distinguish a legitimate filtered hole from a loss. |
| Socket replay/live | [hub socket handler](../../../../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs#L3912) | The hub subscribes before replay and filters each event under a live socket grant. Lag closes with `1013` / rebootstrap. Individual stream events are not a page and carry no accepted scan frontier. |
| TS socket cursor | [DirectoryClient.stream](../../../../../../../../../🧰️framework/🛍️products/💻️os/🟦️.ts#L4333) | It advances `lastSeq` on both an event and a global `heartbeat.headSeq` (lines 4383–4389). A heartbeat is not proof that every member-visible event through that global head reached the app projection. Malformed stream frames are dropped without resync. |
| Native socket cursor | [DirectoryStream](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs#L853) | The same issue exists: `track` advances `since` from `Heartbeat.head_seq` at lines 958–963. `ShellDirectoryRunner` then queues raw stream messages (max 256) and drains them into a `Vec` (WGPU shell lines 1337–1462). |
| Browser shell dispatch | [ShellHost directory lane](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx#L4233) | `directory-message` is forwarded straight to a void `onAction` as `foldDirectoryEvents { eventsJson: JSON.stringify(events) }`. No completion result, cursor, cancellation, or error is retained. The worker always opens from `since: 0` once (lines 1661–1667). |
| Native shell dispatch | [WGPU helper](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs#L406) and [pump](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs#L4211) | The helper emits `{ events: [...] }`; Home and Space Index only read `eventsJson`. `pump_directory_events` drops heartbeat/connection/presence and reports dispatch errors to stderr, then has no retry or receipt. |
| Home fold | [Home command](../../../../../../../../../✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📇️fold-directory-events/🦀️.rs#L13) | It `unwrap_or_default`s malformed JSON and emits one config mutation per event. The config fold is idempotent only for `seq <= DirectoryReadModel.cursor`; it does not reject a gap, bind a page frontier, retain byte owners, or atomically checkpoint a page. |
| Home persisted projection | [Home config](../../../../../../../../../✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs#L17) | The JSON decoder falls back to an empty model on malformed config. Its wire omits `DirectorySpace.documents`, so a document-announcement projection is lost across config serialization. The only persisted cursor is the last **visible event** cursor, not a scan frontier. |
| Space Index fold | [Space Index command](../../../../../../../../../✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📇fold-directory-events/🦀️.rs#L1) | Its own contract says full history is required. It reconstructs from empty on every action, so it is explicitly outside a delta-page P0. |
| Factory and retained ingress | [Home factory](../../../../../../../../../✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs#L57) and [ActionBus](../../../../../../../../../🧰️framework/🔨️modules/🎯️action-bus/🦀️.rs#L109) | Existing `RetainedToolWireInput` reserves exact pages and has bounded close. The current Home factory excludes `foldDirectoryEvents`, has an 8 KiB raw cap, and the generic retained command copies wire pages to a `Vec<u8>`; it is not a safe event-page decoder. |
| Runtime progression | [plugin maintenance](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs#L24246) | Worker stages are driven through `maintenance_step`; `plugin_continue_typed_operations` only advances publication/result output. A directory implementation cannot poll publication alone and call that liveness. |

The existing directory core fold at [lines 72–166](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🦀️.rs#L72) is reusable for valid visible events, but it must be wrapped by page-frontier validation. It intentionally ignores old/out-of-order events; it is not gap detection.

## P0 wire contract and authority

Add one shared Rust/TypeScript/JSON-schema contract beside the existing directory schema, without accepting the bare vector route as an alias:

```text
DirectoryEventPageV1 {
  schema: "semio.directory.event-page/v1",
  sessionId: bounded opaque server id,
  authorizationGeneration: safe u64,
  afterSeqExclusive: safe u64,
  throughSeqInclusive: safe u64,
  headSeq: safe u64,
  hasMore: bool,
  events: [DirectoryEvent],
  pageSha256: lower hex SHA-256 of the canonical frame without pageSha256
}
```

Use a fixed P0 policy of at most **128 raw rows and 64 KiB canonical UTF-8 frame bytes**. `hasMore` is true whenever the server did not scan through `headSeq`; it is not inferred from `events.length`. A page may contain zero visible events and may have non-contiguous event sequences because privacy filtering is current-membership based ([`event_visible`](../../../../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs#L3711)). Thus no client may require `event.seq == prior + 1`.

The server must instead:

1. Authenticate the bearer session and take its exact `(session_id, authorization_generation)` before reading. Recheck it after the bounded durable read and before serializing the page; revoked/stale sessions receive no page.
2. Read raw durable rows after `afterSeqExclusive` in ascending order, up to 128, and evaluate the current visibility policy per row. It advances `throughSeqInclusive` over invisible rows too. If adding the next visible row would exceed 64 KiB, it stops **before** that raw row; it never claims to have scanned it.
3. Set `headSeq` from the same bounded read epoch (or reread after the page and require `headSeq >= through`). A byte-cap/row-cap continuation must be explicit. The raw event log's global density remains server authority; the client only owns the sealed scan interval.
4. Canonically encode, hash, and return the complete page. The process must not return a bare event list from the new route. The old `/directory/events` may be removed in the coordinated change rather than supported as a legacy ingestion path.

The page owner stores `scan_cursor` separately from `DirectoryReadModel.cursor`. On a normal page its required `after == scan_cursor`; it validates bounds/schema/canonical hash, strictly increasing unique event ids/sequences within `(after, through]`, and `through <= head`. It folds only `events`, then commits one local config snapshot that records `(model, scan_cursor = through, last_page_hash, session_id, authorization_generation)`. A duplicate with `(after, through) == (previous_after, scan_cursor)` is a no-op only if the hash is identical. Any stale/out-of-order header, hash mismatch, malformed body, wrong session/generation, unknown schema, non-canonical JSON, row/byte overflow, or frontier mismatch preserves the prior persisted state, closes the retained page, and enters `ResyncRequired`.

Do not use `DirectoryReadModel.cursor` as the HTTP/socket resume cursor: visible-filter holes make it insufficient. Do not advance it from a heartbeat. The socket becomes a wake/dirty hint only; page pull is the sole projection authority.

## Retained page owner and lifecycle

Create a dedicated `HomeDirectoryEventPageJob` and factory rather than widening `ArtifactRetainedCommandJob`.

* Reuse `ActionBus::begin_exact_wire` / `RetainedToolWireInput` ([action bus lines 628–706](../../../../../../../../../🧰️framework/🔨️modules/🎯️action-bus/🦀️.rs#L628)) for exact page reservation, page ingress, stale factory admission rejection, and bounded close.
* Give the new factory only `foldDirectoryEventPage`, a new payload schema id (for example `space.home.directory-event-page.v1`), exact 64 KiB input / 128 decoded-record / 1 config-output contract, and only the `Config` publication lane. Register it beside, not inside, `HomeRetainedCommandJobFactory`; its `Job` is a concrete page-owner job, so it does not force the generic command's whole-`Vec<u8>` decoder on this input.
* The job owns: sealed wire input; a fixed header decoder; one bounded event decode/fold scratch; validated pre-page `DirectoryProjectionStateV1`; the next state; one Home config one-item preparation/receipt; and its cancellation lease. It decodes and validates incrementally, checks cancellation before and after every page/event/factory-preparation step, reports `Reading page i/n`, `Validating event i/n`, `Applying directory projection`, and `Retiring page` with EN/DE labels supplied by the host UI layer.
* It must not emit any config mutation until all page bytes and all events validate. It then emits exactly one `ReplaceDirectoryProjectionState` mutation, not one `FoldDirectoryEvent` mutation per event. The config mutator must preserve `documents`, must parse its state strictly, and must never convert malformed persisted projection text to an empty successful projection.
* On decode/admission/freshness/cancel/factory rejection, preserve the previous projection and drive `RetainedToolWireInput::close_step` until its terminal-empty witness. On post-preparation failure, use the existing `ArtifactStoreOneItemPrepared` abort/close route; do not drop either the page input or a prepared config candidate. On success, wait for the exact config publication result acknowledgement before the shell advances its cursor.

The generic runtime already supplies the two required engines: `maintenance_step` chooses worker and retirement stages fairly (plugin lines 24279–24331), while `advance_typed_operation_publication_one` publishes one prepared unit (lines 22440–22475). The new job's native driver must invoke both via the production continuation/maintenance cadence and acknowledge its exact result token. Calling only `advance_typed_operation_publication` cannot progress the worker.

## Shell, worker, and resync integration

1. **Hub:** add authenticated `GET /directory/event-pages/v1?after=<safe-u64>` at the router near the current `/directory/events` route ([router](../../../../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs#L5137)). Move page construction behind a `DirectoryEventPageSource`-style trait; the existing private `DirectoryEventPageSource` and its monotonic paging test around lines 3353–3399 are the closest reusable server primitive, but it returns raw vectors and has no privacy/session/page seal.
2. **Shared TS client:** add `DirectoryClient.eventPage(after, signal)` that strictly parses the shared page schema. Remove the `Heartbeat -> lastSeq` update in `DirectoryClient.stream`; live `event` and heartbeat messages only call `requestResync()` / coalesce one pull. A rebootstrap-required, close, malformed stream frame, page failure, or session-generation mismatch clears in-flight work and schedules a fresh pull from the **last acknowledged page receipt**, not the highest observed stream sequence.
3. **Browser worker/ShellHost:** extend `BackboneWorkerRequest/Response` (currently lines 703–747 in [`🟦️.ts`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🟦️.ts#L703)) with a single retained-page request/result protocol, not `directory-message -> raw events`. The worker coalesces dirty signals while one page is active; it never has more than one retained 64 KiB page. ShellHost dispatches `foldDirectoryEventPage` only when Home is the mounted editor and returns the terminal page receipt to the worker. A non-Home mounted app leaves the page unconsumed/closed and retains only the durable cursor; it does not misroute events to Space Index.
4. **WGPU:** change `ShellDirectoryRunner::drain` consumer and `pump_directory_events` to schedule the same page pull, not collect `Vec<DirectoryEvent>`. Remove `fold_directory_events_action`; its `{events}` shape is already incompatible. Reuse the runner's finite pool, cancellation token, timer and 256-message wake queue only as hints; the event page itself becomes the bounded retained owner. WGPU needs an exact operation-result/ACK bridge in addition to its current `ProgramBridge::handle_action`: it presently gets `InvocationResult` but neither retains nor acknowledges typed result pages ([bridge](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs#L123)).
5. **Home registration:** remove `foldDirectoryEvents` from `app_commands!`, manifests, current raw tests, and the legacy config mutation in the same change; add `foldDirectoryEventPage` as a non-user view/host route with the dedicated app-owned retained factory, Config-only publication contract and strict current Home editor identity. Do not advertise it as a clickable user action. Space Index stays on the old full-bootstrap path until it obtains its own snapshot/page projection contract.

The acceptance receipt must contain only `after`, `through`, the page digest, and terminal status; it is an internal shell/worker cursor authority, not a user-visible secret or a durable shared directory event.

## Required laws

### Neutral, language-agnostic oracle

Add `directory-event-page-v1` fixture/schema/oracle under the existing shared directory schema domain. Independently parse and canonical-frame with TypeScript's `TextEncoder`/`DataView` plus a third-party JSON-schema validator and SHA-256 implementation; native Rust uses its own SHA-256. Required rows:

* normal 0→3 page; a zero-visible page that advances 3→8; two legal visible holes; 128-row/64-KiB exact boundary;
* exact duplicate accepted no-op and duplicate-range/different-hash denial;
* reversed/duplicate event sequence, event out of interval, `after != persisted cursor`, `through > head`, bad hash, noncanonical JSON, unknown schema, unsafe integer, excessive UTF-8/control text, 129th raw row, 64 KiB+1;
* session id or authorization generation substitution, revoked reply, and a current-session retry;
* cancellation before the first byte, between every header field/event, after validation/before config prepare, after prepare/before publish, and after result presentation; every outcome states prior cursor and exact retired bytes/pages;
* socket heartbeat larger than the last acknowledged cursor followed by a valid page; it must not skip the intervening page.

### Native laws

Register a focused Home/plugin-native target, not a broad shell build, proving with a real `ActionBus`, Home factory, config one-item preparation, mounted worker driver, publication ACK, and bounded close:

1. One page produces exactly one local config edit, preserves a document announcement through encode/decode, yields an acknowledgement receipt, and terminally retires every input/prepared owner.
2. Duplicate does not add an edit; a gap/hash/session failure emits no mutation and preserves the prior state.
3. Cancellation/close at every phase preserves the original state and ends terminal-empty under fixed grants 1, page-size-minus-one, and page-size.
4. A second page cannot start before first ACK; a stale generation cannot commit after the config changes; one rejected page cannot starve a valid queued dirty wake.
5. WGPU's real `ShellDirectoryRunner` plus operation driver turns a wake into a page request, processes worker → publishing → result → ACK → retiring, and does not use `dispatch_action`'s old `{events}` payload.

### Hub/process laws and registrations

Add a hub script command and Nx/launch registration alongside the existing `socket-grant-check` and `directory-ordered-publication-check` targets in [hub `project.json`](../../../../../../../../../🌎️hub/📦️packages/🦀️rust/📋️project.json). It should run one exact route test for: private-member page, filtered-empty advance, concurrent new append between page reads, revoke before response, stale generation, max-byte continuation, and restart from accepted cursor. Then add one process/browser scenario to the existing `@semio-tech/framework-os-dev:collab-e2e` gate (already registered as `⚖️gate🌎️collab-e2e`): authenticate, observe Home row, force directory socket close/heartbeat and reconnect, append a second directory event, assert the row appears once and no raw `/directory/events` action is observed.

The existing source-only worker test only proves command queue flush on a heartbeat; it does not prove event ingestion, page correctness, app publication, browser rendering, or native rendering. The current hub socket grant test proves grant/revocation transport, not client projection resync.

## Intentionally outside P0

This packet does not make Space Index incremental, make public visitors receive raw events, change event visibility policy, retain presence as a directory log page, or claim browser/native rendering success. It also does not solve full catalogue boot, document member opening, or two-user artifact mutation. Those follow only after this one authenticated Home projection path has a real bounded page/ACK/resync proof.
