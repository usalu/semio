# TXT Production Compile Admission

The active FEM/GIS release graph `wKvvlU8` selects `semio_s_artifact_stdio_txt_tests` as a `wasm32-wasip2` build-mode library. It is required by both roots.

- FEM component → FEM hub → FEM 2D I/O → CSV artifact → TXT artifact → TXT production declaration and app dispatch.
- GIS component → GIS hub → GIS map artifact → TXT artifact → TXT production declaration and app dispatch.

The package name and path contain “tests”, but its Cargo library entry is a separate 43-line source with a path-preserving artifact mirror. The TXT artifact facade depends on its declaration, format and native codec registries, capability contribution, standards, editor and viewer, and `TxtApps` runtime dispatch. The actual 178-line schema unit test is a separate source behind `cfg(test)`. The repository WASI source-closure reader found 11 production files and 1,301 active lines. Of paths within `🧪️tests`, that closure includes only the production library entry and its mirror, and it excludes the actual test body.

Admission forecast 2,947 MB at a sample with 5,285 MB host availability and a 2,458 MB margin. The earlier measured peak was 1,738 MiB. The actual guarded build compiled this library at 16:37 PST. Cargo emitted `libsemio_s_artifact_stdio_txt_tests-2e8ccaa1713eae7f.rlib` with `fresh=false`, and the compiler owner recorded 1,808 MiB peak RSS. Removing the dependency would remove the public TXT artifact and its production APIs. No repository source or gate was changed.

Evidence is the run's `unit-graph-release-00.json`, the ticket FEM/GIS release log and compiler-owner peak record. The artifact's production facade is `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🦀️.rs`.
