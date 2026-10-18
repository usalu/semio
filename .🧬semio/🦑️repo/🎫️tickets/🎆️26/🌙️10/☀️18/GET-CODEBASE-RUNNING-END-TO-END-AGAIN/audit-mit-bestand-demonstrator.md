# Audit: mit-bestand demonstrator (read-only, 2026-10-18)

Scope: `♻️mit-bestand/` (demonstrator, bericht, präsentation), the framework/hub pieces it depends on, and `.vscode/launch.json`. Nothing was built, started, or edited. Line numbers refer to the files as they are now. Launch-file lines are from the live `.vscode/launch.json` unless marked seed.

## 1. What it is

- **Demonstrator** = one React SPA, `♻️mit-bestand/🧺️demonstrator/` (`📋️project.json`, `🏗️builder/🌐️vite/🟦️.ts`, `🌐️.html`, `🟦️.tsx`). Host `demonstrator.entwerfen.mit-bestand.de` (`🔨️modules/🧩️runtime/🔣️.json`). Page title "Entwerfen mit Bestand · Demonstrator".
- It shows **8 panes**, defined in `🪧️brand.ts` `DEMONSTRATOR_PANES` (L918+):

  | pane id (brand.ts) | playground variant | runtime variant (🔣️.json) | app (hub Cargo.toml) |
  |---|---|---|---|
  | generator (L920) | generator | generation3d | s.procedural.generation3d@1 |
  | koordinator (L~930) | koordinator | koordinator | s.cad.cad@1 |
  | aggregator (L~940) | aggregator | aggregator | s.puzzle.puzzle3d@1 |
  | energie (L950) | energy | energy | not checked |
  | aussuchen | aussuchen | aussuchen | s.sourcing.curation@1 |
  | bearbeiten | bearbeiten | bearbeiten | s.process.process3d@1 |
  | verfolgen | verfolgen | verfolgen | s.gis.gismap@1 |
  | statik (L990) | fem3d | fem3d | not checked |

- Plugin sources are in the hub, not in `✏️s`. Playground rows are in `🌎️hub/🧩️compositions/🎪️demonstrator/🧩️component/📦️packages/🦀️rust/Cargo.toml` (L37–~110, crate `semio-hub-demonstrator-component`). Other pane rows come from procedural (generation3d), energy and fem. `✏️s/🔌️plugins/🎪️demonstrator/` holds only the artifact crate `🗿️artifacts/🎪️playground` (`semio-s-artifact-demonstrator-playground`) and a bridge. It has no playground metadata.
- Renderer: React is the main path (`SEMIO_RENDERER=react`, port 6107 per Cargo L41). A wgpu variant also exists (`…🧊️wgpu🌐️wasm` launch, port 6207).
- **Print/Zwischenbericht output: none in the demonstrator.** The demonstrator's output is a static site (`build` → `🔨️modules/📦️site`).
- Other deliverables in `♻️mit-bestand/` (`package.json` workspaces, L1 onward):
  - `📋️bericht/` (`@semio-tech/mit-bestand-bericht`): Zwischenbericht (`📋️zwischenbericht/📋️zwischenbericht.tex`), Forschungsbericht, Kompaktbericht, Anhang, actor-network generator (`🔨️modules/👥️actor-network`), tests. Builds `dist/documents/zwischenbericht` via `@semio-tech/print:*` (`🧰️framework/🛍️products/📓️print`).
  - `🎤️präsentation/📅️33.projektetage/` (`@semio-tech/mit-bestand-praesentation-projektetage`): `dev` on port 6050.
  - `🖼️asset/` (logos, `abbau-aufbau`, a `.3dm` logo).

## 2. Launch entries (`.vscode/launch.json`, all `type: node-terminal`)

