# R2 av: render review, weak spots, plan dimension (OI-87, OI-123, OI-133)

Scripts in `🧰️av/` (`render.ts` renders asset, scale, view and viewpoint, `batch.py` / `zoom.py` crop and rasterise with PyMuPDF). Generated PNGs and SVGs in `🗑️generated/av` are deleted.

## OI-87 render review (dark-line and thermal at 1:50 and 1:100, rasterised and looked at)
decentralised-hrv-fan, desktop-monitor, duct-fan, kitchen-extractor-hood, server-rack, expansion-valve, sleeping-person, person-cooking, seated-office-worker, chalkboard, storage-crate-stack and panel-radiator (reviewed last, side section without wall, 0.71 m): no broken geometry, missing regions or heavy pens. The expansion valve (0.09 m) and the monitor side profile are legible but tiny by nature. Weak spots found only in laptop, chalkboard and gas-hob (below). Catalog check `--asset` passes for all 14 assets touched here.

## OI-123 weak spots (all 1:50)
- **laptop**: root cause of the V shape is the 0.35 mm ground pen (17 mm at 1:50), wider than the 12.5 mm base, which swallowed the base slab. Ground pen 0.35 to 0.18; new `hinge-shoulder` (34 mm by 10.5 mm rounded block, rear of the base, as on real hinge-raised decks); hinge barrel radius 3.6 to 4.8 mm. Real thicknesses kept (lid 6.5 mm, base 12.5 mm), decisions en and de updated. Zoom shows base with shoulder, hinge and lid.
- **chalkboard**: `frame-profile` and `leaf-profile` (inner edge line 12 mm inside the 35 mm frame, on frame and leaf), `tray-lip` (16 mm front lip); chalk and sponge moved up to sit behind the lip. `retained` text en and de updated. The 60 mm tray is only 1.2 mm on paper, so the lip is a hairline; 1:100 not changed (the profile line would be 0.12 mm).
- **gas-hob**: the 40 mm plate is now a 10 mm overhanging rim (r 3 mm) over a 0.28 m housing (new `housing` shape, seam line at 12 mm). Overall height, regions and trivet unchanged. `retained` en and de updated. 1:100 and 1:200 unchanged.

## OI-133 plan dimension label (engine, fixed generally)
Root cause: `dimensionLine` (`🎨️depiction/🏷️annotations/🟦️.ts`) always measured `state.bounds`, the rig bounds of the elevation, whatever the viewpoint, so the plan showed the height (0.09 m) and called it "height". Fix: `dimensionLine(canvas, ctx, extent)` now takes the extent to measure and the quantity follows the viewpoint (`height` for elevation/front, `depth` with label "Overall depth" / "Gesamttiefe" for the plan). New `dimensionExtent` in `🎨️depiction/🟦️.ts`: rig bounds in the elevations, drawn shape extent (paper boxes over `k`) in the plan; no dimension when nothing is drawn. Hob plan now shows 0.52 m.
Test: `🧪️tests/📐️plan-dimension/🟦️.ts` + `🥒️.feature` (jsdom reads the written SVG: data-quantity, value, label text and extension-line span vs catalog `heightM` / `depthM`): 2 passed.

## Test counts
- Staffage vitest (q2 alias config): 56 files, 1120 passed, 11 skipped, 2 failed. Both failures belong to agent ap's concurrent panel-radiator wall removal, not to my files: `export-performance` pinned digest `radiator-transient/0000.svg`, and `front-projection` "radiator-side height expected 0.71 to be greater than 0.83". Pin and expectation need updating by ap.
- Python `staffage_only` guard: 20 tests OK.
- Catalog check: laptop, chalkboard, gas-hob and the 11 reviewed assets all pass.

## Open
- ap: radiator digest pin and `front-projection` side-height expectation.
- Laptop/hob/chalkboard stay hairline-thin at 1:50 by physics; no proportions were exaggerated.
