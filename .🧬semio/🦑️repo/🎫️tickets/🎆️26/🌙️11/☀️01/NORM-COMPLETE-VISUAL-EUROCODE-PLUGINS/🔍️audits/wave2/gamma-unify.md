# GAMMA-UNIFY (wave 2): one source for partial factors on actions and combination factors

Scope: gamma_G,sup/inf, gamma_Q, gamma_P, xi, psi0/1/2 and K_FI in en1990-en1999 and the shared crates, plus the material partial factors of the four named codes. No cargo/rustc was run; nothing was compiled. Announcements and the `ready for rebuild` lines are in `📌️important/requests.md` ("gamma-unify").

## 1. Shared API

`⚖️compliance/📜️annex-and-tables/⚖️action-factors/🦀️.rs` (mounted by `📜️annex-and-tables/🦀️.rs`, re-exported as `semio_s_plugin_norm_compliance::{ActionFactors, ReliabilityClass}` and `::action_factors::{Domain, FactorSet, PsiRow, gamma_g, gamma_q, xi, gamma_p, canonical_category, psi_row_of_key, resolve_psi_category}`):

- `ReliabilityClass { RC1, RC2, RC3 }` (moved from the en1990 snapshot, same derives and DSL keys; the snapshot now `pub use`s it; `Default` = RC2, `k_fi(annex)` = registry `ndp.en1990.kfi.<n>`).
- `ActionFactors::new(annex, reliability_class, situation)` (buildings, Set B) with `.domain(Domain)`, `.set(FactorSet)`, `.in_situation(DesignSituation)`, `.serviceability()`, `.k_fi_override(f64)` and the readers `gamma_g_sup()` (K_FI x gG_sup), `gamma_g_inf()`, `xi()`, `gamma_g_sup_610b()`, `gamma_q(category)` / `gamma_q_building()` (K_FI x gQ, bridge families road/rail/other), `gamma_p(relieving)`, `psi(category)` / `psi_known(category)` / `psi_snow(altitude)`, `k_fi()`, `exceptional()`. Accidental, seismic and serviceability contexts return 1.0 for the partial factors (EN 1990 6.4.3.3/4, 6.5.3), exactly what the en1990 generator does.
- Every number is a registry read (`ndp_number_opt`, so the checks still cite the rows they read). Python twin for the oracles: `⚖️action-factors/🐍️.py` (`ndp(row, annex)`, `ActionFactors`, no dataclass so it loads with `spec_from_file_location`).
- en1990 keeps its public names: `ndp-tables` re-exports the moved items; `PsiEntry::row` and `k_fi_for_rc` read through them; `Context`/`combinations_dir` already used the same rows.
- Test (language-agnostic data + Rust): `action_factors_follow_the_registry_class_situation_and_domain` in `⚖️compliance/🧪️tests/🔬️unit/🦀️.rs` (K_FI scaling per class and annex, 1.0 in accidental/seismic/serviceability, bridge families, psi, unknown category = (1,1,1)). Third-party cross-check of the same numbers: the en1990 python oracles now build their tables from the same rows and are compared with the Rust generator by the existing oracle tests.

Reliability class in the documents: only en1990 has the field. For en1992/1994/1995/1996 it defaults to RC2 (K_FI = 1.0) through one hook per artifact (en1992 `En1992Snapshot::action_factors()`, en1995 `AnnexParams::with_reliability_class`, en1994 `part_en1990::factors(annex)`, en1996 `action_factors(annex, exceptional)`). The schema field itself (every layer, ~50 files per artifact plus all fixtures) was not added blind without a compiler; routed as `[open]`.

## 2. Inventory before and after

