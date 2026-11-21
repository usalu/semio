# Round 3 — `objects/retractable-bollard`

Author: round-3 author (claude-code), 2026-11-20. No earlier round-3 work existed; the catalog `volumes` equalled the source at the start (the bulk passes of 03:40-04:10 had not touched this asset). Scripts: `r3-retractable-bollard-kit.py` (dimensions and every solid, stated by hand in world coordinates), `r3-retractable-bollard-model.py` (writes `volumes/objects/retractable-bollard.json`), `r3-retractable-bollard-roots.py` (rig, anchors, poses, 2D drawings, symbol, areas; candidate or `--write`).

## Reference

Sources are my own knowledge of the product class (electro-hydraulic automatic rising bollards such as FAAC J275 HA, Urbaco G6, Pilomat 200) and the asset's `provenance.references` (`bollard-class`). No web lookup was made; nothing below is a quoted datasheet value.

| item | real value | used |
|---|---|---|
| rising cylinder | 200-275 mm diameter, 500-800 mm raised, steel tube 5-10 mm, guide flange or shoes at the foot that stay 0.2-0.5 m inside the pit | 200 x 5 mm, 0.65 m raised, 1.15 m long (0.50 m stays guided) |
| pit casing | galvanised steel tube or box, about 1.0-1.3 m deep for 0.6-0.8 m stroke, anchor lugs for the concrete, drainage in the floor | 300 x 8 mm, 1.25 m deep, 8 lugs, 3 drain holes |
| cover flange | cast or steel plate bolted over the pit, set about 10-15 mm above the paving against water, bevelled edge, scraper seal against the cylinder | 0.40 m square, 25 mm, 15 mm proud, splayed edge, 4 anchor bolts, retainer ring and wiper |
| head | cast cap with an LED ring behind an amber lens and a retro-reflective band 50-110 mm below it | lens ring 60 x 3 mm, 12 LED windows, film 110 x 0.5 mm, domed crown 70 mm |
| drive | single-acting cylinder standing in the pit inside the rising cylinder, rod fixed under the head; barrel longer than the stroke; bore 30-60 mm, rod 25-40 mm | barrel 70 x 5 mm, 0.89 m tube for 0.65 m stroke, rod 36 mm, piston 60 mm |
| power unit | 0.75 kW motor-pump-tank pack in a sealed housing, hose to the cylinder, power cable | housing 0.19 x 0.188 x 0.40 m beside the sleeve (the asset's own concept, kept) |

## Audit (before)

- Rig: all three bones lay horizontal (`angleDeg` 0) although sleeve, shaft and cylinder are vertical. The structural evaluation runs its N/M diagrams along the bone, so the nudge of 1.5 kN gave the shaft 150 N·m (lever 0.1 m) instead of a cantilever moment. `hydraulic` was a 0.4 m bone at the unit, not on the cylinder.
- Cylinder: barrel 0.55 m for a stroke of 0.65 m (impossible single stage), barrel 120 mm for a 38 kg shaft, the rod bound to the fixed bone (it did not move with the shaft), the shaft drawn and modelled solid so the cylinder could not stand inside it.
- Body 1:50: 16 solids; shaft a solid bar in four stacked discs, reflector and beacon as full discs, frame without collar, unit a box with a hole, no guide, seal, flange, crosshead, piston, buffers, lugs, brackets.
- 2D: front "section B-B" repeated A-A and showed exterior features (chevrons, LEDs) on a cut; 1:200 and 1:500 drew the sleeve 0.20 m wide (real 0.30 m), the frame 0.42 m long and the unit 0.26 m; reflector tone 0.08 (black) for a white film; the unit straddled the section plane (z -0.01).
- Structure: unit box and brush seal as member and joint; 1:500 ram as a solid 0.12 m bar.

## Root fixes (catalog file, outside `volumes`)

| member | old | new |
|---|---|---|
| bone `sleeve` | x 0, y 0, angle 0 | x 0, y -1.25, angle 90 (runs up to the ground), com (0.65, 0), section tube 300 x 8 |
| bone `shaft` | x 0, y 0, angle 0, prismatic axis 90 | x 0.10, y 0 on the sleeve, angle 0, prismatic axis 0 (0..0.65), com (0.60, 0), section tube 200 x 5 |
| bone `hydraulic` | x 0.2, y -0.8, length 0.4 | x 0.008, y 0 (on the floor plate, the cylinder axis), length 0.972, com (0.442, -0.2), section tube 70 x 5 |
| material `steel-hyd` | no modulus | `elasticModulusPa` 210e9 (needed by the section) |
| anchors | sleeve-fixed (0, 0), push-point (0, -0.1), beacon-glow (0, -0.07) | (1.25, 0), (1.05, 0), (1.05, 0): the same world points |
| poses | `shaft: { dy }` | `shaft: { dx }` (same travel) |
| mass centres | | unchanged in the world (physical data not touched) |
| `representations` | sections A-A and B-B at 1:50, 1:100; none at 1:200, 1:500 | elevation = section A-A at all four scales, front = exterior view at all four; every shape of every view redrawn by hand from the body; all `decisions` rewritten (en, de) |
| symbol true-extent | ram 0.12 x 0.55, unit at x 0.15 / z -0.01 | cylinder 0.07 x 0.972, unit at x 0.17 / z 0.012, hose path, gland |
| `thermal.regions` `cover-frame.areaM2` | 0.093 (top face only) | 0.13: top 0.096 plus the 15 mm edge in the air (C25: free faces are never section faces); factor 1.40 (C16) |
| `thermal.regions` `hydraulic-unit.areaM2` | 0.46 (old box 0.21 x 0.21 x 0.40 alone) | 0.58: housing and lid 0.36, cylinder outer surface 0.20, hose 0.02; factor 1.26 (C16). `heatW` unchanged |

Effect on the analysis (pose raised-nudged): shaft 1.57 kN·m at its foot, sleeve 1.64 kN·m, support 888 N·m as before, residual zero.

## Model

| scale | body | thermal | structure |
|---|---|---|---|
| 1:50 | 42 solids: sleeve tube, floor with drains, 8 lugs, guide bushing, 3 buffers, collar, plate, splayed top, 4 bolts, retainer, wiper; shaft tube, flange, film, head core, lens, 12 LED windows, crown, 3 screws, crosshead, nut, rod, piston; cylinder flange, bolts, cap, tube, gland, port, hose, grommet, union; unit body, lid, bolts, gland, cable, conduit, 2 brackets | 14: shaft skin and crown, film, lens and LEDs, plate above ground, sleeve skin, floor, lugs, unit body and lid, hose, cylinder surface | 16: sleeve, shaft, cylinder, cap, rod; bushing, flange, crosshead, piston, foot flange, 2 brackets; collar support; 3 masses |
| 1:100 | 21 | 11 | 13 |
| 1:200 | 10 | 9 | 11 |
| 1:500 | 6 | 6 | 8 |

Final lines: `volumes-check.ts objects/retractable-bollard --product` PASS (12.0 s); silhouette IoU elevation / front / plan 0.988 / 0.997 / 1.000 (1:50), 0.992 / 1.000 / 1.000 (1:100), 0.981 / 0.991 / 1.000 (1:200), 1.000 / 1.000 / 1.000 (1:500). `r3-validate.ts objects/retractable-bollard --volumes`: VALID, schema 0, validateAsset 0, checkVolumes 0; `--catalog-only` VALID. Print suitability: all four scales suitable. Spliced with `volumes-insert.py`.

## What I saw (sheets opened after the last change)

- 1:50 body, six views and front alone: a shaft with white band, lens ring with LED windows, grooved crown, standing in a bevelled square frame with four bolts and a seal ring; below ground the sleeve with collar and lugs, the housing with lid, gland and cable on two brackets, the hose from the foot of the sleeve to the union. Cut views raised and lowered: flange on the buffers when lowered, crown under the frame, piston inside the barrel in both end positions, crosshead clear of the gland.
- 1:50 thermal (cut): six regions, cylinder and hose inside; structure: tubes, joints, green collar, masses.
- 1:100, 1:200, 1:500 body, thermal, structure: same object, simplified, proportions unchanged. After the first look the 1:200 structure lacked the band ring of the tube (rod stood free); added and looked again.
- 2D at 1:50 (raised, lowered, raised-nudged forces and moments), 1:100, 1:200, 1:500; clip `raise-and-lower` at 4.5 s (1:100 cut).

## Still imperfect

- Sleeve wall 8 mm (thermal layer) against the declared 42 kg for sleeve and frame: the tube alone weighs about 69 kg. Not changed, I have no cited reference for either number.
- The asset has no rubber, aluminium or chrome material: buffers and wiper carry `cast-iron-frame`, the head casting `steel-grey`, the rod `steel-hyd`.
- Not modelled: spiral cable of the beacon, motor, pump and tank inside the sealed housing, piston seals.
- `beacon-led` measures 0.084 m2 against 0.098 (14.4 %, inside the tolerance, left as it is).
- 1:500 thermal: the whole cap carries `beacon-led` (a 60 mm ring is below the feature); the crown is not separate there.
- Plan views draw no shape for `sleeve-steel`, `reflector-band`, `beacon-led` (hidden from above); the validator accepts it.
- The reference table is from memory, not from fetched datasheets.

State: DONE

## Round 5 handcraft (2026-11-21)

Scripts: `r5-retractable-bollard-kit.py`, `-model.py`, `-roots.py` (the round-3 chain carried over; `python3 r5-retractable-bollard-model.py`, `python3 r5-retractable-bollard-roots.py --write`, `volumes-insert.py`). Before any change the r5 chain reproduced the catalog exactly. Backups: `🗑️generated/r5-retractable-bollard/catalog-before-r5.json`, `source-before-r5.json`.

### Reference (modelled to)

| quantity | reference | source |
|---|---|---|
| rising shaft | 200 mm (J200 class) to 275 mm (J275 class), 600-800 mm raised, 5-10 mm wall | FAAC J200/J275, Pilomat 200 product class (knowledge, no data sheet fetched) |
| pit casing | 1.0-1.3 m deep for 0.6-0.8 m stroke, 300-400 mm tube | same |
| cover flange | 10-15 mm proud of the paving | same |
| head | LED ring under an amber lens, retro-reflective band 50-110 mm below | same |

### Model against reference (1:50)

| measure | model | reference | deviation |
|---|---|---|---|
| shaft diameter / wall | 200 / 5 mm | 200 / 5-10 mm | 0 % |
| raised height above the cover | 0.65 m | 0.60-0.70 m (J200 0.60, Pilomat 0.70) | inside the range |
| pit depth | 1.25 m | 1.0-1.3 m | inside |
| cover plate | 0.40 m, 15 mm proud | 0.40 m class, 10-15 mm | 0 % |
| reflective band | 110 mm | 50-110 mm | inside |

### Grades (opened: body, thermal cut, structure at 1:50, 1:100, 1:200, 1:500; poses raised, lowered, moving-up, raised-nudged; clips raise-and-lower t 12 s, nudge-test t 4 s; 2D 1:50 to 1:500)

| scale | body before → after | thermal | structure |
|---|---|---|---|
| 1:50 | A- → A- | A- | A- |
| 1:100 | A- → A- | A- | A- |
| 1:200 | A- → A- | A- | A- |
| 1:500 | B+ (faceted chisel cap); catalog INVALID: three print findings at 1:1000, the coarse end of the band (frame-l/frame-r gap 0.130 mm, sleeve feature 0.188 mm, shaft feature 0.184 mm) → A-, VALID | A- | A- |

### Changes

- 1:500 cap: the kit's crown curve on four stations (a dome from the 0.10 m skirt to the apex) instead of the faceted cap, in the body and in the 2D (`cap500`, section and front).
- 1:500 section A-A: the cover frame is one cut block of 0.40 m instead of two blocks with the bore between them (the 30 mm play around the 0.20 m shaft is below the 75 mm feature); body pen 0.2 mm (was 0.13) so that the 20 mm sleeve wall and the shaft still print at 1:1000. The range 1:350-1:1000 is kept, no hole between the bands.
- 1:500 decisions (silhouette, styles) rewritten in both languages.
- `r3-validate.ts objects/retractable-bollard --volumes`: VALID (schema 0, validateAsset 0, checkVolumes 0).

### Remaining

- The reference is product-class knowledge, not a fetched data sheet (as in round 3).
- Rubber, chrome and aluminium parts still carry the nearest existing material.
- Motor, pump, tank, piston seals and the beacon's spiral cable are not modelled.
