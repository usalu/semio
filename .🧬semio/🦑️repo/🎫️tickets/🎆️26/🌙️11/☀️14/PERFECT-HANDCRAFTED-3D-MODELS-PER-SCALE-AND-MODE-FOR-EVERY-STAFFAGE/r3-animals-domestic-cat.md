# Round 3 — `animals/domestic-cat` (2026-11-19)

Two authors worked on this. The first one (afternoon, stopped by a usage limit) rebuilt the rig, anchors, poses, all three authored scales and the 2D redraw. The second one (evening) resumed: it re-ran validation and the check, looked at every list, scale and pose, and fixed what was still wrong (head and jaw, eyes, ears, the sitting pose, the stale 2D decisions). It then re-spliced, re-validated and wrote this report.

Final state: `r3-validate.ts animals/domestic-cat` gives `VALID (schema 0, validateAsset 0)`, and `volumes-check.ts animals/domestic-cat --product` gives `PASS` at 1:50, 1:100 and 1:200. 1:500 is `unavailable` by design: the cat is 1.5 mm long on paper.

Rebuild from the hand tables: `sh r3-domestic-cat-run.sh`, then `python3 r3-domestic-cat-apply.py`. The run script writes the rig and poses (`r3-domestic-cat-rig.py`), 1:50 (`r3-domestic-cat-50.ts`), 1:100 and 1:200 (`r3-domestic-cat-coarse.ts`), the 2D redraw (`r3-domestic-cat-draw2d.ts` + `r3-domestic-cat-rep-decisions.json`) and the candidate, then validates the candidate. `r3-domestic-cat-apply.py` writes the root members into the catalog file and splices the volumes.

## 1. Reference: a 4.2 kg short-haired domestic cat standing alert

| Measure | Real value | Model (rig / 1:50 body) | Source |
|---|---|---|---|
| Mass | 3.5–5 kg (female ~3.5, male ~4.5) | 4.2 kg (`dimensions.massKg`) | veterinary standard values; provenance |
| Height at withers | 0.23–0.26 m | withers (T1 spine, `t7`/`ct` landmarks, dorsal coat) 0.25 m | breed standards for European / domestic shorthair |
| Hip joint height | ~0.93 × withers | 0.235 m | Goslow et al. 1973 (hind limb geometry); stance radiographs |
| Head + body length (nose to tail root) | 0.46 m (0.40–0.50) | 0.47 m | general zoology tables |
| Tail length | 0.25–0.30 m | 0.30 m (5 × ~0.06 m bones) | same |
| Overall length incl. tail, standing | 0.75–0.80 m | 0.77 m (`widthM`) | same |
| Height incl. ears, head up | 0.33–0.37 m | 0.36 m (`heightM`) | same |
| Body width (chest / hips) | 0.11–0.13 m | 0.13 m (`depthM`), trunk half width 0.0635 | same |
| Skull length (condylobasal) | 85–95 mm | head bone 89.5 mm poll to nose | Feline skull osteometry (Hildebrand; vet anatomy texts) |
| Zygomatic width | 65–72 mm (≈ 0.75 × skull length) | head loft max 81–82 mm with fur and cheeks | same |
| Eye: palpebral fissure | 15–17 mm wide, 10–12 mm high, centres ~32–38 mm apart | 13.2 × 10.2 mm, centres 39 mm apart | feline ophthalmology tables; limited by the area rule (see 6) |
| Ear pinna | 45–60 mm long, base 35–45 mm, set wide on the crown, turned forward-outward | 54 mm long, base 43 mm, 26° outward | cat morphology |
| Femur / tibia | 95–110 / 100–115 mm | 98 / 104 mm | Hoy & Zernicke 1985, cat segment data |
| Metatarsus + hind paw | 65–75 / 30–35 mm | 70 / 34 mm | same |
| Scapula / humerus / radius | 60–70 / 90–100 / 80–95 mm | 64 / 90 / 84 mm | same |
| Metacarpus + fore paw | 25–40 / 25–30 mm | 40 / 30 mm | same |
| Scapula angle (standing) | ~55–60° to horizontal | 55° | stance kinematics (Muybridge 1887; Halbertsma 1983) |
| Humerus angle (standing) | ~40–50° below horizontal, elbow behind the point of shoulder | 40° | same; the chain shoulder → ground with the lengths above forces it |
| Shoulder joint angle | 95–110° | ~95° | same |
| Hind stance | stifle well forward, hock 0.085 m above ground, metatarsus near vertical | stifle (0.047, 0.149), hock 0.085 m, metatarsus 100° | Goslow et al. 1973 |
| Gait | lateral-sequence walk, duty factor 0.6–0.7 | 0.625 (walk), 0.75 (stalk) | Halbertsma 1983; Leyhausen 1979 |
| Sitting | sitting on the ischial tuberosities, hip joint 4–6 cm up, forelegs straight, tail laid round the paws | hip 0.051 m, rump on the ground, forelegs nearly straight, tail along the ground | behaviour references (Leyhausen 1979) |

