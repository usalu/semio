# Repository Source Ownership Extraction

## Outcome

The bounded repository source set now owns its implementation bytes at domain directories with anonymous language-kind leaves. Native package and framework-required entry files remain only where Cargo, TypeScript packages, or Next require them. The portable contract records exactly 22 identities and rejects missing owners, stale non-framework entries, duplicate identities, non-anonymous owner basenames, oversized glue, unresolved glue targets, lost Next method exports, invalid TypeScript syntax, incorrect native manifest paths, changed package-root anchors, and missing launch registration.

No Go source or process/cancellation helper was changed. No compatibility source was added. Package names, public TypeScript exports, Rust crate names, the .NET assembly identity, HTTP paths and methods, and the VS Code extension entry remain unchanged.

## Exact source moves

| Former source | Semantic owner | Native boundary after the move |
| --- | --- | --- |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🦀️rust/🦀️.rs` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/🦀️.rs` | Cargo `[lib] path = "../../🦀️.rs"` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🦀️rust/📦️main.rs` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/🚪️entrypoint/🦀️.rs` | Cargo `[[bin]] path = "../../🚪️entrypoint/🦀️.rs"` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/⚙️build/🟦️.ts` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/🏗️builder/🟦️.ts` | Package `📜️script.ts` imports the builder |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/🟦️.ts` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/🟦️.ts` | One-line package re-export |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts` | One-line package re-export |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧩️runtime/🟨️.d.mts` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧩️runtime/🟦️.d.mts` | Correct TypeScript declaration kind; no active consumer used the former name |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🟦️typescript/🖥️server-implementations.ts` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/🔌️ports/🟦️.ts` | Active server-library import points directly to the owner |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🟦️typescript/app/api/v1/auth/route.ts` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/🔐️authentication/🟦️.ts` | Next wrapper re-exports `GET` and `POST` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🟦️typescript/app/api/v1/breach/route.ts` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/🚨️breaches/🟦️.ts` | Next wrapper re-exports `GET` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🟦️typescript/app/api/v1/diff/route.ts` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/🧾️diff/🟦️.ts` | Next wrapper re-exports `POST` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🟦️typescript/app/api/v1/event/route.ts` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📡️event/🟦️.ts` | Next wrapper re-exports `GET` and `POST` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🟦️typescript/app/api/v1/repo/route.ts` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📚️repository/🟦️.ts` | Next wrapper re-exports `POST` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🟦️typescript/app/api/v1/scope/route.ts` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/🎯️scope/🟦️.ts` | Next wrapper re-exports `GET` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🟦️typescript/app/api/v1/ticket/[id]/route.ts` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/🎫️ticket/🔎️detail/🟦️.ts` | Next wrapper re-exports `GET` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🟦️typescript/app/api/v1/ticket/route.ts` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/🎫️ticket/🟦️.ts` | Next wrapper re-exports `GET` and `POST` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🟦️typescript/app/api/v1/warning/route.ts` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/⚠️warnings/🟦️.ts` | Next wrapper re-exports `GET` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🟦️typescript/app/api/webhooks/github/route.ts` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/🪝️webhook/🐙️github/🟦️.ts` | Next wrapper re-exports `POST` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🟦️typescript/app/layout.tsx` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/🖼️shell/🟦️.tsx` | Next wrapper re-exports `default` and `metadata`; owner remains a server component |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🟦️typescript/app/page.tsx` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📊️dashboard/🟦️.tsx` | Next wrapper re-exports `default`; owner remains a server component |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🔷️dotnet/🔷️.cs` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🔷️.cs` | `.csproj` explicitly compiles and links the owner; default compile items remain disabled |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟦️.ts` | One-line package re-export |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🦀️rust/📦️lib.rs` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🦀️.rs` | Cargo `[lib] path = "../../🦀️.rs"` |

## Referent and consumer closure

- The CLI owner now resolves command modules from its semantic owner and retains the existing unit-test modules. Cargo points both native targets at the new anonymous leaves.
- `getLibRoot()` still resolves `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages`; moving the body did not reinterpret that public path.
- The coordinator PostgreSQL adapter now anchors `createRequire` to `🔌️ports/../📦️packages/🟦️typescript/package.json`, preserving dependency resolution from the declaring package.
- The server library consumer `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/📚️library/📦️packages/🟦️typescript/🟦️.ts` imports `../../../🎛️coordinator/🔌️ports/🟦️.ts` directly.
- VS Code build entry resolution still starts at the TypeScript package root; its script imports `../../🏗️builder/🟦️.ts`. Package and host-build tests use the same builder owner. The extension package still emits `out/extension.js`.
- The library, VS Code, coordinator and test Nx project metadata now declares each semantic module root as `sourceRoot`, so source inputs include the extracted owners.
- The TypeScript package roots for library, VS Code and test contain one export declaration each. The live policy classifier reports all three as `declaration`.
- The coordinator wrappers contain only export declarations. Their exact owner path and method/default/metadata export sets are part of the portable fixture. The layout and page owners have no client directive, preserving their prior Next server-component execution mode.
- No live source refers to the former named declaration, CLI/bin names, test Rust lib name, VS Code build path, or coordinator server implementation path. The new regression fixture records those former paths intentionally. The frozen `🧼️remaining-package-purity-authority` catalog still records historical findings and was not rewritten. The VS Code package fixture retains `⚙️build/🟦️.ts` as an intentionally rejected build-only payload in its isolated test tree.

