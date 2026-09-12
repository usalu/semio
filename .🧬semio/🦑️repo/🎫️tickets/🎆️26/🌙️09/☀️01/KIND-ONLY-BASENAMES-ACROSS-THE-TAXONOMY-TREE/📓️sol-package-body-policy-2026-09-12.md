# Package Body Policy Execution — 2026-09-12

## Result

The package-purity gate now evaluates source content after a path is admitted as package glue or as an exact external-tool filename. Discovery and normalization share `classifyPackageSource(content, grammar)`, which returns the role and supporting evidence. The former role-only public wrapper and normalization's duplicate classifier were removed; active consumers use the shared decision API.

The classifier treats executable computation and owned domain declarations as implementation. Type-only import and re-export wiring remains declaration. An enum, struct, interface, type alias, or literal domain table is not admitted merely because a language parser calls it a declaration. Exact compiler/plugin registration and bootstrap shapes retain narrow roles.

Source-format tool entries require a live fixed/configurable contract and a validator that covers the whole file. Vitest configuration admits only imports, its default `defineConfig` root, and private configuration helpers reachable from that root. Hidden domain classes, unused computation, side effects, and arbitrary top-level statements fail the disposition. Command routers admit one direct registered runner call, optionally with referenced `BundleScript` subclasses whose sole `run` method only delegates to imported runners. Unrelated classes, functions, or computation fail the disposition.

The five coordinator counterexamples are closed:

- a Python import followed by a function/class is implementation because the import grammar now covers the entire file;
- a private domain class beside Vitest `defineConfig` is not tool metadata;
- a private domain class beside `runBundleScriptMain` is not a command router;
- Rust macro names do not authorize arbitrary token bodies;
- C preprocessing directives remain visible and every function-like macro definition is authored implementation.

Rust registration is fail closed. Only exact `semio_framework_plugin::plugin_exports`, `semio_framework_plugin::extension_exports`, and `inventory::submit` forms with their narrow path/constructor argument grammar are registration. Unknown macros remain unresolved; an executable body under a recognized registration macro is implementation. C/C++ admits include/pragma/typedef/using/extern declarations and does not treat an arbitrary `#define` as trivia or declaration.

The `root-script` disposition now declares `grammarId: "typescript"`. Discovery and normalization therefore analyze `📜️script.ts` as TypeScript even inside Rust, Go, .NET, or Python packages instead of incorrectly selecting the enclosing package ecosystem grammar.

The repository-root package-purity implementation is `/Users/ueli/Documents/semio/📜️script.ts`. The 595-byte `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/📜️script.ts` file is only the product wrapper and was not changed for the gate.

## Portable contract and independent oracles

The language-neutral JSON fixture contains 39 positive and negative cases and is independently validated with Ajv. TypeScript/JavaScript cases use the TypeScript compiler AST, Rust cases use a standalone `syn` executable, Go cases use `go/parser`, Python cases use Python's native `ast`, and C cases use the system compiler preprocessor. These dependencies and toolchains are test-only.

The fixture includes exact positive and negative forms for delegated routes, domain declarations, literal data ownership, private configuration helpers, hidden tool-entry bodies, direct and class-based command routers, known and unknown Rust registration macros, Python whole-file wiring, C includes/type aliases, and C function-like macro computation/calls.

The focused command remains registered end to end:

- direct package route: `bun ./📜️script.ts test package-body-policy`, with cwd `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript`;
- package target: `nx run @semio-tech/repo-lib:test-package-body-policy`;
- root package script: `test:repo-lib:package-body-policy`;
- launch entry: `🧹clean📦️package🧪️body-policy` in `.vscode/🧩️launch.seed.jsonc` and derived `.vscode/launch.json`.

The old remaining-package-purity mapping stays immutable under `remaining-package-purity-history-v1`. Its `decisionState` identifies it as non-authoritative concurrent source-byte evidence. The portable fixture and current discovery census are the active policy evidence.

## Oracle taxonomy correction

