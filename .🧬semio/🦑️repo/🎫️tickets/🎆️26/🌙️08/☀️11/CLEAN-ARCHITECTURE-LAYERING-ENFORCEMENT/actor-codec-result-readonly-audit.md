# Actor Codec Result Read-Only Audit

Current authored source inspected only. No tests, compiler, GUI or runtime process launched; no source changes. No current General, Product or Native qualification claim.

## Contracts and Evidence

`🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🧬️component-codec/🟦️.ts:19–31` closes success to Uint8Array, document pair or readonly two-string tuple; Answer is ok or original Uint8Array fault. Public `📦️packages/🟦️typescript/🧬️codec/🟦️.ts` explicitly reexports the defining implementation/types and schema. `ShardClient.codec:2259` remains the one transport method; private packSchemaHash/genesis/printMirror forwards were absent in the inspected symbol search.

The defining JSON schema models language-neutral hex **projections**, rather than the JS typed-array object itself. Request print-mirror pair consequently uses packHex/sprHex in schema and pack/spr Uint8Array in TypeScript. This intentional projection requires explicit conversion, already present in both fixture harnesses; it is not a runtime validation implementation.

Fixture contains seven scenarios (three successes, guest fault, missing codec, unknown operation and host trap) and eight invalid answer projection rows. Its schema constrains exactly seven rows and the exact invalid corpus. General Bun law validates fixture/schema with Ajv, compares byte hex using Node Buffer, and stores observed/expected projections in SQLite. No structural schema defect was confirmed by inspection, but validation has not been executed.

## Actual Source Gaps

- `actorCodecAnswer:55–57` preserves successful return object identity but trusts declared component export result types; there is no runtime admission of malformed success values. A cast/JS component returning number, malformed pair or wrong tuple can escape under the closed return type. Invalid answer fixtures only exercise Ajv projections, not malformed component successes through the function. Determine whether generated binding validation is the authoritative contract before adding a second implementation.
- Fault conversion at 60–62 preserves actual fault view identity for same-realm Uint8Array, but rejects a genuine cross-realm Uint8Array fault. Generated Product VM currently supplies its codec from the outer realm; it does not establish cross-realm guest fault handling.
- General test `🧪️tests/🟦️.ts:43–52` catches both the owner call and subsequent assertions/projection code, translating assertion failures into refusal observations. Outer exact expected comparison usually still fails, but this obscures the primary assertion failure. Catch only owner call rejection; keep assertion failures outside the refusal channel.
- General success cases assert projection equality, not exact success owner identity. Fault case asserts identity. Add success byte/pair/tuple identity evidence if owner preservation is part of acceptance.
- Product generated bridge test admits original bootstrap stock before awaiting `createActorApi`; its `try/finally` begins only after API creation (component-codec-reply source around 80–94). Creation failure can leave that stock admission unreclaimed. The generated worker test closes/returns stock only at its final normal tail (around 151–158), with no surrounding finally, so assertion/dispatch failures leak test-owned stock.

## Registered Coverage and Qualification Limits

Actor permanent script `📦️packages/🟦️typescript/📜️script.ts:42,44` explicitly includes the new General Bun law in both strict typecheck and Bun test lists. Actor project registers uncached `test-product-independent-source` and `canonical-architecture`, both calling this permanent command. General inclusion is confirmed in source, not execution.

Product generated integration is registered through `💻️os/🔨️modules/🔌️plugin/🧪️tests/🎭️actor-transport/🟦️.ts:10,19`, and OS Vitest config `💻️os/🧪️tests/🎚️config/🟦️.ts:26` explicitly includes that registration file. OS package permanent script registers `test` and uses this Product config. The actual Product test route, with SEMIO_TEST_ARTIFACT_DIR supplied, can execute these generated bridge/worker cases; the General Actor source task cannot qualify them.

Bridge test now writes actual `hostShimSource()` and generated bridge beside a stand-in component module. It uses genuine bootstrap admission and performance-derived microsecond clock; closes original source per case and checks live stock count. Worker test executes actual generated source in VM, with mocked WebAssembly and an injected codec API. These establish generated integration when executed; they remain stand-in component tests and do not qualify actual compiled jco actor or Native codec behavior. Awaiting source execution and real artifact probes remains necessary.

## Follow-Up: Superseded Findings and Remaining Scope

Latest read confirms the earlier General broad-catch finding is superseded: the catch surrounds only actorCodecAnswer await, with result projection/schema assertions afterward. Exact public type equality replaces the one-direction conditional guard. The three success owners are preconstructed and asserted by identity; host trap is asserted as the actual thrown object. No new syntax/control issue was confirmed from reading these changes; compiler/runtime evidence remains absent. The printMirror stub still asserts its input inside the awaited owner call, so a failed stub assertion becomes a refusal observation, but the expected success comparison fails afterward.

Earlier Product bridge creation leakage is superseded by a guarded API creation, holding the same original admission/failure if creation fails and closing only when the actual API exists. Worker fixture-loop assertions now have a finally closing/returning original source. **Remaining setup cleanup gap:** originalSource is created before vm.createContext, vm.Script construction/execution, actors injection and missing-dispatcher guard; the try starts only before the fixture loop. Failure in any of those setup operations bypasses cleanup. Begin the cleanup scope as soon as its original source exists. This is a source control-flow finding, not observed execution.

Malformed runtime success enforcement, genuine cross-realm guest faults, compiled whole-producer behavior and Native behavior remain explicitly unqualified. Projection rejection cases do not constitute runtime malformed-answer admission.
