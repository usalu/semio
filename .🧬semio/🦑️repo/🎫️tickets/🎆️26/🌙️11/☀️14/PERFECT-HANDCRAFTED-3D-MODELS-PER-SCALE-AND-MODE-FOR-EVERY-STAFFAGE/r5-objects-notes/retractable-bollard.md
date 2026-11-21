
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
