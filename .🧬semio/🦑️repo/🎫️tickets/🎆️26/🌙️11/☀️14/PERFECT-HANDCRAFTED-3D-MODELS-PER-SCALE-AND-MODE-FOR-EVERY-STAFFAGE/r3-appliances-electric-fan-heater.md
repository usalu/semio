# Round 3: appliances/electric-fan-heater

State on 2026-11-20: finished and spliced. No earlier round-3 work existed for this asset (only the round-1 script `author-11-electric-fan-heater.py`), so it was started from scratch. The catalog file holds the corrected roots, the redrawn 2D of every scale and view, and the new volumes. 1:500 stays unavailable, as its representation already was.

Final lines:
- `$BUN r3-validate.ts appliances/electric-fan-heater --catalog-only --volumes`: `VALID ... schema 0, validateAsset 0, checkVolumes 0`
- `$BUN volumes-check.ts appliances/electric-fan-heater --product`: `PASS (2.2 s)`

Scripts in the ticket folder:
- `r3-electric-fan-heater-kit.py`: the measures, profiles, handle centre line, fan blade outline and the solid writers.
- `r3-electric-fan-heater-model.py`: states every solid of every scale and list and writes `volumes/appliances/electric-fan-heater.json`.
- `r3-electric-fan-heater-draw.py`: draws the 2D of every scale and view from the same measures (ruling E1).
- `r3-electric-fan-heater-roots.py`: builds the candidate document; with `--apply` it writes only the changed members into the catalog file in that file's own style (`--check-style` proves the style round trip).
- `r3-electric-fan-heater-texts.json`: provenance notes, volume decisions and drawing decisions, en and de.

## 1. Reference

| Quantity | Real subject | Source | Model now |
|---|---|---|---|
| Envelope | 2 kW box and upright fan heaters with two heat settings, thermostat and handle: Einhell EH 2000 22.1 x 19.8 x 31.7 cm; Black+Decker BXFSH2000E 20.2 x 26.8 x 31.6 cm, 1.5 kg; Steba VTH 2 27.5 x 26 x 20 cm, 1.8 kg; Beldray EH0567WK 26.5 x 22.5 x 12.5 cm, 1.15 kg; Logik L20PTC23 25.7 x 17 x 13 cm, 1.49 kg | retailer and manufacturer listings read 2026-11-20 (web search); new provenance reference `retail-box-fan-heaters-2026` | 0.196 long x 0.24 wide x 0.31 high, 1.5 kg: `dimensions` unchanged, inside the class |
| Housing | injection-moulded ABS, 2 to 3 mm wall, edge radii 15 to 25 mm, separate front and rear grille frames | product knowledge; `casing` layer 2.5 mm | U-shell and top cover with 2.5 mm wall and 22 mm edge radius, 0.166 m long; front and rear frame 8 mm deep with 13 mm chamfers; housing 0.182 x 0.228 x 0.256 m |
| Feet | four rubber pads 25 to 30 mm across, about 10 mm, on moulded bosses | product knowledge; `feet` layer 10 mm | pads 25 to 30 mm x 10 mm under 34 mm bosses of 6 mm, at x +-0.058, z +-0.075 |
| Outlet | stamped steel louvre grille, opening about 0.17 x 0.19 m, slats sloping down and out | product knowledge; `grille` layer 1.5 mm | cassette 0.188 x 0.17 m with a 5 mm rim 4 mm proud, eight 1.5 mm slats of 17.6 mm chord at 34.6 degrees, pitch 20 mm (each stands for about two real ones), tips 4 mm past the rim at x 0.098 |
| Intake | moulded grille over the fan, slots of 5 to 6 mm | product knowledge; `intake` layer 2 mm | panel 0.194 x 0.164 m, 6 mm proud, seven slots of 11 mm (each stands for two real ones) |
| Handle | bar across the appliance, hand opening at least 90 mm wide, 25 to 35 mm clear | hand breadth 85 to 97 mm (DIN 33402); product knowledge | arch across the cover: opening 100 x 26 mm, bar 26 x 12 mm, posts 26 x 16 mm with flared roots |
| Controls | thermostat knob 40 to 45 mm, 10 to 15 mm proud; rocker or rotary mode switch | product knowledge | dial 44 mm, 12 mm proud with a grip bar, on the right wall at y 0.185; rocker 32 x 16 mm, 3.5 mm proud |
| Fan | axial impeller 120 to 140 mm, shaded-pole motor on a bracket behind the intake | product knowledge | five pitched blades of 0.13 m on a 36 mm hub in a shroud, lamination stack 20 x 46 x 54 mm with coil, on a bridge between the side walls |
| Heating element | two stages of open resistance coils in a sheet-metal or mica frame downstream of the fan | product knowledge; asset's two stages | sheet-steel box 0.16 x 0.142 x 0.054 m with two rows of three 7 mm coils, on two brackets |
| Mass | 1.1 to 1.8 kg | listings above | rig 1.47 + 0.03 kg unchanged; estimate from the model: shell and frames 0.68, motor and fan 0.35, heater 0.12, grille 0.06, rest 0.2 kg; centre of mass about (-0.005, 0.143) against the rig's (0, 0.14) |

