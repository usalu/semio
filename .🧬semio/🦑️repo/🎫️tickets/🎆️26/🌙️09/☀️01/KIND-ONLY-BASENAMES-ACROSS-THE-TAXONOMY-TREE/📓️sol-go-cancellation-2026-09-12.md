# Go Dispatch Cancellation Tree Correction

## Scope

This correction closes the independent P1 finding in the registered canonical Go test route. It is limited to owned subprocess-tree termination, native `command.Command` context cancellation, nested Go overlay lifetime, and the existing portable dispatch fixture. It does not change Go package selection, definition filtering, progress streams, the package-body policy, Storybook work, generated styling work, Git state, AGENTS files, ticket state, or goal state.

The existing execution routes remain authoritative:

- Bun: `bun <repo-lib>/📜️script.ts test go-test-dispatch`
- Nx: `@semio-tech/repo-lib:test-go-dispatch`
- launch: `⚖️gate🐹️test-dispatch`

No second native or compatibility route was added.

## Red evidence

The independent Terra audit recorded outer Bun PID 81286, inner Bun PID 81631, and detached nested Go PID/PGID 81669. After the outer 15-second budget killed its immediate process group, PID 81669 remained stopped with PPID 1. The audit also found that direct native context cancellation did not return before its 37.5-second observation bound. Those observations are retained in `📓️terra-go-body-audit-2026-09-12.md` and the correction packet `📓️go-cancellation-correction-packet-2026-09-12.md`.

The first local portable-fixture run against the old implementation exited 1. Its direct context case reached the five-second fallback without a prompt natural return, while the old `exec.CommandContext` behavior left inherited output open through a detached descendant. The initial fixture revision also exposed and then corrected two fixture-only issues: the inactive blocking test now skips during ordinary bundle selection, and cleanup excludes the observing Go test process itself.

## Implementation

The neutral TypeScript process owner now snapshots `pid`, `ppid`, and `pgid` from the native process table before terminating anything. It computes only descendants of the registered root PID, kills descendant-created groups from deepest to shallowest, and then kills the remaining owned PIDs. A group is eligible only when its leader PID belongs to the captured owned tree. Windows uses `taskkill /PID <root> /T /F`. No process-name matching or shared-worker lookup is used.

`runTestBudgeted` delegates to that owner, so nested detached groups are closed even when an upstream owner uses SIGKILL and inner signal handlers cannot run.

The Go CLI now starts an external process explicitly, waits for either exit or `command.Command.Context()` cancellation, snapshots and terminates the complete owned tree on cancellation, waits for child closure, and returns the context error. The implementation uses only Go and operating-system facilities. Its Windows branch uses the same recursive `taskkill` contract.

Canonical Go overlays now share one inherited owner directory. Nested dispatchers create separate `run-*` children below it. Canonical Go execution requests throwable failure propagation so its `finally` cleanup runs; the surviving outer owner removes the full overlay tree after forced nested cancellation.

The language-neutral JSON vector and schema now describe a blocking native case with a readiness file and a continuously changing marker. The native Go oracle and Bun harness cover:

- ordinary bundle and exact-definition success with streamed output;
- missing executable spawn error and bounded return;
- native command-context cancellation only after the inner test reports readiness;
- an outer 12-second budget after explicitly suspending the deepest detached Go group;
- every captured descendant PID disappearing;
- marker contents remaining stable after cancellation;
- direct cancellation returning the nested overlay set to its pre-run value;
- nested budget cleanup leaving no `semio-go-tests-*` owner;
- semantic-only source mapping, local replacement behavior, and no standalone semantic Go package.

## Green evidence

### Registered Bun route

`SEMIO_TEST_ARTIFACT_DIR=<ticket>/🗑️generated/sol-go-cancellation bun <repo-lib>/📜️script.ts test go-test-dispatch`

- 2 Bun tests passed, 0 failed, 9 assertions.
- Native Go success, context cancellation, and spawn-error cases passed.
- Context cancellation returned in 242.6 ms in the retained direct run after observing four descendants.
- The stopped nested-budget case observed six descendants across three process groups and returned 10.2 seconds after readiness, within the 12-second total budget.
- The post-run process-table query found no fixture, nested-budget, or overlay process.
- The fixture assertions found no surviving overlay owner and no marker change after cancellation.

### Registered uncached Nx route

`NX_DAEMON=false NX_ISOLATE_PLUGINS=false NX_WORKSPACE_DATA_DIRECTORY=<ticket>/🗑️generated/sol-go-cancellation/nx-workspace SEMIO_TEST_ARTIFACT_DIR=<ticket>/🗑️generated/sol-go-cancellation bun nx run @semio-tech/repo-lib:test-go-dispatch --skip-nx-cache`

- Nx ran the target without a cache hit and succeeded.
- 2 Bun tests passed, 0 failed, 9 assertions.
- Native context cancellation returned in 70.9 ms after observing four descendants.
- The stopped nested-budget case observed six descendants across three groups and returned 9.1 seconds after readiness.
- Nx reported a 21.0-second run duration.

### Focused budget regression

`bun <repo-lib>/📜️script.ts test process-budgets --test-name-pattern 'process budgets preserve explicit async test deadlines'`

- 1 test passed, 0 failed, 12 tests filtered out.
- The explicit asynchronous timeout retained its budget diagnostic and non-zero result.

### Compiler projection and Windows

The live canonical planner maps `⌨️cli/🧵️executor/🐹️.go` exactly once to a virtual root-package input, reports 14 declared packages, and does not add `./🧵️executor` as a standalone package.

The canonical overlay build for `./cmd/repo` passed with `GOOS=windows GOARCH=amd64 CGO_ENABLED=0`. The output was identified as `PE32+ executable (console) x86-64, for MS Windows`, with SHA-256:

`7bfd6773d43a39a61688c0ec20f9a7c27643fb8f1eec07feef23217fc0c810b6`

`git diff --check` passed for every file in this correction.

## Exact source and support files

| File | Bounded correction |
| --- | --- |
| `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🟦️.ts` | Neutral owned process-tree snapshot and termination. |
| `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts` | Budget delegation, throwable canonical failure, and shared nested-overlay owner. |
| `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧩️component/🐹️.go` | Context-aware external-command start, wait, cancellation, and output preservation. |
| `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧵️executor/🐹️.go` | Anonymous semantic Go implementation of native owned-tree termination. |
| `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧬️schema/🚦️test-dispatch/🔣️.json` | Portable cancellation case schema. |
| `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧫️fixtures/🚦️test-dispatch/🔣️.json` | Portable readiness and marker vector. |
| `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧪️tests/🚦️test-dispatch/🐹️.go` | Independent native process-table oracle, context case, spawn error, and nested driver. |
| `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🚦️test-dispatch/🟦️.ts` | Actual registered-route harness and stopped detached-group assertion. |
| `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/KIND-ONLY-BASENAMES-ACROSS-THE-TAXONOMY-TREE/📓️sol-go-cancellation-2026-09-12.md` | Retained correction record. |

## Platform limit

The process-tree and suspended-group runtime assertions ran on native macOS. The Windows implementation was compiled into the amd64 executable, but `taskkill /T /F` was not executed on a Windows host in this lane.
