# 🧗️ Silhouette Batch R2-O: 8 Objects, Second Round

Fixer report of 2026-11-23 for `🧰️silhouette/R2-O.tsv` (brief `📋️silhouette-brief.md`): the eight objects batch SO left red, in `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🪑️objects`. The ops of batch SO stay in force and run before the ops of this batch. No test, rule, threshold, engine, schema, fixture, README or manifest file was edited.

## Method

- Every check of the real test measured anew per asset with `🧰️silhouette/R2-O/measure.ts` (the test's own `measureAsset`), single views with `one.ts`; the place of a mismatch with `clusters.ts`, `png.ts`, `near.ts`; what the guard compares a slender solid as with `strokes.ts`, `follow.ts`, `overshoot.ts`; what a drawn shape evaluates to in a pose with `rings.ts`.
- New tool `outline.ts`: the outline of body solids projected to a view (exact union, `polygon-clipping`, a tool of the tests) as a filled shape in the coordinates of its bone, or as a stroke along a bar; it probes the mapping through `evaluate` and checks the placed result (off by at most 0.007 mm).
- Every repair is a named, idempotent ops file `🧰️silhouette/R2-O/ops/<id>.json`, written by `make/<id>.sh` or `make/<id>.py` from the state before the repair. The six objects with a generator have the ops hooked as the last step of the generator (`hook.py`, after the step of batch SO where there is one); `gen.sh` runs the generator and copies its document to the catalog (byte-identical, `cmp`). The two older objects (`shopping-trolley`, `stepladder`) have a chain of patch scripts in earlier tickets and no single generator: their repair is `apply.ts <id>`, to run after that chain.
- Proof per asset by `prove.sh <id>`, each run to the end in this session: the real `🧊️catalog-silhouette` test with the name filter, `validateAsset`, the product check CLI, then `🧊️three-views`, `🌡️catalog-physics`, `🫁️breathing-catalog`, `✅️asset-validation` (manifest cases) and `⚡️asset-power` (manifest and catalog-wide cases) with the name filter. Baseline before any change: `validateAsset` 0 issues for all eight.
- `checkVolumes` of every repaired scale read on a work copy (`measure.ts --volumes`) where the body changed (piano, drum kit): no finding.
- Sheets (`look.sh`, the product's own 2D drawings: side, front, plan in line and solid view) rendered after the repair and looked at, whole and cropped (`crop.sh`).

Notation: per view IoU / largest boundary distance; `!` fails (limits: IoU 0.92; distance two smallest features: 15, 30, 60, 150 mm at 1:50, 1:100, 1:200, 1:500).

## Result

- Real `🧊️catalog-silhouette` test, final run with all eight objects in the name filter (`vt.sh final-sil`): `🧊️catalog-silhouette exit 0 … Tests 8 passed | 377 skipped (385)` (before: 8 failed). Failing checks over all scales and views: 30 measured at the start of this round (three more stood behind them: the rim corners of the bath, the leg collars of the piano at 1:50 and 1:100), 0 after.
- Each of the eight ran alone through `prove.sh` after its last change: `🧊️catalog-silhouette exit 0 … Tests 1 passed | 384 skipped (385)`, `validateAsset 0 issues`, `check ✓ <id> (1/1)`, `🧊️three-views … Tests 2 passed`, `🌡️catalog-physics … Tests 2 passed`, `🫁️breathing-catalog … 290 skipped` (no object case), `✅️asset-validation … Tests 9 passed`, `⚡️asset-power … Tests 7 passed`.
- Final run of the other suites with all eight in the name filter (`others.sh final`): `🧊️three-views` 16 passed, `🌡️catalog-physics` 16 passed, `🫁️breathing-catalog` 290 skipped, `✅️asset-validation` 9 passed, `⚡️asset-power` 7 passed: as before. `validateAsset` of all eight after: 0 issues each.
- The six generator documents are byte-identical to their catalog files (`cmp`).
- No declared dimension or power changed: `manifest-changes.json` is `{}`.

## Assets

### `shopping-trolley`

- Before: 1:50 front 0.976/45.0mm!; 1:50 plan 0.997/28.1mm!; 1:100 front 0.965/52.5mm!.
- After: green. 1:50 elevation 0.991/5.3mm, front 0.993/9.4mm, plan 0.997/3.8mm; 1:100 front 0.991/22.5mm; the other views as before.
- Changed (drawing only, 40 ops by `apply.ts`): the front view was a scheme that left parts of the body out. 1:50 front: the hubs and axles of the four castors are drawn (the gap between fork and wheel was open downwards in the drawing and closed by axle and hub in the body); the crate tray, drawn as a hat with its high part in the middle, is drawn as the body has it (floor frame at 0.183 m and rear lip at 0.223 m over the full width as 0.13 mm strokes, the four posts); the four brackets between frame and basket are drawn. 1:50 plan: the four brackets (a slit of 2 mm between frame tube and basket rim was open in the drawing). 1:100 front: the fork plates of the four castors where the body has them (15 mm plates outside the hub, the drawing had a U inside), the hubs, and the tray as at 1:50. Decision texts of 1:50 and 1:100 name the parts.

### `stepladder`

- Before: 1:50 front 0.978/22.5mm!; 1:50 plan 0.968/37.5mm!; 1:100 plan 0.939/30.2mm!; 1:500 front 0.946/213.8mm!.
- After: green. 1:50 front 0.985/13.1mm, plan 0.994/11.3mm; 1:100 plan 0.941/15.0mm; 1:500 front 0.999/6.7mm.
- Changed (drawing only, 28 ops by `apply.ts`): 1:50 front: the hinge bolts are drawn by their outline (they were 0.16 mm strokes beside the rails) and the caps of the rear rails are drawn. 1:50 plan: rear rails, stiles and the four treads are drawn by the outline of their solids (the drawn bands ended 22 mm short and left hairlines to the spreaders and to the rail caps), rail caps, hinge bolts, platform axle and rest tube are drawn. 1:100 plan: rest tube and hinge drawn, the grab bar at its true length. 1:500 front: the front drew two of the four treads, so two openings of 0.15 m2 stood against four of 0.074 m2 that the guard closes: all four treads are drawn (the sheet stays readable, four marks and the platform). Decision texts of 1:50, 1:100 and 1:500 follow.

### `used-baby-grand-piano`

- Before: 1:50 elevation 0.999/20.6mm!; 1:50 front 0.991/28.1mm!; 1:50 plan 0.982/67.5mm!; 1:100 plan 0.981/67.5mm!. Behind them: the plan of 1:50 and 1:100 48.8 mm at the leg collars; 1:200 front 66.4 mm at the lid wear patch once the straps were right.
- After: green. 1:50 elevation 0.999/5.6mm, front 1.000/1.9mm, plan 1.000/0.0mm; 1:100 elevation 1.000/22.8mm, front 1.000/3.8mm, plan 1.000/0.0mm; 1:200 elevation 0.993/32.2mm, front 1.000/0.0mm, plan 0.995/40.4mm; 1:500 1.000/0.0mm, 1.000/0.0mm, 0.999/40.4mm. `checkVolumes` of all four scales: no finding.
- Changed (220 ops, last step of `author-piano.py`): the body was wrong in three places and is repaired first. The two ratchet straps were `slender` solids of 3 x 50 mm webbing: a flat band has no centre line, the guard's stroke wandered 26 mm sideways and ran 28 mm past both ends, below the floor. The webbing is 2.5 mm (the gauge of 50 mm lashing webbing) and a declared `sheet` at all four scales, the drawn bands have the same gauge. The collars of the three legs lay 0.5 mm clear of the edges of the skid (sealed in the body mask, open in the drawing): they are 65.5 mm long and touch the board, body and views. The worn patch on the lid edge, 80 x 200 mm, reached 21 mm past the curved edge of the lid: it is cut to the outline of the lid and reaches further in so that it keeps its 0.016 m2 (the thermal area of `worn-finish` stays within its tolerance), body and views of every scale. Drawing: the plan of 1:50 and 1:100 shows the two hand slots of skid board and carpet (35 x 120 mm) as openings; the shape stays a prism swept through its thickness, the slots are joined to the outline by slits of no width (see requests.md). Decision texts follow.

### `used-basketball-hoop`

- Before: 1:50 front 0.999/16.9mm!; 1:50 plan 0.938/70.8mm!; 1:100 elevation 0.991/47.4mm!.
- After: green. 1:50 front 1.000/3.8mm, plan 0.985/10.9mm; 1:100 elevation 0.997/11.3mm.
- Changed (drawing only, 4 ops, last step of `author-basketball-hoop.py`): the generator drew a repeated or concave solid as the box or the hull of all its copies. 1:50 front: the anchor nuts are four nuts, not a bar over the base plate. 1:50 plan: the two gussets of the ring mount are two 8 mm plates, not a plate of 148 mm between them. 1:100 side elevation: the ring bracket has its concave outline.

### `used-climbing-net-pyramid`

- Before: IoU 0.899 to 0.911! in all nine views of 1:50, 1:100 and 1:200 (boundary distances 2.7 to 15.0 mm); 1:500 plan 0.949/360.7mm!.
- After: green. 1:50 0.989/6.8mm, 0.990/14.6mm, 0.986/1.9mm; 1:100 0.989/8.4mm, 0.990/13.5mm, 0.986/3.8mm; 1:200 0.989/7.5mm, 0.989/7.5mm, 0.986/7.5mm; 1:500 0.990/18.8mm, 0.991/18.8mm, 0.999/18.8mm.
- Changed (drawing only, 780 ops, last step of `author-net-pyramid.py`): every rope is drawn to the round ends of its body. A rope is a slender capsule of 16 mm between two nodes and was drawn as a bar between the nodes; the guard's centre line runs to the cap tips, 7.5 mm past the bar at both ends, just beyond its limit for following the drawn bar, so most ropes were compared at the guard's estimate of their width (14.3 to 16.8 mm) and two ring ropes of the 1:500 plan fell between the cells. The bars run 8 mm past the nodes like the capsules, in the three views of every scale; all ropes are followed at 16 mm (requests.md).

### `used-drum-kit`

- Before: 1:50 elevation 0.979/33.8mm!; 1:50 front 0.964/33.8mm!; 1:100 elevation 0.979/103.0mm!; 1:100 front 0.964/33.8mm!.
- After: green. 1:50 0.999/13.1mm, 0.999/13.1mm, 1.000/9.4mm; 1:100 0.999/11.3mm, 0.999/11.3mm, 1.000/8.4mm; 1:200 and 1:500 as before. `checkVolumes` of all four scales: no finding.
- Changed (41 ops, last step of `author-drum-kit.py`): body: the skirts of the five case lids, bands of 4 mm PE up to 56 mm high, were `slender` rings; the guard draws such a ring as a stroke of the band height with square ends, which stood 28 mm past the case on both sides and closed the gap between two cases from above. At 1:50 and 1:100 the skirts are modelled at the smallest feature (7.5 and 15 mm), grown inwards into the case wall, outline unchanged. Drawing: the four cymbals (crash, ride, hi-hat pair) were drawn in the side and front elevation as their 1.1 mm section, a line where the side shows the bowed body of 35 to 43 mm: they are drawn by the outline of the body at 1:50 and 1:100. Decision texts follow.

### `used-bathtub`

- Before: 1:50 front 0.997/320.6mm! (the open waste hole); behind it 24.4 mm at the two lower corners of the rim.
- After: green. 1:50 elevation 0.987/4.2mm, front 1.000/1.9mm, plan 0.995/11.4mm; the other scales as before.
- Changed (drawing only, 26 ops, last step of `author-used-bathtub.py`, after the generator code of batch SO): a drawing can show a see-through hole. The stored bath leans back 10 degrees and the line of sight through the hole passes the sixteen bands of wall and inside and the floor sheet, each a side profile swept across the width, which cannot carry a hole. In the front view of 1:50 they are prisms swept through their depth whose outline lies in the plane of the bath: the rectangle that projects exactly as the side profile did, with a slot along the sight line (the floor with the round hole itself); hole and outline are one ring joined by a slit of no width, so the view along the sweep has the hole and the views across it are closed. Checked with `rings.ts`: in the poses `in-use` and `carried` every changed shape evaluates to the same rectangle as before. The rim, four strips with square corners, is one such prism with the outline of the rim plate (corners of 50 mm outside, 120 mm inside); its inner edge is a 0.25 mm line of its own (the line view drew two loose bars where the rim happened to lie on the edge of the first inside band). The dark square that stood for the hole is removed. The sheet is better than before: round hole, round rim, whole inner rim line, no seams.

### `used-marching-snare-and-bass-drum`

- Before: 1:200 elevation 0.968/60.5mm!.
- After: green. 1:200 elevation 0.971/24.0mm; the other views and scales as batch SO left them.
- Changed (drawing only, 2 ops, last step of `author-marching-drums.py` after the four ops of batch SO): at 1:200 the two J bars of the bass drum carrier get a 0.11 mm stroke (22 mm, the tube) along their centre line beside the filled band. The guard's centre line of the bent loft is jagged and its width estimate 27.6 and 28.9 mm, which closed the slit between bar and drum shell; with the stroke the tubes are followed at 22 mm (requests.md). Batch SO had tried the stroke at 0.18 mm (36 mm), which closed the slit further.

## Files

Paths are relative to the repository root; `U` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS`, `C` = `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🪑️objects`, `P` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/USED-PARK-OBJECTS-STAFFAGE-COLLECTION/assets`, `M` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/USED-MUSICAL-INSTRUMENTS-STAFFAGE-COLLECTION/assets`, `H` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/USED-HOUSEHOLD-OBJECTS-STAFFAGE-COLLECTION/assets`.

Created:

- `U/silhouette-R2-O.md` (this report).
- `U/🧰️silhouette/R2-O/`, copied from batch SO and adapted: `env.sh`, `measure.ts`, `clusters.ts`, `png.ts`, `solids.ts`, `extents.ts`, `validate.ts`, `ops.ts`, `apply.ts`, `repair.ts`, `people.ts`, `lib.py`, `hook.py`, `show.py`, `try.sh`, `vt.sh`, `others.sh`, `prove.sh`, `look.sh`, `gen.sh`; new: `one.ts`, `near.ts`, `outline.ts`, `rings.ts`, `strokes.ts`, `follow.ts`, `overshoot.ts`, `texts.py`, `pristine.sh`, `crop.sh`, `manifest-changes.json` (`{}`: no declared dimension or power changed).
- `U/🧰️silhouette/R2-O/make/`: `shopping-trolley.sh`, `stepladder.sh`, `used-baby-grand-piano.sh`, `used-baby-grand-piano.py`, `used-basketball-hoop.sh`, `used-climbing-net-pyramid.sh`, `used-climbing-net-pyramid.py`, `used-drum-kit.sh`, `used-drum-kit.py`, `used-bathtub.sh`, `used-bathtub.py`, `used-marching-snare-and-bass-drum.py`.
- `U/🧰️silhouette/R2-O/ops/<id>.json` for the eight objects. The ops file is the repair; the make script wrote it from the state before the repair and is its record.

Updated:

- `C/<id>/🔣️.json` of the eight objects. The six `used-*` files are the byte-identical output of their generators; `shopping-trolley` and `stepladder` are rewritten by `apply.ts` in their own indentation.
- Generators, last step added (it applies `R2-O/ops/<id>.json` to the document the generator wrote): `P/used-basketball-hoop/author-basketball-hoop.py`, `P/used-climbing-net-pyramid/author-net-pyramid.py`, `M/used-baby-grand-piano/author-piano.py`, `M/used-drum-kit/author-drum-kit.py`, `M/used-marching-snare-and-bass-drum/author-marching-drums.py`, `H/used-bathtub/author-used-bathtub.py`; and the `🔣️.json` beside each (their output, identical to the catalog file).
- `U/requests.md`: three sections `## silhouette R2-O — …`.

Removed:

- `U/🗑️generated/silhouette/R2-O` (tool output, work copies, sheets).

## What Stays Red

Nothing of this batch: all eight objects pass the real `🧊️catalog-silhouette` test.

Stated plainly, where this round did less than the brief asks or left something open:

- Sheets: opened and looked at were 1:50 of all eight (bathtub, trolley and stepladder also cropped to the changed view), 1:500 of `stepladder`, `used-climbing-net-pyramid` and `used-baby-grand-piano` (the three whose 1:500 changed), 1:100 of `shopping-trolley` and 1:200 of `used-marching-snare-and-bass-drum`. The 1:500 sheets of the five objects whose 1:500 is unchanged were rendered and not opened; the 1:100 sheets of piano, hoop, stepladder, pyramid and drum kit and the 1:200 sheets of piano and pyramid were not rendered. Nothing looked at was uglier or wrong; bathtub, drum kit (cymbals) and trolley (tray, castors) read better than before.
- `checkVolumes` was read for the two objects whose body changed (piano, drum kit, all four scales: no finding); for the six drawing-only repairs the product check CLI (`✓`) is the evidence.
- `used-baby-grand-piano`: the webbing of the straps is 2.5 mm instead of 3 mm (a thin sheet needs twenty times its thickness across); the declared masses were left as they are (0.31 kg per run, authored estimates on the bones).
- `used-bathtub`: the plan of the stored bath and the views of 1:100 and coarser still draw the waste hole as a dark mark (they pass; at 1:100 the hole is below the size the guard counts).
- `shopping-trolley` and `stepladder` have no single generator: after a re-run of the patch chain of the earlier tickets `apply.ts shopping-trolley stepladder` has to run again.
- The guard defects met on the way are in `requests.md` (three sections `silhouette R2-O`): width and centre line of slender capsules, bent lofts and flat bands, and the hairline seal. Each is worked around in the asset, none is fixed in the engine.
