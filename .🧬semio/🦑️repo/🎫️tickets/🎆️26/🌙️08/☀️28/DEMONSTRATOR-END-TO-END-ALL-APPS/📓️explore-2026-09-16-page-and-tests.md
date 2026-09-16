# 📓️ Explore — demonstrator page mechanics, tests, and vite builder (2026-09-16)

Read-only pass. Paths below use `D = ♻️mit-bestand/🧺️demonstrator`. Verified against the current module
layout (post-restructure); ticket history in `📓️status.md`/`📓️plan-acceptance-test.md`/
`📓️acceptance-spec-audit.md` is older and some findings there are now stale — noted inline where relevant.

## 1. How the demonstrator page boots

**Single React root, six embedded `FrameworkOsShell`s, no URL-based playground routing.** Entry
`D/🌐️.html` loads `D/🟦️.tsx` as a module (`<script type="module" src="./🟦️.tsx">`, `🌐️.html:18`);
`🟦️.tsx:941` calls `mountUiRoot(document.getElementById("root")!, <DemonstratorLanding />)`.

- **Locale/appearance lock-in before first paint**: `bootstrapElementsSurfaceChromeDocument` and
  `initUiLocaleSync(DEMONSTRATOR_LOCALE)` run at module scope (`🟦️.tsx:34-40`) — the whole demonstrator is
  German-locked (`DEMONSTRATOR_LOCALE = "de"`, `🪧️brand.ts:23`) so chrome text never flashes English.
- **Pane list**: `DEMONSTRATOR_PANES` (`🪧️brand.ts:781-788`) — 6 entries `{id, variant, brand, label,
  tagline, icon}`: generator, koordinator, aggregator, aussuchen, bearbeiten, verfolgen. Each carries its
  own `ShellBrand` (e.g. `ENTWERFEN_MIT_BESTAND_AGGREGATOR_BRAND`, `🪧️brand.ts:343-500`).
- **Hash routing** (`🟦️.tsx:60-64` `paneIdFromLocationHash`): `#<paneId>` on load sets `initialFocusId`,
  which both skips the overview grid and fast-boots straight to that one pane (used by the acceptance
  suite — see §3). A `hashchange` listener (`🟦️.tsx:614-621`) calls `focusPane`/`returnToOverview`; `Escape`
  also returns to overview (`🟦️.tsx:623-630`). `focusPane` (`🟦️.tsx:580-596`) does
  `window.history.replaceState(null, "", "#"+id)`.
- **Sequential pane boot** (`useSequentialPaneBoot`, `🟦️.tsx:191-221`): only the hash-focused pane (if any)
  starts booted; the rest queue and boot one at a time, first after 1.5s then every
  `DEMONSTRATOR_PANE_BOOT_INTERVAL_MS = 35_000` (`🟦️.tsx:189`) via `scheduleDemonstratorIdle` (requestIdleCallback
  wrapper, `🪧️brand.ts:751-764`) — deliberately staggered so 6 simultaneous WASM plugin boots don't jank the
  first paint. Hover/focus can `promote` a not-yet-booted pane to the front of the queue
  (`promoteAndResume`, `🟦️.tsx:466-472`). `skipIdleQueue` is true in touch-list mode or once something is
  already focused (`🟦️.tsx:459`).
- **Suspension** (`usePaneSuspension`, `🟦️.tsx:263-340`): a booted-but-unfocused pane that's been idle past
  a policy threshold (`DEMONSTRATOR_SUSPENSION_POLICY`, `🟦️.tsx:184`: 30s offscreen / 5min overview-idle /
  60s hidden-tab) has its live shell torn down and replaced by a captured canvas poster
  (`capturePanePoster`, `🟦️.tsx:230-258`), reviving instantly on hover/focus. A pane the user has actually
  touched (`onPointerDownCapture`/`onKeyDownCapture` on `DemonstratorPane`, `🟦️.tsx:414-416`) is permanently
  exempt — the framework has no snapshot/restore for an interacted pane's live document across
  suspend/resume yet (doc comment `🟦️.tsx:174-183`).
