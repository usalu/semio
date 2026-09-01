# End-to-End Automated Gates for "S Works End to End"

## 1. Existing Automated Gates and Tests

### Launch Configurations

**`.claude/launch.json`** — "s-react" dev entry
- **Name**: `s-react`
- **Command**: `bun ./📜️script.ts dev s`
- **Port**: 6070
- **Purpose**: Dev server launcher for the S studio shell (port 6070)

**`.vscode/launch.json`** — OS collaboration e2e gate
- **Name**: `⚖️gate🌎️collab-e2e`
- **Command**: `bun nx run @semio-tech/framework-os-dev:collab-e2e`
- **Purpose**: Automated end-to-end test gate for OS (framework-os-dev project)
- **Status**: Referenced but target definition not yet found in project.json

### Test Scripts (package.json)

- **`test`**: `bun ./📜️script.ts test` — General test runner (fundamental level, 15s timeout by default)
- **`test:storybook`**: `bun nx run workspace:test-storybook` — Storybook Playwright specs against static build
- **Level-based test variants**: `test-storybook` has implicit quick/long/exhaustive variants via `SEMIO_TEST_LEVEL` env var

### Storybook Playwright Test Suite

Located in `.storybook/*.spec.ts`, runs via:
```bash
bun run test:storybook  # or: bun nx run workspace:test-storybook
```

**Config**: `.storybook/playwright.config.ts`
- Runs 10+ Playwright specs serially against single static Storybook server
- Software rendering: `--use-angle=swiftshader --enable-unsafe-swiftshader --enable-unsafe-webgpu`
- Base URL from `PLAYWRIGHT_BASE_URL` env var or defaults to `http://127.0.0.1:6010/`

**Test Files Relevant to OS/Plugins**:
1. `.storybook/os-plugins.spec.ts` — Per-plugin boot matrix for framework/os scope
   - Tests every `PLUGIN_BUILD_TARGETS` entry
   - Waits for readiness beacon (`data.semioOsReady` / `data.semioOsError`)
   - Verifies zero unexpected console errors
   - Timeout: 60s per plugin (via `READY_TIMEOUT_MS`)

2. `.storybook/framework-hosts-wasm.spec.ts` — WASM plugin boot and DOM readiness
   - Verifies page loads cleanly without errors
   - Checks `#storybook-root` is populated
   
3. `.storybook/puzzle-3d-5d-infinite.spec.ts` — Infinite/WASM story boot patterns

4. `.storybook/cad-renderer.spec.ts` — CAD rendering verification

### OS Dev Project Tests

**Package**: `@semio-tech/framework-os-dev` (at `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/`)
- Includes Playwright and Vitest in devDeps
- Multiple dev/build/test scripts defined
- The `collab-e2e` target is referenced but definition not yet found

## 2. What Currently Reports for S

### Port 6070 Server is Ready
- Dev server on port 6070 accessible via `bun ./📜️script.ts dev s`
- Server is live but no automated gate has been wired to verify full e2e flow

### Storybook Specs Cover Plugin Boot, Not App Flow
- `.storybook/os-plugins.spec.ts` verifies each plugin artifact boots and reaches readiness
- Does NOT verify: user interactions, multi-pane orchestration, data flow between components
- Scope: Artifact availability + DOM readiness + console errors

## 3. What's Missing for S E2E Coverage

### No App-Level E2E Tests
- No Playwright specs that:
  - Navigate to `http://localhost:6070`
  - Perform end-to-end workflows (e.g., create object → modify properties → save state)
  - Verify inter-pane communication
  - Assert side effects (storage, DOM updates, agent interactions)

### No Demonstrator-Style Acceptance Tests Yet
- The DEMONSTRATOR-END-TO-END-ALL-APPS ticket (dated 2026-08-28) proposes a framework:
  - **Plan file**: `📓️plan-acceptance-test.md` (35.8 KB, detailed Playwright patterns)
  - **Key insights**:
    - Use deterministic readiness beacon scoped per pane (currently global; collides with 6+ simultaneous panes)
    - Stable selectors via `data-shell-id`, `data-demonstrator-list-scroll`, test IDs
    - Reusable helpers: `expectStoryLoads()`, `pluginArtifactAvailable()`, `waitForOsBeaconOrArtifactMissing()`
    - Single static server + Playwright serial execution pattern (same as Storybook)
  - **Critical blocker noted**: Readiness beacon (global `document.documentElement.dataset.semioOsReady`) cannot distinguish "all 6 panes ready" — needs per-shell scoping fix before multi-pane e2e is viable

## 4. Test Level Budget System

All tests respect `SEMIO_TEST_LEVEL` env var and per-level timeouts:
```
fundamental: 15 seconds (default)
quick:       30 seconds
long:       300 seconds
exhaustive: 900 seconds
```

Overridable via `SEMIO_TEST_BUDGET_MS` env var.

## 5. Recommendations for S E2E

1. **Use Demonstrator ticket's acceptance-test pattern** as template
2. **Scope readiness beacon fix**: Make `data.semioOsReady` per-shell instead of global
3. **Create `.playwright/s-e2e.spec.ts`** or similar:
   - Port 6070 boot verification
   - Basic interaction flows
   - Storage/state persistence checks
4. **Wire into CI gates**: Add target to framework-os-dev project or top-level test matrix
5. **Test Level**: Start with `fundamental` (15s), scale to `long` (300s) as coverage grows
