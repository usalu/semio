# Taxonomy Enforcement Implementation

Date: 2026-09-12

## Result

The repository now has a shared, closed policy for anonymous implementation leaves and target-first package ownership. The policy emits two stable breaches through the taxonomy report/enforce pipeline:

- `taxonomy/kind-only-basename`: an authored implementation leaf is not exactly `<registered file-kind emoji><longest registered extension chain>`.
- `taxonomy/target-inside-package-boundary`: a `🎯️targets` subtree occurs below `📦️packages/<registered language>`.

The basename rule applies to every registered `source` role plus the authored CSS and HTML implementation kinds. It covers source files used for runtime, tests, benchmarks, adapters, and oracles because those files resolve through the same language source kinds. Arbitrary leading emoji, generic stems such as `component`, `index`, and `test`, and Cargo's configurable `src/lib.rs` and `src/main.rs` defaults do not bypass the rule.

Only a fixed contract derived from the candidate's actual path and live taxonomy scope can admit a non-kind-only implementation filename. The retained source exception is Cargo's exact package-root `build.rs` contract. It now requires a readable `Cargo.toml` with a valid `[package]` identity whose `build` field is absent or exactly `"build.rs"`; a naked directory, `build = false`, a custom build path, a workspace-only manifest, or malformed TOML cannot activate it. The path-only classifier therefore reports `build.rs` conservatively, while the filesystem census and normalizer supply manifest-derived evidence internally. The removed `src/lib.rs` and `src/main.rs` contracts are configurable Cargo paths rather than fixed external requirements.

Handpicked named identities remain available for non-implementation schemas, specifications, and assets. Normalization now uses the same implementation-kind predicate as the focused census, so it converts named implementation leaves to kind-only leaves while retaining named non-implementation identities.

## Policy and API

`🔣️taxonomy.json` owns `implementationLeafPolicy` with `roles: ["source"]`, `fileKindIds: ["css", "html"]`, and these exact ignored generated, cache, or tool-state patterns:

```text
**/.git
**/.pytest_cache
**/.venv
**/.🧬semio
**/__pycache__
**/coverage
**/dist
**/node_modules
**/storybook-static
**/target
**/📤️dist
**/🗑️generated
**/🔌️plugin-modules
**/📦️packages/🦀️rust/pkg
**/📦️packages/🟦️typescript/out
**/📦️packages/🔷️dotnet/obj
```

The focused walker still visits governed hidden source roots such as `.agents` and `.cursor`. It also visits persistent `🤖️generated` source owners; generated provenance does not relax physical source naming. The redundant global `**/target-*` wildcard was removed because every observed match lived below an already-excluded Cargo `target` cache. It applies the existing schema-backed opaque path exclusions for `compose/`, `temp/compose/`, and `♻️mit-bestand/🔎️recherche/` before filesystem access.

The discovery library exports:

- `taxonomyFileKindIsImplementation(fileKindId, taxonomy)`
- `implementationLeafBasenameFinding(path, taxonomy)`
- `targetInsidePackageBoundaryFinding(path, taxonomy)`
- `taxonomyImplementationFilesystemFindings(repoRoot, taxonomy, options)`

The focused filesystem census is asynchronous, no-follow, deterministically sorted, progress-reporting, and cancellable. It carries `Dirent` node kinds through the walk, retries interrupted reads, reports concurrently vanished directories through progress, and propagates other filesystem errors. Its default taxonomy loader is `loadCatalogTaxonomy()`, which validates the vocabulary without running workspace generator-authority diagnostics. The existing full taxonomy gate continues to use its stronger workspace validation.

Root commands are registered in `📋️project.json`, `package.json`, the authoritative `.vscode/🧩️launch.seed.jsonc`, and generated `.vscode/launch.json`:

```text
verify taxonomy implementation report
verify taxonomy implementation enforce
test kind-only-basename
```

The report command returns success while printing every finding. The enforce command uses the same census and fails when findings remain.

## Language-neutral contract

