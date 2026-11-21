# Round 3: `animals/pig` (2026-11-19)

An earlier author started this asset and was cut off at 15:02. They left the rig, anchors and poses applied to the catalog, a draft 1:50 model, and old 1:100/1:200/1:500 lists built on the old rig. The 2D drawings and `dimensions` were not redrawn, so the catalog was INVALID with 8 `extent-*` errors. I resumed from that state, redid the parts that were not good enough, and finished the asset.

Scripts (all in the ticket folder):
- `r3-pig-rig.py`: rig, anchors, poses, `bone` material.
- `r3-pig-io.py`: writes members of the asset file.
- `r3-pig-kit.ts`: placement of the hand tables.
- `r3-pig-50.ts` and `r3-pig-coarse.ts`: every hand table, for 1:50 and for 1:100, 1:200 and 1:500.
- `r3-pig-2d.ts`: E1 redraw into a candidate document.
- `r3-pig-decisions.py`: drawing-decision texts.
- `r3-pig-apply.py`: writes `representations` and `dimensions`.
- `r3-pig-suit.ts`: print-suitability lister.
- `r3-pig-outside.ts`: structure or thermal solids outside the body silhouette.
- `r3-pig-run.sh`: the whole pipeline. It writes the catalog only when the candidate is VALID and printable.
- `r3-pig-bbox.ts`, `r3-pig-frames.ts`, `r3-pig-pose.ts`: helpers.

Input kept for re-runs: `🗑️generated/r3-pig/asset-before.json`, the asset before round 3.

## 1. Reference: 100 kg finishing pig (Large White × Landrace type), standing

Sources:
- EFSA AHAW 2022 welfare of pigs on farm, table of pig dimensions after Petherick (1983): about 0.71 m height of the back at 100 kg.
- Condotta et al. 2018, *Dimensions of the modern pig*, Trans. ASABE (doi 10.13031/trans.12826): modern grow-finish pigs are 10 % lower, 5 % shorter and 15 % wider than the older engineering values. Read only as the abstract (3tres3 summary); the full table was not reachable (403 / captcha).
- Pig skeletal anatomy after Nickel/Schummer/Seiferle and Ellenberger/Dittrich/Baum, plus own knowledge of the surface form.
- The asset's provenance: 100 kg, 0.64 m withers.

These are textbook orders of magnitude, not measurements of one animal.

| landmark / size | reference | model (world, x ahead of the hip joint, y up) |
|---|---|---|
| height of the back | 0.64 to 0.71 m (Petherick 0.71, modern −10 %) | 0.684 (mid back), withers 0.65 |
| length, snout to tail curl | 1.20 to 1.30 m | 1.27 (snout disc 1.113, curl −0.161) |
| width over hams / barrel | 0.30 to 0.35 m (modern +15 %) | 0.331 |
| chest depth, belly height | 0.36 to 0.38 m; belly 0.28 to 0.30 m | 0.39 at the brisket; belly 0.29 |
| hip joint / stifle / hock / hind fetlock | 0.50 / 0.33 / 0.20 / 0.08 m | 0.505 / 0.330 / 0.205 / 0.078 |
| shoulder joint / elbow / carpus / fore fetlock | 0.44 / 0.30 / 0.165 / 0.075 m | 0.445 / 0.305 / 0.165 / 0.075 |
| scapula, inclination | 0.18 to 0.19 m, about 55° | 0.187 m, 56° |
| femur, tibia, humerus, radius | 0.20, 0.185, 0.18, 0.14 m | 0.200, 0.185, 0.178, 0.140 |
| head (poll to disc) | 0.26 to 0.30 m, carried 20 to 30° below horizontal | 0.262 m at −24°, disc to 0.27 |
| lop ear (Landrace type) | 0.15 to 0.20 × 0.10 to 0.12 m, hanging forward over the eyes | 0.18 × 0.11 m, hanging forward at −52° |
| claws | two per foot, 50 to 60 × 20 to 25 × 30 to 35 mm, plus two dewclaws | 52 × 24 × 35 mm, dewclaws 32 × 17 × 18 mm |
| tail | corkscrew of about 1.5 turns, 0.15 to 0.25 m uncoiled | 1.5 turns, hangs at the rump |

## 2. Audit of the state before round 3 (`asset-before.json`)

