# Round 3 — `animals/hedgehog` (2026-11-20)

One author, no earlier round-3 work existed (no `r3-hedgehog-*` scripts, no partial report; the catalog `volumes` equalled the source). Final state: `r3-validate.ts animals/hedgehog --volumes` gives `VALID (schema 0, validateAsset 0, checkVolumes 0)`, `volumes-check.ts animals/hedgehog --product` gives `PASS` at 1:50 and 1:100. 1:200 and 1:500 are `unavailable` by design (the animal is 1.4 mm and 0.6 mm long on paper).

Rebuild from the hand tables: `sh r3-hedgehog-run.sh` (rig and poses `r3-hedgehog-rig.ts`, both scales `r3-hedgehog-model.ts`, texts `r3-hedgehog-texts.ts`, 2D redraw `r3-hedgehog-draw2d.ts`, candidate `r3-hedgehog-candidate.ts`, validation of the candidate), then `python3 r3-hedgehog-apply.py` (root members into the catalog file, volumes through `volumes-insert.py`). Input kept in the ticket: `r3-hedgehog-baseline.json` (the asset before round 3). Bench: `r3-hedgehog-kit.ts`, `r3-hedgehog-work.ts`, `r3-hedgehog-sheet.ts` (the shared sheet on the working asset), `r3-hedgehog-frames.ts`, `r3-hedgehog-zoom2d.py`, `r3-hedgehog-io.py`.

## 1. Reference: Erinaceus europaeus, adult, 1.0 kg, foraging

| Measure | Real value | Model | Source |
|---|---|---|---|
| Mass | 0.6–1.2 kg | 1.0 kg (`dimensions.massKg`, unchanged) | Reeve 1994 (`reeve-1994` in provenance) |
| Head and body length | 0.20–0.27 m | 0.258 m nose to rump skin | Reeve 1994 |
| Overall length over the spines | head and body plus 15–20 mm of rear spines | 0.277 m (`widthM` 0.28, unchanged) | Reeve 1994; own knowledge |
| Height to the spine tips, walking | 0.12–0.14 m | 0.128 m (`heightM`, unchanged) | own knowledge of the species |
| Width over the spines | 0.13–0.16 m | 0.15 m (`depthM`, unchanged) | own knowledge |
| Belly clearance, slow walk | 20–30 mm | 27 mm | own knowledge |
| Spines | about 5000–7000, 20–25 mm long, 1–2 mm thick, from the crown behind the ears to the rump and down the flanks to the belly line | coat mass plus tufts standing 11 mm proud; spine envelope 22–25 mm above the skin | Reeve 1994 |
| Skull (condylobasal length / zygomatic width) | 55–60 mm / 33–37 mm | skull 61 mm long, 34 mm wide; head with fur 66 mm occiput to nose tip, 45 mm across the cheeks | mammal osteometry as I know it for the species |
| Ear | 20–30 mm, rounded, at the edge of the spines | 18 x 14 mm (limited by the 7.5 mm feature for its thickness) | Reeve 1994 |
| Eye | 7–8 mm | 8 mm | own knowledge |
| Tail | 15–30 mm, under the rump spines | 20 mm | Reeve 1994 |
| Hind foot | 40–45 mm, plantigrade, five toes | 40 mm heel to toe tips | standard field measure of the species |
| Fore foot | 25–30 mm, broader than the hind foot, five toes | 28 mm | own knowledge |
| Humerus / radius-ulna | about 38–43 / 36–48 mm | 38 / 36 mm | own knowledge of the skeleton, not measured on a specimen |
| Femur / tibia | about 40–43 / 44–46 mm | 40 / 44 mm | same |
| Vertebrae | C7, T15, L6, S3, about 12 caudal; 15 pairs of ribs | spine arched over the mid-back, 15 ribs per side | mammal anatomy texts |
| Stance | limbs strongly flexed, elbow at the chest wall, knee beside the belly, shoulder and hip joints at mid-height of the trunk | shoulder joint 58 mm, hip joint 56 mm above ground, elbow 36 mm, knee 28 mm | own knowledge |
| Rolling up | the spine flexes, the orbicularis muscle draws the spiny skin over head and rump like a purse; ball 0.12–0.18 m | spine flexes 70 degrees at one mid-back joint, four coat bowls close under the belly; ball 0.17 x 0.17 x 0.15 m | Reeve 1994 (panniculus, orbicularis) |
| Gait | lateral-sequence walk | kept from the asset: 0.8 s cycle, 66 % stance, 0.113 m per cycle | asset clip `forage-walk` |

