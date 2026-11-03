# 🎯️ Oracle wiring EN 1995

New test module `🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🧪️tests/🎯️oracle-wiring/🦀️.rs`, mounted as `mod oracle_wiring` in `⚖️checks/🦀️.rs` next to `compliance_tests`. The old Python-harness test is untouched.

## Counts
- Cases: 24. Checks: 4 distinct, all wired. Quantities compared: 232 (22 of them known differences). Known differences: 22 (EN 338 rows). Gaps: 3 cases (`en1995.338.strength-class-t14|t22|t30`).

## Wired checks and entry points
| check | evaluator |
|---|---|
| en1995.338.strength-class | `schema::checks::timber::strength_row` (kernel; MPa to Pa rounded to whole Pa, kg/m3 as is) |
| en1995.7.5.natural-frequency | `floor_f1` |
| mechanics.beam-continuous | `semio_s_plugin_norm_mechanics::Analysis::solve_case` (48 elements per span) |
| mechanics.beam-envelope | the same kernel over the oracle load arrangements |

## Known differences (request `oracle -> en1995 norm agent`, status `[answered ...]`, tracked by the `[open] en1995 → NDP/table worklist` line)
22 entries, found by comparing the NDP registry (`ndp.en1995.strength.<class>.<property>`, EN and DE agree) with the cases: C20 rho_mean; C27, C35, C40 f_c,90,k; C45 f_c,90,k and E_0,05; C50 f_t,0,k and f_c,90,k; D24 f_c,90,k, f_v,k, E_0,05, E_90,mean, G_mean; D40 f_c,0,k, f_c,90,k, f_v,k, E_90,mean; D70 f_c,0,k, f_c,90,k, f_v,k, rho_k, rho_mean. Note: D40 E_90,mean (860/870) and D70 rho_mean (1080/960) are not in the 9-row worklist text of requests.md; they are real differences too.

## Gaps
- `en1995.338.strength-class-{t14,t22,t30}`: the registry has no T class rows (EN 338:2016 poplar classes, expected quantities `e_t_0_mean`, `e_t_0_k` also have no field in `StrengthRow`). Suggested owner API: `strength_row("T14")` with a tension modulus field `e_t_0_mean` / `e_t_0_05`, NDP ids `ndp.en1995.strength.t14.*`.

## Files changed
- created `...⚖️checks/🧪️tests/🎯️oracle-wiring/🦀️.rs`
- edited `...⚖️checks/🦀️.rs` (mount), `...⚖️checks/📦️packages/🦀️rust/Cargo.toml` (dev-dependency `compliance` gets `compliance-testing`; new dev-dependency `semio-s-plugin-norm-mechanics`)

## Assumptions / unverified compile points
No build was run. Same kernel sign convention and `Cargo.lock` remark as EN 1994. `strength_row` reads the registry through `ndp_common` (EN and DE agree for all class rows). The hand comparison of the registry against the cases is the basis of the 22 known differences; the driver fails the test when one of them unexpectedly agrees.
