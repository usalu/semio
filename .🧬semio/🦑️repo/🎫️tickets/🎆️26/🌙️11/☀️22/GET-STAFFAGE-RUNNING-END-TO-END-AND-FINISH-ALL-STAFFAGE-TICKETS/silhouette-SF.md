# 🪑️ Silhouette SF: the 18 Furniture Assets of the Batch (2026-11-23)

Fixer report for `🧰️silhouette/SF.tsv` after `📋️silhouette-brief.md`. Numbers are `IoU/largest boundary distance` per view from `🧰️silhouette/SF/measure.ts` (= `measureAsset`, the path of the guard); `!` marks a failing check, bbox is body/declared per axis (width, height, depth). No test, rule, tolerance, engine, schema, fixture or README was edited; the manifest was not edited and no declared dimension or power changed (`🧰️silhouette/SF/manifest-changes.json` is `{}`).

## Result in short

- **17 of 18 green**: the real `🧊️catalog-silhouette` test of the asset passes for 17 (each run to its end in this session, last line of the proofs below). **`used-bentwood-dining-chairs` stays red** at 1:200 (side elevation 0.908, front elevation 0.918 against 0.92); its 1:500 failure (375.9 mm) is repaired.
- `single-bed`: the 900 s went into the sharp reference of the test, not into the asset's check (owned raster 2.6 s, librsvg > 300 s for the 1:50 front elevation). Worked around in the asset (caps as prisms); the test passes in 116 s, 110 s of it still librsvg for that one view. Request filed with the measurement.
- `validateAsset` 0 issues and check CLI `✓` for all 18 (before: 0 issues for all 18). `🧊️three-views` 38 passed before and after, `🌡️catalog-physics` 38 passed before and after (name filter over the 18; the filter also takes `used-backrest-park-bench`). `checkVolumes` of all scales: one finding before and after, not from this batch (`operating-table` 1:50 `volumes-connected`, `member-column-upper` 5.0 mm from `member-column-lower`).
- `🫁️breathing-catalog` names humans and trees only, `✅️asset-validation` and `⚡️asset-power` name no asset in their cases (83 and 24 skipped under the filter), so there is nothing of these three to run per asset; no thermal region, emitter or rating was touched.
- Three requests in `requests.md` (sections `## silhouette SF — …`): the sharp reference and overlapping copies; the banded measure against the whole raster; the reference of slender solids.

## Sheets

The 1:50 and 1:500 sheets of all 18 were rendered after the repair (`look.sh`, r3-2d). Opened and looked at: 1:50 of `bar-stool`, `height-adjustable-desk`, `office-swivel-chair`, `operating-table`, `picnic-table`, `single-bed`, `used-recliner-chair`, `used-wall-unit`, and 1:500 of `bar-stool`; none is uglier or wrong (the bar stool shows its bent back pad, the wall unit its cable hole as a ring, the recliner its tighter back corners). **Not opened**: the sheets of the other ten assets and the other 1:500 sheets. Their changes are 2 to 10 mm at 1:50 (0.04 to 0.2 mm of paper) or, for `park-bench` and the chairs at 1:500, below what the fitted sheet resolves (a 1:500 sheet shows the object 3 to 5 mm large); they were judged on the zoomed mask pictures of `where.ts` before and after instead.

## Per asset

### `bar-stool`

- First failure of the list: 1:50 plan Hausdorff: expected 0.058876194255403434 to be less than or equal to 0.015
- Failing before: 1:50 0.4961/0.498 0.8700/0.87 0.4982/0.498  elevation 0.990/13.1mm  front 0.988/13.1mm  plan 0.968/58.9mm!
- After: 1:50 0.4961/0.498 0.8700/0.87 0.4982/0.498  elevation 0.990/13.1mm  front 0.988/13.1mm  plan 0.989/8.4mm; 1:100 0.4968/0.498 0.8700/0.87 0.4989/0.498  elevation 0.994/6.8mm  front 0.985/9.6mm  plan 0.989/1.9mm; 1:200 0.4972/0.498 0.8700/0.87 0.4993/0.498  elevation 0.989/6.8mm  front 0.988/7.6mm  plan 0.987/2.0mm; 1:500 0.4979/0.498 0.8700/0.87 0.5001/0.498  elevation 0.975/14.8mm  front 0.971/25.9mm  plan 0.975/9.8mm
- Proof: catalog-silhouette: exit 0  Test Files 1 passed (1); Tests 1 passed | 384 skipped (385); / validateAsset: 0 issues / check CLI: ✓ bar-stool (1/1) / bar-stool: OK
- Repair in `fix-bar-stool.ts`: Plan `back-pad` of 1:50, 1:100 and 1:200: the outline of the bent loft (0.30 m radius) instead of the straight prism of the elevation profile. Drawing only.