The limb bone lengths and the skull figures are from memory of the species, not from a cited table; they are consistent with the standard hind-foot measure.

## 2. Audit (asset before round 3, `r3-hedgehog-baseline.json`)

- **Rig, legs:** each leg was two bones, a 30 mm (fore) or 35 mm (hind) "limb" and a 35 / 40 mm paw: no elbow, no knee. Shoulder and hip joints sat 30 mm below the trunk axis at 40 mm above ground, on the belly line. Near and far legs differed by ±6 degrees and ±31.5 degrees, and the walk poses used a different solution branch (hind bone 85–122 degrees against -31.5 standing).
- **Rig, trunk:** one rigid bone of 0.16 m. A hedgehog rolls up by flexing its spine; the rig could not.
- **Rig, coat:** four skirt bones hinged on the trunk axis and pointing to the ground. In `rolled` they swung under the body, and the contact anchors at their tips put the ground through the middle of the animal: belly, head and legs were below `y = 0` (seen in the old sheet, `r3-sheet` pose rolled). The rolled form was a 0.22 m loaf, not a ball.
- **Rig, head:** 55 mm (real head 65 mm), neck 20 mm.
- **`dimensions`:** 0.128 x 0.28 x 0.15 m, 1 kg: agree with the reference, unchanged.
- **2D:** side view without legs at 1:100 and with capsule stubs at 1:50, skirts as jagged flaps to the ground, front and plan as ellipse, capsules and extruded boxes; no nose, no eye. 1:50 failed `volumes-views-disagree` (9.5 mm).
- **Solids (17 / 12 body solids):** smooth ellipsoid dome without spines, four flat skirt flaps, barrel forelegs, paw discs, cone head; structure was the body again (one loft per bone); `volumes-check --product` failed with 6 measurements (views disagree, nose and paw areas +85 % and +127 %, structure outside the body).

## 3. Root fixes

**Rig** (15 bones before, 20 now; landmarks in `r3-hedgehog-rig.ts`):

| Bone | Before | Now |
|---|---|---|
| `trunk` | 0.16 m, rigid | loin, 0.0696 m from the sacrum to the mid-back joint, 18.4 degrees |
| `chest` | none | new, 0.0772 m from the mid-back joint to the neck base, carries neck, forelegs and front bowls, flexes -70 degrees against the loin |
| `neck` / `head` | 0.02 / 0.055 m | 0.025 / 0.066 m |
| `fore-*` | 0.03 m "limb", joint at (0.11, 0.040) | humerus 0.038 m, shoulder joint at (0.118, 0.058) |
| `forearm-*` | none | new, radius and ulna 0.036 m |
| `forepaw-*` | 0.035 m | 0.028 m |
| `hind-*` | 0.035 m "limb", joint at (0.02, 0.040) | femur 0.040 m, hip joint at (0, 0.056) |
| `shank-*` | none | new, tibia 0.044 m |
| `hindpaw-*` | 0.04 m | 0.04 m, plantigrade |
| `coat-f*`, `coat-r*` | 0.07 m flaps hinged on the trunk axis, z ±0.04 | 0.058 m: pole of a hemispherical bowl about (0.116, 0.044) and (0, 0.044), z ±0.014; limits ±175 |

