# Puzzle Hub component refresh

The first refresh reported an unresolved import of `semio_framework_3d_brep_axial_face_preview` in `⏱️tessellation-job/🦀️.rs`. The live tessellation crate already declares the direct workspace dependency. The Framework workspace resolves it to the axial-face preview crate, which exports both imported types, and the Hub lock records the edge. This check required no manifest change.

The same build's DWG snapshot object compiler exited with status 137. The host kernel reported a global out-of-memory kill of Rust compiler PID 1750084 at 07:05:53, with 4.54 GB anonymous resident memory. The retry serializes Cargo to one job in the Puzzle Hub component directory.

```sh
CARGO_BUILD_JOBS=1 repo cargo --store=shared --configuration=prototype --parallelism=1 --lang=en -- build --lib --profile prototype --target wasm32-unknown-unknown
```

The first retry had already compiled snapshot-clone when its native suite exposed two cancellation defects. That Cargo child was stopped before it reached DWG or tessellation. The repository launcher's external jobserver controls compiler parallelism, so the final retry sets that launcher's explicit one-core limit and uses the corrected source. The earlier generated package, which passed four browser tests, is available while the component rebuilds.

Before binding, freshness is checked for `semio-os-store-snapshot-clone`, `semio-plugin-command`, `semio-s-artifact-puzzle-5d-dispatch`, and `semio-s-artifact-puzzle-5d-puzzle5d`. These packages carry the latest WindowConfig Work, command handoff, and small-example buffer fixes.

The first repository-serialized build verified one admitted core and compiled the corrected snapshot-clone source. It reached a newer emission declaration that referred to `ArtifactMutationPages`, while its store publication metadata dated 07:54:41. The live store reexport and paged input owner were saved at 08:06:13 and 08:09:30. The live declaration and reexport agree. A fresh serialized build is running with these sources.
