# Round 3 — `animals/red-squirrel` (Sciurus vulgaris), 2026-11-20

State: root members written to the catalog file, volumes spliced, manifest entry (`heightM`, `widthM`, `distinction`) updated.
`r3-validate.ts animals/red-squirrel --catalog-only --volumes`: `VALID (schema 0, validateAsset 0, checkVolumes 0)`. `volumes-check.ts animals/red-squirrel --product`: `PASS` at 1:50 and 1:100 (1:200 and 1:500 are `unavailable`, as the representations are: the animal is 0.75 x 1.05 mm on paper at 1:200).

Two authors. The first (2026-11-19, cut off) left a geometry kit, a corrected rig with solved poses, a first 1:50 and 1:100 body and unfinished lists and drawing code; nothing was in the catalog, no report. I kept his kit, bone lengths and pose solver, judged everything on renders and changed or rewrote: sitting, climbing and all bound poses, head, ears, bibs, feet and soles, both 1:100 lists, the thermal and structure lists, the whole 2D redraw, all texts.

My own grades: 1:50 body A-, thermal A-, structure B+; 1:100 body B+, thermal B+, structure B-; 2D elevation A-, front and plan B. It is a red squirrel from every side and in every pose; it is not perfect (section 6).

Rebuild: `sh r3-red-squirrel-run.sh` (candidate document + validation), `sh r3-red-squirrel-run.sh catalog` (writes the catalog file and the manifest entry, splices, validates). Hand tables: `r3_red_squirrel_rig.py` (bones, anchors, every pose as trunk angles and limb targets), `r3_red_squirrel_body50.py`, `r3_red_squirrel_body100.py`, `r3_red_squirrel_lists.py` (thermal, skeleton), `r3_red_squirrel_draw.py` (2D, ruling E1), `r3_red_squirrel_text.py` (texts, dimensions, material, decisions). Helpers: `r3_red_squirrel_geo.py`, `r3_red_squirrel_fk.py`, `r3-red-squirrel-build.py`, `r3-red-squirrel-bounds.ts`, `r3-red-squirrel-probe.ts` (root and anchors of a clip over time), `r3-red-squirrel-shapes.ts` (extents of the placed 2D shapes), `r3-red-squirrel-view2d.ts` (enlarged 2D preview through QuickLook).

## 1. Reference

| measure | real value | model | source |
|---|---|---|---|
| head and body | 19-24 cm | 0.21 m (rump 72 + chest 62 + neck 24 + head 52 mm) | Gurnell 1987 (asset reference) |
| tail | 14-20 cm | 0.175 m of vertebrae + 19 mm hair = 0.194 m | Gurnell 1987 |
| mass | 0.25-0.40 kg | 0.30 kg (unchanged) | Gurnell 1987 |
| hind foot (heel to toe tips) | 51-63 mm, about a quarter of head and body | 59 mm (foot bone 50 mm from the ankle joint) | field guides, my knowledge; proportion after Thorington & Heaney 1981 |
| ear | 25-36 mm | 32 mm with the short summer tuft, 17 mm wide | field guides, my knowledge |
| skull, greatest length / zygomatic width | about 50-55 / 30-33 mm | skull member 52.5 / 30 mm; head with fur 68 x 38 x 41.6 mm | my knowledge of Sciurus skulls, not a cited table |
| humerus / radius | about 38-41 / 38-40 mm, nearly equal | 40 / 40 mm | proportion after Thorington et al. 1997; lengths my estimate |
| femur / tibia | about 48-52 / 56-60 mm, tibia longer | 50 / 58 mm | same |
| long-bone shafts / joint ends | about 3-4 mm / 7-10 mm | 3.2-4.4 mm / 7.6-9.2 mm | my knowledge |
| acetabula apart / shoulder joints apart | about 24-26 / 30 mm | 26 / 30 mm | my knowledge |
| brush | flattened, about 50-70 mm wide | 54 x 35 mm | my knowledge |
| eye | about 9-10 mm, on the side of the head | 10 x 9.6 mm, 27 mm behind the nose tip | my knowledge |
| stance | plantigrade hind feet, sits on the haunches with the knees up, hands to the mouth | modelled so | Shorten 1954 (asset reference) |
| bounding run | 0.3-0.6 m per bound at an easy pace | 0.36 m per 0.36 s cycle (1.0 m/s) | Hildebrand 1985 (asset reference), my estimate of the stride |

