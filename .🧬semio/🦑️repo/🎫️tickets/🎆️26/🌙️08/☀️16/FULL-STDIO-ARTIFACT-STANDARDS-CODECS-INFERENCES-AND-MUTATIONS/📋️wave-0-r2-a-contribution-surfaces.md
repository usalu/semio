# Wave 0 R2-A Contribution Surfaces

## Scope

Framework-owned contribution assembly surfaces replacing the remaining non-stdio setup registrars:

- OS-media mesh/DWG bridge and 2D SVG-export declarations.
- Immutable `flow.extension` descriptors with typed manifests and executable identities.
- Schema-keyed foreign document codecs in the single artifact assembly plan.
- Explicit public re-export of `FormatRegistryError`.

## Design

`PluginBuilder` collects all contribution facets as data. It validates plugin ownership, declared artifact-kind descriptor equality, document schema equality, stable contribution IDs, typed flow-manifest structure, app ownership for foreign codecs, and exact executable identity before any IO/store mutation.

`HostMediaHandlerDeclaration` is the sole function-pointer surface. Its converter pointers have fixed request/result contracts and are retained in `PluginRuntimeRegistry`; plugin assembly never invokes them. `Plugin::import_mesh_dwg` and `Plugin::export_two_d_svg` are the typed runtime invocation surfaces.

`FlowExtensionDeclaration` intentionally contains no installer callback. It freezes a `FlowExtensionManifest` and `FlowExtensionExecutableIdentity` in the plugin runtime registry, with stable ID and extension-target conflict rejection.

`foreign_document_codec::<App>(schema)` retains the app codec thunk as a declaration and appends its codec to the existing aggregate `ArtifactAssemblyRegistryPlan`; it must name an app declared by the same builder. The existing aggregate preflight/commit remains the only IO/store write boundary.

## Focused Coverage

Builder tests cover:

- identical media and flow declarations are idempotent;
- conflicting media target/executable ownership is rejected before converter invocation;
- media functions execute only at typed runtime request time;
- conflicting flow extension targets are rejected during local registry assembly.

## Validation Status

- `cargo check --manifest-path 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/Cargo.toml` passed. Existing workspace warnings remain outside this lane.
- `cargo test --manifest-path 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/Cargo.toml plugin_builder_dependency_tests --lib -- --test-threads=1` passed: 5 passed, 0 failed.
- `rustfmt --edition 2021 --check` was run for the two plugin component sources and framework glue. It reports broad pre-existing formatting diffs in those monolithic/path-mounted files; this lane did not format or alter unrelated source.
