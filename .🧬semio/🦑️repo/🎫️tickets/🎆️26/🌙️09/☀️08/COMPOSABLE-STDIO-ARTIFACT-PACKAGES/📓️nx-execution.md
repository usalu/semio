# Nx Artifact Package Execution

## Scope

This execution owns the reusable Nx/Bun package boundary, all stdio artifact package declarations, the 24 single-artifact plugin extractions, the Norm and multi-artifact Nx declarations, the three Block TypeScript declarations, launch integration, and package-contract/cache evidence.

## Implemented

- Added schema-first stdio package fixtures and a repository-wide language-neutral artifact package contract backed by Ajv and Cargo/Nx metadata.
- Registered 36 stdio Rust and 36 stdio TypeScript artifact projects. TypeScript declarations export only built `dist` JavaScript and declaration files.
- Added the neutral Rust and TypeScript package routers to the existing Cargo `📜️script.ts`. Expensive native work supports progress and process-tree cancellation; native time limits use `SEMIO_BUILD_BUDGET_MS` and default to unlimited.
- Extracted and declared 24 single-artifact plugin Rust packages with direct taxonomy-root `lib.path = "../../🦀️.rs"` ownership and private composition mounts.
- Added Nx declarations/routers for Norm contract plus 15 Norm artifacts and 17 multi-artifact packages.
- Added declaration-only TypeScript packages for Block 2D, 3D, and 5D with emitted declarations and real package-name resolution tests. The dynamic catalog now also includes the independently extracted Sequence browser artifact as the 40th TypeScript package.
- Standardized the seven framework artifact Nx names under `@semio-tech/framework-*` and added Space/Collection Nx declarations.
- Added authoritative artifact contract/selector launch configurations to `.vscode/🧩️launch.seed.jsonc`; generation produced matching entries in `.vscode/launch.json`.
- Replaced Lowpoly's optional CAD plugin dependency with the direct CAD artifact package.
- Remounted Draw FSM runtime and proc-macro crates directly from their taxonomy roots and removed package-local Rust implementation files. Updated the live taxonomy descendant contract to require declaration-only package roots.
- Memoized native dependency-root closures in the Nx plugin. Direct normalization improved from 34.5 s to 24.8 s for 446 projects after warm repository file discovery.
- Rewrote all 36 stdio TypeScript package routers to use the neutral artifact builder directly. Artifact TypeScript targets now hash their exact transitive source imports and exact command implementation rather than the repository-wide library tree.
- The neutral TypeScript builder copies JSON assets referenced by emitted declarations. Its package consumer typecheck disables `skipLibCheck`, isolates ambient type roots, and asserts the actual exported definition shape when present.
- Generalized Nx native target normalization so local artifact wrappers use `nativeSources`/`nativeTestSources` and their actual Cargo dependency owners.
- Native dependency inputs conservatively include declared optional normal/build dependencies (and dev dependencies only for test inputs). Default builds can therefore invalidate on an inactive optional dependency; feature-aware narrowing requires explicit, verified target-feature metadata. The PDF sibling-isolation proof remains precise because PDF's declared closure is narrow.
- Removed Writer's obsolete public artifact host re-export and duplicate package-level example mounts.
- Retargeted all 13 Flow child identity, edit, and retained-command native laws from the composition plugin to `semio-s-artifact-flow-flow`. A source-mount audit confirms each named law is compiled below the leaf crate's editor, retained, canonical, presence, transient, viewer, command, or testkit module.
- Retargeted the scoped Lowpoly `wasm-dev` optimization override from the composition package to `semio-s-artifact-lowpoly-lowpoly`; the render/app implementation is owned by that taxonomy-root leaf.
- Repaired the shared cache-contract, runner-self-test, browser-bundle, fresh-component, spatial-kernel geometry, and brepjs TypeScript sources after concurrent `import.source.*` rewrites left invalid source-context expressions or moved-path imports. Direct module-import smoke checks now pass and the repository has no remaining `import.source`/`source.directoryname` spellings outside generated dependencies.
- Closed the artifact-router argument boundary: build/check/test now reject caller-supplied workspace, package, manifest, configuration, and undeclared-target selectors; test levels are stripped before validation. Cargo build capture now emits the same periodic progress receipts as check/test while waiting for a shared build lock.

