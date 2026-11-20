# Round 3 — objects/shopping-trolley

State (done, 2026-11-20): roots, 2D of all scales and views, and volumes of all scales rebuilt as the real Wanzl EL/ELX 101 class construction; catalog written and spliced. `r3-validate` VALID (catalog only and with source). `volumes-check --product`: every rule green at every scale except `volumes-thermal-coverage` at 1:50 and 1:100 on `frame-tube` and `wheels`, whose declared areas may not be corrected beyond the factor 1.5 of C16 (listed in `area-review.md`).

Scripts (all in T): `r3-shopping-trolley-kit.py` (hand-stated geometry, bone frames, helpers), `-roots.py` (rig, anchors, loads, titles, provenance), `-draw.py` (2D, 4 scales x 3 views, decisions), `-s50.py`, `-s100.py`, `-s200.py`, `-s500.py` (volumes), `-build.py` (candidate, `--source`, `--apply`), `-areas.json` (area corrections), `-suit.ts` (print-profile suitability of a candidate).

## Reference (real subject)

Class: a European 100 litre wire-basket shopping trolley with four swivel castors (Wanzl EL / ELX 101 type).

| quantity | value | source |
|---|---|---|
| L x W x H, EL 101 | 1002 x 600 x 978 mm, 100 l, castors o125, nesting 202 mm | Wanzl catalogue "Shopping trolleys" 1193 EN (2025), order section EL/ELX/Light |
| L x W x H, ELX 101 | 1035 x 590 x 978 mm, 101 l, o125, nesting 230 mm | same |
| D 101 RC / ELT 7/101 | 895 x 568 x 1075 / 902 x 573 x 1073 mm | same |
| mass | 15.0 kg (ELT 7/130), 14-15 kg (City Shopper 60 l): class 13-16 kg | same |
| asset dimensions (kept) | L 1.05, W 0.57, H 1.00 m, 14 kg: inside the class band | provenance `trolley-class` + new `wanzl-catalogue-2025` |
| castor | o125 wheel, 32 mm tread, rubber/PU on nylon hub, all four swivel, trail about 35 mm, castor height about 155 mm | catalogue ("4 swivel castors, o125, rubber tread"), class knowledge |
| chassis | o25 tubes: lower rail at about 0.17 m bending up at the rear into the push-frame uprights (back lean about 9 degrees) that carry the handle; rails converge to the front so the next chassis nests; open at the rear bottom; front cross tube; struts to the basket floor | class construction |
| basket | welded o4-5 mesh (pickets about 30 mm, hoops about 70 mm), o8 rim, o6 floor frame and corner wires; floor 0.62 m long, 0.42 m wide at the rear and 0.37 m at the front, rising 25 mm to the front (nesting); rim 0.915 m long, 0.51 / 0.43 m wide at 0.875 m; front wall sloping 38 degrees; 106 l geometric (prismoid) for 100 l nominal | class construction |
| rear gate | wire panel hanging from the rear rim bar, swings up and inward when the next trolley's nose enters | class construction |
| handle | o25 tube at 0.98 m, o35 PP grip, plastic corner caps, coin lock box at the centre | catalogue accessories (handle caps, coin deposit systems) |
| crate tray | wire tray on the rails with front and rear lips | catalogue ("load capacity of crate holder 20 kg") |

## Audit of the catalog state before round 3

- Rig: chassis bone at the ground origin, 0.7 m long and running past the front; basket bone at y 0.38 m (real floor about 0.54 m); handle a child of the basket (it belongs to the push frame); gate bone horizontal (angle 0, length 0.495) while the gate hangs vertically and is 0.33 m long.
- Basket: floor at 0.38 m, depth 0.50 m (real 0.32-0.34), rectangular in plan (0.92 x 0.57, real tapering 0.51 to 0.43), floor 0.64 x 0.40 flat; no front slope taper in plan, no nesting rise. Volume about 190 l (nearly twice the class).
- Chassis: a flat o14 rectangle at 0.17 m with four "forks" rising from it to the basket; no push-frame uprights at all (the handle stood on the basket rim), no front cross tube, closed at the rear (cannot nest), no tray.
- Castors: a bare wheel (old "revolve" wheels drawn 0.0625 r, area 0.063 m2 total) with a box plate; no swivel head, crown, fork legs or trail; front and rear tracks equal (z 0.20 / 0.24).
- Handle: a U of two short posts and a grip 0.13 m behind the basket; no caps, no coin lock.
- Gate: four o25 bars (finding `model-findings.md`: should be an o4 grid).
- 2D: the drawings followed the same wrong body (basket filled dark as a solid body, chassis line, posts); front and plan were constructed views, not projections.
- Structure: rim and corner members, joints at invented places; masses as floating points; the 1:50 check failed `structure-coverage` (outsideBody 10-14 mm) and `thermal-coverage` (gate-wire +26 %).
- Visual: the basket read as an opaque box; reviewers (qa-visual) could not grade it.