- **Rig.** The spine chain was the top line of the back, not the vertebral axis: the lumbar, thorax and neck origins all sat on one line 0.115 m above the hip, so the neck base was 0.10 m too high.
  - Lumbar 0.312 m (real 0.245), thorax 0.280 m (real 0.36).
  - Head 0.300 m, carried nearly level.
  - Hoof bones 0.065 m. The real fetlock-to-toe length is 0.09 to 0.095 m, so the fetlocks sat about 0.06 m above the ground instead of 0.075 to 0.08.
  - Fore cannon 0.11 m (real 0.09).
  - Tail bones 0.08 and 0.07 m, with the tail inside the rump.
- **Dimensions.** 0.763 × 1.23 × 0.42 m (height × length × width). The 2D drew the old tall ears; the width counted far legs that splayed outward.
- **2D.** Front and plan were constructed drawings, not projections: ellipses and capsules for the head, legs and ears. Hooves were drawn 110 mm long against a real 52 mm claw. The 1:500 elevation lacked the far legs (`author-requests.md`, entry 06).
- **Solids at the previous author's cut-off** (looked at in a render):
  - Ham and shoulder were slabs with hard edges standing proud of the barrel.
  - Ears were wings held sideways and buried in the head.
  - The legs had cuffs at every joint.
  - A flat cap ended the chest where the neck pitched down.
  - The tail curl was 6 cm behind the ham.
  - Coarser scales were still built on the old rig.

## 3. Root fixes in the asset file

**`rig.bones`.** Every origin, angle and length now comes from the landmark table `J` in `r3-pig-rig.py`. Changes from old to new:
- Lumbar 0.312 to 0.245 m; thorax 0.280 to 0.359 m. The vertebral axis now runs: lumbosacral joint (0.10, 0.60), thoracolumbar (0.345, 0.615), C7/T1 (0.69, 0.515), poll (0.855, 0.535).
- Neck 0.150 to 0.166 m; head 0.300 to 0.262 m at −24°; jaw 0.200 to 0.208 m.
- Ears 0.170 to 0.160 m, now hanging forward-down at −52°.
- Tail 0.080/0.070 to 0.043/0.033 m, following the curl.
- Thigh 0.19 to 0.20 m; metatarsus 0.12 to 0.129 m; hind hoof 0.065 to 0.095 m.
- Scapula 0.20 to 0.187 m; humerus 0.16 to 0.178 m; forearm 0.15 to 0.14 m; metacarpus 0.11 to 0.09 m; fore hoof 0.065 to 0.09 m.
- Centres of mass moved with the bones.
- Neck and head joint limits are now −70 to 50°.

**`poses`.** All 15 were re-solved, with the hooves kept on their ground contacts:
- Standing: far hind leg 0.055 m ahead; far fore leg 0.09 m ahead with the scapula 9° forward. This puts four separately visible legs in the elevation and keeps the 1:500 far fore leg printable.
- Walk keys: each toe keeps its old offset, with two-bone IK.
- Rooting and lying were authored anew.
- `rooting` sets `root.y`; `lying-wallowing` sets `root.y` = 0.211.

**`anchors`.** Hoof contacts are at the new toe tips. Belly and brisket contacts, nose, mouth and back plume are on the new surfaces.

**`materials`.** `bone` was added; the skeleton used skin before.

**`dimensions`.** 0.763 × 1.23 × 0.42 m became 0.684 × 1.27 × 0.331 m (height, length, width). The cited back height (0.64 to 0.71 m) and width (0.30 to 0.35 m) are met; the old values were the old tall ears and splayed legs. Mass stays 100 kg.

**`representations`.** All 217 closed shapes of the four scales and three views were redrawn as the true orthographic outline of the solids assigned to them (ruling E1). They keep the same ids, bones, roles, order, tones, regions and hatches. Each outline was:
- traced on a raster of 1/12 of the scale's smallest feature;
- simplified by Douglas-Peucker at a fifth of a feature, keeping the extreme points;
- mapped into each shape's own frame through the evaluated drawing (affine residual 0.000 mm).

Hand changes to the 2D:
- `mouth-line` (1:50 elevation) was redrawn by hand along the lip.
- The two eye shapes of the 1:50 plan were removed: from above they lie under the lop ears. The far one was a 9.8 mm sliver that made 1:50 unprintable (feature 0.196 mm against 0.2).
- Decision texts were updated: paper sizes, dewclaws in the hoof outline, eyes hidden in plan, 1:500 leg width.

