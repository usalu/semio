# Round 3 — objects/chalkboard (2026-11-20)

Fresh start: no earlier round-3 scripts, report or generated folder existed for this asset. At the start the catalog `volumes` equalled `T/volumes/objects/chalkboard.json` (17 / 8 / 5 / 3 body solids), `volumes-check --product` passed and the bulk passes of 03:40 had not changed the scales (all four authored, all four printable). The old source is kept as `🗑️generated/r3-chalkboard/volumes-before.json`.

Scripts in T, all re-runnable:
- `r3-chalkboard-kit.py`: every dimension of the board, the profile writers, solid and shape writers (world coordinates in, bone frames out), the catalog layout writer.
- `r3-chalkboard-volumes.py`: every solid of every scale and list, the masses from the modelled parts.
- `r3-chalkboard-roots.py`: root fixes and the 2D of all scales; `--apply` writes the catalog members.
- `r3-chalkboard-ship.sh`: the ship chain (solids, candidate, validation of the candidate, roots, splice, product check, validation, print suitability).
- `r3-chalkboard-close.ts`: close-up and section renderer (an 80 mm deep object on a 4 m sheet cannot be judged on the six-view sheet alone).

## 1. Reference (the real subject)

A wall-mounted horizontal sliding board unit: a fixed back board of two enamelled panels and one framed leaf that slides in front of it, with a tray below.

| item | real value | source |
|---|---|---|
| unit class | fixed back panel plus sliding panels in anodised aluminium tube or edge frames, panels on nylon rollers in an aluminium track, full-length tray with enclosed end caps | manufacturers' product data (GMi Ghent Duo-Track, MooreCo Best-Rite, Claridge, AARCO), web search 2026-11-20 |
| writing panel | porcelain-enamel (ceramic) steel laminated on 12 to 16 mm board | same; the asset's own layers (0.6 mm steel on 16 mm MDF) |
| overall depth | about 90 mm (3 1/2 in.) for a unit with two planes and tray on the Ghent drawing; 150 mm for four tracks (AARCO) | same; the Ghent figure is read from a drawing whose text does not explain it |
| board class | 4.0 m wide, 1.2 m high plus tray | EN 14434:2010 (asset reference), the asset's notes |
| safety | no sharp edges or corners, rounded corners and tray ends | EN 14434:2010 |
| surface mass of a panel | 0.6 mm steel 4.71 kg/m² + 16 mm MDF 12.0 kg/m² = 16.7 kg/m² | densities of the asset's materials (7850 and 750 kg/m³) |
| plain framed boards for comparison | 8.7 to 10.9 kg/m² (thin sandwich boards without MDF core) | product sheets of 1.2 x 2.0 and 1.2 x 2.4 m enamel boards found in the same search |
| chalk | round sticks about 10 mm x 80 mm | my knowledge of school chalk |
| board ruler, sponge | 1 m beech ruler with a handle, sponge about 140 x 45 x 36 mm | my knowledge; the old drawing's sizes |
| polyamide, beech | 1150 kg/m³, 0.25 W/mK, 1600 J/kgK; timber 700 class, 0.18 W/mK, 1600 J/kgK | ISO 10456:2007 (asset reference) |

Two manufacturer PDFs could not be opened (404 and 403), so the depth and construction figures come from search summaries, not from the sheets themselves.

## 2. Audit (before)

