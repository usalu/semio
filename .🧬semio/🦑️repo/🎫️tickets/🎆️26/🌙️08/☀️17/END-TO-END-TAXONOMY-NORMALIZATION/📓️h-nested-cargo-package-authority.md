# Nested Cargo Package Authority

## Scope and safety boundary

This is a read-only classification of the three workspace-nested, non-ticket Cargo residuals identified by `📓️s-cargo-stem-residual-classification.md`. No production/schema/script/Git state was changed. No `compose/**`, `temp/compose/**`, or `temp-compose/**` path was traversed or read. Content scans used explicit non-excluded pathspecs; retained ticket evidence was counted separately from live consumers.

The retained inventory is pre-transaction-v2 evidence, not final acceptance:

- inventory: `📊️taxonomy-inventory/🔣️.json`
- bytes: `116,981,622`
- SHA-256: `f03a718e8069da55f53606add55a8417f1ccb91c1e0ead3f182daa08dfc19f10`

The earlier report spells the WGPU directory `🧊gpu`. The current admitted tree and inventory-normalized identity are NFC/VS16-canonical `🧊️wgpu`; this report uses the exact current identity.

## Decision summary

All three files have external Cargo authority. None should create a semantic directory named `Cargo`, and none warrants a broad fixture or basename exception.

| Current file | Classification | Exact semantic owner | Exact destination |
|---|---|---|---|
| `…/engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/Cargo.toml` | Handwritten authoritative Cargo package metadata | Registered target owner `engine/🎯️targets/🧊️wgpu` | `…/engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/Cargo.toml` |
| `…/🧫️fixtures/🔌️jcoprobe/👽️guest/Cargo.toml` | Handwritten authoritative isolated fixture workspace/package metadata | Exact JCO component-guest owner `…/🔌️jcoprobe/👽️guest` | `…/🔌️jcoprobe/👽️guest/📦️packages/🦀️rust/Cargo.toml` |
| `…/🧫️fixtures/🔌️jcoprobe/👽️guest/Cargo.lock` | Cargo-generated, tracked reproducibility evidence owned by the isolated fixture package | Same JCO component-guest package | `…/🔌️jcoprobe/👽️guest/📦️packages/🦀️rust/Cargo.lock` |

The destination paths are collision-free and within the 240-byte budget:

| File | Source bytes | Destination bytes | Destination exists |
|---|---:|---:|---|
| WGPU `Cargo.toml` | 163 | 163 | no |
| JCO `Cargo.toml` | 98 | 126 | no |
| JCO `Cargo.lock` | 98 | 126 | no |

## WGPU renderer package

### Authority

Exact current root:

```text
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu
```

The manifest is 8,382 bytes, mode-preserved tracked content, SHA-256:

```text
87379ad73bdb3209522852ea49d2f47246461dfc2ab6e52795d25c801ee35fb6
```

Its authority is conclusive:

- `[package].name = "semio-framework-os-renderer-wgpu"`.
- `[package.metadata.semio]` declares `role = "framework"`, `id = "renderer-wgpu"`.
- The root Cargo workspace explicitly lists the current directory at `Cargo.toml:72`.
- Root `package.json:29` independently lists the same package directory.
- `📋️project.json` declares the same directory as an Nx library source root.
- `cargo metadata --no-deps --locked --offline` resolves it as a member of the repository workspace with a `cdylib`/`rlib`, native binary, and build-script target.

This is not generated evidence and not an incidental target leaf. It is a first-class Cargo and Nx package boundary whose structural order is inverted. Current schema already registers `targets`, `wgpu-target` with parent `targets`, `packages`, and `rust-language` with parent `packages`. The unique schema-derived owner order is therefore:

```text
engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust
```

The existing stale future-looking string `engine/🧊️wgpu/📦️packages/🦀️rust` in the OS dev script is not authority: it omits the required registered `🎯️targets` parent and must be corrected to the full destination above.

### Transaction boundary

Relocating only `Cargo.toml` would split Cargo metadata from its configured targets and break the package. The atomic unit is all 32 admitted files plus the one admitted `🟦️typescript` descendant directory under the current WGPU root. Cargo-configured target leaves include `🦀️lib.rs`, `📦️bin.rs`, and `build.rs`; Nx/package/script/Trunk/browser/Rust/test leaves share the same owner.

