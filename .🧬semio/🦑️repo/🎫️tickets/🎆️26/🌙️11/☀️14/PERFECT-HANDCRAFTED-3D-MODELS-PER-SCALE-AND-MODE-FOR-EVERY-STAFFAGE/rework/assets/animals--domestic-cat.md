# animals/domestic-cat

## Pass 1 (second round)

Status: **partial**. Published and `VALID` (`r3-validate.ts animals/domestic-cat --catalog-only --volumes`: schema 0, validateAsset 0, checkVolumes 0). `T/rig-lateral-rest.md` did not exist at the start nor at the end of this pass, so the girdles are not on a lateral rest placement.

Source and catalog were identical at the start (no half-finished edit of an earlier agent). Chisel scripts, in the order they were run once: `rework-domestic-cat-trunk.py`, `rework-domestic-cat-skeleton.py`, `rework-domestic-cat-drawing.py`, `rework-domestic-cat-pens.py`, `rework-domestic-cat-decisions.py`. The first two append sections and are not idempotent: do not run them again on the current source.

### Reference (adult domestic shorthair, 4.2 kg)

| Measure | Real | Model | Verdict |
|---|---|---|---|
| Shoulder height | 0.23-0.25 m | 0.247 m | right |
| Head and body | 0.46 m | about 0.47 m | right |
| Tail | 0.25-0.30 m | 0.30 m (5 bones) | right, upper end |
| Chest girth | 0.36-0.40 m | 0.37 m (section 104 x 133 mm) | right |
| Belly girth | 0.38-0.42 m | 0.40 m (121 x 137 mm) | right |
| Head width over the cheeks | 0.08-0.09 m | 0.088 m | right |
| Scapula | 65-70 mm | 64 mm | right |
| Humerus, greatest length / joint centre to joint centre | 95-100 / 90-93 mm | 90 mm | right as joint distance |
| Radius, greatest / joint to joint | 92-95 / 86-88 mm | 84 mm | 3 % short |
| Carpus with metacarpus | 40-45 mm | 40 mm | right |
| Femur, greatest / joint to joint | 105-110 / 99-102 mm | 98 mm | right as joint distance |
| Tibia, greatest / joint to joint | 110-115 / 104-108 mm | 104 mm | right |
| Tarsus with metatarsus | 70-75 mm | 70 mm | right |
| Neck, chord C7/T1 to occiput | 70-75 mm (column 85-90 mm in its S curve) | 72 mm | right as chord |
| Shoulder joints apart | 70-80 mm | 80 mm (z offsets 22 + 18 mm) | right |
| Hip joints apart | 50-60 mm | 60 mm | upper end |
| Sacrum over the alae | 30-38 mm | was 24 mm, now 38 mm | corrected |
| Ilium wings apart (centres) | 40-46 mm | was 54 mm, now 44-46 mm | corrected |
| Ribs | 13 pairs: 9 sternal, 4 asternal on the costal arch | 13 pairs, 1-8 ended free 10-23 mm beside the sternum | corrected |

Joint order is right on both sides and the sides are identical: scapula, humerus, radius/ulna, carpus with metacarpus, digits (digitigrade); femur, tibia, tarsus with metatarsus, digits. The earlier batch review (`rework/08.md`) called the long bones 5-10 % short; that compares greatest bone lengths with joint-to-joint distances. Measured joint to joint they are within 3 %, so the sagittal rig was left as it is and no limb was stretched.

### Defects found (rendered and read)

