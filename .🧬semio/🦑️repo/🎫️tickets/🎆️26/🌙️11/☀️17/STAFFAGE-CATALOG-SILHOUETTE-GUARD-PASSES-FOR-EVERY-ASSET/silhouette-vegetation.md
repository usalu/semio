# Silhouette guard: vegetation (16 assets)

Scope: `🗂️catalog/🌳️vegetation` of `🧊️catalog-silhouette`. The guard was not loosened. Edits are value edits through `sil-span.py`/`mvlib.py` (untouched bytes stay) plus replaced and inserted shapes; the catalog manifest got the new dimensions of 15 entries (value edits). Scripts of this ticket carry the prefix `vegetation-`, the generated work folder `vegetation-work` was deleted after the last run.

## Result

| Asset | Failing checks before | After | Dimensions (m) before → after |
|---|---:|---:|---|
| hornbeam-hedge | 3 | 0 | h 1.88 → 1.87 |
| hosta-clump | 9 | 0 | w 0.88 → 0.876, h 0.62 → 0.618, d 0.905 → 0.901 |
| ivy-on-trellis | 14 | 0 | w 1.91 → 1.92, h 2.11 → 2.105 |
| agave | 16 | 0 | w 2.52 → 2.5, h 1.43 → 1.445, d 2.4 → 2.373 |
| apple-tree | 17 | 0 | w 6.73 → 6.789 |
| weeping-willow | 17 | 0 | w 16.23 → 16.212 |
| japanese-maple | 18 | 0 | w 5.27 → 5.294, h 4.42 → 4.429, d 5.2 → 5.188 |
| small-leaved-lime | 20 | 0 | w 9.06 → 9.005, h 14.19 → 14.167, d 9 → 8.947 |
| london-plane-pollarded | 21 | 0 | w 6.81 → 6.8, h 8.9 → 8.901, d 6.5 → 6.538 |
| multi-stem-birch | 21 | 0 | w 9.26 → 9.205, h 13 → 13.018, d 7 → 6.94 |
| lombardy-poplar | 21 | 0 | w 4.16 → 4.203, h 26 → 25.96, d 4.4 → 4.392 |
| english-oak | 24 | 0 | w 18.35 → 18.348, h 18.2 → 18.226, d 15 → 15.026 |
| miscanthus-grass | 24 | 0 | unchanged (symbols and 1:500 drawings are tied to it by `extent-*`) |
| scots-pine | 27 | 0 | w 12.15 → 12.076, h 22.2 → 22.179, d 8 → 7.907 |
| bamboo-clump | 28 | 0 | w 3.03 → 2.977, h 4.57 → 4.552, d 1.7 → 1.714 |
| date-palm | 34 | 9 | unchanged |

Gates after the work: `validate-all.ts` 0 failing of 152 (all 16 vegetation assets with 0 issues), `validate-manifest.ts` 0 vegetation issues (11 left belong to other categories), vitest `asset-power`, `fill-rule`, `plan-projection`, `section-views`, `volumes-rules`, `volume-traps`, `three-views` and `schema-conformance` pass for every vegetation asset (the failures that remain in those files are other categories: pigeon, horse, jogger, person-cooking, city-bicycle, tour-guide), and `catalog-silhouette` (the official test, including the sharp oracle) for the 15 passing assets: 15 of 15 tests pass (572 s).

## Still failing: date-palm (9 of 34 checks left, `validate-all.ts` 0 issues)

- 1:200: elevation IoU 0.915 (gate 0.92), elevation Hausdorff 0.134 m and front Hausdorff 0.106 m (limit 0.06)
- 1:100: elevation Hausdorff 0.176 m, front 0.123 m, plan 0.185 m (limit 0.03)
- 1:50: elevation Hausdorff 0.214 m, front 0.075 m, plan 0.064 m (limit 0.015)

