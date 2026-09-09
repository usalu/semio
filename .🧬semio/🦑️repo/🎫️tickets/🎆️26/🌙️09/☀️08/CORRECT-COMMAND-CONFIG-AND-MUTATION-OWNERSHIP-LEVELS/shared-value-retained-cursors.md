# Shared Value Retained Cursors

The Wires move audit found two reusable framework concerns: copying immutable dynamic values with bounded progress/cancellation, and exposing those values to the existing Store canonical JSON encoder. The copy cursor lives beside framework Value; the canonical trait implementation belongs beside the Store encoder it serves. Neither introduces a runtime dependency.

The neutral clone contract and fixtures were added first. `shared-value-clone-red-1.log` failed as expected with missing cursor symbols in 9.8 seconds. The implementation now owns an immutable source, advances one structural item or up to 256 UTF-8 bytes, checks depth and payload capacity before allocation, preserves all numeric variants and ordered duplicate object keys, and retires abandoned output before returning the exact source. Native validation of the implementation is pending.

The first green attempt was interrupted while waiting for the shared Cargo lock because the disk filled. This ticket had accumulated 79 GB of incremental compiler cache. After all observed holders of its Cargo lock had stopped, only its generated `cargo-trinity/debug/incremental` directory was removed. Compiled dependencies, fingerprints, inputs, and reports were retained. The ticket validation wrapper now disables incremental compilation to prevent recurrence. The forced-remove command was rejected by command policy; the non-forced recursive removal succeeded.

The canonical encoder bridge reuses the same neutral values and a serde_json byte oracle with output chunks of 1, 7, and 256 bytes. Its test and implementation work remains in progress. Both verification commands are registered through the root script, Nx, and the launch catalogs.

The post-reset clone attempt reached compilation and exposed an incorrect relative fixture path (`shared-value-clone-green-2.log`, 50.6 seconds). That include path is corrected; the next clone run and the canonical bridge RED run are queued. Cache removal completed successfully and the disk reports 94 GB free.

## Clone native checkpoint

`shared-value-clone-green-3.log` is GREEN: 3 native tests passed in the Nx run (2 minutes 7 seconds including the compiler queue). Runtime receipts confirm all nine neutral JSON values, the large UTF-8 string, exact integer/float variants and duplicate-key order, ten cancellation checkpoints returning the exact source, and capacity/depth rejection without owner loss.

The Store canonical DslValue bridge RED run (`shared-value-canonical-json-red-1.log`, 1 minute 50 seconds) failed solely with the expected missing `ArtifactCanonicalJson` implementation. The implementation now performs at most 64 indexed lookups and exposes the native numeric variants, string borrows, array length, and exact object entry order to the existing encoder. `shared-value-canonical-json-green-1.log` is pending.

The first canonical bridge run compiled but exposed an oracle mistake: `DslValue`'s existing serde adapter first builds serde_json::Value, which sorts keys and collapses duplicate keys. The production first-party JSON writer preserves unique-key insertion order. The oracle now uses serde_json's independent streaming map serializer over ordered input and checks its bytes against the production writer before comparing the bounded encoder at all three chunk sizes. Clone fidelity still separately tests duplicate-key preservation. The canonical bridge rerun is `shared-value-canonical-json-green-2.log`.

## Canonical bridge native checkpoint

`shared-value-canonical-json-green-2.log` is GREEN: one native test passed, covering eleven neutral/adversarial values at 1-, 7-, and 256-byte chunk sizes. Every expected stream agrees first with both the independent serde_json streaming serializer and the production first-party JSON writer. The bounded encoder then matches those exact bytes. Its runtime receipt confirms the full matrix. Both shared primitives are now natively validated; integration into Wires preparation and the Wires gesture runtime gate remain unfinished.
