# Bounded Fill Runtime Plan

## Current Evidence

Retained fill-budget.json describes a256×256 exact RGBA fill under real interactive component grants (fuel256,32items,32768copybytes,16MiBcapacity,1MiBrelease), at least65536paid work and256progress turns. Current work.step ignores cx, returns preparation then completes a monolithic reducer. The model flood_fill allocates visited/stack and traverses all pixels; lowpoly_paint_tick additionally materializes/clones full pixels and scans the whole diff. Current fixture has source-level reasons to fail its work/progress assertions, but no actual Native red result has been received. Root owns Native execution.

The existing test resets operation IDs and constructs a fresh manual StepContext per turn. Verify owns conversion to persistent OriginalJobFixtureCaller/decoder with real cancel token, and separate prefix cancellation/checkpoint coverage. Its old name alone does not establish those outcomes.

## Kernel Owner And Contract

Use framework 🔲️pixels/✍️editing/🪣️flood with Rust+TypeScript implementations and one language-neutral fixture. Existing framework flood_selection is domain-neutral but synchronous; PixelEditJob bounds ordinary per-pixel edits, not flood traversal, and cancel currently drops buffers immediately. Neither is a paid-close implementation to copy blindly. Reuse existing validate_extent and exact four-neighbour/tolerance/selection semantics.

A cursor is metadata-only at birth. Explicit phases quote allocation, initialize seen+coverage in bounded pixel slices, seed, traverse the four-connected queue, and complete. At most one enqueue per pixel proves queue capacity≤pixel count. Native workspace vectors reserve bounded known capacity before incremental initialization, avoiding a whole-image zero-fill domain loop in birth. Source RGBA is borrowed immutable input; implicit white Lowpoly layers need a uniform-color source view rather than materialized_pixels cloning. Optional selection coverage/tolerance remains domain-neutral. Maximum pixel visits and copied bytes per advance derive from actual remaining work and physical grant. Source bounds, dimensions and seed refuse before allocation.

The pure cursor reports actual work/bytes/capacity and has a deterministic logical position independent of turn partitioning. Lowpoly wraps each phase in existing original construction/receive_original_retained_work APIs; no manual receipt after unbounded work, synthetic fuel count, or larger turn budget. Allocation failure retains initialized original buffers for paid closure.

## Lowpoly Integration

Retain a fill workspace only for paintFill/fillBucket, selected before lowpoly_retained_reduce. Preserve exact UV clamp/default-center and active object/layer semantics. Initialization, flood traversal and row-major sparse delta extraction must all yield; do not fix BFS while leaving full materialization/diff scan as terminal monolithic work. Publish one EditPaintLayer only after completion; no snapshot/gesture/history publication during progress or cancellation.

Output runs must preserve sparse changed ranges because writing unchanged gaps would broaden event/undo effects in multi-user history. Store preparation currently caps EditPaintLayer at4096runs (editor/📬️store-preparation/🦀️.rs:57). A256² topology can have more sparse runs; settle this exact mathematical output capacity as a prerequisite rather than silently coalescing unchanged pixels or refusing valid bounded texture geometry. Individual run bytes and run-vector metadata must grow under explicit admitted capacity and copy grants.

ArtifactCommandWork.extent is the semantic durable-footprint preflight ceiling, separate from per-pixel scheduling fuel. Do not blindly inflate max_work_items to pixel count: one EditPaintLayer still has its one durable mutation/inverse footprint. Work progress may span many physical turns; current host does not cap Progress turn count by extent. Charge every initialized/scanned/dequeued pixel through original work currency.

## Replay And Close

Existing LPC3 checkpoint only stores stage and identities, so it cannot distinguish mid-fill positions. Define a new authored checkpoint format without a legacy decoder. Keep exact tool/operation/generation/base-revision/context checks and append deterministic logical fill-work position/phase. Restore recreates unpublished workspace and replays to that logical position under normal fuel/grants, returning Replay while catching up. It must not serialize an entire queue or perform cold unbounded replay in restore.

Cancellation checks actual cx token before every admitted chunk, and publication freshness remains authoritative. begin_close only marks cancellation/closing; no Vec replacement/drop. Each queue/seen/coverage/pending-run owner moves through native RetainedCloneClose and existing close_original_retained_step, with accurate release/copy/capacity demand. terminal_is_empty requires every backing returned; frame release comes last. Tests must cancel after actual pixel progress and close the persistent original source, not only cancel before reducer starts.

## TDD Sequence And Prerequisites

First add focused TS cursor contract test importing the not-yet-existing kernel, run public registered repo Bun for a real red result, then implement TS cursor and get it green with a shared fixture and independent Graphlib oracle. Implement Rust kernel with the same corpus and owning pixel tests. Root alone runs Rust. Verify supplies small topology/alpha/no-op corpus and persistent retained harness.

Before Lowpoly integration: actual Rust red evidence (or explicit root go-ahead), stable borrowed/uniform source API, paid buffer construction/close seam, sparse-run bound decision and checkpoint format. Native command crate tests then Lowpoly/pixel suites must explicitly execute owning units; dependency compilation is not test execution. Final7.5ms/cancellation acceptance requires actual Native/mounted evidence; no claim follows from numeric TS budgets alone.

## Executed Kernel Milestone

