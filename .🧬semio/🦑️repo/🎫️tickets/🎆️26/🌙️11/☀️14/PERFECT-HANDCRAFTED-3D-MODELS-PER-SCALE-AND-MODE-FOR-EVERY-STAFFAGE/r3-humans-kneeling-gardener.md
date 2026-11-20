# Round 3 — `humans/kneeling-gardener`

State: written to the catalog (rig, anchors, poses, dimensions, symbols, garden reference, 2D geometry and drawing decisions of 1:50 and 1:100, `volumes`). `r3-validate.ts humans/kneeling-gardener --catalog-only`: `VALID ... schema 0, validateAsset 0`. `volumes-check.ts humans/kneeling-gardener --product`: `PASS`, every rule green at 1:50 and 1:100 (1:200 and 1:500 are `unavailable` in the asset and stay so). No earlier round-3 work existed for this asset; this is a first complete pass.

Scripts (ticket folder): `r3-kneeling-gardener-kit.ts` (constructors), `-body.ts` (1:50 body, every section stated by hand), `-lists.ts` (1:100 body, thermal, skeleton, decisions), `-figure.ts` (assembly), `-roots.py` + `-fk.py` (root fixes, sagittal FK/IK), `-draw2d.ts` + `-draw2d.py` + `-decisions.py` (2D redraw), `-write.py` + `-splice.py` (catalog writer), `-run.sh` / `-all.sh` (rebuild candidate / rebuild, gate and write), `-sheet.ts` (copy of `r3-sheet.ts` with `--file`), `-frames.ts`, `-diag.ts`, `-bisect.ts`, `-suit.ts`, `-probe.ts` (probes).

## 1. Reference (woman of 58, stature 1.66 m in clogs, 64 kg)

