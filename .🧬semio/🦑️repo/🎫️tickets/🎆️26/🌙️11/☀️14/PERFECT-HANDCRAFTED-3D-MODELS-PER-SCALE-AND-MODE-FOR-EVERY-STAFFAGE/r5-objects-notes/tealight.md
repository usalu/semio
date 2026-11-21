
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
