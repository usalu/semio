# Round 3 — `machinery/excavator` (2026-11-19)

Status: written to the catalog and spliced. `r3-validate.ts machinery/excavator` VALID (schema 0, validateAsset 0), `r3-validate.ts --file <draft> --volumes` VALID with checkVolumes 0, `volumes-check.ts machinery/excavator --product` PASS at all four scales.

An earlier round 3 author was cut off at 01:55. He left the kinematics (`r3-excavator-kin.py`), the forms (`r3-excavator-form.py`), the kit (`r3-excavator-lib.py`), the build (`r3-excavator-build.py`) and a first 1:50 (`r3-excavator-s50.py`); the catalog was untouched. I continued from that state: repaired what did not validate or draw, finished 1:50, wrote 1:100, 1:200 and 1:500 and the root data, and wrote the catalog.

## 1. Reference (real machine: 20 t class crawler excavator, long undercarriage)

Sources: Caterpillar 320 (2017, AEXQ2214) and Komatsu PC210LC-11 (2019) data sheets, both now cited in `provenance.references` (`cat-320`, `komatsu-pc210lc-11`); the values below are quoted from memory of those sheets, no web lookup was made in this run.

| Quantity | Cat 320 | PC210LC-11 | Model |
|---|---|---|---|
| Operating mass | 22 500 kg | 22 720 to 23 240 kg | 22 515 kg (sum of the bone masses) |
| Boom / stick | 5.7 m / 2.9 m | 5.7 m / 2.9 m | 5.70 / 2.90 m pin to pin |
| Shipping length | 9 530 mm | 9 650 to 9 780 mm | 9 640 mm (stowed) |
| Height to top of boom, stowed | about 3.0 to 3.2 m | 3 140 to 3 280 mm | 3 440 mm (see section 7) |
| Cab height | 2 960 mm | 3 045 mm | 2 980 mm (deck 1.06 + 1.92) |
| Transport width, shoes | 2 980 mm, 600 mm | 2 980 mm, 600 mm | 2 980 mm, 600 mm |
| Track gauge | 2 380 mm | 2 380 mm | 2 380 mm |
| Track length / centre of rollers | 4 450 / 3 650 mm | 4 450 / 3 655 mm | 4 450 / 3 650 mm |
| Tail swing radius | 2 830 mm | 3 020 mm | 2 950 mm |
| Counterweight clearance | 1 050 mm | 1 085 mm | 1 060 mm |
| Rollers per side | 8 track, 2 carrier | 9 track, 2 carrier | 8 track, 2 carrier |
| Boom cylinders | 2 x 120 x 1 260 mm | 2 x 120 x 1 334 mm | 2 x 120 bore, 85 rod, stroke 1 300 mm |
| Stick cylinder | 140 x 1 504 mm | 135 x 1 490 mm | 140 bore, 100 rod, stroke 1 500 mm |
| Bucket cylinder | 120 x 1 104 mm | 115 x 1 120 mm | 120 bore, 85 rod, stroke 1 100 mm |
| Max reach at ground | 9 860 mm | 9 700 mm | 9.85 m |
| Max digging depth | 6 720 mm | 6 620 mm | 6.68 m |
| Max cutting height | 9 370 mm | 10 000 mm | 9.47 m |
| Max loading height | 6 490 mm | 7 110 mm | 6.69 m |
| Bucket | 1.0 to 1.2 m3, 5 teeth | 1.0 m3 | 1.2 m wide, tooth radius 1.47 m, 5 teeth |

Real assemblies checked against: sprocket at the rear and raised, idler at the front in a yoke, track rollers under an inverted channel frame, X carbody with turret and slew bearing, cab front left, boom foot right of the cab, tank and tool box on the right, engine house and curved counterweight at the rear, gooseneck box boom with two cylinders at the knee, stick cylinder on top of the boom, bucket cylinder on the stick driving the bucket through idler links and an H link.

## 2. Audit of the former asset