| quantity | reference | value used | source |
|---|---|---|---|
| hip joint height | 0.52 to 0.53 H | 0.877 m (rig, kept) | Winter, Biomechanics and Motor Control; brief |
| knee / ankle joint height | 0.285 H / 0.039 H + sole | 0.472 / 0.075 m (kept) | Winter |
| shoulder joint height | 0.818 H = 1.358 m | 1.358 m | Winter; brief |
| C7 (neck base) height | 0.86 H = 1.428 m | 1.427 m | de Leva 1996 (asset reference), female: vertex to cervicale 243.7 mm at 1.735 m |
| head height chin to vertex | H / 7.5 to 7.7 = 0.215 to 0.221 m | 0.215 m | brief; ANSUR II female menton to vertex |
| head breadth / length | 0.146 / 0.187 m | 0.145 / 0.184 m | ANSUR II female means |
| shoulder joint spacing | biacromial 0.365 m minus 2 x 2.5 cm | 0.32 m | ANSUR II female biacromial breadth |
| bideltoid breadth | 0.43 to 0.45 m | 0.442 m with sleeves | ANSUR II female |
| hip joint spacing / hip breadth | 0.17 to 0.18 / 0.34 to 0.36 m | 0.18 (kept) / 0.344 m in trousers | Bell et al. 1990; ANSUR II female |
| upper arm / forearm / hand | 0.159 to 0.186 H / 0.146 to 0.152 H / 0.108 H | 0.2925 / 0.2546 / 0.185 m (bones kept) | Winter; de Leva |
| thigh / shank | 0.245 H / 0.246 H | 0.4055 / 0.3961 m (kept) | Winter |
| chest (bust) breadth x depth | 0.29 x 0.25 m | 0.324 x 0.28 m in a loose shirt | ANSUR II female, age-adjusted by eye |
| waist breadth x depth | 0.27 x 0.21 m | 0.30 x 0.25 m in a loose shirt | as above |
| thigh top / mid / above knee girth | 0.60 / 0.52 / 0.40 m | sections 0.176 x 0.196, 0.15 x 0.164, 0.122 x 0.132 m in trousers | ANSUR II female |
| calf / ankle girth | 0.365 / 0.215 m | 0.112 x 0.128 / 0.062 x 0.074 m | ANSUR II female |
| upper arm / forearm / wrist | 0.29 / 0.245 m girth, wrist 0.052 x 0.036 m | 0.094 x 0.104 (bare 0.086 x 0.094), 0.084 x 0.082, 0.055 x 0.042 m | ANSUR II female |
| hand breadth / thickness, glove | 0.078 / 0.026 m | 0.088 / 0.034 m gloved | ANSUR II female |
| foot / clog | 0.243 m; clog 0.265 x 0.10 m, sole 20 to 25 mm | 0.275 x 0.102 m, sole 20 to 25 mm | Winter; garden clog size tables (my knowledge) |
| lumbar flexion | about 50 to 60 degrees at most; bending is shared with the hips | pelvis 55 %, lumbar 45 % of the lean | orthopaedic range-of-motion tables (my knowledge) |
| ankle plantarflexion when kneeling on the instep | 50 to 70 degrees | 65 degrees (rig limit) | as above |
| wrist ulnar deviation | 30 to 40 degrees | 22 to 35 degrees in every pose | as above |
| hammer grip of a tool | tool 40 to 80 degrees to the forearm | 50 degrees to the hand bone | ergonomics of hand tools (my knowledge) |
| kneeling mat | extra-large EVA mats about 0.71 to 0.76 x 0.40 x 0.04 m | 0.76 x 0.40 x 0.04 m (as drawn and as `areaM2` 0.70 assumes) | catalogue ranges (my knowledge, not looked up) |
| hand trowel | about 0.30 m, blade 0.15 x 0.08 m, 2 to 2.5 mm steel, handle 0.03 m | 0.316 m, blade 0.154 x 0.08 m, 2.5 mm scoop 12 mm deep, handle 0.132 m long, 0.032 m thick | asset `garden-spec`; my knowledge |
| flexible tub (trug) | shallow tubs 0.45 to 0.50 m, 0.2 to 0.25 m high, 3 to 4 mm wall, two moulded handles | 0.50 m top, 0.35 m base, 0.21 m high, 4 mm wall, rolled rim, handles on the rim line | asset `garden-spec`; my knowledge |
| straw hat | crown 0.19 x 0.16 m, brim 0.09 to 0.11 m | crown 0.20 x 0.164 m, 0.10 m high, round brim of 0.198 m radius | my knowledge; see section 6 |
| segment masses, body surface | de Leva 1996, DuBois | unchanged | asset `provenance.references` |

No web search was made; values marked "my knowledge" are not verified against a manufacturer.

## 2. Audit findings

Rig
- Trunk too short, head-and-neck too long: `torso` 0.4339 m put C7 at 1.405 m (0.846 H; reference 1.428 m), `head` 0.2546 m against 0.233 m.
- Shoulder joints too high and too wide: at 1.377 m (0.830 H; reference 1.358 m) and 0.38 m apart (reference about 0.32 m): shoulders of a much larger person.
- `trowel` in line with the fingers (`angleDeg` 0 on the hand bone): no hand holds a trowel as an extension of its fingers.
- Hip, knee, ankle heights, limb lengths, hip spacing: within 1 cm of the reference; kept.
- Root `pad` bone: origin under the hips, `lengthM` 0.6 while the mat is 0.76 m and runs from -0.56 to 0.20 m; kept (see section 6).

Poses (all six)
- The whole forward lean (58 degrees in the default pose, 76 in `reaching-trug`) was put on the lumbar joint with the pelvis upright: beyond what a lumbar spine does.
- Feet at 60 degrees: with a real clog the toe box would sink 2 cm into the mat.
- Left "hand on the thigh": the wrist lay on the thigh axis at hip height and the arm 0.19 m to the side, so in 3D the hand hung beside the thigh.
- Right arm: tool as prolongation of the forearm (see rig).

Anchors: `mouth`, `nostrils` 2 to 4 cm off the face of a 0.215 m head; `crown` inside the hat; `palm-l` 3 cm off the hand axis (a hand is 2.6 cm thick); `instep-*` 4 cm above the foot axis where no clog surface is; `trowel-tip` at 0.30 m from the fist.

