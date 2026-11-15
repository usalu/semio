# Author 02 — vegetation (8 assets): status report

Assets: japanese-maple, lombardy-poplar, london-plane-pollarded, miscanthus-grass, multi-stem-birch, scots-pine, small-leaved-lime, weeping-willow.

## Result, stated plainly

**Nothing is spliced. No asset meets the brief's quality bar. None of the 8 assets passed a full `volumes-check.ts` run.** I did not finish the task.

- Sources exist for all 8 assets at all four scales: `volumes/vegetation/<slug>.json`.
- The final full checks (`🗑️generated/author-02/<slug>.check.txt`) were still running when I wrote this. Each takes more than 5 minutes with 150–700 solids per scale. I have no check result for any asset on the final sources.
- Only partial maple runs gave me numbers, listed below.
- The generated sources are untested drafts.

## What was built

- `author-02-lib.ts`
  - Kit that works from each scale's own 2D shapes in the default pose.
  - Limb lofts: z centre and depth come from the front or plan outline of the same shape.
  - Lobes as stacks of frusta: the exact plan outline, scaled by the front width profile.
  - Visual-hull lofts from elevation, front and plan.
  - Area calibration.
- `author-02-tree.ts`: thermal builders (bark and foliage by `{ of }` reference, emitter patches), structure builders (members along bones or limb paths, joints, mass ellipsoids, fixed support carrying `anchor`).
- `author-02-maple.ts`: hand-tuned maple script.
- `author-02-gen.ts <slug>`: generic generator for the other seven assets.
- `author-02-run.sh`: condensed check summary.

## Per asset (solid counts body/thermal/structure at 1:50, 1:100, 1:200, 1:500)

| asset | counts | note |
|---|---|---|
| japanese-maple | 219/231/31, 98/110/31, 44/62/26, 4/19/13 | hand-tuned script |
| lombardy-poplar | 485/494/53, 196/204/53, 150/158/32, 2/15/23 | generic generator |
| london-plane-pollarded | 328/339/44, 161/172/44, 77/88/27, 2/32/19 | generic generator |
| miscanthus-grass | 238/250/62, 72/84/62, 26/38/38, 1:500 not read | generic generator |
| multi-stem-birch | 391/400/53, 197/206/53, 135/144/32, 2/23/23 | generic generator |
| scots-pine | 330/341/33, 165/175/33, 99/109/21, 3/26/14 | generic generator |
| small-leaved-lime | 573/591/49, 286/322/49, 172/225/32, 2/56/19 | generic generator |
| weeping-willow | 692/702/48, 328/337/48, 166/175/30, 3/20/20 | generic generator |

## Measurements and renders

**Maple 1:50, earlier partial runs:**
- Silhouette IoU elevation 0.88, front 0.94, plan 0.995.
- Hausdorff 90–165 mm against a limit of 15 mm.
- 1:200 body IoU: elevation 0.67, front 0.53. The plan-fill hull spilled outside the front drawing. I changed it to a visual hull but did not re-check it.

**Renders viewed:**
- Maple 1:50 (earlier version): layered tiers of foliage pads look right. The stacked frusta show visible stepping.
- Poplar 1:50: front IoU 0.969, plan 0.998, elevation 0.772. The 2D elevation alternates left and right while front and plan do not show it. Structure members run outside the body in elevation.
- Miscanthus 1:50: bad. IoU 0.51 / 0.60 / 0.47, and the blades fall apart into fragments. The generic approach does not suit grass.
- No other asset or scale has been looked at.

## Known defects

1. **Hausdorff.** The silhouette limit is 2 × feature, 15 mm at 1:50. The 2D foliage outlines are scalloped clouds, and the three views disagree with each other by 30–50 mm. For example, in maple the east tier is drawn about 50 mm higher in elevation than in front. I expect this limit to be missed at 1:50 and 1:100 for every foliage asset. I did not measure the full set and wrote no entry in `author-requests.md`.
2. **Maple 1:100 and 1:500 coverage.**
   - Tier bones have a `detail` polyline at 1:100. I added capsules for them but never checked them.
   - 1:500 root-plate needs its own solid. I added a root collar but never checked it.
3. **Cross-scale.** 1:50 to 1:100 differences come largely from the 2D archives themselves. 1:200 to 1:500 was untested.
4. **Thermal.**
   - Per ruling C1, foliage and bark use `{ of }` references to exposed solids. I did not correct `areaM2` at the root and have no old or new values.
   - Maple's 1:50 thermal list holds many `{ of }` entries because it follows slab count. Leaf-region area deviation is unmeasured.
5. **Slab counts.** The generic assets have hundreds of frusta slabs per scale. The check is slow, and the model looks terraced rather than smooth.
6. **Decisions text.** Maple still has the placeholder decision `TODO`. The generic assets have one generic bilingual sentence per scale. Neither is the per-scale rationale the brief asks for.
7. **Structure.** Members have not been verified to lie inside the body silhouette. The new product validator requires this within one feature.
8. **Untested rules.** I have not checked `volumes-copied` or `volumes-monotonic`, and the spec for the new fixed-quality triangle count (ruling C4) has not been looked at.
9. **Tool timing.** `volumes-check.ts` took more than two minutes on a single maple run. A full run of 8 assets in parallel was too slow to iterate on. I added no entry to `tool-requests.md`.

## Suggested next steps

- Reduce slab and solid counts per lobe.
- Replace miscanthus with a dedicated blade-sheaf model.
- Run the checks and read every render sheet.
- Fix the coverage and silhouette failures.
- Write the real per-scale decisions.
- Then splice with `volumes-insert.py`.