The bone lengths are estimates guided by cited proportions, not cited measurements; the new reference `thorington-1997` says so in its citation.

## 2. Audit of the asset as found

- Rig: pelvis and thorax 80 + 80 mm with a 28 mm neck (head and body 0.24 m, upper end of the species); hip joint 24 mm below the trunk axis and 8 mm from the tail root (real: beside the sacrum); shoulder 20 mm below the axis; no lateral position of hips and shoulders (all limbs in the median plane); humerus and forearm 32 + 32 mm (real 40 + 40), femur 48, tibia 56, foot 44 mm; tail 72 + 80 mm. The summary called the hind limbs digitigrade; squirrels are plantigrade.
- Poses: no abduction anywhere, so sitting knees, climbing limbs and the hands at the mouth lay in one plane.
- `dimensions`: 0.275 high, 0.21 long, 0.12 wide for the sitting animal (the model built on the real lengths is 0.209 x 0.151 x 0.068 m).
- 2D: front and plan were constructed from ellipses and capsules (ears as ellipses floating 27 mm beside the axis, thighs as ellipses, feet 44 mm wide); hands and feet 15-22 mm thick; the belly patch tone 0.05 (black) for white fur; 1:100 had no limbs at all.
- 3D (qa-visual-3: C+): ball ears detached above the head, two fat tail sausages, box-like chest, tube limbs, cartoon joint balls, 1:100 "a pin with ears".
- Clip `bound`: `advanceM` 0.2227 m; hands and hind feet passed below the ground between the keys (measured after the rig fix: hind foot 20 mm, hands 8 mm under y = 0 at t = 0.04 s) and the root moved backwards in the air.

## 3. Root fixes (catalog asset)

| member | old | new |
|---|---|---|
| `rig` lengths | spine-r 0.08, spine-f 0.08, neck 0.028, tail 0.072 + 0.08, arm 0.032, forearm 0.032, hand 0.024, thigh 0.048, shank 0.056, foot 0.044 | 0.072, 0.062, 0.024, 0.085 + 0.09, 0.04, 0.04, 0.026, 0.05, 0.058, 0.05 (head 0.052 unchanged) |
| `rig` joints | shoulder (0.068, -0.02), hip (0.008, -0.024), tail root (-0.008, 0.016), no z | shoulder (0.052, -0.012, z ±0.015), hip (0.016, 0.002, z ±0.013), tail root (-0.004, 0.012), neck root y 0.006; `mass.comX` at mid bone |
| `anchors` | contacts and head points for the old rig | ground-foot (0.04, -0.0049) under the ball on the sole, footprints 50 x 13 and 22 x 10 mm, ground-hand (0.016, -0.0035), bark anchors at the claw tips, nostrils, snout tip and eye on the new head, back-plume on the loin surface |
| `poses` | 12 planar poses | 14 poses solved from trunk angles and limb targets with abduction: sitting (rounded back, knees up beside the belly, hands at the mouth, tail raised with the tip curling away), gather/running, climbing (hind feet beside the pelvis, toes up, head lifted off the bark), nibble, the bound keys, three climb keys; new keys `bound-support` (hands carry the body) and `bound-reach` (airborne, elbows folded) |
| `clips.bound` | 5 smooth keys, feet on the ground until 0.26 s, `advanceM` 0.2227 | 7 linear keys, hind stance 0.14-0.24 s (lift-off at the push key), `advanceM` 0.3591; the root now advances monotonically and no contact anchor goes more than 2 mm below the ground (probed at 36 instants) |
| `clips.climb` | text: body rises 0.11 m | text corrected to what the keys do (0.05 m) |
| `dimensions` | 0.275 / 0.21 / 0.12 | 0.209 / 0.151 / 0.068 (bounding box of the sitting 1:50 model; follows from the cited body lengths, not itself a cited measurement) |
| `summary`, manifest `distinction` | "digitigrade hind limbs" | plantigrade hind feet, body and tail lengths named |
| `materials`, `provenance` | - | material `bone`, reference `thorington-1997` |
| `representations` 50 and 100 | see audit | every shape redrawn (section 4), all eight decision texts rewritten; sizes in the `reason` texts of 1:200 and 1:500 corrected |

