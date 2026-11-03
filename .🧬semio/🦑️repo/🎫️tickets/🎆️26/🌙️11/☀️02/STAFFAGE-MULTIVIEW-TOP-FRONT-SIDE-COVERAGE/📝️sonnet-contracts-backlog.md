# Staffage multiview backlog (owner: sonnet-contracts)

Generated 2026-11-02 by `T/🛠️scripts/sonnet-contracts/failures.ts` and `backlog.ts` from the current catalog. Machine-readable: `T/🗑️generated/contracts/failures.json`. 149 assets checked, 10 failing.

## Totals per code

| code | issues |
|---|---|
| lateral-ambiguous | 27 |
| manifest-mismatch | 16 |
| unavailable-but-drawable | 4 |
| extent-plan-depth | 3 |
| manifest-dimension-mismatch | 2 |
| extent-front-height | 1 |

Rules and their meaning: `T/🔗️interfaces.md` sections 4 and 6. Tolerance of every extent rule: |measured - declared| <= max(2 % of declared, 15 mm). `lateral-ambiguous` is one issue per bone and lists the shapes, the bands (`1:<scale> <view>`), `thetaMax` (the largest out-of-plane rotation of that bone over every pose, every clip and 9 clip samples) and the worst paper error `extent * sin(thetaMax)` in mm, flagged when it exceeds the 0.2 mm print minimum feature.

## About `elevation-lateral` (answer to the inventory question)

The predicate reads (1) the abduction and twist offsets (`abductionDeg`, `twistDeg`) in every pose and every clip key, taking the bones that carry one and all their descendants, and (2) the lateral width of the shape's own front twin (same id and bone). It does not read rest-pose bone z, and a crown silhouette on a bone that never leaves the elevation plane (no ancestor with abduction or twist anywhere) cannot trigger it. So it fired on the 11 trees only because a crosswind sway key rotated those bones out of plane; D29 applies (vegetation reverted the crosswind) and the rule no longer fires in the current catalog.

## animals

### other findings

| asset | code | path | message |
|---|---|---|---|
| labrador-dog | extent-front-height | representations.500.geometry.front | The default-pose height 0.702 m of the 1:500 front elevation differs from the real dimension 0.72 m by more than max(2 % of the declared value, 15 mm). |

## humans

### lateral-ambiguous

