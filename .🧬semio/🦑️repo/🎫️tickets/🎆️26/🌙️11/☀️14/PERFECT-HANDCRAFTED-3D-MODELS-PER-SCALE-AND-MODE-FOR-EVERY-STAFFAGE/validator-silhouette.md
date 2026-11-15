# Silhouette maintainer report

Scope: the silhouette side of `🧊️volumes` (raster, strokes, tangent contact, slender solids, `thinnestDrawn`), working beside the thermal maintainer. Scratch module `🧊️volumes-next-b`, scratch tests `🧪️tests/🧊️volumes-rules-b` (removed after the swap). Scripts: `vs-*.ts`, `vs-vitest.config.ts`; output under `🗑️generated/vs/`.

## Queue and result

| # | item | result |
|---|---|---|
| 1 | resolution dependence of section / banded raster | already fixed by swap 14 of the other maintainer (`encloses` asked once per outline on its own probe grid, tile margin 6 features); re-verified here, see numbers; the test `every raster measure is independent of the resolution` (IoU and p95 at 600/900/1200 cells, tiled against whole, sample / section view / unfilled outlines, cross-scale at three cell sizes) already covers every raster measure that is limited (IoU, p95, cross-scale IoU and p95); Hausdorff is information and is compared only tiled against whole |
| 2 | tangent contact | LIVE (swap 16), `Mask.seal` |
| 3 | C24 refined | LIVE (swap 16): follower width by bone and nearest centre line, curved / closed centre line, `volumes-slender` along the centre line |
| 4 | C26 b, c | already LIVE through swap 15 (other maintainer); nothing left on my side |
| 5 | `thinnestDrawn` of a rotated capsule | LIVE (swap 16), `minimumWidth`; union-volume voxel cell: NOT done, proposal below |
| 6 | C23 `retained` and the red silhouette tests | C23 landed with swap 12; the red tests are green since swap 15 (volumes-rules, model-silhouette, asset-validation, see the swap log) and were run again after swap 16 |
| 7 | scissor-lift front p95 | the model / 2D is at fault, not the measure, see below |

## 1. Resolution dependence (re-verified)

retractable-bollard 1:50 elevation, `vs-res.ts`, module `🧊️volumes`:

| cells | 400 | 800 | 1100 | 1400 | rule (cell = feature / 4) |
|---|---|---|---|---|---|
| p95 mm | 4.8 | 2.4 | 1.7 | 1.4 | 1.9 |
| Hausdorff mm | 14.2 | 16.6 | 13.8 | 14.9 | 15 |
| IoU | 0.9947 | 0.9965 | 0.9965 | 0.9965 | 0.9965 |

