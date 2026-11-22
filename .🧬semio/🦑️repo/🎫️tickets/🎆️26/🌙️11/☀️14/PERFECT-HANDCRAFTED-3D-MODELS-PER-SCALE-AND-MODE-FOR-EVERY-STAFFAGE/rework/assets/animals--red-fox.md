# animals/red-fox — rework report

## Pass 1 (second round), 2026-11-22

Status: **partial**. The girdles are on the lateral rest placement, the 1:50 skeleton has a closed load path, the shoulder is rebuilt at 1:50, 1:100 and 1:200, the pose `curled-sleeping` is repaired, and everything is published. `r3-validate.ts animals/red-fox --volumes` and `--catalog-only --volumes` both end `VALID … schema 0, validateAsset 0, checkVolumes 0`. Head, trunk, hind legs, paws, the thermal list, the 2D drawings and the coarse skeletons were looked at but not re-authored; not every pose was read after the last change (list under "Remaining").

State found: no earlier report for this asset (`rework/07.md` lists it as "not started"). Source `volumes/animals/red-fox.json` and catalog `volumes` were identical. The catalog asset was `VALID` without `--volumes`; with `--volumes` it had five `volumes-connected` findings that nobody had run into (tail 19 mm from the sacrum, rib heads 3.6 mm from the vertebrae, fore limbs without any load path to the trunk, ear cartilages 2.2 mm from the skull, at 1:50, 1:100 and 1:500).

### Reference (Vulpes vulpes, adult, 6 kg, autumn coat; own knowledge, no source consulted)

| Measure | Real | Model before | Model after |
|---|---|---|---|
| Head and body / tail / total | 0.62–0.72 / 0.35–0.45 / 1.00–1.15 m | 0.66 / 0.40 / 1.06 m | unchanged |
| Shoulder height (withers, with coat) | 0.35–0.42 m | 0.41 m (0.44 with coat) | unchanged |
| Hip joint / stifle / hock above ground | 0.36–0.38 / 0.25–0.27 / 0.12–0.13 m | 0.375 / 0.257 / 0.123 m | unchanged |
| Shoulder joint / elbow / carpus above ground | 0.31–0.33 / 0.20–0.22 / 0.09–0.10 m | 0.328 / 0.217 / 0.099 m | unchanged |
| Femur / tibia / tarsus + metatarsus / hind digits | 0.125–0.135 / 0.140–0.150 / 0.085–0.095 / 0.05 m | 0.130 / 0.141 / 0.088 / 0.055 m | unchanged |
| Scapula / humerus / radius / carpus + metacarpus / fore digits | 0.090–0.100 / 0.118–0.128 / 0.115–0.125 / 0.062–0.068 / 0.05 m | 0.095 / 0.120 / 0.118 / 0.066 / 0.050 m | scapula 0.097 m, rest unchanged |
| Joint order fore limb | scapula – humerus – radius/ulna – carpus/metacarpus – digits | right | unchanged |
| Joint order hind limb | hip bone – femur – tibia – tarsus/metatarsus – digits | femur hung directly from the root bone | hip bone – femur – … |
| Scapula on the rib cage | dorsal border about 22–25 mm, shoulder joint about 42–46 mm beside the median plane (blade about 12° out of the sagittal plane) | sagittal bone at ±30 mm, humerus stepped out to ±44 mm by a `z` offset | bone from ±24 mm to ±44 mm, `abductionDeg` ∓11.85, humerus at its tip without offset |
| Hip bone | sacroiliac joint about 15–18 mm, hip joint about 30–36 mm beside the plane; os coxae about 100 mm long | no bone; ilium and ischium solids on the root bone, 126 mm long | bones `ilium-hn` / `ilium-hf` 51.5 mm from [0.040, 0.027, ∓0.018] to the hip joint [0, 0, ∓0.036]; os coxae 101 mm |
| Pelvic canal height (sacrum to floor) | 35–40 mm | 52 mm | 41 mm |
| Rib cage breadth (bone) | 90–100 mm | 120 mm | 98 mm |
| Thoracic spinous processes | 26–36 mm at T1–T5, leaning back, anticlinal T11 | short stubs of equal height | 28 / 34 / 36 / 34 / 30 / 26 / 22 / 18 / 15 / 13 / 13 / 13 / 13 mm, leaning back to T10, upright T11, forward T12–T13 |
| Skull length / ear | 0.14–0.15 / 0.085–0.095 m | 0.155 / 0.09 m | unchanged |

The sagittal chain (segment lengths, joint order, both sides alike) was right before this pass; the defect of the rig was the missing girdle.

### Defects found

- Rig: no pelvic girdle bone, scapula faked beside the plane by `z` offsets.
- Structure 1:50: pelvis 126 mm long with a pubis rod as only floor and a joint ball floating in the pelvic canal; rib cage 120 mm broad inside a 150 mm trunk; rib heads, tail and ear cartilages not touching what carries them; the fore limbs had no load path to the trunk at all; thoracic spinous processes as equal stubs.
- Body 1:50 / 1:100 / 1:200: the shoulder was one loft bent around the shoulder joint, which gave a slab with a hard flap edge in front and three-quarter view.
- Pose `curled-sleeping`: fore legs twisted out sideways (humerus twist −38° / −48°) with the fore paws 21 and 33 mm away from the ground plane of the hind paws.

### Rebuilt

