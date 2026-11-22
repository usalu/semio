# Round 3 — components/tilt-window

State: finished and spliced. `r3-validate.ts components/tilt-window --catalog-only --volumes`: `VALID ... schema 0, validateAsset 0, checkVolumes 0`. `volumes-check.ts components/tilt-window --product`: `PASS (2.4 s)` at 1:50, 1:100 and 1:200 (1:500 stays unavailable, as before).

No earlier round-3 work existed for this asset: no `r3-tilt-window-*` scripts, no partial report, and the source matched the catalog. I started from the catalog state.

Scripts (all in the ticket folder):

- `r3-tilt-window-kit.py`: every real dimension, the two bone frames (`sill`, `sash`), the section polygons (rails, stiles, beads, sill, weather bar, quadrant arc), the mass budget, the section properties and the kinematic checks.
- `r3-tilt-window-volumes.py`: every solid of every scale and list, stated part by part. It writes `volumes/components/tilt-window.json`.
- `r3-tilt-window-roots.py`: the root fixes and the 2D drawings of all three scales and views. Without a flag it writes the candidate to `🗑️generated/r3-tilt-window/🔣️.json`. With `--apply` it re-reads the catalog file, replaces only the listed members (with a byte-layout check and a check that nothing else changed) and sets the manifest entry's `heightM`.
- `r3-tilt-window-suit.ts`: prints the print suitability of the three drawings of a candidate. It showed that 1:200 was refused because of a 0.1 mm sash strip in the plan.

## 1. Reference

