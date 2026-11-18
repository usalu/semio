# Silhouette guard, humans group A (wheelchair-user, seated-office-worker, tour-guide)

Result: wheelchair-user and seated-office-worker 0 failing checks at all scales, `validate-all` lists none of the three. tour-guide 9 to 4 failing: 100:hf(0.0488) 100:hp(0.051) 50:hf(0.0507) 50:hp(0.0338).

## wheelchair-user (10 to 0)
- 1:200 front/plan: drawing lacked back tubes, push handles, lower rail (copied from 1:100); 1:200 spokes thickened (pen 0.2 mm) so the wheel hole matches the hole-fill of the volume.
- 1:100/1:50: plan lacked the back sling (rect +-0.2 m) and back tube; beanie/head/neck/hood/hoods/shoes/hands refitted to the volume outline; thumbs missing in the 1:50 volume (capsules added); hand rim overlapped with the tyre in plan.
## seated-office-worker (10 to 0)
- Volume: desk foot plate 0.05 m wide (bbox depth 0.60), hair loft lowered (bbox height 1.411).
- Drawing: backrest plan +-0.245 m, seat plan reaches the backrest, armrest length, chair base band in front, mechanism/column/armrest post shapes, traced head/shirt/sleeves/shoes/pole/seat shapes at 1:50; the 1:100 and 1:200 plan/front/elevation equivalents; 1:500 elevation greedy trace of shirt, forearm, hand, shank, shoe, laptop.
- Front shoulders/upper arms keep their original solids (validator `lateral-ambiguous`); fill shapes on the torso bone cover the volume-only parts.
## tour-guide (9 to 4)
- Volume: flag pole 18 mm longer (bbox height 2.275), flag as `extrude` of the drawn wave; drawing: flag edge at the pole, crotch shape, cap front outline, front pole and finial, torso profile radii 7 to 17 mm smaller (matches the volume), traced elevation shapes (1:50 elevation passes), 1:100 head.
- Still failing: front and plan at 1:50 and 1:100. Cause: the front/plan limb shapes are revolved/extruded solids on bones that leave the plane (arm, forearm, hand, head); tracing them as flat polygons fails `lateral-ambiguous`, so they were restored. Remaining clusters: neck/hood and shoulder at the front (torso/sleeve profile vs loft, 30 to 50 mm), plan torso depth 34 mm and arm/hand at plan. Next: fit the revolved profile radii per station (as done for the torso) and add fill shapes on the pelvis bone for volume-only areas (front 1:50 30.5 mm with fills).
## Tools (this folder)
humansb-A-diag.ts + humansb-A-d.sh (diag with PATCH and ASCII; FILLED=1 shows boundaries), humansb-A-hullfit.ts + humansb-A-trace.sh (hull/trace refit), humansb-A-snap*.ts, humansb-A-greedy.py, humansb-A-norm.py (ops normaliser), humansb-A-copy/ids/merge/autospec/ext/solids.
