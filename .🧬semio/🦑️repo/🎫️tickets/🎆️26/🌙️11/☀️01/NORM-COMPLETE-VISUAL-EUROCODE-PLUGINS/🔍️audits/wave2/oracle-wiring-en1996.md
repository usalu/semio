# 🎯️ Oracle wiring EN 1996

New test module `🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧪️tests/🎯️oracle-wiring/🦀️.rs`, mounted as `mod oracle_wiring` in the inferences crate `🦀️.rs` next to `compliance_report_tests` (public API only, through `schema::checks::masonry`). The old Python-harness test is untouched.

## Counts
- Cases: 9. Checks: 3 distinct, all wired. Quantities compared: 21. Known differences: 0. Gaps: 3 cases (`en1996.G.phi-m-*`: quantities `lambda`, `A_1`, `u`).

## Wired checks and entry points
| check | evaluator |
|---|---|
| en1996.6.6.1.flexure-reinforced | `numerical_models::MnSection::{at, m_rd}` (plane sections, 0.8 x block, ε_mu 3.5 permille, E_s 200 GPa); x is the root of N(c) = 0 over `MnSection::at` with the bracket of `m_rd` |
| en1996.G.phi-m | `phi_m(h_ef/t, e_mk, t, E/f_k)` |
| en1996.6.2.shear-wall | catalogue M-20 (`lc`, `v-rd`) and M-03 (`fvk` at σ_d = N_Ed/(t l_c)); the mortar class is the one whose Table 3.4 f_vk0 equals the case input (clay, general-purpose) |

## Gaps
- `en1996.G.phi-m-{lambda-0.41, eccentric, slender}`: `lambda`, `A_1`, `u` are internal to `phi_m`; suggested owner API: `phi_m_terms(slenderness, e_mk, t, k_e) -> (lambda, a1, u, phi)`.
- `sigma_d` is composed in the adapter from our l_c (`N_Ed/(t l_c)`); no separate entry point exists (inline in catalogue M-20).

## Files changed
- created `...💡️inferences/🧪️tests/🎯️oracle-wiring/🦀️.rs`
- edited `...💡️inferences/🦀️.rs` (mount). Cargo.toml unchanged: the inferences crate already has `compliance` with `compliance-testing`, `schema` and `model`.

## Assumptions / unverified compile points
No build was run. `MortarClass` is `Copy` (used by value after `f_vk0_mpa` in catalogue M-03). `numerical_models` and `evaluate_outcomes` are expected through the glob re-exports of `checks::masonry`. Outcome values are read from `Outcome::capacity` (limit outcomes carry the resistance there, info outcomes the value).
