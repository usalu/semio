# Round 3: appliances/gas-hob

State on 2026-11-20: finished and spliced.

No earlier round-3 work existed for this asset. Only the round-1 script `author-12-gas-hob.py` was there, so I started from scratch.

- `r3-validate.ts appliances/gas-hob --catalog-only` returns `VALID ... schema 0, validateAsset 0`.
- `volumes-check.ts appliances/gas-hob --product` passes every rule at every scale except one: `volumes-thermal-coverage` on the region `flame`, at 1:50 and 1:100. That failure is intended and waits for the area reviewers (section 5).

Scripts in the ticket folder:
- `r3-gas-hob-kit.py`: the measures and the solid writers, including the flame-tongue loft.
- `-model.py`: states every solid of every scale and list by hand.
- `-draw.py`: draws the 2D of every scale and view (ruling E1).
- `-roots.py`: builds the candidate and, with `--apply`, writes the changed members into the catalog.
- `-texts.json`: provenance notes, areas, and the volume and drawing decisions in en and de.
- `-flames.json`: the projected 1:50 tongue outlines.
- `-zoom.ts`: crops the 2D panels to the drawn object.
- `-run.sh`: rebuilds everything, writes the catalog only when the candidate validates, then checks.

## 1. Reference

| Quantity | Real subject | Source | Model now |
|---|---|---|---|
| Overall W x D | 30 cm domino: 306 x 527 mm, built-in height 45 mm | BSH data sheets (Bosch PRB326B90X, PRA3A6D70; Siemens ER3A6AD70), read 2026-11-20 (new reference `bsh-domino-gas-2024`) | 0.30 x 0.52 m: a 0.497 m plate plus a 23 mm knob in front of it. `dimensions` unchanged |
| Height | about 90 mm with the pan supports (45 mm box below the top + about 45 mm supports) | the same | 0.088 m: feet 7.5, housing to 30, top 10 mm, pan-support top 48 mm above the plate |
| Rapid burner | 2.5 to 3 kW. Burner caps are 73 to 100 mm across; the crown has a ring of ports under the cap edge; the cup sits in a pressed well | GEV catalogue Gas 2015, Sabaf series 1 and 2 (new reference `gev-burners-2015`); DIN EN 30-1-1 | cup Ø 82 mm, crown Ø 90 mm with 36 ports, brass cap Ø 96 mm. Burner 20 mm high, in a 0.15 m spill well |
| Flames | blue inner cones 5 to 8 mm and tongues about 20 mm at full rate, leaning out and up, almost touching | burner practice | inner-cone ring plus 26 tongues of 7.5 mm, up to 22 mm (world height 0.069) |
| Pan support | one cast-iron grate around a single burner, 8 to 10 mm bars, fingers towards the burner, pan 30 to 50 mm above the plate | BSH domino data sheets; asset (48 mm) | 0.27 m square frame of 8 x 8 mm bars on four 8 mm legs, four fingers drooping to 66 mm from the axis. About 0.78 kg of iron against the asset's 0.7 kg |
| Knob | hob knobs: turned body with a grip ridge and a pointer, axis normal to the panel it sits on | product knowledge; the rig's knob is a revolute joint in the elevation plane | Ø 26 mm turned body with a 7.5 x 22 mm ridge, on the front fascia (see 2) |
| Feet | rubber feet of 15 to 25 mm under free-standing appliances | product knowledge | four EPDM feet, Ø 20 x 7.5 mm |
| Sheet | enamelled top 0.6 to 0.8 mm, pressed housing 0.6 to 0.8 mm | asset (`plate-skin` 0.8 mm) | housing tray with 0.8 mm walls (structure); top modelled as a 10 mm plate with its rim |

## 2. Audit of the state before round 3

- **Rig.** The joints are in the right places.
  - The `knob` revolute turns in the elevation plane. That is only true for a knob on the front fascia, which is what the elevation drew (round face and pointer).
  - The flame bones are acceptable as they are.
  - The rig is unchanged.
