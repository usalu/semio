# 🔎️ 1107 Wiring Audit — eight demonstrator panes (lane `aud-wiring`)

Read-only static trace. No repo runs, no sub-agents, no edits outside C. Every claim below was checked by reading or grepping tracked files; nothing was executed, so anything marked UNVERIFIED needs a run to settle.

Paths: manifest `♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/🔣️.json`; brand `♻️mit-bestand/🧺️demonstrator/🪧️brand.ts`; playground rows are `[[package.metadata.semio.playground]]` blocks in `Cargo.toml` files under `🌎️hub/🧩️compositions/…`, which the registry reads (`🧰️framework/…/📇️registry/🎮️playground/🔎️discovery/🟦️.ts`). The generated `PLAYGROUND_BUILD_TARGETS` file is not in the checkout (built at activation), so the rows were read from their source.

## 1. Pane chain (pane → variant → row → app → brand → example → first acceptance action)

| Pane id (brand) | Manifest variant → runtime variant | Playground row (Cargo.toml) | App id | Brand (brand.ts) | Default example (slug present) | First action in acceptance suite |
|---|---|---|---|---|---|---|
| generator | generator → generation3d | demonstrator `…/🎪️demonstrator/🧩️component/…/Cargo.toml:43-50`; runtime row procedural `…/🌀️procedural/…/Cargo.toml:39-45` | `s.procedural.generation3d@1/*#editor` | :512 | hexagonal-mushroom-column (yes) | `/#generator` standalone open; tree-label `height` in artifact tab, fill 6.5 in inspection `procedural-play-inspector.value.input`, Tab, then undo/redo keys on `procedural-main` (acceptance :932) |
| koordinator | koordinator | demonstrator Cargo.toml:52-59 | `s.cad.cad@1/*#editor` | :561 | demo (yes) | click canvas, open catalogue, click `cad-play-catalogue.spatial.shape.primitive.box` (adds a Box), undo/redo (:977) |
| aggregator | aggregator | demonstrator Cargo.toml:61-68 | `s.puzzle.puzzle3d@1/*#editor` | :354 | concrete-forest (yes); kind "Hexagonal Cut Concrete Forest Left", seed `seed-left-001` (yes) | catalogue click adds one object, undo/redo (:1005) |
| energie | energy | energy `…/🔋️energy/…/Cargo.toml:20-24` | `s.energy.model@1/*#editor` | :772 | bestest-600 (yes) | `/#energie`, open tool tab `tool.energySimulation`, Ctrl+Enter runs, run-cancel (:788) |
| aussuchen | aussuchen | demonstrator Cargo.toml:70-77 | `s.sourcing.curation@1/*#editor` | :614 | demo (yes); `beam-glulam-gl24h` (yes) | `/#aussuchen`, increment stock spinbutton, undo/redo/keyboard (:684) |
| bearbeiten | bearbeiten | demonstrator Cargo.toml:79-86 | `s.process.process3d@1/*#editor` | :657 | concrete-forest (yes) | click row `wall-saw-cut`, toggle `enabled`, undo/redo (:1040) |
| verfolgen | verfolgen | demonstrator Cargo.toml:88-96 | `s.gis.gismap@1/*#editor` | :706 | demo (yes) | toggle engagement, `addFeature` with label, execute, undo/redo (:1078) |
| statik | fem3d | fem `…/🏗️fem/…/Cargo.toml:28-32` | `s.fem.fem3d@1/*#editor` | :821 | concrete-forest (yes) | `/#statik`, click Results caption, phase slider Home, scale 300→600, undo/redo (:834) |

Pane roster: `brandPaneIds()` (acceptance :25-31) parses the brand.ts `DEMONSTRATOR_PANES` literal; `PANE_CASES` (acceptance :216-310) matches it (test at :473-474).

## 2. Findings (ranked)

### F1. Aggregator host-config verbs use a bare bounded factory (UNVERIFIED at runtime, affects aggregator)
- `🧊️3d/…/✏️editor/🧰️retained-commands/🦀️.rs:230-235`: `bounded_first_step_tool_proofs!` with `factory: "BoundedFirstStepCommandJobFactory"` and no `factory_type`, tools `setActiveTool`, `setActiveUtility`. This is the exact shape in memory `project-bare-bounded-factory-means-every-action-dead`.
- The file documents it as deliberate (`:216-219`: generic host-configuration proofs, not app-owned tools). Neither verb appears in the puzzle manifest (`📋️manifest/🦀️.rs`), and the framework has no classification row for them either.
- Pane impact if the deliberate reading is wrong: the aggregator's `transform` utility and `tool.fill` steps. brand.ts `:440-450` (transform), `:479-489` (fill), tutorial gestures `:342` and `:346`, UI deltas `:288-289`. Nothing in the acceptance suite clicks these two, so it would not show as a red test.
- Status: intentional by comment, no static evidence of breakage. Needs one live click on each in the aggregator pane.