| part | found | real / required |
|---|---|---|
| rig | frame and tray bones start at mid width and run 4 m to the right, 2 m beyond the board; the leaf bone starts at the leaf's right edge and points away from the leaf | bones lie on their bodies |
| supports | anchors without lateral position, `wall-rest` at y 0.1 on the panel, support solids sat on the visible front face (qa-visual) | at the back: hanging rail and buffers |
| masses | 72 kg (45 / 22 / 5), notes say 9 kg/m² | the declared layers weigh 16.7 kg/m²: about 130 kg |
| how the leaf runs | leaf as tall as the board, two 44 mm discs and a 48 mm strip drawn behind it; no track a roller could run in, no bottom guide | a track, trolleys, a guide |
| frame | one extruded plate with two holes, 45 mm deep; panels 17.2 mm boxes; no corners, no pockets | edge profile with chamber and panel pocket, corner pieces, mullion profile |
| leaf grips | two 120 x 20 mm plates of 7.5 mm at the bottom of the leaf face | pull handles on the stiles |
| tray | U trough with 22 mm floor and 8 mm walls, chalk 14 mm thick, ruler made of the material chalk | thin-walled extrusion with lip, trough and bearing box, end caps |
| wall fixing | absent in the body | hanging cleats and buffers |
| structure | body panels and tray relabelled, blue slab (qa-visual grade B) | bars, joints, supports at the back, panels as masses |
| thermal 1:100 to 1:500 | grey panels without layer (qa-visual grade B) | enamel faces |
| 1:500 | tray drawn as a line at y 0 and missing in 3D in round 1, then a 3D bar the drawing did not show | one tray, drawn and modelled |
| 1:200 print | not checked by any validator | see root fixes: my first redraw was unprintable at 1:200 and no validator said so |

## 3. Root fixes (catalog, old → new)

