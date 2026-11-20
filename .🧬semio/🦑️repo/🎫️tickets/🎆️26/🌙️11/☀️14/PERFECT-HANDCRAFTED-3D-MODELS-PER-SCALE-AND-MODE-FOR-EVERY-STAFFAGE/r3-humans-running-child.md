# Round 3 — `humans/running-child`

State (updated as the work proceeds): spliced into the catalog. `r3-validate.ts humans/running-child --catalog-only`: `VALID ... schema 0, validateAsset 0`. `volumes-check.ts humans/running-child --product`: `PASS` at every scale. No earlier round-3 work existed for this asset (no `r3-running-child-*` scripts, no partial report, catalog `volumes` equal to the source of 2026-11-19 01:00), so the round started from the validated round-2 state.

Author scripts (ticket folder): `r3-running-child-kit.ts` (constructors: lofts, upright lofts on the sloping foot bone, swept bands, rib rings, layer shells), `r3-running-child-body.ts` (1:50 body, every section stated by hand), `r3-running-child-lists.ts` (1:100, 1:200, 1:500 bodies, thermal model, skeleton), `r3-running-child-texts.ts` and `r3-running-child-texts.json` (decisions and texts), `r3-running-child-figure.ts` (assembly into `volumes/humans/running-child.json`), `r3-running-child-draw2d.ts` (2D redraw), `r3-running-child-retarget.ts` with its result `r3-running-child-motion.json` (poses and clip keys for the corrected rig), `r3-running-child-roots.py`, `r3-running-child-apply.py`, `r3-running-child-splice.py`, `r3-running-child-manifest.py` (writers), `r3-running-child-run.sh` (candidate build and validation), `r3-running-child-deploy.sh` (catalog write, only when the candidate is valid), probes `r3-running-child-frames.ts`, `-motion-probe.ts`, `-probe2d.ts`, `-power.ts`, `-sheet.ts` (copy of `r3-sheet.ts` that reads the candidate).

## 1. Reference

Subject: a slim child of 1.125 m barefoot and 20.7 kg (22.1 kg with the 1.4 kg backpack), in sneakers with a 12 mm sole (1.137 m). Stature and mass are the asset's own physiological data and were kept. Sources: Snyder et al. 1977 (UM-HSRI-77-17, child anthropometry by age), the WHO 2007 growth reference, Winter / Drillis and Contini (segment lengths, already in `provenance.references`), de Leva 1996, the brief. The child values are stated as I know these tables, rounded and scaled to this stature; I did not re-read the tables in this session (no web lookup was made).

