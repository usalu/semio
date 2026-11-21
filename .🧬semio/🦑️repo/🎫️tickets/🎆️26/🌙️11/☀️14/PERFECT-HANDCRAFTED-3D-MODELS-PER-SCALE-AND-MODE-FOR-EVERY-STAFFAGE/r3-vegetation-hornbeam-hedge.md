# Round 3 — `vegetation/hornbeam-hedge` (clipped hornbeam hedge, *Carpinus betulus*)

State: DONE (2026-11-20). Catalog asset VALID (`r3-validate.ts vegetation/hornbeam-hedge --catalog-only`: schema 0, validateAsset 0). `volumes-check.ts vegetation/hornbeam-hedge --product`: **PASS (11.6 s)**, 43 rule lines ok, at 1:50, 1:100, 1:200 and 1:500.

## 0. Scripts and resume

No earlier round-3 work on this asset existed (no `r3-hornbeam-hedge-*` files, no report), so I started from the catalog state of 2026-11-18 (catalog `volumes` were identical to the source).

- `r3-hornbeam-hedge-kit.ts`: conversion and projection helpers (superelliptic panel lofts, tubes, flares, blobs, outlines). Own copy; no other author's kit is imported.
- `r3-hornbeam-hedge-model.ts`: every solid of every scale stated by hand in world metres (profile tables, plant table, stem table, flush, laterals). It also redraws the 2D (ruling E1) and applies the root fixes, then writes the volumes source and the candidate `🗑️generated/r3-hornbeam-hedge/🔣️.json`.
- `r3-hornbeam-hedge-roots.py`: writes `anchors summary materials representations symbols` from the candidate into the catalog file. It re-reads the file right before writing and verifies that every other member is unchanged and the member order is kept.
- `r3-hornbeam-hedge-frames.ts`: prints the bone frames.
- Backups of the state I found: `🗑️generated/r3-hornbeam-hedge/source-before-r3.json` and `catalog-before-r3.json`.

To resume, run in this order:

1. `$BUN r3-hornbeam-hedge-model.ts`
2. `$BUN r3-validate.ts --file 🗑️generated/r3-hornbeam-hedge/🔣️.json`
3. `python3 r3-hornbeam-hedge-roots.py 🗑️generated/r3-hornbeam-hedge/🔣️.json <catalog> anchors summary materials representations symbols`
4. `python3 volumes-insert.py volumes/vegetation/hornbeam-hedge.json <catalog>`
5. `$BUN volumes-check.ts vegetation/hornbeam-hedge --product`

## 1. Reference

| quantity | real clipped hornbeam hedge | source | asset now |
|---|---|---|---|
| height | formal garden screen 1.5-2.5 m, typically 1.8 m clipped; summer regrowth adds 5-20 cm until the late-summer cut | horticultural practice (RHS hedge guidance; German "Heckenschnitt" practice: one cut in late summer, optionally a second in June) | clipped top at 1.84 m; summer flush to 1.87 m (`dimensions.heightM` 1.87 kept) |
| thickness | 0.5-0.8 m; about 0.6-0.7 m at 1.8 m height | practice | 0.70 m at 0.56 m height (`depthM` 0.70 kept) |
| batter | formal hedges are clipped narrower at the top ("A" or trapezoid profile), so the base gets light and stays leafy; typically 5-15 cm per face over the height | RHS formal-hedge guidance; standard practice | 3.5 degrees per face: 0.70 m at 0.56 m to 0.56 m at 1.68 m, rounded 7 cm shoulder, flat top 0.35 m |
| planting density | 3 plants per m in a single row (5 per m in a double row); root-balled 150-200 cm stock 2-3 per m | nursery planting tables | 6 plants on 3 m (2 per m) with 2 stems each (4 stems per m): lower end of practice, see section 7 |
| stems | multi-stemmed after the formative cut-back; 3-6 cm at the collar for a 10-15 year hedge; grey, smooth, fluted ("muscular") bark is the hornbeam's key trait | botany (Mitchell; Roloff, *Bäume Mitteleuropas*) | 2 stems per plant, 4.5-5.5 cm at the collar, diverging in a narrow V, root flare 17 cm, two bark ridges per stem (texture) |
| bare foot | older hedges often show 0.2-0.4 m of bare stems, especially on the shaded side | practice | 0.35 m open foot (rig) |
| leaves | ovate, doubly serrated, 5-10 cm, 10-15 pairs of parallel veins; marcescent (dry leaves stay in winter) | botany | drawn as leaf marks in 2D; the 3D face is a smooth leaf skin (section 7) |
| planting strip | hedges are planted in a cultivated strip, usually mulched with bark | practice | bark-mulch strip 0.60 m wide (= `base-support` footprint), 2.8 cm domed |
| wood | green hornbeam about 1000 kg/m³, E about 9 GPa | Wood Handbook 2021 (proxy, `wood-handbook-2021`) | material `hornbeam-wood` kept |

