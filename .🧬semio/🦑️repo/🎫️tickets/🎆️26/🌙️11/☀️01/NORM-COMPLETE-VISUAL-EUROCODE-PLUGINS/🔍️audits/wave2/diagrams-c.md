# DIAGRAMS-C (wave 2): gaps 14, 15, 16 of editor-ux-audit

Nothing compiled (no cargo/rustc/`repo run`). `- ready for rebuild: diagrams-c` appended to `build-en1990/latest.md` and `build-en1992/latest.md`. No schema JSON touched, so no twin regeneration. freeze.txt absent at every batch. Gate `handle-paths-check.py en1990 en1992`: en1990 0 uncovered; en1992 3 uncovered, all `studRings` (v14 work, not mine); my new paths `sectionPoints[].y|z`, `seismics[].importanceClass`, `testSeries[].mean|stdDev` are covered. Showcase `--check` has no en1990/en1992 builder (only en1991/93/94/98/99): n/a. Fixture checker: no fixture or schema changed: n/a.

Root en1990: `⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene`; en1992: `🏛️en1992/…/🎬️scene`.

## Verification of the audit claims
- en1990 factorOverrides diagram: already exists (`factor-overrides`, annex vs override bars). trafficGroups diagram: already exists (`traffic-groups`). Not redone.
- bridgeSls: only the rail frequency/resonance diagrams (RailBridge) and the member-based `sls-limits` existed: gap real. effects, fatigue spectrum histogram: gap real. seismics a_Ek grip: already existed (`magnitude`); importance class: gap real. testSeries grips: gap real.
- en1992 cases: diagrams already exist (`diagrams()` extends with `case_diagram`). Colour members by grade: the `grade-fck`/`grade-fyk` field layer on every member already exists (hidden-by-default layer, legends); not changed.

## en1990 changes
- New `📊️effects-and-spectra/🦀️.rs` mounted from `📈️diagrams/🦀️.rs` and called in `diagrams()`:
  - `bridge-sls:<id>`: deflection, deck twist, deck acceleration as share of their limit (hatched when >1, dashed limit u=1); the deflection diamond binds `bridgeSls[id].bridgeDeflection` (y axis, k = limit). Table twin.
  - `effects:<member>`: per action effect the min..max range of factor x influence x magnitude over all generated combinations plus the characteristic effect diamond; honours `focus`. Table twin.
  - `fatigue-spectrum:<member>`: bins sorted by range, damage share n/N per bar (EN 1993-1-9 endurance), cumulative Miner line vs D=1, total-damage diamond bound to `en1990.fat.damage.<member>`. Table twin.
- Grips (`🕹️handles/🦀️.rs`): `seismic_handles` (choice stepper `E:importance` writing `seismics[id].importanceClass` with tokens I..IV; name carries live A_Ed = gamma_I x A_Ek), `test_series_handles` (`TS:mean` and `TS:sd` on the design-value bar; the sd grip sits at mean+sd with base -mean; names carry live X_d and k_n of Annex D, so the fractile shows live and the design bar re-heights through the live path). `raw_value` made `pub(crate)`; `action_ids` advertises `:importance`; `📊️analyses` `handle_tag`/`spread_tag` return `mean`/`sd` for the testSeries design bar so the scene entity lists the grips.
- Palette (`🧰️kit`): `ENTITY_KINDS` gains `effect` (effects) and `placement` (placements), scene kind Group. `rows_of`/`entities_of` untouched (effects have no id, no scene entity); the generic insert-tool copies the last row (fresh id only where an `id` exists).
- Tests (`🧪️tests/🔬️unit`): `the_bridge_effect_and_spectrum_diagrams_project_the_document_with_table_twins`, `seismic_and_test_series_grips_write_their_leaves_and_show_the_live_fractile`, `effects_and_placements_are_insertable_kinds`. Incremental-equals-full: the new grip ids are part of the scene entities, covered by the existing random-edit live test (`🔬️live`); diagrams are pure functions of document and report.
- NOT done: trafficGroups in a showcase. en1990 has no showcase builder (examples are Rust + DSL asset); all current examples are RailBridge and the group registry only has road categories. Needs a new road-bridge example (snapshot, DSL asset, examples registry, tests) compiled together: plan for the next wave.

## en1992 changes
- `🧰️kit` `ENTITY_KINDS`: `concreteGrade`, `reinforcementGrade`, `prestressSteel` (palette and outliner; Group kind; not scene entities, so `entities_of` unchanged).
- `🎨️grades/🦀️.rs` `legend_diagram`: diagram `grades` with a bar per concrete (f_ck), reinforcement (f_yk) and prestressing (f_pk) grade, hatched when a member uses it, a diamond per bar binding `concreteGrades|reinforcementGrades|prestressSteels[id].fCk|fYk|fPk` (k = 1e6, Pa), table twin with the number of members using each grade.
- `📈️diagrams/🦀️.rs`: calls the legend, adds an anchor diagram per anchor via the shared `case_diagram` (utilisation per anchor check, bound to the checks) and a `placements` plan (diamond per placement binding `placements[id].x`, twin id/x/y/z/yaw).
- `🏛️members/🦀️.rs` `concrete()`: for explicit outlines (>= 3 section points) two grips per point (`<member>/outline-<point>-y|z`, written to `members[id].sectionPoints[id=P].y|z`, anchored on the centroid-relative start face), attached to the member entity.
- Tests (`🧪️tests/🔬️unit`): `the_grade_legend_and_the_anchor_and_placement_diagrams_project_the_document`, `every_section_point_of_an_outline_has_a_grip_per_axis_on_the_member`, `the_palette_offers_the_grade_tables`.

## Unverified / for the compiler
- `legend_diagram` closure typing (`&dyn Fn(&RcMember) -> &str`), `column_of` closure over `&dyn Fn(&Placement) -> f64`, the y-axis marker bindings (mirrors en1990 `sls-limits`), `Tone`/`Hatch` being `Copy`.
- Golden scene re-bless is only needed for documents with explicit outlines (new grips on the member).
- Section-point drag shifts the centroid, so the drawn corner moves slightly less than the pointer (outline is drawn centroid-relative).
- Not coordinated with v14: I only touched `concrete()` in members, the end of `diagrams()` in diagrams, and the `ENTITY_KINDS` table in kit.
