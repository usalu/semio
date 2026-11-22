# animals/horse — running report

## Pass 1 (second round), 2026-11-22

Status: **partial**. Rig stays sagittal: `rig-lateral-rest.md` did not exist at the start, after stage 1, or at the end, so no girdle was built and none was faked. Source and catalog were identical at the start (the earlier pass of `rework/07.md` had been published); they are identical again now.

Scripts (ticket folder): `rework-p1-horse-body.py` (section tables of trunk, withers, breast, pectorals, neck base, shoulder, upper arm, tail at every scale, and the `decisions` entries), `rework-p1-horse-structure.py` (skeleton), `rework-p1-horse-2d.py` (layering of the redrawn shapes). The 2D outlines were carried with the earlier `rework-07-2d.py` (ruler `r3-project2d.ts`).

### Reference (warmblood, 1.61 m at the withers, 550 kg; own knowledge, no source consulted)

| Measure | Real | Model at the start | Model now |
|---|---|---|---|
| Withers / croup / lowest point of the back | 1.61 / 1.59–1.61 / 1.50–1.53 m | 1.56 / 1.60 / 1.50 m (croup-high, withers a wedge lying on the back) | 1.615 / 1.60 / 1.517 m |
| Body length, point of shoulder to point of buttock | about the withers height, 1.60–1.68 m | 1.65 m | 1.66 m |
| Girth depth (withers to breastbone) | 0.72–0.76 m | 0.63 m | 0.74 m |
| Barrel breadth / over the shoulders / points of the shoulder | 0.58–0.62 / 0.50–0.55 / 0.40–0.45 m | 0.59 / 0.47 / arm caps standing free at 0.34 m | 0.61 / 0.52 / 0.43 m |
| Rump breadth at the points of the buttock | 0.30–0.36 m | 0.26 m, pointed | 0.30–0.37 m, squared |
| Scapula / humerus / radius / fore cannon with carpus / pastern | 0.45–0.50 / 0.30–0.34 / 0.36–0.40 / 0.30–0.33 / 0.13 m | 0.483 / 0.326 / 0.375 / 0.303 / 0.13 m | unchanged |
| Point of shoulder / elbow / carpus / fetlock height | 1.05–1.10 / 0.86–0.90 / 0.45–0.50 / 0.18–0.20 m | 1.10 / 0.86 / 0.485 / 0.18 m | unchanged |
| Femur / tibia / hind cannon with hock / pastern | 0.42–0.46 / 0.40–0.44 / 0.34–0.37 / 0.14 m | 0.420 / 0.434 / 0.356 / 0.14 m | unchanged |
| Hip joint / stifle / hock height | 1.25–1.32 / 0.93–1.00 / 0.54–0.58 m | 1.31 / 0.93 / 0.54 m | unchanged |
| Neck top line, withers to poll; head | 0.85–0.95 m; 0.58–0.65 m | 0.88 m; 0.58 m | unchanged |
| Cervical chord C7 to poll (rig) | 0.75–0.85 m | 0.98 m | unchanged, see "remaining" |
| Clavicle | none in the horse: the trunk hangs between the shoulder blades in the serratus sling | none | none (stated in `decisions`) |

Joint order was already right (scapula–humerus–radius–carpus/cannon–pastern–hoof; pelvis–femur–tibia–hock/cannon–pastern–hoof). The only rig change: joint limits of three fore bones differed between near and far side by one degree; both sides now carry the wider pair.

### Defects seen at full size before the cut

Body 1:50: withers a wedge on the back and croup 4 cm above it; trunk a sausage without breast; shoulder a thin plate buried in the ribs; upper arm a horizontal bolster with a flat cap sticking out in front of the chest; breast lumps as two buns; rump pointed in plan; tail in four pieces with a step at every joint. 1:500: upper foreleg a capped tube, trunk 15 mm too narrow. Structure: under the new connectivity and per-scale feature rules 28 + 35 findings (thoracic vertebrae 1.5–2 mm apart, dock 9 mm off the sacrum, ear cartilages 45–68 mm off the skull, orbits off the skull, hip joint marker and pelvis halves not joined to the spine at 1:100, 1:200, 1:500, shoulder blades 19 mm off the ribs at 1:500, 28 members under 75 mm at 1:500); cervical spine a straight rod. 2D: drew the old croup-high back.

