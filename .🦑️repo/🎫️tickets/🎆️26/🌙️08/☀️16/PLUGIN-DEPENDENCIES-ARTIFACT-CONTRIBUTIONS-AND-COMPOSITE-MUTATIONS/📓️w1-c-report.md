# W1-C Report — Store Transaction Coordinator

Lane: **1-C store coordinator**. Contract: `📋️contract-freeze.md` §1/§5; scout findings
`📓️scout-2-group-undo-and-hosts.md` §1/§2. Start commit `7ad8955884`.

## Files touched (exclusive lease, full authorship)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs` — `🔖️Composition` region's
  `SpaceMember` trait area and the `🔖️CompositionCoordinator` region only.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🦀️component.rs` — read/verified, no functional change
  needed (see "vcs" below).

No other file was touched.

## API as landed

### `SpaceMember` trait — one new method

`fn stamp_tail_origin(&mut self, origin: crate::os_spr::MutationOrigin) -> Result<(), VcsError>;`
— `stamp_tail_group_id`'s provenance-direction twin: stamps `origin` onto every `MutationMeta` entry
of a member's TAIL applied edit. Implemented for `ArtifactStore<P, Mutation>` identically to
`stamp_tail_group_id` (same `NothingToUndo`/`UnknownEdit` failure modes). The test-local
`SpaceMember` delegate wrapper in the test module already carried a matching delegation by the time
I reached it (a concurrent session added the same method shape independently — see "Concurrent
churn" below); verified it delegates correctly.

### `MemberRelation` (new)

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MemberRelation { Owned, Peer }
```

### `TransactionCoordinator` (renamed from `CompositionCoordinator`)

`CompositionCoordinator` is kept as an exact type alias (`pub type CompositionCoordinator =
TransactionCoordinator;`), not a wrapper — same type, same methods, same associated functions. This
was a deliberate deviation from a literal in-place rename: `CompositionCoordinator` is referenced
from `🔌️plugin/🦀️component.rs` (leased to sibling lanes 1-A/1-B, under concurrent edit at
non-overlapping regions of the same 14k-line file) and from two plugin artifact files
(`✏️s/🔌️plugins/📐️cad/…`, `✏️s/🔌️plugins/🗄️stdio/…`), all outside this lease. Renaming in place
without touching those files would have broken their compile mid-wave; the alias keeps every
existing/concurrent call site compiling and behaving unchanged while the canonical name going
forward is `TransactionCoordinator`. Flagging this for the coordinator: a follow-up mechanical
rename of `CompositionCoordinator` → `TransactionCoordinator` in those out-of-lease call sites (pure
find/replace, no behavior change) would let the alias be dropped.

- `pub fn dispatch_group(&mut self, parent_ref, parent, children, parent_ops, genesis, meta) ->
  Result<GroupReceipt, VcsError>` — **signature and behavior byte-identical** to before. Now a thin
  wrapper: `self.dispatch_relation_group(MemberRelation::Owned, ...)`.
- `pub fn dispatch_peer_group(&mut self, initiator_ref, initiator, peers, initiator_ops, meta) ->
  Result<GroupReceipt, VcsError>` (new) — Peer-relation twin, no `genesis` parameter (a peer
  transaction never creates a child). Thin wrapper: `self.dispatch_relation_group(MemberRelation::
  Peer, ..., Vec::new(), meta)`.
- `fn dispatch_relation_group(&mut self, relation, parent_ref, parent, children, parent_ops,
  genesis, meta) -> Result<GroupReceipt, VcsError>` (new, private) — the shared two-phase engine.
  Phase 1's per-child check branches on `relation`: `Owned` runs the exact original `self.graph.
  owner_of(child) == parent` check (verbatim, same error `VcsError::OwnershipViolation`); `Peer`
  runs `self.graph.would_cycle_links(parent_ref, child)` instead (no ownership check at all,
  `VcsError::CompositionCycle` on hit — including the trivial self-transaction case, since
  `would_cycle_links` treats `from == to` as a cycle). Genesis handling is completely untouched and
  only ever reached under `Owned` (Peer callers structurally cannot pass genesis — no parameter).
  Phase 2 is identical for both relations EXCEPT: under `Peer`, after a child's `stamp_tail_group_id`
  succeeds, it additionally gets `stamp_tail_origin(MutationOrigin::Transaction { initiator:
  ForeignTarget::from(parent_ref) })` and the coordinator records `self.graph.insert_link(parent_ref,
  child)` (best-effort — the edge was already cleared by `would_cycle_links` in phase 1, so this can
  only fail on a concurrent graph mutation, not worth failing an already-applied transaction over).
  The initiator's own edit is never origin-stamped under either relation, since it is not foreign to
  itself. Compensation (`Self::compensate`, unchanged) is called identically from every phase-2
  error branch under both relations.
- `undo_group`/`redo_group` — **zero code changes**. They were already fully relation-agnostic: they
  only ever consult `member.tail_group_id()`/`member.redo_tail()`, never `self.graph`/ownership, so
  a `Peer` group reverses as one exactly the same way an `Owned` group does. Confirmed with a new
  end-to-end test (`undo_group_reverses_both_members_of_a_real_peer_transaction`) rather than
  changing anything.

### vcs component.rs

No functional change. `VcsError::CompositionCycle`/`OwnershipViolation`'s existing doc comments
already named `would_cycle_owns`/`would_cycle_links` as the two raising primitives before this wave
touched anything — the Peer cycle guard design (reusing `would_cycle_links`) matches what those
comments already anticipated. Read and verified only.

## How `Owned` stayed identical

`dispatch_group`'s public signature is unchanged, and `dispatch_relation_group`'s `Owned` arm is the
original code moved verbatim (ownership check, genesis loop, phase-2 apply order, compensation) with
the `if relation == MemberRelation::Peer { … }` origin/link-stamping block skipped entirely. No
existing test was modified. A new test, `dispatch_group_owned_path_never_stamps_a_transaction_origin`,
explicitly asserts both the parent's and an owned child's `MutationMeta.origin` stay at the ordinary
`MutationOrigin::Owner` default after a real `dispatch_group` call (proving `stamp_tail_origin` is
never invoked on this path) and does so through the `CompositionCoordinator` alias, exercising alias
equivalence at the same time.

## Tests written (all new, alongside the unmodified existing composition suite)

In `🏪️store/🦀️component.rs`'s `//#region 🔖️TransactionPeerTests` (nested inside the existing
`🔖️CompositionTests` region):