**Not changed:** every `areaM2` (see section 6), masses, clips, thermal and structural data.

## 4. Models (`volumes`)

| scale | body | thermal | structure |
|---|---|---|---|
| 1:50 | 48 | 52 | 127 |
| 1:100 | 26 | 30 | 64 (7 rib hoops per side) |
| 1:200 | 21 | 29 | 51 (3 rib hoops per side) |
| 1:500 | 12 (minimal) | 22 | 30 (one bar per bone) |

**1:50 body (48 solids).**
- Barrel: two lofts of 15 hand sections, densified. The loin loft dips inside the chest loft at the thoracolumbar joint. It has a rounded rump, a slightly arched level back, a straight underline, a deep brisket and a rounded front end that stays closed when the neck pitches down.
- Hams: two hanging masses from the rump to above the hock. They stand 4 to 8 mm proud of the flank and 4 cm behind the pin bone, with a cleft between them seen from behind.
- Shoulders on the scapulae, flush with the barrel. Upper arms inside the chest that close the elbow.
- Head: a short deep neck, a wedge head of 9 sections, a jowled lower jaw as its own solid, the rooting disc, and eyes under the lop ears. The ears hang forward over the face like a roof.
- Tail: a corkscrew of 1.5 turns in two parts.
- Each leg, in matching sections that overlap at every joint: gaskin or forearm, point of hock, cannon with fetlock, pastern, two claws, and two dewclaws attached behind the fetlock pointing down and back.

**1:50 thermal.** Each region is a reference to the body surface it covers. Every region has at most one declared layer, and the 3 mm bristle layer is the surface. Outlets sit on the surface: two nostrils in the disc, the lips, and a back patch for the plume.

**1:50 structure.**
- Skull and mandible.
- Spine: cervical, thoracic with the spinous blade of the withers, lumbar with transverse and spinous processes, sacrum.
- 14 ribs per side with costal arch and sternum.
- Pelvis: ilium, ischium, pubis.
- Caudal vertebrae along the curl; ear cartilages.
- Forelimb: scapula with its spine, humerus, radius and ulna, carpus, two metacarpals, two digits with pedal bones.
- Hindlimb: femur with patella, tibia and fibula, tarsus with calcaneus, two metatarsals, two digits with pedal bones.
- Joints at the rig joints; masses of thorax and loin at their centres of mass.
- No supports: `structure.supports` is empty.

**Coarser scales.** Each has its own hand tables in `r3-pig-coarse.ts`; `decisions` are bilingual per list.
- **1:100:** the barrel in 6 + 5 sections; hams of 4 sections; shoulder merged into the chest; jowl merged into the head; disc and ears kept; the tail curls in one piece; per leg gaskin or forearm, cannon, pastern, and one hoof for both claws.
- **1:200:** the hams are kept as 3-section masses because they make the pig at this size. The head carries jowl and disc. The ears are thick leaves. Each leg is an upper mass, a cannon and a hoof wedge. Thermal splits the hoof wedge into pastern skin and claw horn and adds the tail, which the body leaves out.
- **1:500:** barrel, neck stub, head with ears merged, and per leg an upper mass plus a post to the ground at 92 mm. Thermal adds own solids at the size floor for tail, ears, snout and hooves.

**Final check** (`volumes-check.ts animals/pig --product`): 39 rule lines ok, 4 measurements fail on 2 lines. All four are `volumes-thermal-coverage` area shares at 1:50/1:100, see section 6.

| scale | silhouette IoU (elevation / front / plan) | p95 (elevation) |
|---|---|---|
| 1:50 | 0.995 / 0.984 / 0.995 | 1.9 mm |
| 1:100 | 0.994 / 0.982 / 0.990 | — |
| 1:200 | 0.989 / 0.978 / 0.989 | — |
| 1:500 | 0.977 / 0.966 / 0.992 | — |

- Bounds, cross-scale, monotonic, feature, bones and structure coverage are ok at every scale.
- `r3-validate.ts animals/pig`: VALID (schema 0, validateAsset 0). `--catalog-only`: VALID.
- Print suitability at every scale and both ends of `validRange`: suitable.
- Spliced with `volumes-insert.py`.

## 5. What I saw (images opened with Read)

