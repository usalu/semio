# Invalid staffage assets: causes and fixes

Validator: `validate-assets.ts` (this folder), `validateAsset` of `@teaching/staffage` with every catalog asset as `context.assets`. Run: `<bun> validate-assets.ts [asset.json ...]` (all assets when no argument). Output under `🗑️generated/`.

Furniture and vehicles: 21 checked, 0 failing after the fixes. Whole catalog: 152 checked, 4 failing, all owned by other agents (tower-crane, wheelchair-user, person-with-rollator, tour-guide).

## Why they regressed
The validator (extent checks per scale on the default pose, `lateral-ambiguous`, `volumes-*` rules, `exports-model-volume`) was tightened by the 3D-volume tickets (`☀️10/ENGINE-NEUTRAL-3D-MODELS-FOR-EVERY-STAFFAGE`, `☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE`) after the 2D shapes were authored. Zero-width centre lines (1:500 and 1:200 `poly` legs) measure the centre line, not the outer face, and flat shapes on steered bones were never checked before. `city-bicycle` was authored (`☀️14/.../volumes/vehicles/city-bicycle.json`, report `author-43.md`) but never spliced.

## Fixes (all handcrafted in the asset JSON, validator untouched)
| Asset | Cause | Fix |
|---|---|---|
| bar-stool | 1:500 and 1:200 elevation and front legs ended at +-0.1975 (centre line, 0.395 m) against 0.42 m; 1:500 plan footrest ring rx 0.2275 (0.455 m) | leg feet at +-0.21 (outer face, as capsule centre 0.1975 plus radius 0.0125 at 1:100); 1:500 ring rx/ry 0.205 as at 1:200 |
| cafe-chair | front legs splayed to +-0.2395 and plan to +-0.236..0.238 (0.479 m against 0.5 m); 1:500 back panel rect 0.075 high tilted the elevation to 0.457 m against 0.44 m | front-elevation feet +-0.25, plan front-leg feet +-0.245 at 1:500, 1:200, 1:100; 1:500 back panel 0.05 high |
| folding-deck-chair | 1:500 and 1:200 rear poly corners at x = 0 stuck out 0.02 m behind the capsule used at 1:100 (0.801 m against 0.78 m) | rear leg polys of elevation, front and plan start at x = 0.0225 like the 1:100 capsules |
| operating-table | emitter `light-plume` had no thermal solid tagged with its id | thermal solid `th-light-plume` tagged `["emitter", "light-plume"]` at 1:50, 1:100, 1:200, 1:500 |
| single-bed | hidden region `slats` reused body solids at 1:50 and 1:100 | real hidden slat boxes inside the body (1:50 0.027 x 0.84, 1:100 0.029 x 0.85, 1:200 unchanged 0.031 x 0.86 high x wide), distinct per scale |
| electric-kick-scooter | duplicate id `fender-r` in 1:100 elevation | the outline stroke is `fender-r-edge` |
| cargo-bike | flat `fork-line` (1:500) and `tyre-f` (1:200, 1:100, 1:50) on bones that leave the drawing plane by 14 degrees | `fork-line` a closed poly with an extruded solid (z +-0.04); front tyre paths with extruded solids (1:200 +-0.04, 1:100 +-0.03, 1:50 +-0.065, as the rims) |
| city-bicycle | no `volumes` (and so no model exports); flat `wheel-f`, `tyre-f`, `rim-f` on the steered wheel | authored volumes of `author-43` spliced with `volumes-insert.py` (1:50, 1:100, 1:200 authored, 1:500 unavailable like the representation); extruded solids on the flat wheel shapes (+-0.02, rim +-0.015) |

Pre-splice copy of city-bicycle: `🗑️generated/city-bicycle.before-splice.json`.

## Changed files
All under `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/`: `🛋️furniture/{bar-stool,cafe-chair,folding-deck-chair,operating-table,single-bed}/🔣️.json`, `🚜️vehicles/{cargo-bike,city-bicycle,electric-kick-scooter}/🔣️.json`.
