# BREP Tolerance Contract Recovery

## Scope

This repair is limited to the persisted BREP vertex, edge, and face tolerance contract; its typed create payloads; sparse diffs; delete inverses; committed mutation fixtures; and the exact downstream Rust payload literals. The previously audited 15 geometry, topology, and engine failures remain a separate concern. This repair did not change their dependencies, feature selection, or algorithms.

## Result

The snapshot model now treats `tol` as required in Rust as well as JSON Schema for every persisted vertex, edge, and face. The three `CreateVertex`, `CreateEdge`, and `CreateFace` payloads carry an explicit `f64` tolerance across Rust, JSON Schema, TypeScript, Protobuf, GraphQL, and the three text grammar mirrors. Text and binary mutation codecs require and preserve that value.

Create diffs use the supplied tolerance. Delete inverses recapture the deleted vertex, edge, or face tolerance, including every edge cascaded by vertex deletion. No inverse recreates these records with `0.0`.

`BrepVertexDiff`, `BrepEdgeDiff`, and `BrepFaceDiff` now expose sparse optional tolerance fields. Apply, between, inverse, absorb, text encode/decode, and all schema mirrors preserve a nonzero tolerance. The shared snapshot record codecs remain the single implementation used by snapshot and diff. Their six helpers are `pub(in super::super)`, which gives the sibling `schema::diff` module access while keeping them inside the schema boundary.

All 65 committed centralized mutation JSON documents now contain explicit nonzero tolerance values. Existing records use `1e-7`; newly created vertex, edge, and face records use `2e-7`, `3e-7`, and `4e-7` respectively. Mutation payloads and expected added diff records use the matching value. No fixture was weakened to zero.

The stale fixture test aggregator left by fixture centralization was repaired to include the 13 current semantic Rust test modules. The old hashed fixture module paths are no longer referenced by that aggregator.

## Language-neutral contract and oracle

`🧫️fixtures/📏️tolerance/🔣️.json` now defines 27 language-neutral cases plus one source census check:

- valid nonzero and invalid missing tolerance for all three create mutations;
- valid nonzero numeric and invalid string tolerance for all three sparse entity diffs.
- distinct nonzero delete/inverse preservation for a vertex and its two cascaded edges, one edge, and one face;
- rejected one-, three-, and four-field vertex, edge, and face sparse diff records in both text and binary frames;
- rejected text create mutations that omit `tol`;
- the authoritative required-field map for each flattened Protobuf/GraphQL create discriminant;
- a recursive census of all Rust `CreateVertex`, `CreateEdge`, and `CreateFace` literals in BREP and its Base consumer.

The Rust test source consumes the same cases through the production inverse, text, binary, and JSON decoders. The existing third-party `jsonschema@1.5.0` library validates the payloads while the TypeScript oracle independently derives inverse records, decodes binary segment framing, measures sparse-record arity, checks the authoritative text grammar, checks both flattened transport declarations, and performs the source-literal census through the permanent generator command `tolerance-contract`.

The 65 committed mutation documents and their native fixture tests already proved exact inverse reapplication and asserted restored tolerance for delete vertex, its edge cascade, delete edge, and delete face. Their persisted records all use `1e-7`, so they could not expose a tolerance copied from the wrong entity. The new inverse cases use five distinct nonzero values and reuse the existing inverse law rather than duplicating its implementation. The existing codec law remains the positive round-trip proof; the six new cases add the missing old-arity rejection boundary.

## Flattened transport limitation

The Protobuf and GraphQL mutation mirrors flatten all 13 variants into one transport record. `tol` must therefore remain `optional double` in Protobuf and nullable `Float` in GraphQL because delete and non-create variants do not carry it. Those declarations cannot express the conditional rule by themselves. Consumers must select the authoritative create payload schema from `CREATE_VERTEX`, `CREATE_EDGE`, or `CREATE_FACE` and enforce that schema's required fields. The shared transport map and oracle verify the discriminants, mirror field availability, and this conditional requirement without redesigning the mirrors.

## Validation

