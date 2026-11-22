# Typed Publication Stall Witness Audit

The witness reads the next runnable operation before and after a unit, but the unit rotates the cursor. Two runnable operations therefore produce different witnesses on every turn and never accumulate a stall streak. The process-global atomics also mix independent instances and operations; their separate swap, store and increment operations are not a single atomic transaction. Hashing the witness introduces collisions.

Track the streak on the operation that actually received a unit, compare that same operation before and after, and skip fuel-exhausted and latest-wins turns. Preserve the Genesis exemption. Evaluate the fault ceiling independently from diagnostic sampling.

Large Capsule Dream publication reports genuine progress through `ArtifactStoreOneItemAdvance::Progress` and `PreparationProgress`: the typed publication unit calls `note_publication_checkpoint`, which increments `publication_progress` when the store checkpoint changes. Blocked and refused preparation do not increment it. Private child-group and window-config refresh also increment the counter on reported progress.

Regression coverage should alternate two runnable operations, interleave instances with identical operation IDs, verify repeated unchanged checkpoints reach the fault ceiling, and verify more than 4,096 changing checkpoints remain healthy. Include no-unit turns and the Genesis exemption.
