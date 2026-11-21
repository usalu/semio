# Round 3 — objects/waste-bin (2026-11-20)

Status: done, spliced, catalog VALID (schema 0, validateAsset 0, checkVolumes 0). Scripts: `r3-waste-bin-build.py` (all volumes of all scales, every section stated by hand), `r3-waste-bin-roots.py` (rig and 2D root fixes; replaces only the `rig` and `representations` members in the file text, round-trip checked).

## 1. Reference

The asset is a lidded, moulded HDPE litter bin standing on a foot skirt with four pads, 0.50 m square, 1.03 m high, hinged lid at the rear. No manufacturer is named in `provenance`; the construction follows the moulded HDPE waste containers of EN 840-1/-5 (wheelie-bin family, the same moulding practice without wheels) and the asset's own references (Ashby 2011 for HDPE density, ISO 10456 for conductivity/heat capacity).

| quantity | real value / practice | source | asset |
|---|---|---|---|
| overall height (lid closed) | 0.93-1.10 m for 120-240 l moulded bins (EN 840 120 l: about 0.93 m; 240 l: about 1.07 m) | EN 840-1 dimension tables, manufacturer data sheets (knowledge) | 1.03 m |
| plan size | 0.48 x 0.55 m (120 l) to 0.58 x 0.73 m (240 l); square litter bins 0.45-0.55 m | same | 0.53 x 0.50 m (lid 0.52, hinge lugs behind) |
| draft of the walls (nesting, demoulding) | straight taper, typically 2-3 degrees per side; never segmented | moulding practice | 0.400 m at the floor to 0.496 m under the rim = 1.5 degrees per side |
| wall thickness | 3-5 mm HDPE, thicker at rim and floor | EN 840-5 test practice, injection moulding | 3 mm wall, 4 mm lid (thermal layers) |
| rim | a stiff rolled bead around the opening, the lid skirt closing flush over it; the lid seats on an upstand inside the skirt | moulding practice | 12 mm bead + 28 mm seat |
| lid | 4 mm dish with rounded edges, skirt 25-30 mm on sides and front, open at the hinge rail; front grip | moulding practice | dish 38 mm, skirt 28 mm, grip bump 15 mm |
| hinge | pin through lugs moulded on the rear of the rim, lid knuckle (barrel) between the lugs, lid notched over the lugs so it can swing open | wheelie-bin hinge construction | pin 9 mm, barrel 18 mm, lugs 21 x 62 x 60 mm |
| base | moulded foot skirt with the floor above it (hollow underside), pads at the corners | moulding practice | skirt 17 mm + 8.5 mm pads, floor 7.5 mm at 25 mm |
| bag | liner bag folded over the rim, clamped by the lid, hanging a little over the outside | use | collar of 30 micrometre LDPE film |
| masses | shell about 1.65 m2 x 3 mm x 980 kg/m3 = 4.9 kg; lid 0.27 m2 x 4 mm = 1.1 kg | Ashby 2011 density | body 4.5 kg at 0.5 m, lid 1.2 kg (kept) |

## 2. Audit of the round-2 state (before this round)

