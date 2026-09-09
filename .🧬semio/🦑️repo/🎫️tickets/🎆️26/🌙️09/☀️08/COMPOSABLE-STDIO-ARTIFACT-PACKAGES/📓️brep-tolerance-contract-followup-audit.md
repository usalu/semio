# BREP Tolerance Contract Follow-up Audit

## Scope

Read-only review of the post-audit BREP tolerance follow-up recorded in `📓️post-repair-brep-viewer-audit.md`, constrained to the five paths named by `📓️brep-tolerance-touched-path-ledger.md`:

- `🧊️brep/🏗️generator/📜️script.ts`
- `🧊️brep/🧫️fixtures/📏️tolerance/🔣️.json`
- `🧊️brep/🧬️schema/🧬️mutations/🔗️.graphql`
- `🧊️brep/🧬️schema/🧬️mutations/🛰️.proto`
- `🧊️brep/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs`

The separate geometry-15, Value-algebra-2, Flow temporary-host retirement, PDF-cache, and default-library work was not examined. No source, Git state, formatter, build, or Nx command was changed or run by this audit.

## Finding

No residual actionable BREP follow-up defect was found in the five-path scope.

The shared fixture now gives the two consumers a meaningful division of responsibility without weakening the earlier schema cases:

| Contract boundary | Fixture evidence | Third-party consumer | Native consumer |
| --- | --- | --- | --- |
| Typed creates and sparse diffs | 6 create and 6 diff cases retain valid numeric and rejected missing/string `tol` coverage | `jsonschema` validates each authoritative JSON Schema | Production JSON mutation/diff decoders are called directly |
| Delete inverse preservation | 3 delete cases: vertex plus two cascaded edges, edge, and face; the five restored values are `2.1e-7`, `3.1e-7`, `3.2e-7`, `3.3e-7`, and `4.4e-7` | A structural oracle derives the expected create operations from the before snapshot and validates each with `jsonschema` | Production `inverse`, `diff`, and `apply` must equal the expected operations and restore the snapshot |
| Old sparse-diff arities | 3 text and 3 binary rejection cases: vertex `1/2`, edge `3/4`, face `4/5` fields | Independently reads the binary LEB128 frame and top-level fields | Production `parse_diff` and `decode_diff` are called directly |
| Omitted text-create tolerance | 3 rejected text cases, one for each create variant | Confirms the declared grammar requires `tol` | Production `parse_op` is called directly |
| Flattened transport requiredness | 3 discriminant maps for `CREATE_VERTEX`, `CREATE_EDGE`, and `CREATE_FACE` | Compares map fields with each authoritative create schema and verifies both transport mirrors contain the fields and discriminants | Confirms complete payloads decode to the matching Rust mutation variant, while incomplete payloads reject |
| Caller literals | One BREP/Base census declaration | Recursively enumerates every `.rs` source below both roots and requires `tol` on each matched create literal | Not duplicated; this is a source-contract check that protects native callers before compilation |

The generator's 28-check total is correctly calculated from `6 + 6 + 3 + 6 + 3 + 3 + 1`; `$schema` is metadata and not a check. Its recorded ordinary Nx receipt, `🗑️generated/stdio-brep-tolerance-schema-oracle-followup-retry.txt`, reports every category above and `28/28`, including a source-literal census of `CreateVertex=6`, `CreateEdge=4`, and `CreateFace=3`.

The follow-up does not make the tests tautological. The TypeScript side derives inverse data and decodes binary framing without calling the Rust codec; Rust then exercises the production inverse, JSON decoder, text parser, binary decoder, and diff/application path against the same independent fixture. The text oracle is intentionally narrower: it verifies the missing-`tol` condition by checking the grammar declaration, while Rust supplies the actual parser rejection.

## Flattened Transport Limitation

The limitation is documented honestly in both transport mirrors and in `📓️brep-tolerance-contract-recovery.md`. Because all 13 variants share one flattened record, `tol` remains `optional double` in Protobuf and nullable `Float` in GraphQL. Those declarations cannot encode conditional requiredness. The fixture therefore maps each create discriminant to its authoritative payload schema and tests the map; consumers still must enforce that selected create schema.

## Viewer Fixture Follow-up

The current source comparison finds no remaining omitted forwarding hook: `ArtifactViewer` declares 44 methods and `BoundedViewerFixture<V>` implements the same 44, with no missing or extra method names. It forwards all three constants (`ROLE`, `DIALECT`, `DOCUMENT_SCHEMA`) and all nine associated types. Each regular hook delegates to `V`; only the bounded fixture's documented owner/disposer and no-presence/no-transient fallbacks use `or_else`, preserving the fixture's bounded authority.

This is source evidence only. The focused lifecycle runtime regression remains pending in the separate Norm-owned native lane.

## Validation Status

The third-party oracle has recorded a passing Nx execution. The native focused BREP gate has not yet reached Cargo: its two recorded attempts stopped in Nx project-graph construction, first on a Note fixture and missing npm project, then on the replication Map test import and the same missing npm project. It must be rerun after the shared Norm Cargo lane and graph repair are available; this audit does not treat it as passed.
