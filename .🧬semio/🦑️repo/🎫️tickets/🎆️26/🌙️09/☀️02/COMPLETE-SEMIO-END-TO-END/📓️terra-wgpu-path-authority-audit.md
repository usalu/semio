# WGPU Path Authority Audit

## Decision

The sole authoritative package root is:

`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust`

It is a package/build-output boundary. The target-level implementation remains above it:

`…/🎯️targets/🧊️wgpu/{🧊️renderer,🧵️browser-boot,🧵️frame-worker,🧪️tests,…}`.

The legacy sibling order, `…/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu`, is not an executable package root. It happens to remain as a physical directory, which masks the bad `cwd`, but it contains no `📜️script.ts`.

## Evidence and Reproduction

All inspection commands were read-only. The direct reproduction of the command declared by the Nx `test-quick` target, from its presently configured `cwd`, produced:

```text
error: Module not found "./📜️script.ts"
```

This is exactly explained by the filesystem state:

- canonical `…/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📜️script.ts`: present;
- legacy `…/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📜️script.ts`: absent;
- legacy directory: present, so Nx can `chdir` successfully before Bun fails.

The canonical router imports without executing its command router (`canonical-router-module-load: ok`). Its project metadata contains 17 stale instances of the old root: one `sourceRoot` plus 16 target `cwd` values.

The deterministic path probes also established the source/output split:

| Current router/package lookup | Result | Authoritative counterpart |
| --- | --- | --- |
| `rust/🟦️typescript/🟦️.ts` | missing | `wgpu/🧵️browser-boot/🟦️.ts` |
| `rust/🟦️typescript/🧵️frame-worker.ts` | missing | `wgpu/🧵️frame-worker/🟦️.ts` |
| `rust/🧪️tests/🟦️.ts` | missing | `rust/🟦️typescript/🧪️test/🟦️s.ts` |
| `rust/🦀️.rs` | missing | `wgpu/🧊️renderer/🦀️.rs` (package adapter is `rust/📚️library/🦀️.rs`) |
| `rust/🟦️.ts` | missing | `rust/🟦️typescript/📚️library/🟦️.ts` |
| `rust/🟦️typescript/🟨️boot.js` and `🟨️frame-worker.js` | missing and ignored | generated package outputs; do not treat as authored source |

The existing canonical Vitest config uses `root: "../.."`, making each of its three `🧪️tests/*` inclusions resolve under the package, where all are missing. The same three files exist under the WGPU target. Bun's config check consequently reaches a missing `…/rust/🟦️typescript/🟦️.ts` module. No test or build was run because this audit was constrained to read-only operations.

## Intended Taxonomy and File Authority

| Location | Authority and repair treatment |
| --- | --- |
| `…/🎯️targets/🧊️wgpu/📦️packages/🦀️rust` | Keep all package metadata, Cargo adapter, executable `📜️script.ts`, Nx project, Trunk manifest/HTML, package TypeScript adapters, and generated browser output destination here. |
| `…/🎯️targets/🧊️wgpu/🧊️renderer`, `🎠️runtime`, `🏠️os-host`, `📐️surface-lane`, `📸️render-snapshot`, `🧵️browser-worker`, `🧵️frame-job`, `🪟️winit-app`, `⌨️native-entrypoint` | Keep as target-level authored Rust implementation. These replace the legacy flat `🦀️*.rs` paths used by root policy scans. |
| `…/🎯️targets/🧊️wgpu/🧵️browser-boot/🟦️.ts` and `🧵️frame-worker/🟦️.ts` | Keep as authored browser inputs. The router must read them here, not invent package-local source copies. |
| `…/🎯️targets/🧊️wgpu/🧪️tests/*` | Keep as target-level authored tests. The canonical Vitest config must point its `root` to this target. |
| legacy `…/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🟦️typescript/🐚️plugin-bridge.ts` | Tracked authored source: move verbatim to canonical `rust/🟦️typescript/🐚️plugin-bridge.ts`. Its three relative imports remain valid because the old and new bridge locations have the same ancestry depth to their referenced `engine`/framework nodes. |
| legacy `…/🧪️tests/🟦️.ts` | Tracked obsolete Vitest config: remove after the canonical config is repaired; do not move it. Its generic `coverage: ["index.ts"]` is not the current target test contract. |
| legacy `🟦️typescript/🟨️boot.js`, `node_modules/.vite/**`, `.DS_Store` | Ignored build/local residue: do not move into source control. Regenerate outputs in canonical `rust/🟦️typescript` after repair; clean stale local residue separately if desired. |
| legacy `.🦑️repo/🎫️tickets/**` | Historical ticket material. Do not recursively delete or move the old tree: it would destroy audit history and unrelated generated material. |

## Permanent Repair Inventory

### Nx project authority

`…/rust/📋️project.json` must replace the old root with the canonical root at line 4 and at every `cwd` below. The 16 affected target lines are 14, 21, 28, 36, 43, 50, 57, 64, 71, 78, 85, 93, 101, 110, 118, and 126:

