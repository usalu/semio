# humans/jogger — running report

Earlier work: `rework/02.md`, section 1 (author 02: body, thermal and 2D of all four scales re-authored from stations with `rework-02-jogger-body.py`, `rework-02-jogger-structure.py`, `rework-02-draw.py`; rig left as it was).

## Pass 1 (second round)

State found: source `volumes/humans/jogger.json` and catalog `asset.volumes` identical; `r3-validate.ts` `VALID`. Sheets read at full size before any change: 1:50 body in `initial-contact-l` and `stretch-stand`, 1:50 structure in `stretch-stand`.

### Reference (lean male runner, 1.795 m barefoot, 1.82 m in shoes, 72 kg; Drillis and Contini, DIN 33402-2 P50, de Leva 1996)

| Measure | Reality | Rig before | Rig now |
|---|---|---|---|
| Hip joint height (shod) | 0.52 H + 0.025 = 0.96 m | 0.962 m | unchanged |
| Thigh / shank | 0.245 H / 0.246 H = 0.440 / 0.442 m | 0.4447 / 0.4343 m | unchanged |
| Ankle height (shod), ankle to ball | 0.095 m, 0.13 to 0.14 m | 0.083 m, 0.134 m | unchanged |
| Hip axis to L5/S1 | 0.09 to 0.10 m up, 0.03 to 0.04 m behind | 0.1034 m up, on the axis | 0.1034 m up, 0.032 m behind |
| Lumbar spine L5/S1 to T12/L1 | 0.18 to 0.19 m (5 x 37 mm) | none (one 0.51 m `torso` through the middle of the chest) | `torso` 0.1925 m on the vertebral bodies |
| Thoracic spine T12/L1 to C7/T1 | 0.29 to 0.31 m (12 x 25 mm) | none | `chest` 0.3175 m |
| C7 height (shod) | 0.863 H + 0.025 = 1.574 m | 1.573 m | unchanged |
| Neck C7 to atlanto-occipital joint | 0.095 to 0.105 m, inclined 8 to 12 degrees forward | 0.095 m, vertical, from the middle of the collar | 0.0963 m, 9.56 degrees forward, from the C7 body |
| Head, atlanto-occipital joint to vertex | 0.15 m | 0.1499 m | unchanged |
| Clavicle, sternoclavicular joint to acromial end | 0.15 to 0.16 m, 15 to 25 degrees backwards | no bone | `clavicle-l/r` 0.1494 m, 27 degrees backwards, 1 degree up, joint 18 mm off the midline |
| Glenohumeral centre | 0.818 H + 0.025 = 1.493 m (Drillis and Contini shoulder), half breadth 0.175 m | 1.508 m, 0.175 m | unchanged (15 mm high, see remaining) |
| Hip bone, sacroiliac joint to hip joint | 0.10 to 0.11 m, hip joints 0.17 to 0.18 m apart | no bone, thighs on the pelvis root, 0.18 m apart | `ilium-l/r` 0.1084 m from [-0.045, 0.085, 0.04] to the hip joint [0, 0, 0.09] |
| Upper arm / forearm / hand | 0.186 H / 0.146 H / 0.108 H = 0.334 / 0.262 / 0.194 m | 0.336 / 0.2792 / 0.195 m | unchanged (forearm 17 mm long, see remaining) |
| Trunk masses (de Leva) | middle trunk 16.33 %, upper trunk 15.96 % | 23.249 kg on one bone | `torso` 11.76 kg, `chest` 11.489 kg, same common centre |

### Defects found

- Rig: one straight `torso` bone from 10 cm above the hip axis to C7 through the middle of the chest; no thoracic segment, no shoulder girdle, no hip bones; the neck bone stood in the middle of the collar; arms and thighs hung on the trunk and the pelvis root. Limb lengths, both sides alike, and joint order were right (table).
- Structure: the vertebral column lay on that axis, 3 to 4 cm ahead of where vertebral bodies are; all 17 vertebrae 29.3 mm high (lumbar too low, thoracic too high); the sacrum ended ahead of the column it carries.
- Foot bone runs horizontally from the ankle at ankle height and the forefoot joint is off its axis (not changed, see remaining). Hand bone runs to the tip of a straight finger while the fingers are curled, and there are no finger bones (both hands alike; not changed).

### Rebuilt

