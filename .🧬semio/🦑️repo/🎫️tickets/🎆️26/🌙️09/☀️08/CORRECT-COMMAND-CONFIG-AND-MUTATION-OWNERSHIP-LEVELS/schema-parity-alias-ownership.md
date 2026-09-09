# Shared Schema Inspector Ownership

The shared schema inspector now resolves explicitly named TypeScript aliases and reexports back to the defining interface. Artifact/snapshot schemas can share one canonical declaration. Missing exports, missing modules, and cyclic aliases still produce missing-declaration findings. Runtime inspection uses only first-party code and filesystem reads; it never evaluates imported modules.

Eleven neutral module graphs are compared with the independent TypeScript compiler symbol/type checker, including renamed imports, local exports, chained module aliases and invalid cycles. The registered Bun/Nx field-parity test first failed on the local exported alias (`schema-parity-alias-red-1.log`) and then passed after implementation (`schema-parity-alias-green-1.log`, exit 0). Existing native field extraction, Ajv exact-record comparison, and fast-glob owner discovery also passed in that run.

GraphQL metadata extraction is being corrected separately: directive arguments currently become false document fields, and comments/quoted strings can interfere with body discovery. Neutral GraphQL/TypeScript equivalents now cover directive ordering, nested argument records and misleading strings/comments; independent TypeScript AST extraction provides the field-set oracle.

Files changed:

- `📜️script.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📏️ownership/🧫️fixtures/🪪️field-parity/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📏️ownership/🧪️tests/🪪️field-parity/🟦️.ts`

No new command was introduced; existing registered test/report/enforce commands are used.


The GraphQL neutral test first failed because `roles`, `many`, and `class` were reported as fields (`schema-parity-directive-red-1.log`). The corrected extractor masks strings/comments and separates directive argument nesting from top-level field declarations; all three neutral cases passed (`schema-parity-directive-green-1.log`). That run exposed two additional file-based ownership audit entry points still bypassing the module resolver. Both are now corrected, with an integration assertion for the authored TypeScript corpus; the integrated report is pending.


Final integration passed: `schema-parity-integrated-green-2.log`, exit 0, fifteen TypeScript alias/inheritance module graphs, three GraphQL metadata cases, independent TypeScript compiler/AST, Ajv field sets and 192-owner fast-glob discovery. The first integration retry exposed CAD's empty interface extending an imported artifact; inherited fields, own-field overrides and diamond inheritance are now resolved. The report lists exactly seven remaining authored disagreements, all Layout. The previous four missing-TypeScript-declaration findings are gone. This is a passing report command, not a zero-breach enforcement result.


The TypeScript extractor now preserves `@state` when a property JSDoc also carries child-slot tags. The neutral fixture was corrected to place JSDoc at a compiler-recognized property boundary; the next run reproduced the real extractor mismatch (`schema-parity-child-state-red-2.log`). The fixed extractor matched the independent compiler JSDoc tags and passed the complete registered test (`schema-parity-child-state-green-1.log`, exit 0): sixteen alias/inheritance/state module graphs, three GraphQL cases and 192 owners. The first red attempt (`...red-1.log`) failed fixture/oracle agreement and was not product evidence.
