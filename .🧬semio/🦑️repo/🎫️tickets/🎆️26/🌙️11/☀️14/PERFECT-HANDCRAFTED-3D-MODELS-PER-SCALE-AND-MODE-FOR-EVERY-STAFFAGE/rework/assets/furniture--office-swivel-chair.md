# `furniture/office-swivel-chair` — running report

Earlier pass: `rework/14.md`, section 1 (reference table, structure castors closed by reference). Source and catalog were identical when this pass began.

## Pass 1 (second round), 2026-11-22

Scripts (inputs, kept): `rework-office-swivel-chair-armrest.py`, `rework-office-swivel-chair-castor-swivel.py`, `rework-office-swivel-chair-decisions.py`.

### Reference table (mm, EN 1335-1 type B chair, manufacturer practice, from memory)

| Quantity | Reference | Model (measured in `rework/14.md`, re-read on the sheets) |
|---|---|---|
| Seat height, top of cushion | 400 – 510, stroke at least 100 | 449 – 549, stroke 100 |
| Seat depth x width, cushion | 440 – 470 x 460 – 500, 40 – 60 thick, waterfall front | 460 x 480 x 49, rounded front |
| Backrest top above seat, width | 500 – 580, 420 – 460, horizontal radius at least 400 | 554, 450, about 440 |
| Backrest recline | at least 15 degrees about a hinge under the seat rear | 0 – 20 degrees |
| Armrest top above seat, pad, clear width | 200 – 250, at least 200 x 40, 460 – 510 | 211, 250 x 85, 490 |
| Armrest fixing | strap screwed to the underside of the seat pan | strap hung 3.2 mm under the pan (defect) |
| Five-star base | radius to castor stem 290 – 350, arms fall to the stem bosses | 300, about 20 fall |
| Castor | twin wheel 50, stem 11 in the boss, swivels freely about the vertical stem, axle trails 20 behind it, wheels roll on the axle | 50, trail 20; no swivel pivot in the rig, housing bound to the rolling wheel bone (defect) |
| Gas lift | standpipe 50, piston 28 | 50, 28 |
| Mass | 12 – 18 kg | 15.4 kg |

### Defects found (judged on all sheets: four scales x body, thermal, structure, thermal cut, every pose, two instants per clip, close-up from below)

1. 1:50 body and structure: armrest straps did not touch the seat pan (3.2 mm air); `volumes-connected` (new rule) reported the armrests as floating and two open load paths. 1:200 and 1:500 had the same gap of about 1 mm inside their tolerance.
2. Rig: a castor had one bone at the axle with a revolute joint; the vertical swivel stem, the pivot that defines a castor, did not exist, and neck, head, web and hood sat on the wheel bone, so they would have rolled with the wheel.
3. 2D: castor shapes were flat shapes, wrong as soon as a castor turns out of the paper plane.
4. Nothing else wrong in proportion: body, thermal nesting (foam inside fabric, shells, steel, tyres) and structure (hub, arms, tubes, mechanism, pan, posts, carrier, shell, castors, supports, masses) read as the same real chair at all four scales.

### Rebuilt

- Body 1:50, 1:200, 1:500 (`…-armrest.py`): straps lengthened under the seat and raised to bear on the pan (1:50: stations z 130 and 190 mm at x 41.5 mm; 1:200: 170 / 220 at 40; 1:500: 200 at 41.5). Thermal and structure follow by reference.
- Rig (`…-castor-swivel.py`): five new bones `swivel-front|side-a|side-b|back-a|back-b` (child of the arm at the boss underside, pointing down the stem, revolute, axial twist = swivel, 0.06 kg); `castor-*` is now the wheel bone under it (20 mm down, 20 mm behind, rolls, 0.09 kg). Total mass unchanged (15.4 kg). Anchors stay on the wheel bones.
- Volumes, all scales and lists: neck, head, web, hood (1:50), head and centre body (1:100) and the structural stem joints rebound to the stem bones with their coordinates carried over exactly; wheels, rims, tyres, axle joints and contact pads stay on the wheel bones.
- 2D, all four scales, all three views: every castor shape redrawn as a placed solid with one profile for elevation, front and plan (wheels and hub extruded along the axle, web and hood extruded with the 3D profile and span, swivel head revolved about the stem). The 1:50 hub is one solid across both wheels, because two hub solids left unprintable slivers between them in oblique positions.
- Poses: `castors-scattered` (stems twisted 120, -60, 150, 35, -140 degrees) and `rolled-sideways` (all 90 degrees).
- Decisions: one entry per scale (en, de) on the rig and the strap.

