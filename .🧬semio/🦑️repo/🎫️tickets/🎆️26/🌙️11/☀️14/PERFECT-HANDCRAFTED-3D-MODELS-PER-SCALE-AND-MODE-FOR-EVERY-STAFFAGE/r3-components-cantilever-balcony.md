# Round 3 — components/cantilever-balcony

State: finished and spliced. `r3-validate.ts components/cantilever-balcony --volumes`: `VALID ... schema 0, validateAsset 0, checkVolumes 0`. `volumes-check.ts components/cantilever-balcony --product`: `PASS` at all four scales (before this round the catalog state failed thermal coverage at 1:50 and 1:100: railing-steel +41 %, wall-face +108 %, thermal-break +56 %).

No earlier round-3 work existed for this asset (no `r3-cantilever-balcony-*` scripts, no partial report, source equal to the catalog); started from the catalog state.

Scripts (all in the ticket folder):

- `r3-cantilever-balcony-kit.py` — the real dimensions and the small writers for solids and shapes.
- `r3-cantilever-balcony-volumes.py` — every solid of every scale and list, stated part by part; writes `volumes/components/cantilever-balcony.json`.
- `r3-cantilever-balcony-roots.py` — root values and the 2D drawings; without flag writes a candidate document for `r3-validate.ts --file` / `r3-2d.ts --file`, with `--apply` re-reads the catalog file and replaces only `provenance`, `dimensions`, `rig`, `representations`, `thermal`, `structure` (refuses when a member is not byte-identical to the plain layout, checks that no other member changed).

## 1. Reference

| item | real value | source |
|---|---|---|
| cantilever slab | 1.5 m projection, 200 mm at the root (typical 180 to 250 mm) | asset rig and section; EN 1992-1-1 |
| drainage | bare concrete balcony drains with a fall; 1.5 % is the usual planning value, giving 177.5 mm at the edge | DIN 18531-5 |
| slab edge | 10 mm arris chamfers (triangular fillet in the formwork); drip groove about 25/15 x 15 mm, 50 to 80 mm behind the edge | construction practice (own knowledge) |
| thermal break | insulation body 120 mm; element heights 160 to 250 mm (180 to 250 with CV50); tension bars in the top layer with cover CV35 or CV50; HTE-Compact concrete pressure bearings at the bottom; diagonal shear bars falling towards the balcony; type K-U connects to a wall below with bars bent down | Schöck Isokorb XT type K / K-U technical information (WebSearch, schoeck.com) |
| wall | 180 mm reinforced concrete, 100 mm mineral wool, 20 mm render (ETICS) | asset thermal layers, EN ISO 6946 |
| reinforcement | 12 mm top bars at 200 mm (about 565 mm2/m for roughly 20 kNm/m), 8 mm cross bars at 250 mm, 8 mm bottom bars, hairpins at the free edge; bend radius 2.5 d to the bar axis (mandrel 4 d) | EN 1992-1-1, own estimate of the design moment |
| railing height | at least 0.90 m, 1.10 m above 12 m fall height; asset has 1.10 m | MBO section 38 |
| guard openings | at most 120 mm clear; no climbable horizontal rails in housing | DIN 18065 |
| handrail | graspable 25 to 60 mm; circular hollow section 60.3 x 2.9, 4.11 kg/m | DIN 18065, EN 10219-2 |
| post | cold-formed square hollow section 50 x 50 x 4: A 6.95 cm2, I 23.7 cm4, 5.45 kg/m, outer corner radius 2 t = 8 mm, inner 4 mm | EN 10219-2 |
| post fixing | top-mounted base plate about 150 x 150 x 12 on a grout bed with four M10 anchors; anchors need roughly 50 to 65 mm edge distance in cracked concrete | anchor approvals (own knowledge, typical values) |
| infill | welded flat-bar chords 40 x 10 with 12 mm round balusters; lower chord at most 120 mm above the floor | construction practice, DIN 18065 |
| loads | barrier load 1 kN/m at the railing top, balcony imposed load 4 kN/m2 | EN 1991-1-1 (asset poses) |

## 2. Audit findings (before this round)

Rig and dimensions

- `railing` bone at x 1.45: the post axis 50 mm from the slab edge with a 100 mm plate reaching the edge exactly; the anchor bolts sat 15 mm from the concrete edge. Not buildable (needs about 65 mm).
- The rig has one railing post per metre strip (one bone, barrier load 1 kN/m on it, mass 6.355 kg = one post), but the 2D front views and all solids showed two posts at +-0.44 m. As a strip of a continuing balcony that gives post spacings of 0.88 / 0.12 m.
- Railing mass 6.355 kg counted only the post; handrail, rail and plates were drawn but weightless. A real baluster railing weighs about 24 kg/m.
- Post section cited EN 10219 but used the sharp-cornered tube (A 7.36 cm2, I 26.15 cm4); the standard values are 6.95 cm2 and 23.7 cm4.
- Slab: dead level bare concrete with a drip groove, no fall; mass 750 kg for the full 200 mm prism.