### Rebuilt

- **Body, all four scales, own tables per scale** (23 / 14 / 10 / 7 trunk sections, 10 / 7 / 5 / 4 withers sections): trunk as one loft of upright sections from the points of the buttock to the breastbone; withers as a ridge that runs on under the crest; breast mass between the points of the shoulder (rigid on the thorax now); pectorals (1:50, 1:100); first neck piece starting inside the chest; shoulder widened towards the triceps and set 20 mm proud of the ribs; upper arm with a domed point of the shoulder; tail pieces sharing their end sections (1:50 re-cut, coarser scales: step sections removed and starts matched). 1:500: arm table, broader breast that takes the arm caps, hooves 6 mm longer for the feature.
- **Thermal**: follows by reference at every scale; cut sheet at 1:50 read, regions are the same object.
- **Structure**: cervical vertebrae C7–C1 on the S of the neck (1:50, 1:100), tuber sacrale beside the midline on the sacrum, ischia 0.20 m apart inside the rump, pubis under the hip sockets (added at 1:100), dock continuing the sacrum, ear cartilages standing on the skull, orbits on the skull, thoracic vertebrae touching; 1:500: every member at least 75 mm, ilium wings joining the hip bones across the sacrum, shoulder blades on the ribs.
- **2D**: trunk, withers, breast, first neck piece, shoulder and upper arm redrawn in elevation, front and plan at all four scales from the new body; withers shape moved under the trunk, plan trunk tone set to the coat tone.
- **Decisions**: one dated entry per scale, English and German.

### Validation and what was read

`r3-validate.ts animals/horse --volumes` → `VALID … schema 0, validateAsset 0, checkVolumes 0`; published with `volumes-insert.py` three times; `r3-validate.ts animals/horse --catalog-only --volumes` → `VALID`.

Read after the cuts: 1:50 body six views (twice), three-quarter and front close-ups, grazing-head-down, walk-cycle t = 0.3 s, 1:50 structure six views, 1:50 thermal cut, 1:200 body, 1:500 body (before the last breast change) and 1:500 structure, 2D sheets 1:50 and 1:100. **Rendered but not read after the last change:** standing-rest, walk-cycle t = 0.7 s, graze-and-swish t = 1 s, 1:100 body, 1:100 and 1:200 structure, 1:100 / 1:200 / 1:500 thermal, 1:500 body after the broader breast, 2D 1:200 and 1:500.

### Remaining (next pass)

- Rig: shoulder girdle (scapulae at their true breadth) and pelvis halves on the lateral rest placement once `rig-lateral-rest.md` exists; cervical chord 0.98 m against 0.75–0.85 m (C7 joint sits low and far back, to be settled together with the girdle); poses and clips then re-bound.
- 1:50 body: grazing pose opens a notch between withers ridge and neck crest (first neck piece is rigid on `neck-1`, needs a weighted bridge); upper arm still reads as a separate bolster under the shoulder from three-quarter front; pectorals read as cushions; mane fall is angular plates; hock tendon a thin stick, ergots and coronet as jagged slivers; head, ears, lower legs, thigh and gaskin not re-cut in this pass (only looked at).
- 1:100 / 1:200 / 1:500 body: tail still shows steps at the joints in the default pose; 1:500 ears are capped cylinders, upper hind leg a wedge; neck, head and legs at the coarser scales not re-cut.
- Structure 1:50: skull is one wedge loft without cranium / facial crest / ramus, pelvis is rods rather than ilium wings and ischial plates, ribs are separate hoops (18, slanted) whose sternal ends were not checked one by one, `tail-hair-core-1/2` are members of hair, not bone. 1:200 / 1:500: cervical spine three straight capsules; 1:500 ribcage is two hoops and the leg bones end above the hooves (no coffin bone member).
- 2D: the redrawn shapes are ruler outlines (18–45 points), not yet drawn economically by hand; plan width label reads 0.61 m against `dimensions.depthM` 0.6 (within the rule).

