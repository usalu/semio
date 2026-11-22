# animals/dairy-cow — rework report

## Pass 1 (second round), 2026-11-22

Status: **partial**. Rig, limbs at all four scales, skeleton connectivity, girdle placement and the limb drawings were rebuilt and published; `r3-validate.ts animals/dairy-cow --volumes` and `--catalog-only --volumes` both end `VALID … schema 0, validateAsset 0, checkVolumes 0`. Trunk, neck, head, udder, tail and the thermal-only solids were not re-modelled, and the image reader failed for the last third of the pass (see "Not looked at").

State found: source `volumes/animals/dairy-cow.json` and catalog `volumes` were identical. The catalog asset was already `INVALID` (15 `volumes-feature` findings at 1:500: claws 45 mm, fore cannon 74 mm, spine 60/68 mm, tail vertebrae 16 to 40 mm) because the per-scale allowance was removed from the rule today; the measured rules also reported open load paths in the skeleton at every scale.

### Reference (Holstein-Friesian cow, 1.45 m at the withers, 650 kg; own knowledge, no source consulted)

| Measure | Real | Model before | Model after |
|---|---|---|---|
| Hip joint above ground | 1.15–1.20 m | 1.11 m | 1.17 m |
| Stifle height / angle femur–tibia | 0.80–0.85 m / 120–130° | 0.74 m / 105° | 0.79 m / 123° |
| Hock joint height / angle tibia–cannon | 0.47–0.52 m / 140–150° | 0.47 m / 130° | 0.47 m / 140° |
| Femur, joint to joint (bone 0.46 m) | 0.41–0.43 m | 0.431 m | 0.42 m |
| Tibia, joint to joint | 0.37–0.40 m | 0.385 m | 0.385 m |
| Tarsus + metatarsus | 0.34–0.36 m | 0.349 m | 0.349 m |
| Scapula with cartilage, inclination | 0.43–0.46 m, 55–62° | 0.46 m, 63° | 0.44 m, 61° |
| Humerus, joint to joint (bone 0.36 m), inclination | 0.30–0.33 m, 40–50° | 0.36 m, 32° | 0.32 m, 42° |
| Shoulder angle / elbow angle | 100–110° / 125–140° | 95° / 118° | 103° / 129° |
| Elbow position | under the dorsal end of the scapula, 0.78–0.80 m | 0.10 m behind it, 0.79 m | 0.02 m behind it, 0.79 m |
| Radius + carpus / metacarpus | 0.38–0.40 / 0.25–0.27 m | 0.39 / 0.26 m | unchanged |
| Pastern (two phalanges) / hoof | 0.10 m at 50–55° / 0.075 m high | one 0.15 m bone "pastern and hoof" | pastern bone 0.10 m + hoof bone 0.05 m |
| Forearm below elbow, depth × width | 0.18 × 0.115 m | 0.15 × 0.104 m | 0.178 × 0.116 m |
| Forearm above carpus / carpus | 0.082 × 0.076 / 0.10 × 0.10 m | 0.08 / 0.084 m rod | 0.082 × 0.076 / 0.10 × 0.10 m |
| Fore cannon mid (girth 0.20 m) | 0.062 × 0.066 m | not separable from the knee cylinder | 0.062 × 0.066 m |
| Gaskin below stifle / above hock (with Achilles) | 0.22 × 0.13 / 0.13–0.15 × 0.075 m | 0.25 × 0.14 / 0.11 m | 0.216 × 0.128 / 0.14 × 0.072 m |
| Point of hock above the joint | 0.10–0.12 m | separate finger | 0.095 m, on the Achilles line |
| Hind cannon mid | 0.064 × 0.062 m | 0.07 × 0.06 m | 0.064 × 0.062 m |
| Between the hip joints / shoulder joints | 0.30 / 0.38–0.42 m | all bones in the median plane | 0.30 / 0.396 m |
| Over the hooks | 0.55–0.58 m | eggs at ±0.232 m on the back | ±0.262 m ± 0.035, at the side of the loin |

### Defects found

Crouched hind legs (hip joint 6 cm low, stifle 105°, hock 130°); fore leg standing 10 cm behind the withers with a nearly horizontal upper arm; no pastern bone; shoulder as a plate, upper arm as a rod with a free elbow; legs as tapered rods with cylindrical knees; hook bones as eggs on the back; point of the hock as a loose finger; at 1:500 arms and thighs partly outside or buried in a chest that was too narrow; skeleton with gaps (scapula 32 mm from its joint, atlas 8 mm from the axis and 24 mm from the skull, ear cartilages 8–12 mm from the skull, 1:500 hip bones and udder ligament unconnected, 1:200/1:500 neck column 18 mm short of the skull); every limb bone in the median plane.

### What was rebuilt

