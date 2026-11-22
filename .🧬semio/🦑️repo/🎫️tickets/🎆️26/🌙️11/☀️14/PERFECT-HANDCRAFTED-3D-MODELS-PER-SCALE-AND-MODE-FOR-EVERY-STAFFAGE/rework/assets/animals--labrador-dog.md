# animals/labrador-dog — running author report

Earlier pass: `rework/08.md`, section 1 (sit pose solved anew, sternum bars at 1:200 and 1:500, straight limb bars at 1:500).

## Pass 1 (second round), 2026-11-22

Status: **partial**. Published and `VALID` (source and catalog, with `--volumes`). The shoulder and pelvic girdles are not on a lateral rest placement: `rig-lateral-rest.md` did not exist at the start nor at the end of this pass, so no girdle was faked with sagittal bones.

Source and catalog were identical at the start (`volumes` equal), so there was no interrupted state to reconcile.

### Reference table (adult male Labrador Retriever, 30 kg; FCI 122, Evans & de Lahunta, own knowledge) against the model

| measure | real | model (rig / solids / sheet) | verdict |
|---|---|---|---|
| withers height | 0.56-0.57 m | 0.57 m (sheet, top of withers) | ok |
| chest depth, elbow height | 0.26-0.28 m, elbow at half the height | 0.287 m, elbow 0.284 m | ok |
| trunk, point of shoulder to buttock | 1.05-1.12 x withers = 0.60-0.64 m | 0.65 m | 2 to 8 % long, kept (see below) |
| thoracolumbar spine T1-L7 | 0.26 + 0.19 = 0.45-0.47 m | thorax 0.211 + lumbar 0.190 = 0.40 m to the neck joint | bones are not long; the excess length is soft form (forechest and buttock) |
| neck C1-C7 / head | 0.20-0.23 / 0.23-0.25 m | 0.224 / 0.240 | ok |
| tail | 0.35-0.40 m, thick at the root | 0.36 m in three bones | ok |
| scapula / humerus / radius-ulna / metacarpus / digits | 0.17 / 0.19 / 0.19 / 0.07 / 0.07 | 0.158 / 0.176 / 0.179 / 0.0745 / 0.0728 | 6-7 % short, both sides alike, in the breed spread |
| femur / tibia / metatarsus / digits | 0.20 / 0.21 / 0.12 / 0.07 | 0.198 / 0.202 / 0.121 / 0.075 | ok |
| joint order fore | scapula, humerus, radius-ulna, carpus-metacarpus, digits (digitigrade) | the same, both sides | ok |
| joint order hind | femur, tibia, tarsus-metatarsus, digits | the same, both sides | ok |
| hip joints apart / stifles / elbows | 0.10-0.12 / 0.15-0.17 / 0.15-0.17 m | 0.11 / 0.16 / 0.16 (z offsets of the bones) | ok |
| thigh, sagittal x lateral | 0.17-0.19 x 0.08-0.09 m | 0.196 x 0.096 | upper bound, kept |
| second thigh above the hock | 0.05-0.06 m | 0.054 | ok |
| skull breadth / muzzle depth with jaw | 0.12-0.13 / 0.08 m | 0.132 / 0.08 | ok |
| ribs | 13 pairs, sloping caudoventrally, 9 sternal | 9 hoops at 1:50 sloping caudoventrally, sternum from xiphoid to manubrium | simplified for 7.5 mm (13 ribs would leave 6 mm gaps), not wrong |

The rig's sagittal chain, symmetry, joint order and segment lengths were measured and left as they are: they are right. What the dev calls a wrong skeleton is, for this asset, the missing girdles (scapulae and pelvis halves sit on the sagittal chain with a `z` offset only) and that waits for the lateral rest placement.

### Defects found by rendering (all sheets read at full size)

