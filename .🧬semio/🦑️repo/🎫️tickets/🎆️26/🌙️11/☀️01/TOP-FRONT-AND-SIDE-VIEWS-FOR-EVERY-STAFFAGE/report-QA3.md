# Report QA3 (vegetation visual QA and fix)

Tools in `🧰️QA3/`: `all.ts` (check, optional `--probe` of min height and extents of every pose and 3 clip instants, `--views` sheets at 150 dpi rasterized in parallel inside one queued process), `montage.py` (8 sheets per montage), `abd_exact.py` (azimuth solver), `agave_apply.py`/`hosta_apply.py`/`palm_apply.py` (rig offsets) with `agave3.py`/`hosta3.py`/`palm3.py` (views), `grade.py`, `hedge.py`, `tipy.py`. Evidence: `🗑️generated/QA3/keep/`.

## Radial parts: exact azimuth through spread, twist and sagittal offsets
Rest angles are unchanged. Every leaf (agave 10, hosta 9) and frond (palm 8) bone `a` gets `angleDeg`, `abductionDeg` and `twistDeg` in every pose and every clip key so that its frame is the rest frame turned about the vertical by its azimuth (`abd_exact.solve_offsets`, closed form, verified against the engine chain: tips match to 1 mm). Bone `b` follows its parent and keeps only its own flex, so folding happens in the leaf's own radial plane. Pose angle offsets keep their meaning. Joint limits of the `a` bones are widened to cover the static azimuth offset (the limit describes the flex, the azimuth is carried by the same three offsets); agave tip bones to +-30 for the stronger poses.
- Views: plan = flat capsules in the bone's rest frame (`followTip`), front = extruded solids (lens profile in the bone frame, z swept), elevation = the existing bone-local outlines (now a true side projection). 1:500 composites stay static (caudex, root-plate or crown bound) and were re-expressed in that frame.
- agave: drought-infolded now reads (plan 2.39 to 1.80 m, leaves upright, height 1.53 m); frost-sag droops, with floors so no leaf goes below grade. The old plan lean and the boxy tip are gone (the front is a solid now).
- hosta: summer, rain-bowed and autumn-collapse (blades lie down to the clearance of the summer pose); the central vertical blade is a thin solid (it was a box). depthM 0.85 to 0.905 (true azimuth extent).
- date-palm: the eight bone fronds only (the seven extra "short" fronds of the earlier drawing were bound to the same bones at other azimuths and cannot be carried; dropped from plan and front, the elevation twin stays). widthM 6.87 to 6.3 (true side projection); the 1:500 crown was rescaled (x 0.917) to match. Plan leaflet textures and front rachis lines were dropped (rigid unfilled shapes cannot follow an azimuth).
- Not radial: apple, oak and bamboo masses and culms only lean with the wind in the sagittal plane; hedge blocks and ivy are planar.

## Below grade
`--probe` over all 8 assets, every pose and 3 instants per clip: agave frost-sag (-0.14 m) and hosta collapse (-0.23 m), flutter (-0.007 m) fixed through per-leaf angle floors; apple trunk, hedge stems and bamboo culm bases (1 to 13 mm) fixed in `grade.py`. Final probe: no query below -0.003 m.

## Other fixes
- hornbeam-hedge: plan and front cores are irregular seeded masses, ground strip 0.7 m.
- Earlier round: date-palm plan reach and widths, check gap at 1:500 (star spikes narrowed).

## Reading
All 366 sheets were rendered (8 montages of 8 sheets per asset and scale). Read in full: agave 50, 100, 200, 500 (first montages), hosta 50, 100, 200, palm 50, 100, 200, 500, apple 50, bamboo 50, oak 100, hedge 100, ivy 100. Not read one by one: the second montage of each asset and scale (wind-sway instants, 2 to 5 sheets each) and the remaining apple, bamboo, oak, hedge, ivy scales; the probe and the vitest cover them numerically.

## Checks
- `checkAssetDetailed` ok for all eight (physics off); three-views vitest 16 passed for the eight assets; `coverage.py --consistency`: all 8 assets complete, 31 of 31 representations ok.
- Doubts: plan and front of a leaf are two different rigid models (flat blade seen from above, slab seen from the side), so extents agree within tolerance but their widths do not match exactly; the palm lost the seven short fronds in front and plan.

## Round 3 (overrules)
- date-palm: the elevation never drew the seven short fronds (its geometry at 1:50 to 1:200 has only the eight bone fronds l1..l4, r1..r4; the short fronds existed only in the earlier front and plan drawing). Front and plan now match the elevation: eight bone fronds in all three views at every scale. Restored as shapes that follow the frond: plan leaflet margin strokes (capsules on the frond bones, 1:50), front rachis (1:200 and finer) and comb-shaped leaflet solids (1:50) as thin extruded solids on the frond bones. widthM stays 6.3 (true extent). The storm 1:50 failure (front x vs plan y, 0.38 m) came from rigid plan zigzags and is fixed by the capsules. The palm has no revolved solids (trunk and crown are paths on stem bones), so the revolve-axis note does not apply.
- Read by eye (14-sheet montages): all poses and clip instants, wind-sway included, of apple, bamboo at 50/100/200/500; oak, hedge, ivy at 50/200/500; palm 50/100/200; agave 100; hosta 100; plus the earlier readings. Nothing wrong found beyond the fixed points; oak 1:50 is low contrast by its tone set but intact.
- Final: three-views vitest 16 of 16, check ok for all eight, probe: nothing below grade. Evidence kept: `🗑️generated/QA3/keep/` (agave drought 1:100, hosta collapse 1:100, palm storm 1:100).
