FINAL (2026-11-28: lossless GLB writer cut of 8 to 10 % (shared buffer views, glTF defaults left out, byte indices, topology-sharing primitives, 9-digit short floats); common-ash, ginkgo, sycamore-maple, honey-locust, lombardy-poplar now pass the 8 MiB budget, shoulders and guide-dog pass with CL-T4; english-oak stays 0.09 MiB over, horse-chestnut and sweetgum need modelling; no test, budget or limit weakened)

# CX-JSON: JSON and structure of the GLB writer

Paths: `R`, `S`, `U` as in the brief, `W` = `U/🧰️matrix/CX-JSON` (tools `anat.ts`, `kinds.ts`, `dup.ts`, `unref.ts`, `split.ts`, `th.ts`, `thr.ts`, `u8.ts`, `glb.ts`, `sizes.ts`, `repin.ts`, `cat.sh`, `t.sh`, `q.sh`).

## 1. Measured before any change

| | horse-chestnut (default pose, 12.10 MB) | parent-with-child-on-shoulders (9.72 MB) |
| --- | --- | --- |
| JSON chunk | 3.07 MB | 2.00 MB |
| nodes / count | 476 KB / 2980 (names 90, extras 129, TRS 130 KB) | 252 KB / 1393 |
| meshes / primitives | 1.54 MB / 4224 (solids extras 607, variant mappings 388, names 70 KB) | 805 KB / 2474 (extras 258, mappings 248 KB) |
| materials | 605 KB / 2005 (names 51, extras titles 182, pbr 198 KB); only 71 distinct bodies | 590 KB / 2009 |
| accessors + bufferViews | 213 KB + 141 KB, 1979 each, one view per accessor | 189 KB + 132 KB |
| animations, skins | none | skins 0.7 KB |
| binary | 9.03 MB: POSITION 5.29, NORMAL 1.59, triangle indices 2.15, line indices 0.13 | 7.71 MB |

Findings: no exact duplicates left (all 1979 accessors distinct, no unreferenced vertices, 0 exact duplicate vertices in horse-chestnut, 609 in shoulders). Index bytes were the lever: the 4.4 MB of per-solid index ranges of horse-chestnut hold only 0.40 MB of distinct patterns counted from the first vertex of the solid (one tessellation pattern drawn at many places, the lobes of a crown). Names, titles, extras and solid ids are read by tests and oracles (`🧪️oracles`, vectors, `🎞️scenes`, model-catalog oracle) and stay.

## 2. Cures (framework `🧰️framework/🔨️modules/🧊️3d/🗿️model/📤️export/🧊️glb`, option `compact`; default encoding byte-identical, 23/23 vectors)

1. Topology sharing (`🧱️parts`, `partitionRigidParts(..., shareTopology)`): a solid whose indices counted from its first vertex repeat in other solids becomes a primitive of its own, so the builder shares one index accessor across all of them, when the shared index bytes outweigh about 320 bytes of JSON per extra primitive (value measured on horse-chestnut: +14 KB JSON for 733 more primitives; thresholds 640, 320, 160 measured on english-oak and lombardy-poplar, 320 is the minimum). Triangles, lines, solid ranges, materials and order unchanged.
2. Shared buffer views (`🧱️buffer`): one view per target and element stride (3 to 5 views instead of 1979), accessors address by `byteOffset`, every vertex attribute view carries `byteStride`. Saves about 105 KB of JSON on horse-chestnut.
3. glTF defaults left out: material `alphaMode` OPAQUE, `doubleSided` false, primitive `mode` 4. About 110 KB.
4. Indices of a primitive below 255 vertices as unsigned bytes (about 70 KB on horse-chestnut).
5. `shortFloat` now also accepts 9 digits (a 32-bit float needs up to 9; before, such factors fell back to 17 digits).
Not done on purpose: material or node dedup (names and bilingual titles differ, read back by tests), shortening names, merging primitives (would break the shared accessors), lossy anything, EXT_meshopt_compression (rejected by the Khronos validator), 16-bit positions (degenerate triangles).

## 3. Tests (TDD)

Fixture `🧰️framework/.../🧊️glb/🧪️tests/🧪️oracles/🧫️fixtures/📦️compact-structure/🔣️.json` (language-agnostic: pattern generator, 3 copies, expected counts) and test "compact encoding leaves out glTF defaults, packs accessors of one kind into shared views and shares the indices of a pattern drawn at many places" next to `📉️compact-normals`: Khronos validator 0 errors 0 warnings, glTF-Transform read-back of every corner, line segment and solid range against the generator, three's `GLTFLoader` triangle count, `transparent` and `DoubleSide`, default encoding keeps every field. Framework `🧊️glb` oracles 21/21 (counting each), vectors 23/23, scenes 10/10.

