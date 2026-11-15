# Cargo test killed at about one hour while still compiling

## Root cause

The progress-aware machinery already existed: `⏱️budget` declares `ProgressDeadlinePolicy` (idle window 1 h, nonrenewable maximum from `BuildPreparationDefault` = 6 h, renewal by child CPU time, child output, declared waits), and `withRepositoryCompilerStoreV1` wraps the whole compile in a renewable control.

The break was at the last hop. `launchRepositoryCargoV1` (`🛍️products/🦑️repo/.../🏃️process/🦀️cargo/🚪️launcher/🟦️.ts`, stream path) read `remaining` from that renewable control once at spawn and passed the number to `runOwnedCommand(..., timeoutMs)`. `runOwnedCommand` armed `startOwnedCommandTimeoutV1`, a fixed monotonic wall clock. The renewable idle window (3 600 000 ms, minus about 20 s of setup = 3 579 610 ms) was frozen into a hard deadline, so output and CPU progress could not renew it. Message source: `execution.timed-out` in `🏃️process/📖️texts/🔣️.json`, exit code 124.

The same snapshot pattern (`compilerAdmissionBudgetV1(control)` as `timeoutMs`) existed at 31 more call sites (bundle, vite, storybook, bazel, typescript, go-plan, dashboard, graphql, wgpu, dotnet, ...).

## Fix

- `runOwnedCommand` (`🏃️process/🎛️owned-execution/🟦️.ts`) gets `options.deadline`: a live `ResourceLeaseControl` supervised with the existing `watchResourceLeaseControlDeadlineV1`, which re-reads the control whenever it believes the deadline is due, so renewals move the expiry. The fixed `timeoutMs` stays for genuinely fixed budgets (0 = none).
- Distinct refusals (exit class timeout, 124): `execution.progress-expired` (carries the bilingual idle or maximum notice) and `execution.deadline-exhausted`; a cancelled control stays a cancellation. Texts en and de in `🏃️process/📖️texts/🔣️.json`.
- Cargo launcher stream path and `runNxSetupCommandV1` pass `0` plus `deadline: control`. 31 further sites converted mechanically; unused imports removed.
- Limits stay in schema: `⏱️budget/🧬️schema/🔣️.json` (idle window, 6 h ceiling via `BuildPreparationDefault`, `SEMIO_PROGRESS_DEADLINE_IDLE_MS` override). Progress signals: stdout and stderr data (`noteScriptProcessActivityV1`), child CPU sampling, declared waits. Cancellation is unchanged (control signal, SIGINT, SIGTERM).

## Tests

`🏃️process/🎛️owned-execution/🧪️tests/⏳️deadline/🟦️.ts` with language-agnostic table `🔣️.json`: renewal by output outlives the window, silence stops with the progress notice, an unrenewed finite deadline stops as exhausted, cancellation wins.

The next `repo cargo` run loads the new code; the running build is unaffected.
