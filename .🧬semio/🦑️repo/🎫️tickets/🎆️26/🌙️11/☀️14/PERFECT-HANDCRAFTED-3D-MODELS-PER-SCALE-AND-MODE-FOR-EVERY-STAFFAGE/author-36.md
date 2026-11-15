# Author 36: machinery (excavator, forklift, heat-pump-outdoor-unit)

Generators: `author-36-excavator.py`, `author-36-forklift.py`, `author-36-heat-pump-outdoor-unit.py` (+ `author-36-lib.py`, a scale-selective copy of the author-03 lib), 2D repairs `author-36-fix2d-excavator.py`, `-2.py`, `-3.py` and `author-36-fix-hp-anchors.py`, area helper `author-36-areas.py`, probes `author-36-dump.ts`, `author-36-vol.ts`. Check line = `volumes-check.ts <asset> --product` (all four scales, run after the last change). All three assets are spliced.

areaM2 policy: where the round-1 author changed a value beyond a factor 1.5 I restored the committed value (the coordinator's brief); the honest exposure is then red at 1:50/1:100 and listed in `area-review.md` (rows appended, unapplied). Some values were restored or clamped by someone else while I worked (forklift drive-motor, mast; excavator cab-glazing 4.0 = 6/1.5; heat pump casing); I left those as found.

## machinery/excavator (spliced, all four scales)

Solids body / thermal / structure: 1:50 56/29/33, 1:100 31/25/32, 1:200 22/22/28, 1:500 18/20/27.

Check (product): silhouette, bounds, feature, bones, slender, copied, monotonic, structure-coverage green at all four scales (elevation/front/plan p95 at 1:50 11/21/24 mm against 15/30/30). Red lines, all stated:
- `volumes-thermal-coverage` 1:50 and 1:100: areaDeviation of house (0.41), counterweight (1.55), cab-glazing (0.65/0.59), hydraulics (0.56/0.55) and tracks (1.72/1.93). Cause: committed `areaM2` restored where round-1 had changed it beyond a factor 1.5 (house 30, counterweight 4.5, hydraulics 3.5, tracks 14; cab-glazing is 4.0; engine-compartment 2.58, exhaust-stack 0.188 and attachment 33.6 are within 1.5 and stay). The honestly modelled object exposes 17.6, 11.5, 1.45, 1.55 and 38.1 m2. Rows are in `area-review.md`. 1:200 and 1:500 (total only) are green (0.064, 0.059).
- `volumes-cross-scale` 1:100: only `unionVolumeBandExcess` 0.0013 (about 30 L of 22.7 m3). Tool defect, see `tool-requests.md` section 36: the product voxel cell is 21 mm while the band distance is 15 mm, so the band collapses to "volume 1:100 equals volume 1:50"; IoU 0.999/0.9999/0.996 and p95 0 to 26 mm in all views.

### 2D repaired at the root (all in the asset file, text-level)
- Front view, every scale: the `house` shape was the whole side profile swept over the engine half and drew it at cab height (1.97 m, front p95 0.36-0.40 m). Now the house is a rect of 1.25 m (hood height); the separate `cab-block` carries 1.97 m. At 1:500 the front also drew `house-counterweight` at cab height: that front shape is removed (the `house` and `cab-block` rects cover the front).
- Front, 1:50: the `hood-rail` extent was swept over 0.95 m of the roof (z -1.15..-0.2); the rail is one 50 mm line at z -1.15.
- 1:100, 1:200, 1:500: house and counterweight bottoms were 0.16 m lower than at 1:50 in all views, closing the real gap around the slew drum; that made the coarse body 1.8 m3 larger than the fine body (cross-scale volume). Bottoms are back at 1.0 m world, the `slew-ring` rect is drawn in the 1:100/200/500 elevation and in the 1:200/500 front like at 1:50, the cab block starts at the same height as the house.

### What I modelled by hand
Nothing is the 1:50 list filtered: `author-36-excavator.py` builds each scale from that scale's drawings.
- 1:50: kept the part vocabulary but corrected the 3D against the drawings: exact Bezier house rear corner, cab outline, counterweight outline; windscreen as a 25 mm sheet that follows the cab front (the old tilted box stuck out 138 mm at the roof corner); side panes as sheets flush in the cab wall (z 1.24-1.25); stack as a capsule with the drawn rounded top; hood rail as three 25 mm tubes (the old 353 cm2 panel and 50 mm bars were outside the drawn U); bucket floor lip inside the outline (it hung 50 mm below); grousers as texture ribs recessed 5 mm into the belt (they were 30 mm outside the silhouette); belt wall 70 mm with a through hole.
- 1:100: belts keep their hole but no lugs, hubs, rollers; web plate stands for frame and rollers; one merged window sheet, one boom-foot block, one rail tube plus posts, no sill and no stack cap, five teeth merged into the cutting lip the 1:100 outline draws, merged pivot bosses.
- 1:200: solid belt blocks, three masses for house/cab/counterweight, 40 mm window sheet and windscreen sheet, 32 mm hood panels, no bosses/rail/sill/teeth/boom foot, cylinders kept as barrel+rod because the 1:200 drawing keeps them.
- 1:500: two track blocks, car body, house, cab with one 80 mm window sheet, counterweight, boom, stick, 75 mm bucket shell with lip; stack and engine hood live in the thermal list only (below the drawing's resolution), cylinders kept in the body (barrels of 100-200 mm are above 75 mm, though the 1:500 plan omits them: blue strips in the plan overlay, p95 plan 169 mm of 300).
- Thermal: references to the body solids that carry each region's surface; stack jet disc; at 1:500 thermal-only stack capsule and engine-hood block. Structure: members per load-bearing bone, joints at 1:50/1:100, eight support pads, four masses; 1:100 and 1:500 merge the bucket cheeks into the shell member (monotonic strict drop).
- `decisions`: three real bilingual paragraphs per scale (geometry, thermal, structure).

### Supports (previous author filled the empty `structure.supports`)
Checked physically: the eight anchors are the track patches of the asset (four per belt at x -1.35/-0.45/0.45/1.35, z +-1.2, footprint 0.9 x 0.6 m, belt 0.6 m wide at z 0.895-1.495). The supports listed are exactly those eight contact anchors, correct for a tracked machine (no wheels or feet). I replaced the 300 x 500 mm pads of round 1 by 600 x 550 mm pads lying on the ground under the belt (inside the 3.3 m flat run, centred on the anchor x so `supportOffM` is 0); the anchors' 0.9 m footprints exceed the flat run at the two ends (1.8 m from centre against 1.65 m), left as authored.

### What I saw (sheets opened after the last change)
All four sheets: overlays green with thin red/blue edges only; the axonometric body reads as a tracked excavator in 3D (belts with web and wheels, deck and cab with window sheets, rail, grille panels, stack, boom/stick/bucket with scoop and teeth/lip); at 1:50 the front shows the engine half at 1.25 m and the cab at 1.97 m, the arm column between them. At 1:200 and 1:500 the plan shows red slivers at the counterweight edge (my counterweight is 1.2 m wide, the drawing 1.25 at 1:500) and a thin strip at the boom-stick pivot; 1:500 plan has the blue cylinders. Thermal cutaway: shells nested/regions complete; structure: members inside the body, green support pads under the belts.

### Left imperfect
- Five areaM2 deviations (above) are the committed estimates against honest exposure; needs a coordinator decision.
- 1:100 cross-scale volume (tool defect).
- The rest pose is not a closed linkage (as before).

## machinery/forklift (spliced, all four scales)

Solids body / thermal / structure: 1:50 52/45/47, 1:100 41/37/44, 1:200 32/32/43, 1:500 23/26/33.

Check (product): silhouette, bounds, feature, bones, slender, copied, monotonic, cross-scale and structure-coverage green at all scales (1:50 elevation/front/plan p95 6/24/13 mm against 15/30/30). Red: `volumes-thermal-coverage` only. 1:50: chassis-paint 0.36, battery-pack 0.49, drive-motor 0.77, mast 0.54, cargo 0.37 (1:100 the first four; 1:200 total 0.302 against 0.30). Cause: committed areaM2 restored (chassis-paint 8, battery-pack 4.5, cargo 5; drive-motor 0.9 and mast 5 were already restored when I started); exposures 5.13, 2.31, 3.14, 0.21, 2.32 m2. Rows in `area-review.md`.

### What was wrong and what I changed by hand (`author-36-forklift.py`, copy of the round-1 generator with fixes)
- 1:50 elevation p95 0.024 m came from the wheel arches: the 2D chassis is solid behind the tyre (no arch drawn), the 3D arch was cut 40 mm wider than the tyre on a 5-segment polygon (sagitta 18 mm cut into the tyre) and left a visible ring. Arches now follow the tyre circles at 1 mm clearance with 16 segments (all scales use the same chassis outline): p95 elevation 6 mm at 1:50, 11/15/38 mm at 1:100/200/500.
- Front guard post leaned the wrong way (bottom x -0.045, top -0.105 against the drawn -0.1 to -0.05) at 1:50 to 1:500: fixed. Steering column 7.5 mm tube flagged `slender` (it follows a drawn stroke, was a 40 mm tube), beacon a sphere like the drawn circle (was a cylinder with corners outside), seat back an extruded capsule outline like the drawn capsule (was a rotated box), counterweight vent plate flush inside the rear face (stuck out 5 mm), tilt-rod joint pin 20 mm (was 30 mm, sticking 9 mm out of the 25 mm rod at 1:50).
- The 1:100 to 1:500 models are still the round-1 structure (chassis/counterweight extrusions kept, parts dropped per scale, tyres plain revolves at 1:200, plain cylinders at 1:500, one guard slab at 1:500): I did not rebuild them from scratch; the brief only named the 1:50 p95 and the unopened sheets for this asset.
- decisions: the 1:50 geometry and thermal paragraphs updated for the arch clearance and the areaM2 policy.

### Supports
Four contact anchors (`contact-f-l/f-r` front wheels at z +-0.5, `contact-r-l/r-r` rear wheels at z +-0.24): physically right for a four-wheel truck (front tyres 0.2 m wide at z +-0.5, rear 0.16 m at +-0.24); the four 160 x 100 mm pads sit under the tyre contact patches on the axle bones, `supportOffM` 0 at every scale.

### What I saw (sheets of all four scales opened after the last change)
1:50 and 1:100: overlays green, red thin rims at the chassis and counterweight corners, blue at the cab roof seam in the front (guard slats); the axonometric truck reads right (chassis with arches, battery and counterweight, seat and steering, guard frame with slats, mast, carriage, forks, pallet with goods and straps, dished tyres). 1:200: no seat or wheel details as the drawing, red roof edge in front/elevation (3D roof 1.35 m against the stroke), blue strip below the pallet (feet box is a solid block, the drawing leaves gaps). 1:500: plain slab roof on four posts, plain cylinder tyres, blue diagonal of the tilt cylinders crossing the open cab (the 1:500 elevation does not draw them), red roof overhang (the 1:500 guard stroke starts 0.2 m inside the 3D roof).

### Left imperfect
- areaM2 reds above.
- 1:200/1:500 pallet and guard approximations as seen; 1:100 to 1:500 not rebuilt (see above).

## machinery/heat-pump-outdoor-unit (spliced, all four scales)

Solids body / thermal / structure: 1:50 26/29/18, 1:100 22/25/17, 1:200 13/16/17, 1:500 12/16/16.

Check (product): silhouette, bounds, feature, bones, slender, copied, monotonic, cross-scale, structure-coverage green at all scales (1:50 elevation p95 3.75 mm against 15 mm, was 24 mm). Red: `volumes-thermal-coverage` only: 1:50 casing 0.80, fan-guard 1.05, discharge-vane 0.66, feet 0.87 (1:100 the same, fan-guard 1.2; 1:200 and 1:500 only the total, 0.42 and 0.38 against 0.30). Cause: committed values (casing 3.4 was already restored by the coordinator; I restored fan-guard 0.6, discharge-vane 0.25, feet 0.1; base-tray 0.712, top-lid 0.655, compressor-casing 1 are within 1.5 and stay). Exposures 6.12, 1.23, 0.41, 0.187 m2. `coil-guard` (coil face and fins, committed 1.6) now carries `areaBasis: "exchange"`, so it is exempt and never corrected.

### What was wrong and what I changed
- 1:50 elevation p95 0.024 m: the elevation draws the fan guard as a solid 45 mm profile, the 3D guard was a wire basket with a 35 mm empty gap between the casing front and the rings (the whole gap boundary, 0.7 m, counts). I replaced the conical skirt rods by a bell-mouth sleeve (a 38 mm tube, the region title says "grille and bell mouth"), also at 1:100 (15.5 mm wall). The first attempt as a revolve was wrong: the kernel closes an off-axis profile with discs (area 0.9 m2, cancelling volume), so the sleeve is an extruded ring (circle with hole).
- Anchors repaired at the root: the file said front and rear rail (x +-0.13, 0.08 x 0.45 m) but the drawings and the 3D show two rubber feet at left and right (0.34 m deep, 0.18 m wide at z +-0.43). `contact-l/r` now sit at z +-0.43 with that footprint and left/right titles; the pads under the feet match exactly (`supportOffM` 0).
- 1:200 and 1:500 are boxes with the fan as a disc, as the drawings are; the 1:100 keeps the 1:50 parts with 20 mm walls and no spokes/second ring.
- decisions: added a paragraph at 1:50 and 1:100 for the sleeve and the anchors.

### Supports
Two anchors, both rubber feet (left/right), see above: physically right after the repair; the 3D feet and pads agree with the front (two feet) and elevation (one 0.34 m foot) drawings.

### What I saw (all four sheets after the last change)
1:50 and 1:100: green overlays, the fan sits in a round opening with blades, hub, rings, spokes and the sleeve, vane and lid with hinge rods, coil block and compressor compartment visible in the cutaway; 1:200 and 1:500: plain box with round fan disc, vane and lid plates; the front overlay shows a red strip under the casing between the feet (the 1:200/1:500 drawings close the 80 mm gap under the unit, the 3D keeps it: p95 front 82 mm and 94 mm, inside the limits) and at 1:500 a red slim strip on the plan front edge.

### Left imperfect
- areaM2 reds above (casing, fan-guard, vane, feet).
- The red strip under the casing at 1:200/1:500 (a 2D simplification, left).