2D drawings

- Railing was two posts, a 60 mm handrail and one 40 mm mid-rail: openings of about 0.5 m, a climbable rail, not a guard (qa-visual asked for balusters).
- Elevation 1:50: `wall-insulation` 0.12 wide overlapped the render (plan had 0.10); tension bar hook only 115 mm long ending inside the wall; "bottom bar" of the break was a straight bar instead of a pressure bearing; no shear bar; slab top bars horizontal.
- 1:500 front showed the railing only as a rail line although the model had posts.

Solids

- 1:50 body: wall core in two pieces with a seam that does not exist; drip groove square 20 x 20; solid 60 mm handrail bar; sharp-cornered post open at the top; 12 mm bolt cylinders without nuts.
- Thermal: skins 50 mm (the thermal model stores in 80 mm), top and soffit skins marked `+u` (a free face marked as a cut, against C25); wall layers and break referenced from the body without the interior-face marks, so the areas read 2.28 against 1.1 and 0.688 against 0.44.
- Structure: slab block plus bars that stood in the ghost wall; no pressure bearings, no shear bars, bars not crossing the joint as a connection; support a 30 mm steel plate in the slab instead of the wall; a 50 mm cube as "load-carrier joint".
- 1:100 nearly the 1:50 list; 1:500 railing thickened to two 75 mm posts.

## 3. Root fixes (catalog file, outside `volumes`)

| member | old | new |
|---|---|---|
| `rig` railing x, y | 1.45, 0.1 | 1.38, 0.0793 (120 mm inside the edge, on the falling top) |
| `rig` railing mass | 6.355 kg, comX 0.55, literature | 24.11 kg, comX 0.566, authored-estimate from EN 10219 sections (post 5.28, plate 2.12, cap 0.16, bracket 0.09, handrail 4.11, chords 5.97, balusters 6.11, anchors 0.28) |
| `rig` railing section | A 0.000736, I 2.61525e-7 | A 0.000695, I 2.37e-7 (EN 10219-2) |
| `rig` railing title | Railing post | Railing (one post per metre with handrail and baluster infill) |
| `rig` slab mass | 750 kg, com 0.75 / 0 | 706.8 kg, com 0.734 / -0.0054 (section area 0.2827 m2 of the real profile x 2500) |
| `rig` slab section title | 1 m strip, 200 mm slab | 200 mm at the wall (177.5 mm at the edge); section values unchanged (root section) |
| `dimensions.massKg` | 756.36 | 730.91 |
| `thermal` slab-edge | areaM2 0.2, length 0.2 | 0.16, 0.18 (edge face between the chamfers; factor 0.8, inside C16) |
| `thermal` railing-steel | areaM2 0.78 | 0.88 (hand calculation of the new railing: post 0.18, plate 0.05, handrail 0.19, chords 0.19, balusters 0.26, small parts 0.01; factor 1.13, inside C16); title updated |
| `structure.assumptions` | 200 mm strip, post | tapered strip with root stiffness; whole railing mass on the post |
| `provenance` | notes, EN 10219 citation | notes rewritten; EN 10219 citation with the values used; added `schoeck-isokorb-xt-k`, `din-18065`, `din-18531-5` |
| `representations` 50/100/200/500 | see audit | all three views of every scale redrawn as true views of that scale's body (E1); decisions texts `silhouette`, `retained`, `merged`, `omitted`, `thinElements` rewritten in en and de |

Poses, clips, anchors and supports did not need changes (the anchors ride on the bones; `railing-top` is still the bone tip at 1.10 m). Checked: the forces and thermal views of `moving-point-load` at 3 s draw correctly, total weight 7.17 kN = 730.9 kg.

2D shape ids: kept where the part still exists; removed with their parts (`railing-mid-rail`, second post and plate, `break-bar-bottom`, `rebar-top-hook`); new for new parts (grout pad, anchors, cap, bracket, chords, balusters, bearing, shear bar, tension bar, edge hairpin, cross-bar dots, `slab-top-fall`, 1:100 `railing-foot`, 1:200 `railing-infill`, 1:500 `railing-post-line`).

## 4. What is modelled

