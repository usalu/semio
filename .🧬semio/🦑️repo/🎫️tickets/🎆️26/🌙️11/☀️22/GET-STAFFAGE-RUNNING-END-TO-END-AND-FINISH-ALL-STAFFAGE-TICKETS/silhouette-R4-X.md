# 🏗️ Silhouette Fixer R4-X: Tower Crane and Vintage Car

Report of 2026-11-24 for `📋️silhouette-brief.md`, batch `🧰️silhouette/R4-X.tsv` (`machinery/tower-crane`, `vehicles/vintage-car-1929`). Paths as in the brief (`R`, `S`, `U`). The ops of `🧰️silhouette/SAM`, `R2-M`, `R3-M` (crane) and `SAV`, `R3-V` (car) stay in force; the ops of this batch are applied after them. Every number below was measured in this session with the pinned bun, one tool process at a time.

## Result in short

- **Both green.** Each passes the real `🧊️catalog-silhouette` test, run to the end in this session; 0 of 2 before.
- Failing checks (bounding-box axis, IoU, largest boundary distance per authored scale and view), measured anew: `tower-crane` 14 before, 0 after; `vintage-car-1929` 5 before, 0 after.
- `validateAsset` 0 issues and product check CLI `✓` for both; `checkVolumes` has no finding at any scale of either asset (the crane had two `volumes-silhouette` findings at 1:500).
- No declared dimension and no power changed: `🧰️silhouette/R4-X/manifest-changes.json` is `{}`.
- No test, rule, threshold, engine, schema, validator, fixture, README or manifest was edited. One section went to `U/requests.md` (`## silhouette R4-X — …`).
- The crane's lattice at 1:500 was not a conflict between the print rule and the guard, as two earlier rounds and the open request state: seven of its 362 solids made the whole IoU failure (below).

Notation: IoU / largest boundary distance; limits IoU 0.92 and 15, 30, 60, 150 mm at 1:50, 1:100, 1:200, 1:500.

## `tower-crane`

### Failing checks before (14, all measured anew on the file as R3-M left it)

| Scale | Elevation | Front | Plan |
|---|---|---|---|
| 1:50 | 0.976 / 60.0 mm! | 0.990 / 108.2 mm! | 0.998 / 61.9 mm! |
| 1:100 | 0.976 / 60.0 mm! | 0.904! / 141.8 mm! | 0.999 / 60.0 mm! |
| 1:200 | 0.979 / 60.0 mm | 0.905! / 158.2 mm! | 0.998 / 60.0 mm |
| 1:500 | 0.835! / 112.5 mm | 0.906! / 419.3 mm! | 0.833! / 187.5 mm! |

`checkVolumes` before: two `volumes-silhouette` findings at 1:500 (elevation IoU 0.835, plan IoU 0.833).

### After (0 failing)

| Scale | Elevation | Front | Plan |
|---|---|---|---|
| 1:50 | 0.999 / 12.0 mm | 1.000 / 12.0 mm | 0.998 / 13.1 mm |
| 1:100 | 0.999 / 13.5 mm | 1.000 / 15.9 mm | 0.998 / 22.5 mm |
| 1:200 | 0.999 / 21.2 mm | 1.000 / 53.0 mm | 0.998 / 30.0 mm |
| 1:500 | 0.999 / 93.8 mm | 0.999 / 37.5 mm | 0.997 / 75.0 mm |

Bounding box 71.8222 / 52.2500 / 7.0000 m (71.8237 m long at 1:500) against the declared 71.822 / 52.25 / 7 m. `checkVolumes` of the four scales: no finding.

### Proof (`prove.sh tower-crane`, each run to the end)

- `🧊️catalog-silhouette exit 0  Test Files 1 passed (1); Tests 1 passed | 384 skipped (385);` (395 s; before: `tower-crane 1:500 elevation: expected 0.8351696506823588 to be greater than or equal to 0.92`)
- `validateAsset 0 issues | check ✓ tower-crane (1/1)`
- `🧊️three-views exit 0  Test Files 1 passed (1); Tests 2 passed | 773 skipped (775);`
- `🌡️catalog-physics exit 0  Test Files 1 passed (1); Tests 2 passed | 838 skipped (840);`
- `🫁️breathing-catalog exit 0  Test Files 1 skipped (1); Tests 290 skipped (290);`
- `✅️asset-validation exit 0  Test Files 1 passed (1); Tests 9 passed | 74 skipped (83);`
- `⚡️asset-power exit 0  Test Files 1 passed (1); Tests 7 passed | 17 skipped (24);`

`🫁️breathing-catalog` has no test named after the crane (all skipped), as before; `🧊️three-views` and `🌡️catalog-physics` 2 passed each, as R3-M measured them before. The catalog file is byte-identical to the ops of this batch applied to the file as it stood before (`cmp`), and a second `apply.ts tower-crane` leaves it unchanged.