1. 1:50 to 1:500 body, hindquarters: each thigh ended in a flat capped drum that stood proud beside and above the croup, with a cleft between thigh, croup and tail root (rear three-quarter view).
2. All scales, neck base: the neck loft started with a flat disc 47 mm behind its joint (at 1:500 even with two sections in the wrong order), so any neck movement turned a shelf out of the chest.
3. 1:100, 1:200, 1:500 body: the head carried the 1:50 parts list (eyelids 7.8 mm, pupils, nostrils, eyes 12 mm, tongue 9 mm, flews) unchanged at every scale; at 1:500 also a separate 37 mm jaw, 40 mm paws and 13 mm ears. The old rule let these pass because they were identical to their 1:50 counterparts; the per-scale feature rule now reports them (62 findings at the start of this pass).
4. 1:200 and 1:500 structure: seven lumbar vertebrae as 480-vertex polyhedra copied from 1:50, zygomatic arches of 9 mm, at 1:500 spine, scapulae, tail and toes at real widths next to 75 mm limb bars (a mixed diagram, as `rework/08.md` already said).
5. Structure, load path (`volumes-connected`): ear cartilage 7.6 to 13 mm off the skull at 1:50, 1:100, 1:200; shoulder joint markers 2.5 mm off the scapulae at 1:50; first caudal bar 11 mm off the pelvis at 1:200.

### What was rebuilt (scripts in the ticket folder, hand-chosen numbers)

- `rework-p1-labrador-dog-rump-neck.py`: thigh lofts of both sides at all four scales start as a gluteal dome inside the croup (top 52 mm above the hip, shifted 12 mm towards the midline, 10 / 5 / 3 / 2 own sections per scale) and reach the full thigh section below the hip; the neck base at all four scales is a ball about the cervicothoracic joint (7 / 5 / 4 / 3 sections).
- `rework-p1-labrador-dog-scales.py`:
  - 1:100: eyelids removed (body, thermal), tongue thickened to 15 mm, zygomatic arches to 11 mm, ear roots on the skull.
  - 1:200: lids, pupils, nostrils removed; eyes and tongue are thermal solids of their regions only; nose leather and flews stated at 30 mm; lumbar spine as one tapering loft with the breadth of the transverse processes; mandible bar 31 mm; zygomatic arches removed.
  - 1:500: head re-authored as one loft of skull, muzzle and lower jaw (12 sections), body jaw, flews, eyes, nose leather, tongue, lids, pupils, nostrils removed from the body; eyes, nose and tongue stay as thermal solids; paws 46 mm, front pasterns 51 mm, ears 19 mm; skeleton as a consistent bar diagram at pen width (cervical, thoracic, lumbar, scapulae, caudal bars, phalanges), second rib hoop and zygomatic arches removed.
  - 1:50: ear roots (annular cartilage) on the skull, shoulder joint markers enlarged to reach the scapulae.
- `rework-p1-labrador-dog-joins.py`: ear roots at 1:200, first caudal bar into the sacrum at 1:200, mandible as one bar along the muzzle at 1:500 (the jaw is a load-bearing bone and needs a member).
- `rework-p1-labrador-dog-decisions.py`: decisions (en, de) for every scale; the outdated 1:500 sentence about "real widths" replaced.
- Rig, poses, clips, anchors, thermal regions, structural members, 2D drawings: unchanged. The 2D drawings were checked (`r3-2d` at 1:500 read; silhouette, cross-scale and views rules pass at all scales), the changed forms lie inside the drawn outlines.

### Rendered and read

Before: 1:50 body (sheet, right, front, rear three-quarter), structure (sheet, right), thermal; 1:100 body; 1:500 body and structure; poses sitting-waiting and walk-phase-2.
After the last geometry change: 1:50 body rear three-quarter, sitting-waiting, sniffing-head-down, clips sit-and-pant (t 3 s) and trot-cycle (t 0.2 s); 1:500 body and structure; 1:200 structure; 2D at 1:500.
Rendered but not read, or not rendered after the change: 1:100 and 1:200 body, thermal sheets of 1:100 to 1:500, 1:100 structure, walk-cycle and sniff-ground instants, walk and trot phase poses other than the ones named, 2D at 1:50 to 1:200.

