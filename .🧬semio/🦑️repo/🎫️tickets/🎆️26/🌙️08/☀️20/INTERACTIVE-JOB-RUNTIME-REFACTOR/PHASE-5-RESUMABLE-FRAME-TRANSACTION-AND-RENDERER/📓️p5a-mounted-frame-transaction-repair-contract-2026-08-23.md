# P5a Mounted Frame-transaction Repair Contract — 2026-08-23

## Status

**RED contract prepared for implementation.** The seven-stage type is a useful test primitive but
has no production constructor and does not yet own bounded ingress, publication, supersession, or
terminal cleanup. This is a read-only coordinator census; no Cargo, Nx, Wasm, browser, runtime,
timing, or network gate was run.

## Exact Reachability

Repository production census remains zero for `UiRuntime::new` and `FrameTransaction::new` outside
their defining/test region. `UiRuntime::transact` is test-only. The mounted OS renderer uses the
separate UI-WGPU `Ui`/frame-preparation route. P5a therefore cannot pass by improving a dormant
library state machine; one product host must own and drive it, or the duplicate family must be
consolidated into the mounted frame authority.

This packet depends on P2a1's independently accepted worker-session/paged-outcome protocol. It must
reuse the single process `WorkerPool`; no frame pool, runtime, terminal loop, self-requeue, blocking
mutex/channel, or UI-thread job driver is allowed.

## Current Source Defects

### Dynamic ingress before admission

`UiRuntime` owns dynamic surface/presenter, first-present, intent, handler, and wake collections.
`register_surface`, `submit_intent`, `register_custom_deferred`, and `request_wake` insert/push before
any operation/item/byte/process reservation. `input_epoch` uses `wrapping_add`, so stale work can
alias after exhaustion.

`FrameTransaction` then owns dynamic command/deferred/effect/surface/tree/reconcile/commit/output
collections. Its 262,144-item/node and 64 MiB limits are checked after work through saturating usage
counters; they do not prevent allocation or preserve the exact owner on cap + 1.

### One call can perform unbounded or opaque work

- `step` falls through stages in a `loop` until a unit reports work or the transaction publishes.
- delta drain allocates a temporary vector for one item;
- one intent dispatch may construct arbitrary commands/deferred owners and `extend` both queues;
- gateway admission clones a command and silently loses both owners on rejection;
- presence publication loops every peer and clones dynamic ids;
- one effect call, notify scan, `flush_effects`, deferred drain, and release flush can each traverse a
  complete collection;
- surface preparation drains/collects/extends/sorts the complete dirty set;
- one presenter call constructs a complete component tree;
- current reconciliation still reaches the separately rejected P5b cursor;
- publication drains every reconciler, expires/flushes all presence, retains/scans every wake, and
  moves every output vector in one opportunity.

Fuel is consumed only after these calls return, so a deadline crossed inside them cannot yield.

### Supersession, fault, cancel, reset, and Drop destroy retained graphs

All four paths use `clear`, `take`, or `None` across trees, patches, reconcilers, commands, effects,
and output owners. There is no public exact rejected/terminal take-resume authority, close cursor,
process credit return, or terminal-empty witness. `Transacted` and `TransactFault` are dynamic owner
graphs; effect-storm diagnostics collect complete pending sets before publication.

### Publication is not one atomic retained snapshot

`publish` mutates each live surface reconciler in a loop before returning the output, then performs
whole presence/wake maintenance. There is no exact operation/base-revision/generation validation at
the final swap and no immutable candidate snapshot pointer shared by renderer, hit testing, and
accessibility. Last-valid state is therefore not represented as one O(1) publication authority.

## Required Mounted Design

### Fixed runtime ingress

- Replace dynamic runtime registries/queues with actual fixed/page storage keyed by non-wrapping
  surface and input generations. Admit id/key/string/tree/command/effect/wake items and bytes before
  transfer. Cap + 1 returns the identical producer owner; commands and commits are never silently
  dropped. Preview/coalescible state may replace only through an explicit policy key.
- `u64::MAX` permanently exhausts the exact slot/epoch. Stale/duplicate/wrong tokens and ABA reuse
  fail closed without freeing the current occupant.
