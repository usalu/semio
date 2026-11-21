# Round 3 — humans/person-with-white-cane

Scripts (all in the ticket folder): `r3-person-with-white-cane-roots.ts` (rig, anchors, poses, clips), `-kit.ts`, `-s50.ts` (1:50 body, thermal, structure), `-coarse.ts` (1:100, 1:200, 1:500), `-draw2d.ts` (2D redraw, ruling E1), `-texts.json` (decisions, dimensions, materials, assumptions), `-apply.py` (candidate and catalog write), `-build.ts`, `-check.ts`, `-suit.ts`, `-extent.ts`, `-run.sh` (the whole ship chain; `--commit` writes the catalog and splices).

State: spliced on 2026-11-20. `r3-validate.ts humans/person-with-white-cane` and `--catalog-only`: `VALID ... schema 0, validateAsset 0`. `volumes-check.ts humans/person-with-white-cane --product`: every rule green at every scale except one measurement, `areaDeviation:boots` 0.395 at 1:50 (see "Still imperfect").

The work was done in two sessions. The first (2026-11-20, 02:20 to 03:18) made the reference, the root fixes, all four scales and the 2D redraw and spliced them, but left no report; it was interrupted. This session re-ran the ship chain (byte-identical result, catalog untouched by the bulk passes of 03:40 to 04:10, all four scales already present), looked at every list at every scale again, fixed four defects (section 5) and wrote this report. The reference and audit tables below are reconstructed from the scripts and from the difference between the untouched copy `🗑️generated/r3-person-with-white-cane/original.json` and the catalog; I did not see the first session's own notes.

## 1. Reference (blind woman, stature 1.68 m barefoot, 62 kg; 1.70 m in ankle boots, 1.71 m with the hair)

| Quantity | Real value | Model | Source |
| --- | --- | --- | --- |
| stature, shod | 1.68 m + 20 to 25 mm heel | top of skull 1.701 m, hair 1.711 m | Drillis and Contini 1966; Winter 2009 |
| hip joint height | 0.53 H = 0.890 m + 20 mm sole | 0.909 m in the default stance | Winter 2009 |
| knee, ankle | 0.285 H = 0.479 m, 0.039 H = 0.066 m, + sole | knee 0.500 m, ankle 0.087 m (thigh 0.4116 m, shank 0.412 m) | Winter 2009 |
| shoulder joint; elbow; wrist | 0.818 H = 1.374 m; 0.63 H = 1.058 m; 0.485 H = 0.815 m, + sole | 1.391 m; 1.081 m; 0.843 m (left arm hanging) | Winter 2009 |
| upper arm, forearm, hand | 0.186 H, 0.146 H, 0.108 H | 0.3125, 0.2453, 0.1814 m | Winter 2009; de Leva 1996 |
| shoulder spacing (joint centres) | biacromial 0.245 H = 0.41 m, joint centres about 0.33 m | 0.33 m (`z` = 0.165) | Drillis and Contini 1966 |
| hip joint spacing | about 0.17 m in women | 0.17 m (`z` = 0.085) | Winter 2009 |
| head height | H / 7.5 = 0.224 m | chin 1.478 m to vertex 1.701 m | Drillis and Contini 1966 |
| foot, boot | foot 0.152 H = 0.255 m; Chelsea boot EU 39 about 0.27 m long, 0.095 to 0.10 m sole, 25 mm heel | 0.27 m, 0.098 m, 25 mm heel, 10 mm sole, shaft to 0.17 m | Winter 2009; shoe size tables (from memory) |
| long cane | sternum to chin height, 1.25 to 1.35 m for this stature; shaft 12 mm, folding in four or five sections, grip 0.2 m, roller tip 25 to 40 mm | 1.30 m overall, 12 mm shaft, three section joints, 0.2 m grip with wrist loop, roller tip | Jacobson 2013 (orientation and mobility); manufacturer data from memory |
| cane technique | hand at the midline a forearm ahead of the belt, index finger along the grip, tip one step ahead, arc a little wider than the shoulders | default pose and both clips | Jacobson 2013; Perry and Burnfield 2010 for the gait |
| body mass, segment masses | 62.24 kg with clothing and cane | unchanged | de Leva 1996 |

