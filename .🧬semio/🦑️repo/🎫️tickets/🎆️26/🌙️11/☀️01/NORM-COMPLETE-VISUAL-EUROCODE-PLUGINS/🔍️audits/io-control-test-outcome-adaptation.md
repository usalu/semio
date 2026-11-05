# IO Control Native Outcome Tests

Only existing io/control/🧪️tests/🦀️.rs and io/control/🧪️tests/nested-io.rs were adapted. Existing fixtures, mounted tests, third-party serde comparisons, source identity, physical receipt, allocation, cancellation and retirement assertions remain.

Callbacks now use the existing native_snapshot_turn and NativeSnapshotWork accumulator. Fixture-selected allocation, copy, checkpoint and transfer actions record explicit performed work; scoped and cursor children fold their actual outcome through work.take before observing either result. Direct delegated callbacks return the child outcome unchanged. Cursor fixture failures carry the actual callback progress into the owning native failure. Terminal assertions access canonical result and selected success/cancellation paths require nonzero actual reports; denied cursor frames require work zero and preserve the original zero-allocation and no-callback assertions. No default or guessed progress-to-work conversion was added.

No production source or extra Rust file changed. Native recovery owns the single coherent successor gate; no compiler or passing-test claim is made yet. The earlier settlement red2296 had compiler failures in preexisting Result fixtures and therefore did not execute those tests.

The native-owned settlement successor58271 log native-recovery-settlement-green-2.log reached lib-test compilation and emitted only three E0277 errors in the separately owned rich_drive tests (drive source lines10/33/48, receiving_issued_grant Option propagation). No errors in these two adapted files were emitted. No test body/pass evidence yet; native recovery owns that separate fixture repair and next gate.