### What was found

Measured with two new tools: `misfit.ts` (per body solid the cells of what the guard compares it as that the drawing leaves empty, with the width its strokes get; per drawn shape the cells beside the body) and `where.ts` without the whole-view raster.

1. **The lattice IoU of 1:500 was seven solids.** The four mast chords (`slender` boxes of 160 mm, 39 m long) were compared as strokes of 223 mm; the jib chord (140 mm tube, 54 m) and the two jib rails at about twice their section (16 765 and 16 362 of 37 843 cells each not covered by the drawing). Every other lattice bar read within 3 mm of its section. Cause as in the R2-O, R3-M and SH-1 entries of `requests.md`: the guard takes the centre line of a `slender` solid from its mesh; over 39 to 54 m it stands askew and the width estimate grows. With `slender` taken off the bars of 75 mm and more alone, 1:500 read 0.969 / 0.996 / 0.974. The same four chords were the front IoU of 1:200 and 1:100 (0.905 and 0.904).
2. **Solids below the smallest feature.** The 14 mm hoist rope on the 18.75 mm raster of 1:500 is in the mask or not by the luck of a cell centre, in body and drawing separately: front 1:500, 415 mm at the rope inside the tower head.
3. **Bars of the body that a view did not draw**: the second diagonal direction of the tower head in the front view (`tower-head-diagonal-7`, `-20`), the jib tip axle, the first jib diagonals, the tail frame of the counter-jib and its posts in the plan.
4. **Parts drawn elsewhere or as something else**: the vent stack of the winch house stood 3 m behind the house in every plan (drawn at x = -8 m, the body has it at -5 m); the jib rails were drawn 120 mm high and 140 mm wide, the body has them 140 x 120 mm; the winch house was drawn as one block down to the underside of its skids (the front view filled the 1.14 m between the two skids); the motor grille was drawn as a block inside the house where the body has a plate on the roof; the hook block as a solid block where the body has two cheek plates, sheave and pin; the heel sheave hanger likewise; the rope falls 4 mm beside their solids; the bail of the skip 30 to 40 mm beside its solid; the slew motors without the step of their caps; the cross-base and ballast with square corners (the body has 10 and 20 mm radii).

### What was changed (`🧰️silhouette/R4-X/ops/tower-crane.json`, 1 310 ops, written by `make/tower-crane.sh`)

Authored against the file as it stood before this batch; three steps:

- **`make/tower-crane.py`, parametric (1 248 ops).**
  - Volume, all four scales: a scale models no bar below its smallest feature, and a bar of that size or more is a body, not a `slender` line. All 1 022 `slender` solids lose the flag and are compared as their own mesh. Bars below the smallest feature are typified to it: 1:500 the hoist rope and its eight pieces, the three 50 mm pendants and the 60 and 70 mm lacing to 75 mm; 1:200 the rope falls and rope run to 30 mm; 1:100 to 15 mm; 1:50 nothing (the 14 mm rope is above 7.5 mm). The capsules of R3-M (pendants, rope runs at 1:200 and 1:500) are tubes of revolution again: the capsule was the work-around for the centre line, its round tips are not the bar. No solid of the crane is `slender` any more.
  - Drawing: the `member` shapes and rope strips get the sections of their solids; the jib rails 140 x 120 mm; the rope falls where their solids hang (the front view draws one of the two: two rings of one swept shape that cover each other cancel).
  - Parts that are no bars, each as a prism, a disc or a body of revolution of its bone, so that it is right in the three views and in the slewed pose `parked-weathervane` (turntable twisted 60 degrees): winch house on its two skids, roof grille as the plate of the body, side glass of the cab in front and plan at 1:200, hook block as two cheek plates with sheave and pin (1:100, 1:50), skip lugs and trolley axles (1:100, 1:50), the two plates of the heel sheave hanger, the skip rim, the round bow of the hook and the stepped slew motors (1:50), the sheave block of the trolley (1:500).
  - Decision texts (en, de) of the volume and of `thinElements` at every scale state the rule and what is typified.
- **`cover.ts`, per scale and view (bars the drawing leaves out).** Every straight bar solid whose projection the drawing leaves uncovered, or whose `member` of the same name lies beside its axis by more than 2 mm or has another section, gets a `member` with the end points and section of the solid, greedy by the largest gap so that bars that project onto each other are drawn once: 29 members added over the twelve views (jib tip axle, tower head diagonals 7 and 20, first jib diagonals, jib tip posts, tail frame and tail posts of the counter-jib), 16 moved onto their solid (the bail legs of the skip, four frame bars of the tower head).
- **`outline.ts`, flat outlines only where a flat shape is right**: in the plan (the slewing unit turns about the vertical) the vent stack at every scale, skip lugs and trolley wheels at 1:100 and 1:50, the skip rim at 1:50; on the fixed base the cross-base and the ballast in elevation and front at 1:50.

