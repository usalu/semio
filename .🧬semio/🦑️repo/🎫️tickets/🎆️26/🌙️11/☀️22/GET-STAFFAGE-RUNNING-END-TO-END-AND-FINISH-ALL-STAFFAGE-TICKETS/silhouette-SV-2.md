# 🌳️ Silhouette Fixer Report: Batch SV-2 (19 Plants)

Written 2026-11-24 by the fixer of `U/🧰️silhouette/SV-2.tsv`; every number was measured in this session. `S` is `🎓️teaching/🏛️architecture/🧍️staffage`, `U` this ticket folder, `W` is `U/🧰️silhouette/SV-2`.

## Result

19 of 19 plants pass the real `🧊️catalog-silhouette` test (one vitest run per plant, each to the end, sharp oracle included). Before the repair the 19 plants failed 198 checks (every check measured, not only the first). No rule, threshold, test or tool was changed.

Per plant the proof line reads: real silhouette test | `validateAsset` (issues now, issues before the repair, new ones) | staffage check CLI | the plant's tests in `🧊️three-views` (2), `🌡️catalog-physics` (2) and `🫁️breathing-catalog` (1). Before the repair these suites passed 38, 38 and 19 tests for the 19 plants; they pass the same after it. `✅️asset-validation` and `⚡️asset-power` have no test named by a plant of this batch (their per-asset cases are fixture assets), see "What stays red".

The 1:50 and the 1:500 sheet (side, front, plan; line and solid) of every plant were rendered from the final catalog file and looked at; nothing was kept that reads worse than before. Two things changed visibly and are meant: `multi-stem-birch` 1:500 and 1:200 now draw the thin branches the volume has between the leaf sprays as strokes (they were not drawn), and `miscanthus-grass`, `ivy-on-trellis` draw every blade, stem and trailer exactly on the centre line of its solid.

## Causes found

1. Boundary distances (118 of the 198 checks): a filled shape reached a few centimetres outside its volume, or a bone had volume the drawing did not cover (lobes moved since the drawing was traced, gaps between lobes drawn closed). Fix: the shape is cut to the volume, or joined with the outline of the solids of its bone, on the raster and the tile grid of the guard (`W/retrace.ts`).
2. Bounding boxes (72 checks): the four volumes of a plant differed among each other by up to 0.31 m (`red-oak` 18.39 m at 1:50, 18.71 m at 1:500) and from the declared dimensions. Fix (`W/fit-box.ts`): the declared dimension stays where the fewest scales have to move, else it becomes the extent of the 1:50 body; at the other scales the lobes that reach beyond the reference are pushed back by their own overshoot (102 solids, foliage lobes and tufts, at most 174 mm at 1:500, where one feature is 75 mm), two root solids are set on the ground (`olive-tree` 1:200 `root-plate` 44 mm, `stone-pine` 1:500 `trunk-1`, which started 0.2 m below the ground).
3. Slender solids without a stroke or beside it (`multi-stem-birch`, `miscanthus-grass`, `ivy-on-trellis`, `weeping-willow`): the stroke is drawn on the centre line of the solid, or added.
4. IoU (8 checks) followed from 1 to 3.
5. Two properties of the guard decide single cells and are reported in `U/requests.md` ("silhouette SV-2"): small holes are filled per tile, and only the volume raster is sealed.

A first pass traced the outlines to 0.15 feature and passed the guard, but made the drawings 5 to 20 times heavier (`weeping-willow` 31 257 to 628 175 points): `🧊️three-views` then overflowed the stack for `small-leaved-lime` and `weeping-willow` (`Math.min(...xs)` over all points of a view) and the check CLI ran beyond 15 minutes for three plants. All traced outlines were therefore thinned to 0.5 feature (`W/slim.ts`) and repaired again; 14 of the 19 plants now carry between 0.90 and 1.19 times the points they had before the repair; the others carry more: `london-plane-pollarded` 12 995 to 24 781, `multi-stem-birch` 134 864 to 257 246, `miscanthus-grass` 41 212 to 87 738 (every blade follows the bends of its solid), `small-leaved-lime` 36 310 to 82 652, `weeping-willow` 31 257 to 210 948 (its volume is 190 narrow curtains whose gaps the guard measures to 15 mm). Every proof below is of that final state.

## Reproducibility