## Root fixes (catalog asset, old -> new)

- `rig.bones`:
  - chassis: origin (0, 0) length 0.70, 5.5 kg com (0.02, 0.15) -> origin (-0.295, 0.1675) = rear end of the flat rail, length 0.625 (to the front cross tube), 6.7 kg, com (0.247, 0.0885) local = (-0.048, 0.256) world (from tubes, castors and tray).
  - basket: parent chassis at (0, 0.38), length 0.92, 6.0 kg -> at (-0.065, 0.3725) = floor rear edge (-0.36, 0.54) world, length 0.62 (floor), 5.5 kg, com (0.40, 0.15).
  - handle: parent basket at (-0.4, 0.5), length 0.2, 1.2 kg -> parent chassis at (-0.2125, 0.815) = grip axis (-0.5075, 0.9825), length 0.055, 0.8 kg.
  - gate: angle 0, length 0.495, 1.3 kg at (-0.42, 0.5) of basket -> hinge on the rim bar (-0.035, 0.335) of basket, angle -84 (hanging), length 0.328, 1.0 kg, com (0.165, 0); joint 0..80 kept, pose `gate-raised` 65 degrees kept (now swings up into the basket correctly).
- `anchors` (chassis local): rear wheels x -0.295 world z +-0.22 -> x 0.0, y -0.1675, z +-0.25 (castor trail 35 mm behind the swivel at -0.26); front wheels x 0.33 world z +-0.22 -> x 0.56 (world 0.265), z +-0.198; basket-centre (0.02, 0.04) -> (0.31, 0.0155) on the floor.
- `poses` load `push`: handle (-0.15, 0.095) -> handle (0, 0) on the grip axis.
- `structure.assumptions`: wheel positions updated (en/de).
- `thermal.regions`: titles `frame-tube` "Chassis and handle tubes" -> "Chassis and handle tubes, castor housings"; `wheels` "Wheels and castors" -> "Castor wheels". Areas (C16): basket-wire 0.57 -> 0.78, handle-grip 0.067 -> 0.078, frame-tube 0.23 -> 0.345 (limit), wheels 0.063 -> 0.0945 (limit); honest values 0.48 and 0.14 listed in `area-review.md`.
- `provenance`: reference `wanzl-catalogue-2025` added; notes rewritten (en/de).
- `representations` (all four scales, three views): redrawn per ruling E1 from the same hand-stated geometry as the solids, same bones, roles and line hierarchy; basket and gate mesh as translucent `glazing` (pens.glazing 0.13 at every scale, it was null), tube strokes at true width (o25 = 0.5 mm at 1:50), real wire positions, castors with fork, head, plate, tyre and hub; gate keeps its extruded `solid` with a tapering span (`zTip`); decisions rewritten bilingually. Print-profile suitability: all four scales `authored` with no violation.
- `dimensions`, `materials`, `clips`, `presentation` unchanged.

## Volumes per scale

| scale | body | thermal | structure | what was modelled |
|---|---|---|---|---|
| 1:50 | 223 | 217 | 92 | every wire: 2 x 28 side pickets (o4.5 at 30 mm, stubs under 95 mm dropped), 13 front pickets, 4 hoops, 13 floor wires on 5 cross wires, o8 rim with rear hinge bar, o6 floor frame and corner wires; four mesh glazing sheets; gate with o6 frame, 20 o4 wires, 2 cross wires, 3 hinge eyes and its sheet; chassis tubes o25 with two-segment bends, o20 struts, front cross tube, rim and floor brackets; four castors (plate, ball-race head, crown, two fork legs 7.5 mm, axle, tyre with rounded tread, nylon hub); crate tray (frame, lips, posts, 13 wires, sheet); handle arms with two-segment bends, cross tube, o35 grip (revolve), caps, coin lock. Thermal: every non-glazing part by reference. Structure: tubes, struts, brackets, castors and wheels (chassis), rim/floor frame/corner wires (basket), gate frame, arms and cross tube (handle) by reference; gate eyes as joints; 6 basket fixings and 2 handle bends as joint ellipsoids; 4 contact pads; masses of basket, gate, handle and the four castors (the chassis centre lies in the air between the rails, so its concentrated masses are the castors). |
| 1:100 | 145 | 139 | 75 | mesh at 60 mm with o9 wires (two 1:50 wires merged, same surface), 2 hoops, 7 front pickets, 7 + 3 floor wires; gate 10 o8 wires, 1 cross wire, 2 eyes; tubes with one bend; castor plate and head one block, crown, two 8 mm legs, simplified tyre and hub; tray 7 wires; grip a capsule. |
| 1:200 | 57 | 74 | 54 | mesh planes with rim, floor frame, corner wires and 2 hoops (slender); tubes slender o25 (rear bend and front cross thickened to 30 mm), slender o18 struts; castors a wheel disc under a 60 mm housing block; tray plane in its frame with lips; gate panel in frame; o30 arms, o35 grip bar, coin lock. Thermal adds o18/o12/o16 bundles in the mesh planes for the surface the drawing omits. |
| 1:500 | 20 | 33 | 26 | four mesh planes and the rim; chassis two slender rods from the o19 front cross rod along the rails and straight to the handle corner (12 mm behind the grip axis so the 1.05 m length holds); four 75 mm castor blocks; gate panel and frame line; slender handle bar. Thermal adds a slender grip sleeve and o24/o18/o20 bundles. |

