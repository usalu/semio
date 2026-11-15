# Shared authoring kit (`kit-*`) — manual v2

Run from `T` (this ticket folder) with the repo bun: `BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)`. `<asset>` = `category/slug`. Every tool takes `--scale N` (50|100|200|500), reads and writes only `T/volumes/<category>/<slug>.json` through `saveLists` (re-reads the file right before writing and replaces only the lists it names, so another author's other scales survive), builds on `volumes-lib.ts` (poses, meshes, drawn shapes) and `volumes-check.ts` (the one truth for verdicts), and never re-implements a limit. A run takes 1-5 s. Output: `T/🗑️generated/kit/`.

All six pieces exist: `kit-lib`, `kit-fit`, `kit-coarsen`, `kit-thermal`, `kit-structure`, `kit-diagnose`.

## Read this first (coordinator rulings the kit embodies)

- **Everything the kit generates is a first pass.** Before you splice, look at the render sheet of EVERY scale (`volumes-render.ts`, overlays plus the three axonometric views) and hand-correct what looks wrong, even when the numbers pass. A passing check on a crude model is not done.
- Never scale a thermal copy or shrink a region's coverage to hit an area: thermal solids are `{ of }` references to body solids (or real solids where the thermal surface is not a body surface). If the honestly modelled object exposes another area, correct the model or `areaM2` at the root, not the coverage.
- C15: `areaM2` is compared per region at 1:50 and 1:100 only; at 1:200 and 1:500 every region needs a solid and only the total is compared. `areaM2` is corrected at the root only from the 1:50 model.
- C16: regions of thermal model `leaf` and kinds foliage/fur/feather/hair are exchange areas: never propose or change their `areaM2`. For every other region `--apply-area` changes `areaM2` only within a factor 1.5; larger discrepancies are appended unapplied to `T/area-review.md`.
- C18: `areaM2` of body-model regions (skin, clothing, hair, fur, feather of persons and animals) is physiological data: `kit-thermal.ts --apply-area` never touches them (like C16 regions); their tolerance is 30 % at 1:50 and 1:100, and a larger deviation is printed as a `model finding` naming the bound solids and the solids of other regions on the same bones that overlap or bury the surface: fix the model (expose the surface), not the number.
- No copies: every list of a scale is built from that scale's own drawings (`volumes-copied`).

## Which tool when

| you want | run |
|---|---|
| a body for a scale that is empty, or a body that matches that scale's drawings | `$BUN kit-fit.ts <asset> --scale N [--init draft] [--write]` |
| a rigid part (van body, table top, machine casing, trunk) built from the drawings | `$BUN kit-fit.ts <asset> --scale N --hull chassis:u --hull-shapes body-shell` |
| a coarser scale's merged-mass starting point | `$BUN kit-coarsen.ts <asset> --from 50 --to N [--apply --force]` |
| thermal list (nested shells, hidden, emitters), exposed areas | `$BUN kit-thermal.ts <asset> --scale N [--write] [--apply-area]` |
| structure list (members, joints, supports with `anchor`, masses) | `$BUN kit-structure.ts <asset> --scale N [--write]` |
| to know why a scale fails, where, by how much | `$BUN kit-diagnose.ts <asset> --scale N` (then look at the PNG it prints) |
| primitives in your own generator script | `import { loftOf, stationLoft, hullLoft, shellOffset, … } from "./kit-lib.ts"` |

## kit-fit.ts

`kit-fit.ts <asset> --scale N [--solid id,id] [--init source|draft] [--hull bone[:u|v|w],…] [--hull-shapes id,id] [--keep-bone-solids] [--rounds 3] [--steps 0.12,0.06,0.03] [--sections] [--shift] [--exp] [--bounds 0.5,2] [--ridge 2e-5] [--detail] [--write]`

- Start: the body already in the source for that scale (default) or the drafter's proposal (`--init draft`, automatic when the body is empty). It never starts from another scale's list: the only inputs are this scale's own list and this scale's own drawings (rule `volumes-copied` stays satisfiable).
- `--hull bone:axis`: replaces the body-role solids of that bone by a loft whose stations come from the scale's own drawings (union span of the bone's shapes in the two views that contain the axis: elevation + plan for a horizontal axis, elevation + front for a vertical one; Douglas-Peucker reduced to half a feature). Only for bones that are axis-aligned in `presentation.defaultPose`. With several parts on one bone pass `--hull-shapes <2D shape ids>`.
- Fit: coordinate descent maximising the mean IoU of elevation, front and plan (same raster semantics as `volumes-silhouette`, cell <= feature/2) over loft sections (`rx`, `ry`; per section when <= 8 sections or `--sections`; `--shift` also moves section centres in v/w, which is how lateral depth is found; `--exp` also fits the section exponent), ellipsoid radii, capsule `r0`/`r1`/`lateralScale`, box sizes, revolve radius, extrude profile u/v and depth. `--solid a,b` frees only those solids (the others still count in the union). Details stay fixed unless `--detail` or named. Lower/upper bounds `--bounds` stop a solid collapsing; a `1.00 0.50 0.50` multiplier line in the output means it hit the bound: the solid is wrong, replace it by hand.
- Report: IoU per view before/after, the moved solids with their multipliers, and per bone the recall (share of its drawn area covered by any solid) and the precision of each solid (share of its projection inside the drawing) per view; worst first. Precision < 0.9 lists the solid: that is where the lateral depth or the section is wrong.
- `--write` saves only `body` of that scale.

