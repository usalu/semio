# Author 05 report: animals (dairy-cow, domestic-cat, goat, hedgehog, herring-gull, horse, labrador-dog)

## Status in one sentence

All seven assets have `volumes` for every scale their `representations` author, spliced into the catalog assets (`volumes-insert.py`), but only the horse was iterated against the renders; the other six are first-pass models from the same generator and **do not pass every rule** (see the table). This is not finished work.

## What was built

Generator in `T` (inputs kept): `author-05-lib.ts` (Author: lofts from the scale's own posed 2D shapes, with section width/centre/exponent cut from the visual hull of elevation, front and plan; fallback of a shape without front/plan twin to its parent bone's shape), `author-05-gen.ts` (trim of sections to the drawn silhouettes, fill capsules for bones drawn only in front/plan, thermal skins as `{ of }` references with nested shells at 1:50, emitter solids, texture-sheet fallbacks for regions the coarse body does not draw, structure members/joints/masses with per-scale section kinds, containment shrink), `author-05-animal.ts` + one `author-05-<slug>.ts` per asset (run: `bun author-05-<slug>.ts [scale]`), `author-05-areafix.py` (root fix of mis-estimated `areaM2`), diagnostics `author-05-haus.ts`, `-solid.ts`, `-hull.ts`, `-ext.ts`, `-dump.ts`. Sources: `volumes/animals/<slug>.json`. Check outputs: `🗑️generated/author-05/<slug>.check.txt`.

Solid counts (body, thermal, structure) per scale:

| asset | 1:50 | 1:100 | 1:200 | 1:500 |
|---|---|---|---|---|
| horse | 45, 47, 65 | 43, 45, 61 | 40, 42, 50 | 24, 33, 36 |
| dairy-cow | 35, 37, 62 | 35, 37, 62 | 29, 34, 56 | 15, 28, 49 |
| domestic-cat | 39, 42, 64 | 37, 41, 64 | 24, 35, 55 | unavailable |
| goat | 34, 37, 55 | 28, 31, 50 | 23, 26, 46 | 14, 23, 33 |
| hedgehog | 16, 30, 30 | 11, 14, 30 | unavailable | unavailable |
| herring-gull | 19, 33, 35 | 13, 17, 35 | 12, 16, 30 | unavailable |
| labrador-dog | 35, 38, 60 | 35, 38, 60 | 23, 32, 54 | 20, 31, 46 |

Bilingual `decisions` (3 sentences per scale) are in every list; the horse has hand-written ones, the others a shared template with asset names.

## Check results (last full `volumes-check.ts`, rulings C1-C11 as measured by the tool at the time)

Failing rule lines per asset (details in the check files): horse 8 (silhouette 1:500, cross-scale 1:200, thermal 1:50-1:500 only for `mane`/`muzzle-skin` (buried) and 1:500 limbs, structure 1:500); dairy-cow 16; domestic-cat 8 (1:200 silhouette: the 1:200 drawing disagrees with itself, thorax-b length elevation 234 mm vs plan 134 mm); goat 19; hedgehog 8; herring-gull 10; labrador-dog 15 (1:500 elevation IoU 0.21, a bug in my coarse trunk/neck or fill capsules, not investigated). Typical failures: member solids below the feature (structure lists), thermal areaDeviation on small regions, silhouette IoU at the coarsest scale, bounds depth a few mm off `dimensions`. Horse 1:50/1:100/1:200 pass silhouette (IoU 0.926 front is the lowest), bounds, feature, bones, solids, copied, monotonic.

## Root fixes to assets (areaM2, C1; old -> new, m2)

horse: coat-head 0.3 -> 0.481, coat-forelimbs 0.55 -> 1.26, coat-hindlimbs 0.75 -> 1.33, ears 0.03 -> 0.061, hooves 0.12 -> 0.144, tail 0.12 -> 0.409 (mane 0.1 and muzzle-skin 0.015 left: their model skins are buried in the neck/head loft). dairy-cow: coat-trunk 1.17 -> 4.11, coat-head-neck 0.45 -> 1.11, coat-forelimbs 0.38 -> 0.963, coat-hindlimbs 0.57 -> 0.885, udder 0.45 -> 0.264, muzzle 0.03 -> 0.0462, eyes 0.002 -> 0.0033, ears 0.07 -> 0.104, coat-tail 0.08 -> 0.212. goat: coat-trunk 0.7632 -> 0.893, coat-neck 0.1248 -> 0.189, coat-upper-legs 0.2652 -> 0.724, lower-legs 0.14 -> 0.167, face 0.06 -> 0.0905, tail 0.015 -> 0.0126. labrador-dog: coat-trunk 0.544 -> 0.701, coat-head 0.07 -> 0.111, ears 0.03 -> 0.0476, coat-forelimbs 0.16 -> 0.423, coat-hindlimbs 0.2 -> 0.264, coat-tail 0.04 -> 0.0756, pads 0.02 -> 0.0149. domestic-cat: coat-trunk 0.124 -> 0.145, coat-head 0.013 -> 0.0354, ears 0.005 -> 0.0082, coat-forelimbs 0.03 -> 0.0736, coat-hindlimbs 0.047 -> 0.095, coat-tail 0.03 -> 0.0382, nose-skin 0.0005 -> 0.0004, pads 0.0085 -> 0.0057. hedgehog: spines-dome 0.04 -> 0.048, spines-skirt 0.02 -> 0.0503, belly-fur 0.02 -> 0.0127, face-fur 0.006 -> 0.012, paws-skin 0.0035 -> 0.006. herring-gull: body-plumage 0.05 -> 0.167, neck-plumage 0.01 -> 0.0311, bill 0.002 -> 0.0067. These were applied before the bodies were final; they must be redone (`python3 author-05-areafix.py <slug> --apply`) after any body change, and several (cow trunk, goat upper legs, gull) are large corrections that the coordinator should sanity-check against the heat-flow data. Regions skipped as not plain mis-estimates (ratio outside 0.5-3.6): goat horns/hooves, cow hooves, dog nose-skin/tongue, cat eyes/tongue, hedgehog limbs-fur, gull head/wing/tail/legs/web feet.

## What I saw in the renders (horse, 1:50 and 1:200 only)

1:50 horse reads as a horse in all three axonometric views, the trunk, neck and limbs are real cross-sections, but the scapula and thigh are still plate-like and the loft tessellation is faceted. Front and plan overlays show red at neck and scapula corners (drawn only) up to 40 mm. 1:200 before the parent-shape fix showed lower legs missing in the front; fixed. The cat 1:200 render shows a messy tail and an elevation IoU far below the limit. I did not look at the renders of the other five assets.

## Left imperfect, plainly

- Six of seven assets were not iterated; structure members smaller than the feature (goat, dog, cat at coarse scales), thermal area deviations, 1:500 silhouettes (dog 0.21 elevation IoU) and bounds depth deviations remain.
- Cow patches, rumen patch and teats, gull mantle, horse engraved lines are not modelled; the cow `pelvis`, cat/dog/goat `pelvis` carry only a fill capsule inside the trunk loft (horse has a real rump loft).
- Hedgehog limbs, gull etc. were not checked visually.
- No 2D drawing was repaired; horse 1:100/1:200/1:500 `volumes-views-disagree` and the cat 1:200 drawing inconsistency are 2D defects (ears, jaw, mane and lower legs drawn with `filled: false` in elevation at coarse scales).
