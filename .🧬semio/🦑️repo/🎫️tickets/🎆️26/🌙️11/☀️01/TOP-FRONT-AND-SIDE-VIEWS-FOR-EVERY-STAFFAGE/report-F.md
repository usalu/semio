# Report F: Furniture (12 Assets)

**Assets:** bar-stool, bookshelf, cafe-chair, dining-table, folding-deck-chair, height-adjustable-desk, office-swivel-chair, operating-table, park-bench, picnic-table, single-bed, three-seat-sofa.

## State
- 47 authored representations (1:500 to 1:50) have front and plan views. operating-table 1:500 stays unavailable.
- `presentation.viewpoints` is removed. The en/de `decisions.silhouette` texts are extended.
- Builders: `T/🧰️F/<asset>.py`; pristine originals in `T/🧰️F/orig/`.

## Method
- Rest-pose metres with the root at the origin. Scales: 1:500 is a silhouette, 1:200 shows masses, 1:100 is articulated, and 1:50 has full detail.
- **Moving parts** use `solid: extruded`, so the motion reads in front and plan: deck-chair rails and canvas, picnic benches, bed mattress sections and slats, office-chair backrest, operating-table carrier and pad.
- **Sections instead of end views:** the bookshelf and sofa fronts are sections, because the side panel or arm would hide the motion.

## Verification
- **`check`:** passes for all 12 assets; every scale is suitable under `office-laser`. office-swivel-chair 1:500 was fixed with a hand-drawn backrest profile.
- **Consistency:** within tolerance everywhere. operating-table 1:100 was fixed (the arm end was added at 1:100 and 1:200), but `render.ts` was not re-run afterwards.
- **Three-views vitest** (filtered to these 12 assets): 24 passed.
- **Previews:** every pose was inspected on all 12 assets, and most clips were checked. The second-round sheets for operating-table and office-swivel-chair were not seen.

## Doubts (handed to QA7)
- **bar-stool:** swivel is stored only as backrest x travel, so at 90° the plan backrest drifts off the seat circle.
- **Tilted extruded parts** (operating-table pad, rails) show as axis-aligned bounding boxes in front and plan.
- **cafe-chair:** the tilt clip shears the plan slightly.
- **JSON formatting:** files were rewritten with `indent=2`, which differs from the repo's float formatting.

## QA7 update (visual QA and fix)
- **bar-stool swivel:** the seat bone is now vertical and revolute (angle limits 0..0) and carries `twistDeg` in turned-90, turned-270-key, occupied-turned-key and the swivel clips. The backrest is its fixed child at radius 0.14 m, a tangential bar 0.10 m wide, and orbits the seat circle in plan and front. Plate, pad and ring are drawn on the frame so the round seat keeps its width. Builder: `🧰️F/bar_stool.py` (`pre`). At exactly 90 degrees the side view shows the backrest as a thin post at the axis (flat-profile limit).
- **Tilted extruded parts (operating-table):** pad and carrier end faces are drawn as extra extruded slices over the tilt envelope, and the plan pad joints are thin extruded strips that follow the tilt.
- **cafe-chair plan shear:** seat and back panel are extruded solids in plan, so every part follows the root tilt.
- **JSON formatting:** each file keeps its original indent (bookshelf 1, others 2), decimals instead of exponents. Diffs are now additive.
- **office-swivel-chair:** the swivel via `twistDeg` on a vertical seat bone was tried (`🧰️F/office_chair.twist.py.txt`). The guard failed at intermediate angles: plan x-range [-0.348, 0.325] against elevation [-0.268, 0.325] m, because the 0.44 m wide backrest sweeps laterally, which the flat elevation profile cannot show. The file is back to the static clip.
- **Verification:** check and consistency clean for all 12 assets at every authored scale; three-views vitest 24 passed. 579 sheets rendered (every pose, three instants per clip, every scale); montages were inspected for bar-stool, cafe-chair, operating-table and folding-deck-chair; the other assets were checked by guard and spot checks only.
- **QA7 round 3:** height-adjustable-desk 1:500 had its leg lines detached from the feet at raised heights (evidence: `🗑️generated/evidence-QA7/desk-500-before-legs-detached.png` and `...after-raise-and-lower-15s.png`). Sleeve and stage lines were added in elevation and front. Park-bench and sofa details were raised above their bodies for the same-region merge rule. Builders now write atomically (`lib.py` `atomic_write`).

