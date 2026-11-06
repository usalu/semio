# General Prepaid Failure Parts and Paid Unused Storage
The current control contract already supplies the allocation-free leaf constructor OriginalRetirementFailure::from_causes. No redundant constructor or compatibility alias was added.

OriginalFailureStorage::attach_parts now accepts the original child and all three independent parent causes. It initializes the already-admitted node and transfers its Box into the child field without allocating a second node. The existing specialized work-post attach delegates to the general constructor.

Unused OriginalFailureStorage now implements the canonical RetireOwned protocol in the existing refusal retirement module. Its cursor quotes the exact original node release, retains the node on any denied grant, releases it only after admission, and separately exposes its own terminal wrapper release. There is no allocator dependency or automatic ordinary drop of an unpaid node.

The finite original_failure_storage_parts integration target was authored before implementation. It checks all six owned string pointers across a two-node chain; verifies attachment allocates/releases zero bytes; closes the complete chain under exact per-turn grants with actual heap/receipt comparison; and verifies an unused node survives a release-byte denial with zero work/heap, then closes exactly. This is a lower native unit target, not a genuine OriginalJob proof.

The schema-first language-neutral fixture is beside the native test. Its independent Ajv/SQLite oracle passes one test with three assertions. Production and test source pass Rust AST parsing. The native target is queued in the sole verification driver and is unrun. The oracle initially had an incorrect relative fixture import; its exact path was corrected before the passing rerun.

Changed files: the actual control-contract failure owner, the existing refusal retirement implementation, the retirement package manifest, and parts.rs / parts.json / parts.schema.json beneath the retirement contract tests. Ticket input: failure-parts.test.ts.

First native target attempt reached rustc but stopped before tests at two E0061 errors in concurrently changed prepared retirement settlement: the helper now requires an actual effects projection. Root passed the real successful step.progress() in both original work/loan paths, forwarding it without fabricating progress. Fresh native retry queued; no passing native result yet.

The second attempt compiled value-retirement, including the new storage implementation, but unused parent dev-dependencies then pulled in native encoding during its allocation-event migration and stopped before the target at three type errors. A finite companion package now mounts the exact same parts.rs and fixtures with only retirement, allocator-observer and serde_json test dependencies. The parent integration entry added by root was removed; no sources were copied and no quotas changed. Native retry now targets semio-framework-value-failure-tests --lib.

## Native Result
Label original-failure-storage-parts-companion completed with outer rc0. Actual Rust prototype compilation succeeded and semio-framework-value-failure-tests ran three tests: every original leaf/parent pointer retained with zero attach allocation; unused storage survives denied release and closes under exact heap/receipt matching; and a concurrently added own-progress override test preserves nested causes. Three passed,zero failed,zero filtered. Root authored the first two; the third is concurrent work preserved in the same physical test file. This is genuine native lower-layer evidence, not whole editor, Source/Job, or recursive wire proof. Compiler reported prototype completion19.14s.