The current physical root also contains ignored local material (`.DS_Store`, `.🦑️repo`, and `node_modules`). These are not in the Git-index-exact inventory and must not be moved by a parent-directory rename. The transaction must move the 32 admitted files individually, create admitted destination directories, and leave ignored/cache material outside the plan. A preflight should fail if any ignored node would be overwritten or followed.

Because source and destination package roots have equal depth, existing relative ascents in `Cargo.toml`, `Trunk.toml`, `$schema` values, and scripts retain their depth semantics. Intra-package relative links remain valid. Absolute/repository-relative tokens below require edits.

## JCO probe guest package

### Authority

Exact current root:

```text
🧰️framework/🛍️products/💻️os/🧫️fixtures/🔌️jcoprobe/👽️guest
```

The two Cargo files are unchanged from the retained inventory:

| File | Bytes | SHA-256 |
|---|---:|---|
| `Cargo.toml` | 584 | `41cac3870bbc659710d902b34a66f69a621a3a9cd39a10d5bb551e937fd3da2c` |
| `Cargo.lock` | 11,211 | `15f877b56093fdbcc4a9aa06512583a989684068e944202f46885c4eca0986c8` |

`Cargo.toml` is handwritten package authority:

- `[package].name = "semio-jcoprobe-guest"`, `publish = false`.
- A literal empty `[workspace]` intentionally makes this directory its own Cargo workspace root rather than a root-workspace member.
- `[lib] crate-type = ["cdylib"]` and `path = "🦀️component.rs"` bind the manifest to the adjacent source.
- `wit-bindgen` is the sole dependency.

The owner is also independently evidenced by the code and schema: `🦀️component.rs` invokes `wit_bindgen::generate!` against `🧬️schema/📜️world.wit`, implements the generated `Guest`, and exports `Component`; the WIT file defines package `semio:jcoprobe@0.1.0` and world `jcoprobe`.

`cargo metadata --no-deps --locked --offline` confirms:

```text
workspace root = …/🔌️jcoprobe/👽️guest
workspace member = semio-jcoprobe-guest@0.1.0
target = cdylib at 🦀️component.rs
```

`Cargo.lock` begins with Cargo's generated-file header and version 4. It is generated evidence, but it is authoritative tracked reproducibility state for this deliberately isolated workspace. It must move with, and be verified against, the package; it must not be discarded or treated as handwritten semantic content.

### Transaction boundary

The atomic admitted boundary is four files and one descendant directory:

```text
Cargo.toml
Cargo.lock
🦀️component.rs
🧬️schema/📜️world.wit
```

Move them together beneath:

```text
👽️guest/📦️packages/🦀️rust
```

The configured `lib.path` and WIT `path` stay relative to the package root and therefore remain byte-identical when the whole four-file package moves. The workspace root and Cargo target directory move to the new package root. The adjacent `🌐️harness` is a consumer/evidence sibling, not part of this Cargo boundary; no live harness source contains a path token to the current guest Cargo root.

This report does not rename `🔌️jcoprobe` or invent a new emoji. If the existing `👽️guest` directory is not yet governed globally, admit it through an exact owner-local JCO component-guest identity contract; do not add a wildcard `guest` or fixture exception.

## Live non-Compose reference edits

### WGPU: 191 path-token occurrences in 18 files

The primary exact current renderer-root token occurs 183 times in 14 files:

| File | Count | Exact current line locations |
|---|---:|---|
| `.storybook/scopes.ts` | 2 | 129, 136 |
| `.vscode/launch.json` | 65 | 158, 209, 291, 342, 393, 444, 495, 546, 598, 649, 700, 751, 802, 864, 915, 966, 1017, 1068, 1289, 1340, 1391, 1442, 1493, 1588, 1639, 1690, 1741, 1837, 1888, 2186, 2248, 2299, 2350, 2401, 2452, 2601, 2880, 2931, 2982, 3033, 3293, 3344, 3984, 4021, 4058, 4117, 4176, 4235, 4294, 4353, 4412, 4471, 4530, 4589, 4648, 4707, 4766, 4825, 4884, 4943, 4980, 5039, 5076, 5135, 5172 |
| `.vscode/🧩️launch.seed.jsonc` | 42 | 120, 171, 215, 228, 241, 254, 267, 280, 293, 306, 319, 332, 345, 369, 382, 395, 408, 459, 528, 541, 592, 605, 656, 713, 726, 739, 752, 810, 861, 1121, 1145, 1158, 1209, 1222, 1235, 1249, 1488, 1501, 1514, 1527, 1749, 1762 |
| `Cargo.toml` | 1 | 72 |
| `bun.lock` | 2 | 405, 1408 |
| `package.json` | 1 | 29 |
| root `📜️script.ts` | 20 | 8183, 8767, 8797, 9482, 9641, 9718, 9724–9727, 9796, 10108–10110, 11568, 12225–12228, 12539 |
| `🔒️dependencies.json` | 18 | 1755, 1828, 2138, 2158, 2304, 2412, 2454, 2862, 3063, 3186, 3308, 3346, 3430, 3445, 3517, 3546, 3596, 3648 |
| `…/♾️infinite/🌍️world/🦀️component.rs` | 5 | 11478–11481, 12750 |
| WGPU `📋️project.json` | 16 | 4, 14, 21, 28, 35, 42, 49, 56, 63, 70, 77, 85, 93, 102, 110, 118 |
| `…/🔌️plugin/📇️registry/🖥️launch.ts` | 1 | 116 |
| `…/🧑️‍💻️dev/…/⚙️vite.config.ts` | 1 | 95 |
| `…/🧑️‍💻️dev/…/📜️script.ts` | 4 | 1891, 1893, 2161, 5045 |
| taxonomy `🔣️taxonomy.json` | 5 | 1109, 1114–1116, 1118 |

Eight additional prose/relative leaf references name a file below `🎯️targets/🧊️wgpu` without the full renderer prefix and must gain `/📦️packages/🦀️rust`:

| File | Locations | Meaning |
|---|---|---|
| `…/🏃️run/🦀️component.rs` | 38, 1424, 1443, 1456, 1525 | `📦️glue.rs` and members under it |
| `…/💻️os/🖥️host/🎠️activation.rs` | 197 | `📦️glue.rs` helper |
| `…/🔌️plugin/🖥️host/🧵️shard/🦀️component.rs` | 941 | WGPU `🎠️runtime.rs` |
| moved WGPU `📦️glue.rs` | 7036 | own `🎠️runtime.rs` |

Owner-only references such as `🎯️targets/🧊️wgpu/**` remain semantically valid because the destination package stays beneath that exact owner. Three `include_str!` references in WGPU `📦️glue.rs:10108–10110` point to the separate framework UI WGPU target and must not be rewritten by this relocation.

One additional pre-existing metadata defect is not part of the 191 exact Unicode path tokens: WGPU `package.json:33` says `framework/product/os/module/renderer/wgpu`. Its repository directory is already not the current path and should be corrected to the exact destination during the same package-manifest edit.

Generated/reference-owner disposition:

- Edit source authority, then regenerate `.vscode/launch.json` from the launch seed/registry; do not hand-edit its 65 rows.
- Regenerate `bun.lock` after root workspace/package path edits.
- Regenerate `🔒️dependencies.json` through its existing owner after Cargo/package paths settle.
- Update the launch seed, plugin launch registry, root Cargo/Node workspaces, root task/reference tests, Storybook scopes, Nx project, taxonomy exact identities, dev/Vite consumers, Rust `include_str!` sites, and precise prose locations transactionally.

### JCO: 3 live path-token edits in 3 files

| File | Location | Required action |
|---|---:|---|
| `🔒️dependencies.json` | 3661 | Regenerate the manifest identity to the destination. |
| taxonomy `🔣️taxonomy.json` | 895 | Update the exact tracked-lock identity through schema ownership. |
| moved `👽️guest/🦀️component.rs` | 4 | Change the prose schema location to include `📦️packages/🦀️rust`, or make it package-root-relative. |

The Cargo `lib.path = "🦀️component.rs"` and `wit_bindgen` path `🧬️schema/📜️world.wit` remain valid and must not be edited. The `semio-jcoprobe-guest` strings in `Cargo.toml` and `Cargo.lock` are package identity, not path references.

## Retained evidence references

Historical ticket evidence is not a live consumer and must not be rewritten to pretend the old execution used the new path. Before creating this report, exact tracked ticket-evidence scans found:

- WGPU current full source prefix: 5,991 occurrences across 208 `.🧬semio` files.
- JCO current full guest prefix: 10 occurrences across 6 `.🧬semio` files.

These include compiler logs, reports, the retained inventory, and fixture copies. Preserve their bytes. Stale-output verification must distinguish live governed consumers from immutable evidence; otherwise relocation would attempt thousands of false historical edits.

## Smallest schema-first closure

1. Add an exact structural package projection for the WGPU identity:

   ```text
   source owner segments: engine / packages / rust / targets / wgpu
   destination owner segments: engine / targets / wgpu / packages / rust
   package identity: semio-framework-os-renderer-wgpu
   verification: root cargo metadata + Nx project + Bun workspace
   ```

   Reuse existing `targets`, `wgpu-target`, `packages`, and `rust-language` kinds; no new semantic emoji is needed.

2. Add an exact owner-local external package projection for JCO:

   ```text
   source owner: …/fixtures/jcoprobe/guest
   destination package root: <owner>/packages/rust
   identity: semio-jcoprobe-guest
   required manifest evidence: [package] + [workspace] + configured cdylib
   required source evidence: component.rs + schema/world.wit
   verification: cargo metadata --locked --offline
   ```

   The exact `jcoprobe/guest` identity is authority. Do not authorize arbitrary fixture workspaces or arbitrary `guest` directories.

3. The existing `cargo-manifest` fixed contract becomes applicable once each manifest is at a canonical Rust package root. Add a strict Cargo-lock authority for the isolated JCO workspace: either an exact JCO destination identity or a package-root contract that additionally requires an adjacent Cargo manifest whose parsed root contains `[workspace]`. Pattern match alone is insufficient.

4. The projection contract must carry the complete admitted subtree, preimage hashes, source/destination package identity, reference edits, and generator regenerations. It must explicitly reject ignored-node capture, collisions, stale workspace manifests, stale generated registries, or a Cargo metadata package root different from the destination.

## TDD and acceptance checks

1. Language-neutral golden vectors contain all three exact source/destination file mappings and the two package-root mappings.
2. WGPU authority passes only when root Cargo metadata, root Node workspace, and Nx source root agree on `semio-framework-os-renderer-wgpu`.
3. WGPU plan contains exactly 32 admitted file moves; ignored `.DS_Store`, `.🦑️repo`, and `node_modules` nodes are absent and never followed.
4. JCO authority passes only for the exact guest root with `[package]`, empty `[workspace]`, configured `cdylib`, adjacent component source, WIT world, and matching Cargo lock.
5. Removing `[workspace]`, changing the package name, moving the lock to a sibling, or presenting a production lookalike fails closed.
6. `cargo metadata --no-deps --locked --offline` resolves the destination WGPU package in the repository workspace and the destination JCO package as its own workspace.
7. Root `Cargo.toml`, root `package.json`, Nx project, launch source, dependency registry, Bun lock, Storybook, Vite/dev, Rust include sites, schema identities, and all 191 WGPU live path tokens converge to the destination; the three JCO live tokens converge likewise.
8. Generated launch, Bun lock, and dependency-registry checks are green after regeneration; preview/check output roots remain unchanged during read-only checks.
9. Retained ticket evidence hashes remain unchanged even though it contains old paths.
10. A second inventory/plan is empty for these three residuals, with no `semantic-stem-unresolved`, `package-implementation-destination-unresolved`, collision, or stale-reference violation.

## Risks and blockers

- WGPU is a mixed Rust/TypeScript package with 32 admitted leaves and ignored local material. A directory-level filesystem rename would capture excluded caches; only Git-index-exact node moves are safe.
- WGPU has 191 live path tokens and three generated output families. Apply must update sources before regeneration and verify exact preimages.
- The JCO guest has no Nx project/script owner. Cargo metadata is the authoritative verifier; do not fabricate a generator owner.
- JCO `Cargo.lock` is generated but intentionally tracked. Any regeneration must use the pinned Cargo/toolchain and compare canonical bytes rather than silently accepting drift.
- The current schema's package-root fixed scope does not recognize these inverted/noncanonical roots. Broadening `**/Cargo.toml` or `**/Cargo.lock` would bless unrelated production lookalikes and is rejected.
