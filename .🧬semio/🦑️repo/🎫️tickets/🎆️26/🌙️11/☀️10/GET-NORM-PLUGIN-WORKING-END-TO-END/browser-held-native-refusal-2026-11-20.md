# Browser reporting of held native refusals

The Norm guest refused an archive hydration poll with a WIT `semantic` failure. The browser bridge held that failure and its native receipt. Its first error was JCO's generic `[object Object] (see error.payload)`. A later action hit `OriginalOperationSource.issue` and reported `original-turn.source-not-ready`, masking the reason already held by the bridge.

The generated component bridge reports the WIT failure tag on the first poll error and checks the held native error before a queued operation attempts another original turn. It still passes the original receipt and refusal to the existing budget stock, and closure still requires the original return to clear. The current WIT payload exposes the `semantic` tag. The underlying archive hydration fault is tracked separately.

The focused Vitest case failed on the original bridge with JCO's generic message. With the change, the new case and the existing generated-bridge codec fixtures passed. The new case checks identical error messages across two calls, original receipt epochs 1 through 3, one retained refusal object, and the expected close refusal. An existing generated shard-worker test also ran and failed its overlapping-codec assertion. That case uses a direct mock API and does not instantiate this component bridge.

Chromium on Standards Play at port 6034 used the unchanged staged Norm core, and only the test pages received the bridge text from the updated production generator. The original core returned HTTP 200 for both routes. The ISO 16757 and VDI 3805 document-transfer and automatic example action logs now both carry `original-turn.native-return-refused: semantic`. Each shell still stops during document loading. The browser run verified error propagation through the same application path that previously reported `original-turn.source-not-ready`.

## Files

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🌉️component-bridge-source/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️component-codec-reply/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️component-codec-reply/🧫️fixtures/🚫️native-refusal.json`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️10/GET-NORM-PLUGIN-WORKING-END-TO-END/probe-norm-catalogue-examples.mjs`
