# P5c Mounted Layout and Text-worker Repair Contract — 2026-08-23

## Status

**RED contract prepared for implementation.** This read-only coordinator census narrows the next
layout/text packet against the current shared tree. It does not accept P5c or Phase 5. No Cargo, Nx,
Wasm, browser, runtime, timing, or network gate was run.

## Exact Live Reachability

`Ui::step_layouts` has no non-test caller. The mounted renderer reaches
`Interpreter::render_ui_node`, which calls `Ui::{set_theme,apply_tree,set_viewport,frame}` and then
composites the last retained draw list. A dirty layout therefore remains queued but is never driven
by the product route.

Mounting the existing method verbatim would still fail:

- `render_ui_node` calls `set_theme` on every render. `set_theme` unconditionally clones every window
  id, sets every `layout_job` to `None`, marks every root dirty, and requeues every window even when
  the copied theme is unchanged. A multi-turn job cannot survive the next render.
- `set_viewport`, changed `apply_tree`, cancellation, and completion also assign
  `layout_job = None`; any retained vectors/maps/text/results are ordinarily deep-dropped.
- `UiWindow::revision` advances with `wrapping_add(1).max(1)`, so a stale generation can alias a new
  tree after overflow.

## Current Cursor Defects

1. `LayoutJob::new` owns dynamic `Vec` and `HashMap` collections without pre-admitted operation,
   node, string, glyph, result, page, or process-byte credits. Growth occurs after the tree is
   already retained.
2. `LayoutJob::step` has a stage-fallthrough `loop`. `collect_node` can unwind an arbitrary retained
   depth in one call and later reserves the entire preorder result length in one opportunity.
   `shape_text` may skip arbitrarily many exhausted/empty text nodes before returning.
3. `measure_fallback` calls `FontAtlas::measure` on a whole string. `arrange_fallback` clones a whole
   text value twice and measures it again. The shaped atlas path enters parley/swash and rasterization
   through an opaque per-character call, while `FontAtlas` itself owns large pixel vectors and a
   dynamic glyph map. None of those owners is transferred through a bounded worker authority.
4. The production path intentionally disables the retained taffy solve and always falls back. The
   fallback's `HashMap::insert/entry` and result pushes have no fixed backing. Standard-library layout
   guesses or decorative accounting pages are not acceptable.
5. `publish` loops over every result, clones cached text values, and mutates live tree nodes one at a
   time. There is no exact tree revision/generation recheck at publication and no atomic last-valid
   geometry swap; readers can observe a partially published layout if this is naively mounted.
6. `UiLayoutStep` and all three lane queues clone/store dynamic `String` window ids. Theme changes and
   lane changes scan/retain entire maps/queues. Surface and queue admission/close are missing, so the
   tested weighted wheel is not a fixed multi-window authority.
7. The current cursor runs wherever its caller runs. It is not submitted to the single process
   `WorkerPool`, has no mounted worker session, and has no rejected/terminal take-resume/close path.

## Required Implementation Boundary

This packet depends on independent acceptance of P2a1's universal retained worker-session and paged
outcome authority. Reuse that one scheduler; do not add a layout pool, private runtime, blocking
mutex/channel, self-requeue loop, or UI-thread terminal drain.

Owned files should stay within UI-WGPU `engine.rs`, `flex.rs`, the minimum text-owned interface,
`Interpreter/🧊️component.rs`, the existing root `📜️script.ts` verifier region, fixtures, and this
ticket report. Do not absorb paint/tessellation/GPU submission or the live plugin reactor reconcile
packet.

## Required Retained Design

### Admission and immutable input

- Give every surface a fixed slot and non-wrapping generation. Admit window id, tree/layout-relevant
  nodes, strings, style fields, text clusters, output records, glyph requests, pages, and process
  bytes before transferring them to the worker authority. The admitted fixed/page collections must
  be the actual storage, not std maps/vectors accompanied by unrelated pages.
- Prepare the layout input through a persistent mounted census/copy cursor: at most one exact node,
  string, cluster, or <=16 KiB page per UI/frame grant. Do not clone or serialize the whole `UiTree`
  before admission. Rejection returns the exact source and leaves the last-valid layout/draw active.
