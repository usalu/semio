# 📋️ DEV Demonstrator Execution Plan (`@semio-tech/mit-bestand-demonstrator`)

Ticket: `2026/11/13/GET-MIT-BESTAND-DEMONSTRATOR-RUNNING-END-TO-END`  
Workspace: `/Users/ueli/Documents/semio` (live-replicated shared worktree — re-read before every edit; zero mutating `git` commands; no worktrees; never run `repo dashboard`; use `/Users/ueli/.local/bin/repo`).

## Current execution

Steps 1 and 4 are already in flight. `activate-prototype` is the single build worker (`🗑️generated/activate-22.log`), with the remote-cache variables unset and port `6029` free. Do not start `dependencies sync`, `:test`, `:test-runtime-catalog`, `workspace:components describe`, or a second `activate-prototype` until this run exits. A walk of `*/🧩️component/🔣️.json` found 75 descriptors, all `appChannelVersion` 25, so step 3 does not run unless a later check names a stale file. Steps 6–10 wait for `Demonstrator ready`.

---

## 1. Ordered Execution Steps

1. **Preflight host, environment, and port `6029` ownership**: Confirm no stale or competing listener holds `127.0.0.1:6029` (only the designated build/dev worker binds `6029`). If `SEMIO_REMOTE_CACHE_ENDPOINT` is exported without `SEMIO_REMOTE_CACHE_TOKEN`, run `repo` commands with `env -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE` to avoid Nx `401 Unauthorized` task skips.
2. **Verify workspace dependencies and fast catalog/unit invariants**: Ensure `/Users/ueli/.local/bin/repo dependencies sync --all` is current when lockfiles change, then run `/Users/ueli/.local/bin/repo run @semio-tech/mit-bestand-demonstrator:test` and `:test-runtime-catalog` (abort and fall back to file inspection if `repo` stalls >2 min on graph/TCC).
3. **Ensure component channel descriptors match pin (`25`)**: If `@semio-tech/framework-os-dev:channel-version-descriptors-check` reports stale `*/🧩️component/🔣️.json` descriptors in the closure, regenerate them via `/Users/ueli/.local/bin/repo run workspace:components describe <id> --profile prototype` (see `🩹patches/describe-stale-descriptors.sh`) rather than hand-editing generated JSON.
4. **Run prototype preparation & union activation (designated build worker only)**: Execute `/Users/ueli/.local/bin/repo run @semio-tech/mit-bestand-demonstrator:activate-prototype`. This compiles the 4 WASM engines (`surface`, `editor`, `node-graph`, `puzzle`), prepares all 9 runtime variants (`aggregator`, `aussuchen`, `bearbeiten`, `energy`, `fem3d`, `generation3d`, `generator`, `koordinator`, `verfolgen`), activates the 3 disjoint closure lanes (`generator`, `energy`, `fem3d`), and writes the 32-component union receipt to `♻️mit-bestand/🧺️demonstrator/dist/♻️activation/prototype`.
5. **Handle shared-worktree compiler races cleanly**: If `activate-prototype` refuses a unit with `Original mutable compiler input changed` (another agent edited Rust/TS sources mid-compile) or `Stale Demonstrator activation lane` (`artifactSha256` mismatch across `generator`/`energy`/`fem3d` lanes), rerun the affected `activate-<lane>-react-prototype` and `:activate-prototype` without touching foreign crates.
6. **Start the DEV server on `127.0.0.1:6029` (single worker)**: Run `/Users/ueli/.local/bin/repo run @semio-tech/mit-bestand-demonstrator:dev` and wait for `Demonstrator ready: http://127.0.0.1:6029`. Keep a single long-lived server instance; Vite's `demonstratorUnionReceiptVitePlugin` and `semioActivationVitePlugin` hot-watch receipt updates.
7. **Verify landing chrome and locale contract**: Load `http://127.0.0.1:6029/` in `en` and `de` (plus unset locale -> `[data-slot="demonstrator-locale-picker"]`), verify the 4×2 `LayeredOverview` card grid, funding/partner logos, and live `[data-slot="demonstrator-locale-switcher"]` without shell remounts.
8. **Probe all eight panes in `en` and `de` against the boot & surface contract**: Deep-link `/#generator`, `/#koordinator`, `/#aggregator`, `/#energie`, `/#aussuchen`, `/#bearbeiten`, `/#verfolgen`, and `/#statik`; verify `data-shell-ready`, window attachment, non-empty surface data attributes/paint, and zero page/console errors.
9. **Triage failures strictly by ownership boundary**: Fix demonstrator-owned shell, brand, Vite route, union-receipt, and non-delegated demonstrator composition issues locally (re-reading every file immediately before editing); log and hand off `FEM`, `GIS`, `trinity`, `jack`, and `staffage` defects to their owning agents.
10. **Run isolated end-to-end acceptance gate**: Ensure Chromium is installed (`/Users/ueli/.local/bin/repo run workspace:deps-browsers`) and run `/Users/ueli/.local/bin/repo run @semio-tech/mit-bestand-demonstrator:test-e2e` (uses an ephemeral OS port via `serve-e2e`, never colliding with `6029`), storing any temporary logs under `🗑️generated/` and deleting `🗑️generated/` once complete.