**Rig, stage 1** (`rework-jogger-rig.py`, 19 to 20 bones): `torso` is the lumbar spine (L5/S1 32 mm behind the old axis to T12/L1, 0.1925 m, limits -60..25), new `chest` the thoracic spine (0.3175 m, limits -35..20), `neck` from the C7 body to the atlanto-occipital joint (inclined 9.56 degrees), head frame unchanged. Every solid of body, thermal and structure at all four scales, every 2D shape of the old `torso` and `neck`, the masses, the breathing motion and the arm joints were re-expressed so that the rest placement in the world is unchanged. Poses and all clip keys: the trunk lean is split between lumbar and thoracic spine (60 % lumbar for the running lean of 9 degrees, rising to 70 % for the forward fold: -56 / -24 degrees). `dimensions.widthM` 1.007 to 0.984 m (measured default pose; the lean now turns about the vertebrae).

**Rig, stage 2** (`rework-jogger-girdle.py`, 24 bones, on `rig-lateral-rest.md`): `clavicle-l/r` (child of `chest`, angle 87.73, abduction ±62.90, twist ±177.45) and `ilium-l/r` (child of `pelvis`, angle -152.10, abduction ±27.47, twist ∓76.28) on the lateral rest placement; `upper-arm-l/r` and `thigh-l/r` re-parented onto them without moving (`rig-rest.ts --file`: limb frames identical; `rig-lateral-compare.ts` against the state before: largest move of any 2D shape or anchor over all poses and clip instants 0.012 mm). The placement solver in the script was checked against `rig-rest.ts`. Poses do not move the girdle bones yet.

**Body**: the shirt is cut at T12/L1 into `shirt` (chest) and `shirt-waist` (lumbar, with a 30 mm tongue tucked 3 mm under the chest part so the back does not open when the trunk bends) at 1:50 and 1:100; at 1:200 and 1:500 it stays one solid on the chest (a waist piece is below what those scales resolve; the split produced print slivers at 1:500). All other body solids as author 02 left them.

**Thermal**: shirt fabric and air shell follow the cut (`th-shirt-waist-*`, same region and layers); everything else re-bound only.

**Structure**: vertebral column on the new bones, cut into `sk-lumbar-spine` and `sk-thoracic-spine` meeting at the joint; vertebral bodies 35 mm (lumbar) to 20 mm (upper thoracic) further back; lumbar levels 38.5 mm apart, thoracic 25.8 mm; rib heads follow their vertebra; sacrum 30 mm back under L5; cervical column 28 to 14 mm back in the neck; `sk-clavicle-l/r` are the clavicle bones' members; new `sk-iliac-body-l/r` (sacroiliac joint to socket, the real load path into the femur) at 1:50 and 1:100, `sk-hip-l/r` re-bound to the hip bones at 1:200 and 1:500; joint markers for `chest`, clavicles and hip bones; mass markers for `torso` and `chest`.

**2D**: shapes of the trunk and neck re-bound to the new bones in all 12 drawings; shirt outline cut at the waist joint at 1:50 and 1:100 (elevation, front, plan core).

### Validation

`r3-validate.ts humans/jogger --catalog-only --volumes` after each of the two publishes: `VALID humans/jogger (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0`.

### Read at full size after the changes

After rig stage 1 (before the last two adjustments: fold share 0.7, shirt whole at 1:200/1:500): 1:50 structure right side in `stretch-stand` (column on the bones, behind the ribcage middle), 1:50 body six views in `stretch-hamstring` (hands on the shins, natural fold), 2D 1:50 all three views in line and tone.

**Not read**: every sheet rendered after rig stage 2 and after the last stage-1 adjustments. The image reader of this session stopped answering (tool hook timeout, five attempts); the PNGs are in `🗑️generated/r3-sheet/humans/jogger.*` and `🗑️generated/r3-sheet/file/r3-jogger-girdle.*`. The next pass must open them before anything else.

### Remaining, stated plainly

