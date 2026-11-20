# Round 3 — objects/drinking-fountain

State: DONE

Scripts (ship chain `sh r3-drinking-fountain-ship.sh`): `r3-drinking-fountain-kit.py` (geometry), `-model.py` (volumes), `-draw.py` (2D, decisions), `-roots.py` (rig, anchors, poses, texts), `-build.py` (candidate, source, catalog members), `-areas.json` (corrected areas). No earlier round-3 work existed; the catalog `volumes` equalled the source at the start.

## 1. Reference

No web search was made; the values are class knowledge of heritage cast-iron pedestal fountains (Murdock, Haws 3500 class, European Trinkbrunnen) and the asset's own references (`fountain-class`, EN 1717).

| item | real value | used |
|---|---|---|
| overall height / rim height | 0.95-1.05 m / 0.85-0.95 m | 1.000 m / 0.899 m (`dimensions` kept) |
| bowl | spun stainless 0.30-0.45 m, 80-120 mm deep, 1.2-1.5 mm, rolled rim | 0.400 m over a 12 mm bead, 103 mm deep, 1.5 mm |
| pedestal | stacked hollow castings 10-14 mm wall on a tie rod, base flange 0.35-0.45 m, shaft 0.15-0.20 m | flange 0.400 m x 22 mm, die 0.236 m, fluted shaft 0.184 to 0.160 m (16 flutes, 6 mm), 12 mm wall, M16 tie rod |
| anchors | 4 x M12 on the flange | 4 studs with hexagon nuts AF 19, bolt circle 0.362 m |
| spout | 22-28 mm chrome brass, free outlet at least 20 mm above the rim (EN 1717) | 28 mm goose-neck, bend radius 45 mm, mouth 40 degrees down, 57 mm above the rim |
| jet | 3 L/min through 12 mm gives 0.44 m/s | parabola from 0.44 m/s, 0.16 m fall, thinning 12 to 6 mm (continuity) |
| push button | 38 mm button in a 54-60 mm bezel, 5-10 mm travel | 38 mm, bezel 54 mm, boss 60 mm, 10 mm travel (pose value kept) |
| mass | 50-90 kg | 68 kg kept; hand sum of the castings about 64 kg against the rig's 62.3 kg |

## 2. Audit of the state before

- Rig: every bone had `angleDeg` 0, so the 0.74 m pedestal bone lay on the ground pointing ahead and the basin, bubbler and jet bones were horizontal stubs. The jet was an eligible structural member (0.12 m).
- Form: the pedestal was a plain cone 0.30 to 0.20 m with three wire collars, no die, no capital, no joints; the bowl a hemisphere balanced on the cone top; the goose-neck 55 mm thick and 145 mm high with the jet leaving upward at 60 degrees from a nozzle that points down; eight ball "bolts" on the flange edge; button a 40 x 50 mm ellipsoid on the bare cone.
- 2D: drawn to that form at every scale (cone with flute hatch over the whole pedestal, box neck at 1:500); in the front view the idle jet was drawn over the pedestal.
- Coarse scales: jet thickened to 17 / 30 / 75 mm, bubbler a 150 x 140 x 60 mm box at 1:500, structure by reference to the whole body.

## 3. Root fixes (catalog file)

