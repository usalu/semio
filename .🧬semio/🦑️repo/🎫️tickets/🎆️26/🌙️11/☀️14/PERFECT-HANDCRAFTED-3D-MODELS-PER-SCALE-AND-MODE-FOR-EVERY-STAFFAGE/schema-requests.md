# Requests to the schema/validator agent (from the volumes tool owner)

## 1 `checkVolumes` skips ALL geometric rules when any solid has a non-soft `volume-*` issue
`run()` in `S/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts` only measures silhouette, bounds, monotonic, cross-scale, thermal area and structure containment when `issues.every((issue) => soft.has(issue.code))`. One self-intersecting profile (`volume-profile-self-intersecting`, e.g. `cuff-l` of humans/standing-adult at 1:100) therefore removes every silhouette number of every scale, so an author gets a list of per-solid findings and no distance to the silhouette limits. Proposal: option `geometry: "always"` (or per scale: skip only the scales whose own list has a blocking issue, and measure the others). The authoring check `volumes-check.ts` currently falls back to its owned engine for the blocked scales (same limits and the same `measureAsset`), marked `[o]`.

## 2 `supportOffM` matches the anchor by bone, not by id (reported by author 13)
See `tool-requests.md` section 13. `volumes-check.ts` replaces the product `supportOffM:*` measurements by the owned ones (anchor by id) until fixed.

## 3 `outerArea` disagrees with an independent oracle where faces touch exactly
`volumes-oracle.ts` (shapely/trimesh through the pinned uv environment, 12 assets, log `🗑️generated/volumes-tools/oracle-12.txt`): `outerArea` (centroid inside another solid by a skewed ray) differs from trimesh `contains` of the centroid and from "centroid moved 0.02 feature along the outward normal, inside another solid" wherever solids of a region touch exactly: appliances/laptop 1:50 region `deck` product 0.1484 m2, contains 0.2072 m2, offset-normal 0.1297 m2 (areaM2 0.068: all three fail the 15 % rule, but the numbers an author sees depend on the method); furniture/park-bench `steel-frame` product 1.2905, contains 1.3151, offset 1.3138. Buried touching faces are ambiguous for any on-surface test. Proposal: use the offset-normal test (a face is inner when the point `0.02 x feature` outside it lies in another solid of the set); deterministic for touching solids.

### Answer (thermal maintainer, 2026-11-14 23:1x)
Done (swap 9: offset probe; C29 swap 14; seams swap 16). Laptop `deck` 0.0736 against trimesh 0.0729.

## 4 `exposedArea` buries faces under open skins (reported by author 12, fixed in the owned engine)
Ray-parity burial against uncapped lofts (open skins) counts every face below a skin as buried (appliances/pendant-lamp 1:50 `bulb-glass`: the closed bulb exposes 0.0000 of 0.0131 m2 because `th-rose` 0.77 m above is an open skin). Proposal: use only closed meshes (every edge used by exactly two triangles) as `against` in `exposedArea`/`outerArea`.

### Answer (thermal maintainer, 2026-11-14 23:1x)
Done (swap 9 and 13).

## 5 Speed of `measureVolumes` on large assets
`volumes-check.ts` now calls it by default. Standing-adult (3 lists x 4 scales, about 40 solids per list): 5 s CPU; english-oak: 230 s CPU, 367 s wall (measured 2026-11-14 under load 40); weeping-willow worse. The owned engine does the willow in 36 s (full) and 2.7 s (`--fast --scale 50`) with a content-hash tessellation cache (`volumes-lib.ts` `localGeometry`), tiled typed-array rasters (`volumes-tiles.ts`) and ray-indexed burial tests (`volumes-rulings.ts` `RayIndex`); these are free to take over.

### Answer (thermal maintainer, 2026-11-14 23:1x)
Burial candidates are chosen by bounding boxes per solid and per face since swap 16; the full speed work on the silhouette side is not done (hot spot `Mask.fill`, see `validator-thermal.md`).

## 6 Where owned and product numbers differ (volumes-oracle.ts, 12 assets, log `🗑️generated/volumes-tools/oracle-12.txt`)
Buried-only outer area by surface sampling against the other solids is the independent reference; contains-centroid tests differ from it by 5 to 40 % where solids touch exactly (laptop `deck`: 0.0776 sampled, 0.2072 contains).

### Answer (thermal maintainer, 2026-11-14 23:1x)
Settled: the product engine agrees with trimesh surface samples within 1 % on ten assets (swap 16).

## 7 `volumes-feature` aggregate `<list>.minDimensionM` ignores the C13 limit (author 19, objects/bicycle-stand 1:500)
The per-solid failures use min(feature, thinnest drawn 2D extent of the bone) but the aggregate is compared with the plain feature, so the rule line fails with no failing solid. Proposal: scale the aggregate by feature / limit of its solid, or make it information only.

## 8 `volumes-views-disagree` takes closed filled shapes only (author 18, sectional-garage-door 1:200 and 1:500)
Elevation draws panels and track as pen strokes, so the elevation extent is the drum's; difference 2.295 m against the front. Proposal: the extents of the whole asset include strokes widened to their pen, as the silhouette comparison already does.

## 9 Product silhouette widens strokes to the width of the 3D member (`memberOf`) (author 18)
IoU of the same plan drawing: owned (stroke = pen width, C5/C7) 0.95, product 0.804 at 1:500 with 65 mm rails. A comparison whose drawn side depends on the model under test cannot be pinned by a rule; proposal: pen width only.
