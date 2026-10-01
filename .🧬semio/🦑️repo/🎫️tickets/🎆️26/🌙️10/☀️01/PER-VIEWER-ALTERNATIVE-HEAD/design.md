# Per-Viewer Alternative Head

The shared history fold treats `HistoryTransition::Branch` and `HistoryTransition::Checkout` as facts that move one head for every replica. `fold_history` calls `checkout`, which clears the live edit set down to the named checkpoint's change chain and then writes `fold.alternative`. `adopt_history_positions` and hydration copy that alternative and checkpoint onto the local envelope. A collaborator's `createAlternative`, `switchAlternative`, or `checkoutCheckpoint` therefore moves everyone, and an uncommitted edit older than the checkout (by HLC) drops out of `applied` on every replica (`checkout_governs_concurrent_edits_by_hlc`).

This ticket splits the head. Branch registration, Commit, Repin, Revert, Reinstate, and Supersede stay shared events. Which line a replica is looking at, and whether it is at that line's tip or at an explicit checkpoint, is persisted local-only state.

## Head

`ViewerHead { line_id, checkpoint_id }`.

- `line_id` is `trunk_alternative_id(document)` or a registered alternative id.
- `checkpoint_id: None` means the tip of that line: the tip's committed change chain, plus uncommitted edits tagged to that line.
- `checkpoint_id: Some(id)` means that checkpoint only. Uncommitted edits stay hidden until the viewer returns to the tip.
- `ViewerHead::canonical_trunk` is the trunk at its tip. `fold_history` is that projection. Hub Check In materializes it and never a viewer's alternative.
- `fold_history_for(..., head)` is the viewer's projection. Changes, checkpoints, alternative registration, refused transitions, and the redo stack do not depend on the head. `applied`, `checkpoint`, `alternative`, and the filtered `supersessions` do.

`Branch` registers the alternative (id, name, root checkpoint) and refuses a claim of the trunk id. It does not call `checkout` and does not select a head. `Checkout` still refuses an unknown checkpoint and otherwise changes nothing. Neither kind is authored by the store anymore: `createAlternative` emits `Branch` and then sets only the author's local head to the new alternative at its tip; `switchAlternative` and `checkoutCheckpoint` set the local head and reproject.

## Commit names its line

A branch root sits on the trunk chain and the new alternative, so a later commit cannot be attributed by parent id. `TransitionCheckpoint.line_id: Option<String>` is encoded after `timestamp` as an optional string. Absent, or equal to the trunk id, grows the trunk chain. Any other id must already be a registered alternative. The field is the domain default (absent means trunk), the same shape as `parent_id`, not a second codec.

## Edit line

An uncommitted edit is visible only at the tip of the line it was authored on. The tag is absent for the trunk. It lives on the edit (`Edit.line`, `HistoryEdit.line`, edit format 2) and on the wire as `MutationEnvelope.line` (trailing-flag bit 2, `0b100`, beside transaction and verb). Absent means trunk. Peers copy the envelope line onto the edit they materialize. There is no second map: the edit record is the save, and the envelope flag is the announcement.

## Viewer persistence

`HistoryLog` record `REC_VIEWER` (`0x45`, non-critical) stores the line id and the explicit checkpoint id. An absent record means the canonical trunk. Local `print_document_spr` writes it. `replay_envelopes_onto_pair` clears the viewer head to the canonical trunk before ingest and print, so the checkpoint pair Check In publishes has no viewer record and hydrates to the trunk. `fold.checkpoint` is the resolved checkpoint for display and for the parent of the next commit. It is not written back into the explicit viewer checkpoint, or a tip view would hide uncommitted edits.

## Supersede

Scoped supersessions stay shared events. A scope applies when it is absent or equal to the viewer head's line (the trunk id for `fold_history` and for Check In). The concurrent ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING` owns the `Supersede` variant (tag 6). This ticket does not change that variant. It stops `Branch` and `Checkout` from selecting the line the scope is compared with. Expect steps in the supersede-fold fixture that need a branched line pass `head` and fold independently of the checkout events in the log.

`CreateAlternativeWithSupersede` dry-runs the fold with the new alternative as the head, then installs that head only if the install succeeds.

## Surfaces

- Hub Check In names `canonical_check_in_line(document_id)` and materializes `replay_envelopes_onto_pair` after the head is cleared.
- `HistoryView.active_alternative_id` and `current_checkpoint_id` stay the fields they are. They now read this viewer's head. The shared registration stays in `alternatives`.
- `createAlternative` says, in English and in German, that the new alternative can be edited without disturbing the current one because only this viewer's head moves.

## Implementation

`fold_history` is the canonical trunk tip. `fold_history_for` takes a `ViewerHead`. `Branch` only registers; `Checkout` only validates. The store no longer authors either as a head move: `createAlternative` emits `Branch` and then sets this replica's head; `switchAlternative` and `checkoutCheckpoint` set the head and reproject. `REC_VIEWER` (`0x45`) persists that head. Check In calls `canonical_check_in_line` and `replay_envelopes_onto_pair` clears the head before it republishes, so the pair hydrates to the trunk.
