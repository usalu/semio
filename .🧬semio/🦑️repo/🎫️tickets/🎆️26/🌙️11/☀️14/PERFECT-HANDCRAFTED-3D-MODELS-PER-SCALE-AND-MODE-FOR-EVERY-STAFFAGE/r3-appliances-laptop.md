# Round 3 — appliances/laptop

State: DONE (2026-11-20). Catalog asset valid and spliced at 1:50, 1:100, 1:200 and the digital 1:500; `volumes-check.ts --product` PASS, `r3-validate.ts --volumes` VALID (schema 0, validateAsset 0, checkVolumes 0), print suitability without violations at the three drawn scales.

Scripts (ticket folder): `r3-laptop-kit.py` (measures, bone frames, solid helpers, corrected rig), `r3-laptop-model.py` (every solid of every scale, decisions), `r3-laptop-draw.py` (2D of every drawn scale and view), `r3-laptop-roots.py` (candidate document), `r3-laptop-apply.py` (writes changed root members and the volumes source), `r3-laptop-suit.ts` (print suitability), `r3-laptop-sheet500.ts` (render of the 1:500 volume), `r3-laptop-ship.sh` (whole chain: candidate, validate, write roots, splice, validate).

## 1. Reference

Subject: 14-inch aluminium clamshell of the thin-and-light class; no single product, measures rounded from two.

| measure | reference | source | model |
|---|---|---|---|
| width | 313.6 / 312.6 mm | ASUS Zenbook 14 OLED UX3402 spec sheet / Apple MacBook Pro 14 (2021) SP854 | 313 mm |
| depth | 220.6 / 221.2 mm | same | 218 mm |
| closed thickness | 16.9 / 15.5 mm | same | 17 mm without feet (chassis 10.5, lid 6.5), feet 2 mm |
| mass | 1.39 / 1.6 kg | same | 1.30 kg (base 0.82, lid 0.48), unchanged |
| panel | 14.0 in, 16:10 = 301.6 x 188.5 mm | geometry of the diagonal | 301.6 x 188.5 mm behind glass with 3 mm rim |
| key pitch / cap | 19 mm / about 16 mm, function row half height | ISO/IEC 9995 pitch, product practice | 19 mm pitch, 16 x 15.5 mm caps, function row 9.5 mm |
| keyboard field | about 280 x 105 mm, six rows, 78 to 79 keys | product practice | well 279 x 104 mm, 78 caps incl. inverted-T arrows |
| trackpad | 130 x 78 to 80 mm | product practice | 130 x 78 mm glass |
| hinge | barrel 8 to 10 mm, axis just above the deck at the rear edge, two friction hinges near the ends | product practice | barrel 9 mm, axis 4.5 mm in front of the rear edge and 2 mm above the deck |
| sheet thickness | 1.0 to 1.2 mm aluminium | asset thermal layers | thermal layers 1.2 / 1.2 / 1.0 mm unchanged; structure plates drawn 2 mm (stated in decisions) |
| feet | four rubber feet about 12 mm, 2 mm high | product practice | four revolved feet of 12 mm, 2.4 mm |
| open angle | 105 degrees working, 135 maximum | asset pose and joint | unchanged |

## 2. Audit (state before round 3)

- Rig: `base` joint sat at the middle of the chassis (x 0) and `lid` at x -0.104, y 0.0125, i.e. the lid turned about the top rear corner of the deck plane, not about the hinge barrel; lid length 0.213. A lid that turns about the deck edge cuts into the chassis when it closes.
- Anchors: `feet-contact` and `deck-top` followed the old base origin (0, 0 and 0.01, 0.0125).
- Dimensions: height 0.218 and length 0.275 were the old box lid at 105 degrees; with the real hinge axis and lid the envelope is 0.221 x 0.273.
- 2D: 1:50 elevation of 11 shapes with a box chassis without feet, bilge, ledge or barrel; front 5, plan 8; 1:100 and 1:200 the same boxes thinner.
- 3D: 1:50 body 9 solids (two boxes, key block, pad, feet), 1:100 and 1:200 two boxes each (1:200 a copy of 1:100, qa-visual finding 4), structure = block members, no 1:500.
- Materials: key caps, hinge and brackets had no material; references named no product dimension source.
- Thermal areas (deck 0.068, underside 0.068, lid 0.137 m2) agree with the honest model within 13 %, 10 % and 3 %: not changed.

## 3. Root fixes (old -> new)

