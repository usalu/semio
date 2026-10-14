# Complete Authorizer Runtime Source Capture

Normal SDK provisioning correctly refused the newly consumed leases metadata module, but the initial diagnosis of concurrent Source edits was incorrect. The physical module mtime was stable. Direct capture produced 95 Sources with no metadata row, while the compiler consumed the real runtime dynamic import. Bun's `scanImports` omitted dynamic imports; `scan(source).imports` includes them. Exact expected-byte comparison prevented this incomplete source census from being published.

The collector now uses the complete runtime scan. Source path normalization, no-follow ancestry and regular-file validation, exact hashes, the closed manifest schema, explicit system/tool external module allowlist, and Source/byte caps remain in force. This retains runtime literal dynamic imports and the existing UTF-8 / escaped Unicode import semantics.

TDD extended the language-neutral corpus with an actual lazy TypeScript module. The old implementation failed both tests: the expected source census lacked the lazy module, and the genuine release compiler refused its consumed bytes. The repaired collector matches independent esbuild metafile coverage and the own closed manifest validated by Ajv. The actual compiled bundle executes in Node with lazy value 17. Editing either the dynamic Source module or the static helper refuses current source receipts and stale compiler input.

Actual registered private Nx authorizers gates passed prototype and release: two laws / 15 assertions each (689ms / 348ms test bodies), plus strict/noUnchecked TypeScript validation. The first repaired body gate passed runtime but exposed an existing preflight `findLast` use unsupported by its ES2022 type target; the owning agent preserved last-row semantics with `reverse().find`, and the complete gates then passed without weakening the type target.

These are collector and actual release-runtime qualifications. Current normal SDK publication and ordinary Rust TaskOrchestrator qualification are recorded separately after the real commands complete.
