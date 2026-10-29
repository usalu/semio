# 1028 Audit: What `test-e2e` Of `@semio-tech/mit-bestand-demonstrator` Exercises

Author: `e2eaudit` (read-only, Haiku). No builds, no edits outside this report.
Demonstrator root: `♻️mit-bestand/🧺️demonstrator`. Suite: `🧪️tests/🎭️acceptance/🟦️.ts` (1041 lines, one Playwright file).

## 0. Corrections To The Brief

- Pane ids are `energie` and `statik`, not `energy` and `fem3d`. Brand variants are `energy` and `fem3d`. Test titles are `energie boot`, `energie run-cancel`, `statik boot`, `statik undo-redo`.
- `test-e2e` does NOT use port 6029. `serve-e2e` runs `serve-test`, which binds `127.0.0.1` on port `0` (ephemeral) and publishes the URL through the service session. Port 6029 belongs to the dev `serve` target only.
- `test-e2e` depends on `serve-e2e`, which depends on `prepare-e2e`, which depends on `activate-prototype` and `workspace:deps-browsers`. Running `repo run …:test-e2e` alone is enough, because the chain is pulled in.

## 1. Command Chain

Source: `📋️project.json` (targets `prepare-e2e` line ~330, `serve-e2e` ~359, `test-e2e` ~389), `🔨️modules/🧩️runtime/📜️script.ts`, `🔨️modules/🧪️e2e/📜️script.ts`.

| Step | Target | Command (cwd `♻️mit-bestand/🧺️demonstrator`) | Notes |
|---|---|---|---|
| 1 | `activate-prototype` | `repo ./🔨️modules/🧩️runtime/📜️script.ts activate`, env `SEMIO_BUILD_MODE=prototype` | Bazel/Nx cached, depends on `activate-lanes-prototype` (energy, fem3d, generator react prototypes). |
| 2 | `workspace:deps-browsers` | `repo "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🏗️native/📜️script.ts" sync browsers` | Side-effecting, installs Playwright browsers into the repo tool cache. |
| 3 | `prepare-e2e` | `repo ./🔨️modules/🧩️runtime/📜️script.ts prepare-test` | `openServiceSession(demonstratorE2eSessionRoot)`, needs `SEMIO_BUILD_INVOCATION_PID`. Session root `dist/services/e2e`. |
| 4 | `serve-e2e` | `repo ./🔨️modules/🧩️runtime/📜️script.ts serve-test` | Continuous. `serveVite` host `127.0.0.1`, port `0`, publishes `baseURL` via `publishServiceReady`. Session closed in `finally`. |
| 5 | `test-e2e` | `repo ./🔨️modules/🧪️e2e/📜️script.ts test` | Waits for service ready (readiness budget below), then runs `playwright test --config 🔨️modules/🧪️e2e/🎚️config/🟦️.ts --output <dir>`. |

Readiness budget (`🔨️modules/🧩️runtime/🧪️e2e/🔣️.json`): `readyTimeoutMs` 600000, `progressIntervalMs` 15000. Estimate only, not measured.

Env vars read or set:
- `SEMIO_BUILD_INVOCATION_PID` (required, must be a positive integer, otherwise "must run through the build tool").
- `SEMIO_BUILD_MODE` (serve-test uses `readBuildMode`).
- `SEMIO_TICKET_DIR` (if set, output goes to `$SEMIO_TICKET_DIR/🗑️generated/e/<uuid-hex>`, otherwise `dist/reports/e/<uuid-hex>`).
- `SEMIO_LANGUAGE` / progress language via `serviceReadyProgressLanguage`.
- `PLAYWRIGHT_BASE_URL` (set by `TestScript` to the published service URL; config throws if missing).
- `PLAYWRIGHT_BROWSERS_PATH` (set by `TestScript` to `repoCacheDirectory(repoRoot,"tools","ms-playwright")`).
- `CI` (true: `retries` 2, `forbidOnly`).
- `DEMONSTRATOR_E2E_GPU=swiftshader` (software GPU override, see §3).
- `DEVCONTAINER`, `MIT_BESTAND_DEMONSTRATOR_PORT` affect only `serve` (6029), not `serve-test`.

