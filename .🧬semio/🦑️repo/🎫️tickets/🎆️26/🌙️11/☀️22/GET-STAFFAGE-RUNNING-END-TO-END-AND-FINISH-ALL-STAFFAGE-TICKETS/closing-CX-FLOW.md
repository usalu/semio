FINAL (2026-11-27 11:00: flow puffs as instances, 8-vertex tubes and arrow heads, welding and short decimals in the GLB writer; norway-maple, red-oak, tulip-tree, field-maple now pass the 8 MiB GLB budget, common-ash, ginkgo, sycamore-maple are within 0 to 4 %, honey-locust, sweetgum, horse-chestnut, parent-with-child-on-shoulders, person-with-guide-dog stay over because their bytes are body and structure; european-beech goes to Fable; no test, budget or limit weakened)

# 🌊️ CX-FLOW: GLB size of the trees, flow geometry and writer

Paths: `R`/`S`/`U` as in the brief, `W` = `U/🧰️matrix/CX-FLOW` (tools: `glb.ts`, `own.ts`, `jsonparts.ts`, `weld.ts`, `sizes.ts`, `repin.ts`, `cat.sh`, `q.sh`).

## 1. Measured (norway-maple, default pose, before any change: 10.38 MiB; horse-chestnut 13.47 MiB)

norway-maple bytes: JSON 1.68 MB (materials 348 KB, nodes 257 KB, meshes 772 KB of which `extras.solids` 349 KB and variant mappings 174 KB, accessors+views 258 KB), binary 9.20 MB: POSITION 5.0, NORMAL 0.9, indices 3.3 (all 16-bit, no accessor shared twice left).
By group (first owner): body 4.42 MB, structure 0.77 MB, thermal-own 0.12 MB, **air flow 3.88 MB** (vapour puffs 1.90 MB for 593 puffs, stream tubes and arrows 1.98 MB).
Flow triangles over the four scales: puffs 164,800, tubes 134,660, arrows 38,933 (about 338,000).
horse-chestnut: JSON 3.26 MB, binary 9.4 MB (body 7.4 MB, structure 1.0 MB, flow 0.8 MB after the cure).
Not a cure (measured): float quantization (forbidden), u32 indices (none), bufferView merging (about 65 KB), unread-normal welding alone (about 0.2 MB).
Considered and dropped: `EXT_meshopt_compression` (lossless, would cut the binary by 44 %: 9.2 -> 5.2 MB on norway-maple with the reference encoder). The installed Khronos validator (2.0.0-dev.3.10) does not know the extension, and a required extension it does not know is an error, so the GLB would stop being Khronos-valid. Left as an open decision for the coordinator.

## 2. Cures (shared code)