Every change is an idempotent op in `W/repair/<slug>.json` (whole shapes by id, `solid-set` with absolute values, `dims`), applied by `W/apply.ts <slug>` as the last step after the generator of the plant (the generators of the plant wave and of the older plants write the catalog file; `apply.ts` reads it right before it writes and keeps every byte of a hand-formatted file that did not change, `W/spans.ts`). The ops were made by `W/fix.sh` (`fit-box.ts`, `retrace.ts`, `apply.ts`, `measure.ts`), `W/slim.ts`, `W/rebind.ts`, and by hand-directed runs for four plants: `fix-stone-pine-ground.py`; `retrace.ts ivy-on-trellis 100 elevation --patch=-0.42,0.812,0.08`; `retrace.ts ivy-on-trellis 200 front --limit 0.02`; `retrace.ts miscanthus-grass <100|50> … --restroke`; `retrace.ts miscanthus-grass 50 front --patch=0.3724,1.2413,0.004,blade-c3b-1`.

Declared dimensions changed for 14 plants: `W/manifest-changes.json` (for the coordinator; the manifest `S/🗂️catalog/🔣️.json` was not edited, so `validateManifest` and the manifest test of `⚡️asset-power` report these 14 rows until it is applied).

## Per plant

### `ivy-on-trellis`

- Before: 8 failing checks: 1:200 front IoU 0.9124; 1:100 width 1.9000 m against 1.92 m; 1:100 elevation distance 0.1050 m (limit 0.030); 1:100 front IoU 0.9135; 1:100 front distance 0.0452 m (limit 0.030); 1:50 width 1.9000 m against 1.92 m; 1:50 elevation distance 0.0327 m (limit 0.015); 1:50 front distance 0.0244 m (limit 0.015).
- After: 0 failing checks.
- Changed (23 ops in `repair/ivy-on-trellis.json`): declared widthM 1.9; 5 × 1:200 filled shapes retraced, 4 × 1:200 strokes redrawn, 1 × 1:100 filled shapes retraced, 6 × 1:100 strokes redrawn, 1 × 1:50 filled shapes retraced, 5 × 1:50 strokes redrawn.
- Proof: `ivy-on-trellis | silhouette: exit 0  Test Files 1 passed (1); Tests 1 passed | 384 skipped (385);  | validate: 0 issues	0 before, 0 new | check: ✓ ivy-on-trellis (1/1) | three-views: exit 0  Test Files 1 passed (1); Tests 2 passed | 773 skipped (775); | physics: exit 0  Test Files 1 passed (1); Tests 2 passed | 838 skipped (840); | breathing: exit 0  Test Files 1 passed (1); Tests 1 passed | 289 skipped (290);`

### `japanese-cherry`

- Before: 5 failing checks: 1:200 front distance 0.0605 m (limit 0.060); 1:100 front distance 0.0302 m (limit 0.030); 1:50 elevation distance 0.0186 m (limit 0.015); 1:50 front distance 0.0521 m (limit 0.015); 1:50 plan distance 0.2026 m (limit 0.015).
- After: 0 failing checks.
- Changed (23 ops in `repair/japanese-cherry.json`): 1 × 1:200 filled shapes retraced, 2 × 1:100 filled shapes retraced, 20 × 1:50 filled shapes retraced.
- Proof: `japanese-cherry | silhouette: exit 0  Test Files 1 passed (1); Tests 1 passed | 384 skipped (385);  | validate: 0 issues	0 before, 0 new | check: ✓ japanese-cherry (1/1) | three-views: exit 0  Test Files 1 passed (1); Tests 2 passed | 773 skipped (775); | physics: exit 0  Test Files 1 passed (1); Tests 2 passed | 838 skipped (840); | breathing: exit 0  Test Files 1 passed (1); Tests 1 passed | 289 skipped (290);`

### `japanese-maple`

- Before: 4 failing checks: 1:500 front distance 0.5450 m (limit 0.150); 1:100 front distance 0.0642 m (limit 0.030); 1:50 elevation distance 0.0495 m (limit 0.015); 1:50 front distance 0.0488 m (limit 0.015).
- After: 0 failing checks.
- Changed (23 ops in `repair/japanese-maple.json`): 3 × 1:500 filled shapes retraced, 7 × 1:100 filled shapes retraced, 6 × 1:50 filled shapes added, 7 × 1:50 filled shapes retraced.
- Proof: `japanese-maple | silhouette: exit 0  Test Files 1 passed (1); Tests 1 passed | 384 skipped (385);  | validate: 0 issues	0 before, 0 new | check: ✓ japanese-maple (1/1) | three-views: exit 0  Test Files 1 passed (1); Tests 2 passed | 773 skipped (775); | physics: exit 0  Test Files 1 passed (1); Tests 2 passed | 838 skipped (840); | breathing: exit 0  Test Files 1 passed (1); Tests 1 passed | 289 skipped (290);`

### `lombardy-poplar`

