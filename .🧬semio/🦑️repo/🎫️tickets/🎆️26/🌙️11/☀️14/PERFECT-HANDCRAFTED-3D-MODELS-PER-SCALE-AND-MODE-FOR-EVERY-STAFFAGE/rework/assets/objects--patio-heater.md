# objects/patio-heater — rework report

## Pass 1 (second round), 2026-11-22

State before: source `T/volumes/objects/patio-heater.json` and catalog `asset.volumes` were identical (no half-finished edit). Earlier work: `T/r3-objects-patio-heater.md` (complete round-3 model with fetched references), `T/rework/15.md` section 5 ("judged, not rebuilt"; a later pass copied the 1:50 wheel parts unchanged into 1:100, 1:200 and 1:500 and added knob grip, boss and stem to the coarse scales).

Edit script (states every hand-chosen number, idempotent by id): `T/rework-patio-heater-edit.py` (`--apply` writes source and catalog roots; then `volumes-insert.py`).

### Reference (commercial mushroom patio heater, 12-14 kW, EN 14543 class; my own knowledge plus the table of `r3-objects-patio-heater.md`)

| quantity | real | model before | verdict |
|---|---|---|---|
| height / reflector | 2.2-2.3 m / o 0.80-0.85 m, shallow spun dome, rise 0.12-0.17 m, rolled rim, on 3 studs | 2.25 m / o 0.80, rise 0.15, rim bead, rib, 3 studs with dome nuts, finial | right |
| emitter | perforated screen o 0.20-0.22 x 0.21-0.30 m, cap, guard rods seated in deck flange and cap rim | o 0.22 x 0.21, 8 rods o 8 | rods ended 4 mm short of flange and rim (floating at 1:100, open load path at 1:50) |
| valve housing | cone to burner deck, knob o 45-50 on the valve, piezo button | as real, knob on own bone 0/60/120 degrees | right |
| post | o 57-60 x 1.5 stainless, 2-3 sections | o 60, three sections, couplings | right |
| bottle housing | sheet cylinder o 0.38-0.42, top at 0.73-0.80 m, access door about 0.30 x 0.56 m on hinges with latch and vent louvres (or lift-up shroud) | o 0.42 x 0.73, door only a relief 3 mm proud of a closed wall | no opening, no hinge, door cannot open |
| bottle | 11 kg propane, o 0.30 x 0.58-0.60 m with collar, valve, regulator o 60-70 on the outlet, hose o 13 up into the post | only in the thermal list, no regulator, no hose, closed collar | missing from the body |
| base | weighted pan o 0.46 x 0.04 | as real | right |
| wheels | 2 wheels o 90 x 30 on a o 12 axle in two brackets on the pan rim, 5 mm clear of the ground | geometry right at 1:50, but rigid on the base bone; coarse scales carried the 1:50 parts (4 mm bush, 8 mm bracket at 1:500) | no axle bone; coarse scales not modelled |
| mass | 17-18 kg + bottle 13.5 + 11 kg gas | 45 kg | right |

### Defects found (sheets 1:50 body, thermal cut, structure, three-quarter; 1:500 body)

- Rig: no door hinge, no wheel axle; 6 bones.
- Body 1:50: closed housing wall behind a relief door; bottle, regulator, hose absent.
- 1:100, 1:200, 1:500: wheel kit = unchanged 1:50 solids; knob grip (8.5 mm), knob boss (15 mm), knob stem (8 mm) under the feature (`volumes-feature`, 9 findings); 1:500 knob 27 mm.
- Head joinery: guard rods floating 4.5 mm (1:100 body and structure, `volumes-connected`), 1:50 structure had no screen between deck and cap (open load path), 1:200 structure valve housing ended at r 85 under a screen frame starting at r 95 and the spacers ended 7.5 mm under the dome.

### Rebuilt

