# `humans/person-on-crutches` — rework report

## Pass 1 (second round), 2026-11-22

Status: **partial**. Published and `VALID`; the shoulder and pelvic girdles of the rig are not on lateral rest placement because `rig-lateral-rest.md` did not exist at the start, in the middle or at the end of this pass.

Chisel: `rework-person-on-crutches-skeleton.ts` (every number stated by hand; reads `rework-person-on-crutches-before.json`, the source as it was before this pass, and writes `volumes/humans/person-on-crutches.json`; a second run reproduces the published source byte for byte). It uses the constructors of `rework-04-kit.ts`.

### State found

Source and catalog `volumes` were identical. The catalog asset was `INVALID` (validateAsset 64, checkVolumes 70): 64 structure solids of 1:100 to 1:500 below the smallest feature (the 1:50 long bones repeated at every scale), laces floating above the sneaker, mandible, scapulae, patellae, fibulae and at 1:100 both arms not connected to the rest of the skeleton, the pelvic ring of 1:200 21 mm away from the spine.

### Reference (man, 1.80 m barefoot, 1.82 m in shoes, 79 kg) against the model

| quantity | reference | model before | verdict |
|---|---|---|---|
| stature in shoes | 1.80 m + 0.02 m sole | 1.82 m | right |
| hip joint height | 0.530 H + sole = 0.974 m (Drillis-Contini) | 0.977 m | right |
| thigh, hip to knee | 0.245 H = 0.441 m | 0.446 m | right |
| shank, knee to ankle | 0.246 H = 0.443 m | 0.436 m | right |
| ankle height | 0.039 H + sole = 0.090 m | 0.095 m | right |
| shoulder joint height | 0.818 H + sole = 1.492 m | 1.499 m | right |
| upper arm | 0.186 H = 0.335 m | 0.3385 m | right |
| forearm | 0.146 H = 0.263 m | 0.2657 m | right |
| hand | 0.108 H = 0.194 m | 0.1966 m | right |
| head, chin to vertex | 0.130 H = 0.234 m; breadth 0.155 m, length 0.195 m (DIN 33402-2) | 0.241 m, 0.154 m, 0.198 m | right |
| shoulder joint centres apart | biacromial 0.40 m, joint centres 15 to 20 mm inside each acromion: 0.36 to 0.37 m | 0.403 m | **20 mm too wide per side** (open, girdle pass) |
| hip joint centres apart | 0.17 to 0.18 m | 0.192 m | about 8 mm too wide per side (open, girdle pass) |
| bi-iliac breadth, ASIS apart | 0.28 to 0.29 m, 0.24 m | ilia were two discs of 0.13 m | wrong, rebuilt: 0.284 m, 0.24 m |
| lumbar spine, thoracic spine | 0.19 m, 0.28 to 0.30 m | one smooth rod with a pinch | wrong, rebuilt: 0.196 m, 0.291 m |
| mid-thigh, knee, calf girth | 0.52 to 0.56 m, 0.37 m, 0.37 m | 0.51 m, 0.36 m, 0.37 m | right |
| forearm crutch | 22 mm upper tube, 19 mm telescopic lower tube, cuff 50 to 60 mm below the elbow, handle 0.82 to 0.86 m above the tip | same dimensions, but both tubes were tessellated dumps (`polyhedron`, 40 kB and 260 kB per side) | form right, modelling wrong: restated as turned parts |
| aluminium surface of two crutches | 2 x (0.48 m x pi x 22 mm + 0.38 m x pi x 19 mm + 0.23 m x pi x 22 mm + joint) = 0.156 m2 | `areaM2` 0.25435 (the dump counted hidden and inner faces) | wrong, set to 0.156 |

The sagittal chain of the rig (pelvis, torso, neck, head, arm, forearm, hand, thigh, shank, foot, toe, crutch; both sides identical) has the right joint order and segment lengths, so it was left as it is. Body and thermal lists read as the same young man in hooded sweatshirt, shorts, sneaker and walker boot from all six directions at every scale and in every pose and clip instant I rendered; they were not rebuilt.

### Rebuilt