Typical failure causes it finds: a limb whose depth is wrong (blue in plan/front, precision < 0.9), a drawn part with no solid (red, recall < 1), a solid at its bound (`0.50` multiplier) that must be rebuilt.

Measured on delivery-van 1:100 from an empty body: draft + fit IoU 0.973 -> 0.981 (elevation/front/plan 0.988/0.965/0.989) in 0.9 s.

## kit-coarsen.ts

`kit-coarsen.ts <asset> --from 50 --to N [--absorb 0.85] [--small 6] [--rounds 3] [--apply --force]`. Takes your finer body as the vocabulary, not the result: sections are reduced by Douglas-Peucker to half the target feature and floored at 0.55 feature, details that are below two features in every view or at least 85 % covered by the other solids are absorbed (never the last solid of a bone, so `volumes-bones` holds), small same-bone same-material parts are merged into one bounding ellipsoid, then the whole list is refitted to the TARGET scale's own three views, and `compareSolidLists` (rule `volumes-copied`) must say `distinct`. It prints every absorption and merge with its reason and a bilingual `decisions` proposal. Measured on jogger 1:50 -> 1:200: 42 -> 22 solids, IoU 0.923/0.872/0.844 -> 0.959/0.946/0.926 (elevation/front/plan), 1.9 s. Judge it: merged masses are where proportions get lost.

## kit-thermal.ts

