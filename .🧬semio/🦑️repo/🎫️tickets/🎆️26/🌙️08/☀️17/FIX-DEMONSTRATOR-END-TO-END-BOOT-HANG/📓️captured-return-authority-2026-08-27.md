# Captured Return Authority

## Source Boundary

`ShardInstanceLifecycleLease.reserveReturn(maximumResponses)` now reserves one private `OwnedShardReturn` under the original `ShardInstanceOwner` and activation. The limit is explicit; it is a retained response-slot limit, not a physical heap lease or timing guarantee. `pendingReturn` keeps the exact owner accessible after transport/validation failure. Only one return owner may be reserved per activation.

Execute derives the immutable origin from its actual dispatched transport rN. A refused post retains its original input/event root and response outcome; retry uses a new transport correlation id but the original canonical origin. Native identity is checked before a private page is minted, including the activation's last-admitted return-sequence frontier. Poll/cancel use the original worker/slot and contain no semantic events. They remain usable after operation routing is revoked. Canonical fixed-result bytes are required; old whole-result objects are not accepted by this path.

Each actual response envelope is retained in the pre-reserved output slot before pending removal, heartbeat recomputation or external continuation. Raw response getters are not exposed on the return facade. Its page is a privately minted `OwnedShardReturnPage`, with exact host-owner/activation/lifetime verification and scalar byte reads from the fixed decoded page. Public receipt objects, output reservations and prototype fabrication cannot mint it.

The worker slot now records loss independently of the routing roster. An original worker error after exclusive reassignment therefore invalidates captured controls, even though that actor name is no longer in the old slot's routing set. Ordinary and legacy lifecycle turns cannot bypass an owned canonical return. Disposal is refused while the return owner remains retained.

## Executed Tests

- RED1: missing actual reserveReturn API, 3 failed / 108 skipped, eight files, 644 ms, start 21:30:20.
- Initial actual full actor GREEN: 111/111, eight files, 2.82 s, start 21:32:51. Initial strict: exactly tutorial seven.
- RED2: public raw-response exposure and original-worker loss after route reassignment, 2 failed / 3 passed / 108 skipped, 1.05 s, start 21:35:26. The original lost worker wrongly accepted cancellation in that run.
- Second actual full actor GREEN: 113/113, eight files, 1.30 s, start 21:36:31, exit 0. Strict rerun: exactly tutorial seven.
- RED3: a malformed response after native identity admission incorrectly revoked cancellation, 1 failed / 113 skipped, 507 ms, start 21:39:45. Cancellation must remain valid after validation fault and operation revocation, while the original malformed response stays retained.
- Current actual full actor GREEN: 114/114, eight files, 3.50 s, start 21:40:08, exit 0. Current strict rerun: exactly tutorial seven, no actor or input diagnostics.

Logs are 🧪️actor-captured-return-{red,green}-{1,2,3}.log and 🧪️actor-captured-return-strict-{1,2,3}.log. The language-neutral fixture/schema and strict Ajv oracle agree with the observed authority/capacity boundary.

Hashes: ShardClient 5798a3b4a0f39380f6118360869cad7150493beffad8c7fba96e5fc1b3ef58c3; fixture d1f3d4b19d005f7ba32fe97f082adb29b10711e8dc088de16be917fa0cafebcb; fixture schema 39fa71243788ec9746a24ff24ab0ab73c2c5999fa48397246556fc1844174f19.

## Deliberately Unclaimed Boundaries

This is staged captured transport ownership with controlled worker fixtures, not a PluginRuntime or generated-worker mount. The existing `turn` transport carries canonical return-drive bytes in this staged path; there is no old/new result decoder union. The generated producer and native poll still need their one-ABI cutover before a real consumer calls it.

No InputAck or RetiredAck method exists yet. The owner retains every response, including refused/unknown wrappers, and refuses when its explicit response capacity is full. This is fault containment, not return-stream liveness or final retirement. Private Field/Fragment/Release, page/input cleanup, fixed-control response retirement, exact final raw-root witness and composition admission remain required before mounting. There is no false complete state or map deletion to stand in for these obligations.

The retained request events remain their original mutable input root; this packet does not certify bounded request encoding, event-array immutability, callback latency or 8 ms execution. The fixed page decoder's allocation is not a fulfilled shared host byte lease. No raw page/result union or arbitrary boolean/callback release authority was introduced.

All six demonstrator apps still require coherent rebuilt guest artifacts and actual content, interaction and close/reopen verification. No build output, cache or active evidence was deleted or relocated.