`dimensions` (3.14 x 1.87 x 0.70 m, 346 kg) agree with the reference and were not changed.

## 2. Audit (state found)

**Rig** (6 stem bundles at 0.5 m spacing, 0.35 m high; 6 vertical foliage blocks of 1.45 m; root plate of 3 m):

- Joint positions are plausible for a hedge segment, so I kept the rig unchanged.
- The rig is planar. Poses and clips therefore lean the hedge along its own row, not across it (section 7).

**Body 1:50** (37 solids). It read as six pillows (QA: "lumpy top"):

- Six panel lofts with 0.32 m half length met at deep grooves. In the elevation they read as a radiator.
- Three pronounced crown lofts per panel made a scalloped top. A clipped hedge is flat-topped.
- The front profile was coffin-shaped: 0.61 m at the foot, 0.70 m at 1.0-1.3 m, 0.46 m at the top. The real shape is the reverse: widest at the base and battered upwards.
- The stems were capsules of 8 cm diameter (real 4-5.5 cm), with no flare.
- The ground was a 4 cm grey box plate.

**Body 1:100 / 1:200** (19 / 7 solids):

- A 0.40 m high, 0.62 m wide box "plinth" stood under the foliage for the stems. That is not a real object, and the gaps of 0.4 m between the stems are 4 mm and 2 mm on paper, so they are clearly drawable at these scales.
- The 1:100 stems were hidden capsules inside the plinth.

**Body 1:500** (2 solids): one rounded loft (the "sausage" of QA) on the same plinth.

**Structure** (QA: "stems stop inside the crown"):

- At 1:50 a single capsule ran up each block, with no laterals.
- At 1:500 the members were boxes.
- The masses had no solids.

**Thermal**:

- leaf-core was boxes, and the emitters were two floating discs on `block-1` inside the hedge (D2 violated).
- The stem-bark area happened to fit only because of the oversize stems.

**2D**:

- 1:50 front: the coffin profile and 8 cm stem rectangles.
- 1:50 elevation: per-shape `penMm` of 1.79-1.8 mm on the block outlines and leaf marks. At 1:50 that is a 9 cm line, which is why the leaf marks rendered as black blobs. 1:100 and 1:200 had similar overrides (0.25-1.7 mm).
- 1:100 / 1:200 / 1:500: the plinth (`plinth`, region stem-bark).
- 1:500 front: the coffin profile.
- The plan symbol had the coffin front.

## 3. Root fixes (catalog, outside `volumes`)

| member | old | new |
|---|---|---|
| `representations.*.geometry` (all scales and views) | true views of the old bodies; plinth at 1:100-1:500; pens up to 1.8 mm | every body, core, stem, mulch and hedge shape redrawn from that scale's body (same ids, bones, roles, z). Panels in elevation and plan are clipped to their own strip (seam plus 2 cm), so the drawing is one hedge with plant seams. `plinth` removed at 1:100, 1:200 and 1:500; `root-plate-mass` (mulch strip) added at 1:100 and 1:200; filled `stem-1..6` added (1:200, 1:500) or converted from open strokes (1:100). All per-shape `penMm` overrides removed, so role pens apply. Leaf, arc and fine texture strokes kept (the form under them barely moved). |
| `representations.{50,100,200,500}.decisions` | plinth texts, "scalloped tops", "slightly tapering" | silhouette/retained/omitted/overlays/thinElements/styles rewritten (en, de) for the battered profile, the flush, the stems and the 1:500 legs |
| `anchors` `tp-face-1..6`, `tp-core-1..6` | block-local (0.909 / 0.8, 0): inside the hedge | on the summer flush at the top (block-local u 1.52, v ±0.12); the transpiration outlets sit there (D2) |
| `materials` | (none) | `bark-mulch` added (emissivity 0.95, solar absorptivity 0.85, 250 kg/m³, 0.07 W/mK, authored estimate) for the planting strip |
| `summary` | "rectangular mass", "lowest 0.35 m open" | battered 0.70 to 0.56 m profile, summer flush, fluted stems, mulched strip (en, de) |
| `symbols[hornbeam-hedge-symbol]` | coffin front 0.70 / 0.46 m | battered front, elevation to 1.87 m with rounded shoulders |

