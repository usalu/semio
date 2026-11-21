# Round 3: appliances/split-ac-indoor-unit

State: finished and spliced on 2026-11-20. No earlier round-3 work existed for this asset, so it was rebuilt from the roots. Everything below describes the catalog file as it is now.

Scripts in the ticket folder:
- `r3-split-ac-indoor-unit-geo.py`: measures, the D profile, the discharge mouth, the flap section and the vane frame transform.
- `r3-split-ac-indoor-unit-volumes.py`: every solid of every scale and list, stated by hand. It writes `volumes/appliances/split-ac-indoor-unit.json`.
- `r3-split-ac-indoor-unit-roots.py`: the root fixes and the 2D of 1:50, 1:100 and 1:200. It writes the candidate `🗑️generated/r3-split-ac-indoor-unit/🔣️.json`.
- `r3-split-ac-indoor-unit-apply.py`: writes the candidate's root members into the catalog, keeping its formatting. It is a copy of the mvhr apply script.

To resume, run in order:
1. `python3 r3-split-ac-indoor-unit-volumes.py`
2. `python3 r3-split-ac-indoor-unit-roots.py`
3. `$BUN r3-validate.ts --file <candidate>`
4. `python3 r3-split-ac-indoor-unit-apply.py`
5. `python3 volumes-insert.py volumes/appliances/split-ac-indoor-unit.json <catalog>`

## 1. Reference

| Quantity | Real value | Source | Model |
|---|---|---|---|
| H x W x D | 299 x 798 x 219 mm | Mitsubishi Electric MSZ-AP25VG spec sheet (web search 2026-11-20) | 290 x 850 x 210 mm kept (class value, within 5 to 7 %) |
| H x W x D | 294 x 811 x 272 mm | Daikin FTXM25N technical table (web search 2026-11-20) | as above |
| Mass | 10.5 kg (Mitsubishi), 10 kg (Daikin) | same | 9 kg kept (8.55 kg unit plus 0.45 kg flap) |
| Profile | Flat back on the wall plate, flat top, front panel, and a large lower front radius that carries the discharge flap | product knowledge | D profile: top corner R 35 mm, lower quarter circle R 115 mm about (0.095, 0.115) |
| Intake | Top grille across nearly the full width, with filters under it | product knowledge | Recess 105 mm x 0.79 m, 9 mm deep; 5 bars of 7.5 mm, 9 ribs, dark filter |
| Front panel | Hinged ABS panel between the top edge and the lower radius | product knowledge | 10 mm panel, y 0.118 to 0.255, with seams |
| Discharge | Opening of about 80 to 90 mm under the front. A curved main flap closes it flush with the casing; vertical swing louvres sit behind it | product knowledge | Opening 85 mm chord, between the lips at -82.5 and -37.5 degrees on R 115 mm. Flap 8 mm thick at the rear, tapering to 4 mm. 17 louvres |
| Coil | Fin-and-tube evaporator folded over the fan in 2 rows, about 25 mm deep, fin pitch about 1.3 mm | product knowledge | Folded band of 25 mm. Wetted area about 9.6 m2 (540 fins x 0.35 m x 25 mm x 2) |
| Fan | Cross-flow rotor of about 100 mm, about 0.62 to 0.7 m long, with its motor at one end | product knowledge | Diameter 100 mm, 0.66 m long, 9 support discs; motor diameter 60 mm at the right end |
| Mounting | Steel wall plate with top hooks; the lower edge of the chassis clips onto it | installation manuals | Galvanised plate 1.5 mm with a folded hook and a lower lip |

## 2. Audit before round 3

- **Rig.** The `vane` bone sat at (0.065, 0.015), angle 0, length 0.125. The flap was a 30 mm thick slab filling a rectangular notch 120 x 30 mm cut into the underside. A real flap is a thin curved blade that closes the lower front radius flush.
- **2D.**
  - The side elevation showed the top grille, its slats and the display, although a true side view sees only the side cover.
  - The plan showed the display, which is not visible from above.
  - The front view drew an intake grille strip on the front face; the grille is on top.
  - The vane was drawn 30 mm at 1:50, 34 mm at 1:100 and 40 mm at 1:200 (real: 4 to 8 mm).
