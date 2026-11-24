# 🌳️ Silhouette SV-1: 19 Plants

Fixer report of batch `🧰️silhouette/SV-1.tsv` (2026-11-23 to 2026-11-24) for the brief `📋️silhouette-brief.md`. Scope: `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌳️vegetation/<id>/🔣️.json` of the 19 plants below. No test, rule, threshold, engine, schema, fixture, README or site file was edited; the manifest was not edited (`🧰️silhouette/SV-1/manifest-changes.json` carries the declared dimensions that changed).

## Result in Short

- **15 of 19 pass the real `🧊️catalog-silhouette` test** (0 before): `apple-tree`, `black-alder`, `black-locust`, `common-ash`, `crab-apple`, `english-oak`, `european-beech`, `field-maple`, `fig-tree`, `ginkgo`, `hawthorn`, `honey-locust`, `hornbeam-hedge`, `horse-chestnut`, `italian-cypress`.
- **4 stay red**: `agave`, `bamboo-clump`, `date-palm`, `hosta-clump`. Their bounding boxes are repaired where they failed (agave, bamboo-clump, date-palm); their drawings are untouched because every automatic repair made them worse or unprintable (numbers under "What Stays Red").
- `validateAsset` reports 0 issues for all 19 (as before); the check CLI prints `✓` for all 19; the tests of the 19 in `🧊️three-views`, `🌡️catalog-physics`, `🫁️breathing-catalog`, `✅️asset-validation` and `⚡️asset-power` (name filter) pass as before (38, 38, 19, 0 and 2 tests; `✅️asset-validation` has no test per asset). `🧊️three-views` was run once more for the 19 at the very end: 38 passed.
- `ginkgo`, `honey-locust`, `european-beech` and `horse-chestnut` first crashed in the test's librsvg reference (a path of more than 10 MB at 1:50; 13.6 MB for ginkgo, the same before my change); after the coordinator lifted that tool limit the real test passes for all four (114 s to 691 s). Their volumes were not reduced.

## How It Was Measured

- Every check, not only the first: `🧰️silhouette/SV-1/measure.ts` (`measureAsset` per authored scale: bounding box per axis against `dimensions`, IoU and largest boundary distance per view; `--volumes` adds every rule of `checkVolumes`). `locate.ts` cuts a view into the guard's own tiles and names every cluster of offending boundary cells with its place, its side, the shapes and the solids next to it, and writes a PNG of the place.
- The real test: `vt.sh` runs `bun x vitest run --config ../../🧪️tests/🎚️config/🟦️.ts ../../🧪️tests/<suite>/🟦️.ts -t "<ids>"` in `📦️packages/🟦️typescript` with the pinned bun, one process at a time.
- Before a file was written: the candidate (the ops replayed on a copy) was measured with every volume rule, validated, checked for print suitability (`suit.ts`, `resolveRepresentation` under the default print profile) and compared line by line with the catalog file as it was (`one.sh`). After writing: real test, `validateAsset`, check CLI (`ship.sh`).

## How It Was Repaired

