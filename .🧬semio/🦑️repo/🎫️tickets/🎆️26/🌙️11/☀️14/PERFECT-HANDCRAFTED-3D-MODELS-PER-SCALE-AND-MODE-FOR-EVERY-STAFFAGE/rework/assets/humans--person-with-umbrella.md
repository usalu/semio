# humans/person-with-umbrella — rework report

Edit script (hand-chosen numbers): `T/rework-05-person-with-umbrella-girdles.py` (no argument: candidate under `🗑️generated/rework-person-with-umbrella/cand/`; `--apply`: catalog `rig`, `poses`, `volumes` and the volumes source).

## Pass 1 (second round), 2026-11-22

### State found

- No earlier report of this asset; `rework/05.md` lists it as "not started". Source `T/volumes/humans/person-with-umbrella.json` and catalog `asset.volumes` were identical (both of 21 Nov 20:25), so nothing had to be reconciled.
- The catalog asset was INVALID before this pass: 54 `volumes-feature` findings of `validateAsset`, all in the structure lists of 1:100, 1:200 and 1:500 (the limb skeleton of the coarse scales was the 1:50 parts list).
- Rig found: `pelvis – torso – neck – head`, arms `upper-arm – forearm – hand` hung directly from `torso` 0.17 m beside the plane, legs `thigh – shank – foot – toe` hung directly from `pelvis` 0.088 m beside the plane, umbrella (shaft, 8 ribs, runner, 8 stretchers, 32 pleat faces) on `hand-r`. No shoulder girdle and no pelvis halves. Both sides alike, both hands one bone each.

### Reference table (subject: woman, stature 1.72 m, 70 kg with clothing, trench coat, hair in a bun, stick umbrella)

| Measure | Reference | Model (rig / solids) | Verdict |
| --- | --- | --- | --- |
| Stature | 1.72 m (tall woman, about P90 of DIN 33402-2) | sole to vertex 0.925 + 0.653 + 0.142 = 1.720 m | right |
| Head height chin–vertex | stature / 7.5 = 0.229 m | head loft 0.228 m | right |
| Hip joint height | 0.530 H = 0.912 m (Drillis-Contini) | 0.925 m | right (+13 mm, shoe sole) |
| Thigh (hip–knee) | 0.245 H = 0.421 m | 0.4202 m | right |
| Shank (knee–ankle) | 0.246 H = 0.423 m | 0.4185 m | right |
| Ankle height with shoe | 0.039 H = 0.067 m + 15 mm sole | 0.086 m | right |
| Foot length (shoe) | 0.152 H = 0.261 m + shoe | 0.127 + 0.096 + heel 0.07 = about 0.29 m | right |
| Upper arm (shoulder–elbow) | 0.186 H = 0.320 m | 0.316 m | right |
| Forearm (elbow–wrist) | 0.146 H = 0.251 m | 0.248 m | right |
| Hand (wrist–finger tip) | 0.108 H = 0.186 m | 0.184 m | right |
| Trunk hip joint–C7 | 0.288 H + 0.03 = about 0.53 m | 0.0977 + 0.4495 = 0.547 m | right |
| Neck C7–atlas | 0.10–0.11 m | 0.106 m | right |
| Shoulder joint centres | biacromial 0.36–0.39 m for this stature, joint centre 15–20 mm inside the acromion: ±0.16–0.175 m | ±0.170 m | upper end of the range, kept |
| Hip joint centres | 0.36 × inter-ASIS breadth (0.24 m) = ±0.086 m (Bell) | ±0.088 m | right |
| Clavicle | 0.14–0.16 m, S-shaped, medial two thirds convex forwards, running about 20° back and 10° up | was a straight capsule on the torso bone; now a bone of 0.166 m (wide shoulders) and an S-shaped loft | rebuilt |
| Scapula | triangular blade about 0.15 × 0.10 m on ribs 2–7, 30° to the frontal plane, spine, acromion over the humeral head, glenoid neck, coracoid | was one flat ellipsoid | rebuilt |
| Hip bone | iliac crest arching from ASIS (72 mm above, 46 mm ahead, 32 mm outside the hip centre) to PSIS, acetabulum, pubis to the symphysis, ischium with tuberosity (bi-ischial 0.12 m), ischiopubic ramus around the obturator foramen | was a flat ellipsoid disc with three capsules | rebuilt |
| Sacrum | 0.11 m across the alae, tapering into the coccyx, convex backwards | was a round rod of 56 mm | rebuilt |
| Umbrella | stick umbrella: shaft 0.86–0.90 m with crook handle, 8 ribs of 0.56–0.61 m, open diameter 1.0–1.05 m, canopy rise about 0.2 m | shaft 0.78 m + handle, ribs 0.56 m, 1.03 m across | right |

