# humans/runner-with-blade-prosthesis — rework report

## Pass 1 (second round), 2026-11-22

Status: **partial**. Catalog and source agree; `r3-validate.ts humans/runner-with-blade-prosthesis --catalog-only --volumes` ends `VALID … schema 0, validateAsset 0, checkVolumes 0`.

### State found

- Source `volumes/humans/runner-with-blade-prosthesis.json` (01:45, author 02, cut off) differed from the catalog (21 Nov) in every list of every scale. Rendered both at 1:50 (body and structure): the source was the better state (hands with fingers, fuller thighs and calves, skeleton of 238 instead of 138 solids). It had 4 findings (`shoe-toe-l` 51.2 mm at 1:500; `shorts` area at 1:50 and 1:100; `face-skin` area at 1:100). The source was kept, corrected and published.
- `rework/02.md` has no section on this asset. `rework-02-runner-with-blade-prosthesis-body.py` / `-structure.py` are the previous author's chisels; they describe the state BEFORE this pass (no girdle bones). Running them again would undo the girdle; the next author must move the frames into them first or leave them alone.

### Reference (male sprinter with a right transtibial amputation; 1.775 m barefoot, 1.80 m in the shoe, about 70 kg; Drillis and Contini ratios, DIN 33402-2 P50 scaled, de Leva joint centres)

| Measure | Reality | Rig / model | Verdict |
|---|---|---|---|
| Stature in shoes, crown | 1.80 m | head tip 1.800 m | right |
| Hip joint height | 0.53 H = 0.94 m | 0.9335 m | right |
| Thigh (hip to knee) | 0.245 H = 0.435 m | 0.435 m | right |
| Shank (knee to ankle), left | 0.246 H = 0.437 m | 0.425 m (ankle at 0.0735 m in the shoe) | right within 12 mm |
| Foot: ankle to ball, forefoot | 0.15 m, 0.08 to 0.09 m | 0.154 m, 0.09 m | right |
| Residual limb with socket, right | 0.15 to 0.20 m below the knee | 0.18 m, then adapter, C blade of two rig bones to the ground at the same leg length | right |
| Shoulder joint height | 0.818 H = 1.47 m | 1.4703 m | right |
| C7 height | 0.86 H = 1.55 m | 1.5467 m | right |
| Jugular notch / sternoclavicular joint | 55 to 60 mm below C7 = 1.49 m | manubrium top was 1.517 m, clavicle start 1.509 m | **40 mm too high, corrected to 1.4887 m** |
| Collarbone length | 0.15 to 0.16 m | none in the rig; skeleton capsule 0.154 m sloping down outwards | **bone added, 0.149 m, 4.5° up, 27° back** |
| Distance of the shoulder joint centres | 0.35 to 0.37 m (biacromial 0.39 m) | 0.36 m | right |
| Distance of the hip joint centres | 0.17 to 0.18 m | 0.18 m | right |
| Hip bone, sacroiliac joint to hip joint | about 0.10 m, 40 mm beside the midline, 80 mm above and 35 mm behind the hip joint | none in the rig | **bone added, 0.0996 m** |
| Upper arm, forearm, hand | 0.186 H, 0.146 H, 0.108 H = 0.330, 0.259, 0.192 m | 0.33, 0.26, 0.19 m, both sides alike | right |
| Head height, neck | 0.23 m, 0.10 m | head bone 0.151 m from the atlas, neck 0.103 m | right |

### Defects found

1. Rig: no shoulder girdle and no pelvic girdle. Arms hung from `torso` and thighs from `pelvis` by a bare `z` offset.
2. Skeleton: sternum top 40 mm too high; collarbone a straight capsule sloping down outwards.
3. Pose `hands-on-hips` (and the clip `recover-breathing` built on it): the hand bones ran from the wrist into the lumbar spine, the hands were inside the trunk in the top and back view.
4. Structure 1:200 and 1:500: femur, tibia, humerus and one "forearm" as 44 to 75 mm rods (validator rule 2 asks for the true shaft with `slender`), pelvis a slab with flat ends, cervical and thoracic spine 75 mm thick at 1:500.
5. The four findings named above.

### Rebuilt