---

## 2. Definition of "App Runs"

An app in the DEV demonstrator counts as **running** only when all five layers pass in both **English (`en`)** and **German (`de`)**:

1. **Catalog & Activation Union**: Its runtime component and extensions exist under `.cache/b/p/m/browser/prototype` with valid `.semio-artifact.json`, `🌉️bridge.js`, and `🔣️.json`, its staged playground session matches `(variant, pluginId)`, and `dist/♻️activation/prototype` merges all 32 closure components (11 plugins: `cad`, `demonstrator`, `energy`, `fem`, `flow`, `gis`, `playbook`, `procedural`, `process`, `puzzle`, `sourcing`; 21 extensions for `cad`, `flow`, `playbook`, `process`, `sourcing`) with identical `artifactSha256` across lanes.
2. **Shell Boot Outcome**: Navigating to `/#<paneId>` (or opening its card from `/`) mounts `[data-shell-id="<paneId>"]` and resolves `data-shell-ready` within `120s` (never `data-shell-error` or `data-shell-not-found`), with `<html lang>` and `[data-shell-id] lang` matching the active locale (`en` or `de`). Note `generator` boots `runtimeVariant: "generation3d"` (`procedural` plugin) paired with `manifest: "generator"` (`s.procedural.generation3d@1/*#editor`).
3. **Overlay & Chrome Responsiveness**: The app's `replayIntroductionOnLoad` tour (`[id="ui.introduction.skip"]` / `Escape`) dismisses cleanly without blocking pointer/keyboard focus, partner footer credits render, and switching language in the overview updates the live shell without unmounting it.
4. **Default Example & Non-Empty Surface Content**: Every expected default window attaches (`toBeVisible`) and publishes real guest content within `60s` of shell readiness:
   - `world3d`: `.semio-world-3d-host` visible with `JSON.parse(data-meshes-json).length + JSON.parse(data-instances-json).length > 0` and `[data-slot="world-compute-status"]` settled (not `aria-busy="true"` or `cancel|fault|fail|error`).
   - `nodeGraph`: `.semio-node-graph-host` visible with `JSON.parse(data-host-snapshot-json).widgets.length > 0`.
   - `table`: `.semio-table-host` visible with `[data-row-id]` count `> 0` (except `aussuchen`'s `sourcing-curated`, which starts at `0` by design).
   - `tree`: `[role="tree"]` visible with `[role="treeitem"]` count `> 0`.
   - `tiledMap`: `.semio-tiled-map-host canvas` visible and composited centre-60% screenshot has `distinctColors > 1` via same-origin `/osm` + `/vt` proxy on a real GPU adapter (Metal/GL).
   - `placeholder`: `aussuchen`'s `sourcing-preview` shows localized no-selection text (`No selection` / `Keine Auswahl`) initially and upgrades to a non-empty `world3d` scene upon selecting a stock row.
5. **Interactive Round-Trip & Console Cleanliness**: Primary user action (edit + `Cmd/Ctrl+Z` / `Cmd/Ctrl+Shift+Z` undo-redo, or `ControlOrMeta+Enter` tool run + finalize/abort in `energie`) updates projections deterministically, with **zero** uncaught `pageerror` events and **zero** significant `console.error` messages.

---

## 3. Failure Triage: Fix in Demonstrator vs. Leave to Plugin Agents

| Failure Surface / Symptom | Action | Owner & Boundary Rule |
|---|---|---|
| `♻️mit-bestand/🧺️demonstrator/**` (`🪧️brand.ts`, `🟦️.tsx`, `🐚️shell/`, `🔨️modules/🧩️runtime/`, `🏗️builder/🌐️vite/`, `📋️project.json`, locales, logos) | **FIX HERE** | Demonstrator owns landing, locale picker/switcher, `demonstratorPaneBootVariants` (`generator` -> `generation3d`), union receipt merge, Vite static/asset/tile-proxy wiring, and acceptance specs. |
| `🌎️hub/🧩️compositions/🎪️demonstrator/**` & demonstrator-coupled boot issues (`setContributions` host-command classification, `Stale Demonstrator activation lane`, session staging) | **FIX HERE** | Demonstrator composition hub owns packaging `koordinator`, `aggregator`, `aussuchen`, `bearbeiten`, `verfolgen`, and `generator` manifest metadata. |
| Non-delegated pane guest/host integration (`procedural`/`generation3d` preview eval, `cad` composition child scene wiring, `puzzle3d` aggregator, `sourcing` curation, `process3d` workpiece, `energy` simulation) | **FIX / COORDINATE** | Fix demonstrator/host integration defects only after re-reading live files; avoid broad refactors if another agent is actively editing the crate. |
| **FEM** (`✏️s/🔌️plugins/🏗️fem/**`, `🌎️hub/🧩️compositions/🏗️fem/**`) — `semio-hub-fem-component` compile failures, `s.fem.fem3d` solver faults, `fem3d-model` / `fem3d-results` mesh/deformation bugs | **LEAVE TO FEM AGENT** | Only verify `activate-fem3d-react-prototype` inclusion and `ENTWERFEN_MIT_BESTAND_STATIK_BRAND` (`concrete-forest`) in demonstrator; report guest/crate defects to FEM owner. |
| **GIS** (`✏️s/🔌️plugins/🌍️gis/**`, `🌎️hub/🧩️compositions/🌍️gis/**`, `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/**`) — `s.gis.gismap` feature mutations, GeoJSON/terrain, `semio-hub-gis-component` source-cap/compile errors | **LEAVE TO GIS AGENT** | Demonstrator only configures `demonstratorGisMapTileServeMode("serve")` and `/osm` + `/vt` proxy routes (`Play` asset declarations). Leave GIS Rust/plugin code to GIS owner. |
| **Trinity & Jack** (`✏️s/🔌️plugins/🔱️trinity/**`, `🗿️artifacts/🔌️jack/**`, `🗿️artifacts/♻️rewriting/**`, `🌎️hub/🧩️compositions/🔱️trinity/**`) | **LEAVE TO TRINITY / JACK AGENTS** | **Not in demonstrator's 32-component runtime closure.** Never edit these files or kill their Cargo test processes; keep commands scoped to `@semio-tech/mit-bestand-demonstrator:*`. |
| **Staffage** (`🎓️teaching/🏛️architecture/🧍️staffage/**`, staffage mesh authoring scripts/catalogs) | **LEAVE TO STAFFAGE AGENTS** | Demonstrator only serves `/mesh` (`🧰️framework/🔨️modules/🖼️assets/🥽️mesh/📇️catalog.json`) for `aggregator`. Do not hand-edit staffage models or manifests. |

---

## 4. Eight-Pane & Landing Verification Checklist

### Global & Landing (`/`)
- [ ] `http://127.0.0.1:6029/` responds `200`; `dist/♻️activation/prototype` union receipt is valid with 32 components.
- [ ] Unset locale (`fr-FR` context, empty storage) renders `[data-slot="demonstrator-locale-picker"]` with `en` and `de` unselected; choosing persists across reload.
- [ ] Both `en` and `de` render all 8 `[data-layered-card]` window-silhouette cards in a 4×2 grid, 3 visible decoded funding logos in step 3 of general intro, and zero `pageerror` / `console.error`.

### Per-Pane Readiness Matrix (`/#<paneId>` in `en` and `de`)
- [ ] **1. Generator (`/#generator`)** — `variant: generator` | `runtimeVariant: generation3d` (`procedural`, `s.procedural.generation3d@1/*#editor`, default `hexagonal-mushroom-column`)
  - `[data-shell-id="generator"][data-shell-ready]` reached; intro dismisses.
  - `procedural-main` (`nodeGraph`) has `widgets.length > 0` (7 widgets + synapses in `data-host-snapshot-json`).
  - `procedural-preview` (`world3d`) settles `previewEval` with `meshes + instances > 0`.
  - Inspector height edit updates graph + preview geometry and round-trips through `Undo` / `Redo`.
- [ ] **2. Koordinator (`/#koordinator`)** — `variant: koordinator` | `runtimeVariant: koordinator` (`demonstrator`, `s.cad.cad@1/*#editor`, default `demo`)
  - `[data-shell-id="koordinator"][data-shell-ready]` reached; `/cad-assets` static route serves cleanly.
  - All 4 `world3d` windows attach with `meshes + instances > 0`: `cad-play-shape`, `cad-play-building`, `cad-play-energy`, `cad-play-structure-classic`.
  - Catalogue add-item mutates `cad-play-shape` and round-trips through `Undo` / `Redo` without disturbing sibling windows.
- [ ] **3. Aggregator (`/#aggregator`)** — `variant: aggregator` | `runtimeVariant: aggregator` (`demonstrator`, `s.puzzle.puzzle3d@1/*#editor`, default `concrete-forest`)
  - `[data-shell-id="aggregator"][data-shell-ready]` reached; `/mesh` and `/infinite-assets` serve with zero `404`s.
  - Both split `puzzle3d-main` instances (`puzzle3d-main-top`, `puzzle3d-main-perspective`, `world3d`) carry `meshes + instances > 0` and `data-vortices-json`.
  - Adding a catalogue component updates both views and round-trips through `Undo` / `Redo`.
- [ ] **4. Energie (`/#energie`)** — `variant: energy` | `runtimeVariant: energy` (`energy`, `s.energy.model@1/*#editor`, default `bestest-600`)
  - `[data-shell-id="energie"][data-shell-ready]` reached.
  - All 4 windows carry content: `energy.model.3d` (`world3d`, `meshes + instances > 0`), `framework.window.tree` (`tree`, `role="treeitem"` > 0), `framework.window.table` (`table`, `[data-row-id]` >= 1), `energy.simulation` (`tree`, `role="treeitem"` > 0).
  - `ControlOrMeta+Enter` starts `framework.toolRun.*`, reaches finalized status + accepted result, and a restarted run aborts cleanly via `.toolRunAbort`.
- [ ] **5. Aussuchen (`/#aussuchen`)** — `variant: aussuchen` | `runtimeVariant: aussuchen` (`demonstrator`, `s.sourcing.curation@1/*#editor`, default `demo`)
  - `[data-shell-id="aussuchen"][data-shell-ready]` reached.
  - `sourcing-pool` (`table`) loads 12 stock `[data-row-id]` rows; `sourcing-curated` (`table`) and `sourcing-grid` (`world3d`) start empty; `sourcing-preview` shows localized no-selection placeholder.
  - Clicking a pool row upgrades `sourcing-preview` to `world3d` (`meshes + instances > 0`); incrementing stock updates `sourcing-curated` + `sourcing-grid`, survives offscreen revisit, and round-trips `Undo` / `Redo`.
- [ ] **6. Bearbeiten (`/#bearbeiten`)** — `variant: bearbeiten` | `runtimeVariant: bearbeiten` (`demonstrator`, `s.process.process3d@1/*#editor`, default `concrete-forest`)
  - `[data-shell-id="bearbeiten"][data-shell-ready]` reached.
  - `process-workpiece` (`world3d`) settles with `meshes + instances > 0` and all authored machining step rows present in the `artifact` tab.
  - Toggling step `enabled` (or adding a Wall Saw capability step) changes workpiece geometry and round-trips `Undo` / `Redo`.
- [ ] **7. Verfolgen (`/#verfolgen`)** — `variant: verfolgen` | `runtimeVariant: verfolgen` (`demonstrator`, `s.gis.gismap@1/*#editor`, default `demo`) *(GIS plugin owned by GIS agent)*
  - `[data-shell-id="verfolgen"][data-shell-ready]` reached on a real GPU adapter.
  - `gis2d-main` (`tiledMap`) paints `/osm` + `/vt` tiles (`distinctColors > 1` in centre 60% of canvas screenshot) with zero tile console errors.
  - `action.addFeature` increments `gis2d-play-inspector.positions-count` and round-trips `Undo` / `Redo`.
- [ ] **8. Statik (`/#statik`)** — `variant: fem3d` | `runtimeVariant: fem3d` (`fem`, `s.fem.fem3d@1/*#editor`, default `concrete-forest`) *(FEM plugin owned by FEM agent)*
  - `[data-shell-id="statik"][data-shell-ready]` reached.
  - `fem3d-model` and `fem3d-results` (`world3d`) both render `meshes + instances > 0`, and initial solve settles `Case: dead` / `Lastfall: dead` caption without fault/pending suffix.
  - Deformation-scale change (`300` -> updated scale) updates `fem3d-results` projection and round-trips `Undo` / `Redo`.