Scripts in the ticket folder, run in this order by `rework-dairy-cow-build.sh` from the frozen copy `🗑️generated/rework-dairy-cow/catalog-before.json` into the candidate `🗑️generated/r3-dairy-cow/🔣️.json`; `rework-dairy-cow-publish.py [--keep-2d]` writes the catalog file and the source; `rework-dairy-cow-2d.py` draws.

- **Rig** (`rework-dairy-cow-rig.py`): hip joint raised 0.06 m (pelvis bone 0.270 m at 51°), femur 0.42 m, scapula 0.44 m at 61°, humerus 0.32 m, four `pastern-*` bones (fetlock joint, old hoof limits) and `hoof-*` as coffin joint to toe. All 14 poses and the 12 clip keys that override limb bones were re-solved by two-bone inverse kinematics so that hock, carpus, cannon and digits keep their place against the spine (fore limb 73.5 mm further ahead as a whole); no target was out of reach; joint limits shifted with the rest angles. Contact anchors, pelvis-bound solids, masses and elevation shapes re-expressed in the new frames.
- **Girdles** (`rework-dairy-cow-girdle.py`, after `rig-lateral-rest.md`): new bones `ilium-hn/-hf` from the sacroiliac joint (0.07 m beside the plane) to the hip joint (0.15 m), thighs re-parented to them with unchanged sagittal frames; scapulae at 0.09 m beside the plane, shoulder joints at 0.198 m. Every solid of the 22 moved bones re-expressed so that it keeps its world place; the ilium / hip-bone members are bound to the ilium bones at every scale.
- **Body** (`rework-dairy-cow-body.py`, hand tables per scale): shoulder, upper arm with triceps, elbow point, forearm, carpus, cannons, fetlocks, pasterns, dewclaws, thigh, stifle, gaskin with Achilles line, hock with its point, hook bones at 1:50; the same parts with their own stations at 1:100 (6–7 stations, hock point in the cannon), 1:200 (4–5) and 1:500 (2–4, every part at least 75 mm, one cloven hoof per foot, chest widened to carry the arms). Thermal follows by reference (`of`); the merged 1:500 claws were renamed in both lists.
- **Structure** (`rework-dairy-cow-structure.py`): femur, humerus and scapula members carried to the new bone lengths at every scale; scapular cartilage up to its joint at the withers, occipital condyles, longer atlas, ear cartilages reaching the skull, neck column reaching the skull at 1:200 and 1:500; 1:500 pelvis as ilium wings, hip bones, ischia and pubis with the udder ligament carried to the pubis; 1:500 spine and tail at the printable feature. Pastern phalanges on the pastern bones, claw bones on the hoof bones.
- **2D** (`rework-dairy-cow-2d.py`): limb shapes of every scale and view re-cut from the new body with `r3-project2d.ts`; new `*-pastern` shapes, at 1:500 and in the 1:200 plan also new `*-cannon` and `*-hoof` shapes (the old drawings had the foot in the cannon shape); hoof shapes are now the dark claws. All other front shapes moved down 0.06 m (the front frame is root-relative and the root joint was raised).
- **Decisions** (`rework-dairy-cow-decisions.py`): one sentence block per scale, en and de.

### Looked at

Read at full size: 1:50 body six views, right side and three-quarter in the default pose; `lying-resting` six views; `walk-cycle` at 0.3 s; 1:100 and 1:500 body six views; 1:50 and 1:500 structure six views. Each led to a correction (hock point, shoulder mound, hooks, 1:500 chest and arms).

### Not looked at (the image reader stopped answering after the first 2D carry; renders were produced but could not be read)

Everything after the last shoulder correction, the 1:500 chest widening, the 2D carry and the girdle: no 2D sheet of any scale, no thermal sheet, 1:200 body, the poses `grazing-head-down`, `kneeling-front`, `lowering-hindquarters`, the clips `cud-chewing`, `lie-down`, `graze-and-chew`, a second walk instant, and the final state of every list. The validator is the only evidence for these.

### Remaining (for the next pass)