## QA7 update 4 (solids landed)
- **bar-stool:** plate and pad are revolved solids on the twisting seat bone (plate) and the pad bone, and the backrest post and pad are extruded solids (0.10 m wide pad at radius 0.14 m) in elevation, front and plan. At 90 degrees the side view shows the pad face-on and the front view shows its depth. Check and guard are clean. Builder `🧰️F/bar_stool.py`.
- **office-swivel-chair:** the swivel is an axial twist of the vertical seat bone. Seat cushion, mechanism, armrest posts and pads, backrest shell and strut (as far as authored per scale) are extruded solids shared by all three views. The clip `swivel-rotation` swings a quarter turn to each side (poses `swivel-quarter` and `swivel-quarter-opposite-key`). Check and guard are clean. The star base stays static (consistent in all views), and detail lines stay flat shapes. Builder `🧰️F/office_chair.py` with `office_chair_base.py`.
- **All 12 assets after the single 3D path:** check, `consistency.ts`, hidden-detail scan and the three-views vitest (24) are clean. Evidence: `🗑️generated/evidence-QA7/recheck-single-path-1to50.png`.

## QA7 update 5 (revolve axis, 1:100/1:200/1:500 read)
- **bar-stool revolve axis:** plate and pad now both revolve about the vertical `seat` bone (the pad had used the horizontal `frame` bone, which the new rule rejects). Default-pose consistency, check and guard are clean.
- **office-swivel-chair 1:500:** the front column line started at 0.225 m and left a gap above the star base. It now starts at 0.05 m (`🧰️F/office_chair_base.py`), so the column reaches the base at every seat height.
- **Read at 1:100, 1:200 and 1:500 (zoomed crops):** bar-stool (unoccupied, occupied, turned-90) and office-swivel-chair (seat-low, swivel-quarter, reclined, seat-high at 1:500). The swivel reads in all three views; the 1:500 silhouettes are fat but consistent.
- **Open:** `T/audit-sheets-F.md` had not landed when this was written; its findings are not included.
- **State:** all 12 assets pass check, consistency and the hidden-detail scan; three-views vitest 24 of 24.

## QA7 update 6 (audit-sheets-F follow-ups)
- **park-bench (legitimate, no change):** the occupant and sun states are load and thermal states with no geometric motion by design (poses `occupied-one`, `occupied-two` have no bone offsets; the clip `sun-exposure-cycle` animates `sunlitFraction` through tracks). I checked that the states differ where the data says they should: the thermal view differs between `unloaded` and `occupied-two` and between clip instants 0 s and 30 s, and the forces view differs between `unloaded` and `occupied-two`. The dark-line views cannot show them, as for every other furniture asset (occupants belong to the humans catalog). The same holds for picnic-table `occupied` and `bench-proof-load`.
- **three-seat-sofa:** the 1:500 front silhouette now uses the same leaning backrest front face as 1:200 to 1:50 (from (0.265, 0.62) to (0.30, 0.82) instead of an upright edge). The elevation is the facade and stays upright at every scale; the lean exists only in the side section.
- **operating-table (real clash, fixed):** the light stand column stood at x = 0.62 m and lateral 0, inside the 1.0 m half-length and 0.25 m half-width of the pad, so the column passed through the pad at every pose, tilted or not. The stand now stands beyond the head end (column at 1.17 m; the tilted top reaches x = 0.974 m at 13 degrees, a clearance of 0.17 m). The arm still reaches back over the table. `dimensions.widthM` is 2.0 -> 2.45 m, the plan stand shapes moved by +0.55 m, and the `decisions` size sentences and the structure assumptions are updated. Check, consistency and guard are clean.
- **single-bed:** the centre support leg is drawn at 1:500 in the side view. The raised head section is drawn in plan as its projected footprint; the open frame under the lifted section is correct (the deck lifts with the section), so I left it.
- **picnic-table stowed (legitimate):** the benches hang from a hinge at 0.40 m with a 0.30 m board, so their lower edge is 0.10 m above the floor, which is correct.
- **office-swivel-chair 1:500:** the seat-low backrest started 0.015 m above the seat band; it now starts at the hinge, so it touches. The backrest and arms outside the seat outline in plan at large swivel are physically true: the 0.44 m wide backrest sweeps a radius of about 0.32 m about the column.
- **Re-run:** check (12 of 12 clean), consistency, hidden-detail scan, the three-views vitest (24) and the lateral/axial config (216) pass. Evidence: `🗑️generated/evidence-QA7/audit-fixes-optable-sofa-bed-chair.png`.