`kit-thermal.ts <asset> --scale N [--map region=id+id] [--write] [--apply-area] [--section-faces]`. Binds each body solid to the regions whose drawn shapes (same bone, this scale's drawings) cover at least half of it (old 1:50 seed bindings win; `--map` overrides); 1:50 multi-layer regions become nested shells (`shellOffset` by the real cumulative `thicknessM`, else resistance x conductivity, else 2 mm with a note); hidden regions get a placeholder ellipsoid to move; emitters become tagged jets or discs at their anchors (`directionDeg`: 0 = up, positive towards -x); the product `volumes-thermal-coverage` measures the exposed area (C1) and the table shows exposed vs `areaM2` per region. It prints the old/new `areaM2` corrections it would make (1:50 only, never leaf/foliage/fur/feather/hair or any body-model skin/clothing region (C16, C18), applied only with `--apply-area` and only within 1.5x; larger ones go to `T/area-review.md`) and bilingual `decisions` lines. Thermal list = references plus shells plus emitters, never scaled copies.

## kit-structure.ts

`kit-structure.ts <asset> --scale N [--core 0.7] [--no-joints] [--write]`. Member solids (B1) as cores of the bone's primary body solid (loft sampled at 4/3/2 sections), bone `section` or mass estimate when no body solid, 2D `member` shapes as bars; each member is shrunk until it lies inside the union of the body (<= 2 % of its projection outside; the rest is listed for hand work); joints at 1:50 and 1:100 only; support solids per `structure.supports` anchor with `anchor` (C6) shaped by `SupportType`; mass ellipsoids for bones with >= 10 % of the mass. Sections here are estimates: check them against the bone data and the render.

## kit-diagnose.ts

`kit-diagnose.ts <asset> --scale N [--rules a,b] [--no-png]` runs `volumes-check.ts --json` for the scale and, for every failing rule, localises the offenders:
silhouette (per view: 3D solids with area outside the drawing, bones with drawn area no solid covers, the cell farthest in each direction in mm with its paper position, shapes and solids there), bounds (which solid sets each extreme), feature (solids under the smallest feature), thermal (per region exposed vs `areaM2` and the scale factor that would fix it), structure (uncovered members), plus the raw failing measurements. PNG `T/🗑️generated/kit/diagnose/<category>-<slug>.s<N>.png`: three views, green = both, red = drawn but missing in 3D, blue = 3D but not drawn, yellow rings = worst cells.

## kit-lib.ts (primitives, all pure; each docstring has an `@example`)

| primitive | what |
|---|---|
| `openCtx(asset, scale)` / `saveLists(ctx, { body, thermal, structure, decisions })` | load one scale (source, merged asset, pose state, feature) and write lists back safely |
| `drawn(ctx, view, bone?)` | closed drawn shapes (body, detail, glazing) in the default pose with `bone`, `region`, rings |
| `superellipseSection(at, rx, ry, e)` / `superellipsePoints(rx, ry, e, n)` | a loft section / a polygon for extrude profiles |
| `loftOf(common, sections)` / `stationLoft(common, axis, stations)` | loft through sections / along an axis (`LOFT_AXES` says which of rx, ry is which local axis) |
| `loftFromOutline(common, outline, { axis, across, depth, stations, tol })` | loft from a drawn outline plus a depth function |
| `hullLoft(ctx, bone, axis, common, { pick, stations, e, tol })` | hull loft from the scale's own drawings (what `--hull` calls) |
| `selectStations(stations, tol, fmin)` | Douglas-Peucker on a station list |
| `shellOffset(solid, t, id)` | nested layer shell: the solid inset by `t` (box, ellipsoid, capsule, revolve, extrude, loft), `null` when it vanishes |
| `scaledSolid(solid, k, id)` | cross-section scaled by `k` (area ~ k^0.8) |
| `lobeStack(common, lobes)` | ellipsoid lobes for crowns and foliage |
| `latticeBars(common, nodes, edges, r)` | one capsule per truss/frame bar, `slender` set when 40 r <= length |
| `envelopePrism(common, min, max, corner)` | the box that replaces a lattice at a coarse scale (`envelope: true`) |
| `revolveOf`, `extrudeOf`, `capsuleOf`, `ellipsoidOf`, `boxOf` | plain constructors with the schema's field names |

Frames (A1): coordinates are bone-local, u along the bone, v sagittal normal, w lateral. A loft section without `rotation` lies perpendicular to the path: for a path along u `rx` is lateral (w), `ry` sagittal (v); along v `rx` is u, `ry` is w; along w `rx` is u, `ry` is v.

## Rules of thumb the tools already encode

- Spokes, slats, grilles drawn as `texture` do not belong in `body` (not silhouette-relevant; use `slender`/`texture: true`, C2/C10).
- Smallest radius is 0.55 feature (7.5/15/30/75 mm x 0.55); a fit never goes below.
- Emitters need `tags: [<emitter id>]`; thin details (fingers, zips) stay out of layered thermal regions.

## Self-test and end-to-end proof

- `$BUN kit-selftest.ts` runs every primitive's documented example through the kernel (13 of 13 tessellate).
- `python3 kit-prove.py <asset>` runs fit 1:50 -> coarsen 100/200/500 -> thermal -> structure -> decisions -> check on a scratch copy (`🗑️generated/kit/src/<slug>.json`, never your source) in 10-30 s; output in `🗑️generated/kit/prove-<slug>.txt`.

Measured first passes (no hand work, nothing spliced, nothing written to `T/volumes`):

| asset | result of the generated first pass |
|---|---|
| humans/jogger | 1:50 and 1:100 silhouette, bounds, feature, bones, monotonic, cross-scale pass; 1:200 passes every rule; 1:500 elevation IoU 0.79-0.84 (limit 0.9), structure members under 75 mm, structure triangles not dropping. Open at 1:50/1:100: torso-clothing exposes 0.44 m2 of 0.783 (model finding, C18: the shirt is buried/covered, expose it), structure solids (joints, hands) outside the body in front/plan (0.05 m outside, listed by `kit-structure.ts`). |
| vehicles/delivery-van | silhouette IoU elevation/front/plan 0.99/0.96/0.99 at 1:50, 0.988/0.963/0.995 at 1:100 from the generic drafter plus `kit-fit` (front p95 0.05 m vs 0.015 limit at 1:50 is the cab front, needs `--hull`/hand work); thermal bonnet area 1.0 off (no body solid exposes it), body-panels 0.71 at 1:100, structure members stick out of the body in elevation by 0.03-0.04 m; each is localised by `kit-diagnose.ts`. |
| vegetation/scots-pine | not finished: 330 body solids make the per-solid descent take minutes under the present machine load; run `kit-fit.ts --solid <ids>` for the crown lobes and trunk only, `--rounds 1`, and use `lobeStack` for the crown at 1:100-1:500. |

What these runs taught (tool behaviour authors should know):
- The first pass gets silhouettes close but never replaces judgement: thermal binding of overlapping regions (clothing over skin) is one best region per solid from 1:100 on, which hides skin; structure cores are 0.7 of the body section and joints sit at bone origins that can lie outside thin limbs.
- Thickening below-feature parts: a flat loft ring (shorts hem) is absorbed into its bone's other solids rather than inflated.
- `kit-diagnose.ts` plus the PNG is the fastest way to see which of these is the problem; fix by hand or with `kit-fit.ts --solid`.
