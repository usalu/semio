# Round 3 — `animals/goat` (2026-11-19)

State: spliced. `r3-validate.ts animals/goat`: VALID (schema 0, validateAsset 0). `volumes-check.ts animals/goat --product`: 1:500 PASS; 1:50, 1:100 and 1:200 fail on one rule line each, `volumes-thermal-coverage`, because the declared `areaM2` of horns and hooves (0.012 m² each) is half and a quarter of the real parts (section 6); every other rule is green at every scale. Grade by my own eye: body A- at 1:50, B+ at 1:100 and 1:200, B at 1:500; structure A- / B+ / B+ / B; thermal A- at every scale; 2D side elevation A-, front and plan B+ in the standing pose and broken for a steeply pitched trunk (section 7).

This asset was taken over from an author who was cut off: the rig, the kit and first versions of all lists existed, the catalog held the new rig with the old volumes and the old drawings (20 validator findings). Everything was re-read, repaired and finished.

Scripts (ticket folder): `r3-goat-rig.py` (rig, anchors, poses; run once on `🗑️generated/r3-goat/asset-before.json`), `r3-goat-apply.py` (writes named `asset` members into the catalog file, everything else byte-identical), `r3-goat-kit.ts` (bench: stations in world metres, written into bone frames), `r3-goat-50.ts`, `r3-goat-50-lists.ts`, `r3-goat-coarse.ts` (hand tables of every solid), `r3-goat-decisions.ts`, `r3-goat-draw2d.ts` + `r3-goat-rep-decisions.json` (2D redraw), `r3-goat-candidate.py`, `r3-goat-run.sh` (whole pipeline), `r3-goat-outside.ts` (which structure solid leaves the body, per view and in 3D), `r3-goat-frames.ts`, `r3-goat-verify.ts`, `r3-goat-look2d.sh`, `r3-goat-probe.ts`, `r3-goat-work.ts`. Kept input: `🗑️generated/r3-goat/asset-before.json`, `roots-applied.json`, `rig-z.json`, `anchors.json`, `reps.json`.

## 1. Reference (horned dairy-type doe, 55 kg, standing alert)

Sources: own knowledge of goat anatomy after Nickel/Schummer/Seiferle "Lehrbuch der Anatomie der Haustiere" (bone lengths, joint positions of small ruminants), Popesko "Atlas of Topographical Anatomy of the Domestic Animals", Ellenberger/Dittrich/Baum and Goldfinger (surface anatomy for artists); breed descriptions of the Bunte Deutsche Edelziege, Toggenburg and Alpine doe (withers 0.70 to 0.80 m at 55 to 75 kg, body length point of shoulder to pin bone 1.05 to 1.2 times the withers height, chest depth 43 to 48 % of it); the asset's own `summary` (0.76 m withers, 55 kg). No web source was consulted.

| landmark / size | reference | model (world: x ahead of the hip joint, y up, m) |
|---|---|---|
| withers height | 0.76 (asset), breed 0.70 to 0.80 | 0.760 (skin over T4) |
| croup (hook bones) height | about withers height, slightly lower | 0.759 top line at the hip, hook bone 0.72 |
| trunk length, point of shoulder to pin bone | 0.80 to 0.91 | 0.90 (x 0.789 to -0.114); at the long end of the range |
| chest depth / width | 0.33 to 0.36 / 0.20 to 0.23 behind the elbow | 0.345 (0.76 to 0.415) / 0.20 at the girth, belly 0.31 |
| elbow height (leg share) | 56 to 60 % of withers | 0.438 (58 %) |
| hip joint height | 0.62 to 0.66 | 0.640 |
| femur / stifle height | 0.20 to 0.22 / 0.45 to 0.47 | 0.214 / 0.462 |
| tibia / hock joint height | 0.23 to 0.25 / 0.24 to 0.26 | 0.236 / 0.250, point of hock 0.292 |
| metatarsal with tarsus / fetlock height | 0.16 to 0.18 / 0.075 to 0.085 | 0.172 / 0.079 |
| pastern and claw, fetlock to toe tip | 0.095 to 0.105, dorsal wall 50 to 55 degrees | 0.100, 52 degrees |
| scapula, inclination to the horizontal | 0.17 to 0.19, 55 to 60 degrees | 0.175, 58 degrees; dorsal border 0.04 m from the midline |
| shoulder joint height | 0.58 to 0.61 | 0.596 |
| humerus / radius / metacarpal | 0.16 to 0.18 / 0.20 to 0.22 / 0.13 to 0.15 | 0.174 / 0.212 / 0.147 |
| carpus height | 0.21 to 0.23 | 0.226 |
| cannon | 26 to 32 mm sagittal, 24 to 28 mm lateral | fore 28 x 26, hind 30 x 25 mm |
| lumbar / thoracic / cervical column | 0.25 to 0.28 / 0.31 to 0.34 / 0.36 to 0.40 | 0.267 / 0.330 / 0.373 |
| skull length / depth / width at the orbits | 0.23 to 0.26 / 0.11 to 0.13 / 0.11 to 0.125 | 0.245 with lip / 0.115 with jaw 0.14 / 0.12 |
| ear | 0.12 to 0.14 x 0.055 to 0.065, carried sideways and slightly forward | 0.128 x 0.064 |
| horns of a doe | 0.18 to 0.28 along the curve, base 30 x 22 mm, scimitar, diverging | 0.225, base 31 x 23 mm, tips 0.17 m apart |
| claw | 50 to 60 x 20 to 23 x 30 to 35 mm, two per foot, dewclaws | 56 x 22 x 34 mm (fore 5 % larger) |
| tail | 0.12 to 0.15, carried up | 0.13 |
| udder | 0.10 to 0.12 wide, two teats 40 mm | 0.10 x 0.11 x 0.10, teats 40 mm |