- **Rig** (`rework-red-fox-girdle.py`): new bones `ilium-hn`, `ilium-hf` (parent `pelvis`, `abductionDeg` ∓20.45, joint ±2°); thighs re-parented without moving (`angleDeg` 81.53, `abductionDeg` ±3.14, `twistDeg` ±20.22); scapulae from the dorsal border to the shoulder joint with `abductionDeg` ∓11.85; humeri re-parented without moving. Largest rest-frame change of any other bone: 0.04 mm. The humerus offsets of the eleven poses that swing the scapula were re-solved so the limb stays in the sagittal plane (no `lateral-ambiguous`). No clip key overrides these bones. 31 bones.
- **Structure 1:50**: scapula blade, spine and acromion in the new frame; per side ilium wing, acetabulum, ischium with tuber, pubis, ischial arch and a sacroiliac joint ball on the hip bone; pelvic symphysis; `joint-pelvis` removed; rib heads to the vertebral bodies and the rib cage narrowed to 98 mm (all scales); thirteen thoracic spinous processes; two caudal vertebrae in the croup; ear cartilage roots into the skull; `near-/far-serratus-sling` from the scapula to the third rib (material `red-fur` for want of a muscle material) as the load path of the fore limb. 211 solids.
- **Structure 1:100**: sacrum plus one hip bone per side on the new bones, caudal root, sling, ear roots, rib heads. **1:200 / 1:500**: `pelvis-girdle` re-shaped under the croup with `members: ["ilium-hn", "ilium-hf"]`, scapulae in the new frame, rib heads.
- **Body and thermal 1:50 / 1:100 / 1:200** (`rework-red-fox-body.py`): `near-/far-shoulder-blade` on the scapula bone and `near-/far-shoulder` on the humerus as two overlapping rounded lofts; thermal twins `th-…-shoulder-blade` in `coat-trunk`.
- **Pose** (`rework-red-fox-pose.py`): `curled-sleeping` fore legs folded forward on the ground, elbows at 22 mm, carpus and paws at 14 mm above the ground of the hind paws, no twist.
- **Decisions** (`rework-red-fox-decisions.py`): body and structure texts of all four scales, en and de.
- **2D**: nothing redrawn; `volumes-silhouette` passes with the new shoulder and `r3-2d` at 1:50 draws all three views.

Scripts, in the order they were run: `rework-red-fox-girdle.py --apply`, `rework-red-fox-skeleton.py`, `rework-red-fox-body.py`, `rework-red-fox-pose.py --apply`, `rework-red-fox-decisions.py`, then `volumes-insert.py`. `rework_red_fox_linear.py` is their vector helper (the host python has no numpy).

### Rendered and read

- 1:50 body: six views before and after, three-quarter and front close-ups before the shoulder change; structure: six views, top, right; poses `pounce-flight` and `curled-sleeping`; clip `trot-cycle` at 0.1 s; 2D at 1:50.
- 1:100 and 1:500 structure after the change.
- Rendered but not read: 1:50 thermal with `--cut`, 1:100 and 1:500 body, `mousing-crouch`, the structure three-quarter after the symphysis change, `curled-sleeping` after the last 4° change of the forearm.
- Not rendered: 1:200 in any list, thermal at 1:100 / 1:200 / 1:500, poses `pounce-land` and the ten trot phases as poses, clip `mousing-pounce`, 2D at 1:100 and 1:200.

### Remaining (for the next pass)

- 1:50 body, head: no cheek ruffs in the solids although the decisions text names them; muzzle and skull not checked against the reference in close-up.
- 1:50 body, hind leg: stifle 137° and hock 154° read straight ("post-legged"); a standing fox has about 125–135° and 135–145°. This is a rig rest-angle change with all poses and 2D shapes behind it.
- 1:50 body, trunk, neck, tail, paws: looked at in the sheets only, not measured section by section.
- 1:50 structure: skull, mandible, cervical and lumbar vertebrae, long bones, carpus/tarsus and digits untouched (digits are one loft per paw, no carpal or tarsal bones); scapula and ilium blades are 8.4 mm thick to stay above the 7.5 mm feature (real 2–3 mm).
- 1:100 structure: ribs 7–13 end free (no costal arch at this scale).
- 1:500 structure: still a thinned 1:50 parts list (5–6 mm ribs, 8 mm scapula at a 75 mm feature) that passes through `slender`; it needs to be re-modelled as a handful of members.
- 1:100 / 1:200 / 1:500 body and all thermal lists: not re-authored, thermal not read at any scale.
- Pose `curled-sleeping`: hind toes still dip a few millimetres below the ground; the pose is a sternal rest with the fore legs stretched forward rather than a tight curl.
- 2D: no drawing was redrawn; the new shoulder blade mass has no shape of its own (it lies inside the chest outline).
- `rework-red-fox-girdle.py` rebuilds the pelvis and scapula solids from the source; run `rework-red-fox-skeleton.py` after it, never alone.

### Fundamental issues (outside this asset)

- Print suitability: with the forearm of `curled-sleeping` exactly level the product refused the whole 1:200 representation (`PRINT nominal-feature … forearm-n is 0.182 mm`, the same 0.18 mm at 1:150, 1:200 and 1:350, i.e. the pen width, not a size that scales); with the forearm 4° off level the same shapes pass. The measure seems to collapse a `solid` extruded shape to its stroke when the bone is level in a pose that is turned in plan. I avoided it by posing the forearm 4° down from elbow to carpus, which is anatomically fine, but the measure should be looked at.
- `r3-validate.ts` without `--volumes` reported the asset `VALID` while `--volumes` had five connectivity findings; batch reports that quote only the short form can be wrong.
- No muscle or tendon material exists in the asset materials; the serratus sling uses `red-fur`.
- The host python has no numpy, although `rig-lateral-rest.md` points authors at scripts in that style.