Focused public command repo tools exec --light bun -- test explicit ./framework/pixels/✍️editing/🪣️flood/🧪️tests/🟦️.ts first refused missing kernel (0pass1failure). Initial independent Graphlib check needed nodeEdges traversal for undirected reachability; corrected oracle enumeration retained fixture expectations. Subsequent actual kernel milestone2pass695assertions; Light now owns approved TS grant/outcome extension and further tests, with separately reported actual missing-demands red. No Native pass follows from these runs.

Rust mirror is authored with metadata-only birth, one fallible buffer allocation per phase, bounded seen/mask initialization and BFS, borrowed-slice/uniform source identities, exact per-turn grants/quotes and RetainedWorkOutcome success/failure receipts. Native demand now bounds newly enqueued frontier work by remaining whole extent. Oversized actual capacity refuses after recording and retaining backing; failed attempts report work1. Existing test-only native allocator observes each allocation/refusal/denied-grant/close receipt. Test dependency is dev-only. Native has not executed the mirror. Optional selection/tolerance extension from earlier exploration is not implemented: current reusable cursor is exact RGBA four-connectivity, sufficient for this Lowpoly operation.

Native29416 is the one root-authorized separate paid-fill-red request; root57735 owns generic producer validation. See fill-native.md and generated polling log. At lastpoll~1040s it remained in workspace/source admission, with no compiler/testbody or assertionred. No production Lowpoly paid-fill integration has started while waiting for genuine red.

## Sparse Delta Builder Detail

The adversarial connected comb fixture fill-comb.json/schema and independent Graphlib oracle pass:32896changedpixels,32641sparseintervals, mathematical ceiling32768=ceil(width*height/2). Keep only genuinely changed pixel ranges; no gap coalescing. Replace4096 only with a layer-extent-derived validated bound, preserving one semantic EditPaintLayer.

To avoid hidden Vec growth copies during bounded delta emission: first scan completed mask/source under pixel fuel into preallocated bounded (offset,length) descriptors; then reserve exact per-run byte extent one admitted allocation at a time, fill its replacement RGBA bytes in bounded chunks, and transfer complete PixelRuns into an exactly reserved output vector. Reserve descriptor/output capacities under their own physical quote, never collect/clone the full mask/image in the terminal step. Keep all initialized prefixes in workspace until output custody transfers or paid close completes. Source/selected pixels remain immutable and unpublished.

## Terminal Publication Ownership Audit

Do not call lowpoly_tool_emit on the large completed leaf: it invokes lowpoly_tool_once, whose borrowed statechart yield_once clones every mutation. That would copy all sparse-run metadata and pixel bytes in a single terminal step. Existing domain-neutral TransactionRef::original_mint_demands/mint_original (replication mutation edit owner) supports paid metadata minting; preserve the current tool namespace/authoring actor and transfer the original leaf Vec directly to Emit::commit_transaction. No marker leaf, dummy chart effect or framework redesign is required. Freeze the authoring clock once and include its physical tick in the authored checkpoint so replay does not mint a different transaction; therefore the earlier80byte checkpoint sketch is provisional and needs the additional clock field before implementation.

Also align the actual producer checkpoint bool: build_tool_job currently passes false to admit_retained_tool_producer even though factory admission says true, so actual incoming host checkpoint would refuse before work.restore. Direct work replay tests alone cannot prove the mounted checkpoint lane. Correct the owned fill producer policy when integration is implemented and preserve cancellation/source identity through close.

## UI Gesture Fill Boundary

The direct paintFill/fillBucket commands are not the whole bucket UI. paintAt, paintStroke and CanvasPointerDown also call lowpoly_paint_step, which calls lowpoly_paint_tick under config.paint_utility=fill and therefore reaches the old synchronous model flood. A direct-command cooperative fix must not be reported as bounding every paint interaction. Reuse the bounded fill leaf preparation for those entries while preserving existing phase semantics: one-shot commits once; pointer-down/stream owns an ephemeral preview and zero persisted edits; pointer-up commits that same net delta once; abort/lost capture discards it. Prepared large delta must use an owned original gesture/transaction path, not re-enter clone-heavy chart handlers. Actual UI/native acceptance needs those entrypoint variants and mid-work cancellation after real pixel progress.

## Canonical Extent Consequences For Paid Delta Workspace
Strict Lowpoly256-square validity gives65536pixels and an exact sparse changed-run bound ceil(65536/2)=32768. A64-bit native PixelRun consists of a u32 offset and Vec header (32bytes with alignment), so an exactly reserved maximum directory is1,048,576bytes, matching the real mounted1MiB release grain. Each individual contiguous run backing is at most262144bytes; exact per-run capacity reservation followed by bounded initialization avoids Vec growth/copy spikes. Combined output capacity can exceed one release grain while every independently owned backing remains retireable. This bound must be expressed through size_of and canonical pixel count, with quotes and actual receipts checked on the native target; do not retain a guessed4096-run ceiling or coalesce unchanged gaps.

A deterministic bounded pipeline can pre-scan completed region coverage to count runs, reserve the exact output directory once, reserve one exact run buffer at a time, and append bounded pixel chunks per funded step. A no-op same-color fill can refuse to mint any semantic edit before allocating the region workspace. The final mutation must move into a paid original transaction constructor, avoiding the ordinary chart clone path. This remains an implementation plan, not an executed Lowpoly fill fix.
