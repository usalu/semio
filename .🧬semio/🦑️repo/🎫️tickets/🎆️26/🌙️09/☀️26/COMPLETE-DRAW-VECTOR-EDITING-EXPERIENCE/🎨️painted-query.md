# Painted Path Queries

## Current Scope

The original complete Draw editing goal and ticket remain active. This stage provides a schema-first Rust/TypeScript painted query over borrowed source segments. Existing gesture pointers still use PathHitCursor. Matched completed-scene integration, host waiting/fairness, region selection, handle admission and browser end-user journeys remain required. No finished end-user selection experience is claimed here.

## Implementation

The query admits one fixed-shape source segment per work unit and calls the existing first-party path flatten and stroke outline kernels under one-unit grants. Actual flatten and stroke owners transfer into their genuine retirement frontiers. Fill winding includes implicit closure; stroke geometry preserves authored closure, butt/round/square caps, miter/round/bevel joins, the renderer miter limit of four, dash patterns, reflection and nonuniform transforms. Cursor output includes containment and world bounds in `[minX,minY,maxX,maxY]` form. World tolerance expands proximity to the resulting painted boundaries, rather than scaling the stroke to a circular envelope. Curve precision is controlled by world-space flattening tolerance. Opacity/paint alpha and ancestry are policy decisions for the future scene query, outside this geometry-region kernel.

Source geometry is not cloned in the constructor. Each admitted segment is copied as a fixed-shape value, and source callbacks stop after admission. TypeScript also owns the bounded query tuple and dash copies. The native flatten child consumes its actual segment buffer; TypeScript retains its actual admitted source until the borrowed child reaches terminal. Contours and stroke polygons move directly from genuine child output. Cancellation and failure retain unpublished owners until granted cleanup. Normal completion also waits for real private cleanup before allowing a result; consuming retirement moves only a completed scalar result.

Rust numeric contour buffers retire as shallow buffer headers; TypeScript owned point-array references retire individually, then their collection headers. This is structural ownership accounting, not a byte or allocator-latency bound. No whole mounted-query budget or producer constructor admission bound is inferred from these tests.

## Test-First Evidence and Validation History

The registered Draw TypeScript test first failed on the absent production painted query module (session 16555, exit 1; 562 existing tests passed, one skipped). Shared neutral cases precede production implementation.

The first implemented full TypeScript run (29853) passed the new region, real-child cancellation and unsafe-grant laws, including 51 independent SVG samples, but ended with 592 passed and one unrelated PDF oracle failure: the newly named oracle output folder contained no PDFs. The existing actual native-produced `pdf-compositing` assets were then selected. No PDF check was disabled.

The first native command (25158) stopped before Draw compilation because the concurrently edited Store factory signature and Stdio routing implementation disagreed. Read-only revalidation found the current sources already agreed on the two-argument preflight. Neither file was edited here. The next native run (47305) stopped before Draw at two Store parser mutable-borrow errors; the current source already separates both presence-byte reads before calling the parser. Again, no Store edit was made.

The next TypeScript run (31263) passed all 593 enabled assertions tests, including the PDF oracle, but strict production typechecking found the new Point alias incompatible with first-party readonly Vec2 tuples. Point now accepts readonly tuples; inputs remain copied and owned at admission. Curved path, world bounds and genuine failed-child retirement cases were then added.

Current native run 74707 uses the original 1,800,000 ms build budget and a fresh ticket-local PDF output directory. Current TypeScript run 40373 includes strict production checks and enabled independent PDF/SVG oracles. Exact handles are retained until terminal. Results are pending and must not be reported as green until observed.

## Resolved Source and Real Mounted Runtime

Six new neutral points exercise a Boolean difference hole, an encoded PNG trace hole and a traced Boolean operand under its actual translated transform. TypeScript prepares each genuine source through DocumentVectorJob, transfers and retires its real producer, borrows its immutable completed segments, then retires the actual output plan. Independent authored SVG pixels check those same six points under grants 1, 7 and 4096. The new `from_prepared`/`fromPrepared` constructor takes the actual resolved path leaf's transform and paint semantics and rejects unresolved/non-path content.

