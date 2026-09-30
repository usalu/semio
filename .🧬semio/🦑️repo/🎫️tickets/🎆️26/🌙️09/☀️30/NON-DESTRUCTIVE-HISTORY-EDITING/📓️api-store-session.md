# 📓️ Store Session API (W1-G, compiled and tested)

Re-exported at `store::` (`semio_framework_os_kernel::os_store`).

- `state_before(&mut self, mutation_id: &MutationId, drafts: &BTreeMap<MutationId, protocol::InputReplacement>) -> Result<Arc<P>, VcsError>`; retire the returned alias with `store.retire_snapshot_alias(arc)`.
- `begin_report_replay(&self, drafts: &BTreeMap<MutationId, protocol::InputReplacement>, from: Option<&MutationId>) -> Result<EditReplay<P, M>, VcsError>` (stamped with store generation, content revision, drafts).
- `EditReplay<P, M>`: `step(&mut self, edits: &impl ReplayEdits<M>, deadline: &mut dyn FnMut() -> bool) -> Result<ReplayStep, VcsError>` with `ReplayStep::{Pending(ReplayProgress), Finished(ReplayProgress)}`, `ReplayProgress { done: u32, total: u32 }`; `progress()`, `is_finished()`, `generation()`, `drafts()`, `from_position()`; `cancel(self)` (drop = same, retires scratch); `finish(self) -> Result<EditReplayResult<P, M>, VcsError>`. Pass `store.replay_edits()` as `edits`.
- `EditReplayResult<P, M>`: `state()`, `take_state()`, `converged_at()`, `report()`, `take_report()`, `replayed()`, `committed()`, `drafts()`, `generation()`, `from_position()`; retires on drop.
- `replay_report(&self, result: &EditReplayResult<P, M>) -> Result<protocol::ReplayReport, VcsError>` (splices durable tail outcomes when converged). Use for UI and the finalize gate (`blocks_finalize()`).
- `async commit_finished_replay(&mut self, finished: EditReplayResult<P, M>, finalization: HistoryFinalization) -> Result<CommandReceipt, VcsError> where P: Sync`; `enum HistoryFinalization { Overwrite, Alternative { name: String } }`; errors `VcsError::Stale { expected_generation, generation }`, `VcsError::Rejected { policy: Normal, messages }`.
- `ArtifactCommand::Supersede { scope: Option<String>, inputs: Vec<SupersedeInput<M>> }` (ordinal 17), `ArtifactCommand::CreateAlternativeWithSupersede { name: String, inputs: Vec<SupersedeInput<M>> }` (18), `SupersedeInput<M> { target: MutationId, replacement: Option<M> }` (`None` = withdraw).
- `mutation_outcomes(&self) -> Result<Vec<protocol::MutationReplayOutcome>, VcsError>` (durable per-mutation outcomes).
- `supersessions(&self) -> &EffectiveSupersessions` (`BTreeMap<MutationId, protocol::EffectiveSupersession>`).
- `mutation_ops(&self) -> Result<Vec<AppliedMutation<'_, M>>, VcsError>`: `{ mutation_id, edit_id: &str, position: usize, op_index: u32, operation: &M, meta: Option<&MutationMeta>, transaction: Option<&TransactionRef>, supersession: Option<&EffectiveSupersession> }`.
- Transaction slot: `ArtifactCommand::Apply { mutations, description, transaction: Option<TransactionRef> }`, `ApplyInLane { …, lane, transaction }`; `begin_apply_batch(…, factory, transaction)`, `begin_outbound_apply_batch(…, factory, transaction)` (trailing parameter); `fold_batch_item` stamps it; `ArtifactStoreBatchPublication::transaction()`.
- `enable_convergence_early_exit(&mut self)` (needs `P: PartialEq`); call it on `VcsArtifactApp` stores.