### Validation

`VALID animals/labrador-dog (catalog asset + volumes source): schema 0, validateAsset 0, checkVolumes 0`
`VALID animals/labrador-dog (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0` (`--catalog-only --volumes`, after `volumes-insert.py`).

### Remains imperfect (open list for the next pass)

- Rig, all scales: shoulder and pelvic girdle on lateral rest placement (scapulae, pelvis halves; limbs hung at the true breadths), with poses, clips, 2D shapes, anchors, thermal regions and members re-bound. Blocked on `rig-lateral-rest.md`.
- 1:50 body, sitting pose and sit-and-pant: the neck ball shows as a low hump with a seam at the withers; the shoulder mass disappears into the chest so the foreleg starts as a short stub at the elbow; the open mouth shows no tongue from the side.
- 1:50 body: trunk 2 to 8 % long (forechest and buttock, not the bones); thigh at the upper bound of breadth; ears are plain oval leathers without fold; the step from thigh to second thigh at the stifle is still visible from behind.
- 1:50 structure: 9 rib hoops (planar revolves) instead of 13 curved ribs with costal cartilages; no separate radius and ulna; pelvis halves share one sagittal bone.
- 1:100 structure sheet and 1:100 / 1:200 body sheets: changed (lids, tongue, arches, thigh, neck) but not read after the change.
- 1:500 structure: caudal bars (75 / 61 / 75 mm) are wider than the tail and the mandible bar lies coaxial with the skull bar: the feature rule asks for it, the diagram is consistent but the bars stand outside the body ghost at the tail; one thin rib hoop (10 mm, not reported by the rule) and a 76 mm sternum bar remain from the earlier pass.
- 1:200 and 1:500 coarse lofts other than head, thigh and neck are still reductions of the 1:50 section tables (chest, hindbody, limbs): proportions are right, but they were not re-authored section by section in this pass.
- 2D drawings: not redrawn; they still validate against the changed body.

### Fundamental issues (outside this asset)

- `validator.md` has no section "Rule changes authors must know" yet; the per-scale feature rule and `volumes-connected` are described further down under "Connectivity and per-scale feature".
- `volumes-feature` at 1:500 forces every structure member of a small animal to 75 mm (or the thinnest drawn stroke of its bone) although the thermal list is not judged that way: a caudal vertebra bar must be wider than the tail it lies in, and a load-bearing bone smaller than the feature (jaw) must still carry a member (`volumes-structure-coverage`). A structure member declared as an axis stroke (judged by its length, drawn at pen width) would let the diagram stay inside the body.
- `volumes-feature` reported rib hoop `rib-8` (10 mm section) at 1:500 but not the identical hoop `rib-3`; the rule seems to judge the two revolves differently.
- `r3-sheet.ts` prints nothing but the path; when called with a quoted argument list from zsh it reports "scale undefined" (word splitting), worth a line in `r3-tools.md`.

## Pass 2 (second round), 2026-11-22

Status: **partial**. Published; source and catalog agree. `r3-validate.ts animals/labrador-dog --volumes` and `--catalog-only --volumes` both end `VALID` (schema 0, validateAsset 0, checkVolumes 0). The girdles are on the lateral rest placement. Not signed as finished: the sitting pose and the items under "Remains imperfect".

`rig-lateral-rest.md` existed at the start. A script `rework-p2-labrador-dog-girdle.py` and a candidate of an interrupted earlier run of this pass were found (15:24, not applied, no report entry); I read the script, checked its frames with `rig-rest.ts`, and adopted it.

### Additions to the reference table

