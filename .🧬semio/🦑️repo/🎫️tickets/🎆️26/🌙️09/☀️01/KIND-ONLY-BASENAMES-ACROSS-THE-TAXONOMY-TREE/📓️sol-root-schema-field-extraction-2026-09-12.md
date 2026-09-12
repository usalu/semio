# Root Schema Field Ownership Extraction

**Status: complete and independently accepted.** Nineteen schema-field declarations formerly embedded in root `📜️script.ts` now belong to eight semantic directories. Every implementation is an anonymous `🟦️.ts` leaf. The directory tree describes the schema representation or operation, so another implementation language can replace a leaf without renaming its domain owner.

## Owner inventory

All paths are relative to the repository root.

| Concern | Owner kind | Anonymous leaf | Declarations | Lines |
| --- | --- | --- | --- | ---: |
| shared field contract and canonicalization | `repo-schema-field-contract` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🔍️field-discovery/🧱️contract/🟦️.ts` | `PolicySchemaFieldCardinality`, `PolicySchemaFieldShape`, `PolicySchemaLeafExtract`, `policyCanonicalState`, `policyCanonicalScalar`, `policyFindSchemaDeclaration`, `policySnakeToCamel` | 56 |
| Rust representation | `repo-schema-field-rust-representation` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🔍️field-discovery/🦀️rust/🟦️.ts` | `policyParseRustFieldType`, `policyExtractRustSchemaFields` | 54 |
| pure TypeScript representation | `repo-schema-field-typescript-representation` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🔍️field-discovery/🟦️typescript/🟦️.ts` | `policyParseTsFieldType`, `policyExtractTypescriptSchemaFields` | 94 |
| TypeScript module filesystem resolution | `repo-schema-field-typescript-module-resolution` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🔍️field-discovery/🟦️typescript/📂️module-resolution/🟦️.ts` | `policyExtractTypescriptSchemaFile` | 13 |
| GraphQL representation | `repo-schema-field-graphql-representation` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🔍️field-discovery/🔗️graphql/🟦️.ts` | `policyExtractGraphqlSchemaFields` | 43 |
| JSON Schema representation | `repo-schema-field-json-schema-representation` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🔍️field-discovery/🔣️json-schema/🟦️.ts` | `policyJsonSchemaScalar`, `policyParseJsonSchemaProperty`, `policyExtractJsonSchemaFields` | 56 |
| Protobuf representation | `repo-schema-field-protobuf-representation` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🔍️field-discovery/🛰️protobuf/🟦️.ts` | `policyParseProtoFieldType`, `policyExtractProtobufSchemaFields` | 31 |
| representation comparison | `repo-schema-field-comparison` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🔍️field-discovery/⚖️comparison/🟦️.ts` | `policySchemaFieldDifferences` | 5 |

`repo-schema-field-discovery` is the shared parent below the existing `schema` kind. The taxonomy registers it and the eight child contexts with exact parent constraints. The final owner graph has eight nodes and seven owner-to-owner edges. It is acyclic and has no edge back to root.

## Behavior and consumer closure

The pure TypeScript extractor retains an injected module resolver. Filesystem traversal lives in the separate `📂️module-resolution` owner and uses only Node system modules. Named exports, local aliases, imported aliases, re-export aliases, interface inheritance, declaration-name selection, and cycle refusal remain in the pure owner. The Rust, GraphQL, JSON Schema, and Protobuf owners retain field name, optionality, cardinality, scalar, state, and declared source-type behavior.

Root `📜️script.ts` imports all eight owners directly for its live policies. It declares and named-re-exports none of the nineteen moved bindings, so it remains an executable consumer rather than a compatibility facade.

The field-parity oracle imports Rust, TypeScript, GraphQL, and comparison behavior from the extracted owners. Its remaining root imports are the three separate artifact policy entry points that were outside this packet.

The root-script compiler fixture records the exact eight owner paths and their nineteen declaration names. It parses those owner files as source data, executes field naming from the contract owner through Bun and esbuild, and imports live Rust and Protobuf behavior from their owners. Its prior dynamic import of the moved root bindings is gone. The compiler also reads `POLICY_TS_COMPONENT_LEAF` and `POLICY_RS_COMPONENT_LEAF_NAME` from their existing mutation-identity owner rather than expecting stale root declarations.

The compiler fixture now retains generated glue inputs under `SEMIO_TEST_ARTIFACT_DIR` or, by default, this ticket's `🗑️generated/sol-root-schema-field-extraction/root-script-compiler` directory. It no longer names or writes into the August 17 normalization ticket. Its expected vocabulary and launch record were updated to the current kind-only leaves and current registered launch.