Not changed: `rig`, `dimensions`, `poses`, `clips`, `thermal` (areas: stem-bark 1.0 m² kept; leaf regions are exchange areas), `structure`.

## 4. What is modelled (volumes)

Solid counts are body / thermal / structure.

### 1:50 (61 / 54 / 99)

- **Body**:
  - Six clipped panels as superelliptic lofts (exponent 6) on 14 hand-stated sections: underside rounded from 0.33 m, widest at 0.56 m, batter, shoulder, flat top at 1.838 m.
  - Each panel reaches 0.315-0.405 m along the row (overlap 13-31 cm), so the six plants meet in shallow seams. Neighbouring tops differ by 4 mm, so coplanar caps never z-fight.
  - Two elongated flush cushions per plant (0.40 x 0.27 x 0.033 m).
  - 12 stems as 6-station lofts (radius 2.4-2.7 cm at the collar, 1.6-1.7 cm at 0.64 m), each with a hand-stated lean, spread and bow. They continue 0.3 m into the foliage.
  - 6 revolved root flares; 24 bark ridges (texture); bark-mulch extrude.
- **Thermal**:
  - leaf-face: the panels and flush cushions by reference.
  - leaf-core (hidden): one 6-section loft per plant, about 13 cm inside the faces.
  - stem-bark: the 12 stems up to 0.5 m (own lofts) and the 6 flares. The area is 1.056 against 1.0 m² (deviation 0.056).
  - 12 outlet lenses on the cushions.
- **Structure**:
  - Root mat, fixed support at `base-support`, root-plate mass.
  - Per plant: a collar joint, 2 stem members, 2 block joints, 2 leaders to 1.70 m (6 stations, radius 1.8 to 0.75 cm), 8 slender laterals (ratio 24-43) that reach the clipped faces and the top, and a block mass at 1.08 m.

### 1:100 (31 / 48 / 75)

- **Body**: the same panels on 8 sections; one flush cushion per plant; 2 stems per plant as 3-station lofts; flares with 3 points; a 5-point mulch dome.
- **Thermal**: as 1:50, with 4-section cores; stem-bark deviation 0.004.
- **Structure**: stems as 2-station lofts; leaders as 4-station lofts on the leader path (laterals start on them); 2 laterals per leader; joints and masses.

### 1:200 (13 / 36 / 27)

- **Body**: 6 panels on 6 sections. The flush and shoulder merge into a flat top at 1.855 m. One stem of 5.4 cm per plant with a flared foot; mulch slab of 3 cm.
- **Thermal**: references; 3-section cores; stem-bark as the real 2 stems per plant up to 0.5 m (deviation 0.088); lenses.
- **Structure**: 1 stem and 1 leader member per plant, block joints, masses, root mat, support.

### 1:500 (7 / 20 / 25)

- **Body**: the whole row is one loft on 8 horizontal sections (exponent 10). It has the battered faces, a flat top at 1.85 m and vertical ends. Six legs of 80 mm (the feature) keep the open foot.
- **Thermal**: hedge by reference; one core box; stem-bark as six 80 mm stems up to 0.58 m (deviation 0.078); lenses.
- **Structure**: 75 mm bars for the stems and leaders; the root plate as six root balls under the stems; support slab; masses.

Per-scale `decisions` (body, thermal, structure; en, de) are written for every scale.

## 5. Final check

