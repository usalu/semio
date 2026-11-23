# F4 Selection timeoutMilliseconds overflow

Derivation: suites run (`🗂️suites/🏃️run/🟦️.ts:47,70`) passes `stretchedBudgetV1(suite.budgetMs).milliseconds` as `timeoutMilliseconds` to `runOwnedBunFilesV1`, which builds the Selection and validates it (`🟨️bun/📁️files/🟦️.ts`). test-file reaches it via `runProcessTestSuiteV1`, so one path. No margins or per-file multiplication are added.
Root: the two ceilings were two independent literals (Suite.budgetMs and Selection.timeoutMilliseconds, both 1,800,000) joined only by coincidence, and `ceiling = Math.max(milliseconds, CEILING)` let a declared budget above the ceiling through. Nothing guaranteed the Selection bound.
Fix: `STRETCHED_BUDGET_CEILING_MILLISECONDS_V1` is now min of both schemas' maxima (read from the schemas); `Math.max` removed. `runOwnedBunFilesV1` additionally clamps `timeoutMilliseconds` to `OWNED_BUN_TIMEOUT_CEILING_MILLISECONDS_V1` (read from its Selection schema) before building the Selection. Clamped runs stay admitted with F2's "budget clamped" notice (suites run and test-file).
Other paths: envelope deadlines (`📥️invocation`) and `scriptInvocationBudget` use the same clamped function, no Selection.
Law test (contention tests, last test): budgets x ratios (300000x7.5, 300000x80, 1800000x80, ...) produce a funded value that validates against the Selection schema with Ajv.
Tests: contention file 14 pass / 1 fail (pre-existing load-timing "survives a busy host" at 2 s budget); bun/files test 1 pass (one earlier run failed under load, rerun passed 14.8 s).
Open: a live refusal could not be reproduced with the F2 code already in place; if it recurs, the process predates F2's tree.