| scale | body | thermal | structure |
|---|---|---|---|
| 1:50 | 21 | 23 | 18 |
| 1:100 | 12 | 14 | 9 |
| 1:200 | 6 | 7 | 6 |
| 1:500 | 3 | 7 | 4 |

1:50

- Body: slab as one extrusion with fall, chamfers and trapezoid drip groove; break module; wall core in one piece, wool, render; grout bed wedge that levels the fall; base plate 150 x 150 x 12; four hexagon nuts and stud ends; hollow post with rounded corners and real 4 mm wall; 8 mm cap; 20 mm bracket; handrail tube with its 2.9 mm wall, cut at the strip ends; two chords each side of the post; eight balusters at 95 mm clear.
- Thermal: three 80 mm skins as wedges meeting on mitres (no free face is marked; chamfers belong to top and soffit); wall as three nested plates in real thickness, outer face 1.10 m2; thermal bridge as concrete band plus module, 0.44 m2; railing as every steel part, post and handrail as closed bodies (sealed bores, C31). Area deviations: 0.3 %, 1.7 %, 1.6 %, 0.9 %, 0 %, 0 %.
- Structure: slab as the member (reference, it is the member); wall core as the fixed support; connection as joint: five 12 mm tension bars bent into the wall with 30 mm radius and lapped 700 mm, four pressure bearings, four diagonal 8 mm shear bars; slab cage: five top bars along the fall, six cross bars top and bottom, five bottom bars, five edge hairpins; mass ball at the centre of mass; railing member post, cap, bracket, handrail; joint base plate and four anchors 90 mm into the slab.

1:100: plain slab wedge; wool and render one plate; foot block for grout and plate; sharp hollow post with solid head; handrail tube; chords and balusters at true size as slender parts (drawn as pen lines). Thermal with merged wool/render shell. Structure reduced to the force couple (five straight tension bars, one pressure strip), five top bars, railing with foot joint.

1:200: wall as one L mass, break module, slab wedge, solid post and handrail, baluster panel as one texture sheet (drawn as a tone). Structure: slab, wall core, tension layer and pressure layer.

1:500: one L poché mass with level top, one 75 mm post under one 75 mm rail. Structure: slab block, connection block, wall core, post (declared minimal).

Final check line: `PASS`; silhouette IoU elevation / front / plan 0.997 / 1 / 1 (1:50), 0.997 / 1 / 0.998 (1:100), 1.000 / 1 / 1.000 (1:200), 1 / 1 / 1 (1:500).

## 5. What I saw in the final renders

- `r3-sheet` 1:50 body, six views: front shows the baluster panel between two chords under the handrail with the single post on its plate and nuts; right and left side show the falling slab top, the drip groove notch, the chamfered edge, the post standing clearly inside the edge on plate and grout, the open tube end of the handrail; top shows plate with four nuts under the handrail; three-quarter reads at once as a balcony with a guard. No floating parts, no gaps. The slab top appears as a dark band in the front view (grazing light on the fall), which is the render, not geometry.
- `r3-sheet` 1:50 structure, six views (also pose `point-load-tip`): green wall, blue slab, and in the 120 mm gap the tension bars on top, the bearings at the bottom and the diagonal shear bars between; base plate with anchors orange under the blue post and handrail.
- `r3-sheet` 1:50 thermal, cut: the three skins with their mitres, the nested wall plates, the two layers of the thermal bridge, the whole railing in one colour; nothing pokes out.
- `r3-sheet` 1:100 body and structure, six views each: same balcony, simpler; balusters and chords shown yellow (slender); pressure strip and bars in the gap.
- `volumes-render` sheets of all four scales (overlays plus the three lists): overlays all green; 1:200 shows the panel sheet, 1:500 the T of post and rail.
- `r3-2d` of all four scales and of clip `moving-point-load` at 3 s with forces and thermal views: drawings read as section, front and plan of the same object.
- 1:100 thermal, `r3-sheet --cut` six views (resume pass 2026-11-19 23:25): merged wool/render shell beside the concrete, the two thermal-bridge layers in the 120 mm gap, the three slab skins meeting on mitres, the whole railing yellow; nothing pokes out of the body.

## 6. Still imperfect

