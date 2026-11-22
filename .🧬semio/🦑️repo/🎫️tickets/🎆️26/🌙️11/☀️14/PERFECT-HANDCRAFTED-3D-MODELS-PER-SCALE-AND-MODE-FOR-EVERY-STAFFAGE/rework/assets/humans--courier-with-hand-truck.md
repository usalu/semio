# humans/courier-with-hand-truck — author report

Earlier work: `rework/06.md`, section 2 (shoulder yoke, sleeve caps, plate hip bones; rig, coarse scales, poses untouched).

## Pass 1 (second round) — 2026-11-22

State found: source `volumes/humans/courier-with-hand-truck.json` identical to the catalog `asset.volumes`. `r3-validate.ts --volumes`: INVALID, 1 + 8 findings (snap ring at 1:100, open load paths of the skeleton at 1:50 / 1:200 / 1:500, glove off the forearm at 1:200, axle off the frame at 1:500).

### Reference (male courier, stature 1.81 m with 30 mm sole, about 85 kg; sack truck after fetra B1125L)

| Measure | Reality (Drillis-Contini / DIN 33402-2 P50 male scaled to 1.78 m barefoot) | Model |
|---|---|---|
| Head height chin to vertex | 0.235 to 0.24 m (1/7.5) | 0.234 m |
| Shoulder joint height | 0.818 H = 1.48 m | 1.491 m (10 mm high, kept: every pose and clip is solved on it) |
| Hip joint height | 0.530 H = 0.96 m | 0.957 m |
| Knee / ankle height | 0.285 H = 0.515 m / 0.039 H + sole = 0.08 m | 0.514 / 0.080 m |
| Upper arm / forearm / hand | 0.186 / 0.146 / 0.108 H = 0.337 / 0.264 / 0.195 m | 0.337 / 0.265 / 0.194 m |
| Thigh / shank | 0.245 / 0.246 H = 0.443 / 0.445 m | 0.444 / 0.434 m |
| Clavicle length | 0.15 to 0.17 m | was a 0.19 m straight bar to the joint centre; now 0.174 m sternoclavicular [0.076, 1.478, ±0.022] to acromioclavicular joint [-0.008, 1.513, ±0.170] |
| Biacromial breadth | 0.40 m | acromion tips ±0.187 m (0.37 m bone), jacket 0.40 m |
| Shoulder joint centres | 0.36 to 0.37 m apart | 0.37 m |
| Hip joint centres | 0.17 to 0.19 m apart | 0.19 m |
| Bi-iliac breadth (bone) | 0.28 to 0.29 m | was about 0.23 m, now 0.288 m |
| Scapula height / breadth | 0.16 / 0.105 m | 0.162 / 0.075 m blade plus neck to the glenoid |
| Proximal / middle / distal phalanx, index | 0.040 / 0.023 / 0.018 m (glove adds 3 to 5 mm) | 0.035 / 0.029 / 0.021 m (taken from the glove fingers as modelled) |
| Truck: height / wheel / track | 1.15 m / 0.26 m / 0.50 m | 1.15 / 0.26 / 0.50 m |

### Defects found

- Rig: no shoulder girdle and no pelvic girdle (arms and thighs hung from torso and pelvis beside the plane by `z` alone), no finger bones (fingers were solids on the hand, a free hand stayed a fist), torso one bone from L5 to C7. Segment lengths, joint order and left/right symmetry were right (table).
- Structure 1:50: clavicle 0.19 m to the joint centre, scapula a loose tilted plate 12 mm off the ribs with no spine, acromion or glenoid, hip bone two small plates giving 0.23 m bi-iliac breadth, mandible three bars 3 mm off the skull, cranium 9 mm above the atlas, patella 3 mm off the condyles, finger bones as three bars across all four fingers.
- Structure 1:200 / 1:500: one bar through both shoulders, one bar through the pelvis (26 mm off the spine), long bones thickened to the feature (30 / 75 mm) against rule 2 of `validator.md`, rib hoops of 8 mm at 1:500.
- Body: sleeve head poked 10 mm above the yoke; 1:200 glove did not reach the forearm; 1:500 handle was a 112 mm sausage and nothing joined rails and axle.

### Rebuilt (scripts `rework-courier-with-hand-truck-girdles.py`, `rework-courier-with-hand-truck-hands.py`; the numbers in them are hand-chosen world rest points [ahead, up, left])

