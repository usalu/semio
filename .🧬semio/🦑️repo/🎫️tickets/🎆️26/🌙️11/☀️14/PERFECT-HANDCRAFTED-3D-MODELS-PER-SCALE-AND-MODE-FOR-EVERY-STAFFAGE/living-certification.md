# Plant and animal model verification

Recorded on 2026-11-21T16:43+01:00. Each row covers body, thermal and structural models at 1:50, 1:100, 1:200 and 1:500. All 30 passed the per-solid validator, rig and region associations, printable features, slenderness and the product’s cross-scale geometry-copy check on the hashes listed below. The geometric scene budget is 400,000 triangles and the presentation file limit is 8,388,608 bytes.

The strict column includes silhouette agreement, physical bounds, union volume and exposed area. The export column includes the independent GLB reload. A preceding-code measurement used the same catalog geometry. The checkpointed audit repeats it when relevant product code changes. Earlier results without a checkpointed code signature are shown only where the size budget was explicitly measured.

| Asset | Catalog SHA-256 prefix | Strict volume result | Measured presentation GLB |
|---|---|---|---|
| agave | `09d04e6dcd43` | Pass | 7,841,248 bytes, preceding code |
| apple-tree | `22da0dbd3167` | Pass | 4,644,648 bytes, preceding code |
| bamboo-clump | `bfee35702485` | Pass on preceding code | Queued |
| dairy-cow | `447ff05f10ec` | Pass | 7,423,780 bytes, preceding code |
| date-palm | `ffda6f891e83` | Pass on preceding code | Queued |
| domestic-cat | `3611bf9b3367` | Pass | 3,997,596 bytes, preceding code |
| english-oak | `386417ae6432` | Pass | 6,121,264 bytes |
| goat | `16bf18cde2ee` | Pass | 5,356,900 bytes, preceding code |
| hedgehog | `a9e8a4babf4b` | Pass | 4,289,240 bytes, preceding code |
| herring-gull | `2398618bd66e` | Pass | 2,838,568 bytes, preceding code |
| hornbeam-hedge | `8f3ea2fb3a60` | Pass | 3,533,448 bytes, preceding code |
| horse | `268822512914` | Pass | 7,205,572 bytes, preceding code |
| hosta-clump | `404a79322758` | Pass | 7,441,416 bytes |
| ivy-on-trellis | `1566b5e367b5` | Pass | 6,990,104 bytes, preceding code |
| japanese-maple | `ec5bc737a45d` | Pass | 3,748,708 bytes, preceding code |
| labrador-dog | `298115c3e168` | Pass | 5,026,648 bytes, preceding code |
| lombardy-poplar | `055cbf34fb43` | Pass | 6,147,256 bytes |
| london-plane-pollarded | `51d02bb75ccf` | Pass | 4,583,016 bytes, preceding code |
| mallard-duck | `df195859feee` | Pass | 2,746,344 bytes, preceding code |
| miscanthus-grass | `a21ab0f43079` | Pass | 4,795,516 bytes |
| multi-stem-birch | `c14b7f870ee9` | Pass | Queued |
| pig | `a332d7f66f6e` | Pass | 4,170,564 bytes, preceding code |
| pigeon | `489afa4efa0e` | Pass | 2,527,216 bytes, preceding code |
| red-fox | `5bc3cf66c551` | Pass | 4,128,408 bytes, preceding code |
| red-squirrel | `8ff3dacfd4c0` | Pass | 3,559,996 bytes, preceding code |
| roe-deer | `0396319eb4d4` | Pass | 4,386,252 bytes, preceding code |
| scots-pine | `6916c561558a` | Pass | 4,097,644 bytes, preceding code |
| sheep | `2ea368a271ba` | Pass | 3,863,760 bytes, preceding code |
| small-leaved-lime | `8c2fa916d4f0` | Pass | 5,189,204 bytes, preceding code |
| weeping-willow | `8e2bc7f15a50` | Pass | Queued |

At this checkpoint, the exact catalog documents in the table have 30 complete strict passes and 26 budgeted GLB passes. Of these, 28 strict and 4 GLB checks also match the current test and product code signature, including 4 that match in both columns. All file sizes come from the presentation-pose GLB binary. The source-only triangle and binding checks are recorded in [the living model audit](living-current-audit.md).
