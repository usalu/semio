# Report QA4: visual QA and fixes for the humans (H1 assets)

Assets: child-on-kick-scooter, courier-with-hand-truck, cyclist, elderly-with-cane, jogger, kneeling-gardener, leaning-person.

## Fixes
- **Courier parcels (front and plan, 1:50 to 1:500):** `extruded` solids on the `parcels` bone. The profile is the elevation parcel shape (1:500: one stack box), lateral range is the parcel half width. Tapes are thin extruded strips. No drift in tilt-and-settle.
- **Elderly cane ferrule, shaft and crook in plan:** `extruded` solids on the `cane` bone (the ferrule 1:100 and 1:50, the crook from the elevation path). 1:500 uses a 0.1 m wide shaft box centred on the axis, so the plan x range matches the elevation within tolerance.
- **Gardener hat brim in front:** `extruded` solid on the `head` bone from the elevation brim path, lateral range +-0.235 m; it follows the head pitch.
- **Jogger (new, stretch clip and guard):**
  - stretch-stand: `abductionDeg` upper arms +-28, forearms -+22 (elbows out, hands near the hips); `Bone.z` = +-0.175 on both upper arms; the same values in the clip keys that carry explicit stand bones.
  - Plan feet and toes are extruded shoe profiles (the followTip capsule overshoots when the foot points backward in the quad hold).
  - Front torso is an extruded solid (hamstring fold: a horizontal torso keeps its chest depth as height).
  - Head: followTip polys in front, a followTip pill capsule in plan (head down in the fold).
  - 1:200 head grown by 0.02 m so the fold stays within the 0.053 m tolerance.
- **Cyclist (new):** dismounted-astride thighs `abductionDeg` +-9, `Bone.z` +-0.095 on the thighs. Cranks (1:200) and hands (1:500) in front are extruded solids (the 3D placement collapses a followTip poly to a line when the bone is horizontal, which broke the 0.2 mm feature rule). Front thighs 1:500 narrowed to stay within the `elevation-lateral` limit.
- **Scooter:** front fork blades overlap the head tube and the plan fork blades overlap the stem (gap rule at 1:200, 1:100, 1:50).
- **Leaning person:** plan phone screen widened (0.2 mm feature rule at 1:50).
- Sources: `T/🧰️H1/a_*.py`, `human.py`, `lib.py` (new `extrude_like`, `extrude_poly`, `L`) rebuilt every asset; `T/🧰️QA4/lateral.py` writes the abduction values and `Bone.z`.

## Checks
- Driver `T/🧰️QA4/all.ts` (check, three-views sweep and consistency, `--views` sheets at 150 dpi for every pose and three clip instants at every authored scale; 282 sheets). `T/🧰️QA4/zoom.py` and `montage.py` crop the sheets for inspection, `suit.ts` isolates suitability violations per view and pose.
- Final run: check ok for all seven assets, no sweep or consistency finding (including the `spread-*` checks for jogger stretch and cyclist dismount).
- Vitest three-views for the seven assets: 14 passed.
- Inspected sheets: all four scales for the courier, cyclist and elderly (1:500 zoomed), jogger 1:50 (every stretch instant, stand, quad, hamstring), cyclist astride at 1:50 and 1:200, elderly 1:200.

## Final round (coordinator)
- **Hands on hips (jogger stretch-stand):** a grid search (`T/🧰️QA4/hips.ts`, real evaluation) found the pose: upper arms -30 degrees back and `abductionDeg` +-30, forearms 80 degrees with `abductionDeg` -+30, hands 0. The wrist lands at the hip (lateral 0.20 m, 0.11 m above the pelvis origin, elbows about 0.38 m out). The clip keys that carry explicit stand bones keep their small breathing offsets from the new pose. `T/🧰️QA4/lateral.py` documents and applies it (run once). Elevation, front and plan show the hands at the hips.
- **Face details:** eyes, brows, nose, mouth and ears of the jogger (1:50 and 1:100) are extruded solids on the head bone (profile at the face surface in head-local coordinates), so they follow the head pitch in the hamstring fold.
- **Rounded profiles instead of boxes:** cane shaft and ferrule in plan are one `followTip` capsule from the hand to the ferrule (round ends, tapered, no solid needed); the crook is four stepped extruded slabs (rounded end); the jogger soles are stepped slabs (heel, ball, toe) at 1:50, 1:100 and 1:200 (new `Ctx.slices`). Parcels stay boxes (they are boxes). 1:500 stays one slab per part (min feature).
- **z audit (`T/🧰️QA4/audit_z.py`):** all front and plan details lying on the same part as their body were below a higher filled shape of their region and are raised above it (trouser creases, shorts hems and stripes, cargo pockets, jeans seams at z 3.1; hat-crown ring in plan 6.05). The remaining cuts are real occlusions (arm in front of the jacket zip, fork over the fender, crank over the chainring, buttons over the placket line) and were left. Elevation strokes were reviewed the same way: all remaining cuts are occlusions by another limb or layer, none changed.
- **All sheets re-rendered and read:** every 1:200 and 1:500 sheet (26 contact images, every pose and three instants per clip, all seven assets) was read after the last rebuild; nothing read wrong. The jogger 1:50 stand, hamstring and initial-contact sheets were also read.
- Check: ok for all seven; sweep and consistency: no findings; three-views vitest: 14 passed.