- **The 2D views contradicted each other about the knob.**
  - The elevation drew it on the fascia (y 0.008 to 0.030).
  - The front view and the plan drew it on top of the plate, 60 mm inside the front edge.
  - I kept the fascia, which agrees with the rig. A built-in domino has its knob on the top; this asset stands on its housing as a table-top unit.
- **Other 2D errors.**
  - Plan: the pan support was a 0.27 x 0.134 m rectangle that barely cleared the 0.104 m burner.
  - Elevation: the flames were three tongues at ±30 mm, inside the burner ring, under a 40 mm cap. Real flames come out at the crown rim, about Ø 90.
  - `plate-lip` was a line at y 0.012 on the housing, where nothing is.
  - `vent-slots` was a zigzag.
  - Feet and spill well were missing.
  - At 1:200 the pan support was drawn as a closed block.
  - The 1:100 plan pan support was an unfilled outline.
- **Solids** (`qa-visual.md` graded body B/B/B, thermal B/B/B, structure C/C/B):
  - The 1:100 and 1:200 burner and flame revolves were open profiles, filled through the axis (`kernel-traps.md`).
  - The housing was a box with a flat underside.
  - The flame was spiky cones.
  - The trivet mass floated in the air.
  - Structure was mostly `{ of }` copies of the body.
- **Thermal areas.**
  - `knob` 0.0025 had been accepted for "two knobs", but a single-burner domino has one.
  - `burner` 0.012 was kept by the reviewers; their own hand value was 0.0145.
  - `plate` 0.14 counted only the top face.

## 3. Root fixes (catalog file, outside `volumes`)

| Member | Old | New |
|---|---|---|
| `materials` | 5 | + `rubber` (EPDM, ε 0.92, 1150 kg/m³, 0.25 W/mK, 1900 J/kgK, Incropera) |
| `anchors.base-contact` | footprint 0.30 x 0.50 m | 0.26 x 0.455 m (the four feet) |
| `structure.assumptions[0]` | contact of 0.30 m by 0.50 m | four rubber feet on 0.26 m by 0.455 m (en and de) |
| `thermal.regions.plate.areaM2` | 0.14 | 0.175 (1:50 outer area; factor 1.25) |
| `thermal.regions.knob.areaM2` | 0.0025 | 0.002 (one knob; factor 1.25) |
| `thermal.regions.burner.areaM2` | 0.012 | 0.0137 (cup, crown and cap; factor 1.14) |
| `thermal.regions.trivet.areaM2` | 0.043 | 0.047 (factor 1.09) |
| `thermal.regions.flame.areaM2` | 0.008 | unchanged: the honest value 0.0185 is more than 1.5 times the old one, so it is proposed in `area-review.md` and not applied |
| `provenance` | 4 references, old notes | + `bsh-domino-gas-2024` and `gev-burners-2015`; notes rewritten in en and de (table-top domino, 0.497 m plate and knob, burner, pan support, origin) |
| `representations[50,100,200].geometry` | old drawings | every shape redrawn from that scale's model, keeping ids, bones, roles, z, tones, regions and materials (section 4) |
| `representations[*].decisions` | old texts with duplicated sentences | rewritten per scale in en and de |

Shapes added and removed:

- **New at 1:50:**
  - `feet` in the elevation and front view (material rubber).
  - `plan-plate-well` (detail).
- **New at 1:100:**
  - `knob` in the elevation and front view.
  - `plan-knob`.
  - The plate shape now carries only the plate region.
- **New at 1:200:**
  - `trivet-leg-left` and `trivet-leg-right` in the elevation.
  - `plan-trivet`.
- **Removed:** `plate-lip` at 1:50, which drew no real edge.
- **The knob uses an `extruded` solid hint** (sweep w, from the fascia to the ridge) in the elevation and front view, so the side view stays right in every pose. Without the hint, the 110-degree pose squashed the rest-pose side view into a sliver.

Unchanged: rig, poses, clips, emitters, supports, `dimensions`, masses and summary.

## 4. What was modelled (solid counts are list entries; repeats expand to more)

### 1:50 (feature 7.5 mm): body 27, thermal 16, structure 26