| item | found | real / required | verdict |
|---|---|---|---|
| tub wall 1:50 | five stacked tapered extrudes (`wall-1..5`, 0.025-0.2-0.4-0.6-0.8-0.955 m) with a visible step and seam every 0.2 m | one moulding with a straight draft | segmented barrel, forbidden; rebuilt as one wall |
| tub floor | `base-plate` bone at y = 0.10 m: an 8 mm false floor 75 mm above the 21 mm solid pedestal slab, i.e. two floors | one floor at the bottom of the tub, above the foot skirt | root defect of the rig |
| pedestal | solid HDPE slab 0.4 x 0.021 x 0.4 m (3.3 kg of HDPE, more than half the bin mass) | hollow moulded foot skirt with the floor above it | wrong construction |
| feet | square 50 x 50 pads whose outer corner stood 9 mm outside the 30 mm skirt radius | pads following the skirt corner | fixed |
| rim | a solid ring 15-20 mm thick, 40 mm high, x +-0.255 / z +-0.25 | bead + seat; the lid skirt must close over it | lid skirt (inner +-0.246) and rim (+-0.25) interpenetrated by 4 mm on both sides |
| hinge | brackets 26 x 65 x 60 mm at x -0.268..-0.242 overlapping the lid's rear skirt wall; 28 mm barrel centred 4 mm above the pivot; lid neither notched nor open at the rear | lugs beside the barrel, lid notched over the lugs, barrel coaxial with the pivot | static interpenetration with the lid, and the rear skirt swept through the rim when opening |
| bag collar (thermal) | a 30 micrometre ring coincident with the old rim face from 0.94 to 0.98 m | film over the seat, between seat and skirt, under the skirt edge, over the bead | rebuilt |
| structure | `of` references to the five wall segments, slab, rim, false floor, lid; body mass ellipsoid 0.12 x 0.30 x 0.12 m at y 0.45 (rig centre of mass 0.50) | monocoque load path, compact masses at the rig centres | mass moved to 0.50, compact |
| 1:200 | lid dish with its skirt to -0.02 m modelled over the rim ring of the same width (interpenetration of 20 mm) | skirt below the 30 mm feature merged into the rim | lid drawn and modelled from the seat |
| 1:500 | supports as 75 mm cubes centred on the anchors, standing 12.5 mm outside the plinth; false-floor member on the moved bone | inside the plinth | fixed |
| 2D 1:50 | pedestal drawn solid down to the ground between the feet; hinge barrel r 14 at cy 0.004; brackets 26 mm deep; front brackets at z 0.174-0.2 against plan 0.16-0.22; bag hem festoon down to 0.918 m | true view of the model | redrawn (E1) |
| dimensions | 1.03 x 0.53 x 0.50 m, 5.7 kg | consistent with the moulding and the references | unchanged |

## 3. Root fixes (catalog file, outside `volumes`)

| path | old | new | why |
|---|---|---|---|
| `rig.bones[base-plate].y` | 0.10 | 0.025 | the floor is the tub floor on the foot skirt; the waste load `waste-contents` stays on this bone (vertical, so only its height changes) |
| `representations.50.geometry.elevation/front.pedestal.d` | solid from y 0 with rounded top | `M -0.2 0.008 L 0.2 0.008 L 0.2 0.017 L 0.19 0.025 L -0.19 0.025 L -0.2 0.017 Z` | skirt above the 8 mm pads (daylight between the feet), chamfered shoulder as modelled |
| `...50.elevation.hinge-bracket` | w 0.026, h 0.065 | w 0.021, h 0.062 | lug 21 x 62 mm behind the bead |
| `...50/100.front.hinge-bracket-l/-r` | x 0.174 / -0.2, w 0.026, h 0.065 | x 0.16 / -0.22, w 0.06, h 0.062 | front now agrees with plan (z 0.16-0.22) |
| `...50/100.plan.hinge-bracket-l/-r` | w 0.026 | w 0.021 | as elevation |
| `...50.elevation/front/plan.hinge-barrel` | r 0.014, cy 0.004, span +-0.13 | r 0.009, cy 0, span +-0.155 | barrel coaxial with the pivot, between the lugs |
| `...50.elevation/front/plan.lid-skirt` | x 0, w 0.52 | x 0.012, w 0.508 | skirt open at the hinge rail |
| `...50.elevation.liner-collar.d` / front | festoon from +-0.245 at 0.945 to 0.918 | festoon from the bead corners (elevation +-0.2552, front +-0.2438 inside the pen) at 0.951 to 0.941 | the bag hangs over the bead down to 0.944 m |
| `...50.plan.lid-corner-a0` + new `lid-corner-a0-l`, `lid-corner-a0-r` | one strip z +-0.24 | z +-0.157, 0.223..0.24, -0.24..-0.223 | the 14 mm lid notches over the lugs are drawn in plan |
| `...200.elevation/front.lid.d` | from v -0.02 | from v 0 | the 28 mm skirt is below the 30 mm feature and merged into the rim ring |
| `...50.decisions` | "the 0.26 m hinge barrel" | "the 0.31 m hinge barrel between the lugs, the lid notched over the lugs" (en, de) | text follows the drawing |

No `dimensions`, masses, thermal values, poses or clips changed. `areaM2` unchanged (see section 6).

## 4. Model per scale (final)