- **Body.** One extrude, two box end caps and a flat grille plate with capsule slats. There was no front panel, no discharge mouth and no louvres. qa-visual graded the body B.
- **Thermal.**
  - The casing was the solid body, so the cutaway showed a thick hatched wall.
  - The coil and fan were missing.
  - The emitter was a 0.1 m box floating inside the casing.
  - 1:50, 1:100 and 1:200 were copies. qa-visual graded thermal C at every scale.
- **Structure.** A solid block plus a box for the back plate, with one ellipsoid mass. qa-visual graded structure C.
- **Areas.** Casing 0.73 m2 against 0.905 m2 measured on the honest shell. Vane 0.1243 m2 against 0.141 m2 measured.

## 3. Root fixes (catalog outside `volumes`)

| Member | Old | New |
|---|---|---|
| `rig.vane` | (0.065, 0.015), 0 deg, 0.125 m, com (0.06, 0) | Pivot (0.1147, 0.0058) at the flap's rear edge, 31 deg (the closed chord), 0.079 m, com (0.033, -0.005). Title "Discharge flap". Limits stay -55 to 0; pose offsets are unchanged |
| `rig.unit` title | casing with wall plate | casing with coil, fan and chassis on the wall plate |
| `anchors.jet-outlet` | (0.13, 0.02) | (0.148, 0.022), the centre of the discharge mouth. Hook and rest titles name the wall plate |
| `materials` | 3 | Added `aluminium-fin`, `san-gf`, `galvanised-steel` |
| `thermal.regions` | 3 | Added hidden `evaporator`: hydronic on `cooling`, evaporating at 7 degC at level 1 and 12 degC at 0.4, area 9.6 m2 (exchange), layer `fins`. Added hidden `fan`: passive, 0.69 m2 blade area (exchange), layer `blades` |
| `casing.areaM2` | 0.73 | 0.9: the measured total outer area of the 1:50 shell, factor 1.23, within C16 |
| `discharge-vane.areaM2` | 0.1243 | 0.141: the measured area of the 1:50 flap, factor 1.13 |
| `poses[*].thermal.regions` | 3 | Added `evaporator` and `fan` (exposed 1, sunlit 0) |
| `thermal.assumptions`, `structure.assumptions` | | One sentence added on the hidden regions; the wall plate is named |
| `provenance` | 5 references | Added `mitsubishi-msz-ap25vg` and `daikin-ftxm25n`; notes rewritten (en, de) |
| `summary` | | Now names the wall plate, coil, fan and flap |
| `representations.50/100/200` | see audit | Redrawn as the true views of each scale's body (E1); decisions rewritten |
| `representations.500` | Reason: vane 0.03 m | Reason: unit 0.58 mm tall, flap 0.016 mm; `limitingSizeM` 0.008. Still unavailable |

`dimensions`, masses, poses, clips and the emitter values are unchanged.

## 4. What was modelled

### 1:50: body 11 (41 with repeats), thermal 21, structure 10

- **Body:**
  - Casing extrude with the grille recess, the panel pocket and the discharge mouth.
  - 10 mm front panel with edge chamfers.
  - Side covers: slab plus a tapered rim (0.92) on each side.
  - Grille: filter texture, 5 bars, 9 ribs.
  - 17 swing louvres (texture).
  - The curved flap (8 mm to 4 mm, round edges) on bone `vane`.
  - Display window.
- **Thermal:**
  - 2 mm ABS shell built from panels: back, two top strips, grille-end fillers, rounded top edge, panel, lower bezel, underside, the rear wall and roof of the duct, flap-end fillers, and two end plates. Inside faces are marked `sectionFaces`.
  - Grille plate and the vane reference.
  - Hidden: the folded coil with two tube sheets, the fan cage, 9 discs and the motor.
  - Cold-jet outlet patch in the mouth.
- **Structure:**
  - Galvanised wall plate with a folded hook and lip.
  - ABS chassis (back wall and base).
  - Two side frames with openings above and behind the fan.
  - Drain-pan beam.
  - Flap member (reference) on two 8 mm pivot pins.
  - Hook pin support and wall roller support.
  - Masses at the unit's centre of mass and the flap's centre of mass.

