# 🚦️ Doc oracles v2 (wave 2): check-outcome for EN 1996 and EN 1997

Nothing compiled or run on the Rust side (no cargo). Python side with `/home/deck/semio/.venv/bin/python -I`: `--self-check` green for both oracles, dry run of both cases (`🗑️generated/doc-oracles/dry-run.py`) green (see the end).

## Trace situation
The spec-trace of en1996 and en1997 has no `check` rows (37 and 42 `design-case` rows), so the families come from the check ids the evaluators emit (`format!("en1996.…")`, `format!("en1997.…")`) and from the existing document oracles (`⚖️compliance/🐍️.py`).

## What exists now
| artifact | oracle `🔮️oracles/🚦️check-outcome/🐍️.py` | families | fixtures | case rows |
|---|---|---|---|---|
| en1996 | wraps `⚖️compliance/🐍️.py` snapshot mode (compression, mid-height, slenderness, simplified as worst row over the load cases) and adds per load case `6.1.2.eccentricity`, `6.2.shear`, `6.3.flexure`, `6.1.3.concentrated`, `6.1.3.area` | 9 (4 old, 5 new) | `masonry-rich-pass`, `masonry-rich-fail` | 7 old + 6 pass + 5 fail |
| en1997 | wraps `⚖️compliance/🐍️.py` `evaluate` (stdlib library `📚️geo.py`): bearing 6.5, sliding 6.5.3, eccentricity 6.5.4, settlement and elastic 6.6, pair angular, rotation and tilt, piles 7.6, retaining wall sliding and overturning 9, slope Bishop 11, uplift and hydraulic 10, anchors anc-01/02/04, embedded walls rw-07/12, pile groups pile-06, investigation 2, gi-12/17, hyd-01 | about 30 | `geotechnical-rich-pass`, `geotechnical-rich-fail` | 11 old + 10 pass + 9 fail |

The case adapters (`🧪️tests/🚦️check-outcome-enXXXX-1/🐍️.py`) now run the new oracle with `--json` (en1996 used the old snapshot mode with a key map, en1997 the compliance oracle). The Rust adapters are unchanged. Manifest rationale of `enXXXX-1-compliance-independent` extended with `🧰️scripts/doc-oracles-v2-manifest.py`.

## Third-party validation (`--self-check`)
- en1996: wall section modulus `l t^2 / 6` against `sectionproperties` (rectangle, ratio 1.000000000000008); the midspan wind moment `w h^2 / 8` against PyNite (ratio 1.0); the concentrated-load factor cap 1.5 against `scipy.optimize.minimize_scalar`.
- en1997: `N_q` of the Annex D library against `groundhog` `nq_frictionangle_sand` (1e-9 at 22, 30, 35, 40 deg); `N_gamma` EN `2 (N_q - 1) tan phi` against groundhog's Vesic factor (difference exactly `4 tan phi`); the Boussinesq stress under the centre of a rectangle against groundhog's Newmark corner solution `stresses_rectangle` superposed (1e-3). The plotting packages groundhog imports at module level (plotly, jinja2, …) are not installed and are stubbed with `unittest.mock`; nothing is drawn. The pair-settlement plane fit already uses `numpy.linalg.lstsq` and the Boussinesq kernel `scipy.integrate.dblquad` in the old oracle.

## Rows
Fixtures are hand-built from committed mutation snapshots (`🧰️scripts/doc-oracles-v2-en1996-fixtures.py`, `…en1997-fixtures.py`). All rows are outside 0.93 to 1.05. Values are in the two `🥒️.feature` files. Examples: en1996 `6.3.flexure.wall-concentrated.uls-snow` 0.5389 (pass), `6.2.shear.wall-slender.uls-snow` 107.89 (fail: l_c collapses under earth pressure 90 kN); en1997 `6.6.settlement.footing-F1` 1.2069 and `6.6.angular.el.footing-F1.footing-F2` 1.9457 (fail), `11.bishop.slope-S1` 0.7193 (pass, 20 deg) and 1.5596 (fail, 38 deg).

## Mismatches and findings (owners in `📌️important/requests.md`, section "from: doc-oracles v2 (en1996/en1997)")
1. en1996 `6.2.shear`: `inPlaneShearN` and `inPlaneMomentNm` never enter the document shear check, only the catalogue shear-wall cases; mutations `change-in-plane-shear` and `-moment` do not move the row (0.0931).
2. en1996 `6.1.2.compression`: a Phi of 0 gives utilisation 6.6e14 to 7.4e14 (floor 1e-9 N).
3. en1997 `9.earthPressure` and category-1 bearing have no utilisation (status only).
4. Oracle side (fixed in the wrapper, not an owner issue): none new; the en1997 library had no third-party validation, now has groundhog.
No numeric disagreement of a check value is known, because the Rust could not be run. A failing row after the rebuild names the oracle number in the feature; the oracle runs on the fixture with `python -I 🔮️oracles/🚦️check-outcome/🐍️.py <snapshot>`.

## Not covered
en1996: fire `1-2.fire`, chase and opening rules, reinforced `6.6.reinforced`, material and durability (status rows), `m-31/m-32/m-95` catalogue document cases (the catalogue case oracle covers those). en1997: `bishop3d`, tolerant catalogue GI/GEO document cases of the design-case rows, category 1 bearing.

## Files
- `🌍️en1997|🪨️en1996/…/✳️any/🔮️oracles/🚦️check-outcome/🐍️.py` (two new), `…/🔮️oracles/🔣️.json` (two edited)
- `…/🧪️tests/🧫️fixtures/🚦️check-outcome/{masonry-rich-pass,masonry-rich-fail,geotechnical-rich-pass,geotechnical-rich-fail}/🔣️.json` (four new)
- `…/🧪️tests/🚦️check-outcome-en1996-1|en1997-1/{🥒️.feature,🐍️.py}` (four edited)
- `🧰️scripts/doc-oracles-v2-en1996-fixtures.py`, `🧰️scripts/doc-oracles-v2-en1997-fixtures.py`, `🧰️scripts/doc-oracles-v2-manifest.py` (shared with the other forks), `📌️important/requests.md`, `🗑️generated/build-en1996|1997/latest.md` (appended)