### F2. Energy and statik playground rows are not branded like the other six (LOW, consistency)
- energy row (`🔋️energy/…/Cargo.toml:20-24`) has no `brand`, `devContribution` or `aliases`. fem3d row (`🏗️fem/…/Cargo.toml:28-32`) has no `brand` or `devContribution`; it has `aliases = ["fem 3d"]` and `distDir`.
- Effect is limited to standalone builds. `🧑‍💻dev/🏗️builder/🌐️vite/🟦️.ts:57-58` resolves the brand from the row, so these builds get `undefined`. The loader `🧑‍💻dev/🧩️contribution/📥️loading/🟦️.ts:19` only demands a contribution for branded rows, so the two configurations are internally consistent.
- The demonstrator page is not affected: `♻️mit-bestand/🧺️demonstrator/🟦️.tsx:92` passes `brand={pane.brand}` from brand.ts, so energie and statik keep their brand chrome in the page.
- Decision for the owner: add brand and devContribution to these two rows, or leave them as intentionally unbranded standalone shells.

### F3. fem3d `distDir` points at an untracked path (LOW, UNVERIFIED)
- fem Cargo.toml:32 `distDir = "✏️s/🔌️plugins/🏗️fem/dist"`. It is the only playground row with a `distDir` among the eight, and no `dist` folder is tracked. Likely stale; the builder reads it at `🧑‍💻dev/🏗️builder/🌐️vite/🟦️.ts` (`playgroundDistDir`).

### F4. brand.ts comments cite a stale descriptor path (LOW, doc drift for energie and statik)
- brand.ts `:752-758` says the committed descriptor is `✏️s/🔌️plugins/🔋️energy/🔣️.json` and that it omits `energy.model.3d`. brand.ts `:765-767` cites `✏️s/🔌️plugins/🏗️fem/🔣️.json`. Neither path is tracked.
- The descriptors that exist are `🌎️hub/🧩️compositions/🔋️energy/🧩️component/🔣️.json` (contains `energy.model.3d` at line 5030, `energy.simulation` at 3477, `framework.window.table` at 1812, `framework.window.tree` at 58) and `🌎️hub/🧩️compositions/🏗️fem/🧩️component/🔣️.json` (contains `fem3d-model` at line 9404).
- So the "descriptor omits the window" claim is out of date. Whether these descriptors match the current rs sources is UNVERIFIED (memory `project-stale-plugin-descriptor-breaks-host-manifest`: regenerate with `describe` after any action or window change).

### F5. Test-side element ids with no literal producer (UNVERIFIED, affects acceptance, not the product)
- verfolgen (acceptance :1078-1100): `framework.window.gis2dMain.engagement.toggle` and `action.addFeature.execute`. No literal in the gis plugin or the framework renderer. `addFeature` maps from the DSL id (`🗿️gismap/…/🕹️commands-and-retained-jobs/🦀️.rs:25`), so the `action.` prefix is probably framework-built.
- energie (acceptance :788+, `simulation/🧫️cases.json` `toolTabId`): `tool.energySimulation`. The energy plugin has `energySimulation` but no literal `tool.energySimulation`. The `tool.` prefix is probably framework-built.
- `framework.category.tool` and `framework.category.display` (energie and statik tab openers): no literal producer found in the plugin or framework trees.
- Confirm with one run of the energie and verfolgen specs.

### F6. Aggregator layout expectation (UNVERIFIED)
- Acceptance expects two instances `puzzle3d-main-top` and `puzzle3d-main-perspective` (acceptance :245; `🧬️edits/🧫️cases.json`). The brand's intro and tutorial target the unsplit base `puzzle3d-main` (brand.ts `:109-112`). Both ids exist in the puzzle plugin, but whether the default layout produces both instances is UNVERIFIED.

## 3. Checked and clean

