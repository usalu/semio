# Round 3 — `humans/pregnant-person`

State: root fixes, 2D redraw and volumes are in the catalog asset (written by `r3-pregnant-person-roots.ts --commit` member by member, then `volumes-insert.py`). `r3-validate.ts humans/pregnant-person --catalog-only`: `VALID ... schema 0, validateAsset 0`. `r3-validate.ts humans/pregnant-person --volumes`: `checkVolumes 3`, all three are C18 garment areas (below). `volumes-check.ts humans/pregnant-person --product`: `FAIL: 3 measurement(s) over 2 rule line(s) (10.4 s)` — only `volumes-thermal-coverage` 1:50 (`leggings`, `ankle-boots`) and 1:100 (`leggings`), filed in `author-requests.md` ("2026-11-20 r3 humans/pregnant-person"). Every other rule is green at every scale (silhouette, bounds, cross-scale, monotonic, feature, bones, slender, copied, structure coverage). Manifest row updated (heightM 1.65 -> 1.7, widthM 0.69 -> 0.525).

No earlier round-3 work existed (no `r3-pregnant-person-*` files; catalog untouched since 2026-11-18, volumes source identical to it). Everything is from this run.

Scripts (ticket folder): `r3-pregnant-person-data.ts` (landmarks), `-forms.ts` (measured trunk, seat and abdomen slices), `-roots.ts` (rig, anchors, legs, arm solver `--solve`, clips, texts, dimensions, candidate, `--commit`), `-kit.ts` (station helpers, copied from the leaning-person kit, neck added), `-s50.ts` (1:50 body), `-th50.ts` (1:50 thermal), `-st50.ts` (1:50 skeleton with fetus), `-coarse.ts` (1:100, 1:200, 1:500), `-build.ts`, `-draw2d.ts` (2D redraw), `-texts.json` (decisions), `-dimensions.json`, `-fmt.ts`, `-io.ts`, `-fk.ts`, `-check.ts`, `-sheet.ts`. Cache and originals: `🗑️generated/r3-pregnant-person/` (`original.json` is the catalog as found and the base of the roots script; keep it).

## 1. Reference (woman 1.66 m barefoot, week 36, 71 kg; ankle boots 25 mm heel, 12 mm forefoot)

