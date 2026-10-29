# 1029 result: bridge (ex reactor, Finding A3)

STATUS: GREEN. The browser host now delivers events, command pages and cold-pair pages, one event per guest turn, the same way the native component host does. Premise correction below: the native host does NOT use `reserve-poll-input`.

## Premise correction (read first)
No host in the repo implements the `reserve-poll-input` / `write-poll-input` / `close-poll-input` protocol, and the guest never consumes it:
- Guest: `reactor/🎟️context/🌉️call/📥️input/🦀️.rs` stages bytes in a resident buffer (`with_original_poll_input`), but nothing in `poll_wit`/`poll_owned` decodes them (only tests and the close ladder read it). Its byte format is undefined.
- Native component host (wasmtime): `plugin/🖥️host/🎭️guest-runtime/🗂️wasmtime-guest-runtime-and-codec-dispatch/🦀️.rs:~198-214` per turn calls `stage-command-page` (if a command page), `stage-cold-pair-page` (if a cold page), then `poll(list<event>)`, where the list holds at most one event (see 1029-result-reactor.md A1).
- Native owned/interpreter host: one `semio_owned_poll_v1` call with a JSON `PollInput { events, command_page, cold_pair_page, budget }` (`🧠️interpreter/🏃️runtime/🔄️owned-runtime/🦀️.rs:118-146`).
So "behaviourally identical to native" = the stage-then-poll sequence. Inventing a poll-input wire format (guest decode + TS encode of kernel events + native twin) would be a new design, not a mirror; I did not do it. The three poll-input exports remain unused guest surface (decide later: implement end to end or delete).

## Changes
1. `plugin/🌐️browser-bundle/🏗️materialization/🌉️component-bridge-source/🟦️.ts` `createActorApi().poll`: the `canonical-input-held` refusal is gone. Now: events validated and mapped to WIT variants BEFORE the turn is issued (an invalid event takes no custody); `events.length > 1` throws `actor-poll.event-count: a reactor turn admits at most one event`; inside the existing custody try: `reactor.stageCommandPage(commandPage.cursor, commandPage.bytes)`, `reactor.stageColdPairPage(coldPairPage)`, then `reactor.poll(wireEvents, ...)`. A stage failure follows the same trap/held-native-refusal path as a poll failure.
2. `plugin/🌐️browser-bundle/🏗️materialization/🧵️shard-worker-source/📨️message-handler/🟦️.ts` `frame` case: one `actor.api.poll([event])` per envelope in frame order (matching the Rust ShardLoop); an empty frame still polls once with `[]`; reply is the single turn result for one envelope, an array of turn results for several. `turn` messages stay one poll (every production caller sends 0 or 1 event: `activation-registry` `enqueueTurn` call sites; the bridge now refuses more).
3. Test `dev/🧪️tests/🧪️ticket-owned-browser-host-staging/🛂️routes-and-bridge/🟦️.ts` (3 cases rewritten from refusal to delivery): command page staged before poll (order, cursor and bytes identity, empty poll list, no held producer refusal, next poll works), cold-pair page staged, patch ack/rejected delivered as the single `{tag,val}` event, every lifecycle open/close/ack delivered as the single event, two events refused with the law message.
(The four TS reservation-hub one-event changes of 1029-result-reactor.md A2 stand; this change removes the bridge refusal they would have hit.)

## Verification
- vitest (project config `🧑‍💻dev/🧪️tests/🎚️config/🟦️.ts`, `SEMIO_TEST_LEVEL=long`): `ticket-owned-browser-host-staging` 72 passed, 3 failed; all 9 `pluginComponentBridgeSource` cases pass (incl. the 3 rewritten). The 3 failures are environmental and unrelated (`SEMIO_SCRIPT_PROVIDERS` not set under direct vitest x2, vendor `cli.js` byte-identity); logs `C/🗑️generated/1029/bridge-vt{1,2}.txt`.
- vitest (framework-os config) `worker-law-suites` (browser-actor-reservation incl. poll lane, reconcile, surface rendering): 100/100 pass (`bridge-vt3.txt`).
- `bun test ./🚪️lifetime/🟦️.ts` (materialization tests, real shard worker): 2/2 pass with `SEMIO_TEST_ARTIFACT_DIR` set; `./🏛️bootstrap/🟦️.ts` 3/3.
- `tsc --noEmit -p 🛍️products/💻️os/tsconfig.json`: 9 errors, none in touched files (all in store/paged-one-item tests, store/replay tests, renderer PluginRuntime handle-ui-intake `issueTarget`, a renderer test helper); `bridge-tsc1.txt`.
- NOT verified: a real compiled component in a browser (no wasm build yet); the frame multi-envelope reply shape has no existing consumer (no production caller of `ShardClient.grant/envelope`).

## Files changed
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🌉️component-bridge-source/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🧵️shard-worker-source/📨️message-handler/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧪️ticket-owned-browser-host-staging/🛂️routes-and-bridge/🟦️.ts`
SHARED CHANGE: none. Not touched: instance-typed-ops, time-travel-turns.