The new Draft-07 fixture describes implementation status, longest extension chains, exact external contracts, target/package topology, physical files, and expected stable findings without depending on TypeScript APIs. It includes Rust, TypeScript, JavaScript, Go, Python, .NET, CSS, HTML, hidden authored source, persistent generated source, a non-implementation grammar, an asset identity, Cargo `build.rs`, configurable Cargo entry defaults, target-first Rust and React packages, and the rejected package-first inverse. Cargo cases cover a valid default package, a naked `build.rs`, `build = false`, and a custom configured build path.

The independent oracle materializes the fixture, discovers it with `fast-glob`, resolves the longest extension chain independently, and compiles exact basename predicates with Ajv. Its result must equal both the fixture and the production census.

The contract also registers `.d.ts`, `.d.mts`, and `.d.cts` as TypeScript source extension chains. Canonical and named `.d.mts`/`.d.cts` cases prevent declaration semantics from falling through to `.mts`/`.cts`. Exact physical resolution rules exist for both added chains.

## Verification

- `bun ./📜️script.ts test kind-only-basename` from the TypeScript repo-library package: 6 passed, 0 failed, 42 assertions. The fixture contains 35 files and 21 expected findings; production and independent fast-glob/Ajv/TOML oracle output matched.
- Focused normalization regression under both Bun and TypeScript compilers: 1 passed, 0 failed, 49 assertions. It verifies preservation of named non-implementation identities and conversion of `🧩️component.rs` to `🦀️.rs` with the stable breach.
- `loadCatalogTaxonomy()` plus `validateTaxonomy(...)`: no schema problems after registering `.d.mts` and `.d.cts`.
- Focused actual-path probe: `🟨️.d.mts` resolves to `typescript-source` with expected basename `🟦️.d.mts`; canonical `🟦️.d.cts` produces no finding.
- Edited JSON registry, fixture, schema, package, and project documents parse successfully. `.vscode/launch.json` is JSONC and its three command registrations were inspected directly.
- `bun nx run workspace:verify-taxonomy-implementation-report --skip-nx-cache`: completed successfully in 19.2 seconds through the registered Nx target. The captured coordinator log is `🗑️generated/coordinator/implementation-report.log`.
- The authoritative `.vscode/🧩️launch.seed.jsonc` now owns all three focused commands. `bun nx run @semio-tech/plugin-registry:generate` completed with both targets green in 21.3 seconds and regenerated `.vscode/launch.json`; a regression requires exactly one matching name/command pair in both files.

The earlier isolated Nx package test attempt spent more than 90 seconds constructing the shared project graph without starting the target and was interrupted. The direct registered package router test above exercises the same target command and is green. A broad TypeScript lint run is not a useful gate for this lane because concurrent workspace relocation produces unrelated root and module-resolution diagnostics; no new discovery errors appeared in the filtered compiler output.

## Current live census

The post-audit focused report visited and classified 137,774 paths and recorded 555 remaining `taxonomy/kind-only-basename` findings and zero `taxonomy/target-inside-package-boundary` findings. It completed with no vanished directories. The increase from the prior 530-result report is exactly the 25 persistent named implementation leaves formerly concealed by `**/🤖️generated`. The captured output is `🗑️generated/enforcement/live-report-audit-repair.txt`.

By extension:

| Extension | Findings |
| --- | ---: |
| `.rs` | 186 |
| `.ts` | 139 |
| `.tsx` | 75 |
| `.js` | 66 |
| `.go` | 27 |
| `.css` | 25 |
| `.html` | 20 |
| `.sh` | 6 |
| `.mjs` | 3 |
| `.py` | 3 |
| `.cs` | 2 |
| `.sql`, `.ps1`, `.mts` | 1 each |

By top-level governed root:

| Root | Findings |
| --- | ---: |
| `🧰️framework` | 251 |
| `✏️s` | 106 |
| `.storybook` | 83 |
| `temp` | 53 |
| `♻️mit-bestand` | 50 |
| `.devcontainer` | 6 |
| `🌎️hub` | 3 |
| `.cursor` | 2 |
| `vitest.config.ts` | 1 |