| asset | bone | shapes | bands | thetaMax deg | error mm |
|---|---|---|---|---|---|
| jogger | upper-arm-l | upper-arm-l, shoulders, sleeve-l, armband, sleeve-hem-l, sleeve-l-shoulder, armband-shoulder, shoulder-seam-l, +1 | 1:500 elevation, 1:500 front, 1:500 plan, 1:200 elevation, 1:200 front, 1:200 plan, 1:100 elevation, 1:100 front, 1:100 plan, 1:50 elevation, 1:50 front, 1:50 plan | 88.25 | 8.871 |
| jogger | forearm-l | forearm-l | 1:500 elevation, 1:500 plan, 1:200 elevation, 1:200 front, 1:200 plan, 1:100 elevation, 1:100 front, 1:100 plan, 1:50 elevation, 1:50 front, 1:50 plan | 102.53 | 6.037 |
| jogger | upper-arm-r | upper-arm-r, sleeve-r, sleeve-hem-r, sleeve-r-shoulder | 1:500 elevation, 1:500 front, 1:200 elevation, 1:200 front, 1:200 plan, 1:100 elevation, 1:100 front, 1:100 plan, 1:50 elevation, 1:50 front, 1:50 plan | 55.3 | 6.669 |
| jogger | forearm-r | forearm-r | 1:500 elevation, 1:500 plan, 1:200 elevation, 1:200 front, 1:200 plan, 1:100 elevation, 1:100 front, 1:100 plan, 1:50 elevation, 1:50 front, 1:50 plan | 96.11 | 6.149 |
| jogger | hand-l | hand-l, thumb-l, fingers-l-0, fingers-l-1, fingers-l-2 | 1:500 elevation, 1:500 front, 1:500 plan, 1:200 elevation, 1:200 front, 1:200 plan, 1:100 elevation, 1:100 front, 1:100 plan, 1:50 elevation, 1:50 front, 1:50 plan | 106.89 | 4.136 |
| jogger | hand-r | hand-r, thumb-r, fingers-r-0, fingers-r-1, fingers-r-2 | 1:500 elevation, 1:500 front, 1:500 plan, 1:200 elevation, 1:200 front, 1:200 plan, 1:100 elevation, 1:100 front, 1:100 plan, 1:50 elevation, 1:50 front, 1:50 plan | 101.11 | 4.241 |
| pregnant-person | arm-l | sil-arm-l, upper-arm-l, sleeve-l, shoulder-l | 1:500 elevation, 1:500 plan, 1:200 elevation, 1:200 front, 1:200 plan, 1:100 elevation, 1:100 front, 1:100 plan, 1:50 elevation, 1:50 front, 1:50 plan | 51.17 | 2.421 |
| pregnant-person | forearm-l | sil-forearm-l, forearm-l, elbow-l, cuff-l, cuff-rib-l | 1:500 elevation, 1:500 plan, 1:200 elevation, 1:200 front, 1:200 plan, 1:100 elevation, 1:100 front, 1:100 plan, 1:50 elevation, 1:50 front, 1:50 plan | 107.43 | 3.92 |
| pregnant-person | arm-r | sil-arm-r, upper-arm-r, sleeve-r, shoulder-r | 1:500 elevation, 1:500 plan, 1:200 elevation, 1:200 front, 1:200 plan, 1:100 elevation, 1:100 front, 1:100 plan, 1:50 elevation, 1:50 front, 1:50 plan | 59.79 | 2.866 |
| pregnant-person | forearm-r | sil-forearm-r, forearm-r, elbow-r, cuff-r, cuff-rib-r | 1:500 elevation, 1:500 plan, 1:200 elevation, 1:200 front, 1:200 plan, 1:100 elevation, 1:100 front, 1:100 plan, 1:50 elevation, 1:50 front, 1:50 plan | 90.57 | 4.711 |
| pregnant-person | hand-l | hand-l, fist-l, thumb-l, finger-l-0, finger-l-1, finger-l-2 | 1:500 elevation, 1:500 front, 1:500 plan, 1:200 elevation, 1:200 front, 1:200 plan, 1:100 elevation, 1:100 front, 1:100 plan, 1:50 elevation, 1:50 front, 1:50 plan | 63.73 | 2.506 |
| pregnant-person | hand-r | hand-r, fist-r, thumb-r, finger-r-0, finger-r-1, finger-r-2 | 1:500 elevation, 1:500 front, 1:500 plan, 1:200 elevation, 1:200 front, 1:200 plan, 1:100 elevation, 1:100 front, 1:100 plan, 1:50 elevation, 1:50 front, 1:50 plan | 48.12 | 3.016 |
| sleeping-person | foot-l | sil-foot-l, foot-l | 1:500 front, 1:500 plan, 1:200 front, 1:200 plan, 1:100 front, 1:100 plan, 1:50 front, 1:50 plan | 66.96 | 4.347 |
| sleeping-person | foot-r | sil-foot-r, foot-r | 1:500 front, 1:500 plan, 1:200 front, 1:200 plan, 1:100 front, 1:100 plan, 1:50 front, 1:50 plan | 66.96 | 4.347 |
| tour-guide | torso | figure, sil-body, torso, badge, jacket-zip, lanyard, jacket-hem, torso-waistfold-0, +3 | 1:500 elevation, 1:500 front, 1:200 front, 1:100 elevation, 1:100 front, 1:50 elevation, 1:50 front, 1:50 plan | 20.38 | 4.043 |
| tour-guide | upper-arm-l | upper-arm-l, sil-arm-l, sleeve-l, upper-arm-l-shoulder | 1:500 elevation, 1:500 plan, 1:200 elevation, 1:200 front, 1:200 plan, 1:100 elevation, 1:100 front, 1:100 plan, 1:50 elevation, 1:50 front, 1:50 plan | 70.12 | 6.781 |
| tour-guide | forearm-l | forearm-l, cuff-l, forearm-l-elbow-0, forearm-l-elbow-1 | 1:500 elevation, 1:200 elevation, 1:200 front, 1:100 elevation, 1:100 front, 1:50 elevation, 1:50 front, 1:50 plan | 171.12 | 2.194 |
| tour-guide | upper-arm-r | upper-arm-r, sleeve-r, upper-arm-r-shoulder | 1:500 elevation, 1:200 elevation, 1:200 front, 1:200 plan, 1:100 elevation, 1:100 front, 1:100 plan, 1:50 elevation, 1:50 front, 1:50 plan | 166 | 5.276 |
| tour-guide | forearm-r | forearm-r, wrist-skin-r, cuff-r, forearm-r-elbow-0, forearm-r-elbow-1 | 1:500 elevation, 1:200 elevation, 1:200 front, 1:200 plan, 1:100 elevation, 1:100 front, 1:100 plan, 1:50 elevation, 1:50 front, 1:50 plan | 154.86 | 5.119 |
| tour-guide | flag-pole | pole, pole-finial | 1:500 elevation, 1:200 elevation, 1:100 elevation, 1:50 elevation | 20 | 9.358 |
| tour-guide | flag | flag, flag-emblem, flag-stripe | 1:500 elevation, 1:200 elevation, 1:100 elevation, 1:50 elevation | 20 | 3.204 |
| tour-guide | pointer | pointer, pointer-tip | 1:500 elevation, 1:500 front, 1:200 elevation, 1:200 front, 1:100 elevation, 1:100 front, 1:50 elevation, 1:50 front, 1:50 plan | 158.84 | 8.774 |
| tour-guide | head | sil-head, head, hair, neck, cap, shoulders, cap-seam, ear-l, +20 | 1:500 front, 1:200 elevation, 1:200 front, 1:200 plan, 1:100 elevation, 1:100 front, 1:100 plan, 1:50 elevation, 1:50 front, 1:50 plan | 45.94 | 3.978 |
| tour-guide | hand-l | hand-l, pole-grip, thumb-l, fingers-l-0, fingers-l-1, fingers-l-2, finger-l-0, finger-l-1, +1 | 1:200 elevation, 1:200 front, 1:200 plan, 1:100 elevation, 1:100 front, 1:100 plan, 1:50 elevation, 1:50 front, 1:50 plan | 129 | 4.801 |
| tour-guide | hand-r | hand-r, thumb-r, fingers-r-0, fingers-r-1, fingers-r-2, finger-r-0, finger-r-1, finger-r-2 | 1:200 elevation, 1:200 front, 1:100 elevation, 1:100 front, 1:100 plan, 1:50 elevation, 1:50 front, 1:50 plan | 158.84 | 3.25 |

