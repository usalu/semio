# Round 3 — components/etics-brick-wall-30

Scripts: `r3-etics-brick-wall-30-kit.py` (real build-up, bond rule, writers), `r3-etics-brick-wall-30-roots.py` (root fixes outside `volumes`; idempotent; `--write` writes the catalog), `r3-etics-brick-wall-30-volumes.py` (all four scales, three lists each, written to `volumes/components/etics-brick-wall-30.json`). The catalog file is spliced (`volumes-insert.py`).

## 1. Reference

| item | real value | source |
|---|---|---|
| Build-up (asset datum) | 15 mm lime-cement plaster, 365 mm solid brick, 300 mm mineral wool ETICS, 20 mm mineral render = 700 mm | asset summary, thermal layers |
| Brick format (1960, Germany) | NF 240 x 115 x 71 mm | DIN 105-1 |
| Course height | 1/12 m = 83.3 mm (71 mm brick + 12.3 mm bed joint), 12 courses per metre | DIN 4172 (Maßordnung im Hochbau) |
| Head / longitudinal joint | 10 mm | DIN 4172 / DIN 1053-1 |
| 36.5 cm wall bond | 1.5-brick block bond: per course one row of stretchers (115 deep) and one row of headers (240 deep) with a 10 mm longitudinal joint (115 + 10 + 240 = 365); the rows change sides course by course; head joints of neighbouring courses offset by a quarter brick (62.5 mm); stretcher courses stack vertically | DIN 1053-1 (Verbandsregeln), standard masonry texts |
| Mineral wool ETICS boards | 800 x 625 mm, laid in stretcher bond, vertical joints offset (here 400 mm), butt-jointed without a gap | manufacturer data (ETICS MW façade boards), EAD 040083 / ETAG 004 |
| Plate anchors | plate 60 mm, shank 8 mm, anchorage about 25 to 50 mm in solid brick, T scheme (at the board T junctions and board centres), about 4 to 6 per m² | ETAG 014 / EAD 330196, system approvals |
| Mortar | lime-cement, 1800 kg/m³, λ 1.0 W/(m K) (cement-sand row; lime-sand 1600 / 0.80) | EN ISO 10456 Table 3 |
| Steel (anchor pin) | 7800 kg/m³, λ 50 W/(m K) | EN ISO 10456 |
| Mass per m² | 21 + 657 + 18 + 34 = 730 kg | layer thickness x declared density |
| Centre of mass from room face | (21·0.0075 + 657·0.1975 + 18·0.53 + 34·0.69) / 730 = **0.2232 m** | computed |
| Bearing | the brick leaf carries everything to the slab; the ETICS hangs on the leaf (adhesive and anchors) | construction practice |

## 2. Audit (before)

- Rig mass `comX` 0.35 m (the middle of the 700 mm thickness): wrong by 127 mm, the wool weighs only 18 of the 730 kg. Real 0.2232 m.
- Anchor `base` at x 0.35 m: in the middle of the wool, not under the bearing leaf. Real: centre of the leaf, 0.1975 m.
- 1:50 elevation: twelve bed joint strokes at a 75 mm pitch (0.075 ... 0.9) = 13.3 courses per m; real NF course 83.3 mm, 12 courses. No longitudinal joint, so the 36.5 cm leaf read as stack-bonded single bricks 365 mm long. No board joint.
- 1:50 plan: four plain layer rectangles; the cut top course and its bond missing.
- 1:50 body: 13 brick "courses" 65 mm high as full-length 365 x 1000 mm slabs (no bricks, no head or longitudinal joints, 70 and 95 mm odd courses to close the metre); bed joints with material `solid-brick`; wool one slab. QA (qa-visual.md): courses invisible, body a smooth box.
- 1:50 thermal: plaster and render as `{ of }` references without `sectionFaces`: outer area 2.08 m² against 1.0 (FAIL).
- 1:50 structure: the 13 courses as `{ of }` members with 10 mm gaps between them (the load path floated over the bed joints); support 7.5 mm inside the body foot, invisible; no mass solid; ETICS fixing absent.
- 1:500 thermal: one 700 mm solid; the 300 mm wool (four features) lost (QA grade C).
- 2D decisions: every scale said "A highly-insulated-30 section …, layers drawn at their real thickness" (a slug in prose; wrong at 1:500 which draws one field); 1:100 claimed layers under 0.25 mm merge but kept the 0.2 mm render.
- `dimensions` (0.7 x 1 x 1 m, 730 kg): correct, unchanged.

## 3. Root fixes (catalog file)