- The slab reinforcement cage at 1:50 lies inside the opaque slab member and is visible only where it crosses the joint; a viewer of the structure list sees a blue slab unless the member is made transparent. The member of a reinforced slab is the slab, so I kept it.
- The handrail is drawn as a filled circle in the 1:50 and 1:100 section although the model shows the open tube end (wall 2.9 mm is below the pen); this is the 0.3 % the elevation IoU lacks.
- `load-carrier` has no solid any more: it is a massless slider of length 0 and not a member; a block riding on it would float over or sink into the falling slab top.
- The wall has no interior plaster and no floor slab behind it; the asset's thermal layers define the wall as concrete, wool, render, and the connection is the wall type of the break element. The wall reinforcement is drawn in 2D but not modelled in `structure` (the wall is the support solid).
- Structural stiffness still uses the constant 200 mm root section for the tapered slab (stated in the assumptions); the solver has one section per bone.
- The EN 10219 values, the Isokorb family data and the anchor edge distance are typical values from the standard, the manufacturer page and practice, not a design of a specific product.
- `area-decisions.md` lists railing-steel as SET 0.78 and slab-edge 0.2; both changed with the geometry (0.88, 0.16) and need the area reviewers' next pass.

Sources: [Schöck Isokorb XT type K](https://www.schoeck.com/en-gb/isokorb/xt-type-k), [Schöck Isokorb XT type K-U](https://schoeck.com/en/isokorb-xt-type-k-u)

## 7. Resume pass (2026-11-19 23:25)

Started with the brief's resume procedure: `r3-validate.ts components/cantilever-balcony` `VALID` (schema 0, validateAsset 0); `volumes-check.ts --product` `PASS` (7.4 s); the catalog `volumes` are identical to `volumes/components/cantilever-balcony.json`; no broken or half-written state. Re-rendered and opened: `r3-sheet` body 1:50, 1:100, 1:200, 1:500; structure 1:50; thermal `--cut` 1:50 and 1:100; `r3-2d` 1:50. All read as described in section 5; no change needed. Two render-only effects seen: in the `--cut` three-quarter panel a dark triangle on the cut wall top and moire in the plan over the nested wall plates (coincident top faces of the nested shells, the renderer's depth fight, not geometry: front and side cut panels show the plates clean). In the 1:50 plan line drawing the base plate and the post read as one cross-shaped outline, which is the true outline of the two parts in plan.

## Round 5 handcraft (2026-11-21)

Script: `r5-cantilever-balcony-volumes.py` (round-3 script plus the changes below). Renders (opened with Read) under `🗑️generated/r5-cantilever-balcony/before/` and `after/` (made by `r5-c01-sheets.sh`). I opened body, structure and the cut thermal sheets at 1:50, 1:100, 1:200 and 1:500, the 2D at 1:50 and 1:500, and the pose `point-load-tip`. The other poses and the clip only move loads; the solids do not move.

### Reference and model against reference (1:50)

| quantity | reference | model | deviation |
|---|---|---|---|
| slab at the wall / at the edge | 200 / 177.5 mm (1.5 % fall, DIN 18531-5) | 200 / 177.5 mm | 0 % |
| projection | 1.50 m (rig) | 1.50 m | 0 % |
| guard height | at least 1.10 m above 12 m fall height (MBO section 38) | 1.10 m | 0 % |
| clear baluster gap | at most 120 mm (DIN 18065) | 95 mm | in range |
| post | SHS 50 x 50 x 4, outer corner radius 8 mm (EN 10219-2) | 50 x 50 x 4, r 8 | 0 % |
| handrail | CHS 60.3 x 2.9 (EN 10219-2) | 60.3 x 2.9 | 0 % |
| thermal-break module | 120 mm insulation body, height 200 mm (Schöck Isokorb XT K) | 120 x 200 mm | 0 % |
| anchor edge distance | about 65 mm | 65 mm | 0 % |

### Grades (body / thermal / structure)

| scale | before | after |
|---|---|---|
| 1:50 | A- / A- / A- (the cage is hidden inside the opaque slab member) | unchanged |
| 1:100 | A- / A- / A- | unchanged |
| 1:200 | B+ / B / A- (thermal: the wall concrete band in slab height had no solid) | B+ / A- / A- |
| 1:500 | B+ / B / B+ (thermal: same band missing; structure: the rail that carries the barrier load was not a member) | B+ / A- / A- |

### Changes

- At 1:200 and 1:500 the thermal list now has `th-break-core`: the wall concrete in slab height, region `thermal-break`, with the same section faces as at 1:50. It contributes no area.
- At 1:500 the structure now has `railing-handrail-member` (`of` the rail).
- The decisions texts are updated in en and de.
- Checks: `volumes-check --product` PASS (70.5 s). `r3-validate --volumes` VALID (checkVolumes 0). Spliced; `--catalog-only` VALID.

### Remaining

- At 1:200 and 1:500 the guard reads as a post, a rail and a panel or bar, and not as a baluster field. The 2D at these scales draws it the same way.
- In the `--cut` thermal sheets the renderer draws a dark triangle on the wall top and moiré in the plan. These are artefacts of the cut cap and of coincident faces, not geometry.