### `height-adjustable-desk`

- First failure of the list: 1:100 front Hausdorff: expected 0.08657547285461396 to be less than or equal to 0.03
- Failing before: 1:50 1.6000/1.6 1.1800/1.18 0.8120/0.8!  elevation 0.999/5.6mm  front 0.986/16.0mm!  plan 1.000/1.9mm; 1:100 1.6000/1.6 1.1800/1.18 0.8000/0.8  elevation 1.000/0.0mm  front 0.972/86.6mm!  plan 1.000/5.3mm
- After: 1:50 1.6000/1.6 1.1800/1.18 0.8070/0.8  elevation 0.999/5.6mm  front 0.993/13.7mm  plan 1.000/1.9mm; 1:100 1.6000/1.6 1.1800/1.18 0.8000/0.8  elevation 1.000/0.0mm  front 0.995/13.5mm  plan 1.000/5.3mm; 1:200 1.6000/1.6 1.1800/1.18 0.8000/0.8  elevation 1.000/0.0mm  front 0.978/32.3mm  plan 1.000/0.0mm; 1:500 1.6000/1.6 1.1800/1.18 0.8000/0.8  elevation 1.000/0.0mm  front 0.956/46.1mm  plan 1.000/0.0mm
- Proof: catalog-silhouette: exit 0  Test Files 1 passed (1); Tests 1 passed | 384 skipped (385); / validateAsset: 0 issues / check CLI: ✓ height-adjustable-desk (1/1) / height-adjustable-desk: OK
- Repair in `fix-height-adjustable-desk.ts`: Volume: cable tray `top-tray` of 1:50 and 1:100 from a 1.5 mm sheet to the 4 mm and 8 mm walls the front elevation draws; handset of 1:50 7 mm instead of 12 mm proud of the top edge (solid, front, plan), so the declared depth 0.80 m holds. Drawing: VESA plate (1:50, 1:100) and the two hinge ears (1:50) stated in front and plan; monitor bulge of 1:100 as deep as its loft.

### `office-swivel-chair`

- First failure of the list: 1:50 elevation Hausdorff: expected 0.041249999999999995 to be less than or equal to 0.015
- Failing before: 1:50 0.6630/0.66 1.0029/1 0.6600/0.66  elevation 0.989/41.2mm!  front 0.985/16.9mm!  plan 0.998/1.9mm
- After: 1:50 0.6630/0.66 1.0029/1 0.6600/0.66  elevation 0.989/9.4mm  front 0.987/11.3mm  plan 0.998/1.9mm; 1:100 0.6660/0.66 1.0029/1 0.6600/0.66  elevation 0.987/15.5mm  front 0.992/7.5mm  plan 0.996/3.7mm; 1:200 0.6640/0.66 1.0029/1 0.6590/0.66  elevation 0.982/47.0mm  front 0.980/43.9mm  plan 0.998/2.6mm; 1:500 0.6581/0.66 1.0016/1 0.6600/0.66  elevation 0.985/8.7mm  front 0.977/47.0mm  plan 0.995/2.6mm
- Proof: catalog-silhouette: exit 0  Test Files 1 passed (1); Tests 1 passed | 384 skipped (385); / validateAsset: 0 issues / check CLI: ✓ office-swivel-chair (1/1) / office-swivel-chair: OK
- Repair in `fix-office-swivel-chair.ts`: 1:50: seat mechanism and seat shell meet at the top of the seat plate (0.29 mm hairline closed); loft section of the shell at v 0.17 m lowered by 1 mm so the underside lies flat on the plate (body and thermal list) and the drawn underside follows. Every twin-castor hub drawn as two rims over their wheels instead of one disc across the gap (10 hubs, all views that drew it).

### `operating-table`