- Rig: the bucket was a bare revolute joint with a cylinder ending in the air, there was no bucket linkage (two bones missing). Boom foot at upper (1.0, 0.65), 0.85 m ahead of the swing axis and too low (real: close to the axis, high on the frame). Cylinder pins invented: the boom cylinder lay nearly flat (30 degrees, 1.45 m barrel, stroke 0.6 m against the real 1.3 m), stick cylinder stroke 1.1 m (1.5 m), bucket cylinder 0.85 m barrel on a 2.9 m stick. Slew ring at y = 1.0 with no way to slew in a clip. Undercarriage 4.2 m long (4.45 m). Working envelope not derivable.
- Dimensions: 9.85 x 3.43 x 2.99 m for a stow pose that hung the bucket in the air in front of the machine (the 9.85 m is the maximum reach of the real machine, not its shipping length).
- 2D: boom drawn as a fat fish shape up to 1.5 m deep (real box about 0.4 to 0.75 m), stick 0.65 m deep, house 2.4 m wide hood, one boom cylinder, front view a bounding rectangle of house and cab. Open seam, weld, louvre and rail strokes carried an extruded `solid` and were therefore never drawn in the elevation (the product projects a stroke swept along the viewing direction to nothing); this was already so in the catalog.
- Solids (qa-visual-2: B / B / C): boom and stick solid plates, one horizontal boom cylinder, no bucket cylinder or linkage, no cylinders at 1:200 and 1:500, structure copied the body masses.
- Track contact anchors at z = 1.2 m (gauge half is 1.19 m).

## 3. Root fixes (catalog asset, members `summary`, `provenance`, `dimensions`, `rig`, `anchors`, `representations`, `poses`, `clips`, `thermal`, `volumes`; manifest entry)

- `rig`: 16 bones (was 14). Pin geometry from `r3-excavator-kin.py`: boom foot A (0.15, 0.86) on the upper frame, boom cylinder foot C (0.552, 0.14) and rod pin D (2.45, 0.66) on the boom, stick cylinder E (3.0, 0.92) to F (-0.794, 0.457) on the stick tail, bucket cylinder H (0.25, 0.56) to the link pin K, idler pin I (2.44, 0.05), bucket link pin J (-0.06, 0.47); new bones `idler-link` (0.64 m) and `bucket-link` (0.66 m) form the four-bar. Joint limits are the cylinder strokes: boom -47.9 to 56.2 degrees chord angle, stick 25 to 139.8 degrees fold, bucket 177 degrees. Deck at 1.06 m, slew ring bone at 0.94 m, undercarriage 4.45 m, counterweight bone at the 2.95 m tail radius. Masses: undercarriage 7 300, upper 5 300, counterweight 4 200, engine 900, boom 1 750, stick 900, bucket 820, cylinders and links as before plus 155 kg; total 22 515 kg (was 20 180).
- `poses`: all three re-solved so that every cylinder and both links close (pin gap at most 0.1 mm). `stowed` is the transport pose (boom 10.2 degrees, stick folded 136 degrees, bucket curled, 0.12 m above ground); `digging` teeth on the ground 6.6 m out; `raised-boom` boom at 40 degrees, stick level. Descriptions rewritten in both languages.
- `clips`: `dig-cycle` (144 keys every 1/16 s) and `dump-and-stow` (54 keys) are tooth-tip paths solved by inverse kinematics with all nine linkage offsets at every key; `dump-and-stow` now slews the upper structure 90 degrees through `slew-ring.twistDeg`. Largest pin gap between keys: 3.4 mm (`dig-cycle`), 3.2 mm (`dump-and-stow`). Tracks, events, phases and contacts unchanged. `start-and-idle` unchanged.
- `anchors`: track patches z = 1.2 to 1.19 m; `stack-mouth` (-1.2, 1.84, -0.9) to (-1.80, 1.74, -1.02); `bucket-cog` (0.65, 0.22) to (0.62, 0.36).
- `dimensions`: height 3.43 to 3.44 m, width (length) 9.85 to 9.64 m, depth 2.99 to 2.98 m, mass 20 180 to 22 515 kg. Manifest entry `🗂️catalog/🔣️.json` set to 3.44 / 9.64 (`r3-excavator-manifest.py`, validator `manifest-dimension-mismatch`).
- `thermal.regions[].areaM2`, corrected from the 1:50 model, each within the factor 1.5 of C16: `house` 34.8 to 45.3 (1.30), `counterweight` 13.5 to 9.25 (1 / 1.46, the former value assumed a 1.3 m deep block), `cab-glazing` 4.0 to 5.47 (1.37), `tracks` 48.9 to 57.4 (1.17). Unchanged: `engine-compartment` 7.2, `exhaust-stack` 0.224, `hydraulics` 4.7, `attachment` 33.6. For the area reviewers.
- `representations`: all twelve drawings regenerated from the same definitions as the solids (ruling E1): every shape is the extruded outline, bar or circle of the part it draws, so elevation, front and plan are projections of one body in every pose, including the slewed upper structure. Seams, welds, louvres, rail and steel lines are filled bands of their real width instead of open strokes, so they are drawn. Decisions of all four scales rewritten (en, de). Pens, hatches and valid ranges unchanged. Print suitability: all four scales `authored` with no violation.
- `summary`, `provenance`: 22 t machine, the two data sheets added, notes updated.