## 2. Audit of the state before this round

- Rig (joint heights above ground, old against reference): the spine was one line 0.59 m high through the middle of the barrel (lumbar joint 0.596, thorax 0.589; vertebral axis is 0.71) and jumped 0.17 m up to the neck base at 0.759, which sat on the withers instead of at C7/T1 (0.655); fetlocks 0.047 (0.079), hock 0.216 (0.250), stifle 0.444 (0.462); hoof bones 0.060 long (0.100); neck 0.300 (0.373); jaw 0.115 (0.175); metacarpus 0.170 (0.147); lumbar 0.320 (0.267). Scapula joints 0.075 m from the midline (the dorsal border lies 0.04 m out).
- `dimensions` 1.09 x 1.31 x 0.34 m, 55 kg: consistent with the reference (model 1.089 x 1.289 x 0.337); not changed.
- 2D: every view drawn on the wrong skeleton; front and plan constructed from ellipses and columns, forelegs and neck layered behind the chest in the front view, far-side tones asymmetric in a symmetric view; hooves as wedges three times the real claw, horns as 30 mm blades; outlines with up to 500 points per shape.
- Solids before: segmented barrel, cone neck, stick legs, a T-bar head at 1:500, plume disc above the back, structure of capsules with joint balls (visual review grades B to C).
- As found at take-over: horns too fat towards the tip, no Achilles tendon (a notch between gaskin and point of hock), hook bones and scapula tops outside the body, joint pins wider than the legs, outlets thinner than the feature at every scale, 1:500 skeleton rods with gaps at every joint, tail of the two coarse scales bent against the rig, placeholder decisions.

## 3. Root fixes in the asset file