No web lookup was made in this session; product figures are from memory and are marked so.

## 2. Audit (state before round 3)

- `dimensions` 1.69 / 1.074 / 0.40 m (height / width ahead / lateral): no heel, the cane reach too short for a 1.3 m cane held ahead, and the bag and the hanging arm not inside the lateral 0.40 m.
- Rig: pelvis joint 0.9024 m with boots drawn under a barefoot skeleton; torso 0.4284 m (shoulders 4 cm low); head bone 0.198 m from a neck of 0.079 m (head joint in the wrong place); shoulders 0.30 m apart; both hip joints on the midline (no `z`); foot bone 0.158 m; the cane bone started 0.082 m along the hand with no lateral offset, so it ran through the palm instead of lying in the fist.
- Poses and clips: a stance with bent knees, the cane hand off the midline; the walk without a rolling tip.
- 2D: 69 / 59 / 39 shapes at 1:50 constructed per view, not views of one body.
- Solids: 1:50 91 / 136 / 39, structure a copy of the body lofts; coarser scales the finer lists thinned out.

## 3. Root fixes (old -> new)

| Member | Old | New |
| --- | --- | --- |
| `dimensions` heightM / widthM / depthM | 1.69 / 1.074 / 0.40 | 1.71 / 1.26 / 0.56 (manifest row updated too) |
| `pelvis.y` | 0.9024 | 0.9114 |
| `torso.lengthM`, `neck.x` | 0.4284 | 0.4706 |
| `neck.lengthM`, `head.x` | 0.079 | 0.085 |
| `head.lengthM` | 0.1982 | 0.15 |
| `arm-l.z`, `arm-r.z` | 0.15, -0.15 | 0.165, -0.165 |
| `thigh-l.z`, `thigh-r.z` | none | 0.085, -0.085 |
| `foot-*.lengthM`, `toe-*.x` | 0.1581 | 0.1627 |
| `foot-*.angleDeg` / `toe-*.angleDeg` | 60.63 / 29.37 | 57.88 / 32.12 |
| `cane` x, z, angleDeg, lengthM | 0.0816, none, 0, 1.3 | 0.05, 0.024, 10, 1.222 (the bone starts in the fist; the cane is still 1.30 m overall) |
| anchors (10), poses (4), clips (2), `summary` | | all solved anew on the new rig, same ids |
| materials | | `bone` added |
| 2D geometry | 1:50 69 / 59 / 39 shapes | 39 / 50 / 32; 1:100 32 / 33 / 29; 1:200 26 / 27 / 25; 1:500 22 / 23 / 21, every filled shape the projected outline of its solids |

## 4. What is modelled (body / thermal / structure solids)

- **1:50 — 96 / 127 / 142.** Body: head with nose, lips, chin, chin-length bob with fringe, sunglasses with front and temples; neck, shirt front, collar; buttoned denim jacket as one garment with chest pockets and six buttons; sleeves with sleeve heads and cuffs; both hands with thenar, thumb and four two-part fingers, the right closed round the grip with the index finger along it; jeans seat, thighs, shanks; Chelsea boots with shaft, quarters, toe box, heel and sole; shoulder bag with flap, clasp and strap across the body; cane with grip, wrist loop, shaft, three joint bands and roller tip. Thermal: every garment in its outer layer with the shells of its layers inside (shirt and air under the jacket, air under the jeans, sock and air in the boots), face skin below the hairline, neck, hands, breath outlets, plume outlet, bag, glasses and cane by reference. Structure: skull and jaw, curved spine, ribcage of hoops with sternum, clavicles and scapulae, pelvis with ilia, long bones with joint ends, hand and foot bones, the cane tube, joints at the rig joints, supports at feet and cane tip, masses at the segment centres.
- **1:100 — 37 / 40 / 77**, **1:200 — 26 / 29 / 45**, **1:500 — 23 / 25 / 31.** Authored anew per scale: fewer sections, mitten hands (a fist at 1:500), glasses as one band, boots merged with their soles, the cane in three parts and then two; the skeleton reduced to five rib hoops (1:100), then to spine, girdles and limb bars; at 1:500 the smallest feature of 75 mm makes the members as thick as bars.
- `decisions` per scale are bilingual sentences from `-texts.json`.

