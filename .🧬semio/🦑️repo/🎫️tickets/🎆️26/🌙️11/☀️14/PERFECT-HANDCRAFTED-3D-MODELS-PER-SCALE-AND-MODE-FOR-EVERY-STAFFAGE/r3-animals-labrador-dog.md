# Round 3 — `animals/labrador-dog` (2026-11-19)

State: finished and spliced. `r3-validate.ts animals/labrador-dog --catalog-only --volumes`: VALID (schema 0, validateAsset 0, checkVolumes 0). `volumes-check.ts animals/labrador-dog --product`: PASS at 1:50, 1:100, 1:200 and 1:500. Print suitability of the four representations over all named poses: suitable (`r3-labrador-dog-suit.ts`). Grade by my own eye: body A- at 1:50, B+ at 1:100 and 1:200, B at 1:500; structure A- / B+ / B / C+ (rule-bound, section 7); thermal A- at every scale; 2D side elevation A-, front and plan B+.

No earlier round-3 work existed for this asset; everything was started from the catalog state of HEAD (kept as `🗑️generated/r3-labrador-dog/asset-before.json`).

Scripts (ticket folder): `r3-labrador-dog-rig.py` (rig, anchors, poses, dimensions, materials, references; reads `asset-before.json`), `r3-labrador-dog-kit.ts` (bench: stations in world metres written into bone frames), `r3-labrador-dog-50.ts`, `r3-labrador-dog-50-lists.ts`, `r3-labrador-dog-coarse.ts` (hand tables of every solid), `r3-labrador-dog-decisions.ts`, `r3-labrador-dog-draw2d.ts` + `r3-labrador-dog-rep-decisions.json` (2D redraw), `r3-labrador-dog-candidate.py`, `r3-labrador-dog-apply.py`, `r3-labrador-dog-run.sh` (whole pipeline; `apply` writes the catalog only when the candidate validates), `r3-labrador-dog-work.ts`, `r3-labrador-dog-sheet.ts` (`r3-sheet.ts` on the working asset, used before the first catalog write), `r3-labrador-dog-suit.ts` (print suitability), `r3-labrador-dog-frames.ts`. Kept input: `🗑️generated/r3-labrador-dog/asset-before.json`, `volumes-before.json`, `roots.json`, `reps.json`.

## 1. Reference (adult Labrador Retriever, dog, 30 kg, standing alert)

Sources: FCI Standard No. 122 (height at the withers 56-57 cm for dogs, broad skull, defined stop, deep wide chest with barrel ribs, otter tail of medium length); AKC standard (point of shoulder to point of rump equal to or slightly longer than withers to ground; elbow to ground half the height at the withers; tail no longer than to the hock); Evans and de Lahunta, "Miller's Anatomy of the Dog" (skeleton, joints, landmarks; my recollection of bone proportions of large breeds, no table was looked up); Jaegger et al. 2002 (already in the asset, ranges of motion); the asset's own summary (0.57 m, 30 kg). Girth and overall length are from my knowledge of harness and size charts (chest girth 0.80-0.90 m, nose to rump about 0.92-0.99 m at this height), not from a cited table. No web source was consulted. The three first sources were added to `provenance.references` (`fci-122`, `akc-labrador-standard`, `evans-2013`).

