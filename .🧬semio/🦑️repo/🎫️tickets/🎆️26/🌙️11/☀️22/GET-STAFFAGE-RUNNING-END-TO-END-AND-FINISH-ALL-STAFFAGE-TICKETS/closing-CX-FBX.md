FINAL (2026-11-27 01:45: assimp FBX failures cured in the oracle by a fresh assimp instance per file; the writer's FBX files are valid)

# CX-FBX: assimp `Aborted()` / `undefined` / `import_error`

## Cause
- The FBX files are valid. I exported `city-car` (24 FBX, 0.3 to 3.6 MB) and read each with assimp in a fresh instance (24 of 24 ok), and all of them again through one shared instance (all ok). The framework's second-reader test and `🧊️model-exports` (18 tests) are green.
- The oracle kept ONE assimpjs WebAssembly instance for the whole process (`assimp ??= ...`). A shard runs 24 assets (thousands of conversions of multi-MB files) in one process, and the heap of a WebAssembly instance only grows and is never returned. Once that instance aborted (`Aborted(). Build with -sASSERTIONS`), every later conversion in the process failed (`undefined`, `import_error`) until the shard ended. This is why only shard 09 shows the issues, and why they appear in runs of consecutive assets.
- Evidence: the single-asset `city-car` run with the old cached-instance code gives 0 assimp findings, so the asset alone is fine. I did not reproduce the abort in a standalone soak (the soak script ended without output), so "cumulative heap of the shared instance" is the inference from these facts, not a measured fact.

## Cure
- `S/🧪️tests/🧊️model-catalog/🔍️oracles/🟦️.ts`: `assimpModule` now returns a fresh instance for every file (the check stays, nothing skipped; native objects are still deleted).
- Regression test: `🧰️framework/🔨️modules/🧊️3d/🗿️model/📤️export/🎬️fbx/🧪️tests/🔁️repeated-reads/🟦️.ts` (60 reads of the animated rig fixture, each in a fresh instance, same result each time); scenario "A long run of FBX files never exhausts assimp" in `S/🧪️tests/🧊️model-catalog/🥒️.feature`.

## Proof
- Single-run `city-car,reclaimed-lattice-girder,scroll-compressor,tower-crane` of `🧊️model-catalog` after the fix: 0 findings with `fbx` or `assimp`. The 4 tests still fail only on other kinds that are not mine (GLB budget over 8 MB, and glTF-Transform "Missing required extension KHR_mesh_quantization", which comes from a concurrent GLB change by another lane).
- `repo test-file` on `🧊️model-exports`: 18 pass. Framework `repeated-reads` and `second-reader`: pass.
- No catalog file rewritten.
