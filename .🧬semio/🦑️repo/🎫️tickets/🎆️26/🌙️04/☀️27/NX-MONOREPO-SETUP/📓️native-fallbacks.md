# Conservative Native Source Inputs

The current graph retains conservative domain inputs for 13 concrete Cargo packages. These reasons must be resolved before claiming minimal invalidation throughout the real native dependency graph.

| Project | Cause |
| --- | --- |
| @semio-tech/framework-renderer-wgpu | Custom build script: 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/build.rs |
| @semio-tech/dsl-fixture-sweep-rs | No discovered entrypoints |
| @semio-tech/dsl-derive-rs | Dynamic Rust include requires explicit source inputs: 🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️.rs |
| semio-framework-os-infinite | Custom build script: 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/build.rs |
| @semio-tech/framework-plugin | Generated Rust modules require conservative source inputs: 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs |
| @semio-tech/framework-schema | Custom build script: 🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust/build.rs |
| semio-s-plugin-stdio-test-oracle | Dynamic Rust include requires explicit source inputs: ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🔮️oracle/🦀️.rs |
| @semio-tech/framework-graph | Custom build script: 🧰️framework/🔨️modules/🕸️graph/📦️packages/🦀️rust/build.rs |
| @semio-tech/framework-async-rs | Generated Rust modules require conservative source inputs: 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs |
| @semio-tech/ui-rs | Custom build script: 🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/build.rs |
| @semio-tech/puzzle-plugin | Custom build script: ✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/build.rs |
| @semio-tech/stdio-plugin | Dynamic Rust include requires explicit source inputs: ✏️s/🔌️plugins/🗄️stdio/📇️registry/🦀️.rs |
| @semio-tech/norm-plugin | Generated Rust modules require conservative source inputs: ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs |

## External Build Inputs Requiring Explicit Contracts

Infinite's build.rs reads the framework asset shortcode JSON and both the standard and metabolism SVG catalogs outside its owner domain. Puzzle's build.rs reads the external metabolism SVG catalog. These files are not covered merely by the conservative local-domain fallback; neither inspected project currently declares a corresponding explicit asset input/prerequisite. The previous production-source defaults were also insufficient for SVG bytes. Add source/producer declarations and prove an asset mutation invalidates the consuming native graph before qualifying these real producer closures.

WGPU's ASCII build.rs is a thin include of its schema-owned builder module; inspect that module for external inputs before changing its fallback. DSL derive emits include expressions inside quote-generated Rust token streams, while Stdio has include_bytes of a macro parameter. The current scanner conservatively sees those as dynamic include expressions. Framework plugin, Async (through a mounted plugin fixture) and Norm contain macro-defined modules; the scanner currently rejects every macro body containing mod, including potentially inline-only modules. Narrowing these cases requires compiler/procedural-macro input witnesses, not suppressing the conservative error.

WGPU builder inspection confirms two external input families: the framework standard SVG catalog and the renderer-owner logo at `../../../../🔣️.svg` from its package. Both need explicit source inputs alongside the builder module.

Explicit nativeAssets input groups now track these external SVG catalogs, the shortcode JSON, and the WGPU logo in all three projects. Both ordinary commands and the separate native source/test sets include those asset groups. Generated SVG subtrees excluded by the build-script enumerators are excluded from these source groups. Generator ordering and real asset-mutation qualification remain open; adding source bytes does not by itself provide missing codegen prerequisites.

## Native Generation Prerequisites

The assets renderer confirms the shortcode JSON is an external snapshot, not an output of assets:build. Its existing location is ignored by Git. Therefore adding assets:build would not establish a zero-touch producer for that snapshot; the external authority and acquisition contract still need resolution. SVG catalogs are source inputs and need no compilation prerequisite.

A schema-owned nativeConsumers declaration now identifies the Rust consumers of graph catalog, entity catalog, UI axes, and styling token generation. The plugin resolves each Cargo compilation closure and adds only its required generators directly to the outer Nx task graph. Dev-dependencies participate only for the selected package test/benchmark; downstream packages retain normal and build dependencies. Generator outputs contribute to native result hashes. A language-neutral four-case fixture passed against independent Cargo metadata. The actual workspace graph and clean-output runtime ordering are being validated before removing the three build.rs generator dispatchers.

The real Nx fixture passed cold generator-before-Cargo execution, identical warm reuse, deletion/restoration of both generated Rust and native deliverables with the compiler store removed, and schema mutation (native runs 1 → 2; generator runs 3 → 4). A subsequent regression reproduced development-only dependency over-invalidation (native count 2 instead of 1); native hashes now select exact Cargo compilation owners using Nx named-input projects, and the corrected test passed without rerunning any production producer. Three Cargo build.rs dispatchers (schema, graph, UI) and their automatic build-script settings were removed; the schema-owned prerequisite graph now owns these generators. Real application compilation is still in progress and this does not qualify the full application.
