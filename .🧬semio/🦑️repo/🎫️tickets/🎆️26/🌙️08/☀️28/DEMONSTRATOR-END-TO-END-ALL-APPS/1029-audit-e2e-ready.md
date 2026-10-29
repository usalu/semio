# 🧪️ 1029 Audit: E2E readiness after compile is green (auditor e2eready)

Status: NOT READY. Two blockers stand between today's tree and a green `test-e2e`: the compile frontier (`plugin-instance-dispatch`, per brief) and the browser install (`workspace:deps-browsers` failed rc=1 on 10/28 and was never re-run). Port 6029 is free. The output-path guard passes with 1 char of headroom only if `SEMIO_TICKET_DIR` is the ticket folder; leaving it unset is the cleaner route.

Read-only audit. No source edited, no activate/prepare/serve/test-e2e/dev/cargo run, no sub-agents. Measurements taken 10/29 02:00 CET.

## 1. Target chain and exact commands

Dependency graph (from `♻️mit-bestand/🧺️demonstrator/📋️project.json`):

- `test-e2e` dependsOn `serve-e2e` (continuous, `127.0.0.1:0`, ephemeral) dependsOn `prepare-e2e` dependsOn `activate-prototype` + `workspace:deps-browsers`.
- `activate-prototype` dependsOn `prepare-prototype` (9 `framework-os-dev:prepare-*-react-prototype` + 4 wasm targets) + `activate-lanes-prototype` (energy, fem3d, generator).
- `serve` (port 6029) dependsOn `activate-prototype` only. It is NOT a prerequisite of `test-e2e`: `test-e2e` starts its own server on an ephemeral port. A `serve` on 6029 is a manual probe target only.

Commands, cwd `/Users/ueli/Documents/semio`, always through `repo`, same tool flag 1028 used (`--tool=bazel`), `SEMIO_LANGUAGE=en` as in `1028-tools/act.sh`:

