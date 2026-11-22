# Round 3 — vehicles/motor-scooter

State: DONE

Catalog file: `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🚜️vehicles/motor-scooter/🔣️.json`. Source: `volumes/vehicles/motor-scooter.json`. Scripts (all in the ticket folder): `r3-motor-scooter-kit.py` (rest frames, writers), `-parts.py` (shared hand-stated numbers), `-s50.py`, `-s100.py`, `-s200.py`, `-s500.py` (every section of every scale stated by hand), `-build.py`, `-project2d.ts` (outline of the drawn parts), `-roots.py` (root fixes outside `volumes`), `-sheet.ts`, `-suit.ts`, `-run.sh` (build, project, validate candidate, render), `-ship.sh` (run, apply roots, splice, validate catalog, product check).

The session of 2026-11-20 03:00 to 04:10 was interrupted after its splice of 04:06; this session re-ran the chain, compared catalog, source and candidate (identical, no foreign edits in this asset from the bulk passes; all four scales are declared and authored), looked at every sheet again, fixed three defects (below) and shipped again.

## 1. Reference

Subject: 125 cc four-stroke step-through scooter with a pressed-steel monocoque, Vespa Primavera 125 class.

| quantity | real value | source | asset |
| --- | --- | --- | --- |
| length | 1852 to 1870 mm | Piaggio data as listed by 1000PS / UltimateSpecs (provenance `piaggio-primavera-125`) | 1.85 m (`widthM`) |
| width over mirrors / bars | 735 mm | same | 0.74 m (`depthM`) |
| height to mirrors | 1150 mm | same | 1.15 m |
| wheelbase | 1340 mm | same | 1.34 m (axles x = +0.67 / -0.67) |
| seat height | 790 mm | same | seat top 0.775 to 0.83 m (stepped) |
| kerb mass | 126 to 132 kg | same | 130 kg |
| tyres | 110/70-12 front, 120/70-12 rear: outer radius 229 / 236 mm, rim 12 in = 305 mm bead seat | ETRTO (provenance `etrto-2023`) | radius 0.23 m both (the 6 mm of the rear is not modelled so that the ride loop closes), widths 110 / 120 mm, rim flange radius 0.166 m |
| front brake | disc 200 mm, single caliper | Piaggio | disc r 0.10 m on the hub side of the single-sided mount |
| rear brake | drum 140 mm | Piaggio | drum hub r 0.083 m (casing) |
| steering axis | rake about 25 degrees, trail about 80 mm | class value, my knowledge (authored-estimate) | rake 25 degrees, axle offset 25 mm, trail 82 mm |
| front suspension | single-sided leg on the right with trailing link and coil-over shock | Vespa construction | fork leg bent to z = -0.078, link 85 mm, shock outside the mudguard |
| engine unit | swinging unit pivoted under the cylinder, CVT belt case on the left, silencer on the right, one rear shock on the left | Vespa construction | pivot (-0.27, 0.205), arm 0.40 m, swing -8 to +8 degrees |
| monocoque | 0.8 mm pressed steel: leg shield, floor with tunnel, rear body with two side cowls | Vespa construction | thermal wall 0.8 mm under the paint film; structure spine, tunnel, two shell rails |
| handlebar | 0.68 m over the grips, grip 22 mm tube with 33 mm rubber | my knowledge | grips to z = +-0.34, grip r 16.5 mm |
| centre stand | two splayed legs about 0.2 m, swings about 90 degrees over centre | my knowledge | 0.215 m, down at 93 degrees |

## 2. Audit against the reference (state before round 3, committed catalog)

- Rig: `steer` joint at (0.42, 0.98) with the axis 17 degrees from the vertical (real 25) and 0.78 m long; the front axle 19.8 mm off the axis; `swing` pivot at (-0.2, 0.3), 0.1 m above the real pivot under the cylinder, arm 0.55 m with the rear axle 70 mm below the pivot line; seat hinge at (-0.22, 0.745), 0.1 m behind the seat nose, seat 0.58 m (real 0.69); stand 0.335 m long from (-0.1, 0.3) (a real stand is about 0.2 m), down at 84 degrees.
- Dimensions: correct against the manufacturer (1.85 x 0.74 x 1.15 m, 130 kg); unchanged.
- 2D: drawn against that rig; wheel as nested rings, leg shield a plate, no single-sided fork, engine a block.
- Solids: 29 / 20 / 13 / 9 body solids; box body, tube fork, disc wheels, no monocoque form, no engine unit; structure 33 solids at three scales (a copy), thermal without rims.