## Remaining doubts
- Extruded plan slabs still have straight edges (stepped, not curved); the cane crook step is the coarsest.
- The jogger face details are bounding boxes of the feature profile in the front view (small rectangles).

## Jogger 1:500 hands and forearms, final 1:500 front review
- **Feature width (corrected measure):** at 1:500 the jogger front forearm and hand polys collapsed to 0.01 to 0.02 m height when a bone turned horizontal (hand-r 0.167, forearm-l 0.182, hand-l 0.174 mm). The hand is now merged into the forearm as one extruded solid on the `forearm` bone (0.116 m wide and thick, regions arm-skin and hand-skin), see `fix_pitching` in `T/🧰️H1/a_jogger.py`. Check ok for all seven assets, no sweep or consistency findings, three-views vitest 14 passed.
- **Look at the sheets:** all 1:500 sheets (66 sheets: every pose and three instants per clip) of the jogger and the other six H1 assets were rendered after the single 3D path landed and every front view (with side and top beside it) was read. Jogger: run poses show the arms as short bars beside the torso, quad-r shows the raised arm as a block at the shoulder, stand shows hands on the hips with elbows out, hamstring shows the folded torso box with arms hanging in front of the legs. Courier: parcel stack and handle stay with the hand truck in every pose and in tilt-and-settle. Cyclist: legs straddle the frame in dismounted-astride, arms and cranks stay attached in all clip instants. Scooter, elderly (cane beside the leg, hand on the crook in rest-on-cane) and leaning person (wall frame, raised phone arm) read correctly. Nothing detached or wrong; no further edits were needed.

## QA9 H1 Round
Owner of the H1 edits: QA9. Builders: `T/🧰️H1/a_gardener.py`, `a_cyclist.py`, `a_elderly.py` (rerun, atomic save in `lib.py`); probes `T/🧰️QA9/probe2.ts`.
- **K-1 kneeling-gardener (fixed):** the front hat brim was an extruded bounding box of the pitched head (x +-0.235, y 0.81 to 1.12 at dig-cycle @0.16), which hid the head. It is now a revolved solid about the `head` bone (half profile from the elevation brim box, mirror), so the brim is the true flattened ellipse of the tilted hat and the head and shoulders read. Sheets read: dig-cycle @0.16, @0.8 at 1:50 and 1:100.
- **C-1 cyclist (real, fixed):** pixel measurement was not needed, evaluation shows it: at pedal-cycle @0.4 the front tyres were 0.21 m tall strips at y 0.235 to 0.445 (rigid capsules foreshortened by the wheel spin) instead of 0 to 0.68; the plan tyres were 0.21 m long instead of 0.68. Both tyres are now extruded solids with the circular wheel profile (spin invariant): front y 0 to 0.68 (on the ground line), plan x 0.71 to 1.39. The dash under the front ground line was the left pedal (front y -0.117 to -0.101 against elevation 0.223 to 0.239); front pedals are now extruded solids of the elevation pedal profile and agree with the side view at every instant (@0.4 and @2 verified numerically, @1.2 dismount read on the sheet).
- **J-3 jogger 1:500 (not clean):** hand-r, forearm-l and hand-l are fixed by QA4's extruded forearm solids, but with the corrected feature measure the check now reports other 1:500 shapes: front foot-r 0.158 mm, plan upper-arm-l 0.154, upper-arm-r 0.152, forearm-r 0.168, foot-l 0.152, shank-r 0.166 (limbs foreshortened to near-circles in some poses). Widening the 1:500 limb clamp from 0.053 to 0.07 m broke the forward-extent consistency (plan 1.102 m against 1.035 m), so it was reverted. The same measure now reports 51 shapes at 1:500, 1:200 and a few at 1:100 and 1:50 across the other H1 assets (scooter, courier, cyclist, elderly, leaning); none is a geometry defect of the three views. The measure of ear-l/r (1:100 elderly) did not move when the ears were widened from 0.018 to 0.024 m, so the measure needs a look by FW/TS before content is changed.
- **E-1 elderly plan 1:100:** cane shaft radius 0.012 to 0.02 m (1:100) and 0.016 to 0.022 m (1:200), crook slabs follow; the 1:100 sheet cannot be rendered while the ear feature finding makes the representation unsuitable, so this is verified by numbers only.
- **CS-1 scooter 1:50 front:** read again at 4x: handlebar, grips, elbows and forearms are separate outlines; accepted unchanged. **J-1 / J-2 jogger fold and raised leg:** read again (hamstring, quad-r at 1:50); head, torso block and bent leg are readable; accepted unchanged.
- Checks: gardener ok; sweep/consistency clean for all seven; three-views vitest 18 passed (seven H1 plus parasol and grill).
