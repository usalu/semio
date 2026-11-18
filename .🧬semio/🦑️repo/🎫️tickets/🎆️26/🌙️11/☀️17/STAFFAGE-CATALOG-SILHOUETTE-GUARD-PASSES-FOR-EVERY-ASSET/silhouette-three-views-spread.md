# Three-views `spread-*` repairs after the silhouette pass

Result: `three-views` (10 tests) and `catalog-silhouette` (5 tests) pass for pigeon, person-cooking, city-bicycle, horse and jogger; `validateAsset` is 0 for all five. Tests and guard untouched; only authored data edited (byte-stable span edits).

## Mechanism

Flat front/plan shapes are carried by the 3D transform of their bone (rest to current) with a depth profile from the elevation twin. A traced polygon therefore only matches at the pose it was traced in (default pose); in the stretch or graze poses it is carried wrongly. Near-vertical `followTip` bones (jogger shank) make the inverse ill-conditioned: traced rest coordinates reach 0.5 to 1 m and blow the plan extent. Extruded or revolved `solid`s and `followTip` capsules are projected in true 3D and stay right in every pose. A bone-attached flat patch that stands in for another body part (jogger torso fits at the arm) leaves the figure when the pose changes.

## Fixes

- pigeon 1:50 front near/far toes: rectangle widened along the bone axis by 0.004 m (toe is nearly vertical, so this is the height); 0.006 m already broke front IoU (0.916).
- person-cooking 1:200 plan head: traced polygon was 0.17 m in x off the volume head (silhouette hides it, Hausdorff tolerance); shifted (-0.168, +0.027).
- city-bicycle 1:200 plan: grips were missing from the followed handlebar members (members to z +-0.31) and the world-fixed `sweep-l/r` bar traces were clipped to |y| <= 0.27 so a steered bar stays within the front extent.
- horse: plan `head` (all scales) and front `neck-2` at 1:200 restored from the solid-bearing originals; 1:50 plan head solid half width 0.095 to 0.088 (nose corners, Hausdorff 0.0173 to pass). Front necks/head at 1:100/1:50 keep the traced polygons.
- jogger: plan `shank-r` at 1:50/1:100 restored to the followTip capsule; dropped `fit-shank-r-50-2/-3`, `-100-3`, `-200-1`, `fit-thigh-l-500-1`, `fit-torso-50-5/-6`, `-100-1`, `-500-2`, front `fit-toe-r-50-15` (hangs 0.1 m under ground), front `fit-torso-100-5/-6/-7/-15`; front `torso` 1:100 and `head`/`hair` 1:200 restored from the old versions (solid / followTip). `fit-torso-50-1` is kept (needed by plan Hausdorff).

## Tools (prefix `spread-`)

`spread-sweep.ts` (sweep findings), `spread-rev.ts` (sweep at a revision), `spread-ext.ts` (extremal shapes per view), `spread-out.ts` (shapes leaving the other view's range), `spread-solids.ts` (projected volume solids), `spread-clu.ts` (silhouette mismatch clusters), `spread-diff.py` (shapes changed against a revision), `spread-lib.py` and the per-asset `spread-*-N.py` edit scripts, `spread-chk.sh` (sweep plus triage).
