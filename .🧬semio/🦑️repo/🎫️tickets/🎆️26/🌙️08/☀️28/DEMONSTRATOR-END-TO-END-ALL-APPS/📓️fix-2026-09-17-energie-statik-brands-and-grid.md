# Energie + Statik panes — brands, grid, registry, engine-contract (2026-09-17)

Slice: add the ENERGY app as **Energie** (4th pane, top row) and the FEM 3D app as **Statik**
(4th pane, bottom row). Grid becomes **4 columns × 2 rows**. Runtime/activation wiring and the
acceptance `PANE_CASES` were owned by sibling agents and are NOT touched here.

## Files changed

### 1. `♻️mit-bestand/🧺️demonstrator/🪧️brand.ts`
- `ENTWERFEN_MIT_BESTAND_BRAND_IDS` → 8 entries, alphabetical:
  `aggregator, aussuchen, bearbeiten, energie, generator, koordinator, statik, verfolgen`.
- New `//#region 🏷️EntwerfenMitBestandEnergieBrand` → `ENTWERFEN_MIT_BESTAND_ENERGIE_BRAND`
  (`id: entwerfen-mit-bestand-energie`, windowTitle `Entwerfen mit Bestand · Energie`,
  `defaults.exampleId: "bestest-600"`, shared logo, `locks {de, reuse, semio}`, `ephemeral`,
  `replayIntroductionOnLoad`, `assetsDir: DEMONSTRATOR_ASSETS_DIR`).
  Two German steps: `viewport` (Das Energiemodell — zoom/pan/orbit on the 3D model window) and
  `panels` (Katalog-Reiter, with the simulation window in `show`).
- New `//#region 🏷️EntwerfenMitBestandStatikBrand` → `ENTWERFEN_MIT_BESTAND_STATIK_BRAND`
  (`id: entwerfen-mit-bestand-statik`, windowTitle `Entwerfen mit Bestand · Statik`,
  `defaults.exampleId: "concrete-forest"`). Two steps: `viewport` (Das Tragwerksmodell —
  zoom/pan/orbit) and `panels`.
- `DEMONSTRATOR_PANES`: `energie` inserted at index 3, `statik` appended. Final order —
  `generator, koordinator, aggregator, energie, aussuchen, bearbeiten, verfolgen, statik`.
- `DemonstratorPaneSpec` doc comment: `3×2 grid … index 0-2 top row, 3-5 bottom row`
  → `4×2 grid … index 0-3 top row, 4-7 bottom row`.
- `ENTWERFEN_MIT_BESTAND_GENERAL_INTRODUCTION` welcome body: „vereint sechs Werkzeuge" →
  „vereint **acht** Werkzeuge".

### 2. `♻️mit-bestand/🧺️demonstrator/🟦️.tsx`
- `DEMONSTRATOR_GRID_COLUMNS = 3` → `4`.
- Header comment `six live app panes` → `eight live app panes`.
- Pane-boot comment `instead of all six simultaneously — six live WASM plugin boots` →
  `all eight … eight live WASM plugin boots`.

### 3. `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🏷️brand/🟦️.ts`
- Both new brand consts added to the import block, to `SHELL_BRANDS`, and to the re-export list
  (all three lists kept alphabetical).

### 4. `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts`
- Import block (~line 1722) extended with `ENTWERFEN_MIT_BESTAND_ENERGIE_BRAND` /
  `..._STATIK_BRAND`.
- Test renamed `registers all six …` → `registers all eight Entwerfen mit Bestand demonstrator
  shell brands`; the pinned `ENTWERFEN_MIT_BESTAND_BRAND_IDS` array and the per-brand `.id`
  assertions extended to eight.

## Window kind ids used in the introductions

| Brand | Window kind id | Surface | Source of truth |
| --- | --- | --- | --- |
| Energie | `energy.model.3d` | `world-3d` | `✏️s/🔌️plugins/🔋️energy/…/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️model/🦀️.rs` → `pub const WINDOW_KIND_ID` |
| Energie (secondary, in `show`) | `energy.simulation` | tree body | `…/✏️editor/🎭️modes/✏️edit/🪟️windows/⚡️simulation/🦀️.rs` |
| Statik | `fem3d-model` | `world-3d` (`surfaceKind` in the manifest) | `✏️s/🔌️plugins/🏗️fem/🔣️.json`, app `s.fem.fem3d@1/*#editor` |

**Finding — the committed energy plugin descriptor is stale.** `✏️s/🔌️plugins/🔋️energy/🔣️.json`
lists only `framework.window.tree` / `framework.window.table` / `energy.simulation` for
`s.energy.model@1/*#editor` and omits `energy.model.3d` entirely (it also spells
`framework.window.tree` as `block-list`). The editor's real edit-mode layout
(`…/✏️editor/🎭️modes/✏️edit/🦀️.rs:44`) leads with `model_window::WINDOW_KIND_ID`
(= `energy.model.3d`) at `MODEL_VIEWPORT_SHARE = 0.55`. The brand was first authored against the
stale descriptor (panel-only step on `energy.simulation`), then corrected to `energy.model.3d`
with the usual zoom/pan/orbit interactions — this agrees with the sibling's acceptance
`PANE_CASES` note, which was measured against the live :6106 lane and records the same staleness.
Both consts carry a doc comment naming the Rust file as the source of truth, not the JSON.

