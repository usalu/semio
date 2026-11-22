# vehicles/vintage-car-1929 — running report

Earlier work on this asset: `r3-vehicles-vintage-car-1929.md` (round 3, full rebuild on the 3.20 m wheelbase) and the section in `rework/09.md` (first rework round: floating parts, load path). This file continues from there.

## Pass 1 (second round), 2026-11-22

Scripts (hand-stated numbers, in `T`): `rework-vintage-car-1929-pass1.py` (rig, wings, 1:500 spare and lamps, 2D, decisions; run once, not idempotent for the appended decisions), `rework-vintage-car-1929-pass1b.py` (thermal twins, radiator area; idempotent).

### State found

Source `T/volumes/vehicles/vintage-car-1929.json` and catalog `volumes` were identical (no half-finished edit). `--catalog-only` was `VALID`. Solids 1:50 / 1:100 / 1:200 / 1:500: body 221 / 131 / 96 / 45, thermal 102 / 68 / 56 / 38, structure 82 / 73 / 62 / 62.

### Reference table (Voisin C11 / C14 class saloon, 1927 to 1931; from my own knowledge, not looked up again)

| quantity | real subject | model | deviation |
|---|---|---|---|
| length / width / height | 4.40 to 4.50 / 1.65 to 1.72 / 1.70 to 1.75 m | 4.45 / 1.70 / 1.70 m | none |
| wheelbase | 3.20 to 3.22 m | 3.20 m (axles at x +1.70 and -1.50) | none |
| track | 1.35 to 1.42 m | tyre centres at z ±0.70 = 1.40 m | none |
| tyre and rim | balloon tyre about 0.74 m outside, section 0.13 m, rim seat about 0.49 to 0.51 m | radius 0.37 m, section 0.13 m by 0.125 m, rim seat radius 0.245 m | none |
| wheel | wooden or steel artillery wheel with 10 to 12 spokes, hub cap, drum about 0.30 to 0.33 m inboard | 12 tapered spokes, hub, cap, drum radius 0.155 m | none |
| wing | crowned crescent on a radius about 0.10 m larger than the tyre, 0.25 to 0.29 m wide, front wing running into the running board | arch radius 0.47 m about the axle, 0.29 m front and 0.25 m rear, 20 mm crown section; stations every 15 degrees (4 mm sagitta, flats visible in the shaded sheet) | fixed: 7.5 degrees |
| front axle | dropped I-beam on two semi-elliptic springs, kingpins about 1.19 m apart, stub axles, tie rod behind the axle, drag link to the drop arm | all present at 1:50; kingpins at z ±0.595; no steering pivot in the rig | open, see below |
| suspension | semi-elliptic leaf springs front and rear, travel about ±60 to 80 mm; pitch under braking small (rigid axles, stiff springs) | springs modelled, but both axle bones `fixed`, and the structure text said "no suspension is modelled" | fixed |
| bonnet | centre-hinged in two halves with louvred side panels, 1.0 m wide at the scuttle, about 0.57 m at the radiator | one fixed shell with hinge line and louvres; widths as real | hinge not rigged, open |
| doors | four, front hinged on the A pillar and rear on the C pillar, handles at the B pillar | panel joints, hinges and handles as real; not rigged | open |
| radiator | shell about 0.57 m wide and 0.68 m tall over the front axle with the core set back in it | shell and core at 1:50 and 1:100, a block at 1:200 and 1:500 | thermal lacked the core, fixed |
| sidemount spare | one spare in the right front wing, 0.74 m, with a disc cover | present at 1:50 to 1:200, missing at 1:500 | fixed |
| headlamps | two bowls about 0.24 m on a tie bar between the wings, centre about 0.98 m high | present at 1:50 to 1:200, missing at 1:500 (3 features large) | fixed |
| bumpers | two-blade bars front and rear, about 1.5 m wide | present in body at every scale; in no thermal list | fixed |
| frame | ladder frame, two channel rails 0.60 to 0.75 m apart with kick-up over the rear axle, 5 to 6 cross-members | as real, in body and structure | none |
| body frame | ash frame: sills, four pillars a side, cant rails, roof bows, floor bearers | as real in structure at every scale | none |

### Defects found by reading the sheets (all four scales, three lists, six views)

1. Rig: axle bones fixed although the car stands on four leaf springs; pose `braking` identical to `parked`.
2. Body 1:50: wing crowns were rows of 15 degree flats (front and rear, both sides).
3. Body 1:500: sidemount spare wheel and both headlamps left out, although 0.74 m and 0.24 m against a 75 mm feature; the 2D text even called the spare "below 0.1 m".
4. Thermal, every scale: bumpers, headlamps with bar and stalks, fender lamps, tail lamp and spare-wheel cover were in no region, so the thermal sheet showed a car without bumpers and lamps, while the 2D shapes of the same parts carry `body-panels` and `fenders`.
5. Thermal 1:50 and 1:100: the radiator region was the empty shell frame; the core (the part that rejects the heat) was missing and the engine block showed through.
6. 2D 1:200: the frame rail tips in the side elevation were 50 and 60 mm deep (redrawn to 75 mm, the depth that prints at the band end 1:350).
7. Not defects after checking: tyre section (12 point round profile), track, wheelbase, heights, radiator over the axle, door handles both at the B pillar (rear doors are rear-hinged), structure load path (rails, cross-members, springs with hangers and shackles, axles, bearings, hubs, spokes, rims, tyres, support pads; ash frame on the rails), one chassis mass at the centre of mass of the rig.

### Rebuilt