- Direct `rustfmt --edition 2021 --config skip_children=true` over the original 26 exact Rust files and the updated mutation unit file completed successfully. `cargo fmt` was never run.
- A read-only Bun census parsed all 65 centralized mutation fixture documents. It inspected 106 vertex-shaped, 103 edge-shaped, and 25 face-shaped records and found no missing, nonnumeric, nonfinite, or zero tolerance. Receipt: `🗑️generated/stdio-brep-tolerance-fixture-census.txt`.
- All 72 JSON documents in the exact touched-path ledger parsed successfully. Receipt: `🗑️generated/stdio-brep-tolerance-json-parse.txt`.
- `git diff --check` over all 112 exact touched paths completed cleanly. Receipt: `🗑️generated/stdio-brep-tolerance-diff-check.txt`.
- `bun x nx run @semio-tech/fixture-generator-stdio-semio-v1-brep:📏️tolerance-contract --output-style=stream` first passed 12/12 schema cases, then passed all expanded 28/28 tolerance checks after the audit follow-up. Receipts: `🗑️generated/stdio-brep-tolerance-schema-oracle.txt` and `🗑️generated/stdio-brep-tolerance-schema-oracle-followup-retry.txt`.
- The audit follow-up fixture parsed successfully and its five changed source paths pass `git diff --check`. Receipts: `🗑️generated/stdio-brep-tolerance-followup-json-parse.txt` and `🗑️generated/stdio-brep-tolerance-followup-diff-check.txt`.
- The final ordinary `@semio-tech/stdio-semio:check` and `@semio-tech/stdio-semio:test` targets both passed after shared graph repair. Receipts: `🗑️generated/stdio-semio-ts-final-check.txt`, `🗑️generated/stdio-semio-ts-final-test.txt`, and `🗑️generated/stdio-semio-ts-final.tsv`. The audit follow-up changed only the neutral fixture, generator oracle, Rust tests, and comments in transport mirrors, so these package targets were not repeated.
- The focused native gate used `@semio-tech/stdio-semio-rs:test` with `-E 'test(/brep::schema::(mutations|diff)::/)'`, the shared ticket Cargo target, two build jobs, disabled incrementality, offline Cargo, unlimited build budget, and private Nx state/cache. Its first attempt stopped during Nx project graph construction before Cargo ran because a Note document-contract import and the missing npm project remained in the graph. Receipt: `🗑️generated/stdio-semio-brep-tolerance-native-final.txt`. After those JSON import repairs, one fresh retry stopped before Cargo because root `📜️script.ts` required a missing replication Map test module and `npm:@asamuzakjp/css-color` remained absent from the graph. Retry receipt: `🗑️generated/stdio-semio-brep-tolerance-native-retry.txt`. These receipts are graph failures, not native validation passes.

The audit-follow-up native acceptance remains pending until the active Norm Semio compilation releases the shared Cargo lane. The exact handoff command is:

```sh
CARGO_TARGET_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️08/COMPOSABLE-STDIO-ARTIFACT-PACKAGES/🗑️generated/cargo' \
CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0 CARGO_NET_OFFLINE=true SEMIO_BUILD_BUDGET_MS=0 \
SEMIO_TEST_ARTIFACT_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️08/COMPOSABLE-STDIO-ARTIFACT-PACKAGES/🗑️generated/nextest-brep-tolerance-audit-followup' \
NX_DAEMON=false NX_ISOLATE_PLUGINS=false NX_NO_CLOUD=true \
NX_WORKSPACE_DATA_DIRECTORY='.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️08/COMPOSABLE-STDIO-ARTIFACT-PACKAGES/🗑️generated/nx-workspace-data/brep-native-tolerance-audit-followup' \
NX_CACHE_DIRECTORY='.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️08/COMPOSABLE-STDIO-ARTIFACT-PACKAGES/🗑️generated/nx-cache/brep-native-tolerance-audit-followup' \
bun x nx run @semio-tech/stdio-semio-rs:test --output-style=stream -- -E 'test(/brep::schema::(mutations|diff)::/)'
```

The exact source ledger is in `📓️brep-tolerance-touched-path-ledger.md`.
