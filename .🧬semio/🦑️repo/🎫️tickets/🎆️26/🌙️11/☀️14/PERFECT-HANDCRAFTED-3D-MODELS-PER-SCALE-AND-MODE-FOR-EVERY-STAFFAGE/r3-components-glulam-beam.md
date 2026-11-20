# Round 3: components/glulam-beam

Status: in progress (roots written to the catalog; volumes being modelled).

Scripts: `r3-glulam-beam-kit.py` (lamella lay-up shared by 2D and 3D), `r3-glulam-beam-roots.py` (root fixes, `--write`), `r3-glulam-beam-model.py` (volumes source). Output under `🗑️generated/r3-glulam-beam/`.

## 1. Reference

| item | real value | source |
|---|---|---|
| strength class | GL24h, E0,mean 11.6 GPa, mean density 420 kg/m3, fm,k 24 MPa, fc,90,k 2.5 MPa | EN 14080:2013 Table 5 (`provenance.references` en-14080) |
| lamella thickness | 40 mm (EN 14080 allows up to 45 mm in service classes 1 and 2) | EN 14080 |
| section | 160 x 600 mm = 15 lamellae; widths in 20 mm steps, depth a multiple of the lamella | manufacturer ranges, EN 14080 |
| section values | A = 0.096 m2, I = b h3 / 12 = 0.00288 m4, W = 0.0096 m3, extreme fibre 0.30 m | hand calculation |
| mass | 0.096 x 8 x 420 = 322.56 kg (0.396 kN/m self-weight) | hand calculation |
| board lengths | sawn spruce 3 to 5 m; lamellae are finger-jointed endless and cut to length, so an 8 m lamella has one or two joints | sawmill practice |
| finger joints | finger length 15 or 20 mm, pitch 3.8 or 6.2 mm; profile visible on the narrow face of the lamella, i.e. the beam side (both orientations exist in production; the asset's drawing had the side profile and keeps it) | EN 14080 / former EN 385, knowledge; web search (Wikipedia "Glued laminated timber", prEN 14080 abstract) did not settle the orientation |
| joint placement | staggered between neighbouring lamellae; good practice keeps joints of the outer tension lamella out of the zone of highest moment | glulam production practice |
| arrises | long edges eased or chamfered by a few millimetres; ends square-sawn | manufacturer delivery standard |
| bearings | steel plates 120 x 20 x 160 mm, one per end, under the square end; effective span = distance between bearing centres = 8.00 - 0.12 = 7.88 m | EN 1995-1-1 5.4.2, hand calculation |
| bearing check | ULS reaction (1.35 g + 1.5 q) = 6.53 kN/m x 7.88 / 2 = 25.7 kN; Aef = (120 + 30) x 160 mm = 0.024 m2, sigma c,90 = 1.07 MPa < kc,90 fc,90,d = 1.75 x 1.54 = 2.69 MPa | EN 1995-1-1 6.1.5 |
| bending and deflection | M,d = 50.7 kNm, sigma m,d = 5.3 MPa < fm,d 14.8 MPa (utilisation 0.36); w(q) = 6.0 mm = L/1300 | hand calculation |
| loads | floor imposed 2 kN/m2 over 2 m tributary width = 4 kN/m; concentrated 2 kN | EN 1991-1-1 Tables 6.1, 6.2 |
| thermal storage depth | half the 24 h penetration depth of 73 mm = 36 mm (24 kJ/m2K) | EN ISO 13786 |

## 2. Audit (before)

Rig and anchors
- `beam` bone: origin (0, 0.32) on the neutral axis (0.02 plate + 0.30), length 8 m, section A 0.096 / I 0.00288 / extreme fibre 0.30: correct.
- `load-carrier`: prismatic joint at x 0.5, range 0..7 m: correct.
- Anchor `bearing-left` at (0, 0) and `bearing-right` at (8, 0) on the beam axis: wrong. The bearing plates lie at x 0..0.12 and 7.88..8.00, so the reactions act at the plate centres x 0.06 and 7.94 on the support below (y -0.32 = ground). The solved span was 8.00 m instead of 7.88 m (midspan moment under the 2 kN point load 7.16 instead of 7.01 kNm) and the support glyphs sat in the end faces at mid-depth.
- Anchor `load-point` at the beam axis (y 0): the load acts on the top face (y +0.30).

Dimensions: height 0.62 (beam 0.60 + plate 0.02), length 8.00, width 0.16, mass 322.56 kg: all correct.

2D drawings
- 1:50 elevation `beam-chamfered`: 6 mm 45-degree cuts at the four corners of the side view. A beam with chamfered long edges has square ends; in side view the chamfers are lines along the length (0.12 mm on paper, under the pen), never cut corners. Same error in plan (`beam-chamfered-top`). The end view chamfer (6 mm = 0.12 mm) is below the 7.5 mm smallest feature.
- 1:50 finger joints: six random two-finger symbols in elevation (one at x 4.40 in the bottom tension lamella at midspan, the worst place), none in the top lamella, while the plan drew two zig-zag joints in the top lamella at 2.6 and 5.9 m. Elevation and plan contradicted each other (a joint goes through the whole lamella), and the plan zig-zag showed a profile on the wide face while the elevation showed it on the narrow face.
- 1:100: hatch spacing 0.8 mm = 80 mm, i.e. every second glue line, so the end view read as 7.5 lamellae. Plates drawn 25 mm (print minimum, stated in the decisions; kept).
- 1:200: plates drawn as 0.18 mm open strokes centred at y -0.32, i.e. a band from 18 mm below ground to 2 mm into the soffit; front `bearing-end` 124 mm wide instead of 160 mm.
- 1:500: one 8.00 x 0.62 block, right for the scale.

Solids (before)
- 1:50 body: 15 full-length lamella extrudes (outer two with the 6 mm chamfer) + plates: no board joints, so the construction of glulam (finger-jointed boards) was absent.
- 1:100 / 1:200 body: one box + plates; 1:100 plates 25 mm and 1:200 plates 30 mm, both reaching 5 and 10 mm into the beam (interpenetration).
- Thermal: the same six skins at every scale, thickness 36 / 40 / 45 / 75 mm only to dodge the copy rule.
- Structure: one block member; pin 12 mm and roller 16 mm, hardly visible; the side skin named `th-side-l` lay on -w, which is the right side in the schema's frame (left is +w).

## 3. Root fixes (catalog, `r3-glulam-beam-roots.py --write`)

| path | old | new |
|---|---|---|
| anchors `bearing-left` x, y | 0, 0 | 0.06, -0.32 |
| anchors `bearing-right` x, y | 8, 0 | 7.94, -0.32 |
| anchors `load-point` y | 0 | 0.30 |
| provenance notes | section, tributary width | plus beam length 8.00 m and span 7.88 m between bearing centres |
| 1:50 elevation `beam-chamfered` (8-point chamfered poly) | cut corners | `beam`, rect 0..8 x -0.30..0.30 |
| 1:50 elevation `finger-joint-0..5` | 6 joints | `finger-joint-00..24`: the 25 joints of the lay-up (kit table), two fingers 20 mm long at 20 mm pitch |
| 1:50 front `beam-chamfered-end` | chamfered poly | `beam-end`, rect -0.08..0.08 x 0.02..0.62 |
| 1:50 plan `beam-chamfered-top` | chamfered poly | `beam-top`, rect |
| 1:50 plan `finger-joint-top-0/1` | zig-zags at 2.60 / 5.90 | straight lines across the width at 2.59 / 5.89 (where the joints of the top lamella meet the top face) |
| 1:100 hatch `grain` spacingMm | 0.8 | 0.4 (= 40 mm lamella) |
| 1:200 elevation `bearing-left/right` | open strokes at y -0.32 | filled rects 0.12 x 0.02 between ground and soffit |
| 1:200 front `bearing-end` | open stroke -0.062..0.062 | filled rect -0.08..0.08 x 0..0.02 |
| decisions 1:50, 1:100, 1:200 | chamfers, 0.8 mm hatch, plate lines | rewritten (en, de) |

Checked: candidate `VALID` (`r3-validate.ts --file`), 2D 1:50 and 1:200 looked at (`r3-2d.ts --file`), forces and moments at `point-load-midspan`: reactions 2.58 kN each, M,max 7.01 kNm (= 2 x 7.88 / 4 + 0.396 x 7.88^2 / 8), overhang moment 0.71 Nm at the bearings (0.396 kN/m x 0.06^2 / 2, physically right).
