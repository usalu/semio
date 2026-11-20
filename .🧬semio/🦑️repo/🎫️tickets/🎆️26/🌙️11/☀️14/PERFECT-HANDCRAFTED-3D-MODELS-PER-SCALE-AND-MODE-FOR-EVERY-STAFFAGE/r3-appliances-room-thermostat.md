# Round 3 — appliances/room-thermostat

Resumed on 2026-11-20 from the interrupted session (scripts `r3-room-thermostat-kit.py`, `-model.py`, `-roots.py`; root fixes were already in the catalog). This session: compared catalog, source and script output, replaced the bulk pass's 1:200 and 1:500 lists by handcrafted ones, reworked the 1:50 index, the 1:100 housing and the 1:100 structure text, looked, checked, spliced.

## Reference

| measure | value | source |
|---|---|---|
| face | 90 x 90 mm class (84 x 84 Danfoss, 75 x 75 Eberle) | Danfoss RET1000 data sheet; Eberle RTR-E 6121 product data (both in `provenance.references`) |
| depth | housing 15 mm, 27 mm with dial (RET1000: 28 to 37 mm, RTR-E: 25.5 mm) | same |
| fixing | two screws 60 mm apart on a 60 mm flush box | DIN 49073-1:2010 |
| corner radius | 8 mm, face-edge radius 2 mm | own knowledge of the product class |
| construction | ABS base plate on the wall, snap-on cover with a 1.5 mm shadow gap, 2 mm walls | own knowledge |
| dial | 42 mm, 12 mm proud, 24 finger flutes, 270 degrees for 5 to 30 degrees Celsius, on a 6 mm potentiometer spindle | EN 60730-1 class device, own knowledge |
| display, lamp | 36 x 21 mm lens, 7 mm lamp dome | asset drawing, own knowledge |
| mass | 0.135 kg (housing 0.11, dial 0.025) | authored estimate, in the range of the two products |

## Audit and root fixes (previous session, verified in the catalog today)

- Rig: `housing` now runs from the lower to the upper fixing screw (joint at y -0.03, length 0.06, was a centre bone), `dial` sits on the spindle axis 14 mm below the centre, length 0.021, revolute -135 to 135 degrees.
- Anchors: `wall-screw-top` (x 0.06) and `wall-screw-bottom` (x 0), both at the wall plane z 0.0135; upper screw pin, lower one roller in its slot.
- 2D: 1:50 shapes moved into the new housing frame, front and plan edge radius 2 mm; 1:100 redrawn (housing r 4 mm, display 36 x 21 mm, dial disc; front and plan at the true depths 15 and 12 mm). 1:200 and 1:500 stay unavailable for print (0.45 and 0.18 mm on paper).
- Thermal: `dial.areaM2` 0.0028 (was 0.00243, factor 1.15, C16). Structure assumption and provenance notes and references rewritten.
- `dimensions` unchanged (0.09 x 0.09 x 0.027 m, 0.135 kg): they agree with the references.
- Bulk pass of 03:48 found today: it had turned 1:200 and 1:500 into authored digital scales with drafted lists (12-gon dial, generic one-sentence decisions, a 2 mm plate as structure). Kept: the fact that both scales are authored. Replaced: every solid and text of both.

## Model per scale

| scale | body | thermal | structure |
|---|---|---|---|
| 1:50 | 6: housing loft (2 mm face radius, cover skirt, shadow gap, base plate), display lens, lamp dome, fluted grip ring, conical cap, index tongue (new: tapered from 11 mm to a rounded 4 mm tip, was a plain bar) | 11: hollow 2 mm casing (face with window, four skirt walls, base), lens, lamp, three knob solids | 7: base plate, round pin and slotted roller at the screws, 7.5 mm spindle, potentiometer joint, circuit-board mass, knob-hub mass |
| 1:100 | 3: prism with 4 mm corners (was a loft whose wall cap shaded as a pyramid), display plate, conical dial | 9: hollow casing with square-cut sections, lens, lamp, dial | 4: 15 mm bar screw to screw, two 15 mm bosses, 15 mm shaft (feature rule, see below) |
| 1:200 | 3: prism with one 6 mm chamfer per corner, display plate, 16-sided dial cylinder | 4: solid casing (cavity under the feature, C30), display, flat lamp dome, dial | 4: 20 mm strip at the wall, two square screw blocks, 10 mm shaft |
| 1:500 | 2: square slab, 12-sided dial boss | 4: slab, display patch, lamp patch, boss | 4: 12 mm spine, two flat pads, square stub |

Final check: `volumes-check.ts appliances/room-thermostat --product` PASS; `r3-validate.ts appliances/room-thermostat --catalog-only --volumes` VALID (schema 0, validateAsset 0, checkVolumes 0). Spliced with `volumes-insert.py` (all four scales authored).

## What I saw

- 1:50 body (`r3-sheet`, six views): face-on a rounded square with dark lens, lamp dome top right, fluted knob with the index pointing to the upper right in pose `calling`; from the side the cover, shadow gap and narrower base plate, knob 12 mm proud with its cap step; back flat. Reads as a dial thermostat at first sight.
- 1:50 thermal cut: hollow red shell with the air space, green lens in its rebate, lamp ring, knob solid. 1:50 structure: base plate with green pin and slot on the screw axis, magenta board above the orange potentiometer, spindle and hub mass inside the knob ghost.
- 1:100 body (sheet and `volumes-render` overlay): prism, display, conical dial; overlay IoU 1.000 / 1.000 / 0.955 (the plan draws the dial as a rectangle, the cone leaves two red slivers).
- 1:200 and 1:500 (own six-view sheets, body, thermal, structure): chamfered plate with display and round knob; square slab with knob; structure strip or spine with green screw blocks inside the ghost, shaft ending on it; back flat.

## Still imperfect

- 1:100 structure is a 15 mm bar, bosses and shaft without joint and mass: `volumes-feature` refuses every thinner structure solid in this 15 mm deep device (request with numbers in `author-requests.md`).
- 1:50 thermal: the knob is modelled solid; a hollow knob made the validator count its inner faces (dial area +94 %), so the three body solids are reused.
- No hidden electronics region: the thermal data declares the electronics heat on the casing region, and adding regions would change physical data.
- Ventilation slots, scale ticks and screws heads are below the 7.5 mm feature and absent at every scale.
- `r3-sheet.ts` and `volumes-render.ts` fail at the digital scales (`tool-requests.md`); 1:200 and 1:500 were judged on `r3-room-thermostat-sheet.ts` without rig marks. Poses other than `calling` were rendered only at 1:50 (`set-high`, three-quarter), not opened one by one at the coarse scales; the dial is rotationally near-symmetric there.

State: DONE (with the listed limits)