- First failure of the list: 1:100 front Hausdorff: expected 0.0675 to be less than or equal to 0.03
- Failing before: 1:50 2.4500/2.45 1.9600/1.96 0.5600/0.56  elevation 0.998/10.9mm  front 0.997/11.3mm  plan 0.994/101.2mm!; 1:100 2.4500/2.45 1.9600/1.96 0.5600/0.56  elevation 0.994/18.8mm  front 0.996/67.5mm!  plan 0.997/101.2mm!
- After: 1:50 2.4500/2.45 1.9600/1.96 0.5600/0.56  elevation 0.998/10.9mm  front 0.997/11.3mm  plan 1.000/9.4mm; 1:100 2.4500/2.45 1.9600/1.96 0.5600/0.56  elevation 0.994/18.8mm  front 0.998/7.5mm  plan 1.000/5.3mm; 1:200 2.4500/2.45 1.9600/1.96 0.5600/0.56  elevation 0.997/15.0mm  front 0.998/7.5mm  plan 1.000/0.0mm; 1:500 2.4500/2.45 1.9600/1.96 0.5600/0.56  elevation 0.999/19.1mm  front 0.996/23.2mm  plan 1.000/0.0mm
- Proof: catalog-silhouette: exit 0  Test Files 1 passed (1); Tests 1 passed | 384 skipped (385); / validateAsset: 0 issues / check CLI: ✓ operating-table (1/1) / operating-table: OK
- Repair in `fix-operating-table.ts`: Drawing only. Plan 1:50 and 1:100: four hinge barrels, the left castors of table and lamp stand, at 1:50 the second wheel of every twin castor; base beams of 1:50 as the outline of their superelliptic lofts. Front and plan 1:100: the tilt pin. At 1:50 the hinges are unfilled outlines (a filled sliver in the 10 mm joint fails `suitability-feature`: found by the check CLI, corrected before the final proof).

### `park-bench`

- First failure of the list: 1:500 elevation: expected 0.9028271280704465 to be greater than or equal to 0.92
- Failing before: 1:500 1.8000/1.8 0.8467/0.85 0.5992/0.6  elevation 0.903/14.1mm!  front 0.961/16.9mm  plan 0.931/56.3mm
- After: 1:50 1.8000/1.8 0.8468/0.85 0.6034/0.6  elevation 0.999/3.8mm  front 0.990/7.7mm  plan 0.943/5.6mm; 1:100 1.8000/1.8 0.8476/0.85 0.6034/0.6  elevation 0.937/11.3mm  front 0.991/13.7mm  plan 1.000/0.0mm; 1:200 1.8000/1.8 0.8476/0.85 0.5992/0.6  elevation 0.943/14.1mm  front 0.991/16.9mm  plan 0.931/56.3mm; 1:500 1.8000/1.8 0.8467/0.85 0.5992/0.6  elevation 0.979/14.1mm  front 0.961/16.9mm  plan 0.931/56.3mm
- Proof: catalog-silhouette: exit 0  Test Files 1 passed (1); Tests 1 passed | 384 skipped (385); / validateAsset: 0 issues / check CLI: ✓ park-bench (1/1) / park-bench: OK
- Repair in `fix-park-bench.ts`: Side elevation of 1:500: the middle and the top back slat of the path `three-back-slats` at the heights of their solids (82 mm high, 28 mm apart instead of 71 mm and 53 mm). Drawing only.

### `picnic-table`

- First failure of the list: 1:50 elevation Hausdorff: expected 0.025155764746872634 to be less than or equal to 0.015
- Failing before: 1:50 1.5200/1.52 0.7500/0.75 1.8000/1.8  elevation 0.984/25.2mm!  front 1.000/0.0mm  plan 0.956/22.7mm!
- After: 1:50 1.5200/1.52 0.7500/0.75 1.8000/1.8  elevation 1.000/1.9mm  front 1.000/0.0mm  plan 1.000/0.0mm; 1:100 1.5200/1.52 0.7500/0.75 1.8000/1.8  elevation 0.991/25.2mm  front 1.000/3.8mm  plan 0.999/19.1mm; 1:200 1.5200/1.52 0.7500/0.75 1.8000/1.8  elevation 1.000/0.0mm  front 0.999/7.0mm  plan 1.000/0.0mm; 1:500 1.5200/1.52 0.7500/0.75 1.8000/1.8  elevation 1.000/0.0mm  front 0.977/35.9mm  plan 0.996/35.9mm
- Proof: catalog-silhouette: exit 0  Test Files 1 passed (1); Tests 1 passed | 384 skipped (385); / validateAsset: 0 issues / check CLI: ✓ picnic-table (1/1) / picnic-table: OK
- Repair in `fix-picnic-table.ts`: 1:50. Volume and drawing: table top as five planks of 128 mm at a pitch of 143 mm (15 mm joints) in body, thermal list, elevation and plan (the body had 10 mm joints, which cannot be printed: `suitability-gap` 0.07 mm, the drawing 24 mm joints). Drawing: cleat under the top and the two bench battens as unfilled outlines in plan; the two battens in the side elevation.

### `single-bed`