- Rig (54 bones, was 22): `clavicle-l/-r` on `torso` and `ilium-l/-r` on `pelvis` on the lateral rest placement of `rig-lateral-rest.md` (solved with the same `placed` / `fields` as `rig-rest.ts --solve`); `arm-*` and `thigh-*` re-parented without moving (checked to 3e-5 for every old bone, and with `rig-rest.ts`: arm and thigh frames unchanged). 28 finger bones (`index|middle|ring|little-proximal|middle|distal-*`, `thumb-proximal|distal-*`), rest placement = the grip around the bow handle, so the four poses and two clips keep their grip without any re-solving; pose `parked-upright` opens the free left hand by offsets (38 / 42 / 18 degrees, thumb 15).
- Structure 1:50: S-curved clavicle (loft), scapula blade of seven sections with spine, acromion, neck and coracoid, all on the clavicle bone; hip bone on the ilium bone: ala (five sections), auricular plate to the sacrum, acetabulum, superior pubic ramus, ischium, ischiopubic ramus, symphysis; mandible of ramus and body plates with chin, temporal bones and temporomandibular joints, occipital base on the atlas; 28 phalanges on the finger bones; patella on the condyles. Load path closed: hand, humerus, glenoid, acromion, clavicle, sternum and femur, acetabulum, ilium, sacrum, spine.
- Structure 1:100 / 1:200 / 1:500: girdles re-modelled per feature size on the same bones (1:100 six + four solids per side, 1:200 three + four, 1:500 clavicle and one hip bar per side plus sacrum); humerus, forearm, femur, tibia back at the true shaft with `slender: true`; 1:500 rib hoops replaced by one rib cage volume.
- Body: sleeve heads lowered 10 mm at every scale (body and thermal shells); 1:200 forearm reaches into the glove; 1:500 handle 75 mm, wheel brackets `strut-l/-r` between rail and axle (body, thermal, structure); snap ring left out at 1:100; glove fingers bound to the finger bones (thermal follows by `of`).
- Decisions: one paragraph per scale (en, de) on what was modelled.

### Validation

`r3-validate.ts humans/courier-with-hand-truck --catalog-only --volumes` → `VALID humans/courier-with-hand-truck (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0` (after the last change).

Rendered and read after the changes: structure 1:50 (six views, back, two three-quarter views), 1:200, 1:500; body 1:50 `waiting` (before), `walking-contact`, `parked-upright` (six views, front, left), clip `tilt-and-settle` t 4 s, 1:500 body; thermal 1:50 cut; 2D 1:50 `parked-upright`. Rendered but not read: `tilted-back`, `push-walk` t 0.3 / 0.9, `tilt-and-settle` t 1.5, body 1:100 / 1:200, structure 1:100. Not rendered after the change: thermal 1:100 / 1:200 / 1:500, 2D at 1:100 / 1:200 / 1:500.

Incident: my second script wrote the catalog file with a one-character offset error (`"rig": {{`, `"poses": [[`); the file was invalid JSON for about a minute, then repaired by hand and the script corrected. Nothing else in the file was touched (`rig`, `poses`, `volumes` only).

### What remains imperfect

- Rig: torso is still one bone (no lumbar / thoracic split, no cervical segments); scapula rides rigidly on the clavicle; finger rest placement is the grip, not the open hand; finger lengths follow the glove solids (index proximal 35 mm against 40 mm). Shoulder joint 10 mm high.
- Structure 1:50: spine is one smooth loft without vertebrae, sacrum a wedge, facial skeleton a mask without orbits or nasal opening, ribs without costal cartilage distinction, hand without separate metacarpals and carpals, foot as two lofts. 1:100 skull is two ellipsoids.
- Body 1:50: I did not re-author torso, limbs, head, boots or the hand truck; they read plausibly in all poses I looked at, but the jacket is a straight tube in side view (no chest / shoulder-blade modelling), ears are flat discs, the face is schematic, the seat shows as a brief-like panel under the hem from behind.
- Body, thermal 1:100 / 1:200 / 1:500: still the thinned 1:50 section rows of batch 06 except the parts named above; not re-modelled per feature size. 1:500 boots are boxes.
- Thermal: only followed the body changes; areas against `areaM2` not re-measured by me (validator passes).
- 2D: not redrawn. The free hand of `parked-upright` is drawn as a fist while the 3D hand is now half open; `dimensions.widthM` 1.579 / `depthM` 0.578 were not checked.
- Hand truck: taken over as found (plausible: tyres, rims, axle, struts, toe plate, bow handle); no closer review of its hardware.