| landmark / size | reference | model (x ahead of the hip joint, y up, m) |
|---|---|---|
| withers height | 0.57 (asset), FCI 0.56-0.57 | 0.568 (skin over T3 at x 0.41) |
| point of shoulder to point of rump | 1.0-1.1 x withers = 0.57-0.63 | 0.63 (x 0.526 to -0.104) |
| elbow height | half the withers height = 0.285 | 0.284 |
| chest depth / width | to the elbow, 0.28-0.29 / 0.24-0.27, girth 0.80-0.90 | 0.284 (0.568 to 0.284) / 0.260 at the ribs, 0.275 over the upper arms |
| nose to rump / with tail | 0.92-0.99 / 1.25-1.35 | 0.96 / 1.29 |
| top of head, alert | 0.70-0.74 | 0.707 |
| hip joint height | 0.47-0.50 | 0.485 |
| femur / stifle height / stifle angle | 0.19-0.21 / 0.28-0.31 / 120-140 degrees | 0.198 / 0.291 / 121 degrees |
| tibia / hock joint height / point of hock | 0.20-0.22 / 0.15-0.16 / 0.18-0.19 | 0.202 / 0.155 / 0.184 |
| tarsus with metatarsals / paw joint height | 0.12-0.13 / 0.03-0.035 | 0.121 / 0.034 |
| scapula, inclination to the horizontal | 0.155-0.17, 50-60 degrees | 0.158, 55 degrees; dorsal border 0.035 m from the midline |
| shoulder joint height | 0.41-0.43 | 0.423 |
| humerus / radius / carpus height / metacarpus | 0.175-0.19 / 0.18-0.20 / 0.10-0.115 / 0.07-0.085 | 0.176 / 0.179 / 0.105 / 0.0745 |
| lumbar / thoracic / cervical column | 0.19-0.21 / 0.23-0.27 / 0.21-0.24 | 0.190 / 0.231 (chord) / 0.245 |
| head, occiput to nose / skull width / muzzle depth with jaw | 0.23-0.25 / 0.13-0.14 / 0.085-0.095 | 0.242 / 0.138 / 0.095 |
| ear leather | 0.11-0.13 long, 0.07-0.09 wide, hanging | 0.117 x 0.074 x 0.008 |
| tail | 0.33-0.38, base 0.06-0.07 thick, to the hock at most | 0.36, base 0.062 |
| paw | fore 0.085-0.095 x 0.07-0.08, hind slightly smaller, four arched toes | fore 0.095 x 0.075, hind 0.086 x 0.068 |
| share of weight on the forelegs | about 60 % | 64 % (2 x 93.7 N of 294 N in the product's forces view) |

## 2. Audit of the state before this round

- Rig: the spine was one line through the middle of the barrel (lumbar joint at y 0.483, thorax 0.441; the vertebral axis is at 0.50) that jumped 0.10 m up to a neck base at 0.543; hip joint 0.513 (0.485); femur vertical (stifle 0.01 m ahead of the hip, real about 0.04) and hock only 0.055 m behind the hip; femur 0.185 and tibia 0.175 (0.198, 0.202), an almost straight hind leg (hip to hock 0.352 of 0.360 reach); forearm 0.200 with a metacarpus of 0.060 (0.179, 0.0745); jaw 0.12 (0.186); pelvis bone 0.14 long along the back; all limb joints on the midline (no lateral offsets); a helper bone `head-core` without anatomical meaning; forepaws 0.55 m ahead of the hind paws, trunk from hip to neck base 0.55 m (0.47).
- `dimensions`: 0.72 x 1.44 x 0.28 m. Height and breadth are plausible; the length of 1.44 m is 0.15 m more than a dog of 0.57 m with a level tail can be (reference: 1.25-1.35).
- 2D: every view drawn on that skeleton: tube limbs in five sausages, a trunk of two blobs, front and plan constructed from ellipses, capsules and revolved profiles around the mid-barrel spine; leg tones asymmetric in the symmetric views; 1358 points in the 1:50 elevation.
- Solids before: lofts fitted to those drawings (segmented barrel, cone neck, stick legs, block paws), plume disc and breath discs, structure of capsules with joint balls along the wrong bones, `head-core` as a member.

## 3. Root fixes in the asset file

- `rig.bones` (old to new length, m): pelvis 0.140 to 0.060, lumbar 0.170 to 0.190, thorax 0.240 to 0.231, neck 0.200 to 0.245, head 0.230 to 0.240, jaw 0.120 to 0.186, ears 0.110 to 0.105, tail 0.150 / 0.140 / 0.120 to 0.130 / 0.120 / 0.110, thigh 0.185 to 0.198, shank 0.175 to 0.202, metatarsus 0.135 to 0.121, hind paw 0.073 to 0.075, scapula 0.160 to 0.158, humerus 0.170 to 0.176, forearm 0.200 to 0.179, metacarpus 0.060 to 0.0745, forepaw 0.070 to 0.073. All origins and rest angles follow from the world joint table in `r3-labrador-dog-rig.py`. The legs now rest in the standing position. Lateral offsets: thigh ±0.055, stifle a further ±0.025, hock ∓0.012, scapula ±0.035, shoulder joint a further ±0.04, elbow ±0.005, carpus ∓0.005, ears ±0.06. `head-core` removed. Joint limits of the leg bones restated around the new rest (thigh -50..75, shank -95..45, metatarsus -45..110, humerus -60..60, forearm -45..110, metacarpus -70..110, paws -60..60). Masses unchanged; centres of mass scaled with the bones, those of pelvis, lumbar and thorax placed in the trunk masses.
- `anchors`: pad contacts under the pads of the new paws; `nose` (0.245, 0.012) to (0.241, 0.004), `collar` (0.1, -0.075) to (0.11, -0.085), `contact-rump` to the back of the thigh under the seat bone, `back-plume` from 0.06 m above the back onto the back line.
- `poses` (19): spine, head, jaw, ear and tail offsets kept; every leg keeps its pad contact point, paw inclination and pastern inclination (shifted by the constant difference between old and new standing dog), thigh and shank, upper arm and forearm are solved anew. No planted pad moves (deviation below 0.5 mm) in any pose; in `sitting-waiting` the scapula turns 6.5 degrees further so the foreleg reaches its pad. Clips reference the poses and were not touched.
- `dimensions.widthM` 1.44 to 1.29 (section 2; the model measures 1.288). Height and breadth unchanged (model 0.707 and 0.275, inside the tolerances).
- `materials`: `bone` added. `provenance.references`: three sources added.
- `representations`: every filled shape of the four scales and three views redrawn as the orthographic outline of the solids it stands for (E1), in the frame the product places it in, simplified to a fifth of the smallest feature: 622 / 549 / 532 points at 1:50 (elevation / front / plan; before 1358 in the elevation), 384 / 348 / 345 at 1:100, 179 / 178 / 191 at 1:200, 112 / 107 / 120 at 1:500. Shape ids are the same in all three views of a scale now (the front and plan shapes had other ids than their elevation twins). New shapes: `croup`, pad shapes in one mark per paw, the far eye in front and plan. Removed: `shoulder-line`, `hip-line`, `elbow-line` (the outlines of shoulder, thigh and upper arm carry them now) and the ellipse, capsule and revolved-solid shapes of front and plan. Restated by hand: brow, mouth line, one toe line per near paw, four fur strokes (1:50), the eye (1:100). Leg shapes keep `followTip` in front and plan. `decisions` of all four representations rewritten.
- Not changed: every `areaM2`, masses, thermal and structural data, clips, `heightM`, `depthM`.

## 4. Models

| scale | body | thermal | structure |
|---|---|---|---|
| 1:50 | 73: chest, hindbody, croup, neck, head, jaw, tongue, nose leather, eyes, ears, tail in three; per hind leg thigh, second thigh with tendon, point of hock, rear pastern, foot, four toes, pad and four digital pads; per foreleg shoulder, upper arm, forearm, pastern, carpal pad, foot, four toes, pad and four digital pads | 77: one reference per body solid, two nostril patches, lip patch, plume strip | 108: skull, mandible, cervical, thoracic and lumbar column with processes, nine rib hoops, sternum, sacrum, hip bones, three caudal runs, ear cartilages, scapulae, humeri, radius with olecranon, metacarpals, femora with trochanter and patella, tibiae, calcanei, metatarsals, two phalanx bars per digit, 28 hinge pins, three masses |
| 1:100 | 33: chest, hindbody with croup, neck, head, jaw, nose, ears, tail in three; per leg its segments and one paw mass with one pad | 37 | 59: skull, mandible, columns, five rib hoops, sternum, pelvic girdle, scapula plates, tapered bars, 21 pins, three masses |
| 1:200 | 20: chest, hindbody, neck, head, jaw, ears, tail; per hind leg thigh, second thigh, lower piece; per foreleg shoulder with arm, forearm, lower piece | 29 (nose, tongue, four pad slabs as thermal solids) | 34: skull, bars, three rib hoops, pelvic girdle, two masses |
| 1:500 | 17: chest, hindbody, neck, head, tail; per leg upper piece, leg piece, foot | 28 (ears, nose, tongue, pads as thermal solids, muzzle caps as outlets) | 32: skull, one rod per bone, one rib girth, two masses |

Final check lines: `PASS` at 1:50, 1:100, 1:200, 1:500. Silhouette IoU elevation / front / plan: 0.993 / 0.992 / 0.995 at 1:50, 0.986 / 0.981 / 0.989 at 1:100, 0.975 / 0.966 / 0.977 at 1:200, 0.933 / 0.933 / 0.951 at 1:500. Thermal areas at 1:50: nose 0.0031 of 0.004 m², tongue 0.0116 of 0.012, pads 0.0235 of 0.02 (all inside 30 %). Structure outside the body: 0 mm at 1:50 and 1:100, 2.7 mm at 1:200, 20 mm at 1:500.

## 5. What I saw (images opened with the Read tool)

- Before: the six-view body sheet and the 2D sheet of the old state (a hound of sausages with a tube tail and block paws).
- 1:50 body, six views and full-size right side (several rounds), front, three-quarter from the front left and from the front right: reads as a Labrador from every side: level back with a slight rise at the withers, deep chest to the elbow, moderate tuck-up, broad thigh, well-bent stifle, low hock with tendon and point of hock, straight forelegs with elbow and carpal pad, strong neck, broad skull with stop, square muzzle, hanging ears beside the cheeks, otter tail level. Front: forelegs straight under the shoulders, chest between them, four toes per paw. Top: barrel widest behind the shoulders, slight waist, thighs and shoulders standing out. Corrected after looking: tail set above the croup, a notch at the stifle, a spur-like point of hock, ears above the skull line, a shoulder blade that stood off the ribcage like a fin (now a muscle mass on the chest), toes like fingers (now only the front third of each toe is free).
- 1:50 thermal, six views: every region coloured over the whole body, nostril patches in the nose, pads under the paws, nothing floats. The cutaway sheet was rendered and seen small in the `volumes-render.ts` sheet (single fur layer, no nested shells to inspect).
- 1:50 structure, six views and full-size right side: a dog skeleton with the cervical column entering the chest low, tall spines at the withers, tilted rib hoops, sternum with rising manubrium, lumbar column, pelvis from iliac wing to seat bone, patella, calcaneus, olecranon, four digits per paw; pins small at the joints.
- 1:100 and 1:200 body sheets, 1:100 structure sheet: the same animal with merged masses; skeleton with five rib hoops and bars. The 1:200 structure sheet was rendered, not opened.
- 1:500 body (twice) and structure: recognisable dog with stout legs; the structure is thick rods (section 7). 1:500 thermal rendered, not opened.
- Poses at 1:50: `sitting-waiting` (right side with rig, later the six-view sheet) is a sitting dog, closed at every joint, haunches and all four paws on the ground; `walk-cycle` t = 0.3 s (right side with rig) with three paws planted, no gaps. `trot-cycle` t = 0.1 s at 1:100 (right side): closed, jaw open for panting; the lifted near upper arm lies horizontal and reads as a lobe on the chest. `sniffing-head-down` and `trot-cycle` at 1:50 were rendered on the working asset, not opened.
- `volumes-render.ts` sheet of 1:50 (overlays all green); the sheets of 1:100, 1:200, 1:500 were rendered, not opened.
- 2D (`r3-2d.ts`): 1:50 light-line and light-solid in three views, 1:50 `sitting-waiting`, 1:50 thermal and forces views (reactions 53.4 N per hind paw, 93.7 N per forepaw, residual 0), the 1:200 sheet. The 1:100 and 1:500 2D sheets were rendered, not opened.

## 6. Requests filed

- `author-requests.md`, "r3 animals/labrador-dog": `volumes-feature` forces 75 mm structure rods at 1:500 (measurements there).

## 7. Still imperfect

- 1:500 structure: every member must be 75 mm thick (or as thick as its drawn limb), so the skeleton is rods as thick as the legs, the tail rods stand up to 17 mm out of the tail tip, and a 75 mm rod stands for an ear. The body legs at 1:500 are 75 mm thick for the same rule (real 45-60 mm).
- Front and plan drawings are polygons in rest coordinates carried by their bones: exact standing, walking and trotting; in `sitting-waiting` the trunk shapes are foreshortened sheets (the front view still reads as a sitting dog, the plan shows a trunk that is too short). The revolved-solid shapes the old drawing used for this cannot follow a spine that lies at the top of the barrel.
- The overall length changed from 1.44 to 1.29 m on my judgement of the breed, without a measured table for nose-to-tail length; `depthM` 0.28 was kept although the model is 0.275 and a slimmer dog (0.25-0.26) would be as plausible.
- Bone lengths and joint heights come from my knowledge of canine anatomy, not from a looked-up table; the hip joint height (0.485) and the stifle angle (121 degrees, at the bent end of the range) are the least certain values.
- Shoulder, upper arm and thigh are separate solids that lie on the trunk with visible seams; in a strongly flexed foreleg (trot swing) the upper arm reads as a lobe.
- No coat texture in 3D, no eyelids, no dewclaws, no claws as separate solids (each toe ends in a point), no sex characteristics; the skull is one mass without eye sockets or teeth; the ear cartilage is the ear leather itself.
- The lower jaw is a separate loft under the lips, so the mouth line shows as a seam around the muzzle in the closed mouth.
- The 2D toe lines and fur strokes were placed by hand in world coordinates and checked only in the 1:50 standing and sitting sheets.

## Round 5 handcraft (2026-11-20)

Scripts: `r5-labrador-dog-rig.py`, `r5-labrador-dog-kit.ts`, `r5-labrador-dog-50.ts`, `r5-labrador-dog-50-lists.ts`, `r5-labrador-dog-coarse.ts`, `r5-labrador-dog-decisions.ts`, `r5-labrador-dog-draw2d.ts` (+ `r5-labrador-dog-rep-decisions.json`), `r5-labrador-dog-work.ts`, `r5-labrador-dog-sheet.ts`, `r5-labrador-dog-candidate.py`, `r5-labrador-dog-apply.py`, `r5-labrador-dog-build.sh` (whole rebuild + candidate validation), `r5-labrador-dog-run.sh`. Input kept: `🗑️generated/r3-labrador-dog/asset-before.json` (the rig script still re-targets every pose from it).

### Reference table (adult male Labrador Retriever, 30 kg, standing alert)

Sources: FCI standard No. 122 (withers 56-57 cm, broad skull with stop, deep wide barrel chest, otter tail "very thick towards the base, gradually tapering", clothed all round); AKC standard (point of shoulder to point of rump equal to or slightly longer than withers height, elbow at half the withers height, tail not beyond the hock); Evans and de Lahunta, Miller's Anatomy of the Dog (thoracic inlet bounded by T1, the first ribs and the manubrium; ribs leave the vertebral bodies and run caudoventrally; cervical column runs in the ventral half of the neck and meets T1 low; thoracic spinous processes tallest at T1-T5, forming the withers). Breed numbers as in section 1 of this report.

| measure | reference | round 4 model | round 5 model |
|---|---|---|---|
| withers (skin over T3) | 0.56-0.57 m | 0.568 | 0.568 |
| top of head, alert | 0.70-0.74 m | 0.707 | 0.708 (`heightM` 0.72 -> 0.71, the measured model, inside the reference) |
| T1 vertebral body height (thoracic inlet: manubrium 0.39 m + inlet height 0.09-0.10 m) | 0.48-0.49 m | 0.448 (on the shoulder joint, 4 cm too low, -8 %) | 0.488 |
| T13 vertebral body height (skin 0.555 m minus 5 cm) | 0.50-0.51 m | 0.502 | 0.505 |
| thoracic column, T13 to T1 chord | 0.21-0.23 m | 0.231 | 0.211 |
| cervical column, C7 to atlas | 0.21-0.24 m | 0.245 | 0.224 |
| neck inclination at rest | 40-50 degrees | 54.6 (pointer carriage) | 45.5 |
| neck girth at mid neck (fur) | 0.45-0.50 m | about 0.41 (0.135 x 0.122 m) | about 0.47 (0.15 x 0.144 m) |
| rib slope, dorsal end to ventral end | caudoventral, 10-25 degrees from the vertical | vertical hoops, dorsal ends 4 cm above the vertebral bodies | 8 to 20 degrees, dorsal ends on the vertebral body line |
| tail base thickness (fur) | 0.06-0.07 m | 0.056-0.062 | 0.066-0.072 |
| tail at tip third | 0.03-0.04 m | 0.022-0.031 | 0.027-0.045 |
| tail carriage at rest | from the croup slightly downward, a slight upward curve at the end, not over the back | straight, 4 degrees down (plank) | 16 degrees down, 6 degrees down, 4 degrees up |
| muzzle width (fur) | 0.08-0.09 m | 0.072-0.08 | 0.082-0.094 |
| forearm width mid (fur) | 0.055-0.06 m | 0.05 | 0.055 |
| hind and fore limb joints | section 1 | unchanged | unchanged (hip 0.485, stifle 0.291, hock 0.155, shoulder 0.423, elbow 0.284, carpus 0.105; within 1 % of section 1) |

### Grades (my eye, opened sheets)

| scale | list | before | after |
|---|---|---|---|
| 1:50 | body | C+ (pointer neck, plank tail, thin muzzle) | A- |
| 1:50 | structure | C (thoracic column dropping to the shoulder joint, ribs standing above the spine) | A- |
| 1:50 | thermal | B (reference list over the old body) | A- (references of the new body, outlets unchanged) |
| 1:100 | body / structure / thermal | C+ / C / B | A- / B+ / A- |
| 1:200 | body / structure / thermal | B- / C / B (area total failing, 34 %) | A- / B+ / A- (total 16 %) |
| 1:500 | body / structure / thermal | C / D / B | B+ / B / B+ |

### Changes

- Rig (root): `thorax` (0.245, 0.505) -> (0.455, 0.488), `neck` (0.455, 0.488) -> atlas (0.612, 0.648), `lumbar` end 0.502 -> 0.505; tail rest angles 184/180/176 -> 196/186/176 degrees; thorax centre of mass moved into the new barrel. All 19 poses re-solved by the round 3 solver with every pad contact kept (only `sitting-waiting` needed the scapula turned by -6 degrees, as before). Anchors recomputed on the new bones.
- 1:50 body: neck rebuilt with a crest and a deeper throat (lateral half-width 0.084 -> 0.056 m along the neck); otter tail thick at the root and tapering in three overlapping pieces along the new curve; broader muzzle and lower jaw; ears hang against the cheeks (no plan-view flare); forearm with more bone.
- 1:50 structure: `SPINE_Y` vertebral body line (kyphosis over the 8th rib, T1 low at the inlet), thoracic column as spinous process ridge plus bodies on that line, nine rib hoops from the vertebral bodies to the sternum sloping caudoventrally 8 to 20 degrees (`hoop` gained a slope), cervical column in its S curve meeting T1, cervical transverse process plate added, caudal bars on the new tail joints.
- 1:100 / 1:200 / 1:500: same spine, rib, neck and tail corrections stated anew with the scale's own sections; 1:500 tail root lowered into the croup (no fin), 1:500 caudal rods kept inside the tail; 1:200 pad slabs sized to the honest pad area (thermal total 34 % -> 16 %).
- 2D: every scale and view redrawn from the new bodies (`r5-labrador-dog-draw2d.ts`); the 1:200 lower legs are now drawn per bone (`hind-lower`/`fore-lower` + `hind-paw`/`forepaw`), the paw pad marks are one hull per paw (the two pad rings stood 1.75 mm apart, below the print gap).
- Decisions of 1:50 structure rewritten (en + de).
- Final: `r3-validate.ts animals/labrador-dog --volumes` VALID (schema 0, validateAsset 0, checkVolumes 0); `volumes-check.ts animals/labrador-dog --product` PASS at every scale.

### Seen (opened)

1:50 body six views (standing), `sitting-waiting`, `trot-cycle` t 0.1; 1:50 structure six views, three-quarter and right side alone (rib slope checked and corrected after a first wrong sign); 1:200 body six views; 1:100, 1:500 body and structure right side; 2D 1:50 of the candidate. Not opened this round: thermal cut sheets (thermal is by reference to the body solids), 1:100/1:500 2D sheets.

### Remaining

- 1:500 structure stays rods of 75 mm (feature rule, filed in round 3); 1:500 legs stouter than life for the same rule.
- The scapula and upper-arm masses still show seams on the chest; the 1:50 lumbar spinous and transverse processes are continuous plates (one feature wide), not single vertebrae.
- Plan view of `sitting-waiting`: a small notch between neck and shoulder mass.

## Lumbar column and shoulder in the sitting pose

Seven closed vertebral shells replace the three continuous lumbar strips. The hand-specified bodies increase in width from 21.5 to 25.7 mm. The spinous processes change height down the column, and the paired transverse processes extend and sweep forward by different amounts at each level. Every shell has a continuous central canal. Independent Three.js rays cross the solid centrum and pass through the canal without intersecting a triangle at each scale. The seven vertebrae follow the existing lumbar rig segment as a group.

The forequarter has reshaped scapular and humeral tissue, a broader costal region and a smoother lower neck. The seated neckline drop over an 8 mm sample fell from 32/32/20/30 mm to 4/5/5/4 mm, in scale order. The two-sided attachment probe measured 304 connected pairs in 19 poses at four scales. The trunk and forelimb thermal exchange areas were measured on exposed skin and fur. The combined catalog revision `dbe3b1e913aaf784` passed document, geometry and twelve-scene GLB checks at 5,128,224 bytes with the then-current encoder. The oblique, dorsal and axial lumbar detail views and the seated model in all three modes were visually inspected. The smooth-normal view also exposed weak eye and muzzle detail in the coarse scales.

The levels and proportions are authored for this figure. The anatomical reference for vertebral bodies and their variation along the canine lumbar spine is the CT morphometry study. [Canine lumbar CT measurements](https://pmc.ncbi.nlm.nih.gov/articles/PMC11754510/)
