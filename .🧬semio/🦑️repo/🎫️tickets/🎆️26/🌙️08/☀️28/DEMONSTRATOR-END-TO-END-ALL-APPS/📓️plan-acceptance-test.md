# Plan: Automated Browser Acceptance Test for the "Entwerfen mit Bestand" Demonstrator

Scope: `♻️mit-bestand/🧺️demonstrator/` (dev server `bun nx run @semio-tech/mit-bestand-demonstrator:dev`, port 6029, six panes: generator, koordinator, aggregator, aussuchen, bearbeiten, verfolgen). Read-only investigation; no files besides this one were modified.

---

## 1. Existing Playwright setup in `.storybook/`

**Config** — `/Users/ueli/Documents/semio/.storybook/playwright.config.ts:1-51`:
- Header (lines 1-6) states the invocation contract verbatim: *"`bun run test:storybook` builds, serves `storybook-static/` via `script.ts dev storybook-static`, then runs Playwright against every `*.spec.ts` in this directory with `PLAYWRIGHT_BASE_URL` set; this config does not start its own server."*
- `testDir: storybookDir; testMatch: ["*.spec.ts"]` (line 27-28) — every `.storybook/*.spec.ts` file is picked up automatically (no per-file registration needed).
- `timeout: playwrightTimeoutMs` (line 32) and `expect: { timeout: Math.min(playwrightTimeoutMs, 120_000) }` (line 33) both come from `playwrightTestTimeoutMs()`.
- `fullyParallel: false, workers: 1` (lines 29, 34) — specs run serially against one shared static server.
- `baseURL` (line 24) resolves from `PLAYWRIGHT_BASE_URL` env var, falling back to `http://127.0.0.1:${STORYBOOK_PORT ?? 6010}/`.
- Chromium project launches with `--use-angle=swiftshader --enable-unsafe-swiftshader --enable-unsafe-webgpu` (line 46) — software GL/WebGPU so canvases render headlessly in CI.

**`playwrightTestTimeoutMs()`** — `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts:1208-1210`:
```ts
export function playwrightTestTimeoutMs(level: TestLevel = activeTestLevel()): number {
  return testLevelBudgetMs(level);
}
```
This chains into the repo's global test-level budget system (same file, lines 1115-1163):
- `TEST_LEVELS = ["fundamental", "quick", "long", "exhaustive"]`
- `TEST_LEVEL_BUDGET_MS = { fundamental: 15_000, quick: 30_000, long: 300_000, exhaustive: 900_000 }`
- `activeTestLevel()` reads `process.env.SEMIO_TEST_LEVEL`, defaulting to `"fundamental"` (i.e. **15s per-spec timeout** unless a level is explicitly requested).
- `resolveTestLevel(segments)` (lines 1139-1147) consumes a leading level word off the CLI segments (or falls back to `SEMIO_TEST_LEVEL`), sets `process.env.SEMIO_TEST_LEVEL` for all child processes, and returns the rest.
- `SEMIO_TEST_BUDGET_MS` env var overrides the level's budget outright (`testLevelBudgetMs`, line 1162-1164).

**Wiring, root → leaf**:
1. `package.json:125`: `"test:storybook": "bun nx run workspace:test-storybook"`.
2. `📋️project.json:421` target `test-storybook` → `nx:run-commands`, `"command": "bun ./📜️script.ts test storybook"`, env `STORYBOOK_PORT: "6010"`.
3. `📜️script.ts` `TestScript.run()` (line 18962-18967): `rest[0] === "storybook"` → `await this.runStorybookPlaywright()`.
4. `runStorybookPlaywright()` (`📜️script.ts:19082-19107`):
   - Picks a free static-server port near `STORYBOOK_PORT` (`pickStorybookStaticPort`, lines 19069-19074, scanning a 50-port span via `isTcpPortFree`).
   - `runCmd("bun", [...,"build","storybook"], ...)` — builds the static Storybook first (`build storybook` → `bunx storybook build -c .storybook --output-dir storybook-static`, `📜️script.ts:19747-19751`).
   - `spawnDaemon("bun", [...,"dev","storybook-static"], { env: { STORYBOOK_PORT: storybookPort } })` — starts the static file server (`DevScript.runStorybookStatic()`, `📜️script.ts:19140-19168`, a plain `Bun.serve` static file server).
   - `waitForUrl(new URL("🌐️index.html", baseUrl).href, 120000)` (private helper, `📜️script.ts:19042-19048`) — polls with `fetch`, retrying every 500ms, throwing after the timeout.
   - `runCmd("bunx", ["playwright","test","--config",".storybook/playwright.config.ts"], { env: { PLAYWRIGHT_BASE_URL: baseUrl, PLAYWRIGHT_BROWSERS_PATH, STORYBOOK_PORT } })`.
   - `finally { server.kill() }`.

