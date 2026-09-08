# Multi Artifact Execution

## Scope

- 17 Rust artifact packages: procedural 3, GIS 2, FEM 2, Trinity 2, Puzzle 3, Block 3, Space 2.
- Parent composition facades: Block and GIS are owned and completed by the coordinator; this task owns Procedural, FEM, Trinity, Puzzle, and Space.

## Current implementation

- All 17 Cargo packages, workspace declarations, taxonomy crate roots, core source mounts, and component feature boundaries exist.
- Block shared document records have one implementation in Block 2D; Block 3D and 5D use that lower package.
- FEM 3D uses FEM 2D's shared numerical engine.
- Trinity Rewriting uses Jack's shared parser/executor engine.
- Assembly has a package neutral WFC module root.
- Puzzle 2D uses a checked in metabolism icon table and no extracted package build script.
- Ten editor/viewer declaration trees are generic over the parent app enum through each leaf's `ArtifactApps` trait.
- Procedural Flow types bind directly to `semio-framework-artifact-flow-flow`; Flow host behavior remains optional component assembly.
- The 15 leaves still owned here use the explicit `semio_framework_schema` crate path for framework descriptors while preserving each artifact's local `crate::schema` module. The coordinator owns the equivalent final repair for both GIS leaves.
- Procedural shared semantic UI builders now have one implementation at `🌀️procedural/🫀️core/🖼️semantic-ui/🦀️.rs`, mounted by both generation artifacts only for component assembly.
- Space shared catalog, retained-store, and projection helpers now have one implementation at `🪐️space/🫀️core/🦀️.rs`, mounted by the Space artifact component and consumed from Home and the parent Studio engine.
- Procedural, FEM, Trinity, Puzzle, and Space parent packages now compile from their taxonomy composition roots with direct artifact package dependencies. Their obsolete package-local Rust monolith roots are removed; Puzzle's obsolete `build.rs` is also removed because the metabolism table is checked-in source in Puzzle 2D.
- The 13 owned artifact roots that previously exposed plugin-era `crate::{schema,io,op,...}` adapter modules now use their canonical `standards::v1::subsets::any` implementation paths directly; the adapter blocks and their old internal references are gone.
- Ten existing artifact app demo-session modules and their tests are mounted under their owning leaf editors instead of being dropped with the parent monolith roots.
- All five owned parent composition roots directly declare the OS kernel aliases required by dispatch macros, and their Cargo manifests declare `semio-framework-os-kernel` directly.
- Home's component assembly now declares its actual framework action-bus surface, direct framework Space data artifact, and target-specific OS host boundary. The OS host uses `space-guest` for WASI-P2 and `os-host-full` elsewhere; default Home remains free of those component dependencies.

## Validation ledger

- PASS: `cargo metadata --offline --format-version=1` after initial 17 workspace registration.
- PASS: all 17 extracted crate roots parse with rustfmt.
- FAIL then repaired: Block 2D exposed the missing Block shared records and the collision between the derive macro's `dsl` crate alias and the artifact grammar module. The grammar module is now `document_dsl`, while derive macros retain the required `dsl` alias.
- PASS: `cargo check --locked -p semio-s-artifact-block-2d` completed in 49.72 seconds (`🗑️generated/check-block-2d-6.txt`).
- FAIL then repaired: the first combined leaf check found unresolved `schema` paths in extracted sources. The 15 owned manifests retain `semio-framework-schema`, and framework descriptor paths now use `semio_framework_schema` directly. Metadata passes after the repair.
- PASS: `cargo metadata --offline --format-version=1 --no-deps` after all five owned parent facade changes (`🗑️generated/metadata-five-parent-facades.json`).
- FAIL then repaired: Generation 3D schema projection referenced Flow host APIs in its default data boundary; it now projects typed fixtures without a host. Puzzle 2D mutation paths corrupted by a previous broad replacement were restored to the canonical schema mutation modules.
- PASS: the coordinator's fourth framework/Block/GIS integration pass compiled the extracted Space artifact component and the full stdio catalog through Semio before reaching Home.
- FAIL then repaired: Home component assembly lacked its direct framework and OS host dependencies (`🗑️generated/framework-host-integration-check-4.txt`). The manifest now mirrors the target split already used by the Space leaf.
- FAIL then repaired: the first four-leaf boundary check reached Puzzle 2D and found one remaining call through Puzzle 3D's removed `schema` namespace (`🗑️generated/check-boundary-repair-1.txt`). Puzzle 3D now exports only its shared flattening and engine contract at the crate root; Puzzle 2D and 5D use that contract. Jack likewise exports the one mutation constructor and existing graph operations used by Rewriting. No cross-artifact reference to the removed `schema`, `op`, `mutations`, or other adapter namespaces remains in the six multi-artifact plugin trees.
- RUNNING: the warm six-package component-aware boundary repair check uses `CARGO_INCREMENTAL=0` in the shared ticket target (`🗑️generated/check-cross-boundary-repair-2.txt`).

## Remaining work

- Compile drive all 17 default libraries, tests, and component features.
- Compile the five completed owned parent composition facades and their tests.
- Compile the representative STEP oracle host and the stdio composed gates.
- Remove legacy namespace text and generated command logs after validation.
