# Current Public SQLite Typed Refusal Integration Audit

This is a read-only source audit of the unfinished refusal cohort. No producer, feature law, dispatcher or refusal API was modified. No Cargo ran here.

## Actual Current State

The physical refusal test already imports `semio_framework_value::ValueError` at `framework/modules/io/sqlite-snapshot/⚠️refusal/tests/🦀️.rs:2`. The previously reported missing import is no longer present. Its helper nevertheless promises `ValueError` while returning three incompatible existing authorities:

| Actual producer | Current refusal type | Test operations |
| --- | --- | --- |
| `SnapshotEncoding::parse`, `SqliteDatabase::from_schema`, `export_sqlite_database` | `SqliteSnapshotError` | encoding, schema, NaN, page cancellation, seven structural/resource limits |
| `artifact::Projection::new` | `String` | projection cancellation |
| `SqliteSnapshotControl::admit_allocation_bytes` | `String` | allocation ceiling |

The direct match arms cannot satisfy the declared helper result or `.kind` assertions. This is a conceptual compiler mismatch, not a runtime assertion receipt. The physical module includes this test under `#[cfg(test)]` at its current lines 522–524. The exact smallest missing work is the feature owner's producer return-type/category port; adding another import in this lane would not repair it.

The physical engine remains typed through its existing `SqliteSnapshotError` enum and `Result` alias (`🦀️.rs:173–182`). The `Limit(&'static str)` payload currently records a reason but does not distinguish ownership from work categories. The neutral corpus requires ownership limits for bytes and work limits for rows/columns/tables/pages. Categories must therefore be authored at each actual predicate or through a typed enum variant, never recovered from the displayed prose or reason string. Existing typed errors should remain typed while that owner finishes the port.

## Metadata And Dispatcher Boundaries

The generic artifact helpers currently return `String`, including controlled copying, projection, IEEE validation and reconstruction. Their schema admission explicitly converts the engine's enum with `.to_string()` (`🧩️artifact/🦀️.rs:67–71`). `SqliteSnapshotControl` likewise returns `String` from allocation admission, row/value limits, reconstruction admission, database validation and cancellation (`physical 🦀️.rs:136–168`). These are actual internal semantic ownership stages; they currently lose refusal provenance before the public I/O terminal.

Owned metadata and dispatch live in the canonical framework I/O module, not the OS tree. `validate_snapshot_schema`, `sqlite_snapshot_metadata`, `attach_sqlite_snapshot_metadata` and `take_sqlite_snapshot_metadata` return `String` (`io/🦀️.rs:2167–2224`). Metadata encoding parsing converts the engine enum to text. Invalid metadata schema is folded to a new textual failure after `.is_err()`. Attach limits and overflow sites create text directly, while callback failures arrive through the control's textual result.

Typed public export/import (`io/🦀️.rs:2413–2436`) and erased `run_snapshot_hop` (`2439–2476`) project those messages to `IoError`. That public error currently owns only `message` plus `diagnostics`; its sole conversion is `From<String>` (`io/schema/🦀️.rs:289–300`). The erased helper's `fail` closure explicitly takes `String`. No `From<ValueError>` or enum-to-`IoError` conversion exists in the inspected contract. A later canonical typed return port must consequently preserve the producer through metadata/ownership stages and project explicitly only at the intended diagnostic terminal. A global `From<ValueError> for String` would hide these boundaries and was not added.

The Store capability's owner hooks currently declare `Result<_, String>` (`store/🦀️.rs:10886–10905`); the erased codec itself returns `IoResult` and holds the owned snapshot retirement guard throughout fallible conversion. Its current internal signature agrees with the existing String helpers, so the inspected refusal test does not identify a new Store caller compile defect owned by this lane. The shared controlled native decoder also explicitly projects native typed failures at its existing String terminal; it is outside this audit's allowed unfinished feature edits.

## Conclusion And Ownership

No smallest producer prerequisite in this executor's owned source was found. The stale missing-import report is resolved in current source; the real test/type mismatch remains the unfinished refusal feature owner's responsibility. This audit preserves all current typed producers and does not weaken or mask the new laws. Root's Cargo lane remains the authority for compiler and runtime receipts.

Separately, Root's latest actual CommonMark run passed all original 24 laws, including zero-allocation linear retirement and final SQL cancellation, within a 25-law selector whose new shared allocation law failed. Its receipt is retained in `📓️md-native-retirement-runtime-correction.md`.