- Before: 13 failing checks: 1:200 width 4.4025 m against 4.464 m; 1:200 depth 4.1296 m against 4.178 m; 1:200 elevation distance 0.1610 m (limit 0.060); 1:200 front distance 0.0708 m (limit 0.060); 1:100 width 4.5129 m against 4.464 m; 1:100 depth 4.1594 m against 4.178 m; 1:100 elevation distance 0.0750 m (limit 0.030); 1:100 front distance 0.1956 m (limit 0.030); 1:50 width 4.5259 m against 4.464 m; 1:50 depth 4.2261 m against 4.178 m; 1:50 elevation distance 0.0735 m (limit 0.015); 1:50 front distance 0.0984 m (limit 0.015); 1:50 plan distance 0.0265 m (limit 0.015).
- After: 0 failing checks.
- Changed (78 ops in `repair/lombardy-poplar.json`): 7 × 1:200 filled shapes retraced, 4 × 1:200 solids moved, 14 × 1:100 filled shapes retraced, 4 × 1:100 solids moved, 45 × 1:50 filled shapes retraced, 4 × 1:50 solids moved.
- Proof: `lombardy-poplar | silhouette: exit 0  Test Files 1 passed (1); Tests 1 passed | 384 skipped (385);  | validate: 0 issues	0 before, 0 new | check: ✓ lombardy-poplar (1/1) | three-views: exit 0  Test Files 1 passed (1); Tests 2 passed | 773 skipped (775); | physics: exit 0  Test Files 1 passed (1); Tests 2 passed | 838 skipped (840); | breathing: exit 0  Test Files 1 passed (1); Tests 1 passed | 289 skipped (290);`

### `london-plane-pollarded`

- Before: 8 failing checks: 1:100 height 8.6145 m against 8.634 m; 1:100 front distance 0.0335 m (limit 0.030); 1:50 width 5.6716 m against 5.657 m; 1:50 height 8.6543 m against 8.634 m; 1:50 depth 5.6061 m against 5.595 m; 1:50 elevation distance 0.0235 m (limit 0.015); 1:50 front distance 0.0797 m (limit 0.015); 1:50 plan distance 0.0319 m (limit 0.015).
- After: 0 failing checks.
- Changed (38 ops in `repair/london-plane-pollarded.json`): declared widthM 5.672, heightM 8.654, depthM 5.606; 1 × 1:200 filled shapes added, 3 × 1:200 solids moved, 1 × 1:100 filled shapes added, 3 × 1:100 filled shapes retraced, 3 × 1:100 solids moved, 5 × 1:50 filled shapes added, 21 × 1:50 filled shapes retraced.
- Proof: `london-plane-pollarded | silhouette: exit 0  Test Files 1 passed (1); Tests 1 passed | 384 skipped (385);  | validate: 0 issues	0 before, 0 new | check: ✓ london-plane-pollarded (1/1) | three-views: exit 0  Test Files 1 passed (1); Tests 2 passed | 773 skipped (775); | physics: exit 0  Test Files 1 passed (1); Tests 2 passed | 838 skipped (840); | breathing: exit 0  Test Files 1 passed (1); Tests 1 passed | 289 skipped (290);`

### `miscanthus-grass`

- Before: 11 failing checks: 1:100 width 2.1248 m against 2.14 m; 1:100 depth 1.7849 m against 1.8 m; 1:100 elevation distance 0.0993 m (limit 0.030); 1:50 width 2.1249 m against 2.14 m; 1:50 depth 1.7853 m against 1.8 m; 1:50 elevation IoU 0.9039; 1:50 elevation distance 0.1076 m (limit 0.015); 1:50 front IoU 0.9025; 1:50 front distance 0.0537 m (limit 0.015); 1:50 plan IoU 0.9174; 1:50 plan distance 0.0759 m (limit 0.015).
- After: 0 failing checks.
- Changed (1486 ops in `repair/miscanthus-grass.json`): declared widthM 2.125, depthM 1.785; 187 × 1:100 strokes redrawn, 1 × 1:50 filled shapes added, 1 × 1:50 filled shapes retraced, 1296 × 1:50 strokes redrawn.
- Proof: `miscanthus-grass | silhouette: exit 0  Test Files 1 passed (1); Tests 1 passed | 384 skipped (385);  | validate: 0 issues	0 before, 0 new | check: ✓ miscanthus-grass (1/1) | three-views: exit 0  Test Files 1 passed (1); Tests 2 passed | 773 skipped (775); | physics: exit 0  Test Files 1 passed (1); Tests 2 passed | 838 skipped (840); | breathing: exit 0  Test Files 1 passed (1); Tests 1 passed | 289 skipped (290);`

### `multi-stem-birch`