- First failure of the list: Test timed out in 900000ms
- Failing before: 1:50 the test did not end within 900 s (owned measure of all four scales green: 1:50 elevation 0.9996/5.6 mm, front 1.000/0.0 mm, plan 0.998/5.6 mm; sharp reference of the 1:50 front elevation > 300 s)
- After: 1:50 2.1200/2.12 0.9500/0.95 1.0100/1.01  elevation 1.000/5.6mm  front 1.000/0.0mm  plan 0.998/5.6mm; 1:100 2.1200/2.12 0.9500/0.95 1.0100/1.01  elevation 0.999/7.5mm  front 1.000/3.8mm  plan 1.000/3.8mm; 1:200 2.1200/2.12 0.9500/0.95 1.0100/1.01  elevation 1.000/7.5mm  front 1.000/0.0mm  plan 0.997/52.5mm; 1:500 2.1200/2.12 0.9500/0.95 1.0100/1.01  elevation 1.000/0.0mm  front 0.990/138.1mm  plan 1.000/0.0mm
- Proof: catalog-silhouette: exit 0  Test Files 1 passed (1); Tests 1 passed | 384 skipped (385); / validateAsset: 0 issues / check CLI: ✓ single-bed (1/1) / single-bed: OK
- Repair in `fix-single-bed.ts`: Volume of 1:50 (body and thermal list): the 56 rubber caps of the slats from boxes with all edges rounded (2028 triangles each) to prisms of their rounded section (60 triangles each): 132 004 to 21 796 triangles. No drawing changed. The cause of the timeout is the sharp reference of the test (request filed); the real test now ends in 116 s.

### `used-beer-garden-set`

- First failure of the list: 1:50 elevation Hausdorff: expected 0.032745467243574335 to be less than or equal to 0.015
- Failing before: 1:50 1.1100/1.11 0.7700/0.77 2.2000/2.2  elevation 0.996/32.7mm!  front 0.997/7.7mm  plan 1.000/3.8mm
- After: 1:50 1.1100/1.11 0.7700/0.77 2.2000/2.2  elevation 0.997/3.8mm  front 0.997/7.7mm  plan 1.000/3.8mm; 1:100 1.1100/1.11 0.7700/0.77 2.2000/2.2  elevation 0.997/7.5mm  front 0.997/8.4mm  plan 1.000/3.8mm; 1:200 1.1100/1.11 0.7700/0.77 2.2000/2.2  elevation 1.000/0.0mm  front 0.997/7.5mm  plan 0.999/15.0mm; 1:500 1.1100/1.11 0.7700/0.77 2.2000/2.2  elevation 1.000/0.0mm  front 0.999/8.6mm  plan 1.000/0.0mm
- Proof: catalog-silhouette: exit 0  Test Files 1 passed (1); Tests 1 passed | 384 skipped (385); / validateAsset: 0 issues / check CLI: ✓ used-beer-garden-set (1/1) / used-beer-garden-set: OK
- Repair in generator `USED-PARK-OBJECTS-STAFFAGE-COLLECTION/assets/used-beer-garden-set/author-beer-garden-set.py` (`STRUT_EYE`, `brace_part(eye=)`): 1:50, body and drawings: the folding struts reach 8 mm beyond their pin into the catch plate (strut and catch were 4.4 mm apart). Bone masses of the struts follow (total 42.12 to 42.15 kg, declared 42.1 kg unchanged).

### `used-bentwood-dining-chairs`

- First failure of the list: 1:500 front Hausdorff: expected 0.3758927230377559 to be less than or equal to 0.15
- Failing before: 1:200 0.9000/0.9 0.9371/0.936 1.0028/1  elevation 0.908/18.6mm!  front 0.918/35.9mm!  plan 0.997/8.9mm; 1:500 0.9000/0.9 0.9371/0.936 1.0028/1  elevation 0.940/23.8mm  front 0.941/375.9mm!  plan 0.993/18.6mm
- After: 1:50 0.9000/0.9 0.9371/0.936 1.0028/1  elevation 0.993/11.9mm  front 0.992/13.3mm  plan 0.999/6.8mm; 1:100 0.9000/0.9 0.9371/0.936 1.0028/1  elevation 0.995/11.7mm  front 0.988/13.5mm  plan 0.999/8.4mm; 1:200 0.9000/0.9 0.9371/0.936 1.0028/1  elevation 0.908/18.6mm!  front 0.918/35.9mm!  plan 0.997/8.9mm; 1:500 0.9000/0.9 0.9371/0.936 1.0028/1  elevation 0.938/23.8mm  front 0.941/34.8mm  plan 0.993/18.6mm
- Proof: catalog-silhouette: exit 1  Test Files 1 failed (1); Tests 1 failed | 384 skipped (385); / AssertionError: used-bentwood-dining-chairs 1:200 elevation: expected 0.9084358395807555 to be greater than or equal to 0.92 / validateAsset: 0 issues / check CLI: ✓ used-bentwood-dining-chairs (1/1) / used-bentwood-dining-chairs: 2 failing checks
- Repair in generator `USED-HOUSEHOLD-OBJECTS-STAFFAGE-COLLECTION/assets/used-bentwood-dining-chairs/author-bentwood.ts` (`partStroke(edge)`): 1:500 drawings: a leg ring seen edge-on is drawn from outer edge to outer edge, so the rings of the two stacks touch as in the body (front 375.9 mm to 34.8 mm). 1:200 untouched (see what stays red).