1. `dispatch_peer_group_commits_both_members_with_one_shared_group_id` — two artifacts with no
   ownership edge seeded; asserts both members applied, both share the same `group_id` (== the
   minted `invocation_id`), the peer's origin is `MutationOrigin::Transaction { initiator }` naming
   the real initiator, and the initiator's own origin stays `Owner`.
2. `compensate_undoes_applied_peer_members_in_reverse_order` — models "the second member's dispatch
   failed" the same way the existing `compensate_undoes_applied_members_in_reverse_order` (Owned)
   test does: calls the relation-agnostic `compensate` directly with only the already-applied
   members, asserts reverse-order rollback with zero skips.
3. `undo_group_reverses_both_members_of_a_real_peer_transaction` — real `dispatch_peer_group` call,
   then `undo_group` with the receipt's `invocation_id`; both members revert, zero skips.
4. `dispatch_peer_group_rejects_a_transaction_that_would_close_a_peer_link_cycle` — a real two-hop
   cycle: transaction 1 (A initiates, B is the peer) succeeds and persists a `Links` edge A→B in the
   coordinator's graph; transaction 2 (B initiates, A is the peer) is rejected with
   `VcsError::CompositionCycle` and zero side effects, proving the cycle guard is stateful across
   separate calls, not just a same-call self-reference check.
5. `dispatch_group_owned_path_never_stamps_a_transaction_origin` — described above.

## Gates (real output, this session)

`cargo check -p semio-framework-os-kernel --lib`: clean (warnings only) once a concurrent session's
in-flight edit to `🚪️io/🦀️component.rs` (unrelated `CodecOutput`/`PayloadSource` refactor, nothing in
my lease) settled — confirmed via `git status`/`git log --date=iso` before assuming it was mine, per
this ticket's shared-tree rules; it cleared on its own within ~15s of polling.

`cargo test -p semio-framework-os-kernel --lib os_store`:
```
test result: FAILED. 120 passed; 1 failed; 0 ignored; 0 measured; 777 filtered out; finished in 0.05s
```
The 1 failure is `os_store::component::tests::switch_to_an_alternative_whose_pinned_checkpoint_is_missing_is_rejected`
— the exact KNOWN EXTERNAL FAILURE named in this lane's brief (a stricter alternative/checkpoint
validation another live session added after commit `3140b01d2c`). All 5 new tests above pass; every
pre-existing composition test (`dispatch_group_*`, `compensate_*`, `undo_group_*`, `redo_group_*`,
`mint_child_id_*`, etc.) passes unmodified.

`cargo test -p semio-framework-os-kernel --lib` (full):
```
test result: FAILED. 895 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.63s
```
Failures: `os_io::tests::io_registry_rejects_a_conflicting_key_without_replacing_the_first_entry`,
`os_dsl::fixture_sweep::m5_cross_artifact_rejection::all_non_stdio_grammars_reject_each_others_shipped_fixtures`,
`os_store::component::tests::switch_to_an_alternative_whose_pinned_checkpoint_is_missing_is_rejected`
— exactly the 3 named baseline failures, no new ones. **Delta vs the 890/3 baseline: 890→895 passed
(+5, my new tests), 3→3 failed (unchanged, same three tests).**

## Notes for later waves

- `dispatch_peer_group` takes an explicit `initiator_ref`/`initiator` + `peers` shape mirroring
  `dispatch_group`'s `parent_ref`/`parent` + `children` — this matches contract §5.2's "member #0 =
  initiator instance" framing directly, so a host wiring the real transaction protocol (guest SDK/
  transaction lanes) can map `TransactionPrepare`'s member list onto this call with member #0 as
  `initiator` and the rest as `peers` with no translation layer.
- The `Links` graph edges `dispatch_peer_group` persists (`insert_link(initiator, peer)`) are
  directed and additive only — nothing in this lease removes them (e.g. on a completed/undone
  transaction). If a later wave wants a transaction's link edge to expire with its group undo, that
  is new scope, not implied by anything in this contract section.
- The `CompositionCoordinator` → `TransactionCoordinator` alias (see "API as landed" above) is a
  live cross-lease compatibility seam, not a permanent design choice — worth a coordinator-tracked
  follow-up once 1-A/1-B/other out-of-lease callers land, to do the mechanical rename and drop the
  alias.
