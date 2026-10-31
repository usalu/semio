# Cargo Test Flag Correction

Native10 outer83035 actually ended EXIT1/inner1/null during Cargo test unit-graph planning: --keep-going is rejected, with a diagnostic recommending --no-fail-fast. Its final45,188-byte log SHA is cd760251c875659f7079425b4422f951f4779383f196f2d7fbbe42c9abf8abf4. Machine/image/headroom and scheduler-only Source1/1 had passed; the selected graph, compilation and tests never began. All three actual-log postprocessors are closed unused with zero attempts.

Root chose the unsupported option. The earlier read-only review established argument forwarding and a Cargo build precedent, not actual Cargo test support. Actual execution contradicted the combined-flags recommendation; the earlier review now has a prominent correction. No Source repair or runtime acceptance follows from this attempt.

The next exact full19 request removes only --keep-going while retaining --no-fail-fast before the final runtime separator. The official [Cargo test documentation](https://doc.rust-lang.org/cargo/commands/cargo-test.html#test-options) documents --no-fail-fast for continuing other test executables after one fails; the Rust harness still completes tests inside each executable. A nonzero result remains failure. No new compiler flag, filter, wrapper, quota, profile, source or acceptance weakening is introduced. Actual planner acceptance remains necessary.