- **Mounting**: `DemonstratorPane` (`🟦️.tsx:377-435`) renders `<FrameworkOsShell pluginFilter=... plugins=...
  surfaceSessionFactories={PUZZLE_BOARD_SESSION_FACTORIES} appId=... locks=... defaults=... brand=...
  shellId={pane.id} storageNamespace={pane.id} suppressAutoIntroduction={!focused} />` wrapped in a
  per-pane `PaneErrorBoundary` (`🟦️.tsx:349-368`, one crashing pane never takes down the other five).
  `runtimeBoot`/`manifestBoot` come from `resolvePlaygroundBoot(PLUGIN_CATALOG, bootVariants.runtime|manifest)`
  where `bootVariants = demonstratorPaneBootVariants(pane.variant)` (`🪧️brand.ts:777-780`) separates the
  module-owning runtime variant from the branded manifest row — only `generator` differs (`runtime:
  "generation3d"`, `manifest: "generator"`; every other pane's runtime === manifest).
- **"Ready" in the DOM**: a **per-shell** readiness beacon, not just the old global one. `ShellHost`
  (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:10556-10606`,
  region `🔖️ReadinessBeacon`) sets `data-shell-ready` / `data-shell-error` / `data-shell-not-found` on the
  shell's own scope root (`scope.rootRef.current`, which carries `[data-shell-id="<pane.id>"]`) in addition
  to the pre-existing single global `document.documentElement` `data-semio-os-ready/-error/-not-found`
  slot — the global slot can't distinguish six simultaneously-mounted shells, which is exactly why the
  demonstrator needed the per-shell one. Before boot/while queued, the pane shows a placeholder
  (`CanvasSkeleton` + a dimmed brand logo, `role="status" aria-busy="true"`, `🟦️.tsx:430-434`); once
  suspended it shows the captured `<img>` poster instead (`🟦️.tsx:428`).
- **Navbar/footer**: `Navbar` renders only in the un-focused overview (`!hoveredPaneId`, `🟦️.tsx:801-820`) —
  logo (`ShellBrandLogo`) + "Entwerfen mit Bestand" title, `data-slot="app-name"` (`🟦️.tsx:790`),
  `showFullscreenToggle={false}`. The overview footer shows two partner-credit `NavbarItem`s built by
  `D/⚛️footer.tsx`: `aProjectOfLuhUdkFooterItem` (LUH + UdK logos, left, `⚛️footer.tsx:135-165`) and
  `fundedByZukunftBauFooterItem` (Zukunft Bau/BBSR, right, `⚛️footer.tsx:~90-125`) — both placed directly in
  `overviewChrome` (`🟦️.tsx:783-786`), not through the framework's own footer mechanism. **Once a pane is
  focused**, the navbar/footer disappear entirely and each `FrameworkOsShell` renders its own brand's
  in-app navbar (`ShellHost`'s own `Navbar`) — the demonstrator's outer chrome is overview-only.
- **Introduction**: `ENTWERFEN_MIT_BESTAND_GENERAL_INTRODUCTION` (`🪧️brand.ts:43-125`) is the
  page-level intro shown via `UIIntroduction` while `showIntroduction` is true (`🟦️.tsx:788-795`) —
  `showIntroduction` starts true only when there's no hash deep-link (`!initialFocusId`, `🟦️.tsx:378`) and
  is dismissed on first pane focus (`focusPane` sets it false, `🟦️.tsx:583`) or its own `onDismiss`.
  Separately, **each brand replays its own app-level introduction on load once focused**
  (`replayIntroductionOnLoad: true` per brand, e.g. `🪧️brand.ts:351`, gated by
  `shouldReplayIntroductionOnLoad`/`suppressAutoIntroduction={!focused}`) — the acceptance suite dismisses
  that per-pane intro via its `ui.introduction.skip` element before asserting content
  (`dismissIntroductionIfPresent`, acceptance `🟦️.ts:114-118`).
- **Examples (`setActiveExample`)**: not a demonstrator-owned concept — it's `ShellHost`'s generic
  boot-default mechanism. Each brand declares `defaults: { exampleId: "…" }` (e.g. aggregator →
  `"concrete-forest"`, `🪧️brand.ts:350`). `resolveShellDefaults(brand, undefined)` in `DemonstratorPane`
  (`🟦️.tsx:391`) feeds that into `FrameworkOsShell`'s `defaults` prop. On first session mount,
  `ShellHost` resolves the boot example via `resolveBootExampleId(activeExampleId, exampleOptions,
  defaults.exampleId)` (`🐚️Shell/🟦️.tsx:287-294`: keep a still-valid current id → else the brand's default →
  else the first registered example) and dispatches it once per session instance
  (`ShellHost/🟦️.tsx:9888-9904`). The in-app navbar's example `<select>` (only rendered when
  `exampleOptions.length > 0 && !locks.exampleId`, `ShellHost/🟦️.tsx:9975`) lets the user change it
  afterwards via `dispatchActiveExample`→`buildActiveExampleAction` (`ShellHost/🟦️.tsx:9051-9076`). No
  `🎬️demo` symbol exists in the current demonstrator code — the closest analog is each plugin's own
  `📚️examples/🎬️demo*` fixture directory (e.g. puzzle's `🎬️demo-session`, referenced only as a fixture
  asset path, not a runtime API this page calls).
- **Grid vs. touch-list layout**: desktop/tablet uses a `DEMONSTRATOR_GRID_COLUMNS×ROWS = 3×2` CSS grid,
  `100vw × 100vh` per cell, panned via a single rAF loop that's either free-pan "follow" (mouse position,
  overview) or a 500ms eased "glide" (focus/hover pin) — never both at once (`ScrollDrive`, `🟦️.tsx:81-89`,
  drive loop `🟦️.tsx:718-767`). Touch-first viewports (`DEMONSTRATOR_TOUCH_LIST_MEDIA_QUERY = mobile
  media query AND (hover: none) AND (pointer: coarse)`, `🟦️.tsx:42-43`) instead render a vertical
  snap-scroll list (`data-demonstrator-list-scroll`, `🟦️.tsx:820`), one `100dvh` section per pane.

### Contrast with `os/dev` playground pages
- `os/dev` (`🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🏗️builder/🌐️vite/🟦️.ts`) is **one URL → one
  `SEMIO_PLUGIN`/`plugin` env-selected variant**, resolved server-side into a single session module
  (`playgroundSessionViteAlias`) and a single `FrameworkOsShell` mounted at `/🟦️.ts` (its own entry file,
  not `🟦️.tsx`). The demonstrator instead mounts **six** `FrameworkOsShell` instances in one page, each
  with its own `pluginFilter`/`shellId`/`storageNamespace` and its own ephemeral `ShellBrand` — there is no
  server-side variant selection; the client statically knows all six panes via `DEMONSTRATOR_PANES`.
- `os/dev` brands persist local storage per its own real host (`resolveShellBrandById`); every demonstrator
  brand is `ephemeral: true` (in-memory storage per pane, `🟦️.tsx:35-37` docstring) since the panes share
  one page/origin and must not leak state into each other via `localStorage`.
- `os/dev`'s HTML entry is root-absolute (`/🟦️.ts`) specifically because Vite's `transformIndexHtml` also
  serves SPA-fallback deep links like `/spaces/{id}` (`🏗️builder/🌐️vite/🟦️.ts:167-171` comment); the
  demonstrator has no nested path routes (routing is hash-only against one fixed `/`), so its relative
  `./🟦️.tsx` entry (`🏗️builder/🌐️vite/🟦️.ts:80`, matching the static `🌐️.html:18`) is safe.
- `os/dev` supports a live-reload "studio" host mode (`isHostPlaygroundFilter`, catalog-wide smoke) and a
  brand-driven single production `outDir`; the demonstrator always serves/builds its fixed union of 6
  panes and has no host/studio mode.

## 2. Tests

Four Vitest/Playwright suites plus the Nx task graph that runs them.

### Unit / in-source tests (Vitest, Node environment)
Config: `D/🧪️tests/🎚️config/🟦️.ts` — `environment: "node"`, `include: []` (no standalone spec files picked
up), `includeSource: ["./📜️script.ts", "./🪧️brand.ts"]` (Vitest's `import.meta.vitest` in-source pattern),
`passWithNoTests: false`. Two source files carry `if (import.meta.vitest) { … }` blocks that pull in the
actual assertions from small `registerTests1` helper modules:
- `D/📜️script.ts:18-21` → `D/🧪️tests/🧪️demonstratorruntimebuildvariants/🟦️.ts` (34 lines):
  - `demonstratorRuntimeBuildVariants("generator")` returns `["generation3d"]` — one extra runtime variant
    beyond the 6 panes' own (generator's `generation3d` module).
  - Validates `D/🔨️modules/🧩️runtime/🔣️.json` (the pane catalog) and `D/🔨️modules/🧩️runtime/🧫️pipeline.json`
    against `D/🔨️modules/🧩️runtime/🧬️schema/🔣️.json` via `ajv` — both a positive parse and negative-case
    rejections (`schemaVersion: 1` empty-panes doc must fail; `pipeline` with unknown `schemaVersion` must
    fail).
- `D/🪧️brand.ts:791-793` → `D/🧪️tests/🧪️scheduledemonstratoridle/🟦️.ts` (44 lines):
  - `scheduleDemonstratorIdle` never fires before its minimum delay (fake `setTimeout`/`requestIdleCallback`
    scheduler — 0 idle calls / 0 callback calls until the delayed timer fires, then exactly 1/1).
  - `demonstratorPaneBootVariants("generator")` → `{runtime: "generation3d", manifest: "generator"}`;
    `demonstratorPaneBootVariants("koordinator")` → `{runtime: "koordinator", manifest: "koordinator"}`
    (drift guard on the runtime/manifest split).

  **Run**: `bun ./📜️script.ts test` (from `D`), or `bun nx run @semio-tech/mit-bestand-demonstrator:test`.
  Router: `D/📜️script.ts:7-16` — `resolveTestLevel(segments)` reads `SEMIO_TEST_LEVEL`/a leading level
  arg (`fundamental|quick|long|exhaustive`, default `fundamental`, budgets 15s/30s/300s/900s —
  `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts:1083-1088,1097-1099`), then
  `runVitest(root, rest, "./🧪️tests/🎚️config/🟦️.ts")`. No server needed — pure Node/Vitest, no browser.

### Acceptance E2E (Playwright, `D/🧪️tests/🎭️acceptance/🟦️.ts`, 345 lines)
Full breakdown in §3. Config: `D/🔨️modules/🧪️e2e/🎚️config/🟦️.ts` — `testMatch: ["🎭️acceptance/🟦️.ts"]`,
`fullyParallel: false`, `workers: 1`, `retries: CI ? 2 : 0`, per-test `timeout:
playwrightTestTimeoutMs()` (reads `SEMIO_TEST_LEVEL`, same budget table as above — i.e. **15s by default**
unless something upstream raised the level) and `expect.timeout: min(that, 120_000)`. The suite itself
overrides per-test: `SHELL_READY_TIMEOUT_MS = 120_000`, `TEST_TIMEOUT_MS = 180_000` via
`test.setTimeout(TEST_TIMEOUT_MS)` (acceptance `🟦️.ts:46-47,293`) — generous on purpose for cold WASM plugin
boots, so this config-level 15s floor is moot in practice as long as the suite's own `setTimeout` call
runs first. `use.baseURL` **must** come from `PLAYWRIGHT_BASE_URL` — the config throws immediately if
unset (`🎚️config/🟦️.ts:23`), i.e. it refuses to guess a default `:6029` and never starts its own server.
Browser project `chromium` launches with `["--use-angle=swiftshader", "--enable-unsafe-swiftshader",
"--enable-unsafe-webgpu"]` (`🎚️config/🟦️.ts:45-47`) — **software rendering is forced explicitly** (not
just headless Chromium's usual SwiftShader default); this differs from some ad-hoc ticket probe scripts
(e.g. `DEV-CAD-REACT-E2E/🐍️console-dump-probe.mjs`) that instead pass `--use-angle=metal` for real-GPU
perf — worth remembering if a future perf-sensitive probe is added here.

**Server lifecycle — isolated, NOT the fixed `:6029` dev server**:
`test-e2e` (project.json:163-175) → `serve-e2e` (continuous, `serve-test`) ← `prepare-e2e` (`prepare-test`)
← `activate-dev` + `workspace:deps-browsers`. `ServeTestScript` (`D/🔨️modules/🧩️runtime/📜️script.ts:67-79`)
calls `serveVite({ …, port: 0, session, ready: url => publishServiceReady(...) })` — **port 0** (OS-assigned
ephemeral port), registered as a named service session (`DEMONSTRATOR_E2E_OWNER =
"@semio-tech/mit-bestand-demonstrator:e2e"`, `D/🔨️modules/🧩️runtime/🧪️e2e/🟦️.ts:3`) under
`dist/services/e2e`, gated on `NX_INVOCATION_ROOT_PID` (must run through Nx —
`demonstratorE2eInvocationPid`, `…/🧪️e2e/🟦️.ts:11-14`). `D/🔨️modules/🧪️e2e/📜️script.ts:9-30` (the
`test-e2e` consumer) reads that session, `waitForServiceReady`, then spawns Playwright's CLI directly with
`PLAYWRIGHT_BASE_URL` set to the discovered URL. So a plain `:6029` dev server (`nx run …:dev`/`:serve`)
is a **separate, unrelated instance** — the acceptance suite never touches it.

**Exact commands**:
- (a) unit tests: `cd D && bun ./📜️script.ts test` (or a level: `bun ./📜️script.ts test long`).
- (b) acceptance against an *already-running* `:6029` (manual, bypassing Nx's session machinery — the
  config's own `throw` means you must supply the var yourself):
  `cd D && PLAYWRIGHT_BASE_URL=http://127.0.0.1:6029 bun ../../node_modules/playwright/cli.js test --config ./🔨️modules/🧪️e2e/🎚️config/🟦️.ts`
  (PLAYWRIGHT_BROWSERS_PATH should point at the repo's cached browsers, as the real script does —
  `repoCacheDirectory(repoRoot, "tools", "ms-playwright")`, `…/🧪️e2e/📜️script.ts:21` — otherwise Playwright
  may try to use/install a different browser copy.)
- (c) full target (own isolated server, the way CI/Nx actually runs it):
  `bun nx run @semio-tech/mit-bestand-demonstrator:test-e2e` — chains `prepare-dev` (framework-os-dev
  prepare-*-react-dev ×7) → `activate-dev` → `deps-browsers` → `prepare-e2e` → `serve-e2e` (port 0) →
  Playwright against the discovered URL.

**Memory-relevant notes**: (1) headless Chromium's SwiftShader default is made explicit/forced here via
launch args, not left implicit — no `--use-angle=metal` override exists in the shipped config, so this
suite is always software-rendered. (2) reproducing a recurrence across reloads would need an in-page
console hook (per the "Browser Console Buffer Survives Reload" memory) — this suite doesn't reload within a
test, it does one `page.goto` per pane per test, so that concern doesn't currently apply here but would for
any future test that navigates the SPA more than once.

### `test`, `test:e2e` npm scripts
`D/package.json:19-23` just forward to the Nx targets above (`bun nx run …:test` / `…:test-e2e`).

## 3. Acceptance suite coverage per pane

`D/🧪️tests/🎭️acceptance/🟦️.ts` is table-driven off `PANE_CASES` (`🟦️.ts:146-193`), one Playwright `test()`
per pane (`🟦️.ts:291-344`) plus a drift guard (`🟦️.ts:286-288`: `PANE_CASES` pane-id list must equal
`DEMONSTRATOR_PANES`' ids, parsed out of `🪧️brand.ts` as **text**, not imported — importing the brand
module would drag in `@semio-tech/ui-react`'s JSON import that Node's ESM loader rejects, `🟦️.ts:31-35`).

Per-test flow: `page.goto("/#<paneId>")` (deep-link fast boot) → wait for `[data-shell-id="<paneId>"]` to
exist → `waitForPaneShellOutcome` polls `dataset.shellReady/shellError/shellNotFound` and asserts `"ready"`
→ dismiss the per-pane intro if shown → for each declared window, resolve its `[id]`/`data-element-alias`
element (`paneElementSelector`, mirrors `framework/ui/elements/🆔️ElementId` + `windowElementId`) and assert
it's visible, then read a **surface-specific, already-production DOM attribute** (no test-only
instrumentation):

| surface | selector | content signal |
|---|---|---|
| `world3d` | `.semio-world-3d-host` / `.semio-world-3d-empty` | `data-meshes-json` + `data-instances-json` array lengths (`World3dHost/🟦️.tsx` ~5063) |
| `nodeGraph` | `.semio-node-graph-host` / `-empty` | `data-fixture-json`'s `widgets[]` length (`NodeGraph/🟦️.tsx:1154`) |
| `table` | `.semio-table-host` / `-empty` | count of `[data-row-id]` `<tr>`s (`framework/ui/elements/📊️Table/🟦️.tsx:201/262`) |
| `tiledMap` | `.semio-tiled-map-host` / `-empty` | canvas pixel-diff sample (no DOM count attribute exists for this surface — `tiledMapHasVisibleContent`, `🟦️.ts:253-282`) |

**`PANE_CASES` table** (`🟦️.ts:146-193`):
- **generator**: `procedural-main` (nodeGraph, expectContent) + `procedural-preview` (world3d,
  **expectContent: true but documented KNOWN GAP** — edit-mode `render()` uses an always-fresh
  `FlowEvalSession`, so `eval_json`/mesh/instance JSON is always empty; matches `app-generator.md`).
- **koordinator**: 4 windows, `cad-play-{shape,building,energy,structure-classic}`, all world3d,
  `expectContent: true`. Per the header comment these were the same known-gap story until commit
  `f394df99d4` wired `cad_pane_working_scene` through `ArtifactChild::local_owner` — **the comment in this
  file says that fix was "not yet compile-verified" at spec-authoring time**, but `DEV-CAD-REACT-E2E`'s
  2026-09-16 note (closed) confirms it now renders real inline tessellation in all four panes with live
  interaction (see §4) — this suite's assertions should now pass for koordinator.
- **aggregator**: `puzzle3d-main` with `instanceIds: [puzzle3d-main-top, puzzle3d-main-perspective]`
  (split top/perspective views sharing one kind id via `data-element-alias`), world3d, expectContent.
- **aussuchen**: `sourcing-pool` (table, expectContent) / `sourcing-curated` (table, **expectContent:
  false by design** — nothing curated yet) / `sourcing-preview` (world3d, **expectContent: false, KNOWN
  GAP** — app always calls `preview::render(snapshot, &[], labels)`, permanently "No selection") /
  `sourcing-grid` (world3d, expectContent).
- **bearbeiten**: `process-workpiece` (world3d, expectContent).
- **verfolgen**: `gis2d-main` (tiledMap, expectContent — canvas must paint more than one flat color; relies
  on the dev server's same-origin tile proxy so the canvas isn't cross-origin-tainted).

Every test also fails on any `pageerror` or console `error` not matching a `40x` resource-load 404
(`significantConsoleErrors`, `🟦️.ts:49-51,342-343`) — so a plugin boot warning that logs a real JS error
fails the suite even if the DOM assertions above would have passed.

### Extensions / sub-modules coverage
**None of the flow-extension-\*, process-extension-\*, or sourcing sub-modules get any dedicated
assertion.** The suite only asserts pane-level window content; it never inspects which extensions loaded,
which catalogue/workshop entries an extension contributed, or extension-specific behavior (e.g.
bearbeiten's Workshop panel listing `process-extension-{concrete,metal,robotic,wood}` machines, verified
manually in `DEV-PROCESS-REACT-E2E` but not encoded here as a Playwright assertion; likewise
generator/koordinator's `flow-extension-*` nodes). An extension crashing at load would still be caught
indirectly (console error / pane never reaching `data-shell-ready`), but a *silently* broken/missing
extension (e.g. one that loads but contributes nothing) would not fail this suite.

### Gaps — panes/extensions with NO automated end-to-end assertion
1. **Interactivity is entirely unasserted**: no click/drag/select/undo is exercised anywhere in this
   suite — only "does the pane boot and does its default window show non-empty content". Real interaction
   coverage today lives only in ad-hoc, ticket-scoped probe scripts under
   `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️15/DEV-CAD-REACT-E2E/🐍️*.mjs` and similar one-off `.mjs`/`.py`
   probes per ticket — none of these are wired into `test-e2e` or run automatically.
2. **All 9 flow-extensions** (bim, brep, dictionary, draw, list, logic, math, primitive, text) and **all 4
   process-extensions** (concrete, metal, robotic, wood) — present in `D/dist/extensions/` and loaded at
   runtime, but zero assertions reference them by name or content.
3. **Sourcing module** (aussuchen's curation/filter/sort commands, `setFilterQuery` etc.) — confirmed wired
   at the dispatch layer per `app-aussuchen.md` but has no UI chrome and thus nothing for this suite (or
   any E2E suite) to click; not covered here.
4. **Generator's koordinator-adjacent Generate mode** vs edit-mode split is not distinguished — the suite
   only ever hits the pane's default boot state (edit-mode preview, the known-empty one).
5. **Aggregator's catalogue-empty-for-concrete-forest gap** (`app-aggregator.md` §8.1: `meta.kind-catalogs`
   unpopulated) is not asserted — the suite only checks that `puzzle3d-main` has non-empty
   meshes/instances, which is already true from the placed forest objects; a dev dragging from an empty
   catalogue would not fail this suite.
6. **Koordinator's object mutations are documented no-ops** (`DEV-CAD-REACT-E2E` §"Open", child
   re-materialization gap) — not asserted; the suite only checks initial boot content, never a
   drag/mutate/verify round-trip.
7. **Verfolgen's feature click-select/hover-popup break** (`app-verfolgen.md` §5: dead action ids after the
   26/08/14 interaction-domain migration) — not asserted; the suite only checks the map canvas paints.

