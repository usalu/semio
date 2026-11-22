# humans/parent-carrying-toddler — running report

## Pass 1 (second round), 2026-11-22

### State found

- Catalog and `T/volumes/humans/parent-carrying-toddler.json` were identical (the old model of 2026-11-21: one trunk bone, no girdles, no finger bones, toddler held out on a straight forearm, tube limbs).
- Batch author 05 had rebuilt the whole asset in scripts (`rework-05-kit.py`, `rework-05-parent-carrying-toddler-roots.py`, `…-model.py`, `rework-05-draw2d.py`, `rework-05-build.sh`) and was cut off before publishing; the candidate lay in `🗑️generated/r3-parent-carrying-toddler/🔣️.json`. Reference table, first look and defect list of that author: `T/rework/05.md` (they hold for this pass and are not repeated here).
- Rendered and read both states at 1:50 (six views, default pose): the candidate is clearly the better one (lumbar/thorax split, clavicles, finger chains, child seated on the forearm against the chest with the far arm across it, lofted limbs with thigh/calf form, proper toddler proportions). I adopted it as the base.
- Under the tools of this night the candidate was no longer valid: `lateral-ambiguous` on both clavicles (new rule for bones that leave the sagittal plane at rest) and, after a rebuild from the scripts, up to seven `PRINT` findings at 1:500, because the previous author's last drops of sub-feature shapes had been made by hand in the candidate and were in no script.

### Reference (measured on the published model)

| Quantity | Reference | Model |
|---|---|---|
| Parent stature with sole / head height, breadth, depth | 1.72 m / 0.227, 0.155, 0.19 | 1.744 with beanie / 0.224, 0.152, 0.192 |
| Clavicle (SC joint to acromion) | 0.15–0.16 m, rising about 10° | 0.155 m, lateral rest placement `abductionDeg ±80`, tips at z ±0.173 |
| Shoulder joint centres | 0.33–0.35 m apart, 0.04 below the acromion | 0.345 m apart, 0.04 below |
| Hip bone (sacroiliac joint to hip centre) | SI joints about 0.09 apart, 0.075 above and 0.055 behind the hip axis | `hip-bone-l/-r` 0.1037 m from (0.075, 0.055, ±0.045) to the hip centres at z ±0.0909 |
| Upper arm, forearm, hand (metacarpus, fingers) | 0.316, 0.248, 0.185 | 0.316, 0.248, 0.095 + 0.045 + 0.048 |
| Thigh, shank, foot | 0.40, 0.418, 0.21 + forefoot 0.08 | same |
| Girth thigh top / mid / above knee (jeans) | 0.57 / 0.50 / 0.40 | 0.58 / 0.49 / 0.41 |
| Girth calf / lower shank (jeans leg) | 0.37 / 0.25 body, jeans about 0.36 | 0.39 / 0.36 |
| Sleeve upper / elbow / wrist (coat) | 0.40 / 0.33 / 0.25 | 0.42 / 0.33 / 0.25 |
| Coat chest / waist / hip girth | 1.00 / 0.92 / 1.06 | 1.03 / 0.92 / 1.08 |
| Toddler trunk girth, thigh girth in the padded suit | 0.60, 0.36 | 0.63, 0.37 |
| Toddler neck, head | 0.045 visible, head 0.205 of 0.865 | neck bone 0.045, head 0.16 bone + crown |

Bilateral symmetry: every `-l` solid has its `-r` twin in all twelve lists (4 scales x body, thermal, structure): 0 unpaired ids.

### What I changed in this pass