- **Rig** (8 bones): `door` (child of `base`, vertical hinge line at the rear door edge, x 0.1, y 0.14273, z -0.15322, length 0.56, revolute by twist -110..0 degrees, 1.7 kg) and `wheels` (child of `base`, lateral rest along the axle: x 0.05, y 0.26, z -0.18, `abductionDeg` 90, length 0.36, twist -180..180, 0.6 kg); `base` mass 8.7 -> 6.4 kg (sum stays 45.0).
- **Pose / clip**: new pose `bottle-change` (door open 105 degrees, burner off, bottle 60 % exposed), new clip `open-for-bottle-change` (8 s: open, hold, close). `gas-bottle.hidden` text says that only this pose shows the bottle.
- **Body 1:50** (46 -> 60 solids): wall = sill ring, C-shaped wall with the 86-degree opening, head ring; door leaf 1.2 mm flush in the opening on the `door` bone with its 12 louvres, latch and two o 12 pin hinges; bottle (foot ring, shell, collar with handle opening towards the door, valve), regulator on its union nut, o 13 hose in four runs into the post; tyres, hubs and axle on the `wheels` bone, bushes and brackets stay on the pan; guard rods now run from the deck flange into the cap rim.
- **Body 1:100**: wall with opening, flush door leaf, bottle shell and collar, each wheel one bored body, slender axle, 15 mm bracket plates, guard rods seated, knob grip removed.
- **Body 1:200**: housing cut into pan with sill, wall with opening, top; door leaf; bottle; wheel discs o 90 x 30, slender axle, 30 mm bracket blocks; knob grip and boss removed.
- **Body 1:500**: can cut into base, wall with opening, top; 75 mm door leaf; bottle; one o 90 roller for wheels and axle on one bracket block; knob as a o 75 x 75 boss sunk into the valve housing.
- **Thermal**: hidden bottle region keeps its own solids at every scale (a hidden region may not reuse body solids), at 1:50 now with regulator, nut and hose and the open collar; hinge and wheel-kit references per scale.
- **Structure**: door leaf as member of `door` with the hinges as joints, wheel set as members of `wheels` with hubs and bushes as joints; 1:50 screen and bands added between deck and cap; 1:200 valve housing with its deck flare, spacers reaching from cap plate into the dome; knob stem 15 mm at 1:100, removed at 1:200 and 1:500.
- **2D** (all four scales): elevation = dark `door-opening`, `bottle` silhouette clipped to the opening, door leaf and seam (1:50 also the 12 slots and the latch) on the `door` bone as extruded solids; the same leaf as a solid shape in front and plan. Closed pose reads as before; `bottle-change` shows the open housing with the bottle in the side elevation, the leaf standing out in the front elevation and beyond the dome in plan.
- **Decisions**: one "Rework" paragraph per scale in `volumes[scale].decisions` (en, de) and one sentence added to each `representations[scale].decisions.silhouette`.

### Validation

`r3-validate.ts objects/patio-heater --catalog-only --volumes` after the last change: `VALID objects/patio-heater (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0`. Published with `volumes-insert.py` three times (after body/rig, after 2D front/plan, after the solid fix).

### Rendered and read after the last volume change

1:50 body sheet, thermal cut, structure; 1:50 `bottle-change` six views and three-quarter, `low` front, clip `open-for-bottle-change` t 1 s, `turn-down-and-shut-off` t 900 s; 1:100 body, thermal cut, structure; 1:200 body, structure, `bottle-change` three-quarter; 1:500 body, thermal cut, `bottle-change` three-quarter; 2D 1:50 default and `bottle-change`, 1:100 default, 1:500 `bottle-change`.

Rendered but NOT read: 1:200 thermal cut, 1:500 structure, 1:100 `bottle-change`, 1:50 `off-afternoon`, `high-in-breeze`, `ignite-and-warm-up` t 2 / t 450, `turn-down-and-shut-off` t 30, `open-for-bottle-change` t 4; 2D 1:200 `bottle-change`. Not rendered: 2D 1:200 and 1:500 default.

### What remains imperfect

- No transport pose: the heater is never shown tilted onto its wheels (needs `pose.root` rotation about the axle and a wheel contact anchor); the `wheels` bone exists but no pose or clip turns it.
- 1:50: hubs are plain turned discs (no ribs); emitter perforation only as 2D hatch; burner, venturi, pilot, thermocouple and tilt switch inside the head are not modelled; door has no return flange or stop; regulator and hose are not drawn in 2D.
- 1:500 thermal: the dome plume outlet patch (75 mm thick, on the dome) reads as a second dome in the cut sheet (unchanged, from round 3).
- 2D: the 12 vent slots (0.8 mm long on paper at 1:50) could not be confirmed at full zoom; in the front elevation at rest the door edge shows as a narrow strip at the housing edge in the solid views.
- The three-quarter sheets show a black band at the far dome rim at every scale (crease lines of the rim seen from above); not investigated.

### Fundamental issues (outside this asset)

- `volumes-feature`: a partial revolve (`sweepRad` < 2 pi) of a closed thin loop with `sheet: true` is judged by the extents of the full turn ("420 mm thick, 420 mm across ... no thin sheet"); without the flag the same 1.2 mm leaf passes as a hollow revolve. The declaration is refused where it is true.
- `r3-2d.ts --svg` output carries no shape ids, so a single small stroke cannot be found in it.
