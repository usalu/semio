# Run Public Serde and Schema Parity Client

## Purpose

`🧪️run-schema-35/🦀️.rs` is a standalone public Rust client for the strict Run JSON contract. It imports only public `semio_framework::workflow` payload types and `RunMutation`; it does not copy production payload definitions or use a handwritten mirror.

The client reads the retained authoritative vector input directly with:

```rust
include_str!("../🧪️run-direct-31-independent-review/🔣️payload-vectors.json")
```

The input is neither copied nor modified. It has the exact 52-case roster below.

| Group | Cases | Behaviour |
| --- | ---: | --- |
| Leaf positives | 6 | Deserialize the declared actual leaf type, then serialize and require exact JSON equality. |
| Leaf negatives | 19 | Require the declared actual leaf type to reject the payload. |
| Aggregate positives | 5 | Deserialize `RunMutation`, then serialize and require exact JSON equality including `operation`. |
| Aggregate negatives | 22 | Reject three malformed aggregate envelopes and the 19 leaf-negative payloads after wrapping them with their declared `operation`. |

The runtime client prints every vector name as `[DEBUG] Run schema <name>=passed|failed`, checks the roster count is exactly 52, and fails after reporting every failed name. The expected success summary is:

```text
[DEBUG] Run schema parity accepted cases=52 leafPositive=6 leafNegative=19 aggregatePositive=5 aggregateNegative=22
```

## Exact named roster

Leaf positives: `start-run-manual`, `start-run-automation`, `start-run-node`, `finish-run-node`, `append-run-log`, `seal-run`.

Leaf negatives: `start-root-unknown`, `start-parameter-unknown`, `start-manual-trigger-unknown`, `start-automation-trigger-unknown`, `start-workflow-ref-type`, `start-trigger-kind`, `start-node-root-unknown`, `start-node-id-type`, `finish-root-unknown`, `finish-record-unknown`, `finish-input-fingerprint-unknown`, `finish-output-fingerprint-unknown`, `finish-output-unknown`, `finish-status-enum`, `finish-duration-type`, `append-root-unknown`, `append-level-type`, `seal-root-unknown`, `seal-status-enum`.

Aggregate positives: `aggregate-start`, `aggregate-start-node`, `aggregate-finish`, `aggregate-append`, `aggregate-seal`.

Aggregate envelope negatives: `aggregate-unknown-operation`, `aggregate-mismatched-payload`, `aggregate-unknown-field`.

Wrapped aggregate negatives: the 19 leaf-negative names above, each prefixed `aggregate-`.

## Execution status

Prepared only. Root owns the compiler controller and will compile/run this public client against the fresh framework artifact pair. This lane did not run Cargo, rustc, or alter the controller. Production Run sources and the frozen direct-review client were not edited.

## Executed Root Gate

After framework build35, root compiled this real public client through `🧪️workflow-actual-source-34/📜️script.ts run-schema`. The retained result is `🧫️run-dSE9RN`: compiler exit0, runtime exit0, all52 named cases passed and complete source/artifact/vector fingerprints remained unchanged. The final runtime summary exactly reports6leaf positives/19leaf negatives/5aggregate positives/22aggregate negatives. This matches the independent Ajv oracle; it proves the scoped strict JSON acceptance/rejection and exact positive serialization contract, not Run inverse/algebra or runner/Plugin behavior.
