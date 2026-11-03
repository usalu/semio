# DIAGRAMS-B (wave 2): en1995, en1997, en1999 diagrams, showcase, palette

Nothing compiled (no cargo/rustc/`repo run`), no git writes. Rebuild lines appended to `🗑️generated/build-en1995|en1997|en1999/latest.md` ("ready for rebuild: diagrams-b"). Paths below are under `ART(code)` = `✏️s/🔌️plugins/📕️norm/🗿️artifacts/<emoji><code>/🏅️standards/🔖️1/🪆️subsets/✳️any`.

## Verified gaps (editor-ux-audit 12, 13, 19)
- Gap 12 (en1995): true for tapers (T-30), notches (T-45), diaphragms (T-94), trusses (T-95), bracings (T-96), decks (T-122). Panels already had `case-layup` (T-97) and `case-results`; not touched. Placements: no diagram (layout only, no physical quantity), left out on purpose. No full showcase document existed for en1995 at all.
- Gap 13 (en1997): STALE for slopes (`slip-forces` and `slip-centre` already exist, centre sweep with the current circle marked) and for layerWaters (`pore-pressure` exists). True for upliftCases, anchors, pileGroups; added a head-profile for layerWaters on top.
- Gap 19 (en1999): STALE for the outline diagram (existed for free outlines), insert rules (`insert-section|material|placement|outline-vertex` exist in edit-rules) and transverseStiffeners in the showcase (generator already adds 5 rows). Real gaps closed: parametric sections had no outline, the diagram had no focus and could not be reached by section id, and the insert palette (`ENTITY_KINDS`) lacked sections, alloys, outline vertices and placements.

## en1995 (all pure projections of the case row the entity is drawn as; EN + DE; table twin each)
- `🎬️scene/📈️diagrams/🧩️components/🦀️.rs` (new, mounted in `📈️diagrams/🦀️.rs` `case_diagrams`): `case-taper` (k_m,alpha and edge utilisation over the angle 0..10 deg, marker binds `alpha_deg`), `case-notch` (k_v and shear utilisation over h_ef/h, marker binds `h_ef`), `case-racking` (wall) / `case-shear-flow` (floor, roof) for T-94, `case-truss-chord` (q_d over the restraint length vs q_Rd), `case-bracing` (delta = sum F_d / k_br over k_br vs L/500, label carries k_req), `case-wheel-spread` (b_ef = a + 2h unverified, stress vs f_c,90,d). Entity rows re-home markers through the existing `entities::rehome`.
- Kernel (`🧬️schema/⚖️kernel`): `taper_probe`/`TaperProbe` and `notch_probe`/`NotchProbe::kv` (t30 and t45 refactored onto them, same maths, `m_d`/`v_d` optional so drawing rows work), `racking_factor` extracted from t94, re-exports `km_alpha`, `bracing_qd`, `wall_panel_racking`. The utilisation curves scale the report's own utilisation (u(alpha) = u0 k(alpha0)/k(alpha), notch likewise), so they are exact against the check without needing the governing load combination.
- `drawing_row` (`🧬️schema/⚖️timber/🧱️entities`): T-45 row gained text `side` (support/other, was missing: k_v was always 1 in the drawing), T-94 row gained `f_f_rd` and `q_v_rd`.
- `🎬️scene/📈️diagrams/🧩️special`: `val`, `marker`, `diagram`, `text` are `pub(super)` for reuse.
- Showcase: `🧰️scripts/showcase/en1995.py` (ticket) writes `🧪️tests/🧫️fixtures/🎪️showcase/🔣️.json` (all 9 entity collections, wall and roof diaphragm, 2 tapers incl. single-tapered, notch and hole, placement, 9 catalogue cases; validated against the schema). `En1995Snapshot::full_showcase()` in `🧬️schema/📸️snapshot/🦀️.rs`, palette entry `full-showcase` in `✏️editor/🦀️.rs`.
- Tests: scene unit `component_diagrams_project_the_tapers_notches_diaphragms_trusses_bracings_and_decks_and_an_edit_changes_only_its_own` (closed forms: k_m,alpha(0) = 1, k_v(1) = 1, racking = n F_f,Rd b_i/s, deflection monotone, only the edited taper's diagrams change, pure function); editor test for the palette entry (DSL round trip, every entity collection populated).

## en1997
- `🎬️scene/📈️diagrams/🗺️plans/🦀️.rs` (new, mounted in `diagrams()`): `uplift-stack:<case>` (stabilising bar, stacked permanent+variable destabilising bar, design bars from the `en1997.10.upl.<id>` check), `anchor-layout:<wall>` (wall section, free length dashed, fixed length solid coloured by anchor utilisation, head markers with utilisation), `pile-group-plan:<group>` (pile grid, rigid-cap reaction N_i = V/n + Mx y/Sy2 + My x/Sx2 per pile vs single resistance, utilisation colours), `head-profile:<water>` (h(x) = dh (1 - x/L) with i and i_crit in the label).
- Test: `uplift_anchor_pile_group_and_head_diagrams_are_pure_projections_that_follow_the_document` (900/550 kN, sum of reactions = V, head 1 to 0, edit of uplift and pile columns changes only those, anchors and head untouched).

## en1999
- `outline_diagram` (`🎬️scene/📈️diagrams/🦀️.rs`) now covers parametric sections through `kit::section_shape(...).outline()` + `checks::polygon_props`; free outlines get one marker per vertex; parametric get a `width` marker (binds `sections[id].width`, x*2) and a `web-thickness` marker (I: x*2, box: outer - x); diagram has a section focus and is emitted once per section, found by section id, using member id or outline-vertex id.
- `🎬️scene/🧰️kit/🦀️.rs`: `ENTITY_KINDS` + `entity_kind_of` gained `sections`, `materials`, `outlineVertices`, `placements` (insert palette and outliner rows; `rows_of`/`entities_of` untouched, they are not scene entities of their own). Edited with a fresh read; CHECK-BINDING edits to the same file were not touched.
- Tests: `the_section_outline_is_drawn_for_parametric_sections_with_width_and_web_markers_and_follows_the_edit`, `the_palette_offers_sections_alloys_outline_vertices_and_placements`.
- Showcase: transverseStiffeners already present; `--check` unchanged.

## Gates
- handle-paths en1995, en1997, en1999: 0 uncovered. Showcase generators en1995/en1999 `--check`: unchanged.
- Fixture checker (`kind-convention-v4/verify_fixtures.py`, via heavy.sh): see the last line of this file.

## Open and unverified
- Never compiled. Risks to look at first: glob visibility of parent privates in the two new modules (`components`, `plans`), `Series::Polygon`/`Marker` field names copied from existing code, `Vec2` is `Copy` in the en1995 test, `Ring2` points `.x/.y`, `check.computed/limit.value` meaning of `.minimum(stab, dest)` in the uplift diagram.
- Marker drag axis in diagrams is read as x (existing code maps x); the height/flange-thickness of a section stay on the 3D handles, only width and web thickness are x-bound on the outline.
- "Incremental equals full" is asserted as: diagrams are pure functions of the document (clone equality) and an edit changes only the edited entity's diagrams; the incremental report path itself is covered by the existing `incremental_laws!`.
- T-122 `b_ef` stays flagged unverified (Annex A.2 text not in specs). Truss "member forces" is the chord stabilising load of T-95; no member-force solver for `truss_layout` was added.
- New diagrams add ids to `diagrams(doc, report, None)`; golden or count tests of en1997/en1999 diagram lists (not found by grep) would need the new ids.