| member | old | new |
|---|---|---|
| `rig.bones.base` x, com | x 0, com (0, 0.0065) | x -0.109 (rear edge), com (0.109, 0.0074) |
| `rig.bones.lid` x, y, length, com | -0.104, 0.0125, 0.213, (0.106, 0.003) | 0.0045, 0.0145 (barrel axis), 0.2135, (0.1, 0.0008) |
| `anchors.feet-contact` | (0, 0) | (0.109, 0) |
| `anchors.deck-top` | (0.01, 0.0125) | (0.114, 0.0125) |
| `dimensions` height / length | 0.218 / 0.275 | 0.221 / 0.273 (catalog index row updated alike) |
| `materials` | 3 | + `keycap-plastic`, `hinge-steel` |
| `provenance.references`, `notes` | no product source | + ASUS UX3402, Apple MacBook Pro 14 spec sheets; geometry note rewritten (en, de) |
| `representations` 50 / 100 / 200 | 11+5+8, 3+3+3, 2+2+2 shapes | redrawn as true orthographic views of the new bodies: 7+7+10, 3+4+4, 2+2+2, same roles and bone bindings, new decisions |

Poses (`open`, `open-idle`, `closed`) and clips (`open-lid`, `work-and-sleep`) keep their angles; with the corrected axis the closed lid lies 0.3 mm above the deck and nothing intersects.

## 4. Model per scale (solids before repeat / expanded)

- 1:50 — body 39 / 134: tub with boat-shaped bilge on four revolved rubber feet; deck plate with hinge ledge, keyboard well and trackpad opening; keyboard bed; 78 key caps in six rows with half-height function row and inverted-T arrow keys; glass trackpad; two speaker fields; exhaust grille on the ledge; two USB-C left, USB-C and audio jack right; underside with two fields of 14 intake slots and 8 cover screws; lid shell with rounded back edge, cover glass, lit panel, camera lens; hinge barrel. Thermal 33: every body surface under deck / underside / lid, casing joints as `sectionFaces`, plume outlet patch on the deck (D2); no hidden regions and one layer per region in the asset, so no shells or internals. Structure 15 / 19: floor plate on the feet, perimeter wall frame, palm-rest cross rib and two battery ribs, deck plate with both openings, two steel hinge brackets, two friction-hinge joints on the axis, lid back plate with rim and two steel hinge arms, two masses, contact pad.
- 1:100 — body 6: one hull with bilge standing on the desk, key field and trackpad as inlays, lid shell, glass, barrel. Thermal 6: hull split at the parting line, lid, glass, barrel, outlet. Structure 9 / 10: floor, wall frame with one rib, deck plate with keyboard opening, lid frame, two hinge joints, masses, pad.
- 1:200 — body 4: two plates with rounded plan corners and chamfered bottom edge, dark glass face, dark key field (both several features large; added in this session, before it was two bare plates). Thermal 5: bottom and deck shells, lid, glass, outlet. Structure 6: chassis perimeter frame, lid frame, one hinge axle, masses, pad.
- 1:500 (digital, no printed drawing) — handcrafted anew, replacing the bulk pass's wedge-shaped base (a taper the real chassis does not have): body 3 (two plain plates and the glass face), thermal 5 (chassis halved at mid height, lid, glass, octagonal outlet), structure 6 (floor plate, lid back plate, square hinge axle, masses, pad).
- `decisions`: three bilingual texts (body, thermal, structure) per scale, en then de.

Final check line: `volumes-check.ts appliances/laptop --product` -> `PASS (3.4 s)`; 1:50 silhouette IoU 0.982 / 0.999 / 1.000 (elevation / front / plan), area deviation deck 0.13, underside 0.10, lid 0.03 (limit 0.15). `r3-validate.ts appliances/laptop --volumes` -> `VALID ... schema 0, validateAsset 0, checkVolumes 0`.

## 5. What the final renders show (opened with the image viewer)

- 1:50 body sheet, six views, pose `open`: reads at first sight as a current 14-inch aluminium laptop. Front: glass with even rim and a taller chin, barrel between lid and deck. Side: lid 105 degrees turning on the barrel, chassis on feet with bilge, ports visible. Back: plain lid, rounded corners. Top: keyboard with staggered rows, wide modifier keys, space bar, inverted-T arrows at the user's right, trackpad centred, speaker fields, hinge ledge. Three-quarter from below (turn 230, tilt -35): four feet, two slot fields, eight screws, ports on the edge.
- 1:50 pose `closed` and clip `open-lid` at 0.5 s: lid lies flat on the chassis with a hairline gap, barrel stays in the ledge, no part pokes through; rig joints sit on the barrel axis.
- 1:50 thermal (cut): green underside tub, red deck with keys and outlet disc, violet lid; nothing outside the body. 1:50 structure: tray, frame, ribs, brackets, orange hinge joints at the two real hinge positions, lid plate with arms, masses at the centres of mass.
- 1:100 body, thermal, structure: clean simplification, key field and trackpad still tell the deck.
- 1:200 body: two plates with glass face and key field; recognisable as a laptop, screen side distinct from the back.
- 1:500 (own render `r3-laptop-sheet500.ts`, front and back three-quarter for body, thermal, structure): two plates with the dark screen face; thermal halves and outlet; plates, axle and masses in structure.
- 2D sheets 1:50, 1:100, 1:200 (`r3-2d.ts`): side, front, plan agree with the bodies.