## 3. Root fixes (old -> new)

| item | old | new |
| --- | --- | --- |
| `steer` x, y, angleDeg, lengthM | 0.42, 0.98, -73, 0.78 | 0.2834, 1.0, -65, 0.8613 |
| `steer` mass com | 0.2205, 0.0372 | 0.42, 0.0 |
| `wheel-f` x, y, angleDeg | 0.7903, 0.0198, 73 | 0.8613, 0.025, 65 |
| `swing` x, y, lengthM, com | -0.2, 0.3, 0.55, (-0.3, 0.1) | -0.27, 0.205, 0.4008, (-0.2, 0.053) |
| `wheel-r` x, y | -0.47, -0.07 | -0.4, 0.025 |
| `seat` x, y, lengthM, com | -0.22, 0.745, 0.58, (0.29, 0.03) | -0.125, 0.725, 0.69, (0.34, -0.04) |
| `stand` x, y, angleDeg, lengthM, maxDeg, com | -0.1, 0.3, 178, 0.335, 90, (0.17, 0) | -0.14, 0.19, 185, 0.215, 95, (0.12, 0) |
| `frame` com | 0, 0.55 | -0.25, 0.5 |
| anchors `contact-f`, `contact-r`, `stand-foot`, `tailpipe`, `rider-seat` | (1.0103, -0.0474), (-0.47, -0.3), (0.335, 0), (-0.73, 0.16), (0.3, 0.06) | (1.0697, -0.0722), (-0.4, -0.205), (0.215, 0), (-0.644, 0.057), (0.3, -0.068) |
| pose `parked-on-stand`, `idling-seat-open` | root -2.23, stand 84 | root -1.675, stand 93 (front tyre and stand feet on the ground, solved) |
| pose `braking-nose-down` | swing 1.5 | swing 5.02 (rear tyre stays on the ground, solved), description rewritten |
| thermal region `rims` | absent | new (cast rims, spokes, hubs; 0.62 m2), with pose fractions in every pose |
| `areaM2` | body-panels 2.58, steer-assembly 0.6, headlamp 0.06, engine 0.255, exhaust-system 0.156, tyres 1.01, brake-disc 0.185 | 3.25, 0.4, 0.069, 0.46, 0.26, 0.79, 0.129 |
| provenance | | reference `piaggio-primavera-125` added, notes and summary rewritten, structure assumptions rewritten |
| symbols | | true-extent symbol redrawn to the corrected body |
| representations 50 / 100 / 200 / 500 | | elevation, front, plan redrawn from the body (E1): 47/37/38, 31/24/22, 14/12/10, 10/8/7 shapes, with decisions and pens |

Area note (C16): `engine` and `exhaust-system` are within a factor 1.5 of the authored 0.35 but 1.80 and 1.67 times the area reviewers' values, which were measured on the round-2 block. The hand sums are in `area-review.md` (section "r3 vehicles/motor-scooter"); the reviewers decide.

## 4. What is modelled

| scale | body | thermal | structure |
| --- | --- | --- | --- |
| 1:50 | 79: floor with six rubber rails, leg shield (nose, two swept wings, glovebox core, chrome tie), rear body with two cowls, lamps, four indicators, plate holder, grab rail with posts, hanger plates, rear shock; headset, headlamp, dash, switch housings, flanged grips, levers, mirrors on stalks, column, bent single-sided fork leg, link, stub axle, coil-over shock, caliper, mudguard channel; tyre, rim with flanges, five-spoke centre with windows, hub, disc / drum and cap; crankcase, cylinder, belt case with cover, air box, right arm, header, silencer, heat shield, tail pipe; stepped seat, hinge; stand legs, feet, bar, pivot | 130: paint film by reference with the 0.8 mm wall nested, single layers, engine / rims / brake regions, tail pipe outlet patch | 44: head tube, spine, hollow tunnel, two shell rails, four cross members, shock mount, hangers, rear shock; handlebar, column, fork leg, link, stub axle, shock; crankcase, belt case, arm; rims, spokes, hubs; seat pan; stand; 8 joints, contact pad, 3 masses |
| 1:100 | 40: nose merged into the shield core, dished wheel discs, plain engine block | 41 | 36 |
| 1:200 | 16 masses | 23 | 23 |
| 1:500 | 10 masses | 16 | 15 |