- `rig.bones` (old to new length, m): pelvis 0.1453 to 0.1298, lumbar 0.3197 to 0.267, thorax 0.34 to 0.33, neck 0.30 to 0.3726, head 0.22 to 0.23, jaw 0.115 to 0.1746, ears 0.10 to 0.12, thigh 0.22 to 0.2136, shank 0.24 to 0.2361, metatarsus 0.17 to 0.1717, hoof bones 0.06 to 0.0998, scapula 0.18 to 0.1751, humerus 0.18 to 0.174, forearm 0.22 to 0.2122, metacarpus 0.17 to 0.1471; all origins and rest angles from the world joint table of `r3-goat-rig.py`; masses unchanged, centres of mass carried over. Lateral offsets: thigh ±0.07, ears ±0.048, scapula ±0.04 with humerus a further ±0.035 (was ±0.075 at the scapula: its joint lay outside the withers).
- `anchors`: hoof contacts at the new toe tips (x 0.06 to 0.0998), nose (0.228, 0) to (0.236, 0), mouth (0.2, -0.022) to (0.205, -0.024), `back-plume` from 0.08 m above the back onto the back line, thorax (0.1359, 0.0645).
- `poses`: all 15 carried over: trunk, neck, head, ear and tail offsets kept, each leg re-solved by two-bone IK so that every toe tip and sole inclination of the old pose is kept (worst planted contact shift 0.00000 m, `r3-goat-verify.ts`). Clips reference the poses and were not touched.
- `materials`: `bone` added.
- `representations`: every filled shape of the four scales and three views redrawn as the orthographic outline of the solids it stands for (E1), in the frame the product places it in (solved through `evaluate`, including `followTip`), simplified to a fifth of the smallest feature: 612 / 619 / 617 points at 1:50 (elevation / front / plan), 389 / 404 / 362 at 1:100, 198 / 205 / 208 at 1:200, 92 / 105 / 106 at 1:500 (before: 3491 / 2518 / 3077 at 1:50). Existing ids, roles, pens and tones kept; new shapes where a bone or part had none (croup, udder, jaw in front and plan, far thigh, shoulder and upper arm, pasterns, second claw in front and plan, far ear and horn at the coarse scales, tail at 1:200 and 1:500). Drawn by decision, not dumped: the rear edge of the chest in the side elevation is the costal arch (the hindbody shape carries what lies behind it, the two trunk lofts overlap 0.2 m inside the ribcage for it); the croup lies under the hindbody so no cap edge crosses the loin; in the front view neck (z 3.4) and forelegs (z 3.6) stand in front of the chest; leg tones are symmetric in front and plan; `mouth-line` and the tendon line restated by hand. `decisions` of all four representations rewritten (`r3-goat-rep-decisions.json`).
- Not changed: `dimensions`, every `areaM2`, masses, thermal and structural data, clips.

## 4. Models

| scale | body | thermal | structure |
|---|---|---|---|
| 1:50 | 58: ribcage, abdomen, loin, withers, croup, udder, two teats, neck, head, jaw, beard, eyes, nostrils, horns, ears, tail in two; per hind leg thigh, gaskin with tendon, point of hock, cannon, pastern, two claws, two dewclaws; per foreleg shoulder, arm, forearm, cannon, pastern, two claws, two dewclaws | 62: one reference per body solid by region, two nostril patches, lip patch, plume strip | 89: skull, horn cores, mandible, cervical, thoracic and lumbar column, lumbar processes, eight rib hoops, sternum, sacrum, hip bones, tail vertebrae, ear cartilages, scapulae, humeri, radius with olecranon, metacarpals, femora with trochanter and patella, tibiae, calcanei, metatarsals, two digits and coffin bones per foot, 27 hinge pins, two masses |
| 1:100 | 35: chest, hindbody, croup, udder, neck, head, beard, horns, ears, tail in two; per leg thigh or shoulder and arm, gaskin or forearm, cannon, pastern, hoof block | 38 | 57: skull, mandible, columns, five rib hoops, sternum, pelvic girdle, scapula plates, tapered bars, 21 pins, two masses |
| 1:200 | 25: chest, hindbody, neck, head, horns, ears, tail; per leg thigh or shoulder, gaskin or forearm, cannon, hoof block | 28 | 33: skull, bars, three rib hoops, pelvic girdle, two masses |
| 1:500 | 17: chest, hindbody, neck, head, horn wedges, ear stubs, tail stub; per leg upper and lower piece | 24 (with four hoof blocks and muzzle caps as outlets) | 31: skull, joint-to-joint rods of 75 mm, one rib girth, two masses |

Final check lines: 1:50 `FAIL: 2 measurement(s) over 1 rule line(s)` (areaDeviation horns 1.02, hooves 2.89); 1:100 `FAIL: 2 measurement(s) over 1 rule line(s)` (horns 1.00, hooves 1.88); 1:200 `FAIL: 1 measurement(s) over 1 rule line(s)` (areaDeviationTotal 1.29); 1:500 `PASS`. Silhouette IoU elevation / front / plan: 0.994 / 0.989 / 0.995 at 1:50, 0.991 / 0.982 / 0.988 at 1:100, 0.979 / 0.961 / 0.981 at 1:200, 0.955 / 0.931 / 0.946 at 1:500. Structure outside the body: at most 2.3 mm (1:50), 3.7 mm, 5.3 mm, 21 mm (limit 7.5 / 15 / 30 / 75); no member vertex share above 8 % outside the body in 3D at 1:50 and 1:100.

