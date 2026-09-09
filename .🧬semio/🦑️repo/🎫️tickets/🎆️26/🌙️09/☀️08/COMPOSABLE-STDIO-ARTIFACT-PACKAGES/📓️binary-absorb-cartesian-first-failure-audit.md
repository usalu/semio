# Binary Absorb Cartesian First-Failure Audit

Captured 2026-09-09T15:42:12+02:00. This is a read-only analysis of the completed Binary part of the combined Binary/TXT run: 42 tests selected, 40 passed, 2 failed. The analysis covers only the first `absorb_law_cartesian` panic; the separate text-codec failure is outside this report's source scope.

The panic happens before `BinaryDiff::absorb` is called. The failing unwrap is at `🧬️mutations/🧪️tests/🔬️unit/🦀️.rs:50`, which applies the second diff to `mid`. Its concrete pair is:

- Base bytes: `[1, 2, 3, 4, 5]`.
- First mutation: `SetSnapshot(bytes=[9, 9])`; `BinaryDiff::between` produces a splice removing five bytes and leaves `mid` with length 2.
- Second representative mutation: `ReplaceByteRange { offset: 1, remove_len: 2, insert: [0xAA, 0xBB, 0xCC] }`.
- Applying its diff to the two-byte `mid` must reject because `1 + 2 > 2`, producing the observed `mutation.apply.invalid-range` error.

This is a test-fixture domain-precondition error. `MutationDiff::apply` intentionally rejects a splice whose removal exceeds its current buffer in `🧬️schema/🔺️diff/🦀️.rs:71-75`; the nearby `apply_rejects_invalid_splice_without_mutating_base` unit law explicitly preserves that strict behavior. The JSON schemas can require non-negative offset/removal length but cannot encode the runtime-dependent `offset + remove_len <= current bytes length` relation. Existing direct diff-algebra unit fixtures use valid sequential pairs and do not contradict it.

A small standard-library replay of the exact four `demo_mutation_cases` reproduced one invalid sequential pair (`set-snapshot → splice`) and fifteen valid pairs. It found no absorb output mismatch or invalid merged splice among those fifteen. Raw receipt: `🗑️generated/binary-absorb-replay.txt`.

The smallest correct repair direction is to make the Cartesian law assert absorption only when the sequential second application succeeds, for example by matching `d2.diff().apply(&mid)` and continuing on its error. This matches the `DiffAlgebra` contract, which requires equality whenever sequential application succeeds. Do not clamp the mutation's range or make `BinaryDiff::apply` accept this pair: that would weaken the intentionally strict domain validation and change observable mutation semantics.