### 1:100: body 7, thermal 14, structure 9

- **Body:** casing with a simple mouth, 15 mm panel, tapered side covers, the grille as a dark field, the flap at 15 mm, and the display.
- **Thermal:**
  - Shell of 5 panels plus 2 end plates.
  - Grille plate and grille-end fillers.
  - Coil band, a fan drum with motor, and the jet patch.
- **Structure:** plate with 15 mm folds, chassis, 15 mm side frames, flap on 15 mm pins, supports and two masses.

### 1:200: body 4 (5), thermal 11, structure 7

- **Body:** a D casing, square side covers, the grille field and a 30 mm flap.
- **Thermal:** back, top, front band, underside, end plates, grille, flap, coil band, fan cylinder and jet patch.
- **Structure:**
  - One 30 mm back member.
  - 30 mm side frames, and the flap on 30 mm pins.
  - Supports and the unit mass.

1:500 stays unavailable. Decisions for each scale are three bilingual sentences.

## 5. Checks

- `$BUN volumes-check.ts appliances/split-ac-indoor-unit --product`: **PASS** (5.8 s).
  - IoU 1:50 (elevation / front / plan): 1.000 / 0.999 / 0.999. IoU at 1:100 and 1:200: 0.999 to 1.000. p95 is 0 at every scale.
  - Area deviation at 1:50: casing 0.5 %, grille 4.4 %, flap 0.07 %. At 1:100 every region is under 5 %. The 1:200 total is 4.6 %.
  - Feature, monotonic, cross-scale, copied, structure coverage and bones are all ok.
- `$BUN r3-validate.ts appliances/split-ac-indoor-unit --catalog-only --volumes`: `VALID ... schema 0, validateAsset 0, checkVolumes 0`.

## 6. What the renders show (opened with the Read tool)

Seen:
- `r3-sheet` body at 1:50 and 1:100 (six views each);
- 1:50 body in pose `cooling`, right side only;
- thermal cut at 1:50;
- structure at 1:50, before and after adding the side-frame openings;
- `volumes-render` sheets at 1:50, 1:100 and 1:200 (overlays, body, thermal cutaway, structure);
- `r3-2d` at 1:50 in pose `cooling`.

Rendered but not opened: `r3-sheet` body, thermal and structure at 1:200; thermal and structure at 1:100; `r3-2d` at 1:50 off, 1:100 and 1:200. Their content was judged through the `volumes-render` sheets of those scales.

What the views show:
- **Front:**
  - A long white casing with the panel seams top and bottom, the side-cover joints and rims, and the display at the right.
  - The flap reads as a band along the underside.
- **Right and left side:**
  - The D: flat back, flat top, small top-front radius, and a large lower radius.
  - The bevel line of the cover rim.
  - In `cooling` the flap swings down about its rear edge to below the casing and opens the mouth in front of it, like a lower lip.
- **Back:** a plain flat back with the cover rims.
- **Top:** the grille recess with five bars and nine ribs over the dark filter; the cover rims at both ends.
- **Three-quarter:** reads at once as a high-wall split unit.
- **Thermal cut:**
  - The thin red skin around a hollow casing.
  - The yellow folded coil arches over the blue fan drum, and the discharge mouth sits under the fan.
  - The far side cover's inside face is hatched as a marked face.
- **Structure:**
  - From the three-quarter view, a blue tub: chassis back and base with two side frames with openings.
  - The drain-pan beam, the wall plate at the back, green supports, the flap pivots and the magenta mass.
- **1:100 and 1:200:** read as the same unit with merged detail. At 1:200 the flap's 30 mm thickness shows as a groove along the lower front.

## 7. Still imperfect