### other findings

| asset | code | path | message |
|---|---|---|---|
| kneeling-gardener | unavailable-but-drawable | representations.500.status | The asset measures 3.7 mm on paper at 1:500, at least 3 mm: draw it, or offer a true-extent symbol that has all three views and passes the extent and print-minimum checks. |
| kneeling-gardener | unavailable-but-drawable | representations.200.status | The asset measures 9.2 mm on paper at 1:200, at least 3 mm: draw it, or offer a true-extent symbol that has all three views and passes the extent and print-minimum checks. |
| parent-carrying-toddler | unavailable-but-drawable | representations.500.status | The asset measures 3.5 mm on paper at 1:500, at least 3 mm: draw it, or offer a true-extent symbol that has all three views and passes the extent and print-minimum checks. |
| parent-carrying-toddler | unavailable-but-drawable | representations.200.status | The asset measures 8.7 mm on paper at 1:200, at least 3 mm: draw it, or offer a true-extent symbol that has all three views and passes the extent and print-minimum checks. |
| seated-guitarist | extent-plan-depth | representations.200.geometry.plan | The default-pose lateral depth 0.518 m of the 1:200 plan (top view) differs from the real dimension 0.55 m by more than max(2 % of the declared value, 15 mm). |
| seated-guitarist | extent-plan-depth | representations.100.geometry.plan | The default-pose lateral depth 0.518 m of the 1:100 plan (top view) differs from the real dimension 0.55 m by more than max(2 % of the declared value, 15 mm). |
| seated-guitarist | extent-plan-depth | representations.50.geometry.plan | The default-pose lateral depth 0.518 m of the 1:50 plan (top view) differs from the real dimension 0.55 m by more than max(2 % of the declared value, 15 mm). |