- Renders after the girdle (1:50 structure front/back, hamstring pose, thermal cut, 1:100/200/500 body, 1:200 structure, 1:100 thermal, run-cycle and stretch instants) rendered but not looked at.
- 2D 1:50 and 1:100, elevation and front, line views: the cut of the shirt at the waist is drawn as an outline across the shirt (a seam that does not exist).
- 1:50 body in `stretch-hamstring`: the rigid shirt hem stands off the back over the seat.
- Rig: foot bone still horizontal at ankle height with the forefoot joint off its axis (wanted: `foot` from the ankle to the ball, 0.1578 m, rest 58.4 degrees, `toe` from its tip at +31.6 degrees; needs the foot solids, anchors and the front/plan `solid` shapes of the shoe redrawn, a rotation cannot carry an extruded front profile). No finger bones (hand bone 0.195 m to a straight fingertip while the fingers are curled). Forearm 17 mm long, shoulder joint 15 mm high against Drillis and Contini. No pose or clip moves clavicles or hip bones (no counter-rotation of shoulders and pelvis in the run cycle).
- Structure: ribs and lower costal margin not re-modelled (rib heads moved with their vertebrae only); scapulae still bound to `chest`, no scapula bone; 1:100, 1:200, 1:500 skeletons only re-bound and shifted, not re-modelled for their feature size in this pass.
- Body, thermal at all scales: not re-judged solid by solid in this pass beyond the trunk cut (author 02's stations stand).
- `decisions` texts of volumes and representations do not yet mention the new trunk bones, the girdle and the shirt cut.
- `rework-02-jogger-body.py` / `-structure.py` / `rework-02-draw.py` still write the old single-`torso` frames: do not re-run them over the published asset without moving their trunk frames first.
- Tool output of this pass is still under `🗑️generated/rework-jogger/` (`base`, `base2` = the documents before stage 1 and 2, needed to re-run the two scripts) and `🗑️generated/r3-sheet`, `r3-2d`.

### Fundamental issues

- The `Read` tool of this agent session timed out on its PreToolUse hook from the middle of the pass on (also `Write`), so no image could be opened afterwards.
- `validator.md` has no section "Rule changes authors must know".
- `rig-rest.ts --solve` refuses a bone that does not exist yet (`no bone "clavicle-l"`), although `rig-lateral-rest.md` 1.2 gives it as the way to place a new lateral bone.
- A 2D shape cut at a joint of one garment always draws its cut edge as an outline; there is no way to mark an edge of a filled shape as undrawn.

## Pass 2 (second round)

State found: catalog and `volumes/humans/jogger.json` identical and `VALID`; `rig-lateral-rest.md` present (girdles of pass 1 already on the lateral rest placement); an unpublished script `rework-jogger-extremities.py` (finger bones, shirt hem on the pelvis) from an interrupted run, checked and taken over. The image reader worked in this pass: every sheet named below was opened.

### Reference, extended (1.795 m man in 25 mm shoes; ANSUR II P50 scaled by stature, Drillis and Contini, de Leva 1996)

| Measure | Reality | Rig before this pass | Rig now |
|---|---|---|---|
| Cervicale (C7) height | 0.865 H + 0.025 = 1.578 m | 1.575 m | unchanged |
| Suprasternal notch / sternoclavicular joint | 1.49 m / 1.50 to 1.51 m | 1.537 m | 1.507 m |
| Acromion (end of the clavicle) | 0.822 H + 0.025 = 1.50 to 1.51 m | 1.540 m | 1.510 m |
| Glenohumeral centre | 35 to 40 mm below the acromion, 1.47 to 1.48 m | 1.510 m | 1.480 m |
| Glenohumeral centres apart | 0.35 to 0.37 m | 0.350 m | unchanged |
| Elbow axis (radiale) | 0.630 H + 0.025 = 1.156 m | 1.174 m | 1.158 m |
| Wrist (stylion) | 0.482 H + 0.025 = 0.890 m | 0.895 m | 0.890 m |
| Upper arm, joint centre to joint centre | 0.31 to 0.33 m (acromion to radiale 0.342 m) | 0.336 m | 0.322 m |
| Forearm, elbow to wrist | 0.262 m (Drillis) to 0.275 m (ANSUR) | 0.2792 m | 0.268 m |
| Hand, wrist to third knuckle / knuckle to fingertip (curled) | 0.09 to 0.10 m / 0.06 m chord | one bone 0.195 m to a straight fingertip | `hand` 0.100 m, `fingers` 0.0589 m |
| Hip joints apart, hip joint height | 0.17 to 0.18 m, 0.96 m | 0.18 m, 0.962 m | unchanged |
| Thigh / shank | 0.440 / 0.442 m | 0.4447 / 0.4343 m | unchanged |
| Chest breadth with shirt | 0.32 to 0.33 m + ease = 0.35 m | 0.372 m | unchanged (see remaining) |
| Sternum, notch to xiphoid | 0.16 to 0.18 m, top at T2/T3 | 0.225 m, top at 1.545 m | 0.19 m, top at 1.51 m |
| Scapula | T2/T3 to T7/T8, 0.15 to 0.16 m high | T1 to T9, 0.175 m | 0.147 m, top at 1.50 m, glenoid at the humeral head |

### Defects found (by looking and measuring)

- The whole shoulder girdle stood 30 to 40 mm high (sternoclavicular joint, clavicle, acromion, glenohumeral joint): square shoulders with the sleeve cap at clavicle height, short neck; the arm was 25 mm long from a joint that was too high (elbow 16 mm high). This is the item "forearm 17 mm long, shoulder 15 mm high" of pass 1, measured again against landmark heights.
- The shirt had a flat top out to 0.16 m half breadth instead of the slope of the trapezius.
- Sternum 45 mm too long at the top, first ribs nearly horizontal; scapula from T1 to T9.
- 1:500 structure was the 1:200 list "at the printable thickness": every rod 75 mm thick (femur, humerus, clavicle, hip strut as sausages). 1:200: femur 44 to 52 mm, clavicle 31 mm, one forearm rod, hip strut 60 mm.
- Hand bone to a straight fingertip, no finger bones; rigid shirt hem lifting off the seat in the forward fold (pass 1 list).

### Rebuilt

**Stage A, `rework-jogger-extremities.py` (published 15:39).** `fingers-l/r` (child of the hand at the knuckle line, 0.0589 m, limits -50..40, flexing about the knuckle line; both hands alike), hand bones end at the knuckles (0.100 m); finger solids and phalanges of all lists re-bound without moving; coarse scales carry the fingers through `members` of the hand member. The shirt below L5/S1 is `shirt-hem` on the pelvis with a domed top inside the lumbar piece (1:50 body and thermal, 1:100 body): in `stretch-hamstring` the hem now lies on the seat and the back stays closed. Decisions texts of all scales state the trunk bones, the girdles, the fingers and the shirt cut.

**Stage B, `rework-jogger-shoulder.py` (published).** Clavicles 30 mm lower (arms ride on them: glenohumeral joint 1.480 m, acromion 1.510 m), upper arm 0.322 m, forearm 0.268 m: bones, masses, every solid of body, thermal and structure at all four scales and all 2D shapes of these bones scaled along the bone below the joint. Shirt (chest piece and collar, body and thermal shells, all scales) with a hand-set trapezius slope (half breadth 0.168 m at 1.504 m falling to 0.100 m at 1.551 m, 35 degrees); front shirt outline of all four drawings follows. Structure 1:50 and 1:100: scapulae re-set (blade T3 to T8, acromion on the clavicle end, glenoid at the humeral head), manubrium ends under the sternoclavicular joints, front ends of ribs 1 to 4 fall to it. The twelve running poses turn the clavicles about their upright normal with the arm swing (0.3 of the upper-arm angle, at most 6 degrees: the shoulder goes back with the arm that swings back); `dimensions.depthM` 0.462 to 0.45 (measured default-pose breadth with the swing).

**Stage C, `rework-jogger-coarse-structure.py` (published).** 1:200: clavicle 12 to 13 mm, femur shaft 32 to 36 mm with neck/head and condyles, tibia 24 to 29 mm with plateau and fibula, humerus 22 to 25 mm with head, radius and ulna as two bones (all `slender: true`), hip strut 36 mm. 1:500: skull with the jaw in one solid, thoracic and cervical column as one slender rod at true section (`members: neck`), lumbar column as the 75 mm envelope of vertebrae with processes, ribcage carries the clavicles and the pelvic basin the hip bones (`members`), femur on its mechanical axis, tibia, humerus, radius and ulna at their true shafts (`slender`), one scapula strut to the glenoid, one foot member (heel to ball, `members: toe`), fibula left out. Decisions texts of the structure at 1:200 and 1:500 rewritten for what is modelled.

Tools of this pass: `rework-jogger-contact.ts` (contact sheet of several `r3-sheet` / `r3-2d` renders, own PNG codec, because the host has no image library).

### Validation

`r3-validate.ts humans/jogger --catalog-only --volumes` after stage C: `VALID humans/jogger (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0`; the same line for each candidate before its publish (`--file … --volumes`).

### Rendered and read after the last change of what they show

- 1:50 body, six views: `initial-contact-l`, `loading-l`, `propulsion-l`, `initial-contact-r`, `midstance-r`, `toe-off-r`, `flight-r`, `stretch-stand`, `stretch-quad-r`, `stretch-hamstring` (before stage B; the trunk fold is unchanged since) and clip instants `run-cycle` 0.12 / 0.24 / 0.30 / 0.51 s, `stretch` 4 / 9 s. Not rendered as poses of their own: `midstance-l`, `toe-off-l`, `flight-l`, `loading-r`, `propulsion-r` (the clip instants 0.12, 0.24, 0.30 s are the first three; the last two only through their mirror poses).
- 1:50 structure: front, back, left, top, three-quarter; 1:100 structure; 1:200 and 1:500 structure after stage C (default, `stretch-quad-r`, `stretch-hamstring`).
- Thermal: 1:50 cut, 1:100, 1:200 cut, 1:500 (before stage C, which does not touch thermal).
- 1:100, 1:200, 1:500 body six views.
- 2D: all three views, line and tone, at 1:50, 1:100, 1:200, 1:500 in the default pose, 1:50 in `stretch-hamstring` and `stretch-stand`.

### Remaining, stated plainly

- Rig, feet: `foot` is still a horizontal bone from the ankle with the forefoot joint below its axis. The pivots are right (talocrural axis, metatarsophalangeal line) and every pose is unchanged by the bone's direction, but the bone line is not the anatomical one. Not changed because the plan and front shoe shapes are `solid` extrusions with `sweep: "v"`, which follow the bone normal: on a bone that runs 31.6 degrees downwards the shoe would be extruded obliquely (see fundamental).
- 2D 1:50 and 1:100, elevation and front, line views: the cut of the shirt at the waist joint is still drawn as a line across the shirt (see fundamental). The 2D shirt of 1:50 and 1:100 has no separate hem piece on the pelvis (3D has it); in the 2D forward fold the lumbar piece carries the hem.
- Pelvis: no pose moves the hip bones (sacroiliac joints move under 2 degrees, so this is right) and the pelvis does not rotate about the vertical or list in the run cycle; a root twist would turn every sagittal drawing out of the paper (`lateral-ambiguous`), so only the shoulders counter-rotate, by at most 6 degrees where 8 to 10 would be real.
- Body, all scales: chest breadth 0.372 m with the shirt (0.35 m expected); arms overlap the trunk at the axilla instead of lying against it. Thigh, calf, forearm stations of author 02 were looked at in every pose and kept, not re-measured girth by girth.
- Structure 1:50: ribs are four-segment capsule chains (only the front of ribs 1 to 4 was re-set), the lower costal margin was not re-modelled; scapulae are on `chest` without a scapula bone (they do not glide when the clavicle swings); iliac blades are a thin loft with crest rods. 1:100: every second rib, otherwise the 1:50 parts; only re-set with stage B, not re-modelled for 15 mm.
- 1:500 structure: hand member is a 75 mm capsule (the hand is 85 by 190 by 30 mm), joint markers are 75 mm spheres and dominate the picture.
- Thermal at all scales: looked at (layers nested, regions follow the body), not re-judged solid by solid; `areaM2` passed the validator after the 4 % shorter arms, values not changed.
- `S/🧪️tests/🧊️model-skeleton` was not run after stage C.
- `rework-02-jogger-body.py`, `rework-02-jogger-structure.py`, `rework-02-draw.py` still write the frames before both rounds; the asset is now defined by the published documents plus the five `rework-jogger-*.py` scripts, which each refuse to run twice.

### Fundamental issues

- A filled 2D shape always draws its whole outline, and two shapes of one region and material on neighbouring bones are not joined: a garment cut at a joint shows a seam line in the line views.
- `solid` `extruded` with `sweep: "v"` takes its direction from the bone normal; there is no way to give a prism a direction of its own, so a bone cannot be re-directed (foot from ankle to ball) without losing its plan-profile parts.
- `r3-sheet.ts --list thermal` draws `of` references (1:100 to 1:500 clothing) in grey instead of the region colour, so the coarse thermal sheets read as mostly grey.
- The host has no image library (no PIL, ImageMagick); contact sheets need a hand-written PNG codec.
- Pass 1 wrote "24 bones"; the rig had 23 before the finger bones and has 25 now.
