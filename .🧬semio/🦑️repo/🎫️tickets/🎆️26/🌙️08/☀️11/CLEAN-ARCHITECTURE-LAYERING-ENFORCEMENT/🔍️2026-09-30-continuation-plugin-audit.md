# Continuation Plugin Audit

Read-only production architecture audit, 2026-09-30. No source changes, tests, runtime execution, Git mutations or generated logs. Existing strict inventory reports 291 edges; the table below inventories current Rust source references independently rather than claiming the gate passes.

## Finding and Clean Composition Direction

Most plugin-to-artifact edges are component assembly, not reusable algorithms. Moving aliases alone cannot satisfy removability: closed `PluginApp` fleets, typed factory registration, activation identities, codec receipts, and compile-time protocol includes still name the removed artifact. Cargo optional dependencies also require a present manifest during graph resolution. Remove these declarations completely from the general plugin crate.

Keep general plugin identity, capabilities, domain-neutral services and extension contract in the plugin package. Artifact-owned packages provide declarations, concrete app factories, mutation rosters, codecs and schema assets. Compose those packages in an explicitly outward deployment/component owner, discovered from present package metadata by repository tooling. That owner may depend on plugin and artifact packages; plugin may not depend on the owner. Removing an artifact must remove only its discovered contribution, leaving the general plugin buildable. Generate/select deployment inputs through the existing 📜️script.ts mechanism and register Nx/launch routes. Do not relabel the old plugin manifest as a module to evade physical ownership rules.

Retain monomorphized closed app dispatch at the deployment boundary: `PluginApp` has async methods and is not directly object safe. A naive `Box<dyn PluginApp>` registry is therefore not a drop-in implementation. Package-owned runtime components with existing host manifest/router registration are also feasible; static discovery is simpler for existing native test and WASM fleet routes and preserves performance.