The Rust oracle's `Cargo.toml` and `Cargo.lock` moved from the semantic oracle owner into ordinary `🔮️oracles/📦️packages/🦀️rust`. Its anonymous `🦀️.rs` source remains at the semantic oracle owner, as do the anonymous Go and Python sources. The manifest's explicit binary path points back to `../../🦀️.rs`; native invocation uses the nested package root.

`📦️package-boundary-classification` is registered as an exact member of the existing tests context. The final scoped inventory has 16 entries. Every newly owned test-context, oracle, package, Cargo metadata, and anonymous source node is clean. Its only two findings are the ambient ancestors `.../📚️library` and `.../📚️library/🧹️normalization`, both `directory-kind-unresolved`; the correction added no overlapping global kind and did not add a Cargo metadata exemption.

## Live census and extraction backlog

The original reviewed census remains captured in [the retained extraction backlog](./📓️package-body-extraction-backlog-2026-09-12.md): 63 authored extraction candidates, one facade for parser/ownership review, and six structural or generated-output ownership items. This retained backlog was not expanded into source moves in this policy lane.

After closing the whole-file disposition bypasses, the final live body census reports 255 implementation and four unresolved findings. Of those, 187 are required-name `📜️script.ts` files and are retained separately in `🗑️generated/sol-package-body-policy/script-semantic-review-final.json`. They are semantic review candidates rather than automatic extraction orders: AGENTS requires permanent executable orchestration to remain in `📜️script.ts`, while unrelated domain implementation still cannot hide behind that filename. The full current result is `live-body-findings-final.json`; counts are in `live-body-counts-final.json`.

The remaining package-purity work therefore separates source-body extraction, required script orchestration review, package output/cache ownership, and unsupported grammar review. No application body, generated source, or script was moved in this lane.

## Validation

- Direct registered package route: 45 passed, zero failed, 138 expectations (`registered-test-final.log`). This includes schema validation and every independent oracle.
- Scoped taxonomy inventory: 16 entries and only the two ambient ancestor findings described above (`own-taxonomy-corrected-final.json` and `.stdout.log`).
- Focused current live census: 259 body findings, consisting of 255 implementation and four unresolved; 187 required scripts are separated for semantic review (`live-body-findings-final.json`).
- The earlier isolated Nx focused target did not reach test execution because graph construction referenced a missing mathematical-equation `json-engine/Cargo.toml` and a nonexistent `npm:@asamuzakjp/css-color` source project (`nx-focused-test.log`). The successful 45-test result is the registered direct package route, not an Nx pass.
- Repo-lib TypeScript lint remains red on workspace-wide `rootDir` and normalization type diagnostics captured in `tsc.log`. This report does not assign baseline provenance to those diagnostics.
- The existing generic-stem run reached 38 passes and seven stale-UI-coordinate failures (`generic-stem-regression.log`); that source-relocation fixture work is outside this bounded correction.

## Files changed

- `/Users/ueli/Documents/semio/📜️script.ts`
- `/Users/ueli/Documents/semio/package.json`
- `/Users/ueli/Documents/semio/.vscode/🧩️launch.seed.jsonc`
- `/Users/ueli/Documents/semio/.vscode/launch.json`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧬️schema/📦️package-boundary-classification/🔣️.json`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧫️fixtures/📦️package-boundary-classification/🔣️.json`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧪️tests/📦️package-boundary-classification/🟦️.ts`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧪️tests/📦️package-boundary-classification/🔮️oracles/🦀️.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧪️tests/📦️package-boundary-classification/🔮️oracles/🐹️.go`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧪️tests/📦️package-boundary-classification/🔮️oracles/🐍️.py`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧪️tests/📦️package-boundary-classification/🔮️oracles/📦️packages/🦀️rust/Cargo.toml`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧪️tests/📦️package-boundary-classification/🔮️oracles/📦️packages/🦀️rust/Cargo.lock`
- removed former oracle-root `.../🔮️oracles/Cargo.toml` and `.../🔮️oracles/Cargo.lock`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/💥️generic-stem-collision-resolution/🟦️.ts`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📋️project.json`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/package.json`
- this report.
