# 📋️ Event-Sourced Framework Version Control — Design

## 🔎️ Diagnosis (2026-09-19)

- `ArtifactStore::dispatch` replicates `Apply` as per-op `MutationEnvelope`s (`BackboneMessage::Mutations`)
  and ingests them through `ingest_remote`: causal DAG → HLC-sorted insertion into `applied_edit_ids`
  → replay of the divergent suffix. That half is already event-sourced.
- Every *structural* command (`Undo*`, `Redo*`, `CommitCheckpoint`, `CreateAlternative`,
  `SwitchAlternative`, `CheckoutCheckpoint`, `AmendLast*`, non-document-lane applies) and
  `attach_backbone` instead broadcast `BackboneMessage::Snapshot { pack, spr }` (whole initial snapshot +
  whole op log + cursor). The receiver must *retrospectively merge* that snapshot against its own history
  (`merge_remote_snapshot`, `snapshot_ledger_targets`, `remap_snapshot_message_ledger`) — all three are
  fail-closed stubs, so `attach_backbone` on the second peer of every pair errors and
  `two_instances_converge_disjoint_edits_via_backbone` is `#[ignore]`d in lowpoly/remodel.
- The persisted `.spr` carries a `REC_CURSOR` record (`applied_edit_ids`, `redo_edit_ids`,
  `checkpoint_id`) plus `active_alternative_id` — mutable *state*, not semantic history.

## 🎯️ Target model

The document is `initial_snapshot` + an append-only, HLC-totally-ordered log of semantic events.
Everything else (applied set/order, redo stack, current checkpoint, active alternative, alternative
checkpoint chains, the payload projection) is a pure fold over that log. Peers exchange events only;
no peer ever receives or merges another peer's snapshot.

### Event kinds

| kind | wire | authored by |
|---|---|---|
| edit op | `MutationEnvelope` (existing) | `Apply*`, `AmendLast*` (only the new ops) |
| `Revert { mutation_ids }` | `HistoryEventEnvelope` | `Undo*` (own edits only) |
| `Reinstate { mutation_ids }` | `HistoryEventEnvelope` | `Redo*` (own edits only) |
| `Commit { change, checkpoint }` | `HistoryEventEnvelope` | `CommitCheckpoint` |
| `Branch { alternative_id, name, checkpoint_id }` | `HistoryEventEnvelope` | `CreateAlternative` |
| `Checkout { checkpoint_id, alternative_id }` | `HistoryEventEnvelope` | `CheckoutCheckpoint`, `SwitchAlternative` |

Identity across peers is the per-op `MutationId` (a receiver materializes one local edit per wire op), so
history events reference mutation ids; each store resolves them to its own edit ids. Change/checkpoint/
alternative ids are minted once by the author and carried verbatim.

### Fold (deterministic projection)

Events sorted by `(hlc.cmp_key(), id)`:

- edit E by actor A: `applied += E`; drop A's own entries from `redo`.
- `Revert(ids)`: edits owning `ids` leave `applied`, enter `redo`.
- `Reinstate(ids)`: edits leave `redo`, re-enter `applied`.
- `Commit`: register change/checkpoint facts; `checkpoint := id`; append to the active alternative.
- `Branch`: register alternative; `alternative := id`; checkout its checkpoint.
- `Checkout(c, alt)`: `applied := edits of c`, `redo := []`, `checkpoint := c`, `alternative := alt`.

`applied` is always kept in edit-HLC order, so the payload projection is `fold(initial, applied)` and is a
function of the event *set* only. Late (older-HLC) events trigger a re-fold of the bookkeeping and a payload
replay from the first divergent position (`reproject`). Quarantined edits (merge policy) stay excluded.

### Persistence

`.pack` = initial snapshot. `.spr` = edits + changes/checkpoints/alternatives facts + conflicts + the
history-event records. `REC_CURSOR` and `active_alternative_id` state are removed; loading re-folds.
The sync actor appends events (both kinds) instead of persisting whole snapshots on structural commands.
Initial load / external change pushes the persisted document through a dedicated load path (an empty
store *replaying* events), never through a merge.