## 4. Known open issues per pane (recent tickets, 09-13…09-16)

- **koordinator (CAD)** — `DEV-CAD-REACT-E2E` (`.../🎆️26/🌙️09/☀️15/DEV-CAD-REACT-E2E`, **status: closed**,
  latest note `📓️cad-react-e2e-2026-09-16.md`): all four panes now render real inline BREP tessellation +
  the Concrete Forest reference PNG, click/hover/gumball/inspection/Artifact-tree all wired to a real `cad`
  interaction domain. **Still open** (explicitly listed, not closed by this ticket): (1) object mutations
  (`addObject`/`patchObject`/`translateSelection`/etc.) are documented no-ops — geometry lives in composed
  `s.stdio.semio.model` children and the re-materialization seam after a child edit doesn't exist yet, so
  the gumball snaps back after drag; (2) tree paging has no `setPanelPage` command; (3) the "N selected"
  engagement HUD reads 0 (no request context); (4) 21 native test failures are framework-harness debt
  (registry-less `testkit::new_app`, store-harness contracts), not CAD runtime bugs.
- **bearbeiten (process3d)** — `DEV-PROCESS-REACT-E2E` (`.../🎆️26/🌙️09/☀️15/DEV-PROCESS-REACT-E2E`,
  **status: closed**): plugin + all 4 extensions load, Workshop panel lists 11 machines +
  concrete/metal/robotic catalogs, `addWorkshopMachine`/`setStepEnabled` verified to actually re-replay the
  workpiece mesh. Unit suite went from all-red (since 09-08) to 294/331 green; 37 still-red are pre-existing
  framework/harness debt (`BuiltChildren requires retained page transport`, wasm-bridge history-insertion
  laws, `export_brep_out` step-export, several dispatch-effect laws) — outside this ticket's scope.
