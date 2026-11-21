# Round 3 — objects/tealight

State: DONE

No earlier round-3 work existed for this asset; started at step 1. The catalog `volumes` at the start equalled `volumes/objects/tealight.json` (including the digital-only 1:500 another agent's pass had added); all four scales are replaced by the model below.

Scripts (inputs, kept): `r3-tealight-build.py` (every solid, 2D shape and root value stated by hand; modes `volumes`, `candidate`, `catalog`), `r3-tealight-suit.ts` (print-suitability probe: the product's `validateRanges` and `measureSuitability`), `r3-tealight-zoom.ts` (magnifies the sub-millimetre 2D drawings for looking).

## 1. Reference

| quantity | value | source |
|---|---|---|
| overall size of a standard tealight | 38 mm diameter, 16 mm high | en.wikipedia.org/wiki/Tealight (web search 2026-11-20); asset summary |
| cup sizes on the market | 36 to 39 mm diameter, 15 to 18 mm high | candle-making suppliers found by the same search (36 x 16.4, 38 x 15, 37.5 x 20, 39 x 18 mm) |
| cup construction | deep-drawn aluminium foil about 0.1 mm, cut rim, draft of a few degrees, base radius about 1 mm, embossed floor with a standing ring | my knowledge of the product; the asset notes give 0.1 mm |
| cup base diameter | 36 mm (rim 38 mm, 3.6 degrees of draft over 15 mm) | my knowledge; no source in the search states both diameters, so this is an estimate (the old 31 mm base means 12 degrees of draft, which no drawn cup has) |
| wax | pressed paraffin puck, 12.8 g, 14 mm high, loose in the cup, wick bore and recess for the sustainer | asset notes (12.8 g, 4 h); check: a 14 mm puck of 35.6 to 37.2 mm is 14.5 cm3, 13.1 g at 900 kg/m3 |
| wick | braided cotton about 2 mm, in a tin-plate sustainer (disc about 12 mm, crimp neck about 3 mm), about 7 mm above the wax | my knowledge; suppliers list the separate wick with sustainer; no source for the 12 mm |
| flame | 22 mm high, 8 mm wide at a third of the height, rounded foot, pointed tip; the wick reaches into its lower part | asset notes (22 x 8 mm, drysdale-2011, quintiere-2006); form from my knowledge of a laminar candle flame |
| mass | 14 g (cup about 1 g, wax 12.8 g, wick and sustainer 0.2 g) | asset |

## 2. Audit against the reference

- Rig: all six bones lay horizontal (`angleDeg` 0, x to the right) although every part stands on the vertical axis: `r3-sheet` showed six bones sticking out sideways, and the 2D structural view colours members along the bones. Joint heights were right (wax top 14.5 mm, flame foot 18.5 mm, tongue 26.5 mm).
- Wax bone at 1.4 mm with 13 mm length: a 1.3 mm void under the puck that no cup has.
- The flame lift bone had a length of 12.5 mm (a copy of flame-body), so it counted as a structural member.
- Unlit pose: the flame parts were "hidden" by lowering them, but the flame body ended 1.5 mm above the wax and the tongue went 1 mm through the floor.
- Cup: base 31 mm (12 degrees of draft); base-contact footprint 31 mm.
- Wick 5 mm above the wax with the flame foot at 4 mm: the flame sat on the last millimetre of the wick.
- 2D: the flame was drawn 11.6 mm wide at 1:50, 14.4 mm at 1:100 and 23.5 mm wide at 1:200, the cup 33 to 43 mm at 1:200, the wax level as a domed line that reached the rim (it protruded over the cup). All views were symmetric and agreed with each other.
- Old solids: a cup revolve with a rim radius of 19.5 mm and a 2 mm floor step, a domed wax, a wick with an 8 mm "tab", a flame as a bulb with a needle on top (tongue 2.8 mm wide), thermal emitter as a 20 mm disc floating above the flame (against D2), structure entirely `{ of }` references to the body plus a mass ellipsoid (against D3); 1:200 and 1:500 structure with a 33 mm mass ellipsoid around a 38 mm candle; all revolves 12-sided in the renders.
- HEAD version of the catalog file: two print findings at the end of the 1:200 band (1:350: cup 0.176 mm, flame 0.182 mm, limit 0.2 mm).

## 3. Root fixes (catalog asset, written by `r3-tealight-build.py catalog`)

| member | old | new |
|---|---|---|
| `rig` cup | angle 0 | angle 90 (every bone now runs up the axis; children at angle 0, offsets along x) |
| `rig` wax | y 0.0014, length 0.013, com (0, 0.0065) | x 0.0005, length 0.014, com (0.007, 0) |
| `rig` wick | y 0.0131, length 0.005, com (0, 0.0025) | x 0.014, length 0.007, com (-0.011, 0) (sustainer on the floor) |
| `rig` cup mass | com (0, 0.007) | com (0.007, 0) |
| `rig` flame (lift) | y 0.004, length 0.0125, axis 90, -0.0165..0.003 | x 0.004, length 0, axis 0, -0.01775..0.003 |
| `rig` flame-body | length 0.0125 | 0.0135 |
| `rig` flame-tip | y 0.008, axis 90, -0.011..0.006 | x 0.008, axis 0, -0.006..0.006 |
| anchor base-contact | footprint 0.031 x 0.031 | 0.034 x 0.034 (standing ring), title |
| anchor flame-top | (0, 0.0135) | (0.014, 0): the real tip |
| pose unlit | flame dy -0.0165, tip dy -0.011 | flame dx -0.01775, tip dx -0.00575 (both parts fully inside puck and wick) |
| pose flicker-peak, clip flicker key | dy | dx (same values) |
| `thermal.regions` cup `areaM2` | 0.005 | 0.0056 (hand calculation for the 36/38 mm cup: outside wall 1750 + base radius 110 + underside 915 + inside 2790 mm2; measured on the model 0.00557; factor 1.12) |
| `thermal.regions` wick | `areaM2` 3e-05, length 0.005 | 4.7e-05, 0.007 (7 mm of 2 mm braid) |
| `structure.assumptions` | contact 31 x 31 mm | annular contact 28 to 34 mm |
| `provenance` | — | reference `tealight-trade-sizes`; notes rewritten (cup 36/38, puck, sustainer, wick 7 mm, bones up the axis) |
| representations 50, 100, 200 | see audit | every shape redrawn as the true view of that scale's body (same ids, bones, roles, z, tones, regions): cup 36 to 38 mm, flame 8 x 22 mm in two parts (one at 1:200), wick stroke 7 mm, wax level a straight line at 14.5 mm that stops 4 mm short of the wall, plan discs 38 / 35.6 / 8 mm; decisions `silhouette`, `retained`, `omitted`, `thinElements` (and `styles` at 1:200) rewritten |
| representation 200 `pens.body` | 0.13 | 0.18 (true-size drawing prints over the whole band 1:150 to 1:350; removes the two old 1:350 findings too) |

`dimensions` are unchanged (38 x 38 x 40.5 mm, 14 g). Print suitability after the fix (office-laser, `r3-tealight-suit.ts` on the catalog file): no `validateRanges` finding; smallest feature 0.290 / 0.246 / 0.219 mm at 1:50 / 1:100 / 1:200.

## 4. The model

Solid counts body / thermal / structure: 1:50 5 / 7 / 9, 1:100 4 / 6 / 6, 1:200 3 / 5 / 6, 1:500 3 / 5 / 6. All revolves about the candle axis, profiles stated station by station.

- 1:50 body: `cup` (one 0.1 mm foil shell: embossed floor with the centre panel 0.35 mm up inside the standing ring 28 to 34 mm, 1 mm base radius in five stations, wall 36 to 38 mm, cut rim), `wax` (puck 14 mm, 0.15 mm loose, chamfers, sustainer recess, wick bore), `wick` (2 mm braid from the floor to 7 mm above the wax with the 12 mm sustainer disc and 3 mm crimp neck, rounded tip), `flame` (14 stations, 8 mm at a third), `flame-tongue` (14 stations; slides inside the bulb so that lit it is one teardrop and stretched it stays closed).
- 1:50 thermal: cup shell (layer foil), wax (top face only; the faces against cup and wick are section faces, C25 b), wick in the air, wick root with sustainer as internal (faces against the wax, no area), flame, tongue, emitter `flame-plume` as a 0.2 mm outlet cap on the upper 6 mm of the tongue (D2).
- 1:50 structure: foil shell (it is the member), wax pressure column of 26 mm on the raised floor panel, bearing joint disc, wick strand with sustainer foot, joint marker at the wick, wax mass at its centre, two slender axes for the massless flame bones, annular support pad under the standing ring.
- 1:100: plain foil cone, wax and wick as one plug with a 2 mm pin (bone wick, as the plan binds it), flame in two parts with fewer stations; thermal with an own wick pin and a plain cone cap; structure shell, plug, slender wick strand, flame axes, annular support.
- 1:200: solid cup-with-wax cone with the wax level sunk 1.5 mm (C30), wax surface sheet with a wick cone (`texture`, the plan's wax disc), one flame drop; structure with an own cone shell.
- 1:500 (digital only): solid cone, wax surface with wick cone, four-station drop; structure as an own shell, wax column, plain slender cylinders, contact disc.

Final check lines (after the last change): `volumes-check.ts objects/tealight --product` → `PASS (5.7 s)`; `r3-validate.ts objects/tealight --volumes` → `VALID objects/tealight (catalog asset + volumes source): schema 0, validateAsset 0, checkVolumes 0`; `--catalog-only --volumes` → `VALID`. Silhouette IoU 1:50 0.959 / 0.977 / 0.966 (elevation / front / plan), 1:100 1 / 1 / 0.988, 1:200 1 / 1 / 1. Thermal areas at 1:50: cup 0.00557 against 0.0056, wax 0.00115 against 0.0011, wick 4.3e-05 against 4.7e-05. Spliced with `volumes-insert.py`.

## 5. What I saw (all opened after the last change of the solids)

- `r3-sheet` 1:50 body, six views: reads as a tealight at first sight: a low cup with a slight draft and a rounded base edge, wax level 1.5 mm under the rim, dark wick, slim teardrop flame with one seam where bulb and tongue meet; bones on the axis, joints at wax top, flame foot and tongue. From below (turn 40, tilt -40): flat underside, the 0.35 mm embossing is only a faint ring.
- Poses: `unlit` shows cup, wax and wick only, no flame part anywhere; `flicker-peak` leans 10 degrees and is a taller teardrop with a slight shoulder at the seam; `light-and-burn` at 2.5 s shows a short flame rising out of the wax around the wick.
- 1:50 thermal (cut): red foil shell around the green puck, purple wick down to the sustainer on the floor, yellow flame with the cap on its tip. 1:50 structure (cut): shell, wax column, magenta mass in the middle, orange joint marker and bearing disc, green ring under the base.
- 1:100, 1:200, 1:500 body: same proportions with visibly simpler forms (plain cone, plug or sheet, fewer flame stations); the flame never floats (pin or cone under it). Thermal and structure cuts show the lists described above; nothing pokes out.
- `volumes-render` overlays: 1:50 red only where the 0.13 mm strokes of wick and wax level are pen-wide; 1:100 blue only at the wick pin (not drawn at that scale); 1:200 exact.
- 2D (`r3-2d.ts --svg`, magnified by `r3-tealight-zoom.ts`): 1:50 cup with wax line, wick and flame in line and solid view, unlit without flame in elevation and plan, flicker-peak leaning; 1:100 and 1:200 are pen-dominated marks with a flame bump, as the paper size (0.4 and 0.2 mm) dictates.

## 6. Still imperfect

- Every revolve is rendered with 12 sides (8 on the flame at the coarse scales) by the tessellation quality of the tools; the schema has no segment count for an author to set.
- The base diameter of 36 mm and the sustainer sizes rest on my knowledge, not on a cited data sheet.
- The sustainer is part of the `wick` solid and carries the cotton material: as its own 3 mm solid it fails `volumes-feature`. For the same rule the wax is merged with the wick at 1:100, the wax member references the body at 1:100 and 1:200, mass and joint markers exist only at 1:50, and the 1:50 joint marker is a 7.5 mm ball on a 2 mm wick. Measurements in `author-requests.md` (2026-11-20 r3 objects/tealight).
- 1:200 in the solid views: the true-size flame is 0.04 mm wide on paper and practically vanishes; before it was drawn 23.5 mm wide. I kept one true form for drawing and body (E1) and let the 0.18 mm pen carry it in the line views.
- The unlit pose still hides the flame by sinking it into wax and wick (the schema has no visibility switch); it is now invisible from every side, but the solids interpenetrate there.
- `r3-validate.ts` does not check print suitability; I checked it with my own probe (`tool-requests.md`, 2026-11-20).

## Round 5 handcraft (2026-11-21)

Scripts: `r5-tealight-build.py` (copy of the round-3 build with the changes below; modes `volumes`, `candidate`, `catalog`). Backups of the state before this round: `🗑️generated/r5-tealight/catalog-before-r5.json`.

### Reference (modelled to)

| quantity | reference | source |
|---|---|---|
| cup | 38 mm rim, 16 mm high, 0.1 mm deep-drawn aluminium | Wikipedia "Tealight"; candle suppliers 36-39 x 15-18 mm |
| wax puck | 14 mm, 12.8 g | asset provenance |
| wick | 2 mm braid, 7 mm free above the wax, tin sustainer on the floor | product knowledge (no data sheet) |
| flame | 8 mm wide at a third of its height, 22 mm high, rounded foot, pointed tip | asset notes (drysdale-2011, quintiere-2006) |

### Model against reference (1:50 unless stated)

| measure | model | reference | deviation |
|---|---|---|---|
| rim diameter | 38.0 mm | 38 mm | 0 % |
| base diameter | 36.0 mm | 36 mm (estimate) | 0 % |
| cup height | 16.0 mm | 16 mm | 0 % |
| wax top above floor | 14.5 mm | 14 mm puck + 0.5 mm floor | 0 % |
| free wick | 7.0 mm | 7 mm | 0 % |
| flame width / height, 1:50, 1:100, 1:200, 1:500 | 8.0 / 22.0 mm each | 8 / 22 mm | 0 % |

### Grades (opened sheets: body, thermal cut and structure at 1:50, 1:100, 1:200, 1:500; poses unlit, lit, flicker-peak; clips flicker t 0.8 s and light-and-burn t 120 s; 2D 1:50, 1:100, 1:200; 1:500 is digital only)

| scale | body before → after | thermal | structure |
|---|---|---|---|
| 1:50 | A- → A- | A- | A- |
| 1:100 | A- → A- | A- | A- |
| 1:200 | B (flame a cone-footed spindle with a sharp collar) → A- | A- | A- |
| 1:500 | B- (flame a four-station diamond) → A- | A- | A- |

### Changes

- 1:200 body flame: nine stations instead of six, a blunt rounded foot at the wick and the widest point at a third of the height (0.6/1.9, 2.0/3.2, 4.5/3.9, 7.0/4.0, 11.0/3.4, 15.0/2.3, 18.5/1.2 mm); the plume outlet cap follows the new surface (inner radius 1.99 mm at 16 mm). The 1:200 2D flame (elevation and front) redrawn with the same rounded foot.
- 1:500 body flame: six stations instead of four (rounded foot and shoulder instead of a diamond); plume outlet cap moved onto it.
- Decisions of 1:200 and 1:500 rewritten in both languages.
- `volumes-check.ts objects/tealight --product`: PASS (26.9 s); `r3-validate.ts objects/tealight --volumes`: VALID (schema 0, validateAsset 0, checkVolumes 0). Spliced.

### Remaining

- The 1:50 bulb and tongue meet at a visible seam; leaning flicker poses show a slight shoulder there (two bones, two solids by design).
- The unlit pose still hides the flame by sinking it into wax and wick (no visibility switch in the schema).
- Base diameter and sustainer sizes rest on product knowledge, not a data sheet.
