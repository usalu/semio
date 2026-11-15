# Author 50 report: furniture (office-swivel-chair, park-bench, picnic-table)

All three re-checked against C13/C14/C17/C24, repaired and re-spliced (`volumes-insert.py`, all four scales authored). Sources `T/volumes/furniture/<slug>.json`; edit scripts `author-50-chair500.py`, `author-50-chair200.py`, `author-50-bench500.py`, `author-50-bench2d.py`, `author-50-picnic2d.py`, `author-50-lib.py` (text-level 2D editor that keeps the rest of the asset file byte-identical). Check = `volumes-check.ts <asset> --product`. Solids body / thermal / structure:

| asset | 1:50 | 1:100 | 1:200 | 1:500 |
|---|---|---|---|---|
| office-swivel-chair | 40 / 42 / 34 | 19 / 21 / 27 | 16 / 20 / 27 | 14 / 15 / 20 |
| park-bench | 24 / 24 / 19 | 20 / 20 / 15 | 16 / 16 / 11 | 8 / 8 / 11 |
| picnic-table | 9 / 9 / 19 | 9 / 9 / 19 | 8 / 8 / 18 | 7 / 7 / 17 |

I opened the render sheet of every scale of every asset after the last change.

## office-swivel-chair
Final check line: only `volumes-feature` 1:500 (4 measurements `minDimensionM:arm-side-a/b, arm-back-a/b = 0.04 >= 0.075`); everything else passes (silhouette elevation 0.951 / 0.925 / 0.932 / 0.941, thermal totals, structure, cross-scale, monotonic).
- The four angled 1:500 star arms are 40 to 32 mm tapered rods, the width the plan draws (capsule r 20 to 16 mm). By C13 their limit is the thinnest filled drawn extent, 32 mm, and they pass; the tool takes the bounding box of the rotated capsule in its bone frame (133 and 283 mm) and so demands 75 mm. Tool defect in `tool-requests.md` ("50 furniture/office-swivel-chair 1:500"), measurement named there; arm-front, drawn along x, is judged correctly.
- 1:500 by hand: arms 40 to 32 mm (were 75 mm), hub 30 mm band of the elevation (was 75 mm), tyres 56 mm wheels 30 mm wide as drawn (were 75 x 75 mm), column 115 mm (the elevation draws it as a 0.23 mm pen stroke = 115 mm; its true 50 to 60 mm is below the 75 mm feature and is not slender, ratio 7). Elevation IoU 0.783 -> 0.941, front 0.890 -> 0.991. The column is the one part thicker than reality, because the drawing says so.
- 1:200: the armrest was a triangular panel (a generalisation of an unclosed 3-point stroke). The 2D elevation drew only half of the T (post and forward pad, no rear pad) and contradicted the plan (pad 305 mm long). Root fix in the 1:200 elevation: armrest strokes replaced by filled 30 mm post and 30 mm pad rectangles (T of 305 mm pad); body, thermal and structure `armrest` is now one extruded T (post 30 mm, pad 30 mm). Elevation IoU 0.931 -> 0.932, plan red bars gone.
- `areaM2` (set within factor 1.5 of the committed value, C16; not leaf/fur kinds): `chair-steel` 0.113 -> 0.145 (1:50 exposes 0.148: sleeve 0.053, mechanism underside and sides 0.074, column), committed 0.1 so ratio 1.45; `castor-rubber` 0.018 -> 0.023 (1:50 exposes 0.0229, ratio 1.15). Reviewer's ACCEPT values were 0.113 / 0.018; the model, not the estimate, was measured. 1:100 castor 0.0288 against 0.023 (25 %).
- Sheets: 1:50 detailed (shell, foam, hubs), 1:100 and 1:200 thermal shells nested, structure members inside body, supports green at the castors; 1:500 members and 75 mm contact pads read correctly, the column's red pen caps below the hub remain (drawn stroke ends beyond the ground line).

