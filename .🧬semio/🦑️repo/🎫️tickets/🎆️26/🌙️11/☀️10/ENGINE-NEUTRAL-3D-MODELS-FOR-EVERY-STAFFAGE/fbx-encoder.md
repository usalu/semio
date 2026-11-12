# FBX Encoder — `encodeFbx` (binary FBX 7.4)

Scope: `🧰️framework/🔨️modules/🧊️3d/🗿️model/📤️export/🎬️fbx/` (called `F` below).

## Files

| Path (under `F`) | Content |
|---|---|
| `🟦️.ts` | `encodeFbx(document, { animation?, variant?, format: "binary" }, operation) → Uint8Array`, `encodeFbxDetailed(…, compressor?) → { bytes, notices }`, `FbxCancelledError`, `FbxEncodeError` (stable `code`, English `message`, `german`) |
| `🗜️deflate/🟦️.ts` | owned zlib: LZ77 hash chains, dynamic or fixed Huffman block (the smaller), Adler-32, owned inflate; `Compressor` interface behind which any zlib producer can stand |
| `🧱️records/🟦️.ts` | node records, typed properties, `Properties70` helpers, binary serialiser (header, 13-byte sentinels, footer) |
| `🧮️matrix/🟦️.ts` | column-major matrices, quaternions, FBX Euler XYZ (`Rz*Ry*Rx`), Euler unwrapping incl. the gimbal-equivalent triple, sRGB encode |
| `🦴️mechanism/🟦️.ts` | joint → `PreRotation` + `Rotation*/Translation*` limit properties, notices |
| `🎞️animation/🟦️.ts` | per-axis curves, unwrapped Euler rotation, slerp subdivision of arcs > 15° (linear channels), colour curves |
| `🎨️material/🟦️.ts` | `Material` children (Phong lit, Lambert unlit) |
| `📖️reader/🟦️.ts` | owned binary FBX reader plus the digests: `outline`, `objectCounts`, `connectionList` |
| `🧪️tests/📦️structure`, `🗜️deflate`, `🧍️three-roundtrip`, `🔍️second-reader`, `🚫️rejections` | five test files, 36 tests; `🧪️tests/🧰️support/🟦️.ts` and `🧰️support/🧩️assimpjs/🟦️.ts` helpers |
| `🧫️fixtures/🧍️rigged-arm`, `🧊️static-cube`, `🎞️rotation-continuity`, `🚫️procedural`, `🗜️deflate` (each `🔣️.json`) | language-agnostic vectors: document + cases with byte length, SHA-256, object counts, resolved connection list, full record outline, notices; deflate vectors with input and zlib SHA-256 |

Outside `F`: root `package.json` (+1 line `assimpjs` 0.0.10) and `bun.lock` (+workspace line, +package entry), and `node_modules/assimpjs` (copy of the npm package). `T/fbx-fixture-source.ts` generates the fixtures; `T/vitest.fbx.config.ts` and `T/tsconfig.fbx.json` are the direct vitest and type-check configs.

## Test command and result

- The core project `@semio-tech/framework-3d-model-js` had no `📋️project.json`/config on disk at the end (its `📦️packages` and `🧪️tests/🎚️config` had disappeared), and `repo show projects` queued for minutes on the memory gate. So `repo run` was **not** used. No glob line of mine was added to a core config because none exists.
- Used: `bun node_modules/vitest/vitest.mjs run --config T/vitest.fbx.config.ts` from the repo root (bun = pinned runtime of `repo tools list`). Result: 5 files, 36 tests, 36 passed, 0 failed. `tsc -p T/tsconfig.fbx.json`: 0 errors.
- Once the core project exists, its glob needs `../../📤️export/🎬️fbx/🧪️tests/*/🟦️.ts` plus the exclusions `🧪️tests/🧰️support/🟦️.ts` for these cases.
- Regenerate fixtures after an intended encoder change: `bun T/fbx-fixture-source.ts` (run from repo root).

## Oracle results

