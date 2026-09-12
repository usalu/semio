# Router Lexical Prebinding Acceptance Audit

Date: 2026-09-12

## Verdict

Accepted for the bounded command-router lexical-scope and package-discovery correction. The source recognizer now models program and block declaration visibility before initializer/body analysis, retains scope boundaries, and admits the current plugin router through a narrow imported path-conversion rule. No implementation or schema source was changed by this audit.

## Independent Scope Evidence

The retained `🧪router-scope-controls.ts` uses TypeScript's compiler and symbol checker independently of the owned recognizer. Its current results are:

| Control | Owned disposition | Native evidence |
| --- | --- | --- |
| Ambient console and runtime import | `tool-metadata` | clean; `execute` resolves to imported declaration |
| Block, closure-parameter, and loop shadows | `unresolved` | lexical local/parameter binding wins where referenced |
| Sibling block with a local `console`, then ambient `console.log` outside it | `tool-metadata` | clean; outer use remains ambient |
| Two sibling `BundleScript` classes, only one with a local `console` | `tool-metadata` | clean; the other class resolves `execute` to the import |
| Runtime aliases | `tool-metadata` | clean |
| Type-only alias used at runtime | `unresolved` | TS1361 |
| Later local `console` and `process` bindings | `unresolved` | TS2448 for each |
| Closure capture of later local `execute` | `unresolved` | both arrow references resolve to the line-2 local declaration; no diagnostic is expected for the legal capture |
| Self-referential pending router local | `unresolved` | TS2448 and TS7022 |

Source inspection confirms the mechanism behind those results. Imports are installed first; `ecmaRoutePrebind` then places program non-import declarations, and each `ecmaRouteBlock` prebinds only its own declarations. `EcmaRouteScope.initialize` replaces only a same-scope `pending` slot. Child blocks, loop scopes, closures, and method scopes each receive their own `EcmaRouteScope`, so a child declaration cannot contaminate a sibling or parent scope. The control outcomes above exercise both the rejection and non-leak sides of that design.

## Router Constructor And Discovery Evidence

The nested router constructor control and the live plugin source both return `tool-metadata`. The plugin has seven text lines (eight newline segments in the census) and only imports `dirname`, `fileURLToPath`, the established router/terminal, and imported command classes. The admission is limited to one-argument calls whose imported root has original export identity `dirname` or `fileURLToPath`, with recursive input validation. The prior hostile arbitrary `execute([])` registration input remains outside this route.

The public `discoverPackageProblems` path was independently exercised using a temporary manifest-bearing package:

| Wrapper | Result |
| --- | --- |
| Type-only terminal called as a value | `package-role-unresolved` |
| Type-only ordinary delegate called as a value | `package-role-unresolved` |
| Runtime `runArtifactRustPackageMain` wrapper | no package problem |

This proves the actual fallback boundary still rejects type-only routing forms and accepts the established runtime terminal; it is not a direct-classifier-only result.

## Focused Verification

- Direct `bun ./📜️script.ts test package-body-policy`: **117 pass, 455 expectations, 0 fail** in 6.04 seconds.
- Registered `@semio-tech/repo-lib:test-package-body-policy`, cache skipped with ticket-local Nx workspace and test-artifact paths: **117 pass, 455 expectations, 0 fail**; Bun 5.67 seconds, Nx 5.9 seconds, critical path 5.8 seconds.
- The independent control runner reproduced every row above against the current source and cleaned its temporary discovery package afterward.

## Limits

This accepts only the observed lexical shadowing, precise imported path-conversion forwarding, and package-discovery fallback behavior. It does not treat broader unfamiliar constructors or delegated vocabulary as defects merely because they are outside this grammar, and it does not decide the separate fixed-script normalizer gate or the broader mandatory-script census.