- Before: 14 failing checks: 1:500 elevation IoU 0.9003; 1:500 elevation distance 0.5094 m (limit 0.150); 1:500 front distance 0.5637 m (limit 0.150); 1:500 plan distance 0.5703 m (limit 0.150); 1:200 depth 6.7536 m against 6.8 m; 1:200 elevation distance 0.2163 m (limit 0.060); 1:200 front distance 0.2677 m (limit 0.060); 1:100 elevation distance 0.1765 m (limit 0.030); 1:100 front distance 0.2101 m (limit 0.030); 1:100 plan distance 0.1247 m (limit 0.030); 1:50 height 13.0306 m against 13.018 m; 1:50 elevation distance 0.0731 m (limit 0.015); 1:50 front distance 0.2054 m (limit 0.015); 1:50 plan distance 0.0394 m (limit 0.015).
- After: 0 failing checks.
- Changed (203 ops in `repair/multi-stem-birch.json`): declared heightM 13.031; 50 × 1:500 filled shapes retraced, 29 × 1:500 strokes added, 26 × 1:200 filled shapes retraced, 2 × 1:200 solids moved, 47 × 1:100 filled shapes retraced, 48 × 1:50 filled shapes retraced.
- Proof: `multi-stem-birch | silhouette: exit 0  Test Files 1 passed (1); Tests 1 passed | 384 skipped (385);  | validate: 0 issues	0 before, 0 new | check: ✓ multi-stem-birch (1/1) | three-views: exit 0  Test Files 1 passed (1); Tests 2 passed | 773 skipped (775); | physics: exit 0  Test Files 1 passed (1); Tests 2 passed | 838 skipped (840); | breathing: exit 0  Test Files 1 passed (1); Tests 1 passed | 289 skipped (290);`

### `norway-maple`

- Before: 7 failing checks: 1:100 depth 14.1609 m against 14.182 m; 1:100 elevation distance 0.0369 m (limit 0.030); 1:100 front distance 0.0354 m (limit 0.030); 1:50 depth 14.1712 m against 14.182 m; 1:50 elevation distance 0.0229 m (limit 0.015); 1:50 front distance 0.0286 m (limit 0.015); 1:50 plan distance 0.0214 m (limit 0.015).
- After: 0 failing checks.
- Changed (37 ops in `repair/norway-maple.json`): declared depthM 14.171; 2 × 1:200 solids moved, 3 × 1:100 filled shapes retraced, 2 × 1:100 solids moved, 29 × 1:50 filled shapes retraced.
- Proof: `norway-maple | silhouette: exit 0  Test Files 1 passed (1); Tests 1 passed | 384 skipped (385);  | validate: 0 issues	0 before, 0 new | check: ✓ norway-maple (1/1) | three-views: exit 0  Test Files 1 passed (1); Tests 2 passed | 773 skipped (775); | physics: exit 0  Test Files 1 passed (1); Tests 2 passed | 838 skipped (840); | breathing: exit 0  Test Files 1 passed (1); Tests 1 passed | 289 skipped (290);`

### `olive-tree`

- Before: 12 failing checks: 1:500 elevation distance 0.1698 m (limit 0.150); 1:500 front distance 0.1729 m (limit 0.150); 1:200 height 6.9391 m against 6.891 m; 1:200 front distance 0.0928 m (limit 0.060); 1:100 elevation distance 0.0819 m (limit 0.030); 1:100 front distance 0.0456 m (limit 0.030); 1:100 plan distance 0.2475 m (limit 0.030); 1:50 height 6.8767 m against 6.891 m; 1:50 depth 7.7073 m against 7.695 m; 1:50 elevation distance 0.0286 m (limit 0.015); 1:50 front distance 0.0346 m (limit 0.015); 1:50 plan distance 0.2335 m (limit 0.015).
- After: 0 failing checks.
- Changed (61 ops in `repair/olive-tree.json`): declared heightM 6.877; 3 × 1:500 filled shapes retraced, 2 × 1:200 filled shapes added, 5 × 1:200 filled shapes retraced, 2 × 1:200 solids moved, 20 × 1:100 filled shapes retraced, 25 × 1:50 filled shapes retraced, 1 × 1:50 shapes removed, 2 × 1:50 solids moved.
- Proof: `olive-tree | silhouette: exit 0  Test Files 1 passed (1); Tests 1 passed | 384 skipped (385);  | validate: 0 issues	0 before, 0 new | check: ✓ olive-tree (1/1) | three-views: exit 0  Test Files 1 passed (1); Tests 2 passed | 773 skipped (775); | physics: exit 0  Test Files 1 passed (1); Tests 2 passed | 838 skipped (840); | breathing: exit 0  Test Files 1 passed (1); Tests 1 passed | 289 skipped (290);`

### `red-oak`

