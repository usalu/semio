# Cover Axonometry With Staffage

Request (2026-11-24): "Die Axo auf der ersten Seite sollte nicht technisch sein, sondern noch staffage haben."

## Result

- `model.tex` has a second picture, `\EnergyCoverAxonometry`, used as `cover-content` in `de.content.tex` and `en.content.tex`. `\EnergyAxonometry` and `\EnergyAxonometryFigure` on the geometry page are untouched.
- Same isometric vectors, same south-east view, same geometry (box 3 × 9 × 2.6, south window, east sliding door, canopy). No dimensions, origin label, coordinate triad, leaders or dashed hidden edges.
- Drawn by hand in TikZ: lawn patch, shadows of house, canopy, person and deck chair (sun from south-south-west, clipped to the lawn), dark vertical cladding, roof edge, interior seen through both glazings (floor boards, west and north wall), wooden step with boards in front of the sliding door, grass tufts.
- Colours are only `semio-dark` mixed into `semio-light` (style `ink=<percent>`), so the picture follows the Print theme.

## Staffage and Its Source

All figures come from the staffage catalogue `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog`, exported with the catalogue's own command line (`🔨️modules/📤️export/🎬️cli/🟦️.ts`, view `light-line`, viewpoint `axonometric`, camera elevation 35.2644°, which is the isometric view of the sheet).

| Macro | Asset | Pose | Camera azimuth | Place in building coordinates |
|---|---|---|---|---|
| `\EnergyStaffagePerson` | `👤️humans/person-drinking-coffee` | `holding-cup` | −45° (faces south) | lawn in front of the step, (4.7, 4.6, 0) |
| `\EnergyStaffageSofa` | `🛋️furniture/three-seat-sofa` | `unoccupied` | 45° (faces east) | inside along the west wall, (0.55, 1.45, 0), clipped to the glazing |
| `\EnergyStaffageDeckChair` | `🛋️furniture/folding-deck-chair` | `unfolded` | −45° (faces south) | lawn, (5.6, 1.3, 0) |
| `\EnergyStaffageTree` | `🌳️vegetation/japanese-maple` | `summer-calm` | 45° | behind the house, (−1.6, 6.2, 0) |
| `\EnergyStaffageLavender` | `🌳️vegetation/lavender-bush` | `calm` | 45° | three along the east facade |
| `\EnergyStaffageBoxwood` | `🌳️vegetation/boxwood-ball` | `calm` | 45° | two at the north-east corner |
| `\EnergyStaffageCat` | `🐕️animals/domestic-cat` | `sitting-tail-wrapped` | 0° (faces the viewer) | on the step, (3.75, 4.2, 0.2) |

The camera azimuth runs from the asset's ahead axis towards its right; the sheet's camera stands in the south-east, so `azimuth = 135° − compass heading of the asset`.

Tried and dropped: `miscanthus-grass` (reads as a black tangle at this size and needs 57 kB of paths), `apple-tree` (too large), `potted-monstera` and `used-floor-lamp` (the window became cluttered).

## Reproducing the Macros

1. `sh <ticket>/cover-staffage-export.sh` from the repo root writes the SVG exports to `🗑️generated/staffage`.
2. `python <ticket>/cover-staffage-tikz.py` turns them into `🗑️generated/staffage/staffage.tex`: polylines in metres on the picture plane relative to the asset origin, simplified with Douglas-Peucker, two decimals, page-coloured occluders below the lines (none for the tree). `python <ticket>/cover-staffage-tikz.py contact` writes a contact sheet of all exports.

The staffage macros add about 51 kB to `model.tex`; the person is the largest with 13 kB.

## Verification

- `repo "🎓️teaching/🏛️architecture/⚡️energy/📝️assignment/🧪️tests/🟦️.ts"` exits 0. The test now requires `cover-content={\EnergyCoverAxonometry}` and `\EnergyAxonometryFigure` in both languages, a cover macro with staffage and without dimension, label, node, math or dashed lines, and a technical macro that keeps its dimensions and hidden edges and has no staffage.
- Both languages compile with the local MiKTeX XeLaTeX against the real class to five pages without errors or overfull boxes. Cover (de and en) and the German geometry page were reviewed as images: the picture sits centred in the cover window with free margin on all sides, and the geometry page still shows the technical drawing.
- The managed build `repo run @teaching/architecture-course:build-heating-cooling-de` was not run; it was reported as failing in repo infrastructure before this change.

## Notes

- Python on this machine cannot open an existing replicated file with mode `w` (`OSError 22`); opening with `r+`, writing and truncating works, as does the Edit tool.
- Bash here-documents in this harness collapse `\\` to `\`, so TeX was written with the Write and Edit tools.