| quantity | reference | asset now | source |
|---|---|---|---|
| boot lift at the ankle | heel 25, ball 12 mm, heel contact 0.04 m behind, ball 0.128 m ahead of the ankle | 21.9 mm | product knowledge (women's block-heel Chelsea/zip boots, from memory) |
| ankle | 0.039 H = 0.065 m barefoot | 0.0866 m shod | Winter 2009 |
| hip joint | 0.52 H = 0.863 m | 0.869 barefoot (0.523 H), 0.891 shod | Winter 2009; de Leva thigh/shank kept |
| knee | 0.285 H = 0.473 m | 0.462 barefoot (shank 0.397 kept, 11 mm low) | Winter 2009 |
| lumbosacral joint | 9-11 cm above, 2-4 cm behind the hip axis (pelvic tilt about 12 degrees) | 92.75 mm above, 30 mm behind | sagittal pelvic parameters (own knowledge) |
| C7 | 0.855 H = 1.419 m | 1.419 barefoot, 2 cm behind the hip joints | de Leva 1996 |
| atlanto-occipital joint, vertex | vertex H; joint about 0.145 m below it | joint 1.515 barefoot, 32 mm ahead of C7 | ANSUR II head height (Gordon 2014) |
| head height, length, breadth (female) | 0.216 / 0.185 / 0.145 m | chin -0.071 to vertex +0.146 about the joint; 0.183 x 0.144 m skull | ANSUR II |
| shoulder joint | 0.818 H = 1.358 m (brief); joints 0.30 m apart | 1.358 barefoot, over the hip joints, z ±0.152 | Winter 2009, brief |
| upper arm / forearm / hand | 0.186 / 0.146 / 0.108 H | 0.3102 / 0.2444 / 0.1786 m (kept) | Winter 2009 |
| hip joint spacing | 0.16-0.18 m | 0.17 m (was 0: both thighs on the midline) | Bell 1990 |
| hip breadth, buttock depth | 0.36-0.37 m, 0.24 m | leggings 0.368 m, 0.25 m | ANSUR II female, + pregnancy fat |
| abdomen in week 36 | fundal height 36 cm (fundus at the xiphoid), girth 1.00-1.05 m at the umbilicus, depth back-front 0.33-0.35 m | apex 0.263 m ahead of the hip joints at 1.02 m shod, clothed depth 0.375 m, cover 0.36 x 0.34 m | obstetric standard (fundal height = weeks), Hytten 1980 (asset reference) |
| fetal unit | 6 kg; uterus from the lumbar spine to the abdominal wall | centre 0.15 m ahead of L3 (was 0.11) | asset; geometry of the modelled uterus |
| fetus at 36 weeks | crown-rump 0.33 m, BPD 90 mm, head circ. 0.32 m, cephalic, mostly left occiput anterior | skull 110 x 92 x 90 mm in the pelvic inlet, spine along the left front | obstetric standard (own knowledge) |
| bust in late pregnancy | bust point about 0.72 H, depth 0.26 m | apex 1.245 m shod, knit front 0.158 m ahead of the hip joints | ANSUR II, + pregnancy |
| thigh / calf / ankle girth | 0.58 / 0.35 / 0.21 m | 0.57 / 0.35 / 0.20 m | ANSUR II |
| hand length / breadth / thickness | 0.18 / 0.075 / 0.026 m | 0.18 / 0.076 / 0.023-0.033 m | ANSUR II |
| posture in the third trimester | trunk leans back 4-8 degrees, lordosis increased, pelvis tipped forward slightly, knees near straight | trunk 4.7 degrees back, pelvis 3 degrees forward, lumbar extension 9 degrees at L5/S1, knees 5 degrees | Okanishi 2012 (added to references) |
| skin area | DuBois 1.789 m² | unchanged (C18) | asset |

## 2. Audit findings (before)

Rig: the same template defects as the old standing adult: thighs without lateral offset (both hip joints on the midline, feet together), torso straight up from the hip joints (C7 at 1.393 m = 0.839 H, 26 mm low, no lumbosacral offset), neck 0.093 m and head 0.175 m with the head joint in the jaw (1.485 m) and the bone 1 cm above the vertex, a barefoot foot bone (66 degrees, 0.160 m) under a drawn heeled boot, belly bone 0.24 m pointing out from 3 cm above L5 with the fetal centre 0.11 m ahead (inside the trunk wall).
Poses: knees bent 20 degrees in all standing poses (shank -19.9); hand-on-back had the far forearm sticking straight ahead past the abdomen with the hand in the air; the abdomen of the solids hung below the hip line.
Dimensions: 1.65 x 0.69 x 0.508 m, with 0.69 m set by the floating hand; the body measures 1.70 x 0.52 x 0.60 m.
2D (all 12 drawings): constructed, not views of one body; different ids per view (`torso`/`sweater`, `belly`/`belly-cover`, `shoe`/`boot`, `trunk-l`, `waist`, `shoulders`), ellipse belly, capsule plan.
Solids (before; 1:50 72 / 62 / 38): a ball belly at thigh level, box torso, tube legs, the structure largely a copy of the body; kernel traps on boot and head lofts (`kernel-traps.md`).

## 3. Root fixes (old -> new)

| member | old | new |
|---|---|---|
| `pelvis.y` | 0.8686 | 0.89075 (boot lift) |
| `torso` y / angleDeg / lengthM / title | 0 / 0 / 0.4313 / "Torso with coat" | 0.03 / -1.2516 / 0.45781 / "Torso with sweater" |
| `torso.mass` comX / comY | 0.1993 / 0 | 0.1994 / -0.0257 (same point) |
| `neck` x / angleDeg / lengthM / comX | 0.4313 / -8 / 0.09275 / 0.0464 | 0.45781 / -17.2372 / 0.10091 / 0.0505 |
| `head` x / angleDeg / lengthM / comX / comY | 0.09275 / 8 / 0.1746 / 0.0728 / 0 | 0.10091 / 18.4888 / 0.145 / 0.042 / -0.0009 (same point) |
| `arm-*` x / y / angleDeg | 0.38956 / 0 / 180 | 0.39684 / -0.02134 / 181.2516 (joint over the hip joints at 0.818 H) |
| `thigh-*` z | none | ±0.085 |
| `foot-*` angleDeg / lengthM / comX / comY / title | 66.193 / 0.15979 / 0.0687 / 0 / "foot and shoe" | 90 / 0.128 / 0.056 / -0.035 / "foot in the ankle boot" (horizontal foot to above the ball; sole orientation of every pose and clip frame unchanged) |
| `toe-*` x / y / angleDeg / lengthM / comX / comY | 0.15979 / 0 / 23.807 / 0.0559 / 0.0257 / 0 | 0.128 / -0.08664 / 0 / 0.075 / 0.0338 / 0.022 (origin on the ground under the ball) |
| `belly` x / angleDeg / lengthM / comX | 0.03 / -90 / 0.24 / 0.11 | 0.045 / -88.7484 / 0.28 / 0.15 |
| anchors heel / ball | (-0.0094, -0.0746) / (0.1598, 0) | (-0.036, -0.0866), 0.062 x 0.058 / (0.128, -0.0866), 0.095 x 0.086 |
| anchors mouth / nostrils / crown | (-0.0049, -0.0777) / (0.0291, -0.1009) / (0.1697, 0) | (-0.0215, -0.098) / (-0.0005, -0.107) / (0.152, 0.012) |
| anchors belly-front / belly-cog / palms | 0.225 / 0.11 / 0.08 | (0.286, -0.012) / (0.15, -0.01) / 0.075 |
| standing legs (3 poses) | thigh 11.4, shank -19.9, foot 11.5 | knee 5 degrees, ankle 2-3 cm behind the hip: thigh 3.0-4.8, shank -5/-6, foot 4.0-5.2 |
| neck / head offsets | -7/4, -10/-2, -2/8, -6/3 | -2/-4, -2/-14 (bowed to the abdomen), 2/0, -2/-3 |
| arms (4 poses) | hand-picked angles | solved on the product chain against the modelled cloth: right palm flat on the lower back (2 mm), left palm under the abdomen (6 mm), cradling palms on the front of the abdomen (3 to 5 mm), both palms on the lumbar back (2 to 4 mm) |
| clip `waddle-walk` | arm keys of the old pose, advanceM 1.105 | the new pose plus each key's old oscillation (clamped to the joint limits), neck/head likewise; advanceM 1.1202 (planted travel, 0.86 m/s; description updated) |
| clip `rest-and-breathe` | leg overrides of the old stance | the new hand-on-back legs |
| `dimensions` heightM / widthM / depthM | 1.65 / 0.69 / 0.508 | 1.7 / 0.525 / 0.61 (vertex with hair on 25 mm heels; elbow behind to the left thumb ahead; right elbow to left sleeve) |
| `materials` | — | `bone` (skeleton) |
| `provenance` | — | references `gordon-2014`, `okanishi-2012`; round-3 note (en, de) |
| `structure.assumptions[0]`, pose and clip descriptions | fetal unit 0.11 m | 0.15 m, the centre of the uterus; texts rewritten |
| `representations[*].geometry` | 47/45/31, 29/31/22, 19/26/22, 15/18/20 shapes | 40/58/39, 22/25/23, 21/21/21, 20/20/20 (redrawn, below) |
| `representations[*].decisions` | silhouette, retained, merged, omitted, thinElements, overlays of the constructed drawings | rewritten for the views of the bodies (hatching and styles kept) |

2D redraw (E1, `r3-pregnant-person-draw2d.ts`): every filled shape is the orthographic outline of its solids at that scale in the default pose, solved into its bone frame through the product's placement (residual 0.000 mm), simplified at 0.8 / 1.6 / 3.2 / 6.5 mm. One id per part in all three views, the old elevation ids (`sweater`, `belly-cover`, `hips`, `thigh-*`, `shank-*`, `boot-*`, `boot-toe-*`, `boot-sole-*`, `boot-toe-sole-*`, `sleeve-*`, `forearm-*`, `cuff-rib-*`, `hand-*`, `neck`, `head`, `hair`, `ear-*`, `sweater-hem-rib`, `sweater-collar`, `eye-*`, `brow-*`, `nose`, `lips`, `mouth`, `boot-zip-*`), with the old tones and the knit hatch. Gone: `torso`, `belly`, `trunk-*`, `waist`, `shoulders`, `shoe-*`, `upper-arm-*`, `shoulder-*`, `elbow-*`, `fist-*`, `crotch`, `knee-*` (in the shank), `finger-*`, `hair-strand-*`, `belly-stretch-*`, `linea-fold`, `legging-seam-*`, `shoulder-seam-*`, `hair-parting`, `face`, `sil-*` at 1:500. Arms, forearms and hands are solids in their bone frames (sleeves revolved about the bone, hands extruded across their thickness), so every pose and the walk follow; filled parts inside their bone's outline also carry their outline as a detail line.

## 4. What was modelled (solids body / thermal / structure)

- **1:50 — 83 / 97 / 164.** Body: oversized knit sweater as one loft of 16 measured slices whose skirt hangs from the pelvis (its slices are carried by the pelvis of the default pose, the lowest tilted 12 degrees so the hem rides up under the abdomen), rib hem, crew-neck rib; the stretched knit over the abdomen (9 cross slices, open where it enters the sweater); leggings seat (top cut `+u`), thighs with trochanter and quadriceps, kneecaps, shanks with calf and ankle; sleeves with dropped seam, forearm sleeves with three soft folds, rib cuffs; ankle boots of block heel, waist sole, toe sole, shaft (horizontal slices), vamp, toe box, rim, inner zip (slender) and pull (texture); head of 12 slices with chin, lips, nose, eyes, brows, ears; neck on the neck bone; shoulder-length bob tucked behind the ears; hands with palm, thenar and hypothenar pads, four three-phalanx fingers, three-phalanx thumb, curled flat (right) and cupped (left). Thermal: face ring and face parts and neck, hair, hands, knit layer by reference, T-shirt 13 mm inside the knit (trunk and short sleeves) and 5.6 mm inside the abdomen knit, briefs 1 mm inside the seat, socks 4 mm inside shaft, vamp and toe box; outlets under the nose, on the lips and on the hair crown. Structure: see the 1:50 decision; 18 joints, masses of pelvis, torso, head, thighs, shanks and the fetal unit.
- **1:100 — 25 / 28 / 68.** New, fewer sections; hem and collar into the sweater, cuffs into the sleeves, heel and sole into one boot loft, mitten hands with thumbs, head with nose and ears. Skeleton with six rib hoops, the fetus as skull, spine, trunk and legs.
- **1:200 — 20 / 23 / 54.** One mass per part, mittens without thumbs, no face parts; skeleton of bars and three hoops, fetal skull and body.
- **1:500 — 20 / 23 / 36.** Fewest masses; hands are 76 mm mittens and the skeleton 75 mm bars (size floor); the fetal unit as one mass.
- Decisions: three bilingual sentences per scale (`r3-pregnant-person-texts.json`).

Final check: `volumes-check.ts --product` silhouette IoU elevation / front / plan at 1:50 0.982 / 0.989 / 0.972 (p95 13 / 6 / 9 mm); all bounds, cross-scale and monotonic lines ok. Areas at 1:50 (measured / declared): face-skin 0.140 / 0.125, hand-skin 0.078 / 0.0895, sweater 0.978 / 0.823, belly-sweater 0.183 / 0.190, leggings 0.770 / 0.53 (+45 %), ankle-boots 0.282 / 0.143 (+97 %); 1:100 leggings +43 %, everything else in band; 1:200 and 1:500 totals pass.

## 5. What I saw in the final renders

- `r3-pregnant-person-sheet` 1:50 body, hand-on-back, six views (with and without rig): reads at once as a heavily pregnant woman: the abdomen is a full round dome from under the bust to the pubis, lower half fuller, apex at the navel; the sweater hem rides up at the front and hangs over the seat at the back; lumbar hollow and backward lean in both side views; right hand flat on the lower back with the elbow out and back (back view), left hand cupped under the abdomen (front, left side, three-quarter); feet at hip width; boots with heel, shaft and toe. Rig joints sit at hip, knee, ankle, shoulder, elbow, wrist, C7 and skull base.
- Head close-up (front, both sides, back, three-quarter, after the lip and chin fix): female face with brows, eyes, nose, lips, chin; ears visible in front of the tucked hair; bob with fringe line. Seen before the last lip/chin edit only in the close-up; the full sheet after it.
- 1:50 structure, six views: skeleton inside the body ghost everywhere, rib cage, pelvis, femur necks, the fetus head-down in the pelvic inlet with its spine on the left front, masses visible.
- 1:50 thermal cut, six views: T-shirt inside the sweater, tee under the abdomen knit, briefs in the seat, socks in the boots, face ring red under the green hair.
- 1:100 and 1:500 body sheets: the same woman simplified; 1:500 back showed the seat poking through the sweater between two skirt slices, fixed by a second skirt slice (re-rendered only through the check, not re-opened).
- `cradling-belly` 1:50: both palms on the front of the abdomen, right higher, head bowed; reads right. `waddle-walk` t 0.4 s: stride with heeled boots, hands keep their places (left hand lower, at the pubis, in this frame).
- 1:500 structure: thick stick skeleton, fetal unit as one ellipsoid ahead of the spine.
- `volumes-render` 1:50 overlay: green everywhere except thin red seams at the arm and hand edges.
- `r3-2d` candidate 1:50 (light-solid and light-line): the three drawings are views of the same woman (hatched sweater, abdomen, hands in place); the redraw went through three rounds (arm prisms first, then plain projections that the validator refused on twisted bones, then revolved sleeves and w-swept hand prisms with the rest-lateral offset). Final catalog `r3-2d` 1:50 (opened): side, front and plan are views of one woman (lean-back, lumbar hollow, abdomen dome, right elbow back and out, hatched knit, hem rib, collar, face lines, boots with heels); the left hand under the abdomen reads in the side view only as a small tilted block (its prism seen edge-on). 1:500 `r3-2d` rendered, not opened.
- Rendered, not opened: 1:100 thermal sheet, 1:200 sheets, `stretching-back`, the 1:100/1:200/1:500 overlay sheets.

## 6. Still imperfect

- Grade as I see it: body 1:50 B+ (abdomen, posture, boots, hands A-; sweater and legs B: superellipse lofts, the knit has folds only above the cuffs, no drape lines from the bust onto the abdomen; knee is two overlapping lofts with a ragged seam line in the back view and a visible joint in deep walk flexion).
- Arm joint angles: to put the palm flat on the lower back the solver uses the wrist at its 60 degree limit and a forearm twist of -170 degrees (pronation beyond the anatomical ~90 degrees, compensated by the upper-arm twist); the left hand is at -60 degrees. The hands sit right, the angle split is not anatomical. A wider wrist limit or a shoulder internal-rotation term would be the clean fix.
- The thighs have no spread or turn-out: a wider, turned-out stance (typical late in pregnancy) needs solids for every leg and foot shape in 2D, and w-swept prisms on twisted feet drew wrongly; kept at hip width.
- `leggings` and `ankle-boots` areas exceed the C18 band (filed); not shrunk.
- Hand and sole contact numbers, boot dimensions and pregnancy body measures are from my knowledge, not a web lookup.
- Tool finding (`tool-requests.md`): `extruded` sweep `w` takes z0/z1 as rest-world lateral positions; the hand prisms here are written that way.
- Not run: the staffage test suites; fixtures that embed this asset may need regenerating after the rig, dimension and 2D change.
