# Stdio Artifact Package Boundary Audit

## Scope

Read-only audit of `✏️s/🔌️plugins/🗄️stdio` on 2026-09-08. No production file was changed and no build was run.

## Current Compilation Topology

There is one compilable Rust plugin package:

- `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/Cargo.toml` declares `semio-s-plugin-stdio`, with `[lib] path = "🦀️.rs"` and the `full-artifact-catalog` feature.
- `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/🦀️.rs` is a 14,091-line, 809,562-byte facade. It contains 4,681 `#[path]` attributes and directly mounts 2,247 paths below `../../🗿️artifacts/`.
- The facade begins by declaring that it is “WIRING ONLY”, then nests every artifact under `pub mod artifacts`; see `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/🦀️.rs:1` and `:181`.

Consequently, changing any mounted Rust source invalidates one very large `semio-s-plugin-stdio` compilation unit. The desired package boundary does not exist today.

Every artifact does have a package-neutral implementation taxonomy. Each root is `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/<artifact>/🦀️.rs`, with Rust `🦀️.rs` and TypeScript `🟦️.ts` leaves below its standards/subsets/facets. The PDF and OBJ roots demonstrate the present coupling: they use `crate::registry` and `crate::artifacts::<artifact>` paths, so they are source components of the single facade rather than independently compilable crates:

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🦀️.rs`

## Artifact Inventory And Package Status

All 36 artifact roots currently exist in the taxonomy:

`☁️las`, `🌐️html`, `🌦️epw`, `🎒️zip`, `🎞️gif`, `🎥️mp4`, `🎨️svg`, `🎵️mp3`, `🏗️ifc`, `💬️bcf`, `💾️binary`, `📊️csv`, `📐️step`, `📑️tsv`, `📕️xlsx`, `📖️pdf`, `📜️docx`, `📝️md`, `📰️xml`, `📷️png`, `📸️jpg`, `📼️avi`, `📽️pptx`, `🔊️wav`, `🔤️txt`, `🔺️stl`, `🖊️dwg`, `🖋️dxf`, `🖼️tiff`, `🗜️deflate`, `🗽️obj`, `🧊️gltf`, `🧱️ply`, `🧾️json`, `🧿️semio`, and `🪟️bmp`.

There are **zero** `📦️packages` directories below `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts`. Thus all 36 are missing the requested artifact-local package declarations for both current languages.

| Current state | Artifact roots | Evidence |
| --- | --- | --- |
| Nested Rust tool/oracle crate(s), but no artifact package | `☁️las`, `🎞️gif`, `🎨️svg`, `📐️step`, `📖️pdf`, `📰️xml`, `📷️png`, `📸️jpg`, `📼️avi`, `🖋️dxf`, `🖼️tiff`, `🗽️obj`, `🧊️gltf`, `🧾️json`, `🧿️semio`, `🪟️bmp` | 38 `Cargo.toml` files occur under `🏭️generator`, `🔬️probes`, or `🏭️bridge`, for example `…/📖️pdf/…/🏭️generator/⚖️lopdf-engine/Cargo.toml`. Their implementation is co-located in `src/`, so they are not declaration-only packages. |
| Generator Nx project only, but no Rust crate | `💬️bcf`, `📜️docx` | `…/🏭️generator/📋️project.json` exists but no `Cargo.toml` occurs in the artifact. |
| Taxonomy source only | `🌐️html`, `🌦️epw`, `🎒️zip`, `🎥️mp4`, `🎵️mp3`, `🏗️ifc`, `💾️binary`, `📊️csv`, `📑️tsv`, `📕️xlsx`, `📝️md`, `📽️pptx`, `🔊️wav`, `🔤️txt`, `🔺️stl`, `🖊️dwg`, `🗜️deflate`, `🧱️ply` | No `📦️packages`, `Cargo.toml`, or `📋️project.json` below these artifact roots. |

The 32 existing `📋️project.json` files are generator-specific Nx projects, not artifact-library projects. PDF alone owns ten, under version/subset/generator paths; see `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/`.

## Boundary And Cycle Risks

1. **Root-to-artifact source inclusion is the primary performance coupling.** The root package compiles every taxonomy leaf through `#[path]`; adding manifests alone would not improve incremental compilation while this facade remains the dependency root.
2. **Artifact sources assume the current facade namespace.** `📖️pdf/🦀️.rs` and `🗽️obj/🦀️.rs` call `crate::registry` and use `crate::artifacts::*`. A package crate whose `[lib] path` points at such a source cannot retain those references unchanged. Retaining them by making each artifact depend on the stdio root reverses the desired dependency direction.
3. **There is an existing explicit back-edge.** `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/6️⃣cc6/🏭️bridge/Cargo.toml` depends by path on `semio-s-plugin-stdio` at `../../../../../../../📦️packages/🦀️rust`. If the stdio facade then depends on a STEP package, Cargo will form `stdio → step → stdio`. This bridge must target a narrower contract package or be refactored behind an artifact-owned interface before STEP joins the facade dependency graph.
4. **Cross-format composition must become explicit package dependencies.** The taxonomy already names cross-format locations, for example ZIP import mounts both `💾️binary` and `🗜️deflate` serializer/deserializer facets under `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/…/🚪️io/`. The eventual ZIP package must declare the corresponding artifact-package dependencies, keeping the graph directed and avoiding a re-created umbrella crate.