| item | real value | source |
|---|---|---|
| type | Bottom-hung tilt sash (Kippflügel) in a timber lining frame (Futterstock) that lines the full 300 mm reveal. Exterior stop strips; the room is at -x. | asset summary and symbols ("opening in a 0.30 m reveal"); construction practice |
| lining boards | 50 mm softwood, 300 mm deep. Sill, two jambs, head. | asset (sill 50 mm, jambs 50 mm); a head board is needed by any four-sided frame |
| sash profile | 90 mm deep. Stiles and top rail about 70 mm, bottom rail about 80 mm. | DIN 68121-1 profile family (IV 68/78/92); own knowledge |
| rebate clearance (Falzluft) | 12 mm all round between sash and frame | DIN 68121 / Euro-groove hardware convention |
| hardware groove | 16 x 12 mm Euro groove (modelled 12 x 10 mm) on the sash's outer side faces | DIN 68121, hardware catalogues |
| glass | 4-16-4 low-e insulating unit, 24 mm. Let about 14 mm into an 18 mm rebate. Glazing bead on the room side. Edge seal about 12 mm deep. | asset layers; EN ISO 10077-1; glazing practice |
| glass mass | 4 mm float glass = 10 kg/m2 (25 kN/m3). The cavity is gas and has no mass. | EN 572-1, EN 1991-1-1 (asset references) |
| timber density | C24, 420 kg/m3 | EN 338 (asset material) |
| tilt opening | 130 mm at the head for a tilt-and-turn stay (Winkhaus SH.SE data sheet); 70 to 80 mm with a tilt limiter | WebSearch: [Baunetz Wissen, Scheren](https://www.baunetzwissen.de/beschlaege/fachwissen/fensterbeschlaege/scheren-150290), [Baunetz Wissen, Öffnungsbegrenzer](https://www.baunetzwissen.de/drucken/oeffnungsbegrenzer-2451109) |
| tilt hinges | Two hinges (Kippbänder) at the room-side bottom corner. The axis sits just inside the room and above the frame rebate, so the sash rises off its frame as it tilts. | hardware practice |
| weather bar | Wetterschenkel on the bottom rail, about 35 to 40 mm proud, with a sloped top and a drip nose. The exterior sill is weathered and has a drip groove. | joinery practice |

## 2. Audit findings (before this round)

Rig and kinematics

- The sash joint sat at the sash centre line on the sill top (`x 0.15, y 0`). Tilting inward about that point drives the room-side bottom corner into the sill board: 11.6 mm at 15° and 22.5 mm at 30°. This was a root defect.
- The poses tilted to 15° and 30°, which puts the head 0.31 m and 0.60 m into the room. Tilt hardware stops at 70 to 130 mm (about 3 to 6°).
- Sash mass was 72.8 kg because the whole 0.9 x 1.2 m area was counted as 24 mm of solid glass (64.8 kg glass). The real sash weighs 25.8 kg. The section was a nominal 90 x 90 mm square.
- The stay anchor sat at the sash tip, although nothing physical was there. No stay was drawn or modelled at any scale.

Dimensions and 2D

- The frame had no head. A lining or frame without a head is not a buildable object, and the sash closed against nothing.
- The sash filled the opening between the jambs with no clearance and had no stops. In the front view the sash read as part of the frame (QA: "sash not distinguishable").
- The glass sat in a 15 mm groove at the sash centre with the unit butted against the rail faces. There were no beads, no rebate clearance and no edge-seal geometry.
- There was no weather bar. The sill drip was a line inside the wall.
- The front view (exterior) drew the hinges, which are on the room side and hidden from outside.

Solids

- 1:50: the sash members were plain boxes and the glass was one 24 mm block. The "hinge barrel" was a 40 mm cylinder at the old wrong axis. There was no handle, no stay and no stop.
- Structure: the lining was referenced as solid members. The hinge joint was buried in the sill, and the support was a 20 mm rod spanning the whole width through air.
- 1:100 was nearly a copy of 1:50. The thermal glass at 1:200 was one slab with a second region marked fully cut.

## 3. Root fixes (catalog file, outside `volumes`)

| member | old | new |
|---|---|---|
| `dimensions.heightM` | 1.25 | 1.324 (sill 50 + clearance 12 + sash 1200 + clearance 12 + head 50). The manifest entry was updated with it. |
| `dimensions.massKg` | 72.8 | 54.4 (sash 25.8 + lining, stops and steel 28.6) |
| `rig.bones[sash]` x, y | 0.15, 0 | 0.097, 0.008 (the tilt axis 8 mm into the room and above the sill, at the room-side bottom corner) |
| `rig.bones[sash].joint.maxDeg` | 30 | 6 (the stay stop) |
| `rig.bones[sash].mass` | 72.8 kg, comX 0.6, comY 0, literature | 25.78 kg, comX 0.581, comY -0.0485, authored-estimate. Parts: panes 14.3, timber 9.7, edge seal 1.0, steel 0.8. |
| `rig.bones[sash].section` | 0.0081 m2, 5.4675e-6 m4, e 0.045 | Two stiles with rebate and groove: 0.01056 m2, 7.284e-6 m4, e 0.0484 |
| bone titles | "Frame sill", "Sash, revolute at the bottom hinge" | "Lining frame (rigid root)", "Sash, revolute at the tilt-hinge axis" |
| `anchors.stay` | (1.2, 0) "Head stay" | (1.15, -0.053): the quadrant-stay pins, horizontal roller |
| `anchors.hinge` | title only | "Tilt hinges at the room-side bottom corner, pin" |
| pose `tilted-open` | 15° | 3° (63 mm at the head) |
| pose `fully-tilted` | 30°, wind wx -623.5 / wy -360 N/m | 6° (126 mm, the stay stop), wind wx -716.1 / wy -75.3 N/m (720 N/m normal to the sash) |
| clip `tilt-open-close` | text 15/30° | text 3° / stop at 6°; event "Stay stop reached" |
| `thermal.regions` areaM2 | pane-glass 0.6, pane-edge 0.24 | 0.47, 0.2 (visible glass of the real profiles; factor 1.28 and 1.2; listed in `area-review.md`) |
| thermal titles and assumptions | "sash frame", "24 mm in geometry and mass" | "sash and lining frame", "real panes, cavity and edge seal; mass counts the glass" |
| `structure.assumptions` | head roller at the tip | Pin on the tilt axis; quadrant stays concentric with it; the pins idealised as one horizontal roller at 1.15 m |
| `provenance` | — | Added references `din-68121-1` and `tilt-hardware`, and new notes (en/de) |
| `summary` | "0.9 x 1.2 m timber sash" | "0.776 x 1.2 m sash in a 0.9 x 1.324 m lining frame, tilting to 6°" |
| `symbols` (both) | outline 1.25, sash 0.8 x 1.2 | Outline 1.324, sash 0.776 x 1.2 from y 0.062, glass line at x -0.007, sash faces at ±0.045, notes updated |
| `representations.*.geometry` | see 2D below | Redrawn at all three scales |
| `representations.500.reason` | 0.30 x 1.25 m | 0.30 x 1.32 m |

The 2D drawings were redrawn by hand under ruling E1, from the same kit numbers as the solids.

- **1:50 elevation** (vertical section at z = 0):
  - Cut and cross-hatched: the sill with its weathering and drip groove, the head, the sill and head stops, both rails with their room-side rebates, both chamfered beads and the weather bar.
  - Edge seals as small dark blocks. Each pane is one line, split into edge zone and centre.
  - Seen beyond the cut: the far jamb and its stop strip, the far stile, the hinge leaves, the 16 mm hinge pin, the far quadrant arc with its slot and pin, and the grip (post and cut bar).
- **1:50 front** (exterior): lining boards, the four stop strips over the sash edges, stiles, rails, weather bar, pane, edge-seal line and reflections. The hinges were removed because they are hidden from outside.
- **1:50 plan**: the head board over everything (z above the sash), the jambs, and the sash, weather bar and grip as extruded profiles. They show in the room when the sash tilts.
- **1:100** is simplified: rails without rebates, a weather bar block, the glass band split into centre and edge, the stops, and the hinge pin.
- **1:200** is simplified further. The plan no longer draws the sash: at the stop it shows beyond the head only as a 21 mm strip (0.1 mm on paper), which the print profile refuses.
- Decisions were rewritten in English and German.

## 4. Volumes per scale

### 1:50: body 28 solids, thermal 23, structure 13

- **Body, lining (`sill` bone):**
  - Sill board: an extruded section with the 10 mm weathered fall beyond the stop and an 8 x 10 mm drip groove.
  - Jambs: boxes running to the floor line.
  - Head board.
  - Four stop strips, 25 x 30 mm.
  - Two frame hinge leaves, 60 x 60 x 8 mm.
  - Two quadrant stays: steel arcs 24 x 8 mm, concentric with the tilt axis (R 1.151 m), with a 10 mm slot. They are screwed flat on the jamb faces in the 12 mm rebate slot.
- **Body, sash (`sash` bone):**
  - Stiles: extruded from the real section, with the 18 mm room-side rebate and the hardware groove.
  - Rails: rebated sections running under the stile rebates (mortise joint).
  - Four beads, 26 x 18 mm, with an 8 mm chamfer.
  - Weather bar: sloped and with a nose, closing on the sill stop and ending exactly at the jamb stops.
  - Two 4 mm panes, 664 x 1078 mm, and the 16 x 12 mm edge-seal ring between them.
  - Two hinge barrels (16 mm, on the axis) with their sash leaves.
  - Pull grip: two posts and a 12 mm bar.
  - Two 8 mm stay pins passing through the arc slots.
- **Kinematics** (kit check):
  - The pin stays on the arc centre line in every pose (0.0 mm off at 0°, 3° and 6°), because the arc is concentric with the axis.
  - The slot ends are the 0° and 6° stops.
  - Every point of the sash rises as it tilts, so there is no contact with the sill or the stops.
  - The top exterior corner rises at most 4 mm into the 12 mm head clearance.
- **Thermal:**
  - pane-glass and pane-edge as three real shells each (inner pane, cavity or edge body, outer pane) over the visible glass. Rims are cut faces. The room-side face and the faces inside the sealed unit are marked (C25 b, c).
  - Sash timber references the body with its room-side faces marked.
  - The jamb, head and sill boards have their room-side end faces marked. The jamb backs and the head top are marked as faces against the masonry.
  - Outer areas: pane-glass 0.466 / 0.47, pane-edge 0.202 / 0.2, frame-timber 2.354 / 2.25, frame-sill 0.550 / 0.62.
- **Structure:**
  - Members: stiles, rails, sill, jambs, head and the stay arcs.
  - Joints: the hinge barrels and the stay pins.
  - Support: a 12 mm pin through each hinge barrel, 15 mm proud at both ends.
  - Mass at the centre of mass of the sash.

### 1:100: body 12, thermal 12, structure 12

- **Body:** plain boards; the stops as one stop ring; plain stiles and rails with rebates and beads merged; a weather bar block 38 x 62 mm; the glass as one 24 mm slab at sight size; the hinge barrels.
- **Dropped** (below 15 mm): grip, stays, pins, leaves, edge seal, drip groove and sill fall.
- **Thermal:** glass centre and edge each merged to one 24 mm solid (B2), plus the timber references.
- **Structure:** the members, the hinge joints, 15 mm stay-pin joint marks, a 15 mm support pin and the mass.

### 1:200: body 4, thermal 5, structure 7

- **Body:** the sill board, the lining arch (jambs and head in one piece), the sash as one 70 x 90 mm ring, and the glass slab.
- **Thermal:** the glass centre takes the slab and the edge zone shares it with every face marked. This mirrors the shared 2D glass line; only the total is compared at this scale.
- **Structure:** ring, sill and arch as members, a 32 mm hinge joint, 30 mm stay joints, a 30 mm support pin and the mass.

Final check line: `PASS (2.4 s)`.

- Silhouette IoU is 1.000 in all views at all scales, and bounds deviation is 0.
- Cross-scale 1:200 front p95 is 0.06 against a limit of 0.09.
- The monotonic rule drops at every step.
- Every thermal region is within its tolerance.

## 5. What I saw in the final renders

Files are in `🗑️generated/r3-sheet/components/tilt-window.*` and `🗑️generated/r3-2d/components/tilt-window.*`.

- **1:50 body, front (exterior):**
  - The lining box with the stop strips framing the sash.
  - The sash reads as a separate frame: the stile strip between stop and sightline, rails, and the weather bar over the sill stop.
  - The glass sits in the sightline.
- **1:50 body, side views:** a 300 mm lining slab with the sash inside. Tilted to 6°, the sash leans into the room, the grip projects, and the gap at the head is visible.
- **1:50 body, back (room side):** the beads and their mitres, the grip centred on the top rail, the green steel hinges at the bottom corners and small stay parts at the top corners.
- **1:50 body, top:** at rest only the head board shows. Tilted, the sash top and the grip appear beyond it, in the room.
- **1:50 body, three-quarter from the room** (turn 225, tilt 15, fully tilted): it reads at first sight as a real tilted timber window. The hinges sit at the room-side corners and the glass sits behind the beads.
- **1:50 thermal, cut:** red centre, green 65 mm edge band, purple timber, yellow sill, cut faces hatched.
- **1:50 structure:**
  - Blue boards and sash members; orange hinge joints with green support stubs at their ends; magenta mass at the sash centre.
  - In the tilted three-quarter view the joints sit on the axis at the corner.
- **1:100 and 1:200:** recognisably the same window, simplified as described. At 1:200 the 12 mm rebate clearance shows as a thin light line around the sash ring.
- **2D at 1:50:**
  - The section reads as a joinery detail (crops checked: rail and bead with chamfer, weather bar, stops, sloped sill with drip, hinge pin, grip).
  - The front line view is clean after the weather bar was reshaped to close the 8 mm and 4 mm slivers. Before that change, the line view showed dots there.
  - The tilted pose reads correctly in all three views.

## 6. Still imperfect

1. **The stays are idealised.** The quadrant stays are modelled as the real concentric hardware, but the structural model still idealises their two pins as one horizontal roller at 1.15 m. The real reaction is tangential to the arc, 2.6° to 6° off horizontal. The solver has no inclined roller per pose, so this was not changed.
2. **The spring catch is not modelled.** It is mortised in the top rail and invisible; this is stated in the decisions.
3. **Hardware thickened to the 1:50 feature.** Hinge leaves and the arc are 8 mm thick, while real parts are 3 to 5 mm. The 1:50 feature is 7.5 mm.
4. **The sill underside stays counted.** It rests on masonry, so by C25 b it should be excluded, which gives 0.31 m². The area reviewers' R-C value of 0.62 counts it, and the gap is beyond the factor 1.5 an author may correct. The model keeps the underside counted (0.55 / 0.62) and the question is in `area-review.md`. The jamb backs and the head top are marked; this asymmetry is deliberate and documented there.
5. **The front line view keeps four short ticks** where the sill and head stops meet the jamb stops. They are real joint lines 30 mm long (0.6 mm on paper).
6. **No 2D shapes were drawn for some parts at 1:50.** The stay arc and grip are not in the 1:50 front or plan, because they are hidden behind the stops or the head.

## Round 5 handcraft (2026-11-21)

Script: `r5-tilt-window-volumes.py`. Renders (opened with Read) under `🗑️generated/r5-tilt-window/before/` and `after/` (made by `r5-c01-sheets.sh`). I opened body, structure and the cut thermal sheets at 1:50; body at 1:100; body and structure at 1:200; the 2D at 1:200; and the pose `fully-tilted`. 1:500 stays unavailable.

### Model against reference (1:50)

| quantity | reference | model | deviation |
|---|---|---|---|
| sash profile depth | 90 mm (DIN 68121-1 IV 92 family) | 90 mm | 0 % |
| rebate clearance | 12 mm (DIN 68121) | 12 mm | 0 % |
| glass | 4-16-4, 24 mm | 24 mm | 0 % |
| tilt at the stay stop | 130 mm at the head (Winkhaus SH.SE) | 126 mm (6°) | -3 % |
| lining | 50 mm boards, 300 mm reveal | 50 / 300 mm | 0 % |
| sash mass | glass 14.3 kg + timber 9.7 kg + seal 1.0 kg + steel 0.8 kg | 25.78 kg | from the parts |

### Grades (body / thermal / structure)

| scale | before | after |
|---|---|---|
| 1:50 | A- / A- / A- | unchanged |
| 1:100 | A- / A- / A- | unchanged |
| 1:200 | B (the 12 mm clearance stood open all round: daylight through the frame, a gap below the 30 mm feature) / A- / A- | A- / A- / A- |

### Changes

- At 1:200 there is a new 30 x 30 mm exterior stop ring (`stop-ring`, with a thermal reference). It covers the clearance, as the stop strips do at 1:50 and 1:100.
- **Root state repaired:** the catalog carried a stale authored 1:500 volume (lining, sash ring, pane) although `representations.500` is unavailable. The splice restores `status: unavailable`.
- Checks: `volumes-check --product` PASS (18 s). `r3-validate --volumes` VALID. Spliced; `--catalog-only` VALID.

### Remaining

- The 1:200 front 2D still draws the clearance as a double line. The true outline of the stop lies 18 mm inside it, below the 30 mm feature.
- Moiré on the thermal glass faces is a coincident-face artefact of the renderer.
- The remaining points of round 3 (idealised stay roller, 8 mm hardware) stand.

## Canonical component readback, 2026-11-21

Canonical source `315c6ca893481216`. Two separate 4 mm panes surround a clear 16 mm gas cavity and a perimeter seal. The glass and handle travel with the tilting sash. The stay, hinges, sill and stops remain distinct. All twelve exported scenes reopened (572,504 bytes). The strict catalog source readback passed.
