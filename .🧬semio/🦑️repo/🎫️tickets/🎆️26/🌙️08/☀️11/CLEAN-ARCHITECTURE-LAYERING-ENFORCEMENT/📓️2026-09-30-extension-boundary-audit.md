# Extension Boundary Audit

Read-only source audit on 2026-09-30. Read root AGENTS.md, s AGENTS.md, CAD AGENTS.md and spatial-kernel brepjs AGENTS.md. No implementation files changed and no tests run.

## Concrete Violations

1. `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧱️brepjs/🟦️.ts:65` imports the CAD plugin's external B-Rep interface implementation. Removing CAD prevents resolving this general spatial module. Its file header describes the kernel as a differential test oracle; ownership should therefore be in a differential-test owner rather than general production modules.
2. Spatial `📐️geometry/🟦️.ts:3511` and `🗺️spatial/🟦️.ts:324` dynamically import CAD artifact runtime under `import.meta.vitest`. These remain source/build dependency edges, despite runtime guards. Extracted suites also receive these plugin values through exported `GeometryTestDependencies` and `SpatialTestDependencies`. Move integration suites into CAD, and retain domain-independent suites in spatial modules.
3. Spatial semio session Rust package `📜️script.ts:6` imports the stdio semio artifact's retained-extrusion oracle. Its test/task execution therefore requires a deletable plugin artifact.
4. `✏️s/🔌️plugins/📐️cad/⚙️engine/🫀️core/🟦️.ts:4` reexports the CAD artifact's registry. CAD engine should own general registration contracts; the artifact should supply registrations. The existing registry already defines `ModelDefinitionAssetModules` at line 20 and `registerModelDefinitionAssets` at line 87, providing a natural interface boundary.
5. `✏️s/🔌️plugins/📐️cad/📦️packages/🟦️typescript/🟦️.ts` eagerly exports CAD artifact schema, snapshots, diffs, mutations, IO and runtime. This prevents importing the CAD engine after deleting its artifact. Similar artifact aggregation exists in the energy, architect, shooting, mathematical and GIS plugin barrels.
6. CAD artifact runtime `✏️editor/⚙️engine/🏃️runtime/🟦️.ts:8-11` imports four concrete CAD extension packages, and lines 34-37 hardcode their registration callbacks. CAD package.json lists the same packages as runtime dependencies. Keep extension registration generic and put this selected assembly in a product/composition owner.
7. `✏️s/🔌️plugins/📐️cad/📦️packages/🦀️rust/Cargo.toml:58-63` requires the CAD artifact and stdio DWG, OBJ, semio, STEP and STL artifacts. The crate barrel reexports CAD editor/viewer; plugin root `🦀️.rs` uses a closed `CadApps` enum containing artifact apps and semio members. Deleting any of those artifact crates breaks the plugin's compile graph.
8. Plugin production bridge Cargo.toml files explicitly enumerate artifacts: block has three; procedural has two; WFC has five. These are composition binaries and should be classified and built as selected assemblies, separate from general plugin APIs.

## Recommended Independent Scope

First enforce boundary ownership for the spatial module test cycle: move plugin integration test composition out of general source files, remove plugin-dependent public test dependency types, and relocate differential B-Rep implementation and oracle suites together under a test owner. No compatibility facade should remain. Update all import consumers manually. A separate CAD engine scope can move registry ownership upward, expose a narrow CAD engine package API, and place artifact-only exports in artifact package entries.

For runtime registration, define schema-first descriptors owned by the general layer. Specific owners contribute artifact and extension factories against those contracts. A composition owner selects installed contributions. Zero contributions must be a valid host/plugin state. Removing an installed child changes discovered contributions without requiring edits to its parent API or parent build graph.

## Validation Targets

- Existing extracted geometry suite: `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/📐️geometry/🧪️tests/🧪️semio-tech-cad-js-core-vec/🟦️.ts`.
- Existing spatial commit suite: `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🗺️spatial/🧪️tests/🧪️semio-tech-cad-js-core-model-commit-mesh/🟦️.ts`.
- CAD package test target uses `📜️script.ts test` and Vitest config at `../../🧪️tests/🎚️config/🟦️.ts`.
- CAD Rust plugin has existing `🧪️tests/🔬️surface/🦀️.rs` and `🧪️tests/🔬️assembly/🦀️.rs`.
- Add language-neutral fixtures for empty registration, one independent registration, and removed contribution. Use an existing third-party validator as a differential oracle. Validate absent-owner source resolution through an isolated in-memory or ticket-local fixture, without deleting real shared-workspace files.

## Limitations

This scan identifies explicit source and manifest edges; it does not establish every package's complete transitive dependency graph. Development composition and test assemblies legitimately select specifics when their ownership and execution scope are explicit. No runtime behavior or passing tests are claimed.
