# Authoring report: humans-a (14 assets)

Authored sources: `volumes/humans/<slug>.json` (inputs kept); generator scripts and helpers: `authoring-humans-a/` (`a_<slug>.py`, `lib.py`, `body.py`, `decisions.py`, `inspect.ts`, `validate.ts`, `run.sh`, `table.py`); tool output: `🗑️generated/authoring-humans-a/`. Every volume is spliced into its catalog asset with `volume-insert.py`.

Method: lofts follow the 1:50 elevation outline (depth, centre) with front/plan widths; every loft section carries an explicit `rotation` (rx on w, ry on v, path along u) as the coordinator required; limbs and tubes are capsules, wheels/pot/canopy/hat brim revolves, tyre rings 36 chord capsules repeated about the axle, plates extrudes, parcels/counter boxes; left/right pairs are separate solids (or one solid with `repeat` where rigidly identical); far limbs hidden in the side view (gardener leg, toddler limbs) are mirrored/shifted copies of the near outlines; view-only draft parts were re-authored by hand; fine parts carry `maxDenominator` derived from the ids the 200/500 drawings still contain.

Columns: bbox error per axis (x widthM / y heightM / z depthM) at rest as measured by `volume-check` and in the asset's default pose (`presentation.defaultPose`, measured with the posed solids); IoU elevation/front/plan at 1:50 at rest; "validator volume issues" = `volumeIssues` findings (all 0), "other validateAsset" = findings outside `volume-*`; poses checked = all named poses of `inspect.ts` (evaluated with `solveChain` + `carry`, connectivity of all solids at gap 0.02 m, lateral side flips).

## Table

| slug | solids | bbox err % x/y/z at rest | bbox err % x/y/z in default pose | default pose | IoU e/f/p | validator volume issues | other validateAsset | poses checked | flips | detach |
|---|---|---|---|---|---|---|---|---|---|---|
| child-on-kick-scooter | 59 | -0.2/+4.0/-1.3 | -0.2/-0.2/-1.3 | gliding | 0.98/0.93/0.92 | 0 | 0 | 3 | False | False |
| courier-with-hand-truck | 61 | -1.1/+0.9/+1.7 | -0.9/+0.4/+1.7 | waiting | 0.98/0.95/0.93 | 0 | 0 | 4 | False | False |
| cyclist | 87 | +0.1/+0.2/+0.0 | +0.1/-0.2/+0.0 | riding-upright | 0.94/0.89/0.93 | 0 | 0 | 4 | False | False |
| elderly-with-cane | 55 | -18.6/+8.9/-1.0 | +0.0/-1.1/-1.0 | stooped-standing | 0.98/0.95/0.92 | 0 | 0 | 9 | False | False |
| jogger | 42 | -65.6/+3.4/+3.4 | +0.0/-0.2/+3.3 | initial-contact-l | 0.99/0.93/0.90 | 0 | 0 | 15 | False | False |
| kneeling-gardener | 40 | -0.3/+50.3/+0.0 | -0.3/+0.9/+0.0 | kneeling-weeding | 0.96/0.96/0.98 | 0 | 2 | 6 | False | False |
| leaning-person | 44 | -36.1/+1.3/+0.9 | -1.3/-0.2/+0.9 | leaning-shoulder-wall | 0.99/0.93/0.91 | 0 | 0 | 5 | False | False |
| parent-carrying-toddler | 40 | -0.0/+0.6/+0.0 | +0.0/+0.2/+0.0 | standing-carry | 0.97/0.94/0.91 | 0 | 0 | 4 | False | False |
| parent-with-stroller | 80 | -32.4/+5.2/+0.3 | -0.2/-0.6/+0.3 | pushing-stroller | 0.97/0.93/0.80 | 0 | 0 | 14 | False | False |
| person-cooking | 53 | +0.7/+0.3/+0.0 | -0.1/-0.4/+0.0 | stirring | 0.99/0.91/0.96 | 0 | 0 | 3 | False | False |
| person-on-crutches | 51 | -23.0/+6.6/+0.9 | -0.7/+0.2/+0.9 | tripod-stand | 0.96/0.91/0.85 | 0 | 0 | 3 | False | False |
| person-with-rollator | 84 | -1.6/+2.5/+0.6 | +0.1/+0.1/+0.6 | stand-at-grips | 0.96/0.86/0.87 | 0 | 0 | 4 | False | False |
| person-with-umbrella | 59 | -3.2/-12.3/-1.6 | -3.1/+0.2/-1.6 | umbrella-up-still | 0.78/0.92/0.95 | 0 | 0 | 5 | False | False |
| person-with-white-cane | 57 | -62.1/+33.1/+0.0 | +0.6/-0.2/+0.0 | constant-contact-stance | 0.97/0.91/0.90 | 0 | 0 | 4 | True | False |

