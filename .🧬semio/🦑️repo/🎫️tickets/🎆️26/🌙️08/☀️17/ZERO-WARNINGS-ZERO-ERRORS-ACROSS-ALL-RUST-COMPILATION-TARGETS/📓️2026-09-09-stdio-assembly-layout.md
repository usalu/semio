# Stdio Assembly Layout

Native948 identified ArtifactAssembly's 408-byte runtime variant versus its 72-byte definition-only variant. Added a language-neutral 128-byte aggregate budget and exact binary artifact identity, read and projected with the existing workspace serde_json test oracle. Added that regression, the Base64 reference test, and both existing stdio catalog invariants to the combined runtime catalog. Runtime baseline and representation repair are pending.

- ✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/🧪️tests/🔬️unit/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/📦️packages/🦀️rust/Cargo.toml
- /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS/📜️script.ts
- ✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/🧫️fixtures/📦️assembly/🔣️.json


Corrected the new fixture include to its verified two-parent relative path and used the Rust prelude size_of function. Native948 read Cargo metadata before the new test-only serde_json dependency was added; this fresh baseline resolves that dependency from the current manifest.


assembly952: {"status":"failed","error":"Error: exact Cargo law native failed: status=101 signal=none artifacts=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS/🗑️generated/native-laws/exact-cargo-laws-zdoNxL/00; native assertion tests::artifact_assembly_layout_and_identity_match_neutral_budget did not pass exactly once; \nrunning 1 test\ntest tests::artifact_assembly_layout_and_identity_match_neutral_budget ... FAILED\n\nsuccesses:\n\nsuccesses:\n\nfailures:\n\n---- tests::artifact_assembly_layout_and_identity_match_neutral_budget stdout ----\n[DEBUG] Artifact assembly inline bytes=408\n\nthread 'tests::artifact_assembly_layout_and_identity_match_neutral_budget' (7723291) panicked at ✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/📦️packages/🦀️rust/../../🧪️tests/🔬️unit/🦀️.rs:19:5:\nassertion failed: bytes as u64 <=\n    fixture[\"maximumInlineBytes\"].as_u64().expect(\"assembly budget\")\nnote: run with `RUST_BACKTRACE=1` environment variable to display a backtrace\n\n\nfailures:\n    tests::artifact_assembly_layout_and_identity_match_neutral_budget\n\ntest result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.01s\n\n","stage":"native","artifactDir":"/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS/🗑️generated/native-laws/exact-cargo-laws-zdoNxL/00"}


The assembly952 baseline built successfully and failed only the intended 128-byte layout assertion, logging 408 bytes after its definition identity matched the JSON oracle. Boxed the runtime declaration at its single assembly constructor and transferred that exact declaration into the plugin builder when consuming the enum. Definition-only rows remain inline. Syntax checks passed before both guarded writes; post-change runtime verification follows.

- ✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🔌️plugin/🦀️.rs


assembly957 passed: [{"package":"semio-s-artifact-stdio-contract","target":{"kind":"lib"},"executable":"/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS/🗑️generated/derive-target/debug/deps/semio_s_artifact_stdio_contract-08dbea34ef3a245d","sha256":"77bfddff746645bc3cc8e9bad5ea68c5b7f55e0d0c6bea221e9719f9c9d396b9","laws":["tests::artifact_assembly_layout_and_identity_match_neutral_budget","tests::standard_base64_matches_the_reference_implementation"],"assertions":2,"artifactDir":"/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS/🗑️generated/native-laws/exact-cargo-laws-Acgiiy/00","cargoTargetDir":"/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS/🗑️generated/derive-target"}]


Post-change assembly957 passed both exact native tests. The runtime logged 72 inline bytes, down from the baseline's 408 bytes, within the committed 128-byte budget. Artifact identity matched the serde_json projection, and Base64 matched the independent base64 library. Executable SHA-256: 77bfddff746645bc3cc8e9bad5ea68c5b7f55e0d0c6bea221e9719f9c9d396b9. The combined catalog now contains 72 package targets and 545 exact selectors, validated in runtime955-scope.json; the full combined runtime run remains pending.