- Before: 15 failing checks: 1:500 width 18.7099 m against 18.394 m; 1:500 height 21.2003 m against 21.098 m; 1:500 depth 19.0589 m against 18.743 m; 1:200 width 18.6003 m against 18.394 m; 1:200 height 21.1651 m against 21.098 m; 1:200 depth 18.9521 m against 18.743 m; 1:100 width 18.4935 m against 18.394 m; 1:100 height 21.1299 m against 21.098 m; 1:100 depth 18.8481 m against 18.743 m; 1:100 elevation distance 0.1145 m (limit 0.030); 1:100 front distance 0.0691 m (limit 0.030); 1:100 plan distance 0.8400 m (limit 0.030); 1:50 elevation distance 0.1987 m (limit 0.015); 1:50 front distance 0.0822 m (limit 0.015); 1:50 plan distance 0.1181 m (limit 0.015).
- After: 0 failing checks.
- Changed (122 ops in `repair/red-oak.json`): declared widthM 18.389; 8 × 1:500 filled shapes retraced, 9 × 1:500 solids moved, 10 × 1:200 filled shapes retraced, 6 × 1:200 solids moved, 29 × 1:100 filled shapes retraced, 1 × 1:100 shapes removed, 6 × 1:100 solids moved, 51 × 1:50 filled shapes retraced, 1 × 1:50 shapes removed.
- Proof: `red-oak | silhouette: exit 0  Test Files 1 passed (1); Tests 1 passed | 384 skipped (385);  | validate: 0 issues	0 before, 0 new | check: ✓ red-oak (1/1) | three-views: exit 0  Test Files 1 passed (1); Tests 2 passed | 773 skipped (775); | physics: exit 0  Test Files 1 passed (1); Tests 2 passed | 838 skipped (840); | breathing: exit 0  Test Files 1 passed (1); Tests 1 passed | 289 skipped (290);`

### `rowan`

- Before: 8 failing checks: 1:200 elevation distance 0.2520 m (limit 0.060); 1:200 front distance 0.0618 m (limit 0.060); 1:100 elevation distance 0.0500 m (limit 0.030); 1:100 front distance 0.0470 m (limit 0.030); 1:100 plan distance 0.0521 m (limit 0.030); 1:50 elevation distance 0.0369 m (limit 0.015); 1:50 front distance 0.0421 m (limit 0.015); 1:50 plan distance 0.0796 m (limit 0.015).
- After: 0 failing checks.
- Changed (38 ops in `repair/rowan.json`): 6 × 1:200 filled shapes retraced, 10 × 1:100 filled shapes retraced, 3 × 1:50 filled shapes added, 19 × 1:50 filled shapes retraced.
- Proof: `rowan | silhouette: exit 0  Test Files 1 passed (1); Tests 1 passed | 384 skipped (385);  | validate: 0 issues	0 before, 0 new | check: ✓ rowan (1/1) | three-views: exit 0  Test Files 1 passed (1); Tests 2 passed | 773 skipped (775); | physics: exit 0  Test Files 1 passed (1); Tests 2 passed | 838 skipped (840); | breathing: exit 0  Test Files 1 passed (1); Tests 1 passed | 289 skipped (290);`

### `saucer-magnolia`

- Before: 3 failing checks: 1:100 front distance 0.0489 m (limit 0.030); 1:50 elevation distance 0.4069 m (limit 0.015); 1:50 front distance 0.0185 m (limit 0.015).
- After: 0 failing checks.
- Changed (10 ops in `repair/saucer-magnolia.json`): 1 × 1:100 filled shapes retraced, 9 × 1:50 filled shapes retraced.
- Proof: `saucer-magnolia | silhouette: exit 0  Test Files 1 passed (1); Tests 1 passed | 384 skipped (385);  | validate: 0 issues	0 before, 0 new | check: ✓ saucer-magnolia (1/1) | three-views: exit 0  Test Files 1 passed (1); Tests 2 passed | 773 skipped (775); | physics: exit 0  Test Files 1 passed (1); Tests 2 passed | 838 skipped (840); | breathing: exit 0  Test Files 1 passed (1); Tests 1 passed | 289 skipped (290);`

### `scots-pine`

