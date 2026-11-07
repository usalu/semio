# 1105b Audit Panes (aud-panes)

Read-only map of the 8 demonstrator panes. No source file, build, cargo, repo or bun command was run. Inspection used `find`, `grep` and `node` JSON reads only. Paths are relative to the repo root `/Users/ueli/Documents/semio` unless shown absolute. Ticket folder C = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS`.

## Key findings

1. Pane-to-lane mapping in `C/1101-audit-matrix.md` is partly wrong. It maps aggregator to the flow lane. Brand and playground rows say aggregator = `s.puzzle.puzzle3d@1` (puzzle3d plugin). The matrix's "flow" lane evidence for aggregator is not the pane.
2. Variant names differ from pane ids: `generator` pane runs runtime `generation3d`; `energie` pane has variant `energy`; `statik` pane has variant `fem3d`. Source: `♻️mit-bestand/🧺️demonstrator/🪧️brand.ts` `DEMONSTRATOR_PANES` (L911-968) and `♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/🔣️.json`.
3. Playground rows come from `[[package.metadata.semio.playground]]` in Cargo component manifests. `🧰️framework/.../🕸️graph/📥️inference/🎮️playground-targets/🟨️.mjs` (`collectPlaygroundCatalog`, `playgroundSessionTargets`) turns them into `session-<variant>`, `build-<variant>-react` and `build-<variant>-site`. The flattened rows are in `✏️s/🧑‍💻dev/🧩️catalog/🤖️generated/🚀️playgrounds.json` (156 rows).
4. `prepare-<variant>-react-prototype` names are not defined in any file I searched. They are referenced by `♻️mit-bestand/🧺️demonstrator/📋️project.json`, target `prepare-prototype` (L22-66, 9 deps with owner `@semio-tech/framework-os-dev`). The owner package itself did not resolve under `🧰️framework/🛍️products/💻️os` (only `@semio-tech/framework-os` is there), so its target file is unlocated.
5. Test laws: all pane smokes depend on the shared law crate `🧪️testing/🧪️artifact-app-laws` (path only; its Cargo package name did not resolve). It never publishes framework schema documents, so `/link` schemas fail with RefUnresolved (1102-audit-smoke G6). Energie is the only pane that publishes first.
6. Conflicting status: `1102-result-puzzle.md` reports aggregator smoke "boot, both views and all 5 bodies en+de PASS (p9)". `1102-result-rtpanes.md` reports aggregator 0/1 at 18:15 (`pane_smoke.rs:30`, vortices not rendered). Both are cited below; neither is a passing native run of the current tree.

## Shared resolution data

- Demonstrator hub component (plugin id `demonstrator`, one cdylib, `depends-on` = cad, gis, procedural, process, puzzle, sourcing plus 8 flow extensions): `🌎️hub/🧩️compositions/🎪️demonstrator/🧩️component/📦️packages/🦀️rust/Cargo.toml` (`semio-hub-demonstrator-component`, package `semio:demonstrator`). Library: `🌎️hub/🧩️compositions/🎪️demonstrator/📦️packages/🦀️rust/Cargo.toml` (`semio-hub-demonstrator`).
- Playground ports (react/wgpu) from the rows: generator 6027/6127, koordinator 6028/6128, aggregator 6023/6123, aussuchen 6030/6130, bearbeiten 6031/6131, verfolgen 6032/6132; standalone generation3d 6018/6118, energy 6106/6206, fem3d 6087/6187; demonstrator shell 6107/6207. Pipeline dev port 6029 (`pipeline.json`).
- Acceptance suite: `♻️mit-bestand/🧺️demonstrator/🧪️tests/🎭️acceptance/🟦️.ts` (browser, Playwright). Per-pane case list `PANE_CASES` (L212-300). Language cases en-US/de-DE (`LANGUAGE_CASES`, L68-71). Case data files: `🧬️curation/🧫️cases.json`, `🧬️edits/🧫️cases.json`, `🧬️simulation/🧫️cases.json`, `🧬️console/🧫️cases.json`.

---

## Pane 1: generator

| Field | Value |
|---|---|
| Brand / pane id | `entwerfen-mit-bestand-generator`; pane id `generator` (brand.ts L512-517, L911 ff.) |
| Runtime variant (`🔣️.json`) | `generator` -> runtimeVariant `generation3d` |
| Playground rows | (a) `generator` in demonstrator component, react 6027, app `s.procedural.generation3d@1/*#editor`, brand row, devContribution `♻️mit-bestand/🧺️demonstrator/🐚️shell/🧬️schema/🔣️.json`. (b) standalone `generation3d` in `🌎️hub/🧩️compositions/🌀️procedural/🧩️component/📦️packages/🦀️rust` (react 6018, 11 examples) |
| Plugin crate(s), package, manifest | Artifact `semio-s-artifact-procedural-generation3d`: `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/Cargo.toml`. Editor `semio-s-artifact-procedural-3d-editor`: `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📦️packages/🦀️rust/Cargo.toml` |
| Hub component crate | `semio-hub-demonstrator-component` (shared, see above). Standalone: `semio-hub-procedural-component`: `🌎️hub/🧩️compositions/🌀️procedural/🧩️component/📦️packages/🦀️rust/Cargo.toml` |
| App / editor id | `s.procedural.generation3d@1/*#editor` |
| Fixture document | brand `defaults.exampleId` = `hexagonal-mushroom-column` (L517). Smoke loads same id via `SetActiveExample`: MATCH |
| Native mounted tests | Smoke: `✏️s/🧑‍💻dev/🧩️composition/🧪️tests/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🪧️pane-smoke/🦀️.rs` (in crate `semio-s-composition-laws`, `✏️s/🧑‍💻dev/🧩️composition/📦️packages/🦀️rust/Cargo.toml`, target `generation3d-app-laws-editor-unit`, fn `generator_pane`). Boot/preview: `the_demonstrator_boot_example_renders_a_non_empty_preview_scene` (generation3d edit preview unit tests, cited in 1102-result-gen). Editor-routes/mutation laws also live in `semio-s-artifact-procedural-generation3d` `🧪️tests/`. |
| Acceptance cases | `generator boot` (PANE_CASES, paneId generator, 3D window `procedural-main`); `generator undo-redo` (L932; editCases.generator: heightId `height`, 6 -> 6.5, widgets height/radius/sides/profile/extrusion-axis/extrude/column-preview); overview chrome en/de; language switch and picker; funding logos en/de |
| Latest status | Matrix: wasm `gen-hub14.txt` rc 1 (18:02, `stdio-pdf`). Smoke `rtpanes-generator.txt`: E0117 orphan impl fixed (rc 0 check, `rtpanes-seqchk`), then `semio-s-artifact-sequence-sequence-v1` 28 errors (E0433 dsl/json, E0050 InteractiveJob::step, E0599 RetainedWorkOutcome), then foreign flow `node-graph-extras` E0277 (s10, 14:14). No run. `1102-result-gen`: gen-t11 native preview-eval GREEN 8/8 (3D editor/viewer UNVERIFIED). Browser: not run. |

## Pane 2: koordinator

| Field | Value |
|---|---|
| Brand / pane id | `entwerfen-mit-bestand-koordinator`; pane id `koordinator` (brand.ts L561-566) |
| Runtime variant | `koordinator` -> `koordinator` |
| Playground rows | `koordinator` in demonstrator component, react 6028, app `s.cad.cad@1/*#editor`, asset route `/cad-assets` (from `📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets`) |
| Plugin crate(s), package, manifest | Artifact `semio-s-artifact-cad-cad`: `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/📦️packages/🦀️rust/Cargo.toml` (declares `[[test]] cad_editor_unit`). Editor `semio-s-artifact-cad-cad-editor`: `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📦️packages/🦀️rust/Cargo.toml`. Model `semio-s-artifact-cad-cad-model` |
| Hub component crate | `semio-hub-demonstrator-component` (shared). Cad hub component (`semio-hub-cad-component`, named in 1101-result-cad) path not resolved |
| App / editor id | `s.cad.cad@1/*#editor` |
| Fixture document | brand default `demo` (L566). Smoke uses cad `examples::demo::PRIMARY_TEXT` loaded as text (not via example command): MATCH |
| Native mounted tests | Smoke: `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🪧️pane-smoke/🦀️.rs` (mod at `🔬️unit/🦀️.rs:30-31`), target `cad_editor_unit` in `semio-s-artifact-cad-cad` (run `-p semio-s-artifact-cad-cad --test cad_editor_unit -- koordinator_pane`). Other cad targets: `cad_transform_tool`, `cad_panels_inspection` (same crate) |
| Acceptance cases | `koordinator boot` (PANE_CASES: windows cad-play-shape, cad-play-building, cad-play-energy, cad-play-structure-classic, all world3d); `koordinator undo-redo` (L977; catalogueId `cad-play-catalogue.spatial.shape.primitive.box`, rowPrefix `cad-play-document.shape`) |
| Latest status | Matrix: `cad-8.txt` rc 1 (12:47, `stdio-pdf-schema-4`); `cad-t2` running (18:17). Smoke: killed by 3595204 ms `cargo:test` deadline on cold build, 18:33. `1102-result-cad` PARTIAL: cad-editor and native check GREEN (cad-n2, cad-t7); `cad_editor_unit` 135 errors (needs visibility port). `rtpanes-koordinator` s9 12:34 compile FAIL (flow-document E0308); later E0499 foreign (`store/🕰️history/🏁️replay/checkout.rs:46`). Browser: not run. |

## Pane 3: aggregator

| Field | Value |
|---|---|
| Brand / pane id | `entwerfen-mit-bestand-aggregator`; pane id `aggregator` (brand.ts L352-359) |
| Runtime variant | `aggregator` -> `aggregator` |
| Playground rows | `aggregator` in demonstrator component, react 6023, app `s.puzzle.puzzle3d@1/*#editor`, assets `/mesh` and `/infinite-assets` |
| Plugin crate(s), package, manifest | Artifact `semio-s-artifact-puzzle-3d`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/Cargo.toml`. Editor `semio-s-artifact-puzzle-3d-editor`: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📦️packages/🦀️rust/Cargo.toml`. Host `semio-s-artifact-puzzle-3d-host`. Framework crate in the failing path: `semio-framework-os-flow-extensions` (foreign) |
| Hub component crate | `semio-hub-demonstrator-component` (shared) |
| App / editor id | `s.puzzle.puzzle3d@1/*#editor` (the matrix lane "flow" is not the pane; see Key findings 1) |
| Fixture document | brand default `concrete-forest` (L359). Smoke was `nakagin-capsule-tower` (MISMATCH at audit time); test changed to `concrete-forest` via `setActiveExample` (1102-result-rtpanes) |
| Native mounted tests | Smoke: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️pane-smoke/🦀️.rs` (mod at `🔬️gestures/🦀️.rs:11-12`), run `-p semio-s-artifact-puzzle-3d-editor --test editor-gestures -- aggregator_pane` (target name as cited in 1102-audit-smoke, not re-verified). Other puzzle3d editor targets: `editor-surface`, `editor-defects`, `editor-manifest`, `editor-action-producer`, `editor-task-resume` |
| Acceptance cases | `aggregator undo-redo` (L1005; kindId "Hexagonal Cut Concrete Forest Left", seedId `seed-left-001`, vortexCount 11, meshUrl `hexagonal-cut-concrete-forest-left.glb`, windows `puzzle3d-main-top`, `puzzle3d-main-perspective`); `aggregator boot` (PANE_CASES: window `puzzle3d-main`, two instances, world3d) |
| Latest status | Matrix (flow-lane mapping, see finding 1): `rtpanes-aggregator.txt` rc 1 (16:21, `os-flow-extensions` private `work` / missing `take_work`). `rtpanes-aggregator` 18:15: COMPILED AND RAN 0/1, `pane_smoke.rs:30` "puzzle3d-main-top renders vortices in En" (`rtpanes-aggregator.txt:79911`). `1102-result-puzzle` (lane puzzle, p9): boot, both views, 5 bodies en+de PASS (`puzzle-p9.txt`); `addObjectKind` stalls with WorkLimit; add/undo/redo UNVERIFIED. Browser: not run. |

## Pane 4: aussuchen

| Field | Value |
|---|---|
| Brand / pane id | `entwerfen-mit-bestand-aussuchen`; pane id `aussuchen` (brand.ts L614-619) |
| Runtime variant | `aussuchen` -> `aussuchen` |
| Playground rows | `aussuchen` in demonstrator component, react 6030, app `s.sourcing.curation@1/*#editor` |
| Plugin crate(s), package, manifest | Artifact `semio-s-artifact-sourcing-curation`: `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/📦️packages/🦀️rust/Cargo.toml` (declares `[[test]] aussuchen_pane_smoke`, path `…/✏️editor/🧪️tests/🪧️pane-smoke/🦀️.rs`, and `original_host_configuration`). Examples/schema crates: `semio-s-artifact-sourcing-curation-examples`, `-schema` |
| Hub component crate | `semio-hub-demonstrator-component` (shared) |
| App / editor id | `s.sourcing.curation@1/*#editor` |
| Fixture document | brand default `demo` (L619). Smoke `DEMO_STOCK_TEXT` = `examples::demo::PRIMARY_TEXT`: MATCH by id (texts not compared) |
| Native mounted tests | Smoke: `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🪧️pane-smoke/🦀️.rs`. Run `-p semio-s-artifact-sourcing-curation --test aussuchen_pane_smoke -- aussuchen_pane` (1102-result-rtpanes says seq.sh was fixed from `--lib`; the `--lib` run matched 0 tests, 1102-audit-smoke G1) |
| Acceptance cases | `aussuchen preview` (L637); `aussuchen undo-redo` (L684; curation case objectId `beam-glulam-gl24h`, steps increment/undo/redo/keyboardIncrement/remove, revisit via bearbeiten after 31000 ms); `aussuchen boot` (PANE_CASES, Preview window `sourcing-preview` world3d-or-text, `sourcing-pool`, `sourcing-curated`, `sourcing-grid`) |
| Latest status | Matrix: `rtpanes-aussuchen.txt` rc 1 (13:32, `test-original-snapshot-owners`, RetainedWorkOutcome). `1102-result-rtpanes` s10 15:30: COMPILED AND RAN 0/1: bundled example load fails `InvariantViolated: original return source exceeds its admitting lifetime stock` (`rtpanes-aussuchen.txt:73433`). `1102-result-hubplug`: `--lib` RED 74 pass / 20 fail (s9). 1102-audit-smoke: SIGTERM 02:15, no verdict (earlier timeline). Browser: not run. |

## Pane 5: bearbeiten

| Field | Value |
|---|---|
| Brand / pane id | `entwerfen-mit-bestand-bearbeiten`; pane id `bearbeiten` (brand.ts L655-662) |
| Runtime variant | `bearbeiten` -> `bearbeiten` |
| Playground rows | `bearbeiten` in demonstrator component, react 6031, app `s.process.process3d@1/*#editor` |
| Plugin crate(s), package, manifest | Artifact `semio-s-artifact-process-process3d`: `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/📦️packages/🦀️rust/Cargo.toml`. Editor `semio-s-artifact-process-process3d-editor`: `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📦️packages/🦀️rust/Cargo.toml` (declares `[[test]] process3d_editor_unit`, `process3d_workpiece_window`, `process3d_panels_inspection`, `process3d_panels_artifact`). Host `semio-s-artifact-process-process3d-host`, examples `semio-s-artifact-process-process3d-examples` |
| Hub component crate | `semio-hub-demonstrator-component` (shared) |
| App / editor id | `s.process.process3d@1/*#editor` |
| Fixture document | brand default `concrete-forest` (L662). Smoke was `demo` (MISMATCH); test changed to `examples::concrete_forest::PRIMARY_TEXT` (1102-result-rtpanes, tests dir only) |
| Native mounted tests | Smoke: `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🪧️pane-smoke/🦀️.rs` (mod at `🔬️unit/🦀️.rs:89-90`). Run `-p semio-s-artifact-process-process3d-editor --test process3d_editor_unit -- bearbeiten_pane`. Orphan not declared in Cargo: `…/✏️editor/🧪️tests/🪧️pane-smoke-root/🦀️.rs` (G8) |
| Acceptance cases | `bearbeiten undo-redo` (L1040; stepId `wall-saw-cut`, label "Wall Saw Slab Cut", machine `wallSaw`, capability `wallCut`, 7 step ids); `bearbeiten boot` (PANE_CASES: `process-workpiece` world3d) |
| Latest status | Matrix: `rtpanes-bearbeiten.txt` rc 1 (11:54, cargo deadline, no test ran); wasm `extfix-process-concrete-r4.txt` rc 0 (13:15). 1102-audit-smoke: rc 1 at 23:48, ~90 E0425 in process3d editor closure. `1102-result-rtpanes` s9 11:48 compile FAIL (`renew_turn` missing), later foreign E0382 (`value/🛬️decode/…`). `1102-result-hubplug`: process3d host GREEN 9/9 (p2). Browser: not run. |

## Pane 6: verfolgen

| Field | Value |
|---|---|
| Brand / pane id | `entwerfen-mit-bestand-verfolgen`; pane id `verfolgen` (brand.ts L704-711) |
| Runtime variant | `verfolgen` -> `verfolgen` |
| Playground rows | `verfolgen` in demonstrator component, react 6032, app `s.gis.gismap@1/*#editor`, engines `./🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust`, tile-proxy `/osm` (cache `.🧬semio/🗺️map/osm-tiles`) |
| Plugin crate(s), package, manifest | Artifact `semio-s-artifact-gis-gismap`: `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/Cargo.toml`. Editor `semio-s-artifact-gis-gismap-gis2d`: `…/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📦️packages/🦀️rust/Cargo.toml`. Test crate `semio-s-artifact-gis-gismap-gis2d-unit-tests`: `…/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️laws/📦️packages/🦀️rust/Cargo.toml`. Demo asset `semio-s-artifact-gis-gismap-demo-asset` (`…/🖼️assets/🎬️demo`). (`…` = `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap`.) |
| Hub component crate | `semio-hub-demonstrator-component` (shared) |
| App / editor id | `s.gis.gismap@1/*#editor` |
| Fixture document | brand default `demo` (L711) = gis demo asset `DEMO_DSL`. MATCH by id (texts not compared) |
| Native mounted tests | Smoke: `…/✏️editor/🧪️tests/🪧️pane-smoke/🦀️.rs` (mod at `🔬️laws/🦀️.rs:20-21`). Run `-p semio-s-artifact-gis-gismap-gis2d-unit-tests --lib -- verfolgen_pane`. Map tiles need GPU (acceptance note) |
| Acceptance cases | `verfolgen undo-redo` (L1078; label "Acceptance Reuse Marker", delta 1, collection `positions`, lon/lat 0/0); `verfolgen boot` (PANE_CASES: `gis2d-main`, tiledMap) |
| Latest status | Matrix: `gis-11.txt` rc 1 (16:37, `os-flow-extensions`, `stdio-pdf-schema-4`); `rtpanes-verfolgen.txt` rc 1 (15:02, cargo deadline). `1102-result-rtpanes` 19:08: COMPILED AND RAN 0/1, `document-load.history-invalid … Stale` (`rtpanes-verfolgen.txt:85575`). `1102-result-stale`: Stale FIXED where reachable, verfolgen smoke still RED on harness custody. `1102-result-gis`: `semio-s-artifact-gis-gismap-tests --lib` 84/84 GREEN (gis-t16), not the pane smoke target. `1102-result-hubplug`/1101 unowned: `semio-s-artifact-stdio-docx-base-model`, `…tiff-mutation-regressions` (gis-9). Browser: not run. |

## Pane 7: energie

| Field | Value |
|---|---|
| Brand / pane id | `entwerfen-mit-bestand-energie`; pane id `energie` (brand.ts L772-777, row `variant: "energy"`) |
| Runtime variant | `energy` -> `energy` (`🔣️.json`: `energie` is NOT the variant; `energy` is) |
| Playground rows | `energy` standalone: `🌎️hub/🧩️compositions/🔋️energy/🧩️component/📦️packages/🦀️rust/Cargo.toml` (`semio:energy`), react 6106, app `s.energy.model@1/*#editor`, 15 examples. No demonstrator-component row for energy |
| Plugin crate(s), package, manifest | Artifact `semio-s-artifact-energy-model`: `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/📦️packages/🦀️rust/Cargo.toml`. Editor `semio-s-artifact-energy-model-editor`: `…/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📦️packages/🦀️rust/Cargo.toml` (`[[test]] authoring_law`). Mounted tests crate `semio-s-artifact-energy-model-editor-mounted-tests`: `…/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️mounted-bestest/📦️packages/🦀️rust/Cargo.toml` (lib only, no `[[test]]`) |
| Hub component crate | `semio-hub-energy-component`: `🌎️hub/🧩️compositions/🔋️energy/🧩️component/📦️packages/🦀️rust/Cargo.toml`; lib `semio-hub-energy`: `🌎️hub/🧩️compositions/🔋️energy/📦️packages/🦀️rust/Cargo.toml` |
| App / editor id | `s.energy.model@1/*#editor` |
| Fixture document | brand default `bestest-600` (L777). Test builds `semio_s_plugin_energy_engine_bestest::model("600")` in Rust, bypassing the example command (G4) |
| Native mounted tests | `…/🔬️mounted-bestest/🦀️.rs` (run `-p semio-s-artifact-energy-model-editor-mounted-tests --lib`); fn dispatches load-example action at `:30` (rewritten by energy lane, 1102-result-rtpanes) |
| Acceptance cases | `energie run-cancel` (L788; simulation cases: paneId `energie`, toolTabId `tool.energySimulation`, completion automatic, final/aborted/noResult regexes, en+de); `energie boot` (PANE_CASES: tree surface, 15 treeitem rows) |
| Latest status | Matrix: `rtpanes-energie.txt` rc 1 (10:45, `energy-model-snapshot` `RetireOwned`). Wasm `energy-26.txt` rc 0 (16:59). `1102-result-energy` AMBER, UNVERIFIED (bridge patch applied, etest-11 cold compile timeout). `1102-result-testkit`: B3 schema-publication patch makes the former RefUnresolved point pass. `1102-result-rtpanes` s10 13:25 and s9: 0/2 `WorkLimit` at load-example dispatch (`rtpanes-energie.txt:96661`/`:96710`). 1102-audit-smoke: 0/2 at 23:00, fixed in test, not rerun. Browser: not run. |

## Pane 8: statik

| Field | Value |
|---|---|
| Brand / pane id | `entwerfen-mit-bestand-statik`; pane id `statik` (brand.ts L821-826, row `variant: "fem3d"`) |
| Runtime variant | `fem3d` -> `fem3d` |
| Playground rows | `fem3d` standalone: `🌎️hub/🧩️compositions/🏗️fem/🧩️component/📦️packages/🦀️rust/Cargo.toml` (`semio:fem`), react 6087, app `s.fem.fem3d@1/*#editor`, 4 examples. No demonstrator-component row for fem3d |
| Plugin crate(s), package, manifest | Artifact `semio-s-artifact-fem-3d`: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/Cargo.toml`. Engine `semio-s-artifact-fem-3d-engine` (`…/🧊️3d/⚙️engine/🧊️3d/`). Editor `semio-s-artifact-fem-3d-editor`: `…/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/📦️packages/🦀️rust/Cargo.toml` (`[[test]] editor_app`, `editor_app_2`, `statik_pane_smoke`). Engine modules `semio-s-fem-*` under `✏️s/🔨️modules/🏗️fem/⚙️engine/` (for example `semio-s-fem-model`, `semio-s-fem-static`) |
| Hub component crate | `semio-hub-fem-component`: `🌎️hub/🧩️compositions/🏗️fem/🧩️component/📦️packages/🦀️rust/Cargo.toml` |
| App / editor id | `s.fem.fem3d@1/*#editor` |
| Fixture document | brand default `concrete-forest` (L826). Smoke was `examples::demo::ID` (MISMATCH); changed to `concrete-forest` in tests dir (1102-result-rtpanes) |
| Native mounted tests | Smoke: `…/🧊️3d/…/🌐️any/✏️editor/🧪️tests/🪧️pane-smoke/🦀️.rs` (self-contained imports); target `statik_pane_smoke` -> `🧪️tests/🪧️pane-smoke-target/🦀️.rs` (minimal context, added in window W4). Run `-p semio-s-artifact-fem-3d-editor --test statik_pane_smoke -- statik_pane` (per 1102-result-rtpanes; not re-verified). `editor_app` (~150 stale-test errors) and `editor_app_2` are not usable for this pane |
| Acceptance cases | `statik undo-redo` (L834; fem case: panelTabId `fem3d.panel.results`, caption `Case: dead`, initialScale 300, changedScale 600, steps change/undo/redo); `statik boot` (PANE_CASES: Model and Results, both world3d) |
| Latest status | Matrix: `rtpanes-statik.txt` rc 1 (12:49, stdio-pdf-schema-4 E0432). `fem-24.txt` 3d rc 1 (17:53, `stdio-pdf`). `1102-result-rtpanes` s9 03:00: `editor_app` compile FAIL; s10 statik app-2 target stale; dedicated target added, queued s11. Never compiled to a verdict. `1102-result-fem`: 3d snapshot tests 20/20 at fem-t18, later W4 edits UNVERIFIED (fem-t35 blocked by foreign stdio). Browser: not run. |

---

## Shared crates (used by 2 or more panes)

| Crate / artifact | Panes (and variants) | Role | Current status |
|---|---|---|---|
| `semio-hub-demonstrator-component` (`🌎️hub/🧩️compositions/🎪️demonstrator/🧩️component/📦️packages/🦀️rust/Cargo.toml`) and lib `semio-hub-demonstrator` (`🌎️hub/🧩️compositions/🎪️demonstrator/📦️packages/🦀️rust/Cargo.toml`) | generator, koordinator, aggregator, aussuchen, bearbeiten, verfolgen (6 panes); also the `demonstrator` playground row | One cdylib serves 6 panes | Hub wasm checks per 1101-result-hubplug / 1101-result-gen; no native test |
| Plugin artifacts pulled by the demonstrator hub `depends-on`: `semio-s-artifact-cad-cad`, `…-gis-gismap`, `…-procedural-generation3d`, `…-process-process3d`, `…-puzzle-3d`, `…-sourcing-curation` | the 6 demonstrator panes above, one artifact each | Pane content | cad: cad-editor GREEN, `cad_editor_unit` 135 errors; gis: 84/84 gismap-tests, pane smoke Stale; procedural: gen-t11 preview GREEN; process3d: host GREEN 9/9, editor E0425 in smoke; puzzle: smoke partly PASS (p9) vs 0/1 (18:15); sourcing: `--lib` 74/20 red |
| Shared test law `artifact_app_laws` (`🧪️testing/🧪️artifact-app-laws`, `pane-smoke-law/🦀️.rs`) | generator (by-example, no redo), statik (by-example), koordinator, bearbeiten, verfolgen, aussuchen, energie (uses shared load law) | Boot, action, undo, redo; no render, no locale, no declared-verb conformance | Never publishes framework schema docs: RefUnresolved for `/link` schemas (G6). Stale fault for every pane that loads a document (1102-result-stale). Aussuchen: InvariantViolated (lifetime stock). Energie: WorkLimit at load-example dispatch |
| `semio-framework-os-flow-extensions` (framework, foreign) | aggregator (16:21 rc 1), verfolgen (gis-11, 16:37) | Extension owner `take_work` drift | Red; owned by no lane per 1101-audit-matrix |
| `semio-s-artifact-stdio-pdf` / `…-pdf-schema-4` / `…-pdf-schema-5` (stdio2 lane) | generator (`gen-hub14`), koordinator (`cad-8`), verfolgen (`gis-11`), statik (`fem-24`, `rtpanes-statik`) | PDF export chain transitive dependency | Red in 4 panes' newest logs; act15 names `schema-5` while earlier logs say `schema-4` (matrix contradiction 7) |
| `RetainedWorkOutcome` / `admit_borrowed` API drift (framework, foreign) | aussuchen (`test-original-snapshot-owners`), verfolgen (`expect` on RetainedWorkOutcome), generator (`sequence-v1` E0599) | Test and crate port drift | Red, each pane blocked |
| `semio-s-artifact-sequence-sequence-v1` | generator only (but also a dev-dependency of the generation3d pipeline per 1102-result-rtpanes) | Sequence editor | 28 errors, not owned by pane |
| Engine/mesh stack `semio-s-fem-*` (`✏️s/🔨️modules/🏗️fem/⚙️engine`) | statik; also standalone fem3d variant | Fem engine | Engine wasm GREEN (fem-4); 3d snapshot tests UNVERIFIED |
| Process3d editor closure (`semio-s-artifact-process-process3d-editor`) | bearbeiten only | Editor closure | ~90 E0425 (23:48) |
| Variant pairs sharing one app id: `generator` and `generation3d` both run `s.procedural.generation3d@1` | generator (pane), generation3d (standalone) | Same procedural artifact, two playground rows | Same blockers as generator |

## Pane-to-variant-to-app cross-check

| Pane | Brand id | Variant | Runtime | App id | Hub | Fixture | Smoke fixture matches |
|---|---|---|---|---|---|---|---|
| generator | entwerfen-mit-bestand-generator | generator | generation3d | s.procedural.generation3d@1 | demonstrator component | hexagonal-mushroom-column | yes |
| koordinator | entwerfen-mit-bestand-koordinator | koordinator | koordinator | s.cad.cad@1 | demonstrator component | demo | yes (text by equivalence) |
| aggregator | entwerfen-mit-bestand-aggregator | aggregator | aggregator | s.puzzle.puzzle3d@1 | demonstrator component | concrete-forest | yes after test edit |
| aussuchen | entwerfen-mit-bestand-aussuchen | aussuchen | aussuchen | s.sourcing.curation@1 | demonstrator component | demo | yes by id |
| bearbeiten | entwerfen-mit-bestand-bearbeiten | bearbeiten | bearbeiten | s.process.process3d@1 | demonstrator component | concrete-forest | yes after test edit |
| verfolgen | entwerfen-mit-bestand-verfolgen | verfolgen | verfolgen | s.gis.gismap@1 | demonstrator component | demo | yes by id |
| energie | entwerfen-mit-bestand-energie | energy | energy | s.energy.model@1 | energy component (standalone row) | bestest-600 | different path (Rust builder) |
| statik | entwerfen-mit-bestand-statik | fem3d | fem3d | s.fem.fem3d@1 | fem component (standalone row) | concrete-forest | yes after test edit |

## Not verified

- No native test, browser run or build was executed. All "status" statements are copied from the cited logs and result notes, with their timestamps.
- `prepare-<variant>-react-prototype` target definitions and the `@semio-tech/framework-os-dev` package were not found; only the references in `project.json` were confirmed.
- Cargo package names for `semio-s-artifact-fem-3d-engine`-level crates and the `artifact_app_laws` law crate were not resolved to manifest paths.
- `editor-gestures` (aggregator) and `statik_pane_smoke` run commands are from 1102 notes and are not checked against the current seq.sh.
