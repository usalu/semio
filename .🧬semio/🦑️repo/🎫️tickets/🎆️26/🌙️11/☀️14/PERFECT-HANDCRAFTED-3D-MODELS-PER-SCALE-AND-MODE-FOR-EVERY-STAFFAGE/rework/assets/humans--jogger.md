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