- Before: 13 failing checks: 1:200 width 12.0413 m against 12.076 m; 1:200 depth 7.8505 m against 7.907 m; 1:100 width 12.0573 m against 12.076 m; 1:100 depth 7.8868 m against 7.907 m; 1:100 elevation distance 0.0855 m (limit 0.030); 1:100 front IoU 0.8866; 1:100 front distance 0.3663 m (limit 0.030); 1:100 plan distance 0.8001 m (limit 0.030); 1:50 width 12.0556 m against 12.076 m; 1:50 depth 7.8755 m against 7.907 m; 1:50 elevation distance 0.1011 m (limit 0.015); 1:50 front distance 0.0478 m (limit 0.015); 1:50 plan distance 0.0508 m (limit 0.015).
- After: 0 failing checks.
- Changed (89 ops in `repair/scots-pine.json`): declared widthM 12.056, depthM 7.876; 2 × 1:500 solids moved, 2 × 1:200 solids moved, 33 × 1:100 filled shapes retraced, 2 × 1:100 solids moved, 8 × 1:100 strokes redrawn, 41 × 1:50 filled shapes retraced.
- Proof: `scots-pine | silhouette: exit 0  Test Files 1 passed (1); Tests 1 passed | 384 skipped (385);  | validate: 0 issues	0 before, 0 new | check: ✓ scots-pine (1/1) | three-views: exit 0  Test Files 1 passed (1); Tests 2 passed | 773 skipped (775); | physics: exit 0  Test Files 1 passed (1); Tests 2 passed | 838 skipped (840); | breathing: exit 0  Test Files 1 passed (1); Tests 1 passed | 289 skipped (290);`

### `small-leaved-lime`

- Before: 11 failing checks: 1:200 elevation distance 0.2550 m (limit 0.060); 1:200 front distance 0.0915 m (limit 0.060); 1:100 elevation distance 0.0419 m (limit 0.030); 1:100 front distance 0.3563 m (limit 0.030); 1:100 plan distance 0.0318 m (limit 0.030); 1:50 width 8.9811 m against 8.954 m; 1:50 height 14.3986 m against 14.386 m; 1:50 depth 8.8091 m against 8.789 m; 1:50 elevation distance 0.0506 m (limit 0.015); 1:50 front distance 0.1205 m (limit 0.015); 1:50 plan distance 0.0227 m (limit 0.015).
- After: 0 failing checks.
- Changed (189 ops in `repair/small-leaved-lime.json`): declared heightM 14.399; 1 × 1:200 filled shapes added, 8 × 1:200 filled shapes retraced, 1 × 1:200 shapes removed, 4 × 1:200 solids moved, 26 × 1:100 filled shapes retraced, 1 × 1:100 shapes removed, 2 × 1:100 solids moved, 5 × 1:50 filled shapes added, 110 × 1:50 filled shapes retraced, 27 × 1:50 shapes removed, 3 × 1:50 solids moved.
- Proof: `small-leaved-lime | silhouette: exit 0  Test Files 1 passed (1); Tests 1 passed | 384 skipped (385);  | validate: 0 issues	0 before, 0 new | check: ✓ small-leaved-lime (1/1) | three-views: exit 0  Test Files 1 passed (1); Tests 2 passed | 773 skipped (775); | physics: exit 0  Test Files 1 passed (1); Tests 2 passed | 838 skipped (840); | breathing: exit 0  Test Files 1 passed (1); Tests 1 passed | 289 skipped (290);`

### `stone-pine`

- Before: 15 failing checks: 1:500 height 20.6158 m against 20.494 m; 1:500 elevation distance 0.1546 m (limit 0.150); 1:200 height 20.4517 m against 20.494 m; 1:200 depth 10.5937 m against 10.626 m; 1:100 width 14.4746 m against 14.452 m; 1:100 height 20.4517 m against 20.494 m; 1:100 elevation distance 0.1082 m (limit 0.030); 1:100 front distance 0.5149 m (limit 0.030); 1:100 plan IoU 0.9178; 1:100 plan distance 1.0374 m (limit 0.030); 1:50 width 14.4725 m against 14.452 m; 1:50 height 20.4517 m against 20.494 m; 1:50 elevation distance 0.1021 m (limit 0.015); 1:50 front distance 0.0862 m (limit 0.015); 1:50 plan distance 0.3450 m (limit 0.015).
- After: 0 failing checks.
- Changed (108 ops in `repair/stone-pine.json`): declared widthM 14.472, heightM 20.452; 6 × 1:500 filled shapes retraced, 4 × 1:500 solids moved, 1 × 1:200 filled shapes retraced, 4 × 1:200 solids moved, 33 × 1:100 filled shapes retraced, 2 × 1:100 solids moved, 7 × 1:100 strokes redrawn, 50 × 1:50 filled shapes retraced.
- Proof: `stone-pine | silhouette: exit 0  Test Files 1 passed (1); Tests 1 passed | 384 skipped (385);  | validate: 0 issues	0 before, 0 new | check: ✓ stone-pine (1/1) | three-views: exit 0  Test Files 1 passed (1); Tests 2 passed | 773 skipped (775); | physics: exit 0  Test Files 1 passed (1); Tests 2 passed | 838 skipped (840); | breathing: exit 0  Test Files 1 passed (1); Tests 1 passed | 289 skipped (290);`

