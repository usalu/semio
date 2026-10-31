# Six-Category Print Check

Office-laser `checkAssetDetailed` (`samples: 4`, `physics: true`) on all 69 assets in vegetation, furniture, objects, vehicles, machinery, and components: **0 problems**.

Unit tests before the edits: `print-suitability` 5, `representation-resolution` 8, `catalog-physics` 322. **335 passed**.

## Fixes

Shape `penMm` was raised, only on the shapes that still failed, so the ink of a real gap meets `(penA + penB) / 2 >= centre distance`, and thin fills and strokes reach the office-laser feature and stroke limits. Pens stay at or below 1.99 mm. Outlines, extents, and `compareRepresentations` were not rewritten.

Touched assets (plus `date-palm`, 7 pens, edited in the first pass): miscanthus-grass, ivy-on-trellis, english-oak, japanese-maple, weeping-willow, apple-tree, lombardy-poplar, multi-stem-birch, bamboo-clump, london-plane-pollarded, scots-pine, small-leaved-lime, agave, diesel-generator, tower-crane, wheel-loader, heat-pump-outdoor-unit, forklift, cafe-chair, office-swivel-chair, height-adjustable-desk, park-bench, retractable-bollard, market-parasol, shopping-trolley, stepladder, charcoal-grill, drinking-fountain, playground-swing, waste-bin, street-lamp, city-car, city-bus, tram, city-bicycle, external-venetian-blind, panel-radiator, sectional-garage-door, curtain-wall-bay, spiral-stair.

A later pass over the same 69 files measured 0 suitability violations, and the detailed check confirmed 0 problems on every asset.
