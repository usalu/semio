
## Round 5 handcraft (2026-11-21)

Scripts: `r5-stepladder-kit.py`, `-volumes.py`, `-draw.py`, `-roots.py`, `-areas.json`, `-build.py` (the round-3 chain carried over; the r5 chain reproduced the catalog exactly before any change). Backup: `🗑️generated/r5-stepladder/catalog-before-r5.json`.

### Reference (modelled to)

| quantity | reference | source |
|---|---|---|
| platform height | 1.06 m | Hailo L60 StandardLine 8160-507 (retailer data) |
| open height / folded length | 1.68 / 1.82 m | same |
| width at the feet | 0.48 m | same |
| tread depth | 80 mm | same |
| step pitch | 0.212 m | 1.06 / 5 |
| climbing inclination | 60-70 degrees | EN 131-1 functional sizes (knowledge) |
| rail above the platform | at least 0.6 m | EN 131-1 (knowledge) |

### Model against reference (1:50)

| measure | model | reference | deviation |
|---|---|---|---|
| platform | 1.060 m | 1.06 m | 0 % |
| open height | 1.680 m | 1.68 m | 0 % |
| width at the feet | 0.47 m | 0.48 m | -2.1 % |
| tread depth | 80 mm | 80 mm | 0 % |
| pitch | 212 mm | 212 mm | 0 % |
| stile inclination | 67.5 degrees | 60-70 | inside |
| stile tops above the platform | 0.62 m | >= 0.6 m | inside |

### Grades (opened: body, thermal cut, structure at 1:50 to 1:500; poses open-empty, open-in-use, rated-load-test; clips climb-and-work t 4.8 s, rated-load-ramp t 8 s; 2D 1:50 to 1:500)

| scale | body before → after | thermal | structure |
|---|---|---|---|
| 1:50 | A- → A- | A- | A- |
| 1:100 | A- → A- | A- | A- |
| 1:200 | A- (upright shoe boxes) → A- | A- | A- |
| 1:500 | B+ (upright 100 x 75 x 75 mm shoe cubes beside the leaning stiles) → A- | A- | A- |

### Changes

- 1:200 and 1:500 shoes: flat sole on the ground, sides leaning with the stile or rail they cap (`coarse_shoe` in the kit), like the real rubber shoe; the side elevation draws the same outline and the plan takes its extent. Volume and drawing decisions updated in both languages.

### Remaining

- At 1:500 treads, platform and grab bar are 75 mm bars (feature floor) on true-size slender stiles; the mix is what the rules force.
- Mass and EN 131 figures rest on computation and knowledge (round 3).
- The ladder is rigid in every pose (the folded state is not authored).
- The after-renders of the leaning shoes (`🗑️generated/r5-stepladder/m-after.png`: 1:200 and 1:500 three-quarter, 2D 1:200 and 1:500) were written but could not be opened: the image viewer refused every read at that moment (hook timeout). The change is checked only by `r3-validate.ts --file ... --volumes` (VALID, every silhouette, bounds and cross-scale rule green) and by the outline arithmetic (the top of each block is shifted by the stile run over its height, 18 mm at 1:200, 31 mm at 1:500).