- **Pelvic girdle on the lateral rest placement** (`…-roots.py`: `SACROILIAC`, `HIP_CENTRE`, `turn`, `angles`, `hip_bone`): new bones `hip-bone-l/-r` from the sacroiliac joint down, forward and out to the hip centre (`angleDeg -143.75`, `abductionDeg ±26.27`, joint ±4°); the thighs hang from their tips with solved `angleDeg / abductionDeg / twistDeg`, so their world rest frames are unchanged (checked with `rig-rest.ts`: thigh origin [0, 0, ±0.0909], u down, w left, to 1e-4) and every pose, clip, shape, anchor, region and member on the legs keeps its meaning. Rig now 48 bones.
- **Structure**: `iliac-column-l/-r` (capsule along the hip bone, the load path sacrum → sacroiliac joint → acetabulum) and `acetabulum-l/-r` on the hip bones at 1:50 and 1:100, the column alone (thickened to the feature) at 1:200 and 1:500; joint spheres at the sacroiliac joints come from the rig.
- **Shoulder girdle**: kept as author 05 built it — clavicles are real lateral bones on the rest placement (thorax (0.228, -0.10, ±0.02), 80° out of the sagittal plane), a 40 mm scapula link from the acromion to the glenohumeral centre, upper arm from there. No sagittal stub is involved.
- **2D**: the coat yokes on the clavicles are now `solid` (extruded) shapes at every scale (`rework-05-draw2d.py`, `moving_out_of_plane`), which clears `lateral-ambiguous`.
- **`rework-05-parent-carrying-toddler-finish.py`** (new): the hand-made omissions of the previous author as a script — at 1:100 the two coat buttons (elevation, plan) and the toddler nose (front), at 1:500 boot toes, toddler neck, pompom, and per view the toddler's boots, forearms, mittens and the parent's boot shafts where they are below the printable feature.
- **1:500 body for the print feature**: parent beanie starts 12 mm higher (0.048), toddler hat 8 mm higher (0.06), toddler sleeves 11 mm fuller, so the faces and the sleeve print at 1:300; standing pose left thigh 3° / foot 1° (was 2° / 2°) so the far shank reads behind the near one; hoisting thorax 4°, neck -7° (the previous author's last tuning, which was not in the script).

### Validation

- `r3-validate.ts --file <candidate> --volumes`: `VALID … schema 0, validateAsset 0, checkVolumes 0`.
- Published with `kit.publish` (catalog file in the formatting of `volumes-insert.py`, volumes source written from the same document).
- `r3-validate.ts humans/parent-carrying-toddler --catalog-only --volumes`: `VALID humans/parent-carrying-toddler (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0`.

### What is not done, stated plainly

**From the fourteenth tool call on, the Read tool (and the browser) failed with "PreToolUse hook did not respond", so no image could be read after the first two sheets.** Everything after that was judged by numbers only (rest frames, section girths, symmetry count, the measured volume rules including silhouette, views and connectivity). This is not the standard of the brief. Open for the next pass:

1. Render and read: 1:50 body in all ten poses and two instants of both clips; 1:50 structure and thermal (also `--cut`); 1:100, 1:200, 1:500 in all three lists; `r3-2d` at every scale (in particular the yokes as extruded shapes on the clavicles, and the 1:500 drawings with the omitted shapes). None of these was seen after the last change.
2. All scales, structure: iliac wing, ischium and pubis solids are still bound to `pelvis`; only the iliac column and acetabulum ride on `hip-bone-l/-r`. Re-bind the hip-bone solids (extrude in plane `yz` needs re-expression in the hip-bone frame).
3. Rig, clavicles: no rest `twistDeg`, so a pose `angleDeg` protracts/retracts and `abductionDeg` lifts the shoulder; the guide's convention (`twistDeg ±90`, `angleDeg` = shrug) would need the clavicle-bound solids (`clav`, `clav_row`, `clav_row2` in the model script) re-expressed. The clavicle runs straight sideways and 10° up; a real one also runs about 15–20° back. The scapula is a 40 mm link bone; the blade is a solid on the clavicle bone.
4. 1:50 body, parent: first sheet showed a mask-like face under the beanie and jeans legs that stay 0.36 m in girth down to the boot (no ankle taper visible under the trouser leg); hands and feet could not be judged close up.
5. 1:50 body, toddler: legs stick forward nearly horizontally from the seat in the side view; a relaxed seated child lets the shanks hang (check `toddler-shank` -78/-72 against the render).
6. Decisions texts of `volumes` and the summary do not yet mention the hip bones.
7. Biacromial breadth is about 0.375 m (acromion tips), right for a 1.70 m woman, 15 mm narrow for a mixed-sex median.

## Fundamental issues

- Host: the PreToolUse hook of the Read tool and of the browser tools timed out for the rest of the session ("host client may be unreachable"); Bash kept working. An author cannot look at renders while this lasts.
- `r3-validate` results drift over the night without a change of the asset: a document that was `VALID` at 01:46 failed at 02:20 with `lateral-ambiguous` (intended new rule) and with a `PRINT nominal-feature` of 0.198 mm against 0.2 mm on a shape nobody touched. Print-feature limits that are met to 1 % flip with unrelated tool edits; a tolerance or a stable measure would stop this.
- `rework-05-draw2d.py` is a projection of the solids into the drawing (one outline per solid); the brief asks for economical hand-drawn shapes. Not changed in this pass.
