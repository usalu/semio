# Round 3 — furniture/operating-table

State: DONE (2026-11-20). No earlier round-3 work existed; the asset was rebuilt from the roots. The catalog `volumes` equalled `volumes/furniture/operating-table.json` at the start (the 03:40 bulk passes had not touched this asset; all four scales were already authored). Everything below describes the catalog file as it is now.

Scripts (ticket folder): `r3-operating-table-kit.py` (measures, bone frames, part primitives that carry a solid and its three orthographic outlines), `-model.py` (every part of every scale stated by hand; thermal, structure, volume decisions), `-draw.py` (2D shapes of every scale and view from the same parts; representation decisions), `-roots.py` (rig, anchors, dimensions, areas, references, texts), `-build.py` (candidate document), `-apply.py` (writes changed root members into the catalog file), `-ship.sh` (build, validate, apply, splice, validate), `-sheet.ts` (private copy of `r3-sheet.ts` that reads the candidate, used before the first write), `-suit.ts` (print-suitability violations). `sh r3-operating-table-ship.sh` reproduces the catalog state.

## 1. Reference

| Quantity | Reference | Source | Model |
| --- | --- | --- | --- |
| Table top | 500 to 540 mm wide, 560 to 590 mm over the side rails, 2030 to 2130 mm long with head and leg sections | ALVO Etiuda (2100 long, 500 wide, 560 total), Schmitz OPX mobilis (540 wide, 2130 long), web search 2026-11-20; IEC 60601-2-46 (asset reference) | 2.0 m long, pads 0.50 m, 0.56 m over the rails (asset values 2.0 and 0.5 kept) |
| Height range | 690 to 1050 mm | Schmitz catalogue 60, medifa 6000 brochure | drawn at 0.90 m (kept) |
| Base | 940 x 540 mm (medifa 6000), 1120 x 580 mm (Schmitz); narrow middle for foot room | manufacturer data | H chassis 0.94 x 0.54 m: spine 0.30 m wide, two cross beams 0.14 x 0.54 m, 90 mm high |
| Castors | twin castors 100 to 125 mm, partly inside the base | product knowledge | four twin castors 100 mm at x +-0.40, z +-0.21 m |
| Column | telescopic stainless shrouds, about 0.25 to 0.30 m, eccentric or central | product knowledge | three shrouds 0.30 x 0.26, 0.27 x 0.23, 0.24 x 0.20 m, flange, cap, bearing block |
| Tilt | Trendelenburg axis in the column head below the top frame | product knowledge | pin at 0.74 m, 40 mm under the frame, saddle cheeks; 25 degrees clear the column cap by 14 mm |
| Sections | head plate about 0.25 to 0.30 m on two rods, back, seat, split leg plates 0.65 to 0.78 m | product knowledge | head 0.265 x 0.30 m, back 0.475, seat 0.47, two leg plates 0.735 x 0.242 m, 10 mm joints |
| Pads | 60 to 80 mm foam under vinyl | asset (80 mm) | 80 mm, rounded edges |
| Side rails | 25 x 10 mm stainless on posts, about 20 mm off the frame | product knowledge (European rail; the web search did not confirm the profile) | 25 x 10 mm, 16 mm posts, three pieces per side |
| Mass | about 200 kg (medifa 6000) | brochure | 210 kg kept |
| Light stand | stand 1.88 m, spring arm 0.805 m, head height 0.9 to 2.08 m | Dr. Mach LED 150 FP on mobile stand, MSF catalogue | 1.96 m (asset dimension kept), arm 0.8 m at 1.92 m |
| Light head | diameter 440 to 520 mm, thickness up to 150 mm | Constella C 520 mm, CL-2MB 440 mm, FY-500BS 500 mm (medicalexpo) | 0.5 m, 105 mm thick, lens 0.472 m, sterile handle 30 mm |
| Working distance | 0.7 to 1.4 m, 1.0 m nominal | IEC 60601-2-41 (asset reference) | lens at 1.66 m, 0.76 m above the top (kept) |

Sources: https://cdn.shopify.com/s/files/1/0044/7319/2566/files/schmitz_kat60_en.pdf , https://rynamedical.co.uk/s/BROCHURE-6000-Medifa-Operating-Table-2018.pdf , https://unicat.msf.org/cat/product/65939 , https://www.medicalexpo.com/prod/epmd-group/product-4612107-2808068.html , https://www.medicalexpo.com/prod/uezuemcue-hospital-equipment/product-303834-1077812.html , https://hospimedica.com/expo/product/13305/mobile-surgical-table .

## 2. Audit of the state before round 3

