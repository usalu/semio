# `humans/running-child` — rework report

## Pass 1 (second round), 2026-11-22

Status: **partial**. Published; `r3-validate.ts humans/running-child --volumes` ended `VALID … schema 0, validateAsset 0, checkVolumes 0` on the source merged over the catalog, and `--catalog-only` ended `VALID` after the last `volumes-insert.py`.

State found: source `volumes/humans/running-child.json` and catalog were identical (author 02 had published body, thermal and structure from `rework-02-running-child-body.py` / `-structure.py` at 01:35 but written no report). `rig-lateral-rest.md` existed at the start, so the girdles were built.

### Scripts (ticket folder, run in this order)

| File | What it states |
|---|---|
| `rework-02-running-child-body.py` (author 02, edited) | body and thermal from hand-chosen stations; I changed the sleeve cap and one shirt row (shoulder) |
| `rework-02-running-child-structure.py` (author 02, untouched) | skeleton from the shared `rework-02-skeleton.py`; it still writes the old girdle stubs on `torso` / `pelvis` |
| `rework-running-child-girdles.py` (new) | rig bones `clavicle-l/-r`, `ilium-l/-r`, re-parenting of arms and thighs, shrug of the jump poses and clip keys, all girdle solids of the four scales; idempotent, `--apply` writes source and `rig` / `poses` / `clips` of the catalog |
| `rework-running-child-skull.py` (new) | skull at 1:50 and 1:100, true shafts at 1:200 and 1:500, shoulder of the 2D drawings, structure texts |

### Reference (six-year-old, 1.125 m barefoot, 1.14 m in shoes, 20.7 kg plus 1.4 kg pack; Snyder 1977 child anthropometry, DIN 33402-2, Jensen 1986 as cited in the rig)

| Measure | Reality | Model before | Model now |
|---|---|---|---|
| Stature in shoes (rest pose, crown) | 1.14 m | 1.137 m | unchanged |
| Hip joint height / half distance of the hip joints | 0.57 m / 0.053 m | 0.5745 / 0.055 | unchanged |
| Knee and ankle joint height | 0.32 / 0.06 m | 0.318 / 0.056 | unchanged |
| Thigh, shank (joint to joint) | 0.255, 0.26 m | 0.2565, 0.262 | unchanged |
| Shoulder joint height / half distance | 0.87 m / 0.105 m | 0.870 / 0.105 | unchanged |
| Upper arm, forearm, hand | 0.19, 0.15, 0.125 m | 0.1848, 0.1456, 0.12 | unchanged |
| Head height / breadth / length | 0.185 / 0.14 / 0.178 m | 0.188 / 0.14 / 0.178 | unchanged |
| Biacromial breadth | 0.25 m | skeleton 0.19 m (adult skeleton scaled by 0.64), acromial end 21 mm below the sternal end | 0.24 m over the acromion tips, acromioclavicular joints at ±0.102 m, clavicle 99 mm, nearly level |
| Acromion above the shoulder joint centre | 30 mm | body shoulder cap ended 24 mm above the joint | cap 38 mm above the joint, acromion top 31 mm |
| Bicristal / inter-ASIS / bi-ischial breadth | 0.18 / 0.16 / 0.075 m | 0.185 / 0.17 / 0.074, crest 100 mm above the hip joint | 0.177 / 0.17 / 0.072, crest 86 mm above the hip joint |
| Girths: upper thigh, knee, calf, ankle, upper arm, forearm, wrist, neck | 0.33, 0.24, 0.235, 0.16, 0.175, 0.17, 0.115, 0.26 m | 0.333, 0.242, 0.242, 0.16, 0.18, 0.173, 0.11, 0.257 (from the station rows) | unchanged |
| Shaft diameters femur, tibia, humerus | 18, 17, 15 mm | 1:200 31 mm, 1:500 75 mm | true shaft with `slender: true` at 1:200 and 1:500 |

Segment lengths, joint order, symmetry and joint positions of the sagittal chain agreed with the table, so they were kept. The limbs look slim in the sheets; the girths are those of the table.

### Defects found

- Rig had no girdles: arms hung from `torso`, thighs from `pelvis`.
- Shoulder skeleton too narrow and drooping (see table); the acromion of the true breadth stood 11.6 mm outside the body in the front view (`volumes-structure-coverage`), because the body's shoulder cap was too low.
- Skull: cranium ball, a small face ball and four jaw sticks; no orbits, no cheek bones, jaw not closed at the chin.
- 1:200 and 1:500: femur, tibia and humerus thickened to the feature (rule change 2 of `validator.md`).
- Pack strap ends of the structure sat on the old straight clavicle.