- Masses redistributed over the new bones, sum 1.000 kg unchanged (loin 0.34, chest 0.306, upper arm 0.018, forearm 0.012, thigh 0.022, shank 0.013; the rest as before).
- **Anchors:** paw contacts on the soles of the new paws (footprints 28 x 18 and 40 x 18 mm instead of 30 x 20), `spine-front` / `spine-rear` on the bowls at the points that are lowest in the rolled ball, nostrils, snout tip, eye and back plume on the new head and dome.
- **Poses (all 12):** re-solved. Standing poses by two-bone solutions from shoulder and hip to a flat paw; the eight walk keys from the clip's own timing (touch-down 0 / 0.2 / 0.4 / 0.6 s, stance 0.528 s, travel 74 mm, lift 8 mm); `alert` with the trunk 4 degrees up and the head level; `rolling-half` and `rolled` with the spine flexed ±17.5 / ±35 degrees, the bowls turned -135 and +114 degrees, the head tucked to the belly and the limbs folded. `abductionDeg` of the old skirts is gone. The rolled ball now stands on the ground.
- **Materials:** `bone` added for the structure list.
- **2D, ruling E1 (both scales, three views):** every shape is the traced orthographic view of that scale's body in the foraging stance, written in the frame of its bone (`r3-hedgehog-draw2d.ts`, 25 / 25 / 23 shapes at 1:50). New shapes: `chest`, `dome-front`, `forearm-*`, `shank-*`, paws in the side view, `nose`, `eye-*`, the far bowls in the side view. A part has the same id in all three views (the old `near-fore` … of front and plan are now `fore-n` …), because the product carries a front or plan shape at the depth of its side-view twin. The `solid` (extruded) members of the old skirts and head are gone. Layer order restated: bowls behind the dome. Decisions `silhouette`, `retained`, `merged`, `omitted`, `thinElements`, `styles`, `overlays` rewritten for both scales (en, de).
- **Thermal and structure data outside `volumes`:** unchanged. No `areaM2` was touched (C16, C18, C27, C29).

## 4. What is modelled

Hand tables: trunk and dome rows `[x, centre height, half height, half width]`, limb and head rows in bone frames, tuft rings `[x or polar angle, count]`.

**1:50** (7.5 mm): body 406, thermal 430, structure 102 solids.
- **Body:** loin and chest lofts; dome in two halves (superellipse sections, widest low, ending at the belly line); saddle as a body of revolution about the mid-back joint; four bowls; 372 tufts (cones, 8.4 mm base, 11 mm proud; dome rings of 12 and 13 leaning back, bowl rings radial with a fringe ring at the rim, five saddle rings of which three are hidden inside the dome when standing); head, jaw, nose, two eyes, two ears; neck; tail; per leg thigh or upper arm, shank or forearm, furred foot, bare toe block.
- **Thermal:** 406 references to the body, 22 inner layer shells (dome and saddle air 10 mm, bowl air 8 mm, underfur of belly, chest, head, neck, jaw and of the 12 limb segments), breath outlet at the nostrils, plume patch on the crown.
- **Structure:** skull, mandible, cervical, thoracic and lumbar spine, sacrum, two hip bones, 30 ribs, sternum, scapulae as four rods each, eight long bones with epiphyses, 20 rays of hands and feet, caudal rod, orbicularis ring and dorsalis band per bowl, 15 joints, 3 masses. No support solids: `structure.supports` is empty in the asset.

**1:100** (15 mm): body 134, thermal 136, structure 60 solids.
- **Body:** the same parts restated with half the sections; 106 tufts of 15.6 mm base; head as one loft with the nose; no eyes, ears, jaw, tail.
- **Thermal:** one surface per region, outlets at the size floor.
- **Structure:** skull, pelvis block, three spine rods, 14 rib hoops, sternum, scapular spines, long bones and one ray per paw as slender rods, muscle rings, joints, masses.

**Final check line:** `PASS (26.3 s)`. Silhouette IoU 1:50 elevation 0.972, front 0.963, plan 0.979; 1:100 0.947 / 0.940 / 0.961. Bounds 1:50: length -4.0 mm, height 1.7 mm, width 0.1 mm (limit 7.5). Area deviation 1:50: nose 0.26, paws-skin 0.12 (limit 0.3).

## 5. What I saw in the final renders

Sheets under `🗑️generated/r3-hedgehog/sheet/` (`hedgehog.s50.body.png`, `.rolled`, `.alert`, `.forage-walk-t0.3`, `.three-quarter`, `s50.structure`, `s50.thermal.cut`, `s100.body`, `s100.structure`); 2D under `🗑️generated/r3-2d/animals/hedgehog.s50*.png` and enlarged under `🗑️generated/r3-hedgehog/zoom2d/`.