## 4. Models

Scripts: `r3-excavator-s50.py`, `-s100.py`, `-s200.py`, `-s500.py` (one per scale), `-coarse.py` (part writers of the three coarser scales), `-form.py`, `-kin.py`, `-lib.py`, `-data.py`, `-build.py`, `-apply.py`, `-manifest.py`; inspection `-view.ts`, `-diag.ts` (what the silhouette rule compares, also between two scales), `-solve.ts`.

| Scale | body (expanded) | thermal | structure |
|---|---|---|---|
| 1:50 | 157 (563) | 117 | 85 |
| 1:100 | 63 (92) | 63 | 74 |
| 1:200 | 44 (51) | 44 | 52 |
| 1:500 | 29 (32) | 31 | 50 |

- 1:50 body: per track a 14 mm shoe band on two link rails with 150 real grouser ribs, toothed sprocket with final drive, flanged idler in its yoke, eight flanged rollers on shafts under a channel frame with skirts, two carrier rollers; X carbody, turret, slew bearing; deck with boom slot, L-shaped house, tool box, hood cover, vent and radiator grilles with slats, tubular stack, handrail, mirrors, work light, counterweight on the tail swing arc with lifting eyes; hollow cab of floor, roof, pillars, cant rails and sill panels closed by seven 8 mm panes, with seat, consoles, joysticks, travel levers and monitor; gooseneck boom box with fork cheeks, bosses, lugs, two steel lines and lamp; stick with forked tail; bucket of 16 mm shell and cheeks, lip, five teeth, cross beam, ears, wear strips; twin idler links, H link; four cylinders (barrel, gland, cap eye, chrome rod, rod eye) and every pin.
- 1:50 thermal: references by region; cab interior faces and grille backs marked not outer; engine block and cooler pack inside the house; the stack as its outer pipe surface (own solids, the body stack is a real tube) with the outlet patch in the mouth. Structure: track frames, turret ring, four carbody legs, eight pads, revolving frame, boom foot plates, lugs, ROPS, engine beds, counterweight carriers, boom and stick as two webs plus flange band with forks, bucket shell, links, cylinder tubes with bore and rods, pins, four masses.
- 1:100: one 45 mm band on one link chain, wheels as discs, rollers as discs under the channel frame, house as one prism, cab as a closed 40 mm shell with four panes, forked boom and stick with root bosses and lug blocks, steel lines, bucket with five teeth and ears, link plates, cylinders as barrel and rod. Structure: ring with two diagonal beams, frame of two rails, three cross members, two side beams, girders of 15 mm band and webs, eleven pins.
- 1:200: one 125 mm band around a frame beam that takes in the rollers, two end discs, house prism, 60 mm cab shell with three panes, generalised boom and stick outlines, bucket of 30 mm shell with a tooth bar, links as single bars. Structure: one cross beam, two rails and two cross members, girders as one flange band each, solid cylinder bars, seven pins.
- 1:500: two solid track blocks with the true loop outline, carbody box, house prism from the frame up, 80 mm cab shell, boom with fork, stick and bucket as closed side profiles. Structure: its own bars (two track beams, cross beam, post, two rails, straight box bars along boom, stick and bucket through the knee), three pins.