Playwright config (`🎚️config/🟦️.ts`): `testDir` `🧪️tests`, `testMatch` `🎭️acceptance/🟦️.ts`, `fullyParallel` false, `workers` 1, `retries` 0 locally, reporter `list` (stdout only), single project `chromium` (`devices["Desktop Chrome"]`), `trace: on-first-retry`, `expect.timeout = min(playwrightTestTimeoutMs(), 120000)`.

Browser requirements: Chromium from `PLAYWRIGHT_BROWSERS_PATH`. The config does not pin a channel. Desktop Chrome viewport.

## 2. Results And Artifacts

- Playwright `--output` = `<ticket>/🗑️generated/e/<32-hex uuid>` (or `dist/reports/e/<hex>`). Policy in `📁️artifacts/🔣️.json`: `maxPathChars` 256, `reservedSuffixChars` 102, `maxTitleChars` 24, `maxCases` 32, namespace `e`. A ticket path longer than about 150 chars throws `Demonstrator E2E artifact root exceeds 154 characters` before Playwright starts.
- Per test: `p.png` (pane screenshot via `capturePaneArtifact`, also attached as `<pane>-p`), `trace.zip` and `error-context.md` (on failure, trace only on retry).
- Stale `🎭️…/test-results` folder sits at demonstrator root (Sep 16). Not used when `--output` is set.
- Exit: `test` throws `Demonstrator acceptance tests failed (<status>)` on non-zero unless aborted by signal.

## 3. GPU / Browser Flags

From `browserLaunchArgs()`:
- Default on darwin: `--use-angle=metal --enable-gpu --ignore-gpu-blocklist --enable-unsafe-webgpu`.
- Default on linux/windows: `--use-angle=gl --enable-gpu --ignore-gpu-blocklist --enable-unsafe-webgpu`.
- `DEMONSTRATOR_E2E_GPU=swiftshader`: `--use-angle=swiftshader --enable-unsafe-swiftshader --enable-unsafe-webgpu`.
- The verfolgen map (`TiledMapHost`, wasm/wgpu) requests zero tiles under swiftshader. That case cannot be graded there. Devcontainers without GPU must set swiftshader and accept that verfolgen map and the verfolgen undo-redo map check will fail.

## 4. Pane Cases (acceptance `PANE_CASES`, plus boot test per pane)

Boot test title = `${paneId} boot`. Each test: `goto /#<pane>`, expect `[data-shell-id=<pane>]` count 1 (120 s), wait for `data-shell-ready` / `data-shell-error` / `data-shell-not-found` on that shell root (120 s), dismiss intro (`[id="ui.introduction.skip"]`, escape fallback), then per window check. Then screenshot.

Surface check helpers and the attributes they wait for:
- world3d: `.semio-world-3d-host` (or `.semio-world-3d-empty`) with `data-meshes-json`, `data-instances-json` (non-empty JSON array length > 0 when content expected).
- nodeGraph: `.semio-node-graph-host` with `data-host-snapshot-json` (`widgets` length > 0).
- table: `.semio-table-host` with `[data-row-id]` rows (> 0).
- tree: `[role="tree"]` body, `[role="treeitem"]` rows (> 0).
- tiledMap: `.semio-tiled-map-host canvas`, screenshot centre 60%, distinct RGBA > 1.
- placeholder: text match `/Keine Auswahl|No selection/`.