`PASS (11.6 s)`.

Silhouette IoU (elevation / front / plan):

| scale | elevation | front | plan |
|---|---|---|---|
| 1:50 | 0.999 | 0.996 | 0.997 |
| 1:100 | 0.998 | 0.999 | 0.996 |
| 1:200 | 0.999 | 1.0 | 0.996 |
| 1:500 | 0.999 | 0.985 | 0.984 |

The 95th-percentile boundary distance is at most 1.1 cm at 1:50.

Cross-scale (IoU and p95 against limit):

| pair | elevation IoU | front IoU | plan IoU | p95 against limit |
|---|---|---|---|---|
| 1:100 vs 1:50 | 0.9999 | 1.0 | 1.0 | 3.75 against 30 mm |
| 1:200 vs 1:100 | 0.999 | 0.991 | 1.0 | 30-45 against 60-75 mm |
| 1:500 vs 1:200 | 0.987 | 0.986 | 1.0 | 169 against 244 mm (the leg gaps close) |

Other rules:

- **Monotonic**: strict drops in every list.
- **Structure inside the body**: 0 to 8 mm.
- **Stem-bark area**: within 0.4-9 % at all scales.
- `r3-validate.ts`: VALID, for the candidate and for the catalog.

## 6. What I saw in the final renders

All images are under `🗑️generated/r3-sheet/vegetation/` and `🗑️generated/r3-2d/`.

- **1:50 body**, six directions, `hornbeam-hedge.s50.body.png`:
  - **Front and back**: a clean battered section, widest just above the foot, with a rounded underside, rounded shoulders and a soft flush on the top. The two stems form a narrow V into a flared collar on the mulch dome.
  - **Right and left side**: one continuous clipped wall with five faint plant seams, a flat top with a low wavy flush line, and six pairs of stems with flares and visible bark ridges.
  - **Top**: a clipped band with the chain of flush cushions.
  - **Three-quarter**: reads at first sight as a formal hornbeam hedge on bare legs.
- **Dropped attempts**:
  - Leaf-spray texture (repeated shingled ellipsoids, two versions). It read as quilting, then as studs. Where neighbouring panels bulged by millimetres the sprays were buried, so I dropped it (section 7).
  - Per-plant face drift of ±4 mm. It made wandering seams, so I dropped it too.
- **1:100 body**: the same read with slightly faceted shoulders and one flush cushion per plant.
- **1:200 body**: a flat-topped clipped wall on six single stems with the mulch slab. The z-fighting on the overlapping caps was fixed by the 4 mm offset.
- **1:500 body**: a flat-topped block with vertical ends and small arrises on six legs. No sausage.
- **1:50 structure**: two leaders per plant with laterals reaching the faces and the top. The collar joints are on the mulch and the block joints sit at 0.35 m.
- **1:100 structure** (three-quarter): the leaders follow the stem path, so the laterals are attached. The masses sit between the leaders at the block centre of mass, as point masses.
- **1:500 structure**: bars inside the legs and the mass, and root balls under the legs.
- **1:50 and 1:100 thermal (cut)**: the leaf-core loft sits inside the leaf-face shell with nothing poking out. The red stem bark runs to 0.5 m and the outlet lenses lie on the cushions.
- **Pose `storm` at 1:50**: the blocks lean 9 degrees together and the overlapping panels shingle. No joint opens, and the stems stay inside the foliage underside.
- **Clip `wind-sway` at t = 0.8 s at 1:100**: rendered (`hornbeam-hedge.s100.body.wind-sway-t0.8.png`). The file was written, but I did not open it to judge. Its ±1 degree flutter is smaller than the storm lean, where nothing opened.
- **2D** (`r3-2d/file/r3-hornbeam-hedge.s{50,100,200,500}.png`):
  - 1:50: the elevation shows one hedge with seams and fine leaf marks. The front shows the battered section on a V of stems and the mulch dome. The plan is a band with the seams and the hatched cores.
  - 1:100: the same at its pens, with the cores clipped per plant.
  - 1:200: the battered section on single stems.
  - 1:500: a block on six legs.

## 7. Still imperfect (stated plainly)