Silhouette IoU elevation / front / plan after the final splice: 1:50 0.987 / 0.981 / 0.939; 1:100 0.991 / 0.978 / 0.940; 1:200 0.990 / 0.977 / 0.966; 1:500 0.976 / 0.976 / 0.936.

## 5. Defects found and fixed in this session

1. **Thermal 1:100 and 1:200: the jeans were in three floating pieces.** The seat ended up to 6 cm below the jacket hem and the thigh tubes began up to 10 cm below the seat, because the cut picked the nearest authored section. `between()` in `-s50.ts` now interpolates a section exactly at the hem and at the crotch line; the same code serves 1:50.
2. **Thermal 1:200: head floating above the jacket.** The neck skin was only added at 1:100; it is now in the 1:200 list as well (28 -> 29 solids).
3. **2D front and plan, all scales: the left hand was drawn on the left thigh**, 0.16 m inwards of the cuff. Its extruded span was given relative to the arm, the product reads it from the midline. It is now `ARM_Z` + span, tapering 21 mm inwards to the finger tips, and sits at the end of the sleeve (front extent 0.216 to 0.281 m; the lateral 0.56 m is kept).
4. **Boot shaft opening counted as boot surface.** The top face of each shaft, where the leg goes in, is now `sectionFaces: ["-u"]` (0.235 -> 0.226 m²).

## 6. What I saw in the final renders

- `r3-sheet` 1:50 body, six views with rig, plus front and three-quarter alone: reads at once as a woman with a long cane. Joints lie at shoulder, elbow, wrist, hip, knee and ankle of the figure; upright stance, weight on both feet; the right hand at the midline with the cane running straight ahead to a tip 1.05 m in front; bag at the right hip with the strap over the left shoulder; hands with separate fingers; boots with heel and sole. Walk clip at 0.3 s: stride closed at hips, knees and ankles, tip on the ground.
- 1:100, 1:200, 1:500 body sheets: same figure and proportions, progressively plainer; still recognisable at 1:500 by cane, bag and bob.
- Thermal: 1:50 cut sheet shows the nested shells inside jacket, jeans and boots and nothing poking out; 1:100, 1:200 (re-rendered after the fixes) and 1:500 are closed figures in region colours.
- Structure: 1:50 and 1:100 show a real skeleton inside the pale body, 1:500 a bar figure with the cane and its support.
- 2D (`r3-2d.ts`): 1:50 default pose, 1:100 walk at 0.3 s, 1:500 `cane-parked`: the three views agree with the body.

Not looked at in this session: thermal and structure sheets of 1:200 structure and 1:500 from single directions, and the poses `diagonal-technique` and `walking-midstance`.

## 7. Still imperfect

- **`volumes-thermal-coverage` is red at 1:50 for `boots`**: 0.226 m² modelled against the declared 0.1619 m² (deviation 0.395, limit 0.30). The declared value is a skin share; a real pair of ankle boots with soles has 0.20 to 0.23 m². C27 reserves the change for the area reviewer; filed in `author-requests.md` ("r3 humans/person-with-white-cane").
- **The left hand in the 2D front view is a plain tapering slab**, because the product generates the front of an extruded solid as its span; the side elevation and the plan show the hand outline.
- **The bob reads as a helmet from the front** at 1:50 (straight sides, flat fringe); the hair has no parting or strands.
- **Seams at elbow and knee** show as small steps in the outline of the 1:50 body (left elbow in the front view, both knees).
- **1:500 structure** is a bar figure whose members are as thick as the limbs; the 75 mm feature leaves no room for bones.

State: not DONE — one product check measurement red (`boots` area share, with the reviewer) and the four visual points above.

## Round 5 handcraft (2026-11-21)

Scripts: `r5-person-with-white-cane-*.ts|py|sh` (the round-3 chain carried over under the r5 prefix and remodelled; `-run.sh` builds roots, volumes, 2D and the candidate and, with `--commit`, writes the catalog and splices), `r5-person-measure.ts` (model against the anthropometric table, every scale), `r5-person-sheets.sh` (every sheet the brief asks for), `r5-person-ship.sh` (commit, measure, check, validate). Output under `🗑️generated/r5-person-with-white-cane/` (`measure.md`, `check.log`, `run.log`).