## 2. Audit (against the reference)

Pre-round-3 asset (`🗑️generated/r3-domestic-cat/asset-before.json`), audited by the first author and re-measured by the second:

- **Rig, bone lengths:** pelvis 0.060 m (real sacral–iliac segment ~0.047). Lumbar 0.085 m: real lumbar spine ~0.13 m, so the trunk was short and pony-like. Femur and tibia 0.110 m (too long against a 0.25 m withers). Humerus 0.080 m (short). Scapula 0.075 m. Head 0.100 m (long, dog-like muzzle). Hind paw 0.040 m and fore paw 0.036 m.
- **Rig, angles and joints:** the scapula lay at 62° from the trunk axis with the humerus at -73°, which put the elbow below the shoulder and gave a straight post-leg. The thigh at -45° put the stifle behind the hip, so the hind leg read like a dog's. The neck was set at 30° and the head at -30°: a long, low head carriage instead of the alert raised head. The thigh joint was 0.03 m below the pelvis origin (hip joint not on the acetabulum).
- **`dimensions`:** correct against the reference (0.36 × 0.77 × 0.13 m, 4.2 kg), unchanged.
- **2D drawings:** fitted to the old rig (long head, short trunk, dog-legged hind limbs). The front and plan trunks and necks were revolved `solid` profiles that did not match the body.
- **Solids (old):** 51 / 50 / 25 body solids at 1:50 / 1:100 / 1:200, fitted to the old 2D. The 1:100 list was nearly the 1:50 list.
- **After the first author (seen by the second):**
  - (a) The jaw was a separate slab under the whole head (mandible bottom 8–9 mm below the skull from s = 0.02 m to 0.08 m), which read as a long beak-like mouth line in every side view.
  - (b) The face narrowed from 81 mm to 51 mm between s = 0.038 and 0.065 m, giving a rodent or fox face in front view.
  - (c) The eyes were 10 × 12 mm discs: too small.
  - (d) The ears were 38 mm wide at the base and 56 mm long (ratio 0.68), tilted 19° outward: fox or bat ears.
  - (e) Sitting pose: the forepaws hung 14 mm above the ground (the solver reported `unreachable by 0.014`). The rump contact was 27 mm above the ground. The tail tip curled up through the space in front of the paws.
  - (f) The representation `decisions` still described the old drawings ("three trunk masses", "revolved solids").
  - (g) The thermal eye area had to stay within ±30 % of the declared 0.0005 m² (C29).

## 3. Root fixes

**Rig** (`r3-domestic-cat-rig.py`, from the landmarks in world metres, hip joint above x = 0). Old → new lengths, all in m:

| Bone | Old | New |
|---|---|---|
| pelvis | 0.060 | 0.0474 |
| lumbar | 0.085 | 0.1331 |
| thorax-a | 0.070 | 0.0677 |
| thorax-b | 0.070 | 0.0662 |
| neck | 0.080 | 0.0716 |
| head | 0.100 | 0.0895 |
| jaw | 0.055 | 0.0636 |
| ears | 0.050 | 0.055 |
| tail | 0.06/0.06/0.06/0.055/0.045 | 0.064/0.064/0.063/0.059/0.050 |
| thigh | 0.110 | 0.098 |
| shank | 0.110 | 0.104 |
| metatarsus | 0.075 | 0.070 |
| hind paw | 0.040 | 0.034 |
| scapula | 0.075 | 0.064 |
| humerus | 0.080 | 0.090 |
| forearm | 0.085 | 0.0841 |
| metacarpus | 0.040 | 0.040 |
| fore paw | 0.036 | 0.030 |