Tried and taken back: flat outlines of winch house, grille, block and lugs in the elevations (wrong in the slewed pose; replaced by prisms and members before anything reached the catalog); the sheave of the hook block drawn from cheek to cheek to satisfy `suitability-gap` at 1:50 (the pocket under the sheave became a closed hole in the drawing only: front 50.6 and 52.5 mm; instead the four block shapes have 0.13 mm outlines at 1:50, so 0.27 mm of the 0.4 mm play stays white); a first version without region handling (8 `region-ambiguous` issues on the skid shapes, fixed in the script).

### Sheets looked at

Product sheets (side, front, plan; line and solid view) of 1:500 and 1:50 before and after, 1:200 and 1:100 after; the head of the 1:500 side elevation cropped before and after; the tower head of the 1:50 front view magnified from the product SVG (`zoom.sh`) before and after. Nothing uglier or wrong. Visible differences: the vent stack sits on the winch house in the plan; the tower head shows both diagonal directions in the front view; the motor grille is no longer a dark rectangle on the house wall in the side elevation; at 1:500 pendants and hoist rope are a little heavier than before (a 75 mm bar with its 0.13 mm outline where a 50 mm bar and a 14 mm rope stood), about as heavy as the lacing diagonals; the sheets of 1:500 and 1:50 were opened again from the final catalog file.

## `vintage-car-1929`

### Failing checks before (5, measured anew)

1:50 elevation 0.994 / 64.5 mm!, front 0.997 / 54.5 mm!, plan 0.995 / 21.4 mm!; 1:100 elevation 0.993 / 32.0 mm!, plan 0.998 / 72.1 mm! (1:100 front 0.996 / 22.8 mm, 1:200 and 1:500 pass).

### After (0 failing)

| Scale | Elevation | Front | Plan |
|---|---|---|---|
| 1:50 | 1.000 / 8.4 mm | 0.999 / 13.7 mm | 1.000 / 10.9 mm |
| 1:100 | 0.999 / 8.4 mm | 0.998 / 15.0 mm | 1.000 / 7.5 mm |
| 1:200 | 0.985 / 23.7 mm | 0.982 / 47.9 mm | 0.996 / 16.8 mm |
| 1:500 | 0.997 / 17.4 mm | 0.970 / 59.8 mm | 0.989 / 52.2 mm |

`checkVolumes` of the four scales: no finding.

### Proof (`prove.sh vintage-car-1929`, each run to the end)

- `🧊️catalog-silhouette exit 0  Test Files 1 passed (1); Tests 1 passed | 384 skipped (385);`
- `validateAsset 0 issues | check ✓ vintage-car-1929 (1/1)`
- `🧊️three-views exit 0  Test Files 1 passed (1); Tests 2 passed | 773 skipped (775);`
- `🌡️catalog-physics exit 0  Test Files 1 passed (1); Tests 2 passed | 838 skipped (840);`
- `🫁️breathing-catalog exit 0  Test Files 1 skipped (1); Tests 290 skipped (290);` (no vehicle case, as before)
- `✅️asset-validation exit 0  Test Files 1 passed (1); Tests 9 passed | 74 skipped (83);`
- `⚡️asset-power exit 0  Test Files 1 passed (1); Tests 7 passed | 17 skipped (24);`

All as R3-V measured them before, the silhouette test green instead of red. The catalog file is byte-identical to the ops applied to the file as it stood before (`cmp`); a second `apply.ts` and a second run of the make script leave file and ops unchanged.

### What was changed (`🧰️silhouette/R4-X/ops/vintage-car-1929.json`, 133 ops, written by `make/vintage-car-1929.sh`; drawing only, 1:50 and 1:100)

`misfit.ts` listed per view what the drawing left out or drew differently; each such part is drawn as the outline of its solids (`outline.ts`; every part is on the fixed chassis, on an axle that only travels up and down, or on a wheel that turns in the plane of the side elevation; no pose turns the car out of its planes).

