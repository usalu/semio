# Silhouette guard for the humans: repairs and remaining checks

Scope: `👤️humans` only (28 assets; 27 failed before). Tools of this pass in this folder: `humans-locate.ts` (drawn shapes and volume solids near a point of a view), `humans-hull.ts` (convex hull of solids in view coordinates, rings of a drawn shape), `humans-try.ts` (in-memory patch trial of the exact checks of the guard, JSON ops), `humans-ops.py` (applies the same ops to the asset file as span edits: formatting of every other byte stays). The guard is unchanged.

## Fixed (1 asset, 0 failing checks at all four scales)

**walking-commuter** (3 checks: 1:100 plan, 1:50 front, 1:50 plan, all Hausdorff):
- Plan, drawn waist: the `waist` ellipse (rx 0.1545 x ry 0.1737 m) was smaller than the coat skirt of the volume (`torso-coat-lo`, 0.40 m across, 0.31 m deep, superellipse exponent 2.8), 4 cm off at the corners. The drawing is wrong: replaced at 1:50 and 1:100 by a polygon of the real skirt outline (convex hull of skirt and pelvis, 24 points).
- Plan, hands: the drawn mitten stopped 28 mm short of and ran straight ahead where the volume hand points 5 degrees towards the body. 1:100 `hand-l/r` `x1` 0.0105 to 0.028 m; 1:50 `y1` 0.1505 to 0.13 m (mirrored on the right).
- Front 1:50, elbow: the volume sleeve (`forearm-l/r-coat-lo`) had an elbow cap with radius 0.060 m on a 0.040 m sleeve, which stuck out 21 mm beyond the drawn sleeve. The volume is wrong: the four cap sections are 0.0405 m wide now (also the bounding box depth is 0.494 m against 0.5 m, within one feature).

## Partly repaired, still failing

**pregnant-person** 1:50 front: the drawing closed the gap between the thighs 80 mm higher than the volume (crotch at 0.725 m, real inseam of a 1.65 m person). The drawing was wrong: a `crotch` shape (extruded box between the thighs on bone `thigh-l`) was added at 1:50, Hausdorff 0.092 m to 0.030 m. At 1:100 the same shape made it worse (0.0375 to 0.068 m), so it is not added there. Remaining: see table.

## Causes found for the open assets (same families as the repaired ones)

- Hands and fingers: the drawn hand is shorter than the volume hand (standing-adult: left hand 0.138 against 0.171 m, right 0.180 against 0.207 m) or points straight where the volume hand angles in.
- Elbow and shoulder caps of lofts wider or higher than the drawn sleeve and torso (pregnant-person shoulders: volume 2 to 3 cm higher than the drawn torso top; the torso polygon is shortened by about 15 mm by `followTip` in the pose).
- Crotch height differs between drawing and volume (pregnant-person, standing-adult front 0.056 m).
- Shoe soles: the volume sole strip is 1 cm lower or longer than the traced shoe (seated-reader), heads/scarves (cowl) 2 cm.
- Pelvis hollows: the drawn sweater outline has a notch where the volume pelvis closes it (pregnant-person elevation 0.059 m).

## Remaining humans (27) and their exact failing checks

Notation: `scale:kind+view(value)`, kind i = IoU, h = Hausdorff in m, b = bounding box; view e = elevation, f = front, p = plan. Measured before this pass (identical afterwards, except pregnant-person).