- **Body**
  - Housing: a rounded extrusion of 0.28 x 0.484 m from 7.5 to 31 mm, with texture bead lines and 9 + 2 x 11 vent slots.
  - Feet: four EPDM feet (two repeats).
  - Top: the 10 mm field over the housing, the rim ring (2 mm proud at the front, 10 mm at the sides and back) and the turned spill well (Ø 150 mm, 4 mm deep).
  - Burner: cup on the well floor, crown with 36 texture ports, cap with a domed top.
  - Flames:
    - the inner-cone ring on bone `flame`;
    - 2 x 11 tongues on `flame-body`, as repeats of one loft each with sections square to the curved centre line;
    - 2 longer-rooted tongues on `flame-tip`, front and back.
  - Knob: turned skirt and body, grip ridge, and a pointer stripe as texture.
  - Pan support: the frame extrusion, two pairs of legs, and four drooping fingers (one profile, rotation repeat).
- **Thermal**
  - Plate: the top with `-v` on the faces lying on the housing (C25 b), the rim, and the well (`-v`).
  - Knob: body (`+w` on the fascia) and ridge.
  - Burner: cup (`-v` seat on the well floor) and cap.
  - Flame: ring and the three tongue groups.
  - Emitter: an outlet ring of 7.5 mm at the tongue tips (D2).
  - Pan support: frame, legs and fingers.
