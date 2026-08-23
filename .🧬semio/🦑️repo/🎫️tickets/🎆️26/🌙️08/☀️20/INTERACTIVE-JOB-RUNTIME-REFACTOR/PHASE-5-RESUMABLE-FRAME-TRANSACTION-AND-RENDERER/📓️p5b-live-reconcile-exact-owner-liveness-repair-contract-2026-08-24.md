# P5b Live Reconcile Exact-owner and Liveness Repair Contract — 2026-08-24

## Status

**Prepared; P5b remains RED.** This contract merges the live-reactor cutover packet with the five
blockers from its second independent reaudit. It must start only after P4e source acceptance because
both packets touch the root verifier and renderer-adjacent production paths. It does not accept or
start P5a, P5c, P5d, or P5e.

No source, Cargo, Nx, Wasm, browser, runtime, or network action was performed for this contract.

## Owned Boundary

- `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs`;
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🩹️patches/🦀️component.rs`;
- only the narrow reactor poll cursor in the parent reactor component when scheduling cannot remain
  inside `PatchTracker`;
- the distinct P5b predicate/mutation region in root `📜️script.ts`.

Do not absorb frame transaction mounting, layout/text, tessellation/atlas/GPU, multi-window/resize,
P4 diagnostics, or unrelated plugin rendering into this packet.

## Required Production Cutover

Remove all production reachability of run-to-completion `SurfaceReconciler::reconcile`; any direct
parity oracle is `#[cfg(test)]`. Replace `PatchTracker::diff` with one fixed, generation-tagged live
authority supporting begin, one-opportunity step, take-ready, take-rejected, resume, request-close,
close-step, and terminal-empty. The reactor poll may advance at most one semantic scalar,
container/entry, node/child identifier, patch operation, or close owner per grant.

Stable FIFO surface order, revision checks, ACK/rejection behavior, duplicate-key semantics,
last-valid reconciler visibility, and atomic candidate adoption must remain exact. Add an O(1)
revision accessor; a scalar revision read must not clone a complete snapshot.

## B1 — Cursorized Semantic Census

Delete live use of recursive `tree_node_semantic_usage`. Store a fixed-capacity census stack and
explicit node, field, container, list/map entry, binding, action, data-attribute, string-byte, and
depth cursors in the retained authority. A nonzero grant advances one unit and checks cancellation,
deadline, fuel, generation, and limits before ownership transfer or allocation. Zero fuel and an
expired deadline leave every cursor and owner unchanged.

Reject at the first item, byte, depth, or backing-capacity violation. A wide list/map and a deeply
nested `UiValue` must require multiple turns and no recursion. The census result must be the actual
admission transition for the later build/diff path, not a decorative measurement repeated by an
unbounded helper.

## B2 — Exact Persistent Credits and Producer Ownership

Account for actual backing ownership of every live class:

- census stack/pages and pending tree;
- candidate/current reconciler records and all keys/strings/values;
- traversal/index/seen/parent/path structures;
- patch-operation storage and output pages;
- ready, deferred, unadmitted, rejected, surface, terminal, and retirement owners;
- operation/generation/control shells and aggregate process bookkeeping.

Do not treat requested `Vec` capacity, BTree length estimates, or a fixed slot as byte admission.
Use fixed owned storage or allocate-inspect-admit with a retained rejection disposer. Oversized
producer output must be paged/admitted before it can occupy an unadmitted slot.

`take_ready` transfers its attached persistent credit with the candidate; it never releases credit
while the tracker or next generation still owns the reconciler. A later generation that retains old
reconciler + new tree + candidate + patch must charge the simultaneous aggregate. Credit returns
only after the exact owner/backing reaches terminal-empty through bounded close.

## B3 — Saturation-safe Close Ordering

When the terminal array is full, `PatchTracker::close_step` must advance one matching already
close-marked terminal before trying to convert an unadmitted, rejected, or surface owner. It may not
repeatedly return on the blocked conversion while capacity-producing terminal work exists. After a
terminal step frees capacity, a later grant converts exactly one pending class.

The close state must be fair across every class and cannot require external terminal checkout.
Blocked/transient contention reports zero released ownership; durable close intent and the exact
cursor remain discoverable. Terminal-empty proves every class, backing, and aggregate credit empty.