### `used-bunk-bed`

- First failure of the list: 1:50 elevation Hausdorff: expected 0.024375 to be less than or equal to 0.015
- Failing before: 1:50 2.3000/2.3 0.8094/0.809 1.2000/1.2  elevation 1.000/24.4mm!  front 1.000/0.0mm  plan 1.000/0.0mm
- After: 1:50 2.3000/2.3 0.8094/0.809 1.2000/1.2  elevation 1.000/1.9mm  front 1.000/0.0mm  plan 1.000/0.0mm; 1:100 2.3000/2.3 0.8094/0.809 1.2000/1.2  elevation 1.000/3.8mm  front 1.000/0.0mm  plan 1.000/0.0mm; 1:200 2.3000/2.3 0.8094/0.809 1.2000/1.2  elevation 1.000/0.0mm  front 1.000/0.0mm  plan 1.000/0.0mm; 1:500 2.3000/2.3 0.8094/0.809 1.2000/1.2  elevation 1.000/0.0mm  front 1.000/0.0mm  plan 1.000/0.0mm
- Proof: catalog-silhouette: exit 0  Test Files 1 passed (1); Tests 1 passed | 384 skipped (385); / validateAsset: 0 issues / check CLI: ✓ used-bunk-bed (1/1) / used-bunk-bed: OK
- Repair in generator `…/used-bunk-bed/author-used-bunk-bed.py` (`rounded(lip=)`): 1:50, body and drawings: the lying hollow of the lower mattress begins with a 2.5 mm rim 5 mm inside each end, so the void under the upper mattress of the stored bundle is a slit and not a wedge that tapers to nothing.

### `used-double-bed`

- First failure of the list: 1:50 front Hausdorff: expected 0.016875 to be less than or equal to 0.015
- Failing before: 1:50 2.4000/2.4 0.4760/0.476 1.8000/1.8  elevation 1.000/1.9mm  front 0.998/16.9mm!  plan 1.000/0.0mm
- After: 1:50 2.4000/2.4 0.4760/0.476 1.8000/1.8  elevation 1.000/1.9mm  front 1.000/9.4mm  plan 1.000/0.0mm; 1:100 2.4000/2.4 0.4760/0.476 1.8000/1.8  elevation 1.000/0.0mm  front 0.998/15.0mm  plan 1.000/0.0mm; 1:200 2.4000/2.4 0.4760/0.476 1.8000/1.8  elevation 1.000/0.0mm  front 1.000/0.0mm  plan 1.000/0.0mm; 1:500 2.4000/2.4 0.4760/0.476 1.8000/1.8  elevation 1.000/0.0mm  front 1.000/0.0mm  plan 1.000/0.0mm
- Proof: catalog-silhouette: exit 0  Test Files 1 passed (1); Tests 1 passed | 384 skipped (385); / validateAsset: 0 issues / check CLI: ✓ used-double-bed (1/1) / used-double-bed: OK
- Repair in generator `…/used-double-bed/author-used-double-bed.py` (`mattress_sections`): 1:50 volume: superellipse exponent of the mattress cross-section 20 to 60 (corner 5 mm instead of 14 mm inside the box the end view draws; exponent 40 measured exactly 15.0 mm and was not kept).

### `used-extendable-dining-table`

