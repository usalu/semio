# H-REFERENCES-PACKAGES Report

## Existing Reusable Mechanism

`🔍️discovery/🟦️component.ts` already owns one repository walk, package discovery, semantic sources, Rust `#[path]` resolution, Rust relative-use resolution, TypeScript package exports and path maps, Go module roots, Python roots, `.csproj` project references and semantic consumer edges. It deliberately skips `compose`, `.git`, dependency/build outputs and generated caches. This is the correct source to refactor into adapters; a second unrelated census would duplicate authority.

The current reference model is read-only and narrow. `SemanticConsumerEdge.mechanism` distinguishes `static-import`, `path-attribute`, `project-reference` and `runtime-registration`, but it does not record byte/AST locations, old/new literal values, preimage hashes, generated ownership, or rewrite capability. Resolver defaults still hardcode `./🟦️glue.ts`, `index<extension>`, `__init__.py` and taxonomy leaf filenames.

## Package Census

The existing non-`compose` discovery scan reported:

| Measure | Count |
| --- | ---: |
| Owners | 141 |
| Marked packages | 116 |
| Derived clean owners | 138 |
| Derived mixed owners | 3 |
| Residual implementation directories | 1 |
| Unmarked manifests | 73 |
| Packaging violations | 126 |
| Ambiguous language shapes | 2 |
| Unknown language directories | 5 |
| Enforced-area manifests without markers | 47 |

The three mixed owners are `✏️s/🔌️plugins/🎞️animate` (one residual implementation directory), `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp` and `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run` (owner-root `📦️bin.rs`). Representative packaging implementation is concentrated in framework actor/UI runtime/contract/host TypeScript and Rust files, React renderer files, package-local tests/bindings, and build target directories.

## Purity Gap

The scanner is filename/directory allowlist-oriented. `isPackagingFile` explicitly accepts ecosystem entry and leaf filenames, `📦️index.ts[x]`, emoji-prefixed test/spec names, story leaves and all configured suffixes. It never parses bodies to prove glue. Therefore implementation can be preserved merely by an allowed name. The new analyzer must recurse and classify every package source as configuration, declaration/registration, thin delegation, implementation or unresolved.

Rust can reuse the repository's path parser but needs a syntax-aware body classifier for domain `struct`/`enum`/`trait`, meaningful `impl` blocks and non-trivial functions. TypeScript/JavaScript needs import/export/re-export/registration/bootstrap classification. Go, Python and .NET need corresponding conservative grammars. Unknown syntax must be unresolved, never valid glue.

## Adapter Ownership

- Rust: module/path/include macros and Cargo target/workspace/build paths.
- TypeScript/JavaScript: imports/exports/require/dynamic imports, package exports/types/bin, TS paths, Nx, Vite/Vitest/Storybook.
- Go/Python: modules/workspaces/imports/embeds/generators and package discovery/entry/resource configuration.
- .NET/native: project/solution/compile/content references, includes, CMake source lists and resources.
- Structured: JSON/JSONC, YAML, TOML, XML, Markdown, launch/tasks/CI and fixture manifests.
- Generated: template/generator source first, then regeneration and stale-output proof.

Each edit must identify adapter and structured location and carry a preimage hash. Shared Cargo, JS/Bun, Nx, TS, Go, .NET, CMake, CI, docs and generated families each require one writer.

## Acceptance Checks

Every package file has a deterministic role; no implementation remains under any `📦️packages`; every package source passes its ecosystem grammar; every moved target has all structured incoming references updated; ambiguous/binary references remain unresolved and block apply; compiler/package metadata validation agrees with the internal graph.
