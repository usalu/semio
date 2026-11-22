# appliances/pendant-lamp — rework report

## Pass 1 (2026-11-22)

Earlier batch report `rework/16.md` section 10: "nothing changed", remaining "the cord is a rigid bone (no swing)". Judged anew below.

### Reference (enamelled dome pendant with E27 lampholder and A60 lamp)

| Feature | Reality | Model before | Model now |
|---|---|---|---|
| Drop ceiling to lamp tip | 0.6–1.2 m | 0.80 m | 0.80 m |
| Ceiling rose | 80–100 mm across, 25–60 mm deep, cord grip 14 mm | 100 x 24 mm, grip 14 x 10 mm | unchanged |
| Cord | round textile 3 x 0.75 mm², 6–7.5 mm | 6.6 mm, 0.6 m | unchanged |
| Suspension | flexible cord, lamp swings about the cord grip; physical pendulum of this mass distribution: T = 2π·sqrt(I / (g·Σm·r)) = 2π·sqrt(0.2455 / (9.807·0.3603)) = 1.66 s | all joints `fixed`, no swing pose or clip | cord joint `revolute` ±20°, poses `swinging` / `swinging-back` (±8°), looping clip `pendulum-swing` 1.68 s |
| Cord-grip collar | brass, 30 mm across, 30 mm high, domed top, M10x1 nipple | 31 x 30 mm | unchanged |
| Shade | enamelled steel 0.6 mm, 250–300 mm across, 120–150 mm deep, rolled rim 5–6 mm | 266 x 138.5 mm, bead 6 mm | unchanged at 1:50 |
| E27 lampholder (IEC 60238) | 39–42 mm across, 60–65 mm long: domed cap on the nipple, seam where cap and skirt screw together, rolled mouth, bore 28–29 mm | plain 40 mm sleeve with one chamfer | domed cap (39 mm), seam bead 42.4 mm, skirt 40 mm, rolled mouth 42.4 mm, bore 28.4 mm |
| A60 lamp (IEC 60064) | 60 mm across, 105–110 mm long, widest 73 mm below the cap base; E27 cap 26.5 x 27 mm | 60 x 105 mm, cap 27 x 27 mm | unchanged at 1:50 |
| Lamp tip against rim | inside or just below in a 133 mm dome | 7.5 mm below | unchanged |

### Defects found (by looking at the sheets and cuts)

- Rig: no pivot at all; the lamp could not swing (also named by the earlier report).
- 1:50 body: the lampholder was a featureless sleeve.
- 1:100 body: the cord started 4 mm under the rose (the cord grip is omitted at this scale), so it floated; the rose edge was a two-step notch.
- 1:100 structure: 9 mm gap between the ceiling pin and the cord member (no hanger through the rose).
- 1:500 body: the dome had five stations and an inner wall that ended lower than the outer one (a lip at the rim) and stopped at 0.26 m instead of 0.266 m; the lamp was a diamond with a pointed tip that ended above the rim although the tip shows below it at every other scale.
- 1:500 thermal: the lampholder region (40 mm) stood out around the 31 mm neck of the shade.
- 1:100 drawing: the rose outline no longer matched the model's chamfer.
- 1:50 body, shade, bulb, collar, rose; 1:50 thermal and structure; 1:200 all lists: judged right for their feature size, left as they are.

### Rebuilt

- Rig (`asset.rig`): `cord` joint revolute −20…+20°. New poses `swinging` (+8°) and key pose `swinging-back` (−8°), thermal binding of `on`. New looping clip `pendulum-swing` (1.68 s, 25 fps, smooth keys, events at the plumb line). `structure.assumptions` and the rig sentence of the notes rewritten (en, de).
- 1:50 body: `socket` re-turned as a real E27 holder (16 hand-set profile points).
- 1:100: `rose` chamfer 7.5 mm, `cord` enters the rose by 2 mm, structure `member-rose-hanger` (15 mm capsule, pin to cord joint).
- 1:500: `shade` re-turned at seven stations of the 1:50 curve with an evenly offset 4 mm shell to the true rim; `bulb` as a pear with a spherical crown (three points on the 30 mm sphere); `th-bulb` to the same envelope; `th-lampholder` 1 mm proud of the neck with a waist where the shade sheet passes.
- 2D: 1:100 `rose` redrawn in elevation and front to the new chamfer. The swing needs no new shapes: every shape is bound to its bone, the side elevation tilts, the front elevation foreshortens (0.79 m) and the plan shows the shade moved off the rose (read in `r3-2d --pose swinging` at 1:50 and 1:200 and in the clip at 1.26 s).
- Decisions: volume decisions of 1:50, 1:100 and 1:500 extended or rewritten (en, de).
- Chisel: `rework-pendant-lamp-pass1.py` (states every number). Reading aid: `rework-pendant-lamp-montage.ts` (joins crops of sheets into one image).

### Rendered and read after the last change

1:50 body six views (`on`, `swinging`), right-side cuts of body, thermal and structure at 1:50, 1:100, 1:200, 1:500, three-quarter views in `swinging` at 1:100, 1:200, 1:500, six views at 1:500, poses `off`, `dimmed`, `swinging-back`, clips `switch-on-warm-up` (2 s, 100 s), `switch-off-cool-down` (2 s, 100 s), `pendulum-swing` (0.42 s, 1.2 s), 2D at 1:50 (pose and clip), 1:100, 1:200.

### Validation

`r3-validate.ts appliances/pendant-lamp --catalog-only --volumes`: `VALID appliances/pendant-lamp (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0`.

### What remains imperfect

- The swing is one rigid pendulum in the elevation plane (the rig is planar); the cord does not flex and there is no swing across the view.
- The lamp glass is an opaque turned solid; stem and filament exist only in the 1:50 drawing.
- The rose is the shallow 24 mm type (real, but at the lower end); kept because height, cord length and all drawings hang on it.
- 1:200 and 1:500 thermal cut: the neck of the bell shows in the shade colour, the lampholder region is only the 1 mm proud sleeve around it.
- 1:500 has no 2D drawing (`unavailable`, 1.6 mm on paper), unchanged.

### Fundamental issues

- `r3-project2d.ts --check` leaves `slender` solids out of the projection but removes their 2D shape, so every asset with a slender member reports a false low silhouette IoU (here 0.84–0.87 with a Hausdorff distance of 0.30 m, exactly half the cord). The per-shape round trips are right (0.93–0.995).
