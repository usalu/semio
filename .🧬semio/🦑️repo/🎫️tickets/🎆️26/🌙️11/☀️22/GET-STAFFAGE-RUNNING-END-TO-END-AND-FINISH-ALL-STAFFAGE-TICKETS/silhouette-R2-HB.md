# 🧗️ Silhouette Batch R2-HB: 7 Humans, Second Round

Fixer report of 2026-11-23 21:45 to 2026-11-24 02:20 for `🧰️silhouette/R2-HB.tsv` (brief `📋️silhouette-brief.md`): the seven people batch SH-2 left red. Paths: `S` = `🎓️teaching/🏛️architecture/🧍️staffage`, `U` = this ticket folder, `K` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE`, `W` = `U/🧰️silhouette/R2-HB`. The ops of batch SH-2 stay in force and run before the ops of this batch. No test, rule, threshold, engine, schema, fixture, README or manifest file was edited.

## Result

- All seven pass the real `🧊️catalog-silhouette` test. Final run with the seven in the name filter: `🧊️catalog-silhouette exit 0 Test Files 1 passed (1); Tests 7 passed | 378 skipped (385)` (before: 7 failed). Failing checks measured at the start of this round: 19; after: 0.
- Other suites, final run with the seven in the name filter: `🧊️three-views exit 0 … Tests 14 passed | 761 skipped (775)`, `🌡️catalog-physics exit 0 … Tests 21 passed | 819 skipped (840)`, `🫁️breathing-catalog exit 0 … Tests 21 passed | 269 skipped (290)`, `✅️asset-validation exit 0 … Tests 9 passed | 74 skipped (83)` (manifest cases), `⚡️asset-power exit 0 … Tests 7 passed | 17 skipped (24)` (manifest and catalog-wide cases).
- Per person after the last change: `validateAsset` 0 issues (0 before), three-views sweep and consistency 0 findings (0 before), product check CLI `✓ <id> (1/1)`, for all seven.
- No declared dimension or power changed: `W/manifest-changes.json` is `{}`.
- Interruptions: the dependency image of the host (`.cache/b/d/fixed.dmg`) was detached from 22:21 to about 01:35 (entries "silhouette R3-M" and "R3-V" in `requests.md`); in that time no vitest suite, no sheet and no tool that needs `polygon-clipping` could run. I called `repo doctor` and `repo dependencies ensure` once each (01:30, both failed; someone else was running `fsck_apfs` on the image) and changed nothing else. After the coordinator's notice the three files changed until then were checked first (guard measure, `validateAsset`, sweep, check CLI: all intact).

## Method

- Every check measured anew per person with `measure.ts` (the test's own `measureAsset`); place and cause of a mismatch with `clusters.ts` (`HOLES=1` lists the pockets each mask closes), `png.ts`, `near.ts`, `strokes.ts`, `follow.ts`, `rings.ts`, `shapes.py` (drawn shapes against solids), `spread.ts` (which shapes make the extents the three-views sweep compares in a pose).
- `outline.ts` (from batch R2-O) got three ways to draw a part from the outline of its solids so that it holds in every pose: `keep` (the shape keeps its sweep, the outline becomes its profile), `sheet` (a plate of 0.8 mm on the bone) and `prism` (a prism on the bone of the solid, swept along the bone axis nearest to the viewing direction; the profile is the outline shrunk along the sweep as the view sees it, so the prism covers the outline again; `fit=<mm>` takes the deepest prism that still covers the outline to that distance). Plates alone passed the silhouette but gave the three-views sweep three findings on `skateboarder` (front and plan extents apart by up to 98 mm in twisted poses); prisms with depth do not. `centre.ts` draws the stroke of a slender tube along the centre line the guard compares it by.
- Every repair is a named, idempotent ops file `W/ops/<id>.json`, written by `W/make/<id>.sh` or `.py` from the catalog file and applied by `apply.py` (rewrites only the text spans it changes). `worker-with-pallet-jack` has a generator chain: its ops are also the last ops of `K/🧰️scripts/HW-18/repair/worker-with-pallet-jack.json` (`chain.py`); the chain's applier on the state before gives the catalog document (compared equal).
- `trial.sh` tries ops on a copy (guard, `validateAsset`, sweep) before the catalog file is touched; `prove.sh` and `others.sh` run the proofs.

Notation: per view IoU / largest boundary distance; `!` fails (limits: IoU 0.92; distance 15, 30, 60, 150 mm at 1:50, 1:100, 1:200, 1:500).

## Assets

### `seated-guitarist`

- Before: 1:50 plan 0.966/23.2mm!.
- After: green. 1:50 elevation 0.981/11.4mm, front 0.986/11.3mm, plan 0.978/11.9mm; 1:100 plan 0.972/23.7mm (was 0.959/23.7mm); 1:200 and 1:500 unchanged.
- Cause and change (volume, 48 ops): the six strings were slender lofts bent at the nut; the guard compared them as strokes 9 to 49 mm wide (true 0.6 to 1.3 mm), a band of 23 mm beside the neck in the plan. Each string is two straight lofts at 1:50 and 1:100 (speaking length, and nut to tuner post as `<id>-head`) with thermal and structure entries like the string. The drawing is unchanged. `checkVolumes` was not read separately; the check CLI is `✓`.

### `seated-office-worker`

- Before: 1:50 elevation 0.976/71.3mm!; 1:200 elevation 0.965/66.3mm!; 1:500 elevation IoU 0.916!/44.2mm.
- After: green. 1:50 elevation 0.989/10.1mm; 1:100 elevation 0.987/22.8mm (was 26.5); 1:200 elevation 0.973/47.0mm; 1:500 elevation 0.930/19.9mm; front and plan as before.
- Cause and change (drawing only, 73 ops): the four castors and arms of the chair base that stand 72 and 144 degrees to the view were drawn in the side elevation as profiles of the bone plane and stood up to 70 mm beside their solids, at every scale (hidden behind the shoes at one side). Each part is drawn by the outline of its solid (the base never moves in a pose or clip), the stems of the castors get shapes at 1:50, the summary outline SH-2 had laid over one castor is removed. With the base right the raster of 1:100 moved and the pocket under the typing hands measured 53 mm: the hand sheets of the 1:100 side elevation drew the fingers 15 mm thinner than the body; their profile is the outline of hand, fingers and thumb.

### `skateboarder`

- Before: 1:50 elevation 0.950/35.8mm!, front 0.938/26.1mm!, plan 0.966/18.8mm!.
- After: green. 1:50 elevation 0.994/9.4mm, front 0.999/2.7mm, plan 0.995/10.1mm; other scales unchanged.
- Cause and change (drawing only, 75 ops): the figure stands turned on the board; limbs drawn as bodies of revolution and torso, head and shoes drawn as thick prisms stood 19 to 36 mm beside their lofts. Every part of the figure is a prism on its bone with the outline of its solids in each view; neck and nose get shapes; the stacked slices of hoodie, head and beanie of the plan give way to one outline each. The legs of the plan stay the bodies of revolution they were (seen end-on, a short prism does not follow a lifted knee: sweep finding in pose `ollie-air`). The knit hatch of the beanie is kept.

### `tour-guide`

- Before: 1:50 elevation 0.962/26.3mm!, front 0.955/28.1mm!, plan 0.935/25.2mm!.
- After: green. 1:50 elevation 0.981/11.9mm, front 0.972/12.0mm, plan 0.983/11.9mm; other scales unchanged. `checkVolumes` of 1:50: no finding.
- Cause and change (28 ops): sleeves, forearms, hands, jacket and skirt (with its hem) are prisms with the outline of their solids; the badge is drawn where its solid hangs (2 mm off, it closed the slit beside the hand); the flag of the plan by its outline. Volume: the flag pole of 24 mm was marked `slender` and compared as a stroke 33 mm wide that ran 13 mm past the ferrule; it is three smallest features thick and loses the mark.

### `wheelchair-user`

- Before: 1:50 elevation 0.984/36.4mm!, front 0.983/22.5mm!, plan 0.993/22.5mm!; 1:200 elevation 0.950/112.5mm!.
- After: green. 1:50 elevation 0.990/7.7mm, front 0.983/15.0mm, plan 0.993/15.0mm; 1:200 elevation 0.971/16.9mm; 1:100 and 1:500 unchanged. `checkVolumes` of all four scales: no finding.
- Cause and change (20 ops): 1:50: the six tabs that carry each hand rim were not drawn, so the slit between tyre and hand rim was open in front and plan and a row of closed pockets in the body: drawn as prisms along the wheel axis in all three views (two shapes of three per side, see `requests.md`). The hoodie of the side elevation stood 21 mm off at the neck: prism with its outline. Volume: the two drawstrings (bowed slender lofts of 4.8 mm) were compared as strokes 33.6 mm wide; they are typified to the smallest feature (7.5 mm) and no longer `slender`. 1:200 side elevation: axle plates and seat sling were not drawn (62 mm); the side tubes are drawn along the centre line the guard compares them by.
- Stated plainly: front and plan of 1:50 stand at 15.0 mm against the limit of 15 mm, and the 1:200 side elevation passes because the pocket under the knee is 456 cells in the drawing and 461 in the body at a limit of 455; one cell less and it measures 112 mm again. Both pass the real test; the second is a property of the guard reported in `requests.md`.

### `worker-carrying-box`

- Before: 1:50 elevation 0.991/107.4mm!, front 0.982/16.9mm!, plan 0.967/18.8mm!.
- After: green. 1:50 elevation 0.993/13.3mm, front 0.986/13.1mm, plan 0.983/10.9mm; other scales unchanged. `checkVolumes` of 1:50: no finding.
- Cause and change (8 ops): volume: the 20 kg carton was an empty shell, so the side elevation saw through both hand holes where the drawing shows the hand hole as a dark patch on the closed carton (at 1:100 the carton is a block). The carton gets its load at 1:50, one block inside the board, 60 mm clear of the end walls for the fingers, with a structure entry; the masses on the bones are unchanged. Drawing: the boots of the front view and the upper arms of the plan are prisms with the outline of their solids.

### `worker-with-pallet-jack`

- Before: 1:50 elevation 0.996/15.9mm!, plan 0.987/29.3mm!.
- After: green. 1:50 elevation 0.998/8.4mm, front 0.995/12.0mm, plan 0.997/11.3mm; other scales unchanged.
- Cause and change (drawing only, 11 ops): the gloves of the side elevation, and in the plan the right sleeve, forearm and cuff strap, both hands and the two bows of the tiller head (round tubes drawn as square prisms 6 mm too wide) are prisms with the outline of their solids; the cargo pockets on the thighs, which the plan did not draw (17 mm), get shapes.

## Sheets

Rendered with `look.sh` (the product's own 2D drawings: side, front, plan in line and solid view) and opened: 1:50 after of all seven (with the 1:50 before of `seated-office-worker`, `skateboarder` and `tour-guide`), 1:100 and 1:200 of `seated-office-worker`, 1:200 of `wheelchair-user`, 1:500 of `seated-office-worker` whole; of the other six 1:500 sheets (their 1:500 drawing is unchanged) a strip of the solid row was opened, not the whole sheet. Nothing looked at is uglier or wrong; the chair base of the office worker is right now (five castors under the arms), the flag of the tour guide's plan reads as cloth. Not rendered: 1:100 of `seated-guitarist` (drawing unchanged).

## Files

Paths relative to the repository root; `C` = `S/🗂️catalog/👤️humans`.

Updated:

- `C/<id>/🔣️.json` for `seated-guitarist`, `seated-office-worker`, `skateboarder`, `tour-guide`, `wheelchair-user`, `worker-carrying-box`, `worker-with-pallet-jack`.
- `K/🧰️scripts/HW-18/repair/worker-with-pallet-jack.json` (11 ops of this round appended as its last ops).
- `U/requests.md`: one section `## silhouette R2-HB — …`.