- **Variant chain.** Manifest rows (`🔣️.json` 5-38) = the eight brand variants. `pipeline.json` `variants` (7-17) lists nine, which is the eight plus `generation3d`. `primaryVariant` is `generator` (25). Module-load check in `🔨️modules/🧩️runtime/🟦️.ts:46-48` throws if any variant has no build row. Every pane runtime variant has a row.
- **Plugin id drift (memory `project-plugin-id-drift-builder-vs-component`).** For all eight compositions, `package = "semio:<id>"` in the component Cargo.toml line 13 matches `package_id("semio:<id>")` in the composition `🦀️.rs`: procedural 75, puzzle 32, sourcing 39, cad 30, process 30, gis 43, energy 44, fem 44. The app prefixes (`s.procedural`, `s.cad`, `s.puzzle`, `s.sourcing`, `s.process`, `s.gis`, `s.energy`, `s.fem`) each occur in their plugin tree. No `builder("…")` drift found.
- **Row parsing.** `🔎️discovery/🟦️.ts:24-51` reads every key the eight rows use, including `viewer_path` (snake case, `:47`). `ports` is required there, and all eight rows declare it.
- **Locale.** `🌐️locale/🔣️.json`: 146 keys, all with non-empty en and de cells. All 121 `demonstratorLabel`/`demonstratorText` keys used in brand.ts exist. The 40 pane keys (8 × 5) exist.
- **Brand ids.** All eight `entwerfen-mit-bestand-*` ids are in `ENTWERFEN_MIT_BESTAND_BRAND_IDS`. Each shell brand is also a `DEMONSTRATOR_PANES` entry, so `DEMONSTRATOR_SHELL_BRANDS` has eight.
- **Window and element ids.** These exist in the plugins: puzzle3d-main, puzzle3d-main-top/-perspective, puzzle3d-play-vortex-show (generated at `🌀️vortex/🦀️.rs:15`), puzzle3d-play-distribution (generated at `joint-weights-and-distribution/🦀️.rs:292`), puzzle3d-fill-count, procedural-main, procedural-preview, procedural-play-inspector (`🔍️inspection/🦀️.rs:42`), cad-play-shape/-building/-energy/-structure-classic, cad-play-catalogue.{typology} (`🛍️catalogue/🦀️.rs:39`), sourcing-pool, sourcing-curated, sourcing-grid, process-workpiece, process3d-play-inspector.step, gis2d-main, gis2d-play-inspector.*-count, energy.model.3d, energy.simulation, fem3d-model, fem3d-results, fem3d-play-results.transport.play.
- **Aggregator action classification (memory `project-unclassified-verbs-abort-plugin-descriptor-probe`).** `📋️manifest/🦀️.rs` has 63 `action_interactive_job` rows. `addObjectKind` (:344), `setVortexShow` (:404), `acceptSuggestion` (:342) and `setFillCount` (:382) are `Migrated`. The retained-commands tool list carries the same four.
- **Other pane plugins.** process uses bulk `.interactive_jobs(Migrated)` (`🪟️editor-impl`, editor line 385). energy loops over tool ids (`📚️examples-and-manifest/🦀️.rs:106`). The builders panic on any `Unclassified` at build time (`energy …/🦀️.rs:73`), so a gap fails loudly, not silently. Per-verb coverage for cad, gis, sourcing, fem, procedural, process and energy was NOT traced one by one.
- **Bare factories (memory `project-bare-bounded-factory-means-every-action-dead`).** Apart from F1, no bare `factory:` without `factory_type` exists in the seven other pane plugins. Every bounded block I opened (procedural, gis, process, cad, fem, energy, sourcing, puzzle 3d retained) carries `factory_type`. The check is per file, so a file that mixes typed and bare blocks would hide a bare one; F1 was found by reading, not by the grep.
- **Composed children (memory `project-composed-child-load-requirements`).**
  - gis: `child_id == target.artifact_id` (`🧩️map-composition/🦀️.rs:35-46`, "gismap-drawing"/"gismap-value"); `type Members` and `genesis_child_pack` on editor (`🧭️play-app-editor` :65, :140) and viewer (:197, :223).
  - cad: `Members` and `genesis_child_pack` on both surfaces (editor `🪟️editor-and-interaction-scope` :66/:73, viewer :91/:101).
  - sourcing: `Members` and `genesis_child_pack` on both (`🧩️sourcing-app` :27/:56, viewer :45/:72); presence child is built with `artifact_id: child_id` (`👥️presence` :397-398).
  - process: `Members` and `genesis_child_pack` on both (editor `🪟️editor-impl` :64/:172, viewer :89/:107). The child-handle equality was not traced.
  - energy: `Members` on both, but no `#[child]` slots and no `ArtifactChild` in the artifacts, so nothing to load.
  - puzzle 3d, fem3d, procedural: no composed children (the puzzle `ArtifactChild` is in the 5D kind-catalogs only).
  - Memory point 3 (store init job) is superseded. The trait default at `🧰️framework/…/📜️declaration/🧩️app/🎛️artifact-app-trait/🦀️.rs:256-266` is now the framework's bounded initializer, and no pane app overrides it.
- **Demonstrator closure.** `demonstrator` `depends-on` (Cargo.toml:20-35) omits energy and fem. This is deliberate: the activation comment (`♻️activation/🟦️.ts:30-37`) says both are unreachable from demonstrator and get their own lanes. The runtime script roots (`📜️script.ts:24`) use all nine targets, so both are covered.

## 4. Not checked

- No runs: no activation, serve, probe or browser session.
- Per-verb `Migrated` coverage for seven of the eight plugins.
- Whether the committed energy and fem descriptors match their rs sources.
- Child-handle equality in process's retained cursors.

## 5. Housekeeping

- A scratch listing I wrote, `C/🗑️generated/aud-wiring-ls.txt` (tracked-file list used for the greps), is still there. I did not delete it because the brief forbids rm inside `🗑️generated`. The coordinator can drop it.
