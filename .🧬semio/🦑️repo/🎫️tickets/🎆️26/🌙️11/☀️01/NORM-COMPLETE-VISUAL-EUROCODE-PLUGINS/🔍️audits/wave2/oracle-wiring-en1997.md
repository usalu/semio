# Oracle wiring en1997

## Counts
- Blessed cases: 36 in 4 files (16 distinct checks).
- Checks wired: 14 (boussinesq, bearing-resistance, bearing-factors, stress-strain-settlement, settlement-pairs, oedometer-settlement, embedded-wall, pile-group, pile-resistance, anchor-resistance, earth-pressure, cpt-friction-angle, cpt-undrained-strength, cpt-behaviour-index).
- Known differences registered: 0 (the `include_depth` note is covered by calling `bearing_drained` with `include_depth: false`, so the Annex D cases are compared directly).
- Gap checks: 2 unregistered, 3 partly registered. Gap cases: 8 of 36.

## Gaps (no usable evaluator entry point; nothing faked)
| Check | Cases | Quantity | Missing | Suggested owner API |
|---|---|---|---|---|
| en1997.2.phase-relations | 2 (saturated-clay, partly-saturated-sand) | gamma, gamma_eff | no phase-relation function (G_s, S_r, e, gamma_w to bulk and buoyant unit weight) | `catalogue::factors` or `G-01` helper `bulk_unit_weight(g_s, s_r, e, gamma_w)` and `buoyant_unit_weight` |
| en1997.8.anchor-elongation | 1 | delta, N | no free-length elongation delta = A_d l / (E A_t) | `walls_and_anchors` ANC-04 outcome `elongation` |
| en1997.8.anchor-resistance | 1 | R_a_h, utilisation | no horizontal component R_a cos(theta); ANC-02 utilisation is against R_a,d = R_a,k / gamma_a (R1 gamma_a = 1.1), no characteristic-level utilisation | ANC-02 outcomes `r_a_h` (needs `theta_deg`) and `utilisation_k` |
| en1997.7.pile-group | 2 | R_group | PILE-06 gives eta only, no group resistance eta m n R_c | PILE-06 outcome `r_group_k` |
| en1997.F.oedometer-settlement | 2 | delta_e | SLS-02 returns the settlement only, not the void-ratio change | SLS-02 outcome `delta_e` |

## Adapter notes
- Unfactored catalogue cases: `AnnexChoice::En` with `situation = bsA` (a-acc, M1, R1: all factors 1.0 except `r1.a` = 1.1, `r1.st` = 1.25, neither used).
- boussinesq: strip at offset by superposition of `sigma_z_strip_centre` (point at distance d from a strip edge = half the centre stress of the width 2d, signed); rectangle corner = `q * newmark_corner(width/z, length/z)`; circle axis = `sigma_z_circle`.
- stress-strain-settlement and settlement-pairs: SLS-01 over 64 equal layers (midpoint rule) down to z_max, the series limit of the entry; the oracles integrate continuously, expect an error far below the 0.5 percent of `numerical-v1`.
- settlement-pairs: beta_12 = SLS-04 `tan_omega`; tilt = gradient of `sls_pairs::settlement_plane`.
- embedded-wall: K_ah, K_ph from `horizontal_coefficients`, t from RW-07 `t0` (equilibrium embedment, no 1.2 factor). K_ph uses `kp_curved` (Annex C logarithmic spiral) while the oracle uses the plane Poncelet/Coulomb K_p: expect visible MISMATCHES on K_ph and t for delta_p > 0 (`embedded-wall-cantilever-friction`); delta_p = 0 cases should agree. No open request covers it, so no known difference registered.
- cpt-undrained-strength: GI-03 needs qt and sigma_v0; `qt = q_net + 100 kPa`, `sigma_v0 = 100 kPa` so that `(qt - sigma_v0) / N_kt = q_net / N_k`.
- cpt-behaviour-index: GI-02 with `qc = qt`, u2 = 0.
- pile-resistance: `part_1::{shaft_resistance_n, base_resistance_n, pile_correlation_factors, pile_characteristic_resistance_n}`; the base area pi D^2 / 4 is formed in the adapter.

## Files changed
- New: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🧪️tests/🎯️oracle-wiring/🦀️.rs`
- Edited: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🦀️.rs` (mount `#[cfg(test)] mod oracle_wiring;` after `sls_pairs`).
- Cargo.toml: none (checks crate already has `semio-s-plugin-norm-compliance` with `compliance-testing` as dev-dependency).

## Mount choice
The existing `third_party_libraries_reproduce_the_blessed_oracle_cases` lives in the artifact root crate whose `CARGO_MANIFEST_DIR` is above the oracle-cases folder, so the driver (walks up) cannot be used there. The registry is mounted in the `semio-s-artifact-norm-en1997-checks` crate (below the leaf, sees `catalogue::*`, `part_1`, `sls_pairs`).

## Unverified compile points
- Nothing was built. `Case::with_n/with_s` builder chain, `evaluate_outcomes`, `Outcome::{key,value}` and `crate::catalogue::*` through the glob re-export in the checks crate.
- The checks crate currently fails to build (build-en1997/latest.md: `loc` not found in the catalogue-core `catalogue/🦀️.rs:227`); the wiring cannot run before that is fixed.
- NDP values `a-acc.*`, `m1.*`, `r1.*` read through `ndp_common` at test time.
