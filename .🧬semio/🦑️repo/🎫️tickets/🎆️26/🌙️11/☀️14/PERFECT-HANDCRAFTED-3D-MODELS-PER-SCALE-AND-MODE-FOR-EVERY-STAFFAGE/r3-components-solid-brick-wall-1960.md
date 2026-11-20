# Round 3 — components/solid-brick-wall-1960

Resume state found on 2026-11-20: only `r3-solid-brick-wall-1960-kit.py` (03:18) existed, no volumes or roots script, no report. Between 03:40 and 04:10 a bulk pass had written a block-bond 1:50 list into the catalog and the source (both identical, 57 / 2 / 2 / 1 body solids). It was kept where real (the bond itself, the `masonry-mortar` material) and replaced where not (see the audit). All four scales and three lists are now stated by my own scripts.

Scripts: `r3-solid-brick-wall-1960-kit.py` (real build-up, bond rule, writers; coordinates now kept to 1e-9), `r3-solid-brick-wall-1960-roots.py` (root fixes outside `volumes`; idempotent; default writes a candidate, `--write` the catalog), `r3-solid-brick-wall-1960-volumes.py` (four scales, three lists each, to `volumes/components/solid-brick-wall-1960.json`). The catalog file is written and spliced (`volumes-insert.py`).

## 1. Reference

| item | real value | source |
|---|---|---|
| Build-up (asset datum) | 15 mm lime-cement plaster, 365 mm solid brick, 20 mm mineral render = 400 mm; U 1.43 W/(m² K) | asset summary, thermal layers, EN ISO 6946 |
| Check of the datum | R = 0.13 + 0.015/0.70 + 0.365/0.75 + 0.02/0.87 + 0.04 = 0.701 m²K/W, U = 1.426 | EN ISO 10456 conductivities |
| Brick format (1960, Germany) | NF 240 x 115 x 71 mm | DIN 105-1 |
| Course height | 1/12 m = 83.3 mm (71 mm brick + 12.3 mm bed joint), 12 courses per metre | DIN 4172 |
| Head / longitudinal joint | 10 mm | DIN 4172 / DIN 1053-1 |
| 36.5 cm wall bond | 1.5-brick block bond: per course a stretcher row (115 deep) and a header row (240 deep) with a 10 mm longitudinal joint (115 + 10 + 240 = 365); the rows change sides course by course; head joints of neighbouring courses offset by a quarter brick (62.5 mm); stretcher joints of every second course in line (block, not cross bond) | DIN 1053-1 bond rules |
| Mortar | 1800 kg/m³, λ 1.0 W/(m K) | EN ISO 10456 Table 3 |
| Mass per m² | 21 + 657 + 34 = 712 kg | thickness x declared density (= `dimensions.massKg`) |
| Centre of mass from room face | (21·0.0075 + 657·0.1975 + 34·0.39) / 712 = **0.20109 m**, at half height | computed |
| Bearing | the brick leaf carries everything; plaster and render are skins | construction practice |

## 2. Audit (state found)

- Rig mass `comX` 0.2 (middle of the 400 mm), source `authored-estimate`: real 0.2011 m (the 20 mm render is heavier than the 15 mm plaster). Rig otherwise right (one fixed bone, length 0.40 = thickness).
- Anchor `base` at x 0.2 without footprint: the bearing is under the leaf, centre 0.1975, 365 x 1000 mm.
- `structure.assumptions[0]`: "self-weight acts at its centre", no number.
- `dimensions` 0.40 x 1 x 1 m, 712 kg and `areaM2` 1.0: correct, unchanged.
- 1:50 elevation: bond lines right, but `joint-1` drawn at y 6 mm on the ground line and the top longitudinal joint run into the top outline. 1:50 plan: longitudinal joint run into both end outlines.
- 2D decisions: 1:100, 1:200 and 1:500 carried the 1:50 sentences ("The 15 mm plaster is 0.3 mm at 1:50", "A diagonal hatch …" at scales that have no hatch, "detail pen … (1:50 only)").
- 1:50 body: bond right; repeat steps and sizes rounded to 5 and 8 digits left a 37 µm gap under the top course (a black depth line in the side views).
- 1:50 thermal: plaster reference without `sectionFaces`.
- 1:50 structure: 55 references, support 7.6 mm placed inside the foot of the wall (v 0 … 0.0076, coincident with the first bed joint, invisible), no mass. 1:100 to 1:500: member + support only, no mass; 1:100 member 380 mm (plaster counted as bearing).
- 1:200 body: masonry 370 mm + a "render skin" of 30 mm, a thickness that does not exist (10 mm of brick given to the render to reach the feature); thermal the same.
- 1:500 structure: member 380 mm.

## 3. Root fixes (catalog file)