### Rebuilt

- **Rig**: `clavicle-l/-r` under `torso` (sternoclavicular joint at up 0.260, front 0.037, ±0.013 of the torso frame to the acromioclavicular joint at up 0.254, back 0.006, ±0.102; `angleDeg` lifts the shoulder, limits -8..30) and `ilium-l/-r` under `pelvis` (sacroiliac joint to hip joint, 69 mm, limits -4..4). `arm-*` and `thigh-*` re-parented without moving (checked with `rig-rest.ts`: origins, u, v, w identical to before). `arm-*` `maxDeg` 170 to 180 (a re-solved clip key reached 174).
- **Poses, clips**: `jump-takeoff` lifts the clavicles 15 degrees, `jump-apex` 12 degrees; arm offsets re-solved so the arms keep their direction; the `standing-jump` keys that inherit these poses re-solved the same way. All other poses and `sprint-cycle` are unchanged in content.
- **Body**: sleeve cap raised 14 mm and the shirt 8 mm broader per side at the shoulder line (all four scales, thermal rebuilt over it by the body script).
- **Structure 1:50**: girdles authored on their own bones: clavicle as S-curve of five bars, scapula blade wrapped 27 degrees round the ribcage with spine, acromion, neck, glenoid, coracoid; ilium blade, crest in four bars from posterior to anterior superior spine, sacroiliac bar, acetabulum, pubis, ischium, ischiopubic ramus; joints at both girdle pivots. Skull: cranium, frontal bulge, flat base, face block narrowing to the nasal bridge, brows, lateral orbital rims, cheek bones, zygomatic arches, temporal bars, mandible as one arch closed at the chin. Strap ends moved onto the new clavicle.
- **Structure 1:100 / 1:200 / 1:500**: the same girdle bones in 15 mm bars and blades / one clavicle, scapula strut, shoulder strut and hip strut per side; skull re-authored at 1:100 only; true shafts at 1:200 and 1:500.
- **2D**: sleeve shapes of all three views at all four scales and the shirt front at 1:50, 1:100, 1:200 redrawn by hand for the new shoulder.
- **Decisions**: structure texts of the four scales rewritten (en, de).

### Rendered and read after the changes

1:50 structure (`standing-ready` front, back, two three-quarter views; `toe-off` from behind), 1:50 body `jump-takeoff`, `crouch`, `standing-jump` at 1.0 s, 1:50 thermal cut, 1:50 2D `toe-off`, 1:100 structure, 1:200 structure `jump-takeoff`, 1:500 structure, 1:500 body. Read before the changes only: 1:50 body and structure `toe-off`.

### Open (for the next pass)

- Not rendered or not read after the last change: body 1:50 in `standing-ready`, `mid-stride`, `airborne`, `landing`, `jump-apex`, `flight-1..3`; both clips beyond one instant (`sprint-cycle` not read at all); body and thermal at 1:100 and 1:200; thermal at 1:500; structure 1:200 in the default pose; 2D at 1:100, 1:200, 1:500 and in any pose other than `toe-off`; close-ups of hands, face, feet.
- 1:50 and 1:100 structure outside girdles and skull is still author 02's shared skeleton (`rework-02-skeleton.py`, an adult layout scaled by 0.64 with a 0.66 chest): ribs as four straight bars each, column as one tube with processes, limb bones, hands and feet not checked bone by bone against a child.
- 1:500 structure reads badly: thin true shafts for femur, tibia and humerus beside 75 mm lumps for forearm, hand, foot, girdle struts and joints. It follows the rule, I would not sign it.
- 1:200 and 1:500 skull still cranium plus one jaw bar.
- Body: only the shoulder was changed; the rest is author 02's stations, checked against the table by number, not re-sculpted. Swung back (`crouch`), the sleeves run into the sides of the 0.19 m wide pack.
- Pack structure is one 0.22 x 0.15 m panel with two straight straps that cut through the shoulder instead of running over it.
- `sprint-cycle` has no clavicle or hip-bone motion; no pose uses `ilium-*`.
- `rework-02-running-child-structure.py` still writes the old stubs; after it, `rework-running-child-girdles.py --apply` and `rework-running-child-skull.py` must run again.
- `--volumes` was last run before the final `volumes-insert.py` on source over catalog; after publishing only `--catalog-only` (without the measured rules) was run. The spliced volumes are that same source.

### Fundamental issues

None found in kernel, validator or tools. `rig-lateral-rest.md` section 1.6 does not say that a re-solved clip key can exceed the limb's joint limit after a shrug (here 174 against 170 degrees).
