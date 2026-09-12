# Tool Disposition Precision Correction

## Scope

This bounded correction makes the package-body policy recognize two real tool-wiring forms without admitting a filename, import, runner name, or declaration by presence alone. It also repairs the seven live paths in the generic-stem filesystem fixture. No application source was extracted, no package-body finding was suppressed, and no taxonomy or package-boundary schema entry was broadened.

## Exact policy changes

The ECMAScript statement splitter now treats a newline immediately after `=>` as continuation of the same statement. This admits the renderer's reachable multiline `elementSuite` helper while leaving the whole-file Vitest grammar intact. The validator still accounts for every statement, requires the `defineConfig` module shape, checks helper reachability, and rejects unrelated domain declarations, unreferenced computation, and top-level effects.

The command-router grammar now recognizes the common repository `BundleScript` orchestration form under closed structural rules:

- each admitted class extends `BundleScript` and has exactly one `run` method;
- a `run` body contains only bounded delegation statements to imported functions, including local `const` argument plumbing whose right-hand side is an imported call;
- the remaining module contains only imports, the optional `ScriptRouter` construction/registration expression, and one supported imported main runner;
- every admitted command class is referenced by the router wiring.

The focused live regression admits these current files as tool metadata:

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript/vitest.config.ts`
- `🧰️framework/🛍️products/💻️os/🎚️config/📦️packages/🦀️rust/📜️script.ts`
- `🧰️framework/🔨️modules/🔀️dispatch/📦️packages/🦀️rust/📜️script.ts`

The repository-library `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts` remains unresolved because it contains behavior beyond the admitted orchestration grammar. This demonstrates that the mandatory `📜️script.ts` name is not a blanket body exemption.

The portable fixture now includes a renderer-shaped reachable multiline arrow positive, the same configuration with an unrelated domain function negative, and a command-router argument-plumbing positive. The independent TypeScript AST oracle implements the same boundary separately. The existing whole-file rejection coverage remains green for the seven audited implementation variants: Python import plus function, Vitest configuration plus hidden class, root script plus hidden class, unknown Rust registration computation, C arithmetic macro, known Rust registration with an executable call, and C domain-call macro. Additional closed-grammar negatives remain in the fixture.

## Generic-stem coordinate closure

The language-neutral filesystem fixture now points to the live owners:

- `⌨️control-keybinding-context` → `🕹️control-keybinding-context`
- both `☑️Select` rows → `🔽️Select`
- `🎚️Toggle` → `🔀️Toggle`
- `📊️Diagram` → `🕸️Diagram`
- `🪵️Tree` → `🌳️Tree`
- `🎨️styling/🧪️test/🟦️s.ts` → `🎨️styling/🧪️tests/🧩️suite/🟦️.ts`

These remain live filesystem assertions with the suite's independent native filesystem/glob checks; they were not converted to historical exceptions.

## Validation

All commands were run from the current shared workspace on 2026-09-12.

- Direct registered package route, from `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript`: `bun ./📜️script.ts test package-body-policy` — 49 passed, zero failed, 154 expectations. This exercises 42 portable vectors plus the independent TypeScript, Rust, Go, Python, and C/C++ native or AST oracles. Evidence: `🗑️generated/sol-tool-disposition/package-body-policy.log`.
- Generic-stem suite, from the repository root: `bun test './🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧪️tests/💥️generic-stem-collision-resolution/🟦️.ts'` — 45 passed, zero failed, 89 expectations. Evidence: `🗑️generated/sol-tool-disposition/generic-stem.log`.
- Strict taxonomy load, from the repository root: `loadTaxonomy()` completed with schema version 7. Evidence: `🗑️generated/sol-tool-disposition/strict-taxonomy.log`.

No Nx result is claimed for this correction. The policy was exercised through its registered package script directly, avoiding a broad workspace-graph run.

## Fresh categorized body census

The fresh focused census used `discoverPackageProblems(repositoryRoot, loadCatalogTaxonomy())` and retained only `package-implementation` and `package-role-unresolved` findings. It reports 207 findings: 204 implementation and three unresolved.

| Category | Total | Disposition |
| --- | ---: | --- |
| Authored package source | 62 | Extraction or ownership review backlog; unchanged by this bounded correction. |
| Mandatory `📜️script.ts` semantic review | 138 | Review individually against the exact orchestration grammar; the required filename is not itself an admission or an extraction order. |
| Structural or generated output | 5 | Three .NET `obj` files are unresolved and two VSCode `out` files contain implementation; handle as output ownership, not source-body exemptions. |
| Tool configuration review | 2 | The UI React `postcss.config.ts` declares an owned domain interface, and the OS dev `⚙️vite.config.ts` performs non-delegating work. The corrected renderer Vitest configuration is absent. |

The complete paths and messages are retained in `🗑️generated/sol-tool-disposition/body-census.json`; category counts are in `body-census.log`. The census remains intentionally red because body extraction and output ownership are separate execution lanes.

## Exact changed files

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧫️fixtures/📦️package-boundary-classification/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧪️tests/📦️package-boundary-classification/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧫️fixtures/💥️generic-stem-collision-resolution/🔣️.json`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/KIND-ONLY-BASENAMES-ACROSS-THE-TAXONOMY-TREE/📓️sol-tool-disposition-correction-2026-09-12.md`

Generated evidence is confined to `🗑️generated/sol-tool-disposition` under this ticket.

## Coordinator Registered Nx Verification

After the executor report, the coordinator ran `bun nx run @semio-tech/repo-lib:test-package-body-policy --skip-nx-cache` from the repository root with `NX_DAEMON=false`, `NX_ISOLATE_PLUGINS=false` and an isolated ticket workspace-data directory. Nx executed the target and exited zero: 49 tests passed, zero failed, 154 expectations. The target duration was 7.9 seconds, cache explicitly skipped; preceding graph preparation was longer and is not included in that target duration. This closes the executor's direct-route-only validation limit for the body policy. Exact disposable output is `🗑️generated/coordinator/tool-disposition-nx.log`. No source was changed by this coordinator check.