| scale | body | thermal | structure |
|---|---|---|---|
| 1:50 | 27: 4 feet (rounded outer corner), foot skirt + chamfered shoulder (4 mm), floor 7.5 mm (base-plate), one 3 mm wall 0.025-0.952 m with draft 0.400 -> 0.496 m, 3 bands + bag bead (texture), rim bead 12 mm (+-0.255/+-0.25), seat 0.948-0.98 m inside the skirt, 2 lugs, pin d 9, bag drape (texture film), lid: dish clipped at the 14 mm notch, hinge rail (centre), 2 rail ends, 2 end walls, U skirt open at the rear, handle, barrel d 18 | 30: body references per region (bin-hdpe, bin-ribs, lid-hdpe with the inward dish faces as section faces) + 4 film shells of 30 micrometres (top, sleeve, step, drape) | 21: wall, skirt, shoulder, bead, seat (body), floor (base-plate), dish, rail, end walls, skirt (lid), pin, barrel, 2 lugs (joints), masses 4.5 kg at (0, 0.5, 0) and 1.2 kg at the lid centre, 4 foot-shaped supports |
| 1:100 | 16: hollow 25 mm foot skirt (feet merged), floor 15 mm, wall, 3 bands, bead 15 mm, seat 15 mm wide, 2 lugs, lid: dish, rail, 27 mm end walls taking in the rail corners, U skirt, handle (pin and barrel not drawn) | 21 | 18 |
| 1:200 | 6: 30 mm slab, wall, 40 mm rim ring, dish seated on the rim, 2 end walls | 10 (bands thermal-only, film ring) | 13 |
| 1:500 | 3: 75 mm plinth, tapered wall incl. rim, lid plate | 7 | 10 (supports pulled 1 mm inside the plinth, floor member inset) |

Decisions per scale: three bilingual sentences each (body, thermal, structure), in the source.

Final check: `volumes-check.ts objects/waste-bin --product` PASS (41 rule lines ok). Silhouette IoU elevation/front/plan: 1:50 0.998/0.996/0.999 (p95 2/4/0 mm), 1:100 0.998/0.996/1.000, 1:200 0.995/0.994/0.999, 1:500 0.997/0.993/1.000. Areas at 1:50: bin-hdpe 3.82 (+0.6 %), lid-hdpe 0.425 (-4.6 %), liner-film 0.206 (+8.3 %), bin-ribs 0.279 (-0.3 %). `r3-validate.ts objects/waste-bin --catalog-only --volumes`: VALID, schema 0, validateAsset 0, checkVolumes 0.

## 5. What I saw in the final renders

- 1:50 body, front (+x): straight tapered tub without the old segment seams, three bands and the bag bead, bead and lid flush at the top, handle bump; feet visible under the skirt corners. Right side: lid dome with the front overhang, lug behind the rear, bead below the skirt. Back: the two lugs, the exposed pin and the barrel between them, the lid notches over the lugs. Top: lid with handle, lugs behind the rear edge. Three-quarter: clean moulding, rounded corners growing with the draft.
- 1:50 lid-open pose (three-quarter from behind) and clip `lid-open-close` at t 0.8 s (right side): the lid swings about the pin from the rear; no part of the lid enters the lugs, seat or bead; seat and inward lip visible, tub interior open.
- 1:50 thermal (cut): wall, floor and skirt red, bands yellow, lid green with its inner faces hatched as section faces, the film in purple over seat and bead.
- 1:50 structure (cut): shell members blue, lugs/pin/barrel orange, body mass at mid-height, lid mass inside the dish, four green pads under the feet.
- 1:100: the same reading with fewer parts; the lid ends show as wider strips in plan (27 mm end walls). 1:200: slab, wall, rim ring, domed lid; the overlays green with red only at the band strokes. 1:500: plinth, flared wall, lid plate; supports flush inside the plinth corners after the fix.
- 2D 1:50 (product drawings, closed and lid-open): pedestal with daylight between the feet, narrower lugs, the smaller barrel, the bag hem over the bead; the open lid stands up correctly in elevation, front and plan.

## 6. Still imperfect (stated plainly)

- Lid end walls at 1:50 are 9 mm solid plates (real: 4 mm curved dish ends): a separate 4 mm plate would fail `volumes-feature` (7.5 mm); outwardly identical.
- The bag hem is a straight edge in 3D (extrude), the 2D draws a shallow festoon (3-7 mm sag, under one feature at 1:50).
- 1:500 plinth is 75 mm (the print feature) for a real 25 mm pedestal; 1:500 lid mass is 38 mm thick to stay inside the lid plate.
- Structure is the monocoque moulding itself (references to wall, rim, skirt, floor, lid), which is the real load path of a moulded bin; the members reuse the body solids by D3's exception, not a block.
- Thermal areas rest on the area reviewers' convention (tub inner faces count, lid underside is a section face); under C31's flood rule the closed tub's inner faces would not count. Noted in `area-review.md`, nothing changed.

## Round 5 handcraft (2026-11-21)

