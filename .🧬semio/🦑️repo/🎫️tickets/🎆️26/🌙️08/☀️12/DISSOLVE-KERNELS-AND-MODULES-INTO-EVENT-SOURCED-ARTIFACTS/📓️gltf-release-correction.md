# glTF Release Correction

## Applied Leaf Corrections

- Declared `s.stdio.gltf.inference.geometric-analysis` as the sixth direct terminal consumer of `s.stdio.gltf.module.mesh-topology`.
- Removed two verified-empty stale inference directories: `📦bounds` and the retired nested `🔨modules` collection.
- Changed metric member targets to actual serialized aggregate fields under `geometricAnalysis.overall.*`; the aggregate member remains the `geometricAnalysis` target.

## Local Verification

- All edited glTF manifests parse as JSON.
- `git diff --check -- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf` produced no whitespace errors.

The central coordinator must now rerun the repaired consumer graph and scoped taxonomy report. The post-manifest Cargo/Nx execution remains a release prerequisite.