| Artifact / file | Before | After |
|---|---|---|
| en1992 `🧮️design-effects` | local `(1.35, 1.0, 1.5, 0.85)`, local psi table (`PsiCategory` match) | `combine_member_actions(member, &factors)`, `psi_key` + `ActionFactors::psi` |
| en1992 `⚓️anchor-evaluation` | `gamma_g = 1.35`, `gamma_q = 1.5`, local psi | factors from the call, same psi path |
| en1992 `🎬️scene/🔬️mechanics` | `gamma(kind)` 1.35/1.5, `1.35 g + 1.5 q + 1.05 rest` | `design_effects(member, &factors)`: gamma_G,sup, gamma_Q, accompanying gamma_Q psi0 of the category (1.05 = 1.5 x 0.7 only for offices) |
| en1992 effect profile, crack state | psi via local table | `Profile::quasi_permanent(member, &factors)`, `crack_state(sec, member, &factors)` |
| en1992 `🔥️fire-design` R-06, `✂️shear` | defaults 1.35/1.5, `GAMMA_S = 1.15` | `ndp.en1990.gG_sup/gQ.str.A1`, `ndp.en1992.gamma-s` |
| en1994 `part_en1990` | `GAMMA_G_EN/DE`, `GAMMA_Q_EN/DE`, local psi (kind/category match, `construction` = (1,1,0)) | `factors(annex)`, `gamma_g/gamma_q(annex)`, `psi_factors(kind, category)` over the registry (`construction_load` row (1,0,1)) |
| en1994 `📈️diagrams::shear_shape` | `1.35`/`1.5` | `part_en1990::gamma_g/gamma_q(f.annex)` |
| en1994 `cat::constants` | `GAMMA_AP = 1.0`, `GAMMA_VS = 1.25`, `GAMMA_S_FAT = 1.15` | `gamma_ap()`, `gamma_vs()` (new rows `ndp.en1994.gamma-ap/-vs`), `gamma_s_fat()` (`ndp.en1992.gamma-s-fat`) via `ndp_common` |
| en1995 `🧮️load-combinations` | `ULS_XI = 0.85`, `1.35`/`1.5` in member and connection combos, local psi | `AnnexParams.factors`; `enumerate_combos(m, &params)`, `enumerate_connection_combos(c, &params)`; the `_with` variant is removed |
| en1995 `psi_factors` | local table incl. permanent (1,1,1) / accidental (0,0,0) | `psi_factors(&factors, kind, category)` over the registry (permanent/accidental (1,1,1)) |
| en1995 bridge member | `m_ed = 1.5 * m_crowd` | Annex A2 `gamma_q("footbridge_crowd")` = 1.35 |
| en1995 gamma_M | 11 x `gamma_m_for(..).unwrap_or(1.3 / 1.25)` | `unwrap_or(f64::NAN)` (all rows registered; a missing row fails the registry test instead of hiding) |
| en1996 `🔥️fire-and-design-effects` | `GAMMA_G_SUP/INF`, `GAMMA_Q`, `gamma_q(bool)`, `PSI0_SNOW`, `PSI0_WIND` | `action_factors(annex, exceptional)`, `gamma_q(annex, exceptional)`, `design_effects(annex, ..)` |
| en1996 `🎬️scene/🧮️profile` | second copy of the combination with its own `1.35/1.0/1.5` | maps `m::design_effects` (one combination) |
| en1996 tables | `psi_imposed(cat)` A-H literals, `PSI0_SNOW/PSI1_SNOW/PSI0_WIND`, `GAMMA_S = 1.15`, `F_YD_B500_PA` | `psi_imposed(annex, cat)` (registry), `gamma_s(annex)`, `f_yd_b500_pa(annex)` (`ndp.en1992.gamma-s`) |
| en1991 fire catalogue E12.07 | defaults `1.35`/`1.5` | `ndp.en1990.gG_sup/gQ.str.A1` of the case annex |
| en1990 | own definitions of Domain/FactorSet/gamma_g/.../PsiRow/canonical_category, snapshot enum | re-exports of the shared module |
| python oracles | tables and constants in en1990 (2), en1992 (3), en1994 (3), en1995 (1), en1996 (2) | all read `ndp.*` rows through `⚖️action-factors/🐍️.py` / `FACTORS.ndp` |