Scripts: `r5-waste-bin-rig.py` (rig root fix, 1:200 plan and lid, `--apply`), shared `r5_o02_lib.py`, `r5-o02-sheets.sh`, `r5-o02-montage.sh`, `r5-o02-measure.ts`. Renders under `🗑️generated/r5-waste-bin/{before,after}/` (every scale body, thermal cut, structure, 2D; poses closed-empty, lid-open, full-load; clip lid-open-close 0.8 s), all opened.

### Model against reference (1:50, default pose, `r5-o02-measure.ts`)

| quantity | reference (section 1, EN 840 moulded bin class) | model | deviation |
|---|---|---|---|
| height, lid closed | 0.93 to 1.10 m (asset 1.03) | 1.033 m | +0.3 % to `heightM` |
| plan size | square 0.45 to 0.55 m (asset 0.53 x 0.50 incl. lugs) | 0.533 x 0.502 m | +0.6 % / +0.4 % |
| tub wall draft | 2 to 3 degrees per side typical | 0.400 to 0.496 m over 0.927 m = 3.0 degrees | inside |
| rim bead | 510 x 500 mm, 12 mm | 510 x 500 x 12 mm | 0 % |
| floor | at the bottom of the tub on the foot skirt | 25 mm above ground, 396 mm square | 0 % |
| hinge pin | 9 mm, 0.44 m | 9 mm, 0.440 m | 0 % |

### Grades (body / thermal / structure)

| scale | before | after |
|---|---|---|
| 1:50 | A- / A- / B (rig: `body` bone lay on the ground and ran 0.5 m ahead, its tip 0.25 m in the air in front of the bin; the forces view drew the shell member as a floor bar) | A- / A- / A- (bone up the tub axis to the pivot) |
| 1:100 | A- / A- / B | A- / A- / A- |
| 1:200 | A- / A- / B; 2D plan lid in six stepped strips leaving 0.01 mm white slivers at 1:150 | A- / A- / A- (plan lid one strip; lid one solid dome) |
| 1:500 | A- / A- / B | A- / A- / A- |

Catalog before: INVALID (5 `range-end-gap` at 1:150, thermal total at 1:200 and 1:500). After: schema 0, validateAsset 0; `checkVolumes` 2 = the thermal totals at 1:200 and 1:500 (2.57 / 2.62 against 4.715 m2), an area dispute listed in `area-review.md` (2026-11-21) and `tool-requests.md`, not forced.

### Changes (root, `r5-waste-bin-rig.py`)

- `rig.bones.body`: angle 0 -> 90, length 0.5 -> 0.98 (base to pivot), title en/de; mass centre (0, 0.5) -> (0.5, 0) in the new frame (same world point). Children `base-plate` (-0.2, 0.025, 0 deg) -> (0.025, 0.2, -90 deg) and `lid` (-0.255, 0.98, 0 deg) -> (0.98, 0.255, -90 deg): same world places; the lid still opens 0 to 110 degrees about the pin (pose lid-open and the clip checked).
- `anchors` foot-*: (±0.175, 0) -> (0, ∓0.175) in the new frame (same world places); normals unchanged.
- Every elevation shape bound to `body` at every scale and every `body`-bone solid of every scale and list restated in the new frame (frame turn as transform, `sectionFaces` remapped); world geometry identical (extents measured before and after).
- 1:200: plan corner strips `lid-corner-a0..b2` removed, `lid-span` 0.52 m; 3D lid one solid dome over 0.5 m (the 4 mm shell and 16 mm end walls are below 30 mm); its thermal and structure references follow; decisions en/de rewritten.

### Remaining

- Thermal total at 1:200 and 1:500 fails (see above): the closed lid seals the tub at the coarse flood cells, the 14 mm notches that open it at 1:50/1:100 are below the coarse resolution. Left for the area reviewers; the author did not enlarge real openings.
- Section 6 items stand (9 mm lid end plates at 1:50, straight bag hem in 3D, 75 mm plinth at 1:500, monocoque structure by reference).

The current bin retains the 9 mm lid end walls, 14 mm hinge notches, rolled seat, cap, rear rail, open skirt, four 8.5 mm pads and 7.5 mm floor across the scale series. The air-space check now resolves the measured notch, including at 1:500, and distinguishes it from the same enclosure with the slit closed. The 1:200 and 1:500 exposure failures recorded above have been resolved. The four measured geometry checks and the independent twelve-scene GLB reload passed (1,305,728 bytes).
