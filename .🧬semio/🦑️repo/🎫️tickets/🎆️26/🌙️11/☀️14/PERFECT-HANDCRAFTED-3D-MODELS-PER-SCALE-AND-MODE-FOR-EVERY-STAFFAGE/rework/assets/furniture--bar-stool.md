# `furniture/bar-stool` — running author report

Earlier state: batch report `rework/14.md` section 3 judged the asset "no defect found, not changed". Source `volumes/furniture/bar-stool.json` and catalog `volumes` were identical when this pass began.

## Pass 1 (second round), 2026-11-22

### Reference table (written before any edit; mm)

Subject: swivel bar stool, counter/bar class for a 1050 – 1100 mm bar, chromed four-leg tube frame with footrest ring, round upholstered seat, low back. Sources from memory: EN 1022 (seating stability: overturning line through two adjacent feet), EN 16139 / EN 12520 (strength), EN 1728 (seat proof load), common manufacturer dimensions of four-leg swivel stools (seat 29 – 30 in).

| Quantity | Reference | Model before | Verdict |
|---|---|---|---|
| Seat height (top of cushion) | 740 – 780 | 752 | right |
| Seat diameter | 340 – 380 | 360 | right |
| Cushion thickness, board | 45 – 60 on 12 ply | 54 on 12 | right |
| Swivel plate | 150 – 180 across, 12 – 16 high | 170, 12 | right (round instead of square, hidden) |
| Leg tube | 22 – 25 x 1.5 | 25 | right |
| Leg arrangement | four legs on the diagonals of the seat (front pair, rear pair) | on the axes: one leg straight ahead between the sitter's feet, one under the backrest | wrong |
| Foot square (centres), side | 360 – 420, never less than the seat diameter | feet on a 395 circle on the axes = square of 279 | wrong |
| Overturning distance (stool axis to the line of two adjacent feet) | at least the seat radius, 180 – 210 | 140, i.e. the seat overhangs the tipping line by 40 | wrong, EN 1022 would fail |
| Leg splay (true, in the diagonal plane) | 9 – 13 degrees; 6.5 – 9 seen in elevation | 4.7 | wrong, reads as a bundle of sticks |
| Footrest height below seat top | 430 – 480 | 482 (ring at 270) | at the upper limit |
| Footrest ring, tube / diameter | 22 – 25 tube, outside the legs, 450 – 510 over all on a four-leg stool | 25 tube, 425 | follows the legs |
| Glides | 28 – 32 rubber, 20 – 25 high | 30 x 22 | right |
| Back: pad, top above seat | 280 – 320 wide, 80 – 100 high, top 100 – 150 above the seat | 300 x 90 x 26, top 118 | right |
| Back bar | flat 40 x 8 bent, screwed under the board | so | right |
| Mass | 7 – 9 kg | 7.8 | right |

### Defects found (look step, 1:50 body / thermal / structure, then the coarse scales)

- Frame, all four scales, all three lists, all 2D views, symbol, anchors: stance too narrow and legs on the axes (table above). In front and side view a third leg stood in the middle of the stool; the tipping line passed 40 mm inside the seat edge.
- Rig: seat swivel had no twist limits declared although every turned pose twists it.
- Seat, swivel, pan, cushion, piping, back bar, back pad: measured right, kept.
- Loaded poses (`occupied`, `seat-proof-load`, both clips with an occupant), 1:50 and 1:100: the whole cushion slid down on the pad bone, 18 / 30 mm through the swivel plate and 6 / 18 mm into the pan.
- Structure, all scales: the single frame mass marker sat on the stool axis, which is air once no leg stands there.
- Thermal, 1:100 to 1:500: cushion and back solids carried no layer, so the sheet showed them grey instead of as the cover.

### Rebuilt

Edit scripts (hand-chosen numbers, idempotent, kept): `rework-bar-stool-frame.py`, `rework-bar-stool-cushion.py`.