- Rig: `column` and `stand-column` had `angleDeg` 0, so both upright bones lay horizontally (0.62 m and 1.86 m along +x). `top` was 2.0 m long from the pivot (tip at x = 2.0 m, 1 m beyond the table). `base` 0.8 m and `stand-base` 0.56 m ran from the centre past the end of their parts. `stand-arm` pointed along +x while the arm lies at -x. `light-head` 0.5 m from the centre. The tilt pivot sat on the underside of the carrier (0.78 m), where a 0.32 m head plate would collide at any tilt.
- Dimensions: `depthM` 0.5 with no side rails; base 0.8 x 0.4 m (real 0.94 x 0.54 or larger); castors at +-0.32, +-0.17 m.
- Stand: arm 0.5 m (real 0.8 m), a single rod to the head, base a plain plate; texts called the stand position "beyond the head end" although the head pad is at -x and the stand at +x; notes said "0.62 m beside the table centre".
- 2D: four equal-width pads on one carrier strip, a tapered block and a pleated box as column, base plate with two circles; front and plan constructed, not projected.
- Solids: 22 / 20 / 8 / 7 body solids: lofted plate, box column, one carrier box, four rounded boxes, stand of plate, cylinder, box arm, knob, rod, dome. Structure referenced the body blocks (qa-visual: B at every scale, column and base "filled blocks").

## 3. Root fixes (catalog file, outside `volumes`)

| Member | Old | New |
| --- | --- | --- |
| `dimensions.depthM` | 0.5 | 0.56 (over the rails); height 1.96, width 2.45, mass 248 unchanged |
| `rig` `base` | length 0.8 | 0.47 |
| `rig` `column` | angle 0, length 0.62, com (0, 0.3) | angle 90, length 0.58, com (0.3, 0) |
| `rig` `top` | at (0, 0.62), angle 0, length 2, comY 0.07 | at (0.58, 0) on the upright column, angle -90 (level), length 1.0, comY 0.11; pivot at 0.74 m |
| `rig` `stand-base` | length 0.56, comY 0.06 | 0.28, 0.1 |
| `rig` `stand-column` | at y 0.1, angle 0, length 1.86, com (0, 0.9) | at y 0.17 (socket), angle 90, length 1.75, com (0.9, 0) |
| `rig` `stand-arm` | at (0, 1.82), angle 0, length 0.5, comX -0.25 | at (1.75, 0), angle 90 (world 180, along the arm), length 0.8, comX 0.4 |
| `rig` `light-head` | at (-0.5, -0.26), angle 0, length 0.5, comY 0.06 | at (0.8, 0.23), angle 180 (level), length 0.25, comY 0.02; joint on the tilt bearing at 1.69 m, x 0.37 m |
| `anchors` | table castors x +-0.32, z +-0.17; `light-top` y 0.115 | x +-0.40, z +-0.21; y 0.075 |
| `thermal.regions[].areaM2` | frame 1.76, column 0.9, base 0.76, stand 1.08, housing 0.43 | 2.64, 0.75, 1.06, 0.96, 0.5 (all within factor 1.5; model 2.681, 0.752, 1.064, 0.961, 0.495); pad 2.4 and lens 0.19 kept (model 2.363, 0.184) |
| `thermal.regions[table-column].layers[0].title` | steel and polymer bellows | stainless steel shroud 2 mm |
| `summary`, `provenance.notes`, `structure.assumptions` | old measures, "head end" | rewritten (en, de) |
| `provenance.references` | 5 | + `schmitz-opx-mobilis`, `medifa-6000`, `mach-led150fp` |
| `representations[*].geometry` | 28/26/12, 20/20/9, 8/9/4, 7/7/4 | 55/71/73, 36/46/40, 26/26/25, 9/9/9 shapes (elevation/front/plan), ruling E1 |
| `representations[*].decisions` | old | rewritten per scale (en, de) |

Table-frame area: the hand sum of trays, rails, saddle and head plate is 2.68 m2; 2.64 is the largest value the factor 1.5 allows and is 1.5 % below the model. Poses, clips, masses, supports list, materials, catalog index (height, width) are unchanged. Every shape is the outline of one hand-stated part; shapes of the tilting `top` are extruded solids in front and plan.

## 4. What was modelled

### 1:50 — body 104, thermal 110, structure 88
- Body: chassis spine with chamfered shoulders, two lofted cross beams, plinth, four twin castors with forks, two brake pedals; column flange, three rounded shrouds, cap, bearing block rounded about the pin; tilt pin, two saddle cheeks, two saddle plates; four section trays (side beams under a 14 mm panel), end bar, four hinge barrels, head plate, clamp, two rods, six rails on twelve posts, five pads as superelliptic lofts with pulled-in ends. Stand: two legs, eight wheels, forks, stems, spine, ballast box, socket, tube, driver box, bearing head, arm with both joint barrels, swivel, half yoke of five tube pieces, tilt bearing, housing as one revolve (lens seat, rim, cooling step, dome), lens, handle.
- Thermal: body surfaces per region; each pad is its vinyl skin over an own foam core 1 mm inside; lens with `sectionFaces +v`; plume as a 2 mm skin on the dome crown (D2).
- Structure: chassis bars, wheels, contact pads, two hollow lifting sections, bearing block, pin, cheeks, saddle plates, side beams of every section, cross bars, hinge barrels, head plate on rods, stand legs, spine, 2.5 mm tube, two arm links and the spring between the joint barrels, yoke, ring and spar in the head, handle; seven masses.

