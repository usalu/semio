# Round 3 — `humans/person-with-umbrella`

Author scripts (ticket folder): `r3-person-with-umbrella-roots.ts` (rig, anchors, poses, clips, summary), `-kit.ts` (section constructors), `-s50.ts` (every section of the 1:50 body, thermal shells and skeleton), `-coarse.ts` (1:100, 1:200, 1:500, each stated anew), `-build.ts` (writes `volumes/humans/person-with-umbrella.json`), `-draw2d.ts` (2D redraw, ruling E1), `-texts.json` + `-apply.py` (splice of roots, 2D geometry, decisions, dimensions, material, reference, canopy area), `-look.ts` (multi-direction renders on the candidate before the catalog had the new rig), `-extent.ts`, `-probe.ts`.

Takeover: the first author was cut off after writing roots, kit and the 1:50 script; nothing was built, validated or written to the catalog. I ran the roots, fixed a crash in the kit (`mirrored` on `{ of }` references), kept the rig design and most of the 1:50 model after looking at it, and did everything else: 1:50 fixes, all coarser scales, the whole 2D redraw, texts, splice, checks, renders, this report.

State: catalog written and spliced. `r3-validate.ts humans/person-with-umbrella` and `--catalog-only`: `VALID ... schema 0, validateAsset 0`. `volumes-check.ts --product`: every rule green at every scale except `volumes-thermal-coverage` at 1:50 and 1:100 for `head-skin` and `shoes` (C18 values, see 6).

## 1. Reference (adult of 1.70 m barefoot, 1.72 m in shoes, 70 kg; stick umbrella)

| quantity | reference | model (default pose) | source |
|---|---|---|---|
| hip joint | 0.52 H = 0.884 m | 0.905 m above the sole (0.925 in shoes) | Winter 2009, table 4.1 |
| knee / ankle | 0.285 H = 0.485 / 0.039 H = 0.066 m | 0.484 / 0.066 m above the sole | Winter 2009 |
| shoulder / elbow / wrist | 0.818 / 0.63 / 0.485 H = 1.391 / 1.071 / 0.825 m | 1.391 / 1.075 / 0.831 m | Winter 2009 |
| upper arm / forearm / hand | 0.186 / 0.146 / 0.108 H = 0.316 / 0.248 / 0.184 m | same | Winter 2009 |
| shank | 0.246 H = 0.418 m | 0.4185 m | Winter 2009 |
| shoulder joint spacing | biacromial 0.245 H = 0.417 m minus 2 x 3.5 cm deltoid | 0.34 m, elbow 35 mm, wrist 20 mm outwards (carrying angle) | brief; ANSUR II |
| hip joint spacing | 0.17-0.19 m | 0.176 m | Bell et al. 1990 |
| head chin-vertex / length / breadth | H/7.5 = 0.227 / 0.196 / 0.151 m | 0.227 / 0.196 / 0.151 m | ANSUR II |
| shoe | EU 42: 0.285 x 0.104 m, heel 22 mm | 0.29 x 0.105 m, heel 22 mm | size tables |
| raincoat | knee length, belt at the waist, A-line skirt | hem 0.535 m, belt 0.99-1.045 m | garment practice |
| stick umbrella | 8 ribs ~0.56-0.60 m, open span ~1.0-1.05 m, stretchers pinned at ~45 % of the rib, runner on the shaft, crook handle r 35 mm, shaft 10 mm steel, ribs open ~60-70 degrees from the shaft | 8 ribs 0.56 m chord bowed 28 mm, span 1.02-1.04 m, stretcher 0.25 m pinned at 0.25 m, runner travel 0.16 m, open 66 degrees, crook r 35 mm, shaft 10 mm | product data of stick umbrellas (Fox, Swaine Adeney Brigg style), first author |
| grip | hand a hand's breadth ahead of the chest, elbow ~120 degrees | grip 0.235 m ahead, 0.185 m below the shoulder, elbow 126 degrees | first author's hold solver |
| masses | de Leva 1996 | unchanged | asset |

## 2. Audit findings (catalog before this round)

- Rig: shoulder joints at 0.4202 along the torso and z ±0.230 (shoulders 0.46 m apart, 0.12 m too wide), upper arm 0.303, forearm 0.264, hand 0.176, hip joints z ±0.110 (0.22 m), shank 0.4105, foot sole offset 0.078, head 0.264 long. The umbrella had two ribs (`rib-front`, `rib-back`), no runner, no stretchers: a canopy cannot open or furl as a linkage.
- Poses: default pose had both arms bent 94 degrees with the wrists cocked 56 degrees (both hands at the umbrella, the left one floating); grip at the hand origin + 73 mm on the hand axis, not in the palm.
- Dimensions: height 2.09 m, width and depth 1.05 m (old canopy).
- 2D: all 12 drawings were old staircase fit polygons (`fit-*` shapes with 1.9 mm steps), a coat with a hood the 3D never had, canopy as two lens paths with a revolve.
- Solids: canopy one revolve (kernel trap: self-intersection and implied closure at every scale), handle as 10 capsules, 1:100-1:500 capsule figures, structure copies of body lofts.

