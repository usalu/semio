# Author 17 — components (9 assets)

Sources: `volumes/components/<slug>.json`, generators `author-17-<asset>.py` (shared `author-17-lib.py`), spliced with `author-17-splice.sh`. All nine are spliced; none of them passes the full check yet (see "Not clean"). Solid counts are body / thermal / structure (expanded `repeat` not counted).

| asset | 1:50 | 1:100 | 1:200 | 1:500 |
|---|---|---|---|---|
| glulam-beam | 17 / 6 / 4 | 3 / 6 / 4 | 3 / 6 / 3 | 1 / 6 / 3 |
| etics-brick-wall-12 | 28 / 4 / 14 | 3 / 3 / 2 | 2 / 2 / 2 | 1 / 1 / 2 |
| etics-brick-wall-30 | 28 / 4 / 14 | 3 / 3 / 2 | 2 / 2 / 2 | 1 / 1 / 2 |
| cantilever-balcony | 14 / 12 / 10 | 7 / 10 / 9 | 5 / 7 / 4 | 3 / 7 / 3 |
| glass-balustrade | 7 / 7 / 4 | 3 / 3 / 3 | 3 / 3 / 3 | unavailable |
| masonry-wall-with-lintel | 16 / 16 / 8 | 8 / 8 / 7 | 3 / 3 / 6 | 3 / 3 / 6 |
| curtain-wall-bay | 21 / 18 / 13 | 6 / 7 / 9 | 3 / 7 / 5 | 3 / 5 / 5 |
| panel-radiator | 15 / 15 / 5 | 8 / 13 / 5 | 6 / 12 / 3 | unavailable |
| external-venetian-blind | 32 / 33 / 23 | 24 / 24 / 23 | 24 / 24 / 6 | 24 / 24 / 6 |

## What was modelled by hand
- **glulam-beam**: 15 lamellae with the outer arris chamfers, plates; 1:100/200 merged beam, 1:500 one block. Supports by type (pin = pad with wedge, roller = cylinder), thermal skins at the 36 mm storage depth.
- **etics walls**: 13 brick courses with 10 mm mortar joints, layers per scale as in the 2D (4 / 3 / 2 / 1), stacked layer slabs in thermal, `sectionFaces` on the edges, leaf-only member.
- **cantilever-balcony**: slab as extrusion with drip groove, split wall core, break module, SHS posts (hollow), plates/bolts, 12 mm rebars as structure members, fixed block at the wall.
- **glass-balustrade**: U-shoe extrusion, gaskets, 5 cross bolts, laminated glass, handrail with domed caps; no 1:500 (unavailable).
- **masonry-wall-with-lintel**: four-layer cavity wall as extrusions with window and recess hole, 4-layer lintel and sill, closing back pieces; contact pads at the three foundation anchors.
- **curtain-wall-bay**: mullions/transoms in three layers, IGU as panes + spacer bars, spandrel layers, IGU zones as nested shells (centre/edge ring), frame merged to one extrusion at 1:200.
- **panel-radiator**: pressed panels, zig-zag convector fins (texture role), caps, grille, brackets, valve (knob on its own bone), emitters as tagged discs.
- **external-venetian-blind**: 17 lens lamellae on their own bones (the old model had none), roller box as hollow extrusion with slot, C guide rails, wall layers, IGU; coarser scales use thin sheets/bars.

## What I saw in the renders
Every scale of every asset was rendered; I looked at the sheets of balcony (1:50/100/500), balustrade (50/100), etics-12 (50), radiator (50), curtain wall (50), blind (500), masonry (500), glulam (50/500). Shapes read right (joints visible, nested thermal layers, members blue inside body). Red overlay areas remain where the drawing is a section (below).

## Not clean (stated plainly)
- `volumes-thermal-coverage` areaDeviation fails for every asset at every scale: the tool still measures closed-mesh area; C1 (exposed area, `sectionFaces`) is only in the etics walls so far. I did not yet correct any `areaM2` at the root (no asset value changed). Re-check and re-splice after the tool shows C1.
- Section views (masonry all scales, curtain wall 1:50/100 front+plan 86 mm, radiator 1:50 elevation knob, blind 1:500 elevation IoU 0.56): see `author-requests.md`.
- `volumes-views-disagree` (2D defects) for blind, balustrade 1:50, radiator: not repaired.
- curtain-wall-bay 1:200 thermal count/triangles equal to 1:100 (monotonic).
- Slender parts (`slender: true`), `sectionFaces` on balcony/curtain/glulam/masonry/blind: not applied.
- Blind: slide bones merged into slat joints; radiator brackets/valve positions follow the elevation where the 2D conflicts (valve centre 85 vs 70 mm: front used).