(before swap 14: 3-6 mm at 300-800 cells, 44-61 mm at 1000-1400, 24 mm at the rule's resolution). wheel-loader and tower-crane have no authored volumes yet, nothing to re-run.

## 2. Tangent contact (swap 16)

Root cause: a solid tessellated a hair inside its true outline (a cylinder is a polygon), tangent to another, leaves a gap of a few millimetres; whether a row of cell centres falls into it is chance, and when one does the whole contact length becomes 3D boundary with no counterpart in the drawing (diesel-generator silencer on the roof: p95 0.3375 m at 1:500, today the asset itself overlaps the solids and reads 0).

Fix: `Mask.fill(rings, true)` remembers triangles, `Mask.seal()` closes a cell that no triangle covers, that is covered on both sides (row, column or either diagonal) and whose disc of an eighth of a cell lies entirely within an eighth of a cell of the triangles all around: the morphological closing by an eighth of a cell (gaps up to a quarter of a cell), nothing else; outer boundary, holes and real gaps are untouched. Used for the 3D masks of `measureView` (solid, upper, whole) and of the cross-scale comparison (`compareGroups`, `viewMasks`).

Test `solids that touch exactly leave no hairline`: roof and bar with a gap of 2 mm (a fifth of a cell, closed: the row of 100 cells is added) and 3 mm (0.3 cell, stays); oracle shapely closing with radius cell / 8 (`closing` case in `🐍️.py`), fixture `tangent`.

## 3. C24 refined (swap 16)

- New `🧵️centreline` (`centreLineOf`, `clusterPoints`): medial axis from the triangles of the solid's own mesh (every surface sample shoots a ray inward and shrinks the touching ball, Ma et al.); a straight solid (all points within three thicknesses of the principal axis) is one segment between its ends, a curved or closed one a tree of cluster centres (minimum spanning tree, gaps cut, three smoothing passes); points are stored in the frame of three vertices so every copy of a `repeat` finds its own line.
- `strokesOf(view, solid, drawn?)`: one stroke quadrilateral per centre-line segment, widened to (a) the width of the 2D shape it follows, found by bone and nearest centre line (mean distance of the projected centre line to the shape's centre line): an open stroke or unfilled outline its `penMm` (else the role pen), a filled strip (length at least three times the width) its own width; (b) its true projected width (90th percentile of the distance of the projected vertices to the centre line, doubled) where the viewpoint draws no shape for the bone or only a non-strip filled shape; (c) without a drawing (callers that pass none) the larger of the role pen and the true width as before.
- `Drawn`, `drawnOf`, `drawnFor` carry the shapes of a viewpoint (bone, closed, filled, rings, pen in metres); `referenceOf(view, solids, section, drawn?)` and the cross-scale comparison (`options.drawnFirst`, `drawnSecond`) take them.
- `slendernessOf`: larger of the extents ratio and (centre-line length over largest cross extent); a hoop of a 20 mm tube 0.5 m from the axis reads 150, was 1.
- Tests (`C24 refined`): five follower cases (own pen, strip width, nearest centre line of the bone, other bone ignored, non-strip ignored) against the hand area (length + width) x width; the centre-line length of a hoop against Pappus (trimesh volume over shapely profile area, within 5 %) and the tube thickness within 20 %; the face-on hoop drawn as an annulus against shapely's ring (within 8 %).

playground-swing, module `🧊️volumes` against `🧊️volumes-next-b` (IoU / p95 mm):

| scale | view | before | after |
|---|---|---|---|
| 1:100 | elevation | 0.972 | 0.996 / 0 |
| 1:100 | front, plan | | 0.931 / 3.8, 0.984 / 7.5 |
| 1:200 | elevation | 0.869 / 7.5 | 0.915 / 7.5 |
| 1:200 | front | 0.965 / 7.5 | 0.997 / 0 |
| 1:200 | plan | 1.000 | 1.000 |
| 1:500 | elevation | 0.982 | 0.982 / 0 |
| 1:500 | front | 0.965 | 0.980 / 0 |
| 1:500 | plan | 0.813 / 18.8 | 1.000 / 0 |

(the asset still carries the thickened parts of author 65; with true-size parts the new width logic draws them at the width of the 2D stroke).

## 5. `thinnestDrawn` of a rotated capsule (swap 16)

`minimumWidth(points)` (rotating calipers over the convex hull) replaces the bounding box in `thinnestDrawn`: the limit of an arm drawn as a capsule at 40 degrees is its 32 mm, not the 0.133 m of the box (office-swivel-chair 1:500). Never larger than the box. Test: rectangle turned 40 degrees and capsule turned 72 degrees against shapely's minimum rotated rectangle.

NOT done: the union-volume voxel cell at most a quarter of the band distance for excavator 1:100 and city-bus 1:100. The cell is capped by the 150 M voxel budget (9 mm for a 15 mm band on the excavator); a quarter (3.75 mm) would need about 2 G voxels. `unionVolumeBandExcess` reads 0 there (the shipped swap 13 result) so no author is blocked. Proposal for the coordinator: a narrow-band voxelisation (only voxels within the band distance of the fine surface, about area x band / cell^3 = 90 M for the excavator at 3.75 mm) or an analytic tolerance of the cell on the band edges; either is a change of the thermal-side `🧮️measures` file, so it is left to the other maintainer.

## 7. scissor-lift front p95 (47 mm against 30 at 1:50, 75 against 60 at 1:100)

The measure is right, the drawing and the model disagree. Script `vs-far.ts`: the drawn silhouette (2D filled shapes) of the front view has two enclosed slits that the 3D silhouette does not have: x = -0.309..-0.281 and +0.279..+0.307 m, y = 0.534..0.997 m, 28 mm wide and 0.46 m tall (3936 cells each), between the a-arm (x -0.37..-0.31) and the b-arm (x -0.28..-0.22) of every scissor stage. In 3D the pivots, pins and the top pins of `a1..a5/b1..b5` span z -0.37..0.37 and close the slit; the 2D front draws no pins there. The boundary of the two slits (986 cells, 6.4 % of the 15446 boundary cells) is 58 mm from the nearest 3D boundary, which puts the 95th percentile on it (reported as 47 mm in the tiled measurement, which caps distances at the 6-feature margin of 45 mm, 58 mm in a single raster). The overlay does not show it because a 28 mm slit is 0.56 mm on the 1:50 paper and both silhouettes are filled. Remedy for the author: either draw the pins and pivots in the 2D front view (or leave a slit in 3D by shortening the pins to the arm width), the model is otherwise sound. No limit is weakened.

Observation for the coordinator: the hole-fill threshold (4 features squared = 900 mm^2 at 1:50) makes a 28 mm slit of 13800 mm^2 count as outline although it is 3.7 features wide; I kept it, it is a real feature on the print.

## Tests

After swap 16, run against the live module: `🧪️tests/🧊️volumes-rules` 110 passed (96 before, 14 added: tangent 3, C24 refined 8, minimum width 3), `🧊️model-silhouette` 16, `✅️asset-validation` 73; `🧊️volume-schema` and `🪤️volume-traps` were started last (logs `🗑️generated/vs/after-swap-16-*.log`). The scratch module and tests were removed after the swap.

Caveat: the centre line costs about 2 s for a 1150-triangle hoop (ray casting and ball shrinking, once per distinct mesh); straight bars take milliseconds.
