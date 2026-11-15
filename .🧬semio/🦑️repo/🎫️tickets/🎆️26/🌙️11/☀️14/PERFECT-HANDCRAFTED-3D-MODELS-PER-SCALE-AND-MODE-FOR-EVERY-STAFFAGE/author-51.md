# Author 51 — furniture round 2 (bar-stool, bookshelf, cafe-chair, folding-deck-chair, dining-table, height-adjustable-desk)

Generators: the author-15 scripts `author_15_<asset>.py`, edited (they write `volumes/furniture/<slug>.json`); 2D root repairs by `author-51-*-2d*.py` (through `author_51_edit.py`, which compares JSON instead of text because the number formatting of the deck chair file had changed). Gate: `volumes-check.ts <asset> --product` after the last change, every scale's render sheet opened after the last change, all six spliced with `volumes-insert.py`. Live area decisions (C25, `area-decisions.md`) were applied: no `sectionFaces` on free-air faces any more.

## Check lines and counts (body / thermal / structure per scale)

| asset | 1:50 | 1:100 | 1:200 | 1:500 | final check |
|---|---|---|---|---|---|
| bar-stool | 7 / 9 / 16 | 6 / 6 / 16 | 5 / 5 / 14 | 4 / 5 / 12 | PASS |
| bookshelf | 26 / 26 / 39 | 16 / 16 / 24 | 16 / 16 / 24 | 16 / 16 / 24 | PASS |
| cafe-chair | 27 / 27 / 21 | 12 / 12 / 20 | 11 / 11 / 19 | 10 / 10 / 18 | PASS |
| folding-deck-chair | 9 / 7 / 22 | 9 / 7 / 22 | 5 / 7 / 18 | 5 / 7 / 18 | PASS |
| height-adjustable-desk | 33 / 34 / 22 | 22 / 22 / 22 | 17 / 17 / 21 | 14 / 14 / 19 | PASS |
| dining-table | 14 / 14 / 18 | 8 / 8 / 14 | 8 / 8 / 12 | 5 / 8 / 10 | 1 line: 1:50 `areaDeviation:table-top` 0.229 (limit 0.15), see below |

## bar-stool
- Re-check found: seat-plate thermal area (the hidden core counted both faces), backrest area, 1:200/1:500 silhouette, 1:100 monotonic.
- Root causes: the hidden plywood core exposed both faces (0.214 m2 against 0.124): its upper face is glued to the pad, so it now carries `sectionFaces: ["+u"]` (legitimate under C25, listed there). The 1:200/1:500 2D legs ran to the outer extent of the tube (0.21) instead of the centre line (0.1975, as 1:100), and carried a private `penMm`; centre lines corrected, `penMm` dropped so the scale pen of the solid (C24) and of the drawing agree. Legs are now `slender` true 25 mm tubes at 1:200/1:500 (were 30 / 75 mm).
- The footrest ring stays feature thick (30 mm / 75 mm): a closed ring cannot be `slender` (ratio 1, `tool-requests.md`). Stated plainly: the one thickened member.
- Backrest (`areaM2` 0.06 restored by the reviewer, "model defect"): the cushion was 0.1 m wide and exposed 0.021 m2. Now a real wrap cushion, 0.2 m long on the 0.14 m seat radius (nine-section superelliptic loft, 184 mm chord, 90 mm high, 40 mm thick, top still 0.87): 0.052 m2. The 2D back-post / back-pad / back shapes of all four scales were redrawn to the wider cushion (root repair; the back is inside the seat circle, so the plan is unchanged).
- 1:100/1:200/1:500 structure and thermal lists rebuilt for the monotonic count (5-section cushion loft at 1:100, boxes at 1:200/1:500).
- Seen: all four sheets. Overlays green; the nested foam/leatherette shells, plywood core and wrap cushion read at 1:50/1:100; slender legs are drawn true size in the sheet (red outline = the pen of the drawing), the validator widens them.
- areaM2: none changed.

## bookshelf
- Re-check: only `volumes-monotonic` 1:100 (structure). Cause: the five duplicate shelf members at 1:100 were references to the rounded-corner boards (588 triangles each at the fixed quality) against 96 per stud at 1:50. Now one plain full-width board per shelf pair (12 triangles). Nothing else changed; front view is the A-A section (C12/C23), IoU 0.997-0.998 at every scale.
- Seen: all four sheets (carcass, shelves, studs at 1:50, plinth and supports).

