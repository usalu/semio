# Round 3 — humans/person-on-crutches (2026-11-19)

State: spliced into the catalog (roots, redrawn 2D of all four scales, `volumes`). `r3-validate.ts humans/person-on-crutches` and `--catalog-only`: `VALID ... schema 0, validateAsset 0`. Print-suitable at all four scales under the default profile (1:500 was unsuitable on the way, fixed, see root fixes). `volumes-check.ts --product`: every rule line green at every scale except `volumes-thermal-coverage` at 1:50 and 1:100 (`shorts`, `boot`, `sneaker` declared areas, see "Still imperfect"; request in `author-requests.md`).

This asset was started by an earlier author who was cut off before the report and the splice. I resumed from the scripts: ran the build, the validator and the check, looked at every sheet, then found and fixed what was missing or wrong (placeholder `decisions` "Draft." at all four scales, 1:500 not print-suitable, the catalog unspliced and invalid with the source merged because the `bone` material and the new rig were only in the candidate), wrote the catalog and spliced.

Scripts (in `T`): `r3-person-on-crutches-roots.ts` (rig, anchors, poses, clips), `-kit.ts`, `-parts.ts`, `-s50.ts`, `-coarse.ts` (handcrafted sections of every scale), `-draw2d.ts` (E1 redraw), `-build.ts`, `-run.sh` (build + draw + validate the candidate), `-write.py` (catalog root members, new this session), `-sheet.ts` (r3-sheet copy reading a candidate file), `-frames.ts`, `-probe.ts`, `-suit.ts`.

## Reference (male, 24 years, stature H = 1.82 m, 79 kg)

| Item | Real value | Share / note | Source |
|---|---|---|---|
| Hip joint centre | 0.95 m (barefoot) | 0.52 H | Drillis-Contini 1966; Winter 2009 tab. 4.1 |
| Knee joint | 0.52 m | 0.285 H | same |
| Ankle joint | 0.071 m barefoot, +25 mm sneaker sole | 0.039 H | same |
| Shoulder joint | 1.49 m | 0.818 H | same |
| C7 / neck base | 1.56 m | 0.86 H | Winter 2009 |
| Head height chin–vertex | 0.243 m | H / 7.5 | anthropometry |
| Thigh / shank length | 0.446 / 0.448 m | 0.245 / 0.246 H | Winter 2009 |
| Upper arm / forearm | 0.339 / 0.266 m | 0.186 / 0.146 H | Winter 2009 |
| Biacromial breadth | 0.446 m (joint centres ~0.40 m) | 0.245 H | anthropometry |
| Hip breadth | 0.35 m; hip joint spacing 0.19 m | 0.19 H | anthropometry |
| Segment masses / COM | trunk 25.2 kg, head 5.4 kg, hand 0.48 kg | de Leva 1996 tab. 4 | asset `provenance` |
| Forearm crutch | 22 mm Al tube, cuff arm 0.20–0.25 m behind forearm, open 270 deg cuff 0.10 m high 5–8 cm below elbow, handle 0.12 m at 90–100 deg to shaft, rubber ferrule o 40 mm, grip-to-tip 0.90 m (handle at wrist-crease height 0.485 H = 0.88 m) | ISO 11334-1:2007 construction; length authored |
| Walker boot (CAM) | shell ~0.42 m high, foot 0.30 x 0.13 m, rocker sole, two struts, 4–5 straps | product data, generic |
| Swing-to gait | crutches 0.3–0.5 m forward, body swings to the crutch line; non-weight-bearing on the injured side | Perry & Burnfield 2010; Waters & Mulroy 1999 |

## Audit of the catalog before round 3

- Rig: trunk 0.4641 m put C7 at 1.53 m (0.84 H, low) and the head bone 0.2148 m long from a too-low atlas (vertex by bone length, not by skull); neck 0.0855. The crutch bones hung off the hand at x 0.0885 m along the hand with their own pose angle (-31 / -32 deg), so the crutch was not rigid in the grip and rotated against the hand between poses; no lateral offsets, so arms, crutches and tips stood in the body plane. Grip anchor at the hand bone axis (0.0885, 0), hand COM 0.155 m out (outside the hand).
- Poses: `tripod-stand` stood on a right knee flexed 24 deg with the right foot pitched 12 deg; the left (off-loaded) knee flexed 56 deg; the crutch tips did not stay planted between clip keys; the gait was not a swing-to gait (tips and stance foot slid).
- 2D: drawn per scale by hand from the old body; crutches as straight poles without cuff arm in elevation, the grip as a disc, tips not drawn as their own region; outlines of a box torso and tube legs (front width 0.556 m, depth 0.65 m against 0.52 / 0.70 m of the true default pose).
- Solids 1:50 (35 body / 61 thermal / 32 structure): box-like torso loft, tube limbs, head as a capsule-ish loft with a cap, mitten hands, the crutch as a straight pole held in the fist without handle, cuff arm, bracket or ferrule, a closed cuff ring floating on the sleeve; structure a stick skeleton. The coarser scales were the same lists thinned (35 / 26 / 20 body).