### `sweetgum`

- Before: 9 failing checks: 1:500 elevation distance 0.4276 m (limit 0.150); 1:200 elevation distance 0.2616 m (limit 0.060); 1:200 plan distance 0.0641 m (limit 0.060); 1:100 width 9.3747 m against 9.443 m; 1:100 height 17.9801 m against 18 m; 1:100 depth 8.4854 m against 8.593 m; 1:50 width 9.3720 m against 9.443 m; 1:50 height 17.9791 m against 18 m; 1:50 depth 8.4922 m against 8.593 m.
- After: 0 failing checks.
- Changed (25 ops in `repair/sweetgum.json`): declared widthM 9.372, heightM 17.979; 1 × 1:500 filled shapes retraced, 7 × 1:200 filled shapes retraced, 3 × 1:200 solids moved, 4 × 1:100 filled shapes retraced, 2 × 1:100 solids moved, 5 × 1:50 filled shapes retraced, 2 × 1:50 solids moved.
- Proof: `sweetgum | silhouette: exit 0  Test Files 1 passed (1); Tests 1 passed | 384 skipped (385);  | validate: 0 issues	0 before, 0 new | check: ✓ sweetgum (1/1) | three-views: exit 0  Test Files 1 passed (1); Tests 2 passed | 773 skipped (775); | physics: exit 0  Test Files 1 passed (1); Tests 2 passed | 838 skipped (840); | breathing: exit 0  Test Files 1 passed (1); Tests 1 passed | 289 skipped (290);`

### `sycamore-maple`

- Before: 9 failing checks: 1:200 elevation distance 0.0739 m (limit 0.060); 1:100 width 20.7787 m against 20.795 m; 1:100 elevation distance 0.0773 m (limit 0.030); 1:100 front distance 0.0309 m (limit 0.030); 1:100 plan distance 0.0309 m (limit 0.030); 1:50 width 20.7776 m against 20.795 m; 1:50 elevation distance 0.0247 m (limit 0.015); 1:50 front distance 0.0281 m (limit 0.015); 1:50 plan distance 0.0266 m (limit 0.015).
- After: 0 failing checks.
- Changed (52 ops in `repair/sycamore-maple.json`): declared widthM 20.778, heightM 25.104, depthM 20.029; 2 × 1:500 solids moved, 1 × 1:200 filled shapes retraced, 2 × 1:200 solids moved, 13 × 1:100 filled shapes retraced, 33 × 1:50 filled shapes retraced.
- Proof: `sycamore-maple | silhouette: exit 0  Test Files 1 passed (1); Tests 1 passed | 384 skipped (385);  | validate: 0 issues	0 before, 0 new | check: ✓ sycamore-maple (1/1) | three-views: exit 0  Test Files 1 passed (1); Tests 2 passed | 773 skipped (775); | physics: exit 0  Test Files 1 passed (1); Tests 2 passed | 838 skipped (840); | breathing: exit 0  Test Files 1 passed (1); Tests 1 passed | 289 skipped (290);`

### `tulip-tree`

- Before: 15 failing checks: 1:500 width 13.7688 m against 13.509 m; 1:500 depth 14.1034 m against 13.853 m; 1:200 width 13.6752 m against 13.509 m; 1:200 height 30.3528 m against 30.314 m; 1:200 depth 14.0112 m against 13.853 m; 1:200 front distance 0.3547 m (limit 0.060); 1:100 width 13.5856 m against 13.509 m; 1:100 height 30.3328 m against 30.314 m; 1:100 depth 13.9230 m against 13.853 m; 1:100 elevation distance 0.0428 m (limit 0.030); 1:100 front distance 0.1192 m (limit 0.030); 1:50 depth 13.8428 m against 13.853 m; 1:50 elevation distance 0.0597 m (limit 0.015); 1:50 front distance 0.1837 m (limit 0.015); 1:50 plan distance 0.0191 m (limit 0.015).
- After: 0 failing checks.
- Changed (119 ops in `repair/tulip-tree.json`): declared depthM 13.843; 8 × 1:500 filled shapes retraced, 7 × 1:500 solids moved, 13 × 1:200 filled shapes retraced, 6 × 1:200 solids moved, 29 × 1:100 filled shapes retraced, 5 × 1:100 solids moved, 50 × 1:50 filled shapes retraced.
- Proof: `tulip-tree | silhouette: exit 0  Test Files 1 passed (1); Tests 1 passed | 384 skipped (385);  | validate: 0 issues	0 before, 0 new | check: ✓ tulip-tree (1/1) | three-views: exit 0  Test Files 1 passed (1); Tests 2 passed | 773 skipped (775); | physics: exit 0  Test Files 1 passed (1); Tests 2 passed | 838 skipped (840); | breathing: exit 0  Test Files 1 passed (1); Tests 1 passed | 289 skipped (290);`