### Validation

`VALID furniture/office-swivel-chair (catalog asset + volumes source): schema 0, validateAsset 0, checkVolumes 0`; after the last publish `VALID furniture/office-swivel-chair (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0` (`--catalog-only --volumes`).

### Read after the last geometry change

1:50 body: clip `height-adjust` t 3.6 s (six views), pose `rolled-sideways` from below behind, pose `castors-scattered` (six views, candidate with identical volumes); 1:50 thermal default; 1:100 structure in `castors-scattered`; 2D 1:50 default and `castors-scattered`, 2D 1:100 and 1:500 `castors-scattered`. Rendered but not read again after the rig change: 1:100, 1:200, 1:500 body, thermal and structure in the default pose, 1:50 structure and thermal cut, poses `seat-high`, `reclined`, `swivel-quarter`, `swivel-quarter-opposite-key`, clip `swivel-rotation`, the other instant of `height-adjust`, 2D 1:200 and the default 2D of 1:100, 1:200, 1:500 (all were read before the change; the change does not move anything in them at rest).

### Remaining imperfect (open list for the next pass)

- Rig: the four side and back arm bones are still sagittal projections (length 92.7 and 242.7 mm instead of 300 mm, lateral part carried as `z` of the children). The rig supports a rest `abductionDeg`; true 300 mm arm bones turned 72 and 144 degrees would be the right skeleton. Solids, 2D shapes and members of the arms would have to be rebound.
- Rig: armrest height (stem telescoping in the post, about 100 mm) and seat tilt of a synchro mechanism are not rigged; the armrests are fixed solids of the seat bone.
- Clips: no clip uses the castor swivel or the wheel roll (a "roll aside" clip with root motion is missing).
- 1:50 body: the seat cushion has no sewn seam; the backrest has its two panel seams.
- 1:200 and 1:500 body: arms (54 to 36 mm wide), column (50 and 28 mm), carrier flat steel (10 mm), straps and the 50 mm wheels are kept at true size below the 30 / 75 mm feature; the validator accepts them, a stricter reading of "modelled for its feature" would thicken or merge them.
- Catalog file: the members outside `volumes` were rewritten with indent 2 by the root script (values of all untouched members are identical, checked by a diff of the parsed document).

### Fundamental issues (outside this asset)

- `S/README.md`, "Solid placements": `z0` / `z1` of an extruded 2D solid are lateral positions of the rest pose in asset coordinates (`🔨️modules/🧮️evaluation/🟦️.ts`, `local(p, (z0 + z1) / 2 - rest.frame.o[2])`), not offsets from the bone. The README does not say so; with bone-relative spans on a bone that has `z`, the shapes swing about the stem by the bone's lateral offset in a twisted pose.
- Validator: with those wrong (bone-relative) spans the four castors with `z` were drawn at the centre line in front and plan in the default pose, and `r3-validate.ts --volumes` still ended `checkVolumes 0`. Observed once, not isolated: the silhouette / views-disagree rules seem not to see 50 mm castors that are displaced by up to 285 mm.
- `r3-sheet.ts --file` wrote the same file name for different `--pose` values in one of my runs before the pose existed in the document list (the first three renders overwrote each other); later runs carried the pose in the name. Not isolated.
- `rig-lateral-rest.md` named in the task does not exist in the ticket folder.
