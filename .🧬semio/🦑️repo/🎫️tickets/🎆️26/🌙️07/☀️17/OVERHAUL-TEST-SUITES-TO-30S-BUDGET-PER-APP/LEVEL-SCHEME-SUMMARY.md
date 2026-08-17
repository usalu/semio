# Leveled Test Scheme — Rollout Summary

Replaces the earlier binary 30s-budget/`test-e2e` split with four ordered, cumulative levels: **fundamental** (15s, default) < **quick** (30s) < **long** (5min) < **exhaustive** (15min). Level L runs everything at levels ≤ L. No unit/integration/e2e distinction — every test belongs to exactly one level.

## Infrastructure (repo/lib/js/index.ts)

- `TestLevel`, `TEST_LEVELS`, `TEST_LEVEL_BUDGET_MS`.
- `resolveTestLevel(segments)` — pops a leading level word or reads `SEMIO_TEST_LEVEL`, sets it as an env side effect, returns `{level, rest}`. **Every project's `TestScript.run` must call this and pass `rest` (not raw `segments`) to its runner** — the single most important wiring rule.
- `runTestBudgeted`/`runCargoTestBudgeted` read the active level for their wall-clock budget and (for cargo) cumulative `--skip <level>::` filters automatically.
- `goLevelTestArgs`/`pytestLevelArgs`/`dotnetLevelArgs` — exported arg-builders for runners without a dedicated budgeted wrapper.
- `testLevelRank()` — for gating vitest/bun-test `describe` blocks in internal (non-published) test files.

## Root / shared config

- `nx.json`: `test-quick`/`test-long`/`test-exhaustive` targetDefaults, `SEMIO_TEST_LEVEL`/`SEMIO_TEST_BUDGET_MS` added as cache inputs (prevents env-only overrides from replaying a stale cache).
- Root `script.ts`: `test [level]` dispatch; storybook board e2e folded into `long`+; `test e2e` subcommand removed.
- Root `project.json`, `.vscode/launch.json`: leveled targets/launch entries.

## Former `test-e2e` projects — converted and verified working end-to-end

- **compose-hub**: Postgres testcontainer suite renamed `mod e2_e_testcontainer_tests` → `mod exhaustive`, `#[ignore]` attributes dropped (9 sites). Verified: ran and **passed** under `exhaustive` in 23.68s.
- **compose-js**: embedded `⚡️FastUnit`/`🐘️WasmE2e` regions now gated by an inline rank check (`long`+) instead of a second env var. Verified: fundamental = 7 tests/351ms, long = 19 tests/66s.
- **compose/client/ui/desktop**: Electron integration suite gated to `exhaustive` only.
- **compose/client/lib/sketchpad/js**: vitest at fundamental/quick via script.ts; Playwright board e2e wired directly at `test-long`/`test-exhaustive` (different toolchain).
- **print**: Tectonic 12-PDF build folded into `long`+; unit tests (parseHex6, resolvePaint, etc.) always run. Verified.
- **repo/client/vscode**: extension-host Mocha suite gated to `long`+.

## Fleet sweep (85 remaining units)

Two workflow runs against the same 85-unit list (the second resuming after the first hit a session cap); both eventually hit **weekly agent-usage limits**, so the fleet stopped short of 100%. Final state, verified directly against the repo (not just agent self-reports):

- **~74/85 fully wired** (`resolveTestLevel` called, `rest` passed through, `test-quick`/`test-long`/`test-exhaustive` nx targets added) — including 3 units the fleet wired in `script.ts` but didn't finish in `project.json` (`cad/module/spatial-shape`, `cad/machine/stately`, `ui/js/react`), fixed directly, and `compose/client/lib/go` (`@semio-tech/compose-go`), wired directly after a separately-run bugfix un-blocked its build.
- **10 units still blocked**, all confirmed to be pre-existing issues owned by other concurrent sessions in this live multi-dev monorepo, not caused by this work:
  - `@semio-tech/repo-lib` — `index.test.ts` imports `FRAMEWORK_OS_PLAYGROUND_PLUGIN_ALIASES`, which `index.ts` doesn't export (mid-flight elsewhere).
  - `compose-grasshopper-tests`, `@semio-tech/compose-grasshopper`, `compose-net-tests` — stale `.csproj` `ProjectReference` paths after an in-progress `cs/` subfolder migration.
  - `@semio-tech/compose-engine` — script.ts reads GraphQL/OpenAPI schema paths one level too shallow (real files live under `compose/client/schema/`, not `compose/client/bin/`).
  - `@semio-tech/gis-3d-rs`, `@semio-tech/gis-2d-rs`, `@semio-tech/framework-graph-rs`, `@semio-tech/raster-rs` — crate directories no longer exist in the working tree (relocated/removed by a concurrent `RELOCATE-...-INTO-FRAMEWORK-SURFACE` ticket); confirmed via direct filesystem search, not found under any new path yet either.
  - `@semio-tech/framework-presentation-core` — `js/` source directory deleted in a Rust-plugin migration commit; only exists in another session's active worktree.
- **Timing measurements were frequently blocked by severe repo-wide cargo build-lock/CPU contention** (dozens of concurrent sessions running this same kind of overhaul simultaneously) rather than by defects — consistent with this repo's documented "Concurrent Cargo Workspace Churn" pattern. Where wiring was confirmed correct by direct inspection/execution (e.g. `fsm-rs`, `graph-dsl`) but a clean timed run couldn't be captured in-session, that's noted rather than treated as a failure.

## Verification performed directly (not just agent self-report)

- `compose-hub:test` (fundamental) → budget guard correctly kills a too-slow run with the level-aware message; `:test-quick` → passes in ~0s (17 pre-existing fixture-drift failures, unrelated); `:test-exhaustive` → Postgres suite runs for real and passes (23.68s).
- `compose-js:test` / `:test long` → 7 vs 19 tests, budget-compliant.
- `print:test` → unit tests only, passes.
- `compose/client/ui/desktop:test`/`:test-quick` → correctly no-ops with a level-appropriate message.
- `compose-go:test` → after a separate bugfix landed, budgeted run passes the timing gate (0.5s), surfaces pre-existing schema-drift test failures unrelated to this work.
- Spot-checked `fsm-rs` and `framework-renderer-react` wiring by direct file read — both correct.
