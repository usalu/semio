# Round 3: appliances/circulation-pump

State: finished and spliced on 2026-11-20. No earlier round-3 work existed for this asset (only the round-1 script `author-11-circulation-pump.py`), so it was rebuilt from the roots. Everything below describes the catalog file as it is now.

Scripts in the ticket folder:
- `r3-circulation-pump-kit.py`: the measures of the real pump and the shared profiles (volute sections, neck, stator, gland, hexagon).
- `r3-circulation-pump-model.py`: every solid of every scale and list, stated by hand. Writes `volumes/appliances/circulation-pump.json`.
- `r3-circulation-pump-roots.py` with `r3-circulation-pump-texts.json`: the root fixes and all nine 2D drawings (3 scales x 3 views), drawn from the same measures as each scale's body. Writes the candidate `🗑️generated/r3-circulation-pump/🔣️.json`. Idempotent.
- `r3-circulation-pump-apply.py`: writes the candidate's root members into the catalog in the file's formatting (copy of the duct-fan apply script).
- `r3-circulation-pump-print.ts <asset file>`: print suitability per scale (why `r3-2d.ts` can say "no suitable representation").

To resume, run in order: `python3 r3-circulation-pump-model.py`, `python3 r3-circulation-pump-roots.py`, `$BUN r3-validate.ts --file 🗑️generated/r3-circulation-pump/🔣️.json --volumes`, `python3 r3-circulation-pump-apply.py`, `python3 volumes-insert.py volumes/appliances/circulation-pump.json <catalog>`.

## 1. Reference

| Quantity | Real value | Source | Model |
|---|---|---|---|
| Class | Wet-rotor (glandless) inline heating circulator DN 25, 6 m head, installation length 180 mm | EN 16297-1; Grundfos UPS 25-60 180 | Same |
| Port to port | 180 mm between the faces of the thread ends | Grundfos UPS 25-60 180 / N 180 data sheets (products 96281483, 96913085, 96913058) | 180 mm |
| Connection | G 1 1/2 (ISO 228-1), thread major diameter 47.8 mm; union nut about 52 mm across flats, 60 mm across corners; Rp 1 or 28 mm solder tail | Grundfos data sheet; ISO 228-1; product knowledge of unions | Thread end 47.8 mm, nut 52 / 60 mm x 19 mm, copper tail 28 x 1.5 mm |
| Seen along the shaft | 75 mm to the terminal-box side, 51 mm to the other side of the pump axis | Grundfos dimensional sketch (values 75 / 51; one variant 73) | Top of the terminal box 75 mm above, lowest point of the volute 51 mm below the shaft: height 126 mm |
| Along the shaft | 32 mm behind and 102 mm in front of the pipe axis | Grundfos dimensional sketch (32 / 102) | Back of the housing 32 mm behind, sight glass 102 mm in front: depth 134 mm |
| Mass | 2.5 to 2.9 kg net (Grundfos N 180 variants), about 2.8 kg (Wilo Star-RS 25/6 RG) | Grundfos and Wilo product pages | 2.77 kg kept |
| Power | 50 to 60 W for the old three-speed UPS; about 45 W at full speed and 3 to 5 W at the lowest stage for an EEI 0.20 circulator | Grundfos data sheet; EU 622/2012 | 45 W / 5 W kept |
| Construction | Cast housing (iron or stainless) with a volute chamber, two in-line port necks ending in threads, a square bolting flange; stator housing with a square flange, four M6 socket screws; terminal box on the side of the stator can with lid and cable gland; composite impeller on the rotor shaft in the can, two bearings | Grundfos data sheet (materials), product knowledge | All of it modelled, see section 4 |
| Sight glass with indicator | The UPS has a name plate with a vent screw at the motor front, no window | Product knowledge | Kept as the asset's declared simplification (rig bone `rotor`, three clips depend on it); the notes say so |