- **Rig** (`rework-runner-with-blade-prosthesis-girdle.py`): new bones `clavicle-l/-r` (child of `torso`, sternoclavicular joint [0.058, 1.4887, ±0.024] to acromioclavicular joint [-0.010, 1.5003, ±0.156], joint −8…35°, twist limits as `standing-adult`) and `ilium-l/-r` (child of `pelvis`, sacroiliac joint [-0.036, 1.0117, ±0.040] to the hip joint centre [0, 0.9335, ±0.090], joint ±4°), on the lateral rest placement of `rig-lateral-rest.md` (solved with the functions of `rig-lateral-standing-adult.py`, right side by the mirror rule). `arm-*` re-parented to the clavicles and `thigh-*` to the hip bones without moving (checked in the script: every old bone within 3e-5 m and 3e-5 of its old frame; `rig-rest.ts` confirms). So all 27 poses, both clips, 2D shapes, anchors, masses and thermal regions of the limbs keep their meaning; nothing else had to be re-bound.
- **Structure, all scales**: collarbone, shoulder blade, scapular spine, scapular neck and glenoid re-expressed in the clavicle frame; acetabulum, iliac crest, ilium, sacroiliac part, pubis, ischium and ischiopubic ramus (1:50, 1:100) and the hip bars (1:200, 1:500) in the hip-bone frame (same world geometry, `member` = the new bone). New joint solids `joint-clavicle-*`, `joint-ilium-*`. 1:50 and 1:100: collarbone re-modelled along its bone as the S-curved bone (7 stations, thick sternal end, flat acromial end), manubrium lowered to the jugular notch.
- **Structure 1:200 and 1:500** (`rework-runner-with-blade-prosthesis-coarse-skeleton.py`): femur as a lofted true shaft over neck and trochanter to the knee, tibia, residual tibia, humerus as true shafts, radius and ulna instead of one forearm rod (all `slender`), pelvis as a basin narrowing to the pubis; 1:200 collarbone as the true rod; 1:500 spine as one true rod from the sacrum to the skull base that is also the member of `neck` (`members`), the separate 75 mm cervical rod removed.
- **Poses** (`rework-runner-with-blade-prosthesis-pose.py`): `hands-on-hips` solved from stated targets (wrist [0, 1.06, ±0.215] on the iliac crest, elbow out and back, hand down and forward along the hip, palm towards the body), collarbones lifted 5°; the inhaling keys of `recover-breathing` lift them to 8°.
- **Fixes** (`rework-runner-with-blade-prosthesis-fix.py`): forefoot of the shoe at 1:500 52 mm; `thermal.regions` `shorts.areaM2` 0.2516 → 0.374 (waist to mid-thigh over both legs, as modelled) and `face-skin.areaM2` 0.1243 → 0.10 (face, ears and neck as modelled).
- **Decisions** (`rework-runner-with-blade-prosthesis-texts.py`): structure decision of every scale extended (en, de).
- Body and thermal solids of the previous author were judged by rendering and kept; 2D drawings were not changed (the body form did not change; the girdle bones carry no 2D shapes).

### Rendered and read in this pass

Body 1:50: default pose (source and catalog state before), `standing-ready`, `hands-on-hips` (before, two iterations after), `blade-midstance`, `flight`, `run-cycle` t 0.2, `recover-breathing` t 1.25. Structure 1:50 `standing-ready` and `hands-on-hips` front (with the new bones); structure 1:100, 1:200 (before the coarse rebuild), 1:500 (before and after). Thermal 1:50 cut. Body 1:500. 2D 1:50 default and `hands-on-hips`.

### Not done / imperfect (for the next pass)

- **Not rendered or not read**: body 1:100 and 1:200; thermal 1:100, 1:200, 1:500; structure 1:200 after the coarse rebuild (only 1:500 was read after it); pose `left-forefoot-strike`; `run-cycle` t 0.5 and `recover-breathing` t 3.1 (rendered, not read); 2D at 1:100, 1:200, 1:500; no close-ups of hands, face, feet, socket hardware.
- Rig: the spine is still one `torso` bone (no lumbar / thoracic segments); the shoulder blade has no bone of its own (it rides on the collarbone).
- Running poses and the 21 `run-*` keys do not use the new girdle (no protraction / retraction of the shoulders against the arm swing, no pelvic drop).
- `hands-on-hips`: contact of palm and fingers with the hip judged from the six-view sheet only; in the 3D front view the hands are largely hidden behind the hip contour.
- 1:500 structure: collarbone, shoulder blade and glenoid remain a 75 mm yoke per side (the 0.149 m collarbone is 1 mm short of the slender length of two features); hands and feet are 75 mm lumps. The first sentences of the 1:200 / 1:500 structure decisions still speak of 30 mm / 75 mm members although the long bones are true shafts now (my appended sentences say so).
- Body 1:50 was not re-sculpted by me; I take the previous author's forms as right from the sheets, without measured girths.
- No vitest suite was run (`🧊️model-skeleton` does not list this asset among its subjects).

### Fundamental issues

- `rig-lateral-standing-adult.py` holds the only reusable frame / solve / splice functions for a girdle conversion, but its `rebind` assumes that old and new bone share their origin; a re-bind to a NEW bone (other origin) had to be written again here (`rebind` in `rework-runner-with-blade-prosthesis-girdle.py`). A shared `rig-rebind` helper would save every human author this step.
- `volumes-slender` length limit of two features shuts out the adult collarbone at 1:500 by a millimetre (0.149 m against 0.150 m), so a true collarbone cannot be shown there.
