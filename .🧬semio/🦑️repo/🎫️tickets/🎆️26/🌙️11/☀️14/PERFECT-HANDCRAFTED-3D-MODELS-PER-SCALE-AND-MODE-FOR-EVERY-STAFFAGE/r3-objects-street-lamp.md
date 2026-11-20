# Round 3 — `objects/street-lamp`

Author: round-3 author (claude-code), 2026-11-20 (started 02:00, interrupted 04:10, resumed and finished the same day).

Scripts (all in the ticket folder, re-runnable, idempotent):

- `r3-street-lamp-kit.py` — every hand-stated dimension, shared by the solids and the 2D drawings.
- `r3-street-lamp-model.py` — writes `volumes/objects/street-lamp.json` (all four scales, three lists, decisions).
- `r3-street-lamp-roots.py [--write]` — root fixes outside `volumes` (rig, dimensions, anchor, references, thermal titles, every 2D drawing and its decisions).
- `r3-street-lamp-zoom.ts`, `r3-street-lamp-montage.ts`, `r3-street-lamp-svgzoom.ts` — close-up renders (a 6 m column is 3 px wide on the standard sheet, so the judging was done on zooms of head, bracket and base).

Ship chain: `python3 r3-street-lamp-model.py; python3 r3-street-lamp-roots.py; $BUN r3-validate.ts --file 🗑️generated/r3-street-lamp/🔣️.json; python3 r3-street-lamp-roots.py --write; $BUN volumes-check.ts objects/street-lamp --product; python3 volumes-insert.py volumes/objects/street-lamp.json <catalog file>; $BUN r3-validate.ts objects/street-lamp --volumes`.

## Reference

| item | real value | source |
|---|---|---|
| column | conical steel lighting column, flange plate mounted, 6 m, hot-dip galvanised, wall 3-4 mm, taper about 13-15 mm per m on the diameter (straight cone, no entasis), inspection door about 400-500 x 85-100 mm at 0.6-1.2 m above ground | EN 40-2 / EN 40-5 practice; asset 170 -> 80 mm over 5.88 m = 15.3 mm/m |
| pole mass | cone shell mean diameter 0.125 m x 4 mm x 5.88 m x 7850 kg/m3 = 72 kg (declared 70 kg) | EN 1993-1-1 density |
| base plate | 300 x 300 x 20 mm = 14.1 kg (declared 14.1 kg), 4 anchor bolts M20 (hex nut AF 30 x 16 mm, washer 37 mm) | EN 1993-1-1; ISO 4032 |
| bracket arm | single side-entry bracket, tube 60 mm (spigot 48-60 mm), luminaire spigot entry about 100 mm, clamp sleeve bolted with M10 (hex AF 17) | luminaire data below (side entry 48-60 mm); ISO 4017 |
| luminaire | compact LED street luminaire: Philips side-entry 48-60 mm 505 x 270 mm, 4.5 kg; Thorn Isaro small 550 x 238 x 157 mm, 4.1 kg; DIALux sheet 602 x 282 x 195 mm, 5.32 kg, 52 W, spigot 60 mm; flat lens underneath, die-cast body with cooling fins over the LED board, gear hatch with a joint across the cover | https://www.assets.lighting.philips.com/is/content/PhilipsLighting/fp910925864077-pss-en_gb ; https://www.thornlighting.co.uk/en-gb/products/outdoor-lighting/road-lighting/Isaro/isaro-small-narrow-road-optic/96636141/@@download-description ; https://luminaires.dialux.com/en/article/cPthucZZTuWLhRwU72u6AA |

## Audit against the reference (state before round 3)

- Rig: `arm.lengthM` and `head.lengthM` described a 0.33 m luminaire on a short arm; a real compact road luminaire is 0.50-0.60 m. `head.mass` was an estimate without source. Arm and pole had no real tube sections.
- `dimensions.widthM` followed the short head.
- 2D: the column had circumferential seams (a 6 m cone is one piece), a "reflector" under a domed lens (an LED head has a flat lens), the head was a box with a dome; in plan the arm and clamp were layered under the base shroud although they are 5.8 m above it.
- Solids: box and dome luminaire, arm without clamp, strut without fixings, no base plate fixings; structure referenced body solids wholesale; kernel traps (open revolve profiles filled by implication, `kernel-traps.md` rows 34-51 and 179-198).