| paneId | variant | exampleId (brand line) | fixture location (verified exists) | windows (kindId → surface, content expected) | interactions in its boot test |
|---|---|---|---|---|---|
| generator | generator | `hexagonal-mushroom-column` (523) | procedural `📚️examples/🍄️hexagonal-mushroom-column` | `procedural-main` nodeGraph yes; `procedural-preview` world3d yes | none beyond boot |
| koordinator | koordinator | `demo` (574) | cad `📚️examples/🎬️demo` | `cad-play-shape`, `cad-play-building`, `cad-play-energy`, `cad-play-structure-classic` all world3d yes | none |
| aggregator | aggregator | `concrete-forest` (363) | puzzle 3d `📚️examples/🌲️concrete-forest` (contains `seed-left-001` in `🖼️assets/🌲️forest/🗣️.dsl.semio`) | `puzzle3d-main` with instances `puzzle3d-main-top`, `puzzle3d-main-perspective` (2 alias matches) world3d yes | none |
| energie | energy | `bestest-600` (793) | energy `📚️examples/🏛️bestest-600` (exports `id = "bestest-600"`) | `energy.model.3d` world3d yes; `framework.window.tree` (structure) tree ≥1 row; `framework.window.table` (zones) table ≥1 row; `energy.simulation` tree ≥1 row | none |
| aussuchen | aussuchen | `demo` (629) | sourcing `📚️examples/🎬️demo` (contains `beam-glulam-gl24h`) | `sourcing-pool` table yes; `sourcing-curated` table no; `sourcing-preview` placeholder no-selection text; `sourcing-grid` world3d no | none in boot; interactions in `aussuchen preview` and `aussuchen undo-redo` |
| bearbeiten | bearbeiten | `concrete-forest` (674) | process3d `📚️examples/🌲️concrete-forest` (contains `wall-saw-cut`) | `process-workpiece` world3d yes | none |
| verfolgen | verfolgen | `demo` (725) | gis `📚️examples/🎬️demo` | `gis2d-main` tiledMap yes (paints > 1 colour) | none |
| statik | fem3d | `concrete-forest` (844) | fem `📚️examples/🌲️concrete-forest` | `fem3d-model` world3d yes; `fem3d-results` world3d yes | none |

Example ids: all eight resolve to an example folder in the owning plugin. Verified by `ls`-equivalent `rg --files` on `✏️s/🔌️plugins/*/📚️examples/`. The brief's "fixture/example id that does not resolve" check finds none for brand defaults. Note `statik`'s brand default is `concrete-forest`, but the spec's own note says the served pane looked like it booted `demo`. That is a brand/default question, not a failure cause.

Interactive tests (each is its own Playwright test):

