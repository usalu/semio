# Stdio Artifact Extraction Boundary Plan

## Verified Topology

The current `semio-s-plugin-stdio` package is one Rust compilation unit. Its package-local `🦀️.rs` is 14,091 lines / 809,562 bytes, contains 4,681 `#[path]` mounts, and reaches 2,247 paths under `🗿️artifacts`. Its corresponding TypeScript package is a 36-export barrel. All 36 artifact roots have package-neutral Rust and TypeScript source, but none has an artifact-local first-party package declaration directory.

The 36 roots are `binary`, `txt`, `json`, `xml`, `csv`, `md`, `deflate`, `zip`, `step`, `ifc`, `las`, `gltf`, `obj`, `stl`, `ply`, `dwg`, `dxf`, `svg`, `bmp`, `png`, `jpg`, `gif`, `tiff`, `pdf`, `docx`, `pptx`, `xlsx`, `bcf`, `semio`, `mp4`, `avi`, `mp3`, `wav`, `epw`, `tsv`, and `html`.

Sources show the monolith is structural rather than an accidental build input:

- Every artifact root calls `crate::registry::{runtime_assembly, format_descriptors_for}` and its local leaves use `crate::artifacts::<artifact>`.
- The root registry at `📇️registry/🦀️.rs` combines generic definition parsing with static references to all 36 artifact assembly functions and all 26 native codec factories.
- The current component root calls `registry::artifact_assemblies()` and mounts the global `editor` and `viewer` trees from the package-local facade.

Adding manifests without changing those three relationships would either fail to compile or pull the monolith back into every leaf package.

## Required Dependency Direction

Use these package names, derived from the existing schema identities:

```text
semio-s-plugin-stdio                       component aggregation only
semio-s-artifact-stdio-contract            shared artifact declaration / registration contract
semio-s-artifact-stdio-<artifact>          one Rust package for each of the 36 roots
@semio-tech/stdio-<artifact>-rs            matching Nx project identity
@semio-tech/stdio-<artifact>               matching TypeScript package identity
```

The only permitted direction is:

```text
stdio component -> selected artifact packages -> artifact contract + framework
selected artifact package -> lower artifact packages + artifact contract + framework
```

No artifact package may depend on `semio-s-plugin-stdio`, directly or through a bridge, and the contract must not depend on an artifact package. `semio` is the top composition artifact: it currently imports 28 other artifact models. It must declare those package dependencies; no other stdio artifact imports `semio`, so it remains a DAG apex rather than a cycle.

The definite current exception is `📐️step/.../6️⃣cc6/🏭️bridge/Cargo.toml`, which depends on the component package `semio-s-plugin-stdio`. Refactor that bridge to depend on the STEP artifact package before making the component depend on STEP. Otherwise Cargo creates `stdio component -> step -> stdio component`.

## Shared API Seam

Move only generic, artifact-independent code from `📇️registry/🦀️.rs` to the package-neutral source root `📇️registry/🧬️contract/🦀️.rs`, exposed by the declaration-only `📇️registry/🧬️contract/📦️packages/🦀️rust/Cargo.toml`. Its public API should be a compact contribution contract:

```rust
pub struct ArtifactContribution {
    pub identity: &'static str,
    pub definition: fn() -> Result<ArtifactDefinition, PluginAssemblyError>,
    pub assembly: fn() -> Result<ArtifactAssembly, PluginAssemblyError>,
    pub formats: fn() -> Result<Vec<FormatDescriptor>, ArtifactDefinitionError>,
    pub native_codecs: fn() -> Result<Vec<NativeCodecFactory>, PluginAssemblyError>,
}

pub fn definition_from_schema(schema: &'static str) -> Result<ArtifactDefinition, PluginAssemblyError>;
pub fn runtime_assembly(
    identity: &'static str,
    definition: ArtifactDefinition,
    declaration: fn(ArtifactDefinition) -> Result<ArtifactDeclaration, ArtifactDefinitionError>,
) -> Result<ArtifactAssembly, PluginAssemblyError>;
pub fn format_descriptors(schema: &'static str) -> Result<Vec<FormatDescriptor>, ArtifactDefinitionError>;
```

`NativeCodecFactory` and receipt validation also move to this contract, but factory constructors stay in the owning artifact package. An artifact with no native codec contributes an empty vector. This removes the contract-to-all-artifacts reverse edge while retaining the existing 26-codec validation in the aggregator.

