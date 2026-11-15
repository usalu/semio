# fstatat race in the relative directory descriptor

## Root cause (confirmed by reproduction, not by the service log)
- The service log prints only the stack of `Relative filesystem fstatat failed` (no errno), reference 65e48b6c, at 10:37:17Z and 17:48:32Z.
- The throwing code is `readEntry` in `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts` (line 153). It is the descriptor-relative entry observer used by the cache stores (dependency store, tools, budget store), not the worktree walker. ENOENT on `fstatat` was already mapped to "absent", so the failing errno was something else.
- Reproduction: opening the directory and calling `entry(name)` in a loop while three `sh` loops create/rename/delete files, dirs and symlinks produced, once in 262890 observations, `Relative filesystem fstatat failed` with code `Unknown system error -316`: errno 316 is not a real errno, so the errno cell read after the failing call was not the one the syscall set (transient/unreliable errno under churn). The same churn produced further plain, non-retryable errors on the same path: `readlinkat` ENOENT and EINVAL (symlink removed or replaced after the stat), `Current Darwin relative entry descriptor changed`, and `Original relative entry facts changed`. All were plain `Error`s, so the churn layer (`🔁️churn`, `SEMIO_CONCURRENT_EDIT`) could not retry them and the request failed.
- Caveat: the production errno itself was never logged, so "unknown errno" is the best-evidenced explanation (same call site, same failure message), not a proven one.

## Fix
- `settle` helper: a failing native call whose errno is EINTR or not a known errno is repeated up to `RelativeProtocol.maximumErrnoAttempts` (4, in the schema), calling the cancellation checkpoint between attempts. Used for `fstatat`, `openat` and `readlinkat` in `readEntry`.
- `readlinkat` ENOENT means the entry vanished (absent); EINVAL and the two identity-mismatch checks now raise `concurrentEditV1("identity")`, so the churn layer re-observes with its bounded attempts and cancellation. Other errnos stay loud.
- `failure()` now puts the errno name in the message, so the next occurrence shows it in the service log.
- No fence weakened: identity and directory checks are unchanged; only their failure type changed.

## Files
- `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts`
- `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🧬️schema/🔣️.json`
- `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/🧪️tests/📁️descriptor/🟦️.ts` (new test: entries that vanish or reappear under a live observer)

## Verification
- `repo run @semio-tech/filesystem-observation:test-typescript --skip-remote-cache`: the new test failed with the plain errors above and passes after the fix (46 pass, 2 fail). The 2 failures are in `🔁️churn/🧪️tests/🧾️refusals` ("Stale loaded Graph reader policy; restart required" has no site in `build/🕸️graph/🟦️.ts`), a concurrent change by someone else, unrelated to this fix.
- The wrapper command reported exit 124 (my `timeout`) because the run stayed in "Verifying build tools" after the tests finished; the test results are from the test output.
- The new test is a bounded 4 s churn stress, not a deterministic injection (the module has no errno seam); the unknown-errno case itself reproduced about once per 260k observations and is therefore not guaranteed to trigger.

## Service check
- `repo service host stop` (exit 0), then `repo run @teaching/architecture-course:check --skip-remote-cache`: exit 0, 1 task up to date (plans and runs on the restarted service; cached result, so the check body itself did not re-execute).