This is the exact pattern to imitate for the demonstrator (Section 6 below), because the demonstrator is **not** part of the Storybook build — it's a standalone Vite app with its own dev server and port.

---

## 2. Patterns for waiting on WASM plugin boot / asserting DOM readiness

Three specs share (with small variations) one base assertion helper — page loads, no page/console errors, `#storybook-root` populated:

- `.storybook/framework-hosts-wasm.spec.ts:20-38` (`expectHostStory`)
- `.storybook/puzzle-3d-5d-infinite.spec.ts:16-35` (`expectStoryLoads`)
- `.storybook/os-plugins.spec.ts` uses a **different**, beacon-based helper (below) because it specifically needs to distinguish "ready" from "error" from "artifact intentionally missing".

**Reusable "story loaded cleanly" helper** (`.storybook/framework-hosts-wasm.spec.ts:15-38`):
```ts
function significantConsoleErrors(messages: string[]): string[] {
  return messages.filter((text) => !/Failed to load resource:.*\b404\b/i.test(text));
}
async function expectHostStory(page: Page, storyId: string): Promise<void> {
  const pageErrors: Error[] = [];
  const consoleErrors: string[] = [];
  page.on("pageerror", (error) => pageErrors.push(error));
  page.on("console", (message) => { if (message.type() === "error") consoleErrors.push(message.text()); });
  await page.goto(`iframe.html?id=${storyId}&viewMode=story`, { waitUntil: "domcontentloaded" });
  await expect(page.locator("body")).not.toContainText("Couldn't find story matching");
  await expect(page.locator("body")).not.toContainText("Failed to load the Storybook preview file");
  await page.waitForFunction(() => {
    const root = document.querySelector("#storybook-root");
    return !!root && root.childElementCount > 0;
  });
  expect(pageErrors.map((error) => error.message)).toEqual([]);
  expect(significantConsoleErrors(consoleErrors)).toEqual([]);
}
```
`puzzle-3d-5d-infinite.spec.ts:16-35`'s `expectStoryLoads` is byte-for-byte the same idea (percent-encodes emoji story ids, and widens the 404 filter regex to `\b40[0-9]\b`). Both patterns generalize directly to the demonstrator: swap `page.goto("iframe.html?id=...")` for `page.goto("/#<paneId>")` and swap `#storybook-root` for the pane's own container (`[data-shell-id="<paneId>"]`, see §3).