What was done: the volume fronds (ribbons tilted 7 to 8 degrees, arched 0.1 m, centred 0.1 m beside the bone axis) were laid flat in the (u, w) plane of their bone with the centre on the axis, like the drawn plates (extruded solids that the validator `lateral-ambiguous` forbids to replace by flat shapes), which took the 1:200 plan from IoU 0.887 to 0.956; the stem, root, crown and bunch shapes (missing or 2 times too tall) were inserted or traced; the plate profiles were scaled by a greedy search on the Hausdorff distance of the view (`vegetation-hsearch.ts`: 1:200 elevation 0.435 to 0.134 m, 1:200 plan 0.131 to below the limit, 1:200 front 0.315 to 0.106 m); the 1:50 views are too slow for that search (one iteration is about 15 minutes) and were not searched. What is left is the frond geometry itself: the drawn plates (one flat polygon of 6 points per frond) have no way to follow the arch and the width profile of the volume ribbons in the elevation and the front where the plates are nearly edge-on, and patches on a frond bone are rejected by the validator (`lateral-ambiguous`). The next step is to author the fronds of the date palm again, volume and plate together, from the plate (profile from the ribbon: width profile `rx(u)`, no arch).

## The shared cause (why all 16 failed at every scale)

1. Every canopy, foliage mass and curtain was drawn as a smooth or lobed area (one path or polygon per bone with its own random lumps) while the volume is hundreds of loft lobes with gaps between them and tapered ends. The guard compares boundaries, holes above (4 features)^2 included, at a limit of two features (15 mm at 1:50, 30/60/150 mm at 1:100/1:200/1:500): a 5 cm lobe gap that the area drawing fills (or the reverse) is 3 to 15 limits. Most of the 314 failing checks (Hausdorff and IoU at every scale and view) were of this kind.
2. The declared `dimensions` (one set for four scales) disagreed with the volumes by 1 to 40 cm, and the volumes among each other (the pine depth was 7.54 m at 1:50 and 8.11 m at 1:500) while the guard needs every scale within one feature (7.5 mm at 1:50) of the same number. Roughly one check in five was a bounding box.
3. Smaller recurring causes, each found by the worst boundary cluster and fixed where it sat: hairline seams (one cell, 2 to 4 mm at 1:50) between lofts that the drawing closes or the reverse; 4-connectivity of one-cell channels (a diagonal channel breaks into enclosed holes that the guard fills in one mask and not in the other); `texture`/`hint`/`twig` strokes and closed unfilled outlines (they count as fills when they enclose volume) outside the envelope; `penMm` overrides 2 to 4 times the member (ivy battens drew 0.2 m wide at 1:100, ivy stems 0.09 m at 1:50 against 0.026 m); the wiggly stem polylines of the ivy up to 0.04 m beside the capsules; a bamboo plan ellipse 0.1 m beside its culm with 5 times the radius; a miscanthus plume plate 0.02 m longer than the volume plume; `slender` volume members widened to a filled drawing polygon of aspect 3 or more (the maple limbs became 0.35 m wide strips) and `followTip` shapes that cannot hold more than one ring.

## Decision per mismatch

- Canopy area against volume envelope: the drawing follows the volume (the volume is the 3D body, the drawing its projection). Every drawn mass that disagreed is replaced by the traced outline of its bone's volume projection (even-odd path, holes kept above (2 features)^2, simplified with 0.3 feature, 0.12 and 0.04 feature where a hairline needed it), in the coordinates of the view (elevation: bone-local through the inverse of the bone matrix; front and plan: world metres, refined by Newton steps against the evaluated carry of displaced bones). Foliage outside the envelope is cut to it, strokes outside are cut or removed, extra rings of a `followTip` shape go to inserted parts, slender members get an open line of their own so that the strip rule does not widen them.
- Declared dimension: moved to the volumes where they agree; otherwise the volumes were moved (a section or a whole lobe translated by the exact excess, the pine 1:500 crown scaled to 0.974 in depth) until all four scales sit within 0.25 feature of one value. Miscanthus and the date palm keep their dimension.
- Volume wrong: hairline seams closed (hornbeam crown bases 0.8 to 7.5 mm above the blocks, pine upper bole 20 mm into the lower bole, loft sections inflated by 3/16 feature at the scale where the cracks sit: willow, agave, ivy, plane, pine, miscanthus), the date-palm ribbons laid flat in the plane of the drawn plates.
- Hosta (plates drawn as extruded solids, the validator forbids flat shapes there): the petiole plate of `petiole-l2` widened 15 % in the 1:50 plan and `blade-l4` narrowed 2 % in the 1:100 elevation so that the 4-connected one-cell gaps open and close like those of the volume.

## Method and tools (this folder)

