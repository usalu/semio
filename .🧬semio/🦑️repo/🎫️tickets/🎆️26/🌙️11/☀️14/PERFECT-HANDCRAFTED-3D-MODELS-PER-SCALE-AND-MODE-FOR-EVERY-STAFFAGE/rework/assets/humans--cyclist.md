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
