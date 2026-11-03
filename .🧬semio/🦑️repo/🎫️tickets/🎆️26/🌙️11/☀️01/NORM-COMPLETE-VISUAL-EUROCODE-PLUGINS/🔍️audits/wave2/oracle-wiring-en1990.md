# Oracle wiring en1990

Test: `evaluators_reproduce_the_blessed_oracle_cases` in `⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧪️tests/🎯️oracle-wiring/🦀️.rs`, mounted from the inferences `🦀️.rs` (`mod oracle_wiring`, cfg(test)). Dev-dependency `semio-s-plugin-norm-compliance` with `compliance-testing` already present.

## Counts
- Cases: 21 in 4 files. Checks: 7. Checks wired: 6. Quantities compared: 58. Known differences: 0.
- Wired: en1990.C.beta (phi_inv), C.form-linear and C.form-nonnormal (form_r_minus_e, Dist::{Normal,LogNormal,Gumbel}), 6.4.combinations (combinations/governing/governing_610ab, EN annex, building, StrB, k_FI 1; the case psi triples are mapped to the PSI_TABLE category with that row), D.7.characteristic-from-tests (k_n, test_design_value), fatigue.miner (miner_damage).
- Gap checks: 1 unwired (mechanics.beam-envelope), 2 partial. Gap cases: 6.

## Gaps
| case | check | quantity | missing | suggested owner API |
|---|---|---|---|---|
| en1990.mechanics.two-span-load-arrangement-envelope, en1990.mechanics.unequal-spans-envelope | mechanics.beam-envelope | M_sag_0, M_sag_1, M_hog_support, R_max_support, R_min_0 | no continuous beam solver in en1990 | `continuous_beam_envelope(spans, g, q, gamma_G, gamma_Q, gamma_G_inf)` |
| en1990.fatigue.miner-two-slope, en1990.fatigue.miner-below-limit | fatigue.miner | dsigma_D, dsigma_L | cut-off ranges only computed inside `en1993_endurance_cycles` | `en1993_cut_off_ranges(delta_sigma_c)` |
| en1990.C.form-vs-exact-moderate, en1990.C.form-vs-exact-high-cov-action | C.form-nonnormal | beta_exact | no exact P_f quadrature | `exact_failure_probability(&Dist, &Dist)` |

## Changes
- New: the oracle-wiring module above. Edited: inferences `🦀️.rs` (mount only).
- Edited case file `third-party-v2-extended.json`: per-expectation `tolerance` 0.002 on X_k and X_k_known of the three D.7 cases (Table D1 is printed with two decimals, error up to 0.09 percent); reason added to the clause text. JSON validated. k_n_unknown / k_n_known stay on numerical-v1 (0.1 to 0.3 percent deviation). The n = 20 note in requests.md concerns no case.

## Assumptions / unverified (no build run)
- Compile unverified; static referencing static (`&ADAPTERS`) needs Rust 1.83+ (workspace 1.95).
- Combination adapter maps ψ triples to the first PSI_TABLE EN row (residential for 0.7/0.5/0.3, wind for 0.6/0.2/0, snow for 0.5/0.2/0); gamma_Q is category independent for buildings.
- FORM cases may show real mismatches (Rackwitz-Fiessler vs SLSQP, numerical-v1 0.5 percent) and are left visible.