## Passing evidence

- `cargo metadata --offline --format-version 1` refreshed the workspace lock successfully after the Lowpoly dependency change.
- Block 3D TypeScript package test passes with progress reporting, JavaScript build, `.d.ts` emission, package-name `tsc` resolution, and runtime module resolution (`🗑️generated/block-3d-typescript-test-async.txt`). Earlier direct Block 2D/3D/5D package tests also pass.
- The stdio home-I/O source/fixture gates pass, including direct/shared/full feature closure and third-party Ajv/TOML oracles.
- Direct stdio PDF TypeScript build/test passes with runtime package-name resolution and emitted declaration consumption.
- Launch seed/output validation finds one contract configuration, one selector configuration, and both required inputs.
- Authoritative isolated `bun x nx show project @semio-tech/stdio-pdf-rs --json` passes with `NX_ISOLATE_PLUGINS=false`. The normalized project hashes PDF taxonomy sources through `production -> default`; `nativeSources` also enumerates the mounted PDF source graph.
- Repository-wide package contract passes on the final current registry and stdio parent mount after its package-root wrapper removal: `AJV=2/3 packages=99 rust=99 typescript=40 dag=clean`. It covers schema, declaration, source-owner, Nx ownership, resolved runtime Cargo closure, parent duplicate-mount/public-compatibility rejection, and retired composition-namespace checks. The legacy scan deliberately retains the independent `semio_s_plugin_stdio_test_oracle::artifacts::*` third-party oracle namespace (`🗑️generated/all-artifact-package-contract-final-current-2.txt`).
- PDF cache restoration and source isolation pass with exact inputs. Baseline task hash `2961489084152580833`; deleting only PDF `dist` produced a 100% local hit and restored JS (`28f47dcb40e97669e54c02a5cddc600d294e8298cf6b2cc6827110e4cbc90dd3`) plus `.d.ts` (`4a51329398088b7239d0e916c6c01177000713c6dbf5264a7f62b95a639b3e89`). A temporary PDF source marker caused a cache miss at hash `6201406382484965991`. After exact PDF restoration, a temporary JPG-only source marker plus PDF `dist` deletion returned a 100% local hit at the original hash and restored identical outputs. All probe markers were removed.
- The declaration-aware PDF test passes through Nx and emits three required outputs. A fresh build cache followed by deleting the entire `dist` restored JS, `.d.ts`, and `🧬️schema/📜️artifact-definition.json` from a 100% local cache hit with exact hashes `28f47d…`, `4a5132…`, and `5ca2eb…`.
- Launch regeneration passed through the normal `repo:generator-inputs` prerequisite and `@semio-tech/plugin-registry:generate`. Seed/output contain the canonical stdio contract project and no stale Cargo-name target.
- Final launch regeneration after the Sequence extraction passed in 1m45s with both tasks executed. The generated JSONC contains the repository contract, generic artifact selector, and `@semio-tech/sequence-tests:test` harness exactly once; it contains no stale `semio-s-artifact-stdio-contract:*` command.
- The Nx aggregate completed all 40 artifact TypeScript package tests with status 0. Each package built its distribution, emitted declarations, typechecked a package-name consumer, and resolved the runtime package. Sequence emitted and consumed its handwritten declaration sidecar.
- Sequence normalized inputs explicitly include its root TypeScript facade, browser JavaScript, host JavaScript dependency, and adjacent handwritten browser `.d.ts` sidecar.
- Sequence sidecar hashing and restoration pass. Baseline task hash `2708385174624081084`; a temporary adjacent browser `.d.ts` marker forced a cache miss at `10778854061449214357`. The source was restored to SHA-256 `81985d256abea2c8b944f709a050b4dbd3cbdc4fd22772732f7ce928e4acb400`. After deleting only Sequence `dist`, Nx produced a 100% local hit at the baseline hash and restored the sidecar (`81985d…`), entry declaration (`450e1a…`), and JavaScript (`a6e877…`).
- The schema-first native-preparation fixture, third-party `jsonschema` oracle, Cargo metadata oracle, artifact-router selector regression, and queued-build progress regression pass. The unsafe-router test was recorded red before the implementation and green afterward.
- The parent duplicate-mount contract was recorded red on Mathematical's package wrapper. All 22 remaining composition-level example mounts, Mathematical's duplicate CAS/polynomial mounts, and Imperative's public artifact extension re-export were removed. The repository-wide package-wrapper scan now reports zero direct taxonomy implementation mounts and zero root-level artifact public re-exports. Canonical example access remains on the artifact leaves; Raster now owns its previously parent-only demo-session mount and test.
- Current normalized Rust PDF inputs contain the PDF taxonomy root and its real Cargo dependency owners and contain no JPG taxonomy source or `stdio-jpg` owner (`🗑️generated/nx-pdf-rust-current-input-isolation.json`). This is the Rust input boundary; the native task cache mutation/restoration receipt remains pending the shared Cargo queue.
- A normal `@semio-tech/forms-forms-rs:check` invocation completed its four ordinary prerequisites, including a successful `framework-graph:generate`. The earlier print-latex/report-actor preview-target routing error did not recur; the leaf Cargo check is waiting on the shared build directory with periodic progress output.