## Gates and remarks

All 14 assets have a spliced volume that passes Ajv and `volumeIssues` with 0 findings. IoU gates (0.80) are met for every view of 13 assets; the bounding-box gate is met at rest for 6 assets and, where the rest pose differs from the presentation pose, in the default pose for all 14 (coordinator amendment 6: `dimensions` describe the presentation state).

Misses of the literal gates (measured at rest):
- elderly-with-cane (x -18.6 %, y +8.9 %), jogger (x -65.6 %), kneeling-gardener (y +50.3 %), leaning-person (x -36.1 %), parent-with-stroller (x -32.4 %, y +5.2 %), person-on-crutches (x -23.0 %), person-with-umbrella (y -12.3 %), person-with-white-cane (x -62.1 %, y +33.1 %): the authored rest pose (all bone offsets zero, which is what the 2D shapes show and what elevation IoU 0.96 to 0.99 matches) is not the pose `dimensions` describes (stride, hanging stroller or umbrella, cane up); in the default pose the same solids measure within 3.3 % on every axis (jogger z +3.3 %, cyclist, child etc. below 2 %).
- person-with-umbrella: elevation IoU 0.78 (< 0.80) at rest. The 2D side view draws the canopy only as a thin sliver along each rib (rim 0.57 m, bulge 0.095 m), while the canopy is a rotationally symmetric dome in the front view (radius 0.52 m) and the plan (circle); a dome's side silhouette is the filled cone, so the three 2D views cannot all be matched by one body. The dome (two half revolves, one per rib bone, folds with the ribs when furled) is kept; precision 0.79, recall 0.99.
- parent-with-stroller: plan IoU 0.80 (0.802, marginal): the 2D plan draws no seat; the seat sling is limited to the child width (0.22 m) to keep the gate; the child legs are drawn 0.14 m thick in the elevation but 0.045 m in the front/plan (inconsistent 2D, elevation kept).

2D inconsistencies and things for a reviewer to look at:
- person-cooking: head, neck and hair are drawn in the front/plan views as revolves about an axis 0.15 m right of the body (`axisZ: -0.15`); the volume centres them on the body axis (front IoU 0.91 still passes). The spoon bowl reaches into the pot in `stirring` (distance 0.039 m to the pot wall vertices, inside the hollow pot), so the groups counter+pot and person show as two groups there.
- person-with-white-cane: `diagonal-technique` swings the cane across the body (z extent 0.87 m, left/right flip of hand-r and the cane parts) as authored; intended.
- courier-with-hand-truck: the draft merged the front cross braces and the elevation toe stays by id; authored separately. Rolled tape strips are modelled on the parcel front faces (the 2D tape lines lie inside the parcels).
- Parts hidden in the 2D views are authored with plausible positions: gardener right leg (mirror of the left), toddler far limbs (copies shifted by 0.10 m), stroller canopy (two tapered extrusions).
- Pose connectivity: all solids form one connected group in every named pose except where an object is separate by design (gardener trug and weeds, person-cooking counter and pot in `stirring`) and at rest for objects carried at the hands (courier hand truck, scooter, stroller, rollator, umbrella hang away from the rest-pose hands).
- parent-with-stroller `other validateAsset` is 0; kneeling-gardener reports 2 non-volume findings (`unavailable-but-drawable` for the 1:200 and 1:500 representations), pre-existing and untouched.
- Coordinator note 4 (closed revolve profile) was not needed: tyres and rings use chord capsules; the umbrella canopy profile is an open, simple polyline (closed to the axis by the kernel) with no validator finding.
