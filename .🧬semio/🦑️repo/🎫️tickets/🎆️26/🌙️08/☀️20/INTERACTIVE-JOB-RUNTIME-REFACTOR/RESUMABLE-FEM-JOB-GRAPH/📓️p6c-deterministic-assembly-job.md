# P6c Deterministic Assembly Job

## Result

The FEM global-system assembly boundary now runs through a resumable `InteractiveJob` while preserving the batch `assemble_system` API as a driver of that same implementation.

## Implementation

- `AssemblyJob` separates validation and DOF planning from fuel-bounded element preparation and local-matrix cell emission.
- Element triplets are accumulated in worker-local partition buffers and merged by a stable global sequence, so the assembled matrices are byte-for-byte independent of partition count.
- `AssemblyPreview` reports the active stage, completed element count, triplet counts, and stable assembled-element identifiers.
- Freshness and cancellation are checked before every state mutation.
- Checkpoints encode a compact resume cursor: schema version, model signature, total/completed elements, and partition count. Restore validates the signature and deterministically replays the completed prefix through the same bounded steps.
- The original checkpoint design serialized all internal triplets. Its timing test measured a 33.104 ms step and therefore failed the 8 ms ceiling. The compact resume cursor removed that unbounded serialization path; the repaired debug and release timing gates pass.

## Verification

- Debug focused harness: 4 passed, 0 failed in 0.15 s.
- Release focused harness: 4 passed, 0 failed in 0.04 s.
- `wasm32-unknown-unknown` harness check: passed.
- Exactness gate: partition counts 1 and 7 produce identical full and free stiffness matrices, force vectors, DOF maps, and permutations.
- Resume gate: immediate checkpoint/restore is byte-stable and the resumed result exactly matches an uninterrupted run.
- Interactivity gate: 512-element assembly with one fuel unit per step stays below 8 ms in debug and release.
- Safety gate: stale and cancelled steps return without mutating job state.

## Product Boundary

The isolated harness proves the assembly implementation and its native/release/wasm boundaries. The full FEM product package still has a separate, pre-existing de-async compiler wall across element, editor, view, configuration, presence, and schema modules. That product repair is tracked by the active Phase 1.5 sweep and must be green before Phase 6 can close.
