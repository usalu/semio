# 🪑️ Silhouette Batch SO: 27 Objects

Fixer report of 2026-11-23 for `🧰️silhouette/SO.tsv` (brief `📋️silhouette-brief.md`). Scope: the 27 objects of the batch in `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🪑️objects`. No test, rule, threshold, engine, schema, fixture, README or manifest file was edited.

## Method

- Every check of the real test measured per asset with `🧰️silhouette/SO/measure.ts` (the test's own `measureAsset`, `featureOf`: bounding box per axis, IoU and largest boundary distance per authored scale and view), the place of a mismatch with `clusters.ts`, `png.ts` and `solids.ts`.
- Every repair is a named, idempotent ops file `🧰️silhouette/SO/ops/<id>.json`, written by `🧰️silhouette/SO/make/<id>.py` and applied by `apply.ts` (reads the catalog file right before it writes). None of the 27 assets has a live generator that would overwrite the file: where a ticket holds an authoring script, the ops file is the last step to run after it.
- Proof per asset by `prove.sh <id>`, each run to the end in this session: the real `🧊️catalog-silhouette` test with the name filter, `validateAsset`, the product check CLI, then `🧊️three-views`, `🌡️catalog-physics`, `🫁️breathing-catalog`, `✅️asset-validation` (manifest cases) and `⚡️asset-power` (manifest and catalog-wide cases) with the name filter. Baseline of the 27 before any change: `validateAsset` 0 issues for all; three-views 54 of 54, catalog-physics 54 of 54, asset-validation 9 of 9, asset-power 7 of 7 passed; breathing-catalog has no case for an object (290 skipped).
- Sheets (`look.sh`, the product's own 2D drawings: side, front, plan in line and solid view) rendered after the repair at 1:50 and 1:500 and looked at.

Notation of the measured lines: `bbox` width/height/depth body against declared; per view IoU / largest boundary distance; `!` fails (limits: one smallest feature 7.5, 15, 30, 75 mm per axis; IoU 0.92; distance two features 15, 30, 60, 150 mm).

## Result and Final Proof

- Real `🧊️catalog-silhouette` test, final run with all 27 objects in the name filter (`vt.sh final-sil`): **19 passed, 8 failed** (before: 27 failed). Green: `bicycle-stand`, `chalkboard`, `charcoal-grill`, `drinking-fountain`, `market-parasol`, `patio-heater`, `playground-swing`, `street-lamp`, `tealight`, `used-bicycle-shelter`, `used-dog-waste-station`, `used-double-bass`, `used-football-goal`, `used-recycling-bin-cluster`, `used-seesaw`, `used-slide-tower`, `used-upright-piano`, `used-viola`, `used-water-play-table`.
- Each of the 21 touched assets also ran alone through `prove.sh` after its last change: the 19 green ones `🧊️catalog-silhouette exit 0 … Tests 1 passed | 384 skipped (385)`, `validateAsset 0 issues`, `check ✓ <id> (1/1)`; `used-bathtub` and `used-marching-snare-and-bass-drum` the same except the silhouette test (see "What Stays Red").
- Final run of the other suites with all 27 in the name filter (`others.sh final`): `🧊️three-views` 54 passed, `🌡️catalog-physics` 54 passed, `🫁️breathing-catalog` 290 skipped (no object case), `✅️asset-validation` 9 passed, `⚡️asset-power` 7 passed: as before.
- `validateAsset` of all 27 after: 0 issues each.
- Failing checks (bounding box, IoU, boundary distance over all scales and views) of the 27: 66 before, 30 after, all 30 in the eight red assets (28 of them in the six untouched ones).

## Assets

### `bicycle-stand`

- Before: 1:500 front 0.834!/6.3mm; 1:500 plan 0.845!/6.6mm.
- After: green.
- Changed: 1:500: the tube of the print floor is an oval of 75.2 mm in the plane of the hoop and 63 mm across (it was round, 75.2 mm, against bars of 63 mm in front and plan, which the lateral extent rule caps); the front also draws the far leg and the cross rail behind the near leg, so `volumes-feature` reads 63 mm for every bone; decisions texts of 1:500 say so.

### `chalkboard`

- Before: 1:50 plan 0.971/22.5mm!; 1:100 front 0.902!/15.0mm.
- After: green.
- Changed: 1:50: the ruler is a 7.5 mm board as the elevation draws it and no `slender` strip (its stroke stood up to 22 mm outside the tray, requests.md). 1:100: the end view draws the leaf where its body is (3.5 mm towards the room, lower edge at 68 mm).

### `charcoal-grill`

- Before: 1:50 bbox 0.6095/0.6! 1.0320/1.03 0.6704/0.67.
- After: green.
- Changed: 1:50: the cap of the front foot is 27 mm across on the 25 mm leg instead of 33 mm, in the volume and the three views; the body is 0.6064 m wide. No declared dimension changed.

### `drinking-fountain`

- Before: 1:100 elevation 0.994/31.8mm!; 1:200 elevation 0.991/71.8mm!.
- After: green.
- Changed: 1:100, 1:200, 1:500: the water jet is a `texture` solid, so its stroke in elevation and front is drawn in the `texture` role (at 1:200 the stroke closed the arch of the bubbler into a filled area).

### `market-parasol`

- Before: 1:50 front 0.997/18.8mm!; 1:100 bbox 2.5523/2.59! 2.4625/2.46 2.5523/2.59!; 1:200 bbox 2.5523/2.59! 2.4625/2.46 2.5523/2.59!.
- After: green.
- Changed: 1:50: the front draws the pin of the tilt knuckle. 1:100, 1:200: the rib rods run to the end of the rib (tip cap merged) in the volume and both elevations, body 2.591 m across.

### `patio-heater`

- Before: 1:50 elevation 0.994/26.3mm!.
- After: green.
- Changed: 1:50 side elevation: the wheel bracket of the volume is drawn between wheel and housing; the two visible reflector studs are drawn on the side where the volume has them.

### `playground-swing`

- Before: 1:50 bbox 2.1800/2.18 2.5135/2.5! 1.9000/1.9; 1:50 front 0.938/17.3mm!.
- After: green.
- Changed: 1:50: the through bolts of the apex sleeves lie horizontal like the apex bolts (body 2.5072 m high instead of 2.5135 m), the hanger bands have the wall of the sleeves; the front draws the hook shank at 13 mm, starts the chain band at the lower eye and draws the two cheeks of each hanger fork apart.

### `shopping-trolley`

- Before: 1:50 front 0.976/45.0mm!; 1:50 plan 0.997/28.1mm!; 1:100 front 0.965/52.5mm!.
- After: RED: 1:50 front 0.976/45.0mm!; 1:50 plan 0.997/28.1mm!; 1:100 front 0.965/52.5mm!.
- Changed: Not repaired. Measured mismatches at 1:50: castor forks and axle bolts, tray lip and strut in the front, rim corner in the plan (several places of 20 to 50 mm each). The greedy repair `auto.ts` needed more than 15 minutes per step on this wire asset and found no accepted step.

### `stepladder`

- Before: 1:50 front 0.978/22.5mm!; 1:50 plan 0.968/37.5mm!; 1:100 plan 0.939/30.2mm!; 1:500 front 0.946/213.8mm!.
- After: RED: 1:50 front 0.978/22.5mm!; 1:50 plan 0.968/37.5mm!; 1:100 plan 0.939/30.2mm!; 1:500 front 0.946/213.8mm!.
- Changed: Not repaired (not reached within this session). Measured places: 1:50 front, hinge and rail caps (22 mm); 1:50 and 1:100 plan, rear rails and spreaders beside the platform (30 to 37 mm); 1:500 front, a tread (214 mm).

### `street-lamp`

- Before: 1:50 elevation 0.994/18.5mm!; 1:50 front 0.997/15.1mm!.
- After: green.
- Changed: 1:50: the six cooling fins of the head are a `texture` solid (4 mm plates), so their strokes in the three views are drawn in the `texture` role.

### `tealight`

- Before: 1:50 elevation 0.914!/2.9mm; 1:50 front 0.914!/2.9mm.
- After: green.
- Changed: 1:50: the wick is drawn as the filled 2 mm strand of the volume instead of a stroke of 6.5 mm; the wax line lies 1.75 mm lower so that its stroke ends at the cup rim.

### `used-baby-grand-piano`

- Before: 1:50 elevation 0.999/20.6mm!; 1:50 front 0.991/28.1mm!; 1:50 plan 0.982/67.5mm!; 1:100 plan 0.981/67.5mm!.
- After: RED: 1:50 elevation 0.999/20.6mm!; 1:50 front 0.991/28.1mm!; 1:50 plan 0.982/67.5mm!; 1:100 plan 0.981/67.5mm!.
- Changed: Not repaired (not reached within this session).

### `used-basketball-hoop`

- Before: 1:50 front 0.999/16.9mm!; 1:50 plan 0.938/70.8mm!; 1:100 elevation 0.991/47.4mm!.
- After: RED: 1:50 front 0.999/16.9mm!; 1:50 plan 0.938/70.8mm!; 1:100 elevation 0.991/47.4mm!.
- Changed: Not repaired (not reached within this session). Measured places: 1:50 front at the anchor nuts of the base plate (17 mm), 1:50 plan at ring mount and ring rod (71 mm), 1:100 side elevation at the ring bracket (47 mm).

### `used-bathtub`

- Before: 1:50 front 0.997/320.6mm!; 1:50 plan 0.991/20.6mm!.
- After: RED: 1:50 front 0.997/320.6mm!.
- Changed: In the generator (`author-used-bathtub.py`), 1:50: eight wall bands instead of four in front and plan, and the lowest band, the lowest inside band and the floor step in at both ends like the rounded corners of the bottom sheet (`coved`); the plan passes (11.4 mm). The front stays red at the open waste hole, see below and requests.md.

### `used-bicycle-shelter`

- Before: 1:50 elevation 0.990/84.6mm!; 1:500 front 0.884!/187.5mm!.
- After: green.
- Changed: 1:500: the front draws the three purlins as 1:200 does (the open strip between gutter and panels). 1:50 side elevation: tyre outline 0.5 mm smaller (an opening at the 256-cell limit, requests.md); the paper depot tag is drawn in the `texture` role.

### `used-climbing-net-pyramid`

- Before: 1:50 elevation 0.911!/6.8mm; 1:50 front 0.911!/14.6mm; 1:50 plan 0.904!/2.7mm; 1:100 elevation 0.911!/8.4mm; 1:100 front 0.911!/13.5mm; 1:100 plan 0.904!/5.3mm; 1:200 elevation 0.906!/15.0mm; 1:200 front 0.906!/15.0mm; 1:200 plan 0.899!/10.6mm; 1:500 plan 0.949/360.7mm!.
- After: RED: 1:50 elevation 0.911!/6.8mm; 1:50 front 0.911!/14.6mm; 1:50 plan 0.904!/2.7mm; 1:100 elevation 0.911!/8.4mm; 1:100 front 0.911!/13.5mm; 1:100 plan 0.904!/5.3mm; 1:200 elevation 0.906!/15.0mm; 1:200 front 0.906!/15.0mm; 1:200 plan 0.899!/10.6mm; 1:500 plan 0.949/360.7mm!.
- Changed: Not repaired (not reached within this session). Measured now: IoU 0.899 to 0.911 in all views of 1:50, 1:100 and 1:200 and 361 mm in the plan of 1:500 (the suite of 16:27 named only the latter).

### `used-dog-waste-station`

- Before: 1:50 elevation 1.000/42.3mm!.
- After: green.
- Changed: 1:50: the mounting plate between post and bin runs to the top of the bin (0.41 m) in the volume, the side elevation and the front.

### `used-double-bass`

- Before: 1:50 plan 0.980/61.2mm!; 1:100 plan 0.980/60.5mm!; 1:200 plan 0.980/74.2mm!.
- After: green.
- Changed: 1:50, 1:100, 1:200 plan: the neck sleeve of the cover is drawn as lateral slices that follow the width of its loft instead of one block; 1:50 plan: the heel of the neck ends its full width where the cover band ends.

### `used-drum-kit`

- Before: 1:50 elevation 0.979/33.8mm!; 1:50 front 0.964/33.8mm!; 1:100 elevation 0.979/103.0mm!; 1:100 front 0.964/33.8mm!.
- After: RED: 1:50 elevation 0.979/33.8mm!; 1:50 front 0.964/33.8mm!; 1:100 elevation 0.979/103.0mm!; 1:100 front 0.964/33.8mm!.
- Changed: Not repaired (not reached within this session).

### `used-football-goal`

- Before: 1:50 elevation 0.993/24.4mm!.
- After: green.
- Changed: 1:50 side elevation: the sagging back mesh is drawn by the outline of its volume (26 points from the rows of the mesh polyhedron).

### `used-marching-snare-and-bass-drum`

- Before: 1:200 elevation 0.968/60.5mm!; 1:500 elevation 0.908!/55.5mm.
- After: RED: 1:200 elevation 0.968/60.5mm!.
- Changed: 1:500 side elevation: a stroke of 0.13 mm along the centre line of each of the four J bars beside its filled band; IoU 0.964. 1:200 side elevation stays red by 0.5 mm, see below.

### `used-recycling-bin-cluster`

- Before: 1:50 front 0.988/15.9mm!.
- After: green.
- Changed: 1:50: the front draws the six trunnion bosses and pins between bins and posts.

### `used-seesaw`

- Before: 1:50 plan 0.993/20.6mm!.
- After: green.
- Changed: In the generator (`author-seesaw.py`): the hollow of the old seat was drawn 0.28 m towards the pivot instead of along the seat (a sign), which made an outline of 20 mm beside the beam in the plan of 1:50.

### `used-slide-tower`

- Before: 1:200 bbox 5.9063/5.906299 4.3553/4.427342! 1.6000/1.6.
- After: green.
- Changed: 1:200: the two lifting eyes on the ridge are kept as lugs of 30 x 62 x 30 mm in the volume, its thermal list and the three views; body 4.417 m high.

### `used-upright-piano`

- Before: 1:50 front 0.997/80.6mm!.
- After: green.
- Changed: 1:50: the front draws the dark backs of the three pedal slots of the knee board.

### `used-viola`

- Before: 1:50 plan 0.994/28.1mm!.
- After: green.
- Changed: 1:50 plan: the case handle is drawn as grip and two tabs with the finger slot open; the last 18 mm of the slit under the raised lid are drawn closed.

### `used-water-play-table`

- Before: 1:50 plan 0.998/46.9mm!; 1:100 elevation 0.990/41.2mm!.
- After: green.
- Changed: 1:50: the plan draws the strap of the chute. 1:100: the two sluice gates are 15 mm plates instead of 3 mm sheets, the thickness their strokes draw.

## Files

Paths are relative to the repository root; `U` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS`, `C` = `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🪑️objects`, `P` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/USED-PARK-OBJECTS-STAFFAGE-COLLECTION/assets`, `M` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/USED-MUSICAL-INSTRUMENTS-STAFFAGE-COLLECTION/assets`, `H` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/USED-HOUSEHOLD-OBJECTS-STAFFAGE-COLLECTION/assets`.

Created:

- `U/silhouette-SO.md` (this report).
- `U/🧰️silhouette/SO/`: `env.sh`, `measure.ts`, `clusters.ts`, `png.ts`, `solids.ts`, `extents.ts`, `validate.ts`, `auto.ts`, `ops.ts`, `people.ts`, `apply.ts`, `repair.ts`, `lib.py`, `hook.py`, `show.py`, `report.py`, `try.sh`, `vt.sh`, `others.sh`, `prove.sh`, `look.sh`, `gen.sh`, `manifest-changes.json` (`{}`: no declared dimension or power changed).
- `U/🧰️silhouette/SO/make/<id>.py` and `U/🧰️silhouette/SO/ops/<id>.json` for: `bicycle-stand`, `chalkboard`, `charcoal-grill`, `drinking-fountain`, `market-parasol`, `patio-heater`, `playground-swing`, `street-lamp`, `tealight`, `used-bicycle-shelter`, `used-dog-waste-station`, `used-double-bass`, `used-football-goal`, `used-marching-snare-and-bass-drum`, `used-recycling-bin-cluster`, `used-slide-tower`, `used-upright-piano`, `used-viola`, `used-water-play-table`. The ops file is the repair; the make script wrote it from the state before the repair and is its record.

Updated:

- `C/<id>/🔣️.json` of the 19 assets above and of `used-bathtub` and `used-seesaw` (21 catalog files). The nine older objects are rewritten by `apply.ts` in their own indentation (JSON.stringify layout); the twelve `used-*` files are the byte-identical output of their generators.
- Generators, last step added (it applies `ops/<id>.json` to the document the generator wrote; `hook.py`, for the viola by hand in TypeScript through `repair.ts`): `P/used-slide-tower/author-slide-tower.py`, `P/used-recycling-bin-cluster/author-build.py`, `P/used-dog-waste-station/author-used-dog-waste-station.py`, `P/used-football-goal/author-football-goal.py`, `P/used-water-play-table/author-water-play-table.py`, `P/used-bicycle-shelter/author-bicycle-shelter.py`, `M/used-double-bass/author-double-bass.py`, `M/used-marching-snare-and-bass-drum/author-marching-drums.py`, `M/used-viola/author-viola.ts`, `H/used-upright-piano/author-piano.py`.
- Generators, repaired in their own code: `H/used-bathtub/author-used-bathtub.py` (`clip_x`, `corner_half`, `coved`, eight wall bands at 1:50), `P/used-seesaw/author-seesaw.py` (sign of the seat hollow).
- The `🔣️.json` beside each of these twelve generators (their output, identical to the catalog file).
- `U/requests.md`: three sections `## silhouette SO — …`.

Removed:

- `U/🗑️generated/silhouette/SO` (tool output, backups, sheets).

## What Stays Red

Eight of the 27 objects still fail the real `🧊️catalog-silhouette` test (final run of this session with all 27 in the name filter: 19 passed, 8 failed; before: 0 passed, 27 failed). The numbers are in the asset section above (line "After").

- `used-bathtub`: 1:50 front, 320.6 mm against 15 mm at the open 52 mm waste hole. A drawing cannot show a see-through hole (requests.md); the plan of 1:50 is repaired (20.6 mm to 11.4 mm).
- `used-marching-snare-and-bass-drum`: 1:200 side elevation, 60.5 mm against 60 mm at the slit between a carrier bar and the bass drum shell (closed in the body mask, open in the drawn mask); 1:500 is repaired (IoU 0.908 to 0.964).
- `shopping-trolley`, `stepladder`, `used-baby-grand-piano`, `used-basketball-hoop`, `used-climbing-net-pyramid`, `used-drum-kit`: not repaired, their files are untouched. Each has several independent mismatches (three to ten failing checks); the session's budget went into the 21 assets above. They need a fixer of their own.

Not done as the brief asks, stated plainly:

- The sheets of 1:50 and 1:500 were rendered for every repaired asset; opened and looked at were 1:50 of all 21 (small, 560 to 1000 px wide, so a change of a few millimetres is not visible in them), and 1:500 of `bicycle-stand`, `chalkboard`, `charcoal-grill`, `drinking-fountain`, `market-parasol`, `patio-heater`, `used-bicycle-shelter`, `used-marching-snare-and-bass-drum`, `used-slide-tower`; the other 1:500 sheets were not opened (their 1:500 representation is unchanged, `tealight` and `used-viola` have none). Nothing looked at was uglier or wrong.
- `checkVolumes` was read per repaired scale on the work copy (`try.sh`): no new finding; `chalkboard` keeps the `volumes-connected` finding of 1:50 it had before ("header-stops" 2 mm from "header-track"). A full `checkVolumes` of all four scales after the repair was not run for every asset.
- The nine older objects have no single generator but a chain of patch scripts in earlier tickets; their repair is `apply.ts <id>` with the ops file, to run after that chain.
