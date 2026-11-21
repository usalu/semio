# Round 3 — `animals/roe-deer` (2026-11-19)

STATE: DONE and spliced. Catalog asset valid (`r3-validate.ts animals/roe-deer --catalog-only`: schema 0, validateAsset 0), `volumes-check.ts animals/roe-deer --product`: 41 rule lines ok, 0 fail. Open points are in section 6.

How it was built: the validator ties rig, 2D extents and `dimensions` together, so the round was authored in a work document (`🗑️generated/r3-roe-deer/🔣️.json`, volumes source `volumes/adhoc/r3-roe-deer.json`; every tool takes the file path as asset spec) and the catalog asset was replaced in one write when everything agreed. Both work files are tool output and were deleted at the end; `sh r3-roe-deer-run.sh check` rebuilds them from `🗑️generated/r3-roe-deer/asset-before.json` (the asset before this round, kept as input), `sh r3-roe-deer-publish.sh` writes the catalog and splices with `volumes-insert.py`.

Scripts (ticket folder): `r3-roe-deer-io.py` (member writer, work document), `r3-roe-deer-rig.py` (rig, anchors, poses, clip keys, bone material, provenance note), `r3-roe-deer-kit.ts`, `r3-roe-deer-50.ts` (1:50 body and thermal, hand tables), `r3-roe-deer-bones.ts` (structure of all four scales), `r3-roe-deer-coarse.ts` (1:100, 1:200, 1:500), `r3-roe-deer-draw2d.ts` (2D redraw), `r3-roe-deer-texts.py` (drawing decisions), `r3-roe-deer-apply.py`, `r3-roe-deer-bbox.ts`, `r3-roe-deer-print.ts` (print suitability), `r3-roe-deer-frames.ts`, `r3-roe-deer-run.sh`, `r3-roe-deer-publish.sh`.

## 1. Reference (roe buck, Capreolus capreolus, 25 kg, summer coat, six-point antlers)

Sources: Andersen, Duncan, Linnell (1998) "The European Roe Deer" (asset reference `andersen-1998`: head-body length 0.95 to 1.35 m, shoulder height 0.65 to 0.75 m, 15 to 35 kg, tail 2 to 3 cm, antlers 0.20 to 0.25 m); own knowledge of ruminant anatomy after Ellenberger/Dittrich/Baum "Handbook of Animal Anatomy for Artists", Goldfinger "Animal Anatomy for Artists" (deer plates), Nickel/Schummer/Seiferle "Lehrbuch der Anatomie der Haustiere"; greatest lengths of roe deer long bones as archaeozoological collections report them (femur 180 to 195 mm, tibia 215 to 240, metatarsal 185 to 205, humerus 150 to 165, radius 165 to 180, metacarpal 155 to 170, scapula height 140 to 150, first / second / third phalanx about 39 / 27 / 28 mm); hunting literature for the track (4.5 x 3.5 cm) and the ear (12 to 14 cm). No web search was run: the values are orders of magnitude from memory, not measurements of one animal.

| landmark / size | reference | model (world: x ahead of the hip joint, y above ground, rest pose) |
|---|---|---|
| withers height | 0.65 to 0.75 m (large buck 0.75) | skin 0.754 |
| croup height | 2 to 4 cm above the withers | 0.776 |
| trunk, point of shoulder to point of buttock | about 1.0 x withers height | 0.757 (-0.109 to 0.648) |
| chest depth | about 40 % of withers height | 0.309 (0.754 to 0.445) |
| chest width at the barrel / over the shoulders / hindquarters | 0.18 to 0.20 m | 0.192 / 0.184 / 0.198 |
| hip joint | under the croup | (0, 0.660) |
| femur, hip to stifle | 0.185 m | 0.1846, stifle (0.092, 0.500) |
| tibia, stifle to hock | 0.225 m | 0.2249, hock (-0.020, 0.305) |
| tarsus + metatarsal, hock to fetlock | 0.23 m | 0.2299, fetlock (0.000, 0.076) |
| pastern and claw, fetlock to toe tip | 0.09 m, 55 degrees | 0.0921, 55.6 degrees |
| scapula with cartilage | 0.165 m, 50 to 55 degrees to the horizontal | 0.164, 52.4 degrees, top (0.505, 0.705) |
| shoulder joint | about 0.76 x withers | (0.605, 0.575) |
| humerus | 0.15 to 0.16 m | 0.1521, elbow (0.535, 0.440) |
| radius + half carpus | 0.19 to 0.20 m | 0.1953, carpus (0.545, 0.245) |
| metacarpal | 0.17 m | 0.169, fetlock (0.548, 0.076) |
| lumbar / thoracic / cervical spine | 0.17 to 0.19 / 0.27 to 0.30 / 0.29 to 0.32 m | 0.190 / 0.295 / 0.310 |
| skull with nose | 0.21 to 0.22 m, 0.095 wide at the eyes | 0.220, 0.092 |
| ear | 0.12 to 0.14 m | 0.135 x 0.066 |
| antler | 0.20 to 0.25 m, beam with brow and back tine on a burr | 0.20, beam 21 to 16 mm, burr 33 mm |
| cannon thickness | 19 to 22 mm with skin | hind 22 x 19 mm, fore 20 x 18 mm |
| claw | 44 to 50 mm long, 13 to 15 mm wide, track 45 x 35 mm | 44 x 12.4 x 24 mm, two per foot |
| tail | 2 to 3 cm, hidden in the rump patch | no tail solid |