- **1:50 body, `r3-sheet` six views with the rig, final:** the rig lies inside the forms in every panel.
  - Front: barrel, ears as two flaps over the face, the disc below them, four straight legs.
  - Right side: level back, long underline, ham proud behind, short thick neck, wedge head with the ear over the eye, far legs staggered ahead.
  - Back: two hams with the cleft, the curled tail on top.
  - Left side: the same from the other side; the far fore leg is forward.
  - Top: long rectangular body widening at the hams, ears spread over the head, disc forward.
  - Three-quarter: reads as a pig at first sight.
- **Poses at 1:50:**
  - Rooting (side): the trunk is closed and the head is low with the ears hanging.
  - Lying in the wallow (side): legs folded under the barrel, one hind claw just showing.
  - Walk at t 0.3 (three-quarter): three hooves on the ground, one lifted, every joint closed.
- **Thermal 1:50, cut sheet:** regions coloured; the plume patch lies flat on the back; the legs and claws are separate regions.
- **Structure:**
  - 1:50 side: the skeleton reads (ribcage, scapula blade, pelvis, long bones, skull).
  - 1:100 six views: hoops, spine, limb bars and joints, all inside the body.
  - 1:200 and 1:500 three-quarter rendered.
- **2D (`r3-2d.ts`):**
  - 1:50: elevation, front and plan in line and solid read as a pig. Shoulder and ham shapes are visible tone patches; bristle stipple is inside the outlines.
  - 1:500: tiny but complete with four legs.
- **`volumes-render.ts` sheet at 1:200:** green overlays without red or blue rims.
- **Rendered but not opened:** the 1:100 and 1:200 2D panels, the `volumes-render` sheets of 1:50, 1:100 and 1:500, the 1:200 and 1:500 structure three-quarter views, and the 1:100 thermal cut. All numbers above come from the check.
- **Not viewed after the last edits:** the 1:100 body six-view sheet was opened before the tail and chest edits (it read as a smaller pig). The 1:500 body was opened before its legs went from 82 to 92 mm and the neck/head transition was smoothed.

## 6. Still imperfect

- **Area shares (C27).** `skin-head` (0.170 against 0.10 m²; 1:100 0.159) and `hooves` (0.050 against 0.018 m²; 1:100 0.033) fail. The model is honest: two claws and two dewclaws per foot, and a jowled head. C18 forbids me to change them. The request to the area reviewers is in `author-requests.md` ("2026-11-19 r3 animals/pig"); it proposes moving 0.11 m² from `skin-trunk`.
- **Seams.** The shoulder and ham masses show as soft patches with a visible seam on the barrel, a real solid boundary. The upper arm bulges slightly below the chest at the elbow.
- **Leg detail.** The legs have faint joint rings at the knees and fetlocks where segments overlap. The cannons are smooth tubes with no tendon relief.
- **Head detail.** The lips and the nostril openings are not modelled at 1:50; the nostrils exist only as thermal outlets.
- **Standing stance.** The far fore leg stands 0.09 m forward with its forearm 16° off vertical. It is a natural stance, chosen partly so that the 1:500 far fore leg shows enough to print.
- **1:500 legs.** They are 92 mm thick against a real 50 to 90 mm: the scale's printable floor.
- **Clips.** Only one frame of the walk clip was rendered.
- **Reference.** The Condotta table values were not readable (403 / captcha); the reference relies on the summary and textbook values.

## Round 5 handcraft (2026-11-20)

Scripts: `r5-pig-rig.py` (rig root fix), `r5-pig-50.ts`, `r5-pig-coarse.ts` (remodels of the r3 tables), `r5-pig-run.sh` (whole pipeline: 4 scales, E1 redraw of the 2D through `r3-pig-2d.ts`, candidate validation, write). Backup of the asset before this round: `🗑️generated/r5-pig/asset-r4.json`.

### Reference (100 kg Large White x Landrace finisher) and model

The table of section 1 still stands. New rows this round:

| landmark | reference | model before | model after |
|---|---|---|---|
| hip joint, lateral | half the acetabular width, 0.08 to 0.09 m (pelvis 0.16 to 0.18 m between the acetabula) | 0 (on the midline) | 0.085 |
| dorsal border of scapula / shoulder joint, lateral | 0.04 to 0.06 / 0.09 to 0.10 m (chest 0.30 to 0.33 m wide) | 0 / 0 | 0.05 / 0.095 |
| ear base, lateral | 0.045 to 0.055 m (poll 0.10 m wide) | 0 | 0.05 |
| back height, length, width | 0.64 to 0.71 / 1.20 to 1.30 / 0.30 to 0.35 m | 0.684 / 1.27 / 0.331 | unchanged |
| upper edge of the shoulder | under the top line (the pig shows no plated shoulder) | 8 to 12 mm above the barrel near the withers | inside the barrel, flush at the point of the shoulder |

