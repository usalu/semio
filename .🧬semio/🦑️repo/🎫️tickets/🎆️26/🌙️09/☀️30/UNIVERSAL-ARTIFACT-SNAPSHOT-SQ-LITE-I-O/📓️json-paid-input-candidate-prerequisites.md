# JSON Paid Input Candidate Prerequisites

Read-only design evidence for the candidate after authentic Native20 RED,2026-10-03. No candidate implementation mounted and no Cargo run. Root's current baseline intentionally retains semantic max_value_bytes as its direct native allowance.

## Owner Boundary

JSON owned pack defines a private flat Snapshot and derives DslRecord. Its existing controlled output already calls `Snapshot::__dsl_spec_producer()`, proving a real declared producer exists. A sibling SQL module cannot name that private Snapshot; the clean candidate is an owner `pub(super)` decode helper within pack root, where it can pass that producer directly into canonical `store::decode_sqlite_snapshot_record_native`. No public flat carrier or external runtime dependency is needed. The SQL hook forwards its payload/control to that owner helper.

The canonical helper owns input file-byte admission, controlled declared spec decoding, controlled Semio framing, physical Binary/Text record decoding, one native control and one caller allocation_stage. The owner callback can call existing `reconstruct_record(record,native,maximum_rows)` and convert its ValueError to TextError using `TextError::from_value_error` with an explicit synthetic span, as current controlled output does. No separate ordinary to_value/from_value or native wire normalization belongs in the candidate.

## Borrowed Census and Exact Binding

Existing pack decode bind first borrows source fields, verifies exactly two root fields, nodes nonempty, exact six node fields and row count including links, with controlled census steps. Only then does it copy schema and allocate Node storage. Each optional text, item index and member key gets controlled admission. Ordered member vectors preserve duplicate keys, empty strings, Unicode/NUL, schema and literal number lexemes. This is full typed intermediate state rather than a JSON physical-file carrier.

Existing reconstruction validates every node variant and forward unique ownership, checks orphaned nodes and row/work overflow, then allocates the typed value slots. `Values`, `Items`, and `Members` guards retire partial recursive values on failure/cancellation; the final checkpoint occurs before taking the root from Values. No post-construction callback that can fail appears after root publication inside this helper.

## Cumulative Admission and Diagnostics

Canonical allocation_stage receives remaining max_allocation_bytes; NativeDecodeControl admits declared metadata, parser and owner binding backing under that same remaining ceiling, then reports native.owned_bytes for settlement on success or error. The outer caller retires no cumulative allowance. Text and Binary use the same callback and retained typed owner. max_file_bytes applies to both; max_rows is enforced by owner census; semantic max_value_bytes must remain independent of this native allocation authority. The final SQL projection subsequently enforces semantic values.

`TextError::from_value_error` and canonical conversion back to ValueError move the existing message String and preserve kind. They do not independently charge diagnostic strings to the native payload control. ValueError literal/formatted construction and some text parsing diagnostics allocate error prose outside native owned_bytes. Thus allocation settlement proves admitted producer backing; it does not establish an all-heap-allocation ceiling including error representation. This distinction already exists in the canonical helper and should not be inferred away by Native20's exact backing check.

## Retirement Limit

Existing JSON `retire_value` uses an allocating Vec frontier. Guards prevent recursive drop of partial deep values, but this source alone does not prove allocation-free retirement when the actual process allocator refuses all allocations. Native20 uses a shallow fixture and does not establish that stronger lifecycle law. Candidate reuse should retain current guards; any stronger retirement requirement needs its own actual evidence rather than being asserted from the paid parser change.