## 2. Audit of the state before this round

- `dimensions`: 1.46 m long (drawn 1.41 m rump to nose) at 0.75 m withers. That is longer than the longest head-body length of the species measured along the back (1.35 m); the trunk from buttock to point of shoulder was 0.89 m, 1.19 x the withers height. The asset read as a long-bodied calf.
- Rig: the spine chain was one horizontal line at the hip height (0.695 m): pelvis 0.22, lumbar 0.26, thorax 0.36 m (real 0.10, 0.19, 0.295), the neck base sat on that line at 0.745 m, 0.13 m above the real C7/T1 joint, so the neck grew out of the back. Jaw bone 0.12 m (real 0.17). The alert pose lowered the neck below the rest pose (32 degrees) and dropped the body 17 mm by a wide stagger. Limb bones were within 10 % of the reference (forearm 0.21 and fore cannon 0.19 a little long).
- Summary says roe buck with antlers, the provenance note said doe, and no antler was drawn or modelled anywhere; material `antler-bone` was unused.
- Anchors: hoof footprints 60 x 50 and 65 x 55 mm (real track 45 x 35 mm), `back-plume` 99 mm above the thorax bone in the air.
- 2D: elevation with a box trunk, a neck stuck on the withers, a wedge head, no croup; front and plan constructed, not projections; the white rump patch drawn with tone 0.12 (near black) and as a full disc in front and plan; hoof-cleft strokes in the side elevation (not visible from the side). The 1:500 representation did not print at all: `measureSuitability` reported feature 0.19 mm under the 0.2 mm of the default print profile, so the product refused the drawing ("no suitable representation at 1:500"); nothing in `validateAsset` catches this.
- Solids: 35 body solids at 1:50 (tube trunk, capsule legs, ears detached), thermal with a floating plume disc and one eye solid, structure of capsules with joint balls larger than the members.

## 3. Root fixes in the asset file (members changed: `provenance`, `dimensions`, `materials`, `rig`, `anchors`, `representations`, `poses`, `clips`, `volumes`)

