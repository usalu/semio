# Terra Fresh P5b Live Reconcile Exact-owner/Liveness Acceptance Audit

Date: 2026-08-24  
Disposition: **RED — REJECT**

## Scope and method

Independent, read-only review of the P5b contract, the prior Terra RED audit, and Sol's repair
report, followed by direct reconstruction of the live ownership/census/ACK/close paths in:

- `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs`;
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🩹️patches/🦀️component.rs`;
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️component.rs`; and
- root `📜️script.ts` P5b predicate and mutation corpus.

No Cargo, Nx, Wasm, browser, network, runtime matrix, broad build, or production-source mutation
was run. No executor report was treated as proof.

## Confirmed improvements

- Production run-to-completion `SurfaceReconciler::reconcile` is test-only, while the mounted route
  reserves, commits, calls one `drive_one`, and transfers a ready patch. The scalar revision accessor
  is real.
- `mark_rejected` now obtains the fixed terminal target and successfully builds its terminal before
  `commit_generation`; this repairs the prior maximum-consumption ordering at
  `🩹️patches/🦀️component.rs:366-391`.
- The B3 capacity-first order is present. With a full terminal array, the matching terminal is
  advanced before ready/deferred/unadmitted/rejected/surface conversion
  (`🩹️patches/🦀️component.rs:424-438`).
- `UiValue::Map` no longer uses `iter().nth(entry)`. It retains the last key and calls
  `range((Excluded(last_key), Unbounded)).next()`
  (`🦀️reconcile.rs:482-511`). This avoids the prior linear preceding-entry scan, but does not meet
  the stronger no-rewalk source gate below.

## Blocking findings

### B1 — map cursor restarts a B-tree traversal every grant

`SurfaceSemanticMapPage` retains only raw pointers to the current/last key/value. It does **not**
retain a B-tree traversal stack/iterator. Every successor opportunity calls a fresh
`BTreeMap::range(...).next()` (`🦀️reconcile.rs:492-497`), which begins another tree search from the
root and re-walks its search path. Thus a wide map does not have a retained one-entry cursor with
constant bounded source work; it has one semantic entry result after a fresh logarithmic traversal.
The contract requires no per-grant rewalk and an explicit retained map-entry cursor, not merely
removal of `nth`.

The P5b predicate only bans `values.iter().nth(entry)` and requires the replacement range text
(`📜️script.ts:6319`). Its `btree-rewalk` mutation changes that text to `iter().nth(0)`
(`📜️script.ts:6404`), so it cannot discriminate this live range-restart counterexample.

### B2 — exact backing and durable ACK authority remain false

1. The map census charges the size of the small `SurfaceSemanticMapPage`, not the owned backing of
   the source `BTreeMap` (`🦀️reconcile.rs:484-511`). A B-tree's nodes and allocations are neither
   fixed storage nor allocate-inspect-admit backing. The original tree remains an unmeasured owner.

2. The hash table helpers still calculate a synthetic byte cost as `capacity delta *
   (size_of<(K,V)> + size_of<u64>)` or `capacity delta * (size_of<T> + size_of<u64>)`
   (`🦀️reconcile.rs:327-345`, `348-365`). `HashMap::capacity()` is an element threshold, not the
   byte allocation size, and those formulas omit table control bytes, load-factor slack, alignment,
   and allocator layout. They are capacity/length estimates, forbidden by B2, rather than actual
   backing credits.

3. A mounted reservation explicitly exists before tree production, but `MountedReconcileGrant::commit`
   places the complete `ComponentTree` directly into `UnadmittedSlot.tree`
   (`🩹️patches/🦀️component.rs:48-61`). The equivalent `retain_unadmitted` path also retains its
   tree before its semantic census. Therefore a wide/deep source tree and all of its backing can
   occupy the unadmitted owner without actual-capacity admission or a paged producer. The eventual
   census cannot retroactively make that source ownership admitted.

   The public `take_unadmitted` escape additionally takes the whole slot but returns only
   `(surface, tree)` (`🩹️patches/🦀️component.rs:262-266`). Its reservation is dropped on return,
   releasing the aggregate credit and handback reservation while the unmeasured source tree remains
   live with the caller. That is an uncredited producer owner and a lost generation-qualified
   handback handle.

4. ACK is accepted by `PatchTracker` solely because the revision is within the current reconciler
   range (`🩹️patches/🦀️component.rs:394-401`). The event handler invokes that acceptance *before*
   asking `PendingPatchAuthority` whether a matching owner is published
   (`🦀️component.rs:1105-1108`). `PendingPatchAuthority::mark_ack` correctly removes only a
   matching `Published` owner (`🦀️component.rs:1653-1659`), but it cannot undo the tracker update.
   A forged/early ACK for a ready-but-not-yet-published revision therefore advances
   `acknowledged_revision`; `can_begin` may then admit the next candidate even though the previous
   published-credit owner was never acknowledged. This violates the fixed pending/published ACK
   authority and stale-ACK requirement.

The predicate recognizes only the *presence* of `pending.borrow_mut().mark_ack`
(`📜️script.ts:6342`) and its only ACK mutation deletes that call (`📜️script.ts:6408`). It has no
ordering or early/stale-ACK mutation. The isolated source probe recorded the live tracker-first
order and confirmed that such a mutation name/check is absent.

### B5 — public ordinary Drop still bulk-destroys populated ownership

