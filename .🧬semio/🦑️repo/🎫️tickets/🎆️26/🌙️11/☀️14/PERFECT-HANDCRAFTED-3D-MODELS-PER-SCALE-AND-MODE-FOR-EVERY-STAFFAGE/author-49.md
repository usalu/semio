# Author 49: machinery (forklift, concrete-mixer-truck, diesel-generator, evaporative-humidifier)

Generators: `author-49-forklift.py` (1:100/200/500 anew), `author-49-mixer.py` (all four scales), `author-49-diesel.py` (copy of the author-03 generator, scale-selective, silencer and louvre changes), the humidifier was edited by hand in `volumes/machinery/evaporative-humidifier.json`. They use `author-36-lib.py` (scale-selective write). All four assets are spliced with `volumes-insert.py` after the last change. Check line = `volumes-check.ts <asset> --product` (all scales, run after the last change); the render sheets of every authored scale were opened with the Read tool after the last change (forklift 100/200/500; mixer 50/100/200/500; diesel 50/100/200/500; humidifier 50/100/200).

Final check lines (only `volumes-thermal-coverage` is red, everything else is green: silhouette, views-disagree, bounds, feature, monotonic, cross-scale, structure-coverage, bones, slender, copied):

| asset | red lines |
|---|---|
| forklift | 1:50 chassis-paint 0.36, battery-pack 0.49, drive-motor 0.84, mast 0.54, cargo 0.37; 1:100 the first four (1:200 and 1:500 green, total within 30 %) |
| concrete-mixer-truck | 1:50 and 1:100 engine-bay 0.72 / 0.70 only (drum, chute, hopper green, cross-scale band excess gone) |
| diesel-generator | 1:50 and 1:100 discharge-hood 1.28 / 1.27 only (1:500 silhouette and the louvres green) |
| evaporative-humidifier | 1:50 and 1:100 trough 0.41 / 0.56 only |

All of these are committed or decided `areaM2` values against the honest exposure; none was changed by me (decisions in `area-decisions.md` followed); rows with measurements are appended to `area-review.md`.

## 2D repaired at the root (asset file, text level)

- forklift 1:500 elevation `guard-frame`: the roof stroke started at x = -1.0 while the plan and every other scale draw the roof from x = -1.2 (a stray shortening of 0.2 m). Path now `M -1.0,1.05 L -1.0,2.125 M -1.2,2.125 L 0.15,2.125 M -0.05,0.96 L -0.05,2.1` (rear post and roof are separate sub-paths).
- evaporative-humidifier 1:50 plan `film`: rect 0.38 x 0.34 at (-0.19, -0.17) was smaller than the pad it lies on (elevation 0.40 wide, front 0.36 wide, 3D block 0.40 x 0.36 x 0.40) and than the trough interior; now (-0.20, -0.18, 0.40 x 0.36), the 3D film follows (0.398 x 0.358 at 1:50 and 1:100, 1 mm clear of the trough walls). The elevation (0.40, x axis) and front (0.36, z axis) never contradicted each other: they are two different axes of a 0.40 x 0.36 pad; the contradiction was the film in the plan.

## machinery/forklift (1:100, 1:200, 1:500 modelled anew; 1:50 untouched)

Solids body / thermal / structure: 1:50 52/45/47 (author 36), 1:100 33/28/38, 1:200 28/28/35, 1:500 22/25/32.