- Coalesce newer theme/tree/viewport revisions by durable supersede intent. Equality-check the theme
  before invalidation. Exact generation exhaustion permanently retires the slot; never wrap or
  saturate into an alias.

### Worker turns and text shaping

- One worker opportunity advances at most one stage transition, traversal edge, node, text cluster,
  glyph request, measurement fragment, result record, or bounded page. Remove fallthrough/nested
  scans and whole-result reserve/publish operations from production reachability.
- Keep parley/swash behind the owned text interface for this phase. Invoke it only on a bounded
  retained cluster/glyph unit on the worker. Retain its output in fixed pages and preserve progress
  and cancellation before and after the opaque call. No live UI-thread whole-string measurement,
  glyph rasterization, or atlas-vector clone is allowed.
- A glyph/atlas candidate is a generation-tagged retained output reference. P5d/P3 atlas upload
  consumes its pages later; P5c must not upload a whole atlas or mutate UI-owned atlas storage from a
  worker.

### Atomic publication and close

- Accumulate geometry/text results in an inactive fixed/page layout snapshot. Publication rechecks
  surface id, tree revision, theme revision, viewport revision, operation generation, cancellation,
  and result completeness, then performs one O(1) last-valid snapshot/generation swap. Paint and
  event hit testing must read that same accepted snapshot, not partially rewritten `UiNode.layout`
  fields.
- Newer input abandons the candidate but retains it in a close cursor. Rejected, stale, superseded,
  cancelled, faulted, completed, receiver-dropped, window-closed, and application-closed paths all
  expose exact take/resume and one-owner/page `close_step` semantics. Terminal-empty is witnessed
  before slot and process credit return; ordinary `Drop` may not recursively free retained graphs.
- The mounted renderer performs at most one session poll/submit/publication/close opportunity per
  frame transaction opportunity, prioritizes fresh input, and requests a wake only on a meaningful
  ready/terminal transition. Last-valid draw/layout remains interactive while work is pending.

### Fixed multi-window lanes

- Replace dynamic string queues with fixed surface tokens and fixed per-lane rings. One token exists
  in at most one lane; lane change is O(1) remove/reinsert through owned indices, not three retain
  scans. Max/max + 1, window close, generation exhaustion, and quiet wake must preserve exact slots.
- Preserve the existing deterministic weighted lane order and prove interactive surfaces cannot
  starve visible/background surfaces under resize and effect storms.

## Required Fixtures

- production reachability fails if `render_ui_node` no longer mounts one worker-session opportunity;
- unchanged theme does not cancel/requeue; changed theme, tree, and viewport supersede exact older
  generations without deep drop or stale publication;
- node/string/cluster/glyph/result/page/surface/lane/process caps at max and max + 1 with exact owner
  identity on rejection;
- deep and wide trees, thousands of empty text nodes, multi-page Unicode clusters, low fuel at every
  stage, cancellation before/after an opaque text call, and output requiring many turns;
- stale/duplicate/ABA publication, every revision at `u64::MAX`, receiver/session/window/application
  drop in every phase, checked-out terminal handback, interrupted close, and one-page close;
- atomic last-valid geometry: pointer/generation remains unchanged until complete, then changes once;
  event hit testing and paint observe the same accepted snapshot;
- native and Wasm-shaped 1/2/4-worker schedules, resize/effect/input storms, quiet wake, lost-wake
  race, and per-stage watchdog/timing evidence below 8 ms.

## Permanent Verifier Requirements

Add faithful mutations for zero production driver, UI-thread stepping, unconditional theme reset,
wrapping generation, whole tree/text clone, post-allocation admission, std-layout/decorative-page
accounting, stage/nested loop, whole preorder reserve, whole-string measure, UI-thread atlas mutation,
bulk publish, partial live-tree mutation, dynamic string lanes, bulk job drop, missing freshness,
missing take/resume/close, and a second scheduler. Every mutation must be rejected before baseline
success is considered evidence.

P5c and Phase 5 remain open.
