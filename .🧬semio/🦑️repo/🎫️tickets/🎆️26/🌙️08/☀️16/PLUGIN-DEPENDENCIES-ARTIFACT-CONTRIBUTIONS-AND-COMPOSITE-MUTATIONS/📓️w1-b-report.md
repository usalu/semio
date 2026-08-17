# W1-B Report — Guest Transaction Dispatch

Lane: **1-B guest transactions**. Contract: `📋️contract-freeze.md` §5 (transaction protocol) + the M2 wire shapes from §2. Start commit `7ad8955884`.

## Exclusive lease

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` — regions `🔖️Emit` (new), `VcsArtifactApp` (struct + both its impl blocks + the `PluginApp` trait it implements), `🔖️Exchange` (existing), `🧪️testkit` (new). No other file touched.

`🔖️Emit` and `🧪️testkit` did not exist before this lane — both are new nested subregions, created rather than moving any pre-existing region boundary:
- `//#region 🔖️Emit` wraps the `transaction_fault` helper and `dispatch_emit` itself, nested inside the pre-existing (unowned) `🔖️DocumentContract` mega-region — unavoidable, since `dispatch_emit` is the one choke point every mutating dispatch path (`handle_action`, `handle_command`, `handle_action_invocation`, `handle_command_frame`, `import_media`) already funnels through, and the freeze/proposal checks have to live at that exact point. No other line of `🔖️DocumentContract` was touched.
- `//#region 🧪️testkit` is nested inside the pre-existing `🔖️Testkit` region (`pub mod testkit`), as two new blocks: three new `pub fn assert_*` script helpers (before the existing `#[cfg(test)] mod testkit_tests`) and a new sibling `#[cfg(test)] mod transaction_testkit_tests` (after it, self-contained, does not touch `testkit_tests`' own `DummyApp` fixture since `DummyMutation` never overrides `foreign_steps`).

## State machine as landed

**`PendingTransaction<Op>`** (new type, in `VcsArtifactApp`'s doc-adjacent area): `{ txn_id: String, ops: Vec<Op>, label: String, origin: protocol::MutationOrigin, base_generation: u64 }`. `VcsArtifactApp<A>` gained one field `pending_transaction: Option<PendingTransaction<A::Mutation>>` — the "one pending transaction per instance" invariant (§5.9) is exactly "this `Option` is not already `Some`".

**`TransactionProposalDraft`** (new type): `{ local_ops: Vec<Vec<u8>>, description: String, coalesce_key: String, foreign: Vec<protocol::ForeignStep> }`. `VcsArtifactApp<A>` gained `pending_transaction_proposal: Option<TransactionProposalDraft>`, populated by `dispatch_emit` instead of applying, drained once by `plugin_exchange` via the new `PluginApp::take_pending_transaction_proposal`.

### 1. Proposal instead of application (§5.1) — `dispatch_emit`

At the top of `dispatch_emit`, before any store dispatch: folds `artifact_mutations` forward against `self.store.snapshot()` using the public `Mutation::diff`/`MutationDiff::apply` pair (mirrors `ArtifactStore::replay_mutations`'s private `apply_mutation` fold, which lives across the crate boundary and isn't reachable from this crate), collecting `Mutation::foreign_steps(&running)` for every op. If any are non-empty: encodes `local_ops` (`OpBinary::encode_op` per op), stashes `TransactionProposalDraft`, and returns `Self::empty_result(...)` — applying nothing (config/draft dispatch, `self.store.dispatch`, `record_command` are all still below this check and never reached).

`plugin_exchange` (`🔖️Exchange`): after `AppCommand::Command` and `AppCommand::ArtifactCommand` succeed, calls `take_pending_transaction_proposal()`; if `Some`, pushes `AppFrame::TransactionProposal { in_reply_to: seq, proposal_id: "<instance_id>:<seq>", local_ops, description, coalesce_key, foreign }` (each `foreign` element `encode_wire_serialized`-encoded per contract §2) instead of the ordinary `Invocation`/`Done` frame. `push_invocation_side_frames` (effects/events) still fires either way.

### 2. Member state machine (§5.3–§5.7) — new `PluginApp` methods + `🔖️Exchange` arms

Six new object-safe `PluginApp` trait methods, implemented on `VcsArtifactApp<A>`:

- `take_pending_transaction_proposal(&mut self) -> Option<TransactionProposalDraft>`
- `transaction_prepare(&mut self, txn_id, mutation_id, payload, prepared_ops, label, origin: Option<protocol::MutationOrigin>) -> TransactionPrepareOutcome` — decodes EITHER wire form (`prepared_ops` empty ⇒ owner-mutation form, decodes `payload` as one op; non-empty ⇒ pre-planned form, decodes each entry + uses the already-decoded `origin`), validates every op against the current snapshot (folding forward the same way as the proposal check), and on success stores `PendingTransaction` with `base_generation = self.store.generation()`. Returns `{ foreign: Vec<protocol::ForeignStep>, rejection: Option<Fault> }` — never `Err`, so `plugin_exchange` can always frame a reply.
- `transaction_commit(&mut self, txn_id, meta) -> Result<String, Fault>` — checks `txn_id` matches the pending one and `base_generation == self.store.generation()` (§5.8), then `self.store.dispatch(ArtifactCommand::Apply { mutations: ops, description })` as the ONE edit, `self.store.stamp_tail_group_id(&txn_id)`, `self.store.stamp_tail_origin(origin)` (both via the `SpaceMember` trait 1-C landed on `ArtifactStore` mid-session — see Cross-lane dependency below), records a command-log entry, returns the edit id.
- `transaction_rollback(&mut self, txn_id) -> Result<(), Fault>` — clears `pending_transaction` if `txn_id` matches; nothing was ever applied, so this is pure bookkeeping (§5.5/§5.6).
- `transaction_undo(&mut self, group_id) -> Result<(), Fault>` — `self.store.tail_group_id() == Some(group_id)` then `self.store.undo()` (§5.7, single-member half of `CompositionCoordinator::undo_group`'s tail-based test).
- `transaction_redo(&mut self, group_id) -> Result<(), Fault>` — same via `self.store.redo_tail()`/`self.store.redo()`.

`🔖️Exchange` gained five new `AppCommand` match arms (tags 22–26, all previously fell into the removed-then-restored catch-all): `TransactionPrepare` (decodes `origin` bytes via `decode_wire_serialized` before calling, empty ⇒ `None`; pushes `AppFrame::TransactionPrepared { txn_id, foreign, rejection }`, `foreign`/`rejection` wire-encoded from the typed outcome), `TransactionCommit` (pushes `AppFrame::TransactionCommitted`, sets `mutated = true`), `TransactionRollback` (pushes `AppFrame::TransactionRolledBack`), `TransactionUndo`/`TransactionRedo` (push `AppFrame::Done { in_reply_to: seq }`, `mutated = true`, on success — no dedicated frame exists for these two in the frozen §2 shape).

### 3. The two safety rules (§5.8–§5.10)

- **§5.8 generation mismatch**: `transaction_commit` compares `pending.base_generation` to `self.store.generation()`; on mismatch restores `pending_transaction` (not discarded — a retry/explicit rollback can still act on it) and returns `transaction.generation-mismatch`.
- **§5.9 one pending transaction**: `transaction_prepare`'s first check is `self.pending_transaction.is_some()` ⇒ `transaction.instance-busy`, before any decode/validate work.
- **§5.10 mutating-surface freeze**: `dispatch_emit`'s very first check (before `last_emit_wire`, before config/draft dispatch) is `!artifact_mutations.is_empty() && self.pending_transaction.is_some()` ⇒ `transaction.instance-busy`. Read-only commands (`RefreshUi`, `ReadDocument`, `ContextMenu`, ephemeral lanes) never call `dispatch_emit` at all, so they are unaffected by construction — proved by `a_mutating_command_while_pending_is_rejected_but_reads_still_work` (`app.snapshot()` still reads correctly while a transaction is pending).

### 4. Rejection codes used

`transaction.instance-busy` (§5.9 second-prepare; §5.10 freeze), `transaction.unknown-mutation` (decode failure, either wire form), `transaction.member-rejected` (op fails `Mutation::validate`, `FaultOrigin::App`; also a snapshot-read failure, `FaultOrigin::Plugin`), `transaction.generation-mismatch` (§5.8), `transaction.commit-failed` (any other commit-path failure: unknown/mismatched `txn_id`, the `Apply`/`stamp_tail_*` calls themselves). `transaction.dependency-missing`/`version-mismatch`/`unknown-target`/`contribution-not-permitted`/`depth-exceeded`/`cycle` are host/router-side per §5.3–§5.4 and never produced by this guest lane. All carried as `encode_wire_serialized(&fault)` bytes in `TransactionPrepared.rejection` (empty = accepted), matching `AppFrame::Error`'s own existing encoding.

Rollback/undo/redo plumbing errors (unknown/mismatched `txn_id`/`group_id`) use a plain `plugin_sdk_fault` rather than a taxonomy code — the ten-code list is explicitly for prepare/commit rejections, not for these.

### 5. Contract gap flagged, not improvised

The owner-mutation form of `TransactionPrepare` (`mutation_id`+`payload`, `prepared_ops` empty) carries no `origin` on the wire per §2's encoding note. This member is by construction a foreign target of someone else's transaction, so `transaction_prepare` assigns `MutationOrigin::Transaction { initiator: ForeignTarget { artifact_id: "", artifact_kind: "", dialect: None } }` — a placeholder initiator identity, since the wire genuinely carries none. Flagged inline with a `🚧️` comment at the exact decode site. Recommend either the host always prefers the pre-planned form for foreign targets (which does carry `origin`), or a future contract revision adds an `origin` field to the owner-mutation form too.

## `🧪️testkit` helpers

`assert_proposes_transaction(app, command) -> TransactionProposalDraft` — dispatches, asserts generation/edit-count untouched, returns the drained draft.
`assert_transaction_commits_as_one_edit(app, txn_id, ops, label, origin) -> String` — pre-planned-form prepare + commit, asserts exactly one new `Edit` whose every `MutationMeta.{group_id,origin}` matches.
`assert_transaction_rollback_leaves_state_untouched(app, txn_id, ops, label)` — prepare + rollback, asserts generation/edit-count untouched.

Self-contained fixture `transaction_testkit_tests::{TxnSnapshot, TxnDiff, TxnMutation, TxnCommand, TxnApp}` (mirrors the pre-existing `testkit_tests::DummyApp` pattern exactly) — `TxnMutation::SetCountAndNotify` is the one variant with a real (non-defaulted) `foreign_steps` override, proving the proposal path against real trait dispatch rather than a mock.

## Tests written (8, all in `transaction_testkit_tests`)

1. `dispatching_a_mutation_with_foreign_steps_proposes_instead_of_applying`
2. `plain_command_still_applies_normally` (no false positives)
3. `commit_produces_exactly_one_edit_with_group_id_and_origin` (+ asserts a second commit of the same `txn_id` errors instead of double-applying)
4. `rollback_leaves_state_untouched`
5. `generation_mismatch_is_rejected_with_the_frozen_code`
6. `second_prepare_while_pending_is_rejected_instance_busy`
7. `a_mutating_command_while_pending_is_rejected_but_reads_still_work`
8. `undo_and_redo_by_group`

## Gates

### `cargo check -p semio-framework-plugin -p semio-framework-plugin-host`

`semio-framework-plugin-host`: **clean**, exit 0, zero errors.

`semio-framework-plugin`: **1 error, not mine, verified with the coordinator**. Every arm of this lane's own work compiles with zero errors and zero warnings referencing any of my new symbols (`PendingTransaction`, `TransactionProposalDraft`, `TransactionPrepareOutcome`, `transaction_*`, `Txn*`). The one remaining error:

```
error[E0063]: missing fields `dialect` and `role` in initializer of `AppDefinition`
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:4665:34
   |
4665 |             let mut definition = AppDefinition {
   |                                  ^^^^^^^^^^^^^ missing `dialect` and `role`
```

Attributed by the coordinator to a live concurrent session (ticket `26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET`, which added `AppRole`/`ArtifactDialect` fields to `AppDefinition`) editing the same file outside every named lease region here — line 4665 sits between `//#endregion 🔖️ArtifactContribution` and the next named region, not inside `🔖️Emit`/`VcsArtifactApp`/`🔖️Exchange`/`🧪️testkit`. Per explicit coordinator instruction, **not touched**.

A second, DIFFERENT error surfaced mid-session and WAS mine to fix: that same concurrent ticket's CHANNEL_VERSION 10 also appended three new `AppCommand` variants (`OpenArtifact`, `SetDefaultApp`, `ClearDefaultApp`). My `🔖️Exchange` match had been made fully exhaustive (I had deleted its pre-existing `_ => {}` fallback as dead-code cleanup once every prior variant was covered), so the match started failing `E0004: non-exhaustive patterns` the moment those three variants landed. Fixed with three explicit arms — not a silent wildcard — each pushing `push_os_fault(..., "unsupported", "<Variant> not yet wired (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET)")`, greppable and clearly commented as that ticket's seam to replace, per the coordinator's explicit instruction.

### `cargo test -p semio-framework-plugin --lib`

**Could not run — honestly reporting, not fabricating a pass.** `cargo test` requires the whole crate (including its test binary) to compile first; the single external `AppDefinition` error above blocks that unconditionally, for every module in the crate, not just mine. There is no cargo mechanism to compile/test one module of a single crate in isolation — `-p` selects a crate, not a module, and this crate has no separate lib target to narrow. I confirmed this directly: `cargo test -p semio-framework-plugin --lib` fails at the identical compile step (`error: could not compile 'semio-framework-plugin' (lib test) due to 1 previous error`), naming the same line 4665 construction site. I did not run the resulting test binary — none was produced. My 8 new tests are written and were re-read for correctness (fixture mirrors the proven `DummyApp` pattern field-for-field, my new `pub fn` helpers were checked against `cargo check`'s clean type-check of the whole module), but I have not personally observed any of them pass or fail at runtime, and I am not claiming otherwise. Re-run `cargo test -p semio-framework-plugin --lib` once line 4665 is fixed by its owning ticket.

## What the two hosts (W2) must drive

- Mint `txn_id`, discover members (`InstanceDirectory`/`ArtifactMutationRouter`), and fan out `TransactionPrepare` in EITHER wire form per §5.3 — this guest only reacts to whichever form arrives, it does not choose one.
- Collect every member's `TransactionPrepared.foreign` and recurse (depth/cycle bookkeeping is host-side, §5.4) — this guest reports its own foreign steps but enforces no depth/cycle limit itself.
- On any non-empty `rejection`, send `TransactionRollback` to every already-prepared member (§5.5) — this guest's `pending_transaction` only ever clears via an explicit `TransactionRollback`/successful `TransactionCommit`, never a timeout.
- Commit in reverse discovery order (§5.6); on a commit failure, `TransactionUndo` the already-committed members then roll back the rest — this guest's `transaction_commit` failing leaves its `pending_transaction` restored (see §5.8 note above), ready for either a retry or an explicit rollback.
- Fan out `TransactionUndo`/`TransactionRedo{group_id}` to every member of a group for group undo/redo (§5.7) — each member independently checks its own tail/redo-tail group id; a host must not assume success for a member whose tail has since moved on (this guest then errors, not silently no-ops).
- For the owner-mutation form specifically: consider always preferring the pre-planned form when the target is NOT the initiator, to close the origin-provenance gap noted above.

## Cross-lane dependency (not this lease's file, load-bearing for correctness)

`self.store.stamp_tail_group_id`/`stamp_tail_origin` (both on the `SpaceMember` trait, `🏪️store/🦀️component.rs`) are 1-C's landing, observed mid-session (not present when this lane started, present by the time `transaction_commit` needed them — confirmed via direct re-read of the live file, not assumed). No `🏪️store` file was edited by this lane.

## Files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` — regions `🔖️Emit` (new), `VcsArtifactApp` (struct + `PluginApp` trait + both impl blocks), `🔖️Exchange`, `🧪️testkit` (new). No region boundary moved; no other lane's region touched.