1. **Puffs are instances** (`S/🔨️modules/🧊️volume/🌊️flow`): a vapour or fog puff is the unit icosphere on a node of its own (`flow-vapour-<i>`, translation = centre, uniform scale = radius). Every puff of one sphere has the same vertices, the writer stores them once. norway-maple flow-vapour bytes: 1.90 MB -> 4.8 KB (JSON +0.19 MB for 593 nodes and meshes). Triangles per puff unchanged (80 / 320), so the triangle count of a scene is unchanged by this. Proof of geometry: `🌊️flow-model` measures the world vertices (translation + scale applied) against the numpy/trimesh oracle (volume, area, radius), and three's GLTFLoader reads every puff node centred on the puff with its clamped radius, all puffs of a scene on at most two vertex accessors.
2. **Tube rings and arrow heads have 8 vertices** (`segments` on `loft` and `revolve`, schema `semio.model/v1`, validation `invalid-dimension` below 3, tessellation uses it instead of the quality rule). Reason: a tube is at most 0.15 mm of paper in radius, a head 0.375 mm; an octagon deviates 0.011 and 0.029 mm of paper from its circle, within the 0.08 mm chord that the tube axis already keeps (`FLOW_MODEL.tubeSegments`, asserted per tube and head in `🌊️flow-model`). The quality rule gave 12 to 24 vertices. Triangles of tubes and arrows fall by about half; the thermal triangle count of every flowing asset falls (this is a change in counts, by design; monotony over the scales is still asserted and green). Proofs run: framework `🧊️glb` oracles 20/20 and vectors 23/23, `🧵️loft` 97, `🌀️revolve` 30, validation 49, `🌊️flow-model` 12/12, `🧊️model-glyph-size` 8/8, `🧊️model-digests` 43/43, `🧊️model-exports` 18/18 (earlier code state). Catalog runs: A (field-maple, ginkgo, red-oak, tulip-tree, norway-maple, sycamore-maple), B (honey-locust, common-ash, sweetgum, horse-chestnut, european-beech, weeping-willow) on the code before the short bounds, C (ginkgo, sycamore-maple, common-ash, honey-locust, sweetgum, horse-chestnut) and D (the other 10 assets) on the final code. Framework tests: fixtures `🧵️loft` and `🌀️revolve` case `explicit-segments` (volume of the regular octagon prism, bounds, triangle count 48, against three's `CylinderGeometry` with 8 radial segments).
3. **GLB writer, option `compact`** (`🧰️framework/🔨️modules/🧊️3d/🗿️model/📤️export/🧊️glb`): (a) vertices that differ only in an unread normal are welded inside each solid of an all-unlit primitive, lines and solid ranges keep their corners and order (`🧱️weld`; flow tubes 69,640 -> 52,159 vertices on norway-maple); (b) colour factors, node transforms and accessor bounds are the shortest decimals of the same 32-bit floats (about 0.1 MB on a big tree, validator 0/0). Default encoding is byte-identical (23/23 vectors). Tests with the Khronos validator, glTF-Transform and three: `🧪️oracles` (21 tests), fixtures `📉️compact-normals` (short floats) and `🧷️welded-unread`.
4. Tried and removed: partition of skinned parts for accessor sharing (framework test passed, no effect on the skinned assets measured).

## 3. Digests

`S/🧫️fixtures/🧊️model-digests/🔣️.json` re-pinned twice with `W/repin.ts`. Reasons: (1) GLB `byteLength`, `sha256`, `triangles`, `segments`, `primitives`, `nodes`, `meshes`, `accessors`, `scenes` (fewer triangles of tubes and arrows, a node and mesh per puff, welded vertices); FBX `byteLength`, `sha256`, `objectCounts` (a Model and Geometry per puff) and `notices` (`solids-as-property` per puff node); (2) after the short decimals only GLB `byteLength` and `sha256`. `🧊️model-digests` 43/43 green (Khronos validator, three, glTF-Transform, FBXLoader, assimp).

## 4. Result per asset (largest GLB pose in MiB, `🧊️model-catalog` single-asset runs, final code)

| Asset | Before | After | Catalog run |
| --- | --- | --- | --- |
| norway-maple | 10.4 (10.2 to 10.6) | 7.65 | green |
| red-oak | 10.2 | 7.17 | green |
| tulip-tree | 10.4 | 7.46 | green |
| field-maple | 8.1 | 6.13 | green |
| weeping-willow | 10.7 (before CX-GLB) | 4.84 | green |
| common-ash | 11.1 | 8.0 to 8.2 | only `budget pose:*` (5 poses, 8.0 to 8.2 MB) |
| ginkgo | 9.9 | 8.0 | only `budget` (2 poses, just above 8 MiB) |
| sycamore-maple | 10.1 | 8.1 to 8.3 | only `budget` (5 poses) |
| honey-locust | 10.7 | 8.7 to 8.8 | only `budget` (4 poses) |
| sweetgum | 11.7 | 9.4 to 9.7 | only `budget` (5 poses) |
| horse-chestnut | 13.5 (up to 14.0) | 11.2 to 11.6 | only `budget` (6 poses) |
| european-beech | 36.7 (asset not rewritten) | 36.7 | triangles 1.84 M body / 1.96 M thermal at 1:50, GLB 36.7 MB |
| parent-with-child-on-shoulders | 9.4 | 9.3 to 9.4 | only `budget` (3 poses) |
| person-with-guide-dog | 8.2 | 8.1 to 8.2 | only `budget` (3 poses) |
| dairy-cow | 8.1 (one pose) | 6.13 | green |
| tower-crane | 8.5 to 8.6 | under 8 | green |
| city-car | 6.4 | under 8 | green |
| multi-stem-birch | 10.9 | under 8 | green |
| fig-tree | 9.5 | under 8 | green |
| japanese-maple | 8.7 | under 8 | green |
| atlas-cedar | 8.5 | under 8 | green |
| windmill-palm | 8.3 | under 8 | green |

For the six trees that stay over, every other check of the catalog run (validator, three, glTF-Transform, FBX against GLB triangles, extents, closed, json) passes: the only findings are the GLB size.

## 5. What is left, with numbers

- Trees over: common-ash, ginkgo, sycamore-maple miss by 0 to 4 %, honey-locust by 10 %, sweetgum by 21 %, horse-chestnut by 45 %. After the flow cure their bytes are body: horse-chestnut body 7.4 MB, structure 1.0 MB, JSON 3.3 MB (materials 652 KB, nodes 565 KB, meshes 1.5 MB). Shared code cannot cut the body without loss; the remaining levers are (i) authored foliage (fewer lobes, as CL-T1 measured, breaks the silhouette guard), (ii) `EXT_meshopt_compression`, which is lossless and cuts about 44 % but needs a validator that knows it, (iii) merging bufferViews (about 65 KB).
- parent-with-child-on-shoulders (poses 9.30, 9.39, 9.38 MiB) and person-with-guide-dog (8.17, 8.10, 8.18): no flow share. Breakdown of shoulders `shoulder-carry` (9.30 MiB): binary 7.71 MB = structure scenes 3.43 MB (421,372 triangles of member and force glyph geometry), body 2.53 MB, thermal skinned geometry that is not shared with the body 1.64 MB, flow 0.11 MB; JSON 2.04 MB (materials 590 KB). The cure there is the skinned structure and thermal geometry (duplicated per scene because skinned meshes are grouped per scene and material), not the flow.
- european-beech: the catalog file is untouched (1.84 M body triangles at 1:50). On the CL-T1 trial copy (reference facets) the thermal scene was 525,508 triangles with 138,000 of air flow; the budget counts the flow (the audit counts every solid of the scene). With the cures the flow at 1:50 is 66,528 triangles (puffs 42,240 + tubes 20,704 + arrows 3,584 measured on the catalog file); the trial copy is therefore about 454,000, still over 400,000 by about 54,000, because its thermal scene repeats its 380,000 body triangles. **Assigned to a Fable modelling follow-up**: the body at 1:50 must fall to about 330,000 triangles (400,000 minus 66,500 of flow minus margin). The asset is not rewritten.
