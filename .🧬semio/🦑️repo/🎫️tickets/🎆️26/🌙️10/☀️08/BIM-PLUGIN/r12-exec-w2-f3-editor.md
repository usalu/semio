# r12-exec-w2-f3-editor: components and MEP in the BIM editor

Label `w2-f3-editor`. S = `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any`, E = `S/✏️editor`, T = this ticket folder.

## Status (read first)

* `cargo check --lib --tests` through the gate (label `w2-f3-editor`, 16:10): **Finished, 0 errors**, with all files below wired.
* `cargo test --lib` of my filters: **one run reached the tests** (17 tests matched an early, partly wrong filter): 6 passed (`gesture_keys` x3, `panels::families` x3) and 11 failed for two known reasons that are
  test-data problems, not logic (below). Every later attempt (t5 to t7, r1 to r8, until 19:05) did not reach the tests: `semio-framework` / `os-kernel` / `plugin` / `framework-2d` were being rewritten by other agents
  (serde derive errors, unresolved `RetirementCursor`, a link error on a locked test exe, a peer's `[DEBUG]` call in `gestures/🧪️tests/🧷️app`). **No test of the place tool, the route tool, the entity rows or the override rows has
  been run.** I paused crate edits as the coordinator asked (waiting for `T/r13-blessed.flag`, which did not appear), so the two known fixes below are NOT applied.
* Known failures of the one run: (1) `entities::components::tests::placed()` / `furnished()` take the demo room (`gestures::tests::fixture::room()`) whose walls are empty at the moment because `default_snapshot()`
  = `parse_dsl(BIM_EXAMPLE_TEXT).unwrap_or_default()` fails until the integrator re-blesses the example text (10 tests panic "the room has walls"); once blessed they should pass, else build the wall in the helper.
  (2) The table family's solids span 1.61 x 0.81 m (legs), not 1.6 x 0.8: `panels::families` test `the_size_is_the_union...` and `gestures::place` test `moving_shows_the_ghost...` must expect `1.61`/`0.81`.
* Test filter for the next run: `-- ::place::tests ::route::tests components::tests mep::tests overrides::tests set_override browse_families gesture_keys panels::families`.

## Delivered (files)

Entities (rows of the entity table, create-entity container rules, outliner topology, inspector fields, inferred read-only rows)
* `E/🧩️entities/🪑️components/🦀️.rs` (+ `🧪️tests/🔬️unit`, 13 tests; exports the `furnished()` / `placed()` fixtures): kinds `component` (storey, family picker grouped by category, position, elevation, rotation in degrees, mirrored, host wall picker,
  system picker; host and system settable AND clearable through `Assigned`; inferred category, override count, volume, issue count) and `component-override` (read-only component and parameter, formula writes
  `set-component-override`, delete = `remove-component-override` by key; shown under its component); `placeable()` (non-profile families by category, name, id), `matching()` (search in both languages), pickers, `SYSTEMS`.
* `E/🧩️entities/🌀️mep/🦀️.rs` (+ tests, 5): kind `mep-element` (system, section as `duct 0.3 x 0.2` / `pipe 0.1` / `tray 0.3 x 0.06` in metres, path as `x, y, z; ...`; inferred section, length, volume, issues); `create_mep`.
* `E/🧩️entities/🦀️.rs` (mods, 3 table rows, `storey_of` arm), `E/🕹️interaction/🦀️.rs` (overrides under their component, kept out of the storey loop). Delete/copy/move/rename of the new kinds work through the entity table and the leaves' cascade/placement.
Properties: `E/📌️panels/🔍️properties/🎚️overrides/🦀️.rs` (+ tests, 6): one row per family parameter of the selected component (value evaluated under the override with `parameters::resolve`, issue text from `issues::message` in the viewer's locale, input with
the family formula as placeholder, reset row only while overridden); wired as the section `section_overrides` of the properties panel.
Family library browser: `E/📌️panels/🪑️families/🦀️.rs` (+ tests, 5): panel tab "Family browser" (search box, per-category windowed sections, details of the selected family: category, parameters, solids, size from the inferred meshes, volume,
issues, and the place / open rows). Selecting a family selects it in the `library` domain, so the existing family preview window (`bim-edit-family-view`) previews it in 3D; there is no 3D surface inside the panel itself.
Commands (16, rows in `bim_command_table!`, bridge arms, action args, 6 fault notices, palette entries):
* `E/🎮️commands/🪑️browse-families` (`placeComponent`, `openFamily` with `Effect::OpenWindow`, `searchFamilies`), `E/🎮️commands/🎚️set-override` (`setOverride`: canonical formula, empty or the family's own formula removes the override, parse errors refuse),
  `E/🎮️commands/🔑️gesture-keys` (9 keys), `armComponent` / `armRoute` in `🛠️arm-utility`, utilities `component` and `route` in `🪛️utilities` (group `components`).
* Hotkeys: Shift+C component tool, Shift+M route tool; in a gesture: Alt+R / Alt+Shift+R turn +-15 deg, Ctrl+Alt+R quarter turn, Alt+M mirror, Tab / Shift+Tab next / previous family (route: section kind), Alt+PageUp / PageDown elevation +-0.1 m,
  Alt+T system (component: none, the nine systems; route: the nine). R / M are taken (roof, measure), so the contract's "R / M" became Alt+R / Alt+M. Tab follows the precedent of the puzzle plugin; whether the shell leaves Tab to focus traversal
  while no gesture is armed is a shell question I could not verify.
Gestures
* `E/🧵️gestures/🪑️place` (+ tests, 14): ghost of the family (footprint of the inferred visible solids, front tick, connector cross for a terminal, label), free-standing snap, wall-mounted suggestion for Plumbing / Lighting / Electrical / Casework
  (host set, position = pointer, Ctrl = free), repeated placement, Escape leaves, typed options `z`, `rot`, `mirror`, `sys`, `fam`. `fit()` and `Frame` are the plan law of the host fit (origin on the face of the pointer's side, local +y away from the wall).
  I did not wait for the graph API: `fit` duplicates the inference's law and is audited by the oracle below; switch to a graph helper if `T/r12-w2-f3-api.md` publishes one.
* `E/🧵️gestures/🌀️route` (+ tests, 9): clicks add vertices at the current elevation (a click at the same plan point with another elevation is a riser), Enter / double click writes one `create-mep-element`, Backspace (consumed through `deleteSelection`
  only while a route has vertices) takes a vertex back, Escape drops the route then leaves the tool, system / section kind / elevation keys, typed `z`, `duct`, `tray`, `pipe`, `sys` with units, defaults of the section from the system.
* Shared edits: `🧭️session` (`GestureKey`, `Tool::key`, `Tool::line` with default bodies), `🧵️gestures/🦀️.rs` (`run_key`, `settle_taken`, `ToolSession::key`, typed-option hook, registry arms), `🗑️delete-selection` (Backspace hook), `🧵️gestures/🧪️tests` (`Rig::key`).
Labels: `T/r12-w2-f3-editor-labels.ts` (anchor-insert, idempotent; 80 rows en then de). Python helper `T/r12-w2-f3-editor-edit.py` (anchored edits).
Oracle and feature: `S/🧪️tests/🛠️gestures-bim-1/🐍️.py` extended (shapely/GEOS projection and numpy for `component_fits`, numpy rotation matrices for `component_frames`, numpy segment norms for `routes`, numpy modulo for `turns`);
fixture `S/🧫️fixtures/🛠️gestures/🔣️.json` written by the oracle; `python 🐍️.py check` = `ok: 60 cases agree with numpy 2.5.0 and shapely`. Six scenarios appended to `🥒️.feature` (`@id-component-fit`, `-frame`, `-place`, `-host`,
`@id-route-length`, `-keys`). The Rust side replays the oracle cases in the place and route test files (`the_wall_fits_the_oracle_wrote...`, `the_frames...`, `the_turn_runs...`, `the_routes_the_oracle_measured...`): written, not run.

## Commands that ran
| command | result |
|---|---|
| `bun r12-w2-f3-editor-labels.ts` | 80 rows added |
| `.venv/Scripts/python.exe -I 🐍️.py write` then `check` of the gestures oracle | wrote; `ok: 60 cases agree` |
| `gate.sh w2-f3-editor -- cargo check -p semio-s-artifact-bim-model --lib --tests` (after the `Point2` fix) | Finished, no error |
| `gate.sh ... cargo test --lib -- <9 filters>` (t4) | 6 passed, 11 failed (causes above) |
| later `cargo test` attempts | not reached the tests (framework mid-refactor / link lock) |

## Open items
1. Apply the two test fixes (1.61 / 0.81; room walls) and run the filter above; then the `cargo test --lib` counts, `--target wasm32-wasip2` check, `bun T/r3-f1-check-names.ts` (new folders are unique among siblings by construction, not run).
2. Not done: MEP path handles in the select tool (the path is editable as a text field and per-vertex through `set-mep-element`; no drag handles); view-toggle categories `Components` / `Mep` (the view-category vocabulary lives in the views package, not touched);
   `🔭️create-view` `extents` ignores component and MEP points (must change together with `T/r10-w12-views-examples.ts`; flagged in `📓️coordination.md`); no mounted-app (`bim_app()`) test of the place gesture through the retained route.
3. `keep the place tool aligned with the inference`: if the graph agent changes the host-fit law, `fit()` and the oracle's `component_fit` change together.
4. `.vscode/launch.json` registers dev servers only, no windows or commands, so nothing to add.


## Update after `r13-blessed.flag` (22:45)

* Applied: table size expectations 1.61 / 0.81 (place ghost test, families panel test). The room fixture was left as is: `room()` returns the demo room again only once the example text parses, which is the integrator's bless.
* Done: MEP path handles in the select tool (`E/🧵️gestures/🎯️select/🦀️.rs`: `route_handles`, `dragged_route`, `Mode::Route`, marks, ghost; two tests added to its test file). Dragging a vertex of the one selected MEP element writes one
  `set-mep-element` with the new path (x, y moved, the vertex keeps its z; refused when it would stack two neighbours). Written, not compiled, not run (see below).
* Done (by the assets agent, verified by reading): view extents include component positions and MEP path points in `E/🎮️commands/🔭️create-view/🦀️.rs` AND `T/r10-w12-views-examples.ts`, both sides identical.
* Not done, justified: view-toggle categories. `ViewCategory` (schema `💠️values`) has no `Ceilings`/`Ramps` either; adding `Components`/`Mep` changes the schema, the generators, the plan/section projections and the view fixtures of the views package, which is not cheap.
* `bun T/r3-f1-check-names.ts`: 13 problems, all in other packages (sqlite schema, ifc psets fixtures, diagnostics export fixture, sheets/energy fixtures, set-type-thermal-data tests); **0 in `✏️editor`**, so my new folders are unique among their siblings.
* NOT verified: `cargo check --lib --tests` after the select edit, the editor test filters, the gesture feature replays and the wasm32-wasip2 check. A retry loop (`gate.sh w2-f3-editor -- cargo test ... --lib -- <filters>`, 10 attempts between 20:02 and 22:36)
  never reached the tests: `semio-framework`, `semio-framework-pack`, `plugin` (close ladder, `ContextOwnedFields.tool_run`), `framework-tool-run` (missing `👥️entities/🧾️provenance/🦀️.rs`), a stdio zip crate (`apply_mutation`) and a locked test exe were broken by the
  framework refactor of other agents at each attempt. The last green result of mine is `cargo check --lib --tests` Finished at 16:10, before the select edit and the two test-expectation edits.
* Next run (when the framework builds): `cargo check --lib --tests` (select edit), then `cargo test --lib -- ::place::tests ::route::tests components::tests mep::tests overrides::tests set_override browse_families gesture_keys panels::families select::tests`,
  then `cargo check --lib --target wasm32-wasip2`.
