# P8yt Full Operation Store Seam Scout

Date: 2026-08-22  
Verdict: **The typed route cannot become resumable until the store/view capture and emit-commit seams are event-maintained and cursor-owned.**

## Preparation blockers in current source

The document and configuration snapshot owners are already shareable `Arc` values after
`refresh_cache`, but refreshing the cache can rebuild and scan command history. The other inputs are
not constant-time capture authorities:

- `ChildContentView::new` allocates a new `HashMap`, clones every string key/dialect, and awaits every
  child's revision and erased snapshot on every command.
- draft, interaction, presence, peer-presence, and transient preparation materializes or clones the
  current value/collection before the worker job.
- the command-log/history cache can walk edits, commands, and operations while extending or rebuilding
  `HistoryView`.
- several lookup APIs construct temporary `(String, String)` keys, so even reads hash/allocate
  unbounded identifiers.

Wrapping those awaits in named enum phases would not bound their internal work.

## Required preparation architecture

Make every command input an immutable operation authority captured in O(1):

1. Stores publish revisioned `Arc` snapshot roots as part of their event application, not on command
   capture. Draft, interaction, presence, transient, and peer presence follow the same owned snapshot
   interface.
2. Maintain the child content index incrementally on child open/register/change/close events. Its
   published root uses fixed-width interned slot/child authority and structurally shared pages; command
   capture clones one `Arc`, not N children.
3. Maintain command history incrementally when the command/edit event is appended or folded. Publish
   the filtered history root by revision; do not rebuild it at dispatch time.
4. Any costly snapshot/page construction becomes its own cancellable cursor job and publishes only a
   complete revision-matching root. Admission fails closed while the required root is unavailable;
   it never falls back to whole materialization.
5. Bound identifier interning and all page/table growth through fixed item/byte admission and
   incremental rehash/trie/page cursors. No ordinary resizable string-key map is permitted on a step.

## Required commit architecture

`dispatch_emit` must be split into a persistent prepare/validate job and a constant-time publication
boundary:

- cursor artifact/config/draft/child operations, effect/event/task records, labels/descriptions,
  command-log output, and every serialization step with exact item/byte credits;
- build immutable per-store event batches and next snapshot roots off-thread;
- validate kind, base revision, generation, child authority, transaction state, and cancellation before
  any exposure;
- publish one generation-validated multi-store commit candidate through a constant-time root/event-log
  swap, then enqueue effects/tasks/progress on bounded channels;
- stale or rejected candidates drop through a bounded cleanup job, never through a monolithic
  destructor.

Only this full prepare/reducer/output-validation/ephemeral/emit/expose pipeline can activate the nine
owner-local proofs. Handler-only jobs and per-await phase labels remain non-authoritative.

## Suggested serialized packets

1. Owned immutable snapshot-root interface for draft/interaction/presence/transient and peer presence.
2. Event-maintained child content root and fixed-width identity index.
3. Event-maintained history root.
4. Cursorized emit candidate builder and multi-store O(1) publication boundary.
5. `TypedCommandFullOperationJob` integration, activation, runtime timing/cancel/stale/saturation tests.

The packets share the plugin/store API and should be serialized under one Sol High owner. Importers
and reserved routes should remain fail closed until they can consume the same foundations.

No production source was modified and no runtime/build gate was run by this scout.