| quantity | reference | value used |
|---|---|---|
| stature for age | WHO: boys 6 y median 1.160 m, 7 y median 1.217 m (SD about 0.05) | 1.125 m kept: 25th percentile at 6 y, 4th at 7 y; mass 20.7 kg is the 6 y median, so the summary now says six-year-old |
| head height (chin to vertex) | 0.185 to 0.19 m at 6 to 7 y, about 1/6 of stature | 0.185 m (6.1 heads) |
| head breadth / length | 0.14 / 0.18 m (head girth 0.515 m) | 0.138 / 0.178 m |
| hip joint height | sitting height 0.545 to 0.55 H at 6 y, so seat plane at 0.455 H and hip joint about 0.045 H above it: 0.50 H (trochanterion about 0.497 H); adults 0.52 to 0.53 H | 0.5625 m barefoot, 0.5745 m shod |
| knee joint height | tibiale about 0.263 H, joint centre about 0.27 H | 0.306 m barefoot (thigh 0.2565 m) |
| ankle joint height | 0.039 H | 0.044 m barefoot, 0.056 m shod (shank 0.262 m) |
| C7 height | cervicale about 0.816 to 0.82 H | 0.9225 m barefoot, 0.9345 m shod |
| shoulder joint height | acromion about 0.785 H, joint centre 2.5 cm lower: 0.76 H | 0.858 m barefoot, 0.870 m shod |
| upper arm / forearm / hand | 0.165 H / 0.13 to 0.137 H / 0.108 H | 0.185 / 0.146 / 0.120 m |
| biacromial / bideltoid breadth | 0.243 / 0.27 to 0.28 m | joints 0.21 m apart, 0.28 m over the sleeves |
| hip joint spacing, hip breadth | about 0.12 m, 0.20 m | 0.11 m (kept), 0.208 m |
| chest breadth x depth, girth | 0.19 x 0.135 m, 0.57 m | under a loose T-shirt of 0.21 x 0.16 m |
| waist girth | 0.52 m | under the T-shirt |
| thigh girth top / mid / above knee | 0.33 / 0.29 / 0.24 m | sections 0.094 x 0.104, 0.088 x 0.098, 0.072 x 0.08 m |
| knee breadth, calf girth, ankle girth | 0.07 m, 0.235 m, 0.155 m | 0.071 x 0.08, 0.07 x 0.082 (calf set back 8 mm), 0.043 x 0.054 m |
| upper arm / forearm / wrist girth | 0.17 / 0.165 / 0.115 m | 0.052 x 0.06, 0.051 x 0.051, 0.029 x 0.039 m |
| hand breadth / thickness | 0.056 / 0.018 m | 0.056 / 0.019 m |
| foot length / breadth; shoe | 0.152 H = 0.172 m, 0.068 m; sneaker EU 28: about 0.19 x 0.076 m | shoe 0.19 x 0.075 m, sole 12 to 16 mm |
| T-shirt, shorts, backpack | size 116 garments; small child's pack 0.20 x 0.26 x 0.09 m (`area-decisions.md`) | hem at hip joint height, sleeves 0.10 m; shorts to mid thigh; pack as stated |
| segment masses | Jensen 1986 (asset) | unchanged, centres kept at their ratios |
| body surface | DuBois 0.812 m² (asset) | unchanged |

## 2. Audit findings

Rig (old values, shod, stature 1.137 m)
- Hip joints at 0.5096 m = 0.443 H barefoot: that is crotch height, not the hip joint (0.50 H). Legs 65 mm too short (thigh 0.2371, shank 0.2316 m), trunk correspondingly too long. Root defect.
- Ankle joint 0.041 m above the ground including the sole (0.031 m barefoot; reference 0.044 m).
- C7 at 0.905 m (0.794 H; reference 0.82 H), neck and head bones together 0.232 m against 0.203 m from C7 to the vertex. Root defect.
- Hand 0.106 m (reference 0.12 m). Shoulder joint height (0.874 m), shoulder and hip spacing, upper arm and forearm lengths within 1 cm: kept.
- Backpack centre of mass 45 mm inside the back (`comY` 0, `comX` 0.1).
- `presentation.defaultPose` was `standing-ready`, so a "running child" stood (visual QA 3).
- Head anchors sat on the old face; foot anchors on the old foot bone.

Poses and clips: solved for the old leg lengths. With the corrected rig alone the toe-off pose floated 28 mm, the flight arc no longer met the landing and the swing foot went 14 mm into the ground in mid flight.

Dimensions: 1.13 x 0.281 x 0.28 m described the standing pose.

2D drawings: mannequin drawing of the old rig (ball shoulders, elbow and knee discs, chest print block, limbs as capsules in plan), different ids for the same part in different views, 1:500 as one merged torso silhouette with stroke limbs.

Solids (old 1:50: 49 body): box torso with ball shoulders, tube limbs with knee rings, slipper shoes, head 1/7.4 of the height, backpack as a slab; skin under every garment; structure a core copy of the body; coarser scales fitted to the old drawings.

Texts: summary "seven-year-old ... 5.8 heads"; sprint description "about 1.9 m/s".

## 3. Root fixes (old -> new)