| Test title | Pane | Fixture (`🧬️…/🧫️cases.json`) | Interaction | Asserts |
|---|---|---|---|---|
| `pane roster` | all | none | none | `PANE_CASES` pane ids equal `DEMONSTRATOR_PANES` ids parsed as text from `🪧️brand.ts` |
| `overview chrome` | `/` | none | dismiss intro | 8 `[data-layered-card]`, silhouettes, glass classes, title chip svg, 2 intro paragraphs, `Demonstrator öffnen` chip, no drag handles |
| `aussuchen preview` | aussuchen | `🧬️curation/🧫️cases.json` `beam-glulam-gl24h` / "Glulam GL24h 200×400" | click stock row | placeholder → world3d with meshes/instances > 0, instance id list equals `[beam-glulam-gl24h]` |
| `aussuchen undo-redo` | aussuchen | curation steps (increment, undo, redo, keyboardIncrement, undo, redo, remove, undo), availability 24, revisit `bearbeiten` after 31000 ms off-screen | stepper `[data-stepper-control="increment"]`, ArrowUp, remove button `Remove|Entfernen`, Ctrl/Cmd+Z and Ctrl/Cmd+Shift+Z, layered overview button, card open chip | pool spinbutton value, curated row count, grid instance count per step, scene snapshot stable per count |
| `energie run-cancel` | energie | `🧬️simulation/🧫️cases.json` energy | open tool tab `tool.energySimulation` (category `framework.category.tool`), Ctrl/Cmd+Enter, toolRunStart, toolRunAbort | progressbar visible, status `aria-live`, `Finalized|Abgeschlossen ·`, final step regex, accepted-result text; restart gives new run group; abort gives `Aborted|Abgebrochen`, no-result text |
| `statik undo-redo` | statik | `🧬️simulation/🧫️cases.json` fem: caption `Case: dead`, phase 0.1, scale 300 → 600 → 300 → 600 | open results tab `fem3d.panel.results`, phase slider Home/PageUp, scale fill+Tab, Ctrl/Cmd+Z/Shift+Z | transport paused (Play/Abspielen), projection changes per step, undo/redo restores solved projection |
| `generator undo-redo` | generator | `🧬️edits/🧫️cases.json` generator: height `6` → `6.5`, 7 widgets, 6 synapses | artifact tree label click on `height`, inspection `procedural-play-inspector.value.input` fill, Tab, canvas click + Ctrl/Cmd+Z/Shift+Z | widget ids, synapse ids, height value, preview geometry undo/redo |
| `koordinator undo-redo` | koordinator | edits: catalogue `cad-play-catalogue.spatial.shape.primitive.box`, windows `cad-play-shape`, `cad-play-building`, `cad-play-energy`, `cad-play-structure-classic`, row prefix `cad-play-document.shape` | catalogue tab, box label click, artifact tab, canvas click + undo/redo | +1 instance in window 0 only, other windows unchanged, undo/redo restore |
| `aggregator undo-redo` | aggregator | edits: kind `Hexagonal Cut Concrete Forest Left`, seed `seed-left-001`, vortexCount 11, mesh `hexagonal-cut-concrete-forest-left.glb`, windows `puzzle3d-main-top`, `puzzle3d-main-perspective` | catalogue `[id="<kindId>"] [data-slot="tree-label"]` click, artifact tab, canvas undo/redo | new id matches `^puzzle3d\.object\.[0-9a-f]+$`, mesh/vortex counts, undo/redo per window |
| `bearbeiten undo-redo` | bearbeiten | edits: step `wall-saw-cut` label "Wall Saw Slab Cut", machine `wallSaw`, capability `wallCut`, 7 step ids | artifact row click, inspection fields, button `Enabled|Aktiviert` toggle, undo/redo | inspection fields text, projection changes and restores |
| `verfolgen undo-redo` | verfolgen | edits: label "Acceptance Reuse Marker", delta +1, lon/lat 0 | engagement toggle, addFeature action label + execute, undo/redo | `positions` count +1 then back, routes/regions unchanged |
| `funding logos` | `/` intro | none | click `Weiter` twice (`[data-slot="introduction-info-box"] button`), title `Förderhinweis` | at least 3 visible logos with width, height and naturalWidth > 0 |

Overall test count: 20 test titles (matches `fixture.titles` in `📁️artifacts/🧪️tests/🟦️.ts` and `🧪️demonstratorcompileclosure` artifact cases).

## 5. Timeouts

- `SHELL_READY_TIMEOUT_MS` 120000 (pane shell outcome, element visibility, world/table/tiled host visibility).
- `TEST_TIMEOUT_MS` 240000 (`test.setTimeout` on boot and most tests).
- `SURFACE_CONTENT_TIMEOUT_MS` 60000 (per content count poll, polls every 500 ms).
- Undo/redo tests: `TEST_TIMEOUT_MS + 3 * 60000` (about 420 s). Energie run-cancel: `240000 + 4 * 60000`. Statik: `240000 + 3 * 60000`. Aussuchen undo-redo: `240000 + 120000 + 31000`.
- Funding logos: 60000 flat.
- `expect` default: `min(playwrightTestTimeoutMs(), 120000)`. `playwrightTestTimeoutMs` (repo-lib) not read.
- Introduction dismiss: 3 s visibility probe, 20 s click, 15 s for skip count to reach 0.
- Known slow spots noted by the spec: cold WASM boot (generous 120 s), verfolgen tile paint, fem solve (not waited on at boot).

## 6. Console And Page Error Policy