## B4 — Permanent Generation Exhaustion

Replace every `checked_add(1).unwrap_or(u64::MAX)` generation assignment in begin, unadmitted
retention, mounted reservation, rejection, and related paths. Zero and exhausted generations are
never issued; `u64::MAX` is admitted at most once when reached legitimately, and every subsequent
request fail-closes with the identical tree/reservation/owner returned or durably retained. No
saturation, reset, wrap, or repeated maximum creates an ABA alias.

All callbacks, ACKs, resumes, terminal handles, and close cursors validate the complete slot and
generation authority before mutation.

## B5 — Lossless Public Handback

Public Drop paths may not call a best-effort fixed global registry and then recursively destroy the
cap-plus-one owner. Make terminal-handback capacity part of admission or introduce a lossless
owner-preserving saturation protocol with fixed durable intent and mounted maintenance. An owner
whose handback cannot complete remains intact, attributable, and recoverable; ordinary Drop is
fail-closed and never the deep disposer.

Mounted and public lifecycles must converge on the same terminal model. Rejected, ready, checked-out
terminal, retained reconciler, source tree, patch, and public job Drop all eventually drain one
owner/backing per close grant without duplicate credit.

## Required Fixtures

1. low nonzero fuel, zero fuel, near-deadline, wide node, deep nested value, and recursive-helper
   removal proof for semantic census;
2. operation/node/key/string/value/patch/page/control item and byte maximum plus maximum +1, with
   exact rejected pointer/page identity before candidate mutation;
3. actual backing over-capacity despite a smaller requested capacity, including patch storage and
   retirement forest;
4. successful publication followed by a second generation at the simultaneous aggregate cap and
   +1 while current + source + candidate + patch + unadmitted/ready/terminal classes are live;
5. cancellation, stale generation, duplicate key, panic/fault, ACK rejection, consumer abandonment,
   and close interruption after every cursor/owner phase, preserving last-valid revision;
6. terminal array full plus matching unadmitted, rejected, and surface owners, each reaching
   terminal-empty without external checkout;
7. generation near maximum, maximum, first post-maximum refusal, and repeated refusal with no state
   mutation or alias;
8. public ordinary Drop at terminal cap and cap +1, checked-out terminal Drop handback, registry
   contention, and exact eventual credit return;
9. quiet wake and register/race/recheck schedules proving no spin, lost wake, duplicate wake, or
   close starvation;
10. O(1) revision observation and byte-identical parity against the test-only direct oracle across
    worker counts after the final serialized runtime matrix is permitted.

## Permanent Verifier Mutations

The P5b predicate must read the live reconcile, tracker, reactor, and relevant exported sources. Its
self-tests must restore and kill:

- production `.reconcile(tree)` or a terminal loop around the cursor;
- `tree_node_semantic_usage` whole recursive preflight, recursive list/map descent, or one-grant
  whole container traversal;
- requested-capacity/length estimates, post-admission dynamic growth, or missing backing credit;
- credit release inside `take_ready` before candidate/reconciler retirement;
- scalar revision via `snapshot()` clone;
- bulk `clear`, wholesale replacement/drop, or missing take/resume/close/terminal witness;
- unadmitted/rejected/surface early return ahead of a capacity-producing terminal close step;
- saturating/wrapping/repeated-maximum generation;
- best-effort public terminal handback that drops on full capacity;
- missing aggregate-cap +1, terminal-saturation, public-drop, or exhaustion fixtures.

Token-name presence alone is insufficient. Each mutation must change the actual production form and
make the focused self-test reject it.

## Source Handoff Gates

- exact production caller census shows zero run-to-completion reconcile and one mounted retained
  reactor path;
- scoped edition-2021 rustfmt and working/cached/HEAD diff checks, with peer-owned index divergence
  reported rather than mutated;
- permanent interactivity verifier self-tests green and live P5b predicate clean;
- independent Terra audit accepts B1–B5, caller reachability, owner/credit inventory, close/liveness,
  hostile fixtures, and faithful mutations.

Cargo, Nx, debug/release, Wasm, browser, runtime, stress, replay, allocation, cancellation, and
timing evidence remains deferred to the one-owner final matrix. P5b acceptance does not make Phase 5
green; P5c, P5a, P5d, and P5e remain separate packets.