1. Body, all scales, every bent-spine pose (sitting, play-bow, stretch): the trunk lofts were flat-capped tubes overlapping 20-55 mm past their spinal joints. Bent, the caps swung out as sharp steps and slabs on the back and under the chest, with ragged seams where equal surfaces coincided.
2. Structure 1:50: ribs 1-8 ended in the air beside the sternum; rib 8 off the spine; ilia 10 mm lateral of a too narrow sacrum (hind limbs not connected to the spine); pubis floating between the acetabula; ear cartilages 3 mm off the skull.
3. Structure 1:100: pelvis one flat plate that did not reach the tail root (tail chain loose), four rib hoops per side ending free, sternum loose and its cranial end poking out of the chest; the decision text claimed seven hoops.
4. Structure 1:200 and 1:500: rib arcs as loose hoops, sternum loose.
5. 2D side elevation 1:50, 1:100, 1:200: the trunk shapes had the same flat overlapping ends, so the sitting cat drew steps and inner cap lines.
6. 2D side elevation 1:200: every shape carried its own `penMm` of 0.5-1.6 mm; the light-line drawing of a figure 3.8 mm long was a black blob in every pose.

### Rebuilt

- **Body** (`rework-domestic-cat-trunk.py`): `rump`, `hindbody`, `midbody`, `chest` at 1:50 and 1:100, `hindbody` and `chest` at 1:200 and 1:500: each loft now ends at its spinal joint in a blunt half ellipsoid 55-62 mm long (seven sections at the fine scales, four at the coarse ones) that lies inside the neighbour at rest. Rest silhouette unchanged; the sitting, play-bow and stretch trunks are continuous. Thermal fur regions reference these solids and follow.
- **Structure** (`rework-domestic-cat-skeleton.py`): rib heads on the vertebrae at all scales; costal cartilage section to the sternum for ribs 1-8 (1:50), hoops 1-3 (1:100), 3 and 7 (1:200), 4 and 9 (1:500); the last hoop joins the one before it as costal arch (1:100, 1:200). 1:50: sacral alae 38 mm, ilium wings moved onto them, pubis extended to both acetabula, ear cartilage rooted on the skull. 1:100: the pelvis plate replaced by `sacrum`, `near-ilium`, `near-ischium`, `far-ilium`, `far-ischium`, `pubis` at 15 mm, sacrum reaching the first caudal rod; cranial end of the sternum pulled back into the thoracic inlet.
- **2D** (`rework-domestic-cat-drawing.py`, `rework-domestic-cat-pens.py`): side elevation trunk shapes at 1:50, 1:100, 1:200 end in the same noses; the 24 per-shape pens of the 1:200 side elevation removed, the drawing uses the 0.2 mm body pen of the scale and reads as a cat.
- **Decisions**: body and structure texts of all four scales say what was modelled; the wrong hoop count at 1:100 corrected.
- **Rig, poses, clips, anchors**: unchanged.

### Read after the last geometry change

1:50 body standing, sitting, play-bow, stretch-deep, walk-cycle 0.15 s; 1:50, 1:100, 1:200 structure; 1:50 thermal cut; 1:200 body sitting; 2D 1:50 sitting, 2D 1:200 standing. Rendered but not read: stretch-stance, walk-cycle 0.55 s, stalk-cycle 0.3 and 0.9 s, stretch 1 and 2.5 s, 1:100 and 1:500 sitting, 1:500 structure, 2D 1:100. Not rendered after the change: thermal at 1:100, 1:200, 1:500 (read before the change at 1:50 only).

### Remaining, stated plainly

- Rig: shoulder and pelvic girdle on lateral rest placement (waits for `rig-lateral-rest.md`); today the breadths come from `z` offsets of sagittal bones. No clavicle (vestigial in the cat, correct to omit as a bone, but the scapula has no lateral rest direction).
- 1:50 body: forelimb reads thin below the elbow against the trunk and the carpus is a ridge between forearm and metacarpus lofts; upper arm and shoulder are pads lying on the chest wall, not blended into it; neck is one ball-like loft; scapula tops show as two bumps in the sitting back view; small ragged seam on the sitting thigh. Head, paws, toes, pads, eyes not reworked (judged acceptable, not sculpted further).
- 1:200 and 1:500 body sitting: forefoot solids kink at the carpus; coarse trunk has only two solids, so `thorax-a` bends are taken by the noses.
- Structure 1:200 and 1:500: pelvis still one plate (1:500 thinner than 1:200); skull one loft without mandible detail at coarse scales; 1:50 skull without zygomatic arches or orbits; vertebrae are continuous rods, not segments.
- Thermal: single fur layer everywhere (no skin or core under it); coarse-scale thermal sheets not read.
- 2D: plan and front trunk shapes still have flat ends (not visible in line mode, visible as segment lines in solid mode); 2D of play-bow, stretch, walk not read; 1:100 2D not read.
- Poses and clips not read listed above.