| member | old | new |
|---|---|---|
| `rig.bones[root].mass.comX` | 0.35 | 0.22317 |
| `rig.bones[root].mass.source` / `reference` | authored-estimate / — | literature / en-iso-10456 |
| `anchors[base].x` | 0.35 | 0.1975 (+ `footprintLengthM` 0.365, `footprintWidthM` 1.0) |
| `materials` | 4 | + `masonry-mortar` (1800, 1.0, 1000), + `etics-anchor` (steel pin, 7800, 50, 450), both en-iso-10456 |
| `representations.50.geometry.elevation` | 4 layer rects + 12 strokes at 75 mm | 4 layer rects + 11 bed joints at the real joint centres (k/12 + 6.2 mm; the bottom bed lies on the ground line) + 12 longitudinal joints alternating at u 0.135 / 0.260 + board joint at v 0.40 |
| `representations.50.geometry.plan` | 4 layer rects | 4 layer rects + the cut top course (course 12: headers inside, stretchers outside): longitudinal joint, 8 header joints, 4 stretcher joints, wool board joint at w −0.25 |
| `representations.*.decisions` | generic, partly wrong | rewritten per scale (en, de) |

Strokes that end at the outline stop 4.5 mm short (half the 1:50 detail pen) so the pen cap does not cross the outline. Plan lateral sign verified against the 3D top view (25 mm cut header at w −0.5 and board joint at w −0.25 coincide). Poses have no bone motion (thermal-only poses); 2D checked in `winter-night` and `winter-sun`.

## 4. Models per scale

| scale | body | thermal | structure |
|---|---|---|---|
| 1:50 | 61 solids (342 expanded): plaster; NF block bond — bed joint (x12), per course kind A/B (x6 each): longitudinal joint, 9 header pieces + 8 head joints, 5 stretcher pieces + 4 head joints (bricks at both sample ends cut: 25/90 mm headers, 87.5/152.5 and 212.5/27.5 mm stretchers, no sliver below 25 mm); four 800 x 625 wool boards in stretcher bond; render. Cut faces flush, joints by mortar tone | 4 nested shells innermost first: plaster (ref, `-u` = conditioned interior face C25c), brick 365 (homogeneous, as the layer is declared), wool 300, render (ref, `-u` against the wool, C25b) | leaf member 365 x 1000 x 1000, 7.5 mm fixed bed under the leaf at `base`, mass sphere r 40 mm at (0.2232, 0.5), 4 plate anchors (revolve: 8 mm shank 50 mm into brick, 60 x 8 mm plate flush with wool face) as `joint` |
| 1:100 | 3: masonry 380 (plaster merged), wool 300, render 20 | 3 refs (masonry `-u`) | leaf, 15 mm bed, mass r 50, 4 anchors as 8 mm slender shanks |
| 1:200 | 2: masonry 380, ETICS 320 (render merged) | 2 refs | leaf, 30 mm bed, mass r 60 (anchors dropped) |
| 1:500 | 1: wall 700 (as drawn: one poché) | 2 real shells: masonry 380 (`-u`) + ETICS 320 — the wool is kept | leaf, 75 mm bed, mass r 75; "Minimal:" declared |

## 5. Checks

- `volumes-check.ts components/etics-brick-wall-30 --product`: **PASS** at every scale (silhouette IoU 1.000 / p95 0 mm in all views and scales; thermal area wall-field 1.00 m² at 1:50 and 1:100, total 1.00 at 1:200/1:500; structure inside body, supports on the ground; monotonic; copied none).
- `r3-validate.ts components/etics-brick-wall-30 --volumes`: `VALID … schema 0, validateAsset 0, checkVolumes 0`; `--catalog-only` VALID.

## 6. What the final renders show

- 1:50 body (`r3-sheet`): right side (elevation): plaster sliver, the block bond with alternating short/long bricks and the longitudinal joint changing side every course, twelve equal courses, wool with its board joint at 0.40 m, render — matches the 2D elevation line by line. Left side: the mirror course pattern at the other cut, board joint at 0.40 m. Top: interior headers (25 mm cut piece at one end), longitudinal joint, exterior stretchers, wool joint at w −0.25 — matches the 2D plan. Front and back: plain render and plaster faces, as the real wall. Three-quarter: bricks on top and end cuts read as masonry. The smooth shading of r3-sheet puts a soft dark gradient on the large wool end faces (renderer interpolation, not geometry).
- 1:50 structure: blue leaf, four orange anchors piercing the ghosted wool into the leaf (front: T pattern at the board junctions and centres), green bed visible as a line at the foot; the mass sphere lies inside the leaf (real centre of mass) and is hidden by the opaque member.
- 1:50 thermal (`--cut`): side views show the four shells plaster/brick/wool/render in distinct reds; the front/back/three-quarter cut panels show moiré and a diagonal cut artifact of the tool on a slab cut along its own face (tool behaviour, not model).
- 1:100 / 1:200 / 1:500 (`volumes-render`, r3-sheet 1:100): 3 / 2 / 1 body slabs, overlays all green; 1:500 thermal shows masonry and ETICS shells; supports visible as green beds (1:500 clearly).

