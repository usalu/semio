# Shared Worker Inbox Ownership

## Decision And Scope

The previous one-canonical-return-per-worker proposal does **not** identify the next callback's owner. It is superseded as an inbox-routing or ownership solution. No worker-wide serialization, second channel, object/binary compatibility decoder, or whole-result fallback is being mounted.

This is a current-source inventory and a proposed ownership matrix. The neutral inventory schema is not a new transport protocol. Source minima below are not funded live admission envelopes. The generated JavaScript is exercised in Node VM with controlled actors, not a browser worker, transferred backing, Wasm guest or demonstrator app.

## Actual Producer And Consumer Census

[Shard source](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧵️shard-client.ts:351) declares ten outbound kinds and five inbound shapes: result-success, result-fault, heartbeat, trap and frame. [The generated worker](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🌐plugin-web-materialize.ts:94) adds attachHeartbeatSab to its consumed kinds. Its frame branch has a distinct effect-complete/effect-error reply path that intentionally returns before ordinary request dispatch.

| Host-to-worker kind | Existing producer/caller | Worker consumer and response |
| --- | --- | --- |
| activate | ShardClient.activate; Kernel ActivationRegistry | loadActor/createActorApi, heartbeat then result-success(undefined) or result-fault |
| turn | activation.turn, instance lifecycle, captured-return foundation | exact actor generation/in-flight actor check, heartbeat then whole poll result; the actual worker does not read returnDrive |
| startJob | ShardClient.startJob | host job API, heartbeat then success(undefined)/fault |
| stepJob | ShardClient.stepJob | host job API, heartbeat then result/fault |
| cancelJob | ShardClient.cancelJob | fire-and-forget; no heartbeat or result |
| takeSegmentedDownloadChunk | ShardClient and React handle | heartbeat, checked chunk or undefined/fault; the chunk maximum does not fund wrapper/PendingEntry |
| checkpoint | ShardClient; ActivationRegistry eviction | heartbeat then whole checkpoint result/fault |
| restore | ShardClient; ActivationRegistry restoration | heartbeat then success(undefined)/fault |
| dispose | captured activation teardown | generation check and actor Map deletion, no response; not a worker/memory retirement receipt |
| frame: Register/Unregister | declared ShardFrame path | ordinary request result(undefined) |
| frame: Grant/Envelope | ShardClient.grant/envelope | ordinary request heartbeat and whole poll result/fault |
| frame: effect-complete/effect-error | postEffectReply | exact actor generation/envelope check, resolves/rejects host-shim pending Promise; **no result of its own** |
| attachHeartbeatSab | spawnShard | installs shared view; no result |

The worker can process another actor's request while awaiting a first actor's poll. The current inFlightTurnActors Set is per actor, not per worker. Delaying effect-complete/effect-error behind that pending poll would deadlock the poll's awaited host import.

[Host shim](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🌐plugin-web-materialize.ts:842) creates three frame payload kinds: effect-request, effect-emit and ui-patch-emit. Its 24 effect-request operations are storage-read/write/delete, blob-load/write/read, http-fetch, document-read/write, link-resolve, registry-query, io-compose/run, cache-derive/read, invoke-extension, open-window/dialog, dispatch-action, spawn-plugin-instance, request-file-open, request-media-frames, request-capability and spawn-job. Their parameters/results are not bounded by the canonical return's4161-byte allowance.

## Current Original-Root Ownership

“None” below means no explicit retained original-root owner was found in the named path; it does not mean a JavaScript engine immediately collects the object.

