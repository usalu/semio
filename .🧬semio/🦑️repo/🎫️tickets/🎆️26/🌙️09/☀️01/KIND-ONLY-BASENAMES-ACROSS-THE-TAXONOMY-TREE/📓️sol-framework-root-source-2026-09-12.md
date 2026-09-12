# Framework Root Source Ownership

## Result

The two framework package roots now contain package wiring only. The Rust schema metadata and type generation implementation lives in an anonymous Rust leaf under the schema projection owner. The five TypeScript operations live in anonymous leaves under their async or I/O concerns. Existing package names, entry paths, public exports, Rust feature gating, native behavior, and WGPU browser inputs remain active.

The permanent topology gate validates structure, live consumers, package-source classification, native mounts, browser compilation, and editor command registration. It deliberately does not freeze authored implementation hashes. Hashes below are retained only as this move's provenance evidence.

## Source identity and move map

The live roots were captured before editing:

| Source | Pre-move SHA-256 | Bytes | Lines |
| --- | --- | ---: | ---: |
| `🧰️framework/📦️packages/🦀️rust/🦀️.rs` | `314f942a6d50f606fca3c143f47264d67186ab57a33b3f9075baa5703067e2ab` | 117155 | 2176 |
| `🧰️framework/📦️packages/🟦️typescript/🟦️.ts` | `f3207e60fd3edd37549a409996ea329437a665248dcd5b93d0ead2b9bf6fd462` | 17451 | 417 |

The authored regions were relocated without rewriting their behavior:

| Former owner | Canonical anonymous owner | Moved-owner SHA-256 | Consumer anchor |
| --- | --- | --- | --- |
| Rust package root | `🧰️framework/🔨️modules/🧬️schema/📽️projection/🦀️.rs` | `a313210e724537a66c4498dbccb3305ac4b8813b20c11fe1c8dcd67bf46c01a6` | `pub struct SchemaMetadata` |
| TypeScript package root | `🧰️framework/🔨️modules/⏳️async/🎟️lease-pool/🟦️.ts` | `fd554e34c5efa890b1b638023709366bc2f5967c32c03e5766ba41ba5f0123fa` | `export function createLeasePool` |
| TypeScript package root | `🧰️framework/🔨️modules/⏳️async/🔁️jittered-backoff/🟦️.ts` | `401521507cbf52fca7ab53acb6454ca08faeaf9664fd4a0eb0770bcd0b834369` | `export async function retryWithJitteredBackoff` |
| TypeScript package root | `🧰️framework/🔨️modules/⏳️async/🥇️latest-wins/🟦️.ts` | `84a41b41933bd741e8e00c0c50a8b6a5b13f008d64057f7d443894d88841a273` | `export function latestWins` |
| TypeScript package root | `🧰️framework/🔨️modules/⏳️async/🔔️event-wait/🟦️.ts` | `5c959c6f7de7d4c47b8c424764ec520723543fa8305a7ec04eb3692243af232d` | `export function waitForEvent` |
| TypeScript package root | `🧰️framework/🔨️modules/🚪️io/🌐️fetch-timeout/🟦️.ts` | `f9936021eec01d4400d997d7c69ac76fbd2afe739024c4413035d2e5a926b66c` | `export async function fetchWithTimeout` |

The resulting Rust entry is 201 lines and the TypeScript entry is 30 lines. The shared package classifier reports `declaration` for both entries. The permanent gate checks this classification and maximum entry sizes without retaining the hashes as product invariants.

## Package and consumer closure

- `🧰️framework/📦️packages/🦀️rust/🦀️.rs` retains the `#[cfg(feature = "typegen")]` module mount through `#[path = "../../🔨️modules/🧬️schema/📽️projection/🦀️.rs"] pub mod schema_metadata;`. Cargo retains package `semio-framework`, library path `🦀️.rs`, and its `typegen` feature.
- `🧰️framework/📦️packages/🟦️typescript/🟦️.ts` explicitly reexports values and types from all five owners, preserving the `@semio-tech/framework` surface.
- `🧰️framework/🧪️tests/🧪️docklayoutstore/🟦️.ts` owns the source-mode Vitest registration and dynamically imports `@semio-tech/framework`.
- `🧰️framework/📦️packages/🟦️typescript/vitest.config.ts` includes that exact test owner for source-mode coverage. Production package glue contains no test activation or implementation logic.
- The WGPU `wgpu-frame-worker` authority now lists all five TypeScript owners in both `browserProfile.sourceModulePaths` and `inputPatterns`, in UTF-8 byte order. `@semio-tech/framework-os:generate-wgpu` regenerated the exact frame-worker projection, and the subsequent check reported zero changed artifacts.
- `.vscode/🧩️launch.seed.jsonc` and the derived `.vscode/launch.json` each contain one `🧹clean🧩️taxonomy🧪️framework-root-source-topology` command. The normal plugin-registry generator produced the derived launch file.

## Schema-first portable gate