## 6. Notes and remaining limits

- `r3-sheet.ts` cannot render a scale without a 2D representation (it throws in `poseState`, `rep.geometry` undefined at 1:500); worked around with `r3-laptop-sheet500.ts`. Not a model defect.
- The 1:50 ship chain's candidate and the pre-round backup under `🗑️generated/r3-laptop/` were removed; `sh r3-laptop-ship.sh` rebuilds everything.
- Structure plates are 2 mm instead of the real 1 to 1.2 mm so that they stay visible; stated in the decisions.
- Internals (battery, board, fans, heat pipe) are not modelled: the asset declares no hidden thermal region, and inventing them would steer physical data.
- Key legends, logo and the display gasket are below any modelled feature and left out.

## Round 5 handcraft (2026-11-21)

Process (all assets of E03; final `r3-validate.ts <asset> --catalog-only --volumes` VALID for all five after the splice): `r5-appliances-render.sh` rendered every list at every scale, every pose and both clips at 1:50 (t 0.5 s) and the 2D of every drawn scale into `🗑️generated/r5-<slug>/before` and `/after`; `r5-seated-montage.ts` laid them out for reading; every sheet named below was opened with the Read tool. `r5-appliances-measure.ts` measured the body extents per scale in the default pose (numbers below). Grades: body / thermal / structure.

### Reference and model (default pose `open`, 105 degrees)

| quantity | reference | source | model | deviation |
|---|---|---|---|---|
| width | 313.6 / 312.6 mm | ASUS Zenbook 14 UX3402 / MacBook Pro 14 | 313.0 mm (all scales) | -0.2 % |
| depth (chassis) | 220.6 / 221.2 mm | same | 218.0 mm | -1.2 % |
| closed thickness | 16.9 / 15.5 mm | same | 17 mm + 2 mm feet | +0.6 % |
| panel 14 in 16:10 | 301.6 x 188.5 mm | diagonal | 301.6 x 188.5 mm | 0 % |
| trackpad | 130 x 78-80 mm | product practice | 130 x 78 mm | 0 % |
| keyboard well | about 280 x 105 mm | product practice | 279 x 104 mm | -0.4 / -1 % |
| hinge barrel | 8-10 mm | product practice | 8.9 mm | - |
| open envelope | 0.221 x 0.273 m (asset dimensions) | - | 0.2211 x 0.2725 m (1:50), 0.2212 x 0.2731 m (1:500) | 0 / +0.1 % |

### Grades

| scale | before | after |
|---|---|---|
| 1:50 | A / A- / A-; catalog INVALID (print: hidden plan barrel 0.187 mm at 1:20) | A / A- / A-; VALID |
| 1:100 | A- / A- / A- | unchanged |
| 1:200 | A- / A- / A-; catalog INVALID (print: 6.5 mm lid 0.199 mm at 1:350) | unchanged; band ends at 1:320 |
| 1:500 | A- / A- / B+ | unchanged |
| poses `open-idle`, `closed`, clips `open-lid`, `work-and-sleep` (t 0.5) | A (lid closes flush onto the deck, half-open lid clear of the chassis) | unchanged |

Seen: body at all scales, structure and thermal cut at all scales, three poses and both clip frames at 1:50, 2D 1:50, 1:100, 1:200. The clamshell reads right at first sight: tub with bilge and feet, 78 keys with inverted-T arrows, glass trackpad, speaker grilles, hinge barrel at the real axis, display in its bezel; closed, it is one 19 mm slab. The model needed no change; the catalog asset was refused by the print check.

### Change (root, `r5-laptop-print.py`, candidate validated before writing)

- 1:50 plan: the hinge barrel shape lay under the lid shell, only a 1 mm sliver of it stayed visible (0.187 mm at the 1:20 end of the band); it is removed from the plan, the silhouette decision says the lid band hides the barrel (en, de). The 3D barrel stays (it is hidden in plan in 3D as well).
- 1:200 `validRange.maxDenominator` 350 -> 320: the 6.5 mm lid with the 0.18 mm body pen reaches 0.200 mm exactly at 1:320 (6.5 / 320 + 0.18); the 1:500 drawing is unavailable, so no band starts at 1:350. 2D 1:50 and 1:200 re-rendered and opened after the change.

### Open points

- 1:500 keeps a dark screen inset in the lid at the 75 mm feature (digital only), structure there is two plates on one hinge axle (B+).