- **No C31 flood.** The product has no cavity flood, so the thermal shell is built from panels with marked inside faces (as mvhr did). The far end plate's marked inside face is drawn hatched in the cut view.
- **Hidden evaporator model.** It is `hydronic` because no other model can hold the refrigerant temperature. The circuit rule needs supply at or above return, so evaporation is written isothermal (7 and 12 degC) and superheat is not represented. Whether the solver's hydronic heat double-counts with the cold-jet emitter's sensible cooling is not checked here; the hidden text says the cooling is carried by the jet.
- **Front panel.** Flat with chamfered edges, not curved. Many real panels are flat, so this is a choice, not an error.
- **Louvres.** They are static, with no left-right swing bone.
- **Not modelled.** Refrigerant pipes and the drain hose: they leave through the wall.
- **Display.** A plain glazing plate, without an LED or receiver window.
- **Side elevation thermal view.** The side elevation merges `intake-grille` into the casing outline (`regions`), because `region-undrawn` requires every visible region in the elevation while a true side view cannot see the top grille. The 2D thermal side view therefore colours the casing and grille as one merged form.
- **Areas.** The `casing` and `discharge-vane` corrections are within the factor of 1.5 allowed by C16. They are listed here and not in `area-review.md`.

Sources: [Mitsubishi Electric MSZ-AP series](https://www.mitsubishielectric.com.au/product/msz-ap25-80vgd-split-system-air-conditioner/), [Daikin FTXM-N table](https://www.daikin-ce.com/en_us/products/product.table.html/FTXM-N.html).

## Round 5 handcraft (2026-11-21)

Scripts: `r5-split-ac-indoor-unit-500.py` (1:500 anew and the 1:50 thermal grille), `r5-split-ac-indoor-unit-area.py` (one areaM2 root edit).

### Reference and model vs reference

| Quantity | Reference (Mitsubishi MSZ-AP25VG, Daikin FTXM25N) | Model | Deviation |
|---|---|---|---|
| H x W x D | 299 x 798 x 219 / 294 x 811 x 272 mm | 290 x 850 x 210 mm (class value) | -3 % / +6.5 % / -4 % against the Mitsubishi |
| Lower front radius | large radius carrying the flap | R 115 mm | ok |
| Flap | thin curved blade 4-8 mm | 8 -> 4 mm (1:50), 8 -> 5 mm (1:500) | ok |
| Intake grille | top, open lattice | 5 bars, 9 ribs; thermal grille now open between them | ok |

### Grades (body / thermal / structure)

| Scale | Before | After |
|---|---|---|
| 1:50 | B+ / A- (check red: closed grille plate sealed the casing, flap inner face uncounted, vane area -46 %) / A- | A- / A- / A- |
| 1:100 | A- / B+ / B+ | A- / B+ / B+ (unchanged) |
| 1:200 | B+ / B+ / B+ | B+ / B+ / B+ (unchanged) |
| 1:500 | B- (round-1 polygon profile, wedge vane) / B (copies of 1:200 shells) / B | A- / A- / A- |

Opened: body 1:50, 1:100, 1:200, 1:500 before; 1:500 body (sheet and from below), thermal cut and structure after; pose `swing-low` (right). Not opened in this round: 1:50-1:200 thermal and structure sheets (unchanged except the 1:50 grille), 2D.

### Changes

- 1:50 thermal `th-grille`: the 2 mm plate with 32 openings of 13.5 x 82.5 mm between the body's bars and ribs; the room air now reaches coil, fan and the inner face of the closed flap (C31), so `discharge-vane` measures 0.1409 against 0.141 (was 0.0755).
- Root: `thermal.regions.intake-grille.areaM2` 0.0829 -> 0.11 (measured 0.1105 on the open lattice, both faces and opening edges; factor 1.33, within C16).
- 1:500 anew: one casing of the true D profile in 15-degree steps with a 9 mm seat for the flap, the closed curved flap flush in it; thermal casing by reference (back face against the wall plate marked), grille plate, flap, hidden coil and fan, jet patch behind the seat; structure back member, two D side frames, flap, hook, rest, mass.

### Checks

`volumes-check.ts --product` PASS (was FAIL at 1:50); `r3-validate.ts --volumes` VALID; spliced.

### Remaining

- Width 0.85 m is 6.5 % over the 0.798 m reference unit (the asset's class value, kept by round 3).
- The area reviewers should note the `intake-grille` correction (0.0829 -> 0.11) against their ACCEPT of 0.0829.
