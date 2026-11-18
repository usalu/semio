# Silhouette repairs: furniture, objects, vegetation

Scope: `🛋️furniture`, `🪑️objects`, `🌳️vegetation` of the guard `catalog-silhouette`. Edits are in-place value edits (`sfov-lib.py` on top of `sil-span.py`, scripts `sfov-*.py` in this folder); no formatting of untouched values changed; the guard was not loosened.

## Fixed (5 assets, 0 failing checks each, measured with `silhouette-triage.ts current`)

| Asset | Failing before | Cause and decision |
|---|---|---|
| park-bench | 1:50 front Hausdorff 0.0375 and 0.0185, plan 0.0169 | the 5 mm gaps between the seat slats were closed in the drawing by the 17 mm pen and by one chamfer line across all gaps, the volume had the slits. Real slat gaps: slats 0.100 m wide, pitch 0.130 m (30 mm gaps) in volume, front and plan; the chamfer line is split per slat; the plan now draws the feet (0.124 x 0.55 m) that are visible through the gaps (the volume had them); the drawn arm scroll stayed inside the arm post |
| tealight | 1:50 front IoU 0.919 | the volume cup wall was 0.3 mm thin (below one cell) and 0.7 mm narrower than the drawn cup; wall 2 mm thick, outer radius to the drawn 0.0162 to 0.0195 m |
| patio-heater | 1:50 front Hausdorff 0.041 | drawn castors sat at 0.11 to 0.13 m lateral, the volume has them at 0.19 to 0.21 m; drawing moved to the volume; the axles between castor and housing are drawn |
| charcoal-grill | 1:100 front Hausdorff 0.0346 | volume error: the charcoal bed (radius 0.215 m at y = 0.523 m) stuck out of the bowl underside (bowl radius 0.13 m there); bed base lowered to fit inside the bowl (profile radius 0.15 m at the base) |
| picnic-table | 1:100 front 0.045, 1:50 front 0.0488, 1:50 elevation 0.0394 | hinge blocks drawn 0.01 m short of the leg (volume 0.11 m wide); hinges widened to 0.11 m at 1:100 and 1:50; tabletop planks 0.14 m pitch with 2 mm slits in the volume (the 17 mm pen closes them in the drawing): planks butt jointed 0.14 m, drawn boards 0.14 m |

Checks run after the edits: `validate-all.ts` 1 failing of 152 (`electric-kick-scooter`, vehicles: not edited by this task, a `tone`/lateral-ambiguous issue of another agent), `validate-manifest.ts` 0 issues; vitest schema-conformance, asset-power, three-views, fill-rule, plan-projection, section-views: 5 of 6 files pass (445 of 446 tests); the one failure is schema-conformance of `vehicles/electric-kick-scooter` (shape `fork-crown` lacks `tone`), not an asset of this task.

## Remaining (exact failing checks of the catalog table, measured before this work; the numbers of the assets above are removed)

- furniture: office-swivel-chair (1:100 and 1:50 plan Hausdorff 0.0488 m: volume bridge at x -0.07..-0.02, lateral -0.24 in the slit between seat and armrest; castor volume origins at x = -0.0003 against drawn 0.093 m, open), cafe-chair (1:100 elevation H 0.0302; 1:50 front H 0.0203; 1:50 plan H 0.0169), height-adjustable-desk (5), three-seat-sofa (5, plan 0.2888 m), folding-deck-chair (6), operating-table (6).
- objects: shopping-trolley (1:100 front IoU 0.913; wire members, volume wires 1 cell wider than drawn), playground-swing (2), stepladder (6), street-lamp (6), market-parasol (8).
- vegetation (all 16 untouched, 3 to 34 checks each): hornbeam-hedge 3, hosta-clump 9, ivy-on-trellis 14, agave 16, apple-tree 17, weeping-willow 17, japanese-maple 18, small-leaved-lime 20, london-plane-pollarded 21, multi-stem-birch 21, lombardy-poplar 21, english-oak 24, miscanthus-grass 24, scots-pine 27, bamboo-clump 28, date-palm 34 (canopy and foliage drawn as areas against the volume envelopes; each needs its own pass of drawing and envelope).

## Method notes

`silhouette-diag.ts <asset> <scale> <view> <x> <y> <r>` prints the aligned mask (`#` both, `D` drawn only, `V` volume only). Findings: thin pen strokes (0.35 mm paper = 17 mm at 1:50) close slits of the drawing that the volume keeps; hidden volume members that the drawing never shows (feet under slats) show through real gaps; volume members that poke out of enclosing envelopes (charcoal bed).
