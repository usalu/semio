# Nx Artifact Package Execution

## Scope

This execution owns the reusable Nx/Bun package boundary, all stdio artifact package declarations, the 24 single-artifact plugin extractions, the Norm and multi-artifact Nx declarations, the three Block TypeScript declarations, launch integration, and package-contract/cache evidence.

## Implemented

- Added schema-first stdio package fixtures and a repository-wide language-neutral artifact package contract backed by Ajv and Cargo/Nx metadata.
- Registered 36 stdio Rust and 36 stdio TypeScript artifact projects. TypeScript declarations export only built `dist` JavaScript and declaration files.
- Added the neutral Rust and TypeScript package routers to the existing Cargo `📜️script.ts`. Expensive native work supports progress and process-tree cancellation; native time limits use `SEMIO_BUILD_BUDGET_MS` and default to unlimited.
- Extracted and declared 24 single-artifact plugin Rust packages with direct taxonomy-root `lib.path = "../../🦀️.rs"` ownership and private composition mounts.
- Added Nx declarations/routers for Norm contract plus 15 Norm artifacts and 17 multi-artifact packages.
- Added declaration-only TypeScript packages for Block 2D, 3D, and 5D with emitted declarations and real package-name resolution tests.
- Standardized the seven framework artifact Nx names under `@semio-tech/framework-*` and added Space/Collection Nx declarations.
- Added authoritative artifact contract/selector launch configurations to `.vscode/🧩️launch.seed.jsonc`; generation produced matching entries in `.vscode/launch.json`.
- Replaced Lowpoly's optional CAD plugin dependency with the direct CAD artifact package.
- Remounted Draw FSM runtime and proc-macro crates directly from their taxonomy roots and removed package-local Rust implementation files. Updated the live taxonomy descendant contract to require declaration-only package roots.
- Memoized native dependency-root closures in the Nx plugin. Direct normalization improved from 34.5 s to 24.8 s for 446 projects after warm repository file discovery.
- Rewrote all 36 stdio TypeScript package routers to use the neutral artifact builder directly. Artifact TypeScript targets now hash their exact transitive source imports and exact command implementation rather than the repository-wide library tree.
- Generalized Nx native target normalization so local artifact wrappers use `nativeSources`/`nativeTestSources` and their actual Cargo dependency owners.
- Removed Writer's obsolete public artifact host re-export and duplicate package-level example mounts.

## Passing evidence

- `cargo metadata --offline --format-version 1` refreshed the workspace lock successfully after the Lowpoly dependency change.
- Block 3D TypeScript package test passes with progress reporting, JavaScript build, `.d.ts` emission, package-name `tsc` resolution, and runtime module resolution (`🗑️generated/block-3d-typescript-test-async.txt`). Earlier direct Block 2D/3D/5D package tests also pass.
- The stdio home-I/O source/fixture gates pass, including direct/shared/full feature closure and third-party Ajv/TOML oracles.
- Direct stdio PDF TypeScript build/test passes with runtime package-name resolution and emitted declaration consumption.
- Launch seed/output validation finds one contract configuration, one selector configuration, and both required inputs.
- Authoritative isolated `bun x nx show project @semio-tech/stdio-pdf-rs --json` passes with `NX_ISOLATE_PLUGINS=false`. The normalized project hashes PDF taxonomy sources through `production -> default`; `nativeSources` also enumerates the mounted PDF source graph.
- Repository-wide package contract passes schema, declaration, source-owner, and Nx ownership discovery and reaches Cargo metadata.
- PDF cache restoration and source isolation pass with exact inputs. Baseline task hash `2961489084152580833`; deleting only PDF `dist` produced a 100% local hit and restored JS (`28f47dcb40e97669e54c02a5cddc600d294e8298cf6b2cc6827110e4cbc90dd3`) plus `.d.ts` (`4a51329398088b7239d0e916c6c01177000713c6dbf5264a7f62b95a639b3e89`). A temporary PDF source marker caused a cache miss at hash `6201406382484965991`. After exact PDF restoration, a temporary JPG-only source marker plus PDF `dist` deletion returned a 100% local hit at the original hash and restored identical outputs. All probe markers were removed.

## Current failures and pending gates

- The latest repository package-contract attempt reached Cargo metadata but failed because concurrent manifest changes made `Cargo.lock` stale. The lock has since been refreshed; rerun remains pending after active native work releases the ticket target.
- Draw FSM remount reached rustc and exposed only proc-macro entry integration errors. The duplicate import/public helper errors are fixed; its retry remains pending behind active native work.
- The 24 owned single-artifact leaves need their final selective compile/test loop against the settled shared source graph.
- The 39 TypeScript artifact packages need one final aggregate Nx/direct test record.
- One normal Nx target including prerequisites must pass after the earlier unrelated print/report generator `previewTarget` mismatches are rechecked.

## Relevant generated evidence

- `🗑️generated/nx-normalizer-profile-tracked.txt`
- `🗑️generated/nx-normalizer-profile-memoized.txt`
- `🗑️generated/nx-pdf-normalized-project.json`
- `🗑️generated/nx-pdf-normalized-inputs.txt`
- `🗑️generated/all-artifact-package-contract-current.txt`
- `🗑️generated/launch-generate.txt`
- `🗑️generated/launch-validation.txt`
- `🗑️generated/block-3d-typescript-test-async.txt`
- `🗑️generated/draw-fsm-package-remount-check.txt`
- `🗑️generated/nx-pdf-cache-precise-restore.txt`
- `🗑️generated/nx-pdf-cache-relevant-mutation.txt`
- `🗑️generated/nx-pdf-cache-exact-populate.txt`
- `🗑️generated/nx-pdf-cache-exact-restore.txt`
- `🗑️generated/nx-pdf-cache-exact-relevant.txt`
- `🗑️generated/nx-pdf-cache-exact-unrelated-jpg.txt`
- `🗑️generated/nx-pdf-cache-exact-*-run.json`
- `🗑️generated/nx-pdf-cache-exact-*-hashes.txt`