| Name | Line (name / command) | Command |
|---|---|---|
| 🛠️dev♻️mit-bestand🧺️demonstrator | 88005 / 88008 | `bun repo run @semio-tech/mit-bestand-demonstrator:dev`, env `MIT_BESTAND_DEMONSTRATOR_PORT=6029` |
| ♻️activate🏚️mitbestand🎪️demonstrator | 88027 / 88030 | `…:activate-prototype --configuration=prototype` |
| 🖥️serve🏚️mitbestand🎪️demonstrator | 88042 / 88045 | `…:serve` |
| 🛠️activate-lanes-dev🧺️demonstrator | 175795 / 175798 | `…:activate-lanes-prototype --configuration=prototype` (name says "dev", runs prototype) |
| 📦️prepare-dev🧺️demonstrator | 175810 / 175813 | `…:prepare-prototype --configuration=prototype` |
| 📦️prepare-e2e🧺️demonstrator | 175825 / 175828 | `…:prepare-e2e` |
| 📦️prepare-release🧺️demonstrator | 175840 / 175843 | `…:prepare-release --configuration=release` |
| 🛠️serve-e2e🧺️demonstrator | 175855 / 175858 | `…:serve-e2e` |
| ⚖️test-e2e🧺️demonstrator | 141608 / 141611 | `…:test-e2e` |
| ⚖️test♻️demonstrator📇️runtime-catalog | 77070 / 77073 | `…:test-runtime-catalog` |
| ⚖️gate♻️mit-bestand🧺️demonstratorengine-producers | 84702 / 84705 | `bun repo exec --projects=@semio-tech/mit-bestand-demonstrator --exclude-dependencies -- bun test …/🧪️tests/🧩️engine-producers/🟦️.ts`, env `SEMIO_BUILD_MODE=ship` (L84708, stale) |
| 🛠️dev♻️mit-bestand🧺️demonstrator⚛️react | 157731 / 157734 | `bun repo run workspace:dev -- demonstrator` (STALE, see §4) |
| 🛠️dev♻️mit-bestand🧺️demonstrator🧊️wgpu🌐️wasm | 157755 / 157758 | `bun repo run workspace:dev -- demonstrator` (STALE) |
| Präsentation entry | command at 87972 | `bun repo run @semio-tech/mit-bestand-praesentation-projektetage:dev` |
| Bericht watch entries | 87990 / 87993 (bericht); 109411 / 109414, 109426 / 109429, 109441 / 109444 (zwischen/forschungs/kompakt) | `…mit-bestand-bericht:watch`, `:watch-zwischenbericht`, `:watch-forschungsbericht`, `:watch-kompaktbericht` |
| Bericht build entries | commands at 170875, 170890, 170905, 170920, 170935 | `…mit-bestand-bericht:akteursnetz-validate`, `:build-forschungsbericht`, `:build-kompaktbericht`, `:build-zwischenbericht`, `:generate-actor-network` |
| Demonstrator test-e2e (second entry) | 132089 | `bun repo run @semio-tech/mit-bestand-demonstrator:test-e2e` |

- All targets referenced by these entries exist in their `📋️project.json` (checked by JSON parse), except the `workspace:dev` form (see §4).
- The seed file `.vscode/🧩️launch.seed.jsonc` has 21 `workspace:dev -- ` occurrences. The live file has 335 (`"command": "bun repo run workspace:dev -- …"`). The live file is also much larger (9 MB).

## 3. Build chain

- Root CLI: `package.json` script `repo` → `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts repo`.
- Demonstrator `📋️project.json` (targets: prepare-prototype L13, dev L83, build L114, activate-prototype L142, activate-lanes-prototype L168, serve L193, prepare-e2e L217, serve-e2e, test-e2e L250, prepare L271, test-runtime-catalog):
  1. `prepare-prototype` depends on 9 `@semio-tech/framework-os-dev:prepare-<variant>-react-prototype` plus `@semio-tech/framework-surface-rs:wasm`, `framework-editor-rs:wasm`, `framework-os-node-graph-rs:wasm`, `@semio-tech/puzzle-plugin:wasm`. Its command is the runtime script `prepare` (`🔨️modules/🧩️runtime/📜️script.ts`).
  2. `activate-lanes-prototype` depends on `framework-os-dev:activate-{generator,energy,fem3d}-react-prototype`. Lanes come from `demonstratorActivationLanes()` (`🔨️modules/🧩️runtime/♻️activation/🟦️.ts` L37–48). Other panes are covered by the demonstrator closure.
  3. `activate-prototype` (depends on prepare-prototype + activate-lanes-prototype) runs runtime `activate`. That merges lane receipts into the union receipt and throws on any mismatch (`mergeDemonstratorActivationReceipts`, L72–89).
  4. `dev` / `serve` run runtime `serve`, which calls `serveVite` with `MIT_BESTAND_DEMONSTRATOR_PORT` (default 6029), host 127.0.0.1 (0.0.0.0 with `DEVCONTAINER=true`), and config `🏗️builder/🌐️vite/🟦️.ts`. That config sets `strictPort`, port 6029 (L81), and `semioActivationVitePlugin` on the union receipt (L104).
  5. `build` depends on `prepare` (L116–117) plus the four wasm targets, then runs `🔨️modules/📦️site/📜️script.ts build`. Output is `{projectRoot}/dist/{mode}/site`.
  6. E2E: `prepare-e2e` → `serve-e2e` (runtime `serve-test`, port 0, service record under `♻️mit-bestand/🧺️demonstrator/dist/services/e2e`) → `test-e2e` (Playwright, `PLAYWRIGHT_BASE_URL` from the service).
