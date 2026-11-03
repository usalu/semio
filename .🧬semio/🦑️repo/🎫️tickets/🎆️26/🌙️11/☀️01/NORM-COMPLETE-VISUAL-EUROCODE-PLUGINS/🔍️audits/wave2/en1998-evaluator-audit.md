# EN 1998 evaluator audit (wave 2): `⚖️part-evaluators/🦀️.rs` after the v17 truncation

Nothing compiled (no cargo/rustc/`repo run`). Reading, grep and Python gates only.

File: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️part-evaluators/🦀️.rs` (652 lines; HEAD 385 lines, HEAD predates every wave-2 change, so HEAD is not a reference for the tail: it still has the old `mononobe_okabe_k_ae`, `h_rd_n`, `design_spectrum_sd`).

## 1. Expected behaviour per report and result

| Report | Expected in this file | Found |
|---|---|---|
| part5-v2 | `.thrust` via kernel `thrust(Side::Active, Interface::vertical(phi, 0), gamma, H, Water, Acceleration{kh, kv = share kh})`, share from registry (threshold/high/low, type-2 avg row), `k_h = alpha S / r` with gamma_I a_gR, no-equilibrium Fail with remedy on `r`, water in `H_Ed` | present, correct (kernel fn names/fields/derives checked against `mechanics::geotechnics::earth_pressure`) |
| part5-v2 | `.sliding`: `(V-u) tan(delta)/gamma_Rd + passive`, `gamma_Rd` = `ndp.en1998.en5.sliding.gamma-rd`, V = supported-building seismic weight, `u = basePorePressurePa * area`, remedies on `passiveResistanceN` and `baseFrictionAngleDeg` (EN+DE) | present (`part_5::sliding_friction_resistance_n`, `required_friction_angle_deg` exist and match) |
| part5-v2 | `.water` (hydrostatic + Westergaard against H_Rd, remedy lowers the water height, only when `waterHeightM > 0`), arm `WESTERGAARD_ARM` | present; `Remedy::at_most`, H scaled by sqrt(H_Rd/H_w) (thrust ~ H^2) |
| part5-v2 / v17 | `.tie` (+ `.foundationAId/.foundationBId`, `.displacement`), ground A/E not applicable, remedy on steel area / displacement limit | present, formula equals the Python oracle `tie_beams` (checked incl. the gamma_I a_gR argument) |
| v17 | `.slope` Bishop on shared kernel, M2 factors, grid + given circle, remedies on friction angle and slope angle (bisection), dangling layer Fail with one-of remedy | present, structure equals oracle `seismic_slopes` |
| v16 / part3 | assessment checks: `.supportedBuilding`, `.limitState` (T_R, a_g factor, CF, `na_return_period_note`), `.capacity`, then `check_chord_rotation`, `check_cyclic_shear`, `check_analysis_method`, `check_n2_performance` | present; callee signatures verified (`🪨️part-details`, `🏚️assessment-checks`, `part_3_state::basis`) |
| part2-v2 / v3 | bridge: `check_bridge_action`, `.periodRatio` as `minimum(.., 1.0)` with remedy, `bridge_analysis(.., annex)`, `check_bridge_rows`, `check_bridge_piers` (stiffness check lives in `structure-checks`), fallback `.bearing` + `check_pier` when no analysis | present, all arities/types match `🌉️structure-checks` (`analyse`, `check_rows`, `check_action`, `check_piers`, `Ground`) |
| v14 | tanks: `check_tank_hydrodynamics(report, annex, tank, S_e_i, S_e_c, T_c)`; towers: `tower_period`, `check_tower_modal`, `.overturning` at the period of `tower_period` | present, signatures match; ids `.period`, `.modal.*`, `.interaction`, `.overturning` match scene readers and structures tests |
| v17 | `evaluate`, `evaluate_towers` | `evaluate` identical to HEAD; `evaluate_towers` = wave-2 shape (scope-aware, `design_spectrum_at`, `cantilever_force_factor`, `floor_limit`), consistent with `part_6` (the renamed `cantilever_force_factor` exists, `cantilever_modal_participation_factor` is gone everywhere) |

Also verified: caller order in `check_full_seismic_scoped` (all five `evaluate_*` take `scope`; `evaluate_foundations_walls` has `gamma_i` last), unit order in `🧮️units` (`walls-na`, `tie-beams-na`, `slopes-na` and the `retainingWalls`/`tieBeams`/`slopes` tags match the `scope.part`/`scope.admits` strings and the evaluation order), all 13 NDP ids exist in the registry, all snapshot fields used exist (`passive_resistance_n`, `base_friction_angle_deg`, `base_pore_pressure_pa`, `water_height_m`, `saturated_gamma`, `permeability_m_s`, tie and slope fields), mechanics dependency in the inferences Cargo.toml, helpers (`q_*`, `floor_limit`, `lc`, `id_sel`), `Remedy::{at_least, at_most, one_of}`, `CheckResult::ndps`, `Quantity::{area_m2, length_m}`, `Scope::{part, admits}`.

Conclusion: no function lost or wrong. The v17 re-creation of `check_wall_thrust` and `evaluate_towers` is complete against every report; no earlier edit (part5-v2, v16, part3, part2-v2/v3, v14) is missing. Calculations agree with the independent oracles (`⚖️compliance/🐍️.py`: `wall_thrust_permeable`, `wall_thrust_low_permeability`, `westergaard_thrust`, `sliding_friction_n`; `🚦️check-outcome/🐍️.py`: tie, slope).

## 2. Fixes made (small)
1. `check_wall_thrust` and `check_slope`: the NDP list (`.ndps`) named only `ndp.en1998.vertical-spectrum.avg-ratio` although spectrum type 2 reads `avg-ratio-type2`; the list now carries the row actually used (`avg_id`).
2. Check-outcome oracle `🔮️oracles/🚦️check-outcome/🐍️.py` `foundation_gravity`: an empty `supportedBuildingId` fell back to no building (weight 0) while the Rust evaluator, like bearing and sliding, falls back to the first building; the oracle now does the same (no committed fixture is affected).

## 3. Open (not changed)
- Oracle coverage gap: the Python check-outcome oracle and the feature rows cover no wall thrust, sliding or water outcome (only the kernel numbers via `⚖️compliance` and the blessed geotech cases); a fixture with a wall and water plus rows would pin `.thrust`/`.water`/`.sliding` outcomes (needs the oracle to implement MO; kernel is already third-party-validated against scipy).
- Scene reads `.water` computed (static + dynamic) as the arrow magnitude `dynamic` (variable name only).
- Everything still compile-unverified.

## 4. Gates run
- en1998 check-outcome Python dry run (`wave2-features-dryrun.py`): 64 ok, 0 failed (before and after the oracle edit); oracle `--self-check` ok.
- Fixture checker (`kind-convention-v4/verify_fixtures.py en1998`, heavy.sh): 335 scenarios, 327 inverted, 0 failures.
- Remedy law (`remedy-law/scan.py`, heavy.sh): 507 sites, 0 findings.
- Spec-trace verifier (`⚖️compliance/🧭️spec-trace/🧰️codegen/🐍️.py`, heavy.sh): en1998 68 ok (all ten codes ok).

- ready for rebuild: en1998 evaluator audit