| member | old | new |
|---|---|---|
| `rig.bones[pelvis].y` | 0.5096 | 0.5745 |
| `thigh-*.lengthM` / `mass.comX` | 0.237075 / 0.0972 | 0.2565 / 0.1052 |
| `shank-*.x` / `lengthM` / `mass.comX` | 0.2371 / 0.231625 / 0.1033 | 0.2565 / 0.262 / 0.1168 |
| `foot-*.x` / `angleDeg` / `lengthM` / `mass.comX` | 0.2316 / 66.2 / 0.101261 / 0.0431 | 0.262 / 55.34 / 0.0985 / 0.0419 |
| `toe-*.x` / `angleDeg` / `lengthM` / `mass.comX` | 0.1013 / 23.8 / 0.035425 / 0.0163 | 0.0985 / 34.66 / 0.05 / 0.023 |
| `torso.lengthM` / `mass.comX` | 0.3255 / 0.15 | 0.29 / 0.1336 |
| `neck.x` / `lengthM` / `mass.comX` | 0.3255 / 0.08 / 0.04 | 0.29 / 0.07 / 0.035 |
| `head.x` / `lengthM` / `mass.comX` | 0.08 / 0.153 / 0.07 | 0.07 / 0.1335 / 0.0611 (vertex stays at 1.137 m) |
| `arm-*.x` | 0.294 | 0.2255 (shoulder joint at 0.870 m) |
| `hand-*.lengthM` / `mass.comX` | 0.1064 / 0.0532 | 0.12 / 0.06 |
| `pack.x` / `mass.comX` / `mass.comY` | 0.1 / 0.1 / 0 | 0.075 / 0.035 / 0.08 (centre of the pack) |
| anchors `heel-*` | (-0.0059, -0.0473) | (0.0088, -0.062) (28 mm behind the ankle on the ground) |
| anchors `ball-*` | (0.1013, 0) | (0.0985, 0) |
| anchors `mouth` / `nostrils` / `crown` | (-0.0043, -0.0681) / (0.0255, -0.0875) / (0.145, 0) | (-0.0185, -0.079) / (0.0005, -0.086) / (0.1335, 0) |
| `poses[*].bones` thigh, shank, foot; `poses[*].root.y` | old solution | retargeted (`r3-running-child-motion.json`): e.g. toe-off root -0.0416 -> -0.0469, airborne -0.03 -> -0.0353, jump-takeoff 0.0434 -> 0.0295, jump-apex 0.3234 -> 0.3094 |
| `clips[*].keys[*].bones` thigh, shank, foot | old solution | retargeted, 90 keys |
| `clips[sprint-cycle].advanceM` | 1.014955 | 1.1228 (planted travel of the longer legs) |
| `presentation.defaultPose` | standing-ready | toe-off |
| `dimensions` height / width / depth | 1.13 / 0.281 / 0.28 | 1.096 / 0.671 / 0.29 (extents of the body at toe-off) |
| manifest row `heightM` / `widthM` / `powerW` | 1.13 / 0.281 / 66.1 | 1.096 / 0.671 / 283.3 (the validator sums the power of the default pose, now a running pose) |
| `representations[*].geometry` and `.decisions` | old drawing | redrawn (below), texts rewritten |
| `summary`, `clips[sprint-cycle].description`, `poses[toe-off].description`, `provenance.notes`, `provenance.references` | seven-year-old, 5.8 heads; 1.9 m/s; "knee fully extended" | six-year-old, six heads, hip joints at half the stature; 2.2 m/s; knee still soft; round-3 note; + `snyder-1977`, `who-2007` |

Retarget (`r3-running-child-retarget.ts`, run once on the old catalog state): the pelvis path is the old one scaled by the ratio of the hip heights (1.1274), flights keep their absolute rise above the take-off because the flight times stay (the jump stays 0.28 m), every foot keeps its world orientation and its place under the pelvis scaled by the leg ratio (1.1063) with planted anchors on the ground, and thigh, shank and foot are solved again by two-link inverse kinematics; the pelvis is lowered where a leg would lock (take-off of the jump). Measured with the product's own evaluation (`r3-running-child-motion-probe.ts`): sprint pelvis 0.5276 m at toe-off and landing, 0.4845 m at mid stance, 0.5392 m at the top of the flight; planted anchors at 0.0000 m through the stance; jump from 0.6040 m to 0.9037 m.

