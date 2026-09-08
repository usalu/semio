# Final Boundary Audit

## Scope

Read-only final boundary audit on 2026-09-08. It inspected current Cargo metadata, artifact package declarations, package-local implementation sources, direct dependency directions, Rust root imports, TypeScript package declarations, Nx normalizer source, launch seed persistence, and exact native-law route ownership. It did not compile native code, run broad Nx work, mutate production files, Git state, tickets, or goals.

## Reconciled Inventory

`cargo metadata --offline --format-version 1 --no-deps` exited zero. The current workspace has 230 members and exactly 101 artifact-named Rust packages:

| Category | Count |
| --- | ---: |
| Stdio product artifacts | 36 |
| Other plugin product artifacts | 56 |
| Framework product artifacts | 7 |
| Product artifacts | 99 |
| Shared contracts | 2 |
| Artifact-named Rust packages | 101 |

The shared contracts are `semio-s-artifact-stdio-contract` and `semio-s-artifact-norm-contract`. All 99 product artifacts live in artifact taxonomy paths. Adding the two non-product Draw FSM support crates under that taxonomy gives the separate physical-Cargo-path count of 101. The two nested glTF JSON serializer/deserializer pseudo-artifacts were not counted as product artifacts.

## Boundary Results

All 101 artifact/contract Cargo packages have a declared project router and a `📜️script.ts`; each library target resolves to an existing taxonomy-root `🦀️.rs`. The earlier package-local source scan found zero Rust, TypeScript, JavaScript, or declaration implementation files below artifact `📦️packages` directories after excluding routers and generated outputs. This includes the Draw FSM and the moved Sequence browser/host/type sidecar.

The TypeScript declaration inventory is 40: 36 stdio packages, 3 Block packages, and `@semio-tech/sequence-sequence`. Each has a package manifest, Nx declaration, router, and declared build outputs. The Sequence package directory contains only its declaration/router files; its handwritten browser, host, and type implementation files are now outside that directory.

The direct artifact-root import scan found no `crate::artifacts`, `crate::registry`, or `semio_s_plugin_*` imports in any of the 101 artifact/contract roots. The full stdio production-source scan found only three explanatory/oracle documentation mentions after test and generator exclusions, not executable imports.

The static optional-dependency expansion found 114 optional normal dependencies and none selected from a package default feature. The only direct dependency whose Cargo package name starts `semio-s-plugin-` is `semio-s-plugin-draw-fsm`; its manifest is a descendant of the Draw artifact, so it is artifact-local FSM support rather than a parent-plugin backedge. No actual parent-plugin backedge was found.

## Nx And Launch Wiring

The Nx normalizer derives package-owner delivery inputs from `TAXONOMY.testDeliveryScopeDirectoryNames`, adds the owner tree to `default`, and uses `nativeSources`/`nativeTestSources` for Cargo targets. Artifact TypeScript targets instead use the exact owner TypeScript source plus their router inputs. This matches the current implementation in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs` lines 448-507 and 569-578.

Both `.vscode/🧩️launch.seed.jsonc` and generated `.vscode/launch.json` retain the artifact package contract launcher, parameterized artifact target launcher, and matching `artifactPackageProject`/`artifactPackageTarget` inputs. Existing runtime and cache evidence remains recorded in `📓️nx-execution.md`; this audit did not repeat it.

## Actionable Finding

**Open — Flow exact native laws still target the retired parent package.**

`✏️s/🔌️plugins/🌊️flow/📦️packages/🦀️rust/📜️script.ts` lines 34, 56, and 171 still select `semio-s-plugin-flow`, while every named law is defined below `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow`. The three groups must target `semio-s-artifact-flow-flow` before their list/exact checks can exercise the extracted implementation. This has been sent to the Nx owner; no source or package declaration fix is included in this audit.

The comparable Space Home routes have been repaired to `semio-s-artifact-space-home` with `component-app-assembly`. Its interactive-job group properly remains on the parent composition package. The Norm configuration and surface routes were also repaired during this audit.

## Verification Limits

Native `--list` and exact-law execution for the freshly retargeted routes remains delegated to the execution owners. No compiler success, Nx rerun, or cache result is claimed here beyond the pre-existing evidence cited above.

## Raw Evidence

- `🗑️generated/final-boundary-cargo-metadata-final.json`
- `🗑️generated/final-boundary-cargo-final-analysis.json`
- `🗑️generated/final-boundary-catalog-reconciliation.json`
- `🗑️generated/final-boundary-rust-project-declaration-analysis.json`
- `🗑️generated/final-boundary-typescript-project-declaration-analysis.json`
- `🗑️generated/final-boundary-artifact-root-import-analysis.json`
- `🗑️generated/final-boundary-stdio-production-source-backedges.txt`
- `🗑️generated/final-boundary-parent-plugin-dependency-analysis.json`
- `🗑️generated/final-boundary-optional-dependency-summary.json`
- `🗑️generated/final-boundary-nx-normalizer-source.txt`
- `🗑️generated/final-boundary-launch-artifact-entries.txt`
- `🗑️generated/final-boundary-flow-space-routes-final.txt`

## Coordinator Resolution Of Flow Routing

A subsequent direct source read confirms that all three identified Flow law groups now select `semio-s-artifact-flow-flow`. The ordinary parent plugin check, plugin tests and component description routes correctly remain on the composition package. This resolves the open static package-routing finding; native law execution remains part of the pending acceptance gates.