| Actual inbound producer | Original pre-post owner and charge now | Earliest current consumer retention | Current destination and fault/refusal behavior |
| --- | --- | --- | --- |
| worker reply: result-success | local await result and new object wrapper; no pre-post response record/byte lease | handleMessage reads kind and requestId, looks up pending, then conditionally captureResponse(message) only for entry.output | output-backed controlled return retains wrapper; ordinary PendingEntry resolves only value after Map/Set deletion. Missing/wrong-slot entry returns without a retained wrapper |
| worker replyError: result-fault | catch-local original error; string/stack/type/framesBytes normalization before post, no original fault roster | same late conditional output capture | ordinary route synthesizes Error through graftWorkerStack after pending deletion. Throwing normalization can fail before a reply. A post that succeeds then throws can lead to a second result |
| worker heartbeat | local fixed object, plus optional SAB store; no charged post owner | none before kind/turnSeq read and recordHeartbeat | updates five-field heartbeat state, discards wrapper. No proof covers unknown added descendants or malformed scalars |
| worker bootstrap trap | local diagnostic string/object before throwing; activation absent, actorId=* and generation=null | none before callbacks | onActorTrap callback receives selected values; original wrapper/error is not retained. Callback failure has no original ingress owner |
| shim frame: effect-request | pendingEffects Map retains resolving functions, **not the exact posted frame/params root**; new Promise/frame/params, no receiver resident reservation | handleMessage reads actor/generation; handleInboundFrame traverses envelope before host entry creation | exact current activation routes to PendingHostEffect. That entry retains activation/controller/requestId/links, not original envelope/params. Duplicate/foreign silently returns; full quota or missing handler emits effect-error. Callback throw/then fault has no whole original-frame owner |
| shim frame: effect-emit | local argument and new frame, no pending request/receiver reservation | none | current handleInboundFrame intentionally ignores it; this is an existing unsupported path, not retirement or success |
| shim frame: ui-patch-emit | local patch and new frame, no pending request/receiver reservation | none | current handleInboundFrame intentionally ignores it; exact original patch is not transferred to retained UI |
| platform onerror | platform event, not a worker post; no pre-callback event owner | none before stale-slot check, logging and failShard | failShard rejects/deletes pending entries and clears routes. Original event/error is not a close receipt or retained terminal witness |
| platform messageerror | not a declared application message | no handler in ShardWorkerLike/spawnShard or generated worker | no explicit owner/response settlement; needs a separate platform-event contract on the existing worker, not a fabricated result |
| arbitrary, malformed, duplicate or stale callback data | no valid producer credit established | none before current discriminator/lookup | null can throw at kind; unknown objects or unmatched binary can fall through to absent request lookup; stale worker returns immediately. No general quarantine is mounted |

The current canonical-return foundation calls send with an extra returnDrive and expects an object result containing encoded result bytes in controlled tests. The actual generated worker still calls poll(events,commandPage,budget) and emits object result. The sole replacement binary response format is already specified, but this source inventory is **not** evidence that it is transported.

## Required Original-Owner Transfer Matrix

These are obligations for the later single-protocol cutover, not new callable APIs.

Every callback first needs an original-worker-owned, preinstalled ingress registration before any data discriminator, request lookup, callback, formatting or fallible parser construction. The event/data references and first fault each need declared metadata. Its funding cannot come from a request record that might already retire. Capturing data after reading a caller-supplied event.data getter does not cover failure while extracting the original event; the platform event boundary must be explicit.

The transient ingress registration is not synonymous with the canonical raw backing slot. Its exact root must move to the appropriate already-admitted destination before the transient slot can be reused. Moving ownership is not retiring the root.