Each scale is built from its own drawings and is not a filtered 1:50 list any more:
- 1:100: each fork is one extruded L (heel and tine, 15 mm), the fork backrest one extruded U frame (front profile on the rotated fork bone, local-plane extrusion), the overhead guard roof one plan extrusion with six openings between 50 mm slats (was rails plus slats), the pallet one comb extrusion (deck and three runners), seat cushion and back one extruded L, steering wheel an eight-segment ring, column a slender 15 mm tube, rim and hub of each wheel one brake drum cylinder; bolts, straps, mast pivot, brackets and trunnions are not modelled.
- 1:200: chassis and counterweight are plain blocks without arches (the 1:200 drawing fills the chassis behind the tyre), cowl and cover are one dash extrusion plus a cover plate, guard a 60 mm roof slab on four 60 mm posts as the drawing fills it, pallet a comb extrusion (the drawing gaps the runners; a solid block showed a blue strip), forks L, backrest 60 mm U frame, tyres plain revolves with a 40 mm brake drum on the outer face. This also brings the total area deviation under 30 % (0.302 before, green now).
- 1:500: two solid blocks, roof slab 75 mm on 75 mm posts, four mast rails, plate, forks as L under a 75 mm backrest bar, one pallet block, goods, plain cylinder tyres; drive motor, controller and brakes are real thermal solids at their places. The two tilt cylinders stay 75 mm bars in the body (the structure needs members inside the body; they show as a thin blue diagonal across the cab since the 1:500 elevation does not draw them).
Supports: four contact anchors under the tyres at every scale (`supportOffM` 0). `decisions`: three bilingual paragraphs per scale, rewritten.
What I saw: 1:100 and 1:200 overlays green, thin red rims at the chassis and counterweight corners and a blue sliver at the cab roof in the front; 1:500 roof edges show small red squares at both ends (the 150 mm pen stroke against my 75 mm slab), the blue tilt diagonal; axonometric views read as a forklift (chassis with arches at 1:100, blocks at 1:200/500, guard grate at 1:100, mast, carriage, forks, pallet with goods).
Left imperfect: area reds above (drive-motor: no motor housing exists in any drawing; mast: the four rails are filled rectangles in plan so boxes, the hand calculation counts open C channels); 1:100 to 1:500 share the chassis side profile and the mast rails with 1:50 because the drawings are identical there.

## machinery/concrete-mixer-truck (all four scales rebuilt)

Solids body / thermal / structure: 1:50 69/71/49, 1:100 50/52/47, 1:200 33/35/44, 1:500 15/23/38.

- The drum was not a real shell: the belly was an open off-axis profile, which the kernel closes through the axis (a solid cylinder, `revolve-implied-closure`), the cone a hollow ring. Now the drum is a real revolved hollow loop at every scale: dome and belly one loop, cone a second loop (1:50 8 mm, 1:100 15.5 mm); 1:200 and 1:500 one loop for the whole drum (31 / 75 mm), open at the mouth. The helical flight is a rib on the inside of the wall (it was buried in the solid belly), the seam capsule ring became one 60 / 100 mm weld band (`texture: true`, 1:50 and 1:100 only).
- Hidden mix volume: a `part: "mass"` revolve `mass-mix` in the structure list fills the inner contour from the dome to 3.9 m along the axis (12.6 m3 as the envelope of the 9 m3 nominal charge; the lump simplifies the free surface; the drum holds 16 m3). It buries the inner skin and the flight there, so the drum exposes 39.3 m2 at 1:50 and 40.1 at 1:100 against the committed 36 (the old model exposed 51.8 / 52.1). It stops at 3.9 m because the rest-pose chute enters the cone there: filling the whole cone buries 65 % of the chute (8.17 to 2.9 m2). This is a finding: in the default pose the chute passes through the drum cone (the plan and elevation draw it on the centre line inside the cone); it is a rig/pose problem, not modelled away.
- Chute: 1:50 keeps walls and floor as real thin plates, 1:100 to 1:500 are one U-section trough per chute extruded along its bone (15.5 / 31 / 75 mm walls), the troughs are members as channel beams. Frame: real 130 / 150 mm rail cores (`member-frame-rail`, inside the rail walls) are the members at 1:50 to 1:200, the 1:500 frame block is its own member.
- 1:200: cab one 2.4 m block, frame two plain rails with deck and bumper, hopper five sections, tyres plain revolves with brake drums. 1:500: frame block and cab block as the 1:500 front draws one block, drum shell, hopper four sections, tyre discs, grille, drive housing and brakes thermal-only.
- Cross-scale band excess at 1:100 (0.003): gone; it came from the solid belly (16 m3 of volume that the coarser scales did not reproduce): all scales are hollow shells now.
What I saw (all four sheets): overlays green with thin red/blue edges; the axonometric views show the shell with its rib in the thermal cutaway, the chute crossing the cone, hopper, frame with cross members (1:50/1:100), arched cab at 1:50/1:100, block cab at 1:200/500; the structure view shows frame, pedestal, drum and the pink chassis mass.
Left imperfect: engine-bay 0.91 against 3.2 (only the grille plate is drawn, the engine tunnel lies inside the cab block); the chute/cone interpenetration; the mix lump is an envelope, not the true free surface.

