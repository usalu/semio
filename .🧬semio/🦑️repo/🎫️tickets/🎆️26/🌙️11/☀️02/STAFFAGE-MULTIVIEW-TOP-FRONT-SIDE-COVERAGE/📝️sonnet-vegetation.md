# 📝️ sonnet-vegetation: per-asset status (16 assets)

Scripts: `T/🛠️scripts/vegetation/` (audit4.ts extent audit, bandfix.ts, symbols.ts, trunks.ts, fill.ts, anchorz.ts, sway.ts, unsway.ts, ranges.ts, resid*.ts, plume.ts, proof.sh).
Proofs: `T/🗑️generated/vegetation/proof/<asset>/` (three views side by side, light solid, 1:500, 1:200, 1:100, 1:50, plus one sheet per symbol).
Backlog evidence: `T/🗑️generated/vegetation/failures-now.json` (before the sway revert), `failures-after.json` (current), `d29-before-revert.txt`.
Extent rule used: within max(2 %, 15 mm) of `dimensions` at the default pose, ground excluded, every authored band and every true-extent symbol (`audit4.ts`: TOTAL 0).

## Baseline audit (before)

- Authored bands 500/200/100/50: all three views existed for every authored band of all 16 assets. Hosta 1:500 is unavailable (alternatives: symbol plan-extent, scale 200).
- 136 extent failures. 105 of them were symbol failures: no true-extent symbol had a front, 7 had no elevation, 8 had a plan with swapped axes (oak, palm, hornbeam, poplar, miscanthus, birch, pine, willow: plan x was the depth; the elevation was the plan outline lying on its side). The four nominal `tree-sign` symbols lacked elevation and front.
- Bands: crown masses in plan disagreed with the front laterally (oak low masses front +-4.9 m, plan +-0.4 m) and with the elevation forward (maple, plane, birch, lime, pine, willow). Width or depth were 2 to 3 % off for oak, apple, poplar, pine, willow, palm, hosta, bamboo, hedge 1:500, birch 1:500, miscanthus 1:200.
- Miscanthus front height was 1.655 against 1.750 m: the drawing was wrong (front plumes too short), `dimensions` kept.
- Plans lacked the trunk dot (all trees) and the branch skeleton at fine scale. Plan crowns of the broad trees were diagonal ribbons (ellipse coverage 0.42 for the oak).
- Sway: sagittal only. Wind bearing is 0 (along +x) for all 16 assets, so plan x and front height moved, but plan y and front x never did.
- Four assets had a one-denominator gap between valid ranges (agave, bamboo, hosta, ivy).

## Done, per asset

| Asset | Bands 500/200/100/50 | Symbols (3 views each) | Notes |
|---|---|---|---|
| agave | unchanged, passes extents; valid-range gaps closed | plan-extent redrawn (rosette) | 3D-chain asset; 18 `lateral-ambiguous` remain (below) |
| apple-tree | plan masses aligned, extents exact, trunk dot, scaffold limbs, crown fill (coverage 0.49 to 0.88) | plan-crown, tree-sign | emitter anchors z set |
| bamboo-clump | plan and front lateral fixed, culm dots at 1:50, valid-range gaps closed | plan-extent | |
| date-palm | outer fronds widened (front x scale, plan tip radius), 1:500 crown scaled, depth 7 m | plan-extent | 16 `lateral-ambiguous` remain |
| english-oak | plan masses moved to the front diagonal, extents exact, trunk dot, limbs, crown fill (0.42 to 0.90) | plan-extent | |
| hornbeam-hedge | 1:500 front 0.74 to 0.70 m | plan-extent | |
| hosta-clump | blade radii and 1:100 blade ends fixed; `cause: below-printable-feature` on 1:500; valid-range gaps closed | plan-extent | 18 `lateral-ambiguous` remain |
| ivy-on-trellis | passes | plan-extent | valid-range gaps closed |
| japanese-maple | tier masses moved to elevation x, extents exact, stem dots | plan-crown, tree-sign | |
| lombardy-poplar | extents exact, trunk dot, branch lines | plan-extent | |
| london-plane-pollarded | plan masses moved to elevation x, trunk dot, shoots, crown fill | plan-crown, tree-sign | |
| miscanthus-grass | front plumes lengthened to reach 1.75 m, plan plumes matched (0.403 m), 1:200 plan width fixed | plan-extent | |
| multi-stem-birch | plan masses moved, extents exact, 3 stem dots | plan-extent | |
| scots-pine | extents exact, trunk dot, limbs | plan-extent | |
| small-leaved-lime | plan masses moved, extents exact, trunk dot, limbs, crown fill | plan-crown, tree-sign | |
| weeping-willow | curtains moved to elevation x, extents exact, trunk dot, crown fill | plan-extent | |

Emitter anchors: lateral `z` set from the front position of each mass for 11 assets (apple, bamboo, oak, maple, poplar, plane, miscanthus, birch, pine, lime, willow). Agave, palm and hosta are skipped because their bones already carry the lateral position through the 3D chain.

Thermal bindings: leaf, needle, blade and frond emitter regions exist in the front and plan of every band of every asset (`parity.ts`). Bark regions of trunks and limbs are missing in plan or front at 1:200 and 1:500 for oak, willow, lime, pine, apple, poplar, plane, birch and the palm stem. They are hidden under the crown from above and below the print minimum, so the thermal view paints the crown only. Where a trunk is visible, the plan has a dot with the trunk-bark region.

## D29 outcome: crosswind sway (θ_max = largest out-of-plane angle of a bone, error = largest flattening error on paper)

The crosswind `abductionDeg` keys (about 0.4 of the along-wind swing, quarter-cycle lag) were published to D28 as an experiment. Every tree that carried them was flagged, so all were reverted to pure along-wind sway (bearing 0, which matches D12).

| Tree | θ_max | Largest error | Outcome |
|---|---|---|---|
| apple-tree | 12.5° | 10.97 mm | reverted |
| bamboo-clump | 32.9° | 17.67 mm | reverted |
| english-oak | 24.2° | 67.89 mm | reverted |
| hornbeam-hedge | 7.0° | 3.75 mm | reverted |
| ivy-on-trellis | 9.9° | 2.61 mm | reverted |
| japanese-maple | 6.8° | 5.75 mm | reverted |
| lombardy-poplar | 26.2° | 54.05 mm | reverted |
| london-plane-pollarded | 7.7° | 9.28 mm | reverted |
| miscanthus-grass | 19.0° | 3.56 mm | reverted |
| multi-stem-birch | 23.4° | 26.23 mm | reverted |
| scots-pine | 23.9° | 43.63 mm | reverted |
| small-leaved-lime | 6.5° | 9.92 mm | reverted |
| weeping-willow | 23.9° | 61.29 mm | reverted |

No tree kept its crosswind. After the revert, 13 of 16 assets have zero failures in the current contracts backlog (`failures-after.json`). In plan and front these trees still follow the sway: plan x moves with the bone travel, the front shows the height change.

## Open

- **agave, date-palm, hosta-clump (52 `lateral-ambiguous`):** their own spread poses (θ_max 92°, 80.2°, 84°; errors 18.07, 40.55, 6.07 mm) leave flat leaf, frond and blade shapes in the elevation and plan. They are pre-existing rig data, not my sway keys. The fix is to convert those shapes to solids (extruded leaf plates, revolved frond bundles), which is a rig-level rewrite of about 50 shapes per asset and needs a ruling on the intended leaf thickness. Not done.
- Other deliverables pending a re-check: the proof sheets of all assets must be inspected (only the oak 1:100 was inspected so far).
