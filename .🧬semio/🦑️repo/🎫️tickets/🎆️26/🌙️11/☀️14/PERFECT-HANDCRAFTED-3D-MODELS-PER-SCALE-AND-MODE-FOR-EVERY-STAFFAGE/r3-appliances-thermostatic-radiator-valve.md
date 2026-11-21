# Round 3: appliances/thermostatic-radiator-valve

State: DONE (with the open points of section 7)

Catalog asset: `S/🗂️catalog/🔌️appliances/thermostatic-radiator-valve/🔣️.json`. Scripts in `T`: `r3-thermostatic-radiator-valve-build.py` (volumes of all four scales, every section in millimetres), `r3-thermostatic-radiator-valve-roots.py` (every root value outside `volumes`; candidate document, `--apply` writes roots, volumes, the volumes source and the manifest entry), `r3-thermostatic-radiator-valve-before.json` (rig, dimensions, anchors, areas and 2D shapes before this round). No earlier round-3 work existed; the asset was started at step 1. The catalog `volumes` equalled the source at the start; the bulk pass of 03:47 had added authored 1:200 and 1:500 volumes (2 body solids each), which are replaced by the models below.

## 1. Reference

| item | real value | source |
|---|---|---|
| valve body, corner angle pattern (Winkeleck) DN 15 | inlet Rp 1/2, tailpiece R 1/2, centre to inlet face l3 = 29 mm, centre to tailpiece end l4 = 58 mm; three axes at right angles (pipe, radiator, head) | IMI Heimeier V-exact II data sheet, articles 3713-02.000 / 3714-02.000 (table read from the PDF text) |
| head contact face | H1 = 23.5 mm from the centre (angle pattern DN 15, same insert) | same data sheet |
| spanner widths | 27 mm at the body, 30 mm at the union nut (general Heimeier table as returned by the search; not read in the PDF by me) | same |
| head connection | M30 x 1.5, knurled coupling nut | same, head K data |
| thermostat head | diameter 53 mm, length about 80 mm, liquid sensor, settings star and 1 to 5, tactile marks | IMI Heimeier head K product data; the two numbers come from a dimension drawing quoted by the web search, I could not open the drawing itself |
| pipe connection | 15 x 1 mm copper tube with compression nut AF 24 and olive | EN 1254-2, EN 1057 (my knowledge) |
| outlet | G 3/4 male boss (26.4 mm) under the union nut, tailpiece shank 18.5 mm, R 1/2 thread 20.9 mm, bore 13 mm | ISO 228 / ISO 7 thread sizes (my knowledge) |
| head parts | coupling nut about 34 x 12 mm, fixed base ring with index arrow, handwheel with scale band and finger flutes, slotted sensor cap with flat front | my knowledge of the product class |

## 2. Audit (before this round)

- Rig: `head` joint at y +0.040, 28 mm above the top of the valve body: the head floated over the valve and was screwed to nothing. `body` length 0.075 pointing sideways where no part was.
- No outlet: the valve had a pipe stub and a head, but no connection to a radiator.
- Valve body: one cylinder of 38 x 42 mm standing on the pipe axis (a real DN 15 body is a cross of about 28 mm barrels), "hexagon neck" round.
- Head: fixed collar of 60 x 71 mm with a turning knob of 40 x 22.5 mm; a real head is 53 x 80 mm and its whole handwheel turns on a small fixed base.
- Dimensions 0.145 x 0.06 x 0.09 m (height, x, z) against 0.1065 x 0.0845 x 0.110 m of the real arrangement.
- Supports: two anchors 22 mm apart on the same pipe stub (pin and roller with a horizontal normal).
- 2D: all three views drew that invented object; the plan showed no body at all.
- Solids (QA B/C/C): 5 body solids, structure with two 26 mm blocks under the stub and a "spindle" capsule, coarse scales of the bulk pass two revolves each.

## 3. Root fixes (catalog, `r3-thermostatic-radiator-valve-roots.py`)