## 7. Still imperfect (plainly)

- The board adhesive (about 10 mm bead-and-dab) is not modelled: the asset's declared build-up (and its U value 0.11) has no adhesive layer and totals 700 mm; adding it would change `dimensions` and the thermal data without a datum for this wall.
- The 20 mm render is one solid (base coat with mesh plus finish coat not split: no source for the split of this system).
- Anchors appear only in `structure` (they are inside the wool and invisible in the body, and the thermal region is declared homogeneous: no point thermal bridges).
- Brick tone is uniform; real bricks vary in colour.
- The mass solid is inside the leaf and cannot be seen in opaque renders — that is where the real centre of mass is.


## Round 5 handcraft

No geometry change: the round 3 model already meets the bar; the only change is the sheet tool fix that made the cut thermal panels readable. Zoom tool `r5-c02-zoom.ts`; backup of the source: `🗑️generated/r5-etics-brick-wall-30/volumes-before.json` (identical to the source now).

### Reference vs model (1:50, measured in the source solids)

| item | reference | model | deviation |
|---|---|---|---|
| NF brick | 240 x 115 x 71 mm (DIN 105-1) | headers 240 deep, stretchers 115, 71 high | 0 % |
| course | 1/12 m = 83.33 mm, bed 12.33 mm (DIN 4172) | 12 courses, repeat 83.33, bed 12.33 | 0 % |
| head / longitudinal joint | 10 mm | 10 mm | 0 % |
| 36.5 cm block bond | stretcher row 115 + joint 10 + header row 240, rows change side each course, quarter-brick offset 62.5 mm, header pitch 125, stretcher pitch 250 | as stated (header joints 0.03 + n x 0.125, stretcher joints 0.0925 / 0.2175 + n x 0.25) | 0 % |
| build-up | 15 mm lime-cement plaster, 365 mm solid brick NF in block bond, 300 mm mineral wool, boards 800 x 625, 20 mm mineral render = 700 mm | same thicknesses | 0 % |
| centre of mass from the room face | computed from layer masses | 0.22317 m (730 kg) | 0 % |
| ETICS anchors | plate 60 mm, shank 8 mm, 50 mm into the brick, T scheme (about 6 per m2 on average; this 1 m window holds one board joint line: 2 T junctions + 2 board centres) | 4 anchors as stated | inside |

### Grades (opened: body, structure and thermal `--cut` at 1:50, 1:100, 1:200, 1:500; 2D at every scale; 1:50 in winter-night (default); winter-sun and clip sunrise t 0.5 were rendered but could not be opened (image viewer hook timed out), they move no bone; the poses change the thermal state only)

- Body: A / A / A / A (1:50 / 1:100 / 1:200 / 1:500): 1:50 the bond reads at both cut ends and the top course matches the 2D plan line for line; 1:100 the leaves as slabs; 1:200 masonry and ETICS; 1:500 one crisp slab as drawn. Before = after.
- Thermal: A at every scale (the wool stays a shell of its own down to 1:500); before B as read on the sheets only because of the tool moiré, the model itself was right.
- Structure: A at every scale: the leaf member, the bed under the leaf at the bearing anchor, the mass at the real centre of mass, the plate anchors of the ETICS as joints (1:50, 1:100).
- Solid counts body 61 / 3 / 2 / 1, thermal 4 / 3 / 2 / 2, structure 7 / 7 / 3 / 3.

### Changes

- Thermal `--cut` sheets: the front, back and three-quarter panels showed a dense moiré (round 3 report: "tool cut artifact"). Cause found in the sheet tool: the cut shows the back face of one layer shell and the front face of the next in the same plane, and float noise decided pixel by pixel. Fixed at the root in `r3-lib.ts` (depth tie of 0.01 mm, later solid wins; `tool-requests.md` 2026-11-21); re-rendered, the panels now show clean layers.

### Checks

`volumes-check.ts components/etics-brick-wall-30 --product` PASS; `r3-validate.ts components/etics-brick-wall-30 --volumes` VALID (schema 0, validateAsset 0, checkVolumes 0); catalog `volumes` equal to the source (nothing to splice).

### Remaining (with reason)

- Board adhesive and the base/finish split of the render are not modelled: the declared build-up has no such layers; adding them would change `dimensions` and the thermal layers without a datum.
- Mineral wool density 60 kg/m3 is at the low end of approved ETICS carrier boards (DIBt approvals list about 60 to 400 kg/m3); kept as declared.
- The mass sphere lies at the real centre of mass inside the opaque leaf and is hidden in opaque renders; brick tone is uniform.