Each artifact root exposes `contribution() -> ArtifactContribution` and owns its own source-schema `include_str!`, declaration, formats, codec factories, editor/viewer surface installation, and direct framework contracts. The aggregator owns exactly the selected contribution list, component package id, `plugin_exports!`, catalog-wide validation, and feature-gated surface registration. It must not mount taxonomy code with `#[path]`.

The 36 artifacts cannot retain `crate::artifacts::<self>` or `crate::registry` references. At extraction, change self paths to `crate::...`, replace registry calls with the contract crate, and convert cross-artifact imports to the declared lower artifact crate. Place all Rust mount code at each artifact's existing `🗿️artifacts/<artifact>/🦀️.rs` root and its domain children. Do not put `🦀️.rs` implementation files under `📦️packages` and do not add an umbrella compatibility facade.

## Declaration And Nx Shape

For every artifact add only declarations at:

```text
🗿️artifacts/<artifact>/📦️packages/🦀️rust/Cargo.toml
🗿️artifacts/<artifact>/📦️packages/🦀️rust/📋️project.json
🗿️artifacts/<artifact>/📦️packages/🦀️rust/📜️script.ts
🗿️artifacts/<artifact>/📦️packages/🟦️typescript/package.json
🗿️artifacts/<artifact>/📦️packages/🟦️typescript/📋️project.json
🗿️artifacts/<artifact>/📦️packages/🟦️typescript/📜️script.ts
```

Each Rust manifest uses `[lib] path = "../../🦀️.rs"`; the implementation remains at the artifact root. Artifact manifests declare only framework, contract, and direct lower-artifact dependencies. Add all 36 manifest directories to the root Cargo workspace. The component manifest declares optional dependencies and uses one feature per artifact, with `home-io` selecting its current eight-artifact closure and `full-artifact-catalog` selecting all 36.

The existing custom Nx plugin already discovers Cargo manifests, follows Cargo path dependencies and literal Rust mounts from a `lib.path`, and hashes package-neutral source inputs. It requires no discovery-plugin change. Artifact `📋️project.json` targets must call their local `bun ./📜️script.ts <command>` only; scripts use existing shared Cargo/Nx helpers. Add schema-derived launch entries in the existing build/gate groups. Do not parallelize Cargo package tasks while the workspace shares a target directory.

For TypeScript, a Node `exports` target may not escape the package root (`../..` is invalid). Therefore a declaration-only TypeScript package cannot expose the artifact-root `🟦️.ts` by ordinary Node resolution. Keep the artifact source as the authoritative TS entry and use the root schema generator to configure internal TypeScript/Nx path resolution, or introduce a supported repository package-resolution mechanism before advertising individual npm runtime entrypoints. Do not solve this with symlinks: they violate the required zero-touch native Windows compatibility. The Rust split is the immediate compiler-performance boundary.

## Migration Order

1. Add the schema-first package/contribution inventory and language-agnostic positive and negative contract fixture; derive names, workspace members, Nx identities, aggregate selection and launch entries from it.
2. Extract the generic registry contract and change the component registry to consume contributions. Keep its selected list feature-gated.
3. Prove the boundary with `binary`, then convert independent leaves. Each package-root source owns local mounts outside `📦️packages`.
4. Add directed dependencies in the verified order: `binary -> txt -> xml/json -> deflate -> zip`; then geometry/document/media artifacts; finally `semio`.
5. Refactor the STEP CC6 bridge before STEP enters the component feature list. Audit each nested bridge/generator separately; generator/oracle packages are not artifact implementation packages.
6. Remove root facade artifact/editor/viewer mounts as their contributions become direct dependencies. Complete the all-at-once consumer import migration; do not preserve the old umbrella API.
7. Validate one independent package, one dependent package, the current home-io closure, the full 36-artifact component, Cargo metadata DAG, Nx project graph and a repeated cache restoration. Compare a selected codec result with an existing third-party generator/probe.

## Evidence

- Monolithic facade: `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/🦀️.rs:181`, component manifest, and Nx project.
- Current all-artifact registry: `✏️s/🔌️plugins/🗄️stdio/📇️registry/🦀️.rs:719-1053`.
- Component assembly: `✏️s/🔌️plugins/🗄️stdio/🦀️.rs:203-212`.
- Package-neutral artifact roots: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/*/{🦀️.rs,🟦️.ts}`.
- STEP bridge back-edge: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/6️⃣cc6/🏭️bridge/Cargo.toml`.
- Nx Cargo discovery and source ownership: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs:260-330`, `:588-645`.