| member | old | new |
|---|---|---|
| `rig.bones[root].mass.comX` | 0.2 | 0.201088483 |
| `rig.bones[root].mass.source` / `reference` | authored-estimate / none | literature / en-iso-10456 |
| `anchors[base].x` | 0.2 | 0.1975, + `footprintLengthM` 0.365, `footprintWidthM` 1.0 |
| `structure.assumptions[0]` | weight "at its centre" | 712 kg at the centre of mass 201 mm from the room face, base under the leaf (en, de) |
| `representations.50.geometry.elevation` | 3 rects + 12 bed joints (one on the ground line) + 12 collars | 3 rects + 11 bed joints at the real joint centres + 12 longitudinal joints alternating at u 0.135 / 0.260, top one stopped under the outline |
| `representations.50.geometry.plan` | 3 rects + collar + 4 + 8 head joints | same lines from the kit's bond rule (ids `plan-…`), longitudinal joint stopped inside both end outlines |
| `representations.*.decisions` | 1:50 text repeated at every scale | rewritten per scale (en, de) |
| `materials` | `masonry-mortar` present | unchanged values, restated |

Member order and the two-space formatting are unchanged; only `materials`, `rig`, `anchors`, `representations`, `structure`, `volumes` differ. Poses and clips move no bone (thermal fractions only); nothing else depended on the changed values. The candidate was validated with `r3-validate.ts --file … --volumes` (VALID) before the catalog was written.

## 4. Models per scale

| scale | body | thermal | structure |
|---|---|---|---|
| 1:50 | 57 solids (337 expanded): plaster 15; NF block bond (bed joint x12; per course kind longitudinal joint, 9 header pieces + 8 head joints, 5 stretcher pieces + 4 head joints, cut bricks at both sample ends, repeated x6); render 20 | 3 nested shells innermost first at real thickness: plaster (ref, room face marked C25 c), brick 365 (one homogeneous shell), render (ref) | 57: the bonded leaf itself (every brick and joint by reference, without plaster and render), 7.5 mm fixed bed under the leaf at `base` (below the ground line), mass sphere at (0.2011, 0.5) |
| 1:100 | 2: masonry 380 (plaster merged), render 20 | 2 refs | 3: leaf 365 at its real place, 15 mm bed, mass |
| 1:200 | 1: wall 400 (both skins under the 30 mm feature, none thickened) | 1 ref | 3: leaf 365 inside the block, 30 mm bed, mass |
| 1:500 | 1: wall 400, dark poché tone | 1 ref | 3: leaf 365, 75 mm bed of the same width, mass |

Decisions are bilingual sentences per list at every scale; "Minimal:" declared at 1:100, 1:200 and 1:500.

## 5. Checks (after the last change)

- `volumes-check.ts components/solid-brick-wall-1960 --product`: **PASS**; bounds deviation 0 at every scale (1:50 height was 0.00002 before the precision fix); silhouette IoU 1 / p95 0 in every view; `wall-field` outer area 1.00 m² at every scale; `volumes-copied` 0.
- `r3-validate.ts … --volumes`: `VALID … schema 0, validateAsset 0, checkVolumes 0`; `--catalog-only --volumes`: the same.

## 6. What the renders show (all opened after the last change that touched them)

- 2D 1:50 (`r3-2d`, winter-night and winter-sun): elevation with twelve equal courses, the longitudinal joint changing side each course (stretcher 115 on the room side in the first course), plaster and render strips; front a plain render face; plan with headers on the room side and stretchers outside in the cut top course, head joints offset. Line for line the 3D right-side and top panels. 1:100: hatched 380 leaf + render strip. 1:200: one outlined field. 1:500: one dark field.
- 1:50 body (`r3-sheet`, six panels, and `--only right`): right and left side show the block bond at the two cuts as mirror images, no gap line any more; top shows the cut top course; front and back the plain render and plaster faces; three-quarter reads at once as a rendered 1.5-brick wall.
- 1:50 structure: the bond in blue through the whole leaf, plaster and render as pale ghost strips on both sides, the bed as a thin green line under the foot.
- 1:50 thermal: three reds, plaster dark at the room side, render pale outside, in side, top and three-quarter panels; `--cut` shows the same three shells in the side and top panels (front, back and three-quarter cut panels show the tool's moiré when a slab is cut along its own face).
- 1:100: body two slabs with the render skin visible in side, top and three-quarter; structure the leaf between two ghost strips on a clear green bed; thermal two shells.
- 1:200 and 1:500: one block; structure leaf inside the ghost on a thick green bed (bed and leaf of equal width); thermal one shell.

## 7. Still imperfect

- The mass sphere sits at the real centre of mass inside the opaque leaf and cannot be seen in any render (true position kept rather than moved to be visible).
- 1:200 and 1:500 body are the same single 400 mm block (they differ in tone, bed thickness and decisions only): a homogeneous wall whose skins are 15 and 20 mm has nothing else to show under a 30 mm feature, and inventing a skin thickness was the defect removed here.
- The two coats of plaster and render are not told apart: the declared build-up has no datum for them.
- Brick tone is uniform; flush saw-cut faces show joints by tone only.
- The thermal `--cut` sheet is unreadable from front and back (tool cut artifact, as reported for the sister assets).

State: DONE
