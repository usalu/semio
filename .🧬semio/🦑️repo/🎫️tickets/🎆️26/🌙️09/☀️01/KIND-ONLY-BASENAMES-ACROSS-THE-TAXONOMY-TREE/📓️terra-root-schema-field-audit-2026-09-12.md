# Terra Root Schema Field Extraction Acceptance Audit — 2026-09-12

## Disposition

**Accepted.** The reusable schema-field extraction concern is no longer implemented by the root router. Its eight direct owners contain the exact 19 declarations, direct consumers import those owners without a compatibility facade, the TypeScript IO seam is separated from pure extraction, and portable, direct, and registered controls are green.

This was a read-only audit. No product, Git, lifecycle, or AGENTS file was changed.

## Ownership closure

The portable ownership fixture at `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🧱️root-schema-field-source/🔣️.json` declares eight owners, 19 declarations, nine taxonomy contexts, three direct consumers, and one registered route. It has no source-body hashes or snapshots.

| Owner | Declarations | Boundary |
| --- | ---: | --- |
| `🧬️schema/🔍️field-discovery/🧱️contract/🟦️.ts` | 7 | field shape/cardinality/extract contract, canonical scalar/state, declaration selection, and snake-to-camel conversion |
| `🦀️rust/🟦️.ts` | 2 | pure Rust representation parsing/extraction |
| `🟦️typescript/🟦️.ts` | 2 | pure TypeScript text parsing/extraction |
| `🟦️typescript/📂️module-resolution/🟦️.ts` | 1 | file-backed TypeScript module traversal and resolution |
| `🔗️graphql/🟦️.ts` | 1 | GraphQL extraction |
| `🔣️json-schema/🟦️.ts` | 3 | JSON Schema scalar/property/extraction |
| `🛰️protobuf/🟦️.ts` | 2 | Protobuf type parsing/extraction |
| `⚖️comparison/🟦️.ts` | 1 | missing/extra field comparison |

The root imports each owner directly at `📜️script.ts:202-209`. Its AST contains none of the 19 moved declarations. The ownership test checks that condition, exact declaration lists, all nine `semanticDirectoryKindId` contexts, the three consumer bindings, and owner import acyclicity.

My independent import scan found eight nodes and seven internal edges: GraphQL, JSON Schema, Protobuf, pure TypeScript, and Rust each import the contract; module resolution imports pure TypeScript plus the contract; comparison is independent. There is no cycle and no owner import back to `📜️script.ts`.

## Consumer and IO separation

`📏️ownership/🧪️tests/🪪️field-parity/🟦️.ts:7-11` imports the Rust, pure TypeScript, GraphQL, and comparison owners directly. Its remaining root import is limited to unrelated artifact-policy APIs. `🧪️tests/⚙️root-script-compiler/🟦️.ts:8-10` imports the contract, Rust, and Protobuf owners directly; it parses each declared owner/declaration record rather than preserving a root definition for source-as-data inspection.

The plain TypeScript owner imports only the contract. Filesystem APIs appear only in `🟦️typescript/📂️module-resolution/🟦️.ts:1-4`, which uses `existsSync`, `readFileSync`, `statSync`, `dirname`, and `resolve` around the pure TypeScript extractor. This preserves the injected resolver of the pure extractor while keeping module IO separate.

The compiler fixture’s retained glue inputs now use the current ticket artifact seam: `root-script-compiler/🟦️.ts:14-16` selects `SEMIO_TEST_ARTIFACT_DIR` or the active ticket’s generated root, and `:91-106` writes only below that root. My direct compiler execution set `SEMIO_TEST_ARTIFACT_DIR` to this audit’s scratch; it did not create inputs below the historical ticket.

## Taxonomy and routing

`🔣️taxonomy.json:4755-4826` declares the field-discovery parent and its eight exact semantic child contexts. `🧪️tests/🧱️root-schema-field-source/🟦️.ts:21-80` validates the fixture with AJV, all contexts, direct-consumer bindings, the no-root-owner/no-cycle condition, the Nx command, package script, launch seed, and generated launch entry.

The registered route is `@semio-tech/repo-lib:test-root-schema-field-source`, which runs `bun ./📜️script.ts test root-schema-field-source` through `📦️packages/🟦️typescript/📋️project.json:115-123` and the package router at `📜️script.ts:123-127`. The route, seed, and derived launch entry are checked by the portable test. The root compiler target remains separately registered at project lines `316-321`.

## Executed evidence

| Invocation | Result | Evidence scope |
| --- | --- | --- |
| Direct `bun ./📜️script.ts test root-schema-field-source` | Pass — 4/4, 67 assertions, 3.92s | AJV fixture, 8 owners/19 declarations/9 contexts, consumer bindings, no root declaration/cycle, package/Nx/launch registration |
| Direct `SEMIO_TEST_ARTIFACT_DIR=<audit scratch> bun ./📜️script.ts test root-script-compiler` | Pass — 6/6, 86 assertions, 4.91s | Bun and esbuild compilation, owner/declaration source-as-data records, direct runtime contract/Rust/Protobuf extraction, current-ticket glue inputs, Nx/launch registration |
| Direct `testArtifactFieldParityOracle()` | Pass — 21.9s | 16 alias/module graphs against TypeScript, 3 GraphQL metadata cases, 192 fast-glob owners, and AJV exact-record missing/extra parity |
| Sol isolated `bun nx run @semio-tech/repo-lib:test-root-schema-field-source --skip-nx-cache` | Pass — 4/4, 67 assertions | Actual registered Nx target with private ticket-local workspace/cache/TMP/artifact roots |
| Sol isolated `bun nx run @semio-tech/repo-lib:test-root-script-compiler --skip-nx-cache` | Pass — 6/6, 86 assertions | Actual registered compiler route with the current-ticket artifact seam |
| Sol isolated `bun nx run workspace:artifact-field-parity-test --skip-nx-cache` | Pass | Actual registered parity route with the TypeScript/GraphQL/glob/AJV oracle set |
| Sol scoped field-discovery inventory | Pass — 0 violations, 0 unresolved | Exact new taxonomy contexts |

I started one global registered Nx source-route attempt while Sol’s same route was already constructing a project graph. It remained in Nx project-graph setup without starting a test body, so I terminated only that redundant process tree and do not use it as evidence. The direct run above and Sol’s isolated registered run are the audit evidence.

## Limits

This audit covers only schema-field extraction ownership and its direct consumers. It does not accept or reject unrelated root policy implementations still imported by field parity. The source-as-data fixture intentionally verifies named declarations and topology rather than freezing implementation bytes.

Disposable audit logs and retained test inputs are confined to `🗑️generated/terra-root-schema-field-audit` and will be removed after this report is retained.
