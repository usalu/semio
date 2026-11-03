# EN 1997 norm completion report

Artifact: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997` (subset root `S = 🏅️standards/🔖️1/🪆️subsets/✳️any`, checks root `K = S/🧬️schema/💡️inferences/⚖️checks`).
Second attempt after the blocked first one. Built through the `repo` CLI in the background (build loop), never through the MCP build tools.

## Status

Source complete; compile and test status is listed in "Verification" at the end. Everything below that is marked verified was run for real in the standalone harness (see Verification), the `repo` runs are the final gate.

## Implemented catalogue ids (`K/📇️catalogue`, registry `catalogue::registry()`, `evaluate_case(id, &Case)`, 62 entries)

| Group | Module | Ids |
|---|---|---|
| Partial factors | `🧮️factors` | PF-01 (A1/A2, M1/M2, R1-R4, DIN 1054 Tables A 2.1-A 2.3 for BS-P/T/A, DA1-C1/C2, DA2, DA2*, DA3, GEO-3 / GZ 1C, EQU, UPL, HYD) |
| Ground model, statistics, investigation (EN 1997-2) | `🔬️investigation` | G-01 stress profile with groundwater, G-02 / GI-13 characteristic values (k_n with Student t, mean or local, known V_x), GI-01 extent, GI-02 CPT classification I_c, GI-03 cu, GI-04 Kulhawy-Mayne φ′, GI-05 E_oed, GI-06 SPT N60 and (N1)60, GI-07 Hatanaka-Uchida, GI-08 Skempton D_r, GI-09 DPH, GI-10 oedometer C_c C_s c_v, GI-11 triaxial, GI-12 direct shear, GI-14 Menard, GI-15 grain size and Hazen k, GI-16 Thiem |
| Spread foundations, EQU/UPL/HYD | `🏛️foundations` | GEO-F01 drained Annex D.4 (eccentricity both axes, tilted base, two-way H, optional depth factors), GEO-F02 undrained D.3, GEO-F03 DIN 4017, GEO-F04 sliding (drained, undrained, secured passive share), EQU-01, UPL-01 (ballast), HYD-01 (i, i_crit, γH, filter criteria) |
| Settlement | `📉️settlement` | SLS-01 stress-strain (circle, rectangle by Newmark, strip, 2:1), SLS-01B Annex F.2 elastic, SLS-02 consolidation (NC, OC, t90, U(t)), SLS-03 secondary, SLS-04 tilt |
| Piles | `🪵️piles` | PILE-01 static tests (ξ1 ξ2 EN and NA-DE, stiff structure), PILE-02 ground tests (ξ3 ξ4 Table A.10), PILE-03 tension, PILE-04 transverse (Winkler, Broms, p-multiplier), PILE-05 negative skin friction, PILE-06 group (Converse-Labarre, load distribution, block), PILE-07 buckling interface, PILE-08 dynamic (ξ5 ξ6, driving formula), PILE-09 resistance-settlement |
| Earth pressure and walls | `🧱️earth-pressure` | RW-01 Coulomb (wall batter, slope, δ), RW-02 Rankine (slope, cohesion, tension zone), RW-03 passive on a curved surface, RW-04 K0, RW-05 water and surcharges (Boussinesq strip), RW-06 gravity/cantilever wall (sliding, EQU, eccentricity, base pressure), RW-08 Mononobe-Okabe, RW-09 reinforced soil, RW-10 traffic surcharge |
| Embedded walls, anchors, dewatering | `⚓️walls-and-anchors` | RW-07 free-earth-support and cantilever (Blum ×1.2), prop force, M_max, basal heave; ANC-01 tests, ANC-02 empirical, ANC-03 force/proof/Kranz, ANC-04 tendon and free length; DEW-01 single well (Dupuit/Thiem, Sichardt/Kusakin), DEW-02 well group and superposition, DEW-03 settlement from drawdown |
| Slopes | `⛰️slopes` | SLOPE-01 Bishop (and undrained circle moment form), SLOPE-01B Fellenius, SLOPE-02 Janbu with f0, SLOPE-03 infinite slope (dry, seepage, c-φ), SLOPE-04 plane wedge |

Every entry returns outcomes with SI quantities; limit outcomes carry utilisation and, when failing, a remedy (the builder asserts this in debug). Missing inputs give one `NotApplicable` result naming the key.

## Fixed defects

1. `ka_coulomb` was not Coulomb. Now `K_a = cos²(φ−α) / (cos²α cos(α+δ)[1+√(sin(φ+δ)sin(φ−β)/(cos(α+δ)cos(α−β)))]²)`; Ka(30°,20°) = 0.29731, Ka(30°,0) = 1/3 (= Rankine). The thrust is split into E_ah = E·cos δ and E_av = E·sin δ; K0 walls take δ = 0.
2. `kp_coulomb = 1/Ka` removed. Passive pressure uses `kp_curved`: logarithmic-spiral moment equilibrium minimised over the pole position (Rankine at δ = 0, 4.65 at φ = 30°, δ = 15° against Caquot-Kerisel 4.78, 6.93 at δ = φ), with a cache. Plane Coulomb passive is kept only as an information value.
3. Shape factor `s_c = (s_q N_q − 1)/(N_q − 1)` (1.5287 for a square at 30°, was 0.8826). The inclination factor used `A c / N_c` instead of `A c cot φ`; replaced by the EN form with the θ-weighted exponent m. The hidden depth factors are gone (Annex D has none; `include_depth` switches Hansen factors on, default off).
4. φ_d was `atan(φ_rad/γφ)`; now `atan(tan φ/γφ)` in bearing and sliding. EX97-01 μ = 0.5607 is reproduced.
5. DA1 was inverted: C1 = A1+M1+R1 (1.35/1.5, γφ = γc = 1.0), C2 = A2+M2+R1 (1.0/1.3, 1.25/1.25/1.4). New DA2*, DA3 (R3), GEO-3. DA2 DE follows DIN 1054 Tables A 2.1-A 2.3 per situation (BS-A γG 1.10, γQ 1.15; γR,e = γR,v; pile 1.10; tension 1.15). EN DA2 uses R2 (γR,v 1.4). The old values of the inventory (UPL 1.10/0.90, BS-T 0.96 factors, γR,e 1.1) are gone.
6. Sliding resisted with the factored vertical load; it now uses the characteristic permanent V_G (EX97-05 μ = 0.6073). Passive is divided by γR,e instead of γR,h.
7. DIN 4085 row had an invented utilisation ((K0, 1) or (2, 1)); it is now a pure verdict (Pass or Fail with remedies) and additionally rejects δ outside 0…φ and φ outside 0°…50°.
8. Governing-situation rows could not fail (`limit = max(u, 1)`); the limit is 1.0 and the row warns when exceeded. The GK row no longer carries a fake utilisation of 1.0, so it cannot be "governing".
9. Wall overturning used arbitrary arms; it is now the EQU check with E_a split into soil and surcharge parts, EQU factors per situation, vertical thrust component, DIN 4085 horizontal resultant.
10. Bishop (layered): wrong slice-angle sign (it analysed uphill sliding and returned 3.2 for the 25° demo slope), a legacy closed-form fallback, a pond/pore-pressure model with `max(0)`; replaced by crest-toe circle grid (200 circles, 60 slices, m_α > 0.2 admissibility, submerged weights for a horizontal water table) whose `length` is the modelled horizontal extent. The compliant slope now has FoS 1.03 (u 0.971), the non-compliant one fails.
11. Settlement: water table "buoyancy factor" removed (stress increase does not depend on it), 2:1 spread replaced by Boussinesq under the centre (Newmark), sublayers of 0.5 m, cut-off at σz ≤ 0.2 σ′v0. Immediate settlement is the elastic half-space formula `s = 4p(B/2)(1−ν²)/E · I(L/B)` with E_m over 2B (Annex F.2), clauses are no longer mislabelled.
12. Piles: ξ3/ξ4 from EN Table A.10 for every n (was flat 1.30/1.15 for n ≥ 5), type-specific ad-hoc ξ and γ floors removed; pile type enters only where EN does (R4 of DA1-C2: bored 1.6/1.3, CFA 1.45/1.3, driven 1.3/1.3). Tension uses γs,t. The calculated model resistance (single profile, ξ = 1.40) caps the test-derived resistance, so α_s, q_s, q_b, D, L stay meaningful.
13. Layer "strength" heuristic (c_u ≥ 0.15 σ′v, φ+c/1000 ≥ 20) was no EN clause; replaced by the ground-model consistency row `en1997.layer.model.<id>` (thickness, gap/overlap, γ′ < γ, φ′, c, E_oed). CPT/SPT correlations are Kulhawy-Mayne and Hatanaka-Uchida with (N1)60 instead of unreferenced clamped constants. The investigation depth is `max(6 m, 2B)` (EN 1997-2) instead of the 2/3/3.5·B guess.
14. Unknown `designApproach`/`designSituation` fell back silently; `en1997.design.selection` now fails with a one-of remedy. UPL/HYD: stabilising total stress is multiplied by γG,stb.
15. Compile fix of the checked-in tree: four `&mut report` re-borrows of an `&mut CheckReport` parameter (E0596) corrected.
16. Oracle: the old Python snapshot mode mirrored the Rust formulas (including the bugs above). It is rewritten on an independent library (see Tests) and covers walls, EQU/UPL/HYD, undrained, eccentricity, investigation depth, CPT/SPT, tension piles, elastic settlement.

## Tests added

All Rust test code is under `S/🧬️schema/🧪️tests/⚖️compliance`:

- `📇️catalogue-laws/🦀️.rs`: `catalogue_matches_independent_python_oracle_within_one_percent` (107 fixture cases, 694 compared values, runs `🐍️.py --catalogue` through `semio_s_plugin_norm_compliance::python_oracle`), `fixture_covers_every_registry_entry_and_registry_has_all_catalogue_ids`, `every_entry_reports_missing_inputs_instead_of_panicking`, `failing_catalogue_outcomes_carry_a_remedy`, `worked_examples_of_the_specification_are_reproduced` (EX97-01 … EX97-15), `coulomb_active_coefficient_is_the_textbook_value_and_rankine_at_zero_friction`, `shape_factor_sc_divides_by_nq_minus_one`, `pile_correlation_factors_follow_the_en_tables`, `din_4085_row_has_a_verdict_but_no_invented_utilisation`, `governing_rows_warn_when_the_governing_utilisation_exceeds_one`, `unknown_design_approach_or_situation_fails_instead_of_falling_back`, `ground_model_gap_between_layers_fails_with_a_depth_remedy`, `sliding_uses_characteristic_permanent_vertical_force_not_the_factored_load`, `layered_bishop_agrees_with_the_catalogue_bishop_for_one_homogeneous_layer`.
- Hand-fixed existing tests (no compatibility): `da2_de_resistance_factors_follow_din_1054_per_situation`, `da1_combinations_are_a1_m1_r1_and_a2_m2_r1`, `investigation_depth_rule`, `phi_investigation_correlation_numeric`, `passive_earth_pressure_can_change_sliding_pass_fail`, `pile_type_changes_design_compression_resistance` (now under `da1geo`), the perturbation test (documents the three regime-conditional leaves: pile type under R4, OCR at rest, deep-layer parameters below a deep base).
- Language-agnostic: `S/🧪️tests/📇️catalogue-en1997-1/🥒️.feature` (107 example rows plus the snapshot-parity scenario) over the shared fixture `S/🧪️tests/🧫️fixtures/📇️catalogue/🔣️.json`.
- Independent oracle (Python, stdlib, `python3 -I`): `S/🔮️oracles/⚖️compliance/{🐍️.py,📚️geo.py,📇️catalogue.py,📌️reference-values.py}`. `📌️reference-values.py` is the moved reference script of the first attempt (`🐍️.py --reference` asserts Ka 0.29731, s_c 1.5287, EX97-01 μ 0.5607, EX97-12 R_c,d 1212.12). Independent formulations: trial-wedge maximisation for K_a, work-rate balance with contour integrals about the spiral pole and a hand-coded Nelder-Mead for K_p, Boussinesq quadrature (disc, rectangle, strip) for stresses, half-space settlement by integrating the boundary distance over the angle, `scipy.stats.t.ppf` when installed (Simpson inversion of the t density otherwise), `statistics.linear_regression` for laboratory envelopes, closed-form moment polynomials for embedded walls, bisection on U(T_v). The layered Bishop of the snapshot mode mirrors the Rust circle grid (stated limit); the catalogue Bishop is independent.

## Verification

* Standalone harness (scratchpad, `rustc` on the real source files with stubbed compliance types): the whole checks crate (`design`, `geotechnical-parts`, `layer-helpers`, `project-checks`, `pile-wall…`, `catalogue`) compiles in debug (debug assertions on, so the "Fail needs a remedy" assertion holds); 45 of the artifact's own tests (design laws, catalogue laws, remedy laws, perturbation-adjacent laws) pass; the compliant demo complies, the non-compliant demo fails 11 checks, all with remedies.
* Rust vs Python: catalogue 694 values, 0 deviations above 1 %; snapshot mode 37 checks, 0 deviations above 0.5 %.
* A leaf-by-leaf perturbation emulation (every editable leaf, same candidate values as the test) leaves only the three documented regime-conditional groups unchanged, and each of them changes under its regime.
* `repo` runs: see the end of this file.

## Unverified normative values

All spec values marked `[V]` plus the following choices made here: DIN 1054:2010 Tables A 2.1-A 2.3 (BS-T/BS-A rows, γH 1.80/1.45/1.30, EQU 1.05/0.95/1.5, UPL 1.0/0.9/1.5, γb = γs = 1.10, γs,t 1.15, γa 1.10, soil factors 1.25/1.15/1.10 for GZ 1C and DA3); the German annex has no DA1, DA1 uses the EN recommended values for both annexes; EN accidental situation = all factors 1.0; EN R3 γs,t 1.1 and R4 γs,t 1.6, R4 type factors 1.6/1.45/1.3; NA-DE ξ1/ξ2 table (1.35/1.35, 1.25/1.15, 1.15/1.05, 1.05/1.00); NA-DE empirical (EA-Pfähle) resistances are an input switch (`empirical`, ξ = 1) without table data; γw 10 kN/m³ (DE) and 9.81 (EN); DIN 4017 inclination exponents (m+1) and cubic strip form; eccentricity limits B/6 and B/3; Caquot-Kerisel comparison values 4.78 and 6.4 (textbook memory), the log-spiral K_p differs by up to 8 % at δ = φ; Broms cohesionless head load, Winkler 0.322, p-multipliers 0.8/0.4/0.3; Converse-Labarre; anchor ξ_a 1.10, proof 1.25, lock-off 0.9, Kranz η 1.5 and the Kranz wedge with horizontal wall and block forces; free lengths 4.5 m strand / 3 m bar; Blum ×1.2; Sichardt 3000, Kusakin 575; CPT N_kt 15 and α_E 4; DPH N10 table (coarse, not the DIN 4094 text); investigation spacing 20-40 m / 10-25 m; t quantiles for ν between tabulated rows; presumptive bearing table for GK 1 (old values, unchanged).

## Remaining gaps

1. Editor edit coverage: `EDIT_RULES` still maps only the 14 pre-existing set-field paths; the remaining 55 snapshot leaves have no typed kind. Every kind costs about 22 files (leaf, diff patch lane plus its five schema twins, mutation twins, text and binary codecs, oracle manifest, vectors, Python mutation engine, taxonomy fixture), and nested rows (load cases, test profiles) need a nested patch lane in `🔺️diff` that the framework's list-delta macros do not give yet. Designed but not written: one `update-<row>` leaf per row type with all-optional fields and one `SetFieldRule` per leaf sharing the kind. Not done because it cannot be validated within the build queue; request added to `📌️important/requests.md`.
2. The snapshot has no anchors, embedded walls, pile groups, lab tests or per-layer water entities; those verifications run on `Case` inputs through the catalogue (as EN 1995 does) and are not reachable from a document. Adding the entities needs the same schema-twin and mutation work as gap 1.
3. No step adapter for the new `.feature` (the host test API is not exercised); the Rust and Python sides read the same fixture and are compared by the unit test instead.
4. The editor does not list the catalogue yet (visual agents own it; request added), nor the `da2star` approach is wired in the editor panel tables.
5. NDP values are not registered in the shared registry (kernel owner; request added).

## `repo` runs (final gate, NOT yet green)

`repo run @semio-tech/norm-en1997-rs:check` (log `🗑️generated/en1997/check0.txt`) and `repo run @semio-tech/norm-en1997-rs:test` (log `🗑️generated/en1997/test1.txt`) were started in the background and stayed in the capacity queue for over 2 h (`12 ahead` / `35 ahead`, head of queue needs 6.4 GB and never fits). Neither has compiled yet, so "compiling and green" is established only by the standalone harness above, not by the repo build. Both runs are still queued and will read the tree as it is when they start; their logs hold the verdict and must be checked (and then deleted with `🗑️generated/en1997`).

## Follow-up round (coordinator request)

* Kernel drift: `⚖️compliance` gained `CheckStatus::UnverifiedBasis`, `CheckBuilder::ndp`, a capped `ratio` and `.minimum` utilisation `= minimum/computed` (at least 1 on failure). The oracle's `minimum_util` was updated to the same rule. My checks cite no NDP id, so no verdict is downgraded; the registry (`📜️annex-and-tables/🔖️ndp`, owned by the kernel agent) already holds 22 spec-derived `ndp.en1997.*` rows marked Unverified. Registering my own choices (list above) would mean editing its id-sorted generated table and JSON; not done, still a request.
* `check0` (and `test1`) reached `dependencies:generation-verify` after the capacity queue freed and then sat behind another process' dependency build for over 1 h; no compiler diagnostic has been produced yet. Logs: `🗑️generated/en1997/check0.txt`, `test1.txt`.
* NOT done: the 55 set-field leaves, the anchor / embedded-wall / pile-group snapshot entities with mutations, schema twins, diff, I/O and fixtures, and the `.feature` step adapter. Each new kind touches about 22 files across schema twins, codecs, manifests and vectors and cannot be validated without a working build loop; attempting it blind would risk the green state. Gaps 1-3 above stand unchanged.

## Edition alignment (DIN 1054)

DIN 1054:2010-12 is withdrawn; the current edition is DIN 1054:2021-04 (`🔍️audits/ndp-public-verification.md`). Clause references in the code docstrings now cite 2021-04. The factor values (Tables A 2.1-A 2.3, A 7.1 ξ) were transcribed from the specification, which follows 2010-12, and could not be confirmed against 2021-04 in public sources: all of them stay **unverified** and must be re-keyed from a print copy; the kernel registry rows `ndp.en1997.l45.1` and `ndp.en1997.l183.1` already carry status Unverified and should be re-based to 2021-04 by the registry owner (the table is generated and id-sorted, not edited here). No check cites those ids, so no verdict claims compliance on them.

## Review round (`🔍️audits/review-en1992-en1997.md`, EN 1997 S1/S2 items)

Fixed, harness-verified (52 tests, catalogue parity 725 values, snapshot parity 37 checks, leaf-perturbation emulation) and saved for run 3:
1. `Outcome::utilization()` returns 1e6 (fail) for demand > 0 and capacity <= 0 or non-finite; test `zero_or_negative_capacity_never_passes`.
2. M2 soil factors now act on earth pressure and passive resistance: RW-03 (φ_d, c_d), RW-06 (tan φ/γφ), RW-07 (φ_d in K_ah, K_ph), and in the project wall check per situation (K_a, K_p, wall friction from φ_d). Test `m2_soil_factors_reduce_passive_pressure_and_the_embedded_wall_gets_longer`.
3. Passive wall friction capped at 2/3 φ (`kp_curved`, cache key and Python oracle).
4. Cancelled slip search or no admissible circle returns NaN, and the project check reports NotApplicable "not evaluated" instead of F = 50.
5. Project Bishop: total weights with saturated weight below the phreatic line, pore pressure γw·(y_w − y_b), the line parallel to the ground at depth `groundwaterLevel`. Compliant slope u = 0.93, non-compliant fails. Catalogue slope surcharge multiplied by γQ.
6. HYD: no inverted fallback pore pressure; the row is NotApplicable when `porePressure` is not given.
7. Piles: γt separate from γb (`gamma_t`, used by PILE-01/08), R1 type factors (bored 1.25, CFA 1.2, driven 1.0; unverified), R1 γs,t 1.25, ξ tables are step functions.
8. Mononobe-Okabe: no silent clamp (error when φ − θ − β < 0), more onerous of ±kv.
9. Wall self-weight is `t_stem·(H + B)` instead of `B·t_stem`.
10. UPL water force scales with an `area` input; GEO-F01 swaps the H components with B′/L′ and reports e_L and the core ellipse.
11. DIN 1054:2021-04 citations (values still unverified).

Not done from the review: new entries (rock, near-slope, gaping joint, CPT pile resistance, p-y lateral, SSI walls, Spencer/Morgenstern-Price, EN 1997-2 lab series beyond GI-10..12), GK 1 presumptive table removal, separate K_pc/K_pq, rigid-wall strip-load doubling.
