# Child Borrowed Receiving Audit

The borrowed `DslReceivingField` delegates to `receive_decoded`, which calls `ArtifactChild::from_record_controlled` before publishing the result. In `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🦀️.rs`, the child ID is copied at line 470 into a local `String`. A malformed nested target or a refusal at lines 471–473 drops this allocated partial result without retained custody and paid cleanup. Cancellation inside `NativeDecodeControl::copy_text` can likewise drop a partial buffer. A staged receiver with caller-owned partial destinations is required.

The existing `🪆️child/🛬️receive/🧪️tests` Rust fixture exercises `NativeReceive<DslValue>`, not the borrowed `DslReceivingField` receiver. Its Bun oracle checks schema and SQLite round trips but does not execute the borrowed receiver or malformed `FieldValue` and interrupted-copy cases.

The audit did not run a compiler or modify production files.