## 3. Root fixes (old -> new)

| member | old | new |
|---|---|---|
| `upper-arm-l/r` x, z | 0.4202, ±0.2299 | 0.3886, ±0.17 |
| `upper-arm` / `forearm` / `hand` length | 0.303 / 0.2639 / 0.1759 | 0.316 / 0.248 / 0.184 (mass comX scaled) |
| `forearm` z / `hand` z | 0 / 0 | ±0.035 / ±0.02 |
| `thigh` z / `shank` length / `toe` y / `foot` comY | ±0.1099 / 0.4105 / -0.0782 / -0.0391 | ±0.088 / 0.4185 / -0.086 / -0.043 |
| `head` length | 0.2639 | 0.2482 |
| `umbrella` x, z | 0.0733, 0 | 0.075, 0.026 (handle in the palm) |
| umbrella bones | `rib-front`, `rib-back` | 8 ribs (`rib-front`, `-front-l`, `-l`, `-back-l`, `-back`, `-back-r`, `-r`, `-front-r`), `runner` (prismatic), 8 `stretcher-*` |
| poses (5) | arm offsets above, two rib angles | right arm re-solved per pose (residual < 1e-4), left arm hangs plumb, ribs/stretchers/runner as a closed linkage (open 66, furled 5 degrees) |
| clips | 2 rib angles | `close-umbrella` keys with ribs at 36 degrees |
| anchors | contact y -0.0782, heel x -0.0293, mouth, nostrils, crown | -0.086, -0.035, (0.063, -0.1), (0.083, -0.108), (0.2482, -0.01) |
| `dimensions` | 2.09 / 1.05 / 1.05 | 2.07 / 1.03 / 1.03 |
| `thermal.regions[canopy].areaM2` | 2.1 | 1.75 (C16, factor 1.2: 8 gores, rib chord 0.555 m, hem width 0.392 m: 0.87 m² per face, both faces 1.74 m² + hem) |
| `materials` | 8 | + `bone` |
| `provenance.references` | 14 | + `winter-2009` |
| `summary` | two-rib wording | eight ribs, stretchers, runner |
| `representations[*].geometry` | 101+59+29 ... shapes | all redrawn: 1:50 30/39/28, 1:100 30/32/28, 1:200 29/30/28, 1:500 23/23/22 shapes |
| `representations[*].decisions` | old texts | rewritten (8 keys per scale, en and de) |

2D redraw (`-draw2d.ts`): every filled shape is the outline of its body solids in the default pose, traced and simplified (0.9 / 1.8 / 3.5 / 7 mm) and solved into its bone frame with the product's own placement (no residual or edge-on warning). Layer order stated by hand per view. Canopy: front half on `rib-front`, rear half on `rib-back` (the only ribs that stay in the median plane; drawing gores on the spread ribs triggers `lateral-ambiguous`); in the front view both halves are `solid: revolved` about the shaft from their elevation profile, so they narrow as the umbrella furls; the plan keeps the true octagon. Shaft as a strip (10 / 12 / 24 / 60 mm). 1:50 front strokes: placket lines and mouth.

## 4. What was modelled (body / thermal / structure solids)

- **1:50 — 115 / 138 / 135.** Head loft with jaw, cheeks, brow; nose, lips, ears, eyes, brows; hair swept back and a low bun; neck in a shirt collar; raincoat upper and A-line skirt with collar, lapels, back yoke (narrowed this round: it stood out as a box at the shoulders), placket, 4 buttons, welt pockets, 4 hanging folds, and (new) a belt with buckle that also closes the jagged seam between upper and skirt; sleeves with the elbow carried outwards, cuff straps, open shirt cuffs; hanging left hand with thumb and curled fingers, right fist with four fingers and thumb round the crook; trouser legs, derby shoes with upper, toe, sole, heel. Umbrella: crook handle, collar, shaft, runner, notch, ferrule, 8 bowed ribs and tips, 8 stretchers, 16 half panels (7.6 mm thick by the feature rule, real cloth 0.3 mm). Thermal: nested coat-air/shirt shells in trunk, skirt and sleeves, trouser air, shoe air and sock, references for the rest, breath patches, plume patch. Structure: spine, 10 rib hoops, sternum, clavicles, scapulae, sacrum, ilia, pubic and ischial rami, skull, face, mandible, cervical spine, femur with neck and trochanter, condyles, patella, tibia, fibula, foot bones, humerus, ulna, radius, carpus, metacarpals, finger rays, joints, masses; the umbrella frame by reference.
- **1:100 — 69 / 70 / 86.** Same figure in fewer sections; lapels, belt and collar kept; left hand one bent loft plus thumb, right fist palm + one finger band round the handle + thumb; shoes with merged sole; 16 half panels 15 mm; runner. Skeleton with 5 hoops, ischiopubic bars, finger rays.
- **1:200 — 64 / 65 / 60.** Head with nose, hair and bun, belt, collar; three-station legs; 30 mm crook; panels 30 mm; ribs/stretchers true-size slender. Skeleton: 3 hoops, shoulder girdle bar, pelvic ring, one bone per segment.
- **1:500 — 29 / 30 / 43.** Head, hair cap, collar, coat upper overhanging the skirt (clean waist step), sleeves, cuffs, single-form hands, legs, shoes; canopy as 8 whole gores (elliptic sections through rib and both panel middles, superellipse 2.6, overlapping so no slits). Stick skeleton at 75 mm, slender ribs and stretchers in the structure only.