`vegetation-trace.ts` (outline of every bone per scale and view, mismatch per bone and per shape), `vegetation-apply.py` (rewrite, insert, clip, strokes, slender lines, spill), `vegetation-refine.ts` (Newton refinement), `vegetation-cycle.sh`, `vegetation-all.sh` (restore, extents and dimensions, rewrite, fine passes, patches, triage, validate), `vegetation-bbox*`/`vegetation-extent*`/`vegetation-dims*` (dimensions), `vegetation-inflate.py`, `vegetation-fit*`/`vegetation-hsearch*` (profile scales of solid-drawn plates), `vegetation-patch*` (local patches and cuts), `vegetation-manifest.py`, diagnostics (`-ascii`, `-at`, `-cover`, `-holes`, `-row`, `-shape`, `-stroke`, `-solidrow`, `-validate`), per-asset hooks (`vegetation-pre-*`, `-volume-*`, `-fix-*`, `-env-*`). The pipeline restores the original asset and replays everything, so every state above is reproducible with `vegetation-all.sh <slug>`.

## Traps worth keeping

1. `followedWidth`: a filled closed polygon of aspect 3 or more bound to the bone of a slender member widens it to the polygon (strip rule); a dedicated open line along the member (pen width) fixes it, a thin filled quad does not (aspect below 3 is ignored).
2. A closed unfilled shape counts as a fill when it encloses volume; open strokes count with their pen (`penMm` overrides included).
3. Front and plan shapes without `solid` are in world metres only when the bone is at rest; displaced bones carry them (3D rigid carry at the depth of the elevation twin). `followTip` polygons grow with the tip.
4. A one-cell diagonal channel is 4-connected in one mask and not in the other if the polygon is simplified by more than a fraction of a cell; the volume side (inflating the lofts by 3/16 feature) is the stable fix.
5. Tests of the guard run banded for large assets: `silhouette-diag.ts` (unbanded) and the triage differ for the 20 m trees; trust the triage.

## Date palm fixed (0 failing checks, `validateAsset` 0 issues)

Cause found: the fronds were never one body. The plates (extruded profile in the (u, w) plane of the bone, swept along v) and the volume ribbons (loft sections, rotation `[0.707, 0, 0.707, 0]` puts the section x axis on w and y on v) had different profiles, different thickness and tip offsets in v and w; the revolved rachis outlines and the 1:50 `-blade-plug` quads had no body in the volume. The loft is ruled between sections, so the silhouette of a flat ribbon in its plane is exactly the polygon `(u_k, +-rx_k)`.

Fix (`datepalm-all.sh` replays it from the snapshot `datepalm-work/s2.json`, `datepalm-flatten.py` is the core):
1. Every frond: volume ribbon thickness `ry` = 0.52 feature (0.0039, 0.0078, 0.0156 m at 1:50, 1:100, 1:200; the `volumes-feature` floor is 0.5 feature), tip offsets in v and w removed, and the plate path of all three views and three scales generated from the section list (one profile, 144 plates), the extrusion `z0`, `z1` = -+0.25 ry (0.2 to 0.3 passes, 0.4 and up fails 1:50 elevation: the crossing of the two vertical fronds `l4a` and `r4a` forms a wedge slit whose apex moves 0.1 m per 3 mm of plate thickness).
2. The dimensions the offsets held (height 12.5 m, depth 7 m) are kept by lengthening the tip section along u: `fr-l4b` +0.0182/+0.014/+0.03, `fr-r1b` +0.028 (1:100), `fr-l1b` +0.0156 (1:50), +0.028 (1:200).
3. Removed: 240 rachis shapes (revolved solids project with another foreshortening than the plates: 0.1 m beyond the tip), 48 `fr-*-blade-plug*` shapes of 1:50.
4. `stem-3-rings` at 1:50: pen 0.93 mm (4.7 cm) set to 0.222 mm like `stem-2-rings`, ends inside the bark (+-0.2).
Traps: `at` of a loft section outside the plate plane cannot be drawn by a flat plate; revolved detail outlines are projected differently from extruded plates; the apex of a slit between two plates is the most sensitive boundary of the guard.
Tools: `datepalm-axes.ts` (extreme solids per axis and scale), `datepalm-ext.ts` (extent of a solid in the bone frame), `datepalm-rings.ts` (evaluated rings), `datepalm-run.sh`.
