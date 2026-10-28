# Static SELECT Parent Index Independent Audit

Read-only source checkpoint of select-parents.rs, metadata::select_parents, the cold evaluator.rs TYPEOF use, retained machine-population.rs stages 4–8, and new machine-typeof-tests.rs. No new confirmed defect. Production was not edited by this auditor.

## Table parity and bounded lookup

Ticket select-parent-index-audit.ts independently parses the Rust table, rebuilds direct reverse edges from the pinned catalog JSON, checks strict key order, duplicate key/cardinality, every parent slice and the published digest. Actual generated/select-parent-index-audit.json reports 214 keys, 271 edges, 61 SELECTs, maximum key length 45 bytes, maximum seven parents and zero discrepancies. Digest is 778efef13d7cc4248687d82c7a34f45d77aa183b29b93630604cd7142a7b02bb. The original generator's read-only --check also passed after its outer documentation correction. This is source-table parity, separate from the parent's independent IfcOpenShell/NumPy reachability oracle.

metadata::select_parents uses a fixed 128-byte stack buffer; oversized or non-ASCII input returns an empty slice, ASCII is uppercased, and strict sorted binary lookup returns only a borrowed static parent slice. Empty/missing keys cannot access an out-of-range row. The table and nested literals are immutable static data, with no lazy container, allocation cache or result lifetime tied to the caller's temporary name. The included file now starts with outer /// documentation, avoiding the initially noted inner-doc include hazard.

## Cold caller

Cold TYPEOF keeps its existing result Vec queue and per-candidate dedup. Only schema-prefixed runtime seed strings enter lookup; primitives and foreign-schema names do not spuriously gain SELECTs. Each queue item, parent and dedup comparison ticks; accepted names use existing make/push_value and final aggregate checks. A bounded binary lookup replaces the former full declaration/member scan. Cancellation/refusal remains local to the cold call with no partial aggregate publication. These cold work/node controls do not constitute retained original-wallet custody.

## Retained caller

Stage 4 retains the original linked edge handle and resets parent position. Stage 5 reads that seed, borrows its immutable direct-parent slice and processes one parent, or takes the exhausted-parent transition. Existing TextCopy, Contains and append stages remain intact. Stage 8 advances one parent position. Newly appended SELECT edges are visited through the live linked queue; dedup bounds cycles. No new mutable container/frame field/growth or retirement path was introduced. The bounded stack normalization and binary search copy no owned payload and allocate no heap; stage 5 retains the existing fixed metadata/frame quote, while string birth/copy still belongs to existing tasks.

New authored tests compare static slices with all released entity/type memberships outside the measured allocation interval, observe lookup with zero heap birth, and check each actual retained result edge visits only its direct parent positions plus exhaustion. Existing helpers deny each positive funding axis, measure receipts and close under original control. The test's borrowed-name set is dropped before mutable close. No obvious source-level lifetime or API mismatch was observed.

Native compiler ingress still refuses before rustc. None of these authored Rust tests were type-checked or executed here; rustfmt parsing, source inspection and reference runs cannot prove actual grants/receipts/retirement. Full retained admission remains refused. Guided staging is outside this checkpoint and awaits its worker's stable producer.