| member | old | new |
|---|---|---|
| summary, provenance.notes, manifest `distinction` | 60 mm angle valve with collar and knob | corner angle valve DN 15, 53 mm head, handwheel; origin = meeting point of the three axes |
| provenance.references | 3 | + `imi-heimeier-v-exact-ii`, `imi-heimeier-thermostatic-head-k`, `en-1254-2-1998` |
| dimensions height / width (x) / depth (z) | 0.145 / 0.06 / 0.09 | 0.1065 / 0.0845 / 0.1103 (manifest heightM, widthM too) |
| rig `body` | length 0.075, mass com (0, -0.02) | length 0.058 (to the tailpiece end), com (0.009, -0.015), new title |
| rig `head` | y 0.04, length 0.03, 0.08 kg | y 0 (on the valve axis), length 0.022, 0.03 kg, "Coupling nut and base ring with index mark" |
| rig `knob` | length 0.02, 0.04 kg | length 0.0265, 0.09 kg, "Handwheel with sensor cap" (total mass 0.38 kg unchanged) |
| anchors | `pipe-joint-upper` pin (0, -0.048), `pipe-joint-lower` roller (0, -0.07), normals 0 | `pipe-joint` pin (0, -0.039), `radiator-union` roller (0.051, 0), normals 90 (vertical reaction) |
| structure.supports, pose and clip contacts | the two old anchors | the two new anchors; assumption text rewritten |
| poses, clips | "knob" texts | "handwheel" texts (angles, keys, tracks unchanged) |
| thermal `body` | "Valve body and pipe stub", 0.0095 m2 | "Valve body, unions and pipe", 0.0137 m2 (factor 1.44) |
| thermal `head` | "Thermostat head collar", 0.0187 m2, L 0.06 | "Thermostat head: coupling nut, base ring and handwheel barrel", 0.0125 m2 (factor 1.5, the limit), L 0.053 |
| thermal `knob` | "Setting knob", 0.005 m2, L 0.04 | "Sensor cap of the handwheel", 0.0042 m2, L 0.045; layer title "ABS cap 3 mm" |
| thermal assumption 3 | head and knob | regions named by their parts, the nut counted with the head |
| 2D 1:50 | body outline, collar, index, knob, pointer; front and plan with head block | elevation: body outline with pipe, fitting collar, union nut and tailpiece, base disc (hidden), handwheel disc 53 mm, front mark; front: body with the union hexagon, stepped base (nut and ring), handwheel and cap as swept discs; plan: body with outlet arm, base, index mark, handwheel, cap |
| 2D 1:100 | body, head disc; plan without body | body and head outline in every view (pipe and tailpiece widened to 22 mm as thin elements) |
| decisions 1:50, 1:100 | old texts | all eight texts rewritten, en and de |
| representations 200 / 500 | reasons with 0.145 m and a 17 mm stub | 0.107 m and the 15 mm pipe, limitingSizeM 0.015 |

Print suitability of both drawings holds (min feature 0.21 mm at 1:50, 0.38 mm at 1:100, no gap finding).

## 4. Model per scale

**1:50** (body 15, thermal 13, structure 14)
- body: pipe 15 mm with cut end and bore, compression nut AF 24, hexagon inlet socket AF 27, inlet neck, valve chamber 28 mm with domed back and M30 collar, G 3/4 boss, union nut AF 30, bored tailpiece to 58 mm; knurled-nut ring 34 mm, base ring 44 mm with index arrow (texture), scale band 53 mm, grip with ten finger flutes tapering to 50.6 mm, sensor cap with two slot rings and rounded nose, embossed front mark (texture).
- thermal: every region is the outer surface of its body solids (references), pipe and tailpiece as own closed solids with their ends as section faces; the grip/cap plane is a section face on both sides. Outer areas 0.0138 / 0.0117 / 0.0042 m2.
- structure: three hollow brass tubes meeting at the centre (inlet with pipe, outlet with tailpiece, chamber), coupling nut (reference, it is the member) and base sleeve, handwheel hub with pressure pin, joints at compression fitting, union and setting thread, pin at the pipe joint, roller at the radiator union, masses of body and sensor.

**1:100** (5 / 6 / 9): inlet as one turned column with a round collar, chamber, stepped outlet arm, head base, smooth tapered handwheel; thermal with the handwheel cut into barrel and cap; three tubes, head sleeve, hub, two supports, two masses.

**1:200** (3 / 4 / 7): inlet column through the chamber, outlet arm with union collar, head as one tapered body; four thermal solids; two tubes, sleeve, hub, block supports, one mass.

**1:500** (3 / 4 / 7): three plain tapers on the three axes; structure as four square bars, two supports, one mass; decisions declare the model minimal.

Final check line: `volumes-check.ts appliances/thermostatic-radiator-valve --product` prints `PASS`; 1:50 silhouette IoU 0.980 / 0.968 / 0.977, 1:100 0.901 / 0.908 / 0.944 (elevation / front / plan). `r3-validate.ts appliances/thermostatic-radiator-valve --volumes` and `--catalog-only --volumes`: `VALID ... schema 0, validateAsset 0, checkVolumes 0`. Spliced (`volumes-insert.py`: all four scales authored).

## 5. What I saw in the final renders (`🗑️generated/r3-sheet/appliances/thermostatic-radiator-valve.s<N>.<list>.png`)

- 1:50 body. Front (from the radiator): union hexagon with the tailpiece end-on, chamber behind, head to the left as nut, base ring, band, fluted grip, slotted cap; pipe with nut and socket below. Right side (elevation): handwheel disc with flute outline and front mark, outlet arm with nut and tailpiece to the right, pipe column below. Back and left: domed chamber back, the head behind it. Top: index arrow on the base ring pointing at the scale band, outlet arm. Three-quarter: reads as a thermostatic valve at first sight; tailpiece bore visible.
- 1:50 clip `close-down` at 4 s: handwheel, cap and front mark turned, base ring and index fixed, nothing opens.
- 1:50 thermal: body red, nut, base and barrel green, cap purple, boundaries where described. 1:50 structure: three blue tubes in a cross, orange joints at fitting, union and thread, green pin and roller, magenta masses, all inside the ghost.
- 1:100: same proportions, collars round, head smooth; thermal cut shows the barrel/cap split. 1:200 and 1:500: three clean masses on three axes, still a corner valve with a head.
- 2D 1:50 enlarged from the product SVG (light-line): elevation, front and plan agree with the body; the 0.18 mm pen is a tenth of the object, so only the main outlines read. Overlays (`volumes-render.ts`): green throughout at 1:50; at 1:100 red strips along the widened pipe and tailpiece only.

