# Silhouette guard: animals, appliances, components

Scope: `🐕️animals`, `🔌️appliances`, `🧱️components` (34 failing assets at start). Guard `catalog-silhouette` was not loosened; every fix is a value edit through `sil-span.py` / `sil-edit.py` (formatting of the asset files is kept).

## Result

14 of 34 assets now pass every check: server-rack, circulation-pump, electric-kettle, split-ac-indoor-unit, electric-fan-heater, hot-water-cylinder, laptop, vacuum-cleaner, thermostatic-radiator-valve, desktop-monitor, cantilever-balcony, curtain-wall-bay, steel-column, wheelchair-ramp. 20 remain (below).

Verification: `validate-all.ts` 1 failing of 152 (`🚜️vehicles/electric-kick-scooter`, shape `fork-crown` lacks `tone`; not one of these categories, not touched); `validate-manifest.ts` 0 issues; vitest `asset-power`, `three-views`, `fill-rule`, `plan-projection`, `section-views`: 5 files, 403 tests passed; `schema-conformance`: 42 of 43 pass, the one failure is the same scooter shape.

## Fixes and causes

| Asset | Cause | Side corrected |
|---|---|---|
| server-rack 1:50 front | volume base rail 1.0 m deep (z -0.5), drawn cabinet 0.975 m; drawn door edge stopped below the door header | volume rail 0.975 m at z 0.0125; drawn door edge height to 2.0 m |
| circulation-pump 1:50 front | terminal box drawn 5 mm off the volume; 4.5 mm gap between motor head and terminal box that the volume bridges with the neck | drawn box x -0.05, new `terminal-neck` rect |
| electric-kettle 1:50 front | spout (0.223 m high) bridges the 3 mm lid lift gap in the volume, the drawing had no front spout | drawn front `spout` rect added |
| split-ac-indoor-unit 1:50 front | drawn corner radius 0.045 m, the volume is a straight extrusion | drawn radius 0.03 m |
| electric-fan-heater 1:50 elevation | handle opening is a hole in the volume, drawn only as a stroke (the 12 mm stroke filled the hole) | hole as second subpath of the handle path, stroke removed |
| vacuum-cleaner 1:50, 1:500 elevation | same handle opening; 1:500 drawn hose radius 0.03 m against volume 0.0375 m | hole ring added, hose, wand and grip radii 0.0375 m |
| hot-water-cylinder 1:50 front | feet drawn at x 0.18..0.25 (wrong axis) instead of four 0.07 m cubes at z ±0.12 | drawn front feet at z 0.085..0.155 and -0.155..-0.085 |
| laptop 1:100, 1:50 elevation | strokes (screen line, bezel, keys) thicker than the 6.5 mm lid / 1.2 mm keys, drawn front bevel the volume lacks | strokes removed or turned into a filled key rect, base-upper without bevel |
| thermostatic-radiator-valve 1:100 front, plan | drawn knob stepped and 3.5 mm short, volume knob a single r 0.03 cylinder to z -0.0745 | drawn head polygons follow the volume |
| desktop-monitor 1:200 front, 1:100 elevation, 1:50 plan | volume panel lofts with superellipse exponent 3 to 4 (round corners) against rectangular drawn panel; slab thickness 0.0105 against 0.0152 | volume exponents 10, drawn slab 0.0152, drawn plan housing tapered polygon (passes at 0.0150 m, the limit) |
| cantilever-balcony 1:50 elevation | drip groove notch (20 x 20 mm) in the volume, drawn slab a plain rectangle | drawn slab polygon with the notch, groove stroke removed |
| curtain-wall-bay 1:200 plan (section B-B) | the visible bottom transom beyond the cut plane missing in the drawing | `transom-top` as at 1:100 |
| steel-column 1:50 elevation, front | splice mark stroke 0.14 m wide beyond the 0.2 m flanges | splice-mark removed |
| wheelchair-ramp 1:100, 1:50 | handrail start cap 0.0212 m below x 1.2 (bbox 10.5212 m against 10.5 m), returns drawn as 9 mm strokes against 42 mm tubes | rail ends moved in (start 0.015, end 9.285), returns drawn 0.85 mm pen at the volume axis, plan capsules shortened |

## Remaining (exact failing checks, scale, view, value against limit)