| measure | real | model before | model now |
|---|---|---|---|
| chest width at the widest rib (rib 8 to 9), with coat | 0.24-0.25 m | 0.28 m (round barrel, girth about 0.90 m) | 0.248 m (oval 0.29 deep, girth about 0.80 m) |
| width between the shoulders (rib cage under the scapulae) | 0.19-0.21 m | 0.27 m, shoulders and upper arms buried in the chest | 0.20-0.21 m, shoulders outside |
| loin width | 0.18-0.21 m | 0.236 m | 0.21 m |
| overall width | 0.25-0.26 m | 0.28 m | 0.258 m across the upper arms (`dimensions.depthM`) |
| scapula: dorsal borders apart / shoulder joints apart | 0.07 / 0.15 m | bone sagittal, offset by `z` only | 0.07 / 0.15 m, bone inclined 14.2 degrees, 0.163 m long |
| hip bone: sacroiliac joints apart / hip joints apart | 0.05 / 0.11 m | no bone, thigh on `pelvis` with `z` | `ilium-hn/-hf` 0.052 / 0.11 m, 55 mm long |
| ribs | 13 pairs, 9 sternal, 3 asternal on the costal arch, 1 floating | 9 planar hoops | 13 curved pairs with cartilages |
| forearm | radius cranial, ulna caudal with olecranon about 35 mm above the joint | one solid | two bones |

### What was done

1. **Rig** (`rework-p2-labrador-dog-girdle.py --apply`): new bones `ilium-hn`, `ilium-hf` (sacroiliac joint [0.038, 0.028, ±0.026] to hip joint [0, 0, ±0.055], joint ±2 degrees); thighs re-parented onto them without moving (rest frames equal to 5e-6). `scapula-fn/-ff` rest with `abductionDeg ∓14.2072` so that the blade ends in the shoulder joint; humeri re-parented onto the scapula tip with their sagittal rest frame. All solids of the scapula bones re-expressed in the new frame (7 per side over all scales and lists), scapula elevation shapes stretched to the new length, pelvis structure rebuilt per half on the ilium bones at 1:50 and 1:100 (ilium wing, acetabulum, ischium, pubis, ischial arch, symphysis; 1:200 and 1:500 one girdle solid spanning both bones through `members`), sacroiliac joint markers, scapula plates with spine per scale. `walk-cycle.advanceM` 0.5212 (planted travel after the change).
   - Decision on poses: the scapula swings about its own inclined axis and carries the limb with it; the pose offsets were **not** re-solved. Re-solving (option `--repose`, tried) keeps every paw where it was but puts abduction offsets on the humeri, and `validateAsset` then reports 10 findings (`lateral-ambiguous`, `elevation-lateral`) for the flat limb drawings. Consequence, measured with `rig-lateral-compare.ts`: 2D shapes of the forelimbs move by up to 92 mm in walk and trot and 150 mm in the sitting pose against the old rig (forepaws come towards the midline when the scapula is retracted). In gait this reads as natural convergence; in the sit the forepaws stand side by side at about ±27 mm (see open list).
2. **1:50 structure** (`rework-p2-labrador-dog-skeleton.py`): 13 rib pairs as curved lofts (vertebra, angle, body, costochondral junction; 9 x 6 mm, `slender` where 20:1 holds, ribs 1 and 13 stout), 13 cartilage pairs (9 to the sternum, 10 to 12 onto the cartilage ahead, 13 free); radius and ulna as two lofts per forearm. 175 solids.
3. **1:100 and 1:200 structure**: planar hoops replaced by typified curved ribs on the same rib lines (7 pairs of 15 mm; 3 pairs of 30 mm drawn in under the skin).
4. **Body, all four scales** (`rework-p2-labrador-dog-trunk.py`, `-depth.py`): chest and hindbody lofts narrowed laterally by a hand-set factor along the trunk (1.0 at the rump, 0.86 loin, 0.90 widest ribs, 0.80 between the shoulders, 1.0 at the forechest); the same factor on the plan shapes `chest` and `hindbody` point by point, the ratio of the widest section on the front shapes; `dimensions.depthM` 0.28 to 0.258.
5. **Decisions** en/de at every scale (`rework-p2-labrador-dog-decisions.py`): girdles, trunk section, rib cage; stale wording (barrel chest, rib hoops, sternum bar at 1:500) corrected.
6. Helpers: `rework-p2-labrador-dog-tiles.sh` (every pose and two instants of every clip in one view), `rework-p2-labrador-dog-tile.py` (tiles PNGs without third-party modules).

