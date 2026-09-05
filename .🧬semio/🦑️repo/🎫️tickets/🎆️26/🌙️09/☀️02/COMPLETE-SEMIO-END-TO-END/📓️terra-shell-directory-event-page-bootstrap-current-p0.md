# Shell Directory Event-Page Bootstrap — Current P0

## Verdict

**RED: neither shell can consume an authenticated event page before opening its live directory socket.** The shared page and the Home-side retained config replacement now exist, but the browser and native shells still start a socket at `0`, route its raw events through the legacy `foldDirectoryEvents` action, and have no acknowledged page frontier, page owner, or page/action acknowledgement protocol.

This is a source audit only. No build, native law, browser test, or process test was run here.

## What is now real, and what is stale

The earlier retained-owner blueprint is stale in material ways:

* `DirectoryEventPageV1` is now a real cross-language contract. It has canonical JSON, receipt validation, a 128 raw-row limit, a 64 KiB page limit, and a 48 KiB event limit in [directory schema](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs#L197-L305). Its existing neutral/source and isolated native gates are `@semio-tech/framework-os-kernel:directory-event-page-contract-check` and `...:directory-event-page-contract-native-check`.
* Home now has one concrete `applyDirectoryEventPage` command. It parses **canonical** `pageJson`, calls `HomeConfig::apply_directory_event_page`, and emits exactly one Config replacement or no-op on exact replay: [command](../../../../../../../../../✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📬️apply-directory-event-page/🦀️.rs#L9-L35). The `HomeConfig` fold enforces receipt, authority generation/binding, sequential `after`, idempotent exact replay, and reset-only-at-zero: [config](../../../../../../../../../✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs#L101-L137).
* The action is already in `HomeRetainedCommandJobFactory`, has a Config-only publication contract, and has a registered source/native owner check: [factory](../../../../../../../../../✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs#L59-L150), [Nx targets](../../../../../../../../../✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/📋️project.json#L8-L22). The current native law is only the direct command/config law; it is not a shell acknowledgement or process proof.
* The earlier claims that Home drops `documents`, resets corrupt JSON to empty, has only an 8 KiB retained ingress, or lacks a page action are obsolete. The `documents` field is preserved and malformed persisted projection is a fault.

The hub `GET /directory/event-page/v1?after=<u64>` route remains absent; its bounded scan/revalidation packet is a prerequisite and is not repeated here.

## Exact current RED seams

| Boundary | Current source | Consequence |
| --- | --- | --- |
| Browser identity → socket | [ShellHost identity effect](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx#L1665-L1688) posts `directory-open` with `since: 0` immediately after identity resolution. | A socket opens before any sealed projection is applied. |
| Browser worker transport | [worker `openDirectory`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts#L1360-L1394) immediately owns `client.stream(since)`; [request allowlist](../../../../../../../../../🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts#L1540-L1550) admits the old `/directory/events?since=` but not event-page. | No page fetch, no canonical raw-body verifier, and no owner that can await Home acknowledgement. |
| Browser main-thread delivery | [worker response routing](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx#L1449-L1488) only understands `directory-message`; [legacy dispatcher](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx#L4281-L4294) invokes `foldDirectoryEvents` without awaiting it. | No `pageJson` action, no ACK-before-next-page, and no retry state. |
| Browser client resume | [TS `DirectoryClient.events/stream`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🟦️.ts#L4304-L4353) has no event-page method. The stream moves `lastSeq` on every event and global heartbeat: [tracking](../../../../../../../../../🧰️framework/🛍️products/💻️os/🟦️.ts#L4402-L4408). | Reconnect uses an *observed* frontier, not a Home-committed frontier; an unacknowledged message/heartbeat can make a reconnect skip a page. |
| Native identity → socket | [WGPU identity completion](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs#L4170-L4209) immediately calls `client.stream(0)`. | Same pre-page socket race. |
| Native live delivery | [WGPU pump](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs#L4212-L4246) batches `Event`s and calls [raw fold dispatch](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs#L4138-L4149). Its helper builds `{events: [...]}`, while Home’s legacy parser reads only `eventsJson`: [Home parser](../../../../../../../../../✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs#L484-L485). | Native raw folds are not a valid substitute for page application; they can silently become `[]`. |
| Native client resume | [Rust `DirectoryClient`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs#L633-L685) only exposes legacy `events`; [stream tracking](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs#L1016-L1029) advances on heartbeat. | It cannot preserve canonical response bytes or resume only from a config-acknowledged `throughSeqInclusive`. |
| Home recipient lifetime | React [switches the visible app](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx#L2954-L2996); native holds only the current `session` when dispatching raw batches. | Once the visible app is Studio/Space Index, there is no named retained Home projection recipient. A page controller must not dispatch `applyDirectoryEventPage` to whichever app happens to be visible. |

`applyDirectoryEventPage` also still accepts a legacy `page_json` argument alias at [Home parser line 445](../../../../../../../../../✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs#L444-L446). The new sealed ingress must use—and tests must require—only `pageJson`; remove the alias as part of this no-compatibility packet.

## Minimal ownership-preserving packet

### 1. Keep one hidden Home projection instance, distinct from the visible app

Add one shell-owned `DirectoryHomeProjection` record per shell, not a global cache and not a second directory read model. It contains only the already-created Home plugin instance/bridge, its immutable Home controller id, an operation/cancel root, and a monotonic bootstrap epoch. It is created after identity *and Home plugin/app availability*; it remains alive while Studio/Space Index is visible, and is closed through the existing app destroy/close route on shell unmount. Do not apply pages to Studio or Space Index.

React owns this beside `sessionRef` in `ShellHost`; native owns it beside `directory_client`/`directory_stream` in `ShellState` ([native lifetime fields](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs#L1740-L1810)). Its only dispatch is the existing retained `applyDirectoryEventPage` with `{ pageJson: canonicalPageJson }`.

The ACK is successful only after the real invocation resolves **and** its returned config/history outcome identifies the same receipt/binding/generation/through frontier. A mere `onActionRef.current(...)` call is not an ACK: that ref is intentionally void. Add a small dedicated `applyDirectoryEventPageAndAck` bridge, reusing the normal action encoding/response/effect route but returning `Promise<DirectoryPageAck>` / native `Result<DirectoryPageAck, _>`. It must not broaden the generic UI action API.

### 2. Add one page transport method per existing client

Add `event_page(ctx|options, after)` to the existing clients, not a second HTTP client:

* Rust: [directory client](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs) needs a raw-body request branch that bounds the bytes, strict-UTF-8 decodes, calls `DirectoryEventPageV1::parse_canonical_json` on the **received** bytes, and returns one private `CanonicalDirectoryEventPage { page_json, header }` owner. Generic `request_json` is insufficient because it loses exact input canonicality before re-encoding.
* TypeScript: [DirectoryClient](../../../../../../../../../🧰️framework/🛍️products/💻️os/🟦️.ts) needs an equivalent text response path using `parseDirectoryEventPageV1`. It returns the original validated canonical string plus parsed header; it must not call `response.json()` then `JSON.stringify`.
* Extend the browser-worker allowlist exactly for `GET /_semio/hub/directory/event-page/v1?after=<decimal safe u64>`; do not grant arbitrary directory URLs.

The page owner is capped at `DIRECTORY_EVENT_PAGE_MAX_BYTES` (64 KiB). It holds only canonical page bytes/header and never a session id, bearer, or grant. The only client-visible session fact is the existing `sessionBindingSha256`.

### 3. Serialize fetch → Home ACK → next fetch

Use the same small three-state state machine in the browser worker and native WGPU Shell:

```text
Idle(ackThrough) ─fetch(after=ackThrough)→ AwaitingHomeAck(page)
AwaitingHomeAck ─exact accepted ACK→ Idle(page.through)
Idle(page.through) ─page.hasMore→ fetch(after=page.through)
Idle(finalThrough) ─open socket(since=finalThrough)→ Live(finalThrough)
Live ─event/heartbeat/rebootstrap signal→ Idle(ackThrough)
```

Only one fetch/page/action is live. Do not prefetch page N+1. An ACK carries exactly `{ receiptSha256, sessionBindingSha256, authorizationGeneration, throughSeqInclusive, bootstrapEpoch }`; the receiver compares all five to the still-owned page before advancing. It never carries a raw session id.

On action fault, timeout, or transport failure, retain `ackThrough`, close the transient action/page owner, and retry the same `after` under bounded jitter and the existing operation deadline. On cancellation/unmount, cancel the fetch/context, reject or close the pending Home action, clear the one page string, and never send an ACK. The browser worker is already terminated on shell teardown, but it needs an explicit `directory-bootstrap-close` path for ordinary identity replacement/unmount rather than treating worker destruction as page-owner retirement. Native must cancel a `ShellPoolFuture`/receiver tied to `directory_cancel`; the existing `ShellDirectoryRunner::cancel` pattern is the reusable close model, not a new queue.

When a page changes binding or authorization generation, the Home action itself permits only `after=0`; controller rejection therefore means: close stream, discard pending page, perform a fresh authenticated bootstrap at `after=0`, then reacquire live. HTTP 401 follows the existing identity revalidation/sign-out path, never local reset plus continued socket use.

### 4. Make the socket a wakeup channel, never page authority

Open the global socket only after the final page ACK, with `since = throughSeqInclusive`. A page response and then socket handshake is gap-safe: records appended between them replay from that `through` value.

While live, events/heartbeats only set one bounded `directoryDirty` latch. They do not call `foldDirectoryEvents`, mutate Home directly, or advance the durable resume frontier. The next page fetch starts at the last ACKed `through`. A successful page ACK updates the socket's reconnect frontier; observed `Event.seq` and `Heartbeat.headSeq` must not do so. This requires a narrowly separate acknowledged-frontier mode on `DirectoryStream`/TS `streamFor`, or an explicit `acknowledge(through)` operation owned by the page controller. Reconnecting before an ACK reuses the previous ACKed frontier and tolerates duplicate wakeups; it cannot skip a page.

`RebootstrapRequired`, a session binding mismatch, or authorization-generation mismatch cancels the socket first and goes through the `after=0` bootstrap path. This global bootstrap is deliberately separate from scoped-document 4401 work.

## Ordered source slices

1. **Hub prerequisite (separate owner):** implement the audited authenticated page route and canonical response contract. It must be complete before either client slice.
2. **Shared client source:** Rust client raw canonical page method plus acknowledged-frontier stream mode; TS equivalent plus parser. Extend only the existing worker request allowlist.
3. **Home packet:** retain current `applyDirectoryEventPage` route/config preflight and remove the snake-case action alias. Add an invocation-level acknowledgement witness, not a duplicate Home projection parser.
4. **Browser packet:** extend `BackboneWorkerRequest`/`Response` in `os/🟦️.ts`, add one worker page state machine, and add a ShellHost private Home-projection bridge. Replace the identity effect's direct `directory-open` at line 1682 with bootstrap start. Replace the raw `directory-message` fold path.
5. **Native packet:** add the same bounded page state beside `directory_stream`; use `ShellPoolFuture`/the existing worker pool for HTTP, `dispatch_action` for one Home action, and only then `ShellDirectoryRunner::start(client.stream(through))`. Delete the raw fold dispatcher from this global lane rather than trying to repair its incompatible `{events}` shape.

## Non-vacuous acceptance matrix

### Neutral fixture/oracle

Extend the existing `event-page-v1.json` under `🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory` with a bootstrap trace schema shared by Bun/AJV and Rust/TS:

* two pages with invisible raw holes; page 2 is requested only after page 1 exact ACK;
* empty-visible, 128-raw page; UTF-8 near 64 KiB; oversized/noncanonical/duplicate-key/bad receipt denial;
* repeated exact ACK is idempotent; wrong receipt, binding, generation, through, or bootstrap epoch does not advance;
* action fault/cancel leaves the request cursor unchanged; retry gives identical `after`;
* final ACK opens socket at exact `through`, not 0 or heartbeat head; socket reconnect before ACK uses old through;
* changed binding/generation forces `after=0`; no fixture contains a raw session/bearer.

### Native laws

Keep the existing contract and Home checks, then add isolated exact laws:

* `DirectoryClient::event_page` accepts only raw canonical bytes and proves cancel-before-I/O, cancellation in flight, and 64 KiB release.
* Home `plugin()/create_app` action-path law drives the retained job to terminal Config publication, reads the actual Config receipt/frontier, and proves no ACK until that result. The present direct `handle` law is necessary but not sufficient.
* WGPU Shell fake transport/bridge law: `Worker -> AwaitingHomeAck -> hasMore -> stream(through)`, delayed action, action rejection/retry, cancel/drop, reconnect prior to ACK, and no `foldDirectoryEvents` call. It must inspect the dial `since` and the Home config receipt, not merely count scheduled callbacks.

### Browser and process laws

* Worker Vitest law with an exact allowlisted event-page URL, one pending page, explicit ACK, retry after action failure, abort/close cleanup, and reconnect at the ACKed cursor.
* ShellHost React law with a real mocked Home plugin invocation that returns a config outcome: page 2 cannot be requested before action 1 resolves; session switch does not reroute the page to Studio; unmount aborts and no late ACK reaches the worker.
* Two authenticated hub processes: append an event between final page response and socket open, observe it through the resumed socket/page loop exactly once; restart/reconnect and generation change require rebootstrap. Capture no bearer/session id in page body or browser console.

Register new commands through the owning `📜️script.ts`, `📋️project.json`, and `.vscode/🧩️launch.seed.jsonc` (then regenerate `launch.json`). Existing registered anchors are:

* `bun nx run @semio-tech/framework-os-kernel:directory-event-page-contract-check --skip-nx-cache`
* `bun nx run @semio-tech/framework-os-kernel:directory-event-page-contract-native-check --skip-nx-cache`
* `bun nx run @semio-tech/space-plugin:home-directory-event-page-owner-check --skip-nx-cache`
* `bun nx run @semio-tech/space-plugin:home-directory-event-page-owner-native-check --skip-nx-cache`

## Honest nonclaims

The current Home command/config source proves only direct sealed-page reduction. It does **not** prove the hub route, raw HTTP canonicality at either client, an owning Home instance across session changes, browser/native ACK ordering, live-socket resumption, or a two-client process journey. The existing scoped-socket revocation and invite/presence work are not included in this packet.