- **Joints:** the thigh now starts at the hip joint (0, 0) instead of (0.015, -0.03). Scapula, humerus and elbow follow the 55° / 40° stance angles. The ear bases sit on the crown at ±0.024 m lateral. The bone masses got new centres of mass.
- **Anchors:** paw contacts, rump, nose, mouth and back plume were recomputed on the new bones.
- **Poses:** all 21 poses were re-solved on the new chain. Axial changes from standing are kept, planted paws are flat on the ground, and lifted paws keep their lift. Joint limits were widened only to enclose the poses (see the `LIMIT` lines of the rig script).
- **Second author, sitting pose:** `TWEAK["sitting-tail-wrapped"]` = hip 0.184 m lower than standing (hip joint 0.051 m, rump contact on the ground), scapula 8° steeper (forepaws now reach the ground) and new tail angles (tail-1 +13°, tail-2 to tail-4 -11/-9/-8°, tail-5 -22°), so the tail lies on the ground along the paws with a slight lift at the tip.
- **Materials:** `bone` added (density 1900 kg/m³) for the structure list.
- **2D, ruling E1:** every shape of every view at 1:50, 1:100 and 1:200 is the traced orthographic view of that scale's body in standing-alert. Ids, roles, bones, z order, tones and regions are kept. `rump` was added on the pelvis (all three views, 1:50 and 1:100). At 1:200 front, `tail-a` and `tail-b` were added. The front `tail-tip` and the 1:200 front tail columns (no body part of their own) were removed. Detail strokes (brow, mouth line, inner ear, toes, shoulder and hip lines, stripes, tail ring) were restated by hand. The second author's head changes were redrawn with it.
- **2D decisions:** `silhouette` at 1:50, 1:100 and 1:200 and `thinElements` at 1:200 were rewritten to describe the drawings as they are (`r3-domestic-cat-rep-decisions.json`, en + de).
- **Thermal / structure data outside `volumes`:** unchanged. The region areas (C16, C18, C19, C27, C29) stand, and the model was fitted to them.

## 4. What is modelled, per scale and list

All sections are stated by hand in tables: trunk rows `[x, centre height, half height, half width, exponent]`, limb rows in bone frames.

**1:50** (feature 7.5 mm): body 74, thermal 77, structure 109 solids.
- **Body:**
  - Trunk: four overlapping lofts (rump, loin, hind and fore ribcage) with a level back, a chest deep behind the elbows and the belly tucked up.
  - Neck: short and thick.
  - Head (second author): a domed skull with the stop and jowls that cover the mandible back to the mouth corner below the eye. The face stays broad at eye level (79 → 71 → 57 mm at s = 0.05 / 0.058 / 0.065 m). It carries a short muzzle with two whisker pads, the nose leather and forward-facing almond eyes (13 × 10 mm).
  - Ears: cupped, 54 mm long with a 43 mm base, turned forward-outward at 26°.
  - Jaw: mandible with the tongue inside.
  - Tail: five domed tail segments.
  - Legs: per hind leg a flat broad thigh, a shank with the calf, and a metatarsus with the heel of the hock. Per foreleg the shoulder over the blade, an upper arm with the triceps, a forearm with the point of the elbow, and the metacarpus.
  - Paws: each has a body, four toes in their arc, a metapodial pad and four digital pads.
  - Every segment overlaps its neighbours, so poses stay closed.
- **Thermal:** every region is a reference to the body solids (fur layer), plus breath outlets at nose and mouth and the back plume as emitter patches.
- **Structure:**
  - Skull and mandible, and the cervical spine in its S curve.
  - Thoracic spine with the withers processes, 13 slender ribs per side with the costal arch and sternum, and the lumbar spine.
  - Pelvis with the sacrum, five caudal rods and the ear cartilages.
  - Scapulae and all long bones at their true lengths, plus metapodial bundles and paw plates.
  - Joints at the rig joints, supports at the four paw contacts, masses at the bone centres of mass.

**1:100** (feature 15 mm): body 40, thermal 43, structure 76 solids.
- **Body:**
  - Restated with fewer sections: four trunk pieces, and a single-loft head with jowls over the mandible (second author), jaw, eyes and nose.
  - Ears 43 mm at the base.
  - Every limb segment, and one paw and sole per foot.
- **Structure:** the skeleton simplified, with 7 rib hoops per side for 13 ribs.

**1:200** (feature 30 mm): body 23, thermal 34, structure 50 solids.
- **Body:**
  - Two trunk masses (hindquarters with loin, ribcage with chest), neck, and a head with jowls (second author).
  - Jaw, two pointed ears and a two-piece tail.
  - Three pieces per leg.
- **Structure:** one bone per rig bone at its true thickness, with 3 rib hoops per side.

