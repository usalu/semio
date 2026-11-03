# Report H3: Humans (pregnant-person, runner-with-blade-prosthesis, running-child, seated-guitarist, seated-office-worker, seated-reader, skateboarder)

## State
- Front and plan at every authored scale (skateboarder 1:50 and 1:100 only). `presentation.viewpoints` removed; `decisions.silhouette` extended in en and de per scale.
- Builders in `T/🧰️H3/` (`humanlib.py`, `human.py`, `p_<asset>.py`, `abduct.py`, `dims.py`, `verify.ts`, `shapes.ts`, `preview.ts`); atomic writes. Tone, region and hatch copied from the elevation twin.

## Method
- Rest frame, centred on bones. Front: torso, hips and head rigid on their bones; thighs, shanks, arms and hands are extruded solids (tapered bone outline swept across the limb thickness), plus joint balls. Plan: `followTip` capsules for limbs and trunk, with waist and shoulder ellipses.
- Scales: 1:500 silhouette (limb radius ≥ 0.08–0.09 m), 1:200 masses and joints, 1:100 articulated, 1:50 facial and prop detail. Details sit above bodies in z.
- Lateral spread: runner `hands-on-hips` (upper arms ±42, forearms ∓36), skateboarder `cruising` ±30 and `ollie-air` ±45; arm bones carry `Bone.z` ±0.11.
- Dimension changes: skateboarder `depthM` 0.30 → 1.03 and `widthM` 1.218 → 1.057 (arms out); accepted by the coordinator. Office-worker plan desk 1.4 → 0.6 m to match `depthM`.

## Verification (run-v21)
- Validator clean; default-pose consistency clean at every scale; all poses and 9 instants per clip non-empty and finite in all three views.
- `office-laser`: 1:50 to 1:200 suitable everywhere. 1:500 fails by 0.01–0.05 mm for runner (blade-toe, fist-l), running-child (upper-arm-r, thigh-r, shoe-r), seated-guitarist (fist-r), seated-reader (hips, fist-r, hand-l).
- Previews: seated-reader 1:100 all poses and guitarist strumming and fretting-high read correctly. Others numerically only; previews predate the last fixes.

## Open (Dispatched)
- The 1:500 print failures, after FW3's visible-geometry measure lands.
- Authored `trim` values that shorten 1:500 plan shapes (pregnant-person, running-child `head_trim`, `arm_trim`) are workarounds: overruled.
- Arms only move sagittally apart from the two abducted poses; crossed or folded arms need spread or twist.
