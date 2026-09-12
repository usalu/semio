# Router Scope And Discovery Fallback Audit

Date: 2026-09-12

## Result

The scoped command-router policy is not yet semantically complete. The owned parser has three false admissions caused by later lexical bindings, and it rejects one current, implementation-free router shape. No implementation source was changed by this audit.

`🧪router-scope-controls.ts` is the retained independent control runner. It uses the installed TypeScript compiler API as a separate AST and binding oracle, then exercises the public source-disposition and disk-discovery APIs.

## Lexical Binding Controls

| Control | Public disposition | TypeScript evidence | Disposition |
| --- | --- | --- | --- |
| Unshadowed ambient `console` with imported delegate | `tool-metadata` | clean | Correct acceptance |
| Block-local `console` shadow | `unresolved` | clean | Correct rejection |
| Closure parameter `execute` shadow | `unresolved` | checker binds both uses to the parameter | Correct rejection |
| Loop-local `process` shadow | `unresolved` | clean | Correct rejection |
| Runtime import aliases | `tool-metadata` | clean | Correct acceptance |
| Type-only import alias used at runtime | `unresolved` | TS1361 | Correct rejection |
| `console.log(args); const console: any = args;` | `tool-metadata` | TS2448 | **False admission** |
| `process.exit(await execute(args)); const process: any = args;` | `tool-metadata` | TS2448 | **False admission** |
| `const invoke = () => execute(args); const execute: any = args; await invoke();` | `tool-metadata` | the compiler binds the arrow references to the later local declaration on line 2 | **False admission** |

The parser validates and installs each `const` while traversing `ecmaRouteBlock`. JavaScript instead creates the lexical binding for the full block before execution; early evaluation hits its temporal dead zone. The closure is also validated before the later method-local binding is installed, so the parser sees the imported `execute` even though the native checker resolves the capture to the local binding. The correction needs lexical declaration visibility for the full block/closure scope before routing provenance is accepted. It must retain the already-correct ordinary block, closure-parameter, and loop-shadow rejections.

## Current Router Acceptance Gap

The current plugin router at `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/📜️script.ts` has seven text lines of routing only (the census counts eight newline segments): it imports `dirname`, `fileURLToPath`, `ScriptRouter`, the existing terminal, and two imported command classes; creates and registers the router; then invokes the imported terminal. It contains no domain behavior.

The direct paired control reproduces its nested constructor value:

```ts
new ScriptRouter(dirname(fileURLToPath(import.meta.url))).register(...)
```

It is `unresolved` with no TypeScript diagnostics. `ecmaRouteValue` correctly proves each imported conversion as an opaque receipt, but `ecmaRouteRegistrationValue`, used for `ScriptRouter` construction, accepts only data or finite values. Imported opaque results are already allowed to flow to imported owners; this constructor argument is that same valid routing pattern. Admit this proven nested imported receipt specifically through the router-constructor path, without a filename or path exception and without treating hidden local behavior as routing.

## Actual Discovery Fallback

The public `discoverPackageProblems` path was exercised with a temporary manifest-bearing package, not only the specialized classifier:

| Source body | Actual problem |
| --- | --- |
| `import type` terminal, then runtime terminal call | `package-role-unresolved` |
| `import type` delegate, then runtime delegate call | `package-role-unresolved` |
| Runtime `runArtifactRustPackageMain` terminal wrapper | no problem |

This confirms the current `collectPackageRoles` fallback preserves a specialized `unresolved` decision when generic classification does not establish implementation, while still admitting a valid runtime wrapper. The type-only cases are not silently admitted by generic thin-delegation classification. This conclusion concerns package discovery only; the separately queued fixed-script package/nonpackage normalizer closure remains outside this parser slice.

## Focused Verification

- Direct `bun test` on `📦️package-boundary-classification/🟦️.ts`: 113 tests, 431 expectations, 0 failures (6.35 seconds).
- Isolated registered Nx target `@semio-tech/repo-lib:test-package-body-policy`, with `NX_DAEMON=false`, a ticket-local `NX_WORKSPACE_DATA_DIRECTORY`, ticket-local `SEMIO_TEST_ARTIFACT_DIR`, and cache skipped: 113 tests, 431 expectations, 0 failures (Bun 6.51 seconds; Nx 6.7 seconds).
- Fresh independent control run: the three TDZ/later-local admissions and the nested-receipt rejection above reproduced; the paired acceptance, shadow, type-only, and disk-discovery controls had the exact results listed above.

The focused suite predates these four controls, so its green result does not establish acceptance of this audit's identified gaps.

## Current Validation Limits

The fresh no-follow census is a snapshot of 455 mandatory scripts and 104,388 source newline segments: 277 `tool-metadata`, 178 `unresolved`, and 12 unresolved scripts shorter than 30 lines. It scanned 160,005 entries in 4.854 seconds without absent roots, source-read failures, or TypeScript parse errors. The 178 rows combine real bodies with valid router forms that are not yet modeled; they are not a defect count.

This audit did not assess the fixed-script semantic-enforcement gap, infer source extraction requirements from a short file, or widen exemptions for existing styling violations. It is limited to the command-router lexical binding and package-discovery fallback path.
