# humans/elderly-with-cane

- 2026-11-22 12:2x, rig verification (not the volume author): `poses` and `clips` re-solved with `rig-reoffset.ts humans/elderly-with-cane --write` because the rig now turns a pose offset about the bone's own rest axes instead of the parent's joint axis. hand-r (rest abduction -50) carries the cane: its tip had left the ground by up to 62 mm and moved by up to 260 mm in every pose and in both clips; re-solved, the tip stands on the ground again (y 0.000 in all poses except the swing of passing-right) and validateAsset went from 10 findings to 2. Every bone keeps the posed frame it had before the rig change (largest difference of a bone tip after the re-solve under 0.01 mm). Only `asset.poses` and `asset.clips` were written; rig, shapes, volumes untouched. Do not run the tool on this asset again (`rig-reoffset.done.json`); new poses are authored about the bone's own axes (`rig-lateral-rest.md` 1.1).

## Pass 1 (second round), 2026-11-22

Script (the chisel, hand-chosen numbers stated at its top): `rework-elderly-with-cane-girdle.py` (candidate without argument, `--apply` writes `rig`, `representations`, `volumes` of the catalog file and the volumes source). The scripts `rework-04-elderly-with-cane-*.ts` of the first round still describe the old rig (arms on `chest`, thighs on `pelvis`, poses before the re-solve of 12:2x): do not run that build again, it would undo this pass.

### State found

- Source `volumes/humans/elderly-with-cane.json` and catalog were identical. Reference table: `rework/04.md` section 1.1 (man of 74, 1.65 m; Drillis and Contini / Winter segment lengths, DIN 33402-2 breadths); the rig measured against it again: thigh 0.405, shank 0.395 + ankle and shoe, upper arm 0.307, forearm 0.242, hand 0.09 + fingers, shoulder joints 0.36 m apart, hip joints 0.18 m apart, both sides alike, fifteen finger bones per hand. No deviation in the sagittal chain.
- Defects found: (1) arms were children of `chest`, thighs of `pelvis`; the clavicles carried nothing and their joint axis pointed forward-down, so `angleDeg` did not lift the shoulder; no pelvis halves. (2) Structure 1:100 to 1:500: long bones thickened to the feature (1:500 every shaft 75 mm), radius and fibula dropped from 1:200 (validator item 2 names this asset). (3) The catalog file was INVALID with two `lateral-ambiguous` findings: flat front and plan shapes of `cane-shaft`, `cane-ferrule` and `hand-r` on bones that rest out of the paper and are turned by poses.

### Rebuilt

- **Rig, 52 to 54 bones.** `clavicle-l|r`: same joint and tip (sternoclavicular joint [0.146, 1.3256, ±0.025], acromion [0.0961, 1.3656, ±0.170], 0.1585 m), new `twistDeg ±138.83` so that the normal points up and the joint axis back (left) / ahead (right): `angleDeg` lifts the shoulder; limits -5 to 25. `arm-l|r` are children of the clavicle at the unchanged glenohumeral centre [0.09, 1.3356, ±0.18]. New `hip-l|r` (os coxae) from the sacroiliac joint [-0.05, 0.9446, ±0.045] to the hip socket [0, 0.8696, ±0.09], 0.1008 m, `angleDeg` hikes the hip, limits ±5; `thigh-l|r` are their children. Every other bone keeps its world rest frame to 3e-5 (checked in the script), so poses, clips, anchors, masses, thermal regions and the 2D of the limbs are valid as they are; no pose or clip key was changed.
- **Structure.** The clavicle solids (clavicle, scapula, scapular spine, joint; all scales) are re-expressed in the new clavicle frame. `ilium`, `ilium-body`, `pubis-ischium` of each side (1:50, 1:100) are bound to the hip bone and are its members; at 1:200 and 1:500 `pelvic-ring` names both hip bones in `members`. Humerus, ulna, radius, femur, tibia and fibula at 1:100, 1:200 and 1:500 are the 1:50 shaft sections with `slender: true`, reaching the joint stations of the coarse list; radius and fibula are back at 1:200 and 1:500 (structure 55 to 59 and 36 to 40 solids).
- **2D.** Front and plan of the cane at all four scales: shaft and ferrule are `revolved` solids about the cane bone, the derby handle (1:50, 1:100) an `extruded` solid of 27 mm; the elevation shaft is split into `cane-shaft` and `cane-handle`. Front and plan of `hand-r` (1:50, 1:100, 1:200): an `extruded` solid with a hand-drawn outline of the back of the hand in the bone plane.
- **Decisions.** One sentence per scale on girdles and long bones (en, de); the 1:500 structure text corrected.

### Proof

- Candidate: `r3-validate.ts --file … --volumes`: `VALID file/elderly-with-cane (file as it is): schema 0, validateAsset 0, checkVolumes 0`. Published with `--apply`; `r3-validate.ts humans/elderly-with-cane --catalog-only --volumes`: `VALID humans/elderly-with-cane (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0`.
- Read at full size after the change: structure 1:50, 1:200, 1:500 (six views), body 1:50 `cane-planted` and `cane-gait` 0.5 s, 2D 1:50 and 1:200. Read before the change (body and thermal lists are untouched by it): body 1:50 default and `resting-on-cane`, body 1:200.
- Rendered and not read: structure 1:100, `cane-gait` 1.3 s, `rest-on-cane` 3 s, 2D 1:100 and 1:500. Not rendered in this pass: thermal sheets of any scale, body 1:100 and 1:500, the six gait key poses singly.

### Still imperfect

- 2D front, 1:50 / 1:100 / 1:200: the right hand is a tilted slab on the cane handle (pose-true, but it no longer shows fingers); the plan hand is a block. A drawing of the gripping hand from several solids, or bound to `forearm-r`, is open.
- 2D plan: the cane shaft is the circle of its section, drawn for a plumb cane; in the default pose the cane leans 5 degrees.
- No pose or clip uses the new girdle joints yet (no shrug, no pelvic drop in `cane-gait`; a real cane gait drops the pelvis on the swing side by about 4 degrees).
- 1:500 structure: hands and feet are blunt blocks and the joint markers are 75 mm spheres beside 22 mm shafts.
- From the first round, unchanged: garments rigid per bone (knee seam, waist seam, trailing thigh through the jacket skirt in the gait), face built from added forms, jacket collar a tube, wrist deviation up to 46 degrees in the swing of the gait.

### Fundamental issues

- `K.splice` of `rig-lateral-standing-adult.py` knows two layouts; `representations` of this asset is in a third (plain two-space `json.dumps`). The script here adds that layout (`spliced`).
- A closed 2D profile whose last edge nearly repeats the first vertex (two points 0.8 mm apart at the collar of the 1:100 cane handle) made every light-solid view of that scale fail with "the union of 8 polygon(s) with area cannot be built" once the shape was an `extruded` solid; the same polygon as a flat shape drew. The polygon union of the depiction is not robust against such a sliver.