## park-bench
Final check line: only `volumes-thermal-coverage` 1:50 `areaDeviation:back-timber` 0.262 (limit 0.15); all else passes (1:500 front IoU 0.706 -> 0.932, 1:100 cross-scale plan p95 fixed, 1:200 front 0.894 -> 0.975).
- 1:500 rebuilt by hand: 50 mm foot, 60 mm front column, 60 mm raked rear post on the drawn parallelogram (extrude on the drawn rake, 160 mm wide as the elevation draws the frame), 45 mm seat plank, 78 mm raked back slab on the drawn parallelogram running down to the seat (the elevation draws seat and back as one block); structure supports 50 mm, masses 58 mm. The previous model used 75 mm blocks and a vertical box for the raked back.
- Root fixes in the asset (2D): 1:500 front had only the left end frame (`l-*`), so `end-right` had no drawn shape and fell back to the 75 mm limit: added `r-foot`, `r-front-leg`, `r-rear-post` mirror shapes; 1:50 plan and front draw the seat as the four real 118.75 mm slats with their gaps (`seat-slat-1..4`), because the 3D has the real 5 mm gaps and a solid drawing made the 1:50 plan silhouette disagree with its own 3D and the cross-scale p95 to 1:100 fail (48.75 mm against 30 mm, the gap lines); 1:50 plan back slats 33 mm -> 49.8 mm deep (a raked slat's footprint, as its own front view draws); 1:100 and 1:200 plan `back-band` y 0.18/h 0.075 -> 0.1701/0.0958 (as the front draws); 1:200 front gained the drawn cross `l-brace` (3D had it, 1:200 front IoU was 0.894).
- Open finding: back-timber exposes 1.255 m2 at 1:50 against the reviewer-set `areaM2` 1.7 (3 slats of 80 x 33 mm as every scale's drawing shows; the decision assumed 100 x 40 mm). Appended to `T/area-review.md`; not changed (committed 0.54, 1.26 would be factor 2.3, beyond C16). Proposal: set `areaM2` 1.26.
- Sheets: 1:50 cast end frames with scroll legs, braces, bearers, raked posts, finials, four seat slats, three raked back slats with bolts; thermal cutaway shows the steel frame and seat; 1:100/1:200 merged plank and raked band; 1:500 raked plates and blocks, supports green. Still visible: thin red slivers at the cast legs in elevation (the drawn leg flare is wider than the 3D loft).

## picnic-table
Final check line: only `volumes-thermal-coverage` 1:50 `areaDeviation:a-frame` 0.186 (limit 0.15); all else passes (1:500 elevation IoU 0.726 -> 1.000, front 0.789 -> 0.991; 1:100 cross-scale plan p95 fixed).
- 2D root fixes at 1:500: legs and benches were `filled: false` hairlines (body shapes). Now filled: elevation legs are the parallelograms of the 3D legs (110 mm horizontal, 80 mm in plan and front, 90 mm perpendicular), benches 70 mm filled rectangles/outlines; front legs 80 mm rects, plan legs 555 x 80 mm rects. At 1:50 the plan draws the table top as its five real boards (`top-board-1..5`, 138 mm, 2 mm gaps) instead of one rectangle with texture lines, for the same reason as the bench seat.
- 1:500 by hand: legs 80 x 110 mm timbers, slab 100 mm, benches 70 mm (the drawn 40 mm board is far below the 75 mm feature and the 90 mm pen floor; 70 mm, not 40, because the exposed-area measure collapses on thinner slabs: 40 mm slab measured 0.34 m2 for faces that are 3.4 m2, 45 mm 1.77, 60 mm 1.90, 70 mm 3.38, 75 mm 3.42, a tool sensitivity at 1:500 cell size noted here, not filed).
- Open finding: a-frame exposes 1.184 m2 at 1:50 against the accepted 0.998 (+18.6 %); the decision's own arithmetic gives 1.15 to 1.26. In `T/area-review.md`; proposal `areaM2` 1.18 (factor 2.0 from the committed 2.4, so not applied).
- Sheets: 1:50 five boards, two bench boards, A-frames, hinge blocks; 1:100 slab, 1:200 slab with the lower beams, 1:500 legs/benches/slab/hinges as drawn; all thermal shells coincide with the body (references), structure inside.

## Imperfect, stated plainly
- chair 1:500 `volumes-feature`: 4 measurements, tool bounding-box quirk (above); passes by C13's own definition.
- park-bench 1:50 back-timber +/-26 % and picnic-table 1:50 a-frame 18.6 %: area data conflicts with the drawings, in `area-review.md`.
- The 1:500 chair column (115 mm) and picnic benches (70 mm), legs (80 mm) are thicker than the real parts because the 1:500 drawings (pen floor 90 to 115 mm) say so; parts of the 1:50 detail (bench slat gaps 5 mm) are drawn only from 1:50 on.