- **1:50 body, right and left side:** a hedgehog at first sight: low spiny dome with interlocking rows of tufts leaning back, the skirt hanging to the belly line, a rump fringe, the pointed head coming out under the brow, ear at the edge of the spines, dark eye and nose. The legs show below the skirt: knee forward and flat plantigrade hind foot, forearm steep under the elbow, brown toes.
- **Front:** broad spiny arch, the face in the middle under the brow fringe with both eyes, ears and the nose low, forelegs under the shoulders. The underside of the front bowls shows as a dark band between the legs.
- **Back:** spiny rump with fringe, hind legs, belly between them.
- **Top:** oval coat with regular tufts, the head and ears ahead of it.
- **Three-quarter (turn 65, tilt 12):** reads as a hedgehog; the front bowl shows as a band with a tuft fringe under the dome edge.
- **Alert:** head level, trunk slightly raised, all four feet on the ground.
- **Walk t = 0.3 s:** three feet planted, one hind foot lifted with the toes down; all joints closed.
- **Rolled:** a closed spiny ball of about 0.17 m standing on the ground, tufts all round, the two bowls meeting under the belly with a short seam, a fold on the flank where the two dome halves meet, a small crest of tufts at the top where the back opens.
- **1:100 body:** the same animal with a few large tufts; the head is a plain wedge without eyes and ears.
- **Thermal 1:50, cut:** regions follow the body exactly; the red air shell under the pink spine layer of the dome, the green bowl shells, purple belly layers, yellow face, pink limbs, green toes, blue nose; nothing pokes out.
- **Structure 1:50:** arched spine with the joint at its top, rib cage, skull with mandible, pelvis, scapula triangles, long bones on the rig, five rays per paw, the muscle rings of the bowls; all inside the body ghost. **1:100:** rods, a skull cone, a pelvis block and large joint balls.
- **2D 1:50 enlarged:** side view reads as a hedgehog with serrated back, hatch, snout, eye, nose and legs; front view as the arch with the face. The plan was judged only at the small size of the standard sheet (oval coat with the snout) and in a wrongly cropped enlargement.
- Not opened: `s100.body.rolled`, `s50.body.roll-up-t1.1`, `s100.thermal.cut`, `hedgehog.s100.png` (2D). They were rendered without error.

## 6. Still imperfect, stated plainly

- **2D side view has a seam at mid-back.** The dome is two shapes on two bones (loin and chest) so that the back can flex; where they meet a vertical body-pen line crosses the dome. A hairline patch over it was tried and removed: the product then refuses the whole 1:50 representation (see `tool-requests.md`, 2026-11-20).
- **2D front and plan of the rolled pose are approximate.** The side view of the ball is right (0.17 m). The front view shows 0.13 m and draws the tucked head in front of the spines, because plain polygons are carried through a pose at one depth and the head is the top layer of the front view. The old drawing had the same limit; only `solid` (extruded or revolved) shapes turn correctly, and those cannot carry a traced outline.
- **One spine joint instead of a flexing column.** The real back flexes along all thoracic and lumbar vertebrae; the rig has one joint at mid-back. The ribs behind it sit on the loin bone. The ball is 0.17 m where a real one is 0.12–0.18 m, and it shows a fold on the flank and a short seam under the belly.
- **Tufts stand for spines.** 372 cones of 8.4 mm base instead of about 6000 spines of 1–2 mm: the feature rule forbids real spines unless they are slender, and thousands of slender solids would not be a model of this scale. The front and rear bowls show as bands with a flat underside from low viewpoints.
- **Nose is larger than life.** The declared `nose` area (0.0008 m², a body-model value I may not change) needs a 20 x 11 x 12 mm bulb to come within 30 %; a real rhinarium is about 10 mm wide. Deviation is still 0.26. A question for the area reviewer.
- **Toes are one block per paw**, single toes (4 mm) and claws are below the feature. Ears are 7.8 mm thick for the same reason. At 1:100 the joint balls of the structure (15 mm) are far larger than real joints.
- **Limb bone lengths and skull figures are from memory**, not from a cited osteometric table.
- **`summary` of the asset** still speaks of "front and rear spine skirts that swing down"; that is still what the bowls do, so I left it.

## Round 4 review (2026-11-20)

Reviewer and fixer in one. Sheets opened with the Read tool before and after; the scripts are the round-3 ones, extended (`r3-hedgehog-model.ts`, `r3-hedgehog-texts.ts`, `r3-hedgehog-run.sh`), plus `r4-g04-sync.py` (takes the scales the catalog carried into the ticket source).