`test`, `test-quick`, `test-native`, `test-browser-worker`, `test-preview-generated`, `check-browser-worker`, `generate-frame-worker`, `preview-generated`, `check-frame-worker`, `test-long`, `test-exhaustive`, `wasm`, `serve`, `dev`, `native`, and `lint`.

Do not alter the declared commands: all already conform to `bun ./📜️script.ts <command>`. Correcting only `sourceRoot` is insufficient; every command `cwd` must change atomically.

### Package router, browser sources, tests, and runtime bridge

`…/rust/📜️script.ts` has two distinct roots that need to be explicit:

- Lines 145–155: leave generated destinations under `this.root/🟦️typescript/{🟨️boot.js,🟨️frame-worker.js}`, but change the two input entry paths from package-local missing paths to target-level `../../🧵️browser-boot/🟦️.ts` and `../../🧵️frame-worker/🟦️.ts`.
- Lines 289–312: change all three Vitest config arguments from missing `🧪️tests/🟦️.ts` to the existing canonical `🟦️typescript/🧪️test/🟦️s.ts`.
- Lines 351–354: point the colour-literal scan at authored `../../🧊️renderer/🦀️.rs`; the current package-local `🦀️.rs` is absent, so lint silently scans nothing.
- Lines 168–198: retain `this.root` as the Trunk working directory. Trunk must still consume the package-local `Cargo.toml`, `Trunk.toml`, HTML, and generated browser outputs.

`…/rust/🟦️typescript/🧪️test/🟦️s.ts:4` must set its Vitest root to `../../../..` (the WGPU target), so its existing line-8 `🧪️tests/*` inclusions and line-9 renderer-boot coverage pattern resolve to real authored files.

Move the bridge to canonical `rust/🟦️typescript/🐚️plugin-bridge.ts`, then repair both consumers:

- `…/🧊️renderer-boot/🟦️.ts:6`: replace the old cross-engine import with `../📦️packages/🦀️rust/🟦️typescript/🐚️plugin-bridge.ts`.
- `…/🧪️tests/🟦️package-integration.ts:8`: import the bridge from `../📦️packages/🦀️rust/🟦️typescript/🐚️plugin-bridge.ts`.
- `…/🧪️tests/🟦️package-integration.ts:105`: pass `../🧵️browser-boot/🟦️.ts` to `renderBrowserEntry`; the current test-local `🟦️typescript/🟦️.ts` path does not exist.

### Package, Cargo, and Trunk metadata

`…/rust/package.json` requires two metadata corrections:

- line 10: point `exports["."]` at the actual `./🟦️typescript/📚️library/🟦️.ts` adapter, rather than absent `./🟦️.ts`;
- line 33: update `repository.directory` to the canonical products/modules/engine/targets/packages taxonomy.

Its Nx scripts (lines 12–18) are already correctly delegated to Nx and need no change.

`Cargo.toml` needs no relocation rewrite. Its external dependency paths remain valid because the package remains at the same depth and every listed relative route ascends beyond the reordered package/target nodes. The Cargo lib and binary paths already use canonical adapters. Likewise, `build.rs:7` deliberately includes `../../🏗️builder/🦀️.rs`: from the canonical package this resolves to the target-level builder and must remain unchanged. The TypeScript library adapter at `🟦️typescript/📚️library/🟦️.ts:2` is also correct.

`Trunk.toml` retains the correct package-local build target, dist path, and HTML relationship. Its watch list is not correct: line 8 watches nonexistent `🦀️.rs` and fails to watch the authored browser entries outside the package. Replace that entry with explicit target input directories (at least `../../🧊️renderer`, `../../🧵️browser-boot`, `../../🧵️frame-worker`, and their browser-transport dependencies), or an equivalent safely scoped target-root watch. Do not redirect `🌐️.html:7–12` to target sources: it correctly consumes package-local Cargo and generated outputs.

### Root policy router and launch verification

The root `📜️script.ts` contains active policy/audit paths, not harmless documentation. A literal token replacement is unsafe because the old flat file names were split into domain directories. Update these references semantically:

| Root-script lines | Old flat intent | New authored target |
| --- | --- | --- |
| 10155, 10775, 11706, 13573, 14230 | renderer glue | `…/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs` |
| 10745, 14544 | browser worker | `…/🎯️targets/🧊️wgpu/🧵️browser-worker/🦀️.rs` |
| 11700 | runtime | `…/🎯️targets/🧊️wgpu/🎠️runtime/🦀️.rs` |
| 11707 | OS host | `…/🎯️targets/🧊️wgpu/🏠️os-host/🦀️.rs` |
| 11708 | surface lane | `…/🎯️targets/🧊️wgpu/📐️surface-lane/🦀️.rs` |
| 11709, 12091, 14232 | winit host | `…/🎯️targets/🧊️wgpu/🪟️winit-app/🦀️.rs` |
| 11778 | native binary | `…/🎯️targets/🧊️wgpu/⌨️native-entrypoint/🦀️.rs` |
| 12090, 14231 | frame job | `…/🎯️targets/🧊️wgpu/🧵️frame-job/🦀️.rs` |
| 12092, 14233 | render snapshot | `…/🎯️targets/🧊️wgpu/📸️render-snapshot/🦀️.rs` |