## Contracts and supporting files

Added:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🦑️repo-source-ownership/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🦑️repo-source-ownership/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🦑️repo-source-ownership/🟦️.ts`

Updated native/package consumers and task metadata:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/📜️script.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/📋️project.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/tsconfig.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/🧪️tests/📦️package/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/🧪️tests/🧩️host-build/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/package.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📋️project.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🟦️typescript/tsconfig.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🟦️typescript/📋️project.json`
- the 12 coordinator Next entry files listed in the move table
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/📚️library/📦️packages/🟦️typescript/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/📋️project.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🔷️dotnet/🧪️Semio.Repo.Test.csproj`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`
- `.vscode/🧩️launch.seed.jsonc`
- `.vscode/launch.json`

The library script now routes `test repo-source-ownership`; the package and Nx project expose `test-repo-source-ownership`; and seed plus derived launch authorities contain exactly one `🧹clean🦑️repo🧪️source-ownership` command. The normal plugin-registry generator regenerated the derived launch file after the seed change. Narrow taxonomy members were added for `🚪️entrypoint`, `⚠️warnings`, `🔐️authentication`, `🎫️ticket`, `🎯️scope`, `📊️dashboard`, `🖼️shell`, `🧾️diff`, `🪝️webhook`, `🦑️repo-source-ownership`, `🚨️breaches`, `🔎️detail`, and `🐙️github`.

## Verification

| Check | Result |
| --- | --- |
| Registered Nx `@semio-tech/repo-lib:test-repo-source-ownership --skip-nx-cache` | Passed: 6 tests, 212 assertions, 0 failures; Nx target succeeded in 1.4 s after graph startup |
| Direct portable contract | Passed: the same 6 tests and 212 assertions in 601 ms |
| Strict `loadTaxonomy()` plus `validateTaxonomy()` | Passed: `[]`, zero problems |
| Registered package-body classifier route | Passed: 49 tests, 154 assertions, 0 failures in 7.89 s |
| Plugin-registry/launch generator | Passed: 59 plugin crates, 61 playgrounds and 49 framework packages processed; portable check confirms the seed and derived command exactly once |
| Repository CLI `cargo test --lib` | Passed: 24 tests, 0 failures |
| Repository test-host `cargo test` | Passed: library and doc-test targets completed, 0 failures |
| Repository test-host `.NET 8` build | Passed: 0 warnings, 0 errors; `Semio.Repo.Test.dll` emitted from the project that includes `../../🔷️.cs` |
| VS Code Vite production bundle through relocated builder | Passed: 53 modules; `extension.js` 85.18 kB / 19.22 kB gzip; 357 ms |
| Bun native bundle of 12 Next entries | Passed: 12 entrypoints, 12 outputs, no diagnostics |
| Bun native bundle of active server-library coordinator consumer | Passed: one output, no diagnostics |
| Bun native bundles of library and test package entries | Passed: one output each, no diagnostics |
| Rust test-host `cargo fmt --check` | Passed |

## Observed limits and provenance

- Full VS Code `tsc --noEmit` is currently red. Observed diagnostics include owner lines 874/876 (a declaration without an implementation and an undefined `documents` binding), strict optional-output errors, and numerous transitively included framework diagnostics. This run establishes the current limit but does not attribute every diagnostic to a preimage. The actual extension Vite bundle is green and resolves the relocated package entry and builder.
- Full coordinator `tsc --noEmit` remains red because the existing `@/lib` mapping targets `../lib/index.ts`, which does not exist in either `HEAD` or the current tree. The same unresolved import appears in the `HEAD` route sources. Further broad imported-framework diagnostics also remain. The extracted route ownership, exact Next exports, relative schema imports, and syntax were validated independently. The installed Bun bundler resolved all relative source edges but deliberately externalized bare package and alias imports, so it does not prove `@/lib` runtime resolution.
- CLI `cargo fmt --check` is currently red on leading blank lines in command unit-test files and import ordering in the CLI unit-test file. This run establishes the current formatter limit without assigning those bytes to a prior revision. The moved CLI library and binary compile and all 24 CLI tests pass. The test-host Rust formatter check is green.
- An initial direct `bun test` invocation omitted the required `./` path prefix, so Bun treated the path as a filter and found no test. The corrected invocation and the registered uncached Nx route both pass.

All disposable compiler, bundle, Nx and probe output was confined to `🗑️generated/sol-repo-source-extraction`; it can be removed without affecting any retained source or evidence in this report.