Created:

- `U/silhouette-R2-HB.md` (this report).
- `W/`: `env.sh`, `people.ts`, `ops.ts`, `lib.py`, `apply.py`, `sh2_doc.py`, `join.py`, `chain.py`, `measure.ts`, `one.ts`, `clusters.ts`, `png.ts`, `near.ts`, `solids.ts`, `strokes.ts`, `follow.ts`, `rings.ts`, `spread.ts`, `centre.ts`, `outline.ts`, `shapes.py`, `validate.ts`, `sweep.ts`, `t.sh`, `gate.sh`, `vt.sh`, `trial.sh`, `prove.sh`, `others.sh`, `png.sh`, `look.sh`, `crop.sh`, `manifest-changes.json` (`{}`).
- `W/make/`: `seated-guitarist.py`, `seated-office-worker.sh`, `skateboarder.sh`, `tour-guide.sh`, `tour-guide.py`, `wheelchair-user.sh`, `wheelchair-user.py`, `worker-carrying-box.sh`, `worker-carrying-box.py`, `worker-with-pallet-jack.sh`.
- `W/ops/`: `<id>.json` for the seven, and the parts `wheelchair-user.hand.json`, `wheelchair-user.outline.json`, `worker-carrying-box.hand.json`, `worker-carrying-box.outline.json` that `join.py` merges.

Removed:

- `U/🗑️generated/silhouette/R2-HB` (backups, logs, overlays, sheets, trial copies).

## What Stays Red

Nothing of this batch: all seven pass the real `🧊️catalog-silhouette` test.

Open, stated plainly:

- The make scripts read the catalog file as it is now (outlines come from the solids, properties from the shape they replace); the ops files are the repair. For the six people without a generator the state before this round is the SH-2 state; after a re-run of older patch chains `apply.py <id>` has to run again after the SH-2 ops.
- `wheelchair-user`: two checks pass at their limit (see above).
- `seated-guitarist` 1:100: the decision text of the 1:100 volume says the strings are left out while the body has them (as before this round); not touched.
- The prisms follow their bones in every pose, but a pose that turns a limb far out of its default direction shows the bounding prism of the part, not its round body; the three-views sweep (every pose and clip instant) reports nothing for the seven.
- Decision texts of the representations were not extended for the redrawn parts.
