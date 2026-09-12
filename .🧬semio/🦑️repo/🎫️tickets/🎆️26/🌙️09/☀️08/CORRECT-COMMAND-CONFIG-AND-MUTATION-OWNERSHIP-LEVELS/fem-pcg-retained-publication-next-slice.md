# FEM PCG Retained Publication Next Slice

## Actual Boundary

The shared sparse engine owns PCG numerical state and its preview/checkpoint/terminal serialization. FEM3D owns the resulting numerical child, retained outcome and visual field transfer. The renderer must not serialize or reconstruct solver state. Root inspected the live PcgJob::step branches: complete and preview derive whole PcgPreview vectors and encode them; checkpoint calls checkpoint_bytes over the entire PcgCheckpoint. The payload_from_bytes call then creates retained output from the already-materialized bytes in the same control step. The new stage labels identify these branches but do not repair their ownership.

This is source evidence, independent of whether native29's small production fixture completes within its unchanged wall bound. The preceding numerical source fixes and scalar law must remain intact.

## Execution Contract

Define the preview/checkpoint/terminal wire envelope and scalar ordering in a closed language-neutral fixture/schema before production changes. A retained PCG writer must use the existing RetainedJobPayloadWriter and numerical page/cursor infrastructure, with one explicit admission or scalar write per fuel. Derive negative residuals and absolute displacement directly from the borrowed scalar, without temporary reactions/contour Vecs or a whole PcgPreview.

The checkpoint writer must cover the actual CSR owner, all seven scalar vectors, configuration, stage, convergence accumulators, numerical cursors, iteration and due flags. A retained restore cursor must reserve/copy these exact owners under bounded grants and return the same next numerical state. The cold checkpoint helper can drive the same codec to completion for test/batch use; it must not remain a synchronous fallback in interactive execution. Replace all actual consumers and fixtures together, without a legacy wire adapter.

Output writer state is a distinct owner from numerical state. Zero fuel, expired deadline and pre-cancel preserve numerical cursors, due flags, writer cursor, staged bytes and physical backing. Completion consumes the due state only after the retained output can transfer. Faults preserve the first fault and retain every partial writer/input/decoded owner until close. begin_close, close_step and terminal_is_empty must include checkpoint, preview, terminal and restore owners.

Do not change the native full-child200,000 opportunity limit or8ms wall ceiling just to accommodate a broken cursor. If the correct paged publication adds bounded work, measure and report the resulting complete trace before making a separate evidence-based scheduler-budget decision.

## Required Evidence

Use strict Ajv and an independent existing byte/numeric reference for the same neutral envelope and scalar values. Cover empty and multi-page vectors, negative residual and contour derivation, exact page boundary, maximum-plus-one admission refusal, every publication cut, insufficient/zero cleanup grants, cancellation and exact physical release.

Drive actual PcgJob::step and retained restore from native tests. Preserve batch-equivalent convergence, exact checkpoint resume, coarse preview semantics, initial search-direction backing preservation and the publication grant corpus. Update old helper-based tests to explicitly close all retained outcomes and job owners. The full FEM3D route must still publish displacement, reaction and actual subspace eigen fields before exact bounded close.

The source belongs in sparse/🦀️.rs and its colocated schema/fixtures/tests. Existing root/ticket FEM routes and launch entries should include the new law, using Bun/Nx. No new script file or runtime library is needed.