Sources: [Grundfos UPS 25-60 N 180 (Lenntech data page)](https://www.lenntech.com/grundfos/UPSB1/96913085/UPS-25-60-N-180.html), [Grundfos UPS 25-60 N 180 data sheet 96913085](https://www.lenntech.com/uploads/grundfos/96913085/Grundfos_UPS-25-60-N-180.pdf), [data sheet 96913058](https://www.lenntech.com/uploads/grundfos/96913058/Grundfos_UPS-25-60-N-180.pdf), [Grundfos UPS range table](https://magazines.grundfos.com/Grundfos/gpa/home-and-garden-range/ups/), [Wilo Star-RS 25/6 RG](https://wilo.com/ie/en/Solutions-Finder/Selection-and-configuration/Heating-replacement-guide/en/wilo/wilo-star-rs/star-rs-25-6-rg). The sketch labels (which of 75 / 51 / 32 / 102 is which) were not readable in the search results; the assignment above follows the Grundfos column order (two widths, two depths) and the only arrangement in which a 180 mm pump with a terminal box fits.

## 2. Audit before round 3

- **Dimensions.** Height 0.150 m against 0.126 m real (volute 115 mm against about 100 mm, terminal box reaching 92.5 mm above the shaft against 75 mm). Depth 0.130 m against 0.134 m. Width 0.18 m was the installation length, but the drawing squeezed the housing, both unions and both pipe stubs into those 180 mm: the real 180 mm is the housing alone, thread face to thread face.
- **Unions.** Nuts 42 mm high and 23.5 mm long on a G 1 1/2 thread of 47.8 mm: a nut smaller than its own thread. Round stepped revolves, no hexagon.
- **Housing.** A plain chamfered disc of 115 mm about the shaft, no port necks (the unions grew straight out of the disc), no flange, no volute offset. qa-visual: "volute is a plain cylinder, not a spiral".
- **Motor.** A cylinder with no flange and no screws; nothing held it to the housing.
- **Rig.** `housing` started at the shaft and ran 115 mm to the right, past the right support, so the structural view showed half a beam. `terminal` lay horizontally (angle 0) although the box stands up. The pointer was drawn across its bone.
- **Structure.** One 37 mm bar through everything, the motor head and the terminal box as members by reference, supports as two cylinders inside the stubs.
- **Thermal.** The plume emitter was an open loft skin hovering in the motor (qa-visual: "sliver fins"). `rotor-window` failed the check on the catalog as found (both faces of the pane counted, 0.0046 against 0.0022 m2): the asset was red before this round.
- **2D.** Front and plan were constructed block drawings (volute and motor as rounded rectangles, a 5 mm "window" slab on the side), not views of one body. 1:200 drew the pipe over the motor disc.
- **Coarser scales.** qa-visual: thermal 1:200 was the 1:100 list.

## 3. Root fixes (catalog outside `volumes`)

| Member | Old | New |
|---|---|---|
| `dimensions` | 0.15 x 0.18 x 0.13 m (h x w x d) | 0.126 x 0.22 x 0.134 m; mass unchanged. Width: 180 mm housing plus, per side, 7 mm of nut and 13 mm of tail beyond the thread face |
| `rig.housing` | x 0, y 0.0575, length 0.115 | x -0.104, y 0.051, length 0.208: from the left to the right support along the pipe axis; mass at the middle |
| `rig.motor` | x 0, length 0.09 | x 0.104 (on the shaft), length 0.042 (the head's radius in the view plane) |
| `rig.terminal` | y 0.05, angle 0, length 0.0425 | y 0.040, angle 90, length 0.035: stands on the can, tip at the box top |
| `rig.rotor` | length 0.025 | 0.024; pointer now drawn along the bone |
| `anchors.pipe-left / pipe-right` | x -0.085 / 0.085, z 0.04 | bone x 0 / 0.208 (world -0.104 / 0.104, on the tails), z 0.035 |
| `anchors.motor-top` | (0, 0.0425) | bone tip (0.035, 0), z -0.028: centre of the lid |
| `materials` | 6 | + `steel` (zinc-plated screws) |
| `thermal.regions.casing.areaM2` | 0.048 | 0.053: measured 1:50 outer area 0.0526 (factor 1.10). Hand check: drum side 0.0136, back 0.0072, flange about 0.012, necks 0.011, nuts 0.016, tails 0.002, less the buried joints |
| `thermal.regions.motor.areaM2` | 0.034 | 0.035 (measured 0.0350) |
| `thermal.regions.terminal-box.areaM2` | 0.0128 | 0.019 (measured 0.0194, factor 1.48, inside the C16 limit of 1.5): the old value was a bare 60 x 40 x 30 block; lid, shell, saddle and gland are 0.019 |
| `thermal.regions.rotor-window.areaM2` | 0.0022 | 0.0025: face 0.0022 + edge 0.0003; the inner face looks into the sealed indicator chamber and is marked `sectionFaces` on the thermal reference (C25 b) |
| `characteristicLengthM` | casing 0.115, motor 0.09 | 0.102, 0.084 |
| `structure.assumptions` | 1 | 2: beam between the tails 208 mm apart; the motor's lever arm is lateral to the view plane |
| `provenance` | 4 references | + `grundfos-ups-25-60-180`; notes rewritten (en, de) |
| `summary`, casing and emitter titles, bone and anchor titles | | rewritten |
| `representations.50/100/200.geometry` | see audit | all nine drawings redrawn (E1) |
| `representations.*.decisions`, `500.reason` | | rewritten (en, de); 1:500 is 0.44 x 0.25 mm |

Unchanged: poses, clips, operating levels, heat values, emitter data, 1:500 unavailable.

## 4. What was modelled

Frame: x along the pipe, y up, z along the shaft; the motor points to -z, at the viewer of the elevation. Pipe axis at z 0.035.

### 1:50: body 22 (25 with the screw repeat), thermal 19, structure 18

- **Body.**
  - Housing: the volute as a loft of seven circular sections about a centre 3 mm below and 1.5 mm beside the shaft (96 mm drum, small front chamfer, back rounded over 10 mm down to a flat of 66 mm); square bolting flange 84 x 84 x 7 mm with 10 mm corners; two port necks as revolves tapering 43 to 39 mm, fillet, thread end 47.8 mm ending 90 mm from the shaft.
  - Unions: hexagon nuts 52 mm across flats from 78 to 97 mm; hollow copper tails 28 x 1.5 mm to 110 mm.
  - Motor: aluminium flange 84 x 84 x 8 mm; four M6 socket heads 10 x 6 mm on a 68 mm square; stator can 84 mm with a 4 mm front rounding; bezel ring 61 mm; sight glass 53 x 2 mm set 0.5 mm back; behind it dial, hub, pointer and tail on the `rotor` bone (texture parts, C10).
  - Terminal box: saddle on the can, lower shell 60 x 66 x 20 mm, lid 62 x 68 x 14 mm, cable gland with dome nut on the left wall, lamp on the lid front.
- **Thermal.** Every region is its body surface in its single layer (references). The tails are own solids that count the outer wall only, their ends are cuts through the pipe. The plume outlet is a 1 mm patch on the lid top (D2).
- **Structure.** The real load path instead of the body: volute as a cup with a 5 mm wall, two port tubes with their bore, housing flange and motor flange as plates with the 78 mm bore, the 3 mm stator shell as an open tube, four screws as the motor joint, the unions as joints, the floor tray of the terminal box on its saddle. In the open shells: the rotor as shaft 8 mm with rotor pack 34 mm and impeller 62 mm, two bearings. Masses of housing and motor. Pin = clamp collar on the left tail, roller = pin across the pipe axis at the right tail.

### 1:100: body 12, thermal 13, structure 13

- **Body.** Volute with one chamfer; the two flanges as one 15 mm plate; plain necks 41 mm; hexagon prisms; solid tails; can with a 4 mm chamfer and the sight glass as a flush 2 mm disc; one terminal block; plain gland.
- **Thermal.** References, tails without their cut ends, pane without its inner face, smaller plume patch.
- **Structure.** Volute cup with a 15 mm wall, necks, unions as joints, stator as a 15 mm tube on the bored flange plate, box base, 15 mm rotor with pack and impeller, two masses, collar and roller.

### 1:200: body 4, thermal 6, structure 7

- **Body.** Volute drum with the flange in it, motor drum, one 44 mm pipe bar through the housing (ends are cuts), terminal block.
- **Thermal.** The four masses plus a flush sight-glass disc and the plume patch.
- **Structure.** A 30 mm beam between the supports, the stator shell, the rotor shaft, a post for the box, one mass at the common centre, collar and roller.

1:500 stays unavailable. Each scale has three bilingual decisions.

## 5. Checks

- `$BUN volumes-check.ts appliances/circulation-pump --product`: **PASS**.
- `$BUN r3-validate.ts appliances/circulation-pump --catalog-only --volumes`: **VALID**, schema 0, validateAsset 0, checkVolumes 0.
- Print suitability: suitable at 1:50, 1:100, 1:200 (smallest feature 0.23 / 0.22 / 0.21 mm).
- Silhouette IoU (elevation / front / plan): 1:50 0.994 / 0.991 / 0.993; 1:100 0.999 / 0.974 / 0.989; 1:200 1.000 / 0.996 / 1.000. p95 at most 3.75 mm.
- Area deviation: 1:50 casing 0.7 %, motor 0.1 %, terminal box 2.2 %, window 0.5 %; 1:100 at most 7.2 %; 1:200 total 14.4 %.
- `volumes-status.py`: `aaa+ aaa+ aaa+ uuu+ pass`, spliced.

## 6. What I saw in the final renders

- **1:50 body, six-view sheet and a rear three-quarter.** Reads at once as an inline circulator: pipe run with necks swelling into the volute, hexagon nuts, open copper tube ends; square flange with four screw heads at the corners, round can, bezel and dark glass; box with lid, gland to the left, lamp. From behind: smooth domed housing back, flange corners showing past the drum. Top: can pointing to the viewer under the lid. Bones: housing from support to support, terminal upright.
- **1:100 body.** Same pump with fewer parts; the sight glass sits flush in the chamfered can front; block and gland right.
- **1:200 body.** Disc, drum, bar and block; still an inline pump with a motor across the pipe.
- **Thermal, cut sheets at all three scales.** Housing, motor, box and glass in their colours, nothing poking out; the patch is a thin strip on the lid; at 1:200 the glass is the yellow disc on the motor front.
- **Structure at all three scales.** 1:50 and 1:100: looking into the open stator shell the rotor pack, shaft and bearing are visible; unions orange; collar and roller green. 1:200: beam, shell, shaft, post, one magenta mass.
- **2D, zoomed line and tone panels of all three scales, and 1:50 at `run-high` t 0.6.** The three views agree with the model; the pointer turns about the hub inside the glass; at 1:200 the disc now covers the pipe.

## 7. Still imperfect

- **Faceted circles in the sheets.** The 84 and 96 mm cylinders show about 15 sides. That is the tessellation quality, not the authoring: `QUALITY_FOR` floors the chord at 1 mm, and the product's own job quality is coarser still (0.1 mm on paper). Revolves and lofts have no segment field, so an author cannot change it. Not filed as a tool request because the product setting is deliberate.
- **Sight glass.** A real UPS has no window. It is kept because the rig, three clips and the 2D indicator are built on it; the provenance notes name it as a simplification. Removing it is a content decision beyond this asset's geometry.
- **Volute.** The spiral is an offset drum, the usual casting simplification; the cutwater and the cast flow arrow are not modelled (under 7.5 mm).
- **Motor bone.** The motor and rotor run along z, which a bone in the view plane cannot express; their bones carry the radius and the pointer length instead.
- **Masses at 1:50 and 1:100** sit inside the volute cup and the rotor pack and are therefore not visible from outside.
- **1:200 front view** draws the 3 mm step between can and volute flush (the depiction otherwise turns it into a slanted edge); the decisions say so.
- **Reference labels.** See the note under the sources: the four Grundfos sketch values were assigned by reasoning, not read off a labelled drawing.