Final check line (`$BUN volumes-check.ts objects/shopping-trolley --product`, 166 s): `FAIL: 4 measurement(s) over 2 rule line(s)` = `volumes-thermal-coverage` 1:50 frame-tube +39 %, wheels +47 %; 1:100 frame-tube +37 %, wheels +39 % (C16 limit, see below). All other rules ok at every scale: silhouette IoU elevation/front/plan 1:50 0.988/0.932/0.981, 1:100 0.982/0.941/0.975, 1:200 0.987/0.945/0.958, 1:500 0.962/0.988/0.927; bounds within 1 mm (1:50); feature, monotonic, cross-scale, structure-coverage (outsideBody 0.8 mm at 1:50), bones, slender, copied green; thermal totals 1:200 15 %, 1:500 3 %. Areas measured at 1:50: basket-wire 0.782 (declared 0.78), gate-wire 0.119 (0.12), handle-grip 0.078 (0.078), frame-tube 0.479 (0.345), wheels 0.139 (0.0945).

## What the final renders show

- 1:50 body sheet: front: U of uprights and rails, the tapering basket with its front pickets and hoops, struts, tray lip, the rear castor pair outside the front pair, grip with caps and coin lock. Right and left side: rail bending up into the 9 degree upright, handle arm bending back to the grip, basket with 38 degree front slope and floor rising to the front, gate hanging at the rear, struts, brackets, tray with lips, castors trailing with fork, head and plate. Back: gate panel with 20 wires and three hinge eyes on the rim bar, coin lock under the grip. Top: rail trapezoid converging to the front, castor plates, floor wires, grip. Three-quarter (turn 50 and 230): reads at once as a real supermarket trolley; tyres, forks, plates, tray and gate all in place, nothing floating, joints closed.
- 1:50 gate-raised: the gate swings up and forward inside the basket about its hinge; 2D elevation, front and plan follow.
- 1:50 thermal: five regions in place (basket and tray wires, gate grid, tubes and castor steel, grip group, wheels). Structure: blue tube frame, rim and floor rings, gate frame, castors and wheels as the load path, orange hinge eyes, magenta castor/basket/gate/handle masses, green pads under the wheels.
- 1:100, 1:200, 1:500 body sheets: each a recognisable, proportioned trolley at its own abstraction; 1:200 thermal shows the bundles inside the mesh planes, 1:200 structure the frame, rings, hinge joint and fixings.
- 2D 1:50, 1:100, 1:500 (product drawings) look like the 3D: translucent basket, true tube widths, castors with forks.
- The r3 sheet renderer draws glazing opaque (dark planes); the product renders them translucent (45 % in solid views, transparent in line views).

## Still imperfect (stated plainly)

- `volumes-thermal-coverage` stays red at 1:50 and 1:100 for `frame-tube` and `wheels`: the old declared values (0.23, 0.063) came from the old model (o14 tubes, no castor housings, small wheels); the honest values are 0.48 and 0.14, beyond the factor 1.5 an author may apply (C16). Applied at the limit, listed for the area reviewers in `area-review.md` (2026-11-20 entry).
- Not modelled: wire welds, brake pedal, plastic deflector rings, coin-lock chain, child seat flap on the gate (an option of the class), tray nesting hinge (the tray is fixed).
- Castor fork legs are 7.5 mm (1:50) and 8 mm (1:100) instead of about 4 mm pressed steel: thickened to the scale's feature.
- 1:500 straightens the rail bend, upright and handle arm into one line and puts the bar 12 mm behind the grip axis; 1:200 and 1:500 coarse-scale wire bundles exist only in thermal.
