# machinery/finned-heat-exchanger — rework report

Chisel: `T/rework-fhx-finned-heat-exchanger-build.py` (states every solid of every scale by hand in world axes: x with the air, y up, z along the tubes, headers at +z; a second run changes nothing). It imports the constructors of `T/rework_11_lib.py`.

## Pass 1

### Reference (two-row duct coil in P40 geometry, 0.4 m finned length, 0.48 m finned height; manufacturer-typical, EN 1216 terms)

| Part | Real | Model before pass 1 (1:50) |
|---|---|---|
| Fins | aluminium 0.1–0.2 mm, pitch 2–4 mm (VDI 3803-1), plate fin as deep as rows x row pitch: 2 x 34.64 = 69.3 mm, as high as tubes per row x tube pitch: 12 x 40 = 480 mm | 132 fins 0.2 mm at 3 mm + 2 pierced end fins, 69.28 x 480 mm: right |
| Tubes | Cu 5/8 in = 15.88 mm (EN 12735-2), 40 mm tube pitch, 34.64 mm row pitch, staggered by half a pitch (equilateral) | 2 x 12 tubes r 7.94 mm at y 30…470 and 50…490 mm, x -17.32 / +17.32 mm: right |
| Return bends | U bends, centre-line radius = half the distance of the joined tubes = 20 mm, protrude 20 + 8 = 28 mm beyond the tube sheet | 12 lofted bends, radius 20 mm, to z = -239.4 mm: right |
| Headers | Cu 28 x 1 mm (EN 1057), vertical, 15–35 mm off the tube sheet, capped, vent on the high point of the return, drain at the low point of the supply | 2 x r 14 mm, 476 mm, domed caps, axis 31.5 mm off the web; vent and drain plug r 5 mm: right |
| Connections | stubs with R 1 sleeve (33.25 mm thread, about 34 mm sleeve), 50–80 mm long, supply low / return high for counterflow and venting | 2 stubs r 14 mm with r 17 mm sleeve, 64 mm, inlet y 70, outlet y 450 mm: right |
| Tube sheets | galvanised sheet 1.5–2 mm, pierced for every tube (hole 0.3–0.5 mm over tube), 20–30 mm duct flanges turned outward with 9 mm fixing holes at 100–150 mm | one C extrude 100 x 26.5 mm, **no tube holes (24 tubes run through solid sheet), no fixing holes** |
| Top / bottom | channel 100 mm wide, 20 mm legs, 1.5 mm | C extrudes 100 x 20 x 1.5 mm, legs down / up: right |
| Overall | H 520, depth in air 100, length over bends and stubs 535 mm; 7.9 kg dry | 0.52 x 0.1 x 0.535 m: right |

### Defects found

- 1:50 body: tube sheets were unpierced C extrudes that the 24 tubes interpenetrated; flanges had no fixing holes. Everything else measured right against the table (rig: three fixed bones frame → fins → tubes stacked on the vertical axis with origins at 0, 20 and 30 mm, which is the correct "skeleton" of a coil without moving parts; no pose moves a bone, the three poses and two clips change only the duty).
- 1:100 was the 1:50 list minus the two plugs with shortened hairpins (132 fins of 0.2 mm at a scale whose feature is 15 mm).
- 1:200 and 1:500 were the 1:50 list again (identical to each other, 0.2 mm fins and 16 mm tubes at 30 and 75 mm feature size), and did not agree with the 1:200 drawing, which is drawn as masses (coil block, bars, end plates, header block, stub prisms).
- Structure at 1:200 / 1:500: no joints, mass ball too small for 7.9 kg (27 mm instead of the 32 mm of the 1:50 rule of 16 mm per cube root of a kilogram).
- `decisions` were the same two paragraphs at all four scales and described 1:50 only.

### Rebuilt

