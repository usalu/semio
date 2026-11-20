# Browser shard worker crossing order

The prior codec test set an internal busy flag and expected an overlap refusal. The worker deliberately queues legitimate crossings for a busy actor. Testing with an actual suspended request exposed a separate defect. Jobs, checkpoint and restore bypassed the worker queue, and segmented download bypassed both the worker queue and the component bridge's queue. A segmented download could enter a guest while its codec call remained suspended. A close requested during a codec or frame crossing could also go unissued.

All ordinary dispatch uses one FIFO lane per actor. Disposal marks the activation immediately and closes it on the same lane after its active crossing returns. Requests queued for that retiring activation fail before reaching the guest. Other actors and effect responses still run while an actor is suspended. The worker records host input at message receipt, so a turn that receives newer input during its first poll yields before a `more-work` drive can delay the queue.

## Verification

The new fixture failed against the original worker when a segmented download reached the suspended guest. All 181 actor-transport tests passed with the fix. They exercise cross-actor progress, FIFO guest crossings, the pending-input drive boundary, stale activation rejection, teardown during a suspended codec and effect settlement.

Chromium independently reproduced the overlap using the actual worker endpoint on Standards Play at port 6034 with a controlled stand-in guest. The served worker returned `download`, then `other`, while the first codec remained suspended. With the generated replacement, only `other` returned while the codec waited. The effect response then released the codec, and the worker answered `first`, `download`, `second` in order. Both tests used page-local interception and read the existing server without restarting it.

The EN 1992 and ISO 16757 shells were checked on the same server with the served and generated workers. Each reproduced its existing guest fault in both runs. EN 1992 reported the staged enum tag error and UI capacity refusal. ISO 16757 reported the staged native source-readiness error. The queue fix corrects a real Play worker defect while those observed guest failures have separate causes.

## Files

- `🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🧵️shard-worker-source/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🧵️shard-worker-source/📨️message-handler/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🧵️shard-worker-source/🔌️actor-loading-and-replies/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️component-codec-reply/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️component-codec-reply/🧫️fixtures/🚦️actor-turn-order.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️actorreturnresponseframing-uses-canonical-vectors-with-no-payload-copies/🟦️.ts`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️10/GET-NORM-PLUGIN-WORKING-END-TO-END/probe-norm-routes.mjs`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️10/GET-NORM-PLUGIN-WORKING-END-TO-END/probe-play-shard-crossing.mjs`
