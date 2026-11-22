# humans/kneeling-gardener — rework report

## Pass 1 (second round), 2026-11-22

Status: **partial**. The rig now has the lumbar joint and both girdles on the lateral rest placement, the shirt has a real neckline, catalog and source agree and validate. Finger bones are still missing and not every sheet was read after the last change (list at the end).

### State found

- No report existed. Batch report `rework/03.md` lists the asset as "not started".
- Source `volumes/humans/kneeling-gardener.json` and the catalog `asset.volumes` were identical (both written 15:22).
- An earlier agent of this round had left three scripts without a report: `rework-kneeling-gardener-girdles.py` (clavicles and hip bones on the lateral rest placement of `rig-lateral-rest.md`, limbs re-parented without moving), `rework-kneeling-gardener-head.py` (head hung from the atlanto-occipital joint, 65 mm lower), `rework-kneeling-gardener-fixes.py` (palm, thumbs, weeds, toes). Their result was published and `VALID`; I judged it by rendering (all four scales, three lists, three poses) and kept it.

### Reference (woman, 58 years, 1.66 m, 64 kg; Drillis-Contini ratios, de Leva 1996 female segments, DIN 33402 breadths)

| Measure | Reference | Model (rig / solids) | Verdict |
| --- | --- | --- | --- |
| Thigh, hip to knee | 0.245 H = 0.407 m | 0.4055 m | right |
| Shank, knee to ankle | 0.246 H = 0.408 m | 0.3961 m | right (-3 %) |
| Upper arm | de Leva F 0.275 m, 0.186 H = 0.309 m | 0.2925 m | right |
| Forearm | 0.146 H = 0.242 m, de Leva F 0.264 m | 0.2546 m | right |
| Hand | 0.108 H = 0.179 m | 0.1698 m | right (-5 %) |
| Foot length | 0.152 H = 0.252 m | heel -0.028 to toe tip 0.209 = 0.237 m | 6 % short, kept (size 37/38) |
| Hip joint to C7 | 0.86 H - 0.53 H = 0.55 m | 0.0943 + 0.16 + 0.296 = 0.550 m | right |
| Lumbar column L5/S1 to T12/L1 | 0.16 to 0.18 m | 0.16 m (new bone) | right |
| C7 to atlanto-occipital joint | 0.09 to 0.10 m | 0.09 m | right |
| Chin to vertex | 0.21 m | 0.065 + 0.15 = 0.215 m | right |
| Hip joint centres apart | 0.17 to 0.18 m | 0.18 m (hip bones end at z ±0.09) | right |
| Biacromial breadth | 0.355 to 0.36 m | acromioclavicular joints ±0.158, shoulder joint centres ±0.16 | right for the joint centres; deltoid surface at ±0.21 |
| Head breadth | 0.145 m | 0.145 m (front sheet) | right |
| Trunk masses | de Leva F: pelvis 12.47 %, abdomen 14.65 %, thorax 15.45 % | 7.981, 9.376, 9.888 kg | right (the old `torso` carried abdomen and thorax as one 19.264 kg point) |
| Neck visible in front | chin 20 mm above C7, jugular notch 75 mm below C7 | chin 25 mm above C7; shirt opening was a level ring 7 mm under the chin | **wrong, rebuilt** |
| Spine articulation | pelvis, lumbar, thoracic, cervical | pelvis, one rigid 0.456 m trunk, neck | **wrong, rebuilt** |
| Hand articulation | 15 finger bones per hand (as `standing-adult`, `elderly-with-cane`) | one `hand` bone per side, fingers modelled as fixed solids | **open** |

### Defects found

1. Rig: the trunk was one rigid bone from L5/S1 to C7. A kneeling, weeding figure bent only at the lumbosacral joint, so the back stayed a straight board with a kink at the hem in every pose.
2. Body, all scales: the shirt ended in a level ring at C7 height, 7 mm below the chin. From the front the head sat on the shirt with no neck (front sheet of `kneeling-upright`).
3. Thermal: the shirt layer of the lower trunk has to follow the body split (found by `volumes-thermal-coverage`, 46 % of the shirt area missing in the first candidate, corrected before publishing).
4. Rig: hands have no finger bones (not corrected, see "Remaining").

### Rebuilt

**Rig** (`rework-kneeling-gardener-lumbar.py`): new bone `lumbar` (parent `pelvis`, 0.16 m, revolute -50..20°, 9.376 kg). `torso` is now the rib cage with the thoracic spine (parent `lumbar`, 0.296 m, revolute -45..20°, 9.888 kg, centre of mass 0.1698 m). `neck`, `clavicle-l/-r` ride 0.16 m lower on it; nothing moves in the rest pose (`rig-rest.ts` frames of all limbs unchanged). Girdles are those of the earlier agent, checked against `rig-lateral-rest.md` section 1.3 and 1.4: `clavicle-l/-r` from the sternoclavicular joint [0.050, 1.002, ±0.020] to the acromioclavicular joint [-0.012, 1.022, ±0.158], `hip-bone-l/-r` from the sacroiliac joint [-0.058, 0.565, ±0.040] to the hip joint [0, 0.5, ±0.09]; arms and thighs hang from their tips.

**Poses** (all six, both clips use them): 60 % of the former trunk bend goes to the lumbar joint, the rib cage bends until the shoulders are at their former height, the neck takes the difference back so the gaze stays; both arms are solved again (two-link) so that wrists, hands and trowel stay exactly where they were. `kneeling-weeding`: lumbar -16°, rib cage -16.4°, neck 16.4°. `reaching-trug`: -20°, -23.3°, 17.3°. A first split of 45 % with unchanged total bend lifted the shoulder 39 mm and left the right arm 0.2° from full extension in three poses; rejected by rendering.