### Rendered and read after the last geometry change

- 1:50 body: six-view sheet in the default pose; all 19 poses and 2 instants of each of the 4 clips in the three-quarter view (27 images, read as three tiled sheets at 800 px each); front view of walk-phase-6/7 and trot-phase-0..6; six-view sheet of sit-and-pant 4.5 s.
- All four scales: body and structure six-view sheets (tiled four to a sheet, 1200 px each); 2D drawings of 1:50 (default and sitting-waiting), 1:100, 1:200, 1:500.
- Read before the trunk was narrowed and not again after it: side view of all 27 poses and instants (the change is lateral only), front view of the first nine poses, thermal sheets of the four scales (thermal trunk solids are references to the body solids), 1:50 structure close-ups.
- Not rendered: thermal and structure lists in any pose other than the default; `--cut` thermal sheets.

### Remains imperfect (open list for the next pass)

- 1:50 to 1:500 body, sitting-waiting and sit-and-pant: the scapula retracts 35 degrees and takes the shoulder mass into the trunk, so the forelegs still leave the trunk at the elbows; neck and chest meet in a visible seam at the withers; the trunk reads as one column. Needs a shoulder mass that stays outside the chest in that pose (or a sit solved with less scapula rotation and re-planted paws).
- Rig, sitting pose: forepaws side by side at the midline (about ±27 mm) because pose offsets were not re-solved; a re-solve needs the forelimb 2D shapes authored as `solid` shapes first.
- 1:50 body: thigh still a rounded drum with a crease against the flank in swing phases and a step at the stifle from behind; ears plain leathers without fold; trunk length 2 to 8 % long (soft form); open mouth shows the tongue from the front and three-quarter view, from the side it is hidden by the jaw rim.
- 1:50 structure: ribs have one section along their length (no head, neck and tubercle), sternum is one loft without sternebrae, carpal and tarsal bones not separated; pelvis halves at 1:200 and 1:500 are one solid on two bones.
- 1:100 structure: radius and ulna one bar (two 15 mm bars would fill the forearm).
- 1:200 and 1:500 body: limb lofts are still reductions of the 1:50 section tables (chest and hindbody now re-stated in width only).
- 1:500 structure: no ribs; caudal bars (75 / 61 / 75 mm) wider than the tail, mandible bar coaxial with the skull bar (feature rule).
- 2D: only `chest` and `hindbody` front and plan shapes were redrawn; the scapula elevation shapes were stretched 3 %, not redrawn. 2D of walk and trot poses was not rendered after the rig change (valid, not looked at).
- `S/🧪️tests/🧊️model-skeleton` pins on this asset ("Missing lumbar vertebra L1" was already failing) were not run against the new rib and forearm ids.

### Fundamental issues (outside this asset)

- A limb that hangs from an inclined girdle bone cannot be kept in its sagittal plane in a pose without `lateral-ambiguous` / `elevation-lateral` findings on every flat limb shape (10 findings for 8 degrees of humerus abduction). `rig-lateral-rest.md` 1.5 states this; for quadrupeds it means either all forelimb drawings become `solid` shapes or the scapula swing drags the paws sideways. A rig option "child keeps its parent-of-parent plane" or a validator allowance for compensating offsets would remove the conflict.
- No image tool on the host (no PIL, ImageMagick, ffmpeg): reading 27 pose renders one by one is not affordable; `r3-sheet.ts` could offer a contact sheet of all poses and clip instants.