`thermal` (all `areaM2`), `structure`, masses and the thermal pose data are untouched. `structure.supports` is empty, so there are no support solids.

## 4. Model

| scale | body | thermal | structure | 2D shapes per view |
|---|---|---|---|---|
| 1:50 | 30 | 49 | 48 | 51 |
| 1:100 | 24 | 27 | 22 | 45 |

- 1:50 body: rump, chest and neck as elliptic lofts with domes about their joints (each inner dome a little smaller, so the seam is one clean line and every pose stays closed); head as a blunt wedge with round braincase and convex forehead; two leaf ears turned 30° outwards, rooted in the skull fur; eyes, nose pad; brush in two flattened links, tip curling away; white underside in three shells (throat, chest, belly) 2.2 mm proud, cutting the red coat at a clear edge; per side upper arm, forearm, paw, finger group, thigh plate, shank with calf, a foot whose underside is one straight line, and the bare sole.
- 1:50 thermal: guard layer = body surface by reference, underfur shell = the body offset inwards by the guard thickness, for all seven fur regions; fingers, soles (upper face marked `sectionFaces`, it lies against the foot) and a cap over the muzzle tip as bare skin; breath outlet = the nose pad, plume outlet = a saddle patch on the loin. Areas: nose 6.1 cm² (+2 %), hands 8.2 (+3 %), feet 16.2 (+1 %).
- 1:50 structure: lumbar, thoracic, cervical column dorsal in the trunk, six slender rib hoops, sternum, two innominate bones through the hip joints, two scapulae, skull with zygomatic width, mandible, caudal vertebrae, humerus, forearm with olecranon, femur, lower leg with real shafts and joint ends, hand and foot plates, 16 joint spheres of joint-end size, two masses.
- 1:100: every mass with three to five own sections; fingers in the paw, throat in the chest patch, no eye or nose pad. Structure: skull, rib-cage hull and pelvis as masses, column and limb bones as slender bars, no joint solids.
- 2D (E1): each shape is the outline of one solid in its bone frame with a `solid` hint; parts deeper than wide are the prism of the side outline plus the body of revolution of the lateral half width, flat parts (head, tail, foot) add the prism of the plan outline; in each view the true outline lies on top and nearer parts lie above. Limbs are drawn at 1:100 too (the validator accepted it), so a running or climbing squirrel has legs at both scales.

Final check line: `PASS`. Silhouette IoU elevation / front / plan: 1:50 0.991 / 0.966 / 0.940, 1:100 0.990 / 0.967 / 0.929. These follow from the redraw and prove nothing about quality.

## 5. What I saw (PNGs opened with the Read tool)

- 1:50 body, six-view sheet, sitting: side views read as a squirrel at once (rounded back, haunch with the knee at mid belly, long flat foot, hands at the mouth, wedge head with a large eye, pointed ear, brush up behind the back). Front: white belly band, narrow pointed ears, haunches beside the body, tail behind. Back: tail over the back, haunches. Top: spindle with the head ahead. Three-quarter from turn 50 and from turn 140: convincing from both.
- 1:50 body, `running` (six views), `climbing` (six views, before and after the foot fix), bound clip at t = 0.04, 0.045, 0.115, 0.2 s: closed at every joint; the first t = 0.04 frame showed the feet under the ground, which led to the clip fix.
- 1:50 structure sheet: a small mammal skeleton inside the ghost; the innominate bones read as broad blades, the skull as a solid wedge.
- 1:50 thermal cut sheet: nested shells follow the body, nothing pokes out, soles, fingers and muzzle cap coloured.
- 1:100 body sheet (sitting) and `climbing`: the same animal with plainer sections. 1:100 structure: skull, rib-cage egg, pelvis keg and bars; readable, crude. 1:100 thermal cut: correct geometry, but the sheet tool draws the fur regions grey (note in `tool-requests.md`).
- 2D: product sheet 1:50 (a 4 mm squirrel under a 0.25 mm pen: a blob with tail and ear, as on paper), enlarged previews 1:50 sitting, 1:100 running, 1:50 climbing (partly cut off by the panel).

