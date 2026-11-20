# Round 3 — `humans/elderly-with-cane`

Author scripts (ticket folder): `r3-elderly-with-cane-kit.ts` (constructors), `r3-elderly-with-cane-s50.ts` (every section of the 1:50 body, thermal shells and skeleton), `r3-elderly-with-cane-coarse.ts` (1:100, 1:200, 1:500, each stated anew), `r3-elderly-with-cane-build.ts` (writes `volumes/humans/elderly-with-cane.json`), `r3-elderly-with-cane-roots.ts` (cane bone, anchors, arm of every pose and clip key), `r3-elderly-with-cane-draw2d.ts` (2D redraw), `r3-elderly-with-cane-edit.py` + `r3-elderly-with-cane-texts.json` (splice of 2D geometry, drawing decisions, materials, dimensions), probes `r3-elderly-with-cane-probe*.ts`, `-fk.py`, `-dump2d.py`, `-crop.sh`.

This asset was taken over from an author who was cut off. Found: the cane root fix already written to the catalog (2D not yet redrawn, so the catalog was INVALID with 16 extent findings), a finished 1:50 list set in the source, no coarser scales, no report. Kept after looking at it: the root fix and the 1:50 model. Done in this run: 1:100, 1:200, 1:500 (all lists), the 2D redraw of all 12 drawings, materials, dimensions, drawing decisions, thermal corrections, splice, this report.

State: spliced. `r3-validate.ts humans/elderly-with-cane` and `--catalog-only`: `VALID ... schema 0, validateAsset 0`. `volumes-check.ts humans/elderly-with-cane --product`: every rule green at every scale except `volumes-thermal-coverage` at 1:50 and 1:100 (`cap` at both, `shoes` at 1:50; see "Still imperfect").

## 1. Reference (man of 74, 1.70 m in his prime, about 1.65 m now, 1.63 m in the stooped stance with shoes and cap, 70 kg + 3.8 kg clothing and cane)

| quantity | reference | asset | source |
|---|---|---|---|
| hip joint height | 0.52 H = 0.884 m (H 1.70) | 0.870 m (shod) | Winter 2009 table 4.1; Drillis and Contini 1966 (asset references) |
| knee joint height | 0.285 H = 0.485 m | 0.465 m | Winter 2009 |
| ankle joint height | 0.039 H = 0.066 m + sole | 0.070 m | Winter 2009 |
| thigh / shank length | 0.245 H / 0.246 H = 0.417 / 0.418 m | 0.405 / 0.395 m | Winter 2009 (the asset scales lengths by 0.93 of a 1.77 m norm, see `provenance.notes`) |
| shoulder joint height, erect | 0.818 H = 1.39 m | 1.353 m erect, 1.338 m stooped | Winter 2009; minus 3 to 4 cm trunk height loss by vertebral compression and kyphosis at 70 to 80 years (Sorkin et al. 1999, longitudinal stature loss) |
| upper arm / forearm / hand | 0.186 H / 0.146 H / 0.108 H = 0.316 / 0.248 / 0.184 m | 0.307 / 0.242 / 0.177 m | Winter 2009 |
| shoulder joint spacing | biacromial 0.245 H = 0.417 m minus 2 x 2.5 cm | 0.37 m | brief; ANSUR II |
| hip joint spacing | 0.17 to 0.19 m | 0.18 m | Bell et al. 1990 |
| head height chin to vertex | H / 7.5 = 0.227 m | 0.219 m | brief; ANSUR II menton to vertex 0.232 m |
| head breadth / length | 0.152 / 0.197 m | 0.151 / 0.191 m | ANSUR II |
| trunk inclination, head carriage | thoracic kyphosis of age 40 to 55 degrees Cobb, forward head | torso -16, neck -18, head +20 degrees | Perry and Burnfield 2010; clinical kyphosis ranges |
| chest breadth x depth (jacket) | 0.32 x 0.25 m body, +3 to 4 cm clothing | 0.36 x 0.28 m | ANSUR II, older build with belly |
| bideltoid breadth (jacket) | 0.49 m + clothing | 0.48 m with sleeves | ANSUR II |
| thigh / knee / calf / ankle girth (trousers) | 0.56 / 0.38 / 0.37 / 0.22 m body | sections 0.17 x 0.20, 0.13 x 0.14, 0.128 x 0.142, 0.124 x 0.144 m | ANSUR II; straight wool trousers hang from the seat |
| hand breadth / thickness | 0.086 / 0.028 m | 0.083 / 0.029 m | ANSUR II |
| shoe | EU 43: 0.285 m long, 0.104 m wide, heel 25 mm | 0.285 x 0.104 m, heel 25 mm, sole 12 mm | manufacturer size tables |
| cane | handle at the wrist crease of the standing user (0.485 H = 0.82 m); shaft 22 mm, crook radius 45 mm, rubber ferrule 28 mm; tip 10 to 15 cm ahead of and beside the toes | shaft 0.734 m + crook = handle top 0.82 m, shaft 22 to 18 mm, ferrule 28 mm, tip 0.12 m ahead of the toes, 0.135 m beside the foot axis | cane fitting rule of physiotherapy practice (wrist crease, elbow 15 to 30 degrees); derby and crook cane product data |
| elbow on the cane | 15 to 30 degrees flexion | 29.6 degrees | same |
| flat cap | crown about 0.25 x 0.20 m, peak 6 to 7 cm | 0.24 x 0.20 m, peak 7 cm | product dimensions |
| segment masses and centres | de Leva 1996 | unchanged | asset |
| body surface | DuBois 1.8 m² | unchanged | asset |

