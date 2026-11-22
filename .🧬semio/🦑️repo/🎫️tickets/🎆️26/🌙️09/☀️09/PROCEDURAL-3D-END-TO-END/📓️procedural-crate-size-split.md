# Procedural generation3d source-boundary blocker

## What the guard counts
`rust-source-boundary` (`maxSourceBytes = 262144`, `maxSourceFiles`, `maxActiveLines`) in `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🎟️admission/🟦️.ts`. Per compilation unit (lib and every `[[test]]` target) it reads the whole closure: the entry, every `#[path]`/`mod` file and every `include_str!`/`include_bytes!` file, cfg(test) included. Data files embedded at compile time therefore count as source.

## Root causes (not the Rust code)
- `semio-s-artifact-procedural-3d-io-text` (779563 bytes): one test, `catalogue_covers_every_exposed_kernel_method_and_mesh_operation_once`, `include_str!`-ed the 739 KB packaged brep component manifest (`🌊️flow/🧩️extensions/📐️brep/🧩️component/🔣️.json`). The crate's real sources are ~100 KB.
- `semio-s-artifact-procedural-generation3d` (275029 bytes): the `editor-routes` test target path-includes the editor unit tests, which `include_str!` the 97 KB set-widget-input, 54 KB surface and 27 KB edit-mesh-selection fixtures (about 330 KB closure).

## Change
Splitting into more crates would not reduce the data, only move it, and the fixtures are test data rather than source. The tree already owns the runtime fixture reader `semio-framework-test-fixtures`, so the large fixtures are read at test run time instead of embedded at compile time:
- io-text `📸️snapshot/🗂️catalogue/🧪️tests/🔬️unit/🦀️.rs`: `include_str!` of the manifest to `fixture_text!` (manifest-relative, 11 levels up).
- editor unit tests `✏️editor/🧪️tests/🔬️unit/🦀️.rs` (7 sites) and `📥️import-preview.rs` (1 site): `include_str!` of the three big fixtures to `repo_text!` (repo-root-relative, robust to whichever crate path-includes the tests).
No crates, registries or limits changed; nothing minified.

## Verification (repo cargo)
- Admission of the whole generation3d workspace (`check --workspace --tests`) and of `-p semio-s-artifact-procedural-generation3d --test editor-routes` now passes; the compiler starts. No boundary refusal anywhere.
- The runs then stop on other lanes' in-progress work: `semio-plugin-interpreter` (E0599 `charge_retirement_retained_item` missing on `StepContext`) and `semio-os-infinite-world-interaction` (E0425 `world3d_ground_plane_pick`, `plan_world3d_engagement_pointer`, `publish_world3d_engagement_pointer`; E0063/E0061/E0599 on `World3dRelocateSession`). Re-run when those land.
- Test results of the edited tests are therefore not yet observed.

Re-run: `repo cargo test --manifest-path ./✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/Cargo.toml -p semio-s-artifact-procedural-3d-io-text -p semio-s-artifact-procedural-generation3d --no-fail-fast`
