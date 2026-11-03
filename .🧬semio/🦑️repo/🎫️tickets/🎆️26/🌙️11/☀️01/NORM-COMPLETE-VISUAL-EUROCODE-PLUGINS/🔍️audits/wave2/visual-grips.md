# 🕹️ Visual Grips, Wave 2 (VISUAL-GRIPS)

Scope: Eurocode artifacts only. Nothing compiled or run (builds are the infra agent's); every claim below is by reading. "ready for rebuild: visual grips" is appended to the `latest.md` of en1990, en1991, en1992, en1993, en1994, en1996, en1998, en1999.

## 1. Audit verification (static literal matching had false negatives)

- en1995 (9 "no grip" collections: tapers, cutouts, fasteners, panels, floors, diaphragms, trusses, bracings, decks): FALSE NEGATIVE. `🎬️scene/🕹️handles` `entity_handles` builds grips for all nine through `ENTITY_TABLES` + `drawing_row` homes (`timber::entities`); the v5/v6 grips exist under the table name, not under the literal collection. No change.
- en1991 "17 collections without diagram": mostly FALSE NEGATIVE (diagrams are keyed by group, not collection: wind qp/cpe, snow, fire, bridge lanes/influence/rail, stage timeline, silo, psi, bridge wind, dynamic, vortex, impact, gas explosion, section temperature). Real gaps closed below: tieRuns, helicopterLandings.
- en1998 "13 of 14 collections without diagram": FALSE NEGATIVE. Spectra, vertical spectrum, drift, mode, participation, base shear / force profile, pushover, q chart, chord rotation, diaphragm, time history, soil/liquefaction/pile, annex F, bridge modes and pier moments, tank wall pressure, tower modes/forces exist (`📈️diagrams`, `🌋️capacity`, `🌍️geotechnical`, `🧱️horizontal-elements`, `🎬️scene-structures`). Real gap closed: silos.
- en1993 (11 no-grip collections): real for sections, fireExposures, tensionComponents, towerLegs, craneRunways, memberActions; non-geometric and left (see 7): materials, loadCases, fatigueDetails, frameBars.

## 2. Placements: move/rotate grips (item 1)

Shared (additive) `🎬️structural-scene/🧰️projection-kit/🕹️handle-kit/🦀️.rs`: `move_handles(entity, id, anchor, stored, standing_yaw, yaw)` returns `<entity>:move-x`, `:move-y` (Translate, base = persisted row else standing pose via `grip_base`) and optionally `:move-yaw` (`Angular` about +Z through the anchor, zero at +X, so `s` is the absolute yaw; unit rad). Writes are `placements[id=E].x|y|yaw`; `PlacementCommit::of_writes` turns them into one insert (first drag) or one set = one undo step. All six edit-rule sets already have `placements[].x|y|yaw`.

| Artifact | Entities | yaw |
|---|---|---|
| en1990 | members (cases already had it via `design_case::forms`) | yes |
| en1992 | members (`🏛️members` concrete spec lists the ids), anchors (`⚓️anchors` shaft) | members yes, anchors no (layout ignores yaw) |
| en1993 | members; plated panels, piles, cold-formed, tension, silo shells, parametric sections, bridge fatigue (row entities) | members yes |
| en1994 | beams, columns, slab strips (also in `ids`/`column_ids`/`slab_ids`) | yes |
| en1996 | walls (cases already had it) | yes |
| en1998 | buildings, retaining walls, bridges, silos, tanks, towers (`all()`) | no (layout has origins only) |

Tests: projection-kit unit `move_handles_write_placement_leaves_and_yaw_anchors_at_the_angle`; en1993 `placed_entities_get_move_and_rotate_grips_and_sections_get_dimension_grips`; en1992 `members_and_anchors_advertise_move_and_rotate_grips` (+ `EXISTING_RULES` gained `x`, `y`, `yaw`); en1998 `every_placed_entity_has_move_grips_and_every_silo_a_demand_diagram`. Not done: en1999 offers move grips only for members that already have a placement row (existing open request: `add-placement` insert kind), en1991/en1995/en1997 were already done.

## 3. Collections without grips (item 2)

en1993 (`🎬️scene-members/🕹️handles`, all leaves have existing edit rules):
- `sections[id].h|b|tw|tf` as `section-h|b|tw|tf` on the member that references the section (half-extent grips use k = 2).
- `tensionComponents[id].cableLength` (`cable-length`); the cable is now drawn with that length (`🧱️plates-and-shells`, fallback 3 m if 0).
- `towerLegs[id].height|diameter`, `craneRunways[id].webThickness|flangeThickness|wheelEccentricity`, `fireExposures[id].protectionThickness` on their load glyph entities (the glyph spec lists the ids via `gripped_glyph`).
en1995: already covered (section 1).

## 4. Nested action rows (item 3)

New shared `handle_kit::load_arrow` (grip at `|value| / per_metre` from the glyph point, outward drag changes the value by `per_metre`, sign kept).
- en1993: `memberActions[id].action.n|vz|my` on glyphs `<row>:n|vz|my` (`<glyph>:magnitude`), `towerLegs[id].actions[id=A].force` (wind actions) and `craneRunways[id].actions[id=A].force`; test `nested_action_rows_get_magnitude_grips_on_their_glyphs`.
- en1999: `members|connections|coldFormed[id].actions[id].nK|vZK|mYK|qKLine` on glyphs `<owner>:load-<action>:n|vz|my|udl`; `action_grips` in `🏗️members` is the single source for glyph handle lists and the `build` dispatch (`ACTION_MARK`); test `nested_action_rows_get_magnitude_grips_on_their_glyphs`. Shells draw no action glyph, so none.
- en1991: `accidentalCases[id].impact[j].vehicleSpeed` and `.explosion[j].standoff|explosionMass` (rows have no ids: position paths, edit rules use `SelectorKey::Implied`) via new `accidental()` in `🕹️handles` (the owner returned no handles before); `IMPACT_*`/`BLAST_HEIGHT` made `pub(crate)`; test helpers `known_path`/`current` extended for the nested paths.
- en1998: NOT done. `Storey/Bridge/Tower.variables` have no 3D glyph at all; a grip needs a new glyph entity (new geometry), not a binding on an existing one. Open item.

## 5. Diagrams (item 4)

Axes with unit and bilingual labels, table twins, markers bound to the field they edit:
- en1998 `silo-demand:<id>` (`🎬️scene-structures`): N_Ed = m (g + 0.9 a_g) and V_Ed = S_d m over a_g with N_Rd, V_Rd limits and design markers (same kernels as the silo checks `en1998.4.<id>.wall/.anchor`).
- en1991 `tie-forces` (T_p/T_i over span with the 75 kN cap, required and provided markers; provided is draggable) and `helicopter-landing-force` (F_d = 3 √m kN as in the check E.17.05, assumed force marker); `Group::Tie`/`Group::Helicopter` focus routing. Tests `tie_runs_and_helicopter_pads_have_force_diagrams_with_axes_and_units`, en1998 test above.
- Not added for lack of a normative formula in the checks: fallingLoads, executionImperfections, derailmentCases, constructionLoads, specialVehicles, accidentalCases (summary diagram).

## 6. en1992 grade colours (item 5)

New `🎬️scene/🎨️grades`: per member an overlay surface `<member>:grade-field` on the side face with scalar fields `grade-fck` (concrete f_ck) and `grade-fyk` (reinforcement f_yk) in MPa on the viridis ramp, two scalar legends (domain = min/max of the document's grade tables, so one colour names one grade), layer `grades` hidden by default (the toggled view option; the `Fields` channel lets the viewer switch fck/fyk). Test `members_carry_a_hidden_grade_field_with_one_legend_per_grade_table`.

## 7. Open items / not done

- Goldens of en1990, en1992, en1993, en1994, en1996, en1998 (and en1999/en1991 entity handle lists) change; re-bless once with `SEMIO_UPDATE_GOLDEN=1` after the first green build (request filed).
- en1993 materials, loadCases, fatigueDetails (category), frameBars carry no geometric field; not gripped by design. en1990 trafficGroups/seismics etc. out of this brief.
- en1998 variables glyphs (section 4); en1999 first-drag placement rows need the `add-placement` kind.
- `structural-scene` crate is over the source-boundary budget in the scan (224 660 bytes); my additions are about 3 KB (move_handles, load_arrow, test) and should move with the planned split.
- Unverified by compiler: everything. Riskiest spots to read first on a rustc error: `handle_kit::move_handles` iterator chain, en1992 `members::concrete` move block, en1993 `glyph_handles`, en1999 `action_grips` closure `push`, en1991 test helper `current`.