### Reference (woman, 1.68 m barefoot, 62 kg; 22 mm boot sole and heel)

| quantity | reference | source |
| --- | --- | --- |
| stature shod | H + 22 mm = 1.702 m | DIN 33402-2 women 50th percentile 1.65-1.68 m |
| hip / knee / ankle joint | 0.53 / 0.285 / 0.039 H (+ sole) | Winter 2009 table 4.1; Drillis and Contini 1966 |
| shoulder joint / elbow / wrist | 0.818 / 0.63 / 0.485 H (+ sole) | Winter 2009 |
| thigh / shank / upper arm / forearm / hand | 0.245 / 0.246 / 0.186 / 0.146 / 0.108 H | Winter 2009; de Leva 1996 |
| head chin-vertex | H / 7.5 | brief; Neufert |
| biacromial breadth | 0.23 H | brief; ISO 7250-1 |
| bideltoid breadth | 0.265 H skin, + 10 mm denim each side | ANSUR II female (453 mm at 1.629 m stature, ratio 0.278, civilian lower) |
| hip breadth standing | 0.215 H, + 5 mm denim each side | DIN 33402-2 women |
| chest (bust) depth | 0.15 H, + jacket | DIN 33402-2 women |
| thigh breadth mid-thigh | 0.092 H + 10 mm jeans | ISO 7250 thigh circumference / pi |
| calf depth | 0.064 H + 8 mm | ISO 7250 calf circumference / pi |
| foot / boot length | foot 0.152 H, boot + 18 mm (toe allowance, welt) | Winter 2009; EU 39 Chelsea boot about 0.27 m |
| boot width / shaft width | 0.055 H + 8 mm / about 80 mm | shoe size tables (EU 39) |
| relaxed hanging hand reach | 0.95 x 0.108 H (fingers flexed 10 to 30 degrees) | Napier, Hands; Winter 2009 |
| long cane | sternum to chin height, 1.30 m; 12 mm shaft, 25 mm grip, roller tip | Jacobson 2013 (as round 3) |

### Model against reference (default pose `constant-contact-stance`, 1:50; `r5-person-measure.ts`)

| quantity | model m | reference m | deviation |
| --- | --- | --- | --- |
| stature shod (top of hair) | 1.711 | 1.702 | 0.5 % |
| hip / knee / ankle joint height | 0.909 / 0.500 / 0.087 | 0.912 / 0.501 / 0.088 | -0.4 / -0.2 / -1.1 % |
| shoulder / elbow / wrist height | 1.391 / 1.081 / 0.843 | 1.396 / 1.080 / 0.837 | -0.3 / 0.0 / 0.7 % |
| thigh / shank / upper arm / forearm | 0.412 / 0.413 / 0.313 / 0.245 | 0.412 / 0.413 / 0.312 / 0.245 | 0.0 % each |
| head chin-vertex | 0.223 | 0.224 | -0.5 % |
| hand of the rig / relaxed hand reach | 0.181 / 0.176 | 0.181 / 0.172 | 0.0 / 2.3 % |
| bideltoid (jacket) / biacromial | 0.455 / 0.380 | 0.465 / 0.386 | -2.1 / -1.7 % |
| hip breadth (jeans) / chest depth (jacket) | 0.371 / 0.266 | 0.381 / 0.272 | -2.6 / -2.0 % |
| boot length / width | 0.270 / 0.098 | 0.273 / 0.100 | -1.4 / -2.4 % |
| thigh breadth / calf depth / boot shaft | 0.164 / 0.117 / 0.080 | 0.165 / 0.116 / 0.080 | -0.2 / 1.5 / 0.1 % |
| neck breadth | 0.107 | 0.109 | -1.9 % |

Every 1:50 value is within 3 %. Coarser scales (full tables in `measure.md`): joint heights and bone lengths identical (same rig); widths within 3.6 % at 1:100 and 1:200; at 1:500 hand reach -6.7 % (the 75 mm hand mass is kept short so the lateral extent stays inside `dimensions`), boot shaft +6 % (the 75 mm floor); every 1:500 deviation is below one smallest feature (75 mm).

### Grades (my own look, strict)

