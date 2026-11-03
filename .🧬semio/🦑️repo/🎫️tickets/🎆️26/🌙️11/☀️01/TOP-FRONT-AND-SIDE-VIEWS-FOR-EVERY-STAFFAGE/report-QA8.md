# Report QA8: Animals visual QA and fixes (mallard-duck, pig, pigeon, red-fox, red-squirrel, roe-deer, sheep)

## Result
Final driver round (`T/🧰️QA8/all.ts`): `check` OK for all seven assets at every authored scale (office-laser), three-views sweep and consistency `ok` at every scale, including the spread and twist guards. Three-views vitest filtered to the seven assets: 14 passed. Full suite not run.

## Fixes
- **Fox 1:200 cannon min-feature**: not the elevation. The front followTip trapezoid of the cannon squashes to a sliver in poses where the bone lies horizontal. The cannon rows now carry a rigid pad (`over`/`under`) at 1:200. Also a plan muzzle disc and a longer forepaw capsule at 1:200 so the plan x range agrees with the elevation in `mousing-pounce`.
- **Necks and tails (boxy)**: the builder `ext` in `T/🧰️A2/lib.py` now decides per bone. Bones tilted at rest (necks) become tapered followTip trapezoids. Horizontal bones (tails, brush) become a stack of extruded slices (5 at 1:50, 3 at 1:100, 1 at 1:200) whose width tapers.
- **Torso in steep poses**: FW's full-3D placement now carries rigid front shapes by pitch, so upending, climbing and grazing foreshorten correctly. The duck and pigeon trunk additionally get a narrow hidden core solid (inside the body ellipse at rest) so the extent follows steep trunks.
- **Wings**: duck `wing-stretch` has `abductionDeg` -55/+55 with `Bone.z` -0.058/+0.058; front wing blobs are followTip (end-on bone). Pigeon: spread 20..60 degrees in `wings-half-open`, all take-off and airborne poses, `Bone.z` +-0.05 on the humeri; front wing parts are extruded slab solids (3D carry gives V shaped and drooping wings in the front sheets). Fox: `twistDeg` 28 (t=1) and 12 (t=3) on `head` in `mousing-pounce`. The duck 1:200 got far/near wing strokes in front and plan.
- Pigeon toes front as extruded solid (the followTip wedge collapsed in take-off). Roe-deer 1:500: front head and ears are strokes (the foreshortened head in `grazing` fell under 0.2 mm).
- Asset writes are atomic (`T/🧰️A2/jsonfmt.py` uses temp file plus `os.replace`).

## Tools (`T/🧰️QA8/`)
`all.ts` (one process: check, consistency, `--views` sheets for every pose and 3 instants per clip at every authored scale, `--no-png`), `png.py` (one-process rasterizer), `zoom.py` (crop and stack sheets), `rebuild.py` (regenerate front/plan from the A2 builders without evaluating), `bonez.py`, `abduct.py`, `maxx.ts`, `maxy.ts`, `boxes.ts`, `ring.ts`. PNG per sheet through the CLI took about a minute each on the loaded host, hence SVG only plus one rasterizer process.

## Inspected (real pipeline `--views` sheets, zoomed)
Duck: wing-stretch, dabbling (1:50). Pigeon: airborne-wings-up/down, take-off and half-open poses (1:50). Fox: all poses and pounce instants (1:50). Squirrel: all poses and bound, climb, nibble instants (1:50). The other scales, pig, sheep and deer sheets were generated (265 PNGs) but only checked by the guard, not by eye.

## Remaining doubts
- Fox `curled-sleeping` plan shows an extended body; a curl needs lateral spine, neck and tail bending (`abductionDeg`) that the pose lacks.
- Fox brush in front is a stepped column when raised (mousing-crouch, pounce-land); pigeon wing slices show segment ticks in front.
- Dabbling duck front: ball body plus wedge neck is schematic.
- Decision texts still say "extruded masses" for the brush and necks; not rewritten for the slice and trapezoid change.
- Pig, sheep, deer sheets at 1:100 to 1:500 and all clips except the ones above were not inspected visually.

## Round 2 (single 3D path, elevation solids)
- **Final driver round:** `check` OK, three-views sweep and consistency `ok` at every authored scale for all seven animals (278 sheets rendered through the real pipeline; squirrel re-run after the last edit). Pig 1:200 `cannon-f-n/f` (0.199 mm) fixed by a rigid pad on the cannon rows at 1:200 and later. The duck sweep failure after the flat-shape height rule was the wing: wings are now extruded solids with the side-view wing profile in elevation, front and plan (`Bone.z` +-0.058, z +-0.044..0.074).
- **Fox curled-sleeping:** `abductionDeg` lumbar 22, thorax 28, neck 40, head 25, tail-1/2/3 45/50/40 (curl in the horizontal plane); plan shows the C-shaped ring, side view foreshortens, guard passes. Elevation `hindbody` and `chest` carry extruded solids (validator `elevation-lateral`).
- **Workarounds removed:** hidden cores and slice stacks are gone. Tails, brush, pigeon wings, toes, sheep neck and squirrel tails and neck are single extruded solids whose profile is the side-view outline, and the same solid is written to the elevation shape (`T/🧰️A2/lib.py` `ext`, registry applied in `commit`). Revolved solids are not usable for necks and tails (their axis is the bone-frame vertical, not the bone axis), so tilted necks (duck, pigeon, deer) stay tapered followTip trapezoids.
- **Decisions texts (en/de):** every silhouette decision now ends with what is drawn: solids, spread and twist, curl, strokes at 1:500 for the deer, padded cannons (`T/🧰️A2/extra.py`).
- **Read by eye (montages of all poses and clip instants):** duck 1:50 and 1:200, pig 1:50, pigeon 1:50, fox 1:50, squirrel 1:50, deer 1:50, sheep 1:50 and 1:500. Not individually read: 1:100 and the other 1:200 and 1:500 montages (guard and check only).
- **Remaining doubts:** fox brush in front is a rectangle when raised (extruded prism); climbing squirrel front shows a gap between head and rump (torsos are rigid ovals, no solid along the bone axis); dabbling duck front is schematic; the deer 1:500 front head is a stroke.
