# Script Policy Audit

## Result

The settled command-router policy is not yet safe to accept as a closed, lexical grammar. Four TypeScript forms with type-only imports parse successfully, produce `tool-metadata`, and have the installed compiler's `TS1361` diagnostic because no runtime value was imported. Two receiver-rename pairs also show that `console.log` and `process.exit` are admitted from a parameter that shadows the intrinsic name, while the identical capability shape under a neutral name is rejected. These are source-disposition false admissions, not a preference about how many imported operations a router may chain.

The correction must make runtime value-import provenance and lexical scopes first-class in both the production recognizer and the installed-TypeScript oracle. A callee, superclass, router constructor, terminal, `console`, or `process` must resolve to its actual in-scope runtime binding. Global identifier-name sets and member-name allowlists cannot establish that fact.

## Confirmed Runtime-Binding Admissions

All sources below had zero TypeScript parse diagnostics. The value-import rows used an in-memory `dep.ts` module exporting the corresponding runtime values; the compiler's semantic diagnostics therefore isolate the source's type-only misuse rather than a missing module.

| Case | Current disposition | Compiler evidence |
| --- | --- | --- |
| `import type { execute }` followed by `execute(args)` | `tool-metadata` | `TS1361` |
| `import type { runBundleScriptMain }` followed by the terminal call | `tool-metadata` | `TS1361` |
| type-only `BundleScript`, `ScriptRouter`, and terminal used as runtime values | `tool-metadata` | three `TS1361` diagnostics |
| `import { type execute as delegated }` followed by `delegated(args)` | `tool-metadata` | `TS1361` |

The production source at `🔍️discovery/🟦️.ts:10247` accepts `import type` in `ecmaImportedBindings`, strips braces and `type`, and stores the remaining name in the same set as a runtime import. The installed oracle also records named imports without checking `ImportClause.isTypeOnly` or `ImportSpecifier.isTypeOnly`, so it repeats the same error. Its current agreement with the production recognizer is therefore not independent evidence for runtime binding provenance.

## Confirmed Lexical-Receiver Admissions

Both members of each pair are syntax-clean and use the same method body structure. The name alone changes the decision.

| Parameter capability | Intrinsic-looking name | Neutral rename |
| --- | --- | --- |
| `.log(args)` | `console: any` → `tool-metadata` | `output: any` → `unresolved` |
| `.exit(args)` | `process: any` → `tool-metadata` | `lifecycle: any` → `unresolved` |

The live router precomputes global `parameters`, `localBindings`, and `trustedMemberRoots` before it parses class-body statements (`🔍️discovery/🟦️.ts:10399`). It later grants special handling to the text `console.log`, `console.error`, and `process.exit`. A parameter shadows the ambient intrinsic binding, but the name-only special case still admits it. Local object receivers with a forwarding closure are admitted for the same reason. Local object receivers with a nested `throw` happened to reject, but that rejection comes from unrelated textual detection and does not establish lexical resolution.

## Boundary Observations

Opaque values returned by imported owners remain valid router plumbing. The following parse-clean forms currently classify as `tool-metadata`, and this audit does **not** treat them as defects solely because their imported receipt is forwarded:

- `await execute(prepare(args))`
- `const receipt = prepare(args); await execute(receipt)`
- `const invoke = () => execute(receipt); await invoke()`

The existing portable positives already cover imported result status and imported argument resolution. They justify that narrow conclusion.

The scratch probe also found admitted local map/callback forms over imported receipts or command arguments. The portable fixture currently permits literal path map/flatMap construction, but it does not yet establish the exact accepted source provenance for these broader forms. They remain a follow-up design/fixture decision; this audit does not ask the correction to reject ordinary parameter indexing or opaque imported receipt forwarding.

## Regression And Focused Verification

The retained coordinator report `🗑️generated/coordinator/router-argument-adversaries-corrected.json` shows the original positive delegation remains `tool-metadata` and all nine original hostile controls are now `unresolved`.

I ran the current registered policy suite directly and through its isolated Nx target:

- `bun test ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧪️tests/📦️package-boundary-classification/🟦️.ts` — 91 passing tests, 326 assertions.
- `NX_DAEMON=false NX_ISOLATE_PLUGINS=false ... bun nx run @semio-tech/repo-lib:test-package-body-policy --skip-nx-cache` — passed; Nx reported 5.1 seconds, with a 5.0-second critical path.

Those routes prove the existing vector corpus and manifest-present/absent fixture still execute. They do not cover the four type-only forms or the intrinsic rename pairs, so their green result cannot establish the stronger closure claim in the executor report.

The repository ownership correction is coherent within its narrow scope. The direct `test-repo-source-ownership` route passed 7 tests and 219 assertions. Its two actual owners have the complete registered chains:

1. `modules` → `📚️library` → `members-of-modules`.
2. `modules` → `📚️library` → `members-of-modules` → `🕸️dependencies` → `members-of-members-of-modules` → `🧩️runtime` → `members-of-members-of-members-of-modules`.

No broad `library`, `dependencies`, `runtime`, declaration, or source exemption was observed. This audit intentionally did not broaden the package-only policy to the queued 455-script census.

The native DSL law mismatch remains a separately documented source/fixture inconsistency in `📓️dsl-fixture-source-provenance-2026-09-12.md`. No law was invented and no expectation was weakened.

## Required Correction Evidence

The implementation follow-up should add exact portable valid/hostile pairs and make both recognizers agree on them:

1. Runtime import versus `import type` and named `type` import for each ordinary delegate, terminal, router constructor, and base class role.
2. Ambient `console`/`process` versus a local or parameter shadow, including the neutral-name rename pairs above.
3. Nested lexical scopes, aliases, closures, registration and terminal arguments, validated through the TypeScript AST's actual bindings rather than a global name set.

Retain the existing finite literal loop/path-map positives and the original nine hostile controls. The correction must use a recursively scoped expression/statement grammar, not broaden a name blacklist or add a filename, class, path, or import-name exception.