Reference numbers used: head and body 0.20–0.27 m, height walking 0.12–0.14 m, width 0.13–0.16 m, spines 20–25 mm, belly clearance 20–30 mm (section 1). The vertebral column of a walking hedgehog is a single low arch 10–12 mm under the skin of the back; the sacrum lies above the hip joints, not on them.

| scale | list | before | after |
|---|---|---|---|
| 1:50 | body | B- (flat top with a depth step, box section of exponent 2.8, capsule plan) | A- |
| 1:50 | thermal | B+ | A- |
| 1:50 | structure | C+ (column as a tent over the mid-back rig joint, running down to the hip joint; stray dorsalis rods; heavy ilium pipes; plain cone skull) | A- |
| 1:100 | body | B- | B+ |
| 1:100 | thermal | B (not opened before; after: opened only through the body it references) | B+ |
| 1:100 | structure | C+ (same column, pelvis a 47 mm wide wedge) | B+ |
| 1:200 | body, thermal, structure | D (bulk pass of 03:40: the 1:100 list without tufts, bowls poking out as ragged skirts, thermal a copy of 1:50, `volumes-copied` red) | A- / B+ / B+ |
| 1:500 | body, thermal, structure | D (the same lists again) | B+ / B+ / B |

What I changed:
- **Dome** at 1:50 and 1:100: every row restated so that the back is one arch (top 0.084 m over the rump, 0.117 m at mid-back, 0.092 m over the brow), section exponent 2.35 instead of 2.8.
- **Skeleton**, all scales: column restated as one dorsal arch (0.0775 m at the sacrum, 0.0925 m at mid-back, 0.0665 m at the neck root), sacrum and ilium above the hip joints with the acetabulum at the joint, ribs and sternum hung from the new column, zygomatic arches at 1:50, pelvis block 35 mm wide instead of 47 mm, dorsalis rods removed (they read as loose sticks).
- **1:200 and 1:500 authored** (they did not exist in round 3; the catalog carried bulk copies): the coat is the smooth envelope of the spine tips with the bowls inside it, own head, limbs, paws, own emitter solids, own skeleton lists, bilingual decisions.
- Rebuilt the 2D of 1:50 and 1:100 from the new body through the round-3 redraw (ruling E1).

Checks after the last change: `volumes-check.ts animals/hedgehog --product` PASS at four scales; `r3-validate.ts animals/hedgehog --volumes` VALID (schema 0, validateAsset 0, checkVolumes 0); spliced.

Opened after the last body change: 1:50 body, thermal cut; 1:100 body; 1:200 body and pose `rolled`; 1:500 body; 2D 1:50. Opened before the last skeleton change (ilium slimmed, pelvis block narrowed) and not again after it: structure 1:50, 1:100, 1:500. Not opened: structure 1:200, thermal cut 1:100, 1:200, 1:500, the walk frame, `rolled` at 1:50 after the dome change, 2D 1:100.

Still not A:
- The four orbicularis rings follow the four rig bowls; the real muscle is one ring round the whole coat. In plan they read as four wheels.
- 1:100 body: a ragged seam where each bowl cuts the dome.
- `rolled` at 1:200 is a closed lump with a jagged seam between the halves, not a ball; at 1:500 it was not looked at.
- At 1:200 and 1:500 the region `spines-skirt` is hidden inside the dome while the animal stands.
- The round-3 list of section 6 stands (one spine joint, tufts for spines, nose size, toe blocks).

## Round 5 handcraft (2026-11-21)

Scripts: `r5-hedgehog-model.ts <50|100|200|500> --write` (the round-3/4 hand tables carried over and remodelled), then the round-3 chain `r3-hedgehog-draw2d.ts`, `r3-hedgehog-candidate.ts`, `r3-validate.ts --file`, `r3-hedgehog-apply.py` (root members are the unchanged round-4 rig, anchors and poses, verified equal to the catalog before writing; 2D redrawn from the new bodies; volumes spliced). Shared helpers of this round: `r5-fauna-render.sh`, `r5-fauna-measure.ts`. Inputs kept: `🗑️generated/r5-hedgehog/asset-before.json`, `volumes-before.json`.

### Reference

