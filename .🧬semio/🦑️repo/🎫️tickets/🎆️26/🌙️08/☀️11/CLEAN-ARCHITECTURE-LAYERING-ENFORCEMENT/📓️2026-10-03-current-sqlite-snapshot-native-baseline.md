# Current Lower SQLite Snapshot Native Baseline

The existing complete registered @semio-tech/framework-rs:test-snapshot-sqlite-native target (session38111) compiled the actual lower library in1.60s and discovered31 names. All31 ran under the original --lib --no-fail-fast policy:28 passed, three failed, zero skipped or unrun, runtime0.144s, Nx22.5s. The later authored oracle binary build was not reached because the native library runtime failed; no binary/interoperability success is inferred.

The failing actual laws are allocation_cross_stage_frontier_is_independent_of_semantic_payload, allocation_real_native_bridge_settles_interior_failure_before_next_copy, and allocation_failed_stage_cannot_refund_owned_backing. The first and third observe the current baseline unimplemented backing-allocation admission; the second observes the unimplemented native allocation bridge. Every original assertion remains unchanged.

## Source Binding Limitation

A complete39-row source capture preceded the run. Post observation found exactly one Rust source delta: SQLite production root compiled49341 bytes (SHA958086…) and now49715 (SHA54f5ad…). The compiled .d has12 authored inputs: eleven remain current, while the changed root exactly matches the genuine complete before capture by compiled BLAKE3 and byte length. Authorship is unknown; Root declared no SQLite Rust mutation in its lane. Native test source remained current. Therefore the actual28/3 baseline is source-bound to the captured compiled epoch, not a verdict on the newer production bytes. TypeScript allocation carrier observations are separately captured and are not inferred from native .d.

## Evidence

Generated/current-native-worker/sqlite-snapshot-original-full-native-before-1.json retains complete inputs; sqlite-snapshot-original-native-1-runtime-and-compiler-bindings.json retains actual31 names and12 checks with captured-before verification; sqlite-snapshot-original-native-1-post-source.json and sqlite-snapshot-original-independent-rust-delta-1.json retain exact whole current/before source and unknown attribution. The unchanged full registered log retains all three failures. Root owns the subsequent typed test-first feature; no production fix or count is claimed here.