2D redraw (ruling E1): every filled shape of every scale and view is the orthographic outline of the body solids of its part (Douglas-Peucker 0.8 / 1.5 / 3 / 6 mm). Elevation: bone-local outlines. Front and plan: trunk parts as prisms of their lateral outline with a flat plan outline over a prism core, limbs as revolved profiles, hands, toe caps and the backpack as prisms, the shoe on the sloping foot bone as the prism of its elevation profile (exact length and height in every pose; its plan is a rectangle of 0.88 of the shoe width). One id, tone, region and material per part in all views; the far limbs are 8 % darker in the side elevation. Strokes stated by hand: the two straps (the strap solids are slender and drawn as strokes), hems, sock tops, and at 1:50 brow, mouth, jaw, nose, hair strands, sole lines, laces and the pack seam. The chest print block is gone.

## 4. What was modelled (solids body / thermal / structure)

- **1:50 — 64 / 71 / 89.** See the decisions in the asset. Body: T-shirt as one garment loft with collar rib, sleeves on the arm bones, neck, head (10 sections), nose, ears, eyes, hair cap, shorts with seat masses and two legs with rounded tops, thighs, knee caps, shanks with set-back calf, sock cuffs, shoe with sole and upper built from upright sections on the sloping foot bone, toe cap with sole on the toe bone, upper arms, forearms, palms, four two-segment fingers and thumb per hand, backpack loft with pocket and grab handle, two straps swept along a 14-point path over shoulder, chest and under the arm. Thermal: references to those surfaces, two nested shells for T-shirt and sleeves, face mask and bare-neck segment as own solids, section faces on the cuts through the continuing body, outlet patches at nostrils, lips and crown. Structure: skeleton (cranium, face, mandible, cervical spine, spine, 8 rib hoops, sternum, clavicles, scapulae, acromia, sacrum, hip bones, pubic rami, femur head, neck, trochanter, shaft, patella, tibia, fibula, talus, calcaneus, metatarsus, phalanges, humerus, radius, ulna, carpus, metacarpus, finger row, thumb), back panel and straps of the pack, 17 joints, 6 masses.
- **1:100 — 29 / 32 / 59.** New longer sections; nose, ears, eyes, knee caps, collar, handle dropped; mitten hands; one shoe form; straps kept as slender bands. Skeleton with merged parts and 4 rib hoops.
- **1:200 — 25 / 29 / 39.** Head with neck, hair, T-shirt, sleeves, shorts, shorts legs, thighs, shanks, sock cuffs, shoes, toe caps, arms, mitten hands, backpack. Skeleton of single thickened bones, rib barrel, pelvis basin, 10 joints, pack panel and true-size straps.
- **1:500 — 19 / 27 / 26.** Head, hair, T-shirt, shorts, backpack, and per side thigh, shank, sock cuff, shoe, toe cap, upper arm, forearm with hand. Structure: skull, rib cage and pelvis masses with slender rods for spine, neck and limb bones.

Final check line: `PASS` (`volumes-check.ts humans/running-child --product`). Silhouette IoU elevation / front / plan: 1:50 0.991 / 0.974 / 0.963, 1:100 0.991 / 0.969 / 0.964, 1:200 0.992 / 0.966 / 0.949, 1:500 0.981 / 0.963 / 0.927. Thermal area deviations at 1:50: face 15 %, arms 28 %, legs 16 %, T-shirt 26 %, shorts 28 %, socks 27 %, shoes 25 %, backpack 14 %.

## 5. What I saw in the renders

Resumed on 2026-11-19 23:24 after the first author was cut off at this section. On resuming: `r3-validate.ts humans/running-child` `VALID ... schema 0, validateAsset 0`; `volumes-check.ts humans/running-child --product` `PASS (11.2 s)`; the catalog `volumes` equal the source `volumes/humans/running-child.json`; `presentation.defaultPose` `toe-off`, `dimensions` 1.096 x 0.671 x 0.29 m, 22.1 kg. Nothing was broken, so no write was needed. Renders opened with the Read tool (all `r3-sheet.ts`: front, right side, back, left side, top, three-quarter in one image; `r3-2d.ts` for the product drawings):

