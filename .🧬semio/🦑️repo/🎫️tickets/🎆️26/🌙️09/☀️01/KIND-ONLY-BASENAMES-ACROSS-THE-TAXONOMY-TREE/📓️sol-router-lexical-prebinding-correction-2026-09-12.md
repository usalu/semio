# Router Lexical Prebinding Correction

Date: 2026-09-12

## Outcome

The command-router policy now creates every lexical declaration binding for a program or block before it analyzes any initializer or body. A later `const console`, `const process`, or local `const execute` therefore shadows the ambient/imported binding throughout its lexical scope, matching TypeScript binding semantics. Each pending declaration can be initialized only in its exact scope slot, so sibling blocks remain independent and a declaration cannot overwrite an import, parameter, or another declaration.

The current OS plugin package router is accepted without changing its valid seven-line source. Its `ScriptRouter` constructor receives `dirname(fileURLToPath(import.meta.url))`; both calls resolve to runtime value imports with the original exported identities `dirname` and `fileURLToPath`. The registration grammar permits only those one-argument imported path conversions and recursively verifies their inputs. It still rejects the existing hostile `execute([])` constructor/registration argument and arbitrary imported effects.

## Portable And Native Proof

Four portable fixture cases extend the prior 113-test corpus:

- a later block-local `console` declaration makes the earlier `console.log` unresolved;
- a later block-local `process` declaration makes the earlier `process.exit` unresolved;
- a delegation closure captures the later local `execute` binding and is unresolved;
- the nested imported path conversion passed to `ScriptRouter` is tool metadata.

The existing unshadowed ambient, ordinary block shadow, parameter shadow, loop shadow, runtime/type-only import, opaque imported-result forwarding, immutable receipt forwarding, closure forwarding, harmless index, terminal fallback, and hostile argument controls remain unchanged.

The installed TypeScript oracle supplies two independent forms of evidence. Its AST implementation has its own lexical scopes and declaration prebinding. Its semantic program uses the TypeScript checker to resolve each runtime `execute` call through aliases to either the imported declaration or the local source declaration. The valid closure-forwarding fixture resolves to the import; the later-local fixture resolves to the local declaration. The compiler reports TS2448 for both temporal-dead-zone intrinsic cases, TS1361 for the four retained type-only runtime cases, and no diagnostic for the valid runtime controls or the legal later-local closure capture.

The retained audit runner `🧪router-scope-controls.ts` was rerun without modification. Its three prior false admissions now return `unresolved`, the nested router receipt returns `tool-metadata`, and all earlier acceptance/shadow/type-only results remain as audited. The actual package-discovery controls still produce `package-role-unresolved` for type-only terminal and ordinary delegate wrappers, while the runtime artifact wrapper produces no package problem.

## Test-Driven Evidence

Before implementation, the four new fixture cases reproduced the three false admissions and one false rejection. The focused run reported 112 passes, 5 failures, and 440 assertions; the fifth failure was the not-yet-extended semantic diagnostic collector.

After implementation:

- Direct `bun ./📜️script.ts test package-body-policy`: **117 tests, 455 assertions, 0 failures** in 7.07 seconds.
- Registered isolated Nx target `@semio-tech/repo-lib:test-package-body-policy` with cache skipped: **117 tests, 455 assertions, 0 failures**; Bun reported 12.76 seconds, Nx 13.4 seconds, and the critical path 13.1 seconds.
- Retained TypeScript audit control: later `console` and `process` cases are `unresolved` with TS2448; later-local closure capture is `unresolved` with the checker resolving both `execute` references to the line-2 local declaration; nested router receipt is `tool-metadata` with no diagnostic.
- End-to-end temporary-package discovery: type-only terminal and delegate are rejected, runtime artifact wrapper and the byte-current live plugin router are accepted.
- Direct live plugin source classification: `tool-metadata`.
- Strict `loadTaxonomy()` plus `validateTaxonomy()`: zero diagnostics.
- Catalog discovery: 223 packages and 223 unique package roots in the current shared checkout.

## Exact Changed Files

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts`: pending lexical bindings, full program/block prebinding, exact-slot initialization, and nested imported path-conversion admission.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧫️fixtures/📦️package-boundary-classification/🔣️.json`: three hostile lexical controls, one valid path-conversion router, and imported/local binding expectations.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧬️schema/📦️package-boundary-classification/🔣️.json`: portable `expectedExecuteBinding` contract.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧪️tests/📦️package-boundary-classification/🟦️.ts`: independent AST prebinding, TypeScript semantic diagnostics and symbol-origin proof, live plugin classification, and disk-discovery acceptance.
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/KIND-ONLY-BASENAMES-ACROSS-THE-TAXONOMY-TREE/📓️sol-router-lexical-prebinding-correction-2026-09-12.md`: retained correction evidence.

## Boundary

This correction is limited to the reviewed command-router grammar and package-discovery fallback. It does not claim semantic enforcement of fixed scripts through the general package/nonpackage inventory path, and it does not classify the broader 455-script census as accepted or defective. That integration gate remains a separate bounded lane.