### Rebuilt in this pass

**Rig (lateral rest placement, `rig-lateral-rest.md`).**

- `clavicle-l/-r` under `torso`: sternoclavicular joint (torso frame x up, y back, z left) `[0.3806, -0.066, ±0.022]` to the acromioclavicular joint `[0.415, 0.010, ±0.165]`, 0.16555 m, running 28° back and 12° up; rest placement solved with `rig-rest.ts --solve` (`angleDeg 65.647, abductionDeg ±59.7417, twistDeg ±152.3444`), joint axis to the back so `angleDeg` lifts the shoulder on both sides; limits −8..35°, twist −40..5° (mirrored on the right).
- `ilium-l/-r` under `pelvis`: sacroiliac joint `[0.075, 0.045, ±0.035]` to the hip joint `[0, 0, ±0.088]`, 0.10187 m (`angleDeg −149.0362, abductionDeg ±31.2143, twistDeg ∓72.7272`), limits ±4°.
- `upper-arm-l/-r` re-parented to the clavicles and `thigh-l/-r` to the hip bones without moving: largest move of any existing bone origin 0.003 mm, largest change of a basis entry 1e-6 (printed by the script, confirmed with `rig-rest.ts --file` on the candidate). Their solids, 2D shapes, anchors, masses, thermal regions, poses and clips therefore keep their meaning unchanged; the five poses name the four new bones at 0°.

(The pass above was cut off after the candidate stage; nothing had been written to the catalog. Continued the same day, same heading.)

### Pass 1 (second round), continued — what was written

Scripts (hand-chosen numbers, run in this order; each is idempotent): `rework-05-person-with-umbrella-girdles.py --apply`, `rework-05-person-with-umbrella-skeleton.py --apply`, `rework-05-person-with-umbrella-head.py --apply`, `rework-05-person-with-umbrella-2d.py` (needs `r3-project2d.ts humans/person-with-umbrella --scale 50 --view elevation --bone head --candidate` first).

**Rig.** Published as described above: `clavicle-l/-r` and `ilium-l/-r` on the lateral rest placement, arms and thighs re-parented without moving (largest move of an existing bone origin 0.003 mm), the five poses name the four new bones at 0 degrees. Segment lengths, joint order and both sides were already right against the reference table and are unchanged; hands stay one bone each (alike on both sides), feet `foot` + `toe`.

**Structure 1:50 (257 solids).**
- Girdles as listed in the script: S-shaped clavicle, scapula (blade, spine, acromion, glenoid neck, coracoid), hip bone (iliac wing, body, acetabulum, pubis, pubic body, ischium, ischiopubic ramus), broad sacrum with coccyx, sternoclavicular and sacroiliac joints.
- Skull rebuilt (was a ball, a face ellipsoid and four wires): neurocranium as a loft narrowed at the orbits (glabella to opisthocranion 0.170 m, breadth 0.132 m, height 0.137 m), maxilla on two frontal processes beside the nasal opening, cheek bones with zygomatic arches, mandible as one arch (body 24 mm high) with two rami in the jaw joints.
- Column rebuilt (was two smooth tubes): 7 cervical, 12 thoracic, 5 lumbar bodies waisted between their end plates along the hand-stated curvature, one spinous process per vertebra (thoracic ones steep, C7 prominent), transverse processes on the lumbar vertebrae.
- Ankle and foot rebuilt (was heel capsule, one wedge, one toe plate): fibula with its head under the lateral tibial condyle and the lateral malleolus in the tibial notch, talus in the mortise between both malleoli, calcaneus, tarsus, five metatarsals, five toes. The long-bone shafts that `model-skeleton` pins were not touched.
- A grip joint ties the umbrella handle to the hand bones (the handle group passed 2.5 mm beside the phalanges without a load path).

