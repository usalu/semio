# Scalar Fixture Audit

The scalar integration test initially did not compile: it called a nonpublic compiled-guest method and referred to an unimported export enum. The test now uses the already-instantiated independent Wasmtime guest to verify the same exported function arity. The native scalar parity test passes with all assertions intact.

The companion failed-input test exposed malformed synthetic Wasm: offsets 152 and 160 were emitted as single bytes instead of ULEB128, and an i64 was used as an `if` condition. The fixture now encodes the offsets and compares the condition against zero. It advances through failure acknowledgment to canceled input cleanup, where Core rejects the resumed `AdmitActorInput` as a non-retirement export. `UnitContinuation::allows` explicitly authorizes this continuation, whereas Core's `retirement_export` only accepts exports classified by `is_original_retirement`. This requires reconciling the authenticated continuation with the Core retirement boundary; no production authorization was widened in this fixture task.

Validation: `repo tools exec --light bun -- <ticket>/mcp-native-build.ts --isolated-gateway --optimize-interpreter --interpreter-scalar-test` compiled and ran both tests. Scalar parity passed; failed-input cleanup failed on the retirement boundary described above.
