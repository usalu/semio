# Canonical Boundary Enforcement Integration

## Implementation

The root layering task now runs the strict TypeScript import graph, Rust compile-input graph, and Cargo declaration graph through the library's existing Bun/Nx script router. Portable tests have separate uncached targets so a real repository violation cannot prevent checking the verifier itself. The canonical aggregate retains the real lint checks. New targets are registered in the library package and the existing launch groups.

The Cargo policy now rejects plugin-to-artifact declarations, including normal, development, optional, conditional, and renamed dependencies. Target role metadata and exact artifact directory segments are alternative selectors for the same prohibition. A package inside an artifact folder cannot bypass the rule by declaring itself a general module or library. Similar-looking folder names do not match. This extends the language-independent policy schema, fixtures, and existing implementation; it adds no runtime dependency or compatibility alias.

The shared Rust string decoder was corrected to preserve compiler path semantics for Unicode, byte escapes, null escapes, raw strings, and escaped newlines. The first independent compiler witness caught the previous Unicode decoder error. A subsequent independent audit found module-base, non-module attribute, manifest environment, and expression-completeness gaps; the Rust revision report records their final treatment. The initial green literal-only scan is therefore superseded, not used as final proof.

Root TS/JS inventory now includes filesystem symlinks so the graph verifier can reject them explicitly instead of silently dropping a source. A native Cargo environment object has an explicit repository-owned `NodeJS.ProcessEnv` annotation so removing inherited `RUST_TEST_NOCAPTURE` typechecks. The unused concrete puzzle self-test import was removed from the root router.

The Bun lock was refreshed using `bun install --lockfile-only --ignore-scripts` after CAD removed its required extension dependencies. No package was added for runtime use. No modifying Git command, worktree, or AGENTS.md change was used.

## Validation

- The plugin-to-artifact Cargo fixtures first failed before the rule was added. Cargo's independent metadata oracle also caught an invalid optional development-dependency fixture, which was corrected to a valid development case and a separate optional normal case.
- The role-and-folder Cargo witnesses failed before exact owner-segment evaluation was implemented. The final uncached Cargo suite passed 4 tests and 204 assertions. The final uncached TypeScript suite passed 8 tests and 514 assertions.
- The Cargo differential suite uses Ajv, `@iarna/toml`, Bun TOML, and actual `cargo metadata --offline` against generated isolated workspaces. The TS suite uses dependency-cruiser as an independent graph oracle. Rust witnesses use rustc dep-info and a separate path library.
- The library and coordinator typecheck passed after the native environment annotation. A later integration attempt encountered a missing store command-rejection test during concurrent workspace edits; the file was present on inspection, no unrelated source was changed, and the final uncached typecheck passed in 23.6 seconds.
- Scoped `git diff --check` passed during integration. Final whitespace validation is recorded in the completion report.
- An independent audit caught missing Cargo package scripts. Those scripts and the missing TypeScript lint package script were added. JSONC-based routing verification passed all six dependency targets: matching package-to-Nx and Nx-to-script routes, with exactly one launch entry for each target.
- A final shared-workspace projection replaced the manually inserted source-only launch row. The framework owner added a dedicated uncached source Nx target and validated its actual run and canonical synthesis. Root's final structural check passed all nine dependency/ownership launch routes, with no superseded argument-only source entry.
- The real physical-owner Cargo gate failed with 291 strict violations, zero metadata problems, 3,165 local declarations, and 280 packages. All 291 dependency rows are preserved in `🔍️2026-09-30-live-cargo-physical-violations.md`; generated logs are disposable.
- Final Rust portable validation passed 4 tests and 91 assertions with 19 actual rustc compilations. After the final neutral catalog fixture/helper edit, the live framework scan inventoried 2,183 Rust files and 4,033 authored references, finding zero strict edges and one explicitly unsupported proc-macro emitted path source. Its exit status was nonzero; this is not reported as a passing live gate.
- Final scoped whitespace verification passed over the 132 attributed paths. Root removed its 13 remaining owned output entries after evidence capture; the generated parent directory was empty and removed. Authored ticket inputs and all audit/report Markdown files were preserved.

## Scope Limits

Strict gates expose existing violations rather than silently grandfathering them. The first real TS run stopped at an unresolved authored presentation-package import. The first role-only artifact Cargo run found five forbidden declarations; the physical-owner rule broadens that inventory to mislabeled artifact packages as well. Existing concrete root self-test imports, broad workspace membership catalogs, plugin/artifact assembly dependencies, and production GIS-specific framework types are not all removed by this increment. A passed verifier suite does not establish whole-repository deletability.

## Root-Owned Files

- `📜️script.ts`
- `.vscode/launch.json`
- `bun.lock`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📋️project.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/package.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🧱️cargo-dependency-direction/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🧱️cargo-dependency-direction/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🦀️cargo/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🧱️rust-source-direction/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🧱️rust-source-direction/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️rust-source-direction/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🦀️source/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🦀️source/🏃️execution/🟦️.ts`

Shared-file attribution is limited to the edits described here. Other contributors' work was preserved in place.