- **Rig**: `axle-f` and `axle-r` are prismatic along the vertical with ±0.08 m travel (leaf spring travel). Pose `braking` carries the computed load transfer of 0.05 g braking: 143 N per axle on about 40 N/mm gives 3.5 mm, so `axle-f` +0.0035 m, `axle-r` -0.0035 m and root pitch -0.125 degrees. A larger dive (12 mm, 0.4 degrees) was tried first and rejected: it is not what this car does at 0.05 g, and it pushed the 1:200 front drawing under the print minimum. `structure.assumptions[0]` now describes the springs and the travel.
- **Body 1:50**: the four wings (and their four thermal sheet twins) have a section every 7.5 degrees of the arch, inserted on the 0.47 m circle with interpolated orientation; 1 mm sagitta, the crown reads as a curve in every view.
- **Body 1:500**: `spare-tyre`, `spare-cover` (75 mm disc), `headlamp-l`, `headlamp-r`, `lamp-bar` (true 28 mm tube, `slender`), all touching the wings; the spare stands in the right front wing.
- **Thermal**: twins for bumper blades and clamps, tail lamp and spare cover in `body-panels`, headlamp bowls, lamp bar, stalks, posts and fender lamps in `fenders` (the regions of their 2D shapes) at every scale; `th-spare-tyre` at 1:500; `radiator-core` in `radiator` at 1:50 and 1:100 and `radiator-cap` at 1:200. `thermal.regions[radiator].areaM2` 0.75 to 1.1 (hand value: two core faces 0.58, shell band 0.26, face rings 0.17, neck and cap 0.03, recess edges 0.06; the old value was the shell alone including its inner band, which the core now covers). All regions within the limits at all scales.
- **Structure**: no solid changed; checked on all four sheets.
- **2D**: 1:500 elevation gains `headlamp-bowl`, `spare-tyre`, `spare-cover`; front and plan gain the two headlamps; the spare is left out of front and plan with the reason stated (0.06 m proud of the cabin side, inside the outline). 1:200 elevation `frame` tips redrawn. 1:500 `decisions.retained` and `decisions.omitted` rewritten.
- **Decisions**: one sentence pair added to `volumes.50.decisions` (wings, spring travel), one to `volumes.500.decisions` (spare, lamps), one thermal sentence pair at every scale.
- Catalog file: only the members `rig`, `poses`, `structure`, `representations`, the one `areaM2` number and `volumes` were rewritten; `representations` and `structure` are now laid out by the dump of `volumes-insert.py` (content unchanged apart from the edits above).

### Looked at after the last geometry change

1:50, 1:100, 1:200, 1:500 body, thermal and structure sheets; 1:50 three-quarter, front and right close-ups; 1:50 poses `braking` (close-up), `driving`, key poses and the clips `drive-by` (1 s, 3 s) and `arrive-and-park` (1.3 s, 5 s) rendered, of which `braking` and `arrive-and-park` 1.3 s were read (the others differ only by wheel angle and root position); 2D 1:500 and 2D 1:200 in `braking`.

### Validation

`r3-validate.ts vehicles/vintage-car-1929 --catalog-only --volumes`: `VALID vehicles/vintage-car-1929 (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0`. Source and catalog `volumes` are equal.

### What remains imperfect, plainly

- Rig, 1:50 to 1:500: no steering. The two front wheels are one bone `wheel-f` on `axle-f`; real pivots are the two kingpins at z ±0.595 (axis vertical, about ±35 degrees). Needs `knuckle-l/-r` (lateral rest placement, joint axis up) with `wheel-f-l/-r` on them, the stub axles, back plates, steering arms and drums moved to the knuckles, every `wheel-f` solid of the three lists at four scales rebound with its lateral offset, the 63 2D shapes bound to `wheel-f` split, and poses and clips given both wheel angles. No sibling vehicle has this yet (`city-car`, `delivery-van` use the same single bone).
- Rig: doors (four, hinge axes vertical at the A and C pillars) and the two bonnet halves (hinge along the centre line) are fixed parts of `chassis`; the tub sides are single panels with drawn joints, so opening them needs the panels cut into door leaves at every scale.
- Rig: when an axle travels, the leaf springs stay at static camber (they are `chassis` solids); at the ±3.5 mm of the only pose that uses the travel nothing opens, at full travel the spring seats would slide on the axle.
- Thermal 1:200 and 1:500: the radiator is one closed block (core and shell merged), right for the feature size but not hollow.
- Thermal: brightwork (nickel) counts with painted sheet regions, as the 2D drawings already did; a region of its own with the low emissivity of nickel would be more exact.
- 1:500 spare wheel is not drawn in front view and plan (reason above).
- 2D of 1:50, 1:100, 1:200 were not redrawn in this pass (the 1 mm wing change does not move them); they were not re-read against the 3D with `r3-project2d` in this pass.
- Structure: one lumped chassis mass in the cabin (the rig's mass model); engine, gearbox and rear axle are not separate masses.

### Fundamental issues (outside this asset)

- `r3-validate.ts` print check: a pose that only pitches the root by 0.4 degrees changed the measured feature of a chassis-bound front-view rectangle (`frame`, 1:200) from passing to 0.195 mm, and a 10 mm axle offset changed `axle-r` to 0.177 mm; the message names neither the view nor the pose, so the author has to bisect. Wish: name view and pose in `PRINT range-end-feature`.
- The catalog file mixes three layouts (`json.dumps(indent=1)`, the `volumes-insert.py` dump, and a third one in `representations` and `structure`), so no tool can rewrite a root member byte-identically; a shared root-member splice tool with one layout is missing.