- **Structure 1:50** (174 to 260 solids). Skull: braincase loft, facial skeleton loft, zygomatic arches, mandible of two rami and a curved body. Spine: C1 to C7, T1 to T12, L1 to L5 as single bodies on a disc column along lordosis and kyphosis, arches, spinous and transverse processes; the thoracic bodies sit at the levels of their ribs. Clavicles S-curved from the sternum to the acromion; scapulae as blades on the ribs with spine, acromion and glenoid on the head of the humerus. Pelvis: iliac blades from the anterior to the posterior superior spine, acetabula round the femoral heads, pubic and ischial rami with the ischiopubic ramus, symphysis, sacrum with coccyx. Forefoot: tarsus, five metatarsals, five toes instead of two wedge plates. Patella and fibula head moved into contact. Ribs, sternum, long bones, hands kept.
- **Structure 1:100** (118 to 141): the same skeleton with every blade and thin bone 15 mm thick, the spinous processes as one crest, the neck as one column, a flat sternum.
- **Structure 1:200** (77 to 58): skull as one loft, spine as one curved column, rib cage as one closed basket, clavicles, scapula blades, iliac blades, acetabula, pubic and ischial bars, sacrum, one bone per limb segment (radius and ulna merged), nothing thinner than 30 mm.
- **Structure 1:500** (66 to 37): skull, spine, rib basket, two clavicle bars to the shoulder joints, the pelvis as one basin, one bone per limb segment, nothing thinner than 75 mm. The loose rib wires, the bar through the shoulders and the bar through the hips are gone.
- **Body, all scales**: crutch tubes as two revolves per side (22 mm and 19 mm); the 5 mm adjustment holes are below every feature size and left out; laces set onto the tongue (1:50). Spring button dropped from 1:100, clamp collar and cuff arm from 1:200 (below the feature, not slender), with their thermal and structural references.
- **Thermal**: `thermal.regions[crutch-frame].areaM2` 0.25435 to 0.156 in the catalog file (hand calculation above; measured 0.149 m2 at 1:50).
- **Decisions** en and de rewritten for crutch and structure at every scale.
- **2D**: the body silhouette did not change; `r3-2d` at 1:50 read after the last change, drawings and model agree.

### Proof

`r3-validate.ts humans/person-on-crutches --catalog-only --volumes`: `VALID humans/person-on-crutches (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0`.

Rendered and read after the changes: structure 1:50 (back, right, front, two three-quarter views, poses `crutches-forward` and default), structure 1:100, 1:200, 1:500 full sheets, body 1:50 three-quarter, pose `body-swing`, clip `three-point-gait` at 1.3 s, body 1:500, 2D 1:50. Rendered but not read after the last change (forefoot rays): structure in `body-swing`, clip `rest-on-crutches`, thermal 1:50, body 1:200.

### What remains imperfect

1. Rig: shoulder/pelvic girdle on lateral rest placement (clavicle and scapula bones, pelvis halves); shoulder joint centres 20 mm too far out per side, hip joint centres about 8 mm; sleeves, crutches, 2D front and plan shapes, anchors and thermal regions have to follow when the breadths are corrected.
2. Rig: the hands have no finger bones (the grip is modelled closed on the hand bone, both sides alike); the cuff and cuff arm hang on the forearm while the shaft hangs on the hand, so they only agree while the wrist stays at 33 degrees, as in every pose and clip of the asset. The crutch should be one rigid bone under the forearm.
3. Rig: one torso bone from L5 to C7; no lumbar/thoracic split.
4. Body 1:50: the sleeve heads stand as separate caps above the shoulder line of the sweatshirt (front and back view), the seam between left thigh and shank is ragged at the knee, the hem of the sweatshirt is cut by the raised left thigh.
5. Structure 1:50: skull without orbits and nasal aperture (a solid cannot be subtracted, a dark patch would read as goggles); ribs without separate costal cartilage; carpals as one ellipsoid.
6. Structure 1:100: the foot keeps metatarsus and toes as two plates.
7. Thermal lists of all scales and body lists of 1:100 to 1:500 were looked at once and judged right, not re-authored section by section.

### Fundamental issues

1. `🧪️tests/🧊️model-skeleton` ("adultLongBones", subject `person-on-crutches`) demands `femur`, `tibia`, `humerus`, `radius`, `ulna` in the structure list of every scale at their 1:50 section; `volumes-feature` forbids exactly that below the feature size. This asset now follows the validator (`radius-ulna-l` at 1:200 and 1:500, floored shafts), so that test fails for it until rule and test are reconciled. Not run (no `repo` binary), not changed.
2. `validator.md` has no section "Rule changes authors must know" (searched twice); the rule changes were read from its "Connectivity and per-scale feature" section.
3. `rig-lateral-rest.md` did not appear during the pass.
