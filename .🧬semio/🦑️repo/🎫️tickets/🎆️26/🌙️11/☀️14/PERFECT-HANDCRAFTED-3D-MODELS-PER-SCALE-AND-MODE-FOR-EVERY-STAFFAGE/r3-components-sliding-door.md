# Round 3 — components/sliding-door (2026-11-20)

Fresh start: no earlier round-3 scripts or report existed for this asset. At the start the catalog `volumes` equalled `T/volumes/components/sliding-door.json`, `r3-validate` said VALID, and `volumes-check --product` failed on 1:50 views-disagree (z 9 mm) and three 1:50 thermal areas. The 1:50 drawing was also **unsuitable for print**: the product refused to draw it (`r3-2d` said "no suitable representation at 1:50", because the plan handle was 2.5 mm from the stile, a 0.05 mm paper gap). No validator reported this.

Scripts in T, all re-runnable:
- `r3-sliding-door-kit.py`: dimensions and profile writers.
- `r3-sliding-door-volumes.py`: every solid of every scale.
- `r3-sliding-door-roots.py`: root fixes and the 2D of all scales. `--apply` writes the catalog.
- `r3-sliding-door-layout.py`: member splice in the catalog layout.
- `r3-sliding-door-suit.ts`: print suitability per scale.
- `r3-sliding-door-crop.ts`: enlarges a crop of a render.

## 1. Reference (the real subject)

A top-hung sliding door: one steel-framed glazed leaf on two trolleys in a slotted steel box track, with a floor guide. The track spans as a beam onto two steel jamb posts. The concept stays as it was: one leaf of a wider opening, and the other bays are not modelled.

| item | real value | source |
|---|---|---|
| leaf height, nominal opening | leaf about 1985 mm for the 2010 mm opening; handle 1050 mm above finished floor | DIN 18101:2014 |
| floor clearance of a top-hung leaf | 5–10 mm (brush or seal), never 100 mm | EN 1527:2019, top-hung gear practice |
| top-hung gear | trolleys with 4 wheels running on the treads of a slotted box track; hanger bolt with adjusting and lock nut; bracket or clevis on the leaf; floor guide in a bottom groove; end stops | EN 1527:2019 |
| hanger spacing | trolleys near the leaf ends (about 100–150 mm in), not at the centre | EN 1527 practice; stability against racking |
| box track | cold-formed, 2.5–4 mm, slot about 16–25 mm between upturned lips; inside bend radius about t | EN 10162:2003 |
| steel door frame tubes | 50 x 50 mm, wall 1.5–2 mm (welded steel/glass doors); bottom rail taller (about 100 mm) | trade practice (steel door profile systems) |
| glazing | 10 mm toughened, held by 15 x 15 mm steel glazing beads on both faces | EN 572-1, glazing practice |
| pull handle on toughened glass | back-to-back through the pane, Ø 16–25 mm grip, legs 200–300 mm c/c, finger clearance ≥ 25 mm | trade practice |
| densities | steel 7850, glass 2500, EPDM 1150 kg/m³ | EN 1991-1-1 Table A.1, EN ISO 10456 |

## 2. Audit (before)