| Traffic | Required pre-callback/pre-post owner and charge | Exact destination transfer | Refusal/unknown/duplicate path | Event permitting ingress reuse |
| --- | --- | --- | --- | --- |
| canonical binary result/fixed fault | original dispatched request, exact worker/activation, one reply credit, external4161 reservation plus neutral/domain/cell/parser charges; worker retains original guest result/error before encoding | ingress root to that original request's private receiver; header checks actual dispatched request, not return origin; backing custody and semantic interpretation are separate | retain first malformed/foreign root in independently funded worker violation owner; keep original expected pending/output; late genuine reply remains retirement-authorized | exact original receiver has strongly installed the entire root/custody and ingress has detached its alias in a separate observed phase; format validity or post return is insufficient |
| ordinary result-success | its own original pending request/response registration and declared payload capacity, not canonical credit | whole original response to its matching ordinary request, then exact value transfer to the original public caller | wrong request/slot or a result aimed at a canonical request is not accepted as compatibility; no current-map replacement lookup or drop | original ordinary destination confirms complete-root ownership; unresolved raw wrapper/extra fields remain with that destination |
| ordinary result-fault | original worker error + closed failure representation and ordinary response owner; Error/stack/string conversions separately admitted | exact ordinary pending fault owner before graft/callback | original normalization/post failure stays owned; secondary thrown values cannot replace the first error or certify no reply | original destination retains original response/error before any consumer code; formatting is not a release witness |
| heartbeat | worker-owned scalar-control registration and whole-root contract independently funded from data replies | exact original worker heartbeat participant | malformed/foreign/extra payload is retained as violation, not silently coalesced or charged against a turn | actual scalar control root release after producer/whole-root proof and callback alias detach; copying turnSeq alone is insufficient |
| trap | original worker or exact activation fault participant preinstalled before worker startup | original fault participant, then diagnostic observation | original root stays retained if callback throws; wildcard bootstrap does not invent activation identity | strong exact fault handoff plus ingress alias detach; logging/throwing is not disposal |
| effect-request | preadmitted original activation host-effect receiver; request/params/frame/response/Promise graph accounted independently | whole original envelope to exact activation/request effect owner before invoking handler | valid concurrent traffic keeps its route. Full admission explicitly refuses while retaining original request until the exact error-reply/input handoff; stale/duplicate enters its own retained rejection path | exact host-effect owner retains every original descendant and ingress alias detaches; a resolving-function Map entry alone is not enough |
| effect-emit | real original effect delivery owner and declaration required before emission | schema-owned effect consumer, preserving original frame | current ignored path must be replaced by owned rejection or genuine delivery, not canonical result classification | exact effect consumer takes root; no consumer means no success/reuse proof |
| ui-patch-emit | real native patch producer receipt and retained UI intake admission, not arbitrary frame shape | original patch/input owner; cannot mint receipt from actorId, surface or revision | unavailable authority retains/rejects original frame; cannot route it into a different instance aggregate | genuine source/UI ownership handoff, not a structural patch or callback return |
| platform error/messageerror | preexisting original worker platform-fault slot, separately charged | original worker terminal coordinator while keeping accepted request roots | no-result or worker failure is not cancellation proof; exact transport terminal evidence remains required | exact fault participant takes entire original event/error root and observed ingress detach; no current route deletion shortcut |
| unknown/malformed | finite first-violation reservation independent of ordinary/canonical destinations | exact worker-incarnation quarantine, never replacing its prior root | stop **new** admissions, retain already accepted owners and permit genuine credited retirement responses/host replies. No bound claimed for arbitrary rogue flooding | only a genuine later typed whole-root retirement/transfer; otherwise slot remains occupied and charged |

A closed, faulted, old activation can still own a legitimate previously-issued retirement response. That authority is distinct from permission for new commands or new effects. The current inboundActivation active-operation predicate is not a sufficient close-time host-effect/return authority.

A public object discriminator or an intrinsic ArrayBuffer shape check is only routing evidence after capture. It is not original producer authority, exclusive custody, immutable contents or unknown-field disposal. Current arbitrary object messages cannot be declared bounded merely because the canonical binary backing is bounded.

## Concurrency And Credit

There is no blanket worker mutex in the proposal. Existing ordinary requests, per-actor execution, heartbeat and host-effect completion continue independently. Any future canonical admission limit must be explicit before accepting work; it cannot cancel or serialize unrelated already-accepted work.

One independently owned callback root plus one violation slot does not bound all unsolicited traffic or the platform's cloned message queue. The actual producer must have same-protocol pre-post credit for each admitted response/control/effect class, and the receiver must have matching strong destinations. The present unlimited one-way emit paths and unbounded ordinary result graphs do not meet that contract. Their quotas/retirement cannot be inferred from64 host requests or4161 canonical bytes. This remains a design blocker for a whole inbox claim, not a reason to add a channel or drop messages.

## Source-Bound Shell Census

The test compares exact TypeScript AST fields with the neutral inventory and independently computes each shell group using BigInt. Values use64 bytes per shell plus16 per field; no claim of physical heap size.

| Existing source subset | Fields | Logical bytes / slots / owners |
| --- | --- | --- |
| ShardClient shell | 32 | 576 / 1 / 1 |
| ShardSlot + ShardHeartbeatState | 6 + 5 | 304 / 2 / 2 |
| ShardActivation + CapturedShardActivation | 13 + 5 | 416 / 2 / 2 |
| ordinary PendingEntry | 6 | 160 / 1 / 1 |
| HostEffectLedger | 4 | 128 / 1 / 1 |
| PendingHostEffect | 5 | 144 / 1 / 1 |

ShardSlot is **six** fields, not seven. The ShardClient shell includes the existing UI-controller fields, so adding its576 to that controller's already-accounted208 would double-count those fields. The pending160 is already part of the earlier608 current-source pending inventory; it is not another newly admitted resource.