- Frame, all four scales, body / thermal / structure: four 25 mm legs on the diagonals from the pan radius 144 (146 at 1:200, 1:500) to a foot radius of 262, i.e. a foot square of 370 on centres, 400 over the glides; true splay 10.2 degrees, 7.2 in each elevation; overturning distance 185 (seat radius 180). Footrest ring outside the legs at 290 (462 under the seat top), centre line radius 236.5, 498 over all. Glides, weld joints (leg tops, ring nodes), foot members and the four support solids moved with the legs.
- Dimensions 0.498 x 0.498 x 0.87 m. Anchors renamed and moved: `foot-fl`, `foot-fr`, `foot-rl`, `foot-rr` (poses, clips, `structure.supports`, support solids follow); `ring-front` on top of the new ring. Thermal `footrest-ring.areaM2` 0.099 to 0.117 (longer tube).
- Rig: seat joint is `continuousTwist` (a ball-race swivel has no stop). Bones, lengths and the prismatic pad joint were right and stay.
- Cushion, 1:50 and 1:100: `cushion-skirt` on the seat bone (cover wrapped round the board, 698 to 730) and the crown `cushion` on the pad bone (728 to 752) that sinks into the skirt; piping in the groove on the seat bone. Thermal: skirt cover and skirt foam on the seat bone, crown cover and crown foam on the pad bone, board inside. Structure: skirt as seat member, foam joint lifted into the seam. At 1:200 the cushion stays one disc (the 18 to 30 mm of sinking is under the 30 mm feature), at 1:500 the seat block does not move.
- Structure masses: the frame mass is marked on the steel (pan, middle of each leg, four points of the ring; at 1:500 the four ring nodes).
- Thermal 1:100, 1:200, 1:500: cushion, skirt, seat block and back carry the `leatherette` layer.
- 2D, all four scales: elevation and front show the two leg pairs (third leg and its ferrule removed), ring at 290 and 473 wide on centres; plan shows four diagonal legs under a true ring annulus, ferrules outside the ring at 1:50, legs as strokes at 1:500 (the ring disc of the old 1:200 / 1:500 plan failed the silhouette once the ring stood 44 mm clear of the seat). 1:50 and 1:100: cushion drawn as skirt (seat bone) and crown (pad bone), the seam replaces the piping stroke in the elevations. Symbol redrawn to the new stance.
- Texts (en, de): provenance notes, symbol description and note, structure assumption, representation decisions of all four scales, volume decisions of all four scales.

### Validation

`VALID furniture/bar-stool (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0` (`r3-validate.ts furniture/bar-stool --catalog-only --volumes`, after the last publish). Source and catalog `volumes` are identical.

### Looked at after the last change (sheets read)

1:50 body three-quarter, `turned-90`, `seat-proof-load` close-up, `swivel-turn` 0.75 s, `sit-and-turn` 1.75 s, `stand-up` 0.5 s; thermal cut 1:50, 1:100, 1:200, 1:500; structure 1:100, 1:200, 1:500; 2D 1:50 (default and occupied), 1:200, 1:500. Rendered but not read after the last change: 1:50 `occupied` sheet, `swivel-turn` 4.5 s, `sit-and-turn` 2.5 s, `stand-up` 2 s, 1:50 structure after the cushion split, 1:100 body and 1:100 2D after the cushion split.

### Remains imperfect

- 2D plan, all scales: the back pad is the extruded twin of its elevation profile, a straight bar 300 wide, while the 3D pad is bent on a 300 radius (sagitta about 37 mm). The validator accepts it; a curved plan shape needs its own hand-drawn poly instead of the `solid` twin.
- 1:50 body: the swivel is a round 170 plate; real ball-race swivels are two square 160 plates. Hidden between pan and cushion in every view but from below.
- 1:200: the loaded cushion disc overlaps the upper 18 to 30 mm of the mount band (stated in the decisions).
- Catalog file: the root members were rewritten in the compact array layout of `volumes-insert.py` (the file was a mix of layouts before); content only changed where listed above.

### Fundamental issues (outside this asset)

- `r3-2d.ts` at 1:500 dimensions the stool 0.86 m although body, drawing and `dimensions.heightM` are 0.870 (seen before and after this pass; looks like truncation of 0.8699… in the product's dimension text).
- `volumes-structure-coverage` requires a `mass` marker inside the body silhouette; the centre of mass of an open frame is in the air, so a single true marker cannot be authored and the mass has to be split over the parts.
