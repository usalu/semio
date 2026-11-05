# Norm Browser Scenario Source Review

Read-only review of the new norm time-travel scenario, witness JSON/schema, contract tests and feature. No build, browser or server launched.

## Concrete Import Blocker

The scenario imports mirrorLocator from @semio-tech/framework/time-travel/testing/wgpu-mirror. Its current public facade at 🧰️framework/📦️packages/🟦️typescript/⏪️time-travel/🧪️testing/🧊️wgpu-mirror/🟦️.ts does not export that name, although the underlying owned time-travel module defines/exports it. This blocks module loading even for the React journey because it is a static named import. Reexport the existing owned helper through the selected facade, or use an already-public equivalent; do not import private paths. All other named framework harness imports in this scenario were found in their explicit facades. TypeScript AST parse diagnostics for the scenario: zero. No full semantic compilation was performed.

## Source-Owned Witness Coherence

The canonical Bridge SLS address scope:1:9:bridgeSls6:sls-D1 matches native row_address encoding. The numeric row norm-inputs.bridgeSls[id=sls-D1].deckAcceleration and .input path match quantity control construction; Deck acceleration is a quantity input committed on blur. The scenario fills the actual numeric input and moves focus with Tab after Enter, so it reaches that commit boundary. The norm- bootWindowPrefix matches current norm window IDs.

The witness check ID correctly includes deck-D1.sls-D1, while subject ownership is sls-D1. Selecting the SLS row rather than member deck-D1 avoids the earlier owner-filter hazard. Before/after 0.2/0.4 divided by limit0.5 yields utilization0.4/0.8. Reading the main row 40%/80% label is correct: pass rows do not necessarily own expanded .utilization children. The field and same-check conjunction avoids mistaking a changed input alone for engineering recomputation.

## Settlement Acceptance Gap

reload calls await reprojectionSettled(60000) and discards its result. The helper returns {ok:false,waitedMs} on timeout rather than throwing. Therefore this call does not assert that the restore finished. Later field/history comparisons may still fail, but the scenario should record the settle verdict and stop/defer dependent assertions when it is false. The attachFolder return likewise reports card/input/button availability, not successful archive admission; current later state/history assertions are the stronger evidence and should remain.

## Selector Precision Risk

selectOwner scans every ID ending in the canonical owner and picks the shortest, without scoping to the outliner window or requiring an interactive entity row. The same shared collection renderer can render entity rows in the inspector. If both are mounted with the same local address, key-length ordering alone does not establish which row is clicked. Scope selection to the actual outliner window/body or require its authored interaction/granularity identity. This is a source-level ambiguity requiring browser observation, not a confirmed failure in the current DOM.

The contract tests validate a closed bilingual fixture and independent arithmetic but do not load the scenario module or check public imports. They cannot catch the concrete missing mirrorLocator export. No executed browser acceptance or semantic type-check result is claimed by this review.

## Current Recheck After Parent Repairs

Parent removed the unavailable mirrorLocator public import, keeping exact mirror/DOM lookup local to the scenario, and added an explicit verdict for reprojectionSettled.ok. The contract suite now imports the actual scenario, closing the earlier fixture-only import gap. A read-only TypeScript compiler Program checked this current scenario source with strict=true, ESNext/Bundler module resolution, JSON imports, noEmit and skipLibCheck; source syntactic and semantic diagnostics were both empty. This is scoped source semantic checking, not a whole-repository build.

Current source-owned numeric IDs, canonical SLS owner, boot prefix and percentage reading remain coherent. No further confirmed source/API/type error was found. The unscoped shortest-owner-row selector remains an ambiguity to resolve through actual DOM observation or an explicit outliner constraint; it is not a demonstrated failure.
