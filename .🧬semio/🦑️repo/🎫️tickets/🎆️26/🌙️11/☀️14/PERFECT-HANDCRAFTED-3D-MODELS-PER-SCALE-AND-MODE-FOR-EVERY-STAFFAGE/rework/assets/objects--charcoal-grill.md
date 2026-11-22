# Rework — objects/charcoal-grill

One running report, one section per author pass.

## Pass 1 (2026-11-22)

Script (the chisel, hand-chosen numbers only): `T/rework-charcoal-grill-lid-top.py` (`--source` writes `T/volumes/objects/charcoal-grill.json`, `--catalog` splices `dimensions`, the two bones, the changed 2D point lists and decision sentences into the catalog file in place and checks that the result equals the reworked document). Helper: `T/rework-charcoal-grill-fault.ts` (runs the product's `polyhedronFault` on the lid shells of a candidate). The earlier batch report `T/rework/15.md` never reached this asset; the state I started from was the round-3 model of `T/r3-objects-charcoal-grill.md`.

### Reference table (57 cm kettle of the Weber Original Kettle class)

| Quantity | Real | Model before | Model after pass 1 | Verdict |
|---|---|---|---|---|
| Bowl / lid diameter | 570 mm nominal, 585 over the rolled rims | 585 | 585 | right |
| Bowl depth / lid height | bowl about 250, lid 200–235 | 257 / 236 (242 to the crown surface) | same | in class; lid on the tall side |
| Lid grip | wood or nylon bar about 150–160 x 30 x 22 mm ACROSS THE CROWN of the dome, on two bent steel straps about 115–120 mm apart, 25–30 mm finger clearance | bar on two rods sticking out horizontally 40 mm IN FRONT OF THE RIM, 60 mm above it (a saucepan handle) | bar 160 x 30 x 22 on the crown, 28 mm clear, two 18 x 8 straps 116 mm apart | was wrong, now right |
| Lid vent | 80 mm aluminium dial, OFF the crown (the grip owns the crown), about 110–130 mm along the dome, lying on the dome and tilted with it; three holes through lid and dial | dial flat on the crown, holes at the crown | 24 degrees (123 mm) off the crown toward the wheels, tilted with the dome, shell holes moved with it | was wrong, now right |
| Height, lid closed | 1003 mm to the top of the grip (Original 57); 1060–1070 with the 5 cm longer legs of E-5710 / Premium | 1.00 m to the fin of the dial | 1.03 m to the top of the grip | 27 mm high for the Original, 35 mm low for the E-5710: legs are 27 mm long for the Original — OPEN |
| Front-to-back extent | no single published value without the grip; tripod foot circle about 600 | `widthM` 0.64 (measured to the false front grip) | `widthM` 0.60 (rim to front foot) | corrected |
| Width over side handles | about 670 | 0.67 | 0.67 | right |
| Legs | three 25 mm tubes, sockets on the bowl, splay about 10–11 degrees, one foot, two on the wheel axle | as real | same | right |
| Wheels | 152 x 38 mm, hub caps, 10 mm axle | 152 x 38 with hub cap, plain tread | same | form right; no tread ribs — OPEN (minor) |
| Wire triangle, ash pan | 8 mm wire triangle about 240 above ground; 300 mm pan under the bowl | as real | same | right |
| Bottom damper | 120 mm sheet disc 4.5 mm under the bowl, lever with knob | dished plate, lever, knob at every scale (1:50 copies) | 1:50 unchanged; coarser scales see below | right |
| Mass | 14.5 kg + 1.5 kg charcoal | 16.0 | 16.0; lid centre of mass moved from x 0.331 (false grip) to 0.287, y 0.115 | right |

### Defects found (sheets 1:50 body / thermal / structure, pose `lit-lid-open`, then the lists)

1. Lid grip in the wrong place and of the wrong kind (see table) at all four scales, in 3D and in all twelve 2D drawings.
2. Vent dial on the crown, where the grip belongs.
3. `dimensions.heightM` and `widthM` described the false grip and dial.
4. 1:100, 1:200 and 1:500 carried 1:50 copies of the small fittings: vent dial with 12 mm holes and its 27 kB skirt mesh, 10 mm rod brackets, 8 mm rests and hangers, 8 x 13 lever, 18 mm knob, 40 mm charcoal mound at 1:500. While I worked (01:10) the shared `volumes-feature` rule changed and reported exactly these 65 solids on the untouched catalog; they are remodelled, not exempted.

### Rebuilt

- **Rig**: `lid-vent` moved to the dome point 24 degrees off the crown (lid x 0.17273, y 0.21654) and turned to the dome normal (114 degrees), so its twist is about the dial's real axis; `lid` mass point to the true centre. Poses, clips, anchors (`vent-top`), emitter and thermal regions bind to the same ids and stay valid.
- **Lid shell and thermal lid envelope, all scales**: every dome vertex is turned about the lateral axis through the sphere centre, by the full 24 degrees within 10 degrees of the crown and fading smoothly to nothing at 72 degrees (rim untouched), so the three vent holes of the shell travel under the dial, both faces stay on their spheres and the mesh stays closed and free of self-intersection (checked with the product's `polyhedronFault`; at 1:500 the inner face is set 1.5 mm further in, because the coarse inner and outer facets crossed).
- **1:50 body**: grip loft of nine stations across the crown, two strap boxes into grip and shell.
- **1:100**: grip in five stations, 15 mm straps, 15 mm fin, dial one 15 mm plate with holes seated in the dome (skirt gone), rests and pan hangers 12 mm, damper as its flat 4.5 mm sheet disc, 15 mm lever.
- **1:200**: grip a 30 mm bar on two 30 mm posts, dial a 25 mm cap without holes or fin, side handles 30 mm bars on one 30 mm block each, hangers 30 mm, damper sheet disc with a 30 mm lever; rests inside the bowl and the knob left out.
- **1:500**: grip and straps one block 80 x 84 x 160, dial its 3 mm sheet disc of 80 mm on the dome, side handles and blocks 34 mm, hangers 34 mm, damper sheet disc, charcoal bed 76 mm deep; rests, lever, knob left out.
- **Thermal / structure lists**: references to removed solids removed at each scale; `mass-lid` moved to the new centre of mass at every scale.
- **2D**: `handle` and `handle-bracket*` redrawn at all four scales in elevation, front and plan (grip section on the crown, strap under it, lateral spans of the straps; block at 1:200 and 1:500 sized so the 1:500 band prints to 1:1000); the dial shapes follow their bone. Representation decisions corrected.
- **Decisions**: one sentence pair per scale for the lid top, one per coarse scale for the fittings; two false phrases corrected.

### Validation

See the end of this section (filled in after the full run).
Second cut of pass 1 (`T/rework-charcoal-grill-load-path.py`), after the first full measured run reported five findings:

- **Structure, all scales**: each wheel is now a member between its axle bearing and its support pad (before, the bearing joint ended in the air 26 mm above the pad). 1:50: the lid hinge joint is a 15 mm pin on the rim line that reaches both rims (the lid group was 4.8 mm from the bowl), the front support pad is 12 mm high so the leg tube stands in it (was 1.5 mm short).
- **Thermal 1:200 and 1:500**: the hidden cooking grate keeps its ring and every second bar (19.5 mm pitch is below both features; 1:200 was heavier than 1:100).
- **1:500 body**: side handles and their blocks left out, as the 1:500 drawings always decided (130 x 34 mm is 0.07 mm on paper); with them the plan silhouette disagreed with the drawing (IoU 0.864).

### Validation

`r3-validate.ts objects/charcoal-grill --catalog-only --volumes` on the published catalog, 2026-11-22 about 01:50: `VALID objects/charcoal-grill (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0`.

Rendered and read after the changes: 1:50 body (default pose, `lit-lid-open`, clip `light-and-close` at 303 s), 1:100 / 1:200 / 1:500 body, 1:500 body `lit-lid-open`, 1:50 thermal with `--cut`, 1:500 structure, the 2D drawings of 1:50 and 1:500. The 1:50 and 1:500 structure and thermal sheets were rendered before the last thermal thinning at 1:500.

### What remains imperfect (open list for pass 2)

1. **All scales, body + rig + 2D: legs 27 mm too long for the 100.3 cm Original Kettle.** The grip is now where the real height is measured, so the model stands 1.03 m. Right fix: lower bowl, lid, grate, charcoal, damper, pan and sockets by 27 mm (rim at 0.713 m, grate at 0.673 m) and shorten the three leg tubes on their own lines (feet move about 5 mm outward), in every list of every scale, the bones `grate`, `charcoal`, `lid`, `ash-damper`, the anchor `bowl-mouth`, and all elevation and front shapes. Not done: it touches every solid and every drawing.
2. **Not rendered and read after the last change**: pose `cold-closed` and `embers-vents-low` (the latter rendered at 1:50 three-quarter but not read), clip `light-and-close` at a second instant, thermal and structure sheets of 1:100 and 1:200, 2D drawings of 1:100 and 1:200, poses at 1:100.
3. **1:50 body, wheels**: plain tread, no ribs; hub cap only. **1:500 body, wheels**: 75 mm wide drums (kept from before).
4. **1:500 thermal**: `th-vents` and `lid-vent-flue-outlet` are 75 mm cylinders that reach 57 mm into the lid (kept from before; visible inside the raised lid in the thermal list). Should become the dial's 3 mm sheet disc and a flat outlet patch.
5. **1:200 body**: the vent dial is a 25 mm cap without holes and the lid shell still has the three 12 mm holes under it (also at 1:500 under the sheet disc); the shell holes at 1:200 and 1:500 are below the feature and should be closed in the mesh once the flue passage rule allows it.
6. **Lid**: 242 mm to the crown is on the tall side of the class (real lids about 200–235 mm); the lid is modelled as hinged at the rear rim, the real one lifts off and hangs on a hook (kept: the rig has no free lid).
7. **All scales, lid shell**: the dome mesh is stretched by up to 1.6 on the front side and compressed to 0.4 behind the dial by the slide; the vertices are exact on the sphere, but at 1:500 the facets on the front side are visibly larger. A fresh mesh with the holes cut at the dial would be cleaner.
8. **Structure**: the ash-damper mass shows as a ball below the bowl at 1:500; the cooking and fuel grate members were not re-examined in this pass.
9. **2D 1:50 plan**: the straps under the grip are drawn, the three vent holes of the tilted dial were not checked at full size.

### Fundamental issues (outside this asset)

- `S/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts` changed at 01:10 while authors were working: the untouched catalog asset went from `VALID` (01:05) to 65 `volumes-feature` findings (01:12) without any change to the asset. The findings are right in substance (coarse scales carried 1:50 fittings), but a rule that changes under 19 running authors invalidates their baselines; it should be announced in `T/r3-tools.md`.
- `r3-validate.ts --volumes` takes 7 to 8 minutes on this asset (four polyhedron shells), longer than the default tool timeout; a per-rule or per-scale switch would let an author iterate.
- `r3-sheet.ts`, `r3-2d.ts` and `r3-project2d.ts` cannot read a candidate document, so a rig change cannot be seen before the catalog is written; I used the earlier copy `T/r3-charcoal-grill-sheet.ts --file`. `r3-sheet.ts` should take `--file` like `r3-validate.ts`.
- `r3-sheet.ts --list thermal --cut` draws long thin spikes from the cut faces of polyhedron shells with holes (lid: from the vent to the rim; bowl: from the bottom holes upward). They are not in the uncut sheet; not verified whether the cut cap triangulation or the mesh is the cause.
- `volumes-insert.py` runs `main()` on import, so its layout function cannot be reused; the catalog file has two layouts (members written by `volumes-insert.py` and members in plain two-space JSON), and `representations` matches neither, which forces text surgery for root edits.

## Pass 1 (second round) (2026-11-22, afternoon)

Script (the chisel): `T/rework-charcoal-grill-wheels-grate.py` (uses `T/rework_15_kit.py`; without flags it writes the candidate `T/🗑️generated/rework-charcoal-grill/🔣️.json`, `--apply` writes the source and the changed catalog root members `rig`, `anchors`, `representations`, `poses`). Start state: source and catalog `volumes` were identical (diffed), the state of the first pass 1 above; it was no longer `VALID` under today's rules (four `volumes-feature` findings: `damper-plate` at 1:100, 1:200, 1:500 and `vent-dial` at 1:500 are true sheet discs that now need `sheet: true`).

### Reference table (57 cm kettle, Weber Original Kettle class), second look

| Quantity | Real | Model at the start of this round | After | Verdict |
|---|---|---|---|---|
| Overall height, lid closed | 0.99 to 1.07 m over the class (Original 57 about 1.00, E-5710 / Premium 1.07) | 1.03 m to the top of the grip | same | in class; decided to keep (see open list of the first pass, item 1: no real model is contradicted by 1.03 m) |
| Bowl / lid | 570 mm nominal, 585 over the beads, bowl about 250 deep, lid 200 to 240 | 585, 257, 242 | same | right |
| Lid straps | two bent 18 x 8 mm steel straps welded ON the dome, 116 mm apart | boxes that ran 5 mm (1:50), 4 mm (1:100), 3 mm (1:200) through the 0.8 mm shell into the lid: visible as two lugs inside the raised lid | end 0.5 mm in the dome surface (bottom from the dome sphere R 297.8 mm at the inner strap edge) | was wrong, now right |
| Wheels | two 152 x 38 mm plastic wheels that TURN on a fixed 10 mm axle through the rear legs, hub caps, moulded radial ribs on the web | solids bound rigidly to the bone `bowl`; plain web | bone `wheels` at the axle (x -0.14278, y 0.076, free revolute about the lateral axis, 0.6 kg taken out of the bowl mass; bowl 9.4 kg with its centre moved so the total is unchanged); five 30 x 9 x 8 mm ribs per wheel at 1:50 | rig was incomplete, now right |
| Moving the grill | lifted at the front handle and wheeled on the two wheels, front foot in the air | no such pose | pose `wheeled-tilted`: root turned 20 degrees about the axle, wheels counter-turned, contacts only the two wheels, front foot 0.15 m up | added |
| Cooking grate | 546 mm ring, 27 bars of 6 mm at 19.5 mm pitch on two cross rods | all 27 bars at every scale (1:50 parts list inside the 1:200 and 1:500 models) | 1:50 and 1:100: 27 true bars; 1:200: ring, 7 bars at 78 mm (as the plan draws it); 1:500: ring, 3 bars at 195 mm, both cross rods | coarse scales were a copy, now typified |
| Fuel grate | 416 mm ring, 9 bars at 42 mm | 9 bars at every scale | 1:200: 5 bars at 84 mm; 1:500: 3 bars at 168 mm | typified |
| Charcoal | 1.5 kg briquettes about 50 x 45 x 25 mm on a mound | 14 briquettes 48 x 24 x 38 at every scale | 1:200: 7 briquettes 54 x 30 x 42; 1:500: mound only | typified |
| Bottom damper, vent dial | 120 x 4.5 mm and 80 x 3 mm sheet discs | sheet discs without declaration (findings) | `sheet: true` where the disc is below the feature | rule change followed, form unchanged |

### Rebuilt

- **Rig**: new bone `wheels`; contact anchors `wheel-left` / `wheel-right` ride on it (x 0, y -0.076). The lid stays a hinge at the rear rim (the first pass's decision), `lid-vent` and `ash-damper` unchanged. No lateral rest placement is needed: the axle is the lateral joint axis of a sagittal bone.
- **Body**: wheels on `wheels` at all four scales (origin at the bone), ribs at 1:50; straps / posts at 1:50, 1:100, 1:200; grates, fuel bars, briquettes as in the table.
- **Thermal**: rib references in `legs-steel` (1:50); `th-grate-bar-*` follow the body bars at 1:200 and 1:500 (were every second bar, now the same typified bars as the body). 1:100 keeps all 27 rods: with 13 the measured rod surface is 0.177 m2 against `areaM2` 0.3 (41 %), so the true rods stay in body and thermal.
- **Structure**: `member-wheel-*`, `joint-wheel-bearing-*` and `support-wheel-*` belong to `wheels` (bearing joint on the axle, support pad under the tread); references to removed bars removed.
- **2D**: `wheel`, `wheel-hub` (elevation, bone-local now), `wheel-l/r`, `wheel-hub-l/r` (front, plan) bound to `wheels`; five rib capsules in the 1:50 elevation; an `axle` twin in the elevation of 1:50, 1:100, 1:200 (hidden behind the wheel) so the front and plan axle is carried at its true depth when the root tilts (without it the axle rose to the top of the wheels in the front view of `wheeled-tilted`).
- **Decisions**: one sentence pair per scale (en, de).
