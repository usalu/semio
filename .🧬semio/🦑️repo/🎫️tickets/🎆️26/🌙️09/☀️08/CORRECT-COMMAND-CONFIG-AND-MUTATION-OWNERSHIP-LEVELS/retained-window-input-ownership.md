# Retained Window Input Ownership

The framework `ArtifactOwnedToolJobContext` now retains the exact captured `WindowTransientSnapshot`. The snapshot exposes its immutable generation along with concrete window id, kind, and typed state. Operation context identity includes a presence marker followed by length-delimited window id/kind and generation; a checkpoint from another window or an older window generation cannot match the new context digest. This supplies the previous-state read needed by retained gesture reducers without copying window data into app config.

## Validation

The shared JSON fixture and independent Ajv uniqueItems evaluator passed for four distinct concrete owner/generation tuples and rejected a duplicate. A native test checks those tuples, distinct context digests, equality under a snapshot clone, and distinction from an absent window owner. The first two native attempts exposed compile errors corrected before the successful third run.

Native run 3 completed successfully: one test passed, zero failed, with the expected `[DEBUG]` owner/generation receipt. The SDK now also captures WindowConfig identity (window id, kind, generation, revision) in ARC-CONTEXT-4. The public `window_transient_snapshot` query is exercised by the Wires retained-window lifecycle test, which subsequently passed within a five-test native run.

The root permanent script, root Nx targets, authored/generated launch configurations, and ticket isolated targets register `retained-window-input` and its oracle-only variant. Lowpoly's sole direct test constructor now supplies the required optional captured window input.