- **verfolgen (gis2d)** — `GIS-2D-END-TO-END-BUILD` (`.../🎆️26/🌙️09/☀️16/GIS-2D-END-TO-END-BUILD`, no
  `ticket.json` found, open): blocked purely on this machine's **unaccepted Xcode license** (`cc` link exit
  69) — `cargo check` passes, native `cargo test`/link doesn't. `📓️acceptance-spec-audit.md`-era app note
  (`app-verfolgen.md`) separately documents a real app bug: feature click-select/hover-popup dispatch the
  legacy `worldPick`/named-verb ids that were deleted in favor of the generic `features`
  interaction-domain (`interactionSelect`/`interactionHover`) during 26/08/14/FIRST-CLASS-HOVER-AND-
  SELECTION-MECHANISM — `TiledMapHost.tsx` was never updated to dispatch the new ids, so click-select and
  hover-popup silently fail today; `setActiveExample` also only distinguishes empty-vs-non-empty (no real
  example catalogue).
- **generator (procedural3d)** — no dedicated 09-13..16 end-to-end ticket; `app-generator.md` (dated
  earlier, still current per file content) documents the edit-mode Preview gap already encoded as a KNOWN
  GAP in the acceptance suite (§3): `render()` builds a fresh, never-ticked `FlowEvalSession` each call, so
  `eval_json` stays empty. Adjacent 09-14/09-15 tickets `PROCEDURAL-3D-FLOW-WINDOW-ARTIFACT-TREE` (moved the
  graph-outline tree from the Flow window into the Artifact panel) and
  `PROCEDURAL-3D-HISTORY-CAMERA-SPURIOUS` (fixed spurious "camera camera" history rows on widget drags) are
  both narrow UI-polish fixes, not scoped to the render-session gap.
