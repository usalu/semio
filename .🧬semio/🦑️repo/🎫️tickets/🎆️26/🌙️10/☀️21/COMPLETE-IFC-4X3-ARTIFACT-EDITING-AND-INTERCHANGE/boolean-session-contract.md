# Shared Boolean Session and Workspace Contract

The independently closed50-file standalone kernel remains the checkpoint baseline. The next session extends that single evaluator; it must not create a second Boolean implementation. Whole-expression IFC evaluation will own one session, keep one monotonic private work counter and original captured observers, and borrow that session for each actual child/kernel traversal. Standalone sync/step/async operations create exactly one such session for their own call. No reset, caller-spent counter or observer receiver exposes its grants.

## Policy and State

The schema-owned policy distinguishes maxWork, maxPolygons, BSP maxDepth and maxIntermediateValues. Existing IFC maxDepth continues to bound the expression/source stack; the BSP depth is explicitly separate. Capture each getter once and default only undefined, preserving the recent receiver/null regression laws. Work starts at zero, only increases and fails before the first over-budget turn; initial/completion observations cost zero. A session failure latches the first failure including thrown undefined, prevents resumed successful work and permits only bounded entered cleanup. Reentrant observer use of the same session must refuse; nested producer traversal is an explicit internal loan, not callback admission.

IFC will add a closed Boolean policy to captured geometry control rather than reinterpret maxValues as CPU work. The Boolean request owner captures it before any progress callback. Every expression frame/query/leaf turn promised by the whole-expression work contract must debit the same session. A per-node standalone factory with a fresh default remains forbidden even when it happens to pass simple geometry examples.

## Live Mesh Values

A private workspace tracks live logical coordinate/index values; normal data is counted only when retained. Handles have session provenance and lifecycle identity, cannot be forged or moved between sessions, and never represent native physical funding. Reserve before backing growth or exact leaf allocation, release on last private use and transfer an output's owned reservation rather than charging it twice at parent adoption. Neither a caller-supplied value count nor a free mesh object is a completed reservation certificate. Closed/session-failed cleanup drops original frame/mesh references and reservations without resetting the work grant.

Count simultaneous child meshes, normalized input copies, generated output/compaction/index rows and any retained output mesh overlap. Auxiliary BSP polygons/split vertices, vertex maps, candidate arrays and frame metadata remain bounded by original work/polygon/source/depth ceilings; state precisely which logical carriers belong to the mesh ceiling. Do not silently claim this is complete physical allocation accounting.

The final central publication maxValues ledger remains cumulative across completed products and distinct from workspace residency. A small final result may require large workspace; exposing the separate grant makes that tradeoff adjustable. Empty regularized results have an explicit outcome and no degenerate triangles. Existing partial-product output behavior remains intact on later refusal.

## Required Neutral and Actual Boundaries

Use language-neutral reservation/work programs for exact admission, one-over denial, release/transfer, duplicate/stale/cross-session handles, failure latching and close; corroborate exact integer arithmetic and maxima through actual SQLite behind the ticket test interface. Apply those same owner primitives in the real kernel before considering the session complete.

Measure actual public Block60, cone645 and cylinder1284 leaf allocation preflights; test exact W−1/W for nested two-operation whole-expression work and measured workspace peak−1/peak including retained siblings/copies/output. Test repeated DAG source occurrences and active-path cycles separately. Original callback replacement, receiver attacks, explicit null/noncallable policy, exact Error/undefined, original signal, real timer fairness, entered return cleanup and final publication must remain passing. Preserve current762 standalone laws and external Manifold/Three/actual30 STEP evidence.

The first finite IFC31 red is already saved and has independent exact30 full EXPRESS validation. Its expected regularized covering-difference emptiness is supported analytically/Manifold while the installed SDK incorrectly retains volume64; preserve the discrepancy. Face-contact Manifold zero-volume triangles corroborate volume only. Do not rewrite source inputs or neutral expected results to suit an external backend limitation.

This contract is work in progress during the broad145 source hold. No new session runtime API or IFC dispatcher change has been made, and the full goal remains incomplete.


## Borrowed Kernel and Reentrancy Trap

A public raw child kernel generator must not be drainable without debiting the original session. Conversely, wrapping an already controlled child inside a second controlling root must not silently charge/reset a second grant. Define one explicit transition owner: source/leaf turns debit once, each private kernel turn debits once, and the complete expression reports the same monotonic original work. Session entry/next/reservation methods must reject entry from inside an observer, while internal staged producer loans remain explicit. An observer that closes over the public session must not start/resume a child between protected transitions or obtain a raw unmetered private evaluator. Actual ECMAScript-private fields and unbound invocation remain mandatory. Reentrant refusal must latch so catching the thrown refusal inside the callback cannot resume successful engine publication.
