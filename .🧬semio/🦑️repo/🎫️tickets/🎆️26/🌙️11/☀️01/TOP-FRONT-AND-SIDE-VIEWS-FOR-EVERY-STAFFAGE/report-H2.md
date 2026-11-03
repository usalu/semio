# Report H2: front and plan views for seven human assets

Assets: person-with-white-cane, person-cooking, person-on-crutches, person-with-rollator, parent-with-stroller, parent-carrying-toddler, person-with-umbrella.

## Method

- Scripts in `T/🧰️H2/`: `lib.py` (rest-pose forward kinematics, shape constructors, default-pose to rest-coordinate conversion `Conv`), `human.py` (shared standing-person front and plan builder, four detail levels), one `a_<asset>.py` per asset (accessories, decisions text), `apply.py` (writes front, plan, extended decisions, removes `presentation.viewpoints`; atomic writes), `all.ts` (one process: consistency sweep, check, previews), `lateral.py` (new rig DOFs).
- Coordinates follow the rest pose with the rig root at the origin (pelvis at y = 0 for the stroller parent, toddler parent and umbrella person; pelvis at hip height for the others). Accessories were designed in default-pose coordinates and converted per bone.
- Detail by scale: 1:500 head ellipse, torso-and-arms silhouette, thigh and shank-with-shoe masses (plan: shoulders ellipse, head, shoe capsules); 1:200 head, torso, arm, forearm-with-hand, thigh, shank-with-shoe (plan adds trunk, arm capsules, foot capsules); 1:100 adds hair, neck, hips, mitten hands, shoes, cuff-end ellipses, thigh and shank capsules; 1:50 adds face lines, ears, placket, trouser creases, cuffs, thumbs, burner rings and similar.
- Limbs use `followTip` poly (front) and `followTip` capsules (plan) on their own bones; hands and heads are rigid.
- All shapes carry the region and material of their elevation counterpart. Unfilled details have a higher z than their body.
- Decisions: every authored scale got a front-and-plan paragraph appended to `silhouette` (en and de).

## Per asset

- **person-with-white-cane**: cane is a strip on the cane bone (front, `followTip`) and a capsule from hand to tip (plan); bag at left hip; sunglasses in front at 1:100 and 1:50. Arm bones got `Bone.z` of -0.15 and 0.15 for the spread pivot. New DOFs: `diagonal-technique` arm-r +25, forearm-r +20, cane +14 abduction; `constant-contact-stance` arm-r +8, forearm-r +10, cane -4 (also in clip keys that use these poses).
- **person-cooking**: counter, hob, pot, spoon on their own bones; the counter hides the cook below the worktop in front; the pot sits at lateral -0.14 where the right hand reaches; spoon is a tapered capsule (plan) and a strip (front). `standing-by`: neck twist -10, head twist -25 (turned to the pot).
- **person-on-crutches**: crutches are slanted strips (front) and capsules (plan) from grip to tip 0.325 m from the centre line on the crutch bones; cuffs, grips, tips at 1:100 and 1:50; bare thighs below shorts, left boot and right sneaker keep their regions.
- **person-with-rollator**: frame, legs, seat, backrest, basket, bag and handlebar in front, wheels as extruded circle solids on the wheel bones (rear wheels hidden behind the front ones); in plan rails, bars, seat, backrest and four wheel solids below the hands; bound to the frame and wheel bones.
- **parent-with-stroller**: after the single 3D path landed (rigid shapes rotate with their bones, and the stroller bone hangs down at rest) every stroller part is an `extruded` solid authored in its bone's local frame, the same solid in front and plan: wheels (circles on the wheel bones, so spin does not change the outline), chassis and push-bar rails, forks, handle bar, hood (profile block), child body, head, legs and hat on the child bone, basket and footrest. Per-view z orders the hood in front of the child in plan and behind it in front.
- **parent-carrying-toddler** (1:100 and 1:50 only; 1:200 and 1:500 stay unavailable): toddler faces forward in front of the parent (suit, face, hat, thighs end-on, shanks, boots, arms); plan capsules for thigh, shank and arms stretch while hoisting.
- **person-with-umbrella**: canopy front and plan use `solid: revolved` about the hub on `rib-front` (profile is the elevation canopy profile, so the front silhouette equals the side view and narrows when furled; plan is the circle of widest reach). Shaft is a strip on the umbrella bone (front) and a capsule (plan); the ferrule tip is bound rigidly to the rib bone because the umbrella bone is flipped between rest and default pose. Canopy and shaft are centred on the body (lateral 0) so front and plan agree. The canopy solid is bound to the rib bone (profile in the rib frame, so furling changes it) and revolves about the `umbrella` shaft bone, the only bone along the symmetry axis (new rest-axis rule); the first version named the rib bone as axis and gave a 0.58 m width. The ferrule tip is authored at rest beyond the shaft end on the umbrella bone.

## Checks (round 7, `T/🗑️generated/H2/round7/all.log`; consistency sweep including the spread guard, then check CLI at every authored scale)

- Print profile office-laser: suitable at every authored scale for all seven assets (minimum features 0.200 to 0.456 mm; minimum stroke 0.100 mm).
- Three-views consistency at default pose and the spread/twist guard: no finding for any asset (cane person incl. diagonal-technique and constant-contact-stance with abduction; cook standing-by with twist; umbrella at all four scales).
- Fix history of this task: detail-level conditions (inverted for 1:200 and 1:500), crutch tube 1:500, umbrella ferrule and centring and rest-axis revolve, thumb overshoot, stroller rails and the move to solids when rigid shapes began to rotate with their bones, 1:200 cane forearm collapsing end-on (now an extruded solid), rollator head height with beret at 1:200 and 1:500, stroller gaps (distinct z per same-bone member).

## Previews inspected

Cane: default pose 1:50, walking-midstance 1:100, diagonal-technique 1:100. Crutches: tripod-stand 1:50, body-swing 1:100. Rollator: stand-at-grips 1:50, seated-rest 1:100. Stroller: pushing 1:50, kerb-tilt 1:100 (round 7, solids). Toddler: standing-carry 1:50. Umbrella: up-still 1:50, furled 1:100 (round 7: narrow in front, small circle in plan). Cooking: standing-by 1:100. All read correctly. The other generated sheets (every pose, two instants of every clip) exist under `T/🗑️generated/H2/round*/` but were not all opened.

## Remaining doubts

- Many generated sheets were produced but not individually viewed; clips at 1:200 and 1:500 were only covered by the sweep and check, not by eye.
- The front arms of raised forearms are foreshortened to a cuff-end ellipse; hand skin is not visible end-on.
- Cane abduction values and the cook's head twist are estimates from the pose descriptions.
- Rear rollator wheels and the stroller basket are not drawn separately where hidden.
- Crutches and toddler results of round 7 are given in the final message; both were clean in rounds 3 and 6.
