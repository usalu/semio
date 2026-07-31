# @semio-tech/repo-coordinator — test budget report

## Unit
- Nx project: `@semio-tech/repo-coordinator`
- Root: `repo/server/coordinator`

## Finding: zero pre-existing tests
- No `#[cfg(test)]`/`*.test.ts`/`*_test.go` files exist anywhere under `repo/server/coordinator` (checked `go/main.go`, all of `js/**`). Nothing to classify or delete.
- The `test` target was NOT wired through `script.ts` at all: `project.json` called the raw shell command `bunx vitest run` directly via `nx:run-commands`, bypassing both the repo's `script.ts`-only convention and the shared `runTestBudgeted`/`runVitest` budget infrastructure entirely.

## Bug uncovered by baseline measurement
Running the pre-existing target failed outright:
```
Startup Error
Error: Projects definition references a non-existing file or a directory:
/Users/ueli/Documents/semio/repo/server/coordinator/cad/core/js/vitest.config.ts
```
Cause: bare `bunx vitest run` (no `--config`) with `cwd: repo/server/coordinator` and no local vitest config climbs up and picks up the monorepo-root `vitest.config.ts` (a `projects: [...]` aggregator of ~23 packages). Vite/Vitest's default `root` is `process.cwd()`, not the config file's own directory, so every relative project path in that root config (e.g. `./cad/core/js/vitest.config.ts`) got wrongly re-resolved against `repo/server/coordinator/...` instead of the repo root — a guaranteed failure for any bundle invoking plain `vitest`/`bunx vitest` without an explicit `--config`. This is exactly what the shared `runVitest()` helper in `repo/lib/js/index.ts` exists to prevent (it always passes an explicit `--config`).

## Fix applied (mechanical, matches repo convention, e.g. `cad/core/script.ts`)
- `repo/server/coordinator/script.ts`: added a `TestScript extends BundleScript` that calls `runVitest(this.root, segments, "js/vitest.config.ts")` (imported from `../../lib/js/index.ts`, already budget-enforced via `runTestBudgeted`); registered it as `"test"` on the router.
- `repo/server/coordinator/js/vitest.config.ts`: new minimal config (`root` scoped to `js/`, `environment: "node"`, `passWithNoTests: true`) — required because `runVitest` always passes an explicit `--config` path, and none existed for this bundle.
- `repo/server/coordinator/project.json`: `test` target command changed from `bunx vitest run` to `bun ./script.ts test` (added `"dependsOn": []` to match sibling projects, e.g. `cad/core/project.json`).

## Before / after
- Before: crash (`Startup Error`, non-zero exit) — not measurable as a pass/fail time, effectively "broken," not merely slow.
- After: `bun nx run @semio-tech/repo-coordinator:test --skip-nx-cache` passes, ~3.4s wall clock (`No test files found, exiting with code 0`). Well within the 30s budget. Warm nx-cache reruns land under 3s.

## Test counts
- Before: 0 tests (target didn't even execute).
- After: 0 tests (none exist in this bundle; nothing was deleted since nothing trivial/complicated existed to begin with).

## Removed
- Nothing removed — there was no existing test content in this unit to trim.

## Notes
- No `test-e2e` split was needed/added: there is no e2e/integration suite here (no server/container/browser spin-up in the test path) — the prior target was simply mis-wired, not slow.
- Did not touch `go/main.go` (no Go tests present) or add any new tests — out of scope per instructions (no new test files to be created; only classify/trim existing ones and fix the runner wiring).
