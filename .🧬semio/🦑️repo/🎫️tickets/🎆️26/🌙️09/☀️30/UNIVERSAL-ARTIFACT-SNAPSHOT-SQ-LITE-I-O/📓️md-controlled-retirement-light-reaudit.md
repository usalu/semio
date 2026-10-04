# CommonMark Controlled Retirement Light Re-audit

Read-only inspection on 2026-10-03 of the current staging report and five ticket-only Native drafts. No Cargo, build, runtime test, production edit, Git mutation or worktree. Concurrent changes remain possible. Source/public 12 laws and 90 assertions are recorded owner receipts; this audit does not independently execute them. The 21 Native candidates and two draft retirement tests remain authored, unmounted and unexecuted.

## Concrete Remaining Finding

`md-block-native-draft.rs:25` reconstructs a list through `for item` without a scoped `items.len()` stage or a per-item `step`. `paid_children` (`md-owned-native-draft.rs:45`) begins a child-count stage, so 4,096 empty items do publish cancellation checkpoints, each with `(completed,total)=(0,0)`. This is **not** an uncancellable loop. It does, however, fail to publish an interior known list-item frontier during reconstruction; there is no `(256,4096)` item checkpoint. Add an enclosing scoped item stage and advance it after each guarded child insertion, matching output projection and census. Author a decode cancellation law targeting this reconstruction frontier explicitly, since cancellation earlier in generated record binding does not establish this stage.

## Resolved Earlier Output Finding

`md-owned-native-draft.rs:13` now encloses list census in an exact item-count stage; every item advances once, including empty items. Child enqueue stages restore that parent. `md-block-native-draft.rs:5` similarly advances projected list items independently of child counts. Shared Native controls restore enclosing stage counters while retaining cumulative ownership charges. Thus the earlier missing wide-empty **output census** frontier is repaired in the inspected text, without a native execution claim.

## Ownership And Partial Construction

Borrowed census and projection queue reservations charge pointer slots before `try_reserve`; checked arithmetic refuses overflow. The flat block/inline vectors, root/child ID vectors, list-item vectors and copied UTF-8 use shared controlled allocation/copy helpers. These are cumulative logical storage charges, not a demonstrated allocator peak measurement: VecDeque/Vec reserve may select more capacity than requested.

`OwnedNodes`, `OwnedSlots` and `OwnedItems` retain transferred descendants on every inspected `?` path. `paid_children` moves each taken child into its guard before the next cancellation checkpoint; block/inline reconstruction stores the completed node in guarded slots before stepping. Missing required scalar/text fields are checked before descendant transfer. Reuse, backward edges, unknown IDs, orphans and unrelated variant fields explicitly refuse. No inspected path silently discards an owned literal field during successful reconstruction. Canonical guard mismatch retains the reconstructed candidate in `OwnedSnapshot` until controlled retirement.

## Retirement Inspection

`md-owned-native-draft.rs:33-35` admits already-owned child frontiers through `try_reserve(children.len())` before append. Rust's checked reserve handles capacity overflow; refusal selects `drain_blocks` or `drain_inlines`. Retirement itself may attempt allocations; the fallback functions do not allocate. `Vec::new`, `mem::take`, moving fields, `pop`, and child borrowing create no heap allocation. Strings and leaf payloads are normally dropped, including Link URL/title and CodeBlock info/literal; recursive child vectors are empty before their parent is dropped. No `forget` or deliberately leaked field appears.

The fallback descends iteratively and removes a leaf on each outer iteration. Block lists additionally remove trailing empty item vectors before descending into the last nonempty item. This decreases a finite node/item population and provides a textual termination argument, including wide empty lists. Inline retirement called from a popped block adds a fixed function frame, not one frame per tree level. No tree-recursive call chain was found.

The fallback restarts from the root after each removal. A depth-D chain therefore requires approximately D(D+1)/2 ancestor visits: about 33.6 million for D=8,192, before additional list/variant checks. This is a bounded-stack, allocation-free traversal with potentially quadratic cleanup latency. The nonfallible retirement contract has no controller; cleanup cannot publish progress or accept cancellation, and must complete to release already-owned state. This limitation should remain explicit rather than describing cleanup as controlled or constant-time.

## Required Native Evidence

The two draft tests directly call fallback functions on 8,192-level trees with a 64 KiB thread stack. They do not force `try_reserve` failure in retirement dispatch, instrument allocation attempts, exercise wide-only retirement, or assert elapsed work. Add a native allocation-refusal path law and wide-empty/mixed-list law before claiming refusal dispatch works at runtime. Keep allocation observation narrow to fallback execution after fixture construction. Existing deep partial-construction refusal laws still require execution through the mounted controlled hook; syntax receipts establish neither guard destruction nor preserved refusal categories.

Logical invalidity, row/ownership limits, allocation failure, work overflow and cancellation are distinct `ValueRefusalKind` constructors in the draft. The draft preserves them through `TextError::from_value_error` at the declared terminal. Final outward String classification still depends on the shared physical helper and must be verified by authentic native cancellation/limit/failure receipts. No current Native green or runtime stack-safety claim is made.