## Root fixes (in the catalog asset, by `r3-street-lamp-roots.py`)

| member | old | new |
|---|---|---|
| `rig.bones.arm.lengthM` / `mass.comX` | short arm | 0.82 / 0.41 |
| `rig.bones.arm.section` | none | tube 60.4 x 3 mm: A 5.41e-4 m2, I 2.234e-7 m4, extreme fibre 0.0302 |
| `rig.bones.pole.section` | none | tube 170 x 4 mm at the foot: A 2.086e-3 m2, I 7.19e-6 m4, extreme fibre 0.085 |
| `rig.bones.head.lengthM` / `mass` | 0.33 / estimate | 0.505 / 4.5 kg at (0.24, -0.005), literature `luminaire-philips-505` |
| `dimensions.widthM` / `massKg` | short head | 1.375 / 90.1 |
| anchor `lamp-plume` | above the old dome | (0.29, 0.0602) on the fin tops over the LED board |
| `thermal.regions.arm-alu.areaM2` | 0.22 | 0.27 (factor 1.23, within C16; measured 0.272) |
| `thermal.regions.lamp-housing.areaM2` | 0.23 | 0.36 is in the catalog now (not set by my scripts; my proposal 0.37 stands in `area-review.md`, measured 0.373 at 1:50) |
| `provenance.references` | — | + three luminaire data sheets, EN 40 |
| 2D, all four scales, three views | old head and seams | redrawn as the true views of the modelled body from the same kit dimensions (ruling E1), decisions rewritten |

Resume pass (after 04:10) added: plan layer order by real height (arm 3.6, clamp 3.7, cap 3.8 over column, band, shroud, plinth) at every scale; at 1:50 the foot lug of the strut (elevation), the two M10 clamp bolts (elevation hexagons, plan bar) and the cap circle in plan. The inspection door faces the elevation viewer (-z) in the drawing and in the model.

The bulk passes of 03:40-04:10 did not change this asset's `volumes` (catalog equal to my source at resume) and added no scale; all four scales are authored.

## Model, per scale and list

Counts body / thermal / structure: 1:50 23 / 23 / 20, 1:100 9 / 10 / 13, 1:200 6 / 7 / 13, 1:500 4 / 6 / 11.

- **1:50 body**: concrete plinth with rounded arrises; base plate 300 x 300 x 20; four M20 hex nuts and bolt ends; flared 3 mm base shroud (hollow revolve); hollow cone 170 -> 80 mm, 4 mm wall; push-in dome cap; brace band; flush inspection door 500 x 90 (texture); cast clamp sleeve with bolting ear and two M10 bolts; socket; 60 mm arm; 27 mm strut with foot lug on the band and gusset lug under the arm; luminaire as domed cover loft (13 stations, superellipse 2.4) over a flat-bottomed tray loft (superellipse 6), spigot collar, raised gear-hatch joint, six transverse fins, flat lens.
- **1:50 thermal**: closed outer envelopes for column and shroud (C31), references elsewhere; lens with its inner face as section face; plume outlet as a 2 mm curved plate on the fin tops (D2).
- **1:50 structure**: plinth above a 15 mm bearing pad (support), plate, four anchor bolts through the plinth, hollow column with base fillet weld, clamp sleeve and bolts, band and both lugs as joints, hollow arm 60 x 3, hollow strut 26.9 x 2.6, spigot clamp joint, tray as head chassis, four masses at the declared centres.
- **1:100**: plate merged into the plinth, solid shroud flare, solid cone with merged dome, one clamp sleeve, slender strut, cover and tray on nine stations, lens texture; structure keeps the hollow tubes (C30).
- **1:200**: plinth, cone with rounded top, arm from the axis, slender strut, one superelliptic housing loft on six stations, lens texture; structure with its own flat chassis loft, solid joints, pad.
- **1:500**: plinth, straight cone, 80 mm arm bar, head on four stations; structure with chassis loft, base and clamp joints, bearing pad of one feature.
- Decisions are bilingual per scale and list, updated for every part named above.

