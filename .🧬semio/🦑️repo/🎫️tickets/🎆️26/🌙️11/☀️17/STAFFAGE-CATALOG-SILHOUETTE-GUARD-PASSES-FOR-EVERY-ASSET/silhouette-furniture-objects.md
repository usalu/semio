# Silhouette guard: furniture and objects

Scope: `🛋️furniture` and `🪑️objects` of `catalog-silhouette`. All 16 assets that failed at the start (park-bench, picnic-table, charcoal-grill, patio-heater, tealight were already repaired by the earlier ticket) have zero failing checks at every scale (measured with `silhouette-triage.ts current`, which runs the checks of the test). The guard was not loosened; every repair is a value edit or an inserted/replaced single shape through `sil-span.py`/`mvlib.py` (`furniture-objects-lib.py`).

Gates after the edits: `validate-some.ts` 0 issues for all 16 assets; `validate-manifest.ts` shows none of these assets (the manifest entries of folding-deck-chair and market-parasol were updated to the new declared widths); vitest `schema-conformance`, `asset-power`, `section-views`, `volumes-rules`, `three-views`, `fill-rule`, `plan-projection`: no failure on these 16 assets (remaining failures are other categories: vegetation ajv, humans and animals three-views).

## Method findings

1. In front and plan, a shape with a `bone` but without `solid` is placed in world metres only when its bone frame is the identity (`base`, `mast`, `ground`); a world polygon on a rotated bone (`rib-right`) is rotated by the bone. For a world polygon use a fixed bone.
2. Hausdorff clusters usually are enclosed holes or slits: pen strokes (0.35 mm = 17 mm at 1:50) close slits of the drawing that the volume keeps (operating-table pad seams), and the volume keeps thin gaps of hidden members that the drawing never shows (office-swivel-chair armrest bracket in the 2 to 4 mm slit).
3. A volume member that sits above or below the drawing (rail mounts, hanger tab, knuckles, feet) shows as `volume-only`; a drawn decoration above the volume envelope (head fins, driver, scallops) as `drawn-only`.
4. Pens on filled shapes do not change the filled area; pens on open polylines do.
5. New tools in this folder: `furniture-objects-hulls.ts` (convex hull of the projection of volume solids per view, also in bone coordinates; exact for convex members, rails, treads, lofts of convex sections), `furniture-objects-frames.ts` (bone frames), `furniture-objects-diag-fine.ts` and `furniture-objects-fine.sh` (patch with every row; the original skips every second row and hides one-cell slits), `furniture-objects-near.sh` (drawn shapes and volume solids near a point), `furniture-objects-tri.sh` (triage of assets), `furniture-objects-set.py` (set keys of one shape or solid), `furniture-objects-lib.py` (`delete_items`, `hulls`, `hull_of`).

## Repairs (decision per mismatch)

| Asset | Failing before | Cause | Repair |
|---|---|---|---|
| office-swivel-chair | 1:100 and 1:50 plan Hausdorff 0.0488 | the volume bridges seat and armrest pad with a bracket (1:50) and a post box (1:100), the drawing had a slit | drawing: plan shapes `armrest-bracket-l/r` (1:50) and `armrest-post-left/right` (1:100) |
| cafe-chair | 1:100 elevation 0.0302, 1:50 front 0.0203, plan 0.0169 | drawn seat band a full box, volume is pad plus inset apron; drawn leg tops flat, volume has knuckles; drawn stretcher straight, volume slanted | drawing: pad and apron at 1:100, four front knuckle ellipses at 1:50, plan stretchers on the slanted centre line |
| height-adjustable-desk | 5 checks (elevation, front at 1:100 and 1:50) | rail mounts, drive mounts, laptop feet missing in the drawing; cable tray drawn as an open line; hollow rail end seen through | drawing: mounts, drive mounts, laptop feet, filled cable tray; volume: two end caps closing the hollow rail |
| three-seat-sofa | 5 checks | drawn back cushions and plan corners rounded, volume sharp in x; arms rounded all around in the volume only; skirt missing in the plan | drawing: sharp back cushions, 0.09 arm corners (plan, front, elevation), plan skirt |
| folding-deck-chair | 6 checks | canvas drawn as a line against a sagging sheet volume; rear leg drawn from 0.0225 against 0 (1:200, 1:500); 1:50 legs 25 mm short of the ground | drawing: canvas profile polygons (1:50, 1:100), rear legs from 0, 1:500 canvas pen 0.15; volume: 1:50 legs start 20 mm from the bone origin; dimension: widthM 0.787 (the symbol is 0.773, scale extents 0.792 to 0.80) |
| operating-table | 6 checks | drawn pad one slab with joint strokes, volume four pads with 4 mm seams; 1:100 base box against chamfered drawing; 1:50 castors 20 mm inside; 1:200 column box against tapered drawing | drawing: four pads (elevation, plan; plan without extrusion) and no joint strokes; volume: 1:100 base plate loft of 1:50, 1:50 castors at 0.17 m, 1:200 column loft |
| shopping-trolley | 1:100 front IoU 0.913 | basket drawn on the wire centre line, volume 4.5 mm wider; wheels and hubs 10 mm off; chassis pens too fat | drawing: basket 0.2895 and 0.2045, wheels and hubs on the volume positions, pens 0.15 |
| playground-swing | 1:200 elevation IoU 0.915, 1:50 front 0.0303 | leg pen too thin; hanger drawn as an open arc whose enclosed area filled | drawing: leg pen 0.3, hanger collar and tab rectangles |
| stepladder | 6 checks | spreader missing in the 1:200 elevation, 1:200 plan slit, 1:50 capsule spreaders against flat-ended solids, knuckle missing, 1:500 tiny schematic drawing | drawing: 1:200 spreader polygon, foot overlap, 1:50 flat spreaders and knuckles, 1:500 rails, treads, platform and feet as exact projections (hulls); dimension: depthM 0.47 (feet ±0.235) |
| street-lamp | 6 checks | base cover bezier against polyline revolve; head housing, lens and cap bezier against lofts; fins and driver above the housing; cap 13 mm above the declared height | drawing: base cover from the profile, hull polygons for head, lens and cap, fins and driver inside the housing; volume: pole cap compressed to 6.000 m |
| market-parasol | 8 checks | scallops, hem sliver and revolved ribs; stretchers drawn slanted in the front (they are seen edge-on); base plate, mast, crown finial missing at 1:100; canopy bezier | drawing: straight scallops (texture at 1:50), ribs texture in the front, vertical stretcher, base plate, mast, finial, canopy lens from the volume profile (still revolved about the mast, so the furled pose works); dimension: width and depth 2.59 m |

## Files

Assets (all `🔣️.json`): `🛋️furniture/{office-swivel-chair,cafe-chair,height-adjustable-desk,three-seat-sofa,folding-deck-chair,operating-table}`, `🪑️objects/{shopping-trolley,playground-swing,stepladder,street-lamp,market-parasol}`; manifest `🗂️catalog/🔣️.json` (two widths). Scripts: `furniture-objects-<asset>*.py` in this folder. Generated outputs deleted.
