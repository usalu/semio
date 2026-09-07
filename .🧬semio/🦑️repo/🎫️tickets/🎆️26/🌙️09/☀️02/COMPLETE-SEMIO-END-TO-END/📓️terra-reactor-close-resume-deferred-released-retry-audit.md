# Reactor Close: Resume, Deferred Surface, and Released-ACK Audit

Read-only source audit on 2026-09-06. No build or runtime test was run.

## P0 — close can leave an old task resume to execute on a replacement lifetime

PendingResume retains only a numeric instance, not the allocation-bound NativeCloseKey
([reactor source:615](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️.rs:615)). Close reservation snapshots the queue length
([:872](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️.rs:872)), then drains that
finite snapshot before it advances the still-executable task table
([:924](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️.rs:924)). A task that settles in
the later task-close step removes its record and appends a new numeric resume
([:798](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️.rs:798)). It was not in the
snapshot, so resumes_complete remains true.

Normal draining asks only whether some *currently live* lifetime exists for the numeric instance
([turn source:857](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs:857)); native_close_key returns the key
of the current live slot, not a key held by the resume ([turn:42](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs:42)). Thus the old
command/emit can dispatch into a later InstanceOpen reusing the numeric id after final ACK removed
the old slot.

The in-tree direct task producer is currently cfg(test), but this is still an executable native
law path. The production checkpoint replay producer is also raw numeric
([reactor:1010](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️.rs:1010)); it must not be
treated as an old live task when fixing the first route.

### Smallest owner-preserving repair

1. Replace the live-task queue owner with a PendingResume containing NativeCloseKey, meta, and
   outcome. Capture the key on task admission, carry it in TaskRecord, and make close matching
   exact-key based. Completion uses the captured key, never a current numeric lookup.
2. Drain compares the queued key to the exact current live key. On mismatch or no live slot,
   retire/drop the owned resume; do not requeue it. The current branch can rotate permanently
   stale entries forever.
3. Preserve restore semantics explicitly: only restore_now may enqueue a private
   Restored(instance) owner. Its first dispatch binds the then-live exact key; it is not a
   completion from a prior lifetime.
4. The snapshot scan may remain a bounded optimisation. After task cancellation, either tail-sweep
   the captured key or rely on the stale-key retirement rule; the latter is enough for safety.

### Required real-native law

With the existing TestRuntimeApps lifecycle driver, queue a controlled old-lifetime completion
after resumes_complete became true, finish its close, reopen the same numeric instance, and drain.
The replacement must receive no plugin_resume_task, mutation, effect, or frame from old bytes and
the old queue entry must be gone. A new resume carrying the replacement key must dispatch once.
Add a restore row proving a checkpoint restart waits for a new live key rather than being dropped
as stale.

## P1 — deferred surfaces are not represented with their exact lifetime

PatchTracker is public and public defer/take_deferred_ready retain only SurfaceId
([patches source:330](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🩹️patches/🦀️.rs:330)). DirtyPollOwners likewise
owns numeric instance plus SurfaceId ([turn:56](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs:56)). The sole production
deferral call originated after a current-key render reservation attempt ([turn:633](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs:633))
but discards that key.

I did not find a current direct stale-publish trace: close drains deferred rows before declaring
patch close complete ([patches:731](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🩹️patches/🦀️.rs:731)), and a deferred
surface taken into the per-turn dirty list is checked against a live key before reservation/render
([turn:633](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs:633)). Final ACK and replacement open cannot share a turn
because one lifecycle command is admitted per turn.

The API is still an identity regression: another caller can enqueue a numeric surface without
proving a lifetime key, while close removal matches only the numeric prefix.

### Small containment change and law

Make the production API crate-private. Replace its raw value with
DeferredSurface { key: NativeCloseKey, surface: SurfaceId }; require the key in defer, retain it
through take_deferred_ready, and make DirtyPollOwners own the same key. Immediately before
reserve_mounted, compare that key to the exact current live key. Patch close removes only a
matching key; surface-text equality remains coalescing only.

Add a tracker/native turn law with old/new NativeCloseKeys for the same instance and surface: old
close drains only old deferred work; old dirty work cannot render after reuse; new-key deferred
work survives. This is P1 hardening, not evidence of an already reachable publish bypass.

## P1 — successful final Retired ACK cannot be retried after slot removal

NativeLifecycleRegistry.finish_turn consumes a successful final ACK and immediately removes the
slot ([lifetime source:300](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🚪️lifetime/🦀️.rs:300)). A replay of the exact final ACK then
fails with ACK lifetime absent ([turn:130](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs:130)). No native owner is
lost—the terminal owner was already empty and released—but an uncertain/lost caller response
cannot receive idempotent success.

The real lifecycle helper retries only until slot removal ([runtime test:47](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🚪️lifetime/🧪️tests/🧵️runtime.rs:47));
there is no post-removal retry law. The foreign/reopen test correctly requires an old Captured
receipt to fail after numeric reuse ([runtime test:99](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🚪️lifetime/🧪️tests/🧵️runtime.rs:99)), which must
remain true.

### Bounded retry witness

If lifecycle ACK delivery is retryable, add a fixed ownerless ReleasedAckTombstone table to
NativeLifecycleRegistry, indexed like lifecycle slots and containing the complete final
ActorInstanceLifecycleAck. Before removing a Released slot, move its exact final ACK there.
ACK admission then has three outcomes:

- a current slot stages the normal ACK;
- no current slot plus an exact tombstone is an accepted duplicate with no lifecycle work/output;
- any different receipt, including an old final ACK after a replacement is live, faults.

admit(open) may clear a colliding tombstone only after validation and immediately before installing
a fresh slot. This gives a bounded retry window without allowing an old ACK to address a new
lifetime or blocking reopen. Tombstones retain no app/lease and do not weaken the Drop assertion.

The native law must complete a real Retired ACK to slot removal, replay it once successfully with
no new app/patch/effect, reject a byte-different final ACK, then reopen the same numeric id and
reject the old final ACK. If the protocol intentionally specifies at-most-once ACK delivery,
document that instead; it is not compatible with a generic transport retry contract.

## Scope

This report is restricted to close-drain, deferred-surface identity, and post-removal final-ACK
retry. It does not reassess the pending patch receipt work in progress.