- 1:50 side elevation (64.5 to 8.4 mm): the feet of the lamp bar were not drawn (the pocket between radiator shell, lamp bowl and wing was open in the drawing and closed in the body: the 64.5 mm); bonnet hinge, wiper with motor, drop arm, spring shackles added; drag link and wing stays as strokes at the width of their solids; the twelve spokes of each wheel as the outline of the spoke solid (the capsules stood one to two raster cells beside it); tyres, rims, wings, wheelhouse, tail pipe, radiator cap with neck, lamp lens, tail lamp and rear bumper blades retraced.
- 1:50 front (54.5 to 13.7 mm): rear springs with hangers and shackles, door hinges, belt moulding, dampers, tie rod, exhaust down pipe and lamp bar feet added; front axle with king pins and steering arms; roof slab, tyres with their round shoulders, drums and hubs retraced (the hubs behind the tyres).
- 1:50 plan (21.4 to 10.9 mm): the four wings, both bumpers with their clamps, lamp bowls and lenses, radiator shell, frame rails and tail lamp retraced; wing stays, dampers, bonnet catches and steering arms added; drag link and tie rod as strokes of their solids.
- 1:100 side elevation (32.0 to 8.4 mm): the eight spokes of each wheel as the outline of the spoke solid; tyres, rims, lamp lens, radiator cap retraced; spring shackles added.
- 1:100 front (22.8 to 15.0 mm) and plan (72.1 to 7.5 mm): roof slab and frame; the four wings, front springs, bumpers and frame rails retraced (the 72.1 mm stood beside the bonnet over the front springs, which were drawn narrower than their solids).
- Decision texts of 1:50 (`retained`, `omitted`; en, de) name the parts that are drawn now.

Tried and taken back: the rear springs in the 1:100 front view (what stays visible between running-board bracket and bumper is 0.18 mm, `suitability-feature`); the hubs in front of the tyres in the 1:50 front view (they showed as light discs on the tread; they lie behind the tyre now). `outline.ts` got a fallback for unions on which `polygon-clipping` throws (coarser grid down to 0.1 mm; the exhaust down pipe needed it).

### Sheets looked at

1:50 before and after (whole, side elevation cropped), 1:100 after, 1:500 after (unchanged scale). The same car to the eye; wiper, bonnet hinge and the rear springs under the frame are there now.

## Files

Paths relative to the repository root; `U` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS`, `C` = `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog`.

Updated:

- `C/🏗️machinery/tower-crane/🔣️.json`
- `C/🚜️vehicles/vintage-car-1929/🔣️.json`
- `U/requests.md` (one section appended: `## silhouette R4-X — …`)

Created:

- `U/silhouette-R4-X.md` (this report)
- `U/🧰️silhouette/R4-X/manifest-changes.json` (`{}`)
- `U/🧰️silhouette/R4-X/ops/`: `tower-crane.json`, `vintage-car-1929.json`
- `U/🧰️silhouette/R4-X/make/`: `tower-crane.sh`, `tower-crane.py`, `vintage-car-1929.sh`
- `U/🧰️silhouette/R4-X/`, new: `misfit.ts`, `mf.sh`, `cover.ts`, `audit.py`, `spots.sh`, `w.sh`, `wh.sh`, `box.sh`, `zoom.ts`, `zoom.sh`; from R3-M with changes: `where.ts` (no whole-view raster, several pictures in one run), `probe.ts`, `dump.py`, `tile.py`; from R3-V with the paths of this batch: `env.sh`, `people.ts`, `ops.ts`, `apply.ts`, `measure.ts`, `one.ts`, `near.ts`, `strokes.ts`, `validate.ts`, `outline.ts` (union fallback, `after` honoured), `texts.py`, `svg.ts`, `try.sh`, `vt.sh`, `others.sh`, `prove.sh`, `look.sh`, `crop.sh`

Removed: `U/🗑️generated/silhouette/R4-X` (copies of the state before, logs, rasters, sheets) at the end.

## What stays red

Nothing in this batch. Stated plainly, where the result is close or where this round did less than it could:

- Results near the limit: crane 1:200 front 53.0 mm of 60, 1:50 plan 13.1 mm of 15, 1:50 elevation and front 12.0 mm of 15; car 1:100 front 15.0 mm of 30, 1:50 front 13.7 mm of 15.
- The crane's make script is authored against the state before this batch, which lies in `🗑️generated` and is deleted with it: `ops/tower-crane.json` with `apply.ts tower-crane` is the reproducible repair (same result on the file before or after, checked). The car's make script reproduces its ops from the repaired file as well (checked: identical ops).
- The crane's typified ropes and pendants change the thermal areas that reference them (`of` solids); `🌡️catalog-physics` passes as before and `checkVolumes` reports no `volumes-thermal-coverage` finding; nothing else was measured about it.
- At 1:500 the 24 bars of 70 mm and the four of 60 mm are now 75 mm in body and drawing; a reader of the 1:500 file finds that lacing 5 to 15 mm thicker than at 1:200. `volumes-cross-scale` has no finding.
- The crane's make script regenerates everything in about three minutes, its proof takes about seventeen (the real test alone 395 s); the outline of a flat part is only used in the plan and on the fixed base.