## 2. Audit findings

Rig
- Human joints: hip, knee, ankle, shoulder, elbow, wrist heights and all limb lengths within 2 cm of the reference for a man who has lost stature; shoulder and hip spacing right; kept.
- Cane (root defect, fixed by the first author): bone 0.87 m long from an origin 88 mm along the hand, so the handle stood near the waist of the forearm line and the elbow was bent 87 degrees in the default pose (68 to 109 degrees in the others); the cane had no lateral offset from the hand (z absent), no crook geometry in the rig, and the hand was not on the handle.
- Far arm: bent up to 86 degrees with the hand in front of the belly in `resting-on-cane`; it now hangs plumb.

Dimensions: `widthM` 0.54 and `depthM` 0.48 described the old cane position (tip far ahead).

2D drawings (all 12): after the cane root fix the old cane shapes ran through the ground (drawn height 1.76 to 1.89 m against 1.63 m, plan length 0.95 m). Before that: elevation outlines of an older study with a hunched jacket of a different form than front and plan, front view a symmetric mannequin with ball knees and tube limbs, plan made of stadiums; whole-asset extents disagreed between the views (`volumes-views-disagree` y 0.10 to 0.13 m, x 0.44 to 0.46 m at every scale); cane drawn 0.87 m long.

Solids (catalog before this round, 97 / 100 / 38 at 1:50 from the earlier rounds, 45 / 34 / 38 at 1:100): 1:100 to 1:500 were box-and-prism figures with shoulder blocks, a floating cane beside an open hand, structure = one loft or box per bone (body copies), thermal emitters as discs.

## 3. Root fixes (old -> new)

| member | old | new |
|---|---|---|
| `rig.bones[cane]` x, y, z | 0.0884, 0, absent | 0.1172, -0.0004, -0.04 (shaft top under the gripping hand, beside it) |
| `rig.bones[cane].angleDeg` / `lengthM` | 0 / 0.87 | -40 (rigid in the hand; the pose angle is 0) / 0.734 |
| `rig.bones[cane].mass` comX, comY | 0.3915, 0 | 0.29, -0.005 |
| `anchors[cane-tip].x` | 0.87 | 0.734 |
| `anchors[cane-grip]` x, y | 0.0884, 0 | 0.0538, -0.0059 |
| `poses[*].bones` arm-r / forearm-r / hand-r / cane | e.g. default -26.3 / 87.2 / 0 / -51.7 | re-solved so the tip stands on the ground where it is a contact: default -3.75 / 29.6 / 19.2 / 0; all nine poses |
| `poses[*].bones` arm-l / forearm-l / hand-l (three standing poses) | 6 / 16 / 3, 14 / 22 / 3, 34 / 86 / -10 | plumb arm: -3 / 12 / 6, 5 / 12 / 6, -8 / 12 / 6 |
| `clips[cane-gait]`, `clips[rest-on-cane]` right arm keys | old cane | re-solved per key (tip fixed while planted, lifted 45 mm in the swing) |
| pose descriptions `stooped-standing`, `cane-planted`, `resting-on-cane`, `contact-left` | old wording | rewritten for the new cane placement |
| `materials` | 13 entries | + `bone` (as sheep and goat: emissivity 0.9, absorptivity 0.5, 1900 kg/m³, authored estimate), + `spectacle-frame` (emissivity 0.3, absorptivity 0.5, authored estimate) |
| `dimensions.widthM` / `depthM` | 0.54 / 0.48 | 0.50 / 0.49 (extents of the default pose with the cane placed by the fitting rule) |
| manifest row `elderly-with-cane` `widthM` | 0.54 | 0.5 |
| `representations[*].geometry` | 340 shapes | 331 shapes, all redrawn, see below |
| `representations[*].decisions` | texts of the old drawings | rewritten (8 texts per scale, en and de) |