## 2. Audit of the state before round 3

- **Rig**: `body` (fixed, at the ground centre) had `lengthM` 0.31, so its tip lay 0.21 m in front of the outlet. `knob` (revolute about z at y 0.185, radius 0.024) is right in place and axis.
- **Anchors**: footprints 0.035 x 0.04 m described a rectangular foot; the outlet anchor (0.098, 0.145) is right.
- **Dimensions**: in the class. Unchanged.
- **2D**: the housing was drawn 2.5 mm off centre (z -0.115 to 0.12, x -0.09 to 0.095). The handle was a 0.1 x 0.13 m slab with a 65 x 20 mm slot running sideways through it, which no hand can use. The outlet was a 20 mm box without frame, the "lower seam" and the inlet slot strokes on the panel edge matched no real feature, and the dial stood only 5 mm out of the wall.
- **Solids** (`qa-visual.md`: body A/B, thermal B, structure C): the front frame was a square plate on a superelliptic loft, the top cover a box with clipped corners, the louvres 7.5 mm thick, the feet rounded bricks. There was nothing inside. The structure was the housing block with a joint ring on the knob.
- **Check**: `volumes-thermal-coverage` already failed at 1:50 and 1:100 (top-cover 0.102 against 0.055, grille 0.095 against 0.05, intake 0.074, feet 0.0137), because closed slabs counted their hidden faces.

## 3. Root fixes (catalog file, outside `volumes`)

| Member | Old | New |
|---|---|---|
| `rig.body.lengthM` | 0.31 | 0.098 (centre to the outlet face) |
| `anchors.foot-back`, `foot-front` footprint | 0.035 x 0.04 | 0.025 x 0.025 (contact face of one pad) |
| `materials` | 5 | + `steel-zinc` (bridge, motor stack, heater box) and `nichrome-coil`, both from `incropera-2018` |
| `thermal.regions.casing.areaM2` | 0.33 | 0.22: the C16 bound (factor 1.5). The model measures 0.197; see section 6 |
| `thermal.regions.intake.areaM2` | 0.05 | 0.037 (measured 0.0371, factor 1.35) |
| `thermal.regions.control.areaM2` | 0.0045 | 0.0058 (measured 0.00578, factor 1.29) |
| `provenance` | 4 references, old notes | + `retail-box-fan-heaters-2026`; notes rewritten in en and de for the real construction |
| `representations[50,100,200].geometry` | old drawings | redrawn in every view from the kit measures; ids kept where the part exists. Removed: `seam-lower`, `inlet-slot-*`, `dial-hub`, `handle-grip`, the front `top-cover`. New: `boss-*`, `dial-grip`, `cord-grommet`, `frame-edge`, `grille-opening`, and `dial` in the 1:100 front and plan |
| `representations[*].decisions` | old sentences | silhouette, retained, merged, omitted, thinElements and styles rewritten per scale, en and de |

`top-cover` (0.055), `grille` (0.05), `feet` (0.0085), every `heatW`, the emitters, poses, clips, summary and `dimensions` are unchanged. No test pins thermal values of this asset (only `model-glyph-edges` and `model-glyph-size` name it, for geometry).

## 4. What was modelled

| Scale | body | thermal | structure |
|---|---|---|---|
| 1:50 | 26 | 18 | 18 |
| 1:100 | 11 | 12 | 10 |
| 1:200 | 4 | 8 | 7 |

### 1:50 (feature 7.5 mm)

- **Body**: U-shell and top cover as hollow extrusions along the air path with the real 2.5 mm wall; front and rear frame as tapered rings with their openings; four bosses and rubber pads (revolves); cassette rim with eight slats; intake panel with seven slot holes; handle as one loft of 17 sections square to its centre line; dial (revolve) with grip bar on the knob bone; rocker; cord grommet. Inside: motor bridge, lamination stack, coil, fan hub, five pitched blades, shroud with round aperture, heater box (hollow), two coil rows, two brackets.
- **Thermal**: shell and cover each as two half shells with marked inward faces, frames, handle, bosses, cassette rim and slats (inward faces marked), intake panel, dial, grip, rocker, pads, and one outlet patch of 0.1 m for both emitters on the louvre tips.
- **Structure**: U-shell, cover band under the handle, both frames as rings, handle with two root joints, motor bridge and heater brackets with fixing joints, motor and heater masses, thermostat spindle with bush, bosses, pads as the two supports.