- First failure of the list: 1:50 elevation Hausdorff: expected 0.039375 to be less than or equal to 0.015
- Failing before: 1:50 1.4000/1.4 0.7500/0.75 0.9000/0.9  elevation 0.999/39.4mm!  front 0.999/52.5mm!  plan 1.000/0.0mm
- After: 1:50 1.4000/1.4 0.7500/0.75 0.9000/0.9  elevation 0.999/7.5mm  front 1.000/1.9mm  plan 1.000/0.0mm; 1:100 1.4000/1.4 0.7500/0.75 0.9000/0.9  elevation 1.000/3.8mm  front 0.988/0.0mm  plan 1.000/0.0mm; 1:200 1.4000/1.4 0.7500/0.75 0.9000/0.9  elevation 1.000/5.5mm  front 1.000/0.0mm  plan 1.000/0.0mm; 1:500 1.4000/1.4 0.7500/0.75 0.9000/0.9  elevation 1.000/0.0mm  front 1.000/3.5mm  plan 1.000/0.0mm
- Proof: catalog-silhouette: exit 0  Test Files 1 passed (1); Tests 1 passed | 384 skipped (385); / validateAsset: 0 issues / check CLI: ✓ used-extendable-dining-table (1/1) / used-extendable-dining-table: OK
- Repair in generator `…/used-extendable-dining-table/author-table.py`: Drawings of 1:50, 1:100 and 1:200: side aprons (elevation) and end apron (front) as long as their solids, 10 mm into each leg.

### `used-hinged-wardrobe`

- First failure of the list: 1:50 front Hausdorff: expected 0.020624999999999998 to be less than or equal to 0.015
- Failing before: 1:50 1.5000/1.5 2.0420/2.042 0.5800/0.58  elevation 1.000/1.9mm  front 0.999/20.6mm!  plan 1.000/7.5mm
- After: 1:50 1.5000/1.5 2.0420/2.042 0.5800/0.58  elevation 1.000/1.9mm  front 1.000/11.4mm  plan 1.000/7.5mm; 1:100 1.5000/1.5 2.0420/2.042 0.5800/0.58  elevation 1.000/0.0mm  front 1.000/0.0mm  plan 1.000/7.5mm; 1:200 1.5000/1.5 2.0120/2.042 0.5800/0.58  elevation 1.000/0.0mm  front 1.000/0.0mm  plan 1.000/0.0mm; 1:500 1.5000/1.5 2.0120/2.042 0.5560/0.58  elevation 1.000/0.0mm  front 0.960/23.6mm  plan 0.959/41.0mm
- Proof: catalog-silhouette: exit 0  Test Files 1 passed (1); Tests 1 passed | 384 skipped (385); / validateAsset: 0 issues / check CLI: ✓ used-hinged-wardrobe (1/1) / used-hinged-wardrobe: OK
- Repair in generator `…/used-hinged-wardrobe/author-wardrobe.py`: 1:50 drawings (three views): each bow handle as bar and two posts with the opening between them, as the body has it, instead of one block.

### `used-kitchen-base-cabinet`

- First failure of the list: 1:50 front Hausdorff: expected 0.0544073179820509 to be less than or equal to 0.015
- Failing before: 1:50 0.6090/0.609 1.0700/1.071 1.2000/1.2  elevation 1.000/1.9mm  front 0.999/54.4mm!  plan 1.000/0.0mm
- After: 1:50 0.6090/0.609 1.0700/1.071 1.2000/1.2  elevation 1.000/1.9mm  front 1.000/13.1mm  plan 1.000/0.0mm; 1:100 0.6090/0.609 1.0715/1.071 1.2000/1.2  elevation 0.999/20.2mm  front 0.997/20.2mm  plan 1.000/0.0mm; 1:200 0.6000/0.609 1.0630/1.071 1.2000/1.2  elevation 1.000/0.0mm  front 1.000/0.0mm  plan 1.000/0.0mm; 1:500 0.6000/0.609 1.0710/1.071 1.2000/1.2  elevation 1.000/0.0mm  front 0.985/14.1mm  plan 1.000/0.0mm
- Proof: catalog-silhouette: exit 0  Test Files 1 passed (1); Tests 1 passed | 384 skipped (385); / validateAsset: 0 issues / check CLI: ✓ used-kitchen-base-cabinet (1/1) / used-kitchen-base-cabinet: OK
- Repair in generator `…/used-kitchen-base-cabinet/author_scales.py` (`plinth`): 1:50, body and drawings: the plinth panel reaches the carcass bottom (the 2 mm slit above it is closed).

### `used-recliner-chair`