The first four blocks were written by the first author (`r3-elderly-with-cane-roots.ts`); I verified them by FK, by the gait pose render and by the validator, and did not change them.

2D redraw (ruling E1): `r3-elderly-with-cane-draw2d.ts` takes, per drawn shape, the body solids it stands for (stated by hand per view), projects them in the default pose, traces the outline at a third of the tolerance, simplifies it (Douglas-Peucker 0.9 / 1.8 / 3.5 / 7 mm) and solves it into the shape frame of its bone with the product's own placement (`inverse` of `r3-project2d.ts`, residual under 0.01 mm for every shape). Layer order per view is stated by hand (far to near). Shapes and points (elevation / front / plan): 1:50 42 / 57 / 28 shapes, 620 / 766 / 579 points; 1:100 28 / 31 / 25, 326 / 393 / 383; 1:200 23 / 22 / 22; 1:500 19 / 18 / 18, 134 / 143 / 173. One id, tone, region and material per part in all views. Strokes stated by hand: 1:50 glasses (temple and rim in the side view, two rims and the bar in the front), mouth line, nose line, jacket front edge; 1:100 jacket front edge. Tweed hatch kept on the 1:50 side view of the jacket. The cane at 1:200 and 1:500 is a strip of 36 and 65 mm along the true shaft axis (crook as a 0.18 mm stroke at 1:200), the 3D cane stays 22 mm and `slender`.

## 4. What was modelled (solids body / thermal / structure)

- **1:50 — 97 / 135 / 139** (first author; I changed the thermal list only). Body: jacket as one loft of 12 horizontal sections falling from the rounded back, lapels, waistcoat, shirt front, buttons, pockets; sleeves with inset sleeve heads, cuffs, shirt cuffs, wrists; seat and trouser legs with calf and break over the instep, socks, shoes with upper, toe box, heel and welted sole; neck, shirt collar, jacket collar; head with chin, jaw, cheeks, brow, nose, nostrils, lips, ears, eyes, glasses; nape and temple hair; flat cap with peak; left hand relaxed with thumb and three-part fingers, right hand wrapped over the crook; cane with tapered shaft, crook, ferrule. Thermal: nested shells for jacket (3), trousers (2), shoes (2), cap (2) at the declared thicknesses, open skins for face and neck, outlet patches under the nose, on the lips and on the cap. My changes: the trouser seat is no thermal surface (it lies under the jacket skirt), the cap underside is a face against the head, breath patches 8 mm. Structure: spine with kyphosis, 9 rib hoops, sternum, clavicles, scapulae, sacrum, ilia, pubic and ischial rami, cranium, facial bones, mandible, cervical spine, femur with neck, trochanter and condyles, patella, tibia, fibula, calcaneus, midfoot, phalanges, humerus, radius, ulna, carpals, metacarpals and finger bones, ball joints, 13 masses, cane by reference. No support solids (`structure.supports` is empty).
- **1:100 — 36 / 38 / 76.** New sections: jacket 9 rows, shirt-front wedge, legs 4 stations each, shoes with merged sole, mitten hand with thumb, gripping palm with a finger band and thumb, nose and ears as single forms, cap and 15 mm peak, cane in three parts. Skeleton with merged parts (5 rib hoops, ischiopubic bars, jaw, tibia with widened head, hand plates).
- **1:200 — 23 / 25 / 41.** Jacket 6 rows, legs 3 stations, one shoe form per foot bone, mitten hands, head with chin, cap drawn into its peak as one form, cane one slender bent rod. Skeleton: skull, cervical bar, spine, 3 rib hoops, shoulder girdle bar, pelvic ring, one bone per limb segment, 13 joints, 5 masses.
- **1:500 — 19 / 21 / 28.** Jacket 5 rows, thigh and shank, one shoe per foot, sleeve and forearm, hands as single forms, head, cap, nape hair, cane. Structure is a stick skeleton at the 75 mm floor with shoulder and pelvic bars, 8 joints, 2 masses.