## cafe-chair
- Re-check: 1:100/1:200/1:500 silhouette, 1:500 thermal total. Cause: tubes drawn as pen strokes against 28/30/75 mm solid tubes. Legs, stretchers and back posts are now `slender` true size (20 mm legs and posts, the mean of the 19 to 26 mm taper; 14 mm stretchers, role `detail` at 1:100/1:200 so the pen role matches the 2D stroke). Note: at 25 mm and below only (length / diameter >= 20), so 20 mm is the largest constant diameter that passes `volumes-slender` for the 0.42 m legs.
- 2D roots repaired: the front-view leg strokes ended at the outer extent (0.25) instead of the centre line (0.2395) at 1:100/1:200/1:500; the plan legs/posts were filled capsules while the other views are strokes (the whole-asset width disagreed by 25 mm): plan legs and posts are strokes now. 1:500 had no back panel in elevation and front and no stretchers in elevation and plan: added (copied from 1:200 with the 1:500 pen).
- Thermal 1:500: total 1.05 against 0.92 m2 (+14 %).
- Seen: all four sheets.

## folding-deck-chair
- Sagging sling: the two flat canvas boxes are now extruded curved bands (7.5 mm at 1:50, 15 mm at 1:100): a parabolic belly of 8 mm (back) and 12 mm (seat) off the drawn straight lines, mitred on the bisector at the crease so no notch opens, six / twelve segments. 1:200/1:500 stay flat 30 / 75 mm panels (the belly is below the feature; the canvas is drawn as a stroke there).
- Re-check found 1:200/1:500 silhouette, views-disagree, feature (shoe 60 mm), thermal total. Roots: the coarse elevation drew the 45 mm bars as open pen strokes (the whole-asset length disagreed by exactly one pen between elevation and the extruded front/plan): the three bars are filled 45 mm outlines in elevation, front and plan at 1:200/1:500, so C13 lets the solids keep the true 45 mm (were 75 mm at 1:500). 1:500 elevation had no seat-canvas stroke: added. Foot shoes 45 / 75 mm (were 36 / 60). Thermal rails: 25 mm top dowel `slender` at 1:200, 45 mm blocks at 1:500 (`volumes-copied`/monotonic).
- Imperfect: 1:50 elevation IoU 0.905 (limit 0.90): the thin sling triangle against the drawn strokes.
- Seen: all four sheets (sag visible in the 1:50/1:100 axonometric).

## height-adjustable-desk
- Cause of the top-board 0.76 m2 (against 1.28; now 2.68 by C25): `exposedArea` probes each mesh face once at its centroid. The top is two 0.64 m2 triangles; their centroids (1.067, -0.133) and (0.533, 0.133) m lie under the laptop and the monitor base, so both big faces count as buried and the whole top face is lost (`tool-requests.md`, section 51). Fix in the model, no area touched: the outer shell of the board is a real thermal solid cut into strips (`repeat`: 16 at 1:50, 8, 4, 2 at the coarser scales, differing per scale for `volumes-copied`); at 1:50 nested over the board core. Result 2.60 m2 at 1:50 and 1:100, 2.68 at 1:200, 2.04 at 1:500 (total 0.27 off).
- Laptop lid (declared 0.2 m2 = both faces): the laptop sat flat on the board and exposed 0.111. It now stands on four rubber feet (8 mm at 1:50, 15 mm at 1:100 = the scale features; drawn raised accordingly), 1 mm clear at 1:200/1:500: 0.196-0.203 m2. Monitor base rests directly (monitor-housing 0.55, +10 %).
- Coarse scales are true size, not feature size (C13): the 2D at 1:200/1:500 was open pen strokes for plates, top, sleeves, posts (whole-asset x and z disagreed by one pen, silhouette 0.65-0.77); they are filled outlines at the true 16 mm plates, 25 mm top, 64/54/44 mm posts, 50 mm rail, 40 mm monitor, 20 mm laptop and the solids follow (were 75 mm). The monitor-neck and plates of 1:100 are filled outlines too. 1:100 posts are the hollow nested tubes again (steel-desk 1.89 against 2.15, was 1.40 with solid posts); plates without rounding (the rounded boxes were what broke the structure triangle count).
- Seen: all four sheets.

## dining-table
- Re-check: only 1:500 (silhouette 0.8988, thermal total 0.655). 1:500 2D legs were open 90 mm strokes (the stroke caps run 45 mm below the floor): filled 60 mm outlines (the mean of the 45-70 mm taper), solids 60 mm. Thermal at 1:500 counted the merged slab twice (table-top and apron both referenced the whole slab): the slab surface is counted once under table-top, apron keeps coverage as a buried 30 mm ring. C25: `sectionFaces: ["-v"]` removed from all table-top references (free-air underside).
- Left: 1:50 `table-top` exposes 2.31 m2 against the reviewer's 3.0 (-23 %, limit 15 %): the leaf (0.6 x 0.88 m) lies directly under the plates in pose CLOSED and covers 0.53 m2 of the underside; written to `author-requests.md` (section 51), model not changed to hit it. 1:100 (2.31, limit 30 %) and 1:200/1:500 pass.
- Seen: all four sheets.

## Tool / rule observations
- `tool-requests.md` 51: face-wise burial test (the desk) and `volumes-slender` for closed rings.
- Leftover files: scripts `author-51-*.py`, `author_51_edit.py`, edited `author_15_*.py` generators; no generated output kept.
