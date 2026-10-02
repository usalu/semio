# 📓️ API: deferred history replays (design §16.6, gap N17)

For S3-W2A (runtime adoption) and S3-E2E. Owner: W1-G (store). Status and verification: `📓️w1-g-report.md`, section
"Session 3 — 2026-10-02".

## 1. What it is

A history step can need a Report replay of everything downstream of it. Over a long history (the paged ledger admits
thousands of edits) that replay must not run inside one reactor turn. The store therefore replays in budgets:

- **Remote changes** (a supersession, undo or redo another replica authored): `defer_remote_replays(Some(n))`. This
  already existed (session 2) and the runtime already drives it.
- **Local steps** (new, N17): `defer_local_replays(Some(n))`. Covered:
  - an interior undo or redo;
  - a checkout, or an alternative switch, away from the applied tail;
  - a supersession (finalize as overwrite or as a new alternative) authored without a finished replay. This is the old
    synchronous `dry_run` path.
- **Reload**: the bounded retained initializer replays one operation per job step. It sits behind
  `build_document_store_initialization_job`, `begin_persisted_document_store_replacement` and every archive load.
  `ArtifactStore::reset` (`load_document_pack` / `load_document_text` / `hydrate_document_lane`) still folds the whole
  history synchronously; see §4.

Steps that need no replay stay synchronous. These are a tail undo, a commit, a branch, and a checkout that only drops
tail edits.

## 2. Store API (`OS/🏪️store/🦀️.rs`, region `🔖️DeferredReprojection`)

```rust
store.defer_local_replays(Some(operations))   // None (default): every local step replays inside its dispatch
store.local_step_pending() -> bool            // a local step waits for later turns
store.discard_local_step() -> bool            // drop it: zero trace (nothing was logged, announced or shown)
store.reprojection_progress() -> Option<ReplayProgress>   // remote, local or both; {done,total}
store.step_reprojection().await -> Result<Option<ReplayProgress>, VcsError>  // one budget; None once adopted
store.cancel_reprojection() -> bool           // pause: drops the running replay, the change stays admitted
ArtifactCommand::moves_history(&self) -> bool
VcsError::HistoryReplaying                    // fault code `history.replaying`
```

### Contract while a local step waits

- **Dispatch.** The dispatch of the step replays one budget (plus at most one ring stride to reach its start) and
  answers `Ok`. The receipt carries no change yet.
- **What the user sees.** The replica shows the history before the step. Nothing of the step is in the log or on the
  backbone.
- **Refused meanwhile.** Every history-moving command is refused with `VcsError::HistoryReplaying`, and so is
  `commit_finished_replay`. History-moving commands are undo, redo, checkpoint, alternative, checkout, supersede and
  resolve-conflict.
- **Allowed meanwhile.** Edits, tool transactions and remote changes still land, and they restart the step's replay.
- **Adoption.** Each `step_reprojection()` (also every `tick()`) replays one budget. When the replay finishes:
  - the step's transitions are recorded and announced (flushed at once);
  - the head moves;
  - the generation bumps;
  - `last_projection_cause` is `Replay`.

  The result is exactly what an undeferred dispatch would record.
- **Blocking finalize.** A finalize whose report blocks (Error or Fatal) makes `step_reprojection()` answer
  `Err(VcsError::Rejected { policy: Normal, messages })`. Nothing of the finalize is recorded. Any waiting remote change
  keeps waiting.

## 3. Runtime adoption (S3-W2A, `OS/🔌️plugin/🦀️.rs` + `⏪️time-travel/🦀️.rs`)

1. **Turn it on.** Call `store.defer_local_replays(Some(time_travel::TIME_TRAVEL_REMOTE_REPLAY_OPERATIONS))` next to
   both `defer_remote_replays` call sites (`🔌️plugin/🦀️.rs` ≈24789 and ≈25800).
2. **Driving.** No new driver is needed. `step_remote_replay` already calls `step_reprojection()` every turn while
   `reprojection_progress()` is `Some`, and its `Ok(None)` branch already refreshes the bodies and delivers `BaseMoved`.
3. **History band.** When `local_step_pending()` is true:
   - Show "Replaying history…" / "Verlauf wird nachgespielt…" instead of the remote wording.
   - Cancel calls `discard_local_step()`, which leaves zero trace, instead of pausing.
   - A `Rejected` from `step_reprojection()` shows the blocking notice: "Errors must be fixed or withdrawn before
     finalizing" (design §16.1).
4. **Refusals.**
   - Undo, redo and the alternative and checkpoint verbs answer `history.replaying` while a step waits. Notice: en
     "History is still replaying — wait for it or cancel it first." / de "Der Verlauf wird noch nachgespielt — warten
     oder zuerst abbrechen."
   - `historyEditBegin` should answer `timeTravel.busy` while `local_step_pending()`.
5. **Reload.** `hydrate_document_lane`, `load_document_pack` and `load_document_text` call `store.reset(...)`. That
   function, and `parse_document_pack`'s `replay_loaded_history` before it, fold the whole history synchronously. Route
   these whole-document loads through `begin_persisted_document_store_replacement` (the stepped retained initializer,
   proven one operation per step by the law `a_long_history_reloads_one_operation_per_initializer_step`), as the archive
   load already does.

## 4. Known bounds and open items

- **Ring stride at the start of a replay.** A replay first folds from the nearest retained prefix to its start, and
  that fold is synchronous. It covers at most one prefix-ring stride, which is ⌈N/capacity⌉ edits with the capacity
  clamped to [8, 16]. That is ≈ √N up to 256 edits and N/16 beyond. Making this prefix fold a stepped phase of
  `EditReplay` is the remaining store gap.
- **Edit-uniqueness check on reload.** The retained initializer validates edit-id uniqueness pairwise
  (`ValidateEditPair`). That is one O(1) step each, but there are N²/2 steps, so reloading a 10k-edit history costs about
  50 M steps. A single pass over a seen-set would be linear.
