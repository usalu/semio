# Component stage hour and loan deadline (Energy failure, Eurocode attempt 2)

## Cause 1: the hour is a lease wait, not stage compute
- Energy run wRmPw8r: 929 units, 0 failed, then 9 stages for 3809.7 s. About 225 s is real stage work (transpile, wasm-opt, descriptor, probe). About 3584 s is the final publication waiting for the EXCLUSIVE lease `artifact:.cache/b/p/m/browser/prototype/components/19b9d819c199409755908d92`.
- The `[lease] waiting` lines name the holder: pid 26427, `📜️script.ts dev mcp stdio os ...`, started Thu 14:19 (about 20.5 h old). Same holder for Eurocode's `components/4acbc218...` (waiting since about 10:18, sampled at 1900 s).
- Live sample of Eurocode driver (pid 17491): the main thread sits in kevent64, about 1 % CPU, no children, `.stage-1d443167` already holds the complete 248 MB component. It is idle, waiting on the lease, not wedged and not looping. It is not wasm-opt, signing, a fleet link or a loop. I did not kill it.
- Component directories are mutable per-plugin paths. Publishing a changed module needs an exclusive lease on the directory, and the long-lived dev MCP server holds the shared reader lease for its whole life (readers run a body callback that spans serving). Executables avoid this by being content-addressed (`stageExecutableDigestV1`); components are not.
- Unblock: restart the `dev mcp stdio os` server (pid 26427). Readers are the user's; I did not touch it.

## Cause 2: the deadline froze the renewable idle window
- Loan failure = `ResourceLoanDeadlineExhausted` thrown by `remaining()` (invocation/process/🟦️.ts:336) from the publication body control.
- `withAcceptedPhysicalCleanupV1` (stage publication funding) computed the body cutoff from `remaining(original)`. For a renewable `native-child` caller that value is the idle window (3,600 s) minus reserve, so a fixed, non-renewable body cutoff of about 3,595 s was created. Declared lease waits renew the idle window but cannot move a frozen cutoff. 3584 s of waiting plus about 4000 s of compile = 7584 s, matching the failure.
- Sizing: the window is compile-agnostic. Idle 3,600 s, maximum 6 h (`buildPreparationBudgetMs`), but publication was capped at the idle window.

## Fix
- `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts`: `withAcceptedPhysicalCleanupV1` funds from `remaining(original, true)` (nonrenewable maximum only), same principle as `scriptResourceProcessEnvironmentV1`. The idle window still supervises the body through its parent chain and is renewed by declared waits, so progress-driven extension is preserved inside the 6 h maximum.
- Test (fixture + feature + bun test): `.../🏃️process/🧪️tests/⏳️renewable-cleanup-funding/{🔣️.json,🥒️.feature,🟦️.ts}`: idle 1 s, max 60 s, reserve 5 s; asserts body window 50-55 s and body control alive after a 1.5 s declared wait. Passed via `repo test-file` together with the existing cleanup suite.

## Not fixed (needs a decision)
- The wait itself: even with the fix a publication waits up to 6 h for a reader that never leaves. Proper fix is content-addressed component directories (new digest = new path, as for executables) or having the MCP reader release/reload on republish. That is a cross-cutting change in readers (dev MCP, registry, playground session) and was not attempted.
- `publish-chain.sh` RETRY regex does not match `Original resource loan deadline exhausted`, so such a failure is recorded as FAILED, not retried. Running Eurocode loaded the old code and will die at about 3,600 s of waiting (about 11:18) unless pid 26427 is restarted first.