- **1:50 body, toe-off** (`running-child.s50.body.png`). Front: six-heads child, head 1/6 of the stature, narrow shoulders under short sleeves, straps over the shoulders, shorts to mid thigh, slim thighs and shanks, sneakers; rear arm hangs beside the hip, front forearm crosses the chest. Right side: forward lean of the trunk, push-off leg with the shoe plantar-flexed and only the toe cap on the ground, swing knee forward and up, swing shoe flat under the knee; pack on the back with handle, front pocket and straps under the arm; hand with thumb and fingers. Back: pack centred between the shoulder blades, pocket panel visible, shorts seat, both shoe heels. Left side: nose, ear and hair line read, forward arm bent with the hand open, rear elbow behind the trunk with the hand at the hip. Top: head over the trunk, pack behind, arms and legs in counter-phase. Three-quarter: closed figure, no gaps at shoulder, elbow, hip, knee or ankle; every joint within its overlap.
- **1:50 body, clip `sprint-cycle` t 0.15 s** (`...s50.body.sprint-cycle-t0.15.png`): mid stance, stance shoe flat on the ground, swing thigh forward, both forearms bent near 90 degrees and passing the trunk; no part leaves the body, the shoes stay on `Y = 0`.
- **1:100 body** (`...s100.body.png`): same figure with fewer sections: mitten hands, one shoe form, no ears, nose or collar; proportions identical to 1:50, straps still read.
- **1:200 body** (`...s200.body.png`): head with hair cap and neck, T-shirt, sleeves, shorts, limbs and shoes as clean lofts; pack as a rounded block; reads as a running child at a glance.
- **1:500 body** (`...s500.body.png`): head, trunk, pack, shorts and the limbs as tapered prisms; the head is faceted (crown octagon) because its 0.185 m is about 2.5 features of 75 mm; it reads as a head with hair, the running gait stays clear.
- **1:50 structure** (`...s50.structure.png`): skull with face and mandible, S-curved cervical and thoracic spine, rib hoops with sternum, clavicles and scapulae, pelvis with sacrum and pubic rami, femur with neck and trochanter, patella, tibia and fibula, foot bones, humerus, radius and ulna, carpus and finger rows; joints (orange) at shoulders, hips, knees, ankles; masses (magenta) at trunk, thighs, head and the pack centre behind the back; the pack back panel and straps are the frame of the pack. No member pokes out of the body ghost.
- **1:100 structure** (`...s100.structure.png`): the same skeleton merged (4 rib hoops, single long bones with epiphyses), same joint positions.
- **1:50 thermal, cut** (`...s50.thermal.cut.png`): nested T-shirt shells (dark and light cyan), shorts over leg skin, face skin with the bare neck segment, hair cap, arm skin, socks, shoes, backpack; the cut shows the inner shell and the section faces at the T-shirt hem and the shorts legs; no shell protrudes through its outer layer.
- **1:100 thermal** (`...s100.thermal.png`): every region present and exactly on the body form; face red under the green hair cap, arm skin purple from the sleeve hem.
- **2D 1:50** (`r3-2d/humans/running-child.s50.png`, light line and light solid, enlarged crop of the side elevation): side elevation is the true outline of the toe-off body, pack with handle and pocket, profile with nose, mouth and ear, laces and sole lines; the near shorts leg over the thigh in the darker shorts tone; front elevation symmetric with the pack straps; plan with head, pack, outstretched forward arm and both shoes. Earlier sessions also opened the poses `standing-ready`, `airborne`, `landing` (`🗑️generated/r3-running-child/sheet-*`, now deleted with the generated folder).

## 6. Still imperfect (stated plainly)

- The face is a simple mask (brow, nose, eyes as slits, ears): it reads as a child's face at 1:50 but has no modelled cheeks or lips; the hair is a cap with a fringe edge, no strands in 3D (strands are 2D strokes only).
- In the 2D elevations the per-bone shapes show the outline of the thigh and shank overlap at the knee as a line: a consequence of one shape per bone with overlaps that keep every pose closed; a single-outline leg would break the clips.
- 1:500 head is faceted (feature limit), acceptable at that scale.
- Thermal area deviations at 1:50 of 14 to 28 % per region (listed in section 4) are the product's area rules against the real garment areas; they pass the check and were not steered (C29, C31).
- The arm poses and arm clip keys were not retargeted (only legs and pelvis changed with the rig); they read right in every opened frame.

Final state: catalog spliced, `VALID` and `PASS` at every scale (2026-11-19 23:30).