Final check line: `FAIL: 3 measurement(s) over 2 rule line(s)`: `areaDeviation:cap` 0.396 (1:50) and 0.362 (1:100), `areaDeviation:shoes` 0.548 (1:50). Silhouette IoU elevation / front / plan: 1:50 0.996 / 0.996 / 0.998, 1:100 0.995 / 0.997 / 0.996, 1:200 0.985 / 0.987 / 0.989, 1:500 0.967 / 0.990 / 0.959. Bounds, feature, monotonic, cross-scale, structure coverage, bones, slender, copied: green at every scale.

## 5. What I saw in the renders

- `r3-sheet` 1:50 body, six views, default pose: reads at first sight as an old man in jacket and flat cap leaning on a crook cane. Front: sloping shoulders, lapels, shirt and waistcoat, pockets, glasses, hanging left hand with fingers, right hand on the handle. Sides: rounded upper back, head carried forward, jacket falling over the seat, straight trousers breaking on the shoe, cane tip on the ground ahead of the toes. Back: nape hair under the cap, plain jacket back. Top: cap over shoulders, hand on the crook. Three-quarter (single view): convincing; the knee shows a slightly ragged intersection line between thigh and shank lofts.
- 1:50 body in `contact-left` (six views): stride closed at hip, knee and ankle, heel strike and toe-off right, cane planted, elbow closed.
- 1:50 structure (six views): recognisable stooped skeleton, femora running inwards, rib cage tilted with the trunk, scapulae on the back. 1:50 thermal cut (six views): shells nested, nothing outside the body. Both seen before my thermal edits; the edited 1:50 thermal list was not rendered again.
- 1:100, 1:200, 1:500 body (six views each): same man, same stance, progressively plainer; at 1:500 the first version let the seat poke through the jacket back, fixed by another jacket row and seen only through the green checks afterwards (the 1:200 and 1:500 body sheets were not opened again after the last changes: cap of 1:200 merged with its peak, hands of 1:500 thickened).
- 1:100, 1:200, 1:500 structure (six views each): 1:100 a full skeleton, 1:200 hoops and bars, 1:500 sticks; all inside the body ghost.
- 1:100 thermal cut (six views) showed breath patches of 10 cm (a wrong factor of mine); fixed, and the 1:100 thermal right side view opened afterwards: patches small on nose and lips, regions right, the plume patch stands a little proud of the cap as a thin tilted disc.
- `r3-2d` 1:50 default pose (candidate before the cane strip and text changes): clean line and tone drawings in all three views, front with lapels, glasses and pockets. `r3-2d` 1:50 `contact-left`: the side view strides correctly; front and plan are flat shapes carried by their bones, so the stride shows in plan as a displaced leg outline and hardly at all in front. `r3-2d` 1:100, 1:200, 1:500 final state: read as the same figure, cane visible at every scale.
- Not opened: `volumes-render.ts` overlay sheets (the silhouette numbers above stand in for them), thermal and structure sheets from a second direction as single views, `rest-on-cane` and `cane-gait` clip frames.

## 6. Still imperfect

- **Thermal coverage is red at 1:50 and 1:100** for `cap` (+40 % / +36 %) and at 1:50 for `shoes` (+55 %). Both declared values are skin shares (scalp, feet), not the surfaces of the cap and the shoes; C18 forbids me to change them. Filed in `author-requests.md` ("r3 humans/elderly-with-cane"). The `sectionFaces: ["-u"]` mark on the cap underside is my reading of C25 b (a face lying against the head); if the reviewers reject it the cap reads 0.120 m².
- Front and plan 2D shapes are flat outlines without `solid`: exact in the default pose, approximate in the gait poses (as the old drawings were).
- Body grade as I see it: 1:50 A- (knee intersection line, no cloth folds beyond the trouser break, symmetric limb sections), 1:100 A-, 1:200 B+ (angular shoulders), 1:500 B+ (hands thickened to 54 mm and a fist of 80 mm on the cane because the feature rule follows the drawing).
- 1:500 structure members are 75 mm sticks by rule, close to the size of the limbs they are in.
- The hair region at 1:500 is the 44 mm nape hair solid, below the 75 mm feature but as thick as its drawing.
- `provenance.notes` and `provenance.author` are unchanged and do not mention this round.