- `dimensions` (cited: Andersen et al. 1998 body size ranges, section 1): heightM 1.08 to 1.122 (antler tips in the default pose), widthM (length) 1.46 to 1.035, depthM 0.2 to 0.203; mass unchanged.
- `rig.bones` lengths, old to new (m): pelvis 0.22 to 0.1039, lumbar 0.26 to 0.1901, thorax 0.36 to 0.2954, neck 0.34 to 0.31, head 0.22 (kept), jaw 0.12 to 0.1716, ears 0.14 to 0.135, thigh 0.21 to 0.1846, shank 0.25 to 0.2249, hind cannon 0.23 to 0.2299, hoof 0.09 to 0.0921, scapula 0.17 to 0.164, humerus 0.16 to 0.1521, forearm 0.21 to 0.1953, fore cannon 0.19 to 0.169. All origins and rest angles follow the landmarks of section 1 (table `J` in `r3-roe-deer-rig.py`); the spine chain is now the vertebral axis (lumbosacral 0.090 / 0.712, thoracolumbar 0.280 / 0.706, C7/T1 0.560 / 0.612, poll 0.751 / 0.856 at rest). Centres of mass moved with the bones. Joint limits widened where the re-solved poses need it: neck -125..45 to -150..45, head -45..60 to -60..75, ears -50..60 to -75..60, thigh -50..62 to -60..65, humerus -45..75 to -55..90, forearm -30..120 to -40..120.
- `poses` (21) and the 32 clip keys that carry a full set of bone offsets: every one re-solved (the three partial keys of `alarm-freeze` keep their jaw and ear offsets, the ear ones shifted with the new alert pose). Per leg the sole anchor keeps its old shift from its rest position and its height above the ground, hoof, cannon and scapula keep their rotations, thigh/shank and humerus/forearm come from two-bone IK. `alert` and `alert-foreleg-raised` were authored anew (neck raised to 62 degrees, head carried at -12 degrees, ears pricked, stagger 35 mm, withers at full height); `grazing` anew (trunk pitched 9 degrees, forelegs spread 0.19 m ahead and 0.11 m back instead of kneeling, muzzle 35 mm above the ground). Known rest: in eight swing-phase keys of the walk and in `alert-foreleg-raised` a fore sole misses its old target by 2 to 8 mm because the shorter foreleg cannot reach it (the solver clamps; no ground contact is affected).
- `anchors`: sole contacts 21 mm behind the new toe tips with footprints 45 x 32 mm (hind) and 48 x 35 mm (fore); `nose` and `eye-anchor` on the new head; `back-plume` on the back surface.
- `materials`: `bone` added for the skeleton. `provenance.notes`: buck instead of doe, and what round 3 re-authored.
- `representations`: every filled shape of the four scales and three views redrawn as the orthographic outline of the body solids of its shape (ruling E1) with the same ids, roles, layer order and tones, solved through the product's own placement (`inverse` of `r3-project2d.ts`), so limb shapes keep `followTip`. Added shapes: `croup` (pelvis bone; the rump moved with the lumbar spine before), `near-antler` / `far-antler`, thigh, shoulder and upper arm in front and plan, eyes in the front view at 1:100, at 1:500 the lower legs (cannon bones, filled) and the far ear; the 1:500 leg strokes became filled shank and forearm shapes so that the legs bend. The neck is one extruded `solid` shape, so front and plan are its projections in every pose (a flat plan shape vanished when the neck points down to graze). `rump-patch`: rebound to the pelvis, tone 0.12 to 0.95, layered under the croup so that only the part behind the body shows. Removed: `hoof-cleft-h` / `hoof-cleft-f` in the 1:50 elevation, eyes in the 1:50 plan. Detail and texture strokes (brow, mouth line, inner ear, coat strokes, shoulder and hip lines) restated by hand along their bones. Shape and point counts (elevation / front / plan): 1:50 37 / 32 / 30 shapes with 572 / 511 / 463 points (before 36 / 23 / 23 shapes with up to 482 points in one shape), 1:100 33 / 32 / 30, 1:200 31 / 29 / 29, 1:500 20 / 20 / 20. The texts of `representations[*].decisions` were rewritten for the new drawings. Print suitability under the default profile is now true at all four scales (1:500 feature 0.206 mm).
- Not changed: summary, every `areaM2`, masses, thermal and structural data, clip timing, contacts and events.

## 4. Models

| scale | body | thermal | structure |
|---|---|---|---|
| 1:50 | 58: croup, loin, chest (three lofts, 20 hand sections), neck, skull, mandible, nose leather, rump patch, 2 eyes, 2 ears, per antler pedicle, burr, beam, brow tine, back tine; per hind leg thigh, gaskin, hock point, cannon, 2 dewclaws, pastern, 2 claws; per foreleg shoulder, upper arm, forearm, cannon, 2 dewclaws, pastern, 2 claws | 53: one reference per body solid by region (antlers excluded), 2 nostril outlets on the nose leather, plume patch on the back | 122: skull, pedicles, mandible, cervical / thoracic / lumbar spine with processes and withers blade, sacrum, ilium, ischium, pubis, 26 ribs, costal arches, sternum, ear cartilages, scapula with spine, humerus, radius, olecranon, metacarpal, femur, patella, tibia, metatarsal, calcaneus, 2 digits and pedal bones per foot, antlers (references: the antler is bone), 26 joints, 3 masses |
| 1:100 | 42: own tables; one hoof per foot, no dewclaws, antler as beam with two tines | 38 | 74: simplified bones, six rib hoops per side |
| 1:200 | 37: own tables; head wedge with jaw bar, pastern and one rounded hoof, antler spike, no eyes or nose | 38 (eyes, nose as own solids at the floor) | 54: plain shafts, pelvic girdle, three rib hoops per side |
| 1:500 | 24: loin-croup and chest mass, neck, head, 2 ear stubs, 2 antler spikes, per leg three segments and a joint fill | 31 (rump patch, eyes, nose, hooves as own solids at the floor) | 29: one bar per bone |