## 4. Digests and exports

`S/🧫️fixtures/🧊️model-digests/🔣️.json` re-pinned with `W/repin.ts`, GLB part, `byteLength` and `sha256` only of the six poses `stand`, `stride-left-forward`, `passing-left-stance`, `stride-right-forward`, `passing-right-stance`, `carry-bag` (e.g. stand 1,948,564 to 1,877,244). Reason: shared views, left-out defaults, byte indices, short decimals. Triangles, segments, primitives, nodes, meshes, FBX digests unchanged. `🧊️model-digests` and `🧊️model-exports` 61/61 pass.

## 5. Result (largest default-set GLB pose in MiB; before = CX-FLOW report / coordinator numbers, after = this code)

| Asset | Before | After | Note |
| --- | --- | --- | --- |
| horse-chestnut (default pose) | 12.10 (poses 11.2 to 11.6 per catalog) | 11.15 at the first cut, poses 10.3 to 10.7 in the catalog run | Fable lane CL-T5 remodels it |
| sweetgum | 9.4 to 9.7 | 9.3 to 9.5 (default pose 9.29) | body only, CL-T5 |
| parent-with-child-on-shoulders | 9.3 to 9.4 | 8.6 to 8.7 with this cut; 7.19 to 7.27 after CL-T4 | pass |
| person-with-guide-dog | 8.1 to 8.2 | 6.81 to 6.89 (CL-T4) | pass |
| common-ash | 8.0 to 8.2 | 7.69 | pass |
| ginkgo | ~8.0 | 7.10 | pass |
| sycamore-maple | 8.1 to 8.3 | 6.87 | pass |
| honey-locust | 8.7 to 8.8 | pass (catalog `F`, calm 8.0 before the last threshold) | pass |
| lombardy-poplar | 8.0 to 8.1 | calm 7.98 (threshold run), catalog `G` pass | pass |
| english-oak | 8.4 | 8.07 (calm), 8.09 (windy), 8.07 (storm) | over by 0.09 MiB |
| norway-maple, tulip-tree, dairy-cow, tower-crane, city-car, external-venetian-blind | | catalog run `I`: 8/8 pass | controls |
Catalog runs `A` to `J` in `W`'s generated folder (deleted at the end). Every other check of the catalog (Khronos, three, glTF-Transform, FBX against GLB triangles, extents, closed, json) passed for all assets; the only findings were `budget pose:*` sizes. One run (`H`, sycamore-maple) died by a 900 s test timeout under machine load; sycamore-maple was re-measured with `sizes.ts` (6.87).

## 6. Byte breakdown for what stays over (for the Fable lanes; MB = 10^6 bytes, default pose)

- english-oak calm 8,465,036 B (8.07 MiB; limit 8,388,608): JSON 3,498,700 (nodes 486 KB / 2877, meshes 2,005 KB / 6090 primitives of which solids extras 677 KB and variant mappings 620 KB, materials 497 KB / 1909, accessors 422 KB / 3762, scenes 76 KB), binary 4,966,308 (POSITION 2.91, NORMAL 0.80, triangle indices 0.70, line indices 0.56). It is JSON-heavy (43 %): 76 KB fewer solids or primitives per mode, or fewer feature-edge lines, closes it.
- sweetgum calm 9,743,556 B: JSON 2,411,840, binary 7,331,688 (POSITION 4.72 MB in 684 accessors, NORMAL 1.44, triangle indices 1.15); body geometry owns 5.79 MB, structure 1.18, flow 0.27, thermal 0.09. Needs about 1.4 MB less body.
- horse-chestnut (first cut) 11,150,840 B: JSON 3,087,816, binary 8,062,996 (POSITION 5.29 MB, NORMAL 1.59, indices 1.18). Needs about 2.4 MB less body.
- no further lossless cure found: positions and normals are unique per solid, the remaining index accessors are distinct patterns.

## 7. ETA

Shared writer work is finished. What is left is modelling (CL-T5: horse-chestnut, sweetgum, small-leaved-lime; english-oak 0.09 MiB). After a modelling change, one `sizes.ts` run per asset (about 10 minutes of queue time) re-measures it.

Files: `🧰️framework/🔨️modules/🧊️3d/🗿️model/📤️export/🧊️glb/🟦️.ts`, `.../🧱️buffer/🟦️.ts`, `.../🧱️parts/🟦️.ts`, `.../🧪️tests/🧪️oracles/🟦️.ts`, `.../🧪️tests/🧪️oracles/🧫️fixtures/📦️compact-structure/🔣️.json`, `S/🧫️fixtures/🧊️model-digests/🔣️.json`, `U/requests.md`.