Dimensions: `heightM` 1.132 described the old pose; the garden reference said 0.6 x 0.3 m for a mat drawn 0.76 x 0.40 m.

2D drawings (1:50 and 1:100): constructed, not projected. Elevation with one leg only (bound to the far leg), paths of 300 to 1900 characters; front with rest-height ellipses for head and knees, so the front view of a trunk leaning 58 degrees showed an upright figure 1.14 to 1.26 m high; plan of stadiums; different ids for the same part per view (`glove-l` / `hand-l`, `shirt` / `torso` / `trunk`).

Solids (old 1:50, 69 / 56 / 37): box-like shirt, tube limbs with joint balls, blade hands, a hat brim as an implied-closure revolve (kernel-traps rows 149, 150), tub with a 15 to 20 mm wall and a dome of weeds, structure of 37 body-like blobs identical in count at both scales.

## 3. Root fixes (old -> new)

| member | old | new |
|---|---|---|
| `rig.bones[torso].lengthM` / `.mass.comX` | 0.4339 / 0.1981 | 0.456 / 0.2082 (same de Leva ratio 0.4566) |
| `rig.bones[head].x` / `.lengthM` / `.mass.comX` | 0.4339 / 0.2546 / 0.1273 | 0.456 / 0.2325 / 0.1163 (vertex stays at 1.66 m) |
| `rig.bones[upper-arm-l|r].x` / `.z` | 0.4057 / +-0.19 | 0.387 / +-0.16 |
| `rig.bones[trowel].angleDeg` / `.lengthM` / `.mass.comX` | 0 / 0.3 / 0.15 | 50 / 0.26 / 0.11 (fist centre to tip; the handle runs 0.056 m behind the fist) |
| `anchors[mouth]` / `[nostrils]` / `[crown]` | (0.1067, -0.0839) / (0.1323, -0.093) / (0.2623, 0) | (0.0655, -0.0905) / (0.088, -0.105) / (0.266, 0) on lips, nose base, hat top |
| `anchors[palm-l]` / `[instep-l|r]` / `[trowel-tip]` | (0.09, -0.03) / (0.06, 0.04) / (0.3, 0) | (0.09, -0.019) / (0.17, -0.014) / (0.26, 0) |
| poses: `pelvis`, `torso` | 0 and the whole lean | 55 % and 45 % of the lean (default pose -32 / -26; upright -4 / -4; reaching -42 / -34; dig-up -33 / -27; dig-pull -30 / -24) |
| poses: `thigh-l|r` | 10 / 11 (14 / 15 reaching) | plus the pelvis tilt (default 42 / 43), so the legs stay where they were |
| poses: `foot-l|r` | -60 / -60.5 | -65 |
| poses: left arm | wrist on the thigh axis | solved again: palm on the front of the left thigh (0.24 m below the hip, 0.13 m upright, 0.31 m reaching), hand along the thigh |
| poses: right arm, `trowel` | tool in line with the arm | solved again for the hammer grip: tip and blade direction per pose (default tip at (0.70, 0) at -50 degrees), wrist deviation -22 to -35 degrees, `trowel` offset -5 / 0 / -40 / 0 / -10 / 0 |
| `dimensions.heightM` | 1.132 | 1.098 (pose height of the corrected body; width, depth, mass unchanged) |
| manifest row `kneeling-gardener` `heightM` | 1.132 | 1.098 |
| `symbols[gardener-block]` height (points and both texts) | 1.145 | 1.098 |
| `provenance.references[garden-spec].citation` | 0.6 x 0.3 x 0.04 m pad | 0.76 x 0.4 x 0.04 m mat, trowel and tub as modelled |
| `representations[50|100].geometry.*` | 95 + 77 shapes | redrawn, see below |
| `representations[50|100].decisions` | texts of the old drawing | rewritten (en, de) |

Clips reference poses only and needed no change. `r3-kneeling-gardener-probe.ts`: all 96 states (6 poses and 21 instants of both clips at both scales) evaluate.

