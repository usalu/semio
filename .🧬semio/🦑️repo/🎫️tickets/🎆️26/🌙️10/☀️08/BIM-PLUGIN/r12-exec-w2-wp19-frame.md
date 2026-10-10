# 🏗️ Exec report `w2-wp19-frame` — WP-19 beams, columns and curtain-wall depth

T = ticket folder `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️08/BIM-PLUGIN`, S = `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any`.
Design record: `T/r12-w2-wp19-design.md` (binding). Ruling applied: `r9-decisions.md` §2 (curtain panel overrides are a keyed collection, unique by `(curtain, u, v)`).

## Status in one paragraph

Everything the brief lists is written: schema, 13 mutation leaves, inference, editor, IO, examples, oracles and features. **Nothing of the BIM crate has been compiled or run**, because the framework crates below it
do not compile yet. Last gate run after `r11-exec-store.md` appeared: `semio-framework-os-flow` 208 errors and `semio-s-artifact-stdio-contract` 15 errors (215 in total, none in `🏙️bim`; earlier runs failed in
`semio-framework-plugin`). The coordinator asked to stop polling with cargo, so no further build was started. Every file I touched passes `rustc -Z unpretty=normal` (full parse, module loading included); the python oracles run and agree with the committed tables. The compile
fix-up, the `cargo test --lib` counts, the blessing of the new committed outputs and the wasm32-wasip2 check are the open items at the end.

## What was done

