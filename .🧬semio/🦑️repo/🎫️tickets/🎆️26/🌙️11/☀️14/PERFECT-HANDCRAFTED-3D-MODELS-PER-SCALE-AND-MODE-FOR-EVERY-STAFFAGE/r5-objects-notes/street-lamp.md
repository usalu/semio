
## Round 5 handcraft (2026-11-21)

Scripts: `r5-street-lamp-kit.py`, `-model.py`, `-roots.py` (the round-3 chain carried over; the r5 chain reproduced the catalog exactly before any change). Zoom renders with the round-3 helpers `r3-street-lamp-zoom.ts` and `-montage.ts`. Backups: `🗑️generated/r5-street-lamp/catalog-before-r5.json`, `source-before-r5.json`.

### Reference (modelled to)

| quantity | reference | source |
|---|---|---|
| column | 6 m conical steel column, flange plate, taper 13-15 mm/m, wall 3-4 mm | EN 40-2 / EN 40-5 practice |
| base plate | 300 x 300 x 20 mm, 4 x M20 | EN 1993-1-1, ISO 4032 |
| bracket | single side-entry arm, 60 mm tube | luminaire spigot 48-60 mm |
| luminaire | 505 x 270 mm, 4.5 kg (Philips side entry); 550 x 238 x 157 mm (Thorn Isaro small) | data sheets cited in round 3 |

### Model against reference (1:50)

| measure | model | reference | deviation |
|---|---|---|---|
| column height | 6.00 m | 6 m | 0 % |
| foot / top diameter | 170 / 80 mm | taper 15.3 mm/m (EN 40 practice 13-15) | +2 % on the taper |
| base plate | 300 x 300 x 20 | 300 x 300 x 20 | 0 % |
| luminaire length / width | 505 / 274 mm | 505 / 270 mm | 0 % / +1.5 % |
| arm tube | 60.4 mm | 60 mm | +0.7 % |

### Grades (opened: body, thermal cut, structure at 1:50 to 1:500; zooms of head and base at every scale; poses dusk-off, lit-night, dimmed-night, lit-in-wind, day-off and clip frames ignition-at-dusk t 360 s, pole-sway-in-gust t 3.2 s in the pose montage; 2D 1:50 to 1:500)

| scale | body before → after | thermal | structure |
|---|---|---|---|
| 1:50 | A- → A- | A- | A- |
| 1:100 | A- → A- | A- | A- |
| 1:200 | A- → A- | A- | A- |
| 1:500 | B+ (luminaire ends in a chisel: last station 60 mm wide at full length) → A- | A- (lens and plume follow the new head) | A- |

### Changes

- 1:500 luminaire: five stations instead of four (0.40 m 128/40/-50 mm, 0.475 m 104/22/-46 mm, 0.505 m 50/2/-32 mm), a rounded nose seen from above and from the side; the 2D of all three views takes the same stations (`head_views` reads `HEAD_500`), the plume outlet follows the crown.
- 1:500 decisions rewritten in both languages (the German one also gave a wrong 0.28 x 0.08 m).
- `r3-validate.ts objects/street-lamp --volumes`: VALID (schema 0, validateAsset 0, checkVolumes 0). Spliced.

### Remaining

- Door lock and hatch latch are not modelled (round 3 reasons stand).
- At 1:500 the 80 mm arm bar is as thick as the luminaire is high; that is the feature floor (true 60 mm tube).
- 1:500 structure bearing pad wider than the plinth (round 3 reason stands).
