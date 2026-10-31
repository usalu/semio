# Staffage Catalog Suitability Pass

## Geometry

`leaning-person` was rewritten by `CV-H3/leaning_build.py`. The 1:500 paths are cubic outlines again. `thermal.passengers` is an empty list. `structure.dynamics` stays `inertial`.

Git index comparison of every human asset showed scrambled `d` paths only on `leaning-person`. Other human pen edits stay at or below 0.50 mm.

`runner-with-blade-prosthesis` `blade-toe` at 1:500 was widened so the filled feature clears 0.20 mm paper.

Pens above 0.50 mm that remain are the 1:50 ground weight (0.70 mm) and shape pens already accepted by `measureSuitability`. They were left in place.

## Results

- `validateManifest`: 0 issues, 107 assets
- `checkAssetDetailed` (`samples: 5`, `physics: true`): 0 problems, 107/107
- Unit tests `print-suitability`, `representation-resolution`, `catalog-physics`: 3 files, 335 passed
