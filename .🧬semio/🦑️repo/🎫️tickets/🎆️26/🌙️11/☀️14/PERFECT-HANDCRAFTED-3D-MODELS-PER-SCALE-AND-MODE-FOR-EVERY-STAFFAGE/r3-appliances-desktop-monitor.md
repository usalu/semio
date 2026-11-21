# Round 3 — appliances/desktop-monitor

State: finished and spliced (2026-11-20). No earlier round 3 work existed for this asset (only the round 1 `author-11-desktop-monitor.py`), so it was rebuilt from the roots. Everything below describes the catalog file as it is now.

Scripts in the ticket folder:
- `r3-desktop-monitor-kit.py`: measures, bone frames, solid helpers, corrected rig.
- `-body.py`: the section tables and every body solid of every scale, by hand.
- `-lists.py`: thermal, structure and volume decisions.
- `-roots.py`: anchors, plume direction, reference, notes, structural assumption, representation decisions.
- `-draw.py`: the 2D of every scale and view (E1), drawn from the same section tables.
- `-build.py`: writes the candidate to `🗑️generated/r3-desktop-monitor/` and `volumes/adhoc/`.
- `-apply.py`: writes the changed root members in the file's own formatting and copies the source.
- `-zoom.ts`: rasterises a cropped 2D SVG large enough to judge the strokes.

Running build and then apply a second time changes nothing (`members: none`). The scratch candidate and the adhoc renders were deleted. The final renders are kept under `🗑️generated/r3-sheet/appliances/`, `🗑️generated/r3-2d/appliances/` and `🗑️generated/volumes-tools/render/appliances/`.

## 1. Reference

| Quantity | Real subject | Source | Model |
|---|---|---|---|
| Class | 23.8 to 24 inch 16:9 IPS LED monitor on a tilt-only, height-fixed stand | Dell SE2422H user's guide; asset notes | same |
| With stand | 420.3 H x 552.6 W x 178.8 D mm | Dell SE2422H specifications (physical characteristics) | 411 x 540 x 210 mm (asset `dimensions`, kept: class estimate within 3 % in height, see below) |
| Panel without stand | 331.6 x 552.6 x 49.7 mm deep | Dell SE2422H | 323 x 540 mm, 9.5 mm edge, 36 mm total depth to the back of the hump |
| Active area | 23.8 inch: 527.0 x 296.5 mm (0.2745 mm pitch, 1920 x 1080) | panel datasheets of the class (product knowledge) | glass 527 x 296.5 mm; bezel 6.5 mm at the sides and top, 19 mm chin |
| Stand | 173.9 H x 240.4 W x 178.8 D mm, 0.33 kg | Dell SE2422H | base plate 250 x 210 mm and 10 mm thick on 2 mm pads; riser 62 x 22 mm |
| Tilt | -5 to +21 degrees | Dell SE2422H | rig revolute -5..+20 (kept), default 5 degrees back |
| Mass | 3.77 kg with stand and cables, 3.18 kg without stand | Dell SE2422H | 3.6 kg (kept): panel 3.0, neck 0.2, base 0.4 (was 2.4 / 0.35 / 0.85) |
| Construction | bezel frame and LCD module in a steel frame, rear cover sloping to an electronics hump (PSU, boards) with vent slots on its upper slope and ports underneath, VESA 100 field with the quick-release bracket of the riser, tilt hinge in the riser head, steel-cored base plate on rubber pads | product knowledge; asset notes (16 W at the rear housing) | all modelled at 1:50 except ports, menu buttons, cable and VESA screws |
| Heat | 25 W on, 9 W front, 16 W rear housing | Wilkins and Hosni 2011 (asset) | unchanged |

Sources: Dell SE2422H user's guide, specifications pages (manualslib copy of the Dell guide, pages 13 and 16; Dell product listing). Added to `provenance.references` as `dell-se2422h-2021`.

The asset `dimensions` (0.411 / 0.211 / 0.54 m, 3.6 kg) are a rounded class estimate, not one product. They lie within 2 to 3 % of the reference in height and width, and the 0.21 m base is deeper than the SE2422H's 0.179 m. There is no cited single product to correct them against, so they are kept. The notes say so.

## 2. Audit of the state before round 3

