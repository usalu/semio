STATUS: UNVERIFIED for all 8 panes. No native pane smoke test has a passing run. Last verdicts: energie 0 passed / 2 failed (23:00, fixed in test, not rerun); the other 7 died at compile or cargo deadline; aussuchen and koordinator were never run through their real target. Read-only audit, no cargo/repo/bun run.

# 1102 audit: native end-to-end smoke tests for the 8 demonstrator panes (aud-smoke)

Ticket folder C = `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS`.
Logs: `C/🗑️generated/1101/rtpanes-<pane>.{txt,rc}` (1101 runs), `C/🗑️generated/1102/rtpanes-energie.txt` (02:27, queued on cargo capacity, no verdict), `C/🗑️generated/1102/etest-11.txt` (02:27, still compiling, no `test result` line).
Runner: `C/1101-tools/rtpanes/run.sh` (one `repo cargo ... test --manifest-path <ws>/Cargo.toml --no-fail-fast <args>`, writes `rtpanes-<name>.txt` and `.rc`) and `C/1101-tools/rtpanes/seq.sh` (one pane after another, done marker `rtpanes-seq-<tag>.rc`).

## 1. What the shared law proves

`artifact_app_laws::assert_pane_smoke` (`🧰️framework/.../🧪️testing/🧪️artifact-app-laws/🪧️pane-smoke-law/🦀️.rs`): loads an example DSL text through the document archive door, applies one typed command, asserts the probe changed, undo restores, redo reapplies, a second undo restores, then `close_registered_fixture_app`.
`assert_pane_smoke_by_example`: same, but the example comes from the app's own load-example command (published `LoadDocument`). It does undo, then close. It has NO redo.
Neither law renders a pane body, checks a locale, or runs declared-verb conformance. Only the energie test renders bodies.

## 2. Per pane

