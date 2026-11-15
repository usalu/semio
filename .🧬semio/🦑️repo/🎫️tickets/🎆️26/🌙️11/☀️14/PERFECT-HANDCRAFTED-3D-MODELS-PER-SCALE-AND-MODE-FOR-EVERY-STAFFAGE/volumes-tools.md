# Volume authoring tools (`volumes-*`)

## Rulings implemented by `volumes-check.ts` right now (2026-11-14, updated at the end of each tool change)

Engine (since 2026-11-14 21:00): `volumes-check.ts` calls the product `measureVolumes` (and `checkVolumes` for the analytic findings) for every rule, so authors see exactly the catalog validator's numbers; the product has C1-C18 live (`schema-api.md`). The table below describes the OWNED engine, which now only runs behind `--fast` (iteration, about 1-3 s on any asset, approximate numbers, the output says so) and `--owned` / `--compare` (diagnostics, disagreement list at more than 5 %). It is kept, not deleted, because the product validator needs about 5 s CPU on a 40-solid human but 230 s CPU (6 minutes wall under load) on the english-oak: the default run therefore gives the product 60 s (`VOLUMES_PRODUCT_BUDGET_S`) and, when it is over budget, shows the owned full check instead (marked `[o]` with a note; the owned full check of the oak needs about 30 s CPU, the willow 36 s wall). `--product` waits for the product without a limit. Delete the owned engine when the product is fast enough. C17 is in the owned drawn side (an unfilled closed outline that the 3D fills at least half counts as filled); C18 (30 % for body-model regions) is not in the owned engine.

| ruling | state in the check |
|---|---|
| A6 pose | done: silhouette, bounds, cross-scale and containment evaluate `presentation.defaultPose` |
| C1 exposed thermal area | done: faces not buried in another solid of the scale (other base geometry), not in the contact plane of a support/contact anchor, not `sectionFaces` (45 degrees of the bone axis); outermost layer; sampled (seven points per face, at most about 900 points per region) so the number carries a few percent of noise |
| C2 `slender` | done: ratio length / largest cross extent >= 20 checked (`volumes-slender`), exempt from `volumes-feature`, excluded from cross-scale (silhouette comparison keeps them) |
| C3 tolerant cross-scale | done: mismatch within one coarser feature of the other outline does not count; union volume of the coarser body within the finer body eroded/dilated by one feature (first order through the surface area) |
| C4 fixed-quality monotonic | done: triangle counts at chord 2 mm, 16 segments for every radius, per list |
| C5 Hausdorff / strokes | done: open strokes and unfilled outlines widened to their pen |
| C6 support `anchor` | done: one support solid per `structure.supports` anchor, checked against that anchor by id (`supportSolidsOfAnchor`, `supportSolidsExtra`, `supportOffM`) |
| C7 resolution independence | done: tiled raster, cell = a quarter of the smallest feature (coarser only when the occupied area would exceed about 6 million cells: then up to 2/3 of a feature, never more); 2D polygons by non-zero winding; `--fast` uses half a feature |
| C8a/b views, soft masses | done: soft limit max(base, 2 % of the largest extent of the view) when more than 30 % of the declared region area is `foliage`/`fur`/`feather`/`hair` (approximation of "solids of foliage masses": the whole silhouette gets the relaxed limit) |
| C9 limits | done: IoU 0.90 elevation, 0.88 front and plan; 95th percentile of the boundary distance in both directions <= 2 features (elevation), 4 (front, plan); maximum reported (`hausdorffMaxM`), never limited; bounds within max(1 feature, 2 % of the axis) |
| C10 `texture: true` | done: left out of silhouette, cross-scale and `volumes-feature` |
| C11 views disagree | done: `volumes-views-disagree` on the whole-asset extents of the drawings (height elevation/front, length elevation/plan, width front/plan) > max(feature, 2 %); per bone only reported |
| C12 section views | done: where `representations[scale].sections[view]` exists the 3D side is the part of the body at and beyond the plane (triangles clipped at the plane, projected) |
| C13 feature follows the drawing | done: limit of a solid = min(feature, thinnest filled closed 2D shape extent of its bone at that scale) |
| C14 cross-scale relative to the 2D pair | done: IoU limit min(0.85, IoU of the two filled 2D drawings - 0.05), boundary p95 limit max(2 features, 2D p95 + 1 feature); the note of each measurement prints the 2D pair's number |
| C15 thermal area per scale | done: per region 15 % at 1:50, 30 % at 1:100; at 1:200 and 1:500 only the total (`areaDeviationTotal`, 30 %), per region reported; every region still needs a solid |
| B4 hidden regions | done: the faces of a `hidden` region are never buried (they sit inside the body by design) |
| C16 exchange areas | done: regions of model `leaf` or kind foliage/fur/feather/hair print "exchange area, not compared" and are excluded from the area comparison and the total; for other regions a measured/declared ratio outside 1.5 prints a WARNING (no correction is ever suggested) |
| `envelope: true` (C7) | done in cross-scale: the finer scale's solids of an envelope bone are compared as their convex hull per view |