| scale | body before -> after | thermal before -> after | structure before -> after |
| --- | --- | --- | --- |
| 1:50 | B (helmet bob, slab pocket boxes, steps at knees and elbows, clawed hanging hand, bust and hips 4-6 % small) -> A | A- -> A | A- -> A |
| 1:100 | B+ -> A | A- -> A | A- -> A- |
| 1:200 | B+ -> A- | A- -> A- | B+ -> B+ |
| 1:500 | B (goggle-like glasses band standing off the face, beret-like hair) -> A- | B+ -> A- | B (bar figure, forced by the 75 mm floor) -> B |

### Changes

- Bob rebuilt (all scales): fullest at the cheeks (half-width 0.089 m), turned in at the ends, rounder crown (exponent 2.0 instead of 2.3), fringe over the forehead and the face open from the temples down; no longer a helmet from the front.
- Jacket: pointed trucker flap pockets (free loft that follows the curve of the chest, 9 mm thick, point 21 mm below the straight edge) with a press stud each, replacing the slab boxes; a separate waistband 4.5 mm proud; bust 8 mm deeper at all scales (chest depth -5 % -> -2 %).
- Jeans: hips 8 mm wider at the buttocks, thighs 4 to 6 mm wider (hip -4.5 % -> -2.6 %, thigh -6 % -> 0 %), calf 5 mm shallower (+8.9 % -> +1.5 %); 1:100 to 1:500 carried along (extra hip and thigh stations).
- Knee and elbow: thigh and sleeve end 14 mm beyond the joint with a short dome, shank and forearm start 20 to 26 mm above it slightly slimmer, so the straight outline runs on without the ring step and the bent knee of the walk shows no shelf (looked at in `constant-contact-walk` 0.44 s).
- Hanging left hand: fingers relaxed (10/28/46 degrees instead of 22/50/74) and drawn together, length 0.88 of the round-3 phalanges (reach +5 % -> +2 %). Coarse hands lengthened to the same reach.
- 1:200 and 1:500: chin dome added (head height -4 % -> -1.8 %); 1:500 glasses band moved onto the face surface (it stood 6 to 10 mm off the face and wider than the head).
- 2D: redrawn from the new bodies by `r5-person-with-white-cane-draw2d.ts` at every scale; the waistband line follows the new band; the 1:500 cane strip is 75 mm wide and the smallest 1:500 parts (fist, glasses, fringe, shanks, boots) and the 1:200 neck carry a 0.16 to 0.17 mm pen, which fixes the print refusal the catalog had (`PRINT range-end-feature` x5 at 1:1000, introduced by the new print check).
- Texts: 1:50 body decision and 1:500 silhouette decision rewritten (en, de).

### Checks

`r3-validate.ts humans/person-with-white-cane`: `VALID ... schema 0, validateAsset 0` (catalog and candidate). `volumes-check.ts --product`: every rule green at every scale except `volumes-thermal-coverage` 1:50 `boots` 0.216 m² against 0.1619 (33 %, C18/C27 segment share, listed in `area-review.md`); `r3-validate --volumes` reports the same single line.

### Looked at (opened)

Body sheets 1:50, 1:100, 1:200, 1:500 (six panels), 1:50 front and three-quarter close-ups, 1:500 three-quarter close-up, structure 1:50, thermal cut 1:50, 2D 1:50 (light-line, light-solid), 1:50 clip `constant-contact-walk` 0.44 s (sheet and left view). Rendered with `r5-person-sheets.sh` and not all opened one by one: structure and thermal of 1:100 to 1:500, 2D of 1:100 to 1:500 and the poses `diagonal-technique`, `walking-midstance`, `cane-parked`, clip `listen-and-scan`.

### Open points

- `boots` area share (C27, area reviewer).
- The hanging left hand is still a tapered slab in every front drawing: an `extruded` shape can only be a band in front, a flat shape is refused as `lateral-ambiguous`, a scaled `revolved` turns the plan into a disc (tool request 2026-11-21 in `tool-requests.md`).
- 1:500 structure stays a bar figure: members cannot be thinner than 75 mm there.

The grip and reflective band have individually set 0.20 mm printed outlines at 1:200. Both are readable at the 1:350 endpoint. The official document and range checks passed.
