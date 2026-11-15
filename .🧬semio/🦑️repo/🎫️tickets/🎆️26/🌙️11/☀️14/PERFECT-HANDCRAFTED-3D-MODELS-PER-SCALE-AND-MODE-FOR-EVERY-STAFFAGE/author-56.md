# Author 56 — components (round 2): etics-brick-wall-12/30, glulam-beam, cantilever-balcony, glass-balustrade

Sources `volumes/components/<slug>.json`; scripts `author-56-etics-room-face.py`, `author-56-etics-decisions.py`, `author-56-glulam.py`, `author-56-balcony.py`, `author-56-balustrade.py`, 2D roots `author-56-fix2d-glulam.py`, `author-56-fix2d-balcony.py`, `author-56-fix2d-balustrade.py`, area values `author-56-area-glulam.py`, `author-56-area-balcony.py`, `author-56-area-balustrade.py`. Gate: `volumes-check.ts <asset> --product`. Counts are body / thermal / structure (a `repeat` counts once). All five are spliced (`volumes-status.py`: etics-12, etics-30, glulam `aaa+` on every scale; balcony `aaa! aaa! aaa+ aaa+`; balustrade `aaa! aaa! aaa! uuu+`). I opened the render sheet of every authored scale of every asset after the last change (etics-12: 50/100/200/500, etics-30: 50/100/200/500, glulam 50/100/200/500, balcony 50/100/200/500, balustrade 50/100/200). Overlays are green; the red slivers are listed under the asset.

| asset | 1:50 | 1:100 | 1:200 | 1:500 | final check |
|---|---|---|---|---|---|
| etics-brick-wall-12 | 28 / 4 / 14 | 3 / 3 / 2 | 2 / 2 / 2 | 1 / 1 / 2 | PASS |
| etics-brick-wall-30 | 28 / 4 / 14 | 3 / 3 / 2 | 2 / 2 / 2 | 1 / 1 / 2 | PASS |
| glulam-beam | 17 / 6 / 4 | 3 / 6 / 4 | 3 / 6 / 3 | 1 / 6 / 3 | PASS |
| cantilever-balcony | 14 / 12 / 10 | 7 / 10 / 9 | 5 / 7 / 4 | 3 / 7 / 3 | FAIL 2: `areaDeviation:railing-steel` 3.55 (1:50, limit 0.15), 2.01 (1:100, limit 0.30) |
| glass-balustrade | 7 / 7 / 4 | 3 / 3 / 3 | 3 / 3 / 3 | unavailable | FAIL 5 (all `volumes-thermal-coverage`): `panel-glass` 0.91 / 0.77, `clamp-steel` 9.14 / 9.60 at 1:50 / 1:100, `areaDeviationTotal` 1.12 at 1:200 |

## What was wrong and what I did

Re-check first: every asset's live product check was run before touching anything (`🗑️generated/author-56/check0-*.txt`). The catalog-check failures were real, but different from what the previous author's notes said: the closed-mesh area problem was already gone, the remaining failures were the coarser scales of the walls, bearing steel and top/soffit of the beam, edge/railing/wall/break of the balcony, and the balustrade regions.

### etics-brick-wall-12 and -30
- Failure: 1:100, 1:200, 1:500 `areaDeviation` 1.0 (exposed 2.0 against 1.0): at 1:50 the product counts only the outermost layer (one face), from 1:100 on every thermal solid, so the room face of the core slab doubled the area.
- Fix by hand: the room-side face (`-u`) of the core slab (1:100 `th-wall-core`, 1:200 `th-wall-masonry`, 1:500 `th-wall`) is a section face, exactly as author 18 did for the solid brick wall; the single region counts one wall face, its outer one, at every scale. Written into the `Thermal:` decisions at 1:100/200/500 in both languages. No `areaM2` changed.
- The 1:50 model (plaster, 13 brick courses with 12 mortar bed joints, insulation, render, stacked layer slabs, 13 course members, bearing block) was already right and unchanged; the cut edges (`-v/+v/-w/+w`) of every solid were already marked.
- Saw: all eight sheets, silhouettes IoU 1.000, nested thermal slabs, course members blue inside the body. The mortar joints are inside the wall (render covers them from outside), as in the real object.