1. Activation (the coordinator's `act03`): `nohup "<C>/1028-tools/act.sh" act03 >/dev/null 2>&1 &` which runs `repo --tool=bazel run @semio-tech/mit-bestand-demonstrator:activate-prototype`. The target sets `SEMIO_BUILD_MODE=prototype` itself; do not export it.
2. Browsers, once: `repo --tool=bazel run workspace:deps-browsers` (fails today, see section 3). Playwright is run with `PLAYWRIGHT_BROWSERS_PATH` set by the e2e script to the repo cache `tools/ms-playwright`; do not export it.
3. Optional probe server: `repo --tool=bazel run @semio-tech/mit-bestand-demonstrator:serve`. Target env sets `MIT_BESTAND_DEMONSTRATOR_PORT=6029`, `SEMIO_BUILD_MODE=prototype`. Continuous: start detached and record the pid. `DEVCONTAINER=true` switches the host to `0.0.0.0`.
4. The official run: `[SEMIO_TICKET_DIR=<dir>] [DEMONSTRATOR_E2E_GPU=swiftshader] repo --tool=bazel run @semio-tech/mit-bestand-demonstrator:test-e2e`. This pulls steps 1, 2 and the e2e serve itself. `SEMIO_BUILD_INVOCATION_PID` is supplied by the build tool; running `📜️script.ts` directly throws "must run through the build tool".

Env var meanings: `SEMIO_TICKET_DIR` moves the Playwright output (section 3). `DEMONSTRATOR_E2E_GPU=swiftshader` is only for GPU-less hosts such as a devcontainer, and then the verfolgen map case cannot pass by design. Nothing else needs setting.

The Playwright invocation the e2e script builds: `playwright test --config <demonstrator>/🔨️modules/🧪️e2e/🎚️config/🟦️.ts --output <out>`, cwd = repo root, `PLAYWRIGHT_BASE_URL` = the ephemeral server's URL. Config: `testMatch` only `🎭️acceptance/🟦️.ts`, `workers: 1`, `retries: 0` locally, one project `chromium`, `timeout` 240 s per test, readiness 600 s (`🧩️runtime/🧪️e2e/🔣️.json`).

## 2. Port 6029

Free now. `lsof -nP -iTCP:6029 -sTCP:LISTEN` returned no lines (rc 1) at 02:00. `test-e2e` never uses 6029, so the two cannot collide.

## 3. Playwright browsers and output path

Browser install state:

- `1028-tools/…/e2eprep-br.rc` = 1, log `C/🗑️generated/1028/e2eprep-br.txt` (10/28 23:32). The run never got to Playwright. It failed in build-tool storage planning (`nx:storage-planning`) with `error: Exact original descendant creator identity required` at `🧰️framework/…/native-build/💾️storage/🟦️.ts:1155-1162`. Chromium is NOT installed by this route.
- Likely cause: `.cache/b/d/fixed.dmg` was rewritten in place at 10/29 01:21:47 (mtime). Its inode 422334443 still matches the receipt `.cache/b/d/image.json`, so the failing check is the creator-claim sidecar, not the inode. Partition for `tools/ms-playwright` is probably `.cache/b/t` (memory: `t` = tools). Not confirmed; the repo MCP is disconnected so `repoCacheDirectory` could not be resolved.
- `~/Library/Caches/ms-playwright` contains only `chromium-1234`, `chromium_headless_shell-1234`, `ffmpeg-1011`. Placeholder revisions, not the one Playwright 1.64.1 wants, and the repo path does not use this directory. Do not rely on it.
- Playwright 1.64.1 is installed at the root: `node_modules/playwright/cli.js` and `node_modules/@playwright/test` (via `.bun`). Demonstrator's own `node_modules` symlink dangles (target `.cache/b/d/m/j/a27cea38c77f97611644/w/308/node_modules` does not exist), so the demonstrator package does not resolve its own deps. Repair as part of the activation pass, not by hand.

Output path (measured with node, JS `.length` = the guard's unit):

- Guard: `demonstratorE2eOutputDirectory` throws if `output.length + 102 > 256`.
- Ticket root `<ticket>/🗑️generated/e/<32 hex>` = 153 chars (127 bytes). 153 + 102 = 255, passes with 1 char to spare.
- Worst Playwright worker suffix `/.playwright-artifacts-95/traces/00000000000000000000-00000000000000000000-retry2-recording191.network` = 102 chars, exactly the reserve. Resource file `/.playwright-artifacts-95/traces/resources/<40hex>.jpeg` = 88.
- Typical per-test suffix `/<test-dir>/error-context.md` ≈ 71 for a realistic test dir. Fine.
- The 1028 e2eprep note said the default output is 207 chars; I measure the default root `demonstrator/dist/reports/e/<32 hex>` at 105 chars (113 bytes). The 207 figure does not reproduce; use 105.

Cleanest fix: leave `SEMIO_TICKET_DIR` unset. The guard then has 49+ chars of headroom and no code changes. After the run, move (`mv`, not copy) `♻️mit-bestand/🧺️demonstrator/dist/reports/e/<sid>` into `C/🗑️generated/1029/` for inspection, and delete it when the ticket closes. This keeps the ticket rule (generated output under `🗑️generated`) without lengthening anything. Do not raise `reservedSuffixChars` or `maxPathChars`.

## 4. Per pane: fixture, assertion, components

Pane ids are the brand's `DEMONSTRATOR_PANES`: `generator, koordinator, aggregator, energie, aussuchen, bearbeiten, verfolgen, statik`. The brief's labels `energy` and `fem3d` are plugin names; use `energie` and `statik` in any `-p`/URL or filter. The spec's `pane roster` test checks equality with these ids.

Each pane gets a `boot` test (shell reaches `ready`, each expected window attaches and carries content) plus interaction tests where listed.

| Pane (id) | Windows graded at boot | Interaction test and fixture | Components that must load |
|---|---|---|---|
| generator | `procedural-main` (node graph, widgets > 0), `procedural-preview` (world3d, meshes+instances > 0) | `generator undo-redo`: height widget 6 to 6.5 via inspector; widget ids 7 exact, synapses e1..e6; undo/redo projections equal | procedural plugin, generation3d preview (edit mode); runtime variant `generator` (primary) |
| koordinator | `cad-play-shape`, `cad-play-building`, `cad-play-energy`, `cad-play-structure-classic` (all world3d, content > 0) | `koordinator undo-redo`: catalogue `cad-play-catalogue.spatial.shape.primitive.box` adds one `cad-play-document.shape…` row; undo removes it | cad (`semio-hub-cad`), variant `koordinator` |
| aggregator | `puzzle3d-main-top`, `puzzle3d-main-perspective` (world3d, same object set) | `aggregator undo-redo`: kind `Hexagonal Cut Concrete Forest Left`, seed `seed-left-001`, mesh `hexagonal-cut-concrete-forest-left.glb`, 11 vortices | puzzle plugin (`semio-hub-puzzle` wasm), variant `aggregator` |
| energie | `energy.model.3d` (world3d, meshes > 0), `framework.window.tree` (Structure, >=1 treeitem), `framework.window.table` (Zones, exactly 1 row), `energy.simulation` (tree, >=1 treeitem) | `energie run-cancel` (simulation cases: `tool.energySimulation` finalize/abort text); example `bestest-600` | energy plugin, variant `energy`, activate-lane `energy` |
| aussuchen | `sourcing-pool` (table, rows > 0); `sourcing-curated`, `sourcing-grid` allowed empty; `sourcing-preview` text placeholder `Keine Auswahl` or `No selection` | `aussuchen preview`: click stock row `Glulam GL24h 200×400` (`beam-glulam-gl24h`), preview must become a world3d host with meshes; `aussuchen undo-redo`: curation steps (increment/undo/redo/remove), revisit via bearbeiten | sourcing plugin, variant `aussuchen` |
| bearbeiten | `process-workpiece` (world3d, content > 0) | `bearbeiten undo-redo`: step `wall-saw-cut` (`Wall Saw Slab Cut`, wallSaw/wallCut), toggle Enabled to false, undo/redo over 7 steps | process plugin, variant `bearbeiten` |
| verfolgen | `gis2d-main` (tiledMap, canvas paints >1 flat colour) | `verfolgen undo-redo`: add feature `Acceptance Reuse Marker` at lon 0 lat 0, positions +1, routes/regions unchanged; undo/redo | gis plugin + tile artifacts; needs a real GPU adapter (Metal on darwin) |
| statik | `fem3d-model`, `fem3d-results` (both world3d, content > 0) | `statik undo-redo`: result scale 300 to 600 to 300 (caption `Case: dead`) | fem3d plugin, variant `fem3d`, activate-lane `fem3d` |

Global: `overview chrome` (8 cards, silhouettes, 2 intro paragraphs), `funding logos` (3+ visible logos decode), `pane roster`. Every test also collects `pageerror` and significant console errors (soft expect); any uncaught error in any pane fails all tests.

Known soft spots in the spec's own notes: `energie` depends on the energy descriptor (see section 5). `statik` default example is `concrete-forest` per the brand, but measured `demo` on the standalone lane. Either example passes the non-empty assertion.

## 5. Stale artifacts and config that can break the first run

- `♻️mit-bestand/🧺️demonstrator/dist` does not exist, so nothing stale is staged. `prepare-e2e` checks marker files of every runtime component (`.semio-artifact.json` with `🌉️bridge.js` and `🔣️.json`) and the staged playground session per variant. Its loop uses `pluginModuleDirNames.slice(2)`, so the first two plugin dirs are skipped. Check that this matches the layout when the first run hits it.
- `test-results/` in the demonstrator dates from 09-16 and is not used (the config writes to `--output`).
- Old activation evidence: `1028-audit-…` act01 (10/28 16:30) and act02 (stopped 18:38). The `activate-prototype` cache class is `deterministic`, so input changes make act03 re-run. Do not reuse act02 logs.
- Energy descriptor `✏️s/🔌️plugins/🔋️energy/🔣️.json` still declares the editor window as `block-list` and omits `energy.model.3d` (acknowledged in the spec note). If the served build reads that descriptor, the `energie` boot assertion fails on window kind. Verify on the first served run; the spec was written from the live DOM.
- Demonstrator `node_modules` symlink dangles (section 3).
- Peer-touched files named in the handover: `energy` `handle_intent_frame` (6-arg, not resumable) and reactor `poll_owned` refusing batches over 1 event. Both are runtime risks that show up as pane errors, not compile errors. See `1028-HANDOVER.md` lines 36-38.

## 6. Risks ranked

1. BLOCKER: compile frontier. Last `puzzleplugin` check fails in `semio-framework-plugin-instance-dispatch` (8 errors). Activation cannot pass until every component wasm check is green (`1028-HANDOVER.md`).
2. BLOCKER: `workspace:deps-browsers` storage failure (section 3). It blocks `prepare-e2e` even if activation is green. Diagnose the creator-claim sidecar on the partition holding `tools/ms-playwright` before the next attempt; run the browsers target alone, detached, with an rc marker, first.
3. HIGH: output-path guard with 1 char headroom if `SEMIO_TICKET_DIR` is set. Fix: leave it unset, then `mv` the output into `C/🗑️generated/1029/` (section 3).
4. HIGH: `energie` window kind vs stale descriptor; `statik` default example (demo vs concrete-forest).
5. MEDIUM: GPU. On devcontainer use `DEMONSTRATOR_E2E_GPU=swiftshader`; `verfolgen` map then fails by design. On darwin the default Metal path is what the spec expects.
6. MEDIUM: wall clock. Readiness 600 s, per-test 240 s, one worker, 21 tests. Expect 30-60 min on a warm cache, longer on cold. Run detached with an rc marker.
7. LOW: demonstrator `node_modules` dangling symlink; `serve` on 6029 and `test-e2e` are independent (no conflict).

## Sequence the coordinator should follow after compile green

1. Confirm every component wasm check green (`C/🗑️generated/1028/*chk*.rc`).
2. Fix `workspace:deps-browsers` first (risk 2). It is cheap to verify alone.
3. `act03` (activate-prototype). Expect the prepare-prototype fan-out, then the `activate` validation.
4. Optional `serve` on 6029 for a manual probe. Record the pid. Probe with `C/🔍️landing-eight-pane-probe.ts`, `PROBE_BASE_URL=http://127.0.0.1:6029/`, `PLAYWRIGHT_BROWSERS_PATH` set to the browser dir from step 2.
5. `test-e2e` with `SEMIO_TICKET_DIR` unset (see section 3), then move the output into `C/🗑️generated/1029/`.

Result path: this file. No other output written.
