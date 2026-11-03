# Doc oracles v2: EN 1995 and EN 1999 (fork B)

Nothing compiled, no cargo. Python dry runs (`/home/deck/semio/.venv/bin/python -I`): en1995 31/31 rows ok, en1999 18/18 rows ok (`--self-check` of both oracles green).

## New oracles (`🔮️oracles/🚦️check-outcome/🐍️.py`, `--json` default, `--self-check`)
| code | covered | added | third-party self-check |
|---|---|---|---|
| en1995 | everything of the compliance oracle (6.1.5/6/7, 6.2.3, 6.3.2, 6.3.3 `c.combined`, 7.2, 7.3, 8.2.2, 8 spacing, fire 1-2, floors, bridges); rows with no utilisation dropped (unknown class) | design-case families `en1995.t-nn.<key>.<case>`: T-14, T-18 (+ hole limit), T-30 (both edges), T-31, T-32, T-33, T-45, T-46 | sectionproperties (rectangle A, W), scipy brentq (k_c Ritter root), PyNite (simple-beam deflection) |
| en1999 | the whole compliance oracle (class, 6.2.x, 6.3.1/2/3, 6.7, 8.5/8.6, fatigue, fire, 1-4, 1-5, 7.2) | `6.1.6.haz.<run>` (A-02, status `warning` above 1), `6.3.1.haz.<member>` (A-21), `hazExtent` override in the strips | sectionproperties (I-section A, Iy, Iz exact; J 0.45 %, Iw 0.6 %), scipy brentq (Ayrton-Perry chi), PyNite (deflection) |
Both wrap the existing oracle by import (no copy). Case adapters point at the new oracle; en1999 adapter takes the optional `status` of a row.

## Fixtures (`🧪️tests/🧫️fixtures/🚦️check-outcome/`)
en1995: `design-cases-pass`, `design-cases-fail` (9 design cases, load scale 0.6 / 6), `column-pass`, `column-fail` (compression, 6.3.2, fire). en1999: `weld-run-haz-pass`, `weld-run-haz-warning`, `column-welded-fail`, `weld-transverse-column` (not used: reach = width gives exactly 1.0). Rows in `🚦️check-outcome-en1995-1` (+22) and `-en1999-1` (+8), tolerance 1e-3 (1e-6 for exact ratios), all away from 0.95 to 1.02.

## Mismatches / notes for the Rust owners (details in `📌️important/requests.md`, section "from: doc-oracles v2 (en1995/en1999)")
1. en1999 `6.1.6.haz`: status `warning` at utilisation 1.9355 (weld-1, flange 31 mm).
2. en1999: old oracle ignored `hazExtent` (fixed in the wrapper).
3. en1995: hardwood solid `k_h` = 1.0 in Rust, no hardwood branch in the member oracle (not fixture-covered).
4. en1995: `nKN`, `mKNm` fields are SI (misleading names).
5. Gaps: T-97 rolling shear, EN 1995-2 avert/fatigue document rows; en1999 6.1.4 class etc. stay with the older oracle's independent formulation (stdlib, slice integration), validated by the self-check only for section constants.
No numeric disagreement of a Rust value is known; the Rust could not be run.

## Files
`🪵️en1995` and `🪶️en1999` `…/✳️any/🔮️oracles/🚦️check-outcome/🐍️.py` (new), `…/🔮️oracles/🔣️.json` (rationale), `…/🧪️tests/🚦️check-outcome-enXXXX-1/{🥒️.feature,🐍️.py}`, `…/🧪️tests/🧫️fixtures/🚦️check-outcome/*`.