### Schema and mutations (generated, `bun T/r3-f1-gen-model.ts` + `T/r12-w2-wp19-leaves.ts`)
* `Beam.axis: Axis` (Line | Arc) replaces `start`/`end`; `Beam.end_top_offset: Option<f64>` (inclined beams); `Column.tilt: Option<Slope>`; `CurtainWall` is type-based (`curtain_wall_type`, optional `u_grid`/`v_grid` rules over the type's);
  new `CurtainWallType`, `CurtainPanelOverride`, `CurtainGrid { Spacing | Lines }`, `CurtainPanel { Glass | Solid | Door | Window | Empty }` and the collections `curtain_wall_types`, `curtain_panel_overrides`.
* New leaves (binary tags 19000..19009): `set-beam-axis`, `set-column-tilt`, `create/set/delete-curtain-wall-type`, `set-curtain-wall-type-of`, `set-curtain-wall-grid`, `create/set/delete-curtain-panel-override`;
  changed leaves: `set-beam`, `set-curtain-wall`, `create-beam`, `create-curtain-wall`, `create-column`, `split-beam` (splits an arc/inclined beam at an arc-length fraction), `delete-curtain-wall` (cascades its overrides),
  `delete-material/door-type/window-type` (refuse while a panel names them), copy/mirror/array/move kernel (`Placement::Beam { axis }`, `Placement::Column { tilt }`). Duplicate cell: `mutation.duplicate-id` with the existing override id as path.
* Fixtures: every leaf has the quintet per case, concrete inverses, sum-law tests. Extra case folders were renamed to unused emojis by `T/r12-w2-wp19-rename-cases.ts`; `bun r3-f1-gen-mutation-facets.ts`, `gen-oracle.ts`,
  `gen-feature.ts` run clean (173 kinds, 1327 scenarios); `check-names` reports only problems that are not mine (sqlite schema folder, wall-depth vs create-stair, psets measure).

### Inference
* `🪞️curtain-layout`: resolved grid edges (wall rule over type rule), per-cell panel, in-grid overrides, stray and repeated overrides, ignored lines; dependency hash covers wall, type and its overrides.
* `🧊️element-solids`: `🏛️columns` (tilted columns are sheared prisms: section stretched by `1/cos(a)`, top moved by `rise*tan(a)`), `➖️beams` (Line | Arc, inclined via `sweep_profile_ramped`, ends cut back to column faces: joins
  derived at mid-depth, bisection along the axis), `🪟️curtain-walls` (border vs interior mullions, glass / solid / empty / door / window panels, hosted openings cut panels, filler parts carry `layer = 1 + row-major cell`).
* Framework geometry: `sweep_profile_ramped` (+3 passing tests). Plan linework, bodies, spaces, quantities (panels by kind, mullions by section, column length `h/cos`, beam net volume after joins), diagnostics
  (`curtain-wall.override-out-of-grid`, `door-not-at-base`, `grid-line-outside`, `duplicate-override`, `reference.curtain-*`, en+de), schedules (curtain wall type name), model graph (honest dependency, gating).

### Editor
* Entity table `🧩️entities/🏗️frame`: fields `tilt`, beam `axis` / `end_top_offset`, curtain wall `curtain_wall_type` / `u_grid` / `v_grid`, kinds `curtain-wall-type` (library) and `curtain-panel-override` (under its wall in outliner and topology).
* Tools (`🧵️gestures/🏗️frame`, utilities, arm commands, hotkeys, labels en+de): `beam-arc` (Alt+B), `column-tilt` (Alt+C), `curtain-grid` (Shift+U, plan/world/section along the wall; Shift or command key removes the nearest line),
  `curtain-cell` (Alt+U, click opens a cell or gives it back to its type, Shift makes it a door). Command `placeGridColumns` (`🎮️commands/🏛️place-grid-columns`) emits ordinary `create-column` mutations at every free grid crossing.
* Chain tool: beam and curtain wall creation follow the new shapes; snap, reshape (split beam), create-view extents, diagnostics category label `curtain-wall`.

### IO
* IFC export: leaning column = extrusion of the stretched horizontal section along `(sin a cos d, sin a sin d, cos a)` (`Column` authoring row); arc / inclined / joined beam = faceted brep of the inferred solid with an `Axis`
  representation (`IfcTrimmedCurve` for arcs) and `Beam` authoring row; plain level beams keep the swept form; `NetVolume` added to column/beam quantities; curtain wall = `IfcCurtainWall` aggregating `IfcMember`, one `IfcPlate` per
  panel material, one `IfcDoor` / `IfcWindow` per filled cell, authoring rows `CurtainWall`, `CurtainWallTypeId`, `CurtainWallType`, `CurtainPanelOverrides`.
* IFC import: restores columns, beams and curtain walls (type reused when equal, overrides keyed as authored) from those rows; parts are marked imported.
* Not touched on purpose (listed under "Exclusions"): sqlite lane, snapshot grammar/proto files.

### Examples
* `T/r12-w2-wp19-examples.ts` (`frameFor`, called by `T/r4-x-examples-gen.ts`): office gets type `cwt-facade`, a 2.4 m door row on `cu-0-south` with a service door, an open vent, a concrete spandrel, a ribbon window panel on
  `cu-1-south`, explicit bays on `cu-2-north`, arc beam `bm-1-arc-E1-F2` (joined to E1 and F2), inclined beam `bm-3-incline-E3-F4`, leaning column `c-1-lean`. Office tests updated (counts 97 columns, 156 beams) and a new test checks them.
  `📚️examples/🧰️checks` replays the new kinds and checks their references.

### Oracles and features
* shapely (`🧪️tests/📦️infer-bim-1-solids-rest/🐍️.py`): leaning columns (shear), beams of any axis with incline and joins; new fixture case `📐️frame-tilt-joins` plus extended `🏛️columns-profiles`, `➖️beams-profiles` (`T/r12-w2-wp19-fixtures.ts`).
* ifcopenshell wall-solids oracle (`🔳️infer-bim-1-wall-solids`): type-based curtain closed form incl. overrides; new case `🪟️curtain-overrides`. Plan/diagnostics oracle (`🗺️…`), quantities oracle and schedules oracle follow the new shapes.
* three.js (`🎲️infer-bim-1-solids-three`): cases `frame-tilt-joins`, `curtain-overrides` added to the scenario. ifcopenshell IFC oracle (`🏗️export-bim-1-ifc`): new case `🏗️frame` / scenario `export-ifc-frame`
  (extrusion direction, trimmed-circle axis, parts of a curtain wall, authored records).
* Python results run now: wall-solids `check`/`write`, solids-rest `check`/`write`, plan-and-diagnostics `check`, quantities `check` — all "oracle agrees".

## Exact commands and results

| Command | Result |
| --- | --- |
| `cargo test --manifest-path 🧰️framework/Cargo.toml -p semio-framework-geometry sweep_profile_ramped` | 3 passed |
| `bun T/r3-f1-gen-mutation-facets.ts`, `gen-oracle.ts`, `gen-feature.ts` | 173 leaves / 1327 scenarios, clean |
| `bun T/r3-f1-check-names.ts` | 6 problems, none in WP-19 paths |
| `bun T/r4-x-examples-gen.ts` | house + office written (office: 97 columns, 156 beams, 8 curtain walls, 4 overrides) |
| `python -X utf8 …/🔳️infer-bim-1-wall-solids/🐍️.py check|write <fixtures>` | oracle agrees (5 element-solids cases) |
| `python -X utf8 …/📦️infer-bim-1-solids-rest/🐍️.py check|write <fixtures>` | ok (columns 9, beams 13, frame 12 elements) |
| `python -X utf8 …/🗺️infer-bim-1-plan-and-diagnostics/🐍️.py check` | oracle agrees |
| `python -X utf8 …/🧮️infer-bim-1-quantities/🐍️.py check` | oracle agrees |
| `rustc -Z unpretty=normal` over all 161 changed `.rs` files of `✏️s/🔌️plugins/🏙️bim` | all parse |
| `cargo check --manifest-path …/🏢️model/📦️packages/🦀️rust/Cargo.toml --lib` | **blocked** by framework: os-flow 208, stdio-contract 15 errors, 0 in bim |

## Open items (in order)

1. When the framework compiles: `cargo check --lib`, fix the compile errors in WP-19 files (nothing of the editor, IO or inference I wrote was ever compiled), then `cargo test --lib` with exact counts.
2. Bless the committed outputs with `BIM_BLESS=1 cargo test --lib` (gate: `T/🚦️gate.sh`): the meshes of the element-solids cases (`📐️frame-tilt-joins`, `🪟️curtain-overrides`, re-blessed curtain-grid), the IFC files and snapshot of
   `🧫️fixtures/🏗️ifc/🏗️frame`, the committed `house.ifc` and other IFC/SVG/glTF files that carry the new curtain authoring rows or beam `NetVolume`, the office/house DSL texts (`bless_the_*`), then `python 🐍️.py write` of the
   IFC export oracle for the new `🏗️frame` case so `🔬️measure/🔣️.json` exists.
3. wasm32-wasip2 check of the crate, the three.js / ifcopenshell scenarios through the host.
4. Delete `T/🗑️generated/w2-wp19-frame/` (logs).

## Exclusions and honest limits

* **sqlite snapshot lane and the snapshot grammar files (`ebnf`, `proto`, `g4`, …) of `🚪️io`**: not updated. They were not updated by any W1 package either (no ceilings, ramps, zones), the Rust sqlite module is not mounted in the crate
  and `restore` builds `ModelSnapshot` without the newer collections; a regeneration of that lane from the schema is a separate package.
* **Property inheritance by curtain wall type** (`effective-properties`): `TemplateTarget` has no `CurtainWallType` variant, adding one is a psets-package change; curtain walls keep their own properties.
* **Door/window panels in the shapely closed form**: panels of kind Door/Window have no closed form in the oracle (the wall is left out of the expected table); they are asserted in unit tests of `🪟️curtain-walls`
  and in the IFC oracle (parts count) instead.
* **Foreign curtain walls on IFC import** (no authoring rows) are reported by the unsupported-class note, as before; the IFC4 library is untouched.
* Hosted openings are not matched to door panels in the IFC export (a hosted opening and a door panel are separate products).
* The framework `parse` of the environment: `cargo` in this environment needs `--manifest-path` of the crate (`-p` from the repo root finds no package).

## Hand-off (coordinator, after the report above)

Agent `r13-integrate` owns making the BIM crate compile (it may touch any BIM file) and writes `T/r13-compiles.flag` when it does. I stopped all builds and polling with cargo; I waited ~1 h on the flag without seeing it.
When the flag exists, the verification of this package is: `cargo check --lib`, fix failures in WP-19 files, `cargo test --lib` (counts), then the blesses and oracle writes listed under "Open items" 2 and 3. Nothing else of WP-19 is pending.
