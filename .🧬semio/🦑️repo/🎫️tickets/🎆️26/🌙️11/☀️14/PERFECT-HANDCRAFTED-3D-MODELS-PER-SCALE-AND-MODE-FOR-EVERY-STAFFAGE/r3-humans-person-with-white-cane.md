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
