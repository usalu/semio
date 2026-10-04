# Preparation and Vector Parent Ownership

The previous goal turn is verified progress: native and TypeScript document Boolean/trace ownership changed, complete Draw suites passed (520 native/563 TypeScript), and focused native diagnostics confirmed fifteen Boolean and sixteen trace interruption cases. All validation handles are terminal. The full editor goal stays active.

Current sources were reread at this continuation. DocumentSceneCursor still clears tree entries at completion/failure and drops nested frame groups. DocumentVectorJob still eagerly cancels children on failure. TypeScript vector success drops each privately produced child input plan without explicit cleanup. Native owns a genuine SnapshotRead, which must remain retained until every child and candidate is empty.

Next implementation composes work-granted preparation, trace, Boolean and scene cleanup and returns the actual native read only after terminal retirement. TypeScript borrows the source document and preserves its containers. Neutral fixtures and existing independent SVG/PNG evidence must validate the composed pipeline. Repo MCP ticket/goals capabilities are still absent; the existing ticket is retained. Implementation and verification are pending.

## Implemented Contract

Preparation preserves failed state, drains completed ID/asset/visited indexes one entry per unit, empties private frame groups before dropping frames and composes partial nodes/plans through ScenePlanCloseJob. Depth refusal now retains the current node rather than dropping it outside the cursor. Native preparation validates JavaScript-safe grants too.

Normal vector transitions now retire each real child and its remaining input plan before constructing the next stage or exposing final output. Failure remains private until consuming retirement; the vector cursor adopts any active child, partially drained child cursor, intermediate plan and incomplete candidate. Native complete output moves unchanged. TypeScript preserves its borrowed source containers. Structural grants are not allocator byte credits.

Native retirement keeps the genuine source read while private work drains. Its progress done=true means private work finished and the read is ready for explicit handback. terminal_is_empty stays false until take_snapshot_read hands the actual lease back; that distinction is schema-documented and tested. Borrowed native sources and TypeScript sources have no owned lease handback. Early dropping any retirement cursor is not certified; the future mounted owner must retain it through terminal state. Existing eager cancellation entry points are not claimed as mounted bounded cancellation.

The red gate 87884 exited 1 as expected: 563 existing tests passed and both new missing transfer-method tests failed. New tests cover thirteen preparation cases and twelve vector cases with explicit interrupts during each child handoff, failures, completion and previous cancellation. Full TypeScript/native verification is running.

The first implementation run executed all 565 TypeScript tests successfully, then strict checking caught a stray raster cancellation guard and readonly ownership on private frame group arrays. The raster path was restored to its previous behavior and the private frame arrays are now correctly mutable (source layer arrays remain readonly). Final verification is running; the first implementation gate is not claimed fully green.

## Canvas Consumer Revalidation

The mounted Draw canvas was directly reread again. It still recursively flattens the source, builds all DSL records and synchronously serializes them with pack_json::to_json_string into Canvas2dScene.layers_json; snapshot is explicitly None. Thus the 16 KiB Canvas2dSnapshot registry is not the current Draw carrier. It remains insufficient for a new large drawing/image snapshot, but its capacity does not explain current Draw rendering. Any new carrier decision must inspect both React/native readers and preserve admitted large resources. The scheduled complete vector result is still not consumed by this canvas.

## Executed Full Gates

Final TypeScript gate 85720 exited 0: 565 passed, zero failed, 1,828,588 assertions; strict production checks and enabled independent PDF/SVG checks passed. Native 13431 exited 0: 522 passed, zero skipped. This includes both new neutral retirement suites and the updated genuine store-read failure/cancellation test. A focused native retirement runtime gate is running to expose the actual structural and source-handback diagnostics. Scoped whitespace checks passed.

The repo MCP was revalidated beyond tool-name discovery: resources/list against server repo failed during MCP startup, with the initialize handshake connection closed. The goals resource and ticket lifecycle remain unavailable for that concrete reason. No new ticket or goal was created or closed.

Focused native 34337 exited 0: six retirement tests passed, 516 intentionally filtered. Actual diagnostics cover every one of the thirteen preparation and twelve vector cases at grants 1/7/4096, including real snapshot-read retention and exact handback. All stage verification handles are terminal. The entire editor goal is still unfinished; the next implementation must mount and consume this producer in the actual canvas/picking/bounds/conversion workflow.