Left untouched on purpose (not factors, or owner trees blocked): en1993 `gamma_mf(method)` 1.0/1.15/1.35 and `LowConsequence`, en1993 `phi = 1.35` / `load_factor = 1.35` (snapshot demo constructors), en1998 `_ => 1.35`, en1995 Johansen `1.05`, en1995 fire `kfi` 1.05/1.15 (EN 1995-1-2 k_fi, not K_FI), demo/example inputs (`Demo { gamma_g: 1.35 }`, `InputSpec ... .example(1.35)`), en1994 `PSI_L_*` (creep multipliers), en1997 sets (own partial-factor sets), en1993/en1999 Rust psi/gamma reads (already registry rows through `ndp_common` and their own key maps).

## 3. What changes numerically

At the default RC2 and unchanged psi rows, verdicts are identical except where a local copy disagreed with the registry:

1. en1995 bridge-role members (glulam footbridge assets): crowd gamma_Q 1.5 -> 1.35 (EN 1990 Table A2.4(B), footbridge family). Rust and python oracle changed together; blessed outputs of those two assets (scene golden, reports) must be regenerated at the next build.
2. en1995 imposed categories F, G, T, Z and an empty category: registry rows (F (0.7,0.7,0.6), G (0.7,0.5,0.3), T/Z follow A) instead of the office default; permanent/accidental unchanged.
3. en1992 fallback `mechanics::design_effects`: accompanying actions gamma_Q psi0(category) instead of the fixed 1.05; only the illustrative picture when the report has no station.
4. en1994 construction-stage psi: (1,1,0) -> registry `construction_load` (1,0,1); ULS and characteristic use psi0 = 1.0 (unchanged); frequent/quasi-permanent of the construction stage are not used by a check.
5. en1992 python test oracle: congregation/shopping/storage psi now match the Rust values (its local table was wrong).
6. K_FI of RC1/RC3 will scale gamma_G,sup and gamma_Q in every artifact once the document carries the class (hooks above); gamma_G,inf and xi are not scaled (EN 1990 B3.3).

Pre-existing divergence found and not changed: en1996 `concentrated(..)` multiplies the concentrated load by the persistent gamma_G,sup in every situation, the check-outcome oracle uses 1.0 in accidental/seismic load cases; the oracle now reads the situation-aware factor.

## 4. Material partial factors (gamma_M, gamma_c, gamma_s, gamma_M0...)

- en1992: `AnnexParams` already reads `ndp.en1992.m-01.*` and `gamma-s*`; the only literal left was the scene shear `GAMMA_S`, fixed.
- en1994: `AnnexParams` reads `ndp.en1994.*` / `ndp.en1993.gamma-m*`; literals `GAMMA_AP`, `GAMMA_VS`, `GAMMA_S_FAT` moved to the registry (two rows added; `codegen.py --check` ok, 3019 entries).
- en1995: `gamma_m_for` reads `ndp.en1995.gamma-m.*`; the literal fallbacks were the only copies, replaced.
- en1996: `gamma_m` rows were registry reads; the `GAMMA_S` / `F_YD_B500_PA` consts now come from `ndp.en1992.gamma-s`.
- Open: en1993 `gamma_mf` per assessment method; python oracles of en1994 `check-outcome` and `🏛️columns` (`GAMMA = dict(v=1.25, c=1.5, s=1.15, m0=1.0)`), the en1992 catalogue oracle `1.5`/`1.15` inside formulas, the en1999 oracle constants.

## 5. Open items and blockers

- BLOCKER (en1994 owner): `part_en1990` in `🧬️schema/💡️inferences/⚖️checks/🦀️.rs` is structurally broken in the working tree (see the requests entry): `accumulate_column` runs into the beam `match mode`; `uls_column`, `fire_column` and the header of `accumulate` are missing. The staged index version has them. Not repaired here because the owner was editing the span; en1994 cannot build until it is.
- Schema field `reliabilityClass` for en1992/1994/1995/1996 (every layer, fixtures, mutation leaf) and threading it through en1994 (35 call sites with `annex`) and en1996 (about 20 functions with `annex`).
- Registry rows for en1993 gamma_Mf per method.
- Nothing was compiled: the likeliest compile risks are the glob-import name resolution of the new module in `⚖️compliance/🧱️core`, the `AnnexChoice` import in the en1996 `🧮️profile` (added), and the en1994 repair above.