2D redraw (ruling E1): every filled shape is the outline of the body solids of its bone, Douglas-Peucker at 0.8 / 1.5 mm, one id, tone, region and material per part in all three views. 1:50: 84 / 66 / 67 shapes with 861 / 745 / 745 points (elevation / front / plan); 1:100: 49 / 45 / 46 shapes with 468 / 389 / 389 points. Elevation: bone-local outlines, both legs and both arms. Front and plan: shapes with a `solid` (limbs, neck, hat, tub, each weed heap, trowel handle revolved; trunk, head, hair, hands, feet, mat, blade, flat leaves swept along the bone normal; handles, ears, eyes, stalks, trowel neck, the two pitched leaves swept laterally), so they follow the bones in every pose. Strokes, side elevation only: shirt hem, trouser hems, tub rim; at 1:50 also brow, mouth, jaw line, placket, trouser and sleeve seams, sole lines, blade rib, mat edge; hatch `straw-weave` kept on the hat crown. Plan: the tub rim as an unfilled circle under the weeds.

## 4. What was modelled (solids body / thermal / structure)

- **1:50 — 102 / 147 / 112.** Body: shirt as one garment loft (hem over the hips, waist, bust, rounded upper back, sloping shoulders, neck opening) with collar; sleeves that start as a deltoid dome and end in a rolled cuff; bare elbow and forearms (left forearm turned in to the thigh, wrists flat or upright by the turn of the hand); glove cuffs; left hand flat with four two-part fingers and thumb, right hand a fist with dorsum, thenar, four fingers of three parts wrapped round the handle and the thumb over them; neck, head with jaw and chin, nose, lips, ears, eyes; hair with bun; hat crown, band, drooping round brim; trouser seat with gluteal masses, thighs closing over the knee, knee patches, shanks with the calf to the back, socks, clogs (sole, upper, heel strap, toe sole, toe box); mat; tub (4 mm shell, rolled rim, two handles), five weed heaps, nine leaves, three stalks; trowel (turned handle, cranked neck, hollow blade in three extrusions). Thermal: nested shells per layer for shirt, sleeves, trousers, gloves (2), clogs (3), hat (straw over hair pack); face as its own solid in front of the ears, bare neck only, shoulder domes counted to the trunk shirt, weeds left out, outlet patches under the nose, on the lips and on the hat top. Structure: skull, facial block, mandible, cervical and thoraco-lumbar spine with curvature, 8 rib hoops, sternum, clavicles, scapulae, acromia, sacrum, hip bones, pubic rami, femora (head, neck, trochanter, bowed shaft, condyles), patellae, tibiae, fibulae, tarsus, metatarsus, toe phalanges, humeri, radius and ulna (crossed on the left), carpus, metacarpus, finger and thumb bones, 16 joint solids, 4 masses; mat and tub as their own members, trowel as tang, neck and blade. `structure.supports` is empty, so there are no support solids.
- **1:100 — 49 / 44 / 65.** Authored anew with fewer, longer sections: no eyes, lips, ears, band, collar, patches, straps; hair and bun one mass, cuff part of the sleeve, gluteal masses part of the seat, one clog form per bone, mitten hand with thumb, one fist with thumb, two weed heaps with four leaves and two stalks, blade 15 mm deep. Skeleton with merged parts and 4 rib hoops.

Final check line: `PASS`. Silhouette IoU elevation / front / plan: 1:50 0.995 / 0.991 / 0.993 (p95 1.9 / 3.8 / 5.6 mm), 1:100 0.996 / 0.984 / 0.990 (p95 3.8 mm each). Thermal area deviations at 1:50: face 0.23, hat 0.29, shirt-torso 0.01, sleeves 0.13, forearms 0.25, gloves 0.16, trousers 0.17, clogs 0.22, pad 0.05, trowel 0.13, trug 0.06.

## 5. What I saw in the renders