Fixed in this session after looking:
1. 1:50 leg shield: the glovebox core poked 10 to 30 mm through the front faces of the two wings (ragged intersection lines in the front and three-quarter views). The core front is now 10 to 20 mm behind the wing fronts at every height (`CORE` high values 0.447 / 0.462 / 0.44 / 0.39 / 0.345 -> 0.415 / 0.424 / 0.408 / 0.364 / 0.328).
2. Structure 1:50, 1:100, 1:200: the rear shock ended 0.1 m under the left shell rail, an open load path. 1:50: new `cross-shock` between the rails and `shock-mount` down to the shock eye; 1:100: `shock-mount`; 1:200: the shock runs up to the rail.
3. Structure 1:50: the spine top stood 30 mm behind the head tube; two new top sections lead it into the tube.

Final check (`r3-motor-scooter-ship.sh`): candidate `VALID`, print suitability without violations at four scales (min feature 0.330 / 0.316 / 0.299 / 0.290 mm), catalog `VALID vehicles/motor-scooter (catalog file as it is): schema 0, validateAsset 0`, `volumes-check.ts vehicles/motor-scooter --product`: every rule `ok` at every scale, `PASS (30.6 s)`.

## 5. What I saw in the final renders

- 1:50 body, sheet (front, right, back, left, top, three-quarter) and three-quarter at turn 50 / 60 / 140: reads at once as a Vespa-class scooter. Front: headset with round lamp, mirrors, clean nose between the two wings, indicators, mudguard over the tyre, shock and fork leg on the right only. Right: silencer with heat shield, header from the cylinder, five-spoke wheel with disc and caliper, link and shock. Left: belt case with raised cover, air box, rear shock, spoke face of the front wheel. Back: tail lamp, plate holder, cowls, grab rail. Top: tapered floor with rails, stepped seat, headset with dash. Joints are closed, nothing floats.
- 1:50 body in `parked-on-stand`: stand feet and front tyre on the ground line, rear wheel clear by about 25 mm, seat and steering closed. `idling-seat-open` rendered (right side), seat turns about the nose hinge.
- 1:50 structure (sheet and left view after the fix): continuous from the handlebar through column, fork leg, link and axle to the contact pad, and from the head tube through spine, tunnel, rails and shock to the engine unit and the rear wheel.
- 1:50 thermal cut: wall inside paint on every panel, engine and silencer as their own regions, rims separate from tyres, brake disc and drum.
- 1:100 body, structure, thermal cut; 1:200 and 1:500 body: new, simpler masses with the same proportions; at 1:500 the handlebar with mirrors is one spindle with raised ends.
- 2D 1:50 and 1:200 (`r3-2d.ts`): the three views are the outlines of the body, line hierarchy intact.

## 6. Still imperfect

- The rear shock is bound to the `frame` bone, so in `braking-nose-down` (swing 5 degrees) its lower eye is about 30 mm off the belt case; the rig has no telescopic bone.
- The renderer facets the 12 inch tyres and lofts visibly; the solids themselves are true revolves and lofts.
- Of the coarse lists I opened 1:200 structure (closed load path, shock up to the rail) and 1:500 thermal cut (every region present; the sheet tool paints the unlayered panel and steering regions grey because its legend only knows their 1:50 layers); 1:200 thermal, 1:500 structure and 1:100 thermal cut were checked by the product check only.
- Passenger footrests, number plate and brake hoses are not modelled.

## Round 5 handcraft (2026-11-21)

Scripts: the round-3 scale files were edited at the root (`r3-motor-scooter-s50.py`, `-s100.py`, `-s200.py`, `-s500.py`, `-roots.py`); `r5-v03-sheets.sh` (all sheets of one asset), `r5-v03-measure.py` (rig against reference), `r5-v03-check.sh`. Shipped with `r3-motor-scooter-ship.sh`.

### Reference and model against reference

| quantity | reference (Vespa Primavera 125, Piaggio data; ETRTO) | model | deviation |
| --- | --- | --- | --- |
| wheelbase | 1340 mm | 1340.1 mm (rig axles) | 0.0 % |
| length / width / height | 1852 / 735 / 1150 mm | dims 1.85 / 0.74 / 1.15 m, bounds within the check limits at every scale | < 1 % |
| wheel outer radius | 229 / 236 mm (110/70-12, 120/70-12) | 230 / 230 mm | +0.4 % / -2.5 % (rear kept so the ride loop closes) |
| head angle (rake 25 degrees) | 65 degrees from horizontal | 65.0 degrees | 0 % |
| trail | about 80 mm | 79.6 mm | -0.5 % |
| seat height | 790 mm | seat top 0.775 to 0.83 m (stepped) | within the step |
| centre stand leg | about 200 mm | 215 mm | +7 % (own estimate, not a published value) |
| swing pivot under the cylinder, arm to axle | about 0.40 m | 0.4008 m | 0.2 % |