1. Rig: the scapula bone is parallel to the median plane (shoulder joint 0.108 m beside its tip) instead of inclined 14° outwards: inclined, `lateral-ambiguous` rejects the flat shapes of the blade (2.6 mm on paper) and, once the limb is kept upright by abduction/twist offsets, the front and plan shapes of every bone below it. Needs `solid`-authored 2D shapes for scapula and fore limb (switch `INCLINED_SCAPULA` in the girdle script). No clavicle (cattle have none).
2. All scales, body: trunk (`barrel`, `back`, `rump`; `trunk`), neck, head, jaw, ears, udder, tail and coat patches untouched: neck top line dead straight, chest dome under the neck reads as a ball, head a wedge without cheek and muzzle breadth, rump/thigh seam visible, withers angular. Claws and dewclaws are the old solids re-bound.
3. All scales, structure: skull is a wedge with two plates as mandible; sternum a bent tube; ribs do not reach the sternum; pelvis members at 1:50–1:200 are the old ones moved; 1:50 udder ligament is a plain quadrilateral plate; scapula blade still a flat loft without neck and glenoid.
4. Thermal: `th-*-rumen-flank`, plume and breath emitters not checked against the changed hindquarters; `areaM2` not re-derived (rules pass).
5. 2D: the new limb shapes are cleaned ruler outlines (not economical hand-drawn shapes), lost `followTip`, and were never seen; hoof cleft strokes not redrawn; `representations[*].decisions` not updated.
6. Default pose: the near hind leg still trails with a tibia of about 45°; hock sits about 0.07 m ahead of the pin-bone vertical in the rest pose.
7. Lying poses: udder dips below the ground line (pose offsets, as before).

### Fundamental issues

- `validator.md` has no section "Rule changes authors must know".
- `lateral-ambiguous` makes an anatomically inclined scapula impossible while limb drawings are flat shapes (see Remaining 1); the guide's quadruped example (12° abduction) cannot validate with elevation shapes on the scapula.
- The front-view frame is root-relative: moving the root joint against the body shifts every front shape; `r3-tools.md` does not say so.
- `rework-07-2d.py` and the ruler leave `NEW shape` outlines to the author; bones whose drawing lived in a neighbour's shape lose it silently when the neighbour is re-cut (feet at 1:500).
- The catalog file was written once in a layout that the shared `dump` did not reproduce byte for byte (whole-file re-layout on the first publish; later publishes round-trip).
- The Read tool's PreToolUse hook timed out for every image from 02:25 on.

## Pass 2 (second round), 2026-11-22

Status: **partial**. Published; `r3-validate.ts animals/dairy-cow --catalog-only --volumes` ends `VALID animals/dairy-cow (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0`, and the form with the source merged is `VALID` too. `rig-lateral-rest.md` existed at the start and at the end and was followed.

### Reference added this pass (Holstein-Friesian cow, own knowledge)

| Measure | Real | Before | After |
|---|---|---|---|
| Neck at the shoulder, depth × breadth | 0.55–0.60 × 0.30 m | 0.56 × 0.28 m | 0.58 × 0.30 m |
| Neck at the throat, depth × breadth | 0.27–0.30 × 0.16–0.18 m | 0.20 × 0.15 m | 0.28 × 0.17 m |
| Top line of the neck between withers and poll | dips 0.06–0.10 m | straight | dips 0.08 m |
| Withers over the back | 0.02–0.04 m, no hump | 0.05 m hump over 0.2 m | 0.03 m, six stations |
| Fore chest ahead of the point of the shoulder | 0.08–0.12 m (brisket only) | 0.20 m dome | 0.10 m |
| Muzzle breadth / cheek depth | 0.16–0.18 / 0.18–0.20 m | 0.15 / 0.176 m | 0.17 / 0.19 m |
| Scapula, dorsal border to glenoid, inclination outwards | 0.43–0.46 m, 12–15° | 0.44 m, 0° | 0.453 m, 13.8° |
| Sternum | 0.50–0.55 m, flat, 7 sternebrae, ribs 1–8 | bent tube 0.45 m, reached rib 5 | flat bar 0.62 m with xiphoid, through the ventral ends of ribs 1–8 |
| Mandible | 0.38–0.40 m, ramus 0.12 m with condyle and coronoid process | two flat plates | halves with ramus, condyle, coronoid process, incisors at the symphysis |

### Looked at (read at full size unless noted)

Before any change: 1:50 body six views with rig, right and front close-ups, 1:50 structure six views. After the trunk and girdle change: 1:50 body six views; poses `grazing-head-down`, `lying-resting`, `kneeling-front`, `lowering-hindquarters` and `walk-phase-0/2/4/6` (right side, tiled at half size); clips `cud-chewing` 2.5 s, `lie-down` 4 s and 8 s, `graze-and-chew` 6 s (tiled); 1:50 thermal six views; 1:50 structure six views after the skeleton change; `lying-resting` six views; 1:100, 1:200, 1:500 body (tiled), structure (tiled) and thermal (tiled); the 2D sheet of 1:50 at full size and of all four scales tiled; final 1:50 right side and structure front view (scapulae run from 0.09 m to the shoulder joints at 0.198 m, hip bones from the sacroiliac joints to the hip joints, limbs hang from them, feet on the ground).

### What was rebuilt