- First failure of the list: 1:200 front Hausdorff: expected 0.12034687750153925 to be less than or equal to 0.06
- Failing before: 1:50 1.0000/1 1.0282/1.0282 0.9000/0.9  elevation 1.000/3.8mm  front 0.990/41.2mm!  plan 1.000/0.0mm; 1:100 1.0000/1 1.0282/1.0282 0.9000/0.9  elevation 1.000/7.5mm  front 0.992/45.0mm!  plan 1.000/0.0mm; 1:200 1.0000/1 1.0243/1.0282 0.9000/0.9  elevation 0.999/8.0mm  front 0.990/120.3mm!  plan 1.000/0.0mm
- After: 1:50 1.0000/1 1.0282/1.0282 0.9000/0.9  elevation 1.000/3.8mm  front 0.997/13.1mm  plan 1.000/0.0mm; 1:100 1.0000/1 1.0282/1.0282 0.9000/0.9  elevation 1.000/7.5mm  front 0.997/18.8mm  plan 1.000/0.0mm; 1:200 1.0000/1 1.0243/1.0282 0.9000/0.9  elevation 0.999/8.0mm  front 0.992/37.0mm  plan 1.000/0.0mm; 1:500 1.0000/1 1.0312/1.0282 0.9000/0.9  elevation 1.000/0.0mm  front 0.997/4.0mm  plan 1.000/0.0mm
- Proof: catalog-silhouette: exit 0  Test Files 1 passed (1); Tests 1 passed | 384 skipped (385); / validateAsset: 0 issues / check CLI: ✓ used-recliner-chair (1/1) / used-recliner-chair: OK
- Repair in generator `…/used-recliner-chair/author-recliner.py` (`back_front_outline`, `SEAT_HALF`, `PAD_HALF`): Front drawing: top corners of the back cushion 25 mm (1:50) and 50 mm (1:100) instead of 90 mm (the body is the prism of the side profile, square in front); rear rail drawn at 1:50 and 1:100. 1:200 and 1:500, body and drawings: seat cushion and footrest pad as wide as the seat frame (the 2 and 3 mm slits beside the arms are closed).

### `used-shoe-cabinet`

- First failure of the list: 1:100 axis 1: 1.4649999999999999 m against 1.49 m: expected 0.025000000000000133 to be less than or equal to 0.015
- Failing before: 1:100 0.6000/0.6 1.4650/1.49! 0.9000/0.9  elevation 1.000/0.0mm  front 1.000/0.0mm  plan 1.000/0.0mm
- After: 1:50 0.6000/0.6 1.4900/1.49 0.9000/0.9  elevation 1.000/0.0mm  front 1.000/0.0mm  plan 1.000/0.0mm; 1:100 0.6000/0.6 1.4900/1.49 0.9000/0.9  elevation 1.000/0.0mm  front 1.000/0.0mm  plan 1.000/0.0mm; 1:200 0.6000/0.6 1.4650/1.49 0.9000/0.9  elevation 1.000/0.0mm  front 1.000/0.0mm  plan 1.000/0.0mm; 1:500 0.6000/0.6 1.4650/1.49 0.9000/0.9  elevation 1.000/17.2mm  front 1.000/0.0mm  plan 1.000/0.0mm
- Proof: catalog-silhouette: exit 0  Test Files 1 passed (1); Tests 1 passed | 384 skipped (385); / validateAsset: 0 issues / check CLI: ✓ used-shoe-cabinet (1/1) / used-shoe-cabinet: OK
- Repair in generator `…/used-shoe-cabinet/author_parts.py` (`scale_100`): 1:100, body, drawings and structure: the remaining wall bracket as an angle of 15 mm legs (25 mm above the top, the top of the declared height).

### `used-sun-lounger-bench`

- First failure of the list: 1:50 plan Hausdorff: expected 0.080625 to be less than or equal to 0.015
- Failing before: 1:50 1.9993/2 1.2639/1.26447 0.8000/0.8  elevation 1.000/7.5mm  front 0.996/9.4mm  plan 0.989/80.6mm!
- After: 1:50 1.9993/2 1.2639/1.26447 0.8000/0.8  elevation 1.000/7.5mm  front 0.996/9.4mm  plan 0.999/1.9mm; 1:100 2.0000/2 1.2645/1.26447 0.8000/0.8  elevation 1.000/7.5mm  front 0.986/13.5mm  plan 1.000/0.0mm; 1:200 1.9993/2 1.2645/1.26447 0.8000/0.8  elevation 1.000/0.0mm  front 0.984/17.8mm  plan 0.992/15.0mm; 1:500 1.9993/2 1.2645/1.26447 0.8000/0.8  elevation 1.000/0.0mm  front 1.000/0.0mm  plan 1.000/0.0mm
- Proof: catalog-silhouette: exit 0  Test Files 1 passed (1); Tests 1 passed | 384 skipped (385); / validateAsset: 0 issues / check CLI: ✓ used-sun-lounger-bench (1/1) / used-sun-lounger-bench: OK
- Repair in generator `USED-PARK-OBJECTS-STAFFAGE-COLLECTION/assets/used-sun-lounger-bench/author-lounger.py` (`plan_spans(eased)`): Plan of the scales with eased slats: the slat spans are measured with the eased arrises, so the open joint between slat 3 and slat 4 that the body shows is drawn.