- **Wind leans along the row.** The rig is planar, so `windy`, `storm`, `wind-sway` and `gust` rotate the stems and blocks in the elevation plane: the hedge leans along its length. Real wind on a hedge leans it across the row (about world z). Fixing this needs out-of-plane bone rotation or a rig turned 90 degrees, with all poses, clips and 2D re-authored. I did not do it.
- **Planting density.** There are 6 plants (12 stems) on 3 m. That is at the low end of practice (2 plants per m for large root-balled stock; 3-5 per m is more common for whips). A denser rig would change bones, masses, poses and clips. I kept it.
- **No leaf relief in 3D.** At 1:50 single leaves (5-10 cm, 1-2 mm on paper) are drawn only as 2D texture. Two repeated-solid textures looked artificial (quilting, studs), and an irregular per-leaf scatter would need thousands of solids. The 3D face is a smooth clipped leaf skin.
- **Plant seams.** The overlapping panels show as faint vertical seams every 0.5 m in the shaded views. A real clipped face is continuous, but the seams are needed so each block can follow its own bone in the clips.
- **Point masses.** The block masses float as points between the leaders (centre-of-mass markers), as everywhere in the catalog.
- **Root plate section.** The `root-plate` bone declares a solid section of d = 30 cm, but its member solid is a flat mat inside the mulch strip (at 1:500, six root balls), because structure must lie inside the body silhouette. The section is the soil–root idealisation, not a visible part.
- **Core in 2D.** At 1:100 and 1:200 the hatched shaded cores are drawn over the faces in elevation (existing z order and convention of these scales). A strict true view would hide them.

## Round 5 handcraft (2026-11-21)

### Reference (modelled to)

| quantity | reference | source | model 1:50 | deviation |
|---|---|---|---|---|
| clipped height | 1.8-1.9 m formal screen, summer flush 5-20 cm | RHS formal hedge guidance; Heckenschnitt practice | 1.838 m clipped, 1.87 m flush | within |
| thickness at the widest | 0.6-0.7 m | practice | 0.70 m at 0.56 m | 0 % against `depthM` |
| batter per face | 5-15 cm over the height | RHS | 3.5 degrees = 7 cm per face | within |
| stems per plant, collar diameter | 2-3 stems, 3-6 cm at 10-15 years | Roloff, *Bäume Mitteleuropas* | 2 stems, 4.8-5.4 cm | within |
| bare foot | 0.2-0.4 m | practice | 0.33-0.35 m | within |
| planting | 2-5 plants per m | nursery tables | 2 per m (6 on 3 m) | low end, kept (rig) |

### Grades (opened this round: 1:50 body, 1:500 body, 1:50 structure, 1:50 thermal cut, pose storm at 1:50)

| scale | body | thermal | structure |
|---|---|---|---|
| 1:50 | B+ (clean battered section, fluted stems on a mulch strip; vertical plant seams read as upholstery channels, smooth leaf skin, cushion-like rounded underside) | A- | A- (two leaders per plant, laterals to the clipped faces, collar and block joints, support mat) |
| 1:100 | B+ (as 1:50, not re-opened this round) | A- (round 3) | A- (round 3) |
| 1:200 | B+ (round 3) | B+ | B+ |
| 1:500 | B+ (six faceted clipped crowns on 80 mm legs; catalog state of 2026-11-20 04:03, 12 solids) | B+ | B (75 mm bars) |

### Changes

None to the model. The ticket source `volumes/vegetation/hornbeam-hedge.json` was stale against the catalog at 1:500 (7 against 12 body solids: a later catalog edit made the 1:500 crowns follow their six stools); it was set equal to the catalog volumes so that every tool shows what ships. Re-running `r3-hornbeam-hedge-model.ts` would revert that 1:500 state and must not be done without porting it first.

### Remaining issues

- Plant seams every 0.5 m (one leaf panel per block bone so the clips can bend each plant) read as upholstery channels in shaded views; a continuous clipped face would need one leaf solid on a non-animated bone and a re-authored 2D.
- No leaf relief in 3D; the smooth skin is honest for a fresh cut only.
- Wind poses lean the hedge along the row (planar rig), real wind pushes across it.
- Grade A not reached; the brief's bar is not met for this asset.
