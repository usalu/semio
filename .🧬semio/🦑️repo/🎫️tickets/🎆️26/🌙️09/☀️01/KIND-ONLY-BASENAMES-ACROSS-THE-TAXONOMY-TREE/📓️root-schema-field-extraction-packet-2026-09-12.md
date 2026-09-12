# Root Schema Field Extraction Packet

This bounds a later Sol Extra High source lane independently of root inventory/mutation workflow. Root read the actual field extractors and their native tests on 2026-09-12. The native TypeScript dependency graph isolates nineteen non-import declarations and403 declaration lines from seven explicit API seeds; exact rows are in generated/coordinator/root-schema-extraction-closure.json. Re-read current source before edits. No Git, AGENTS, ticket/goal lifecycle or unrelated schema behavior changes.

## Actual Concern

The root script still defines the common field shape/cardinality/extraction contracts, scalar and state normalization, declared-type selection, field extraction from Rust/TypeScript/GraphQL/JSON Schema/Protobuf, file-backed TypeScript module resolution and missing/extra field comparison. These are reusable schema discovery/equivalence implementations, not command routing.

Separate the common domain contract and exact representation-specific extraction concerns into anonymous implementation leaves under repository schema discovery/ownership. A format identity here describes the input schema representation; it must not become an implementation-language package bucket. Keep source traversal/IO distinct from pure text extraction where the existing injected resolveModule seam allows it. Reuse existing owned parser/field interfaces if semantically identical; do not introduce external parser/compiler runtime dependencies.

The nineteen current local declarations are:
PolicySchemaFieldCardinality, PolicySchemaFieldShape, PolicySchemaLeafExtract;
policyCanonicalState, policyCanonicalScalar, policyFindSchemaDeclaration;
policyParseRustFieldType, policyExtractRustSchemaFields;
policyParseTsFieldType, policyExtractTypescriptSchemaFields, policyExtractTypescriptSchemaFile;
policyExtractGraphqlSchemaFields;
policyParseJsonSchemaProperty, policyJsonSchemaScalar, policyExtractJsonSchemaFields;
policyParseProtoFieldType, policyExtractProtobufSchemaFields;
policySchemaFieldDifferences and policySnakeToCamel.

Preserve exact declared-name selection, named/local/re-export alias traversal, cycle refusal, inherited field metadata, state/scalar/cardinality values, source module identity and absence behavior. The root's actual artifact/app/inference laws consume these APIs; leave their policy behavior for its own lane while importing the canonical new owners directly. Do not retain compatibility exports in the root script.

## Consumers And Executed Prerequisite

Root's search found two direct implementation consumers outside command sources:

1. library/📏️ownership/🧪️tests/🪪️field-parity/🟦️.ts imports extractors and schema policy APIs from root. Its existing portable fixture lives at ownership/🧫️fixtures/🪪️field-parity/🔣️.json. Root executed testArtifactFieldParityOracle directly: it passed, discovering192 standard/subset owners against fast-glob, matching sixteen alias/module graphs against the installed TypeScript checker, matching three GraphQL metadata cases against equivalent TypeScript AST fields, and comparing missing/extra reports against AJV. Console evidence is generated/coordinator/root-schema-field-prerequisite.log. This is a passing oracle invocation, not the full field-policy enforcement route.
2. library/🧪️tests/⚙️root-script-compiler/🟦️.ts parses root source declarations and compiles policySnakeToCamel through Bun and esbuild, then imports root's Rust/Protobuf extractors to verify actual use. Its portable fixture names expected root declarations and eager vocabulary. Rebase these source-as-data checks to explicit canonical owner/declaration records while preserving one effective implementation, both compiler outputs and actual extractor invocation. Do not leave a duplicate root function solely for a test or weaken the expected names.

The compiler fixture's glue discovery test currently creates retained inputs under an older normalization ticket and has no artifact-root override in the read source. Do not execute it unchanged for this task. Provide the appropriate current-ticket output/input ownership seam while preserving all historical authored fixtures. No older ticket cleanup is authorized by this source move.

Registered routes are workspace:artifact-field-parity-test/report/enforce and @semio-tech/repo-lib:test-root-script-compiler, with seed/derived launch entries. The field test route is separate from enforcing every current repository schema breach. Rebase direct/dynamic root consumers and source-as-data authority across the full live tree; the two confirmed files are not an exhaustive name-only migration plan.

## Verification

Add a portable ownership/consumer fixture before extraction and validate with installed native TypeScript AST. Retain the existing language-neutral field vectors and their independent TypeScript/AJV/glob/esbuild oracles. Run the focused direct and registered tests and strict/scoped taxonomy for every new ancestor context. Do not freeze source-body SHA values or add native library types to public runtime interfaces. Record exact changed paths, commands, outcomes and current limits in 📓️sol-root-schema-field-extraction-2026-09-12.md. Own scratch stays below generated/sol-root-schema-field-extraction and is removed when complete.