## Current failures and pending gates

- Draw FSM remount reached rustc and exposed only proc-macro entry integration errors. The duplicate import/public helper errors are fixed; its retry remains pending behind active native work.
- The 24 owned single-artifact leaves need their final selective compile/test loop against the settled shared source graph.
- The 13 Flow exact-law runtime receipt remains queued behind active native Cargo writers; package identity and module mounting are already verified statically.
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
- `🗑️generated/nx-pdf-declaration-assets-test.txt`
- `🗑️generated/nx-pdf-assets-cache-populate.txt`
- `🗑️generated/nx-pdf-assets-cache-restore.txt`
- `🗑️generated/launch-generate-final.txt`
- `🗑️generated/launch-validation-final.txt`
- `🗑️generated/artifact-typescript-40-nx-summary.json`
- `🗑️generated/sequence-typescript-normalized-inputs.txt`
- `🗑️generated/sequence-sidecar-cache-baseline.txt`
- `🗑️generated/sequence-sidecar-cache-mutation.txt`
- `🗑️generated/sequence-sidecar-cache-mutation-run.json`
- `🗑️generated/sequence-sidecar-cache-restore.txt`
- `🗑️generated/sequence-sidecar-cache-restore-run.json`
- `🗑️generated/sequence-sidecar-cache-proof.txt`
- `🗑️generated/launch-generate-sequence-final-4.txt`
- `🗑️generated/launch-validation-sequence-final.txt`
- `🗑️generated/all-artifact-package-contract-final-6.txt`
- `🗑️generated/all-artifact-package-contract-final-current-2.txt`
- `🗑️generated/artifact-router-arguments-red.txt`
- `🗑️generated/artifact-build-progress-red.txt`
- `🗑️generated/artifact-router-progress-green.txt`
- `🗑️generated/artifact-router-existing-contract.txt`
- `🗑️generated/parent-artifact-mount-contract-red.txt`
- `🗑️generated/all-parent-artifact-mounts-after-owned.txt`
- `🗑️generated/all-parent-artifact-compatibility-after.txt`
- `🗑️generated/nx-pdf-rust-current-input-isolation.json`
- `🗑️generated/forms-normal-prerequisite-nx-check-2.txt`