## 5. What I saw (images opened with the Read tool)

- 1:50 body, six-view sheet with rig, and full-size right side, front and three-quarter: reads as a dairy goat at once from every side: straight top line with a slight rise at the withers, sloping croup, deep flat-sided chest, wide belly, long legs with a clear hock and tendon, long neck with throat line, wedge head with jaw, beard, scimitar horns, sideways ears, udder between the hind legs. Front: narrow brisket between straight forelegs, barrel wider behind. Back: rounded quarters, udder with two teats. Top: barrel widest at the belly, shoulders and thighs standing out. Three-quarter: shoulder and upper arm lie on the ribcage as slightly plated masses with visible seams; the rig joints lie inside the body at every joint.
- 1:50 thermal (six views): every region coloured over the whole body, the plume strip a low ridge in the back line, nostril and lip patches on the muzzle; nothing floats. The cutaway sheet was rendered but not opened: there are no nested shells to inspect (one coat layer per region).
- 1:50 structure (six views and full-size three-quarter): a goat skeleton with the cervical column entering the chest low, tall withers processes, rib basket, lumbar processes, pelvis from hook to pin bone, legs with the right bone proportions, two digits per foot; pins small at the joints.
- 1:100 body and structure sheets: the same animal with merged trunk, one hoof per foot; skeleton with five rib hoops and bars. 1:100 thermal sheet rendered, not opened.
- 1:200 body and structure sheets: clean main masses, horns and ears thickened; skeleton as a bar diagram with three rib hoops. 1:200 thermal rendered, not opened.
- 1:500 body, thermal and structure: recognisable goat with stout legs, horn wedges and ear stubs; hoof blocks and muzzle caps in the thermal list; rods now continuous through every joint (they had gaps before the last change; right side opened again after it).
- Poses at 1:50: `browsing-upright` (six views) stays closed at every joint, hind legs deeply crouched, forelegs folded; `walk-cycle` t = 0.3 s (six views) with three hooves planted and one lifted, no gaps. `climbing-ledge` rendered, not opened.
- 2D (product drawings through `r3-2d.ts`, enlarged from the SVG): 1:50 side elevation light-line and light-solid, front and plan light-solid; 1:100 side elevation; the 1:200 sheet; 1:50 thermal and forces views (reactions 118 N per hind hoof, 151 N per fore hoof, residual 0); `browsing-upright` at 1:50. The 1:500 2D sheet was rendered but not opened.

## 6. Open: areas of horns and hooves

`areaM2` of `horns` and `hooves` is 0.012 m² each. The model measures 0.024 and 0.047 m² at 1:50; a hand calculation of the real parts gives 0.024 and 0.042 to 0.047 m². C18 and C27 forbid the author to change these values, and shrinking the horns to 0.11 m or the claws to 28 mm would be the opposite of this round. Filed for the area reviewers in `area-review.md` and `author-requests.md` (section "r3 animals/goat") with the proposal horns 0.024, hooves 0.047, taken from `coat-trunk`. With those values every scale passes. Until then the catalog asset carries five `checkVolumes` findings (`r3-validate.ts animals/goat --volumes`).

## 7. Still imperfect

- Front and plan drawings are polygons in rest coordinates: exact standing and walking, wrong for a steeply pitched trunk. In `browsing-upright` the front view stacks chest, hindbody and neck outlines above each other and the head floats; the old drawings did the same. The schema's `solid` hints cannot express a barrel that hangs below the spine bones (`tool-requests.md`, "r3 animals/goat").
- The trunk length (0.90 m shoulder point to pin bone) is at the long end for 0.76 m withers; it was kept because `dimensions.widthM` 1.31 fixes the overall length.
- Shoulder and upper arm read as plates on the ribcage in the three-quarter view; a real goat shows them less.
- In `browsing-upright` the hind legs crouch more than a rearing goat's (the pose keeps the old toe positions on the corrected, longer hoof bones).
- Solid-tone 2D at 1:50 and 1:100 shows one line across the back in plan where the chest shape ends; it is a straight articulation line, not an anatomical one.
- 1:500: legs, horns, ears and tail are 75 mm thick by the feature rule, so the animal is stouter than life; the skeleton rods are as thick as the legs.
- No hair texture, no eyelids or pupils, no hoof cleft line in the side elevation.
