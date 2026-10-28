# GLB Reader Family Baseline

Captured actual pre-change readIfcGltfGlb on2026-10-21 using authored ticket glb-reader-families-baseline.ts. Immutable output generated/glb-reader-baseline-v3.json:75cases,57accepted18refused. Original v2 artifact/script remain unchanged.

The additional42 cases exercise material/PBR and texture-info fields/defaults; alpha modes; textures/sampler; images URI/bufferView; skins; animation paths/interpolation; perspective/orthographic camera/defaults/tag refusal; sparse indices/values/default offsets; full node transforms/weights; mesh morph targets; asset/scene/extensions rosters; bufferView metadata; all accessor type enums/defaults; null and unknown nested member refusal. Prior33-case semantic/UTF8/number/namespace matrix is retained.

Inputs are built from the actual checked-in Box JSON/BIN through the endpoint fixture, with valid aligned GLB framing and one BIN. Output normalization records exact native-number binary64 words, decimal bigint values and ordered object entries plus full snapshot digest/input hash. --compare reads v3 and writes only generated/glb-reader-comparison-v3.json. Baseline overwrite is refused.

This is actual TypeScript parser-role acceptance evidence. Many synthetic family combinations intentionally lack complete resource semantics; acceptance does not assert glTF semantic validity or Three renderability. No native run, source copy, external runtime library or production edit occurred. Independent meaningful-asset validation remains the existing official Khronos/Three fixture path.