Known approximations: C8b is per asset, not per solid; C12 ignores the "cut faces" distinction (the retained part's projection contains them); the union-volume band is first order; the oracle (`volumes-oracle.ts`) agrees with these numbers on IoU, p95 and buried-only areas within the stated tolerances on the assets listed in `🗑️generated/volumes-tools/oracle-12.txt`.

## Speed (2026-11-14)

- Tessellation is cached by content hash (everything that shapes a mesh, nothing that only labels it) in memory and on disk under `🗑️generated/volumes-tools/cache/tess`; an unchanged solid costs nothing on the next run, a solid that moved or only changed `tone`, `material`, `region`... reuses the mesh; repeat copies are part of the cached mesh.
- `--scale N` and `--rules a,b` do only that work. `--fast` (coarser raster and tessellation; availability, bones, solids, bounds, silhouette, views-disagree and the coverage counts only) is for iteration: a 1:50 willow (700 solids per list) costs about 1 s after the first run; the full check runs without `--fast` (it is what the status table caches). A first run on a cold cache pays the kernel tessellation once (the 1442-solid willow: about 4 s).
- Closed solids drop back-facing triangles from the silhouette fill, empty tiles and tiles without a mismatch cost nothing, nothing is allocated per triangle or pixel.
- `volumes-lib.ts` exports stay stable: this change added `LIMITS`, `quality`, `localGeometry`, `geometryKey`, `meshesAt`, `poseState`, `scaleMeshes(asset, scale, volume)` (the scale is now an argument), `minExtent(asset, scale, solid)`, `RULES` (now with `volumes-views-disagree`, `volumes-slender`, `volumes-copied`).


Run from this ticket folder `T` with the repo's bun: `BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)`. `<asset>` = `category/slug` (e.g. `humans/standing-adult`). A full check of one asset takes 1-5 s, a render about 1 s. All output goes to `T/🗑️generated/volumes-tools/`. Sources live in `T/volumes/<category>/<slug>.json` = `{ "volumes": { "50", "100", "200", "500" } }` (design.md). Nothing touches the catalog except `volumes-insert.py`.

## Commands

| step | command |
|---|---|
| 1 seed | `$BUN volumes-seed.ts <asset> [--force]` |
| 2 draft | `$BUN volumes-draft.ts <asset> --scale N [--list body\|thermal\|structure] [--apply] [--force]` |
| 3 check | `$BUN volumes-check.ts <asset> [--scale N] [--rules volumes-silhouette,...] [--quiet] [--json]` |
| 4 render | `$BUN volumes-render.ts <asset> [--scale N] [--list body\|thermal\|structure] [--az 35] [--el 25] [--no-cut] [--width 1600]` |
| 5 splice | `python3 volumes-insert.py volumes/<asset>.json "<asset 🔣️.json>" [--allow-incomplete]` (coordinator/author when the check passes) |
| 6 status | `python3 volumes-status.py [--category humans] [--json]` |

Typical loop: seed, `volumes-draft.ts --scale N --apply --force` for each of 50/100/200/500 (or draft to a file and copy the good solids by hand), edit the source JSON by hand, `volumes-check.ts`, `volumes-render.ts`, look at `🗑️generated/volumes-tools/render/<category>/<slug>.s<N>.png`, repeat.

## What each tool does

- **seed**: 1:50 `body` = old `asset.volume.solids` visible at 1:50 without `region`, `regions`, `maxDenominator`; 1:50 `thermal` = `{ id: th-<solid>, of, region }` references from the old bindings (a solid bound to several regions gets `th-<region>-<solid>` per region); `structure` empty; coarser authored scales empty-authored (`body/thermal/structure/decisions` all `[]`); scales whose representation is unavailable become `{ status: "unavailable", denominator }`. Never overwrites without `--force`.
- **draft** (proposals only, `{ solids, flags }` written to `🗑️generated/volumes-tools/draft/<category>/<slug>.s<N>.<list>.json`; `--apply` writes the list into the source, needs `--force` if non-empty):
  - body: the previous ticket's drafter run on that scale's own elevation/front/plan shapes (lofts, capsules, extrudes, revolves per bone); flags `needs-author`, `low-confidence-lateral`.
  - thermal: `{ of }` reference to the drafted body solid of each region binding; at 1:50 a region with several layers: outer layer = reference, inner layers = the body solid shrunk inwards by the thickness of the layers outside it (box, ellipsoid, capsule, revolve, loft; other kinds flagged `layer-not-modelled`); layers without `thicknessM` get a thickness from resistance x conductivity when declared, else flagged `layer-without-thickness` (merge and say so in `decisions`, B2); hidden regions, regions without a bound body shape and every `thermal.emitters[]` entry are flagged `needs-author` (B3, B4): add the solid by hand (emitter solids: id contains the emitter id, or tag it).
  - structure: member solids for `memberBones` (B1: the 2D structural view's `eligibleBones` at `dimensions.heightM` plus bones of the scale's 2D `member` shapes), section from the bone `section` (tube from area and second moment when I is larger than a solid disc's), else estimated from mass and length (flag `section-estimated`, check by eye); 2D `member` shapes as bars with their physical section; joint ellipsoids at 1:50 and 1:100; supports from `structure.supports` as pads (flag `support-type`: shape by `SupportType` by hand, B3); mass ellipsoids for bones with at least 10 % of the mass. Members run along the bone's local +x for `lengthM`: for a bone that is a whole sub-assembly (bicycle frame) this is only a placeholder, replace it by the real members.
- **check** (older description; the dated table at the top is current): all rules of the design on the source merged over the asset (no splice needed). One line per rule and scale; `!` marks a failing measurement `name=value<limit` and `[note]` says who. Limits: silhouette IoU >= 0.92 and Hausdorff <= 2 x feature (rule 2; rest pose, three views, 2D filled body/detail/glazing shapes of that scale vs the projected tessellated body; Hausdorff reduced by one raster cell), bbox delta <= feature (3), smallest principal extent of every solid >= feature unless role in `glazing`/`texture` (4), expanded solids and triangles non-increasing with at least one strictly dropping unless decisions say "minimal" (5), cross-scale IoU >= 0.85 / Hausdorff <= 2 x coarser feature / union volume deviation <= 15 % (6), thermal coverage: unknown region/layer, regions without solid, layers without shell at 1:50, emitters without solid, outer-layer surface area vs `areaM2` within 15 % (1:50) / 30 %, thermal solids inside the body box grown by one feature (7), structure coverage: member solid for every `memberBones` bone, supports present, structure inside the body silhouette (>= 90 % of its projected area within one feature), supports on the ground or at their anchor (8), 2D-drawn bones have a body solid (9), per-solid validity (bone, material, tessellates, unique ids, `of` resolves, non-empty lists, `decisions`) (10). Smallest feature = 0.15 mm x denominator (a print profile of the asset overrides). Exit 1 on any failure. Writes the cache `🗑️generated/volumes-tools/check/<category>/<slug>.json` for the status table.
- **render**: sheet of about 1600 x 890 px per authored scale: top row elevation, front, plan overlays (green = in both drawing and 3D body, red = drawn but missing in 3D, blue = in 3D but not drawn; IoU and Hausdorff in the title), bottom row shaded axonometric body, thermal (far half cut away, near half removed, nested layer shells visible, colour per region and layer, legend below, body ghosted) and structure (members blue, joints orange, supports green, masses magenta, body as ghost). `--list` renders one list at the full width.
- **insert**: splices `volumes` where `volume` was (else before `presentation`), removes `volume` in the same write, re-reads the asset right before, verifies nothing else changed and that the member order is intact. Authored scales must have non-empty body, thermal, structure and decisions unless `--allow-incomplete`.
- **status**: table of all 152 assets: per scale three letters (body, thermal, structure: `.` unseeded, `s` empty, `a` has solids, `u` unavailable) and `+` check passes, `!` fails, `?` unchecked or stale; plus check and spliced columns.

## Files

`volumes-lib.ts` (measurements, rules, sources, member bones), `volumes-base.ts` and `volumes-draft-base.ts` (copies of the previous ticket's `volume-lib.ts` / `volume-draft.ts`, so this folder does not depend on the previous ticket), `volumes-raster.ts` (owned rasteriser, bitmap font, PNG via system zlib), `volumes-seed.ts`, `volumes-draft.ts`, `volumes-check.ts`, `volumes-render.ts`, `volumes-insert.py`, `volumes-status.py`.

## Known limits (be aware)

- Silhouette checks use the rest pose of the synthetic rest frames, not `presentation.defaultPose`; the product `checkVolumes` may differ slightly. `schema-api.md` is published; the product module `🗂️catalog/🧊️volumes` did not exist yet when these tools were written, so the measurements are owned (kernel tessellation `tessellateSolid`, owned rasteriser and distance field). Converge on `checkVolumes` once it lands: same rule ids (`volumes-*`).
- No shapely/trimesh oracle here (not installed on this machine); union volume is a ray-parity estimate on a 220-cell grid, areas are triangle sums of the outer layer.
- Hausdorff is measured on a raster of long side / 640; one cell is subtracted. Thin features under one cell are under-sampled.
- Thermal `areaM2` of foliage regions counts both sides of closed leaf solids.
- Structural loaded bones follow B1 (`eligibleBones`); sections without data are estimated.

## Oracle (2026-11-14)

`bun volumes-oracle.ts <asset> [--scale N]` runs `volumes-oracle.py` (shapely, trimesh) through the repo's pinned uv environment (`S/🧪️tests/🧰️support/🐍️.ts` `python(..., "geometry")`, machine cache, hash-locked): per view IoU, maximum and 95th-percentile boundary distance (boundary sampled and measured to the region, interior points of the difference included), per region the buried-only outer area by surface sampling against the other solids, the union volume on a grid (about 1.5 million points) and the bounding box. `DIFF` marks IoU > 0.01, distance > 2 %/2 cells, area > 8 %. Learnings: GEOS `hausdorff_distance` measures to the nearest vertex only and was replaced by an own directed distance; the owned and product `outerArea` (centroid ray test) disagree with surface sampling by 5 to 40 % where solids touch exactly or overlap partly (reported in `schema-requests.md`).

## Insert, status, render (changed)

- `volumes-render.ts`: overlays use the owned silhouette drawn side (pen strokes, non-zero fill), cut faces (`sectionFaces`) hatched in the axonometric panels, `slender` solids yellow in the body panel, coincident faces no longer z-fight (depth bias).
- `volumes-insert.py` copies every field of a solid unchanged (`sectionFaces`, `slender`, `texture`, `envelope`, `anchor`); nothing to configure.
- `volumes-status.py` reads the cache that the full check (without `--fast`, `--scale`, `--rules`) writes.

### Oracle agreement, 12 assets at 1:100 (2026-11-14, `🗑️generated/volumes-tools/oracle-12.txt`, partly complete: the run is slow under load)
Horse, laptop, gas-boiler, glulam-beam and city-bicycle compared so far: union volume within 0.1-2 %, bounding boxes exact, silhouette IoU within 0.002, p95 boundary distance within the stated tolerance, buried-only outer areas within 1-6 % (the owned centroid-in-other-solid area against surface sampling). Open: gas-boiler `sight-glass` and `burner` (owned 0.0000 m2, sampled 0.0040 / 0.0094 m2): the owned ray parity buries them under the casing, trimesh `contains` does not (non-watertight reference); not decided which is right, both are model artifacts of flush or inner parts. Tool changes this round: hidden regions never buried, open skins never bury, view extents include pen-widened strokes, render camera automatic for wall-mounted assets, render overlay shows the validator's IoU and p95.
