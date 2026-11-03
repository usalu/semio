# 🎯️ Oracle wiring EN 1994

New test module `🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🧪️tests/🎯️oracle-wiring/🦀️.rs`, mounted as `mod oracle_wiring` in `⚖️checks/🦀️.rs` next to `compliance_tests`. The old Python-harness test `third_party_libraries_reproduce_the_blessed_oracle_cases` is untouched.

## Counts
- Cases: 19 in 4 case files. Checks: 8 distinct, all wired (8 adapters). Quantities compared: 81. Known differences: 0. Gaps: 3 cases (quantity `Iw` of `en1994.6.4.ltb-restrained`).
- Expected visible mismatches (hand-derived, not built): see below.

## Wired checks and entry points
| check | evaluator |
|---|---|
| en1994.5.4.composite-elastic | `catalogue::transformed`, `catalogue::modular_ratio`; r > 0 uses the IPE300 catalogue row (A 5381 mm2, I_y 8356 cm4), r = 0 the plate model |
| en1994.6.2.composite-plastic | `catalogue::plastic_full`, `fcc_beam_pa` (z_pna = x in the slab, else the steel axis depth) |
| en1994.6.2.1.4.composite-ultimate | `catalogue::ultimate_curvature`, `moment_at_curvature` with ε_cu, ε_su of the case, no rebar |
| en1994.6.2.1.4.mkappa-elastic | M = E_a I_eff κ with I_eff of `catalogue::transformed` (our M-κ routine is non-linear, no elastic entry) |
| en1994.6.6.3.stud-resistance | `catalogue::stud_resistance_n`, `part_1_1::stud_alpha`, `catalogue::sheeting_reduction` (ribs transverse, t = 1 mm) |
| en1994.6.4.ltb-restrained | `catalogue::mcr_uframe` (k_s > 0) or `mcr_bare` (k_s = 0), `chi_lt` (welded branch for α_LT ≥ 0.49), W_el of the plate section |
| mechanics.beam-continuous | `semio_s_plugin_norm_mechanics::Analysis::solve_case` (48 elements per span) |
| mechanics.beam-envelope | the same kernel over the three load arrangements of the oracle (all spans, each span alone) plus the γ_G,inf case |

## Gaps
- `en1994.6.4.ltb-restrained-{inverted-u-ks, inverted-u-stiff, no-restraint}` quantity `Iw`: warping constant is computed inline in `catalogue::mcr_bare`; suggested owner API `catalogue::ISection::iw()` (I_z (h - t_f)^2 / 4).

## Expected visible mismatches (no `[open] oracle` note covers them, so no KnownDifference)
- `stud-resistance-h-d-5.3-steel-governs` and `-two-studs-per-rib`: `k_t` and `P_Rd_sheeting`. `cat::sheeting_reduction` derives the k_t,max of Table 6.2 from the stud diameter (d ≤ 20 mm counts as through-welded: 0.85 / 0.70) while the cases prescribe 0.75 / 0.60 (profiled sheeting with holes).
- `ltb-restrained-inverted-u-ks` and `-stiff`: M_cr, λ_LT, χ_LT, M_b,Rd. `cat::mcr_uframe` omits the warping term and uses I_fz = I_z/2 (about 0.69 of the blessed M_cr for k_s = 5e4).

## Files changed
- created `...⚖️checks/🧪️tests/🎯️oracle-wiring/🦀️.rs`
- edited `...⚖️checks/🦀️.rs` (mount), `...⚖️checks/📦️packages/🦀️rust/Cargo.toml` (dev-dependency `semio-s-plugin-norm-mechanics`; `compliance` already had `compliance-testing`)

## Assumptions / unverified compile points
No build was run. Hand-derived sign convention of the kernel diagram: `my = -M_sag` (sagging negative), derived from `diagram()` in `🧮️mechanics/📏️statics`. `Outputs::with` needs `&'static str` names (const arrays). `Cargo.lock` will gain the new dev-dependency edge on first build. `cat::NonlinearInput` is built field by field (all fields are `pub`).
