# Author 57 — round 2: shower-head, toaster, tower-server, vacuum-cleaner (+ room-thermostat, laptop checked)

Scripts: `author-57-shower.py` (hose resampling, thermal/structure fixes), `author-57-toaster.py`, `author-57-each.ts`/`author-57-each.py` (each structure solid alone), `author-57-burial.ts` (what buries a region: removes each solid in turn), `author-57-evalsh.py`, `author-57-setarea.py`; root edits logged in `author-57-dimfix.log`. Final check = `volumes-check.ts <asset> --product`. All four assets spliced with `--allow-incomplete` (the committed catalog already carried the unfinished previous version; the new one is strictly better). Counts body/thermal/structure.

| asset | 1:50 | 1:100 | 1:200 | 1:500 | what still fails (live validator) |
|---|---|---|---|---|---|
| shower-head | 46/47/11 | 38/39/9 | 15/17/8 | unavailable | 1:200 elevation IoU 0.881 (limit 0.90); everything else green |
| toaster | 12/11/8 | 9/10/7 | 4/7/7 | unavailable | thermal areas: top-plate, control, feet at 1:50/1:100 (area-review) |
| tower-server | 14/13/8 | 10/12/7 | 2/11/6 | unavailable | feet area 0.0062 / 0.0081 against 0.002 (area-review); all else green |
| vacuum-cleaner | 41/22/37 | 25/22/25 | 16/20/20 | 16/20/20 | brush area 1:50/1:100; 1:500 total 0.347 (limit 0.30) |
| room-thermostat | unchanged | unchanged | - | - | dial 1:100 0.78 (declared 0.00243 vs the 1:100 drawing's o48 x 20 mm dial exposing 0.0043) |
| laptop | unchanged | unchanged | unchanged | - | `underside` (validator contact-plane tolerance, not fixed yet: exposed 0.0025 / 0.0004 against 0.068) |

## shower-head (rest of the work was in the hose and in the area burial)
- Hose: the previous 12 capsules sampled every Bezier four times (sagitta up to 3 mm on a 12 mm stroke): the elevation IoU was 0.865. Resampled at the 2D's own parametrisation (x, y from the elevation Bezier, z from the plan polyline at the same parameter): 36 capsules at 1:50 (elevation 0.979, front 0.976, plan 0.956), 30 + one slender straight piece at 1:100 (0.943 / 0.936 / 0.945), 8 + slender climb pieces at 1:200.
- Slender hose pieces (C24): only the first 0.233 m of the climb is straight enough (3 mm deviation) and long enough (ratio >= 20) at the true 12 mm; the loop stays at the scale's feature (15 / 30 mm). Tool request in `tool-requests.md` (57, slender along the path).
- 1:50 thermal: `rail` now carries the holder block and `valve` the lever and its boss (both are 2D regions of those parts); hose thermal are references of the body capsules (area 0.0533 against 0.047); the structure `mass-shower` ellipsoid (20 mm, bigger than the 16.5 mm handle) buried the handle (exposed 0.0038) and poked 9 mm out of the body: now a 12 mm sphere at the mass centre; `joint-lever` was 5.6 / 7.2 mm outside the body: now a flat 18 x 18 x 8 mm lens inside the lever boss.
- 1:100: bracket skins are references (rail 0.059), handle/mass fixed. 1:200: loop thermal are true 12.6 mm capsules inside the 30 mm body hose (buried by C1: honest true-size thermal, the exposed total then is 8 % off); slender climb pieces expose their true surface.
- Root edit: `dimensions.widthM` 0.23 -> 0.238 (the 2D plan and elevation both draw the hose to x = 0.2316 + 6 mm; the dimension was the centre line).
- Left imperfect: 1:200 elevation IoU 0.881 (hose 30 mm against 20 mm drawn; with a 20 mm hose it would be 0.915). Handle area is only within tolerance because the holder block buries 0.0098 m2 of it.
- Saw: 1:50 hose follows the drawn loop, holder/lever/valve nested, structure inside; 1:100 and 1:200 look like the same object (rail, brackets, handle, square-faced head, loop).

## toaster
- Root cause of bread 0.011 / control 0.002: the structure list filled the cavity: a solid 0.26 x 0.14 x 0.12 box member and a 0.1 x 0.06 x 0.05 mass ellipsoid buried the bread slices; the emitter box at the slots overlapped both slices. Now the base frame is a hollow rectangular ring (10 mm walls, around the cavity), the mass a flat box under the slices, the emitter a box between the slices, `joint-slices` moved inside the carriage (it stuck out 10.5 mm).
- Missing bottom pan added (`pan`, 7.5 / 15 / 30 mm sheet, region casing): casing exposed 0.167 -> 0.255 against 0.24 at 1:50 (was 0.30 off); the thermal list lost `th-foot-r` (only half of the feet) and the `lever`, `cancel-button` of region `control`: added.
- 1:100: slices 22 mm thick as drawn (were 13 mm: `volumes-feature` and plan overlay), pan under the 20 mm feet.
- Area applied: bread 0.045 -> 0.054 (1.2, both faces of two slices plus rim). Findings in `area-review.md` (57): feet (0.0065 vs 0.00326), control (0.0049 vs 0.003), top-plate underside (C25).
- Saw: 1:50, 1:100, 1:200 sheets: silhouettes 0.99 / 0.96-0.99 / 0.97-0.99, shells nested, ring frame around the slices, pan visible in the cutaway.

## tower-server
- Side-panel 0.271 vs 0.38: no model defect in the panels; a 10 mm thermal CPU grille box stood in the panel plane and the product's centroid test buried half the panel's face. Grille now a 3 mm texture plate 0.4 mm outside the panel: side-panel 0.3618 (dev 0.05). Tool note in `tool-requests.md`.
- Feet: the previous lists covered one of the four feet to match 0.002; all four are in the thermal list now (1:200 four boxes). Honest areas exceed the declared value (area-review).
- Saw: all three sheets (silhouette IoU 0.99-1.00, panels, grilles, hollow frame, mass ellipsoid inside).

## vacuum-cleaner
- Wheels 0.016 -> 0.0515 (declared 0.05): the thermal tyre skins were smaller than the body wheels (buried); now references of the rear wheels and the two casters.
- 1:200/1:500: wand is a true 35 mm slender capsule at 1:200 (ratio 23, thermal true skin 0.0182); 1:500 kept at 75 mm (slender made elevation 0.86). 1:200 hose thermal skins 0.6 mm outside the body capsules (were buried: exposed 0).
- Areas applied (all within 1.5): body-shell 0.42 -> 0.31, wand 0.09 -> 0.066, hose 0.14 -> 0.12. Findings: brush (no honest solid near 0.01), 1:500 total.
- Saw: all four sheets: silhouettes 0.985/0.981/0.979, 0.994/0.985/0.981, 0.956/0.984/0.982, 0.904/0.990/0.996; hose ribs, dished tyres, structure inside the canister.

## room-thermostat, laptop
Not changed. Room-thermostat 1:100: the dial is a o48 x 20 mm solid because the 1:100 2D draws it so; the declared 0.00243 is the o42 x 6 mm real dial (finding in `area-review.md`). Laptop: the validator has not changed (contact-plane tolerance); the underside line still fails with the numbers above (`author-requests.md` 12, `tool-requests.md` 57).

## Root edits (old -> new)
shower-head `dimensions.widthM` 0.23 -> 0.238; toaster bread 0.045 -> 0.054; vacuum-cleaner body-shell 0.42 -> 0.31, wand 0.09 -> 0.066, hose 0.14 -> 0.12.