## Root fixes (written to the catalog)

- `rig`: torso length 0.4641 -> 0.497 (C7 at 1.559 m = 0.857 H), neck 0.0855 -> 0.100 (atlanto-occipital at 1.659 m), head 0.2148 -> 0.156 (vertex 1.815 m; with hair 1.82); torso COM x 0.2088 -> 0.2236, head COM x 0.1074 -> 0.045. Crutch bones: joint at the handle–shaft junction in the hand frame (x -0.0017, y -0.0471), rest angle 0 -> -51.5 deg, so the crutch is rigid in the hand (pose offsets 0); COM (0.40, 0) -> (0.29, -0.01). Lateral spread: forearm z ±0.0295, hand z ±0.0205, crutch z ±0.0035; hand COM (0.1553, 0) -> (0.055, -0.012). Material `bone` added for the structure lists.
- `anchors`: grip (0.0885, 0) -> (0.05, -0.017) on the handle, z ±0.003; tip z ±0.0775 (tips a hand's breadth outside the feet); nose (0.0781, -0.1112) -> (-0.016, -0.1), mouth -> (-0.038, -0.094), crown -> (0.164, -0.012) on the corrected head.
- `poses` / `clips`: every key solved from one choreography (`r3-person-on-crutches-roots.ts`): standing on a straight right leg (knee 8 deg), tips planted 0.17 m ahead of the hip, elbows bent 30 deg, the booted left foot held level 0.137 m above the ground; `crutches-forward` t 0.4, `body-swing` t 1.05; `three-point-gait` as a swing-to gait of 0.4 m per 1.8 s with planted tips and a heel-strike / toe-off of the right foot; `rest-on-crutches` sways the trunk with the tips solved back onto their planted point. Descriptions rewritten en/de.
- `dimensions`: widthM 0.556 -> 0.52, depthM 0.65 -> 0.70 (extent of the default pose of the new 1:50 body; height 1.82 and mass unchanged).
- `representations[*].geometry`: every filled shape of all 12 drawings redrawn as the orthographic outline of its body solids in the default pose (E1, same ids, roles, bones, regions, layer order by hand), slender crutch rods as strokes at 1:200 / 1:500, drawstrings and mouth as detail strokes at 1:50. New this session: at 1:500 the ferrule (41 mm, under the 75 mm feature and 0.18 mm on paper, which made 1:500 unprintable) is no own shape; the rod stroke ends in a stroke across the ferrule base and carries the regions `crutch-frame`, `crutch-grips`, `crutch-tips`. Points per view 1:50 elevation / front / plan 795 / 976 / 801, 1:500 176 / 215 / 275.

## Model per scale (body / thermal / structure solids)

- 1:50 (130 / 156 / 156): skull loft with brow, nose with nostrils, lips, chin, cheeks, ears, eyes, eyebrows, hair cap; neck with larynx; sweatshirt over ribcage and shoulder mass with collar, hood lying on the back, kangaroo pocket, drawstrings, sleeve cuffs; loose shorts over the gluteal mass with the left leg cut at the hem; thighs, knees, calves with muscle sections; hands with four metacarpals, three-phalanx fingers and thumb wrapped around the handle; walker boot with sole, shell, two struts, five straps; sneaker with sole, front, rear, tongue and four laces; each crutch with cuff arm, bracket, upper tube, joint, lower tube, clip, five adjustment holes, rubber handle, ferrule, open 270 deg cuff with mount. Thermal: every region, nested shells (hoodie air / t-shirt / sweat, shorts air / poly, boot air / sock / shell, sneaker air / sock / upper), outlets for nose and mouth breath and the plume. Structure: cranium, face bones, mandible, cervical and curved spine, ten rib hoops, sternum, clavicles, scapulae with spine, ilium, ischium, pubis, sacrum, femur with neck, trochanter and condyles, patella, tibia with plateau, fibula, malleolus, talus, calcaneus, metatarsals, phalanges, humerus with head and condyles, radius, ulna, carpals, finger bones, crutch frames as members; joints at every rig joint, masses after de Leva.
- 1:100 (65 / 68 / 91): new tables; face reduced to nose and ears, hands as palm with two-part fingers and thumb, boot with three straps, crutch as tube, bracket, handle, ferrule, cuff; structure with five rib hoops, simplified hands and feet.
- 1:200 (32 / 35 / 41): merged masses, hands as one mass, crutch rod slender with grip, ferrule, cuff; structure skull, spine, three rib hoops, girdles, long bones.
- 1:500 (25 / 32 / 29): few-section masses in the same proportions, crutch as one slender rod from cuff to ground with its cuff (handle and ferrule below the feature, now also in the body: the rod runs to the ground); thermal grips and ferrules as surfaces at the size floor on the rod.
- `decisions`: the earlier author had left "Draft." at all four scales; replaced by real en/de sentences (5 / 3 / 2 / 2).

## Check

`$BUN volumes-check.ts humans/person-on-crutches --product`: `FAIL: 6 measurement(s) over 2 rule line(s)` — only `volumes-thermal-coverage` 1:50 / 1:100: `areaDeviation` shorts 1.456 / 1.456, boot 0.587 / 0.449, sneaker 0.656 / 0.695 (limit 0.3). Everything else ok at every scale (bounds, feature, monotonic, cross-scale, structure coverage, bones, slender, copied); thermal totals 1:200 0.159, 1:500 0.177. Silhouette IoU elevation / front / plan: 1:50 0.990 / 0.997 / 0.994, 1:100 0.996 / 0.993 / 0.994, 1:200 0.992 / 0.988 / 0.986, 1:500 0.985 / 0.978 / 0.961. `r3-validate.ts`: VALID (with source and `--catalog-only`).

## What I saw in the final renders

- 1:50 body, six views (`r3-person-on-crutches-sheet.ts`, candidate = catalog): front — upright young man, shoulders wider than hips, sweatshirt with pocket and drawstrings, both crutches from the cuff under the elbow to tips outside the feet; right side — straight standing leg, the boot leg held forward with bent knee, cuff arm running from the handle up behind the forearm to the cuff (real forearm-crutch geometry), handle forward of the shaft, head profile with nose, lips, chin, hood folded on the back; back — hood, shoulder blades read through the cloth, the boot with its straps; left side — same, boot struts visible; top — shoulders, head, crutches spreading forward and outward; three-quarter (and close crops) — fingers wrapped around the black handles, thumb on top, open cuff, telescopic joint and holes on the lower tube, laced sneaker, walker boot with straps and rocker sole.
- 1:100, 1:200, 1:500 body sheets: same man and stance; proportions hold; at 1:200 / 1:500 the rods are thin strokes (yellow highlight = slender in the sheet), cuffs still present.
- 1:50 structure: skull, curved spine, ribcage, scapulae, pelvis, long bones with heads, finger bones in the grip, crutch frames; joints and masses where they belong; nothing outside the body.
- 1:50 thermal, cut: nested hoodie, shorts and boot shells, legs skin under the shorts, face and hands as skin, crutch regions.
- 1:500 thermal front: ferrule surfaces at the rod ends on both sides (first render showed the right one mirrored wrongly — fixed and re-rendered).
- Poses / clips: `body-swing` pose (1:50) and `three-point-gait` t 0.9 (1:100) and t 1.1 (1:50, catalog): body between the crutches, right foot reaching the landing behind the tips, tips planted, no part detaching; 2D `body-swing` at 1:100 reads right in all three views.
- 2D (r3-2d) 1:50 and 1:500: clean outlines; light-line and light-solid read as a man on forearm crutches with a walker boot.

## Still imperfect

- `volumes-thermal-coverage` 1:50 / 1:100: the declared `shorts` (0.197), `boot` (0.224) and `sneaker` (0.0697) areas are skin shares, not the surfaces of loose sports shorts, a walker boot and a sneaker; C18 forbids me to change them and I did not shrink the garments. Request in `T/author-requests.md` (dated 2026-11-19).
- The face at 1:50 is simplified (eyes as flat lenses under the brows); at 7.5 mm feature an eye of 30 mm cannot carry more.
- The seam between upper and lower sleeve lofts at the elbow shows as a jagged grey seam line in the shaded sheets (two lofts overlapping; closed in every pose by design).