All changes are data of the plant, recorded as idempotent ops in `🧰️silhouette/SV-1/ops/<id>.json` and replayed by `apply.ts <id>`, which is the last step of the chain of each plant (after its generator and the finisher patches of the earlier tickets; none of those reproduces today's catalog file on its own, so the ops are applied to the file as it stands). `apply.ts` writes the changes into the text of the file (`preserve.ts`): everything the ops do not touch keeps its bytes and its layout.

- **Bounding box (`bbox.ts`).** Per axis the declared value stays when every scale has it. Otherwise, when one value lies within a smallest feature of every scale, the axis is redeclared at the such value nearest to the declared one and no geometry changes. Otherwise the outermost solids of the scales that miss it are moved along the axis onto the ends of the finest agreeing scale (a plain polyhedron is stretched from its far end instead, so it stays joined).
- **Drawing (`fix.ts`).** Per scale and view, every cluster of boundary cells further than two smallest features from the other boundary is repaired on the guard's own raster. Where the volume shows beyond the drawing, a filled patch traced from the volume is added on the bone of the solid it belongs to, in the look of the neighbouring shape. Where the drawing shows beyond the volume (mostly see-through gaps between the leaf masses that the drawing had closed), the shapes that cover the excess are cut back by polygon difference; a shape with a `solid` keeps it when its outline in the view is its profile and is left alone otherwise. Local coordinates are found by probing `evaluate` (Newton per vertex), so a shape lands where it was traced also where the front and plan carry depends on the elevation twin; a vertex on a jump of the carry is left out.
- **Print suitability.** Patches keep half a smallest feature away from white gaps narrower than a smallest feature; seven pairs of patches whose outlines the print profile found closer than 0.15 mm were joined into one (`--join`: crab-apple, ginkgo, apple-tree twice, english-oak three times).
- **Validator.** 58 patches of `english-oak` that `validateAsset` called `lateral-ambiguous` on a bone that turns in a pose were moved to the parent bone, up the chain until accepted. `english-oak` depth is declared at 15.878 m (`--set`), the value two scales have and the symbol of the asset accepts; 16.035 m (the finest body) made `symbol-extent-front-depth` and `symbol-extent-plan-depth` fire.

## What the Sheets Show

The 1:50 and 1:500 sheets of the 15 plants with a repaired drawing were rendered with the product pipeline (`look.sh`, `r3-2d.ts` of the 3D ticket). Opened: 1:50 of all 15, and 1:500 of `apple-tree`, `black-alder`, `black-locust`, `english-oak`, `field-maple`, `ginkgo`, `honey-locust` and `hornbeam-hedge`. Rendered but not opened: 1:500 of `common-ash`, `crab-apple`, `european-beech`, `fig-tree`, `hawthorn`, `horse-chestnut` and `italian-cypress`, whose 1:500 drawing no op touches (the repair found no offending cell there). The visible change is the same everywhere: gaps of the crown through which the volume lets light pass are now open in the drawing (small diamonds between leaf masses; in plan the opening over the stem of `hawthorn`, `european-beech`, `common-ash`), and limbs that stand out of the crown are drawn where the volume has them (`honey-locust` 1:500, `field-maple` 1:500 front). `hornbeam-hedge` 1:50 follows the leafy surface of its blocks instead of a straight box edge. Nothing reads wrong; the open gaps are the price of one body in volume and drawing.

## Per Plant

Columns: body extent against declared dimension per axis (width, height, depth), then IoU and largest boundary distance per view; `!` marks a failing check (limits: one smallest feature per axis, IoU 0.92, two smallest features).

<!-- tables -->

### agave

- Failing checks (bounding box axes, IoU and largest boundary distance over all scales and views): **10 before, 7 after**.
- Real `🧊️catalog-silhouette` test: FAILS (Test Files 1 failed (1); Tests 1 failed | 384 skipped (385)); first failing expectation: `agave 1:500 elevation: expected 0.9121383203952012 to be greater than or equal to 0.92`. `validateAsset`: 0 issues. Check CLI: `✓ agave (1/1)`.
- Ops (`🧰️silhouette/SV-1/ops/agave.json`): 1 dimension op. Declared dimensions for the manifest: `{"widthM": 2.4916, "heightM": 1.447, "depthM": 2.3666}`.

| scale | before (width height depth: body/declared; view IoU/largest distance; `!` fails) | after |
|---|---|---|
| 1:50 | 2.4852/2.5! 1.4534/1.445! 2.3602/2.373!  elevation 0.9440/62.1mm!  front 0.9436/56.2mm!  plan 0.9931/15.0mm | 2.4852/2.4916 1.4534/1.447 2.3602/2.3666  elevation 0.9440/62.1mm!  front 0.9436/56.2mm!  plan 0.9931/15.0mm |
| 1:100 | 2.4879/2.5 1.4484/1.445 2.3620/2.373  elevation 0.9274/82.3mm!  front 0.9280/69.8mm!  plan 0.9931/8.4mm | 2.4879/2.4916 1.4484/1.447 2.3620/2.3666  elevation 0.9274/82.3mm!  front 0.9280/69.8mm!  plan 0.9931/8.4mm |
| 1:200 | 2.4877/2.5 1.4453/1.445 2.3618/2.373  elevation 0.9269/54.1mm  front 0.9298/75.0mm!  plan 0.9906/16.8mm | 2.4877/2.4916 1.4453/1.447 2.3618/2.3666  elevation 0.9269/54.1mm  front 0.9298/75.0mm!  plan 0.9906/16.8mm |
| 1:500 | 2.4890/2.5 1.4484/1.445 2.3623/2.373  elevation 0.9121!/105.4mm  front 0.9071!/112.8mm  plan 0.9769/40.2mm | 2.4890/2.4916 1.4484/1.447 2.3623/2.3666  elevation 0.9121!/105.4mm  front 0.9071!/112.8mm  plan 0.9769/40.2mm |

### apple-tree

- Failing checks (bounding box axes, IoU and largest boundary distance over all scales and views): **17 before, 0 after**.
- Real `🧊️catalog-silhouette` test: passes (Test Files 1 passed (1); Tests 1 passed | 384 skipped (385)). `validateAsset`: 0 issues. Check CLI: `✓ apple-tree (1/1)`.
- Ops (`🧰️silhouette/SV-1/ops/apple-tree.json`): 1 dimension op, 34 patches added, 62 shapes cut back or rebound, 2 shapes removed. Declared dimensions for the manifest: `{"widthM": 6.8071, "heightM": 6.3963, "depthM": 6.1723}`.

| scale | before (width height depth: body/declared; view IoU/largest distance; `!` fails) | after |
|---|---|---|
| 1:50 | 6.8135/6.789! 6.3958/6.35! 6.1659/6.2!  elevation 0.9943/34.6mm!  front 0.9915/46.9mm!  plan 0.9880/250.0mm! | 6.8135/6.8071 6.3958/6.3963 6.1659/6.1723  elevation 0.9957/13.1mm  front 0.9973/13.1mm  plan 0.9967/13.1mm |
| 1:100 | 6.8140/6.789! 6.3977/6.35! 6.1587/6.2!  elevation 0.9905/66.5mm!  front 0.9889/43.9mm!  plan 0.9855/540.0mm! | 6.8140/6.8071 6.3977/6.3963 6.1587/6.1723  elevation 0.9922/24.0mm  front 0.9943/25.2mm  plan 0.9920/26.3mm |
| 1:200 | 6.8195/6.789! 6.4251/6.35! 6.1474/6.2!  elevation 0.9880/40.4mm  front 0.9859/109.2mm!  plan 0.9890/40.4mm | 6.8195/6.8071 6.4251/6.3963 6.1474/6.1723  elevation 0.9880/40.4mm  front 0.9905/43.7mm  plan 0.9890/40.4mm |
| 1:500 | 6.7759/6.789 6.3588/6.35 6.1564/6.2  elevation 0.9545/151.2mm!  front 0.9576/142.8mm  plan 0.9590/118.6mm | 6.7759/6.8071 6.3588/6.3963 6.1564/6.1723  elevation 0.9618/120.1mm  front 0.9721/93.8mm  plan 0.9590/118.6mm |

### bamboo-clump

- Failing checks (bounding box axes, IoU and largest boundary distance over all scales and views): **11 before, 8 after**.
- Real `🧊️catalog-silhouette` test: FAILS (Test Files 1 failed (1); Tests 1 failed | 384 skipped (385)); first failing expectation: `bamboo-clump 1:100 elevation: expected 0.908022579269059 to be greater than or equal to 0.92`. `validateAsset`: 0 issues. Check CLI: `✓ bamboo-clump (1/1)`.
- Ops (`🧰️silhouette/SV-1/ops/bamboo-clump.json`): 1 dimension op. Declared dimensions for the manifest: `{"widthM": 2.974, "heightM": 4.5418, "depthM": 1.7192}`.

| scale | before (width height depth: body/declared; view IoU/largest distance; `!` fails) | after |
|---|---|---|
| 1:50 | 2.9676/2.977! 4.5354/4.552! 1.7256/1.714!  elevation 0.8958!/47.8mm!  front 0.9391/38.0mm!  plan 0.9583/62.9mm! | 2.9676/2.974 4.5354/4.5418 1.7256/1.7192  elevation 0.8958!/47.8mm!  front 0.9391/38.0mm!  plan 0.9583/62.9mm! |
| 1:100 | 2.9826/2.977 4.5372/4.552 1.7281/1.714  elevation 0.9080!/75.0mm!  front 0.9324/144.8mm!  plan 0.9859/43.7mm! | 2.9826/2.974 4.5372/4.5418 1.7281/1.7192  elevation 0.9080!/75.0mm!  front 0.9324/144.8mm!  plan 0.9859/43.7mm! |
| 1:200 | 2.9652/2.977 4.5225/4.552 1.7353/1.714  elevation 0.9337/30.0mm  front 0.9565/22.5mm  plan 0.9917/15.0mm | 2.9652/2.974 4.5225/4.5418 1.7353/1.7192  elevation 0.9337/30.0mm  front 0.9565/22.5mm  plan 0.9917/15.0mm |
| 1:500 | 2.9514/2.977 4.5406/4.552 1.6951/1.714  elevation 0.9242/40.0mm  front 0.9432/136.2mm  plan 0.9724/65.3mm | 2.9514/2.974 4.5406/4.5418 1.6951/1.7192  elevation 0.9242/40.0mm  front 0.9432/136.2mm  plan 0.9724/65.3mm |

### black-alder

- Failing checks (bounding box axes, IoU and largest boundary distance over all scales and views): **9 before, 0 after**.
- Real `🧊️catalog-silhouette` test: passes (Test Files 1 passed (1); Tests 1 passed | 384 skipped (385)). `validateAsset`: 0 issues. Check CLI: `✓ black-alder (1/1)`.
- Ops (`🧰️silhouette/SV-1/ops/black-alder.json`): 19 shapes cut back or rebound, 13 patches added, 6 solids moved or stretched.

| scale | before (width height depth: body/declared; view IoU/largest distance; `!` fails) | after |
|---|---|---|
| 1:50 | 7.9611/7.9611 20.0000/20 7.4835/7.4835  elevation 0.9973/42.8mm!  front 0.9972/32.7mm!  plan 0.9959/150.3mm! | 7.9611/7.9611 20.0000/20 7.4835/7.4835  elevation 0.9974/13.1mm  front 0.9973/13.1mm  plan 0.9969/9.4mm |
| 1:100 | 7.9611/7.9611 20.0000/20 7.4835/7.4835  elevation 0.9976/101.2mm!  front 0.9975/101.2mm!  plan 0.9969/100.8mm! | 7.9611/7.9611 20.0000/20 7.4835/7.4835  elevation 0.9976/22.5mm  front 0.9975/22.8mm  plan 0.9973/11.3mm |
| 1:200 | 7.9611/7.9611 20.0000/20 7.4835/7.4835  elevation 0.9979/16.8mm  front 0.9980/1136.9mm!  plan 0.9976/15.0mm | 7.9611/7.9611 20.0000/20 7.4835/7.4835  elevation 0.9979/16.8mm  front 0.9981/16.8mm  plan 0.9976/15.0mm |
| 1:500 | 7.8558/7.9611! 20.0000/20 7.5869/7.4835!  elevation 0.9944/26.5mm  front 0.9935/59.3mm  plan 0.9956/37.5mm | 7.9611/7.9611 20.0000/20 7.4835/7.4835  elevation 0.9921/59.3mm  front 0.9927/75.0mm  plan 0.9829/75.0mm |

### black-locust

- Failing checks (bounding box axes, IoU and largest boundary distance over all scales and views): **15 before, 0 after**.
- Real `🧊️catalog-silhouette` test: passes (Test Files 1 passed (1); Tests 1 passed | 384 skipped (385)). `validateAsset`: 0 issues. Check CLI: `✓ black-locust (1/1)`.
- Ops (`🧰️silhouette/SV-1/ops/black-locust.json`): 18 solids moved or stretched, 75 shapes cut back or rebound, 10 patches added.

| scale | before (width height depth: body/declared; view IoU/largest distance; `!` fails) | after |
|---|---|---|
| 1:50 | 12.6827/12.686 19.7798/19.78 11.8676/11.869  elevation 0.9948/148.5mm!  front 0.9946/40.3mm!  plan 0.9953/54.5mm! | 12.6827/12.686 19.7798/19.78 11.8676/11.869  elevation 0.9965/12.6mm  front 0.9959/13.1mm  plan 0.9963/13.1mm |
| 1:100 | 12.7551/12.686! 19.8028/19.78! 11.9355/11.869!  elevation 0.9953/36.9mm!  front 0.9951/56.7mm!  plan 0.9958/57.4mm! | 12.6860/12.686 19.7800/19.78 11.8690/11.869  elevation 0.9953/22.8mm  front 0.9951/26.3mm  plan 0.9960/22.5mm |
| 1:200 | 12.8216/12.686! 19.8247/19.78! 11.9899/11.869!  elevation 0.9960/23.7mm  front 0.9960/37.5mm  plan 0.9964/27.0mm | 12.6860/12.686 19.7800/19.78 11.8690/11.869  elevation 0.9956/45.6mm  front 0.9953/47.4mm  plan 0.9963/33.5mm |
| 1:500 | 12.8917/12.686! 19.8477/19.78 12.0476/11.869!  elevation 0.9959/37.5mm  front 0.9957/37.5mm  plan 0.9956/1715.3mm! | 12.6860/12.686 19.8477/19.78 11.8690/11.869  elevation 0.9941/112.5mm  front 0.9934/93.8mm  plan 0.9904/112.5mm |

### common-ash

- Failing checks (bounding box axes, IoU and largest boundary distance over all scales and views): **8 before, 0 after**.
- Real `🧊️catalog-silhouette` test: passes (Test Files 1 passed (1); Tests 1 passed | 384 skipped (385)). `validateAsset`: 0 issues. Check CLI: `✓ common-ash (1/1)`.
- Ops (`🧰️silhouette/SV-1/ops/common-ash.json`): 1 dimension op, 140 shapes cut back or rebound. Declared dimensions for the manifest: `{"widthM": 15.7557}`.

| scale | before (width height depth: body/declared; view IoU/largest distance; `!` fails) | after |
|---|---|---|
| 1:50 | 15.7494/15.769! 25.6000/25.6 14.4187/14.426  elevation 0.9971/107.0mm!  front 0.9964/116.6mm!  plan 0.9970/159.4mm! | 15.7494/15.7557 25.6000/25.6 14.4187/14.426  elevation 0.9978/13.1mm  front 0.9972/13.1mm  plan 0.9985/13.1mm |
| 1:100 | 15.7626/15.769 25.6000/25.6 14.4211/14.426  elevation 0.9956/395.2mm!  front 0.9965/258.8mm!  plan 0.9970/27.0mm | 15.7626/15.7557 25.6000/25.6 14.4211/14.426  elevation 0.9956/26.3mm  front 0.9965/23.7mm  plan 0.9972/22.5mm |
| 1:200 | 15.7638/15.769 25.6000/25.6 14.4122/14.426  elevation 0.9961/930.0mm!  front 0.9966/877.5mm!  plan 0.9964/37.5mm | 15.7638/15.7557 25.6000/25.6 14.4122/14.426  elevation 0.9962/38.2mm  front 0.9966/40.4mm  plan 0.9964/37.5mm |
| 1:500 | 15.7333/15.769 25.6000/25.6 14.4109/14.426  elevation 0.9976/41.9mm  front 0.9978/41.9mm  plan 0.9971/37.5mm | 15.7333/15.7557 25.6000/25.6 14.4109/14.426  elevation 0.9976/41.9mm  front 0.9978/41.9mm  plan 0.9971/37.5mm |

### crab-apple

- Failing checks (bounding box axes, IoU and largest boundary distance over all scales and views): **8 before, 0 after**.
- Real `🧊️catalog-silhouette` test: passes (Test Files 1 passed (1); Tests 1 passed | 384 skipped (385)). `validateAsset`: 0 issues. Check CLI: `✓ crab-apple (1/1)`.
- Ops (`🧰️silhouette/SV-1/ops/crab-apple.json`): 42 shapes cut back or rebound, 67 patches added, 8 shapes removed.

| scale | before (width height depth: body/declared; view IoU/largest distance; `!` fails) | after |
|---|---|---|
| 1:50 | 7.6590/7.659 6.9921/6.992 7.4675/7.468  elevation 0.9975/23.5mm!  front 0.9674/335.6mm!  plan 0.9846/140.6mm! | 7.6590/7.659 6.9921/6.992 7.4675/7.468  elevation 0.9976/11.3mm  front 0.9971/13.1mm  plan 0.9979/13.1mm |
| 1:100 | 7.6590/7.659 6.9921/6.992 7.4675/7.468  elevation 0.9979/20.2mm  front 0.9705/151.9mm!  plan 0.9858/117.7mm! | 7.6590/7.659 6.9921/6.992 7.4675/7.468  elevation 0.9979/20.2mm  front 0.9953/22.5mm  plan 0.9959/26.3mm |
| 1:200 | 7.6591/7.659 6.9951/6.992 7.4676/7.468  elevation 0.9976/282.7mm!  front 0.9725/111.5mm!  plan 0.9856/102.0mm! | 7.6591/7.659 6.9951/6.992 7.4676/7.468  elevation 0.9972/23.7mm  front 0.9926/52.5mm  plan 0.9942/38.2mm |
| 1:500 | 7.6582/7.659 6.9963/6.992 7.4689/7.468  elevation 0.9970/18.8mm  front 0.9950/26.5mm  plan 0.9966/26.5mm | 7.6582/7.659 6.9963/6.992 7.4689/7.468  elevation 0.9970/18.8mm  front 0.9950/26.5mm  plan 0.9966/26.5mm |

### date-palm

- Failing checks (bounding box axes, IoU and largest boundary distance over all scales and views): **24 before, 16 after**.
- Real `🧊️catalog-silhouette` test: FAILS (Test Files 1 failed (1); Tests 1 failed | 384 skipped (385)); first failing expectation: `date-palm 1:500 elevation Hausdorff: expected 0.25155764746872633 to be less than or equal to 0.15`. `validateAsset`: 0 issues. Check CLI: `✓ date-palm (1/1)`.
- Ops (`🧰️silhouette/SV-1/ops/date-palm.json`): 14 solids moved or stretched.

| scale | before (width height depth: body/declared; view IoU/largest distance; `!` fails) | after |
|---|---|---|
| 1:50 | 6.1759/6.3! 12.3854/12.5! 6.8989/7!  elevation 0.9094!/105.7mm!  front 0.8973!/157.6mm!  plan 0.9892/83.0mm! | 6.3000/6.3 12.5014/12.5 7.0000/7  elevation 0.9021!/105.7mm!  front 0.8882!/157.9mm!  plan 0.9851/102.3mm! |
| 1:100 | 6.1803/6.3! 12.3873/12.5! 6.9015/7!  elevation 0.9121!/151.2mm!  front 0.8999!/210.0mm!  plan 0.9846/39.2mm! | 6.3000/6.3 12.5024/12.5 7.0000/7  elevation 0.9054!/154.9mm!  front 0.8918!/213.8mm!  plan 0.9812/68.4mm! |
| 1:200 | 6.2505/6.3! 12.4488/12.5! 7.0026/7  elevation 0.9317/256.0mm!  front 0.9070!/214.5mm!  plan 0.9732/204.8mm! | 6.3000/6.3 12.5000/12.5 7.0026/7  elevation 0.9296/256.0mm!  front 0.9068!/214.5mm!  plan 0.9736/204.8mm! |
| 1:500 | 6.2890/6.3 12.4596/12.5 7.0250/7  elevation 0.9257/251.6mm!  front 0.9060!/93.8mm  plan 0.9891/59.3mm | 6.2890/6.3 12.4596/12.5 7.0250/7  elevation 0.9257/251.6mm!  front 0.9060!/93.8mm  plan 0.9891/59.3mm |

### english-oak

- Failing checks (bounding box axes, IoU and largest boundary distance over all scales and views): **18 before, 0 after**.
- Real `🧊️catalog-silhouette` test: passes (Test Files 1 passed (1); Tests 1 passed | 384 skipped (385)). `validateAsset`: 0 issues. Check CLI: `✓ english-oak (1/1)`.
- Ops (`🧰️silhouette/SV-1/ops/english-oak.json`): 1 dimension op, 13 solids moved or stretched, 87 shapes cut back or rebound, 209 patches added, 3 shapes removed. Declared dimensions for the manifest: `{"widthM": 18.853, "depthM": 15.878}`.

| scale | before (width height depth: body/declared; view IoU/largest distance; `!` fails) | after |
|---|---|---|
| 1:50 | 18.8530/19.07! 17.3424/17.302! 16.0347/15.8!  elevation 0.9966/69.4mm!  front 0.9967/75.6mm!  plan 0.9961/133.2mm! | 18.8530/18.853 17.3020/17.302 15.8780/15.878  elevation 0.9980/13.1mm  front 0.9982/13.1mm  plan 0.9977/13.1mm |
| 1:100 | 18.8222/19.07! 17.2956/17.302 15.5649/15.8!  elevation 0.9976/135.2mm!  front 0.9973/22.5mm  plan 0.9984/64.7mm! | 18.8530/18.853 17.2956/17.302 15.8780/15.878  elevation 0.9976/26.3mm  front 0.9973/24.0mm  plan 0.9985/26.3mm |
| 1:200 | 19.3167/19.07! 17.2613/17.302! 15.8784/15.8!  elevation 0.9963/40.4mm  front 0.9965/532.5mm!  plan 0.9978/27.0mm | 18.8530/18.853 17.3024/17.302 15.8784/15.878  elevation 0.9949/50.3mm  front 0.9956/48.0mm  plan 0.9974/50.3mm |
| 1:500 | 19.3001/19.07! 17.3274/17.302 15.8765/15.8!  elevation 0.9922/201.9mm!  front 0.9930/335.4mm!  plan 0.9961/53.0mm | 18.8530/18.853 17.3274/17.302 15.8765/15.878  elevation 0.9902/125.8mm  front 0.9930/59.3mm  plan 0.9946/131.3mm |

### european-beech

- Failing checks (bounding box axes, IoU and largest boundary distance over all scales and views): **10 before, 0 after**.
- Real `🧊️catalog-silhouette` test: passes (Test Files 1 passed (1); Tests 1 passed | 384 skipped (385)). `validateAsset`: 0 issues. Check CLI: `✓ european-beech (1/1)`.
- Ops (`🧰️silhouette/SV-1/ops/european-beech.json`): 1 dimension op, 89 shapes cut back or rebound. Declared dimensions for the manifest: `{"widthM": 21.2911, "depthM": 20.8507}`.

| scale | before (width height depth: body/declared; view IoU/largest distance; `!` fails) | after |
|---|---|---|
| 1:50 | 21.2974/21.299 29.6000/29.6 20.8516/20.856  elevation 0.9984/33.2mm!  front 0.9983/34.6mm!  plan 0.9982/24.4mm! | 21.2974/21.2911 29.6000/29.6 20.8516/20.8507  elevation 0.9990/13.1mm  front 0.9990/13.1mm  plan 0.9991/13.1mm |
| 1:100 | 21.2913/21.299 29.6000/29.6 20.8369/20.856!  elevation 0.9979/167.9mm!  front 0.9979/98.4mm!  plan 0.9978/791.3mm! | 21.2913/21.2911 29.6000/29.6 20.8369/20.8507  elevation 0.9980/26.3mm  front 0.9980/26.3mm  plan 0.9979/26.3mm |
| 1:200 | 21.2622/21.299! 29.6000/29.6 20.8234/20.856!  elevation 0.9980/33.5mm  front 0.9979/97.5mm!  plan 0.9955/37.5mm | 21.2622/21.2911 29.6000/29.6 20.8234/20.8507  elevation 0.9980/33.5mm  front 0.9979/37.5mm  plan 0.9955/37.5mm |
| 1:500 | 21.2488/21.299 29.6000/29.6 20.8073/20.856  elevation 0.9984/37.5mm  front 0.9983/37.5mm  plan 0.9974/41.9mm | 21.2488/21.2911 29.6000/29.6 20.8073/20.8507  elevation 0.9984/37.5mm  front 0.9983/37.5mm  plan 0.9974/41.9mm |

### field-maple

- Failing checks (bounding box axes, IoU and largest boundary distance over all scales and views): **3 before, 0 after**.
- Real `🧊️catalog-silhouette` test: passes (Test Files 1 passed (1); Tests 1 passed | 384 skipped (385)). `validateAsset`: 0 issues. Check CLI: `✓ field-maple (1/1)`.
- Ops (`🧰️silhouette/SV-1/ops/field-maple.json`): 5 shapes cut back or rebound.

| scale | before (width height depth: body/declared; view IoU/largest distance; `!` fails) | after |
|---|---|---|
| 1:50 | 8.2597/8.263 10.2515/10.25 7.7014/7.703  elevation 0.9979/17.3mm!  front 0.9981/11.9mm  plan 0.9982/9.4mm | 8.2597/8.263 10.2515/10.25 7.7014/7.703  elevation 0.9979/11.4mm  front 0.9981/11.9mm  plan 0.9982/9.4mm |
| 1:100 | 8.2587/8.263 10.2514/10.25 7.7005/7.703  elevation 0.9977/23.7mm  front 0.9978/87.5mm!  plan 0.9977/13.5mm | 8.2587/8.263 10.2514/10.25 7.7005/7.703  elevation 0.9977/23.7mm  front 0.9978/18.8mm  plan 0.9977/13.5mm |
| 1:200 | 8.2544/8.263 10.2576/10.25 7.6933/7.703  elevation 0.9977/27.0mm  front 0.9974/15.0mm  plan 0.9967/15.0mm | 8.2544/8.263 10.2576/10.25 7.6933/7.703  elevation 0.9977/27.0mm  front 0.9974/15.0mm  plan 0.9967/15.0mm |
| 1:500 | 8.2571/8.263 10.2500/10.25 7.6940/7.703  elevation 0.9970/26.5mm  front 0.9973/261.2mm!  plan 0.9976/26.5mm | 8.2571/8.263 10.2500/10.25 7.6940/7.703  elevation 0.9970/26.5mm  front 0.9973/41.9mm  plan 0.9976/26.5mm |

### fig-tree

- Failing checks (bounding box axes, IoU and largest boundary distance over all scales and views): **6 before, 0 after**.
- Real `🧊️catalog-silhouette` test: passes (Test Files 1 passed (1); Tests 1 passed | 384 skipped (385)). `validateAsset`: 0 issues. Check CLI: `✓ fig-tree (1/1)`.
- Ops (`🧰️silhouette/SV-1/ops/fig-tree.json`): 1 dimension op, 2 patches added. Declared dimensions for the manifest: `{"widthM": 7.0112, "heightM": 5.0076, "depthM": 7.2402}`.

| scale | before (width height depth: body/declared; view IoU/largest distance; `!` fails) | after |
|---|---|---|
| 1:50 | 7.0048/7.021! 5.0012/5.011! 7.2338/7.246!  elevation 0.9969/9.6mm  front 0.9964/86.3mm!  plan 0.9987/5.6mm | 7.0048/7.0112 5.0012/5.0076 7.2338/7.2402  elevation 0.9969/9.6mm  front 0.9967/11.4mm  plan 0.9987/5.6mm |
| 1:100 | 7.0043/7.021! 5.0009/5.011 7.2335/7.246  elevation 0.9949/15.5mm  front 0.9957/19.1mm  plan 0.9984/7.5mm | 7.0043/7.0112 5.0009/5.0076 7.2335/7.2402  elevation 0.9949/15.5mm  front 0.9957/19.1mm  plan 0.9984/7.5mm |
| 1:200 | 7.0040/7.021 5.0020/5.011 7.2333/7.246  elevation 0.9893/64.1mm!  front 0.9916/52.5mm  plan 0.9969/15.0mm | 7.0040/7.0112 5.0020/5.0076 7.2333/7.2402  elevation 0.9898/23.7mm  front 0.9916/52.5mm  plan 0.9969/15.0mm |
| 1:500 | 7.0086/7.021 5.0021/5.011 7.2406/7.246  elevation 0.9849/56.2mm  front 0.9854/37.5mm  plan 0.9937/37.5mm | 7.0086/7.0112 5.0021/5.0076 7.2406/7.2402  elevation 0.9849/56.2mm  front 0.9854/37.5mm  plan 0.9937/37.5mm |

### ginkgo

- Failing checks (bounding box axes, IoU and largest boundary distance over all scales and views): **8 before, 0 after**.
- Real `🧊️catalog-silhouette` test: passes (Test Files 1 passed (1); Tests 1 passed | 384 skipped (385)). `validateAsset`: 0 issues. Check CLI: `✓ ginkgo (1/1)`.
- Ops (`🧰️silhouette/SV-1/ops/ginkgo.json`): 10 solids moved or stretched, 21 patches added, 11 shapes cut back or rebound, 1 shapes removed.

| scale | before (width height depth: body/declared; view IoU/largest distance; `!` fails) | after |
|---|---|---|
| 1:50 | 7.6212/7.701! 14.8472/14.9! 6.9971/7.071!  elevation 0.9966/8.0mm  front 0.9969/11.4mm  plan 0.9971/14.6mm | 7.7010/7.701 14.9000/14.9 7.0710/7.071  elevation 0.9967/11.4mm  front 0.9970/11.4mm  plan 0.9975/13.1mm |
| 1:100 | 7.6334/7.701! 14.8394/14.9! 6.9775/7.071!  elevation 0.9965/42.8mm!  front 0.9966/16.8mm  plan 0.9968/10.6mm | 7.7010/7.701 14.9000/14.9 7.0710/7.071  elevation 0.9965/15.0mm  front 0.9965/20.2mm  plan 0.9969/19.1mm |
| 1:200 | 7.7009/7.701 14.9001/14.9 7.0705/7.071  elevation 0.9947/23.7mm  front 0.9949/16.8mm  plan 0.9954/15.0mm | 7.7009/7.701 14.9001/14.9 7.0705/7.071  elevation 0.9947/23.7mm  front 0.9949/16.8mm  plan 0.9954/15.0mm |
| 1:500 | 7.6659/7.701 14.8576/14.9 7.0156/7.071  elevation 0.9923/372.2mm!  front 0.9921/37.5mm  plan 0.9936/37.5mm | 7.6659/7.701 14.8576/14.9 7.0156/7.071  elevation 0.9924/41.9mm  front 0.9921/37.5mm  plan 0.9936/37.5mm |

### hawthorn

- Failing checks (bounding box axes, IoU and largest boundary distance over all scales and views): **7 before, 0 after**.
- Real `🧊️catalog-silhouette` test: passes (Test Files 1 passed (1); Tests 1 passed | 384 skipped (385)). `validateAsset`: 0 issues. Check CLI: `✓ hawthorn (1/1)`.
- Ops (`🧰️silhouette/SV-1/ops/hawthorn.json`): 1 dimension op, 24 patches added, 2 shapes cut back or rebound. Declared dimensions for the manifest: `{"heightM": 6.015}`.

| scale | before (width height depth: body/declared; view IoU/largest distance; `!` fails) | after |
|---|---|---|
| 1:50 | 6.3921/6.3942 6.0214/6! 6.0084/6.0105  elevation 0.9981/13.1mm  front 0.9976/21.4mm!  plan 0.9819/268.5mm! | 6.3921/6.3942 6.0214/6.015 6.0084/6.0105  elevation 0.9981/13.1mm  front 0.9981/13.1mm  plan 0.9957/12.0mm |
| 1:100 | 6.3921/6.3942 6.0241/6! 6.0084/6.0105  elevation 0.9981/47.4mm!  front 0.9976/26.3mm  plan 0.9885/211.2mm! | 6.3921/6.3942 6.0241/6.015 6.0084/6.0105  elevation 0.9981/15.0mm  front 0.9976/26.3mm  plan 0.9968/20.2mm |
| 1:200 | 6.3903/6.3942 6.0199/6 6.0067/6.0105  elevation 0.9984/16.8mm  front 0.9982/22.5mm  plan 0.9825/357.6mm! | 6.3903/6.3942 6.0199/6.015 6.0067/6.0105  elevation 0.9984/16.8mm  front 0.9982/22.5mm  plan 0.9963/10.6mm |
| 1:500 | 6.3937/6.3942 6.0175/6 6.0098/6.0105  elevation 0.9949/26.5mm  front 0.9893/26.5mm  plan 0.9940/18.8mm | 6.3937/6.3942 6.0175/6.015 6.0098/6.0105  elevation 0.9949/26.5mm  front 0.9893/26.5mm  plan 0.9940/18.8mm |

### honey-locust

- Failing checks (bounding box axes, IoU and largest boundary distance over all scales and views): **11 before, 0 after**.
- Real `🧊️catalog-silhouette` test: passes (Test Files 1 passed (1); Tests 1 passed | 384 skipped (385)). `validateAsset`: 0 issues. Check CLI: `✓ honey-locust (1/1)`.
- Ops (`🧰️silhouette/SV-1/ops/honey-locust.json`): 9 solids moved or stretched, 29 shapes cut back or rebound, 16 patches added.

| scale | before (width height depth: body/declared; view IoU/largest distance; `!` fails) | after |
|---|---|---|
| 1:50 | 11.0356/11.146! 15.0033/15 12.2600/12.315!  elevation 0.9971/9.6mm  front 0.9963/56.4mm!  plan 0.9956/13.7mm | 11.1460/11.146 15.0033/15 12.3150/12.315  elevation 0.9972/12.6mm  front 0.9963/12.6mm  plan 0.9963/11.4mm |
| 1:100 | 11.0727/11.146! 14.9527/15! 12.1920/12.315!  elevation 0.9966/146.3mm!  front 0.9960/33.5mm!  plan 0.9952/15.5mm | 11.1460/11.146 15.0000/15 12.3150/12.315  elevation 0.9965/18.8mm  front 0.9958/25.2mm  plan 0.9955/20.2mm |
| 1:200 | 11.1460/11.146 15.0000/15 12.3152/12.315  elevation 0.9946/23.7mm  front 0.9939/132.7mm!  plan 0.9933/45.0mm | 11.1460/11.146 15.0000/15 12.3152/12.315  elevation 0.9946/23.7mm  front 0.9939/22.5mm  plan 0.9933/45.0mm |
| 1:500 | 11.1325/11.146 14.9725/15 12.3338/12.315  elevation 0.9895/1242.0mm!  front 0.9942/77.3mm  plan 0.9857/712.7mm! | 11.1325/11.146 14.9725/15 12.3338/12.315  elevation 0.9923/37.5mm  front 0.9942/77.3mm  plan 0.9924/26.5mm |

### hornbeam-hedge

- Failing checks (bounding box axes, IoU and largest boundary distance over all scales and views): **9 before, 0 after**.
- Real `🧊️catalog-silhouette` test: passes (Test Files 1 passed (1); Tests 1 passed | 384 skipped (385)). `validateAsset`: 0 issues. Check CLI: `✓ hornbeam-hedge (1/1)`.
- Ops (`🧰️silhouette/SV-1/ops/hornbeam-hedge.json`): 1 dimension op, 10 patches added, 32 shapes cut back or rebound, 1 shapes removed. Declared dimensions for the manifest: `{"widthM": 3.1882, "heightM": 1.8654}`.

| scale | before (width height depth: body/declared; view IoU/largest distance; `!` fails) | after |
|---|---|---|
| 1:50 | 3.1946/3.14! 1.8590/1.87! 0.7023/0.7  elevation 0.9831/28.1mm!  front 0.9798/21.4mm!  plan 0.9523/30.0mm! | 3.1946/3.1882 1.8590/1.8654 0.7023/0.7  elevation 0.9971/13.1mm  front 0.9863/13.1mm  plan 0.9916/13.1mm |
| 1:100 | 3.1993/3.14! 1.8602/1.87 0.6965/0.7  elevation 0.9848/33.8mm!  front 0.9773/29.3mm  plan 0.9403/33.8mm! | 3.1993/3.1882 1.8602/1.8654 0.6965/0.7  elevation 0.9865/26.3mm  front 0.9822/24.0mm  plan 0.9721/26.3mm |
| 1:200 | 3.1926/3.14! 1.8712/1.87 0.6840/0.7  elevation 0.9859/37.5mm  front 0.9742/32.7mm  plan 0.9359/37.5mm | 3.1926/3.1882 1.8712/1.8654 0.6840/0.7  elevation 0.9859/37.5mm  front 0.9742/32.7mm  plan 0.9359/37.5mm |
| 1:500 | 3.1621/3.14 1.8712/1.87 0.6740/0.7  elevation 0.9782/37.1mm  front 0.9350/53.2mm  plan 0.9348/49.4mm | 3.1621/3.1882 1.8712/1.8654 0.6740/0.7  elevation 0.9782/37.1mm  front 0.9350/53.2mm  plan 0.9348/49.4mm |

### horse-chestnut

- Failing checks (bounding box axes, IoU and largest boundary distance over all scales and views): **9 before, 0 after**.
- Real `🧊️catalog-silhouette` test: passes (Test Files 1 passed (1); Tests 1 passed | 384 skipped (385)). `validateAsset`: 0 issues. Check CLI: `✓ horse-chestnut (1/1)`.
- Ops (`🧰️silhouette/SV-1/ops/horse-chestnut.json`): 1 dimension op, 73 shapes cut back or rebound. Declared dimensions for the manifest: `{"widthM": 19.0432, "depthM": 18.0616}`.

| scale | before (width height depth: body/declared; view IoU/largest distance; `!` fails) | after |
|---|---|---|
| 1:50 | 19.0368/19.047! 24.1675/24.168 18.0656/18.076!  elevation 0.9982/25.5mm!  front 0.9981/28.4mm!  plan 0.9978/25.5mm! | 19.0368/19.0432 24.1675/24.168 18.0656/18.0616  elevation 0.9992/13.1mm  front 0.9993/13.1mm  plan 0.9992/13.1mm |
| 1:100 | 19.0302/19.047! 24.1677/24.168 18.0626/18.076  elevation 0.9978/39.2mm!  front 0.9977/30.2mm!  plan 0.9974/30.0mm | 19.0302/19.0432 24.1677/24.168 18.0626/18.0616  elevation 0.9980/26.3mm  front 0.9980/26.3mm  plan 0.9976/26.3mm |
| 1:200 | 19.0251/19.047 24.1676/24.168 18.0327/18.076!  elevation 0.9978/33.5mm  front 0.9975/37.5mm  plan 0.9963/37.5mm | 19.0251/19.0432 24.1676/24.168 18.0327/18.0616  elevation 0.9978/33.5mm  front 0.9975/37.5mm  plan 0.9963/37.5mm |
| 1:500 | 19.0271/19.047 24.1677/24.168 18.0530/18.076  elevation 0.9983/37.5mm  front 0.9980/37.5mm  plan 0.9966/41.9mm | 19.0271/19.0432 24.1677/24.168 18.0530/18.0616  elevation 0.9983/37.5mm  front 0.9980/37.5mm  plan 0.9966/41.9mm |

### hosta-clump

- Failing checks (bounding box axes, IoU and largest boundary distance over all scales and views): **8 before, 8 after**.
- Real `🧊️catalog-silhouette` test: FAILS (Test Files 1 failed (1); Tests 1 failed | 384 skipped (385)); first failing expectation: `hosta-clump 1:100 elevation: expected 0.9030535174794994 to be greater than or equal to 0.92`. `validateAsset`: 0 issues. Check CLI: `✓ hosta-clump (1/1)`.
- Ops (`🧰️silhouette/SV-1/ops/hosta-clump.json`): none.

| scale | before (width height depth: body/declared; view IoU/largest distance; `!` fails) | after |
|---|---|---|
| 1:50 | 0.9782/0.982 0.6157/0.618 0.9827/0.986  elevation 0.9162!/51.0mm!  front 0.9018!/57.0mm!  plan 0.9982/3.8mm | 0.9782/0.982 0.6157/0.618 0.9827/0.986  elevation 0.9162!/51.0mm!  front 0.9018!/57.0mm!  plan 0.9982/3.8mm |
| 1:100 | 0.9796/0.982 0.6159/0.618 0.9842/0.986  elevation 0.9031!/57.4mm!  front 0.8967!/33.5mm!  plan 0.9977/3.8mm | 0.9796/0.982 0.6159/0.618 0.9842/0.986  elevation 0.9031!/57.4mm!  front 0.8967!/33.5mm!  plan 0.9977/3.8mm |
| 1:200 | 0.9837/0.982 0.6226/0.618 0.9886/0.986  elevation 0.9242/35.4mm  front 0.9207/41.8mm  plan 0.9874/7.7mm | 0.9837/0.982 0.6226/0.618 0.9886/0.986  elevation 0.9242/35.4mm  front 0.9207/41.8mm  plan 0.9874/7.7mm |
| 1:500 | (not measured before: no drawing of this scale) | 0.9844/0.982 0.6180/0.618 0.9893/0.986  (no drawing of this scale: the bounding box only) |

### italian-cypress

- Failing checks (bounding box axes, IoU and largest boundary distance over all scales and views): **2 before, 0 after**.
- Real `🧊️catalog-silhouette` test: passes (Test Files 1 passed (1); Tests 1 passed | 384 skipped (385)). `validateAsset`: 0 issues. Check CLI: `✓ italian-cypress (1/1)`.
- Ops (`🧰️silhouette/SV-1/ops/italian-cypress.json`): 4 solids moved or stretched.

| scale | before (width height depth: body/declared; view IoU/largest distance; `!` fails) | after |
|---|---|---|
| 1:50 | 2.6360/2.636 18.0000/18 2.6221/2.622  elevation 0.9993/1.9mm  front 0.9989/3.8mm  plan 0.9987/2.7mm | 2.6360/2.636 18.0000/18 2.6221/2.622  elevation 0.9993/1.9mm  front 0.9989/3.8mm  plan 0.9987/2.7mm |
| 1:100 | 2.6120/2.636! 18.0000/18 2.6087/2.622  elevation 0.9985/3.8mm  front 0.9988/5.3mm  plan 0.9981/3.8mm | 2.6360/2.636 18.0000/18 2.6087/2.622  elevation 0.9967/22.5mm  front 0.9988/5.3mm  plan 0.9953/19.1mm |
| 1:200 | 2.6160/2.636 18.0000/18 2.5752/2.622!  elevation 0.9973/7.5mm  front 0.9975/7.5mm  plan 0.9961/7.5mm | 2.6160/2.636 18.0000/18 2.6220/2.622  elevation 0.9973/7.5mm  front 0.9939/43.7mm  plan 0.9903/40.4mm |
| 1:500 | 2.6359/2.636 18.0000/18 2.6221/2.622  elevation 0.9938/18.8mm  front 0.9940/18.8mm  plan 0.9921/20.6mm | 2.6359/2.636 18.0000/18 2.6221/2.622  elevation 0.9938/18.8mm  front 0.9940/18.8mm  plan 0.9921/20.6mm |

<!-- /tables -->

## What Stays Red

- **`agave`** (real test: `1:500 elevation: expected 0.912 to be greater than or equal to 0.92`). The three bounding-box failures of 1:50 are gone (declared 2.4916 x 1.447 x 2.3666 m, no geometry changed). Still failing: 1:50 elevation 62.1 mm and front 56.2 mm (limit 15), 1:100 elevation 82.3 mm and front 69.8 mm (limit 30), 1:200 front 75.0 mm (limit 60), 1:500 IoU 0.912 and 0.907. Every leaf is drawn as a shape with a `solid` whose outline is not its profile, so it cannot be cut (1088 refused cuts); the 50 patches the tool could add put the 1:50 front at 949 mm and raised `extent-front-depth`, `extent-front-height` and `volumes-views-disagree`, so they were discarded (a first write of them was taken back; the file is the original plus the dimensions, checked by comparison).
- **`bamboo-clump`** (real test: `1:100 elevation: expected 0.908 to be greater than or equal to 0.92`). The three bounding-box failures of 1:50 are gone (declared 2.974 x 4.5418 x 1.7192 m). Still failing: 1:50 elevation IoU 0.896 and 47.8 mm, front 38.0 mm, plan 62.9 mm; 1:100 elevation IoU 0.908 and 75.0 mm, front 144.8 mm, plan 43.7 mm. The automatic repair (521 patches, 113 cuts) reached 25.5 to 36.9 mm at 1:50 but left 338 print-gap violations at 1:50 and the IoU below the limit; not kept.
- **`date-palm`** (real test: `1:500 elevation Hausdorff: expected 0.2516 to be less than or equal to 0.15`). The eight bounding-box failures are gone (the outermost frond solids of 1:50, 1:100 and 1:200 moved by 25 to 116 mm onto the declared 6.3 x 12.5 x 7 m; no new volume rule, `volumes-connected` unchanged). Still failing: 16 drawing checks, IoU 0.888 to 0.930 in elevation and front, largest distances 68 to 256 mm. Moving the frond tips onto the declared box cost the already failing drawing checks of 1:50 and 1:100 a little (elevation IoU 0.909 to 0.902 at 1:50, plan 83.0 to 102.3 mm at 1:50 and 39.2 to 68.4 mm at 1:100); no check that passed before fails now. The fronds are `solid` shapes (1743 refused cuts); patches made the plan worse (330 mm) and raised `extent-plan-width`; not kept.
- **`hosta-clump`** (real test: `1:100 elevation: expected 0.903 to be greater than or equal to 0.92`). Untouched: all eight failing checks are elevation and front of 1:50 and 1:100 (IoU 0.897 to 0.916, 33.5 to 57.4 mm), every shape is a `solid` shape that cannot be cut (226 refused cuts), one patch changed nothing.
- These four need their leaves redrawn from the volume as 3D shapes (profile and sweep of each `solid` shape fitted to its polyhedron), not a silhouette patch; that is bespoke work per plant and was not done here.

## Requests

None added. The 10 MB limit of the librsvg reference (requested by batch SC) hit four plants of this batch and was lifted by the coordinator during the run.

## Files

Created (repo-relative, under `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS/`):

- `silhouette-SV-1.md` (this report)
- `🧰️silhouette/SV-1/`: `env.sh`, `plants.ts`, `lib.ts`, `trace.ts`, `ops.ts`, `preserve.ts`, `measure.ts`, `locate.ts`, `bbox.ts`, `fix.ts`, `apply.ts`, `validate.ts`, `suit.ts`, `svgsize.ts`, `gate.sh`, `one.sh`, `ship.sh`, `vt.sh`, `look.sh`, `report.py`, `manifest-changes.json`, `ops/<id>.json` for the 18 plants with ops (all but `hosta-clump`)

Updated: `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌳️vegetation/<id>/🔣️.json` for `agave`, `apple-tree`, `bamboo-clump`, `black-alder`, `black-locust`, `common-ash`, `crab-apple`, `date-palm`, `english-oak`, `european-beech`, `field-maple`, `fig-tree`, `ginkgo`, `hawthorn`, `honey-locust`, `hornbeam-hedge`, `horse-chestnut`, `italian-cypress` (18; `hosta-clump` unchanged).

Removed: `🗑️generated/silhouette/SV-1` (tool output).

For the coordinator: `manifest-changes.json` holds new declared dimensions for `agave`, `apple-tree`, `bamboo-clump`, `common-ash`, `english-oak`, `european-beech`, `fig-tree`, `hawthorn`, `hornbeam-hedge` and `horse-chestnut`; `⚡️asset-power` "keeps the manifest in step with the assets" was not run by name filter and will want these rows.
