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
