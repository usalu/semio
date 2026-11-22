# humans/cyclist — author report

Earlier work on this asset: `rework/06.md`, section 3 (bicycle measured, hip bones and scapulae redrawn as plates at 1:50 to 1:200, rig, body, thermal and 2D untouched).

## Pass 1 (second round), 2026-11-22

### State found

- Source `volumes/humans/cyclist.json` and catalog `asset.volumes` were identical (no half-written edit).
- Rendered and read at full size before any change: 1:50 body and 1:50 structure, default pose `riding-upright`, six views each.
- `rig-lateral-rest.md` existed at the start; the girdles were built on it (below).

### Reference (woman 1.72 m, about 66 kg, softshell jacket, on a step-through city bicycle)

| Measure | Reality (DIN 33402-2 women P50 scaled to 1.72 m, Drillis-Contini, de Leva) | Model as found | Verdict |
|---|---|---|---|
| Thigh (hip joint to knee) | 0.245 H = 0.42 m | 0.404 m | right (within 4 %) |
| Shank (knee to ankle) | 0.246 H = 0.42 m | 0.423 m | right |
| Foot bone ankle to toe tip (foot + toe) | 0.19 to 0.21 m (foot length 0.255 m) | 0.137 + 0.066 = 0.203 m | right |
| Upper arm (shoulder to elbow) | 0.186 H = 0.32 m | 0.319 m | right |
| Forearm (elbow to wrist) | 0.146 H = 0.25 m | 0.251 m | right |
| Hand (wrist to finger tip) | 0.108 H = 0.186 m | 0.184 m | right |
| Hip joint to shoulder joint | 0.288 H = 0.495 m | 0.0966 + 0.4057 = 0.502 m | right |
| Neck base to vertex | 0.27 to 0.30 m | 0.112 + 0.158 = 0.27 m | right, short end |
| Hip joint centres apart | 0.17 to 0.18 m | 0.176 m | right |
| Biacromial breadth | 0.36 to 0.37 m | acromial end of the clavicle at ±0.18 m = 0.36 m, but the joint beside it | see next row |
| Glenohumeral joint centres apart | biacromial minus 2 × 0.02 = 0.32 to 0.33 m | 0.38 m (arm bones at ±0.19 m) | **wrong, 25 mm too wide per side** |
| Bideltoid breadth, jacket on | 0.42 to 0.43 m nude, 0.44 to 0.45 m dressed | jacket 0.396 m at joint height, sleeves to ±0.246 m = 0.49 m | **wrong, 40 mm too wide** |
| Clavicle | 0.14 to 0.15 m, S-curved, sternoclavicular to acromioclavicular joint | a loft on the torso bone, 0.19 m long, 28 mm from the sternum | **wrong: no bone, too long, not attached** |
| Shoulder girdle | clavicle carries scapula and arm | arm hung from the torso bone 0.19 m beside the plane | **wrong topology** |
| Pelvic girdle | hip bone from sacroiliac joint to hip joint carries the thigh | thighs hung from the pelvis bone 0.088 m beside the plane | **wrong topology** |
| Acromion | roofs the humeral head, joins scapular spine and clavicle | none: scapula plate ended 30 mm behind the humeral head | **missing** |
| Bicycle | see `rework/06.md` (wheelbase 1.05 m, 71° head angle, 170 mm cranks, 0.62 m bar) | as measured there | right |

Limb segment lengths, joint order and left/right symmetry of the sagittal chain were right as found (both sides have the same bones, lengths and limits; hands are gloves with four finger lofts and a thumb on one hand bone each, feet have a toe bone).

### Rebuilt (script `rework-cyclist-girdle.py`, hand-chosen numbers at its top)