### `weeping-willow`

- Before: 18 failing checks: 1:500 depth 13.8521 m against 14 m; 1:500 elevation distance 0.2445 m (limit 0.150); 1:500 plan distance 0.5363 m (limit 0.150); 1:200 width 16.1077 m against 16.212 m; 1:200 depth 13.8978 m against 14 m; 1:200 elevation distance 1.4798 m (limit 0.060); 1:200 front distance 1.5144 m (limit 0.060); 1:200 plan distance 1.0003 m (limit 0.060); 1:100 width 16.1243 m against 16.212 m; 1:100 depth 13.8854 m against 14 m; 1:100 elevation distance 1.0050 m (limit 0.030); 1:100 front distance 0.7800 m (limit 0.030); 1:100 plan distance 0.8796 m (limit 0.030); 1:50 width 16.1243 m against 16.212 m; 1:50 depth 13.8854 m against 14 m; 1:50 elevation distance 0.6300 m (limit 0.015); 1:50 front distance 0.5044 m (limit 0.015); 1:50 plan distance 0.4461 m (limit 0.015).
- After: 0 failing checks.
- Changed (344 ops in `repair/weeping-willow.json`): declared widthM 16.124, heightM 11.094, depthM 13.885; 5 × 1:500 filled shapes retraced, 77 × 1:200 filled shapes retraced, 3 × 1:200 shapes removed, 2 × 1:200 solids moved, 19 × 1:200 strokes redrawn, 101 × 1:100 filled shapes retraced, 136 × 1:50 filled shapes retraced.
- Proof: `weeping-willow | silhouette: exit 0  Test Files 1 passed (1); Tests 1 passed | 384 skipped (385);  | validate: 0 issues	0 before, 0 new | check: ✓ weeping-willow (1/1) | three-views: exit 0  Test Files 1 passed (1); Tests 2 passed | 773 skipped (775); | physics: exit 0  Test Files 1 passed (1); Tests 2 passed | 838 skipped (840); | breathing: exit 0  Test Files 1 passed (1); Tests 1 passed | 289 skipped (290);`

## Files

Updated (catalog, 19): `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌳️vegetation/<slug>/🔣️.json` for ivy-on-trellis, japanese-cherry, japanese-maple, lombardy-poplar, london-plane-pollarded, miscanthus-grass, multi-stem-birch, norway-maple, olive-tree, red-oak, rowan, saucer-magnolia, scots-pine, small-leaved-lime, stone-pine, sweetgum, sycamore-maple, tulip-tree, weeping-willow.

Updated: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS/requests.md` (section "silhouette SV-2" appended).

Created under `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS/`: `silhouette-SV-2.md`; in `🧰️silhouette/SV-2/`: `env.sh`, `lib.ts`, `measure-lib.ts`, `ops.ts`, `spans.ts`, `measure.ts`, `m.sh`, `diag.ts`, `d.sh`, `dbg.ts`, `axes.ts`, `fit-box.ts`, `retrace.ts`, `slim.ts`, `rebind.ts`, `apply.ts`, `validate.ts`, `roundtrip.ts`, `fix.sh`, `t.sh`, `vt.sh`, `prove.sh`, `look.sh`, `restore.sh`, `fix-stone-pine-ground.py`, `manifest-changes.py`, `manifest-changes.json`, `report-rows.py`, `write-report.py`, `repair/<slug>.json` (19).

Removed: `🗑️generated/silhouette/SV-2` (logs, sheets, the copies of the 19 files before the repair; `restore.sh` and the `--against` proof need those copies and work only while they exist).

## What stays red

- Nothing in `🧊️catalog-silhouette`, `validateAsset`, the check CLI, `🧊️three-views`, `🌡️catalog-physics` and `🫁️breathing-catalog` for the 19 plants.
- Not run: `✅️asset-validation` and `⚡️asset-power` as whole suites (the brief forbids a suite without a name filter, and a filter by a plant of this batch selects no test). Expected from the facts above: the manifest tests report the 14 plants of `manifest-changes.json` until the coordinator applies it.
- `checkVolumes` as a whole was not run after the repair (its silhouette rule is what the real test measures; the cross-scale and connection findings that 8 of these plants had before were not measured again). The moved lobes went towards the 1:50 body, which is the direction `volumes-cross-scale` asks for.
- The check CLI of `multi-stem-birch` needs more than 15 minutes on this host (it passes; the first proof run stopped it at 900 s).
