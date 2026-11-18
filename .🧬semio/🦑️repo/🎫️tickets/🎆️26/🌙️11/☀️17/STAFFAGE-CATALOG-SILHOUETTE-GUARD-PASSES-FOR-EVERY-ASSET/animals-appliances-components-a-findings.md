# Silhouette guard: animals group A (red-fox, mallard-duck, domestic-cat, pigeon, hedgehog, labrador-dog, herring-gull)

All seven assets have zero failing checks of `catalog-silhouette` at every authored scale and none fails `validate-all.ts`. Guard unchanged. Tools of this pass (this folder): `animals-appliances-components-a-t.sh` (in-memory patch trial of the guard checks, ops JSON), `-d.sh` / `-diag.ts` (clusters, ASCII, bins with an ops patch), `-c.sh` / `-contrib.ts` (per drawn shape: cells outside the volume and world extent), `-gen.mjs` (ops interpreter), `-apply.py` (applies the same ops to the asset file: value diff via span edits, plus insert and replace), `animals-appliances-components-a-ops-<asset>.json` (the applied ops).

| Asset | Cause | Side corrected |
|---|---|---|
| red-fox | 1:50 front: extruded body drawing is a box, volume lofts had exponent 2.2 (rounded corners 22 mm off); 1:50 elevation: slit between the two fore legs reached 15 mm higher in the volume | volume `hindbody`, `chest` exponent 8; forearms rx +2 mm in three sections |
| mallard-duck | 1:50 front: breast ellipse 50 mm too high; far foot volume a thin flat loft (near foot splayed); tarsus 5 mm too narrow; far thigh volume 52 mm wide against 28 mm | drawn breast `cy`/`ry`; volume far-foot copies near-foot sections; drawn tarsus rects; volume far-thigh `ry` 0.0145, z 0.052 |
| domestic-cat | 1:50 front: tail-5 ellipse 15 mm above volume (and above the declared height); 1:50 elevation: volume ears left a 2 mm slit 33 mm deep; 1:200 front: drawn feet 4 to 10 mm above ground | drawn tail-5 `ry`/`cy`; ear rx x1.8; 1:200 front foot polygons lowered |
| pigeon | toes drawn 16 mm tall against 8 mm volume; wing primaries drawn 25 mm thick against 22 mm; volume tail root left a 30 mm slit to the body (plan); tarsus 8 mm off in elevation; 1:100 volume body 12 mm lower than the (shared) front drawing; 1:200 legs | drawn toes rects, primaries z range; volume tail root x -0.012; tarsus capsules shifted; drawn 1:100 body `cy`; 1:200 leg capsules +0.5 radius, shifted down |
| hedgehog | 1:50 volume paws below ground (bbox y 0.1445 against 0.128); elevation drawing lacked the visible legs; skirts drawn 14 mm too long and 5 mm too narrow | volume paws lifted; 4 leg capsules added to the 1:50 elevation (new shapes); drawn skirt points compressed x0.85; 1:100 front skirt sweep widened |
| herring-gull | drawn neck-1/neck-2 gap (front) 9 mm: hole above fill threshold; body ellipse 22 mm low and thighs 11 to 24 mm short (front); volume thighs too thin (elevation hole); 1:200 plan hand drawn 18 mm against 33 mm; bbox x 0.6915 against 0.699 | drawn body, neck-2, thighs, hand z range; volume thigh ry x1.2; declared `widthM` 0.699 to 0.692 (drawing and volume agree on 0.6915) |
| labrador-dog | revolved chest drawn circular, volume 20 mm deeper than wide; scapula tops 35 mm low; ears 17 mm wide drawn against 40 mm volume and slit between ear and head; paws, tail, thigh outlines too long or narrow in plan; mouth slit in elevation | chest front shape replaced (ellipse), scapula tops, ear polygons/capsules and volume ear z shift 0.010, paw/tail capsules, thighs replaced by 48-point polygons (followTip), jaw rx x1.15; replicated at 1:100 and the plan items at 1:500 |

## Traps

- An ellipse with `followTip` is invalid (`shape-field`); an ellipse is not placed like a poly: a poly with `followTip` ended 0.0195 m further than the ellipse at the same `cx` (cx 0.0205 ellipse = cx 0.002 poly).
- Relative edits (x1.15, `+= 0.010`) are not idempotent: apply the ops file once; later corrections only with absolute-value ops.
- Hatch ids must exist in the asset (`fur` does not exist in labrador-dog).
- Drawn extruded `solid` z ranges define the footprint in plan and front: widen them to match the volume instead of reshaping the polygon.
- A hole larger than (4 features)^2 cells (a closed slit between ear and head, a 9 mm gap between two neck segments) is a boundary only one side has: close it in the side that is wrong.
- Representation 1:500 of cat and fox is unavailable; do not call the trial with 500 for them.
