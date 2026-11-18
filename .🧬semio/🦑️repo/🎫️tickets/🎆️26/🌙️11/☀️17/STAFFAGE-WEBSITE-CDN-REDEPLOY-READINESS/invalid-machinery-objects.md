# Invalid Machinery, Components and Objects Assets

All eight assets validate with zero issues (`validate-machinery-objects.ts`, a file-argument variant of `validate-all.ts`).

| Asset | Issue | Fix |
|---|---|---|
| machinery/excavator | `shape-role-undrawn`: `slew-ring` has role detail, pen null at 1:500 | The ring (0.28 mm on paper) is dropped from the 1:500 elevation and front, the level-of-detail convention of the detail pen being null |
| machinery/plate-heat-exchanger | height 0.640 vs 0.62 m at 1:200 | The bypass damper at 1:200 was 0.04 m thick centred on its hinge at 0.62 m (top 0.64 m); it is now 0.02 m thick below the hinge like 1:100 (elevation and front) |
| machinery/scissor-lift | 12 `volumes-structure-coverage`: four declared supports without support solids | One rubber `support` solid per contact anchor (`contact-f-l|f-r|r-l|r-r`) on the chassis at 1:50, 1:100, 1:200, at the tyre centres (x = +-0.55, z = +-0.325 m), contact patch 0.1 x 0.16 m, thickness 0.15 mm on paper |
| machinery/tower-crane | `lateral-ambiguous`: flat jib and counter-jib lattice paths in the elevation at 1:100 and 1:200 | Replaced by solids: chords, rails, zigzag diagonal walls and end frames of the jib, chords and diagonal walls of the counter-jib, each a filled closed shape with an extruded `w` solid over its true lateral band (left and right) |
| machinery/wheel-loader | 16 `volume-anchor`: support solids name anchors absent from `structure.supports` | `structure.supports` now lists the four tyre contact anchors the solids and anchors already define |
| components/sectional-garage-door | front and plan lateral 2.560 vs 2.5 m at 1:200 | The bearing flags at 1:200 were widened outwards to 0.03 m beyond the tracks (+-1.28 m); they now sit on the track plane (+-1.25 m) |
| objects/chalkboard | height 1.225 vs 1.26 m at 1:500 | The tray line sat at its mid height 0.035 m, hiding the lowest 35 mm; it is now drawn at the base y = 0 |
| objects/cooking-pot | `shape-role-undrawn`: `wall-far` role detail, pen null at 1:100 and 1:200 | The far wall is a sectioned part of the pot, so its role is body at those scales (1:50 keeps detail) |

Validator untouched. Edits are textual, so the file layouts are preserved.