These are actionable relocation findings for the active source migration waves, so the enforce target is intentionally red until the physical moves complete. The census produced no findings inside the declared transient `obj`, `out`, `pkg`, `target`, or ticket `🗑️generated` paths. Every live package-root `build.rs` was backed by an active default Cargo manifest, so the corrected census produced no `build.rs` findings.

## Generated source relocation input

The next generator migration lane must move each semantic identity into a domain directory, emit only the kind-only leaf there, and update the producer, output contract, imports, and fixtures together. None of these files was moved in this enforcement repair.

| Generator contract | Current named output |
| --- | --- |
| `assets-build` | `🧰️framework/🔨️modules/🖼️assets/🔣️icons/🤖️generated/🦀️icon_name.rs` |
| `assets-build` | `🧰️framework/🔨️modules/🖼️assets/🔣️icons/🤖️generated/🔤️shortcodes.ts` |
| `assets-build` | `🧰️framework/🔨️modules/🖼️assets/🔣️icons/🤖️generated/🐍️icons.py` |
| `assets-build` | `🧰️framework/🔨️modules/🖼️assets/🔣️icons/🤖️generated/🔷️Icons.cs` |
| `assets-build` | `🧰️framework/🔨️modules/🖼️assets/🔣️icons/🤖️generated/🟦️icons.ts` |
| `assets-build` | `🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🔣️icons/🤖️generated/🟦️metabolism_icons.ts` |
| `assets-build` | `🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🔣️icons/🤖️generated/🦀️metabolism_icon_name.rs` |
| `actor-typegen` | `🧰️framework/🔨️modules/🎭️actor/🤖️generated/🟦️actor.ts` |
| `ui-contract` | `🧰️framework/🔨️modules/🛂️manifest/🤖️generated/📜️ui-contract.ts` |
| `framework-manifest` | `🧰️framework/🔨️modules/🛂️manifest/🤖️generated/🪪️manifest.ts` |
| `ui-axes` | `🧰️framework/🔨️modules/🛂️manifest/🤖️generated/🎚️ui-axes.ts` |
| `graph-catalog` | `🧰️framework/🔨️modules/🕸️graph/🤖️generated/🦀️registry.rs` |
| `graph-catalog` | `🧰️framework/🔨️modules/🕸️graph/🤖️generated/🔠️types.ts` |
| `schema-entity-catalog` | `🧰️framework/🔨️modules/🧬️schema/🤖️generated/🟦️entity-kinds.ts` |
| `async-typegen` | `🧰️framework/🔨️modules/⏳️async/🤖️generated/🟦️async.ts` |
| `styling-tokens` | `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🤖️generated/🚦️palette-presence.css` |
| `styling-tokens` | `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🤖️generated/🔤️palette-fonts.css` |
| `styling-tokens` | `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🤖️generated/🌓️palette-theme.css` |
| `styling-tokens` | `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🤖️generated/🟦️tokens.generated.ts` |
| `plugin-registry` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🏗️framework.ts` |
| `plugin-registry` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🖥️hosts.rs` |
| `plugin-registry` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🎮️playgrounds.ts` |
| `plugin-registry` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🧩️plugins.ts` |
| `plugin-registry` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🗿️artifacts.rs` |
| `playground-session` producer, missing from declared generator roots | `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🤖️generated/🟦️session.ts` |

The independent generator-root audit found 47 implementation leaves inside 65 declared output roots. The final row above is an additional persistent generated implementation leaf outside those declared roots, so its producer/output declaration must be repaired as part of the same migration.

Source naming and package-body ownership are separate checks. An exact filename contract admits a physical name only; it does not prove that the file contains package glue. Existing package-purity analysis remains responsible for detecting domain implementation bodies under admitted package descendants, including the separately audited coordinator route and page bodies.

## Files

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🌳️kind-only-basename/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🌳️kind-only-basename/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🌳️kind-only-basename/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🤝️package-language-kind-handoff/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🤝️package-language-kind-handoff/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️workspace-contract/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔏️path-emoji-statutes/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📋️project.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/package.json`
- `📜️script.ts`
- `📋️project.json`
- `package.json`
- `.vscode/launch.json`