## Existing Contracts to Reuse

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🦀️.rs`: `PluginBuilder`, `declare_artifact` (line 277), `contributes_topic` (365); no duplicate registrar API needed.
- Same owner `🦀️.rs`: `PluginApp` (14269), `Plugin<PA>` (35264), `ArtifactDeclaration<PA>` (38735), `AppFactory<PA>` (38744), `SurfaceDeclaration`, codec tables, native artifact definition and mutation roster machinery. Factories/declaration types retain the concrete deployment fleet parameter.
- Same owner `🖥️host/🦀️.rs`: `PluginGraph::register`, manifest registration and app routing. This is the existing runtime authority for separately shipped packages.
- `✏️s/🔌️plugins/🗄️stdio/📇️registry/🦀️.rs`: existing `ArtifactContribution`, `ArtifactAssembly`, `NativeCodecFactory` and receipt validation from `semio-s-artifact-stdio-contract`; `selected_contributions` hardcodes 36/8 rows and `expected_artifact_count` prevents arbitrary removal. Move contribution selection to outward deployment and validate uniqueness/claims against the selected roster, without fixed catalog cardinality.

## Concrete Cases

- Block, puzzle, FEM, WFC, procedural, trinity and norm roots name concrete artifact app types in closed enums and declaration/activation calls. These are assembly. Norm's 165 source references are declaration/fleet volume, not evidence that norm algorithms require its artifacts.
- Most single-artifact package roots mount `artifacts`, `editor`, `viewer` aliases, then include their root plugin assembler. Examples: raster/remodel/draw/energy `📦️packages/🦀️rust/🦀️.rs`. Delete these alias surfaces from the general plugin API and update concrete consumers to artifact packages in the outward owner; no compatibility reexports.
- GIS `🦀️.rs` explicitly exports map editor types for demonstrator; replace that consumer path atomically. GIS `📇️native-codecs/🦀️.rs` owns typed Map/Terrain receipts and directly includes both binary protocol files; move each factory plus protocol identity into its artifact-owned package and let deployment combine receipts. Renaming the type does not fix includes.
- Every `🏭️bridge/🦀️.rs` enumerates concrete mutation aggregates and coordinates for production inventory. These are test infrastructure assemblies, independently compiled with artifact dependencies. They must move to outward inventory ownership or discover artifact-owned inventory exports. Do not remove descriptor validation.
- Stdio `🔌️plugin/🦀️.rs` plus nine `🧩️extensions/*/🦀️.rs` files are concrete editor/viewer fleets. Root `📇️registry` additionally owns format/catalog receipt composition. This is the largest coherent assembly refactor, affecting the 36 root plus 30 family edges in the prior gate inventory.
- Energy `🔨️modules/⚡️simulation/⚙️engine/📍️site/🦀️.rs` is a true service-to-format dependency: `WeatherData::parse` decodes EPW and `from_snapshot` exposes `EpwSnapshot`; `TryFrom<&EpwRecord>` also names EPW data. Keep weather records/data in simulation; move EPW reading/mapping to the EPW consumer contribution and pass weather records into the engine. No duplicate EPW codec or compatibility parse facade.
- Flow `🧩️extensions/📐️brep/🦀️.rs` calls actual artifact-owned `BrepKernel`, jobs, admission and quality operations. It is a specific consumer extension, not general flow behavior. Keep it outward, with its artifact dependency explicit; do not lift BREP algorithms wholesale into flow. CAD building, sourcing module, and process material extension files likewise need ownership classification separately from general plugin assembly.

## Execution Partitions

1. **General plugin/deployment boundary:** establish schema-first present-contribution deployment selection using the existing generic declaration/factory contracts; move fleets, aliases, activation rows and entry exports for the single/multi-artifact plugins atomically. Include Cargo workspace declarations, lock state, tests and playground metadata. Begin with block plus draw as two representative fixtures, then apply the coherent fleet change. General package tests must compile with their artifact directories physically absent.
2. **Stdio/native/inventory boundary:** move stdio catalog/family fleets and bridge selections to outward deployments; relocate GIS receipts/protocol inputs and energy EPW import mapping into specific contributions. Reuse framework definitions/receipts and update host/native/inventory consumers. Can run in parallel only after agreeing on deployment ownership/discovery contract; do not concurrently edit the central builder/schema contract or shared workspace manifest.

Acceptance requires language-agnostic removal vectors, independent Cargo graph comparison, no surviving artifact aliases/imports/includes in general owners, native and WASM deployments with full and reduced rosters, concrete mutation inventory parity, and runtime open/edit/view/close evidence. None of these checks was executed in this audit.

## Complete Non-Artifact Rust Reference Inventory

Table includes authored Rust files outside `🗿️artifacts` and `🧪️tests` containing the exact `semio_s_artifact_` prefix. Alias-mediated references are covered by root mounts above. Counts are occurrences, not edges. Bridges are separate inventory binaries, not runtime algorithms.

| Plugin | File (relative to plugin) | Occurrences |
| --- | --- | ---: |
| `🔱️trinity` | `🦀️.rs` | 14 |
| `🔱️trinity` | `🏭️bridge/🦀️.rs` | 12 |
| `📸️remodel` | `📦️packages/🦀️rust/🦀️.rs` | 3 |
| `📸️remodel` | `🏭️bridge/🦀️.rs` | 2 |
| `🖨️raster` | `📦️packages/🦀️rust/🦀️.rs` | 3 |
| `🖨️raster` | `🏭️bridge/🦀️.rs` | 2 |
| `🌊️flow` | `🦀️.rs` | 2 |
| `🌊️flow` | `📦️packages/🦀️rust/🦀️.rs` | 3 |
| `🌊️flow` | `🧩️extensions/📐️brep/🦀️.rs` | 6 |
| `🌊️flow` | `🏭️bridge/🦀️.rs` | 2 |
| `🏭️process` | `🦀️.rs` | 2 |
| `🏭️process` | `📦️packages/🦀️rust/🦀️.rs` | 3 |
| `🏭️process` | `🧩️extensions/🔩️metal/🦀️.rs` | 1 |
| `🏭️process` | `🧩️extensions/🪵️wood/🦀️.rs` | 1 |
| `🏭️process` | `🧩️extensions/🤖️robotic/🦀️.rs` | 1 |
| `🏭️process` | `🧩️extensions/🧱️concrete/🦀️.rs` | 1 |
| `🏭️process` | `🏭️bridge/🦀️.rs` | 2 |
| `📕️norm` | `🦀️.rs` | 165 |
| `📕️norm` | `🏭️bridge/🦀️.rs` | 30 |
| `📐️cad` | `🦀️.rs` | 2 |
| `📐️cad` | `📦️packages/🦀️rust/🦀️.rs` | 3 |
| `📐️cad` | `🧩️extensions/🏢️aec-building/🦀️.rs` | 1 |
| `📐️cad` | `🧩️extensions/🏢️aec-building/🧬️schema/🧬️mutations/🏢️create-building-storey/🦀️.rs` | 2 |
| `📐️cad` | `🏭️bridge/🦀️.rs` | 2 |
| `🎪️demonstrator` | `🪪️manifest/🎪️demonstrator/🦀️.rs` | 3 |
| `🎪️demonstrator` | `📦️packages/🦀️rust/🦀️.rs` | 3 |
| `🎪️demonstrator` | `🏭️bridge/🦀️.rs` | 2 |
| `🧱️block` | `🦀️.rs` | 18 |
| `🧱️block` | `🏭️bridge/🦀️.rs` | 8 |
| `🕸️dag` | `🦀️.rs` | 2 |
| `🕸️dag` | `📦️packages/🦀️rust/🦀️.rs` | 3 |
| `🕸️dag` | `🏭️bridge/🦀️.rs` | 6 |
| `🗄️stdio` | `🔌️plugin/🦀️.rs` | 62 |
| `🗄️stdio` | `📇️registry/🦀️.rs` | 46 |
| `🗄️stdio` | `🧩️extensions/📘️pdf/🦀️.rs` | 63 |
| `🗄️stdio` | `🧩️extensions/🛠️cad/🦀️.rs` | 69 |
| `🗄️stdio` | `🧩️extensions/🏠️bim/🦀️.rs` | 42 |
| `🗄️stdio` | `🧩️extensions/💼️office/🦀️.rs` | 65 |
| `🗄️stdio` | `🧩️extensions/🧿️semio/🦀️.rs` | 133 |
| `🗄️stdio` | `🧩️extensions/🎵️media/🦀️.rs` | 37 |
| `🗄️stdio` | `🧩️extensions/🔺️mesh/🦀️.rs` | 45 |
| `🗄️stdio` | `🧩️extensions/🖼️image/🦀️.rs` | 85 |
| `🗄️stdio` | `🧩️extensions/🔢️binary/🦀️.rs` | 42 |
| `🗄️stdio` | `🏭️bridge/🦀️.rs` | 174 |
| `💡️reasoning` | `🦀️.rs` | 2 |
| `💡️reasoning` | `📦️packages/🦀️rust/🦀️.rs` | 3 |
| `💡️reasoning` | `🏭️bridge/🦀️.rs` | 6 |
| `🎬️sequence` | `🦀️.rs` | 2 |
| `🎬️sequence` | `📦️packages/🦀️rust/🦀️.rs` | 3 |
| `🎬️sequence` | `🏭️bridge/🦀️.rs` | 2 |
| `✒️writer` | `🦀️.rs` | 2 |
| `✒️writer` | `📦️packages/🦀️rust/🦀️.rs` | 3 |
| `✒️writer` | `🏭️bridge/🦀️.rs` | 6 |
| `🎞️animate` | `🦀️.rs` | 2 |
| `🎞️animate` | `📦️packages/🦀️rust/🦀️.rs` | 3 |
| `🎞️animate` | `🏭️bridge/🦀️.rs` | 6 |
| `🪐️space` | `🦀️.rs` | 21 |
| `🪐️space` | `🏭️bridge/🦀️.rs` | 4 |
| `🌀️procedural` | `🦀️.rs` | 24 |
| `🌀️procedural` | `🏭️bridge/🦀️.rs` | 16 |
| `🌿️vcs` | `📦️packages/🦀️rust/🦀️.rs` | 3 |
| `🌿️vcs` | `🏭️bridge/🦀️.rs` | 2 |
| `🌍️gis` | `🦀️.rs` | 28 |
| `🌍️gis` | `📇️native-codecs/🦀️.rs` | 6 |
| `🌍️gis` | `🏭️bridge/🦀️.rs` | 10 |
| `🀄️wfc` | `🦀️.rs` | 40 |
| `🀄️wfc` | `🏭️bridge/🦀️.rs` | 20 |
| `📜️imperative` | `🦀️.rs` | 2 |
| `📜️imperative` | `📦️packages/🦀️rust/🦀️.rs` | 3 |
| `📜️imperative` | `🏭️bridge/🦀️.rs` | 4 |
| `🪵️sourcing` | `🦀️.rs` | 2 |
| `🪵️sourcing` | `📦️packages/🦀️rust/🦀️.rs` | 3 |
| `🪵️sourcing` | `🧩️extensions/🪵️beams/🦀️.rs` | 1 |
| `🪵️sourcing` | `🧩️extensions/🧱️slabs/🦀️.rs` | 1 |
| `🪵️sourcing` | `🧩️extensions/🪟️windows/🦀️.rs` | 1 |
| `🪵️sourcing` | `🏭️bridge/🦀️.rs` | 2 |
| `🗒️note` | `📦️packages/🦀️rust/🦀️.rs` | 3 |
| `🗒️note` | `🏭️bridge/🦀️.rs` | 4 |
| `📋️forms` | `🦀️.rs` | 2 |
| `📋️forms` | `📦️packages/🦀️rust/🦀️.rs` | 3 |
| `📋️forms` | `🏭️bridge/🦀️.rs` | 4 |
| `🏛️architect` | `🦀️.rs` | 2 |
| `🏛️architect` | `📦️packages/🦀️rust/🦀️.rs` | 3 |
| `🏛️architect` | `🏭️bridge/🦀️.rs` | 6 |
| `🎥️shooting` | `📦️packages/🦀️rust/🦀️.rs` | 3 |
| `🎥️shooting` | `🏭️bridge/🦀️.rs` | 6 |
| `➗️mathematical` | `🦀️.rs` | 2 |
| `➗️mathematical` | `📦️packages/🦀️rust/🦀️.rs` | 3 |
| `➗️mathematical` | `🏭️bridge/🦀️.rs` | 4 |
| `📏️layout` | `📦️packages/🦀️rust/🦀️.rs` | 3 |
| `📏️layout` | `🏭️bridge/🦀️.rs` | 2 |
| `🧩️puzzle` | `🦀️.rs` | 19 |
| `🧩️puzzle` | `🏭️bridge/🦀️.rs` | 6 |
| `🏗️fem` | `🦀️.rs` | 16 |
| `🏗️fem` | `🏭️bridge/🦀️.rs` | 6 |
| `🖍️draw` | `📦️packages/🦀️rust/🦀️.rs` | 3 |
| `🖍️draw` | `🏭️bridge/🦀️.rs` | 2 |
| `📖️playbook` | `🦀️.rs` | 2 |
| `📖️playbook` | `📦️packages/🦀️rust/🦀️.rs` | 3 |
| `📖️playbook` | `🏭️bridge/🦀️.rs` | 4 |
| `💠️lowpoly` | `📦️packages/🦀️rust/🦀️.rs` | 3 |
| `💠️lowpoly` | `🏭️bridge/🦀️.rs` | 2 |
| `🔋️energy` | `🦀️.rs` | 2 |
| `🔋️energy` | `🔨️modules/⚡️simulation/⚙️engine/📍️site/🦀️.rs` | 4 |
| `🔋️energy` | `📦️packages/🦀️rust/🦀️.rs` | 3 |
| `🔋️energy` | `🏭️bridge/🦀️.rs` | 8 |