| member | old | new |
|---|---|---|
| `rig` pedestal | (0, 0) 0 deg, 0.74 m | (0, 0) 90 deg, 0.79 m, com (0.3, 0) |
| `rig` basin | (0, 0.74) 0 deg, 0.12 | on pedestal (0.79, 0) 0 deg, 0.109 |
| `rig` bubbler | (-0.15, 0.12), 0.14 | on basin (0.015, 0.10), 0.195, com (0.1, -0.03) |
| `rig` button | (0.102, 0.66), 0.02, axis 180 | on pedestal (0.735, -0.096) -90 deg, 0.0225, axis 90 |
| `rig` jet | (0.105, 0.095), 0.12, travel -0.25 axis 90 | at the nozzle mouth, -130 deg (along the outlet), 0.04 m (no longer a structural member), travel -0.63 axis 0 (sinks into the hollow die) |
| `anchors` | nozzle (0.105, 0.095); lean-point at the bowl centre | nozzle at the mouth; lean-point on the front of the rim |
| `poses` | jet `dy` -0.25, button `dx` -0.01 | jet `dx` -0.63, button `dy` 0.01; two descriptions |
| `thermal.emitters[0].directionDeg` | 60 | -40 |
| `areaM2` basin-steel | 0.4 | 0.36 (measured 0.360) |
| `areaM2` bubbler-brass | 0.054 | 0.036 (measured 0.033; 0.036 is the C16 limit of factor 1.5) |
| `areaM2` water-jet | 0.0071 | 0.0048 (measured 0.0045; C16 limit 0.00473) |
| `areaM2` pedestal-cast | 0.87 | unchanged (measured 0.889) |
| texts | | summary, provenance notes and `fountain-class`, bone and region titles |
| `representations` | | geometry and decisions of all four scales redrawn from the kit (E1); flute hatch replaced by seven arris lines in true projection |

## 4. What is modelled

| scale | body | thermal | structure |
|---|---|---|---|
| 1:50 | 26: base casting (flange, torus, scotia, die, cornice), 4 anchor studs and nuts, access door with 4 screws, shaft base, fluted shaft (extrude with bore and taper), astragal, capital with cup, button boss, bezel, button, bowl sheet, rim bead, waste with clamp collar, strainer, neck base, union nut, goose-neck, nozzle tip, jet, 2 drops | 27: references; the five castings as filled outer skins (C31 workaround); outlet patch in the nozzle bore | 15: wall tube at the flute roots, tie rod, foot nut, top nut, yoke, studs, nuts, bowl, bead, waste, neck, base, nut, bedding pad, mass |
| 1:100 | 10 | 11 | 9 |
| 1:200 | 5 (solid turned pedestal) | 6 | 5 |
| 1:500 | 5 | 6 | 5 |

Final lines: `volumes-check.ts objects/drinking-fountain --product` PASS; `r3-validate.ts objects/drinking-fountain --volumes` VALID (schema 0, validateAsset 0, checkVolumes 0); `--catalog-only` VALID. Silhouette IoU 1:50 0.987 / 0.986 / 0.995, 1:500 0.970 / 0.993 / 0.992; bounds within 1.2 mm.

## 5. What the final renders show (all opened after the last change)

- 1:50 body, six views and a close three-quarter: reads at once as a cast pedestal fountain; mouldings, flutes with arrises, door on the right side with screws, anchor nuts on the flange, button in bezel and boss on the neck band, bowl with bead and strainer, goose-neck with nut and tip, thin jet landing beside the waste. Rig upright through the axis, jet bone along the outlet.
- 1:50 idle-sun: jet gone (inside the die), button 10 mm further out. 2D idle sheet: same.
- 1:50 thermal (cut): four regions, hollow section visible; structure (cut): wall tube, tie rod, yoke, nuts, mass in the die, green bedding.
- 1:100, 1:200, 1:500 body, thermal, structure: each a simpler turned form that stays recognisable; 1:500 neck is a small horn, not a block.
- 2D sheets of all four scales: true views of the bodies, clean outlines.

## 6. Still imperfect

- Supply pipe, valve and drain inside the pedestal are not modelled: no thermal region exists for them (the asset's assumption neglects the drain heat flow).
- The neck shank and the bowl seat pass through the bowl and into the cast cup without modelled holes (hidden inside the cup).
- The jet hides by sinking 0.63 m into the die (the asset's retract mechanism); during the clip it travels through the bowl.
- The jet keeps its true 12 to 6 mm at 1:100 to 1:500 by the `texture` flag (it is 14 diameters long, so not `slender`); the emitter patch is exempt from the feature rule through its `layer`.
- `bubbler-brass` and `water-jet` sit at the C16 limit, 7 % above the measured areas; values beyond factor 1.5 were not applied.
- The hollow castings count their cavity in the product's area measure (no C31 flood): filled thermal skins instead, noted in `tool-requests.md`.