- **Structure**
  - Housing tray with 0.8 mm walls; top, rim and well as members; feet as members; a slender gas pipe (ratio 29.9).
  - Support ring at `base-contact`; the hob mass at its centre.
  - Burner body on a seat ring (joint), with its mass.
  - Knob spindle into the brass gas tap (the knob's joint).
  - Pan-support frame, legs and fingers; leg seats as joints; four mass lumps on the frame.
  - Flame volumes as the members of the flame bones, which carry no load; a joint at `flame-body`.

### 1:100 (feature 15 mm): body 8, thermal 9, structure 16

- **Body**
  - One rounded block of housing and top.
  - Knob as a 26 mm turned stub.
  - Burner as one turned puck of 94 mm.
  - Twelve 15 mm tongues.
  - Pan support of 10 x 12 mm bars: frame, legs and four fingers.
- **Thermal:** a top slab of 15 mm (`-v`), the references, and a 15 mm emitter ring.
- **Structure**
  - Housing tray, top slab with the burner opening, a 15 mm support ring, a 15 mm burner seat.
  - Knob spindle and tap.
  - Pan-support members and mass lumps.
  - Flame ring, crown, and two tip tongues.

### 1:200 (feature 30 mm): body 7, thermal 8, structure 13

- **Body**
  - The block with the knob as a 30 mm stub.
  - Burner as a 30 mm puck; flame as a 28 mm crown dome (as the drawing has it).
  - Pan support: a 15 mm frame on four 15 mm posts, with the 33 mm gap left open. The 1:200 drawing now draws the legs separately (C13), which `qa-visual.md` asked for.
- **Thermal:** a 30 mm top slab (`-v`), knob stub, references, and a 30 mm emitter ring.
- **Structure:** a 30 mm wall box, top slab, support ring, burner, spindle, pan support with lumps, and the flame members.

### 2D (ruling E1)

- Everything except the flames is drawn analytically from the same profiles as the solids: cup and cap halves, the rail outline with the finger droops, the plan bars with their fingers, the 24-lobe flame star, and the knob.
- The 1:50 tongue outlines come from `r3-project2d.ts` and are drawn as one path per group. The tool's round trip is 0.85, which is the scale's simplification.
- At 1:100 the tool lost the middle tongues (round trip 0.68), so I drew the crown by hand: crenellated, with tips at 0, ±28 and ±51 mm.

### Final check (`volumes-check.ts appliances/gas-hob --product`)

| Rule | Result |
|---|---|
| Silhouette IoU, elevation / front / plan | 1:50: 0.988 / 0.983 / 1.000. 1:100: 0.981 / 0.983 / 1.000. 1:200: 1.000 / 0.983 / 1.000 |
| Bounds, feature, monotonic, cross-scale, structure, bones, slender, copied | all ok |
| `volumes-thermal-coverage` | FAIL at 1:50 (flame +132 %) and 1:100 (flame +65 %); every other region is within 2 to 15 %. 1:200 ok (total) |

Summary line: `FAIL: 2 measurement(s) over 2 rule line(s)`.

## 5. What I saw in the final renders

Images I opened: `r3-sheet` body sheets at 1:50, 1:100 and 1:200; the 1:50 three-quarter close-up; the right side at 1:50 in the `flicker` clip at t = 0.5; thermal at 1:50 (plain and cut) and the 1:200 cut; structure at 1:50 and the 1:100 three-quarter; the `r3-2d` zooms at every scale and the 1:50 `off` pose.

- **Front, from the elevation side.**
  - Fascia with bead, vent slots and the ridged knob at the right; rubber feet; the plate flush with the fascia.
  - The burner reads as cup, crown and cap, with a crown of flame tongues.
  - The pan support stands on two visible legs, its finger droops are visible, and the back finger shows end-on through the frame.
- **Side, from +x.**
  - The knob stands 23 mm proud at the left end.
  - Burner and flames sit 30 mm behind the middle.
  - The pan support stands on its legs.
- **Back.** The plain fascia, with the knob showing past the end.
- **Plan.**
  - The top with its spill well.
  - The 96 mm cap ringed by flame lobes.
  - The square pan support with four fingers ending 4 mm short of the flame crown.
  - The knob in front of the plate.
- **Three-quarter.** Reads at once as a single-burner domino with a cast-iron pan support. The flame reads as a crown of separate tongues.
- **Thermal.**
  - The plate, burner, flame and pan support are separate.
  - The cut shows the burner under the flame ring and the outlet ring at the tongue tips.
  - The housing is a ghost.
- **Structure.**
  - The tray, top and feet; the support ring on the ground.
  - Mass lumps on the frame; leg seats as joints; the tap and spindle inside the housing.
- **Pose `off`.** The flames are hidden inside the housing and well, in 2D and in 3D.
- **`flicker` at -8 degrees.** The whole crown tilts about the burner axis; the tongues on one side drop to 1.5 mm above the plate.
- **1:100 and 1:200.** Generalised but in proportion; the gap under the 1:200 frame is open.

## 6. Still imperfect, stated plainly

- **Flame area.** `flame` stays 0.008 against an honest 0.0185, so the check fails at 1:50 and 1:100 until the area reviewers decide (`area-review.md`, new section `appliances/gas-hob`).
- **Flame lean.** The rig leans the flame as one ring about the burner axis (`flame-body` revolute), not each tongue about its own root. A real draught flattens the windward flames and stretches the leeward ones. A per-tongue lean would need a different rig; I did not invent one.
- **Flame colour in renders.** The tongues render grey because the shading uses the tone, not the material colour (tool behaviour).
- **Flames hide the cap in 2D.** In the 1:50 elevation the flame outlines (z 3.4) cover the cap (z 3.3) where real gaps between the tongues would show it. The layer order was kept as E1 requires.
- **Seam on the top.** The top is split into field, rim and well so that its mounting faces can be marked honestly. `r3-sheet` therefore shows a seam line about 10 mm inside the edge; it is not a real edge of the pressed sheet.
- **Sheet thickness.** The top is modelled 10 mm solid, not as a 0.8 mm sheet with a hemmed rim. A closed sheet lid cannot be built from the kernel's six solid kinds, and the rim radius of 2 to 3 mm is below the feature.
- **Small parts.** The ignition electrode and thermocouple (5 to 6 mm) are below the 7.5 mm feature and are left out.
- **Interior parts.** The gas pipe and tap exist only in `structure`. The injector and venturi were tried in `thermal`, but they count as outer area (C29 counts faces covered by other solids) and would have made the burner area meaningless, so they were removed.
- **Table-top variant.** Built-in dominoes have their knob on the top. This asset stands on its housing as a table-top unit, so its knob is on the fascia, as its rig requires.
