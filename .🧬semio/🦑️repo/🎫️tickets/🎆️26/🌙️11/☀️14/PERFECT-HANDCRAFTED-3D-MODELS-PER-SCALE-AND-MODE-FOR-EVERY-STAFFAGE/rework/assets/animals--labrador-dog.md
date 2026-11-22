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
