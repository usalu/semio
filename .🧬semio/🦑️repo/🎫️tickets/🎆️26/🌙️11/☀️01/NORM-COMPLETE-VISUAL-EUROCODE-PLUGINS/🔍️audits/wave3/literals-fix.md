# W3-LITERALS-FIX: report

All new rows are `unverified` (no standard text in the repo); each note says the value is the one the artifact hard-coded before. Codegen `--check`, ndp-registry (390 lookups, 873 id literals resolve), ndp-registry-law and remedy-law pass (3802 rows). No Rust compiled (runner does it); "- ready for rebuild: literals" appended for mechanics, en1992, en1993, en1994, en1996. Script: `🧰️ndp/wave3_literals.py`. No fixture or oracle values changed (all values equal the old literals). spec-trace: en1992/en1994/en1995 ok; en1993 and en1996 report a stale twin (not touched by this work). ndp-adoption (`🏚️assessment-state` NDPS), kind-kebab (en1994), handle-paths, path-budget fail for other owners.

## Fixed (literal -> registry row, read in code, cited)
| Audit row | New row(s) | Read in |
|---|---|---|
| 4, 36 EN 1992-4 partial factors | `ndp.en1992.anchor-gamma-ms.{f-uk-max,ratio-max,n.slope,n.min,v.slope,v.min,other}` | `gamma_ms` in en1992 fastenings via `ndp_cell_common`; `SteelPartialFactors::EN_1992_4` const removed from mechanics `fastening` (domain-neutral, caller supplies rows); mechanics test uses explicit values; FA-02/FA-07 basis |
| 1 span/250 (profile) | `ndp.en1992.defl-limit-qp` (250) | members check `check_deflection_profile`, cited in `ndps` |
| 2 l/250, l/500 | `ndp.en1992.defl-limit-qp`, `ndp.en1992.defl-limit-after` (500) | SL-08 (`c.ndp`), entry basis |
| 3 composite slab L/250 | `ndp.en1994.slab.deflection-divisor` (250 both annexes) | en1994 slabs, `.ndp(..)` citation |
| 10 bracing L/500 | `ndp.en1993.bracing.deflection-limit-divisor` | storey bracing check, cited |
| 11 e0 = alpha_m L/500, built-up e0 = L/500 | `ndp.en1993.bracing.imperfection-divisor`, `ndp.en1993.built-up.bow-divisor` | `bracing_qd`, `built_up_member`; cited in storey and built-up checks |
| 6 f_vlt | `ndp.en1996.fvlt.f-b-factor` (EN 0.065), `fvlt.de-k` (0.45), `fvlt.de-share.{g1,g2,g34}` (0.032/0.026/0.020) | `f_vlt_mpa`; M-03 outcome basis |
| 7 cover by exposure | `ndp.en1996.m60.cover.mx1..mx5` | M-60 cover outcome basis |
| 8 DE cot theta 0.48 | `ndp.en1992.vrdcc-k` (DE 0.48) | `cot_theta_max`; entry basis lists |
| 9 nu1 DE 1.1 and 500 | `ndp.en1992.nu2-offset`, `ndp.en1992.nu2-fck-divisor` | `nu1`; entry basis lists |
| 17 theta0 = 1/200 | `ndp.en1992.imperfection-theta0-divisor` | A-03 |

## Verdict on row 8 (possible double count)
No double count. `ndp.en1992.vrdcc-c` = 0.5 is c of the NA-DE V_Rd,cc = c * 0.48 * f_ck^(1/3) * (1 - 1.2 sigma_cd/f_cd) * b_w * z, and 0.48 is the separate formula coefficient; the product is 0.24 and the repo's E-03 worked example (cot theta_max = 2.194 for fck 30, bw 300, z 405, V_Ed 200 kN) only reproduces with it. Added the test `de_cot_theta_counts_the_concrete_shear_term_once` (independent closed form with literal 0.5 * 0.48) and `deflection_and_anchor_limits_come_from_registry_rows` in the en1992 compliance tests. The 0.48 is now a registry row so it is cited.

## Open observations (not changed, values unverified)
- Anchor gamma_Ms "other": the artifact uses 1.5 for both tension and shear outside the validity limits. From memory EN 1992-4 Table 4.1 may give 1.4 for tension; row `other` notes it. Not changed (no spec text).
- `ndp.en1994.deflection-divisor` (beam row) has DE 300 while the slab row stays 250 (unchanged behaviour); the slab DE value needs verification.
- Cantilever factor 2 in the deflection limit and `0.0054` / `5/384` elastic coefficients kept (geometry/formula constants).

## Not done (left as is, with reason)
- Row 5 psi_re,N = 0.5 + h_ef/200: fixed EN 1992-4 formula constant (Low).
- Rows 12-16, 18-35, 37: EN-fixed formula constants and large tables (torsion alpha, C1, buckling alpha, silo, rail, fire kc/ks/ky copies, ψL/φ of the en1994 bridge catalogue, creep tables). Each needs table rows plus spec text; none verifiable in the repo. Next candidates: en1994 `PSI_L_*`/`CREEP_PHI` (15 uses in const contexts), en1996 and en1993 buckling alpha table (row 22), steel fire ky copies (rows 29, 30: read existing `ndp.en1993.1-2.t31.ky.*`).