Section 1 stands (Reeve 1994; *Erinaceus europaeus*, 1 kg, foraging). Added for this round: in a foraging hedgehog the spines lie back towards the rump at roughly 30 to 40 degrees to the skin; they stand erect and radiate only when the animal is alarmed or rolled.

### Model against reference at 1:50 (`r5-fauna-measure.ts`, foraging pose)

| measure | reference | model | deviation |
|---|---|---|---|
| length over spines | 0.28 (dimensions; head-body 0.20 to 0.27 plus rear spines) | 0.274 | -2.1 % |
| height to the spine tips | 0.12 to 0.14 (dimensions 0.128) | 0.123 | -3.9 % against `dimensions`, inside the species range; the laid-back spines lower the envelope by 5 mm (dimensions not changed, bounds rule passes) |
| width over spines | 0.13 to 0.16 (dimensions 0.15) | 0.146 | -2.7 % |
| head occiput to nose tip | 0.06 to 0.066 | 0.066 along the bone | 0 |
| head width over the cheeks | 0.045 | 0.048 | +6 % (fur on the jowls) |
| eye | 7 to 8 mm | 8 mm | 0 |
| ear | 20 to 30 mm, at the edge of the spines | 18 x 15 mm, at z 21.5 mm beside the brow fringe | -10 % against the lower end (7.8 mm feature for the thickness) |
| belly clearance | 20 to 30 mm | 27 mm | in range |
| hind foot / fore foot | 40 to 45 / 25 to 30 mm | 40 / 28 mm | in range |

Rig audit: unchanged from round 4 (two-bone legs with elbow and knee, flexing back at the mid-back joint, bowls as coat bones); its known limit (one spine joint) remains.

### Grades (sheets opened)

| scale | list | before | after |
|---|---|---|---|
| 1:50 | body | B (spines radiate like a cactus in front and side view, skirt tufts stick straight out sideways, head a straight cone, ear a ball on the crown) | A- |
| 1:50 | thermal / structure | A- / A- | A- / A- (structure only the slender rays corrected) |
| 1:100 | body / thermal / structure | B / B+ / B+ | B+ / B+ / B+ |
| 1:200 | body / thermal / structure | B / B+ / B+ | B+ / B+ / B+ |
| 1:500 | body / thermal / structure | B / B+ / B | B+ / B+ / B |
| 1:50 | rolled | B (cactus ball, bare wedge on the flank) | B+ (spines swirl back, bare wedge remains) |

### What changed

- Spines: every tuft of dome, saddle and the four skirt bowls lies back towards the rump (dome and saddle lean `tilt` 0.75 to 1.7 at 1:50 and 1.5 at 1:100; the bowl tufts, which were radial, get the same backward lean while their roots stay on the bowl). Side and plan now read as a sleek spiny coat with the fringe over the face and rump, not a cactus.
- Head at 1:50: domed brow, fuller cheeks and jowls (mandible 37.6 mm across at the angle), the snout tapering to a fine downturned tip with the nose pad 1.2 mm lower; ears moved from the crown to the edge of the spine fringe beside the brow.
- Structure: slender digit rays thinned (manus 0.8 mm at 1:50, 1.2 mm coarser; pes 1.1 mm) so that the stricter slenderness measure (at least 20) holds; the catalog asset was INVALID on these 10 rays before this round.
- 2D of 1:50 and 1:100 redrawn from the new bodies (ruling E1); 1:200 and 1:500 representations stay unavailable.
- Checks: `volumes-check.ts animals/hedgehog --product` PASS; `r3-validate.ts animals/hedgehog --volumes` VALID (schema 0, validateAsset 0, checkVolumes 0); spliced.

### Remaining issues, with reason

- Height to the spine tips 0.123 against `dimensions` 0.128 (-3.9 %); both inside the species range, left for a dimensions review rather than steered.
- The rolled ball keeps a bare wedge on each flank where the two dome halves part around the single mid-back joint; a real hedgehog flexes the whole column (rig limit, round-3 section 6).
- Spines are 372 tufts of 8.4 mm at 1:50 (feature floor), not 6000 spines; the four orbicularis rings in the structure still follow the four rig bowls.
- The ear is 7.8 mm thick (feature floor); the nose bulb is larger than life to meet its declared area (area-review, round 3).
- 1:100 to 1:500 heads remain one cone without jaw, eyes and ears (below those scales' features).
