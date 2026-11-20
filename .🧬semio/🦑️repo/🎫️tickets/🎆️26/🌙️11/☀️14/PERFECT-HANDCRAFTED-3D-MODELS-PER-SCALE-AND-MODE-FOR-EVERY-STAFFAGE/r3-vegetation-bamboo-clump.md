# Round 3 — `vegetation/bamboo-clump` (yellow-groove bamboo, *Phyllostachys aureosulcata*, six culms in a rhizome barrier)

Resumed on 2026-11-20 after the interruption of 04:10 (round-3 scripts, source and a spliced earlier build existed; the catalog roots were identical to the candidate, the bulk passes of 03:40 to 04:10 had added no scale and changed no member of this asset outside `volumes`).

No earlier round-3 work existed for this asset (no `r3-bamboo-clump-*` scripts, no report). Starting state: round-2 volumes (author 54) spliced, `r3-validate` VALID, but the 2D could not be drawn at 1:50 and 1:100 (`r3-2d.ts`: "no suitable representation": 1 and 57 print-gap violations of the round-2 drawings).

Scripts (ticket folder): `r3-bamboo-clump-plant.ts` (the plant in world space per scale: every measure stated in tables at the top), `r3-bamboo-clump-model.ts` (root fixes, masses and sections from the modelled culms, volumes of all scales, 2D redraw, plan symbol, decisions; writes the candidate and the source), `r3-bamboo-clump-roots.py` (writes the root members of the candidate into the catalog file), `r3-bamboo-clump-baseline.json` (input: the round-2 rig, poses, anchors, materials, symbols as they were, so every root fix is idempotent), `r3-bamboo-clump-ship.sh` (build, validate, roots, splice), probes `r3-bamboo-clump-why.ts`, `-suit.ts`, `-masks.ts`, `-follow.ts`, `-find.ts` (one product rule of one scale with the culprits by id), `-member.ts` (a member against its culm as the product draws them).

## 1. Reference

| item | real subject | source | asset |
|---|---|---|---|
| species, habit | running (leptomorph) bamboo; culms erect, slightly zig-zag at the base, tops nodding; evergreen | Ohrnberger 1999; Liese and Koehl 2015 | as modelled |
| culm size | *P. aureosulcata* to 6–8 m and 3–4 cm; in a garden clump 3–5 m; height and diameter go together (about 150–170 × D) | Ohrnberger 1999; Liese and Koehl 2015 | culms 2.43–4.50 m (rig, unchanged) with 16–29 mm (was 29 mm for every culm) |
| taper | nearly cylindrical in the lower third, 80 % of the base diameter at mid height, tip a few millimetres | Liese 1985 (culm form) | `TAPER` table: 1.0 / 0.93 (0.3 L) / 0.80 (0.5 L) / 0.60 / 0.40 / 0.13 (tip) |
| wall | about an eighth of the outer diameter; closed at every node by a diaphragm | Liese 1985 | 0.13 D (3.8 mm at 29 mm; the material title says 3.5–4 mm) |
| internodes | short at the base (3–6 cm), longest at about a third of the height (15–20 cm for a 3 cm culm), shorter towards the top | Liese 1985; Liese and Koehl 2015 | `INTERNODE` table, longest 19 cm × (D/29 mm)^0.6; 21–27 nodes per culm; every rig joint on a node |
| node | sheath scar (lower ring) and supranodal ridge (upper ring), moderately raised in *P. aureosulcata* | Ohrnberger 1999 | scar ring +5.5 %, ridge +7 % of the radius at 1:50 |
| branching | from mid-culm upward, two unequal branches per node, on alternating sides (the groove above the branch alternates); branches leave at 40–60° and arch out | Ohrnberger 1999 | from 40 % of the culm length, primary and secondary (0.55 of the length) branch, 52–32° from the culm, droop 32–42° at the tip, 0.12–0.58 m |
| leaves | 2–4 per ultimate branchlet in a fan, lanceolate, 5–15 cm × 1–2 cm, hanging | Ohrnberger 1999 | fans of 2–4, blades 10–14 cm × 1.7 cm, petiole narrowed base, widest at 30 %, acuminate tip, arching down by 4 cm |
| barrier | HDPE sheet about 2 mm, 60–70 cm deep, a few cm proud of the soil so rhizomes cannot jump it | RHS (bamboo spread) | 1.6 × 1.0 m loop (the declared support footprint), 2 mm, 5.5 cm proud |
| leaf mass | fresh leaf about 0.13 kg/m² (dry LMA 55–65 g/m², about 55 % water) | Jones 2014 (leaf traits) | leaves 13.6 m² × 0.13 = 1.77 kg |

