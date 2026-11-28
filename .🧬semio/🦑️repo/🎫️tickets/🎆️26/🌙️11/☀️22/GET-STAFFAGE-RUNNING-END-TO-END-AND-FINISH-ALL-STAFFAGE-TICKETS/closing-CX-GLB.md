FINAL (2026-11-27: GLB writer cured by 8-bit normals and unread-normal omission; 107 budget findings cut to tower-crane x3 poses, dairy-cow x1 pose and the trees; position quantization measured and dropped)

# CX-GLB: GLB over 8 MB

## Measured (person-with-rollator `stand-at-grips`, 9.03 MB, 12 scenes = 4 scales x body/thermal/structure)
- POSITION 3.06 MB, NORMAL (float32) 2.65 MB, triangle indices 1.59 MB (already 16-bit), line indices 0.12 MB, JSON chunk 1.6 MB.
- Sharing is already complete: 0 byte-identical accessors left, meshes and accessors shared across modes and scales, lines share the triangle positions. Only 17 position accessors (61 KB) are verbatim sub-ranges of another one; tower-crane 35 (338 KB), dairy-cow 0. Not exploited (needs accessor byteOffset search, gain under 4 %).
- Fully flat normals: 4.6 KB (nothing to drop). Normals of primitives whose every material (base and all variants) is unlit: 8.9 % rollator, 3.8 % city-car, 30 % dairy-cow.
- JSON: float32-shortest numbers would save 6 % (100 KB), extras.solids (up to 708 KB on tower-crane) is specified as written unchanged, so JSON was left alone.
- tower-crane per scale/mode unique bytes (parked-weathervane, normals-only writer): s50 body 154 KB / thermal 252 KB / structure 366 KB; s100 110 / 184 / 243 KB; s200 17 / 99 / 118 KB; s500 37 / 58 / 99 KB; shared between modes 4.6 MB (1.3 MB by 2 scales-modes, 3.2 MB by 3).
- dairy-cow: s50 thermal 935 KB / structure 1340 KB; s100 563 / 439 KB; s200 295 / 191 KB; s500 172 / 121 KB; shared 2.4 MB. Positions 3.5 MB are float32 because a small skinned part (JOINTS_0/WEIGHTS_0) exists.

## Cure (framework `🧰️framework/🔨️modules/🧊️3d/🗿️model/📤️export/🧊️glb/`, option `compact`, set by `S/🔨️modules/📤️export/🧊️glb/🟦️.ts`)
- NORMAL as normalized signed bytes (VEC3, 4-byte elements, `byteStride` 4) with `KHR_mesh_quantization` in extensionsUsed and extensionsRequired. 8 bits is the gltfpack default; angular error under 0.45 degrees (test tolerance 0.012 rad).
- NORMAL omitted on triangle primitives whose material and every variant-bound material are unlit.
- `BufferBuilder`: byte and short components, `normalized`, `aligned` (stride), exact-compare sharing kept (also for strided data). Bug found on the way: `-0` after rounding broke exact-match sharing; fixed with `+ 0`.
- Default encoding (no `compact`) is byte-identical to before: all 23 framework vectors unchanged.
- Triangle counts per scene unchanged (only attribute encoding changes).
- Oracle change (not a weakening): `🧪️tests/🧊️model-catalog/🔍️oracles/🟦️.ts` registers `KHRMeshQuantization` on glTF-Transform's `NodeIO`, because a required extension that is not registered makes `NodeIO` throw ("Missing required extension"). three's GLTFLoader reads it without change. Khronos validator: 0 errors, 0 warnings on every checked file.
- Test + feature: `.../🧊️glb/🧪️tests/🧪️oracles/🟦️.ts` ("compact encoding writes quantized normals only where a lit material can read them and shares identical ones"), fixture `🧪️tests/🧪️oracles/🧫️fixtures/📉️compact-normals/🔣️.json`, scenarios in `.../🧊️glb/🥒️.feature`. Covers layout, required extension, validator 0/0, glTF-Transform and three read-back within tolerance, unlit-only primitive without NORMAL, lit-bound-by-variant keeps NORMAL, identical normals and positions share one accessor, default encoding unchanged.

## Position quantization: tried and dropped
16-bit positions on one power-of-two grid with a per-scene scale node cut tower-crane to 6.9 MB, city-car 6.4 MB, rollator 6.2 MB and the suite passed, but it is lossy: grid step 1.95 mm (tower-crane, worst error 0.98 mm), 0.49 mm (city-car, 0.24 mm), 0.12 mm (rollator, 0.06 mm); the step is per document, not per scale. It created zero-area triangles that float32 does not have: tower-crane 7868, city-car 162, rollator 4. Not provably lossless, so removed (coordinator rule). Skinned assets (dairy-cow) could not use it anyway.

## Proof
- Framework `🧊️glb` oracles 18/18 and vectors 23/23 pass (repo test-file).
- `🧊️model-exports` 18/18 pass. `🧊️model-digests` 43/43 pass after the re-pin below.
- Digest re-pin `S/🧫️fixtures/🧊️model-digests/🔣️.json`, GLB part only: the six poses `stand`, `stride-left-forward`, `passing-left-stance`, `stride-right-forward`, `passing-right-stance`, `carry-bag` changed in `byteLength` (e.g. stand 3,435,372 to 2,620,856), `sha256`, `extensionsUsed` (adds KHR_mesh_quantization) and `accessors` only. Nodes, meshes, materials, skins, scenes, triangles, segments and every FBX digest are unchanged. Reason: normals are 8-bit and unlit-only primitives lost their NORMAL.
- Single-asset model-catalog runs with the final writer (all checks incl. glb, three and glTF-Transform reads, FBX versus GLB triangle equality "set", extents, closed, json):
  - person-with-rollator (person): pass
  - tennis-player (person): pass
  - city-car (vehicle): pass
  - atlas-cedar (tree, 8.5 MB before): pass
  - tower-crane (machine): only `budget pose:lifting-load 8.5 MB`, `max-reach 8.6 MB`, `skip-at-ground 8.5 MB`
  - dairy-cow (animal): only `budget pose:grazing-head-down 8.1 MB`
  Every other check of these seven assets passes, so quantized normals do not disturb the FBX/GLB triangle comparison, extents or readers.

## Still over 8 MB (the budget is not raised)
- tower-crane: lifting-load 8.5, max-reach 8.6, skip-at-ground 8.5 MB (about 0.6 MB too much). For a later lane (fewer vertices at 1:50 thermal/structure, or lines).
- dairy-cow: grazing-head-down 8.1 MB (about 0.1 MB). For a later lane.
- Trees, for CL-T1 (sizes before the cure, 8-bit normals cut about 1 MB of them, far from enough): norway-maple 37.4, black-alder 25.3, red-oak 23.7, rowan 20.3, crab-apple 17.7, saucer-magnolia 15.4, european-larch 13.8, multi-stem-birch 10.9, weeping-willow 10.7, fig-tree 9.5, shrub-rose 8.7, japanese-maple 8.7.
- Not re-run, expected to improve by 1 to 2 MB and possibly pass: parent-carrying-toddler (12.0), basketball-player (10.2), worker-carrying-box (9.4), person-pushing-bicycle (9.0), tram (8.6), standing-desk-worker, person-sweeping, parent-with-stroller (8.6), windmill-palm (8.3), waiter-with-tray (8.3), person-in-winter-coat (8.1), plus the other assets of shards still running. Re-run their model-catalog to see what is left.

Tools: `U/🧰️matrix/CX-GLB/` (measure, analyze, loss, repin scripts).
