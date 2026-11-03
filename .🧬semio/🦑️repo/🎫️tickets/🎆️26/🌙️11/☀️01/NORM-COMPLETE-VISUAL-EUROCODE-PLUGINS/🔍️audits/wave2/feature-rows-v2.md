# Feature rows v2 (wave 2): EN 1998, EN 1999, EN 1990, EN 1991, EN 1995, EN 1996, EN 1997

No cargo/rustc. Python only, `nice -n 19 ionice -c3`, one artifact at a time. en1994 untouched. Scripts in `🧰️scripts/feature-rows/`: `gen1998.py`, `gen1999.py` (fixture generators, `--write`), `spec_families.py` (coverage plan: per family, which pass/fail check is missing and which existing fixture yields it away from the limit band), `append_rows.py` (idempotent), `crosscheck1998.py` (scipy), `rows199*.tsv`, `run_rest.sh`.

## Method
Family = check id with the entity instance segments removed (building, system, storey, member, wall, beam, purlin, sheet, ...). A family is covered when the check-outcome feature has a pass and a fail row. Candidate rows come from every `🧫️fixtures/🚦️check-outcome/*` document run through the artifact's check-outcome oracle; a pass row needs u in [0, 0.9], a fail row u >= 1.1 (near-limit rows are skipped on purpose). Missing statuses without a fixture got a new derived fixture. All new rows use relative tolerance 1e-6 (same Python code path as the oracle, deterministic).

## Before / after (oracle-emitted families with pass AND fail row / emitted families)
| code | before | after | rows before -> after | notes |
|---|---|---|---|---|
| EN 1998 | 8 of 21 families with both (baseShear, joint, nodes, elevationRegularity, joint.gap, rc.rho, pdelta, storeyForces) | 21 of 21 | 31 -> 55 | all families, incl. the new cracked-stiffness (`seismic-cracked-fail`) and joint rows (joint-1 fail, joint-2 pass) |
| EN 1999 | 6 of 58 | 58 of 58 | 23 -> 114 | stiffeners 6.7.6 (pass+fail pre-existing), web patch 6.7.5.web, HAZ 6.1.6 / 6.3.1.haz / 8.6.haz, fire, fatigue, sheet 1-4, shell 1-5, bolts 8.5, welds 8.6 |
| EN 1990 | 33 spec-trace prefixes, 18 pair (v1 scan); oracle families 5 without pair | 4 without pair | 69 -> 84 | still no fixture: `a2.traffic-groups` pass, `annex-b.dsl` / `annex-b.il` fail, `fat` fail, `table-2.1.design-working-life` fail |
| EN 1991 | all families paired by v2 rows (the planned 27 were duplicates of existing `pass-v2-*`) | unchanged | 58 -> 58 | no gap |
| EN 1995 | about 20 families without a fail row | 15 rows added; open: 20 (list below) | 43 -> 58 | |
| EN 1996 | 1 gap | 4 left without a fail fixture | 35 -> 36 | open: `5.4.extent`, `5.5.1.slenderness`, `6.1.3.area`, `8.6.chase-extent` fail |
| EN 1997 | many one-sided families | 25 rows added | 37 -> 62 | residual list not printed (planning run was cut by the load); rerun `spec_families.py en1997 ...` for it |

EN 1995 open (no failing fixture): 6.1.5.c90, 6.1.7.shear, 6.3.2.compression, 7.2.wfin/wfreq/winst/floor-creep, 7.3.f1/stiffness/velocity, 8.spacing.a1/a4t, t-122.wheel_spread (fail) and plate_bearing (pass), t-18.hole_limit/net_section, t-30.taper_angle, t-63.f1, t-97.bending/rolling_shear. They need an overloaded `beam`/`floor`/`panel` variant like the EN 1999 `overload-fail` (generator pattern is in `gen1999.py`).