Final check line (`volumes-check.ts humans/person-with-umbrella --product`): `FAIL: 4 measurement(s) over 2 rule line(s)` — `areaDeviation:head-skin` 0.81 (1:50) / 0.76 (1:100), `areaDeviation:shoes` 0.76 (1:50) / 0.36 (1:100). Silhouette IoU elevation / front / plan: 1:50 0.996 / 0.976 / 0.999, 1:100 0.996 / 0.996 / 0.999, 1:200 0.994 / 0.996 / 0.996, 1:500 0.990 / 0.987 / 0.997. Bounds, feature, monotonic, cross-scale, structure coverage, bones, slender, copied, views-disagree: green at every scale.

## 5. What I saw in the renders

- 1:50 body, six directions (front, right, back, left, two oblique): reads at first sight as a woman in a belted raincoat under an open eight-rib umbrella. Front: sloping shoulders, lapels, buttons, pockets, hanging left hand with fingers; right fist round the crook just ahead of the chest. Side: shaft vertical beside the face, canopy rim 6 cm above the head, bun, coat skirt flaring. Back: yoke now flush, folds. Oblique from above: eight gores with ribs and tips, ferrule. Zoom of the upper body before the fixes showed the jagged waist seam and the boxy yoke: both fixed.
- 1:50 `umbrella-furled`: arm hangs, umbrella tip down; canopy panels fold with their ribs but stay as a loose fan about 0.2 m wide (rigid panels cannot wrap; see 6). `gust-heavy`: person leans into the wind, umbrella tilted forward, feet planted; reads right.
- 1:100, 1:200 body (six directions): same figure, belt and lapels / belt only, scalloped hem between ribs. 1:500 body: first version had slits between the gores (petal look), a beret-like hair cap and a jagged waist; gores widened and overlapped, hair domed, waist step introduced; re-rendered: clean.
- 1:50 thermal (r3-sheet, cut): shells nested in the coat and sleeves, regions coloured right, nothing pokes out; the cut through the canopy centre removes most of the figure in the left-side panel (tool plane at the scene centre).
- 1:50, 1:100, 1:500 structure (six directions): skeleton with curved spine, rib hoops, scapulae, pelvis, femora inward, ribs and stretchers of the umbrella; 1:500 sticks inside the ghost. 1:200 structure was not opened as an image.
- 2D (`r3-2d`): 1:50 default pose clean in all three views; 1:50 `umbrella-furled` checked twice: flat front shapes showed a 1 m bowl, fixed with the revolved canopy, now a narrow bell. 1:500 2D was rendered but not opened.
- Overlays (`volumes-render`) 1:50 and 1:100: green everywhere.

## 6. Still imperfect

- Thermal red at 1:50/1:100 for `head-skin` and `shoes`: declared skin shares (C18, not changeable by authors); filed in `author-requests.md` with measurements.
- Furled umbrella: panels are rigid solids on their ribs, so the closed umbrella shows a loose 0.2 m fan instead of cloth wrapped round the shaft; the plan drawing of the furled pose keeps the flat octagon half shapes (true only open).
- The two median ribs (`rib-front`, `rib-back`) are flagged `texture: true` besides `slender`: with only `slender` the product widens them to the width of the canopy shape on their bone (C24 nearest shape), which broke the front and plan silhouette (IoU 0.916, p95 49 mm). Tool behaviour, as reported for single-bed in `author-requests.md`.
- Canopy cloth is as thick as the feature (7.6 / 15 / 30 / 75 mm), not 0.3 mm (no thin role for membranes).
- Grades as I see them: 1:50 body A- (face is schematic, trousers straight tubes), 1:100 A-, 1:200 B+, 1:500 B+ (hands are mitts at the 75 mm floor). Structure A- / B+. Thermal A-.
- `provenance.notes` and `author` unchanged.