### Fundamental issues (outside this asset)

- `validator.md` has no section "Rule changes authors must know" (the brief of this pass points to it).
- `rework-07-2d.py` runs `main()` on import (no guard), so its `splice` helper cannot be imported; `rework-p1-horse-2d.py` executes its head instead.
- `S/README.md` line 133 states loft `rx` along w and `ry` along v; with the limb and trunk rotations used in the catalog `rx` is the sagittal and `ry` the lateral half extent (already noted in `rework/07.md`).
- A loft section is a symmetric superellipse, so an egg-shaped barrel or a withers ridge needs a second solid and leaves a seam; and loft ends are flat caps that cannot go below the feature at 1:500, so a domed end is impossible there.

## Pass 2 (second round), 2026-11-22

Status: **partial**. The girdles are on the lateral rest placement (`rig-lateral-rest.md` existed at the start and was followed); source and catalog agree; `r3-validate.ts animals/horse --catalog-only --volumes` → `VALID … schema 0, validateAsset 0, checkVolumes 0`.

Scripts (ticket folder): `rework-p2-horse-rig.py` (rig, re-binding, pelvis), `rework-p2-horse-detail.py` (skull, nuchal bridge), `rework-p2-horse-coarse.py` (1:500 ears), `rework-p2-horse-decisions.py`.

### Rig (stage 1, published)

| Joint | Breadth beside the midline, real | Before | Now |
|---|---|---|---|
| Scapular cartilage (top of the blade) | 0.07–0.10 m | 0 (bone in the median plane, solids pushed out by a local offset) | 0.09 m |
| Shoulder joint | 0.17–0.20 m | 0 | 0.19 m |
| Elbow / carpus / fore fetlock / fore hoof | 0.18 / 0.15 / 0.14 / 0.13–0.15 m | 0 | 0.186 / 0.156 / 0.144 / 0.142 m |
| Sacroiliac joint | 0.05–0.07 m | no bone | 0.06 m |
| Hip joint | 0.12–0.15 m | 0 | 0.15 m |
| Stifle / hock / hind fetlock / hind hoof | 0.19 / 0.15 / 0.15 / 0.14–0.15 m | 0 | 0.19 / 0.155 / 0.147 / 0.146 m |

- New bones `hip-bone-hn`, `hip-bone-hf` (0.221 m, sacroiliac joint to hip joint, 24° out of the median plane, joint ±3°); the thighs hang from their tips. The scapulae rest 11.7° out of the plane and are 0.493 m long (0.483 m in the side view as before). No clavicle (the horse has none).
- Every limb bone now runs joint to joint in 3D (rest `abductionDeg` / `twistDeg` solved from the joint points, mirrored exactly near/far); sagittal projection of every bone is unchanged, so the side elevation is the same drawing.
- Re-bound: all body, thermal and structure solids of the 22 limb bones at four scales (rigid transform old frame → new frame, world position unchanged); elevation shapes of these bones (u scaled by the cosine of the tilt); mass centres; anchors; every pose and every clip key re-solved so each limb bone keeps the sagittal direction it had (largest change of an offset 0.97°, no joint limit touched). Front and plan drawings are world-rest drawings and did not move.
- First try with the hip joint at 0.14 m gave `lateral-ambiguous` on the thigh shapes (plane 6.8° against the paper, 0.21 mm on paper); the hip joint is at 0.15 m, where the femoral head of the skeleton already was.
- Read after the change: 1:50 structure six views and three-quarter, walk-cycle t = 0.7 s, grazing-head-down, standing-rest, graze-and-swish t = 1 s, default pose, 2D sheet of walk-phase-3: limbs leave from the girdle tips, hooves on the ground, all read naturally.