| part | found | real / required |
|---|---|---|
| leaf height and floor gap | frame from y 2.0 m to 0.1 m: leaf 1.9 m with a 100 mm floor gap | 1.99 m leaf with 10 mm floor clearance |
| open pose | `dx` 1.0 m: leaf edge at x 3.0 m, inside the east jamb (2.95–3.05) by 50 mm | parks against the jamb buffer |
| track | body: a 100 x 84 housing with a 40 x 54 notch, no chamber, so no trolley could run in it. Rig section: RHS 100x50x4, a closed tube with no slot. 2D: 100 x 100 | slotted box track with chamber, treads, lips and slot |
| track face screws | four domed screws on the track face at mid-span, fixing nothing (no wall behind) | removed; the track bears on the jamb heads |
| jambs | an 80 x 20 groove cut into a closed post, with the seal line drawn on the face where it cannot be seen | cold-formed rebated post with a 60 x 20 receiving channel and an EPDM buffer |
| hangers | two 40 x 50 blocks at ±0.16 m from the centre (320 mm apart) | pendant hangers at ±0.35 m: trolley, rod, nuts, clevis, pin |
| carriage, wheels, floor guide | absent | 2 trolleys x 4 wheels, floor guide |
| handle | 290 x 60 x 8 plate glued on the glass plus a 20 mm block grip: about 8 mm clearance, unusable | back-to-back bar pull: rosettes, legs, 300 mm grip, 25 mm clearance |
| frame | four plain boxes, bottom rail 50 mm, glass with no glazing beads | rounded 50 x 50 x 2 tubes, 50 x 100 grooved bottom rail, beads |
| plan `leaf-rail-below` | region `leaf-glass`, material glass | it is the steel bottom rail |
| thermal 1:50 | skin-a was a 16 mm core rod inside each member. Track and jambs failed the area check. Cavity thickness 0.05 in a 100 mm section | real nested walls: skin-a is the room-side wall |
| structure | supports were flat discs. Joints were `of` hanger blocks. No floor guide, no masses | hollow members, trolley joints, pins, pin and roller supports, masses |
| masses | track 26.75 kg (RHS, 3.0 m). Leaf 60 kg. `massKg` 86.75 left out the jambs | recomputed from the modelled sections |

## 3. Root fixes (catalog, old → new)

- Rig `track.section`:
  - areaM2 0.001136 → 0.0015229
  - secondMomentM4 1.44126e-6 → 2.2208e-6
  - extremeFibreM 0.05 → 0.05017
  - title "RHS 100x50x4" → "Slotted box track 100 x 100 x 4"
  - All values computed from the modelled profile.
- `track.mass`: kg 26.75 → 38.32 (3.084 m of profile, 8 mm end plates, brushes); reference en-10219 → en-10162.
- `leaf`:
  - lengthM 2 → 2.09
  - joint maxM 1 → 0.962
  - mass kg 60 → 68.5
  - comX 1 → 1.072 (parts listed by the volumes script)
- Pose `open`: dx 1 → 0.962. The leaf edge now stops at the 8 mm EPDM buffer in the east channel (x 2.962). The description text is updated.
- `dimensions.massKg` 86.75 → 161.38: track 38.32 + jambs 54.31 + leaf 68.50 + guide 0.25.
- Materials: added `seal` (EPDM and nylon brush; EN ISO 10456 values).
- References added: en-1527, en-10162, din-18101. Provenance notes and summary rewritten in en and de.
- Thermal layers:
  - track-steel and jamb-steel cavity thicknessM 0.05 → 0.092.
  - leaf-frame skins 0.004 → 0.002 m (R 8e-5 → 4e-5, C 14100 → 7050).
  - leaf-frame cavity 0.05 → 0.046; heatCapacityJM2K 28000 → 14100.
  - Titles corrected.
- Thermal area: **track-steel areaM2 1.1 → 0.92**. This is the honest outer area of the brush-closed housing (0.3 m²/m x 3.1 m + end plates, less the room face). The change is a factor 0.84, so within the C16 author limit of 1.5.
- Structure assumptions rewritten: two trolleys; the floor guide restrains the leaf out of plane only.
- 2D, all four scales, redrawn as the true views of the new body (ruling E1):
  - **1:50 elevation**:
    - Kept and redrawn: jambs, track, leaf-frame, leaf-glass, the reflection strokes.
    - New: floor-guide, glazing-bead line, hanger-a/b as rod neck, nuts and clevis at ±0.35 with pin dots, handle-base / handle-base-b rosettes, handle-grip stadium, bottom-seal brush.
    - Removed: the four track screws and the two invisible jamb seal lines.
  - **1:50 front**: hidden outlines moved to the new leaf; the channel edges are at ±0.03; handle-behind added.
  - **1:50 plan**:
    - Rebated jamb sections.
    - Stiles with beads.
    - Rosette and grip cuts 15 mm off the beads, which removes the print-gap violation.
    - Bottom rail beyond the cut, now steel and `leaf-frame`.
    - Buffer lines; hangers above at ±0.35.
  - **1:100**: leaf to 10 mm above the floor, hanger blocks at ±0.35, handle bar, rebated jambs and the cut handle in plan.
  - **1:200 / 1:500**: leaf down to 10 mm. At 1:500 the leaf touches the drawn head.
  - Decisions texts updated in en and de.
  - The symbol's leaf follows: y 0.01, glass 0.13.
  - The handle shapes carry no `region`. The product's line view merges same-region outlines, so the handle would vanish against the frame region in light-line (it did in the old drawing too).