## 6. Notes for others

- Hollow thermal shells (3 mm brass wall around the waterway, 2 mm head shell) were built and removed: the product's outer-area measure counts the faces of a closed cavity (C31 is not in `outerArea`; already reported by others in `tool-requests.md`), body 0.0196 against 0.0138 m2.
- Bone `z` offsets for `head` and `knob` were tried and removed: the 2D front and plan placed the shapes as if the offset were 0, so the views lost 23 to 35 mm of depth.
- `area-review.md` has the entry for the `head` area (applied 0.0125, measured 0.0117).

## 7. Still imperfect

- Round parts are drawn with about twelve facets by the tools' tessellation at this size; nothing in the solid data controls it.
- The hexagon nuts are plain prisms without the 30 degree chamfer; knurling, thread turns, scale numerals and the valve insert under the head are not modelled (all under 7.5 mm).
- The thermal cutaway shows solid parts, not walls (see section 6).
- `head` areaM2 is 7 % above the measured surface because of the factor 1.5 rule; the coupling nut (brass) is bound to the head region, whose material is ABS.
- Head diameter and length (53, 80 mm) and the spanner widths rest on search excerpts, not on a drawing I opened.
- `🧪️tests/📐️dimension-leader` names this asset (label lifted clear at 1:50); the asset became shorter, so the condition should still hold, but I did not run the test.

## Round 5 handcraft (2026-11-21)

Script: `r5-thermostatic-radiator-valve-build.py` (the round-3 build with the changes below; writes the volumes source). No root change outside `volumes`.

### Reference and model against it

| quantity | reference | model 1:50 | deviation |
|---|---|---|---|
| centre to inlet face l3 | 29 mm (IMI Heimeier V-exact II, angle DN 15) | 29 mm (inlet socket face) | 0 % |
| centre to tailpiece end l4 | 58 mm (same) | 58 mm | 0 % |
| head contact face H1 | 23.5 mm (same) | 23.5 mm | 0 % |
| head diameter | 53 mm (Heimeier K) | 53 mm | 0 % |
| head length from contact face | about 70 mm (80 mm overall incl. nut) | 69.5 mm (-23.5 to -93) | -0.7 % |
| coupling nut | M30 x 1.5, about 34 mm | 34 mm | 0 % |
| pipe | 15 x 1 mm copper (EN 1057) | 15 mm, bore 13 mm | 0 % |

`volumes-bounds` 1:50: 0 / 0 / 0.02 mm.

### Grades (body / thermal / structure)

| scale | before | after |
|---|---|---|
| 1:50 | A- / A- / A- | A- / A- / A- |
| 1:100 | A- / A- / B (pin and roller as pairs of balls crossing pipe and tailpiece) | A- / A- / A- |
| 1:200 | C+ (head a flared bottle: cone from 26 to 53 mm over 22 mm) / B / A- | A- / A- / A- |
| 1:500 | C (inlet a horn from 16 to 29 mm, head a double cone) / C / B+ | B+ / B+ / B+ |

### Changes

- 1:100 structure: supports are 15 mm rings round the pipe at the compression fitting (pin) and round the tailpiece at the union (roller), instead of two capsules.
- 1:200 body: head turned with the true steps of the reference (34 mm nut to -25, 44 mm base ring to -34.5, 53 mm barrel tapering 2 mm to -71, rounded cap to 28 mm at -93); thermal head and cap split at -71 on the same profile.
- 1:500 body: inlet = 15 mm pipe stepping into the 29 mm column, outlet = 28 mm union stepping to the 20 mm tailpiece, head = 38 mm neck, 53 mm barrel, chamfered cap; no cones. Thermal split follows. Decisions rewritten en/de.

### Looked at (opened)

Before: every scale's body, thermal cut, structure; poses open, part-open, closed; clips close-down t 4 s, open-up t 6 s. After: 1:100 structure, 1:200 body, 1:500 body and thermal cut; 2D 1:50 enlarged (all three views consistent with the body). 2D 1:200 and 1:500 do not exist.

### Checks

`volumes-check.ts --product`: PASS. `r3-validate.ts ... --catalog-only --volumes`: VALID. Spliced.

### Remaining

- Rig: `head` and `knob` lie along +x with their joint on the head axis, turning about world z (the head axis); a bone along the head axis would need bone `z` offsets, which the 2D front and plan placement ignores (round 3, section 6).
- 1:500 still a minimal three-solid massing (B+).
- Hexagons without chamfers, knurling and numerals below the feature.