These minima exclude Map/Set/array headers, entries and backing, closure environments and callbacks, Promise/reaction graphs, strings/assets/events/errors, platform queues, worker/Wasm startup, neutral cells/records, raw storage, new worker ingress/quarantine fields and actual request construction/parser owners. No final worker or request admission total is asserted. The existing admitted output1008/12/12 and captured-return1360/13/13 cannot fund these omitted graphs.

## Authored Caller Cutover

The explicit framework search used rg -a for TS/TSX because PluginRuntime contains an existing NUL separator in its coalescing key.

| Actual caller | Current role | Required later change / preserved boundary |
| --- | --- | --- |
| ShardClient OwnedShardReturn + module-private submitCapturedReturn | execute/retry/poll/cancel return Promise; reserveReturn/reserveResponse are phased | replace only this canonical dispatch with original event-driven request owner once fully admitted; no Promise compatibility branch |
| Kernel returned-content InputOwner | owns exact source/page/content/field relationships; no direct execute call | keep original source binding and private fields; replace page backing access only at actual custody cutover |
| UiDocumentStore nativePagedFieldFixture | sole external reserveReturn/reserveResponse/execute caller found | coordinate exact new grant/API once source is released; never fabricate success from a test-only response |
| React PluginRuntime production | direct ShardClient bootstrap, ordinary typed turn scheduler/activation.turn | no production OwnedShardReturn mount currently; its Promise/waiter/coalescing records need an explicit later original-owner cutover |
| PluginRuntime lifecycle scheduler | existing captured open/poll/close/receipt/issued-UI-ACK branch, currently exercised by tests | preserve retirement authority and original aggregate; no allowClosed or arbitrary Event[] |
| WGPU plugin bridge | createPooledActorRuntime, ordinary per-actor Promise chain and turn | same ledger profile does not admit its request graph; no renderer-specific fallback |
| actor shard-runtime factory | constructs ShardClient/Worker pool and watchdog | must participate in actual original bootstrap/callback admission before new receiver fields can claim funding |
| common Kernel ActivationRegistry | activate/turn/checkpoint/dispose/restore and extension activation | ordinary API behavior stays intact; do not put unrelated accepted work behind canonical return |
| web benchmark and TaskManager/Kernel tests | direct constructor/ordinary fake worker users | authored fixture updates required if bootstrap/receiver contract changes; no optional default ledger or legacy constructor |

Both current React and WGPU production option builders omit onHostEffect; source therefore selects the existing explicit missing-handler failure. This is a source finding, not a fresh observed all-app failure or an instruction to synthesize an extension result.

## Required Failure Laws Before Mount

1. Capture the original callback event/data before discriminating; retain getter/validation/finalizer failures without overwriting first roots.
2. Interleave another actor's ordinary response, heartbeat and awaited effect request/completion with a canonical pending request. Preserve both routes and the original pending identity.
3. Admit no post before exact request/worker/response registrations, byte reservations and receive fence. A wrapper that calls real post then throws keeps the posted owner; it is not retry permission.
4. Worker reply ownership accepts one original completion only. A transfer-consumed exception is not proof of delivery or permission for a second reply. Original result and fault stay retained.
5. Malformed canonical object, ordinary result carrying a canonical request id, foreign worker/activation/header, duplicate and malformed heartbeat/frame each retain their original root without stealing valid ordinary credit.
6. Throw before/after result normalization, pending publication, consumer callback and exact handoff. Preserve original errors plus owned partial output; no cold normalization fallback.
7. Closing/revoked original owners accept only their exact issued retirement traffic; no new command authority. Same-slot worker replacement never inherits old roots.
8. Each slot release follows an exact typed transfer/retirement and separate alias observation; no release from shape, counts, callback return, route deletion or an empty current map.
9. Prove the producer/receiver credit bound for all admitted traffic and declared fixed metadata before claiming a bounded inbox. Arbitrary uncredited flooding remains outside that finite proof.

## Executed Inventory Evidence

The first full actor run03:37:31 completed177PASS/3FAIL180 in2.88s. The generated mixed-traffic case passed. One new source oracle failed because it incorrectly included nested effect-complete/effect-error payload discriminators among top-level worker kinds. The test was corrected to inventory that nested reply scope separately; this was a test scoping defect, not a product RED.

The second run03:38:17 completed178PASS/2FAIL180 in3.47s. Both inbox tests and the existing actual response-module strict TypeScript test passed. The two remaining failures are the previously reported copied-payload/page-boundary tests. Seven selected source/schema/router hashes stayed stable for each run. This is not a whole-tree stable capture of the concurrently changing UI.