**Structure 1:100, 1:200, 1:500.** Girdle bones as written by the girdles script (1:100 clavicle, scapula with spine, acromion and neck, iliac wing, ilium body, acetabulum, pubis, ischium, ramus, sacrum; 1:200 clavicle rod, scapula plate, hip-bone bar; 1:500 clavicle rod), long bones kept at their true shaft with `slender: true`, sub-feature epiphyses dropped, joints at the feature size; cranium of 1:100 and 1:200 is the 1:50 loft in fewer sections. The 54 `volumes-feature` findings the asset had are gone.

**Body and thermal 1:50, head.** The head was a flat mask (the front of every section stood on one vertical plane, the hair hung over the forehead as a flat-topped visor, the nose was a faceted wedge). Re-authored from a profile table per height (front edge, back edge, half breadth): chin prominence, mentolabial fold, lips, upper lip, cheek, hollow at eye level under a brow ridge, forehead, vault; depth 0.186 m, breadth 0.146 m, chin to vertex 0.226 m. Nose as a loft along its dorsum (tip 18 mm ahead of the upper lip), two lips, eyes set into the hollow, brows on the ridge. Hair as a second shell stated the same way: hairline above the forehead, back over the temples, behind the ears, down to the nape, into the bun. `th-head` carries the same geometry, the other thermal head solids follow by reference; head-skin area is inside the 30 % band again.

**2D.** Side elevation 1:50: `head` and `hair` redrawn from the ruled projection of the new solids (elevation p95 was 18.8 mm against 15 mm with the old profile). Front and plan pass `volumes-silhouette` unchanged.

**Decisions.** Structure text of 1:50 rewritten (en, de); the structure texts of 1:100, 1:200 and 1:500 now name the girdle bones, the slender long bones and the shoulder joints.

### Rendered and read

- Before the head change: 1:50 body, structure (six views, front, two three-quarter views), poses `gust-heavy`, `umbrella-furled`, clip `close-umbrella` 3.2 s (structure), 1:100 / 1:200 / 1:500 structure, 1:200 and 1:500 body, 1:50 thermal cut. Poses and the clip read naturally, the skeleton stays inside the body in every one read, the girdle bones run joint to joint.
- Head close-ups (front, left, three-quarter) read after each of the three head iterations except the last (crown rounded with three more sections).
- Rendered but NOT read (the Read tool stopped answering at 16:10): the final 2D sheet 1:50, the four poses and two clip instants after the head change, `umbrella-tilted-to-wind`, `gust-light`, `rain-gust-sway` 2 s / 4 s and `close-umbrella` 2 s at any time, thermal 1:100 / 1:200 / 1:500, body 1:100.

### Validation

`r3-validate.ts humans/person-with-umbrella --volumes`: VALID (schema 0, validateAsset 0, checkVolumes 0) after the last change. Catalog only: `VALID humans/person-with-umbrella (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0`

### What remains imperfect

- 2D front and plan 1:50: `hair` and `head` still draw the old straight hairline and face (silhouette passes, the inner line does not match the new hairline); the pasted side elevation outline is a ruled trace (44 and 14 points), not yet an economical hand drawing with the brow, nose and lip lines.
- Head of 1:100 and 1:200 (body and thermal) still has the flat face plane and the hair visor of the old 1:50; 1:500 is a typified head and fine.
- Body 1:50 not re-modelled in this pass: trouser legs are near-straight tubes (knee and calf barely show), the trunk under the coat is one straight form, ears are flat ellipsoids, the left hand is small against the 0.184 m reference when curled; shoulder joint centres at ±0.170 m are the upper end for a woman of 1.72 m (bideltoid over the coat about 0.48 m).
- Thermal: no hidden thigh solids under the coat skirt at any scale (the trouser region starts at the shank); layers were not re-audited beyond the thermal sheet 1:50.
- Structure: ribs are round rods without costal cartilage distinction; scapula blade is a flat loft; carpus is one form; hands of 1:500 are 75 mm clubs (feature floor); no clavicle motion is used in any pose or clip (shoulder of the umbrella arm does not rise).
- The renders listed above as not read.

### Fundamental issues

- `r3-sheet.ts` has no zoom or crop: a head is 150 px in a 2400 px sheet, too small to judge a face; I cropped with `sips`. A `--crop "x0,y0,x1,y1"` in metres would help every human author.
- A full `r3-validate.ts --volumes` took 7 minutes wall time (92 s CPU) under the load of the other authors.
