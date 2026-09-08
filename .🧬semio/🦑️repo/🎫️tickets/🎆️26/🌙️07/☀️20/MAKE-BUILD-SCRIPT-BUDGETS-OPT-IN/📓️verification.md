# Build Budgets Are Opt In

Builds and their generic command, Nx orchestration and dev-server wrappers now have no default wall-clock timeout. Zero means unlimited. Explicit positive `budgetMs`, `SEMIO_BUILD_BUDGET_MS`, `SEMIO_CMD_BUDGET_MS`, `SEMIO_ORCHESTRATOR_BUDGET_MS` and `SEMIO_DAEMON_BUDGET_MS` still impose deadlines. Existing explicit caller/configuration overrides remain supported.

The editor WASM runner already selects `buildBudgetMs()`, so it now waits through Cargo lock contention without the former 1,200,000 ms kill. Shared synchronous runners, captured-process wrappers, asynchronous warm builds and exact Cargo compilation all handle unlimited budgets consistently. Cancellation and output limits remain active in the exact Cargo process runner. Coverage compilation now has its own optional build deadline, followed by assertions under the existing test budget.

## Verification

All focused checks used Bun and Nx with caching disabled:

- `bun nx run @semio-tech/repo-lib:test-process-budgets --skip-nx-cache`: **12 passed**, 0 failed. Covers neutral JSON defaults, positive environment overrides, explicit zero, actual child completion/timeout, synchronous and asynchronous runners, nextest metadata capture, and both coverage runner branches. Captured output and timeout outcomes agree with the existing third-party Execa library. The unlimited capture case completes after 5.2 seconds, proving the wrapper no longer adds an implicit five-second deadline.
- `bun nx run @semio-tech/repo-lib:test-exact-cargo-laws --skip-nx-cache`: **26 passed**, 0 failed. Includes unlimited build admission, positive assertion budgets, actual subprocess cancellation, output limits, and timeout handling.
- `bun nx run @semio-tech/repo-lib:test --skip-nx-cache --args='-t "command budgets"'`: **16 passed**, 0 failed. Existing environment overrides, explicit timeout kills, command statuses, and test-level budgets remain covered.
- Targeted `git diff HEAD --check`: passed.

The new regression suite initially failed on the old finite defaults and on the captured subprocess wrapper's five-second deadline. Exact Cargo tests initially failed with zero-budget timers and finite-default expectations before the implementation changes.

Runtime console evidence included `[DEBUG] unlimited: {"status":0,"output":"build-complete","timeout":false}` and the same successful result after 5.2 seconds, while explicit deadlines reported `timeout:true`. Cargo command routing in the nextest/coverage tests uses actual child processes with a substituted compiler command; a full Rust/WASM or instrumented Cargo rebuild was not run. Installed cargo-llvm-cov 0.8.7 and nextest help confirmed the compilation/execution flags used by the coverage split.

## Broader Check Limitations

The combined command-budget/nextest-filter run passed 17 checks and failed one existing fixture lookup: the test requests `🧪️tests/🏎️nextest/🔣️.schema.json`, while the available schema is named `🧬️.schema.json`. That lookup was not changed.

`bun nx run @semio-tech/repo-lib:lint --skip-nx-cache` failed on existing repository type errors, including imports outside `rootDir`, replication typed-array signatures, styling `ImportMeta` fields, missing Bun types, and the extension manifest `directoryName` requirement. None of the reported errors point to the changed budget logic. Unrelated concurrent edits were preserved.

The new regression target is available in both VS Code launch files as `⚖️gate⏱️process-budgets`. Generated test evidence and command logs were removed after recording these results.