The third full run03:40:39 completed177PASS/3FAIL180 in3.24s. Its post-after-observation probe reached the real second result, then failed a harness expectation: the host-created Error crosses a VM realm, so the actual generated error text is String(error), including the Error prefix, rather than error.message. The expectation was corrected to that actual cross-realm branch. This is not a production repair or evidence that two replies are allowed.

The final focused registered ActorWorkerInboxInventory run03:43:25 passed2/178skip180 in1.05s, Nx0, with seven selected pre/post files stable. It executed all of the following against the generated JavaScript:

- a poll awaiting the actual shim storage-read Promise;
- another actor's ordinary checkpoint reply before that poll settles;
- heartbeat and effect-request/effect-emit/ui-patch-emit sharing the callback;
- an effect-complete that resumes the poll without producing a reply of its own;
- an ordinary unknown-request fault and a bootstrap JSPI trap;
- a post callback that first records the actual success envelope and then throws: the worker emits success and then a second fault for the same request;
- an actual guest error whose payload getter throws during replyError normalization: only the heartbeat is emitted, and the generated handler rejects with the secondary normalization value.

The last two are characterized **existing defects**. The neutral trace marks semanticallyAccepted=false; this test must change with the canonical producer cutover and is not a conformance certificate for the one-response protocol. This NodeVM probe does not model browser delivery/transfer success, platform error events, guest Wasm, raw-root retirement or timing.

Exact focused inputs:

| Input | SHA256 |
| --- | --- |
| Shard | 98710401ee3d18c95536fa64a8e7cfabd09e9ba06adf8d745792d5d452376a73 |
| Response module with inventory tests | 7217f95c5b236b950228b771c8413ea50e682a6a1e2151ca77ff6cdde8d472d7 |
| Materializer, unchanged | a246d95516306aa6fdbfb32bcaf8bdf825c685bc20f12eeb09eaa7af5b4c1d5c |
| Inbox inventory schema | 42edaa3e6b42e912c259d3b8ee5904e39583a20c3908048692dbc4b142d0f68b |
| Inbox inventory fixture | 8d02dd1fd5d8db33c8f24eee643a97c317a2d74fd7e94c4c4122644860e4a8f4 |
| Actor script | ecf50673fdc515eba3de67cd47a37e333d1cd061d28233e44083e67b230bf863 |
| Actor Vitest config | c2db1037203c711da2d3af2e7ae600677eb6864de35f05fb0b3f533281124508 |

Logs remain in this ticket: 🧪️worker-inbox-inventory-full-1.log, full-2.log, full-3.log and 🧪️worker-inbox-inventory-focused-1.log. The canonical command is registered by taxonomy as ⚖️gate🎭️actor📬️inbox-inventory at400.995; their publication report is separate evidence, not execution of these tests.

No worker/Shard runtime behavior, materializer, WIT/native schema, generated output, global quota, existing source/input close contract or compiler target changed in this packet. New files are the two inventory JSON files; only tests were added to the existing response module.

## Final Joined Checkpoint

Independent peer checkpoint, reported by the runtime coordinator: UI page9 passed9/698skip707,4.84s at03:47:47, Nx0, with73 selected pre/post hashes identical. The requested imported-source hold was honored and explicitly released at terminal. This is delegated page-admission evidence, not our rerun, reader/stream proof, raw InputAck or receiver-mount approval. No held source was edited during that window.

The full actor rerun03:44:02 completed178PASS/2FAIL180, nine files,3.00s, Nx1. All28 selected pre/post files were stable, including Shard/output, Kernel input, UI resident/pages and imported declaration fixtures, the response module and the new inventory. The existing actual response-module strict TypeScript test also ran successfully; a new whole React strict run was not performed. The remaining failures are exactly the two copied-payload/page-boundary tests, not inbox or cancellation tests.

The full output is retained as 🧪️worker-inbox-inventory-full-4.log. No process remains active from this packet. Receiver API/mount approval, actual fixed producer credit, pre-callback bootstrap ownership and fresh native/browser/all-app acceptance remain open. This closes the requested census, not the demonstrator goal.

### Final Pre/Post Manifest

Every line below was identical at the captured endpoints of the final joined run. This is an endpoint hash assertion over the selected inputs, not a global source lock.