- Print suitability after the fixes: all four scales `authored`. 1:50 has min gap 0.29 mm and min feature 0.33 mm.

## 4. Models per scale

| scale | body | thermal | structure | what |
|---|---|---|---|---|
| 1:50 | 38 | 28 | 16 | Rebated jambs (r 8) with EPDM buffers. Slotted box track: 48-point profile with chamber, treads, lips and slot; 8 mm end plates; two slot brushes. Floor guide (T tongue). Leaf: stiles and top rail 50 x 50 tubes (r 4), 50 x 100 bottom rail with a 20 x 25 guide groove, 10 mm pane, 15 x 15 beads on both faces, two bottom brushes. Two pendant hangers: chassis, 4 crowned steel wheels and stub axles per trolley, M12 rod, lock and adjusting nuts, clevis, Ø16 pin. Back-to-back pull: rosettes, legs, grip. Thermal: nested skin-b / cavity / skin-a per jamb, track and leaf member, real walls; room faces `-w`; outdoor beads, clevises, pins and handle in skin-b; pane `-w`. Structure: track = the body profile (`of`), end plates, hollow rebated jambs, four hollow leaf members, hanger rods, trolley joints, pin joints, floor-guide joint, pin support (west) and roller pair (east) at the anchors, masses of leaf and track. |
| 1:100 | 8 | 8 | 14 | Rebated jambs (sharp), track bar, leaf frame extrusion (brush and groove merged), pane, hanger blocks, D-profile handle per face (half-turn repeat). Thermal: one skin per part, `-w`. Structure: hollow 4 mm jambs and track, four sharp 2 mm leaf tubes, trolley blocks, hanger joints, supports, masses. |
| 1:200 | 5 | 5 | 10 | Three 100 mm bars and the frame ring (to the track, 75 mm inset), 20 mm pane. Structure: jambs and track `of` (solid at this scale, C30), leaf stiles and rails, trolley blocks, supports, leaf mass. |
| 1:500 | 4 | 5 | 8 | 150 mm portal and one 75 mm leaf block. Thermal: frame ring plus a pane that fills it, together exactly the body leaf ("Minimal:" declared). Structure: real 100 mm jambs and track on their axes, 75 mm frame ring, carriage joint bar, pin and roller, leaf mass. |

Final check line: `$BUN volumes-check.ts components/sliding-door --product` → **PASS (7.3 s)**. `r3-validate.ts components/sliding-door --volumes` → VALID (schema 0, validateAsset 0, checkVolumes 0); also VALID with `--catalog-only` after the splice.

Silhouette IoU at 1:50 (elevation / front / plan): 0.99996 / 0.9995 / 0.994. Coarser scales: 1.000.

Areas at 1:50, deviation from `areaM2`:

| region | deviation | ok |
|---|---|---|
| glass | +4.5 % | yes |
| leaf-frame | −8.4 % | yes |
| track | +0.3 % | yes |
| jamb | −8.8 % | yes |

Totals at 1:200 and 1:500: 2.5 % and 14.8 %. The volumes are spliced with `volumes-insert.py`.

## 5. What I saw (final renders)

- **1:50 body, six views** (`r3-sheet`): reads as a real top-hung steel and glass door.
  - From −z and +z: the clevises with pins under the track, the nuts and the rod entering the slot.
  - The bead line round the pane, the taller bottom rail, the brush line at the floor.
  - The pull handle with two rosettes at 1.07 m.
  - Front and back: only the jamb with the track head; the leaf is behind it, as the 2D draws.
  - Top: the track hides everything.
  - Three-quarter from below (turn 30, tilt −15): the dark slot line under the track, the channel at the foot of the east jamb, the brush under the leaf.
  - Enlarged hanger crop: clevis, pin head, nuts and the rod into the brushed slot, all read clearly.