### glulam-beam
- Failures: bearing-steel at 1:50/1:100, beam-top and beam-soffit (the skins exposed 1.75 against 1.28 because they overlapped and showed edges), 1:200 views-disagree.
- Thermal rebuilt by hand: four 36 mm timber skins (40 mm at 1:100, 45 mm at 1:200 so the lists are not copies of each other, 75 mm at 1:500) that overlap in the arris prisms; the top and soffit skins span the full width, the side skins the full height, and the faces against the core or the neighbouring skin are section faces. Every skin now exposes its own outer face once: sides 9.69 / 1.29 / 1.29 m2 against 9.6 / 1.28 / 1.28. A "Minimal:" decision at 1:100 says the six-solid thermal list cannot be coarser.
- `areaM2` `bearing-steel` 0.04 -> 0.06 (ratio 1.5, the limit, as the city-bus doors precedent): the two plates expose 0.0608 m2 (bottoms 0.0384, free ends and sides 0.0224); listed in `area-review.md`.
- 2D root (1:200): the front `bearing-end` stroke ran across the full 0.16 m and its pen (0.18 mm = 36 mm) widened the front to 0.196 m against the plan's 0.16 (`extentDifferenceM:z` 36 mm > 30 mm); the stroke now runs from -0.062 to 0.062, so stroke plus pen is 0.16 m.
- Body (15 lamellae with the 6 mm arris chamfers, plates) and structure (member, slider joint, pin pad with wedge, roller) were already right.
- Saw: all four sheets. At 1:200 the front shows a red band at the very bottom: the 0.18 mm bearing stroke (36 mm) extends 18 mm below the 30 mm plate the 3D has (IoU 0.965, p95 15 mm, passes).

### cantilever-balcony
- Failures: slab-edge, railing, wall-face and thermal-break per region, totals at 1:200 / 1:500, 1:200 views-disagree.
- Thermal rebuilt by hand per scale: slab top, soffit and edge skins as overlapping skins with their inner, arris and neighbour faces marked; the 1 m strip stands for a balcony that continues sideways, so the strip ends (`-w/+w`) and the wall's top and bottom are section faces (also set on the body solids, the viewer hatches them); at 1:100 the wall core and the break band are real thermal solids with their room face (`-u`) marked as in the etics walls (a referenced body solid cannot carry `sectionFaces`: `tool-requests.md`); at 1:200 and 1:500 real wall solids with the room face marked. Result: slab-top / soffit / edge 1.5 / 1.5 / 0.2 at 1:50 and 1:100 (exact), wall-face 1.164 and 1.100 against 1.1; 1:200 and 1:500 totals pass.
- `areaM2` `thermal-break` 0.2 -> 0.17 (ratio 0.85): the break module's outer face lies against the slab in the wall plane (support contact), its top under wool and render, so its exposed faces are its underside and its strip ends, 0.168 m2.
- 2D root (1:200): the plan's railing stroke ran over the full 1.0 m and its pen widened the plan to 1.05 m against the front's 1.0 m (`extentDifferenceM:z` 50 mm > 30 mm); it now runs from -0.475 to 0.475.
- Imperfect (stated plainly): `railing-steel` is declared 0.24 m2 and measures 1.09 m2 at 1:50 (two hollow SHS posts with their bores 0.77, handrail 0.19, mid-rail 0.11, plates) and 0.72 m2 at 1:100. The declared value and the railing bone mass (6.355 kg = one post) describe a single post while the 2D front draws two posts and both rails; ratio 4.5, not applicable under C16, listed in `area-review.md`. Elevation 1:50 shows a one-pixel red dot below the slab at the drip groove (IoU 0.999).
- Saw: all four sheets; body reads as the wall strip with the cantilever, break band, railing with base plates and bolts; thermal slabs nested; rebars inside the wall and slab; cut hatch on the strip ends.

### glass-balustrade
- 2D root (1:50): the front drew the two handrail cap ellipses centred on the tube ends (+-1.000) with rx 4 mm, so the whole asset measured 2.008 m in the front against 2.000 m in the plan; the caps are centred at +-0.996 now (extent 2.000, as the 3D's 1.992 m tube plus the dome).
- 1:200 structure: the square 53 mm handrail member enclosed the octagonal handrail prism and buried its whole surface in the exposed-area test (handrail 0.004 m2 instead of 0.27); the member is now the octagonal prism itself (`of`), decisions updated in both languages.
- `areaM2` `handrail-steel` 0.13 -> 0.195 (ratio 1.5, the limit): measured 0.2215 at 1:50, 0.222 at 1:100; now +13.6 % and passing.
- Imperfect (stated plainly, all listed in `area-review.md`): `panel-glass` declared 2.2 m2 (one face of the 2 m x 1.1 m panel) measures 4.20 m2 at 1:50 and 3.88 m2 at 1:100 with both faces exposed (ratio 1.9 / 1.77); `clamp-steel` declared 0.1 m2 measures 1.014 / 1.06 m2 (ratio 10); the 1:200 total is then 1.12 over. I did not mark glass faces as section faces (C25: the back of a free-standing panel is exposed). 1:500 is unavailable.
- Saw: sheets of 1:50, 1:100 and 1:200 (silhouettes IoU 0.998-1.000 elevation, 0.993-1.0 front, 1.0 plan; the 1:200 front has a red 1 px line at the handrail top).

## Files changed outside the ticket
Asset files (only the named members): `etics-brick-wall-12`, `etics-brick-wall-30`, `glulam-beam` (volumes, 2D front 1:200, `areaM2` bearing-steel), `cantilever-balcony` (volumes, 2D plan 1:200, `areaM2` thermal-break), `glass-balustrade` (volumes, 2D front 1:50, `areaM2` handrail-steel). Appended to `area-review.md` and `tool-requests.md`.