- Fixture `browserHealth` (auto) records `pageerror` (any) and `console` messages of type `error`. After the test body it runs `expect.soft` on both: page errors must be `[]`, and `significantConsoleErrors(consoleErrors)` must be `[]`.
- `significantConsoleErrors` only drops the exact message `Failed to load resource: the server responded with a status of 404` with an optional ` (Not Found)` suffix (case-insensitive, anchored). A 404 with any extra suffix, and every other status (400-409, 500), counts as significant.
- Covered by unit tests in `🧬️console/🧪️.ts` (Vitest, not Playwright). These are NOT run by `test-e2e`.
- Consequence: a single unrelated console error in any pane fails that test even if all its assertions pass.

## 7. Flags For Triage

1. Port 6029 is not used by `test-e2e`. A failure that says "cannot reach localhost:6029" points at the wrong target.
2. `test-e2e` requires `SEMIO_BUILD_INVOCATION_PID`. Running `playwright test` by hand, or running the `📜️script.ts test` file directly, fails before any browser starts.
3. Browsers come from `PLAYWRIGHT_BROWSERS_PATH` (repo tool cache), filled by `workspace:deps-browsers`. A missing browser shows up as an early Playwright launch error, not a test failure.
4. GPU: on a host without a real adapter, verfolgen map cases (`verfolgen boot`, `verfolgen undo-redo`) fail under the default `gl`/`metal` flags only if the adapter is missing. Under `swiftshader` they fail by design (map paints zero tiles).
5. `koordinator undo-redo` depends on `data-row-id` for a runtime-derived catalogue id `cad-play-catalogue.spatial.shape.primitive.box`. Not verified statically. A failure there is most likely a catalogue id drift, not a fixture miss.
6. `aggregator undo-redo` selects the catalogue row by raw kind label `[id="Hexagonal Cut Concrete Forest Left"]` without the `elementIdSegment` folding used elsewhere in the same file. If the DOM id is folded, this selector never matches. Check this first when the aggregator undo test fails at the catalogue click.
7. `statik` brand default `concrete-forest` vs the spec's note that the served pane looked like `demo`. Non-blocking, but if `fem3d` assertions show 3 meshes / 47 instances, the served example is `demo`, not `concrete-forest`.
8. `demonstratorcompileclosure/🎭️e2e` is a separate artifact/compile-closure suite. It is not part of `test-e2e` and does not run Playwright.
9. Output path risk: `demonstratorE2eOutputDirectory` throws when the ticket path is too long. Keep `SEMIO_TICKET_DIR` short, or the run fails before the browser starts.
10. Readiness: first `test-e2e` cold boot may take 1 to 3 minutes (estimate). The 600 s ready timeout is generous, but the first pane boot test still allows only 120 s for shell readiness after `goto`.
11. The stale `test-results` folder in the demonstrator root (Sep 16) is not the current output. Do not read it as the result of this run.

## 8. Files Read

- `♻️mit-bestand/🧺️demonstrator/🔨️modules/🧪️e2e/🟦️.ts`, `📜️script.ts`, `🎚️config/🟦️.ts`, `📁️artifacts/🟦️.ts`, `🔣️.json`, `🧫️cases.json`, `🧪️tests/🟦️.ts`
- `♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/🟦️.ts`, `📜️script.ts`, `🧪️e2e/🟦️.ts`, `🧪️e2e/🔣️.json`
- `♻️mit-bestand/🧺️demonstrator/🧪️tests/🎭️acceptance/🟦️.ts`, `🧬️console/🟦️.ts`, `🧬️console/🧪️.ts`, `🧬️simulation/🟦️.ts`, `🧬️simulation/🧫️cases.json`, `🧬️curation/🧫️cases.json`, `🧬️edits/🧫️cases.json` and their `🔣️.json` schemas
- `♻️mit-bestand/🧺️demonstrator/🪧️brand.ts` (pane list line 912+, `defaults.exampleId` lines 165, 363, 523, 574, 629, 674, 725, 793, 844)
- `♻️mit-bestand/🧺️demonstrator/📋️project.json` (serve, prepare-e2e, serve-e2e, test-e2e, deps-browsers)
- `✏️s/🔌️plugins/*/📚️examples/` for example existence