- **Rig.**
  - The `neck` bone lay horizontal (`angleDeg` 0). It ran 0.243 m forward along the desk, out past the front of the base plate, while the riser it carries stands upright. The bone was in the wrong place.
  - The `panel` bone was 0.323 m long from the hinge. It ended 0.17 m above the top edge of the panel, so the rig overlay showed it far outside the model.
  - The `base` bone started at the plate centre (x 0) and ran 0.096 m past the front edge.
  - Mass split: panel 2.4 kg against the reference 3.18 kg; base 0.85 kg against a whole stand of 0.33 kg.
- **Emitter.** `rear-plume` had `directionDeg` 90 relative to the 95-degree panel bone, which points backwards and slightly down. The transport solver ignores the direction for plumes, but the glyph and any reader do not. Its anchor `housing-top` lay on the flat back of the housing, not at a vent.
- **2D drawings.**
  - The front view placed the panel at the heights of the bone axis, 3.4 mm low.
  - The hinge was drawn as a 24 mm ball beside the riser.
  - In the side view the glass was a strip on the face; in the real monitor it is recessed behind the bezel.
  - The plan drew vents as five short ticks across the depth.
  - At 1:200 the stand was one T-shaped path, with the neck widened to 60 mm deep.
  - The housing outline did not follow any real rear cover.
- **Solids** (qa-visual: B/B/B at 1:50 and 1:100, C/B/B at 1:200).
  - 1:50:
    - The hump was a stepped, faceted loft in ring terraces.
    - The 0.54 x 0.323 m bezel slab was one plate, with a glass sheet standing 1.4 mm proud of it.
    - The hinge could not be seen.
    - The foot was a plain plate on box feet.
  - 1:200: the panel was a squircle blob with 60 mm corner radii, and the base a bowed wedge.
  - Thermal: front and housing counted both faces of the module joint. The plume was a flat box inside the hump.
  - Structure:
    - The panel member was the bezel slab itself, i.e. the body.
    - The mass was a plate-sized disc.
    - The 1:200 support was a warped wedge.

## 3. Root fixes (catalog file, outside `volumes`)

| Member | Old | New |
|---|---|---|
| `rig.base` | x 0, length 0.21, 0.85 kg, com (0.01, 0.006) | x -0.096 (rear edge of the plate; world unchanged), length 0.21, 0.4 kg, com (0.105, 0.006) |
| `rig.neck` | x -0.06, y 0.012, angle 0 (horizontal), 0.35 kg, com (0, 0.12) | x 0.036, y 0.012, angle 90 (upright), 0.2 kg, com (0.12, 0.008) |
| `rig.panel` | x 0, y 0.243, angle 95, length 0.323, 2.4 kg, com (-0.01, -0.025) | x 0.243, y 0, angle 5 (same 95 degrees in the world, hinge unchanged), length 0.153 (hinge to top edge), 3.0 kg, com (-0.01, -0.024) |
| `anchors.base-contact` | x 0.01 | x 0.106 (same world point) |
| `anchors.housing-top` | (0.06, -0.004), "warm air above the rear housing" | (0.10716, -0.01746) on the vent slots, "vent slots on the upper slope of the rear housing" |
| `thermal.emitters.rear-plume.directionDeg` | 90 (backwards) | -5 (world up on the 95-degree panel) |
| `provenance` | 5 references, notes | + `dell-se2422h-2021`; notes rewritten (construction, bezel and active area, hump, VESA bracket, riser and plate, tilt range, mass split, comparison with the reference) |
| `structure.assumptions` | panel centre 0.05 m behind the base centre | panel hangs on the hinge, centre 0.025 m in front of the hinge and 0.044 m behind the base centre |
| `representations.50/100/200.geometry` | 16 / 13 / 6 shapes | redrawn under E1: 1:50 elevation 6, front 7, plan 7; 1:100 4 / 4 / 5; 1:200 3 / 3 / 3 |
| `representations.*.decisions` | old texts | rewritten (all eight keys, en and de) |

Unchanged: `dimensions`, `summary`, materials, poses, clips, thermal regions and every `areaM2`.

The shape ids that left the drawings:
- `hinge`: hidden inside the riser head and the bracket in every true view.
- `screen-glass` in the elevation: hidden by the bezel.
- `vents-1..5` in the plan: replaced by one `vents` stroke.
- `panel-edge` in the plan: now `panel-slab`, the solid's id.
- `stand` at 1:200: now `base-plate` and `neck`.

New shape ids: `mount-plate` (1:50 elevation and plan), `vents`.

## 4. What was modelled

All coordinates are bone-local, stated in section tables in `-body.py`. The same tables drive the 2D.

