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
- Six generic component declarations had accidentally placed `ArtifactApps` inside `definition()`. Puzzle 2D/3D/5D, FEM 2D/3D, and Jack now declare that trait at crate scope, so their standard/subset factories resolve the intended bound.
- Puzzle 2D/3D/5D and Generation 3D now mount their artifact example sources and source tests at the crate root. Their existing component editors can use `crate::examples` without depending on a removed parent monolith.
- Procedural layouts now use their local Flow-window constants, and both Flow windows address the declared `semio_framework_ui_styling` dependency by its canonical crate name.
- Jack owns the shared Trinity LOD scale and JSON projection under its component editor. Rewriting consumes that lower package API; Jack has no reverse reference to Rewriting.
- Jack's default data package no longer selects the framework composition facade, geometry renderer, or Infinite canvas. Those three dependencies are enabled only by `component-app-assembly`; its default wire runtime uses the lower plugin fault contract directly.
- Procedural, FEM, Trinity, and Puzzle parent manifests now match their thin composition roots: direct leaf packages plus the plugin, dispatch, and kernel contracts only. Static source/test use and Bun TOML parsing confirm dependency sets of 6, 5, 5, and 6 respectively; the removed former-monolith lists no longer make the parent crates direct consumers of every format and framework subsystem.
- Space's three Home-specific exact Cargo law groups now target `semio-s-artifact-space-home --features component-app-assembly`. The seven cross-surface interactive job catalog laws remain correctly owned by the Space composition crate; their mounted test source was preserved and now addresses Home and Space Index through the two direct artifact crates instead of removed `crate::{editor,viewer}` namespaces. The same direct boundary repair covers the parent surface tests and Studio's set-active-example tests.
- The measured Puzzle 3D wasm-dev optimization exception now applies to `semio-s-artifact-puzzle-3d`, where the render turn lives, instead of forcing the entire Puzzle composition package to optimization level 2.
- Seven stdio artifact tests that only need the artifact DSL now import `semio_framework_os_kernel::ArtifactDsl` directly instead of reaching through the stdio composition package. The STEP CC6 bridge also reports `semio-s-artifact-stdio-step` as its producer, matching its direct Cargo dependency and source import.
- Space is a registered workspace member, so its composition manifest now inherits the shared framework, schema, pack, UI, plugin, dispatch, OS host, and test dependencies from the root workspace. The two package-local aliases without root workspace keys remain explicit path dependencies. The stale direct Collection and five stdio format dependencies were removed after a complete composition-root/engine/test namespace scan found no use; Space receives those capabilities through its actual artifact/host contracts instead of compiling unrelated leaves itself.
- The stdio composition package now enters through the taxonomy `🗄️stdio/🦀️.rs` root. Its package-local Rust wrapper is removed, while the unchanged full plugin assembly lives at `🗄️stdio/🔌️plugin/🦀️.rs`; the root preserves the existing `plugin`, registry, manifest, aliases, root export, and feature boundaries.

## Owned file ledger

