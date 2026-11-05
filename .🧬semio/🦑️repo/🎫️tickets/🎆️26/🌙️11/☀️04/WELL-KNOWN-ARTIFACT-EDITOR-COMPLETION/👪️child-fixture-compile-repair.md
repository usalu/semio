# Child Fixture Compile Repair

Changed only child-genesis-wire-and-group-owner source and the exact shared root ArtifactRef import. The root previously imported ArgFormat::ArtifactRef, an enum variant shadowing the actual artifact-reference struct; it now imports semio_framework_artifact_reference::ArtifactRef.

The child ephemeral preparation law now owns a genuine OriginalFixtureScope and passes its same cx to the existing financed advance_cancellation_presence helper. That helper loans actual NativeDecodeControl through with_native_receiving and reconciles receiving-retained progress. The stale two-argument PresenceStore call is gone. All existing Progress, unchanged-root, generation and closure assertions remain intact. No fixture_step_context was introduced and no Rust production source changed.

Nightly rustc parse-crate-root-only/ast-tree checks for both changed files exit0 with no diagnostics. The pinned toolchain has no rustfmt component; no installation was attempted. This is parsing only, not typechecking or actual native test execution. Root owns the next tests01 build.
