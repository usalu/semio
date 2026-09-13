# Demonstrator Guarded Registration Grammar Audit

## Status

Read-only source audit. Root reports its focused portable control as one test, 300 assertions, green; this audit did not rerun it. The registered classification route and Root's in-flight optional-chain and argument-shape controls remain pending.

## Current Parser Review

The changed parser in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts` parses a unary `await` by recursively parsing its operand at precedence 9. The operand parser still consumes member and call postfixes before considering binary operators. Consequently `await import("./🧪️tests/…")` now forms a unary await whose operand is the dynamic-import call, rather than a call whose callee is the awaited `import` identifier. This is the necessary precedence correction.

`ecmaRouteTestRegistration` remains deliberately narrow:

- exactly one `if (import.meta.vitest)` block, with no `else`;
- exactly two statements: one destructured `const` from an awaited, literal, relative dynamic import and one awaited invocation;
- one unshadowed callback binding, invoked once with exactly three arguments;
- an exact `import.meta.vitest` first argument;
- only direct runtime imports, literals, and `import.meta.dir` or `.url` as the remaining dependency values;
- no spread, computed property, defaulted binding, second registration block, or local shadow.

The current Demonstrator package script has that form: it dynamically imports its direct test owner, calls `registerTests1(import.meta.vitest, { demonstratorRuntimeBuildVariants, join }, { directory: import.meta.dir, url: import.meta.url })`, and retains its existing standard `runBundleScriptMain` terminal. The test owner exports `registerTests1`; it registers two Vitest cases. Classification reads structure only and does not execute that callback or the Demonstrator application.

## Evidence And Limits

The fixture adds the true guarded registration form plus hostile computed dependency and spread dependency cases. Root also reports an installed TypeScript AST/compiler check. This source review agrees that the new form is bounded and does not introduce a product-specific command terminal or a broad dynamic-import exemption.

Acceptance remains conditional on the in-flight optional-chain and argument-shape parity controls and the registered classification result. No Demonstrator build, Vite server, browser, or application test is implied.