A native registered mounted-registry test independently advances the real GeometryJob with one-unit fuel while queries report Pending and expose no unfinished plan. Ready queries borrow the exact actual complete segment pointer on every step. A real event-sourced deletion and prepared replacement then make an interrupted query Stale; it consumes its actual flatten child and real source lease/plan owners retire to empty. This is a genuine mounted source/geometry law, not the still-missing editor InteractiveJob fairness/browser law. The query does not advance the producer itself.

Native 7854 completed the full registered Draw suite: 557 passed, zero skipped, exit 0, 22.676 seconds of assertion execution, 53.5 seconds overall. All 72 region/grant cases and 30 cancellation/grant cases executed in Rust; the six mounted samples exercised 18 actual source/query lifecycles. The existing demo archive door also passed in 2.070 seconds on this run; previous slow observations remain historical and no archive performance change is attributed to this geometry kernel. A focused current-source mounted follow-up is checking the strengthened interrupted-real-child assertion.

The 74707 native compile reached Draw and revealed two wrong new retirement error conversions (PathFlattenError/StrokeOutlineError versus `str::to_string`) plus the pre-existing mounted deletion law's obsolete fourth Store apply_one argument. Error mapping now uses each actual error's Display, and both old/new deletion laws call the current three-argument API. No Store implementation was changed.

The resolved TypeScript test first refused raw neutral floating source numbers because persisted Drawing values require canonical first-party binary64 fields. Its input lifting now follows the existing vector fixture convention. The next run (67048) passed all 595 assertion tests and 18 resolved SVG comparisons but strict production found fromPrepared's union guard was not narrowing through an arrow-function never helper. It now throws directly before accessing path paint fields. The current 71875 run uses freshly regenerated native PDF assets and all strict checks; terminal result is pending.

## Next Integration Decision

The current pointer retains a bounded curve stack and declares a fixed gesture byte budget. Replacing it directly with a per-hover flatten/stroke owner would allocate potentially large private buffers repeatedly and invalidate that declaration. Completed-scene preparation should retain painted contours and world bounds once per revision, under the existing producer's progress/cancellation ownership. Picking can then borrow those completed regions and spend one edge per query unit without rebuilding strokes on every hover. Extend genuine scene-plan retirement for those cached contour owners, preserve the exact source lanes and ancestry relation, and migrate generic/trace point, marquee/lasso and handle admission together. Opacity, authored point IDs and semantic text extent policy remain explicit. The original complete editing goal remains active.

## Terminal Current Verification

TypeScript 71875 exited 0: all 595 tests passed, zero failed, 2,238,215 assertions across 43 registered roots, strict production including the new painted query and resolved-leaf constructor, enabled independent SVG/PDF oracles. The new query laws matched 69 independent SVG region samples and 18 genuine resolved Boolean/PNG-trace SVG samples. All 30 phase/grant cancellation cases reached empty real child retirement; invalid grants refused source reads and the failed flatten child was retained then retired. Fresh current native PDF output contains the same 38 byte-identical files as the prior oracle assets. The final TypeScript gate consumed that newly generated directory.

Native full 7854 exited 0 with all 557 registered Draw tests passed, zero skipped. Current focused native 93567 exited 0 after 42.6 seconds: the one strengthened mounted law passed, 556 other tests filtered. Its eighteen actual mounted source lifecycles each reached a real flatten-child interruption, then refused a genuinely replaced source and advanced eleven actual structural cleanup units, exceeding the eight parent headers. Separate actual producer waits were 1,376 units for Boolean difference, 759 for encoded PNG trace, and 2,819 for the traced Boolean operand. Every completed query borrowed the exact source segment pointer under all four captured source lanes; query and source owners reached empty terminal states.

Every launched validation handle in this stage is now terminal, including the earlier red/prerequisite/typecheck attempts. No live handle, timeout, process or lock was interrupted. The full suite preceded a focused rerun because the mounted interruption assertion was strengthened after initial source compilation. No broader repeat follows the passing current checks.

The current stage authored eighteen paths, listed in [the scoped catalog](📎️painted-query-files.md). Scoped `git diff --check` passed. Native/TypeScript source runtime diagnostics confirm this geometry boundary, not activation of the new query by end-user gestures. The ticket and full goal stay active; cached prepared painted regions, query byte admission, actual editor host fairness and end-user/browser integration remain required.