**Deterministic readiness beacon** (`os-plugins.spec.ts:29-46`, sourced from the shell's own `#region 🔖️ReadinessBeacon`, `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx:6891-6916`):
```ts
useEffect(() => {
  const root = document.documentElement;
  const beaconId = pluginFilter ?? "unknown";
  if (notFound) { root.dataset.semioOsNotFound = beaconId; delete root.dataset.semioOsReady; delete root.dataset.semioOsError; }
  else if (error) { root.dataset.semioOsError = beaconId; delete root.dataset.semioOsReady; delete root.dataset.semioOsNotFound; }
  else if (session) { root.dataset.semioOsReady = beaconId; delete root.dataset.semioOsError; delete root.dataset.semioOsNotFound; }
  return () => { delete root.dataset.semioOsReady; delete root.dataset.semioOsError; delete root.dataset.semioOsNotFound; };
}, [session, error, pluginFilter, shellRoute.kind, hostMode]);
```
The spec waits on it with `page.waitForFunction` keyed by the plugin id, then reads which of `semioOsReady`/`semioOsError` matched (`os-plugins.spec.ts:31-46`).

**⚠️ Critical gap for the demonstrator**: this beacon is written to `document.documentElement` — **one global attribute for the whole document**, keyed only by `pluginFilter`. Storybook specs only ever mount one `FrameworkOsShell` per page, so this is safe there. The demonstrator mounts **up to six `FrameworkOsShell` instances simultaneously** in the same document (`♻️mit-bestand/🧺️demonstrator/📦️index.tsx:410-424`, one per booted pane). Their `pluginFilter` values are distinct (`generator`→`procedural3d`, others→their own variant name unchanged, per `demonstratorPaneBootVariants`/`demonstratorPaneRuntimeVariant`, `♻️mit-bestand/🧺️demonstrator/🟦️brand.ts:778-787`), so the *values* don't collide, but the *slot* does: `document.documentElement.dataset.semioOsReady` can only ever hold **one** id at a time, and only the most recently mounted/re-rendered shell's effect gets to write it. A test that needs "pane A is ready AND pane B is ready at the same time" cannot rely on this beacon as-is. See §4 for the proposed minimal fix (scope the beacon to the shell's own root instead of `document.documentElement`).

---

## 3. Stable selectors already exposed

### In the demonstrator page itself (`♻️mit-bestand/🧺️demonstrator/📦️index.tsx`)
- Each pane's live shell/placeholder/poster sits in a `<div>` keyed by pane id: `onContainerElement`/`registerContainer` (lines 400, 838-840, 884), but **the container div itself carries no `data-testid`/`id`** — only `inert={!focused}` and pointer-capture handlers (lines 401-409). It is *not* independently selectable except by DOM order/child content.
- Each `FrameworkOsShell` is given `shellId={pane.id}` and `storageNamespace={pane.id}` (line 420-421) — see next bullet, this *does* surface a selector, just one level down inside the shell, not on the demonstrator's own wrapper.
- Overview cards: plain `<button>` with icon + `pane.label` + `pane.tagline` text (`DemonstratorCard`, lines 442-487) — no `data-testid`, selectable only via visible text/role (`getByRole("button", { name: /Generator/ })`).
- Touch/mobile list root: `data-demonstrator-list-scroll=""` (line 818).
- Navbar app name: `data-slot="app-name"` (line 788, text "Entwerfen mit Bestand").
- "Übersicht" (back to overview) button: no test id, only icon + visible German text (line 802-811).
- **Nothing** marks pristine/booted/suspended/live pane state for a test to assert on directly (no `data-pane-state`, no `data-booted`); this must be inferred from DOM content (poster `<img>` vs. `CanvasSkeleton` `role="status" aria-busy="true"` placeholder (line 428) vs. live shell markup).

### In the OS shell (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx` and `.../Shell/🟦️component.tsx`)
Present and reusable:
- **`data-shell-id={scope.shellId}`** on the shell's own root `.semio-scope` div (`ShellHost/🟦️component.tsx:1073`). Since the demonstrator passes `shellId={pane.id}`, **`[data-shell-id="generator"]` / `[data-shell-id="koordinator"]` / … / `[data-shell-id="verfolgen"]` are already stable, ready-made per-pane scoping roots** — the single best anchor for every per-pane assertion in the new spec.
- The generic element-alias mechanism (`🧰️framework/🔨️modules/🖱️ui/🧱️elements/🆔️ElementId/🟦️component.tsx:64-86`): `elementIdSelector(id) = '[id="${id}"], [data-element-alias~="${id}"]'`. Window bodies get `data-element-alias={childElementId("framework.window", kind.id)}` (`ShellHost/🟦️component.tsx:6585`), i.e. `windowElementId(kindId)` (`🧰️framework/🔨️modules/🖥️platform/🟦️component.ts:51-53`) → `"framework.window.<camelCasedKindId>"`. Each demonstrator pane's main window id is already named in `🟦️brand.ts`: `procedural-main` (generator, line 499), `cad-play-shape` (koordinator, line 500), `puzzle3d-main` (aggregator, line 100), `sourcing-pool` (aussuchen, line 600), `process-workpiece` (bearbeiten, line 601), `gis2d-main` (verfolgen, line 602) — so e.g. `[data-shell-id="aggregator"] [data-element-alias~="framework.window.puzzle3dMain"]` is a concrete, already-existing selector for "the aggregator pane's main viewport is present".
- Undo/redo/history is a **framework-wide, per-app-guaranteed** panel tab: `FRAMEWORK_PANEL_TAB_HISTORY_ID = "framework.panel.history"` (`🧰️framework/🔨️modules/🛂️manifest/🟦️component.ts:306`; sibling `FRAMEWORK_PANEL_TAB_CATALOGUE_ID` at line 294). Built in `ShellHost/🟦️component.tsx:5512-5588` (`frameworkUtilitiesHistoryTab`): tree items `id: "framework.history.undo"` / `id: "framework.history.redo"` (lines 5533-5534) each wrapping a `<button>Undo</button>`/`<button>Redo</button>` gated by `historyProjection.canUndo`/`canRedo`; `id: "framework.history.checkpoint"` / `"framework.history.checkin"` (lines 5544-5570); command list entries `id: \`framework.history.entry.${entry.seq}\`` (line 5578, one per undo-stack entry — directly useful for "history grows/shrinks on undo/redo"). These render through the same `singleTreeLeaf`/tree-row machinery seen elsewhere stamping `id`/`data-slot="tree-item-row"` on the DOM node (pattern confirmed at `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx:11330`), so the buttons should be reachable via `elementIdSelector("framework.history.undo")` once the History panel tab is open — **needs a one-time manual DOM check against a running dev server to confirm the exact rendered node** before locking a spec to it (not verified at runtime in this read-only pass).
- Command-palette undo/redo (always available, no panel needed): `id: "studio.undo"` / `id: "studio.redo"` (`ShellHost/🟦️component.tsx:6443-6454`) inside the command palette (opened via keybinding, not yet identified in this pass).
- `[role="alert"][data-semio-os-shell-error]` (`ShellHost/🟦️component.tsx:6700`) — fatal shell error banner.
- `[role="status"][aria-live="polite"][data-semio-transient-notice][data-notice-code=...]` (`ShellHost/🟦️component.tsx:7033-7036`) — transient notices.
- `role="alert" data-shell-fault-boundary={boundaryId}` (`Shell/🟦️component.tsx:1158`) — the class-component error boundary's fallback UI.
- Introduction/tutorial overlay (`UIIntroduction`, `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx:5552-5827`): root flag `[data-introduction-active="true"]` (line ~5613 inside the component body); dismiss/"Skip" control is a `WindowChrome` `close` prop with `id: "ui.introduction.skip"` (lines 5763-5768) — i.e. `#ui\\.introduction\\.skip` (or `[id="ui.introduction.skip"]`) is a stable dismiss button; Next/Done is `<Button id="ui.introduction.next">` (line 5822), Back is `<Button id="ui.introduction.back">` (line 5821); `Escape`-style dismissal is also wired as the `"ui.introduction.skip"` control keybinding (line 5717).
- `PaneErrorBoundary` in the demonstrator itself (`📦️index.tsx:353-367`) renders plain text `"{label} konnte nicht geladen werden."` with no test id on crash — only text-matchable.

### What is MISSING for reliable per-pane/per-window assertions
1. **No per-shell-scoped readiness beacon.** `semioOsReady`/`semioOsError` live on `document.documentElement`, a single global slot shared by all six simultaneously-mounted shells (§2). A test cannot currently ask "is `[data-shell-id="koordinator"]` specifically ready?" via the beacon alone — it must wait on DOM content within that shell's container as a substitute (e.g. its main window's `data-element-alias` selector becoming visible, or a canvas appearing), or the beacon needs to move onto the shell's own scope root.
2. **No `data-testid`/`data-pane-state` on the demonstrator's own pane wrapper** (`📦️index.tsx`) distinguishing "not booted" vs. "live" vs. "suspended (poster)" — must be inferred from which of the three JSX branches rendered (skeleton `role="status"`/`img[alt=""]`/live shell markup, lines 410-434).
3. **No stable id on the demonstrator overview cards or the "Übersicht" button** — only visible German text + icon, which is workable (`getByRole("button", { name: "Generator" })`) but brittle to copy edits and is not `data-testid`-based like most of the rest of the repo's UI.
4. **Undo/redo DOM anchors not runtime-verified.** `framework.history.undo`/`redo` ids are declared in the tree-node data model; whether they land on the real DOM node (vs. only being consumed internally by the tree renderer) was not confirmed against a live page in this read-only pass — first implementation step must open the dev server and inspect (§6, step 1).
5. **Two apps' default example fixtures are missing/undefined** per this ticket's own prior research (`.🧬semio/…/DEMONSTRATOR-END-TO-END-ALL-APPS/📓️explore-fixtures.md:52-65`): `aussuchen`'s default example `demo-stock` and `verfolgen`'s default example `reuse-map` were **not found** in the codebase (only `demo-session` exists for aussuchen's plugin). Any acceptance assertion of "expected example/fixture content" for these two panes must first confirm what the demonstrator actually boots today, or the test will correctly start failing until those fixtures are fixed — this is a **pre-existing content gap**, not a testing gap.

---

## 4. Timing constraints and available knobs

Boot pacing (`♻️mit-bestand/🧺️demonstrator/📦️index.tsx`):
- `DEMONSTRATOR_PANE_BOOT_INTERVAL_MS = 35_000` (line 167, hardcoded `const`).
- First pane boots after `1_500`ms (line 194, hardcoded literal, no named constant).
- `useSequentialPaneBoot` (lines 172-200) drives one `scheduleDemonstratorIdle` timer at a time through a queue of the remaining 5 panes at the 35s cadence — full six-pane boot via the overview alone takes **≥ 1.5s + 5×35s = 176.5s**, plus each pane's own WASM/session boot time on top.
- `scheduleDemonstratorIdle` (`🟦️brand.ts:754-764`) layers `requestIdleCallback` on top of the `setTimeout`, so actual wall-clock delay can exceed the nominal figure under load.

Suspension (`DEMONSTRATOR_SUSPENSION_POLICY`, `📦️index.tsx:213-222`, hardcoded `as const`):
```ts
offscreenSuspendDelayMs: 30_000,
overviewIdleSuspendMs: 5 * 60_000,
hiddenTabSuspendMs: 60_000,
sweepIntervalMs: 5_000,
```

**No env vars or props exist to shorten/disable any of these.** Confirmed by exhaustive `grep -n "process.env\|import.meta.env"` over both `📦️index.tsx` and `🟦️brand.ts` — zero matches. Every timing constant above is a literal `const`/object field with no override seam (unlike, e.g., `SEMIO_BUILD_BUDGET_MS`/`SEMIO_CMD_BUDGET_MS`/`SEMIO_TEST_BUDGET_MS` elsewhere in the repo's own conventions, `🧰️framework/…/📦️index.ts:1224-1249`).

**Proposed minimal, non-hacky knob** (consistent with the repo's existing `SEMIO_*_MS` override convention used throughout `📦️index.ts`'s budget functions): thread the same pattern into the demonstrator's own timing constants, e.g.
```ts
const DEMONSTRATOR_PANE_BOOT_INTERVAL_MS = Number(process.env.DEMONSTRATOR_PANE_BOOT_INTERVAL_MS ?? 35_000);
const DEMONSTRATOR_PANE_BOOT_FIRST_DELAY_MS = Number(process.env.DEMONSTRATOR_PANE_BOOT_FIRST_DELAY_MS ?? 1_500);
const DEMONSTRATOR_SUSPENSION_POLICY = {
  offscreenSuspendDelayMs: Number(process.env.DEMONSTRATOR_SUSPEND_OFFSCREEN_MS ?? 30_000),
  overviewIdleSuspendMs: Number(process.env.DEMONSTRATOR_SUSPEND_OVERVIEW_IDLE_MS ?? 5 * 60_000),
  hiddenTabSuspendMs: Number(process.env.DEMONSTRATOR_SUSPEND_HIDDEN_TAB_MS ?? 60_000),
  sweepIntervalMs: Number(process.env.DEMONSTRATOR_SUSPEND_SWEEP_INTERVAL_MS ?? 5_000),
} as const;
```
`import.meta.env.VITE_*` is the idiomatic Vite-side alternative, but plain `process.env` (as used pervasively in `📦️index.ts`) is a closer match to repo convention and works transparently because Vite still exposes `process.env.*` at build/dev time via its Node-context config loader (the demonstrator's own `⚙️vite.config.ts` already runs in that context). This is the cleanest fix, but it is **optional** for the acceptance test itself: see the recommended test strategy below, which avoids needing it for the bulk of the coverage.

**Recommended test strategy given these constraints** (avoids waiting out 35s/300s/60s timers wherever possible):
- The demonstrator already supports **direct deep-linking to a single pane via the URL hash**: `paneIdFromLocationHash()` (`📦️index.tsx:63-67`) is read into `initialFocusId` (line 505), which seeds `bootedIds` with that one pane immediately (`useSequentialPaneBoot(initialFocusId, …)`, line 176: `new Set(initialFocusId ? [initialFocusId] : [])`) **and** sets `skipIdleQueue: touchListMode || focusedId != null` (line 509), so the sequential-boot timer for the *other* five panes never even starts. Navigating straight to `http://127.0.0.1:6029/#generator` (etc.) boots exactly that one pane on first paint, with none of the 35s pacing delay — this is the fast path the per-pane acceptance tests (§6) should use, keeping each test within a `quick` (30s) or even `fundamental` (15s) budget.
- `showIntroduction` is `false` whenever `initialFocusId` is set (line 507: `useState(!initialFocusId)`), so the *overview's* landing-page tour never appears on a hash-deep-linked load — but the **pane's own shell** still runs its normal auto-introduction once focused (`suppressAutoIntroduction={!focused}` is `false` for the focused pane, line 422), so each per-pane spec must still dismiss that app-level tour (via `#ui\\.introduction\\.skip`, §3) before asserting interactivity.
- The 35s cross-pane pacing and the 30s/5min/60s suspension timers are pure client-side setTimeout/requestIdleCallback logic already covered by fast, deterministic **Vitest** unit tests using injected fake schedulers (`scheduleDemonstratorIdle` tests, `🟦️brand.ts:799-827`, injecting a mock `DemonstratorIdleScheduler`) — the acceptance test should **not** re-derive these delays through real wall-clock waits; that would make the suite slow and flaky for something already unit-tested. If a browser-level regression test of the *overview*'s pacing is desired, gate it behind the `long`/`exhaustive` `SEMIO_TEST_LEVEL` (§1) as a single, explicitly slow, opt-in test — do not make it part of the default per-pane matrix.

---

## 5. Where the spec should live, and how the dev server starts for it

CLAUDE.md's taxonomy rules (root `CLAUDE.md`, "You MUST use a domain-driven taxonomy tree…", "You MUST implement all permanent scripts in `📜️script.ts`…", "`project.json` MUST only call `📜️script.ts <command> <subcommand...> <args>`", "All devs are using `launch.json` … You MUST register all executable commands there") point at the demonstrator's **own** directory, not `.storybook/`:

- `.storybook/*.spec.ts` is the harness for **Storybook stories** (single-shell-per-`iframe.html` pages served from the aggregated static build). The demonstrator is a **separate, standalone Vite app** with its own `dev`/`build`/`test` targets and its own port — it is not a Storybook story and is not built into `storybook-static/`. Domain-driven taxonomy says its acceptance test belongs beside its own `📜️script.ts`/`🧪️vitest.config.ts` in `♻️mit-bestand/🧺️demonstrator/`, exactly like `.storybook/playwright.config.ts` sits beside the Storybook stories it exercises.
- Config/spec files are not "scripts" under CLAUDE.md's "no other script files than `📜️script.ts`" rule — `🧪️vitest.config.ts` already lives in this same directory as a sibling config file (`♻️mit-bestand/🧺️demonstrator/🧪️vitest.config.ts`), establishing the precedent that test *configuration* files are fine there; only executable task orchestration must funnel through `📜️script.ts`.
- **`📋️project.json`** (`♻️mit-bestand/🧺️demonstrator/📋️project.json:1-27`) needs **no changes** at all: its existing `"test"` target already runs `bun ./📜️script.ts test` with `forwardAllArgs: true`. Adding a new subcommand branch inside the demonstrator's own `TestScript.run()` (currently `♻️mit-bestand/🧺️demonstrator/📜️script.ts:137-142`, only routing to `runVitest`) is all that's needed for `bun nx run @semio-tech/mit-bestand-demonstrator:test e2e` (or similar) to reach it.
- **Dev server for the test**: reuse the demonstrator's own `DevScript` (`♻️mit-bestand/🧺️demonstrator/📜️script.ts:115-124`), which already builds every pane's plugins (`buildDemonstratorPlugins()`) and then runs `runViteBunxDev(...)` fixed to `MIT_BESTAND_DEMONSTRATOR_PORT` (default `6029`) — spawn it as a daemon (mirroring root's `runStorybookPlaywright`, §1) rather than the Storybook static-file server, since the demonstrator is a live Vite dev server, not a prebuilt static bundle. A free-port probe (same `isTcpPortFree`/port-span-scan idea as `pickStorybookStaticPort`, `📜️script.ts:19069-19080`) avoids colliding with a developer's already-running `6029` instance.
- **`.vscode/launch.json`** registration: today there is **no** entry for the demonstrator's `test` target at all (only `🛠️dev🏚️mitbestand🎪️demonstrator`, line 1126, group `3_dev`, order 213, and `📦️build🏚️mitbestand🎪️demonstrator`, line 5577, group `4_build`, order 133). Add a new entry alongside the existing gate/test entries (pattern: `⚖️gate📖️storybook`, line 4631, group `4_gate`, order 400 → `"command": "bun nx run workspace:test-storybook"`), e.g. `⚖️gate🏚️mitbestand🎪️demonstrator` → `"command": "bun nx run @semio-tech/mit-bestand-demonstrator:test e2e"`, group `4_gate`, next free order near 400s.
- A convenience root `package.json` script (matching `dev:mit-bestand:demonstrator`/`build:mit-bestand:demonstrator`, lines 84/113) is optional but consistent: `"test:mit-bestand:demonstrator:e2e": "bun nx run @semio-tech/mit-bestand-demonstrator:test e2e"`.

---

## 6. Concrete, ordered implementation proposal

1. **Runtime DOM verification pass** (manual, not code): boot the demonstrator (`bun nx run @semio-tech/mit-bestand-demonstrator:dev`, port 6029), open `http://127.0.0.1:6029/#generator` (and each other pane hash), and confirm in devtools:
   - `[data-shell-id="generator"]` exists and contains the pane's main window (`[data-element-alias~="framework.window.proceduralMain"]` — verify the exact camelCase output of `elementIdSegment("procedural-main")`).
   - Whether `framework.history.undo`/`framework.history.redo` land as a real DOM `id` attribute reachable via `elementIdSelector(...)`, and which panel tab (bottom-right utilities, per `frameworkUtilitiesHistoryTab`, `ShellHost/🟦️component.tsx:5512`) must be opened first to reveal them.
   - What `aussuchen` and `verfolgen` actually render today given the `demo-stock`/`reuse-map` fixture gap (§3, point 5) — decide real vs. placeholder-tolerant assertions for those two panes specifically.
   This pass produces the exact selectors the spec files in step 3 will hard-code; skipping it risks writing assertions against `id`s that only exist in the tree-node data model, not the rendered DOM.

2. **(Optional but recommended) Add env-overridable timing knobs** in `♻️mit-bestand/🧺️demonstrator/📦️index.tsx` per §4's proposed snippet (`DEMONSTRATOR_PANE_BOOT_INTERVAL_MS`, `DEMONSTRATOR_PANE_BOOT_FIRST_DELAY_MS`, and the four `DEMONSTRATOR_SUSPENSION_POLICY` fields), all defaulting to today's literals so production behavior is unchanged. Only needed if step 4's "full sequential boot" and "suspension" tests (last bullet below) are implemented against real timers instead of being left to the existing Vitest coverage in `🟦️brand.ts:799-827`.

3. **Author the spec + config** in `♻️mit-bestand/🧺️demonstrator/`:
   - `🧪️playwright.config.ts` — mirrors `.storybook/playwright.config.ts` structure (import `playwrightTestTimeoutMs` from `@semio-tech/repo-lib`; `testDir` = this directory; `testMatch: ["*.acceptance.spec.ts"]` or similar to avoid colliding with future non-e2e specs; `baseURL` from `PLAYWRIGHT_BASE_URL`/`MIT_BESTAND_DEMONSTRATOR_PORT`; same Chromium `swiftshader`/`webgpu` launch args, since the demonstrator hosts the same WebGL/WGPU-backed hosts as the Storybook specs).
   - `🧪️demonstrator.acceptance.spec.ts` (or one file per pane if preferred — six panes is small enough for one file with a `for (const pane of DEMONSTRATOR_PANES)` loop, importing `DEMONSTRATOR_PANES` directly from `🟦️brand.ts` so the pane list never drifts from the app itself). Per-pane assertions, using the `#<paneId>` deep-link fast path (§4):
     - **Boots**: `page.goto("/#" + pane.id")`; wait for `[data-shell-id="${pane.id}"]` to attach (`page.waitForSelector`); assert zero `pageerror`s and zero `significantConsoleErrors` (reuse the exact helper from `framework-hosts-wasm.spec.ts:15-18`).
     - **Renders expected example/fixture content**: dismiss the pane's own intro tour if present (`page.locator('[id="ui.introduction.skip"]')`, click if visible within a short timeout, tolerate absence); then assert the pane's main window container is visible (`[data-shell-id="${pane.id}"] [data-element-alias~="${windowAlias}"]`) and, where the fixture is confirmed real (generator/koordinator/aggregator per the fixtures research, §3 point 5), assert pane-specific fixture markers (e.g. aggregator: object/node counts via the same debug pattern `puzzle-3d-5d-infinite.spec.ts` uses elsewhere, `getByTestId(...)`, if the demonstrator's own puzzle window exposes an equivalent debug element — verify in step 1) or, at minimum, a non-empty canvas/viewport (`expect(canvas).toBeVisible()`, `boundingBox()` non-zero, mirroring `.storybook/puzzle-3d-5d-infinite.spec.ts:82-90`'s canvas-visibility check). For `aussuchen`/`verfolgen`, assert against whatever step 1 found actually renders (flag as a known content gap in the test's own comment, referencing this ticket, rather than asserting against the missing `demo-stock`/`reuse-map` examples).
     - **Window is interactive**: perform one real pointer interaction inside the pane's main window (e.g. a click or wheel-zoom at its center, mirroring `puzzle-3d-5d-infinite.spec.ts:82-90`'s pointer-count pattern or `🟦️brand.ts`'s own recorded demonstration gestures, e.g. `{ kind: "scroll", at: { kind: "windowNormalized", ... }, deltaY: -100 }` for aggregator, lines 297-320) and assert an observable, app-specific effect (viewport transform change, selection state change, or — simplest and most uniform across all six apps — that the pointer/keyboard capture handler fired by asserting the pane's `onDirty`/history side effect below actually ran, since any interaction marks the pane permanently non-suspendable, `📦️index.tsx:407-408`, `markDirty`).
     - **Undo/redo + history work**: open the History panel tab (`FRAMEWORK_PANEL_TAB_HISTORY_ID`, needs its own tab-selector confirmed in step 1, likely similar to `os-plugins.spec.ts`/`🟦️brand.ts`'s pattern of clicking `FRAMEWORK_PANEL_TAB_CATALOGUE_ID`'s equivalent chrome tab); perform one mutating interaction (add/move/edit something in the app, app-specific — reuse each pane's own recorded `demonstrations` gesture from `🟦️brand.ts` where available, e.g. aggregator's `add-object`/`fill-tool` steps, lines 412-483); assert a new `framework.history.entry.<seq>` row appears; click Undo (`elementIdSelector("framework.history.undo")`) and assert the entry list/document content reverts; click Redo (`elementIdSelector("framework.history.redo")`) and assert it re-applies. Assert `canUndo`/`canRedo`-driven `disabled` state transitions on the buttons as a secondary, cheap check.

4. **Wire the task**:
   - Extend `♻️mit-bestand/🧺️demonstrator/📜️script.ts`'s `TestScript.run()` (currently lines 137-142) with a new branch, e.g. `if (rest[0] === "e2e") { await this.runAcceptancePlaywright(); return; }`, implementing `runAcceptancePlaywright()` as the demonstrator-local analog of root `📜️script.ts`'s `runStorybookPlaywright()` (§1/§5): pick a free port, `buildDemonstratorPlugins()` + spawn `DevScript`'s own vite-dev command as a daemon (not the static-build path — the demonstrator has no static-serve mode today), poll until the page responds, run `bunx playwright test --config 🧪️playwright.config.ts` with `PLAYWRIGHT_BASE_URL`/`MIT_BESTAND_DEMONSTRATOR_PORT` set to the picked port, kill the daemon in `finally`.
   - No `📋️project.json` changes needed (§5) — `forwardAllArgs: true` already threads `e2e` through.
   - Add the `.vscode/launch.json` entry (§5) and, optionally, a root `package.json` convenience script.
   - (Cleanup-quality, not required for correctness) Consider extracting the private `waitForUrl`/`isTcpPortFree`/`pickStorybookStaticPort` trio out of root `📜️script.ts`'s `TestScript` class (currently unexported, lines 19042-19080) into `@semio-tech/repo-lib` so the demonstrator's own script doesn't have to re-implement the same ~35 lines — CLAUDE.md's "if code is repeated, it MUST be close to each other" plus "aim for clean long term solution, MUST NOT be pragmatic" both point at sharing this rather than duplicating it a second time.

5. **(Optional, explicitly slow) Overview-level pacing/suspension test**: a single additional test, gated to run only at `SEMIO_TEST_LEVEL=long` or `exhaustive` (§1/§4), that loads `/` with no hash, asserts panes boot in declared order at roughly the 35s cadence (tolerant windowed assertions, not exact-millisecond), and — only if step 2's env knobs were added — re-runs the same check with drastically shortened suspension delays to verify a pane's live shell is actually released (canvas/WASM torn down, poster `<img>` shown) and revived on refocus. Keep this out of the default/`quick` suite entirely; the existing `scheduleDemonstratorIdle` Vitest coverage (`🟦️brand.ts:799-827`) already exercises the scheduling logic deterministically and cheaply.

---

## Summary of file:line citations used

- `.storybook/playwright.config.ts:1-51`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts:1115-1210` (test-level budgets, `playwrightTestTimeoutMs`)
- `package.json:84,113,125`
- `📋️project.json:421` (root); `♻️mit-bestand/🧺️demonstrator/📋️project.json:1-27`
- `📜️script.ts:18962-18967,19042-19107,19140-19168,19747-19751` (root `TestScript`/`DevScript`)
- `.storybook/os-plugins.spec.ts:15-46` (readiness beacon consumption)
- `.storybook/framework-hosts-wasm.spec.ts:15-38` (`expectHostStory`)
- `.storybook/puzzle-3d-5d-infinite.spec.ts:5,11-35,82-90,101-106` (testMatch caveat note; `expectStoryLoads`; interaction + fixture-status patterns)
- `♻️mit-bestand/🧺️demonstrator/📦️index.tsx:63-67,166-222,370-437,490-509`
- `♻️mit-bestand/🧺️demonstrator/🟦️brand.ts:100,499-500,600-602,744-797,799-834`
- `♻️mit-bestand/🧺️demonstrator/📜️script.ts:98-145`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx:1043-1080,5512-5588,6443-6454,6585,6700,6891-6916,7033-7036`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Shell/🟦️component.tsx:1158`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🆔️ElementId/🟦️component.tsx:19-94`
- `🧰️framework/🔨️modules/🖥️platform/🟦️component.ts:51-62`
- `🧰️framework/🔨️modules/🛂️manifest/🟦️component.ts:294,306`
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx:5552-5827,11330`
- `.vscode/launch.json:1126,4631-4640,5577-5585`
- Prior ticket research: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/📓️explore-fixtures.md:27-65` (fixture gaps for `aussuchen`/`verfolgen`)
