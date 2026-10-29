# Components content notes (category `components`)

File: `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🧱️components/🔣️.json` (8 assets). `check` prints ✓ for all eight.

## Hand-checked statics vs solver

Hand values use the authored data (masses = section × density; the solver uses the same). W = total weight. R = support reaction. M = moment at the root/support (CCW, reaction). w = tip or midspan deflection.

| Asset / state | Hand check | Solver |
|---|---|---|
| balcony self-weight | R = 7357.5 + 68.0 = 7425.5 N; M = 7357.5×0.75 + 68.0×1.5 = 5620.1 N·m; w ≈ 1.41e-4 + 3.5e-6 m | R 7425.5, M 5620.1, w 1.446e-4 |
| balcony distributed-live-load | R = 13425.5; M = 5620.1 + 4500 = 10120.1 | 13425.5 / 10120.1 / w 2.593e-4 |
| balcony point-load-tip | Fx −1000; Fy 10425.5; M = 5620.1 + 4500 (crowd) + 1200 (barrier at 1.2 m) = 11320.1 | −1000 / 10425.5 / 11320.1 |
| balcony moving load, carrier at 0.75 m | M = 5620.1 + 3000×0.75 + 1200 = 9070.1 | 9070.1 |
| glulam self-weight | R = 3164.3/2 = 1582.2; M = wL²/8 = 3164.3; w = 5wL⁴/384EI = 0.631 mm | 1582.2 / 3164.3 / 0.631 mm |
| glulam distributed | R = 17582.2; M = 35164.3; w = 7.05 mm | 17582.2 / 35164.3 / 7.06 mm |
| glulam midspan 2 kN | R = 2582.2; M = 3164.3 + 4000 = 7164.3; w = 0.631 + PL³/48EI = 1.27 mm | 2582.2 / 7164.3 / 1.27 mm |
| column axial | Fy_base = 250000 + 1804.4 = 251804.4; M = 0 | 251804.4 / 0 |
| column eccentric (e = 0.1 m) | M_base = P·e = 25000 N·m | 25000 |
| column lateral wind (320 N/m) | Fx = −960; M = 320×3²/2 = 1440; tip w = qL⁴/8EI = 0.270 mm | −960 / 1440 / 0.270 mm |
| stair self-weight | each support 1630.4/2 = 815.2; M_mid = 1630.4×3.6/8 = 733.7 | 815.2 / 733.7 |
| stair crowd (1.80 kN/m horizontal) | total 6480 N, each R = 4055.2; M_mid = 6480×3.6/8 + 733.7 = 3649.7 | 4055.5 / 3650.0 |
| stair person 400 N at mid | R = (1630.4+400)/2 = 1015.2; M_mid = 400×3.6/4 + 733.7 = 1093.7 | 1015.2 / 1093.7 |
| door (W = 851.0 N; track L = 3.0) closed | R_E = 851.0×1.5/3 = 425.5 each | 425.5 / 425.5 |
| door half-open (x = 2.0) | R_E = (588.6×2.0 + 262.4×1.5)/3 = 523.6; R_W = 327.4 | 327.4 / 523.6 |
| door open (x = 2.5) | R_E = (588.6×2.5 + 393.6)/3 = 621.7; R_W = 229.3 | 229.3 / 621.7 |
| tilt tilted 15° | stay H = W·0.6·sin15° / (1.2·cos15°) = 95.6 | 95.7 |
| tilt fully tilted 30° + suction 720 N/m | stay H = 705.2; hinge Fx = 43.2; Fy = 1146.2 | 705.0 / 43.2 / 1146.2 |
| balustrade self-weight | R = 580.1 | 580.1 |
| balustrade handrail 1 kN at 1.1 m | Fx = −1000; M = 1100; tip w = PL³/3EI = 7.65 mm | −1000 / 1100 / 7.64 mm |
| balustrade impact 1 kN at 0.9 m | M = 900 | 900 |
| balustrade wind 800 N/m over 1.1 m | Fx = −880; M = 800×1.1²/2 = 484 | −880 / 484 |
| canopy self-weight | T = 851.3×2.75/2.4 = 975.6; pin Fy = 851.3 − 0.6T = 266.0; pin Fx = 0.8T = 780.5 | T 975.4 (tie components −780.4, 585.3); pin 780.4 / 266.0 |
| canopy snow 750 N/m over 5.5 m | T = (4125+851.3)×2.75/2.4 = 5702; pin Fx = 4561.6; pin Fy = 1555.1 | tie 5702 (−4561.6, 3421.2); pin 4561.6 / 1555.1 |
| canopy uplift 500 N/m | tie slack (T would be −1797 N); net up 1898.7 N; residual M = 1898.7×2.75 = 5221 N·m | tie inactive; residual M 5221; Unbalanced note |

