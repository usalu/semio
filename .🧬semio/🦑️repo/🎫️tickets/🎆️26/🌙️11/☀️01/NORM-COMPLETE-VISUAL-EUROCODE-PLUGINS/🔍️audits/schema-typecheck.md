# Norm TypeScript Semantic Closure Verification

The actual TypeScript compiler checked all fifteen artifact cohorts: the root artifact schema, snapshot schema, diff schema and mutation aggregate for each standard. All 60 committed entry modules resolve a closure of 2824 TypeScript/library source files. No declaration or reader errors were found: zero pre-emit diagnostics.

Options: strict, noEmit, skipLibCheck, resolveJsonModule, esModuleInterop, allowImportingTsExtensions, ESNext module/target, bundler resolution, and an empty ambient type package list. This is genuine compiler semantic verification of the actual imports and declarations, independent of textual import/path scans or schema projection checks. skipLibCheck suppresses declaration-file validation only; production .ts declarations remain checked.

A committed language-neutral fixture specifies the four cohort facets, 15 expected owners, 60 expected roots and zero diagnostics. The new permanent Bun regression invokes the real compiler and is included by the main norm package-runner fixture. No runtime code or external runtime dependency was added.

Validation: 1 test passed, 0 failed, 3 assertions, 15.59 seconds. The audit script and test both ran through normal repo tool capacity admission with 2048 MB reserved.

Command: repo --service=off tools exec bun -- test ./✏️s/🔌️plugins/📕️norm/🧪️tests/🧬️schema-typecheck/🟦️.ts (session, language and shared writable host-capacity directory supplied).

## Files

- ✏️s/🔌️plugins/📕️norm/🧪️tests/🧬️schema-typecheck/🟦️.ts
- ✏️s/🔌️plugins/📕️norm/🧪️tests/🧫️fixtures/🧬️schema-typecheck/🔣️.json
- ✏️s/🔌️plugins/📕️norm/🧪️tests/🧫️fixtures/🏃️package-runner/🔣️.json
- Ticket input: 🧰️scripts/schema-typecheck.ts

The generated machine report is disposable and remains under the ticket generated directory for parent cleanup.