### 1:50: body 11 solids (16 with repeats), thermal 12, structure 13

**Body**
- **Bezel frame:** an extrude 0.54 x 0.323 m with 5 mm corners, 9.5 mm deep, around the glass opening.
- **Cover glass:** glazing, 527 x 296.5 mm, set 0.7 mm back and filling the depth of the LCD module.
- **Power light:** a lens in the chin.
- **Rear cover:** one loft of nine superelliptic sections.
  - It runs from the full panel edge (exponent 40, which matches the 5 mm corners) over the upper slope to the flat 0.35 x 0.17 m back of the hump, 36 mm behind the screen.
  - The exponent is constant at 8 behind the edge, so no ring steps show.
- **Vent slots:** four texture slots across the upper slope, sunk into the cover.
- **Bracket:** the quick-release bracket (90 x 80 x 8 mm) on the VESA field.
- **Hinge barrel:** 20 mm, at the riser head.
- **Riser:** a loft of eight rounded sections, 62 x 22 mm, with a flared foot sunk into the plate and a rounded head.
- **Base plate:** a loft of six sections, 250 x 210 mm, exponent 6 in plan, with a bevelled top edge.
- **Pads:** four rubber pads, 22 x 22 x 2 mm.

**Thermal**
- **Front stack:** the 4 mm LCD stack of the `front-stack` layer, as a bezel ring plus the recessed glass. Its back faces are `sectionFaces +v`.
- **Casing rim:** the edge of the casing behind the front stack, as region `housing` with `sectionFaces -v`. In a real monitor the rear cover forms the side walls.
- **Power light** by reference.
- **Rear cover** by reference, `sectionFaces -v`. The faces where the module meets the cover are the sealed joint of one casing (C25 b).
- **Plume outlet:** a 0.30 m patch lying on the vent band.
- **Stand:** bracket, hinge, riser, plate and pads, by reference.

**Structure**
- Steel core of the plate.
- Riser core (44 x 10 mm).
- Foot screw boss and hinge, as joints.
- Bracket (by reference), VESA standoff, and the steel frame of the LCD module as a ring with a horizontal rail and a stile crossing at the VESA field, as members.
- Masses at the three rig centres of mass.
- One 3 mm contact pad over the 0.19 x 0.20 m footprint.

### 1:100: body 5, thermal 7, structure 9

**Body**
- **Bezel frame:** 10.5 mm deep, with its sides and top widened to 15 mm and the chin to 22 mm so the glass edge reads at one feature.
- **Glass.**
- **Rear cover:** five sections.
- **Riser:** four sections; the hinge and bracket merge into its rounded head.
- **Base plate:** standing on the desk, with the pads merged.

**Thermal:** as at 1:50, without the light, bracket, hinge and pads.

**Structure**
- The base plate itself as member.
- Riser core.
- Hinge joint cylinder.
- VESA standoff with the bracket.
- Module frame with its rail.
- Two masses.
- Contact pad, 12.4 mm (the drawn plate thickness sets the floor).

### 1:200: body 3, thermal 6, structure 6

**Body**
- **Panel:** one loft, a 12.5 mm slab stepping to a three-section hump.
- **Riser:** 34 mm deep (one feature).
- **Base plate:** 12 mm.

**Thermal:** the slab front stack, the casing rim and the hump as their own lofts, which together are exactly the body panel; the plume outlet; riser and plate by reference.

**Structure**
- Plate and riser as members (by reference; they are the members).
- Module frame generalised to a 30 mm rail and stile.
- Panel mass.
- Contact pad.

The 1:500 scale stays unavailable, as declared.

## 5. Checks (final, on the catalog asset)

- `r3-validate.ts appliances/desktop-monitor --catalog-only --volumes`: `VALID ... schema 0, validateAsset 0, checkVolumes 0`. The normal form is also VALID. The catalog `volumes` equal `volumes/appliances/desktop-monitor.json`.
- `volumes-check.ts appliances/desktop-monitor --product`: `PASS (3.4 s)`.

Silhouette IoU (elevation / front / plan) and p95 boundary distance:

| Scale | Elevation | Front | Plan | p95 |
|---|---|---|---|---|
| 1:50 | 0.997 | 1.000 | 0.998 | ≤ 1.9 mm |
| 1:100 | 0.995 | 1.000 | 0.999 | 0 |
| 1:200 | 1.000 | 0.999 | 0.992 | 7.5 mm (limit 120) |

