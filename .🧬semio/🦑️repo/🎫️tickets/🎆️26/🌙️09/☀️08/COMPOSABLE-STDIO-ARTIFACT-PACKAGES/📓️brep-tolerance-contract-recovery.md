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

`🧫️fixtures/📏️tolerance/🔣️.json` defines 12 language-neutral cases:

- valid nonzero and invalid missing tolerance for all three create mutations;
- valid nonzero numeric and invalid string tolerance for all three sparse entity diffs.

The Rust decoder consumes the same cases. The existing third-party `jsonschema@1.5.0` library validates them through the permanent generator command `tolerance-contract`.

## Validation

- Direct `rustfmt --edition 2021 --config skip_children=true` over the 26 exact Rust files completed successfully. `cargo fmt` was never run.
- A read-only Bun census parsed all 65 centralized mutation fixture documents. It inspected 106 vertex-shaped, 103 edge-shaped, and 25 face-shaped records and found no missing, nonnumeric, nonfinite, or zero tolerance. Receipt: `🗑️generated/stdio-brep-tolerance-fixture-census.txt`.
- All 72 JSON documents in the exact touched-path ledger parsed successfully. Receipt: `🗑️generated/stdio-brep-tolerance-json-parse.txt`.
- `git diff --check` over all 112 exact touched paths completed cleanly. Receipt: `🗑️generated/stdio-brep-tolerance-diff-check.txt`.
- `bun x nx run @semio-tech/fixture-generator-stdio-semio-v1-brep:📏️tolerance-contract --output-style=stream` passed all 12/12 third-party schema cases. Receipt: `🗑️generated/stdio-brep-tolerance-schema-oracle.txt`.
- Sequential fresh-state `@semio-tech/stdio-semio:check` and `@semio-tech/stdio-semio:test` attempts stopped during Nx project graph construction before either TypeScript target ran. The contemporaneous blockers were relocated Playbook/Note document-contract fixture imports and `Source project does not exist: npm:@asamuzakjp/css-color`. Receipts: `🗑️generated/stdio-semio-typescript-brep-check-final.txt` and `🗑️generated/stdio-semio-typescript-brep-test-final.txt`. These receipts are failures, not TypeScript validation passes.
- The focused native gate used `@semio-tech/stdio-semio-rs:test` with `-E 'test(/brep::schema::(mutations|diff)::/)'`, the shared ticket Cargo target, two build jobs, disabled incrementality, offline Cargo, unlimited build budget, and private Nx state/cache. Its first attempt stopped during Nx project graph construction before Cargo ran because a Note document-contract import and the missing npm project remained in the graph. Receipt: `🗑️generated/stdio-semio-brep-tolerance-native-final.txt`. After those JSON import repairs, one fresh retry stopped before Cargo because root `📜️script.ts` required a missing replication Map test module and `npm:@asamuzakjp/css-color` remained absent from the graph. Retry receipt: `🗑️generated/stdio-semio-brep-tolerance-native-retry.txt`. These receipts are graph failures, not native validation passes.

The exact source ledger is in `📓️brep-tolerance-touched-path-ledger.md`.