```text
98710401ee3d18c95536fa64a8e7cfabd09e9ba06adf8d745792d5d452376a73  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧵️shard-client.ts
55a104f4877689e212d04aaa925e8ce91a6beeb687a11edbd1126e0bf97c89c6  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🪪️activation/🚪️instance/📥️output/🟦️component.ts
72b0e0ba9bab57f7b95c988c40171e7237b0e5ca00e84b063df03e2c3edd6530  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/💾️resident/🟦️component.ts
5edeb104796ee6c8231bc87648a447cb34fc13e5849a768fad8e78f02165cd51  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🟦️component.ts
a1b249899cdfdef4d0b86fe38c5eae98eb94acda7beee5827c3ad2df9106ac32  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🪪️authority/🧪️fixture.json
78a1ae776c65e4eef4a36a3f1ff7ba28569bb74363214599042d995010b928bd  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🪪️authority/🧪️schema.json
8355c06067bd9669dd5f608f9d24da504cbfd9fc1902e7348c08a2601ef6cd00  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️component.ts
0cbd82c7433272a3f63d1947590dc67e50f61b8a2ee72aad05d0f079b8978e9b  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📄️pages/🟦️component.ts
c6665ba2b2f4d69d292e58290c08a3204df0c7ba3896a80c0477e3fc06611fbd  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/🧪️fixture.json
7372c8b826126375744e453dde9f496b6bdd949974d88feaaf1b9b4999bad583  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🏗️builder/🧪️fixture.json
a0dac544ac8525171b22c0c75a9d506df211c5c817eb9dcd01e58cbf31168605  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📦️payload/🧪️fixture.json
bb65777fe21df516694632dd3ebbb60cb37e5a7588f622bde03d1ea7e25e9ef5  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📨️slot/🧪️fixture.json
3e6e680878c616a06acd6d5a1bdfdbd3a6acbfc510e8eaf83985071218845c56  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🪪️activation/🚪️instance/📥️output/🏘️admission/🧬️schema.json
aceb2a84ec6e05202c55299f22fd283c61a5887b1a5cd4c3ea337053ff6fc797  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🪪️activation/🚪️instance/📥️output/🏘️admission/🧬️contract.json
b738c919a3c18f6a402892899ea8be0c092c087264b1a58a408786e45e9b775d  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🪪️activation/🚪️instance/📥️output/🏘️admission/🧪️schema.json
0d5537911a8182d0b880225ba6dbcf7d6ddd035392ea786e164b556e28eae575  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🪪️activation/🚪️instance/📥️output/🏘️admission/🧪️fixture.json
fd740759266dc3ba8c1a086dd970ba9b4da6ba71e89ee9a97690506f0b1e9766  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🪪️activation/📤️return/🏘️admission/🧬️contract.json
76ee6ef761569101fa6e122c9178721c3b75f708260d868f0bec597efc068dea  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🪪️activation/📤️return/🏘️admission/🧬️schema.json
9f21bd7e7468a091b24c6a03e1f5661a849156b883185fa18450f05d4b5b12e3  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🪪️activation/📤️return/🏘️admission/🧪️fixture.json
942592b42a3c444f663e9319ecc2e1b0b0a23fa934c991f3e251a79ce1a5fb5d  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🪪️activation/📤️return/🏘️admission/🧪️schema.json
5025528c8587982119a02b22d759f2e0b05d7a883da7346d9b47e7029c7a3bd4  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/🎟️credit/📋️metadata/🧬️schema.json
854c22858749d607aeb1c5e7181f98029c77d99b76900db1c07e2872bac074af  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/🎟️credit/📋️metadata/🧪️fixture.json
7217f95c5b236b950228b771c8413ea50e682a6a1e2151ca77ff6cdde8d472d7  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/🟦️component.ts
a246d95516306aa6fdbfb32bcaf8bdf825c685bc20f12eeb09eaa7af5b4c1d5c  /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🌐plugin-web-materialize.ts
42edaa3e6b42e912c259d3b8ee5904e39583a20c3908048692dbc4b142d0f68b  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/🎟️credit/📋️metadata/📥️inbox/🧬️schema.json
8d02dd1fd5d8db33c8f24eee643a97c317a2d74fd7e94c4c4122644860e4a8f4  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/🎟️credit/📋️metadata/📥️inbox/🧪️fixture.json
ecf50673fdc515eba3de67cd47a37e333d1cd061d28233e44083e67b230bf863  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/📜️script.ts
c2db1037203c711da2d3af2e7ae600677eb6864de35f05fb0b3f533281124508  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧪️vitest.config.ts
```