## Recommended Package-Neutral Mounting Design

For each artifact root, add declaration-only language package directories such as:

```text
🗿️artifacts/📖️pdf/
  📦️packages/
    🦀️rust/Cargo.toml
    🦀️rust/📋️project.json
    🦀️rust/📜️script.ts
    🟦️typescript/package.json
    🟦️typescript/📋️project.json
    🟦️typescript/📜️script.ts
  🦀️.rs                         # package-neutral Rust mount and implementation tree
  🟦️.ts                         # package-neutral TypeScript mount and implementation tree
```

The language manifests should point their library entry at the taxonomy mount (`[lib].path = "../../🦀️.rs"` for Rust; equivalent TypeScript entry declaration) and must contain no implementation source. Refactor that taxonomy mount into the artifact's crate root: it owns its local modules, exposes an `assembly`/descriptor API, and uses framework contracts rather than `crate::registry` or the umbrella `crate::artifacts` namespace.

The stdio package then becomes a thin component aggregator. It declares dependencies on selected artifact packages, registers each artifact's public assembly contribution, and keeps only plugin-export/component wiring. It must no longer `#[path]`-mount artifact source. The former static 14k-line facade should be generated only as an import-and-registration list, if generation remains necessary.

Nx should discover one project per artifact-language declaration directory. Each `📋️project.json` target must call only `bun ./📜️script.ts <command>`, and that script may invoke the artifact-local package build/test/check. Package-level targets make `nx affected` and Nx caching use the artifact boundary rather than treating every change as an input to `@semio-tech/stdio-plugin`.

## Migration Order

1. Introduce the small shared stdio artifact contract/registration surface without importing an artifact implementation.
2. Convert one dependency-free source-only artifact (for example `💾️binary`) end-to-end: taxonomy mount as library root, declaration-only Rust and TypeScript packages, Nx project, and an aggregator dependency.
3. Convert the other dependency-free artifacts, then encode directed inter-artifact dependencies such as ZIP → binary/deflate.
4. Resolve the STEP bridge's dependency on the root package before converting STEP. Do the same audit for each remaining nested bridge.
5. Remove the root facade's taxonomy `#[path]` mounts only when the aggregator imports the corresponding artifact packages; verify the root compile no longer treats the full catalog as its source input.

## Remaining Uncertainties

- The exact public contract split needs a compile-driven proof because current sources use umbrella-crate paths extensively.
- Nested `Cargo.toml` crates serve mixed roles (fixture generators, external-oracle readers, bridges, and libraries). Their desired ownership after artifact extraction must be decided per crate; they should not be mechanically promoted as the artifact package.
- This audit intentionally did not compile the new graph, so it makes no claim about current build or test health.
