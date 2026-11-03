# Oracle wiring en1991

Test: `evaluators_reproduce_the_blessed_oracle_cases` in `🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧪️tests/🎯️oracle-wiring/🦀️.rs`, mounted from the inferences `🦀️.rs` (`mod oracle_wiring`, cfg(test)). It is NOT mounted beside the old `third_party_libraries_...` test because that file is mounted from the artifact root crate, whose manifest dir (`📦️packages/🦀️rust`) never reaches `✳️any/🧪️tests/🧫️fixtures/🔮️oracle-cases` when the driver walks up (the old test would panic there; owner should move that mount to the inferences crate). Inferences dev-dependency `compliance-testing` is present.

## Counts
- Cases: 35 in 3 files. Checks: 9. Checks wired: 8. Quantities compared: 91. Known differences: 0 (the LM1 tandem proxy was already fixed: `tandem_moment_nm` now matches M_max).
- Gap checks: 1 unwired (mechanics.rail-lm71-mid-moment), 2 partial. Gap cases: 8.

## Gaps
| case | check | quantity | missing | suggested owner API |
|---|---|---|---|---|
| en1991.mechanics.lm1-tandem-span-6_0 / 9_6 / 15_6 | mechanics.lm1-tandem | R_max | no tandem support reaction | `part_2::tandem_reaction_n(axle_n, span_m)` |
| en1991.mechanics.rail-lm71-mid-moment-span-24_0 / 16_0 | mechanics.rail-lm71-mid-moment | M_mid | only axle and UDL constants, no moment | `part_2::rail_lm71_mid_moment_nm(alpha, span_m)` |
| en1991.4.wind-peak-pressure-* (3) | en1991.4.wind-peak-pressure | v_m | no mean velocity function | `part_1_4::mean_wind_velocity(z, terrain, vb, c_o)` |

## Visible differences expected (not registered)
- en1991.snow.drift-minimum: blessed mu_w 0.4 and s_peak 600, ours 0.8 and 1200 (`obstruction_drift` floors mu_w at 0.8; EN 1991-1-3 says 0.8 <= mu_w <= 4). `obstruction_drift` also has no b1/b2 term ((b1+b2)/2h); drift cases 1 and 2 agree only because the gamma h / s_k term governs. Suggested `part_1_3::drift_mu_w(h, b1, b2, gamma, s_k)`. No requests.md discrepancy note covers it, so it stays a mismatch.
- mu_2 of snow.shape-mono-pitch is taken from `shape_mu("multi-span", ..)`, which applies the 1.6 cap (45 degrees: blessed 1.6).

## Case file defect (not edited)
All fire theta_g / theta_max expectations (14 entries) are labelled `K` but hold degC values (iso834 5 min = 576.41 is degC). Adapters return degC. Suggested: relabel to `degC` in `third-party-v2-extended.json`.

## Changes
- New: the oracle-wiring module. Edited: inferences `🦀️.rs` (mount only). No case-file edits.

## Assumptions / unverified (no build run)
- Compile unverified. Terrain category is found from z0 via `TerrainCategory::z_0_m` (cases 0.05, 0.3, 1.0). Parametric fire uses a unit compartment (A_f = A_t = 1, A_v = O, h_eq = 1) in `parametric_fire`; Gamma = gamma_heating (all cases vent-controlled).