## New fixtures
- EN 1998 (`🧫️fixtures/🚦️check-outcome/`): `seismic-detail-pass` (q0 3.0, given T1 0.4 s, steel class 1), `seismic-detail-fail` (q0 7, T1 1.5 s, plan 24 x 4, envelope gap 0.08, nonstructural resistance 2 kN, base 0.1 m without tie-down, storey drift 0.06 m, CoS offset -20 m, omega_wd 0.5, steel class 1 on an RC column, steel class 3), `seismic-cracked-fail` (simplified 0.5 stiffness basis, base-shear resistance 200 kN). 20 rows (pass-36..42, fail-43..55): t1Given, plan.envelope, setback, qLimit, drift, steel.class, noSteelClass, slenderness, nonstructural, torsion, overturning, omegaWdBeam, planRegularity, baseShear and componentCombination under cracked stiffness.
- EN 1999: `overload-fail` (loads x30, lateral V_y/M_z/T, thin plate and sheet, short weld, narrow bolt edge/pitch, fatigue 90 MPa, fire 250 C / 30 min), `calm-pass` (loads x0.1, wide gauge, short fire), `bolts-spread-fail` (edge/pitch 0.5 m for e_max/p_max), `web-panel-patch-pass` / `-fail` (patch load 12 kN / 900 kN on the web-stiffener base, utilisations 0.049 / 3.69, status `unverifiedBasis` / `fail`). 90 rows.
- EN 1990/1995/1996/1997: rows only, from existing fixtures (15 / 15 / 1 / 25 appended; the rest of the plan was already present). Six EN 1995 rows carry the oracle status `unverifiedBasis` (6.1.5.c90, 6.1.7.shear, t-14 bending 6.12, t-18 hole limit, t-30 parallel edge and taper angle), as the EN 1999 web rows do.

## Third-party reproduction (EN 1998, scipy)
`crosscheck1998.py`: T1 from the generalised eigenproblem `scipy.linalg.eigh` against the oracle Rayleigh period: ratio 0.99334 on gross and cracked; base-shear utilisation recomputed with the exact period: gross 0.28606 = 0.28606, cracked 0.20651 vs 0.20513 (-0.67 %), cracked-fail 1.23905 vs 1.23080 (-0.67 %). Rayleigh is an upper bound of T1, so a few tenths of a percent lower force is the expected sign; the Rust must use Rayleigh to match the rows at 1e-6. Existing `--self-check` of the EN 1998 and EN 1999 check-outcome oracles (scipy brentq/eigh, sectionproperties, PyNite) unchanged. EN 1999 rows are closed-form clause values of the committed compliance oracle (sectionproperties constants, brentq Ayrton-Perry root, PyNite deflection through its self-check); no new third-party hook.

## Dry run (Python, via the step layer)
EN 1998 55/55 ok, EN 1999 114/114 ok, EN 1990 84/84 ok, EN 1991 58/58 ok, EN 1995 58/58 ok (after the six `unverifiedBasis` statuses), EN 1996 36/36 ok, EN 1997 62/62 ok. These compare the oracle with itself through the step layer; Rust is unrun.

## Oracle vs Rust, by reading (no Rust changes)
1. EN 1998 `qLimit` boundary: Rust asserts `q <= q_max + 1e-9` (`🧭️direction-and-base-shear/🦀️.rs:77`); the oracle reports `q/q_max` which is 1.0000000000000002 for 4.5 x 1.3 = 5.85 (rich fixtures) and so counts as `fail` in a status computed by `u > 1`. The new rows avoid it (0.667 pass, 1.556 fail), but any row on the 5.85 case would disagree; compare in Rust with the same 1e-9 slack or round the utilisation. Request for the shared/oracle owner, not filed as a blocker.
2. EN 1999 ring stiffener: Rust emits `en1999.1-5.stiffener.<ring>` (entity-checks) while the oracle has only the shell families (`1-5.ring/int/buckle/shear`). `overload-fail` includes `ring-1` on `shell-pass`; the Rust id has no oracle row (family missing in the oracle, not a disagreement). Extend the oracle from §6.7.6 when the ring check becomes normative-table backed.
3. EN 1999 HAZ: Rust `6.1.6.haz.<run>` = reach/width with reach = element width (transverse) or 2 b_haz (otherwise); identical to the oracle `haz_rows`. Weld-run geometry refusals (finite, on member) have no oracle id.
4. EN 1998 cracked stiffness: Rust reads `storeyStiffnessBasis`; oracle uses 0.5 for `simplified` (unverified constant) and the declared ratio (clamped to 1) otherwise. Assumed same; the rows `pass-33/35/54/55` pin it.
5. Status vocabulary: Rust must yield `unverifiedBasis` (not `pass`) for EN 1999 6.7.5.web, 6.7.6 web-stiffness/strength and the six EN 1995 ids above when u <= 1; the rows encode that.

## Open
- EN 1990 five and EN 1995 twenty, EN 1996 four families without a failing/passing fixture; EN 1997 residual list.
- Boundary of `spec_families.py`: families whose `have` set is derived from rows `pass-NN` only; v2-style ids (`pass-v2-NN`) are not seen there (so planned rows can duplicate existing ones; `append_rows.py` skips exact duplicates).
- ready for rebuild: feature rows v2 (appended to `build-en1990`, `1995`, `1996`, `1997`, `1998`, `1999` latest.md; en1991 and en1994 unchanged)
