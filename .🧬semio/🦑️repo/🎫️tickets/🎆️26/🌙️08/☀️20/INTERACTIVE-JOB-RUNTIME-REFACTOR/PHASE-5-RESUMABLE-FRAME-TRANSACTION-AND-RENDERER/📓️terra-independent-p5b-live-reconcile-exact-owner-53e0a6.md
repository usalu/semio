# Terra Independent P5b Exact-owner and Liveness Acceptance Audit

Date: 2026-08-24  
Verdict: **RED — REJECT.**

## Scope and permitted checks

Read-only audit of the P5b contract, the prior second RED reaudit, Sol's status audit and
implementation report, and the current live reconcile/tracker/reactor/verifier sources. No Cargo,
Nx, Wasm, browser, runtime, network, or source mutation was run. The only write is this report in
the existing Phase 5 ticket.

Executed:

- `rustfmt --edition 2021 --config skip_children=true --check` on P5b `reconcile.rs` and tracker
  `patches/component.rs`: PASS.
- scoped working, cached, and `HEAD` `git diff --check` over P5b reconcile, tracker, mounted
  reactor, and root verifier: PASS. The relevant sources are already in the shared index and/or
  working tree; it was not changed.
- `bun 📜️script.ts verify interactivity --self-test`: PASS, DENY clean, with the one pre-existing
  allowlisted test-only bridge. The command's P5b mutation corpus ran, but is insufficient as
  detailed below.

## Confirmed production cutover

The direct run-to-completion reconciler is now `#[cfg(test)]`; repository source inspection found
no mounted production `.reconcile(` call. The retained live route is real:

`poll_kernel` → `PATCHES.reserve_mounted(surface)` → `plugin_render` → `grant.commit(tree)` → one
`PATCHES.drive_one()` → `take_ready_patch()`.

The same route has the O(1) `SurfaceReconciler::revision()` read, FIFO ready selection, ACK gate,
and last-valid reconciler slot. The tracker also has the specified terminal-full ordering for a
matching closing instance: it advances a matching terminal before trying unadmitted, rejected, or
surface conversion. The three matching-terminal fixture names are present.

These improvements do not satisfy B1, B2, B4, B5, or the required hostile-fixture/mutation gates.

## Blocking findings

### B1 — map census is not one entry opportunity

`SurfaceSemanticCensusCursor::value_step` stores a numeric cursor for `UiValue::Map`, but each
grant resolves the entry with `BTreeMap::iter().nth(entry)`
([reconcile.rs](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:353)).
`BTreeMap` iteration has no random access, so the grant for entry *n* walks the preceding entries
again. A wide map therefore has an unbounded semantic traversal inside an alleged one-entry grant.

The same branch charges map backing as `values.len() * size_of::<(String, UiValue)>()`
([reconcile.rs](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:357)), which is a length estimate rather than BTree backing ownership. This is expressly
forbidden by B2 as well.

The wide fixture covers a `UiValue::List`, not a wide `UiValue::Map`; no mutation restores the
`nth` re-walk or this BTree estimate. The verifier only rejects literal `for value in values` and
therefore accepts the actual unbounded implementation.

### B2 — credits are reservations, not exact backing ownership

The fixed aggregate ledger reserves each operation's configured maxima
([reconcile.rs](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:1071)); it does not admit or retain the actual backing that subsequently grows. During
live traversal/diff, production dynamically grows `traversal`, `flat`, `postorder`, `seen`, `ids`,
`new_retained`, `new_key_index`, `removal`, `ops`, and each parent `child_ids` by `push`/`insert`;
the one-node diff also builds and clones a complete record then extends a variable number of patch
operations in one grant ([reconcile.rs](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:906)). None is fixed storage or allocate-inspect-admit with a retained disposer.

Close has the same defect: `retire_forest: Vec<IntoIter<TreeNode>>` grows once per nested tree node
([reconcile.rs](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:1150), [reconcile.rs](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:1203)). This is post-admission, dynamically growing retirement backing; it is neither measured exactly nor represented in a durable close credit.

Although `take_ready` correctly moves `state.credit` into the candidate, the ready `UiPatch` is
separated from that credit. The mounted reactor then moves it to uncapped dynamic
`PENDING_PATCHES: RefCell<Vec<UiPatch>>` and bulk-takes it at end of poll
([reactor component](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️component.rs:1501), [reactor component](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️component.rs:1523)). That live output owner has no P5b byte/page admission, credit transfer, or terminal-close authority.

Consequently the required simultaneous current/source/candidate/patch/unadmitted/ready/terminal
aggregate-cap and +1 proof is absent. The claimed persistent-credit fixture only takes a single
ready reconciler and closes it; it does not establish the required simultaneous live classes.

### B4 — maximum generation can be consumed by a failed rejection

`mark_rejected` calls `issue_generation` before checking whether a terminal slot exists
([tracker](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🩹️patches/🦀️component.rs:338)). At `u64::MAX - 1` with a full terminal array, this changes
`next_generation` to `u64::MAX` and permanently sets `generation_exhausted`, then returns without
issuing that generation to a terminal owner. Subsequent requests are refused despite the maximum
never being admitted. This is a state mutation on a failed related-generation path and does not
meet B4's exact fail-closed owner rule.

The present generation fixture covers begin plus retain-unadmitted only; it does not cover this
rejection/capacity ordering.

### B5 — public handback remains unbounded and unscheduled

The best-effort loss is removed, but it is replaced with an unrestricted global singly linked
`Box<SurfaceReconcileRetained>` chain
([reconcile.rs](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:1236)). `Drop` prepends one owner, while public terminal retrieval scans the complete
chain linearly until it finds a generation
([reconcile.rs](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:1623)).

This chain has no fixed capacity, byte credit, per-grant maintenance cursor, fairness policy, or
automatic close driver. Public `from_sources`, `from_patch`, and `from_reconciler` can create
terminal owners without an admission credit, so credits do not bound the chain. A consumer that
does not know every generation can leave it indefinitely, and retrieval itself is unbounded. The
cap+1 fixture proves that 65 small owners can be manually retrieved, not that the required public
ordinary-Drop/checked-out/drop/registry-contention lifecycle is bounded, one-owner, and eventually
drained.

### Hostile fixtures and verifier mutations are not faithful to the contract

The reported 33 mutations pass their own syntactic predicate but do not mutate or reject the
production failures above. In particular they lack mutations for BTree cursor re-walk, BTree
length-estimate backing, dynamic traversal/record/patch/retirement allocation, uncapped reactor
output transfer, failed-rejection maximum consumption, or unbounded handback traversal.

The required actual-backing over-capacity, patch storage, retirement forest, full simultaneous
aggregate +1, full public drop/check-out/registry contention, and all related-generation exhaustion
fixtures are absent. Therefore the P5b predicate cannot be considered a faithful 33-mutation gate.

## Acceptance disposition

Do not accept P5b. Preserve the confirmed mounted retained cutover and B3 matching-terminal order,
but repair B1/B2/B4/B5 and add discriminating fixtures and restore-and-kill mutations before a new
independent audit. P5a, P5c, P5d, and P5e were not started or assessed for acceptance here.