| Asset | Failing | Checks |
|---|---:|---|
| child-on-kick-scooter | 6 | 500:ie(0.906) 100:he(0.0452) 100:hp(0.0323) 50:he(0.0232) 50:hf(0.0345) 50:hp(0.0319) |
| courier-with-hand-truck | 11 | 500:ie(0.914) 500:ip(0.913) 200:he(0.0739) 100:he(0.0369) 100:hp(0.0675) 50:b(1.1522) 50:b(1.828) 50:b(0.61) 50:he(0.0362) 50:hf(0.0277) 50:hp(0.0675) |
| cyclist | 14 | 500:ie(0.919) 500:if(0.92) 500:ip(0.912) 200:ie(0.901) 200:if(0.907) 200:hf(0.166) 200:ip(0.907) 100:ie(0.919) 100:he(0.0587) 100:hf(0.156) 100:hp(0.0525) 50:he(0.0452) 50:hf(0.1313) 50:hp(0.0394) |
| elderly-with-cane | 9 | 100:b(1.6349) 100:he(0.0439) 100:ip(0.919) 100:hp(0.0302) 50:b(1.6316) 50:he(0.0344) 50:hf(0.0629) 50:ip(0.904) 50:hp(0.0318) |
| jogger | 10 | 500:if(0.912) 500:hf(0.1687) 500:ip(0.91) 200:hf(0.0825) 100:he(0.0375) 100:hf(0.0788) 100:hp(0.0638) 50:he(0.0286) 50:hf(0.0825) 50:hp(0.0752) |
| kneeling-gardener | 7 | 100:he(0.0503) 100:hf(0.0554) 100:hp(0.0655) 50:b(1.1367) 50:he(0.0506) 50:hf(0.0656) 50:hp(0.12) |
| leaning-person | 8 | 200:hf(0.0754) 100:he(0.0335) 100:hf(0.0302) 100:hp(0.0346) 50:he(0.0545) 50:hf(0.0413) 50:ip(0.911) 50:hp(0.0229) |
| parent-carrying-toddler | 13 | 500:if(0.913) 500:hf(0.1677) 500:ip(0.91) 200:if(0.913) 200:hf(0.195) 200:ip(0.907) 200:hp(0.0825) 100:hf(0.2018) 100:hp(0.0802) 50:b(1.7403) 50:he(0.0318) 50:hf(0.1875) 50:hp(0.0808) |
| parent-with-stroller | 16 | 500:ie(0.811) 500:if(0.818) 500:ip(0.869) 200:ie(0.912) 200:he(0.105) 200:if(0.909) 200:hf(0.1546) 100:ie(0.893) 100:he(0.1377) 100:hf(0.1537) 100:hp(0.0638) 50:he(0.0541) 50:if(0.894) 50:hf(0.1157) 50:ip(0.915) 50:hp(0.0675) |
| person-cooking | 10 | 500:ie(0.891) 500:ip(0.918) 200:he(0.075) 200:hf(0.15) 200:hp(0.0839) 100:he(0.0323) 100:hf(0.1162) 50:he(0.0505) 50:hf(0.1275) 50:hp(0.0356) |
| person-on-crutches | 15 | 500:if(0.87) 500:ip(0.878) 200:if(0.913) 200:hf(0.075) 200:ip(0.914) 100:ie(0.913) 100:he(0.0875) 100:if(0.912) 100:hf(0.1427) 100:ip(0.898) 50:b(1.8212) 50:he(0.0773) 50:hf(0.084) 50:ip(0.897) 50:hp(0.0382) |
| person-with-rollator | 16 | 500:ie(0.904) 500:if(0.899) 500:ip(0.872) 200:if(0.844) 200:ip(0.847) 200:hp(0.075) 100:he(0.0647) 100:if(0.898) 100:hf(0.1275) 100:ip(0.916) 100:hp(0.0419) 50:he(0.0447) 50:if(0.861) 50:hf(0.2019) 50:ip(0.877) 50:hp(0.0537) |
| person-with-umbrella | 11 | 500:ie(0.886) 500:if(0.889) 200:he(0.1125) 200:if(0.867) 200:hf(0.1053) 100:he(0.1168) 100:if(0.905) 100:hf(0.1125) 50:he(0.0396) 50:if(0.911) 50:hf(0.1106) |
| person-with-white-cane | 16 | 500:ie(0.756) 500:he(0.2289) 500:if(0.887) 500:ip(0.863) 200:ie(0.896) 200:he(0.0855) 200:if(0.905) 200:hf(0.15) 100:he(0.045) 100:hf(0.0572) 100:hp(0.0525) 50:b(1.0825) 50:he(0.045) 50:hf(0.0703) 50:ip(0.9) 50:hp(0.0319) |
| pregnant-person (after crotch edit: 1:50 front 0.092 to 0.030 m) | 6 | 200:ie(0.919) 100:hf(0.0375) 50:b(0.5081) 50:he(0.0586) 50:hf(0.0919) 50:hp(0.0168) |
| runner-with-blade-prosthesis | 8 | 500:ie(0.913) 200:hf(0.1125) 100:b(1.8139) 100:hf(0.1313) 50:b(1.8114) 50:he(0.0296) 50:hf(0.0526) 50:hp(0.0226) |
| running-child | 6 | 500:ie(0.905) 500:if(0.913) 500:ip(0.892) 50:b(1.1272) 50:he(0.0319) 50:hp(0.0244) |
| seated-guitarist | 9 | 500:ie(0.91) 500:ip(0.92) 100:he(0.0562) 100:hf(0.0309) 100:hp(0.051) 50:he(0.0537) 50:if(0.91) 50:hf(0.1125) 50:hp(0.0545) |
| seated-office-worker | 10 | 500:ie(0.906) 200:he(0.0975) 200:hp(0.0675) 100:hf(0.0412) 100:hp(0.0375) 50:b(1.4289) 50:b(0.61) 50:he(0.0364) 50:hf(0.0394) 50:hp(0.0412) |
| seated-reader | 4 | 500:ie(0.907) 100:he(0.034) 50:he(0.0273) 50:hf(0.0206) |
| skateboarder | 7 | 500:ie(0.906) 100:b(1.1002) 50:b(1.0984) 50:b(1.0376) 50:he(0.0169) 50:hf(0.0168) 50:hp(0.0244) |
| sleeping-person | 7 | 100:b(1.877) 100:hf(0.0505) 100:hp(0.0521) 50:b(1.8786) 50:he(0.0415) 50:hf(0.0505) 50:hp(0.0506) |
| standing-adult | 7 | 500:ie(0.911) 500:ip(0.909) 200:ip(0.914) 100:he(0.0346) 50:he(0.0604) 50:hf(0.0562) 50:hp(0.0286) |
| standing-phone-user | 7 | 500:ie(0.916) 500:ip(0.895) 100:he(0.0335) 100:hf(0.0562) 50:he(0.0314) 50:hf(0.0562) 50:hp(0.0206) |
| tour-guide | 9 | 100:he(0.0318) 100:if(0.916) 100:hf(0.0638) 100:hp(0.0525) 50:b(2.2562) 50:he(0.0361) 50:if(0.92) 50:hf(0.06) 50:hp(0.0338) |
| wheelchair-user | 10 | 200:he(0.0839) 200:if(0.889) 200:hf(0.1875) 200:ip(0.907) 200:hp(0.12) 100:he(0.0399) 100:hp(0.0488) 50:he(0.0454) 50:hf(0.0226) 50:hp(0.0469) |
| worker-carrying-box | 8 | 500:ie(0.912) 100:b(1.8277) 100:he(0.0318) 100:hf(0.06) 50:b(1.8221) 50:he(0.0478) 50:hf(0.06) 50:hp(0.0235) |

## Verification after the edits

- `validate-all.ts`: 1 failing of 152, `vehicles/electric-kick-scooter` (shape `fork-crown` lacks `tone`; another agent's asset, no humans asset fails). `validate-manifest.ts`: 0 issues.
- Vitest `schema-conformance`, `asset-power`, `three-views`, `fill-rule`, `plan-projection`, `section-views`: 445 of 446 passed; the one failure is the same `electric-kick-scooter` schema error.
- Files changed: `🗂️catalog/👤️humans/walking-commuter/🔣️.json` (fixed), `🗂️catalog/👤️humans/pregnant-person/🔣️.json` (1:50 front `crotch` shape added).
- The remaining 27 assets need the same hand-by-hand repair (about 15 to 30 minutes each at 1:50, then the 1:500 and 1:200 legibility thickening): they have 3 to 16 failing checks each, 250 in total.