## 2. Audit against the reference (state before round 3)

| item | found | reference | verdict |
|---|---|---|---|
| culm diameter | 29 mm for all six culms of 2.43 to 4.50 m | 16 to 29 mm (about 155 D) | wrong for five culms |
| culm solids (round 2) | lofts without internodes, nodes or taper table; round 1 had been a 0.74 m wide ribbon (`model-findings.md`) | internodes, double node rings, taper | detail missing |
| culm axis | straight bone to bone, kinks of 0.5 to 6.1 degrees at the two lower joints | a culm does not kink at a node | wrong form |
| rig, lateral | all culms in the plane z = 0 (no `z` on any bone); front and plan drawings placed the culms differently (`author-requests.md` 54: culm 1 top z 0.57 in the front, 0.44 in the plan) | six culms spread over a 1.6 x 1.0 m bed, leaning outwards | root defect |
| rig, root plate | plate bone from x 0 to 1.6, culms from x -0.6 to 0.62: the clump stood off its own rhizome plate | plate under the culms | root defect |
| support anchor | at the plate joint (plate end) | under the plate centre | wrong |
| bone masses | culm bones 0.13 to 0.32 kg from the 29 mm section, foliage bones 1.0 to 1.9 kg, total 20.9 kg | hollow culm at 700 kg/m3, leaves 13.6 m2 at 0.13 kg/m2 = 1.77 kg | foliage about ten times too heavy |
| branching, leaves | foliage as masses on the top bone only | branches from 40 % of the culm, two per node, alternating; fans of leaves | missing |
| barrier | not modelled, not a material | HDPE 2 mm, 5.5 cm proud | missing |
| 2D | 1:50 and 1:100 not drawable (`r3-2d.ts`: no suitable representation, 1 and 57 print-gap violations); culm 80 mm wide in the front, 29 mm in the elevation | true views of one body | root defect |
| dimensions | 2.977 x 4.552 x 1.714 m | garden clump of six culms to 4.5 m | kept (the model is built to them; no cited reference contradicts them) |

## 3. Root fixes (written by `r3-bamboo-clump-roots.py`: `rig`, `anchors`, `summary`, `provenance`, `dimensions`, `materials`, `representations`, `symbols`, `poses`)