All other bone lengths, joint heights and angles are unchanged from round 3 (within 3 % of the reference table, measured there).

### Grades (opened sheets)

| scale | body before / after | thermal before / after | structure before / after |
|---|---|---|---|
| 1:50 | B (rig of the legs on the midline, shoulder plates proud of the top line, cuffs at every leg joint, faceted ear outline) / A- | A- / A- | A- / A- (joints now at the lateral joint centres) |
| 1:100 | B- (ears as flat hexagonal planks, stepped leg segments) / A- | not opened / not opened | not opened / not opened |
| 1:200 | not opened / not opened (same leg rounding applied) | not opened | not opened |
| 1:500 | C+ (legs as two misaligned stacked cylinders, ears as 83 mm blocks standing off the head) / B+ | C (hooves as sideways drums like wheels, ear and snout patches ragged in the head) / B+ | not opened |

### Changes

- Root: `rig.bones` got lateral offsets `z` (thigh-h ±0.085, scapula-f ±0.05, humerus-f a further ±0.045, ear ±0.05); before, every leg and ear bone ran on the midline and the solids sat 0.095 m beside their bones. Solids are stated in world metres, so every scale was rebuilt on the new frames; poses and clips are angle-based and kept; 2D was redrawn (E1, 217 shapes, residual 0.000 mm); candidate VALID before writing.
- 1:50: shoulder table lowered and moved 12 to 15 mm inwards so that it lies flush on the shoulder blade; every leg segment (gaskin, forearm, cannons, pasterns, shoulder) ends in a stated dome (`rounded`) that ends inside its neighbour, so no flat cap or cuff shows at a bent joint; gaskin top reduced (no knee ball under the ham); cannon tops matched to the forearm and gaskin ends; ear tables densified three times so the lop ear has a smooth leaf outline.
- 1:100 / 1:200: same rounded leg ends; ears densified.
- 1:500: ears merged into a broad head (a separate ear is a 75 mm slab); each upper leg mass runs 35 mm past the knee or hock into a flush lower post; thermal ears are two ellipsoid lenses on the sides of the head, the snout a clean cap on the head tip, the hooves bands round the foot of each post.
- `decisions` of the 1:50, 1:100 and 1:500 volumes updated (en, de).

### Checks

- `r3-validate.ts animals/pig --volumes`: schema 0, validateAsset 0 (print suitability clean), checkVolumes 6: all `volumes-thermal-coverage` area shares (skin-head, hooves, skin-legs at 1:50 and 1:100; skin-legs is now 0.26 to 0.27 against 0.20, the honest leg skin). Listed with hand sums in `area-review.md` ("r5 animals/pig"); C18 and C27 forbid the author to change them.
- `volumes-check.ts animals/pig --product`: 39 rule lines ok, the same 6 measurements on the thermal-coverage line.

### What I saw

- 1:50 body six views and a head-on three-quarter (turn 60, tilt 15): the rig now runs inside every leg in the front and three-quarter views; shoulder no longer proud; legs end smoothly at carpus, hock and fetlock (a slight carpal bulge stays, as in a real pig); far ear a smooth curved leaf.
- 1:50 structure six views: spine dorsal under the top line with spinous processes and the withers, 14 sloping ribs with sternum and costal arch, pelvis, scapulae, long bones, digits; joints at the joints.
- 1:50 `lying-wallowing`: closed, legs folded under the barrel.
- 1:100 body six views: clean pig, curved lop ears, continuous legs.
- 1:500 body six views and thermal three-quarter (three iterations): broad head with the ears merged, flush legs; ear lenses and hoof bands clean.

### Still imperfect

- The chest and loin lofts meet in one visible seam ring at the thoracolumbar joint (two bones, two solids; solids are rigid per bone).
- The near lop ear still shows a few corners in its outline at 1:50 (the leaf has flat end caps).
- The lying pose puts one hind claw tip a few millimetres into the ground plane (the wallow).
- 1:200 body, the thermal cut sheets of 1:50 to 1:200, the structure of 1:100 to 1:500, the walk clip and the 2D drawings after the redraw were rendered or regenerated but not opened in this round.
- Area shares above wait for the area reviewers.
