# Result E78 (tests-real-modules-2)

## 1. `reading` (dependencies storage): 67/18 -> 84/0
Cause was neither a product bug nor fixture drift for the named vector. `neutral dependency completion actual all-scope barrier preserves partial failures` runs 31 producer vectors inside the `sdk-fence` budget (25 s body). At load 45-55 the body deadline hits around vector 24 (`preserving-payload-metadata-drift`), the abort surfaces as the refusal at `dependencies:ready-write`, and the closed Original owner then fails every later test (`OriginalAdmissionClosed ... first failure`). Proof: with a temporary filter to that single vector the test passed; with all vectors under `sdk-fence` it failed at 25.7 s; under `sdk-runtime` (110 s) it passes. The first law (`dependency current plan diagnostic owner`) had the same 10 s `caller-birth` deadline problem on `dependencies:plan-worker` and moved to `sdk-runtime` too.
Behind the cascade were real drifts, fixed:
- `bootstrap pressure` and `selected retirement` extracted `observeRepositoryDependencyBootstrapCapsuleV1` / `retireRepositoryDependencyBootstrapCapsuleV1` as text; both now call the new module-level `requireRepositoryDependencyContainerV1` (ReferenceError). Migrated both to `loadProductFunctionsV1`; ceiling 36 -> 34.
- `all-scope projection preserves borrowed slots` asserted an empty cache after planning; `javascriptDependencyBindingsV1` now holds its generation (`hold` directory, E58, intended: "an installer holds its generation from the first byte"). The test now allows exactly the policy's hold directory and nothing else.

## 2. analysis-pair and memo
- analysis-pair 3/1 -> 4/0. Debug run printed the failing observations: two `owner-registry` global facts. The migration caused it: `evaluateExtractedFunctionV1` resolves unlisted free names to a pass-through stub, so `askOwnerRegistryV1` was a no-op before; the loader runs the real helper, which records registry facts for the fixture registry that can never hold against the live one. Fix: explicit port `askOwnerRegistryV1:()=>undefined` (the test is about the single Source image). NOT run: the HEAD text on a scratch copy (outside the worktree `ajv` does not resolve and `repo test-file` refuses outside paths); the causal proof is the loader/stub source plus the observation list.
- memo 3/1 -> 4/0: not caused by the migration; stale expectation. The refusal message in a language-less run is now `refused process.source.crate-refused {...}`; the test expects the key.

## 3. Migration batch 2 (product code needed no change)
Real-module loader now used by: go-plan members, store worker values, cargo custody encoding, cargo fetch-order, script-inputs verdict-wait, bun workspaces frontier (CommonJS: `require` supplied as a port via `createRequire`), inputs quota observation, nx funded retirement. Ceiling 257/770 -> **249/760** (law test 11/0). All target tests green.

## NOT migrated / needs product work (found)
- go-plan lease, pruning workspace, native-orchestration environment, inputs storage lifetime: they substitute a dynamic `import()` in the function text or run the text through several compilers on purpose; they need a declared port for the dynamic import (loader or product) first.
- fixed receipt `recheckPhysical`: a closure inside a function; lift to a named module-level function.

## NOT verified
- Reading ran only at load 41-55; at lower load a retry of `sdk-fence` would probably pass but the 31-vector law is too slow for it by design.
- Before-hashes captured only for reading, analysis-pair, memo.
- Not re-run: dependants of the migrated files beyond the files themselves.