- **Rig.** New bones `clavicle-l/-r` (parent `torso`, sternoclavicular joint at torso `[0.392, -0.085, ±0.018]`, acromioclavicular joint at `[0.428, -0.018, ±0.157]`, 0.156 m, revolute -8..35°) and `ilium-l/-r` (parent `pelvis`, sacroiliac joint `[0.07, 0.04, ±0.045]` to the hip joint `[0, 0, ±0.088]`, 0.0914 m, ±3°), each with full rest placement (`angleDeg`, `abductionDeg`, `twistDeg`; right side mirrored by the rule). `arm-l/-r` hang from the clavicles with the joint centre at ±0.165 m (was ±0.19 m), `thigh-l/-r` from the hip bones without moving; arm and thigh keep their sagittal rest frame (`rig-rest.ts`: u down, v ahead, w left on all four), so their solids, 2D shapes, anchors, masses and thermal regions stay bound as they were.
- **Poses and clips.** With the shoulder 25 mm further in, both arms of all 4 poses and of the 177 clip keys that state the arms (`pedal-cycle` 160, `brake-and-stop` 17) were re-solved (arm angle, abduction, twist; forearm angle; hand angle and abduction) so that each hand has exactly the frame on the grip it had (residual under 1e-6 m).
- **Body, thermal.** Jacket yoke narrowed by hand per station of the torso (22 mm at the joint height, 10 mm at the armpit, nothing below the bust) in `jacket` at all four scales and in the three thermal shells at 1:50; the coarse thermal lists reference the body solids and follow.
- **Structure.** Clavicles re-authored on their own bones as S-curved lofts with a round sternal and a flat acromial end (per scale: 6 sections at 1:50, 4 at 1:100, bars of feature thickness at 1:200 and 1:500, where there was no clavicle at all); acromion added at 1:50 and 1:100 from the scapular spine over the humeral head to the clavicle; scapulae re-bound to the clavicle bones; ilium, ilium wing, pubis, ischium and ischiopubic ramus re-bound to the hip bones (same place, new frame); a hip-bone member per side at 1:500; joint markers for the four new bones.
- **2D.** Jacket outline of the front and plan drawing drawn in to the new yoke at all four scales.

## Pass 1 (second round), continued 14:25 to 15:10 (second agent of this pass)

### State found

- Source and catalog `volumes` identical; catalog `VALID` (`validate10.log`, 13:57) with the girdles of the section above in place. `rig-lateral-rest.md` existed at the start and at the end. `rig-rest.ts` confirms: `clavicle-l` from [0.497, 1.427, 0.018] to [0.450, 1.484, 0.155], `arm-l` hanging from [0.424, 1.471, 0.165] with u down, v ahead, w left; `ilium-l` from [0.228, 1.076, 0.045] to the hip joint [0.255, 1.000, 0.088], `thigh-l` with the sagittal frame; right side mirrored.
- Read at full size before changing anything: 1:50 body six views, 1:50 structure front, right, back and three-quarter.

### Reference for the skeleton (woman 1.72 m)

| Part | Reality | Model as found | Verdict |
|---|---|---|---|
| Thoracolumbar spine T1 to L5 | 0.27 + 0.17 = 0.44 m, 17 vertebrae; lumbar bodies 45 x 33 mm, 27 mm high plus 7 mm disc; thoracic 28 to 40 mm wide, 20 to 28 mm pitch; bodies 50 to 60 mm ahead of the skin of the back | one smooth rod of 24 to 38 mm, 0.47 m along the right curve | **wrong: no vertebrae, no processes** |
| Cervical spine | 7 vertebrae, 0.105 m, atlas 75 mm wide, C7 spinous 20 mm | one rod of 22 mm lying 10 mm under the nape skin | **wrong** |
| Hand | carpus 45 x 30 mm, metacarpals 45 to 65 mm, phalanges of the middle finger 44 / 26 / 19 mm | one flat slab 85 mm long, no fingers, while the glove has four fingers and a thumb round the grip | **wrong** |
| Foot | calcaneus 75 mm, talus, tarsus, metatarsals 60 to 75 mm, hallux 50 mm | one tapering loft from heel to toe joint and a toe slab | **wrong** |
| Femur | head 44 mm, greater trochanter 45 mm lateral of the head, condyles 75 mm across and 60 mm deep | neck and shaft right, no trochanter, a flattened end instead of two condyles | **incomplete** |
| Skull | cranium 176 x 132 mm, zygomatic arches, mandible | cranium ellipsoid, maxilla, mandible with rami; no arches, no orbits | **incomplete** |
| Coarse scales | long bones keep the true shaft as `slender` members (`validator.md` item 2) | femur, tibia, humerus, forearm and spine as 30 mm (1:200) and 75 mm (1:500) pipes | **wrong since the rule change** |