Not opened: the product 2D sheet of 1:100, bound frames after t = 0.2 s (checked by numbers only), `nibble-b`, climb keys b and c in 3D, single-list `volumes-render.ts` renders (I used the six-view sheets).

## 6. Still imperfect

- The tail is two rigid links: a blade with a kink, not the S-curve of a living squirrel. A third link or skinned sections would fix it.
- The hands are flat paddles with one finger group; no single fingers, no nut. Single 2 mm fingers are under the 7.5 mm feature.
- Seams: jagged grey lines where trunk masses and limbs intersect (tessellation of grazing surfaces), most visible at the tail joint and the shoulder.
- Structure: long-bone shafts are true size only because the joint ends reach 7.6 mm (the feature rule measures a loft by its thickest section). No clavicle (too short for a slender bar). At 1:100 the column bars are 1.4-4.4 mm (real about 8 mm) because a slender solid must be 20 times as long as thick, and there are no joint solids.
- 2D front and plan: small rectangles where prism ends show (ears from above, bib ends, soles from the front); the trunk seen along its axis is a circle with a thin fin.
- The foot is rigid: at push-off the toes dip up to 1.5 mm under the ground; the hands dip 1.9 mm at t = 0.10 s.
- Bone lengths and skull dimensions rest on my knowledge and cited proportions, not on a measured table.

## Round 4 review (2026-11-20)

Reviewer and fixer in one. Scripts: the round-3 modules, edited (`r3_red_squirrel_body50.py`, `r3_red_squirrel_body100.py`, `r3_red_squirrel_lists.py`, `r3_red_squirrel_volumes.py`), and the new `r4_red_squirrel_coarse.py` (1:200 and 1:500).

Reference numbers used: head and body 0.21 m, tail 0.175 m of vertebrae, brush 54 mm wide, innominate bone about 45 mm, 12–13 pairs of ribs, column about 8 mm under the skin of the back (section 1 and my knowledge).

| scale | list | before | after |
|---|---|---|---|
| 1:50 | body | B+ (brush two straight links with a hard kink) | A- |
| 1:50 | thermal | A- (opened before only) | A- |
| 1:50 | structure | B- (column 12 mm above the trunk axis and 9 mm thick, six rib hoops) | A- |
| 1:100 | body | B+ | A- by the same tail change (sheet not opened after it) |
| 1:100 | structure | B- (rib cage an egg) | B+ |
| 1:200 | all | C (bulk pass: faceted masses with ragged patches; not in the round-3 source) | A- body; thermal and structure not opened |
| 1:500 | all | C (same) | B+ body; thermal and structure not opened |

What I changed:
- **Brush**: both links bowed (offsets 2 to 4.6 mm across the link, the joint section moved 3 mm), so the two rigid links read as one S; same at 1:100.
- **Skeleton 1:50**: column 14.4–16 mm above the trunk axis and slimmer, nine rib hoops instead of six, hung under it; thoraco-lumbar joint moved with it. **1:100**: column raised, the rib-cage egg replaced by four rib hoops.
- **1:200 and 1:500 authored**: 22 and 19 masses with their own stations, own nose, breath and plume solids, own bar skeletons with three and two rib hoops, bilingual decisions.

Checks after the last change: `volumes-check.ts animals/red-squirrel --product` PASS at four scales, `r3-validate.ts animals/red-squirrel` VALID, spliced, catalog root members rewritten by the round-3 chain (2D redrawn from the new tail).

Opened after the change: 1:50 body and structure, 1:100 structure, 1:200 body, 1:500 body (the last before a final shortening of the forearm piece and one more tail station). Not opened after the change: every thermal sheet, structure 1:200 and 1:500, 1:100 body, poses `running` and `climbing`, the 2D sheets.

Still not A:
- Jagged grey seams where the trunk domes and the limbs graze each other (shoulder, haunch, tail joint) at every scale.
- The brush is still two rigid links; in poses that bend the tail joint far from the sitting angle the bow may read wrong. Not looked at.
- 1:100 pelvis is a keg; hands are paddles; the round-3 list of section 6 stands.
