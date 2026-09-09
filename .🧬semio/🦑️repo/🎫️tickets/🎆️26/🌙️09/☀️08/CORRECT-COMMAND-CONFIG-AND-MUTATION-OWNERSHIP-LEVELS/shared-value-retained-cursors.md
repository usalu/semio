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

## Independent Audit and Canonical Admission Correction

The independent Terra audit found two integration blockers despite the earlier narrow green tests: clone work had no caller grant, and raw duplicate-key objects produced bytes different from the first-party pack writer. The clone grant and source-trait corrections are assigned to a Sol agent. Root added a Store-owned admission contract and neutral fixtures before extending the canonical tests. `canonical-value-admission-red-1.log` confirms the expected missing admission types (E0432).

The direct `ArtifactCanonicalJson for DslValue` implementation is being replaced with an immutable validated wrapper. Bounded admission hashes keys in caller-sized byte chunks, checks collisions by bounded byte comparison, traverses children by index, checks finite numbers and depth, and limits its numeric key-index allocation and total work. Duplicate keys reject before any canonical wrapper can be obtained. The source remains owned through rejection/cancellation and must be returned after bounded close. This retains raw clone fidelity without allowing divergent canonical hashes.

The test expansion covers duplicate keys at root and under arrays; zero and 1/7/256-byte grants; neutral cancellation checkpoints; allocation, work, depth, non-finite failures; late cancellation after completion; and the original ordered serde_json/first-party writer byte comparisons. Implementation is mounted but native validation is pending the shared source-trait export and compiler queue. No new green result is claimed.

The canonical admission implementation's first native attempt (`canonical-value-admission-green-1.log`) was blocked during dependency compilation by the concurrently edited clone cursor: ManuallyDrop has no `as_mut` method. The execution agent corrected that access to `&mut *self.state`; the type mismatch was a cascade. Canonical GREEN2 is queued against the corrected shared source. The tests additionally check long UTF-8 duplicate-key hashing/comparison and verify returned Arc ownership has exactly one strong reference.

## Canonical Admission Native Verification

`canonical-value-admission-green-3.log` is GREEN: four native tests passed, with actual debug receipts for duplicate rejection at 1/7/256 bytes, zero/tiny grants, cancellation and exact source return, capacity/work/depth/non-finite failure paths, and eleven neutral/adversarial JSON values matching both the first-party writer and independent serde_json streaming serializer. Nx exited 0 after 8m19s (compilation and queue included); native tests completed in 0.01s. This validates the current shared canonical admission API; it does not validate the still-unimplemented Wires document preparation. An independent Terra source audit is in progress.

## Close Receipt and Allocation Audit Follow-Up

`canonical-close-receipt-red-1.log` reached the expected missing `ArtifactCanonicalValueCloseStep` and `reserve_table_with` compiler failures. The implementation now returns the exact retained source within the terminal granted close step; zero-item close preserves active state and positive structural grants work with zero byte allowance. The numeric reservation seam checks requested capacity before allocating and actual capacity before retaining the empty table. New allocator-overcapacity fixtures and cancellation tests are in the native suite. GREEN validation is running in `canonical-close-receipt-green-1.log`; no final result is claimed yet.

## Final Audit Follow-Up Runtime Receipt

`canonical-close-receipt-green-2.log` is terminal GREEN: Nx exited 0 in 4m 3s, and all 5 focused native tests passed. Runtime debug receipts confirm zero-item close at every active checkpoint, positive structural/zero-byte close, exact source transfer on the terminal receipt, injected allocator overcapacity rejection, duplicate-key rejection, work/depth/capacity/nonfinite errors, and canonical bytes matching serde_json for 11 values at 1/7/256-byte budgets. GREEN-1 stopped on concurrently invalid taxonomy ordering; that ordering was corrected externally and no taxonomy edit was made here.