### Fundamental issues (outside this asset)

- `T/validator.md` has no section "Rule changes authors must know" (the brief of this pass refers to it).
- `volumes-insert.py` and the catalog file use two different JSON layouts (its `dump` does not round-trip the catalog file), so every root fix outside `volumes` needs a text splice per author; a shared writer for the catalog layout is missing.
- Light-solid 2D draws the outline of every filled shape, so every multi-bone body shows a seam line at each joint overlap in plan and elevation (cat trunk reads as segments); the product has no way to merge shapes of one region into one outline in solid mode.
- `validateAsset` and the print rules accepted per-shape `penMm` up to 1.6 mm on a figure 3.8 mm long (1:200 side elevation was a blob and `VALID`); a rule that a pen may not exceed a fraction of the shape it draws is missing.

## Pass 1 (second round), 2026-11-22 16:14–17:27, cut off; finished by the umbrella finisher 20:35–20:50

The author of this pass was cut off at 17:27 without a report. What the scripts did (all applied, checked by their own "already applied" guards and by `volumes` source = catalog):

- `rework-domestic-cat-girdle.py`: scapulae on the lateral rest placement (dorsal border 22 mm beside the median plane, shoulder joint 40 mm beside it, 66.5 mm, 15.7 degrees out of the sagittal plane), new hip bones `hip-hf` / `hip-hn` (sacroiliac joint 19 mm, hip joint 30 mm beside the plane); scapula solids re-expressed, ilia and ischia bound to the hip bones.
- `rework-domestic-cat-neck-bones.py`: neck slimmed to 0.21 m girth behind the head at every scale (body, thermal); 1:200 skeleton with true `slender` long bones, spine and tail.
- `rework-domestic-cat-sitting.py`: sitting pose solved on the inclined scapulae (forepaws 21 mm beside the plane, side by side); decisions texts.
- `rework-domestic-cat-gait.py`, `rework-domestic-cat-gait-reach.py`: walk and stalk keys keep half of the scapular swing, limb solved in the sagittal plane; `walk-phase-3` and `walk-phase-7` keep the largest reduction that still reaches.

Proof by the finisher (2026-11-22 20:36): `r3-validate.ts animals/domestic-cat --catalog-only --volumes` → `VALID animals/domestic-cat (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0`; check CLI `✓ domestic-cat (1/1)` (1:500 unavailable by decision, 1:50 to 1:200 suitable).

Sheets read at full size by the finisher: 1:50 body `standing-alert`, `sitting-tail-wrapped`, `walk-phase-3`; 1:50 structure; 1:200 body; 2D 1:50 `sitting-tail-wrapped`. Verdict: reads as a cat from all six sides; scapulae and hip bones sit where they belong, ribs hang on the thoracic spine, the sitting forepaws stand side by side, the planted paws of `walk-phase-3` are on the ground. Rendered but not read: 1:100 and 1:500 body, 1:50 thermal, 1:200 structure, `walk-phase-7`, `stalk-phase-3`, `play-bow`, 2D 1:50 default and 1:200.

Remains imperfect: the 2D front elevation of the sitting pose is a stack of ovals (trunk segments) and does not show the forelegs; the 1:200 head is a wedge with block ears; toes are not separated at any scale; clips were not rendered.
