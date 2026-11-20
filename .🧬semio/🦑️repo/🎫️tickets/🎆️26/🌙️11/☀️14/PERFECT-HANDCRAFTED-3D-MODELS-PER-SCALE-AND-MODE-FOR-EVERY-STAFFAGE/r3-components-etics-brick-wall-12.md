# Round 3 — components/etics-brick-wall-12

No earlier round-3 work existed for this asset (no scripts, no report; catalog `volumes` equal to the source; the old state validated but failed `volumes-thermal-coverage` at 1:50 with 2.08 m² against 1.0). The sister asset `etics-brick-wall-30` (same 1960 leaf, 300 mm wool) was already finished in round 3; this asset is the same wall with 120 mm wool, so its scripts are stated on the same real build-up with this wall's own thicknesses, masses and texts.

Scripts: `r3-etics-brick-wall-12-kit.py` (real build-up, bond rule, writers), `r3-etics-brick-wall-12-roots.py` (root fixes outside `volumes`; idempotent; default writes the candidate, `--write` the catalog), `r3-etics-brick-wall-12-volumes.py` (four scales, three lists each, to `volumes/components/etics-brick-wall-12.json`). The catalog file is written and spliced (`volumes-insert.py`).

## 1. Reference

| item | real value | source |
|---|---|---|
| Build-up (asset datum) | 15 mm lime-cement plaster, 365 mm solid brick, 120 mm mineral wool ETICS, 20 mm mineral render = 520 mm; U 0.24 W/(m² K) | asset summary, thermal layers, GEG 2024 Anlage 7 |
| Brick format (1960, Germany) | NF 240 x 115 x 71 mm | DIN 105-1 |
| Course height | 1/12 m = 83.3 mm (71 mm brick + 12.3 mm bed joint), 12 courses per metre | DIN 4172 |
| Head / longitudinal joint | 10 mm | DIN 4172 / DIN 1053-1 |
| 36.5 cm wall bond | 1.5-brick block bond: per course a stretcher row (115 deep) and a header row (240 deep) with a 10 mm longitudinal joint (115 + 10 + 240 = 365); the rows change sides course by course; head joints of neighbouring courses offset by a quarter brick (62.5 mm) | DIN 1053-1 bond rules |
| Mineral wool ETICS boards | 800 x 625 mm, 120 mm thick, stretcher bond, vertical joints offset (here 400 mm), butt-jointed | manufacturer data, EAD 040083 / ETAG 004 |
| Plate anchors | plate 60 mm, shank 8 mm, 50 mm anchorage in solid brick, T scheme, about 4 to 6 per m² | ETAG 014 / EAD 330196 |
| Mortar | 1800 kg/m³, λ 1.0 W/(m K) | EN ISO 10456 Table 3 |
| Steel (anchor pin) | 7800 kg/m³, λ 50 W/(m K) | EN ISO 10456 |
| Mass per m² | 21 + 657 + 7.2 + 34 = 719.2 kg | thickness x declared density (= `dimensions.massKg`) |
| Centre of mass from room face | (21·0.0075 + 657·0.1975 + 7.2·0.44 + 34·0.51) / 719.2 = **0.20915 m** | computed |
| Bearing | the brick leaf carries everything; the ETICS hangs on it (adhesive, anchors) | construction practice |

## 2. Audit (before)

- Rig mass `comX` 0.26 m (middle of the 520 mm): wrong by 51 mm; the wool weighs 7.2 of 719.2 kg. Real 0.2092 m. Rig otherwise right (one fixed bone, length 0.52 = thickness).
- Anchor `base` at x 0.26: under the plaster-side edge of the wool zone, not under the centre of the bearing leaf (0.1975).
- `structure.assumptions[0]`: "self-weight acts at its centre" (not true of a layered wall).
- `dimensions` 0.52 x 1 x 1 m, 719.2 kg: correct, unchanged. `areaM2` 1.0: correct, unchanged.
- 1:50 elevation: 12 bed-joint strokes at 75 mm pitch (13.3 courses per metre; NF is 83.3 mm, 12 courses), material `solid-brick`; no longitudinal joint (the 36.5 cm leaf read as 365 mm long stack-bonded bricks); no board joint. 1:50 plan: four bare rectangles.
- 1:50 body: 13 full-length 365 x 1000 slabs as "courses" (65 mm, odd closers), bed joints coplanar and of brick material, wool one slab: a smooth box (qa-visual grade B).
- 1:50 thermal: plaster and render references without `sectionFaces`: 2.08 m² against 1.0, check FAIL.
- 1:50 structure: 13 course references with gaps at the bed joints, support inside the wall foot (invisible), no mass, no ETICS fixing. 1:100 to 1:500: member + invisible support only.
- 1:500 thermal: one 520 mm solid, the 120 mm wool (1.6 features) lost (qa grade C).
- 2D decisions: every scale carried the slug sentence "A insulated-12 section …"; 1:100 claimed layers under 0.25 mm merge but keeps the 0.2 mm render; 1:500 claimed layers at real thickness but draws one field.

