# GLB Encoder — Report

Scope: `🧰️framework/🔨️modules/🧊️3d/🗿️model/📤️export/🧊️glb/` (G below). Normative source: `design.md`.

## Files

| File | Content |
|---|---|
| `G/🟦️.ts` | `encodeGlb(document, { pose?, animations, variants }, operation) → Uint8Array`, `GlbOptions`, re-exported `PoseSelector` |
| `G/🧱️buffer/🟦️.ts` | `BufferBuilder` (one buffer, 4-byte aligned views, typed accessors, min/max, little-endian through `DataView`), `packGlb` (container v2, JSON padded with spaces, BIN with zeros) |
| `G/🧫️fixtures/🧊️{minimal-triangle,rig-full,rig-bent-pose,rig-wave-quarter}/🔣️.json` | language-agnostic vectors: source `ModelDocument`, options, expected JSON chunk, accessor counts, chunk and total byte lengths, SHA-256 |
| `G/🧪️tests/🧊️vectors/🟦️.ts` | vector law: exact JSON/bytes/SHA-256, alignment, determinism, accessor counts from the source, extension bookkeeping, component types, pose baking, titles/mechanism in extras, progress, cancellation, rejections, weight normalisation (21 tests) |
| `G/🧪️tests/🧪️oracles/🟦️.ts` | third-party oracles (12 tests): Khronos validator, glTF-Transform read-back, three `GLTFLoader` read-back |
| `G/🧪️tests/🧰️support/gltf-validator.d.ts` | minimal typing of the untyped validator package (tests only) |
| `🗿️model/🧪️tests/🎚️config/🟦️.ts` | one added glob (`../../📤️export/🧊️glb/🧪️tests/*/🟦️.ts`) so the core project's `test` target runs these cases |
| `package.json`, `bun.lock` (repo root) | devDependency `gltf-validator` 2.0.0-dev.3.10 added through `repo dependencies lock/sync --all` (test oracle only; no runtime dependency) |
| `T/gen-glb-fixtures.ts`, `T/vitest.glb.config.ts`, `T/validate-one.ts` | ticket-local generator of the fixtures, direct vitest config, one-off validator printer |

## Test command

- Project target: `repo run @semio-tech/framework-3d-model-js:test --output=stream`.
- Direct fallback (used while the machine was memory-starved and `repo`'s capacity gate queued for many minutes): `bun node_modules/vitest/vitest.mjs run --config T/vitest.glb.config.ts` from the repo root, bun being the pinned runtime from `repo tools list`.

Result: 2 files, 33 tests, 33 passed, 0 failed (direct vitest). Project-target result: not obtained. `repo run @semio-tech/framework-3d-model-js:test` was admitted by the capacity gate after 73 s (host swapping) and then failed in the repo graph service ("The resident service met an unexpected failure (reference a05042fa)"), unrelated to this folder; the wiring (project.json, config glob) is in place. The 33/33 result is therefore from the direct vitest invocation only.

## Oracle results

- Khronos `gltf-validator` 2.0.0-dev.3.10 on all four vectors: 0 errors, 0 warnings. Infos only: `INCOMPLETE_EXTENSION_SUPPORT` for `KHR_animation_pointer` (validator knows the extension only partially) and `UNUSED_OBJECT` for the variant that rebinds nothing (`light-solid`).
- glTF-Transform 4.5.1 (`WebIO` + `KHR_materials_unlit` + `KHR_materials_variants`): node names, parents, TRS, positions (min/max), normals, colours, joints, weights, indices, primitive mode, material names/flags/unlit, skin joints and inverse bind matrices, animation sampler input/output/interpolation and node targets, variant names and per-primitive mappings all equal the source.
- three 0.182 `GLTFLoader`: scene node names, TRS, `SkinnedMesh` with the skeleton bones and `boneInverses`, `skinIndex`/`skinWeight`, `LineSegments` for `lines`, geometry arrays, node animation tracks (names, times, values), declared variants in the parsed JSON.

## Behaviour decisions

- Procedural (any `solids`) documents are rejected with a clear error; dangling references, malformed array lengths, non-increasing key times and joint indices beyond the skin are rejected.
- Cancellation throws the kernel's `CancelledError` (`🔺️tessellation`); progress is reported 0 → 1 monotonically; checked per primitive, skin, animation and phase.
- Pose baking reuses the kernel's `poseModel` (joint limits clamp, clip looping) and `sampleMaterials` (material colours baked at the clip time); without `pose` the source TRS is written untouched. Pose findings (`issues`) are not surfaced.
- Skinned mesh nodes become scene roots (glTF ignores their parents for skinning; this removes the validator warning `NODE_SKINNED_MESH_NON_ROOT`); the source parent is kept in `extras.parent`.
- Skin weights are renormalised per vertex; an all-zero vertex becomes joint 0 / weight 1 and joints with weight 0 become 0 (validator-clean).
- Mesh joint/weight attributes are written only for meshes referenced by a skinned node; JOINTS_0 is unsigned byte up to joint 255, else unsigned short; indices are unsigned short below 65535 vertices, else unsigned int.
- `KHR_animation_pointer` channels: `/materials/<i>/pbrMetallicRoughness/baseColorFactor` and `/materials/<i>/emissiveFactor`; not listed in `extensionsRequired`.
- `extensionsUsed` lists only extensions really used, sorted; `extras` carry titles (`en`, `de`), `joint`, `tags`, clip `durationS`/`loop`/`events`, asset id/schema/provenance.

## Deviations from design.md

- None in the contract. Additions: the `extras.parent` hoisting above; `lines` primitives carry no NORMAL (ignored if given); `GlbOptions.pose` is the kernel's `PoseSelector`.
- The expected values in the vectors were recorded from the first reviewed output of this encoder (golden), not from a second implementation; independence comes from the oracles and from the accessor-count/semantic assertions computed from the source document in the test.

## Open issues

- The kernel's `LocalTransform` type is mutable-tuple typed (`V3`/`Q4`) while schema types are `readonly`; the encoder therefore takes a structural `readonly number[]` record. The repo-wide `tsc` run reports unrelated errors in other agents' folders (fbx, tessellation); none in the GLB folder.
- `bun.lock`/`package.json` changed at the repo root (shared file); other agents running `repo dependencies ensure` will install the validator.
- Pose findings of `poseModel` (joint clamps) are dropped silently; a caller wanting them calls `poseModel` itself.
- three's `GLTFLoader` has no `KHR_materials_variants` or `KHR_animation_pointer` support; those two are asserted through glTF-Transform and the validator, plus the JSON for three.

## Maintenance update (A5, `kernel-maintenance.md`)

- A primitive's `solids` (id, first, count) are written to the glTF primitive's `extras.solids`; inconsistent ranges (not contiguous, not whole triangles/segments, not covering) are rejected. Vectors regenerated with `T/gen-glb-fixtures.ts` (the rig carries ranges, `minimal-triangle` none); 36 tests pass, Khronos validator 0 errors, 0 warnings on all four vectors and on a tessellated and edged procedural document; glTF-Transform and three `GLTFLoader` (parsed JSON) read the extras back.