- **Rig, shoulder girdle** (`rework-dairy-cow-girdle.py`): the scapula bone now rests inclined 13.8° outwards (`abductionDeg`), from its dorsal border 0.09 m beside the plane to the shoulder joint at 0.198 m; the humerus hangs from its tip in the sagittal frame. Found and fixed: the inclined bone kept 0.44 m, which lifted the shoulder joint and the whole fore limb 12 mm off the ground; the bone is now 0.4531 m so that its sagittal length stays 0.44 m and the joint is where the drawings and poses have it. `advanceM` of the walk is the planted travel 1.0259 m. `lateral-ambiguous` accepts the flat scapula shapes (validator item 12).
- **Body, all scales** (`rework-dairy-cow-trunk.py`, new, in the build chain after the body chisel): neck as top and bottom line tables (7 / 5 / 4 / 3 stations), neck crest and dewlap carried to the new lines, withers and back front (6 stations at 1:50, 4 at 1:100, the 0.98 station of the 1:200 trunk), fore chest shortened, muzzle and cheek breadth at every scale. Thermal follows by `of`.
- **Structure** (`rework-dairy-cow-skeleton.py`, new, after the girdle chisel): 1:50 skull loft of nine stations with zygomatic arches and cheek-tooth rows, mandible halves with ramus, condyle, coronoid process and incisors, glenoid ends of both scapulae; skull of six stations at 1:100; sternum as a flat bar through the ventral ends of the sternal ribs at 1:50, 1:100 and 1:200.
- **2D** (`rework-dairy-cow-2d.py --bones neck,head,jaw`, new option): neck, head and jaw shapes of every scale and view re-cut from the new body (the copy before this step is `🗑️generated/rework-dairy-cow/catalog-before-2d-pass2.json`).
- **Decisions** (`rework-dairy-cow-decisions.py`): one more block per scale, en and de, for neck, top line, skull, sternum and the scapula placement.

Rebuild chain: `rework-dairy-cow-build.sh` (rig, body, trunk, structure, girdle, skeleton, decisions) then `rework-dairy-cow-publish.py --keep-2d`.

### Not looked at

After the last change (scapula lengthened by 13 mm, which puts the shoulder joint back at its pass 1 place; every pose render above was made with the joint 12 mm higher and 6.5 mm further back) only the default pose was read (right side, structure front). The final renders of `grazing-head-down`, `kneeling-front`, `walk-phase-2` in three-quarter view and of `walk-cycle` at 0.9 s were produced but the image reader timed out twice (PreToolUse hook). No pose other than the default was looked at in 2D. No `--cut` thermal sheet.

### Remaining (for the next pass)

1. All poses and clips: read once more after the 13 mm scapula correction (expected: unchanged against pass 1 placement).
2. `lying-resting`, `lie-down` end: the udder hangs about 0.07 m (teats 0.12 m) below the ground line. The pose's `dx`/`dy` on the udder bone do nothing (revolute joints realise only angles) and a positive `angleDeg` swings it lower; needs a pose-specific flattened udder solid or a prismatic udder joint. The stretched far fore leg has its hoof tip slightly below the ground and the folded near cannon dips about 0.03 m below it.
3. 1:50 and 1:100 body: rump still boxy at the pin bones, coat patches stand 10–15 mm proud of the trunk as slabs (thick outline in the side view), rump/thigh seam visible, the shoulder mound stands about 0.03 m above the back line at the withers, head still one wedge loft plus jaw (only breadths corrected), ears, tail, udder, claws and dewclaws not re-modelled.
4. 1:500 body and 2D: no coat patches (they exist only as thermal lofts because a 28 mm slab is below the feature); a Holstein at 1:500 should carry them as declared sheets or as tone in the trunk.
5. Structure: ribs are still full hoops (sternal and asternal alike, no costal cartilages), pelvis members at 1:50–1:200 are the old ones moved, 1:50 udder ligament is a plain plate, scapula blade without spine relief, skulls at 1:200 and 1:500 are the old wedge.
6. Thermal: `areaM2` not re-derived after the neck became deeper; plume and breath emitters not seen in any sheet (not visible without `--cut`).
7. 2D: all limb, neck, head and jaw shapes are cleaned ruler outlines without `followTip`, not economical hand-drawn shapes; trunk shapes (`hindbody`, `chest`) were not re-cut after the withers and fore-chest change (within the silhouette rule, up to 25 mm at the withers and 50 mm at the hidden fore chest); hoof cleft strokes not redrawn; `representations[*].decisions` not updated; flat scapula shapes are exact at rest only.
8. Default pose: near hind leg still trails with a tibia of about 45°.

### Fundamental issues (outside this asset)

- A revolute joint silently drops `dx`/`dy` of a pose offset (`effectiveOffset` in `🔨️modules/🦴️rig`); `validateAsset` does not report such dead offsets (the cow's `lying-resting` udder offset has been dead since it was written).
- The Read tool's PreToolUse hook timed out again for images at the end of the pass.
- `r3-2d.ts` once ran for more than 100 s on this asset at the start of the pass (4 s later on), probably host load.