## 3. Root fixes (catalog file)

| member | old | new |
|---|---|---|
| `rig.bones[root].mass.comX` | 0.26 | 0.20915 |
| `rig.bones[root].mass.source` / `reference` | authored-estimate / none | literature / en-iso-10456 |
| `anchors[base].x` | 0.26 | 0.1975, + `footprintLengthM` 0.365, `footprintWidthM` 1.0 |
| `materials` | 4 | + `masonry-mortar`, + `etics-anchor` (both en-iso-10456) |
| `structure.assumptions[0]` | weight "at its centre" | weight of 719 kg at the centre of mass 209 mm from the room face, base under the leaf (en, de) |
| `representations.50.geometry.elevation` | 4 rects + 12 strokes at 75 mm | 4 rects + 11 bed joints at the real joint centres + 12 longitudinal joints alternating at u 0.135 / 0.260 + wool board joint at v 0.40 |
| `representations.50.geometry.plan` | 4 rects | 4 rects + cut top course (headers room side, stretchers outside): longitudinal joint, 8 header joints, 4 stretcher joints, wool board joint at w −0.25 |
| `representations.*.decisions` | generic, partly wrong | rewritten per scale (en, de) |

Member order and the two-space formatting of the file are unchanged; only `materials`, `rig`, `anchors`, `representations`, `structure`, `volumes` differ from the file before. Poses and clips move no bone (thermal only); nothing else depended on the changed values.

## 4. Models per scale

| scale | body | thermal | structure |
|---|---|---|---|
| 1:50 | 61 solids (342 expanded): plaster; NF block bond (bed joint x12; per course kind longitudinal joint, 9 header pieces + 8 head joints, 5 stretcher pieces + 4 head joints, cut bricks at both sample ends, repeated x6); four 800 x 625 x 120 wool boards in stretcher bond; render | 4 nested shells innermost first at real thickness: plaster (ref, `-u` interior face C25c), brick 365, wool 120, render (ref) | leaf member 365 x 1000 x 1000; 7.5 mm fixed bed under the leaf at `base`; mass sphere at (0.2092, 0.5); 4 plate anchors (8 mm shank 50 mm into the brick, 60 mm plate flush with the wool face) as joints of the cladding |
| 1:100 | 3: masonry 380 (plaster merged), wool 120, render 20 | 3 refs | leaf, 15 mm bed, mass, 4 anchors as 8 mm slender shanks |
| 1:200 | 2: masonry 380, ETICS 140 | 2 refs | leaf, 30 mm bed, mass |
| 1:500 | 1: wall 520 (one poché, as drawn) | 2 real shells: masonry 380 + ETICS 140 (the wool is kept) | leaf, 75 mm bed, mass; "Minimal:" declared |

Decisions are bilingual sentences per list at every scale.

## 5. Checks

- `volumes-check.ts components/etics-brick-wall-12 --product`: **PASS (2.9 s)**; silhouette IoU 1 / p95 0 in every view and scale; `wall-field` outer area 1.00 m² at every scale.
- `r3-validate.ts … --volumes`: `VALID … schema 0, validateAsset 0, checkVolumes 0`; `--catalog-only`: VALID. The candidate was validated with `--file` before the catalog was written.

## 6. What the renders show (all opened)

- 2D 1:50 (`r3-2d`, winter-night candidate and winter-sun from the catalog): elevation with twelve equal courses, the longitudinal joint changing side each course, wool with its board joint at 0.40 m, render strip; front a plain render face; plan with headers inside, stretchers outside, head joints and the wool joint. Line for line the same as the 3D right-side and top panels.
- 1:50 body (`r3-sheet`, six directions): right and left side show the block bond at the two cuts (mirror patterns), top shows the cut top course, front and back the plain render and plaster faces, three-quarter reads as masonry with a thin insulation and render skin.
- 1:50 structure: blue leaf, four orange anchors through the ghosted wool (T pattern in front view), green bed as a line at the foot. 1:100: anchors as thin shanks, green bed clearly visible; 1:200 / 1:500 (`volumes-render`): leaf on a thick green bed.
- 1:50 thermal `--cut`: side and top panels show plaster, brick, wool, render in four reds; the front, back and three-quarter cut panels show moiré and a diagonal cut (tool behaviour when a slab is cut along its own face).
- 1:100 body (`r3-sheet`): masonry, wool, render as three slabs; 1:200 two, 1:500 one block with two thermal shells; overlays all green.

## 7. Still imperfect

- The mass sphere lies at the real centre of mass inside the opaque leaf and cannot be seen in any render.
- Board adhesive, reinforcing mesh and the base/finish split of the render are not modelled: the declared build-up (520 mm, U 0.24) has no such layers and no datum for them.
- Anchors appear only in `structure` (hidden in the wool; the thermal region is declared homogeneous).
- Brick tone is uniform; flush saw-cut faces show joints by tone only.
- The thermal cut sheet is hard to read from front and back because of the tool's cut artifact.
