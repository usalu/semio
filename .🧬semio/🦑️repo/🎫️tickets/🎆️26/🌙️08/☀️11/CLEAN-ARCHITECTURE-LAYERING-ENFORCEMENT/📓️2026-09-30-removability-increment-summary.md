# Removability Canonicalization Increment

This increment implements concrete ownership refactors and strict boundary enforcement. It does not establish whole-repository deletability: the live gates expose remaining coupling and unproven generated dependencies. No compatibility facade, baseline exemption, runtime library dependency, Git mutation, worktree, or AGENTS.md edit was introduced.

## Implemented Boundaries

- CAD's TypeScript package exposes its general core. Artifact runtime/schema/IO and the OpenCascade oracle are owned by specific consumers. The spatial module no longer imports CAD or stdio plugin implementations. Required CAD extension dependencies were removed and the Bun lock was refreshed.
- Spatial commands and model assets have independent removable contributions. Removing a contribution restores the preceding command, invalidates derived catalogs/caches, and supports idempotent removal. Building, energy, and structure command identities live in their extensions; general dispatch no longer recognizes domain constructors by suffix.
- Shared lease, paint-protection, and slider-label fixtures have framework owners. Specific consumers reference those shared fixtures. GIS/Hub integration tests moved to Hub; general inference tests use neutral transport, cancellation, and policy vectors. Required slider labels now agree across the schema, first-party decoder, and serde oracle; the silent missing-label default was removed.
- TypeScript/JavaScript enforcement inventories present source owners independently of the resolver and accepts removed specific directories while rejecting malformed present owners. It covers aliases, subpaths, exports, type imports, dynamic imports, tests, scripts, linked inputs, and unresolved authored package imports. Old CAD/procedural warning exceptions were removed.
- Cargo enforcement checks every authored local declaration against independent Cargo metadata and rejects plugin-to-artifact dependencies by role or exact artifact-folder ownership. Mislabeling an artifact as a general module/library cannot hide the target boundary. Optional and platform-specific dependencies remain checked.
- Rust enforcement uses compiler string semantics, module mount contexts, include ownership, and manifest environment provenance. Non-module metadata paths are ignored; generated OUT_DIR inputs are classified explicitly. Conditional inline mounts and unproven emitted paths fail closed. Exact native test routes verify actual selected laws and provide progress, deadlines, and cancellation.

## Executed Verification

| Check | Result |
| --- | --- |
| Complete CAD Nx assembly | Passed: 13 files, 339 tests |
| Four CAD extension Nx test targets | Passed |
| CAD bundle with specific artifact/test owners hidden | Passed with independent esbuild resolver |
| TypeScript verifier and removal simulations | Passed: 8 tests, 514 assertions |
| Cargo metadata/TOML differential verifier | Passed: 4 tests, 204 assertions |
| Rust verifier and compiler witnesses | Passed: 4 tests, 91 assertions, 19 rustc compilations |
| Library and coordinator typecheck | Passed: final uncached run, 23.6 seconds |
| Framework fixture/label/catalog ownership | Passed: 5 portable tests, 29 assertions, 28 exact native laws across five owner packages |
| Hub relocated integration contract | Passed: six exactly discovered/executed native laws, zero failed or ignored |
| Dependency package/Nx/launch routing | Passed: six dependency routes and three ownership routes, unique launch entries |
| Live TypeScript graph | Failed: unresolved authored presentation-package import; 8,981 sources inventoried in that run |
| Live Cargo graph | Failed: 291 plugin-to-artifact declarations across 43 source owners; 280 packages, 3,165 local declarations, zero metadata problems |
| Final combined-source Rust graph | Failed closed: 2,183 files, 4,033 authored references, zero strict edges, one unsupported proc-macro quote source |

The fixture, graph, contribution, and geometry tests compare first-party behavior with independent Ajv, dependency-cruiser, Cargo, TOML, rustc, path, lodash, serde_json, d3-hierarchy, esbuild, and OpenCascade oracles. Runtime contribution lifecycle console evidence is preserved in the spatial report; temporary DEBUG logging was removed.

The framework exact native route finished with exit status zero in 3 minutes 44 seconds; the Hub exact route also exited zero after 6 minutes 55 seconds. Their executable fingerprints and per-owner receipts are preserved in the framework report. The first native run exposed two catalog tests treating the lease fixture's intentionally incomplete descriptor stub as a full package descriptor; their consumers now use one schema-owned neutral descriptor witness with first-party/serde parity, and both catalog laws pass. The lease corpus bytes were preserved.

The final launcher check caught a source-only row disappearing during canonical regeneration. Source verification now has its own uncached `@semio-tech/framework-rs:test-fixture-ownership-source` target, calling the existing script's source command. It ran successfully with 5 tests and 29 assertions; read-only canonical launch synthesis retained its exact row once. The superseded argument-only launch row was removed.

Final scoped `git diff --check` passed over all attributed paths. The combined attribution contains 119 source/config/fixture paths and 13 ticket reports; shared-file claims are limited to the described edits. All owned native/exact caches, compiler proof directories, and logs were removed after recording evidence. The ticket's generated directory was then empty and removed; authored inputs and all reports remain.

## Remaining Work Exposed by the Gates

The [Cargo inventory](./🔍️2026-09-30-live-cargo-physical-violations.md) preserves every remaining plugin-to-artifact edge. Rust plugin app fleets and editor/viewer exports still bind concrete artifacts; those assemblies need outward ownership and contribution contracts. Folder/role agreement also needs broader validation on source owners.

Framework production MCP/directory APIs still expose GIS-specific preview/undo types. Their schemas, validators, rendering, and transport contracts need coordinated extraction into domain extensions. Some general task scripts and the root router still statically reference specific owners. Explicit broad workspace membership catalogs also prevent a claim that deleting s/Hub already preserves every tool and build.

The TypeScript live run stopped at `♻️mit-bestand/🎤️präsentation/📅️33.projektetage/🎞️slide/🌷️Einführung/🪻️Einleitung/👋️Einleitung.ts` importing unresolved `@semio-tech/mit-bestand-praesentation-projektetage-spec`; it does not prove there are no additional violations. The Rust gate cannot yet prove consumer provenance for symbolic `include_str!(#...)` emitted by `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️.rs:595`. No blanket proc-macro exemption was added.

## Detailed Evidence and Attribution

- [Spatial/CAD ownership and exact files](./📓️2026-09-30-spatial-cad-ownership.md)
- [Framework fixture/contracts ownership and exact files](./2026-09-30-framework-ownership-changes.md)
- [TypeScript enforcement and exact files](./📓️2026-09-30-ts-removability-enforcement.md)
- [Rust scanner proofs and exact files](./🔍️2026-09-30-rust-source-direction-revision.md)
- [Root integration and exact files](./📓️2026-09-30-root-enforcement.md)
- [Independent final boundary review](./🔍️2026-09-30-final-deletability-public-boundary-audit.md)

This work used the four available concurrent slots for main coordination, GPT 6.1 Sol High execution, and GPT 6.1 Sol Low read-only audits. The requested Light effort is not exposed by the agent tool. The main model/effort could not be changed from this conversation; the plan records that limitation rather than claiming a model switch.
