# Runner Inventory Scope Audit

## Result

The apparent `518` versus `657` discrepancy is a scope and snapshot difference, not evidence that a JavaScript-family extension was omitted.

The saved TypeScript runner classification snapshot contains **518** direct canonical JavaScript/TypeScript leaves:

| Filename | Count |
| --- | ---: |
| `🟦️.ts` | 445 |
| `🟦️.tsx` | 63 |
| `🟨️.js` | 10 |
| `🟨️.mjs` | 0 |
| Total | 518 |

Its category counts (`401` native-suite, `135` guarded-production-registration, `331` exported-self-test-invoked/dispatcher reference, and `10` feature-adapter) overlap. They are classifications of the same leaves, not four additive inventories.

The earlier focused language audit deliberately covered JavaScript, TypeScript, Python, and Go only. Its reported `657` canonical implementations decomposes as the prior JavaScript/TypeScript snapshot (`518`) plus `139` Python/Go leaves. Rust was not in that audit's `extensions` filter, so a raw count of Rust files below `🧪️tests` is outside this comparison.

## Current Read-Only Listing

The current tree has advanced after the saved runner snapshot. Re-listing with `rg --files --no-ignore --hidden` and the runner's direct-leaf matcher now returns **519** JavaScript/TypeScript leaves:

| Filename | Current count |
| --- | ---: |
| `🟦️.ts` | 446 |
| `🟦️.tsx` | 63 |
| `🟨️.js` | 10 |
| `🟨️.mjs` | 0 |
| Total | 519 |

The sole path added since `🗑️generated/typescript-finish/runner-classification.json` was written is:

`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🕸️wasm/🟦️.ts`

There are no removed paths. This is a canonical, routed leaf, not an unclassified executable case: `testWasmOptimizer` is imported and awaited by `🧪️tests/⚡️cache-contracts/🟦️.ts`; `📜️script.ts` imports that cache-contract suite and its `test` router invokes it; `📋️project.json` maps the `repo:test` target to `bun ./📜️script.ts test`.

The same current direct-leaf listing has **134** `🐍️.py` and **9** `🐹️.go` leaves, or **143** native-language leaves. Thus the corresponding current source-only total is **662** (`519 + 143`). The `657/139` values are an earlier audit snapshot; this bounded check did not attribute the four subsequent native-language additions.

## Scope and Extension Checks

The runner matcher is exactly a direct path:

`<owner>/🧪️tests/<one case>/🟦️.(ts|tsx)` or `<owner>/🧪️tests/<one case>/🟨️.(js|mjs)`.

Its source list excludes `node_modules`, `📤️dist`, `🗑️generated`, `.git`, and the repository ticket tree. The full layout inspector additionally excludes the repository metadata root and taxonomy-defined path exclusions. Neither scan treats build/dependency/ticket inputs as authored implementations.

No direct JavaScript/TypeScript leaves with a noncanonical filename were found under the runner exclusions. There are also zero direct `*.mts`, `*.cts`, `*.jsx`, or `*.cjs` leaves, so those declared extension chains do not change the runner result. This audit did not rerun executable-body analysis; it found no concrete unclassified case to report.

## Evidence

- Runner snapshot classifier: `🗑️generated/typescript-finish/runner-classification.json`
- Runner classifier implementation: `🗑️generated/typescript-finish/runner-audit/📜️script.ts`
- Current language audit: `📓️test-layout-final-current-audit-2026-09-08.md`
- Taxonomy-based layout scanner: `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts`
