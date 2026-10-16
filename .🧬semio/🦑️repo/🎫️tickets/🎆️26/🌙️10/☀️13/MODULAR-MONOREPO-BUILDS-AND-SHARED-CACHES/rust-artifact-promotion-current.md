# Rust Artifact Promotion Revision Custody

The cache-promotion handoff now authenticates the artifact revision after acquiring the actual output reader. The compiler's stable slot remains shared across revisions; the fix does not force hits or change semantic cache hashes.

## Changed Owners

- The compiler unit producer emits a closed `{version,id,digest,mode,output}` metadata fact. The patched Native hasher carries it as `semioRustUnit`, and the task hasher retains it in the actual task hash details.
- `storage/enforcement/nx/outputs/promotion` owns the closed schema and bounded receipt/artifact reader. The actual private cache scope admits it only under live fixed-cache EX and the exact compiler output SH, with the canonical mode/slot under the real Rust image.
- The reader checks the existing receipt schema, unit/digest/mode, complete inventory, no-follow single-link regular files, each actual descriptor extent and SHA, descriptor identity before/after consumption, receipt replay and rooted directory post-recensus. Receipt bytes are capped at 1 MiB, artifacts at 4096, names at the existing 256-character path ceiling, and total artifacts at the repository's actual configured artifact limit. It streams 64 KiB chunks with Original checks and continuation.
- The patched DbCache local put checks Source before waiting and again inside the retained output reader. It verifies the output revision before TaskDetails, Native put, task-output notification or remote store. The output reader remains held through the complete retry/remote-store callback.
- Incoming remote adoption verifies the nested actual cached output against the same hashed expectation before TaskDetails or Native import. A refused adoption becomes a miss in both retrieval callers. A valid incoming result must be inside the actual funded cache root; unknown remote temporary layouts refuse.

The two runtime functions are included in the mandatory immutable authorizer contract (35 functions total). No already-installed SDK has been patched in place. A fresh normal publication and authenticated callback/Native/ordinary gates are still required.

## Genuine Interleaving And Independent Evidence

The initial neutral missing-owner-module gate was RED. More significantly, the actual existing installed DbCache.put method was executed with the real fixed-cache/output custody: A passed its precheck, waited behind a genuine output EX, and the peer replaced the slot with B before releasing it. Original `5827` genuinely failed the expected-refusal assertion because the old method resolved. No mirrored replacement condition was used to manufacture that counterexample.

The successor executes the exact new full put/remote methods parsed from the authored patch with TypeScript AST, and calls the actual private scope helper. A narrow native put/import/remote-store receiver counts the effects that must not occur. An `onWait` observer preserves the actual issued control, observes the real output wait, then the peer changes bytes only while retaining output EX. No structural control copy, clock-based race or fabricated Original is used.

Controls cover matching A/A, changed semantic digest, foreign unit, wrong mode, corrupted artifact bytes, and Source changed after the wait. Independent SQLite identity comparisons supply the neutral expectation; a real Python SQLite EX attempt observes the held output SH in the accepted callback. The incoming remote method is executed against real nested cached fixture bytes for the identity corpus. The actual patched Native/task hasher methods are also executed to verify expectation propagation without changing Native hash bytes. Exact patch coordinates pass the first-party validator and independent `diff` parser.

A deliberately thrown data mismatch initially exposed the received Original failure-drain behavior after the fixture's caught rejection. The clean production result for a known revision mismatch is now a typed data refusal (`false`) and withheld promotion inside the callback, before any effect. Actual authority, cancellation, storage and unexpected I/O failures still propagate; they are not converted into successful admission.

## Current Qualification

- Prototype final `15948`: GREEN 3 tests / 58 assertions, 28.00 seconds.
- Release final `4821`: GREEN 3 tests / 58 assertions, 26.11 seconds.
- Expanded output/helper strict `21882` / `49680`: BOTH exit zero before the final test-only parser assertions; final strict `79636` / `64825` BOTH exited zero against the final parser assertions and current production Source.
- Earlier strict failures from newly traversed old test narrowing and the Root-owned metadata-layer helper's transient missing import are retained as failures, not passes. Mechanical fixture typings were corrected; Root supplied its own coherent metadata-layer handback.

Both editor catalogs register the finite `nx-promotion` / `nx-promotion-types` prototype and release prerequisite routes. The permanent output owner already runs the same test file through its existing `📜️script.ts`/Nx target. These Source prerequisite runs do not claim newly patched installed SDK or ordinary Native compilation qualification. Real remote networking and all backend cleanup/eviction paths are not certified by these local incoming-adoption tests.

Final Source freeze: all own Originals are explicitly terminal and collected. Root owns the remaining derivative release/performance lane. This lane has not launched or patched an installed SDK; the normal current capture must include these 35 mandatory functions before the private callback, Native cache and ordinary compiler sequence.