- Owned deflate vs `node:zlib` and `fflate`: all vectors plus about 100 pseudo-random sizes up to 70 kB inflate byte-identically in both; both fixed and dynamic blocks are produced; stored blocks never.
- three `FBXLoader` (0.182) round trip, all 6 cases: node hierarchy and world matrices (incl. PreRotation), bones, positions and normals per triangle corner (flat normals computed with three's `Triangle` where the primitive has none), vertex colours, material groups and colours/opacity/emissive/Phong vs Lambert, skeleton bone order, bind matrix, bone inverses, per-vertex skin weights, animation tracks at all source key times plus slerp midpoints of rotation segments.
- assimp 5-series through `assimpjs` (npm, WebAssembly) as the second independent reader: node tree (incl. its generated PreRotation chain), world matrices, triangle multiset with winding, bone names and weight sums, material colours/opacity, animation keys. It accepted every case. Found and fixed through it: boolean `P` entries must be 32-bit integers, and an `AnimationLayer` needs a child scope (otherwise assimp silently drops all animation).
- Blender, `ufbx`, Autodesk FBX SDK/converter: not installed; `fbx2gltf` (npm wrapper of the Autodesk SDK based FBX2glTF) is installable but its download was not attempted. Blender's exporter conventions (file id, footer constants, `KeyAttr*`) were taken from its public `encode_bin.py` and `fbx_utils` knowledge, not tested against Blender itself.

## Deviations from design.md and decisions

- Extra export `encodeFbxDetailed` returns notices (`lines-skipped`, `joint-axis-skipped`, `unlit-approximated`, `events-dropped`, `material-animation-unbound`, `alpha-animation-skipped`, `channel-duplicate`); `encodeFbx` is its byte projection.
- `lines` primitives are skipped with a notice (three would triangulate a two-vertex polygon into NaN positions; no portable FBX line primitive).
- Colours (material, emissive, vertex colour, colour curves) are written sRGB-encoded from the model's linear values; alpha is `Opacity`/`TransparencyFactor`. Metallic/roughness only approximate through Phong `Shininess`/`SpecularColor`.
- Material colour channels animate only materials of the exported variant (literal id); others are skipped with a notice. Curve nodes are named `Surface DiffuseColor`/`Surface EmissiveColor` (contains an `S`) because three's `FBXLoader` crashes on any curve whose node name it does not register. three still logs two benign warnings for the colour links (reads them as texture links).
- Limited joints (revolute, prismatic, spherical, fixed) carry the rest rotation as `PreRotation` with zero `Lcl Rotation`, so limits are relative to rest; animation rotation is divided by the pre-rotation. Only principal axes can be expressed; others get no limit and a notice. Spherical swing is approximated by ± cone angle on the two perpendicular Euler axes.
- Skin Cluster `Transform` = world matrix of the mesh node at rest, `TransformLink` = that matrix times the inverse of the stored inverse-bind. Bind `Pose` lists the mesh and the joints (first skin wins for a shared joint). A skin shared by several mesh nodes yields one Skin per mesh node.
- Ids are content-hashed (FNV, collision-probed); creation time stamp, file id and footer constants are fixed, so bytes are deterministic. `InheritType` 1, rotation order XYZ, `UnitScaleFactor` 100 (metres), Y up, Z front.

## Open issues

- `assimpjs` is added to the root manifest and lock by hand like `gltf-validator`; `🔒️dependencies.json` (generated) was not regenerated, and the module is not in the dependency-owner list of a project. A coordinator should run the dependency lock and declare the test dependency in the core project's package.
- assimp's JSON export reports identity bone offset matrices for any input (verified by altering the input), so bind matrices are verified by three only. assimp merges PreRotation into the animated rotation keys of limb nodes; the test accepts the keys as equal to the source quaternion (observed behaviour).
- The test lists fixture cases by folder name part, so the fixtures' emoji folder names are looked up on disk.
- `Takes` carries one take; events, clip `loop` and variant titles are not representable and only reported (events) or ignored.

## Maintenance update (A5, `kernel-maintenance.md`)

- FBX geometry has no named sub-parts, so the solid provenance of a mesh's triangle primitives is kept as the user property `SolidRanges` (`KString`, flag `U`) of the mesh `Model`: `id:firstPolygon:polygonCount` entries joined by `;` over the polygons of the merged geometry; notice `solids-as-property` per such mesh node. Vectors regenerated with `T/fbx-fixture-source.ts`; 38 tests pass; three `FBXLoader` and assimp still load every case (assimp's JSON export shows no node metadata, so the property is verified with the owned reader only).
