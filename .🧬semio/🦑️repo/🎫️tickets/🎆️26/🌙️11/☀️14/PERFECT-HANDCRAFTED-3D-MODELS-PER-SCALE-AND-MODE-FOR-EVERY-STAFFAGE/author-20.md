# Author 20 - objects (retractable-bollard, shopping-trolley, stepladder)

Generators: `author-20-<asset>.py` (with `author-20-lib.py`, copy of the author-17 lib; `author-20-2d.py` edits 2D at the root line by line, `author-20-areafix.py` edits `areaM2` text-level). Final check outputs only in the terminal; nothing under `🗑️generated/author-20` except `bollard.bak*.json` (temporary, delete with the folder).

## retractable-bollard

Spliced. Final check at splice time (full, product engine, all four scales): `PASS (1.4 s)`, every rule green at 1:50, 1:100, 1:200, 1:500. A later run after the tool owner changed `volumes-rulings.ts` (21:04) reads 1:50 `p95M:elevation` 24 mm against 15 mm with asset and source unchanged (see `tool-requests.md`).

| scale | body / thermal / structure |
|---|---|
| 1:50 | 16 / 8 / 11 |
| 1:100 | 12 / 8 / 11 |
| 1:200 | 9 / 8 / 10 |
| 1:500 | 6 / 7 / 9 |

Lists: 1:50 body shaft-tube, reflector, beacon, cap, led (10 lenses on 36 degrees), sleeve-wall (hollow revolve ring 8 mm), sleeve-floor, cover-frame (extrude with 0.145 opening), frame-bolt (4), brush-seal, power-unit (rounded housing with a pocket), motor (sits in the pocket), ram, ram-rod, hose, port-boss. 1:100 drops led/bolt/brush-seal/motor and squares the unit; 1:200/1:500 merge beacon and dome into `cap-head`, hug the sleeve onto the shaft as the drawing does, merge the brush seal into the frame ring.