Added:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🧰️framework-root-source-topology/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🧰️framework-root-source-topology/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧰️framework-root-source-topology/🟦️.ts`

The fixture declares six source moves, both package anchors, exact consumers, and nine contextual directory resolutions. AJV validates the language-neutral fixture. `@iarna/toml` independently parses the live Cargo package. Bun's browser compiler parses every TypeScript owner. The shared discovery classifier verifies that both package entries are declarations.

Before the source moves and context registrations, this gate failed four tests because owners were absent, package roots retained implementation anchors and exceeded the size limit, and new owner contexts did not resolve. Before launch derivation, the launch-authority assertion also failed. These failures were resolved by the final source, schema, and command closure.

The gate is registered through the existing repo-library command surfaces:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📋️project.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/package.json`

## Taxonomy contexts

`🔣️taxonomy.json` adds these narrow registry-backed memberships:

| Kind | Parent kind | Member |
| --- | --- | --- |
| `framework-schema-projection` | `schema` | `📽️projection` |
| `framework-async-lease-pool` | `members-of-modules` | `🎟️lease-pool` |
| `framework-async-jittered-backoff` | `members-of-modules` | `🔁️jittered-backoff` |
| `framework-async-latest-wins` | `members-of-modules` | `🥇️latest-wins` |
| `framework-async-event-wait` | `members-of-modules` | `🔔️event-wait` |
| `framework-io-fetch-timeout` | `io` | `🌐️fetch-timeout` |
| `framework-root-source-topology` | `schema`, `fixtures`, `tests` | `🧰️framework-root-source-topology` |

Strict `loadTaxonomy()` plus `validateTaxonomy()` returned `[]`. A final scoped `inventoryTaxonomy` pass covered both package roots, all six semantic owners, and the three portable case directories. Every scope reported zero violations; both package roots resolved as declaration roles.

## Verification

| Check | Outcome |
| --- | --- |
| Direct portable topology test | 5 passed, 67 expectations |
| Registered `@semio-tech/repo-lib:test-framework-root-source-topology` | 5 passed, 67 expectations; Nx succeeded |
| Focused framework TypeScript behavior filter for `LeasePool`, retry, latest-wins, timeout, and event wait | 14 passed, 117 skipped; native strip-only oracle completed |
| `@semio-tech/framework-rs:check` | `exports_typescript_bindings` passed; 236 filtered; TypeScript mirror fresh |
| `@semio-tech/repo-lib:test-package-body-policy` | 69 passed, 238 expectations |
| `@semio-tech/framework-os:generate-wgpu` then `check-wgpu` | 6 exact artifacts regenerated with 1 changed; subsequent check found 0 changed |
| `@semio-tech/plugin-registry:generate` | 2 Nx tasks succeeded and launch derivation completed |
| Strict taxonomy load and validation | `[]` |
| Corrected `@semio-tech/repo-lib:test-os-source-topology` | 5 passed |

The Rust check emitted the current deprecation warning for trace's `AtomicU64::fetch_update`; compilation and the typegen test succeeded. An exploratory `@semio-tech/framework-async:test` run executed 20 passing tests but its suite then failed because the current async package entry imports the absent generated binding `./🤖️generated/🟦️async.js`. This lane preserves the async package API and does not own that generated binding, so this is recorded only as an observed current limitation without baseline attribution.

## OS fixture invariant correction

The earlier OS source-topology fixture permanently compared 30 live implementation bodies to retained SHA-256 fields. Those fields and comparisons were removed from its schema, fixture, and test. Its gate continues to validate topology, anonymous leaves, source roles, package/native mounts, consumers, and native checks. The move-time hashes remain provenance in `📓️sol-os-source-2026-09-12.md`, while legitimate authored edits no longer invalidate a taxonomy test.

## Files changed for this lane

Source and test activation:

- `🧰️framework/📦️packages/🦀️rust/🦀️.rs`
- `🧰️framework/📦️packages/🟦️typescript/🟦️.ts`
- `🧰️framework/📦️packages/🟦️typescript/vitest.config.ts`
- `🧰️framework/🔨️modules/🧬️schema/📽️projection/🦀️.rs`
- `🧰️framework/🔨️modules/⏳️async/🎟️lease-pool/🟦️.ts`
- `🧰️framework/🔨️modules/⏳️async/🔁️jittered-backoff/🟦️.ts`
- `🧰️framework/🔨️modules/⏳️async/🥇️latest-wins/🟦️.ts`
- `🧰️framework/🔨️modules/⏳️async/🔔️event-wait/🟦️.ts`
- `🧰️framework/🔨️modules/🚪️io/🌐️fetch-timeout/🟦️.ts`
- `🧰️framework/🧪️tests/🧪️docklayoutstore/🟦️.ts`

Authority and portable support:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`
- the three framework-root portable case files listed above
- the repo-library `package.json`, `📋️project.json`, and `📜️script.ts` listed above
- `.vscode/🧩️launch.seed.jsonc`
- `.vscode/launch.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎞️frame-worker/🤖️generated/🟨️.js`

Requested OS fixture correction:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🖥️os-source-topology/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🖥️os-source-topology/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🖥️os-source-topology/🟦️.ts`
- `📓️sol-os-source-2026-09-12.md`