**Final check:**
- `volumes-check.ts animals/domestic-cat --product` → `PASS (9.7 s)`.
- Silhouette IoU at 1:50: elevation 0.984, front 0.98, plan 0.99. At 1:100: elevation 0.979, front 0.973. At 1:200: elevation 0.954.
- Bounds at 1:50: width +10.1 mm (limit 15.4), height 5.5 mm (limit 7.5), depth 3.0 mm.
- Eye area deviation 0.25 at 1:50 (limit 0.3).
- Everything else is `ok`.

## 5. What I saw in the final renders

Sheets: `🗑️generated/r3-sheet/animals/domestic-cat.s{50,100,200}.body.png`; 1:50 and 1:100 `thermal.cut` and `structure`; the poses standing-alert, sitting-tail-wrapped, stretch-stance, play-bow and stretch-deep; walk-cycle t 0.3 and 0.6. Close-ups of front, side and three-quarter (turn 50 and turn 120) are in `🗑️generated/r3-domestic-cat/c-*.png`. 2D: `🗑️generated/r3-2d/animals/domestic-cat.s50.png`.

**Body at 1:50, standing-alert:**
- **Right / left side:** a cat at first sight. Level back, the scapula just breaking the topline at the withers, a deep chest with the elbow behind the point of shoulder, the belly tucked to a slender flank. The hind leg shows the forward stifle and a near-vertical metatarsus on a heel. The raised tail tapers in five pieces. The head is round with a short muzzle and the chin under the whisker pads. The long jaw slab of the afternoon is gone: the mouth line now starts below the eye.
- **Front:** broad round face, eyes on the face front, whisker pads with the nose leather above the chin, ears set wide on the crown, a narrow chest with the forelegs close under it, and the tail tip showing above the head.
- **Back:** rump and two hind legs, tail rising from the midline.
- **Top:** a slender spindle trunk, narrower at the loin than at the ribs, a round head with the ears, and the tail straight back.
- **Three-quarter:** reads as an alert cat. From turn 50 the far ear is seen nearly edge-on, which is plausible for ears turned outward.

**Poses:**
- **Sitting (after the fix):** on its haunches, hind feet flat in front of the thighs, forelegs nearly straight, head level. The tail lies on the ground and runs forward along the paws.
- **Walk t 0.3 / 0.6:** lateral-sequence walk with three paws on the ground and the tail up. Closed at every joint.
- **Stretch-stance, play-bow, stretch-deep:** forelegs reaching forward on the ground, back swayed, tail up. Stretch-deep has the yawn the `stretch` clip describes.

**1:100 and 1:200 bodies:** the same proportions with simpler sections. At 1:200 the muzzle seen from above is a plain cone, which is right for a 30 mm feature.

**Thermal 1:50 (cut):** the regions cover the body exactly (trunk, head, ears, fore and hind limbs, tail, nose, eyes, tongue, pads), and nothing sticks out.

**Structure 1:50 and 1:100:**
- Skull, cervical S curve, rib cage behind the scapula, lumbar spine, pelvis and tail rods inside the body ghost.
- Long bones on the rig, joints at the joints, masses in the trunk.

**2D at 1:50:** side, front and plan read as a cat and match the new head (the redraw follows the body).

## 6. Still imperfect, stated plainly

- **Wrapped tail is planar.** The rig is planar (bone angles in the sagittal plane, with lateral offsets only at the girdles and ears), so in the sitting pose the tail cannot curl sideways round the paws. It lies on the ground along the midline between the paws. A real wrapped tail lies beside the paws. Fixing this needs lateral bone rotation in the rig schema, which this asset cannot add on its own.
- **Seams in strong poses.** In sitting and the deep stretch the scapula masses show a small step over the topline, and the coincident surfaces of shoulder and chest give flecked seams in the shaded render. These are overlaps kept so the poses stay closed, not gaps.
- **Eyes are smaller than life.** Real eyes are about 16 × 11 mm; these are 13.2 × 10.2 mm. The visible disc is bounded by two rules at once: the thermal `eyes` area (0.0005 m², ±30 %, measured on the full ellipsoid surface) and the smallest feature (7.5 mm thickness at 1:50, 7.8 mm at 1:100). A bigger visible eye needs a larger declared corneal area or an exposure-weighted measure. Left for `T/author-requests.md` if wanted; not filed now.
- **Ear in exact side view.** The ears read as narrow blades because they are turned 33° outward. This is anatomically right, but less recognisable than a drawn cat ear.
- **Not a separate muscle per head.** Whiskers and claws are below the 7.5 mm feature and left out. The thigh, calf, triceps and shoulder are each one loft per segment, shaped by their sections, not a separate solid per muscle.
