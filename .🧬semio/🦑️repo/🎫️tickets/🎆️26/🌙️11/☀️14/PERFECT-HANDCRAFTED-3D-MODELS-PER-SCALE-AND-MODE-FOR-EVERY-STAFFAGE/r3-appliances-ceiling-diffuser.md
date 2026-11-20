# Round 3 — appliances/ceiling-diffuser (2026-11-20)

Status: DONE. Volumes spliced, root fixes written to the catalog file. `r3-validate.ts`: schema 0, validateAsset 0. `volumes-check.ts --product`: every rule green at 1:50, 1:100 and 1:200 except `volumes-thermal-coverage`. That rule fails only because the declared `areaM2` of `plate` and `collar` counts one face of the sheet. The values are not applied (C16 factor above 1.5) and are entered in `area-review.md`.

Scripts, all in `T`:
- `r3-ceiling-diffuser-model.py` writes all solids of all scales. Every section is stated by hand.
- `r3-ceiling-diffuser-roots.py` makes the root fixes outside `volumes`. It is idempotent, re-reads the catalog file and replaces only `provenance`, `rig`, `anchors` and `representations` with the splice formatter. With `--candidate` it writes a test document instead.
- `r3-ceiling-diffuser-probe.ts` prints the bounds and area of each solid's mesh.

## 1. Reference