- Rig: `frame` x 0 → -2 (frame, tray and leaf bones now lie along the board from its left end); `leaf` y 0.06 → 0.068 (the leaf's lower edge, 8 mm above the tray). Every bone-local coordinate was restated; poses and clips are unchanged and still move the leaf 0 to 2 m.
- Masses from the modelled parts: frame 45 → 86.1 kg, leaf 22 → 39.3 kg, tray 5 → 5.0 kg; `dimensions.massKg` 72 → 130.4. Centres of mass restated in the new bone frames (frame 2.0 / 0.689, leaf 1.0 / 0.577, tray 1.995 / 0.019).
- Anchors: both x 0 → 2 (same world place); `wall-hook` gets z 0.036; `wall-rest` y 0.1 → 0.0775 (the bottom bar, where the buffers sit) and z 0.036. Titles restated.
- Materials added: `polyamide`, `beech-wood` (ISO 10456). The ruler is no longer chalk.
- Reference added: `sliding-board-products`. Summary, provenance notes, the frame bone title, the frame region title and the structure assumption rewritten in en and de.
- `dimensions` height, width and depth unchanged (1.26 / 4.0 / 0.08).
- Thermal `areaM2`: none changed. Measured at 1:50: board-face 4.41 (declared 4.5), frame 1.75 (1.71), leaf 2.15 (2.4), tray 1.20 (1.4).
- 2D, all four scales, redrawn as the true views of the new body (ruling E1):
  - 1:50 elevation: frame with corner radius, two faces, four corner pieces per frame, header track (`top-rail`) with soft-edge line and cap lines, tray with lip line and cap lines, sponge and ruler handle above the lip, leaf frame and face 8 mm clear of tray and track, two hanger nuts in the gap, two bow handles on the stiles. Removed: `frame-profile`, `leaf-profile`, `leaf-top-rail`, chalk in elevation (hidden behind the lip).
  - 1:50 front: tray end cap, sponge, frame stile, header cap, hanging cleat and buffer in the wall gap, leaf stile with handle and hanger. 1:50 plan: header track, hanging cleat, tray strip with chalk, ruler and sponge, the two handles; the leaf runs hidden under the track.
  - 1:100: frame, faces, header, tray, sponge, leaf (running gaps closed). 1:200: board, header, tray, leaf; the leaf end is 24 mm so that it prints. 1:500: board, leaf, tray as a filled band in every view.
  - Decisions of all four scales rewritten.

## 4. What I modelled

Design in one sentence: wall at z +0.040, an 8 mm gap with two Z cleats and five buffers, the fixed board 20 mm deep, 4 mm air, the leaf 20 mm deep, and a tray 80 mm deep whose trough lies under the leaf.

| scale | body | thermal | structure |
|---|---|---|---|
| 1:50 | 45 solids: 4 edge-profile bars (hollow, with panel pocket; the bottom bar with the guide slot), 4 corner pieces, H mullion, 2 panels, header track with chamber, treads, slot and back box, end caps, 2 end stops, wall cleat, board cleat, buffers; leaf with the same bars, corners and panel, 2 trolleys with 8 wheels, hanger bolts, flange nuts, 2 glides, 2 bow handles; tray extrusion, end caps, 3 chalk sticks, sponge, ruler and handle | 43: core and enamel sheet per panel; closed envelopes of bars, mullion, header and tray; the small parts as references | 30: bars, mullion, header and board cleat as members, corner pieces and mullion ends as joints, wall cleat as pin, buffers as roller, 3 panel masses, hanger and glide joints, tray with 5 screw joints |
| 1:100 | 8: two frame plates with openings, 3 panels, header hood reaching the wall, tray section, sponge | 8: enamel sheets, frame plate, header envelope, leaf frame, tray, sponge | 18: plain bars of both frames, mullion, header, supports at the back, masses, hanger joints, tray fixings |
| 1:200 | 4: board slab, header bar, leaf slab, stepped tray | 6: enamel sheets, frame plate, header, tray | 13: four bars per frame, supports, masses, tray |
| 1:500 | 3: board, leaf, tray | 4: enamel sheets, frame band, tray | 5: one ring per frame, two support blocks behind the board, tray |

Final check: `volumes-check.ts objects/chalkboard --product` PASS; `r3-validate.ts objects/chalkboard --volumes` VALID (schema 0, validateAsset 0, checkVolumes 0); print suitability authored at all four scales. Silhouette IoU elevation / front / plan: 1:50 1.000 / 0.977 / 0.953, 1:100 1.000 / 0.985 / 1.000, 1:200 and 1:500 1.000 in all three.

I did not run the product test suites (`repo run … :test`, `export-models`). The mass change moves every structural result of this asset; the catalog-wide tests named in `area-decisions.md` for this asset compare inequalities, and I found no fixture that pins a chalkboard number.

## 5. What I saw in the final renders

- 1:50 body, six-view sheet and close-ups: front (end) view shows tray cap, stile, leaf stile with the bow handle, header cap. Elevation shows two framed panels under the header, dark corner pieces, the leaf in front of the left half with its handles, sponge and ruler handle in the tray. Back shows both cleats (yellow, slender) and five buffers. Sections at the top and bottom show the header chamber with wheels on the treads, bolt and nut in the 8 mm gap, the hooked cleats, the panel in its pocket, the glide in the slot, the trough with the sponge. Poses `leaf-right` and clip `slide-leaf` at 3 s move leaf, trolleys and handles together.
- 1:50 thermal: enamel faces in front, cores behind, frame green, tray yellow. 1:50 structure: bars blue with open fields, three magenta masses, green cleat and buffers at the back.
- 1:100: the same board without small parts; hood, plates and tray section read in the sections. Thermal faces are coloured, structure shows bars, mullion and masses.
- 1:200 and 1:500: slabs and bars, as the scale allows; thermal coloured, structure as rings with the supports behind the board.
- 2D at all four scales and `leaf-middle` at 1:50: drawn, legible, the leaf moves.

## 6. Still imperfect

- The depth stays 0.08 m, so the tray trough lies under the leaf instead of projecting in front of it; only a 21 mm strip is open from above. A deeper tray needs a cited depth that I did not find.
- Screw heads (3 mm) are below the 1:50 feature and are not modelled; the cleats have no visible fixings.
- The panels have no backing sheet, because the asset declares two layers only; their backs show the panel material.
- The two hanging cleats are left out of the thermal list (they lie in the wall gap); with them the frame area would read about 2.3 m² against 1.71.
- Tray area is 14 % under the declared 1.4 m² at 1:50 (limit 15 %), leaf 10 % under 2.4; both left as declared.
- `exposureDeviation` of `board-face` is 0.50: the pose declares half of the back panels covered, the model has the leaf 4 mm in front and buries nothing. For the area reviewers.
- The `wall-rest` roller keeps its in-plane normal (`normalDeg` 0); the real buffer pushes across the board. I did not change the structural model.
- 1:200 and 1:500 bodies are plain slabs; I judge them right for the scale, not rich.

State: DONE