- Lane receipts live under the repo cache, not under `dist/`. `developmentRuntimeRoot(ws, lane, "prototype", "react")` (`♻️activation/🟦️.ts` L51–53). Staging is `browserProductRootsV1` (`🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/📂️paths/🟦️.ts` L7–8): `<cache>/products/<protocol>/<mode>/{modules,sessions,runtime}`.
- Build mode: `readBuildMode` accepts only `prototype` | `release` (`🧰️framework/🔨️modules/🏃️process/🏗️build/🎚️mode/🟦️.ts` L14–16).
- Env vars in use: `SEMIO_BUILD_MODE` (set per target), `MIT_BESTAND_DEMONSTRATOR_PORT`, `DEVCONTAINER`, `SEMIO_BUILD_INVOCATION_PID` (required by E2E, set by `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts`), `PLAYWRIGHT_BASE_URL` / `PLAYWRIGHT_BROWSERS_PATH` (E2E), `SEMIO_TICKET_DIR` (optional E2E output). Launch-only: `S_OS_PORT`, `SEMIO_PLUGIN`, `SEMIO_APP`, `SEMIO_RENDERER`.
- Ports: demonstrator dev 6029; react/wgpu playground 6107/6207 (Cargo L41); generator 6027/6127 (L50); koordinator 6028/6128; aggregator 6023/6123; aussuchen 6030/6130; bearbeiten 6031/6131; Präsentation 6050; E2E port 0. Probe with `127.0.0.1`, not `localhost` (IPv6 ::1 issue, see memory).

## 4. Stale references (the core result)

Status: **STALE** = points at something that does not exist or no longer behaves as written. **DRIFT** = still works but the description is wrong.

