"""📝️ Writes `U/silhouette-SV-2.md` from the fixed text below and the rows of `report-rows.py` (which reads the logs of this session); run before 🗑️generated/silhouette/SV-2 is deleted.
python3 write-report.py"""
import os, subprocess, sys
HERE = os.path.dirname(os.path.abspath(__file__))
rows = subprocess.run([sys.executable, os.path.join(HERE, "report-rows.py")], capture_output=True, text=True, check=True).stdout
HEAD = """# 🌳️ Silhouette Fixer Report: Batch SV-2 (19 Plants)

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

"""
TAIL = """## Files

Updated (catalog, 19): `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌳️vegetation/<slug>/🔣️.json` for ivy-on-trellis, japanese-cherry, japanese-maple, lombardy-poplar, london-plane-pollarded, miscanthus-grass, multi-stem-birch, norway-maple, olive-tree, red-oak, rowan, saucer-magnolia, scots-pine, small-leaved-lime, stone-pine, sweetgum, sycamore-maple, tulip-tree, weeping-willow.

Updated: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS/requests.md` (section "silhouette SV-2" appended).

Created under `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS/`: `silhouette-SV-2.md`; in `🧰️silhouette/SV-2/`: `env.sh`, `lib.ts`, `measure-lib.ts`, `ops.ts`, `spans.ts`, `measure.ts`, `m.sh`, `diag.ts`, `d.sh`, `dbg.ts`, `axes.ts`, `fit-box.ts`, `retrace.ts`, `slim.ts`, `rebind.ts`, `apply.ts`, `validate.ts`, `roundtrip.ts`, `fix.sh`, `t.sh`, `vt.sh`, `prove.sh`, `look.sh`, `restore.sh`, `fix-stone-pine-ground.py`, `manifest-changes.py`, `manifest-changes.json`, `report-rows.py`, `write-report.py`, `repair/<slug>.json` (19).

Removed: `🗑️generated/silhouette/SV-2` (logs, sheets, the copies of the 19 files before the repair; `restore.sh` and the `--against` proof need those copies and work only while they exist).

## What stays red

- Nothing in `🧊️catalog-silhouette`, `validateAsset`, the check CLI, `🧊️three-views`, `🌡️catalog-physics` and `🫁️breathing-catalog` for the 19 plants.
- Not run: `✅️asset-validation` and `⚡️asset-power` as whole suites (the brief forbids a suite without a name filter, and a filter by a plant of this batch selects no test). Expected from the facts above: the manifest tests report the 14 plants of `manifest-changes.json` until the coordinator applies it.
- `checkVolumes` as a whole was not run after the repair (its silhouette rule is what the real test measures; the cross-scale and connection findings that 8 of these plants had before were not measured again). The moved lobes went towards the 1:50 body, which is the direction `volumes-cross-scale` asks for.
- The check CLI of `multi-stem-birch` needs more than 15 minutes on this host (it passes; the first proof run stopped it at 900 s).
"""
out = os.path.join(HERE, "..", "..", "silhouette-SV-2.md")
open(out, "w").write(HEAD + rows + TAIL)
print(os.path.abspath(out))