- Exact current Rust/Cargo/source-router paths are recorded in `📓️artifact-execution-file-ledger.md` (636 paths), including the 36 stdio packages, initial GIS2 roots, 15 owned multi-artifact roots, parent/root removals, legacy test import fixes, and workspace/profile wiring.

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
- REPAIRED FROM INDEPENDENT DIAGNOSTIC: the previous all-target stream exposed component-only failures hidden by default checks: six nested `ArtifactApps` declarations, missing Puzzle/Generation 3D example mounts, stale Procedural host constants, and stale styling aliases. Current source contains none of those failing paths.
- PASS (STATIC): all 15 owned leaf manifests satisfy the canonical package name, taxonomy `../../🦀️.rs` library path, empty default feature, and optional component-dependency rules (`checked=15 failures=0`, Bun TOML oracle).
- PASS (STATIC): the 17-package artifact dependency graph is acyclic (`nodes=17 cycles=0`). Its only intra-batch edges are terrain→map, FEM3D→FEM2D, Rewriting→Jack, Puzzle2D/5D→Puzzle3D, Block3D/5D→Block2D, and Home→Space Index.
- PASS (STATIC): the complete stdio tree has zero `semio_s_plugin_stdio::artifacts` or `crate::artifacts::` Rust references, including documentation and generator sources; direct artifact packages are the only format namespaces.
- PASS (STATIC): the six multi-artifact plugin trees have zero imports through their former parent `semio_s_plugin_*::artifacts` namespaces, and all 15 owned leaves have zero external `::schema` references that would collide with their domain-local `crate::schema` modules.
- INTERRUPTED WITHOUT RESULT: the warm six-package component-aware boundary repair check was queued on the deleted lock inode after the shared `🗑️generated` directory was replaced. It had no rustc child and was cancelled with exit 130 so it cannot overlap the current lock epoch. No pass or failure is inferred from its deleted output.
- OUTPUT PRESERVATION: validation outcomes are recorded here as soon as they complete because the shared generated directory has already been replaced once during active commands.
- REPAIRED SHARED PREREQUISITE: the first four-Space-law run stopped before its oracles on a malformed import in the framework Describe test. The Nx owner repaired that separate parser input; all four Space oracles subsequently reached their own bodies.
- PASS after the shared parser repair: Nx `@semio-tech/space-plugin:home-directory-projection-persistence-check` completed `checks=11 clean`, and `interactive-job-catalog-check` completed `checks=23 clean`.
- PASS: `home-directory-event-page-owner-check` reports `checks=27 clean`. The caller now uses draft-07 as declared by the framework Directory schema, registers extension keywords before compilation, validates the local Home receipt definition, and audits the extracted Home artifact root.
- PASS: Nx `@semio-tech/space-plugin:home-directory-identity-rows-check` reports `checks=54 clean` (exit 0); the direct Bun route also passed. Its oracle reads the extracted Home and Space Index crate roots plus their physical fixture, unit, testkit, and component sources; it contains no deleted parent-monolith source path.

## Remaining gate matrix

| Scope | Gate | Status | Evidence or owner |
| --- | --- | --- | --- |
| stdio 36 | all default libraries | Earlier pass for 36/36; current-source rerun pending | Previous selective sweep completed in 2m08; rerun follows current lock epoch |
| stdio Semio | default library and dependency closure | Earlier pass; current tests pending | Default tree excluded the other 35 format crates; current-source verification queued |
| stdio STEP | third-party CC6 oracle host | Pending | Direct STEP bridge dependency/import is statically correct; representative host must run |
| stdio catalog | `full-artifact-catalog` | Current compile pass; runtime laws pending | Coordinator's GIS integration compiled full stdio and Semio after extraction |
| stdio catalog | `home-io` | Source contracts pass; native gate pending | Nx stdio gate reported six surfaces, direct 4/shared 4/full 36; Space source laws pass at checks 11, 23, 27, and 54 |
| owned artifacts | 15 default libraries and tests | Partial pass; sweep pending | Block 2D passed; all 15 manifests and the 17-node DAG pass static contracts |
| owned artifacts | 15 component assemblies | Pending | Fresh component sweep follows the current active host Cargo process |
| owned parents | Procedural, FEM, Trinity, Puzzle, Space libraries and tests | Static pass; Cargo gates pending | Thin facades and exact direct dependency sets verified; runtime/test compilation queued |
| delegated leaves | GIS Map and Terrain | Coordinator-owned | Excluded from the owned 15 sweep |
| delegated parents | Block and GIS | Coordinator-owned | Coordinator owns their composition/runtime gates |
| framework host | combined host integration | Coordinator-owned | This task repairs leaf diagnostics only and does not duplicate the host gate |

## Remaining work

- Compile drive all 17 default libraries, tests, and component features.
- Compile the five completed owned parent composition facades and their tests.
- Compile the representative STEP oracle host and the stdio composed gates.
- Remove legacy namespace text and generated command logs after validation.
