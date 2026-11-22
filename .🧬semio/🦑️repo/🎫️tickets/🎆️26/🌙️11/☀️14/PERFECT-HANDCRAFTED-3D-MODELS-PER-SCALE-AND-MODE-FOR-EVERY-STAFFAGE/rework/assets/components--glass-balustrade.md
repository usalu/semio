# components/glass-balustrade — rework report

## Pass 1 (second round), 2026-11-22

State found: no report existed. Source `volumes/components/glass-balustrade.json` and catalog `volumes` were identical (an earlier pass had rebuilt the solids with `rework-glass-balustrade-volumes.py` at 13:52 and published; baseline `VALID`). Body solids were judged good by rendering; rig, masses, loads and structure lists were not.

### Reference table (2.0 m panel, DIN 18008-4 category B, clamped base with continuous handrail)

| Part | Reality | Model before | Model now |
|---|---|---|---|
| Laminate | 2 x 10 mm + 1.52 mm PVB, 21.5 mm; 1.96 x 1.055 m; 2500 kg/m3 -> 103.2 kg glass + 3.3 kg PVB | geometry right; mass 59.13 kg (a 1 m strip of 1.1 m) | geometry unchanged; 106.5 kg |
| Clamp | cheeks >= 12 mm steel, clamping height >= 100 mm, elastomer linings, through bolts <= 300 mm | 15 x 110 mm cheeks, 10 mm linings, 7 x M12 at 300 mm | unchanged |
| M12 hardware | head 7.5 mm, nut 10.8 mm, 19 mm across flats (21.9 across corners), washer 24 x 2.5 mm | head, nut, sleeve right; no washers | unchanged; washers left out (2.5 mm is below the 7.5 mm feature and no sheet) |
| Base plate | 300 x 15 mm, anchors M12 in two rows at 400 mm; 70.7 kg | right | unchanged |
| Cheeks | 2 x 15 x 110 x 2000 mm steel = 51.8 kg | right, no mass | mass on the shoe bone |
| Handrail | tube 42.4 x 2 mm slotted over the glass edge, U gasket, end caps; about 3.2 kg/m slotted -> 6.9 kg with gasket and caps | right, no mass | mass on the panel bone |
| Total mass | about 243 kg | 59.13 kg | 242.8 kg (shoe 129.4 kg at 35 mm, panel 113.4 kg at 0.51 m above the bolt line) |
| Static system | glass cantilever clamped in the shoe, shoe fixed to the floor by the plate on its anchors | one bone `panel` from the floor, plate + everything on it | `shoe` (root, fixed, 0 to 125 mm) -> `panel` (fixed joint at the bolt line 75 mm, 1.025 m to the handrail top) |
| Loads | 1.0 kN/m line load x 2.0 m = 2.0 kN; wind 0.8 kN/m2 x 2.0 m = 1.6 kN per m height on the glass above the shoe; impact 1.0 kN at 0.9 m | 1.0 kN, 0.8 kN/m from the floor (1 m strip against a 2 m model) | 2.0 kN at 1.1 m, 1.6 kN/m from 0.125 to 1.1 m, impact at 0.9 m |
| Sections | laminate 2.0 m x 21.5 mm: A 0.043 m2, I 1.656e-6 m4; welded cheek 2.0 m x 15 mm: A 0.03 m2, I 5.625e-7 m4 | 1 m strip | as reality |

### Defects found

1. Rig: a single bone carried glass, handrail and the whole steel shoe; mass 59 kg for an assembly of 243 kg, placed at mid height; loads and section of a 1 m strip on a 2 m model.
2. Structure lists: the cheeks (the cantilever plates of the clamp) were `joint`; at 1:200 and 1:500 the whole shoe was `support`, so the upstand that carries the clamping moment did not read as a member.
3. Body, thermal at all scales: no defect that I would change (rendered six views, cut, three-quarter). Washers are missing by decision of the feature size.

### Rebuilt

- `rework-glass-balustrade-rig.py` (hand numbers): bones `shoe` and `panel` with masses, sections, titles (en, de); anchors, loads, pose descriptions, `structure.assumptions`, `dimensions.massKg`; every solid of glass and handrail re-expressed in the panel frame (minus 75 mm along the bone) at all four scales, shoe solids bound to `shoe`; elevation shapes of glass and handrail moved into the panel frame at 1:50, 1:100, 1:200, shoe shapes bound to `shoe`, symbol shapes bound to the root; cheeks are members of the shoe; a mass marker per bone at its centre of gravity; a rig sentence added to `decisions` of every scale.
- `rework-glass-balustrade-structure.py`: structure lists of 1:200 and 1:500 split by hand into the clamp upstand (member of the shoe) and the 300 x 15 mm base plate (support, declared sheet).
- Poses and clips are load cases without bone motion; the real deflection at the handrail is about 5 mm (0.1 mm on paper at 1:50), so the bones stay at rest by decision.

### Validation

`r3-validate.ts components/glass-balustrade --catalog-only --volumes`: `VALID … schema 0, validateAsset 0, checkVolumes 0` after the last publish.

### Rendered and read after the last change

1:50 structure (six views), 1:50 thermal cut, 1:100 body, 1:200 structure, 1:500 body, 1:500 structure three-quarter, clip `impact-pulse` t 0.4, 2D 1:50 (pose `wind-pressure`) and 2D 1:200. Rendered but not read: poses `self-weight-unloaded`, `crowd-impact-point`, `wind-pressure` (3D), clip `handrail-push` t 0.5 and 3, `impact-pulse` t 1.5, 1:100 thermal and structure, 1:200 body and thermal, 1:500 thermal, 2D 1:100 (none of these has bone motion or changed solids other than the frame shift that the read sheets show).

### Remains imperfect

- 1:500 body and thermal are the same three solids as 1:200 (only `slender` differs); the upstand (71.5 mm) is just under the 75 mm feature. A reviewer may want the shoe as one block at 1:500.
- No washers under bolt heads, nuts and anchor nuts at 1:50.
- Anchor studs end at the floor plane (the embedded length is outside the model).
- The images listed as "not read" above.

### Fundamental issues

- `r3-2d.ts` failed for some minutes with `Cannot find module …/🧰️framework/🔨️modules/🧊️3d/🗿️model/🧮️math/🎯️portable/🟦️.ts` imported by `S/🔨️modules/🎨️depiction/🌊️flow-view/📐️model/🟦️.ts` (concurrent work on the flow view); it worked again later without my doing.
- `r3-tools.md` line 48 says front and plan are rest coordinates, but does not say next to it that elevation shapes are bone-local with x along the bone; a child bone under an upright root needs the shift along x, not y (cost one validation run).