Fixture column: "page" = `♻️mit-bestand/🧺️demonstrator/🪧️brand.ts` `defaults.exampleId` for the pane (the page's own load). Catalog app per pane from `✏️s/🧑‍💻dev/🧩️catalog/🤖️generated/🚀️playgrounds.json`.

| Pane | Catalog app | Run target (seq.sh) | Smoke file | Page fixture vs smoke fixture | BOOT | CONTENT | ACTION | UNDO | REDO | Last verdict |
|---|---|---|---|---|---|---|---|---|---|---|
| generator | s.procedural.generation3d@1 | `-p semio-s-composition-laws --test generation3d-app-laws-editor-unit -- generator_pane` (Cargo.toml:126) | `✏️s/🧑‍💻dev/🧩️composition/🧪️tests/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🪧️pane-smoke/🦀️.rs`, `include!`d from `🧪️tests/🧊️generation3d/🧮️editor-unit-laws/📚️editor-laws/🦀️.rs:7` | `hexagonal-mushroom-column` vs same id via `SetActiveExample`: MATCH | yes (widgets > 0) | no | AddGeneration, +1 generation | yes | NO | rc=1, 21:19: E0117 (orphan impl in value derive expansion, foreign). Never reached a test result. |
| koordinator | s.cad.cad@1 | `-p semio-s-artifact-cad-cad --test cad_editor_unit -- koordinator_pane` (cad Cargo.toml:18-20) | `📐️cad/.../✏️editor/🧪️tests/🔬️unit/🪧️pane-smoke/🦀️.rs` (mod at `🔬️unit/🦀️.rs:30-31`) | page `demo` vs cad `examples::demo::PRIMARY_TEXT` (same bytes, loaded as text, not via the example command): MATCH | yes (node count) | no | AddNode kind solid, +1 | yes | yes | rc=1, 18:33: killed by the 3595204 ms `cargo:test` deadline during cold build, no rustc error. Cad editor lib had earlier compile errors (1030: missing dsl-record-derive, CadDispatchCtx). |
| aggregator | s.puzzle.puzzle3d@1 | `-p semio-s-artifact-puzzle-3d-editor --test editor-gestures -- aggregator_pane` | `puzzle/3d/.../✏️editor/🧪️tests/🔬️pane-smoke/🦀️.rs` (mod at `🔬️gestures/🦀️.rs:11-12`) | page `concrete-forest` vs smoke `nakagin-capsule-tower` (`PUZZLE3D_EXAMPLE_NAKAGIN`): MISMATCH | yes (object count changes, origin read) | no | `worldRelocate` by string dispatch | yes | yes | rc=1, 16:21: E0616 (`work` private in `evaluation`) and E0407 (`take_work` not in trait), both in os-flow-extensions (foreign). Puzzle smoke itself never compiled in a run. |
| energie | s.energy.model@1 | `-p semio-s-artifact-energy-model-editor-mounted-tests --lib` (no filter) | `🔋️energy/.../✏️editor/🧪️tests/🔬️mounted-bestest/🦀️.rs` (lib path) | page `bestest-600` vs test loads `semio_s_plugin_energy_engine_bestest::model("600")` built in Rust (the example id itself is `bestest-600`, `📚️examples/🏛️bestest-600`): EQUIVALENT case, DIFFERENT path (bypasses the example command) | yes | yes (`energy.model.artifact`, `energy.model.inspection` non-empty, en only) | rename via SET_NODE_ACTION | yes | yes | rc=1, 23:00: `test result: FAILED. 0 passed; 2 failed`. Both tests panic `bind-weather-file input schema refused: RefUnresolved /link` (no framework schema published). Fix added in the test `manifest()` (publish first), not rerun. 1102 rerun queued, no verdict. |
| aussuchen | s.sourcing.curation@1 | seq.sh runs `-p semio-s-artifact-sourcing-curation --lib -- aussuchen_pane`: WRONG TARGET (see gap G1) | `🪵️sourcing/.../✏️editor/🧪️tests/🪧️pane-smoke/🦀️.rs` (declared as `[[test]] aussuchen_pane_smoke`, Cargo.toml:76-78) | page `demo` vs `DEMO_STOCK_TEXT` = sourcing `examples::demo::PRIMARY_TEXT`: MATCH by id (texts not compared) | yes (stock_extra non-empty) | no | CurationAdd | yes | yes | no rc. Last log 02:15 SIGTERM (coordinator cancel). Before that rc=1, 13:32: `framework-test-original-snapshot-owners` errors (RetainedWorkOutcome), fixed in 1101, not rerun. |
| bearbeiten | s.process.process3d@1 | `-p semio-s-artifact-process-process3d-editor --test process3d_editor_unit -- bearbeiten_pane` (Cargo.toml:18-20) | `process3d/.../✏️editor/🧪️tests/🔬️unit/🪧️pane-smoke/🦀️.rs` (mod at `🔬️unit/🦀️.rs:89-90`) | page `concrete-forest` vs smoke `demo` (`PRIMARY_TEXT`): MISMATCH | yes (`stock_label` "Timber Beam", steps change) | no | AddStep | yes | yes | rc=1, 23:48: about 90 E0425 (`PROCESS_3D_PLAY_APP_ID`, `Process3dResumableCommandWork` not found) in the process3d editor closure. Not a smoke-file error. |
| verfolgen | s.gis.gismap@1 | `-p semio-s-artifact-gis-gismap-gis2d-unit-tests --lib -- verfolgen_pane` | `gismap/.../✏️editor/🧪️tests/🪧️pane-smoke/🦀️.rs` (mod at `🔬️laws/🦀️.rs:20-21`) | page `demo` vs gis demo asset `DEMO_DSL` (`🖼️assets/🎬️demo`): MATCH by id (texts not compared) | yes (positions non-empty) | no | AddFeature "Depot" | yes | yes | rc=1, 01:07: E0599 (`expect` on `RetainedWorkOutcome`, peer-API mismatch). 1030 also showed missing `SnapshotRetirementStep` in the same lib. |
| statik | s.fem.fem3d@1 | `-p semio-s-artifact-fem-3d-editor --test editor_app -- statik_pane` (reaches the smoke via `🔬️app/🦀️.rs:174-175` `editor_unit` → `🔬️unit/🦀️.rs:601-602`) | `fem/3d/.../✏️editor/🧪️tests/🪧️pane-smoke/🦀️.rs` | page `concrete-forest` vs smoke `examples::demo::ID` = `"demo"`: MISMATCH | yes (solids non-empty) | no | AddNode | yes | NO (by_example law) | rc=1, 12:49: 6× E0432 in `semio-s-artifact-stdio-pdf-schema-4` mirror (`modes::edit::windows`). Chain: fem-3d-io → stdio-semio-mesh-export → stdio-semio-drawing-export → stdio-pdf. Never reached the fem test. |

Notes on the pane list. The user brief names aggregator=flow. The catalog and the demonstrator brand both say aggregator = `s.puzzle.puzzle3d@1` ("Aggregator (puzzle3d)" in brand.ts:353). This audit treats aggregator as puzzle3d. Please confirm.

Demonstrator fixture `♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/🔣.json` lists 8 rows: generator/generation3d, koordinator, aggregator, aussuchen, bearbeiten, verfolgen, energy, fem3d. Those match the 8 smoke targets above. No native test reads this file.

## 3. Testkit conformance

- `artifact_app_laws::new_app_with_registry` and `new_app_with_registry_and_members` (`🏗️registered-app-construction-and-typed-operation-settlement/🦀️.rs:39-50`) build the app from `manifest()` and never call `PluginRuntimeRegistry::publish_framework_schema_documents()`. That call exists only in production ingress (`🧵️instance/📥️staging/🚪️ingress/🦀️.rs:66`) and in registration (`📜️declaration/📇️registration/🦀️.rs:264,278`). Energie is the only pane test that publishes first. Probable (unverified) consequence: every pane whose schemas use `/link` (aussuchen, koordinator, bearbeiten, verfolgen, aggregator, statik, generator) hits the same RefUnresolved panic.
- No pane test calls declared-verb conformance (`assert_declared_verbs_honour_their_declarations`, `probe_declared_verbs`). These exist in `🧪️testing/🧪️artifact-app-laws/🔎️declared-verb-probing`.
- The testkit context helpers (`app()`, `new_app()`, `fem3d_app()`, `context::app()`) publish no schema documents (grep: 0 hits in all 7 pane test trees).

## 4. Gap list (exact)

G1. seq.sh aussuchen uses `--lib -- aussuchen_pane`. The smoke is the `[[test]] aussuchen_pane_smoke` target (sourcing curation Cargo.toml:76-78). With `--lib` it matches 0 tests. Fix in `C/1101-tools/rtpanes/seq.sh`: `aussuchen) run.sh aussuchen "...curation" -p semio-s-artifact-sourcing-curation --test aussuchen_pane_smoke -- aussuchen_pane`.

G2. Missing REDO for generator and statik (by_example law has none). Minimal addition: after the law returns, call `settle_history_verb(app, "redo", receiver, policy, identity, cx)` and assert `probe == applied`. Best place: `assert_pane_smoke_by_example` in the law file (framework, freeze-restricted; needs a patch in `C/1102-tools/<lane>/patches/`). Check `settle_history_verb` is re-exported.

G3. Fixture mismatch with the page for aggregator, bearbeiten, statik. Pick the page default in each case:
- statik: change `example_id` from `semio_s_artifact_fem_3d_io::examples::demo::ID` to `"concrete-forest"`; the fem example `concrete-forest` exists (`🌲️concrete-forest/🦀️.rs:16`). Test file: the statik pane-smoke file above. Builder: `Fem3dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "concrete-forest".into() })`. Keep `assert_pane_smoke_by_example`. Add a second test for `demo` if both are wanted.
- bearbeiten: use `semio_s_artifact_process_process3d_examples::concrete_forest::PRIMARY_TEXT` (defined at `examples/🌲️concrete-forest/🦀️.rs:12`; module path to confirm) instead of `examples::demo`.
- aggregator: change the test to load `concrete-forest` (the puzzle example module `🌲️concrete-forest/🦀️.rs:9` `ID`), via `dispatch(&mut app, "setActiveExample", json!({"exampleId": "concrete-forest"}))`, and assert against that example's object count. Keep nakagin as a second test if wanted.

G4. Energie does not use the page's load path. It builds `model("600")` in Rust. Minimal extra test: `dispatch` the example command `bestest-600` through `assert_pane_smoke_by_example` (same as statik/generator), so the page's example catalogue is exercised.

G5. No pane smoke renders its pane body (only energie, en only). Minimal addition per pane: `laws::render_registered_fixture_app(&mut app.0, "<body key>", &ViewModel::new(Locale::En, Terminology::Native), app.1.cx()).await` and assert non-empty, then repeat with `Locale::De`. The body keys are not listed in this audit; read each pane's view manifest before writing them.

G6. Schema publication gap in the shared helpers. Minimal fix: have `new_app_with_registry_and_members` (and `new_app_with_registry`) call `PluginRuntimeRegistry::publish_framework_schema_documents()` before `VcsArtifactApp::with_registry`. Framework file, freeze-restricted; patch in `C/1102-tools/<lane>/patches/`. Until then, every pane test except energie will hit RefUnresolved on first boot (probable, not run).

G7. No declared-verb conformance in any pane smoke. Minimal add: one call to `assert_declared_verbs_honour_their_declarations` per app test (signature in `🔎️declared-verb-probing/🦀️.rs`, not read in full).

G8. The process3d standalone `🧪️tests/🪧️pane-smoke-root/🦀️.rs` is not declared in `Cargo.toml` and is orphaned. Either declare it as a `[[test]]` or delete it (deletion needs a patch: the ticket rule forbids deletes of input files).

G9. Compile blockers are in other lanes, not in the smoke files: flow-extensions (aggregator, if puzzle), value derive E0117 (generator), process3d editor E0425 (bearbeiten), stdio-pdf mirror (statik), `RetainedWorkOutcome` API (verfolgen, aussuchen). Each pane is blocked until its lane is green.

G10. Compile-to-run timeline: koordinator (18:33) and verfolgen (15:02) hit the 59-min `cargo:test` deadline on cold builds. No pane has been run on a warm cache.

G11. No de-locale native test anywhere; run.sh forces `SEMIO_LANGUAGE=en`.

G12. Native smoke tests can prove the store/undo path, not DOM rendering. The browser proof is only in `C/1101-tools/e2eready/run.sh` (cases list in `♻️mit-bestand/🧺️demonstrator/🔨️modules/🧪️e2e/📁️artifacts/🧫️cases.json`). That run has no verdict yet.

## 5. Minimal new test file for pane fixture parity (optional)

Path: `♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/🧪️tests/🟦️.ts` already exists (reads `DEMONSTRATOR_RUNTIME_TARGETS`). A small parity check there could assert, for each `🔣.json` pane, that the page default `exampleId` appears in the matching native smoke file. This would have caught G3 at once.

## 6. Verdict summary

- Tests exist for 8/8 panes, all in a `🧪️tests/` tree.
- BOOT: 8/8 (law or own assert), but no run has passed.
- CONTENT (pane body renders): 1/8 (energie, en only), and that test has not passed.
- ACTION: 8/8 written. UNDO: 8/8 written. REDO: 6/8 (generator and statik missing).
- Fixture equals the page default: 4/8 by id (koordinator, aussuchen, verfolgen, generator), 1/8 equivalent by a different path (energie), 3/8 mismatch (aggregator, bearbeiten, statik).
- Native pass: 0/8. UNVERIFIED for all eight.
