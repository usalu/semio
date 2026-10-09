# r11-w12-views execution report (WP-12 authored views with elevations)

Status: **WRITTEN, UNVERIFIED BY CARGO** (build impossible, see "Why unverified"). Everything that does not need `cargo` was run and is green.
`S` = `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any`, `E` = `S/🚪️io/📤️export/🎨️svg`, `T` = this ticket folder.

## Why unverified
`cargo check -p semio-s-artifact-bim-model --lib --tests` stops in the framework crates (owner's half-finished retirement refactor of commit 677): after r11-merge,
r11-store-a/-b/-t2 the lowest remaining failures are `semio-framework-plugin` (820 errors: `SnapshotRetirementStep` / `owned_retirement` / `close_step` signatures in
`acts/📖️playbook/…/🧬️generation`, `cts/🕸️dag/…/🧵️retained`, `🫧️transient/♻️retirement`, presence, interaction). The BIM crate is never reached; last log
`check13` (05:4x). Nothing of mine is in an error list (every error path is a framework file). No test of this report was run.
Open until the framework builds: `cargo test --lib` counts, wasm32-wasip2 check, `BIM_BLESS=1` blessings (below), harness `oracle quick` of the two cases.

## Done (by audit task)
1. **Warning `🗺️plan/🦀️.rs:138`**: `self::records` -> `records` (unnecessary qualification).
2. **Unused label `section_line`** removed from `✏️editor/🗣️terminology/🦀️.rs` (no other use in S).
3. **Example views**: `T/r4-x-examples-gen.ts` already calls `viewsFor`; run -> house 13, office 16 views; `bun T/r10-w12-views-examples.ts` -> demo 9 views
   (`🖼️assets/{🏡️house,🏢️office,🎬️demo}/📸️snapshot.json`). The `🗣️.dsl.semio` texts still lack the views: **bless with `BIM_BLESS=1 cargo test bless_the_`**.
   Test `the_example_views_are_the_ones_the_command_makes` in all three example test files (`S/📚️examples/*/🧪️tests/🧩️example/🦀️.rs`) calls the new
   `checks::assert_views_are_the_commands` (`S/📚️examples/🧰️checks/🦀️.rs`): it runs the real `createView` handler (plan per storey, ceiling plan per storey with
   ceilings, `elevations`, `section`, `perspective`) on the example without views and requires every produced view in the example (names ignored for section/camera),
   plus the quarter-turned second section, and that the example has no other view.
   NOTE: the checked-in house/office snapshots are older than the `Beam.axis` schema change (r12 wp19): `r4-x-examples-gen.ts` still writes `start/end` beams and now
   throws in `viewsFor` (I changed `r10-w12-views-examples.ts` and the Rust `create-view::extents` to `axis_ends(&beam.axis)`); r12/r13 must regenerate the examples
   with `axis` beams, the views are then re-derived by the same call. The views already present in the assets were produced before that change and do not depend on beams.
4. **SVG per view from `view-linework`** (no storey path left): `E/🦀️.rs` (`views_to_svg`, `export_svg` over `compute_view_linework`; root `class="sheet"`, `data-unit="mm"`),
   `E/📐️sheet` (slot per drawn view: plans highest level first, ceiling plans, sections, elevations; camera views get no slot; per-view scale), `E/✒️path` (`Frame.mm`,
   `mm_per_metre(scale)`, `SCALE` removed), `E/🖍️drawing` (`view_group`: `g.view.<kind>` id `view-<id>`, `data-view/kind/building/name/scale/storey`, aria-label, same layers),
   `E/🎚️style` (`view_class`, `.sheet` font), `E/📏️projection` (report per view: kind, scale, regions, lines, texts, styles, arcs, poche area, line length, notation).
   Unit tests rewritten/added in every `🧪️tests` of those modules (house now has 10 drawn views: sheet order, scale, group attributes, path counts, section cut vs
   elevation, a view at scale 1:50, empty model, projection per view and scale, vertical drawing group, `view_class`).
   Fixtures: the house of the export cases (`🧫️fixtures/🏗️ifc/🏠️house/📸️snapshot/🔣️.json`, +175 lines only) and the annotated room got authored views
   (`T/r11-w12-views-fixtures.ts`, idempotent, appends the `views` block textually). **To bless: `BIM_BLESS=1 cargo test the_committed_house_file_is_the_current_export`
   and `… the_committed_notated_file_is_the_current_export`, then `python S/🧪️tests/🎨️export-bim-1-svg/🐍️.py write S/🧫️fixtures/🚪️svg`.**
   Oracle `S/🧪️tests/🎨️export-bim-1-svg/🐍️.py` (lxml + shapely) migrated from storey to view groups (kind, scale per view, storey attribute, sampled poche per view);
   feature + Rust adapter texts updated (`{width, height, views}`).
5. **view-linework oracle (shapely) + feature**: new case `S/🧪️tests/🖼️infer-bim-1-views/{🥒️.feature,🐍️.py,🦀️.rs}` (scenario `view-metrics-room`, oracle
   `bim-1-shapely-geometry`, registered by extending its rationale in `S/🔮️oracles/🔣️.json`). Fixture `S/🧫️fixtures/💡️inferences/🖼️view-linework/🏠️room`
   (`T/r11-w12-views-oracle-fixture.ts`): mitred room + parapet + two rotated columns on two storeys, 8 vertical views + a plan and a camera that the table skips
   (plain, shallow depth, oblique elevation, section, oblique section with depth, cropped section, section hiding walls, phase filtered elevation).
   The oracle rebuilds footprints with shapely (`buffer` mitre/flat, `box` + `affinity.rotate/translate`), cuts with the plane `LineString`, clips to the view slab with a
   polygon intersection, takes the extent of each connected piece along the plane, `unary_union`s the projections, clips to the crop `box`; metamorphic audit: turning
   scene and planes by 0.7 rad and shifting leaves every measure unchanged. Measures: `Silhouette.union_area`, `SectionCut.area`, `Datum.length` (edges are hidden-line
   removal, pinned by the subject's unit tests; `VERTICAL_METRICS` and the table were reduced to the three measures). Subject side: new
   `S/🚪️io/📝️text/💡️inferences/🖼️view-linework/🦀️.rs` (`metrics_json`, moved out of the schema, same shape as upstream's `plan_linework`), mounted in
   `…/💡️inferences/🦀️.rs`, slug `view-metrics` in `encode_inference_projection_json` (the arm r11-merge asked for).
   Committed expectation `…/💡️inference/📐️view-metrics/🔣️.json` written by the oracle.
6. **Editor reachability of elevations/sections** (verified by reading the code): section window = `Section|Elevation` views (`vertical_views`, chrome select, `setView view`),
   plan window = plan/ceiling-plan views; edit range/crop/visibility/phase/scale/detail/plane via the view rows of the entity table (`VIEW_FIELDS`: `depth`, `crop`, `hidden`,
   `phase`, ...). The only gap was *creating* non-plan views: `createView` took a free-text kind. Added outliner add-rows (en+de) for ceiling plan, section, elevation, all four
   elevations, 3D view (`S/✏️editor/📌️panels/🌳️outliner/🦀️.rs`, labels `name_ceiling_plan/elevation/elevations` in the terminology; test
   `every_kind_of_view_can_be_added_from_the_outliner_in_both_languages`).
7. **Window tests migrated from `storey` to `view`** (coordinator request): plan window tests (`🗺️plan/🧪️tests`), chrome tests (measure ids `bim.measure.plan.view` /
   `bim.measure.section.view`, view selects list plans by level / sections before elevations), `🖱️canvas-pointer-down` test ctx. The plan config tests were already
   migrated by r11-merge. The viewer (`👁️view`) plan window keeps its own storey config on purpose (not a WP-12 surface).
8. **Generators**: `bun T/r3-f1-gen-mutation-facets.ts` (136 leaves), `r3-f1-gen-oracle.ts` (136 kinds, 955 scenarios), `r3-f1-gen-feature.ts` (136), `r3-f1-check-names.ts`
   -> 3 problems, none mine (`🪶️sqlite 🗄️schema` duplicate emoji, `__pycache__` in `🪜️infer-bim-1-levels-and-wall-heights`, `🔲️ceilings-meshes` duplicate emoji).

## Commands run and exact results (no cargo)
- `bun T/r4-x-examples-gen.ts` / `r10-w12-views-examples.ts` / `r11-w12-views-fixtures.ts` / `r11-w12-views-oracle-fixture.ts`: ok (house 13, office 16, demo 9, fixtures 11 + 2, oracle room 10 views);
  after the beam schema change the first one throws (see 3).
- `PYTHONDONTWRITEBYTECODE=1 .venv/Scripts/python.exe S/🧪️tests/🖼️infer-bim-1-views/🐍️.py write|check S/🧫️fixtures/💡️inferences/🖼️view-linework` -> `check: oracle agrees (shapely 2.1.2)`
  (hand-checked: south silhouette 30.1 = 6.3x3 + 4x2.8; section A cut 2.92 = 2 x 0.3x3 + 0.4x2.8; datum 3 levels x length).
- the three generators above, `r3-f1-check-names` as listed.
- `cargo check … --lib --tests` x13 through the gate: first 1 BIM error in `🧵️gestures/🧱️chain` (not mine, later fixed by peers), then framework errors only.

## Not done / hand-off
- Run cargo once the framework builds: `cargo test --lib` (areas: `export::svg::*`, `view_linework`, `examples::*::the_example_views_are_the_ones_the_command_makes`,
  `outliner`, `chrome`, `windows::plan`, `create_view`), wasm32-wasip2 check, the BIM_BLESS blessings (3 DSL texts, 2 svg files), `svg 🐍️.py write`, then harness
  `oracle quick --case 🖼️infer-bim-1-views` and `🎨️export-bim-1-svg`, `subject quick` for both.
- Possible first-run surprises, all in the first places to look: a one-ulp difference between the TS `viewsFor` and Rust `extents` (compare exact `View` equality in the example
  test), the silhouette of a view whose plane lies inside a solid (near clip) vs the oracle's clipped-prism model, `Column.rotation` unit (radians, used so).
- Notated SVG fixtures (`🚪️svg/🪧️notated`) belong to r11-w11; its room now has the plan views, bless as in that report.