## Icons (both verified present in `ICON_NAMES`, `🧰️framework/🔨️modules/🖼️assets/🔣️icons/🤖️generated/🖼️icons/🟦️.ts`)
- Energie → `"sun"` (no thermometer/flame/zap exists in the generated set; `lightbulb` was the
  only other candidate).
- Statik → `"fem-app"` — the same icon the fem plugin manifest gives the `s.fem.fem3d@1/*#editor`
  app (`iconId: "fem-app"`).
- Taglines: Energie „Energiebilanz simulieren", Statik „Tragwerk berechnen".

## Verification (all foreground)

1. **Demonstrator vitest** — `cd ♻️mit-bestand/🧺️demonstrator && bun ./📜️script.ts test quick`
   → **3 test files / 13 tests passed**, 8.8 s.
   (Plain `bun ./📜️script.ts test` defaults to the `fundamental` level whose 15 s budget kills the
   run before vitest reports; `quick` is the usable level. Config
   `🧪️tests/🎚️config/🟦️.ts` runs in-source tests of `📜️script.ts` + `🪧️brand.ts` only.)
2. **Acceptance drift guard** — the suite in `🧪️tests/🎭️acceptance/🟦️.ts` is **Playwright**
   (`test-e2e` target), not part of `test`, and was **not run** here (needs `serve-e2e` +
   browsers). Statically checked instead: the sibling has already landed `energie` (line 217) and
   `statik` (line 268) in `PANE_CASES`, and re-running the suite's own `brandPaneIds()` regex
   against the edited `🪧️brand.ts` yields
   `["generator","koordinator","aggregator","energie","aussuchen","bearbeiten","verfolgen","statik"]`
   — identical to `PANE_CASES.map(e => e.paneId)`, so the drift guard matches.
3. **Type check of the demonstrator page** — no tsconfig exists under `🧺️demonstrator/`, and the
   repo root `tsconfig.json` is not wired to any typecheck target. A slice config was written to
   this ticket folder and run:
   `bunx tsc --noEmit -p ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/tsconfig.energie-statik-slice.json"`
   (extends the root config, adds `allowImportingTsExtensions` + `types: ["node"]`, includes
   `🪧️brand.ts`, `🟦️.tsx` and the os-dev brand catalog). Real output: 586 error lines across the
   transitively reached tree. **In the three edited files, only pre-existing errors:**
   - `🪧️brand.ts(276,5)` `document` not in `TutorialTracks` — the aggregator tutorial's empty
     document track, identical at line 274 of the pre-change file.
   - `🪧️brand.ts(913,129)` `import.meta.dir` — the vitest tail block, identical at line 793 before.
   - `🟦️.tsx` 414 / 451 / 480 — untouched code; the `.tsx` edits changed no line counts.
   No error names the new brand literals, the new `DEMONSTRATOR_PANES` rows, or the icon names, so
   the `ShellBrand` / `IconName` shapes type-check.
4. **Engine-contract brand test** —
   `cd 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript && bun ./📜️script.ts test long ../../../../🧪️tests/🔬️engine-contract/🟦️.ts --reporter=verbose --testNamePattern="Entwerfen mit Bestand"`
   → `✓ shell option locks (SEMIO_LOCKED_*) > registers all eight Entwerfen mit Bestand
   demonstrator shell brands` — **1 passed, 633 skipped**, 18.6 s.
5. **Package typecheck** of the same package (`bun ./📜️script.ts typecheck`, i.e.
   `tsc --noEmit -p tsconfig.json`) → 1120 pre-existing error lines repo-wide (this gate is red at
   baseline). `🧑‍💻dev/🏷️brand/🟦️.ts` reports **zero** errors; `🔬️engine-contract/🟦️.ts` errors are
   all at lines far from the edited import block (~1722) and brand test (~9320); `🪧️brand.ts`
   shows only the two pre-existing errors above.

## Not done / out of scope
- `🔨️modules/🧩️runtime/🔣️.json`, `🧫️pipeline.json`, `📋️project.json`, `♻️activation/🟦️.ts`, runtime
  `📜️script.ts`, `🏗️builder/🌐️vite/🟦️.ts`, `🧪️tests/🧪️demonstratorruntimebuildvariants` — sibling agent.
- `🧪️tests/🎭️acceptance/🟦️.ts` — sibling agent (already landed).
- `📋️project.json`'s `prepare-dev` / `prepare-release` `dependsOn` still omit the energy and fem3d
  react-dev prepare targets; that is the runtime sibling's file.