Residuals are zero to rounding everywhere except the canopy uplift pose (mechanism, see below).

## Deviations from the brief and manifest (for the coordinator)

- Pose and clip ids: sliding door uses `closed`, `half-open`, `open` (the brief's names) instead of `closed-latched`, `half-open-in-wind`, `fully-open-parked`. Wind is out of plane in an elevation model, so the wind clip `wind-gust` became `slide-open`.
- Balustrade adds a fourth pose `wind-pressure` (brief lists wind). Manifest needs that entry.
- Tilt window hinge: the brief says bottom hinge; the manifest distinction says top-hung. Built as bottom hinge; the coordinator must reconcile.
- Tie rod: built as a tension-only `contact` with `support: "contact"` along the rod (normal 143.13° toward the wall top), not a bilateral roller. A roller would report compression under uplift.
- Dimensions follow the authored geometry (the check only compares them to the evaluated bounds), so the manifest needs syncing: balcony 1.3 × 1.8 (manifest 1.1 × 2.2); steel column 3.0 m height (brief; manifest 4.0); stair 3.02 × 3.65; sliding door 2.15 × 3.1 (manifest 2.1 × 1.0); tilt window 1.2 × 0.3 (manifest 0.9 wide); balustrade 1.1 × 0.3 (manifest width 2.0, the panel length is not drawn in section); canopy 3.0 × 6.0 (matches).
- The balustrade is drawn as a section (the glass normal load lies in the drawing plane), not a face elevation.
- Sliding door: the leaf is hung from its centre carriage (prismatic joint), so the track carries a moving vertical load; wind is not represented. No latch.
- Canopy uplift: under net uplift the tie goes slack and the canopy has no rotational restraint, so the pose is a mechanism (moment residual 5221 N·m, reported by the solver, not hidden). A hold-down or a fixed wall connection is a design decision not made here.

## Thermal, joints and masses

- Thermal: passive surfaces (concrete ε 0.9, steel 0.8–0.85, glass ε 0.84, timber 0.9) and glazing regions; environment 20 °C, 50 %, 1 m/s, 500 W/m² solar. No emitters (none meaningful for components). All regions are bound by shapes. Areas are authored estimates.
- Carrier bones (massless, prismatic) move point loads along the member; they are not physical joints.
- Real moving parts only: sliding-door carriage (prismatic), tilt sash (revolute 0–30°).
- Materials: C24 timber (EN 338), GL24h (EN 14080), S235/S355 (EN 10025-2), HEB 200/HEA 160/IPE 160 (EN 10365), RHS 100×50×4 and SHS 50×50×4 (EN 10219), C30/37 (EN 1992-1-1), E for glass 70 GPa (EN 572). Imposed loads and densities: EN 1991-1-1. Snow EN 1991-1-3, wind EN 1991-1-4.

## Uncertainties (not measured)

- Imposed loads per category are the brief's values (balcony 4 kN/m², stairs 3 kN/m², floor 2 kN/m², barrier 1.0 kN/m); the exact EN 1991-1-1 category and table entries should be verified.
- Wind pressures (0.8 kN/m² suction and pressure, 0.5 kN/m² uplift), snow (0.75 kN/m²) and the crowd impact (1.0 kN at 0.9 m) are authored estimates.
- Emissivity and solar absorptance values are typical literature values, not re-checked against the cited edition.
- Section properties are for standard profiles as listed; the sash and glass laminate sections are equivalent sections.
- Glass laminate thickness is simplified to the sum of the two lites.
- The stair stringer is one of a pair; the treads are a uniform share.

## Verification done

- `repo tools exec bun -- …/🩺️check/🟦️.ts --file …/🧱️components/🔣️.json` → ✓ for all eight assets.
- Solver dump and hand table above (scratch script in `🧰️scripts/`).
- Previews: 25 poses × {light-solid, forces, moments} at 1:100, reviewed for the tilt window and tied canopy forces. Not yet reviewed: other sheets, all seven views, 1:50 and 1:200, and per-asset visual review.
- Preview tooling gap: `🖨️preview` is empty and `T/depiction.md` does not exist; a scratch SVG serialiser was used. Known layout issue: force labels overlap on the canopy and window views (depiction module labelling, not data).