## machinery

### lateral-ambiguous

| asset | bone | shapes | bands | thetaMax deg | error mm |
|---|---|---|---|---|---|
| tower-crane | skip | sling-l, sling-r | 1:500 front, 1:200 front, 1:100 front, 1:50 front | 60 | 17.321 |
| tower-crane | slewing | tower-head-leg-l, tower-head-leg-r, tower-head-diagonals | 1:200 front, 1:100 front, 1:50 front | 60 | 128.172 |

## manifest

### other findings

| asset | code | path | message |
|---|---|---|---|
|  | manifest-mismatch | entries[14].symbols | Entry "english-oak" and its asset disagree on symbols. |
|  | manifest-mismatch | entries[15].symbols | Entry "lombardy-poplar" and its asset disagree on symbols. |
|  | manifest-mismatch | entries[16].symbols | Entry "multi-stem-birch" and its asset disagree on symbols. |
|  | manifest-mismatch | entries[17].symbols | Entry "scots-pine" and its asset disagree on symbols. |
|  | manifest-mismatch | entries[18].symbols | Entry "date-palm" and its asset disagree on symbols. |
|  | manifest-dimension-mismatch | entries[18].widthM | Entry "date-palm" declares widthM 6.87 m but the asset file says 6.3 m; the asset file is the single source of truth (tolerance 1 mm). |
|  | manifest-mismatch | entries[20].symbols | Entry "hornbeam-hedge" and its asset disagree on symbols. |
|  | manifest-mismatch | entries[21].symbols | Entry "miscanthus-grass" and its asset disagree on symbols. |
|  | manifest-mismatch | entries[61].representations | Entry "skateboarder" and its asset disagree on representations. |
|  | manifest-dimension-mismatch | entries[61].widthM | Entry "skateboarder" declares widthM 1.218 m but the asset file says 1.084 m; the asset file is the single source of truth (tolerance 1 mm). |
|  | manifest-mismatch | entries[73].symbols | Entry "agave" and its asset disagree on symbols. |
|  | manifest-mismatch | entries[74].symbols | Entry "apple-tree" and its asset disagree on symbols. |
|  | manifest-mismatch | entries[75].symbols | Entry "bamboo-clump" and its asset disagree on symbols. |
|  | manifest-mismatch | entries[76].symbols | Entry "hosta-clump" and its asset disagree on symbols. |
|  | manifest-mismatch | entries[77].symbols | Entry "ivy-on-trellis" and its asset disagree on symbols. |
|  | manifest-mismatch | entries[78].symbols | Entry "japanese-maple" and its asset disagree on symbols. |
|  | manifest-mismatch | entries[79].symbols | Entry "london-plane-pollarded" and its asset disagree on symbols. |
|  | manifest-mismatch | entries[80].symbols | Entry "small-leaved-lime" and its asset disagree on symbols. |

## Range and nominal suitability (D10, D30)

Not yet complete: the `--ranges` run is in progress (it measures every band at its nominal scale and at both ends of its declared `validRange`); this section is filled when it finishes.