### 1:100 — 46 / 47 / 59
Three rounded chassis bars on single wheels, two shrouds under a faceted block, plain section slabs, rail and posts as one bar, simpler pads, stand with single wheels and a ballast bar, arm as one tapered body, three-piece yoke with a chamfer bend, head with fewer points and a 15 mm lens.

### 1:200 — 26 / 28 / 43
H of three bars on two rollers, two-step column with head block, one frame slab 0.56 m wide, head plate and head pad, body pad, leg pad; stand as legs on rollers, ballast box, tube, bearing head, box arm, swivel and L yoke of 30 mm bars, turned head with handle.

### 1:500 — 9 / 13 / 37
Base slab, column block, top slab with the narrower head piece; foot slab, 60 mm column (slender), arm, hanger, chamfered disc. Declared minimal.

## 5. Checks (final, on the catalog asset)

- `r3-validate.ts furniture/operating-table --catalog-only --volumes`: `VALID ... schema 0, validateAsset 0, checkVolumes 0`. Catalog `volumes` equal the ticket source.
- `volumes-check.ts furniture/operating-table --product`: `PASS (21.6 s)`. Silhouette IoU elevation / front / plan: 1:50 0.997 / 0.996 / 0.994; 1:100 0.996 / 0.998 / 0.999; 1:200 0.998 / 0.998 / 1.000; 1:500 0.999 / 0.995 / 1.000. Bounds deviation 0 at every scale. Area deviation at 1:50 between 0.2 % and 3.2 %; totals 1:200 1.6 %, 1:500 14.9 %.
- Print suitability (`r3-operating-table-suit.ts`): no violation at any scale.

## 6. What the renders show (opened with the Read tool)

Opened after the last change: `r3-sheet` body 1:50 and 1:200; structure 1:50, 1:200, 1:500; thermal cut 1:50, 1:100, 1:200, 1:500; `r3-2d` 1:50 and 1:200. Opened before the last change, which did not touch their lists (it changed only the coarse structure lists and the 1:200 head plate and yoke): body 1:100 and 1:500, structure 1:100, `r3-2d` 1:100, 1:500 and 1:50 `head-down`, 1:50 three-quarter close-up, 1:50 `head-down` from below. Rendered but not opened: the `volumes-render.ts` overlay of 1:50.

- Front (from the foot end): stand column, ballast and legs in front of the two leg pads, rails at the sides, the stepped column and the chassis with its wheels; lamp disc above.
- Right and left side: head pad on its plate and rod, back, seat and leg pad on thin trays with rails and hinge barrels, saddle triangle on the pin, three shrouds stepping in, chassis with a pedal and half-hidden wheels; stand tube with driver box, arm with round joints, L yoke, flat dome with lens line and handle.
- Back: the narrow head pad in front of the wider back pad, column, chassis.
- Top: narrow head pad, three full-width fields with the split leg pad, rails standing off on posts, lamp disc over the leg plates, arm and yoke, stand base past the foot end.
- Three-quarter: reads at once as an operating table with a mobile surgical light. From below in `head-down`: tray recesses, saddle on the pin, the tilt closes without collision, lens and handle under the head.
- Rig: both column bones upright, top bone from the pin to the foot end, arm bone along the arm, head bone from centre to rim.
- Thermal: foam cores inside the vinyl skins in the cut, frame, column, chassis, stand and lamp head each in their colour. Structure: an own load path at every scale, connected from the head carrier to the castors; pads and shrouds ghosted.
- 2D: clean projections at every scale; the tilted pose rises in the front view and shortens in plan.

## 7. Still imperfect

- The lamp head stands over the leg plates (x = 0.37 m), not over the abdomen: with the stand beyond the foot end a real 0.8 m arm reaches no further. A stand beside the table would cover it in the side elevation; the asset's layout was kept.
- Lamp housing: no LED boards or driver as hidden internals (qa-visual wish); the asset declares no hidden region for them and none was invented.
- Foam layer data say 70 mm while the pad is 80 mm (core modelled 78 mm inside the 1 mm skin); the physical layer data were not touched.
- Table-frame `areaM2` is 2.64 against 2.68 measured, held by the factor 1.5 of C16.
- Pads are symmetric superellipses, so their lower edges are rounded like the upper ones; a real pad is flat underneath.
- Tilt actuator, hand control, cables and the gas spring cover of the arm are not modelled; the spring is in the structure list only.
- `r3-sheet --list thermal`: at 1:200 and 1:500 the solids that stand for two regions are striped (two references at one place), and pads are grey at 1:100 and coarser; noted in `tool-requests.md`.
- Repo test suites and exports were not run; fixtures that embed this asset will differ.