## machinery/diesel-generator (1:500 silhouette repaired; louvres; same checks)

Solids body / thermal / structure: 1:50 31/34/23, 1:100 27/30/22, 1:200 22/25/21, 1:500 12/19/19.

- 1:500 elevation p95 0.34 m: it was not the silencer shape. The silencer capsule (and every alternative) sat exactly tangent on the roof; the product raster of the 3D union then keeps a one-cell hairline gap between the two, open at the silencer ends, so the silencer's whole lower edge counts as a 3D boundary the drawn silhouette lacks (distance up to 0.55 m). Found with a per-view histogram (`author-35-hist.ts`) and a 2D-shape drop test. Fix: the silencer saddles 10 mm into the roof (and is now one revolve with the drawn rounded-corner outline: flat 110 mm end, 220 mm corner radius, instead of a hemispherical capsule). p95 elevation 0 mm, silhouette green. Tool request appended (`tool-requests.md`, section 49).
- 1:50 louvres: 60 slats were 25 mm boxes (9.36 m2 exposed against the decided 5.8). They are real 2 mm aluminium sheets now, `texture: true` (the louvre is a grille, C10): 5.77 m2, green; they are only faintly visible in the render, as the real 2 mm blades are.
- The other scales are scale-specific already (author 03): 1:100 eight 40 mm slats per panel, 1:200 four flat 31 mm panels, 1:500 three slabs with thermal-only louvre panels. Nothing was filtered from 1:50.
What I saw (50, 100, 200, 500): overlays green, red rim at the silencer ends (the 2D rounded polygon), blue sliver on the hood slope; hood hollow with its open mouth and flap in the cutaway; nested wall skin and wool shell.
Left imperfect: discharge-hood 9.14 m2 exposed against the decided 4 (hollow duct: outer 4.2 plus the same on the inner faces); the 1:50 louvres are barely visible.

## machinery/evaporative-humidifier (1:50, 1:100, 1:200; 1:500 unavailable)

Solids body / thermal / structure: 1:50 14/15/17, 1:100 14/15/16, 1:200 10/11/15.

- 2D repaired at the root (above); 3D film follows. No other change: the three models were already built per scale (hollow casing, pad block, trough, water, drops) and read right in the sheets (casing with end windows onto the pad, trough rim, drops, sump).
- Pad and fins for the reviewer: the pad region (`pad`, committed 1.2 m2, `areaBasis: "exchange"`, wetted cellulose flutes) is the only exchange area; the asset has no fins. It is modelled as the 0.40 x 0.36 x 0.40 m envelope block (+ film and drops), exempt from the area comparison.
- trough: decided SET 0.2, exposed 0.118 (1:50) / 0.087 (1:100): the water film covers the 0.144 m2 floor face, only rim and walls count; this equals the committed 0.1 within 18 %. Proposal in `area-review.md`: revert to 0.1.
What I saw (50, 100, 200): overlays 1.000 in all views, thermal cutaway with nested casing / pad / trough / sump, structure with rails, brackets and the base pad.

## Areas for the reviewer (appended to `area-review.md`)

forklift chassis-paint 8 vs 5.13, battery-pack 4.5 vs 2.31, drive-motor 0.9 vs 0.14, mast 5 vs 2.32, cargo 5 vs 3.14; mixer engine-bay 3.2 vs 0.91; diesel discharge-hood 4 vs 9.14; humidifier trough 0.2 vs 0.118, pad exchange 1.2 (listed). Nothing was applied.