### Structure (stage 2, published)

- Pelvis 1:50 / 1:100 / 1:200: rods replaced per side by an ilium wing (loft from the crest between tuber sacrale and tuber coxae down to the shaft and the hip socket), an ischial plate back to the seat bone, the iliac crest and a sacroiliac bridge onto the sacrum, all on the hip bone; pubis and ischial arch stay on the pelvis. 1:500: the hip-bone bar re-bound to the new bone.
- Skull 1:50: one wedge replaced by brain case, deep face narrowing to the incisors (nine sections), nuchal crest, zygomatic arches and facial crests. The first cut was too shallow (read in the side view), deepened to 0.17 m at the orbit.

### Body (stages 1 and 3, published)

- 1:50: `nuchal-bridge` (rounded mass on the thorax under the front of the withers) closes the notch between withers and crest in the grazing poses (read from the left in grazing-head-down).
- 1:500: ears are pointed buds (ellipsoids of 75 mm) instead of capped tubes. Spheres at the tail joints were tried and dropped: no visible gain and more triangles than 1:200 (`volumes-monotonic`).

### Read this pass

1:50 body default, grazing-head-down (sheet and left close-up), standing-rest, walk-cycle t = 0.7 s, graze-and-swish t = 1 s; 1:50 structure (six views, left, three-quarter twice); 1:100 body; 1:500 body (twice); 1:200 and 1:500 structure; 1:100 thermal cut; 2D 1:50 walk-phase-3. **Not read:** 1:200 body, 1:100 structure, thermal 1:200 / 1:500, 2D at 1:100 / 1:200 / 1:500, walk-cycle t = 0.3 s after the last change.

### Remaining (next pass)

- Rig: cervical chord C7 to poll still 0.98 m against 0.75–0.85 m (needs the neck, head, mane and their 2D re-cut at all scales; not started). Hooves in the re-solved poses were checked by eye and by the validator only, not measured to the millimetre.
- 1:50 body: neck pieces step at the neck-1 / neck-2 joint when the neck is lowered; mane crest is a flat board that overhangs the poll in standing-rest and steps in grazing; mane fall angular plates; upper arm still a bolster from three-quarter front; pectorals cushions; hock tendon a thin stick; ergots and coronets slivers; head, ears, forearm, cannon, pastern, hoof, thigh, gaskin not re-cut.
- 1:100 / 1:200 / 1:500 body: tail steps at its joints; neck pieces 2–3, head and legs not re-cut; 1:500 upper hind leg a wedge.
- 1:50 structure: ribs are 18 slanted bars whose sternal ends were not checked one by one; `tail-hair-core-1/2` are members of hair (kept because `volumes-structure-coverage` asks a member for bones of 0.22–0.24 m; the caudal vertebrae end with the dock); cervical vertebrae are thick capsules; skull without orbit rims, nasal notch and teeth; femoral head sits 10 mm lateral of where a head on a neck would be.
- 1:100 structure: skull still the old wedge. 1:200 / 1:500 structure: cervical spine three straight capsules; 1:500 ribcage two hoops, leg bones end above the hooves (no coffin-bone member), ear members are tubes.
- Thermal: follows the body by reference; only the 1:100 cut was read this pass.
- 2D: redrawn shapes of pass 1 are still ruler outlines; 2D of tail, head and legs untouched; 2D not re-checked with `r3-project2d --check` after the rig change (elevation is unchanged by construction, front and plan were not touched).

### Fundamental issues (outside this asset)

- Lofts cannot be bound to two bones (no section weights in the schema), so a neck, tail or mane over several bones always steps at the joints in a pose; the nuchal bridge is a workaround with a rigid mass.
- `volumes-monotonic` counts triangles, and an ellipsoid costs several times a three-section loft, so rounding a coarse scale with ellipsoids is punished.
- `rig-lateral-compare.ts` prints its source instead of a usage line when called without arguments.
