# Note Receiving Audit

The nested statement vector now invokes `StatementReceiving` for each tagged item. In `🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🫴️receiving/📥️owned/🎁️wrapping/🦀️.rs:67`, phase 1 pops a `(String, RecordValue)` and binds the keyword as `_`. This drops its allocated string during a metadata-only local effect without retaining or accounting for its release. Retain the keyword in source custody and retire it under a paid effect. This may violate allocation accounting in the new owned receiving path.

The audit was read-only for production files. No native compiler or runtime fixture was run; type correctness and actual test failure remain unverified.
