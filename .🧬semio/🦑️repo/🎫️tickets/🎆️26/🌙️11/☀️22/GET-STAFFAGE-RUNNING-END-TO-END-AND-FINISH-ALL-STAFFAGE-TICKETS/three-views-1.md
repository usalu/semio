# 🧊️ Three-Views Guard, Part 1: Eight Older Assets

Scope: the first eight of the fifteen "Older assets" of `older-tickets.md` section 3.3 in alphabetical order: `child-on-kick-scooter`, `herring-gull`, `horse`, `office-swivel-chair`, `parent-carrying-toddler`, `person-with-umbrella`, `person-with-white-cane`, `pigeon`. Date: 2026-11-23.

Result: all eight pass the pose-and-clip sweep and the default-pose consistency of `🧊️three-views` (run per asset with the guard's own exported `sweep` and `consistency`, not through vitest), validate with schema 0, `validateAsset` 0, `checkVolumes` 0 (print checks included), print `✓` in the check CLI, and keep in `🧊️catalog-silhouette` exactly the set of red measures they had before (measured with the guard's `measureAsset` and gate, without the sharp oracle). No dimension changed: `widthM`, `heightM`, `depthM` of the eight assets are untouched, the manifest needs no re-sync. The vitest suite itself was not run (coordinator's run).

## 1. How each asset was diagnosed

Which view is wrong was not guessed: `🧰️three-views/tv1-truth.ts` and `tv1-bones.ts` place the 3D body of the scale (`volumes`, `worldSolids`) with the same evaluated bones and print its extents beside the extents of the three views, for the whole asset and per bone. In seven of the eight assets the side elevation equals the body to the millimetre and the front or plan drawing is off; in two (gull primaries, chair backrest) the side elevation is the wrong one.

Three causes cover everything:

1. A flat front or plan outline on a bone that turns. `evaluate` carries such an outline as a plate at one depth, so it is right only at the angle it was drawn for (horse crest, pigeon and gull toes, cane walker's boot, scooter wheels, and in the extreme the umbrella arm and the whole toddler, whose outlines had been stretched to look right at the default pose only). Remedy, as the README prescribes: a `solid` (extruded or revolved), or, for the horse, a drawing that is not declared the plate of a side shape.
2. A side-elevation prism that claims more than the body has (chair backrest: one prism over the full width of a shell that is curved in plan).
3. A wrong number (gull: lateral spans of feather prisms given from the bone instead of in asset coordinates).

No generator was changed. The scripts of the 3D ticket that name these assets (`r3-…`, `r5-…`, `rework-…`, `author-…`) are chains of single patches, each applied to the catalog file after the previous one; the last ones of 2026-11-22 patch the catalog file directly, so none of them is a source of truth that could be re-run, and the catalog file is. Each fix is one hand-written script under `🧰️three-views/` that splices its values into the catalog file and touches no other byte (`tv1_edit.py`); a script refuses to write when the file changed on disk after it was read.

## 2. Per asset

### `animals/horse` (3 findings)

- Before: pose `standing-rest`, 1:50, 1:200, 1:500: side elevation top 1.758 / 1.740 / 1.736 m, front 1.854 / 1.866 / 1.837 m (tolerance 0.087 m). Body equals the side elevation.
- Cause: the neck is lowered 32 degrees. The front crest and neck drawings shared their id with a side shape of the same bone and were therefore carried as plates at the middle depth of that shape; on a bone inclined 52 degrees the top of the drawing lies ahead of the middle and sinks more than a plate does. At 1:200 the one front `neck-2` drawing also covered the upper neck, which hangs on `neck-3` in the side elevation.
- Fix (`tv1-horse.py`, 6 edits): 1:50 front `mane-1/2/3` renamed `crest-base`, `crest-mid`, `crest-poll` (they sit on `neck-1/2/3`; the side elevation numbers its manes the other way round, so two of the three names never matched a side shape); 1:200 front `neck-2` cut by hand at the `neck-3` joint into `neck-2` and a new `neck-3` on bone `neck-3`, overlapping 60 mm; 1:500 front `neck-a`, `neck-b` renamed `neck-girth-a`, `neck-girth-b`. Without a twin the depth of each vertex follows the side silhouette of the bone at its height.
- Sheets opened: 1:50, 1:200, 1:500 `standing-rest`, before and after. Before, the front neck stood above the ears; after, the lowered head hangs in front of the chest and both views read 1.76 / 1.78 m at 1:50 (the 2 cm are the ears, inside the tolerance).
- Open: the plan of 1:200 still draws the upper neck on `neck-2` (passes; not touched). The front IoU of the silhouette guard moved by 0.0003 to 0.002 (0.9786 / 0.9624 / 0.9383, gate 0.92).

### `animals/pigeon` (1 finding)

- Before: 1:100 clip `take-off` at 0.43 s: side elevation bottom 0.036 m, front 0.057 m (tolerance 0.020 m); 1:50 used 92 % of the tolerance at the same instant.
- Cause: the toes hang down in flight. The front toes were end-on outlines 4 mm tall and stayed 4 mm tall.
- Fix (`tv1-pigeon.py`, 12 edits): the front toes of 1:50 (`toes`, `toe-inner`, `toe-outer`, `hallux`, both feet) and of 1:100 (`toes`, `toe-spread`) are extruded solids: a hand-written five-point side profile in the toe bone's frame over the lateral span the front drawing had; the splayed toes of 1:50 run diagonally through `zTip`.
- Sheets opened: 1:100 and 1:50 `take-off` at 0.43 s: the feet show under the body in the front elevation, both views 0.24 m.
- Open: 1:200 uses 67 % of the tolerance at the same instant (one 2 mm tick per foot in front; left as it is). The splayed toes of 1:100 are one prism over both toes, a rectangle when they hang.

### `humans/person-with-white-cane` (1 finding)

- Before: 1:500 clip `constant-contact-walk` at 1.10 s: side elevation bottom −0.026 m, front +0.026 m (tolerance 0.051 m).
- Cause: at 1:500 the boot carries the toe (decision "the toe into the boot"), so at toe-off the side elevation and the 1:500 body tip the whole boot about the ankle and its toe dips 26 mm under the ground; the front boot was a plate and only rose.
- Fix (`tv1-person-with-white-cane.py`, 2 edits): the front boots of 1:500 are extruded solids of the boot's side profile over the width the front drawing had.
- Sheet opened: 1:500 at 1.099 s: both views 1.72 m.
- Open, stated plainly: the 26 mm dip under the ground at toe-off is in the 1:500 body and side elevation (0.05 mm on paper); making the toe its own bone shape at 1:500 would change the 3D model and the decision text and was not done.

### `furniture/office-swivel-chair` (8 findings)

- Before: clip `swivel-rotation` at 1.6 and 2.4 s (seat turned about 32 degrees), all four scales: side elevation rear −0.400 m, plan −0.353 m (tolerance 0.030 m). Body equals the plan.
- Cause: the side elevation drew backrest shell and pad as one prism over the full width; the backrest is a shell curved in plan (rear face 68 mm behind the bone in the middle, 19 mm at the edge, rounded corners), so the turned prism reached 47 mm behind the body.
- Fix (`tv1-office-swivel-chair.py`, 8 edits that put 5 shapes in place of 1): `backrest-shell` and `backrest-pad` (1:50, 1:100), `backrest-shell` (1:200), `backrest` (1:500) and the `backrest-rim` and `backrest-texture` lines of 1:50 each become five prisms: the middle (half width 105 mm) keeps the side profile, a `-mid-left/right` band (105 to 170 mm) and an `-edge-left/right` band (170 mm to the edge) take it with the rear edge set forward to where the body stands at the outer edge of the band (measured with `tv1-local.ts` in 16 bands) and with foot and head cut where the rounded corners leave the band. Every band lies inside the old outline, so the drawing at rest keeps it.
- After: side elevation of the backrest at 1.6 s −0.368 (1:50), −0.369, −0.362, −0.359 m against the body −0.353 … −0.355 m.
- Sheets opened: 1:50 at 1.6 s before and after, 1:50 and 1:500 `seat-low`, 1:200 at 1.6 s. The turned backrest now shows the facets of a curved shell; at rest the side elevation shows two fine contours inside the old outline (the edges of the bands).
- A first version set the front edge of the bands back as well; it opened a 15 mm slit at 1:200 that the print check refused (`nominal-gap` 0.075 mm). The catalog file was invalid for about five minutes with that version, then restored from my copy and fixed; the version in the catalog has no such gap (print check in the final validator line).
- Open: the front elevation still draws shell and pad as thin plates (25 mm beside the body at the same instant, 79 to 91 % of the tolerance); the seat cushion prism reaches 18 mm ahead of the body when turned (rounded corners in plan), inside the tolerance. The decision texts were not changed.

### `humans/person-with-umbrella` (7 findings)

- Before: pose `umbrella-furled` and clip `close-umbrella` at 4.0 s, all scales: plan rear −0.256 m, side elevation and body −0.182 m (tolerance 0.062 m).
- Cause: the front and plan drawings of the right forearm, cuff and hand were flat outlines stretched so that they look right with the forearm raised 126 degrees (default pose): the plan forearm is 61 cm long in rest coordinates. With the arm lowered it reached 74 mm behind the coat, and the front hand was 43 cm tall.
- Fix (`tv1-person-with-umbrella.py`, 24 edits): at all four scales, front and plan `forearm-r` and `shirt-cuff-r` are solids revolved about the forearm from the side profile of the same scale, at the lateral position the old drawings had; `hand-r` is a prism of its side profile over the old lateral width.
- After, 1:50 `umbrella-furled`: forearm plan [−0.024, 0.127] m equals body and side elevation.
- Sheets opened: 1:50 `umbrella-furled` before and after, 1:50 `umbrella-up-still` after. Before: a long sleeve lying behind the figure in plan and a ladder of fingers down the coat in front. After: arm and hand where the side elevation has them.
- Open: the revolved sleeves carry `lateralScale`, which `evaluate` does not read (request written); the forearm is 7 mm narrower laterally than the body. At 1:500 the clip `close-umbrella` at 3.2 s uses 99 % of the tolerance (side elevation draws the furled shaft as a 60 mm strip, the plan has no shaft): passes, but only just. The front IoU of the silhouette guard went from 0.9769 / 0.9720 / 0.9651 / 0.9601 to 0.9730 / 0.9700 / 0.9626 / 0.9580 (the hand is a rectangle now).

### `animals/herring-gull` (24 findings)

- Before: poses `landing`, `approach-a`, `flare-key` and clip `landing` at five instants, three scales: e.g. `landing` side elevation [0.288, 0.973] against front [0.305, 0.948] m, clip end side −0.082 against front −0.043 m (tolerance 0.020 m).
- Cause 1 (side elevation wrong): the prisms of the ten primaries and of every exposed rachis (1:50: 64 shapes, 1:100 and 1:200: 20 each) stated `z0`, `z1`, `zTip` from the feather bone, not in asset coordinates as the README demands, so each feather stood 40 to 48 mm beside its bone and swung about it in the twisted wing. Body, front and plan agreed with each other.
- Cause 2 (front wrong): toes and webs were end-on plates, 4 mm tall with the feet trailing.
- Fix (`tv1-herring-gull.py`, 119 edits): the rest z of the feather bone (table from `tv1-rest.ts`, near wing mirrored) added to those spans; `primary-lines` of 1:50, one drawing bound to the ninth primary but spread over the folded stack, takes the span of that feather; front `foot`, `toe-inner`, `toe-outer`, `toe-middle` (1:50), `foot`, `toe-middle` (1:100), `foot` (1:200) are prisms of the side profile over the old lateral span.
- After, 1:50 `landing`, ninth primary: side elevation x [−0.091, −0.001], y [0.827, 0.948] m, equal to body, front and plan.
- Sheets opened: 1:50 `landing` before and after, 1:200 `approach-a` after: wings and feet at the same height in both elevations (0.66 m).
- Open: `gliding-down` uses 86 to 93 % of the tolerance (front body and tail are plates on a pitched trunk: front y [1.425, 1.581] against [1.408, 1.599]); plan `near-mirror` of 1:50 lies 15 mm off the body; the body itself is 43 mm under the ground at the last instant of clip `landing` (3D model, not a view). The splayed toes of 1:50 carry a five-point profile because the twelve-point one does not close in `unionPolygons` at `approach-a` (request written).

### `humans/parent-carrying-toddler` (47 findings)

- Before: pose `hoisting`, clips `shift-toddler` and `walk-carrying`, poses `walk-1`, `walk-5`: e.g. `hoisting` 1:50 front top 2.234 m against 1.775 m, plan rear −0.597 against −0.184 m (tolerance 0.052 m). Body equals the side elevation.
- Cause: the toddler hangs on the parent's right forearm, which rests hanging and is raised 100 degrees in the default pose. The front and plan drawings of that forearm and of all toddler parts were flat outlines stretched to look right at exactly that angle (front `toddler-head` 3.9 m tall in rest coordinates); 12 degrees more and the toddler grew out of the picture. The parent's legs had ordinary flat outlines: the plan shank kept its rest length with the knee bent 54 degrees (147 mm short).
- Fix (`tv1-parent-carrying-toddler.py`, 303 edits, front and plan of all four scales): sleeves, limbs, torso, neck, head, hat and pompom of the toddler and the parent's right forearm and cuff are solids revolved about their bone (`mirror`, as `sleeve-fore-l` of this asset already was) from the side profile, narrowed to the lateral half width of the old drawing; face details, mittens and toddler boots are prisms of the side profile over the old lateral span; the parent's thighs, shanks, boot shafts, boots, toes and soles are prisms of their own hand-drawn front and plan outlines (true at rest, where legs hang and feet point ahead) along the direction the view looks, as long as the side profile says.
- Two consequences handled by hand at 1:500: the toddler boot prisms span the 74 mm of their solid (the feature rule compares the solid with the thinnest drawing of the bone), and the parent's right upper sleeve is drawn 142 mm wide in front instead of 127 mm, because the toddler now hides the arm as the body does and the strip left visible printed at 0.186 mm; `decisions.thinElements` of 1:500 says so in both languages.
- After: the worst instant uses 7 % of the tolerance at 1:50 and 1:100, 17 % at 1:200, 43 % at 1:500.
- Sheets opened: 1:50 `standing-carry` and `hoisting` before and after, 1:200 `walk-1` after. Before, `hoisting`: a 1.2 m spindle above the parent's head in front and a 1.3 m smear in plan. After: the toddler sits on the arm in all three views, 1.78 m in both elevations.
- Open: revolved from a narrowed profile, the toddler parts are short in plan depth by the share they are narrower than deep (head 133 mm instead of 170 mm; request for `lateralScale` written). Toddler face details and boots are rectangles in front and plan. Silhouette guard, front IoU 0.9953 / 0.9927 / 0.9888 / 0.9731 before, 0.9934 / 0.9905 / 0.9862 / 0.9714 after; plan 0.9883 / 0.9864 / 0.9824 / 0.9765 before, 0.9805 / 0.9800 / 0.9769 / 0.9745 after; no measure changed side.

### `humans/child-on-kick-scooter` (78 findings)

- Before: twelve kick poses, e.g. `kick-15` 1:50: front bottom 0.052 m against 0.000 m, plan front end 0.440 against 0.492 m (tolerance 0.040 m).
- Cause: the wheels roll in the kick poses. Front and plan tyres and hubs were outlines of the wheel seen edge-on and turned like plates.
- Fix (`tv1-child-on-kick-scooter.py`, 16 edits): front and plan `tyre-rear/front` (1:50, 1:100, 1:200), front `hub-rear/front` (1:50) and front `wheel-rear/front` (1:500) are the circle of the side elevation extruded over the width the old drawing had (30 mm, 26 mm, 72 mm).
- After: worst instant 40 % (1:50, 1:100), 58 % (1:200), 67 % (1:500) of the tolerance.
- Sheets opened: 1:50 `kick-15` before and after: the wheels stand on the ground line in front, plan and side elevation end together.
- Open: the remaining share at 1:200 and 1:500 is the kicking foot in the front elevation (30 mm at `foot-l`).

## 3. Final lines (one run over the eight catalog files at the end, `🧰️three-views/tv1-proof.sh`)

Per asset: sweep and consistency, validator (`r3-validate.ts --file <catalog file> --volumes`, print checks included), check CLI, silhouette measure (count of red measures).

`child-on-kick-scooter`

```text
✓ child-on-kick-scooter: sweep 0 findings, default-pose consistency 0 findings, scales 50, 100, 200, 500
VALID file/child-on-kick-scooter (file as it is): schema 0, validateAsset 0, checkVolumes 0
✓ child-on-kick-scooter (1/1)
silhouette child-on-kick-scooter: 4 red
```

`herring-gull`

```text
✓ herring-gull: sweep 0 findings, default-pose consistency 0 findings, scales 50, 100, 200
VALID file/herring-gull (file as it is): schema 0, validateAsset 0, checkVolumes 0
✓ herring-gull (1/1)
silhouette herring-gull: 1 red
```

`horse`

```text
✓ horse: sweep 0 findings, default-pose consistency 0 findings, scales 50, 100, 200, 500
VALID file/horse (file as it is): schema 0, validateAsset 0, checkVolumes 0
✓ horse (1/1)
silhouette horse: 6 red
```

`office-swivel-chair`

```text
✓ office-swivel-chair: sweep 0 findings, default-pose consistency 0 findings, scales 50, 100, 200, 500
VALID file/office-swivel-chair (file as it is): schema 0, validateAsset 0, checkVolumes 0
✓ office-swivel-chair (1/1)
silhouette office-swivel-chair: 2 red
```

`parent-carrying-toddler`

```text
✓ parent-carrying-toddler: sweep 0 findings, default-pose consistency 0 findings, scales 50, 100, 200, 500
VALID file/parent-carrying-toddler (file as it is): schema 0, validateAsset 0, checkVolumes 0
✓ parent-carrying-toddler (1/1)
silhouette parent-carrying-toddler: 2 red
```

`person-with-umbrella`

```text
✓ person-with-umbrella: sweep 0 findings, default-pose consistency 0 findings, scales 50, 100, 200, 500
VALID file/person-with-umbrella (file as it is): schema 0, validateAsset 0, checkVolumes 0
✓ person-with-umbrella (1/1)
silhouette person-with-umbrella: 4 red
```

`person-with-white-cane`

```text
✓ person-with-white-cane: sweep 0 findings, default-pose consistency 0 findings, scales 50, 100, 200, 500
VALID file/person-with-white-cane (file as it is): schema 0, validateAsset 0, checkVolumes 0
✓ person-with-white-cane (1/1)
silhouette person-with-white-cane: 6 red
```

`pigeon`

```text
✓ pigeon: sweep 0 findings, default-pose consistency 0 findings, scales 50, 100, 200
VALID file/pigeon (file as it is): schema 0, validateAsset 0, checkVolumes 0
✓ pigeon (1/1)
silhouette pigeon: 2 red
```

Silhouette measure before my change, for comparison (same tool on my copies of the files): horse 6 red, pigeon 2, person-with-white-cane 6, office-swivel-chair 2, person-with-umbrella 4, herring-gull 1, parent-carrying-toddler 2, child-on-kick-scooter 4; in every asset the same measures are red after as before (they are box and Hausdorff limits of the catalog case, see the open request on those limits).

## 4. Files

Updated (asset files, hand-spliced, no other byte touched):

- `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/child-on-kick-scooter/🔣️.json`
- `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🐕️animals/herring-gull/🔣️.json`
- `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🐕️animals/horse/🔣️.json`
- `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🛋️furniture/office-swivel-chair/🔣️.json`
- `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/parent-carrying-toddler/🔣️.json`
- `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/person-with-umbrella/🔣️.json`
- `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/person-with-white-cane/🔣️.json`
- `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🐕️animals/pigeon/🔣️.json`
- `U/requests.md` (three sections appended, heading prefix "three-views part 1": `lateralScale` of 2D revolved solids is ignored; a front shape with a same-named side shape is a plate at one depth; `unionPolygons` on a tapered toe prism)

Created under `U/🧰️three-views/` (all prefixed `tv1`, the other files there belong to part 2):

- tools: `tv1-sweep.ts` (sweep and consistency of given ids or a candidate file, shapes at the edges, share of the tolerance used), `tv1-truth.ts` (body against the three views), `tv1-bones.ts` (bones ranked by distance from the body), `tv1-local.ts` (body of a bone in its own frame, in lateral bands), `tv1-rest.ts` (rest frames), `tv1-silhouette.ts` (catalog-silhouette measure of one asset), `tv1-draw.sh` (2D sheet of one instant), `tv1-proof.sh` (the four final lines), `tv1_edit.py` (splices values into an asset file)
- fixes, one per asset, records of the edits (a second run on the fixed file stops at its own assertions or finds nothing to convert): `tv1-child-on-kick-scooter.py`, `tv1-herring-gull.py`, `tv1-horse.py`, `tv1-office-swivel-chair.py`, `tv1-parent-carrying-toddler.py`, `tv1-person-with-umbrella.py`, `tv1-person-with-white-cane.py`, `tv1-pigeon.py`
- `U/three-views-1.md` (this report)

Removed: `U/🗑️generated/tv1/` (sheets, candidate files, copies of the originals, logs, three debug scripts).

Not touched: shared code, tests, fixtures, the manifest, generators, git.

## 5. What stays open, in one place

- The three requests above (shared code).
- Instants that pass with little room: `person-with-umbrella` 1:500 `close-umbrella` at 3.2 s (99 %), `herring-gull` `gliding-down` (86 to 93 %), `office-swivel-chair` front at 1.6 s (79 to 91 %).
- `office-swivel-chair`: decision texts still describe the backrest as before; front backrest still a plate.
- `person-with-white-cane` 1:500 and `herring-gull` clip `landing` end: the body itself goes 26 and 43 mm under the ground.
- `parent-carrying-toddler`: toddler parts short in plan depth; boxy small parts.
- The catalog-wide vitest suites (`🧊️three-views`, `🧊️catalog-silhouette` with its sharp oracle, the others) were not run by me.