- **1:50 body** (15 solids): geometry kept, because it measures right against the table; tube rows and return bends are restated by hand in the chisel (bend: 20 mm radius in the plane through both tube ends, 11 sections). A first attempt split each tube sheet into a pierced 1.5 mm web (24 holes of 16.6 mm) and two flanges with four 9 mm fixing holes; it rendered correctly, but a separate 1.5 mm sheet is refused by `volumes-feature` and passes only when marked `slender`, which the schema reserves for cords and rods and which removes the part from the silhouette tools. I took it back: the tube sheets stay folded C extrudes without holes (see Remaining and Fundamental issues). Thermal: the same 15 solids in the four regions. Structure (22): unchanged load path (channels, 7.5 mm tube-sheet webs, tube rows, 4 x 12 expanded collar joints, 2 x 12 header joints, corner joints, feet, four masses, five sample fins, headers).
- **1:100** (12 / 12 / 19): fin pack as one block 69.28 x 480 x 399.2 mm, tube rows through it, bends with 7 sections, plain headers, stubs with sleeve step, solid 20 mm bars, C tube sheets whose flanges hold the bends; structure with one joint strip per tube sheet, corner joints, feet, four masses and four sample fin plates of 15 mm instead of the opaque fin block, so that tubes and masses stay visible.
- **1:200** (8 / 9 / 11): the eight masses of the 1:200 drawing with its ids and its numbers (coil-block, casing-bottom/top 30 mm, end-plate-left 30 mm, end-plate-right 39.44 mm over the bends, header-block 70 x 30 mm, two stub prisms); hidden tube core for thermal and structure; two sample fin plates of 30 mm, joints, feet, one 7.9 kg ball of 32 mm radius.
- **1:500** (3 / 4 / 7): one casing frame with the air opening, the coil block, one connection block; tube core through both end plates (differs from 1:200 by hand: z -230…220 mm); two fin plates of 75 mm, joints, base strip, ball.
- `decisions` rewritten per scale (en, de).

### 2D drawings

No shape redrawn: the silhouettes of 1:50 and 1:100 did not change, and the 1:200 model was built to the numbers of the 1:200 drawing (same ids, same rectangles). `r3-2d` read for 1:50, 1:100, 1:200. 1:500 has no print drawing (`unavailable`, below printable feature: the coil is 1 mm high on paper), which is right. `r3-project2d --check` reports iou 0.999 / 0.88 / 0.96 (1:50), 0.997 / 0.985 / 0.964 (1:100), 1.000 / 0.946 / 0.913 (1:200) for its own dump; the low front value at 1:50 comes from the 132 `slender` fins that the tool leaves out, not from the drawing.

### Validation

`r3-validate.ts machinery/finned-heat-exchanger --volumes` and `--catalog-only --volumes`: `VALID … schema 0, validateAsset 0, checkVolumes 0` (after `volumes-insert.py`).

Rendered and read after the last change: 1:50 body (hot, idle), thermal (cut hot, cold), clips reverse t=10 (thermal) and t=30 (body), warm-up t=2 (body) and t=30 (thermal); 1:100 thermal cut and structure; 1:500 body, thermal cut and structure; 2D 1:100, 1:200. Rendered after the last change but read only in the identical state before it: 1:50 body cold, 1:50 structure, 1:100 body, 1:200 body, thermal cut and structure.

### Remaining, stated plainly

- 1:50 body, `tube-sheet-left` / `tube-sheet-right`: no tube holes (the 24 tubes interpenetrate the 1.5 mm web; invisible, the tubes fill the holes) and no 9 mm fixing holes in the four flanges (would be visible in the three-quarter view). Needs the rule change below.
- 1:50 body: no fin collars, no brazed joints at the header tees, no thread on the R 1 sleeves (all under 7.5 mm).
- 1:100 body, `fin-pack`: a plain block; the fin face has no fin texture, so the front view reads as a panel in a frame.
- 1:200 / 1:500: bars 30 mm instead of the real 20 mm (as the 1:200 drawing draws them for printability).
- Rig unchanged (three fixed bones); 2D rig has no lateral axis, so the tube mass sits on the axis in the rig while the structure ball is 67 mm towards the headers at 1:50 and 1:100.

### Fundamental issues

- `volumes-feature` refuses any separate solid thinner than the scale feature, and the only exemption is `slender`, which the schema defines for cords, wires and rods (length at least 20 times the cross extent) and which takes the solid out of the cross-scale comparison and out of `r3-project2d`. Real sheet-metal parts (a pierced tube sheet, a flange with holes, a fin) therefore cannot be authored at true thickness with holes: either as an unpierced folded profile whose bounding box is thick enough, or as a mislabelled `slender`. `volumes-slender` accepted my 100 x 520 x 1.5 mm web and 25 x 520 x 1.5 mm flange as slender although neither is 20 times longer than its largest cross extent, so that rule does not check what the schema says. A `sheet` exemption (thin in exactly one direction, counted in silhouettes) is missing.
- `r3-sheet.ts` tints every `slender` solid amber regardless of material and tone; galvanised sheet then looks like the aluminium fins.
- Default-pose sheets (`…s50.body.png`, `…s50.thermal.png`) disappeared from `🗑️generated/r3-sheet/machinery/` while pose and clip sheets were rendered in the same loop; either the tool or a concurrent cleanup removes them. I copied sheets to `🗑️generated/rework-finned-heat-exchanger/` to read them.
