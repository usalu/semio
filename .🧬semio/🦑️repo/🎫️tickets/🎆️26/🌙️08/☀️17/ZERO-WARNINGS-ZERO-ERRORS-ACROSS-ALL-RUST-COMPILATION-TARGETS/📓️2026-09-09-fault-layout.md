# Compact Fault Layout

Native882 exposed a common 216-byte Fault return value through hundreds of plugin and framework functions. Its inline FaultScope contains five optional strings (120 bytes on the native target). Added language-neutral wire vectors and an inline-layout budget test before changing the representation. The wire test compares the owned ToValue/FromValue projection with independent serde_json values and exercises constructors, scoped faults, causes, spans, retryability and absent/null optional fields. The new tests parsed; the initial runtime run is pending.

- 🧰️framework/🔨️modules/⚠️diagnostic/🧪️tests/🔬️fault-describe/🦀️.rs
- 🧰️framework/🔨️modules/⚠️diagnostic/🧫️fixtures/🧯️fault/🔣️.json

## Initial Exact Runtime Result

Resolved the fixture include paths against their actual source directory before compilation. Baseline result:

{
  "status": "failed",
  "error": "Error: exact Cargo law native failed: status=101 signal=none artifacts=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS/🗑️generated/native-laws/exact-cargo-laws-WVVQ0Y/00; native assertion diagnostic::fault_describe_tests::fault_wire_projection_matches_language_neutral_serde_oracle did not pass exactly once; \nrunning 1 test\ntest diagnostic::fault_describe_tests::fault_wire_projection_matches_language_neutral_serde_oracle ... FAILED\n\nsuccesses:\n\nsuccesses:\n\nfailures:\n\n---- diagnostic::fault_describe_tests::fault_wire_projection_matches_language_neutral_serde_oracle stdout ----\n\nthread 'diagnostic::fault_describe_tests::fault_wire_projection_matches_language_neutral_serde_oracle' (6433607) panicked at 🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/../../../⚠️diagnostic/🧪️tests/🔬️fault-describe/🦀️.rs:46:9:\nassertion `left == right` failed\n  left: Fault { origin: Plugin, code: FaultCode(\"plugin.boundary.refused\"), severity: Error, message: \"Übertragungsfehler 🧩\", scope: FaultScope { plugin_id: Some(\"puzzle\"), app_id: Some(\"editor\"), instance_id: Some(\"17\"), module: Some(\"wire\"), body_key: Some(\"world\") }, span: Some(TextSpan { line: 3, column: 7, length: 11 }), causes: [FaultCause { message: \"source remained owned\", code: Some(FaultCode(\"wire.retained\")) }, FaultCause { message: \"cancelled\", code: None }], retryable: true }\n right: Fault { origin: Plugin, code: FaultCode(\"plugin.boundary.refused\"), severity: Warning, message: \"Übertragungsfehler 🧩\", scope: FaultScope { plugin_id: Some(\"puzzle\"), app_id: Some(\"editor\"), instance_id: Some(\"17\"), module: Some(\"wire\"), body_key: Some(\"world\") }, span: Some(TextSpan { line: 3, column: 7, length: 11 }), causes: [FaultCause { message: \"source remained owned\", code: Some(FaultCode(\"wire.retained\")) }, FaultCause { message: \"cancelled\", code: None }], retryable: true }\nnote: run with `RUST_BACKTRACE=1` environment variable to display a backtrace\n\n\nfailures:\n    diagnostic::fault_describe_tests::fault_wire_projection_matches_language_neutral_serde_oracle\n\ntest result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 263 filtered out; finished in 0.00s\n\n",
  "artifactDir": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS/🗑️generated/native-laws/exact-cargo-laws-WVVQ0Y/00",
  "stage": "native"
}

## Initial Exact Runtime Result

The initial oracle reconstruction omitted its source severity; corrected that test setup before repeating the unchanged production baseline. Result:

{
  "status": "failed",
  "error": "Error: exact Cargo law native failed: status=101 signal=none artifacts=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS/🗑️generated/native-laws/exact-cargo-laws-Wvz3Ii/00; native assertion diagnostic::fault_describe_tests::fault_inline_layout_stays_within_the_language_neutral_budget did not pass exactly once; \nrunning 1 test\ntest diagnostic::fault_describe_tests::fault_inline_layout_stays_within_the_language_neutral_budget ... FAILED\n\nsuccesses:\n\nsuccesses:\n\nfailures:\n\n---- diagnostic::fault_describe_tests::fault_inline_layout_stays_within_the_language_neutral_budget stdout ----\n[DEBUG] Fault inline bytes=216 maximum=112\n\nthread 'diagnostic::fault_describe_tests::fault_inline_layout_stays_within_the_language_neutral_budget' (6438649) panicked at 🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/../../../⚠️diagnostic/🧪️tests/🔬️fault-describe/🦀️.rs:29:5:\nassertion failed: actual <= maximum\nnote: run with `RUST_BACKTRACE=1` environment variable to display a backtrace\n\n\nfailures:\n    diagnostic::fault_describe_tests::fault_inline_layout_stays_within_the_language_neutral_budget\n\ntest result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 263 filtered out; finished in 0.00s\n\n",
  "artifactDir": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS/🗑️generated/native-laws/exact-cargo-laws-Wvz3Ii/00",
  "stage": "native"
}

## Compact Scope Ownership

The corrected baseline wire test passed, and the layout test failed for the intended reason with a runtime log of 216 inline bytes against a 112-byte maximum. Moved only Fault’s five-field scope metadata into an owned Box. Construction and decoding allocate that scope once; with_scope replaces the contents of the existing allocation. ToValue field names and all wire data remain the same. The Rust file parsed before its guarded write. Fresh runtime and strict compilation are pending.

- 🧰️framework/🔨️modules/⚠️diagnostic/🦀️.rs

## Compact Exact Runtime Result

Repeated the same exact wire and layout tests after compacting the scope representation. Result:

{
  "status": "passed",
  "receipts": [
    {
      "package": "semio-framework-replication",
      "target": {
        "kind": "lib"
      },
      "executable": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS/🗑️generated/derive-target/debug/deps/protocol-30f5c31345c51f55",
      "sha256": "6eb30ef672ea29561c6266db2809a79a792a8d132e63f1c6124270d3559e511a",
      "laws": [
        "diagnostic::fault_describe_tests::fault_wire_projection_matches_language_neutral_serde_oracle",
        "diagnostic::fault_describe_tests::fault_inline_layout_stays_within_the_language_neutral_budget"
      ],
      "assertions": 2,
      "artifactDir": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS/🗑️generated/native-laws/exact-cargo-laws-thqeAv/00",
      "cargoTargetDir": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS/🗑️generated/derive-target"
    }
  ]
}

## Runtime Proof and Continued Coverage

The compact implementation passed both exact tests. The runtime log measured 104 inline bytes, down from 216. The executable SHA-256 was 6eb30ef672ea29561c6266db2809a79a792a8d132e63f1c6124270d3559e511a. Both fault laws are now in the combined regression catalog, along with the existing app-context identity oracle for the next context construction refactor.