## Portable contract and route

The language-neutral schema and fixture are:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🧱️root-schema-field-source/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🧱️root-schema-field-source/🔣️.json`

The anonymous test leaf `🧪️tests/🧱️root-schema-field-source/🟦️.ts` validates the fixture with AJV and checks eight owners, nineteen declarations, nine exact semantic contexts, three consumer groups, anonymous leaf basenames, root removal, direct imports, owner graph acyclicity, and executable registration.

The route is `@semio-tech/repo-lib:test-root-schema-field-source`, implemented by `bun ./📜️script.ts test root-schema-field-source`. It is present in the package script, Nx project, Bun router, launch seed, and derived launch catalog under `🧹clean🧩️taxonomy🧪️root-schema-field-source`.

## Test-driven repair evidence

The first portable execution passed schema, consumer, and route checks but failed its semantic-context case because the new test called the established `semanticDirectoryKindId(name, taxonomy, context)` API in the wrong argument order. Correcting the test produced 4/4 passing cases and 67 assertions.

The first compiler execution passed field execution and complete-root compilation but exposed two stale source-as-data assumptions: two eager constants had already moved to mutation identity, and the fixture expected an obsolete launch name. Rebasing both records to the actual owners and current kind-only launch made the compiler gate pass 6/6 cases and 86 assertions.

## Verification

All mutable outputs used `🗑️generated/sol-root-schema-field-extraction` through dedicated `SEMIO_TEST_ARTIFACT_DIR`, `TMPDIR`, `NX_WORKSPACE_DATA_DIRECTORY`, and `NX_CACHE_DIRECTORY` values.

| Check | Result |
| --- | --- |
| JSON and JSONC parse of taxonomy, schemas, fixtures, project/package manifests, and launch catalogs | green |
| Bun build of all eight extracted owners | 8 entry points built |
| direct portable ownership test | 4/4, 67 assertions |
| Bun package router for `test root-schema-field-source` | 4/4, 67 assertions |
| direct root-script compiler | 6/6, 86 assertions |
| direct artifact field-parity oracle | green: 16 TypeScript alias/module graphs, 3 GraphQL metadata cases, 192 fast-glob owners, TypeScript AST and AJV parity |
| strict scoped inventory of `🧬️schema/🔍️field-discovery` | zero violations, zero unresolved paths |
| root runtime import after extraction | green |
| AST check of root moved declarations and named re-exports | zero declarations, zero named re-exports |
| `git diff --check` over every touched product/configuration path | green |
| isolated Nx `@semio-tech/repo-lib:test-root-schema-field-source --skip-nx-cache` | 4/4, 67 assertions |
| isolated Nx `@semio-tech/repo-lib:test-root-script-compiler --skip-nx-cache` | 6/6, 86 assertions |
| isolated Nx `workspace:artifact-field-parity-test --skip-nx-cache` | green with the full TypeScript/AJV/fast-glob oracle set |
| independent Terra acceptance audit | accepted: direct source 4/4 and 67 assertions, compiler 6/6 and 86 assertions, field parity green, eight nodes/seven edges/zero cycles/zero root back edges |

The field extractors remain deliberately textual and representation-specific; the existing third-party oracles establish their behavior for the registered vectors rather than claiming a complete language parser. The enforcement route for every repository field breach was outside this extraction and was not run. No live cleanup, scaffold, apply, lifecycle, Git, worktree, AGENTS, generated schema entity, or native-runtime mutation was performed.

## Exact mutation attribution

This pass owns the following changes:

- root `📜️script.ts`: eight direct field-owner imports and removal of the nineteen local declarations only;
- library `🔣️taxonomy.json`: the discovery parent and eight exact schema-field child kinds only;
- the eight owner leaves listed above;
- the portable schema, fixture, and test listed above;
- `📏️ownership/🧪️tests/🪪️field-parity/🟦️.ts`: four direct owner imports only;
- `🧪️tests/⚙️root-script-compiler/🟦️.ts` and `🧫️fixtures/⚙️root-script-compiler/🔣️.json`: owner-based source/runtime bindings, current ticket artifact seam, current eager vocabulary source, and current launch expectation;
- library TypeScript package `📜️script.ts`, `📋️project.json`, and `package.json`: the schema-field source route only;
- `.vscode/🧩️launch.seed.jsonc` and `.vscode/launch.json`: the one schema-field source launch entry only;
- this report.

The shared root, taxonomy, package router, and launch catalogs contain concurrent work. In particular, the kind-only-basename budget edit in the package router belongs to the coordinator, and print taxonomy/discovery/normalization changes belong to the print lane. This report does not attribute those hunks.