Seen in the final or last-but-one state (the last edits were the brim inner radius and the pubic rami):
- `r3-sheet` 1:50 body, left side, large: reads at once as a woman kneeling and leaning on her left hand; hips flexed with the buttocks behind the knees, rounded back, hat tilted with the head, bun under the brim, nose and chin in profile, rolled sleeves, glove cuffs, knee point and shin on the mat, clog toe on the mat, fist round the trowel with the blade into the ground.
- 1:50 body, three-quarter from the right front (turn 60) and from the left front (turn 140): left palm on the thigh with spread fingers, right fist with finger rolls, scooped blade, tub with rim, handles and leaves over the rim.
- 1:50 body, front, large: sloping shoulders with the sleeve domes, arms, shirt hem between the thighs; showed a slot between crown and brim, closed afterwards and seen closed in the final six-view sheet.
- 1:50 body, final six-view sheet with rig: joints sit in the shoulders, elbows, wrists, hips, knees and ankles of the model; the trowel bone runs along the tool.
- 1:50 body six-view sheets in `kneeling-upright` and `reaching-trug`: upright figure with correct proportions, trowel held forward; reaching pose closed at hip, shoulder and elbow (the right arm was extended further afterwards, re-rendered, not opened again).
- 1:50 structure, six views and three-quarter large: a recognisable skeleton; the pubic rami looked detached in the three-quarter view and were re-attached afterwards.
- 1:50 thermal cut, six views and three-quarter: shells nested, nothing outside the body.
- 1:100 body, six views and three-quarter large; 1:100 structure, six views: same figure in fewer masses, skeleton readable.
- `r3-2d` 1:50 (final state) and 1:100 (one edit earlier), catalog: clean line drawings in all three views, height 1.09 m, depth 0.50 m. In the line views the tub in plan is an empty circle and the hat in front and plan one ellipse, while the solid views show weeds, leaves, crown and band: the line view draws shapes of one region as one outline. I did not find a way to keep those inner outlines in the line view.
- `volumes-render` 1:50 overlay (wide, front panel cropped): green except thin rims; opened before the last 2D corrections.
- Rendered, not opened: `dig-cycle` at 0.2 s, 1:50 back and top large, 1:100 thermal, `r3-2d` of `kneeling-upright` and `reaching-trug`, the final overlays of both scales.

## 6. Still imperfect

- Grade as I see it: body 1:50 B+ to A-, 1:100 B+, structure A-, thermal B+. Not "perfect".
- Lofts are superellipses: no separate breasts, scapulae or calf heads; the trunk is one rigid bone from L5 to C7, so the back is a shaped line, not a bending spine; the shirt has a hanging belly and a rounded back but no modelled folds (fold solids read as lugs and were removed).
- Small rendering artefacts (triangles) where the shirt hem meets the gluteal masses; the knee and elbow seams are intersection lines, not modelled creases.
- The left forearm turns in to the thigh by shape (36 mm at the wrist) because the rig is sagittal and has no shoulder abduction; it does so in every pose.
- The hat: brim radius 0.198 m (0.191 m at 1:100) and, at 1:100, hair left out of the thermal list, because the declared `hat` area counts one brim face (C18 forbids changing it); filed in `author-requests.md`. Brim 7.5 mm thick for the feature rule (real straw 3 to 4 mm).
- 2D front and plan: the trunk is a swept outline with a box section; alone, pelvis and torso reach only IoU 0.78 to 0.85 against their solids in the pitched pose (whole asset 0.98 to 0.99). An elliptic revolved shape would be right but the evaluation ignores `lateralScale` (filed in `tool-requests.md`). Revolved shapes are circular, so the oval hat crown is drawn round in front and plan.
- 2D detail strokes exist only in the side elevation; front and plan carry the detail as filled shapes, and the line views of front and plan lose the weeds in the tub and the hat crown (one outline per region).
- `pad` bone `lengthM` 0.6 and its origin under the hips were not changed (rig root; every pose hangs on it).
- Mat, clog, trowel, tub and hat dimensions are from my knowledge, not from a looked-up catalogue.
- Tests and fixtures that embed this asset (`structure-projection`, `multiview-coverage`, `model-catalog`, per `area-decisions.md`) were not regenerated or run; the rig, poses and drawings changed.
- Structure solids use material `skin` (the asset has no bone material).