`decisions` are bilingual sentences per list at every scale. Final check (catalog, `--product`): 41 ok, 0 fail. Silhouette IoU elevation / front / plan: 0.990 / 0.982 / 0.980 (1:50), 0.983 / 0.967 / 0.974, 0.973 / 0.960 / 0.965, 0.947 / 0.942 / 0.932. Thermal areas at 1:50: nose-skin -25 %, eyes +6 %, hooves +28 % (limit 30 %); every other region is an exchange area.

## 5. What I saw (PNGs opened with the Read tool)

- Before: six-view sheet 1:50 body and structure, 2D 1:50: a long tube body on short legs with a wedge head growing from the back.
- 1:50 body, final state: right side, low front three-quarter (turn 20); one iteration earlier also front, left side, rear three-quarter (turn 215) and the default three-quarter. It reads as a roe buck from every direction: croup above the withers, tucked flank, deep narrow chest, long neck from a low base, wedge head with a slim chin under the cheek, large ears, six-point antlers, thigh and gaskin tapering to a hock with its point, vertical cannons, sloping pasterns on wedge-shaped cloven hooves. The rig lies inside the forms.
- 1:50 structure (six views with rig): a recognisable deer skeleton, joints no larger than the bone ends. 1:50 thermal (six views, cut): regions by colour, plume patch flush on the back, nostril outlets on the nose leather.
- 1:100 and 1:200 body six-view sheets (final), 1:100 structure sheet, 1:500 body and structure sheets, the `volumes-render.ts` overlay sheets of 1:50 and 1:500 (overlays green with thin red and blue rims at the neck in plan, where the extruded neck solid is straighter than the loft).
- Poses at 1:50: grazing (3D six views and 2D), bounding-gathered (3D), walk-cycle at 0.3 s (2D): hooves on the ground, legs closed at every joint, the back arches at the lumbar joint in the bound.
- 2D through `r3-2d.ts`: 1:50 default, grazing and walk frame, 1:500 default. 1:100 and 1:200 2D sheets were rendered but not opened.

## 6. Still imperfect

- Trunk seams: croup, loin and chest are three solids so that the spine can flex; the 2D shows their two vertical boundaries inside the body in line and solid views (the old drawing had the same), and the 3D shows two thin seam rings.
- Claw tops: the thermal references of the eight claws mark their upper faces (`sectionFaces: ["-u"]`) as the junction with the toe (C25 b: the horn capsule is open where the digit enters), because the declared 0.013 m² cannot hold eight honest claws plus eight dewclaws otherwise (measured 0.0205 m² with the tops, 0.0167 without). The claws are also at the small end of the real range (44 x 12.4 mm) for the same reason, and the fore claws are only 1 % larger than the hind ones. An area reviewer should decide whether `hooves` 0.013 m² is too small; the nose leather is sized to cover the whole muzzle front (36 x 34 mm) to reach its declared 0.004 m² within tolerance.
- 1:200 and 1:500: shanks, cannons, pasterns, ears and antler spikes stand at the size floor (33 mm, 76 mm), thicker than the real parts (20 mm cannon); the 1:500 ears are rounded stubs and the muzzle ends in a flat cut.
- The neck in front and plan is a tapered prism (extruded solid), not the oval loft: IoU there is a little lower than in the elevation.
- Skeleton: spinous processes and withers are continuous blades, not single vertebrae; the skull has no orbit or nasal opening; ribs are round bars.
- Small solids come out of the outline tracer as few-point polygons (the 1:100 eye is a diamond of 4 to 5 points).
- The rump patch is a shallow cap 7 mm proud of the buttocks, not a change of coat colour on one surface.
- Not opened after the last edits: structure and thermal of 1:200, thermal of 1:500 beyond the overlay sheet, 2D of 1:100 and 1:200, clips other than one walk frame. No product test run (`repo run ... :test`) was started from this task.
- Reference values are from memory (section 1); no measured specimen or web source was consulted.

## Round 5 handcraft (2026-11-21)

Scripts: `r5-roe-deer-50.ts` (1:50 hand tables; neck base and scapula top remodelled), `r5-roe-deer-coarse.ts` (1:100, 1:200, 1:500 hand tables), `r5-roe-deer-draw2d.ts` (2D redraw, ruling E1), `r5-roe-deer-upper500.py` (the 1:500 shoulder-and-upper-arm outline drawn by hand: the traced outline carries a rear bump whose visible width is 0.176 mm on paper, under the 0.2 mm print feature), `r5-roe-deer-run.sh` (whole work-document pipeline from `asset-before.json` with the r5 scripts; `sh r3-roe-deer-publish.sh` writes the catalog and splices), `r5-grid.ts`. Rollback inputs: `🗑️generated/r5-roe-deer/asset-before.json`, `volumes-before.json`.

