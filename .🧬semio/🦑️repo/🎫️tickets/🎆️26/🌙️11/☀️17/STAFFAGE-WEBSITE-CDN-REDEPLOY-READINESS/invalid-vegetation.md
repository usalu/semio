# Invalid vegetation assets: causes and fixes

All nine assets now report zero `validateAsset` issues (`validate-some.ts`, validator unchanged).

| asset | cause | fix |
|---|---|---|
| japanese-maple, lombardy-poplar, london-plane-pollarded, multi-stem-birch, scots-pine | `volumes` missing, so the schema and the glb/fbx exports failed | Spliced the handcrafted per-scale volumes from the earlier authoring ticket (`PERFECT-HANDCRAFTED-3D-MODELS-...`, `volumes/vegetation/<slug>.json`) with its `volumes-insert.py`, which also drops the old `volume` member |
| weeping-willow | `volumes` missing; 60 elevation shapes `e-c*` set `followTip` (valid for front and plan only) | Spliced volumes; removed `followTip` from those elevation shapes at 1:50, 1:100, 1:200 |
| miscanthus-grass | `volumes` missing; 1:200 plan depth 1.753 m against 1.8 m | Spliced volumes; widened the tip radius of the two depth-extreme blade capsules (`l1-2-b`, `r1-2-b`) from 0.028 to 0.04 m in the 1:200 plan (depth about 1.78 m) |
| small-leaved-lime | `volumes` missing; after splicing: 1:500 root-plate 2 mm thin, 1:500 `member-epi-w` 22.5 mm long, no solid for region `epicormic-stem` at 1:100 and 1:200 | Root-plate radius 0.1 m, member length 0.3 m at 1:500; 1:100 epicormic wood solids moved to region `epicormic-stem`; the same two solids added to 1:200 with thermal references (source JSON in the authoring ticket updated too) |
| date-palm | structure box `member-fr-r1b` 66 mm thin at 1:500 | Cross-section 0.066 to 0.0825 m, equal to sibling `member-fr-r1a` |