**Body, thermal, structure at 1:50, 1:100, 1:200, 1:500**: the shirt loft (and its air layer) is cut at the thoracolumbar joint into `shirt-waist` on `lumbar` and `shirt` on `torso`; the waist runs 40 mm past the joint, the chest 8 mm, each slightly inside the other, so the back rounds and stays closed. Thermal lists gain the `shirt` layer of the waist. `sk-spine` is split into `sk-spine-lumbar` and the thoracic `sk-spine`; spinous processes L5..L1 go to `lumbar`; new `joint-lumbar` (L5/S1), `joint-torso` now at T12/L1; `mass-lumbar` and `mass-torso` at the de Leva centres. Ribs, sternum and every other trunk solid keep their place.

**Neckline** (`rework-kneeling-gardener-neckline.py`, all scales, body and thermal): the upper shirt sections are tilted about the lateral axis up to 30° and lowered up to 12 mm without letting sections cross: the opening lies 36 mm below C7 in front and 18 mm above C7 at the nape; the collar band follows at 30°. The neck loft is carried 40 mm further down into the shirt, the thermal neck 26 mm (the face region stays within its `areaM2`: 31 % over with the first, longer extension, corrected).

**2D** (1:50 and 1:100; 1:200 and 1:500 are declared unavailable for 2D): elevation, front and plan `shirt` cut into `shirt-waist` (on `lumbar`) and `shirt`; `shirt-hem` on `lumbar`, placket in two strokes; elevation `shirt` and `shirt-collar` redrawn from the tilted sections, the collar drawn in front of the neck; front and plan profiles raised to the nape. Anchor `lumbar` bound to the new bone.

**Decisions**: body and structure texts of every scale, en and de (`rework-kneeling-gardener-decisions.py`).

### Validation

- `r3-validate.ts humans/kneeling-gardener --catalog-only --volumes`: `VALID … schema 0, validateAsset 0, checkVolumes 0` after the lumbar stage and again after the neckline stage; re-run after the decision texts (see last line of this report).
- Read after the last geometry change: 1:50 body `kneeling-upright` and `reaching-trug` (six views each), 1:50 structure and thermal (default pose), 1:100 body, 1:200 thermal, 1:500 structure, clip `dig-cycle` 0.2 s, 2D 1:50 `kneeling-upright`. All read as one coherent figure: the back rounds over two joints, hands and trowel are where the poses put them, the neck shows above the sloping neckline. Read after the lumbar stage and before the neckline: 1:50 body `kneeling-weeding`, `reaching-trug`, three-quarter close-up, structure right side, 2D 1:50 default and `reaching-trug`.
- Rendered after the last change but **not read**: thermal and structure of 1:100, body and structure of 1:200, body and thermal of 1:500, poses `dig-up`, `dig-in`, `dig-pull`, clip instants `dig-cycle` 1.1 s and `reach-to-trug` 1.6 s and 3.1 s, 2D 1:100. For these the claim rests on the validator and on the sheets read before the change.

### Remaining (plainly)

- Rig: no finger bones (15 per hand as on `standing-adult`); the fingers of every list are fixed solids on `hand-l/-r`, so the grip on the trowel and the pressing left hand cannot be posed.
- Body 1:50 to 1:500: at the thoracolumbar joint the waist part stands about 5 mm proud of the chest part on the back when the trunk is bent 16° or more (reads as a fold; visible as a short ridge and dark slivers in the three-quarter sheet).
- 2D 1:50 and 1:100: waist and chest of the shirt are two outlined polygons, so a line crosses the shirt at the joint in the line views.
- Body 1:50: face is plain (small eyes, nose, lips barely readable from the front); not remodelled.
- Body: foot 0.237 m, 6 % under the reference; shank 3 % under; kept.
- Structure 1:500: the two rib hoops read as loose loops around the spine; unchanged from the state found.
- Thermal 1:100, 1:200, 1:500: shirt, sleeves, trousers, hat, gloves and clogs are drawn grey in the thermal sheet (seen at 1:200); their solids are `of` references that state the region but no `layer`, while the regions have layers. The validator accepts it; whether the solids must name the layer or `r3-sheet.ts` must colour by region alone is not decided. State found, not changed.
- The sheets listed as "not read" above.

### Fundamental issues (outside this asset)

- `r3-sheet.ts --list thermal` draws a solid of a layered region that states no `layer` in grey without any note (legend shows only region/layer pairs), so a coarse thermal list looks unmodelled although it validates.
- `r3-sheet.ts` has no way to tile several sheets into one image and the host has no PIL, so every sheet costs one read; this limits how many of the 21 sheets an author with a small budget can read.

### Files

- `T/rework-kneeling-gardener-lumbar.py`, `T/rework-kneeling-gardener-neckline.py`, `T/rework-kneeling-gardener-decisions.py` (mine); `-girdles.py`, `-head.py`, `-fixes.py` (earlier agent, kept).
- `T/volumes/humans/kneeling-gardener.json`, catalog `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/kneeling-gardener/🔣️.json` (`rig`, `poses`, `anchors`, `representations`, `volumes`).

### Final proof

After the decision texts and with the generated sheets removed: see the line appended below by the last run.

`VALID humans/kneeling-gardener (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0` (2026-11-22 16:00)
