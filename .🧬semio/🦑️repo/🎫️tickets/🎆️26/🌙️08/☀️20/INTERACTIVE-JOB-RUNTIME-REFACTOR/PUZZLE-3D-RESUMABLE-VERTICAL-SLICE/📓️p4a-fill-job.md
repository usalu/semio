# P4a Fill Job

## Result

Puzzle 3D fill planning now runs through the shared synchronous `semio-framework-job::InteractiveJob` protocol. The old monolithic `fill_step_one` function and decorative async surface were removed from the owned precompute subtree.

## Persistent stages and state

The job advances through `PrepareTargets`, `SelectTarget`, `PrepareCandidates`, `SelectCandidate`, `ConstructPreview`, `QueryBroadPhase`, `TestCollision`, `AcceptCandidate`, and `Complete`.

Persistent state contains:

- operation id, base revision, generation, deterministic seed/RNG, preview sequence;
- deterministic target/candidate lists and cursors;
- candidate preview and transform source data;
- ordered broad-phase ids and cursor;
- current placed-body pair and the complete nested collision checkpoint;
- accepted sequence, appended objects/attractions, applied prefix, rejection reason, and transition count.

The controller drives each transition with `drive_step`, 32 fuel units, and a 2 ms absolute deadline. Scene rebuilds, weight changes, and mesh changes cancel the prior token, increment generation, preserve the accepted prefix, and restart only the pending deterministic search state. Stale operation/generation contexts fault before state mutation.

## Preview and checkpoint contract

`FillBuildPreview` publishes generation, monotonic sequence, stage, target vortex, candidate kind, ordered broad-phase ids, current pair, sample cursor, inside count, last sample, rejection reason, search cursors, and accepted count.

`FillJobCheckpoint` serde-encodes every dynamic planner field plus nested collision RNG/sample state. Restore rebuilds placed collision entries and the spatial index from the authoritative fixture and immutable mesh/catalog inputs. Candidate cache ordering was changed to `BTreeMap` so equal runs and restored runs emit byte-identical checkpoint bytes.

Accepted placements emit `CheckpointReady`; terminal planning emits a lossless `CommitCandidate` with checkpoint state and serialized progress output.

## Tests

- Fill checkpoint encode/restore byte identity.
- Cancellation observed before the next transition with no state change.
- Stale generation faults with no progress.
- Empty fill transition below the 8 ms watchdog.
- Controller-level deterministic terminal checkpoint across drive budgets 1/2/4/8.
- Controller-level mid-run checkpoint/resume equality with uninterrupted execution.
- P4b collision adversarial/batch/checkpoint/cancellation/watchdog tests described in `📓️p4b-collision-state-machine.md`.

## Validation

- Puzzle dependency on `semio-framework-job` added through the workspace dependency.
- `rustfmt --edition 2021`: completed.
- Scoped `git diff --check`: clean.
- Static census: zero decorative async, awaits, private threads/pools, `block_on`, or `run_blocking` in the precompute subtree.
- Gate command: `CARGO_TARGET_DIR=<ticket>/🧪️target-p4 bun nx run @semio-tech/puzzle-plugin:test-quick --skip-nx-cache`.
- The exactly eight upstream `semio-framework-plugin-host` E0053 errors were repaired by synchronizing the `ClearDefaultApp` and `SetDefaultApp` `MutationKind` implementations and tests. Structured gate `CARGO_TARGET_DIR=<ticket>/🧪️target-p4 cargo check -p semio-framework-plugin-host --lib --message-format=json` exited 0; evidence is `📝️p4-host-check-20260821.jsonl`.
- Puzzle gate status after the host repair: still blocked before Puzzle compilation by its direct `semio-s-plugin-stdio` dependency. The active non-PDF stdio wall includes unresolved `semio_framework_async_macros`, E0308 window-kit future/value mismatches, E0277/E0599 de-async fallout, and stdio semantic-store errors. Command `CARGO_TARGET_DIR=<ticket>/🧪️target-p4 bun nx run @semio-tech/puzzle-plugin:test-quick --skip-nx-cache` was stopped with exit 130 after the dependency boundary was proven; evidence is `📝️p4-puzzle-test-quick-20260821.txt`. No Puzzle compile or test pass is claimed.