Components
- shower-head (appliances): 1:200 elevation IoU 0.905 (limit 0.92); thin hose and handle strokes differ by half a cell along the whole length; hose pen set to 0.15 mm (plan now passes) but the elevation stays at 0.905.
- external-venetian-blind: 1:200 plan H 0.075; 1:100 elevation H 0.0522; 1:100 plan H 0.0737 (limits 0.06, 0.03, 0.03). The volume reaches beyond the drawn box footprint in plan.
- masonry-wall-with-lintel: 1:200 front H 0.105, plan H 0.105; 1:100 front H 0.135, plan H 0.105; 1:50 plan H 0.105 (limits 0.06, 0.03, 0.015).
- sectional-garage-door: 1:200 bbox z 2.54 (2.5), elevation IoU 0.766; 1:100 bbox z 2.5304, plan IoU 0.906, H 0.0645; 1:50 bbox z 2.52, elevation H 0.0169, front H 0.0187, plan H 0.06.
- sliding-door: 1:50 front IoU 0.853, H 0.0431; plan IoU 0.887, H 0.0281.
- spiral-stair: 1:200 elevation H 0.0636, front H 0.0605; 1:100 bbox y 3.7511 (3.733); 1:50 bbox y 3.751, elevation H 0.0255, front H 0.0265, plan H 0.0173.
- tied-canopy: 1:500 front H 0.3563; 1:100 elevation H 0.0615; 1:50 plan H 0.2906.

Animals
- red-fox: 1:50 elevation H 0.0169, front H 0.0225 (ear and cheek width in the front drawing wider than the volume head).
- mallard-duck: 1:50 front IoU 0.893, H 0.025.
- domestic-cat: 1:200 front IoU 0.915; 1:50 elevation H 0.016, front H 0.0151 (front drawing wider than the volume at y 0.33).
- pigeon: 1:200 elevation IoU 0.911; 1:100 front IoU 0.899; 1:50 elevation H 0.0151, front IoU 0.887, plan IoU 0.898, H 0.0296.
- labrador-dog: 1:500 plan IoU 0.901; 1:100 front H 0.0356, plan H 0.0377; 1:50 elevation H 0.0187, front H 0.0446, plan H 0.0379.
- hedgehog: 1:100 front IoU 0.909; 1:50 bbox y 0.1445 (0.128), elevation IoU 0.888, H 0.0296, front IoU 0.913, H 0.0159.
- herring-gull: 1:200 front IoU 0.894, plan IoU 0.909; 1:100 front IoU 0.88, H 0.0439; 1:50 bbox x 0.6915 (0.699), elevation H 0.017, front IoU 0.891, H 0.045.
- roe-deer: 1:500 plan IoU 0.885; 1:200 front IoU 0.886; 1:100 front IoU 0.889, plan H 0.0335; 1:50 elevation H 0.0168, front IoU 0.889, H 0.0291, plan H 0.0321.
- pig: 1:200 front IoU 0.918, plan H 0.0828; 1:100 front H 0.0338, plan H 0.0452; 1:50 bbox x 1.221 (1.23), elevation H 0.0185, front H 0.0338, plan H 0.0544.
- sheep: 1:200 plan H 0.0605; 1:100 elevation H 0.0428, front H 0.0318, plan H 0.0428; 1:50 bbox x 1.2841 (1.27), elevation H 0.0173, front H 0.0312, plan H 0.0439.
- dairy-cow: 1:500 front IoU 0.907; 1:200 bbox x 2.5124 (2.55), front H 0.1125, plan IoU 0.917, H 0.0912; 1:100 front H 0.074, plan H 0.0788; 1:50 elevation H 0.0268, front H 0.1064, plan H 0.0772.
- goat: 1:500 front IoU 0.886, plan IoU 0.914; 1:200 front IoU 0.915; 1:100 front IoU 0.906, H 0.06; 1:50 bbox x 1.3 (1.31), elevation H 0.0177, front IoU 0.907, H 0.0569, plan H 0.0319.
- horse: 1:500 plan IoU 0.908; 1:200 elevation H 0.1295, front H 0.1209, plan IoU 0.918, H 0.1092; 1:100 elevation H 0.1284, front H 0.0712, plan H 0.0981; 1:50 bbox x 2.9005 (2.91), z 0.63 (0.62), elevation H 0.1295, front H 0.0712, plan H 0.1425.

## Method notes for the next pass

- Recurring cause 1 (appliances, components): a drawn feature the volume lacks or vice versa (hole in a handle drawn only as stroke, notch, bevel, spout seen in front, visible parts beyond a plan section plane).
- Recurring cause 2: strokes. An unfilled shape is rasterised with its pen at the scale (0.13 mm detail pen is 6.5 mm at 1:50, 13 mm at 1:100), so a stroke thicker than the member it labels adds area outside the volume; remove it or draw a filled shape.
- Recurring cause 3: lofts with superellipse exponent below about 6 against rectangular drawn parts.
- Animals: mismatches are 1 to 3 cells of outline everywhere (hair/fur tessellation against traced contours) and need the loft sections refitted to the traced contours; `silhouette-bins.ts` (STATS=<bin m> env, prints drawn-only/volume-only cell counts per bin) and `silhouette-solids.ts` (world bounding box per solid) of this ticket locate the offset quickly; `sil-edit.py` has the span-preserving helpers (`V`, `R`, `sp`, `rm`, `ins_after`, `ins_before`, `replace_obj`).