### Rebuilt (`rework-cyclist-skeleton.py`, hand-chosen numbers at its top, re-runnable on the current source)

- **Structure 1:50** (164 to 282 solids): L5 to T1 as 17 waisted bodies standing on each other along the hand-set curve, each with a spinous process (thoracic 36 mm sloping down, lumbar 30 mm), lumbar transverse processes 84 mm across; C7 to C1 on the neck bone, moved 8 to 14 mm forward into the neck, atlas 52 mm wide, C7 with the long process; per hand carpus, five metacarpals and fourteen phalanges that take their joints from the sections of the glove fingers, so the finger bones lie inside the fingers round the grip on both sides; per foot talus, calcaneus, tarsal block, five metatarsals (first 13 mm, others 9 mm) and five toe rays on the toe bone; greater trochanter and two condyles on each femur; zygomatic arches.
- **Structure 1:100** (144 to 160): the 17 thoracolumbar bodies without processes (below 15 mm); cervical rod, hand and foot plates kept.
- **Structure 1:200 and 1:500**: spine, femur, tibia, humerus, ulna and radius at their true 1:50 section with `slender: true` (forearm bar split into ulna and radius).
- **Decisions**: the structure sentence of every scale rewritten (en, de) to what is modelled.
- Rig, body, thermal, 2D: not changed by this agent.

### Rendered and read after the last change

1:50 structure right, back (default), three-quarter in `dismounted-astride`; 1:500 structure six views; 1:200 structure six views (before the coarse change); 1:50 body in `standing-on-pedals`, `coasting`, `dismounted-astride`, `pedal-cycle` 1 s, `brake-and-stop` 1.4 s, `dismount` 0.8 s; 1:50 thermal cut; 1:500 body; 2D 1:50 (three views, line and tone). All poses and instants read naturally: hands stay on the grips, feet on the pedals or the ground, the finger and foot bones move with the hand and foot.

Rendered but not read: `pedal-cycle` 3 s, `brake-and-stop` 0.5 s, `dismount` 2.2 s, 1:100 body / structure / thermal, 1:200 body / thermal, 1:500 thermal, 1:50 structure in `pedal-cycle` 3 s. Not rendered: 2D at 1:100 to 1:500, thermal in other poses.

### Validation

`r3-validate.ts humans/cyclist --volumes` → `VALID … schema 0, validateAsset 0, checkVolumes 0`; published with `volumes-insert.py`; `r3-validate.ts humans/cyclist --catalog-only --volumes` → `VALID humans/cyclist (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0`.

### Still imperfect (for the next pass)

- Rig: the trunk is one `torso` bone from the lumbosacral joint to C7 (no lumbar / thoracic split); the `neck` bone runs along the nape, 20 to 30 mm behind the vertebral bodies; one `hand` bone per side without finger bones (the curl of the fingers is fixed in the solids); no scapula bone (scapula rides on the clavicle).
- 1:50 body: the neck is 84 mm deep (real about 100 mm), so the cervical processes have 6 to 10 mm of cover; in `standing-on-pedals` and `dismount` the flared jacket hem stands out behind the seat as a point because the jacket is one solid on `torso`; body solids were judged from the sheets, no section was re-modelled in this pass.
- 1:50 structure: skull without orbits and nasal opening (cranium ellipsoid plus maxilla loft); humerus without modelled head and tubercles (the joint marker covers the head); ribs do not carry costal cartilage as separate parts; carpus and tarsus are one block each, not single bones.
- 1:100 structure: hand and foot are plates, cervical spine is a rod. 1:200 / 1:500 structure: the ribcage is two or three open hexagonal loops, the skull at 1:500 a cylinder-like form, joint markers of 75 mm dominate the 1:500 skeleton.
- Thermal: only the 1:50 cut sheet was read in this pass; coarse thermal sheets not read.
- Decisions: the fourth entry ("Thirty-six crossed wires per wheel …") stands at 1:200 and 1:500 although the wheels there have six spokes / one ring.
- Bicycle (from `rework/06.md`): rim flange 11 mm and tyre 8 mm small on the radius against 37-622, chain without arcs round sprocket and chainring, rim-brake arms as rods.

### Fundamental issues

- None new. The 1:200 / 1:500 rib "loops" and 75 mm joint markers follow the feature rule as it stands.
