# Final Integration Audit

Read-only source review on 2026-10-13. No compiler tests or runtime reproductions were run by this audit. Existing workstream results are reported evidence, not independently reproduced results. Source can change concurrently.

## Finding: Cargo State and Jobserver Token Order Can Deadlock

Priority P1 for the neutral driver API; current Play WASM jobs explicitly use matching lanes, which avoids this particular collision for those jobs.

Owner: `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🟦️.ts`, lines 367–370, with lines 308–315 and 438–445.

`runPass` acquires the generation/profile Cargo state lease and then awaits a jobserver token. `work`, which also runs jobs, acquires jobserver tokens before spawning the child. A child using `buildCargoArtifacts` borrows inherited host capacity and then acquires that same generation/profile Cargo state lease (`🏗️native-build/🟦️.ts`, lines 213–217). Thus these paths use opposite orders for state and jobserver capacity even after the host/state correction.

A schema-admitted pipeline can contain a component pass and a Cargo job with the same profile and no job lane; lane is optional. With one admitted token, the job consumes the token, the pass acquires state, and the child queues for that state. The pass waits for the job's token; the job waits for the pass's state. The same cycle is possible when jobs/stages reserve the rest of a larger pool. Job launch precedes lane processing, making this a realistic arrival order. The stall watchdog also returns while `stagesRunning > 0` (line 466), so a waiting job prevents automatic stall cancellation.

Use one order everywhere: reserve the pass token before acquiring the Cargo state lease, and release that token if lease acquisition fails. Add a deterministic one-token controlled-port fixture with an unlaned Cargo job and a same-profile component pass. Existing same-lane serialization is useful but does not constrain the neutral API schema or independently inherited compilers.

This finding is established by source control-flow inspection; runtime deadlock reproduction remains necessary before declaring its regression test green.

## Reviewed Boundaries and Limitations

Reviewed architecture and current mode/native/web/Nx workstream records, central Cargo configuration, repository configuration ingress, native capture profile admission, capacity owner, driver API and stage cache/publication. Central configuration supplies native prototype/release; specialized WASM profiles remain manifest-owned by design. No missing central WASM profile finding is asserted.

The stage cache publishes complete entries by private-directory rename and restores by copies with byte verification; it does not hard-link consumers to immutable entries. Native capture acquires host capacity before state. These structural observations are not runtime acceptance.

No source edits, Git mutations, dependency changes, process termination or cleanup were performed. Only this report was created. Full workspace compilation, Windows/Linux behavior, pending Nx original-caller/source-inventory acceptance, final prototype tree-shaking/VSCode regressions and concurrent source stability are outside this audit's independently verified evidence.

Parent resolution: the state/token P1 is fixed and independently runtime-verified by the current same-profile unlaned fixture with one token and Python SQLite oracle. Complete original deadline and cancellation regression gate also passes; see mode-integration.md.