- **aggregator (puzzle3d)** — no single end-to-end ticket, but a dense run of 09-13..09-15 polish tickets,
  all diagnosis/fix notes (status not re-verified here, but all read as applied fixes):
  `PUZZLE-3D-WINDOW-ADD-OBJECT-BUTTON`, `PUZZLE-3D-TRANSFORM-GUMBALL-PREVIEW`,
  `PUZZLE-3D-CATALOGUE-DROP-LIVE-PREVIEW` (drag ghost wasn't rendering during drag), `PUZZLE-3D-FILL-ORBIT-
  CAMERA-RESET`, `PUZZLE-3D-HIDE-FIT-LANE-FRAME-OVERLAY` (matches the uncommitted `World3dHost`/
  `engine-contract` diff seen in git status — retires the manual "frame visible instances" overlay button
  since `WorldAutoFit` now reframes continuously for scene-graph-only fit lanes), `PUZZLE3D-MARQUEE-RECT-
  OVERLAY` (rubber-band rectangle wasn't painted despite selection working), `PUZZLE3D-REFERENCE-3D-
  VISIBILITY` (reference image planes missing under wgpu), `PUZZLE3D-OBJECT-TREE-LABELS` (tree showed kind
  ids instead of authored labels). Root-cause gaps still open per `app-aggregator.md` §8: catalogue empty
  for `concrete-forest` (kinds not populated in `meta.kind-catalogs`), inspection panel shows no per-entity
  fields (needs framework selection→`render()` threading, the same cross-cutting gap as generator/
  aussuchen), and the tutorial's `tracks.document` is an empty skeleton pending a recorder pass.
- **aussuchen (sourcing)** — no 09-13..16 ticket; `app-aussuchen.md` (content matches current code, already
  reflected in the acceptance suite's `sourcing-preview: expectContent:false` case) documents: Preview
  window permanently "No selection" (app-level call site hardcodes `preview::render(snapshot, &[], labels)`
  with an empty slice — the same framework selection-threading gap as generator/aggregator); Pool/Curated
  filter/sort commands are fully implemented and dispatch-correct but have **zero UI chrome** — no search
  box, no filter checkboxes, no sortable headers, no drag source/drop target — `sourcing_action()` and
  `SOURCING_DRAG_MIME` are defined but never called anywhere in the crate.
- **Cross-cutting root cause** repeated across generator/aggregator/aussuchen/koordinator: `ArtifactApp::
  render()`/`context_menu()` are called without an `InteractionView`, so any window whose content depends
  on "what's currently selected" is stuck showing its empty/placeholder state regardless of what the user
  clicks. This is the single highest-leverage framework fix mentioned across every `app-*.md` note.

`git log` for `D` (`git log --date=iso --format='%h %ad %s' -n 5 -- ♻️mit-bestand/🧺️demonstrator`):
```
3250e6cb90 2026-09-15 20:01:24 +0200 🐙️ueli🎆️26🌙️06☀️04🚩️627
5b6f77afcf 2026-09-13 11:27:07 +0200 🐙️ueli🎆️26🌙️06☀️04🚩️617
b2064cc237 2026-09-13 03:15:38 +0200 🐙️ueli🎆️26🌙️06☀️04🚩️616
8add1df147 2026-09-12 21:17:59 +0200 🐙️ueli🎆️26🌙️06☀️04🚩️613
521b618cee 2026-09-12 10:48:12 +0200 🐙️ueli🎆️26🌙️06☀️04🚩️612
```
(commit subjects are opaque batch-flag tags, not descriptive — see each date's session in `📓️status.md`/
sibling ticket notes for content.) `📓️status-2026-09-16.md` (today's fresh reopen) notes the ticket was
reopened manually on disk because the repo MCP failed to connect, and that on this machine neither the
`🎪️demonstrator` plugin module nor the `generator` dev activation exist on disk yet — i.e. today's session
starts from a cold/unbuilt state, consistent with `📓️status.md` ending its narrative at 2026-09-06.

## 5. Vite builder config — routes, engines, and the os/dev inconsistency

`D/🏗️builder/🌐️vite/🟦️.ts` (108 lines). Key structure:
- `resolvedPlaygroundAssets = DEMONSTRATOR_RUNTIME_TARGETS.flatMap(t => t.assets)` (`🟦️.ts:33`) — union of
  asset needs for exactly the demonstrator's 6 panes (mirrors `os/dev`'s own `resolvedPlaygroundAssets`
  pattern but scoped down from its host "serve everything" fallback).
- `{ pluginModuleDirNames, extensionModuleDirNames } = demonstratorRuntimeModuleLayout([...new
  Set(DEMONSTRATOR_RUNTIME_TARGETS.map(t => t.pluginId))])` (`🟦️.ts:35`) — calls into
  `D/🔨️modules/🧩️runtime/🟦️.ts:41-48`, which runs `runtimeComponentClosure(components, rootPluginIds)` (the
  shared dependency-closure algorithm, `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/
  🧩️runtime/🟨️.mjs`) to compute the **transitive** set of plugin+extension directories the six panes need.
- **Serve-mode module routing** (`command !== "build"`, `🟟️.ts:91-95`):
  ```
  ...pluginModuleDirNames.flatMap(name => staticDirVitePlugin(..., route: `${MODULE_PLUGIN_ROUTE}/${name}`, ...))
  ...staticDirVitePlugin(..., route: `${MODULE_PLUGIN_ROUTE}/🪞️vendor`, ...)   // fonts vendor shim
  ...extensionModuleDirNames.flatMap(name => staticDirVitePlugin(..., route: `${MODULE_EXTENSION_ROUTE}/${name}`, ...))
  ```
  i.e. **both** `/🔌️plugin-modules/*` and `/🧩️extension-modules/*` are served as **per-name static dirs
  scoped to the computed closure**, not the whole `pluginModulesDir`/`installedExtensionsDir`.
- **Build mode** (`command === "build"`, `🟦️.ts:91`) uses `browserArtifactVitePlugin(
  demonstratorRuntimeAssetSources(repoRoot, "release"))` — `D/🔨️modules/🧩️runtime/📦️assets/🟦️.ts:15-28`
  again derives its file list from `demonstratorRuntimeComponentIds()`, which is the **same**
  `runtimeComponentClosure` call, so build output is bounded by the identical closure computation as dev
  serving (not "the whole dist plugin-modules dir").
- **Asset/engine routes** (`playgroundAssetVitePlugins`, imported from the shared
  `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts`, `D`'s config line 6/97) — `/mesh`,
  `/cad-assets`-style spec routes, `/osm`/`/vt` tile-proxy routes via `createTileProxyMiddleware`
  (`resolveGisMapTileServeMode(process.env.GIS_MAP_TILE_SERVE_MODE)`, defaults to `"fetch"`), and
  `/infinite-assets` (`…/styling/🏗️builder/🌐️vite/🟦️.ts:1514`) — this helper (and the gis tile-serve-mode
  resolution) is **byte-identical code shared with `os/dev`**, so there is no divergence in how these asset
  routes or the gis surface-wasm engine are served between the two builders.

### Finding: extension-route scoping diverges from `os/dev`, and reproduces a bug `status.md` says was already fixed once
`os/dev`'s own config (`🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🏗️builder/🌐️vite/🟦️.ts:185-188`)
serves `MODULE_PLUGIN_ROUTE` the same way (per-name, closure-scoped `pluginModuleDirNames.flatMap(...)`
plus a vendor entry) — that part matches. But for extensions it does:
```ts
staticDirVitePlugin(repoRoot, { kind: "static-dir", route: MODULE_EXTENSION_ROUTE, root: path.relative(repoRoot, installedExtensionsDir) }),
```
— **one whole-directory mount covering every installed extension**, not scoped to any per-plugin closure.

`📓️status.md:2518-2526` ("Demonstrator module routing — third and final route bug", 2026-09-06) documents
this exact class of bug being hit and fixed once already: *"the demonstrator served only the computed
transitive closure of module directories, while the generated runtime session
(`🤖️generated/🟦️session.ts`) lists **every** installed extension's `moduleUrl`. Everything outside the
closure fell through to the SPA fallback… `os/dev`'s own config serves `MODULE_EXTENSION_ROUTE` whole…
Matched that: whole-dir entries for both routes."` The **current** `D/🏗️builder/🌐️vite/🟦️.ts:94` instead
has `extensionModuleDirNames.flatMap(name => staticDirVitePlugin(..., route:
\`${MODULE_EXTENSION_ROUTE}/${name}\`, ...))` — back to **per-name, closure-scoped** entries, i.e. the
whole-dir fix described as applied on 2026-09-06 is not present in the file on disk today. If any
installed extension is referenced by the generated `PLAYGROUND_SESSION` for one of the six panes but isn't
reachable through `demonstratorRuntimeModuleLayout`'s static dependency-closure walk (e.g. an extension
wired only through runtime registration rather than a declared `depends-on` edge — see memory *Registry
dependsOn Is Declared Runtime Metadata*), its `/🧩️extension-modules/<name>/🔣️.json` request will 404 to
Vite's SPA fallback and return HTML instead of the descriptor, exactly reproducing the 2026-09-06 symptom
(`plugin.descriptor-invalid: … returned HTML`). This should be spot-checked against a live `:6029` serve
(fetch each of the 13 extension descriptor URLs under `dist/plugin-modules`'s `flow-extension-*`/
`process-extension-*` list) before trusting that the closure currently happens to cover all of them.

### Other, non-bug differences vs `os/dev` (context, not inconsistencies)
- `D`'s config has no `semioDescriptorRouteGuardVitePlugin`, `semioSourceFreshnessVitePlugins`,
  `semioPlaygroundReactRefreshCoherenceVitePlugin`, or `semioBrandHtmlVitePlugins` — all `os/dev`-only,
  tied to its single-brand-per-URL / live host-reload model, which the demonstrator's fixed 6-pane page
  doesn't need.
- `D`'s HTML entry uses a relative `./🟦️.tsx` (safe here, no nested routes) vs `os/dev`'s root-absolute
  `/🟦️.ts` (required there for SPA-fallback deep links) — see §1.
