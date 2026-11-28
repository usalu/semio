# H77: unbounded Original in `📥️parent`, one envelope for `repo test-file` and suite runs

## Decision: the runner (`repo test-file`) was wrong, the test is right
- A real suite run (`repo test`/`repo run <p>:test-<suite>`) goes through the Nx bootstrap `nxBootstrapEnvelopeV1`, whose policy is `orchestratorBudgetMs()` = 0 (unbounded) by default (`SEMIO_ORCHESTRATOR_BUDGET_MS`). Every file process inherits that Original (`runOwnedBunFilesV1` hands on `readIssuedScriptProcessEnvelopeV1`). The suite row (`budgetMs`) never funds the Original; it funds the per-test `--timeout` (stretched) only. No suite row declares "unbounded"; none needs to.
- `repo test-file` built its own bounded Original (`funded stretch + 120 s slack`, e.g. 1.42-1.69 M ms), so the same file saw a different envelope than under `repo test`. That also explains H50's "Original Script deadline refused" class.

## Fix (landed 13:00 slot, lock test-file)
- `🏗️build/🔬️test-file/🏃️run`: Original = `orchestratorBudgetMs()` (same source as Nx bootstrap). The old deadline is kept as a hang guard outside the Original: `withScriptInvocationSignalV1(original, guard.signal, ...)`, aborted after `funded + slack` with the new bilingual text `run.deadline` (`📖️texts`). The failed-row behaviour (deadline error, run goes on) is unchanged (existing test green).
- Law (`🧪️tests`, new fixture `📨️envelope`): the fixture file asserts its Original's `maximumElapsedMilliseconds` equals the orchestrator's (and deadline null iff 0). Run via `runTestFileRoutesV1` and via `runProcessTestSuiteV1` under an envelope created from `nxBootstrapEnvelopeV1`'s policy, for default and `SEMIO_ORCHESTRATOR_BUDGET_MS=7200000`. Both paths pass.
- Tests: test-file tests 20 pass / 0 fail (run locally); `repo typecheck` of 3 files 0 errors.

## `📥️parent`: the 8 unbounded assertions now pass; the file is still 40/8 (different, deeper cause)
- Test-only: `runLeaseReopenSourceCaseV1` bindings for the extracted `openRepositoryLeasePartitionV1` were stale (`join`, `parentSchema`, `lstatSync`, sweep/retirement helpers): added.
- Remaining root: `lease reopening` now fails `expect(readers).toHaveLength(2)` (received 1): the current opener opens the lease image once where the fixture row (`leaseReopenCases`, `opened 2`) models a reopen after the first attempt. Whether the source or the corpus row is right is a product decision about the lease opener (not a budget issue). The other 7 reds are the closed-Original cascade (first failure closes admission). Open, finding for the owner of `openRepositoryLeasePartitionV1`.
- Not verified: the file under a real `repo test` suite run (Nx graph read); local (non-offloaded) run of `📥️parent` times out on a 10 s test at load 45, remote runs (g1-m06/m13) are the representative ones.
