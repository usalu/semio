# Independent Native STEP Import Source Audit

Reviewed 2026-10-21. This is a read-only production-source audit; the only authored file is this audit. No confirmed P1/P2 correctness or compile defect was established by source inspection. This conclusion is not compiler or runtime validation.

## Scope And Authority

Read `native-step-receiving-plan.md`, `native-step-reading-implementation.md`, all four new production leaves, the mounted Rust tests, language-neutral feature, `step.rs` mounts, the normal `native_exchange.rs` registry, the existing `native_pack.rs` final publication wrapper, and relevant original wallet implementation. The production-source prefix below is `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/`.

- `native-step-import.rs:24-27` actually receives `TxtSnapshot` with `ArtifactPackReceiving` from the supplied Binary Pack, retains it in the paired run, and passes an original snapshot decode loan to the new reader. There is no reconstructed document or cold Part21 parse call here.
- `native_exchange.rs:186` registers `TxtIntoIfc4x3Native` through the existing original deserializer wrapper beside the native TXT export. `native_pack.rs:44-54` retains the actual imported graph and invokes `encode_exchange_pack` through the original encoder before producing Binary output. The root export production sources were not changed by this audit.
- `native-step-read.rs:71-112` receives into one actual retained `Receiving` slot and only extracts the snapshot after structural/reference validation and the final funded effect. `native-step-parse.rs:22-35` retains text, pending/frame values, lists, entity/component prefixes, graph and indexes. The source is borrowed from the parent retained TXT frame.
- `native-step-read.rs:34-64` admits work/copy/depth, capacity for allocations, and release for numeric scratch before mutation; accepted effects record progress before the original post-work `step`. The generic effect result is Copy, so it cannot carry allocated graph ownership out before that check. These facts support the intended original-owner design at source level. They do not prove cancellation cleanup or exact allocator receipts at runtime.

## Lexical And Structural Review

`native-step-lex.rs:34-59` traverses original TXT line bytes and declared endings directly and counts ignored bytes in its original position; it does not flatten source. Token decode replays a borrowed span into originally admitted text. Reviewed string escapes, print directives, nine-page API calls, X2 surrogate pairing, X4 scalar checks, literal UTF-8, positive checked u64 identities, signed i64 conversion, finite f64 conversion, binary high padding and case normalization.

`native-step-parse.rs:37-102` funds the list catalog and reference/entity extents through census passes. The iterative frame parser consumes catalog counts for vector admission. Mandatory ordered headers and exact ADD2 schema are validated before DATA completion; complex instances map the first component to the primary graph fields and retain remaining components separately. `native-step-parse.rs:360-410` uses bounded in-place heapsort and binary lookup for duplicate/unresolved identities rather than a hidden unbounded reference graph.

The fixed 65,536-byte numeric conversion ceiling intentionally differs from the TypeScript policy and is not a defect by itself. `frame` permits the enclosing entity argument frame plus the documented graph nesting, so its `> MAX_DEPTH` condition is not independently evidence of an off-by-one bug. Optional exchange sections and multiple DATA sections remain explicitly outside the declared implementation scope.

## Evidence Limits And Remaining Validation

The native code and its tests have not typechecked or executed because authentic compiler ingress rejects before rustc. This audit did not bypass that enforcement and did not execute substitute reference code as evidence for the new native path. Rustfmt syntax acceptance must not be reported as a compile pass.

`native-step-read-tests.rs:18`, `:39`, `:59`, `:87`, `:128`, `:148`, and `:200` contain substantive intended checks for exact literal graph values, original UTF-8 offsets, all five zero currencies, alphabet/control cases, official fixtures, physical receipts/cancellation, and the registered Pack route. The relative alphabet fixture include resolves to an existing file. The official corpus graph comparison calls the cold parser only as a test comparison; it does not establish native success until these Rust tests execute. Entity-count witnesses are independent but weaker than full independent graph equivalence.

Cancellation testing currently samples four completed-counter cutoffs in one substantial input. Those are useful targeted probes, not exhaustive proof that every token/list/entity/reference-index mutation boundary drains correctly. The registered route test uses its returned actual IFC Pack and gates oracle output on an environment variable; no actual native oracle output was generated by this audit. These evidence limits should remain explicit in the ticket completion claims.

No production changes, no git mutations/worktrees, no build-infrastructure changes, no shared-dependency cleanup, and no ticket or goal lifecycle changes were made. Read-only shell fallback was used under the parent audit's documented unavailable repo-command condition.
