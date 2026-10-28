# IFC 4x3 Editing Implementation

## Implemented

The new TypeScript IFC4x3 editing modules consume the shared IFC Part21 graph with exact bigint identities and binary64 words. The IFC4x3 catalog remains a separate normative schema layer. No third-party runtime dependency was added.

Commands handcraft sparse header/entity/argument diffs. Nested typed or aggregate values and complex constituents can be edited. Central application validates both input and result, checks identity/index coordinates and disjoint targets, refuses dangling references, invalid scalar domains, cycles and depth/value/entity budgets, and publishes no partial result. Synchronous and cooperatively scheduled asynchronous entry points share one generator authority; async supports progress and AbortSignal cancellation.

Inverse diffs transport positional targets and retain original entity positions. Sequential diff composition transports index coordinates, merges sparse modifications, edits inserted records and cancels inserted-then-removed records. The local history records revision-checked mutation/undo/redo events and explicit persisted-local or persisted-shared scope. The central transaction computes forward state and inverse once; async creation/read/execute/undo/redo prepare owned results and recheck cancellation/revision immediately before atomic publication. Sharing transport and persistence remain responsibilities of the existing host.

Schema rule diagnostics are intentionally separate from editing validity: generic graph editing alone does not claim EXPRESS WHERE/UNIQUE/inverse or any MVD conformance.

## Verification

- Test-first shared feature and JSON vectors were written before the executable modules; initial focused run failed for missing diff module.
- 31 focused Bun tests, 244 assertions pass, including independent fast-json-patch output, mixed positional edits, exact u64 entity replacement, nested/complex undo, atomic refusals, cancellation, event history, every four-argument removal mask, sequential composition, annihilation, exact BINARY padding, absent optional diff lanes and independent Ajv validation of decimal mutation/diff transports.
- Focused strict TypeScript check passes.
- A concurrent external change added original index records to IFC entity removals. New code consumes the current shared type directly and validates the original index rather than introducing compatibility handling.

## Execution Routes

The cached repo domain CLI does not execute general scripts. Its test invocation returned success with no test output, so that result is not counted as verification. Focused Bun and TypeScript runtime commands were necessary fallbacks; no build infrastructure was changed.

## Remaining Integration

The public IFC4x3 facade and schema/editing/STEP/GLTF/BIM/FEM/oracle suites are registered. An integration run of schema/editing/STEP/GLTF has 42 passing tests and 208 assertions; strict facade type checking passes before adding BIM/FEM. With BIM/FEM exports, the checker reaches an unrelated missing `@semio-tech/framework-interaction` module through existing FEM dependencies. No build-system repair was made. Official IFC edit/export/reimport is exercised by the GLTF suite. The actual Rust host received an independent audit and source fixes; see host-audit-fixes.md. Full semantic/MVD rule execution and all geometry/exchange representations still require explicit evidence.


## Core Review Follow-up

`core-fixes.md` records the resolved wide-frontier, public command/history cancellation and mutable-result alias findings. Lazy admission refuses a 100,000-child aggregate under `maxValues:3` with zero child reads. All mutable public results own their branches; no native `structuredClone` remains in the editing core. Async progress/cancellation covers commands, inverses, composition/heap sorting, history construction, snapshot/event reads and execution/undo/redo. History creation uses `IfcEditHistory.create` or explicit `createSync`, with a private constructor. The focused strict TypeScript check passes, and retained independent review reproduction passes 1,864 coordinate law cases. This verification does not imply semantic EXPRESS/MVD or whole-host build conformance.


## Owned Capture Re-audit Follow-up

The subsequent P1 caller-mutation race and P2 base-free coordinate admission findings are resolved in `core-ownership-fixes.md`. Caller data is now incrementally captured into bounded private ownership before structural admission; application, inverse construction and event recording use only that authority. Captured command recording agrees with its replay diff. Accepted results remain structurally valid under progress-callback mutation of original inputs, and failed capture/cancellation leaves history unchanged. The optional shared `complex` field now admits omission in the IFC4x3 JSON Schema and canonicalizes to an empty collection in owned graphs. The editing suite has 31 passing tests and 244 assertions; focused strict TypeScript, the retained 1,864-law reproduction and the new ownership reproduction all pass. The capture contract promises a validated owned result; exact start-of-call capture requires caller inputs to remain unchanged until completion.

## Native Shape Review Follow-up

The final runtime shape findings are resolved in core-shape-fixes.md. Native owned captures now validate required fields, discriminants, containers, allowed fields and coordinate domains against the checked-in JSON Schemas through a bounded dependency-free interpreter before planning/publication. Differential Ajv cases cover all eleven mutation branches and header/entity/list/value lanes, including missing/undefined/null/wrong-type fields, extras, holes and nonfinite coordinates across synchronous and asynchronous routes. Refused commands publish no history event and preserve sources; published commands satisfy their native and serialized schema. The combined editing suites now pass 35 tests and 5,126 assertions; focused strict TypeScript and the final, ownership and 1,864-law reproductions pass. Graph identity uses IFC4X3_GRAPH_SCHEMA (stdio.ifc), distinct from the host envelope IFC4X3_DOCUMENT_SCHEMA. No new facade API is required.