| # | Where (path : line) | Reference | Status | Current truth |
|---|---|---|---|---|
| S1 | `.vscode/launch.json` 157734, 157758 (and ~333 other `workspace:dev -- <plugin>` entries, all playground entries) | `bun repo run workspace:dev -- demonstrator` | **STALE** | Root `📋️project.json` has no `dev` target (JSON check; `workspace:deps-browsers` does exist). The `dev` command (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🛠️dev/🟦️.ts` L24–37) accepts only `storybook`, `storybook-static`, `mcp` and exits with "unknown route". Its error text points at `bun run dashboard run playground:<variant>`, which is the old CLI form. Playground targets are now generated per variant (see S2). |
| S2 | `🧰️framework/…/📥️inference/🟨️.mjs` L1446, L1454 | generated `activate-/prepare-<variant>-react-<prototype\|release>` | OK | This is where lane targets are generated now. The demonstrator and launch names match it. |
| S3 | `♻️mit-bestand/🧺️demonstrator/📋️project.json` `prepare` target L271–~289, and `build` L114–117 (`dependsOn: ["prepare", …]`) | `@semio-tech/framework-os-dev:prepare-<variant>-react` (no profile) | **STALE (likely broken)** | The generator only emits `-react-<prototype\|release>` (L1446–1459). The unsuffixed form is not in `framework-os-dev` `📋️project.json` either. `prepare-prototype` and `prepare-release` are correct. `build` inherits the broken dependency. Static conclusion; confirm by running the target once the CLI works. |
| S4 | `♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/📜️script.ts` L23 | `🔌️plugin/📦️packages/🟦️typescript/dist/<profile>/🔌️plugin-modules` | **STALE** | Real modules root is `browserProductRootsV1(…).modules` under the repo cache (S5). Today `dist/release/` and `dist/🔌️plugin-modules/` exist, and no `dist/prototype/` exists. `PreparationScript` will hit ENOENT on the prototype marker check. |
| S5 | same file L30 | `🔌️plugin/📇️registry/dist/sessions/<variant>/🎮️playground-session/🟦️.ts` | **STALE** | Sessions root is `browserProductRootsV1(…).sessions` (cache). `dist/sessions/` has only some variants (`demonstrator`, `animate`, `block2d`, `dag`, `draw`, …). None of the 8 pane variants exist there yet. |
| S6 | `♻️mit-bestand/🧺️demonstrator/♻️activation` memory note: receipt `…dist/runtime/react/dev/…`, union `dist/♻️activation/dev` | `dev` profile | **STALE (memory)** | Now `prototype` only. Union receipt is `♻️mit-bestand/🧺️demonstrator/dist/♻️activation/prototype` (`♻️activation/🟦️.ts` L26). Lane receipts are under the cache (L52, L76 rejects non-prototype). |
| S7 | memory: `🟨️.mjs ~L903-993` in `🦑️repo/🔨️modules/📚️library/🟨️.mjs` | target inference location | **STALE (memory)** | That file has no `prepare`/`activate` strings. Inference is now `🏗️build/🕸️graph/📥️inference/🟨️.mjs` (playground targets ~L1353–1470). |
| S8 | memory: `bun nx run …:dev`, "Nx-inferred targets", `SKIP_PLUGIN_BUILD` | Nx | **STALE (memory)** | Commands are `bun repo run <project>:<target>`. `SKIP_PLUGIN_BUILD` is still dead (forbidden in `🧫️pipeline.json`). |
| S9 | `♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/🧪️e2e/🟦️.ts` L10, L13; `…/🔨️modules/🧪️e2e/📜️script.ts` L9; `…/🔨️modules/📦️site/📜️script.ts` header | comments and error text "must run through Nx", "owned by Nx" | **DRIFT** | `SEMIO_BUILD_INVOCATION_PID` is still set by the repo build script, so the check works. Only the wording is stale. |
| S10 | `♻️mit-bestand/🧺️demonstrator/🧫️pipeline.json` L43 (`e2e.owner`) and `…/🧫️pipeline.json` L33 (`site.output: "dist/site"`) | `…:e2e` target; output `dist/site` | **DRIFT** | Demonstrator has no `e2e` target (only `prepare-e2e`, `serve-e2e`, `test-e2e`). `e2e.owner` is only an id. Build output is `dist/{mode}/site`, not `dist/site`. |
| S11 | `.vscode/launch.json` L84708 (`SEMIO_BUILD_MODE=ship`) | `ship` | **STALE** | `readBuildMode` accepts only `prototype`/`release` (L14–16). The engine-producers test does not read the variable, so it is inert. |
| S12 | `♻️mit-bestand/🧺️demonstrator/🧪️tests/🎭️acceptance/🟦️.ts` L2 | header names `🧪️demonstrator.acceptance.spec.ts` | **DRIFT** (cosmetic) | File lives at `🧪️tests/🎭️acceptance/🟦️.ts`. |
| S13 | `.vscode/launch.json` L175795 | name `🛠️activate-lanes-dev🧺️demonstrator` | **DRIFT** (cosmetic) | Command runs `activate-lanes-prototype`. |
| S14 | memory: demonstrator composition in `✏️s/🔌️plugins/🎪️demonstrator` | crate composes cad/gis/… with `default-features=false` | **DRIFT** | The playground composition is in the hub (`🌎️hub/🧩️compositions/🎪️demonstrator/🧩️component`). `✏️s` only has the artifact crate. |

Checked and still valid (no stale finding):
- All `project.json` script paths in the demonstrator exist: runtime `📜️script.ts`, `🔨️modules/📦️site/📜️script.ts`, `🔨️modules/🧪️e2e/📜️script.ts`, `🏗️builder/🌐️vite/🟦️.ts`, `🎚️config` files, `🧪️tests/🏛️shell-brand`, `🔨️modules/🧩️runtime/🧪️tests/🟦️.ts`.
- All 92 relative imports under `♻️mit-bestand/🧺️demonstrator` resolve (scripted check). Bare `@semio-tech/*` imports resolve through the `framework-os-dev` exports map (`activation`, `builder/vite`, `vite-plugins`, …). `@semio-tech/s-dev-catalog-ts/generated/*` resolves via the exports map to `🤖️generated/`. `@semio-tech/repo-lib` exists.
- Dependency projects exist: `@semio-tech/framework-os-dev`, `framework-surface-rs`, `framework-editor-rs`, `framework-os-node-graph-rs` (all with `wasm`), `@semio-tech/puzzle-plugin` (hub `🌎️hub/🧩️compositions/🧩️puzzle/📦️packages/🦀️rust`), `workspace:deps-browsers`.
- Bericht targets and paths (`📦️packages/🟦️typescript/📋️project.json`) and `📄️documents/📜️script.ts`, `👥️actor-network/📜️script.ts`, `@semio-tech/print` (`📓️print`) all exist.
- All 8 pane variants have Cargo playground rows (see §1).
- Tickets named in the brief exist (`🌙️09/☀️17/DEMONSTRATOR-*`, `🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS`, `🌙️09/☀️05/MERGE-PRINT-AND-MIT-BESTAND-INTO-CURRENT-BRANCH`, `🌙️10/☀️07/*ZWISCHENBERICHT*` ×3). Ticket for this audit: `🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN`.

## 5. Probes, E2E, and how to judge "runs end to end"

Existing checks:
- `test` → `📜️script.ts test` → vitest (`🧪️tests/🎚️config/🟦️.ts`). In-source suites registered in the root script: `demonstratorruntimebuildvariants`, `demonstratorpanedescription`, `demonstratormaptiles`, `demonstratorcompileclosure`. `panebranding`, `panebootvariants`, `unionreceipt` exist but are not registered there. Check the vitest config glob before relying on them.
- `test-runtime-catalog` (`🔨️modules/🧩️runtime/🧪️tests/🟦️.ts`).
- `canonical-architecture` (`🧪️tests/🏛️shell-brand/🟦️.ts`, long budget).
- `engine-producers` gate (`bun test`, `bun:sqlite`).
- Acceptance E2E (`test-e2e`): Playwright (`node_modules/playwright/cli.js`, config `🔨️modules/🧪️e2e/🎚️config/🟦️.ts`). Spec `🧪️tests/🎭️acceptance/🟦️.ts` has 7 `test(` blocks covering the 8 panes via `#<paneId>` deep links. It waits for `data-shell-ready`, fails on `data-shell-error`, asserts content attributes, and fails on any page error or non-404 console error. The config notes say a real GPU adapter is needed (SwiftShader fails for the TiledMap pane).

How to judge "runs end to end" (all must hold):
1. `activate-prototype` completes, and `mergeDemonstratorActivationReceipts` accepts the lane receipts (exact closure, matching `artifactSha256`).
2. `serve` prints `Demonstrator ready: http://127.0.0.1:6029/`, and `curl http://127.0.0.1:6029/` returns 200.
3. In a browser, each of the 8 `#<paneId>` pages reaches `data-shell-ready` with no `data-shell-error`. Console has no errors except 404s and `[DEBUG] ` lines.
4. `test-e2e` is green (needs `serve-e2e` and `SEMIO_BUILD_INVOCATION_PID`).
5. `test-runtime-catalog` and `engine-producers` are green.
6. For the Bericht: `build-zwischenbericht` yields `dist/documents/zwischenbericht` under the bericht package.

Not verified in this audit (no runs, per rules):
- Whether `prepare-<variant>-react` (S3) fails at runtime. The conclusion is from static reading of `🟨️.mjs` and the project files.
- Whether the lane and session targets exist after a real `prepare`. S4/S5 are static.
- Whether `workspace:dev` has a working alias in the newer CLI. The CLI code says no.
- The six `dev♻️mit-bestand…⚛️react` entries at launch lines ~78833–78953 (other panes) were not individually checked.
