# Native build driver: fail fast on the first job failure

## Problem
`runNativeBuildDriverV1` awaited `Promise.all` of the job completions, which rejects at the first failure, but the rejection only ran the final `dispose`. `dispose` terminates the children that exist at that moment, then drains `pending`; jobs still queued or between capacity admission and `spawn` started afterwards and ran to completion (one took 30 minutes) before the failure was reported. No cancellation reached them.

## Change (`🧩️component-pass/🟦️.ts`)
- On the first job rejection the driver snapshots the failed jobs, calls the existing `stop(error)` (the stall-watchdog / deadline path: sets `stopped`, terminates every owned child tree), drains `pending` with `allSettled` (join, no orphans), and throws `Native build job failed (remaining jobs cancelled): <job ids>: <original message>` carrying the original error as an `AggregateError` member.
- `check()` right before `spawn` in the job executor: a job that passed admission after the stop never starts a child. Queued jobs already hit `check()` in `invoke`.
- Successful runs and component/pass failure semantics unchanged.

## Test
`🧪️tests/🟦️.ts`: "a job failing at once rejects the driver, naming it, and terminates the sibling job still running": a job exiting 3 at t=0 plus a sibling sleeping 120 s; asserts rejection naming `job:broken` within 30 s and the sibling pid is gone.

## Addendum: first-failure log and the restart protocol (exit 75)
- The first job failure is now logged at once (`[native-build] job:<id> job: <message>`) before the cancel; the aggregate error follows.
- Exit 75 is the repository's "restart me with a new snapshot" status: a nested `repo` command that finds a peer editing its inputs prints "Another process kept editing ... starting the command again ... (n of 6)" and only yields 75, leaving the re-run to the outermost command (`runWithStaleReexecV1`, `🧷️format-stamps/🔄️reexec/🟨️.mjs`; constants in `🧷️format-stamps/🧬️schema/🔣️.json`: `exitCode` 75, `maximumReexecutions` 6, `editFailure.backoffMs`). The driver is that outermost command for its jobs, so it did not honour the yield and failed the whole run (describe-tool).
- `runJob` now restarts a job whose exit is `formatStampProtocolV1().exitCode`, up to `maximumReexecutions` times, pausing `editFailure.backoffMs[n]` (abortable by the build signal) and logging each restart. No constants are duplicated. Any other exit, or exhausted restarts, fails the job and triggers fail-fast. Each attempt re-runs under the same capacity slot and writes a fresh `job-<id>.log`.
- Cargo pass children (`runAdmittedPass`, unit graph) are plain `cargo` processes with the scheduler as RUSTC_WRAPPER, not `repo` commands; 75 is not a Cargo status, so a 75 there is a real failure and is not restarted. Jobs whose command is `cargo` share the job runner and are covered.
- Test: "a job that exits with the shared restart status runs again ..." (first run exits 75, second succeeds; `starts.log` shows two starts). Runs only through `repo run @semio-tech/framework-process:test-native-build`.