### Grades (before -> after)

| scale | body | thermal | structure | poses/clips |
| --- | --- | --- | --- | --- |
| 1:50 | B+ -> A- | A- -> A- | A- -> A- | B (rear shock drifted 30 mm in braking) -> A- |
| 1:100 | B -> A- | B+ -> A- | B+ -> A- | |
| 1:200 | B- -> B+ | B -> B+ | B -> B | |
| 1:500 | C+ -> B | B -> B | B- -> B- | |

Opened: before and after, the body sheet of every scale; before, structure and thermal cut of every scale, every pose at 1:50 (11) and one frame of each clip (ride-loop 0.31 s, stop-and-shut-down 3.15 s), 2D of 1:50 to 1:500.

Defects seen before: the rear body was a box at every scale, with a flat vertical front face from the seat nose down to the floor and hard corners (a dark slab in the three-quarter views); the rear shock was one rigid part on the frame and left the belt case in `braking-nose-down`; no passenger footrests, number plate or front brake hose (round-4 queue); the 1:500 had no front mudguard (the scooter's most recognisable part); the 1:100 had no header pipe (exhaust area 37 % low under the C31 flood measurement); the catalog was INVALID (PRINT: tail-lamp feature 0.198 mm at 1:20).

Changes:
- Rear body (all four scales, root of the body, drawings re-projected by the pipeline): the front of the rear shell now rakes from a fillet on the floor (toe 15 to 45 mm high depending on the scale) up to the seat nose in four or three rounder sections (superellipse exponent 2.3 to 2.8), like the pressed rear shell of the real scooter; the top meets the seat hinge. The true-extent symbol follows.
- Rear shock (1:50): damper body and top eye on `frame`, piston rod, spring and lower eye on `swing`, overlapping by 60 mm, so the shock telescopes when the engine unit swings ±5 degrees and both eyes stay on their mounts; the structure carries both halves.
- 1:50 detail: folded aluminium passenger footrests on the floor edges, the number plate (texture sheet) on its holder, the front brake hose (8 mm, slender) from inside the headset down the front of the fork leg to the caliper; tail lamp 4 mm deeper so its glazing reads at 1:20 (print root fix, catalog VALID again).
- 1:100: 28 mm header pipe from the cylinder under the engine to the silencer, drawn as a detail; hinge knuckle under the seat nose (the structural seat hinge had been outside the body).
- 1:500: the front mudguard as a 75 mm crescent 0.2 m wide over the front tyre (starts at 40 degrees so the length stays 1.85 m).
- `areaM2` `rims` 0.62 -> 0.60 (C16, factor 0.97): measured outer area 0.52 at 1:50 and 0.70 at 1:100 under the C31 flood; listed in `area-review.md`.
- Decisions of 1:50 and 1:100 (en, de) name the new parts.

Final: `r3-validate.ts vehicles/motor-scooter --catalog-only` VALID (schema 0, validateAsset 0); `volumes-check.ts vehicles/motor-scooter --product` PASS (every rule at every scale).

Seen after (1:50 body sheet, 1:200 and 1:500 body sheets): the rear shell flows down from the seat into the floor, the plate sits under the tail lamp, footrests show as small bars at the floor edges, the 1:500 has its mudguard crescent; nothing floats.

Still imperfect: 1:500 handlebar is one 0.74 m spindle with raised ends standing for the mirrors (75 mm floor); 1:200/1:500 wheels are plain discs; the rear-shock halves are rigid (no telescopic bone), so in the extreme swing the spring leaves a 13 mm visible step on the damper body; the brake hose is not in the thermal list (its 0.006 m² would push `brake-disc` over 15 %).

## November 21 running gear and moving joints

Both cast wheels retain their five windows. The fork, two controls, curved brake levers and hollow steering tube follow the steering joint. The grooved rubber grips have a measured component area of 0.05032 m² and 0.02990 m² exchanges with air in the eleven authored postures. Two clamping halves, a bored steerer socket and twin pinch bolts join the 22 mm handlebar to the steering column. See the [measurements and installed revision](digital-running-gear.md).