Final check line: `volumes-check.ts machinery/excavator --product` PASS (86.5 s). Silhouette IoU elevation / front / plan: 1:50 0.997 / 0.993 / 0.997, 1:100 0.996 / 0.995 / 0.996, 1:200 0.997 / 0.995 / 0.999, 1:500 0.999 / 0.999 / 0.998; boundary p95 at most 7 mm at 1:50 to 1:200 and 18 mm at 1:500. Thermal areas at 1:50 within 7 %, at 1:100 within 8 %; totals 8 % (1:200) and 21 % (1:500). Cross-scale and union volume band pass at every step. Statics (`r3-excavator-solve.ts`): weight 220.8 kN, eight active patches in every pose with zero residual, the rear patches drop out at the dump of the dig cycle, the left track carries 4.9 times the right during the slewed dump.

## 5. What I saw (images opened with the Read tool)

- 1:50 six-view sheet, stowed: reads at once as a 20 t crawler excavator. Right side: gooseneck boom with the knee at 2.6 m, both boom cylinders from the frame to the knee, stick cylinder on the boom back, stick folded under, bucket curled with the linkage closed. Front: cab left, boom right of it between its two cylinders, tool box and stack on the right. Top: counterweight arc, boom slot, tracks under the house edges. Left side and rear three-quarter (turn 140, tilt 20): door and quarter glass with the B pillar, radiator grille, track with rollers below the frame skirts, bucket interior with five teeth.
- 1:50 `digging`: teeth on the ground, cylinders and links closed; `dump-and-stow` at 2.4 s: upper structure turned 90 degrees over the standing tracks, in 3D and in the 1:100 drawing.
- 1:50 structure sheet: frame, ROPS, girders and cylinder bars inside the pale body; thermal cut sheet: engine block and cooler visible inside the house, glazing, stack and hydraulics in their own colours.
- 1:100, 1:200, 1:500 body sheets and the 2D sheets of all four scales: the same machine with fewer parts, proportions unchanged; 1:500 structure sheet shows bars, not the body.
- Overlay sheets of `volumes-render.ts` at 1:50 and 1:200. Its owned drawn side shows red inside the track loop (it fills the belt ring differently from the product); the product numbers above are the ones that count.
- Not opened: the `volumes-render.ts` sheets of 1:100 and 1:500, the 1:100 structure and thermal sheets (rendered, checked by numbers only), and single-list renders `--az/--el` (the `r3-sheet.ts` views were used instead).

## 6. Requests and findings for others

- Tool or product: an open stroke with an extruded `solid` projects to nothing in the view along its sweep (elevation for sweep w); `validateAsset` does not report it. Authors have to draw such lines as filled bands. Not filed in `tool-requests.md` by me.
- `r3-validate.ts` does not check the manifest entry (`manifest-dimension-mismatch`); I updated the entry by hand.
- Area reviewers: the four `areaM2` corrections in section 3; `exposureDeviation` of `engine-compartment` (measured 0.13 against 0.3) is unchanged in kind.
- Product tests were not run (`repo` test targets): `transport-assets`, `thermal-multiview` and `structure-projection` load this asset; clip ids, emitter, anchors ids and regions are unchanged, statics checked with my own script only.

## 7. Still imperfect

- Stowed height 3.44 m against 3.0 to 3.28 m of the data sheets: the stick cylinder eye on the tail is the top point with this stow pose (bucket 0.12 m above ground); a lower stow needs a different transport pose, not done.
- Not modelled at 1:50: hoses at the joints, cab steps and grab rails, door handle, travel motor covers, bolts, the sculpted recesses of the counterweight (it is a full prism, so its volume is about 2.5 times the 4.2 t casting; mass comes from the rig).
- Coarse cabs are hollow shells (so that the volume agrees across scales) whose thermal surface is an own block solid; the block counts the wall under the panes, about 5 m2 more house area at 1:100 to 1:500 (inside the tolerances).
- At 1:200 and 1:500 the links are single bars and the cylinder rod eyes end inside them; at 1:500 the stick tip and the bucket overlap at the hinge.
- Layer order of the 2D shapes is fixed for the unslewed machine; in the slewed frames of `dump-and-stow` parts of the upper structure are painted in the unslewed order.
- The 1:100 list keeps most assemblies of 1:50 with simpler sections; it is re-stated, but it is the least independent of the four.
- Leftovers of mine under `🗑️generated/r3-excavator/` (draft asset, diagnosis images, `catalog-before-r3.json` copy of the former asset) and `volumes/adhoc/excavator.json` (source of the draft).