- New input raises a durable supersede intent. It does not clear a candidate graph synchronously.

### One semantic unit per worker opportunity

- Remove stage fallthrough. One accepted worker turn advances at most one delta field/page, intent
  field, command/deferred owner, effect notification/listener, presence peer, dirty-surface token,
  presentation node, reconciliation unit, packet unit, publication scalar, or close owner/page.
- Split presenter/effect/store/presence/dependency APIs behind owned retained cursors so P5a cannot
  hide complete work in one opaque call. P5b supplies the presentation/reconciliation child; P5c
  and P5d supply layout and prepared-render children. Parent completion is release-mode blocked
  until every exact child is terminal-empty.
- Check cancellation, deadline, operation, input epoch, base revision, generation, and capacity
  immediately before and after each opaque bounded platform/domain call.

### Shadow candidate and atomic publication

- Build every surface reconciler, presence update, wake deadline, render packet reference, hit index,
  accessibility directive, fault, and command receipt into one inactive fixed/page frame snapshot.
- Final publication revalidates all freshness and performs one O(1) pointer/generation swap. The
  renderer, input hit testing, IME/cursor, and accessibility consumers acquire the same accepted
  snapshot. A fault/stale/cancelled candidate cannot mutate the live snapshot or reconciler state.
- Newer input prioritizes a replacement transaction while the obsolete candidate enters retained
  close. Accepted commands remain lossless/exactly-once; preview overload may coalesce only preview.

### Terminal ownership

- Expose exact `take_rejected`, `take_terminal`, `resume`, `close_step`, and
  `terminal_is_empty`. Session/receiver/window/app Drop, worker panic, pool shutdown, saturation,
  stale publication, effect storm, child fault, and cancellation all converge on durable terminal
  intent and a mounted wake-safe close pump.
- One close grant releases at most one actual command, string, node, page, child handle, tree root,
  reconciler, presence owner, wake, packet reference, slot, or process credit. Ordinary Drop and
  `clear` cannot recursively destroy retained graphs.

### Product mount

- Mount one persistent frame session in the live OS/native/browser host. UI callbacks only enqueue
  fixed/coalesced input, acquire the newest accepted snapshot, submit already-prepared packets, and
  apply directives. They never present a tree, reconcile, route effects, or drive a terminal loop.
- Exact source census must show one process worker-pool owner and no product path using the dormant
  alternate transaction family. If consolidation replaces this type, delete the duplicate rather
  than leave a compatibility wrapper.

## Required Fixtures

- surface, intent, command, deferred, effect, presence, wake, tree node/string/page, patch, child,
  output, operation, and process maxima/max + 1 with pointer/page identity on rejection;
- gateway saturation without user-command loss, preview coalescing without commit/checkpoint loss,
  and effect storms requiring many turns;
- deep/wide tree, large dirty set, many surfaces, many presence peers, many wakes, low fuel at every
  stage, deadline before/after every child call, and no fallthrough;
- input arriving at every stage, exact supersession order, stale/duplicate/ABA publication,
  generation/epoch exhaustion, child-live completion refusal, and atomic last-valid pointer;
- panic/fault/cancel/shutdown/session/receiver/window/app Drop in every phase, checked-out terminal
  handback, interrupted one-page close, quiet wake, and lost-wake registration races;
- mounted native/browser-shaped resize/effect/input/multi-window storms with UI callback p99 <=2 ms,
  every worker stage <8 ms, deterministic snapshots at 1/2/4/default workers, and last-valid frame
  preservation.

## Permanent Verifier Requirements

Faithful mutations must restore zero production callers, a second pool/runtime, UI-thread drive,
dynamic ingress, post-allocation/saturating credit, wrapping epoch, stage loop, whole intent/effect/
presence/dirty-set/publish work, gateway command loss, bulk clear/take/Drop, partial live reconciler
mutation, missing freshness, missing child completion, missing take/resume/close, or separate consumer
snapshots. Each mutation must be rejected before baseline success is evidence.

P5a and Phase 5 remain open.