### 1:100 (feature 15 mm)

- **Body**: shell and cover as two closed masses between solid chamfered caps, outlet plate with four slats, plain intake plate, handle with a 15 mm bar and its opening, four foot stubs, plain dial.
- **Thermal**: references to these masses with the seam and built-in faces marked; outlet patch.
- **Structure**: hollow U-shell, cover band, handle with joints, one bridge with the motor mass, spindle with bush, feet as supports.

### 1:200 (feature 30 mm)

- **Body**: one rounded block on a recessed base, outlet block, handle bump.
- **Thermal**: the block divided at the cover seam, outlet, intake, dial and base as patches of true size, outlet patch.
- **Structure**: hollow U-shell, cover band, handle bump, spindle stub, motor mass, one support rail per foot pair.

Measured at the end (product check):

| Scale | IoU elevation / front / plan | areas against `areaM2` |
|---|---|---|
| 1:50 | 0.989 / 0.942 / 0.999 | casing 0.197 (10 %), top-cover 0.049 (10 %), grille 0.049 (1 %), intake 0.037 (0 %), control 0.0058 (0 %), feet 0.0084 (1 %) |
| 1:100 | 0.984 / 0.999 / 0.999 | casing 18 %, top-cover 20 %, grille 11 %, intake 8 %, control 16 %, feet 15 % (limit 30 %) |
| 1:200 | 1.000 / 0.997 / 1.000 | total 12 % (limit 30 %) |

## 5. What I saw in the final renders

- **1:50 body sheet** (`r3-sheet`, six views). Front: rounded housing with the chamfer edge, the cassette with eight slats, the coil rows and fan blades visible between them, the handle arch with a usable opening, dial and rocker on the right wall, two feet. Right side: chamfered frames, cover seam, dial with grip bar, rocker, louvre tips as teeth at the front, intake panel and grommet at the back. Back: slotted panel with motor, bridge and fan behind it. Top: handle bar across the cover, chamfered corners, dial. Three-quarter: reads at once as a box fan heater.
- **1:50 body cut**: bridge, motor, fan in its shroud, heater box with six coils, each fixed to the walls; nothing floats.
- **1:50 thermal cut**: casing, cover, grille, intake, control and feet each in its colour as thin shells; the outlet patch sits on the louvres. The housing is empty, because the asset declares no region for coils and motor.
- **1:50 structure**: an open frame (U-shell, cover band, two rings, handle), joints orange at handle roots, bridge and bracket ends and at the spindle, two masses, green pads. The side walls stay closed panels, which is the real monocoque.
- **1:100 body, structure**: same subject, simpler; four louvres; handle opening kept. Structure is the open U with band, bridge and mass.
- **1:200 body, thermal cut**: block on base with outlet and handle bump; six region colours present. The cut view of the solid thermal blocks shows moire where the ghost body coincides with them.
- **2D** (`r3-2d` on the candidate and the catalog, zoomed panels at all three scales, pose `off` at 1:50): the three views agree with the body; at `off` the grip bar stands upright, at `heating` it is turned 240 degrees.
- `volumes-render` overlay 1:50: red only at the louvre tips (strokes) and, in the front view, in the fan aperture.

## 6. Still imperfect

- **`casing` area.** The honest outer area of the 1:50 model is 0.197 m2. The declared 0.33 is 1.65 times that, so C16 forbids the correction; I applied 0.22 (the bound), which passes with 10 %. The request for 0.20 is in `area-review.md`.
- **No C31 flood in the product.** Inward faces of the thermal shells carry `sectionFaces` as a stand-in, as in the kettle, MVHR and heat-pump entries of `tool-requests.md`. The undersides of the louvre slats are marked the same way (area decision R-B: the rear face is interior).
- **No heater core in the thermal view.** The asset's thermal model states that coils and motor are not regions; adding a hidden region would change the physical model, so the internals are in `body` and `structure` only. This leaves the `qa-visual.md` request for hidden heater solids open.
- **Front silhouette 0.942 at 1:50.** Slats and blades are texture solids and are left out of the measured silhouette, so the fan aperture reads as see-through against the filled grille of the drawing. In the model itself the slats and blades are there.
- **Generalisation.** Eight slats and seven slots stand for about twice as many real ones (7.5 mm feature). The supply cord is not modelled (it would leave the declared envelope); only its grommet is.
- **Line views.** At 1:50 the 4 mm cassette rim and 6 mm intake panel are thinner than the pen and print as black strips; the plan handle shows in the solid views only.
- **Not run.** The catalog-wide test projects (`repo run ...:test`) were not run by me; only the asset validation and the volume check above.