What I modelled by hand: the shaft stays a filled revolve at every scale (the 2D section draws it filled; the 5 mm tube wall is the thermal layer and the structure member ring); sleeve as a true hollow ring; unit with a motor pocket instead of a buried motor; ram rod ends at the shaft base in the raised pose; hose passes the sleeve wall through a boss so the unit is connected; thermal as open loft skins (shaft above ground, sleeve outside) and a 7.5 mm frame top plate with `sectionFaces` (the frame edges are section faces in the paving); structure with member rings for sleeve/shaft/ram, guide-ring joint, boss joint, mass ellipsoids at the declared centres, a square collar as the `sleeve-fixed` support. Coarse scales: each its own list against its own drawing (sleeve wall thickness follows the drawn stroke, floor and plates at the scale's smallest feature).

Roots fixed in the asset file: sleeve outline `closed: true` in elevation and front at all four scales (see `author-requests.md`); front view 1:50 lost power-unit, motor-seam, motor, hose-port, front view 1:100 lost power-unit, motor-seam, motor (they lie on the section's removed half), with the matching sentences of the representation `decisions` (silhouette, retained, omitted) changed in English and German. `areaM2` (old -> new, all within factor 1.5): shaft-steel 0.41 -> 0.307, beacon-led 0.025 -> 0.0375, cover-frame 0.12 -> 0.093, hydraulic-unit 0.35 -> 0.45 (the modelled thermal surface is the unit housing; the ram barrel in the sleeve is not part of it).

What I saw: opened the sheets of all four scales after the last change. Overlays are green in elevation, front and plan (IoU 0.997/0.985/0.998 at 1:50 down to 0.944/0.938/1.0 at 1:200); the body reads as a shaft in a sleeve with a frame and a side unit; thermal skins sit on the body surfaces; structure rings and blocks are inside the body.

Imperfect: the shaft is solid in body (the 2D draws it so), a real rising bollard is a tube over the ram rod; hollow parts fill in section views (tool, see requests); the coarse sleeve is 15 mm thick and hugs the shaft because the 1:200/1:500 drawings draw it on the shaft edge; the 1:200/1:500 frame is 0.42 x 0.40 m as drawn.

## shopping-trolley

Spliced. Final check (full, product engine, all four scales): `PASS (4.5 s)`.

| scale | body / thermal / structure |
|---|---|
| 1:50 | 106 / 93 / 44 |
| 1:100 | 71 / 61 / 41 |
| 1:200 | 36 / 57 / 41 |
| 1:500 | 27 / 34 / 39 |

What I modelled by hand (generator `author-20-trolley.py`): the wire basket as a tapered frustum of five thin sheets (floor, two side walls, rear and front walls, each leaning as the drawings) with `role: glazing`, the only thin-sheet role, because the mesh is see-through and the 2D draws the basket filled; every wire of the 1:50 basket as a 4.5 mm slender capsule (eight loops, ten verticals per side pair, floor grid, rim), chassis loop and eight fork tubes, crowned tyres with hubs, castor plates, handle posts/arms/grip, the gate as four-tube frame plus mesh sheet with hinge rod and eyes; thermal as references to the real wires/tubes plus open crowned tread skins; structure with rim, corner wires, chassis, handle and gate members, hinge joint, ball and bar joints, four masses and four contact pads at the support anchors. Coarser scales are modelled against their own drawings: fewer and thicker wires at 1:100, thin-only wires kept in thermal at 1:200/1:500, chassis and handle strokes at the scale's pen/feature widths, gate reduced to its mesh sheet (its tubes stay slender in thermal/structure), 1:500 handle straightened onto the drawn centre lines.

Roots fixed: `areaM2` (old -> new, within factor 1.5): gate-wire 0.12 -> 0.147, frame-tube 0.28 -> 0.197, handle-grip 0.04 -> 0.06, wheels 0.08 -> 0.058. The honest grip surface is 0.065 (ratio 1.62 to the declared 0.04, beyond 1.5): listed in `area-review.md`.

What I saw: opened all four sheets after the last change: overlays green except the open handle window and the thin gate stroke at 1:200 (drawn only, IoU 0.96), wire grid, thermal skins and structure members read as a trolley.

Imperfect: the mesh is represented by thin `glazing` sheets (not wires) to satisfy the filled 2D outline; chassis tubes at 1:200/1:500 are as wide as the drawn pens (56/75 mm) and masses of chassis/handle sit on the nearest tube (silhouette containment); the 1:500 thermal uses two long chassis tubes and the grip as representatives of their regions (only the total is compared there).

## stepladder

NOT finished and NOT spliced. `volumes/objects/stepladder.json` holds a draft of all four scales (counts body / thermal / structure: 1:50 23 / 23 / 24, 1:100 20 / 20 / 22, 1:200 15 / 15 / 18, 1:500 13 / 13 / 16) generated by `author-20-stepladder.py` from the previous drafter's 1:50 seed (rails as rounded-rectangle lofts, boxes for feet, treads and platform, capsule spreaders and braces, hinge capsule): coarser scales are the seed with corner radii dropped, knuckles/braces/top cap/hinge merged away and the 1:500 rails widened, thermal as `{ of }` references, structure as member/joint/support/mass solids. The decisions say DRAFT in English and German so nobody splices it by mistake. Not hand-reviewed, no render sheet opened after the last change.

Last check lines (full check, product engine):
- silhouette: elevation IoU 0.82 / 0.80 / 0.71 / 0.74 at 1:50 / 1:100 / 1:200 / 1:500 (the rear rail does not follow the drawn rear rail; rails are drawn 138 mm / 146 mm wide at 1:500), front 0.83 and plan 0.84 at 1:500; 1:50 p95 elevation 22 mm and plan 37 mm; 1:50 bounds depth by 1 mm.
- views-disagree 1:500: whole-asset x 80 mm and z 90 mm between the viewpoints (2D to be judged).
- thermal: 1:50 treads 26 %, rear-frame 23 %, spreaders 24 %, feet +112 % (feet declare 0.02 m2, the four modelled rubber blocks expose about 0.043 m2, ratio 2.1: needs thermal skins), 1:100 feet +165 %, 1:500 total 62 % and one region without a solid.
- structure: masses/joint solids outside the body silhouette by 25-37 mm in elevation at 1:50 to 1:200.
Next steps for the person continuing: fit the rear rails against the drawn rear rail (kit-fit), foot thermal skins, structure containment, decisions per scale, render, splice.

## Summary of changes outside the volumes sources

- `retractable-bollard` asset file: sleeve outline `closed: true` (all scales, elevation and front), power-unit/motor/hose-port shapes removed from the 1:50 and 1:100 front view, three decision sentences changed; `areaM2` of four regions; spliced `volumes`.
- `shopping-trolley` asset file: `areaM2` of four regions; spliced `volumes`.
- `author-requests.md`: section 20 retractable-bollard (section views, retained side); `area-review.md`: two rows.
- Temporary files left in `🗑️generated/author-20` (delete with the ticket folder): `bollard.bak*.json`, `trolley.bak*.json`, `trolley-variant.py`, `stepladder.seed.json` (the stepladder generator reads the seed from there).