| member | old | new |
|---|---|---|
| `rig.bones[root-plate].x` | 0 | -0.9 (plate from x -0.9 to 0.7, centred under the culms); every `culm-k-a.x` + 0.9 so that no culm moves in the world |
| `culm-k-a.z` (k = 1..6) | none | 0.25, -0.28, 0.12, -0.22, 0.20, -0.10 m |
| `culm-k-b.z`, `culm-k-c.z`, `fol-k.z` | none | parent length x tan(lean), leans 4.3, -2.6, 2.2, -3.5, 3.5, -5.5 degrees (culm 1: 0.078, 0.0758, 0.064) |
| bone `section` (24 bones) | 29 / 27 / 23 mm for every culm, none on `fol-k` | hollow sections from the modelled diameter at the joint, wall 0.13 D: culm 1 24.0 / 22.5 / 18.1 / 12.1 mm ... culm 6 16.0 / 15.0 / 12.1 / 8.0 mm (all in `model.log`) |
| bone `mass` (24 bones) | e.g. `culm-1-a` 0.228 kg, `fol-1` 1.554 kg, com on the axis | integrated from the solids: `culm-1-a` 0.146 kg com (0.510, -0.006), `fol-1` 0.157 kg com (0.424, 0.064) |
| `dimensions.massKg` | 20.9 | 11.8 (8 kg plate unchanged, 3.8 kg culms, branches and leaves) |
| `anchors[base-support].x` | 0 | 0.8 (plate centre) |
| `anchors[tp-sun-k]`, `[tp-shade-k]` | on the old masses (e.g. `tp-sun-1` 0.5586, 0.0489, 0.551) | on the outlet leaf of each culm top (`tp-sun-1` 0.8523, 0.2226, 0.2479) |
| `materials` | 2 | 4: + `hdpe-root-barrier`, `leaf-litter` |
| `provenance.references` | | + `rhs-bamboo`, `wang-2013-culm`, `ohrnberger-1999`; notes rewritten |
| `representations[50..500].geometry` | round-2 drawings | every view redrawn as the true view of that scale's body (ruling E1): culm bones as body strokes of their mean diameter on the true axis, node rings as texture strokes, branches as detail strokes, foliage as filled outlines per bone and light, barrier and bed; decisions rewritten |
| `symbols[0]` | old crown ellipse | true foliage hull of the 1:200 model with the six culm sections |
| `poses`, `clips` | unchanged (bone angles are the same; the lateral offsets ride on the bones) | checked: `storm-bent` at 1:50 stays closed at every joint |

## 4. What is modelled (final build)

Every measure is stated in the tables at the top of `r3-bamboo-clump-plant.ts`; the culm axis is a cubic Hermite curve through the four rig joints of the culm with the bisector of the meeting bone axes as tangent, so the culms arch without a kink and every joint is still a node.

| scale | body | thermal | structure |
|---|---|---|---|
| 1:50 | 3750: 24 culm lofts (4 bones x 6 culms, 21 to 27 nodes per culm, each a sheath-scar ring under a supranodal ridge), 188 branches (two unequal per node from 40 % of the length, alternating sides, drooping), 598 branchlets (capsules), 2938 leaves (lanceolate blades of 10 to 14 cm, 0.8 mm thick, arching down, fans of 3 to 5), barrier (2 mm HDPE loop, extrude with hole) and bed | 3760: every culm, branch, branchlet and leaf by reference (culms / leaves-sun / leaves-shade, 40 % of the leaf area sunlit: the upper and outer fans), 12 outlet patches on one sunlit and one shaded leaf of each culm top | 51: 24 culm members along the culm's own curve at 95 % of the outer diameter, 24 joints at the rig joints, rhizome plate slab inside the barrier, support pad, plate mass |
| 1:100 | 1006: culms with one ring per node, both branches of every node (188), 792 sprays (one per leaf fan: the sector the splayed leaves sweep, drooping, 16 mm thick), barrier, bed | 1016: all by reference + 12 outlets on sprays | 51, members with the 1:100 stations |
| 1:200 | 126: 24 plain tapered culm lofts with nodding tops, 100 drooping tufts (one per node: branch pair with its leaves), barrier rim, bed | 230: 124 references, 94 branches as slender thermal solids inside the tufts (culm area), 12 outlets | 51 |
| 1:500 | 80: 24 culm lofts, 56 foliage masses following the branch tiers | 186: 80 references, 94 branches, 12 outlets | 51, plate as a 75 mm block at the central culm with its mass inside |

Changes of this session against the build of 02:13: culm axis smooth instead of bone-straight; 1:100 keeps the second branch of every node with its fans (it had dropped them, 510 sprays) and the sprays have the outline of the fan instead of one large leaf; 1:50 thermal references the real leaves (it had used 792 stand-in sprays that are not in the body) and the outlets sit on real leaves; culm members are lofts on the stations of the body culm (the hollow revolves were drawn by the product as rings in the wall, see section 7); 1:500 plate mass moved into the plate block.