Changes of the resume pass in the solids: clamp bolts, foot lug, hatch joint lip; cover nose 1.5 mm inside the tray nose (two coplanar end caps flickered); foundation member no longer coincides with the support pad at 1:50, 1:100, 1:200 (at 1:500 the pad is 0.33 m wide around the block, because two layers of 75 mm do not fit into 120 mm); plume plates thinner (2 / 3 / 4 / 6 mm) and following the highest crown under them (they were half sunk into the cover at 1:100 and 1:200); 1:100 head on nine instead of seven stations (the nose read as a chamfer).

## Checks (final state, after the last change)

- `volumes-check.ts objects/street-lamp --product`: `PASS (22.7 s)`, every rule at every scale.
- Silhouette IoU elevation / front / plan: 1:50 0.994 / 0.997 / 0.995, 1:100 0.996 / 0.998 / 0.999, 1:200 0.985 / 0.996 / 0.965, 1:500 0.985 / 1.000 / 1.000.
- Thermal area deviation at 1:50: pole-galv 1.5 %, plinth-concrete 3.3 %, arm-alu 0.9 %, lamp-housing 3.6 %, lamp-lens 5.1 %.
- `r3-validate.ts objects/street-lamp --volumes`: `VALID ... schema 0, validateAsset 0, checkVolumes 0`; `--catalog-only`: `VALID`.
- Spliced; catalog `volumes` equal to the source; `r3-street-lamp-roots.py` reproduces the catalog file exactly (idempotent).

## What I saw in the final renders

Opened after the last change: all twelve `r3-sheet` sheets (`🗑️generated/r3-sheet/objects/street-lamp.s<N>.<list>.png`), the zoom montages `🗑️generated/r3-street-lamp/zoom/m50.png`, `m100.png`, `m200.png`, `m500.png`, `mplume.png` (head from front-right above, base, bracket from behind and below, each list), the 2D sheets of 1:50, 1:100, 1:200, 1:500 and the 1:50 overlay sheet.

- Sheets, six directions: the rig lies on the model at every scale (foundation at the ground, pole joint on the plate, arm joint at 5.80 m in the clamp, head joint at the spigot); the proportions read as a 6 m road column with a short braced bracket at first sight, also at 1:500.
- 1:50 body: front-right above shows spigot collar, hatch joint, six fins, parting line between cover and tray; from below the flat lens proud of the tray and the gusset lug; from behind the clamp ear with two hex bolts and the strut foot on its lug at the band; the base shows plinth, plate, four nuts with bolt ends and the flare. No gaps, no flicker left at the nose.
- 1:50 thermal: five regions, the fin field with its outlet plate, lens cyan underneath. Structure: hollow tubes visible at the open column top, orange joints at clamp, band, spigot and base weld, green pad under the plinth, the head mass in the chassis.
- 1:100 to 1:500: each coarser model is a new, simpler body that still reads as the same lamp; outlet plates lie on the crown; supports are distinct from the foundation.
- 2D: elevation, front and plan agree with the model; the plan now shows arm, clamp with ear and cap over the base.

## Still imperfect

- The door lock (about 10 mm) and the luminaire's hatch latch are not modelled: the lock is at the feature limit, a latch on the nose would lengthen the declared width.
- 1:500 structure: the bearing pad is wider than the plinth (see above); it reads as a block under the block.
- Shaded renders show curved parts (cap, band, clamp, shroud foot) with visible facets; that is the renderer's segment count, the solids are true revolves.
- The plan overlay at 1:50 shows a hair of "drawn only" along one side of the arm (pen width of the outline), IoU 0.995.
- `lamp-housing.areaM2` 0.36 is beyond factor 1.5 of the old 0.23 and was decided outside my scripts; `heatCapacityJM2K` of its layer was not adjusted by me.

State: DONE