### `used-wall-unit`

- First failure of the list: 1:50 front Hausdorff: expected 0.080625 to be less than or equal to 0.015
- Failing before: 1:50 0.5600/0.56 2.0120/2.012 2.7000/2.7  elevation 1.000/1.9mm  front 0.999/80.6mm!  plan 1.000/0.0mm§
- After: 1:50 0.5600/0.56 2.0120/2.012 2.7000/2.7  elevation 1.000/1.9mm  front 1.000/7.5mm  plan 1.000/0.0mm§; 1:100 0.5600/0.56 2.0120/2.012 2.7000/2.7  elevation 1.000/0.0mm  front 0.999/0.0mm  plan 1.000/0.0mm; 1:200 0.5600/0.56 2.0120/2.012 2.7000/2.7  elevation 0.997/30.0mm  front 0.999/0.0mm  plan 1.000/0.0mm; 1:500 0.5600/0.56 2.0120/2.012 2.7000/2.7  elevation 0.994/70.7mm  front 1.000/0.0mm  plan 1.000/0.0mm
- Proof: catalog-silhouette: exit 0  Test Files 1 passed (1); Tests 1 passed | 384 skipped (385); / validateAsset: 0 issues / check CLI: ✓ used-wall-unit (1/1) / used-wall-unit: OK
- Repair in generator `…/used-wall-unit/author-used-wall-unit.py` (`pierced`, `front_50`): Front drawing of 1:50: the outline shape and the back of the television body are drawn with the 60 mm cable hole, the hole itself as an unfilled circle (it was a dark disc on a closed panel; the body is open there).

## What stays red

- `used-bentwood-dining-chairs` 1:200: side elevation IoU 0.908, front elevation IoU 0.918 (limit 0.92); largest distances 18.6 and 35.9 mm are within the 60 mm. Measured per part (`parts.ts`): the filled parts agree (seats 0.996, blanket 0.998, front legs 0.951); every slender rod has a reference 12 to 36 % larger than its drawn stroke (bows 0.848, inner loops 0.719, leg rings 0.758 in the side elevation), although the drawn stroke runs through the control points of the loft. I did not find a fault in the asset and did not fit the pens to the measure; the cause is suspected in the reference of slender solids and is not verified (request filed). Drawing the rings edge to edge at 1:200 as at 1:500 made it worse (0.904 / 0.914) and was not kept.
- `single-bed`: green, but 116 s of test time, almost all of it one librsvg call.
- `used-double-bed` 1:50 front 9.4 mm, `height-adjustable-desk` 1:50 front 13.7 mm and `used-kitchen-base-cabinet` 1:50 front 13.1 mm pass with little margin against 15 mm.

## Files

Created (all under `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS/`): `silhouette-SF.md`; `🧰️silhouette/SF/` with `env.sh`, `lib.ts`, `measure.ts`, `where.ts`, `near.ts`, `outline.ts`, `parts.ts`, `timing.ts`, `oracle-timing.ts`, `oracle-solids.ts`, `validate.ts`, `vt.sh`, `look.sh`, `prove.sh`, `try.sh`, `ship.sh`, `try-used.sh`, `ship-used.sh`, `manifest-changes.json`, and the repairs `fix-bar-stool.ts`, `fix-height-adjustable-desk.ts`, `fix-office-swivel-chair.ts`, `fix-operating-table.ts`, `fix-park-bench.ts`, `fix-picnic-table.ts`, `fix-single-bed.ts` (each idempotent: a second run reports "already in place"). The seven older assets have generator chains over several tickets that no longer reproduce their files, hence the named repair scripts; the eleven used objects have one generator each that reproduced the catalog file byte for byte before the repair, and the repair went into it.

Updated: `requests.md` (three sections appended); the catalog files `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🛋️furniture/<id>/🔣️.json` of all 18 assets of the batch; the generators named above and their output `assets/<id>/🔣️.json` in `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/USED-HOUSEHOLD-OBJECTS-STAFFAGE-COLLECTION/` (bentwood chairs, bunk bed, double bed, extendable table, wardrobe, kitchen cabinet, recliner, shoe cabinet, wall unit) and `…/USED-PARK-OBJECTS-STAFFAGE-COLLECTION/` (beer garden set, sun lounger), each identical to the catalog file.

Removed: `🗑️generated/silhouette/SF` (logs, backups, pictures, sheets).