`SurfaceReconciler`'s ordinary `Drop` releases its credit and handback reservation
(`🦀️reconcile.rs:186-194`) and then Rust automatically destroys its populated `retained` and
`key_index` maps wholesale. `SurfaceReconcileReadyPatch` does the same for a populated `UiPatch`:
its `Drop` releases credit at `🦀️reconcile.rs:1744-1749`, after which the patch and every operation
are recursively dropped. Neither route hands the owner into the fixed registry nor advances one
owner per close grant.

This is reachable without any stale/corrupt token: `SurfaceReconcileTerminal::try_from_reconciler`
can return the exact reconciler on fixed-registry saturation (`🦀️reconcile.rs:2118-2137`), and an
ordinary caller drop then bulk-destroys it. Likewise `take_ready_patch` exposes a public ready owner
(`🩹️patches/🦀️component.rs:341-355`) whose ordinary drop bulk-destroys its patch. The existing
`public_drop_handback_is_lossless_at_terminal_cap_and_plus_one` fixture creates only empty
`SurfaceReconciler::new(...)` values (`🦀️reconcile.rs:2746-2765`), so it does not exercise a
populated returned reconciler or a checked-out ready patch.

There is also no saturation fallback in `handback_surface_reconcile`: it takes the reservation and
silently returns if it is absent, out of range, stale, occupied, or otherwise mismatched
(`🦀️reconcile.rs:1679-1687`). In every false branch the boxed state then drops. A lossless public
handback must retain that exact state durably on every failure path, not depend on an asserted
invariant while the fallback is destructive.

Finally, `SurfaceTreeRetireCursor::step` writes `frames[self.depth]` after taking children without
a depth guard (`🦀️reconcile.rs:1465-1507`). An unadmitted deep tree can enter the close path before
census and overflow this fixed array/panic instead of incrementally reaching terminal-empty. No
fixture covers deep unadmitted-tree close.

### Hostile fixture/mutation gate is not faithful

The root verifier's baseline can accept this source because it is a substring predicate:

- it declares the range text sufficient for map cursor acceptance (`📜️script.ts:6319`), with no
  retained B-tree traversal state proof;
- it treats helper-name occurrence as backing admission (`📜️script.ts:6320-6321`) and does not
  reject the live B-tree or hash-layout estimates;
- it does not model production of an oversized `ComponentTree` into `UnadmittedSlot` before census;
- it requires ACK-call presence but not published-owner validation before tracker acknowledgement
  (`📜️script.ts:6342`); and
- it checks that public `Drop` implementations exist (`📜️script.ts:6323`), but neither rejects
  direct credit release in populated-owner drops nor requires populated drop fixtures.

The mutation loop itself executes the same textual function (`📜️script.ts:6364-6417`), so passing
its listed mutations would not establish discrimination for the live counterexamples above. A
compile-plausible mutation that merely moves `PATCHES.mark_ack` before/without a successful pending
published match, retains the `range` restart form, or changes a populated owner into its ordinary
Drop is not killed by the current corpus.

## Full B3/B4 disposition

**B3: PASS for the prior blocker.** The full-terminal matching-owner branch advances the matching
terminal and returns before unadmitted/rejected/surface conversion
(`🩹️patches/🦀️component.rs:424-438`). The following branches convert at most one owner, and the
fixture names for matching unadmitted/rejected/surface terminal saturation are present at
`🩹️patches/🦀️component.rs:832-853`.

**B4: PASS for the prior rejection/exhaustion blocker.** `begin`, `retain_unadmitted`, and
`reserve_mounted` call `SurfaceReconcileReservation::try_new(generation)` before
`commit_generation` (`🩹️patches/🦀️component.rs:178-180`, `238-241`, `254-257`). The idle rejection
path finds a terminal slot, constructs `try_from_reconciler`, and only then commits
(`🩹️patches/🦀️component.rs:366-391`). `next_generation` uses `checked_add` and the commit makes
maximum exhaustion permanent (`🩹️patches/🦀️component.rs:582-591`); the exact saturation fixture is
at `🩹️patches/🦀️component.rs:893-923`. No saturation/wrap/repeated-maximum source path was found
in the audited production regions.

These limited B3/B4 passes cannot make P5b green while B1, B2, and B5 are false. No broader runtime
claim is made.

## Commands and results

Executed from `/Users/ueli/Documents/semio`:

```text
rustfmt --edition 2024 --config skip_children=true --check <three P5b Rust files>
rustfmt --edition 2021 --config skip_children=true --check <three P5b Rust files>
```

Both parsed all three files and reported formatting diffs in shared source; formatting is **not**
clean. No formatter write was performed.

```text
git diff --check -- <three P5b Rust files> 📜️script.ts
git diff --cached --check -- <three P5b Rust files> 📜️script.ts
git diff HEAD --check -- <three P5b Rust files> 📜️script.ts
```

All three scoped whitespace checks passed with no output.

An isolated `bun -e` P5b source/mutation probe reported:

```json
{"baselinePredicatePresent":true,"publishedAckPresenceOnly":true,"trackerAckBeforePublishedAck":true,"ackOrderingMutationPresent":false,"populatedReconcilerDropPresent":true,"readyPatchDirectDropPresent":true,"publicDropFixtureIsOnlyEmptyReconciler":true}
```

The whole interactivity verifier was intentionally not run: its invocation would execute unrelated
P1q/P2 corpus and scan work while those Rust writers remain active.

## Required disposition

Do not accept P5b. Preserve the direct-reconcile cutover, B3 ordering, and transactional rejection
generation repair, but add a truly retained map traversal, exact source/map/hash backing model,
pre-admitted/paged tree producer, published-owner-first ACK transition, lossless populated-owner
handback/close paths, deep-close guard, and fixtures/mutations that kill each counterexample above.