Thermal area deviation:

| Scale | panel-front | housing | stand | total |
|---|---|---|---|---|
| 1:50 | 0.088 | 0.135 | 0.044 | — |
| 1:100 | 0.087 | 0.130 | 0.089 | — |
| 1:200 | — | — | — | 0.096 |

The other rules:
- Bounds are within 1.1 mm.
- Monotonic: solid and triangle counts drop at every step.
- Cross-scale IoU is 1.0 in every view.

## 6. What the renders show (opened with the Read tool)

**Body 1:50 (`r3-sheet`, six views, before and after the fixes; right side and rear three-quarter alone)**
- **Front:** a thin black bezel around the dark glass, a 19 mm chin with the power light at the lower corner, and the slim riser flaring into the rounded base plate. It reads as a current 24-inch office monitor.
- **Right side:** the panel leans back by 5 degrees. The 9.5 mm edge widens over the upper slope into the hump. The bracket sits on the hump back, with the riser behind it rising to the hinge head. The plate has its bevel and the two pads.
  - The first state had a stepped hump outline and vent ridges standing proud. Both were fixed: constant exponents, and the slots sunk in.
- **Back and rear three-quarter:** the rear cover is a single smooth form. Four vent slots lie across the upper slope, the bracket sits on the VESA field, the riser head carries the hinge, and the flared foot meets the plate.
- **Top:** the rounded plate, the riser foot behind the panel, the hump narrowing backwards, and the slanted glass.

**Body 1:100 and 1:200**
- 1:100: the same monitor with a wider bezel and a simpler head.
- 1:200: one dark panel mass with its hump and a column on the plate. It is recognisable at a glance and no longer a blob.

**Structure 1:50, 1:100, 1:200**
- 1:50 and 1:100: the module frame with rail and stile, the VESA standoff and bracket, the riser core with the orange hinge, and the steel plate core on the green contact pad. The magenta masses sit inside.
- 1:200: a cross of rail and stile on the riser, with the panel mass at the hinge.
- The first 1:100/1:200 state showed the support pad z-fighting with the plate top. It was raised by 0.2 mm.

**Thermal (cut) 1:50 and 1:100**
- The red front stack, the green casing rim and rear cover, and the purple stand.
- No shell pokes out of the body.

**2D**
- Checked: `r3-2d` at 1:50, 1:100 and 1:200 (candidate and catalog), plus 1:100 in the `standby` pose.
- Light-line and light-solid SVGs were cropped and enlarged (`-zoom.ts`) for 1:50 elevation, front and plan, and for 1:100 and 1:200 elevation and plan.
- The first plan drew the vents as a 0.2 mm band that merged with the bracket. It is now one stroke.
- The first elevation outlined the hidden hinge and bracket over the riser. Those are now removed, or drawn under the riser.
- The first 1:200 plan missed the slab's top corners. That outline is now drawn.

**Overlays (`volumes-render` 1:50 and 1:200):** green almost everywhere; only pixel slivers at the plan corners.

**Other pose and clip frame:** `standby-cool-down` at t = 300 s (right side, 1:50) and pose `standby` (2D 1:100). They are identical to `on`, as they must be: no pose moves a bone.

## 7. Still imperfect

- **Housing area.** The `housing` measures 0.190 m² against the declared 0.22 (-13.5 %, within the 15 % of C15).
  - The honest 36 mm deep cover of this asset has about 0.19 m² of outer surface. The reviewers' figure assumes a deeper hump.
  - The reference product is 49.7 mm deep; this asset keeps its 40 mm class note and its hinge position.
  - `areaM2` was not changed.
- **Thermal split at 1:50.** The thermal model splits the 9.5 mm edge at the 4 mm depth of the `front-stack` layer: front stack in front, casing rim (housing) behind. The body draws the whole edge as the black bezel frame, because a 4 mm body solid would be below the 7.5 mm feature.
- **Omitted at 1:50.** Ports under the hump, menu buttons, the power cable and VESA screws are not modelled (hidden, or below 7.5 mm). The power supply is not a separate hidden region: the asset declares none, and the housing region carries its 16 W.
- **Glass in the line views.** The product's light-line views draw no outline for the glazing and detail fills (the glass and the power light in the front view). This is the product's behaviour and was the same before.
- **Vents in the plan.** The vent slots are 3 mm apart, below what 1:50 separates, so the plan draws their band as one detail stroke.
- **Rig not exercised.** No pose or clip uses the tilt range. The corrected rig (neck upright, panel 5 degrees relative) was only exercised in the default pose and in the `standby` frames.
- **Repo tests.** The repo test suites were not run; only the checks of the brief.