Lines 11464 and 11623 are native launch command expectations, so they must instead point to the canonical package `…/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📜️script.ts`. This matters immediately: both `.vscode/🧩️launch.seed.jsonc` and generated `.vscode/launch.json` already use the canonical command (42 seed entries and 65 generated entries; zero old-path entries). Leaving the root verifier old makes correct launch data fail its own validation.

The launch seed is authoritative and the generated launch file is already aligned. This packet should not hand-edit either; after root verification is repaired, run the repository's normal launch generation/verification path and require the resulting generated launch file to remain canonical.

### Generated registries and lockfiles

These active generated records still name the old package and must be regenerated, not hand-edited:

- `bun.lock:437` and `1457` retain the legacy workspace root; regenerate through Bun after the physical/source repair.
- `🔒️dependencies.json:1801, 1874, 2223, 2243, 2389, 2497, 2539, 2965, 3179, 3286, 3424, 3462, 3546, 3561, 3633, 3662, 3712, 3764` retain legacy package/Cargo paths. Its owning root command is `bun ./📜️script.ts verify dependencies write-baseline`; run it only after the package path graph is correct.

The root `package.json` workspace entry already names the canonical package and must remain as-is.

## Deliberate Old-Path References That Must Remain

Do not globally replace `📦️packages/🦀️rust/🎯️targets/🧊️wgpu`.

1. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json:12433–12439` is a nested-Cargo projection record. Its `sourceManifestPath` deliberately identifies the legacy input while `destinationManifestPath` asserts the canonical package. Its following `sourceModulePaths` are migration/projection inputs. Rewriting those source tokens would destroy the source→destination test contract.
2. The nested-Cargo fixture snapshots under `…/📦️packages/🟦️typescript/{🧫️fixtures,🧪️fixtures}` deliberately contain old source and canonical destination tokens: `nested-cargo-package-authority`, `nested-cargo-package-projection`, and both `nested-cargo-package-purity` fixtures. They are test data, not live routes.
3. Historical `.🧬semio` ticket reports, scratch artifacts, and `.cursor/plans` document previous topology; preserve them.
4. Several matches sharing the shorter tail belong to the separate UI WGPU package (`🧰️framework/🔨️modules/🖱️ui/…/🎯️targets/🧊️wgpu`), not this renderer package. The renderer's `include_str!` UI-source assertions must not be rewritten.

## Risks of Blind Replacement

- Replacing only the old package-root prefix in the root policy script leaves references such as `…/📦️packages/…/🧊️wgpu/🦀️winit_app.rs` pointing to nonexistent package-flat files. Each maps to a different target directory.
- Moving ignored boot/Vite output into the package would falsely promote build products to authored files and leave Trunk's deterministic generation contract broken.
- Deleting the legacy directory recursively would delete retained ticket history and developer-local caches, neither of which is part of the package migration.
- Rewriting taxonomy `sourceManifestPath` or fixture source tokens erases the regression test that proves this migration.
- Updating launch files alone is counterproductive: they are already correct; the root verifier is the stale party.

## Minimal Dependency-Ordered Sol Repair Packet

1. Move only tracked `🐚️plugin-bridge.ts` to canonical package TypeScript; remove the obsolete tracked legacy Vitest config. Leave historical/ignored legacy residue untouched.
2. Repair canonical package router source-vs-output paths, Vitest config arguments, lint input, canonical Vitest root, target test imports, renderer-boot bridge import, package export, repository metadata, and Trunk watches as listed above.
3. Atomically repoint `📋️project.json` `sourceRoot` and all 16 `cwd` values to the canonical root.
4. Update every active old renderer path in root `📜️script.ts` using the semantic mapping table, including native command expectations and audit file constants.
5. Regenerate Bun lock and dependency baseline with their owning commands; retain taxonomy/fixture source-to-destination records.
6. Run normal launch generation/verification without hand-editing the already-canonical seed/generated launch files.
7. Verify, in order: canonical browser-output generation/check; `bun nx run @semio-tech/framework-renderer-wgpu:test-quick`; browser-worker and preview-generated targets; Trunk build/serve smoke; native smoke; root dependency and interactivity/launch verification. Confirm generated `🟨️boot.js` and `🟨️frame-worker.js` appear only under canonical package output paths.

## Audit Boundary

No production/configuration code was edited by this audit. The only changed file is this report. Repository MCP ticket/goal resources were not available in the current tool inventory; the existing umbrella ticket path supplied by the task was used directly.