- **1:50 open pose**: the leaf edge sits in the east jamb channel with no overlap of the steel; hangers and trolleys stay under the track.
- **1:50 2D** (`r3-2d`, closed and open):
  - Elevation: clean portal, leaf, beads, hangers, handle (now also in light-line), brush, floor guide.
  - Plan: C-shaped jamb sections facing the opening, leaf section with beads and handle.
  - Front: jamb and track, with hidden leaf outlines.
- **1:50 thermal**:
  - Cut sheet: pane red, frame green, track violet, jambs olive.
  - Top cut (enlarged): the jamb shows the skin ring, the cavity and the buffer in the channel.
  - The stile section shows the nested layers.
- **1:50 structure**:
  - Blue hollow members and the magenta leaf mass.
  - Cut elevation: the two orange trolley joints in the track, the rods, the pins, the floor-guide joint, the track mass, the hollow jamb walls.
- **1:100**: overlays all green; the handle reads. Clip slide-open-close at t = 2 s (six views): the leaf moves half open and stays closed against the track.
- **1:200 and 1:500**: overlays all green, clean generalised portal and leaf. 1:500 structure shows the orange carriage bar at the head.

## 6. Still imperfect (stated plainly)

- **Supports hidden in renders:** the support solids (pin, roller pair) lie inside the track at the jamb axes, where the structural anchors are, so they cannot be seen in shaded renders. The `--cut` panel does not show them either, because the open cut ends are culled. This is placement by the anchors, not a missing part.
- **Hollow sections closed in body:** jambs, track members of the leaf and the frame tubes are closed solids in `body`; their real walls are in thermal and structure. This is the convention of the asset and is not visible from outside.
- **Rod through brushes:** the M12 rod passes through the slot brush strips, as bristles part round a rod. That is the only intentional overlap in the model.
- **Handle has no thermal tint in 2D:** the 2D handle shapes carry no `region` (see section 3), so the thermal 2D view does not tint the handle; its 3D thermal solid is still `leaf-frame`.
- **Two areas left as declared:** jamb-steel (1.43 declared, 1.30 modelled) and leaf-frame (1.1 declared, 1.01 modelled) are within 15 % and were not changed.
- **Same-region outlines merged in light-line:** a product behaviour, not filed as a defect. Any body shape of the frame region drawn over the glass loses its outline in light-line.
- **Slender seals were widened by the check:** declaring the buffers and brushes `slender` made the product widen them to the 100 mm shape they sit in. I made them ordinary 8 to 12 mm solids instead, so `slender` is not used.

## Round 5 handcraft (2026-11-21)

Renders (opened with Read) under `🗑️generated/r5-sliding-door/before/` and `after/` (made by `r5-c01-sheets.sh`). I opened body (pose `half-open`), structure and the cut thermal sheets at 1:50, and body at 1:100, 1:200 and 1:500. The 1:50 solids were checked against the reference table of round 3: 1990 mm leaf, 10 mm floor clearance (EN 1527), trolleys at ±0.35 m, a 50 x 50 x 2 frame with a 50 x 100 bottom rail, and a back-to-back pull with 25 mm finger clearance. No deviation above 3 %.

| scale | before = after (body / thermal / structure) |
|---|---|
| 1:50 | A- / A- / A- |
| 1:100 | A- / A- / A- |
| 1:200 | A- / A- / A- |
| 1:500 | B+ / A- / A- |

The 1:500 leaf is one block with no frame read, because the 50 mm frame is under the 75 mm feature. In the half-open pose the leaf hangs cleanly under the track.

No change was needed. The source equals the catalog; it is VALID.

Remaining:
- the support solids are hidden inside the track;
- the cut-cap wedge in the thermal three-quarter view is a renderer artefact.

## Canonical component readback, 2026-11-21

Canonical source `39ef8909fe658091`. The 10 mm glass leaf remains framed. Each side has its own stand-off pull handle. Two four-wheel carriages, their hangers and the open overhead track remain visible from behind. The exterior thermal view follows the three exposed faces of the track. All twelve exported scenes reopened (1,192,992 bytes). The strict catalog source readback passed.