## Round 5 handcraft (2026-11-21)

Process (all assets of E03; final `r3-validate.ts <asset> --catalog-only --volumes` VALID for all five after the splice): `r5-appliances-render.sh` rendered every list at every scale, every pose and both clips at 1:50 (t 0.5 s) and the 2D of every drawn scale into `🗑️generated/r5-<slug>/before` and `/after`; `r5-seated-montage.ts` laid them out for reading; every sheet named below was opened with the Read tool. `r5-appliances-measure.ts` measured the body extents per scale in the default pose (numbers below). Grades: body / thermal / structure.

### Reference and model (default pose `on`)

| quantity | reference (Dell SE2422H) | model | deviation |
|---|---|---|---|
| height with stand | 420.3 mm | 410.9 mm (1:50, 1:100, 1:200), 413.9 mm (1:500) | -2.2 % (1:500 -1.5 %) |
| width | 552.6 mm | 540 mm | -2.3 % |
| depth with stand | 178.8 mm | 210 mm (asset `dimensions`, base plate 250 x 210 mm) | +17 % (defect, see open points) |
| panel head height | 331.6 mm | 322.6 mm | -2.7 % |
| active area 23.8 in | 527.0 x 296.5 mm | 527.0 x 296.1 mm | 0 / -0.1 % |
| panel depth | 49.7 mm | 46.4 mm (housing) + glass, 37.6 mm slab | within the class |
| stand base | 240.4 x 178.8 mm | 250 x 210 mm | +4 % / +17 % |

### Grades

| scale | before | after |
|---|---|---|
| 1:50 | A- / A- / B+; catalog INVALID (print check: 2 mm pads 0.190 mm at 1:20) | A- / A- / B+; VALID |
| 1:100 | A- / A- / B+ | unchanged |
| 1:200 | A- / A- / B+ | unchanged |
| 1:500 | B / B- / C+ (bulk copy: thermal lofts and structure copied from 1:200, lopsided base outline, one-sentence decision) | A- / A- / B+ |
| poses `standby`, `off`, clips `power-on-warm-up`, `standby-cool-down` (t 0.5) | A (tilt hinge at the head of the riser) | unchanged |

Seen before: body at all scales, structure and thermal cut at all scales, the three poses and both clip frames, 2D 1:50, 1:100, 1:200. After: 1:50 body and 2D, 1:500 body, thermal cut and structure.

### Changes

- Root, 1:50 (`r5-desktop-monitor-pads.py`, candidate validated before writing): the four 2 mm rubber pads were below the 7.5 mm feature and below 0.2 mm at the 1:20 end of the band, which refused the catalog asset (four `PRINT range-end-feature` lines). They now merge into the base plate in 2D (foot shapes of elevation and front removed, plate drawn down to the desk) and in 3D (pad solids and their thermal references removed, plate loft to the desk); the steel core, its mass and the contact pad are now 7.5 mm (the pad drawing no longer lowers the feature). The 1:50 representation decisions (silhouette, retained, thinElements, styles) and the volume decisions say so in en and de.
- 1:500 (`r5-desktop-monitor-500.py`): authored anew. Body: panel prism of the true head outline 540 x 323 mm with the edge thickening from 14 to 35 mm, tapered riser, base plate 250 x 210 mm with even chamfers (was a trapezoid chamfered at the back only). Thermal: 2 mm front skin, the housing as the rest of the prism, plume outlet flush on the back, stand by reference (the 1:200 lofts copied into 1:500 are gone). Structure: plate and riser as members, the LCD frame as one plate across the VESA field, the tilt hinge as a joint, panel mass, contact pad. Decisions written for body, thermal and structure.
- `volumes-check --product` PASS (9.7 s).

### Open points

- Depth 0.21 m against 0.1788 m of the SE2422H (+17 %) and base 250 x 210 against 240 x 179 mm: the asset declares a class estimate (round 3 kept it); correcting it changes `dimensions`, the base bone, the anchor, the 2D of three scales and the 3D of four. Not done in this round; it is the one measure outside 3 %.
- Structure 1:50 to 1:200: the LCD module frame as a ring with rail and stile is an idealisation of the steel back tray (B+).