Note on the pipeline: re-running the round 3 pipeline unchanged did not reproduce the catalog of 2026-11-20 17:37: the limb outlines differ by tessellation noise (the tessellation fix of round 4) and the 1:500 `near-fore-upper` outline had been simplified by hand afterwards (that is what `r5-roe-deer-upper500.py` now states in a script).

### Reference

Section 1 of round 3 stands (Andersen et al. 1998, Ellenberger/Dittrich/Baum, Nickel/Schummer/Seiferle; bone lengths from archaeozoological collections). Model against it, 1:50, rest pose: withers 0.754 (ref 0.65 to 0.75, large buck, +0.5 %), croup 0.776 (2 to 3 cm above the withers, in range), femur 0.1846 (0.185), tibia 0.2249 (0.225), hind cannon 0.2299 (0.23), scapula 0.164 (0.165), humerus 0.1521 (0.15 to 0.16), radius 0.1953 (0.19 to 0.20), fore cannon 0.169 (0.17), skull 0.22 (0.21 to 0.22), ear 0.135 (0.12 to 0.14), antler 0.20 (0.20 to 0.25): every value within 1 %. Spine dorsal along the vertebral axis (lumbosacral 0.712, thoracolumbar 0.706, C7/T1 0.612), cervical spine curved. No rig fix was needed. Dimensions: length 1.035 to 1.038 m (measured body box after the new 1:500 muzzle row; heightM 1.122 and depthM 0.203 unchanged).

### Grades (my own look)

| list | 1:50 before / after | 1:100 | 1:200 | 1:500 |
|---|---|---|---|---|
| body | B+ / A- | B+ / A- | B / B+ | D / C+ |
| structure | A- / A- | B+ / B+ | B / B | C / C |
| thermal | B+ / B+ | B+ / B+ | B+ / B+ | B- / B- |

Before: in the grazing pose and the alarm-freeze clip the neck base swung its dorsal rear corner up out of the withers as a sharp fin (the loft started 0.07 m behind C7 with a 0.112 m half depth); at 1:200 the scapula top did the same; 1:500 antlers were 76 mm cylinders with ball tips (mushrooms taller than the ears), the muzzle a flat-cut tube.

### Changes

- Neck base at all four scales: first section moved to 0.05 m behind C7, lowered 30 mm, half depth 0.094 m; the withers now stay smooth from the alert pose to the grazing pose.
- Scapula top at 1:50, 1:100, 1:200 starts 30 mm lower on the blade, so it stays under the withers when the trunk pitches.
- 1:500: antlers taper from 84 to 44 mm without a ball tip (overall extent still above the 75 mm feature); head with two closing muzzle rows; joint fills at hock and carpus at the leg radius (no collar). The upper-arm outline is drawn by hand (see above).
- 2D of all four scales redrawn from the new solids (ruling E1), texts unchanged.

### Checks

Work document and catalog: `r3-validate.ts animals/roe-deer --volumes`: schema 0, validateAsset 0, checkVolumes 0 (print suitability included). `volumes-check.ts --product`: 41 ok, 0 fail. Silhouette IoU elevation / front / plan: 0.993 / 0.983 / 0.984 (1:50), 0.986 / 0.966 / 0.975, 0.973 / 0.962 / 0.968, 0.943 / 0.943 / 0.933. Published with `r3-roe-deer-publish.sh` (catalog unchanged since the backup; members `dimensions`, `representations`, `volumes` written; volumes spliced).

### Opened

Six-view body, structure and thermal (cut) sheets of all four scales, 1:500 three-quarter close-ups after each iteration, grazing at 1:50, 1:100 and 1:200 (side) before and after the neck fix, all 21 poses at 1:50 (four contact sheets), clip `alarm-freeze` at t 0.5 s.

### Remaining issues

- 1:500 is still a stack of 76 mm tubes: the 75 mm feature and the 0.2 mm print feature (100 mm visible width at 1:500) forbid tapering legs or slimming the antlers (real beam 20 mm); the far fore leg needs its carpus fill to stay printable. Grade C+ is the honest ceiling under those floors; a `slender` antler would need an open-stroke antler in the 1:500 drawing (tool request material, not done).
- 1:500 structure: one bar per bone at the floor, bulky (C).
- Trunk seams (croup, loin, chest as three solids for spine flexion) still show as thin rings and as two lines in the 2D.
- Claw area: `hooves` passes only with the claw tops marked as junction faces (round 3, section 6).
- The near foreleg in `grazing` is drawn back under the chest with a near-horizontal forearm (round 3 pose); it reads, but a straighter stance would be more typical.