| quantity | real value | source |
|---|---|---|
| nominal module / face | 600 grid, face 595 x 595 mm (Titus: module minus 1/4") | Koolair 50-FR-4-DECO (A = 595); Titus D-TMS-1.0 |
| frame / border width | 33 to 35 mm visible border | CAS AQN (C - A), Koolair 50-FR-4 (35 mm profiles), Air Master ACD4 (33 mm) |
| round neck (spigot) | DN 250, outer diameter B minus 1/8" = 248 to 249 mm, swaged bead for the clamp band | Titus D-TMS-1.0; spigot practice |
| neck height | 35 mm (TMS 24x24, 10" neck, dim. C) to 80 mm (AQN PDZ-1 plenum spigot) | Titus D-TMS-1.0; CAS AQN |
| overall depth | 130 mm (TMS 24x24, 10" neck, dim. D 5 1/8") | Titus D-TMS-1.0 |
| cone drop below the ceiling | 32 mm (TMS full face, dim. E); step-down cores project further | Titus D-TMS-1.0 |
| cones | 2 to 3 nested square cones plus a centre cone. Each cone is a pressed sheet that flares DOWN AND OUTWARD to a lip, and each inner cone sits lower and narrower (stepped face). The air leaves through the slots between the cones and is turned along the ceiling (Coanda effect). | Titus D-TMS-1.0 section; DIN EN 12238 |
| sheet | steel 0.6 to 1.0 mm, powder-coated; neck galvanised 0.5 to 0.8 mm | Titus TMS "Steel"; asset layers 0.8 / 0.7 mm |
| fixing | four screws through the border (surface mount), or lay-in; hanger holes | Titus border types 1 / 4V; CAS AQN |
| ceiling cutout | about face minus two borders (AQN 600: hole 540); the back of the plenum sits in the cutout | CAS AQN (E) |
| mass | 3 to 4 kg for a steel 600 multi-cone diffuser | product class; asset 3.2 kg (kept) |
| design flow | 300 m3/h for a 600 four-way diffuser is mid-range (Koolair 4-way 600: 600 to 1100 m3/h at 24 to 40 dB(A)) | Koolair selection table |

URLs (now also in `provenance.references`):
- https://www.titus-hvac.com/file/287/TMS.pdf
- https://cdn.shopify.com/s/files/1/0321/3613/4796/files/CAS_AQN.pdf
- https://koolair.com/wp-content/pdf/guia/50-FR-4-DECO_en.pdf

## 2. Audit (state before this round)

- **2D cones, all scales and both side views (root defect).** The cones were drawn as trapezoids that are wider at the top (0.265 -> 0.245, 0.2 -> 0.18, 0.135 -> 0.115). That is a stepped pyramid whose faces slope down and inward, and such a face cannot turn the air outward. On a real stepped multi-cone diffuser (Titus section) each cone flares down and out to its lip, and a slot opens under every step. The bands were also drawn contiguous, so no slot existed: the air had nowhere to leave.
- **Construction missing.** There was no plenum, no back plate and no air path. The collar sat on a solid 12 mm slab, so the duct air could not reach the cones. The cones were solid frustums, not sheets. Nothing connected the cones: they were stacked solids touching face to face.
- **Plan.** `flange-edge` was at 0.285 m, a fold the object does not have. The real seam between border and back plate is at 0.262 m (a 35.5 mm border). The screws at 0.27 m sat off the border centre line; the hangers are at 0.28 m. The opening (0.112 / 0.1 m) did not match a 0.7 mm spigot (bore 0.1238 m). The 1:200 collar was 0.13 m, against the 0.125 m tube.
- **Flange heights.** 1:100 drew 20 mm and 1:200 drew 40 mm for a 12 mm frame, more than the feature requires (15 and 30 mm). qa-visual already saw the 1:200 flange as thicker than modelled.
- **Rig.** Bone `body` had its joint at the centre with length 0.595 m, so its tip lay 0.2975 m outside the object (visible in every r3 sheet). `comY` was -0.010. The sheet areas of the 1:50 model put the centre of mass at -0.020 (cones 70 % of the sheet, below the ceiling).
- **Anchors.** `outlet-left/right` at (-/+0.262, -0.022) sat inside the first cone band. The real outlet is the mouth of the outer slot, at (-/+0.2585, -0.017).
- **Solids (old volumes).** Lofts of solid squares (exponent 12) stacked without gaps. Screw heads sat on the back face (ceiling side), invisible from the room. The jets were 40 mm cylinders hanging in the air at the flange edge (qa-visual). The structure was `of: flange` plus a mass and two support blobs (qa-visual: "one flat slab"). At 1:200 the mass was a box lying on the plate.
- **Kernel trap.** The old 1:50 `collar-bead` was an open revolve profile that the kernel filled (kernel-traps row). It is gone: the bead is now part of the closed collar loop.
- **Thermal area.** The declared plate 0.45 counted the room-facing face only. See section 6.

## 3. Root fixes (catalog file, old -> new)

| member | old | new |
|---|---|---|
| `rig.bones[body].lengthM` | 0.595 (tip 0.2975 m outside the object) | 0.2975 (centre of the spigot to the frame edge) |
| `rig.bones[body].mass.comY` | -0.010 | -0.020 (sheet areas x 0.8 mm of the 1:50 model) |
| `anchors.outlet-left` / `outlet-right` | (-/+0.262, -0.022) | (-/+0.2585, -0.017), mouth of the outer slot |
| 1:50 elevation and front: `cone-1`, `cone-2`, `cone-3` | wide-top trapezoids (see audit) | true outlines that flare down and out: cone-1 (0.135,-0.012) (0.255,-0.022) (0.265,-0.032); cone-2 (0.06,-0.032) (0.19,-0.04) (0.2,-0.05); cone-3 (0.071,-0.05) (0.123,-0.058) (0.135,-0.07), each mirrored |
| 1:50 elevation and front: `collar` | x -0.125, w 0.25 | x -0.1245, w 0.249 |
| 1:50 elevation and front: new `backplate` | none | rect 0.525 x 0.0075 above the ceiling line (z 2.8, region plate) |
| 1:50 elevation: new `collar-band` | front only | also in the elevation, at 0.026 (the bead) |
| 1:50 `slot-left` / `slot-right` | diagonal in the air at the frame edge | the shadow of the outer slot on cone 1: (-/+0.25,-0.0246) to (-/+0.17,-0.0179) |
| 1:50 plan | `flange-edge` 0.285, screws 0.27, collar 0.125, opening 0.112, bead 0.119 | 0.262, 0.28, 0.1272, 0.1238, 0.1245 |
| 1:100 elevation and front | flange 20 mm; cones (0.255->0.225), (0.18->0.12), wide on top | flange 15 mm; cone-1 (0.19,-0.015) (0.247,-0.03) (0.262,-0.045); cone-2 (0.11,-0.045) (0.16,-0.06) (0.17,-0.07); collar 0.249 |
| 1:100 plan | collar 0.125, opening 0.1 | 0.1245, 0.1238 |
| 1:200 elevation and front | flange 40 mm; cone (0.24,-0.04) -> (0.12,-0.07) | flange 30 mm; cone (0.262,-0.03) -> (0.135,-0.07); collar 0.249 |
| 1:200 plan | collar 0.13 | 0.1245 |
| `representations.*.decisions` | silhouette, retained, merged, omitted, thinElements of 50/100/200 | rewritten in English and German for the new drawings |
| `provenance` | 3 references, notes on the old shape | 3 more references (Titus TMS, Koolair 50-FR-4, CAS AQN) with URLs; notes describe the real construction |

Not changed: `dimensions` (0.595 x 0.12 x 0.595 m and 3.2 kg agree with the references), `thermal`, `structure`, poses and clips. The rig has one fixed bone, so poses and clips move nothing. The default pose `design` and the clip frame `switch-on` t=5 render identically (`r3-sheet --clip switch-on --t 5 --only right`). `r3-2d --pose off` draws the same shapes.

## 4. Model per scale

### 1:50 (feature 7.5 mm)

**Body: 12 solids, 24 expanded.**
- `collar`: closed revolve loop of the 0.7 mm DN 250 spigot (outer radius 0.1245) with a swaged 2.7 mm bead at 23 to 29 mm. It runs 4 mm down into the plenum.
- `flange`: square border frame 0.595 -> 0.524 m, 12 mm deep, corner radius 6 mm. Its top is marked `+v` (against the ceiling).
- `backplate`: the plenum back, 0.525 m square, with the spigot hole. It sits in the ceiling cutout. It is drawn at the 7.5 mm feature because `volumes-feature` refuses a 0.8 mm flat plate.
- `cone-1-rim` / `cone-1-cone` and `cone-2-rim` / `cone-2-cone`: tapered 0.8 mm sheet rings. Each has a 10 mm rim at 45 degrees from the lip (0.265 / 0.2 m half width) up to the fold, and a shallow cone rising inwards to an opening of 0.15 / 0.12 m.
- `cone-3`: closed centre dish, 0.27 m square at the bottom, with a 12 mm rim and a cone rising to a point under the spigot.
- `spacer-1..3`: 8 mm corner spacers, 4 copies each. They carry each cone from the one above, so no part floats.
- `screws`: four flat lens heads on the border at (+/-0.28, +/-0.28), `texture: true`.
- Slots of 10, 8 and 8 mm stay open.

**Thermal: 14.**
- 11 `of` references in region plate (layer sheet).
- `th-collar` (layer tube).
- Two outlet patches `jet-left` / `jet-right` (7.5 x 9 x 460 mm) in the mouth of the outer slot (D2).

**Structure: 11.**
- Members: `flange`, `backplate`, `spacer-1`, `cone-1-cone`, `spacer-2`, `cone-2-cone`, `spacer-3`. This is the real load path: the frame carries the back plate, and the back plate carries the cone pack through the spacers. Rims, centre dish, spigot and screws are carried, not carrying.
- Fixed joint at the rig joint.
- Mass at the centre of mass (0, -0.020).
- Pin: an 8 mm hanger stud through the frame into the ceiling.
- Roller: an 8 mm roller on the frame, against the ceiling.

### 1:100 (feature 15 mm)

- **Body: 4.** The spigot keeps its 0.7 mm wall (C30). Frame and back plate merge into one 15 mm flange with the spigot hole. There are two solid stepped cones, each with its 45 degree rim to the lip (0.262 / 0.17) and a 15 mm slot notch above it.
- **Thermal: 6.** Four references and two 15 mm outlet patches in the outer notch.
- **Structure: 6.** Flange member, four 15 mm corner posts (the merged spacers), joint, mass, pin, roller.

### 1:200 (feature 30 mm)

- **Body: 3.** A 30 mm flange, one cone mass (the envelope of the three steps: 0.262 under the flange to 0.135 at -0.07), and the spigot as a full cylinder (the plan draws it as a disc).
- **Thermal: 5.** Three references and two 30 mm patches where the cone leaves the flange.
- **Structure: 5.** Flange member, joint, mass, pin, roller.

1:500 stays unavailable, as in `representations`.

### Final check line (`volumes-check.ts appliances/ceiling-diffuser --product`)

| rule | result |
|---|---|
| silhouette 1:50 | IoU 0.997 / 0.996 / 1.000 (elevation / front / plan), p95 0 |
| silhouette 1:100 | IoU 0.999 / 0.995 / 1.000 |
| silhouette 1:200 | IoU 1.000 |
| availability, views-disagree, bounds, feature, monotonic, cross-scale, structure-coverage, bones, copied | all ok |
| thermal-coverage 1:50 | FAIL: plate 1.697 against 0.45, collar 0.090 against 0.04 |
| thermal-coverage 1:100 | FAIL: plate 1.114, collar 0.082 |
| thermal-coverage 1:200 | FAIL: total 0.953 against 0.49 |

The thermal-coverage failures are listed in `area-review.md`; nothing was applied there because the ratio is above 1.5. `r3-validate.ts appliances/ceiling-diffuser`: VALID (schema 0, validateAsset 0). With `--catalog-only --volumes`, the only findings are these 5 area lines.

## 5. What I saw in the final renders

Images: `🗑️generated/r3-sheet/appliances/ceiling-diffuser.s{50,100,200}.{body,thermal.cut,structure}.png`, `…s{50,100,200}.body.three-quarter.png` (seen from below, turn 40, tilt -30), `🗑️generated/volumes-tools/render/appliances/ceiling-diffuser.s{50,100,200}.png` and `🗑️generated/r3-2d/appliances/ceiling-diffuser.s{50,100}.png`.

**1:50 body**
- Front, side, back and left: the spigot with its bead line sits above the ceiling line on the thin back-plate band. Below the ceiling are the border frame and then three flared steps, each with a dark slot band under the step above. This matches the Titus section.
- Top: border and back plate with their seam. Through the spigot you see cone 1's square opening and the centre dish.
- From below (three-quarter): border with four screw heads, then three nested square cones with rounded corners and dark slot lines. The spacers are hidden in the slots, as on the real part.

**1:50 thermal (cut)**
- Plate in red: frame, back plate, sheets and dish, with the hollow plenum visible in the cut.
- Collar in green.
- The jet patches sit in the slot mouth.

**1:50 structure**
- Blue frame and back plate.
- Dark cone sheets and spacers seen edge-on.
- Magenta mass under the spigot.
- Pin left, roller right on the frame at the ceiling.

**1:100**
- Two stepped flared cones with notches, a 15 mm flange and the thin-walled spigot.
- From below: two nested rounded squares on the flange.
- Overlay all green.

**1:200**
- A flange slab, a tapered cone mass and the spigot cylinder.
- Overlay all green.
- Patches visible under the flange.

**2D (catalog, `r3-2d`)**
- 1:50 and 1:100 elevation and front show the flared steps with notches.
- The plan shows the seam square and four screws.
- Line and solid views are clean.

## 6. Still imperfect (stated plainly)

- **Thermal areas not decided.**
  - The honest outer area of `plate` is 1.70 m² (both faces of every sheet, because the plenum is open to the room through the slots) and that of `collar` is 0.09 m². The declared values are 0.45 and 0.04.
  - The ratio is above 1.5, so per C16 nothing was applied. Both are entered in `area-review.md` with the hand calculation.
  - Until the reviewers decide, `volumes-thermal-coverage` stays red at every scale.
- **Back plate thicker than the real sheet.** At 1:50 it is 7.5 mm instead of 0.8 mm, because `volumes-feature` refuses a thin flat plate. Only a hollow part keeps its real wall (C30), and the extent rule measures the axis-aligned extents. It sits in the ceiling cutout, which is real, so the plenum keeps its height.
- **Cone wall tapers.** The shallow cone parts are tapered extrudes, so their sheet thins from about 1.0 mm at the fold to about 0.5 mm at the inner opening. The kernel tapers the hole with the profile.
- **Cones flatter than real.** The cone parts rise only 7.6 to 15 mm, flatter than Titus cones (about 30 degrees). The drawn 70 mm depth, the 12 mm frame and the 7.5 mm feature leave no more height below the ceiling.
- **Mass versus layer thickness.** 3.2 kg agrees with the product class, but 0.8 mm sheet over the modelled area weighs about 5.2 kg. The layers `sheet` 0.8 mm / `tube` 0.7 mm look about 0.3 mm too thick for a 3.2 kg diffuser (about 0.5 mm in reality). This is not changed because it is thermal layer data; it is a point for the area and thermal reviewers.
- **Small gap closed in the drawing.** At 1:50 the inner edge of cone 2 stops 0.4 mm below cone 1. The drawing closes this gap (under the pen), as its `merged` decision says. Elsewhere the side view is closed by the overlapping cones.
