# Doc oracles v2 (wave 2): check-outcome oracles for EN 1990, 1991, 1995, 1996, 1997, 1998, 1999

Nothing compiled or run on the Rust side (no cargo, no builds, no git writes). Python side with `/home/deck/semio/.venv/bin/python -I`: every `--self-check` green; dry run of each case (`🗑️generated/doc-oracles/dry-run.py`) green: en1990 57/57, en1991 42/42, en1995 31/31, en1996 18/18, en1998 28/28; en1997 30/30, en1999 18/18.

## Structure
Each artifact has a new `🔮️oracles/🚦️check-outcome/🐍️.py` (`--json` default stdout `{"checks":[{"id","utilization"}]}`, `--self-check`) that wraps the artifact's older document oracle (`⚖️compliance/🐍️.py`) for what it covers and adds the missing check families of the code's spec-trace `check` rows (en1996 and en1997 have only `design-case` rows: families taken from the ids the evaluators emit). The case `🧪️tests/🚦️check-outcome-enXXXX-1/🐍️.py` now runs the new oracle; the Rust adapters are unchanged. Hand-built passing and failing documents sit in `🧪️tests/🧫️fixtures/🚦️check-outcome/<name>/🔣️.json` (`shared://🚦️check-outcome/<name>/🔣️.json`); manifests: one sentence on `enXXXX-1-compliance-independent` each (`🧰️scripts/doc-oracles-v2-manifest.py`).

| code | sub-report | third-party in `--self-check` | fixtures | rows |
|---|---|---|---|---|
| en1990 | `doc-oracles-v2-en1990-en1991.md` | scipy.stats (FORM, k_n), anastruct, PyNite | 4 | 57 |
| en1991 | same | scipy.integrate (impact ODE), scipy.constants | 2 | 42 |
| en1995 | `doc-oracles-v2-en1995-en1999.md` | sectionproperties, scipy, PyNite | 4 | 31 |
| en1996 | `doc-oracles-v2-en1996-en1997.md` | sectionproperties, PyNite, scipy | 2 | 18 |
| en1997 | same | groundhog (N_q, Vesic N_gamma, Newmark stresses) | 2 | 30 |
| en1998 | `doc-oracles-v2-en1998.md` | scipy.linalg.eigh (shear-building modes), scipy.optimize.brentq (spectrum) | 2 | 28 |
| en1999 | `doc-oracles-v2-en1995-en1999.md` | sectionproperties, scipy, PyNite | 4 | 18 |

## Mismatches against the Rust (found by reading, numbers in `📌️important/requests.md`)
- en1998: accidental-torsion delta computed and dropped (storey shears 1.3x low: 0.1574 against 0.2046); `planDims` utilisation Lx/Ly = 1.5 with forced Pass; `elevationRegularity` only emitted when plan dimensions are given; `planRegularity` max e0 against min r instead of per direction. The older en1998 oracle (EN branch S 1.2/TC 0.4, no gamma_I) was wrong and is replaced.
- en1990/en1991: fire load ignores the occupancy table (450 against 548 MJ/m^2); delta_q1 interpolated in ln A (1.34 against 1.23 at 100 m^2); construction machinery/concreting 3.0 kN/m^2 not in EN 1991-1-6 Table 4.1; fatigue, DSL/IL and working-life checks return Warning above 1 (no failing rows possible); `a14.deflection` may use the sampled maximum.
- en1995/en1999: en1999 `6.1.6.haz` warning at 1.9355 instead of fail; hardwood k_h 1.0 without oracle branch; action fields `nKN`/`mKNm` are SI despite their names (old en1999 oracle ignored `hazExtent`: fixed in the wrapper).
- en1996: `6.2.shear` ignores `inPlaneShearN`/`inPlaneMomentNm` (mutations do not move the row, 0.0931); Phi = 0 gives utilisation 6.6e14 to 7.4e14.
- en1997: `9.earthPressure` and category-1 bearing have no utilisation.
No numeric disagreement of a check value is known: the Rust could not be run. After the rebuild a failing row names the oracle number in the feature.

## Not covered
Per sub-report: en1990 A2 bridge/traffic groups and informative Annex C rows; en1991 parametric fire, wind dynamics, bridge/crane/silo; en1995 T-97 rolling shear and EN 1995-2 rows; en1996 fire, chase/opening rules, reinforced masonry; en1997 bishop3d and category-1 bearing; en1998 ground class, time history, diaphragms, masonry.simple, bridges/silos/tanks/towers; en1999 weld-transverse near-limit row.

"ready for rebuild: doc oracles v2" appended to `🗑️generated/build-en1990..en1999/latest.md`.
