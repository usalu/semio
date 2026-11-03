# Correctness (wave 2): fixes for the doc-oracle findings

Nothing compiled or run on the Rust side (no cargo). Python: `--self-check` of the en1992, en1993 and en1994 oracles passes (`/home/deck/semio/.venv/bin/python -I`); the en1992 oracle evaluates every fixture snapshot with a `bars` row (214 files) without infinity or error.

## 1. en1992 `bars` above the centroid: fixture data, not a sign bug
Convention (snapshot docstring of `RcMember::outline`, scene `member_section`, scene `bar_layout`, generated bars `top - d`): y across, z UP, origin at the gross centroid. `Sec` is correct. The generated placeholder rows were authored in a bottom-left corner frame (`bar-1` at y 0.05, z 0.05 above a 0.35 x 0.5 wall: bottom bar at cover + phi/2 from the corner), i.e. 50 mm ABOVE the centroid in the real frame, so `tension_area() = 0`.
Hand-fixed in 218 JSON files (before/after snapshots, mutation payloads, diffs, catalogue demo document; one-off script in the scratchpad, none left): `bar-1` -> (y -0.125, z -0.20), `bar-1-new` -> (0.125, -0.20), `change-bar-point-z` newZ 0.1 -> -0.15 (snapshot, mutation, diff). Same frame for the placeholder `sectionPoints` stubs (217 files): sp-1 (-0.175, -0.25), sp-2 (0.175, -0.25), `insert-section-point` new point (0.0, 0.25), which also removed a degenerate 3-point collinear outline that crashed the oracle (ZeroDivisionError).
Oracle on the fixed fixtures (finite now): `change-bar-layer-bond-condition` after: flexure 7.88855, crack 13.64875, shear 0.31776; `insert-bar-point` after: flexure 3.08773. Regression rows `fail-08..10` in `🧪️tests/🚦️check-outcome-en1992-1/🥒️.feature` (tolerance 1e-2).

## 2. `shear_v_rd_c_n`
New signature `(b, d, a_c, f_ck, rho_l, n_ed, annex)`, `sigma_cp = -N_Ed / A_c` (0 without area); caller passes `geo.area`. Regression `shear_vrdc_sigma_cp_uses_the_concrete_area_not_b_d_over_point_nine` (compliance tests): b 0.35, d 0.495, A_c 0.175, N +63 kN, C35, rho 0.01: oracle V_Rd,c 101877.082 N (EN), 85209.418 N (DE); the old formula gave 102727.6 / 85889.8 N. The existing worked example got `a_c = 0.15`.

## 3. Polygon members centred
`RcMember::outline_centroid()` (shoelace), `bar_position()`, and `outline()` now move the outline to its centroid; outline and explicit bars share the section-point frame. Used by checks `member_section` (loops and explicit bars), scene `kit::member_section` and `bar_layout`, `depth_from_bars`. Tendon points stay measured from the centroid (documented). Regression `polygon_members_are_centred_on_their_outline_centroid_in_checks_and_bars` (catalogue tests): M_Rd of the wall, N_Ed 63 kN tension = 209.358455 kNm (oracle, structuralcodes 209.358487) to 1e-3, identical to 1e-9 for a corner-frame outline and a shifted frame (0.8, -1.2), centroid at the origin, explicit bar maps to (0, -200 mm).

## 4. en1994 `section_class`
`SteelSection::root_radius_m()` (new, schema-neutral): r from the catalogue area, `A - 2 b t_f - h_w t_w = (4 - pi) r^2`, 0 for welded plates (HEB300: 26.8 mm vs 27 tabulated). `section_class` uses `c_f = (b - t_w)/2 - r` and `c_w = h_w - 2 r`. HEB300/S355: c/t_f 6.18 (was 7.61), class 1 (was 2). Oracle `--self-check` extended with sectionproperties (`i_section r=27`): implied radius 27.09 mm, ratio 6.1842 <= 9 eps. Rust test `section_class_includes_the_root_radius_in_the_outstand_ratios` (+ welded section stays class 2).

## 5. Governing row by utilisation
Resistances are not available at the selection point, so both pickers use a unit-free demand: each effect divided by its peak over the candidate rows, shares added. en1993 `governing_effects` (field `CombinedEffects.situation` became `pub` for the test); en1994 `accumulate` ULS (candidates collected, best by demand). Tests: en1993 `governing_effects_choose_by_unit_free_demand_not_by_a_sum_of_kn_and_knm` (the old sum picks "axial", demand picks "bending"); en1994 `uls_governing_combination_is_chosen_by_unit_free_demand` (M_Ed 360 kNm, V_Ed 180 kN, leading Q1). Check rows of en1993 are unaffected (all ultimate rows evaluated); the en1994 SLS/fire pickers still use |M| only (unchanged).

## Open
- Not compiled: the new Rust tests, `pub situation`, `Moments` untouched. Expected values elsewhere that depended on corner-frame bars (none found in tests) would need a recompute from the oracle.
- True utilisation selection (flexure vs shear separately) needs the resistances in the pickers; not done.
- "ready for rebuild: correctness" appended to `🗑️generated/build-en199{2,3,4}/latest.md`.
